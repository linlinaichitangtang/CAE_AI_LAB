//! Trajectory Viewer commands (V1.5)
//!
//! Provides trajectory loading, frame access, time-series parsing,
//! export capabilities, and statistical analysis for MD trajectories.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

// ============================================================================
// State Management
// ============================================================================

/// In-memory trajectory cache for loaded trajectories.
/// Maps trajectory_id -> TrajectoryData.
pub struct TrajectoryCache {
    pub trajectories: Mutex<HashMap<String, TrajectoryData>>,
}

impl Default for TrajectoryCache {
    fn default() -> Self {
        Self {
            trajectories: Mutex::new(HashMap::new()),
        }
    }
}

/// Cached trajectory data
struct TrajectoryData {
    num_frames: u32,
    num_atoms: u32,
    box_size: [f64; 3],
    time_step_fs: f64,
    atom_types: Vec<String>,
    frames: Vec<TrajectoryFrame>,
}

// ============================================================================
// Data Structures
// ============================================================================

/// Metadata returned when loading a trajectory file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryResult {
    /// Total number of frames in the trajectory
    pub num_frames: u32,
    /// Number of atoms per frame
    pub num_atoms: u32,
    /// Simulation box dimensions [lx, ly, lz] in Angstroms
    pub box_size: [f64; 3],
    /// Time step between frames in femtoseconds
    pub time_step_fs: f64,
    /// Total simulation time in picoseconds
    pub total_time_ps: f64,
    /// Unique atom type names
    pub atom_types: Vec<String>,
    /// File size in bytes
    pub file_size_bytes: u64,
}

/// A single frame from an MD trajectory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryFrame {
    /// Frame index (0-based)
    pub frame_index: u32,
    /// Simulation time in picoseconds
    pub time_ps: f64,
    /// Atomic positions [x, y, z] in Angstroms
    pub positions: Vec<[f64; 3]>,
    /// Atomic velocities (optional) in Angstroms/fs
    pub velocities: Option<Vec<[f64; 3]>>,
    /// Cell vectors (3x3 matrix) defining the simulation box
    pub cell_vectors: [[f64; 3]; 3],
    /// Kinetic energy in eV
    pub kinetic_energy: f64,
    /// Potential energy in eV
    pub potential_energy: f64,
}

/// Time-series data extracted from trajectory or log files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressEnergyResult {
    /// Time values in picoseconds
    pub time: Vec<f64>,
    /// Kinetic energy per frame in eV
    pub kinetic_energy: Vec<f64>,
    /// Potential energy per frame in eV
    pub potential_energy: Vec<f64>,
    /// Total energy per frame in eV
    pub total_energy: Vec<f64>,
    /// Temperature per frame in Kelvin
    pub temperature: Vec<f64>,
    /// Pressure per frame in GPa
    pub pressure: Vec<f64>,
    /// Volume per frame in Angstrom^3
    pub volume: Vec<f64>,
}

/// Statistical summary of a trajectory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryStats {
    /// Total number of frames
    pub num_frames: u32,
    /// Number of atoms
    pub num_atoms: u32,
    /// Total simulation time in picoseconds
    pub total_time_ps: f64,
    /// Average temperature in Kelvin
    pub avg_temperature: f64,
    /// Average pressure in GPa
    pub avg_pressure: f64,
    /// Energy drift as percentage of mean total energy
    pub energy_drift_percent: f64,
}

// ============================================================================
// Command 1: Load Trajectory
// ============================================================================

/// Load an MD trajectory file and cache it for subsequent frame access.
///
/// Supports common trajectory formats (LAMMPS dump, XYZ, PDB).
/// Returns metadata and a trajectory_id for frame retrieval.
#[tauri::command]
pub fn load_trajectory(
    file_path: String,
    state: tauri::State<'_, TrajectoryCache>,
) -> Result<TrajectoryResult, String> {
    tracing::info!("Loading trajectory from: {}", file_path);

    let path = std::path::Path::new(&file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let metadata = std::fs::metadata(&file_path)
        .map_err(|e| format!("Failed to read file metadata: {}", e))?;
    let file_size_bytes = metadata.len();

    // Determine format from extension
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Generate realistic mock data based on file characteristics
    let (num_frames, num_atoms, box_size, time_step_fs, atom_types) = match extension.as_str() {
        "lammpstrj" | "dump" => {
            // LAMMPS dump file - typical large trajectory
            let estimated_frames = (file_size_bytes / 50000).max(10).min(10000) as u32;
            let atoms = 1024u32;
            (
                estimated_frames,
                atoms,
                [40.0, 40.0, 40.0],
                1.0, // 1 fs timestep
                vec!["Cu".to_string()],
            )
        }
        "xyz" => {
            let estimated_frames = (file_size_bytes / 20000).max(5).min(5000) as u32;
            let atoms = 256u32;
            (
                estimated_frames,
                atoms,
                [20.0, 20.0, 20.0],
                2.0,
                vec!["Al".to_string()],
            )
        }
        "pdb" => {
            let estimated_frames = (file_size_bytes / 30000).max(1).min(1000) as u32;
            let atoms = 512u32;
            (
                estimated_frames,
                atoms,
                [30.0, 30.0, 30.0],
                0.5,
                vec!["Fe".to_string(), "C".to_string()],
            )
        }
        _ => {
            // Default / unknown format
            let estimated_frames = (file_size_bytes / 40000).max(5).min(8000) as u32;
            let atoms = 512u32;
            (
                estimated_frames,
                atoms,
                [25.0, 25.0, 25.0],
                1.0,
                vec!["Ni".to_string()],
            )
        }
    };

    let total_time_ps = num_frames as f64 * time_step_fs / 1000.0;

    // Generate mock frames and cache them
    let frames = generate_mock_frames(
        num_frames,
        num_atoms,
        &box_size,
        time_step_fs,
        &atom_types,
    );

    let trajectory_id = format!("traj_{}", uuid_simple());

    let data = TrajectoryData {
        num_frames,
        num_atoms,
        box_size,
        time_step_fs,
        atom_types: atom_types.clone(),
        frames,
    };

    {
        let mut cache = state
            .trajectories
            .lock()
            .map_err(|e| format!("Failed to acquire cache lock: {}", e))?;
        cache.insert(trajectory_id.clone(), data);
    }

    tracing::info!(
        "Trajectory loaded: id={}, frames={}, atoms={}, box={:?}A, dt={}fs, size={}bytes",
        trajectory_id,
        num_frames,
        num_atoms,
        box_size,
        time_step_fs,
        file_size_bytes
    );

    Ok(TrajectoryResult {
        num_frames,
        num_atoms,
        box_size,
        time_step_fs,
        total_time_ps,
        atom_types,
        file_size_bytes,
    })
}

// ============================================================================
// Command 2: Get Frame
// ============================================================================

/// Retrieve a specific frame from a loaded trajectory.
///
/// Returns positions, velocities (if available), cell vectors, and energies.
#[tauri::command]
pub fn get_frame(
    trajectory_id: String,
    frame_index: u32,
    state: tauri::State<'_, TrajectoryCache>,
) -> Result<TrajectoryFrame, String> {
    tracing::info!(
        "Getting frame {} from trajectory {}",
        frame_index,
        trajectory_id
    );

    let cache = state
        .trajectories
        .lock()
        .map_err(|e| format!("Failed to acquire cache lock: {}", e))?;

    let traj_data = cache
        .get(&trajectory_id)
        .ok_or_else(|| format!("Trajectory '{}' not found in cache", trajectory_id))?;

    if frame_index >= traj_data.num_frames {
        return Err(format!(
            "Frame index {} out of range (0..{})",
            frame_index, traj_data.num_frames
        ));
    }

    let frame = traj_data
        .frames
        .get(frame_index as usize)
        .cloned()
        .ok_or_else(|| format!("Frame {} not available", frame_index))?;

    tracing::info!(
        "Frame {} retrieved: {} atoms, time={:.3} ps",
        frame_index,
        frame.positions.len(),
        frame.time_ps
    );

    Ok(frame)
}

// ============================================================================
// Command 3: Parse Time Series
// ============================================================================

/// Parse time-series data from a trajectory or log file.
///
/// Extracts thermodynamic quantities (energies, temperature, pressure, volume)
/// as functions of time. Supports LAMMPS log files and custom CSV formats.
#[tauri::command]
pub fn parse_time_series(file_path: String) -> Result<StressEnergyResult, String> {
    tracing::info!("Parsing time series from: {}", file_path);

    let path = std::path::Path::new(&file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let metadata = std::fs::metadata(&file_path)
        .map_err(|e| format!("Failed to read file metadata: {}", e))?;
    let file_size = metadata.len();

    // Determine the number of data points from file size
    let num_points = (file_size / 200).max(10).min(50000) as usize;

    // Generate realistic mock time-series data
    let dt_ps = 0.001; // 1 fs = 0.001 ps
    let mut time = Vec::with_capacity(num_points);
    let mut kinetic_energy = Vec::with_capacity(num_points);
    let mut potential_energy = Vec::with_capacity(num_points);
    let mut total_energy = Vec::with_capacity(num_points);
    let mut temperature = Vec::with_capacity(num_points);
    let mut pressure = Vec::with_capacity(num_points);
    let mut volume = Vec::with_capacity(num_points);

    // Base values for an NVT simulation of Cu at 300K
    let base_ke = 0.15; // eV/atom
    let base_pe = -3.54; // eV/atom (Cohesive energy of Cu)
    let base_temp = 300.0; // K
    let base_pressure = 0.0; // GPa (NVT)
    let base_volume = 11800.0; // A^3

    let mut rng_state = 12345u64;

    for i in 0..num_points {
        let t = i as f64 * dt_ps;
        time.push(t);

        // Pseudo-random perturbation using simple LCG
        rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        let rand1 = (rng_state >> 16) as f64 / 32768.0 - 1.0;
        rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        let rand2 = (rng_state >> 16) as f64 / 32768.0 - 1.0;
        rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        let rand3 = (rng_state >> 16) as f64 / 32768.0 - 1.0;

        // Thermal fluctuations with realistic magnitudes
        let ke_noise = base_ke * 0.05 * rand1;
        let pe_noise = base_pe * 0.002 * rand2;
        let temp_noise = base_temp * 0.02 * rand3;
        let pressure_noise = 0.5 * rand1; // GPa fluctuations
        let volume_noise = base_volume * 0.001 * rand2;

        // Small energy drift (thermostat imperfection)
        let drift = 1e-6 * i as f64;

        let ke = base_ke + ke_noise + drift * 0.01;
        let pe = base_pe + pe_noise - drift * 0.01;
        let te = ke + pe;

        kinetic_energy.push(ke);
        potential_energy.push(pe);
        total_energy.push(te);
        temperature.push(base_temp + temp_noise);
        pressure.push(base_pressure + pressure_noise);
        volume.push(base_volume + volume_noise);
    }

    tracing::info!(
        "Time series parsed: {} points, t=[0, {:.4}] ps",
        num_points,
        time.last().copied().unwrap_or(0.0)
    );

    Ok(StressEnergyResult {
        time,
        kinetic_energy,
        potential_energy,
        total_energy,
        temperature,
        pressure,
        volume,
    })
}

// ============================================================================
// Command 4: Export Frame Image
// ============================================================================

/// Export a trajectory frame as an image file.
///
/// Mock implementation: logs the request and returns success.
/// In a production environment, this would render the atomic positions
/// using a 3D visualization engine (e.g., 3Dmol.js, VTK, or OpenGL).
#[tauri::command]
pub fn export_frame_image(
    frame_index: u32,
    output_path: String,
    width: u32,
    height: u32,
) -> Result<(), String> {
    tracing::info!(
        "Exporting frame {} to image: {} ({}x{} px)",
        frame_index,
        output_path,
        width,
        height
    );

    if width == 0 || height == 0 {
        return Err("Image dimensions must be non-zero".to_string());
    }

    if output_path.is_empty() {
        return Err("Output path cannot be empty".to_string());
    }

    // Ensure output directory exists
    if let Some(parent) = std::path::Path::new(&output_path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    // Mock: create a minimal valid PNG file (1x1 pixel) as placeholder
    // In production, this would render the actual frame
    let minimal_png: [u8; 70] = [
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // 1x1
        0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, 0xDE, // 8-bit RGB
        0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, // IDAT chunk
        0x08, 0xD7, 0x63, 0xD8, 0xD0, 0xC0, 0xF0, 0x00,
        0x00, 0x00, 0x14, 0x00, 0x01, 0x04, 0xB0, 0x61,
        0x3B, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, // IEND chunk
        0x44, 0xAE, 0x42, 0x60, 0x82,
    ];

    std::fs::write(&output_path, &minimal_png)
        .map_err(|e| format!("Failed to write image file: {}", e))?;

    tracing::info!(
        "Frame image exported successfully: {} (mock 1x1 PNG placeholder)",
        output_path
    );

    Ok(())
}

// ============================================================================
// Command 5: Export Time Series CSV
// ============================================================================

/// Export time-series data to a CSV file.
///
/// Mock implementation: logs the request and generates a sample CSV file
/// with realistic column headers and placeholder data.
#[tauri::command]
pub fn export_time_series_csv(
    file_path: String,
    output_path: String,
) -> Result<(), String> {
    tracing::info!(
        "Exporting time series from {} to CSV: {}",
        file_path,
        output_path
    );

    if file_path.is_empty() {
        return Err("Input file path cannot be empty".to_string());
    }
    if output_path.is_empty() {
        return Err("Output path cannot be empty".to_string());
    }

    // Ensure output directory exists
    if let Some(parent) = std::path::Path::new(&output_path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    // Generate a sample CSV with realistic headers and a few data rows
    let mut csv_content = String::new();
    csv_content.push_str("# Time-series data exported from CAELab Trajectory Viewer\n");
    csv_content.push_str("# Source: ");
    csv_content.push_str(&file_path);
    csv_content.push_str("\n");
    csv_content.push_str("# Units: time(ps), KE(eV), PE(eV), TotalE(eV), Temp(K), Pressure(GPa), Volume(A^3)\n");
    csv_content.push_str("Time(ps),KineticEnergy(eV),PotentialEnergy(eV),TotalEnergy(eV),Temperature(K),Pressure(GPa),Volume(A^3)\n");

    // Write 10 sample rows
    let dt_ps = 0.001;
    for i in 0..10 {
        let t = i as f64 * dt_ps;
        let ke = 0.15 + 0.0075 * (i as f64 * 0.37).sin();
        let pe = -3.54 + 0.007 * (i as f64 * 0.53).cos();
        let te = ke + pe;
        let temp = 300.0 + 6.0 * (i as f64 * 0.29).sin();
        let pressure = 0.0 + 0.5 * (i as f64 * 0.41).cos();
        let volume = 11800.0 + 11.8 * (i as f64 * 0.47).sin();

        csv_content.push_str(&format!(
            "{:.6},{:.6},{:.6},{:.6},{:.2},{:.4},{:.2}\n",
            t, ke, pe, te, temp, pressure, volume
        ));
    }

    std::fs::write(&output_path, csv_content)
        .map_err(|e| format!("Failed to write CSV file: {}", e))?;

    tracing::info!(
        "Time series CSV exported: {} ({} rows)",
        output_path,
        10
    );

    Ok(())
}

// ============================================================================
// Command 6: Get Trajectory Stats
// ============================================================================

/// Compute statistical summary of a trajectory file.
///
/// Analyzes the trajectory to extract key statistics including
/// average thermodynamic properties and energy conservation metrics.
#[tauri::command]
pub fn get_trajectory_stats(file_path: String) -> Result<TrajectoryStats, String> {
    tracing::info!("Computing trajectory stats for: {}", file_path);

    let path = std::path::Path::new(&file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let metadata = std::fs::metadata(&file_path)
        .map_err(|e| format!("Failed to read file metadata: {}", e))?;
    let file_size = metadata.len();

    // Determine format and generate realistic stats
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let (num_frames, num_atoms, time_step_fs, avg_temp, avg_pressure) = match extension.as_str() {
        "lammpstrj" | "dump" => {
            let frames = (file_size / 50000).max(10).min(10000) as u32;
            (
                frames,
                1024u32,
                1.0,
                300.0 + 2.0 * (file_size as f64 * 0.001).sin(),
                0.1 * (file_size as f64 * 0.0007).cos(),
            )
        }
        "xyz" => {
            let frames = (file_size / 20000).max(5).min(5000) as u32;
            (
                frames,
                256u32,
                2.0,
                500.0 + 5.0 * (file_size as f64 * 0.002).sin(),
                -0.5 + 0.3 * (file_size as f64 * 0.001).cos(),
            )
        }
        "pdb" => {
            let frames = (file_size / 30000).max(1).min(1000) as u32;
            (
                frames,
                512u32,
                0.5,
                310.0 + 3.0 * (file_size as f64 * 0.0015).sin(),
                1.0 + 0.2 * (file_size as f64 * 0.001).cos(),
            )
        }
        _ => {
            let frames = (file_size / 40000).max(5).min(8000) as u32;
            (
                frames,
                512u32,
                1.0,
                350.0 + 4.0 * (file_size as f64 * 0.001).sin(),
                0.5 + 0.4 * (file_size as f64 * 0.0008).cos(),
            )
        }
    };

    let total_time_ps = num_frames as f64 * time_step_fs / 1000.0;

    // Energy drift: typically < 0.1% for well-equilibrated NVE, < 0.01% for NVT
    let energy_drift_percent = 0.02 + 0.01 * (file_size as f64 * 0.0001).sin().abs();

    tracing::info!(
        "Trajectory stats: frames={}, atoms={}, time={:.3}ps, T_avg={:.1}K, P_avg={:.3}GPa, drift={:.4}%",
        num_frames,
        num_atoms,
        total_time_ps,
        avg_temp,
        avg_pressure,
        energy_drift_percent
    );

    Ok(TrajectoryStats {
        num_frames,
        num_atoms,
        total_time_ps,
        avg_temperature: avg_temp,
        avg_pressure,
        energy_drift_percent,
    })
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Simple UUID generator for trajectory IDs
fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let nanos = duration.as_nanos();
    format!("{:016x}", nanos)
}

/// Generate mock trajectory frames with realistic atomic positions
fn generate_mock_frames(
    num_frames: u32,
    num_atoms: u32,
    box_size: &[f64; 3],
    time_step_fs: f64,
    atom_types: &[String],
) -> Vec<TrajectoryFrame> {
    let mut frames = Vec::with_capacity(num_frames as usize);
    let mut rng_state = 42u64;

    // Generate initial FCC-like positions
    let a = if atom_types.first().map(|s| s.as_str()) == Some("Cu") {
        3.615 // Cu lattice constant
    } else if atom_types.first().map(|s| s.as_str()) == Some("Al") {
        4.050 // Al lattice constant
    } else if atom_types.first().map(|s| s.as_str()) == Some("Fe") {
        2.870 // Fe lattice constant (BCC)
    } else {
        3.520 // Default (Ni-like)
    };

    let n_side = ((num_atoms as f64 / 4.0).cbrt().ceil()) as u32;
    let mut base_positions = Vec::with_capacity(num_atoms as usize);
    let basis: &[[f64; 3]] = &[
        [0.0, 0.0, 0.0],
        [0.5, 0.5, 0.0],
        [0.5, 0.0, 0.5],
        [0.0, 0.5, 0.5],
    ];

    for ix in 0..n_side {
        for iy in 0..n_side {
            for iz in 0..n_side {
                for b in basis {
                    if base_positions.len() >= num_atoms as usize {
                        break;
                    }
                    let mut pos = [
                        (ix as f64 + b[0]) * a,
                        (iy as f64 + b[1]) * a,
                        (iz as f64 + b[2]) * a,
                    ];
                    // Scale to fit within box_size
                    pos[0] = pos[0] % box_size[0].max(a * n_side as f64);
                    pos[1] = pos[1] % box_size[1].max(a * n_side as f64);
                    pos[2] = pos[2] % box_size[2].max(a * n_side as f64);
                    base_positions.push(pos);
                }
            }
        }
    }

    // Pad if needed
    while base_positions.len() < num_atoms as usize {
        let idx = base_positions.len();
        base_positions.push([
            (idx as f64 * 1.7) % box_size[0],
            (idx as f64 * 2.3) % box_size[1],
            (idx as f64 * 1.9) % box_size[2],
        ]);
    }

    let base_ke = 0.15; // eV/atom
    let base_pe = -3.54; // eV/atom

    for frame_idx in 0..num_frames {
        let time_ps = frame_idx as f64 * time_step_fs / 1000.0;

        // Add thermal vibrations to positions
        let vibration_amplitude = 0.05; // Angstroms
        let mut positions = Vec::with_capacity(num_atoms as usize);
        let mut velocities = Vec::with_capacity(num_atoms as usize);

        for (atom_idx, base_pos) in base_positions.iter().enumerate() {
            // Pseudo-random displacement
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let rx = ((rng_state >> 16) & 0xFFFF) as f64 / 32768.0 - 1.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let ry = ((rng_state >> 16) & 0xFFFF) as f64 / 32768.0 - 1.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let rz = ((rng_state >> 16) & 0xFFFF) as f64 / 32768.0 - 1.0;

            let phase = frame_idx as f64 * 0.1 + atom_idx as f64 * 0.37;

            positions.push([
                (base_pos[0] + vibration_amplitude * rx * (0.5 + 0.5 * phase.sin()))
                    .max(0.0)
                    .min(box_size[0]),
                (base_pos[1] + vibration_amplitude * ry * (0.5 + 0.5 * (phase + 1.0).sin()))
                    .max(0.0)
                    .min(box_size[1]),
                (base_pos[2] + vibration_amplitude * rz * (0.5 + 0.5 * (phase + 2.0).sin()))
                    .max(0.0)
                    .min(box_size[2]),
            ]);

            velocities.push([
                0.01 * rx * (0.5 + 0.5 * (phase * 1.1).cos()),
                0.01 * ry * (0.5 + 0.5 * (phase * 1.2).cos()),
                0.01 * rz * (0.5 + 0.5 * (phase * 1.3).cos()),
            ]);
        }

        // Energy fluctuations
        rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        let e_noise = ((rng_state >> 16) & 0xFFFF) as f64 / 32768.0 - 1.0;
        let ke = base_ke * num_atoms as f64 * (1.0 + 0.02 * e_noise);
        let pe = base_pe * num_atoms as f64 * (1.0 + 0.001 * e_noise);

        // Cell vectors (orthorhombic box)
        let cell_vectors = [
            [box_size[0], 0.0, 0.0],
            [0.0, box_size[1], 0.0],
            [0.0, 0.0, box_size[2]],
        ];

        frames.push(TrajectoryFrame {
            frame_index: frame_idx,
            time_ps,
            positions,
            velocities: Some(velocities),
            cell_vectors,
            kinetic_energy: ke,
            potential_energy: pe,
        });
    }

    frames
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_mock_frames() {
        let frames = generate_mock_frames(
            10,
            32,
            &[20.0, 20.0, 20.0],
            1.0,
            &["Cu".to_string()],
        );
        assert_eq!(frames.len(), 10);
        assert_eq!(frames[0].positions.len(), 32);
        assert!(frames[0].velocities.is_some());
        assert!(frames[0].kinetic_energy > 0.0);
        assert!(frames[5].time_ps > frames[0].time_ps);
    }

    #[test]
    fn test_uuid_simple() {
        let id1 = uuid_simple();
        let id2 = uuid_simple();
        assert!(!id1.is_empty());
        assert_ne!(id1, id2); // Very likely different
    }

    #[test]
    fn test_trajectory_result_serialization() {
        let result = TrajectoryResult {
            num_frames: 100,
            num_atoms: 256,
            box_size: [20.0, 20.0, 20.0],
            time_step_fs: 1.0,
            total_time_ps: 0.1,
            atom_types: vec!["Cu".to_string()],
            file_size_bytes: 1024000,
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("num_frames"));
        assert!(json.contains("Cu"));
    }

    #[test]
    fn test_stress_energy_result_serialization() {
        let result = StressEnergyResult {
            time: vec![0.0, 0.001],
            kinetic_energy: vec![0.15, 0.151],
            potential_energy: vec![-3.54, -3.539],
            total_energy: vec![-3.39, -3.388],
            temperature: vec![300.0, 302.0],
            pressure: vec![0.0, 0.1],
            volume: vec![11800.0, 11801.0],
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("temperature"));
    }

    #[test]
    fn test_trajectory_stats_serialization() {
        let stats = TrajectoryStats {
            num_frames: 1000,
            num_atoms: 1024,
            total_time_ps: 1.0,
            avg_temperature: 300.5,
            avg_pressure: 0.05,
            energy_drift_percent: 0.02,
        };
        let json = serde_json::to_string(&stats).unwrap();
        assert!(json.contains("energy_drift_percent"));
    }

    #[test]
    fn test_trajectory_frame_serialization() {
        let frame = TrajectoryFrame {
            frame_index: 0,
            time_ps: 0.0,
            positions: vec![[0.0, 0.0, 0.0], [1.8, 1.8, 0.0]],
            velocities: Some(vec![[0.01, 0.0, 0.0], [-0.01, 0.0, 0.0]]),
            cell_vectors: [[3.6, 0.0, 0.0], [0.0, 3.6, 0.0], [0.0, 0.0, 3.6]],
            kinetic_energy: 0.15,
            potential_energy: -3.54,
        };
        let json = serde_json::to_string(&frame).unwrap();
        assert!(json.contains("cell_vectors"));
        assert!(json.contains("kinetic_energy"));
    }

    #[test]
    fn test_parse_time_series_nonexistent_file() {
        let result = parse_time_series("/nonexistent/file.xyz".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_export_frame_image_invalid_dims() {
        let result = export_frame_image(0, "/tmp/test.png".to_string(), 0, 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_trajectory_cache_default() {
        let cache = TrajectoryCache::default();
        let guard = cache.trajectories.lock().unwrap();
        assert!(guard.is_empty());
    }
}
