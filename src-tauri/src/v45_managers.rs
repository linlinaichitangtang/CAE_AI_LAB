/**
 * V4.5-002: OpenKIM 力场库管理器
 * V4.5-003: 微结构演化管理器 (MC + PF + CPFEM)
 * V4.5-004: FE² 多尺度并发管理器
 * V4.5-005: 不确定性量化管理器
 * V4.5-006: 材料数据库管理器
 */

use serde::{Deserialize, Serialize};

use crate::python_bridge::{run_python_script_async, PythonScriptRequest};

// ============================================================================
// V4.5-002: OpenKIM 数据结构
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotentialInfo {
    pub kim_id: String,
    pub name: String,
    pub model_type: String,
    pub elements: Vec<String>,
    pub properties: Vec<String>,
    pub verified_tests: u32,
    pub citation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotSearchRequest {
    pub elements: Option<Vec<String>>,
    pub model_type: Option<String>,
    pub material: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotVerifyResult {
    pub valid: bool,
    pub kim_id: String,
    pub target_elements: Vec<String>,
    pub supported_elements: Vec<String>,
    pub missing_elements: Vec<String>,
    pub warning: Option<String>,
}

// ============================================================================
// V4.5-003: 微结构数据结构
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrostructureConfig {
    pub nx: Option<u32>,
    pub ny: Option<u32>,
    pub n_grains: Option<u32>,
    pub temperature: Option<f64>,
    pub mc_steps: Option<u32>,
    pub pf_steps: Option<u32>,
}

// ============================================================================
// V4.5-004: FE² 数据结构
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FE2Config {
    pub n_rve: u32,
    pub n_steps: Option<u32>,
    pub method: Option<String>,
    pub n_workers: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FE2Result {
    pub n_rve: u32,
    pub method: String,
    pub homogenized_elastic_modulus_GPa: f64,
    pub stress_strain_curve: serde_json::Value,
}

// ============================================================================
// V4.5-005: UQ 数据结构
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KpointConvergenceRequest {
    pub energies: Vec<f64>,
    pub kpoint_grids: Vec<Vec<u32>>,
    pub n_atoms: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BayesianUQRequest {
    pub predictions: Vec<f64>,
    pub reference: Option<f64>,
}

// ============================================================================
// V4.5-006: 材料搜索数据结构
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialSearchQuery {
    pub elements: Option<Vec<String>>,
    pub formula: Option<String>,
    pub source: Option<String>,
    pub min_band_gap: Option<f64>,
    pub max_band_gap: Option<f64>,
    pub limit: Option<u32>,
}

// ============================================================================
// V4.5-002: OpenKIM Tauri 命令
// ============================================================================

#[tauri::command]
pub async fn search_kim_potentials(
    request: PotSearchRequest,
) -> Result<Vec<PotentialInfo>, String> {
    let elements_json = request.elements.map(|e| serde_json::to_string(&e).unwrap())
        .unwrap_or_else(|| "null".to_string());
    let mt = request.model_type.as_deref().unwrap_or("null");

    let script = format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.dft.openkim import search_potentials
result = search_potentials(elements={elements}, model_type={mt}, material={material})
print(json.dumps(result, ensure_ascii=False))
"#,
        elements = elements_json,
        mt = if mt == "null" { "None".to_string() } else { format!("'{}'", mt) },
        material = request.material.as_ref().map(|m| format!("'{}'", m)).unwrap_or_else(|| "None".to_string()),
    );

    let result = run_python_script_async(&PythonScriptRequest {
        script, input_data: None, timeout_sec: Some(30), use_temp_file_for_input: None,
    }).await?;

    parse_json(&result.stdout).ok_or_else(|| "解析势函数列表失败".to_string())
}

#[tauri::command]
pub async fn verify_kim_potential(
    kim_id: String, elements: Vec<String>,
) -> Result<PotVerifyResult, String> {
    let elem_json = serde_json::to_string(&elements).unwrap();
    let script = format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.dft.openkim import verify_potential
result = verify_potential("{id}", {elements})
print(json.dumps(result, ensure_ascii=False))
"#, id = kim_id, elements = elem_json,
    );

    let result = run_python_script_async(&PythonScriptRequest {
        script, input_data: None, timeout_sec: Some(30), use_temp_file_for_input: None,
    }).await?;

    parse_json(&result.stdout).ok_or_else(|| "解析验证结果失败".to_string())
}

// ============================================================================
// V4.5-003: 微结构 Tauri 命令
// ============================================================================

#[tauri::command]
pub async fn run_microstructure_chain(
    config: MicrostructureConfig,
) -> Result<serde_json::Value, String> {
    let cfg = serde_json::to_string(&config).unwrap();
    let script = format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.dft.microstructure import run_mc_pf_cpfem_chain
result = run_mc_pf_cpfem_chain({cfg})
print(json.dumps(result, default=str))
"#, cfg = cfg,
    );

    let result = run_python_script_async(&PythonScriptRequest {
        script, input_data: None, timeout_sec: Some(600), use_temp_file_for_input: None,
    }).await?;

    // 提取最终结果
    for line in result.stdout.lines().rev() {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(line.trim()) {
            if val.get("success").is_some() {
                return Ok(val);
            }
        }
    }
    Err("微结构链未返回结果".to_string())
}

// ============================================================================
// V4.5-004: FE² Tauri 命令
// ============================================================================

#[tauri::command]
pub async fn run_fe2_multiscale(
    config: FE2Config,
) -> Result<FE2Result, String> {
    let n_steps = config.n_steps.unwrap_or(10);
    let method = config.method.as_deref().unwrap_or("md");

    let script = format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.dft.fe2 import FE2Solver
solver = FE2Solver(n_rve={n_rve}, method="{method}", n_workers={workers})
result = solver.run(macro_steps={steps})
print(json.dumps(result, default=str))
"#,
        n_rve = config.n_rve, method = method,
        workers = config.n_workers.unwrap_or(4),
        steps = n_steps,
    );

    let result = run_python_script_async(&PythonScriptRequest {
        script, input_data: None, timeout_sec: Some(1800), use_temp_file_for_input: None,
    }).await?;

    for line in result.stdout.lines().rev() {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(line.trim()) {
            if val.get("homogenized_elastic_modulus_GPa").is_some() {
                return Ok(FE2Result {
                    n_rve: val["n_rve"].as_u64().unwrap_or(0) as u32,
                    method: method.to_string(),
                    homogenized_elastic_modulus_GPa: val["homogenized_elastic_modulus_GPa"].as_f64().unwrap_or(0.0),
                    stress_strain_curve: val["stress_strain_curve"].clone(),
                });
            }
        }
    }
    Err("FE² 求解未返回结果".to_string())
}

// ============================================================================
// V4.5-005: UQ Tauri 命令
// ============================================================================

#[tauri::command]
pub async fn check_kpoint_convergence_cmd(
    request: KpointConvergenceRequest,
) -> Result<serde_json::Value, String> {
    let energies = serde_json::to_string(&request.energies).unwrap();
    let grids = serde_json::to_string(&request.kpoint_grids).unwrap();
    let n_atoms = request.n_atoms.unwrap_or(1);

    let script = format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.dft.uq import check_kpoint_convergence
result = check_kpoint_convergence({energies}, {grids}, n_atoms={n_atoms})
print(json.dumps(result, default=str))
"#, energies = energies, grids = grids, n_atoms = n_atoms,
    );

    let result = run_python_script_async(&PythonScriptRequest {
        script, input_data: None, timeout_sec: Some(30), use_temp_file_for_input: None,
    }).await?;

    parse_json(&result.stdout).ok_or_else(|| "k-point 收敛检测失败".to_string())
}

#[tauri::command]
pub async fn run_bayesian_uq(
    request: BayesianUQRequest,
) -> Result<serde_json::Value, String> {
    let preds = serde_json::to_string(&request.predictions).unwrap();
    let ref_arg = request.reference.map(|r| r.to_string()).unwrap_or_else(|| "None".to_string());

    let script = format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.dft.uq import bayesian_uq
result = bayesian_uq({preds}, {ref_arg})
print(json.dumps(result, default=str))
"#, preds = preds, ref_arg = ref_arg,
    );

    let result = run_python_script_async(&PythonScriptRequest {
        script, input_data: None, timeout_sec: Some(60), use_temp_file_for_input: None,
    }).await?;

    parse_json(&result.stdout).ok_or_else(|| "贝叶斯 UQ 失败".to_string())
}

// ============================================================================
// V4.5-006: 材料数据库 Tauri 命令
// ============================================================================

#[tauri::command]
pub async fn search_materials_db(
    query: MaterialSearchQuery,
) -> Result<serde_json::Value, String> {
    let q = serde_json::to_string(&query).unwrap();
    let script = format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.dft.materials import search_materials
result = search_materials(**{query})
print(json.dumps(result, ensure_ascii=False))
"#, query = q,
    );

    let result = run_python_script_async(&PythonScriptRequest {
        script, input_data: None, timeout_sec: Some(30), use_temp_file_for_input: None,
    }).await?;

    parse_json(&result.stdout).ok_or_else(|| "材料搜索失败".to_string())
}

#[tauri::command]
pub async fn batch_download_materials(
    material_ids: Vec<String>,
) -> Result<serde_json::Value, String> {
    let ids = serde_json::to_string(&material_ids).unwrap();
    let script = format!(
        r#"
import json, sys
sys.path.insert(0, ".")
from caelab.dft.materials import batch_download
result = batch_download({ids})
print(json.dumps(result, ensure_ascii=False))
"#, ids = ids,
    );

    let result = run_python_script_async(&PythonScriptRequest {
        script, input_data: None, timeout_sec: Some(60), use_temp_file_for_input: None,
    }).await?;

    parse_json(&result.stdout).ok_or_else(|| "批量下载失败".to_string())
}

// ============================================================================
// 辅助
// ============================================================================

fn parse_json<T: serde::de::DeserializeOwned>(stdout: &str) -> Option<T> {
    for line in stdout.lines().rev() {
        if let Ok(val) = serde_json::from_str::<T>(line.trim()) {
            return Some(val);
        }
    }
    None
}
