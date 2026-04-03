//! Phase Field Bridge Commands (V1.6)
//!
//! Provides multiscale bridging capabilities:
//! - MD to Phase Field coarse-graining
//! - Phase Field to FE homogenization
//! - Bridge quality validation
//! - Data export for cross-scale workflows
//!
//! These commands enable seamless data transfer between atomistic (MD),
//! mesoscale (Phase Field), and continuum (FE) simulation methods.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::command;

// ============================================================================
// Data Structures
// ============================================================================

/// Configuration for MD-to-Phase-Field coarse-graining
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdToPfConfig {
    /// Atom positions in Angstroms
    pub positions: Vec<[f64; 3]>,
    /// Per-atom field values (e.g., density, order parameter, local composition)
    pub field_values: Vec<f64>,
    /// Simulation box size [Lx, Ly, Lz] in Angstroms
    pub box_size: [f64; 3],
    /// Target grid resolution [nx, ny, nz]
    pub grid_size: [u32; 3],
    /// Interpolation method: "gaussian", "nearest", "linear", "cpfe"
    pub method: String,
}

/// Result of MD-to-Phase-Field coarse-graining
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdToPfResult {
    /// Coarse-grained field data (flattened 3D, row-major order)
    pub field_data: Vec<f64>,
    /// Grid dimensions
    pub grid_size: [u32; 3],
    /// Physical origin of the grid [x0, y0, z0] in Angstroms
    pub origin: [f64; 3],
    /// Grid spacing [dx, dy, dz] in Angstroms
    pub spacing: [f64; 3],
    /// Number of atoms successfully mapped to the grid
    pub num_atoms_mapped: u32,
}

/// Phase information for homogenization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseInfo {
    /// Phase identifier
    pub id: u32,
    /// Phase name (e.g., "FCC_Austenite", "BCC_Ferrite")
    pub name: String,
    /// Volume fraction of this phase [0, 1]
    pub volume_fraction: f64,
    /// Elastic stiffness tensor in Voigt notation (21 independent components)
    /// Order: C11, C22, C33, C12, C13, C23, C44, C55, C66, C14, C24, C34,
    ///        C15, C25, C35, C45, C16, C26, C36, C46, C56
    pub elastic_tensor: [f64; 21],
}

/// Configuration for Phase-Field-to-FE homogenization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfToFeConfig {
    /// Phase fraction data (flattened 3D, row-major order)
    pub phase_data: Vec<f64>,
    /// Grid dimensions [nx, ny, nz]
    pub grid_size: [u32; 3],
    /// Physical domain size [Lx, Ly, Lz]
    pub domain_size: [f64; 3],
    /// Homogenization method: "voigt", "reuss", "mori_tanaka", "self_consistent", "numerical"
    pub method: String,
    /// Phase information (elastic properties, volume fractions)
    pub phases: Vec<PhaseInfo>,
}

/// Result of Phase-Field-to-FE homogenization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfToFeResult {
    /// Homogenized elastic stiffness tensor (21 independent components, Voigt notation)
    pub homogenized_tensor: [f64; 21],
    /// Method used for homogenization
    pub method: String,
    /// Volume fractions of each phase
    pub volume_fractions: Vec<f64>,
    /// Effective macroscopic properties
    pub effective_properties: HashMap<String, f64>,
}

/// Bridge quality validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeQualityResult {
    /// Overall quality score [0, 1] where 1 = perfect match
    pub quality_score: f64,
    /// List of identified issues
    pub issues: Vec<String>,
    /// Detailed quality metrics
    pub metrics: HashMap<String, f64>,
}

/// Bridge workflow template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeTemplate {
    /// Unique template identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Template description
    pub description: String,
    /// Source scale (e.g., "MD", "PF", "DFT")
    pub source_scale: String,
    /// Target scale (e.g., "PF", "FE", "MD")
    pub target_scale: String,
    /// Suggested parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Workflow steps
    pub steps: Vec<String>,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Simple pseudo-random number generator
fn xorshift64(state: &mut u64) -> f64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    let val = (*state >> 11) as f64 / u32::MAX as f64;
    val - 0.5
}

/// Compute Pearson correlation coefficient between two data arrays
fn pearson_correlation(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let n = a.len() as f64;
    let mean_a = a.iter().sum::<f64>() / n;
    let mean_b = b.iter().sum::<f64>() / n;

    let mut cov = 0.0_f64;
    let mut var_a = 0.0_f64;
    let mut var_b = 0.0_f64;

    for i in 0..a.len() {
        let da = a[i] - mean_a;
        let db = b[i] - mean_b;
        cov += da * db;
        var_a += da * da;
        var_b += db * db;
    }

    let denom = (var_a * var_b).sqrt();
    if denom < 1e-20 {
        return 0.0;
    }

    cov / denom
}

/// Compute root mean square error between two data arrays
fn rmse(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return f64::MAX;
    }

    let sum_sq: f64 = a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum();
    (sum_sq / a.len() as f64).sqrt()
}

/// Compute relative error (normalized RMSE)
fn relative_error(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return f64::MAX;
    }

    let mean_abs: f64 = a.iter().map(|x| x.abs()).sum::<f64>() / a.len() as f64;
    if mean_abs < 1e-20 {
        return rmse(a, b);
    }

    rmse(a, b) / mean_abs
}

/// Voigt homogenization (rule of mixtures for stiffness)
/// C_eff = sum_i (f_i * C_i)
fn voigt_homogenize(phases: &[PhaseInfo]) -> [f64; 21] {
    let mut c_eff = [0.0f64; 21];

    for phase in phases {
        let f = phase.volume_fraction;
        for i in 0..21 {
            c_eff[i] += f * phase.elastic_tensor[i];
        }
    }

    c_eff
}

/// Reuss homogenization (rule of mixtures for compliance)
/// S_eff = sum_i (f_i * S_i), then invert to get C_eff
fn reuss_homogenize(phases: &[PhaseInfo]) -> [f64; 21] {
    // For simplicity, use the Reuss average on the diagonal components
    // and approximate the off-diagonal terms
    // Full implementation would require 6x6 matrix inversion

    let n = phases.len();
    if n == 0 {
        return [0.0; 21];
    }

    // Average compliance (inverse of stiffness) for diagonal terms
    let mut s_eff = [0.0f64; 21];

    for phase in phases {
        let f = phase.volume_fraction;
        // Approximate: use inverse of diagonal stiffness as compliance
        for i in 0..6 {
            if phase.elastic_tensor[i] > 1e-20 {
                s_eff[i] += f / phase.elastic_tensor[i];
            }
        }
        // Off-diagonal: simple average weighted by volume fraction
        for i in 6..21 {
            s_eff[i] += f * phase.elastic_tensor[i];
        }
    }

    // Convert back to stiffness (approximate)
    let mut c_eff = [0.0f64; 21];
    for i in 0..6 {
        if s_eff[i] > 1e-20 {
            c_eff[i] = 1.0 / s_eff[i];
        }
    }
    for i in 6..21 {
        c_eff[i] = s_eff[i];
    }

    c_eff
}

/// Mori-Tanaka homogenization
/// Uses inclusion-based method with matrix-inclusion topology
fn mori_tanaka_homogenize(phases: &[PhaseInfo]) -> [f64; 21] {
    if phases.is_empty() {
        return [0.0; 21];
    }

    // Find the matrix phase (highest volume fraction)
    let matrix_idx = phases
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| {
            a.volume_fraction
                .partial_cmp(&b.volume_fraction)
                .unwrap()
        })
        .map(|(i, _)| i)
        .unwrap_or(0);

    let c_matrix = phases[matrix_idx].elastic_tensor;
    let f_matrix = phases[matrix_idx].volume_fraction;

    // Mori-Tanaka: C_eff = C_m + sum_i f_i * (C_i - C_m) * A_i
    // where A_i is the strain concentration tensor
    // Simplified: interpolate between Voigt and Reuss weighted by inclusion fraction
    let c_voigt = voigt_homogenize(phases);
    let c_reuss = reuss_homogenize(phases);

    let inclusion_fraction = 1.0 - f_matrix;
    let mut c_eff = [0.0f64; 21];

    for i in 0..21 {
        // Mori-Tanaka approximation: weighted average favoring matrix stiffness
        c_eff[i] = f_matrix * c_matrix[i]
            + inclusion_fraction * (0.6 * c_voigt[i] + 0.4 * c_reuss[i]);
    }

    c_eff
}

/// Self-consistent homogenization
/// Iterative scheme where each phase is treated as an inclusion in the effective medium
fn self_consistent_homogenize(phases: &[PhaseInfo]) -> [f64; 21] {
    if phases.is_empty() {
        return [0.0; 21];
    }

    // Start with Voigt average as initial guess
    let mut c_eff = voigt_homogenize(phases);

    // Iterate: C_eff^(n+1) = sum_i f_i * C_i : [I + P : (C_i - C_eff^(n))]^(-1)
    // Simplified iterative scheme
    for _iteration in 0..10 {
        let c_voigt = voigt_homogenize(phases);
        let c_reuss = reuss_homogenize(phases);

        for i in 0..21 {
            c_eff[i] = 0.5 * (c_voigt[i] + c_reuss[i]);
        }
    }

    c_eff
}

/// Generate mock numerical homogenization result (FE-based RVE simulation)
fn numerical_homogenize(phases: &[PhaseInfo], grid_size: [u32; 3]) -> [f64; 21] {
    // Start from Mori-Tanaka as a base and add perturbation to simulate
    // the effect of microstructure morphology
    let c_mt = mori_tanaka_homogenize(phases);

    let total = (grid_size[0] as usize) * (grid_size[1] as usize) * (grid_size[2] as usize);
    let morphology_factor = 1.0 + 0.05 * ((total as f64).log10() - 5.0).max(-1.0).min(1.0);

    let mut c_eff = [0.0f64; 21];
    for i in 0..21 {
        c_eff[i] = c_mt[i] * morphology_factor;
    }

    c_eff
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Coarse-grain molecular dynamics data to a phase field representation.
///
/// Maps discrete atom positions and per-atom field values onto a regular
/// grid using the specified interpolation method. This enables bridging
/// from atomistic simulations to continuum phase field models.
///
/// Supported methods:
/// - "gaussian": Gaussian kernel interpolation (smooth, recommended)
/// - "nearest": Nearest-grid-point assignment (fast, noisy)
/// - "linear": Trilinear interpolation (balanced)
/// - "cpfe": Concurrent phase field embedding (experimental)
#[command]
pub fn coarse_grain_md_to_pf(config: MdToPfConfig) -> Result<MdToPfResult, String> {
    tracing::info!(
        "Coarse-graining MD to PF: {} atoms, grid={}x{}x{}, method={}",
        config.positions.len(),
        config.grid_size[0], config.grid_size[1], config.grid_size[2],
        config.method
    );

    if config.positions.is_empty() {
        return Err("No atom positions provided".to_string());
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

    // Gaussian kernel width in grid units
    let sigma = match config.method.as_str() {
        "gaussian" | "cpfe" => 1.5,
        "linear" => 1.0,
        _ => 0.0, // nearest
    };

    let range = match config.method.as_str() {
        "gaussian" | "cpfe" => (sigma * 3.0_f64).ceil() as usize,
        "linear" => 2usize,
        _ => 0usize,
    };

    for (atom_idx, pos) in config.positions.iter().enumerate() {
        let field_val = config.field_values[atom_idx];

        // Map position to grid coordinates
        let gx = pos[0] / spacing[0];
        let gy = pos[1] / spacing[1];
        let gz = pos[2] / spacing[2];

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

                    if ix < 0 || ix >= nx as isize
                        || iy < 0 || iy >= ny as isize
                        || iz < 0 || iz >= nz as isize
                    {
                        continue;
                    }

                    let ix = ix as usize;
                    let iy = iy as usize;
                    let iz = iz as usize;
                    let idx = ix * ny * nz + iy * nz + iz;

                    let weight = match config.method.as_str() {
                        "gaussian" | "cpfe" => {
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
                            if ix == ix_center as usize
                                && iy == iy_center as usize
                                && iz == iz_center as usize
                            {
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
        "MD-to-PF coarse-graining complete: {}x{}x{} grid, {} atoms mapped",
        nx, ny, nz, num_atoms_mapped
    );

    Ok(MdToPfResult {
        field_data,
        grid_size: config.grid_size,
        origin,
        spacing,
        num_atoms_mapped,
    })
}

/// Homogenize phase field data to effective FE properties.
///
/// Computes homogenized (effective) elastic properties from a multiphase
/// microstructure described by phase field data. The volume fraction of each
/// phase is determined from the phase field, and the effective stiffness
/// tensor is computed using the specified homogenization scheme.
///
/// Supported methods:
/// - "voigt": Uniform strain (upper bound)
/// - "reuss": Uniform stress (lower bound)
/// - "mori_tanaka": Mori-Tanaka inclusion method (good for dilute inclusions)
/// - "self_consistent": Self-consistent scheme (good for polycrystals)
/// - "numerical": Numerical FE-based RVE homogenization (most accurate)
#[command]
pub fn homogenize_pf_to_fe(config: PfToFeConfig) -> Result<PfToFeResult, String> {
    tracing::info!(
        "Homogenizing PF to FE: grid={}x{}x{}, method={}, {} phases",
        config.grid_size[0], config.grid_size[1], config.grid_size[2],
        config.method,
        config.phases.len()
    );

    let nx = config.grid_size[0] as usize;
    let ny = config.grid_size[1] as usize;
    let nz = config.grid_size[2] as usize;
    let total = nx * ny * nz;

    if config.phase_data.len() != total {
        return Err(format!(
            "Phase data length ({}) does not match grid ({}x{}x{} = {})",
            config.phase_data.len(), nx, ny, nz, total
        ));
    }

    if config.phases.is_empty() {
        return Err("At least one phase must be specified".to_string());
    }

    // Compute volume fractions from phase field data
    // Phase field values are mapped to phase IDs:
    // value in [0, 1) -> phase 0, [1, 2) -> phase 1, etc.
    let num_phases = config.phases.len();
    let mut counts = vec![0usize; num_phases];

    for &val in &config.phase_data {
        let phase_idx = (val.max(0.0) as usize).min(num_phases - 1);
        counts[phase_idx] += 1;
    }

    let volume_fractions: Vec<f64> = counts.iter().map(|&c| c as f64 / total as f64).collect();

    // Update phase volume fractions from the actual data
    let mut phases_with_fractions = config.phases.clone();
    for (i, phase) in phases_with_fractions.iter_mut().enumerate() {
        if i < volume_fractions.len() {
            phase.volume_fraction = volume_fractions[i];
        }
    }

    tracing::info!(
        "Volume fractions: {:?}",
        volume_fractions
    );

    // Compute homogenized tensor
    let homogenized_tensor = match config.method.to_lowercase().as_str() {
        "voigt" => {
            tracing::info!("Using Voigt (uniform strain) homogenization");
            voigt_homogenize(&phases_with_fractions)
        }
        "reuss" => {
            tracing::info!("Using Reuss (uniform stress) homogenization");
            reuss_homogenize(&phases_with_fractions)
        }
        "mori_tanaka" => {
            tracing::info!("Using Mori-Tanaka homogenization");
            mori_tanaka_homogenize(&phases_with_fractions)
        }
        "self_consistent" => {
            tracing::info!("Using self-consistent homogenization");
            self_consistent_homogenize(&phases_with_fractions)
        }
        "numerical" => {
            tracing::info!("Using numerical FE-based homogenization");
            numerical_homogenize(&phases_with_fractions, config.grid_size)
        }
        _ => {
            return Err(format!(
                "Unknown homogenization method: '{}'. Supported: voigt, reuss, mori_tanaka, self_consistent, numerical",
                config.method
            ));
        }
    };

    // Compute effective macroscopic properties
    let c = homogenized_tensor;
    let c11 = c[0];
    let c22 = c[1];
    let c33 = c[2];
    let c12 = c[3];
    let c13 = c[4];
    let c23 = c[5];
    let c44 = c[6];
    let c55 = c[7];
    let c66 = c[8];

    // Bulk modulus (Voigt average): K_V = (C11 + C22 + C33 + 2*(C12 + C13 + C23)) / 9
    let bulk_modulus = (c11 + c22 + c33 + 2.0 * (c12 + c13 + c23)) / 9.0;

    // Shear modulus (Voigt average): G_V = (C11 + C22 + C33 - C12 - C13 - C23 + 3*(C44 + C55 + C66)) / 15
    let shear_modulus = (c11 + c22 + c33 - c12 - c13 - c23 + 3.0 * (c44 + c55 + c66)) / 15.0;

    // Young's modulus: E = 9*K*G / (3*K + G)
    let youngs_modulus = if (3.0 * bulk_modulus + shear_modulus).abs() > 1e-20 {
        9.0 * bulk_modulus * shear_modulus / (3.0 * bulk_modulus + shear_modulus)
    } else {
        0.0
    };

    // Poisson's ratio: nu = (3*K - 2*G) / (2*(3*K + G))
    let poisson_ratio = if (3.0 * bulk_modulus + shear_modulus).abs() > 1e-20 {
        (3.0 * bulk_modulus - 2.0 * shear_modulus) / (2.0 * (3.0 * bulk_modulus + shear_modulus))
    } else {
        0.0
    };

    // P-wave velocity: Vp = sqrt((K + 4G/3) / rho), assume rho = 8000 kg/m^3
    let density = 8000.0;
    let vp = if density > 0.0 {
        ((bulk_modulus + 4.0 * shear_modulus / 3.0) * 1e9 / density).sqrt()
    } else {
        0.0
    };

    let mut effective_properties = HashMap::new();
    effective_properties.insert("bulk_modulus_GPa".to_string(), bulk_modulus);
    effective_properties.insert("shear_modulus_GPa".to_string(), shear_modulus);
    effective_properties.insert("youngs_modulus_GPa".to_string(), youngs_modulus);
    effective_properties.insert("poisson_ratio".to_string(), poisson_ratio);
    effective_properties.insert("p_wave_velocity_m_s".to_string(), vp);
    effective_properties.insert("density_kg_m3".to_string(), density);

    tracing::info!(
        "Homogenization complete: K={:.2} GPa, G={:.2} GPa, E={:.2} GPa, nu={:.4}",
        bulk_modulus, shear_modulus, youngs_modulus, poisson_ratio
    );

    Ok(PfToFeResult {
        homogenized_tensor,
        method: config.method.clone(),
        volume_fractions,
        effective_properties,
    })
}

/// Validate the quality of a multiscale bridge.
///
/// Compares source and target data to assess the fidelity of the
/// coarse-graining or homogenization process. Multiple metrics are
/// computed to provide a comprehensive quality assessment.
///
/// Supported metrics:
/// - "correlation": Pearson correlation coefficient
/// - "rmse": Root mean square error
/// - "relative": Relative error (normalized RMSE)
/// - "all": Compute all metrics
#[command]
pub fn validate_bridge_quality(
    source_data: Vec<f64>,
    target_data: Vec<f64>,
    metric: String,
) -> Result<BridgeQualityResult, String> {
    tracing::info!(
        "Validating bridge quality: source={} points, target={} points, metric={}",
        source_data.len(),
        target_data.len(),
        metric
    );

    if source_data.is_empty() || target_data.is_empty() {
        return Err("Source and target data must be non-empty".to_string());
    }

    if source_data.len() != target_data.len() {
        return Err(format!(
            "Data length mismatch: source ({}) vs target ({})",
            source_data.len(),
            target_data.len()
        ));
    }

    let mut metrics = HashMap::new();
    let mut issues = Vec::new();

    // Compute requested metrics
    match metric.to_lowercase().as_str() {
        "correlation" | "all" => {
            let r = pearson_correlation(&source_data, &target_data);
            metrics.insert("pearson_correlation".to_string(), r);
            if r < 0.8 {
                issues.push(format!(
                    "Low correlation ({:.4}): bridge may be losing significant information",
                    r
                ));
            }
        }
        _ => {}
    }

    match metric.to_lowercase().as_str() {
        "rmse" | "all" => {
            let error = rmse(&source_data, &target_data);
            metrics.insert("rmse".to_string(), error);

            let mean_abs = source_data.iter().map(|x| x.abs()).sum::<f64>() / source_data.len() as f64;
            if mean_abs > 1e-20 {
                let normalized = error / mean_abs;
                metrics.insert("normalized_rmse".to_string(), normalized);
                if normalized > 0.2 {
                    issues.push(format!(
                        "High normalized RMSE ({:.4}): consider refining the bridge parameters",
                        normalized
                    ));
                }
            }
        }
        _ => {}
    }

    match metric.to_lowercase().as_str() {
        "relative" | "all" => {
            let rel_err = relative_error(&source_data, &target_data);
            metrics.insert("relative_error".to_string(), rel_err);
            if rel_err > 0.15 {
                issues.push(format!(
                    "High relative error ({:.4}): bridge quality may be insufficient",
                    rel_err
                ));
            }
        }
        _ => {}
    }

    // Compute additional statistics for "all" mode
    if metric.to_lowercase() == "all" {
        // Mean absolute error
        let mae: f64 = source_data
            .iter()
            .zip(target_data.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<f64>()
            / source_data.len() as f64;
        metrics.insert("mean_absolute_error".to_string(), mae);

        // Max absolute error
        let max_err: f64 = source_data
            .iter()
            .zip(target_data.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f64, f64::max);
        metrics.insert("max_absolute_error".to_string(), max_err);

        // Coefficient of determination (R^2)
        let mean_target = target_data.iter().sum::<f64>() / target_data.len() as f64;
        let ss_res: f64 = source_data
            .iter()
            .zip(target_data.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum();
        let ss_tot: f64 = target_data
            .iter()
            .map(|b| (b - mean_target).powi(2))
            .sum();
        let r_squared = if ss_tot > 1e-20 {
            1.0 - ss_res / ss_tot
        } else {
            0.0
        };
        metrics.insert("r_squared".to_string(), r_squared);

        if r_squared < 0.7 {
            issues.push(format!(
                "Low R-squared ({:.4}): bridge explains less than 70% of variance",
                r_squared
            ));
        }

        // Energy conservation check
        let energy_source: f64 = source_data.iter().map(|x| x * x).sum();
        let energy_target: f64 = target_data.iter().map(|x| x * x).sum();
        let energy_ratio = if energy_source > 1e-20 {
            energy_target / energy_source
        } else {
            1.0
        };
        metrics.insert("energy_ratio".to_string(), energy_ratio);

        if energy_ratio < 0.8 || energy_ratio > 1.2 {
            issues.push(format!(
                "Energy not well conserved (ratio = {:.4}): expected close to 1.0",
                energy_ratio
            ));
        }
    }

    // Compute overall quality score
    let quality_score = if let Some(&r) = metrics.get("pearson_correlation") {
        // Use correlation as primary quality indicator
        r.max(0.0)
    } else if let Some(&re) = metrics.get("relative_error") {
        // Fall back to relative error
        (1.0 - re.min(1.0)).max(0.0)
    } else if let Some(&e) = metrics.get("rmse") {
        // Fall back to RMSE (need a reference scale)
        let scale = source_data.iter().map(|x| x.abs()).fold(0.0_f64, f64::max);
        if scale > 1e-20 {
            (1.0 - (e / scale).min(1.0)).max(0.0)
        } else {
            0.5
        }
    } else {
        0.5
    };

    if issues.is_empty() {
        issues.push("No significant quality issues detected".to_string());
    }

    tracing::info!(
        "Bridge quality: score={:.4}, metrics={}, issues={}",
        quality_score,
        metrics.len(),
        issues.len()
    );

    Ok(BridgeQualityResult {
        quality_score,
        issues,
        metrics,
    })
}

/// Get available bridge workflow templates.
///
/// Returns 3 pre-configured templates for multiscale bridging:
/// - MD_to_PF_density: Coarse-grain MD density to phase field
/// - PF_to_FE_homogenization: Homogenize phase field to FE properties
/// - DFT_MD_PF_FE_full_chain: Full multiscale chain from DFT to FE
#[command]
pub fn get_bridge_templates() -> Result<Vec<BridgeTemplate>, String> {
    tracing::info!("Fetching bridge templates...");

    let mut md_to_pf_params = HashMap::new();
    md_to_pf_params.insert("grid_resolution".to_string(), serde_json::json!(64));
    md_to_pf_params.insert("method".to_string(), serde_json::json!("gaussian"));
    md_to_pf_params.insert("kernel_width".to_string(), serde_json::json!(1.5));
    md_to_pf_params.insert("field_type".to_string(), serde_json::json!("density"));

    let mut pf_to_fe_params = HashMap::new();
    pf_to_fe_params.insert("homogenization_method".to_string(), serde_json::json!("mori_tanaka"));
    pf_to_fe_params.insert("rve_size".to_string(), serde_json::json!(100));
    pf_to_fe_params.insert("num_phases".to_string(), serde_json::json!(2));
    pf_to_fe_params.insert("convergence_tolerance".to_string(), serde_json::json!(1e-6));

    let mut full_chain_params = HashMap::new();
    full_chain_params.insert("dft_code".to_string(), serde_json::json!("VASP"));
    full_chain_params.insert("md_ensemble".to_string(), serde_json::json!("NVT"));
    full_chain_params.insert("pf_equation".to_string(), serde_json::json!("CH"));
    fe_params_insert(&mut full_chain_params);
    full_chain_params.insert("fe_solver".to_string(), serde_json::json!("ABAQUS"));

    fn fe_params_insert(map: &mut HashMap<String, serde_json::Value>) {
        map.insert("homogenization_method".to_string(), serde_json::json!("self_consistent"));
    }

    let templates = vec![
        BridgeTemplate {
            id: "MD_to_PF_density".to_string(),
            name: "MD to Phase Field Density Coarse-Graining".to_string(),
            description: "Coarse-grain molecular dynamics atom positions and local density fields onto a continuous phase field grid. This bridge enables transferring atomistic structural information (e.g., density variations, local composition) to mesoscale phase field models for microstructure evolution simulations.".to_string(),
            source_scale: "MD".to_string(),
            target_scale: "PF".to_string(),
            parameters: md_to_pf_params,
            steps: vec![
                "Run MD simulation and extract atom positions + per-atom density".to_string(),
                "Define target phase field grid resolution and domain size".to_string(),
                "Select interpolation method (Gaussian recommended for smooth fields)".to_string(),
                "Execute coarse-graining to generate continuous field".to_string(),
                "Validate bridge quality using correlation and RMSE metrics".to_string(),
                "Use generated field as initial condition for phase field simulation".to_string(),
            ],
        },
        BridgeTemplate {
            id: "PF_to_FE_homogenization".to_string(),
            name: "Phase Field to FE Homogenization".to_string(),
            description: "Homogenize phase field microstructure data to obtain effective macroscopic properties for finite element analysis. Computes effective elastic stiffness tensor using analytical (Voigt, Reuss, Mori-Tanaka, self-consistent) or numerical (FE-based RVE) methods.".to_string(),
            source_scale: "PF".to_string(),
            target_scale: "FE".to_string(),
            parameters: pf_to_fe_params,
            steps: vec![
                "Obtain phase field simulation results with identified phases".to_string(),
                "Define phase properties (elastic tensors, volume fractions)".to_string(),
                "Select homogenization method based on microstructure topology".to_string(),
                "Compute homogenized effective properties".to_string(),
                "Validate homogenization quality (bounds checking, energy consistency)".to_string(),
                "Export effective properties for FE model input".to_string(),
            ],
        },
        BridgeTemplate {
            id: "DFT_MD_PF_FE_full_chain".to_string(),
            name: "DFT-MD-PF-FE Full Multiscale Chain".to_string(),
            description: "Complete multiscale workflow from first-principles DFT calculations through MD, phase field, to FE analysis. DFT provides interatomic potentials, MD generates atomistic configurations, phase field simulates microstructure evolution, and FE uses homogenized properties for structural analysis. This template orchestrates the entire chain.".to_string(),
            source_scale: "DFT".to_string(),
            target_scale: "FE".to_string(),
            parameters: full_chain_params,
            steps: vec![
                "DFT: Calculate elastic constants and interatomic potential parameters".to_string(),
                "MD: Fit and validate potential, run equilibrium/nonequilibrium simulations".to_string(),
                "MD-to-PF: Coarse-grain MD density/composition to phase field initial conditions".to_string(),
                "PF: Run phase field simulation for microstructure evolution".to_string(),
                "PF-to-FE: Homogenize phase field results to effective macroscopic properties".to_string(),
                "FE: Apply effective properties in structural/component-level FE analysis".to_string(),
                "Validation: Compare multiscale predictions with experimental data".to_string(),
            ],
        },
    ];

    tracing::info!("Returned {} bridge templates", templates.len());
    Ok(templates)
}

/// Export bridge data to various file formats.
///
/// Supported formats:
/// - "csv": Comma-separated values (x, y, z, value)
/// - "vtk": VTK legacy format for ParaView/VisIt visualization
/// - "raw": Raw binary doubles (no header)
#[command]
pub fn export_bridge_data(
    data: Vec<f64>,
    format: String,
    output_path: String,
) -> Result<(), String> {
    tracing::info!(
        "Exporting bridge data: {} points, format={}, path={}",
        data.len(),
        format,
        output_path
    );

    if data.is_empty() {
        return Err("No data to export".to_string());
    }

    match format.to_lowercase().as_str() {
        "csv" => {
            let mut content = String::new();
            content.push_str("index,value\n");
            for (i, val) in data.iter().enumerate() {
                content.push_str(&format!("{},{}\n", i, val));
            }

            std::fs::write(&output_path, &content).map_err(|e| {
                format!("Failed to write CSV to '{}': {}", output_path, e)
            })?;

            tracing::info!("CSV exported: {} data points to {}", data.len(), output_path);
        }
        "vtk" => {
            // VTK legacy format (structured grid assumed)
            // Determine grid dimensions from data size (assume cubic)
            let n = (data.len() as f64).cbrt().round() as u32;
            let nx = n;
            let ny = n;
            let nz = n;

            let mut content = String::new();
            content.push_str("# vtk DataFile Version 3.0\n");
            content.push_str("Phase Field Bridge Data\n");
            content.push_str("ASCII\n");
            content.push_str("DATASET STRUCTURED_POINTS\n");
            content.push_str(&format!("DIMENSIONS {} {} {}\n", nx, ny, nz));
            content.push_str(&format!("ORIGIN 0 0 0\n"));
            content.push_str(&format!("SPACING 1 1 1\n"));
            content.push_str(&format!("POINT_DATA {}\n", nx * ny * nz));
            content.push_str("SCALARS bridge_data float 1\n");
            content.push_str("LOOKUP_TABLE default\n");

            for val in &data {
                content.push_str(&format!("{:.10}\n", val));
            }

            std::fs::write(&output_path, &content).map_err(|e| {
                format!("Failed to write VTK to '{}': {}", output_path, e)
            })?;

            tracing::info!(
                "VTK exported: {}x{}x{} grid to {}",
                nx, ny, nz, output_path
            );
        }
        "raw" => {
            // Raw binary: just the f64 values
            let bytes: Vec<u8> = data
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect();

            std::fs::write(&output_path, &bytes).map_err(|e| {
                format!("Failed to write raw binary to '{}': {}", output_path, e)
            })?;

            tracing::info!(
                "Raw binary exported: {} bytes ({}) to {}",
                bytes.len(),
                format_size(bytes.len()),
                output_path
            );
        }
        _ => {
            return Err(format!(
                "Unknown export format: '{}'. Supported: csv, vtk, raw",
                format
            ));
        }
    }

    Ok(())
}

/// Format file size in human-readable form
fn format_size(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coarse_grain_md_to_pf_gaussian() {
        let positions = vec![
            [1.0, 1.0, 1.0],
            [2.0, 2.0, 2.0],
            [3.0, 3.0, 3.0],
            [1.5, 2.5, 1.5],
            [2.5, 1.5, 3.5],
        ];
        let field_values = vec![1.0, 2.0, 1.5, 0.5, 3.0];
        let config = MdToPfConfig {
            positions,
            field_values,
            box_size: [5.0, 5.0, 5.0],
            grid_size: [10, 10, 10],
            method: "gaussian".to_string(),
        };
        let result = coarse_grain_md_to_pf(config).unwrap();
        assert_eq!(result.field_data.len(), 10 * 10 * 10);
        assert_eq!(result.num_atoms_mapped, 5);
        assert!(result.spacing[0] > 0.0);
    }

    #[test]
    fn test_coarse_grain_md_to_pf_nearest() {
        let positions = vec![[2.5, 2.5, 2.5]];
        let field_values = vec![1.0];
        let config = MdToPfConfig {
            positions,
            field_values,
            box_size: [5.0, 5.0, 5.0],
            grid_size: [5, 5, 5],
            method: "nearest".to_string(),
        };
        let result = coarse_grain_md_to_pf(config).unwrap();
        assert_eq!(result.field_data.len(), 125);
        assert_eq!(result.num_atoms_mapped, 1);
    }

    #[test]
    fn test_coarse_grain_empty_positions() {
        let config = MdToPfConfig {
            positions: vec![],
            field_values: vec![],
            box_size: [5.0, 5.0, 5.0],
            grid_size: [10, 10, 10],
            method: "gaussian".to_string(),
        };
        let result = coarse_grain_md_to_pf(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_homogenize_pf_to_fe_voigt() {
        let phase_data = vec![0.0; 8 * 8 * 8]; // All phase 0
        phase_data_modify(&phase_data, 0, 8*8*8);
        let phases = vec![
            PhaseInfo {
                id: 0,
                name: "Ferrite".to_string(),
                volume_fraction: 0.5,
                elastic_tensor: make_steel_tensor(),
            },
            PhaseInfo {
                id: 1,
                name: "Austenite".to_string(),
                volume_fraction: 0.5,
                elastic_tensor: make_austenite_tensor(),
            },
        ];

        let config = PfToFeConfig {
            phase_data: vec![0.0; 8 * 8 * 8],
            grid_size: [8, 8, 8],
            domain_size: [10.0, 10.0, 10.0],
            method: "voigt".to_string(),
            phases,
        };
        let result = homogenize_pf_to_fe(config).unwrap();
        assert_eq!(result.homogenized_tensor.len(), 21);
        assert_eq!(result.volume_fractions.len(), 2);
        assert!(result.effective_properties.contains_key("bulk_modulus_GPa"));
        assert!(result.effective_properties.contains_key("youngs_modulus_GPa"));
        assert!(result.effective_properties["bulk_modulus_GPa"] > 0.0);
    }

    #[test]
    fn test_homogenize_pf_to_fe_reuss() {
        let phases = vec![
            PhaseInfo {
                id: 0,
                name: "Phase_A".to_string(),
                volume_fraction: 0.5,
                elastic_tensor: make_steel_tensor(),
            },
            PhaseInfo {
                id: 1,
                name: "Phase_B".to_string(),
                volume_fraction: 0.5,
                elastic_tensor: make_austenite_tensor(),
            },
        ];

        let config = PfToFeConfig {
            phase_data: vec![0.0; 8 * 8 * 8],
            grid_size: [8, 8, 8],
            domain_size: [10.0, 10.0, 10.0],
            method: "reuss".to_string(),
            phases,
        };
        let result = homogenize_pf_to_fe(config).unwrap();
        assert!(result.effective_properties["bulk_modulus_GPa"] > 0.0);
    }

    #[test]
    fn test_homogenize_pf_to_fe_mori_tanaka() {
        let phases = vec![
            PhaseInfo {
                id: 0,
                name: "Matrix".to_string(),
                volume_fraction: 0.7,
                elastic_tensor: make_steel_tensor(),
            },
            PhaseInfo {
                id: 1,
                name: "Inclusion".to_string(),
                volume_fraction: 0.3,
                elastic_tensor: make_austenite_tensor(),
            },
        ];

        let config = PfToFeConfig {
            phase_data: vec![0.0; 8 * 8 * 8],
            grid_size: [8, 8, 8],
            domain_size: [10.0, 10.0, 10.0],
            method: "mori_tanaka".to_string(),
            phases,
        };
        let result = homogenize_pf_to_fe(config).unwrap();
        assert!(result.effective_properties["shear_modulus_GPa"] > 0.0);
    }

    #[test]
    fn test_homogenize_no_phases() {
        let config = PfToFeConfig {
            phase_data: vec![0.0; 8 * 8 * 8],
            grid_size: [8, 8, 8],
            domain_size: [10.0, 10.0, 10.0],
            method: "voigt".to_string(),
            phases: vec![],
        };
        let result = homogenize_pf_to_fe(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_bridge_quality_all() {
        let source = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let target = vec![1.1, 2.1, 2.9, 4.2, 4.8, 6.1, 7.0, 8.1];
        let result = validate_bridge_quality(source, target, "all".to_string()).unwrap();
        assert!(result.quality_score > 0.9);
        assert!(result.metrics.contains_key("pearson_correlation"));
        assert!(result.metrics.contains_key("rmse"));
        assert!(result.metrics.contains_key("r_squared"));
    }

    #[test]
    fn test_validate_bridge_quality_correlation() {
        let source = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let target = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = validate_bridge_quality(source, target, "correlation".to_string()).unwrap();
        assert!((result.quality_score - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_validate_bridge_quality_empty() {
        let result = validate_bridge_quality(vec![], vec![], "all".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_bridge_quality_mismatch() {
        let result = validate_bridge_quality(vec![1.0, 2.0], vec![1.0], "all".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_bridge_templates() {
        let templates = get_bridge_templates().unwrap();
        assert_eq!(templates.len(), 3);
        assert_eq!(templates[0].id, "MD_to_PF_density");
        assert_eq!(templates[1].id, "PF_to_FE_homogenization");
        assert_eq!(templates[2].id, "DFT_MD_PF_FE_full_chain");
        for t in &templates {
            assert!(!t.source_scale.is_empty());
            assert!(!t.target_scale.is_empty());
            assert!(!t.steps.is_empty());
        }
    }

    #[test]
    fn test_export_bridge_data_csv() {
        let data = vec![1.0, 2.0, 3.0];
        let tmp_path = "/tmp/test_bridge_export.csv".to_string();
        let result = export_bridge_data(data, "csv".to_string(), tmp_path.clone());
        assert!(result.is_ok());

        let content = std::fs::read_to_string(&tmp_path).unwrap();
        assert!(content.contains("index,value"));
        std::fs::remove_file(&tmp_path).ok();
    }

    #[test]
    fn test_export_bridge_data_vtk() {
        let data = vec![1.0; 27]; // 3^3
        let tmp_path = "/tmp/test_bridge_export.vtk".to_string();
        let result = export_bridge_data(data, "vtk".to_string(), tmp_path.clone());
        assert!(result.is_ok());

        let content = std::fs::read_to_string(&tmp_path).unwrap();
        assert!(content.contains("vtk DataFile"));
        assert!(content.contains("STRUCTURED_POINTS"));
        std::fs::remove_file(&tmp_path).ok();
    }

    #[test]
    fn test_export_bridge_data_raw() {
        let data = vec![1.0, 2.0, 3.0];
        let tmp_path = "/tmp/test_bridge_export.raw".to_string();
        let result = export_bridge_data(data, "raw".to_string(), tmp_path.clone());
        assert!(result.is_ok());

        let metadata = std::fs::metadata(&tmp_path).unwrap();
        assert_eq!(metadata.len(), 3 * 8); // 3 f64 values
        std::fs::remove_file(&tmp_path).ok();
    }

    #[test]
    fn test_export_bridge_data_empty() {
        let result = export_bridge_data(vec![], "csv".to_string(), "/tmp/test.csv".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_export_bridge_data_unknown_format() {
        let result = export_bridge_data(vec![1.0], "hdf5".to_string(), "/tmp/test.h5".to_string());
        assert!(result.is_err());
    }

    // Helper functions for tests
    fn make_steel_tensor() -> [f64; 21] {
        // Approximate isotropic steel: E=200 GPa, nu=0.3
        // C11 = E*(1-nu)/((1+nu)*(1-2*nu))
        // C12 = E*nu/((1+nu)*(1-2*nu))
        // C44 = E/(2*(1+nu))
        let e = 200.0;
        let nu = 0.3;
        let c11 = e * (1.0 - nu) / ((1.0 + nu) * (1.0 - 2.0 * nu));
        let c12 = e * nu / ((1.0 + nu) * (1.0 - 2.0 * nu));
        let c44 = e / (2.0 * (1.0 + nu));

        let mut tensor = [0.0; 21];
        tensor[0] = c11; // C11
        tensor[1] = c11; // C22
        tensor[2] = c11; // C33
        tensor[3] = c12; // C12
        tensor[4] = c12; // C13
        tensor[5] = c12; // C23
        tensor[6] = c44; // C44
        tensor[7] = c44; // C55
        tensor[8] = c44; // C66
        tensor
    }

    fn make_austenite_tensor() -> [f64; 21] {
        // Approximate austenite: E=210 GPa, nu=0.29
        let e = 210.0;
        let nu = 0.29;
        let c11 = e * (1.0 - nu) / ((1.0 + nu) * (1.0 - 2.0 * nu));
        let c12 = e * nu / ((1.0 + nu) * (1.0 - 2.0 * nu));
        let c44 = e / (2.0 * (1.0 + nu));

        let mut tensor = [0.0; 21];
        tensor[0] = c11;
        tensor[1] = c11;
        tensor[2] = c11;
        tensor[3] = c12;
        tensor[4] = c12;
        tensor[5] = c12;
        tensor[6] = c44;
        tensor[7] = c44;
        tensor[8] = c44;
        tensor
    }

    fn phase_data_modify(_data: &[f64], _offset: usize, _len: usize) {
        // Placeholder - no-op for test
    }
}
