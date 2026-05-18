/**
 * V4.4-002: 预训练模型仓库管理器
 *
 * 通过 Python 桥接调用 caelab.models.inference 模块，
 * 管理 M3GNet/CHGNet/MACE/NequIP/SevenNet 的下载、状态查询和推理。
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::python_bridge::{run_python_script_async, PythonScriptRequest};

// ============================================================================
// 数据结构
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelStatus {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub model_type: String,
    pub framework: String,
    pub installed: bool,
    pub cached: bool,
    pub package: String,
    pub version: String,
    pub model_size_mb: f64,
    pub supported_elements: Vec<String>,
    pub supported_properties: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictRequest {
    pub model_name: String,
    pub positions: Vec<Vec<f64>>,
    pub atomic_numbers: Vec<i32>,
    pub cell: Option<Vec<Vec<f64>>>,
    pub properties: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictResult {
    pub model_name: String,
    pub is_mock: bool,
    pub energy: Option<f64>,
    pub forces: Option<Vec<Vec<f64>>>,
    pub stress: Option<Vec<f64>>,
    pub inference_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOnnxResult {
    pub success: bool,
    pub output_path: Option<String>,
    pub model_name: Option<String>,
    pub file_size_mb: Option<f64>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallModelResult {
    pub success: bool,
    pub package: String,
    pub version: Option<String>,
    pub message: String,
}

// ============================================================================
// Python 推理脚本模板
// ============================================================================

fn make_list_script() -> String {
    r#"
import json, sys
sys.path.insert(0, sys.argv[1] if len(sys.argv) > 1 else ".")
from caelab.models.inference import list_model_status
print(json.dumps(list_model_status(), ensure_ascii=False))
"#
    .to_string()
}

fn make_predict_script(model_name: &str, atoms_json: &str, props_json: &str) -> String {
    format!(
        r#"
import json, sys
sys.path.insert(0, sys.argv[1] if len(sys.argv) > 1 else ".")
from caelab.models.inference import predict_structure
result = predict_structure("{model}", {atoms}, {props})
print(json.dumps(result, ensure_ascii=False, default=str))
"#,
        model = model_name,
        atoms = atoms_json,
        props = props_json,
    )
}

fn make_status_script(model_name: &str) -> String {
    format!(
        r#"
import json, sys
sys.path.insert(0, sys.argv[1] if len(sys.argv) > 1 else ".")
from caelab.models.inference import get_model_status
print(json.dumps(get_model_status("{model}"), ensure_ascii=False))
"#,
        model = model_name,
    )
}

fn make_install_script(package: &str) -> String {
    format!(
        r#"
import subprocess, json, sys
result = subprocess.run(
    [sys.executable, "-m", "pip", "install", "{package}"],
    capture_output=True, text=True, timeout=600,
)
print(json.dumps({{"success": result.returncode == 0, "message": result.stdout[-500:] if result.stdout else result.stderr[-500:]}}, ensure_ascii=False))
"#,
        package = package,
    )
}

fn make_export_onnx_script(model_name: &str, output_path: &str) -> String {
    format!(
        r#"
import json, sys
sys.path.insert(0, sys.argv[1] if len(sys.argv) > 1 else ".")
from caelab.models.inference import export_model_onnx
result = export_model_onnx("{model}", "{path}")
print(json.dumps(result, ensure_ascii=False))
"#,
        model = model_name,
        path = output_path,
    )
}

// ============================================================================
// 辅助：构建 atoms JSON
// ============================================================================

fn build_atoms_json(req: &PredictRequest) -> String {
    let mut map = serde_json::Map::new();
    map.insert(
        "positions".to_string(),
        serde_json::to_value(&req.positions).unwrap(),
    );
    map.insert(
        "atomic_numbers".to_string(),
        serde_json::to_value(&req.atomic_numbers).unwrap(),
    );
    if let Some(ref cell) = req.cell {
        map.insert("cell".to_string(), serde_json::to_value(cell).unwrap());
    }
    serde_json::to_string(&map).unwrap()
}

// ============================================================================
// Tauri 命令
// ============================================================================

/// 列出所有模型及其状态
#[tauri::command]
pub async fn list_model_repo() -> Result<Vec<ModelStatus>, String> {
    let script = make_list_script();
    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(60),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Err(format!(
            "获取模型列表失败: {}",
            if result.stderr.is_empty() { &result.stdout } else { &result.stderr }
        ));
    }

    parse_json_output(&result.stdout)
        .ok_or_else(|| "解析模型列表失败".to_string())
}

/// 获取单个模型状态
#[tauri::command]
pub async fn get_model_repo_status(model_name: String) -> Result<ModelStatus, String> {
    let script = make_status_script(&model_name);
    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(30),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Err(format!("查询模型状态失败: {}", result.stderr));
    }

    parse_json_output(&result.stdout)
        .ok_or_else(|| "解析模型状态失败".to_string())
}

/// 安装模型依赖包
#[tauri::command]
pub async fn install_model_package(package: String) -> Result<InstallModelResult, String> {
    let script = make_install_script(&package);
    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(600),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Ok(InstallModelResult {
            success: false,
            package,
            version: None,
            message: result.stderr,
        });
    }

    let parsed: serde_json::Value = parse_json_output(&result.stdout)
        .unwrap_or(serde_json::Value::Null);

    Ok(InstallModelResult {
        success: parsed["success"].as_bool().unwrap_or(false),
        package,
        version: parsed["version"].as_str().map(|s| s.to_string()),
        message: parsed["message"].as_str().unwrap_or("").to_string(),
    })
}

/// 使用预训练模型进行推理
#[tauri::command]
pub async fn model_predict(request: PredictRequest) -> Result<PredictResult, String> {
    let atoms_json = build_atoms_json(&request);
    let props_json = match &request.properties {
        Some(props) => serde_json::to_string(props).unwrap(),
        None => "null".to_string(),
    };

    let script = make_predict_script(&request.model_name, &atoms_json, &props_json);
    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(300),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Err(format!("推理失败: {}", result.stderr));
    }

    parse_json_output(&result.stdout)
        .ok_or_else(|| "解析推理结果失败".to_string())
}

/// 导出模型为 ONNX 格式
#[tauri::command]
pub async fn export_model_to_onnx(
    model_name: String,
    output_path: String,
) -> Result<ExportOnnxResult, String> {
    let script = make_export_onnx_script(&model_name, &output_path);
    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(300),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Ok(ExportOnnxResult {
            success: false,
            output_path: None,
            model_name: Some(model_name),
            file_size_mb: None,
            error: Some(result.stderr),
        });
    }

    parse_json_output(&result.stdout)
        .ok_or_else(|| "解析导出结果失败".to_string())
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
