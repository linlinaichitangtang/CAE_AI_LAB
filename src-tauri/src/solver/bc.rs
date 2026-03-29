//! Boundary Conditions Module
//! Implements Fixed, Point Load, and Uniform Load constraints

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use crate::commands::input_gen::{Node, Direction};

#[derive(Error, Debug)]
pub enum BcError {
    #[error("Invalid node ID: {0}")]
    InvalidNodeId(usize),
    #[error("Invalid face ID: {0}")]
    InvalidFaceId(String),
    #[error("Node not found: {0}")]
    NodeNotFound(usize),
    #[error("Mesh not initialized")]
    MeshNotInitialized,
    #[error("Invalid magnitude: {0}")]
    InvalidMagnitude(f64),
}

/// Represents a degree of freedom
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Dof {
    TranslationX = 1,
    TranslationY = 2,
    TranslationZ = 3,
    RotationX = 4,
    RotationY = 5,
    RotationZ = 6,
    Temperature = 11,
}

impl Dof {
    pub fn as_int(&self) -> i32 {
        *self as i32
    }
    
    pub fn from_int(v: i32) -> Option<Self> {
        match v {
            1 => Some(Dof::TranslationX),
            2 => Some(Dof::TranslationY),
            3 => Some(Dof::TranslationZ),
            4 => Some(Dof::RotationX),
            5 => Some(Dof::RotationY),
            6 => Some(Dof::RotationZ),
            11 => Some(Dof::Temperature),
            _ => None,
        }
    }
}

/// Face definition for surface loads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Face {
    pub name: String,
    pub node_ids: Vec<usize>,
}

/// Boundary condition type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BcType {
    /// Fixed constraint - all DOFs constrained
    Fixed,
    /// Symmetry constraint
    Symmetry,
    /// Antisymmetry constraint  
    Antisymmetry,
    /// Encastre (fully fixed)
    Encastre,
    /// Pinned (translations fixed, rotations free)
    Pinned,
    /// Roller (one direction free)
    RollerX,
    RollerY,
    RollerZ,
    /// Custom DOF constraints
    Custom(Vec<Dof>),
}

impl BcType {
    /// Get the DOFs constrained by this type
    pub fn get_constrained_dofs(&self) -> Vec<Dof> {
        match self {
            BcType::Fixed | BcType::Encastre => vec![
                Dof::TranslationX, Dof::TranslationY, Dof::TranslationZ,
                Dof::RotationX, Dof::RotationY, Dof::RotationZ,
            ],
            BcType::Symmetry | BcType::Antisymmetry => vec![
                Dof::TranslationY, Dof::TranslationZ,
                Dof::RotationX,
            ],
            BcType::Pinned => vec![
                Dof::TranslationX, Dof::TranslationY, Dof::TranslationZ,
            ],
            BcType::RollerX => vec![
                Dof::TranslationY, Dof::TranslationZ,
            ],
            BcType::RollerY => vec![
                Dof::TranslationX, Dof::TranslationZ,
            ],
            BcType::RollerZ => vec![
                Dof::TranslationX, Dof::TranslationY,
            ],
            BcType::Custom(dofs) => dofs.clone(),
        }
    }
}

/// A fixed boundary condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedBc {
    pub name: String,
    pub nodes: Vec<usize>,
    pub bc_type: BcType,
    pub surfaces: Option<Vec<Face>>,
}

impl FixedBc {
    /// Create a new fixed constraint
    pub fn new(name: &str, nodes: Vec<usize>) -> Self {
        Self {
            name: name.to_string(),
            nodes,
            bc_type: BcType::Fixed,
            surfaces: None,
        }
    }
    
    /// Create an encastre (fully fixed) constraint
    pub fn encastre(name: &str, nodes: Vec<usize>) -> Self {
        Self {
            name: name.to_string(),
            nodes,
            bc_type: BcType::Encastre,
            surfaces: None,
        }
    }
    
    /// Create a pinned constraint
    pub fn pinned(name: &str, nodes: Vec<usize>) -> Self {
        Self {
            name: name.to_string(),
            nodes,
            bc_type: BcType::Pinned,
            surfaces: None,
        }
    }
    
    /// Create a roller constraint (X direction free)
    pub fn roller_x(name: &str, nodes: Vec<usize>) -> Self {
        Self {
            name: name.to_string(),
            nodes,
            bc_type: BcType::RollerX,
            surfaces: None,
        }
    }
    
    /// Create a roller constraint (Y direction free)
    pub fn roller_y(name: &str, nodes: Vec<usize>) -> Self {
        Self {
            name: name.to_string(),
            nodes,
            bc_type: BcType::RollerY,
            surfaces: None,
        }
    }
    
    /// Create a roller constraint (Z direction free)
    pub fn roller_z(name: &str, nodes: Vec<usize>) -> Self {
        Self {
            name: name.to_string(),
            nodes,
            bc_type: BcType::RollerZ,
            surfaces: None,
        }
    }
    
    /// Create a symmetry constraint
    pub fn symmetry(name: &str, nodes: Vec<usize>) -> Self {
        Self {
            name: name.to_string(),
            nodes,
            bc_type: BcType::Symmetry,
            surfaces: None,
        }
    }
    
    /// Create with custom DOF constraints
    pub fn custom(name: &str, nodes: Vec<usize>, dofs: Vec<Dof>) -> Self {
        Self {
            name: name.to_string(),
            nodes,
            bc_type: BcType::Custom(dofs),
            surfaces: None,
        }
    }
    
    /// Add surface definitions for surface-based constraints
    pub fn with_surfaces(mut self, surfaces: Vec<Face>) -> Self {
        self.surfaces = Some(surfaces);
        self
    }
    
    /// Generate INP format output
    pub fn to_inp(&self) -> String {
        let mut output = String::new();
        let dofs = self.bc_type.get_constrained_dofs();
        let dof_str = dofs.iter()
            .map(|d| d.as_int().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        
        output.push_str("*BOUNDARY\n");
        for node_id in &self.nodes {
            output.push_str(&format!("{}, {}, {}\n", node_id, dof_str, 0));
        }
        output.push('\n');
        
        output
    }
}

/// A point load (集中力载荷)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointLoad {
    pub name: String,
    pub node: usize,
    pub magnitude: f64,
    pub direction: LoadDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoadDirection {
    X,
    Y,
    Z,
    Normal,
    Custom(f64, f64, f64),  // Custom vector (fx, fy, fz)
}

impl PointLoad {
    /// Create a new point load
    pub fn new(name: &str, node: usize, magnitude: f64, direction: LoadDirection) -> Self {
        Self {
            name: name.to_string(),
            node,
            magnitude,
            direction,
        }
    }
    
    /// Create a load in X direction
    pub fn x(name: &str, node: usize, magnitude: f64) -> Self {
        Self::new(name, node, magnitude, LoadDirection::X)
    }
    
    /// Create a load in Y direction
    pub fn y(name: &str, node: usize, magnitude: f64) -> Self {
        Self::new(name, node, magnitude, LoadDirection::Y)
    }
    
    /// Create a load in Z direction
    pub fn z(name: &str, node: usize, magnitude: f64) -> Self {
        Self::new(name, node, magnitude, LoadDirection::Z)
    }
    
    /// Create a load in normal direction (for surface loads)
    pub fn normal(name: &str, node: usize, magnitude: f64) -> Self {
        Self::new(name, node, magnitude, LoadDirection::Normal)
    }
    
    /// Create a load with custom vector
    pub fn vector(name: &str, node: usize, fx: f64, fy: f64, fz: f64) -> Self {
        Self::new(name, node, 0.0, LoadDirection::Custom(fx, fy, fz))
    }
    
    /// Generate INP format output (CLOAD)
    pub fn to_inp(&self) -> String {
        let mut output = String::new();
        
        match &self.direction {
            LoadDirection::X => {
                output.push_str("*CLOAD\n");
                output.push_str(&format!("{}, 1, {:.6}\n", self.node, self.magnitude));
            }
            LoadDirection::Y => {
                output.push_str("*CLOAD\n");
                output.push_str(&format!("{}, 2, {:.6}\n", self.node, self.magnitude));
            }
            LoadDirection::Z => {
                output.push_str("*CLOAD\n");
                output.push_str(&format!("{}, 3, {:.6}\n", self.node, self.magnitude));
            }
            LoadDirection::Normal => {
                output.push_str("*CLOAD\n");
                output.push_str(&format!("{}, 3, {:.6}\n", self.node, self.magnitude));
            }
            LoadDirection::Custom(fx, fy, fz) => {
                output.push_str("*CLOAD\n");
                output.push_str(&format!("{}, 1, {:.6}\n", self.node, fx * self.magnitude));
                output.push_str(&format!("{}, 2, {:.6}\n", self.node, fy * self.magnitude));
                output.push_str(&format!("{}, 3, {:.6}\n", self.node, fz * self.magnitude));
            }
        }
        
        output.push('\n');
        output
    }
}

/// A uniform load (均布载荷) applied to a surface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniformLoad {
    pub name: String,
    pub surface_name: String,
    pub element_faces: Vec<(usize, Vec<usize>)>,  // (element_id, face_node_ids)
    pub magnitude: f64,
    pub load_type: UniformLoadType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UniformLoadType {
    /// Pressure load (normal to surface)
    Pressure,
    /// Traction in X direction
    TractionX,
    /// Traction in Y direction
    TractionY,
    /// Traction in Z direction
    TractionZ,
    /// Heat flux
    HeatFlux,
    /// Film coefficient (for convection)
    Film,
}

impl UniformLoad {
    /// Create a new uniform pressure load
    pub fn pressure(name: &str, surface_name: String, magnitude: f64) -> Self {
        Self {
            name: name.to_string(),
            surface_name,
            element_faces: Vec::new(),
            magnitude,
            load_type: UniformLoadType::Pressure,
        }
    }
    
    /// Create a new traction load
    pub fn traction(name: &str, surface_name: String, direction: LoadDirection, magnitude: f64) -> Self {
        let load_type = match direction {
            LoadDirection::X => UniformLoadType::TractionX,
            LoadDirection::Y => UniformLoadType::TractionY,
            LoadDirection::Z => UniformLoadType::TractionZ,
            _ => UniformLoadType::Pressure,
        };
        
        Self {
            name: name.to_string(),
            surface_name,
            element_faces: Vec::new(),
            magnitude,
            load_type,
        }
    }
    
    /// Create a heat flux load
    pub fn heat_flux(name: &str, surface_name: String, magnitude: f64) -> Self {
        Self {
            name: name.to_string(),
            surface_name,
            element_faces: Vec::new(),
            magnitude,
            load_type: UniformLoadType::HeatFlux,
        }
    }
    
    /// Create a convection load
    pub fn film(name: &str, surface_name: String, film_coeff: f64, ambient_temp: f64) -> Self {
        Self {
            name: name.to_string(),
            surface_name,
            element_faces: Vec::new(),
            magnitude: film_coeff,
            load_type: UniformLoadType::Film,
        }
    }
    
    /// Add element faces to this load
    pub fn with_faces(mut self, faces: Vec<(usize, Vec<usize>)>) -> Self {
        self.element_faces = faces;
        self
    }
    
    /// Generate INP format output (DLOAD)
    pub fn to_inp(&self) -> String {
        let mut output = String::new();
        
        match self.load_type {
            UniformLoadType::Pressure => {
                output.push_str(&format!("*DLOAD\n"));
                output.push_str(&format!("{}, P, {:.6}\n", self.surface_name, self.magnitude));
            }
            UniformLoadType::TractionX => {
                output.push_str("*DLOAD\n");
                output.push_str(&format!("{}, TR1, {:.6}\n", self.surface_name, self.magnitude));
            }
            UniformLoadType::TractionY => {
                output.push_str("*DLOAD\n");
                output.push_str(&format!("{}, TR2, {:.6}\n", self.surface_name, self.magnitude));
            }
            UniformLoadType::TractionZ => {
                output.push_str("*DLOAD\n");
                output.push_str(&format!("{}, TR3, {:.6}\n", self.surface_name, self.magnitude));
            }
            UniformLoadType::HeatFlux => {
                output.push_str("*CFLUX\n");
                for (_, node_ids) in &self.element_faces {
                    for node_id in node_ids {
                        output.push_str(&format!("{}, 0, {:.6}\n", node_id, self.magnitude));
                    }
                }
            }
            UniformLoadType::Film => {
                // Film coefficient needs amplitude
                output.push_str("*FILM\n");
                output.push_str(&format!("{}, F, {:.6}\n", self.surface_name, self.magnitude));
            }
        }
        
        output.push('\n');
        output
    }
}

/// Container for all boundary conditions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BcContainer {
    pub fixed_bcs: Vec<FixedBc>,
    pub point_loads: Vec<PointLoad>,
    pub uniform_loads: Vec<UniformLoad>,
}

impl BcContainer {
    /// Create a new empty container
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a fixed boundary condition
    pub fn add_fixed(&mut self, bc: FixedBc) -> &mut Self {
        self.fixed_bcs.push(bc);
        self
    }
    
    /// Add a point load
    pub fn add_point_load(&mut self, load: PointLoad) -> &mut Self {
        self.point_loads.push(load);
        self
    }
    
    /// Add a uniform load
    pub fn add_uniform_load(&mut self, load: UniformLoad) -> &mut Self {
        self.uniform_loads.push(load);
        self
    }
    
    /// Get all fixed BCs
    pub fn get_fixed_bcs(&self) -> &[FixedBc] {
        &self.fixed_bcs
    }
    
    /// Get all point loads
    pub fn get_point_loads(&self) -> &[PointLoad] {
        &self.point_loads
    }
    
    /// Get all uniform loads
    pub fn get_uniform_loads(&self) -> &[UniformLoad] {
        &self.uniform_loads
    }
    
    /// Generate combined INP output
    pub fn to_inp(&self) -> String {
        let mut output = String::new();
        
        // Fixed boundary conditions
        for bc in &self.fixed_bcs {
            output.push_str(&bc.to_inp());
        }
        
        // Point loads
        for load in &self.point_loads {
            output.push_str(&load.to_inp());
        }
        
        // Uniform loads
        for load in &self.uniform_loads {
            output.push_str(&load.to_inp());
        }
        
        output
    }
    
    /// Clear all boundary conditions
    pub fn clear(&mut self) {
        self.fixed_bcs.clear();
        self.point_loads.clear();
        self.uniform_loads.clear();
    }
}

/// Helper trait for applying BCs to mesh entities
pub trait MeshBcApplicator {
    /// Apply fixed constraint to nodes at a specific face
    fn apply_fixed_face(&self, face: &Face, bc_type: BcType) -> Result<FixedBc, BcError>;
    
    /// Apply fixed constraint to nodes in a coordinate range
    fn apply_fixed_region(&self, x_min: f64, x_max: f64, y_min: f64, y_max: f64, z_min: f64, z_max: f64) -> Result<FixedBc, BcError>;
}

/// Helper functions for BC creation
pub mod helpers {
    use super::*;
    
    /// Create a fixed constraint on the left face of a rectangular mesh
    pub fn fixed_left_face(nodes: &[Node], x_value: f64, tolerance: f64) -> FixedBc {
        let constrained_nodes: Vec<usize> = nodes
            .iter()
            .filter(|n| (n.x - x_value).abs() < tolerance)
            .map(|n| n.id)
            .collect();
        
        FixedBc::new("Fixed-Left", constrained_nodes)
    }
    
    /// Create a fixed constraint on the right face of a rectangular mesh
    pub fn fixed_right_face(nodes: &[Node], x_value: f64, tolerance: f64) -> FixedBc {
        let constrained_nodes: Vec<usize> = nodes
            .iter()
            .filter(|n| (n.x - x_value).abs() < tolerance)
            .map(|n| n.id)
            .collect();
        
        FixedBc::new("Fixed-Right", constrained_nodes)
    }
    
    /// Create a fixed constraint on the bottom face (y=0) of a mesh
    pub fn fixed_bottom_face(nodes: &[Node]) -> FixedBc {
        let constrained_nodes: Vec<usize> = nodes
            .iter()
            .filter(|n| n.y.abs() < 1e-6)
            .map(|n| n.id)
            .collect();
        
        FixedBc::new("Fixed-Bottom", constrained_nodes)
    }
    
    /// Create a point load on a specific node
    pub fn point_load_on_node(node_id: usize, magnitude: f64, direction: LoadDirection) -> PointLoad {
        PointLoad::new("Point-Load", node_id, magnitude, direction)
    }
    
    /// Create pressure load on a face
    pub fn pressure_on_face(surface_name: String, magnitude: f64) -> UniformLoad {
        UniformLoad::pressure("Pressure-Load", surface_name, magnitude)
    }
    
    /// Create a gravity load (body force)
    pub fn gravity_load(magnitude: f64, direction: LoadDirection) -> String {
        let mut output = String::new();
        let (dof, sign) = match direction {
            LoadDirection::Y => (2, -1.0),
            LoadDirection::Z => (3, -1.0),
            _ => (2, -1.0),
        };
        
        output.push_str("*DLOAD\n");
        output.push_str(&format!("EALL, GRAV, {:.6}, {}, {:.6}\n", 
            magnitude, dof, sign));
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_bc_creation() {
        let bc = FixedBc::new("Test-Fixed", vec![1, 2, 3]);
        assert_eq!(bc.name, "Test-Fixed");
        assert_eq!(bc.nodes.len(), 3);
        assert_eq!(bc.bc_type, BcType::Fixed);
    }

    #[test]
    fn test_fixed_bc_types() {
        let encastre = FixedBc::encastre("Encastre", vec![1, 2]);
        assert_eq!(encastre.bc_type, BcType::Encastre);
        
        let pinned = FixedBc::pinned("Pinned", vec![3, 4]);
        assert_eq!(pinned.bc_type, BcType::Pinned);
        
        let roller = FixedBc::roller_z("RollerZ", vec![5, 6]);
        assert_eq!(roller.bc_type, BcType::RollerZ);
    }

    #[test]
    fn test_point_load_creation() {
        let load = PointLoad::x("Load-X", 10, 1000.0);
        assert_eq!(load.name, "Load-X");
        assert_eq!(load.node, 10);
        assert_eq!(load.magnitude, 1000.0);
    }

    #[test]
    fn test_uniform_load_creation() {
        let load = UniformLoad::pressure("Pressure", "S1".to_string(), 0.1);
        assert_eq!(load.name, "Pressure");
        assert_eq!(load.magnitude, 0.1);
    }

    #[test]
    fn test_bc_container() {
        let mut container = BcContainer::new();
        
        container.add_fixed(FixedBc::new("BC1", vec![1, 2]));
        container.add_point_load(PointLoad::z("Load1", 10, 500.0));
        container.add_uniform_load(UniformLoad::pressure("P1", "S1".to_string(), 0.05));
        
        assert_eq!(container.fixed_bcs.len(), 1);
        assert_eq!(container.point_loads.len(), 1);
        assert_eq!(container.uniform_loads.len(), 1);
    }

    #[test]
    fn test_dof() {
        assert_eq!(Dof::TranslationX.as_int(), 1);
        assert_eq!(Dof::TranslationY.as_int(), 2);
        assert_eq!(Dof::TranslationZ.as_int(), 3);
        assert_eq!(Dof::Temperature.as_int(), 11);
        
        assert_eq!(Dof::from_int(1), Some(Dof::TranslationX));
        assert_eq!(Dof::from_int(99), None);
    }
}