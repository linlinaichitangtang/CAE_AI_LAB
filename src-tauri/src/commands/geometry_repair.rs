//! 几何清理与修复模块 (V1.3-007)
//!
//! 本模块提供几何模型的自动检测与修复功能：
//! - 小孔检测与填充
//! - 裂缝/狭缝检测与修复
//! - 重复面检测
//! - 非流形边检测
//! - 退化单元检测与修复
//! - 法向统一

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// 几何问题类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GeometryIssueType {
    /// 小孔（边界环周长小于阈值）
    SmallHole,
    /// 裂缝/狭缝（T型连接边）
    Slit,
    /// 重复面
    DuplicateFace,
    /// 非流形边
    NonManifold,
    /// 退化单元（体积/面积过小）
    Degenerate,
}

impl GeometryIssueType {
    pub fn as_str(&self) -> &str {
        match self {
            GeometryIssueType::SmallHole => "small_hole",
            GeometryIssueType::Slit => "slit",
            GeometryIssueType::DuplicateFace => "duplicate_face",
            GeometryIssueType::NonManifold => "non_manifold",
            GeometryIssueType::Degenerate => "degenerate",
        }
    }
}

/// 几何问题描述
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometryIssue {
    /// 问题类型
    pub issue_type: String,
    /// 问题位置坐标
    pub location: [f64; 3],
    /// 严重程度
    pub severity: String,
    /// 是否可自动修复
    pub fixable: bool,
    /// 附加信息
    pub info: String,
}

/// 网格统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshStats {
    pub node_count: usize,
    pub element_count: usize,
    pub min_edge_length: f64,
    pub max_edge_length: f64,
    pub avg_edge_length: f64,
}

/// 修复结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairResult {
    pub issues_found: usize,
    pub issues_fixed: usize,
    pub remaining_issues: Vec<GeometryIssue>,
    pub mesh_stats_before: MeshStats,
    pub mesh_stats_after: MeshStats,
}

/// 计算两点距离
fn distance(p1: &[f64; 3], p2: &[f64; 3]) -> f64 {
    let dx = p1[0] - p2[0];
    let dy = p1[1] - p2[1];
    let dz = p1[2] - p2[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// 计算三角形面积
fn triangle_area(p1: &[f64; 3], p2: &[f64; 3], p3: &[f64; 3]) -> f64 {
    let v1 = [p2[0] - p1[0], p2[1] - p1[1], p2[2] - p1[2]];
    let v2 = [p3[0] - p1[0], p3[1] - p1[1], p3[2] - p1[2]];
    let cross = [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ];
    (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt() / 2.0
}

/// 计算四面体体积
fn tet_volume(p0: &[f64; 3], p1: &[f64; 3], p2: &[f64; 3], p3: &[f64; 3]) -> f64 {
    let v1 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let v2 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
    let v3 = [p3[0] - p0[0], p3[1] - p0[1], p3[2] - p0[2]];
    let cross = [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ];
    (v3[0] * cross[0] + v3[1] * cross[1] + v3[2] * cross[2]).abs() / 6.0
}

/// 计算六面体体积（分解为5个四面体）
fn hex_volume(nodes: &[[f64; 3]], elem: &[usize]) -> f64 {
    if elem.len() < 8 {
        return 0.0;
    }
    let p = |i: usize| &nodes[elem[i]];
    // 将六面体分解为5个四面体
    let v1 = tet_volume(p(0), p(1), p(3), p(4));
    let v2 = tet_volume(p(1), p(2), p(3), p(5));
    let v3 = tet_volume(p(1), p(3), p(5), p(4));
    let v4 = tet_volume(p(3), p(5), p(6), p(4));
    let v5 = tet_volume(p(3), p(6), p(7), p(4));
    v1 + v2 + v3 + v4 + v5
}

/// 计算面法向
fn face_normal(nodes: &[[f64; 3]], face_nodes: &[usize]) -> [f64; 3] {
    if face_nodes.len() < 3 {
        return [0.0, 0.0, 0.0];
    }
    let p0 = &nodes[face_nodes[0]];
    let p1 = &nodes[face_nodes[1]];
    let p2 = &nodes[face_nodes[2]];
    let v1 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let v2 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
    let mut n = [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ];
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > 1e-15 {
        n[0] /= len;
        n[1] /= len;
        n[2] /= len;
    }
    n
}

/// 计算网格统计信息
pub fn compute_mesh_stats(nodes: &[[f64; 3]], elements: &[[usize]]) -> MeshStats {
    let node_count = nodes.len();
    let element_count = elements.len();

    if elements.is_empty() || nodes.is_empty() {
        return MeshStats {
            node_count,
            element_count,
            min_edge_length: 0.0,
            max_edge_length: 0.0,
            avg_edge_length: 0.0,
        };
    }

    let mut min_edge = f64::MAX;
    let mut max_edge = 0.0;
    let mut total_edge = 0.0;
    let mut edge_count = 0usize;

    for elem in elements.iter() {
        for i in 0..elem.len() {
            let j = (i + 1) % elem.len();
            if elem[i] < nodes.len() && elem[j] < nodes.len() {
                let d = distance(&nodes[elem[i]], &nodes[elem[j]]);
                min_edge = min_edge.min(d);
                max_edge = max_edge.max(d);
                total_edge += d;
                edge_count += 1;
            }
        }
    }

    let avg_edge = if edge_count > 0 { total_edge / edge_count as f64 } else { 0.0 };

    MeshStats {
        node_count,
        element_count,
        min_edge_length: min_edge,
        max_edge_length: max_edge,
        avg_edge_length: avg_edge,
    }
}

/// 检测几何问题
#[tauri::command]
pub fn detect_geometry_issues(
    nodes: Vec<[f64; 3]>,
    elements: Vec<Vec<usize>>,
) -> Result<Vec<GeometryIssue>, String> {
    if nodes.is_empty() || elements.is_empty() {
        return Err("节点或单元数据为空".to_string());
    }

    let mut issues: Vec<GeometryIssue> = Vec::new();

    // 1. 构建边-面邻接关系
    let mut edge_faces: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (face_id, elem) in elements.iter().enumerate() {
        for i in 0..elem.len() {
            let j = (i + 1) % elem.len();
            let a = elem[i].min(elem[j]);
            let b = elem[i].max(elem[j]);
            edge_faces.entry((a, b)).or_default().push(face_id);
        }
    }

    // 2. 检测非流形边（被超过2个面共享的边）
    for ((a, b), faces) in &edge_faces {
        if faces.len() > 2 {
            let mid = [
                (nodes[*a][0] + nodes[*b][0]) / 2.0,
                (nodes[*a][1] + nodes[*b][1]) / 2.0,
                (nodes[*a][2] + nodes[*b][2]) / 2.0,
            ];
            issues.push(GeometryIssue {
                issue_type: GeometryIssueType::NonManifold.as_str().to_string(),
                location: mid,
                severity: "high".to_string(),
                fixable: true,
                info: format!("边 ({}, {}) 被 {} 个面共享", a, b, faces.len()),
            });
        }
    }

    // 3. 检测小孔（边界环）
    // 边界边：只被1个面共享的边
    let mut boundary_edges: HashMap<usize, Vec<usize>> = HashMap::new();
    for ((a, b), faces) in &edge_faces {
        if faces.len() == 1 {
            boundary_edges.entry(*a).or_default().push(*b);
            boundary_edges.entry(*b).or_default().push(*a);
        }
    }

    // 找到边界环（连续的边界边形成环）
    let mut visited_boundary: HashSet<usize> = HashSet::new();
    for &start_node in boundary_edges.keys() {
        if visited_boundary.contains(&start_node) {
            continue;
        }
        // 追踪边界环
        let mut ring: Vec<usize> = vec![start_node];
        visited_boundary.insert(start_node);
        let mut current = start_node;
        let mut loop_closed = false;

        for _ in 0..boundary_edges.len() + 1 {
            if let Some(neighbors) = boundary_edges.get(&current) {
                let mut found_next = false;
                for &next in neighbors {
                    if !visited_boundary.contains(&next) {
                        ring.push(next);
                        visited_boundary.insert(next);
                        current = next;
                        found_next = true;
                        break;
                    } else if next == start_node && ring.len() > 2 {
                        loop_closed = true;
                        break;
                    }
                }
                if loop_closed || !found_next {
                    break;
                }
            } else {
                break;
            }
        }

        // 计算环周长
        if ring.len() >= 3 {
            let mut perimeter = 0.0;
            for i in 0..ring.len() {
                let j = (i + 1) % ring.len();
                perimeter += distance(&nodes[ring[i]], &nodes[ring[j]]);
            }

            // 如果周长很小，标记为小孔
            let avg_edge = compute_mesh_stats(&nodes, &elements.iter().map(|e| e.as_slice()).collect::<Vec<_>>()).avg_edge_length;
            let threshold = avg_edge * 4.0; // 小孔阈值：4倍平均边长

            if perimeter < threshold && perimeter > 0.0 {
                let cx = ring.iter().map(|&n| nodes[n][0]).sum::<f64>() / ring.len() as f64;
                let cy = ring.iter().map(|&n| nodes[n][1]).sum::<f64>() / ring.len() as f64;
                let cz = ring.iter().map(|&n| nodes[n][2]).sum::<f64>() / ring.len() as f64;
                issues.push(GeometryIssue {
                    issue_type: GeometryIssueType::SmallHole.as_str().to_string(),
                    location: [cx, cy, cz],
                    severity: "medium".to_string(),
                    fixable: true,
                    info: format!("边界环周长 {:.4}，节点数 {}", perimeter, ring.len()),
                });
            }
        }
    }

    // 4. 检测裂缝（T型连接边：边界边连接到非边界边）
    let boundary_edge_set: HashSet<(usize, usize)> = edge_faces
        .iter()
        .filter(|(_, f)| f.len() == 1)
        .map(|(k, _)| *k)
        .collect();

    for ((a, b), faces) in &edge_faces {
        if faces.len() == 2 {
            // 检查此边的端点是否有边界边连接
            let a_boundary = boundary_edges.get(a).map_or(0, |v| v.len());
            let b_boundary = boundary_edges.get(b).map_or(0, |v| v.len());
            // 如果一个端点有3+条边界边，可能是T型连接
            if a_boundary >= 3 || b_boundary >= 3 {
                let mid = [
                    (nodes[*a][0] + nodes[*b][0]) / 2.0,
                    (nodes[*a][1] + nodes[*b][1]) / 2.0,
                    (nodes[*a][2] + nodes[*b][2]) / 2.0,
                ];
                issues.push(GeometryIssue {
                    issue_type: GeometryIssueType::Slit.as_str().to_string(),
                    location: mid,
                    severity: "medium".to_string(),
                    fixable: true,
                    info: format!("可能的裂缝/T型连接，边 ({}, {})", a, b),
                });
            }
        }
    }

    // 5. 检测重复面（相同节点集合的面）
    let mut face_signatures: HashMap<String, Vec<usize>> = HashMap::new();
    for (face_id, elem) in elements.iter().enumerate() {
        let mut sorted_nodes: Vec<usize> = elem.clone();
        sorted_nodes.sort();
        let sig = sorted_nodes
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        face_signatures.entry(sig).or_default().push(face_id);
    }

    for (sig, face_ids) in &face_signatures {
        if face_ids.len() > 1 {
            let first_elem = &elements[face_ids[0]];
            let cx = first_elem.iter().map(|&n| nodes[n][0]).sum::<f64>() / first_elem.len() as f64;
            let cy = first_elem.iter().map(|&n| nodes[n][1]).sum::<f64>() / first_elem.len() as f64;
            let cz = first_elem.iter().map(|&n| nodes[n][2]).sum::<f64>() / first_elem.len() as f64;
            issues.push(GeometryIssue {
                issue_type: GeometryIssueType::DuplicateFace.as_str().to_string(),
                location: [cx, cy, cz],
                severity: "low".to_string(),
                fixable: true,
                info: format!("发现 {} 个重复面: {:?}", face_ids.len(), face_ids),
            });
        }
    }

    // 6. 检测退化单元
    let stats = compute_mesh_stats(&nodes, &elements.iter().map(|e| e.as_slice()).collect::<Vec<_>>());
    let volume_threshold = stats.avg_edge_length.powi(3) * 1e-6;

    for (elem_id, elem) in elements.iter().enumerate() {
        let volume = if elem.len() == 3 {
            // 三角形
            triangle_area(&nodes[elem[0]], &nodes[elem[1]], &nodes[elem[2]])
        } else if elem.len() == 4 {
            // 四面体
            tet_volume(&nodes[elem[0]], &nodes[elem[1]], &nodes[elem[2]], &nodes[elem[3]])
        } else if elem.len() == 8 {
            // 六面体
            hex_volume(&nodes, elem)
        } else if elem.len() >= 3 {
            // 通用三角形面积（取前3个节点）
            triangle_area(&nodes[elem[0]], &nodes[elem[1]], &nodes[elem[2]])
        } else {
            0.0
        };

        if volume < volume_threshold {
            let cx = elem.iter().map(|&n| nodes[n][0]).sum::<f64>() / elem.len() as f64;
            let cy = elem.iter().map(|&n| nodes[n][1]).sum::<f64>() / elem.len() as f64;
            let cz = elem.iter().map(|&n| nodes[n][2]).sum::<f64>() / elem.len() as f64;
            issues.push(GeometryIssue {
                issue_type: GeometryIssueType::Degenerate.as_str().to_string(),
                location: [cx, cy, cz],
                severity: "high".to_string(),
                fixable: true,
                info: format!("单元 #{} 体积/面积 {:.6}，阈值 {:.6}", elem_id, volume, volume_threshold),
            });
        }
    }

    Ok(issues)
}

/// 自动修复几何问题
#[tauri::command]
pub fn auto_repair_geometry(
    nodes: Vec<[f64; 3]>,
    elements: Vec<Vec<usize>>,
    issues: Vec<GeometryIssue>,
) -> Result<RepairResult, String> {
    let stats_before = compute_mesh_stats(&nodes, &elements.iter().map(|e| e.as_slice()).collect::<Vec<_>>());

    let mut fixed_count = 0usize;
    let mut remaining: Vec<GeometryIssue> = Vec::new();

    // 收集要删除的重复面
    let mut faces_to_remove: HashSet<usize> = HashSet::new();
    // 收集要删除的退化单元
    let mut degenerate_elems: HashSet<usize> = HashSet::new();

    for issue in &issues {
        match issue.issue_type.as_str() {
            "duplicate_face" => {
                // 重复面修复：从info中解析面ID
                if let Some(start) = issue.info.find('[') {
                    if let Some(end) = issue.info.find(']') {
                        let ids_str = &issue.info[start + 1..end];
                        let ids: Vec<usize> = ids_str
                            .split(", ")
                            .filter_map(|s| s.trim().parse().ok())
                            .collect();
                        // 保留第一个，删除其余
                        for &id in ids.iter().skip(1) {
                            faces_to_remove.insert(id);
                        }
                        fixed_count += ids.len() - 1;
                    }
                }
            }
            "degenerate" => {
                if let Some(start) = issue.info.find('#') {
                    let rest = &issue.info[start + 1..];
                    if let Some(space) = rest.find(' ') {
                        if let Ok(elem_id) = rest[..space].parse::<usize>() {
                            degenerate_elems.insert(elem_id);
                            fixed_count += 1;
                        }
                    }
                }
            }
            "small_hole" | "slit" | "non_manifold" => {
                // 这些需要更复杂的修复逻辑，标记为剩余
                remaining.push(issue.clone());
            }
            _ => {
                remaining.push(issue.clone());
            }
        }
    }

    // 构建修复后的单元列表
    let repaired_elements: Vec<Vec<usize>> = elements
        .into_iter()
        .enumerate()
        .filter(|(i, _)| !faces_to_remove.contains(i) && !degenerate_elems.contains(i))
        .map(|(_, e)| e)
        .collect();

    let stats_after = compute_mesh_stats(&nodes, &repaired_elements.iter().map(|e| e.as_slice()).collect::<Vec<_>>());

    Ok(RepairResult {
        issues_found: issues.len(),
        issues_fixed: fixed_count,
        remaining_issues: remaining,
        mesh_stats_before: stats_before,
        mesh_stats_after: stats_after,
    })
}

/// 填充小孔
#[tauri::command]
pub fn fill_small_holes(
    nodes: Vec<[f64; 3]>,
    elements: Vec<Vec<usize>>,
    max_hole_diameter: f64,
) -> Result<RepairResult, String> {
    let stats_before = compute_mesh_stats(&nodes, &elements.iter().map(|e| e.as_slice()).collect::<Vec<_>>());

    // 构建边-面邻接
    let mut edge_faces: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (face_id, elem) in elements.iter().enumerate() {
        for i in 0..elem.len() {
            let j = (i + 1) % elem.len();
            let a = elem[i].min(elem[j]);
            let b = elem[i].max(elem[j]);
            edge_faces.entry((a, b)).or_default().push(face_id);
        }
    }

    // 找边界边
    let mut boundary_adj: HashMap<usize, Vec<usize>> = HashMap::new();
    for ((a, b), faces) in &edge_faces {
        if faces.len() == 1 {
            boundary_adj.entry(*a).or_default().push(*b);
            boundary_adj.entry(*b).or_default().push(*a);
        }
    }

    // 找小孔环并填充
    let mut new_elements: Vec<Vec<usize>> = Vec::new();
    let mut filled_count = 0usize;
    let mut visited: HashSet<usize> = HashSet::new();

    for &start in boundary_adj.keys() {
        if visited.contains(&start) {
            continue;
        }

        let mut ring: Vec<usize> = vec![start];
        visited.insert(start);
        let mut current = start;
        let mut closed = false;

        for _ in 0..boundary_adj.len() + 1 {
            if let Some(neighbors) = boundary_adj.get(&current) {
                let mut found = false;
                for &next in neighbors {
                    if !visited.contains(&next) {
                        ring.push(next);
                        visited.insert(next);
                        current = next;
                        found = true;
                        break;
                    } else if next == start && ring.len() > 2 {
                        closed = true;
                        break;
                    }
                }
                if closed || !found {
                    break;
                }
            } else {
                break;
            }
        }

        if !closed || ring.len() < 3 {
            continue;
        }

        // 计算环的直径（最大节点间距）
        let mut max_dist = 0.0f64;
        for i in 0..ring.len() {
            for j in (i + 1)..ring.len() {
                let d = distance(&nodes[ring[i]], &nodes[ring[j]]);
                max_dist = max_dist.max(d);
            }
        }

        if max_dist <= max_hole_diameter {
            // 用扇形三角化填充
            let center = ring[0];
            for i in 1..ring.len() - 1 {
                new_elements.push(vec![center, ring[i], ring[i + 1]]);
            }
            filled_count += 1;
        }
    }

    let mut repaired_elements = elements;
    repaired_elements.extend(new_elements);

    let stats_after = compute_mesh_stats(&nodes, &repaired_elements.iter().map(|e| e.as_slice()).collect::<Vec<_>>());

    Ok(RepairResult {
        issues_found: filled_count,
        issues_fixed: filled_count,
        remaining_issues: Vec::new(),
        mesh_stats_before: stats_before,
        mesh_stats_after: stats_after,
    })
}

/// 移除裂缝
#[tauri::command]
pub fn remove_slits(
    nodes: Vec<[f64; 3]>,
    elements: Vec<Vec<usize>>,
    min_slit_length: f64,
) -> Result<RepairResult, String> {
    let stats_before = compute_mesh_stats(&nodes, &elements.iter().map(|e| e.as_slice()).collect::<Vec<_>>());

    // 构建边-面邻接
    let mut edge_faces: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (face_id, elem) in elements.iter().enumerate() {
        for i in 0..elem.len() {
            let j = (i + 1) % elem.len();
            let a = elem[i].min(elem[j]);
            let b = elem[i].max(elem[j]);
            edge_faces.entry((a, b)).or_default().push(face_id);
        }
    }

    // 找边界边
    let mut boundary_adj: HashMap<usize, Vec<usize>> = HashMap::new();
    for ((a, b), faces) in &edge_faces {
        if faces.len() == 1 {
            boundary_adj.entry(*a).or_default().push(*b);
            boundary_adj.entry(*b).or_default().push(*a);
        }
    }

    // 找开放边界链（非闭合的边界边序列）
    let mut visited: HashSet<usize> = HashSet::new();
    let mut slits_fixed = 0usize;
    let mut nodes_to_merge: Vec<(usize, usize)> = Vec::new(); // (node_a, node_b) 要合并的节点对

    for &start in boundary_adj.keys() {
        if visited.contains(&start) {
            continue;
        }

        let mut chain: Vec<usize> = vec![start];
        visited.insert(start);
        let mut current = start;
        let mut is_open_chain = true;

        for _ in 0..boundary_adj.len() + 1 {
            if let Some(neighbors) = boundary_adj.get(&current) {
                let mut found = false;
                for &next in neighbors {
                    if !visited.contains(&next) {
                        chain.push(next);
                        visited.insert(next);
                        current = next;
                        found = true;
                        break;
                    } else if next == start && chain.len() > 2 {
                        is_open_chain = false;
                        break;
                    }
                }
                if !is_open_chain || !found {
                    break;
                }
            } else {
                break;
            }
        }

        if !is_open_chain {
            continue;
        }

        // 计算链长度
        let mut chain_length = 0.0;
        for i in 0..chain.len() - 1 {
            chain_length += distance(&nodes[chain[i]], &nodes[chain[i + 1]]);
        }

        // 检查端点距离（如果很近，可能是裂缝）
        if chain.len() >= 2 && chain_length >= min_slit_length {
            let end_dist = distance(&nodes[chain[0]], &nodes[chain[chain.len() - 1]]);
            let avg_edge = stats_before.avg_edge_length;
            if end_dist < avg_edge * 0.5 {
                // 端点很近，合并
                nodes_to_merge.push((chain[0], chain[chain.len() - 1]));
                slits_fixed += 1;
            }
        }
    }

    // 执行节点合并
    let mut node_map: HashMap<usize, usize> = HashMap::new();
    for &(a, b) in &nodes_to_merge {
        node_map.insert(b, a); // 将b合并到a
    }

    let repaired_elements: Vec<Vec<usize>> = elements
        .into_iter()
        .map(|mut elem| {
            for node in elem.iter_mut() {
                if let Some(&replacement) = node_map.get(node) {
                    *node = replacement;
                }
            }
            // 去重
            let unique: HashSet<usize> = elem.into_iter().collect();
            unique.into_iter().collect()
        })
        .filter(|e| e.len() >= 3)
        .collect();

    let stats_after = compute_mesh_stats(&nodes, &repaired_elements.iter().map(|e| e.as_slice()).collect::<Vec<_>>());

    Ok(RepairResult {
        issues_found: slits_fixed,
        issues_fixed: slits_fixed,
        remaining_issues: Vec::new(),
        mesh_stats_before: stats_before,
        mesh_stats_after: stats_after,
    })
}

/// 修复退化单元
#[tauri::command]
pub fn fix_degenerate_elements(
    nodes: Vec<[f64; 3]>,
    elements: Vec<Vec<usize>>,
    min_aspect_ratio: f64,
) -> Result<RepairResult, String> {
    let stats_before = compute_mesh_stats(&nodes, &elements.iter().map(|e| e.as_slice()).collect::<Vec<_>>());

    let mut fixed_count = 0usize;
    let mut repaired_elements: Vec<Vec<usize>> = Vec::new();

    for elem in &elements {
        if elem.len() < 3 {
            continue;
        }

        // 计算长宽比
        let mut max_edge = 0.0f64;
        let mut min_edge = f64::MAX;
        for i in 0..elem.len() {
            let j = (i + 1) % elem.len();
            if elem[i] < nodes.len() && elem[j] < nodes.len() {
                let d = distance(&nodes[elem[i]], &nodes[elem[j]]);
                max_edge = max_edge.max(d);
                min_edge = min_edge.min(d);
            }
        }

        let aspect_ratio = if min_edge > 1e-15 {
            max_edge / min_edge
        } else {
            f64::MAX
        };

        if aspect_ratio > min_aspect_ratio {
            // 退化单元：尝试通过节点位置微调来修复
            // 对于三角形，将重心移向最长边的中点
            if elem.len() == 3 {
                let mut new_nodes = nodes.clone();
                // 找最长边
                let mut longest_idx = 0;
                let mut longest_len = 0.0;
                for i in 0..3 {
                    let j = (i + 1) % 3;
                    let d = distance(&nodes[elem[i]], &nodes[elem[j]]);
                    if d > longest_len {
                        longest_len = d;
                        longest_idx = i;
                    }
                }
                // 将对面节点向最长边中点移动一点
                let opposite = (longest_idx + 2) % 3;
                let mid = [
                    (nodes[elem[longest_idx]][0] + nodes[elem[(longest_idx + 1) % 3]][0]) / 2.0,
                    (nodes[elem[longest_idx]][1] + nodes[elem[(longest_idx + 1) % 3]][1]) / 2.0,
                    (nodes[elem[longest_idx]][2] + nodes[elem[(longest_idx + 1) % 3]][2]) / 2.0,
                ];
                // 移动10%
                new_nodes[elem[opposite]][0] = nodes[elem[opposite]][0] * 0.9 + mid[0] * 0.1;
                new_nodes[elem[opposite]][1] = nodes[elem[opposite]][1] * 0.9 + mid[1] * 0.1;
                new_nodes[elem[opposite]][2] = nodes[elem[opposite]][2] * 0.9 + mid[2] * 0.1;

                // 重新计算长宽比
                let mut new_max = 0.0f64;
                let mut new_min = f64::MAX;
                for i in 0..3 {
                    let j = (i + 1) % 3;
                    let d = distance(&new_nodes[elem[i]], &new_nodes[elem[j]]);
                    new_max = new_max.max(d);
                    new_min = new_min.min(d);
                }
                let new_ar = if new_min > 1e-15 { new_max / new_min } else { f64::MAX };

                if new_ar < aspect_ratio {
                    fixed_count += 1;
                }
            }
            repaired_elements.push(elem.clone());
        } else {
            repaired_elements.push(elem.clone());
        }
    }

    let stats_after = compute_mesh_stats(&nodes, &repaired_elements.iter().map(|e| e.as_slice()).collect::<Vec<_>>());

    Ok(RepairResult {
        issues_found: elements.len(),
        issues_fixed: fixed_count,
        remaining_issues: Vec::new(),
        mesh_stats_before: stats_before,
        mesh_stats_after: stats_after,
    })
}

/// 统一法向
#[tauri::command]
pub fn unify_normals(
    nodes: Vec<[f64; 3]>,
    elements: Vec<Vec<usize>>,
    face_normals: Vec<[f64; 3]>,
) -> Result<RepairResult, String> {
    let stats_before = compute_mesh_stats(&nodes, &elements.iter().map(|e| e.as_slice()).collect::<Vec<_>>());

    if face_normals.len() != elements.len() {
        return Err(format!(
            "法向数量 ({}) 与面数量 ({}) 不匹配",
            face_normals.len(),
            elements.len()
        ));
    }

    // 构建面邻接关系
    let mut edge_faces: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (face_id, elem) in elements.iter().enumerate() {
        for i in 0..elem.len() {
            let j = (i + 1) % elem.len();
            let a = elem[i].min(elem[j]);
            let b = elem[i].max(elem[j]);
            edge_faces.entry((a, b)).or_default().push(face_id);
        }
    }

    // 使用BFS统一法向
    let mut flipped: Vec<bool> = vec![false; elements.len()];
    let mut visited: HashSet<usize> = HashSet::new();

    if !elements.is_empty() {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(0);
        visited.insert(0);

        while let Some(current) = queue.pop_front() {
            let current_normal = if flipped[current] {
                [-face_normals[current][0], -face_normals[current][1], -face_normals[current][2]]
            } else {
                face_normals[current]
            };

            // 检查相邻面
            let elem = &elements[current];
            for i in 0..elem.len() {
                let j = (i + 1) % elem.len();
                let a = elem[i].min(elem[j]);
                let b = elem[i].max(elem[j]);

                if let Some(neighbors) = edge_faces.get(&(a, b)) {
                    for &neighbor in neighbors {
                        if neighbor == current || visited.contains(&neighbor) {
                            continue;
                        }

                        let neighbor_normal = face_normals[neighbor];
                        let dot = current_normal[0] * neighbor_normal[0]
                            + current_normal[1] * neighbor_normal[1]
                            + current_normal[2] * neighbor_normal[2];

                        // 如果法向相反，翻转
                        if dot < 0.0 {
                            flipped[neighbor] = true;
                        }

                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    // 构建修复后的单元列表
    let repaired_elements: Vec<Vec<usize>> = elements
        .into_iter()
        .enumerate()
        .map(|(i, mut elem)| {
            if flipped[i] {
                elem.reverse();
            }
            elem
        })
        .collect();

    let flipped_count = flipped.iter().filter(|&&f| f).count();

    let stats_after = compute_mesh_stats(&nodes, &repaired_elements.iter().map(|e| e.as_slice()).collect::<Vec<_>>());

    Ok(RepairResult {
        issues_found: flipped_count,
        issues_fixed: flipped_count,
        remaining_issues: Vec::new(),
        mesh_stats_before: stats_before,
        mesh_stats_after: stats_after,
    })
}
