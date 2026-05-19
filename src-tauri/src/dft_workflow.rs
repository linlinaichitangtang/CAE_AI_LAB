/**
 * V4.5-001: DFT 工作流管理器
 *
 * 全链条第一性原理计算：
 *   结构优化 → 电子结构 → 声子谱 → 缺陷形成能
 *
 * 支持 VASP / Quantum ESPRESSO / CP2K
 */

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::python_bridge::{run_python_script_async, PythonScriptRequest};

// ============================================================================
// 数据结构
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DftWorkflowConfig {
    pub code: String,
    pub structure: serde_json::Value,
    pub steps: Option<Vec<String>>,
    pub extra_params: Option<serde_json::Value>,
    pub work_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DftWorkflowResult {
    pub code: String,
    pub success: bool,
    pub steps: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DftCodeInfo {
    pub code: String,
    pub name: String,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefectFormationRequest {
    pub code: String,
    pub bulk_structure: serde_json::Value,
    pub defect_type: String,
    pub defect_site: Vec<f64>,
    pub chemical_potentials: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefectFormationResult {
    pub formation_energy_eV: f64,
    pub bulk_energy_eV: f64,
    pub defect_energy_eV: f64,
    pub is_mock: bool,
}

// ============================================================================
// Python 脚本
// ============================================================================

fn make_workflow_script(config: &DftWorkflowConfig) -> String {
    let structure_json = config.structure.to_string();
    let steps_json = config
        .steps
        .as_ref()
        .map(|s| serde_json::to_string(s).unwrap())
        .unwrap_or_else(|| "null".to_string());
    let work_dir = config
        .work_dir
        .as_deref()
        .unwrap_or("/tmp/caelab_dft");

    format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.dft.workflow import create_workflow
wf = create_workflow("{code}", "{work_dir}")
structure = json.loads('{structure}')
steps = json.loads('{steps}') if '{steps}' != "null" else None
result = wf.run_full_chain(structure, steps)
print(json.dumps(result, default=str))
"#,
        code = config.code,
        work_dir = work_dir,
        structure = structure_json.replace('\'', "\\'"),
        steps = steps_json,
    )
}

fn make_list_codes_script() -> String {
    r#"
import json, sys
sys.path.insert(0, ".")
from caelab.dft.workflow import list_workflows
print(json.dumps(list_workflows(), ensure_ascii=False))
"#
    .to_string()
}

fn make_defect_script(req: &DefectFormationRequest) -> String {
    let bulk_json = req.bulk_structure.to_string();
    let chem_json = req
        .chemical_potentials
        .as_ref()
        .map(|v| v.to_string())
        .unwrap_or_else(|| "{}".to_string());

    format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.dft.workflow import create_workflow
wf = create_workflow("{code}")
bulk = json.loads('{bulk}')
# 构建缺陷结构
defect = dict(bulk)
defect["defect_type"] = "{defect_type}"
defect["defect_site"] = {defect_site}
result = wf.run_full_chain(bulk, ["defect_formation"])
parsed = result["steps"].get("defect_formation", {{}}).get("parsed", {{}})
print(json.dumps(parsed, default=str))
"#,
        code = req.code,
        bulk = bulk_json.replace('\'', "\\'"),
        defect_type = req.defect_type,
        defect_site = serde_json::to_string(&req.defect_site).unwrap(),
    )
}

// ============================================================================
// Tauri 命令
// ============================================================================

/// 运行完整 DFT 工作流
#[tauri::command]
pub async fn run_dft_workflow(
    app: AppHandle,
    config: DftWorkflowConfig,
) -> Result<DftWorkflowResult, String> {
    let script = make_workflow_script(&config);
    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(7200),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Err(format!("DFT 工作流失败: {}", result.stderr));
    }

    // 解析 stdout 中的事件
    let mut final_result: Option<DftWorkflowResult> = None;
    for line in result.stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
            if val["type"].as_str() == Some("result") {
                let _ = app.emit("dft-step-result", &val);
            }
        }
    }

    // 最后一个完整 JSON 是最终结果
    for line in result.stdout.lines().rev() {
        let trimmed = line.trim();
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
            if val.get("code").is_some() {
                final_result = Some(DftWorkflowResult {
                    code: val["code"].as_str().unwrap_or("").to_string(),
                    success: val["success"].as_bool().unwrap_or(false),
                    steps: val["steps"].clone(),
                });
                break;
            }
        }
    }

    final_result.ok_or_else(|| "DFT 工作流未返回结果".to_string())
}

/// 列出可用 DFT 求解器
#[tauri::command]
pub async fn list_dft_codes() -> Result<Vec<DftCodeInfo>, String> {
    let script = make_list_codes_script();
    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(30),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Err(format!("查询 DFT 求解器失败: {}", result.stderr));
    }

    parse_json_output(&result.stdout)
        .ok_or_else(|| "解析求解器列表失败".to_string())
}

/// 计算缺陷形成能
#[tauri::command]
pub async fn compute_defect_formation_energy(
    request: DefectFormationRequest,
) -> Result<DefectFormationResult, String> {
    let script = make_defect_script(&request);
    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(3600),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Err(format!("缺陷形成能计算失败: {}", result.stderr));
    }

    parse_json_output(&result.stdout)
        .ok_or_else(|| "解析缺陷形成能结果失败".to_string())
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
