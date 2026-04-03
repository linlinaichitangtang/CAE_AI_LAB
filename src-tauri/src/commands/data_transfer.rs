//! Data Transfer Commands (V2.0 - 多尺度工作流编排器)
//!
//! 提供跨尺度数据传递能力：
//! - 获取尺度输出模式
//! - 验证数据传递
//! - 执行数据传递
//! - 管理传递规则
//!
//! Mock: DFT->MD transfer with 5 field mappings.

use serde::{Deserialize, Serialize};
use tauri::command;

// ============================================================================
// Data Structures
// ============================================================================

/// 字段映射定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMapping {
    pub id: String,
    pub source_field: String,
    pub target_field: String,
    pub transformation: String,
    pub unit_conversion: Option<String>,
    pub description: String,
}

/// 传递规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRule {
    pub id: String,
    pub source_scale: String,
    pub target_scale: String,
    pub name: String,
    pub field_mappings: Vec<FieldMapping>,
    pub validation_config: serde_json::Value,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// 数据传递执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    pub id: String,
    pub source_scale: String,
    pub target_scale: String,
    pub status: String,
    pub fields_transferred: Vec<String>,
    pub warnings: Vec<String>,
    pub execution_time_ms: u64,
    pub timestamp: String,
}

/// 数据传递历史记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferHistoryEntry {
    pub id: String,
    pub source_scale: String,
    pub target_scale: String,
    pub status: String,
    pub fields_count: usize,
    pub timestamp: String,
    pub execution_time_ms: u64,
}

/// 尺度输出模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleOutputSchema {
    pub scale: String,
    pub fields: Vec<SchemaField>,
    pub description: String,
}

/// 模式字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub field_type: String,
    pub unit: String,
    pub dimensions: Option<Vec<usize>>,
    pub description: String,
}

/// 数据传递验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub field_compatibility: Vec<FieldCompatibility>,
}

/// 字段兼容性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldCompatibility {
    pub source_field: String,
    pub target_field: String,
    pub compatible: bool,
    pub notes: String,
}

// ============================================================================
// Mock Data
// ============================================================================

fn mock_dft_to_md_rule() -> TransferRule {
    let now = chrono::Utc::now().to_rfc3339();
    TransferRule {
        id: "rule-dft-md-001".to_string(),
        source_scale: "DFT".to_string(),
        target_scale: "MD".to_string(),
        name: "DFT到MD势函数传递".to_string(),
        field_mappings: vec![
            FieldMapping {
                id: "fm-001".to_string(),
                source_field: "total_energy".to_string(),
                target_field: "reference_energy".to_string(),
                transformation: "linear_scale".to_string(),
                unit_conversion: Some("eV/atom -> eV".to_string()),
                description: "将DFT总能转换为MD参考能量".to_string(),
            },
            FieldMapping {
                id: "fm-002".to_string(),
                source_field: "lattice_constant".to_string(),
                target_field: "box_size".to_string(),
                transformation: "direct_copy".to_string(),
                unit_conversion: Some("Angstrom".to_string()),
                description: "晶格常数直接传递为MD模拟盒尺寸".to_string(),
            },
            FieldMapping {
                id: "fm-003".to_string(),
                source_field: "elastic_constants".to_string(),
                target_field: "stiffness_matrix".to_string(),
                transformation: "voigt_notation".to_string(),
                unit_conversion: None,
                description: "弹性常数从张量形式转换为Voigt记号".to_string(),
            },
            FieldMapping {
                id: "fm-004".to_string(),
                source_field: "formation_energy".to_string(),
                target_field: "cohesive_energy".to_string(),
                transformation: "affine_transform".to_string(),
                unit_conversion: Some("eV/atom -> eV".to_string()),
                description: "形成能转换为内聚能用于势函数拟合".to_string(),
            },
            FieldMapping {
                id: "fm-005".to_string(),
                source_field: "phonon_dispersion".to_string(),
                target_field: "force_constants".to_string(),
                transformation: "fourier_transform".to_string(),
                unit_conversion: None,
                description: "声子色散关系通过傅里叶变换得到力常数".to_string(),
            },
        ],
        validation_config: serde_json::json!({
            "max_energy_error": 0.01,
            "max_force_error": 0.05,
            "require_symmetry": true
        }),
        is_active: true,
        created_at: now.clone(),
        updated_at: now,
    }
}

// ============================================================================
// Commands
// ============================================================================

/// 获取尺度输出模式
#[command]
pub fn get_scale_output_schema(scale: String) -> Result<ScaleOutputSchema, String> {
    tracing::info!("Getting output schema for scale: {}", scale);
    let schema = match scale.as_str() {
        "DFT" => ScaleOutputSchema {
            scale: "DFT".to_string(),
            fields: vec![
                SchemaField {
                    name: "total_energy".to_string(),
                    field_type: "float64".to_string(),
                    unit: "eV".to_string(),
                    dimensions: None,
                    description: "体系总能量".to_string(),
                },
                SchemaField {
                    name: "lattice_constant".to_string(),
                    field_type: "float64".to_string(),
                    unit: "Angstrom".to_string(),
                    dimensions: Some(vec![3]),
                    description: "晶格常数向量".to_string(),
                },
                SchemaField {
                    name: "elastic_constants".to_string(),
                    field_type: "float64".to_string(),
                    unit: "GPa".to_string(),
                    dimensions: Some(vec![6, 6]),
                    description: "弹性常数矩阵 (Voigt记号)".to_string(),
                },
                SchemaField {
                    name: "formation_energy".to_string(),
                    field_type: "float64".to_string(),
                    unit: "eV/atom".to_string(),
                    dimensions: None,
                    description: "形成能".to_string(),
                },
                SchemaField {
                    name: "phonon_dispersion".to_string(),
                    field_type: "float64".to_string(),
                    unit: "THz".to_string(),
                    dimensions: Some(vec![3, 100]),
                    description: "声子色散关系".to_string(),
                },
            ],
            description: "第一性原理计算输出模式".to_string(),
        },
        "MD" => ScaleOutputSchema {
            scale: "MD".to_string(),
            fields: vec![
                SchemaField {
                    name: "trajectory".to_string(),
                    field_type: "float64".to_string(),
                    unit: "Angstrom".to_string(),
                    dimensions: Some(vec![3]),
                    description: "原子轨迹".to_string(),
                },
                SchemaField {
                    name: "radial_distribution".to_string(),
                    field_type: "float64".to_string(),
                    unit: "dimensionless".to_string(),
                    dimensions: Some(vec![500]),
                    description: "径向分布函数".to_string(),
                },
                SchemaField {
                    name: "mean_square_displacement".to_string(),
                    field_type: "float64".to_string(),
                    unit: "Angstrom^2".to_string(),
                    dimensions: Some(vec![1000]),
                    description: "均方位移".to_string(),
                },
            ],
            description: "分子动力学模拟输出模式".to_string(),
        },
        _ => ScaleOutputSchema {
            scale: scale.clone(),
            fields: vec![],
            description: format!("{} 尺度输出模式", scale),
        },
    };
    tracing::info!("Returned schema for scale {} with {} fields", scale, schema.fields.len());
    Ok(schema)
}

/// 验证数据传递
#[command]
pub fn validate_data_transfer(
    source_scale: String,
    target_scale: String,
    field_mappings: Vec<FieldMapping>,
) -> Result<TransferValidationResult, String> {
    tracing::info!(
        "Validating data transfer: {} -> {} ({} mappings)",
        source_scale, target_scale, field_mappings.len()
    );
    let compatibility = field_mappings
        .iter()
        .map(|fm| FieldCompatibility {
            source_field: fm.source_field.clone(),
            target_field: fm.target_field.clone(),
            compatible: true,
            notes: format!("{} -> {} 兼容", fm.source_field, fm.target_field),
        })
        .collect();

    let result = TransferValidationResult {
        is_valid: true,
        errors: vec![],
        warnings: vec![
            "phonon_dispersion -> force_constants 传递可能引入数值误差".to_string(),
        ],
        field_compatibility: compatibility,
    };
    tracing::info!(
        "Validation result: is_valid={}, warnings={}",
        result.is_valid,
        result.warnings.len()
    );
    Ok(result)
}

/// 执行数据传递
#[command]
pub fn execute_data_transfer(
    source_scale: String,
    target_scale: String,
    rule_id: String,
) -> Result<TransferResult, String> {
    tracing::info!(
        "Executing data transfer: {} -> {} (rule={})",
        source_scale, target_scale, rule_id
    );
    let result = TransferResult {
        id: uuid::Uuid::new_v4().to_string(),
        source_scale: source_scale.clone(),
        target_scale: target_scale.clone(),
        status: "completed".to_string(),
        fields_transferred: vec![
            "total_energy".to_string(),
            "lattice_constant".to_string(),
            "elastic_constants".to_string(),
            "formation_energy".to_string(),
            "phonon_dispersion".to_string(),
        ],
        warnings: vec![
            "phonon_dispersion 传递精度: 99.2%".to_string(),
        ],
        execution_time_ms: 342,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    tracing::info!(
        "Data transfer completed: {} fields in {}ms",
        result.fields_transferred.len(),
        result.execution_time_ms
    );
    Ok(result)
}

/// 获取传递规则列表
#[command]
pub fn get_transfer_rules() -> Result<Vec<TransferRule>, String> {
    tracing::info!("Getting transfer rules");
    let rules = vec![
        mock_dft_to_md_rule(),
        TransferRule {
            id: "rule-md-pf-001".to_string(),
            source_scale: "MD".to_string(),
            target_scale: "PF".to_string(),
            name: "MD到相场粗粒化".to_string(),
            field_mappings: vec![],
            validation_config: serde_json::json!({}),
            is_active: true,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        },
        TransferRule {
            id: "rule-pf-fe-001".to_string(),
            source_scale: "PF".to_string(),
            target_scale: "FE".to_string(),
            name: "相场到有限元均质化".to_string(),
            field_mappings: vec![],
            validation_config: serde_json::json!({}),
            is_active: true,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        },
    ];
    tracing::info!("Returned {} transfer rules", rules.len());
    Ok(rules)
}

/// 更新传递规则
#[command]
pub fn update_transfer_rule(rule: TransferRule) -> Result<TransferRule, String> {
    tracing::info!("Updating transfer rule: {}", rule.id);
    let mut updated = rule.clone();
    updated.updated_at = chrono::Utc::now().to_rfc3339();
    tracing::info!("Updated transfer rule: {}", updated.id);
    Ok(updated)
}

/// 获取传递历史
#[command]
pub fn get_transfer_history(limit: Option<usize>) -> Result<Vec<TransferHistoryEntry>, String> {
    tracing::info!("Getting transfer history (limit={:?})", limit);
    let limit = limit.unwrap_or(20);
    let entries = vec![
        TransferHistoryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            source_scale: "DFT".to_string(),
            target_scale: "MD".to_string(),
            status: "completed".to_string(),
            fields_count: 5,
            timestamp: "2026-04-02T14:30:00Z".to_string(),
            execution_time_ms: 342,
        },
        TransferHistoryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            source_scale: "MD".to_string(),
            target_scale: "PF".to_string(),
            status: "completed".to_string(),
            fields_count: 3,
            timestamp: "2026-04-02T15:00:00Z".to_string(),
            execution_time_ms: 1205,
        },
        TransferHistoryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            source_scale: "PF".to_string(),
            target_scale: "FE".to_string(),
            status: "completed".to_string(),
            fields_count: 4,
            timestamp: "2026-04-02T16:30:00Z".to_string(),
            execution_time_ms: 890,
        },
    ];
    tracing::info!("Returned {} history entries", entries.len().min(limit));
    Ok(entries.into_iter().take(limit).collect())
}

/// Converts physical quantities between unit systems (V2.3-022, KI-004).
/// Supports SI, CGS, atomic units (a.u.), and reduced LJ units.
#[command]
pub fn convert_units(
    value: f64,
    from_unit: String,
    to_unit: String,
    quantity: String,
) -> Result<UnitConversionResult, String> {
    tracing::info!(
        value = value,
        from = %from_unit,
        to = %to_unit,
        quantity = %quantity,
        "Converting units"
    );

    let converted = match quantity.to_lowercase().as_str() {
        "length" => convert_length(value, &from_unit, &to_unit)?,
        "energy" => convert_energy(value, &from_unit, &to_unit)?,
        "force" => convert_force(value, &from_unit, &to_unit)?,
        "pressure" | "stress" => convert_pressure(value, &from_unit, &to_unit)?,
        "temperature" => convert_temperature(value, &from_unit, &to_unit)?,
        "mass" => convert_mass(value, &from_unit, &to_unit)?,
        "time" => convert_time(value, &from_unit, &to_unit)?,
        "velocity" => convert_velocity(value, &from_unit, &to_unit)?,
        _ => return Err(format!("Unknown quantity: {}. Supported: length, energy, force, pressure, temperature, mass, time, velocity", quantity)),
    };

    Ok(UnitConversionResult {
        input_value: value,
        output_value: converted,
        from_unit,
        to_unit,
        quantity,
    })
}

fn convert_length(v: f64, from: &str, to: &str) -> Result<f64, String> {
    // All to Angstrom first
    let a = match from.to_lowercase().as_str() {
        "angstrom" | "a" => v,
        "bohr" | "a.u." => v * 0.529177,
        "nm" => v * 10.0,
        "m" => v * 1e10,
        "cm" => v * 1e8,
        "pm" => v * 0.01,
        _ => return Err(format!("Unknown length unit: {}", from)),
    };
    match to.to_lowercase().as_str() {
        "angstrom" | "a" => Ok(a),
        "bohr" | "a.u." => Ok(a / 0.529177),
        "nm" => Ok(a / 10.0),
        "m" => Ok(a / 1e10),
        "cm" => Ok(a / 1e8),
        "pm" => Ok(a / 0.01),
        _ => Err(format!("Unknown length unit: {}", to)),
    }
}

fn convert_energy(v: f64, from: &str, to: &str) -> Result<f64, String> {
    let e = match from.to_lowercase().as_str() {
        "ev" => v,
        "ry" => v * 13.605698,
        "hartree" | "ha" | "a.u." => v * 27.211386,
        "j" => v / 1.602176634e-19,
        "kcal/mol" => v / 23.060548,
        "kj/mol" => v / 96.485336,
        _ => return Err(format!("Unknown energy unit: {}", from)),
    };
    match to.to_lowercase().as_str() {
        "ev" => Ok(e),
        "ry" => Ok(e / 13.605698),
        "hartree" | "ha" | "a.u." => Ok(e / 27.211386),
        "j" => Ok(e * 1.602176634e-19),
        "kcal/mol" => Ok(e * 23.060548),
        "kj/mol" => Ok(e * 96.485336),
        _ => Err(format!("Unknown energy unit: {}", to)),
    }
}

fn convert_force(v: f64, from: &str, to: &str) -> Result<f64, String> {
    let f = match from.to_lowercase().as_str() {
        "ev/a" => v,
        "ry/bohr" => v * 25.711043,
        "n" => v / 8.9875518e9,
        "dyne" => v * 1.602176634e-8,
        _ => return Err(format!("Unknown force unit: {}", from)),
    };
    match to.to_lowercase().as_str() {
        "ev/a" => Ok(f),
        "ry/bohr" => Ok(f / 25.711043),
        "n" => Ok(f * 8.9875518e9),
        "dyne" => Ok(f / 1.602176634e-8),
        _ => Err(format!("Unknown force unit: {}", to)),
    }
}

fn convert_pressure(v: f64, from: &str, to: &str) -> Result<f64, String> {
    let p = match from.to_lowercase().as_str() {
        "gpa" => v,
        "pa" => v * 1e-9,
        "bar" => v * 0.0001,
        "atm" => v * 0.000101325,
        "ev/ang3" => v * 160.2176634,
        "ry/bohr3" => v * 14710.804,
        "kbar" => v * 0.1,
        "mbar" => v * 1e-4,
        _ => return Err(format!("Unknown pressure unit: {}", from)),
    };
    match to.to_lowercase().as_str() {
        "gpa" => Ok(p),
        "pa" => Ok(p / 1e-9),
        "bar" => Ok(p / 0.0001),
        "atm" => Ok(p / 0.000101325),
        "ev/ang3" => Ok(p / 160.2176634),
        "ry/bohr3" => Ok(p / 14710.804),
        "kbar" => Ok(p / 0.1),
        "mbar" => Ok(p / 1e-4),
        _ => Err(format!("Unknown pressure unit: {}", to)),
    }
}

fn convert_temperature(v: f64, from: &str, to: &str) -> Result<f64, String> {
    let k = match from.to_lowercase().as_str() {
        "k" => v,
        "c" => v + 273.15,
        "f" => (v - 32.0) * 5.0/9.0 + 273.15,
        "ry" => v * 157887.3, // 1 Ry = 157887.3 K
        "ev" => v * 11604.518, // 1 eV = 11604.518 K
        _ => return Err(format!("Unknown temperature unit: {}", from)),
    };
    match to.to_lowercase().as_str() {
        "k" => Ok(k),
        "c" => Ok(k - 273.15),
        "f" => Ok((k - 273.15) * 9.0/5.0 + 32.0),
        "ry" => Ok(k / 157887.3),
        "ev" => Ok(k / 11604.518),
        _ => Err(format!("Unknown temperature unit: {}", to)),
    }
}

fn convert_mass(v: f64, from: &str, to: &str) -> Result<f64, String> {
    let m = match from.to_lowercase().as_str() {
        "amu" | "u" => v,
        "kg" => v * 6.02214076e26,
        "g" => v * 6.02214076e23,
        "electron_mass" | "me" => v * 1822.888486,
        _ => return Err(format!("Unknown mass unit: {}", from)),
    };
    match to.to_lowercase().as_str() {
        "amu" | "u" => Ok(m),
        "kg" => Ok(m / 6.02214076e26),
        "g" => Ok(m / 6.02214076e23),
        "electron_mass" | "me" => Ok(m / 1822.888486),
        _ => Err(format!("Unknown mass unit: {}", to)),
    }
}

fn convert_time(v: f64, from: &str, to: &str) -> Result<f64, String> {
    let t = match from.to_lowercase().as_str() {
        "s" => v,
        "fs" => v * 1e-15,
        "ps" => v * 1e-12,
        "ns" => v * 1e-9,
        "lj" => v, // reduced unit, identity
        _ => return Err(format!("Unknown time unit: {}", from)),
    };
    match to.to_lowercase().as_str() {
        "s" => Ok(t),
        "fs" => Ok(t / 1e-15),
        "ps" => Ok(t / 1e-12),
        "ns" => Ok(t / 1e-9),
        "lj" => Ok(t),
        _ => Err(format!("Unknown time unit: {}", to)),
    }
}

fn convert_velocity(v: f64, from: &str, to: &str) -> Result<f64, String> {
    let vel = match from.to_lowercase().as_str() {
        "m/s" => v,
        "angstrom/fs" | "a/fs" => v * 100.0,
        "km/s" => v * 1000.0,
        _ => return Err(format!("Unknown velocity unit: {}", from)),
    };
    match to.to_lowercase().as_str() {
        "m/s" => Ok(vel),
        "angstrom/fs" | "a/fs" => Ok(vel / 100.0),
        "km/s" => Ok(vel / 1000.0),
        _ => Err(format!("Unknown velocity unit: {}", to)),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitConversionResult {
    pub input_value: f64,
    pub output_value: f64,
    pub from_unit: String,
    pub to_unit: String,
    pub quantity: String,
}
