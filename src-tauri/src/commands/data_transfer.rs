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
