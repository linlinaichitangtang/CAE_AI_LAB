// Benchmark Commands - V1.8
// Benchmark suite for validating multiscale simulation modules.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::command;

// ============================================================================
// Data Structures
// ============================================================================

/// A benchmark case definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkCase {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub difficulty: String,       // "basic", "intermediate", "advanced"
    pub estimated_time_s: f64,
    pub reference_values: HashMap<String, f64>,
    pub tolerance_percent: f64,
    pub tags: Vec<String>,
    pub required_modules: Vec<String>,
}

/// Detailed information about a benchmark case including setup and expected results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkDetail {
    pub benchmark: BenchmarkCase,
    pub setup_instructions: Vec<String>,
    pub expected_outputs: Vec<String>,
    pub validation_criteria: Vec<String>,
    pub known_issues: Vec<String>,
    pub references: Vec<String>,
}

/// Result of running a benchmark.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRunResult {
    pub benchmark_id: String,
    pub status: String,           // "passed", "failed", "skipped", "error"
    pub computed_values: HashMap<String, f64>,
    pub reference_values: HashMap<String, f64>,
    pub errors: HashMap<String, f64>,
    pub max_error_percent: f64,
    pub passed: bool,
    pub execution_time_ms: f64,
    pub timestamp: String,
    pub message: String,
}

/// Comparison of benchmark results with reference values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkComparison {
    pub benchmark_id: String,
    pub benchmark_name: String,
    pub comparisons: Vec<ValueComparison>,
    pub overall_passed: bool,
    pub summary: String,
}

/// Comparison of a single value against its reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueComparison {
    pub property: String,
    pub computed: f64,
    pub reference: f64,
    pub absolute_error: f64,
    pub relative_error_percent: f64,
    pub tolerance_percent: f64,
    pub passed: bool,
}

/// Aggregated benchmark statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkStatistics {
    pub total_benchmarks: u32,
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
    pub pass_rate: f64,
    pub average_execution_time_ms: f64,
    pub category_breakdown: HashMap<String, CategoryStats>,
    pub difficulty_breakdown: HashMap<String, DifficultyStats>,
    pub last_updated: String,
}

/// Statistics for a benchmark category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryStats {
    pub total: u32,
    pub passed: u32,
    pub failed: u32,
    pub pass_rate: f64,
}

/// Statistics for a difficulty level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyStats {
    pub total: u32,
    pub passed: u32,
    pub failed: u32,
    pub pass_rate: f64,
}

// ============================================================================
// Mock Data
// ============================================================================

fn build_benchmarks() -> Vec<BenchmarkCase> {
    let mut b1_ref = HashMap::new();
    b1_ref.insert("lattice_constant_A".into(), 3.615);
    b1_ref.insert("bulk_modulus_GPa".into(), 140.0);
    b1_ref.insert("cohesive_energy_eV".into(), -3.54);

    let mut b2_ref = HashMap::new();
    b2_ref.insert("elastic_constant_C11_GPa".into(), 168.0);
    b2_ref.insert("elastic_constant_C12_GPa".into(), 121.0);
    b2_ref.insert("elastic_constant_C44_GPa".into(), 75.0);

    let mut b3_ref = HashMap::new();
    b3_ref.insert("diffusion_coefficient_cm2_s".into(), 2.5e-11);
    b3_ref.insert("activation_energy_eV".into(), 0.85);

    let mut b4_ref = HashMap::new();
    b4_ref.insert("interface_velocity_nm_s".into(), 15.0);
    b4_ref.insert("interface_width_nm".into(), 5.0);

    let mut b5_ref = HashMap::new();
    b5_ref.insert("von_mises_stress_MPa".into(), 250.0);
    b5_ref.insert("max_displacement_mm".into(), 0.45);

    let mut b6_ref = HashMap::new();
    b6_ref.insert("natural_frequency_Hz".into(), 1250.0);
    b6_ref.insert("damping_ratio".into(), 0.02);

    let mut b7_ref = HashMap::new();
    b7_ref.insert("drag_coefficient".into(), 0.42);
    b7_ref.insert("lift_coefficient".into(), 0.35);

    let mut b8_ref = HashMap::new();
    b8_ref.insert("stress_intensity_KIc_MPa_m0_5".into(), 50.0);
    b8_ref.insert("crack_growth_rate".into(), 1e-8);

    let mut b9_ref = HashMap::new();
    b9_ref.insert("thermal_conductivity_W_mK".into(), 401.0);
    b9_ref.insert("specific_heat_J_kgK".into(), 385.0);

    let mut b10_ref = HashMap::new();
    b10_ref.insert("potential_energy_RMSE_meV".into(), 1.5);
    b10_ref.insert("force_RMSE_meV_A".into(), 50.0);

    let mut b11_ref = HashMap::new();
    b11_ref.insert("youngs_modulus_GPa".into(), 210.0);
    b11_ref.insert("poisson_ratio".into(), 0.30);

    let mut b12_ref = HashMap::new();
    b12_ref.insert("phase_fraction_alpha".into(), 0.65);
    b12_ref.insert("grain_size_um".into(), 25.0);

    vec![
        BenchmarkCase { id: "dft-copper-equilibrium".into(), name: "Copper Equilibrium Properties".into(), category: "DFT".into(), description: "DFT calculation of copper equilibrium lattice constant, bulk modulus, and cohesive energy.".into(), difficulty: "basic".into(), estimated_time_s: 120.0, reference_values: b1_ref, tolerance_percent: 5.0, tags: vec!["DFT".into(), "metals".into(), "equilibrium".into()], required_modules: vec!["dft_task".into()] },
        BenchmarkCase { id: "dft-elastic-constants".into(), name: "Elastic Constants Calculation".into(), category: "DFT".into(), description: "Compute elastic constants C11, C12, C44 for cubic crystal using DFT stress-strain method.".into(), difficulty: "intermediate".into(), estimated_time_s: 600.0, reference_values: b2_ref, tolerance_percent: 8.0, tags: vec!["DFT".into(), "elasticity".into()], required_modules: vec!["dft_task".into()] },
        BenchmarkCase { id: "md-vacancy-diffusion".into(), name: "Vacancy Diffusion in FCC Metal".into(), category: "MD".into(), description: "MD simulation of vacancy diffusion to compute diffusion coefficient and activation energy.".into(), difficulty: "intermediate".into(), estimated_time_s: 300.0, reference_values: b3_ref, tolerance_percent: 15.0, tags: vec!["MD".into(), "diffusion".into(), "defects".into()], required_modules: vec!["molecular_dynamics".into()] },
        BenchmarkCase { id: "pf-solidification".into(), name: "Dendritic Solidification".into(), category: "PhaseField".into(), description: "Phase-field simulation of dendritic growth during solidification with anisotropic interface energy.".into(), difficulty: "advanced".into(), estimated_time_s: 180.0, reference_values: b4_ref, tolerance_percent: 20.0, tags: vec!["PF".into(), "solidification".into(), "dendrite".into()], required_modules: vec!["phase_field".into()] },
        BenchmarkCase { id: "fe-tensile-test".into(), name: "Uniaxial Tensile Test".into(), category: "FE".into(), description: "FE simulation of uniaxial tensile test on a dog-bone specimen with von Mises plasticity.".into(), difficulty: "basic".into(), estimated_time_s: 30.0, reference_values: b5_ref, tolerance_percent: 5.0, tags: vec!["FE".into(), "tension".into(), "plasticity".into()], required_modules: vec!["solver".into()] },
        BenchmarkCase { id: "fe-modal-analysis".into(), name: "Cantilever Beam Modal Analysis".into(), category: "FE".into(), description: "Modal analysis of a cantilever beam to extract natural frequencies and mode shapes.".into(), difficulty: "basic".into(), estimated_time_s: 15.0, reference_values: b6_ref, tolerance_percent: 3.0, tags: vec!["FE".into(), "modal".into(), "vibration".into()], required_modules: vec!["modal".into()] },
        BenchmarkCase { id: "cfd-cylinder-flow".into(), name: "Flow Around Cylinder".into(), category: "CFD".into(), description: "2D CFD simulation of laminar flow around a circular cylinder at Re=100.".into(), difficulty: "intermediate".into(), estimated_time_s: 120.0, reference_values: b7_ref, tolerance_percent: 10.0, tags: vec!["CFD".into(), "external_flow".into()], required_modules: vec!["cfd".into()] },
        BenchmarkCase { id: "fracture-lefm".into(), name: "Linear Elastic Fracture Mechanics".into(), category: "FE".into(), description: "LEFM simulation of mode-I crack propagation using XFEM or cohesive elements.".into(), difficulty: "advanced".into(), estimated_time_s: 60.0, reference_values: b8_ref, tolerance_percent: 10.0, tags: vec!["FE".into(), "fracture".into(), "XFEM".into()], required_modules: vec!["fracture".into()] },
        BenchmarkCase { id: "thermal-conductivity".into(), name: "Thermal Conductivity Prediction".into(), category: "MD".into(), description: "Green-Kubo method for computing thermal conductivity from MD equilibrium trajectory.".into(), difficulty: "advanced".into(), estimated_time_s: 900.0, reference_values: b9_ref, tolerance_percent: 20.0, tags: vec!["MD".into(), "thermal".into(), "Green-Kubo".into()], required_modules: vec!["molecular_dynamics".into()] },
        BenchmarkCase { id: "ml-potential-validation".into(), name: "ML Potential Validation".into(), category: "Bridge".into(), description: "Validate a trained ML potential (NEP/MTP) against DFT test set for energy, forces, and stress.".into(), difficulty: "intermediate".into(), estimated_time_s: 60.0, reference_values: b10_ref, tolerance_percent: 30.0, tags: vec!["ML".into(), "potential".into(), "validation".into()], required_modules: vec!["dft_bridge".into()] },
        BenchmarkCase { id: "homogenization-composite".into(), name: "Composite Homogenization".into(), category: "Bridge".into(), description: "Homogenize a two-phase composite microstructure to effective elastic properties using Mori-Tanaka.".into(), difficulty: "intermediate".into(), estimated_time_s: 10.0, reference_values: b11_ref, tolerance_percent: 5.0, tags: vec!["homogenization".into(), "composite".into()], required_modules: vec!["phase_field_bridge".into()] },
        BenchmarkCase { id: "pf-grain-growth".into(), name: "Polycrystalline Grain Growth".into(), description: "Multi-order-parameter phase-field simulation of normal grain growth in a polycrystal.".into(), category: "PhaseField".into(), difficulty: "advanced".into(), estimated_time_s: 240.0, reference_values: b12_ref, tolerance_percent: 15.0, tags: vec!["PF".into(), "grain_growth".into(), "polycrystal".into()], required_modules: vec!["phase_field".into()] },
    ]
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// List all available benchmark cases, optionally filtered by category or difficulty.
#[command]
pub fn list_benchmarks(
    category: Option<String>,
    difficulty: Option<String>,
) -> Result<Vec<BenchmarkCase>, String> {
    tracing::info!(
        category = category.as_deref().unwrap_or("all"),
        difficulty = difficulty.as_deref().unwrap_or("all"),
        "list_benchmarks called"
    );

    let benchmarks = build_benchmarks();

    let filtered: Vec<BenchmarkCase> = benchmarks
        .into_iter()
        .filter(|b| {
            let cat_match = match &category {
                Some(c) if !c.is_empty() => b.category == *c,
                _ => true,
            };
            let diff_match = match &difficulty {
                Some(d) if !d.is_empty() => b.difficulty == *d,
                _ => true,
            };
            cat_match && diff_match
        })
        .collect();

    tracing::info!("Found {} benchmarks", filtered.len());
    Ok(filtered)
}

/// Get detailed information about a specific benchmark case.
#[command]
pub fn get_benchmark_detail(benchmark_id: String) -> Result<BenchmarkDetail, String> {
    tracing::info!(benchmark = %benchmark_id, "get_benchmark_detail called");

    let benchmarks = build_benchmarks();
    let benchmark = benchmarks
        .into_iter()
        .find(|b| b.id == benchmark_id)
        .ok_or_else(|| format!("Benchmark '{}' not found", benchmark_id))?;

    let detail = BenchmarkDetail {
        setup_instructions: vec![
            format!("Prepare input files for {}", benchmark.name),
            "Set up simulation parameters according to reference configuration".into(),
            "Ensure all required modules are available".into(),
            "Run the simulation with recommended settings".into(),
        ],
        expected_outputs: benchmark
            .reference_values
            .keys()
            .map(|k| format!("{}: within {:.1}% of reference", k, benchmark.tolerance_percent))
            .collect(),
        validation_criteria: vec![
            format!("All computed values within {:.1}% of reference", benchmark.tolerance_percent),
            "Simulation converges without numerical instability".into(),
            "Energy conservation within acceptable bounds".into(),
        ],
        known_issues: vec![
            "Results may vary with different random seeds for stochastic simulations".into(),
            "Parallel execution may produce slightly different results due to floating-point ordering".into(),
        ],
        references: vec![
            "Internal validation database V2.1".into(),
            "Published literature values for reference materials".into(),
        ],
        benchmark,
    };

    Ok(detail)
}

/// Run a specific benchmark case and return results.
#[command]
pub fn run_benchmark(benchmark_id: String) -> Result<BenchmarkRunResult, String> {
    tracing::info!(benchmark = %benchmark_id, "run_benchmark called");

    let benchmarks = build_benchmarks();
    let benchmark = benchmarks
        .iter()
        .find(|b| b.id == benchmark_id)
        .ok_or_else(|| format!("Benchmark '{}' not found", benchmark_id))?;

    // Generate mock computed values with small random perturbation from reference
    let mut computed = HashMap::new();
    let mut errors = HashMap::new();
    let mut max_error = 0.0_f64;

    for (key, &ref_val) in &benchmark.reference_values {
        // Add 0-3% random perturbation
        let perturbation = 1.0 + (key.len() as f64 % 3.0 - 1.0) * 0.01;
        let comp_val = ref_val * perturbation;
        let abs_err = (comp_val - ref_val).abs();
        let rel_err = if ref_val.abs() > 1e-15 {
            abs_err / ref_val.abs() * 100.0
        } else {
            abs_err * 100.0
        };
        computed.insert(key.clone(), comp_val);
        errors.insert(key.clone(), rel_err);
        max_error = max_error.max(rel_err);
    }

    let passed = max_error <= benchmark.tolerance_percent;
    let status = if passed { "passed" } else { "failed" };

    tracing::info!(
        status = status,
        max_error = max_error,
        tolerance = benchmark.tolerance_percent,
        "Benchmark completed"
    );

    Ok(BenchmarkRunResult {
        benchmark_id: benchmark.id.clone(),
        status: status.to_string(),
        computed_values: computed,
        reference_values: benchmark.reference_values.clone(),
        errors,
        max_error_percent: max_error,
        passed,
        execution_time_ms: benchmark.estimated_time_s * 1000.0 * 0.85,
        timestamp: chrono::Utc::now().to_rfc3339(),
        message: if passed {
            format!("Benchmark '{}' passed with max error {:.2}% (tolerance {:.1}%)", benchmark.id, max_error, benchmark.tolerance_percent)
        } else {
            format!("Benchmark '{}' failed: max error {:.2}% exceeds tolerance {:.1}%", benchmark.id, max_error, benchmark.tolerance_percent)
        },
    })
}

/// Compare computed benchmark results with reference values.
#[command]
pub fn compare_with_reference(
    benchmark_id: String,
    computed_values: HashMap<String, f64>,
) -> Result<BenchmarkComparison, String> {
    tracing::info!(benchmark = %benchmark_id, "compare_with_reference called");

    let benchmarks = build_benchmarks();
    let benchmark = benchmarks
        .iter()
        .find(|b| b.id == benchmark_id)
        .ok_or_else(|| format!("Benchmark '{}' not found", benchmark_id))?;

    let mut comparisons = Vec::new();
    let mut all_passed = true;

    for (key, &ref_val) in &benchmark.reference_values {
        let comp_val = computed_values.get(key).copied().unwrap_or(0.0);
        let abs_err = (comp_val - ref_val).abs();
        let rel_err = if ref_val.abs() > 1e-15 {
            abs_err / ref_val.abs() * 100.0
        } else {
            abs_err * 100.0
        };
        let passed = rel_err <= benchmark.tolerance_percent;

        if !passed {
            all_passed = false;
        }

        comparisons.push(ValueComparison {
            property: key.clone(),
            computed: comp_val,
            reference: ref_val,
            absolute_error: abs_err,
            relative_error_percent: rel_err,
            tolerance_percent: benchmark.tolerance_percent,
            passed,
        });
    }

    let passed_count = comparisons.iter().filter(|c| c.passed).count();
    let summary = format!(
        "{}/{} properties within tolerance ({:.1}%): {}",
        passed_count,
        comparisons.len(),
        benchmark.tolerance_percent,
        if all_passed { "PASS" } else { "FAIL" }
    );

    tracing::info!(summary = %summary, "Comparison complete");
    Ok(BenchmarkComparison {
        benchmark_id: benchmark.id.clone(),
        benchmark_name: benchmark.name.clone(),
        comparisons,
        overall_passed: all_passed,
        summary,
    })
}

/// Get aggregated benchmark statistics across all categories and difficulties.
#[command]
pub fn get_benchmark_statistics() -> Result<BenchmarkStatistics, String> {
    tracing::info!("get_benchmark_statistics called");

    let benchmarks = build_benchmarks();

    let total = benchmarks.len() as u32;
    let mut passed = 0u32;
    let mut failed = 0u32;
    let skipped = 0u32;
    let mut total_time = 0.0_f64;

    let mut cat_stats: HashMap<String, CategoryStats> = HashMap::new();
    let mut diff_stats: HashMap<String, DifficultyStats> = HashMap::new();

    for b in &benchmarks {
        // Mock: most benchmarks pass
        let mock_passed = b.id != "pf-solidification"; // one fails for realism
        if mock_passed {
            passed += 1;
        } else {
            failed += 1;
        }
        total_time += b.estimated_time_s * 1000.0 * 0.85;

        // Category stats
        let entry = cat_stats.entry(b.category.clone()).or_insert(CategoryStats {
            total: 0, passed: 0, failed: 0, pass_rate: 0.0,
        });
        entry.total += 1;
        if mock_passed { entry.passed += 1; } else { entry.failed += 1; }

        // Difficulty stats
        let entry = diff_stats.entry(b.difficulty.clone()).or_insert(DifficultyStats {
            total: 0, passed: 0, failed: 0, pass_rate: 0.0,
        });
        entry.total += 1;
        if mock_passed { entry.passed += 1; } else { entry.failed += 1; }
    }

    // Compute pass rates
    for stats in cat_stats.values_mut() {
        stats.pass_rate = if stats.total > 0 {
            stats.passed as f64 / stats.total as f64
        } else {
            0.0
        };
    }
    for stats in diff_stats.values_mut() {
        stats.pass_rate = if stats.total > 0 {
            stats.passed as f64 / stats.total as f64
        } else {
            0.0
        };
    }

    let pass_rate = if total > 0 { passed as f64 / total as f64 } else { 0.0 };
    let avg_time = if total > 0 { total_time / total as f64 } else { 0.0 };

    tracing::info!(
        total = total,
        passed = passed,
        failed = failed,
        pass_rate = pass_rate,
        "Statistics generated"
    );

    Ok(BenchmarkStatistics {
        total_benchmarks: total,
        passed,
        failed,
        skipped,
        pass_rate,
        average_execution_time_ms: avg_time,
        category_breakdown: cat_stats,
        difficulty_breakdown: diff_stats,
        last_updated: chrono::Utc::now().to_rfc3339(),
    })
}
