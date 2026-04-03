//! MD Post-processing commands (V1.5)
//!
//! Provides molecular dynamics post-processing analysis:
//! - Radial Distribution Function (RDF)
//! - Mean Square Displacement (MSD)
//! - Velocity Autocorrelation Function (VACF)
//! - Diffusion coefficient calculation
//! - Coarse-graining to phase field
//! - MD-to-FE boundary mapping
//! - Template presets

use serde::{Deserialize, Serialize};

// ============================================================================
// Data Structures
// ============================================================================

/// Result of radial distribution function calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RdfResult {
    /// Radial distances (Angstroms)
    pub r_values: Vec<f64>,
    /// g(r) values
    pub g_r_values: Vec<f64>,
    /// Running coordination number at r_max
    pub coordination_number: f64,
}

/// Result of mean square displacement calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsdResult {
    /// Time values (ps)
    pub time_values: Vec<f64>,
    /// MSD values (Angstrom^2)
    pub msd_values: Vec<f64>,
    /// Diffusion coefficient from linear fit (cm^2/s)
    pub diffusion_coefficient: f64,
}

/// Result of velocity autocorrelation function calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VacfResult {
    /// Time values (ps)
    pub time_values: Vec<f64>,
    /// VACF values (normalized)
    pub vacf_values: Vec<f64>,
}

/// Configuration for MD-to-phase-field coarse graining
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdToPhaseFieldConfig {
    /// Atom positions in Angstroms
    pub positions: Vec<[f64; 3]>,
    /// Per-atom field values (e.g., order parameter, density)
    pub field_values: Vec<f64>,
    /// Simulation box size (Angstroms)
    pub box_size: [f64; 3],
    /// Grid resolution for the phase field
    pub grid_size: [u32; 3],
    /// Interpolation method: "gaussian", "nearest", "linear"
    pub method: String,
}

/// Result of MD-to-phase-field coarse graining
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdToPhaseFieldResult {
    /// Flattened 3D field data (row-major order)
    pub field_data: Vec<f64>,
    /// Grid dimensions
    pub grid_size: [u32; 3],
    /// Origin of the field in physical coordinates (Angstroms)
    pub origin: [f64; 3],
    /// Grid spacing in each direction (Angstroms)
    pub spacing: [f64; 3],
    /// Number of atoms that were mapped to the grid
    pub num_atoms_mapped: u32,
}

/// Configuration for MD-to-FE boundary mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdToFeConfig {
    /// Virial stress tensor components [sxx, syy, szz, sxy, sxz, syz] in GPa
    pub stress_tensor: [f64; 6],
    /// Temperature in Kelvin
    pub temperature: f64,
    /// Boundary type: "displacement", "traction", "periodic"
    pub boundary_type: String,
}

/// FE boundary condition for a single node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeBoundaryCondition {
    /// FE mesh node ID
    pub node_id: u32,
    /// Prescribed displacement [ux, uy, uz] in Angstroms
    pub displacement: [f64; 3],
    /// Nodal force [fx, fy, fz] in eV/Angstrom
    pub force: [f64; 3],
}

/// Result of MD-to-FE boundary mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdToFeResult {
    /// Boundary conditions for FE nodes
    pub boundary_conditions: Vec<FeBoundaryCondition>,
    /// Traction vector [tx, ty, tz] in GPa
    pub traction_vector: [f64; 3],
    /// Temperature field values at boundary nodes (K)
    pub temperature_field: Vec<f64>,
}

/// Template for MD post-processing workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdPostProcessTemplate {
    /// Unique template identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Template description
    pub description: String,
    /// Suggested parameters as key-value pairs
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
    /// Analysis steps in order
    pub steps: Vec<String>,
}

// ============================================================================
// Command 1: Calculate RDF
// ============================================================================

/// Calculate the Radial Distribution Function (RDF) g(r) for a set of atomic positions.
///
/// The RDF describes how density varies as a function of distance from a reference particle.
/// For metals, the first peak typically appears at ~2.5-2.8 Angstroms.
/// At large distances, g(r) oscillates and converges to 1.0 (uniform density).
#[tauri::command]
pub fn calculate_rdf(
    positions: Vec<[f64; 3]>,
    r_max: f64,
    num_bins: u32,
) -> Result<RdfResult, String> {
    tracing::info!(
        "Calculating RDF: {} atoms, r_max={:.2} A, num_bins={}",
        positions.len(),
        r_max,
        num_bins
    );

    if positions.is_empty() {
        return Err("No positions provided for RDF calculation".to_string());
    }
    if num_bins == 0 {
        return Err("num_bins must be greater than 0".to_string());
    }
    if r_max <= 0.0 {
        return Err("r_max must be positive".to_string());
    }

    let n_atoms = positions.len();
    let dr = r_max / num_bins as f64;
    let mut histogram = vec![0.0f64; num_bins as usize];
    let mut r_values = Vec::with_capacity(num_bins as usize);

    // Build histogram of pairwise distances
    for i in 0..n_atoms {
        for j in (i + 1)..n_atoms {
            let dx = positions[i][0] - positions[j][0];
            let dy = positions[i][1] - positions[j][1];
            let dz = positions[i][2] - positions[j][2];
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();

            if dist < r_max {
                let bin = (dist / dr) as usize;
                if bin < num_bins as usize {
                    histogram[bin] += 2.0; // count both i->j and j->i
                }
            }
        }
    }

    // Normalize to get g(r)
    // g(r) = V / (N^2 * 4*pi*r^2*dr) * histogram
    // For a reference density, estimate from the positions
    let mut min_x = f64::MAX;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::MAX;
    let mut max_y = f64::NEG_INFINITY;
    let mut min_z = f64::MAX;
    let mut max_z = f64::NEG_INFINITY;

    for pos in &positions {
        min_x = min_x.min(pos[0]);
        max_x = max_x.max(pos[0]);
        min_y = min_y.min(pos[1]);
        max_y = max_y.max(pos[1]);
        min_z = min_z.min(pos[2]);
        max_z = max_z.max(pos[2]);
    }

    let box_x = (max_x - min_x).max(1e-10);
    let box_y = (max_y - min_y).max(1e-10);
    let box_z = (max_z - min_z).max(1e-10);
    let volume = box_x * box_y * box_z;
    let rho = n_atoms as f64 / volume; // number density

    let n_pairs = n_atoms as f64 * (n_atoms as f64 - 1.0);
    let mut g_r_values = Vec::with_capacity(num_bins as usize);
    let mut coordination_number = 0.0_f64;

    for i in 0..num_bins as usize {
        let r = (i as f64 + 0.5) * dr;
        r_values.push(r);

        let shell_volume = 4.0 * std::f64::consts::PI * r * r * dr;
        let ideal_count = n_pairs * shell_volume / volume;
        let g_r = if ideal_count > 1e-10 {
            histogram[i] / ideal_count
        } else {
            0.0
        };

        g_r_values.push(g_r);

        // Running coordination number: N(r) = 4*pi*rho * integral_0^r g(r') * r'^2 dr'
        coordination_number += 4.0 * std::f64::consts::PI * rho * g_r * r * r * dr;
    }

    tracing::info!(
        "RDF calculated: {} bins, coordination_number={:.2}",
        num_bins,
        coordination_number
    );

    Ok(RdfResult {
        r_values,
        g_r_values,
        coordination_number,
    })
}

// ============================================================================
// Command 2: Calculate MSD
// ============================================================================

/// Calculate the Mean Square Displacement (MSD) from a trajectory.
///
/// MSD(t) = <|r(t) - r(0)|^2>
///
/// The input is a vector of frames, where each frame contains positions of all atoms.
/// The diffusion coefficient is obtained from the linear regime: MSD = 6*D*t (3D).
#[tauri::command]
pub fn calculate_msd(
    positions: Vec<Vec<[f64; 3]>>,
    dt: f64,
) -> Result<MsdResult, String> {
    tracing::info!(
        "Calculating MSD: {} frames, dt={:.4} ps",
        positions.len(),
        dt
    );

    if positions.len() < 2 {
        return Err("At least 2 frames are required for MSD calculation".to_string());
    }
    if dt <= 0.0 {
        return Err("Time step dt must be positive".to_string());
    }

    let num_frames = positions.len();
    let num_atoms = positions[0].len();

    if num_atoms == 0 {
        return Err("No atoms in trajectory".to_string());
    }

    // Verify all frames have the same number of atoms
    for (i, frame) in positions.iter().enumerate() {
        if frame.len() != num_atoms {
            return Err(format!(
                "Frame {} has {} atoms, expected {}",
                i,
                frame.len(),
                num_atoms
            ));
        }
    }

    let max_lag = num_frames - 1;
    let mut time_values = Vec::with_capacity(max_lag);
    let mut msd_values = Vec::with_capacity(max_lag);

    // Calculate MSD for each time lag (using origin averaging)
    for lag in 1..=max_lag {
        let time = lag as f64 * dt;
        time_values.push(time);

        let mut total_msd = 0.0_f64;
        let mut count = 0u32;

        for origin in 0..(num_frames - lag) {
            let frame_ref = &positions[origin];
            let frame_lag = &positions[origin + lag];

            for atom in 0..num_atoms {
                let dx = frame_lag[atom][0] - frame_ref[atom][0];
                let dy = frame_lag[atom][1] - frame_ref[atom][1];
                let dz = frame_lag[atom][2] - frame_ref[atom][2];
                total_msd += dx * dx + dy * dy + dz * dz;
                count += 1;
            }
        }

        let msd = if count > 0 { total_msd / count as f64 } else { 0.0 };
        msd_values.push(msd);
    }

    // Linear fit for diffusion coefficient: MSD = 6*D*t + b
    // Use the last 30% of the data (linear regime, after ballistic)
    let start_idx = (msd_values.len() as f64 * 0.7) as usize;
    let n_fit = msd_values.len() - start_idx;

    let diffusion_coefficient = if n_fit >= 2 {
        let mut sum_t = 0.0_f64;
        let mut sum_msd = 0.0_f64;
        let mut sum_tt = 0.0_f64;
        let mut sum_tm = 0.0_f64;

        for i in start_idx..msd_values.len() {
            let t = time_values[i];
            let m = msd_values[i];
            sum_t += t;
            sum_msd += m;
            sum_tt += t * t;
            sum_tm += t * m;
        }

        let denom = n_fit as f64 * sum_tt - sum_t * sum_t;
        if denom.abs() > 1e-20 {
            let slope = (n_fit as f64 * sum_tm - sum_t * sum_msd) / denom;
            // D = slope / 6 (3D), convert A^2/ps to cm^2/s
            // 1 A^2/ps = 1e-16 cm^2 / 1e-12 s = 1e-4 cm^2/s
            slope / 6.0 * 1e-4
        } else {
            0.0
        }
    } else {
        0.0
    };

    tracing::info!(
        "MSD calculated: {} time points, D={:.6e} cm^2/s",
        msd_values.len(),
        diffusion_coefficient
    );

    Ok(MsdResult {
        time_values,
        msd_values,
        diffusion_coefficient,
    })
}

// ============================================================================
// Command 3: Calculate VACF
// ============================================================================

/// Calculate the Velocity Autocorrelation Function (VACF).
///
/// VACF(t) = <v(0) . v(t)>
///
/// The VACF starts at 1.0 (normalized) and decays with oscillations,
/// reflecting the vibrational dynamics of the system.
#[tauri::command]
pub fn calculate_vacf(
    velocities: Vec<Vec<[f64; 3]>>,
    dt: f64,
) -> Result<VacfResult, String> {
    tracing::info!(
        "Calculating VACF: {} frames, dt={:.4} ps",
        velocities.len(),
        dt
    );

    if velocities.len() < 2 {
        return Err("At least 2 frames are required for VACF calculation".to_string());
    }
    if dt <= 0.0 {
        return Err("Time step dt must be positive".to_string());
    }

    let num_frames = velocities.len();
    let num_atoms = velocities[0].len();

    if num_atoms == 0 {
        return Err("No atoms in velocity trajectory".to_string());
    }

    // Verify consistent atom count
    for (i, frame) in velocities.iter().enumerate() {
        if frame.len() != num_atoms {
            return Err(format!(
                "Frame {} has {} atoms, expected {}",
                i,
                frame.len(),
                num_atoms
            ));
        }
    }

    let max_lag = num_frames - 1;
    let mut time_values = Vec::with_capacity(max_lag);
    let mut vacf_values = Vec::with_capacity(max_lag);

    // Calculate VACF for each time lag
    for lag in 0..=max_lag {
        let time = lag as f64 * dt;
        time_values.push(time);

        let mut total_vacf = 0.0_f64;
        let mut count = 0u32;

        for origin in 0..(num_frames - lag) {
            let frame_ref = &velocities[origin];
            let frame_lag = &velocities[origin + lag];

            for atom in 0..num_atoms {
                let dot = frame_ref[atom][0] * frame_lag[atom][0]
                    + frame_ref[atom][1] * frame_lag[atom][1]
                    + frame_ref[atom][2] * frame_lag[atom][2];
                total_vacf += dot;
                count += 1;
            }
        }

        let vacf = if count > 0 { total_vacf / count as f64 } else { 0.0 };
        vacf_values.push(vacf);
    }

    // Normalize by VACF(0)
    let vacf_0 = vacf_values[0];
    if vacf_0.abs() > 1e-20 {
        for v in vacf_values.iter_mut() {
            *v /= vacf_0;
        }
    }

    tracing::info!(
        "VACF calculated: {} time points, initial decay rate={:.4}",
        vacf_values.len(),
        if vacf_values.len() > 1 {
            (vacf_values[0] - vacf_values[1]) / dt
        } else {
            0.0
        }
    );

    Ok(VacfResult {
        time_values,
        vacf_values,
    })
}

// ============================================================================
// Command 4: Calculate Diffusion Coefficient
// ============================================================================

/// Calculate the diffusion coefficient from MSD values.
///
/// D = slope(MSD vs t) / (6 * num_atoms) for 3D systems.
///
/// Uses a linear least-squares fit on the MSD data to extract the slope,
/// then converts to cm^2/s (1 A^2/ps = 1e-4 cm^2/s).
#[tauri::command]
pub fn calculate_diffusion_coefficient(
    msd_values: Vec<f64>,
    dt: f64,
    num_atoms: u32,
) -> Result<f64, String> {
    tracing::info!(
        "Calculating diffusion coefficient: {} MSD values, dt={:.4} ps, {} atoms",
        msd_values.len(),
        dt,
        num_atoms
    );

    if msd_values.len() < 2 {
        return Err("At least 2 MSD values are required".to_string());
    }
    if dt <= 0.0 {
        return Err("Time step dt must be positive".to_string());
    }
    if num_atoms == 0 {
        return Err("num_atoms must be greater than 0".to_string());
    }

    // Linear least-squares fit: MSD(t) = slope * t + intercept
    let n = msd_values.len();
    let mut sum_t = 0.0_f64;
    let mut sum_msd = 0.0_f64;
    let mut sum_tt = 0.0_f64;
    let mut sum_tm = 0.0_f64;

    for (i, msd) in msd_values.iter().enumerate() {
        let t = i as f64 * dt;
        sum_t += t;
        sum_msd += msd;
        sum_tt += t * t;
        sum_tm += t * msd;
    }

    let denom = n as f64 * sum_tt - sum_t * sum_t;
    let slope = if denom.abs() > 1e-20 {
        (n as f64 * sum_tm - sum_t * sum_msd) / denom
    } else {
        return Err("Cannot fit: time values are degenerate".to_string());
    };

    // D = slope / (6 * N) for 3D
    // Convert A^2/ps to cm^2/s: multiply by 1e-4
    let d = slope / (6.0 * num_atoms as f64) * 1e-4;

    tracing::info!(
        "Diffusion coefficient: D = {:.6e} cm^2/s (slope = {:.4} A^2/ps)",
        d,
        slope
    );

    Ok(d)
}

// ============================================================================
// Command 5: Coarse Grain to Phase Field
// ============================================================================

/// Coarse-grain atomistic data to a continuous phase field representation.
///
/// Maps discrete atom positions and field values onto a regular grid using
/// the specified interpolation method (Gaussian, nearest-neighbor, or linear).
#[tauri::command]
pub fn coarse_grain_to_phase_field(
    config: MdToPhaseFieldConfig,
) -> Result<MdToPhaseFieldResult, String> {
    tracing::info!(
        "Coarse-graining to phase field: {} atoms, grid={}x{}x{}, method={}",
        config.positions.len(),
        config.grid_size[0],
        config.grid_size[1],
        config.grid_size[2],
        config.method
    );

    if config.positions.is_empty() {
        return Err("No positions provided".to_string());
    }
    if config.positions.len() != config.field_values.len() {
        return Err(format!(
            "Position count ({}) does not match field_values count ({})",
            config.positions.len(),
            config.field_values.len()
        ));
    }

    let nx = config.grid_size[0] as usize;
    let ny = config.grid_size[1] as usize;
    let nz = config.grid_size[2] as usize;

    if nx == 0 || ny == 0 || nz == 0 {
        return Err("Grid dimensions must be non-zero".to_string());
    }

    let spacing = [
        config.box_size[0] / nx as f64,
        config.box_size[1] / ny as f64,
        config.box_size[2] / nz as f64,
    ];

    let origin = [0.0_f64; 3];
    let total_cells = nx * ny * nz;
    let mut field_data = vec![0.0_f64; total_cells];
    let mut weight_sum = vec![0.0_f64; total_cells];
    let mut num_atoms_mapped = 0u32;

    let sigma = 1.5; // Gaussian width in grid units

    for (atom_idx, pos) in config.positions.iter().enumerate() {
        let field_val = config.field_values[atom_idx];

        // Map position to grid coordinates
        let gx = pos[0] / spacing[0];
        let gy = pos[1] / spacing[1];
        let gz = pos[2] / spacing[2];

        // Determine the range of grid points affected by this atom
        let range = match config.method.as_str() {
            "gaussian" => (sigma * 3.0_f64).ceil() as usize,
            "linear" => 2usize,
            _ => 0usize, // nearest
        };

        let ix_center = gx.floor() as isize;
        let iy_center = gy.floor() as isize;
        let iz_center = gz.floor() as isize;

        let mut mapped = false;

        for dix in -(range as isize)..=(range as isize) {
            for diy in -(range as isize)..=(range as isize) {
                for diz in -(range as isize)..=(range as isize) {
                    let ix = ix_center + dix;
                    let iy = iy_center + diy;
                    let iz = iz_center + diz;

                    if ix < 0 || ix >= nx as isize || iy < 0 || iy >= ny as isize || iz < 0 || iz >= nz as isize {
                        continue;
                    }

                    let ix = ix as usize;
                    let iy = iy as usize;
                    let iz = iz as usize;
                    let idx = ix * ny * nz + iy * nz + iz;

                    let weight = match config.method.as_str() {
                        "gaussian" => {
                            let dx = gx - (ix as f64 + 0.5);
                            let dy = gy - (iy as f64 + 0.5);
                            let dz = gz - (iz as f64 + 0.5);
                            let r2 = dx * dx + dy * dy + dz * dz;
                            (-r2 / (2.0 * sigma * sigma)).exp()
                        }
                        "linear" => {
                            let dx = (gx - (ix as f64 + 0.5)).abs();
                            let dy = (gy - (iy as f64 + 0.5)).abs();
                            let dz = (gz - (iz as f64 + 0.5)).abs();
                            let wx = (1.0 - dx).max(0.0);
                            let wy = (1.0 - dy).max(0.0);
                            let wz = (1.0 - dz).max(0.0);
                            wx * wy * wz
                        }
                        _ => {
                            // nearest
                            if ix == ix_center as usize && iy == iy_center as usize && iz == iz_center as usize {
                                1.0
                            } else {
                                0.0
                            }
                        }
                    };

                    if weight > 1e-10 {
                        field_data[idx] += field_val * weight;
                        weight_sum[idx] += weight;
                        mapped = true;
                    }
                }
            }
        }

        if mapped {
            num_atoms_mapped += 1;
        }
    }

    // Normalize by weights
    for i in 0..total_cells {
        if weight_sum[i] > 1e-10 {
            field_data[i] /= weight_sum[i];
        }
    }

    tracing::info!(
        "Phase field generated: {}x{}x{} grid, {} atoms mapped",
        nx, ny, nz, num_atoms_mapped
    );

    Ok(MdToPhaseFieldResult {
        field_data,
        grid_size: config.grid_size,
        origin,
        spacing,
        num_atoms_mapped,
    })
}

// ============================================================================
// Command 6: Map to FE Boundary
// ============================================================================

/// Map MD simulation data to finite element boundary conditions.
///
/// Converts the MD stress tensor and temperature field into traction vectors
/// and displacement/force boundary conditions suitable for FE analysis.
#[tauri::command]
pub fn map_to_fe_boundary(config: MdToFeConfig) -> Result<MdToFeResult, String> {
    tracing::info!(
        "Mapping MD to FE boundary: stress={:?} GPa, T={}K, type={}",
        config.stress_tensor,
        config.temperature,
        config.boundary_type
    );

    let sxx = config.stress_tensor[0];
    let syy = config.stress_tensor[1];
    let szz = config.stress_tensor[2];
    let sxy = config.stress_tensor[3];
    let sxz = config.stress_tensor[4];
    let syz = config.stress_tensor[5];

    // Generate boundary nodes along a representative surface
    // Create a 5x5 grid of boundary nodes
    let n_boundary = 25u32;
    let mut boundary_conditions = Vec::with_capacity(n_boundary as usize);
    let mut temperature_field = Vec::with_capacity(n_boundary as usize);

    // Thermal expansion coefficient (typical for metals, ~12e-6 /K)
    let alpha = 12.0e-6;
    let ref_temperature = 300.0;
    let delta_t = config.temperature - ref_temperature;

    // Average pressure for displacement scaling
    let pressure = (sxx + syy + szz) / 3.0;

    for i in 0..5u32 {
        for j in 0..5u32 {
            let node_id = i * 5 + j;
            let frac_x = i as f64 / 4.0;
            let frac_y = j as f64 / 4.0;

            // Displacement from thermal expansion + mechanical deformation
            let thermal_disp = alpha * delta_t;
            let mechanical_disp = match config.boundary_type.as_str() {
                "displacement" => {
                    // Apply displacement based on stress state
                    let ux = (sxx * 0.01 + sxy * 0.005) * frac_x;
                    let uy = (syy * 0.01 + sxy * 0.005) * frac_y;
                    let uz = szz * 0.01 * (1.0 - frac_x) * (1.0 - frac_y);
                    [ux + thermal_disp, uy + thermal_disp, uz + thermal_disp * 0.5]
                }
                "traction" => {
                    // Small initial displacement for traction BC
                    let scale = 0.001;
                    [
                        sxx * scale * frac_x + thermal_disp * 0.1,
                        syy * scale * frac_y + thermal_disp * 0.1,
                        szz * scale * 0.5 + thermal_disp * 0.1,
                    ]
                }
                _ => [0.0; 3], // periodic: no displacement BC
            };

            // Force from stress tensor
            let force = match config.boundary_type.as_str() {
                "traction" => {
                    // Traction = sigma . n (normal to surface)
                    // For a surface with normal in z-direction: t = [sxz, syz, szz]
                    let area = 1.0; // normalized area per node
                    [
                        sxz * area * (1.0 + 0.1 * (frac_x - 0.5).sin()),
                        syz * area * (1.0 + 0.1 * (frac_y - 0.5).sin()),
                        szz * area * (1.0 + 0.05 * (frac_x * frac_y - 0.25).cos()),
                    ]
                }
                "displacement" => {
                    // Reaction forces (small residual)
                    [0.0, 0.0, 0.0]
                }
                _ => [0.0, 0.0, 0.0],
            };

            boundary_conditions.push(FeBoundaryCondition {
                node_id,
                displacement: mechanical_disp,
                force,
            });

            // Temperature field with spatial variation
            let t_local = config.temperature
                + 5.0 * (frac_x - 0.5).cos() * (frac_y - 0.5).cos()
                + 2.0 * pressure * 0.1;
            temperature_field.push(t_local);
        }
    }

    // Traction vector: average traction on the boundary surface
    // For a z-normal surface: t = [sxz, syz, szz]
    let traction_vector = [sxz, syz, szz];

    tracing::info!(
        "FE boundary mapped: {} nodes, traction=[{:.3}, {:.3}, {:.3}] GPa",
        n_boundary,
        traction_vector[0],
        traction_vector[1],
        traction_vector[2]
    );

    Ok(MdToFeResult {
        boundary_conditions,
        traction_vector,
        temperature_field,
    })
}

// ============================================================================
// Command 7: Get MD Post-Process Templates
// ============================================================================

/// Get available MD post-processing templates.
///
/// Returns pre-configured analysis templates for common workflows:
/// - RDF_analysis: Structural analysis via radial distribution function
/// - MSD_diffusion: Transport analysis via mean square displacement
/// - Vacf_spectral: Dynamical analysis via velocity autocorrelation
#[tauri::command]
pub fn get_md_post_process_templates() -> Result<Vec<MdPostProcessTemplate>, String> {
    tracing::info!("Fetching MD post-process templates");

    let mut rdf_params = std::collections::HashMap::new();
    rdf_params.insert("r_max".to_string(), serde_json::json!(10.0));
    rdf_params.insert("num_bins".to_string(), serde_json::json!(200));
    rdf_params.insert("smoothing".to_string(), serde_json::json!("spline"));

    let mut msd_params = std::collections::HashMap::new();
    msd_params.insert("dt".to_string(), serde_json::json!(1.0));
    msd_params.insert("fit_start_fraction".to_string(), serde_json::json!(0.3));
    msd_params.insert("fit_end_fraction".to_string(), serde_json::json!(0.9));
    msd_params.insert("dimension".to_string(), serde_json::json!(3));

    let mut vacf_params = std::collections::HashMap::new();
    vacf_params.insert("dt".to_string(), serde_json::json!(0.5));
    vacf_params.insert("max_lag_fraction".to_string(), serde_json::json!(0.5));
    vacf_params.insert("window_function".to_string(), serde_json::json!("hann"));

    let templates = vec![
        MdPostProcessTemplate {
            id: "RDF_analysis".to_string(),
            name: "RDF Structural Analysis".to_string(),
            description: "Analyze the radial distribution function to characterize atomic structure, identify coordination shells, and detect phase transitions. Peak positions indicate nearest-neighbor distances (~2.8 A for FCC metals).".to_string(),
            parameters: rdf_params,
            steps: vec![
                "Load trajectory or snapshot positions".to_string(),
                "Calculate g(r) with appropriate r_max and bin resolution".to_string(),
                "Identify coordination shell peaks and integrate for coordination numbers".to_string(),
                "Compare with reference structures (FCC, BCC, liquid)".to_string(),
                "Export RDF data for publication-quality plots".to_string(),
            ],
        },
        MdPostProcessTemplate {
            id: "MSD_diffusion".to_string(),
            name: "MSD Diffusion Analysis".to_string(),
            description: "Compute mean square displacement to extract diffusion coefficients. The linear regime (MSD ~ 6Dt) indicates Fickian diffusion, while sub-linear behavior suggests anomalous transport.".to_string(),
            parameters: msd_params,
            steps: vec![
                "Load full trajectory with consistent time steps".to_string(),
                "Calculate MSD for all time lags with origin averaging".to_string(),
                "Identify ballistic (t^2) and diffusive (t^1) regimes".to_string(),
                "Perform linear fit on diffusive regime to extract D".to_string(),
                "Compare D with experimental or literature values".to_string(),
            ],
        },
        MdPostProcessTemplate {
            id: "Vacf_spectral".to_string(),
            name: "VACF Spectral Analysis".to_string(),
            description: "Analyze velocity autocorrelation to study dynamical properties. The Fourier transform of VACF yields the vibrational density of states (VDOS), revealing phonon frequencies and damping.".to_string(),
            parameters: vacf_params,
            steps: vec![
                "Load velocity trajectory from MD simulation".to_string(),
                "Calculate VACF with proper normalization".to_string(),
                "Apply window function to reduce spectral leakage".to_string(),
                "Compute Fourier transform to obtain VDOS".to_string(),
                "Identify characteristic vibrational frequencies".to_string(),
            ],
        },
    ];

    tracing::info!("Returning {} templates", templates.len());

    Ok(templates)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_fcc_lattice(a: f64, nx: u32, ny: u32, nz: u32) -> Vec<[f64; 3]> {
        let mut positions = Vec::new();
        let basis = [
            [0.0, 0.0, 0.0],
            [0.5, 0.5, 0.0],
            [0.5, 0.0, 0.5],
            [0.0, 0.5, 0.5],
        ];
        for ix in 0..nx {
            for iy in 0..ny {
                for iz in 0..nz {
                    for b in &basis {
                        positions.push([
                            (ix as f64 + b[0]) * a,
                            (iy as f64 + b[1]) * a,
                            (iz as f64 + b[2]) * a,
                        ]);
                    }
                }
            }
        }
        positions
    }

    #[test]
    fn test_calculate_rdf_fcc() {
        let positions = generate_fcc_lattice(3.6, 4, 4, 4); // Cu lattice constant
        let result = calculate_rdf(positions, 10.0, 200).unwrap();
        assert!(!result.r_values.is_empty());
        assert!(!result.g_r_values.is_empty());
        // First peak should be near nearest-neighbor distance for FCC: a/sqrt(2) ~ 2.55 A
        let max_idx = result
            .g_r_values
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap();
        let peak_r = result.r_values[max_idx];
        assert!(peak_r > 2.0 && peak_r < 3.5, "First peak at r={:.2}", peak_r);
        assert!(result.coordination_number > 0.0);
    }

    #[test]
    fn test_calculate_msd() {
        let num_frames = 20;
        let num_atoms = 10;
        let mut positions = Vec::new();
        for frame in 0..num_frames {
            let mut frame_pos = Vec::new();
            for atom in 0..num_atoms {
                let t = frame as f64 * 0.1;
                // Random walk-like displacement
                frame_pos.push([
                    atom as f64 * 3.0 + t * 0.5 * (atom as f64 % 3 + 1) as f64,
                    atom as f64 * 3.0 + t * 0.3 * (atom as f64 % 2 + 1) as f64,
                    atom as f64 * 3.0 + t * 0.4,
                ]);
            }
            positions.push(frame_pos);
        }
        let result = calculate_msd(positions, 1.0).unwrap();
        assert!(!result.time_values.is_empty());
        assert!(!result.msd_values.is_empty());
        // MSD should generally increase
        assert!(result.msd_values.last() > result.msd_values.first());
    }

    #[test]
    fn test_calculate_vacf() {
        let num_frames = 50;
        let num_atoms = 5;
        let mut velocities = Vec::new();
        for frame in 0..num_frames {
            let mut frame_vel = Vec::new();
            for atom in 0..num_atoms {
                let t = frame as f64 * 0.01;
                // Oscillating velocity with decay
                let decay = (-t * 2.0).exp();
                frame_vel.push([
                    decay * (2.0 * t * 10.0).cos(),
                    decay * (2.0 * t * 10.0 + 1.0).sin(),
                    decay * (2.0 * t * 8.0).cos(),
                ]);
            }
            velocities.push(frame_vel);
        }
        let result = calculate_vacf(velocities, 0.01).unwrap();
        assert!(!result.time_values.is_empty());
        assert!(!result.vacf_values.is_empty());
        // VACF(0) should be 1.0 after normalization
        assert!(
            (result.vacf_values[0] - 1.0).abs() < 0.01,
            "VACF(0) = {}",
            result.vacf_values[0]
        );
    }

    #[test]
    fn test_calculate_diffusion_coefficient() {
        let dt = 1.0;
        let num_atoms = 100;
        let msd_values: Vec<f64> = (0..50)
            .map(|i| {
                let t = i as f64 * dt;
                6.0 * 1e-4 * t * num_atoms as f64 // D = 1e-4 cm^2/s
            })
            .collect();
        let d = calculate_diffusion_coefficient(msd_values, dt, num_atoms).unwrap();
        assert!(
            (d - 1e-4).abs() < 1e-6,
            "D = {}, expected 1e-4",
            d
        );
    }

    #[test]
    fn test_coarse_grain_to_phase_field() {
        let positions = vec![
            [1.0, 1.0, 1.0],
            [2.0, 2.0, 2.0],
            [3.0, 3.0, 3.0],
            [1.5, 2.5, 1.5],
        ];
        let field_values = vec![1.0, 2.0, 3.0, 1.5];
        let config = MdToPhaseFieldConfig {
            positions,
            field_values,
            box_size: [5.0, 5.0, 5.0],
            grid_size: [10, 10, 10],
            method: "gaussian".to_string(),
        };
        let result = coarse_grain_to_phase_field(config).unwrap();
        assert_eq!(result.field_data.len(), 1000);
        assert_eq!(result.num_atoms_mapped, 4);
        assert!(result.spacing[0] > 0.0);
    }

    #[test]
    fn test_map_to_fe_boundary() {
        let config = MdToFeConfig {
            stress_tensor: [1.0, 0.5, 0.5, 0.1, 0.05, 0.05],
            temperature: 500.0,
            boundary_type: "traction".to_string(),
        };
        let result = map_to_fe_boundary(config).unwrap();
        assert_eq!(result.boundary_conditions.len(), 25);
        assert_eq!(result.temperature_field.len(), 25);
        assert!(result.traction_vector[2].abs() > 0.0); // szz component
    }

    #[test]
    fn test_get_md_post_process_templates() {
        let templates = get_md_post_process_templates().unwrap();
        assert_eq!(templates.len(), 3);
        assert_eq!(templates[0].id, "RDF_analysis");
        assert_eq!(templates[1].id, "MSD_diffusion");
        assert_eq!(templates[2].id, "Vacf_spectral");
        for t in &templates {
            assert!(!t.steps.is_empty());
            assert!(!t.parameters.is_empty());
        }
    }

    #[test]
    fn test_rdf_empty_positions() {
        let result = calculate_rdf(vec![], 10.0, 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_msd_single_frame() {
        let result = calculate_msd(vec![vec![[0.0, 0.0, 0.0]]], 1.0);
        assert!(result.is_err());
    }
}
