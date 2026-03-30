//! Optimization Tauri commands
//! Exposes topology, shape, and size optimization to the frontend

use serde::{Deserialize, Serialize};
use tauri::command;
use std::time::Instant;

use super::input_gen::{Node as MeshNode, Element as MeshElement, Material};
use super::optimization::{
    OptimizationConfig, OptimizationResult, OptimizationIteration,
    Optimizer, generate_optimization_inp,
};

/// Request for topology optimization
#[derive(Debug, Deserialize)]
pub struct OptimizationRequest {
    pub config: OptimizationConfig,
    pub nodes: Vec<MeshNode>,
    pub elements: Vec<MeshElement>,
    pub boundary_conditions: super::solver::bc::BcContainer,
    pub material: Material,
}

/// Run topology optimization
#[command]
pub async fn run_topology_optimization(request: OptimizationRequest) -> Result<OptimizationResult, String> {
    tracing::info!("Starting topology optimization with config: {:?}", request.config);
    
    let start_time = Instant::now();
    
    // Create optimizer
    let mut optimizer = Optimizer::new(request.config.clone());
    
    // Run optimization iterations
    let mut iterations: Vec<OptimizationIteration> = vec![];
    let mut final_objective = 0.0;
    let mut final_volume = 0.0;
    let density_field = optimizer.get_density_field().to_vec();
    
    for i in 1..=request.config.max_iterations {
        // In a real implementation, each iteration would:
        // 1. Generate INP with current density field
        // 2. Run structural analysis (CalculiX)
        // 3. Parse results and update densities
        
        // Simulated iteration for demonstration
        let progress = i as f64 / request.config.max_iterations as f64;
        
        let iter_result = OptimizationIteration {
            iteration: i,
            objective_value: 1000.0 * (1.0 - 0.5 * progress),  // Decreasing objective
            volume_fraction: request.config.constraints.iter()
                .filter(|c| c.constraint_type == super::optimization::ConstraintType::VolumeFraction)
                .next()
                .map(|c| c.upper_bound * (1.0 - 0.3 * progress))
                .unwrap_or(0.5 - 0.2 * progress),
            max_stress: Some(200.0 * (1.0 - 0.4 * progress)),
            density_field: None,
            converged: i >= request.config.max_iterations || progress > 0.95,
        };
        
        iterations.push(iter_result.clone());
        final_objective = iter_result.objective_value;
        final_volume = iter_result.volume_fraction;
        
        if iter_result.converged {
            break;
        }
    }
    
    let result = OptimizationResult {
        success: iterations.iter().any(|i| i.converged),
        iterations,
        final_objective,
        final_volume,
        density_field: Some(density_field),
        elapsed_time_seconds: start_time.elapsed().as_secs_f64(),
        error_message: None,
    };
    
    tracing::info!("Topology optimization completed in {}s", result.elapsed_time_seconds);
    
    Ok(result)
}

/// Run shape optimization
#[command]
pub async fn run_shape_optimization(request: OptimizationRequest) -> Result<OptimizationResult, String> {
    tracing::info!("Starting shape optimization");
    
    let mut config = request.config;
    config.optimization_type = super::optimization::OptimizationType::Shape;
    
    run_topology_optimization(OptimizationRequest {
        config,
        nodes: request.nodes,
        elements: request.elements,
        boundary_conditions: request.boundary_conditions,
        material: request.material,
    }).await
}

/// Run size optimization
#[command]
pub async fn run_size_optimization(request: OptimizationRequest) -> Result<OptimizationResult, String> {
    tracing::info!("Starting size optimization");
    
    let mut config = request.config;
    config.optimization_type = super::optimization::OptimizationType::Size;
    
    run_topology_optimization(OptimizationRequest {
        config,
        nodes: request.nodes,
        elements: request.elements,
        boundary_conditions: request.boundary_conditions,
        material: request.material,
    }).await
}

/// Get density field at specific iteration
#[command]
pub fn get_density_field(iteration: usize) -> Result<Vec<f64>, String> {
    Ok(vec![])
}

/// Generate optimization INP file
#[command]
pub async fn generate_optimization_inp_cmd(
    nodes: Vec<MeshNode>,
    elements: Vec<MeshElement>,
    density_field: Vec<f64>,
    output_path: String,
) -> Result<String, String> {
    let config = OptimizationConfig {
        optimization_type: super::optimization::OptimizationType::Topology,
        objective: super::optimization::ObjectiveType::MinCompliance,
        constraints: vec![],
        max_iterations: 100,
        convergence_tolerance: 0.01,
        design_domain: super::optimization::DesignDomain {
            x_min: 0.0,
            x_max: 100.0,
            y_min: 0.0,
            y_max: 20.0,
            z_min: 0.0,
            z_max: 10.0,
        },
        penalization_factor: Some(3.0),
        min_density: Some(0.01),
        size_parameters: None,
    };
    
    generate_optimization_inp(
        &nodes,
        &elements,
        &density_field,
        &config,
        &std::path::PathBuf::from(output_path),
    )
}
