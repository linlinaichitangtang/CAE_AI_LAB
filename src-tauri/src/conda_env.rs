/**
 * V4.4-001: Miniconda 环境管理
 *
 * 首次启动自动下载 Miniconda → 创建 caelab-ml conda 环境
 * （Python 3.10 + PyTorch + ASE），后续 Python 调用优先使用该环境。
 */

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::Manager;

// ============================================================================
// 数据结构
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CondaEnvStatus {
    pub installed: bool,
    pub env_name: String,
    pub conda_dir: Option<String>,
    pub python_path: Option<String>,
    pub python_version: Option<String>,
    pub pytorch_version: Option<String>,
    pub ase_version: Option<String>,
    pub packages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapProgress {
    pub stage: String,
    pub percent: f32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallPackageResult {
    pub success: bool,
    pub package: String,
    pub version: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CondaEnvRecord {
    created_at: Option<String>,
    python_version: Option<String>,
    packages: Vec<String>,
}

// ============================================================================
// 路径辅助
// ============================================================================

fn conda_base_dir(app_data: &Path) -> PathBuf {
    app_data.join("miniconda")
}

fn conda_bin(app_data: &Path) -> PathBuf {
    let base = conda_base_dir(app_data);
    if cfg!(target_os = "windows") {
        base.join("Scripts").join("conda.exe")
    } else {
        base.join("bin").join("conda")
    }
}

fn env_python_path(app_data: &Path) -> PathBuf {
    let base = conda_base_dir(app_data);
    if cfg!(target_os = "windows") {
        base.join("envs").join("caelab-ml").join("python.exe")
    } else {
        base.join("envs").join("caelab-ml").join("bin").join("python3")
    }
}

fn env_record_path(app_data: &Path) -> PathBuf {
    app_data.join("conda_env.json")
}

fn miniconda_installer_url() -> &'static str {
    if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
        "https://repo.anaconda.com/miniconda/Miniconda3-latest-MacOSX-arm64.sh"
    } else if cfg!(target_os = "macos") {
        "https://repo.anaconda.com/miniconda/Miniconda3-latest-MacOSX-x86_64.sh"
    } else if cfg!(target_os = "linux") {
        "https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh"
    } else {
        "https://repo.anaconda.com/miniconda/Miniconda3-latest-Windows-x86_64.exe"
    }
}

fn installer_filename() -> &'static str {
    if cfg!(target_os = "windows") {
        "Miniconda3-latest-Windows-x86_64.exe"
    } else if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
        "Miniconda3-latest-MacOSX-arm64.sh"
    } else if cfg!(target_os = "macos") {
        "Miniconda3-latest-MacOSX-x86_64.sh"
    } else {
        "Miniconda3-latest-Linux-x86_64.sh"
    }
}

// ============================================================================
// 检测
// ============================================================================

/// 检测 CAELab conda 环境是否已安装
pub fn detect_conda_env(app_data: &Path) -> CondaEnvStatus {
    let python = env_python_path(app_data);
    let conda = conda_bin(app_data);

    if !python.exists() || !conda.exists() {
        return CondaEnvStatus {
            installed: false,
            env_name: "caelab-ml".to_string(),
            conda_dir: None,
            python_path: None,
            python_version: None,
            pytorch_version: None,
            ase_version: None,
            packages: vec![],
        };
    }

    let python_str = python.to_string_lossy().to_string();
    let version = run_python_version(&python);
    let pytorch_ver = check_package_version(&python, "torch");
    let ase_ver = check_package_version(&python, "ase");
    let packages = list_env_packages(&conda, app_data);

    CondaEnvStatus {
        installed: true,
        env_name: "caelab-ml".to_string(),
        conda_dir: Some(conda_base_dir(app_data).to_string_lossy().to_string()),
        python_path: Some(python_str),
        python_version: version,
        pytorch_version: pytorch_ver,
        ase_version: ase_ver,
        packages,
    }
}

/// 获取 conda 环境中的 Python 路径（供 python_bridge 和 code_exec 使用）
pub fn get_conda_python(app_data: &Path) -> Option<PathBuf> {
    let python = env_python_path(app_data);
    if python.exists() { Some(python) } else { None }
}

// ============================================================================
// Bootstrap: 下载 + 安装
// ============================================================================

/// 完整 bootstrap 流程，通过回调推送进度
pub fn bootstrap_conda_env<F>(app_data: &Path, on_progress: F) -> Result<CondaEnvStatus, String>
where
    F: Fn(BootstrapProgress),
{
    let conda = conda_bin(app_data);
    let base_dir = conda_base_dir(app_data);

    // Stage 1: 下载 Miniconda（如果尚未安装）
    if !conda.exists() {
        on_progress(BootstrapProgress {
            stage: "download".to_string(),
            percent: 0.0,
            message: "下载 Miniconda 安装包...".to_string(),
        });

        download_miniconda(app_data, &on_progress)?;

        on_progress(BootstrapProgress {
            stage: "install".to_string(),
            percent: 40.0,
            message: "安装 Miniconda...".to_string(),
        });

        install_miniconda(app_data)?;
    }

    // Stage 2: 创建 caelab-ml 环境
    on_progress(BootstrapProgress {
        stage: "create_env".to_string(),
        percent: 50.0,
        message: "创建 caelab-ml 环境 (Python 3.10)...".to_string(),
    });

    create_env(&conda)?;

    // Stage 3: 安装 PyTorch
    on_progress(BootstrapProgress {
        stage: "install_pytorch".to_string(),
        percent: 60.0,
        message: "安装 PyTorch (CPU)...".to_string(),
    });

    install_pytorch(&conda)?;

    // Stage 4: 安装 ASE
    on_progress(BootstrapProgress {
        stage: "install_ase".to_string(),
        percent: 80.0,
        message: "安装 ASE (原子模拟环境)...".to_string(),
    });

    install_ase(&env_python_path(app_data))?;

    // Stage 5: 验证
    on_progress(BootstrapProgress {
        stage: "verify".to_string(),
        percent: 95.0,
        message: "验证安装...".to_string(),
    });

    verify_installation(app_data)?;

    // 保存记录
    save_env_record(app_data)?;

    on_progress(BootstrapProgress {
        stage: "done".to_string(),
        percent: 100.0,
        message: "AI 环境安装完成".to_string(),
    });

    Ok(detect_conda_env(app_data))
}

// ============================================================================
// 安装包到已有环境
// ============================================================================

pub fn install_package(app_data: &Path, package: &str) -> InstallPackageResult {
    let python = env_python_path(app_data);
    if !python.exists() {
        return InstallPackageResult {
            success: false,
            package: package.to_string(),
            version: None,
            message: "caelab-ml 环境未安装".to_string(),
        };
    }

    let output = Command::new(&python)
        .args(["-m", "pip", "install", package])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let ver = check_package_version(&python, package);
            InstallPackageResult {
                success: true,
                package: package.to_string(),
                version: ver,
                message: format!("{} 安装成功", package),
            }
        }
        Ok(o) => InstallPackageResult {
            success: false,
            package: package.to_string(),
            version: None,
            message: String::from_utf8_lossy(&o.stderr).to_string(),
        },
        Err(e) => InstallPackageResult {
            success: false,
            package: package.to_string(),
            version: None,
            message: format!("pip 执行失败: {}", e),
        },
    }
}

// ============================================================================
// 内部实现
// ============================================================================

fn download_miniconda<F>(app_data: &Path, on_progress: &F) -> Result<(), String>
where
    F: Fn(BootstrapProgress),
{
    let url = miniconda_installer_url();
    let installer_dir = app_data.join("installers");
    fs::create_dir_all(&installer_dir).map_err(|e| format!("创建目录失败: {}", e))?;
    let installer_path = installer_dir.join(installer_filename());

    // 如果已下载则跳过
    if installer_path.exists() && installer_path.metadata().map(|m| m.len() > 1_000_000).unwrap_or(false) {
        return Ok(());
    }

    // 使用 curl 下载（macOS/Linux 保证存在）
    #[cfg(not(target_os = "windows"))]
    {
        let output = Command::new("curl")
            .args(["-fSL", "-o", installer_path.to_str().unwrap(), url])
            .output()
            .map_err(|e| format!("curl 执行失败: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Miniconda 下载失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }

    // Windows 使用 PowerShell
    #[cfg(target_os = "windows")]
    {
        let ps_cmd = format!(
            "Invoke-WebRequest -Uri '{}' -OutFile '{}'",
            url,
            installer_path.to_string_lossy()
        );
        let output = Command::new("powershell")
            .args(["-Command", &ps_cmd])
            .output()
            .map_err(|e| format!("PowerShell 执行失败: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Miniconda 下载失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }

    on_progress(BootstrapProgress {
        stage: "download".to_string(),
        percent: 30.0,
        message: "Miniconda 下载完成".to_string(),
    });

    Ok(())
}

fn install_miniconda(app_data: &Path) -> Result<(), String> {
    let installer_dir = app_data.join("installers");
    let installer_path = installer_dir.join(installer_filename());
    let target_dir = conda_base_dir(app_data);

    if !installer_path.exists() {
        return Err("Miniconda 安装包不存在".to_string());
    }

    #[cfg(not(target_os = "windows"))]
    {
        let output = Command::new("bash")
            .args([
                installer_path.to_str().unwrap(),
                "-b",
                "-p",
                target_dir.to_str().unwrap(),
            ])
            .output()
            .map_err(|e| format!("安装脚本执行失败: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Miniconda 安装失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }

    #[cfg(target_os = "windows")]
    {
        let output = Command::new(&installer_path)
            .args([
                "/InstallationType=JustMe",
                "/AddToPath=0",
                "/RegisterPython=0",
                &format!("/D={}", target_dir.to_string_lossy()),
            ])
            .output()
            .map_err(|e| format!("安装程序执行失败: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Miniconda 安装失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }

    // 清理安装包
    let _ = fs::remove_file(&installer_path);

    Ok(())
}

fn create_env(conda: &Path) -> Result<(), String> {
    let output = Command::new(conda)
        .args(["create", "-n", "caelab-ml", "python=3.10", "-y"])
        .output()
        .map_err(|e| format!("conda create 失败: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // 环境已存在不算错误
        if stderr.contains("already exists") {
            return Ok(());
        }
        return Err(format!("创建环境失败: {}", stderr));
    }
    Ok(())
}

fn install_pytorch(conda: &Path) -> Result<(), String> {
    let output = Command::new(conda)
        .args([
            "install", "-n", "caelab-ml",
            "pytorch", "cpuonly",
            "-c", "pytorch",
            "-y",
        ])
        .output()
        .map_err(|e| format!("conda install pytorch 失败: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "PyTorch 安装失败: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

fn install_ase(python: &Path) -> Result<(), String> {
    let output = Command::new(python)
        .args(["-m", "pip", "install", "ase"])
        .output()
        .map_err(|e| format!("pip install ase 失败: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "ASE 安装失败: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

fn verify_installation(app_data: &Path) -> Result<(), String> {
    let python = env_python_path(app_data);
    let output = Command::new(&python)
        .args([
            "-c",
            "import torch; import ase; print(f'torch={torch.__version__} ase={ase.__version__}')",
        ])
        .output()
        .map_err(|e| format!("验证执行失败: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "验证失败: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

fn run_python_version(python: &Path) -> Option<String> {
    let output = Command::new(python)
        .args(["--version"])
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let text = if text.trim().is_empty() {
        String::from_utf8_lossy(&output.stderr).to_string()
    } else {
        text.to_string()
    };
    let ver = text.trim().strip_prefix("Python ").unwrap_or(text.trim());
    Some(ver.to_string())
}

fn check_package_version(python: &Path, package: &str) -> Option<String> {
    let script = format!(
        "import importlib.metadata as m; print(m.distribution('{}').metadata['Version'])",
        package
    );
    let output = Command::new(python).args(["-c", &script]).output().ok()?;
    if output.status.success() {
        let ver = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !ver.is_empty() {
            return Some(ver);
        }
    }
    None
}

fn list_env_packages(conda: &Path, app_data: &Path) -> Vec<String> {
    let output = Command::new(conda)
        .args(["list", "-n", "caelab-ml", "--json"])
        .output();
    match output {
        Ok(o) if o.status.success() => {
            let text = String::from_utf8_lossy(&o.stdout);
            serde_json::from_str::<Vec<serde_json::Value>>(&text)
                .unwrap_or_default()
                .iter()
                .filter_map(|v| v.get("name").and_then(|n| n.as_str()).map(|s| s.to_string()))
                .collect()
        }
        _ => vec![],
    }
}

fn save_env_record(app_data: &Path) -> Result<(), String> {
    let record = CondaEnvRecord {
        created_at: Some(chrono::Utc::now().to_rfc3339()),
        python_version: run_python_version(&env_python_path(app_data)),
        packages: list_env_packages(&conda_bin(app_data), app_data),
    };
    let json = serde_json::to_string_pretty(&record).map_err(|e| e.to_string())?;
    fs::write(env_record_path(app_data), json).map_err(|e| format!("保存记录失败: {}", e))?;
    Ok(())
}

// ============================================================================
// Tauri 命令
// ============================================================================

use tauri::AppHandle;

#[tauri::command]
pub async fn conda_get_status(app: AppHandle) -> Result<CondaEnvStatus, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("获取数据目录失败: {}", e))?;
    Ok(detect_conda_env(&app_data))
}

#[tauri::command]
pub async fn conda_bootstrap(app: AppHandle) -> Result<CondaEnvStatus, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("获取数据目录失败: {}", e))?;
    fs::create_dir_all(&app_data).map_err(|e| format!("创建数据目录失败: {}", e))?;

    // 在阻塞线程中运行安装
    let app_data_clone = app_data.clone();
    tokio::task::spawn_blocking(move || {
        bootstrap_conda_env(&app_data_clone, |_progress| {
            // TODO: 通过 Tauri event 向前端推送进度
            // app.emit("conda-progress", progress).ok();
        })
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?
}

#[tauri::command]
pub async fn conda_install_package(
    app: AppHandle,
    package: String,
) -> Result<InstallPackageResult, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("获取数据目录失败: {}", e))?;

    let pkg = package.clone();
    let result = tokio::task::spawn_blocking(move || install_package(&app_data, &pkg))
        .await
        .map_err(|e| format!("任务执行失败: {}", e))?;

    Ok(result)
}
