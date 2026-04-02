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
    #[error("Invalid refinement config: {0}")]
    InvalidRefinement(String),
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

/// 局部加密区域类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefinementRegionType {
    /// 按边选择（指定坐标轴+范围，加密该边附近的网格）
    Edge,
    /// 按面选择（指定坐标面+范围，加密该面附近的网格）
    Face,
    /// 按体选择（指定区域范围，加密该体积内的网格）
    Volume,
}

/// 局部加密配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefinementConfig {
    /// 加密区域类型
    pub region_type: RefinementRegionType,
    /// 加密方向（用于边/面选择）
    pub axis: Option<char>, // 'x', 'y', 'z'
    /// 加密区域最小坐标
    pub min_coord: Option<f64>,
    /// 加密区域最大坐标
    pub max_coord: Option<f64>,
    /// 加密比例（1.5 ~ 4.0，表示该区域网格密度为全局的多少倍）
    pub refinement_ratio: f64,
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
        let mut _elem_id = 1;
        for j in 0..ny {
            for i in 0..nx {
                // Node indices for this element
                let n0 = j * num_nodes_x + i;
                let n1 = n0 + 1;
                let n2 = n0 + num_nodes_x + 1;
                let n3 = n0 + num_nodes_x;

                elements.push(vec![n0, n1, n2, n3]);
                _elem_id += 1;
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
        let mut _elem_id = 1;
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
                    _elem_id += 1;
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

    // ========================================================================
    // 局部网格加密方法
    // ========================================================================

    /// 生成带局部加密的 2D 矩形网格
    ///
    /// 支持按边（上下左右）和按区域加密。
    /// 使用双曲正切函数实现平滑过渡的非均匀间距。
    pub fn generate_2d_rect_with_refinement(
        &self,
        refinements: &[RefinementConfig],
    ) -> Result<StructuredMesh, MeshError> {
        let (nx, ny) = (self.config.nx, self.config.ny);
        let (size_x, size_y) = (self.config.size_x, self.config.size_y);

        if nx == 0 || ny == 0 {
            return Err(MeshError::InvalidGridSize { nx, ny, nz: 1 });
        }

        // 验证加密配置
        for (i, ref_cfg) in refinements.iter().enumerate() {
            if ref_cfg.refinement_ratio < 1.0 {
                return Err(MeshError::InvalidRefinement(format!(
                    "refinement_ratio must >= 1.0, got {} at index {}",
                    ref_cfg.refinement_ratio, i
                )));
            }
            if ref_cfg.region_type == RefinementRegionType::Volume {
                return Err(MeshError::InvalidRefinement(format!(
                    "Volume refinement is not supported for 2D mesh at index {}",
                    i
                )));
            }
        }

        // 计算每个方向的有效加密比例
        let ratio_x = compute_effective_ratio(refinements, 'x', size_x);
        let ratio_y = compute_effective_ratio(refinements, 'y', size_y);

        // 生成非均匀节点分布
        let x_coords = generate_nonuniform_coords(0.0, size_x, nx, ratio_x, refinements, 'x');
        let y_coords = generate_nonuniform_coords(0.0, size_y, ny, ratio_y, refinements, 'y');

        let num_nodes_x = x_coords.len();
        let num_nodes_y = y_coords.len();
        let num_nodes = num_nodes_x * num_nodes_y;

        // 生成节点
        let mut nodes = Vec::with_capacity(num_nodes);
        for j in 0..num_nodes_y {
            for i in 0..num_nodes_x {
                nodes.push(vec![x_coords[i], y_coords[j], 0.0]);
            }
        }

        // 生成单元 (Quad4)
        let num_elems_x = num_nodes_x - 1;
        let num_elems_y = num_nodes_y - 1;
        let mut elements = Vec::with_capacity(num_elems_x * num_elems_y);
        for j in 0..num_elems_y {
            for i in 0..num_elems_x {
                let n0 = j * num_nodes_x + i;
                let n1 = n0 + 1;
                let n2 = n0 + num_nodes_x + 1;
                let n3 = n0 + num_nodes_x;
                elements.push(vec![n0, n1, n2, n3]);
            }
        }

        Ok(StructuredMesh {
            dimensions: (num_elems_x, num_elems_y, 1),
            size: (size_x, size_y, 0.0),
            nodes,
            elements,
            element_type: self.config.element_type.as_str().to_string(),
            num_nodes,
            num_elements: num_elems_x * num_elems_y,
        })
    }

    /// 生成带局部加密的 3D 长方体网格
    ///
    /// 支持按面、按边、按体加密。
    /// 使用双曲正切函数实现平滑过渡的非均匀间距。
    pub fn generate_3d_box_with_refinement(
        &self,
        refinements: &[RefinementConfig],
    ) -> Result<StructuredMesh, MeshError> {
        let (nx, ny, nz) = (self.config.nx, self.config.ny, self.config.nz);
        let (size_x, size_y, size_z) = (self.config.size_x, self.config.size_y, self.config.size_z);

        if nx == 0 || ny == 0 || nz == 0 {
            return Err(MeshError::InvalidGridSize { nx, ny, nz });
        }

        // 验证加密配置
        for (i, ref_cfg) in refinements.iter().enumerate() {
            if ref_cfg.refinement_ratio < 1.0 {
                return Err(MeshError::InvalidRefinement(format!(
                    "refinement_ratio must >= 1.0, got {} at index {}",
                    ref_cfg.refinement_ratio, i
                )));
            }
        }

        // 计算每个方向的有效加密比例
        let ratio_x = compute_effective_ratio(refinements, 'x', size_x);
        let ratio_y = compute_effective_ratio(refinements, 'y', size_y);
        let ratio_z = compute_effective_ratio(refinements, 'z', size_z);

        // 生成非均匀节点分布
        let x_coords = generate_nonuniform_coords(0.0, size_x, nx, ratio_x, refinements, 'x');
        let y_coords = generate_nonuniform_coords(0.0, size_y, ny, ratio_y, refinements, 'y');
        let z_coords = generate_nonuniform_coords(0.0, size_z, nz, ratio_z, refinements, 'z');

        let num_nodes_x = x_coords.len();
        let num_nodes_y = y_coords.len();
        let num_nodes_z = z_coords.len();
        let num_nodes = num_nodes_x * num_nodes_y * num_nodes_z;

        // 生成节点
        let mut nodes = Vec::with_capacity(num_nodes);
        for k in 0..num_nodes_z {
            for j in 0..num_nodes_y {
                for i in 0..num_nodes_x {
                    nodes.push(vec![x_coords[i], y_coords[j], z_coords[k]]);
                }
            }
        }

        // 生成单元 (Hex8)
        let num_elems_x = num_nodes_x - 1;
        let num_elems_y = num_nodes_y - 1;
        let num_elems_z = num_nodes_z - 1;
        let mut elements = Vec::with_capacity(num_elems_x * num_elems_y * num_elems_z);
        for k in 0..num_elems_z {
            for j in 0..num_elems_y {
                for i in 0..num_elems_x {
                    let n000 = k * num_nodes_x * num_nodes_y + j * num_nodes_x + i;
                    let n100 = n000 + 1;
                    let n010 = n000 + num_nodes_x;
                    let n110 = n010 + 1;
                    let n001 = n000 + num_nodes_x * num_nodes_y;
                    let n101 = n001 + 1;
                    let n011 = n001 + num_nodes_x;
                    let n111 = n011 + 1;
                    elements.push(vec![n000, n100, n110, n010, n001, n101, n111, n011]);
                }
            }
        }

        Ok(StructuredMesh {
            dimensions: (num_elems_x, num_elems_y, num_elems_z),
            size: (size_x, size_y, size_z),
            nodes,
            elements,
            element_type: self.config.element_type.as_str().to_string(),
            num_nodes,
            num_elements: num_elems_x * num_elems_y * num_elems_z,
        })
    }

    /// 对已有网格进行局部加密（通过细分加密区域内的单元）
    ///
    /// 该方法接收一个已有网格和加密配置，在加密区域内对每个单元进行细分。
    /// 细分比例为 refinement_ratio 的向上取整。
    pub fn refine_existing_mesh(
        mesh: &StructuredMesh,
        refinements: &[RefinementConfig],
    ) -> Result<StructuredMesh, MeshError> {
        if refinements.is_empty() {
            return Ok(mesh.clone());
        }

        // 验证加密配置
        for (i, ref_cfg) in refinements.iter().enumerate() {
            if ref_cfg.refinement_ratio < 1.0 {
                return Err(MeshError::InvalidRefinement(format!(
                    "refinement_ratio must >= 1.0, got {} at index {}",
                    ref_cfg.refinement_ratio, i
                )));
            }
        }

        let is_3d = mesh.dimensions.2 > 1;
        let sub_div = |r: f64| -> usize { (r.ceil()) as usize };

        // 判断每个单元是否在加密区域内
        let mut new_nodes: Vec<Vec<f64>> = Vec::new();
        let mut new_elements: Vec<Vec<usize>> = Vec::new();

        // 旧节点直接复制
        new_nodes.extend(mesh.nodes.iter().cloned());

        for (elem_idx, elem_conn) in mesh.elements.iter().enumerate() {
            // 计算单元中心
            let center = element_center(&mesh.nodes, elem_conn);

            // 检查该单元是否在任一加密区域内
            let max_ratio = refinements
                .iter()
                .filter(|r| is_element_in_refinement_region(center, r, is_3d))
                .map(|r| r.refinement_ratio)
                .fold(1.0, f64::max);

            if max_ratio <= 1.0 {
                // 不需要加密，保留原单元
                new_elements.push(elem_conn.clone());
            } else {
                // 需要细分
                let n = sub_div(max_ratio);
                let subdivided = subdivide_element(
                    &mesh.nodes,
                    elem_conn,
                    elem_idx,
                    n,
                    is_3d,
                    &mut new_nodes,
                );
                new_elements.extend(subdivided);
            }
        }

        Ok(StructuredMesh {
            dimensions: mesh.dimensions, // 细分后 dimensions 不再严格对应
            size: mesh.size,
            num_nodes: new_nodes.len(),
            num_elements: new_elements.len(),
            nodes: new_nodes,
            elements: new_elements,
            element_type: mesh.element_type.clone(),
        })
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
            let node_list = elem.iter().map(|n| (n + 1).to_string()).collect::<Vec<_>>().join(", ");
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

// ============================================================================
// Mesh Quality Checking Module
// ============================================================================

/// 网格质量指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshQualityMetrics {
    /// 总体统计
    pub total_elements: usize,
    pub avg_quality: f64,
    pub min_quality: f64,
    pub max_quality: f64,
    /// 各质量等级的单元数量
    pub excellent_count: usize,  // > 0.8
    pub good_count: usize,       // 0.6 ~ 0.8
    pub fair_count: usize,       // 0.4 ~ 0.6
    pub poor_count: usize,       // 0.2 ~ 0.4
    pub bad_count: usize,        // < 0.2
    /// 每个单元的质量指标
    pub element_qualities: Vec<ElementQuality>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementQuality {
    pub element_id: usize,
    pub aspect_ratio: f64,       // 长宽比 (越接近1越好)
    pub jacobian_ratio: f64,     // 雅可比比 (0~1, 越接近1越好)
    pub skewness: f64,           // 偏斜度 (0~1, 越接近0越好)
    pub warp_angle: f64,         // 翘曲角 (度, 越接近0越好)
    pub overall_quality: f64,    // 综合质量 (0~1, 1为最佳)
}

/// 检查网格质量
pub fn check_mesh_quality(
    nodes: &Vec<Vec<f64>>,
    elements: &Vec<Vec<usize>>,
    element_type: &str,
) -> MeshQualityMetrics {
    let total_elements = elements.len();
    let mut element_qualities = Vec::with_capacity(total_elements);

    for (idx, elem) in elements.iter().enumerate() {
        let eq = compute_element_quality(idx, elem, nodes, element_type);
        element_qualities.push(eq);
    }

    // 汇总统计
    let mut sum_quality = 0.0;
    let mut min_quality = f64::MAX;
    let mut max_quality = f64::MIN;
    let mut excellent_count = 0usize;
    let mut good_count = 0usize;
    let mut fair_count = 0usize;
    let mut poor_count = 0usize;
    let mut bad_count = 0usize;

    for eq in &element_qualities {
        let q = eq.overall_quality;
        sum_quality += q;
        if q < min_quality { min_quality = q; }
        if q > max_quality { max_quality = q; }

        if q > 0.8 {
            excellent_count += 1;
        } else if q >= 0.6 {
            good_count += 1;
        } else if q >= 0.4 {
            fair_count += 1;
        } else if q >= 0.2 {
            poor_count += 1;
        } else {
            bad_count += 1;
        }
    }

    let avg_quality = if total_elements > 0 {
        sum_quality / total_elements as f64
    } else {
        0.0
    };

    if total_elements == 0 {
        min_quality = 0.0;
        max_quality = 0.0;
    }

    MeshQualityMetrics {
        total_elements,
        avg_quality,
        min_quality,
        max_quality,
        excellent_count,
        good_count,
        fair_count,
        poor_count,
        bad_count,
        element_qualities,
    }
}

/// 计算单个单元的质量指标
fn compute_element_quality(
    element_id: usize,
    elem: &[usize],
    nodes: &Vec<Vec<f64>>,
    element_type: &str,
) -> ElementQuality {
    // 获取节点坐标
    let coords: Vec<[f64; 3]> = elem.iter()
        .filter_map(|&nid| {
            if nid < nodes.len() && nodes[nid].len() >= 3 {
                Some([nodes[nid][0], nodes[nid][1], nodes[nid][2]])
            } else {
                None
            }
        })
        .collect();

    if coords.len() < 3 {
        return ElementQuality {
            element_id,
            aspect_ratio: 999.0,
            jacobian_ratio: 0.0,
            skewness: 1.0,
            warp_angle: 90.0,
            overall_quality: 0.0,
        };
    }

    // 判断单元维度
    let is_3d = is_3d_element(element_type);

    // 计算长宽比
    let aspect_ratio = compute_aspect_ratio(&coords);

    // 计算雅可比比
    let jacobian_ratio = if is_3d {
        compute_jacobian_ratio_3d(&coords)
    } else {
        compute_jacobian_ratio_2d(&coords)
    };

    // 计算偏斜度
    let skewness = if is_3d {
        compute_skewness_3d(&coords)
    } else {
        compute_skewness_2d(&coords)
    };

    // 计算翘曲角
    let warp_angle = if is_3d {
        compute_warp_angle_3d(&coords)
    } else {
        compute_warp_angle_2d(&coords)
    };

    // 归一化各指标到 0~1 的质量分数
    let ar_inv: f64 = 1.0 / aspect_ratio;
    let ar_quality: f64 = 1.0 / aspect_ratio.max(ar_inv); // ar=1 => 1.0
    let jac_quality = jacobian_ratio.clamp(0.0, 1.0);
    let skew_quality = (1.0 - skewness).clamp(0.0, 1.0);
    let warp_quality = (1.0 - warp_angle / 90.0).clamp(0.0, 1.0);

    // 综合质量
    let overall_quality = ar_quality * 0.3 + jac_quality * 0.3 + skew_quality * 0.2 + warp_quality * 0.2;

    ElementQuality {
        element_id,
        aspect_ratio,
        jacobian_ratio,
        skewness,
        warp_angle,
        overall_quality: overall_quality.clamp(0.0, 1.0),
    }
}

/// 判断是否为三维单元
fn is_3d_element(element_type: &str) -> bool {
    let et = element_type.to_uppercase();
    et.starts_with("C3D") || et.starts_with("HEX") || et.starts_with("TET") || et.starts_with("WEDGE")
}

/// 计算长宽比：最长边 / 最短边
fn compute_aspect_ratio(coords: &[[f64; 3]]) -> f64 {
    let mut min_len = f64::MAX;
    let mut max_len = f64::MIN;

    let n = coords.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = coords[j][0] - coords[i][0];
            let dy = coords[j][1] - coords[i][1];
            let dz = coords[j][2] - coords[i][2];
            let len = (dx * dx + dy * dy + dz * dz).sqrt();
            if len < min_len { min_len = len; }
            if len > max_len { max_len = len; }
        }
    }

    if min_len < 1e-12 {
        return 999.0;
    }
    max_len / min_len
}

/// 计算二维单元的雅可比比
fn compute_jacobian_ratio_2d(coords: &[[f64; 3]]) -> f64 {
    // 对于四边形，计算4个角点的雅可比行列式
    // J = |dx/dxi  dy/dxi|
    //     |dx/deta dy/deta|
    // 使用有限差分近似
    if coords.len() < 4 {
        return compute_jacobian_ratio_simple(coords);
    }

    // 四边形的4个角点
    let p0 = &coords[0];
    let p1 = &coords[1];
    let p2 = &coords[2];
    let p3 = &coords[3];

    // 在4个角点计算雅可比
    // 角点0 (xi=-1, eta=-1): 使用相邻节点差分
    let j0 = compute_jacobian_at_point_2d(p0, p1, p3);
    // 角点1 (xi=1, eta=-1)
    let j1 = compute_jacobian_at_point_2d(p1, p0, p2);
    // 角点2 (xi=1, eta=1)
    let j2 = compute_jacobian_at_point_2d(p2, p3, p1);
    // 角点3 (xi=-1, eta=1)
    let j3 = compute_jacobian_at_point_2d(p3, p2, p0);

    let jacobians = [j0, j1, j2, j3];
    let min_j = jacobians.iter().cloned().fold(f64::MAX, f64::min);
    let max_j = jacobians.iter().cloned().fold(f64::MIN, f64::max);

    if max_j.abs() < 1e-12 {
        return 0.0;
    }

    (min_j / max_j).abs()
}

fn compute_jacobian_at_point_2d(
    p: &[f64; 3],
    p_xi: &[f64; 3],
    p_eta: &[f64; 3],
) -> f64 {
    let dx_dxi = p_xi[0] - p[0];
    let dy_dxi = p_xi[1] - p[1];
    let dx_deta = p_eta[0] - p[0];
    let dy_deta = p_eta[1] - p[1];
    (dx_dxi * dy_deta - dx_deta * dy_dxi).abs()
}

/// 简单雅可比比计算（用于三角形等）
fn compute_jacobian_ratio_simple(coords: &[[f64; 3]]) -> f64 {
    if coords.len() < 3 {
        return 0.0;
    }
    // 计算所有可能三角形的面积
    let n = coords.len();
    let mut min_area = f64::MAX;
    let mut max_area = f64::MIN;

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                let area = triangle_area_2d(&coords[i], &coords[j], &coords[k]);
                if area < min_area { min_area = area; }
                if area > max_area { max_area = area; }
            }
        }
    }

    if max_area < 1e-12 {
        return 0.0;
    }
    min_area / max_area
}

fn triangle_area_2d(a: &[f64; 3], b: &[f64; 3], c: &[f64; 3]) -> f64 {
    let abx = b[0] - a[0];
    let aby = b[1] - a[1];
    let acx = c[0] - a[0];
    let acy = c[1] - a[1];
    (abx * acy - acx * aby).abs() * 0.5
}

/// 计算三维单元的雅可比比
fn compute_jacobian_ratio_3d(coords: &[[f64; 3]]) -> f64 {
    if coords.len() < 8 {
        return compute_jacobian_ratio_simple(coords);
    }

    // 六面体8节点: 计算各角点的雅可比行列式
    // 在每个角点，使用相邻边构建雅可比矩阵
    let p0 = &coords[0];
    let p1 = &coords[1];
    let p2 = &coords[2];
    let p3 = &coords[3];
    let p4 = &coords[4];
    let p5 = &coords[5];
    let p6 = &coords[6];
    let p7 = &coords[7];

    // 角点0: 边方向 p0->p1 (xi), p0->p3 (eta), p0->p4 (zeta)
    let j0 = jacobian_det_3d(p0, p1, p3, p4);
    // 角点1: 边方向 p1->p0 (xi), p1->p2 (eta), p1->p5 (zeta)
    let j1 = jacobian_det_3d(p1, p0, p2, p5);
    // 角点2: 边方向 p2->p3 (xi), p2->p1 (eta), p2->p6 (zeta)
    let j2 = jacobian_det_3d(p2, p3, p1, p6);
    // 角点3: 边方向 p3->p2 (xi), p3->p0 (eta), p3->p7 (zeta)
    let j3 = jacobian_det_3d(p3, p2, p0, p7);
    // 角点4: 边方向 p4->p5 (xi), p4->p7 (eta), p4->p0 (zeta)
    let j4 = jacobian_det_3d(p4, p5, p7, p0);
    // 角点5: 边方向 p5->p4 (xi), p5->p6 (eta), p5->p1 (zeta)
    let j5 = jacobian_det_3d(p5, p4, p6, p1);
    // 角点6: 边方向 p6->p7 (xi), p6->p5 (eta), p6->p2 (zeta)
    let j6 = jacobian_det_3d(p6, p7, p5, p2);
    // 角点7: 边方向 p7->p6 (xi), p7->p4 (eta), p7->p3 (zeta)
    let j7 = jacobian_det_3d(p7, p6, p4, p3);

    let jacobians = [j0, j1, j2, j3, j4, j5, j6, j7];
    let min_j = jacobians.iter().cloned().fold(f64::MAX, f64::min);
    let max_j = jacobians.iter().cloned().fold(f64::MIN, f64::max);

    if max_j.abs() < 1e-12 {
        return 0.0;
    }

    (min_j / max_j).abs()
}

/// 计算3x3雅可比矩阵的行列式
fn jacobian_det_3d(
    origin: &[f64; 3],
    edge1: &[f64; 3],
    edge2: &[f64; 3],
    edge3: &[f64; 3],
) -> f64 {
    let a1 = edge1[0] - origin[0];
    let a2 = edge1[1] - origin[1];
    let a3 = edge1[2] - origin[2];
    let b1 = edge2[0] - origin[0];
    let b2 = edge2[1] - origin[1];
    let b3 = edge2[2] - origin[2];
    let c1 = edge3[0] - origin[0];
    let c2 = edge3[1] - origin[1];
    let c3 = edge3[2] - origin[2];

    // det = a . (b x c)
    let det = a1 * (b2 * c3 - b3 * c2)
            - a2 * (b1 * c3 - b3 * c1)
            + a3 * (b1 * c2 - b2 * c1);

    det.abs()
}

/// 计算二维单元的偏斜度
fn compute_skewness_2d(coords: &[[f64; 3]]) -> f64 {
    if coords.len() < 3 {
        return 1.0;
    }

    if coords.len() == 3 {
        // 三角形：基于理想角度偏差
        // 理想等边三角形每个角60度
        let angles = triangle_angles(&coords[0], &coords[1], &coords[2]);
        let ideal_angle = 60.0_f64.to_radians();
        let mut max_deviation = 0.0;
        for &angle in &angles {
            let deviation = (angle - ideal_angle).abs();
            if deviation > max_deviation {
                max_deviation = deviation;
            }
        }
        // 归一化到 0~1，最大偏差90度
        (max_deviation / (90.0_f64.to_radians())).min(1.0)
    } else {
        // 四边形：基于对角线夹角偏差
        // 理想矩形对角线互相垂直
        let _center = centroid(coords);
        let _mid01 = midpoint(&coords[0], &coords[1]);
        let _mid23 = midpoint(&coords[2], &coords[3]);
        let _mid12 = midpoint(&coords[1], &coords[2]);
        let _mid30 = midpoint(&coords[3], &coords[0]);

        // 对角线方向
        let d1 = [coords[2][0] - coords[0][0], coords[2][1] - coords[0][1]];
        let d2 = [coords[3][0] - coords[1][0], coords[3][1] - coords[1][1]];

        // 对角线夹角
        let dot = d1[0] * d2[0] + d1[1] * d2[1];
        let len1 = (d1[0] * d1[0] + d1[1] * d1[1]).sqrt();
        let len2 = (d2[0] * d2[0] + d2[1] * d2[1]).sqrt();

        if len1 < 1e-12 || len2 < 1e-12 {
            return 1.0;
        }

        let cos_angle = (dot / (len1 * len2)).clamp(-1.0, 1.0);
        let angle = cos_angle.acos();
        let ideal_angle = 90.0_f64.to_radians();
        let deviation = (angle - ideal_angle).abs();

        // 同时考虑边角度偏差
        let mut max_edge_deviation = 0.0;
        let n = coords.len();
        for i in 0..n {
            let prev = (i + n - 1) % n;
            let next = (i + 1) % n;
            let v1 = [coords[prev][0] - coords[i][0], coords[prev][1] - coords[i][1]];
            let v2 = [coords[next][0] - coords[i][0], coords[next][1] - coords[i][1]];
            let len_v1 = (v1[0] * v1[0] + v1[1] * v1[1]).sqrt();
            let len_v2 = (v2[0] * v2[0] + v2[1] * v2[1]).sqrt();
            if len_v1 > 1e-12 && len_v2 > 1e-12 {
                let cos_a = ((v1[0] * v2[0] + v1[1] * v2[1]) / (len_v1 * len_v2)).clamp(-1.0, 1.0);
                let a = cos_a.acos();
                let dev = (a - ideal_angle).abs();
                if dev > max_edge_deviation {
                    max_edge_deviation = dev;
                }
            }
        }

        let diag_skew: f64 = deviation / (90.0_f64.to_radians());
        let edge_skew: f64 = max_edge_deviation / (90.0_f64.to_radians());

        diag_skew.max(edge_skew).min(1.0)
    }
}

/// 计算三维单元的偏斜度
fn compute_skewness_3d(coords: &[[f64; 3]]) -> f64 {
    if coords.len() < 4 {
        return 1.0;
    }

    // 对于六面体：基于面法向量之间的角度偏差
    // 理想六面体的面法向量互相垂直
    if coords.len() >= 8 {
        // 计算底面和侧面的法向量
        let face_normals = compute_hex_face_normals(coords);

        let mut max_skew = 0.0;
        let ideal_angle = 90.0_f64.to_radians();

        for i in 0..face_normals.len() {
            for j in (i + 1)..face_normals.len() {
                let n1 = &face_normals[i];
                let n2 = &face_normals[j];
                let len1 = (n1[0] * n1[0] + n1[1] * n1[1] + n1[2] * n1[2]).sqrt();
                let len2 = (n2[0] * n2[0] + n2[1] * n2[1] + n2[2] * n2[2]).sqrt();

                if len1 > 1e-12 && len2 > 1e-12 {
                    let dot = n1[0] * n2[0] + n1[1] * n2[1] + n1[2] * n2[2];
                    let cos_angle = (dot / (len1 * len2)).clamp(-1.0, 1.0);
                    let angle = cos_angle.acos();
                    let deviation = (angle - ideal_angle).abs();
                    let skew = deviation / (90.0_f64.to_radians());
                    if skew > max_skew {
                        max_skew = skew;
                    }
                }
            }
        }

        max_skew.min(1.0)
    } else {
        // 四面体：基于面角度偏差
        let mut max_skew = 0.0;
        let ideal_angle = 60.0_f64.to_radians(); // 理想正四面体约70.5度，用60度近似

        let n = coords.len();
        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    let angles = triangle_angles_3d(&coords[i], &coords[j], &coords[k]);
                    for &angle in &angles {
                        let deviation = (angle - ideal_angle).abs();
                        let skew = deviation / (90.0_f64.to_radians());
                        if skew > max_skew {
                            max_skew = skew;
                        }
                    }
                }
            }
        }

        max_skew.min(1.0)
    }
}

/// 计算六面体6个面的法向量
fn compute_hex_face_normals(coords: &[[f64; 3]]) -> Vec<[f64; 3]> {
    // 标准六面体节点顺序: 0-1-2-3 (底面), 4-5-6-7 (顶面)
    let mut normals = Vec::with_capacity(6);

    // 底面 (0,1,2,3)
    normals.push(compute_face_normal(&[coords[0], coords[1], coords[2], coords[3]]));
    // 顶面 (4,5,6,7)
    normals.push(compute_face_normal(&[coords[4], coords[5], coords[6], coords[7]]));
    // 前面 (0,1,5,4)
    normals.push(compute_face_normal(&[coords[0], coords[1], coords[5], coords[4]]));
    // 后面 (2,3,7,6)
    normals.push(compute_face_normal(&[coords[2], coords[3], coords[7], coords[6]]));
    // 左面 (0,3,7,4)
    normals.push(compute_face_normal(&[coords[0], coords[3], coords[7], coords[4]]));
    // 右面 (1,2,6,5)
    normals.push(compute_face_normal(&[coords[1], coords[2], coords[6], coords[5]]));

    normals
}

/// 计算四边形面的法向量
fn compute_face_normal(face: &[[f64; 3]]) -> [f64; 3] {
    if face.len() < 3 {
        return [0.0, 0.0, 0.0];
    }

    let v1 = [face[1][0] - face[0][0], face[1][1] - face[0][1], face[1][2] - face[0][2]];
    let v2 = [face[2][0] - face[0][0], face[2][1] - face[0][1], face[2][2] - face[0][2]];

    let nx = v1[1] * v2[2] - v1[2] * v2[1];
    let ny = v1[2] * v2[0] - v1[0] * v2[2];
    let nz = v1[0] * v2[1] - v1[1] * v2[0];

    let len = (nx * nx + ny * ny + nz * nz).sqrt();
    if len > 1e-12 {
        [nx / len, ny / len, nz / len]
    } else {
        [0.0, 0.0, 0.0]
    }
}

/// 计算二维单元的翘曲角
fn compute_warp_angle_2d(coords: &[[f64; 3]]) -> f64 {
    if coords.len() < 4 {
        // 三角形没有翘曲问题
        return 0.0;
    }

    // 四边形翘曲角：计算4个节点偏离最佳拟合平面的最大角度
    let _center = centroid(coords);

    // 使用前3个节点确定平面
    let v1 = [coords[1][0] - coords[0][0], coords[1][1] - coords[0][1], coords[1][2] - coords[0][2]];
    let v2 = [coords[2][0] - coords[0][0], coords[2][1] - coords[0][1], coords[2][2] - coords[0][2]];

    let normal = [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ];

    let normal_len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
    if normal_len < 1e-12 {
        return 0.0;
    }

    // 计算每个节点到平面的距离
    let mut max_warp = 0.0;
    for coord in coords {
        let dx = coord[0] - coords[0][0];
        let dy = coord[1] - coords[0][1];
        let dz = coord[2] - coords[0][2];
        let dist = (dx * normal[0] + dy * normal[1] + dz * normal[2]).abs() / normal_len;

        // 将距离转换为角度
        let diag = compute_max_diag(coords);
        if diag > 1e-12 {
            let angle = (dist / diag).atan() * 180.0 / std::f64::consts::PI;
            if angle > max_warp {
                max_warp = angle;
            }
        }
    }

    max_warp
}

/// 计算三维单元的翘曲角
fn compute_warp_angle_3d(coords: &[[f64; 3]]) -> f64 {
    if coords.len() < 8 {
        return 0.0;
    }

    // 六面体翘曲角：检查每个四边形面的翘曲
    let faces: Vec<&[[f64; 3]]> = vec![
        &coords[0..4], // 底面
        &coords[4..8], // 顶面
    ];

    // 其他4个侧面
    let side_faces: Vec<Vec<[f64; 3]>> = vec![
        vec![coords[0], coords[1], coords[5], coords[4]],
        vec![coords[2], coords[3], coords[7], coords[6]],
        vec![coords[0], coords[3], coords[7], coords[4]],
        vec![coords[1], coords[2], coords[6], coords[5]],
    ];

    let mut max_warp = 0.0_f64;

    // 检查底面和顶面
    for face in &faces {
        let warp = compute_face_warp(face);
        if warp > max_warp {
            max_warp = warp;
        }
    }

    // 检查侧面
    for face in &side_faces {
        let warp = compute_face_warp(&face);
        if warp > max_warp {
            max_warp = warp;
        }
    }

    max_warp
}

/// 计算单个四边形面的翘曲角
fn compute_face_warp(face: &[[f64; 3]]) -> f64 {
    if face.len() < 4 {
        return 0.0;
    }

    // 使用对角线交点作为参考
    let mid_02 = midpoint(&face[0], &face[2]);
    let mid_13 = midpoint(&face[1], &face[3]);

    // 对角线交点
    let center = [
        (mid_02[0] + mid_13[0]) * 0.5,
        (mid_02[1] + mid_13[1]) * 0.5,
        (mid_02[2] + mid_13[2]) * 0.5,
    ];

    // 使用前3个节点确定最佳拟合平面
    let v1 = [face[1][0] - face[0][0], face[1][1] - face[0][1], face[1][2] - face[0][2]];
    let v2 = [face[2][0] - face[0][0], face[2][1] - face[0][1], face[2][2] - face[0][2]];

    let normal = [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ];

    let normal_len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
    if normal_len < 1e-12 {
        return 0.0;
    }

    // 计算对角线交点偏离平面的距离
    let dx = center[0] - face[0][0];
    let dy = center[1] - face[0][1];
    let dz = center[2] - face[0][2];
    let dist = (dx * normal[0] + dy * normal[1] + dz * normal[2]).abs() / normal_len;

    // 将距离转换为角度
    let diag = ((face[2][0] - face[0][0]).powi(2) + (face[2][1] - face[0][1]).powi(2) + (face[2][2] - face[0][2]).powi(2)).sqrt();
    if diag > 1e-12 {
        let angle = (dist / diag).atan() * 180.0 / std::f64::consts::PI;
        return angle;
    }

    0.0
}

/// 计算三角形内角
fn triangle_angles(a: &[f64; 3], b: &[f64; 3], c: &[f64; 3]) -> [f64; 3] {
    let ab = [b[0] - a[0], b[1] - a[1]];
    let ac = [c[0] - a[0], c[1] - a[1]];
    let ba = [a[0] - b[0], a[1] - b[1]];
    let bc = [c[0] - b[0], c[1] - b[1]];
    let ca = [a[0] - c[0], a[1] - c[1]];
    let cb = [b[0] - c[0], b[1] - c[1]];

    let angle_a = angle_between_2d(&ab, &ac);
    let angle_b = angle_between_2d(&ba, &bc);
    let angle_c = angle_between_2d(&ca, &cb);

    [angle_a, angle_b, angle_c]
}

/// 计算三角形内角（3D版本）
fn triangle_angles_3d(a: &[f64; 3], b: &[f64; 3], c: &[f64; 3]) -> [f64; 3] {
    let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let ba = [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let bc = [c[0] - b[0], c[1] - b[1], c[2] - b[2]];
    let ca = [a[0] - c[0], a[1] - c[1], a[2] - c[2]];
    let cb = [b[0] - c[0], b[1] - c[1], b[2] - c[2]];

    let angle_a = angle_between_3d(&ab, &ac);
    let angle_b = angle_between_3d(&ba, &bc);
    let angle_c = angle_between_3d(&ca, &cb);

    [angle_a, angle_b, angle_c]
}

fn angle_between_2d(v1: &[f64; 2], v2: &[f64; 2]) -> f64 {
    let len1 = (v1[0] * v1[0] + v1[1] * v1[1]).sqrt();
    let len2 = (v2[0] * v2[0] + v2[1] * v2[1]).sqrt();
    if len1 < 1e-12 || len2 < 1e-12 {
        return 0.0;
    }
    let cos = ((v1[0] * v2[0] + v1[1] * v2[1]) / (len1 * len2)).clamp(-1.0, 1.0);
    cos.acos()
}

fn angle_between_3d(v1: &[f64; 3], v2: &[f64; 3]) -> f64 {
    let len1 = (v1[0] * v1[0] + v1[1] * v1[1] + v1[2] * v1[2]).sqrt();
    let len2 = (v2[0] * v2[0] + v2[1] * v2[1] + v2[2] * v2[2]).sqrt();
    if len1 < 1e-12 || len2 < 1e-12 {
        return 0.0;
    }
    let cos = ((v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]) / (len1 * len2)).clamp(-1.0, 1.0);
    cos.acos()
}

fn centroid(coords: &[[f64; 3]]) -> [f64; 3] {
    let n = coords.len() as f64;
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut cz = 0.0;
    for c in coords {
        cx += c[0];
        cy += c[1];
        cz += c[2];
    }
    [cx / n, cy / n, cz / n]
}

fn midpoint(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5, (a[2] + b[2]) * 0.5]
}

fn compute_max_diag(coords: &[[f64; 3]]) -> f64 {
    let mut max_diag = 0.0_f64;
    let n = coords.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = coords[j][0] - coords[i][0];
            let dy = coords[j][1] - coords[i][1];
            let dz = coords[j][2] - coords[i][2];
            let d = (dx * dx + dy * dy + dz * dz).sqrt();
            if d > max_diag {
                max_diag = d;
            }
        }
    }
    max_diag
}

// ============================================================================
// 局部加密辅助函数
// ============================================================================

/// 计算指定方向的最大有效加密比例
///
/// 遍历所有加密配置，找到影响该方向的最大加密比例。
fn compute_effective_ratio(refinements: &[RefinementConfig], axis: char, domain_size: f64) -> f64 {
    let mut max_ratio = 1.0;
    for ref_cfg in refinements {
        let affects_axis = match ref_cfg.region_type {
            RefinementRegionType::Edge => ref_cfg.axis.map_or(false, |a| a == axis),
            RefinementRegionType::Face => {
                // Face 加密影响两个方向：如果 axis 不是 face 所在法线方向，则受影响
                // 例如 XY 面加密影响 X 和 Y 方向
                ref_cfg.axis.map_or(false, |a| a != axis)
            }
            RefinementRegionType::Volume => {
                // Volume 加密影响所有方向
                ref_cfg.min_coord.map_or(false, |min| {
                    ref_cfg.max_coord.map_or(false, |max| {
                        // 检查该方向的加密范围是否有效
                        let range = max - min;
                        range > 0.0 && range < domain_size * 1.5
                    })
                })
            }
        };
        if affects_axis && ref_cfg.refinement_ratio > max_ratio {
            max_ratio = ref_cfg.refinement_ratio;
        }
    }
    max_ratio
}

/// 使用双曲正切函数生成非均匀坐标分布
///
/// 算法说明：
/// - 在加密区域内使用更小的间距（更密集的节点分布）
/// - 在加密区域与正常区域之间使用双曲正切函数实现平滑过渡
/// - 过渡区域宽度约为加密区域宽度的 30%
///
/// 参数：
/// - `start`: 起始坐标
/// - `end`: 结束坐标
/// - `n_elements`: 基础单元数
/// - `max_ratio`: 最大加密比例
/// - `refinements`: 加密配置列表
/// - `axis`: 当前坐标轴 ('x', 'y', 'z')
fn generate_nonuniform_coords(
    start: f64,
    end: f64,
    n_elements: usize,
    max_ratio: f64,
    refinements: &[RefinementConfig],
    axis: char,
) -> Vec<f64> {
    if max_ratio <= 1.0 || refinements.is_empty() {
        // 无加密，使用均匀分布
        return uniform_coords(start, end, n_elements);
    }

    let domain_length = end - start;
    if domain_length <= 0.0 {
        return vec![start];
    }

    // 收集该轴上所有加密区间
    let mut refine_intervals: Vec<(f64, f64, f64)> = Vec::new(); // (min, max, ratio)
    for ref_cfg in refinements {
        let affects = match ref_cfg.region_type {
            RefinementRegionType::Edge => ref_cfg.axis.map_or(false, |a| a == axis),
            RefinementRegionType::Face => ref_cfg.axis.map_or(false, |a| a != axis),
            RefinementRegionType::Volume => true,
        };
        if affects {
            if let (Some(min_c), Some(max_c)) = (ref_cfg.min_coord, ref_cfg.max_coord) {
                if max_c > min_c {
                    refine_intervals.push((min_c, max_c, ref_cfg.refinement_ratio));
                }
            }
        }
    }

    if refine_intervals.is_empty() {
        return uniform_coords(start, end, n_elements);
    }

    // 计算加密区域内需要的额外节点数
    // 基本思路：加密区域内的节点密度 = 基础密度 * ratio
    let base_spacing = domain_length / n_elements as f64;
    let mut total_extra_nodes: usize = 0;

    for &(rmin, rmax, ratio) in &refine_intervals {
        let refine_length = (rmax - rmin).min(domain_length);
        // 加密区域内需要的节点数 = 基础节点数 * ratio
        let base_nodes_in_region = (refine_length / base_spacing).ceil() as usize;
        let refined_nodes_in_region = (base_nodes_in_region as f64 * ratio).ceil() as usize;
        total_extra_nodes += refined_nodes_in_region.saturating_sub(base_nodes_in_region);
    }

    // 总节点数 = 基础节点数 + 额外节点数 + 过渡节点
    let transition_nodes_per_side = 3usize; // 每侧过渡区节点数
    let total_nodes = n_elements + 1 + total_extra_nodes
        + transition_nodes_per_side * 2 * refine_intervals.len();

    // 使用累积分布函数方法生成非均匀坐标
    // 步骤 1: 构建局部间距权重函数
    // 步骤 2: 积分得到累积分布
    // 步骤 3: 均匀采样累积分布得到非均匀坐标

    let n_samples: usize = total_nodes.max(n_elements + 1);
    let n_samples = n_samples.min(n_elements * 10); // 防止节点数过多

    // 构建权重函数：在加密区域权重更高
    let weight_fn = |x: f64| -> f64 {
        let mut w: f64 = 1.0;
        for &(rmin, rmax, ratio) in &refine_intervals {
            let refine_length = rmax - rmin;
            if refine_length <= 0.0 {
                continue;
            }
            // 过渡区域宽度
            let transition_width = refine_length * 0.3;

            if x >= rmin && x <= rmax {
                // 在加密区域内，使用完整比例
                w = w.max(ratio);
            } else if x > rmin - transition_width && x < rmin {
                // 左侧过渡区域，使用双曲正切平滑过渡
                let t = (x - (rmin - transition_width)) / transition_width; // 0..1
                let smooth = 0.5 * (1.0 + (2.0 * std::f64::consts::PI * t - std::f64::consts::PI).tanh());
                w = w.max(1.0 + (ratio - 1.0) * smooth);
            } else if x > rmax && x < rmax + transition_width {
                // 右侧过渡区域，使用双曲正切平滑过渡
                let t = (x - rmax) / transition_width; // 0..1
                let smooth = 0.5 * (1.0 + (std::f64::consts::PI - 2.0 * std::f64::consts::PI * t).tanh());
                w = w.max(1.0 + (ratio - 1.0) * smooth);
            }
        }
        w
    };

    // 数值积分计算累积分布
    let n_integral = 1000;
    let dx = domain_length / n_integral as f64;
    let mut cumulative = vec![0.0; n_integral + 1];
    cumulative[0] = 0.0;
    for i in 1..=n_integral {
        let x_mid = start + (i as f64 - 0.5) * dx;
        cumulative[i] = cumulative[i - 1] + weight_fn(x_mid) * dx;
    }

    let total_integral = cumulative[n_integral];
    if total_integral <= 0.0 {
        return uniform_coords(start, end, n_elements);
    }

    // 均匀采样累积分布
    let mut coords = Vec::with_capacity(n_samples);
    for i in 0..n_samples {
        let target = total_integral * i as f64 / (n_samples - 1) as f64;

        // 在累积分布中二分查找
        let mut lo = 0usize;
        let mut hi = n_integral;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cumulative[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        // 线性插值得到精确坐标
        let x_coord = if lo == 0 {
            start
        } else {
            let frac = (target - cumulative[lo - 1]) / (cumulative[lo] - cumulative[lo - 1]);
            start + (lo as f64 - 1.0 + frac) * dx
        };

        coords.push(x_coord);
    }

    // 确保首尾坐标精确
    if let Some(first) = coords.first_mut() {
        *first = start;
    }
    if let Some(last) = coords.last_mut() {
        *last = end;
    }

    coords
}

/// 生成均匀坐标分布
fn uniform_coords(start: f64, end: f64, n_elements: usize) -> Vec<f64> {
    let n_nodes = n_elements + 1;
    let dx = (end - start) / n_elements as f64;
    (0..n_nodes).map(|i| start + i as f64 * dx).collect()
}

/// 计算单元中心坐标
fn element_center(nodes: &[Vec<f64>], elem_conn: &[usize]) -> [f64; 3] {
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut cz = 0.0;
    let n = elem_conn.len() as f64;
    for &node_id in elem_conn {
        if let Some(node) = nodes.get(node_id) {
            cx += node[0];
            cy += node[1];
            cz += node[2];
        }
    }
    [cx / n, cy / n, cz / n]
}

/// 判断单元中心是否在加密区域内
fn is_element_in_refinement_region(
    center: [f64; 3],
    ref_cfg: &RefinementConfig,
    is_3d: bool,
) -> bool {
    match ref_cfg.region_type {
        RefinementRegionType::Edge => {
            if let Some(axis) = ref_cfg.axis {
                let coord = match axis {
                    'x' => center[0],
                    'y' => center[1],
                    'z' => center[2],
                    _ => return false,
                };
                // 边加密：检查该坐标是否在加密范围内
                if let (Some(min_c), Some(max_c)) = (ref_cfg.min_coord, ref_cfg.max_coord) {
                    return coord >= min_c && coord <= max_c;
                }
            }
            false
        }
        RefinementRegionType::Face => {
            if let Some(axis) = ref_cfg.axis {
                let coord = match axis {
                    'x' => center[0],
                    'y' => center[1],
                    'z' => center[2],
                    _ => return false,
                };
                // 面加密：检查该坐标是否在面的附近
                if let (Some(min_c), Some(max_c)) = (ref_cfg.min_coord, ref_cfg.max_coord) {
                    return coord >= min_c && coord <= max_c;
                }
            }
            false
        }
        RefinementRegionType::Volume => {
            if !is_3d {
                // 2D 网格中 volume 等同于 area
                if let (Some(min_c), Some(max_c)) = (ref_cfg.min_coord, ref_cfg.max_coord) {
                    return center[0] >= min_c && center[0] <= max_c
                        && center[1] >= min_c && center[1] <= max_c;
                }
            } else {
                // 3D 网格中的体加密
                // min_coord 和 max_coord 分别表示体对角线的两端
                if let (Some(min_c), Some(max_c)) = (ref_cfg.min_coord, ref_cfg.max_coord) {
                    // 简化处理：使用 min_coord 作为最小 XYZ，max_coord 作为最大 XYZ
                    // 对于更精确的控制，可以扩展 RefinementConfig
                    return center[0] >= min_c && center[0] <= max_c
                        && center[1] >= min_c && center[1] <= max_c
                        && center[2] >= min_c && center[2] <= max_c;
                }
            }
            false
        }
    }
}

/// 细分一个单元为 n x n (2D) 或 n x n x n (3D) 个子单元
///
/// 返回新创建的子单元连接列表。新节点会追加到 `new_nodes` 中。
fn subdivide_element(
    nodes: &[Vec<f64>],
    elem_conn: &[usize],
    _elem_idx: usize,
    n: usize,
    is_3d: bool,
    new_nodes: &mut Vec<Vec<f64>>,
) -> Vec<Vec<usize>> {
    if n <= 1 {
        return vec![elem_conn.to_vec()];
    }

    if is_3d && elem_conn.len() == 8 {
        // Hex8 单元细分为 n^3 个子单元
        subdivide_hex8(nodes, elem_conn, n, new_nodes)
    } else if !is_3d && elem_conn.len() == 4 {
        // Quad4 单元细分为 n^2 个子单元
        subdivide_quad4(nodes, elem_conn, n, new_nodes)
    } else {
        // 不支持的单元类型，返回原单元
        vec![elem_conn.to_vec()]
    }
}

/// 细分 Quad4 单元
fn subdivide_quad4(
    nodes: &[Vec<f64>],
    elem_conn: &[usize],
    n: usize,
    new_nodes: &mut Vec<Vec<f64>>,
) -> Vec<Vec<usize>> {
    // 获取四个角节点坐标
    let p0 = &nodes[elem_conn[0]];
    let p1 = &nodes[elem_conn[1]];
    let p2 = &nodes[elem_conn[2]];
    let p3 = &nodes[elem_conn[3]];

    // 生成 (n+1) x (n+1) 的内部节点网格
    // 使用参数坐标 (u, v) in [0, 1] x [0, 1]
    // 双线性插值: P(u,v) = (1-u)(1-v)*P0 + u(1-v)*P1 + uv*P2 + (1-u)v*P3
    let mut grid: Vec<Vec<usize>> = Vec::with_capacity(n + 1);
    for j in 0..=n {
        let mut row = Vec::with_capacity(n + 1);
        for i in 0..=n {
            let u = i as f64 / n as f64;
            let v = j as f64 / n as f64;

            // 检查是否是角点（复用已有节点）
            let node_id = if i == 0 && j == 0 {
                elem_conn[0]
            } else if i == n && j == 0 {
                elem_conn[1]
            } else if i == n && j == n {
                elem_conn[2]
            } else if i == 0 && j == n {
                elem_conn[3]
            } else {
                // 新节点
                let x = (1.0 - u) * (1.0 - v) * p0[0]
                    + u * (1.0 - v) * p1[0]
                    + u * v * p2[0]
                    + (1.0 - u) * v * p3[0];
                let y = (1.0 - u) * (1.0 - v) * p0[1]
                    + u * (1.0 - v) * p1[1]
                    + u * v * p2[1]
                    + (1.0 - u) * v * p3[1];
                let z = (1.0 - u) * (1.0 - v) * p0[2]
                    + u * (1.0 - v) * p1[2]
                    + u * v * p2[2]
                    + (1.0 - u) * v * p3[2];
                let id = new_nodes.len();
                new_nodes.push(vec![x, y, z]);
                id
            };
            row.push(node_id);
        }
        grid.push(row);
    }

    // 生成子单元
    let mut sub_elements = Vec::with_capacity(n * n);
    for j in 0..n {
        for i in 0..n {
            let n0 = grid[j][i];
            let n1 = grid[j][i + 1];
            let n2 = grid[j + 1][i + 1];
            let n3 = grid[j + 1][i];
            sub_elements.push(vec![n0, n1, n2, n3]);
        }
    }

    sub_elements
}

/// 细分 Hex8 单元
fn subdivide_hex8(
    nodes: &[Vec<f64>],
    elem_conn: &[usize],
    n: usize,
    new_nodes: &mut Vec<Vec<f64>>,
) -> Vec<Vec<usize>> {
    // 获取八个角节点坐标
    // Hex8 节点顺序: n000, n100, n110, n010, n001, n101, n111, n011
    let p000 = &nodes[elem_conn[0]];
    let p100 = &nodes[elem_conn[1]];
    let p110 = &nodes[elem_conn[2]];
    let p010 = &nodes[elem_conn[3]];
    let p001 = &nodes[elem_conn[4]];
    let p101 = &nodes[elem_conn[5]];
    let p111 = &nodes[elem_conn[6]];
    let p011 = &nodes[elem_conn[7]];

    // 生成 (n+1) x (n+1) x (n+1) 的内部节点网格
    // 使用三线性插值
    let mut grid: Vec<Vec<Vec<usize>>> = Vec::with_capacity(n + 1);
    for k in 0..=n {
        let mut layer = Vec::with_capacity(n + 1);
        for j in 0..=n {
            let mut row = Vec::with_capacity(n + 1);
            for i in 0..=n {
                let u = i as f64 / n as f64;
                let v = j as f64 / n as f64;
                let w = k as f64 / n as f64;

                // 检查是否是角点
                let node_id = if i == 0 && j == 0 && k == 0 {
                    elem_conn[0] // n000
                } else if i == n && j == 0 && k == 0 {
                    elem_conn[1] // n100
                } else if i == n && j == n && k == 0 {
                    elem_conn[2] // n110
                } else if i == 0 && j == n && k == 0 {
                    elem_conn[3] // n010
                } else if i == 0 && j == 0 && k == n {
                    elem_conn[4] // n001
                } else if i == n && j == 0 && k == n {
                    elem_conn[5] // n101
                } else if i == n && j == n && k == n {
                    elem_conn[6] // n111
                } else if i == 0 && j == n && k == n {
                    elem_conn[7] // n011
                } else {
                    // 三线性插值
                    let x = (1.0 - u) * (1.0 - v) * (1.0 - w) * p000[0]
                        + u * (1.0 - v) * (1.0 - w) * p100[0]
                        + u * v * (1.0 - w) * p110[0]
                        + (1.0 - u) * v * (1.0 - w) * p010[0]
                        + (1.0 - u) * (1.0 - v) * w * p001[0]
                        + u * (1.0 - v) * w * p101[0]
                        + u * v * w * p111[0]
                        + (1.0 - u) * v * w * p011[0];
                    let y = (1.0 - u) * (1.0 - v) * (1.0 - w) * p000[1]
                        + u * (1.0 - v) * (1.0 - w) * p100[1]
                        + u * v * (1.0 - w) * p110[1]
                        + (1.0 - u) * v * (1.0 - w) * p010[1]
                        + (1.0 - u) * (1.0 - v) * w * p001[1]
                        + u * (1.0 - v) * w * p101[1]
                        + u * v * w * p111[1]
                        + (1.0 - u) * v * w * p011[1];
                    let z = (1.0 - u) * (1.0 - v) * (1.0 - w) * p000[2]
                        + u * (1.0 - v) * (1.0 - w) * p100[2]
                        + u * v * (1.0 - w) * p110[2]
                        + (1.0 - u) * v * (1.0 - w) * p010[2]
                        + (1.0 - u) * (1.0 - v) * w * p001[2]
                        + u * (1.0 - v) * w * p101[2]
                        + u * v * w * p111[2]
                        + (1.0 - u) * v * w * p011[2];
                    let id = new_nodes.len();
                    new_nodes.push(vec![x, y, z]);
                    id
                };
                row.push(node_id);
            }
            layer.push(row);
        }
        grid.push(layer);
    }

    // 生成子单元
    let mut sub_elements = Vec::with_capacity(n * n * n);
    for k in 0..n {
        for j in 0..n {
            for i in 0..n {
                let c000 = grid[k][j][i];
                let c100 = grid[k][j][i + 1];
                let c110 = grid[k][j + 1][i + 1];
                let c010 = grid[k][j + 1][i];
                let c001 = grid[k + 1][j][i];
                let c101 = grid[k + 1][j][i + 1];
                let c111 = grid[k + 1][j + 1][i + 1];
                let c011 = grid[k + 1][j + 1][i];
                sub_elements.push(vec![c000, c100, c110, c010, c001, c101, c111, c011]);
            }
        }
    }

    sub_elements
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

    #[test]
    fn test_2d_mesh_with_edge_refinement() {
        let config = GridConfig::new_2d(10, 10, 10.0, 10.0);
        let generator = MeshGenerator::new(config);

        let refinements = vec![RefinementConfig {
            region_type: RefinementRegionType::Edge,
            axis: Some('x'),
            min_coord: Some(0.0),
            max_coord: Some(3.0),
            refinement_ratio: 2.0,
        }];

        let mesh = generator.generate_2d_rect_with_refinement(&refinements).unwrap();

        // 加密后节点数应多于均匀网格
        assert!(mesh.num_nodes > 121); // 11x11 = 121 for uniform
        // 加密后单元数也应多于均匀网格
        assert!(mesh.num_elements > 100); // 10x10 = 100 for uniform
        // 首尾坐标应精确
        assert!((mesh.nodes[0][0] - 0.0).abs() < 1e-10);
        assert!((mesh.nodes[0][1] - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_3d_mesh_with_face_refinement() {
        let config = GridConfig::new_3d(5, 5, 5, 10.0, 10.0, 10.0);
        let generator = MeshGenerator::new(config);

        let refinements = vec![RefinementConfig {
            region_type: RefinementRegionType::Face,
            axis: Some('z'),
            min_coord: Some(0.0),
            max_coord: Some(3.0),
            refinement_ratio: 2.0,
        }];

        let mesh = generator.generate_3d_box_with_refinement(&refinements).unwrap();

        // 加密后节点数应多于均匀网格
        assert!(mesh.num_nodes > 216); // 6x6x6 = 216 for uniform
        assert!(mesh.num_elements > 125); // 5x5x5 = 125 for uniform
    }

    #[test]
    fn test_refine_existing_mesh_2d() {
        let config = GridConfig::new_2d(4, 4, 4.0, 4.0);
        let generator = MeshGenerator::new(config);
        let mesh = generator.generate_2d_rect().unwrap();

        // 在中心区域加密
        let refinements = vec![RefinementConfig {
            region_type: RefinementRegionType::Volume,
            axis: None,
            min_coord: Some(1.0),
            max_coord: Some(3.0),
            refinement_ratio: 2.0,
        }];

        let refined = MeshGenerator::refine_existing_mesh(&mesh, &refinements).unwrap();

        // 加密后节点数和单元数应增加
        assert!(refined.num_nodes >= mesh.num_nodes);
        assert!(refined.num_elements >= mesh.num_elements);
    }

    #[test]
    fn test_refine_existing_mesh_3d() {
        let config = GridConfig::new_3d(2, 2, 2, 2.0, 2.0, 2.0);
        let generator = MeshGenerator::new(config);
        let mesh = generator.generate_3d_box().unwrap();

        let refinements = vec![RefinementConfig {
            region_type: RefinementRegionType::Volume,
            axis: None,
            min_coord: Some(0.5),
            max_coord: Some(1.5),
            refinement_ratio: 2.0,
        }];

        let refined = MeshGenerator::refine_existing_mesh(&mesh, &refinements).unwrap();

        assert!(refined.num_nodes >= mesh.num_nodes);
        assert!(refined.num_elements >= mesh.num_elements);
    }

    #[test]
    fn test_multiple_refinement_regions() {
        let config = GridConfig::new_2d(10, 10, 10.0, 10.0);
        let generator = MeshGenerator::new(config);

        // 同时在左边和右边加密
        let refinements = vec![
            RefinementConfig {
                region_type: RefinementRegionType::Edge,
                axis: Some('x'),
                min_coord: Some(0.0),
                max_coord: Some(2.0),
                refinement_ratio: 2.0,
            },
            RefinementConfig {
                region_type: RefinementRegionType::Edge,
                axis: Some('x'),
                min_coord: Some(8.0),
                max_coord: Some(10.0),
                refinement_ratio: 3.0,
            },
        ];

        let mesh = generator.generate_2d_rect_with_refinement(&refinements).unwrap();
        assert!(mesh.num_nodes > 121);
    }

    #[test]
    fn test_empty_refinements() {
        let config = GridConfig::new_2d(4, 4, 4.0, 4.0);
        let generator = MeshGenerator::new(config);
        let mesh = generator.generate_2d_rect().unwrap();

        let refined = MeshGenerator::refine_existing_mesh(&mesh, &[]).unwrap();
        assert_eq!(refined.num_nodes, mesh.num_nodes);
        assert_eq!(refined.num_elements, mesh.num_elements);
    }

    #[test]
    fn test_invalid_refinement_ratio() {
        let config = GridConfig::new_2d(4, 4, 4.0, 4.0);
        let generator = MeshGenerator::new(config);

        let refinements = vec![RefinementConfig {
            region_type: RefinementRegionType::Edge,
            axis: Some('x'),
            min_coord: Some(0.0),
            max_coord: Some(2.0),
            refinement_ratio: 0.5, // 无效
        }];

        let result = generator.generate_2d_rect_with_refinement(&refinements);
        assert!(result.is_err());
    }
}