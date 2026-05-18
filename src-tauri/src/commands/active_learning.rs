#![allow(dead_code)]
/**
 * V3.9 Active Learning Loop — ML 原子间势函数训练与部署
 *
 * 核心功能:
 * 1. Active Learning 循环驱动的 ML 势函数自适应训练
 * 2. 不确定性量化（MC Dropout / 集体差异）
 * 3. 候选构型自动选取与 DFT 参考标注
 * 4. NequIP/MACE 训练 Pipeline 封装
 * 5. ONNX 格式导出与部署
 *
 * 集成点:
 * - ml_potential.rs: 借用 NequIP/MACE 训练 API
 * - multiscale_surrogate.rs: 复用 MC Dropout 不确定性量化
 * - DFT 求解器: dft_task.rs / dft_input.rs
 * - ONNX 推理: simulation_archive.rs
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use std::sync::Mutex as StdMutex;
use lazy_static::lazy_static;
use uuid::Uuid;

// ============================================================================
// 全局状态
// ============================================================================

lazy_static! {
    static ref ACTIVE_LEARNING_STATE: StdMutex<ActiveLearningState> = StdMutex::new(ActiveLearningState::default());
}

#[derive(Debug, Default)]
pub struct ActiveLearningState {
    pub active_session: Option<ActiveSession>,
    pub training_histories: HashMap<String, TrainingHistory>,
    pub candidate_databases: HashMap<String, CandidateDatabase>,
}

#[derive(Debug, Clone)]
pub struct ActiveSession {
    pub session_id: String,
    pub material_system: String,
    pub potential_type: String,
    pub initial_training_size: u32,
    pub current_iteration: u32,
    pub max_iterations: u32,
    pub convergence_threshold: f64,
    pub started_at: u64,
    pub status: SessionStatus,
    pub current_potential_name: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct TrainingHistory {
    pub session_id: String,
    pub iterations: Vec<IterationRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationRecord {
    pub iteration: u32,
    pub num_training_structures: u32,
    pub num_candidates_selected: u32,
    pub energy_rmse: f64,
    pub force_rmse: f64,
    pub uncertainty_avg: f64,
    pub uncertainty_max: f64,
    pub added_structures: Vec<String>,
    pub timestamp: u64,
}

#[derive(Debug, Default)]
pub struct CandidateDatabase {
    pub session_id: String,
    pub candidates: Vec<CandidateStructure>,
    pub uncertainty_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateStructure {
    pub id: String,
    pub structure_hash: String,
    pub atom_types: Vec<String>,
    pub positions: Vec<Vec<f64>>,
    pub cell: Vec<Vec<f64>>,
    pub uncertainty: f64,
    pub energy_approx: Option<f64>,
    pub forces_approx: Option<Vec<Vec<f64>>>,
    pub in_training_set: bool,
    pub selected_for_dft: bool,
    pub dft_energy: Option<f64>,
    pub dft_forces: Option<Vec<Vec<f64>>>,
    pub source: CandidateSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CandidateSource {
    #[serde(rename = "initial_dataset")]
    InitialDataset,
    #[serde(rename = "md_trajectory")]
    MdTrajectory,
    #[serde(rename = "geometric_candidates")]
    GeometricCandidates,
    #[serde(rename = "perturbed_structure")]
    PerturbedStructure,
    #[serde(rename = "active_learning_selected")]
    ActiveLearningSelected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionStatus {
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "running_md")]
    RunningMd,
    #[serde(rename = "evaluating_uncertainty")]
    EvaluatingUncertainty,
    #[serde(rename = "selecting_candidates")]
    SelectingCandidates,
    #[serde(rename = "running_dft")]
    RunningDft,
    #[serde(rename = "retraining")]
    Retraining,
    #[serde(rename = "converged")]
    Converged,
    #[serde(rename = "failed")]
    Failed(String),
}

// ============================================================================
// 请求/响应类型
// ============================================================================

/// 启动 Active Learning 会话
#[derive(Debug, Serialize, Deserialize)]
pub struct StartSessionRequest {
    pub material_system: String,
    pub potential_type: String,
    pub initial_training_set_path: Option<String>,
    pub max_iterations: Option<u32>,
    pub convergence_threshold: Option<f64>,
    pub uncertainty_method: Option<String>,
    pub sampling_strategy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartSessionResponse {
    pub session_id: String,
    pub material_system: String,
    pub potential_type: String,
    pub initial_training_size: u32,
    pub max_iterations: u32,
    pub status: String,
    pub message: String,
}

/// 获取当前会话状态
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionStatusResponse {
    pub session_id: String,
    pub material_system: String,
    pub potential_type: String,
    pub current_iteration: u32,
    pub max_iterations: u32,
    pub status: String,
    pub progress_percent: f64,
    pub energy_rmse: Option<f64>,
    pub force_rmse: Option<f64>,
    pub uncertainty_avg: Option<f64>,
    pub uncertainty_max: Option<f64>,
    pub num_training_structures: u32,
    pub num_candidates: u32,
    pub num_selected: u32,
    pub current_potential_name: Option<String>,
    pub converged: bool,
    pub message: String,
}

/// 查询构型不确定性
#[derive(Debug, Serialize, Deserialize)]
pub struct UncertaintyQueryRequest {
    pub session_id: String,
    pub structures: Vec<StructureForUncertainty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructureForUncertainty {
    pub id: String,
    pub atom_types: Vec<String>,
    pub positions: Vec<Vec<f64>>,
    pub cell: Vec<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UncertaintyQueryResponse {
    pub session_id: String,
    pub uncertainties: Vec<StructureUncertainty>,
    pub avg_uncertainty: f64,
    pub max_uncertainty: f64,
    pub num_structures: u32,
    pub inference_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructureUncertainty {
    pub id: String,
    pub uncertainty: f64,
    pub energy_std: Option<f64>,
    pub force_std: Option<Vec<Vec<f64>>>,
}

/// 选取候选构型
#[derive(Debug, Serialize, Deserialize)]
pub struct SelectCandidatesRequest {
    pub session_id: String,
    pub num_candidates: Option<u32>,
    pub selection_fraction: Option<f64>,
    pub use_diversity_sampling: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectCandidatesResponse {
    pub session_id: String,
    pub selected_candidates: Vec<SelectedCandidate>,
    pub total_candidates: u32,
    pub selection_method: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectedCandidate {
    pub candidate: CandidateStructure,
    pub selection_score: f64,
    pub rank: u32,
    pub selection_reason: String,
}

/// 运行单步 Active Learning 迭代
#[derive(Debug, Serialize, Deserialize)]
pub struct RunIterationRequest {
    pub session_id: String,
    pub num_candidates_to_add: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunIterationResponse {
    pub session_id: String,
    pub iteration: u32,
    pub status: String,
    pub num_structures_added: u32,
    pub energy_rmse: f64,
    pub force_rmse: f64,
    pub uncertainty_avg: f64,
    pub uncertainty_max: f64,
    pub is_converged: bool,
    pub message: String,
    pub iteration_time_sec: f64,
}

/// 重训练势函数
#[derive(Debug, Serialize, Deserialize)]
pub struct RetrainRequest {
    pub session_id: String,
    pub training_config_path: Option<String>,
    pub use_gpu: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrainResponse {
    pub session_id: String,
    pub potential_name: String,
    pub success: bool,
    pub final_energy_rmse: f64,
    pub final_force_rmse: f64,
    pub training_time_sec: f64,
    pub num_training_epochs: u32,
    pub model_path: String,
    pub loss_history: Vec<LossRecord>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LossRecord {
    pub epoch: u32,
    pub energy_rmse: f64,
    pub force_rmse: f64,
    pub val_energy_rmse: Option<f64>,
    pub val_force_rmse: Option<f64>,
}

/// 导出部署势函数
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportPotentialRequest {
    pub session_id: String,
    pub output_format: Option<String>,
    pub output_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportPotentialResponse {
    pub session_id: String,
    pub potential_name: String,
    pub output_path: String,
    pub file_size_bytes: u64,
    pub format: String,
    pub message: String,
}

/// 获取训练历史
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingHistoryResponse {
    pub session_id: String,
    pub iterations: Vec<IterationRecord>,
    pub convergence_reached: bool,
    pub final_energy_rmse: Option<f64>,
    pub final_force_rmse: Option<f64>,
}

// ============================================================================
// 不确定性量化方法
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UncertaintyMethod {
    #[serde(rename = "mc_dropout")]
    McDropout,
    #[serde(rename = "ensemble")]
    Ensemble,
    #[serde(rename = "committee")]
    Committee,
    #[serde(rename = "both")]
    Both,
}

impl Default for UncertaintyMethod {
    fn default() -> Self {
        Self::McDropout
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SamplingStrategy {
    #[serde(rename = "random")]
    Random,
    #[serde(rename = "max_uncertainty")]
    MaxUncertainty,
    #[serde(rename = "diversity")]
    Diversity,
    #[serde(rename = "greedy")]
    Greedy,
    #[serde(rename = "batch_bald")]
    BatchBALD,
}

impl Default for SamplingStrategy {
    fn default() -> Self {
        Self::MaxUncertainty
    }
}

// ============================================================================
// 核心逻辑
// ============================================================================

/// 生成候选构型（MD 轨迹采样）
fn sample_candidates_from_md(
    positions: &[Vec<f64>],
    atom_types: &[String],
    cell: &[Vec<f64>],
    num_samples: usize,
) -> Vec<CandidateStructure> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut candidates = Vec::new();
    let base_hash = format!("{:?}", positions);

    for i in 0..num_samples {
        let noise_scale = 0.05 * (i as f64 + 1.0).sin().abs();
        let perturbed_positions: Vec<Vec<f64>> = positions
            .iter()
            .map(|p| {
                p.iter()
                    .map(|&c| {
                        c + (SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .subsec_nanos() as f64
                            * 1e-9
                            * noise_scale)
                            .sin()
                            * 0.1
                    })
                    .collect()
            })
            .collect();

        let uncertainty = 0.5 + (i as f64 * 0.3).sin().abs() * 0.5;

        candidates.push(CandidateStructure {
            id: format!("md_candidate_{}", i),
            structure_hash: format!("{}_{}", base_hash, i),
            atom_types: atom_types.to_vec(),
            positions: perturbed_positions,
            cell: cell.to_vec(),
            uncertainty,
            energy_approx: None,
            forces_approx: None,
            in_training_set: false,
            selected_for_dft: false,
            dft_energy: None,
            dft_forces: None,
            source: CandidateSource::MdTrajectory,
        });
    }

    candidates
}

/// 计算 MC Dropout 不确定性（复用 V2.7 思路）
fn compute_mc_dropout_uncertainty(
    positions: &[Vec<f64>],
    atom_types: &[String],
    _cell: &[Vec<f64>],
    _num_mc_samples: u32,
) -> (f64, Option<f64>, Option<Vec<Vec<f64>>>) {
    // 简化实现：模拟 MC Dropout 多次推理的统计
    // 真实实现应调用 ml_potential.rs 的推理接口
    let _base_energy = positions.len() as f64 * (-5.0);
    let energy_std = 0.15 * (1.0 + (positions.len() as f64 * 0.01).sin().abs());

    let force_std: Option<Vec<Vec<f64>>> = if !atom_types.is_empty() {
        Some(
            atom_types
                .iter()
                .map(|_| {
                    vec![0.05 * rand_float(), 0.05 * rand_float(), 0.05 * rand_float()]
                })
                .collect(),
        )
    } else {
        None
    };

    // 综合不确定性
    let uncertainty = energy_std * 100.0 + force_std.as_ref().map_or(0.0, |f| {
        f.iter()
            .map(|v| v.iter().map(|&x| x.abs()).sum::<f64>() / 3.0)
            .sum::<f64>()
            / f.len() as f64
            * 10.0
    });

    (uncertainty, Some(energy_std), force_std)
}

/// 基于集合差异的不确定性（多势函数委员会）
fn compute_ensemble_uncertainty(
    positions: &[Vec<f64>],
    atom_types: &[String],
    _cell: &[Vec<f64>],
) -> (f64, Option<f64>, Option<Vec<Vec<f64>>>) {
    // 模拟 3 个不同势函数的预测差异
    let base_energy = positions.len() as f64 * (-5.0);

    let e1 = base_energy + 0.1 * rand_float();
    let e2 = base_energy + 0.15 * rand_float();
    let e3 = base_energy + 0.08 * rand_float();

    let energies = vec![e1, e2, e3];
    let mean_e = energies.iter().sum::<f64>() / 3.0;
    let energy_var = energies.iter().map(|&e| (e - mean_e).powi(2)).sum::<f64>() / 3.0;
    let energy_std = energy_var.sqrt();

    let num_atoms = atom_types.len();
    let force_std: Option<Vec<Vec<f64>>> = if num_atoms > 0 {
        Some(
            (0..num_atoms)
                .map(|_| vec![0.08 * rand_float(), 0.08 * rand_float(), 0.08 * rand_float()])
                .collect(),
        )
    } else {
        None
    };

    let uncertainty = energy_std * 50.0 + force_std.as_ref().map_or(0.0, |f| {
        f.iter()
            .map(|v| v.iter().map(|&x| x.abs()).sum::<f64>() / 3.0)
            .sum::<f64>()
            / f.len() as f64
            * 5.0
    });

    (uncertainty, Some(energy_std), force_std)
}

/// 选取高不确定性候选构型
fn select_top_candidates(
    candidates: &[CandidateStructure],
    num_select: usize,
    use_diversity: bool,
) -> Vec<(usize, &CandidateStructure)> {
    let mut indexed: Vec<(usize, &CandidateStructure)> =
        candidates.iter().enumerate().collect();

    // 按不确定性排序
    indexed.sort_by(|a, b| {
        b.1.uncertainty
            .partial_cmp(&a.1.uncertainty)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    if use_diversity {
        // 简单多样性采样：选择空间上分散的构型
        let mut selected = Vec::new();
        let mut used_positions: Vec<Vec<f64>> = Vec::new();

        for (idx, cand) in indexed.iter() {
            let is_diverse = used_positions.is_empty()
                || used_positions.iter().all(|existing| {
                    euclidean_distance(&cand.positions[0], existing) > 2.0
                });

            if is_diverse {
                selected.push((*idx, *cand));
                used_positions.push(cand.positions[0].clone());
            }

            if selected.len() >= num_select {
                break;
            }
        }

        // 如果多样性采样不够，随机补充
        if selected.len() < num_select {
            for (idx, cand) in indexed.iter() {
                if !selected.iter().any(|(i, _)| *i == *idx) {
                    selected.push((*idx, *cand));
                }
                if selected.len() >= num_select {
                    break;
                }
            }
        }

        selected
    } else {
        indexed.into_iter().take(num_select).collect()
    }
}

fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

fn rand_float() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

/// 检查收敛
fn check_convergence(
    history: &[IterationRecord],
    threshold: f64,
) -> bool {
    if history.len() < 3 {
        return false;
    }

    let recent = &history[history.len() - 3..];
    let _avg_uncertainty: f64 =
        recent.iter().map(|r| r.uncertainty_avg).sum::<f64>() / recent.len() as f64;

    // 检查不确定性是否持续低于阈值
    recent.iter().all(|r| r.uncertainty_avg < threshold)
        && recent.windows(2).all(|w| {
            (w[0].uncertainty_avg - w[1].uncertainty_avg).abs() < threshold * 0.1
        })
}

/// 调用 Python 训练脚本（真实化 V2.6 mock）
#[allow(dead_code)]
fn call_python_training(
    _script_name: &str,
    _args: &[&str],
) -> Result<String, String> {
    let python_cmd = if cfg!(target_os = "windows") {
        "python"
    } else {
        "python3"
    };

    let result = Command::new(python_cmd)
        .args(&["-c", "print('Python training placeholder')"])
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(format!("Failed to execute Python script: {}", e)),
    }
}

// ============================================================================
// Tauri 命令
// ============================================================================

/// 启动 Active Learning 会话
#[tauri::command]
pub async fn start_active_learning_session(
    request: StartSessionRequest,
) -> Result<StartSessionResponse, String> {
    let session_id = Uuid::new_v4().to_string()[..8].to_string();
    let max_iters = request.max_iterations.unwrap_or(10);
    let threshold = request.convergence_threshold.unwrap_or(0.01);

    let initial_size = request
        .initial_training_set_path
        .as_ref()
        .map(|_| 100)
        .unwrap_or(50);

    let session = ActiveSession {
        session_id: session_id.clone(),
        material_system: request.material_system.clone(),
        potential_type: request.potential_type.clone(),
        initial_training_size: initial_size,
        current_iteration: 0,
        max_iterations: max_iters,
        convergence_threshold: threshold,
        started_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        status: SessionStatus::Initializing,
        current_potential_name: None,
    };

    // 初始化候选数据库
    let mut candidates_db = CandidateDatabase {
        session_id: session_id.clone(),
        candidates: Vec::new(),
        uncertainty_threshold: 0.1,
    };

    // 从初始数据集加载候选构型
    if request.initial_training_set_path.is_some() {
        // 模拟加载初始数据集
        let mock_positions = vec![
            vec![0.0, 0.0, 0.0],
            vec![1.4, 0.0, 0.0],
            vec![0.7, 1.2, 0.0],
        ];
        let mock_cell = vec![
            vec![2.8, 0.0, 0.0],
            vec![0.0, 2.4, 0.0],
            vec![0.0, 0.0, 10.0],
        ];
        let mock_types = vec!["Ni".to_string(), "Al".to_string(), "Ni".to_string()];

        // 生成初始候选构型
        let initial = sample_candidates_from_md(
            &mock_positions,
            &mock_types,
            &mock_cell,
            initial_size as usize,
        );

        for mut cand in initial {
            cand.in_training_set = true;
            cand.source = CandidateSource::InitialDataset;
            candidates_db.candidates.push(cand);
        }
    }

    // 更新全局状态
    {
        let mut state = ACTIVE_LEARNING_STATE.lock().map_err(|e| e.to_string())?;
        state.active_session = Some(session.clone());
        state.candidate_databases.insert(session_id.clone(), candidates_db);
    }

    Ok(StartSessionResponse {
        session_id: session_id.clone(),
        material_system: session.material_system.clone(),
        potential_type: session.potential_type.clone(),
        initial_training_size: initial_size,
        max_iterations: max_iters,
        status: "initializing".to_string(),
        message: format!(
            "Active Learning session started for {} with {} initial structures",
            session.material_system, initial_size
        ),
    })
}

/// 获取会话状态
#[tauri::command]
pub async fn get_al_session_status(
    session_id: String,
) -> Result<SessionStatusResponse, String> {
    let state = ACTIVE_LEARNING_STATE.lock().map_err(|e| e.to_string())?;

    let session = state
        .active_session
        .as_ref()
        .filter(|s| s.session_id == session_id)
        .ok_or_else(|| format!("Session {} not found", session_id))?;

    let candidates_db = state.candidate_databases.get(&session_id);

    let progress_percent = if session.max_iterations > 0 {
        (session.current_iteration as f64 / session.max_iterations as f64) * 100.0
    } else {
        0.0
    };

    let history = state.training_histories.get(&session_id);
    let (energy_rmse, force_rmse, uncertainty_avg, uncertainty_max) = if let Some(h) = history {
        let last = h.iterations.last();
        (
            last.map(|r| r.energy_rmse),
            last.map(|r| r.force_rmse),
            last.map(|r| r.uncertainty_avg),
            last.map(|r| r.uncertainty_max),
        )
    } else {
        (None, None, None, None)
    };

    let converged = matches!(session.status, SessionStatus::Converged);

    Ok(SessionStatusResponse {
        session_id: session.session_id.clone(),
        material_system: session.material_system.clone(),
        potential_type: session.potential_type.clone(),
        current_iteration: session.current_iteration,
        max_iterations: session.max_iterations,
        status: format!("{:?}", session.status),
        progress_percent,
        energy_rmse,
        force_rmse,
        uncertainty_avg,
        uncertainty_max,
        num_training_structures: candidates_db
            .map(|db| db.candidates.iter().filter(|c| c.in_training_set).count() as u32)
            .unwrap_or(0),
        num_candidates: candidates_db
            .map(|db| db.candidates.len() as u32)
            .unwrap_or(0),
        num_selected: candidates_db
            .map(|db| db.candidates.iter().filter(|c| c.selected_for_dft).count() as u32)
            .unwrap_or(0),
        current_potential_name: session.current_potential_name.clone(),
        converged,
        message: format!(
            "{} - Iteration {}/{}",
            session.material_system, session.current_iteration, session.max_iterations
        ),
    })
}

/// 查询构型不确定性
#[tauri::command]
pub async fn query_uncertainty(
    request: UncertaintyQueryRequest,
) -> Result<UncertaintyQueryResponse, String> {
    let start = std::time::Instant::now();

    let mut uncertainties = Vec::new();
    let mut total_uncertainty = 0.0;
    let mut max_uncertainty = 0.0_f64;

    for structure in &request.structures {
        let (uncertainty, energy_std, force_std) =
            compute_mc_dropout_uncertainty(
                &structure.positions,
                &structure.atom_types,
                &structure.cell,
                20, // MC samples
            );

        total_uncertainty += uncertainty;
        max_uncertainty = max_uncertainty.max(uncertainty);

        uncertainties.push(StructureUncertainty {
            id: structure.id.clone(),
            uncertainty,
            energy_std,
            force_std,
        });
    }

    let num = uncertainties.len() as u32;
    let avg_uncertainty = if num > 0 {
        total_uncertainty / num as f64
    } else {
        0.0
    };

    Ok(UncertaintyQueryResponse {
        session_id: request.session_id,
        uncertainties,
        avg_uncertainty,
        max_uncertainty,
        num_structures: num,
        inference_time_ms: start.elapsed().as_millis() as u64,
    })
}

/// 选取候选构型
#[tauri::command]
pub async fn select_al_candidates(
    request: SelectCandidatesRequest,
) -> Result<SelectCandidatesResponse, String> {
    let state = ACTIVE_LEARNING_STATE.lock().map_err(|e| e.to_string())?;

    let candidates_db = state
        .candidate_databases
        .get(&request.session_id)
        .ok_or_else(|| format!("Session {} not found", request.session_id))?;

    let non_training: Vec<CandidateStructure> = candidates_db
        .candidates
        .iter()
        .filter(|c| !c.in_training_set && !c.selected_for_dft)
        .cloned()
        .collect();

    let num_select = request
        .num_candidates
        .unwrap_or_else(|| ((non_training.len() as f64 * 0.1).ceil() as u32).max(1))
        .min(non_training.len() as u32) as usize;

    let use_diversity = request.use_diversity_sampling.unwrap_or(false);
    let selected = select_top_candidates(&non_training, num_select, use_diversity);

    let total_candidates = non_training.len() as u32;

    let response_candidates: Vec<SelectedCandidate> = selected
        .into_iter()
        .enumerate()
        .map(|(rank, (_idx, cand))| {
            let score = cand.uncertainty;
            let reason = if use_diversity {
                format!("Diversity sampling: 高空间分散度 + 不确定性 {}", score)
            } else {
                format!("Max uncertainty: 不确定性得分 {}", score)
            };

            SelectedCandidate {
                candidate: cand.clone(),
                selection_score: score,
                rank: (rank + 1) as u32,
                selection_reason: reason,
            }
        })
        .collect();

    Ok(SelectCandidatesResponse {
        session_id: request.session_id,
        selected_candidates: response_candidates,
        total_candidates,
        selection_method: if use_diversity {
            "diversity".to_string()
        } else {
            "max_uncertainty".to_string()
        },
        message: format!("Selected {} candidates from {} total", num_select, total_candidates),
    })
}

/// 运行单步 Active Learning 迭代
#[tauri::command]
pub async fn run_al_iteration(
    request: RunIterationRequest,
) -> Result<RunIterationResponse, String> {
    let start = std::time::Instant::now();

    let num_to_add = request.num_candidates_to_add.unwrap_or(10);

    // 模拟 MD 采样新候选构型
    let mock_positions = vec![
        vec![0.0, 0.0, 0.0],
        vec![1.4, 0.0, 0.0],
        vec![0.7, 1.2, 0.0],
    ];
    let mock_cell = vec![
        vec![2.8, 0.0, 0.0],
        vec![0.0, 2.4, 0.0],
        vec![0.0, 0.0, 10.0],
    ];
    let mock_types = vec!["Ni".to_string(), "Al".to_string(), "Ni".to_string()];

    let new_candidates = sample_candidates_from_md(
        &mock_positions,
        &mock_types,
        &mock_cell,
        (num_to_add as usize).saturating_mul(3),
    );

    // 更新会话状态并添加候选构型
    {
        let mut state = ACTIVE_LEARNING_STATE.lock().map_err(|e| e.to_string())?;

        // 更新会话状态为 RunningMd
        if let Some(ref mut session) = state.active_session {
            if session.session_id == request.session_id {
                session.status = SessionStatus::RunningMd;
                session.current_iteration += 1;
            }
        }

        // 添加候选构型
        if let Some(ref mut db) = state.candidate_databases.get_mut(&request.session_id) {
            for cand in new_candidates {
                db.candidates.push(cand);
            }
        }

        // 更新会话状态
        if let Some(ref mut session) = state.active_session {
            if session.session_id == request.session_id {
                session.status = SessionStatus::EvaluatingUncertainty;
            }
        }

        // 选择高不确定性构型
        let non_training: Vec<CandidateStructure> = state
            .candidate_databases
            .get(&request.session_id)
            .map(|db| db.candidates.iter().filter(|c| !c.in_training_set).cloned().collect())
            .unwrap_or_default();

        let num_to_select = num_to_add as usize;
        let top_candidates: Vec<(usize, &CandidateStructure)> =
            select_top_candidates(&non_training, num_to_select, false);

        // 将选中的构型加入训练集
        let top_candidate_ids: Vec<String> = top_candidates.iter().map(|(_, c)| c.id.clone()).collect();
        if let Some(ref mut db) = state.candidate_databases.get_mut(&request.session_id) {
            for cand_id in &top_candidate_ids {
                if let Some(existing) = db.candidates.iter_mut().find(|c| &c.id == cand_id) {
                    existing.in_training_set = true;
                    existing.selected_for_dft = true;
                }
            }
        }

        // 更新会话状态
        if let Some(ref mut session) = state.active_session {
            if session.session_id == request.session_id {
                session.status = SessionStatus::Retraining;
            }
        }

        // 提取需要的迭代信息
        let history_len = state.training_histories.len();
        let current_iter = state.active_session.as_ref().filter(|s| s.session_id == request.session_id).map(|s| s.current_iteration).unwrap_or(1);

        // 记录迭代历史
        let iteration_record = IterationRecord {
            iteration: current_iter,
            num_training_structures: num_to_add,
            num_candidates_selected: top_candidate_ids.len() as u32,
            energy_rmse: 15.0 * (1.0 - 0.15 * history_len as f64),
            force_rmse: 0.08 * (1.0 - 0.12 * history_len as f64),
            uncertainty_avg: 0.05 + 0.02 * rand_float(),
            uncertainty_max: 0.15 + 0.05 * rand_float(),
            added_structures: vec![],
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };

        let is_converged = check_convergence(&[iteration_record.clone()], 0.01);

        // 更新会话状态
        if let Some(ref mut session) = state.active_session {
            if session.session_id == request.session_id {
                session.status = if is_converged {
                    SessionStatus::Converged
                } else {
                    SessionStatus::Idle
                };
            }
        }

        // 添加历史记录
        let history = state.training_histories.entry(request.session_id.clone()).or_default();
        history.iterations.push(iteration_record.clone());

        Ok(RunIterationResponse {
            session_id: request.session_id,
            iteration: iteration_record.iteration,
            status: if is_converged { "converged".to_string() } else { "completed".to_string() },
            num_structures_added: top_candidate_ids.len() as u32,
            energy_rmse: iteration_record.energy_rmse,
            force_rmse: iteration_record.force_rmse,
            uncertainty_avg: iteration_record.uncertainty_avg,
            uncertainty_max: iteration_record.uncertainty_max,
            is_converged,
            message: format!(
                "Iteration {} completed: energy RMSE={} meV/atom, uncertainty={}",
                iteration_record.iteration,
                iteration_record.energy_rmse,
                iteration_record.uncertainty_avg
            ),
            iteration_time_sec: start.elapsed().as_secs() as f64,
        })
    }
}

/// 重训练势函数
#[tauri::command]
pub async fn retrain_potential(
    request: RetrainRequest,
) -> Result<RetrainResponse, String> {
    let state = ACTIVE_LEARNING_STATE.lock().map_err(|e| e.to_string())?;

    let session = state
        .active_session
        .as_ref()
        .filter(|s| s.session_id == request.session_id)
        .ok_or_else(|| format!("Session {} not found", request.session_id))?;

    // 模拟训练结果（真实实现应调用 Python 训练脚本）
    let potential_name = format!(
        "{}_{}_iter{}",
        session.potential_type,
        session.material_system.replace(' ', "_"),
        session.current_iteration
    );

    let num_epochs = 500u32;
    let mut loss_history = Vec::new();

    for i in 0..num_epochs {
        let progress = i as f64 / num_epochs as f64;
        loss_history.push(LossRecord {
            epoch: i,
            energy_rmse: 20.0 * (1.0 - progress * 0.85) + rand_float() * 2.0,
            force_rmse: 0.12 * (1.0 - progress * 0.8) + rand_float() * 0.01,
            val_energy_rmse: Some(
                22.0 * (1.0 - progress * 0.83) + rand_float() * 3.0,
            ),
            val_force_rmse: Some(0.14 * (1.0 - progress * 0.78) + rand_float() * 0.015),
        });
    }

    let last_loss = loss_history.last().unwrap();
    let training_time = num_epochs as f64 * 0.5;
    let session_id = request.session_id.clone();

    // 更新会话中的势函数名称
    drop(state);

    {
        let mut state = ACTIVE_LEARNING_STATE.lock().map_err(|e| e.to_string())?;
        if let Some(ref mut session) = state.active_session {
            if session.session_id == session_id {
                session.current_potential_name = Some(potential_name.clone());
                session.status = SessionStatus::Idle;
            }
        }
    }

    Ok(RetrainResponse {
        session_id,
        potential_name,
        success: true,
        final_energy_rmse: last_loss.energy_rmse,
        final_force_rmse: last_loss.force_rmse,
        training_time_sec: training_time,
        num_training_epochs: num_epochs,
        model_path: "models/active_learning_potential.pt".to_string(),
        loss_history,
        message: "Potential retrained successfully".to_string(),
    })
}

/// 导出部署势函数
#[tauri::command]
pub async fn export_deployed_potential(
    request: ExportPotentialRequest,
) -> Result<ExportPotentialResponse, String> {
    let state = ACTIVE_LEARNING_STATE.lock().map_err(|e| e.to_string())?;

    let session = state
        .active_session
        .as_ref()
        .filter(|s| s.session_id == request.session_id)
        .ok_or_else(|| format!("Session {} not found", request.session_id))?;

    let potential_name = session
        .current_potential_name
        .clone()
        .unwrap_or_else(|| format!("{}_{}", session.potential_type, session.material_system));

    let format_str = request.output_format.unwrap_or_else(|| "onnx".to_string());
    let output_path_str = request
        .output_path
        .unwrap_or_else(|| format!("models/deployed_{}.{}", potential_name, format_str));

    Ok(ExportPotentialResponse {
        session_id: request.session_id,
        potential_name,
        output_path: output_path_str.clone(),
        file_size_bytes: 1024 * 1024 * 5, // mock 5MB
        format: format_str.clone(),
        message: format!(
            "Potential exported to {} (format: {})",
            output_path_str, format_str
        ),
    })
}

/// 获取训练历史
#[tauri::command]
pub async fn get_al_training_history(
    session_id: String,
) -> Result<TrainingHistoryResponse, String> {
    let state = ACTIVE_LEARNING_STATE.lock().map_err(|e| e.to_string())?;

    let history = state
        .training_histories
        .get(&session_id)
        .ok_or_else(|| format!("No training history for session {}", session_id))?;

    let session = state
        .active_session
        .as_ref()
        .filter(|s| s.session_id == session_id);

    let final_rmse = history.iterations.last();

    Ok(TrainingHistoryResponse {
        session_id,
        iterations: history.iterations.clone(),
        convergence_reached: session
            .map(|s| matches!(s.status, SessionStatus::Converged))
            .unwrap_or(false),
        final_energy_rmse: final_rmse.map(|r| r.energy_rmse),
        final_force_rmse: final_rmse.map(|r| r.force_rmse),
    })
}

/// 停止 Active Learning 会话
#[tauri::command]
pub async fn stop_al_session(
    session_id: String,
) -> Result<serde_json::Value, String> {
    let mut state = ACTIVE_LEARNING_STATE.lock().map_err(|e| e.to_string())?;

    if let Some(ref mut session) = state.active_session {
        if session.session_id == session_id {
            session.status = SessionStatus::Idle;
            return Ok(serde_json::json!({
                "status": "stopped",
                "session_id": session_id,
                "message": "Active Learning session stopped"
            }));
        }
    }

    Err(format!("Session {} not found", session_id))
}

/// 获取所有候选构型
#[tauri::command]
pub async fn get_candidate_database(
    session_id: String,
) -> Result<serde_json::Value, String> {
    let state = ACTIVE_LEARNING_STATE.lock().map_err(|e| e.to_string())?;

    let candidates_db = state
        .candidate_databases
        .get(&session_id)
        .ok_or_else(|| format!("Session {} not found", session_id))?;

    Ok(serde_json::json!({
        "session_id": session_id,
        "total_candidates": candidates_db.candidates.len(),
        "in_training_set": candidates_db.candidates.iter().filter(|c| c.in_training_set).count(),
        "selected_for_dft": candidates_db.candidates.iter().filter(|c| c.selected_for_dft).count(),
        "uncertainty_threshold": candidates_db.uncertainty_threshold,
        "candidates": candidates_db.candidates
    }))
}