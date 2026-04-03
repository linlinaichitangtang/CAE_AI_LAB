//! Explicit Dynamics Analysis Command Module
//! 支持高速冲击、碰撞、爆炸、大变形等显式动力学分析
//! 使用 *DATASET 格式的显式动力学步，支持质量缩放、时间步控制

use serde::{Deserialize, Serialize};
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
    let mut _elem_idx = 0usize;
    for &(id, ref elem_type, ref node_ids) in elements {
        if elem_type == "C3D8R" || elem_type == "C3D8" {
            inp.push_str(&format!("{}", id));
            for &nid in node_ids {
                inp.push_str(&format!(", {}", nid));
            }
            inp.push_str("\n");
            _elem_idx += 1;
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

// ============ V1.3-004: 显式动力学求解器 ============

/// 显式求解器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplicitSolverConfig {
    pub nodes: Vec<[f64; 3]>,
    pub elements: Vec<[usize; 8]>,  // HEX8
    pub node_masses: Vec<f64>,
    pub material: ExplicitMaterial,
    pub boundary_conditions: ExplicitBC,
    pub contact_pairs: Vec<ExplicitContactPair>,
    pub time_step: f64,
    pub end_time: f64,
    pub output_interval: u32,
    pub damping: f64,
}

/// 显式求解器材料参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplicitMaterial {
    pub density: f64,
    pub youngs_modulus: f64,
    pub poisson_ratio: f64,
    pub yield_stress: f64,
    pub hardening_modulus: f64,
    pub failure_strain: f64,
}

/// 边界条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplicitBC {
    pub fixed_nodes: Vec<usize>,
    pub prescribed_velocities: Vec<(usize, [f64; 3])>,
    pub initial_velocities: Vec<(usize, [f64; 3])>,
}

/// 接触对
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplicitContactPair {
    pub master_nodes: Vec<usize>,
    pub slave_nodes: Vec<usize>,
    pub penalty_stiffness: f64,
    pub friction_coefficient: f64,
}

/// 单帧结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplicitFrame {
    pub time: f64,
    pub kinetic_energy: f64,
    pub internal_energy: f64,
    pub total_energy: f64,
    pub max_displacement: f64,
    pub max_velocity: f64,
    pub node_displacements: Vec<[f64; 3]>,
    pub node_velocities: Vec<[f64; 3]>,
    pub element_stresses: Vec<f64>,
    pub failed_elements: Vec<usize>,
}

/// 求解器完整结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplicitSolverResult {
    pub frames: Vec<ExplicitFrame>,
    pub num_time_steps: u64,
    pub energy_error_percent: f64,
    pub num_failed_elements: usize,
    pub status: String,
}

/// HEX8 单元高斯积分点 (2x2x2 = 8 points)
const HEX_GP: [[f64; 3]; 8] = [
    [-0.5773502691896257, -0.5773502691896257, -0.5773502691896257],
    [ 0.5773502691896257, -0.5773502691896257, -0.5773502691896257],
    [-0.5773502691896257,  0.5773502691896257, -0.5773502691896257],
    [ 0.5773502691896257,  0.5773502691896257, -0.5773502691896257],
    [-0.5773502691896257, -0.5773502691896257,  0.5773502691896257],
    [ 0.5773502691896257, -0.5773502691896257,  0.5773502691896257],
    [-0.5773502691896257,  0.5773502691896257,  0.5773502691896257],
    [ 0.5773502691896257,  0.5773502691896257,  0.5773502691896257],
];
const HEX_GP_WEIGHT: f64 = 1.0;

/// HEX8 形函数在参考坐标 (xi, eta, zeta) 下的值
fn hex8_shape_functions(xi: f64, eta: f64, zeta: f64) -> [f64; 8] {
    let n = [
        (1.0 - xi) * (1.0 - eta) * (1.0 - zeta) / 8.0,
        (1.0 + xi) * (1.0 - eta) * (1.0 - zeta) / 8.0,
        (1.0 + xi) * (1.0 + eta) * (1.0 - zeta) / 8.0,
        (1.0 - xi) * (1.0 + eta) * (1.0 - zeta) / 8.0,
        (1.0 - xi) * (1.0 - eta) * (1.0 + zeta) / 8.0,
        (1.0 + xi) * (1.0 - eta) * (1.0 + zeta) / 8.0,
        (1.0 + xi) * (1.0 + eta) * (1.0 + zeta) / 8.0,
        (1.0 - xi) * (1.0 + eta) * (1.0 + zeta) / 8.0,
    ];
    n
}

/// HEX8 形函数对参考坐标的导数 (8x3)
fn hex8_shape_derivatives(xi: f64, eta: f64, zeta: f64) -> [[f64; 3]; 8] {
    [
        [-(1.0-eta)*(1.0-zeta)/8.0, -(1.0-xi)*(1.0-zeta)/8.0, -(1.0-xi)*(1.0-eta)/8.0],
        [ (1.0-eta)*(1.0-zeta)/8.0, -(1.0+xi)*(1.0-zeta)/8.0, -(1.0+xi)*(1.0-eta)/8.0],
        [ (1.0+eta)*(1.0-zeta)/8.0,  (1.0+xi)*(1.0-zeta)/8.0, -(1.0+xi)*(1.0+eta)/8.0],
        [-(1.0+eta)*(1.0-zeta)/8.0,  (1.0-xi)*(1.0-zeta)/8.0, -(1.0-xi)*(1.0+eta)/8.0],
        [-(1.0-eta)*(1.0+zeta)/8.0, -(1.0-xi)*(1.0+zeta)/8.0,  (1.0-xi)*(1.0-eta)/8.0],
        [ (1.0-eta)*(1.0+zeta)/8.0, -(1.0+xi)*(1.0+zeta)/8.0,  (1.0+xi)*(1.0-eta)/8.0],
        [ (1.0+eta)*(1.0+zeta)/8.0,  (1.0+xi)*(1.0+zeta)/8.0,  (1.0+xi)*(1.0+eta)/8.0],
        [-(1.0+eta)*(1.0+zeta)/8.0,  (1.0-xi)*(1.0+zeta)/8.0,  (1.0-xi)*(1.0+eta)/8.0],
    ]
}

/// 3x3 矩阵行列式
fn det3x3(m: &[[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1]*m[2][2] - m[1][2]*m[2][1])
  - m[0][1] * (m[1][0]*m[2][2] - m[1][2]*m[2][0])
  + m[0][2] * (m[1][0]*m[2][1] - m[1][1]*m[2][0])
}

/// 3x3 矩阵求逆
fn inv3x3(m: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let d = det3x3(m);
    if d.abs() < 1e-30 {
        return [[0.0; 3]; 3];
    }
    let id = 1.0 / d;
    [
        [
            (m[1][1]*m[2][2] - m[1][2]*m[2][1]) * id,
            (m[0][2]*m[2][1] - m[0][1]*m[2][2]) * id,
            (m[0][1]*m[1][2] - m[0][2]*m[1][1]) * id,
        ],
        [
            (m[1][2]*m[2][0] - m[1][0]*m[2][2]) * id,
            (m[0][0]*m[2][2] - m[0][2]*m[2][0]) * id,
            (m[0][2]*m[1][0] - m[0][0]*m[1][2]) * id,
        ],
        [
            (m[1][0]*m[2][1] - m[1][1]*m[2][0]) * id,
            (m[0][1]*m[2][0] - m[0][0]*m[2][1]) * id,
            (m[0][0]*m[1][1] - m[0][1]*m[1][0]) * id,
        ],
    ]
}

/// Von Mises 等效应力
fn von_mises_stress(s: &[f64; 6]) -> f64 {
    // s = [s11, s22, s33, s12, s23, s13]
    let s11 = s[0]; let s22 = s[1]; let s33 = s[2];
    let s12 = s[3]; let s23 = s[4]; let s13 = s[5];
    ((s11 - s22).powi(2) + (s22 - s33).powi(2) + (s33 - s11).powi(2)
     + 6.0 * (s12*s12 + s23*s23 + s13*s13)).sqrt() / std::f64::consts::SQRT_2
}

/// 计算单元应变和应力，返回内力向量
fn compute_element_internal_forces(
    elem_nodes: &[[f64; 8]; 3], // [x[8], y[8], z[8]]
    displacements: &[[f64; 3]],
    elem: &[usize; 8],
    mat: &ExplicitMaterial,
    plastic_strain: &mut f64,
) -> (Vec<[f64; 3]>, f64, f64, bool) {
    // (nodal_forces, von_mises, strain_energy_density, is_failed)
    let e = mat.youngs_modulus;
    let nu = mat.poisson_ratio;
    let lambda = e * nu / ((1.0 + nu) * (1.0 - 2.0 * nu));
    let mu = e / (2.0 * (1.0 + nu));

    let mut nodal_forces: Vec<[f64; 3]> = vec![[0.0; 3]; 8];
    let mut total_vm = 0.0;
    let mut total_se = 0.0;
    let mut is_failed = false;

    // 当前节点位移
    let u: [[f64; 3]; 8] = std::array::from_fn(|i| displacements[elem[i]]);

    for gp in &HEX_GP {
        let xi = gp[0]; let eta = gp[1]; let zeta = gp[2];

        // 形函数导数
        let dnds = hex8_shape_derivatives(xi, eta, zeta);

        // Jacobian: J = dN/dxi * x
        let mut j: [[f64; 3]; 3] = [[0.0; 3]; 3];
        for i in 0..8 {
            for a in 0..3 {
                j[0][a] += dnds[i][0] * elem_nodes[a][i];
                j[1][a] += dnds[i][1] * elem_nodes[a][i];
                j[2][a] += dnds[i][2] * elem_nodes[a][i];
            }
        }

        let det_j = det3x3(&j);
        if det_j.abs() < 1e-20 { continue; }
        let jinv = inv3x3(&j);

        // dN/dx = J^-1 * dN/dxi
        let mut dndx: [[f64; 3]; 8] = [[0.0; 3]; 8];
        for i in 0..8 {
            for a in 0..3 {
                dndx[i][a] = jinv[0][a] * dnds[i][0] + jinv[1][a] * dnds[i][1] + jinv[2][a] * dnds[i][2];
            }
        }

        // B matrix (6x24): strain = B * u
        // B for node i: [[dNi/dx, 0, 0], [0, dNi/dy, 0], [0, 0, dNi/dz],
        //                 [dNi/dy, dNi/dx, 0], [0, dNi/dz, dNi/dy], [dNi/dz, 0, dNi/dx]]
        let mut strain: [f64; 6] = [0.0; 6];
        for i in 0..8 {
            strain[0] += dndx[i][0] * u[i][0];
            strain[1] += dndx[i][1] * u[i][1];
            strain[2] += dndx[i][2] * u[i][2];
            strain[3] += dndx[i][1] * u[i][0] + dndx[i][0] * u[i][1];
            strain[4] += dndx[i][2] * u[i][1] + dndx[i][1] * u[i][2];
            strain[5] += dndx[i][2] * u[i][0] + dndx[i][0] * u[i][2];
        }

        // 应力增量 (弹性部分)
        let trace = strain[0] + strain[1] + strain[2];
        let mut stress: [f64; 6] = [
            lambda * trace + 2.0 * mu * strain[0],
            lambda * trace + 2.0 * mu * strain[1],
            lambda * trace + 2.0 * mu * strain[2],
            2.0 * mu * strain[3],
            2.0 * mu * strain[4],
            2.0 * mu * strain[5],
        ];

        let vm = von_mises_stress(&stress);
        total_vm += vm;

        // 塑性修正 (J2 plasticity, return mapping)
        if vm > mat.yield_stress && mat.hardening_modulus > 0.0 {
            let delta_gamma = (vm - mat.yield_stress) / (3.0 * mu + mat.hardening_modulus);
            *plastic_strain += delta_gamma;
            let factor = 1.0 - delta_gamma * 3.0 * mu / vm;
            for s in &mut stress {
                *s *= factor;
            }
            // 检查失效
            if *plastic_strain > mat.failure_strain {
                is_failed = true;
            }
        }

        // 应变能密度
        let se: f64 = 0.5 * (stress[0]*strain[0] + stress[1]*strain[1] + stress[2]*strain[2]
            + stress[3]*strain[3] + stress[4]*strain[4] + stress[5]*strain[5]);
        total_se += se;

        // 内力 = B^T * stress * detJ * weight
        for i in 0..8 {
            let b0 = dndx[i][0]; let b1 = dndx[i][1]; let b2 = dndx[i][2];
            nodal_forces[i][0] -= (stress[0]*b0 + stress[3]*b1 + stress[5]*b2) * det_j * HEX_GP_WEIGHT;
            nodal_forces[i][1] -= (stress[3]*b0 + stress[1]*b1 + stress[4]*b2) * det_j * HEX_GP_WEIGHT;
            nodal_forces[i][2] -= (stress[5]*b0 + stress[4]*b1 + stress[2]*b2) * det_j * HEX_GP_WEIGHT;
        }
    }

    (nodal_forces, total_vm / 8.0, total_se, is_failed)
}

/// 运行显式动力学求解器
#[tauri::command]
pub fn run_explicit_solver(config: ExplicitSolverConfig) -> Result<ExplicitSolverResult, String> {
    let num_nodes = config.nodes.len();
    let num_elements = config.elements.len();
    if num_nodes == 0 || num_elements == 0 {
        return Err("No nodes or elements provided".to_string());
    }

    // 初始化状态向量
    let mut displacements: Vec<[f64; 3]> = vec![[0.0; 3]; num_nodes];
    let mut velocities: Vec<[f64; 3]> = vec![[0.0; 3]; num_nodes];
    let mut accelerations: Vec<[f64; 3]> = vec![[0.0; 3]; num_nodes];
    let mut plastic_strains: Vec<f64> = vec![0.0; num_elements];
    let mut failed_elements: std::collections::HashSet<usize> = std::collections::HashSet::new();

    // 设置初始速度
    for &(nid, vel) in &config.boundary_conditions.initial_velocities {
        if nid < num_nodes {
            velocities[nid] = vel;
        }
    }

    // 集中质量矩阵
    let mut lumped_mass: Vec<f64> = config.node_masses.clone();
    if lumped_mass.len() != num_nodes {
        // 如果没有提供节点质量，从单元质量分配
        lumped_mass = vec![0.0; num_nodes];
        let vol_per_elem = if num_elements > 0 {
            // 估算单元体积
            let e0 = &config.elements[0];
            let n0 = config.nodes[e0[0]];
            let n6 = config.nodes[e0[6]];
            let dx = (n6[0] - n0[0]).abs();
            let dy = (n6[1] - n0[1]).abs();
            let dz = (n6[2] - n0[2]).abs();
            dx * dy * dz
        } else {
            1.0
        };
        let elem_mass = config.material.density * vol_per_elem;
        for elem in &config.elements {
            for &nid in elem {
                lumped_mass[nid] += elem_mass / 8.0;
            }
        }
    }

    let dt = config.time_step;
    let end_time = config.end_time;
    let output_interval = config.output_interval.max(1) as u64;
    let damping = config.damping;
    let num_steps = (end_time / dt).ceil() as u64;

    // 构建固定节点集合
    let fixed_set: std::collections::HashSet<usize> = config.boundary_conditions.fixed_nodes.iter().cloned().collect();

    // 输出帧
    let mut frames: Vec<ExplicitFrame> = Vec::new();

    // 计算初始内力
    let mut internal_forces: Vec<[f64; 3]> = vec![[0.0; 3]; num_nodes];
    let mut element_stresses: Vec<f64> = vec![0.0; num_elements];

    // 初始能量
    let initial_ke = compute_kinetic_energy(&velocities, &lumped_mass);

    // 中心差分时间积分
    for step in 0..=num_steps {
        let current_time = step as f64 * dt;

        // 输出帧
        if step % output_interval == 0 {
            let ke = compute_kinetic_energy(&velocities, &lumped_mass);
            let ie = compute_internal_energy(&element_stresses, &plastic_strains, &config);

            let mut max_disp = 0.0;
            let mut max_vel = 0.0;
            for i in 0..num_nodes {
                let d = (displacements[i][0].powi(2) + displacements[i][1].powi(2) + displacements[i][2].powi(2)).sqrt();
                let v = (velocities[i][0].powi(2) + velocities[i][1].powi(2) + velocities[i][2].powi(2)).sqrt();
                if d > max_disp { max_disp = d; }
                if v > max_vel { max_vel = v; }
            }

            frames.push(ExplicitFrame {
                time: current_time,
                kinetic_energy: ke,
                internal_energy: ie,
                total_energy: ke + ie,
                max_displacement: max_disp,
                max_velocity: max_vel,
                node_displacements: displacements.clone(),
                node_velocities: velocities.clone(),
                element_stresses: element_stresses.clone(),
                failed_elements: failed_elements.iter().cloned().collect(),
            });
        }

        if step == num_steps { break; }

        // 重置内力
        for f in internal_forces.iter_mut() { *f = [0.0; 3]; }

        // 计算每个单元的内力
        for (eidx, elem) in config.elements.iter().enumerate() {
            if failed_elements.contains(&eidx) { continue; }

            // 提取单元节点坐标
            let mut elem_nodes: [[f64; 8]; 3] = [[0.0; 8]; 3];
            for i in 0..8 {
                elem_nodes[0][i] = config.nodes[elem[i]][0];
                elem_nodes[1][i] = config.nodes[elem[i]][1];
                elem_nodes[2][i] = config.nodes[elem[i]][2];
            }

            let (nodal_forces, vm, _se, is_failed) = compute_element_internal_forces(
                &elem_nodes,
                &displacements,
                elem,
                &config.material,
                &mut plastic_strains[eidx],
            );

            element_stresses[eidx] = vm;

            if is_failed {
                failed_elements.insert(eidx);
                continue;
            }

            // 组装内力
            for i in 0..8 {
                let nid = elem[i];
                internal_forces[nid][0] += nodal_forces[i][0];
                internal_forces[nid][1] += nodal_forces[i][1];
                internal_forces[nid][2] += nodal_forces[i][2];
            }
        }

        // 接触力计算（罚函数法）
        for cp in &config.contact_pairs {
            for &sid in &cp.slave_nodes {
                let slave_pos = [
                    config.nodes[sid][0] + displacements[sid][0],
                    config.nodes[sid][1] + displacements[sid][1],
                    config.nodes[sid][2] + displacements[sid][2],
                ];
                for &mid in &cp.master_nodes {
                    let master_pos = [
                        config.nodes[mid][0] + displacements[mid][0],
                        config.nodes[mid][1] + displacements[mid][1],
                        config.nodes[mid][2] + displacements[mid][2],
                    ];
                    let dx = slave_pos[0] - master_pos[0];
                    let dy = slave_pos[1] - master_pos[1];
                    let dz = slave_pos[2] - master_pos[2];
                    let dist = (dx*dx + dy*dy + dz*dz).sqrt();

                    // 简化的接触检测阈值
                    let contact_dist = 0.5; // 可配置
                    if dist < contact_dist && dist > 1e-10 {
                        let penetration = contact_dist - dist;
                        let fn_mag = cp.penalty_stiffness * penetration;
                        let nx = dx / dist;
                        let ny = dy / dist;
                        let nz = dz / dist;

                        // 法向力
                        internal_forces[sid][0] += fn_mag * nx;
                        internal_forces[sid][1] += fn_mag * ny;
                        internal_forces[sid][2] += fn_mag * nz;

                        // 摩擦力（简化）
                        let rel_vx = velocities[sid][0] - velocities[mid][0];
                        let rel_vy = velocities[sid][1] - velocities[mid][1];
                        let rel_vz = velocities[sid][2] - velocities[mid][2];
                        let rel_vt = (rel_vx*rel_vx + rel_vy*rel_vy + rel_vz*rel_vz
                            - (rel_vx*nx + rel_vy*ny + rel_vz*nz).powi(2)).sqrt();
                        if rel_vt > 1e-10 {
                            let ft_mag = cp.friction_coefficient * fn_mag;
                            let ftx = -ft_mag * rel_vx / rel_vt;
                            let fty = -ft_mag * rel_vy / rel_vt;
                            let ftz = -ft_mag * rel_vz / rel_vt;
                            internal_forces[sid][0] += ftx;
                            internal_forces[sid][1] += fty;
                            internal_forces[sid][2] += ftz;
                            internal_forces[mid][0] -= ftx;
                            internal_forces[mid][1] -= fty;
                            internal_forces[mid][2] -= ftz;
                        }
                    }
                }
            }
        }

        // 中心差分更新
        for i in 0..num_nodes {
            if fixed_set.contains(&i) { continue; }
            let m = lumped_mass[i];
            if m < 1e-20 { continue; }

            // a = F/m - damping * v
            accelerations[i][0] = internal_forces[i][0] / m - damping * velocities[i][0];
            accelerations[i][1] = internal_forces[i][1] / m - damping * velocities[i][1];
            accelerations[i][2] = internal_forces[i][2] / m - damping * velocities[i][2];

            // v(t+dt/2) = v(t-dt/2) + a * dt
            velocities[i][0] += accelerations[i][0] * dt;
            velocities[i][1] += accelerations[i][1] * dt;
            velocities[i][2] += accelerations[i][2] * dt;

            // u(t+dt) = u(t) + v(t+dt/2) * dt
            displacements[i][0] += velocities[i][0] * dt;
            displacements[i][1] += velocities[i][1] * dt;
            displacements[i][2] += velocities[i][2] * dt;
        }

        // 施加固定边界条件
        for &nid in &config.boundary_conditions.fixed_nodes {
            displacements[nid] = [0.0; 3];
            velocities[nid] = [0.0; 3];
        }

        // 施加速度边界条件
        for &(nid, vel) in &config.boundary_conditions.prescribed_velocities {
            if nid < num_nodes {
                velocities[nid] = vel;
            }
        }
    }

    // 计算能量误差
    let final_ke = compute_kinetic_energy(&velocities, &lumped_mass);
    let final_ie = compute_internal_energy(&element_stresses, &plastic_strains, &config);
    let total_final = final_ke + final_ie;
    let total_initial = initial_ke;
    let energy_error = if total_initial.abs() > 1e-10 {
        (total_final - total_initial).abs() / total_initial * 100.0
    } else if total_final.abs() > 1e-10 {
        (total_final - total_initial).abs() / total_final * 100.0
    } else {
        0.0
    };

    Ok(ExplicitSolverResult {
        frames,
        num_time_steps: num_steps,
        energy_error_percent: energy_error,
        num_failed_elements: failed_elements.len(),
        status: "completed".to_string(),
    })
}

/// 计算动能
fn compute_kinetic_energy(velocities: &[[f64; 3]], masses: &[f64]) -> f64 {
    let mut ke = 0.0;
    for i in 0..velocities.len() {
        let v2 = velocities[i][0].powi(2) + velocities[i][1].powi(2) + velocities[i][2].powi(2);
        ke += 0.5 * masses[i] * v2;
    }
    ke
}

/// 计算内能（近似）
fn compute_internal_energy(stresses: &[f64], plastic_strains: &[f64], config: &ExplicitSolverConfig) -> f64 {
    let mut ie = 0.0;
    for i in 0..stresses.len() {
        // 弹性应变能 ~ sigma^2 / (2E) * volume
        ie += stresses[i].powi(2) / (2.0 * config.material.youngs_modulus);
        // 塑性耗散
        ie += config.material.yield_stress * plastic_strains[i];
    }
    ie
}

/// 生成示例网格用于求解器演示
#[tauri::command]
pub fn generate_explicit_demo_mesh(
    nx: usize,
    ny: usize,
    nz: usize,
    size: f64,
) -> Result<serde_json::Value, String> {
    let dx = size / nx as f64;
    let dy = size / ny as f64;
    let dz = size / nz as f64;

    let mut nodes: Vec<[f64; 3]> = Vec::new();
    let mut elements: Vec<[usize; 8]> = Vec::new();
    let mut fixed_nodes: Vec<usize> = Vec::new();
    let mut initial_velocities: Vec<(usize, [f64; 3])> = Vec::new();

    // 生成节点
    let (nnx, nny, nnz) = (nx + 1, ny + 1, nz + 1);
    for iz in 0..nnz {
        for iy in 0..nny {
            for ix in 0..nnx {
                nodes.push([ix as f64 * dx, iy as f64 * dy, iz as f64 * dz]);
            }
        }
    }

    // 生成HEX8单元
    for iz in 0..nz {
        for iy in 0..ny {
            for ix in 0..nx {
                let n0 = iz * nny * nnx + iy * nnx + ix;
                let n1 = n0 + 1;
                let n2 = n0 + nnx + 1;
                let n3 = n0 + nnx;
                let n4 = n0 + nny * nnx;
                let n5 = n4 + 1;
                let n6 = n4 + nnx + 1;
                let n7 = n4 + nnx;
                elements.push([n0, n1, n2, n3, n4, n5, n6, n7]);
            }
        }
    }

    // 固定底面 (z=0)
    for iy in 0..nny {
        for ix in 0..nnx {
            fixed_nodes.push(iy * nnx + ix);
        }
    }

    // 顶面初始速度 (z=size)
    for iy in 0..nny {
        for ix in 0..nnx {
            let nid = nz * nny * nnx + iy * nnx + ix;
            initial_velocities.push((nid, [0.0, 0.0, -10.0]));
        }
    }

    Ok(serde_json::json!({
        "nodes": nodes,
        "elements": elements,
        "fixed_nodes": fixed_nodes,
        "initial_velocities": initial_velocities,
        "num_nodes": nodes.len(),
        "num_elements": elements.len(),
    }))
}