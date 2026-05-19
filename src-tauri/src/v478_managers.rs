/**
 * V4.7 材料热力学 + V4.8 产品化 Tauri 命令
 */
use serde::{Deserialize, Serialize};
use crate::python_bridge::{run_python_script_async, PythonScriptRequest};

async fn py_val(script: String) -> Result<serde_json::Value, String> {
    let req = PythonScriptRequest { script, input_data: None, timeout_sec: Some(120), use_temp_file_for_input: None };
    let res = run_python_script_async(&req).await?;
    if !res.success { return Err(res.stderr); }
    for line in res.stdout.lines().rev() {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(line.trim()) { return Ok(v); }
    }
    Err("parse failed".into())
}

async fn py_strs(script: String) -> Result<Vec<String>, String> {
    let req = PythonScriptRequest { script, input_data: None, timeout_sec: Some(120), use_temp_file_for_input: None };
    let res = run_python_script_async(&req).await?;
    if !res.success { return Err(res.stderr); }
    for line in res.stdout.lines().rev() {
        if let Ok(v) = serde_json::from_str::<Vec<String>>(line.trim()) { return Ok(v); }
    }
    Err("parse failed".into())
}

#[tauri::command] pub async fn calphad_elements() -> Result<Vec<String>, String> { py_strs("from caelab.thermo.calphad import list_elements; import json; print(json.dumps(list_elements()))".into()).await }
#[tauri::command] pub async fn thermophysical_list() -> Result<Vec<String>, String> { py_strs("from caelab.thermo.thermophysical import list_materials; import json; print(json.dumps(list_materials()))".into()).await }

#[tauri::command] pub async fn calphad_gibbs(e: String, p: String, t: f64) -> Result<serde_json::Value, String> { py_val(format!("from caelab.thermo.calphad import gibbs_pure; import json; print(json.dumps({{'G':gibbs_pure('{}','{}',{})}}))", e, p, t)).await }
#[tauri::command] pub async fn calphad_curve(c1: String, c2: String, ph: String, t: f64) -> Result<serde_json::Value, String> { py_val(format!("from caelab.thermo.calphad import get_gibbs_curve; import json; print(json.dumps(get_gibbs_curve(['{}','{}'],'{}',{})))", c1, c2, ph, t)).await }
#[tauri::command] pub async fn phase_diagram_binary(c1: String, c2: String, phases: Vec<String>, tmin: f64, tmax: f64) -> Result<serde_json::Value, String> { let ps = serde_json::to_string(&phases).unwrap(); py_val(format!("from caelab.thermo.phase_diagram import binary_phase_diagram; import json; print(json.dumps(binary_phase_diagram('{}','{}',{},({},{}))))", c1, c2, ps, tmin, tmax)).await }
#[tauri::command] pub async fn phase_diagram_isothermal(c1: String, c2: String, phases: Vec<String>, t: f64) -> Result<serde_json::Value, String> { let ps = serde_json::to_string(&phases).unwrap(); py_val(format!("from caelab.thermo.phase_diagram import isothermal_section; import json; print(json.dumps(isothermal_section('{}','{}',{},{})))", c1, c2, ps, t)).await }
#[tauri::command] pub async fn scheil_simulate(c1: String, c2: String, x0: f64, tl: f64, ts: f64, k: f64) -> Result<serde_json::Value, String> { py_val(format!("from caelab.thermo.scheil import scheil_solidification; import json; print(json.dumps(scheil_solidification(['{}','{}'],[{},{:.4}],['FCC_A1','LIQUID'],{},{},{})))", c1, c2, x0, 1.0-x0, tl, ts, k)).await }
#[tauri::command] pub async fn diffusion_simulate(l: f64, n: u32, t: f64, d: f64, cl: f64, cr: f64) -> Result<serde_json::Value, String> { py_val(format!("from caelab.thermo.diffusion import diffusion_couple; import json; print(json.dumps(diffusion_couple({},{},{},{},{},{})))", l, n, t, d, cl, cr)).await }
#[tauri::command] pub async fn kwn_precipitate(t: f64, c0: f64, ce: f64, tt: f64) -> Result<serde_json::Value, String> { py_val(format!("from caelab.thermo.precipitation import kwn_precipitation; import json; print(json.dumps(kwn_precipitation(T={},c0={},ceq={},t_total={})))", t, c0, ce, tt)).await }
#[tauri::command] pub async fn thermophysical_get(m: String, t: f64) -> Result<serde_json::Value, String> { py_val(format!("from caelab.thermo.thermophysical import get_all_properties; import json; print(json.dumps(get_all_properties('{}',{})))", m, t)).await }
#[tauri::command] pub async fn kinetics_isothermal(k: f64, n: f64) -> Result<serde_json::Value, String> { py_val(format!("from caelab.thermo.kinetics import jmak_isothermal; import json; print(json.dumps(jmak_isothermal(k={},n={})))", k, n)).await }
#[tauri::command] pub async fn kinetics_avrami(x: Vec<f64>, y: Vec<f64>) -> Result<serde_json::Value, String> { let xs = serde_json::to_string(&x).unwrap(); let ys = serde_json::to_string(&y).unwrap(); py_val(format!("from caelab.thermo.kinetics import fit_avrami; import json; print(json.dumps(fit_avrami({},{})))", xs, ys)).await }
#[tauri::command] pub async fn defects_concentration(e: String, tmin: f64, tmax: f64) -> Result<serde_json::Value, String> { py_val(format!("from caelab.thermo.defects import vacancy_concentration; import json; print(json.dumps(vacancy_concentration('{}',({},{}))))", e, tmin, tmax)).await }

#[tauri::command] pub async fn wizard_steps() -> Result<serde_json::Value, String> { py_val("from caelab.product.tools import get_wizard_steps; import json; print(json.dumps(get_wizard_steps(), ensure_ascii=False))".into()).await }
#[tauri::command] pub async fn cases_list(cat: Option<String>) -> Result<serde_json::Value, String> {
    let c = cat.unwrap_or_default();
    if c.is_empty() || c == "all" { py_val("from caelab.product.tools import list_cases; import json; print(json.dumps(list_cases(), ensure_ascii=False))".into()).await }
    else { py_val(format!("from caelab.product.tools import list_cases; import json; print(json.dumps(list_cases(category='{}'), ensure_ascii=False))", c)).await }
}
#[tauri::command] pub async fn cases_get(id: String) -> Result<serde_json::Value, String> { py_val(format!("from caelab.product.tools import get_case; import json; print(json.dumps(get_case('{}'), ensure_ascii=False))", id)).await }
#[tauri::command] pub async fn glossary_search(kw: String) -> Result<serde_json::Value, String> { py_val(format!("from caelab.product.tools import lookup_glossary, search_glossary; import json; r=lookup_glossary('{}') or search_glossary('{}'); print(json.dumps(r, ensure_ascii=False))", kw, kw)).await }
#[tauri::command] pub async fn compliance_check(std: String, data: serde_json::Value) -> Result<serde_json::Value, String> { py_val(format!("from caelab.product.tools import check_compliance; import json; print(json.dumps(check_compliance('{}',{}), ensure_ascii=False))", std, data)).await }
#[tauri::command] pub async fn report_generate(name: String, data: serde_json::Value) -> Result<serde_json::Value, String> { py_val(format!("from caelab.product.tools import generate_report; import json; print(json.dumps(generate_report('{}',{}), ensure_ascii=False))", name, data)).await }
