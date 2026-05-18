/**
 * V4.4-004: 实验跟踪管理器
 *
 * 通过 Python 桥接调用 caelab.models.experiment_tracker，
 * 支持 W&B 和 MLflow 的实验记录、指标查询和 artifact 管理。
 */

use serde::{Deserialize, Serialize};

use crate::python_bridge::{run_python_script_async, PythonScriptRequest};

// ============================================================================
// 数据结构
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackerStatus {
    pub tracker: String,
    pub project: String,
    pub name: String,
    pub active: bool,
    pub backend: Option<String>,
    pub api_key_set: Option<bool>,
    pub tracking_uri: Option<String>,
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTrackerRequest {
    pub backend: String,
    pub project: String,
    pub name: Option<String>,
    pub config: Option<serde_json::Value>,
    pub api_key: Option<String>,
    pub tracking_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMetricsRequest {
    pub tracker_name: String,
    pub metrics: serde_json::Value,
    pub step: Option<u32>,
}

// ============================================================================
// Python 脚本生成
// ============================================================================

fn make_create_script(req: &CreateTrackerRequest) -> String {
    let kwargs = match (&req.api_key, &req.tracking_uri) {
        (Some(key), Some(uri)) => format!("api_key='{}', tracking_uri='{}'", key, uri),
        (Some(key), None) => format!("api_key='{}'", key),
        (None, Some(uri)) => format!("tracking_uri='{}'", uri),
        (None, None) => String::new(),
    };
    let config_json = req
        .config
        .as_ref()
        .map(|v| v.to_string())
        .unwrap_or_else(|| "None".to_string());
    let name_arg = req
        .name
        .as_ref()
        .map(|n| format!("name='{}'", n))
        .unwrap_or_else(|| "name=None".to_string());

    format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.models.experiment_tracker import create_tracker
tracker = create_tracker("{backend}", project="{project}", {name_arg}, config={config_json}, {kwargs})
print(json.dumps(tracker.status(), ensure_ascii=False))
"#,
        backend = req.backend,
        project = req.project,
        name_arg = name_arg,
        config_json = config_json,
        kwargs = kwargs,
    )
}

fn make_log_metrics_script(req: &LogMetricsRequest) -> String {
    let step_arg = req
        .step
        .map(|s| s.to_string())
        .unwrap_or_else(|| "None".to_string());

    format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.models.experiment_tracker import log_training_loss
log_training_loss("{name}", {step}, {metrics})
print(json.dumps({{"success": True}}))
"#,
        name = req.tracker_name,
        step = step_arg,
        metrics = req.metrics,
    )
}

fn make_list_script() -> String {
    r#"
import json, sys
sys.path.insert(0, ".")
from caelab.models.experiment_tracker import list_trackers
print(json.dumps(list_trackers(), ensure_ascii=False))
"#
    .to_string()
}

// ============================================================================
// Tauri 命令
// ============================================================================

/// 创建实验跟踪器
#[tauri::command]
pub async fn create_experiment_tracker(
    request: CreateTrackerRequest,
) -> Result<TrackerStatus, String> {
    let script = make_create_script(&request);
    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(60),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Err(format!("创建跟踪器失败: {}", result.stderr));
    }

    parse_json_output(&result.stdout)
        .ok_or_else(|| "解析跟踪器状态失败".to_string())
}

/// 记录训练指标
#[tauri::command]
pub async fn log_experiment_metrics(
    request: LogMetricsRequest,
) -> Result<serde_json::Value, String> {
    let script = make_log_metrics_script(&request);
    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(30),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Err(format!("记录指标失败: {}", result.stderr));
    }

    Ok(serde_json::json!({"success": true}))
}

/// 列出所有活跃跟踪器
#[tauri::command]
pub async fn list_experiment_trackers() -> Result<Vec<TrackerStatus>, String> {
    let script = make_list_script();
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

trait Pipe: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}

impl<T> Pipe for T {}
