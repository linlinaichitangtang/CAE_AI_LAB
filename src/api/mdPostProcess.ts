/**
 * MD Post-Processing API
 * V1.5-007: RDF/MSD/扩散系数/VACF 分析
 * V1.5-009: MD→相场 粗粒化桥接
 * V1.5-010: MD→FE 尺度桥接
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export interface RdfBin {
  r_A: number
  g_r: number
  coordination_number: number
}

export interface RdfResult {
  success: boolean
  element_pair: string
  bins: RdfBin[]
  peak_positions: number[]
  first_peak_height: number
  first_peak_position: number
  coordination_number: number
}

export interface MsdTimePoint {
  time_fs: number
  msd_A2: number
}

export interface MsdResult {
  success: boolean
  element: string
  time_points: MsdTimePoint[]
  diffusion_coefficient_cm2_s: number
  diffusion_coefficient_A2_fs: number
  fit_slope: number
  fit_intercept: number
  r_squared: number
}

export interface VacfTimePoint {
  time_fs: number
  vacf: number
}

export interface VdosPoint {
  frequency_THz: number
  vdos: number
}

export interface VacfResult {
  success: boolean
  element: string
  time_points: VacfTimePoint[]
  velocity_autocorrelation: number
  vibrational_density_of_states: VdosPoint[]
}

export type CoarseGrainingMethod = 'qc' | 'mqc' | 'density_field' | 'order_parameter'

export type FeStressMethod = 'virial' | 'hardy' | 'irving_kirkwood'

export type FeOutputType = 'boundary_condition' | 'initial_condition' | 'material_property'

export interface MdToPhaseFieldConfig {
  trajectory_data: string
  coarse_graining_method: CoarseGrainingMethod
  grid_size: { nx: number; ny: number; nz: number }
  smoothing_length_A: number
  output_fields: string[]
}

export interface PhaseFieldGridPoint {
  ix: number
  iy: number
  iz: number
  fields: Record<string, number>
}

export interface MdToPhaseFieldResult {
  success: boolean
  grid_data: PhaseFieldGridPoint[]
  grid_size: { nx: number; ny: number; nz: number }
  field_names: string[]
  smoothing_info: {
    method: string
    kernel_width: number
    effective_resolution: number
  }
  coarse_graining_stats: {
    num_atoms_source: number
    num_grid_points: number
    reduction_ratio: number
    mass_conservation_error: number
  }
}

export interface MdToFeConfig {
  trajectory_data: string
  fe_mesh_size: number
  stress_averaging_method: FeStressMethod
  temperature_method: string
  output_type: FeOutputType
}

export interface FeStressPoint {
  x: number
  y: number
  z: number
  sigma_xx: number
  sigma_yy: number
  sigma_zz: number
  tau_xy: number
  tau_yz: number
  tau_xz: number
}

export interface FeTemperaturePoint {
  x: number
  y: number
  z: number
  temperature: number
}

export interface FeBoundaryCondition {
  node_id: number
  dof: string
  value: number
}

export interface MdToFeResult {
  success: boolean
  fe_stress_field: FeStressPoint[]
  fe_temperature_field: FeTemperaturePoint[]
  fe_boundary_conditions: FeBoundaryCondition[]
  averaged_stress_tensor: [number, number, number, number, number, number]
  mapping_quality: {
    spatial_correlation: number
    energy_conservation: number
    mesh_convergence: number
    overall_score: number
  }
}

export interface MdPostProcessTemplate {
  id: string
  name: string
  description: string
  config: Record<string, unknown>
}

// ============ API 函数 ============

/**
 * 计算径向分布函数 (RDF)
 */
export async function calculateRdf(
  trajectory: string,
  element_pair: string,
  r_max_A: number,
  bin_width_A: number
): Promise<RdfResult> {
  return invoke<RdfResult>('calculate_rdf', {
    trajectory,
    elementPair: element_pair,
    rMaxA: r_max_A,
    binWidthA: bin_width_A
  })
}

/**
 * 计算均方位移 (MSD)
 */
export async function calculateMsd(
  trajectory: string,
  element: string,
  time_range: { start_fs: number; end_fs: number }
): Promise<MsdResult> {
  return invoke<MsdResult>('calculate_msd', {
    trajectory,
    element,
    timeRange: time_range
  })
}

/**
 * 计算速度自相关函数 (VACF)
 */
export async function calculateVacf(
  trajectory: string,
  element: string
): Promise<VacfResult> {
  return invoke<VacfResult>('calculate_vacf', {
    trajectory,
    element
  })
}

/**
 * 从 MSD 结果计算扩散系数
 */
export async function calculateDiffusionCoefficient(
  msd_result: MsdResult
): Promise<number> {
  return invoke<number>('calculate_diffusion_coefficient', {
    msdResult: msd_result
  })
}

/**
 * MD 轨迹粗粒化到相场
 */
export async function coarseGrainToPhaseField(
  config: MdToPhaseFieldConfig
): Promise<MdToPhaseFieldResult> {
  return invoke<MdToPhaseFieldResult>('coarse_grain_to_phase_field', { config })
}

/**
 * MD 轨迹映射到 FE 边界条件
 */
export async function mapToFeBoundary(
  config: MdToFeConfig
): Promise<MdToFeResult> {
  return invoke<MdToFeResult>('map_to_fe_boundary', { config })
}

/**
 * 获取 MD 后处理模板列表
 */
export async function getMdPostProcessTemplates(): Promise<MdPostProcessTemplate[]> {
  return invoke<MdPostProcessTemplate[]>('get_md_post_process_templates')
}
