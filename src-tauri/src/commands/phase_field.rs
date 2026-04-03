//! Phase Field Simulation Commands (V1.6)
//!
//! Provides phase field modeling capabilities:
//! - Cahn-Hilliard (CH) spinodal decomposition
//! - Allen-Cahn (AC) grain growth
//! - Phase Field Crystal (PFC) dendritic solidification
//! - Karma-type solidification models
//!
//! Supports multiple free energy functionals, time integration schemes,
//! and initial condition generators.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::command;

// ============================================================================
// Data Structures
// ============================================================================

/// Phase field simulation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfConfig {
    /// Equation type: "CH", "AC", "PFC", "Karma"
    pub equation_type: String,
    /// Free energy functional: "double_well", "polynomial", "landau", "flory_huggins"
    pub free_energy_type: String,
    /// Time integration scheme: "explicit", "implicit", "semi_implicit", "rk4"
    pub time_integration: String,
    /// Grid dimensions [nx, ny, nz]
    pub grid_size: [u32; 3],
    /// Physical domain size [Lx, Ly, Lz]
    pub domain_size: [f64; 3],
    /// Time step
    pub dt: f64,
    /// Number of time steps
    pub num_steps: u32,
    /// Initial condition type: "random", "nucleation", "interface", "custom"
    pub initial_condition: String,
    /// Simulation-specific parameters
    pub parameters: HashMap<String, f64>,
    /// Output interval (steps between saves)
    pub output_interval: u32,
}

/// Phase field simulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfResult {
    pub success: bool,
    /// Flattened 3D field data (row-major order)
    pub field_data: Vec<f64>,
    /// Grid dimensions
    pub grid_size: [u32; 3],
    /// Final simulation time
    pub final_time: f64,
    /// Free energy history at each output step
    pub free_energy: Vec<f64>,
    /// Number of steps actually completed
    pub num_steps_completed: u32,
    /// Wall-clock computation time in seconds
    pub computation_time_s: f64,
    /// Path to output files
    pub output_path: String,
}

/// Initial condition generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfInitConfig {
    /// Grid dimensions [nx, ny, nz]
    pub grid_size: [u32; 3],
    /// Physical domain size [Lx, Ly, Lz]
    pub domain_size: [f64; 3],
    /// Condition type: "random", "nucleation", "interface", "custom"
    pub condition_type: String,
    /// Condition-specific parameters
    pub parameters: HashMap<String, f64>,
}

/// Phase field simulation template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub equation_type: String,
    pub free_energy_type: String,
    pub default_grid_size: [u32; 3],
    pub default_domain_size: [f64; 3],
    pub default_dt: f64,
    pub default_steps: u32,
    pub default_parameters: HashMap<String, f64>,
    pub difficulty: String,
}

/// Phase field configuration validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Phase field memory estimation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfMemoryEstimate {
    /// Estimated memory usage in MB
    pub estimated_mb: f64,
    /// Recommended system RAM
    pub recommended_ram: String,
    /// Estimated disk usage for output in MB
    pub disk_estimate_mb: f64,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Simple pseudo-random number generator (xorshift64)
fn xorshift64(state: &mut u64) -> f64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    let val = (*state >> 11) as f64 / u32::MAX as f64;
    val - 0.5
}

/// Generate a timestamp string for output file naming
fn timestamp_str() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

/// Validate equation type string
fn is_valid_equation_type(eq: &str) -> bool {
    matches!(eq.to_uppercase().as_str(), "CH" | "AC" | "PFC" | "KARMA")
}

/// Validate free energy type string
fn is_valid_free_energy_type(fe: &str) -> bool {
    matches!(
        fe.to_lowercase().as_str(),
        "double_well" | "polynomial" | "landau" | "flory_huggins"
    )
}

/// Validate time integration scheme
fn is_valid_time_integration(ti: &str) -> bool {
    matches!(
        ti.to_lowercase().as_str(),
        "explicit" | "implicit" | "semi_implicit" | "rk4"
    )
}

/// Validate initial condition type
fn is_valid_initial_condition(ic: &str) -> bool {
    matches!(
        ic.to_lowercase().as_str(),
        "random" | "nucleation" | "interface" | "custom"
    )
}

/// Compute double-well free energy density: f(phi) = A * phi^2 * (1 - phi)^2
fn double_well_energy(phi: f64, a: f64) -> f64 {
    a * phi * phi * (1.0 - phi).powi(2)
}

/// Compute Landau free energy density: f(phi) = a2*phi^2 + a4*phi^4 + a6*phi^6
fn landau_energy(phi: f64, a2: f64, a4: f64, a6: f64) -> f64 {
    a2 * phi * phi + a4 * phi.powi(4) + a6 * phi.powi(6)
}

/// Compute Flory-Huggins free energy density:
/// f(phi) = (phi/N1)*ln(phi) + ((1-phi)/N2)*ln(1-phi) + chi*phi*(1-phi)
fn flory_huggins_energy(phi: f64, n1: f64, n2: f64, chi: f64) -> f64 {
    let eps = 1e-12;
    let phi_c = phi.max(eps).min(1.0 - eps);
    (phi_c / n1) * phi_c.ln() + ((1.0 - phi_c) / n2) * (1.0 - phi_c).ln() + chi * phi_c * (1.0 - phi_c)
}

/// Compute polynomial free energy density: f(phi) = sum_k a_k * phi^k
fn polynomial_energy(phi: f64, coeffs: &[f64]) -> f64 {
    let mut result = 0.0;
    let mut phi_pow = 1.0;
    for &a in coeffs {
        result += a * phi_pow;
        phi_pow *= phi;
    }
    result
}

/// Generate mock phase field evolution using simplified Cahn-Hilliard dynamics
fn generate_mock_ch_evolution(
    grid_size: [u32; 3],
    domain_size: [f64; 3],
    dt: f64,
    num_steps: u32,
    output_interval: u32,
    parameters: &HashMap<String, f64>,
    init_field: &[f64],
) -> PfResult {
    let nx = grid_size[0] as usize;
    let ny = grid_size[1] as usize;
    let nz = grid_size[2] as usize;
    let _total = nx * ny * nz;

    // Extract parameters with defaults
    let mobility = parameters.get("mobility").copied().unwrap_or(1.0);
    let kappa = parameters.get("kappa").copied().unwrap_or(0.5);
    let a_well = parameters.get("A").copied().unwrap_or(1.0);

    // Grid spacing
    let dx = domain_size[0] / nx as f64;
    let dy = domain_size[1] / ny as f64;
    let dz = domain_size[2] / nz as f64;

    let start_time = std::time::Instant::now();

    // Initialize field
    let mut field = init_field.to_vec();
    let mut free_energy_history = Vec::new();

    // Simplified CH evolution: dphi/dt = M * laplacian(mu), mu = df/dphi - kappa * laplacian(phi)
    // For mock purposes, we apply a smoothing + spinodal decomposition effect
    let num_outputs = (num_steps / output_interval.max(1)) as usize + 1;

    for step in 0..num_steps {
        // Apply a simplified update: slight spinodal decomposition tendency
        let mut new_field = field.clone();

        for ix in 1..nx - 1 {
            for iy in 1..ny - 1 {
                for iz in 1..nz - 1 {
                    let idx = ix * ny * nz + iy * nz + iz;
                    let phi = field[idx];

                    // Laplacian of phi (finite differences)
                    let lap_phi = (field[(ix + 1) * ny * nz + iy * nz + iz]
                        + field[(ix - 1) * ny * nz + iy * nz + iz]
                        - 2.0 * phi)
                        / (dx * dx)
                        + (field[ix * ny * nz + (iy + 1) * nz + iz]
                            + field[ix * ny * nz + (iy - 1) * nz + iz]
                            - 2.0 * phi)
                            / (dy * dy)
                        + (field[ix * ny * nz + iy * nz + iz + 1]
                            + field[ix * ny * nz + iy * nz + iz - 1]
                            - 2.0 * phi)
                            / (dz * dz);

                    // Chemical potential: mu = df/dphi - kappa * laplacian(phi)
                    // df/dphi for double-well: A * 2*phi*(1-phi)*(1-2*phi)
                    let df_dphi = a_well * 2.0 * phi * (1.0 - phi) * (1.0 - 2.0 * phi);
                    let mu = df_dphi - kappa * lap_phi;

                    // Laplacian of mu
                    let lap_mu_approx = -6.0 * mu; // Simplified approximation

                    // Update: dphi/dt = M * laplacian(mu)
                    let dphi = mobility * lap_mu_approx * dt;
                    new_field[idx] = (phi + dphi).clamp(0.0, 1.0);
                }
            }
        }

        field = new_field;

        // Record free energy at output intervals
        if step % output_interval.max(1) == 0 {
            let mut total_fe = 0.0;
            for &phi in &field {
                total_fe += double_well_energy(phi, a_well);
            }
            total_fe *= dx * dy * dz;
            free_energy_history.push(total_fe);
        }
    }

    let computation_time = start_time.elapsed().as_secs_f64();

    // Ensure we have the right number of energy outputs
    while free_energy_history.len() < num_outputs {
        free_energy_history.push(free_energy_history.last().copied().unwrap_or(0.0));
    }

    PfResult {
        success: true,
        field_data: field,
        grid_size,
        final_time: num_steps as f64 * dt,
        free_energy: free_energy_history,
        num_steps_completed: num_steps,
        computation_time_s: computation_time,
        output_path: format!("/tmp/pf_ch_output_{}.dat", timestamp_str()),
    }
}

/// Generate mock AC (Allen-Cahn) evolution
fn generate_mock_ac_evolution(
    grid_size: [u32; 3],
    domain_size: [f64; 3],
    dt: f64,
    num_steps: u32,
    output_interval: u32,
    parameters: &HashMap<String, f64>,
    init_field: &[f64],
) -> PfResult {
    let nx = grid_size[0] as usize;
    let ny = grid_size[1] as usize;
    let nz = grid_size[2] as usize;
    let _total = nx * ny * nz;

    let _kappa = parameters.get("kappa").copied().unwrap_or(0.5);
    let epsilon = parameters.get("epsilon").copied().unwrap_or(0.01);
    let dx = domain_size[0] / nx as f64;

    let start_time = std::time::Instant::now();
    let mut field = init_field.to_vec();
    let mut free_energy_history = Vec::new();

    // Allen-Cahn: dphi/dt = -epsilon^2 * laplacian(phi) + f'(phi)
    for step in 0..num_steps {
        let mut new_field = field.clone();

        for ix in 1..nx - 1 {
            for iy in 1..ny - 1 {
                for iz in 1..nz - 1 {
                    let idx = ix * ny * nz + iy * nz + iz;
                    let phi = field[idx];

                    let lap_phi = (field[(ix + 1) * ny * nz + iy * nz + iz]
                        + field[(ix - 1) * ny * nz + iy * nz + iz]
                        + field[ix * ny * nz + (iy + 1) * nz + iz]
                        + field[ix * ny * nz + (iy - 1) * nz + iz]
                        + field[ix * ny * nz + iy * nz + iz + 1]
                        + field[ix * ny * nz + iy * nz + iz - 1]
                        - 6.0 * phi)
                        / (dx * dx);

                    // Double-well derivative: phi*(1-phi)*(1-2*phi)
                    let df_dphi = phi * (1.0 - phi) * (1.0 - 2.0 * phi);

                    let dphi = (-epsilon * epsilon * lap_phi + df_dphi) * dt;
                    new_field[idx] = (phi + dphi).clamp(-1.0, 1.0);
                }
            }
        }

        field = new_field;

        if step % output_interval.max(1) == 0 {
            let mut total_fe = 0.0;
            for &phi in &field {
                total_fe += 0.25 * (1.0 - phi * phi).powi(2) + 0.5 * epsilon * epsilon;
            }
            total_fe *= dx * dx * dx;
            free_energy_history.push(total_fe);
        }
    }

    let computation_time = start_time.elapsed().as_secs_f64();

    PfResult {
        success: true,
        field_data: field,
        grid_size,
        final_time: num_steps as f64 * dt,
        free_energy: free_energy_history,
        num_steps_completed: num_steps,
        computation_time_s: computation_time,
        output_path: format!("/tmp/pf_ac_output_{}.dat", timestamp_str()),
    }
}

/// Generate mock PFC (Phase Field Crystal) evolution
fn generate_mock_pfc_evolution(
    grid_size: [u32; 3],
    domain_size: [f64; 3],
    dt: f64,
    num_steps: u32,
    output_interval: u32,
    parameters: &HashMap<String, f64>,
    init_field: &[f64],
) -> PfResult {
    let nx = grid_size[0] as usize;
    let ny = grid_size[1] as usize;
    let nz = grid_size[2] as usize;

    let r = parameters.get("r").copied().unwrap_or(-0.3);
    let dx = domain_size[0] / nx as f64;

    let start_time = std::time::Instant::now();
    let mut field = init_field.to_vec();
    let mut free_energy_history = Vec::new();

    // PFC: dpsi/dt = laplacian[(r + (1+laplacian)^2) * psi + psi^3]
    for step in 0..num_steps {
        let mut new_field = field.clone();

        for ix in 2..nx - 2 {
            for iy in 2..ny - 2 {
                for iz in 2..nz - 2 {
                    let idx = ix * ny * nz + iy * nz + iz;
                    let psi = field[idx];

                    // Simplified biharmonic operator approximation
                    let lap_psi = (field[(ix + 1) * ny * nz + iy * nz + iz]
                        + field[(ix - 1) * ny * nz + iy * nz + iz]
                        + field[ix * ny * nz + (iy + 1) * nz + iz]
                        + field[ix * ny * nz + (iy - 1) * nz + iz]
                        + field[ix * ny * nz + iy * nz + iz + 1]
                        + field[ix * ny * nz + iy * nz + iz - 1]
                        - 6.0 * psi)
                        / (dx * dx);

                    let nonlinear = r * psi + psi * psi * psi;
                    let dpsi = lap_psi * nonlinear * dt * 0.01;
                    new_field[idx] = psi + dpsi;
                }
            }
        }

        field = new_field;

        if step % output_interval.max(1) == 0 {
            let mut total_fe = 0.0;
            for &psi in &field {
                total_fe += 0.5 * r * psi * psi + 0.25 * psi.powi(4);
            }
            total_fe *= dx * dx * dx;
            free_energy_history.push(total_fe);
        }
    }

    let computation_time = start_time.elapsed().as_secs_f64();

    PfResult {
        success: true,
        field_data: field,
        grid_size,
        final_time: num_steps as f64 * dt,
        free_energy: free_energy_history,
        num_steps_completed: num_steps,
        computation_time_s: computation_time,
        output_path: format!("/tmp/pf_pfc_output_{}.dat", timestamp_str()),
    }
}

/// Generate mock Karma-type solidification evolution
fn generate_mock_karma_evolution(
    grid_size: [u32; 3],
    domain_size: [f64; 3],
    dt: f64,
    num_steps: u32,
    output_interval: u32,
    parameters: &HashMap<String, f64>,
    init_field: &[f64],
) -> PfResult {
    let nx = grid_size[0] as usize;
    let ny = grid_size[1] as usize;
    let nz = grid_size[2] as usize;

    let undercooling = parameters.get("undercooling").copied().unwrap_or(0.5);
    let coupling = parameters.get("coupling").copied().unwrap_or(1.5);
    let dx = domain_size[0] / nx as f64;

    let start_time = std::time::Instant::now();
    let mut field = init_field.to_vec();
    let mut free_energy_history = Vec::new();

    // Karma model: coupled phi and temperature fields
    // dphi/dt = epsilon^2 * laplacian(phi) + phi(1-phi)(phi - 0.5 + m)
    // where m depends on undercooling
    for step in 0..num_steps {
        let mut new_field = field.clone();

        for ix in 1..nx - 1 {
            for iy in 1..ny - 1 {
                for iz in 1..nz - 1 {
                    let idx = ix * ny * nz + iy * nz + iz;
                    let phi = field[idx];

                    let lap_phi = (field[(ix + 1) * ny * nz + iy * nz + iz]
                        + field[(ix - 1) * ny * nz + iy * nz + iz]
                        + field[ix * ny * nz + (iy + 1) * nz + iz]
                        + field[ix * ny * nz + (iy - 1) * nz + iz]
                        + field[ix * ny * nz + iy * nz + iz + 1]
                        + field[ix * ny * nz + iy * nz + iz - 1]
                        - 6.0 * phi)
                        / (dx * dx);

                    // Karma interface kinetics
                    let m = coupling * undercooling * (1.0 - phi);
                    let reaction = phi * (1.0 - phi) * (phi - 0.5 + m);
                    let dphi = (0.01 * lap_phi + reaction) * dt;
                    new_field[idx] = (phi + dphi).clamp(0.0, 1.0);
                }
            }
        }

        field = new_field;

        if step % output_interval.max(1) == 0 {
            let mut total_fe = 0.0;
            for &phi in &field {
                total_fe += phi * phi * (1.0 - phi).powi(2);
            }
            total_fe *= dx * dx * dx;
            free_energy_history.push(total_fe);
        }
    }

    let computation_time = start_time.elapsed().as_secs_f64();

    PfResult {
        success: true,
        field_data: field,
        grid_size,
        final_time: num_steps as f64 * dt,
        free_energy: free_energy_history,
        num_steps_completed: num_steps,
        computation_time_s: computation_time,
        output_path: format!("/tmp/pf_karma_output_{}.dat", timestamp_str()),
    }
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Run a phase field simulation.
///
/// Supports Cahn-Hilliard (CH), Allen-Cahn (AC), Phase Field Crystal (PFC),
/// and Karma-type solidification models with various free energy functionals
/// and time integration schemes.
///
/// Current implementation uses simplified mock evolution for demonstration.
/// Production deployment should integrate with a high-performance PDE solver
/// (e.g., FFTW-based spectral methods or finite difference with MPI).
#[command]
pub fn run_phase_field_simulation(config: PfConfig) -> Result<PfResult, String> {
    tracing::info!(
        "Starting phase field simulation: eq={}, free_energy={}, integration={}, grid={}x{}x{}, dt={}, steps={}",
        config.equation_type,
        config.free_energy_type,
        config.time_integration,
        config.grid_size[0], config.grid_size[1], config.grid_size[2],
        config.dt,
        config.num_steps
    );

    // Validate grid dimensions
    if config.grid_size[0] == 0 || config.grid_size[1] == 0 || config.grid_size[2] == 0 {
        return Err("Grid dimensions must be non-zero".to_string());
    }

    let total = (config.grid_size[0] as usize)
        * (config.grid_size[1] as usize)
        * (config.grid_size[2] as usize);

    if total > 500_000_000 {
        return Err(format!(
            "Grid too large: {}x{}x{} = {} points. Maximum supported: 500M",
            config.grid_size[0], config.grid_size[1], config.grid_size[2], total
        ));
    }

    if config.dt <= 0.0 {
        return Err("Time step must be positive".to_string());
    }

    if config.num_steps == 0 {
        return Err("Number of steps must be greater than 0".to_string());
    }

    // Generate initial condition
    let init_config = PfInitConfig {
        grid_size: config.grid_size,
        domain_size: config.domain_size,
        condition_type: config.initial_condition.clone(),
        parameters: config.parameters.clone(),
    };
    let init_field = generate_pf_initial_condition_internal(&init_config)?;

    // Run appropriate simulation based on equation type
    let result = match config.equation_type.to_uppercase().as_str() {
        "CH" => generate_mock_ch_evolution(
            config.grid_size,
            config.domain_size,
            config.dt,
            config.num_steps,
            config.output_interval,
            &config.parameters,
            &init_field,
        ),
        "AC" => generate_mock_ac_evolution(
            config.grid_size,
            config.domain_size,
            config.dt,
            config.num_steps,
            config.output_interval,
            &config.parameters,
            &init_field,
        ),
        "PFC" => generate_mock_pfc_evolution(
            config.grid_size,
            config.domain_size,
            config.dt,
            config.num_steps,
            config.output_interval,
            &config.parameters,
            &init_field,
        ),
        "KARMA" => generate_mock_karma_evolution(
            config.grid_size,
            config.domain_size,
            config.dt,
            config.num_steps,
            config.output_interval,
            &config.parameters,
            &init_field,
        ),
        _ => {
            return Err(format!(
                "Unknown equation type: '{}'. Supported: CH, AC, PFC, Karma",
                config.equation_type
            ));
        }
    };

    tracing::info!(
        "Phase field simulation complete: eq={}, steps={}, final_time={:.4}, compute_time={:.3}s",
        config.equation_type,
        result.num_steps_completed,
        result.final_time,
        result.computation_time_s
    );

    Ok(result)
}

/// Generate initial conditions for a phase field simulation.
///
/// Supported types:
/// - "random": Uniform random noise in [mean - amplitude, mean + amplitude]
/// - "nucleation": Uniform background with circular/spherical nucleation seeds
/// - "interface": Planar interface between two phases
/// - "custom": User-specified via parameters
#[command]
pub fn generate_pf_initial_condition(config: PfInitConfig) -> Result<Vec<f64>, String> {
    tracing::info!(
        "Generating initial condition: type={}, grid={}x{}x{}",
        config.condition_type,
        config.grid_size[0], config.grid_size[1], config.grid_size[2]
    );

    generate_pf_initial_condition_internal(&config)
}

/// Internal implementation of initial condition generation
fn generate_pf_initial_condition_internal(config: &PfInitConfig) -> Result<Vec<f64>, String> {
    let nx = config.grid_size[0] as usize;
    let ny = config.grid_size[1] as usize;
    let nz = config.grid_size[2] as usize;
    let total = nx * ny * nz;

    if nx == 0 || ny == 0 || nz == 0 {
        return Err("Grid dimensions must be non-zero".to_string());
    }

    let mut field = Vec::with_capacity(total);
    let mut rng: u64 = 42;

    match config.condition_type.to_lowercase().as_str() {
        "random" => {
            // Random noise around a mean composition
            let mean = config.parameters.get("mean").copied().unwrap_or(0.5);
            let amplitude = config.parameters.get("amplitude").copied().unwrap_or(0.05);

            for _ in 0..total {
                let noise = xorshift64(&mut rng);
                let val = mean + amplitude * noise;
                field.push(val.clamp(0.0, 1.0));
            }

            tracing::info!(
                "Generated random initial condition: mean={:.3}, amplitude={:.3}, {} points",
                mean, amplitude, total
            );
        }
        "nucleation" => {
            // Uniform background with nucleation seeds
            let background = config.parameters.get("background").copied().unwrap_or(0.0);
            let seed_radius = config.parameters.get("seed_radius").copied().unwrap_or(5.0);
            let num_seeds = config.parameters.get("num_seeds").copied().unwrap_or(3.0) as usize;
            let seed_value = config.parameters.get("seed_value").copied().unwrap_or(1.0);

            let _dx = config.domain_size[0] / nx as f64;
            let _dy = config.domain_size[1] / ny as f64;
            let _dz = config.domain_size[2] / nz as f64;

            // Generate seed positions
            let mut seeds = Vec::with_capacity(num_seeds);
            for _ in 0..num_seeds {
                let sx = xorshift64(&mut rng) * 0.4 + 0.3;
                let sy = xorshift64(&mut rng) * 0.4 + 0.3;
                let sz = xorshift64(&mut rng) * 0.4 + 0.3;
                seeds.push((sx, sy, sz));
            }

            for ix in 0..nx {
                for iy in 0..ny {
                    for iz in 0..nz {
                        let x = (ix as f64 + 0.5) / nx as f64;
                        let y = (iy as f64 + 0.5) / ny as f64;
                        let z = (iz as f64 + 0.5) / nz as f64;

                        let mut val = background;
                        for &(sx, sy, sz) in &seeds {
                            let dist = ((x - sx).powi(2) + (y - sy).powi(2) + (z - sz).powi(2)).sqrt();
                            let r_grid = seed_radius / (config.domain_size[0] / nx as f64);
                            if dist < r_grid {
                                // Smooth interface: tanh profile
                                let interface_width = r_grid * 0.2;
                                let profile = 0.5 * (1.0 - ((dist - r_grid) / interface_width).tanh());
                                val = val.max(seed_value * profile);
                            }
                        }
                        field.push(val);
                    }
                }
            }

            tracing::info!(
                "Generated nucleation initial condition: {} seeds, radius={:.1}, {} points",
                num_seeds, seed_radius, total
            );
        }
        "interface" => {
            // Planar interface between two phases
            let phi_a = config.parameters.get("phi_a").copied().unwrap_or(0.0);
            let phi_b = config.parameters.get("phi_b").copied().unwrap_or(1.0);
            let interface_pos = config.parameters.get("interface_position").copied().unwrap_or(0.5);
            let interface_width = config.parameters.get("interface_width").copied().unwrap_or(0.05);

            for ix in 0..nx {
                for _iy in 0..ny {
                    for _iz in 0..nz {
                        let x_frac = (ix as f64 + 0.5) / nx as f64;
                        let profile = 0.5 * (1.0 + ((x_frac - interface_pos) / interface_width).tanh());
                        let val = phi_a + (phi_b - phi_a) * profile;
                        field.push(val);
                    }
                }
            }

            tracing::info!(
                "Generated interface initial condition: phi_a={:.2}, phi_b={:.2}, pos={:.2}, {} points",
                phi_a, phi_b, interface_pos, total
            );
        }
        "custom" => {
            // Custom: sinusoidal perturbation
            let frequency = config.parameters.get("frequency").copied().unwrap_or(4.0);
            let amplitude = config.parameters.get("amplitude").copied().unwrap_or(0.1);
            let mean = config.parameters.get("mean").copied().unwrap_or(0.5);

            for ix in 0..nx {
                for iy in 0..ny {
                    for iz in 0..nz {
                        let x = ix as f64 / nx as f64 * 2.0 * std::f64::consts::PI * frequency;
                        let y = iy as f64 / ny as f64 * 2.0 * std::f64::consts::PI * frequency;
                        let z = iz as f64 / nz as f64 * 2.0 * std::f64::consts::PI * frequency;
                        let val = mean
                            + amplitude * (x.cos() * y.cos() + 0.5 * z.cos())
                            + 0.01 * xorshift64(&mut rng);
                        field.push(val.clamp(0.0, 1.0));
                    }
                }
            }

            tracing::info!(
                "Generated custom initial condition: freq={:.1}, amp={:.2}, {} points",
                frequency, amplitude, total
            );
        }
        _ => {
            return Err(format!(
                "Unknown initial condition type: '{}'. Supported: random, nucleation, interface, custom",
                config.condition_type
            ));
        }
    }

    Ok(field)
}

/// Calculate free energy density at each grid point.
///
/// Computes the local free energy density based on the specified free energy
/// functional type and the field values. The gradient energy term (kappa * |grad(phi)|^2)
/// is included when kappa is provided in the parameters.
#[command]
pub fn calculate_pf_free_energy(
    field_data: Vec<f64>,
    grid_size: [u32; 3],
    domain_size: [f64; 3],
    free_energy_type: String,
    parameters: HashMap<String, f64>,
) -> Result<Vec<f64>, String> {
    tracing::info!(
        "Calculating free energy: type={}, grid={}x{}x{}, {} points",
        free_energy_type,
        grid_size[0], grid_size[1], grid_size[2],
        field_data.len()
    );

    let nx = grid_size[0] as usize;
    let ny = grid_size[1] as usize;
    let nz = grid_size[2] as usize;
    let total = nx * ny * nz;

    if field_data.len() != total {
        return Err(format!(
            "Field data length ({}) does not match grid size ({}x{}x{} = {})",
            field_data.len(), nx, ny, nz, total
        ));
    }

    let dx = domain_size[0] / nx as f64;
    let dy = domain_size[1] / ny as f64;
    let dz = domain_size[2] / nz as f64;

    let mut energy_density = Vec::with_capacity(total);

    match free_energy_type.to_lowercase().as_str() {
        "double_well" => {
            let a = parameters.get("A").copied().unwrap_or(1.0);
            let kappa = parameters.get("kappa").copied().unwrap_or(0.0);

            for ix in 0..nx {
                for iy in 0..ny {
                    for iz in 0..nz {
                        let idx = ix * ny * nz + iy * nz + iz;
                        let phi = field_data[idx];
                        let mut fe = double_well_energy(phi, a);

                        // Add gradient energy
                        if kappa > 0.0 && ix > 0 && ix < nx - 1 && iy > 0 && iy < ny - 1 && iz > 0 && iz < nz - 1 {
                            let dphidx = (field_data[(ix + 1) * ny * nz + iy * nz + iz]
                                - field_data[(ix - 1) * ny * nz + iy * nz + iz])
                                / (2.0 * dx);
                            let dphidy = (field_data[ix * ny * nz + (iy + 1) * nz + iz]
                                - field_data[ix * ny * nz + (iy - 1) * nz + iz])
                                / (2.0 * dy);
                            let dphidz = (field_data[ix * ny * nz + iy * nz + iz + 1]
                                - field_data[ix * ny * nz + iy * nz + iz - 1])
                                / (2.0 * dz);
                            let grad_sq = dphidx * dphidx + dphidy * dphidy + dphidz * dphidz;
                            fe += 0.5 * kappa * grad_sq;
                        }

                        energy_density.push(fe);
                    }
                }
            }
        }
        "landau" => {
            let a2 = parameters.get("a2").copied().unwrap_or(-1.0);
            let a4 = parameters.get("a4").copied().unwrap_or(1.0);
            let a6 = parameters.get("a6").copied().unwrap_or(0.0);

            for &phi in &field_data {
                energy_density.push(landau_energy(phi, a2, a4, a6));
            }
        }
        "flory_huggins" => {
            let n1 = parameters.get("N1").copied().unwrap_or(1.0);
            let n2 = parameters.get("N2").copied().unwrap_or(1.0);
            let chi = parameters.get("chi").copied().unwrap_or(2.0);

            for &phi in &field_data {
                energy_density.push(flory_huggins_energy(phi, n1, n2, chi));
            }
        }
        "polynomial" => {
            // Extract coefficients a0, a1, a2, ... from parameters
            let mut coeffs = Vec::new();
            let mut i = 0;
            while let Some(&a) = parameters.get(&format!("a{}", i)) {
                coeffs.push(a);
                i += 1;
            }
            if coeffs.is_empty() {
                // Default quartic: a0=0, a1=0, a2=-1, a3=0, a4=1
                coeffs = vec![0.0, 0.0, -1.0, 0.0, 1.0];
            }

            for &phi in &field_data {
                energy_density.push(polynomial_energy(phi, &coeffs));
            }
        }
        _ => {
            return Err(format!(
                "Unknown free energy type: '{}'. Supported: double_well, polynomial, landau, flory_huggins",
                free_energy_type
            ));
        }
    }

    tracing::info!(
        "Free energy calculated: {} points, type={}",
        energy_density.len(),
        free_energy_type
    );

    Ok(energy_density)
}

/// Get available phase field simulation templates.
///
/// Returns 4 pre-configured templates:
/// - CH_spinodal_decomposition: Cahn-Hilliard spinodal decomposition
/// - AC_grain_growth: Allen-Cahn polycrystalline grain growth
/// - PFC_dendritic: Phase Field Crystal dendritic solidification
/// - Karma_solidification: Karma-type dendritic growth with thermal coupling
#[command]
pub fn get_phase_field_templates() -> Result<Vec<PfTemplate>, String> {
    tracing::info!("Fetching phase field templates...");

    let mut ch_params = HashMap::new();
    ch_params.insert("mobility".to_string(), 1.0);
    ch_params.insert("kappa".to_string(), 0.5);
    ch_params.insert("A".to_string(), 1.0);

    let mut ac_params = HashMap::new();
    ac_params.insert("kappa".to_string(), 0.5);
    ac_params.insert("epsilon".to_string(), 0.01);

    let mut pfc_params = HashMap::new();
    pfc_params.insert("r".to_string(), -0.3);
    pfc_params.insert("psi0".to_string(), -0.28);

    let mut karma_params = HashMap::new();
    karma_params.insert("undercooling".to_string(), 0.5);
    karma_params.insert("coupling".to_string(), 1.5);
    karma_params.insert("latent_heat".to_string(), 1.6);
    karma_params.insert("thermal_diffusivity".to_string(), 1.0);

    let templates = vec![
        PfTemplate {
            id: "CH_spinodal_decomposition".to_string(),
            name: "Cahn-Hilliard Spinodal Decomposition".to_string(),
            description: "Simulate spinodal decomposition of a binary alloy using the Cahn-Hilliard equation with double-well free energy. The system spontaneously phase-separates into two equilibrium compositions, forming interconnected domain structures. Suitable for studying microstructure evolution in alloys, polymers, and glasses.".to_string(),
            equation_type: "CH".to_string(),
            free_energy_type: "double_well".to_string(),
            default_grid_size: [128, 128, 128],
            default_domain_size: [100.0, 100.0, 100.0],
            default_dt: 0.1,
            default_steps: 10000,
            default_parameters: ch_params,
            difficulty: "intermediate".to_string(),
        },
        PfTemplate {
            id: "AC_grain_growth".to_string(),
            name: "Allen-Cahn Grain Growth".to_string(),
            description: "Simulate normal grain growth in polycrystalline materials using the multi-order-parameter Allen-Cahn equation. Grain boundaries migrate toward their center of curvature, reducing the total grain boundary energy. The grain size distribution evolves toward a self-similar state.".to_string(),
            equation_type: "AC".to_string(),
            free_energy_type: "double_well".to_string(),
            default_grid_size: [256, 256, 1],
            default_domain_size: [200.0, 200.0, 1.0],
            default_dt: 0.05,
            default_steps: 50000,
            default_parameters: ac_params,
            difficulty: "intermediate".to_string(),
        },
        PfTemplate {
            id: "PFC_dendritic".to_string(),
            name: "Phase Field Crystal Dendritic Solidification".to_string(),
            description: "Simulate dendritic solidification using the Phase Field Crystal model, which naturally captures atomic-scale crystallographic orientation and anisotropic growth. The model operates on diffusive time scales while resolving lattice-level structure, making it ideal for studying nucleation and growth competition.".to_string(),
            equation_type: "PFC".to_string(),
            free_energy_type: "landau".to_string(),
            default_grid_size: [256, 256, 64],
            default_domain_size: [200.0, 200.0, 50.0],
            default_dt: 0.5,
            default_steps: 20000,
            default_parameters: pfc_params,
            difficulty: "advanced".to_string(),
        },
        PfTemplate {
            id: "Karma_solidification".to_string(),
            name: "Karma-type Dendritic Solidification".to_string(),
            description: "Simulate dendritic growth during solidification using the Karma model with thin-interface correction. The model couples the phase field to the temperature field, capturing latent heat release, tip selection, and sidebranching. Suitable for studying solidification microstructure in metallic alloys.".to_string(),
            equation_type: "Karma".to_string(),
            free_energy_type: "double_well".to_string(),
            default_grid_size: [300, 300, 64],
            default_domain_size: [300.0, 300.0, 64.0],
            default_dt: 0.01,
            default_steps: 50000,
            default_parameters: karma_params,
            difficulty: "advanced".to_string(),
        },
    ];

    tracing::info!("Returned {} phase field templates", templates.len());
    Ok(templates)
}

/// Validate a phase field simulation configuration.
///
/// Checks all configuration parameters for correctness and consistency,
/// returning a list of errors (must fix) and warnings (should consider).
#[command]
pub fn validate_pf_config(config: PfConfig) -> Result<PfValidationResult, String> {
    tracing::info!("Validating phase field configuration...");

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Validate equation type
    if !is_valid_equation_type(&config.equation_type) {
        errors.push(format!(
            "Invalid equation type: '{}'. Supported: CH, AC, PFC, Karma",
            config.equation_type
        ));
    }

    // Validate free energy type
    if !is_valid_free_energy_type(&config.free_energy_type) {
        errors.push(format!(
            "Invalid free energy type: '{}'. Supported: double_well, polynomial, landau, flory_huggins",
            config.free_energy_type
        ));
    }

    // Validate time integration
    if !is_valid_time_integration(&config.time_integration) {
        errors.push(format!(
            "Invalid time integration: '{}'. Supported: explicit, implicit, semi_implicit, rk4",
            config.time_integration
        ));
    }

    // Validate initial condition
    if !is_valid_initial_condition(&config.initial_condition) {
        errors.push(format!(
            "Invalid initial condition: '{}'. Supported: random, nucleation, interface, custom",
            config.initial_condition
        ));
    }

    // Validate grid dimensions
    for (i, &dim) in config.grid_size.iter().enumerate() {
        if dim == 0 {
            errors.push(format!("Grid dimension [{}] must be non-zero", i));
        }
    }

    let total = (config.grid_size[0] as u64)
        * (config.grid_size[1] as u64)
        * (config.grid_size[2] as u64);

    if total > 500_000_000 {
        errors.push(format!(
            "Grid too large: {} total points. Maximum supported: 500,000,000",
            total
        ));
    } else if total > 100_000_000 {
        warnings.push(format!(
            "Large grid ({} points). Consider reducing resolution or using GPU acceleration.",
            total
        ));
    }

    // Validate domain size
    for (i, &size) in config.domain_size.iter().enumerate() {
        if size <= 0.0 {
            errors.push(format!("Domain size [{}] must be positive, got {}", i, size));
        }
    }

    // Validate time step
    if config.dt <= 0.0 {
        errors.push(format!("Time step must be positive, got {}", config.dt));
    } else {
        // CFL-like stability check for explicit schemes
        if config.time_integration == "explicit" {
            let dx_min = config.domain_size.iter()
                .zip(config.grid_size.iter())
                .map(|(l, n)| l / *n as f64)
                .fold(f64::MAX, f64::min);

            let kappa = config.parameters.get("kappa").copied().unwrap_or(0.5);
            let dt_max = 0.5 * dx_min * dx_min / (4.0 * kappa);

            if config.dt > dt_max {
                warnings.push(format!(
                    "Explicit time step ({:.6}) may exceed stability limit ({:.6}). Consider reducing dt or using semi_implicit integration.",
                    config.dt, dt_max
                ));
            }
        }
    }

    // Validate number of steps
    if config.num_steps == 0 {
        errors.push("Number of steps must be greater than 0".to_string());
    } else if config.num_steps < 100 {
        warnings.push(format!(
            "Very few steps ({}). Simulation may not reach meaningful evolution.",
            config.num_steps
        ));
    }

    // Validate output interval
    if config.output_interval == 0 {
        errors.push("Output interval must be greater than 0".to_string());
    } else if config.output_interval > config.num_steps {
        warnings.push(
            "Output interval exceeds total steps. Only the initial state will be recorded.".to_string(),
        );
    }

    // Equation-specific checks
    match config.equation_type.to_uppercase().as_str() {
        "CH" => {
            if !config.parameters.contains_key("mobility") {
                warnings.push("CH equation: 'mobility' parameter not set, using default (1.0)".to_string());
            }
            if !config.parameters.contains_key("kappa") {
                warnings.push("CH equation: 'kappa' (gradient energy coefficient) not set, using default (0.5)".to_string());
            }
        }
        "AC" => {
            if !config.parameters.contains_key("epsilon") {
                warnings.push("AC equation: 'epsilon' (interface width) not set, using default (0.01)".to_string());
            }
        }
        "PFC" => {
            if !config.parameters.contains_key("r") {
                warnings.push("PFC equation: 'r' (undercooling parameter) not set, using default (-0.3)".to_string());
            }
        }
        "KARMA" => {
            if !config.parameters.contains_key("undercooling") {
                warnings.push("Karma model: 'undercooling' not set, using default (0.5)".to_string());
            }
        }
        _ => {}
    }

    // Free energy compatibility check
    if config.free_energy_type == "flory_huggins" && config.equation_type != "CH" {
        warnings.push(
            "Flory-Huggins free energy is typically used with Cahn-Hilliard (CH) equation.".to_string(),
        );
    }

    let valid = errors.is_empty();

    tracing::info!(
        "Validation complete: valid={}, errors={}, warnings={}",
        valid,
        errors.len(),
        warnings.len()
    );

    Ok(PfValidationResult {
        valid,
        errors,
        warnings,
    })
}

/// Estimate memory requirements for a phase field simulation.
///
/// Accounts for:
/// - Field data arrays (current + previous for implicit schemes)
/// - Gradient and Laplacian buffers
/// - Free energy tracking
/// - Output storage
/// - Solver workspace (for implicit methods)
#[command]
pub fn estimate_pf_memory(
    grid_size: [u32; 3],
    num_fields: u32,
    num_steps: u32,
) -> Result<PfMemoryEstimate, String> {
    tracing::info!(
        "Estimating PF memory: grid={}x{}x{}, fields={}, steps={}",
        grid_size[0], grid_size[1], grid_size[2],
        num_fields, num_steps
    );

    let total = (grid_size[0] as u64)
        * (grid_size[1] as u64)
        * (grid_size[2] as u64);

    if total == 0 {
        return Err("Grid dimensions must be non-zero".to_string());
    }

    // Field data: 8 bytes per double
    // Need current + previous for time integration, plus work arrays
    let field_bytes_per = total * 8; // one field
    let field_mb = field_bytes_per as f64 / (1024.0 * 1024.0);

    // Per-field memory: field + gradient (3 components) + laplacian + chemical potential
    let per_field_mb = field_mb * 6.0; // 6 arrays per field

    // Total field memory
    let total_field_mb = per_field_mb * num_fields as f64;

    // Solver workspace (for implicit: matrix, RHS, etc.)
    let solver_mb = field_mb * 4.0; // Approximate

    // Output storage: assume output_interval saves one snapshot
    let output_interval = (num_steps / 100).max(1);
    let num_snapshots = (num_steps / output_interval) as u64 + 1;
    let output_mb = field_mb * num_fields as f64 * num_snapshots as f64;

    // Total RAM estimate (field + solver, output written to disk)
    let estimated_mb = total_field_mb + solver_mb;

    // Disk estimate
    let disk_estimate_mb = output_mb;

    // Recommended RAM with 50% safety margin
    let recommended_mb = estimated_mb * 1.5;
    let recommended_ram = if recommended_mb < 512.0 {
        format!("{:.0} MB", recommended_mb.max(128.0))
    } else if recommended_mb < 4096.0 {
        format!("{:.1} GB", recommended_mb / 1024.0)
    } else {
        format!("{:.0} GB", recommended_mb / 1024.0)
    };

    tracing::info!(
        "Memory estimate: RAM={:.1} MB (recommended: {}), disk={:.1} MB",
        estimated_mb,
        recommended_ram,
        disk_estimate_mb
    );

    Ok(PfMemoryEstimate {
        estimated_mb,
        recommended_ram,
        disk_estimate_mb,
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_phase_field_simulation_ch() {
        let config = PfConfig {
            equation_type: "CH".to_string(),
            free_energy_type: "double_well".to_string(),
            time_integration: "explicit".to_string(),
            grid_size: [16, 16, 16],
            domain_size: [10.0, 10.0, 10.0],
            dt: 0.1,
            num_steps: 100,
            initial_condition: "random".to_string(),
            parameters: HashMap::new(),
            output_interval: 50,
        };
        let result = run_phase_field_simulation(config).unwrap();
        assert!(result.success);
        assert_eq!(result.field_data.len(), 16 * 16 * 16);
        assert_eq!(result.num_steps_completed, 100);
        assert!(result.computation_time_s >= 0.0);
    }

    #[test]
    fn test_run_phase_field_simulation_ac() {
        let config = PfConfig {
            equation_type: "AC".to_string(),
            free_energy_type: "double_well".to_string(),
            time_integration: "explicit".to_string(),
            grid_size: [16, 16, 1],
            domain_size: [10.0, 10.0, 1.0],
            dt: 0.05,
            num_steps: 50,
            initial_condition: "interface".to_string(),
            parameters: HashMap::new(),
            output_interval: 25,
        };
        let result = run_phase_field_simulation(config).unwrap();
        assert!(result.success);
        assert_eq!(result.field_data.len(), 16 * 16);
    }

    #[test]
    fn test_generate_initial_condition_random() {
        let config = PfInitConfig {
            grid_size: [32, 32, 32],
            domain_size: [10.0, 10.0, 10.0],
            condition_type: "random".to_string(),
            parameters: HashMap::new(),
        };
        let field = generate_pf_initial_condition(config).unwrap();
        assert_eq!(field.len(), 32 * 32 * 32);
        // All values should be in [0, 1]
        for &val in &field {
            assert!(val >= 0.0 && val <= 1.0);
        }
    }

    #[test]
    fn test_generate_initial_condition_nucleation() {
        let config = PfInitConfig {
            grid_size: [32, 32, 32],
            domain_size: [10.0, 10.0, 10.0],
            condition_type: "nucleation".to_string(),
            parameters: HashMap::new(),
        };
        let field = generate_pf_initial_condition(config).unwrap();
        assert_eq!(field.len(), 32 * 32 * 32);
    }

    #[test]
    fn test_generate_initial_condition_interface() {
        let config = PfInitConfig {
            grid_size: [32, 16, 16],
            domain_size: [10.0, 5.0, 5.0],
            condition_type: "interface".to_string(),
            parameters: HashMap::new(),
        };
        let field = generate_pf_initial_condition(config).unwrap();
        assert_eq!(field.len(), 32 * 16 * 16);
        // Values should transition from ~0 to ~1
        let first_val = field[0];
        let last_val = field[field.len() - 1];
        assert!(last_val > first_val);
    }

    #[test]
    fn test_calculate_pf_free_energy_double_well() {
        let field = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let energy = calculate_pf_free_energy(
            field,
            [5, 1, 1],
            [5.0, 1.0, 1.0],
            "double_well".to_string(),
            HashMap::new(),
        )
        .unwrap();
        assert_eq!(energy.len(), 5);
        // Double-well minima at phi=0 and phi=1
        assert!((energy[0] - 0.0).abs() < 1e-10);
        assert!((energy[4] - 0.0).abs() < 1e-10);
        // Maximum at phi=0.5
        assert!(energy[2] > energy[0]);
    }

    #[test]
    fn test_calculate_pf_free_energy_landau() {
        let field = vec![0.0, 0.5, 1.0, -0.5, -1.0];
        let energy = calculate_pf_free_energy(
            field,
            [5, 1, 1],
            [5.0, 1.0, 1.0],
            "landau".to_string(),
            HashMap::new(),
        )
        .unwrap();
        assert_eq!(energy.len(), 5);
    }

    #[test]
    fn test_get_phase_field_templates() {
        let templates = get_phase_field_templates().unwrap();
        assert_eq!(templates.len(), 4);
        assert_eq!(templates[0].id, "CH_spinodal_decomposition");
        assert_eq!(templates[1].id, "AC_grain_growth");
        assert_eq!(templates[2].id, "PFC_dendritic");
        assert_eq!(templates[3].id, "Karma_solidification");
        for t in &templates {
            assert!(!t.name.is_empty());
            assert!(!t.description.is_empty());
            assert!(!t.default_parameters.is_empty());
        }
    }

    #[test]
    fn test_validate_pf_config_valid() {
        let config = PfConfig {
            equation_type: "CH".to_string(),
            free_energy_type: "double_well".to_string(),
            time_integration: "semi_implicit".to_string(),
            grid_size: [64, 64, 64],
            domain_size: [50.0, 50.0, 50.0],
            dt: 0.1,
            num_steps: 1000,
            initial_condition: "random".to_string(),
            parameters: HashMap::new(),
        };
        let result = validate_pf_config(config).unwrap();
        assert!(result.valid);
    }

    #[test]
    fn test_validate_pf_config_invalid() {
        let config = PfConfig {
            equation_type: "INVALID".to_string(),
            free_energy_type: "bad_type".to_string(),
            time_integration: "unknown".to_string(),
            grid_size: [0, 64, 64],
            domain_size: [50.0, 50.0, 50.0],
            dt: -1.0,
            num_steps: 0,
            initial_condition: "bad".to_string(),
            parameters: HashMap::new(),
        };
        let result = validate_pf_config(config).unwrap();
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_estimate_pf_memory() {
        let estimate = estimate_pf_memory([128, 128, 128], 1, 10000).unwrap();
        assert!(estimate.estimated_mb > 0.0);
        assert!(!estimate.recommended_ram.is_empty());
        assert!(estimate.disk_estimate_mb > 0.0);
    }

    #[test]
    fn test_estimate_pf_memory_zero_grid() {
        let result = estimate_pf_memory([0, 128, 128], 1, 10000);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_simulation_invalid_equation() {
        let config = PfConfig {
            equation_type: "INVALID".to_string(),
            free_energy_type: "double_well".to_string(),
            time_integration: "explicit".to_string(),
            grid_size: [16, 16, 16],
            domain_size: [10.0, 10.0, 10.0],
            dt: 0.1,
            num_steps: 100,
            initial_condition: "random".to_string(),
            parameters: HashMap::new(),
            output_interval: 50,
        };
        let result = run_phase_field_simulation(config);
        assert!(result.is_err());
    }
}
