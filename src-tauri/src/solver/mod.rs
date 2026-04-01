//! Solver module
//! Contains mesh generation and solver utilities

pub mod mesh;
pub mod bc;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::io::{BufRead, BufReader};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolverError {
    #[error("CalculiX executable not found at: {0}")]
    ExecutableNotFound(PathBuf),
    #[error("Solver execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Input file error: {0}")]
    InputFileError(String),
    #[error("Output parsing error: {0}")]
    OutputParseError(String),
    #[error("Solver was cancelled")]
    Cancelled,
}

/// 求解器事件 - 用于向前端推送进度和日志
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SolverEvent {
    /// 进度更新 (百分比 0-100)
    Progress {
        percent: f64,
        message: String,
    },
    /// 日志行
    Log {
        level: String,
        message: String,
    },
    /// 求解完成
    Completed {
        success: bool,
        job_id: String,
        elapsed_time_seconds: f64,
        warnings: Vec<String>,
        errors: Vec<String>,
        output_file: Option<String>,
        frd_file: Option<String>,
    },
    /// 求解被取消
    Cancelled,
    /// 错误
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverConfig {
    pub executable_path: String,
    pub num_threads: usize,
    pub memory_limit_mb: Option<usize>,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            executable_path: "ccx".to_string(),
            num_threads: 4,
            memory_limit_mb: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverResult {
    pub success: bool,
    pub job_id: String,
    pub elapsed_time_seconds: f64,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub output_file: Option<PathBuf>,
    pub frd_file: Option<PathBuf>,
}

pub struct CalculiXSolver {
    config: SolverConfig,
}

impl CalculiXSolver {
    pub fn new(config: SolverConfig) -> Self {
        Self { config }
    }

    pub fn check_availability(&self) -> Result<bool, SolverError> {
        let output = Command::new(&self.config.executable_path)
            .arg("-v")
            .output()
            .map_err(|_| SolverError::ExecutableNotFound(PathBuf::from(&self.config.executable_path)))?;
        Ok(output.status.success())
    }

    pub fn solve(&self, input_file: &PathBuf, working_dir: &PathBuf) -> Result<SolverResult, SolverError> {
        let input_file_name = input_file
            .file_stem()
            .ok_or_else(|| SolverError::InputFileError("Invalid input file name".to_string()))?
            .to_str()
            .ok_or_else(|| SolverError::InputFileError("Invalid input file name encoding".to_string()))?;

        let output_file = working_dir.join(format!("{}.dat", input_file_name));
        let frd_file = working_dir.join(format!("{}.frd", input_file_name));
        
        let mut args = vec![];
        
        if self.config.num_threads > 0 {
            args.push("-nt".to_string());
            args.push(self.config.num_threads.to_string());
        }
        
        if let Some(mem_mb) = self.config.memory_limit_mb {
            args.push("-memory".to_string());
            args.push(format!("{}MB", mem_mb));
        }
        
        args.push(input_file_name.to_string());

        tracing::info!("Running CalculiX: {} {:?}", self.config.executable_path, args);

        let start_time = std::time::Instant::now();
        
        let output = Command::new(&self.config.executable_path)
            .args(&args)
            .current_dir(working_dir)
            .output()
            .map_err(|e| SolverError::ExecutionFailed(e.to_string()))?;

        let elapsed = start_time.elapsed().as_secs_f64();

        let _stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let warnings: Vec<String> = stderr
            .lines()
            .filter(|line| line.contains("WARNING") || line.contains("Warning"))
            .map(|s| s.to_string())
            .collect();

        let errors: Vec<String> = stderr
            .lines()
            .filter(|line| line.contains("ERROR") || line.contains("Error"))
            .map(|s| s.to_string())
            .collect();

        let success = output.status.success() && errors.is_empty();

        tracing::info!("CalculiX finished in {:.2}s, success: {}", elapsed, success);

        Ok(SolverResult {
            success,
            job_id: uuid::Uuid::new_v4().to_string(),
            elapsed_time_seconds: elapsed,
            warnings,
            errors,
            output_file: Some(output_file),
            frd_file: Some(frd_file),
        })
    }

    /// 异步求解，通过回调推送进度和日志
    pub fn solve_with_progress<F>(
        &self,
        input_file: &PathBuf,
        working_dir: &PathBuf,
        on_event: F,
        cancel_flag: Arc<AtomicBool>,
    ) -> Result<SolverResult, SolverError>
    where
        F: Fn(SolverEvent) + Send + 'static,
    {
        let input_file_name = input_file
            .file_stem()
            .ok_or_else(|| SolverError::InputFileError("Invalid input file name".to_string()))?
            .to_str()
            .ok_or_else(|| SolverError::InputFileError("Invalid input file name encoding".to_string()))?;

        let output_file = working_dir.join(format!("{}.dat", input_file_name));
        let frd_file = working_dir.join(format!("{}.frd", input_file_name));

        let mut args = vec![];

        if self.config.num_threads > 0 {
            args.push("-nt".to_string());
            args.push(self.config.num_threads.to_string());
        }

        if let Some(mem_mb) = self.config.memory_limit_mb {
            args.push("-memory".to_string());
            args.push(format!("{}MB", mem_mb));
        }

        args.push(input_file_name.to_string());

        tracing::info!("Running CalculiX (with progress): {} {:?}", self.config.executable_path, args);

        let start_time = std::time::Instant::now();

        // 使用 spawn() 启动进程，实现流式读取
        let mut child = Command::new(&self.config.executable_path)
            .args(&args)
            .current_dir(working_dir)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| SolverError::ExecutionFailed(e.to_string()))?;

        // 读取 stdout
        let stdout = child.stdout.take().ok_or_else(|| {
            SolverError::ExecutionFailed("Failed to capture stdout".to_string())
        })?;

        // 读取 stderr
        let stderr = child.stderr.take().ok_or_else(|| {
            SolverError::ExecutionFailed("Failed to capture stderr".to_string())
        })?;

        // 用于在线程间共享收集到的日志数据
        let warnings = Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
        let errors = Arc::new(std::sync::Mutex::new(Vec::<String>::new()));

        let warnings_clone = warnings.clone();
        let errors_clone = errors.clone();
        let cancel_flag_clone = cancel_flag.clone();

        // 在独立线程中读取 stderr
        let stderr_thread = std::thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if cancel_flag_clone.load(Ordering::Relaxed) {
                    break;
                }
                if let Ok(line) = line {
                    let trimmed = line.trim().to_string();
                    if trimmed.contains("WARNING") || trimmed.contains("Warning") {
                        warnings_clone.lock().unwrap().push(trimmed.clone());
                    }
                    if trimmed.contains("ERROR") || trimmed.contains("Error") {
                        errors_clone.lock().unwrap().push(trimmed.clone());
                    }
                }
            }
        });

        // 在主线程中读取 stdout 并解析进度
        let mut total_steps: Option<usize> = None;
        let mut current_step: usize = 0;
        let mut total_iterations: Option<usize> = None;
        let mut current_iteration: usize = 0;

        let stdout_reader = BufReader::new(stdout);
        for line in stdout_reader.lines() {
            // 检查取消标志
            if cancel_flag.load(Ordering::Relaxed) {
                tracing::info!("Solver cancelled by user");
                let _ = child.kill();
                let _ = child.wait();
                on_event(SolverEvent::Cancelled);
                return Err(SolverError::Cancelled);
            }

            if let Ok(line) = line {
                let trimmed = line.trim().to_string();

                // 跳过空行
                if trimmed.is_empty() {
                    continue;
                }

                // 判断日志级别
                let level = if trimmed.contains("ERROR") || trimmed.contains("Error") {
                    "error".to_string()
                } else if trimmed.contains("WARNING") || trimmed.contains("Warning") {
                    "warning".to_string()
                } else {
                    "info".to_string()
                };

                // 推送日志事件
                on_event(SolverEvent::Log {
                    level: level.clone(),
                    message: trimmed.clone(),
                });

                // 解析 CalculiX 输出中的进度信息
                // 解析 "STEP" 行 -> 增量步进度
                if trimmed.contains("STEP") && trimmed.contains("INC") {
                    // 格式示例: "STEP   1     INC   1   TIME   0.1000E+00"
                    current_step += 1;
                    if total_steps.is_none() {
                        // 尝试从 INP 文件名推断总步数（简化处理）
                        total_steps = Some(10); // 默认估计
                    }
                    let steps = total_steps.unwrap_or(10);
                    let percent = ((current_step as f64) / (steps as f64) * 100.0).min(99.0);
                    on_event(SolverEvent::Progress {
                        percent,
                        message: format!("增量步 {}/{}", current_step, steps),
                    });
                }

                // 解析 "ITERATION" 行 -> 迭代进度
                if trimmed.contains("ITERATION") {
                    current_iteration += 1;
                    if total_iterations.is_none() {
                        total_iterations = Some(20); // 默认估计
                    }
                    let iters = total_iterations.unwrap_or(20);
                    let iter_percent = ((current_iteration as f64) / (iters as f64) * 100.0).min(99.0);
                    // 迭代进度在当前步内占较小比例
                    let step_base = ((current_step as f64) / (total_steps.unwrap_or(10) as f64)) * 100.0;
                    let percent = (step_base + iter_percent / (total_steps.unwrap_or(10) as f64)).min(99.0);
                    on_event(SolverEvent::Progress {
                        percent,
                        message: format!("增量步 {} - 迭代 {}", current_step, current_iteration),
                    });
                }

                // 解析 "TIME" 行 -> 时间步进度
                if trimmed.contains("TIME") && trimmed.contains("STEP") {
                    // 格式示例: "TIME   0.5000E+00  STEP   5"
                    on_event(SolverEvent::Progress {
                        percent: 50.0, // 中间状态
                        message: format!("时间步: {}", trimmed),
                    });
                }

                // 检测收敛信息
                if trimmed.contains("CONVERGENCE") {
                    on_event(SolverEvent::Progress {
                        percent: 95.0,
                        message: "收敛中...".to_string(),
                    });
                }
            }
        }

        // 等待 stderr 线程完成
        let _ = stderr_thread.join();

        // 等待进程结束
        let status = child.wait()
            .map_err(|e| SolverError::ExecutionFailed(format!("Failed to wait for process: {}", e)))?;

        let elapsed = start_time.elapsed().as_secs_f64();

        let warnings_vec = warnings.lock().unwrap().clone();
        let errors_vec = errors.lock().unwrap().clone();

        let success = status.success() && errors_vec.is_empty();

        tracing::info!("CalculiX (with progress) finished in {:.2}s, success: {}", elapsed, success);

        // 推送完成事件
        on_event(SolverEvent::Completed {
            success,
            job_id: uuid::Uuid::new_v4().to_string(),
            elapsed_time_seconds: elapsed,
            warnings: warnings_vec.clone(),
            errors: errors_vec.clone(),
            output_file: output_file.to_str().map(|s| s.to_string()),
            frd_file: frd_file.to_str().map(|s| s.to_string()),
        });

        Ok(SolverResult {
            success,
            job_id: uuid::Uuid::new_v4().to_string(),
            elapsed_time_seconds: elapsed,
            warnings: warnings_vec,
            errors: errors_vec,
            output_file: Some(output_file),
            frd_file: Some(frd_file),
        })
    }
}