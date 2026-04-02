/**
 * Homogenization Methods (FE²) API
 * V1.4-008: 均匀化方法 FE² API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type HomogenizationMethod =
  | 'voigt'
  | 'reuss'
  | 'hill'
  | 'mori_tanaka'
  | 'self_consistent'
  | 'fe2'

export interface MacroStrain {
  exx: number
  eyy: number
  ezz: number
  gxy: number
  gyz: number
  gxz: number
}

export interface HomogenizationConfig {
  project_id: string
  method: HomogenizationMethod
  rve_mesh_data: object
  macro_strains: MacroStrain[]
  convergence_tolerance: number
  max_iterations: number
  temperature: number              // degC
}

export interface EffectiveProperties {
  Ex: number                       // Pa
  Ey: number                       // Pa
  Ez: number                       // Pa
  Gxy: number                      // Pa
  Gyz: number                      // Pa
  Gxz: number                      // Pa
  nu_xy: number
  nu_yz: number
  nu_xz: number
}

export interface ConvergenceRecord {
  iteration: number
  error: number
}

export interface HomogenizationResult {
  success: boolean
  effective_properties: EffectiveProperties
  stiffness_matrix_C: number[][]   // 6x6
  compliance_matrix_S: number[][]  // 6x6
  strain_concentration_tensors: number[][][]
  stress_concentration_tensors: number[][][]
  convergence_history: ConvergenceRecord[]
  volume_averaged_stress: MacroStrain
  volume_averaged_strain: MacroStrain
}

export interface HomogenizationTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  method: HomogenizationMethod
  typical_application: string
}

export interface PhaseProperties {
  name: string
  E: number                       // Pa
  nu: number
  density: number                 // kg/m³
}

export interface VoigtReussBounds {
  voigt: EffectiveProperties
  reuss: EffectiveProperties
  hill: EffectiveProperties
}

// ============ API 函数 ============

/**
 * 运行均匀化分析
 */
export async function runHomogenization(config: HomogenizationConfig): Promise<HomogenizationResult> {
  return invoke<HomogenizationResult>('run_homogenization', { config })
}

/**
 * 计算 Voigt/Reuss/Hill 上下界
 */
export async function calculateVoigtReussBounds(
  phases: PhaseProperties[],
  volume_fractions: number[]
): Promise<VoigtReussBounds> {
  return invoke<VoigtReussBounds>('calculate_voigt_reuss_bounds', { phases, volume_fractions })
}

/**
 * 运行 FE² 多尺度分析
 */
export async function runFE2Analysis(
  rve_config: object,
  macro_strain_history: MacroStrain[]
): Promise<HomogenizationResult> {
  return invoke<HomogenizationResult>('run_fe2_analysis', { rve_config, macro_strain_history })
}

/**
 * 获取均匀化方法模板列表
 */
export async function getHomogenizationTemplates(): Promise<HomogenizationTemplate[]> {
  return invoke<HomogenizationTemplate[]>('get_homogenization_templates')
}
