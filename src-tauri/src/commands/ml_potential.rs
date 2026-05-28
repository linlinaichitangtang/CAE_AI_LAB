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
use std::io::Write;
use std::path::Path;
use std::process::Command;

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
    /// V4.2-002: 经典势函数类型
    EAM,
    MEAM,
    ReaxFF,
    LJ,
    AIREBO,
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
            "eam" => Some(Self::EAM),
            "meam" => Some(Self::MEAM),
            "reaxff" => Some(Self::ReaxFF),
            "lj" => Some(Self::LJ),
            "airebo" => Some(Self::AIREBO),
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
            Self::EAM => "eam",
            Self::MEAM => "meam",
            Self::ReaxFF => "reaxff",
            Self::LJ => "lj",
            Self::AIREBO => "airebo",
        }
    }

    /// 是否为通用预训练势（无需训练，直接使用）
    pub fn is_pretrained(&self) -> bool {
        matches!(self, Self::CHGNet | Self::MACE | Self::SevenNet)
    }

    /// V4.2-002: 是否为经典势函数（通过参数文件定义，非 ML 训练得到）
    pub fn is_classical(&self) -> bool {
        matches!(self, Self::EAM | Self::MEAM | Self::ReaxFF | Self::LJ | Self::AIREBO)
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
// V4.2-002: LAMMPS 真实调用数据结构
// ============================================================================

/// LAMMPS 输入配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LammpsInputConfig {
    /// 势函数类型: "eam", "meam", "reaxff", "lj", "airebo", "ml"
    pub potential_type: String,
    /// 势函数文件路径（eam/meam/reaxff 的参数文件，或 ML 模型文件）
    pub potential_file: Option<String>,
    /// 附加势函数文件（如 meam 的 library 文件）
    pub potential_file_extra: Option<String>,
    /// 系综: "nve", "nvt", "npt"
    pub ensemble: String,
    /// 温度 (K)
    pub temperature: f64,
    /// 压强 (bar, 仅 npt)
    pub pressure: Option<f64>,
    /// 时间步长 (fs)
    pub timestep_fs: f64,
    /// 模拟步数
    pub num_steps: u32,
    /// 原子类型列表: ["Fe", "C", ...]
    pub atom_types: Vec<String>,
    /// 原子位置 (Å): [[x, y, z], ...]
    pub positions: Vec<Vec<f64>>,
    /// 晶胞向量 (3x3, Å)
    pub cell: Vec<Vec<f64>>,
    /// thermo 输出频率 (步)
    pub thermo_freq: u32,
    /// dump 输出频率 (步)
    pub dump_freq: u32,
    /// 工作目录
    pub work_dir: String,
    /// 随机数种子
    pub random_seed: u64,
    /// LJ 参数 (epsilon, sigma, cutoff) — 仅 lj 类型使用
    pub lj_params: Option<Vec<Vec<f64>>>,
    /// 元素到 LAMMPS atom type 的映射
    pub element_type_map: Option<HashMap<String, u32>>,
}

/// LAMMPS 热力学输出（单行 thermo 数据）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LammpsThermoData {
    pub step: u32,
    pub temp: f64,
    pub press: f64,
    pub pe: f64,
    pub ke: f64,
    pub etotal: f64,
    pub density: f64,
    pub volume: f64,
}

/// Dump 原子数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DumpAtom {
    pub id: u32,
    pub atom_type: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub fx: Option<f64>,
    pub fy: Option<f64>,
    pub fz: Option<f64>,
}

/// LAMMPS Dump 数据（单帧快照）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LammpsDumpData {
    pub timestep: u32,
    pub num_atoms: u32,
    pub box_bounds: Vec<(f64, f64)>,
    pub atoms: Vec<DumpAtom>,
}

/// LAMMPS 模拟结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LammpsSimulationResult {
    pub success: bool,
    pub potential_type: String,
    pub thermo_data: Vec<LammpsThermoData>,
    pub dump_data: Vec<LammpsDumpData>,
    pub total_time_sec: f64,
    pub num_atoms: u32,
    pub exit_code: Option<i32>,
    pub stderr: String,
    pub is_real: bool,
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

/// V4.2-002: 经典势函数列表 (EAM/MEAM/ReaxFF/LJ/AIREBO)
fn get_classical_potentials() -> Vec<MLPotentialInfo> {
    vec![
        MLPotentialInfo {
            name: "EAM-Fe".to_string(),
            potential_type: "eam".to_string(),
            description: "Embedded Atom Method — Fe 专用势函数 (Mendelev 2003)".to_string(),
            supported_elements: vec!["Fe".into()],
            energy_rmse: Some(0.5),
            force_rmse: Some(0.02),
            cutoff: 5.5,
            model_path: Some("potentials/Fe_mm.eam.fs".to_string()),
            training_data_size: None,
            is_ready: true,
            training_time_sec: None,
            created_at: Some("2025-12-01T00:00:00Z".to_string()),
        },
        MLPotentialInfo {
            name: "EAM-Ni".to_string(),
            potential_type: "eam".to_string(),
            description: "Embedded Atom Method — Ni 专用势函数 (Foiles 1986)".to_string(),
            supported_elements: vec!["Ni".into()],
            energy_rmse: Some(0.8),
            force_rmse: Some(0.03),
            cutoff: 5.5,
            model_path: Some("potentials/Ni.eam".to_string()),
            training_data_size: None,
            is_ready: true,
            training_time_sec: None,
            created_at: Some("2025-12-01T00:00:00Z".to_string()),
        },
        MLPotentialInfo {
            name: "EAM-Cu".to_string(),
            potential_type: "eam".to_string(),
            description: "Embedded Atom Method — Cu 专用势函数 (Mishin 2001)".to_string(),
            supported_elements: vec!["Cu".into()],
            energy_rmse: Some(0.6),
            force_rmse: Some(0.025),
            cutoff: 5.5,
            model_path: Some("potentials/Cu.eam".to_string()),
            training_data_size: None,
            is_ready: true,
            training_time_sec: None,
            created_at: Some("2025-12-01T00:00:00Z".to_string()),
        },
        MLPotentialInfo {
            name: "EAM-Al".to_string(),
            potential_type: "eam".to_string(),
            description: "Embedded Atom Method — Al 专用势函数 (Mishin 1999)".to_string(),
            supported_elements: vec!["Al".into()],
            energy_rmse: Some(0.7),
            force_rmse: Some(0.03),
            cutoff: 5.5,
            model_path: Some("potentials/Al.eam".to_string()),
            training_data_size: None,
            is_ready: true,
            training_time_sec: None,
            created_at: Some("2025-12-01T00:00:00Z".to_string()),
        },
        MLPotentialInfo {
            name: "EAM-W".to_string(),
            potential_type: "eam".to_string(),
            description: "Embedded Atom Method — W 专用势函数 (Zhou 2004)".to_string(),
            supported_elements: vec!["W".into()],
            energy_rmse: Some(0.9),
            force_rmse: Some(0.04),
            cutoff: 5.5,
            model_path: Some("potentials/W.eam".to_string()),
            training_data_size: None,
            is_ready: true,
            training_time_sec: None,
            created_at: Some("2025-12-01T00:00:00Z".to_string()),
        },
        MLPotentialInfo {
            name: "MEAM-SiC".to_string(),
            potential_type: "meam".to_string(),
            description: "Modified Embedded Atom Method — Si-C 二元素体系 (Lenosky 2000)".to_string(),
            supported_elements: vec!["Si".into(), "C".into()],
            energy_rmse: Some(1.2),
            force_rmse: Some(0.05),
            cutoff: 5.0,
            model_path: Some("potentials/library.meam".to_string()),
            training_data_size: None,
            is_ready: true,
            training_time_sec: None,
            created_at: Some("2025-12-01T00:00:00Z".to_string()),
        },
        MLPotentialInfo {
            name: "ReaxFF-CHO".to_string(),
            potential_type: "reaxff".to_string(),
            description: "Reactive Force Field — C-H-O 燃烧体系 (Chenoweth 2008)".to_string(),
            supported_elements: vec!["C".into(), "H".into(), "O".into()],
            energy_rmse: Some(3.0),
            force_rmse: Some(0.1),
            cutoff: 10.0,
            model_path: Some("potentials/ffield.reaxff.CHO".to_string()),
            training_data_size: None,
            is_ready: true,
            training_time_sec: None,
            created_at: Some("2025-12-01T00:00:00Z".to_string()),
        },
        MLPotentialInfo {
            name: "ReaxFF-SiOH".to_string(),
            potential_type: "reaxff".to_string(),
            description: "Reactive Force Field — Si-O-H 水合硅酸盐体系 (van Duin 2003)".to_string(),
            supported_elements: vec!["Si".into(), "O".into(), "H".into()],
            energy_rmse: Some(2.5),
            force_rmse: Some(0.08),
            cutoff: 10.0,
            model_path: Some("potentials/ffield.reaxff.SiOH".to_string()),
            training_data_size: None,
            is_ready: true,
            training_time_sec: None,
            created_at: Some("2025-12-01T00:00:00Z".to_string()),
        },
        MLPotentialInfo {
            name: "LJ-Argon".to_string(),
            potential_type: "lj".to_string(),
            description: "Lennard-Jones 势函数 — Ar 标准参数 (epsilon=0.0104 eV, sigma=3.405 A)".to_string(),
            supported_elements: vec!["Ar".into()],
            energy_rmse: None,
            force_rmse: None,
            cutoff: 10.0,
            model_path: None,
            training_data_size: None,
            is_ready: true,
            training_time_sec: None,
            created_at: Some("2025-12-01T00:00:00Z".to_string()),
        },
        MLPotentialInfo {
            name: "AIREBO-Carbon".to_string(),
            potential_type: "airebo".to_string(),
            description: "Adaptive Intermolecular Reactive Empirical Bond Order — 碳纳米管/石墨烯体系 (Stuart 2000)".to_string(),
            supported_elements: vec!["C".into(), "H".into()],
            energy_rmse: Some(1.5),
            force_rmse: Some(0.06),
            cutoff: 2.0,
            model_path: Some("potentials/CH.airebo".to_string()),
            training_data_size: None,
            is_ready: true,
            training_time_sec: None,
            created_at: Some("2025-12-01T00:00:00Z".to_string()),
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
// V4.2-002: LAMMPS 真实调用辅助函数
// ============================================================================

/// 检测 LAMMPS 是否可用
fn lammps_available() -> bool {
    for name in &["lmp", "lammps", "lmp_serial", "lmp_mpi"] {
        if Command::new("which")
            .arg(name)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return true;
        }
    }
    false
}

/// 构建元素到 LAMMPS atom type 的映射
fn build_element_type_map(atom_types: &[String]) -> (HashMap<String, u32>, u32) {
    let mut map = HashMap::new();
    let mut next_id = 1u32;
    for atom in atom_types {
        if !map.contains_key(atom) {
            map.insert(atom.clone(), next_id);
            next_id += 1;
        }
    }
    let num_types = next_id - 1;
    (map, num_types)
}

/// 生成 LAMMPS data 文件内容
fn generate_lammps_data(config: &LammpsInputConfig) -> String {
    let (elem_map, num_types) = match &config.element_type_map {
        Some(m) => (m.clone(), m.values().copied().max().unwrap_or(1)),
        None => {
            let (m, n) = build_element_type_map(&config.atom_types);
            (m, n)
        }
    };

    let num_atoms = config.positions.len();
    let a = &config.cell[0];
    let b = &config.cell[1];
    let c = &config.cell[2];

    let mut data = String::new();
    data.push_str("# LAMMPS data file generated by SOLO V4.2-002\n\n");
    data.push_str(&format!("{} atoms\n", num_atoms));
    data.push_str(&format!("{} atom types\n\n", num_types));

    // 盒边界
    let xlo = 0.0;
    let ylo = 0.0;
    let zlo = 0.0;
    let xhi = a[0];
    let yhi = b[1];
    let zhi = c[2];

    // 对非正交盒做 tilt
    let xy = a[1];
    let xz = a[2];
    let yz = b[2];

    data.push_str(&format!("{:.6} {:.6} xlo xhi\n", xlo, xhi));
    data.push_str(&format!("{:.6} {:.6} ylo yhi\n", ylo, yhi));
    data.push_str(&format!("{:.6} {:.6} zlo zhi\n", zlo, zhi));

    if xy.abs() > 1e-12 || xz.abs() > 1e-12 || yz.abs() > 1e-12 {
        data.push_str(&format!("{:.6} {:.6} {:.6} xy xz yz\n", xy, xz, yz));
    }

    // Masses section (estimate from element)
    data.push_str("\nMasses\n\n");
    let mut sorted_elements: Vec<&String> = elem_map.keys().collect();
    sorted_elements.sort_by_key(|k| elem_map.get(*k).unwrap());
    for elem in &sorted_elements {
        let mass = estimate_atomic_mass(elem);
        data.push_str(&format!("  {}  {:.3}\n", elem_map.get(*elem).unwrap(), mass));
    }

    // Atoms section
    data.push_str("\nAtoms # atomic\n\n");
    for (i, pos) in config.positions.iter().enumerate() {
        let elem = &config.atom_types[i];
        let atom_type = elem_map.get(elem).copied().unwrap_or(1);
        data.push_str(&format!(
            "{} {} {:.6} {:.6} {:.6}\n",
            i + 1,
            atom_type,
            pos[0],
            pos[1],
            pos[2]
        ));
    }

    data
}

/// 估算常见元素的原子质量 (g/mol)
fn estimate_atomic_mass(elem: &str) -> f64 {
    match elem {
        "H" => 1.008, "He" => 4.003, "Li" => 6.941, "Be" => 9.012,
        "B" => 10.811, "C" => 12.011, "N" => 14.007, "O" => 15.999,
        "F" => 18.998, "Ne" => 20.180, "Na" => 22.990, "Mg" => 24.305,
        "Al" => 26.982, "Si" => 28.086, "P" => 30.974, "S" => 32.065,
        "Cl" => 35.453, "Ar" => 39.948, "K" => 39.098, "Ca" => 40.078,
        "Sc" => 44.956, "Ti" => 47.867, "V" => 50.942, "Cr" => 51.996,
        "Mn" => 54.938, "Fe" => 55.845, "Co" => 58.933, "Ni" => 58.693,
        "Cu" => 63.546, "Zn" => 65.380, "Ga" => 69.723, "Ge" => 72.630,
        "As" => 74.922, "Se" => 78.960, "Br" => 79.904, "Kr" => 83.798,
        "Rb" => 85.468, "Sr" => 87.620, "Y" => 88.906, "Zr" => 91.224,
        "Nb" => 92.906, "Mo" => 95.950, "Tc" => 98.0, "Ru" => 101.070,
        "Rh" => 102.906, "Pd" => 106.420, "Ag" => 107.868, "Cd" => 112.411,
        "In" => 114.818, "Sn" => 118.710, "Sb" => 121.760, "Te" => 127.600,
        "I" => 126.904, "Xe" => 131.293, "Cs" => 132.905, "Ba" => 137.327,
        "La" => 138.905, "Hf" => 178.490, "Ta" => 180.948, "W" => 183.840,
        "Re" => 186.207, "Os" => 190.230, "Ir" => 192.217, "Pt" => 195.078,
        "Au" => 196.967, "Hg" => 200.592, "Tl" => 204.383, "Pb" => 207.200,
        "Bi" => 208.980,
        _ => 12.011, // fallback to carbon
    }
}

/// 生成 LAMMPS 输入脚本
fn generate_lammps_script(config: &LammpsInputConfig) -> String {
    let mut script = String::new();
    script.push_str("# LAMMPS input script generated by SOLO V4.2-002\n\n");

    // 基本设置
    script.push_str("units        metal\n");
    script.push_str("atom_style   atomic\n");
    script.push_str("boundary     p p p\n");
    script.push_str("\n");

    // 读取数据文件
    script.push_str("read_data    data.lammps\n\n");

    // 势函数设置 — 根据类型选择 pair_style
    let pt_lower = config.potential_type.to_lowercase();
    match pt_lower.as_str() {
        "eam" => {
            script.push_str("pair_style   eam\n");
            if let Some(ref pf) = config.potential_file {
                let fname = Path::new(pf).file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| pf.clone());
                script.push_str(&format!("pair_coeff   * * {}\n", fname));
            } else {
                script.push_str("pair_coeff   * * Fe_mm.eam.fs Fe\n");
            }
        }
        "meam" => {
            script.push_str("pair_style   meam\n");
            if let Some(ref pf) = config.potential_file {
                let fname = Path::new(pf).file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| pf.clone());
                script.push_str(&format!("pair_coeff   * * {} ", fname));
                if let Some(ref pf2) = config.potential_file_extra {
                    let fname2 = Path::new(pf2).file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| pf2.clone());
                    script.push_str(&format!("{} ", fname2));
                }
                // 列出所有元素
                for elem in unique_sorted_elements(&config.atom_types) {
                    script.push_str(&format!("{} ", elem));
                }
                script.push('\n');
            } else {
                script.push_str("pair_coeff   * * library.meam SiC.meam C Si\n");
            }
        }
        "reaxff" => {
            script.push_str("pair_style   reaxff NULL\n");
            if let Some(ref pf) = config.potential_file {
                let fname = Path::new(pf).file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| pf.clone());
                script.push_str(&format!("pair_coeff   * * {} ", fname));
            } else {
                script.push_str("pair_coeff   * * ffield.reaxff ");
            }
            for elem in unique_sorted_elements(&config.atom_types) {
                script.push_str(&format!("{} ", elem));
            }
            script.push('\n');
        }
        "lj" => {
            let cutoff = config
                .lj_params
                .as_ref()
                .and_then(|v| v.first())
                .and_then(|p| if p.len() >= 3 { Some(p[2]) } else { None })
                .unwrap_or(10.0);
            script.push_str(&format!("pair_style   lj/cut {:.3}\n", cutoff));

            if let Some(ref lj_params) = config.lj_params {
                // lj_params 是 Vec<Vec<f64>>，每行 [epsilon, sigma, cutoff]
                for (i, params) in lj_params.iter().enumerate() {
                    let eps = params.get(0).copied().unwrap_or(1.0);
                    let sig = params.get(1).copied().unwrap_or(1.0);
                    let cut = params.get(2).copied().unwrap_or(cutoff);
                    script.push_str(&format!(
                        "pair_coeff   {} {} {:.6} {:.6} {:.3}\n",
                        i + 1, i + 1, eps, sig, cut
                    ));
                }
            } else {
                script.push_str("pair_coeff   * * 1.0 1.0 10.0\n");
            }
        }
        "airebo" => {
            script.push_str("pair_style   airebo 2.0 0 0\n");
            script.push_str("pair_coeff   * * CH.airebo C\n");
        }
        "ml" => {
            script.push_str("pair_style   ml\n");
            if let Some(ref pf) = config.potential_file {
                let fname = Path::new(pf).file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| pf.clone());
                script.push_str(&format!("pair_coeff   * * {}\n", fname));
            } else {
                script.push_str("pair_coeff   * * model.pt\n");
            }
        }
        _ => {
            script.push_str("pair_style   lj/cut 10.0\n");
            script.push_str("pair_coeff   * * 1.0 1.0 10.0\n");
        }
    }
    script.push('\n');

    // 速度初始化
    script.push_str(&format!(
        "velocity     all create {:.2} {} dist gaussian\n\n",
        config.temperature, config.random_seed
    ));

    // 系综 fix
    let ensemble_lower = config.ensemble.to_lowercase();
    let tau = (100.0 * config.timestep_fs).max(10.0);
    match ensemble_lower.as_str() {
        "nve" => {
            script.push_str("fix          1 all nve\n");
        }
        "nvt" => {
            script.push_str(&format!(
                "fix          1 all nvt temp {:.2} {:.2} {:.1}\n",
                config.temperature, config.temperature, tau
            ));
        }
        "npt" => {
            let pressure = config.pressure.unwrap_or(0.0);
            let tau_p = tau * 10.0;
            script.push_str(&format!(
                "fix          1 all npt temp {:.2} {:.2} {:.1} iso {:.2} {:.2} {:.1}\n",
                config.temperature, config.temperature, tau,
                pressure, pressure, tau_p
            ));
        }
        _ => {
            script.push_str("fix          1 all nve\n");
        }
    }

    // 时间步长
    script.push_str(&format!("\ntimestep     {:.6}\n", config.timestep_fs * 0.001)); // fs -> ps
    script.push_str(&format!("thermo       {}\n", config.thermo_freq));
    script.push_str("thermo_style custom step temp press pe ke etotal density vol\n\n");

    // Dump
    script.push_str(&format!(
        "dump         1 all custom {} dump.lammps id type x y z fx fy fz\n",
        config.dump_freq
    ));
    script.push_str("dump_modify  1 sort id\n\n");

    // 运行
    script.push_str(&format!("run          {}\n", config.num_steps));

    script
}

/// 返回去重排序后的元素列表
fn unique_sorted_elements(atom_types: &[String]) -> Vec<String> {
    let mut elems: Vec<String> = atom_types.iter().cloned().collect();
    elems.sort();
    elems.dedup();
    elems
}

/// 生成 LAMMPS input script 和 data 文件内容
/// 返回 (input_script, data_file)
pub fn generate_lammps_input(config: &LammpsInputConfig) -> (String, String) {
    let input_script = generate_lammps_script(config);
    let data_file = generate_lammps_data(config);
    (input_script, data_file)
}

/// 解析 LAMMPS log 文件中的 thermo 数据
pub fn parse_lammps_log(log_path: &str) -> Result<Vec<LammpsThermoData>, String> {
    let content = std::fs::read_to_string(log_path)
        .map_err(|e| format!("无法读取 log 文件 {}: {}", log_path, e))?;

    let mut thermo_data = Vec::new();
    let mut in_thermo = false;
    let mut col_map: Vec<usize> = Vec::new();
    // 列索引: 0=Step, 1=Temp, 2=Press, 3=PotEng, 4=KinEng, 5=TotEng, 6=Density, 7=Volume

    for line in content.lines() {
        let trimmed = line.trim();

        // 跳过空行和注释
        if trimmed.is_empty() {
            continue;
        }

        // 检测 thermo header
        if trimmed.starts_with("Step ") || trimmed.to_lowercase().starts_with("step ") {
            let headers: Vec<&str> = trimmed.split_whitespace().collect();
            col_map.clear();
            col_map.resize(8, usize::MAX);
            for (i, h) in headers.iter().enumerate() {
                let h_lower = h.to_lowercase();
                match h_lower.as_str() {
                    "step" => col_map[0] = i,
                    "temp" => col_map[1] = i,
                    "press" => col_map[2] = i,
                    "poteng" | "pe" => col_map[3] = i,
                    "kineng" | "ke" => col_map[4] = i,
                    "toteng" | "etotal" => col_map[5] = i,
                    "density" => col_map[6] = i,
                    "volume" | "vol" => col_map[7] = i,
                    _ => {}
                }
            }
            in_thermo = true;
            continue;
        }

        if !in_thermo {
            continue;
        }

        // 尝试解析数据行
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        // 检查是否是数字（数据行以数字开头）
        if parts[0].parse::<f64>().is_err() {
            continue;
        }

        // 如果是 "Loop time" 之类，停止
        if trimmed.starts_with("Loop time") || trimmed.starts_with("Performance:") {
            break;
        }

        let get_col = |idx: usize| -> Option<f64> {
            if idx < col_map.len() && col_map[idx] < parts.len() {
                parts[col_map[idx]].parse::<f64>().ok()
            } else {
                None
            }
        };

        thermo_data.push(LammpsThermoData {
            step: get_col(0).map(|v| v as u32).unwrap_or(0),
            temp: get_col(1).unwrap_or(0.0),
            press: get_col(2).unwrap_or(0.0),
            pe: get_col(3).unwrap_or(0.0),
            ke: get_col(4).unwrap_or(0.0),
            etotal: get_col(5).unwrap_or(0.0),
            density: get_col(6).unwrap_or(0.0),
            volume: get_col(7).unwrap_or(0.0),
        });
    }

    if thermo_data.is_empty() {
        Err(format!("log 文件 {} 中未找到 thermo 数据", log_path))
    } else {
        Ok(thermo_data)
    }
}

/// 解析 LAMMPS dump 文件
pub fn parse_lammps_dump_file(dump_path: &str) -> Result<Vec<LammpsDumpData>, String> {
    let content = std::fs::read_to_string(dump_path)
        .map_err(|e| format!("无法读取 dump 文件 {}: {}", dump_path, e))?;

    let mut dump_frames: Vec<LammpsDumpData> = Vec::new();
    let mut lines = content.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed != "ITEM: TIMESTEP" {
            continue;
        }

        // 解析 timestep
        let timestep = lines
            .next()
            .ok_or("dump 文件格式错误: 缺少 timestep")?
            .trim()
            .parse::<u32>()
            .map_err(|e| format!("dump 文件格式错误: {}", e))?;

        // ITEM: NUMBER OF ATOMS
        let _ = lines.next();
        let num_atoms = lines
            .next()
            .ok_or("dump 文件格式错误: 缺少原子数")?
            .trim()
            .parse::<u32>()
            .map_err(|e| format!("dump 文件格式错误: {}", e))?;

        // ITEM: BOX BOUNDS
        let _ = lines.next();
        let mut box_bounds: Vec<(f64, f64)> = Vec::new();
        for _ in 0..3 {
            let bline = lines.next().ok_or("dump 文件格式错误: 缺少盒边界")?;
            let parts: Vec<&str> = bline.trim().split_whitespace().collect();
            if parts.len() >= 2 {
                let lo = parts[0].parse::<f64>().unwrap_or(0.0);
                let hi = parts[1].parse::<f64>().unwrap_or(0.0);
                box_bounds.push((lo, hi));
            }
        }

        // ITEM: ATOMS
        let atom_header_line = lines
            .next()
            .ok_or("dump 文件格式错误: 缺少 ITEM: ATOMS")?;
        let atom_headers: Vec<&str> = atom_header_line
            .trim()
            .split_whitespace()
            .skip(2) // 跳过 "ITEM:" 和 "ATOMS"
            .collect();

        // 找到各列的索引
        let id_col = atom_headers.iter().position(|&h| h == "id");
        let type_col = atom_headers.iter().position(|&h| h == "type");
        let x_col = atom_headers.iter().position(|&h| h == "x");
        let y_col = atom_headers.iter().position(|&h| h == "y");
        let z_col = atom_headers.iter().position(|&h| h == "z");
        let fx_col = atom_headers.iter().position(|&h| h == "fx");
        let fy_col = atom_headers.iter().position(|&h| h == "fy");
        let fz_col = atom_headers.iter().position(|&h| h == "fz");

        let mut atoms = Vec::new();
        for _ in 0..num_atoms {
            let aline = lines.next().ok_or("dump 文件格式错误: 原子数据不完整")?;
            let parts: Vec<&str> = aline.trim().split_whitespace().collect();

            let get_col = |col: Option<usize>| -> Option<f64> {
                col.and_then(|c| parts.get(c))
                    .and_then(|v| v.parse::<f64>().ok())
            };

            atoms.push(DumpAtom {
                id: id_col
                    .and_then(|c| parts.get(c))
                    .and_then(|v| v.parse::<u32>().ok())
                    .unwrap_or(0),
                atom_type: type_col
                    .and_then(|c| parts.get(c))
                    .and_then(|v| v.parse::<u32>().ok())
                    .unwrap_or(0),
                x: x_col.and_then(|c| parts.get(c)).and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0),
                y: y_col.and_then(|c| parts.get(c)).and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0),
                z: z_col.and_then(|c| parts.get(c)).and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0),
                fx: get_col(fx_col),
                fy: get_col(fy_col),
                fz: get_col(fz_col),
            });
        }

        dump_frames.push(LammpsDumpData {
            timestep,
            num_atoms,
            box_bounds,
            atoms,
        });
    }

    if dump_frames.is_empty() {
        Err(format!("dump 文件 {} 中未找到数据帧", dump_path))
    } else {
        Ok(dump_frames)
    }
}

/// 执行真实 LAMMPS 模拟
fn run_lammps_real(config: &LammpsInputConfig) -> Result<LammpsSimulationResult, String> {
    let work_dir = Path::new(&config.work_dir);

    // 创建工作目录
    std::fs::create_dir_all(work_dir)
        .map_err(|e| format!("无法创建工作目录 {:?}: {}", work_dir, e))?;

    // 生成输入文件
    let (input_script, data_file) = generate_lammps_input(config);

    // 写入 data 文件
    let data_path = work_dir.join("data.lammps");
    let mut data_f = std::fs::File::create(&data_path)
        .map_err(|e| format!("无法创建 data 文件: {}", e))?;
    data_f
        .write_all(data_file.as_bytes())
        .map_err(|e| format!("无法写入 data 文件: {}", e))?;

    // 写入 input 脚本
    let input_path = work_dir.join("in.lammps");
    let mut input_f = std::fs::File::create(&input_path)
        .map_err(|e| format!("无法创建 input 文件: {}", e))?;
    input_f
        .write_all(input_script.as_bytes())
        .map_err(|e| format!("无法写入 input 文件: {}", e))?;

    // 寻找 LAMMPS 可执行文件
    let lmp_bin = find_lammps_binary().ok_or("未找到 LAMMPS 可执行文件 (已尝试 lmp, lammps, lmp_serial, lmp_mpi)")?;

    let start_time = std::time::Instant::now();

    // 执行 LAMMPS
    let output = Command::new(&lmp_bin)
        .arg("-in")
        .arg("in.lammps")
        .current_dir(work_dir)
        .output()
        .map_err(|e| format!("执行 LAMMPS 失败: {}", e))?;

    let elapsed = start_time.elapsed().as_secs_f64();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        return Ok(LammpsSimulationResult {
            success: false,
            potential_type: config.potential_type.clone(),
            thermo_data: vec![],
            dump_data: vec![],
            total_time_sec: elapsed,
            num_atoms: config.positions.len() as u32,
            exit_code: output.status.code(),
            stderr,
            is_real: true,
        });
    }

    // 解析 log.lammps（thermo 输出）
    let log_path = work_dir.join("log.lammps");
    let thermo_data = if log_path.exists() {
        parse_lammps_log(log_path.to_str().unwrap_or("")).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    // 解析 dump 文件
    let dump_path = work_dir.join("dump.lammps");
    let dump_data = if dump_path.exists() {
        parse_lammps_dump_file(dump_path.to_str().unwrap_or("")).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    Ok(LammpsSimulationResult {
        success: true,
        potential_type: config.potential_type.clone(),
        thermo_data,
        dump_data,
        total_time_sec: elapsed,
        num_atoms: config.positions.len() as u32,
        exit_code: Some(0),
        stderr,
        is_real: true,
    })
}

/// 查找 LAMMPS 可执行文件
fn find_lammps_binary() -> Option<String> {
    for name in &["lmp", "lammps", "lmp_serial", "lmp_mpi"] {
        if let Ok(output) = Command::new("which").arg(name).output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}

/// 检测真实 GPU — 先尝试 nvidia-smi，再尝试 rocm-smi
fn detect_real_gpu() -> Option<Vec<GpuInfo>> {
    // 尝试 nvidia-smi
    if let Ok(output) = Command::new("nvidia-smi")
        .args(&[
            "--query-gpu=name,memory.total,memory.free",
            "--format=csv,noheader,nounits",
        ])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut gpus = Vec::new();
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                if parts.len() >= 3 {
                    let total_mb = parts[1].parse::<f64>().unwrap_or(0.0);
                    let free_mb = parts[2].parse::<f64>().unwrap_or(0.0);
                    gpus.push(GpuInfo {
                        name: parts[0].to_string(),
                        total_memory_gb: total_mb / 1024.0,
                        available_memory_gb: free_mb / 1024.0,
                        is_available: true,
                        cuda_version: detect_cuda_version(),
                    });
                }
            }
            if !gpus.is_empty() {
                return Some(gpus);
            }
        }
    }

    // 尝试 rocm-smi
    if let Ok(output) = Command::new("rocm-smi")
        .args(&["--showproductname", "--showmeminfo", "vram", "--csv"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut gpus = Vec::new();
            for line in stdout.lines().skip(1) {
                let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                if parts.len() >= 3 {
                    let total_mb = parts[1].parse::<f64>().unwrap_or(0.0);
                    let used_mb = parts[2].parse::<f64>().unwrap_or(0.0);
                    let free_mb = total_mb - used_mb;
                    gpus.push(GpuInfo {
                        name: parts[0].to_string(),
                        total_memory_gb: total_mb / 1024.0,
                        available_memory_gb: free_mb / 1024.0,
                        is_available: true,
                        cuda_version: None,
                    });
                }
            }
            if !gpus.is_empty() {
                return Some(gpus);
            }
        }
    }

    None
}

/// 检测 CUDA 版本
fn detect_cuda_version() -> Option<String> {
    if let Ok(output) = Command::new("nvidia-smi").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("CUDA Version") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    return Some(parts[1].trim().to_string());
                }
            }
        }
    }
    None
}

// ============================================================================
// Tauri 命令
// ============================================================================

/// 列出可用的 ML 势函数 (V4.2-002: 增加经典势函数)
#[tauri::command]
pub async fn list_ml_potentials() -> Result<Vec<MLPotentialInfo>, String> {
    let mut potentials = get_mock_potentials();
    // V4.2-002: 追加经典势函数条目
    potentials.extend(get_classical_potentials());
    Ok(potentials)
}

/// ML 势函数推理 (V4.2-002: 真实 LAMMPS 优先)
#[tauri::command]
pub async fn compute_ml_potential(
    request: MLPotentialComputeRequest,
) -> Result<MLPotentialComputeResult, String> {
    let start = std::time::Instant::now();

    // V4.2-002: 尝试真实 LAMMPS 单点能量计算
    if lammps_available() && !request.positions.is_empty() {
        let cell = request.cell.clone().unwrap_or_else(|| {
            let box_size = 20.0;
            vec![
                vec![box_size, 0.0, 0.0],
                vec![0.0, box_size, 0.0],
                vec![0.0, 0.0, box_size],
            ]
        });

        let pt_name = request.potential_name.to_lowercase();
        let (potential_type, potential_file) = if pt_name.contains("eam") {
            ("eam".to_string(), None)
        } else if pt_name.contains("meam") {
            ("meam".to_string(), None)
        } else if pt_name.contains("reaxff") {
            ("reaxff".to_string(), None)
        } else if pt_name.contains("lj") {
            ("lj".to_string(), None)
        } else if pt_name.contains("airebo") {
            ("airebo".to_string(), None)
        } else {
            ("lj".to_string(), None)
        };

        let work_dir = format!(
            "/tmp/solo_lammps_sp_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );

        let lammps_config = LammpsInputConfig {
            potential_type,
            potential_file,
            potential_file_extra: None,
            ensemble: "nve".to_string(),
            temperature: 0.1, // 近似 0 K 单点
            pressure: None,
            timestep_fs: 1.0,
            num_steps: 0, // run 0 — 单点能量
            atom_types: request.atom_types.clone(),
            positions: request.positions.clone(),
            cell,
            thermo_freq: 1,
            dump_freq: 1,
            work_dir: work_dir.clone(),
            random_seed: 12345,
            lj_params: None,
            element_type_map: None,
        };

        match run_lammps_real(&lammps_config) {
            Ok(result) if result.success => {
                let num_atoms = request.atom_types.len() as u32;
                let total_energy = result.thermo_data.last().map(|t| t.etotal).unwrap_or(0.0);
                let energy_per_atom = total_energy / num_atoms as f64;
                let maybe_forces = result
                    .dump_data
                    .first()
                    .map(|d| {
                        d.atoms
                            .iter()
                            .map(|a| {
                                vec![
                                    a.fx.unwrap_or(0.0),
                                    a.fy.unwrap_or(0.0),
                                    a.fz.unwrap_or(0.0),
                                ]
                            })
                            .collect::<Vec<Vec<f64>>>()
                    });

                return Ok(MLPotentialComputeResult {
                    potential_name: request.potential_name.clone(),
                    total_energy,
                    energy_per_atom,
                    forces: if request.compute_forces {
                        maybe_forces
                    } else {
                        None
                    },
                    stress: None,
                    inference_time_ms: start.elapsed().as_millis() as u64,
                    num_atoms,
                });
            }
            Ok(result) => {
                eprintln!("LAMMPS single-point failed: {}", result.stderr);
            }
            Err(e) => {
                eprintln!("LAMMPS single-point error: {}", e);
            }
        }
    }

    // Fallback: Mock 势函数推理
    let mut result = mock_potential_compute(&request);
    result.inference_time_ms = start.elapsed().as_millis() as u64;
    Ok(result)
}

// ============================================================================
// V4.2-002: LAMMPS 直接调用命令
// ============================================================================

/// 直接调用 LAMMPS 执行 ML 势函数模拟（V4.2-002 新增）
/// 返回完整的 LammpsSimulationResult，包含 thermo 和 dump 数据
#[tauri::command]
pub async fn run_ml_lammps_simulation(
    config: LammpsInputConfig,
) -> Result<LammpsSimulationResult, String> {
    if !lammps_available() {
        // LAMMPS 不可用时返回 mock 结果
        let num_atoms = config.positions.len() as u32;
        let num_steps = config.num_steps;
        let mut mock_thermo = Vec::new();
        let thermo_freq = config.thermo_freq.max(1);
        for s in (0..=num_steps).step_by(thermo_freq as usize).take(500) {
            let _ratio = s as f64 / num_steps as f64;
            mock_thermo.push(LammpsThermoData {
                step: s,
                temp: config.temperature + rand_factor() * 10.0 - 5.0,
                press: rand_factor() * 2.0 - 1.0,
                pe: num_atoms as f64 * (-6.5 + rand_factor() * 0.3),
                ke: num_atoms as f64 * 0.04,
                etotal: num_atoms as f64 * (-6.5 + rand_factor() * 0.3 + 0.04),
                density: 7.8 + rand_factor() * 0.1,
                volume: num_atoms as f64 * 10.0,
            });
        }

        return Ok(LammpsSimulationResult {
            success: true,
            potential_type: config.potential_type.clone(),
            thermo_data: mock_thermo,
            dump_data: vec![],
            total_time_sec: 0.0,
            num_atoms,
            exit_code: Some(0),
            stderr: String::new(),
            is_real: false,
        });
    }

    // 执行真实 LAMMPS
    run_lammps_real(&config)
}

/// 势函数自动选择
#[tauri::command]
pub async fn auto_select_potential(
    request: PotentialSelectionRequest,
) -> Result<PotentialSelectionResult, String> {
    Ok(select_potential_impl(&request))
}

/// 获取 GPU 资源状态 (V4.2-002: 真实检测优先)
#[tauri::command]
pub async fn get_gpu_status() -> Result<GpuResourceStatus, String> {
    // V4.2-002: 优先尝试真实 GPU 检测
    if let Some(gpus) = detect_real_gpu() {
        let total_vram: f64 = gpus.iter().map(|g| g.total_memory_gb).sum();
        let used_vram: f64 = gpus.iter().map(|g| g.total_memory_gb - g.available_memory_gb).sum();
        return Ok(GpuResourceStatus {
            gpus,
            total_vram_gb: total_vram,
            used_vram_gb: used_vram,
            running_tasks: 0,
            queued_tasks: 0,
        });
    }
    // Fallback to mock
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
/// V4.4-003: mock 训练结果（真实训练通过 training_manager 的 submit_real_training_job 调用）
#[tauri::command]
pub async fn submit_training_job(
    config: TrainingConfig,
) -> Result<TrainingResult, String> {
    Ok(mock_train_result(&config))
}

/// 验证势函数质量
#[tauri::command]
pub async fn validate_ml_potential(
    request: ValidationRequest,
) -> Result<ValidationResult, String> {
    Ok(mock_validate(&request))
}

/// 使用 ML 势运行 MD 模拟 (V4.2-002: 真实 LAMMPS 优先)
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

    // V4.2-002: 尝试真实 LAMMPS 运行
    if lammps_available() {
        let temp = temperature.unwrap_or(300.0);
        let the_cell = cell.unwrap_or_else(|| {
            // 根据原子位置估算盒大小
            let box_size = 20.0;
            vec![
                vec![box_size, 0.0, 0.0],
                vec![0.0, box_size, 0.0],
                vec![0.0, 0.0, box_size],
            ]
        });

        // 推断势函数类型
        let pt_lower = potential_name.to_lowercase();
        let (potential_type, potential_file) = if pt_lower.contains("eam") {
            ("eam".to_string(), None)
        } else if pt_lower.contains("meam") {
            ("meam".to_string(), None)
        } else if pt_lower.contains("reaxff") {
            ("reaxff".to_string(), None)
        } else if pt_lower.contains("lj") {
            ("lj".to_string(), None)
        } else if pt_lower.contains("airebo") {
            ("airebo".to_string(), None)
        } else {
            ("lj".to_string(), Some(pt_lower.clone()))
        };

        let work_dir = format!(
            "/tmp/solo_lammps_md_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );

        let config = LammpsInputConfig {
            potential_type,
            potential_file,
            potential_file_extra: None,
            ensemble: ensemble.clone(),
            temperature: temp,
            pressure: None,
            timestep_fs,
            num_steps,
            atom_types: atom_types.clone(),
            positions: positions.clone(),
            cell: the_cell,
            thermo_freq: (num_steps / 100).max(1),
            dump_freq: (num_steps / 10).max(1),
            work_dir: work_dir.clone(),
            random_seed: 12345,
            lj_params: None,
            element_type_map: None,
        };

        match run_lammps_real(&config) {
            Ok(result) if result.success => {
                // 提取摘要
                let final_energy = result
                    .thermo_data
                    .last()
                    .map(|t| t.etotal)
                    .unwrap_or(0.0);
                let initial_energy = result
                    .thermo_data
                    .first()
                    .map(|t| t.etotal)
                    .unwrap_or(0.0);
                let final_temp = result
                    .thermo_data
                    .last()
                    .map(|t| t.temp)
                    .unwrap_or(temp);

                return Ok(serde_json::json!({
                    "success": true,
                    "potential_name": potential_name,
                    "ensemble": ensemble,
                    "num_atoms": num_atoms,
                    "num_steps": num_steps,
                    "timestep_fs": timestep_fs,
                    "temperature": final_temp,
                    "initial_energy_eV": initial_energy,
                    "final_energy_eV": final_energy,
                    "energy_drift_eV": (final_energy - initial_energy).abs(),
                    "total_time_ps": num_steps as f64 * timestep_fs / 1000.0,
                    "wall_time_sec": result.total_time_sec,
                    "is_mock": false,
                    "is_real": true,
                    "thermo_steps": result.thermo_data.len(),
                    "dump_frames": result.dump_data.len(),
                    "message": "LAMMPS simulation completed successfully."
                }));
            }
            Ok(result) => {
                // LAMMPS 运行但失败了，记录错误
                eprintln!("LAMMPS simulation failed: {}", result.stderr);
            }
            Err(e) => {
                eprintln!("LAMMPS execution error: {}", e);
            }
        }
    }

    // Fallback: Mock MD 结果
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
        "is_real": false,
        "message": "Mock MD simulation completed. Connect LAMMPS + pair_style ml for real simulation."
    });

    Ok(result)
}
