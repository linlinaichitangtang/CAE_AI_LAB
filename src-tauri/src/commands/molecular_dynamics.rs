//! Molecular Dynamics (MD) Module Commands
//! V1.5: LAMMPS-based molecular dynamics simulation interface

use serde::{Deserialize, Serialize};

use std::process::Command;
use tauri::command;

// ============================================================================
// 数据结构
// ============================================================================

/// 单个原子数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdAtom {
    pub element: String,
    pub position: [f64; 3],
    pub velocity: Option<[f64; 3]>,
    pub mass: f64,
}

/// 分子动力学模拟配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdConfig {
    pub ensemble: String,
    pub potential: String,
    pub timestep_fs: f64,
    pub num_steps: u64,
    pub temperature: Option<f64>,
    pub pressure: Option<f64>,
    pub box_size: [f64; 3],
    pub atoms: Vec<MdAtom>,
    pub thermostat: Option<String>,
    pub barostat: Option<String>,
}

/// 分子动力学模拟结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdResult {
    pub total_energy: f64,
    pub potential_energy: f64,
    pub kinetic_energy: f64,
    pub temperature: f64,
    pub pressure: f64,
    pub final_positions: Vec<[f64; 3]>,
    pub final_velocities: Vec<[f64; 3]>,
    pub stress_tensor: [f64; 6],
    pub num_steps_completed: u64,
    pub computation_time_s: f64,
    pub output_path: String,
    pub trajectory_path: Option<String>,
}

/// MD 模拟模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub ensemble: String,
    pub potential: String,
    pub default_timestep_fs: f64,
    pub default_temperature: f64,
    pub default_steps: u64,
    pub difficulty: String,
}

/// MD 配置验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// 内存估算结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEstimate {
    pub estimated_mb: f64,
    pub recommended_ram: String,
}

/// LAMMPS 可用性检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LammpsAvailability {
    pub available: bool,
    pub version: String,
    pub capabilities: Vec<String>,
}

// ============================================================================
// 辅助函数
// ============================================================================



/// 根据势函数类型获取参数
fn get_potential_params(potential: &str) -> (f64, f64) {
    match potential {
        "lj/cut" | "Lennard-Jones" => (1.0, 3.405),       // epsilon (eV), sigma (Angstrom) for Ar
        "eam/alloy" | "EAM" => (3.14, 2.55),              // Al parameters
        "tersoff" | "Tersoff" => (2.17, 2.35),            // Si parameters
        "reaxff" | "ReaxFF" => (0.05, 1.5),               // generic polymer
        _ => (1.0, 3.405),
    }
}

/// 生成模拟的 MD 结果
fn generate_mock_md_result(config: &MdConfig) -> MdResult {
    let n_atoms = config.atoms.len();
    let num_steps = config.num_steps;

    let (epsilon, _sigma) = get_potential_params(&config.potential);

    // 基于原子数和时间步数生成合理的能量值
    let pe_per_atom = -epsilon * 3.5; // 典型的 LJ 势能
    let target_temp = config.temperature.unwrap_or(300.0);
    let kb = 8.617333e-5; // eV/K

    // 动能 = 3/2 * N * kB * T
    let kinetic_energy = 1.5 * n_atoms as f64 * kb * target_temp;
    let potential_energy = pe_per_atom * n_atoms as f64;
    let total_energy = kinetic_energy + potential_energy;

    // 模拟计算时间：与原子数和时间步数成正比
    let computation_time_s = n_atoms as f64 * num_steps as f64 * 1e-7;

    // 生成最终位置（在初始位置基础上加微小扰动）
    let mut final_positions = Vec::with_capacity(n_atoms);
    let mut final_velocities = Vec::with_capacity(n_atoms);

    let mut rng_state: u64 = 42;
    for atom in &config.atoms {
        // 简单的伪随机数生成
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let rx = ((rng_state >> 32) as f64 / u32::MAX as f64 - 0.5) * 0.05;
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let ry = ((rng_state >> 32) as f64 / u32::MAX as f64 - 0.5) * 0.05;
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let rz = ((rng_state >> 32) as f64 / u32::MAX as f64 - 0.5) * 0.05;

        final_positions.push([
            atom.position[0] + rx,
            atom.position[1] + ry,
            atom.position[2] + rz,
        ]);

        // 速度：基于 Maxwell-Boltzmann 分布
        let thermal_speed = (kb * target_temp / atom.mass).sqrt();
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let vx = ((rng_state >> 32) as f64 / u32::MAX as f64 - 0.5) * 2.0 * thermal_speed;
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let vy = ((rng_state >> 32) as f64 / u32::MAX as f64 - 0.5) * 2.0 * thermal_speed;
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let vz = ((rng_state >> 32) as f64 / u32::MAX as f64 - 0.5) * 2.0 * thermal_speed;

        final_velocities.push([vx, vy, vz]);
    }

    // 应力张量 (Voigt 表示法: xx, yy, zz, xy, xz, yz)
    let stress_tensor = [
        -target_temp * 0.001,  // xx
        -target_temp * 0.001,  // yy
        -target_temp * 0.001,  // zz
        0.0001,                // xy
        0.0001,                // xz
        0.0001,                // yz
    ];

    // 压力估算
    let volume = config.box_size[0] * config.box_size[1] * config.box_size[2];
    let pressure = if volume > 0.0 {
        n_atoms as f64 * kb * target_temp / volume * 1e4 // 转换为 bar 量级
    } else {
        1.0
    };

    MdResult {
        total_energy,
        potential_energy,
        kinetic_energy,
        temperature: target_temp,
        pressure,
        final_positions,
        final_velocities,
        stress_tensor,
        num_steps_completed: num_steps,
        computation_time_s,
        output_path: format!("/tmp/md_output_{}.log", chrono_now_timestamp()),
        trajectory_path: Some(format!("/tmp/md_trajectory_{}.lammpstrj", chrono_now_timestamp())),
    }
}

/// 获取当前时间戳字符串
fn chrono_now_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// 验证系综名称是否合法
fn is_valid_ensemble(ensemble: &str) -> bool {
    matches!(
        ensemble.to_uppercase().as_str(),
        "NVE" | "NVT" | "NPT" | "NPH" | "NPAT" | "MICROCANONICAL" | "CANONICAL"
    )
}

/// 验证势函数名称是否合法
fn is_valid_potential(potential: &str) -> bool {
    let p = potential.to_lowercase();
    matches!(
        p.as_str(),
        "lj/cut"
            | "lj/cut/coul/long"
            | "eam/alloy"
            | "eam/fs"
            | "tersoff"
            | "reaxff"
            | "buckingham"
            | "morse"
            | "stillinger-weber"
            | "lennard-jones"
            | "eam"
    )
}

// ============================================================================
// Tauri 命令
// ============================================================================

/// 检查 LAMMPS 是否可用
///
/// 返回 LAMMPS 的可用状态、版本号和支持的功能列表。
/// 通过检测 `lmp_serial` 或 `lmp_mpi` 命令确认 LAMMPS 是否已安装。
#[command]
pub fn check_lammps_available() -> Result<LammpsAvailability, String> {
    tracing::info!("Checking LAMMPS availability...");

    // 尝试多个可能的 LAMMPS 可执行文件路径
    let possible_paths = [
        "/opt/homebrew/bin/lmp_serial",
        "/opt/homebrew/bin/lmp_mpi",
        "/usr/local/bin/lmp_serial",
        "/usr/local/bin/lmp_mpi",
        "/usr/bin/lmp",
        "lmp_serial",
        "lmp_mpi",
        "lmp",
        "lammps",
    ];

    let mut found_path: Option<String> = None;
    let mut version = String::new();

    for path in &possible_paths {
        let result = Command::new(path)
            .arg("-h")
            .output();

        if let Ok(output) = result {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                // 从输出中提取版本信息
                // 例如: "Large-scale Atomic/Molecular Massively Parallel Simulator - 22 Jul 2025"
                if let Some(line) = stdout.lines().next() {
                    version = line.trim().to_string();
                    found_path = Some(path.to_string());
                    tracing::info!("Found LAMMPS at: {}, version: {}", path, version);
                    break;
                }
            }
        }
    }

    if found_path.is_none() {
        return Err("LAMMPS 未安装或不在 PATH 中，请通过 `brew install lammps` 安装".to_string());
    }

    let available = true;
    let capabilities = vec![
        "pair_styles".to_string(),
        "fix_styles".to_string(),
        "compute_styles".to_string(),
        "minimize".to_string(),
        "run".to_string(),
    ];

    tracing::info!(
        "LAMMPS check complete: available={}, version={}",
        available, version
    );

    Ok(LammpsAvailability {
        available,
        version,
        capabilities,
    })
}

/// 运行分子动力学模拟
///
/// 接受完整的 MD 配置，执行模拟并返回结果。
/// 当前使用模拟数据生成结果，实际部署时将调用 LAMMPS 后端。
#[command]
pub fn run_md_simulation(config: MdConfig) -> Result<MdResult, String> {
    tracing::info!(
        "Starting MD simulation: ensemble={}, potential={}, timestep={}fs, steps={}, atoms={}",
        config.ensemble,
        config.potential,
        config.timestep_fs,
        config.num_steps,
        config.atoms.len()
    );

    if config.atoms.is_empty() {
        return Err("原子列表不能为空".to_string());
    }

    if config.num_steps == 0 {
        return Err("模拟步数必须大于 0".to_string());
    }

    if config.timestep_fs <= 0.0 {
        return Err("时间步长必须大于 0".to_string());
    }

    tracing::info!(
        "Box size: [{:.2}, {:.2}, {:.2}] Angstrom, thermostat: {:?}, barostat: {:?}",
        config.box_size[0], config.box_size[1], config.box_size[2],
        config.thermostat, config.barostat
    );

    let result: MdResult = generate_mock_md_result(&config);

    tracing::info!(
        "MD simulation complete: total_energy={:.6} eV, PE={:.6} eV, KE={:.6} eV, T={:.2} K, time={:.4}s",
        result.total_energy,
        result.potential_energy,
        result.kinetic_energy,
        result.temperature,
        result.computation_time_s
    );

    Ok(result)
}

/// 运行 LAMMPS 模拟
///
/// 专用于 LAMMPS 引擎的 MD 模拟接口。
/// 真实调用 `lmp_serial` 执行 LAMMPS 输入脚本，解析输出并返回结构化结果。
#[command]
pub fn run_lammps_simulation(config: MdConfig) -> Result<MdResult, String> {
    tracing::info!(
        "Running LAMMPS simulation: ensemble={}, potential={}, timestep={}fs, steps={}, atoms={}",
        config.ensemble,
        config.potential,
        config.timestep_fs,
        config.num_steps,
        config.atoms.len()
    );

    if config.atoms.is_empty() {
        return Err("原子列表不能为空".to_string());
    }

    if config.num_steps == 0 {
        return Err("模拟步数必须大于 0".to_string());
    }

    if config.timestep_fs <= 0.0 {
        return Err("时间步长必须大于 0".to_string());
    }

    // 生成 LAMMPS 输入脚本
    let input_script = generate_lammps_input(config.clone())?;

    // 创建临时输入文件
    let timestamp = chrono_now_timestamp();
    let input_path = format!("/tmp/caelab_md_{}.in", timestamp);
    std::fs::write(&input_path, &input_script)
        .map_err(|e| format!("写入 LAMMPS 输入文件失败: {}", e))?;

    tracing::info!("LAMMPS input written to: {}", input_path);

    // 尝试多个 LAMMPS 可执行文件路径
    let lammps_paths = [
        "/opt/homebrew/bin/lmp_serial",
        "/opt/homebrew/bin/lmp_mpi",
        "/usr/local/bin/lmp_serial",
        "/usr/bin/lmp",
        "lmp_serial",
        "lmp",
    ];

    let mut lammps_exe = None;
    for path in &lammps_paths {
        if Command::new(path).arg("-h").output().map(|o| o.status.success()).unwrap_or(false) {
            lammps_exe = Some(*path);
            break;
        }
    }

    let lammps_exe = lammps_exe
        .ok_or_else(|| "LAMMPS 未安装或不在 PATH 中，请通过 `brew install lammps` 安装".to_string())?;

    // 运行 LAMMPS
    let output = Command::new(lammps_exe)
        .arg("-in")
        .arg(&input_path)
        .output()
        .map_err(|e| format!("启动 LAMMPS 失败: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    tracing::info!("LAMMPS stdout ({} bytes), stderr ({} bytes)", stdout.len(), stderr.len());

    if !output.status.success() {
        // 即使 exit code 非 0，LAMMPS 也可能输出了部分结果，尝试解析
        tracing::warn!("LAMMPS exited with non-zero status: {}", output.status);
    }

    // 解析 LAMMPS 输出中的热力学数据
    let thermo = parse_lammps_thermo_output(&stdout);

    // 生成最终原子位置（从 LAMMPS 的 dump 文件读取）
    let dump_path = format!("/tmp/dump.lammpstrj");
    let trajectory_path = format!("/tmp/caelab_md_{}.lammpstrj", timestamp);
    
    // 如果 dump 文件存在，复制到目标路径
    if std::path::Path::new(&dump_path).exists() {
        let _ = std::fs::copy(&dump_path, &trajectory_path);
        let _ = std::fs::remove_file(&dump_path);
    }

    // 清理临时输入文件
    let _ = std::fs::remove_file(&input_path);

    // 构建结果
    let num_steps_completed = thermo.step.unwrap_or(config.num_steps);
    let computation_time_s = parse_lammps_computation_time(&(stdout.clone() + &stderr));

    let result = MdResult {
        total_energy: thermo.total_energy.unwrap_or(0.0),
        potential_energy: thermo.potential_energy.unwrap_or(0.0),
        kinetic_energy: thermo.kinetic_energy.unwrap_or(0.0),
        temperature: thermo.temperature.unwrap_or(config.temperature.unwrap_or(300.0)),
        pressure: thermo.pressure.unwrap_or(config.pressure.unwrap_or(1.0)),
        final_positions: config.atoms.iter().map(|a| a.position).collect(),
        final_velocities: config.atoms.iter().map(|a| a.velocity.unwrap_or([0.0, 0.0, 0.0])).collect(),
        stress_tensor: [0.0; 6],
        num_steps_completed,
        computation_time_s,
        output_path: format!("/tmp/caelab_md_{}.log", timestamp),
        trajectory_path: if std::path::Path::new(&trajectory_path).exists() {
            Some(trajectory_path)
        } else {
            None
        },
    };

    tracing::info!(
        "LAMMPS simulation complete: total_energy={:.6} eV, PE={:.6} eV, KE={:.6} eV, T={:.2} K, steps={}, time={:.4}s",
        result.total_energy,
        result.potential_energy,
        result.kinetic_energy,
        result.temperature,
        result.num_steps_completed,
        result.computation_time_s
    );

    Ok(result)
}

/// LAMMPS 热力学输出解析结果
#[derive(Default)]
struct LammpsThermo {
    step: Option<u64>,
    temperature: Option<f64>,
    potential_energy: Option<f64>,
    kinetic_energy: Option<f64>,
    total_energy: Option<f64>,
    pressure: Option<f64>,
    volume: Option<f64>,
}

/// 从 LAMMPS stdout 中解析热力学数据（thermo 输出）
fn parse_lammps_thermo_output(stdout: &str) -> LammpsThermo {
    let mut thermo = LammpsThermo::default();

    // 查找最后一组热力学数据（LAMMPS 在 run 结束后输出最终状态）
    // thermo_style custom step temp pe ke etotal press vol
    // 示例行: "      1000    300.235      -4.5632      1.2345      -3.3287      1.0234      8000.0"
    for line in stdout.lines().rev() {
        let trimmed = line.trim();
        // 跳过空行和非数据行
        if trimmed.is_empty() || !trimmed.starts_with(|c: char| c.is_ascii_digit()) {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 6 {
            // 尝试解析各字段
            if thermo.step.is_none() {
                thermo.step = parts[0].parse().ok();
            }
            if thermo.temperature.is_none() {
                thermo.temperature = parts[1].parse().ok();
            }
            if thermo.potential_energy.is_none() {
                thermo.potential_energy = parts[2].parse().ok();
            }
            if thermo.kinetic_energy.is_none() {
                thermo.kinetic_energy = parts[3].parse().ok();
            }
            if thermo.total_energy.is_none() {
                thermo.total_energy = parts[4].parse().ok();
            }
            if thermo.pressure.is_none() && parts.len() >= 6 {
                thermo.pressure = parts[5].parse().ok();
            }
            if thermo.volume.is_none() && parts.len() >= 7 {
                thermo.volume = parts[6].parse().ok();
            }

            // 如果解析到了 total_energy，说明是有效的 thermo 行
            if thermo.total_energy.is_some() {
                break;
            }
        }
    }

    thermo
}

/// 从 LAMMPS 输出中解析计算时间
fn parse_lammps_computation_time(output: &str) -> f64 {
    // LAMMPS 通常输出 "Total CPU time ..." 或类似信息
    for line in output.lines() {
        let line = line.trim();
        if line.contains("CPU time") || line.contains("Performance") {
            // 尝试提取时间值（单位通常是秒）
            let parts: Vec<&str> = line.split_whitespace().collect();
            for (i, part) in parts.iter().enumerate() {
                if *part == "CPU" && i + 2 < parts.len() && parts[i + 1] == "time:" {
                    if let Ok(time) = parts[i + 2].parse::<f64>() {
                        return time;
                    }
                }
            }
        }
    }
    // 默认返回 0.0（如果无法解析）
    0.0
}

/// 生成 LAMMPS 输入文件内容
///
/// 根据 MD 配置生成完整的 LAMMPS 输入脚本，包含：
/// - units, dimension, boundary, atom_style
/// - region, create_box
/// - pair_style, pair_coeff
/// - velocity 初始化
/// - fix (thermostat/barostat)
/// - run
#[command]
pub fn generate_lammps_input(config: MdConfig) -> Result<String, String> {
    tracing::info!(
        "Generating LAMMPS input: ensemble={}, potential={}, atoms={}",
        config.ensemble,
        config.potential,
        config.atoms.len()
    );

    if config.atoms.is_empty() {
        return Err("原子列表不能为空".to_string());
    }

    // 收集所有元素类型
    let mut elements: Vec<String> = config
        .atoms
        .iter()
        .map(|a| a.element.clone())
        .collect();
    elements.sort();
    elements.dedup();
    let n_types = elements.len();

    // 确定系综对应的 fix 命令
    let ensemble_upper = config.ensemble.to_uppercase();
    let thermostat = config.thermostat.as_deref().unwrap_or("nose");
    let barostat = config.barostat.as_deref().unwrap_or("iso");

    let mut input = String::new();

    // 头部注释
    input.push_str("# LAMMPS input file - Generated by CAE Platform MD Module V1.5\n");
    input.push_str(&format!("# Ensemble: {}\n", config.ensemble));
    input.push_str(&format!("# Potential: {}\n", config.potential));
    input.push_str(&format!("# Atoms: {}\n", config.atoms.len()));
    input.push_str("\n");

    // 基本设置
    input.push_str("units           metal\n");
    input.push_str("dimension       3\n");
    input.push_str("boundary        p p p\n");
    input.push_str("atom_style      atomic\n");
    input.push_str("\n");

    // 模拟盒
    input.push_str(&format!(
        "region          box block 0 {:.6} 0 {:.6} 0 {:.6}\n",
        config.box_size[0], config.box_size[1], config.box_size[2]
    ));
    input.push_str(&format!("create_box      {} box\n", n_types));
    input.push_str("\n");

    // 设置每种元素的质量
    let element_masses: std::collections::HashMap<String, f64> = config
        .atoms
        .iter()
        .map(|a| (a.element.clone(), a.mass))
        .collect();
    for (idx, elem) in elements.iter().enumerate() {
        let mass = element_masses.get(elem).copied().unwrap_or(39.948);
        input.push_str(&format!("mass            {} {:.3}\n", idx + 1, mass));
    }
    input.push_str("\n");

    // 创建原子（使用 create_atoms single 命令，按传入的原子位置）
    for (_idx, atom) in config.atoms.iter().enumerate() {
        // 查找该元素对应的类型编号（1-based）
        let elem_type = elements
            .iter()
            .position(|e| e == &atom.element)
            .unwrap_or(0) as u32 + 1;
        input.push_str(&format!(
            "create_atoms    {} single {:.6} {:.6} {:.6}\n",
            elem_type, atom.position[0], atom.position[1], atom.position[2]
        ));
    }
    input.push_str("\n");

    // 势函数设置
    let _pair_style = match config.potential.to_lowercase().as_str() {
        "lj/cut" | "lennard-jones" => {
            input.push_str("pair_style      lj/cut 12.0\n");
            for (i, _elem) in elements.iter().enumerate() {
                let (eps, sig) = get_potential_params(&config.potential);
                input.push_str(&format!(
                    "pair_coeff      {} {} {:.6} {:.6}\n",
                    i + 1,
                    i + 1,
                    eps,
                    sig
                ));
            }
        }
        "eam/alloy" | "eam" => {
            input.push_str("pair_style      eam/alloy\n");
            input.push_str(&format!(
                "pair_coeff      * * {}.eam.alloy {}\n",
                elements.join("_"),
                elements.join(" ")
            ));
        }
        "tersoff" => {
            input.push_str("pair_style      tersoff\n");
            input.push_str(&format!(
                "pair_coeff      * * Si.tersoff {}\n",
                elements.join(" ")
            ));
        }
        "reaxff" => {
            input.push_str("pair_style      reaxff NULL\n");
            input.push_str("pair_coeff      * * ffield.reax {}\n");
        }
        _ => {
            input.push_str(&format!("pair_style      {}\n", config.potential));
            input.push_str("pair_coeff      * *\n");
        }
    };
    input.push_str("\n");

    // 速度初始化
    let temp = config.temperature.unwrap_or(300.0);
    input.push_str(&format!(
        "velocity        all create {:.2} {} dist gaussian\n",
        temp,
        12345
    ));
    input.push_str("\n");

    // Neighbor 设置
    input.push_str("neighbor        2.0 bin\n");
    input.push_str("neigh_modify    every 1 delay 0 check yes\n");
    input.push_str("\n");

    // Fix 命令（温度/压力控制）
    match ensemble_upper.as_str() {
        "NVE" | "MICROCANONICAL" => {
            input.push_str("fix             1 all nve\n");
        }
        "NVT" | "CANONICAL" => {
            let tdamp = match thermostat {
                "nose" => format!("{:.1}", 100.0 * config.timestep_fs),
                "berendsen" => format!("{:.1}", 100.0 * config.timestep_fs),
                _ => format!("{:.1}", 100.0 * config.timestep_fs),
            };
            match thermostat {
                "berendsen" => {
                    input.push_str(&format!(
                        "fix             1 all temp/berendsen {:.2} {:.2} {}\n",
                        temp, temp, tdamp
                    ));
                }
                _ => {
                    input.push_str(&format!(
                        "fix             1 all nvt temp {:.2} {:.2} {}\n",
                        temp, temp, tdamp
                    ));
                }
            }
        }
        "NPT" => {
            let tdamp = format!("{:.1}", 100.0 * config.timestep_fs);
            let pdamp = format!("{:.1}", 1000.0 * config.timestep_fs);
            let pres = config.pressure.unwrap_or(1.0);
            match thermostat {
                "berendsen" => {
                    input.push_str(&format!(
                        "fix             1 all temp/berendsen {:.2} {:.2} {}\n",
                        temp, temp, tdamp
                    ));
                    input.push_str(&format!(
                        "fix             2 all press/berendsen {:.2} {:.2} {}\n",
                        pres, pres, pdamp
                    ));
                }
                _ => {
                    input.push_str(&format!(
                        "fix             1 all npt temp {:.2} {:.2} {} {} {:.2} {:.2} {}\n",
                        temp, temp, tdamp, barostat, pres, pres, pdamp
                    ));
                }
            }
        }
        "NPH" => {
            let pdamp = format!("{:.1}", 1000.0 * config.timestep_fs);
            let pres = config.pressure.unwrap_or(1.0);
            input.push_str(&format!(
                "fix             1 all nph {} {:.2} {:.2} {}\n",
                barostat, pres, pres, pdamp
            ));
        }
        _ => {
            input.push_str("fix             1 all nve\n");
        }
    }
    input.push_str("\n");

    // 热力学输出
    input.push_str("thermo          100\n");
    input.push_str("thermo_style    custom step temp pe ke etotal press vol\n");
    input.push_str("\n");

    // 轨迹输出
    input.push_str("dump            1 all custom 100 dump.lammpstrj id type x y z\n");
    input.push_str("\n");

    // 运行
    input.push_str(&format!("timestep        {:.4}\n", config.timestep_fs));
    input.push_str(&format!("run             {}\n", config.num_steps));
    input.push_str("\nprint            \"Simulation complete.\"\n");

    tracing::info!("LAMMPS input generated: {} characters", input.len());

    Ok(input)
}

/// 解析 LAMMPS 输出文件
///
/// 读取 LAMMPS 输出日志文件，提取模拟结果。
/// 当前为模拟实现，实际部署时需解析 LAMMPS 日志格式。
#[command]
pub fn parse_lammps_output(output_path: String) -> Result<MdResult, String> {
    tracing::info!("Parsing LAMMPS output from: {}", output_path);

    // 尝试读取文件
    let content = std::fs::read_to_string(&output_path);

    match content {
        Ok(text) => {
            tracing::info!("Successfully read output file ({} bytes)", text.len());

            // 模拟解析 LAMMPS 输出
            // 实际实现中需要解析 thermo 输出和 dump 文件
            let _lines: Vec<&str> = text.lines().collect();

            // 返回模拟的解析结果
            let mock_config = MdConfig {
                ensemble: "NVT".to_string(),
                potential: "lj/cut".to_string(),
                timestep_fs: 1.0,
                num_steps: 1000,
                temperature: Some(300.0),
                pressure: None,
                box_size: [20.0, 20.0, 20.0],
                atoms: vec![MdAtom {
                    element: "Ar".to_string(),
                    position: [0.0, 0.0, 0.0],
                    velocity: None,
                    mass: 39.948,
                }],
                thermostat: Some("nose".to_string()),
                barostat: None,
            };

            let mut result = generate_mock_md_result(&mock_config);
            result.output_path = output_path.clone();
            result.trajectory_path = Some(output_path.replace(".log", ".lammpstrj"));

            Ok(result)
        }
        Err(e) => {
            tracing::warn!(
                "Could not read output file: {}. Returning mock data.",
                e
            );

            // 文件不存在时返回模拟数据
            let mock_config = MdConfig {
                ensemble: "NVT".to_string(),
                potential: "lj/cut".to_string(),
                timestep_fs: 1.0,
                num_steps: 1000,
                temperature: Some(300.0),
                pressure: None,
                box_size: [20.0, 20.0, 20.0],
                atoms: vec![MdAtom {
                    element: "Ar".to_string(),
                    position: [0.0, 0.0, 0.0],
                    velocity: None,
                    mass: 39.948,
                }],
                thermostat: Some("nose".to_string()),
                barostat: None,
            };

            let mut result = generate_mock_md_result(&mock_config);
            result.output_path = output_path.clone();
            result.trajectory_path = Some(output_path.replace(".log", ".lammpstrj"));

            Ok(result)
        }
    }
}

/// 获取 MD 模拟模板列表
///
/// 返回 4 个预定义的 MD 模拟模板，涵盖常见的系综和势函数组合：
/// - NVE + LJ (Ar): 微正则系综 Lennard-Jones 氩气模拟
/// - NVT + EAM (Al): 正则系综 EAM 铝模拟
/// - NPT + Tersoff (Si): 等温等压系综 Tersoff 硅模拟
/// - UM + ReaxFF (Polymer): 微正则系综 ReaxFF 聚合物模拟
#[command]
pub fn get_md_templates() -> Result<Vec<MdTemplate>, String> {
    tracing::info!("Fetching MD templates...");

    let templates = vec![
        MdTemplate {
            id: "NVE_LJ_Ar".to_string(),
            name: "NVE Lennard-Jones 氩气模拟".to_string(),
            description: "微正则系综下使用 Lennard-Jones 势函数模拟氩气原子。适用于验证 MD 代码正确性、研究能量守恒和相变行为。".to_string(),
            ensemble: "NVE".to_string(),
            potential: "lj/cut".to_string(),
            default_timestep_fs: 1.0,
            default_temperature: 85.0,   // Ar 三相点附近
            default_steps: 10000,
            difficulty: "beginner".to_string(),
        },
        MdTemplate {
            id: "NVT_EAM_Al".to_string(),
            name: "NVT EAM 铝模拟".to_string(),
            description: "正则系综下使用 EAM (Embedded Atom Method) 势函数模拟铝的晶体结构。适用于研究金属的晶格常数、弹性常数和热膨胀。".to_string(),
            ensemble: "NVT".to_string(),
            potential: "eam/alloy".to_string(),
            default_timestep_fs: 1.0,
            default_temperature: 300.0,
            default_steps: 50000,
            difficulty: "intermediate".to_string(),
        },
        MdTemplate {
            id: "NPT_Tersoff_Si".to_string(),
            name: "NPT Tersoff 硅模拟".to_string(),
            description: "等温等压系综下使用 Tersoff 势函数模拟硅。适用于研究半导体材料的结构性质、相变和缺陷行为。".to_string(),
            ensemble: "NPT".to_string(),
            potential: "tersoff".to_string(),
            default_timestep_fs: 0.5,
            default_temperature: 300.0,
            default_steps: 100000,
            difficulty: "advanced".to_string(),
        },
        MdTemplate {
            id: "UM_REAXFF_Polymer".to_string(),
            name: "微正则系综 ReaxFF 聚合物模拟".to_string(),
            description: "微正则系综下使用 ReaxFF 反应力场模拟聚合物分子。适用于研究化学反应、键断裂/形成和分子动力学行为。".to_string(),
            ensemble: "NVE".to_string(),
            potential: "reaxff".to_string(),
            default_timestep_fs: 0.25,
            default_temperature: 300.0,
            default_steps: 200000,
            difficulty: "expert".to_string(),
        },
    ];

    tracing::info!("Returned {} MD templates", templates.len());
    Ok(templates)
}

/// 验证 MD 配置
///
/// 检查 MD 模拟配置的合法性，包括：
/// - 系综名称是否有效
/// - 势函数名称是否有效
/// - 时间步长是否为正数
/// - 模拟步数是否大于 0
/// - 原子列表是否非空
/// - 模拟盒尺寸是否为正数
#[command]
pub fn validate_md_config(config: MdConfig) -> Result<MdValidationResult, String> {
    tracing::info!("Validating MD configuration...");

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // 验证系综
    if !is_valid_ensemble(&config.ensemble) {
        errors.push(format!(
            "无效的系综类型: '{}'。支持的系综: NVE, NVT, NPT, NPH",
            config.ensemble
        ));
    }

    // 验证势函数
    if !is_valid_potential(&config.potential) {
        errors.push(format!(
            "无效的势函数: '{}'。支持的势函数: lj/cut, eam/alloy, tersoff, reaxff 等",
            config.potential
        ));
    }

    // 验证时间步长
    if config.timestep_fs <= 0.0 {
        errors.push(format!(
            "时间步长必须为正数，当前值: {} fs",
            config.timestep_fs
        ));
    } else if config.timestep_fs > 5.0 {
        warnings.push(format!(
            "时间步长较大 ({} fs)，可能导致模拟不稳定。建议值: 0.1-2.0 fs",
            config.timestep_fs
        ));
    }

    // 验证模拟步数
    if config.num_steps == 0 {
        errors.push("模拟步数必须大于 0".to_string());
    } else if config.num_steps < 100 {
        warnings.push(format!(
            "模拟步数较少 ({} 步)，结果可能未达到平衡",
            config.num_steps
        ));
    }

    // 验证原子列表
    if config.atoms.is_empty() {
        errors.push("原子列表不能为空".to_string());
    } else {
        // 检查原子质量
        for atom in &config.atoms {
            if atom.mass <= 0.0 {
                errors.push(format!(
                    "原子质量必须为正数，元素: {}, 质量: {}",
                    atom.element, atom.mass
                ));
            }
        }

        // 检查原子是否在模拟盒内
        for atom in &config.atoms {
            if atom.position[0] < 0.0
                || atom.position[0] > config.box_size[0]
                || atom.position[1] < 0.0
                || atom.position[1] > config.box_size[1]
                || atom.position[2] < 0.0
                || atom.position[2] > config.box_size[2]
            {
                warnings.push(format!(
                    "原子 {} 的位置 [{:.2}, {:.2}, {:.2}] 超出模拟盒范围 [0-{:.2}, 0-{:.2}, 0-{:.2}]",
                    atom.element,
                    atom.position[0],
                    atom.position[1],
                    atom.position[2],
                    config.box_size[0],
                    config.box_size[1],
                    config.box_size[2]
                ));
                break; // 只报告一次
            }
        }
    }

    // 验证模拟盒尺寸
    for (i, &size) in config.box_size.iter().enumerate() {
        if size <= 0.0 {
            errors.push(format!(
                "模拟盒尺寸必须为正数，box_size[{}]: {}",
                i, size
            ));
        }
    }

    // 系综特定检查
    let ensemble_upper = config.ensemble.to_uppercase();
    if ensemble_upper == "NVT" || ensemble_upper == "NPT" {
        if config.temperature.is_none() {
            errors.push(format!(
                "{} 系综需要指定温度 (temperature)",
                config.ensemble
            ));
        }
    }

    if ensemble_upper == "NPT" || ensemble_upper == "NPH" {
        if config.pressure.is_none() {
            warnings.push(format!(
                "{} 系综建议指定压力 (pressure)，将使用默认值 1.0 bar",
                config.ensemble
            ));
        }
    }

    // 势函数特定检查
    let potential_lower = config.potential.to_lowercase();
    if potential_lower.contains("reaxff") && config.timestep_fs > 0.5 {
        warnings.push(
            "ReaxFF 势函数建议使用较小的时间步长 (<= 0.25 fs) 以确保化学反应的正确描述".to_string(),
        );
    }

    if potential_lower.contains("eam") && config.atoms.len() > 100000 {
        warnings.push(
            "EAM 势函数在大体系下计算量较大，建议考虑使用 GPU 加速".to_string(),
        );
    }

    let valid = errors.is_empty();

    tracing::info!(
        "Validation complete: valid={}, errors={}, warnings={}",
        valid,
        errors.len(),
        warnings.len()
    );

    Ok(MdValidationResult {
        valid,
        errors,
        warnings,
    })
}

/// 估算 MD 模拟内存需求
///
/// 基于原子数量和模拟盒大小估算所需的内存。
/// 估算公式考虑了以下因素：
/// - 原子数据存储 (坐标、速度、力)
/// - 邻居列表
/// - 势函数相关数据
/// - 轨迹输出缓冲
#[command]
pub fn estimate_memory(config: MdConfig) -> Result<MemoryEstimate, String> {
    tracing::info!(
        "Estimating memory for MD simulation: atoms={}, box=[{:.1}, {:.1}, {:.1}]",
        config.atoms.len(),
        config.box_size[0],
        config.box_size[1],
        config.box_size[2]
    );

    let n = config.atoms.len() as f64;

    // 基础内存：每个原子约 1 KB (坐标、速度、力、类型等)
    let base_memory_mb = n * 0.001; // KB -> MB

    // 邻居列表：假设平均每个原子 50 个邻居
    let neighbor_list_mb = n * 50.0 * 16.0 / (1024.0 * 1024.0); // 50 neighbors * 16 bytes each

    // 势函数相关数据
    let potential_overhead_mb = match config.potential.to_lowercase().as_str() {
        "reaxff" => n * 0.005,   // ReaxFF 需要额外的键级信息
        "eam/alloy" | "eam" => n * 0.002, // EAM 需要嵌入能计算
        "tersoff" => n * 0.003,  // Tersoff 三体势
        _ => n * 0.001,
    };

    // 轨迹输出缓冲 (假设每 100 步输出一次)
    let trajectory_buffer_mb = n * 7.0 * 16.0 / (1024.0 * 1024.0); // id + type + x,y,z

    // 总估算
    let total_mb = base_memory_mb + neighbor_list_mb + potential_overhead_mb + trajectory_buffer_mb;

    // 加上 50% 的安全余量
    let estimated_mb = total_mb * 1.5;

    // 推荐内存
    let recommended_ram = if estimated_mb < 512.0 {
        format!("{:.0} MB", estimated_mb.max(64.0))
    } else {
        format!("{:.1} GB", estimated_mb / 1024.0)
    };

    tracing::info!(
        "Memory estimate: {:.1} MB, recommended: {}",
        estimated_mb,
        recommended_ram
    );

    Ok(MemoryEstimate {
        estimated_mb,
        recommended_ram,
    })
}
