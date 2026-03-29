//! CalculiX solver integration module
//! Handles external process execution for CAE solver

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

use super::mesh::{GridConfig, MeshElementType, MeshGenerator, StructuredMesh};

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
            executable_path: "ccx".to_string(), // assumes ccx in PATH
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
}

pub struct CalculiXSolver {
    config: SolverConfig,
}

impl CalculiXSolver {
    pub fn new(config: SolverConfig) -> Self {
        Self { config }
    }

    /// Check if CalculiX is available
    pub fn check_availability(&self) -> Result<bool, SolverError> {
        let output = Command::new(&self.config.executable_path)
            .arg("-v")
            .output()
            .map_err(|e| SolverError::ExecutableNotFound(PathBuf::from(&self.config.executable_path)))?;
        
        Ok(output.status.success())
    }

    /// Run the solver on an input file
    pub fn solve(&self, input_file: &PathBuf, working_dir: &PathBuf) -> Result<SolverResult, SolverError> {
        let input_file_name = input_file
            .file_stem()
            .ok_or_else(|| SolverError::InputFileError("Invalid input file name".to_string()))?
            .to_str()
            .ok_or_else(|| SolverError::InputFileError("Invalid input file name encoding".to_string()))?;

        let output_file = working_dir.join(format!("{}.dat", input_file_name));
        let frd_file = working_dir.join(format!("{}.frd", input_file_name));
        
        // Build command args
        let mut args = vec![];
        
        // Set number of threads if specified
        if self.config.num_threads > 0 {
            args.push("-nt".to_string());
            args.push(self.config.num_threads.to_string());
        }
        
        // Memory limit if specified
        if let Some(mem_mb) = self.config.memory_limit_mb {
            args.push("-memory".to_string());
            args.push(format!("{}MB", mem_mb));
        }
        
        // Input file (without extension)
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

        // Parse warnings and errors
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

        let success = output.status.success() && !errors.is_empty() == false;

        tracing::info!("CalculiX finished in {:.2}s, success: {}", elapsed, success);

        Ok(SolverResult {
            success,
            job_id: uuid::Uuid::new_v4().to_string(),
            elapsed_time_seconds: elapsed,
            warnings,
            errors,
            output_file: Some(output_file),
        })
    }
    /// Generate a 2D rectangular mesh
    pub fn generate_2d_mesh(
        nx: usize,
        ny: usize,
        size_x: f64,
        size_y: f64,
    ) -> Result<StructuredMesh, mesh::MeshError> {
        let config = GridConfig::new_2d(nx, ny, size_x, size_y);
        let generator = MeshGenerator::new(config);
        generator.generate_2d_rect()
    }

    /// Generate a 3D rectangular box mesh
    pub fn generate_3d_mesh(
        nx: usize,
        ny: usize,
        nz: usize,
        size_x: f64,
        size_y: f64,
        size_z: f64,
    ) -> Result<StructuredMesh, mesh::MeshError> {
        let config = GridConfig::new_3d(nx, ny, nz, size_x, size_y, size_z);
        let generator = MeshGenerator::new(config);
        generator.generate_3d_box()
    }

    /// Generate a mesh with custom element type
    pub fn generate_structured_mesh(
        config: GridConfig,
    ) -> Result<StructuredMesh, mesh::MeshError> {
        let generator = MeshGenerator::new(config);
        generator.generate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver_config_default() {
        let config = SolverConfig::default();
        assert_eq!(config.executable_path, "ccx");
        assert_eq!(config.num_threads, 4);
    }

    #[test]
    fn test_2d_mesh_generation() {
        let mesh = CalculiXSolver::generate_2d_mesh(2, 2, 1.0, 1.0).unwrap();
        assert_eq!(mesh.num_nodes, 9);
        assert_eq!(mesh.num_elements, 4);
    }

    #[test]
    fn test_3d_mesh_generation() {
        let mesh = CalculiXSolver::generate_3d_mesh(2, 2, 2, 1.0, 1.0, 1.0).unwrap();
        assert_eq!(mesh.num_nodes, 27);
        assert_eq!(mesh.num_elements, 8);
    }
}