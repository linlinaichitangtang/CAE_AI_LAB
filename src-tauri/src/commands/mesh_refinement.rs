//! 局部加密与映射网格模块 (V1.3-009)
//!
//! 本模块提供网格局部加密、边界层网格生成和映射网格功能：
//! - 区域局部加密（球体/圆柱/盒子）
//! - 边界层网格生成（y+控制）
//! - 映射网格生成

use serde::{Deserialize, Serialize};

/// 加密区域类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefinementRegionType {
    /// 球体区域
    Sphere,
    /// 圆柱区域
    Cylinder,
    /// 盒子区域
    Box,
    /// 自定义面区域
    CustomFace,
}

/// 加密区域定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefinementRegion {
    /// 区域类型
    pub region_type: RefinementRegionType,
    /// 区域中心
    pub center: [f64; 3],
    /// 区域尺寸参数 [radius / radius+height / half_x+half_y+half_z]
    pub size: Vec<f64>,
    /// 方向向量（用于圆柱方向）
    pub direction: Option<[f64; 3]>,
    /// 自定义面节点ID列表
    pub face_node_ids: Option<Vec<usize>>,
}

/// 网格输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshOutput {
    /// 节点坐标
    pub nodes: Vec<[f64; 3]>,
    /// 单元连接
    pub elements: Vec<Vec<usize>>,
    /// 单元类型
    pub element_type: String,
    /// 节点数
    pub node_count: usize,
    /// 单元数
    pub element_count: usize,
}

/// 边界层网格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundaryLayerMesh {
    /// 边界层节点
    pub nodes: Vec<[f64; 3]>,
    /// 边界层单元
    pub elements: Vec<Vec<usize>>,
    /// 各层高度
    pub layer_heights: Vec<f64>,
    /// y+值
    pub y_plus_values: Vec<f64>,
    /// 总厚度
    pub total_thickness: f64,
}

/// 计算点到点距离
fn dist(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// 判断点是否在加密区域内
fn point_in_region(point: &[f64; 3], region: &RefinementRegion) -> bool {
    match region.region_type {
        RefinementRegionType::Sphere => {
            let radius = region.size.first().copied().unwrap_or(1.0);
            dist(point, &region.center) <= radius
        }
        RefinementRegionType::Cylinder => {
            let radius = region.size.first().copied().unwrap_or(1.0);
            let height = region.size.get(1).copied().unwrap_or(1.0);
            let dir = region.direction.unwrap_or([0.0, 0.0, 1.0]);
            let dir_len = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt();
            if dir_len < 1e-15 {
                return false;
            }
            let dn = [dir[0] / dir_len, dir[1] / dir_len, dir[2] / dir_len];
            let dp = [
                point[0] - region.center[0],
                point[1] - region.center[1],
                point[2] - region.center[2],
            ];
            let proj = dp[0] * dn[0] + dp[1] * dn[1] + dp[2] * dn[2];
            let perp = [
                dp[0] - proj * dn[0],
                dp[1] - proj * dn[1],
                dp[2] - proj * dn[2],
            ];
            let perp_dist = (perp[0] * perp[0] + perp[1] * perp[1] + perp[2] * perp[2]).sqrt();
            perp_dist <= radius && proj.abs() <= height / 2.0
        }
        RefinementRegionType::Box => {
            let hx = region.size.first().copied().unwrap_or(1.0);
            let hy = region.size.get(1).copied().unwrap_or(1.0);
            let hz = region.size.get(2).copied().unwrap_or(1.0);
            (point[0] - region.center[0]).abs() <= hx
                && (point[1] - region.center[1]).abs() <= hy
                && (point[2] - region.center[2]).abs() <= hz
        }
        RefinementRegionType::CustomFace => {
            // 对于自定义面，检查是否在面附近（简化处理）
            if let Some(ref node_ids) = region.face_node_ids {
                if node_ids.is_empty() {
                    return false;
                }
                // 检查点到面中心的距离
                let cx = region.center[0];
                let cy = region.center[1];
                let cz = region.center[2];
                let d = dist(point, &[cx, cy, cz]);
                let radius = region.size.first().copied().unwrap_or(1.0);
                d <= radius
            } else {
                false
            }
        }
    }
}

/// 局部加密网格
#[tauri::command]
pub fn refine_mesh_region(
    nodes: Vec<[f64; 3]>,
    elements: Vec<Vec<usize>>,
    region: RefinementRegion,
    levels: u32,
) -> Result<MeshOutput, String> {
    if nodes.is_empty() || elements.is_empty() {
        return Err("节点或单元数据为空".to_string());
    }
    if levels == 0 || levels > 5 {
        return Err("加密级别必须在 1-5 之间".to_string());
    }

    // 找到需要加密的单元
    let mut elements_to_refine: Vec<usize> = Vec::new();
    for (elem_id, elem) in elements.iter().enumerate() {
        // 计算单元中心
        let mut cx = 0.0;
        let mut cy = 0.0;
        let mut cz = 0.0;
        for &node_id in elem {
            if node_id < nodes.len() {
                cx += nodes[node_id][0];
                cy += nodes[node_id][1];
                cz += nodes[node_id][2];
            }
        }
        let n = elem.len() as f64;
        let center = [cx / n, cy / n, cz / n];

        if point_in_region(&center, &region) {
            elements_to_refine.push(elem_id);
        }
    }

    let refine_set: HashSet<usize> = elements_to_refine.iter().copied().collect();

    // 执行加密：对每个需要加密的单元进行细分
    let mut new_nodes: Vec<[f64; 3]> = nodes.clone();
    let mut new_elements: Vec<Vec<usize>> = Vec::new();
    let mut node_map: HashMap<(usize, usize), usize> = HashMap::new(); // 边中点映射

    // 为已有边创建中点
    for elem in &elements {
        for i in 0..elem.len() {
            let j = (i + 1) % elem.len();
            let a = elem[i].min(elem[j]);
            let b = elem[i].max(elem[j]);
            let key = (a, b);
            if !node_map.contains_key(&key) {
                let mid = [
                    (new_nodes[a][0] + new_nodes[b][0]) / 2.0,
                    (new_nodes[a][1] + new_nodes[b][1]) / 2.0,
                    (new_nodes[a][2] + new_nodes[b][2]) / 2.0,
                ];
                let new_id = new_nodes.len();
                new_nodes.push(mid);
                node_map.insert(key, new_id);
            }
        }
    }

    // 细分单元
    for (elem_id, elem) in elements.iter().enumerate() {
        if refine_set.contains(&elem_id) {
            // 三角形 -> 4个三角形
            if elem.len() == 3 {
                let m01 = node_map[&(elem[0].min(elem[1]), elem[0].max(elem[1]))];
                let m12 = node_map[&(elem[1].min(elem[2]), elem[1].max(elem[2]))];
                let m20 = node_map[&(elem[2].min(elem[0]), elem[2].max(elem[0]))];
                new_elements.push(vec![elem[0], m01, m20]);
                new_elements.push(vec![elem[1], m12, m01]);
                new_elements.push(vec![elem[2], m20, m12]);
                new_elements.push(vec![m01, m12, m20]);
            }
            // 四边形 -> 4个四边形
            else if elem.len() == 4 {
                let m01 = node_map[&(elem[0].min(elem[1]), elem[0].max(elem[1]))];
                let m12 = node_map[&(elem[1].min(elem[2]), elem[1].max(elem[2]))];
                let m23 = node_map[&(elem[2].min(elem[3]), elem[2].max(elem[3]))];
                let m30 = node_map[&(elem[3].min(elem[0]), elem[3].max(elem[0]))];
                new_elements.push(vec![elem[0], m01, m12.min(m30).max(m30.min(m12)), m30]); // center approximation
                // 简化：用4个三角形近似
                let center = [
                    (new_nodes[m01][0] + new_nodes[m23][0]) / 2.0,
                    (new_nodes[m01][1] + new_nodes[m23][1]) / 2.0,
                    (new_nodes[m01][2] + new_nodes[m23][2]) / 2.0,
                ];
                let cid = new_nodes.len();
                new_nodes.push(center);
                new_elements.push(vec![elem[0], m01, cid, m30]);
                new_elements.push(vec![elem[1], m12, cid, m01]);
                new_elements.push(vec![elem[2], m23, cid, m12]);
                new_elements.push(vec![elem[3], m30, cid, m23]);
            } else {
                // 其他类型保持不变
                new_elements.push(elem.clone());
            }
        } else {
            new_elements.push(elem.clone());
        }
    }

    // 多级加密
    for _level in 1..levels {
        let mut current_nodes = new_nodes.clone();
        let mut current_elements = new_elements.clone();
        let mut current_node_map: HashMap<(usize, usize), usize> = HashMap::new();

        for elem in &current_elements {
            for i in 0..elem.len() {
                let j = (i + 1) % elem.len();
                let a = elem[i].min(elem[j]);
                let b = elem[i].max(elem[j]);
                let key = (a, b);
                if !current_node_map.contains_key(&key) {
                    let mid = [
                        (current_nodes[a][0] + current_nodes[b][0]) / 2.0,
                        (current_nodes[a][1] + current_nodes[b][1]) / 2.0,
                        (current_nodes[a][2] + current_nodes[b][2]) / 2.0,
                    ];
                    let new_id = current_nodes.len();
                    current_nodes.push(mid);
                    current_node_map.insert(key, new_id);
                }
            }
        }

        let mut next_elements: Vec<Vec<usize>> = Vec::new();
        for (elem_id, elem) in current_elements.iter().enumerate() {
            // 只加密在原始加密区域内的单元
            let mut cx = 0.0;
            let mut cy = 0.0;
            let mut cz = 0.0;
            for &nid in elem {
                if nid < current_nodes.len() {
                    cx += current_nodes[nid][0];
                    cy += current_nodes[nid][1];
                    cz += current_nodes[nid][2];
                }
            }
            let n = elem.len() as f64;
            if point_in_region(&[cx / n, cy / n, cz / n], &region) {
                if elem.len() == 3 {
                    let m01 = current_node_map[&(elem[0].min(elem[1]), elem[0].max(elem[1]))];
                    let m12 = current_node_map[&(elem[1].min(elem[2]), elem[1].max(elem[2]))];
                    let m20 = current_node_map[&(elem[2].min(elem[0]), elem[2].max(elem[0]))];
                    next_elements.push(vec![elem[0], m01, m20]);
                    next_elements.push(vec![elem[1], m12, m01]);
                    next_elements.push(vec![elem[2], m20, m12]);
                    next_elements.push(vec![m01, m12, m20]);
                } else {
                    next_elements.push(elem.clone());
                }
            } else {
                next_elements.push(elem.clone());
            }
        }

        new_nodes = current_nodes;
        new_elements = next_elements;
    }

    Ok(MeshOutput {
        node_count: new_nodes.len(),
        element_count: new_elements.len(),
        element_type: if elements[0].len() == 3 {
            "S3".to_string()
        } else {
            "S4".to_string()
        },
        nodes: new_nodes,
        elements: new_elements,
    })
}

/// 生成边界层网格
#[tauri::command]
pub fn generate_boundary_layer_mesh(
    surface_nodes: Vec<[f64; 3]>,
    surface_elements: Vec<Vec<usize>>,
    normal_direction: [f64; 3],
    first_layer_height: f64,
    growth_rate: f64,
    num_layers: u32,
    y_plus_target: f64,
) -> Result<BoundaryLayerMesh, String> {
    if surface_nodes.is_empty() || surface_elements.is_empty() {
        return Err("表面节点或单元数据为空".to_string());
    }
    if num_layers == 0 || num_layers > 50 {
        return Err("边界层数必须在 1-50 之间".to_string());
    }
    if growth_rate < 1.0 || growth_rate > 3.0 {
        return Err("增长率必须在 1.0-3.0 之间".to_string());
    }
    if first_layer_height <= 0.0 {
        return Err("第一层高度必须大于0".to_string());
    }

    // 归一化法向
    let n_len = (normal_direction[0] * normal_direction[0]
        + normal_direction[1] * normal_direction[1]
        + normal_direction[2] * normal_direction[2])
        .sqrt();
    if n_len < 1e-15 {
        return Err("法向方向不能为零向量".to_string());
    }
    let normal = [
        normal_direction[0] / n_len,
        normal_direction[1] / n_len,
        normal_direction[2] / n_len,
    ];

    // 计算各层高度（等比增长）
    let mut layer_heights: Vec<f64> = Vec::new();
    let mut h = first_layer_height;
    for _ in 0..num_layers {
        layer_heights.push(h);
        h *= growth_rate;
    }

    // 计算 y+ 值
    // y+ = y * u_tau / nu, 简化估算
    let nu = 1.5e-5; // 空气运动粘度 m^2/s (20°C)
    let u_tau = y_plus_target * nu / first_layer_height; // 摩擦速度估算
    let mut y_plus_values: Vec<f64> = Vec::new();
    let mut cumulative_height = 0.0;
    for &lh in &layer_heights {
        cumulative_height += lh;
        let y_plus = cumulative_height * u_tau / nu;
        y_plus_values.push(y_plus);
    }

    // 生成边界层节点
    let mut bl_nodes: Vec<[f64; 3]> = surface_nodes.clone();
    let mut bl_elements: Vec<Vec<usize>> = Vec::new();
    let num_surface_nodes = surface_nodes.len();

    // 为每一层生成节点
    for (layer_idx, &lh) in layer_heights.iter().enumerate() {
        let offset = (layer_idx as f64 + 1.0) * lh;
        // 修正：使用累积高度
        let cumulative: f64 = layer_heights[..=layer_idx].iter().sum();
        for node in &surface_nodes {
            bl_nodes.push([
                node[0] + normal[0] * cumulative,
                node[1] + normal[1] * cumulative,
                node[2] + normal[2] * cumulative,
            ]);
        }
    }

    // 生成边界层单元（棱柱/六面体）
    for elem in &surface_elements {
        if elem.len() == 3 {
            // 三角形面 -> 棱柱层
            for layer in 0..num_layers as usize {
                let base_offset = num_surface_nodes + layer * num_surface_nodes;
                let top_offset = num_surface_nodes + (layer + 1) * num_surface_nodes;
                // 两个三角形
                bl_elements.push(vec![
                    elem[0] + base_offset,
                    elem[1] + base_offset,
                    elem[2] + base_offset,
                    elem[0] + top_offset,
                ]);
                bl_elements.push(vec![
                    elem[1] + base_offset,
                    elem[2] + base_offset,
                    elem[1] + top_offset,
                    elem[0] + top_offset,
                ]);
                bl_elements.push(vec![
                    elem[2] + base_offset,
                    elem[0] + top_offset,
                    elem[1] + top_offset,
                    elem[2] + top_offset,
                ]);
            }
        } else if elem.len() == 4 {
            // 四边形面 -> 六面体层
            for layer in 0..num_layers as usize {
                let base_offset = num_surface_nodes + layer * num_surface_nodes;
                let top_offset = num_surface_nodes + (layer + 1) * num_surface_nodes;
                bl_elements.push(vec![
                    elem[0] + base_offset,
                    elem[1] + base_offset,
                    elem[2] + base_offset,
                    elem[3] + base_offset,
                    elem[0] + top_offset,
                    elem[1] + top_offset,
                    elem[2] + top_offset,
                    elem[3] + top_offset,
                ]);
            }
        }
    }

    let total_thickness: f64 = layer_heights.iter().sum();

    Ok(BoundaryLayerMesh {
        nodes: bl_nodes,
        elements: bl_elements,
        layer_heights,
        y_plus_values,
        total_thickness,
    })
}

/// 生成映射网格
#[tauri::command]
pub fn generate_mapped_mesh(
    source_nodes: Vec<[f64; 3]>,
    target_boundary: Vec<[f64; 3]>,
    num_layers: u32,
) -> Result<MeshOutput, String> {
    if source_nodes.is_empty() || target_boundary.is_empty() {
        return Err("源节点或目标边界数据为空".to_string());
    }
    if num_layers == 0 || num_layers > 100 {
        return Err("层数必须在 1-100 之间".to_string());
    }

    // 计算源和目标的边界框
    let source_min = source_nodes.iter().fold([f64::MAX; 3], |acc, p| {
        [acc[0].min(p[0]), acc[1].min(p[1]), acc[2].min(p[2])]
    });
    let source_max = source_nodes.iter().fold([f64::MIN; 3], |acc, p| {
        [acc[0].max(p[0]), acc[1].max(p[1]), acc[2].max(p[2])]
    });
    let target_min = target_boundary.iter().fold([f64::MAX; 3], |acc, p| {
        [acc[0].min(p[0]), acc[1].min(p[1]), acc[2].min(p[2])]
    });
    let target_max = target_boundary.iter().fold([f64::MIN; 3], |acc, p| {
        [acc[0].max(p[0]), acc[1].max(p[1]), acc[2].max(p[2])]
    });

    // 线性插值生成中间层节点
    let mut all_nodes: Vec<[f64; 3]> = source_nodes.clone();
    let n_src = source_nodes.len();
    let n_tgt = target_boundary.len();

    // 如果源和目标节点数不同，使用投影映射
    let use_equal_count = n_src == n_tgt;

    for layer in 1..=num_layers {
        let t = layer as f64 / (num_layers as f64 + 1.0);

        if use_equal_count {
            for i in 0..n_src {
                all_nodes.push([
                    source_nodes[i][0] * (1.0 - t) + target_boundary[i][0] * t,
                    source_nodes[i][1] * (1.0 - t) + target_boundary[i][1] * t,
                    source_nodes[i][2] * (1.0 - t) + target_boundary[i][2] * t,
                ]);
            }
        } else {
            // 使用边界框插值
            let min = [
                source_min[0] * (1.0 - t) + target_min[0] * t,
                source_min[1] * (1.0 - t) + target_min[1] * t,
                source_min[2] * (1.0 - t) + target_min[2] * t,
            ];
            let max = [
                source_max[0] * (1.0 - t) + target_max[0] * t,
                source_max[1] * (1.0 - t) + target_max[1] * t,
                source_max[2] * (1.0 - t) + target_max[2] * t,
            ];

            // 归一化源节点坐标并映射到目标边界框
            for src_node in &source_nodes {
                let mut mapped = [0.0; 3];
                for dim in 0..3 {
                    let range_src = source_max[dim] - source_min[dim];
                    let range_tgt = max[dim] - min[dim];
                    if range_src > 1e-15 {
                        let s = (src_node[dim] - source_min[dim]) / range_src;
                        mapped[dim] = min[dim] + s * range_tgt;
                    } else {
                        mapped[dim] = (min[dim] + max[dim]) / 2.0;
                    }
                }
                all_nodes.push(mapped);
            }
        }
    }

    // 添加目标节点
    all_nodes.extend(target_boundary.clone());

    // 生成连接单元（简化：使用四边形连接相邻层）
    let mut all_elements: Vec<Vec<usize>> = Vec::new();
    let nodes_per_layer = n_src;

    for layer in 0..num_layers as usize {
        let base = layer * nodes_per_layer;
        let top = (layer + 1) * nodes_per_layer;

        // 假设源节点按顺序排列，生成四边形单元
        for i in 0..nodes_per_layer.saturating_sub(1) {
            all_elements.push(vec![
                base + i,
                base + i + 1,
                top + i + 1,
                top + i,
            ]);
        }
    }

    // 最后一层连接到目标
    let last_base = (num_layers as usize) * nodes_per_layer;
    if use_equal_count {
        for i in 0..nodes_per_layer.saturating_sub(1) {
            all_elements.push(vec![
                last_base + i,
                last_base + i + 1,
                all_nodes.len() - n_tgt + i + 1,
                all_nodes.len() - n_tgt + i,
            ]);
        }
    }

    Ok(MeshOutput {
        node_count: all_nodes.len(),
        element_count: all_elements.len(),
        element_type: "S4".to_string(),
        nodes: all_nodes,
        elements: all_elements,
    })
}
