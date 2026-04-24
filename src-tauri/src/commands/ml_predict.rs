/**
 * V2.5 AI × ML 免仿真预测 - Rust 后端命令
 * ONNX Runtime 集成 + 预训练模型推理 (Mock 模式)
 *
 * 当前实现: Mock 推理引擎，返回基于材料成分的预设预测值
 * 后续接入: ort crate 实际加载 ONNX 模型进行推理
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// 数据结构
// ============================================================================

/// ML 模型预测请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PredictionRequest {
    /// 材料成分 (元素 -> 原子百分比)
    pub composition: HashMap<String, f64>,
    /// 预测目标属性
    pub target_properties: Vec<String>,
    /// 模型名称 (默认 "mock_chgnet")
    #[serde(default = "default_model_name")]
    pub model_name: String,
}

fn default_model_name() -> String {
    "mock_chgnet".to_string()
}

/// 单个属性预测结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyPrediction {
    /// 属性名称 (如 "elastic_modulus", "yield_strength", "thermal_conductivity")
    pub property_name: String,
    /// 预测值
    pub predicted_value: f64,
    /// 单位
    pub unit: String,
    /// 置信区间下界
    pub confidence_lower: f64,
    /// 置信区间上界
    pub confidence_upper: f64,
    /// 预测不确定性 (标准差)
    pub uncertainty: f64,
}

/// ML 模型预测响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PredictionResponse {
    /// 模型名称
    pub model_name: String,
    /// 是否使用 mock 模式
    pub is_mock: bool,
    /// 推理耗时 (ms)
    pub inference_time_ms: u64,
    /// 各属性预测结果
    pub predictions: Vec<PropertyPrediction>,
}

/// 可用模型信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInfo {
    /// 模型名称
    pub name: String,
    /// 模型描述
    pub description: String,
    /// 支持的预测属性
    pub supported_properties: Vec<String>,
    /// 是否已加载
    pub is_loaded: bool,
    /// 模型类型 (onnx / mock)
    pub model_type: String,
}

/// 精度评测请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BenchmarkRequest {
    /// 材料名称列表
    pub material_names: Vec<String>,
    /// 要评测的属性
    pub target_properties: Vec<String>,
}

/// 精度评测结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BenchmarkResult {
    /// 材料名称
    pub material_name: String,
    /// 属性名称
    pub property_name: String,
    /// ML 预测值
    pub predicted_value: f64,
    /// CAELab 仿真值 (参考值)
    pub reference_value: f64,
    /// 绝对误差
    pub absolute_error: f64,
    /// 相对误差 (%)
    pub relative_error_percent: f64,
    /// 是否通过 (< 15%)
    pub passed: bool,
}

// ============================================================================
// Mock 数据: 常见材料的参考属性
// ============================================================================

/// 材料参考属性数据库 (用于 mock 推理和精度评测)
fn get_material_reference_db() -> HashMap<String, HashMap<String, f64>> {
    let mut db = HashMap::new();

    // Q235 结构钢
    let mut q235 = HashMap::new();
    q235.insert("elastic_modulus".to_string(), 210e9);      // Pa
    q235.insert("yield_strength".to_string(), 235e6);       // Pa
    q235.insert("thermal_conductivity".to_string(), 50.0);  // W/(m·K)
    q235.insert("density".to_string(), 7850.0);             // kg/m³
    q235.insert("poissons_ratio".to_string(), 0.3);
    db.insert("Q235".to_string(), q235);

    // 304 不锈钢
    let mut ss304 = HashMap::new();
    ss304.insert("elastic_modulus".to_string(), 193e9);
    ss304.insert("yield_strength".to_string(), 215e6);
    ss304.insert("thermal_conductivity".to_string(), 16.2);
    ss304.insert("density".to_string(), 8000.0);
    ss304.insert("poissons_ratio".to_string(), 0.29);
    db.insert("304不锈钢".to_string(), ss304);

    // Al6061 铝合金
    let mut al6061 = HashMap::new();
    al6061.insert("elastic_modulus".to_string(), 68.9e9);
    al6061.insert("yield_strength".to_string(), 276e6);
    al6061.insert("thermal_conductivity".to_string(), 167.0);
    al6061.insert("density".to_string(), 2700.0);
    al6061.insert("poissons_ratio".to_string(), 0.33);
    db.insert("Al6061".to_string(), al6061);

    // Ti6Al4V 钛合金
    let mut ti64 = HashMap::new();
    ti64.insert("elastic_modulus".to_string(), 113.8e9);
    ti64.insert("yield_strength".to_string(), 880e6);
    ti64.insert("thermal_conductivity".to_string(), 6.7);
    ti64.insert("density".to_string(), 4430.0);
    ti64.insert("poissons_ratio".to_string(), 0.342);
    db.insert("Ti6Al4V".to_string(), ti64);

    // Cu 纯铜
    let mut cu = HashMap::new();
    cu.insert("elastic_modulus".to_string(), 117e9);
    cu.insert("yield_strength".to_string(), 70e6);
    cu.insert("thermal_conductivity".to_string(), 401.0);
    cu.insert("density".to_string(), 8960.0);
    cu.insert("poissons_ratio".to_string(), 0.34);
    db.insert("纯铜".to_string(), cu);

    db
}

/// 属性单位映射
fn get_property_units() -> HashMap<String, String> {
    let mut units = HashMap::new();
    units.insert("elastic_modulus".to_string(), "Pa".to_string());
    units.insert("yield_strength".to_string(), "Pa".to_string());
    units.insert("thermal_conductivity".to_string(), "W/(m·K)".to_string());
    units.insert("density".to_string(), "kg/m³".to_string());
    units.insert("poissons_ratio".to_string(), "".to_string());
    units.insert("shear_modulus".to_string(), "Pa".to_string());
    units.insert("bulk_modulus".to_string(), "Pa".to_string());
    units
}

// ============================================================================
// Mock 推理引擎
// ============================================================================

/// Mock 推理: 基于成分生成合理的预测值
fn mock_predict(composition: &HashMap<String, f64>, property: &str) -> (f64, f64) {
    // 基础值 + 成分加权 + 随机扰动
    let base_values: HashMap<&str, f64> = [
        ("elastic_modulus", 200e9),
        ("yield_strength", 300e6),
        ("thermal_conductivity", 50.0),
        ("density", 7800.0),
        ("poissons_ratio", 0.3),
        ("shear_modulus", 80e9),
        ("bulk_modulus", 160e9),
    ]
    .iter()
    .cloned()
    .collect();

    let base = base_values.get(property).copied().unwrap_or(0.0);

    // 基于成分的简单加权 (mock)
    let fe_weight = composition.get("Fe").copied().unwrap_or(0.0);
    let al_weight = composition.get("Al").copied().unwrap_or(0.0);
    let ti_weight = composition.get("Ti").copied().unwrap_or(0.0);
    let cu_weight = composition.get("Cu").copied().unwrap_or(0.0);
    let cr_weight = composition.get("Cr").copied().unwrap_or(0.0);
    let ni_weight = composition.get("Ni").copied().unwrap_or(0.0);

    let composition_factor = match property {
        "elastic_modulus" => {
            1.0 + (fe_weight * 0.05 + ti_weight * -0.4 + al_weight * -0.65 + cu_weight * -0.4 + cr_weight * -0.1 + ni_weight * -0.1)
        }
        "yield_strength" => {
            1.0 + (fe_weight * -0.2 + ti_weight * 1.8 + al_weight * -0.1 + cu_weight * -0.7 + cr_weight * 0.3 + ni_weight * -0.2)
        }
        "thermal_conductivity" => {
            1.0 + (fe_weight * -0.7 + ti_weight * -0.85 + al_weight * 2.3 + cu_weight * 7.0 + cr_weight * -0.6 + ni_weight * -0.6)
        }
        "density" => {
            1.0 + (fe_weight * 0.0 + ti_weight * -0.44 + al_weight * -0.66 + cu_weight * 0.14 + cr_weight * -0.05 + ni_weight * 0.02)
        }
        "poissons_ratio" => {
            1.0 + (al_weight * 0.1 + ti_weight * 0.14 + cu_weight * 0.13)
        }
        _ => 1.0,
    };

    let predicted = base * composition_factor.max(0.1);
    // Mock 不确定性: 预测值的 3~8%
    let uncertainty = predicted * (0.03 + rand_factor() * 0.05);

    (predicted, uncertainty)
}

/// 简单伪随机因子 (基于成分的哈希)
fn rand_factor() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    (nanos % 1000) as f64 / 1000.0
}

// ============================================================================
// Tauri 命令
// ============================================================================

/// ML 属性预测
#[tauri::command]
pub async fn predict_material_properties(
    request: PredictionRequest,
) -> Result<PredictionResponse, String> {
    let start = std::time::Instant::now();

    let units = get_property_units();
    let mut predictions = Vec::new();

    for property in &request.target_properties {
        let (predicted, uncertainty) = mock_predict(&request.composition, property);
        let unit = units.get(property).cloned().unwrap_or_default();

        predictions.push(PropertyPrediction {
            property_name: property.clone(),
            predicted_value: predicted,
            unit,
            confidence_lower: predicted - 1.96 * uncertainty,
            confidence_upper: predicted + 1.96 * uncertainty,
            uncertainty,
        });
    }

    let inference_time_ms = start.elapsed().as_millis() as u64;

    Ok(PredictionResponse {
        model_name: request.model_name.clone(),
        is_mock: true,
        inference_time_ms,
        predictions,
    })
}

/// 获取可用 ML 模型列表
#[tauri::command]
pub async fn list_ml_models() -> Result<Vec<ModelInfo>, String> {
    let models = vec![
        ModelInfo {
            name: "mock_chgnet".to_string(),
            description: "CHGNet 通用材料属性预测模型 (Mock)".to_string(),
            supported_properties: vec![
                "elastic_modulus".to_string(),
                "yield_strength".to_string(),
                "thermal_conductivity".to_string(),
                "density".to_string(),
                "poissons_ratio".to_string(),
            ],
            is_loaded: true,
            model_type: "mock".to_string(),
        },
        ModelInfo {
            name: "mock_m3gnet".to_string(),
            description: "M3GNet 材料图神经网络 (Mock)".to_string(),
            supported_properties: vec![
                "elastic_modulus".to_string(),
                "shear_modulus".to_string(),
                "bulk_modulus".to_string(),
                "thermal_conductivity".to_string(),
            ],
            is_loaded: true,
            model_type: "mock".to_string(),
        },
    ];

    Ok(models)
}

/// 运行精度评测: ML 预测 vs 参考值
#[tauri::command]
pub async fn run_accuracy_benchmark(
    request: BenchmarkRequest,
) -> Result<Vec<BenchmarkResult>, String> {
    let ref_db = get_material_reference_db();
    let mut results = Vec::new();

    for material_name in &request.material_names {
        if let Some(ref_props) = ref_db.get(material_name) {
            // 构造 mock 成分
            let composition = build_mock_composition(material_name);

            for property in &request.target_properties {
                if let Some(&reference_value) = ref_props.get(property) {
                    let (predicted, _) = mock_predict(&composition, property);
                    let absolute_error = (predicted - reference_value).abs();
                    let relative_error_percent = if reference_value.abs() > 1e-10 {
                        (absolute_error / reference_value.abs()) * 100.0
                    } else {
                        0.0
                    };

                    results.push(BenchmarkResult {
                        material_name: material_name.clone(),
                        property_name: property.clone(),
                        predicted_value: predicted,
                        reference_value,
                        absolute_error,
                        relative_error_percent,
                        passed: relative_error_percent < 15.0,
                    });
                }
            }
        }
    }

    if results.is_empty() {
        return Err("未找到匹配的材料参考数据".to_string());
    }

    Ok(results)
}

/// 根据材料名称构造 mock 成分
fn build_mock_composition(material_name: &str) -> HashMap<String, f64> {
    let mut comp = HashMap::new();
    match material_name {
        "Q235" => {
            comp.insert("Fe".to_string(), 99.0);
            comp.insert("C".to_string(), 0.22);
        }
        "304不锈钢" => {
            comp.insert("Fe".to_string(), 70.0);
            comp.insert("Cr".to_string(), 19.0);
            comp.insert("Ni".to_string(), 9.0);
        }
        "Al6061" => {
            comp.insert("Al".to_string(), 97.0);
            comp.insert("Mg".to_string(), 1.0);
            comp.insert("Si".to_string(), 0.6);
        }
        "Ti6Al4V" => {
            comp.insert("Ti".to_string(), 89.5);
            comp.insert("Al".to_string(), 6.0);
            comp.insert("V".to_string(), 4.0);
        }
        "纯铜" => {
            comp.insert("Cu".to_string(), 99.9);
        }
        _ => {
            comp.insert("Fe".to_string(), 50.0);
            comp.insert("Cr".to_string(), 25.0);
            comp.insert("Ni".to_string(), 25.0);
        }
    }
    comp
}
