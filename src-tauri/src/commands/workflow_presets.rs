//! Workflow Presets Commands (V2.0 - 多尺度工作流编排器)
//!
//! 提供工作流预设模板管理能力：
//! - 列出/获取/应用预设
//! - 从图创建预设
//! - 获取推荐预设
//!
//! 所有命令以 `wf_` 前缀命名，避免与 `workflow_template` 命令冲突。
//! Mock: 5 presets.

use serde::{Deserialize, Serialize};
use tauri::command;

// ============================================================================
// Data Structures
// ============================================================================

/// 预设工作流模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WfPreset {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub scales: Vec<String>,
    pub node_count: usize,
    pub edge_count: usize,
    pub graph: serde_json::Value,
    pub difficulty: String,
    pub estimated_time_hours: f64,
    pub is_builtin: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// 预设推荐
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WfPresetRecommendation {
    pub preset_id: String,
    pub preset_name: String,
    pub relevance_score: f64,
    pub reason: String,
    pub alternatives: Vec<WfPresetAlternative>,
}

/// 替代方案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WfPresetAlternative {
    pub preset_id: String,
    pub preset_name: String,
    pub score: f64,
    pub reason: String,
}

// ============================================================================
// Mock Data
// ============================================================================

fn mock_presets() -> Vec<WfPreset> {
    let now = chrono::Utc::now().to_rfc3339();
    vec![
        WfPreset {
            id: "preset-001".to_string(),
            name: "Mg-Al合金蠕变多尺度分析".to_string(),
            description: "从第一性原理到宏观蠕变响应的完整多尺度分析流程，涵盖DFT、MD、相场和有限元四个尺度。".to_string(),
            category: "creep".to_string(),
            tags: vec!["Mg-Al".to_string(), "creep".to_string(), "multiscale".to_string()],
            scales: vec!["DFT".to_string(), "MD".to_string(), "PF".to_string(), "FE".to_string()],
            node_count: 4,
            edge_count: 3,
            graph: serde_json::json!({
                "nodes": ["DFT", "MD", "PF", "FE"],
                "edges": [["DFT", "MD"], ["MD", "PF"], ["PF", "FE"]],
                "topology": "linear_chain"
            }),
            difficulty: "advanced".to_string(),
            estimated_time_hours: 48.0,
            is_builtin: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        WfPreset {
            id: "preset-002".to_string(),
            name: "Ni基高温合金疲劳寿命预测".to_string(),
            description: "结合DFT计算层错能与MD位错动力学，通过相场模拟裂纹萌生，最终有限元预测疲劳寿命。".to_string(),
            category: "fatigue".to_string(),
            tags: vec!["Ni-superalloy".to_string(), "fatigue".to_string(), "crack".to_string()],
            scales: vec!["DFT".to_string(), "MD".to_string(), "PF".to_string(), "FE".to_string()],
            node_count: 4,
            edge_count: 3,
            graph: serde_json::json!({
                "nodes": ["DFT", "MD", "PF", "FE"],
                "edges": [["DFT", "MD"], ["MD", "PF"], ["PF", "FE"]],
                "topology": "linear_chain"
            }),
            difficulty: "advanced".to_string(),
            estimated_time_hours: 72.0,
            is_builtin: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        WfPreset {
            id: "preset-003".to_string(),
            name: "复合材料界面力学研究".to_string(),
            description: "从原子尺度计算界面结合能，通过MD模拟界面脱粘过程，相场描述界面裂纹扩展，有限元分析宏观力学行为。".to_string(),
            category: "interface".to_string(),
            tags: vec!["composite".to_string(), "interface".to_string(), "debonding".to_string()],
            scales: vec!["DFT".to_string(), "MD".to_string(), "PF".to_string(), "FE".to_string()],
            node_count: 4,
            edge_count: 3,
            graph: serde_json::json!({
                "nodes": ["DFT", "MD", "PF", "FE"],
                "edges": [["DFT", "MD"], ["MD", "PF"], ["PF", "FE"]],
                "topology": "linear_chain"
            }),
            difficulty: "intermediate".to_string(),
            estimated_time_hours: 36.0,
            is_builtin: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        WfPreset {
            id: "preset-004".to_string(),
            name: "快速DFT-MD串联计算".to_string(),
            description: "简化的两尺度串联工作流，从DFT获取势函数参数后直接运行MD模拟，适用于快速筛选。".to_string(),
            category: "screening".to_string(),
            tags: vec!["DFT".to_string(), "MD".to_string(), "screening".to_string()],
            scales: vec!["DFT".to_string(), "MD".to_string()],
            node_count: 2,
            edge_count: 1,
            graph: serde_json::json!({
                "nodes": ["DFT", "MD"],
                "edges": [["DFT", "MD"]],
                "topology": "linear_chain"
            }),
            difficulty: "beginner".to_string(),
            estimated_time_hours: 8.0,
            is_builtin: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        WfPreset {
            id: "preset-005".to_string(),
            name: "相场-有限元均质化分析".to_string(),
            description: "从相场模拟获取微观组织演化信息，通过均质化方法计算等效宏观力学性能，用于有限元分析。".to_string(),
            category: "homogenization".to_string(),
            tags: vec!["PF".to_string(), "FE".to_string(), "homogenization".to_string()],
            scales: vec!["PF".to_string(), "FE".to_string()],
            node_count: 2,
            edge_count: 1,
            graph: serde_json::json!({
                "nodes": ["PF", "FE"],
                "edges": [["PF", "FE"]],
                "topology": "linear_chain"
            }),
            difficulty: "intermediate".to_string(),
            estimated_time_hours: 16.0,
            is_builtin: true,
            created_at: now.clone(),
            updated_at: now,
        },
    ]
}

// ============================================================================
// Commands
// ============================================================================

/// 列出所有工作流预设
#[command]
pub fn list_wf_presets(
    category: Option<String>,
    difficulty: Option<String>,
) -> Result<Vec<WfPreset>, String> {
    tracing::info!(
        "Listing workflow presets (category={:?}, difficulty={:?})",
        category, difficulty
    );
    let mut presets = mock_presets();
    if let Some(cat) = category {
        presets.retain(|p| p.category == cat);
    }
    if let Some(diff) = difficulty {
        presets.retain(|p| p.difficulty == diff);
    }
    tracing::info!("Listed {} presets", presets.len());
    Ok(presets)
}

/// 获取预设详情
#[command]
pub fn get_wf_preset_detail(preset_id: String) -> Result<WfPreset, String> {
    tracing::info!("Getting preset detail: {}", preset_id);
    let presets = mock_presets();
    let preset = presets
        .into_iter()
        .find(|p| p.id == preset_id)
        .unwrap_or_else(|| mock_presets().remove(0));
    tracing::info!("Returned preset: {}", preset.name);
    Ok(preset)
}

/// 应用预设
#[command]
pub fn apply_wf_preset(
    preset_id: String,
    project_id: String,
) -> Result<serde_json::Value, String> {
    tracing::info!("Applying preset {} to project {}", preset_id, project_id);
    let presets = mock_presets();
    let preset = presets
        .iter()
        .find(|p| p.id == preset_id)
        .unwrap_or(&presets[0]);
    Ok(serde_json::json!({
        "preset_id": preset_id,
        "preset_name": preset.name,
        "project_id": project_id,
        "graph_id": uuid::Uuid::new_v4().to_string(),
        "nodes_created": preset.node_count,
        "edges_created": preset.edge_count,
        "status": "applied",
        "message": format!("预设 '{}' 已成功应用到项目", preset.name)
    }))
}

/// 从图创建预设
#[command]
pub fn create_wf_preset_from_graph(
    graph_id: String,
    name: String,
    description: String,
    category: String,
    tags: Option<Vec<String>>,
) -> Result<WfPreset, String> {
    tracing::info!(
        "Creating preset from graph {} with name '{}'",
        graph_id, name
    );
    let now = chrono::Utc::now().to_rfc3339();
    let preset = WfPreset {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        description,
        category,
        tags: tags.unwrap_or_default(),
        scales: vec!["DFT".to_string(), "MD".to_string(), "PF".to_string(), "FE".to_string()],
        node_count: 4,
        edge_count: 3,
        graph: serde_json::json!({"source_graph_id": graph_id}),
        difficulty: "custom".to_string(),
        estimated_time_hours: 0.0,
        is_builtin: false,
        created_at: now.clone(),
        updated_at: now,
    };
    tracing::info!("Created preset: {} (id={})", preset.name, preset.id);
    Ok(preset)
}

/// 获取预设推荐
#[command]
pub fn get_wf_preset_recommendation(
    material_system: String,
    analysis_type: String,
) -> Result<WfPresetRecommendation, String> {
    tracing::info!(
        "Getting preset recommendation for material='{}', analysis='{}'",
        material_system, analysis_type
    );
    let recommendation = WfPresetRecommendation {
        preset_id: "preset-001".to_string(),
        preset_name: "Mg-Al合金蠕变多尺度分析".to_string(),
        relevance_score: 0.95,
        reason: "材料体系 Mg-Al 与分析类型 creep 高度匹配，推荐使用完整四尺度串行工作流。".to_string(),
        alternatives: vec![
            WfPresetAlternative {
                preset_id: "preset-004".to_string(),
                preset_name: "快速DFT-MD串联计算".to_string(),
                score: 0.72,
                reason: "如果仅需快速筛选，可使用简化两尺度流程".to_string(),
            },
            WfPresetAlternative {
                preset_id: "preset-005".to_string(),
                preset_name: "相场-有限元均质化分析".to_string(),
                score: 0.65,
                reason: "如果已有微观组织数据，可直接从相场开始".to_string(),
            },
        ],
    };
    tracing::info!(
        "Recommended preset: {} (score={})",
        recommendation.preset_name, recommendation.relevance_score
    );
    Ok(recommendation)
}
