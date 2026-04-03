//! Adaptive Precision Commands (V2.0 - 多尺度工作流编排器)
//!
//! 提供自适应精度控制能力：
//! - 评估精度
//! - 应用调整
//! - 运行自适应循环
//! - 获取精度历史
//! - 更新控制配置
//! - 获取推荐策略
//!
//! Mock: 3 iterations, converged at iteration 3.

use serde::{Deserialize, Serialize};
use tauri::command;

// ============================================================================
// Data Structures
// ============================================================================

/// 精度评估结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecisionEvaluation {
    pub id: String,
    pub scale: String,
    pub parameter: String,
    pub current_value: f64,
    pub reference_value: f64,
    pub absolute_error: f64,
    pub relative_error: f64,
    pub tolerance: f64,
    pub is_converged: bool,
    pub recommendation: String,
    pub evaluated_at: String,
}

/// 精度调整
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecisionAdjustment {
    pub id: String,
    pub scale: String,
    pub parameter: String,
    pub adjustment_type: String,
    pub old_value: serde_json::Value,
    pub new_value: serde_json::Value,
    pub expected_improvement: f64,
    pub applied_at: String,
}

/// 自适应循环迭代
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveIteration {
    pub iteration: usize,
    pub evaluations: Vec<PrecisionEvaluation>,
    pub adjustments: Vec<PrecisionAdjustment>,
    pub max_relative_error: f64,
    pub is_converged: bool,
    pub timestamp: String,
}

/// 自适应循环结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveLoopResult {
    pub id: String,
    pub total_iterations: usize,
    pub converged: bool,
    pub converged_at_iteration: Option<usize>,
    pub iterations: Vec<AdaptiveIteration>,
    pub final_max_error: f64,
    pub total_adjustments: usize,
    pub started_at: String,
    pub completed_at: String,
}

/// 精度历史条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecisionHistoryEntry {
    pub id: String,
    pub loop_id: String,
    pub iteration: usize,
    pub scale: String,
    pub parameter: String,
    pub relative_error: f64,
    pub is_converged: bool,
    pub timestamp: String,
}

/// 控制配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlConfig {
    pub id: String,
    pub max_iterations: usize,
    pub convergence_tolerance: f64,
    pub relaxation_factor: f64,
    pub adaptive_strategy: String,
    pub scales: Vec<String>,
    pub enabled: bool,
    pub updated_at: String,
}

/// 推荐策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedStrategy {
    pub strategy: String,
    pub description: String,
    pub expected_iterations: usize,
    pub expected_precision_gain: f64,
    pub computational_cost_factor: f64,
    pub suitable_scales: Vec<String>,
}

// ============================================================================
// Mock Data
// ============================================================================

fn mock_adaptive_loop() -> AdaptiveLoopResult {
    AdaptiveLoopResult {
        id: uuid::Uuid::new_v4().to_string(),
        total_iterations: 3,
        converged: true,
        converged_at_iteration: Some(3),
        iterations: vec![
            AdaptiveIteration {
                iteration: 1,
                evaluations: vec![
                    PrecisionEvaluation {
                        id: uuid::Uuid::new_v4().to_string(),
                        scale: "DFT".to_string(),
                        parameter: "kpoints_density".to_string(),
                        current_value: 4.0,
                        reference_value: 8.0,
                        absolute_error: 4.0,
                        relative_error: 0.50,
                        tolerance: 0.05,
                        is_converged: false,
                        recommendation: "增加k点密度至6x6x6".to_string(),
                        evaluated_at: "2026-04-03T10:00:00Z".to_string(),
                    },
                    PrecisionEvaluation {
                        id: uuid::Uuid::new_v4().to_string(),
                        scale: "MD".to_string(),
                        parameter: "timestep".to_string(),
                        current_value: 2.0,
                        reference_value: 0.5,
                        absolute_error: 1.5,
                        relative_error: 0.75,
                        tolerance: 0.05,
                        is_converged: false,
                        recommendation: "减小时间步长至1.0 fs".to_string(),
                        evaluated_at: "2026-04-03T10:00:00Z".to_string(),
                    },
                ],
                adjustments: vec![
                    PrecisionAdjustment {
                        id: uuid::Uuid::new_v4().to_string(),
                        scale: "DFT".to_string(),
                        parameter: "kpoints_density".to_string(),
                        adjustment_type: "increase".to_string(),
                        old_value: serde_json::json!([4, 4, 4]),
                        new_value: serde_json::json!([6, 6, 6]),
                        expected_improvement: 0.30,
                        applied_at: "2026-04-03T10:05:00Z".to_string(),
                    },
                    PrecisionAdjustment {
                        id: uuid::Uuid::new_v4().to_string(),
                        scale: "MD".to_string(),
                        parameter: "timestep".to_string(),
                        adjustment_type: "decrease".to_string(),
                        old_value: serde_json::json!(2.0),
                        new_value: serde_json::json!(1.0),
                        expected_improvement: 0.40,
                        applied_at: "2026-04-03T10:05:00Z".to_string(),
                    },
                ],
                max_relative_error: 0.75,
                is_converged: false,
                timestamp: "2026-04-03T10:05:00Z".to_string(),
            },
            AdaptiveIteration {
                iteration: 2,
                evaluations: vec![
                    PrecisionEvaluation {
                        id: uuid::Uuid::new_v4().to_string(),
                        scale: "DFT".to_string(),
                        parameter: "kpoints_density".to_string(),
                        current_value: 6.0,
                        reference_value: 8.0,
                        absolute_error: 2.0,
                        relative_error: 0.25,
                        tolerance: 0.05,
                        is_converged: false,
                        recommendation: "增加k点密度至8x8x8".to_string(),
                        evaluated_at: "2026-04-03T10:30:00Z".to_string(),
                    },
                    PrecisionEvaluation {
                        id: uuid::Uuid::new_v4().to_string(),
                        scale: "MD".to_string(),
                        parameter: "timestep".to_string(),
                        current_value: 1.0,
                        reference_value: 0.5,
                        absolute_error: 0.5,
                        relative_error: 0.15,
                        tolerance: 0.05,
                        is_converged: false,
                        recommendation: "减小时间步长至0.5 fs".to_string(),
                        evaluated_at: "2026-04-03T10:30:00Z".to_string(),
                    },
                ],
                adjustments: vec![
                    PrecisionAdjustment {
                        id: uuid::Uuid::new_v4().to_string(),
                        scale: "DFT".to_string(),
                        parameter: "kpoints_density".to_string(),
                        adjustment_type: "increase".to_string(),
                        old_value: serde_json::json!([6, 6, 6]),
                        new_value: serde_json::json!([8, 8, 8]),
                        expected_improvement: 0.15,
                        applied_at: "2026-04-03T10:35:00Z".to_string(),
                    },
                    PrecisionAdjustment {
                        id: uuid::Uuid::new_v4().to_string(),
                        scale: "MD".to_string(),
                        parameter: "timestep".to_string(),
                        adjustment_type: "decrease".to_string(),
                        old_value: serde_json::json!(1.0),
                        new_value: serde_json::json!(0.5),
                        expected_improvement: 0.10,
                        applied_at: "2026-04-03T10:35:00Z".to_string(),
                    },
                ],
                max_relative_error: 0.25,
                is_converged: false,
                timestamp: "2026-04-03T10:35:00Z".to_string(),
            },
            AdaptiveIteration {
                iteration: 3,
                evaluations: vec![
                    PrecisionEvaluation {
                        id: uuid::Uuid::new_v4().to_string(),
                        scale: "DFT".to_string(),
                        parameter: "kpoints_density".to_string(),
                        current_value: 8.0,
                        reference_value: 8.0,
                        absolute_error: 0.0,
                        relative_error: 0.001,
                        tolerance: 0.05,
                        is_converged: true,
                        recommendation: "DFT精度已收敛".to_string(),
                        evaluated_at: "2026-04-03T11:00:00Z".to_string(),
                    },
                    PrecisionEvaluation {
                        id: uuid::Uuid::new_v4().to_string(),
                        scale: "MD".to_string(),
                        parameter: "timestep".to_string(),
                        current_value: 0.5,
                        reference_value: 0.5,
                        absolute_error: 0.0,
                        relative_error: 0.002,
                        tolerance: 0.05,
                        is_converged: true,
                        recommendation: "MD精度已收敛".to_string(),
                        evaluated_at: "2026-04-03T11:00:00Z".to_string(),
                    },
                ],
                adjustments: vec![],
                max_relative_error: 0.002,
                is_converged: true,
                timestamp: "2026-04-03T11:00:00Z".to_string(),
            },
        ],
        final_max_error: 0.002,
        total_adjustments: 4,
        started_at: "2026-04-03T10:00:00Z".to_string(),
        completed_at: "2026-04-03T11:00:00Z".to_string(),
    }
}

// ============================================================================
// Commands
// ============================================================================

/// 评估精度
#[command]
pub fn evaluate_precision(
    scale: String,
    parameters: Vec<String>,
) -> Result<Vec<PrecisionEvaluation>, String> {
    tracing::info!(
        "Evaluating precision for scale {} ({} parameters)",
        scale,
        parameters.len()
    );
    let evaluations: Vec<PrecisionEvaluation> = parameters
        .iter()
        .map(|param| PrecisionEvaluation {
            id: uuid::Uuid::new_v4().to_string(),
            scale: scale.clone(),
            parameter: param.clone(),
            current_value: 4.0_f64,
            reference_value: 8.0_f64,
            absolute_error: 4.0_f64,
            relative_error: 0.50_f64,
            tolerance: 0.05_f64,
            is_converged: false,
            recommendation: format!("建议调整 {} 参数以提高精度", param),
            evaluated_at: chrono::Utc::now().to_rfc3339(),
        })
        .collect();
    tracing::info!("Evaluated {} parameters", evaluations.len());
    Ok(evaluations)
}

/// 应用调整
#[command]
pub fn apply_adjustments(
    adjustments: Vec<PrecisionAdjustment>,
) -> Result<Vec<PrecisionAdjustment>, String> {
    tracing::info!("Applying {} precision adjustments", adjustments.len());
    let applied: Vec<PrecisionAdjustment> = adjustments
        .into_iter()
        .map(|mut adj| {
            adj.id = uuid::Uuid::new_v4().to_string();
            adj.applied_at = chrono::Utc::now().to_rfc3339();
            adj
        })
        .collect();
    tracing::info!("Applied {} adjustments", applied.len());
    Ok(applied)
}

/// 运行自适应循环
#[command]
pub fn run_adaptive_loop(
    scales: Vec<String>,
    max_iterations: Option<usize>,
    tolerance: Option<f64>,
) -> Result<AdaptiveLoopResult, String> {
    tracing::info!(
        "Running adaptive loop for scales {:?} (max_iter={:?}, tol={:?})",
        scales, max_iterations, tolerance
    );
    let result = mock_adaptive_loop();
    tracing::info!(
        "Adaptive loop completed: converged={} at iteration {:?}",
        result.converged,
        result.converged_at_iteration
    );
    Ok(result)
}

/// 获取精度历史
#[command]
pub fn get_precision_history(
    loop_id: String,
    limit: Option<usize>,
) -> Result<Vec<PrecisionHistoryEntry>, String> {
    tracing::info!(
        "Getting precision history for loop {} (limit={:?})",
        loop_id, limit
    );
    let limit = limit.unwrap_or(50);
    let history = vec![
        PrecisionHistoryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            loop_id: loop_id.clone(),
            iteration: 1,
            scale: "DFT".to_string(),
            parameter: "kpoints_density".to_string(),
            relative_error: 0.50,
            is_converged: false,
            timestamp: "2026-04-03T10:00:00Z".to_string(),
        },
        PrecisionHistoryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            loop_id: loop_id.clone(),
            iteration: 1,
            scale: "MD".to_string(),
            parameter: "timestep".to_string(),
            relative_error: 0.75,
            is_converged: false,
            timestamp: "2026-04-03T10:00:00Z".to_string(),
        },
        PrecisionHistoryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            loop_id: loop_id.clone(),
            iteration: 2,
            scale: "DFT".to_string(),
            parameter: "kpoints_density".to_string(),
            relative_error: 0.25,
            is_converged: false,
            timestamp: "2026-04-03T10:30:00Z".to_string(),
        },
        PrecisionHistoryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            loop_id: loop_id.clone(),
            iteration: 2,
            scale: "MD".to_string(),
            parameter: "timestep".to_string(),
            relative_error: 0.15,
            is_converged: false,
            timestamp: "2026-04-03T10:30:00Z".to_string(),
        },
        PrecisionHistoryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            loop_id: loop_id.clone(),
            iteration: 3,
            scale: "DFT".to_string(),
            parameter: "kpoints_density".to_string(),
            relative_error: 0.001,
            is_converged: true,
            timestamp: "2026-04-03T11:00:00Z".to_string(),
        },
        PrecisionHistoryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            loop_id,
            iteration: 3,
            scale: "MD".to_string(),
            parameter: "timestep".to_string(),
            relative_error: 0.002,
            is_converged: true,
            timestamp: "2026-04-03T11:00:00Z".to_string(),
        },
    ];
    tracing::info!("Returned {} history entries", history.len().min(limit));
    Ok(history.into_iter().take(limit).collect())
}

/// 更新控制配置
#[command]
pub fn update_control_config(config: ControlConfig) -> Result<ControlConfig, String> {
    tracing::info!("Updating control config: {}", config.id);
    let mut updated = config.clone();
    updated.updated_at = chrono::Utc::now().to_rfc3339();
    tracing::info!("Updated control config: {}", updated.id);
    Ok(updated)
}

/// 获取推荐策略
#[command]
pub fn get_recommended_strategy(
    scales: Vec<String>,
    target_precision: Option<f64>,
) -> Result<RecommendedStrategy, String> {
    tracing::info!(
        "Getting recommended strategy for scales {:?} (target_precision={:?})",
        scales, target_precision
    );
    let strategy = RecommendedStrategy {
        strategy: "gradual_refinement".to_string(),
        description: "渐进式精化策略：从粗网格开始，逐步细化各尺度参数，每步评估收敛性，直到满足精度要求。适用于多尺度耦合场景，可在计算成本和精度之间取得良好平衡。".to_string(),
        expected_iterations: 3,
        expected_precision_gain: 0.95,
        computational_cost_factor: 2.5,
        suitable_scales: scales.clone(),
    };
    tracing::info!(
        "Recommended strategy: {} (expected {} iterations)",
        strategy.strategy, strategy.expected_iterations
    );
    Ok(strategy)
}
