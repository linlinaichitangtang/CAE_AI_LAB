// DFT Task Management Commands - V1.7
// Provides job submission, monitoring, cancellation, and queue management for DFT calculations.

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::command;
use tracing::{info, warn};

// ============================================================================
// Data Structures
// ============================================================================

/// Configuration for submitting a DFT job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DftJobConfig {
    pub job_name: String,
    pub code: String,
    pub input_dir: String,
    pub wall_time_hours: f64,
    pub num_cores: u32,
    pub memory_gb: f64,
    pub queue_system: Option<String>,
}

/// Result of a DFT job submission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DftSubmitResult {
    pub job_id: String,
    pub status: String,
    pub submitted_at: String,
}

/// Represents a DFT task with its current state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DftTask {
    pub job_id: String,
    pub job_name: String,
    pub status: String,
    pub code: String,
    pub submitted_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub output_dir: Option<String>,
    pub error: Option<String>,
}

/// Result listing DFT tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DftTaskListResult {
    pub jobs: Vec<DftTask>,
    pub total: u32,
}

/// Result of queue configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigureQueueResult {
    pub success: bool,
}

/// Current queue status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStatus {
    pub running: u32,
    pub queued: u32,
    pub available_cores: u32,
}

// ============================================================================
// In-Memory Job Store (Mock)
// ============================================================================

/// Internal representation of a tracked job.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct JobEntry {
    job_id: String,
    job_name: String,
    code: String,
    input_dir: String,
    wall_time_hours: f64,
    num_cores: u32,
    memory_gb: f64,
    queue_system: Option<String>,
    status: String,
    submitted_at: String,
    started_at: Option<String>,
    completed_at: Option<String>,
    output_dir: Option<String>,
    error: Option<String>,
}

/// Queue configuration state.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct QueueConfig {
    queue_system: String,
    default_cores: u32,
    default_memory_gb: f64,
    max_jobs: u32,
}

/// Global state for job tracking.
#[allow(dead_code)]
pub struct DftTaskState {
    jobs: Mutex<Vec<JobEntry>>,
    queue_config: Mutex<QueueConfig>,
    job_counter: Mutex<u64>,
}

impl Default for DftTaskState {
    fn default() -> Self {
        Self {
            jobs: Mutex::new(Vec::new()),
            queue_config: Mutex::new(QueueConfig {
                queue_system: "slurm".to_string(),
                default_cores: 16,
                default_memory_gb: 64.0,
                max_jobs: 10,
            }),
            job_counter: Mutex::new(1000),
        }
    }
}

impl DftTaskState {
    /// Creates a new DFT task state instance.
    pub fn new() -> Self {
        Self::default()
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Generates a unique job ID.
#[allow(dead_code)]
fn generate_job_id(counter: &mut u64) -> String {
    *counter += 1;
    format!("dft-{:08x}", *counter)
}

/// Returns the current timestamp as an ISO 8601 string.
fn current_timestamp() -> String {
    chrono_free_timestamp()
}

/// Simple timestamp generation without chrono dependency.
fn chrono_free_timestamp() -> String {
    "2026-04-03T10:30:00Z".to_string()
}

/// Validates job configuration and returns warnings.
fn validate_job_config(config: &DftJobConfig) -> Result<(), String> {
    if config.job_name.is_empty() {
        return Err("Job name cannot be empty".to_string());
    }

    let valid_codes = ["vasp", "qe"];
    if !valid_codes.contains(&config.code.as_str()) {
        return Err(format!(
            "Invalid code '{}'. Must be one of: {}",
            config.code,
            valid_codes.join(", ")
        ));
    }

    if config.num_cores == 0 {
        return Err("Number of cores must be greater than 0".to_string());
    }

    if config.memory_gb <= 0.0 {
        return Err("Memory must be greater than 0 GB".to_string());
    }

    if config.wall_time_hours <= 0.0 {
        return Err("Wall time must be greater than 0 hours".to_string());
    }

    if config.wall_time_hours > 168.0 {
        return Err("Wall time exceeds maximum of 168 hours (7 days)".to_string());
    }

    if config.num_cores > 4096 {
        return Err("Number of cores exceeds maximum of 4096".to_string());
    }

    Ok(())
}

/// Converts a JobEntry to a DftTask for the API.
#[allow(dead_code)]
fn job_entry_to_task(entry: &JobEntry) -> DftTask {
    DftTask {
        job_id: entry.job_id.clone(),
        job_name: entry.job_name.clone(),
        status: entry.status.clone(),
        code: entry.code.clone(),
        submitted_at: entry.submitted_at.clone(),
        started_at: entry.started_at.clone(),
        completed_at: entry.completed_at.clone(),
        output_dir: entry.output_dir.clone(),
        error: entry.error.clone(),
    }
}

/// Generates a SLURM batch script.
fn build_slurm_script(config: &DftJobConfig, job_id: &str) -> String {
    let wall_time = format_wall_time(config.wall_time_hours);
    let partition = match config.queue_system.as_deref() {
        Some("gpu") | Some("GPU") => "gpu",
        Some("highmem") | Some("high_mem") => "highmem",
        Some("debug") => "debug",
        _ => "compute",
    };

    let run_command = match config.code.as_str() {
        "vasp" => {
            r#"# Load VASP module
module load vasp/6.5.0

# Run VASP in parallel
mpirun -np $SLURM_NTASKS vasp_std"#
        }
        "qe" => {
            r#"# Load Quantum ESPRESSO module
module load qe/7.3.1

# Run pw.x in parallel
mpirun -np $SLURM_NTASKS pw.x < pw.in > pw.out"#
        }
        _ => "echo 'Unknown DFT code'",
    };

    format!(
        r#"#!/bin/bash
#SBATCH --job-name={job_name}
#SBATCH --output={job_id}.out
#SBATCH --error={job_id}.err
#SBATCH --partition={partition}
#SBATCH --nodes={nodes}
#SBATCH --ntasks-per-node={tasks_per_node}
#SBATCH --cpus-per-task=1
#SBATCH --mem={memory_gb}G
#SBATCH --time={wall_time}

# ============================================================
# DFT Job: {job_name}
# Job ID: {job_id}
# Code: {code}
# Generated by DFT Module V1.7
# ============================================================

echo "============================================"
echo "Job ID: $SLURM_JOB_ID"
echo "Job Name: {job_name}"
echo "Nodes: $SLURM_NNODES"
echo "Tasks: $SLURM_NTASKS"
echo "Working Directory: $(pwd)"
echo "Start Time: $(date)"
echo "============================================"

cd {input_dir}

{run_command}

echo "============================================"
echo "End Time: $(date)"
echo "Job completed"
echo "============================================"
"#,
        job_name = config.job_name,
        job_id = job_id,
        partition = partition,
        nodes = (config.num_cores as f64 / 32.0).ceil().max(1.0) as u32,
        tasks_per_node = config.num_cores.min(32),
        memory_gb = config.memory_gb as u32,
        wall_time = wall_time,
        code = config.code,
        input_dir = config.input_dir,
    )
}

/// Generates a PBS batch script.
fn build_pbs_script(config: &DftJobConfig, job_id: &str) -> String {
    let wall_time = format_wall_time(config.wall_time_hours);
    let queue = match config.queue_system.as_deref() {
        Some("gpu") | Some("GPU") => "gpu",
        Some("highmem") | Some("high_mem") => "highmem",
        Some("debug") => "debug",
        _ => "batch",
    };

    let run_command = match config.code.as_str() {
        "vasp" => {
            r#"# Load VASP module
module load vasp/6.5.0

# Run VASP in parallel
mpirun -np $PBS_NP vasp_std"#
        }
        "qe" => {
            r#"# Load Quantum ESPRESSO module
module load qe/7.3.1

# Run pw.x in parallel
mpirun -np $PBS_NP pw.x < pw.in > pw.out"#
        }
        _ => "echo 'Unknown DFT code'",
    };

    format!(
        r#"#!/bin/bash
#PBS -N {job_name}
#PBS -o {job_id}.out
#PBS -e {job_id}.err
#PBS -q {queue}
#PBS -l nodes={nodes}:ppn={ppn}
#PBS -l mem={memory_gb}gb
#PBS -l walltime={wall_time}

# ============================================================
# DFT Job: {job_name}
# Job ID: {job_id}
# Code: {code}
# Generated by DFT Module V1.7
# ============================================================

echo "============================================"
echo "Job ID: $PBS_JOBID"
echo "Job Name: {job_name}"
echo "Nodes: $(cat $PBS_NODEFILE | wc -l)"
echo "Working Directory: $(pwd)"
echo "Start Time: $(date)"
echo "============================================"

cd {input_dir}

{run_command}

echo "============================================"
echo "End Time: $(date)"
echo "Job completed"
echo "============================================"
"#,
        job_name = config.job_name,
        job_id = job_id,
        queue = queue,
        nodes = (config.num_cores as f64 / 32.0).ceil().max(1.0) as u32,
        ppn = config.num_cores.min(32),
        memory_gb = config.memory_gb as u32,
        wall_time = wall_time,
        code = config.code,
        input_dir = config.input_dir,
    )
}

/// Formats wall time in hours to HH:MM:SS string.
fn format_wall_time(hours: f64) -> String {
    let total_seconds = (hours * 3600.0) as u64;
    let h = total_seconds / 3600;
    let m = (total_seconds % 3600) / 60;
    let s = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Submits a DFT job for execution.
#[command]
pub fn submit_dft_job(config: DftJobConfig) -> Result<DftSubmitResult, String> {
    info!(job_name = %config.job_name, code = %config.code,
          cores = config.num_cores, "Submitting DFT job");

    validate_job_config(&config)?;

    // In a real implementation, this would interact with the queue system.
    // Here we generate a job ID and track it in memory.
    let job_id = format!("dft-{:08x}", 1001 + rand_mock());

    let submitted_at = current_timestamp();

    info!(job_id = %job_id, "DFT job submitted successfully");

    Ok(DftSubmitResult {
        job_id,
        status: "queued".to_string(),
        submitted_at,
    })
}

/// Cancels a running or queued DFT job.
#[command]
pub fn cancel_dft_job(job_id: String) -> Result<bool, String> {
    info!(job_id = %job_id, "Cancelling DFT job");

    if job_id.is_empty() {
        return Err("Job ID cannot be empty".to_string());
    }

    // In a real implementation, this would send a cancellation signal to the queue system.
    // Here we simulate the cancellation.
    let cancelled = !job_id.contains("invalid");

    if cancelled {
        info!(job_id = %job_id, "DFT job cancelled successfully");
    } else {
        warn!(job_id = %job_id, "DFT job not found or cannot be cancelled");
    }

    Ok(cancelled)
}

/// Gets the current status of a DFT job.
#[command]
pub fn get_dft_job_status(job_id: String) -> Result<DftTask, String> {
    info!(job_id = %job_id, "Getting DFT job status");

    if job_id.is_empty() {
        return Err("Job ID cannot be empty".to_string());
    }

    // In a real implementation, this would query the queue system.
    // Here we return mock data based on the job ID.
    let task = DftTask {
        job_id: job_id.clone(),
        job_name: format!("DFT Calculation {}", &job_id[job_id.len().saturating_sub(4)..].trim_start_matches('0')),
        status: "running".to_string(),
        code: "vasp".to_string(),
        submitted_at: "2026-04-03T08:00:00Z".to_string(),
        started_at: Some("2026-04-03T08:05:23Z".to_string()),
        completed_at: None,
        output_dir: Some(format!("/scratch/dft/{}/{}", &job_id[..4], job_id)),
        error: None,
    };

    info!(job_id = %job_id, status = %task.status, "Job status retrieved");
    Ok(task)
}

/// Lists DFT jobs with optional status filter.
#[command]
pub fn get_dft_job_list(status_filter: Option<String>) -> Result<DftTaskListResult, String> {
    info!(filter = ?status_filter, "Listing DFT jobs");

    // In a real implementation, this would query the job database.
    // Here we return mock data.
    let all_jobs = vec![
        DftTask {
            job_id: "dft-000003e8".to_string(),
            job_name: "Si_SCF_convergence".to_string(),
            status: "completed".to_string(),
            code: "vasp".to_string(),
            submitted_at: "2026-04-02T14:30:00Z".to_string(),
            started_at: Some("2026-04-02T14:30:15Z".to_string()),
            completed_at: Some("2026-04-02T14:45:32Z".to_string()),
            output_dir: Some("/scratch/dft/Si_SCF_convergence".to_string()),
            error: None,
        },
        DftTask {
            job_id: "dft-000003e9".to_string(),
            job_name: "Fe_relax_FCC".to_string(),
            status: "running".to_string(),
            code: "vasp".to_string(),
            submitted_at: "2026-04-03T08:00:00Z".to_string(),
            started_at: Some("2026-04-03T08:05:23Z".to_string()),
            completed_at: None,
            output_dir: Some("/scratch/dft/Fe_relax_FCC".to_string()),
            error: None,
        },
        DftTask {
            job_id: "dft-000003ea".to_string(),
            job_name: "Al_bands_Gamma".to_string(),
            status: "queued".to_string(),
            code: "qe".to_string(),
            submitted_at: "2026-04-03T09:15:00Z".to_string(),
            started_at: None,
            completed_at: None,
            output_dir: None,
            error: None,
        },
        DftTask {
            job_id: "dft-000003eb".to_string(),
            job_name: "Cu_dos_analysis".to_string(),
            status: "queued".to_string(),
            code: "vasp".to_string(),
            submitted_at: "2026-04-03T09:20:00Z".to_string(),
            started_at: None,
            completed_at: None,
            output_dir: None,
            error: None,
        },
        DftTask {
            job_id: "dft-000003ec".to_string(),
            job_name: "TiO2_relax_rutile".to_string(),
            status: "failed".to_string(),
            code: "vasp".to_string(),
            submitted_at: "2026-04-03T07:00:00Z".to_string(),
            started_at: Some("2026-04-03T07:00:12Z".to_string()),
            completed_at: Some("2026-04-03T07:02:45Z".to_string()),
            output_dir: Some("/scratch/dft/TiO2_relax_rutile".to_string()),
            error: Some("EDDDAV: Call to ZHEGV failed. Returncode = 1".to_string()),
        },
    ];

    let filtered: Vec<DftTask> = if let Some(ref filter) = status_filter {
        all_jobs
            .into_iter()
            .filter(|j| j.status == *filter)
            .collect()
    } else {
        all_jobs
    };

    let total = filtered.len() as u32;

    info!(total = total, "Job list returned");
    Ok(DftTaskListResult {
        jobs: filtered,
        total,
    })
}

/// Generates a SLURM batch script for the given job configuration.
#[command]
pub fn generate_slurm_script(config: DftJobConfig) -> Result<String, String> {
    info!(job_name = %config.job_name, "Generating SLURM script");

    validate_job_config(&config)?;

    let job_id = format!("dft-{:08x}", 2001);
    let script = build_slurm_script(&config, &job_id);

    info!("SLURM script generated successfully");
    Ok(script)
}

/// Generates a PBS batch script for the given job configuration.
#[command]
pub fn generate_pbs_script(config: DftJobConfig) -> Result<String, String> {
    info!(job_name = %config.job_name, "Generating PBS script");

    validate_job_config(&config)?;

    let job_id = format!("dft-{:08x}", 2001);
    let script = build_pbs_script(&config, &job_id);

    info!("PBS script generated successfully");
    Ok(script)
}

/// Configures the DFT queue system settings.
#[command]
pub fn configure_dft_queue(
    queue_system: String,
    default_cores: u32,
    default_memory_gb: f64,
    max_jobs: u32,
) -> Result<ConfigureQueueResult, String> {
    info!(
        queue_system = %queue_system,
        default_cores = default_cores,
        default_memory_gb = default_memory_gb,
        max_jobs = max_jobs,
        "Configuring DFT queue"
    );

    let valid_systems = ["slurm", "pbs", "sge", "local"];
    if !valid_systems.contains(&queue_system.as_str()) {
        return Err(format!(
            "Invalid queue system '{}'. Must be one of: {}",
            queue_system,
            valid_systems.join(", ")
        ));
    }

    if default_cores == 0 {
        return Err("Default cores must be greater than 0".to_string());
    }

    if default_memory_gb <= 0.0 {
        return Err("Default memory must be greater than 0 GB".to_string());
    }

    if max_jobs == 0 {
        return Err("Max jobs must be greater than 0".to_string());
    }

    // In a real implementation, this would persist the configuration.
    info!("Queue configured successfully: {} with {} cores, {:.1} GB, max {} jobs",
          queue_system, default_cores, default_memory_gb, max_jobs);

    Ok(ConfigureQueueResult { success: true })
}

/// Gets the current status of the DFT job queue.
#[command]
pub fn get_queue_status() -> Result<QueueStatus, String> {
    info!("Getting queue status");

    // In a real implementation, this would query the actual queue system.
    let status = QueueStatus {
        running: 3,
        queued: 5,
        available_cores: 45,
    };

    info!(
        running = status.running,
        queued = status.queued,
        available = status.available_cores,
        "Queue status retrieved"
    );

    Ok(status)
}

/// Runs a Quantum ESPRESSO calculation locally (V2.3-003).
/// Executes `mpirun --allow-run-as-root -np {cores} --host localhost pw.x -in {input_file}`
/// and returns the output content.
#[command]
pub fn run_qe_local(
    input_file: String,
    output_file: String,
    num_cores: u32,
    work_dir: String,
) -> Result<QeLocalResult, String> {
    use std::process::Command;
    use std::path::Path;

    info!(
        input = %input_file,
        output = %output_file,
        cores = num_cores,
        work_dir = %work_dir,
        "Running QE locally"
    );

    if !Path::new(&input_file).exists() {
        return Err(format!("Input file not found: {}", input_file));
    }

    if num_cores == 0 {
        return Err("Number of cores must be > 0".to_string());
    }

    let out_path = Path::new(&output_file);
    let out_file = std::fs::File::create(out_path)
        .map_err(|e| format!("Cannot create output file: {}", e))?;

    let result = Command::new("mpirun")
        .args([
            "--allow-run-as-root",
            "-np", &num_cores.to_string(),
            "--host", "localhost",
            "pw.x",
            "-in", &input_file,
        ])
        .current_dir(&work_dir)
        .env("OMP_NUM_THREADS", "1")
        .stdout(std::process::Stdio::from(out_file))
        .stderr(std::process::Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to execute pw.x: {}", e))?;

    let stderr = String::from_utf8_lossy(&result.stderr).to_string();
    let success = result.status.success();

    // Read output file
    let output_content = std::fs::read_to_string(&output_file)
        .unwrap_or_default();

    let status = if success { "completed" } else { "failed" };

    info!(
        status = status,
        exit_code = ?result.status.code(),
        output_len = output_content.len(),
        "QE local execution finished"
    );

    Ok(QeLocalResult {
        success,
        status: status.to_string(),
        exit_code: result.status.code().unwrap_or(-1),
        output_file: output_file.clone(),
        output_length: output_content.len(),
        error_message: if success { None } else { Some(stderr) },
    })
}

/// Result of a local QE execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QeLocalResult {
    pub success: bool,
    pub status: String,
    pub exit_code: i32,
    pub output_file: String,
    pub output_length: usize,
    pub error_message: Option<String>,
}

// ============================================================================
// Mock Helpers
// ============================================================================

/// Simple mock random number generator for job IDs.
fn rand_mock() -> u64 {
    // Deterministic mock value for reproducibility
    42
}
