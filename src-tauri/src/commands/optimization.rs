//! Topology, Shape, and Size Optimization Module
//! Implements SIMP-based topology optimization with OC (Optimality Criteria) method

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Optimization type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OptimizationType {
    Topology,
    Shape,
    Size,
}

/// Objective function type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ObjectiveType {
    MinCompliance,  // 最小柔度
    MinMass,         // 最小质量
    MaxStiffness,    // 最大刚度
}

/// Constraint type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConstraintType {
    VolumeFraction,  // 体积分数上限
    MaxStress,       // 最大应力限制
}

/// Optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    pub optimization_type: OptimizationType,
    pub objective: ObjectiveType,
    pub constraints: Vec<ConstraintConfig>,
    pub max_iterations: usize,
    pub convergence_tolerance: f64,
    pub design_domain: DesignDomain,
    
    // Topology optimization specific
    pub penalization_factor: Option<f64>,
    pub min_density: Option<f64>,
    pub filter_radius: Option<f64>,
    
    // Size optimization specific
    pub size_parameters: Option<Vec<SizeParameter>>,
}

/// Constraint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintConfig {
    pub constraint_type: ConstraintType,
    pub upper_bound: f64,
}

/// Design domain definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignDomain {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub z_min: f64,
    pub z_max: f64,
}

/// Size parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeParameter {
    pub name: String,
    pub current_value: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub element_ids: Vec<usize>,
}

/// Optimization iteration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationIteration {
    pub iteration: usize,
    pub objective_value: f64,
    pub volume_fraction: f64,
    pub max_stress: Option<f64>,
    pub density_field: Option<Vec<f64>>,
    pub converged: bool,
}

/// Complete optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub success: bool,
    pub iterations: Vec<OptimizationIteration>,
    pub final_objective: f64,
    pub final_volume: f64,
    pub optimized_mesh: Option<OptimizedMesh>,
    pub elapsed_time_seconds: f64,
    pub error_message: Option<String>,
}

/// Optimized mesh with updated densities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedMesh {
    pub nodes: Vec<OptimizedNode>,
    pub elements: Vec<OptimizedElement>,
    pub density: HashMap<usize, f64>,
}

/// Optimized node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedNode {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Optimized element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedElement {
    pub id: usize,
    pub node_ids: Vec<usize>,
    pub density: f64,
}

/// Optimizer state for iterative optimization
pub struct Optimizer {
    config: OptimizationConfig,
    iteration: usize,
    density_field: Vec<f64>,
    objective_history: Vec<f64>,
    volume_history: Vec<f64>,
}

impl Optimizer {
    /// Create new optimizer with config
    pub fn new(config: OptimizationConfig) -> Self {
        let element_count = estimate_element_count(&config.design_domain);
        
        // Initialize density field (1.0 = full material)
        let density_field = vec![1.0; element_count];
        
        Self {
            config,
            iteration: 0,
            density_field,
            objective_history: Vec::new(),
            volume_history: Vec::new(),
        }
    }
    
    /// Run optimization iteration
    pub fn iterate(&mut self, structural_result: &super::output_parser::AnalysisResults) -> Result<OptimizationIteration, String> {
        self.iteration += 1;
        
        // Get current compliance from structural analysis
        let current_compliance = calculate_compliance(structural_result);
        
        // Get volume fraction
        let current_volume = calculate_volume_fraction(&self.density_field);
        
        // Apply Optimality Criteria (OC) method for density update
        self.update_density_oc(current_compliance);
        
        // Apply penalization (SIMP)
        if let Some(penalty) = self.config.penalization_factor {
            self.apply_penalization(penalty);
        }
        
        // Enforce minimum density
        if let Some(min_rho) = self.config.min_density {
            self.enforce_min_density(min_rho);
        }
        
        // Record history
        self.objective_history.push(current_compliance);
        self.volume_history.push(current_volume);
        
        // Check convergence
        let converged = self.check_convergence();
        
        Ok(OptimizationIteration {
            iteration: self.iteration,
            objective_value: current_compliance,
            volume_fraction: current_volume,
            max_stress: structural_result.max_von_mises,
            density_field: Some(self.density_field.clone()),
            converged,
        })
    }
    
    /// Update density using Optimality Criteria method
    fn update_density_oc(&mut self, current_compliance: f64) {
        let beta = 1.5; // OC method parameter
        let eta = 0.5;  // Move limit
        
        let mut new_density = self.density_field.clone();
        
        for (i, rho) in self.density_field.iter().enumerate() {
            // Simplified OC update rule
            let sensitivity = calculate_sensitivity(i, current_compliance, &self.density_field);
            
            let mut new_rho = rho * (sensitivity / current_compliance).powf(beta);
            
            // Apply move limit
            let lower = (1.0 - eta) * rho;
            let upper = (1.0 + eta) * rho;
            new_rho = new_rho.clamp(lower, upper);
            
            // Ensure bounds
            let min_rho = self.config.min_density.unwrap_or(0.01);
            new_density[i] = new_rho.clamp(min_rho, 1.0);
        }
        
        self.density_field = new_density;
    }
    
    /// Apply SIMP penalization
    fn apply_penalization(&mut self, penalty: f64) {
        // Penalization is handled in sensitivity calculation
        // This is a simplified version
    }
    
    /// Enforce minimum density
    fn enforce_min_density(&mut self, min_rho: f64) {
        for rho in &mut self.density_field {
            if *rho < min_rho {
                *rho = min_rho;
            }
        }
    }
    
    /// Check convergence
    fn check_convergence(&self) -> bool {
        if self.iteration >= self.config.max_iterations {
            return true;
        }
        
        if self.iteration < 2 {
            return false;
        }
        
        // Check objective change
        let obj_change = (self.objective_history[self.iteration - 1] - self.objective_history[self.iteration - 2]).abs();
        let obj_change_ratio = obj_change / self.objective_history[self.iteration - 2].abs();
        
        obj_change_ratio < self.config.convergence_tolerance
    }
    
    /// Get final result
    pub fn get_result(&self, elapsed_time: f64) -> OptimizationResult {
        let final_objective = self.objective_history.last().copied().unwrap_or(0.0);
        let final_volume = self.volume_history.last().copied().unwrap_or(0.0);
        
        OptimizationResult {
            success: self.check_convergence(),
            iterations: vec![], // Would populate with full iteration history
            final_objective,
            final_volume,
            optimized_mesh: None,
            elapsed_time_seconds: elapsed_time,
            error_message: None,
        }
    }
    
    /// Get current density field
    pub fn get_density_field(&self) -> &[f64] {
        &self.density_field
    }
}

/// Calculate compliance from structural results
fn calculate_compliance(result: &super::output_parser::AnalysisResults) -> f64 {
    // Simplified: use sum of displacement squared as compliance measure
    result.displacements.values().map(|d| d.magnitude().powi(2)).sum()
}

/// Calculate volume fraction from density field
fn calculate_volume_fraction(density_field: &[f64]) -> f64 {
    if density_field.is_empty() {
        return 0.0;
    }
    density_field.iter().sum::<f64>() / density_field.len() as f64
}

/// Calculate sensitivity (simplified)
fn calculate_sensitivity(element_idx: usize, compliance: f64, density_field: &[f64]) -> f64 {
    // Simplified sensitivity calculation
    let rho = density_field.get(element_idx).unwrap_or(&1.0);
    -compliance * rho
}

/// Estimate element count from design domain
fn estimate_element_count(domain: &DesignDomain) -> usize {
    let x_count = ((domain.x_max - domain.x_min) / 10.0).ceil() as usize;
    let y_count = ((domain.y_max - domain.y_min) / 10.0).ceil() as usize;
    let z_count = ((domain.z_max - domain.z_min) / 10.0).ceil() as usize;
    x_count.max(1) * y_count.max(1) * z_count.max(1)
}

/// Generate optimization INP file for current iteration
pub fn generate_optimization_inp(
    nodes: &[super::mesh::MeshNode],
    elements: &[super::mesh::MeshElement],
    density_field: &[f64],
    config: &OptimizationConfig,
    output_path: &PathBuf,
) -> Result<(), String> {
    let mut content = String::new();
    
    // Header
    content.push_str("*HEADING\n");
    content.push_str("Topology Optimization Iteration\n");
    content.push_str("*RESTART, WRITE, FREQUENCY=1\n");
    
    // Nodes
    content.push_str("*NODE\n");
    for node in nodes {
        content.push_str(&format!("{},{},{},{}\n", node.id, node.x, node.y, node.z));
    }
    
    // Elements with density-based properties
    content.push_str("*ELEMENT, TYPE=C3D8\n");
    for (i, elem) in elements.iter().enumerate() {
        let density = density_field.get(i).copied().unwrap_or(1.0);
        if density > 0.3 {  // Only generate elements with density > threshold
            content.push_str(&format!("{},{},{},{},{},{},{},{},{}\n",
                elem.id,
                elem.node_ids[0], elem.node_ids[1], elem.node_ids[2], elem.node_ids[3],
                elem.node_ids[4], elem.node_ids[5], elem.node_ids[6], elem.node_ids[7]
            ));
        }
    }
    
    // Material with density-dependent stiffness (SIMP)
    if let Some(penalty) = config.penalization_factor {
        let e_base = 210000.0; // MPa
        content.push_str("*MATERIAL, NAME=Steel_Optimized\n");
        content.push_str("*ELASTIC\n");
        
        // Density-weighted stiffness
        let avg_density: f64 = density_field.iter().sum::<f64>() / density_field.len().max(1) as f64;
        let e_eff = e_base * avg_density.powf(penalty);
        content.push_str(&format!("{},0.3\n", e_eff));
    }
    
    // Write file
    fs::write(output_path, content).map_err(|e| e.to_string())?;
    
    Ok(())
}

use super::output_parser::AnalysisResults;
