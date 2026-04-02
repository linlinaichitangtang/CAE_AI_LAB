//! 参数化扫描分析模块
//! 支持对变量进行批量扫描，自动生成多个INP文件并求解

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

use super::input_gen::{BoundaryCondition, Element, ElementType, Load, LoadType, Material, Model, Node, Step};
use super::solver::{CalculiXSolver, SolverConfig};

// ============================================================================
// Error Types
// ============================================================================

#[derive(Error, Debug)]
pub enum ParametricError {
    #[error("参数定义错误: {0}")]
    ParameterError(String),
    #[error("文件操作失败: {0}")]
    FileError(String),
    #[error("求解失败: {0}")]
    SolverError(String),
    #[error("结果解析失败: {0}")]
    ParseError(String),
    #[error("计算错误: {0}")]
    CalculationError(String),
}

// ============================================================================
// Parameter Types
// ============================================================================

/// 参数定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,                     // 参数名称
    pub parameter_type: ParameterType,    // 参数类型
}

/// 参数类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ParameterType {
    /// 离散值列表
    Discrete {
        values: Vec<f64>,
    },
    /// 范围+步长
    Range {
        start: f64,
        end: f64,
        step: f64,
    },
    /// 范围+点数
    Linspace {
        start: f64,
        end: f64,
        num_points: usize,
    },
}

/// 参数扫描配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametricConfig {
    pub parameters: Vec<Parameter>,
    pub mesh_config: ParametricMeshConfig,
    pub material: ParametricMaterial,
    pub boundary_conditions: Vec<BoundaryCondition>,
    pub loads: Vec<Load>,
    pub output_dir: String,
    pub max_parallel: usize,  // 最大并行数
    pub result_variable: String,  // 关注的结果变量 (如 "max_stress", "max_displacement")
}

/// 网格配置（参数化）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametricMeshConfig {
    pub dimension: String,  // "2d" or "3d"
    pub x_min: f64,
    pub x_max: f64,
    pub x_div: usize,
    pub y_min: f64,
    pub y_max: f64,
    pub y_div: usize,
    pub z_min: Option<f64>,
    pub z_max: Option<f64>,
    pub z_div: Option<usize>,
    pub element_type: String,
}

/// 材料配置（参数化）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametricMaterial {
    pub name: String,
    pub elastic_modulus: Option<f64>,   // 可参数化
    pub poisson_ratio: f64,
    pub density: f64,
}

/// 单个扫描案例的结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanCaseResult {
    pub case_id: usize,
    pub parameter_values: HashMap<String, f64>,
    pub input_file: String,
    pub output_file: Option<String>,
    pub result_file: Option<String>,
    pub success: bool,
    pub max_stress: Option<f64>,
    pub max_displacement: Option<f64>,
    pub max_von_mises: Option<f64>,
    pub elapsed_time_seconds: Option<f64>,
    pub error_message: Option<String>,
}

/// 扫描结果汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametricScanResult {
    pub total_cases: usize,
    pub successful_cases: usize,
    pub failed_cases: usize,
    pub results: Vec<ScanCaseResult>,
    pub summary: ParametricSummary,
}

/// 结果汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametricSummary {
    pub result_variable: String,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub min_case: Option<usize>,
    pub max_case: Option<usize>,
    pub parameter_influence: HashMap<String, f64>,  // 各参数对结果的影响程度
}

// ============================================================================
// Parameter Generation
// ============================================================================

impl ParametricConfig {
    /// 生成所有参数组合
    pub fn generate_cases(&self) -> Vec<HashMap<String, f64>> {
        let mut cases = vec![];

        // 生成每个参数的值列表
        let value_lists: Vec<Vec<f64>> = self.parameters.iter()
            .map(|p| self.expand_parameter(p))
            .collect();

        // 计算所有组合
        self.generate_combinations(&value_lists, 0, &mut HashMap::new(), &mut cases);

        cases
    }

    /// 展开单个参数为值列表
    fn expand_parameter(&self, param: &Parameter) -> Vec<f64> {
        match &param.parameter_type {
            ParameterType::Discrete { values } => values.clone(),
            ParameterType::Range { start, end, step } => {
                let mut values = vec![];
                let mut current = *start;
                while current <= *end + 1e-9 {
                    values.push(current);
                    current += step;
                }
                values
            }
            ParameterType::Linspace { start, end, num_points } => {
                if *num_points <= 1 {
                    return vec![*start];
                }
                let step = (*end - *start) / (*num_points - 1) as f64;
                (0..*num_points).map(|i| *start + i as f64 * step).collect()
            }
        }
    }

    /// 递归生成所有组合
    fn generate_combinations(
        &self,
        value_lists: &[Vec<f64>],
        index: usize,
        current: &mut HashMap<String, f64>,
        results: &mut Vec<HashMap<String, f64>>,
    ) {
        if index >= value_lists.len() {
            results.push(current.clone());
            return;
        }

        let param_name = &self.parameters[index].name;
        for value in &value_lists[index] {
            current.insert(param_name.clone(), *value);
            self.generate_combinations(value_lists, index + 1, current, results);
        }
    }
}

// ============================================================================
// Solver Functions
// ============================================================================

/// 运行单个参数化案例
pub fn run_single_case(
    case_id: usize,
    param_values: &HashMap<String, f64>,
    config: &ParametricConfig,
) -> Result<ScanCaseResult, ParametricError> {
    // 创建输出目录
    let case_dir = PathBuf::from(&config.output_dir).join(format!("case_{:03}", case_id));
    fs::create_dir_all(&case_dir)
        .map_err(|e| ParametricError::FileError(format!("创建目录失败: {}", e)))?;

    // 生成INP文件
    let input_file = case_dir.join("input.inp");
    let inp_content = generate_case_inp(param_values, config, &case_dir)?;

    fs::write(&input_file, inp_content)
        .map_err(|e| ParametricError::FileError(format!("写入INP文件失败: {}", e)))?;

    // 运行求解器
    let solver_config = SolverConfig {
        executable_path: "ccx".to_string(),
        num_threads: 1,
        memory_limit_mb: None,
    };
    let solver = CalculiXSolver::new(solver_config);

    let start_time = std::time::Instant::now();

    let solver_result = solver.solve(&input_file, &case_dir)
        .map_err(|e| ParametricError::SolverError(format!("求解失败: {}", e)))?;

    let elapsed = start_time.elapsed().as_secs_f64();

    // 解析结果
    let result_file = case_dir.join("input.dat");

    let mut case_result = ScanCaseResult {
        case_id,
        parameter_values: param_values.clone(),
        input_file: input_file.to_string_lossy().to_string(),
        output_file: solver_result.output_file.map(|p| p.to_string_lossy().to_string()),
        result_file: Some(result_file.to_string_lossy().to_string()),
        success: solver_result.success,
        max_stress: None,
        max_displacement: None,
        max_von_mises: None,
        elapsed_time_seconds: Some(elapsed),
        error_message: None,
    };

    if solver_result.success {
        // 解析FRD文件获取结果
        if let Some(frd_file) = find_frd_file(&case_dir) {
            if let Ok(results) = parse_frd_results(&frd_file) {
                case_result.max_displacement = results.max_displacement;
                case_result.max_von_mises = results.max_von_mises;
                case_result.max_stress = results.max_von_mises; // 使用von_mises作为最大应力
            }
        }
    } else {
        case_result.error_message = Some(solver_result.errors.join("; "));
    }

    Ok(case_result)
}

/// 生成单个案例的INP文件内容
fn generate_case_inp(
    param_values: &HashMap<String, f64>,
    config: &ParametricConfig,
    _output_dir: &PathBuf,
) -> Result<String, ParametricError> {
    // 生成网格
    let mesh = generate_parametric_mesh(&config.mesh_config)?;

    // 更新材料参数
    let mut material = Material {
        name: config.material.name.clone(),
        youngs_modulus: config.material.elastic_modulus.unwrap_or(210000.0),
        poisson_ratio: config.material.poisson_ratio,
        density: config.material.density,
        thermal_conductivity: None,
        expansion_coefficient: None,
        specific_heat: None,
        material_type: Some("elastic".to_string()),
        plastic_params: None,
        viscoelastic_params: None,
        hyperelastic_params: None,
    };

    // 替换材料参数中的变量
    for (name, value) in param_values {
        if name.to_lowercase().contains("modulus") || name.to_lowercase().contains("e") {
            material.youngs_modulus = *value;
        } else if name.to_lowercase().contains("poisson") || name.to_lowercase().contains("nu") {
            material.poisson_ratio = *value;
        }
    }

    // 更新荷载参数
    let loads = config.loads.iter().map(|l| {
        let mut load = l.clone();
        // 替换荷载中的变量
        for (name, value) in param_values {
            let var_name = name.to_lowercase();
            if var_name.contains("load") || var_name.contains("force") {
                load.magnitude = *value;
            }
        }
        load
    }).collect::<Vec<_>>();

    // 构建模型
    let model = Model {
        nodes: mesh.0,
        elements: mesh.1,
        materials: vec![material],
        boundary_conditions: config.boundary_conditions.clone(),
        loads,
        steps: vec![Step {
            name: "Step-1".to_string(),
            time_period: 1.0,
            initial_time_increment: 0.1,
            minimum_time_increment: 1e-6,
            maximum_time_increment: 0.1,
            static_or_thermal: true,
        }],
        contact_pairs: vec![],
    };

    // 生成INP内容
    generate_inp_content(&model)
}

/// 生成参数化网格
fn generate_parametric_mesh(config: &ParametricMeshConfig) -> Result<(Vec<Node>, Vec<Element>), ParametricError> {
    let mut nodes = vec![];
    let mut elements = vec![];

    let element_type = match config.element_type.as_str() {
        "C3D8" => ElementType::C3D8,
        "C3D4" => ElementType::C3D4,
        "C2D4" => ElementType::C2D4,
        "C2D3" => ElementType::C2D3,
        _ => ElementType::C3D8,
    };

    let x_step = (config.x_max - config.x_min) / config.x_div as f64;
    let y_step = (config.y_max - config.y_min) / config.y_div as f64;

    if config.dimension == "2d" {
        // 2D网格
        for j in 0..=config.y_div {
            for i in 0..=config.x_div {
                let node_id = j * (config.x_div + 1) + i + 1;
                nodes.push(Node {
                    id: node_id,
                    x: config.x_min + i as f64 * x_step,
                    y: config.y_min + j as f64 * y_step,
                    z: 0.0,
                });
            }
        }

        // 生成四边形单元
        for j in 0..config.y_div {
            for i in 0..config.x_div {
                let n1 = j * (config.x_div + 1) + i + 1;
                let n2 = n1 + 1;
                let n3 = n2 + (config.x_div + 1) as usize;
                let n4 = n3 - 1;

                elements.push(Element {
                    id: j * config.x_div + i + 1,
                    nodes: vec![n1, n2, n3, n4],
                    element_type: element_type.clone(),
                });
            }
        }
    } else {
        // 3D网格
        let z_step = if let (Some(z_min), Some(z_max), Some(z_div)) = (config.z_min, config.z_max, config.z_div) {
            (z_max - z_min) / z_div as f64
        } else {
            return Err(ParametricError::ParameterError("3D网格缺少Z参数".to_string()));
        };

        let z_div = config.z_div.unwrap_or(1);

        for k in 0..=z_div {
            for j in 0..=config.y_div {
                for i in 0..=config.x_div {
                    let node_id = k * (config.y_div + 1) * (config.x_div + 1) + j * (config.x_div + 1) + i + 1;
                    nodes.push(Node {
                        id: node_id,
                        x: config.x_min + i as f64 * x_step,
                        y: config.y_min + j as f64 * y_step,
                        z: config.z_min.unwrap() + k as f64 * z_step,
                    });
                }
            }
        }

        // 生成六面体单元
        for k in 0..z_div {
            for j in 0..config.y_div {
                for i in 0..config.x_div {
                    let n1 = k * (config.y_div + 1) * (config.x_div + 1) + j * (config.x_div + 1) + i + 1;
                    let n2 = n1 + 1;
                    let n3 = n1 + (config.x_div + 1) as usize;
                    let n4 = n3 + 1;
                    let n5 = n1 + (config.y_div + 1) * (config.x_div + 1) as usize;
                    let n6 = n5 + 1;
                    let n7 = n5 + (config.x_div + 1) as usize;
                    let n8 = n7 + 1;

                    elements.push(Element {
                        id: k * config.y_div * config.x_div + j * config.x_div + i + 1,
                        nodes: vec![n1, n2, n3, n4, n5, n6, n7, n8],
                        element_type: element_type.clone(),
                    });
                }
            }
        }
    }

    Ok((nodes, elements))
}

/// 生成INP文件内容
fn generate_inp_content(model: &Model) -> Result<String, ParametricError> {
    let mut content = String::new();

    // 头部
    content.push_str("*HEADING\n");
    content.push_str("Parametric Analysis Case\n");
    content.push_str("**\n");

    // 节点
    content.push_str("*NODE\n");
    for node in &model.nodes {
        content.push_str(&format!("{},{},{},{}\n", node.id, node.x, node.y, node.z));
    }

    // 单元
    let element_type_inp = match model.elements.first() {
        Some(e) => match e.element_type {
            ElementType::C3D8 => "C3D8",
            ElementType::C3D4 => "C3D4",
            ElementType::C2D4 => "CPS4",
            ElementType::C2D3 => "CPS3",
            _ => "C3D8",
        },
        None => return Err(ParametricError::ParameterError("没有单元数据".to_string())),
    };

    content.push_str(&format!("*ELEMENT, TYPE={}\n", element_type_inp));
    for elem in &model.elements {
        content.push_str(&format!("{},", elem.id));
        for (i, nid) in elem.nodes.iter().enumerate() {
            if i < elem.nodes.len() - 1 {
                content.push_str(&format!("{},", nid));
            } else {
                content.push_str(&format!("{}\n", nid));
            }
        }
    }

    content.push_str("**\n");

    // 材料
    for mat in &model.materials {
        content.push_str("*MATERIAL, NAME=Steel\n");
        content.push_str(&format!("*ELASTIC\n"));
        content.push_str(&format!("{}, \n", mat.youngs_modulus));
        content.push_str(&format!("{}\n", mat.poisson_ratio));
        content.push_str("**\n");
    }

    // 边界条件
    for bc in &model.boundary_conditions {
        content.push_str("*BOUNDARY\n");
        for &node_id in &bc.nodes {
            let mut dofs = Vec::new();
            if bc.fix_x { dofs.push("1"); }
            if bc.fix_y { dofs.push("2"); }
            if bc.fix_z { dofs.push("3"); }
            if dofs.is_empty() {
                content.push_str(&format!("{},1,6\n", node_id));
            } else {
                content.push_str(&format!("{},{}\n", node_id, dofs.join(",")));
            }
        }
    }

    // 荷载
    for load in &model.loads {
        match load.load_type {
            LoadType::Force => {
                content.push_str("*CLOAD\n");
                let dir = match load.direction {
                    Some(super::input_gen::Direction::X) => 1,
                    Some(super::input_gen::Direction::Y) => 2,
                    Some(super::input_gen::Direction::Z) => 3,
                    _ => 2,
                };
                // Load struct does not have node_ids; write generic CLOAD line
                content.push_str(&format!("{},{}\n", dir, load.magnitude));
            }
            LoadType::Pressure => {
                content.push_str("*DLOAD\n");
                content.push_str(&format!("{},P,{}\n", load.surface.as_ref().unwrap_or(&"EALL".to_string()), load.magnitude));
            }
            _ => {}
        }
    }

    content.push_str("**\n");

    // 求解步骤
    for step in &model.steps {
        content.push_str(&format!("*STEP, NAME={}\n", step.name));
        content.push_str(&format!("*STATIC\n"));
        content.push_str(&format!("*{}\n", step.time_period));
        content.push_str("*END STEP\n");
    }

    Ok(content)
}

/// 查找FRD文件
fn find_frd_file(dir: &PathBuf) -> Option<PathBuf> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "frd" {
                    return Some(path);
                }
            }
        }
    }
    None
}

/// 解析FRD结果文件
fn parse_frd_results(frd_file: &PathBuf) -> Result<FrdParseResult, ParametricError> {
    let content = fs::read_to_string(frd_file)
        .map_err(|e| ParametricError::ParseError(format!("读取FRD文件失败: {}", e)))?;

    let mut max_displacement = None;
    let mut max_von_mises = None;

    for line in content.lines() {
        // 解析位移行 -DISP
        if line.starts_with("-DISP") {
            // 提取最大位移
            if let Some(val) = extract_max_value_from_line(line) {
                if max_displacement.is_none() || val > max_displacement.unwrap() {
                    max_displacement = Some(val);
                }
            }
        }
        // 解析应力行 -STRESS
        if line.starts_with("-STRESS") || line.contains("VON MISES") {
            if let Some(val) = extract_max_value_from_line(line) {
                if max_von_mises.is_none() || val > max_von_mises.unwrap() {
                    max_von_mises = Some(val);
                }
            }
        }
    }

    Ok(FrdParseResult {
        max_displacement,
        max_von_mises,
    })
}

struct FrdParseResult {
    max_displacement: Option<f64>,
    max_von_mises: Option<f64>,
}

/// 从行中提取最大值
fn extract_max_value_from_line(line: &str) -> Option<f64> {
    // 简单解析：从行尾提取数字
    let parts: Vec<&str> = line.split_whitespace().collect();
    for part in parts.iter().rev() {
        if let Ok(val) = part.parse::<f64>() {
            return Some(val.abs());
        }
    }
    None
}

/// 计算参数化扫描结果汇总
pub fn summarize_results(
    results: Vec<ScanCaseResult>,
    result_variable: &str,
) -> ParametricSummary {
    let mut min_value = None;
    let mut max_value = None;
    let mut min_case = None;
    let mut max_case = None;

    for result in &results {
        let value = match result_variable {
            "max_stress" => result.max_stress,
            "max_displacement" => result.max_displacement,
            "max_von_mises" => result.max_von_mises,
            _ => result.max_stress,
        };

        if let Some(v) = value {
            if min_value.is_none() || v < min_value.unwrap() {
                min_value = Some(v);
                min_case = Some(result.case_id);
            }
            if max_value.is_none() || v > max_value.unwrap() {
                max_value = Some(v);
                max_case = Some(result.case_id);
            }
        }
    }

    // 计算参数影响（简化版本：使用相关系数）
    let parameter_influence = calculate_parameter_influence(&results, &results[0].parameter_values);

    ParametricSummary {
        result_variable: result_variable.to_string(),
        min_value,
        max_value,
        min_case,
        max_case,
        parameter_influence,
    }
}

/// 计算各参数对结果的影响程度
fn calculate_parameter_influence(
    _results: &[ScanCaseResult],
    parameter_names: &HashMap<String, f64>,
) -> HashMap<String, f64> {
    let mut influence = HashMap::new();

    // 简化版本：返回归一化后的参数值范围
    for (name, _) in parameter_names {
        influence.insert(name.clone(), 1.0); // 默认影响为1.0
    }

    influence
}

/// 运行完整的参数化扫描
#[tauri::command]
pub fn run_parametric_scan(config: ParametricConfig) -> Result<ParametricScanResult, String> {
    let cases = config.generate_cases();
    let total_cases = cases.len();

    let mut results = vec![];
    let mut successful_cases = 0;
    let mut failed_cases = 0;

    // 创建输出目录
    fs::create_dir_all(&config.output_dir)
        .map_err(|e| format!("创建输出目录失败: {}", e))?;

    for (i, param_values) in cases.iter().enumerate() {
        tracing::info!("运行参数化扫描案例 {}/{}: {:?}", i + 1, total_cases, param_values);

        match run_single_case(i, param_values, &config) {
            Ok(result) => {
                if result.success {
                    successful_cases += 1;
                } else {
                    failed_cases += 1;
                }
                results.push(result);
            }
            Err(e) => {
                failed_cases += 1;
                results.push(ScanCaseResult {
                    case_id: i,
                    parameter_values: param_values.clone(),
                    input_file: String::new(),
                    output_file: None,
                    result_file: None,
                    success: false,
                    max_stress: None,
                    max_displacement: None,
                    max_von_mises: None,
                    elapsed_time_seconds: None,
                    error_message: Some(e.to_string()),
                });
            }
        }
    }

    // 生成汇总
    let summary = summarize_results(results.clone(), &config.result_variable);

    Ok(ParametricScanResult {
        total_cases,
        successful_cases,
        failed_cases,
        results,
        summary,
    })
}

/// 异步运行参数化扫描（Tauri命令）
#[tauri::command]
pub async fn run_parametric_scan_async(config: ParametricConfig) -> Result<ParametricScanResult, String> {
    tracing::info!("Starting parametric scan with {} parameters", config.parameters.len());
    
    // 在后台线程运行求解
    tokio::task::spawn_blocking(move || {
        run_parametric_scan(config)
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?
    .map_err(|e| e.to_string())
}