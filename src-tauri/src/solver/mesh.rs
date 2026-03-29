//! Structured Mesh Generation Module
//! Generates 2D/3D structured meshes for CAE analysis

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Invalid dimension: {0}")]
    InvalidDimension(String),
    #[error("Invalid grid size: nx={nx}, ny={ny}, nz={nz}")]
    InvalidGridSize { nx: usize, ny: usize, nz: usize },
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Represents a 2D or 3D structured mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredMesh {
    /// Mesh dimensions: (nx, ny, nz) - nz=1 for 2D
    pub dimensions: (usize, usize, usize),
    /// Physical size: (length_x, length_y, length_z) - z=0 for 2D
    pub size: (f64, f64, f64),
    /// Node coordinates [node_id][x, y, z]
    pub nodes: Vec<Vec<f64>>,
    /// Element connectivity [element_id][node1, node2, ...]
    pub elements: Vec<Vec<usize>>,
    /// Element type string (C3D8, C3D20, S4, etc.)
    pub element_type: String,
    /// Total number of nodes
    pub num_nodes: usize,
    /// Total number of elements
    pub num_elements: usize,
}

impl StructuredMesh {
    /// Create a new structured mesh
    pub fn new(dimensions: (usize, usize, usize), size: (f64, f64, f64)) -> Self {
        Self {
            dimensions,
            size,
            nodes: Vec::new(),
            elements: Vec::new(),
            element_type: String::new(),
            num_nodes: 0,
            num_elements: 0,
        }
    }

    /// Get node coordinates by ID
    pub fn get_node(&self, id: usize) -> Option<&[f64]> {
        if id < self.nodes.len() {
            Some(&self.nodes[id])
        } else {
            None
        }
    }

    /// Get element connectivity by ID
    pub fn get_element(&self, id: usize) -> Option<&[usize]> {
        if id < self.elements.len() {
            Some(&self.elements[id])
        } else {
            None
        }
    }
}

/// Grid size configuration for mesh generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridConfig {
    /// Number of elements in X direction
    pub nx: usize,
    /// Number of elements in Y direction
    pub ny: usize,
    /// Number of elements in Z direction (1 for 2D mesh)
    pub nz: usize,
    /// Physical size in X direction
    pub size_x: f64,
    /// Physical size in Y direction
    pub size_y: f64,
    /// Physical size in Z direction (0 for 2D mesh)
    pub size_z: f64,
    /// Element type to generate
    pub element_type: MeshElementType,
}

impl GridConfig {
    /// Create a 2D grid configuration
    pub fn new_2d(nx: usize, ny: usize, size_x: f64, size_y: f64) -> Self {
        Self {
            nx,
            ny,
            nz: 1,
            size_x,
            size_y,
            size_z: 0.0,
            element_type: MeshElementType::Quad4,
        }
    }

    /// Create a 3D grid configuration
    pub fn new_3d(nx: usize, ny: usize, nz: usize, size_x: f64, size_y: f64, size_z: f64) -> Self {
        Self {
            nx,
            ny,
            nz,
            size_x,
            size_y,
            size_z,
            element_type: MeshElementType::Hex8,
        }
    }
}

/// Supported element types for structured mesh
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MeshElementType {
    // 2D Elements
    Quad4,      // 4-node quadrilateral (Q4)
    Quad8,      // 8-node quadrilateral (Q8)
    Tri3,       // 3-node triangle (T3)
    Tri6,       // 6-node triangle (T6)
    
    // 3D Elements
    Hex8,       // 8-node brick (C3D8)
    Hex20,      // 20-node brick (C3D20)
    Hex27,      // 27-node brick (C3D27)
    Tet4,       // 4-node tetrahedron (C3D4)
    Tet10,      // 10-node tetrahedron (C3D10)
    Wedge6,     // 6-node wedge (C3D6)
    Wedge15,    // 15-node wedge (C3D15)
}

impl MeshElementType {
    pub fn as_str(&self) -> &str {
        match self {
            MeshElementType::Quad4 => "S4",
            MeshElementType::Quad8 => "S8",
            MeshElementType::Tri3 => "S3",
            MeshElementType::Tri6 => "S6",
            MeshElementType::Hex8 => "C3D8",
            MeshElementType::Hex20 => "C3D20",
            MeshElementType::Hex27 => "C3D27",
            MeshElementType::Tet4 => "C3D4",
            MeshElementType::Tet10 => "C3D10",
            MeshElementType::Wedge6 => "C3D6",
            MeshElementType::Wedge15 => "C3D15",
        }
    }
}

/// Structured mesh generator
pub struct MeshGenerator {
    config: GridConfig,
}

impl MeshGenerator {
    /// Create a new mesh generator with the given configuration
    pub fn new(config: GridConfig) -> Self {
        Self { config }
    }

    /// Generate a 2D rectangular mesh
    pub fn generate_2d_rect(&self) -> Result<StructuredMesh, MeshError> {
        let (nx, ny) = (self.config.nx, self.config.ny);
        let (size_x, size_y) = (self.config.size_x, self.config.size_y);

        if nx == 0 || ny == 0 {
            return Err(MeshError::InvalidGridSize { nx, ny, nz: 1 });
        }

        // Calculate number of nodes
        let num_nodes_x = nx + 1;
        let num_nodes_y = ny + 1;
        let num_nodes = num_nodes_x * num_nodes_y;

        // Calculate element spacing
        let dx = size_x / nx as f64;
        let dy = size_y / ny as f64;

        // Generate nodes
        let mut nodes = Vec::with_capacity(num_nodes);
        for j in 0..num_nodes_y {
            for i in 0..num_nodes_x {
                let x = i as f64 * dx;
                let y = j as f64 * dy;
                nodes.push(vec![x, y, 0.0]);
            }
        }

        // Generate elements (Quad4)
        let mut elements = Vec::with_capacity(nx * ny);
        let mut elem_id = 1;
        for j in 0..ny {
            for i in 0..nx {
                // Node indices for this element
                let n0 = j * num_nodes_x + i;
                let n1 = n0 + 1;
                let n2 = n0 + num_nodes_x + 1;
                let n3 = n0 + num_nodes_x;
                
                elements.push(vec![n0, n1, n2, n3]);
                elem_id += 1;
            }
        }

        let mut mesh = StructuredMesh {
            dimensions: (nx, ny, 1),
            size: (size_x, size_y, 0.0),
            nodes,
            elements,
            element_type: self.config.element_type.as_str().to_string(),
            num_nodes,
            num_elements: nx * ny,
        };

        // Set element type based on config
        mesh.element_type = self.config.element_type.as_str().to_string();

        Ok(mesh)
    }

    /// Generate a 3D rectangular box mesh
    pub fn generate_3d_box(&self) -> Result<StructuredMesh, MeshError> {
        let (nx, ny, nz) = (self.config.nx, self.config.ny, self.config.nz);
        let (size_x, size_y, size_z) = (self.config.size_x, self.config.size_y, self.config.size_z);

        if nx == 0 || ny == 0 || nz == 0 {
            return Err(MeshError::InvalidGridSize { nx, ny, nz });
        }

        // Calculate number of nodes
        let num_nodes_x = nx + 1;
        let num_nodes_y = ny + 1;
        let num_nodes_z = nz + 1;
        let num_nodes = num_nodes_x * num_nodes_y * num_nodes_z;

        // Calculate element spacing
        let dx = size_x / nx as f64;
        let dy = size_y / ny as f64;
        let dz = size_z / nz as f64;

        // Generate nodes (using offset ordering for better cache performance)
        let mut nodes = Vec::with_capacity(num_nodes);
        for k in 0..num_nodes_z {
            for j in 0..num_nodes_y {
                for i in 0..num_nodes_x {
                    let x = i as f64 * dx;
                    let y = j as f64 * dy;
                    let z = k as f64 * dz;
                    nodes.push(vec![x, y, z]);
                }
            }
        }

        // Generate elements (Hex8)
        let mut elements = Vec::with_capacity(nx * ny * nz);
        let mut elem_id = 1;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    // Node indices using the same ordering as node generation
                    let n000 = k * num_nodes_x * num_nodes_y + j * num_nodes_x + i;
                    let n100 = n000 + 1;
                    let n010 = n000 + num_nodes_x;
                    let n110 = n010 + 1;
                    let n001 = n000 + num_nodes_x * num_nodes_y;
                    let n101 = n001 + 1;
                    let n011 = n001 + num_nodes_x;
                    let n111 = n011 + 1;
                    
                    elements.push(vec![n000, n100, n110, n010, n001, n101, n111, n011]);
                    elem_id += 1;
                }
            }
        }

        let mut mesh = StructuredMesh {
            dimensions: (nx, ny, nz),
            size: (size_x, size_y, size_z),
            nodes,
            elements,
            element_type: self.config.element_type.as_str().to_string(),
            num_nodes,
            num_elements: nx * ny * nz,
        };

        // Set element type based on config
        mesh.element_type = self.config.element_type.as_str().to_string();

        Ok(mesh)
    }

    /// Generate mesh based on dimension
    pub fn generate(&self) -> Result<StructuredMesh, MeshError> {
        if self.config.nz == 1 {
            self.generate_2d_rect()
        } else {
            self.generate_3d_box()
        }
    }

    /// Get the generated mesh as INP-compatible format
    pub fn to_inp_format(&self, mesh: &StructuredMesh) -> String {
        let mut output = String::new();
        
        // Write nodes
        output.push_str("*NODE\n");
        for (i, node) in mesh.nodes.iter().enumerate() {
            output.push_str(&format!("{}, {:.6}, {:.6}, {:.6}\n", i + 1, node[0], node[1], node[2]));
        }
        output.push('\n');
        
        // Write elements
        output.push_str(&format!("*ELEMENT, TYPE={}\n", mesh.element_type));
        for (i, elem) in mesh.elements.iter().enumerate() {
            let node_list = elem.iter().map(|n| n.to_string() + 1).collect::<Vec<_>>().join(", ");
            output.push_str(&format!("{}, {}\n", i + 1, node_list));
        }
        output.push('\n');
        
        output
    }
}

/// Convert structured mesh to CAE input generator format
pub fn mesh_to_nodes_elements(mesh: &StructuredMesh) -> (Vec<crate::commands::input_gen::Node>, Vec<crate::commands::input_gen::Element>) {
    use crate::commands::input_gen::{Node as INode, Element as IElement, ElementType};
    
    let mut nodes = Vec::new();
    let mut elements = Vec::new();
    
    // Convert nodes
    for (i, coord) in mesh.nodes.iter().enumerate() {
        nodes.push(INode {
            id: i + 1,
            x: coord[0],
            y: coord[1],
            z: coord[2],
        });
    }
    
    // Convert elements
    let elem_type = match mesh.element_type.as_str() {
        "C3D8" => ElementType::C3D8,
        "C3D20" => ElementType::C3D20,
        "C3D4" => ElementType::C3D4,
        "S4" => ElementType::S4,
        "S4R" => ElementType::S4R,
        "B31" => ElementType::B31,
        _ => ElementType::C3D8,
    };
    
    for (i, elem_nodes) in mesh.elements.iter().enumerate() {
        elements.push(IElement {
            id: i + 1,
            element_type: elem_type.clone(),
            nodes: elem_nodes.iter().map(|n| n + 1).collect(),
        });
    }
    
    (nodes, elements)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2d_mesh_generation() {
        let config = GridConfig::new_2d(2, 2, 1.0, 1.0);
        let generator = MeshGenerator::new(config);
        let mesh = generator.generate().unwrap();
        
        assert_eq!(mesh.num_nodes, 9);  // 3x3 nodes
        assert_eq!(mesh.num_elements, 4); // 2x2 elements
        assert_eq!(mesh.dimensions, (2, 2, 1));
    }

    #[test]
    fn test_3d_mesh_generation() {
        let config = GridConfig::new_3d(2, 2, 2, 1.0, 1.0, 1.0);
        let generator = MeshGenerator::new(config);
        let mesh = generator.generate().unwrap();
        
        assert_eq!(mesh.num_nodes, 27);  // 3x3x3 nodes
        assert_eq!(mesh.num_elements, 8); // 2x2x2 elements
        assert_eq!(mesh.dimensions, (2, 2, 2));
    }

    #[test]
    fn test_element_type_string() {
        assert_eq!(MeshElementType::Hex8.as_str(), "C3D8");
        assert_eq!(MeshElementType::Quad4.as_str(), "S4");
    }
}