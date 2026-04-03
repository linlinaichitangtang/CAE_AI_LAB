// Error Tracking Commands - V1.8
// Error propagation pipeline for multiscale simulations: DFT -> MD -> PF -> FE.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tauri::command;

// ============================================================================
// Data Structures
// ============================================================================

/// A single step in an error propagation pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPipelineStep {
    pub id: String,
    pub step_index: u32,
    pub name: String,
    pub method: String,           // "DFT", "MD", "PF", "FE", "experiment"
    pub description: String,
    pub input_error: f64,         // Error coming into this step (relative %)
    pub intrinsic_error: f64,     // Error introduced by this step (relative %)
    pub output_error: f64,        // Error going out of this step (relative %)
    pub error_sources: Vec<ErrorSource>,
    pub mitigation_strategies: Vec<String>,
    pub parameters: HashMap<String, Value>,
}

/// A specific source of error within a pipeline step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorSource {
    pub name: String,
    pub category: String,         // "model", "numerical", "parametric", "data"
    pub magnitude: f64,           // Relative error contribution (%)
    pub description: String,
    pub reducible: bool,
}

/// A complete error propagation pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPipeline {
    pub id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<ErrorPipelineStep>,
    pub total_propagated_error: f64,
    pub created_at: String,
    pub updated_at: String,
}

/// An error contribution to add to a pipeline step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContribution {
    pub pipeline_id: String,
    pub step_id: String,
    pub source_name: String,
    pub category: String,
    pub magnitude: f64,
    pub description: String,
    pub reducible: bool,
}

/// A generated error report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorReport {
    pub pipeline_id: String,
    pub pipeline_name: String,
    pub generated_at: String,
    pub summary: ErrorReportSummary,
    pub step_details: Vec<StepErrorDetail>,
    pub recommendations: Vec<String>,
    pub critical_errors: Vec<CriticalError>,
}

/// Summary statistics for the error report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorReportSummary {
    pub total_propagated_error: f64,
    pub dominant_error_source: String,
    pub dominant_step: String,
    pub reducible_error_fraction: f64,
    pub confidence_level: f64,
}

/// Detailed error information for a single step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepErrorDetail {
    pub step_name: String,
    pub step_method: String,
    pub input_error: f64,
    pub output_error: f64,
    pub error_growth: f64,
    pub error_sources: Vec<ErrorSourceDetail>,
}

/// Detailed error source information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorSourceDetail {
    pub name: String,
    pub category: String,
    pub magnitude: f64,
    pub fraction_of_step_error: f64,
    pub reducible: bool,
}

/// A critical error that exceeds acceptable thresholds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticalError {
    pub step_name: String,
    pub source_name: String,
    pub magnitude: f64,
    pub threshold: f64,
    pub severity: String, // "warning", "critical", "fatal"
    pub recommendation: String,
}

/// Result of a sensitivity analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityAnalysisResult {
    pub pipeline_id: String,
    pub analysis_type: String,
    pub sensitivities: Vec<SensitivityEntry>,
    pub most_sensitive_parameter: String,
    pub created_at: String,
}

/// A single sensitivity entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityEntry {
    pub parameter: String,
    pub step: String,
    pub sensitivity_index: f64,   // d(output_error) / d(parameter)
    pub normalized_sensitivity: f64,
    pub rank: u32,
}

// ============================================================================
// Mock Data
// ============================================================================

fn build_default_pipeline() -> ErrorPipeline {
    let steps = vec![
        ErrorPipelineStep {
            id: "step_dft".into(),
            step_index: 0,
            name: "DFT Calculation".into(),
            method: "DFT".into(),
            description: "First-principles density functional theory calculation for electronic structure and total energies.".into(),
            input_error: 0.0,
            intrinsic_error: 2.5,
            output_error: 2.5,
            error_sources: vec![
                ErrorSource { name: "Exchange-correlation functional".into(), category: "model".into(), magnitude: 1.8, description: "Approximation error in XC functional (e.g., PBE vs. SCAN)".into(), reducible: true },
                ErrorSource { name: "Basis set incompleteness".into(), category: "numerical".into(), magnitude: 0.4, description: "Finite plane-wave cutoff or basis set size".into(), reducible: true },
                ErrorSource { name: "k-point sampling".into(), category: "numerical".into(), magnitude: 0.2, description: "Finite k-point mesh density".into(), reducible: true },
                ErrorSource { name: "Pseudopotential".into(), category: "model".into(), magnitude: 0.1, description: "Pseudopotential approximation for core electrons".into(), reducible: false },
            ],
            mitigation_strategies: vec![
                "Use higher-rung XC functionals (SCAN, r2SCAN, hybrid)".into(),
                "Increase plane-wave cutoff to 600+ eV".into(),
                "Use denser k-point meshes (>= 12x12x12)".into(),
                "Validate with all-electron calculations".into(),
            ],
            parameters: {
                let mut p = HashMap::new();
                p.insert("xc_functional".into(), Value::String("PBE".into()));
                p.insert("energy_cutoff_eV".into(), Value::Number(serde_json::Number::from(520)));
                p.insert("kpoint_density".into(), Value::String("8x8x8".into()));
                p
            },
        },
        ErrorPipelineStep {
            id: "step_md".into(),
            step_index: 1,
            name: "Molecular Dynamics".into(),
            method: "MD".into(),
            description: "Classical molecular dynamics simulation using interatomic potentials fitted to DFT data.".into(),
            input_error: 2.5,
            intrinsic_error: 3.0,
            output_error: 5.2,
            error_sources: vec![
                ErrorSource { name: "Potential fitting error".into(), category: "model".into(), magnitude: 2.0, description: "ML potential or empirical potential deviation from DFT reference".into(), reducible: true },
                ErrorSource { name: "Finite timestep".into(), category: "numerical".into(), magnitude: 0.5, description: "Integration error from finite timestep (typically 1 fs)".into(), reducible: true },
                ErrorSource { name: "Finite system size".into(), category: "parametric".into(), magnitude: 0.3, description: "Finite simulation box size effects on properties".into(), reducible: true },
                ErrorSource { name: "Thermostat/barostat".into(), category: "numerical".into(), magnitude: 0.2, description: "Thermostat and barostat coupling artifacts".into(), reducible: false },
            ],
            mitigation_strategies: vec![
                "Use higher-quality training data for potential fitting".into(),
                "Validate potential on test set outside training distribution".into(),
                "Use smaller timestep (0.5 fs) for stiff interactions".into(),
                "Use larger simulation boxes (> 10k atoms)".into(),
            ],
            parameters: {
                let mut p = HashMap::new();
                p.insert("potential_type".into(), Value::String("NEP".into()));
                p.insert("timestep_fs".into(), Value::Number(serde_json::Number::from_f64(1.0).unwrap()));
                p.insert("num_atoms".into(), Value::Number(serde_json::Number::from(10000)));
                p
            },
        },
        ErrorPipelineStep {
            id: "step_pf".into(),
            step_index: 2,
            name: "Phase Field Simulation".into(),
            method: "PF".into(),
            description: "Phase-field simulation for microstructure evolution using parameters from MD/DFT.".into(),
            input_error: 5.2,
            intrinsic_error: 4.0,
            output_error: 8.8,
            error_sources: vec![
                ErrorSource { name: "Free energy parameters".into(), category: "parametric".into(), magnitude: 2.5, description: "Uncertainty in GL free energy coefficients from DFT/MD".into(), reducible: true },
                ErrorSource { name: "Interface width".into(), category: "model".into(), magnitude: 1.0, description: "Diffuse interface approximation vs. sharp interface".into(), reducible: false },
                ErrorSource { name: "Grid resolution".into(), category: "numerical".into(), magnitude: 0.3, description: "Finite grid spacing effects on interface profile".into(), reducible: true },
                ErrorSource { name: "Mobility calibration".into(), category: "parametric".into(), magnitude: 0.2, description: "Uncertainty in atomic mobility from MD kinetic data".into(), reducible: true },
            ],
            mitigation_strategies: vec![
                "Use Bayesian calibration for free energy parameters".into(),
                "Perform grid convergence study".into(),
                "Validate interface properties against atomistic data".into(),
                "Use higher-order time integration schemes".into(),
            ],
            parameters: {
                let mut p = HashMap::new();
                p.insert("equation_type".into(), Value::String("Allen-Cahn".into()));
                p.insert("grid_size".into(), Value::String("256x256x256".into()));
                p.insert("interface_width_nm".into(), Value::Number(serde_json::Number::from_f64(5.0).unwrap()));
                p
            },
        },
        ErrorPipelineStep {
            id: "step_homogenization".into(),
            step_index: 3,
            name: "Homogenization".into(),
            method: "FE_pre".into(),
            description: "Homogenization of phase-field microstructure to effective continuum properties.".into(),
            input_error: 8.8,
            intrinsic_error: 2.0,
            output_error: 10.5,
            error_sources: vec![
                ErrorSource { name: "RVE size".into(), category: "parametric".into(), magnitude: 1.2, description: "Representative volume element may not capture full microstructure variability".into(), reducible: true },
                ErrorSource { name: "Homogenization scheme".into(), category: "model".into(), magnitude: 0.5, description: "Analytical homogenization (Voigt/Reuss/MT) vs. numerical FE-based".into(), reducible: true },
                ErrorSource { name: "Phase identification".into(), category: "data".into(), magnitude: 0.3, description: "Threshold-based phase identification from continuous order parameters".into(), reducible: true },
            ],
            mitigation_strategies: vec![
                "Use larger RVEs with periodic boundary conditions".into(),
                "Use numerical FE-based homogenization for complex microstructures".into(),
                "Perform statistical analysis over multiple RVE realizations".into(),
            ],
            parameters: {
                let mut p = HashMap::new();
                p.insert("method".into(), Value::String("Mori-Tanaka".into()));
                p.insert("rve_size_nm".into(), Value::Number(serde_json::Number::from(200)));
                p.insert("num_realizations".into(), Value::Number(serde_json::Number::from(10)));
                p
            },
        },
        ErrorPipelineStep {
            id: "step_fe".into(),
            step_index: 4,
            name: "Finite Element Analysis".into(),
            method: "FE".into(),
            description: "Structural finite element analysis using homogenized material properties.".into(),
            input_error: 10.5,
            intrinsic_error: 1.5,
            output_error: 11.8,
            error_sources: vec![
                ErrorSource { name: "Mesh quality".into(), category: "numerical".into(), magnitude: 0.6, description: "Finite element mesh density and quality effects".into(), reducible: true },
                ErrorSource { name: "Boundary conditions".into(), category: "parametric".into(), magnitude: 0.5, description: "Simplification of real boundary conditions in FE model".into(), reducible: false },
                ErrorSource { name: "Solver tolerance".into(), category: "numerical".into(), magnitude: 0.2, description: "Nonlinear solver convergence tolerance".into(), reducible: true },
                ErrorSource { name: "Geometric simplification".into(), category: "model".into(), magnitude: 0.2, description: "CAD model simplification and defeaturing".into(), reducible: false },
            ],
            mitigation_strategies: vec![
                "Perform mesh convergence study".into(),
                "Use adaptive mesh refinement in stress concentration regions".into(),
                "Validate boundary conditions with experimental measurements".into(),
                "Use tighter solver convergence criteria".into(),
            ],
            parameters: {
                let mut p = HashMap::new();
                p.insert("element_type".into(), Value::String("C3D20R".into()));
                p.insert("solver".into(), Value::String("ABAQUS".into()));
                p.insert("mesh_size_mm".into(), Value::Number(serde_json::Number::from_f64(0.5).unwrap()));
                p
            },
        },
    ];

    let now = chrono::Utc::now().to_rfc3339();
    ErrorPipeline {
        id: uuid::Uuid::new_v4().to_string(),
        name: "DFT-MD-PF-FE Standard Pipeline".into(),
        description: "Standard multiscale error propagation pipeline from first-principles DFT through MD, phase-field, homogenization, to FE analysis.".into(),
        steps,
        total_propagated_error: 11.8,
        created_at: now.clone(),
        updated_at: now,
    }
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Create a new error propagation pipeline.
#[command]
pub fn create_error_pipeline(
    name: String,
    description: Option<String>,
    steps_config: Option<Vec<Value>>,
) -> Result<ErrorPipeline, String> {
    tracing::info!(name = %name, "create_error_pipeline called");

    if name.is_empty() {
        return Err("Pipeline name cannot be empty".to_string());
    }

    let pipeline = if let Some(_config) = steps_config {
        // In a real implementation, build pipeline from config
        build_default_pipeline()
    } else {
        build_default_pipeline()
    };

    tracing::info!(
        id = %pipeline.id,
        steps = pipeline.steps.len(),
        total_error = pipeline.total_propagated_error,
        "Error pipeline created"
    );

    Ok(ErrorPipeline {
        id: pipeline.id,
        name,
        description: description.unwrap_or(pipeline.description),
        steps: pipeline.steps,
        total_propagated_error: pipeline.total_propagated_error,
        created_at: pipeline.created_at,
        updated_at: pipeline.updated_at,
    })
}

/// Add an error contribution to an existing pipeline step.
#[command]
pub fn add_error_contribution(
    contribution: ErrorContribution,
) -> Result<ErrorPipelineStep, String> {
    tracing::info!(
        pipeline = %contribution.pipeline_id,
        step = %contribution.step_id,
        source = %contribution.source_name,
        magnitude = contribution.magnitude,
        "add_error_contribution called"
    );

    if contribution.magnitude < 0.0 {
        return Err("Error magnitude must be non-negative".to_string());
    }

    // Mock: return an updated step with the new contribution
    let new_source = ErrorSource {
        name: contribution.source_name,
        category: contribution.category,
        magnitude: contribution.magnitude,
        description: contribution.description,
        reducible: contribution.reducible,
    };

    let updated_step = ErrorPipelineStep {
        id: contribution.step_id,
        step_index: 0,
        name: "Updated Step".into(),
        method: "custom".into(),
        description: "Step with added error contribution".into(),
        input_error: 5.0,
        intrinsic_error: contribution.magnitude + 2.0,
        output_error: 5.0 + contribution.magnitude + 2.0,
        error_sources: vec![new_source],
        mitigation_strategies: vec!["Review and validate data source".into()],
        parameters: HashMap::new(),
    };

    tracing::info!(
        new_output_error = updated_step.output_error,
        "Error contribution added"
    );

    Ok(updated_step)
}

/// Get the propagation steps of an error pipeline.
#[command]
pub fn get_propagation_steps(pipeline_id: String) -> Result<ErrorPipeline, String> {
    tracing::info!(pipeline = %pipeline_id, "get_propagation_steps called");

    let pipeline = build_default_pipeline();

    tracing::info!(
        steps = pipeline.steps.len(),
        total_error = pipeline.total_propagated_error,
        "Propagation steps retrieved"
    );

    Ok(pipeline)
}

/// Generate a comprehensive error report for a pipeline.
#[command]
pub fn generate_error_report(
    pipeline_id: String,
    include_recommendations: Option<bool>,
) -> Result<ErrorReport, String> {
    tracing::info!(
        pipeline = %pipeline_id,
        "generate_error_report called"
    );

    let pipeline = build_default_pipeline();
    let include_recs = include_recommendations.unwrap_or(true);

    let mut step_details = Vec::new();
    let mut critical_errors = Vec::new();
    let mut dominant_source = ("", 0.0_f64);
    let mut dominant_step = ("", 0.0_f64);
    let mut total_reducible = 0.0_f64;
    let mut total_magnitude = 0.0_f64;

    for step in &pipeline.steps {
        let error_growth = step.output_error - step.input_error;

        let sources: Vec<ErrorSourceDetail> = step
            .error_sources
            .iter()
            .map(|s| {
                let frac = if step.intrinsic_error > 0.0 {
                    s.magnitude / step.intrinsic_error
                } else {
                    0.0
                };
                if s.reducible {
                    total_reducible += s.magnitude;
                }
                total_magnitude += s.magnitude;
                if s.magnitude > dominant_source.1 {
                    dominant_source = (&s.name, s.magnitude);
                }
                ErrorSourceDetail {
                    name: s.name.clone(),
                    category: s.category.clone(),
                    magnitude: s.magnitude,
                    fraction_of_step_error: frac,
                    reducible: s.reducible,
                }
            })
            .collect();

        if error_growth > dominant_step.1 {
            dominant_step = (&step.name, error_growth);
        }

        // Check for critical errors (> 3% contribution)
        for s in &step.error_sources {
            if s.magnitude > 3.0 {
                critical_errors.push(CriticalError {
                    step_name: step.name.clone(),
                    source_name: s.name.clone(),
                    magnitude: s.magnitude,
                    threshold: 3.0,
                    severity: if s.magnitude > 5.0 { "fatal".into() } else { "critical".into() },
                    recommendation: if s.reducible {
                        format!("Reduce '{}' error in '{}' step through improved modeling or calibration", s.name, step.name)
                    } else {
                        format!("Acknowledge irreducible '{}' error in '{}' step and account for it in uncertainty quantification", s.name, step.name)
                    },
                });
            } else if s.magnitude > 2.0 {
                critical_errors.push(CriticalError {
                    step_name: step.name.clone(),
                    source_name: s.name.clone(),
                    magnitude: s.magnitude,
                    threshold: 2.0,
                    severity: "warning".into(),
                    recommendation: format!("Monitor '{}' error in '{}' step", s.name, step.name),
                });
            }
        }

        step_details.push(StepErrorDetail {
            step_name: step.name.clone(),
            step_method: step.method.clone(),
            input_error: step.input_error,
            output_error: step.output_error,
            error_growth,
            error_sources: sources,
        });
    }

    let reducible_fraction = if total_magnitude > 0.0 {
        total_reducible / total_magnitude
    } else {
        0.0
    };

    let confidence_level = (100.0 - pipeline.total_propagated_error).max(0.0);

    let mut recommendations = Vec::new();
    if include_recs {
        recommendations.push(format!(
            "Focus error reduction on '{}' in '{}' step (dominant source, {:.1}%)",
            dominant_source.0, dominant_step.0, dominant_source.1
        ));
        recommendations.push(format!(
            "{:.1}% of total error is reducible through improved modeling and calibration",
            reducible_fraction * 100.0
        ));
        recommendations.push(
            "Use Bayesian uncertainty quantification to propagate error bounds through the pipeline".into(),
        );
        recommendations.push(
            "Validate intermediate results against experimental data at each scale transition".into(),
        );
    }

    let report = ErrorReport {
        pipeline_id: pipeline.id.clone(),
        pipeline_name: pipeline.name,
        generated_at: chrono::Utc::now().to_rfc3339(),
        summary: ErrorReportSummary {
            total_propagated_error: pipeline.total_propagated_error,
            dominant_error_source: dominant_source.0.to_string(),
            dominant_step: dominant_step.0.to_string(),
            reducible_error_fraction: reducible_fraction,
            confidence_level,
        },
        step_details,
        recommendations,
        critical_errors,
    };

    let critical_count = report.critical_errors.len();
    tracing::info!(
        total_error = report.summary.total_propagated_error,
        critical_count,
        "Error report generated"
    );

    Ok(report)
}

/// Run a sensitivity analysis on pipeline parameters.
#[command]
pub fn run_sensitivity_analysis(
    pipeline_id: String,
    analysis_type: Option<String>,
    parameter_range: Option<f64>,
) -> Result<SensitivityAnalysisResult, String> {
    tracing::info!(
        pipeline = %pipeline_id,
        analysis_type = analysis_type.as_deref().unwrap_or("local"),
        "run_sensitivity_analysis called"
    );

    let pipeline = build_default_pipeline();
    let a_type = analysis_type.unwrap_or_else(|| "local".into());
    let perturbation = parameter_range.unwrap_or(0.1); // 10% perturbation

    let mut sensitivities = Vec::new();

    // Generate mock sensitivity data for each step's key parameters
    let param_sensitivities: Vec<(&str, &str, f64)> = vec![
        ("xc_functional", "DFT Calculation", 0.45),
        ("energy_cutoff_eV", "DFT Calculation", 0.12),
        ("potential_type", "Molecular Dynamics", 0.62),
        ("timestep_fs", "Molecular Dynamics", 0.08),
        ("num_atoms", "Molecular Dynamics", 0.15),
        ("interface_width_nm", "Phase Field Simulation", 0.28),
        ("grid_size", "Phase Field Simulation", 0.18),
        ("method", "Homogenization", 0.22),
        ("rve_size_nm", "Homogenization", 0.35),
        ("mesh_size_mm", "Finite Element Analysis", 0.10),
        ("solver", "Finite Element Analysis", 0.05),
    ];

    let mut sorted: Vec<_> = param_sensitivities.into_iter().collect();
    sorted.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    for (i, (param, step, sens)) in sorted.iter().enumerate() {
        sensitivities.push(SensitivityEntry {
            parameter: param.to_string(),
            step: step.to_string(),
            sensitivity_index: sens * perturbation,
            normalized_sensitivity: *sens,
            rank: (i + 1) as u32,
        });
    }

    let most_sensitive = sensitivities
        .first()
        .map(|s| s.parameter.clone())
        .unwrap_or_default();

    tracing::info!(
        most_sensitive = %most_sensitive,
        params_analyzed = sensitivities.len(),
        "Sensitivity analysis complete"
    );

    Ok(SensitivityAnalysisResult {
        pipeline_id: pipeline.id,
        analysis_type: a_type,
        sensitivities,
        most_sensitive_parameter: most_sensitive,
        created_at: chrono::Utc::now().to_rfc3339(),
    })
}
