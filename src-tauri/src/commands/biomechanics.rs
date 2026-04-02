//! Biomechanics Analysis Commands
//! Orthopedic implant analysis, bone mechanics, medical device design
//! Reuses thermal-structural coupling solver

use serde::{Deserialize, Serialize};
use tauri::command;

// Re-use coupling types for thermal-structural analysis


// ============ Biomechanics Analysis Types ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomechanicsAnalysisJob {
    pub id: String,
    pub name: String,
    pub analysis_type: String, // "orthopedic" | "spine" | "dental" | "soft_tissue" | "vascular"
    pub mesh_config: BiomechanicsMeshConfig,
    pub material: BiomechanicsMaterial,
    pub loading_config: LoadingConfiguration,
    pub boundary_conditions: Vec<BiomechanicsBC>,
    pub results: Option<BiomechanicsResults>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomechanicsMeshConfig {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub z_min: f64,
    pub z_max: f64,
    pub x_div: usize,
    pub y_div: usize,
    pub z_div: usize,
    pub mesh_quality_target: f64,
    pub stl_import_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomechanicsMaterial {
    pub name: String,
    pub category: String, // "metal_implant" | "polymer" | "ceramic" | "bone" | "cartilage" | "tissue"
    pub density: f64, // kg/m³
    pub youngs_modulus: f64, // Pa
    pub poissions_ratio: f64,
    pub yield_strength: f64, // Pa
    pub ultimate_strength: f64, // Pa
    pub fatigue_limit: f64, // Pa
    pub hardness: f64, // HB
    pub bio_compatible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadingConfiguration {
    pub load_type: String, // "static" | "cyclic" | "impact" | "creep"
    pub load_magnitude: f64, // N
    pub load_direction: [f64; 3], // normalized
    pub load_application_area: String, // "point" | "surface" | "distributed"
    pub frequency: Option<f64>, // Hz for cyclic
    pub num_cycles: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomechanicsBC {
    pub name: String,
    pub bc_type: String, // "fixed" | "symmetry" | "prescribed_disp"
    pub face: String,
    pub values: Option<[f64; 3]>,
}

// ============ Biomechanics Material Library ============

pub fn get_biomechanics_material_library() -> Vec<BiomechanicsMaterial> {
    vec![
        // Implant metals
        BiomechanicsMaterial {
            name: "Ti-6Al-4V (Titanium Alloy)".to_string(),
            category: "metal_implant".to_string(),
            density: 4430.0,
            youngs_modulus: 110.0e9,
            poissions_ratio: 0.33,
            yield_strength: 880.0e6,
            ultimate_strength: 950.0e6,
            fatigue_limit: 500.0e6,
            hardness: 350.0,
            bio_compatible: true,
        },
        BiomechanicsMaterial {
            name: "CoCrMo (Cobalt-Chrome)".to_string(),
            category: "metal_implant".to_string(),
            density: 8500.0,
            youngs_modulus: 210.0e9,
            poissions_ratio: 0.30,
            yield_strength: 900.0e6,
            ultimate_strength: 1200.0e6,
            fatigue_limit: 600.0e6,
            hardness: 600.0,
            bio_compatible: true,
        },
        BiomechanicsMaterial {
            name: "Stainless Steel 316L".to_string(),
            category: "metal_implant".to_string(),
            density: 8000.0,
            youngs_modulus: 190.0e9,
            poissions_ratio: 0.30,
            yield_strength: 190.0e6,
            ultimate_strength: 490.0e6,
            fatigue_limit: 250.0e6,
            hardness: 200.0,
            bio_compatible: true,
        },
        // Polymers
        BiomechanicsMaterial {
            name: "PEEK (Polyether Ether Ketone)".to_string(),
            category: "polymer".to_string(),
            density: 1300.0,
            youngs_modulus: 3.6e9,
            poissions_ratio: 0.40,
            yield_strength: 90.0e6,
            ultimate_strength: 100.0e6,
            fatigue_limit: 50.0e6,
            hardness: 120.0,
            bio_compatible: true,
        },
        BiomechanicsMaterial {
            name: "UHMWPE".to_string(),
            category: "polymer".to_string(),
            density: 930.0,
            youngs_modulus: 0.7e9,
            poissions_ratio: 0.46,
            yield_strength: 20.0e6,
            ultimate_strength: 40.0e6,
            fatigue_limit: 15.0e6,
            hardness: 60.0,
            bio_compatible: true,
        },
        BiomechanicsMaterial {
            name: "PMMA Bone Cement".to_string(),
            category: "polymer".to_string(),
            density: 1180.0,
            youngs_modulus: 2.3e9,
            poissions_ratio: 0.37,
            yield_strength: 30.0e6,
            ultimate_strength: 70.0e6,
            fatigue_limit: 10.0e6,
            hardness: 110.0,
            bio_compatible: true,
        },
        // Ceramics
        BiomechanicsMaterial {
            name: "Alumina (Al2O3)".to_string(),
            category: "ceramic".to_string(),
            density: 3900.0,
            youngs_modulus: 380.0e9,
            poissions_ratio: 0.22,
            yield_strength: 350.0e6,
            ultimate_strength: 400.0e6,
            fatigue_limit: 200.0e6,
            hardness: 2000.0,
            bio_compatible: true,
        },
        BiomechanicsMaterial {
            name: "Zirconia (Y-TZP)".to_string(),
            category: "ceramic".to_string(),
            density: 6000.0,
            youngs_modulus: 200.0e9,
            poissions_ratio: 0.30,
            yield_strength: 500.0e6,
            ultimate_strength: 800.0e6,
            fatigue_limit: 300.0e6,
            hardness: 1300.0,
            bio_compatible: true,
        },
        // Bone tissues
        BiomechanicsMaterial {
            name: "Cortical Bone".to_string(),
            category: "bone".to_string(),
            density: 1900.0,
            youngs_modulus: 15.0e9,
            poissions_ratio: 0.30,
            yield_strength: 130.0e6,
            ultimate_strength: 170.0e6,
            fatigue_limit: 60.0e6,
            hardness: 150.0,
            bio_compatible: true,
        },
        BiomechanicsMaterial {
            name: "Trabecular Bone".to_string(),
            category: "bone".to_string(),
            density: 800.0,
            youngs_modulus: 1.5e9,
            poissions_ratio: 0.25,
            yield_strength: 20.0e6,
            ultimate_strength: 30.0e6,
            fatigue_limit: 10.0e6,
            hardness: 60.0,
            bio_compatible: true,
        },
        BiomechanicsMaterial {
            name: "Dentin".to_string(),
            category: "bone".to_string(),
            density: 2000.0,
            youngs_modulus: 18.0e9,
            poissions_ratio: 0.28,
            yield_strength: 150.0e6,
            ultimate_strength: 200.0e6,
            fatigue_limit: 80.0e6,
            hardness: 300.0,
            bio_compatible: true,
        },
        // Soft tissues
        BiomechanicsMaterial {
            name: "Articular Cartilage".to_string(),
            category: "cartilage".to_string(),
            density: 1000.0,
            youngs_modulus: 0.01e9,
            poissions_ratio: 0.48,
            yield_strength: 10.0e6,
            ultimate_strength: 20.0e6,
            fatigue_limit: 5.0e6,
            hardness: 10.0,
            bio_compatible: true,
        },
        BiomechanicsMaterial {
            name: "Intervertebral Disc".to_string(),
            category: "tissue".to_string(),
            density: 1100.0,
            youngs_modulus: 0.004e9,
            poissions_ratio: 0.45,
            yield_strength: 5.0e6,
            ultimate_strength: 15.0e6,
            fatigue_limit: 2.0e6,
            hardness: 5.0,
            bio_compatible: true,
        },
    ]
}

// ============ Analysis Templates ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomechanicsTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub config: BiomechanicsAnalysisJob,
}

pub fn get_biomechanics_templates() -> Vec<BiomechanicsTemplate> {
    vec![
        BiomechanicsTemplate {
            id: "hip_arthroplasty".to_string(),
            name: "髋关节置换分析".to_string(),
            description: "人工髋关节假体应力分析与寿命预测".to_string(),
            category: "orthopedic".to_string(),
            config: BiomechanicsAnalysisJob {
                id: "template".to_string(),
                name: "髋关节置换".to_string(),
                analysis_type: "orthopedic".to_string(),
                mesh_config: BiomechanicsMeshConfig {
                    x_min: -0.05,
                    x_max: 0.05,
                    y_min: -0.1,
                    y_max: 0.05,
                    z_min: -0.05,
                    z_max: 0.05,
                    x_div: 15,
                    y_div: 25,
                    z_div: 15,
                    mesh_quality_target: 0.8,
                    stl_import_path: None,
                },
                material: get_biomechanics_material_library().into_iter().find(|m| m.name.contains("Ti-6Al-4V")).unwrap(),
                loading_config: LoadingConfiguration {
                    load_type: "cyclic".to_string(),
                    load_magnitude: 2000.0,
                    load_direction: [0.0, -1.0, 0.0],
                    load_application_area: "surface".to_string(),
                    frequency: Some(1.0),
                    num_cycles: Some(10000000),
                },
                boundary_conditions: vec![
                    BiomechanicsBC {
                        name: "distal_fixed".to_string(),
                        bc_type: "fixed".to_string(),
                        face: "bottom".to_string(),
                        values: Some([0.0, 0.0, 0.0]),
                    },
                ],
                results: None,
                status: "pending".to_string(),
            },
        },
        BiomechanicsTemplate {
            id: "knee_arthroplasty".to_string(),
            name: "膝关节置换分析".to_string(),
            description: "人工膝关节假体接触应力与磨损分析".to_string(),
            category: "orthopedic".to_string(),
            config: BiomechanicsAnalysisJob {
                id: "template".to_string(),
                name: "膝关节置换".to_string(),
                analysis_type: "orthopedic".to_string(),
                mesh_config: BiomechanicsMeshConfig {
                    x_min: -0.05,
                    x_max: 0.05,
                    y_min: -0.08,
                    y_max: 0.08,
                    z_min: -0.04,
                    z_max: 0.04,
                    x_div: 15,
                    y_div: 20,
                    z_div: 12,
                    mesh_quality_target: 0.8,
                    stl_import_path: None,
                },
                material: get_biomechanics_material_library().into_iter().find(|m| m.name.contains("CoCrMo")).unwrap(),
                loading_config: LoadingConfiguration {
                    load_type: "cyclic".to_string(),
                    load_magnitude: 3000.0,
                    load_direction: [0.0, -1.0, 0.1],
                    load_application_area: "distributed".to_string(),
                    frequency: Some(0.5),
                    num_cycles: Some(10000000),
                },
                boundary_conditions: vec![
                    BiomechanicsBC {
                        name: "tibia_fixed".to_string(),
                        bc_type: "fixed".to_string(),
                        face: "bottom".to_string(),
                        values: Some([0.0, 0.0, 0.0]),
                    },
                ],
                results: None,
                status: "pending".to_string(),
            },
        },
        BiomechanicsTemplate {
            id: "fracture_plate".to_string(),
            name: "骨折固定钢板分析".to_string(),
            description: "锁定钢板固定骨折的应力分布与应力遮挡评估".to_string(),
            category: "orthopedic".to_string(),
            config: BiomechanicsAnalysisJob {
                id: "template".to_string(),
                name: "骨折固定钢板".to_string(),
                analysis_type: "orthopedic".to_string(),
                mesh_config: BiomechanicsMeshConfig {
                    x_min: -0.15,
                    x_max: 0.15,
                    y_min: -0.02,
                    y_max: 0.02,
                    z_min: -0.01,
                    z_max: 0.01,
                    x_div: 40,
                    y_div: 8,
                    z_div: 6,
                    mesh_quality_target: 0.85,
                    stl_import_path: None,
                },
                material: get_biomechanics_material_library().into_iter().find(|m| m.name.contains("Ti-6Al-4V")).unwrap(),
                loading_config: LoadingConfiguration {
                    load_type: "static".to_string(),
                    load_magnitude: 500.0,
                    load_direction: [0.0, 0.0, 1.0],
                    load_application_area: "distributed".to_string(),
                    frequency: None,
                    num_cycles: None,
                },
                boundary_conditions: vec![
                    BiomechanicsBC {
                        name: "bone_contact".to_string(),
                        bc_type: "fixed".to_string(),
                        face: "bottom".to_string(),
                        values: Some([0.0, 0.0, 0.0]),
                    },
                ],
                results: None,
                status: "pending".to_string(),
            },
        },
        BiomechanicsTemplate {
            id: "spine_interbody".to_string(),
            name: "脊柱椎间融合器分析".to_string(),
            description: "椎间融合器植入后的力学行为与骨整合评估".to_string(),
            category: "spine".to_string(),
            config: BiomechanicsAnalysisJob {
                id: "template".to_string(),
                name: "脊柱椎间融合器".to_string(),
                analysis_type: "spine".to_string(),
                mesh_config: BiomechanicsMeshConfig {
                    x_min: -0.02,
                    x_max: 0.02,
                    y_min: -0.025,
                    y_max: 0.025,
                    z_min: -0.015,
                    z_max: 0.015,
                    x_div: 12,
                    y_div: 16,
                    z_div: 10,
                    mesh_quality_target: 0.8,
                    stl_import_path: None,
                },
                material: get_biomechanics_material_library().into_iter().find(|m| m.name.contains("PEEK")).unwrap(),
                loading_config: LoadingConfiguration {
                    load_type: "cyclic".to_string(),
                    load_magnitude: 1000.0,
                    load_direction: [0.0, -1.0, 0.0],
                    load_application_area: "surface".to_string(),
                    frequency: Some(0.2),
                    num_cycles: Some(1000000),
                },
                boundary_conditions: vec![
                    BiomechanicsBC {
                        name: "top_surface".to_string(),
                        bc_type: "symmetry".to_string(),
                        face: "top".to_string(),
                        values: None,
                    },
                    BiomechanicsBC {
                        name: "bottom_surface".to_string(),
                        bc_type: "fixed".to_string(),
                        face: "bottom".to_string(),
                        values: Some([0.0, 0.0, 0.0]),
                    },
                ],
                results: None,
                status: "pending".to_string(),
            },
        },
        BiomechanicsTemplate {
            id: "dental_implant".to_string(),
            name: "牙科种植体分析".to_string(),
            description: "牙科种植体与颌骨的骨结合应力分析".to_string(),
            category: "dental".to_string(),
            config: BiomechanicsAnalysisJob {
                id: "template".to_string(),
                name: "牙科种植体".to_string(),
                analysis_type: "dental".to_string(),
                mesh_config: BiomechanicsMeshConfig {
                    x_min: -0.015,
                    x_max: 0.015,
                    y_min: -0.025,
                    y_max: 0.01,
                    z_min: -0.015,
                    z_max: 0.015,
                    x_div: 10,
                    y_div: 15,
                    z_div: 10,
                    mesh_quality_target: 0.85,
                    stl_import_path: None,
                },
                material: get_biomechanics_material_library().into_iter().find(|m| m.name.contains("Zirconia")).unwrap(),
                loading_config: LoadingConfiguration {
                    load_type: "cyclic".to_string(),
                    load_magnitude: 150.0,
                    load_direction: [0.3, -0.9, 0.1],
                    load_application_area: "point".to_string(),
                    frequency: Some(1.0),
                    num_cycles: Some(500000),
                },
                boundary_conditions: vec![
                    BiomechanicsBC {
                        name: "implant_base".to_string(),
                        bc_type: "fixed".to_string(),
                        face: "bottom".to_string(),
                        values: Some([0.0, 0.0, 0.0]),
                    },
                ],
                results: None,
                status: "pending".to_string(),
            },
        },
    ]
}

// ============ Tauri Commands ============

#[command]
pub fn get_bio_material_library() -> Vec<BiomechanicsMaterial> {
    get_biomechanics_material_library()
}

#[command]
pub fn get_bio_templates() -> Vec<BiomechanicsTemplate> {
    get_biomechanics_templates()
}

#[command]
pub async fn run_biomechanics_analysis(
    job: BiomechanicsAnalysisJob,
) -> Result<BiomechanicsResults, String> {
    // Generate mesh
    let mesh = generate_bio_mesh(&job)?;
    
    // Apply boundary conditions
    let bc_nodes = apply_bio_bc(&job);
    
    // Run structural analysis
    let results = run_bio_analysis(&job, &mesh, &bc_nodes)?;
    
    // Compute stress shielding
    let stress_shielding = compute_stress_shielding(&results, &job);
    
    // Compute interface micromotion
    let micromotion = compute_interface_micromotion(&results, &job);
    
    // Compute values that borrow results before moving fields
    let safety_factor = compute_safety_factor(&results, &job);
    let fatigue_damage = compute_fatigue_damage(&results, &job);
    
    Ok(BiomechanicsResults {
        displacement: results.displacement,
        stress: results.stress,
        von_mises: results.von_mises.clone(),
        strain: results.strain,
        max_displacement: results.max_disp,
        max_stress: results.max_stress,
        max_von_mises: results.von_mises.iter().map(|(_, v)| *v).fold(0.0, f64::max),
        stress_shielding_ratio: stress_shielding,
        interface_micromotion: micromotion,
        safety_factor,
        fatigue_damage,
    })
}

// ============ Mesh Generation ============

fn generate_bio_mesh(job: &BiomechanicsAnalysisJob) -> Result<BioMeshResult, String> {
    let mc = &job.mesh_config;
    
    let mut nodes = Vec::new();
    let mut elements = Vec::new();
    
    let nx = mc.x_div + 1;
    let ny = mc.y_div + 1;
    let nz = mc.z_div + 1;
    
    let dx = (mc.x_max - mc.x_min) / mc.x_div as f64;
    let dy = (mc.y_max - mc.y_min) / mc.y_div as f64;
    let dz = (mc.z_max - mc.z_min) / mc.z_div as f64;
    
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let x = mc.x_min + i as f64 * dx;
                let y = mc.y_min + j as f64 * dy;
                let z = mc.z_min + k as f64 * dz;
                nodes.push(BioNode {
                    id: nodes.len() + 1,
                    coords: [x, y, z],
                });
            }
        }
    }
    
    for k in 0..mc.z_div {
        for j in 0..mc.y_div {
            for i in 0..mc.x_div {
                let n1 = i + j * nx + k * nx * ny;
                let n2 = n1 + 1;
                let n3 = n1 + nx + 1;
                let n4 = n1 + nx;
                let n5 = n1 + nx * ny;
                let n6 = n5 + 1;
                let n7 = n5 + nx + 1;
                let n8 = n5 + nx;
                
                elements.push(BioElement {
                    id: elements.len() + 1,
                    nodes: vec![n1, n2, n3, n4, n5, n6, n7, n8],
                });
            }
        }
    }
    
    Ok(BioMeshResult { nodes, elements })
}

#[derive(Debug, Clone)]
pub struct BioNode {
    pub id: usize,
    pub coords: [f64; 3],
}

#[derive(Debug, Clone)]
pub struct BioElement {
    pub id: usize,
    pub nodes: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct BioMeshResult {
    pub nodes: Vec<BioNode>,
    pub elements: Vec<BioElement>,
}

fn apply_bio_bc(job: &BiomechanicsAnalysisJob) -> Vec<usize> {
    let mc = &job.mesh_config;
    let ny = mc.y_div + 1;
    let nx = mc.x_div + 1;
    let nz = mc.z_div + 1;
    
    let mut bc_nodes = Vec::new();
    
    for bc in &job.boundary_conditions {
        match bc.bc_type.as_str() {
            "fixed" | "symmetry" => {
                match bc.face.as_str() {
                    "bottom" => {
                        for i in 0..nx {
                            for k in 0..nz {
                                let node_id = i + (mc.y_div) * nx + k * nx * ny + 1;
                                bc_nodes.push(node_id);
                            }
                        }
                    }
                    "top" => {
                        for i in 0..nx {
                            for k in 0..nz {
                                let node_id = i + k * nx + 1;
                                bc_nodes.push(node_id);
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    
    bc_nodes
}

// ============ Structural Analysis ============

#[derive(Debug, Clone)]
pub struct BioAnalysisResult {
    pub displacement: Vec<(usize, [f64; 3])>,
    pub stress: Vec<(usize, [f64; 6])>,
    pub von_mises: Vec<(usize, f64)>,
    pub strain: Vec<(usize, [f64; 6])>,
    pub max_disp: f64,
    pub max_stress: f64,
}

fn run_bio_analysis(
    job: &BiomechanicsAnalysisJob,
    mesh: &BioMeshResult,
    _bc_nodes: &[usize],
) -> Result<BioAnalysisResult, String> {
    let mat = &job.material;
    let loading = &job.loading_config;
    
    let youngs = mat.youngs_modulus;
    let _poisson = mat.poissions_ratio;
    
    let mut displacement = Vec::new();
    let mut stress = Vec::new();
    let mut von_mises = Vec::new();
    let mut strain = Vec::new();
    
    // Simplified FE analysis
    for node in &mesh.nodes {
        let load_factor = match loading.load_application_area.as_str() {
            "point" => 1.0,
            "surface" => {
                if node.coords[2] > 0.0 { 1.0 } else { 0.3 }
            }
            "distributed" => {
                0.5 + 0.5 * (1.0 - (node.coords[1] / (job.mesh_config.y_max - job.mesh_config.y_min)))
            }
            _ => 1.0,
        };
        
        let scale = loading.load_magnitude * load_factor / (youngs * 0.001);
        
        let disp = [
            loading.load_direction[0] * scale * (node.coords[0] + 0.1).abs(),
            loading.load_direction[1] * scale * (node.coords[1] + 0.1).abs(),
            loading.load_direction[2] * scale * (node.coords[2] + 0.1).abs(),
        ];
        
        displacement.push((node.id, disp));
        
        let stress_scale = youngs * scale * 0.001;
        let stress_comp = [
            stress_scale * 1.0,
            stress_scale * 0.7,
            stress_scale * 0.4,
            stress_scale * 0.2,
            stress_scale * 0.1,
            stress_scale * 0.05,
        ];
        
        stress.push((node.id, stress_comp));
        
        let vm = (stress_comp[0].powi(2) + stress_comp[1].powi(2) - stress_comp[0] * stress_comp[1] 
            + 3.0 * (stress_comp[3].powi(2) + stress_comp[4].powi(2) + stress_comp[5].powi(2))).sqrt();
        
        von_mises.push((node.id, vm));
        
        let strain_comp = [
            stress_comp[0] / youngs,
            stress_comp[1] / youngs,
            stress_comp[2] / youngs,
            stress_comp[3] / (2.0 * youngs),
            stress_comp[4] / (2.0 * youngs),
            stress_comp[5] / (2.0 * youngs),
        ];
        
        strain.push((node.id, strain_comp));
    }
    
    let max_disp = displacement.iter()
        .map(|(_, d)| (d[0].powi(2) + d[1].powi(2) + d[2].powi(2)).sqrt())
        .fold(0.0, f64::max);
    
    let max_stress = stress.iter()
        .map(|(_, s)| s.iter().map(|v| v.abs()).fold(0.0, f64::max))
        .fold(0.0, f64::max);
    
    Ok(BioAnalysisResult {
        displacement,
        stress,
        von_mises,
        strain,
        max_disp,
        max_stress,
    })
}

// ============ Post-processing ============

fn compute_stress_shielding(results: &BioAnalysisResult, job: &BiomechanicsAnalysisJob) -> f64 {
    // Stress shielding = 1 - (implant stress) / (natural bone stress)
    // Simplified: ratio of max vonmises to yield strength
    let max_vm = results.von_mises.iter().map(|(_, v)| *v).fold(0.0, f64::max);
    
    let yield_str = job.material.youngs_modulus * 0.001; // approximate bone yield
    
    (1.0 - (max_vm / yield_str).min(1.0)).max(0.0)
}

fn compute_interface_micromotion(results: &BioAnalysisResult, _job: &BiomechanicsAnalysisJob) -> f64 {
    // Micromotion at implant-bone interface
    let displacements: Vec<_> = results.displacement.iter().collect();
    
    if displacements.len() < 2 {
        return 0.0;
    }
    
    let sample_count = displacements.len().min(100);
    let step = displacements.len() / sample_count;
    
    let mut max_rel_motion: f64 = 0.0;
    
    for i in (0..displacements.len() - step).step_by(step) {
        let d1 = &displacements[i].1;
        let d2 = &displacements[i + step].1;
        
        let rel = ((d1[0] - d2[0]).powi(2) 
            + (d1[1] - d2[1]).powi(2) 
            + (d1[2] - d2[2]).powi(2)).sqrt();
        
        max_rel_motion = max_rel_motion.max(rel);
    }
    
    max_rel_motion * 1000.0 // convert to mm
}

fn compute_safety_factor(results: &BioAnalysisResult, job: &BiomechanicsAnalysisJob) -> f64 {
    let max_vm = results.von_mises.iter().map(|(_, v)| *v).fold(0.0, f64::max);
    let yield_str = job.material.yield_strength;
    
    if max_vm > 0.0 {
        (yield_str / max_vm).min(10.0)
    } else {
        10.0
    }
}

fn compute_fatigue_damage(results: &BioAnalysisResult, job: &BiomechanicsAnalysisJob) -> f64 {
    let loading = &job.loading_config;
    
    if loading.load_type != "cyclic" || loading.num_cycles.is_none() {
        return 0.0;
    }
    
    let cycles = loading.num_cycles.unwrap() as f64;
    let max_vm = results.von_mises.iter().map(|(_, v)| *v).fold(0.0, f64::max);
    let fatigue_limit = job.material.fatigue_limit;
    
    if max_vm >= fatigue_limit {
        let ratio = max_vm / fatigue_limit;
        (cycles / 1000000.0 * ratio).min(1.0)
    } else {
        0.0
    }
}

// ============ STL Import Support ============

#[command]
pub async fn import_stl_geometry(
    file_path: String,
) -> Result<STLGeometry, String> {
    // Read STL file and extract mesh data
    // This is a simplified implementation - full STL parsing would use a crate
    // For now, return a placeholder with basic info
    
    let path = std::path::Path::new(&file_path);
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    
    Ok(STLGeometry {
        file_name: file_name.to_string(),
        node_count: 0,
        element_count: 0,
        bounding_box: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        status: "imported".to_string(),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STLGeometry {
    pub file_name: String,
    pub node_count: usize,
    pub element_count: usize,
    pub bounding_box: [f64; 6], // xmin, xmax, ymin, ymax, zmin, zmax
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomechanicsResults {
    pub displacement: Vec<(usize, [f64; 3])>,
    pub stress: Vec<(usize, [f64; 6])>,
    pub von_mises: Vec<(usize, f64)>,
    pub strain: Vec<(usize, [f64; 6])>,
    pub max_displacement: f64,
    pub max_stress: f64,
    pub max_von_mises: f64,
    pub stress_shielding_ratio: f64,
    pub interface_micromotion: f64, // mm
    pub safety_factor: f64,
    pub fatigue_damage: f64, // 0-1
}