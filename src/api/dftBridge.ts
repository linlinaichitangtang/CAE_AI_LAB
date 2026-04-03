/**
 * DFT Bridge API
 * V1.7-006: DFT -> MD 势函数训练接口
 * V1.7-007: DFT -> 相场 GL 参数
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type PotentialType = 'nep' | 'mtp' | 'eam' | 'meam' | 'lj' | 'tersoff'

export type FreeEnergyModel = 'regular_solution' | 'subregular' | 'calphad'

export interface TrainingConfig {
  potential_type: PotentialType
  dft_data_directory: string
  training_set_ratio: number
  validation_set_ratio: number
  test_set_ratio: number
  energy_weight: number
  force_weight: number
  stress_weight: number
  max_neuron: number
  cutoff_A: number
  training_epochs: number
  batch_size: number
  learning_rate: number
}

export interface LossRecord {
  epoch: number
  energy_rmse: number
  force_rmse: number
}

export interface TrainingResult {
  success: boolean
  potential_type: PotentialType
  training_loss_history: LossRecord[]
  validation_loss_history: LossRecord[]
  test_energy_rmse_meV: number
  test_force_rmse_meV_A: number
  test_stress_rmse_GPa: number
  potential_file_path: string
  training_time_seconds: number
  best_epoch: number
}

export interface GinzburgLandauParams {
  free_energy_coefficients: {
    A: number
    B: number
    C: number
    κ: number
  }
  interface_width: number
  equilibrium_order_parameter: number
  mobility: number
  chemical_potential: number
  temperature_range: {
    min: number
    max: number
  }
}

export interface PhaseInfo {
  name: string
  composition_range: { min: number; max: number }
  temperature_range: { min: number; max: number }
  stability: string
}

export interface TieLine {
  T: number
  x1: number
  x2: number
}

export interface CriticalPoint {
  T: number
  x: number
}

export interface PhaseDiagramData {
  phases: PhaseInfo[]
  tie_lines: TieLine[]
  critical_points: CriticalPoint[]
  eutectic_points: CriticalPoint[]
}

export interface FreeEnergyCurvePoint {
  T: number
  composition: number
  free_energy: number
}

export interface DftToPfConfig {
  dft_results_directory: string
  target_phases: string[]
  temperature_range: { min: number; max: number; step: number }
  composition_range: { min: number; max: number; step: number }
  free_energy_model: FreeEnergyModel
}

export interface DftToPfResult {
  success: boolean
  gl_params: GinzburgLandauParams
  phase_diagram: PhaseDiagramData
  free_energy_curves: FreeEnergyCurvePoint[]
  miscibility_gap: { min_T: number; max_T: number; x_left: number[]; x_right: number[] }
  critical_temperature: number
}

export interface DftBridgeTemplate {
  id: string
  name: string
  description: string
  config: Record<string, unknown>
}

// ============ API 函数 ============

/**
 * 准备训练数据：将 DFT 计算结果拆分为训练集/验证集/测试集
 */
export async function prepareTrainingData(
  dft_results: object
): Promise<{ training_file: string; validation_file: string; test_file: string }> {
  return invoke<{ training_file: string; validation_file: string; test_file: string }>(
    'prepare_training_data',
    { dftResults: dft_results }
  )
}

/**
 * 训练势函数
 */
export async function trainPotential(config: TrainingConfig): Promise<TrainingResult> {
  return invoke<TrainingResult>('train_potential', { config })
}

/**
 * 验证势函数精度
 */
export async function validatePotential(
  potential_path: string,
  test_data: object
): Promise<{ energy_rmse: number; force_rmse: number }> {
  return invoke<{ energy_rmse: number; force_rmse: number }>(
    'validate_potential',
    { potentialPath: potential_path, testData: test_data }
  )
}

/**
 * 导出势函数到指定格式
 */
export async function exportPotential(
  potential_path: string,
  format: string
): Promise<string> {
  return invoke<string>('export_potential', { potentialPath: potential_path, format })
}

/**
 * 从 DFT 结果提取 Ginzburg-Landau 参数
 */
export async function extractGlParams(
  dft_results: object
): Promise<GinzburgLandauParams> {
  return invoke<GinzburgLandauParams>('extract_gl_params', { dftResults: dft_results })
}

/**
 * 计算相图
 */
export async function calculatePhaseDiagram(
  config: DftToPfConfig
): Promise<PhaseDiagramData> {
  return invoke<PhaseDiagramData>('calculate_phase_diagram', { config })
}

/**
 * 化学势对齐
 */
export async function alignChemicalPotential(
  dft_energies: object,
  reference_phases: string[]
): Promise<{ chemical_potentials: Record<string, number> }> {
  return invoke<{ chemical_potentials: Record<string, number> }>(
    'align_chemical_potential',
    { dftEnergies: dft_energies, referencePhases: reference_phases }
  )
}

/**
 * 获取 DFT 桥接模板列表
 */
export async function getDftBridgeTemplates(): Promise<DftBridgeTemplate[]> {
  return invoke<DftBridgeTemplate[]>('get_dft_bridge_templates')
}
