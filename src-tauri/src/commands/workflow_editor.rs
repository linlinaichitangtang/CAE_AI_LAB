//! Workflow Editor Commands (V2.0 - 多尺度工作流编排器)
//!
//! 提供工作流图的创建、加载、保存、节点/边管理等能力。
//! 所有命令以 `wf_` 前缀命名，避免与已有命令冲突。
//! Mock: 4 nodes + 3 edges forming DFT -> MD -> PF -> FE chain.

use serde::{Deserialize, Serialize};
use tauri::command;

// ============================================================================
// Data Structures
// ============================================================================

/// 工作流节点类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WfNodeType {
    Dft,
    MolecularDynamics,
    PhaseField,
    FiniteElement,
    Custom { label: String },
}

/// 工作流节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WfNode {
    pub id: String,
    pub node_type: WfNodeType,
    pub label: String,
    pub position: WfPosition,
    pub config: serde_json::Value,
    pub status: String,
}

/// 节点在画布上的位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WfPosition {
    pub x: f64,
    pub y: f64,
}

/// 工作流边（连接）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WfEdge {
    pub id: String,
    pub source_node_id: String,
    pub target_node_id: String,
    pub label: String,
    pub data_mapping: serde_json::Value,
}

/// 条件分支
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WfConditionalBranch {
    pub id: String,
    pub node_id: String,
    pub condition: String,
    pub target_node_id: String,
    pub priority: i32,
}

/// 工作流图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WfGraph {
    pub id: String,
    pub name: String,
    pub description: String,
    pub nodes: Vec<WfNode>,
    pub edges: Vec<WfEdge>,
    pub conditional_branches: Vec<WfConditionalBranch>,
    pub created_at: String,
    pub updated_at: String,
}

/// 图验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WfValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub info: Vec<String>,
}

// ============================================================================
// Mock Data
// ============================================================================

fn mock_wf_graph(name: &str) -> WfGraph {
    let now = chrono::Utc::now().to_rfc3339();
    let dft_node = WfNode {
        id: "node-dft-001".to_string(),
        node_type: WfNodeType::Dft,
        label: "DFT 计算".to_string(),
        position: WfPosition { x: 80.0, y: 200.0 },
        config: serde_json::json!({
            "method": "VASP",
            "kpoints": [4, 4, 4],
            "encut": 520.0,
            "ediff": 1e-6
        }),
        status: "ready".to_string(),
    };
    let md_node = WfNode {
        id: "node-md-001".to_string(),
        node_type: WfNodeType::MolecularDynamics,
        label: "MD 模拟".to_string(),
        position: WfPosition { x: 320.0, y: 200.0 },
        config: serde_json::json!({
            "ensemble": "NVT",
            "temperature": 300.0,
            "timestep": 1.0,
            "steps": 100000
        }),
        status: "ready".to_string(),
    };
    let pf_node = WfNode {
        id: "node-pf-001".to_string(),
        node_type: WfNodeType::PhaseField,
        label: "相场模拟".to_string(),
        position: WfPosition { x: 560.0, y: 200.0 },
        config: serde_json::json!({
            "model": "Allen-Cahn",
            "grid_size": [256, 256],
            "dx": 0.5,
            "dt": 0.01
        }),
        status: "ready".to_string(),
    };
    let fe_node = WfNode {
        id: "node-fe-001".to_string(),
        node_type: WfNodeType::FiniteElement,
        label: "有限元分析".to_string(),
        position: WfPosition { x: 800.0, y: 200.0 },
        config: serde_json::json!({
            "solver": "Abaqus",
            "element_type": "C3D8R",
            "mesh_size": 0.1,
            "max_increment": 1000
        }),
        status: "ready".to_string(),
    };

    let edges = vec![
        WfEdge {
            id: "edge-001".to_string(),
            source_node_id: "node-dft-001".to_string(),
            target_node_id: "node-md-001".to_string(),
            label: "DFT->MD 势函数传递".to_string(),
            data_mapping: serde_json::json!({"type": "interatomic_potential"}),
        },
        WfEdge {
            id: "edge-002".to_string(),
            source_node_id: "node-md-001".to_string(),
            target_node_id: "node-pf-001".to_string(),
            label: "MD->PF 粗粒化".to_string(),
            data_mapping: serde_json::json!({"type": "coarse_graining", "method": "gaussian"}),
        },
        WfEdge {
            id: "edge-003".to_string(),
            source_node_id: "node-pf-001".to_string(),
            target_node_id: "node-fe-001".to_string(),
            label: "PF->FE 均质化".to_string(),
            data_mapping: serde_json::json!({"type": "homogenization", "method": "mori_tanaka"}),
        },
    ];

    WfGraph {
        id: uuid::Uuid::new_v4().to_string(),
        name: name.to_string(),
        description: "DFT->MD->PF->FE 多尺度串行工作流".to_string(),
        nodes: vec![dft_node, md_node, pf_node, fe_node],
        edges,
        conditional_branches: vec![],
        created_at: now.clone(),
        updated_at: now,
    }
}

// ============================================================================
// Commands
// ============================================================================

/// 创建工作流图
#[command]
pub fn create_wf_graph(name: String, description: Option<String>) -> Result<WfGraph, String> {
    tracing::info!("Creating workflow graph: {}", name);
    let mut graph = mock_wf_graph(&name);
    if let Some(desc) = description {
        graph.description = desc;
    }
    tracing::info!("Created workflow graph: {} (id={})", graph.name, graph.id);
    Ok(graph)
}

/// 加载工作流图
#[command]
pub fn load_wf_graph(graph_id: String) -> Result<WfGraph, String> {
    tracing::info!("Loading workflow graph: {}", graph_id);
    let graph = mock_wf_graph("已加载的工作流");
    tracing::info!("Loaded workflow graph: {} (id={})", graph.name, graph.id);
    Ok(graph)
}

/// 保存工作流图
#[command]
pub fn save_wf_graph(graph: WfGraph) -> Result<WfGraph, String> {
    tracing::info!("Saving workflow graph: {} (id={})", graph.name, graph.id);
    let mut saved = graph.clone();
    saved.updated_at = chrono::Utc::now().to_rfc3339();
    tracing::info!("Saved workflow graph: {}", saved.id);
    Ok(saved)
}

/// 添加工作流节点
#[command]
pub fn add_wf_node(
    graph_id: String,
    node_type: WfNodeType,
    label: String,
    position: WfPosition,
    config: Option<serde_json::Value>,
) -> Result<WfNode, String> {
    tracing::info!("Adding node '{}' to graph {}", label, graph_id);
    let node = WfNode {
        id: uuid::Uuid::new_v4().to_string(),
        node_type,
        label,
        position,
        config: config.unwrap_or(serde_json::json!({})),
        status: "ready".to_string(),
    };
    tracing::info!("Added node: {} (id={})", node.label, node.id);
    Ok(node)
}

/// 移除工作流节点
#[command]
pub fn remove_wf_node(graph_id: String, node_id: String) -> Result<serde_json::Value, String> {
    tracing::info!("Removing node {} from graph {}", node_id, graph_id);
    Ok(serde_json::json!({
        "removed_node_id": node_id,
        "graph_id": graph_id,
        "status": "success"
    }))
}

/// 更新工作流节点配置
#[command]
pub fn update_wf_node_config(
    graph_id: String,
    node_id: String,
    config: serde_json::Value,
) -> Result<WfNode, String> {
    tracing::info!("Updating config for node {} in graph {}", node_id, graph_id);
    let node = WfNode {
        id: node_id,
        node_type: WfNodeType::Custom { label: "Updated".to_string() },
        label: "更新后的节点".to_string(),
        position: WfPosition { x: 0.0, y: 0.0 },
        config,
        status: "ready".to_string(),
    };
    tracing::info!("Updated node config: {}", node.id);
    Ok(node)
}

/// 添加工作流边
#[command]
pub fn add_wf_edge(
    graph_id: String,
    source_node_id: String,
    target_node_id: String,
    label: String,
    data_mapping: Option<serde_json::Value>,
) -> Result<WfEdge, String> {
    tracing::info!("Adding edge '{}' to graph {}", label, graph_id);
    let edge = WfEdge {
        id: uuid::Uuid::new_v4().to_string(),
        source_node_id,
        target_node_id,
        label,
        data_mapping: data_mapping.unwrap_or(serde_json::json!({})),
    };
    tracing::info!("Added edge: {} (id={})", edge.label, edge.id);
    Ok(edge)
}

/// 移除工作流边
#[command]
pub fn remove_wf_edge(graph_id: String, edge_id: String) -> Result<serde_json::Value, String> {
    tracing::info!("Removing edge {} from graph {}", edge_id, graph_id);
    Ok(serde_json::json!({
        "removed_edge_id": edge_id,
        "graph_id": graph_id,
        "status": "success"
    }))
}

/// 添加条件分支
#[command]
pub fn add_wf_conditional_branch(
    graph_id: String,
    node_id: String,
    condition: String,
    target_node_id: String,
    priority: Option<i32>,
) -> Result<WfConditionalBranch, String> {
    tracing::info!(
        "Adding conditional branch to node {} in graph {}: {}",
        node_id, graph_id, condition
    );
    let branch = WfConditionalBranch {
        id: uuid::Uuid::new_v4().to_string(),
        node_id,
        condition,
        target_node_id,
        priority: priority.unwrap_or(0),
    };
    tracing::info!("Added conditional branch: {}", branch.id);
    Ok(branch)
}

/// 验证工作流图
#[command]
pub fn validate_wf_graph(graph_id: String) -> Result<WfValidationResult, String> {
    tracing::info!("Validating workflow graph: {}", graph_id);
    let result = WfValidationResult {
        is_valid: true,
        errors: vec![],
        warnings: vec![
            "节点 '有限元分析' 的网格尺寸较大，建议检查收敛性".to_string(),
        ],
        info: vec![
            "工作流包含 4 个节点、3 条边".to_string(),
            "拓扑结构: 线性串行链 DFT->MD->PF->FE".to_string(),
            "所有节点状态均为 ready".to_string(),
        ],
    };
    tracing::info!(
        "Validation result for graph {}: is_valid={}",
        graph_id, result.is_valid
    );
    Ok(result)
}

/// 列出所有工作流图
#[command]
pub fn list_wf_graphs() -> Result<Vec<WfGraph>, String> {
    tracing::info!("Listing all workflow graphs");
    let graphs = vec![
        mock_wf_graph("Mg-Al 合金蠕变多尺度分析"),
        mock_wf_graph("Ni基高温合金疲劳寿命预测"),
        mock_wf_graph("复合材料界面力学研究"),
    ];
    tracing::info!("Listed {} workflow graphs", graphs.len());
    Ok(graphs)
}
