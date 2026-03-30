//! Fatigue analysis commands
//! Handles high-cycle fatigue (S-N), low-cycle fatigue (E-N), and random vibration fatigue

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Analysis type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FatigueAnalysisType {
    S-N,
    E-N,
    PSD,
}

/// Mean stress correction methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MeanStressCorrection {
    None,
    Goodman,
    Gerber,
    Soderberg,
}

/// Surface treatment types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SurfaceTreatment {
    Machined,
    AsCast,
    Polished,
    ShotPeened,
}

/// S-N Curve data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SNDataPoint {
    pub stress: f64,    // Stress amplitude (MPa)
    pub cycles: f64,      // Number of cycles to failure
}

/// S-N Curve parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SNParams {
    pub stress_ratio: f64,
    pub data_points: Vec<SNDataPoint>,
    pub fatigue_limit: f64,
    pub mean_stress_correction: String,
}

/// E-N Curve parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ENParams {
    pub strain_amplitude: f64,
    pub cyclic_stress_coeff: f64,
    pub cyclic_exponent: f64,
    pub use_neuber: bool,
}

/// PSD parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PSDParams {
    pub psd_data: String,
    pub rms_stress: f64,
    pub target_life: f64,
}

/// Load parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadParams {
    pub stress_amplitude: f64,
}

/// Combined fatigue parameters from frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FatigueParams {
    pub analysis_type: String,
    pub sn_params: Option<SNParams>,
    pub en_params: Option<ENParams>,
    pub psd_params: Option<PSDParams>,
    pub load_type: String,
    pub load: LoadParams,
    pub kt: f64,
    pub surface_treatment: String,
}

/// Fatigue analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FatigueResults {
    pub damage: f64,
    pub life_cycles: f64,
    pub safety_factor: f64,
}

/// S-N Curve model
#[derive(Debug, Clone)]
pub struct SNCurve {
    pub data_points: Vec<SNDataPoint>,
    pub fatigue_limit: f64,
    pub log_a: f64,    // Intercept in log-log space
    pub m: f64,        // Slope in log-log space
}

impl SNCurve {
    /// Create new S-N curve from data points using least squares fitting
    pub fn new(data_points: Vec<SNDataPoint>, fatigue_limit: f64) -> Self {
        // Fit S-N curve: log(N) = log(a) - m * log(S)
        // Or: N = a * S^(-m)
        let mut m = 3.0;  // Default slope
        let mut log_a = 12.0;  // Default intercept

        if data_points.len() >= 2 {
            // Simple linear regression in log-log space
            let n = data_points.len() as f64;
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            let mut sum_xx = 0.0;
            let mut sum_xy = 0.0;

            for point in &data_points {
                let x = point.stress.ln();
                let y = point.cycles.ln();
                sum_x += x;
                sum_y += y;
                sum_xx += x * x;
                sum_xy += x * y;
            }

            let denom = n * sum_xx - sum_x * sum_x;
            if denom.abs() > 1e-10 {
                m = (n * sum_xy - sum_x * sum_y) / denom;
                log_a = (sum_y - m * sum_x) / n;
            }
        }

        SNCurve {
            data_points,
            fatigue_limit,
            log_a,
            m,
        }
    }

    /// Calculate cycles to failure for given stress amplitude
    pub fn get_cycles(&self, stress: f64) -> f64 {
        if stress <= self.fatigue_limit {
            return f64::INFINITY;  // Infinite life
        }
        
        let log_n = self.log_a - self.m * stress.ln();
        (-log_n).exp()
    }

    /// Calculate allowable stress for given cycles
    pub fn get_stress(&self, cycles: f64) -> f64 {
        if cycles <= 0.0 {
            return 0.0;
        }
        
        let log_n = cycles.ln();
        let stress = ((self.log_a - log_n) / self.m).exp();
        stress.max(self.fatigue_limit)
    }
}

/// Rainflow cycle extracted from load spectrum
#[derive(Debug, Clone)]
pub struct RainflowCycle {
    pub amplitude: f64,
    pub mean: f64,
    pub count: i32,
}

/// Rainflow counting algorithm (ASTM E1049)
pub fn rainflow_count(loads: &[f64]) -> Vec<RainflowCycle> {
    let n = loads.len();
    if n < 4 {
        return vec![];
    }

    // Simplified rainflow counting
    let mut cycles: Vec<RainflowCycle> = vec![];
    let mut residuals: Vec<f64> = vec![];

    // Half cycles extraction (simplified)
    for i in 0..(n - 1) {
        let range = (loads[i + 1] - loads[i]).abs();
        let mean = (loads[i] + loads[i + 1]) / 2.0;
        
        // Store half cycles
        residuals.push(range);
        residuals.push(mean);
    }

    // Combine half cycles into full cycles
    let mut i = 0;
    while i + 1 < residuals.len() {
        let amp = (residuals[i] + residuals[i + 1]) / 2.0;
        let mean = (residuals[i] - residuals[i + 1]) / 2.0;
        
        if amp > 0.0 {
            cycles.push(RainflowCycle {
                amplitude: amp,
                mean: mean,
                count: 1,
            });
        }
        i += 2;
    }

    cycles
}

/// Apply mean stress correction
pub fn mean_stress_correction(
    stress_amplitude: f64,
    mean_stress: f64,
    correction_type: &str,
    ultimate_tensile: f64,  // Su (MPa)
    yield_strength: f64,   // Sy (MPa)
) -> f64 {
    let r = stress_amplitude / (stress_amplitude + mean_stress.abs());
    
    match correction_type {
        "goodman" => {
            // Goodman: Sa = Su / (Su / Sa + Sm)
            stress_amplitude * ultimate_tensile / (ultimate_tensile + mean_stress.abs())
        },
        "gerber" => {
            // Gerber: Sa = Su * (1 - (Sm/Su)^2)
            stress_amplitude * (1.0 - (mean_stress / ultimate_tensile).powi(2))
        },
        "soderberg" => {
            // Soderberg: Sa = Sy / (Sy / Sa + Sm)
            stress_amplitude * yield_strength / (yield_strength + mean_stress.abs())
        },
        _ => stress_amplitude,  // No correction
    }
}

/// Calculate surface modification factor
pub fn surface_factor(treatment: &str) -> f64 {
    match treatment {
        "polished" => 1.0,
        "machined" => 0.9,
        "as_cast" => 0.5,
        "shot_peened" => 1.2,  // Compressive residual stress improves fatigue
        _ => 1.0,
    }
}

/// Miner linear damage accumulation
pub fn minER_rule(cycles: &[f64], sn_curve: &SNCurve) -> f64 {
    let mut damage = 0.0;
    
    for &n in cycles {
        let allowable = sn_curve.get_cycles(n);
        if allowable.is_finite() && allowable > 0.0 {
            damage += n / allowable;
        }
    }
    
    damage
}

/// Main fatigue analysis function
pub fn analyze_fatigue(params: &FatigueParams) -> Result<FatigueResults, String> {
    let stress_amplitude = params.load.stress_amplitude;
    let kt = params.kt;
    let effective_stress = stress_amplitude * kt;
    
    // Surface treatment factor
    let surface_fact = surface_factor(&params.surface_treatment);
    
    // Adjusted stress for surface treatment
    let adjusted_stress = effective_stress / surface_fact;
    
    let (damage, life_cycles, safety_factor) = match params.analysis_type.as_str() {
        "sn" => {
            // S-N Curve Analysis (High Cycle Fatigue)
            let sn_params = params.sn_params.as_ref()
                .ok_or("S-N parameters required")?;
            
            let sn_curve = SNCurve::new(
                sn_params.data_points.clone(),
                sn_params.fatigue_limit,
            );
            
            // Calculate life
            let life = sn_curve.get_cycles(adjusted_stress);
            
            // Calculate damage (using Miner's rule)
            let damage = if life.is_finite() {
                stress_amplitude / life
            } else {
                0.0
            };
            
            // Safety factor based on fatigue limit
            let sf = sn_params.fatigue_limit / adjusted_stress;
            
            (damage, life, sf)
        },
        "en" => {
            // E-N Curve Analysis (Low Cycle Fatigue)
            let en_params = params.en_params.as_ref()
                .ok_or("E-N parameters required")?;
            
            // Manson-Coffin relationship (simplified)
            // Delta_epsilon/2 = (sigma_f'/2E) * (2N)^b + (epsilon_f') * (2N)^c
            // Simplified: Strain-life relationship
            let cyclic_stress_coeff = en_params.cyclic_stress_coeff;
            let cyclic_exponent = en_params.cyclic_exponent;
            
            // Calculate plastic strain amplitude
            let elastic_strain = adjusted_stress / (2.0 * 1e5);  // E = 200 GPa
            let plastic_strain = (cyclic_stress_coeff / adjusted_stress).powf(1.0 / cyclic_exponent);
            
            let total_strain = (elastic_strain.powi(2) + plastic_strain.powi(2)).sqrt();
            
            // Estimate cycles to failure
            let life = if total_strain > 0.0 {
                ((cyclic_stress_coeff / total_strain).powf(1.0 / cyclic_exponent) / 2.0).floor()
            } else {
                f64::INFINITY
            };
            
            let damage = if life.is_finite() && life > 0.0 {
                stress_amplitude / life
            } else {
                0.0
            };
            
            let sf = if en_params.strain_amplitude > 0.0 {
                en_params.strain_amplitude / total_strain
            } else {
                1.0
            };
            
            (damage, life, sf)
        },
        "psd" => {
            // PSD Random Vibration Fatigue
            let psd_params = params.psd_params.as_ref()
                .ok_or("PSD parameters required")?;
            
            let rms = psd_params.rms_stress;
            let target_life = psd_params.target_life;
            
            // Simplified: use RMS stress to estimate fatigue damage
            // D = n / N where n is actual cycles, N is allowable
            let effective_rms = adjusted_stress * rms / 100.0;
            
            // Estimate cycles per hour (simplified)
            let cycles_per_hour = 3600.0 * 10.0;  // Assume 10 Hz bandwidth
            let total_cycles = cycles_per_hour * target_life;
            
            // Damage using Basquin's law for random loading
            let m = 5.0;  // Steel typical slope
            let damage = total_cycles * effective_rms.powf(m) / 1e9;
            let life = if damage > 0.0 && damage < 1.0 {
                target_life / damage
            } else {
                target_life
            };
            
            let sf = if damage > 0.0 {
                1.0 / damage
            } else {
                f64::INFINITY
            };
            
            (damage, life, sf)
        },
        _ => return Err("Unknown analysis type".to_string()),
    };
    
    Ok(FatigueResults {
        damage,
        life_cycles,
        safety_factor: safety_factor.min(10.0),  // Cap at 10
    })
}

/// Tauri command wrapper
#[tauri::command]
pub fn fatigue_analysis(params: FatigueParams) -> Result<FatigueResults, String> {
    analyze_fatigue(&params)
}

/// Calculate rainflow cycles from load spectrum
#[tauri::command]
pub fn rainflow_analysis(loads: Vec<f64>) -> Vec<RainflowCycle> {
    rainflow_count(&loads)
}

/// Fit S-N curve from experimental data
#[tauri::command]
pub fn fit_sn_curve(data_points: Vec<SNDataPoint>, fatigue_limit: f64) -> HashMap<String, f64> {
    let sn_curve = SNCurve::new(data_points, fatigue_limit);
    
    let mut result = HashMap::new();
    result.insert("log_a".to_string(), sn_curve.log_a);
    result.insert("m".to_string(), sn_curve.m);
    result.insert("fatigue_limit".to_string(), sn_curve.fatigue_limit);
    result
}

/// Calculate damage at each node from stress results
#[tauri::command]
pub fn calculate_node_damage(
    stress_results: Vec<f64>,
    sn_curve_params: SNParams,
) -> Vec<f64> {
    let sn_curve = SNCurve::new(
        sn_curve_params.data_points,
        sn_curve_params.fatigue_limit,
    );
    
    stress_results
        .iter()
        .map(|&s| {
            let cycles = sn_curve.get_cycles(s);
            if cycles.is_finite() && cycles > 0.0 {
                1.0 / cycles
            } else {
                0.0
            }
        })
        .collect()
}