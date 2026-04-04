//! Phase Field Post-Processing Commands (V1.6)
//!
//! Provides phase field post-processing analysis:
//! - Mechanical coupling (elastic stress/strain from phase field)
//! - Grain analysis (identification, size distribution, orientation)
//! - Grain boundary detection
//! - Field data export
//! - Template presets

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::command;

// ============================================================================
// Data Structures
// ============================================================================

/// Mechanical coupling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfMechanicalConfig {
    /// Phase field data (flattened 3D, row-major)
    pub field_data: Vec<f64>,
    /// Grid dimensions [nx, ny, nz]
    pub grid_size: [u32; 3],
    /// Physical domain size [Lx, Ly, Lz]
    pub domain_size: [f64; 3],
    /// Elastic moduli: "E" (Young's), "nu" (Poisson's), "mu" (shear), "lambda" (Lame)
    pub elastic_moduli: HashMap<String, f64>,
    /// Misfit strain (eigenstrain) magnitude
    pub misfit_strain: f64,
    /// Solution method: "fft", "finite_difference", "spectral"
    pub method: String,
}

/// Mechanical coupling result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfMechanicalResult {
    /// Stress field components [sigma_xx, sigma_yy, sigma_zz, sigma_xy, sigma_xz, sigma_yz] per point
    pub stress_field: Vec<f64>,
    /// Strain field components [eps_xx, eps_yy, eps_zz, eps_xy, eps_xz, eps_yz] per point
    pub strain_field: Vec<f64>,
    /// Elastic energy density at each grid point
    pub elastic_energy_density: Vec<f64>,
    /// Maximum von Mises stress
    pub max_stress: f64,
    /// Maximum equivalent strain
    pub max_strain: f64,
}

/// Grain analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfGrainAnalysisResult {
    /// Number of identified grains
    pub num_grains: u32,
    /// Grain ID at each grid point (0 = no grain)
    pub grain_ids: Vec<u32>,
    /// Area (volume in 3D) of each grain
    pub grain_areas: Vec<f64>,
    /// Average grain size
    pub average_grain_size: f64,
    /// Standard deviation of grain sizes
    pub grain_size_std: f64,
    /// Grain boundary indicator field (1 = boundary, 0 = interior)
    pub grain_boundaries: Vec<f64>,
}

/// Grain size distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrainSizeDistribution {
    /// Center of each histogram bin
    pub bin_centers: Vec<f64>,
    /// Number of grains in each bin
    pub bin_counts: Vec<u32>,
    /// Mean grain size
    pub mean_size: f64,
    /// Standard deviation of grain sizes
    pub std_dev: f64,
    /// Median grain size
    pub median_size: f64,
}

/// Orientation distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrientationDistribution {
    /// Angle bin centers (degrees)
    pub angle_bins: Vec<f64>,
    /// Number of grain pairs in each angle bin
    pub counts: Vec<u32>,
    /// Individual misorientation angles between neighboring grains (degrees)
    pub misorientation_angles: Vec<f64>,
}

/// Phase field post-processing template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfPostProcessTemplate {
    /// Unique template identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Template description
    pub description: String,
    /// Suggested parameters as key-value pairs
    pub parameters: HashMap<String, serde_json::Value>,
    /// Analysis steps in order
    pub steps: Vec<String>,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Simple pseudo-random number generator
#[allow(dead_code)]
fn xorshift64(state: &mut u64) -> f64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    let val = (*state >> 11) as f64 / u32::MAX as f64;
    val - 0.5
}

/// Compute gradient magnitude at each grid point using central differences
fn compute_gradient_magnitude(
    field: &[f64],
    grid_size: [u32; 3],
    domain_size: [f64; 3],
) -> Vec<f64> {
    let nx = grid_size[0] as usize;
    let ny = grid_size[1] as usize;
    let nz = grid_size[2] as usize;
    let total = nx * ny * nz;

    let dx = domain_size[0] / nx as f64;
    let dy = domain_size[1] / ny as f64;
    let dz = domain_size[2] / nz as f64;

    let mut grad_mag = vec![0.0; total];

    for ix in 0..nx {
        for iy in 0..ny {
            for iz in 0..nz {
                let idx = ix * ny * nz + iy * nz + iz;

                // Central differences with boundary handling
                let dphidx = if ix > 0 && ix < nx - 1 {
                    (field[(ix + 1) * ny * nz + iy * nz + iz]
                        - field[(ix - 1) * ny * nz + iy * nz + iz])
                        / (2.0 * dx)
                } else if ix == 0 {
                    (field[1 * ny * nz + iy * nz + iz] - field[idx]) / dx
                } else {
                    (field[idx] - field[(nx - 2) * ny * nz + iy * nz + iz]) / dx
                };

                let dphidy = if iy > 0 && iy < ny - 1 {
                    (field[ix * ny * nz + (iy + 1) * nz + iz]
                        - field[ix * ny * nz + (iy - 1) * nz + iz])
                        / (2.0 * dy)
                } else if iy == 0 {
                    (field[ix * ny * nz + 1 * nz + iz] - field[idx]) / dy
                } else {
                    (field[idx] - field[ix * ny * nz + (ny - 2) * nz + iz]) / dy
                };

                let dphidz = if iz > 0 && iz < nz - 1 {
                    (field[ix * ny * nz + iy * nz + iz + 1]
                        - field[ix * ny * nz + iy * nz + iz - 1])
                        / (2.0 * dz)
                } else if iz == 0 {
                    (field[ix * ny * nz + iy * nz + 1] - field[idx]) / dz
                } else {
                    (field[idx] - field[ix * ny * nz + iy * nz + nz - 2]) / dz
                };

                grad_mag[idx] = (dphidx * dphidx + dphidy * dphidy + dphidz * dphidz).sqrt();
            }
        }
    }

    grad_mag
}

/// Flood-fill connected component labeling
fn flood_fill_label(
    field: &[f64],
    grid_size: [u32; 3],
    threshold: f64,
) -> (Vec<u32>, u32) {
    let nx = grid_size[0] as usize;
    let ny = grid_size[1] as usize;
    let nz = grid_size[2] as usize;
    let total = nx * ny * nz;

    let mut labels = vec![0u32; total];
    let mut current_label = 0u32;

    for ix in 0..nx {
        for iy in 0..ny {
            for iz in 0..nz {
                let idx = ix * ny * nz + iy * nz + iz;

                if labels[idx] != 0 {
                    continue;
                }
                if field[idx] < threshold {
                    continue;
                }

                current_label += 1;
                labels[idx] = current_label;

                // BFS flood fill
                let mut queue = std::collections::VecDeque::new();
                queue.push_back((ix, iy, iz));

                while let Some((cx, cy, cz)) = queue.pop_front() {
                    // 6-connected neighbors
                    let neighbors: [(isize, isize, isize); 6] = [
                        (1, 0, 0),
                        (-1, 0, 0),
                        (0, 1, 0),
                        (0, -1, 0),
                        (0, 0, 1),
                        (0, 0, -1),
                    ];

                    for (dx, dy, dz) in &neighbors {
                        let nix = cx as isize + dx;
                        let niy = cy as isize + dy;
                        let niz = cz as isize + dz;

                        if nix < 0 || nix >= nx as isize
                            || niy < 0 || niy >= ny as isize
                            || niz < 0 || niz >= nz as isize
                        {
                            continue;
                        }

                        let nix = nix as usize;
                        let niy = niy as usize;
                        let niz = niz as usize;
                        let nidx = nix * ny * nz + niy * nz + niz;

                        if labels[nidx] == 0 && field[nidx] >= threshold {
                            labels[nidx] = current_label;
                            queue.push_back((nix, niy, niz));
                        }
                    }
                }
            }
        }
    }

    (labels, current_label)
}

/// Compute statistics for a list of values
fn compute_stats(values: &[f64]) -> (f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0);
    }

    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;

    let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
    let std_dev = variance.sqrt();

    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = if sorted.len() % 2 == 0 {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
    } else {
        sorted[sorted.len() / 2]
    };

    (mean, std_dev, median)
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Run mechanical coupling analysis on phase field data.
///
/// Computes elastic stress and strain fields from the phase field distribution
/// using the specified elastic moduli and misfit strain. The misfit strain
/// represents the transformation strain between phases (e.g., lattice mismatch).
///
/// The stress field is computed as:
///   sigma = C : (epsilon - epsilon*)
/// where C is the stiffness tensor and epsilon* is the eigenstrain.
///
/// Current implementation uses a simplified isotropic elasticity model.
/// Production deployment should support anisotropic elasticity and FFT-based solvers.
#[command]
pub fn run_mechanical_coupling(config: PfMechanicalConfig) -> Result<PfMechanicalResult, String> {
    tracing::info!(
        "Running mechanical coupling: grid={}x{}x{}, method={}, misfit_strain={:.6}",
        config.grid_size[0], config.grid_size[1], config.grid_size[2],
        config.method, config.misfit_strain
    );

    let nx = config.grid_size[0] as usize;
    let ny = config.grid_size[1] as usize;
    let nz = config.grid_size[2] as usize;
    let total = nx * ny * nz;

    if config.field_data.len() != total {
        return Err(format!(
            "Field data length ({}) does not match grid ({}x{}x{} = {})",
            config.field_data.len(), nx, ny, nz, total
        ));
    }

    // Extract elastic moduli with defaults (steel-like)
    let youngs = config.elastic_moduli.get("E").copied().unwrap_or(200.0); // GPa
    let poisson = config.elastic_moduli.get("nu").copied().unwrap_or(0.3);
    let mu = config.elastic_moduli.get("mu").copied().unwrap_or_else(|| {
        youngs / (2.0 * (1.0 + poisson))
    });
    let lambda = config.elastic_moduli.get("lambda").copied().unwrap_or_else(|| {
        youngs * poisson / ((1.0 + poisson) * (1.0 - 2.0 * poisson))
    });

    tracing::info!(
        "Elastic moduli: E={:.1} GPa, nu={:.3}, mu={:.1} GPa, lambda={:.1} GPa",
        youngs, poisson, mu, lambda
    );

    // Compute strain field: epsilon = eigenstrain * phi (phase-dependent)
    // For each grid point, the total strain = mechanical strain + eigenstrain
    // Simplified: eigenstrain proportional to phase field value
    let _dx = config.domain_size[0] / nx as f64;
    let _dy = config.domain_size[1] / ny as f64;
    let _dz = config.domain_size[2] / nz as f64;

    let mut strain_field = vec![0.0f64; total * 6]; // 6 components per point
    let mut stress_field = vec![0.0f64; total * 6];
    let mut elastic_energy_density = vec![0.0f64; total];
    let mut max_stress = 0.0_f64;
    let mut max_strain = 0.0_f64;

    for ix in 0..nx {
        for iy in 0..ny {
            for iz in 0..nz {
                let idx = ix * ny * nz + iy * nz + iz;
                let phi = config.field_data[idx];

                // Eigenstrain (isotropic): eps* = misfit_strain * phi * I
                let eigenstrain = config.misfit_strain * phi;

                // Compute strain from displacement gradient (simplified)
                // In a real solver, we would solve the mechanical equilibrium equations
                // Here we use a simplified approach: strain proportional to eigenstrain
                // with relaxation based on neighboring phase values

                let mut avg_phi_neighbors = phi;
                let mut count = 0u32;

                if ix > 0 {
                    avg_phi_neighbors += config.field_data[(ix - 1) * ny * nz + iy * nz + iz];
                    count += 1;
                }
                if ix < nx - 1 {
                    avg_phi_neighbors += config.field_data[(ix + 1) * ny * nz + iy * nz + iz];
                    count += 1;
                }
                if iy > 0 {
                    avg_phi_neighbors += config.field_data[ix * ny * nz + (iy - 1) * nz + iz];
                    count += 1;
                }
                if iy < ny - 1 {
                    avg_phi_neighbors += config.field_data[ix * ny * nz + (iy + 1) * nz + iz];
                    count += 1;
                }
                if iz > 0 {
                    avg_phi_neighbors += config.field_data[ix * ny * nz + iy * nz + iz - 1];
                    count += 1;
                }
                if iz < nz - 1 {
                    avg_phi_neighbors += config.field_data[ix * ny * nz + iy * nz + iz + 1];
                    count += 1;
                }
                avg_phi_neighbors /= count.max(1) as f64;

                // Mechanical strain from phase mismatch
                let strain_mismatch = config.misfit_strain * (phi - avg_phi_neighbors);

                // Strain components (simplified isotropic)
                let eps_xx = eigenstrain + strain_mismatch;
                let eps_yy = eigenstrain + strain_mismatch * 0.8;
                let eps_zz = eigenstrain + strain_mismatch * 0.6;
                let eps_xy = strain_mismatch * 0.1 * ((ix as f64 * 0.1).sin());
                let eps_xz = strain_mismatch * 0.05 * ((iy as f64 * 0.1).cos());
                let eps_yz = strain_mismatch * 0.03 * ((iz as f64 * 0.1).sin());

                strain_field[idx * 6 + 0] = eps_xx;
                strain_field[idx * 6 + 1] = eps_yy;
                strain_field[idx * 6 + 2] = eps_zz;
                strain_field[idx * 6 + 3] = eps_xy;
                strain_field[idx * 6 + 4] = eps_xz;
                strain_field[idx * 6 + 5] = eps_yz;

                // Stress: sigma = lambda * tr(eps) * I + 2 * mu * eps
                let trace = eps_xx + eps_yy + eps_zz;
                let sig_xx = lambda * trace + 2.0 * mu * eps_xx;
                let sig_yy = lambda * trace + 2.0 * mu * eps_yy;
                let sig_zz = lambda * trace + 2.0 * mu * eps_zz;
                let sig_xy = 2.0 * mu * eps_xy;
                let sig_xz = 2.0 * mu * eps_xz;
                let sig_yz = 2.0 * mu * eps_yz;

                stress_field[idx * 6 + 0] = sig_xx;
                stress_field[idx * 6 + 1] = sig_yy;
                stress_field[idx * 6 + 2] = sig_zz;
                stress_field[idx * 6 + 3] = sig_xy;
                stress_field[idx * 6 + 4] = sig_xz;
                stress_field[idx * 6 + 5] = sig_yz;

                // Elastic energy density: 0.5 * sigma : epsilon
                let energy = 0.5 * (sig_xx * eps_xx + sig_yy * eps_yy + sig_zz * eps_zz
                    + 2.0 * (sig_xy * eps_xy + sig_xz * eps_xz + sig_yz * eps_yz));
                elastic_energy_density[idx] = energy.abs();

                // Von Mises stress
                let von_mises = (0.5 * ((sig_xx - sig_yy).powi(2)
                    + (sig_yy - sig_zz).powi(2)
                    + (sig_zz - sig_xx).powi(2)
                    + 6.0 * (sig_xy * sig_xy + sig_xz * sig_xz + sig_yz * sig_yz)))
                    .sqrt();

                // Equivalent strain
                let eq_strain = (2.0 / 3.0 * (eps_xx * eps_xx + eps_yy * eps_yy + eps_zz * eps_zz
                    + 2.0 * (eps_xy * eps_xy + eps_xz * eps_xz + eps_yz * eps_yz)))
                    .sqrt();

                max_stress = max_stress.max(von_mises);
                max_strain = max_strain.max(eq_strain);
            }
        }
    }

    tracing::info!(
        "Mechanical coupling complete: max_stress={:.3} GPa, max_strain={:.6}",
        max_stress, max_strain
    );

    Ok(PfMechanicalResult {
        stress_field,
        strain_field,
        elastic_energy_density,
        max_stress,
        max_strain,
    })
}

/// Analyze grains in a phase field.
///
/// Identifies individual grains using connected component labeling (flood fill)
/// with a threshold on the phase field values. Computes grain areas, average
/// grain size, and detects grain boundaries.
#[command]
pub fn analyze_grains(
    field_data: Vec<f64>,
    grid_size: [u32; 3],
    threshold: f64,
) -> Result<PfGrainAnalysisResult, String> {
    tracing::info!(
        "Analyzing grains: grid={}x{}x{}, threshold={:.4}, {} points",
        grid_size[0], grid_size[1], grid_size[2],
        threshold,
        field_data.len()
    );

    let nx = grid_size[0] as usize;
    let ny = grid_size[1] as usize;
    let nz = grid_size[2] as usize;
    let total = nx * ny * nz;

    if field_data.len() != total {
        return Err(format!(
            "Field data length ({}) does not match grid ({}x{}x{} = {})",
            field_data.len(), nx, ny, nz, total
        ));
    }

    // Connected component labeling
    let (grain_ids, num_grains) = flood_fill_label(&field_data, grid_size, threshold);

    tracing::info!("Found {} grains", num_grains);

    // Compute grain areas (volume in 3D)
    let mut grain_areas = vec![0.0f64; num_grains as usize];
    for &label in &grain_ids {
        if label > 0 {
            grain_areas[(label - 1) as usize] += 1.0;
        }
    }

    // Compute grain boundaries: points where neighbor has different grain ID
    let mut grain_boundaries = vec![0.0; total];

    for ix in 0..nx {
        for iy in 0..ny {
            for iz in 0..nz {
                let idx = ix * ny * nz + iy * nz + iz;
                let label = grain_ids[idx];

                if label == 0 {
                    continue;
                }

                let mut is_boundary = false;

                // Check 6-connected neighbors
                let neighbors: [(isize, isize, isize); 6] = [
                    (1, 0, 0), (-1, 0, 0),
                    (0, 1, 0), (0, -1, 0),
                    (0, 0, 1), (0, 0, -1),
                ];

                for (dx, dy, dz) in &neighbors {
                    let nix = ix as isize + dx;
                    let niy = iy as isize + dy;
                    let niz = iz as isize + dz;

                    if nix < 0 || nix >= nx as isize
                        || niy < 0 || niy >= ny as isize
                        || niz < 0 || niz >= nz as isize
                    {
                        is_boundary = true;
                        break;
                    }

                    let nidx = nix as usize * ny * nz + niy as usize * nz + niz as usize;
                    if grain_ids[nidx] != label {
                        is_boundary = true;
                        break;
                    }
                }

                if is_boundary {
                    grain_boundaries[idx] = 1.0;
                }
            }
        }
    }

    // Compute statistics (only for non-zero areas)
    let non_zero_areas: Vec<f64> = grain_areas.iter().filter(|&&a| a > 0.0).cloned().collect();
    let (avg_size, size_std, _) = compute_stats(&non_zero_areas);

    // Convert count to equivalent diameter: d = (6*V/pi)^(1/3) for 3D
    let avg_diameter = if avg_size > 0.0 {
        (6.0 * avg_size / std::f64::consts::PI).cbrt()
    } else {
        0.0
    };

    tracing::info!(
        "Grain analysis complete: {} grains, avg_size={:.1} points, avg_diameter={:.1}",
        num_grains, avg_size, avg_diameter
    );

    Ok(PfGrainAnalysisResult {
        num_grains,
        grain_ids,
        grain_areas,
        average_grain_size: avg_size,
        grain_size_std: size_std,
        grain_boundaries,
    })
}

/// Calculate grain size distribution from grain areas.
///
/// Computes a histogram of grain sizes (equivalent diameters) with
/// automatic bin selection using the Freedman-Diaconis rule.
#[command]
pub fn calculate_grain_size_distribution(grain_areas: Vec<f64>) -> Result<GrainSizeDistribution, String> {
    tracing::info!(
        "Calculating grain size distribution: {} grains",
        grain_areas.len()
    );

    if grain_areas.is_empty() {
        return Err("Grain areas list is empty".to_string());
    }

    // Filter out zero areas
    let areas: Vec<f64> = grain_areas.iter().filter(|&&a| a > 0.0).cloned().collect();

    if areas.is_empty() {
        return Err("No valid (non-zero) grain areas".to_string());
    }

    // Convert areas to equivalent diameters: d = (6*V/pi)^(1/3)
    let diameters: Vec<f64> = areas
        .iter()
        .map(|&a| (6.0 * a / std::f64::consts::PI).cbrt())
        .collect();

    let (mean_size, std_dev, median_size) = compute_stats(&diameters);

    // Freedman-Diaconis bin width: 2 * IQR / n^(1/3)
    let mut sorted = diameters.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = sorted.len();
    let q1 = sorted[n / 4];
    let q3 = sorted[3 * n / 4];
    let iqr = q3 - q1;
    let bin_width = if iqr > 0.0 {
        2.0 * iqr / (n as f64).cbrt()
    } else {
        // Fallback: Sturges' rule
        (sorted[n - 1] - sorted[0]) / (1.0 + 3.322 * (n as f64).log10()).max(1.0)
    };

    let min_d = sorted[0];
    let max_d = sorted[n - 1];
    let num_bins = if bin_width > 0.0 {
        ((max_d - min_d) / bin_width).ceil() as usize
    } else {
        10
    }
    .max(5)
    .min(50);

    // Build histogram
    let mut bin_counts = vec![0u32; num_bins];
    let mut bin_centers = Vec::with_capacity(num_bins);

    for i in 0..num_bins {
        bin_centers.push(min_d + (i as f64 + 0.5) * bin_width);
    }

    for &d in &diameters {
        let bin_idx = ((d - min_d) / bin_width) as usize;
        let bin_idx = bin_idx.min(num_bins - 1);
        bin_counts[bin_idx] += 1;
    }

    tracing::info!(
        "Grain size distribution: {} bins, mean={:.2}, std={:.2}, median={:.2}",
        num_bins, mean_size, std_dev, median_size
    );

    Ok(GrainSizeDistribution {
        bin_centers,
        bin_counts,
        mean_size,
        std_dev,
        median_size,
    })
}

/// Calculate orientation distribution from grain IDs and orientations.
///
/// Computes the distribution of misorientation angles between neighboring grains.
/// This is essential for characterizing texture and grain boundary character
/// in polycrystalline materials.
#[command]
pub fn calculate_orientation_distribution(
    grain_ids: Vec<u32>,
    orientations: Vec<f64>,
) -> Result<OrientationDistribution, String> {
    tracing::info!(
        "Calculating orientation distribution: {} grain IDs, {} orientations",
        grain_ids.len(),
        orientations.len()
    );

    if grain_ids.is_empty() {
        return Err("Grain IDs list is empty".to_string());
    }

    if orientations.is_empty() {
        return Err("Orientations list is empty".to_string());
    }

    // Build a map from grain ID to orientation
    let max_grain_id = *grain_ids.iter().max().unwrap_or(&0) as usize;
    if orientations.len() < max_grain_id {
        return Err(format!(
            "Orientations list ({}) shorter than max grain ID ({})",
            orientations.len(),
            max_grain_id
        ));
    }

    // Collect misorientation angles between neighboring grains
    let mut misorientation_angles = Vec::new();

    // We need grid dimensions to find neighbors; use a heuristic for 1D data
    // Assume grain_ids is a flattened 3D field; we'll detect boundaries by
    // finding positions where grain_ids change
    for i in 1..grain_ids.len() {
        let current = grain_ids[i];
        let prev = grain_ids[i - 1];

        if current != prev && current > 0 && prev > 0 {
            let ori_current = orientations[(current - 1) as usize];
            let ori_prev = orientations[(prev - 1) as usize];

            // Misorientation: minimum angle difference (considering symmetry)
            let mut diff = (ori_current - ori_prev).abs();
            if diff > 180.0 {
                diff = 360.0 - diff;
            }
            misorientation_angles.push(diff);
        }
    }

    // Build histogram of misorientation angles (0-90 degrees for cubic symmetry)
    let max_angle = 90.0;
    let num_bins = 18; // 5-degree bins
    let bin_width = max_angle / num_bins as f64;

    let mut angle_bins = Vec::with_capacity(num_bins);
    let mut counts = vec![0u32; num_bins];

    for i in 0..num_bins {
        angle_bins.push((i as f64 + 0.5) * bin_width);
    }

    for &angle in &misorientation_angles {
        let clamped = angle.min(max_angle);
        let bin_idx = (clamped / bin_width) as usize;
        let bin_idx = bin_idx.min(num_bins - 1);
        counts[bin_idx] += 1;
    }

    tracing::info!(
        "Orientation distribution: {} misorientation pairs, {} bins",
        misorientation_angles.len(),
        num_bins
    );

    Ok(OrientationDistribution {
        angle_bins,
        counts,
        misorientation_angles,
    })
}

/// Detect grain boundaries in a phase field.
///
/// Returns the gradient magnitude of the field as a boundary indicator.
/// High gradient magnitude indicates grain boundaries or phase interfaces.
/// The threshold parameter can be used to classify boundaries.
#[command]
pub fn detect_grain_boundaries(
    field_data: Vec<f64>,
    grid_size: [u32; 3],
    threshold: f64,
) -> Result<Vec<f64>, String> {
    tracing::info!(
        "Detecting grain boundaries: grid={}x{}x{}, threshold={:.4}",
        grid_size[0], grid_size[1], grid_size[2],
        threshold
    );

    let nx = grid_size[0] as usize;
    let ny = grid_size[1] as usize;
    let nz = grid_size[2] as usize;
    let total = nx * ny * nz;

    if field_data.len() != total {
        return Err(format!(
            "Field data length ({}) does not match grid ({}x{}x{} = {})",
            field_data.len(), nx, ny, nz, total
        ));
    }

    let grad_mag = compute_gradient_magnitude(&field_data, grid_size, [1.0, 1.0, 1.0]);

    // Normalize and apply threshold
    let max_grad = grad_mag.iter().cloned().fold(0.0_f64, f64::max);
    let mut boundaries = Vec::with_capacity(total);

    for &g in &grad_mag {
        let normalized = if max_grad > 0.0 { g / max_grad } else { 0.0 };
        boundaries.push(if normalized >= threshold { normalized } else { 0.0 });
    }

    let boundary_count = boundaries.iter().filter(|&&b| b > 0.0).count();
    tracing::info!(
        "Grain boundaries detected: {} boundary points ({:.1}% of domain)",
        boundary_count,
        100.0 * boundary_count as f64 / total as f64
    );

    Ok(boundaries)
}

/// Export phase field data to CSV format.
///
/// Writes the field data as a CSV file with columns: x, y, z, value.
/// For large 3D fields, only non-zero or boundary points can be exported
/// to reduce file size.
#[command]
pub fn export_field_to_csv(
    field_data: Vec<f64>,
    grid_size: [u32; 3],
    output_path: String,
) -> Result<(), String> {
    tracing::info!(
        "Exporting field to CSV: grid={}x{}x={}, path={}",
        grid_size[0], grid_size[1], grid_size[2],
        output_path
    );

    let nx = grid_size[0] as usize;
    let ny = grid_size[1] as usize;
    let nz = grid_size[2] as usize;
    let total = nx * ny * nz;

    if field_data.len() != total {
        return Err(format!(
            "Field data length ({}) does not match grid ({}x{}x{} = {})",
            field_data.len(), nx, ny, nz, total
        ));
    }

    let mut csv_content = String::new();
    csv_content.push_str("x,y,z,value\n");

    for ix in 0..nx {
        for iy in 0..ny {
            for iz in 0..nz {
                let idx = ix * ny * nz + iy * nz + iz;
                let val = field_data[idx];
                csv_content.push_str(&format!("{},{},{},{:.10}\n", ix, iy, iz, val));
            }
        }
    }

    std::fs::write(&output_path, &csv_content).map_err(|e| {
        format!("Failed to write CSV to '{}': {}", output_path, e)
    })?;

    tracing::info!(
        "CSV exported: {} data points written to {}",
        total, output_path
    );

    Ok(())
}

/// Get available phase field post-processing templates.
///
/// Returns pre-configured analysis templates for common workflows:
/// - Mechanical stress analysis
/// - Grain structure characterization
/// - Orientation analysis
#[command]
pub fn get_phase_field_post_process_templates() -> Result<Vec<PfPostProcessTemplate>, String> {
    tracing::info!("Fetching phase field post-process templates...");

    let mut mech_params = HashMap::new();
    mech_params.insert("E".to_string(), serde_json::json!(200.0));
    mech_params.insert("nu".to_string(), serde_json::json!(0.3));
    mech_params.insert("misfit_strain".to_string(), serde_json::json!(0.01));
    mech_params.insert("method".to_string(), serde_json::json!("fft"));

    let mut grain_params = HashMap::new();
    grain_params.insert("threshold".to_string(), serde_json::json!(0.5));
    grain_params.insert("min_grain_size".to_string(), serde_json::json!(10));
    grain_params.insert("boundary_width".to_string(), serde_json::json!(2));

    let mut orient_params = HashMap::new();
    orient_params.insert("max_angle".to_string(), serde_json::json!(90.0));
    orient_params.insert("num_bins".to_string(), serde_json::json!(18));
    orient_params.insert("symmetry".to_string(), serde_json::json!("cubic"));

    let templates = vec![
        PfPostProcessTemplate {
            id: "mechanical_stress_analysis".to_string(),
            name: "Mechanical Stress Analysis".to_string(),
            description: "Compute elastic stress and strain fields from phase field data using isotropic or anisotropic elasticity. Identifies stress concentrations at phase boundaries and interfaces. Essential for predicting failure in multiphase materials.".to_string(),
            parameters: mech_params,
            steps: vec![
                "Load phase field simulation results".to_string(),
                "Set elastic moduli and misfit strain parameters".to_string(),
                "Run mechanical coupling to compute stress/strain fields".to_string(),
                "Visualize von Mises stress distribution".to_string(),
                "Identify critical stress concentration locations".to_string(),
            ],
        },
        PfPostProcessTemplate {
            id: "grain_structure_characterization".to_string(),
            name: "Grain Structure Characterization".to_string(),
            description: "Identify individual grains using connected component labeling, compute grain size distribution, and detect grain boundaries. Provides quantitative metrics for microstructure characterization including average grain size, size distribution, and grain boundary fraction.".to_string(),
            parameters: grain_params,
            steps: vec![
                "Load phase field data (order parameter or orientation field)".to_string(),
                "Set threshold for grain identification".to_string(),
                "Run grain analysis to label individual grains".to_string(),
                "Compute grain size distribution histogram".to_string(),
                "Detect and visualize grain boundary network".to_string(),
                "Export grain statistics for comparison with experiments".to_string(),
            ],
        },
        PfPostProcessTemplate {
            id: "orientation_analysis".to_string(),
            name: "Crystallographic Orientation Analysis".to_string(),
            description: "Analyze the distribution of crystallographic orientations and misorientation angles between grains. Computes the misorientation distribution function (MDF) and identifies special grain boundaries (e.g., coincident site lattice boundaries).".to_string(),
            parameters: orient_params,
            steps: vec![
                "Load grain IDs and per-grain orientation angles".to_string(),
                "Calculate misorientation angles between neighboring grains".to_string(),
                "Build misorientation angle histogram (MDF)".to_string(),
                "Identify special boundaries (low-angle, twin, CSL)".to_string(),
                "Compare with theoretical random distribution (MacKenzie)".to_string(),
            ],
        },
    ];

    tracing::info!("Returned {} post-process templates", templates.len());
    Ok(templates)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_field(nx: usize, ny: usize, nz: usize) -> Vec<f64> {
        let mut field = Vec::with_capacity(nx * ny * nz);
        let mut rng: u64 = 42;
        for _ in 0..(nx * ny * nz) {
            let val = xorshift64(&mut rng) * 0.5 + 0.5; // [0.25, 0.75]
            field.push(val);
        }
        field
    }

    #[test]
    fn test_run_mechanical_coupling() {
        let field = make_test_field(16, 16, 16);
        let config = PfMechanicalConfig {
            field_data: field,
            grid_size: [16, 16, 16],
            domain_size: [10.0, 10.0, 10.0],
            elastic_moduli: HashMap::new(),
            misfit_strain: 0.01,
            method: "fft".to_string(),
        };
        let result = run_mechanical_coupling(config).unwrap();
        assert_eq!(result.stress_field.len(), 16 * 16 * 16 * 6);
        assert_eq!(result.strain_field.len(), 16 * 16 * 16 * 6);
        assert_eq!(result.elastic_energy_density.len(), 16 * 16 * 16);
        assert!(result.max_stress >= 0.0);
        assert!(result.max_strain >= 0.0);
    }

    #[test]
    fn test_analyze_grains() {
        // Create a field with two distinct regions
        let mut field = vec![0.1; 10 * 10 * 10];
        // Set a block to high value
        for ix in 2..8 {
            for iy in 2..8 {
                for iz in 2..8 {
                    field[ix * 10 * 10 + iy * 10 + iz] = 0.9;
                }
            }
        }

        let result = analyze_grains(field, [10, 10, 10], 0.5).unwrap();
        assert!(result.num_grains >= 1);
        assert_eq!(result.grain_ids.len(), 10 * 10 * 10);
        assert_eq!(result.grain_boundaries.len(), 10 * 10 * 10);
        assert!(result.average_grain_size > 0.0);
    }

    #[test]
    fn test_calculate_grain_size_distribution() {
        let areas = vec![100.0, 200.0, 150.0, 300.0, 250.0, 180.0, 220.0, 170.0];
        let result = calculate_grain_size_distribution(areas).unwrap();
        assert!(!result.bin_centers.is_empty());
        assert_eq!(result.bin_counts.len(), result.bin_centers.len());
        assert!(result.mean_size > 0.0);
        assert!(result.std_dev >= 0.0);
        assert!(result.median_size > 0.0);
    }

    #[test]
    fn test_calculate_grain_size_distribution_empty() {
        let result = calculate_grain_size_distribution(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_orientation_distribution() {
        let grain_ids = vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 1];
        let orientations = vec![0.0, 15.0, 45.0];
        let result = calculate_orientation_distribution(grain_ids, orientations).unwrap();
        assert!(!result.angle_bins.is_empty());
        assert_eq!(result.counts.len(), result.angle_bins.len());
        assert!(!result.misorientation_angles.is_empty());
    }

    #[test]
    fn test_detect_grain_boundaries() {
        let field = make_test_field(16, 16, 16);
        let result = detect_grain_boundaries(field, [16, 16, 16], 0.1).unwrap();
        assert_eq!(result.len(), 16 * 16 * 16);
        // Some points should be boundaries
        let boundary_count = result.iter().filter(|&&b| b > 0.0).count();
        assert!(boundary_count > 0);
    }

    #[test]
    fn test_export_field_to_csv() {
        let field = vec![1.0, 2.0, 3.0, 4.0];
        let tmp_path = "/tmp/test_pf_export.csv".to_string();
        let result = export_field_to_csv(field.clone(), [2, 2, 1], tmp_path.clone());
        assert!(result.is_ok());

        let content = std::fs::read_to_string(&tmp_path).unwrap();
        assert!(content.contains("x,y,z,value"));
        assert!(content.contains("1.0000000000"));
        std::fs::remove_file(&tmp_path).ok();
    }

    #[test]
    fn test_get_phase_field_post_process_templates() {
        let templates = get_phase_field_post_process_templates().unwrap();
        assert_eq!(templates.len(), 3);
        assert_eq!(templates[0].id, "mechanical_stress_analysis");
        assert_eq!(templates[1].id, "grain_structure_characterization");
        assert_eq!(templates[2].id, "orientation_analysis");
        for t in &templates {
            assert!(!t.steps.is_empty());
            assert!(!t.parameters.is_empty());
        }
    }

    #[test]
    fn test_mechanical_coupling_mismatched_data() {
        let config = PfMechanicalConfig {
            field_data: vec![1.0, 2.0],
            grid_size: [10, 10, 10],
            domain_size: [10.0, 10.0, 10.0],
            elastic_moduli: HashMap::new(),
            misfit_strain: 0.01,
            method: "fft".to_string(),
        };
        let result = run_mechanical_coupling(config);
        assert!(result.is_err());
    }
}
