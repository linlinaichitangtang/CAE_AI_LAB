use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StepImportResult {
    pub success: bool,
    pub nodes: Vec<Vec<f64>>,
    pub elements: Vec<Vec<usize>>,
    pub element_type: String,
    pub num_nodes: usize,
    pub num_elements: usize,
    pub bounding_box: Option<[f64; 6]>, // [xmin, ymin, zmin, xmax, ymax, zmax]
    pub warnings: Vec<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StepImportAvailability {
    pub gmsh_available: bool,
    pub freecad_available: bool,
    pub recommended_tool: Option<String>,
}

/// 检测可用的 CAD 转换工具
fn detect_converter() -> Result<String, String> {
    // 优先级：gmsh > freecad
    if Command::new("gmsh").arg("--version").output().is_ok() {
        return Ok("gmsh".to_string());
    }
    if Command::new("freecad").arg("--version").output().map_or(false, |o| o.status.success()) {
        return Ok("freecad".to_string());
    }
    Err("未找到 CAD 转换工具。请安装 GMSH (https://gmsh.info) 或 FreeCAD (https://freecad.org)".to_string())
}

/// 使用 gmsh 将 STEP/IGES 转换为 INP 格式
fn convert_with_gmsh(step_path: &PathBuf, output_dir: &PathBuf, mesh_size: Option<f64>) -> Result<PathBuf, String> {
    let inp_path = output_dir.join("imported_mesh.inp");

    let mut args: Vec<String> = Vec::new();
    args.push(step_path.to_str().unwrap().to_string());
    args.push("-format".to_string());
    args.push("inp".to_string());
    args.push("-o".to_string());
    args.push(inp_path.to_str().unwrap().to_string());
    args.push("-3".to_string()); // 3D mesh
    if let Some(size) = mesh_size {
        args.push("-meshsize".to_string());
        args.push(size.to_string());
    } else {
        args.push("-meshsize".to_string());
        args.push("1.0".to_string());
    }

    let output = Command::new("gmsh")
        .args(&args)
        .output()
        .map_err(|e| format!("GMSH 执行失败: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("GMSH 转换失败: {}", stderr));
    }

    Ok(inp_path)
}

/// 使用 FreeCAD 将 STEP/IGES 转换为 STL
fn convert_with_freecad(step_path: &PathBuf, output_dir: &PathBuf) -> Result<PathBuf, String> {
    let stl_path = output_dir.join("imported_mesh.stl");

    let script = format!(
        r#"
import FreeCAD
import Mesh
import MeshPart
import sys

doc = FreeCAD.openDocument(sys.argv[1])
for obj in doc.Objects:
    mesh = MeshPart.meshFromShape(Shape=obj.Shape, LinearDeflection=0.1, AngularDeflection=0.5)
    mesh.write(r"{}")
FreeCAD.closeDocument(doc.Name)
"#,
        stl_path.to_str().unwrap()
    );

    let script_path = output_dir.join("convert_step.py");
    std::fs::write(&script_path, &script).map_err(|e| format!("写入脚本失败: {}", e))?;

    let output = Command::new("freecad")
        .args(&[
            "-c",
            script_path.to_str().unwrap(),
            step_path.to_str().unwrap(),
        ])
        .output()
        .map_err(|e| format!("FreeCAD 执行失败: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("FreeCAD 转换失败: {}", stderr));
    }

    Ok(stl_path)
}

/// 解析 CalculiX INP 格式的网格数据
fn parse_inp_mesh(inp_path: &PathBuf) -> Result<StepImportResult, String> {
    let content = std::fs::read_to_string(inp_path)
        .map_err(|e| format!("读取 INP 文件失败: {}", e))?;

    let mut nodes: Vec<Vec<f64>> = Vec::new();
    let mut elements: Vec<Vec<usize>> = Vec::new();
    let mut element_type = String::from("C3D4"); // 默认四面体
    let mut warnings: Vec<String> = Vec::new();
    let mut in_node_section = false;
    let mut in_element_section = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // 跳过注释和空行
        if trimmed.starts_with("**") || trimmed.is_empty() {
            continue;
        }

        // 检测节点段开始
        if trimmed.starts_with("*NODE") || trimmed.starts_with("*Node") {
            in_node_section = true;
            in_element_section = false;
            continue;
        }

        // 检测单元段开始
        if trimmed.starts_with("*ELEMENT") || trimmed.starts_with("*Element") {
            in_element_section = true;
            in_node_section = false;

            // 从 *ELEMENT, TYPE=C3D8 等行提取单元类型
            if let Some(type_str) = trimmed.to_uppercase().split("TYPE=").nth(1) {
                let et = type_str.split(',').next().unwrap_or("").trim();
                if !et.is_empty() {
                    element_type = et.to_string();
                }
            }
            continue;
        }

        // 其他关键字段结束当前段
        if trimmed.starts_with('*') && !trimmed.starts_with("*NODE") && !trimmed.starts_with("*Element") {
            if in_node_section || in_element_section {
                in_node_section = false;
                in_element_section = false;
            }
            continue;
        }

        // 解析节点数据: node_id, x, y, z
        if in_node_section {
            let parts: Vec<&str> = trimmed
                .split(',')
                .map(|s| s.trim())
                .collect();
            if parts.len() >= 4 {
                if let (Ok(_id), Ok(x), Ok(y), Ok(z)) = (
                    parts[0].parse::<i64>(),
                    parts[1].parse::<f64>(),
                    parts[2].parse::<f64>(),
                    parts[3].parse::<f64>(),
                ) {
                    nodes.push(vec![x, y, z]);
                }
            }
        }

        // 解析单元数据: elem_id, n1, n2, n3, ...
        if in_element_section {
            let parts: Vec<&str> = trimmed
                .split(',')
                .map(|s| s.trim())
                .collect();
            if parts.len() >= 2 {
                let mut node_ids: Vec<usize> = Vec::new();
                // 第一个是单元ID，跳过
                for part in parts.iter().skip(1) {
                    if let Ok(id) = part.parse::<usize>() {
                        node_ids.push(id);
                    }
                }
                if !node_ids.is_empty() {
                    elements.push(node_ids);
                }
            }
        }
    }

    if nodes.is_empty() {
        return Err("INP 文件中未找到节点数据".to_string());
    }

    if elements.is_empty() {
        return Err("INP 文件中未找到单元数据".to_string());
    }

    // 计算包围盒
    let mut xmin = f64::MAX;
    let mut ymin = f64::MAX;
    let mut zmin = f64::MAX;
    let mut xmax = f64::MIN;
    let mut ymax = f64::MIN;
    let mut zmax = f64::MIN;

    for node in &nodes {
        if node.len() >= 3 {
            xmin = xmin.min(node[0]);
            ymin = ymin.min(node[1]);
            zmin = zmin.min(node[2]);
            xmax = xmax.max(node[0]);
            ymax = ymax.max(node[1]);
            zmax = zmax.max(node[2]);
        }
    }

    let num_nodes = nodes.len();
    let num_elements = elements.len();

    if num_nodes > 500000 {
        warnings.push(format!(
            "网格节点数较多 ({}), 可能影响显示性能",
            num_nodes
        ));
    }

    Ok(StepImportResult {
        success: true,
        nodes,
        elements,
        element_type,
        num_nodes,
        num_elements,
        bounding_box: Some([xmin, ymin, zmin, xmax, ymax, zmax]),
        warnings,
        error: None,
    })
}

/// 解析 STL 格式的网格数据
fn parse_stl_mesh(stl_path: &PathBuf) -> Result<StepImportResult, String> {
    let content = std::fs::read_to_string(stl_path);
    let mut warnings: Vec<String> = Vec::new();

    // 尝试 ASCII 格式
    if let Ok(text) = content {
        if text.to_uppercase().contains("SOLID") {
            return parse_stl_ascii(&text, &mut warnings);
        }
    }

    // 尝试二进制格式
    let binary_data = std::fs::read(stl_path)
        .map_err(|e| format!("读取 STL 文件失败: {}", e))?;
    parse_stl_binary(&binary_data, &mut warnings)
}

/// 解析 ASCII STL 格式
fn parse_stl_ascii(text: &str, warnings: &mut Vec<String>) -> Result<StepImportResult, String> {
    use std::collections::HashMap;

    // 使用 HashMap 去重节点（STL 中三角形单元共享顶点）
    let mut node_map: HashMap<String, usize> = HashMap::new(); // "x,y,z" -> index
    let mut nodes: Vec<Vec<f64>> = Vec::new();
    let mut elements: Vec<Vec<usize>> = Vec::new();

    let mut current_vertices: Vec<[f64; 3]> = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("facet normal") {
            current_vertices.clear();
        } else if trimmed.starts_with("vertex") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 4 {
                if let (Ok(x), Ok(y), Ok(z)) = (
                    parts[1].parse::<f64>(),
                    parts[2].parse::<f64>(),
                    parts[3].parse::<f64>(),
                ) {
                    current_vertices.push([x, y, z]);
                }
            }
        } else if trimmed == "endfacet" {
            // 一个三角形面片结束，添加节点和单元
            if current_vertices.len() == 3 {
                let mut elem: Vec<usize> = Vec::new();
                for v in &current_vertices {
                    let key = format!("{},{},{}", v[0], v[1], v[2]);
                    let idx = if let Some(&existing) = node_map.get(&key) {
                        existing
                    } else {
                        let new_idx = nodes.len();
                        nodes.push(vec![v[0], v[1], v[2]]);
                        node_map.insert(key, new_idx);
                        new_idx
                    };
                    elem.push(idx);
                }
                elements.push(elem);
            }
        }
    }

    if nodes.is_empty() {
        return Err("STL 文件中未找到有效的三角形数据".to_string());
    }

    // 计算包围盒
    let mut xmin = f64::MAX;
    let mut ymin = f64::MAX;
    let mut zmin = f64::MAX;
    let mut xmax = f64::MIN;
    let mut ymax = f64::MIN;
    let mut zmax = f64::MIN;

    for node in &nodes {
        if node.len() >= 3 {
            xmin = xmin.min(node[0]);
            ymin = ymin.min(node[1]);
            zmin = zmin.min(node[2]);
            xmax = xmax.max(node[0]);
            ymax = ymax.max(node[1]);
            zmax = zmax.max(node[2]);
        }
    }

    let num_nodes = nodes.len();
    let num_elements = elements.len();

    if num_nodes > 500000 {
        warnings.push(format!(
            "网格节点数较多 ({}), 可能影响显示性能",
            num_nodes
        ));
    }

    Ok(StepImportResult {
        success: true,
        nodes,
        elements,
        element_type: "C3D4".to_string(), // STL 三角面片映射为四面体标记
        num_nodes,
        num_elements,
        bounding_box: Some([xmin, ymin, zmin, xmax, ymax, zmax]),
        warnings: warnings.clone(),
        error: None,
    })
}

/// 解析二进制 STL 格式
fn parse_stl_binary(data: &[u8], warnings: &mut Vec<String>) -> Result<StepImportResult, String> {
    use std::collections::HashMap;

    if data.len() < 84 {
        return Err("STL 文件太小，不是有效的二进制 STL 格式".to_string());
    }

    let num_triangles =
        u32::from_le_bytes([data[80], data[81], data[82], data[83]]) as usize;

    if data.len() < 84 + num_triangles * 50 {
        return Err("STL 文件数据不完整".to_string());
    }

    let mut node_map: HashMap<String, usize> = HashMap::new();
    let mut nodes: Vec<Vec<f64>> = Vec::new();
    let mut elements: Vec<Vec<usize>> = Vec::new();

    for i in 0..num_triangles {
        let offset = 84 + i * 50;

        // 跳过法向量 (12 bytes), 读取3个顶点
        let mut elem: Vec<usize> = Vec::new();
        for v in 0..3 {
            let v_offset = offset + 12 + v * 12;
            let x = f32::from_le_bytes([
                data[v_offset],
                data[v_offset + 1],
                data[v_offset + 2],
                data[v_offset + 3],
            ]) as f64;
            let y = f32::from_le_bytes([
                data[v_offset + 4],
                data[v_offset + 5],
                data[v_offset + 6],
                data[v_offset + 7],
            ]) as f64;
            let z = f32::from_le_bytes([
                data[v_offset + 8],
                data[v_offset + 9],
                data[v_offset + 10],
                data[v_offset + 11],
            ]) as f64;

            let key = format!("{},{},{}", x, y, z);
            let idx = if let Some(&existing) = node_map.get(&key) {
                existing
            } else {
                let new_idx = nodes.len();
                nodes.push(vec![x, y, z]);
                node_map.insert(key, new_idx);
                new_idx
            };
            elem.push(idx);
        }
        elements.push(elem);
    }

    if nodes.is_empty() {
        return Err("STL 文件中未找到有效的三角形数据".to_string());
    }

    // 计算包围盒
    let mut xmin = f64::MAX;
    let mut ymin = f64::MAX;
    let mut zmin = f64::MAX;
    let mut xmax = f64::MIN;
    let mut ymax = f64::MIN;
    let mut zmax = f64::MIN;

    for node in &nodes {
        if node.len() >= 3 {
            xmin = xmin.min(node[0]);
            ymin = ymin.min(node[1]);
            zmin = zmin.min(node[2]);
            xmax = xmax.max(node[0]);
            ymax = ymax.max(node[1]);
            zmax = zmax.max(node[2]);
        }
    }

    let num_nodes = nodes.len();
    let num_elements = elements.len();

    if num_nodes > 500000 {
        warnings.push(format!(
            "网格节点数较多 ({}), 可能影响显示性能",
            num_nodes
        ));
    }

    Ok(StepImportResult {
        success: true,
        nodes,
        elements,
        element_type: "C3D4".to_string(),
        num_nodes,
        num_elements,
        bounding_box: Some([xmin, ymin, zmin, xmax, ymax, zmax]),
        warnings: warnings.clone(),
        error: None,
    })
}

#[tauri::command]
pub async fn import_step_file(
    file_path: String,
    mesh_size: Option<f64>,
) -> Result<StepImportResult, String> {
    let path = PathBuf::from(&file_path);
    if !path.exists() {
        return Err(format!("文件不存在: {}", file_path));
    }

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if ext != "step"
        && ext != "stp"
        && ext != "iges"
        && ext != "igs"
        && ext != "stl"
    {
        return Err(format!("不支持的文件格式: .{}", ext));
    }

    // 如果是 STL 文件，直接解析
    if ext == "stl" {
        return parse_stl_mesh(&path);
    }

    // STEP/IGES 文件需要转换
    let converter = detect_converter()?;
    let output_dir = path.parent().unwrap().to_path_buf();

    let result = match converter.as_str() {
        "gmsh" => {
            let inp_path = convert_with_gmsh(&path, &output_dir, mesh_size)?;
            parse_inp_mesh(&inp_path)
        }
        "freecad" => {
            let stl_path = convert_with_freecad(&path, &output_dir)?;
            parse_stl_mesh(&stl_path)
        }
        _ => Err("不支持的转换工具".to_string()),
    };

    result
}

#[tauri::command]
pub fn check_step_import_available() -> StepImportAvailability {
    let gmsh_available = Command::new("gmsh")
        .arg("--version")
        .output()
        .is_ok();

    let freecad_available = Command::new("freecad")
        .arg("--version")
        .output()
        .map_or(false, |o| o.status.success());

    let recommended_tool = detect_converter().ok();

    StepImportAvailability {
        gmsh_available,
        freecad_available,
        recommended_tool,
    }
}
