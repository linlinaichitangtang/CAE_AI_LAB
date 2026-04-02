use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// 插件核心接口定义
// ============================================================================

/// 插件元数据信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub loaded: bool,
    pub path: Option<String>,
    pub registered_solvers: Vec<String>,
    pub registered_importers: Vec<String>,
    pub registered_exporters: Vec<String>,
    pub registered_materials: Vec<String>,
}

/// 求解器输入
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SolverInput {
    pub solver_type: String,
    pub mesh_nodes: Vec<Vec<f64>>,
    pub mesh_elements: Vec<Vec<usize>>,
    pub materials: HashMap<String, serde_json::Value>,
    pub boundary_conditions: Vec<serde_json::Value>,
    pub loads: Vec<serde_json::Value>,
    pub analysis_type: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// 求解器输出
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SolverOutput {
    pub success: bool,
    pub node_results: Option<Vec<Vec<f64>>>,
    pub element_results: Option<Vec<Vec<f64>>>,
    pub convergence: bool,
    pub iterations: usize,
    pub messages: Vec<String>,
    pub error: Option<String>,
}

/// 求解器状态
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SolverStatus {
    Idle,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// 导入器输入
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImporterInput {
    pub file_path: String,
    pub options: HashMap<String, serde_json::Value>,
}

/// 导入器输出
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImporterOutput {
    pub success: bool,
    pub mesh_nodes: Vec<Vec<f64>>,
    pub mesh_elements: Vec<Vec<usize>>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub warnings: Vec<String>,
    pub error: Option<String>,
}

/// 导出器输入
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExporterInput {
    pub output_path: String,
    pub mesh_nodes: Vec<Vec<f64>>,
    pub mesh_elements: Vec<Vec<usize>>,
    pub results: Option<Vec<Vec<f64>>>,
    pub options: HashMap<String, serde_json::Value>,
}

/// 导出器输出
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExporterOutput {
    pub success: bool,
    pub output_file: String,
    pub warnings: Vec<String>,
    pub error: Option<String>,
}

/// 材料定义
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaterialDefinition {
    pub name: String,
    pub category: String,
    pub properties: HashMap<String, serde_json::Value>,
    pub description: Option<String>,
}

/// 插件钩子事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginHookEvent {
    pub event_name: String,
    pub data: serde_json::Value,
    pub timestamp: String,
}
