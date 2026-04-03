/**
 * Phase Field Bridge API
 * V1.6-008: MD -> 相场桥接 & V1.6-009: 相场 -> FE 均匀化 API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type CoarseGrainingMethod =
  | 'atomic_density'
  | 'bond_order'
  | 'centro_symmetry'
  | 'voronoi'
  | 'ml_interpolation'

export type OrderParameterMapping =
  | 'local_density'
  | 'crystal_structure'
  | 'custom'

export type TemperatureMapping =
  | 'kinetic_energy'
  | 'local_pressure'

export type HomogenizationMethod =
  | 'voigt'
  | 'reuss'
  | 'mori_tanaka'
  | 'self_consistent'
  | 'numerical'

export type OutputType =
  | 'stress_strain_curve'
  | 'elastic_tensor'
  | 'yield_surface'

export type BridgeDirection = 'md_to_pf' | 'pf_to_fe'

export interface MdToPfConfig {
  trajectory_data: object
  coarse_graining_method: CoarseGrainingMethod
  target_grid: { nx: number; ny: number; nz: number }
  smoothing_length_A: number
  order_parameter_mapping: OrderParameterMapping
  temperature_mapping: TemperatureMapping
  interface_detection_threshold: number
}

export interface MappingQuality {
  spatial_correlation: number
  energy_conservation: number
  interface_sharpness: number
}

export interface CoarseGrainingStats {
  source_atoms: number
  target_grid_points: number
  reduction_ratio: number
}

export interface MdToPfResult {
  success: boolean
  order_parameter_field: number[][]
  temperature_field: number[][]
  grain_structure: number[][]
  mapping_quality: MappingQuality
  coarse_graining_stats: CoarseGrainingStats
}

export interface PfToFeConfig {
  phase_field_data: object
  grid_size: { nx: number; ny: number; nz: number }
  homogenization_method: HomogenizationMethod
  strain_range: { min: number; max: number; steps: number }
  temperature: number
  output_type: OutputType
}

export interface EquivalentProperties {
  E: number
  nu: number
  sigma_y: number
  hardening_modulus: number
}

export interface PfToFeResult {
  success: boolean
  effective_elastic_tensor: number[][]
  yield_stress: number
  hardening_curve: Array<{ strain: number; stress: number }>
  stress_strain_curve: Array<{ strain: number; stress: number }>
  phase_fraction: number
  equivalent_properties: EquivalentProperties
  homogenization_convergence: Array<{ iteration: number; error: number }>
}

export interface BridgeTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  direction: BridgeDirection
  typical_application: string
}

// ============ API 函数 ============

/**
 * MD -> 相场粗粒化
 */
export async function coarseGrainMdToPf(config: MdToPfConfig): Promise<MdToPfResult> {
  return invoke<MdToPfResult>('coarse_grain_md_to_pf', { config })
}

/**
 * 相场 -> FE 均匀化
 */
export async function homogenizePfToFe(config: PfToFeConfig): Promise<PfToFeResult> {
  return invoke<PfToFeResult>('homogenize_pf_to_fe', { config })
}

/**
 * 验证桥接质量
 */
export async function validateBridgeQuality(
  result: MdToPfResult | PfToFeResult
): Promise<{ quality_score: number; issues: string[] }> {
  return invoke<{ quality_score: number; issues: string[] }>('validate_bridge_quality', { result })
}

/**
 * 获取桥接模板列表
 */
export async function getBridgeTemplates(): Promise<BridgeTemplate[]> {
  return invoke<BridgeTemplate[]>('get_bridge_templates')
}

/**
 * 导出桥接数据
 */
export async function exportBridgeData(
  result: MdToPfResult | PfToFeResult,
  format: 'json' | 'csv' | 'vtk',
  filepath: string
): Promise<void> {
  return invoke<void>('export_bridge_data', { result, format, filepath })
}
