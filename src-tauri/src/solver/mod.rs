//! Solver module
//! Contains mesh generation and solver utilities

pub mod mesh;
pub mod input_gen;
pub mod output_parser;
pub mod bc;

pub use mesh::*;
pub use input_gen::*;
pub use output_parser::*;
pub use bc::*;
pub use bc::helpers::*;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
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

        let stdout = String::from_utf8_lossy(&output.stdout);
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
}