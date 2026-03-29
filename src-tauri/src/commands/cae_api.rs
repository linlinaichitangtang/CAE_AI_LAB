//! CAE API - Rust API for frontend
//! Exposes CAE solver functionality to the Tauri frontend

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::command;

// Re-export types from other modules
use crate::commands::input_gen::{BoundaryCondition, Element, ElementType, Load, LoadType, Material, Model, Node, Step};
use crate::commands::output_parser::FrdParser;
use crate::commands::postprocess::{ColorMap, ResultSet, ResultStats};
use crate::commands::solver::{CalculiXSolver, GridConfig, MeshElementType, MeshGenerator, MeshError, SolverConfig, SolverResult, StructuredMesh};
use crate::commands::solver::bc::{BcContainer, Dof, FixedBc, LoadDirection, PointLoad, UniformLoad, UniformLoadType};

// ============================================================================
// Data Structures
// ============================================================================

/// Analysis job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisJob {
    pub id: String,
    pub name: String,
    pub model: Model,
    pub status: JobStatus,
    pub created_at: String,
    pub result: Option<ResultSet>,
}

/// Job status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

/// Mesh data for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshApiResult {
    pub nodes: Vec<NodeApi>,
    pub elements: Vec<ElementApi>,
    pub result_stats: Vec<ResultStats>,
}

/// Node in API format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeApi {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub ux: Option<f64>,
    pub uy: Option<f64>,
    pub uz: Option<f64>,
    pub u_magnitude: Option<f64>,
}

/// Element in API format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementApi {
    pub id: usize,
    pub element_type: String,
    pub node_ids: Vec<usize>,
    pub von_mises: Option<f64>,
    pub s11: Option<f64>,
    pub s22: Option<f64>,
    pub s33: Option<f64>,
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Check if CalculiX solver is available
#[command]
pub fn check_solver() -> Result<bool, String> {
    tracing::info!("Checking solver availability");
    
    let config = SolverConfig::default();
    let calc_solver = CalculiXSolver::new(config);
    
    calc_solver.check_availability()
        .map_err(|e| e.to_string())
}

/// Generate CalculiX input file
#[command]
pub fn generate_input(
    nodes: Vec<Node>,
    elements: Vec<Element>,
    materials: Vec<Material>,
    boundary_conditions: Vec<BoundaryCondition>,
    loads: Vec<Load>,
    output_path: String,
) -> Result<String, String> {
    tracing::info!("Generating input file: {}", output_path);
    
    let model = Model {
        nodes,
        elements,
        materials,
        steps: vec![Step::default()],
        boundary_conditions,
        loads,
        contact_pairs: vec![],
    };
    
    let generator = crate::commands::input_gen::InpGenerator::new(model);
    let path = PathBuf::from(&output_path);
    
    generator.write_file(&path)
        .map_err(|e| e.to_string())?;
    
    Ok(output_path)
}

/// Run CalculiX solver
#[command]
pub fn run_solver(
    input_file: String,
    working_dir: String,
    num_threads: Option<usize>,
) -> Result<SolverResult, String> {
    tracing::info!("Running solver: {} in {}", input_file, working_dir);
    
    let config = SolverConfig {
        executable_path: "ccx".to_string(),
        num_threads: num_threads.unwrap_or(4),
        memory_limit_mb: None,
    };
    
    let calc_solver = CalculiXSolver::new(config);
    let input_path = PathBuf::from(&input_file);
    let work_dir = PathBuf::from(&working_dir);
    
    calc_solver.solve(&input_path, &work_dir)
        .map_err(|e| e.to_string())
}

/// Parse solver results (.frd file)
#[command]
pub fn parse_results(frd_file: String) -> Result<ResultSet, String> {
    tracing::info!("Parsing results: {}", frd_file);
    
    let parser = FrdParser::new(PathBuf::from(&frd_file));
    let results = parser.parse()
        .map_err(|e| e.to_string())?;
    
    // Convert to postprocess ResultSet
    let result_set = ResultSet {
        job_id: results.job_id,
        analysis_type: crate::commands::postprocess::AnalysisType::Static,
        units: crate::commands::postprocess::Units::SI,
        steps: vec![],
        min_displacement: 0.0,
        max_displacement: 0.0,
        max_von_mises: 0.0,
        max_stress_location: None,
        warnings: results.warnings,
        errors: results.errors,
    };
    
    Ok(result_set)
}

/// Get mesh data for visualization
#[command]
pub fn get_mesh_data(result_set: ResultSet) -> Result<MeshApiResult, String> {
    tracing::info!("Getting mesh data");
    
    let mut nodes = vec![];
    let mut elements = vec![];
    let mut u_mags = vec![];
    let mut von_mises = vec![];
    
    // Extract displacement node data
    if let Some(step) = result_set.steps.first() {
        for node in &step.nodes {
            let ux = node.displacement.as_ref().map(|d| d.x);
            let uy = node.displacement.as_ref().map(|d| d.y);
            let uz = node.displacement.as_ref().map(|d| d.z);
            let u_mag = node.displacement.as_ref().map(|d| d.length());
            
            if let Some(m) = u_mag {
                u_mags.push(m);
            }
            
            nodes.push(NodeApi {
                id: node.id,
                x: node.position.x,
                y: node.position.y,
                z: node.position.z,
                ux,
                uy,
                uz,
                u_magnitude: u_mag,
            });
        }
        
        // Extract element stress data
        for elem in &step.elements {
            let vm = elem.max_von_mises;
            if let Some(m) = vm {
                von_mises.push(m);
            }
            
            let (s11, s22, s33) = if let Some(ref stresses) = elem.stresses {
                (
                    stresses.first().map(|s| s.s11),
                    stresses.first().map(|s| s.s22),
                    stresses.first().map(|s| s.s33),
                )
            } else {
                (None, None, None)
            };
            
            elements.push(ElementApi {
                id: elem.id,
                element_type: elem.element_type.clone(),
                node_ids: elem.node_ids.clone(),
                von_mises: vm,
                s11,
                s22,
                s33,
            });
        }
    }
    
    let result_stats = vec![
        ResultStats::compute(&u_mags),
        ResultStats::compute(&von_mises),
    ];
    
    Ok(MeshApiResult {
        nodes,
        elements,
        result_stats,
    })
}

/// Create simple beam model for testing
#[command]
pub fn create_beam_model(
    length: f64, 
    height: f64, 
    width: f64, 
    num_elements: usize,
) -> Result<String, String> {
    tracing::info!("Creating beam model: {} x {} x {}", length, height, width);
    
    let mut nodes: Vec<Node> = vec![];
    let mut elements: Vec<Element> = vec![];
    
    let dx = length / num_elements as f64;
    let num_nodes = num_elements + 1;
    
    // Generate nodes
    for i in 0..num_nodes {
        let x = i as f64 * dx;
        // Bottom nodes
        nodes.push(Node {
            id: i * 2 + 1,
            x,
            y: -height / 2.0,
            z: -width / 2.0,
        });
        nodes.push(Node {
            id: i * 2 + 2,
            x,
            y: -height / 2.0,
            z: width / 2.0,
        });
    }
    
    // Generate elements (C3D8 hexahedrons)
    for i in 0..num_elements {
        let n1 = i * 2 + 1;
        let n2 = i * 2 + 2;
        let n3 = i * 2 + 3;
        let n4 = i * 2 + 4;
        let n5 = n1 + num_nodes * 2;
        let n6 = n2 + num_nodes * 2;
        let n7 = n3 + num_nodes * 2;
        let n8 = n4 + num_nodes * 2;
        
        elements.push(Element {
            id: i + 1,
            element_type: ElementType::C3D8,
            nodes: vec![n1, n2, n3, n4, n5, n6, n7, n8],
        });
    }
    
    // Add top nodes
    for i in 0..num_nodes {
        let x = i as f64 * dx;
        nodes.push(Node {
            id: i * 2 + 1 + num_nodes * 2,
            x,
            y: height / 2.0,
            z: -width / 2.0,
        });
        nodes.push(Node {
            id: i * 2 + 2 + num_nodes * 2,
            x,
            y: height / 2.0,
            z: width / 2.0,
        });
    }
    
    let material = Material::new("Steel", 200000.0, 0.3).with_density(7850.0);
    
    let bc = BoundaryCondition {
        name: "Fixed".to_string(),
        nodes: vec![1, 2],
        fix_x: true,
        fix_y: true,
        fix_z: true,
        fix_temp: false,
    };
    
    let load = Load {
        load_type: LoadType::Force,
        magnitude: -1000.0,
        direction: Some(crate::commands::input_gen::Direction::Y),
        surface: None,
    };
    
    // Return serialized model
    let model = Model {
        nodes,
        elements,
        materials: vec![material],
        steps: vec![Step::default()],
        boundary_conditions: vec![bc],
        loads: vec![load],
        contact_pairs: vec![],
    };
    
    serde_json::to_string(&model)
        .map_err(|e| e.to_string())
}

/// Get color for result visualization
#[command]
pub fn get_color_map(value: f64, min: f64, max: f64, colormap: String) -> Result<(u8, u8, u8), String> {
    let cm = match colormap.to_lowercase().as_str() {
        "jet" => ColorMap::Jet,
        "viridis" => ColorMap::Viridis,
        "plasma" => ColorMap::Plasma,
        "inferno" => ColorMap::Inferno,
        "magma" => ColorMap::Magma,
        "gray" => ColorMap::Gray,
        _ => ColorMap::Jet,
    };
    
    Ok(cm.to_rgb(value, min, max))
}

/// Generate a 2D structured mesh (rectangular)
#[command]
pub fn generate_2d_mesh(
    nx: usize,
    ny: usize,
    size_x: f64,
    size_y: f64,
) -> Result<StructuredMesh, String> {
    tracing::info!("Generating 2D mesh: {}x{} elements, {}x{} size", nx, ny, size_x, size_y);
    
    CalculiXSolver::generate_2d_mesh(nx, ny, size_x, size_y)
        .map_err(|e| e.to_string())
}

/// Generate a 3D structured mesh (rectangular box)
#[command]
pub fn generate_3d_mesh(
    nx: usize,
    ny: usize,
    nz: usize,
    size_x: f64,
    size_y: f64,
    size_z: f64,
) -> Result<StructuredMesh, String> {
    tracing::info!("Generating 3D mesh: {}x{}x{} elements, {}x{}x{} size", nx, ny, nz, size_x, size_y, size_z);
    
    CalculiXSolver::generate_3d_mesh(nx, ny, nz, size_x, size_y, size_z)
        .map_err(|e| e.to_string())
}

/// Generate mesh with custom configuration
#[command]
pub fn generate_structured_mesh(
    nx: usize,
    ny: usize,
    nz: usize,
    size_x: f64,
    size_y: f64,
    size_z: f64,
    element_type: String,
) -> Result<StructuredMesh, String> {
    tracing::info!("Generating structured mesh with custom config");
    
    let elem_type = match element_type.to_uppercase().as_str() {
        "C3D8" | "HEX8" => MeshElementType::Hex8,
        "C3D20" | "HEX20" => MeshElementType::Hex20,
        "C3D4" | "TET4" => MeshElementType::Tet4,
        "C3D10" | "TET10" => MeshElementType::Tet10,
        "C3D6" | "WEDGE6" => MeshElementType::Wedge6,
        "C3D15" | "WEDGE15" => MeshElementType::Wedge15,
        "S4" | "QUAD4" => MeshElementType::Quad4,
        "S8" | "QUAD8" => MeshElementType::Quad8,
        "S3" | "TRI3" => MeshElementType::Tri3,
        "S6" | "TRI6" => MeshElementType::Tri6,
        _ => MeshElementType::Hex8,
    };
    
    let config = GridConfig {
        nx,
        ny,
        nz,
        size_x,
        size_y,
        size_z,
        element_type: elem_type,
    };
    
    CalculiXSolver::generate_structured_mesh(config)
        .map_err(|e| e.to_string())
}

/// Export mesh to INP format file
#[command]
pub fn export_mesh_to_inp(
    mesh: StructuredMesh,
    material_name: String,
    output_path: String,
) -> Result<String, String> {
    tracing::info!("Exporting mesh to INP: {}", output_path);
    
    use crate::commands::input_gen::{InpGenerator, Material as InpMaterial};
    
    // Convert structured mesh to model format
    let mut nodes: Vec<Node> = Vec::new();
    let mut elements: Vec<Element> = Vec::new();
    
    for (i, coord) in mesh.nodes.iter().enumerate() {
        nodes.push(Node {
            id: i + 1,
            x: coord[0],
            y: coord[1],
            z: coord[2],
        });
    }
    
    let elem_type = match mesh.element_type.as_str() {
        "C3D8" => ElementType::C3D8,
        "C3D20" => ElementType::C3D20,
        "C3D4" => ElementType::C3D4,
        "S4" => ElementType::S4,
        _ => ElementType::C3D8,
    };
    
    for (i, elem_nodes) in mesh.elements.iter().enumerate() {
        elements.push(Element {
            id: i + 1,
            element_type: elem_type.clone(),
            nodes: elem_nodes.iter().map(|n| n + 1).collect(),
        });
    }
    
    let material = InpMaterial::new(&material_name, 200000.0, 0.3);
    
    let model = Model {
        nodes,
        elements,
        materials: vec![material],
        steps: vec![Step::default()],
        boundary_conditions: vec![],
        loads: vec![],
        contact_pairs: vec![],
    };
    
    let generator = InpGenerator::new(model);
    let path = PathBuf::from(&output_path);
    
    generator.write_file(&path)
        .map_err(|e| e.to_string())?;
    
    Ok(output_path)
}

// ============================================================================
// Boundary Condition API
// ============================================================================

/// Create a fixed boundary condition
#[command]
pub fn create_fixed_bc(
    name: String,
    node_ids: Vec<usize>,
    bc_type: String,
) -> Result<FixedBc, String> {
    tracing::info!("Creating fixed BC: {} with {} nodes", name, node_ids.len());
    
    let bc = match bc_type.to_lowercase().as_str() {
        "fixed" | "all" => FixedBc::new(&name, node_ids),
        "encastre" => FixedBc::encastre(&name, node_ids),
        "pinned" => FixedBc::pinned(&name, node_ids),
        "roller_x" | "rollerx" => FixedBc::roller_x(&name, node_ids),
        "roller_y" | "rollery" => FixedBc::roller_y(&name, node_ids),
        "roller_z" | "rollerz" => FixedBc::roller_z(&name, node_ids),
        "symmetry" => FixedBc::symmetry(&name, node_ids),
        "custom" => FixedBc::custom(&name, node_ids, vec![Dof::TranslationX, Dof::TranslationY, Dof::TranslationZ]),
        _ => FixedBc::new(&name, node_ids),
    };
    
    Ok(bc)
}

/// Create a custom fixed BC with specific DOFs
#[command]
pub fn create_custom_fixed_bc(
    name: String,
    node_ids: Vec<usize>,
    dofs: Vec<i32>,
) -> Result<FixedBc, String> {
    tracing::info!("Creating custom fixed BC: {} with {} DOFs", name, dofs.len());
    
    let parsed_dofs: Result<Vec<Dof>, String> = dofs.iter()
        .map(|d| Dof::from_int(*d).ok_or_else(|| format!("Invalid DOF: {}", d)))
        .collect();
    
    let parsed_dofs = parsed_dofs?;
    
    Ok(FixedBc::custom(&name, node_ids, parsed_dofs))
}

/// Create a point load
#[command]
pub fn create_point_load(
    name: String,
    node_id: usize,
    magnitude: f64,
    direction: String,
) -> Result<PointLoad, String> {
    tracing::info!("Creating point load: {} on node {} with magnitude {}", name, node_id, magnitude);
    
    let load = match direction.to_lowercase().as_str() {
        "x" => PointLoad::x(&name, node_id, magnitude),
        "y" => PointLoad::y(&name, node_id, magnitude),
        "z" => PointLoad::z(&name, node_id, magnitude),
        "normal" => PointLoad::normal(&name, node_id, magnitude),
        _ => PointLoad::z(&name, node_id, magnitude),
    };
    
    Ok(load)
}

/// Create a point load with custom vector
#[command]
pub fn create_vector_point_load(
    name: String,
    node_id: usize,
    fx: f64,
    fy: f64,
    fz: f64,
) -> Result<PointLoad, String> {
    tracing::info!("Creating vector point load: {} on node {} with ({}, {}, {})", name, node_id, fx, fy, fz);
    
    Ok(PointLoad::vector(&name, node_id, fx, fy, fz))
}

/// Create a uniform pressure load
#[command]
pub fn create_pressure_load(
    name: String,
    surface_name: String,
    magnitude: f64,
) -> Result<UniformLoad, String> {
    tracing::info!("Creating pressure load: {} on surface {} with magnitude {}", name, surface_name, magnitude);
    
    Ok(UniformLoad::pressure(&name, surface_name, magnitude))
}

/// Create a uniform traction load
#[command]
pub fn create_traction_load(
    name: String,
    surface_name: String,
    direction: String,
    magnitude: f64,
) -> Result<UniformLoad, String> {
    tracing::info!("Creating traction load: {} on surface {} in {} direction", name, surface_name, direction);
    
    let load_dir = match direction.to_lowercase().as_str() {
        "x" => LoadDirection::X,
        "y" => LoadDirection::Y,
        "z" => LoadDirection::Z,
        _ => LoadDirection::Z,
    };
    
    Ok(UniformLoad::traction(&name, surface_name, load_dir, magnitude))
}

/// Create a heat flux uniform load
#[command]
pub fn create_heat_flux_load(
    name: String,
    surface_name: String,
    magnitude: f64,
) -> Result<UniformLoad, String> {
    tracing::info!("Creating heat flux load: {} on surface {}", name, surface_name);
    
    Ok(UniformLoad::heat_flux(&name, surface_name, magnitude))
}

/// Create a boundary condition container and add BCs
#[command]
pub fn create_bc_container(
    fixed_bcs: Vec<FixedBc>,
    point_loads: Vec<PointLoad>,
    uniform_loads: Vec<UniformLoad>,
) -> Result<BcContainer, String> {
    tracing::info!("Creating BC container with {} fixed, {} point loads, {} uniform loads",
        fixed_bcs.len(), point_loads.len(), uniform_loads.len());
    
    let mut container = BcContainer::new();
    
    for bc in fixed_bcs {
        container.add_fixed(bc);
    }
    
    for load in point_loads {
        container.add_point_load(load);
    }
    
    for load in uniform_loads {
        container.add_uniform_load(load);
    }
    
    Ok(container)
}

/// Generate INP section for boundary conditions
#[command]
pub fn generate_bc_inp(fixed_bcs: Vec<FixedBc>, point_loads: Vec<PointLoad>, uniform_loads: Vec<UniformLoad>) -> Result<String, String> {
    tracing::info!("Generating BC INP sections");
    
    let mut container = BcContainer::new();
    
    for bc in fixed_bcs {
        container.add_fixed(bc);
    }
    
    for load in point_loads {
        container.add_point_load(load);
    }
    
    for load in uniform_loads {
        container.add_uniform_load(load);
    }
    
    Ok(container.to_inp())
}

/// Get available boundary condition types
#[command]
pub fn get_bc_types() -> Vec<String> {
    vec![
        "fixed".to_string(),
        "encastre".to_string(),
        "pinned".to_string(),
        "roller_x".to_string(),
        "roller_y".to_string(),
        "roller_z".to_string(),
        "symmetry".to_string(),
        "custom".to_string(),
    ]
}

/// Get available load directions
#[command]
pub fn get_load_directions() -> Vec<String> {
    vec![
        "x".to_string(),
        "y".to_string(),
        "z".to_string(),
        "normal".to_string(),
    ]
}

/// Get available uniform load types
#[command]
pub fn get_uniform_load_types() -> Vec<String> {
    vec![
        "pressure".to_string(),
        "traction_x".to_string(),
        "traction_y".to_string(),
        "traction_z".to_string(),
        "heat_flux".to_string(),
        "film".to_string(),
    ]
}

/// Find nodes at a specific face of a mesh
#[command]
pub fn find_nodes_at_face(
    nodes: Vec<Node>,
    axis: String,
    value: f64,
    tolerance: f64,
) -> Result<Vec<usize>, String> {
    tracing::info!("Finding nodes at face: {} = {} (tolerance: {})", axis, value, tolerance);
    
    let node_ids: Vec<usize> = nodes.iter()
        .filter(|n| {
            let coord = match axis.to_lowercase().as_str() {
                "x" => n.x,
                "y" => n.y,
                "z" => n.z,
                _ => return false,
            };
            (coord - value).abs() < tolerance
        })
        .map(|n| n.id)
        .collect();
    
    Ok(node_ids)
}

/// Create fixed BC for cantilever beam (left face)
#[command]
pub fn create_cantilever_fixed_bc(nodes: Vec<Node>) -> Result<FixedBc, String> {
    tracing::info!("Creating cantilever fixed BC");
    
    let fixed_node_ids = find_nodes_at_face(nodes, "x".to_string(), 0.0, 1e-6)?;
    
    if fixed_node_ids.is_empty() {
        return Err("No nodes found at x=0 face".to_string());
    }
    
    Ok(FixedBc::encastre("Cantilever-Fixed", fixed_node_ids))
}

/// Create point load for cantilever beam (right face)
#[command]
pub fn create_cantilever_point_load(nodes: Vec<Node>, max_x: f64, magnitude: f64) -> Result<PointLoad, String> {
    tracing::info!("Creating cantilever point load at right face");
    
    let load_node_id = nodes.iter()
        .filter(|n| (n.x - max_x).abs() < 1e-6)
        .max_by(|a, b| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
        .map(|n| n.id)
        .ok_or("No nodes found at right face")?;
    
    Ok(PointLoad::y("Cantilever-Load", load_node_id, magnitude))
}

/// Generate complete INP file with mesh and BCs
#[command]
pub fn generate_complete_inp(
    nodes: Vec<Node>,
    elements: Vec<Element>,
    material: Material,
    fixed_bc: FixedBc,
    point_load: Option<PointLoad>,
    uniform_loads: Vec<UniformLoad>,
    output_path: String,
) -> Result<String, String> {
    tracing::info!("Generating complete INP file: {}", output_path);
    
    use crate::commands::input_gen::InpGenerator;
    
    let mut model = Model {
        nodes,
        elements,
        materials: vec![material],
        steps: vec![Step::default()],
        boundary_conditions: vec![],
        loads: vec![],
        contact_pairs: vec![],
    };
    
    // Add fixed BC as BoundaryCondition
    let bc = BoundaryCondition {
        name: fixed_bc.name.clone(),
        nodes: fixed_bc.nodes.clone(),
        fix_x: true,
        fix_y: true,
        fix_z: true,
        fix_temp: false,
    };
    model.boundary_conditions.push(bc);
    
    // Add point load if provided
    if let Some(load) = point_load {
        let load_type = match load.direction {
            LoadDirection::X => LoadType::Force,
            LoadDirection::Y => LoadType::Force,
            LoadDirection::Z => LoadType::Force,
            LoadDirection::Normal => LoadType::Force,
            LoadDirection::Custom(_, _, _) => LoadType::Force,
        };
        
        let direction = match load.direction {
            LoadDirection::X => Some(crate::commands::input_gen::Direction::X),
            LoadDirection::Y => Some(crate::commands::input_gen::Direction::Y),
            LoadDirection::Z => Some(crate::commands::input_gen::Direction::Z),
            LoadDirection::Normal => Some(crate::commands::input_gen::Direction::Z),
            LoadDirection::Custom(_, _, _) => Some(crate::commands::input_gen::Direction::Z),
        };
        
        model.loads.push(Load {
            load_type,
            magnitude: load.magnitude,
            direction,
            surface: None,
        });
    }
    
    let generator = InpGenerator::new(model);
    let path = PathBuf::from(&output_path);
    
    generator.write_file(&path)
        .map_err(|e| e.to_string())?;
    
    Ok(output_path)
}