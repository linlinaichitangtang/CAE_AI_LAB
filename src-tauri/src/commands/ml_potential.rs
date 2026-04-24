/**
 * V2.6 AI × MD 大体系加速 — ML 势函数后端命令
 * - V2.6-001: ML 势函数接口抽象层
 * - V2.6-002: LAMMPS + NequIP/MACE 集成 (mock)
 * - V2.6-003: 势函数自动选择逻辑
 * - V2.6-004: GPU 资源管理 (mock)
 * - V2.6-005: DFT 数据接口
 * - V2.6-006/007: NequIP/MACE 训练 Pipeline (mock)
 * - V2.6-008: 势函数质量验证
 * - V2.6-009: 训练超参数推荐
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// V2.6-001: ML 势函数接口抽象层
// ============================================================================

/// ML 势函数类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MLPotentialType {
    NequIP,
    MACE,
    NEP,
    MTP,
    ACE,
    GAP,
    CHGNet,
    SevenNet,
}

impl MLPotentialType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "nequip" => Some(Self::NequIP),
            "mace" => Some(Self::MACE),
            "nep" => Some(Self::NEP),
            "mtp" => Some(Self::MTP),
            "ace" => Some(Self::ACE),
            "gap" => Some(Self::GAP),
            "chgnet" => Some(Self::CHGNet),
            "sevennet" => Some(Self::SevenNet),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::NequIP => "nequip",
            Self::MACE => "mace",
            Self::NEP => "nep",
            Self::MTP => "mtp",
            Self::ACE => "ace",
            Self::GAP => "gap",
            Self::CHGNet => "chgnet",
            Self::SevenNet => "sevennet",
        }
    }

    /// 是否为通用预训练势（无需训练，直接使用）
    pub fn is_pretrained(&self) -> bool {
        matches!(self, Self::CHGNet | Self::MACE | Self::SevenNet)
    }
}

/// ML 势函数信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MLPotentialInfo {
    /// 势函数名称
    pub name: String,
    /// 势函数类型
    pub potential_type: String,
    /// 描述
    pub description: String,
    /// 适用元素列表
    pub supported_elements: Vec<String>,
    /// 精度指标 (meV/atom)
    pub energy_rmse: Option<f64>,
    /// 力精度 (meV/Å)
    pub force_rmse: Option<f64>,
    /// 截断半径 (Å)
    pub cutoff: f64,
    /// 模型文件路径
    pub model_path: Option<String>,
    /// 训练数据集大小
    pub training_data_size: Option<u32>,
    /// 是否已训练/加载
    pub is_ready: bool,
    /// 训练时间 (秒)
    pub training_time_sec: Option<f64>,
    /// 创建时间
    pub created_at: Option<String>,
}

/// ML 势函数推理请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MLPotentialComputeRequest {
    /// 势函数名称
    pub potential_name: String,
    /// 原子位置 (Å): [[x, y, z], ...]
    pub positions: Vec<Vec<f64>>,
    /// 原子类型列表: ["Fe", "Fe", "Cr", ...]
    pub atom_types: Vec<String>,
    /// 晶胞向量 (3x3): [[ax, ay, az], [bx, by, bz], [cx, cy, cz]]
    pub cell: Option<Vec<Vec<f64>>>,
    /// 是否计算力
    pub compute_forces: bool,
    /// 是否计算应力
    pub compute_stress: bool,
}

/// ML 势函数推理结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MLPotentialComputeResult {
    /// 势函数名称
    pub potential_name: String,
    /// 总能量 (eV)
    pub total_energy: f64,
    /// 每原子能量 (eV/atom)
    pub energy_per_atom: f64,
    /// 原子上的力 (eV/Å): [[fx, fy, fz], ...]
    pub forces: Option<Vec<Vec<f64>>>,
    /// 维里应力 (3x3): [[xx, xy, xz], [yx, yy, yz], [zx, zy, zz]]
    pub stress: Option<Vec<Vec<f64>>>,
    /// 推理耗时 (ms)
    pub inference_time_ms: u64,
    /// 原子数
    pub num_atoms: u32,
}

// ============================================================================
// V2.6-003: 势函数自动选择
// ============================================================================

/// 势函数选择请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PotentialSelectionRequest {
    /// 原子种类列表
    pub elements: Vec<String>,
    /// 原子总数
    pub num_atoms: u32,
    /// 精度要求: "high" | "medium" | "low"
    pub accuracy_requirement: String,
    /// 是否有 GPU
    pub has_gpu: bool,
}

/// 势函数选择结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PotentialSelectionResult {
    /// 推荐的势函数类型
    pub recommended_type: String,
    /// 推荐理由
    pub reason: String,
    /// 是否需要训练
    pub needs_training: bool,
    /// 预估训练时间 (分钟)
    pub estimated_training_minutes: Option<u32>,
    /// 预估显存需求 (GB)
    pub estimated_vram_gb: Option<f64>,
    /// 备选方案
    pub alternatives: Vec<AlternativePotential>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlternativePotential {
    pub potential_type: String,
    pub reason: String,
}

// ============================================================================
// V2.6-004: GPU 资源管理
// ============================================================================

/// GPU 信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GpuInfo {
    pub name: String,
    pub total_memory_gb: f64,
    pub available_memory_gb: f64,
    pub is_available: bool,
    pub cuda_version: Option<String>,
}

/// GPU 资源状态
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GpuResourceStatus {
    pub gpus: Vec<GpuInfo>,
    pub total_vram_gb: f64,
    pub used_vram_gb: f64,
    pub running_tasks: u32,
    pub queued_tasks: u32,
}

// ============================================================================
// V2.6-005: DFT 数据接口
// ============================================================================

/// DFT 训练数据点
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DftDataPoint {
    /// 结构 ID
    pub structure_id: String,
    /// 原子类型
    pub atom_types: Vec<String>,
    /// 原子位置
    pub positions: Vec<Vec<f64>>,
    /// 晶胞
    pub cell: Vec<Vec<f64>>,
    /// DFT 能量 (eV)
    pub energy: f64,
    /// DFT 力 (eV/Å)
    pub forces: Vec<Vec<f64>>,
    /// 应力 (可选)
    pub stress: Option<Vec<Vec<f64>>>,
    /// 数据来源
    pub source: String,
}

/// DFT 数据集信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DftDatasetInfo {
    pub name: String,
    pub num_structures: u32,
    pub num_atoms_range: (u32, u32),
    pub elements: Vec<String>,
    pub source: String,
    pub file_path: Option<String>,
}

// ============================================================================
// V2.6-006/007: 训练 Pipeline
// ============================================================================

/// 训练配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrainingConfig {
    /// 势函数类型
    pub potential_type: String,
    /// 训练数据路径
    pub training_data_path: String,
    /// 验证数据路径
    pub validation_data_path: Option<String>,
    /// 截断半径 (Å)
    pub cutoff: f64,
    /// 最大神经网络神经元数
    pub max_neuron: u32,
    /// 训练轮数
    pub epochs: u32,
    /// 批大小
    pub batch_size: u32,
    /// 学习率
    pub learning_rate: f64,
    /// 能量权重
    pub energy_weight: f64,
    /// 力权重
    pub force_weight: f64,
    /// 应力权重
    pub stress_weight: Option<f64>,
    /// 是否使用 GPU
    pub use_gpu: bool,
}

/// 训练状态
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrainingStatus {
    /// 训练任务 ID
    pub task_id: String,
    /// 势函数名称
    pub potential_name: String,
    /// 状态
    pub status: String, // "queued" | "running" | "completed" | "failed"
    /// 当前进度 (0~100)
    pub progress_percent: f64,
    /// 当前 epoch
    pub current_epoch: u32,
    /// 总 epochs
    pub total_epochs: u32,
    /// 当前能量 RMSE
    pub energy_rmse: Option<f64>,
    /// 当前力 RMSE
    pub force_rmse: Option<f64>,
    /// 已用时间 (秒)
    pub elapsed_time_sec: f64,
    /// 预估剩余时间 (秒)
    pub estimated_remaining_sec: Option<f64>,
    /// 错误信息
    pub error: Option<String>,
}

/// 训练结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrainingResult {
    /// 势函数名称
    pub potential_name: String,
    /// 势函数类型
    pub potential_type: String,
    /// 是否成功
    pub success: bool,
    /// 最终能量 RMSE (meV/atom)
    pub final_energy_rmse: f64,
    /// 最终力 RMSE (meV/Å)
    pub final_force_rmse: f64,
    /// 最佳 epoch
    pub best_epoch: u32,
    /// 模型文件路径
    pub model_path: String,
    /// 训练耗时 (秒)
    pub training_time_sec: f64,
    /// 损失历史
    pub loss_history: Vec<LossRecord>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LossRecord {
    pub epoch: u32,
    pub energy_rmse: f64,
    pub force_rmse: f64,
    pub val_energy_rmse: Option<f64>,
    pub val_force_rmse: Option<f64>,
}

// ============================================================================
// V2.6-008: 势函数质量验证
// ============================================================================

/// 验证请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationRequest {
    /// 势函数名称
    pub potential_name: String,
    /// 测试数据路径
    pub test_data_path: String,
}

/// 验证结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationResult {
    /// 势函数名称
    pub potential_name: String,
    /// 能量 RMSE (meV/atom)
    pub energy_rmse: f64,
    /// 力 RMSE (meV/Å)
    pub force_rmse: f64,
    /// 能量 MAE (meV/atom)
    pub energy_mae: f64,
    /// 力 MAE (meV/Å)
    pub force_mae: f64,
    /// 测试结构数
    pub num_test_structures: u32,
    /// 是否通过 (Energy < 5 meV/atom, Forces < 100 meV/Å)
    pub passed: bool,
    /// 详细结果
    pub details: Vec<StructureValidationDetail>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StructureValidationDetail {
    pub structure_id: String,
    pub dft_energy: f64,
    pub ml_energy: f64,
    pub energy_error: f64,
    pub max_force_error: f64,
}

// ============================================================================
// V2.6-009: 训练超参数推荐
// ============================================================================

/// 超参数推荐请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HyperparamRecommendationRequest {
    /// 体系原子数
    pub num_atoms: u32,
    /// 原子种类数
    pub num_species: u32,
    /// 训练数据量
    pub num_training_structures: u32,
    /// 势函数类型
    pub potential_type: String,
}

/// 超参数推荐结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HyperparamRecommendation {
    pub cutoff: f64,
    pub max_neuron: u32,
    pub epochs: u32,
    pub batch_size: u32,
    pub learning_rate: f64,
    pub energy_weight: f64,
    pub force_weight: f64,
    pub estimated_training_time_minutes: u32,
    pub estimated_vram_gb: f64,
    pub reasoning: String,
}

// ============================================================================
// Mock 数据 & 实现函数
// ============================================================================

/// 生成 mock 势函数列表
fn get_mock_potentials() -> Vec<MLPotentialInfo> {
    vec![
        MLPotentialInfo {
            name: "MACE-MP-0".to_string(),
            potential_type: "mace".to_string(),
            description: "MACE 通用势函数 (Materials Project 预训练)".to_string(),
            supported_elements: vec!["H".into(), "He".into(), "Li".into(), "Be".into(), "B".into(), "C".into(), "N".into(), "O".into(), "F".into(), "Ne".into(), "Na".into(), "Mg".into(), "Al".into(), "Si".into(), "P".into(), "S".into(), "Cl".into(), "Ar".into(), "K".into(), "Ca".into(), "Sc".into(), "Ti".into(), "V".into(), "Cr".into(), "Mn".into(), "Fe".into(), "Co".into(), "Ni".into(), "Cu".into(), "Zn".into(), "Ga".into(), "Ge".into(), "As".into(), "Se".into(), "Br".into(), "Kr".into(), "Rb".into(), "Sr".into(), "Y".into(), "Zr".into(), "Nb".into(), "Mo".into(), "Tc".into(), "Ru".into(), "Rh".into(), "Pd".into(), "Ag".into(), "Cd".into(), "In".into(), "Sn".into(), "Sb".into(), "Te".into(), "I".into(), "Xe".into(), "Cs".into(), "Ba".into(), "La".into(), "Hf".into(), "Ta".into(), "W".into(), "Re".into(), "Os".into(), "Ir".into(), "Pt".into(), "Au".into(), "Hg".into(), "Tl".into(), "Pb".into(), "Bi".into()],
            energy_rmse: Some(11.0),
            force_rmse: Some(0.058),
            cutoff: 5.0,
            model_path: Some("models/mace-mp-0.pt".to_string()),
            training_data_size: Some(1_600_000),
            is_ready: true,
            training_time_sec: None,
            created_at: Some("2025-01-15T00:00:00Z".to_string()),
        },
        MLPotentialInfo {
            name: "CHGNet-v1".to_string(),
            potential_type: "chgnet".to_string(),
            description: "CHGNet 通用势函数 (预训练)".to_string(),
            supported_elements: vec!["H".into(), "Li".into(), "Be".into(), "B".into(), "C".into(), "N".into(), "O".into(), "F".into(), "Na".into(), "Mg".into(), "Al".into(), "Si".into(), "P".into(), "S".into(), "Cl".into(), "K".into(), "Ca".into(), "Sc".into(), "Ti".into(), "V".into(), "Cr".into(), "Mn".into(), "Fe".into(), "Co".into(), "Ni".into(), "Cu".into(), "Zn".into(), "Ga".into(), "Ge".into(), "As".into(), "Se".into(), "Br".into(), "Rb".into(), "Sr".into(), "Y".into(), "Zr".into(), "Nb".into(), "Mo".into(), "Ru".into(), "Rh".into(), "Pd".into(), "Ag".into(), "Cd".into(), "In".into(), "Sn".into(), "Sb".into(), "Te".into(), "I".into(), "Cs".into(), "Ba".into(), "La".into(), "Hf".into(), "Ta".into(), "W".into(), "Re".into(), "Os".into(), "Ir".into(), "Pt".into(), "Au".into(), "Hg".into(), "Tl".into(), "Pb".into(), "Bi".into()],
            energy_rmse: Some(15.0),
            force_rmse: Some(0.075),
            cutoff: 6.0,
            model_path: Some("models/chgnet_v1.pt".to_string()),
            training_data_size: Some(1_500_000),
            is_ready: true,
            training_time_sec: None,
            created_at: Some("2025-02-20T00:00:00Z".to_string()),
        },
        MLPotentialInfo {
            name: "SevenNet-0".to_string(),
            potential_type: "sevennet".to_string(),
            description: "SevenNet 通用势函数 (预训练)".to_string(),
            supported_elements: vec!["H".into(), "C".into(), "N".into(), "O".into(), "F".into(), "Si".into(), "P".into(), "S".into(), "Cl".into(), "Fe".into(), "Cu".into(), "Zn".into()],
            energy_rmse: Some(12.0),
            force_rmse: Some(0.060),
            cutoff: 5.0,
            model_path: Some("models/sevennet_0.pt".to_string()),
            training_data_size: Some(800_000),
            is_ready: true,
            training_time_sec: None,
            created_at: Some("2025-03-10T00:00:00Z".to_string()),
        },
    ]
}

/// Mock 势函数推理
fn mock_potential_compute(req: &MLPotentialComputeRequest) -> MLPotentialComputeResult {
    let num_atoms = req.atom_types.len() as u32;
    let base_energy = num_atoms as f64 * (-6.5 + rand_factor() * 0.5);

    let forces = if req.compute_forces {
        Some((0..num_atoms).map(|_| {
            vec![rand_factor() * 0.1, rand_factor() * 0.1, rand_factor() * 0.1]
        }).collect())
    } else {
        None
    };

    let stress = if req.compute_stress {
        Some(vec![
            vec![rand_factor() * 0.5, rand_factor() * 0.1, rand_factor() * 0.1],
            vec![rand_factor() * 0.1, rand_factor() * 0.5, rand_factor() * 0.1],
            vec![rand_factor() * 0.1, rand_factor() * 0.1, rand_factor() * 0.5],
        ])
    } else {
        None
    };

    MLPotentialComputeResult {
        potential_name: req.potential_name.clone(),
        total_energy: base_energy,
        energy_per_atom: base_energy / num_atoms as f64,
        forces,
        stress,
        inference_time_ms: (num_atoms as f64 * 0.01) as u64 + 1,
        num_atoms,
    }
}

/// 势函数自动选择逻辑（内部实现）
fn select_potential_impl(req: &PotentialSelectionRequest) -> PotentialSelectionResult {
    let elements_set: std::collections::HashSet<&str> = req.elements.iter().map(|s| s.as_str()).collect();
    let num_atoms = req.num_atoms;

    // 规则 1: 小体系 (< 500 原子) → 建议纯 DFT
    if num_atoms < 500 {
        return PotentialSelectionResult {
            recommended_type: "dft".to_string(),
            reason: format!("体系较小 ({} 原子 < 500)，建议直接使用 DFT 计算获得最高精度", num_atoms),
            needs_training: false,
            estimated_training_minutes: None,
            estimated_vram_gb: None,
            alternatives: vec![
                AlternativePotential { potential_type: "chgnet".to_string(), reason: "如需快速筛选，可使用 CHGNet 通用势".to_string() },
            ],
        };
    }

    // 规则 2: 检查是否有通用预训练势覆盖
    let pretrained_potentials = ["mace", "chgnet", "sevennet"];
    for pt in &pretrained_potentials {
        // 简化判断: 如果元素都在周期表前 86 个元素内，通用势大概率覆盖
        let all_covered = elements_set.iter().all(|e| {
            matches!(*e, "H"|"He"|"Li"|"Be"|"B"|"C"|"N"|"O"|"F"|"Ne"|"Na"|"Mg"|"Al"|"Si"|"P"|"S"|"Cl"|"Ar"|"K"|"Ca"|"Sc"|"Ti"|"V"|"Cr"|"Mn"|"Fe"|"Co"|"Ni"|"Cu"|"Zn"|"Ga"|"Ge"|"As"|"Se"|"Br"|"Kr"|"Rb"|"Sr"|"Y"|"Zr"|"Nb"|"Mo"|"Tc"|"Ru"|"Rh"|"Pd"|"Ag"|"Cd"|"In"|"Sn"|"Sb"|"Te"|"I"|"Xe"|"Cs"|"Ba"|"La"|"Hf"|"Ta"|"W"|"Re"|"Os"|"Ir"|"Pt"|"Au"|"Hg"|"Tl"|"Pb"|"Bi")
        });

        if all_covered {
            let vram = match num_atoms {
                0..=10000 => 4.0,
                10001..=50000 => 12.0,
                _ => 24.0,
            };

            return PotentialSelectionResult {
                recommended_type: pt.to_string(),
                reason: format!("通用预训练势 {} 覆盖所有元素 ({} 种)，可直接使用", pt, elements_set.len()),
                needs_training: false,
                estimated_training_minutes: None,
                estimated_vram_gb: Some(vram),
                alternatives: vec![
                    AlternativePotential { potential_type: "nequip".to_string(), reason: "如需更高精度，可训练专用 NequIP 势".to_string() },
                    AlternativePotential { potential_type: "nep".to_string(), reason: "NEP 训练速度快，适合快速迭代".to_string() },
                ],
            };
        }
    }

    // 规则 3: 无通用势覆盖 → 需要训练专用势
    let training_time = match num_atoms {
        0..=5000 => 30,
        5001..=20000 => 120,
        _ => 300,
    };

    PotentialSelectionResult {
        recommended_type: "nequip".to_string(),
        reason: format!("元素 {} 无通用预训练势覆盖，建议训练专用 NequIP 势函数", elements_set.iter().cloned().collect::<Vec<_>>().join(",")),
        needs_training: true,
        estimated_training_minutes: Some(training_time),
        estimated_vram_gb: Some(16.0),
        alternatives: vec![
            AlternativePotential { potential_type: "mace".to_string(), reason: "MACE 训练速度更快".to_string() },
            AlternativePotential { potential_type: "nep".to_string(), reason: "NEP 模型更小，推理更快".to_string() },
        ],
    }
}

/// Mock GPU 状态
fn get_mock_gpu_status() -> GpuResourceStatus {
    GpuResourceStatus {
        gpus: vec![GpuInfo {
            name: "Mock GPU (A100 40GB)".to_string(),
            total_memory_gb: 40.0,
            available_memory_gb: 38.5,
            is_available: false, // Mock 模式下标记为不可用
            cuda_version: Some("12.1".to_string()),
        }],
        total_vram_gb: 40.0,
        used_vram_gb: 1.5,
        running_tasks: 0,
        queued_tasks: 0,
    }
}

/// Mock 训练超参数推荐
fn recommend_hyperparams(req: &HyperparamRecommendationRequest) -> HyperparamRecommendation {
    let cutoff = match req.num_species {
        0..=2 => 5.0,
        3..=5 => 6.0,
        _ => 7.0,
    };

    let max_neuron = match req.num_species {
        0..=2 => 128,
        3..=5 => 256,
        _ => 256,
    };

    let epochs = (req.num_training_structures as f64 * 50.0).min(5000.0) as u32;
    let batch_size = match req.num_atoms {
        0..=1000 => 16,
        1001..=10000 => 8,
        _ => 4,
    };

    let estimated_vram = match req.num_atoms {
        0..=5000 => 8.0,
        5001..=20000 => 16.0,
        _ => 40.0,
    };

    let estimated_time = match req.potential_type.as_str() {
        "nequip" => req.num_training_structures as u32 * epochs / 5000,
        "mace" => req.num_training_structures as u32 * epochs / 8000,
        "nep" => req.num_training_structures as u32 * epochs / 10000,
        _ => req.num_training_structures as u32 * epochs / 6000,
    };

    HyperparamRecommendation {
        cutoff,
        max_neuron,
        epochs: epochs.max(100),
        batch_size,
        learning_rate: 0.001,
        energy_weight: 1.0,
        force_weight: 10.0,
        estimated_training_time_minutes: estimated_time.max(5),
        estimated_vram_gb: estimated_vram,
        reasoning: format!(
            "基于 {} 种元素、{} 个原子、{} 条训练数据推荐。cutoff={}Å, {} 神经元, {} epochs",
            req.num_species, req.num_atoms, req.num_training_structures, cutoff, max_neuron, epochs
        ),
    }
}

/// Mock 训练结果
fn mock_train_result(config: &TrainingConfig) -> TrainingResult {
    let epochs = config.epochs;
    let mut loss_history = Vec::new();
    for i in 0..epochs {
        let progress = i as f64 / epochs as f64;
        loss_history.push(LossRecord {
            epoch: i,
            energy_rmse: 50.0 * (1.0 - progress * 0.85) + rand_factor() * 2.0,
            force_rmse: 0.3 * (1.0 - progress * 0.8) + rand_factor() * 0.01,
            val_energy_rmse: Some(55.0 * (1.0 - progress * 0.83) + rand_factor() * 3.0),
            val_force_rmse: Some(0.35 * (1.0 - progress * 0.78) + rand_factor() * 0.015),
        });
    }

    let last = loss_history.last().unwrap();
    TrainingResult {
        potential_name: format!("{}_custom_{}", config.potential_type, chrono::Utc::now().format("%Y%m%d%H%M%S")),
        potential_type: config.potential_type.clone(),
        success: true,
        final_energy_rmse: last.energy_rmse,
        final_force_rmse: last.force_rmse,
        best_epoch: epochs - epochs / 10,
        model_path: format!("models/custom_{}.pt", config.potential_type),
        training_time_sec: epochs as f64 * 2.5,
        loss_history,
    }
}

/// Mock 验证结果
fn mock_validate(req: &ValidationRequest) -> ValidationResult {
    let num_structures = 50u32;
    let mut details = Vec::new();
    let mut total_energy_err = 0.0;
    let mut total_force_err = 0.0;

    for i in 0..num_structures {
        let e_err = 2.0 + rand_factor() * 3.0;
        let f_err = 0.03 + rand_factor() * 0.04;
        total_energy_err += e_err;
        total_force_err += f_err;
        details.push(StructureValidationDetail {
            structure_id: format!("struct_{}", i),
            dft_energy: -100.0 - i as f64 * 0.5,
            ml_energy: -100.0 - i as f64 * 0.5 + e_err,
            energy_error: e_err,
            max_force_error: f_err,
        });
    }

    let avg_energy = total_energy_err / num_structures as f64;
    let avg_force = total_force_err / num_structures as f64;

    ValidationResult {
        potential_name: req.potential_name.clone(),
        energy_rmse: avg_energy,
        force_rmse: avg_force,
        energy_mae: avg_energy * 0.8,
        force_mae: avg_force * 0.8,
        num_test_structures: num_structures,
        passed: avg_energy < 5.0 && avg_force < 0.1,
        details,
    }
}

fn rand_factor() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().subsec_nanos();
    (nanos % 1000) as f64 / 1000.0
}

// ============================================================================
// Tauri 命令
// ============================================================================

/// 列出可用的 ML 势函数
#[tauri::command]
pub async fn list_ml_potentials() -> Result<Vec<MLPotentialInfo>, String> {
    Ok(get_mock_potentials())
}

/// ML 势函数推理
#[tauri::command]
pub async fn compute_ml_potential(
    request: MLPotentialComputeRequest,
) -> Result<MLPotentialComputeResult, String> {
    let start = std::time::Instant::now();
    let mut result = mock_potential_compute(&request);
    result.inference_time_ms = start.elapsed().as_millis() as u64;
    Ok(result)
}

/// 势函数自动选择
#[tauri::command]
pub async fn auto_select_potential(
    request: PotentialSelectionRequest,
) -> Result<PotentialSelectionResult, String> {
    Ok(select_potential_impl(&request))
}

/// 获取 GPU 资源状态
#[tauri::command]
pub async fn get_gpu_status() -> Result<GpuResourceStatus, String> {
    Ok(get_mock_gpu_status())
}

/// 推荐训练超参数
#[tauri::command]
pub async fn recommend_training_hyperparams(
    request: HyperparamRecommendationRequest,
) -> Result<HyperparamRecommendation, String> {
    Ok(recommend_hyperparams(&request))
}

/// 提交训练任务
#[tauri::command]
pub async fn submit_training_job(
    config: TrainingConfig,
) -> Result<TrainingResult, String> {
    // Mock: 直接返回训练结果
    Ok(mock_train_result(&config))
}

/// 验证势函数质量
#[tauri::command]
pub async fn validate_ml_potential(
    request: ValidationRequest,
) -> Result<ValidationResult, String> {
    Ok(mock_validate(&request))
}

/// 使用 ML 势运行 MD 模拟
#[tauri::command]
pub async fn run_md_with_ml_potential(
    potential_name: String,
    positions: Vec<Vec<f64>>,
    atom_types: Vec<String>,
    cell: Option<Vec<Vec<f64>>>,
    ensemble: String,
    temperature: Option<f64>,
    num_steps: u32,
    timestep_fs: f64,
) -> Result<serde_json::Value, String> {
    let num_atoms = atom_types.len();

    // Mock MD 结果
    let initial_energy = num_atoms as f64 * (-6.5);
    let result = serde_json::json!({
        "success": true,
        "potential_name": potential_name,
        "ensemble": ensemble,
        "num_atoms": num_atoms,
        "num_steps": num_steps,
        "timestep_fs": timestep_fs,
        "temperature": temperature.unwrap_or(300.0),
        "initial_energy_eV": initial_energy,
        "final_energy_eV": initial_energy + rand_factor() * 0.1,
        "energy_drift_eV": rand_factor() * 0.05,
        "total_time_ps": num_steps as f64 * timestep_fs / 1000.0,
        "is_mock": true,
        "message": "Mock MD simulation completed. Connect LAMMPS + pair_style ml for real simulation."
    });

    Ok(result)
}
