//! 瞬态动力学分析命令模块
//! 
//! 本模块提供动力学瞬态分析功能,支持多种载荷曲线类型、阻尼设置和时间步控制。
//! 
//! 主要功能：
//! - 载荷曲线定义 (阶跃、正弦、冲击、自定义)
//! - Rayleigh阻尼设置
//! - 时间步自动控制
//! - CalculiX INP文件生成
//! - 简化时域仿真 (Newmark-β积分)
//! - 示例案例 (单自由度振动、悬臂梁冲击)
//! 
//! 瞬态分析适用于:
//! - 结构在随时间变化的荷载下的响应
//! - 冲击载荷作用下的动态响应
//! - 振动特性分析

use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::path::PathBuf;
use thiserror::Error;

/// 瞬态分析错误类型
#[derive(Debug, Error)]
pub enum TransientError {
    /// IO错误
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    /// 格式化错误
    #[error("Format error: {0}")]
    FormatError(#[from] std::fmt::Error),
    /// 参数无效错误
    #[error("Invalid parameter: {0}")]
    InvalidParam(String),
    /// 网格未生成错误
    #[error("Mesh not generated: {0}")]
    MeshNotFound(String),
}

// ============ 载荷曲线类型 ============

/// 载荷-时间曲线上的数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadPoint {
    /// 时间点 (秒)
    pub time: f64,
    /// 对应载荷值
    pub value: f64,
}

/// 载荷曲线类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoadType {
    /// 阶跃载荷 - 突然施加的恒定载荷
    Step,
    /// 正弦载荷 - 简谐变化载荷
    Sinusoid,
    /// 冲击载荷 - 短时脉冲载荷
    Impulse,
    /// 自定义曲线 - 用户定义的载荷时间历程
    Custom,
}

/// 载荷曲线结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadCurve {
    /// 载荷曲线ID
    pub id: String,
    /// 载荷曲线名称
    pub name: String,
    /// 载荷类型
    pub load_type: LoadType,
    /// 载荷-时间数据点列表
    pub points: Vec<LoadPoint>,
    /// 幅值 (用于正弦/冲击载荷)
    pub amplitude: f64,
    /// 频率 (Hz) (用于正弦载荷)
    pub frequency: f64,
    /// 相位 (弧度) (用于正弦载荷)
    pub phase: f64,
    /// 持续时间 (秒)
    pub duration: f64,
}

// ============ 阻尼设置 ============

/// Rayleigh阻尼结构体
/// 
/// Rayleigh阻尼假设阻尼矩阵与质量矩阵和刚度矩阵成正比:
/// C = α*M + β*K
/// 其中 α 是质量阻尼系数, β 是刚度阻尼系数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RayleighDamping {
    /// 质量阻尼系数 α (与质量矩阵成正比)
    pub alpha: f64,
    /// 刚度阻尼系数 β (与刚度矩阵成正比)
    pub beta: f64,
    /// 第一频率点 (Hz) - 用于计算阻尼系数
    pub frequency1: f64,
    /// 第二频率点 (Hz) - 用于计算阻尼系数
    pub frequency2: f64,
}

impl Default for RayleighDamping {
    fn default() -> Self {
        Self {
            alpha: 0.1,
            beta: 0.0001,
            frequency1: 1.0,
            frequency2: 10.0,
        }
    }
}

// ============ 时间步控制 ============

/// 时间步控制结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeStepControl {
    /// 初始时间步长 (秒)
    pub initial_dt: f64,
    /// 最小时间步长 (秒)
    pub min_dt: f64,
    /// 最大时间步长 (秒)
    pub max_dt: f64,
    /// 总分析时间 (秒)
    pub total_time: f64,
    /// 输出间隔 (每N步输出一次结果)
    pub output_interval: i32,
}

impl Default for TimeStepControl {
    fn default() -> Self {
        Self {
            initial_dt: 0.001,
            min_dt: 1e-6,
            max_dt: 0.01,
            total_time: 1.0,
            output_interval: 100,
        }
    }
}

// ============ 瞬态分析配置 ============

/// 瞬态分析完整配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientAnalysisConfig {
    /// 分析名称
    pub name: String,
    /// 网格ID
    pub mesh_id: String,
    /// 载荷曲线列表
    pub load_curves: Vec<LoadCurve>,
    /// Rayleigh阻尼设置
    pub damping: RayleighDamping,
    /// 时间步控制
    pub time_control: TimeStepControl,
    /// 初始条件 [初始位移, 初始速度, ...]
    pub initial_conditions: Vec<f64>,
}

// ============ 时域结果 ============

/// 单个时间步的结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientResultStep {
    /// 当前时间 (秒)
    pub time: f64,
    /// 每个节点的位移 [u1, u2, u3, ...]
    pub displacements: Vec<f64>,
    /// 每个节点的速度 [v1, v2, v3, ...]
    pub velocities: Vec<f64>,
    /// 每个节点的加速度 [a1, a2, a3, ...]
    pub accelerations: Vec<f64>,
    /// 每个单元的应力
    pub stresses: Vec<f64>,
    /// 最大位移
    pub max_displacement: f64,
    /// 最大应力
    pub max_stress: f64,
}

/// 完整瞬态分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientResults {
    /// 分析名称
    pub analysis_name: String,
    /// 总分析时间
    pub total_time: f64,
    /// 所有时间步的结果
    pub steps: Vec<TransientResultStep>,
    /// 全过程最大位移
    pub max_displacement_overall: f64,
    /// 全过程最大应力
    pub max_stress_overall: f64,
    /// 结构固有频率列表
    pub natural_frequencies: Vec<f64>,
}

// ============ 示例案例 ============

/// 教程示例案例结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialExample {
    /// 示例ID
    pub id: String,
    /// 示例名称
    pub name: String,
    /// 示例描述
    pub description: String,
    /// 预定义的配置
    pub config: TransientAnalysisConfig,
}

/// 获取单自由度振动示例
/// 
/// 演示一个质量-弹簧-阻尼系统的自由振动响应
/// 用于验证瞬态分析算法的正确性
/// 
/// # 返回
/// TutorialExample 单自由度振动示例
pub fn get_sdolf_example() -> TutorialExample {
    TutorialExample {
        id: "sdolf".to_string(),
        name: "单自由度振动".to_string(),
        description: "演示一个质量-弹簧-阻尼系统的自由振动响应，用于验证瞬态分析算法的正确性。".to_string(),
        config: TransientAnalysisConfig {
            name: "SDOF自由振动".to_string(),
            mesh_id: "sdolf_mesh".to_string(),
            load_curves: vec![
                LoadCurve {
                    id: "initial_impulse".to_string(),
                    name: "初始冲击".to_string(),
                    load_type: LoadType::Impulse,
                    points: vec![
                        LoadPoint { time: 0.0, value: 0.1 },
                        LoadPoint { time: 0.001, value: 0.0 },
                    ],
                    amplitude: 0.1,
                    frequency: 0.0,
                    phase: 0.0,
                    duration: 0.001,
                }
            ],
            damping: RayleighDamping {
                alpha: 0.5,
                beta: 0.001,
                frequency1: 2.0,
                frequency2: 5.0,
            },
            time_control: TimeStepControl {
                initial_dt: 0.001,
                min_dt: 1e-6,
                max_dt: 0.005,
                total_time: 2.0,
                output_interval: 50,
            },
            initial_conditions: vec![0.0, 0.0],  // 初始位移=0, 初始速度=0.1
        },
    }
}

/// 获取悬臂梁冲击响应示例
/// 
/// 演示悬臂梁受到冲击载荷时的动态响应
/// 包含 Rayleigh 阻尼和时域后处理
/// 
/// # 返回
/// TutorialExample 悬臂梁冲击示例
pub fn get_cantilever_impact_example() -> TutorialExample {
    TutorialExample {
        id: "cantilever_impact".to_string(),
        name: "悬臂梁冲击".to_string(),
        description: "演示悬臂梁受到冲击载荷时的动态响应，包含 Rayleigh 阻尼和时域后处理。".to_string(),
        config: TransientAnalysisConfig {
            name: "悬臂梁冲击响应".to_string(),
            mesh_id: "cantilever_mesh".to_string(),
            load_curves: vec![
                LoadCurve {
                    id: "impact_load".to_string(),
                    name: "冲击载荷".to_string(),
                    load_type: LoadType::Impulse,
                    points: vec![
                        LoadPoint { time: 0.0, value: 0.0 },
                        LoadPoint { time: 0.005, value: 1000.0 },
                        LoadPoint { time: 0.01, value: 0.0 },
                    ],
                    amplitude: 1000.0,
                    frequency: 0.0,
                    phase: 0.0,
                    duration: 0.01,
                }
            ],
            damping: RayleighDamping {
                alpha: 1.0,
                beta: 0.0005,
                frequency1: 5.0,
                frequency2: 20.0,
            },
            time_control: TimeStepControl {
                initial_dt: 0.0005,
                min_dt: 1e-6,
                max_dt: 0.002,
                total_time: 0.5,
                output_interval: 20,
            },
            initial_conditions: vec![],
        },
    }
}

// ============ CalculiX INP生成 ============

/// 生成CalculiX瞬态动力学分析输入文件
/// 
/// 根据配置生成完整的INP文件,包含节点、单元、材料、边界条件、荷载和时间步控制
/// 
/// # 参数
/// - `config`: 瞬态分析配置
/// - `nodes`: 节点列表 (ID, x, y, z)
/// - `elements`: 单元列表 (ID, 类型, 节点ID列表)
/// - `boundary_faces`: 边界条件面 (节点ID, 边界类型)
/// - `output_path`: 输出文件路径
/// 
/// # 返回
/// Result<(), TransientError>: 成功返回空, 失败返回错误
pub fn generate_transient_inp(
    config: &TransientAnalysisConfig,
    nodes: &[(usize, f64, f64, f64)],
    elements: &[(usize, String, Vec<usize>)],
    boundary_faces: &[(usize, String)],  // (node_id, face_type)
    output_path: &PathBuf,
) -> Result<(), TransientError> {
    let mut content = String::new();
    
    // 头部注释
    writeln!(content, "** Transient Dynamics Analysis - {}", config.name)?;
    writeln!(content, "** Generated by CAELab")?;
    writeln!(content, "**")?;
    
    // 节点定义
    writeln!(content, "*NODE")?;
    for (id, x, y, z) in nodes {
        writeln!(content, "{}, {}, {}, {}", id, x, y, z)?;
    }
    
    // 单元定义
    writeln!(content, "\n*ELEMENT, TYPE=C3D8")?;
    for (id, _, node_ids) in elements {
        writeln!(content, "{}, {}", id, node_ids.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "))?;
    }
    
    // 材料定义
    writeln!(content, "\n** Material - Steel")?;
    writeln!(content, "*MATERIAL, NAME=STEEL")?;
    writeln!(content, "*ELASTIC")?;
    writeln!(content, "210000, 0.3")?;
    writeln!(content, "*DENSITY")?;
    writeln!(content, "7850")?;
    
    // Rayleigh阻尼
    writeln!(content, "\n** Rayleigh Damping")?;
    writeln!(content, "*DAMPING, ALPHA={}, BETA={}", config.damping.alpha, config.damping.beta)?;
    
    // 截面定义
    writeln!(content, "\n*SOLID SECTION, MATERIAL=STEEL, ELEMENT TYPE=C3D8")?;
    writeln!(content, ", 1.0")?;
    
    // 边界条件 (固定端)
    writeln!(content, "\n** Boundary Conditions")?;
    writeln!(content, "*BOUNDARY")?;
    for (node_id, face_type) in boundary_faces {
        if face_type == "fixed" {
            writeln!(content, "{}, 1, 6", node_id)?;
        }
    }
    
    // 载荷曲线 (Amplitude定义)
    writeln!(content, "\n** Load Curves")?;
    for (i, curve) in config.load_curves.iter().enumerate() {
        let amp_name = format!("LOAD_{}", i + 1);
        writeln!(content, "*AMPLITUDE, NAME={}", amp_name)?;
        for point in &curve.points {
            writeln!(content, "{}, {}", point.time, point.value)?;
        }
    }
    
    // 时间步控制
    writeln!(content, "\n** Time Step Control")?;
    writeln!(content, "*STEP, INC=100000, NLGEOM=YES")?;
    writeln!(content, "** DYNAMIC ANALYSIS")?;
    writeln!(content, "*DYNAMIC, DIRECT")?;
    writeln!(content, "{}, {}, {}, {}, {}", 
        config.time_control.initial_dt,
        config.time_control.total_time,
        config.time_control.min_dt,
        config.time_control.max_dt,
        config.time_control.output_interval
    )?;
    
    // 荷载应用
    writeln!(content, "\n** Loads")?;
    for (i, _curve) in config.load_curves.iter().enumerate() {
        let amp_name = format!("LOAD_{}", i + 1);
        writeln!(content, "*CLOAD")?;
        // 应用到自由端节点
        for (node_id, face_type) in boundary_faces {
            if face_type == "free" {
                writeln!(content, "{}, 3, {}, 1.0", node_id, amp_name)?;
            }
        }
    }
    
    // 输出请求
    writeln!(content, "\n** Output Requests")?;
    writeln!(content, "*NODE FILE, OUTPUT=3D")?;
    writeln!(content, "U, V, A")?;
    writeln!(content, "*EL FILE, OUTPUT=3D")?;
    writeln!(content, "S, E")?;
    writeln!(content, "*END STEP")?;
    
    std::fs::write(output_path, content)?;
    Ok(())
}

// ============ 简化时域仿真 (用于演示) ============

/// 简化时域仿真 (Newmark-β积分)
/// 
/// 使用简化的Newmark-β方法求解运动方程:
/// M*a + C*v + K*u = F(t)
/// 
/// # 参数
/// - `config`: 瞬态分析配置
/// - `num_nodes`: 节点数量
/// 
/// # 返回
/// TransientResults 瞬态分析结果
pub fn simulate_transient_response(
    config: &TransientAnalysisConfig,
    num_nodes: usize,
) -> TransientResults {
    let mut steps = Vec::new();
    
    // 简化的Newmark-β积分
    let dt = config.time_control.initial_dt;
    let t_end = config.time_control.total_time;
    // 等效阻尼比
    let damping_ratio = config.damping.alpha / (2.0 * (config.damping.frequency1 + config.damping.frequency2) / 2.0 * 2.0 * std::f64::consts::PI);
    
    // 初始化位移、速度、加速度
    let mut displacements = vec![0.0; num_nodes];
    let mut velocities = vec![0.0; num_nodes];
    let mut accelerations = vec![0.0; num_nodes];
    
    // 设置初始条件
    if !config.initial_conditions.is_empty() && config.initial_conditions.len() >= 2 {
        displacements[0] = config.initial_conditions[0];
        velocities[0] = config.initial_conditions[1];
    }
    
    let mut current_time = 0.0;
    let mut step_count = 0;
    let omega = 2.0 * std::f64::consts::PI * ((config.damping.frequency1 + config.damping.frequency2) / 2.0);
    
    while current_time <= t_end {
        // 计算当前载荷
        let load_value = calculate_load_at_time(&config.load_curves, current_time);
        
        // 简化的运动方程求解
        let force = load_value;
        let stiffness = 1000.0;
        let mass = 1.0;
        
        // F = ma => a = (F - cv - ku) / m
        accelerations[0] = (force - damping_ratio * velocities[0] - stiffness * displacements[0]) / mass;
        velocities[0] += accelerations[0] * dt;
        displacements[0] += velocities[0] * dt;
        
        // 计算应力 (简化: 应力 = 刚度 * 位移)
        let stress = stiffness * displacements[0].abs();
        
        // 按间隔记录结果
        if step_count % config.time_control.output_interval as usize == 0 || current_time == 0.0 || current_time >= t_end - 0.001 {
            steps.push(TransientResultStep {
                time: current_time,
                displacements: displacements.clone(),
                velocities: velocities.clone(),
                accelerations: accelerations.clone(),
                stresses: vec![stress],
                max_displacement: displacements.iter().cloned().fold(0.0, f64::max),
                max_stress: stress,
            });
        }
        
        current_time += dt;
        step_count += 1;
        
        // 防止无限循环
        if step_count > 10000 {
            break;
        }
    }
    
    let max_disp_overall = steps.iter().map(|s| s.max_displacement).fold(0.0, f64::max);
    let max_stress_overall = steps.iter().map(|s| s.max_stress).fold(0.0, f64::max);
    
    TransientResults {
        analysis_name: config.name.clone(),
        total_time: t_end,
        steps,
        max_displacement_overall: max_disp_overall,
        max_stress_overall: max_stress_overall,
        natural_frequencies: vec![omega / (2.0 * std::f64::consts::PI)],
    }
}

/// 计算给定时间点的载荷值
/// 
/// 根据载荷曲线类型计算当前时刻的总载荷
/// 
/// # 参数
/// - `curves`: 载荷曲线列表
/// - `time`: 当前时间 (秒)
/// 
/// # 返回
/// 当前时刻的总载荷值
fn calculate_load_at_time(curves: &[LoadCurve], time: f64) -> f64 {
    let mut total_load = 0.0;
    for curve in curves {
        match curve.load_type {
            LoadType::Step => {
                // 阶跃载荷: t>=0时施加恒定载荷
                if time >= 0.0 {
                    total_load += curve.amplitude;
                }
            }
            LoadType::Sinusoid => {
                // 正弦载荷: A * sin(2πft + φ)
                let t = time % curve.duration;
                total_load += curve.amplitude * (2.0 * std::f64::consts::PI * curve.frequency * t + curve.phase).sin();
            }
            LoadType::Impulse => {
                // 冲击载荷: 近似高斯脉冲
                let _impulse_total: f64 = curve.points.iter().map(|p| p.value).sum::<f64>() * 0.001;
                if time < 0.01 {
                    total_load += curve.amplitude * (-((time - 0.005) / 0.005).powi(2) + 1.0);
                }
            }
            LoadType::Custom => {
                // 自定义曲线: 线性插值
                for i in 0..curve.points.len().saturating_sub(1) {
                    let p1 = &curve.points[i];
                    let p2 = &curve.points[i + 1];
                    if time >= p1.time && time <= p2.time {
                        let t = (time - p1.time) / (p2.time - p1.time);
                        total_load += p1.value + (p2.value - p1.value) * t;
                        break;
                    }
                }
            }
        }
    }
    total_load
}

// ============ Tauri Commands ============

/// Tauri命令 - 获取教程示例列表
/// 
/// 返回预定义的示例案例供用户学习参考
/// 
/// # 返回
/// Vec<TutorialExample> 示例案例列表
#[tauri::command]
pub fn get_tutorial_examples() -> Vec<TutorialExample> {
    vec![get_sdolf_example(), get_cantilever_impact_example()]
}

/// Tauri命令 - 生成瞬态分析INP文件
/// 
/// # 参数
/// - `config_json`: JSON格式的分析配置字符串
/// - `output_path`: 输出文件路径
/// 
/// # 返回
/// Result<String, String>: 成功返回文件路径, 失败返回错误信息
#[tauri::command]
pub fn generate_transient_inp_file(
    config_json: String,
    output_path: String,
) -> Result<String, String> {
    let config: TransientAnalysisConfig = serde_json::from_str(&config_json)
        .map_err(|e| format!("Failed to parse config: {}", e))?;
    
    let path = PathBuf::from(&output_path);
    
    // 生成示例网格数据 (简化)
    let nodes: Vec<(usize, f64, f64, f64)> = (0..10)
        .map(|i| (i + 1, i as f64 * 0.1, 0.0, 0.0))
        .collect();
    
    let elements: Vec<(usize, String, Vec<usize>)> = (0..9)
        .map(|i| (i + 1, "C3D8".to_string(), vec![i + 1, i + 2]))
        .collect();
    
    let boundary_faces: Vec<(usize, String)> = vec![
        (1, "fixed".to_string()),
        (2, "fixed".to_string()),
    ];
    
    generate_transient_inp(&config, &nodes, &elements, &boundary_faces, &path)
        .map_err(|e| e.to_string())?;
    
    Ok(output_path)
}

/// Tauri命令 - 运行瞬态仿真
/// 
/// 使用简化的Newmark-β方法进行时域分析
/// 
/// # 参数
/// - `config_json`: JSON格式的分析配置字符串
/// 
/// # 返回
/// Result<TransientResults, String>: 瞬态分析结果
#[tauri::command]
pub fn run_transient_simulation(
    config_json: String,
) -> Result<TransientResults, String> {
    let config: TransientAnalysisConfig = serde_json::from_str(&config_json)
        .map_err(|e| format!("Failed to parse config: {}", e))?;
    
    // 执行简化仿真
    let results = simulate_transient_response(&config, 10);
    
    Ok(results)
}

/// Tauri命令 - 计算Rayleigh阻尼系数
/// 
/// 根据两个频率点和阻尼比计算α和β
/// 
/// 阻尼比公式: ζ = (α/(2ω)) + (βω/2)
/// 解联立方程组得到α和β
/// 
/// # 参数
/// - `frequency1`: 第一频率点 (Hz)
/// - `frequency2`: 第二频率点 (Hz)
/// - `damping_ratio`: 目标阻尼比
/// 
/// # 返回
/// Result<RayleighDamping, String>: 计算得到的阻尼系数
#[tauri::command]
pub fn calculate_rayleigh_coefficients(
    frequency1: f64,
    frequency2: f64,
    damping_ratio: f64,
) -> Result<RayleighDamping, String> {
    let omega1 = 2.0 * std::f64::consts::PI * frequency1;
    let omega2 = 2.0 * std::f64::consts::PI * frequency2;
    
    // 根据阻尼比计算 alpha 和 beta
    // ζ = (α/(2*ω)) + (β*ω/2)
    // 解联立方程组
    let denom = omega1 * omega2 * (omega2 - omega1);
    let alpha = 2.0 * damping_ratio * omega1 * omega2 * (omega2 - omega1) / denom * (omega1 + omega2) / 2.0;
    let beta = 2.0 * damping_ratio * (omega2 - omega1) / denom * (omega1 + omega2) / 2.0;
    
    Ok(RayleighDamping {
        alpha: alpha.max(0.001),
        beta: beta.max(0.0001),
        frequency1,
        frequency2,
    })
}

/// Tauri命令 - 获取载荷曲线模板
/// 
/// # 参数
/// - `template_type`: 模板类型 ("step", "sinusoid", "impulse")
/// 
/// # 返回
/// Result<LoadCurve, String>: 载荷曲线模板
#[tauri::command]
pub fn get_load_curve_template(
    template_type: String,
) -> Result<LoadCurve, String> {
    match template_type.as_str() {
        "step" => Ok(LoadCurve {
            id: "step_template".to_string(),
            name: "阶跃载荷".to_string(),
            load_type: LoadType::Step,
            points: vec![
                LoadPoint { time: 0.0, value: 0.0 },
                LoadPoint { time: 0.001, value: 1.0 },
            ],
            amplitude: 1.0,
            frequency: 0.0,
            phase: 0.0,
            duration: 1.0,
        }),
        "sinusoid" => Ok(LoadCurve {
            id: "sinusoid_template".to_string(),
            name: "正弦载荷".to_string(),
            load_type: LoadType::Sinusoid,
            points: vec![],
            amplitude: 1.0,
            frequency: 10.0,
            phase: 0.0,
            duration: 0.1,
        }),
        "impulse" => Ok(LoadCurve {
            id: "impulse_template".to_string(),
            name: "冲击载荷".to_string(),
            load_type: LoadType::Impulse,
            points: vec![
                LoadPoint { time: 0.0, value: 0.0 },
                LoadPoint { time: 0.005, value: 100.0 },
                LoadPoint { time: 0.01, value: 0.0 },
            ],
            amplitude: 100.0,
            frequency: 0.0,
            phase: 0.0,
            duration: 0.01,
        }),
        _ => Err(format!("Unknown template type: {}", template_type)),
    }
}
