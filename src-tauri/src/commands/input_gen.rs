//! CalculiX input file generator
//! Generates INP format files for CalculiX solver

use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Format error: {0}")]
    FormatError(#[from] std::fmt::Error),
    #[error("Invalid node: {0}")]
    InvalidNode(usize),
    #[error("Invalid element: {0}")]
    InvalidElement(usize),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    pub id: usize,
    pub element_type: ElementType,
    pub nodes: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ElementType {
    C3D4,      // Tetrahedron (4-node)
    C3D8,      // Hexahedron (8-node)
    C3D8R,     // 8-node brick with reduced integration
    C3D20,     // 20-node brick
    C3D6,      // 6-node wedge
    C2D3,      // 2D Triangle (3-node)
    C2D4,      // 2D Quadrilateral (4-node)
    S3,        // 3-node shell
    S4R,       // 4-node shell with reduced integration
    S4,        // 4-node shell
    B31,       // 2-node beam
    B32,       // 3-node beam
    MASS,      // Mass element
}

impl ElementType {
    pub fn as_str(&self) -> &str {
        match self {
            ElementType::C3D4 => "C3D4",
            ElementType::C3D8 => "C3D8",
            ElementType::C3D8R => "C3D8R",
            ElementType::C3D20 => "C3D20",
            ElementType::C3D6 => "C3D6",
            ElementType::C2D3 => "C2D3",
            ElementType::C2D4 => "C2D4",
            ElementType::S3 => "S3",
            ElementType::S4R => "S4R",
            ElementType::S4 => "S4",
            ElementType::B31 => "B31",
            ElementType::B32 => "B32",
            ElementType::MASS => "MASS",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "C3D4" => Some(ElementType::C3D4),
            "C3D8" => Some(ElementType::C3D8),
            "C3D8R" => Some(ElementType::C3D8R),
            "C3D20" => Some(ElementType::C3D20),
            "C3D6" => Some(ElementType::C3D6),
            "C2D3" => Some(ElementType::C2D3),
            "C2D4" => Some(ElementType::C2D4),
            "S3" => Some(ElementType::S3),
            "S4R" => Some(ElementType::S4R),
            "S4" => Some(ElementType::S4),
            "B31" => Some(ElementType::B31),
            "B32" => Some(ElementType::B32),
            "MASS" => Some(ElementType::MASS),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoadType {
    Force,
    Pressure,
    Temperature,
    HeatFlux,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Load {
    pub load_type: LoadType,
    pub magnitude: f64,
    pub direction: Option<Direction>,
    pub surface: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Direction {
    X,
    Y,
    Z,
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    pub density: f64,
    pub youngs_modulus: f64,
    pub poisson_ratio: f64,
    pub thermal_conductivity: Option<f64>,
    pub expansion_coefficient: Option<f64>,
    pub specific_heat: Option<f64>,
    /// Material type: "elastic", "plastic", "viscoelastic", "hyperelastic"
    pub material_type: Option<String>,
    /// Plastic parameters (for elasto-plastic materials)
    pub plastic_params: Option<PlasticParams>,
    /// Viscoelastic parameters (for viscoelastic materials)
    pub viscoelastic_params: Option<ViscoelasticParams>,
    /// Hyperelastic parameters (for hyperelastic materials)
    pub hyperelastic_params: Option<HyperelasticParams>,
}

/// Plastic material parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlasticParams {
    /// Yield criterion: "von_mises", "tresca", "drucker_prager"
    pub yield_criterion: Option<String>,
    /// Hardening type: "isotropic", "kinematic", "combined"
    pub hardening_type: Option<String>,
    /// Plastic model: "bilinear", "multilinear", "exponential"
    pub model: Option<String>,
    /// Yield strength in MPa
    pub yield_strength: Option<f64>,
    /// Tangent modulus for bilinear model in MPa
    pub tangent_modulus: Option<f64>,
    /// Stress-strain table for multilinear model
    pub plastic_table: Option<Vec<Point2D>>,
}

/// Simple 2D point for stress-strain data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,  // strain
    pub y: f64,  // stress (MPa)
}

/// Viscoelastic material parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViscoelasticParams {
    /// Viscoelastic model: "maxwell", "kelvin", "standard_linear", "generalized_maxwell"
    pub model: Option<String>,
    /// Elastic modulus in MPa
    pub elastic_modulus: Option<f64>,
    /// Viscosity for Maxwell/Kelvin models
    pub viscosity: Option<f64>,
    /// Prony series terms for generalized Maxwell
    pub prony_series: Option<Vec<PronyTerm>>,
}

/// Prony series term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PronyTerm {
    /// Relative modulus E_i / E_0
    pub relative_modulus: f64,
    /// Relaxation time in seconds
    pub relaxation_time: f64,
}

/// Hyperelastic material parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperelasticParams {
    /// Hyperelastic model: "neo_hookean", "mooney_rivlin", "mooney_rivlin_3", "ogden", "yeoh"
    pub model: Option<String>,
    /// Mooney-Rivlin parameters
    pub c10: Option<f64>,
    pub c01: Option<f64>,
    pub c20: Option<f64>,
    /// D parameter for compressibility
    pub d: Option<f64>,
    /// Ogden terms
    pub ogden_terms: Option<Vec<OgdenTerm>>,
}

/// Ogden model term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OgdenTerm {
    pub mu: f64,
    pub alpha: f64,
}

impl Material {
    pub fn new(name: &str, youngs_modulus: f64, poisson_ratio: f64) -> Self {
        Self {
            name: name.to_string(),
            density: 0.0,
            youngs_modulus,
            poisson_ratio,
            thermal_conductivity: None,
            expansion_coefficient: None,
            specific_heat: None,
            material_type: Some("elastic".to_string()),
            plastic_params: None,
            viscoelastic_params: None,
            hyperelastic_params: None,
        }
    }

    pub fn with_density(mut self, density: f64) -> Self {
        self.density = density;
        self
    }

    pub fn with_thermal(mut self, conductivity: f64, expansion: f64, specific_heat: f64) -> Self {
        self.thermal_conductivity = Some(conductivity);
        self.expansion_coefficient = Some(expansion);
        self.specific_heat = Some(specific_heat);
        self
    }

    pub fn with_plastic(mut self, params: PlasticParams) -> Self {
        self.material_type = Some("plastic".to_string());
        self.plastic_params = Some(params);
        self
    }

    pub fn with_viscoelastic(mut self, params: ViscoelasticParams) -> Self {
        self.material_type = Some("viscoelastic".to_string());
        self.viscoelastic_params = Some(params);
        self
    }

    pub fn with_hyperelastic(mut self, params: HyperelasticParams) -> Self {
        self.material_type = Some("hyperelastic".to_string());
        self.hyperelastic_params = Some(params);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub name: String,
    pub time_period: f64,
    pub initial_time_increment: f64,
    pub minimum_time_increment: f64,
    pub maximum_time_increment: f64,
    pub static_or_thermal: bool,
}

impl Default for Step {
    fn default() -> Self {
        Self {
            name: "Step-1".to_string(),
            time_period: 1.0,
            initial_time_increment: 0.1,
            minimum_time_increment: 1e-8,
            maximum_time_increment: 0.1,
            static_or_thermal: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundaryCondition {
    pub name: String,
    pub nodes: Vec<usize>,
    pub fix_x: bool,
    pub fix_y: bool,
    pub fix_z: bool,
    pub fix_temp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContactPair {
    pub master_surface: String,
    pub slave_surface: String,
    pub friction: f64,
    pub contact_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub nodes: Vec<Node>,
    pub elements: Vec<Element>,
    pub materials: Vec<Material>,
    pub steps: Vec<Step>,
    pub boundary_conditions: Vec<BoundaryCondition>,
    pub loads: Vec<Load>,
    pub contact_pairs: Vec<ContactPair>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            nodes: vec![],
            elements: vec![],
            materials: vec![],
            steps: vec![Step::default()],
            boundary_conditions: vec![],
            loads: vec![],
            contact_pairs: vec![],
        }
    }
}

pub struct InpGenerator {
    model: Model,
}

impl InpGenerator {
    pub fn new(model: Model) -> Self {
        Self { model }
    }

    /// Generate INP file content
    pub fn generate(&self) -> Result<String, InputError> {
        let mut output = String::new();

        // Write heading
        writeln!(output, "** Generated by CAELab")?;
        writeln!(output, "** CalculiX input file")?;
        writeln!(output)?;

        // Write nodes
        self.write_nodes(&mut output)?;
        
        // Write elements
        self.write_elements(&mut output)?;
        
        // Write materials
        self.write_materials(&mut output)?;
        
        // Write sections
        self.write_sections(&mut output)?;
        
        // Write boundary conditions
        self.write_boundary_conditions(&mut output)?;
        
        // Write loads
        self.write_loads(&mut output)?;
        
        // Write steps
        self.write_steps(&mut output)?;

        Ok(output)
    }

    pub fn write_nodes(&self, output: &mut String) -> Result<(), InputError> {
        writeln!(output, "*NODE")?;
        
        for node in &self.model.nodes {
            writeln!(output, "{}, {:.6}, {:.6}, {:.6}", node.id, node.x, node.y, node.z)?;
        }
        
        writeln!(output)?;
        Ok(())
    }

    pub fn write_elements(&self, output: &mut String) -> Result<(), InputError> {
        // Group elements by type
        let mut elements_by_type: std::collections::HashMap<String, Vec<&Element>> = std::collections::HashMap::new();
        
        for elem in &self.model.elements {
            elements_by_type
                .entry(elem.element_type.as_str().to_string())
                .or_insert_with(Vec::new)
                .push(elem);
        }

        for (etype, elements) in elements_by_type {
            if elements.is_empty() {
                continue;
            }
            
            writeln!(output, "*ELEMENT, TYPE={}", etype)?;
            
            for elem in elements {
                let node_list = elem.nodes.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
                writeln!(output, "{}, {}", elem.id, node_list)?;
            }
            
            writeln!(output)?;
        }

        Ok(())
    }

    pub fn write_materials(&self, output: &mut String) -> Result<(), InputError> {
        for material in &self.model.materials {
            writeln!(output, "*MATERIAL, NAME={}", material.name)?;
            
            // Get material type or default to elastic
            let mat_type = material.material_type.as_deref().unwrap_or("elastic");
            
            match mat_type {
                "plastic" => {
                    self.write_plastic_material(output, material)?;
                }
                "viscoelastic" => {
                    self.write_viscoelastic_material(output, material)?;
                }
                "hyperelastic" => {
                    self.write_hyperelastic_material(output, material)?;
                }
                _ => {
                    // Default: Linear elastic
                    self.write_elastic_material(output, material)?;
                }
            }
            
            writeln!(output)?;
        }

        Ok(())
    }
    
    /// Write linear elastic material (*ELASTIC, *DENSITY)
    fn write_elastic_material(&self, output: &mut String, material: &Material) -> Result<(), InputError> {
        writeln!(output, "*ELASTIC")?;
        writeln!(output, "{}, POISSON={:.6}", material.youngs_modulus, material.poisson_ratio)?;
        
        if material.density > 0.0 {
            writeln!(output, "*DENSITY")?;
            writeln!(output, "{:.6}", material.density)?;
        }
        
        // Thermal properties
        if let (Some(k), Some(alpha), Some(cp)) = (
            material.thermal_conductivity,
            material.expansion_coefficient,
            material.specific_heat,
        ) {
            writeln!(output, "*CONDUCTIVITY")?;
            writeln!(output, "{:.6}", k)?;
            writeln!(output, "*EXPANSION")?;
            writeln!(output, "{:.6}", alpha)?;
            writeln!(output, "*SPECIFIC HEAT")?;
            writeln!(output, "{:.6}", cp)?;
        }
        
        Ok(())
    }
    
    /// Write elasto-plastic material (*ELASTIC, *PLASTIC)
    fn write_plastic_material(&self, output: &mut String, material: &Material) -> Result<(), InputError> {
        // Base elastic properties
        writeln!(output, "*ELASTIC")?;
        writeln!(output, "{}, POISSON={:.6}", material.youngs_modulus, material.poisson_ratio)?;
        
        if material.density > 0.0 {
            writeln!(output, "*DENSITY")?;
            writeln!(output, "{:.6}", material.density)?;
        }
        
        // Get plastic parameters
        if let Some(ref plastic) = material.plastic_params {
            // Determine yield criterion
            let criterion = plastic.yield_criterion.as_deref().unwrap_or("von_mises");
            let hardening = plastic.hardening_type.as_deref().unwrap_or("isotropic");
            let model = plastic.model.as_deref().unwrap_or("bilinear");
            
            // Write *PLASTIC card
            writeln!(output, "*PLASTIC")?;
            
            if model == "bilinear" {
                // Bilinear plastic: yield strength and tangent modulus
                let sigma_y = plastic.yield_strength.unwrap_or(250.0);
                let e_t = plastic.tangent_modulus.unwrap_or(20000.0);
                
                // Table format: strain increment, stress
                // For bilinear, we have two points: (0, sigma_y) and (ep_max, sigma_y + E_t*ep_max)
                // Simplified: just yield stress and hardening parameter
                if hardening == "isotropic" {
                    writeln!(output, "{:.6}, {:.6}", sigma_y, e_t / material.youngs_modulus)?;
                } else {
                    // Kinematic or combined hardening
                    writeln!(output, "{:.6}, {:.6}", sigma_y, e_t / material.youngs_modulus)?;
                }
            } else if model == "multilinear" {
                // Multilinear plastic: use stress-strain table
                if let Some(ref table) = plastic.plastic_table {
                    for point in table {
                        writeln!(output, "{:.6}, {:.6}", point.x, point.y)?;
                    }
                } else {
                    // Default if no table provided
                    let sigma_y = plastic.yield_strength.unwrap_or(250.0);
                    let e_t = plastic.tangent_modulus.unwrap_or(20000.0);
                    writeln!(output, "{:.6}, {:.6}", 0.0, sigma_y)?;
                    writeln!(output, "{:.6}, {:.6}", 0.02, sigma_y + e_t * 0.02)?;
                }
            } else {
                // Exponential or default
                let sigma_y = plastic.yield_strength.unwrap_or(250.0);
                writeln!(output, "{:.6}, {:.6}", sigma_y, 0.0)?;
            }
            
            // Write yield criterion if not von Mises
            if criterion != "von_mises" {
                // CalculiX supports: CRUSHING, MISE, MRTECH, TRIAX
                let criterion_code = match criterion {
                    "tresca" => "TRIAX",
                    "drucker_prager" => "DRUCKER",
                    _ => "MISE",
                };
                writeln!(output, "*YIELD STRESS, YIELD CRITERION={}", criterion_code)?;
                if let Some(ref table) = plastic.plastic_table {
                    for point in table {
                        writeln!(output, "{:.6}", point.y)?;
                    }
                }
            }
        } else {
            // Default plastic parameters
            writeln!(output, "250., 0.0")?;
        }
        
        // Thermal properties
        if let (Some(k), Some(alpha), Some(cp)) = (
            material.thermal_conductivity,
            material.expansion_coefficient,
            material.specific_heat,
        ) {
            writeln!(output, "*CONDUCTIVITY")?;
            writeln!(output, "{:.6}", k)?;
            writeln!(output, "*EXPANSION")?;
            writeln!(output, "{:.6}", alpha)?;
            writeln!(output, "*SPECIFIC HEAT")?;
            writeln!(output, "{:.6}", cp)?;
        }
        
        Ok(())
    }
    
    /// Write viscoelastic material (*ELASTIC, *VISCOELASTIC)
    fn write_viscoelastic_material(&self, output: &mut String, material: &Material) -> Result<(), InputError> {
        // Base elastic properties (instantaneous modulus)
        let e0 = material.viscoelastic_params
            .as_ref()
            .and_then(|v| v.elastic_modulus)
            .unwrap_or(material.youngs_modulus);
        
        writeln!(output, "*ELASTIC")?;
        writeln!(output, "{}, POISSON={:.6}", e0, material.poisson_ratio)?;
        
        if material.density > 0.0 {
            writeln!(output, "*DENSITY")?;
            writeln!(output, "{:.6}", material.density)?;
        }
        
        // Get viscoelastic parameters
        if let Some(ref visco) = material.viscoelastic_params {
            let model = visco.model.as_deref().unwrap_or("generalized_maxwell");
            
            match model {
                "maxwell" | "kelvin" => {
                    // Single mode Maxwell/Kelvin
                    let eta = visco.viscosity.unwrap_or(1000.0);
                    let e = visco.elastic_modulus.unwrap_or(e0);
                    
                    writeln!(output, "*VISCOELASTIC, TIME=PRONY")?;
                    // For single mode: g1, tau1
                    let g1 = eta / (eta + e * 1.0); // Simplified calculation
                    writeln!(output, "{:.6}, {:.6}", 1.0 - g1, 1.0)?;
                }
                "standard_linear" => {
                    // Standard linear solid (2-parameter)
                    writeln!(output, "*VISCOELASTIC, TIME=PRONY")?;
                    writeln!(output, "0.5, 1.0")?;
                }
                "generalized_maxwell" => {
                    // Generalized Maxwell (Prony series)
                    writeln!(output, "*VISCOELASTIC, TIME=PRONY")?;
                    
                    if let Some(ref prony) = visco.prony_series {
                        for term in prony {
                            writeln!(output, "{:.6}, {:.6}", term.relative_modulus, term.relaxation_time)?;
                        }
                    } else {
                        // Default Prony terms
                        writeln!(output, "0.2, 1.0")?;
                        writeln!(output, "0.1, 10.0")?;
                    }
                }
                _ => {
                    writeln!(output, "*VISCOELASTIC, TIME=PRONY")?;
                    writeln!(output, "0.2, 1.0")?;
                }
            }
        } else {
            // Default viscoelastic
            writeln!(output, "*VISCOELASTIC, TIME=PRONY")?;
            writeln!(output, "0.2, 1.0")?;
            writeln!(output, "0.1, 10.0")?;
        }
        
        // Thermal properties
        if let (Some(k), Some(alpha), Some(cp)) = (
            material.thermal_conductivity,
            material.expansion_coefficient,
            material.specific_heat,
        ) {
            writeln!(output, "*CONDUCTIVITY")?;
            writeln!(output, "{:.6}", k)?;
            writeln!(output, "*EXPANSION")?;
            writeln!(output, "{:.6}", alpha)?;
            writeln!(output, "*SPECIFIC HEAT")?;
            writeln!(output, "{:.6}", cp)?;
        }
        
        Ok(())
    }
    
    /// Write hyperelastic material (*HYPERELASTIC)
    fn write_hyperelastic_material(&self, output: &mut String, material: &Material) -> Result<(), InputError> {
        // Get hyperelastic parameters
        if let Some(ref hyper) = material.hyperelastic_params {
            let model = hyper.model.as_deref().unwrap_or("neo_hookean");
            
            match model {
                "neo_hookean" => {
                    let c10 = hyper.c10.unwrap_or(0.5);
                    let d = hyper.d.unwrap_or(0.001);
                    
                    writeln!(output, "*HYPERELASTIC, MODEL=NH")?;
                    writeln!(output, "{:.6}, {:.6}", c10, d)?;
                }
                "mooney_rivlin" | "mooney_rivlin_3" => {
                    let c10 = hyper.c10.unwrap_or(0.3);
                    let c01 = hyper.c01.unwrap_or(0.0);
                    let c20 = hyper.c20.unwrap_or(0.0);
                    let d = hyper.d.unwrap_or(0.001);
                    
                    if model == "mooney_rivlin" {
                        writeln!(output, "*HYPERELASTIC, MODEL=MR")?;
                        writeln!(output, "{:.6}, {:.6}, {:.6}", c10, c01, d)?;
                    } else {
                        writeln!(output, "*HYPERELASTIC, MODEL=MR3")?;
                        writeln!(output, "{:.6}, {:.6}, {:.6}, {:.6}", c10, c01, c20, d)?;
                    }
                }
                "ogden" => {
                    writeln!(output, "*HYPERELASTIC, MODEL=OGDEN")?;
                    
                    if let Some(ref terms) = hyper.ogden_terms {
                        let d = hyper.d.unwrap_or(0.001);
                        // First line: number of terms and D
                        writeln!(output, "{}, {:.6}", terms.len(), d)?;
                        for term in terms {
                            writeln!(output, "{:.6}, {:.6}", term.mu, term.alpha)?;
                        }
                    } else {
                        // Default Ogden term
                        writeln!(output, "1, {:.6}", hyper.d.unwrap_or(0.001))?;
                        writeln!(output, "1.0, 2.0")?;
                    }
                }
                "yeoh" => {
                    let c10 = hyper.c10.unwrap_or(0.5);
                    let c01 = hyper.c01.unwrap_or(0.0);
                    let c20 = hyper.c20.unwrap_or(0.0);
                    let d = hyper.d.unwrap_or(0.001);
                    
                    writeln!(output, "*HYPERELASTIC, MODEL=YEOH")?;
                    writeln!(output, "{:.6}, {:.6}, {:.6}, {:.6}", c10, c01, c20, d)?;
                }
                _ => {
                    // Default to Neo-Hookean
                    let c10 = hyper.c10.unwrap_or(0.5);
                    let d = hyper.d.unwrap_or(0.001);
                    writeln!(output, "*HYPERELASTIC, MODEL=NH")?;
                    writeln!(output, "{:.6}, {:.6}", c10, d)?;
                }
            }
        } else {
            // Default hyperelastic
            writeln!(output, "*HYPERELASTIC, MODEL=NH")?;
            writeln!(output, "0.5, 0.001")?;
        }
        
        // Density
        if material.density > 0.0 {
            writeln!(output, "*DENSITY")?;
            writeln!(output, "{:.6}", material.density)?;
        }
        
        Ok(())
    }

    pub fn write_sections(&self, output: &mut String) -> Result<(), InputError> {
        // Group elements by material
        let mut elem_by_mat: std::collections::HashMap<String, Vec<usize>> = std::collections::HashMap::new();
        
        for (i, elem) in self.model.elements.iter().enumerate() {
            if i < self.model.materials.len() {
                let mat_name = &self.model.materials[i].name;
                elem_by_mat
                    .entry(mat_name.clone())
                    .or_insert_with(Vec::new)
                    .push(elem.id);
            }
        }

        for (mat_name, elem_ids) in elem_by_mat {
            if elem_ids.is_empty() {
                continue;
            }
            
            let _elem_type = self.model.elements
                .iter()
                .find(|e| e.id == elem_ids[0])
                .map(|e| e.element_type.as_str())
                .unwrap_or("C3D8R");

            writeln!(output, "*SOLID SECTION, ELSET=EALL, MATERIAL={}", mat_name)?;
            writeln!(output)?;
        }

        Ok(())
    }

    pub fn write_boundary_conditions(&self, output: &mut String) -> Result<(), InputError> {
        for bc in &self.model.boundary_conditions {
            writeln!(output, "*BOUNDARY")?;
            
            let mut conditions = vec![];
            if bc.fix_x { conditions.push("1"); }
            if bc.fix_y { conditions.push("2"); }
            if bc.fix_z { conditions.push("3"); }
            if bc.fix_temp { conditions.push("11"); }
            
            for node_id in &bc.nodes {
                if conditions.is_empty() {
                    writeln!(output, "{}, 1, 3, 0.0", node_id)?;
                } else {
                    let cond = conditions.join(", ");
                    writeln!(output, "{}, {}, 0.0", node_id, cond)?;
                }
            }
            
            writeln!(output)?;
        }

        Ok(())
    }

    pub fn write_loads(&self, output: &mut String) -> Result<(), InputError> {
        for load in &self.model.loads {
            match load.load_type {
                LoadType::Force => {
                    writeln!(output, "*CLOAD")?;
                    let direction = match load.direction {
                        Some(Direction::X) => "1",
                        Some(Direction::Y) => "2",
                        Some(Direction::Z) => "3",
                        Some(Direction::Normal) => "3",
                        None => "3",
                    };
                    writeln!(output, "{}, {}, {:.6}", direction, load.magnitude, load.magnitude)?;
                }
                LoadType::Pressure => {
                    writeln!(output, "*DLOAD")?;
                    if let Some(ref surface) = load.surface {
                        writeln!(output, "{}, P, {:.6}", surface, load.magnitude)?;
                    }
                }
                LoadType::Temperature => {
                    writeln!(output, "*TEMPERATURE")?;
                    writeln!(output, "{}, {:.6}", load.magnitude, load.magnitude)?;
                }
                LoadType::HeatFlux => {
                    writeln!(output, "*CFLUX")?;
                    writeln!(output, "{}, 1, {:.6}", load.magnitude, load.magnitude)?;
                }
            }
            
            writeln!(output)?;
        }

        Ok(())
    }

    fn write_steps(&self, output: &mut String) -> Result<(), InputError> {
        for step in &self.model.steps {
            if step.static_or_thermal {
                writeln!(output, "*STEP, NLGEOM=NO")?;
            } else {
                writeln!(output, "*STEP, STATIC")?;
            }
            
            writeln!(output, "*{}, END=PERIOD", if step.static_or_thermal { "STATIC" } else { "HEAT" })?;
            writeln!(output, "{:.6}, {:.6}, {:.6}, {:.6}", 
                step.time_period,
                step.initial_time_increment,
                step.minimum_time_increment,
                step.maximum_time_increment)?;
            
            // Write contact pairs in step
            self.write_contact_pairs(output)?;
            
            writeln!(output, "*END STEP")?;
            writeln!(output)?;
        }

        Ok(())
    }

    fn write_contact_pairs(&self, output: &mut String) -> Result<(), InputError> {
        for contact in &self.model.contact_pairs {
            let contact_type_upper = contact.contact_type.to_uppercase();
            
            match contact_type_upper.as_str() {
                "tie" | "TIE" => {
                    // Tie constraint (bonded contact)
                    writeln!(output, "*TIE, NAME={}", contact.master_surface)?;
                    writeln!(output, "{}, {}", contact.slave_surface, contact.master_surface)?;
                }
                "bolt" | "BOLT" | "BOLT_PRELOAD" => {
                    // Bolt preload contact
                    writeln!(output, "*SURFACE INTERACTION, NAME={}", contact.master_surface)?;
                    writeln!(output, "*FRICTION")?;
                    writeln!(output, "{:.6}", contact.friction)?;
                    writeln!(output, "*CONTACT PAIR, INTERACTION={}", contact.master_surface)?;
                    writeln!(output, "{}, {}", contact.slave_surface, contact.master_surface)?;
                }
                _ => {
                    // Standard surface-to-surface contact
                    writeln!(output, "*SURFACE INTERACTION, NAME={}", contact.master_surface)?;
                    
                    // Contact properties - adjust based on algorithm
                    let behavior = if contact_type_upper.contains("LAGRANGE") {
                        "EXPONENTIAL"
                    } else if contact_type_upper.contains("AUGMENTED") {
                        "TIED"
                    } else {
                        "LINEAR"  // penalty or default
                    };
                    writeln!(output, "*SURFACE BEHAVIOR, PRESSURE-OVERCLOSURE={}", behavior)?;
                    
                    // Friction if specified
                    if contact.friction > 0.0 {
                        writeln!(output, "*FRICTION")?;
                        writeln!(output, "{:.6}", contact.friction)?;
                    }
                    
                    writeln!(output, "*CONTACT PAIR, INTERACTION={}", contact.master_surface)?;
                    writeln!(output, "{}, {}", contact.slave_surface, contact.master_surface)?;
                }
            }
            
            writeln!(output)?;
        }
        
        Ok(())
    }

    /// Write INP file to disk
    pub fn write_file(&self, path: &PathBuf) -> Result<(), InputError> {
        let content = self.generate()?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_type_string() {
        assert_eq!(ElementType::C3D8.as_str(), "C3D8");
        assert_eq!(ElementType::S4R.as_str(), "S4R");
    }

    #[test]
    fn test_material() {
        let mat = Material::new("Steel", 200000.0, 0.3).with_density(7850.0);
        assert_eq!(mat.name, "Steel");
        assert_eq!(mat.youngs_modulus, 200000.0);
        assert_eq!(mat.poisson_ratio, 0.3);
        assert_eq!(mat.density, 7850.0);
    }

    #[test]
    fn test_model_generation() {
        let model = Model::default();
        let gen = InpGenerator::new(model);
        let content = gen.generate().unwrap();
        assert!(content.contains("*NODE"));
    }
}