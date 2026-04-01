//! Explicit Dynamics Analysis Command Module
//! 支持高速冲击、碰撞、爆炸、大变形等显式动力学分析
//! 使用 *DATASET 格式的显式动力学步，支持质量缩放、时间步控制

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use thiserror::Error;

// ============ 错误类型 ============

#[derive(Debug, Error)]
pub enum ExplicitDynamicsError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid parameter: {0}")]
    InvalidParam(String),
    #[error("Mesh not generated: {0}")]
    MeshNotFound(String),
    #[error("Material not defined: {0}")]
    MaterialNotFound(String),
}

// ============ 分析类型 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisType {
    HighSpeedImpact,   // 高速冲击
    Collision,         // 碰撞
    Explosion,         // 爆炸
    LargeDeformation,  // 大变形
}

// ============ 材料模型 ============

/// 弹塑性材料模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElastoPlasticMaterial {
    pub id: String,
    pub name: String,
    pub density: f64,                    // 密度 kg/m³
    pub youngs_modulus: f64,              // 弹性模量 MPa
    pub poisson_ratio: f64,              // 泊松比
    pub yield_strength: f64,             // 屈服强度 MPa
    pub tangent_modulus: f64,            // 切线模量 MPa (塑性段斜率)
    pub hardening_type: String,           // 'isotropic' | 'kinematic' | 'combined'
    pub failure_strain: Option<f64>,     // 断裂应变 (可选)
}

/// 断裂失效模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FailureModel {
    None,
    MaximumStrain(f64),                 // 最大主应变失效
    MaximumStress(f64),                  // 最大应力失效
    JohnsonCook {                        // Johnson-Cook 断裂模型
        c1: f64,
        c2: f64,
        c3: f64,
        c4: f64,
        c5: f64,
        melt_temp: f64,
        ref_strain_rate: f64,
        room_temp: f64,
    },
    PlasticStrainEnergy(f64),            // 塑性应变能失效
}

/// 接触碰撞对配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPair {
    pub id: String,
    pub name: String,
    pub master_surface: String,          // 主表面名称
    pub slave_surface: String,           // 从表面名称
    pub contact_type: String,            // 'surface_to_surface' | 'node_to_surface'
    pub friction: f64,                   // 摩擦系数
    pub contact_stiffness: f64,          // 接触刚度
    pub clearance: f64,                  // 初始间隙
    pub damping: f64,                    // 接触阻尼
}

/// 初始速度条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialVelocity {
    pub surface_name: String,            // 施加初始速度的表面
    pub velocity_x: f64,                 // X方向速度 m/s
    pub velocity_y: f64,                 // Y方向速度 m/s
    pub velocity_z: f64,                 // Z方向速度 m/s
    pub angular_velocity_x: f64,         // 角速度 (可选) rad/s
    pub angular_velocity_y: f64,
    pub angular_velocity_z: f64,
}

/// 求解控制参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverControl {
    pub auto_time_step: bool,            // 自动时间步长计算
    pub initial_dt: f64,                 // 初始时间步长 s
    pub min_dt: f64,                     // 最小时间步长 s
    pub max_dt: f64,                     // 最大时间步长 s
    pub mass_scaling: bool,              // 是否启用质量缩放
    pub mass_scaling_factor: f64,        // 质量缩放系数
    pub critical_time_step_ratio: f64,   // 临界时间步比例
    pub energy_dissipation_ratio: f64,   // 能量耗散率阈值
    pub output_frequency: i32,           // 输出频率（每N步输出）
    pub total_time: f64,                 // 总分析时间 s
}

/// 完整显式动力学分析配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplicitDynamicsConfig {
    pub name: String,
    pub analysis_type: AnalysisType,
    pub material: ElastoPlasticMaterial,
    pub failure_model: FailureModel,
    pub contact_pairs: Vec<ContactPair>,
    pub initial_velocities: Vec<InitialVelocity>,
    pub solver_control: SolverControl,
    pub output_dir: String,
}

// ============ 显式动力学结果 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplicitResultStep {
    pub time: f64,
    pub frame_index: i32,
    pub displacements: Vec<f64>,         // 节点位移
    pub velocities: Vec<f64>,             // 节点速度
    pub accelerations: Vec<f64>,          // 节点加速度
    pub stresses: Vec<f64>,              // Von Mises 应力
    pub strains: Vec<f64>,               // 等效塑性应变
    pub kinetic_energy: f64,             // 动能
    pub internal_energy: f64,            // 内能
    pub max_displacement: f64,
    pub max_velocity: f64,
    pub max_stress: f64,
    pub max_plastic_strain: f64,
    pub damage: Option<f64>,             // 损伤变量 (0-1)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplicitDynamicsResults {
    pub analysis_name: String,
    pub analysis_type: AnalysisType,
    pub total_time: f64,
    pub num_frames: i32,
    pub steps: Vec<ExplicitResultStep>,
    pub max_displacement: f64,
    pub max_velocity: f64,
    pub max_stress: f64,
    pub max_plastic_strain: f64,
    pub energy_balance: ExplicitEnergyBalance,
    pub failure_locations: Vec<FailureLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplicitEnergyBalance {
    pub initial_kinetic_energy: f64,
    pub final_kinetic_energy: f64,
    pub max_kinetic_energy: f64,
    pub internal_energy_gained: f64,
    pub energy_dissipated: f64,
    pub hourglass_energy: f64,
    pub artificial_energy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureLocation {
    pub node_id: i32,
    pub time: f64,
    pub frame: i32,
    pub damage: f64,
    pub plastic_strain: f64,
    pub position: [f64; 3],
}

// ============ 示例模板 ============

/// 子弹冲击钢板示例
pub fn get_bullet_impact_template() -> TemplateExample {
    TemplateExample {
        id: "bullet_impact".to_string(),
        name: "子弹冲击钢板".to_string(),
        description: "模拟子弹以高速撞击钢板的过程，分析应力波传播、塑性变形和可能的穿透/回弹。适用于金属动态响应研究。".to_string(),
        config: get_bullet_impact_config(),
    }
}

/// 落锤冲击试验示例
pub fn get_drop_hammer_template() -> TemplateExample {
    TemplateExample {
        id: "drop_hammer".to_string(),
        name: "落锤冲击试验".to_string(),
        description: "模拟落锤冲击试验，测量材料的冲击韧性和动态响应特性。适用于Charpy冲击试验等标准测试。".to_string(),
        config: get_drop_hammer_config(),
    }
}

/// 爆炸载荷示例
pub fn get_explosion_template() -> TemplateExample {
    TemplateExample {
        id: "explosion".to_string(),
        name: "爆炸载荷分析".to_string(),
        description: "模拟爆炸冲击波对结构的瞬态作用，分析结构在极端载荷下的响应和破坏模式。".to_string(),
        config: get_explosion_config(),
    }
}

/// 大变形屈曲示例
pub fn get_large_deformation_template() -> TemplateExample {
    TemplateExample {
        id: "large_deformation".to_string(),
        name: "大变形分析".to_string(),
        description: "模拟结构的大挠度和大转动问题，如金属成型、薄壁结构屈曲等几何非线性分析。".to_string(),
        config: get_large_deformation_config(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateExample {
    pub id: String,
    pub name: String,
    pub description: String,
    pub config: ExplicitDynamicsConfig,
}

fn get_bullet_impact_config() -> ExplicitDynamicsConfig {
    ExplicitDynamicsConfig {
        name: "子弹冲击钢板".to_string(),
        analysis_type: AnalysisType::HighSpeedImpact,
        material: ElastoPlasticMaterial {
            id: "steel_impact".to_string(),
            name: "高强度钢".to_string(),
            density: 7850.0,
            youngs_modulus: 210000.0,
            poisson_ratio: 0.3,
            yield_strength: 250.0,
            tangent_modulus: 1000.0,
            hardening_type: "combined".to_string(),
            failure_strain: Some(0.25),
        },
        failure_model: FailureModel::JohnsonCook {
            c1: 0.13,
            c2: 0.13,
            c3: 1.5,
            c4: 0.014,
            c5: -0.02,
            melt_temp: 1800.0,
            ref_strain_rate: 1.0,
            room_temp: 293.0,
        },
        contact_pairs: vec![
            ContactPair {
                id: "bullet_plate".to_string(),
                name: "子弹-钢板接触".to_string(),
                master_surface: "bullet_surface".to_string(),
                slave_surface: "plate_surface".to_string(),
                contact_type: "surface_to_surface".to_string(),
                friction: 0.3,
                contact_stiffness: 1e8,
                clearance: 0.0,
                damping: 1e5,
            },
        ],
        initial_velocities: vec![
            InitialVelocity {
                surface_name: "bullet_surface".to_string(),
                velocity_x: 800.0,
                velocity_y: 0.0,
                velocity_z: 0.0,
                angular_velocity_x: 0.0,
                angular_velocity_y: 0.0,
                angular_velocity_z: 0.0,
            },
        ],
        solver_control: SolverControl {
            auto_time_step: true,
            initial_dt: 1e-8,
            min_dt: 1e-10,
            max_dt: 1e-5,
            mass_scaling: true,
            mass_scaling_factor: 0.1,
            critical_time_step_ratio: 0.9,
            energy_dissipation_ratio: 0.1,
            output_frequency: 100,
            total_time: 0.001,
        },
        output_dir: "/tmp/explicit/bullet_impact".to_string(),
    }
}

fn get_drop_hammer_config() -> ExplicitDynamicsConfig {
    ExplicitDynamicsConfig {
        name: "落锤冲击试验".to_string(),
        analysis_type: AnalysisType::Collision,
        material: ElastoPlasticMaterial {
            id: "test_material".to_string(),
            name: "低碳钢".to_string(),
            density: 7850.0,
            youngs_modulus: 210000.0,
            poisson_ratio: 0.3,
            yield_strength: 200.0,
            tangent_modulus: 500.0,
            hardening_type: "isotropic".to_string(),
            failure_strain: Some(0.15),
        },
        failure_model: FailureModel::MaximumStrain(0.2),
        contact_pairs: vec![
            ContactPair {
                id: "hammer_specimen".to_string(),
                name: "锤头-试样接触".to_string(),
                master_surface: "hammer_surface".to_string(),
                slave_surface: "specimen_surface".to_string(),
                contact_type: "surface_to_surface".to_string(),
                friction: 0.1,
                contact_stiffness: 1e7,
                clearance: 0.0,
                damping: 5000.0,
            },
        ],
        initial_velocities: vec![
            InitialVelocity {
                surface_name: "hammer_surface".to_string(),
                velocity_x: 0.0,
                velocity_y: -5.0,
                velocity_z: 0.0,
                angular_velocity_x: 0.0,
                angular_velocity_y: 0.0,
                angular_velocity_z: 0.0,
            },
        ],
        solver_control: SolverControl {
            auto_time_step: true,
            initial_dt: 1e-7,
            min_dt: 1e-9,
            max_dt: 1e-4,
            mass_scaling: true,
            mass_scaling_factor: 0.2,
            critical_time_step_ratio: 0.85,
            energy_dissipation_ratio: 0.05,
            output_frequency: 50,
            total_time: 0.01,
        },
        output_dir: "/tmp/explicit/drop_hammer".to_string(),
    }
}

fn get_explosion_config() -> ExplicitDynamicsConfig {
    ExplicitDynamicsConfig {
        name: "爆炸载荷分析".to_string(),
        analysis_type: AnalysisType::Explosion,
        material: ElastoPlasticMaterial {
            id: "structure_material".to_string(),
            name: "铝合金".to_string(),
            density: 2700.0,
            youngs_modulus: 70000.0,
            poisson_ratio: 0.33,
            yield_strength: 270.0,
            tangent_modulus: 500.0,
            hardening_type: "kinematic".to_string(),
            failure_strain: Some(0.05),
        },
        failure_model: FailureModel::MaximumStress(400.0),
        contact_pairs: vec![],
        initial_velocities: vec![],
        solver_control: SolverControl {
            auto_time_step: true,
            initial_dt: 1e-8,
            min_dt: 1e-11,
            max_dt: 1e-6,
            mass_scaling: false,
            mass_scaling_factor: 0.0,
            critical_time_step_ratio: 0.95,
            energy_dissipation_ratio: 0.2,
            output_frequency: 200,
            total_time: 0.0005,
        },
        output_dir: "/tmp/explicit/explosion".to_string(),
    }
}

fn get_large_deformation_config() -> ExplicitDynamicsConfig {
    ExplicitDynamicsConfig {
        name: "大变形分析".to_string(),
        analysis_type: AnalysisType::LargeDeformation,
        material: ElastoPlasticMaterial {
            id: "sheet_metal".to_string(),
            name: "薄板材料".to_string(),
            density: 2700.0,
            youngs_modulus: 70000.0,
            poisson_ratio: 0.33,
            yield_strength: 100.0,
            tangent_modulus: 2000.0,
            hardening_type: "combined".to_string(),
            failure_strain: Some(0.4),
        },
        failure_model: FailureModel::PlasticStrainEnergy(50.0),
        contact_pairs: vec![],
        initial_velocities: vec![
            InitialVelocity {
                surface_name: "loading_surface".to_string(),
                velocity_x: 0.0,
                velocity_y: -0.5,
                velocity_z: 0.0,
                angular_velocity_x: 0.0,
                angular_velocity_y: 0.0,
                angular_velocity_z: 0.0,
            },
        ],
        solver_control: SolverControl {
            auto_time_step: true,
            initial_dt: 1e-6,
            min_dt: 1e-8,
            max_dt: 1e-3,
            mass_scaling: true,
            mass_scaling_factor: 0.3,
            critical_time_step_ratio: 0.8,
            energy_dissipation_ratio: 0.01,
            output_frequency: 20,
            total_time: 0.1,
        },
        output_dir: "/tmp/explicit/large_deformation".to_string(),
    }
}

// ============ INP 生成 ============

/// 生成显式动力学分析的INP文件
pub fn generate_explicit_dynamics_inp(
    config: &ExplicitDynamicsConfig,
    nodes: &[(i32, f64, f64, f64)],
    elements: &[(i32, String, Vec<i32>)],
    surfaces: &[(String, String, Vec<i32>, Vec<String>)],
    output_path: &str,
) -> Result<String, ExplicitDynamicsError> {
    let mut inp = String::new();

    // Header
    inp.push_str("*HEADING\n");
    inp.push_str(&format!("{}\n", config.name));
    inp.push_str(&format!("Analysis Type: {:?}\n", config.analysis_type));

    // Node section
    inp.push_str("\n*NODE\n");
    for &(id, x, y, z) in nodes {
        inp.push_str(&format!("{}, {}, {}, {}\n", id, x, y, z));
    }

    // Element section
    inp.push_str("\n*ELEMENT, TYPE=C3D8R\n");
    let mut elem_idx = 0usize;
    for &(id, ref elem_type, ref node_ids) in elements {
        if elem_type == "C3D8R" || elem_type == "C3D8" {
            inp.push_str(&format!("{}", id));
            for &nid in node_ids {
                inp.push_str(&format!(", {}", nid));
            }
            inp.push_str("\n");
            elem_idx += 1;
        }
    }

    // Material definition - Elasto-Plastic
    inp.push_str("\n*MATERIAL, NAME=ExplicitMaterial\n");
    inp.push_str(&format!("*DENSITY\n{:.6e}\n", config.material.density));
    inp.push_str("*ELASTIC\n");
    inp.push_str(&format!("{:.1}, {:.4}\n", config.material.youngs_modulus, config.material.poisson_ratio));

    // Plastic hardening
    inp.push_str("*PLASTIC\n");
    inp.push_str(&format!("{:.1}, 0.0\n", config.material.yield_strength));
    inp.push_str(&format!(
        "{:.1}, {:.6}\n",
        config.material.youngs_modulus * 0.05,
        config.material.tangent_modulus / config.material.youngs_modulus * 0.01
    ));

    // Section assignment
    inp.push_str("\n*SOLID SECTION, MATERIAL=ExplicitMaterial, ELSET=EALL\n");

    // Failure model
    match &config.failure_model {
        FailureModel::None => {}
        FailureModel::MaximumStrain(strain) => {
            inp.push_str("\n*DAMAGE, TYPE=MAX_STRAIN, SOFTENING=HARDENING\n");
            inp.push_str(&format!("{:.6}\n", strain));
        }
        FailureModel::MaximumStress(stress) => {
            inp.push_str("\n*DAMAGE, TYPE=MAX_STRESS\n");
            inp.push_str(&format!("{:.1}\n", stress));
        }
        FailureModel::JohnsonCook { c1, c2, c3, c4, c5, melt_temp, ref_strain_rate, room_temp } => {
            inp.push_str("\n*DAMAGE, TYPE=JOHNSON_COOK\n");
            inp.push_str(&format!("{:.4}, {:.4}, {:.4}, {:.4}, {:.4}\n", c1, c2, c3, c4, c5));
            inp.push_str(&format!("{:.1}, {:.6}, {:.1}\n", melt_temp, ref_strain_rate, room_temp));
        }
        FailureModel::PlasticStrainEnergy(energy) => {
            inp.push_str("\n*DAMAGE, TYPE=PLASTIC_ENERGY\n");
            inp.push_str(&format!("{:.4}\n", energy));
        }
    }

    // Surface definitions
    for &(ref surf_name, _, _, _) in surfaces {
        inp.push_str(&format!("\n*SURFACE, NAME={}\n", surf_name));
    }

    // Initial velocities
    for iv in &config.initial_velocities {
        inp.push_str(&format!(
            "\n*INITIAL CONDITIONS, TYPE=VELOCITY\n{}, {:.4}, {:.4}, {:.4}\n",
            iv.surface_name, iv.velocity_x, iv.velocity_y, iv.velocity_z
        ));
    }

    // Contact pairs
    for cp in &config.contact_pairs {
        inp.push_str("\n*CONTACT\n");
        inp.push_str(&format!("{}, {}\n", cp.master_surface, cp.slave_surface));
        inp.push_str("*CONTACT PROPERTY\n");
        inp.push_str(&format!("Friction={:.3}, Penalty={:.2e}\n", cp.friction, cp.contact_stiffness));
    }

    // Boundary conditions (optional)
    // Speed up boundary conditions if defined

    // Dynamic explicit step
    inp.push_str("\n*STEP, NLGEOM=YES, INC=100000\n");
    inp.push_str("*DYNAMIC, EXPLICIT\n");

    // Time control
    let sc = &config.solver_control;
    inp.push_str(&format!(
        ", {:.6e}, {:.6e}, {:.6e}, {:.6e}\n",
        sc.initial_dt, sc.total_time, sc.min_dt, sc.max_dt
    ));

    // Mass scaling
    if sc.mass_scaling {
        inp.push_str(&format!(
            "*MASS SCALING, DT={:.6e}, FACTOR={:.4}\n",
            sc.initial_dt * sc.critical_time_step_ratio,
            sc.mass_scaling_factor
        ));
    }

    // Output control - FRD for animation
    inp.push_str("\n*OUTPUT, FIELD, FREQUENCY=1\n");
    inp.push_str("U, V, A, S, PEEQ, DAMD\n");
    inp.push_str("*OUTPUT, HISTORY\n");
    inp.push_str(&format!("ENERGY={}\n", sc.output_frequency));
    inp.push_str("ALLKI, ALLIE, ALLAE\n");

    // End step
    inp.push_str("\n*END STEP\n");

    // Write to file
    std::fs::write(output_path, &inp)?;

    Ok(inp)
}

/// 生成多个显式动力学步骤的INP (用于时域动画)
pub fn generate_explicit_with_steps(
    config: &ExplicitDynamicsConfig,
    nodes: &[(i32, f64, f64, f64)],
    elements: &[(i32, String, Vec<i32>)],
    num_steps: i32,
    output_path: &str,
) -> Result<String, ExplicitDynamicsError> {
    let mut inp = String::new();

    // Same header and model definition
    inp.push_str("*HEADING\n");
    inp.push_str(&format!("{} - Multi-step Animation\n", config.name));

    // Nodes
    inp.push_str("\n*NODE\n");
    for &(id, x, y, z) in nodes {
        inp.push_str(&format!("{}, {}, {}, {}\n", id, x, y, z));
    }

    // Elements
    inp.push_str("\n*ELEMENT, TYPE=C3D8R\n");
    for &(id, _, ref node_ids) in elements {
        inp.push_str(&format!("{}", id));
        for &nid in node_ids {
            inp.push_str(&format!(", {}", nid));
        }
        inp.push_str("\n");
    }

    // Material
    inp.push_str("\n*MATERIAL, NAME=ExplicitMaterial\n");
    inp.push_str(&format!("*DENSITY\n{:.6e}\n", config.material.density));
    inp.push_str("*ELASTIC\n");
    inp.push_str(&format!("{:.1}, {:.4}\n", config.material.youngs_modulus, config.material.poisson_ratio));
    inp.push_str("*PLASTIC\n");
    inp.push_str(&format!("{:.1}, 0.0\n", config.material.yield_strength));

    inp.push_str("\n*SOLID SECTION, MATERIAL=ExplicitMaterial, ELSET=EALL\n");

    // Multiple steps for animation
    let dt = config.solver_control.total_time / num_steps as f64;
    for step in 1..=num_steps {
        let t_end = step as f64 * dt;
        inp.push_str(&format!("\n*STEP, NAME=Step{}, NLGEOM=YES\n", step));
        inp.push_str("*DYNAMIC, EXPLICIT\n");
        inp.push_str(&format!(
            "{:.6e}, {:.6e}, {:.6e}, {:.6e}\n",
            dt, t_end, config.solver_control.min_dt, config.solver_control.max_dt
        ));

        if step == 1 {
            for iv in &config.initial_velocities {
                inp.push_str(&format!(
                    "*INITIAL CONDITIONS, TYPE=VELOCITY\n{}, {:.4}, {:.4}, {:.4}\n",
                    iv.surface_name, iv.velocity_x, iv.velocity_y, iv.velocity_z
                ));
            }
        }

        // Output each step
        inp.push_str("*OUTPUT, FIELD\n");
        inp.push_str("U, V, S, PEEQ\n");
        inp.push_str("*END STEP\n");
    }

    std::fs::write(output_path, &inp)?;
    Ok(inp)
}

/// 获取显式动力学模板列表
#[tauri::command]
pub fn get_explicit_dynamics_templates() -> Vec<TemplateExample> {
    vec![
        get_bullet_impact_template(),
        get_drop_hammer_template(),
        get_explosion_template(),
        get_large_deformation_template(),
    ]
}

/// 获取指定模板的详细配置
#[tauri::command]
pub fn get_explicit_dynamics_template(id: String) -> Option<TemplateExample> {
    match id.as_str() {
        "bullet_impact" => Some(get_bullet_impact_template()),
        "drop_hammer" => Some(get_drop_hammer_template()),
        "explosion" => Some(get_explosion_template()),
        "large_deformation" => Some(get_large_deformation_template()),
        _ => None,
    }
}

/// 生成显式动力学INP文件
#[tauri::command]
pub fn generate_explicit_dynamics_inp_cmd(
    config: String,
    nodes: String,
    elements: String,
    surfaces: String,
    output_path: String,
) -> Result<String, String> {
    let config: ExplicitDynamicsConfig = serde_json::from_str(&config)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    let nodes: Vec<(i32, f64, f64, f64)> = serde_json::from_str(&nodes)
        .map_err(|e| format!("Failed to parse nodes: {}", e))?;

    let elements: Vec<(i32, String, Vec<i32>)> = serde_json::from_str(&elements)
        .map_err(|e| format!("Failed to parse elements: {}", e))?;

    let surfaces: Vec<(String, String, Vec<i32>, Vec<String>)> = serde_json::from_str(&surfaces)
        .map_err(|e| format!("Failed to parse surfaces: {}", e))?;

    generate_explicit_dynamics_inp(&config, &nodes, &elements, &surfaces, &output_path)
        .map_err(|e| e.to_string())
}

/// 生成带动画步的显式动力学INP文件
#[tauri::command]
pub fn generate_explicit_animation_inp_cmd(
    config: String,
    nodes: String,
    elements: String,
    num_steps: i32,
    output_path: String,
) -> Result<String, String> {
    let config: ExplicitDynamicsConfig = serde_json::from_str(&config)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    let nodes: Vec<(i32, f64, f64, f64)> = serde_json::from_str(&nodes)
        .map_err(|e| format!("Failed to parse nodes: {}", e))?;

    let elements: Vec<(i32, String, Vec<i32>)> = serde_json::from_str(&elements)
        .map_err(|e| format!("Failed to parse elements: {}", e))?;

    generate_explicit_with_steps(&config, &nodes, &elements, num_steps, &output_path)
        .map_err(|e| e.to_string())
}

/// 计算临界时间步长（基于最小单元尺寸和材料参数）
#[tauri::command]
pub fn calculate_critical_time_step(
    nodes: String,
    elements: String,
    density: f64,
    youngs_modulus: f64,
) -> Result<f64, String> {
    let nodes: Vec<(i32, f64, f64, f64)> = serde_json::from_str(&nodes)
        .map_err(|e| format!("Failed to parse nodes: {}", e))?;

    let elements: Vec<(i32, String, Vec<i32>)> = serde_json::from_str(&elements)
        .map_err(|e| format!("Failed to parse elements: {}", e))?;

    // 找到最小单元尺寸
    let mut min_size = f64::MAX;
    for &(_, _, ref node_ids) in &elements {
        if node_ids.len() >= 3 {
            // For C3D8 hexahedral elements, estimate from first 4 nodes
            let coords: Vec<[f64; 3]> = node_ids.iter()
                .filter_map(|&nid| nodes.iter().find(|&&(id, _, _, _)| id == nid))
                .map(|&(_, x, y, z)| [x, y, z])
                .collect();

            if coords.len() >= 4 {
                // Estimate characteristic length from edge lengths
                for i in 0..coords.len() {
                    for j in (i + 1)..coords.len() {
                        let dx = coords[i][0] - coords[j][0];
                        let dy = coords[i][1] - coords[j][1];
                        let dz = coords[i][2] - coords[j][2];
                        let dist = (dx * dx + dy * dy + dz * dz).sqrt();
                        if dist > 1e-10 && dist < min_size {
                            min_size = dist;
                        }
                    }
                }
            }
        }
    }

    if min_size == f64::MAX {
        return Err("Could not compute element sizes".to_string());
    }

    // Sound speed in material
    let sound_speed = ((youngs_modulus * 1e6) / density).sqrt();
    // Critical time step for explicit dynamics: dt = L / c
    let dt = min_size / sound_speed;

    Ok(dt)
}

/// 质量缩放建议计算
#[tauri::command]
pub fn calculate_mass_scaling_suggestion(
    original_dt: f64,
    target_dt: f64,
) -> f64 {
    // Mass scaling factor = (original_dt / target_dt)^2
    // because dt ∝ sqrt(1/m)
    (original_dt / target_dt).powi(2)
}