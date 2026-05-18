/**
 * V4.4-003: ML 训练管理器
 *
 * 通过 Python 桥接调用 caelab.models.training 模块，
 * 管理 ML 势函数训练、流式 loss 曲线、断点续训。
 */

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::python_bridge::{run_python_script_async, PythonScriptRequest};

// ============================================================================
// 数据结构
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub potential_type: String,
    pub training_data_path: String,
    pub validation_data_path: Option<String>,
    pub cutoff: f64,
    pub epochs: u32,
    pub batch_size: u32,
    pub learning_rate: f64,
    pub energy_weight: f64,
    pub force_weight: f64,
    pub stress_weight: Option<f64>,
    pub use_gpu: bool,
    pub resume_from: Option<String>,
    pub output_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingStatus {
    pub task_id: String,
    pub potential_name: String,
    pub status: String,
    pub progress_percent: f64,
    pub current_epoch: u32,
    pub total_epochs: u32,
    pub energy_rmse: Option<f64>,
    pub force_rmse: Option<f64>,
    pub elapsed_time_sec: f64,
    pub estimated_remaining_sec: Option<f64>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingResult {
    pub success: bool,
    pub potential_name: String,
    pub potential_type: String,
    pub final_energy_rmse: f64,
    pub final_force_rmse: f64,
    pub best_epoch: u32,
    pub model_path: String,
    pub training_time_sec: f64,
    pub loss_history: Vec<LossRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LossRecord {
    pub epoch: u32,
    pub energy_rmse: f64,
    pub force_rmse: f64,
    pub val_energy_rmse: Option<f64>,
    pub val_force_rmse: Option<f64>,
    pub lr: Option<f64>,
    pub time_sec: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperparamRecommendation {
    pub potential_type: String,
    pub cutoff: f64,
    pub batch_size: u32,
    pub learning_rate: f64,
    pub epochs: u32,
    pub energy_weight: f64,
    pub force_weight: f64,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointInfo {
    pub path: String,
    pub epoch: u32,
    pub energy_rmse: f64,
    pub force_rmse: f64,
}

// ============================================================================
// 训练脚本生成
// ============================================================================

fn make_train_script(config: &TrainingConfig) -> String {
    let config_json = serde_json::to_string(config).unwrap_or_default();
    let train_path = &config.training_data_path;
    let val_path = config
        .validation_data_path
        .as_deref()
        .unwrap_or("null");
    let output_dir = config
        .output_dir
        .as_deref()
        .unwrap_or("/tmp/caelab_training");

    format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.models.training import load_training_data, MACETrainer, NequIPTrainer

config = {config_json}
train_data = load_training_data("{train_path}")
val_data = load_training_data("{val_path}") if "{val_path}" != "null" else None

trainers = {{"mace": MACETrainer, "nequip": NequIPTrainer}}
cls = trainers.get(config["potential_type"], MACETrainer)
trainer = cls(config)
trainer.train(train_data, val_data, "{output_dir}", config.get("resume_from"))
"#,
        config_json = config_json,
        train_path = train_path,
        val_path = val_path,
        output_dir = output_dir,
    )
}

fn make_list_checkpoints_script(output_dir: &str) -> String {
    format!(
        r#"
import json, os
from pathlib import Path
checkpoints = []
d = Path("{dir}")
if d.exists():
    for f in sorted(d.glob("*_ckpt_*.pt")):
        try:
            import json as j
            with open(f) as fh:
                data = j.load(fh)
            checkpoints.append({{
                "path": str(f),
                "epoch": data.get("epoch", 0),
                "energy_rmse": data.get("metrics", {{}}).get("energy_rmse", 0),
                "force_rmse": data.get("metrics", {{}}).get("force_rmse", 0),
            }})
        except Exception:
            pass
print(json.dumps(checkpoints))
"#,
        dir = output_dir,
    )
}

fn make_recommend_script(potential_type: &str, num_structures: u32) -> String {
    format!(
        r#"
import json
configs = {{
    "nequip": {{"potential_type": "nequip", "cutoff": 4.0, "batch_size": 5, "learning_rate": 0.005, "epochs": 100, "energy_weight": 1.0, "force_weight": 50.0, "reasoning": "NequIP 推荐: cutoff=4Å, 较小 batch 防止 OOM, lr=0.005 配合 scheduler"}},
    "mace":   {{"potential_type": "mace",   "cutoff": 5.0, "batch_size": 10, "learning_rate": 0.001, "epochs": 200, "energy_weight": 1.0, "force_weight": 100.0, "reasoning": "MACE 推荐: cutoff=5Å, 高 force_weight 保证力精度, lr=0.001 配合 warmup"}},
    "chgnet": {{"potential_type": "chgnet",  "cutoff": 5.0, "batch_size": 16, "learning_rate": 0.001, "epochs": 50, "energy_weight": 1.0, "force_weight": 2.0, "reasoning": "CHGNet 推荐: 已预训练, 微调时小 lr 避免灾难性遗忘"}},
}}
cfg = configs.get("{pt}", configs["mace"])
if {ns} < 500:
    cfg["epochs"] = min(cfg["epochs"], 50)
    cfg["reasoning"] += " (数据量小, 减少 epoch 防止过拟合)"
print(json.dumps(cfg))
"#,
        pt = potential_type,
        ns = num_structures,
    )
}

// ============================================================================
// Tauri 命令
// ============================================================================

/// 推荐训练超参数
#[tauri::command]
pub async fn recommend_training_config(
    potential_type: String,
    num_structures: u32,
) -> Result<HyperparamRecommendation, String> {
    let script = make_recommend_script(&potential_type, num_structures);
    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(30),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Err(format!("获取推荐配置失败: {}", result.stderr));
    }

    parse_json_output(&result.stdout)
        .ok_or_else(|| "解析推荐配置失败".to_string())
}

/// 提交训练任务（流式 loss 曲线通过 Tauri event 推送）
#[tauri::command]
pub async fn submit_real_training_job(
    app: AppHandle,
    config: TrainingConfig,
) -> Result<TrainingResult, String> {
    let script = make_train_script(&config);
    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(7200), // 2 小时超时
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Err(format!("训练失败: {}", result.stderr));
    }

    // 解析 stdout 中的 JSON 行，提取 loss 记录和最终结果
    let mut loss_history = Vec::new();
    let mut final_result: Option<TrainingResult> = None;

    for line in result.stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
            match val["type"].as_str() {
                Some("loss") => {
                    let record = LossRecord {
                        epoch: val["epoch"].as_u64().unwrap_or(0) as u32,
                        energy_rmse: val["energy_rmse"].as_f64().unwrap_or(0.0),
                        force_rmse: val["force_rmse"].as_f64().unwrap_or(0.0),
                        val_energy_rmse: val["val_energy_rmse"].as_f64(),
                        val_force_rmse: val["val_force_rmse"].as_f64(),
                        lr: val["lr"].as_f64(),
                        time_sec: val["time_sec"].as_f64(),
                    };
                    loss_history.push(record.clone());

                    // 推送 loss 事件到前端
                    let _ = app.emit("training-loss", &record);
                }
                Some("result") => {
                    final_result = serde_json::from_value(val).ok();
                }
                Some("status") => {
                    let _ = app.emit("training-status", &val);
                }
                _ => {}
            }
        }
    }

    final_result.ok_or_else(|| "训练未返回结果".to_string())
}

/// 列出训练检查点
#[tauri::command]
pub async fn list_training_checkpoints(
    output_dir: String,
) -> Result<Vec<CheckpointInfo>, String> {
    let script = make_list_checkpoints_script(&output_dir);
    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(30),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Ok(vec![]);
    }

    parse_json_output(&result.stdout)
        .unwrap_or_else(|| vec![])
        .pipe(Ok)
}

// ============================================================================
// 辅助函数
// ============================================================================

fn parse_json_output<T: serde::de::DeserializeOwned>(stdout: &str) -> Option<T> {
    for line in stdout.lines().rev() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Ok(val) = serde_json::from_str::<T>(trimmed) {
            return Some(val);
        }
    }
    None
}

trait Pipe: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}

impl<T> Pipe for T {}
