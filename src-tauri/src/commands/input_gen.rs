//! CalculiX input file generator
//! Generates INP format files for CalculiX solver

use serde::{Deserialize, Serialize};
use std::fmt::Write as FmtWrite;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
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

    fn write_nodes(&self, output: &mut String) -> Result<(), InputError> {
        writeln!(output, "*NODE")?;
        
        for node in &self.model.nodes {
            writeln!(output, "{}, {:.6}, {:.6}, {:.6}", node.id, node.x, node.y, node.z)?;
        }
        
        writeln!(output)?;
        Ok(())
    }

    fn write_elements(&self, output: &mut String) -> Result<(), InputError> {
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

    fn write_materials(&self, output: &mut String) -> Result<(), InputError> {
        for material in &self.model.materials {
            writeln!(output, "*MATERIAL, NAME={}", material.name)?;
            
            // Elastic properties
            writeln!(output, "*ELASTIC")?;
            writeln!(output, "{}, POISSON={:.6}", material.youngs_modulus, material.poisson_ratio)?;
            
            // Density
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
            
            writeln!(output)?;
        }

        Ok(())
    }

    fn write_sections(&self, output: &mut String) -> Result<(), InputError> {
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
            
            let elem_type = self.model.elements
                .iter()
                .find(|e| e.id == elem_ids[0])
                .map(|e| e.element_type.as_str())
                .unwrap_or("C3D8R");

            writeln!(output, "*SOLID SECTION, ELSET=EALL, MATERIAL={}", mat_name)?;
            writeln!(output)?;
        }

        Ok(())
    }

    fn write_boundary_conditions(&self, output: &mut String) -> Result<(), InputError> {
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

    fn write_loads(&self, output: &mut String) -> Result<(), InputError> {
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
            
            writeln!(output, "*END STEP")?;
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