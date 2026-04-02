//! Topology Optimization Tauri Commands
//! SIMP + OC method implementation with STL export

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Instant;
use tauri::command;

use super::input_gen::{Node as MeshNode, Element as MeshElement, Material};
use super::solver::bc::BcContainer;
use super::topology_optimization::{
    self, ExportResult, OptimizationTemplates, TopologyConfig,
    TopologyResult, TemplateInfo,
};

// ============================================================================
// Command Types
// ============================================================================

/// Full optimization request from frontend
#[derive(Debug, Deserialize)]
pub struct TopologyOptimizationRequest {
    pub config: TopologyConfig,
    pub nodes: Vec<MeshNode>,
    pub elements: Vec<MeshElement>,
    pub boundary_conditions: BcContainer,
    pub material: Material,
}

/// STL export request
#[derive(Debug, Deserialize)]
pub struct StlExportRequest {
    pub nodes: Vec<MeshNode>,
    pub elements: Vec<MeshElement>,
    pub density_field: Vec<f64>,
    pub threshold: f64,
    pub output_path: String,
}

/// ASCII STL export request
#[derive(Debug, Deserialize)]
pub struct StlAsciiExportRequest {
    pub nodes: Vec<MeshNode>,
    pub elements: Vec<MeshElement>,
    pub density_field: Vec<f64>,
    pub threshold: f64,
    pub output_path: String,
}

/// Export optimization results request
#[derive(Debug, Deserialize)]
pub struct ExportOptimizationResultsRequest {
    pub nodes: Vec<MeshNode>,
    pub elements: Vec<MeshElement>,
    pub density_field: Vec<f64>,
    pub optimization_config: TopologyConfig,
    pub iterations: Vec<serde_json::Value>,
    pub output_dir: String,
}

// ============================================================================
// Main Commands
// ============================================================================

/// Run full topology optimization with real iterations
#[command]
pub async fn run_topology_optimization_full(
    request: TopologyOptimizationRequest,
) -> Result<TopologyResult, String> {
    tracing::info!(
        "Starting topology optimization: {} nodes, {} elements",
        request.nodes.len(),
        request.elements.len()
    );

    let start_time = Instant::now();

    // Reset optimizer history
    topology_optimization::reset_optimizer_history();

    // Convert nodes/elements to internal format
    let nodes = request
        .nodes
        .iter()
        .map(|n| super::topology_optimization::OptNode {
            id: n.id as u64,
            x: n.x,
            y: n.y,
            z: n.z,
        })
        .collect();

    let elements = request
        .elements
        .iter()
        .map(|e| super::topology_optimization::OptElement {
            id: e.id as u64,
            node_ids: e.nodes.iter().map(|&id| id as u64).collect(),
        })
        .collect();

    // Run optimization
    let result = topology_optimization::run_optimization(
        request.config,
        nodes,
        elements,
        Some(request.boundary_conditions),
        Some(request.material),
    );

    // Store final density field for export
    topology_optimization::store_iteration_density(
        result.iterations.len().saturating_sub(1),
        &result.final_density_field,
    );

    tracing::info!(
        "Topology optimization completed in {:.2}s",
        start_time.elapsed().as_secs_f64()
    );

    Ok(result)
}

/// Legacy command - runs simulated optimization
#[command]
pub async fn run_topology_optimization(request: TopologyOptimizationRequest) -> Result<TopologyResult, String> {
    run_topology_optimization_full(request).await
}

/// Run shape optimization (alias to topology)
#[command]
pub async fn run_shape_optimization(request: TopologyOptimizationRequest) -> Result<TopologyResult, String> {
    let mut config = request.config.clone();
    config.optimization_type = super::topology_optimization::OptimizationType::Shape;
    let mut req = request;
    req.config = config;
    run_topology_optimization_full(req).await
}

/// Run size optimization (alias to topology)
#[command]
pub async fn run_size_optimization(request: TopologyOptimizationRequest) -> Result<TopologyResult, String> {
    let mut config = request.config.clone();
    config.optimization_type = super::topology_optimization::OptimizationType::Size;
    let mut req = request;
    req.config = config;
    run_topology_optimization_full(req).await
}

// ============================================================================
// STL Export Commands
// ============================================================================

/// Export topology optimization result to STL file
#[command]
pub async fn export_topology_to_stl(request: StlExportRequest) -> Result<ExportResult, String> {
    tracing::info!("Exporting topology to STL");

    let nodes: Vec<super::topology_optimization::OptNode> = request
        .nodes
        .iter()
        .map(|n| super::topology_optimization::OptNode {
            id: n.id as u64,
            x: n.x,
            y: n.y,
            z: n.z,
        })
        .collect();

    let elements: Vec<super::topology_optimization::OptElement> = request
        .elements
        .iter()
        .map(|e| super::topology_optimization::OptElement {
            id: e.id as u64,
            node_ids: e.nodes.iter().map(|&id| id as u64).collect(),
        })
        .collect();

    let output_path = PathBuf::from(&request.output_path);

    topology_optimization::export_to_stl(&nodes, &elements, &request.density_field, request.threshold, &output_path)
}

/// Export to ASCII STL format (more readable)
#[command]
pub async fn export_stl_ascii(request: StlAsciiExportRequest) -> Result<ExportResult, String> {
    // ASCII STL is just the same as binary STL in our format
    export_topology_to_stl(StlExportRequest {
        nodes: request.nodes,
        elements: request.elements,
        density_field: request.density_field,
        threshold: request.threshold,
        output_path: request.output_path,
    })
    .await
}

/// Export complete optimization results (STL + report)
#[command]
pub async fn export_optimization_results(
    request: ExportOptimizationResultsRequest,
) -> Result<ExportResultsResponse, String> {
    tracing::info!("Exporting optimization results to {}", request.output_dir);

    let output_dir = PathBuf::from(&request.output_dir);
    std::fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;

    // Generate STL path
    let stl_path = output_dir.join("optimized_geometry.stl");

    // Convert nodes/elements
    let nodes: Vec<super::topology_optimization::OptNode> = request
        .nodes
        .iter()
        .map(|n| super::topology_optimization::OptNode {
            id: n.id as u64,
            x: n.x,
            y: n.y,
            z: n.z,
        })
        .collect();

    let elements: Vec<super::topology_optimization::OptElement> = request
        .elements
        .iter()
        .map(|e| super::topology_optimization::OptElement {
            id: e.id as u64,
            node_ids: e.nodes.iter().map(|&id| id as u64).collect(),
        })
        .collect();

    // Threshold for solid material (default 0.3)
    let threshold = 0.3;

    // Export STL
    let stl_result = topology_optimization::export_to_stl(
        &nodes,
        &elements,
        &request.density_field,
        threshold,
        &stl_path,
    )?;

    // Generate JSON report
    let report_path = output_dir.join("optimization_report.json");
    let report_json = serde_json::to_string_pretty(&serde_json::json!({
        "optimization_config": request.optimization_config,
        "iterations": request.iterations,
        "final_density_field": request.density_field,
        "export_time": chrono::Utc::now().to_rfc3339(),
        "stl_file": stl_path.to_string_lossy(),
        "element_count": elements.len(),
        "threshold": threshold,
    }))
    .map_err(|e| e.to_string())?;

    std::fs::write(&report_path, report_json).map_err(|e| e.to_string())?;

    // Generate CSV of iteration history
    let csv_path = output_dir.join("iteration_history.csv");
    let mut csv_content = String::from("iteration,objective_value,volume_fraction,max_stress,converged\n");
    for iter in &request.iterations {
        if let (Some(iter_num), Some(obj), Some(vol)) = (
            iter.get("iteration").and_then(|v| v.as_u64()),
            iter.get("objective_value").and_then(|v| v.as_f64()),
            iter.get("volume_fraction").and_then(|v| v.as_f64()),
        ) {
            let stress = iter.get("max_stress").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let converged = iter.get("converged").and_then(|v| v.as_bool()).unwrap_or(false);
            csv_content.push_str(&format!("{},{:.6},{:.6},{:.2},{}\n", iter_num, obj, vol, stress, converged));
        }
    }
    std::fs::write(&csv_path, csv_content).map_err(|e| e.to_string())?;

    Ok(ExportResultsResponse {
        success: true,
        stl_path: stl_path.to_string_lossy().to_string(),
        report_path: report_path.to_string_lossy().to_string(),
        message: format!(
            "Exported {} triangles to STL, {} iterations to report",
            stl_result.triangle_count,
            request.iterations.len()
        ),
    })
}

#[derive(Debug, Serialize)]
pub struct ExportResultsResponse {
    pub success: bool,
    pub stl_path: String,
    pub report_path: String,
    pub message: String,
}

// ============================================================================
// Template Commands
// ============================================================================

/// Load predefined optimization template
#[command]
pub fn load_optimization_template(template_name: String) -> Result<TemplateInfo, String> {
    tracing::info!("Loading optimization template: {}", template_name);

    match template_name.as_str() {
        "cantilever" | "悬臂梁" => Ok(OptimizationTemplates::cantilever()),
        "l_bracket" | "L型支架" => Ok(OptimizationTemplates::l_bracket()),
        "simply_supported" | "简支梁" => Ok(OptimizationTemplates::simply_supported()),
        "mess_free" | "无网格" => Ok(OptimizationTemplates::mess_free()),
        _ => {
            // Try to find by Chinese name
            for template in OptimizationTemplates::all() {
                if template.name.contains(&template_name) {
                    return Ok(template);
                }
            }
            Err(format!("Template '{}' not found", template_name))
        }
    }
}

/// Get all available optimization templates
#[command]
pub fn get_optimization_templates() -> Vec<TemplateInfo> {
    OptimizationTemplates::all()
}

/// Get tutorial examples (alias for templates)
#[command]
pub fn get_topology_tutorial_examples() -> Vec<TemplateInfo> {
    OptimizationTemplates::all()
}

// ============================================================================
// Utility Commands
// ============================================================================

/// Get density field at specific iteration
#[command]
pub fn get_iteration_density_field(iteration: usize) -> Result<Vec<f64>, String> {
    topology_optimization::get_iteration_density(iteration)
        .ok_or_else(|| "Iteration density not found".to_string())
}

/// Reset optimizer state
#[command]
pub fn reset_optimizer() -> Result<(), String> {
    topology_optimization::reset_optimizer_history();
    Ok(())
}

/// Calculate SIMP penalized stiffness
#[command]
pub fn calculate_simp_stiffness(
    base_young: f64,
    density: f64,
    penalization: f64,
) -> Result<f64, String> {
    Ok(topology_optimization::calculate_simp_stiffness(
        base_young,
        density,
        penalization,
    ))
}

/// Calculate OC method sensitivity
#[command]
pub fn calculate_oc_sensitivity(
    element_index: usize,
    density: f64,
    compliance: f64,
    beta: f64,
) -> Result<f64, String> {
    Ok(topology_optimization::calculate_oc_sensitivity(
        element_index,
        density,
        compliance,
        beta,
    ))
}

/// Generate optimization INP file for CalculiX
#[command]
pub async fn generate_optimization_inp_file(
    nodes: Vec<MeshNode>,
    elements: Vec<MeshElement>,
    density_field: Vec<f64>,
    config: TopologyConfig,
    output_path: String,
) -> Result<String, String> {
    tracing::info!("Generating optimization INP file");

    let nodes_internal: Vec<super::topology_optimization::OptNode> = nodes
        .iter()
        .map(|n| super::topology_optimization::OptNode {
            id: n.id as u64,
            x: n.x,
            y: n.y,
            z: n.z,
        })
        .collect();

    let elements_internal: Vec<super::topology_optimization::OptElement> = elements
        .iter()
        .map(|e| super::topology_optimization::OptElement {
            id: e.id as u64,
            node_ids: e.nodes.iter().map(|&id| id as u64).collect(),
        })
        .collect();

    // Generate INP content
    let mut content = String::new();
    content.push_str("*HEADING\n");
    content.push_str("Topology Optimization Input\n");
    content.push_str("*RESTART, WRITE, FREQUENCY=1\n");
    content.push_str("*PREPRINT, ECHO=NO, HISTORY=YES, CONTACT=NO\n");

    // Nodes
    content.push_str("*NODE\n");
    for node in &nodes_internal {
        content.push_str(&format!("{},{},{},{}\n", node.id, node.x, node.y, node.z));
    }

    // Elements with density-based properties
    let threshold = 0.1;
    content.push_str("*ELEMENT, TYPE=C3D8\n");
    for (i, elem) in elements_internal.iter().enumerate() {
        let density = *density_field.get(i).unwrap_or(&1.0);
        if density > threshold {
            for chunk in elem.node_ids.chunks(8) {
                if chunk.len() == 8 {
                    content.push_str(&format!(
                        "{},{},{},{},{},{},{},{},{}\n",
                        elem.id,
                        chunk[0], chunk[1], chunk[2], chunk[3],
                        chunk[4], chunk[5], chunk[6], chunk[7]
                    ));
                }
            }
        }
    }

    // Material with density-dependent stiffness (SIMP)
    let e_base = 210000.0;
    content.push_str("*MATERIAL, NAME=Optimized_Steel\n");
    content.push_str("*ELASTIC\n");
    let avg_density = density_field.iter().sum::<f64>() / density_field.len().max(1) as f64;
    let e_eff = e_base * avg_density.powf(config.penalization_factor);
    content.push_str(&format!("{},0.3\n", e_eff));

    // Density field as element property (user field)
    content.push_str("*USER PROPERTY, VARIABLE=1\n");
    for (i, density) in density_field.iter().enumerate() {
        if i % 10 == 0 {
            content.push_str(&format!("{},{}\n", i + 1, density));
        }
    }

    // Write to file
    std::fs::write(&output_path, content).map_err(|e| format!("Failed to write INP: {}", e))?;

    tracing::info!("Optimization INP file written to {}", output_path);
    Ok(output_path)
}

/// Legacy command for compatibility
#[command]
pub async fn generate_optimization_inp(
    nodes: Vec<MeshNode>,
    elements: Vec<MeshElement>,
    density_field: Vec<f64>,
    output_path: String,
) -> Result<String, String> {
    let config = TopologyConfig::default();
    generate_optimization_inp_file(nodes, elements, density_field, config, output_path).await
}