use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};

// ============================================================================
// V1.2-009: Abaqus INP 完整模型结构体
// ============================================================================

/// 完整的 Abaqus INP 模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AbaqusInpModel {
    pub nodes: Vec<InpNode>,
    pub elements: Vec<InpElement>,
    pub materials: Vec<InpMaterial>,
    pub sections: Vec<InpSection>,
    pub boundaries: Vec<InpBoundary>,
    pub loads: Vec<InpLoad>,
    pub steps: Vec<InpStep>,
    pub contact_pairs: Vec<InpContactPair>,
    pub node_sets: HashMap<String, Vec<usize>>,
    pub element_sets: HashMap<String, Vec<usize>>,
    pub amplitudes: Vec<InpAmplitude>,
    pub initial_conditions: Vec<InpInitialCondition>,
    pub warnings: Vec<String>,
}

impl Default for AbaqusInpModel {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            elements: Vec::new(),
            materials: Vec::new(),
            sections: Vec::new(),
            boundaries: Vec::new(),
            loads: Vec::new(),
            steps: Vec::new(),
            contact_pairs: Vec::new(),
            node_sets: HashMap::new(),
            element_sets: HashMap::new(),
            amplitudes: Vec::new(),
            initial_conditions: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

/// INP 节点
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpNode {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// INP 单元
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpElement {
    pub id: usize,
    pub element_type: String,
    pub node_ids: Vec<usize>,
}

/// INP 材料定义
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpMaterial {
    pub name: String,
    pub elastic_modulus: Option<f64>,
    pub poisson_ratio: Option<f64>,
    pub density: Option<f64>,
    pub thermal_expansion: Option<f64>,
    pub thermal_conductivity: Option<f64>,
    pub specific_heat: Option<f64>,
    pub yield_stress: Option<f64>,
    pub plastic_strain: Option<f64>,
    pub raw_properties: HashMap<String, Vec<f64>>,
}

/// INP 截面定义
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpSection {
    pub section_type: String, // "SOLID", "SHELL", "BEAM"
    pub material: String,
    pub element_set: Option<String>,
    pub thickness: Option<f64>,
}

/// INP 边界条件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpBoundary {
    pub node_set: Option<String>,
    pub node_id: Option<usize>,
    pub dof: String,        // "1", "2", "3", "123", "ENCASTRE"
    pub magnitude: Option<f64>,
    pub step_name: Option<String>,
}

/// INP 载荷
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpLoad {
    pub load_type: String,  // "CLOAD", "DLOAD", "PRESSURE"
    pub node_set: Option<String>,
    pub node_id: Option<usize>,
    pub element_set: Option<String>,
    pub dof: Option<String>,
    pub magnitude: f64,
    pub step_name: Option<String>,
    pub amplitude: Option<String>,
}

/// INP 分析步骤
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpStep {
    pub name: String,
    pub step_type: String,  // "STATIC", "DYNAMIC", "HEAT_TRANSFER", etc.
    pub nlgeom: bool,
    pub description: Option<String>,
}

/// INP 接触对
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpContactPair {
    pub master_surface: String,
    pub slave_surface: String,
    pub interaction: String,
    pub step_name: Option<String>,
}

/// INP 幅值定义
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpAmplitude {
    pub name: String,
    pub time_values: Vec<f64>,
    pub amplitude_values: Vec<f64>,
}

/// INP 初始条件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InpInitialCondition {
    pub condition_type: String,
    pub node_set: Option<String>,
    pub value: f64,
}

/// Abaqus INP 导入结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AbaqusInpImportResult {
    pub success: bool,
    pub model: AbaqusInpModel,
    pub num_nodes: usize,
    pub num_elements: usize,
    pub num_materials: usize,
    pub num_sections: usize,
    pub num_boundaries: usize,
    pub num_loads: usize,
    pub num_steps: usize,
    pub num_contact_pairs: usize,
    pub num_node_sets: usize,
    pub num_element_sets: usize,
    pub bounding_box: Option<[f64; 6]>,
    pub warnings: Vec<String>,
    pub error: Option<String>,
}

// ============================================================================
// 原有结构体
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StepImportResult {
    pub success: bool,
    pub nodes: Vec<Vec<f64>>,
    pub elements: Vec<Vec<usize>>,
    pub element_type: String,
    pub num_nodes: usize,
    pub num_elements: usize,
    pub bounding_box: Option<[f64; 6]>,
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

// ============================================================================
// V1.2-009: 完整 Abaqus INP 解析
// ============================================================================

/// 从关键字行中提取参数值
fn extract_keyword_param(line: &str, param: &str) -> Option<String> {
    let upper = line.to_uppercase();
    let search = format!("{}=", param);
    if let Some(pos) = upper.find(&search) {
        let rest = &line[pos + search.len()..];
        // 取到下一个逗号或行尾
        let end = rest.find(',').unwrap_or(rest.len());
        let val = rest[..end].trim().to_string();
        if !val.is_empty() {
            return Some(val);
        }
    }
    None
}

/// 从关键字行中提取名称 (NAME=xxx)
fn extract_name(line: &str) -> Option<String> {
    extract_keyword_param(line, "NAME")
}

/// 从关键字行中提取 ELSET 或 NSET
fn extract_set_name(line: &str, set_type: &str) -> Option<String> {
    extract_keyword_param(line, set_type)
}

/// 解析数据行中的数值
fn parse_data_values(line: &str) -> Vec<f64> {
    line.split(',')
        .map(|s| s.trim())
        .filter_map(|s| s.parse::<f64>().ok())
        .collect()
}

/// 解析数据行中的整数值
fn parse_int_values(line: &str) -> Vec<usize> {
    line.split(',')
        .map(|s| s.trim())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect()
}

/// 解析完整的 Abaqus INP 文件
pub fn parse_abaqus_inp(content: &str) -> Result<AbaqusInpModel, String> {
    let mut model = AbaqusInpModel::default();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    // 当前解析状态
    let mut current_keyword = String::new();
    let mut current_element_type = String::new();
    let mut current_material_name: String;
    let mut current_step_name = String::new();
    let mut current_nset_name: Option<String> = None;
    let mut current_elset_name: Option<String> = None;
    let mut current_amplitude_name: String;
    let mut amplitude_parsing_time = true;
    let mut nset_generate = false;
    let mut elset_generate = false;

    while i < lines.len() {
        let trimmed = lines[i].trim();

        // 跳过注释和空行
        if trimmed.starts_with("**") || trimmed.is_empty() {
            i += 1;
            continue;
        }

        // 关键字行
        if trimmed.starts_with('*') {
            let upper = trimmed.to_uppercase();

            // 解析 *NODE 段
            if upper.starts_with("*NODE") {
                current_keyword = "NODE".to_string();
                current_nset_name = extract_set_name(trimmed, "NSET");
                i += 1;
                continue;
            }

            // 解析 *ELEMENT 段
            if upper.starts_with("*ELEMENT") {
                current_keyword = "ELEMENT".to_string();
                current_element_type = extract_keyword_param(trimmed, "TYPE")
                    .unwrap_or_else(|| "C3D4".to_string());
                current_elset_name = extract_set_name(trimmed, "ELSET");
                elset_generate = upper.contains("GENERATE");
                i += 1;
                continue;
            }

            // 解析 *MATERIAL 段
            if upper.starts_with("*MATERIAL") {
                current_keyword = "MATERIAL".to_string();
                current_material_name = extract_name(trimmed)
                    .unwrap_or_else(|| format!("Material_{}", model.materials.len() + 1));
                model.materials.push(InpMaterial {
                    name: current_material_name.clone(),
                    elastic_modulus: None,
                    poisson_ratio: None,
                    density: None,
                    thermal_expansion: None,
                    thermal_conductivity: None,
                    specific_heat: None,
                    yield_stress: None,
                    plastic_strain: None,
                    raw_properties: HashMap::new(),
                });
                i += 1;
                continue;
            }

            // 材料属性子关键字
            if upper.starts_with("*ELASTIC") {
                current_keyword = "ELASTIC".to_string();
                i += 1;
                continue;
            }

            if upper.starts_with("*DENSITY") {
                current_keyword = "DENSITY".to_string();
                i += 1;
                continue;
            }

            if upper.starts_with("*EXPANSION") {
                current_keyword = "EXPANSION".to_string();
                i += 1;
                continue;
            }

            if upper.starts_with("*CONDUCTIVITY") || upper.contains("*CONDUCTIVITY") {
                current_keyword = "CONDUCTIVITY".to_string();
                i += 1;
                continue;
            }

            if upper.starts_with("*SPECIFIC HEAT") || upper.contains("*SPECIFIC HEAT") {
                current_keyword = "SPECIFIC_HEAT".to_string();
                i += 1;
                continue;
            }

            if upper.starts_with("*PLASTIC") {
                current_keyword = "PLASTIC".to_string();
                i += 1;
                continue;
            }

            // *SOLID SECTION
            if upper.starts_with("*SOLID SECTION") {
                current_keyword = "SOLID_SECTION".to_string();
                if let Some(mat) = extract_keyword_param(trimmed, "MATERIAL") {
                    let elset = extract_set_name(trimmed, "ELSET");
                    model.sections.push(InpSection {
                        section_type: "SOLID".to_string(),
                        material: mat,
                        element_set: elset,
                        thickness: None,
                    });
                }
                i += 1;
                continue;
            }

            // *SHELL SECTION
            if upper.starts_with("*SHELL SECTION") || upper.starts_with("*SHELL GENERAL SECTION") {
                current_keyword = "SHELL_SECTION".to_string();
                if let Some(mat) = extract_keyword_param(trimmed, "MATERIAL") {
                    let elset = extract_set_name(trimmed, "ELSET");
                    model.sections.push(InpSection {
                        section_type: "SHELL".to_string(),
                        material: mat,
                        element_set: elset,
                        thickness: None,
                    });
                }
                i += 1;
                continue;
            }

            // *BEAM SECTION
            if upper.starts_with("*BEAM SECTION") {
                current_keyword = "BEAM_SECTION".to_string();
                if let Some(mat) = extract_keyword_param(trimmed, "MATERIAL") {
                    let elset = extract_set_name(trimmed, "ELSET");
                    model.sections.push(InpSection {
                        section_type: "BEAM".to_string(),
                        material: mat,
                        element_set: elset,
                        thickness: None,
                    });
                }
                i += 1;
                continue;
            }

            // *BOUNDARY
            if upper.starts_with("*BOUNDARY") {
                current_keyword = "BOUNDARY".to_string();
                i += 1;
                continue;
            }

            // *CLOAD
            if upper.starts_with("*CLOAD") {
                current_keyword = "CLOAD".to_string();
                i += 1;
                continue;
            }

            // *DLOAD
            if upper.starts_with("*DLOAD") {
                current_keyword = "DLOAD".to_string();
                i += 1;
                continue;
            }

            // *DSLOAD (surface distributed load in Abaqus)
            if upper.starts_with("*DSLOAD") {
                current_keyword = "DSLOAD".to_string();
                i += 1;
                continue;
            }

            // *PRESSURE
            if upper.starts_with("*PRESSURE") {
                current_keyword = "PRESSURE".to_string();
                i += 1;
                continue;
            }

            // *STEP
            if upper.starts_with("*STEP") {
                current_keyword = "STEP".to_string();
                current_step_name = extract_name(trimmed)
                    .unwrap_or_else(|| format!("Step-{}", model.steps.len() + 1));
                model.steps.push(InpStep {
                    name: current_step_name.clone(),
                    step_type: String::new(),
                    nlgeom: false,
                    description: None,
                });
                i += 1;
                continue;
            }

            // *STATIC, *DYNAMIC, *HEAT TRANSFER, etc. (分析过程)
            if upper.starts_with("*STATIC") || upper.starts_with("*DYNAMIC")
                || upper.starts_with("*HEAT TRANSFER") || upper.starts_with("*COUPLED TEMPERATURE-DISPLACEMENT")
                || upper.starts_with("*FREQUENCY") || upper.starts_with("*BUCKLE")
                || upper.starts_with("*MODAL DYNAMIC") || upper.starts_with("*STEADY STATE TRANSPORT")
                || upper.starts_with("*COUPLED THERMAL-ELECTRICAL")
            {
                if let Some(step) = model.steps.last_mut() {
                    let proc_type = if upper.starts_with("*STATIC") {
                        "STATIC".to_string()
                    } else if upper.starts_with("*DYNAMIC") {
                        "DYNAMIC".to_string()
                    } else if upper.starts_with("*HEAT TRANSFER") {
                        "HEAT_TRANSFER".to_string()
                    } else if upper.starts_with("*FREQUENCY") {
                        "FREQUENCY".to_string()
                    } else if upper.starts_with("*BUCKLE") {
                        "BUCKLE".to_string()
                    } else {
                        upper.trim_start_matches('*').split(',').next().unwrap_or("UNKNOWN").trim().to_string()
                    };
                    step.step_type = proc_type;
                    step.nlgeom = upper.contains("NLGEOM") || upper.contains("NLGEOM=YES");
                }
                current_keyword = "PROCEDURE".to_string();
                i += 1;
                continue;
            }

            // *AMPLITUDE
            if upper.starts_with("*AMPLITUDE") {
                current_keyword = "AMPLITUDE".to_string();
                current_amplitude_name = extract_name(trimmed)
                    .unwrap_or_else(|| format!("Amp-{}", model.amplitudes.len() + 1));
                amplitude_parsing_time = true;
                model.amplitudes.push(InpAmplitude {
                    name: current_amplitude_name.clone(),
                    time_values: Vec::new(),
                    amplitude_values: Vec::new(),
                });
                i += 1;
                continue;
            }

            // *CONTACT PAIR
            if upper.starts_with("*CONTACT PAIR") {
                current_keyword = "CONTACT_PAIR".to_string();
                let interaction = extract_keyword_param(trimmed, "INTERACTION")
                    .unwrap_or_default();
                // 解析接触对行: *CONTACT PAIR, INTERACTION=Int1, slave, master
                let parts: Vec<&str> = trimmed.split(',').collect();
                let mut slave = String::new();
                let mut master = String::new();
                if parts.len() >= 3 {
                    slave = parts[parts.len() - 2].trim().to_string();
                    master = parts[parts.len() - 1].trim().to_string();
                }
                model.contact_pairs.push(InpContactPair {
                    master_surface: master,
                    slave_surface: slave,
                    interaction,
                    step_name: if current_step_name.is_empty() { None } else { Some(current_step_name.clone()) },
                });
                i += 1;
                continue;
            }

            // *NSET
            if upper.starts_with("*NSET") {
                current_keyword = "NSET".to_string();
                current_nset_name = extract_set_name(trimmed, "NSET");
                nset_generate = upper.contains("GENERATE");
                i += 1;
                continue;
            }

            // *ELSET
            if upper.starts_with("*ELSET") {
                current_keyword = "ELSET".to_string();
                current_elset_name = extract_set_name(trimmed, "ELSET");
                elset_generate = upper.contains("GENERATE");
                i += 1;
                continue;
            }

            // *INITIAL CONDITIONS
            if upper.starts_with("*INITIAL CONDITIONS") {
                current_keyword = "INITIAL_CONDITIONS".to_string();
                i += 1;
                continue;
            }

            // *ORIENTATION, *SURFACE, *SURFACE INTERACTION 等跳过
            if upper.starts_with("*ORIENTATION") || upper.starts_with("*SURFACE")
                || upper.starts_with("*SURFACE INTERACTION") || upper.starts_with("*EQUATION")
                || upper.starts_with("*TIE") || upper.starts_with("*COUPLING")
                || upper.starts_with("*RIGID BODY") || upper.starts_with("*CONNECTOR")
                || upper.starts_with("*SPRING") || upper.starts_with("*DASHPOT")
                || upper.starts_with("*MASS") || upper.starts_with("*ROTARY INERTIA")
                || upper.starts_with("*REBAR") || upper.starts_with("*GASKET")
                || upper.starts_with("*PRE-TENSION SECTION") || upper.starts_with("*FOUNDATION")
                || upper.starts_with("*FILM") || upper.starts_with("*RADIATE")
            {
                current_keyword = "SKIP".to_string();
                i += 1;
                continue;
            }

            // *END STEP
            if upper.starts_with("*END STEP") {
                current_keyword = String::new();
                current_step_name = String::new();
                i += 1;
                continue;
            }

            // 其他未知关键字，结束当前数据段
            if !upper.starts_with('*') {
                // 不是关键字，作为数据行处理
            } else {
                current_keyword = String::new();
                i += 1;
                continue;
            }
        }

        // 数据行处理
        match current_keyword.as_str() {
            "NODE" => {
                let parts: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();
                if parts.len() >= 4 {
                    if let (Ok(id), Ok(x), Ok(y), Ok(z)) = (
                        parts[0].parse::<usize>(),
                        parts[1].parse::<f64>(),
                        parts[2].parse::<f64>(),
                        parts[3].parse::<f64>(),
                    ) {
                        model.nodes.push(InpNode { id, x, y, z });
                    }
                } else if parts.len() >= 2 {
                    // 1D or 2D node
                    if let (Ok(id), Ok(x)) = (parts[0].parse::<usize>(), parts[1].parse::<f64>()) {
                        let y = parts.get(2).and_then(|p| p.parse::<f64>().ok()).unwrap_or(0.0);
                        let z = parts.get(3).and_then(|p| p.parse::<f64>().ok()).unwrap_or(0.0);
                        model.nodes.push(InpNode { id, x, y, z });
                    }
                }
            }
            "ELEMENT" => {
                let values = parse_int_values(trimmed);
                if values.len() >= 2 {
                    let elem_id = values[0];
                    let node_ids: Vec<usize> = values[1..].to_vec();
                    model.elements.push(InpElement {
                        id: elem_id,
                        element_type: current_element_type.clone(),
                        node_ids,
                    });
                }
            }
            "ELASTIC" => {
                if let Some(mat) = model.materials.iter_mut().last() {
                    let values = parse_data_values(trimmed);
                    if values.len() >= 2 {
                        mat.elastic_modulus = Some(values[0]);
                        mat.poisson_ratio = Some(values[1]);
                    }
                    mat.raw_properties.entry("ELASTIC".to_string())
                        .or_insert_with(Vec::new)
                        .extend(values);
                }
            }
            "DENSITY" => {
                if let Some(mat) = model.materials.iter_mut().last() {
                    let values = parse_data_values(trimmed);
                    if !values.is_empty() {
                        mat.density = Some(values[0]);
                    }
                    mat.raw_properties.entry("DENSITY".to_string())
                        .or_insert_with(Vec::new)
                        .extend(values);
                }
            }
            "EXPANSION" => {
                if let Some(mat) = model.materials.iter_mut().last() {
                    let values = parse_data_values(trimmed);
                    if !values.is_empty() {
                        mat.thermal_expansion = Some(values[0]);
                    }
                    mat.raw_properties.entry("EXPANSION".to_string())
                        .or_insert_with(Vec::new)
                        .extend(values);
                }
            }
            "CONDUCTIVITY" => {
                if let Some(mat) = model.materials.iter_mut().last() {
                    let values = parse_data_values(trimmed);
                    if !values.is_empty() {
                        mat.thermal_conductivity = Some(values[0]);
                    }
                    mat.raw_properties.entry("CONDUCTIVITY".to_string())
                        .or_insert_with(Vec::new)
                        .extend(values);
                }
            }
            "SPECIFIC_HEAT" => {
                if let Some(mat) = model.materials.iter_mut().last() {
                    let values = parse_data_values(trimmed);
                    if !values.is_empty() {
                        mat.specific_heat = Some(values[0]);
                    }
                    mat.raw_properties.entry("SPECIFIC_HEAT".to_string())
                        .or_insert_with(Vec::new)
                        .extend(values);
                }
            }
            "PLASTIC" => {
                if let Some(mat) = model.materials.iter_mut().last() {
                    let values = parse_data_values(trimmed);
                    if values.len() >= 2 {
                        mat.yield_stress = Some(values[0]);
                        mat.plastic_strain = Some(values[1]);
                    }
                    mat.raw_properties.entry("PLASTIC".to_string())
                        .or_insert_with(Vec::new)
                        .extend(values);
                }
            }
            "SHELL_SECTION" => {
                // Shell section data: material_id, thickness
                let values = parse_data_values(trimmed);
                if let Some(section) = model.sections.iter_mut().last() {
                    if section.section_type == "SHELL" && values.len() >= 1 {
                        section.thickness = Some(values[0]);
                    }
                }
            }
            "BOUNDARY" => {
                let parts: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();
                if parts.len() >= 2 {
                    let boundary = InpBoundary {
                        node_set: None,
                        node_id: None,
                        dof: parts.get(1).unwrap_or(&"").to_string(),
                        magnitude: parts.get(2).and_then(|p| p.parse::<f64>().ok()),
                        step_name: if current_step_name.is_empty() { None } else { Some(current_step_name.clone()) },
                    };
                    // 判断是节点集还是节点ID
                    if let Ok(nid) = parts[0].parse::<usize>() {
                        model.boundaries.push(InpBoundary {
                            node_id: Some(nid),
                            ..boundary
                        });
                    } else {
                        model.boundaries.push(InpBoundary {
                            node_set: Some(parts[0].to_string()),
                            ..boundary
                        });
                    }
                }
            }
            "CLOAD" => {
                let parts: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();
                if parts.len() >= 3 {
                    let magnitude = parts[2].parse::<f64>().unwrap_or(0.0);
                    let load = InpLoad {
                        load_type: "CLOAD".to_string(),
                        node_set: None,
                        node_id: None,
                        element_set: None,
                        dof: Some(parts[1].to_string()),
                        magnitude,
                        step_name: if current_step_name.is_empty() { None } else { Some(current_step_name.clone()) },
                        amplitude: None,
                    };
                    if let Ok(nid) = parts[0].parse::<usize>() {
                        model.loads.push(InpLoad {
                            node_id: Some(nid),
                            ..load
                        });
                    } else {
                        model.loads.push(InpLoad {
                            node_set: Some(parts[0].to_string()),
                            ..load
                        });
                    }
                }
            }
            "DLOAD" | "DSLOAD" | "PRESSURE" => {
                let parts: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();
                if parts.len() >= 2 {
                    let load_type = if current_keyword == "PRESSURE" {
                        "PRESSURE".to_string()
                    } else {
                        "DLOAD".to_string()
                    };
                    let magnitude = parts[1].parse::<f64>().unwrap_or(0.0);
                    let load = InpLoad {
                        load_type,
                        node_set: None,
                        node_id: None,
                        element_set: None,
                        dof: None,
                        magnitude,
                        step_name: if current_step_name.is_empty() { None } else { Some(current_step_name.clone()) },
                        amplitude: None,
                    };
                    if let Ok(nid) = parts[0].parse::<usize>() {
                        model.loads.push(InpLoad {
                            node_id: Some(nid),
                            ..load
                        });
                    } else {
                        model.loads.push(InpLoad {
                            element_set: Some(parts[0].to_string()),
                            ..load
                        });
                    }
                }
            }
            "AMPLITUDE" => {
                let values = parse_data_values(trimmed);
                if values.len() >= 2 {
                    if let Some(amp) = model.amplitudes.iter_mut().last() {
                        if amplitude_parsing_time {
                            amp.time_values.push(values[0]);
                            amp.amplitude_values.push(values[1]);
                        } else {
                            // Second column pair
                            if let Some(last_time) = amp.time_values.last_mut() {
                                *last_time = values[0];
                            }
                            if let Some(last_amp) = amp.amplitude_values.last_mut() {
                                *last_amp = values[1];
                            }
                        }
                    }
                }
            }
            "NSET" => {
                if let Some(ref set_name) = current_nset_name {
                    if nset_generate {
                        // Generate format: start, end, increment
                        let values = parse_int_values(trimmed);
                        if values.len() >= 3 {
                            let start = values[0];
                            let end = values[1];
                            let inc = if values[2] > 0 { values[2] } else { 1 };
                            let mut val = start;
                            while val <= end {
                                model.node_sets.entry(set_name.clone())
                                    .or_insert_with(Vec::new)
                                    .push(val);
                                val += inc;
                            }
                        }
                    } else {
                        let values = parse_int_values(trimmed);
                        model.node_sets.entry(set_name.clone())
                            .or_insert_with(Vec::new)
                            .extend(values);
                    }
                }
            }
            "ELSET" => {
                if let Some(ref set_name) = current_elset_name {
                    if elset_generate {
                        let values = parse_int_values(trimmed);
                        if values.len() >= 3 {
                            let start = values[0];
                            let end = values[1];
                            let inc = if values[2] > 0 { values[2] } else { 1 };
                            let mut val = start;
                            while val <= end {
                                model.element_sets.entry(set_name.clone())
                                    .or_insert_with(Vec::new)
                                    .push(val);
                                val += inc;
                            }
                        }
                    } else {
                        let values = parse_int_values(trimmed);
                        model.element_sets.entry(set_name.clone())
                            .or_insert_with(Vec::new)
                            .extend(values);
                    }
                }
            }
            "INITIAL_CONDITIONS" => {
                let parts: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();
                if parts.len() >= 2 {
                    let value = parts[1].parse::<f64>().unwrap_or(0.0);
                    model.initial_conditions.push(InpInitialCondition {
                        condition_type: "TEMPERATURE".to_string(),
                        node_set: Some(parts[0].to_string()),
                        value,
                    });
                }
            }
            _ => {}
        }

        i += 1;
    }

    // 后处理警告
    if model.nodes.is_empty() {
        model.warnings.push("INP 文件中未找到节点数据".to_string());
    }
    if model.elements.is_empty() {
        model.warnings.push("INP 文件中未找到单元数据".to_string());
    }
    if model.materials.is_empty() {
        model.warnings.push("INP 文件中未找到材料定义".to_string());
    }
    if model.nodes.len() > 500000 {
        model.warnings.push(format!(
            "网格节点数较多 ({}), 可能影响显示性能",
            model.nodes.len()
        ));
    }

    Ok(model)
}

/// 计算 Abaqus 模型的包围盒
fn compute_bounding_box(model: &AbaqusInpModel) -> Option<[f64; 6]> {
    if model.nodes.is_empty() {
        return None;
    }

    let mut xmin = f64::MAX;
    let mut ymin = f64::MAX;
    let mut zmin = f64::MAX;
    let mut xmax = f64::MIN;
    let mut ymax = f64::MIN;
    let mut zmax = f64::MIN;

    for node in &model.nodes {
        xmin = xmin.min(node.x);
        ymin = ymin.min(node.y);
        zmin = zmin.min(node.z);
        xmax = xmax.max(node.x);
        ymax = ymax.max(node.y);
        zmax = zmax.max(node.z);
    }

    Some([xmin, ymin, zmin, xmax, ymax, zmax])
}

/// Tauri 命令：导入 Abaqus INP 文件
#[tauri::command]
pub fn import_abaqus_inp(file_path: String) -> Result<AbaqusInpImportResult, String> {
    let path = PathBuf::from(&file_path);
    if !path.exists() {
        return Err(format!("文件不存在: {}", file_path));
    }

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if ext != "inp" {
        return Err(format!("不支持的文件格式: .{}，仅支持 .inp 文件", ext));
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("读取 INP 文件失败: {}", e))?;

    let model = parse_abaqus_inp(&content)?;
    let bounding_box = compute_bounding_box(&model);

    let num_nodes = model.nodes.len();
    let num_elements = model.elements.len();
    let num_materials = model.materials.len();
    let num_sections = model.sections.len();
    let num_boundaries = model.boundaries.len();
    let num_loads = model.loads.len();
    let num_steps = model.steps.len();
    let num_contact_pairs = model.contact_pairs.len();
    let num_node_sets = model.node_sets.len();
    let num_element_sets = model.element_sets.len();

    let warnings = model.warnings.clone();

    if num_nodes == 0 {
        return Ok(AbaqusInpImportResult {
            success: false,
            model,
            num_nodes,
            num_elements,
            num_materials,
            num_sections,
            num_boundaries,
            num_loads,
            num_steps,
            num_contact_pairs,
            num_node_sets,
            num_element_sets,
            bounding_box,
            warnings,
            error: Some("INP 文件中未找到有效的节点数据".to_string()),
        });
    }

    Ok(AbaqusInpImportResult {
        success: true,
        model,
        num_nodes,
        num_elements,
        num_materials,
        num_sections,
        num_boundaries,
        num_loads,
        num_steps,
        num_contact_pairs,
        num_node_sets,
        num_element_sets,
        bounding_box,
        warnings,
        error: None,
    })
}
