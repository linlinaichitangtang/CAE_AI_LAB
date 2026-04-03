use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::Instant;

// ============================================================================
// 数据结构
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverStatus {
    pub name: String,
    pub display_name: String,
    pub scale: String,
    pub installed: bool,
    pub version: Option<String>,
    pub path: Option<String>,
    pub install_method: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverVerifyResult {
    pub works: bool,
    pub version: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallResult {
    pub success: bool,
    pub message: String,
    pub duration_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UninstallResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallMethod {
    pub name: String,
    pub display_name: String,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverInfo {
    pub name: String,
    pub display_name: String,
    pub scale: String,
    pub description: String,
    pub version: Option<String>,
    pub path: Option<String>,
    pub installed: bool,
    pub install_methods: Vec<String>,
    pub estimated_size_mb: f64,
    pub website: String,
}

// ============================================================================
// 内部辅助结构
// ============================================================================

struct SolverDef {
    name: &'static str,
    display_name: &'static str,
    scale: &'static str,
    detect_commands: &'static [&'static str],
    description: &'static str,
    estimated_size_mb: f64,
    website: &'static str,
    apt_packages: &'static [&'static str],
    brew_packages: &'static [&'static str],
    pip_packages: &'static [&'static str],
}

/// 求解器定义表
const SOLVER_DEFS: &[SolverDef] = &[
    SolverDef {
        name: "lammps",
        display_name: "LAMMPS",
        scale: "MD",
        detect_commands: &["lmp_serial", "lmp_mpi", "lmp"],
        description: "大规模原子/分子并行模拟器，用于分子动力学仿真",
        estimated_size_mb: 500.0,
        website: "https://www.lammps.org",
        apt_packages: &["lammps"],
        brew_packages: &["lammps"],
        pip_packages: &[],
    },
    SolverDef {
        name: "quantum_espresso",
        display_name: "Quantum ESPRESSO",
        scale: "DFT",
        detect_commands: &["pw.x", "dos.x"],
        description: "开源平面波赝势密度泛函理论(DFT)计算套件",
        estimated_size_mb: 800.0,
        website: "https://www.quantum-espresso.org",
        apt_packages: &["quantum-espresso"],
        brew_packages: &["quantum-espresso"],
        pip_packages: &[],
    },
    SolverDef {
        name: "fipy",
        display_name: "FiPy",
        scale: "PhaseField",
        detect_commands: &[], // 特殊处理：通过 python3 import 检测
        description: "基于有限体积法的相场模拟框架（Python 库）",
        estimated_size_mb: 50.0,
        website: "https://www.ctcms.nist.gov/fipy/",
        apt_packages: &["python3-pip"],
        brew_packages: &["fipy"],
        pip_packages: &["fipy"],
    },
    SolverDef {
        name: "calculix",
        display_name: "CalculiX",
        scale: "FE",
        detect_commands: &["ccx"],
        description: "三维结构有限元求解器，支持线性和非线性分析",
        estimated_size_mb: 30.0,
        website: "http://www.calculix.de",
        apt_packages: &["calculix-ccx", "calculix"],
        brew_packages: &["calculix"],
        pip_packages: &[],
    },
];

// ============================================================================
// 辅助函数
// ============================================================================

/// 搜索路径列表
const SEARCH_PATHS: &[&str] = &[
    "/opt/homebrew/bin",
    "/usr/local/bin",
    "/usr/bin",
];

/// 在指定路径列表中查找可执行文件，返回完整路径
fn find_executable(name: &str) -> Option<String> {
    // 1. 先在固定路径中搜索
    for dir in SEARCH_PATHS {
        let full_path = format!("{}/{}", dir, name);
        if std::path::Path::new(&full_path).exists() {
            tracing::debug!("在固定路径找到可执行文件: {}", full_path);
            return Some(full_path);
        }
    }

    // 2. 通过 which 命令在 PATH 中搜索
    if let Ok(output) = Command::new("which").arg(name).output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                tracing::debug!("通过 which 找到可执行文件: {}", path);
                return Some(path);
            }
        }
    }

    tracing::debug!("未找到可执行文件: {}", name);
    None
}

/// 检测 FiPy 是否已安装（通过 python3 import）
fn detect_fipy() -> (bool, Option<String>, Option<String>, Option<String>) {
    let check_cmd = r#"import fipy; print(fipy.__version__)"#;
    let output = Command::new("python3")
        .args(["-c", check_cmd])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let path = find_executable("python3");
            let install_method = path.as_ref().map(|p| {
                if p.contains("homebrew") {
                    "brew"
                } else {
                    "pip"
                }
            }).map(|s| s.to_string());
            (true, Some(version), path, install_method)
        }
        _ => (false, None, None, None),
    }
}

/// 获取可执行文件版本号
fn get_version(cmd: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(cmd).args(args).output().ok()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = format!("{}\n{}", stdout, stderr);

        // 尝试从输出中提取版本号
        for line in combined.lines() {
            let line = line.trim();
            // 常见版本号模式
            if line.contains("version")
                || line.contains("Version")
                || line.contains("VERSION")
                || regex_pattern_match(line)
            {
                // 提取版本号数字部分
                if let Some(ver) = extract_version_number(line) {
                    return Some(ver);
                }
            }
        }

        // 如果没有找到版本关键字，尝试从第一行提取
        if let Some(first_line) = combined.lines().next() {
            if let Some(ver) = extract_version_number(first_line) {
                return Some(ver);
            }
        }
    }

    None
}

/// 简单的版本号模式匹配
fn regex_pattern_match(line: &str) -> bool {
    // 匹配类似 "LAMMPS (29 Sep 2021)" 或 "v1.2.3" 等格式
    let re = regex_lite_match(r"\d+\.\d+");
    re.is_some() || line.contains('(')
}

/// 简易正则匹配（避免引入 regex 依赖）
fn regex_lite_match(pattern: &str) -> Option<()> {
    // 这里仅做简单检查，实际版本提取由 extract_version_number 完成
    if pattern.contains("\\d") {
        Some(())
    } else {
        None
    }
}

/// 从文本中提取版本号
fn extract_version_number(text: &str) -> Option<String> {
    // 匹配版本号模式：X.Y.Z 或 X.Y 或 (date) 格式
    let text = text.trim();

    // 模式1: 数字.数字.数字 或 数字.数字
    for part in text.split(|c: char| !c.is_ascii_digit() && c != '.') {
        if part.contains('.') {
            let digits: Vec<&str> = part.split('.').collect();
            if digits.len() >= 2 && digits.iter().all(|d| !d.is_empty() && d.chars().all(|c| c.is_ascii_digit())) {
                return Some(part.to_string());
            }
        }
    }

    // 模式2: LAMMPS 风格日期版本 "(29 Sep 2021)"
    if text.contains('(') && text.contains(')') {
        if let Some(start) = text.find('(') {
            if let Some(end) = text[start..].find(')') {
                let date_str = &text[start + 1..start + end];
                let date_str = date_str.trim();
                if !date_str.is_empty() {
                    return Some(date_str.to_string());
                }
            }
        }
    }

    // 模式3: 在文本中寻找 X.Y 格式
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_ascii_digit() {
            let start = i;
            let mut has_dot = false;
            let mut digit_count = 0;
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                if chars[i] == '.' {
                    has_dot = true;
                } else {
                    digit_count += 1;
                }
                i += 1;
            }
            if has_dot && digit_count >= 2 {
                let ver: String = chars[start..i].iter().collect();
                // 确保不以点开头或结尾
                if !ver.starts_with('.') && !ver.ends_with('.') {
                    return Some(ver);
                }
            }
        } else {
            i += 1;
        }
    }

    None
}

/// 检测当前平台
fn get_platform() -> &'static str {
    if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "linux"
    }
}

/// 检测包管理器是否可用
fn check_package_manager(name: &str) -> bool {
    let cmd = match name {
        "apt" => "apt-get",
        "brew" => "brew",
        "pip" => "pip3",
        _ => return false,
    };

    Command::new(cmd).arg("--version").output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

// ============================================================================
// Tauri 命令
// ============================================================================

/// 检测所有求解器的安装状态
#[tauri::command]
pub fn detect_solvers() -> Result<Vec<SolverStatus>, String> {
    tracing::info!("开始检测求解器安装状态");

    let mut results = Vec::new();

    for def in SOLVER_DEFS {
        tracing::info!("检测求解器: {} ({})", def.display_name, def.scale);

        if def.name == "fipy" {
            // FiPy 特殊处理
            let (installed, version, path, install_method) = detect_fipy();
            results.push(SolverStatus {
                name: def.name.to_string(),
                display_name: def.display_name.to_string(),
                scale: def.scale.to_string(),
                installed,
                version,
                path,
                install_method,
            });
        } else {
            // 通用检测：遍历 detect_commands
            let mut found = false;
            let mut found_path = None;
            let mut found_version = None;
            let mut found_method = None;

            for cmd in def.detect_commands {
                if let Some(path) = find_executable(cmd) {
                    found = true;
                    found_path = Some(path.clone());

                    // 推断安装方法
                    found_method = if path.contains("homebrew") {
                        Some("brew".to_string())
                    } else if path.contains("/usr/bin") || path.contains("/usr/local/bin") {
                        Some("apt".to_string())
                    } else {
                        Some("bundle".to_string())
                    };

                    // 尝试获取版本
                    let version_args: &[&str] = match *cmd {
                        "lmp" | "lmp_serial" | "lmp_mpi" => &["-h"],
                        "pw.x" => &["--version"],
                        "dos.x" => &["--version"],
                        "ccx" => &["-h"],
                        _ => &["--version"],
                    };
                    found_version = get_version(cmd, version_args);

                    tracing::info!(
                        "找到 {} -> 路径: {}, 版本: {:?}, 安装方式: {:?}",
                        def.display_name,
                        found_path.as_ref().unwrap(),
                        found_version,
                        found_method
                    );
                    break;
                }
            }

            results.push(SolverStatus {
                name: def.name.to_string(),
                display_name: def.display_name.to_string(),
                scale: def.scale.to_string(),
                installed: found,
                version: found_version,
                path: found_path,
                install_method: found_method,
            });
        }
    }

    tracing::info!("求解器检测完成，共检测 {} 个求解器", results.len());
    Ok(results)
}

/// 验证指定求解器是否正常工作
#[tauri::command]
pub fn check_solver_works(solver_name: String) -> Result<SolverVerifyResult, String> {
    tracing::info!("验证求解器: {}", solver_name);

    let _def = SOLVER_DEFS.iter()
        .find(|d| d.name == solver_name)
        .ok_or_else(|| format!("未知求解器: {}", solver_name))?;

    match solver_name.as_str() {
        "lammps" => {
            // 尝试 lmp, lmp_serial, lmp_mpi
            let cmds = ["lmp", "lmp_serial", "lmp_mpi"];
            let mut last_error = None;

            for cmd in &cmds {
                if let Some(_path) = find_executable(cmd) {
                    match Command::new(cmd).arg("-h").output() {
                        Ok(output) if output.status.success() => {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            let combined = format!("{}\n{}", stdout, stderr);
                            let version = extract_version_number(&combined);

                            tracing::info!("LAMMPS 验证成功，版本: {:?}", version);
                            return Ok(SolverVerifyResult {
                                works: true,
                                version,
                                error_message: None,
                            });
                        }
                        Ok(output) => {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            last_error = Some(format!("命令 {} 执行失败: {}", cmd, stderr.trim()));
                        }
                        Err(e) => {
                            last_error = Some(format!("无法执行 {}: {}", cmd, e));
                        }
                    }
                }
            }

            tracing::warn!("LAMMPS 验证失败: {:?}", last_error);
            Ok(SolverVerifyResult {
                works: false,
                version: None,
                error_message: last_error,
            })
        }

        "quantum_espresso" => {
            // 尝试 pw.x
            match find_executable("pw.x") {
                Some(_path) => {
                    // 方法1: pw.x --version
                    let result = Command::new("pw.x").arg("--version").output();
                    match result {
                        Ok(output) if output.status.success() => {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            let version = extract_version_number(&stdout);
                            tracing::info!("Quantum ESPRESSO 验证成功，版本: {:?}", version);
                            return Ok(SolverVerifyResult {
                                works: true,
                                version,
                                error_message: None,
                            });
                        }
                        _ => {
                            // 方法2: echo "test" | pw.x -in /dev/stdin
                            let result = Command::new("sh")
                                .args(["-c", r#"echo "&CONTROL" | pw.x -in /dev/stdin"#])
                                .output();

                            match result {
                                Ok(output) => {
                                    // pw.x 即使输入不完整也会输出一些信息
                                    let stderr = String::from_utf8_lossy(&output.stderr);
                                    let stdout = String::from_utf8_lossy(&output.stdout);
                                    let combined = format!("{}\n{}", stdout, stderr);

                                    // 如果输出中包含 Quantum ESPRESSO 相关信息，说明可执行
                                    if combined.contains("Quantum ESPRESSO")
                                        || combined.contains("PWscf")
                                        || combined.contains("Program PWscf")
                                    {
                                        let version = extract_version_number(&combined);
                                        tracing::info!("Quantum ESPRESSO 验证成功（stdin模式），版本: {:?}", version);
                                        return Ok(SolverVerifyResult {
                                            works: true,
                                            version,
                                            error_message: None,
                                        });
                                    }

                                    let error_msg = format!("pw.x 执行异常: {}", combined.trim());
                                    tracing::warn!("{}", error_msg);
                                    Ok(SolverVerifyResult {
                                        works: false,
                                        version: None,
                                        error_message: Some(error_msg),
                                    })
                                }
                                Err(e) => {
                                    let error_msg = format!("无法执行 pw.x: {}", e);
                                    tracing::warn!("{}", error_msg);
                                    Ok(SolverVerifyResult {
                                        works: false,
                                        version: None,
                                        error_message: Some(error_msg),
                                    })
                                }
                            }
                        }
                    }
                }
                None => {
                    let error_msg = "未找到 pw.x 可执行文件".to_string();
                    tracing::warn!("{}", error_msg);
                    Ok(SolverVerifyResult {
                        works: false,
                        version: None,
                        error_message: Some(error_msg),
                    })
                }
            }
        }

        "fipy" => {
            let check_cmd = r#"import fipy; print(fipy.__version__)"#;
            match Command::new("python3").args(["-c", check_cmd]).output() {
                Ok(output) if output.status.success() => {
                    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    tracing::info!("FiPy 验证成功，版本: {}", version);
                    Ok(SolverVerifyResult {
                        works: true,
                        version: Some(version),
                        error_message: None,
                    })
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let error_msg = format!("FiPy 导入失败: {}", stderr.trim());
                    tracing::warn!("{}", error_msg);
                    Ok(SolverVerifyResult {
                        works: false,
                        version: None,
                        error_message: Some(error_msg),
                    })
                }
                Err(e) => {
                    let error_msg = format!("无法执行 python3: {}", e);
                    tracing::warn!("{}", error_msg);
                    Ok(SolverVerifyResult {
                        works: false,
                        version: None,
                        error_message: Some(error_msg),
                    })
                }
            }
        }

        "calculix" => {
            match find_executable("ccx") {
                Some(_path) => {
                    match Command::new("ccx").arg("-h").output() {
                        Ok(output) => {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            let combined = format!("{}\n{}", stdout, stderr);

                            // CalculiX ccx -h 通常会输出版本信息
                            let version = extract_version_number(&combined);
                            // ccx -h 可能返回非零退出码但仍然输出了版本信息
                            let works = !combined.is_empty() || output.status.success();

                            if works {
                                tracing::info!("CalculiX 验证成功，版本: {:?}", version);
                            } else {
                                tracing::warn!("CalculiX 验证失败");
                            }

                            Ok(SolverVerifyResult {
                                works,
                                version,
                                error_message: if works { None } else { Some("ccx -h 无输出".to_string()) },
                            })
                        }
                        Err(e) => {
                            let error_msg = format!("无法执行 ccx: {}", e);
                            tracing::warn!("{}", error_msg);
                            Ok(SolverVerifyResult {
                                works: false,
                                version: None,
                                error_message: Some(error_msg),
                            })
                        }
                    }
                }
                None => {
                    let error_msg = "未找到 ccx 可执行文件".to_string();
                    tracing::warn!("{}", error_msg);
                    Ok(SolverVerifyResult {
                        works: false,
                        version: None,
                        error_message: Some(error_msg),
                    })
                }
            }
        }

        _ => Err(format!("未知求解器: {}", solver_name)),
    }
}

/// 安装指定求解器
#[tauri::command]
pub fn install_solver(solver_name: String, method: String) -> Result<InstallResult, String> {
    tracing::info!("安装求解器: {}，方式: {}", solver_name, method);

    let def = SOLVER_DEFS.iter()
        .find(|d| d.name == solver_name)
        .ok_or_else(|| format!("未知求解器: {}", solver_name))?;

    let start = Instant::now();

    let result = match method.as_str() {
        "apt" => install_via_apt(def),
        "brew" => install_via_brew(def),
        "pip" => install_via_pip(def),
        _ => Err(format!("不支持的安装方式: {}", method)),
    };

    let duration = start.elapsed().as_secs_f64();

    match result {
        Ok(message) => {
            tracing::info!("求解器 {} 安装成功，耗时: {:.2}s", solver_name, duration);
            Ok(InstallResult {
                success: true,
                message,
                duration_s: duration,
            })
        }
        Err(error) => {
            tracing::error!("求解器 {} 安装失败: {}", solver_name, error);
            Ok(InstallResult {
                success: false,
                message: error,
                duration_s: duration,
            })
        }
    }
}

/// 通过 apt 安装
fn install_via_apt(def: &SolverDef) -> Result<String, String> {
    if !check_package_manager("apt") {
        return Err("apt-get 包管理器不可用，当前系统可能不支持 apt 安装".to_string());
    }

    if def.apt_packages.is_empty() {
        return Err(format!("{} 不支持通过 apt 安装", def.display_name));
    }

    // 对于 FiPy，需要先安装 pip，再安装 fipy
    if def.name == "fipy" {
        // 先确保 pip3 可用
        let pip_check = Command::new("pip3").arg("--version").output();
        if pip_check.is_err() || !pip_check.unwrap().status.success() {
            tracing::info!("pip3 不可用，尝试安装 python3-pip");
            let output = Command::new("apt-get")
                .args(["install", "-y", "python3-pip"])
                .output()
                .map_err(|e| format!("执行 apt-get install python3-pip 失败: {}", e))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!("安装 python3-pip 失败: {}", stderr.trim()));
            }
        }

        // 安装 fipy
        let output = Command::new("pip3")
            .args(["install", "--break-system-packages", "fipy"])
            .output()
            .map_err(|e| format!("执行 pip3 install fipy 失败: {}", e))?;

        if output.status.success() {
            Ok(format!("{} 安装成功（通过 pip3）", def.display_name))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("安装 {} 失败: {}", def.display_name, stderr.trim()))
        }
    } else {
        // 尝试每个包名，直到成功
        let mut last_error = String::new();
        for package in def.apt_packages {
            tracing::info!("尝试 apt-get install -y {}", package);

            let output = Command::new("apt-get")
                .args(["install", "-y", package])
                .output()
                .map_err(|e| format!("执行 apt-get install 失败: {}", e))?;

            if output.status.success() {
                return Ok(format!("{} 安装成功（通过 apt，包名: {}）", def.display_name, package));
            }

            let stderr = String::from_utf8_lossy(&output.stderr);
            last_error = format!("apt-get install {} 失败: {}", package, stderr.trim());
            tracing::warn!("{}", last_error);
        }

        Err(last_error)
    }
}

/// 通过 brew 安装
fn install_via_brew(def: &SolverDef) -> Result<String, String> {
    if !check_package_manager("brew") {
        return Err("Homebrew 包管理器不可用，请先安装 Homebrew".to_string());
    }

    if def.brew_packages.is_empty() {
        return Err(format!("{} 不支持通过 Homebrew 安装", def.display_name));
    }

    for package in def.brew_packages {
        tracing::info!("尝试 brew install {}", package);

        let output = Command::new("brew")
            .args(["install", package])
            .output()
            .map_err(|e| format!("执行 brew install 失败: {}", e))?;

        if output.status.success() {
            return Ok(format!("{} 安装成功（通过 Homebrew，包名: {}）", def.display_name, package));
        }

        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::warn!("brew install {} 失败: {}", package, stderr.trim());
    }

    Err(format!("通过 Homebrew 安装 {} 失败，所有包名均不可用", def.display_name))
}

/// 通过 pip 安装
fn install_via_pip(def: &SolverDef) -> Result<String, String> {
    if !check_package_manager("pip") {
        return Err("pip3 不可用，请先安装 Python3 和 pip3".to_string());
    }

    if def.pip_packages.is_empty() {
        return Err(format!("{} 不支持通过 pip 安装", def.display_name));
    }

    for package in def.pip_packages {
        tracing::info!("尝试 pip3 install --break-system-packages {}", package);

        let output = Command::new("pip3")
            .args(["install", "--break-system-packages", package])
            .output()
            .map_err(|e| format!("执行 pip3 install 失败: {}", e))?;

        if output.status.success() {
            return Ok(format!("{} 安装成功（通过 pip3，包名: {}）", def.display_name, package));
        }

        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::warn!("pip3 install {} 失败: {}", package, stderr.trim());
    }

    Err(format!("通过 pip3 安装 {} 失败", def.display_name))
}

/// 卸载指定求解器
#[tauri::command]
pub fn uninstall_solver(solver_name: String) -> Result<UninstallResult, String> {
    tracing::info!("卸载求解器: {}", solver_name);

    let def = SOLVER_DEFS.iter()
        .find(|d| d.name == solver_name)
        .ok_or_else(|| format!("未知求解器: {}", solver_name))?;

    let platform = get_platform();

    // 根据平台选择卸载方式
    let result = match platform {
        "macos" => uninstall_via_brew(def),
        "linux" => uninstall_via_apt(def),
        _ => Err(format!("不支持在 {} 平台上卸载求解器", platform)),
    };

    match result {
        Ok(message) => {
            tracing::info!("求解器 {} 卸载成功", solver_name);
            Ok(UninstallResult {
                success: true,
                message,
            })
        }
        Err(error) => {
            tracing::error!("求解器 {} 卸载失败: {}", solver_name, error);
            Ok(UninstallResult {
                success: false,
                message: error,
            })
        }
    }
}

/// 通过 apt 卸载
fn uninstall_via_apt(def: &SolverDef) -> Result<String, String> {
    if !check_package_manager("apt") {
        return Err("apt-get 包管理器不可用".to_string());
    }

    if def.name == "fipy" {
        let output = Command::new("pip3")
            .args(["uninstall", "-y", "fipy"])
            .output()
            .map_err(|e| format!("执行 pip3 uninstall 失败: {}", e))?;

        if output.status.success() {
            return Ok(format!("{} 卸载成功（通过 pip3）", def.display_name));
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("卸载 {} 失败: {}", def.display_name, stderr.trim()))
    } else {
        let mut last_error = String::new();
        for package in def.apt_packages {
            let output = Command::new("apt-get")
                .args(["remove", "-y", package])
                .output()
                .map_err(|e| format!("执行 apt-get remove 失败: {}", e))?;

            if output.status.success() {
                return Ok(format!("{} 卸载成功（通过 apt，包名: {}）", def.display_name, package));
            }
            let stderr = String::from_utf8_lossy(&output.stderr);
            last_error = format!("apt-get remove {} 失败: {}", package, stderr.trim());
        }
        Err(last_error)
    }
}

/// 通过 brew 卸载
fn uninstall_via_brew(def: &SolverDef) -> Result<String, String> {
    if !check_package_manager("brew") {
        return Err("Homebrew 包管理器不可用".to_string());
    }

    if def.name == "fipy" {
        let output = Command::new("pip3")
            .args(["uninstall", "-y", "fipy"])
            .output()
            .map_err(|e| format!("执行 pip3 uninstall 失败: {}", e))?;

        if output.status.success() {
            return Ok(format!("{} 卸载成功（通过 pip3）", def.display_name));
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("卸载 {} 失败: {}", def.display_name, stderr.trim()))
    } else {
        for package in def.brew_packages {
            let output = Command::new("brew")
                .args(["uninstall", package])
                .output()
                .map_err(|e| format!("执行 brew uninstall 失败: {}", e))?;

            if output.status.success() {
                return Ok(format!("{} 卸载成功（通过 Homebrew，包名: {}）", def.display_name, package));
            }
            let stderr = String::from_utf8_lossy(&output.stderr);
            tracing::warn!("brew uninstall {} 失败: {}", package, stderr.trim());
        }
        Err(format!("通过 Homebrew 卸载 {} 失败", def.display_name))
    }
}

/// 获取当前平台可用的安装方法列表
#[tauri::command]
pub fn get_install_methods() -> Result<Vec<InstallMethod>, String> {
    tracing::info!("获取可用安装方法");

    let platform = get_platform();
    tracing::debug!("当前平台: {}", platform);

    let mut methods = Vec::new();

    // apt (Linux)
    let apt_available = check_package_manager("apt");
    methods.push(InstallMethod {
        name: "apt".to_string(),
        display_name: "APT (Debian/Ubuntu)".to_string(),
        available: apt_available,
    });

    // brew (macOS, 也可在 Linux 上使用)
    let brew_available = check_package_manager("brew");
    methods.push(InstallMethod {
        name: "brew".to_string(),
        display_name: "Homebrew".to_string(),
        available: brew_available,
    });

    // pip
    let pip_available = check_package_manager("pip");
    methods.push(InstallMethod {
        name: "pip".to_string(),
        display_name: "pip (Python)".to_string(),
        available: pip_available,
    });

    tracing::info!("可用安装方法: {:?}", methods.iter().filter(|m| m.available).map(|m| &m.name).collect::<Vec<_>>());
    Ok(methods)
}

/// 获取单个求解器的详细信息
#[tauri::command]
pub fn get_solver_info(solver_name: String) -> Result<SolverInfo, String> {
    tracing::info!("获取求解器信息: {}", solver_name);

    let def = SOLVER_DEFS.iter()
        .find(|d| d.name == solver_name)
        .ok_or_else(|| format!("未知求解器: {}", solver_name))?;

    // 检测当前安装状态
    let (installed, version, path) = if def.name == "fipy" {
        let (inst, ver, p, _method) = detect_fipy();
        (inst, ver, p)
    } else {
        let mut found_installed = false;
        let mut found_version = None;
        let mut found_path = None;

        for cmd in def.detect_commands {
            if let Some(p) = find_executable(cmd) {
                found_installed = true;
                found_path = Some(p);
                let version_args: &[&str] = match *cmd {
                    "lmp" | "lmp_serial" | "lmp_mpi" => &["-h"],
                    "pw.x" | "dos.x" => &["--version"],
                    "ccx" => &["-h"],
                    _ => &["--version"],
                };
                found_version = get_version(cmd, version_args);
                break;
            }
        }
        (found_installed, found_version, found_path)
    };

    // 确定可用的安装方法
    let mut available_methods = Vec::new();
    if !def.apt_packages.is_empty() && check_package_manager("apt") {
        available_methods.push("apt".to_string());
    }
    if !def.brew_packages.is_empty() && check_package_manager("brew") {
        available_methods.push("brew".to_string());
    }
    if !def.pip_packages.is_empty() && check_package_manager("pip") {
        available_methods.push("pip".to_string());
    }

    // 如果平台特定方法都不可用，仍然列出理论支持的方法
    if available_methods.is_empty() {
        if !def.apt_packages.is_empty() {
            available_methods.push("apt".to_string());
        }
        if !def.brew_packages.is_empty() {
            available_methods.push("brew".to_string());
        }
        if !def.pip_packages.is_empty() {
            available_methods.push("pip".to_string());
        }
    }

    let info = SolverInfo {
        name: def.name.to_string(),
        display_name: def.display_name.to_string(),
        scale: def.scale.to_string(),
        description: def.description.to_string(),
        version,
        path,
        installed,
        install_methods: available_methods,
        estimated_size_mb: def.estimated_size_mb,
        website: def.website.to_string(),
    };

    tracing::info!("求解器 {} 信息获取完成，已安装: {}", solver_name, installed);
    Ok(info)
}
