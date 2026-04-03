/**
 * Phase Field Post-Processing API
 * V1.6-004: 相场-力学耦合
 * V1.6-005: 场可视化
 * V1.6-006: 晶粒统计
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type StressCouplingType =
  | 'elastic_energy'
  | 'eigenstrain'
  | 'misfit_strain'
  | 'phase_transformation'

export interface GrainData {
  grain_id: number
  area: number
  perimeter: number
  equivalent_diameter: number
  orientation: number          // deg
  centroid: { x: number; y: number }
  aspect_ratio: number
  circularity: number
}

export interface GrainSizeBin {
  size_min: number
  size_max: number
  count: number
}

export interface GrainSizeDistribution {
  bins: GrainSizeBin[]
  mean: number
  median: number
  std_dev: number
  max: number
  min: number
}

export interface OrientationBin {
  angle_min: number
  angle_max: number
  count: number
}

export interface OrientationDistribution {
  bins: OrientationBin[]
  preferred_orientation: number  // deg
  texture_strength: number
}

export type FieldType =
  | 'order_parameter'
  | 'temperature'
  | 'stress'
  | 'free_energy'
  | 'grain_id'
  | 'strain'

export type ColormapType =
  | 'viridis'
  | 'jet'
  | 'coolwarm'
  | 'grayscale'
  | 'rainbow'

export interface FieldVisualizationConfig {
  field: FieldType
  colormap: ColormapType
  contour_levels: number
  show_grain_boundaries: boolean
  slice_z: number
  isosurface_value: number
}

export interface StressField {
  xx: number[][]
  yy: number[][]
  zz: number[][]
  xy: number[][]
  yz: number[][]
  xz: number[][]
}

export interface PfGrid {
  nx: number
  ny: number
  nz: number
  dx: number
  dy: number
  dz: number
}

export interface PfMechanicalConfig {
  elastic_modulus_E: number
  poisson_ratio_nu: number
  eigenstrain_type: StressCouplingType
  misfit_strain_magnitude: number
  coupling_strength: number
  external_stress: { xx: number; yy: number; xy: number }
}

export interface PfMechanicalResult {
  success: boolean
  stress_field: StressField
  strain_field: StressField
  elastic_energy_density: number[][]
  total_elastic_energy: number
  max_stress: number
  von_mises_field: number[][]
}

export interface GrainSizeTimePoint {
  time: number
  size: number
}

export interface PfGrainAnalysisResult {
  success: boolean
  grains: GrainData[]
  size_distribution: GrainSizeDistribution
  orientation_distribution: OrientationDistribution
  total_grain_area: number
  num_grains: number
  avg_grain_size: number
  grain_boundary_length: number
  grain_size_vs_time: GrainSizeTimePoint[]
}

// ============ API 函数 ============

/**
 * 运行相场-力学耦合计算
 */
export async function runMechanicalCoupling(
  order_param_field: number[][],
  config: PfMechanicalConfig
): Promise<PfMechanicalResult> {
  return invoke<PfMechanicalResult>('run_mechanical_coupling', {
    orderParamField: order_param_field,
    config,
  })
}

/**
 * 分析晶粒统计信息
 */
export async function analyzeGrains(
  order_param_field: number[][],
  grid: PfGrid
): Promise<PfGrainAnalysisResult> {
  return invoke<PfGrainAnalysisResult>('analyze_grains', {
    orderParamField: order_param_field,
    grid,
  })
}

/**
 * 计算晶粒尺寸分布
 */
export async function calculateGrainSizeDistribution(
  grains: GrainData[]
): Promise<GrainSizeDistribution> {
  return invoke<GrainSizeDistribution>('calculate_grain_size_distribution', { grains })
}

/**
 * 计算取向分布
 */
export async function calculateOrientationDistribution(
  grains: GrainData[]
): Promise<OrientationDistribution> {
  return invoke<OrientationDistribution>('calculate_orientation_distribution', { grains })
}

/**
 * 检测晶界
 */
export async function detectGrainBoundaries(
  order_param_field: number[][],
  threshold: number
): Promise<number[][]> {
  return invoke<number[][]>('detect_grain_boundaries', {
    orderParamField: order_param_field,
    threshold,
  })
}

/**
 * 导出场数据到 CSV
 */
export async function exportFieldToCSV(
  field: number[][],
  filepath: string
): Promise<void> {
  return invoke<void>('export_field_to_csv', { field, filepath })
}

/**
 * 获取相场后处理模板列表
 */
export async function getPhaseFieldPostProcessTemplates(): Promise<
  Array<{ id: string; name: string; description: string }>
> {
  return invoke<Array<{ id: string; name: string; description: string }>>(
    'get_phase_field_post_process_templates'
  )
}
