//! CalculiX FE solver integration (V2.3-011~015).
//!
//! Provides commands for submitting CalculiX jobs, polling status,
//! and parsing .dat/.frd output files.
//!
//! ## CalculiX 2.20+ Installation (Ubuntu 22.04)
//!
//! The apt version (2.17) has a known *END parsing bug (KI-001).
//! To install CalculiX 2.20+ from source:
//!
//! ```bash
//! sudo apt-get install -y libarpack2-dev libspooles-dev liblapack-dev gfortran
//! wget https://www.dhondt.de/ccx_2.20.src.tar.bz2
//! tar xjf ccx_2.20.src.tar.bz2
//! cd CalculiX/cgx_2.20/src
//! # (optional) edit Makefile for your system
//! make -j$(nproc)
//! sudo cp ccx /usr/local/bin/ccx_2.20
//! sudo ln -sf /usr/local/bin/ccx_2.20 /usr/local/bin/ccx
//! ```

use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::command;
use tracing::info;

/// Submits a CalculiX job for execution (V2.3-013).
///
/// Runs `ccx <input_basename>` in the specified work directory.
/// The input file should be a .inp file; only the basename (without extension) is passed to ccx.
#[command]
pub fn submit_calculix_job(
    input_file: String,
    work_dir: String,
) -> Result<CalculiXJobResult, String> {
    use std::process::Command;

    info!(input = %input_file, work_dir = %work_dir, "Submitting CalculiX job");

    let input_path = Path::new(&input_file);
    if !input_path.exists() {
        return Err(format!("Input file not found: {}", input_file));
    }

    // Extract basename without extension for ccx
    let stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Cannot extract input file stem")?;

    // Check CalculiX version
    let version_check = Command::new("ccx")
        .arg("--version")
        .output();

    let version_warning = match &version_check {
        Ok(output) => {
            let info = String::from_utf8_lossy(&output.stdout).to_string()
                + &String::from_utf8_lossy(&output.stderr).to_string();
            if info.contains("2.17") || info.contains("2.18") || info.contains("2.19") {
                Some("WARNING: CalculiX version < 2.20 may have *END parsing bug (KI-001)".to_string())
            } else {
                None
            }
        }
        Err(_) => Some("WARNING: Could not determine CalculiX version".to_string()),
    };

    let job_id = uuid::Uuid::new_v4().to_string();
    let dat_file = Path::new(&work_dir).join(format!("{}.dat", stem));
    let frd_file = Path::new(&work_dir).join(format!("{}.frd", stem));
    let out_file = Path::new(&work_dir).join(format!("{}.out", stem));

    let out_fd = std::fs::File::create(&out_file)
        .map_err(|e| format!("Cannot create output file: {}", e))?;

    let result = Command::new("ccx")
        .arg(stem)
        .current_dir(&work_dir)
        .stdout(std::process::Stdio::from(out_fd))
        .stderr(std::process::Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to execute ccx: {}", e))?;

    let stderr = String::from_utf8_lossy(&result.stderr).to_string();
    let success = result.status.success();
    let dat_exists = dat_file.exists();
    let frd_exists = frd_file.exists();

    info!(
        job_id = %job_id,
        success = success,
        dat_exists = dat_exists,
        frd_exists = frd_exists,
        "CalculiX job completed"
    );

    Ok(CalculiXJobResult {
        job_id,
        success,
        exit_code: result.status.code().unwrap_or(-1),
        dat_file: dat_file.to_string_lossy().to_string(),
        frd_file: frd_file.to_string_lossy().to_string(),
        out_file: out_file.to_string_lossy().to_string(),
        dat_exists,
        frd_exists,
        error_message: if success { version_warning } else { Some(stderr) },
    })
}

/// Gets the status of a CalculiX job (V2.3-013).
///
/// Checks if .dat and .frd files exist and estimates progress.
#[command]
pub fn get_calculix_job_status(
    job_id: String,
    work_dir: String,
    input_basename: String,
) -> Result<CalculiXJobStatus, String> {
    info!(job_id = %job_id, "Checking CalculiX job status");

    let dat_path = Path::new(&work_dir).join(format!("{}.dat", input_basename));
    let frd_path = Path::new(&work_dir).join(format!("{}.frd", input_basename));
    let sta_path = Path::new(&work_dir).join(format!("{}.sta", input_basename));
    let out_path = Path::new(&work_dir).join(format!("{}.out", input_basename));

    let dat_exists = dat_path.exists();
    let frd_exists = frd_path.exists();
    let sta_exists = sta_path.exists();

    // Estimate progress from .dat file size
    let progress = if dat_exists {
        let size = std::fs::metadata(&dat_path).map(|m| m.len()).unwrap_or(0);
        // Heuristic: a typical .dat file is 1-100KB
        if size > 0 { Some((size.min(100000) as f64 / 100000.0 * 100.0) as u32) } else { Some(0) }
    } else {
        Some(0)
    };

    let status = if frd_exists {
        "completed".to_string()
    } else if dat_exists {
        "running".to_string()
    } else if out_path.exists() {
        "failed".to_string()
    } else {
        "pending".to_string()
    };

    Ok(CalculiXJobStatus {
        job_id,
        status,
        progress,
        dat_exists,
        frd_exists,
        sta_exists,
    })
}

/// Parses CalculiX .dat output file (V2.3-014).
///
/// Extracts max displacement, max stress, and endpoint displacement.
/// CCX .dat format sections:
///   - "displacements (vx,vy,vz) ..." followed by 4-col lines: node_id vx vy vz
///   - "stresses (elem, integ.pnt.,sxx,syy,szz,sxy,sxz,syz) ..." followed by 8-col lines
#[command]
pub fn parse_calculix_dat(dat_file: String) -> Result<CalculiXResult, String> {
    info!(file = %dat_file, "Parsing CalculiX .dat file");

    let content = std::fs::read_to_string(&dat_file)
        .map_err(|e| format!("Cannot read .dat file: {}", e))?;

    let mut max_displacement = 0.0f64;
    let mut max_stress = 0.0f64;
    let mut endpoint_displacement = 0.0f64;
    let mut nodes: u32 = 0;
    let mut elements: u32 = 0;

    // Track which section we're in: "disp", "stress", or None
    let mut section: Option<&str> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        // Detect section headers
        if trimmed.contains("displacements") {
            section = Some("disp");
            continue;
        }
        if trimmed.contains("stresses") {
            section = Some("stress");
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();

        match section {
            Some("disp") => {
                // Format: node_id vx vy vz (4 columns)
                if parts.len() == 4 {
                    if let (Ok(_nid), Ok(dx), Ok(dy), Ok(dz)) = (
                        parts[0].parse::<f64>(),
                        parts[1].parse::<f64>(),
                        parts[2].parse::<f64>(),
                        parts[3].parse::<f64>(),
                    ) {
                        let mag = (dx * dx + dy * dy + dz * dz).sqrt();
                        if mag > max_displacement {
                            max_displacement = mag;
                        }
                        endpoint_displacement = dy.abs();
                    } else {
                        section = None;
                    }
                } else {
                    section = None;
                }
            }
            Some("stress") => {
                // Format: elem_id integ_pnt sxx syy szz sxy sxz syz (8 columns)
                if parts.len() >= 8 {
                    if let (Ok(sxx), Ok(syy), Ok(szz), Ok(sxy), Ok(sxz), Ok(syz)) = (
                        parts[2].parse::<f64>(),
                        parts[3].parse::<f64>(),
                        parts[4].parse::<f64>(),
                        parts[5].parse::<f64>(),
                        parts[6].parse::<f64>(),
                        parts[7].parse::<f64>(),
                    ) {
                        let sxx = sxx.abs();
                        let syy = syy.abs();
                        let szz = szz.abs();
                        let sxy = sxy.abs();
                        let sxz = sxz.abs();
                        let syz = syz.abs();
                        // von Mises stress
                        let vm = (0.5 * ((sxx - syy).powi(2)
                            + (syy - szz).powi(2)
                            + (szz - sxx).powi(2)
                            + 6.0 * (sxy * sxy + sxz * sxz + syz * syz)))
                            .sqrt();
                        if vm > max_stress {
                            max_stress = vm;
                        }
                    } else {
                        section = None;
                    }
                } else {
                    section = None;
                }
            }
            _ => {
                // Parse node/element counts from header
                if trimmed.starts_with("number of nodes") {
                    if let Some(idx) = trimmed.find(':') {
                        if let Ok(n) = trimmed[idx + 1..].trim().parse::<u32>() {
                            nodes = n;
                        }
                    }
                }
                if trimmed.starts_with("number of elements") {
                    if let Some(idx) = trimmed.find(':') {
                        if let Ok(n) = trimmed[idx + 1..].trim().parse::<u32>() {
                            elements = n;
                        }
                    }
                }
            }
        }
    }

    info!(
        max_disp = max_displacement,
        max_stress = max_stress,
        endpoint_disp = endpoint_displacement,
        nodes = nodes,
        elements = elements,
        "CalculiX .dat parsed"
    );

    Ok(CalculiXResult {
        max_displacement,
        max_stress,
        endpoint_displacement,
        nodes,
        elements,
        dat_file,
    })
}

/// Parses CalculiX .frd results file (V2.3-014).
///
/// Extracts node and element counts from FRD header.
#[command]
pub fn parse_calculix_frd(frd_file: String) -> Result<CalculiXFrdResult, String> {
    info!(file = %frd_file, "Parsing CalculiX .frd file");

    let content = std::fs::read_to_string(&frd_file)
        .map_err(|e| format!("Cannot read .frd file: {}", e))?;

    let mut nodes: u32 = 0;
    let mut elements: u32 = 0;
    let mut block_count = 0;

    for line in content.lines() {
        let trimmed = line.trim();
        // FRD format: -3 = node count, -4 = element count
        if trimmed == "-3" {
            block_count += 1;
            continue;
        }
        if block_count == 1 && trimmed.parse::<u32>().is_ok() {
            nodes = trimmed.parse::<u32>().unwrap_or(0);
            block_count = 0;
        }
        if trimmed == "-4" {
            block_count += 1;
            continue;
        }
        if block_count == 1 && trimmed.parse::<u32>().is_ok() {
            elements = trimmed.parse::<u32>().unwrap_or(0);
            block_count = 0;
        }
    }

    info!(nodes = nodes, elements = elements, "CalculiX .frd parsed");

    Ok(CalculiXFrdResult {
        frd_file,
        nodes,
        elements,
    })
}

// ===== Data Structures =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculiXJobResult {
    pub job_id: String,
    pub success: bool,
    pub exit_code: i32,
    pub dat_file: String,
    pub frd_file: String,
    pub out_file: String,
    pub dat_exists: bool,
    pub frd_exists: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculiXJobStatus {
    pub job_id: String,
    pub status: String,
    pub progress: Option<u32>,
    pub dat_exists: bool,
    pub frd_exists: bool,
    pub sta_exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculiXResult {
    pub max_displacement: f64,
    pub max_stress: f64,
    pub endpoint_displacement: f64,
    pub nodes: u32,
    pub elements: u32,
    pub dat_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculiXFrdResult {
    pub frd_file: String,
    pub nodes: u32,
    pub elements: u32,
}
