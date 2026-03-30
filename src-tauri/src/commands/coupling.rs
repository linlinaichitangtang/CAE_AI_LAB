//! Thermomechanical coupling analysis
//! Supports:
//!   - Sequential coupling: thermal analysis first, then structural with temperature loads
//!   - Fully coupled: simultaneous temperature-displacement solution

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CouplingError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid temperature data: {0}")]
    InvalidTemperatureData(String),
    #[error("Missing thermal result: {0}")]
    MissingThermalResult(String),
}

/// Coupling analysis type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CouplingType {
    /// Sequential: run thermal first, then structural with temperature loads
    Sequential,
    /// Fully coupled: simultaneous temperature-displacement
    FullyCoupled,
}

/// Temperature field source
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemperatureSource {
    /// Import from previous thermal analysis result
    FromThermalResult { result_file: String },
    /// Import from CSV file (node_id, temperature)
    FromCsvFile { file_path: String },
    /// Uniform temperature across entire model
    Uniform { temperature: f64 },
}

/// Thermal-structural coupling analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouplingAnalysisConfig {
    pub coupling_type: CouplingType,
    pub temperature_source: TemperatureSource,
    /// Reference temperature (stress-free temperature) in K
    pub reference_temperature: f64,
    /// Stress-free temperature in K (alias for reference_temperature)
    pub stress_free_temperature: f64,
}

impl Default for CouplingAnalysisConfig {
    fn default() -> Self {
        Self {
            coupling_type: CouplingType::Sequential,
            temperature_source: TemperatureSource::Uniform { temperature: 293.15 },
            reference_temperature: 293.15,
            stress_free_temperature: 293.15,
        }
    }
}

/// Temperature field data extracted from thermal analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureField {
    pub node_temperatures: Vec<(usize, f64)>, // (node_id, temperature)
}

impl TemperatureField {
    /// Parse temperature data from CSV file
    /// Expected format: node_id,temperature (header optional)
    pub fn from_csv(path: &PathBuf) -> Result<Self, CouplingError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| CouplingError::IoError(e))?;
        
        let mut node_temperatures = vec![];
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            // Skip header if present
            if line.to_lowercase().contains("node") && line.to_lowercase().contains("temp") {
                continue;
            }
            
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 2 {
                let node_id = parts[0].trim().parse::<usize>()
                    .map_err(|_| CouplingError::InvalidTemperatureData(
                        format!("Invalid node ID: {}", parts[0])
                    ))?;
                let temperature = parts[1].trim().parse::<f64>()
                    .map_err(|_| CouplingError::InvalidTemperatureData(
                        format!("Invalid temperature: {}", parts[1])
                    ))?;
                node_temperatures.push((node_id, temperature));
            }
        }
        
        if node_temperatures.is_empty() {
            return Err(CouplingError::InvalidTemperatureData(
                "No valid temperature data found in CSV".to_string()
            ));
        }
        
        Ok(Self { node_temperatures })
    }
    
    /// Create uniform temperature field
    pub fn uniform(node_ids: &[usize], temperature: f64) -> Self {
        let node_temperatures = node_ids.iter()
            .map(|id| (*id, temperature))
            .collect();
        Self { node_temperatures }
    }
}

/// Generate INP content for sequential coupling
/// Step 1: Thermal analysis
/// Step 2: Structural analysis with temperature loads
pub fn generate_sequential_coupling_inp(
    model: &crate::commands::input_gen::Model,
    coupling_config: &CouplingAnalysisConfig,
    temperature_field: &TemperatureField,
    thermal_step_time: f64,
    structural_step_time: f64,
) -> Result<String, CouplingError> {
    use std::fmt::Write as FmtWrite;
    
    let mut output = String::new();
    
    // ===== HEADER =====
    writeln!(output, "** ==================================================").unwrap();
    writeln!(output, "** Thermomechanical Coupling Analysis - Sequential").unwrap();
    writeln!(output, "** Coupling: Temperature field from {:?}", coupling_config.temperature_source).unwrap();
    writeln!(output, "** ==================================================").unwrap();
    writeln!(output).unwrap();
    
    // ===== NODES =====
    writeln!(output, "*HEADING").unwrap();
    writeln!(output, "Thermal-Structural Coupling Analysis").unwrap();
    writeln!(output).unwrap();
    writeln!(output, "** ------------------- NODES -------------------").unwrap();
    writeln!(output, "*NODE").unwrap();
    for node in &model.nodes {
        writeln!(output, "{}, {:.6}, {:.6}, {:.6}", node.id, node.x, node.y, node.z).unwrap();
    }
    writeln!(output).unwrap();
    
    // ===== ELEMENTS =====
    writeln!(output, "** ------------------- ELEMENTS -------------------").unwrap();
    // Group by element type
    let elem_by_type: std::collections::HashMap<String, Vec<&crate::commands::input_gen::Element>> = 
        model.elements.iter().fold(std::collections::HashMap::new(), |mut acc, elem| {
            acc.entry(elem.element_type.as_str().to_string())
                .or_insert_with(Vec::new)
                .push(elem);
            acc
        });
    
    for (elem_type, elements) in elem_by_type {
        // Write element connectivity
        writeln!(output, "*ELEMENT, TYPE={}", elem_type).unwrap();
        for elem in &elements {
            let nodes_str = elem.nodes.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(output, "{}, {}", elem.id, nodes_str).unwrap();
        }
        writeln!(output).unwrap();
    }
    
    // ===== MATERIALS =====
    writeln!(output, "** ------------------- MATERIALS -------------------").unwrap();
    for (i, material) in model.materials.iter().enumerate() {
        writeln!(output, "** Material: {}", material.name).unwrap();
        writeln!(output, "*MATERIAL, NAME={}", material.name).unwrap();
        
        // Elastic properties
        writeln!(output, "*ELASTIC").unwrap();
        writeln!(output, "{}, POISSON={:.6}", material.youngs_modulus, material.poisson_ratio).unwrap();
        
        // Density
        if material.density > 0.0 {
            writeln!(output, "*DENSITY").unwrap();
            writeln!(output, "{:.6}", material.density).unwrap();
        }
        
        // Thermal expansion coefficient
        if let Some(alpha) = material.expansion_coefficient {
            writeln!(output, "*EXPANSION").unwrap();
            writeln!(output, "{:.6}", alpha).unwrap();
        }
        
        // Thermal properties (for heat transfer step)
        if let (Some(k), Some(cp)) = (material.thermal_conductivity, material.specific_heat) {
            writeln!(output, "*CONDUCTIVITY").unwrap();
            writeln!(output, "{:.6}", k).unwrap();
            writeln!(output, "*SPECIFIC HEAT").unwrap();
            writeln!(output, "{:.6}", cp).unwrap();
        }
        
        writeln!(output).unwrap();
    }
    
    // ===== SECTIONS =====
    writeln!(output, "** ------------------- SECTIONS -------------------").unwrap();
    writeln!(output, "*SOLID SECTION, ELSET=EALL, MATERIAL={}", 
        model.materials.first().map(|m| m.name.as_str()).unwrap_or("Material-1")).unwrap();
    writeln!(output).unwrap();
    
    // ===== STEP 1: THERMAL ANALYSIS =====
    writeln!(output, "** ==================================================").unwrap();
    writeln!(output, "** STEP 1: THERMAL ANALYSIS").unwrap();
    writeln!(output, "** ==================================================").unwrap();
    writeln!(output, "*STEP, NAME=Thermal-Step").unwrap();
    writeln!(output, "*HEAT TRANSFER, DELTMX=100.0").unwrap();
    writeln!(output, "{:.6}, {:.6}, {:.6}, {:.6}", 
        thermal_step_time, thermal_step_time * 0.1, thermal_step_time * 0.01, thermal_step_time).unwrap();
    
    // Write thermal boundary conditions from model
    // (Fixed temperature: *BOUNDARY, type=FIXED TEMP)
    // (Heat flux: *DFLUX)
    // (Convection: *FILM)
    if let Some(ref bcs) = model.boundary_conditions.first() {
        // Fixed temperature BCs
        if bcs.fix_temp {
            writeln!(output, "** Fixed temperature boundaries").unwrap();
            writeln!(output, "*BOUNDARY").unwrap();
            for node_id in &bcs.nodes {
                writeln!(output, "{}, 11, 11, 293.15", node_id).unwrap(); // DOF 11 = temperature
            }
            writeln!(output).unwrap();
        }
        
        // Heat flux BCs
        for load in &model.loads {
            match load.load_type {
                crate::commands::input_gen::LoadType::HeatFlux => {
                    writeln!(output, "*DFLUX").unwrap();
                    writeln!(output, "EALL, BFNU, {:.6}", load.magnitude).unwrap();
                }
                _ => {}
            }
        }
    }
    
    writeln!(output, "*END STEP").unwrap();
    writeln!(output).unwrap();
    
    // ===== STEP 2: STRUCTURAL WITH TEMPERATURE =====
    writeln!(output, "** ==================================================").unwrap();
    writeln!(output, "** STEP 2: STRUCTURAL ANALYSIS WITH THERMAL LOADS").unwrap();
    writeln!(output, "** ==================================================").unwrap();
    writeln!(output, "*STEP, NAME=Structural-Step, NLGEOM=YES").unwrap();
    writeln!(output, "*STATIC").unwrap();
    writeln!(output, "{:.6}, {:.6}, {:.6}, {:.6}",
        structural_step_time, structural_step_time * 0.1, structural_step_time * 0.01, structural_step_time).unwrap();
    
    // Mechanical boundary conditions
    if let Some(ref bcs) = model.boundary_conditions.first() {
        writeln!(output, "** Mechanical boundary conditions").unwrap();
        writeln!(output, "*BOUNDARY").unwrap();
        let mut conditions = vec![];
        if bcs.fix_x { conditions.push("1"); }
        if bcs.fix_y { conditions.push("2"); }
        if bcs.fix_z { conditions.push("3"); }
        if !conditions.is_empty() {
            let cond = conditions.join(", ");
            for node_id in &bcs.nodes {
                writeln!(output, "{}, {}, 0.0", node_id, cond).unwrap();
            }
        }
        writeln!(output).unwrap();
    }
    
    // Mechanical loads
    for load in &model.loads {
        match load.load_type {
            crate::commands::input_gen::LoadType::Force | crate::commands::input_gen::LoadType::Pressure => {
                // These would need surface/element definitions
            }
            _ => {}
        }
    }
    
    // Temperature field as pre-defined field
    writeln!(output, "** Temperature field from thermal analysis").unwrap();
    writeln!(output, "*TEMPERATURE").unwrap();
    for &(node_id, temp) in &temperature_field.node_temperatures {
        writeln!(output, "{}, {:.4}", node_id, temp).unwrap();
    }
    writeln!(output).unwrap();
    
    // Output requests
    writeln!(output, "*NODE FILE, OUTPUT=3D").unwrap();
    writeln!(output, "U, NT").unwrap(); // Displacement + Temperature
    writeln!(output, "*EL FILE").unwrap();
    writeln!(output, "S, E").unwrap(); // Stress + Strain
    writeln!(output, "*END STEP").unwrap();
    
    Ok(output)
}

/// Generate INP content for fully coupled temperature-displacement analysis
pub fn generate_fully_coupled_inp(
    model: &crate::commands::input_gen::Model,
    coupling_config: &CouplingAnalysisConfig,
    step_time: f64,
) -> Result<String, CouplingError> {
    use std::fmt::Write as FmtWrite;
    
    let mut output = String::new();
    
    // ===== HEADER =====
    writeln!(output, "** ==================================================").unwrap();
    writeln!(output, "** Thermomechanical Coupling Analysis - Fully Coupled").unwrap();
    writeln!(output, "** COUPLED TEMPERATURE-DISPLACEMENT").unwrap();
    writeln!(output, "** ==================================================").unwrap();
    writeln!(output).unwrap();
    
    // ===== NODES =====
    writeln!(output, "*HEADING").unwrap();
    writeln!(output, "Fully Coupled Thermal-Structural Analysis").unwrap();
    writeln!(output).unwrap();
    writeln!(output, "** ------------------- NODES -------------------").unwrap();
    writeln!(output, "*NODE").unwrap();
    for node in &model.nodes {
        writeln!(output, "{}, {:.6}, {:.6}, {:.6}", node.id, node.x, node.y, node.z).unwrap();
    }
    writeln!(output).unwrap();
    
    // ===== ELEMENTS =====
    writeln!(output, "** ------------------- ELEMENTS -------------------").unwrap();
    let elem_by_type: std::collections::HashMap<String, Vec<&crate::commands::input_gen::Element>> = 
        model.elements.iter().fold(std::collections::HashMap::new(), |mut acc, elem| {
            acc.entry(elem.element_type.as_str().to_string())
                .or_insert_with(Vec::new)
                .push(elem);
            acc
        });
    
    for (elem_type, elements) in elem_by_type {
        writeln!(output, "*ELEMENT, TYPE={}", elem_type).unwrap();
        for elem in &elements {
            let nodes_str = elem.nodes.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(output, "{}, {}", elem.id, nodes_str).unwrap();
        }
        writeln!(output).unwrap();
    }
    
    // ===== MATERIALS =====
    writeln!(output, "** ------------------- MATERIALS -------------------").unwrap();
    for material in &model.materials {
        writeln!(output, "** Material: {}", material.name).unwrap();
        writeln!(output, "*MATERIAL, NAME={}", material.name).unwrap();
        
        writeln!(output, "*ELASTIC").unwrap();
        writeln!(output, "{}, POISSON={:.6}", material.youngs_modulus, material.poisson_ratio).unwrap();
        
        if material.density > 0.0 {
            writeln!(output, "*DENSITY").unwrap();
            writeln!(output, "{:.6}", material.density).unwrap();
        }
        
        // Thermal expansion
        if let Some(alpha) = material.expansion_coefficient {
            writeln!(output, "*EXPANSION").unwrap();
            writeln!(output, "{:.6}", alpha).unwrap();
        }
        
        // Thermal properties
        if let Some(k) = material.thermal_conductivity {
            writeln!(output, "*CONDUCTIVITY").unwrap();
            writeln!(output, "{:.6}", k).unwrap();
        }
        if let Some(cp) = material.specific_heat {
            writeln!(output, "*SPECIFIC HEAT").unwrap();
            writeln!(output, "{:.6}", cp).unwrap();
        }
        
        writeln!(output).unwrap();
    }
    
    // ===== SECTIONS =====
    writeln!(output, "*SOLID SECTION, ELSET=EALL, MATERIAL={}", 
        model.materials.first().map(|m| m.name.as_str()).unwrap_or("Material-1")).unwrap();
    writeln!(output).unwrap();
    
    // ===== COUPLED STEP =====
    writeln!(output, "** ==================================================").unwrap();
    writeln!(output, "** COUPLED TEMPERATURE-DISPLACEMENT STEP").unwrap();
    writeln!(output, "** ==================================================").unwrap();
    writeln!(output, "*STEP, NAME=Coupled-Step, NLGEOM=YES").unwrap();
    writeln!(output, "*COUPLED TEMPERATURE-DISPLACEMENT").unwrap();
    writeln!(output, "{:.6}, {:.6}, {:.6}, {:.6}",
        step_time, step_time * 0.1, step_time * 0.01, step_time).unwrap();
    
    // Initial temperature
    writeln!(output, "** Initial temperature (stress-free temperature)").unwrap();
    writeln!(output, "*INITIAL CONDITIONS, TYPE=TEMPERATURE").unwrap();
    writeln!(output, "NALL, {:.4}", coupling_config.reference_temperature).unwrap();
    
    // Boundary conditions (mechanical)
    if let Some(ref bcs) = model.boundary_conditions.first() {
        writeln!(output, "** Mechanical boundary conditions").unwrap();
        writeln!(output, "*BOUNDARY").unwrap();
        let mut conditions = vec![];
        if bcs.fix_x { conditions.push("1"); }
        if bcs.fix_y { conditions.push("2"); }
        if bcs.fix_z { conditions.push("3"); }
        if !conditions.is_empty() {
            let cond = conditions.join(", ");
            for node_id in &bcs.nodes {
                writeln!(output, "{}, {}, 0.0", node_id, cond).unwrap();
            }
        }
        writeln!(output).unwrap();
    }
    
    // Heat flux loads
    for load in &model.loads {
        if load.load_type == crate::commands::input_gen::LoadType::HeatFlux {
            writeln!(output, "*DFLUX").unwrap();
            writeln!(output, "EALL, BFNU, {:.6}", load.magnitude).unwrap();
        }
    }
    
    // Output requests
    writeln!(output, "*NODE FILE, OUTPUT=3D").unwrap();
    writeln!(output, "U, NT").unwrap();
    writeln!(output, "*EL FILE").unwrap();
    writeln!(output, "S, E").unwrap();
    writeln!(output, "*END STEP").unwrap();
    
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_temperature_field_uniform() {
        let node_ids = vec![1, 2, 3, 4, 5];
        let temp_field = TemperatureField::uniform(&node_ids, 350.0);
        assert_eq!(temp_field.node_temperatures.len(), 5);
        assert_eq!(temp_field.node_temperatures[0].1, 350.0);
    }
}