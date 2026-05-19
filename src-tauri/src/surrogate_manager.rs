/**
 * V4.4-005: 多尺度 Surrogate 管理器
 *
 * 通过 Python 桥接调用 caelab.models.surrogate，
 * 管理 UNet 分割和 MLP 代理模型的训练与推理。
 */

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::python_bridge::{run_python_script_async, PythonScriptRequest};

// ============================================================================
// 数据结构
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrogateTrainingConfig {
    pub model_type: String,  // "unet" | "mlp"
    pub epochs: u32,
    pub learning_rate: f64,
    pub batch_size: u32,
    pub training_data_path: Option<String>,
    pub features_path: Option<String>,
    pub targets_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrogateTrainingResult {
    pub success: bool,
    pub model_type: String,
    pub final_loss: f64,
    pub final_iou: Option<f64>,
    pub loss_history: Vec<SurrogateLossRecord>,
    pub is_mock: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrogateLossRecord {
    pub epoch: u32,
    pub loss: f64,
    pub iou: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentationRequest {
    pub image_id: String,
    pub image_base64: Option<String>,
    pub image_path: Option<String>,
    pub segmentation_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentationResult {
    pub phase_fractions: std::collections::HashMap<String, f64>,
    pub porosity: f64,
    pub average_grain_size_um: f64,
    pub grain_size_std_um: f64,
    pub iou_score: f64,
    pub is_mock: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroPropertyPrediction {
    pub property_name: String,
    pub predicted_value: f64,
    pub unit: String,
    pub uncertainty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroPropertyResult {
    pub predictions: Vec<MacroPropertyPrediction>,
    pub is_mock: bool,
}

// ============================================================================
// Python 脚本
// ============================================================================

fn make_train_unet_script(epochs: u32) -> String {
    format!(
        r#"
import json, sys, numpy as np
sys.path.insert(0, ".")
from caelab.models.surrogate import SegmentationModel
model = SegmentationModel()
images = np.random.random((10, 64, 64)).astype(np.float32)
masks = np.random.randint(0, 3, (10, 64, 64)).astype(np.int64)
result = model.train(images, masks, epochs={epochs})
print(json.dumps({{"type": "result", **result}}, default=str))
"#,
        epochs = epochs,
    )
}

fn make_train_mlp_script(config: &SurrogateTrainingConfig) -> String {
    let features_path = config
        .features_path
        .as_deref()
        .unwrap_or("null");
    let targets_path = config
        .targets_path
        .as_deref()
        .unwrap_or("null");

    format!(
        r#"
import json, sys, numpy as np
sys.path.insert(0, ".")
from caelab.models.surrogate import SurrogateMLP
if "{feat}" != "null" and "{tgt}" != "null":
    features = np.array(json.load(open("{feat}")))
    targets = np.array(json.load(open("{tgt}")))
else:
    features = np.random.random((100, 128))
    targets = np.random.random((100, 6)) * np.array([180e9, 350e6, 25.0, 1.2e7, 3.5e9, 50.0])
model = SurrogateMLP(input_dim=features.shape[1], output_dim=targets.shape[1])
result = model.train(features, targets, epochs={epochs}, lr={lr}, batch_size={bs})
print(json.dumps({{"type": "result", **result}}, default=str))
"#,
        feat = features_path,
        tgt = targets_path,
        epochs = config.epochs,
        lr = config.learning_rate,
        bs = config.batch_size,
    )
}

fn make_predict_script(features_json: &str) -> String {
    format!(
        r#"
import json, sys, numpy as np
sys.path.insert(0, ".")
from caelab.models.surrogate import SurrogateMLP
features = np.array({features})
model = SurrogateMLP()
result = model.predict(features)
print(json.dumps(result, default=str))
"#,
        features = features_json,
    )
}

// ============================================================================
// Tauri 命令
// ============================================================================

/// 训练 Surrogate 模型（UNet 或 MLP）
#[tauri::command]
pub async fn train_surrogate_model(
    app: AppHandle,
    config: SurrogateTrainingConfig,
) -> Result<SurrogateTrainingResult, String> {
    let script = match config.model_type.as_str() {
        "unet" => make_train_unet_script(config.epochs),
        "mlp" => make_train_mlp_script(&config),
        _ => return Err(format!("不支持的模型类型: {}，支持 unet / mlp", config.model_type)),
    };

    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(3600),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Err(format!("训练失败: {}", result.stderr));
    }

    // 解析 stdout 中的 loss 事件和最终结果
    let mut loss_history = Vec::new();
    let mut final_result: Option<SurrogateTrainingResult> = None;

    for line in result.stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
            match val["type"].as_str() {
                Some("loss") => {
                    let record = SurrogateLossRecord {
                        epoch: val["epoch"].as_u64().unwrap_or(0) as u32,
                        loss: val["loss"].as_f64().unwrap_or(0.0),
                        iou: val["iou"].as_f64(),
                    };
                    loss_history.push(record.clone());
                    let _ = app.emit("surrogate-loss", &record);
                }
                Some("result") => {
                    final_result = Some(SurrogateTrainingResult {
                        success: val["success"].as_bool().unwrap_or(false),
                        model_type: config.model_type.clone(),
                        final_loss: val["final_loss"].as_f64().unwrap_or(0.0),
                        final_iou: val["final_iou"].as_f64(),
                        loss_history: loss_history.clone(),
                        is_mock: val["is_mock"].as_bool().unwrap_or(false),
                    });
                }
                _ => {}
            }
        }
    }

    final_result.ok_or_else(|| "训练未返回结果".to_string())
}

/// Surrogate 模型推理：微观结构特征 → 宏观性能
#[tauri::command]
pub async fn surrogate_predict(
    features: Vec<f64>,
) -> Result<MacroPropertyResult, String> {
    let features_json = serde_json::to_string(&features).unwrap();
    let script = make_predict_script(&features_json);
    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(60),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Err(format!("推理失败: {}", result.stderr));
    }

    parse_json_output(&result.stdout)
        .ok_or_else(|| "解析推理结果失败".to_string())
}

// ============================================================================
// 辅助
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
