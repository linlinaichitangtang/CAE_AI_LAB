/**
 * V4.2-001: Python 运行时桥接
 * 通用 Python 子进程调用框架，支持数据交换和包管理
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::timeout;

/// Python 运行时信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonRuntimeInfo {
    pub available: bool,
    pub path: String,
    pub version: String,
    pub version_major: u8,
    pub version_minor: u8,
    pub packages: Vec<String>,
}

/// Python 脚本执行请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonScriptRequest {
    pub script: String,
    pub input_data: Option<Value>,
    pub timeout_sec: Option<u64>,
    pub use_temp_file_for_input: Option<bool>,
}

/// Python 脚本执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonScriptResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub output_data: Option<Value>,
    pub execution_time_ms: u64,
}

/// Python 包检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageCheckResult {
    pub installed: bool,
    pub version: Option<String>,
}

/// 模块调用请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleRunRequest {
    pub module: String,
    pub args: Vec<String>,
    pub timeout_sec: Option<u64>,
}

// ============================================================================
// Python 检测
// ============================================================================

/// 检测系统中可用的 Python 解释器
pub fn detect_python() -> Result<PythonRuntimeInfo, String> {
    for cmd_name in ["python3", "python", "py"] {
        match check_python_executable(cmd_name) {
            Ok(info) => return Ok(info),
            Err(_) => continue,
        }
    }
    Err("No Python interpreter found. Please install Python 3.8+".to_string())
}

fn check_python_executable(cmd: &str) -> Result<PythonRuntimeInfo, String> {
    let output = Command::new(cmd)
        .args(["--version"])
        .output()
        .map_err(|e| format!("Failed to run {}: {}", cmd, e))?;

    if !output.status.success() {
        return Err(format!("{} returned non-zero exit code", cmd));
    }

    let version_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let version_str = if version_str.is_empty() {
        String::from_utf8_lossy(&output.stderr).trim().to_string()
    } else {
        version_str
    };

    let parts: Vec<&str> = version_str.split_whitespace().collect();
    if parts.len() < 2 {
        return Err("Cannot parse Python version".to_string());
    }

    let ver_nums: Vec<&str> = parts[1].split('.').collect();
    let major = ver_nums.first().and_then(|s| s.parse().ok()).unwrap_or(0u8);
    let minor = ver_nums.get(1).and_then(|s| s.parse().ok()).unwrap_or(0u8);

    if major < 3 || (major == 3 && minor < 8) {
        return Err(format!("Python {}.{} is too old (need 3.8+)", major, minor));
    }

    let packages = list_installed_packages(cmd)?;

    Ok(PythonRuntimeInfo {
        available: true,
        path: which_python(cmd)?,
        version: version_str,
        version_major: major,
        version_minor: minor,
        packages,
    })
}

fn which_python(cmd: &str) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        Ok(format!("{}.exe", cmd))
    }
    #[cfg(not(target_os = "windows"))]
    {
        let output = Command::new("which")
            .arg(cmd)
            .output()
            .map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Ok(cmd.to_string())
        }
    }
}

fn list_installed_packages(python_cmd: &str) -> Result<Vec<String>, String> {
    let output = Command::new(python_cmd)
        .args(["-m", "pip", "list", "--format=freeze"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Ok(vec![]);
    }

    let pkgs = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            line.split("==").next().map(|s| s.to_string())
        })
        .collect();

    Ok(pkgs)
}

// ============================================================================
// 脚本执行（异步）
// ============================================================================

/// 执行 Python 脚本，通过 stdin 传递 JSON 输入，从 stdout 解析 JSON 输出
pub async fn run_python_script_async(req: &PythonScriptRequest) -> Result<PythonScriptResult, String> {
    let python = detect_python()?;
    let timeout_dur = Duration::from_secs(req.timeout_sec.unwrap_or(300));
    let start = std::time::Instant::now();

    // Determine input delivery method
    let input_json_str = match &req.input_data {
        Some(data) => Some(
            serde_json::to_string(data)
                .map_err(|e| format!("Failed to serialize input: {}", e))?,
        ),
        None => None,
    };

    let use_file = req.use_temp_file_for_input.unwrap_or(false)
        || input_json_str.as_ref().map(|s| s.len() > 1_000_000).unwrap_or(false);

    let temp_path: Option<PathBuf> = if use_file {
        if let Some(ref json) = input_json_str {
            let path = write_temp_json_file(json)?;
            Some(path)
        } else {
            None
        }
    } else {
        None
    };

    let script = req.script.clone();
    let python_path = python.path.clone();
    let input_for_stdin = if temp_path.is_none() { input_json_str } else { None };
    let env_input_path = temp_path.clone();

    let result = tokio::task::spawn_blocking(move || {
        let mut cmd = Command::new(&python_path);
        cmd.arg("-u").arg("-c").arg(&script);

        if let Some(ref path) = env_input_path {
            cmd.env("CAELAB_INPUT_FILE", path);
            cmd.stdin(Stdio::null());
        } else {
            cmd.stdin(Stdio::piped());
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| format!("Failed to spawn Python: {}", e))?;

        // Write stdin if needed
        if let Some(ref json) = input_for_stdin {
            if let Some(ref mut stdin) = child.stdin {
                stdin
                    .write_all(json.as_bytes())
                    .map_err(|e| format!("Failed to write stdin: {}", e))?;
            }
        }

        let output = child
            .wait_with_output()
            .map_err(|e| format!("Failed to read Python output: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        let output_data = parse_json_output(&stdout);

        Ok(PythonScriptResult {
            success: output.status.success(),
            stdout,
            stderr,
            output_data,
            execution_time_ms: start.elapsed().as_millis() as u64,
        })
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?;

    // Clean up temp file
    if let Some(path) = temp_path {
        let _ = std::fs::remove_file(&path);
    }

    match timeout(timeout_dur, std::future::ready(result)).await {
        Ok(res) => res,
        Err(_) => Err(format!("Python script timed out after {}s", timeout_dur.as_secs())),
    }
}

/// 执行 Python 模块
pub async fn run_python_module_async(req: &ModuleRunRequest) -> Result<PythonScriptResult, String> {
    let python = detect_python()?;
    let timeout_dur = Duration::from_secs(req.timeout_sec.unwrap_or(300));
    let start = std::time::Instant::now();

    let module = req.module.clone();
    let args = req.args.clone();
    let python_path = python.path.clone();

    let result = tokio::task::spawn_blocking(move || {
        let output = Command::new(&python_path)
            .arg("-m")
            .arg(&module)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("Failed to run Python module: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let output_data = parse_json_output(&stdout);

        Ok(PythonScriptResult {
            success: output.status.success(),
            stdout,
            stderr,
            output_data,
            execution_time_ms: start.elapsed().as_millis() as u64,
        })
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?;

    match timeout(timeout_dur, std::future::ready(result)).await {
        Ok(res) => res,
        Err(_) => Err(format!(
            "Python module timed out after {}s",
            timeout_dur.as_secs()
        )),
    }
}

// ============================================================================
// 包管理
// ============================================================================

/// 检查 Python 包是否安装
pub async fn check_python_package_async(package: &str) -> Result<PackageCheckResult, String> {
    let script = format!(
        r#"import importlib.metadata as m
try: pkg = m.distribution('{}'); print(f'{{pkg.metadata["Version"]}}')
except Exception: print('NOT_INSTALLED')"#,
        package
    );

    let req = PythonScriptRequest {
        script,
        input_data: None,
        timeout_sec: Some(30),
        use_temp_file_for_input: None,
    };

    let result = run_python_script_async(&req).await?;
    if !result.success {
        return Ok(PackageCheckResult {
            installed: false,
            version: None,
        });
    }

    let trimmed = result.stdout.trim();
    if trimmed == "NOT_INSTALLED" {
        Ok(PackageCheckResult {
            installed: false,
            version: None,
        })
    } else {
        Ok(PackageCheckResult {
            installed: true,
            version: Some(trimmed.to_string()),
        })
    }
}

/// 安装 Python 包
pub async fn install_python_package_async(package: &str) -> Result<String, String> {
    let req = ModuleRunRequest {
        module: "pip".to_string(),
        args: vec!["install".to_string(), package.to_string()],
        timeout_sec: Some(300),
    };

    let result = run_python_module_async(&req).await?;
    if result.success {
        Ok(format!("Installed {} successfully", package))
    } else {
        Err(format!(
            "Failed to install {}:\nstdout: {}\nstderr: {}",
            package, result.stdout, result.stderr
        ))
    }
}

// ============================================================================
// 工具函数
// ============================================================================

fn parse_json_output(stdout: &str) -> Option<Value> {
    for line in stdout.lines().rev() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Ok(val) = serde_json::from_str::<Value>(trimmed) {
            return Some(val);
        }
    }
    None
}

fn write_temp_json_file(json_str: &str) -> Result<PathBuf, String> {
    let mut path = std::env::temp_dir();
    path.push(format!("caelab_input_{}.json", uuid::Uuid::new_v4()));
    std::fs::write(&path, json_str).map_err(|e| format!("Failed to write temp file: {}", e))?;
    Ok(path)
}

// ============================================================================
// Tauri 命令
// ============================================================================

#[tauri::command]
pub async fn python_check_environment() -> Result<PythonRuntimeInfo, String> {
    detect_python()
}

#[tauri::command]
pub async fn python_run_script(request: PythonScriptRequest) -> Result<PythonScriptResult, String> {
    run_python_script_async(&request).await
}

#[tauri::command]
pub async fn python_run_module(request: ModuleRunRequest) -> Result<PythonScriptResult, String> {
    run_python_module_async(&request).await
}

#[tauri::command]
pub async fn python_check_package(package: String) -> Result<PackageCheckResult, String> {
    check_python_package_async(&package).await
}

#[tauri::command]
pub async fn python_install_package(package: String) -> Result<String, String> {
    install_python_package_async(&package).await
}
