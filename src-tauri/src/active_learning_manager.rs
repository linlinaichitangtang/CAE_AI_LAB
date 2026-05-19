/**
 * V4.4-006: 主动学习管理器
 *
 * 通过 Python 桥接调用 caelab.models.active_learning，
 * 实现真实不确定性计算（MC Dropout / Ensemble）和候选结构选择。
 */

use serde::{Deserialize, Serialize};

use crate::python_bridge::{run_python_script_async, PythonScriptRequest};

// ============================================================================
// 数据结构
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyRequest {
    pub method: String,  // "mc_dropout" | "ensemble"
    pub model_name: Option<String>,
    pub model_names: Option<Vec<String>>,
    pub positions: Vec<Vec<f64>>,
    pub atomic_numbers: Vec<i32>,
    pub cell: Option<Vec<Vec<f64>>>,
    pub n_samples: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyResult {
    pub uncertainty: f64,
    pub energy_mean: f64,
    pub energy_std: f64,
    pub force_std_mean: f64,
    pub method: String,
    pub is_mock: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateSelectionRequest {
    pub candidates: Vec<serde_json::Value>,
    pub num_select: u32,
    pub strategy: String,
}

// ============================================================================
// Python 脚本
// ============================================================================

fn make_mc_dropout_script(req: &UncertaintyRequest) -> String {
    let atoms = build_atoms_json(req);
    let model = req.model_name.as_deref().unwrap_or("chgnet");
    let n = req.n_samples.unwrap_or(10);

    format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.models.active_learning import compute_mc_dropout_uncertainty
result = compute_mc_dropout_uncertainty("{model}", {atoms}, {n})
print(json.dumps(result, default=str))
"#,
        model = model,
        atoms = atoms,
        n = n,
    )
}

fn make_ensemble_script(req: &UncertaintyRequest) -> String {
    let atoms = build_atoms_json(req);
    let models = req
        .model_names
        .as_ref()
        .map(|m| serde_json::to_string(m).unwrap())
        .unwrap_or_else(|| r#"["chgnet", "m3gnet", "mace"]"#.to_string());

    format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.models.active_learning import compute_ensemble_uncertainty
result = compute_ensemble_uncertainty({models}, {atoms})
print(json.dumps(result, default=str))
"#,
        models = models,
        atoms = atoms,
    )
}

fn make_select_script(req: &CandidateSelectionRequest) -> String {
    let cands = serde_json::to_string(&req.candidates).unwrap();

    format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.models.active_learning import select_candidates
result = select_candidates({cands}, {n}, "{strategy}")
print(json.dumps(result, default=str))
"#,
        cands = cands,
        n = req.num_select,
        strategy = req.strategy,
    )
}

fn build_atoms_json(req: &UncertaintyRequest) -> String {
    let mut map = serde_json::Map::new();
    map.insert("positions".to_string(), serde_json::to_value(&req.positions).unwrap());
    map.insert("atomic_numbers".to_string(), serde_json::to_value(&req.atomic_numbers).unwrap());
    if let Some(ref cell) = req.cell {
        map.insert("cell".to_string(), serde_json::to_value(cell).unwrap());
    }
    serde_json::to_string(&map).unwrap()
}

// ============================================================================
// Tauri 命令
// ============================================================================

/// 计算单个结构的不确定性
#[tauri::command]
pub async fn compute_structure_uncertainty(
    request: UncertaintyRequest,
) -> Result<UncertaintyResult, String> {
    let script = match request.method.as_str() {
        "mc_dropout" => make_mc_dropout_script(&request),
        "ensemble" => make_ensemble_script(&request),
        _ => return Err(format!("不支持的不确定性方法: {}，支持 mc_dropout / ensemble", request.method)),
    };

    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(300),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Err(format!("不确定性计算失败: {}", result.stderr));
    }

    parse_json_output(&result.stdout)
        .ok_or_else(|| "解析不确定性结果失败".to_string())
}

/// 根据不确定性选择候选结构
#[tauri::command]
pub async fn al_select_candidates(
    request: CandidateSelectionRequest,
) -> Result<Vec<serde_json::Value>, String> {
    let script = make_select_script(&request);
    let result = run_python_script_async(&PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(60),
        use_temp_file_for_input: None,
    })
    .await?;

    if !result.success {
        return Err(format!("候选选择失败: {}", result.stderr));
    }

    parse_json_output(&result.stdout)
        .ok_or_else(|| "解析候选列表失败".to_string())
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
