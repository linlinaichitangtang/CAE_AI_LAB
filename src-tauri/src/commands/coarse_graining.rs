// Coarse Graining Commands - V1.8
// Multiscale coarse-graining methods with method recommendation and comparison.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tauri::command;

// ============================================================================
// Data Structures
// ============================================================================

/// Configuration for a coarse-graining operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoarseGrainingConfig {
    pub method: String,           // "QC", "MQC", "radial_average", "fourier_filter", "ml_mapping", "voronoi"
    pub source_scale: String,     // "atomistic", "mesoscale"
    pub target_scale: String,     // "mesoscale", "continuum"
    pub source_data_size: u64,    // Number of source data points
    pub target_resolution: [u32; 3], // Target grid resolution
    pub domain_size: [f64; 3],    // Physical domain size
    pub parameters: HashMap<String, Value>,
}

/// Result of a coarse-graining operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoarseGrainingResult {
    pub id: String,
    pub method: String,
    pub source_points: u64,
    pub target_grid_size: [u32; 3],
    pub field_data: Vec<f64>,
    pub quality_metrics: HashMap<String, f64>,
    pub computation_time_ms: f64,
    pub created_at: String,
}

/// A coarse-graining method description.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoarseGrainingMethod {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source_scales: Vec<String>,
    pub target_scales: Vec<String>,
    pub complexity: String,       // "low", "medium", "high"
    pub accuracy: String,         // "low", "medium", "high"
    pub typical_speedup: f64,
    pub parameters: Vec<MethodParameter>,
    pub use_cases: Vec<String>,
}

/// A parameter definition for a coarse-graining method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodParameter {
    pub name: String,
    pub type_: String,
    pub default_value: Value,
    pub description: String,
    pub range: Option<(f64, f64)>,
}

/// Comparison result between two or more coarse-graining methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodComparison {
    pub methods: Vec<MethodComparisonEntry>,
    pub recommendation: String,
    pub recommendation_reason: String,
}

/// Entry for a single method in a comparison.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodComparisonEntry {
    pub method_id: String,
    pub accuracy_score: f64,
    pub speed_score: f64,
    pub memory_score: f64,
    pub robustness_score: f64,
    pub overall_score: f64,
    pub computation_time_ms: f64,
    pub quality_metrics: HashMap<String, f64>,
}

/// A preset configuration for common coarse-graining scenarios.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoarseGrainingPreset {
    pub id: String,
    pub name: String,
    pub description: String,
    pub scenario: String,
    pub method: String,
    pub parameters: HashMap<String, Value>,
    pub expected_quality: String,
}

// ============================================================================
// Mock Data
// ============================================================================

fn build_methods() -> Vec<CoarseGrainingMethod> {
    vec![
        CoarseGrainingMethod {
            id: "QC".into(),
            name: "Quasicontinuum (QC)".into(),
            description: "Concurrent atomistic-to-continuum coupling method that adaptively refines the mesh near defects while using continuum approximation elsewhere. Reduces degrees of freedom by 2-3 orders of magnitude.".into(),
            source_scales: vec!["atomistic".into()],
            target_scales: vec!["continuum".into()],
            complexity: "high".into(),
            accuracy: "high".into(),
            typical_speedup: 100.0,
            parameters: vec![
                MethodParameter { name: "mesh_refinement_criteria".into(), type_: "string".into(), default_value: Value::String("energy_based".into()), description: "Criteria for adaptive mesh refinement".into(), range: None },
                MethodParameter { name: "max_element_size".into(), type_: "float".into(), default_value: Value::Number(serde_json::Number::from_f64(10.0).unwrap()), description: "Maximum finite element size in Angstroms".into(), range: Some((1.0, 100.0)) },
                MethodParameter { name: "pad_size".into(), type_: "integer".into(), default_value: Value::Number(serde_json::Number::from(5)), description: "Ghost atom padding layer count".into(), range: Some((1.0, 20.0)) },
            ],
            use_cases: vec!["Dislocation nucleation".into(), "Crack propagation".into(), "Nanoindentation".into()],
        },
        CoarseGrainingMethod {
            id: "MQC".into(),
            name: "Multiresolution QC (MQC)".into(),
            description: "Extended QC method with multiple levels of resolution. Uses a hierarchical mesh that allows for gradual transition between atomistic and continuum regions.".into(),
            source_scales: vec!["atomistic".into()],
            target_scales: vec!["continuum".into()],
            complexity: "high".into(),
            accuracy: "high".into(),
            typical_speedup: 200.0,
            parameters: vec![
                MethodParameter { name: "num_levels".into(), type_: "integer".into(), default_value: Value::Number(serde_json::Number::from(3)), description: "Number of resolution levels".into(), range: Some((2.0, 8.0)) },
                MethodParameter { name: "refinement_ratio".into(), type_: "float".into(), default_value: Value::Number(serde_json::Number::from_f64(2.0).unwrap()), description: "Refinement ratio between levels".into(), range: Some((1.5, 4.0)) },
            ],
            use_cases: vec!["Large-scale defect simulation".into(), "Grain boundary migration".into(), "Phase transformation".into()],
        },
        CoarseGrainingMethod {
            id: "radial_average".into(),
            name: "Radial Distribution Averaging".into(),
            description: "Coarse-grains atomistic data by computing radial distribution functions and reconstructing smooth fields. Fast and robust, suitable for isotropic systems.".into(),
            source_scales: vec!["atomistic".into()],
            target_scales: vec!["mesoscale".into()],
            complexity: "low".into(),
            accuracy: "medium".into(),
            typical_speedup: 1000.0,
            parameters: vec![
                MethodParameter { name: "bin_width".into(), type_: "float".into(), default_value: Value::Number(serde_json::Number::from_f64(0.1).unwrap()), description: "Radial bin width in Angstroms".into(), range: Some((0.01, 1.0)) },
                MethodParameter { name: "max_radius".into(), type_: "float".into(), default_value: Value::Number(serde_json::Number::from_f64(15.0).unwrap()), description: "Maximum radial distance".into(), range: Some((1.0, 50.0)) },
                MethodParameter { name: "num_samples".into(), type_: "integer".into(), default_value: Value::Number(serde_json::Number::from(100)), description: "Number of angular samples for anisotropic systems".into(), range: Some((10.0, 1000.0)) },
            ],
            use_cases: vec!["Density field reconstruction".into(), "Stress field averaging".into(), "Temperature mapping".into()],
        },
        CoarseGrainingMethod {
            id: "fourier_filter".into(),
            name: "Fourier Space Filtering".into(),
            description: "Coarse-grains by applying a low-pass filter in Fourier/reciprocal space. Preserves long-wavelength features while removing atomistic noise. Excellent for periodic systems.".into(),
            source_scales: vec!["atomistic".into()],
            target_scales: vec!["mesoscale".into()],
            complexity: "medium".into(),
            accuracy: "medium".into(),
            typical_speedup: 500.0,
            parameters: vec![
                MethodParameter { name: "cutoff_wavenumber".into(), type_: "float".into(), default_value: Value::Number(serde_json::Number::from_f64(0.5).unwrap()), description: "Maximum wavenumber to retain (1/Angstrom)".into(), range: Some((0.01, 5.0)) },
                MethodParameter { name: "filter_type".into(), type_: "string".into(), default_value: Value::String("gaussian".into()), description: "Filter shape: gaussian, sharp, lanczos".into(), range: None },
                MethodParameter { name: "windowing".into(), type_: "string".into(), default_value: Value::String("hann".into()), description: "Window function to reduce spectral leakage".into(), range: None },
            ],
            use_cases: vec!["Phonon-related field extraction".into(), "Long-wavelength elastic fields".into(), "Composition field smoothing".into()],
        },
        CoarseGrainingMethod {
            id: "ml_mapping".into(),
            name: "Machine Learning Mapping".into(),
            description: "Uses trained neural networks or Gaussian process regression to learn the mapping between atomistic and continuum representations. Can capture complex nonlinear relationships.".into(),
            source_scales: vec!["atomistic".into()],
            target_scales: vec!["mesoscale".into(), "continuum".into()],
            complexity: "high".into(),
            accuracy: "high".into(),
            typical_speedup: 5000.0,
            parameters: vec![
                MethodParameter { name: "model_type".into(), type_: "string".into(), default_value: Value::String("neural_network".into()), description: "ML model type: neural_network, gp_regression, kernel_ridge".into(), range: None },
                MethodParameter { name: "hidden_layers".into(), type_: "string".into(), default_value: Value::String("128,64,32".into()), description: "Hidden layer sizes (comma-separated)".into(), range: None },
                MethodParameter { name: "training_epochs".into(), type_: "integer".into(), default_value: Value::Number(serde_json::Number::from(500)), description: "Number of training epochs".into(), range: Some((10.0, 10000.0)) },
                MethodParameter { name: "learning_rate".into(), type_: "float".into(), default_value: Value::Number(serde_json::Number::from_f64(0.001).unwrap()), description: "Learning rate for optimization".into(), range: Some((1e-6, 0.1)) },
            ],
            use_cases: vec!["Nonlinear coarse-graining".into(), "Defect-property mapping".into(), "Complex microstructure features".into()],
        },
        CoarseGrainingMethod {
            id: "voronoi".into(),
            name: "Voronoi Tessellation".into(),
            description: "Partitions space into Voronoi cells around representative atoms and assigns averaged properties to each cell. Natural coarse-graining for irregular atomic arrangements.".into(),
            source_scales: vec!["atomistic".into()],
            target_scales: vec!["mesoscale".into()],
            complexity: "medium".into(),
            accuracy: "medium".into(),
            typical_speedup: 200.0,
            parameters: vec![
                MethodParameter { name: "sampling_rate".into(), type_: "float".into(), default_value: Value::Number(serde_json::Number::from_f64(0.1).unwrap()), description: "Fraction of atoms to use as Voronoi centers".into(), range: Some((0.01, 1.0)) },
                MethodParameter { name: "weighting".into(), type_: "string".into(), default_value: Value::String("volume".into()), description: "Cell weighting: volume, mass, uniform".into(), range: None },
            ],
            use_cases: vec!["Polycrystalline grain averaging".into(), "Local property mapping".into(), "Amorphous material coarse-graining".into()],
        },
    ]
}

fn build_presets() -> Vec<CoarseGrainingPreset> {
    let mut p1_params = HashMap::new();
    p1_params.insert("mesh_refinement_criteria".into(), Value::String("energy_based".into()));
    p1_params.insert("max_element_size".into(), Value::Number(serde_json::Number::from_f64(10.0).unwrap()));
    p1_params.insert("pad_size".into(), Value::Number(serde_json::Number::from(5)));

    let mut p2_params = HashMap::new();
    p2_params.insert("cutoff_wavenumber".into(), Value::Number(serde_json::Number::from_f64(0.3).unwrap()));
    p2_params.insert("filter_type".into(), Value::String("gaussian".into()));

    let mut p3_params = HashMap::new();
    p3_params.insert("model_type".into(), Value::String("neural_network".into()));
    p3_params.insert("hidden_layers".into(), Value::String("256,128,64".into()));
    p3_params.insert("training_epochs".into(), Value::Number(serde_json::Number::from(1000)));

    let mut p4_params = HashMap::new();
    p4_params.insert("sampling_rate".into(), Value::Number(serde_json::Number::from_f64(0.05).unwrap()));
    p4_params.insert("weighting".into(), Value::String("volume".into()));

    let mut p5_params = HashMap::new();
    p5_params.insert("num_levels".into(), Value::Number(serde_json::Number::from(4)));
    p5_params.insert("refinement_ratio".into(), Value::Number(serde_json::Number::from_f64(2.0).unwrap()));

    vec![
        CoarseGrainingPreset {
            id: "qc_crack_propagation".into(),
            name: "QC for Crack Propagation".into(),
            description: "Quasicontinuum setup optimized for simulating crack propagation in single crystals.".into(),
            scenario: "fracture_mechanics".into(),
            method: "QC".into(),
            parameters: p1_params,
            expected_quality: "high".into(),
        },
        CoarseGrainingPreset {
            id: "fourier_phonon_extraction".into(),
            name: "Fourier Phonon Extraction".into(),
            description: "Fourier filtering configured to extract long-wavelength phonon-related displacement fields from MD trajectories.".into(),
            scenario: "phonon_analysis".into(),
            method: "fourier_filter".into(),
            parameters: p2_params,
            expected_quality: "medium".into(),
        },
        CoarseGrainingPreset {
            id: "ml_defect_mapping".into(),
            name: "ML Defect-Property Mapping".into(),
            description: "Neural network trained to map local atomic environments to continuum defect properties (e.g., dislocation density, vacancy concentration).".into(),
            scenario: "defect_analysis".into(),
            method: "ml_mapping".into(),
            parameters: p3_params,
            expected_quality: "high".into(),
        },
        CoarseGrainingPreset {
            id: "voronoi_grain_averaging".into(),
            name: "Voronoi Grain Averaging".into(),
            description: "Voronoi tessellation for averaging per-atom stress/strain within polycrystalline grains.".into(),
            scenario: "polycrystal".into(),
            method: "voronoi".into(),
            parameters: p4_params,
            expected_quality: "medium".into(),
        },
        CoarseGrainingPreset {
            id: "mqc_phase_transformation".into(),
            name: "MQC for Phase Transformation".into(),
            description: "Multiresolution QC setup for simulating solid-state phase transformations with adaptive resolution.".into(),
            scenario: "phase_transformation".into(),
            method: "MQC".into(),
            parameters: p5_params,
            expected_quality: "high".into(),
        },
    ]
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Run a coarse-graining operation with the specified method and configuration.
#[command]
pub fn run_coarse_graining(config: CoarseGrainingConfig) -> Result<CoarseGrainingResult, String> {
    tracing::info!(
        method = %config.method,
        source = %config.source_scale,
        target = %config.target_scale,
        "run_coarse_graining called"
    );

    let valid_methods = ["QC", "MQC", "radial_average", "fourier_filter", "ml_mapping", "voronoi"];
    if !valid_methods.contains(&config.method.as_str()) {
        return Err(format!(
            "Unknown method '{}'. Valid: {}",
            config.method,
            valid_methods.join(", ")
        ));
    }

    let total_cells = (config.target_resolution[0] as u64)
        * (config.target_resolution[1] as u64)
        * (config.target_resolution[2] as u64);

    // Generate mock field data
    let mut field_data = Vec::with_capacity(total_cells as usize);
    let mut state: u64 = 42;
    for _ in 0..total_cells {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let val = (state >> 11) as f64 / u32::MAX as f64;
        field_data.push(val * 2.0 - 1.0); // range [-1, 1]
    }

    let id = uuid::Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now().to_rfc3339();

    // Mock quality metrics based on method
    let (quality, comp_time) = match config.method.as_str() {
        "QC" => {
            let mut m = HashMap::new();
            m.insert("correlation".into(), 0.95);
            m.insert("rmse".into(), 0.032);
            m.insert("relative_error".into(), 0.041);
            m.insert("energy_conservation".into(), 0.98);
            (m, 2450.0)
        }
        "MQC" => {
            let mut m = HashMap::new();
            m.insert("correlation".into(), 0.96);
            m.insert("rmse".into(), 0.028);
            m.insert("relative_error".into(), 0.035);
            m.insert("energy_conservation".into(), 0.99);
            (m, 3200.0)
        }
        "radial_average" => {
            let mut m = HashMap::new();
            m.insert("correlation".into(), 0.82);
            m.insert("rmse".into(), 0.089);
            m.insert("relative_error".into(), 0.112);
            m.insert("energy_conservation".into(), 0.91);
            (m, 120.0)
        }
        "fourier_filter" => {
            let mut m = HashMap::new();
            m.insert("correlation".into(), 0.88);
            m.insert("rmse".into(), 0.065);
            m.insert("relative_error".into(), 0.078);
            m.insert("energy_conservation".into(), 0.94);
            (m, 350.0)
        }
        "ml_mapping" => {
            let mut m = HashMap::new();
            m.insert("correlation".into(), 0.93);
            m.insert("rmse".into(), 0.042);
            m.insert("relative_error".into(), 0.051);
            m.insert("energy_conservation".into(), 0.96);
            (m, 85.0)
        }
        "voronoi" => {
            let mut m = HashMap::new();
            m.insert("correlation".into(), 0.85);
            m.insert("rmse".into(), 0.075);
            m.insert("relative_error".into(), 0.092);
            m.insert("energy_conservation".into(), 0.93);
            (m, 580.0)
        }
        _ => (HashMap::new(), 0.0),
    };

    tracing::info!(
        id = %id,
        cells = total_cells,
        time_ms = comp_time,
        "Coarse-graining complete"
    );

    Ok(CoarseGrainingResult {
        id,
        method: config.method,
        source_points: config.source_data_size,
        target_grid_size: config.target_resolution,
        field_data,
        quality_metrics: quality,
        computation_time_ms: comp_time,
        created_at,
    })
}

/// Recommend the best coarse-graining method for the given scenario.
#[command]
pub fn recommend_method(
    scenario: String,
    data_size: Option<u64>,
    accuracy_requirement: Option<String>,
) -> Result<CoarseGrainingMethod, String> {
    tracing::info!(
        scenario = %scenario,
        data_size = data_size,
        accuracy = accuracy_requirement.as_deref().unwrap_or("any"),
        "recommend_method called"
    );

    let methods = build_methods();

    let recommendation = match scenario.to_lowercase().as_str() {
        "fracture" | "crack" | "dislocation" => methods.iter().find(|m| m.id == "QC"),
        "phase_transformation" | "grain_boundary" => methods.iter().find(|m| m.id == "MQC"),
        "phonon" | "vibration" | "elastic_wave" => methods.iter().find(|m| m.id == "fourier_filter"),
        "density" | "stress_field" | "temperature_field" => methods.iter().find(|m| m.id == "radial_average"),
        "defect" | "nonlinear" | "complex" => methods.iter().find(|m| m.id == "ml_mapping"),
        "polycrystal" | "grain" | "amorphous" => methods.iter().find(|m| m.id == "voronoi"),
        _ => methods.iter().find(|m| m.id == "fourier_filter"), // default
    };

    let method = recommendation
        .cloned()
        .unwrap_or_else(|| methods[0].clone());

    tracing::info!(recommended = %method.id, "Method recommended");
    Ok(method)
}

/// Compare multiple coarse-graining methods side by side.
#[command]
pub fn compare_methods(
    method_ids: Vec<String>,
    _test_data_size: Option<u64>,
) -> Result<MethodComparison, String> {
    tracing::info!(
        methods = ?method_ids,
        "compare_methods called"
    );

    let all_methods = build_methods();
    let mut entries = Vec::new();

    for mid in &method_ids {
        let _method = all_methods.iter().find(|m| m.id == *mid);
        let (acc, speed, mem, robust, time) = match mid.as_str() {
            "QC" => (0.92, 0.45, 0.60, 0.85, 2450.0),
            "MQC" => (0.94, 0.40, 0.55, 0.88, 3200.0),
            "radial_average" => (0.75, 0.95, 0.90, 0.90, 120.0),
            "fourier_filter" => (0.82, 0.80, 0.85, 0.82, 350.0),
            "ml_mapping" => (0.90, 0.92, 0.70, 0.65, 85.0),
            "voronoi" => (0.78, 0.70, 0.80, 0.88, 580.0),
            _ => (0.5, 0.5, 0.5, 0.5, 1000.0),
        };

        let mut quality = HashMap::new();
        quality.insert("correlation".into(), acc);
        quality.insert("relative_error".into(), 1.0 - acc);
        quality.insert("speedup_factor".into(), match mid.as_str() {
            "QC" => 100.0, "MQC" => 200.0, "radial_average" => 1000.0,
            "fourier_filter" => 500.0, "ml_mapping" => 5000.0, "voronoi" => 200.0,
            _ => 100.0,
        });

        let overall = 0.3 * acc + 0.25 * speed + 0.2 * mem + 0.25 * robust;

        entries.push(MethodComparisonEntry {
            method_id: mid.clone(),
            accuracy_score: acc,
            speed_score: speed,
            memory_score: mem,
            robustness_score: robust,
            overall_score: overall,
            computation_time_ms: time,
            quality_metrics: quality,
        });
    }

    // Sort by overall score descending
    entries.sort_by(|a, b| b.overall_score.partial_cmp(&a.overall_score).unwrap());

    let best = entries.first().map(|e| e.method_id.clone()).unwrap_or_default();
    let reason = format!(
        "Method '{}' achieves the highest overall score ({:.3}) based on the weighted combination of accuracy (30%), speed (25%), memory (20%), and robustness (25%).",
        best,
        entries.first().map(|e| e.overall_score).unwrap_or(0.0)
    );

    tracing::info!(recommendation = %best, "Comparison complete");
    Ok(MethodComparison {
        methods: entries,
        recommendation: best,
        recommendation_reason: reason,
    })
}

/// Get available coarse-graining presets for common scenarios.
#[command]
pub fn get_coarse_graining_presets() -> Result<Vec<CoarseGrainingPreset>, String> {
    tracing::info!("get_coarse_graining_presets called");

    let presets = build_presets();
    tracing::info!("Found {} presets", presets.len());
    Ok(presets)
}
