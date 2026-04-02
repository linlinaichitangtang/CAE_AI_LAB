//! Thermal-Structural Coupling Analysis Commands
//! Sequential coupling: thermal analysis first, then structural with temperature loads
//! Exposes thermal coupling functionality to the Tauri frontend

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::command;

// Re-export coupling types


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalCouplingJob {
    pub id: String,
    pub name: String,
    pub coupling_type: String, // "sequential" | "fully_coupled"
    pub mesh_config: ThermalCouplingMeshConfig,
    pub thermal_config: ThermalAnalysisConfig,
    pub structural_config: StructuralAnalysisConfig,
    pub material: ThermalCouplingMaterial,
    pub temperature_field: Option<Vec<(usize, f64)>>,
    pub thermal_result_path: Option<String>,
    pub structural_result_path: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalCouplingMeshConfig {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub z_min: f64,
    pub z_max: f64,
    pub x_div: usize,
    pub y_div: usize,
    pub z_div: usize,
    pub element_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalAnalysisConfig {
    pub analysis_type: String, // "steady_state" | "transient"
    pub initial_temperature: f64,
    pub time_period: f64,
    pub time_increment: f64,
    pub max_iterations: usize,
    pub tolerance: f64,
    pub boundary_conditions: Vec<ThermalBcConfig>,
    pub heat_sources: Vec<HeatSourceConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalBcConfig {
    pub name: String,
    pub bc_type: String, // "fixed_temperature" | "heat_flux" | "convection" | "radiation" | "insulation"
    pub nodes: Vec<usize>,
    pub surface_name: Option<String>,
    pub temperature: Option<f64>,
    pub ambient_temperature: Option<f64>,
    pub film_coefficient: Option<f64>,
    pub heat_flux: Option<f64>,
    pub emissivity: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatSourceConfig {
    pub name: String,
    pub source_type: String, // "point" | "surface" | "volume"
    pub magnitude: f64,
    pub node_ids: Option<Vec<usize>>,
    pub surface_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralAnalysisConfig {
    pub reference_temperature: f64,
    pub stress_free_temperature: f64,
    pub step_time: f64,
    pub boundary_conditions: Vec<StructuralBcConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralBcConfig {
    pub name: String,
    pub bc_type: String, // "fixed" | "symmetry"
    pub nodes: Vec<usize>,
    pub fix_x: bool,
    pub fix_y: bool,
    pub fix_z: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalCouplingMaterial {
    pub name: String,
    pub density: f64,
    pub youngs_modulus: f64,
    pub poisson_ratio: f64,
    pub thermal_conductivity: f64,
    pub expansion_coefficient: f64,
    pub specific_heat: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalCouplingResult {
    pub job_id: String,
    pub success: bool,
    pub thermal_results: Option<ThermalResultData>,
    pub structural_results: Option<StructuralResultData>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalResultData {
    pub node_temperatures: Vec<(usize, f64)>,
    pub max_temperature: f64,
    pub min_temperature: f64,
    pub avg_temperature: f64,
    pub result_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralResultData {
    pub max_displacement: f64,
    pub max_stress: f64,
    pub max_von_mises: f64,
    pub result_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub mesh_config: ThermalCouplingMeshConfig,
    pub thermal_config: ThermalAnalysisConfig,
    pub structural_config: StructuralAnalysisConfig,
    pub material: ThermalCouplingMaterial,
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Generate thermal-structural coupling INP files
#[command]
pub fn generate_thermal_coupling_inp(
    job: ThermalCouplingJob,
    working_dir: String,
) -> Result<String, String> {
    use std::fmt::Write as FmtWrite;

    let mut thermal_output = String::new();
    let mut structural_output = String::new();

    // ===== Build node/element list from mesh config =====
    let (nodes, elements) = generate_mesh_nodes_elements(&job.mesh_config);

    // ===== MATERIALS =====
    let mat = &job.material;
    let mat_inp = format!(
        r#"** MATERIAL: {}
*MATERIAL, NAME={}
*ELASTIC
{}, POISSON={:.6}
*DENSITY
{:.6}
*EXPANSION
{:.6}
*CONDUCTIVITY
{:.6}
*SPECIFIC HEAT
{:.6}
"#,
        mat.name, mat.name, mat.youngs_modulus, mat.poisson_ratio,
        mat.density, mat.expansion_coefficient, mat.thermal_conductivity, mat.specific_heat
    );

    // ===== NODES INP =====
    let mut nodes_inp = String::new();
    writeln!(nodes_inp, "*HEADING").unwrap();
    writeln!(nodes_inp, "Thermal-Structural Coupling Analysis").unwrap();
    writeln!(nodes_inp).unwrap();
    writeln!(nodes_inp, "*NODE").unwrap();
    for node in &nodes {
        writeln!(nodes_inp, "{}, {:.6}, {:.6}, {:.6}", node.0, node.1, node.2, node.3).unwrap();
    }

    // ===== ELEMENTS INP =====
    let mut elements_inp = String::new();
    writeln!(elements_inp, "*ELEMENT, TYPE={}", job.mesh_config.element_type).unwrap();
    for elem in &elements {
        writeln!(elements_inp, "{}, {}", elem.0, elem.1.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ")).unwrap();
    }
    writeln!(elements_inp).unwrap();
    writeln!(elements_inp, "*SOLID SECTION, ELSET=EALL, MATERIAL={}", mat.name).unwrap();
    writeln!(elements_inp).unwrap();

    // ===== THERMAL ANALYSIS INP =====
    writeln!(thermal_output, "** ==================================================").unwrap();
    writeln!(thermal_output, "** THERMAL-STRUCTURAL COUPLING - THERMAL STEP").unwrap();
    writeln!(thermal_output, "** ==================================================").unwrap();
    thermal_output.push_str(&nodes_inp);
    thermal_output.push_str(&elements_inp);
    thermal_output.push_str(&mat_inp);

    writeln!(thermal_output, "** ------------------- THERMAL BCs -------------------").unwrap();

    for bc in &job.thermal_config.boundary_conditions {
        match bc.bc_type.as_str() {
            "fixed_temperature" => {
                if let Some(temp) = bc.temperature {
                    writeln!(thermal_output, "*BOUNDARY").unwrap();
                    for node_id in &bc.nodes {
                        writeln!(thermal_output, "{}, 11, 11, {:.4}", node_id, temp).unwrap();
                    }
                    writeln!(thermal_output).unwrap();
                }
            }
            "heat_flux" => {
                if let Some(flux) = bc.heat_flux {
                    writeln!(thermal_output, "*DFLUX").unwrap();
                    writeln!(thermal_output, "EALL, BFNU, {:.6}", flux).unwrap();
                    writeln!(thermal_output).unwrap();
                }
            }
            "convection" => {
                if let (Some(film), Some(ambient)) = (bc.film_coefficient, bc.ambient_temperature) {
                    let surf = bc.surface_name.as_deref().unwrap_or("EFACEOUT");
                    writeln!(thermal_output, "*FILM").unwrap();
                    writeln!(thermal_output, "{}, F, {:.6}, {:.4}", surf, film, ambient).unwrap();
                    writeln!(thermal_output).unwrap();
                }
            }
            "radiation" => {
                if let (Some(emis), Some(rad_temp)) = (bc.emissivity, bc.ambient_temperature) {
                    let surf = bc.surface_name.as_deref().unwrap_or("EFACEOUT");
                    writeln!(thermal_output, "*FILM").unwrap();
                    writeln!(thermal_output, "{}, R, {:.6}, {:.4}", surf, emis, rad_temp).unwrap();
                    writeln!(thermal_output).unwrap();
                }
            }
            _ => {}
        }
    }

    writeln!(thermal_output, "** ------------------- HEAT SOURCES -------------------").unwrap();
    for hs in &job.thermal_config.heat_sources {
        match hs.source_type.as_str() {
            "point" => {
                if let Some(ref node_ids) = hs.node_ids {
                    writeln!(thermal_output, "*DFLUX").unwrap();
                    for nid in node_ids {
                        writeln!(thermal_output, "{}, BF, {:.6}", nid, hs.magnitude).unwrap();
                    }
                    writeln!(thermal_output).unwrap();
                }
            }
            "surface" => {
                let surf = hs.surface_name.as_deref().unwrap_or("EALL");
                writeln!(thermal_output, "*DFLUX").unwrap();
                writeln!(thermal_output, "{}, S, {:.6}", surf, hs.magnitude).unwrap();
                writeln!(thermal_output).unwrap();
            }
            "volume" => {
                writeln!(thermal_output, "*DFLUX").unwrap();
                writeln!(thermal_output, "EALL, BFV, {:.6}", hs.magnitude).unwrap();
                writeln!(thermal_output).unwrap();
            }
            _ => {}
        }
    }

    let thermal_step_time = if job.thermal_config.analysis_type == "transient" {
        job.thermal_config.time_period
    } else {
        1.0
    };

    writeln!(thermal_output, "** ------------------- THERMAL STEP -------------------").unwrap();
    writeln!(thermal_output, "*STEP, NAME=Thermal-Step").unwrap();
    if job.thermal_config.analysis_type == "transient" {
        writeln!(thermal_output, "*HEAT TRANSFER, DELTMX=100.0, INC=1000").unwrap();
        let dt = job.thermal_config.time_increment;
        writeln!(thermal_output, "{:.6}, {:.6}, {:.6}, {:.6}",
            thermal_step_time, dt, dt * 0.1, thermal_step_time).unwrap();
    } else {
        writeln!(thermal_output, "*HEAT TRANSFER").unwrap();
        writeln!(thermal_output, "1.0, 1.0, 1.0, 1.0").unwrap();
    }
    writeln!(thermal_output, "*NODE FILE").unwrap();
    writeln!(thermal_output, "NT").unwrap(); // Node Temperature
    writeln!(thermal_output, "*END STEP").unwrap();

    // ===== STRUCTURAL ANALYSIS INP =====
    writeln!(structural_output, "** ==================================================").unwrap();
    writeln!(structural_output, "** THERMAL-STRUCTURAL COUPLING - STRUCTURAL STEP").unwrap();
    writeln!(structural_output, "** ==================================================").unwrap();
    structural_output.push_str(&nodes_inp);
    structural_output.push_str(&elements_inp);
    structural_output.push_str(&mat_inp);

    writeln!(structural_output, "** ------------------- STRUCTURAL BCs -------------------").unwrap();
    writeln!(structural_output, "*BOUNDARY").unwrap();
    for sbc in &job.structural_config.boundary_conditions {
        let mut dofs = vec![];
        if sbc.fix_x { dofs.push("1"); }
        if sbc.fix_y { dofs.push("2"); }
        if sbc.fix_z { dofs.push("3"); }
        if !dofs.is_empty() {
            for node_id in &sbc.nodes {
                writeln!(structural_output, "{}, {}, 0.0", node_id, dofs.join(",")).unwrap();
            }
        }
    }
    writeln!(structural_output).unwrap();

    // Temperature field (from thermal results)
    writeln!(structural_output, "** ------------------- TEMPERATURE LOAD -------------------").unwrap();
    if let Some(ref temps) = job.temperature_field {
        writeln!(structural_output, "*TEMPERATURE").unwrap();
        for &(node_id, temp) in temps {
            writeln!(structural_output, "{}, {:.4}", node_id, temp).unwrap();
        }
        writeln!(structural_output).unwrap();
    }

    writeln!(structural_output, "** ------------------- STRUCTURAL STEP -------------------").unwrap();
    writeln!(structural_output, "*STEP, NAME=Structural-Step, NLGEOM=YES").unwrap();
    writeln!(structural_output, "*STATIC").unwrap();
    writeln!(structural_output, "{:.6}, {:.6}, {:.6}, {:.6}",
        job.structural_config.step_time,
        job.structural_config.step_time * 0.1,
        job.structural_config.step_time * 0.01,
        job.structural_config.step_time).unwrap();
    writeln!(structural_output, "*NODE FILE").unwrap();
    writeln!(structural_output, "U, NT").unwrap();
    writeln!(structural_output, "*EL FILE").unwrap();
    writeln!(structural_output, "S, E").unwrap();
    writeln!(structural_output, "*END STEP").unwrap();

    // Write files
    let work_path = PathBuf::from(&working_dir);
    std::fs::create_dir_all(&work_path).map_err(|e| e.to_string())?;

    let thermal_path = work_path.join("thermal_step.inp");
    std::fs::write(&thermal_path, &thermal_output).map_err(|e| e.to_string())?;

    let structural_path = work_path.join("structural_step.inp");
    std::fs::write(&structural_path, &structural_output).map_err(|e| e.to_string())?;

    tracing::info!("Thermal coupling INP files generated at {:?}", work_path);

    Ok(format!(
        "{},{}",
        thermal_path.to_string_lossy(),
        structural_path.to_string_lossy()
    ))
}

/// Generate mesh nodes and elements from mesh config
fn generate_mesh_nodes_elements(config: &ThermalCouplingMeshConfig) -> (Vec<(usize, f64, f64, f64)>, Vec<(usize, Vec<usize>)>) {
    let mut nodes = vec![];
    let mut elements = vec![];

    let nx = config.x_div + 1;
    let ny = config.y_div + 1;
    let nz = config.z_div + 1;

    // Generate nodes
    let dx = (config.x_max - config.x_min) / config.x_div as f64;
    let dy = (config.y_max - config.y_min) / config.y_div as f64;
    let dz = (config.z_max - config.z_min) / config.z_div as f64;

    let mut node_id = 1;
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let x = config.x_min + i as f64 * dx;
                let y = config.y_min + j as f64 * dy;
                let z = config.z_min + k as f64 * dz;
                nodes.push((node_id, x, y, z));
                node_id += 1;
            }
        }
    }

    // Generate C3D8 elements (8-node brick)
    let elem_type = &config.element_type.to_uppercase();
    if elem_type == "C3D8" || elem_type == "C3D8R" {
        let mut elem_id = 1;
        for k in 0..config.z_div {
            for j in 0..config.y_div {
                for i in 0..config.x_div {
                    let n000 = i + j * nx + k * nx * ny + 1;
                    let n100 = n000 + 1;
                    let n010 = n000 + nx;
                    let n110 = n000 + nx + 1;
                    let n001 = n000 + nx * ny;
                    let n101 = n000 + nx * ny + 1;
                    let n011 = n000 + nx * ny + nx;
                    let n111 = n000 + nx * ny + nx + 1;
                    elements.push((elem_id, vec![n000, n100, n110, n010, n001, n101, n111, n011]));
                    elem_id += 1;
                }
            }
        }
    } else if elem_type == "C3D4" {
        // Tetrahedral elements (simplified: split each hex into 5 or 6 tets)
        let mut elem_id = 1;
        for k in 0..config.z_div {
            for j in 0..config.y_div {
                for i in 0..config.x_div {
                    let n000 = i + j * nx + k * nx * ny + 1;
                    let n100 = n000 + 1;
                    let n010 = n000 + nx;
                    let n110 = n000 + nx + 1;
                    let n001 = n000 + nx * ny;
                    let n101 = n000 + nx * ny + 1;
                    let n011 = n000 + nx * ny + nx;
                    let n111 = n000 + nx * ny + nx + 1;
                    // 5 tetrahedra per hex
                    elements.push((elem_id,     vec![n000, n100, n010, n001])); elem_id += 1;
                    elements.push((elem_id,     vec![n100, n110, n010, n111])); elem_id += 1;
                    elements.push((elem_id,     vec![n010, n110, n011, n111])); elem_id += 1;
                    elements.push((elem_id,     vec![n000, n010, n001, n111])); elem_id += 1;
                    elements.push((elem_id,     vec![n100, n001, n101, n111])); elem_id += 1;
                }
            }
        }
    }

    (nodes, elements)
}

/// Get available coupling templates
#[command]
pub fn get_thermal_coupling_templates() -> Vec<TemplateConfig> {
    vec![
        TemplateConfig {
            id: "pcb_thermal".to_string(),
            name: "PCB热应力分析".to_string(),
            category: "电子封装".to_string(),
            description: "模拟PCB板在工作时的温度分布和热应力".to_string(),
            mesh_config: ThermalCouplingMeshConfig {
                x_min: 0.0, x_max: 0.05,
                y_min: 0.0, y_max: 0.05,
                z_min: 0.0, z_max: 0.0016,
                x_div: 10, y_div: 10, z_div: 4,
                element_type: "C3D8".to_string(),
            },
            thermal_config: ThermalAnalysisConfig {
                analysis_type: "steady_state".to_string(),
                initial_temperature: 293.15,
                time_period: 1.0,
                time_increment: 0.1,
                max_iterations: 100,
                tolerance: 1e-6,
                boundary_conditions: vec![
                    ThermalBcConfig {
                        name: "自然对流".to_string(),
                        bc_type: "convection".to_string(),
                        nodes: vec![],
                        surface_name: Some("EALL".to_string()),
                        temperature: None,
                        ambient_temperature: Some(293.15),
                        film_coefficient: Some(10.0),
                        heat_flux: None,
                        emissivity: None,
                    }
                ],
                heat_sources: vec![
                    HeatSourceConfig {
                        name: "芯片热源".to_string(),
                        source_type: "volume".to_string(),
                        magnitude: 1e7,
                        node_ids: None,
                        surface_name: None,
                    }
                ],
            },
            structural_config: StructuralAnalysisConfig {
                reference_temperature: 293.15,
                stress_free_temperature: 298.15,
                step_time: 1.0,
                boundary_conditions: vec![
                    StructuralBcConfig {
                        name: "固定边界".to_string(),
                        bc_type: "fixed".to_string(),
                        nodes: vec![],
                        fix_x: true, fix_y: true, fix_z: false,
                    }
                ],
            },
            material: ThermalCouplingMaterial {
                name: "FR4".to_string(),
                density: 1900.0,
                youngs_modulus: 22e9,
                poisson_ratio: 0.28,
                thermal_conductivity: 0.3,
                expansion_coefficient: 1.5e-5,
                specific_heat: 1150.0,
            },
        },
        TemplateConfig {
            id: "welding_residual".to_string(),
            name: "焊接残余应力分析".to_string(),
            category: "制造工艺".to_string(),
            description: "模拟焊接过程中的温度场和冷却后的残余应力分布".to_string(),
            mesh_config: ThermalCouplingMeshConfig {
                x_min: 0.0, x_max: 0.1,
                y_min: 0.0, y_max: 0.05,
                z_min: 0.0, z_max: 0.005,
                x_div: 20, y_div: 10, z_div: 2,
                element_type: "C3D8".to_string(),
            },
            thermal_config: ThermalAnalysisConfig {
                analysis_type: "transient".to_string(),
                initial_temperature: 293.15,
                time_period: 10.0,
                time_increment: 0.5,
                max_iterations: 200,
                tolerance: 1e-5,
                boundary_conditions: vec![
                    ThermalBcConfig {
                        name: "环境温度".to_string(),
                        bc_type: "convection".to_string(),
                        nodes: vec![],
                        surface_name: Some("EALL".to_string()),
                        temperature: None,
                        ambient_temperature: Some(293.15),
                        film_coefficient: Some(50.0),
                        heat_flux: None,
                        emissivity: None,
                    }
                ],
                heat_sources: vec![
                    HeatSourceConfig {
                        name: "焊接热源".to_string(),
                        source_type: "point".to_string(),
                        magnitude: 1000.0,
                        node_ids: Some(vec![1]),
                        surface_name: None,
                    }
                ],
            },
            structural_config: StructuralAnalysisConfig {
                reference_temperature: 293.15,
                stress_free_temperature: 293.15,
                step_time: 1.0,
                boundary_conditions: vec![
                    StructuralBcConfig {
                        name: "固定".to_string(),
                        bc_type: "fixed".to_string(),
                        nodes: vec![],
                        fix_x: true, fix_y: true, fix_z: true,
                    }
                ],
            },
            material: ThermalCouplingMaterial {
                name: "Steel".to_string(),
                density: 7850.0,
                youngs_modulus: 200e9,
                poisson_ratio: 0.3,
                thermal_conductivity: 50.0,
                expansion_coefficient: 1.2e-5,
                specific_heat: 450.0,
            },
        },
        TemplateConfig {
            id: "electronics_bga".to_string(),
            name: "BGA焊点热可靠性".to_string(),
            category: "电子封装".to_string(),
            description: "BGA封装在温度循环载荷下的热疲劳可靠性分析".to_string(),
            mesh_config: ThermalCouplingMeshConfig {
                x_min: 0.0, x_max: 0.02,
                y_min: 0.0, y_max: 0.02,
                z_min: 0.0, z_max: 0.005,
                x_div: 8, y_div: 8, z_div: 2,
                element_type: "C3D8".to_string(),
            },
            thermal_config: ThermalAnalysisConfig {
                analysis_type: "transient".to_string(),
                initial_temperature: 298.15,
                time_period: 600.0,
                time_increment: 30.0,
                max_iterations: 50,
                tolerance: 1e-4,
                boundary_conditions: vec![
                    ThermalBcConfig {
                        name: "对流换热".to_string(),
                        bc_type: "convection".to_string(),
                        nodes: vec![],
                        surface_name: Some("EALL".to_string()),
                        temperature: None,
                        ambient_temperature: Some(298.15),
                        film_coefficient: Some(20.0),
                        heat_flux: None,
                        emissivity: None,
                    }
                ],
                heat_sources: vec![
                    HeatSourceConfig {
                        name: "芯片热源".to_string(),
                        source_type: "volume".to_string(),
                        magnitude: 5e6,
                        node_ids: None,
                        surface_name: None,
                    }
                ],
            },
            structural_config: StructuralAnalysisConfig {
                reference_temperature: 298.15,
                stress_free_temperature: 298.15,
                step_time: 60.0,
                boundary_conditions: vec![
                    StructuralBcConfig {
                        name: "对称".to_string(),
                        bc_type: "fixed".to_string(),
                        nodes: vec![],
                        fix_x: false, fix_y: false, fix_z: true,
                    }
                ],
            },
            material: ThermalCouplingMaterial {
                name: "Solder".to_string(),
                density: 7380.0,
                youngs_modulus: 30e9,
                poisson_ratio: 0.35,
                thermal_conductivity: 50.0,
                expansion_coefficient: 2.5e-5,
                specific_heat: 230.0,
            },
        },
    ]
}

/// Generate sequential coupling workflow INP
#[command]
pub fn generate_sequential_coupling_inp_files(
    thermal_job: ThermalCouplingJob,
    structural_job: ThermalCouplingJob,
    working_dir: String,
) -> Result<(String, String), String> {
    let thermal_result = generate_thermal_coupling_inp(thermal_job, working_dir.clone())?;
    let structural_result = generate_thermal_coupling_inp(structural_job, working_dir)?;
    let parts: Vec<&str> = thermal_result.split(',').collect();
    let thermal_path = parts.get(0).ok_or("Missing thermal path")?;
    let parts2: Vec<&str> = structural_result.split(',').collect();
    let structural_path = parts2.get(0).ok_or("Missing structural path")?;
    Ok((thermal_path.to_string(), structural_path.to_string()))
}

/// Parse thermal result file and extract temperature field
#[command]
pub fn parse_thermal_result_file(result_path: String) -> Result<ThermalResultData, String> {
    use std::io::{BufRead, BufReader};

    let file = std::fs::File::open(&result_path)
        .map_err(|e| format!("Failed to open result file: {}", e))?;
    let reader = BufReader::new(file);

    let mut node_temperatures = vec![];
    let mut max_temp = f64::MIN;
    let mut min_temp = f64::MAX;
    let mut sum_temp = 0.0;

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let line = line.trim();
        // Look for temperature data in result file
        // CalculiX .dat file format or custom CSV
        if line.is_empty() || line.starts_with('#') || line.starts_with('*') || line.starts_with('-') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let (Ok(node_id), Ok(temp)) = (parts[0].parse::<usize>(), parts[1].parse::<f64>()) {
                node_temperatures.push((node_id, temp));
                max_temp = max_temp.max(temp);
                min_temp = min_temp.min(temp);
                sum_temp += temp;
            }
        }
    }

    let count = node_temperatures.len();
    if count == 0 {
        return Err("No temperature data found in result file".to_string());
    }

    let avg_temp = sum_temp / count as f64;

    Ok(ThermalResultData {
        node_temperatures,
        max_temperature: max_temp,
        min_temperature: min_temp,
        avg_temperature: avg_temp,
        result_path,
    })
}

/// Get node IDs for a given face (for boundary conditions)
#[command]
pub fn get_face_nodes(
    x_min: f64, x_max: f64,
    y_min: f64, y_max: f64,
    z_min: f64, z_max: f64,
    x_div: usize, y_div: usize, z_div: usize,
    face: String,
) -> Vec<usize> {
    let nx = x_div + 1;
    let ny = y_div + 1;
    let nz = z_div + 1;

    let _dx = (x_max - x_min) / x_div as f64;
    let dy = (y_max - y_min) / y_div as f64;
    let dz = (z_max - z_min) / z_div as f64;

    let mut nodes = vec![];

    match face.as_str() {
        "x_min" => {
            for k in 0..nz {
                for j in 0..ny {
                    let i = 0;
                    let nid = i + j * nx + k * nx * ny + 1;
                    let y = y_min + j as f64 * dy;
                    let z = z_min + k as f64 * dz;
                    if y >= y_min - 1e-9 && y <= y_max + 1e-9 && z >= z_min - 1e-9 && z <= z_max + 1e-9 {
                        nodes.push(nid);
                    }
                }
            }
        }
        "x_max" => {
            for k in 0..nz {
                for j in 0..ny {
                    let i = x_div;
                    let nid = i + j * nx + k * nx * ny + 1;
                    nodes.push(nid);
                }
            }
        }
        "y_min" => {
            for k in 0..nz {
                for i in 0..nx {
                    let j = 0;
                    let nid = i + j * nx + k * nx * ny + 1;
                    nodes.push(nid);
                }
            }
        }
        "y_max" => {
            for k in 0..nz {
                for i in 0..nx {
                    let j = y_div;
                    let nid = i + j * nx + k * nx * ny + 1;
                    nodes.push(nid);
                }
            }
        }
        "z_min" => {
            for j in 0..ny {
                for i in 0..nx {
                    let k = 0;
                    let nid = i + j * nx + k * nx * ny + 1;
                    nodes.push(nid);
                }
            }
        }
        "z_max" => {
            for j in 0..ny {
                for i in 0..nx {
                    let k = z_div;
                    let nid = i + j * nx + k * nx * ny + 1;
                    nodes.push(nid);
                }
            }
        }
        "all" => {
            for k in 0..nz {
                for j in 0..ny {
                    for i in 0..nx {
                        let nid = i + j * nx + k * nx * ny + 1;
                        nodes.push(nid);
                    }
                }
            }
        }
        _ => {}
    }

    nodes
}