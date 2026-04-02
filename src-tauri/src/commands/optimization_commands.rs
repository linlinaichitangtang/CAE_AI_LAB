//! Topology Optimization Tauri Commands
//! SIMP + OC method implementation with STL export

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
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

// ============================================================================
// Shape Optimization (V1.1-006)
// ============================================================================

/// Shape optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeOptConfig {
    pub mesh_nodes: Vec<[f64; 3]>,
    pub mesh_elements: Vec<[usize; 4]>, // tetrahedra
    pub boundary_node_ids: Vec<usize>,
    pub fixed_node_ids: Vec<usize>,
    pub load_node_ids: Vec<usize>,
    pub load_values: Vec<f64>,
    pub objective: String, // "minimize_compliance"
    pub max_movement: f64,
    pub max_iterations: u32,
    pub convergence_tolerance: f64,
}

/// Shape optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeOptResult {
    pub optimized_nodes: Vec<[f64; 3]>,
    pub iterations: Vec<ShapeOptIteration>,
    pub converged: bool,
    pub final_compliance: f64,
    pub elapsed_time_seconds: f64,
    pub error_message: Option<String>,
}

/// Single shape optimization iteration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeOptIteration {
    pub iteration: u32,
    pub compliance: f64,
    pub max_movement: f64,
    pub node_displacements: Vec<[f64; 3]>,
}

/// Run shape optimization with boundary node movement
#[command]
pub async fn run_shape_optimization_v2(
    config: ShapeOptConfig,
) -> Result<ShapeOptResult, String> {
    tracing::info!(
        "Starting shape optimization: {} nodes, {} elements, {} boundary nodes",
        config.mesh_nodes.len(),
        config.mesh_elements.len(),
        config.boundary_node_ids.len()
    );

    let start_time = std::time::Instant::now();

    if config.mesh_nodes.is_empty() {
        return Err("Mesh nodes cannot be empty".to_string());
    }
    if config.boundary_node_ids.is_empty() {
        return Err("Boundary node IDs cannot be empty".to_string());
    }

    let node_count = config.mesh_nodes.len();
    let mut current_nodes = config.mesh_nodes.clone();

    // Build boundary node set for fast lookup
    let _boundary_set: std::collections::HashSet<usize> =
        config.boundary_node_ids.iter().cloned().collect();
    let fixed_set: std::collections::HashSet<usize> =
        config.fixed_node_ids.iter().cloned().collect();

    // Compute initial compliance (simplified)
    let initial_compliance = compute_shape_compliance(&current_nodes, &config);

    let mut iterations = Vec::new();
    let mut prev_compliance = initial_compliance;
    let mut converged = false;

    for iter in 1..=config.max_iterations {
        // Compute simplified gradient for each boundary node
        let mut displacements = vec![[0.0_f64; 3]; node_count];
        let mut max_move = 0.0_f64;

        for &bid in &config.boundary_node_ids {
            if fixed_set.contains(&bid) {
                continue;
            }
            if bid >= node_count {
                continue;
            }

            // Simplified gradient: move nodes toward load application direction
            // with a bias toward reducing compliance
            let node = current_nodes[bid];

            // Compute direction toward center of load nodes
            let mut load_center = [0.0_f64; 3];
            let load_count = config.load_node_ids.len().max(1);
            for &lid in &config.load_node_ids {
                if lid < node_count {
                    load_center[0] += current_nodes[lid][0];
                    load_center[1] += current_nodes[lid][1];
                    load_center[2] += current_nodes[lid][2];
                }
            }
            load_center[0] /= load_count as f64;
            load_center[1] /= load_count as f64;
            load_center[2] /= load_count as f64;

            // Direction from node to load center
            let mut dir = [
                load_center[0] - node[0],
                load_center[1] - node[1],
                load_center[2] - node[2],
            ];
            let len = (dir[0].powi(2) + dir[1].powi(2) + dir[2].powi(2)).sqrt();
            if len < 1e-10 {
                continue;
            }
            dir[0] /= len;
            dir[1] /= len;
            dir[2] /= len;

            // Add smoothing: average with neighbor directions
            let mut smooth_dir = [0.0_f64; 3];
            let mut neighbor_count = 0;
            for elem in &config.mesh_elements {
                let mut has_bid = false;
                for &nid in elem {
                    if nid == bid {
                        has_bid = true;
                    }
                }
                if has_bid {
                    for &nid in elem {
                        if nid != bid && nid < node_count {
                            smooth_dir[0] += current_nodes[nid][0] - node[0];
                            smooth_dir[1] += current_nodes[nid][1] - node[1];
                            smooth_dir[2] += current_nodes[nid][2] - node[2];
                            neighbor_count += 1;
                        }
                    }
                }
            }

            if neighbor_count > 0 {
                let slen = (smooth_dir[0].powi(2) + smooth_dir[1].powi(2) + smooth_dir[2].powi(2)).sqrt();
                if slen > 1e-10 {
                    smooth_dir[0] /= slen;
                    smooth_dir[1] /= slen;
                    smooth_dir[2] /= slen;
                    // Blend: 60% load direction + 40% smoothing
                    dir[0] = 0.6 * dir[0] + 0.4 * smooth_dir[0];
                    dir[1] = 0.6 * dir[1] + 0.4 * smooth_dir[1];
                    dir[2] = 0.6 * dir[2] + 0.4 * smooth_dir[2];
                }
            }

            // Step size decreases with iteration (gradient descent with decay)
            let step = config.max_movement * (1.0 - (iter as f64 / config.max_iterations as f64) * 0.5);
            let move_vec = [dir[0] * step, dir[1] * step, dir[2] * step];

            let move_mag = (move_vec[0].powi(2) + move_vec[1].powi(2) + move_vec[2].powi(2)).sqrt();
            if move_mag > config.max_movement {
                let scale = config.max_movement / move_mag;
                displacements[bid] = [move_vec[0] * scale, move_vec[1] * scale, move_vec[2] * scale];
            } else {
                displacements[bid] = move_vec;
            }

            let dm = (displacements[bid][0].powi(2) + displacements[bid][1].powi(2) + displacements[bid][2].powi(2)).sqrt();
            if dm > max_move {
                max_move = dm;
            }
        }

        // Apply displacements
        for i in 0..node_count {
            current_nodes[i][0] += displacements[i][0];
            current_nodes[i][1] += displacements[i][1];
            current_nodes[i][2] += displacements[i][2];
        }

        // Compute compliance
        let compliance = compute_shape_compliance(&current_nodes, &config);

        // Store iteration
        iterations.push(ShapeOptIteration {
            iteration: iter,
            compliance,
            max_movement: max_move,
            node_displacements: displacements,
        });

        // Check convergence
        if iter > 1 {
            let compliance_change = (compliance - prev_compliance).abs();
            if compliance_change < config.convergence_tolerance * prev_compliance.abs().max(1e-10) {
                converged = true;
                tracing::info!(
                    "Shape optimization converged at iteration {}: compliance change = {:.6}",
                    iter,
                    compliance_change
                );
                break;
            }
        }

        prev_compliance = compliance;

        tracing::info!(
            "Shape opt iteration {}: compliance = {:.4}, max_move = {:.6}",
            iter,
            compliance,
            max_move
        );
    }

    let elapsed = start_time.elapsed().as_secs_f64();
    let final_compliance = compute_shape_compliance(&current_nodes, &config);

    tracing::info!(
        "Shape optimization completed in {:.2}s, {} iterations, converged={}",
        elapsed,
        iterations.len(),
        converged
    );

    Ok(ShapeOptResult {
        optimized_nodes: current_nodes,
        iterations,
        converged,
        final_compliance,
        elapsed_time_seconds: elapsed,
        error_message: None,
    })
}

/// Simplified compliance computation for shape optimization
/// Uses a proxy based on strain energy approximation
fn compute_shape_compliance(nodes: &[[f64; 3]], config: &ShapeOptConfig) -> f64 {
    // Simplified: sum of squared distances from load nodes to fixed nodes,
    // weighted by load values. This is a proxy for compliance.
    let mut compliance = 0.0;

    let fixed_center = if !config.fixed_node_ids.is_empty() {
        let mut cx = 0.0;
        let mut cy = 0.0;
        let mut cz = 0.0;
        let count = config.fixed_node_ids.len().max(1) as f64;
        for &fid in &config.fixed_node_ids {
            if fid < nodes.len() {
                cx += nodes[fid][0];
                cy += nodes[fid][1];
                cz += nodes[fid][2];
            }
        }
        [cx / count, cy / count, cz / count]
    } else {
        [0.0, 0.0, 0.0]
    };

    for (i, &lid) in config.load_node_ids.iter().enumerate() {
        if lid >= nodes.len() {
            continue;
        }
        let load_val = config.load_values.get(i).copied().unwrap_or(1.0);
        let dx = nodes[lid][0] - fixed_center[0];
        let dy = nodes[lid][1] - fixed_center[1];
        let dz = nodes[lid][2] - fixed_center[2];
        let dist_sq = dx * dx + dy * dy + dz * dz;
        compliance += load_val * dist_sq;
    }

    // Normalize by element count to keep values reasonable
    let elem_count = config.mesh_elements.len().max(1) as f64;
    compliance / elem_count
}

/// Export shape optimization result to STL
#[command]
pub async fn export_shape_opt_to_stl(
    nodes: Vec<[f64; 3]>,
    elements: Vec<[usize; 4]>,
    output_path: String,
) -> Result<ExportResult, String> {
    tracing::info!(
        "Exporting shape optimization to STL: {} nodes, {} elements",
        nodes.len(),
        elements.len()
    );

    let path = PathBuf::from(&output_path);
    let file = File::create(&path).map_err(|e| format!("Failed to create file: {}", e))?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "solid shape_optimization").map_err(|e| e.to_string())?;

    let mut triangle_count = 0u64;

    for elem in &elements {
        let v0 = nodes.get(elem[0]).copied().unwrap_or([0.0; 3]);
        let v1 = nodes.get(elem[1]).copied().unwrap_or([0.0; 3]);
        let v2 = nodes.get(elem[2]).copied().unwrap_or([0.0; 3]);
        let v3 = nodes.get(elem[3]).copied().unwrap_or([0.0; 3]);

        // Tetrahedron -> 4 triangles
        write_triangle_shape(&mut writer, &v0, &v1, &v2)?;
        write_triangle_shape(&mut writer, &v0, &v2, &v3)?;
        write_triangle_shape(&mut writer, &v0, &v3, &v1)?;
        write_triangle_shape(&mut writer, &v1, &v3, &v2)?;
        triangle_count += 4;
    }

    writeln!(writer, "endsolid shape_optimization").map_err(|e| e.to_string())?;

    Ok(ExportResult {
        success: true,
        file_path: output_path,
        element_count: elements.len(),
        node_count: nodes.len(),
        triangle_count: triangle_count as usize,
    })
}

/// Write a single triangle for shape STL export
fn write_triangle_shape(
    writer: &mut BufWriter<File>,
    v0: &[f64; 3],
    v1: &[f64; 3],
    v2: &[f64; 3],
) -> Result<(), String> {
    let ax = v1[0] - v0[0];
    let ay = v1[1] - v0[1];
    let az = v1[2] - v0[2];
    let bx = v2[0] - v0[0];
    let by = v2[1] - v0[1];
    let bz = v2[2] - v0[2];

    let nx = ay * bz - az * by;
    let ny = az * bx - ax * bz;
    let nz = ax * by - ay * bx;

    let len = (nx * nx + ny * ny + nz * nz).sqrt();
    let (nx, ny, nz) = if len > 1e-10 {
        (nx / len, ny / len, nz / len)
    } else {
        (0.0, 0.0, 1.0)
    };

    writeln!(writer, "  facet normal {} {} {}", nx, ny, nz).map_err(|e| e.to_string())?;
    writeln!(writer, "    outer loop").map_err(|e| e.to_string())?;
    writeln!(writer, "      vertex {} {} {}", v0[0], v0[1], v0[2]).map_err(|e| e.to_string())?;
    writeln!(writer, "      vertex {} {} {}", v1[0], v1[1], v1[2]).map_err(|e| e.to_string())?;
    writeln!(writer, "      vertex {} {} {}", v2[0], v2[1], v2[2]).map_err(|e| e.to_string())?;
    writeln!(writer, "    endloop").map_err(|e| e.to_string())?;
    writeln!(writer, "  endfacet").map_err(|e| e.to_string())?;

    Ok(())
}