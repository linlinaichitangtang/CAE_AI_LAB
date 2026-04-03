//! Parameter Mapping Commands (V2.0 - 多尺度工作流编排器)
//!
//! 提供跨尺度参数映射管理能力：
//! - 获取/更新/添加/删除映射规则
//! - 自动映射参数
//! - 应用映射
//! - 获取映射灵敏度
//!
//! Mock: 12 mapping rules.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::command;

// ============================================================================
// Data Structures
// ============================================================================

/// 映射规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingRule {
    pub id: String,
    pub source_scale: String,
    pub target_scale: String,
    pub source_param: String,
    pub target_param: String,
    pub transformation: String,
    pub formula: Option<String>,
    pub sensitivity: Option<f64>,
    pub description: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// 映射表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingTable {
    pub id: String,
    pub source_scale: String,
    pub target_scale: String,
    pub rules: Vec<MappingRule>,
    pub total_rules: usize,
    pub active_rules: usize,
    pub last_updated: String,
}

/// 自动映射结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoMapResult {
    pub suggested_mappings: Vec<MappingRule>,
    pub confidence_scores: Vec<f64>,
    pub unmapped_source_params: Vec<String>,
    pub unmapped_target_params: Vec<String>,
}

/// 映射应用结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyMappingResult {
    pub id: String,
    pub applied_rules: Vec<String>,
    pub transformed_values: HashMap<String, serde_json::Value>,
    pub warnings: Vec<String>,
    pub timestamp: String,
}

/// 映射灵敏度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingSensitivity {
    pub rule_id: String,
    pub source_param: String,
    pub target_param: String,
    /// 灵敏度系数 (目标参数变化 / 源参数变化)
    pub sensitivity: f64,
    /// 灵敏度等级: "low", "medium", "high", "critical"
    pub level: String,
    /// 推荐精度
    pub recommended_precision: f64,
    pub description: String,
}

// ============================================================================
// Mock Data
// ============================================================================

fn mock_mapping_rules() -> Vec<MappingRule> {
    let now = chrono::Utc::now().to_rfc3339();
    vec![
        MappingRule {
            id: "mr-001".to_string(),
            source_scale: "DFT".to_string(),
            target_scale: "MD".to_string(),
            source_param: "lattice_constant".to_string(),
            target_param: "box_size".to_string(),
            transformation: "direct_copy".to_string(),
            formula: Some("box_size = lattice_constant".to_string()),
            sensitivity: Some(1.0),
            description: "晶格常数直接传递为模拟盒尺寸".to_string(),
            is_active: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        MappingRule {
            id: "mr-002".to_string(),
            source_scale: "DFT".to_string(),
            target_scale: "MD".to_string(),
            source_param: "bulk_modulus".to_string(),
            target_param: "pair_potential_epsilon".to_string(),
            transformation: "power_law".to_string(),
            formula: Some("epsilon = B * V0 / (9 * N)".to_string()),
            sensitivity: Some(0.85),
            description: "体积模量转换为Lennard-Jones势阱深度".to_string(),
            is_active: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        MappingRule {
            id: "mr-003".to_string(),
            source_scale: "DFT".to_string(),
            target_scale: "MD".to_string(),
            source_param: "cohesive_energy".to_string(),
            target_param: "reference_energy".to_string(),
            transformation: "linear_scale".to_string(),
            formula: Some("E_ref = E_coh / N_atoms".to_string()),
            sensitivity: Some(0.92),
            description: "内聚能转换为单原子参考能量".to_string(),
            is_active: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        MappingRule {
            id: "mr-004".to_string(),
            source_scale: "MD".to_string(),
            target_scale: "PF".to_string(),
            source_param: "diffusion_coefficient".to_string(),
            target_param: "mobility".to_string(),
            transformation: "arrhenius".to_string(),
            formula: Some("M = D / (kB * T)".to_string()),
            sensitivity: Some(0.78),
            description: "扩散系数通过Arrhenius关系转换为相场迁移率".to_string(),
            is_active: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        MappingRule {
            id: "mr-005".to_string(),
            source_scale: "MD".to_string(),
            target_scale: "PF".to_string(),
            source_param: "interface_energy".to_string(),
            target_param: "gradient_coefficient".to_string(),
            transformation: "linear_scale".to_string(),
            formula: Some("kappa = gamma * delta".to_string()),
            sensitivity: Some(0.65),
            description: "界面能转换为相场梯度系数".to_string(),
            is_active: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        MappingRule {
            id: "mr-006".to_string(),
            source_scale: "MD".to_string(),
            target_scale: "PF".to_string(),
            source_param: "nucleation_barrier".to_string(),
            target_param: "free_energy_barrier".to_string(),
            transformation: "direct_copy".to_string(),
            formula: None,
            sensitivity: Some(0.88),
            description: "形核势垒直接传递为自由能势垒".to_string(),
            is_active: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        MappingRule {
            id: "mr-007".to_string(),
            source_scale: "PF".to_string(),
            target_scale: "FE".to_string(),
            source_param: "effective_modulus".to_string(),
            target_param: "youngs_modulus".to_string(),
            transformation: "homogenization".to_string(),
            formula: Some("E_eff = Mori_Tanaka(E_matrix, E_incl, f)".to_string()),
            sensitivity: Some(0.95),
            description: "相场有效模量通过Mori-Tanaka均质化转换为宏观弹性模量".to_string(),
            is_active: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        MappingRule {
            id: "mr-008".to_string(),
            source_scale: "PF".to_string(),
            target_scale: "FE".to_string(),
            source_param: "phase_fraction".to_string(),
            target_param: "material_partition".to_string(),
            transformation: "volume_average".to_string(),
            formula: Some("f_i = integral(phi_i) / V".to_string()),
            sensitivity: Some(0.70),
            description: "相分数转换为有限元材料分区".to_string(),
            is_active: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        MappingRule {
            id: "mr-009".to_string(),
            source_scale: "PF".to_string(),
            target_scale: "FE".to_string(),
            source_param: "residual_stress".to_string(),
            target_param: "initial_stress".to_string(),
            transformation: "direct_copy".to_string(),
            formula: None,
            sensitivity: Some(0.82),
            description: "相场残余应力直接传递为有限元初始应力".to_string(),
            is_active: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        MappingRule {
            id: "mr-010".to_string(),
            source_scale: "DFT".to_string(),
            target_scale: "FE".to_string(),
            source_param: "elastic_tensor".to_string(),
            target_param: "constitutive_matrix".to_string(),
            transformation: "voigt_transform".to_string(),
            formula: Some("C_voigt = Voigt(C_full)".to_string()),
            sensitivity: Some(0.99),
            description: "DFT弹性张量转换为有限元本构矩阵".to_string(),
            is_active: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        MappingRule {
            id: "mr-011".to_string(),
            source_scale: "MD".to_string(),
            target_scale: "FE".to_string(),
            source_param: "thermal_conductivity".to_string(),
            target_param: "conductivity_tensor".to_string(),
            transformation: "tensor_interpolation".to_string(),
            formula: Some("k_ij = Green_Kubo(integral)".to_string()),
            sensitivity: Some(0.75),
            description: "MD热导率通过Green-Kubo转换为有限元热导率张量".to_string(),
            is_active: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        MappingRule {
            id: "mr-012".to_string(),
            source_scale: "DFT".to_string(),
            target_scale: "MD".to_string(),
            source_param: "phonon_frequencies".to_string(),
            target_param: "vibrational_dos".to_string(),
            transformation: "spectral_interpolation".to_string(),
            formula: Some("g(w) = sum(delta(w - w_i))".to_string()),
            sensitivity: Some(0.60),
            description: "声子频率转换为振动态密度".to_string(),
            is_active: false,
            created_at: now.clone(),
            updated_at: now,
        },
    ]
}

// ============================================================================
// Commands
// ============================================================================

/// 获取映射表
#[command]
pub fn get_mapping_table(
    source_scale: String,
    target_scale: String,
) -> Result<MappingTable, String> {
    tracing::info!(
        "Getting mapping table: {} -> {}",
        source_scale, target_scale
    );
    let all_rules = mock_mapping_rules();
    let rules: Vec<MappingRule> = all_rules
        .into_iter()
        .filter(|r| r.source_scale == source_scale && r.target_scale == target_scale)
        .collect();
    let active_count = rules.iter().filter(|r| r.is_active).count();
    let table = MappingTable {
        id: uuid::Uuid::new_v4().to_string(),
        source_scale: source_scale.clone(),
        target_scale: target_scale.clone(),
        rules,
        total_rules: active_count,
        active_rules: active_count,
        last_updated: chrono::Utc::now().to_rfc3339(),
    };
    tracing::info!(
        "Mapping table: {} rules ({} active)",
        table.total_rules, table.active_rules
    );
    Ok(table)
}

/// 更新映射规则
#[command]
pub fn update_mapping_rule(rule: MappingRule) -> Result<MappingRule, String> {
    tracing::info!("Updating mapping rule: {}", rule.id);
    let mut updated = rule.clone();
    updated.updated_at = chrono::Utc::now().to_rfc3339();
    tracing::info!("Updated mapping rule: {}", updated.id);
    Ok(updated)
}

/// 添加映射规则
#[command]
pub fn add_mapping_rule(rule: MappingRule) -> Result<MappingRule, String> {
    tracing::info!(
        "Adding mapping rule: {} -> {}",
        rule.source_param, rule.target_param
    );
    let mut new_rule = rule.clone();
    new_rule.id = uuid::Uuid::new_v4().to_string();
    new_rule.created_at = chrono::Utc::now().to_rfc3339();
    new_rule.updated_at = chrono::Utc::now().to_rfc3339();
    tracing::info!("Added mapping rule: {}", new_rule.id);
    Ok(new_rule)
}

/// 删除映射规则
#[command]
pub fn delete_mapping_rule(rule_id: String) -> Result<serde_json::Value, String> {
    tracing::info!("Deleting mapping rule: {}", rule_id);
    Ok(serde_json::json!({
        "deleted_rule_id": rule_id,
        "status": "success",
        "message": "映射规则已删除"
    }))
}

/// 自动映射参数
#[command]
pub fn auto_map_parameters(
    source_scale: String,
    target_scale: String,
) -> Result<AutoMapResult, String> {
    tracing::info!(
        "Auto-mapping parameters: {} -> {}",
        source_scale, target_scale
    );
    let all_rules = mock_mapping_rules();
    let matching: Vec<MappingRule> = all_rules
        .into_iter()
        .filter(|r| r.source_scale == source_scale && r.target_scale == target_scale)
        .collect();
    let result = AutoMapResult {
        suggested_mappings: matching,
        confidence_scores: vec![0.95, 0.88, 0.82, 0.76, 0.70],
        unmapped_source_params: vec!["surface_energy".to_string()],
        unmapped_target_params: vec!["crack_tip_field".to_string()],
    };
    tracing::info!(
        "Auto-map result: {} suggestions, {} unmapped source, {} unmapped target",
        result.suggested_mappings.len(),
        result.unmapped_source_params.len(),
        result.unmapped_target_params.len()
    );
    Ok(result)
}

/// 应用映射
#[command]
pub fn apply_mapping(
    source_scale: String,
    target_scale: String,
    source_values: HashMap<String, serde_json::Value>,
) -> Result<ApplyMappingResult, String> {
    tracing::info!(
        "Applying mapping: {} -> {} ({} values)",
        source_scale,
        target_scale,
        source_values.len()
    );
    let mut transformed: HashMap<String, serde_json::Value> = HashMap::new();
    for (key, value) in &source_values {
        transformed.insert(format!("mapped_{}", key), value.clone());
    }
    let result = ApplyMappingResult {
        id: uuid::Uuid::new_v4().to_string(),
        applied_rules: vec!["mr-001".to_string(), "mr-002".to_string(), "mr-003".to_string()],
        transformed_values: transformed,
        warnings: vec![
            "部分参数映射使用了近似公式，精度可能受限".to_string(),
        ],
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    tracing::info!("Applied {} mapping rules", result.applied_rules.len());
    Ok(result)
}

/// 获取映射灵敏度
#[command]
pub fn get_mapping_sensitivity(
    source_scale: String,
    target_scale: String,
) -> Result<Vec<MappingSensitivity>, String> {
    tracing::info!(
        "Getting mapping sensitivity: {} -> {}",
        source_scale, target_scale
    );
    let sensitivities = vec![
        MappingSensitivity {
            rule_id: "mr-001".to_string(),
            source_param: "lattice_constant".to_string(),
            target_param: "box_size".to_string(),
            sensitivity: 1.0,
            level: "critical".to_string(),
            recommended_precision: 0.001,
            description: "晶格常数对模拟盒尺寸有直接1:1影响".to_string(),
        },
        MappingSensitivity {
            rule_id: "mr-002".to_string(),
            source_param: "bulk_modulus".to_string(),
            target_param: "pair_potential_epsilon".to_string(),
            sensitivity: 0.85,
            level: "high".to_string(),
            recommended_precision: 0.01,
            description: "体积模量对势函数拟合精度影响显著".to_string(),
        },
        MappingSensitivity {
            rule_id: "mr-007".to_string(),
            source_param: "effective_modulus".to_string(),
            target_param: "youngs_modulus".to_string(),
            sensitivity: 0.95,
            level: "critical".to_string(),
            recommended_precision: 0.005,
            description: "有效模量对宏观弹性模量影响极大".to_string(),
        },
    ];
    tracing::info!("Returned {} sensitivity entries", sensitivities.len());
    Ok(sensitivities)
}

/// Maps coordinates between different scale representations (V2.3-022, KI-004).
/// Supports atom box -> phase field grid, fractional -> Cartesian, etc.
#[command]
pub fn map_coordinates(
    positions: Vec<Vec<f64>>,
    from_system: String,
    to_system: String,
    box_vectors: Option<Vec<Vec<f64>>>,
    grid_size: Option<Vec<u32>>,
    domain_size: Option<Vec<f64>>,
) -> Result<CoordinateMappingResult, String> {
    tracing::info!(
        n_points = positions.len(),
        from = %from_system,
        to = %to_system,
        "Mapping coordinates"
    );

    if positions.is_empty() {
        return Err("No positions provided".to_string());
    }

    let mapped = match (from_system.as_str(), to_system.as_str()) {
        ("cartesian", "fractional") => {
            let bv = box_vectors.as_ref().ok_or("box_vectors required for cartesian->fractional")?;
            if bv.len() < 3 { return Err("box_vectors must have 3 vectors".to_string()); }
            let a = &bv[0]; let b = &bv[1]; let c = &bv[2];
            // Inverse of matrix [a,b,c]
            let det = a[0]*(b[1]*c[2]-b[2]*c[1]) - a[1]*(b[0]*c[2]-b[2]*c[0]) + a[2]*(b[0]*c[1]-b[1]*c[0]);
            if det.abs() < 1e-15 { return Err("Singular box matrix".to_string()); }
            let inv_det = 1.0 / det;
            positions.iter().map(|p| {
                let x = p.get(0).copied().unwrap_or(0.0);
                let y = p.get(1).copied().unwrap_or(0.0);
                let z = p.get(2).copied().unwrap_or(0.0);
                let fx = inv_det * ((b[1]*c[2]-b[2]*c[1])*x + (a[2]*c[1]-a[1]*c[2])*y + (a[1]*b[2]-a[2]*b[1])*z);
                let fy = inv_det * ((b[2]*c[0]-b[0]*c[2])*x + (a[0]*c[2]-a[2]*c[0])*y + (a[2]*b[0]-a[0]*b[2])*z);
                let fz = inv_det * ((b[0]*c[1]-b[1]*c[0])*x + (a[1]*c[0]-a[0]*c[1])*y + (a[0]*b[1]-a[1]*b[0])*z);
                vec![fx, fy, fz]
            }).collect()
        }
        ("fractional", "cartesian") => {
            let bv = box_vectors.as_ref().ok_or("box_vectors required for fractional->cartesian")?;
            if bv.len() < 3 { return Err("box_vectors must have 3 vectors".to_string()); }
            let a = &bv[0]; let b = &bv[1]; let c = &bv[2];
            positions.iter().map(|p| {
                let fx = p.get(0).copied().unwrap_or(0.0);
                let fy = p.get(1).copied().unwrap_or(0.0);
                let fz = p.get(2).copied().unwrap_or(0.0);
                vec![
                    a[0]*fx + b[0]*fy + c[0]*fz,
                    a[1]*fx + b[1]*fy + c[1]*fz,
                    a[2]*fx + b[2]*fy + c[2]*fz,
                ]
            }).collect()
        }
        ("cartesian", "grid_index") => {
            let gs = grid_size.as_ref().ok_or("grid_size required for cartesian->grid_index")?;
            let ds = domain_size.as_ref().ok_or("domain_size required for cartesian->grid_index")?;
            if gs.len() < 3 || ds.len() < 3 { return Err("grid_size and domain_size must have 3 elements".to_string()); }
            let dx = ds[0] / gs[0] as f64;
            let dy = ds[1] / gs[1] as f64;
            let dz = ds[2] / gs[2] as f64;
            positions.iter().map(|p| {
                let x = p.get(0).copied().unwrap_or(0.0);
                let y = p.get(1).copied().unwrap_or(0.0);
                let z = p.get(2).copied().unwrap_or(0.0);
                vec![
                    (x / dx).floor().max(0.0).min((gs[0]-1) as f64),
                    (y / dy).floor().max(0.0).min((gs[1]-1) as f64),
                    (z / dz).floor().max(0.0).min((gs[2]-1) as f64),
                ]
            }).collect()
        }
        _ => return Err(format!(
            "Unsupported conversion: {} -> {}. Supported: cartesian<->fractional, cartesian->grid_index",
            from_system, to_system
        )),
    };

    Ok(CoordinateMappingResult {
        mapped_positions: mapped,
        num_points: positions.len() as u32,
        from_system,
        to_system,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinateMappingResult {
    pub mapped_positions: Vec<Vec<f64>>,
    pub num_points: u32,
    pub from_system: String,
    pub to_system: String,
}
