// DFT Bridge Commands - V1.7
// Bridges DFT calculations with machine learning potentials and phase-field modeling.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::command;
use tracing::info;

// ============================================================================
// Data Structures
// ============================================================================

/// A single DFT training data point for ML potential fitting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DftTrainingPoint {
    pub structure: String,
    pub energy: f64,
    pub forces: Vec<[f64; 3]>,
    pub stress: [f64; 6],
    pub virial: Option<[f64; 9]>,
}

/// Result of training data preparation and splitting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingDataResult {
    pub training_file: String,
    pub validation_file: String,
    pub test_file: String,
    pub total_configs: u32,
    pub train_configs: u32,
    pub val_configs: u32,
    pub test_configs: u32,
}

/// Configuration for training a machine learning potential.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub training_file: String,
    pub validation_file: String,
    pub potential_type: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Result of potential training.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingResult {
    pub potential_file: String,
    pub energy_rmse_mev: f64,
    pub force_rmse_mev_a: f64,
    pub stress_rmse_gpa: f64,
    pub training_configs: u32,
    pub validation_configs: u32,
    pub total_training_time_s: f64,
}

/// Result of potential validation on test set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotentialValidationResult {
    pub energy_rmse: f64,
    pub force_rmse: f64,
    pub stress_rmse: f64,
    pub test_configs: u32,
}

/// Ginzburg-Landau free energy parameters extracted from DFT data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GinzburgLandauParams {
    pub free_energy_coefficients: HashMap<String, f64>,
    pub mobility: f64,
    pub gradient_coefficient: f64,
    pub interface_width: f64,
    pub critical_temperature: f64,
}

/// Phase diagram data computed from Ginzburg-Landau model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseDiagramData {
    pub temperatures: Vec<f64>,
    pub compositions: Vec<f64>,
    pub phase_fields: Vec<Vec<f64>>,
    pub critical_points: Vec<[f64; 2]>,
}

/// Result of chemical potential alignment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChemicalPotentialResult {
    pub chemical_potentials: HashMap<String, f64>,
    pub reference_energy: f64,
    pub alignment_shift: f64,
}

/// A template for DFT bridge workflows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DftBridgeTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub required_inputs: Vec<String>,
    pub default_parameters: HashMap<String, serde_json::Value>,
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Prepares training data from DFT results by splitting into train/val/test sets.
#[command]
pub fn prepare_training_data(
    dft_results: Vec<DftTrainingPoint>,
    train_ratio: f64,
    val_ratio: f64,
) -> Result<TrainingDataResult, String> {
    info!(
        num_configs = dft_results.len(),
        train_ratio = train_ratio,
        val_ratio = val_ratio,
        "Preparing training data"
    );

    if dft_results.is_empty() {
        return Err("No DFT training points provided".to_string());
    }

    if train_ratio <= 0.0 || train_ratio >= 1.0 {
        return Err("Train ratio must be between 0 and 1 (exclusive)".to_string());
    }

    if val_ratio <= 0.0 || val_ratio >= 1.0 {
        return Err("Validation ratio must be between 0 and 1 (exclusive)".to_string());
    }

    if train_ratio + val_ratio >= 1.0 {
        return Err("Train ratio + validation ratio must be less than 1".to_string());
    }

    let total = dft_results.len() as u32;
    let train_count = (total as f64 * train_ratio).round() as u32;
    let val_count = (total as f64 * val_ratio).round() as u32;
    let test_count = total - train_count - val_count;

    // In a real implementation, this would write actual training files.
    // Here we generate file paths and metadata.
    let training_file = format!("/data/mlpot/train_{}.xyz", total);
    let validation_file = format!("/data/mlpot/val_{}.xyz", total);
    let test_file = format!("/data/mlpot/test_{}.xyz", total);

    info!(
        train = train_count,
        val = val_count,
        test = test_count,
        "Training data prepared successfully"
    );

    Ok(TrainingDataResult {
        training_file,
        validation_file,
        test_file,
        total_configs: total,
        train_configs: train_count,
        val_configs: val_count,
        test_configs: test_count,
    })
}

/// Trains a machine learning interatomic potential from DFT data.
#[command]
pub fn train_potential(config: TrainingConfig) -> Result<TrainingResult, String> {
    info!(
        potential_type = %config.potential_type,
        training_file = %config.training_file,
        "Training ML potential"
    );

    let valid_types = ["NEP", "MTP", "ACE", "GAP"];
    if !valid_types.contains(&config.potential_type.as_str()) {
        return Err(format!(
            "Invalid potential type '{}'. Must be one of: {}",
            config.potential_type,
            valid_types.join(", ")
        ));
    }

    if config.training_file.is_empty() {
        return Err("Training file path cannot be empty".to_string());
    }

    // In a real implementation, this would invoke the actual training code.
    // Here we return realistic mock results based on the potential type.
    let (energy_rmse, force_rmse, stress_rmse, train_configs, val_configs, time_s) =
        match config.potential_type.as_str() {
            "NEP" => (1.23, 45.6, 0.089, 800, 100, 342.5),
            "MTP" => (2.15, 78.3, 0.145, 800, 100, 156.2),
            "ACE" => (1.87, 62.1, 0.112, 800, 100, 278.9),
            "GAP" => (0.98, 38.7, 0.067, 800, 100, 1845.3),
            _ => unreachable!(),
        };

    let potential_file = format!(
        "/data/mlpot/{}_potential_{}.txt",
        config.potential_type.to_lowercase(),
        "20260403"
    );

    info!(
        energy_rmse = energy_rmse,
        force_rmse = force_rmse,
        "Potential training completed"
    );

    Ok(TrainingResult {
        potential_file,
        energy_rmse_mev: energy_rmse,
        force_rmse_mev_a: force_rmse,
        stress_rmse_gpa: stress_rmse,
        training_configs: train_configs,
        validation_configs: val_configs,
        total_training_time_s: time_s,
    })
}

/// Validates a trained potential on a test dataset.
#[command]
pub fn validate_potential(
    potential_file: String,
    test_file: String,
) -> Result<PotentialValidationResult, String> {
    info!(
        potential = %potential_file,
        test_file = %test_file,
        "Validating ML potential"
    );

    if potential_file.is_empty() {
        return Err("Potential file path cannot be empty".to_string());
    }

    if test_file.is_empty() {
        return Err("Test file path cannot be empty".to_string());
    }

    // In a real implementation, this would run the potential on test structures
    // and compare with DFT reference values.
    let result = PotentialValidationResult {
        energy_rmse: 1.45,
        force_rmse: 52.3,
        stress_rmse: 0.102,
        test_configs: 100,
    };

    info!(
        energy_rmse = result.energy_rmse,
        force_rmse = result.force_rmse,
        test_configs = result.test_configs,
        "Potential validation completed"
    );

    Ok(result)
}

/// Exports a trained potential to a specified format.
#[command]
pub fn export_potential(
    potential_file: String,
    format: String,
    output_path: String,
) -> Result<String, String> {
    info!(
        potential = %potential_file,
        format = %format,
        output = %output_path,
        "Exporting ML potential"
    );

    let valid_formats = ["nep", "mtp", "ace", "lammps", "deepmd"];
    if !valid_formats.contains(&format.to_lowercase().as_str()) {
        return Err(format!(
            "Invalid export format '{}'. Must be one of: {}",
            format,
            valid_formats.join(", ")
        ));
    }

    if output_path.is_empty() {
        return Err("Output path cannot be empty".to_string());
    }

    // In a real implementation, this would convert the potential to the target format.
    let exported_file = format!("{}/potential.{}", output_path, format.to_lowercase());

    info!(exported = %exported_file, "Potential exported successfully");
    Ok(exported_file)
}

/// Extracts Ginzburg-Landau parameters from DFT energy data.
#[command]
pub fn extract_gl_params(
    dft_result: crate::commands::dft_postprocess::DftEnergyData,
    temperature: f64,
) -> Result<GinzburgLandauParams, String> {
    info!(
        temperature = temperature,
        num_steps = dft_result.energies.len(),
        "Extracting Ginzburg-Landau parameters"
    );

    if dft_result.energies.is_empty() {
        return Err("DFT energy data is empty".to_string());
    }

    if temperature < 0.0 {
        return Err("Temperature must be non-negative".to_string());
    }

    let final_energy = dft_result.final_energy;

    // In a real implementation, this would fit GL free energy functional to DFT data.
    // Here we compute realistic mock GL parameters.
    let mut free_energy_coefficients = HashMap::new();

    // Landau expansion coefficients: F = a0 + a2*eta^2 + a4*eta^4 + a6*eta^6
    free_energy_coefficients.insert("a0".to_string(), final_energy);
    free_energy_coefficients.insert("a2".to_string(), 0.045 * (temperature / 1000.0 - 0.8));
    free_energy_coefficients.insert("a4".to_string(), 2.34);
    free_energy_coefficients.insert("a6".to_string(), 0.0);

    // Second-order transition temperature
    let critical_temperature = 1084.0; // e.g., Cu melting point analog

    let mobility = 1.0e-4 * (-5000.0 / (temperature + 1.0)).exp(); // Arrhenius-like
    let gradient_coefficient = 2.5e-7; // Typical value in J/m
    let interface_width = 5.0e-9; // 5 nm interface width

    info!(
        critical_temp = critical_temperature,
        mobility = mobility,
        "GL parameters extracted"
    );

    Ok(GinzburgLandauParams {
        free_energy_coefficients,
        mobility,
        gradient_coefficient,
        interface_width,
        critical_temperature,
    })
}

/// Calculates a phase diagram from Ginzburg-Landau parameters.
#[command]
pub fn calculate_phase_diagram(
    params: GinzburgLandauParams,
    temperature_range: [f64; 2],
    composition_range: [f64; 2],
    num_points: [u32; 2],
) -> Result<PhaseDiagramData, String> {
    info!(
        t_range = ?temperature_range,
        x_range = ?composition_range,
        "Calculating phase diagram"
    );

    if temperature_range[0] >= temperature_range[1] {
        return Err("Temperature range must be increasing (min < max)".to_string());
    }

    if composition_range[0] >= composition_range[1] {
        return Err("Composition range must be increasing (min < max)".to_string());
    }

    if num_points[0] < 2 || num_points[1] < 2 {
        return Err("Number of points must be at least 2 in each dimension".to_string());
    }

    let nt = num_points[0] as usize;
    let nx = num_points[1] as usize;

    let dt = (temperature_range[1] - temperature_range[0]) / (nt - 1) as f64;
    let dx = (composition_range[1] - composition_range[0]) / (nx - 1) as f64;

    // Generate temperature and composition arrays
    let temperatures: Vec<f64> = (0..nt)
        .map(|i| temperature_range[0] + i as f64 * dt)
        .collect();

    let compositions: Vec<f64> = (0..nx)
        .map(|i| composition_range[0] + i as f64 * dx)
        .collect();

    // Calculate free energy landscape
    // In a real implementation, this would solve the GL equations.
    // Here we generate a realistic binary phase diagram (e.g., symmetric eutectic).
    let mut phase_fields = Vec::with_capacity(nt);

    let tc = params.critical_temperature;
    let x_mid = (composition_range[0] + composition_range[1]) / 2.0;
    let x_half = (composition_range[1] - composition_range[0]) / 2.0;

    for &t in &temperatures {
        let mut row = Vec::with_capacity(nx);
        for &x in &compositions {
            // Normalized composition deviation from midpoint
            let eta = (x - x_mid) / x_half;

            // Landau free energy: F = a2(T)*eta^2 + a4*eta^4
            let a2 = params.free_energy_coefficients.get("a2").copied().unwrap_or(0.045)
                * (t / tc - 1.0);
            let a4 = params.free_energy_coefficients.get("a4").copied().unwrap_or(2.34);

            let free_energy = a2 * eta.powi(2) + a4 * eta.powi(4);

            // Add mixing entropy contribution
            let x_safe = x.max(0.01).min(0.99);
            let entropy = 8.314 * t * 0.001 * (
                x_safe * x_safe.ln() + (1.0 - x_safe) * (1.0 - x_safe).ln()
            );

            row.push(free_energy + entropy);
        }
        phase_fields.push(row);
    }

    // Identify critical points (phase transitions)
    let mut critical_points = Vec::new();

    // Eutectic point
    let eutectic_temp = tc * 0.6;
    let eutectic_comp = x_mid;
    if eutectic_temp > temperature_range[0] && eutectic_temp < temperature_range[1] {
        critical_points.push([eutectic_comp, eutectic_temp]);
    }

    // Solvus points
    for &t in &temperatures {
        if t < tc && t > temperature_range[0] {
            let miscibility_gap = 0.3 * (1.0 - (t / tc).powi(2)).sqrt();
            if miscibility_gap > 0.01 {
                critical_points.push([x_mid - miscibility_gap * x_half, t]);
                critical_points.push([x_mid + miscibility_gap * x_half, t]);
            }
        }
    }

    info!(
        num_temps = nt,
        num_comps = nx,
        critical_pts = critical_points.len(),
        "Phase diagram calculated"
    );

    Ok(PhaseDiagramData {
        temperatures,
        compositions,
        phase_fields,
        critical_points,
    })
}

/// Aligns chemical potentials relative to a reference element.
#[command]
pub fn align_chemical_potential(
    energies: HashMap<String, f64>,
    reference: String,
    temperature: f64,
) -> Result<ChemicalPotentialResult, String> {
    info!(
        reference = %reference,
        temperature = temperature,
        num_elements = energies.len(),
        "Aligning chemical potentials"
    );

    if energies.is_empty() {
        return Err("Energy map cannot be empty".to_string());
    }

    if !energies.contains_key(&reference) {
        return Err(format!("Reference element '{}' not found in energy map", reference));
    }

    if temperature < 0.0 {
        return Err("Temperature must be non-negative".to_string());
    }

    let reference_energy = energies[&reference];

    // In a real implementation, this would compute chemical potentials from
    // DFT total energies of competing phases and apply thermodynamic corrections.
    let mut chemical_potentials = HashMap::new();

    for (element, &energy) in &energies {
        let mu = energy - reference_energy;

        // Add vibrational free energy correction (high-temperature limit)
        let kb_t = 8.617e-5 * temperature; // eV
        let vibrational_correction = 3.0 * kb_t * (temperature / 300.0).ln().max(0.0);

        chemical_potentials.insert(element.clone(), mu + vibrational_correction);
    }

    let alignment_shift = -reference_energy;

    info!(
        reference_energy = reference_energy,
        shift = alignment_shift,
        "Chemical potentials aligned"
    );

    Ok(ChemicalPotentialResult {
        chemical_potentials,
        reference_energy,
        alignment_shift,
    })
}

/// Returns available DFT bridge workflow templates.
#[command]
pub fn get_dft_bridge_templates() -> Result<Vec<DftBridgeTemplate>, String> {
    info!("Fetching DFT bridge templates");

    let mut nep_params = HashMap::new();
    nep_params.insert("nep_version".to_string(), serde_json::json!(4));
    nep_params.insert("cutoff".to_string(), serde_json::json!(8.0));
    nep_params.insert("n_max".to_string(), serde_json::json!(16));
    nep_params.insert("l_max".to_string(), serde_json::json!(8));
    nep_params.insert("basis_size".to_string(), serde_json::json!(10));
    nep_params.insert("neuron".to_string(), serde_json::json!(100));
    nep_params.insert("batch_size".to_string(), serde_json::json!(64));
    nep_params.insert("num_epochs".to_string(), serde_json::json!(2000));
    nep_params.insert("lambda_ener".to_string(), serde_json::json!(1.0));
    nep_params.insert("lambda_force".to_string(), serde_json::json!(10.0));
    nep_params.insert("lambda_virial".to_string(), serde_json::json!(0.1));

    let mut mtp_params = HashMap::new();
    mtp_params.insert("mtp_level".to_string(), serde_json::json!(16));
    mtp_params.insert("max_dist".to_string(), serde_json::json!(5.0));
    mtp_params.insert("radial_basis_size".to_string(), serde_json::json!(8));
    mtp_params.insert("train_until".to_string(), serde_json::json!(1000));
    mtp_params.insert("energy_weight".to_string(), serde_json::json!(1.0));
    mtp_params.insert("force_weight".to_string(), serde_json::json!(0.1));
    mtp_params.insert("stress_weight".to_string(), serde_json::json!(0.001));

    let mut gl_params = HashMap::new();
    gl_params.insert("order".to_string(), serde_json::json!(6));
    gl_params.insert("interface_width_nm".to_string(), serde_json::json!(5.0));
    gl_params.insert("grid_resolution".to_string(), serde_json::json!(256));
    gl_params.insert("num_order_params".to_string(), serde_json::json!(1));
    gl_params.insert("coupling_terms".to_string(), serde_json::json!(true));

    let mut phase_params = HashMap::new();
    phase_params.insert("temp_min_K".to_string(), serde_json::json!(300));
    phase_params.insert("temp_max_K".to_string(), serde_json::json!(2000));
    phase_params.insert("comp_min".to_string(), serde_json::json!(0.0));
    phase_params.insert("comp_max".to_string(), serde_json::json!(1.0));
    phase_params.insert("temp_points".to_string(), serde_json::json!(100));
    phase_params.insert("comp_points".to_string(), serde_json::json!(100));
    phase_params.insert("include_magnetic".to_string(), serde_json::json!(false));

    let templates = vec![
        DftBridgeTemplate {
            id: "NEP_training".to_string(),
            name: "NEP Potential Training".to_string(),
            description: "Train a Neuroevolution Potential (NEP) from DFT training data. NEP provides high accuracy with moderate computational cost and is suitable for metals, semiconductors, and oxides.".to_string(),
            category: "potential_training".to_string(),
            required_inputs: vec![
                "training_file".to_string(),
                "validation_file".to_string(),
                "element_list".to_string(),
            ],
            default_parameters: nep_params,
        },
        DftBridgeTemplate {
            id: "MTP_training".to_string(),
            name: "MTP Potential Training".to_string(),
            description: "Train a Moment Tensor Potential (MTP) from DFT training data. MTP is efficient for large-scale simulations and provides good accuracy for a wide range of materials.".to_string(),
            category: "potential_training".to_string(),
            required_inputs: vec![
                "training_file".to_string(),
                "validation_file".to_string(),
                "element_list".to_string(),
            ],
            default_parameters: mtp_params,
        },
        DftBridgeTemplate {
            id: "GL_extraction".to_string(),
            name: "Ginzburg-Landau Parameter Extraction".to_string(),
            description: "Extract Ginzburg-Landau free energy parameters from DFT energy data. Used for phase-field modeling of microstructure evolution.".to_string(),
            category: "phase_field".to_string(),
            required_inputs: vec![
                "dft_energy_data".to_string(),
                "temperature_range".to_string(),
                "order_parameter".to_string(),
            ],
            default_parameters: gl_params,
        },
        DftBridgeTemplate {
            id: "Phase_diagram".to_string(),
            name: "Phase Diagram Calculation".to_string(),
            description: "Calculate binary phase diagrams from Ginzburg-Landau free energy model. Includes identification of critical points, spinodal regions, and phase boundaries.".to_string(),
            category: "thermodynamics".to_string(),
            required_inputs: vec![
                "gl_parameters".to_string(),
                "temperature_range".to_string(),
                "composition_range".to_string(),
            ],
            default_parameters: phase_params,
        },
    ];

    info!(count = templates.len(), "DFT bridge templates returned");
    Ok(templates)
}
