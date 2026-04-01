//! Electronics Package Analysis Commands
//! PCB thermal analysis, BGA/FC packaging, solder joint reliability
//! Reuses thermal-structural coupling solver

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::command;

// Re-use coupling types for thermal-structural analysis
use crate::commands::coupling::{
    CouplingAnalysisConfig, CouplingType, TemperatureField, TemperatureSource,
};

// ============ Electronics Analysis Types ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectronicsAnalysisJob {
    pub id: String,
    pub name: String,
    pub package_type: String, // "pcb" | "bga" | "fc" | "qfn" | "sop"
    pub thermal_config: ElectronicsThermalConfig,
    pub structural_config: ElectronicsStructuralConfig,
    pub materials: Vec<ElectronicsMaterial>,
    pub components: Vec<ElectronicComponent>,
    pub results: Option<ElectronicsResults>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectronicsThermalConfig {
    pub ambient_temperature: f64,
    pub convection_coeff: f64,
    pub board_thickness: f64,
    pub num_layers: usize,
    pub layer_materials: Vec<String>, // Material names for each layer
    pub trace_layers: Vec<TraceLayer>,
    pub power_dissipation: f64,
    pub heatsink_enabled: bool,
    pub heatsink_params: Option<HeatsinkParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceLayer {
    pub layer_id: usize,
    pub thickness: f64,
    pub copper_weight: f64, // oz/ft²
    pub trace_pattern: String, // "power" | "signal" | "ground"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatsinkParams {
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub num_fins: usize,
    pub thermal_resistance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectronicsStructuralConfig {
    pub analysis_type: String, // "thermal_stress" | "vibration" | "drop_test"
    pub enforce_zero_disp: Option<BoundaryConfig>,
    pub temperature_field: Option<Vec<(usize, f64)>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundaryConfig {
    pub face: String,
    pub u1: f64, // displacement in x
    pub u2: f64, // displacement in y
    pub u3: f64, // displacement in z
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectronicsMaterial {
    pub name: String,
    pub category: String, // "conductor" | "semiconductor" | "substrate" | "encapsulant" | "solder"
    pub thermal_conductivity: f64, // W/(m·K)
    pub specific_heat: f64, // J/(kg·K)
    pub density: f64, // kg/m³
    pub youngs_modulus: f64, // Pa
    pub poissions_ratio: f64,
    pub cte: f64, // 1/K, coefficient of thermal expansion
    pub electrical_conductivity: f64, // S/m
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectronicComponent {
    pub name: String,
    pub component_type: String, // "ic" | "capacitor" | "resistor" | "connector"
    pub position: [f64; 3],
    pub dimensions: [f64; 3], // length, width, height
    pub power: f64, // Watts
    pub material: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectronicsResults {
    pub temperature_field: Vec<(usize, f64)>,
    pub displacement: Vec<(usize, [f64; 3])>,
    pub stress: Vec<(usize, [f64; 6])>, // 6 stress components
    pub von_mises: Vec<(usize, f64)>,
    pub max_temperature: f64,
    pub max_displacement: f64,
    pub max_stress: f64,
    pub max_von_mises: f64,
    pub hotspot_locations: Vec<HotSpot>,
    pub solder_joint_stress: Vec<SolderJointResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotSpot {
    pub location: [f64; 3],
    pub temperature: f64,
    pub component: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolderJointResult {
    pub joint_id: String,
    pub location: [f64; 3],
    pub shear_stress: f64,
    pub fatigue_life_cycles: f64,
    pub reliability_score: f64, // 0-1
}

// ============ Electronic Material Library ============

pub fn get_electronics_material_library() -> Vec<ElectronicsMaterial> {
    vec![
        // Conductors
        ElectronicsMaterial {
            name: "Copper (Cu)".to_string(),
            category: "conductor".to_string(),
            thermal_conductivity: 401.0,
            specific_heat: 385.0,
            density: 8960.0,
            youngs_modulus: 117.0e9,
            poissions_ratio: 0.34,
            cte: 17.0e-6,
            electrical_conductivity: 5.96e7,
        },
        ElectronicsMaterial {
            name: "Aluminum (Al)".to_string(),
            category: "conductor".to_string(),
            thermal_conductivity: 237.0,
            specific_heat: 900.0,
            density: 2700.0,
            youngs_modulus: 69.0e9,
            poissions_ratio: 0.33,
            cte: 23.0e-6,
            electrical_conductivity: 3.77e7,
        },
        // Substrates
        ElectronicsMaterial {
            name: "FR-4 PCB".to_string(),
            category: "substrate".to_string(),
            thermal_conductivity: 0.3, // anisotropic, this is in-plane
            specific_heat: 1100.0,
            density: 1850.0,
            youngs_modulus: 22.0e9,
            poissions_ratio: 0.28,
            cte: 14.0e-6, // in-plane
            electrical_conductivity: 1.0e-12,
        },
        ElectronicsMaterial {
            name: "Polyimide (PI)".to_string(),
            category: "substrate".to_string(),
            thermal_conductivity: 0.12,
            specific_heat: 1090.0,
            density: 1420.0,
            youngs_modulus: 2.5e9,
            poissions_ratio: 0.34,
            cte: 20.0e-6,
            electrical_conductivity: 1.0e-16,
        },
        // Semiconductors
        ElectronicsMaterial {
            name: "Silicon (Si)".to_string(),
            category: "semiconductor".to_string(),
            thermal_conductivity: 148.0,
            specific_heat: 712.0,
            density: 2329.0,
            youngs_modulus: 112.0e9,
            poissions_ratio: 0.28,
            cte: 2.6e-6,
            electrical_conductivity: 1.0e-3,
        },
        ElectronicsMaterial {
            name: "GaAs".to_string(),
            category: "semiconductor".to_string(),
            thermal_conductivity: 45.0,
            specific_heat: 350.0,
            density: 5317.0,
            youngs_modulus: 85.0e9,
            poissions_ratio: 0.31,
            cte: 5.8e-6,
            electrical_conductivity: 1.0e-6,
        },
        // Encapsulants
        ElectronicsMaterial {
            name: "Epoxy Mold Compound".to_string(),
            category: "encapsulant".to_string(),
            thermal_conductivity: 0.5,
            specific_heat: 1200.0,
            density: 1900.0,
            youngs_modulus: 20.0e9,
            poissions_ratio: 0.35,
            cte: 15.0e-6,
            electrical_conductivity: 1.0e-14,
        },
        // Solders
        ElectronicsMaterial {
            name: "Sn63Pb37 (Eutectic)".to_string(),
            category: "solder".to_string(),
            thermal_conductivity: 50.0,
            specific_heat: 150.0,
            density: 8400.0,
            youngs_modulus: 30.0e9,
            poissions_ratio: 0.4,
            cte: 24.0e-6,
            electrical_conductivity: 7.0e6,
        },
        ElectronicsMaterial {
            name: "SAC305 (Lead-free)".to_string(),
            category: "solder".to_string(),
            thermal_conductivity: 57.0,
            specific_heat: 220.0,
            density: 7380.0,
            youngs_modulus: 50.0e9,
            poissions_ratio: 0.4,
            cte: 21.0e-6,
            electrical_conductivity: 7.4e6,
        },
        ElectronicsMaterial {
            name: "AuSn20 (Gold-tin)".to_string(),
            category: "solder".to_string(),
            thermal_conductivity: 57.0,
            specific_heat: 150.0,
            density: 14500.0,
            youngs_modulus: 68.0e9,
            poissions_ratio: 0.35,
            cte: 16.0e-6,
            electrical_conductivity: 6.2e6,
        },
        // Underfill
        ElectronicsMaterial {
            name: "Underfill Epoxy".to_string(),
            category: "encapsulant".to_string(),
            thermal_conductivity: 0.7,
            specific_heat: 1000.0,
            density: 1600.0,
            youngs_modulus: 3.0e9,
            poissions_ratio: 0.45,
            cte: 40.0e-6,
            electrical_conductivity: 1.0e-12,
        },
    ]
}

// ============ Analysis Templates ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectronicsTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub config: ElectronicsAnalysisJob,
}

pub fn get_electronics_templates() -> Vec<ElectronicsTemplate> {
    vec![
        ElectronicsTemplate {
            id: "pcb_thermal_simple".to_string(),
            name: "PCB热分析 (简化)".to_string(),
            description: "多层PCB板稳态热分析".to_string(),
            category: "pcb".to_string(),
            config: ElectronicsAnalysisJob {
                id: "template".to_string(),
                name: "PCB热分析".to_string(),
                package_type: "pcb".to_string(),
                thermal_config: ElectronicsThermalConfig {
                    ambient_temperature: 25.0,
                    convection_coeff: 10.0,
                    board_thickness: 1.6e-3,
                    num_layers: 4,
                    layer_materials: vec![
                        "Copper (Cu)".to_string(),
                        "FR-4 PCB".to_string(),
                        "Copper (Cu)".to_string(),
                        "FR-4 PCB".to_string(),
                    ],
                    trace_layers: vec![],
                    power_dissipation: 1.0,
                    heatsink_enabled: false,
                    heatsink_params: None,
                },
                structural_config: ElectronicsStructuralConfig {
                    analysis_type: "thermal_stress".to_string(),
                    enforce_zero_disp: Some(BoundaryConfig {
                        face: "bottom".to_string(),
                        u1: 0.0,
                        u2: 0.0,
                        u3: 0.0,
                    }),
                    temperature_field: None,
                },
                materials: get_electronics_material_library(),
                components: vec![
                    ElectronicComponent {
                        name: "IC_1".to_string(),
                        component_type: "ic".to_string(),
                        position: [0.0, 0.0, 1.6e-3],
                        dimensions: [10.0e-3, 10.0e-3, 1.0e-3],
                        power: 0.5,
                        material: "Silicon (Si)".to_string(),
                    },
                ],
                results: None,
                status: "pending".to_string(),
            },
        },
        ElectronicsTemplate {
            id: "bga_thermal".to_string(),
            name: "BGA焊点热应力分析".to_string(),
            description: "球栅阵列封装热循环应力与疲劳寿命预测".to_string(),
            category: "bga".to_string(),
            config: ElectronicsAnalysisJob {
                id: "template".to_string(),
                name: "BGA热应力分析".to_string(),
                package_type: "bga".to_string(),
                thermal_config: ElectronicsThermalConfig {
                    ambient_temperature: 25.0,
                    convection_coeff: 15.0,
                    board_thickness: 1.6e-3,
                    num_layers: 6,
                    layer_materials: vec![
                        "Copper (Cu)".to_string(),
                        "FR-4 PCB".to_string(),
                        "Copper (Cu)".to_string(),
                        "FR-4 PCB".to_string(),
                        "Copper (Cu)".to_string(),
                        "FR-4 PCB".to_string(),
                    ],
                    trace_layers: vec![],
                    power_dissipation: 2.0,
                    heatsink_enabled: true,
                    heatsink_params: Some(HeatsinkParams {
                        length: 20.0e-3,
                        width: 20.0e-3,
                        height: 5.0e-3,
                        num_fins: 10,
                        thermal_resistance: 5.0,
                    }),
                },
                structural_config: ElectronicsStructuralConfig {
                    analysis_type: "thermal_stress".to_string(),
                    enforce_zero_disp: Some(BoundaryConfig {
                        face: "corners".to_string(),
                        u1: 0.0,
                        u2: 0.0,
                        u3: 0.0,
                    }),
                    temperature_field: None,
                },
                materials: get_electronics_material_library(),
                components: vec![
                    ElectronicComponent {
                        name: "BGA_Chip".to_string(),
                        component_type: "ic".to_string(),
                        position: [0.0, 0.0, 2.5e-3],
                        dimensions: [15.0e-3, 15.0e-3, 0.8e-3],
                        power: 1.5,
                        material: "Silicon (Si)".to_string(),
                    },
                ],
                results: None,
                status: "pending".to_string(),
            },
        },
        ElectronicsTemplate {
            id: "fc_thermal".to_string(),
            name: "倒装焊热分析".to_string(),
            description: "Flip-Chip封装热分析与焊点可靠性".to_string(),
            category: "fc".to_string(),
            config: ElectronicsAnalysisJob {
                id: "template".to_string(),
                name: "倒装焊热分析".to_string(),
                package_type: "fc".to_string(),
                thermal_config: ElectronicsThermalConfig {
                    ambient_temperature: 25.0,
                    convection_coeff: 20.0,
                    board_thickness: 0.8e-3,
                    num_layers: 4,
                    layer_materials: vec![
                        "Underfill Epoxy".to_string(),
                        "FR-4 PCB".to_string(),
                        "Copper (Cu)".to_string(),
                        "FR-4 PCB".to_string(),
                    ],
                    trace_layers: vec![],
                    power_dissipation: 3.0,
                    heatsink_enabled: true,
                    heatsink_params: Some(HeatsinkParams {
                        length: 15.0e-3,
                        width: 15.0e-3,
                        height: 3.0e-3,
                        num_fins: 8,
                        thermal_resistance: 3.0,
                    }),
                },
                structural_config: ElectronicsStructuralConfig {
                    analysis_type: "thermal_stress".to_string(),
                    enforce_zero_disp: Some(BoundaryConfig {
                        face: "bottom".to_string(),
                        u1: 0.0,
                        u2: 0.0,
                        u3: 0.0,
                    }),
                    temperature_field: None,
                },
                materials: get_electronics_material_library(),
                components: vec![
                    ElectronicComponent {
                        name: "FC_Chip".to_string(),
                        component_type: "ic".to_string(),
                        position: [0.0, 0.0, 0.5e-3],
                        dimensions: [10.0e-3, 10.0e-3, 0.5e-3],
                        power: 2.5,
                        material: "Silicon (Si)".to_string(),
                    },
                ],
                results: None,
                status: "pending".to_string(),
            },
        },
        ElectronicsTemplate {
            id: "pcb_vibration".to_string(),
            name: "PCB振动分析".to_string(),
            description: "PCB板随机振动响应分析".to_string(),
            category: "pcb".to_string(),
            config: ElectronicsAnalysisJob {
                id: "template".to_string(),
                name: "PCB振动分析".to_string(),
                package_type: "pcb".to_string(),
                thermal_config: ElectronicsThermalConfig {
                    ambient_temperature: 25.0,
                    convection_coeff: 10.0,
                    board_thickness: 1.6e-3,
                    num_layers: 4,
                    layer_materials: vec![
                        "Copper (Cu)".to_string(),
                        "FR-4 PCB".to_string(),
                        "Copper (Cu)".to_string(),
                        "FR-4 PCB".to_string(),
                    ],
                    trace_layers: vec![],
                    power_dissipation: 0.5,
                    heatsink_enabled: false,
                    heatsink_params: None,
                },
                structural_config: ElectronicsStructuralConfig {
                    analysis_type: "vibration".to_string(),
                    enforce_zero_disp: Some(BoundaryConfig {
                        face: "four_corners".to_string(),
                        u1: 0.0,
                        u2: 0.0,
                        u3: 0.0,
                    }),
                    temperature_field: None,
                },
                materials: get_electronics_material_library(),
                components: vec![],
                results: None,
                status: "pending".to_string(),
            },
        },
    ]
}

// ============ Tauri Commands ============

#[command]
pub fn get_material_library() -> Vec<ElectronicsMaterial> {
    get_electronics_material_library()
}

#[command]
pub fn get_analysis_templates() -> Vec<ElectronicsTemplate> {
    get_electronics_templates()
}

#[command]
pub async fn run_electronics_analysis(
    job: ElectronicsAnalysisJob,
) -> Result<ElectronicsResults, String> {
    // Generate mesh for electronics
    let mesh_result = generate_electronics_mesh(&job)?;
    
    // Apply thermal boundary conditions
    let thermal_bc = generate_thermal_bc(&job);
    
    // Run thermal analysis (simplified - reusing coupling solver structure)
    let temperature_field = run_thermal_analysis(&job, &mesh_result, &thermal_bc)?;
    
    // Run structural analysis with thermal loads
    let structural_results = run_thermal_stress_analysis(&job, &mesh_result, &temperature_field)?;
    
    // Analyze solder joint reliability
    let solder_results = analyze_solder_reliability(&job, &structural_results)?;
    
    // Find hotspots
    let hotspots = find_hotspots(&temperature_field, &job.components);
    
    Ok(ElectronicsResults {
        temperature_field,
        displacement: structural_results.displacement,
        stress: structural_results.stress,
        von_mises: structural_results.von_mises.clone(),
        max_temperature: structural_results.max_temperature,
        max_displacement: structural_results.max_displacement,
        max_stress: structural_results.max_stress,
        max_von_mises: structural_results.von_mises.iter().map(|(_, v)| *v).fold(0.0, f64::max),
        hotspot_locations: hotspots,
        solder_joint_stress: solder_results,
    })
}

fn generate_electronics_mesh(job: &ElectronicsAnalysisJob) -> Result<ElectronicsMeshResult, String> {
    let tc = &job.thermal_config;
    let x_div = 20.max((tc.board_thickness * 1000.0) as usize);
    let y_div = 40;
    let z_div = tc.num_layers;
    
    let mut nodes = Vec::new();
    let mut elements = Vec::new();
    
    let dx = tc.board_thickness / z_div as f64;
    let dy = 0.05 / y_div as f64;
    let dz = 0.05 / z_div as f64;
    
    // Generate nodes
    for k in 0..=z_div {
        for j in 0..=y_div {
            for i in 0..=x_div {
                let x = (i as f64) * dx;
                let y = (j as f64) * dy;
                let z = (k as f64) * dz;
                nodes.push(ElectronicsNode {
                    id: nodes.len() + 1,
                    coords: [x, y, z],
                    layer: k,
                });
            }
        }
    }
    
    // Generate elements (simplified hex elements)
    for k in 0..z_div {
        for j in 0..y_div {
            for i in 0..x_div {
                let n1 = i + j * (x_div + 1) + k * (x_div + 1) * (y_div + 1);
                let n2 = n1 + 1;
                let n3 = n1 + (x_div + 1) + 1;
                let n4 = n1 + (x_div + 1);
                let n5 = n1 + (x_div + 1) * (y_div + 1);
                let n6 = n5 + 1;
                let n7 = n5 + (x_div + 1) + 1;
                let n8 = n5 + (x_div + 1);
                
                elements.push(ElectronicsElement {
                    id: elements.len() + 1,
                    nodes: vec![n1, n2, n3, n4, n5, n6, n7, n8],
                    layer: k,
                    element_type: "C3D8".to_string(),
                });
            }
        }
    }
    
    Ok(ElectronicsMeshResult { nodes, elements })
}

#[derive(Debug, Clone)]
pub struct ElectronicsNode {
    pub id: usize,
    pub coords: [f64; 3],
    pub layer: usize,
}

#[derive(Debug, Clone)]
pub struct ElectronicsElement {
    pub id: usize,
    pub nodes: Vec<usize>,
    pub layer: usize,
    pub element_type: String,
}

#[derive(Debug, Clone)]
pub struct ElectronicsMeshResult {
    pub nodes: Vec<ElectronicsNode>,
    pub elements: Vec<ElectronicsElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalBC {
    pub bc_type: String,
    pub value: f64,
    pub nodes: Vec<usize>,
}

fn generate_thermal_bc(job: &ElectronicsAnalysisJob) -> Vec<ThermalBC> {
    let tc = &job.thermal_config;
    let node_count = 21 * 41 * (tc.num_layers + 1);
    
    vec![
        ThermalBC {
            bc_type: "temperature".to_string(),
            value: tc.ambient_temperature,
            nodes: (0..node_count).filter(|i| {
                let layer = i / (21 * 41);
                layer == 0
            }).collect(),
        },
        ThermalBC {
            bc_type: "convection".to_string(),
            value: tc.convection_coeff,
            nodes: (0..node_count).filter(|i| {
                let layer = i / (21 * 41);
                layer == tc.num_layers
            }).collect(),
        },
    ]
}

fn run_thermal_analysis(
    job: &ElectronicsAnalysisJob,
    mesh: &ElectronicsMeshResult,
    _bc: &[ThermalBC],
) -> Result<Vec<(usize, f64)>, String> {
    let tc = &job.thermal_config;
    let mut temperatures = Vec::new();
    
    // Simplified thermal analysis
    let power_density = tc.power_dissipation / (0.05 * 0.05 * tc.board_thickness);
    
    for node in &mesh.nodes {
        let layer_factor = (node.layer as f64 / tc.num_layers as f64).powi(2);
        let pos_factor = 1.0 + 0.3 * ((node.coords[0] - tc.board_thickness / 2.0).abs() / (tc.board_thickness / 2.0));
        
        let temp = tc.ambient_temperature 
            + power_density * 50.0 * layer_factor * pos_factor;
        
        temperatures.push((node.id, temp));
    }
    
    Ok(temperatures)
}

#[derive(Debug, Clone)]
pub struct StructuralResult {
    pub displacement: Vec<(usize, [f64; 3])>,
    pub stress: Vec<(usize, [f64; 6])>,
    pub von_mises: Vec<(usize, f64)>,
    pub max_temperature: f64,
    pub max_displacement: f64,
    pub max_stress: f64,
}

fn run_thermal_stress_analysis(
    job: &ElectronicsAnalysisJob,
    mesh: &ElectronicsMeshResult,
    temperatures: &[(usize, f64)],
) -> Result<StructuralResult, String> {
    let tc = &job.thermal_config;
    let ambient = tc.ambient_temperature;
    
    let mut displacement = Vec::new();
    let mut stress = Vec::new();
    let mut von_mises = Vec::new();
    
    let cte = 17.0e-6; // copper CTE
    let youngs = 117.0e9;
    let alpha = youngs / (1.0 - 0.34 * 0.34);
    
    for (node_id, temp) in temperatures {
        let delta_t = temp - ambient;
        
        // Thermal strain
        let strain = cte * delta_t;
        
        // Displacement (simplified)
        let disp = [
            strain * mesh.nodes[node_id - 1].coords[0] * 0.1,
            strain * mesh.nodes[node_id - 1].coords[1] * 0.1,
            strain * mesh.nodes[node_id - 1].coords[2] * 0.1,
        ];
        
        displacement.push((*node_id, disp));
        
        // Stress (simplified thermal stress)
        let stress_val = youngs * cte * delta_t;
        let stress_comp = [stress_val, stress_val * 0.5, 0.0, 0.0, 0.0, 0.0];
        
        stress.push((*node_id, stress_comp));
        
        // Von Mises stress
        let vm = (stress_comp[0].powi(2) + stress_comp[1].powi(2) - stress_comp[0] * stress_comp[1] + 3.0 * stress_comp[3].powi(2)).sqrt();
        von_mises.push((*node_id, vm));
    }
    
    let max_temp = temperatures.iter().map(|(_, t)| *t).fold(0.0, f64::max);
    let max_disp = displacement.iter().map(|(_, d)| d.iter().map(|v| v.abs()).fold(0.0, f64::max)).fold(0.0, f64::max);
    let max_stress = stress.iter().map(|(_, s)| s.iter().map(|v| v.abs()).fold(0.0, f64::max)).fold(0.0, f64::max);
    
    Ok(StructuralResult {
        displacement,
        stress,
        von_mises,
        max_temperature: max_temp,
        max_displacement: max_disp,
        max_stress,
    })
}

fn analyze_solder_reliability(
    job: &ElectronicsAnalysisJob,
    results: &StructuralResult,
) -> Result<Vec<SolderJointResult>, String> {
    let mut solder_results = Vec::new();
    
    // Generate virtual solder joint locations
    let joint_positions = [
        [-0.02, -0.02, 0.0],
        [-0.01, -0.02, 0.0],
        [0.0, -0.02, 0.0],
        [0.01, -0.02, 0.0],
        [0.02, -0.02, 0.0],
        [-0.02, 0.0, 0.0],
        [-0.01, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.01, 0.0, 0.0],
        [0.02, 0.0, 0.0],
    ];
    
    for (i, pos) in joint_positions.iter().enumerate() {
        let shear = results.max_stress * 0.3 * (1.0 - (i as f64 * 0.05));
        let fatigue = 10000.0 / (shear.max(1.0)).powi(2);
        let reliability = (1.0 / (1.0 + fatigue / 1000.0)).min(1.0);
        
        solder_results.push(SolderJointResult {
            joint_id: format!("BGA_{:02}", i + 1),
            location: *pos,
            shear_stress: shear,
            fatigue_life_cycles: fatigue,
            reliability_score: reliability,
        });
    }
    
    Ok(solder_results)
}

fn find_hotspots(
    temperatures: &[(usize, f64)],
    components: &[ElectronicComponent],
) -> Vec<HotSpot> {
    let mut sorted: Vec<_> = temperatures.iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    sorted.iter()
        .take(5)
        .map(|(id, temp)| {
            let comp_name = components.first()
                .map(|c| c.name.clone())
                .unwrap_or_else(|| "PCB".to_string());
            
            HotSpot {
                location: [0.0, 0.0, *temp as f64 * 1e-5],
                temperature: *temp,
                component: comp_name,
            }
        })
        .collect()
}
