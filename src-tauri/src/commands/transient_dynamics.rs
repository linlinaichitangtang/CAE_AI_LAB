//! Transient Dynamics Analysis Command Module
//! 支持动力学瞬态分析，包括载荷曲线、阻尼设置、时间步控制

use serde::{Deserialize, Serialize};
use std::fmt::Write as FmtWrite;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Error)]
pub enum TransientError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid parameter: {0}")]
    InvalidParam(String),
    #[error("Mesh not generated: {0}")]
    MeshNotFound(String),
}

// ============ 载荷曲线类型 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadPoint {
    pub time: f64,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoadType {
    Step,      // 阶跃载荷
    Sinusoid,  // 正弦载荷
    Impulse,   // 冲击载荷
    Custom,    // 自定义曲线
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadCurve {
    pub id: String,
    pub name: String,
    pub load_type: LoadType,
    pub points: Vec<LoadPoint>,
    pub amplitude: f64,  // 幅值
    pub frequency: f64,  // 频率 (Hz)
    pub phase: f64,      // 相位 (rad)
    pub duration: f64,   // 持续时间
}

// ============ 阻尼设置 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RayleighDamping {
    pub alpha: f64,  // 质量阻尼系数 α
    pub beta: f64,   // 刚度阻尼系数 β
    pub frequency1: f64,  // 第一频率点 (Hz)
    pub frequency2: f64,  // 第二频率点 (Hz)
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeStepControl {
    pub initial_dt: f64,      // 初始时间步长
    pub min_dt: f64,          // 最小时间步长
    pub max_dt: f64,          // 最大时间步长
    pub total_time: f64,      // 总分析时间
    pub output_interval: i32, // 输出间隔（每N步输出一次）
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientAnalysisConfig {
    pub name: String,
    pub mesh_id: String,
    pub load_curves: Vec<LoadCurve>,
    pub damping: RayleighDamping,
    pub time_control: TimeStepControl,
    pub initial_conditions: Vec<f64>,  // 初始位移/速度
}

// ============ 时域结果 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientResultStep {
    pub time: f64,
    pub displacements: Vec<f64>,      // 每个节点的位移
    pub velocities: Vec<f64>,          // 每个节点的速度
    pub accelerations: Vec<f64>,       // 每个节点的加速度
    pub stresses: Vec<f64>,           // 每个单元的应力
    pub max_displacement: f64,
    pub max_stress: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientResults {
    pub analysis_name: String,
    pub total_time: f64,
    pub steps: Vec<TransientResultStep>,
    pub max_displacement_overall: f64,
    pub max_stress_overall: f64,
    pub natural_frequencies: Vec<f64>,
}

// ============ 示例案例 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialExample {
    pub id: String,
    pub name: String,
    pub description: String,
    pub config: TransientAnalysisConfig,
}

pub fn get_sdolf_example() -> TutorialExample {
    // 单自由度振动示例
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

pub fn get_cantilever_impact_example() -> TutorialExample {
    // 悬臂梁冲击示例
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

// ============ CalculiX INP 生成 ============

pub fn generate_transient_inp(
    config: &TransientAnalysisConfig,
    nodes: &[(usize, f64, f64, f64)],
    elements: &[(usize, String, Vec<usize>)],
    boundary_faces: &[(usize, String)],  // (node_id, face_type)
    output_path: &PathBuf,
) -> Result<(), TransientError> {
    let mut content = String::new();
    
    // Header
    writeln!(content, "** Transient Dynamics Analysis - {}", config.name)?;
    writeln!(content, "** Generated by CAELab")?;
    writeln!(content, "**")?;
    
    // Nodes
    writeln!(content, "*NODE")?;
    for (id, x, y, z) in nodes {
        writeln!(content, "{}, {}, {}, {}", id, x, y, z)?;
    }
    
    // Elements
    writeln!(content, "\n*ELEMENT, TYPE=C3D8")?;
    for (id, _, node_ids) in elements {
        writeln!(content, "{}, {}", id, node_ids.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "))?;
    }
    
    // Materials
    writeln!(content, "\n** Material - Steel")?;
    writeln!(content, "*MATERIAL, NAME=STEEL")?;
    writeln!(content, "*ELASTIC")?;
    writeln!(content, "210000, 0.3")?;
    writeln!(content, "*DENSITY")?;
    writeln!(content, "7850")?;
    
    // Rayleigh Damping
    writeln!(content, "\n** Rayleigh Damping")?;
    writeln!(content, "*DAMPING, ALPHA={}, BETA={}", config.damping.alpha, config.damping.beta)?;
    
    // Section
    writeln!(content, "\n*SOLID SECTION, MATERIAL=STEEL, ELEMENT TYPE=C3D8")?;
    writeln!(content, ", 1.0")?;
    
    // Boundary Conditions (fixed end)
    writeln!(content, "\n** Boundary Conditions")?;
    writeln!(content, "*BOUNDARY")?;
    for (node_id, face_type) in boundary_faces {
        if face_type == "fixed" {
            writeln!(content, "{}, 1, 6", node_id)?;
        }
    }
    
    // Load Curves (Amplitude definitions)
    writeln!(content, "\n** Load Curves")?;
    for (i, curve) in config.load_curves.iter().enumerate() {
        let amp_name = format!("LOAD_{}", i + 1);
        writeln!(content, "*AMPLITUDE, NAME={}", amp_name)?;
        for point in &curve.points {
            writeln!(content, "{}, {}", point.time, point.value)?;
        }
    }
    
    // Time Step Control
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
    
    // Loads
    writeln!(content, "\n** Loads")?;
    for (i, curve) in config.load_curves.iter().enumerate() {
        let amp_name = format!("LOAD_{}", i + 1);
        writeln!(content, "*CLOAD")?;
        // Apply to free end nodes
        for (node_id, face_type) in boundary_faces {
            if face_type == "free" {
                writeln!(content, "{}, 3, {}, 1.0", node_id, amp_name)?;
            }
        }
    }
    
    // Output requests
    writeln!(content, "\n** Output Requests")?;
    writeln!(content, "*NODE FILE, OUTPUT=3D")?;
    writeln!(content, "U, V, A")?;
    writeln!(content, "*EL FILE, OUTPUT=3D")?;
    writeln!(content, "S, E")?;
    writeln!(content, "*END STEP")?;
    
    std::fs::write(output_path, content)?;
    Ok(())
}

// ============ 简化时域仿真（用于演示） ============

pub fn simulate_transient_response(
    config: &TransientAnalysisConfig,
    num_nodes: usize,
) -> TransientResults {
    let mut steps = Vec::new();
    
    // 简化的 Newmark-β 积分
    let dt = config.time_control.initial_dt;
    let t_end = config.time_control.total_time;
    let damping_ratio = config.damping.alpha / (2.0 * (config.damping.frequency1 + config.damping.frequency2) / 2.0 * 2.0 * std::f64::consts::PI);
    
    let mut displacements = vec![0.0; num_nodes];
    let mut velocities = vec![0.0; num_nodes];
    let mut accelerations = vec![0.0; num_nodes];
    
    // 初始条件
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
        
        accelerations[0] = (force - damping_ratio * velocities[0] - stiffness * displacements[0]) / mass;
        velocities[0] += accelerations[0] * dt;
        displacements[0] += velocities[0] * dt;
        
        // 计算应力
        let stress = stiffness * displacements[0].abs();
        
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

fn calculate_load_at_time(curves: &[LoadCurve], time: f64) -> f64 {
    let mut total_load = 0.0;
    for curve in curves {
        match curve.load_type {
            LoadType::Step => {
                if time >= 0.0 {
                    total_load += curve.amplitude;
                }
            }
            LoadType::Sinusoid => {
                let t = time % curve.duration;
                total_load += curve.amplitude * (2.0 * std::f64::consts::PI * curve.frequency * t + curve.phase).sin();
            }
            LoadType::Impulse => {
                // 计算脉冲载荷
                let impulse_total: f64 = curve.points.iter().map(|p| p.value).sum::<f64>() * 0.001;
                if time < 0.01 {
                    total_load += curve.amplitude * (-((time - 0.005) / 0.005).powi(2) + 1.0);
                }
            }
            LoadType::Custom => {
                // 线性插值
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

#[tauri::command]
pub fn get_tutorial_examples() -> Vec<TutorialExample> {
    vec![get_sdolf_example(), get_cantilever_impact_example()]
}

#[tauri::command]
pub fn generate_transient_inp_file(
    config_json: String,
    output_path: String,
) -> Result<String, String> {
    let config: TransientAnalysisConfig = serde_json::from_str(&config_json)
        .map_err(|e| format!("Failed to parse config: {}", e))?;
    
    let path = PathBuf::from(&output_path);
    
    // 生成示例网格数据
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

#[tauri::command]
pub fn run_transient_simulation(
    config_json: String,
) -> Result<TransientResults, String> {
    let config: TransientAnalysisConfig = serde_json::from_str(&config_json)
        .map_err(|e| format!("Failed to parse config: {}", e))?;
    
    // 模拟计算
    let results = simulate_transient_response(&config, 10);
    
    Ok(results)
}

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