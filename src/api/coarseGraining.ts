/**
 * Coarse-Graining Strategy Library API
 * V1.8-003: 粗粒化策略库 (QC/MQC/CG methods) API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type CoarseGrainingMethod =
  | 'quasicontinuum'
  | 'mqc'
  | 'radial_average'
  | 'fourier_filter'
  | 'ml_mapping'
  | 'voronoi'

export type CoarseGrainScaleLevel = 'dft' | 'md' | 'phase_field' | 'fe'

export interface CoarseGrainingConfig {
  method: CoarseGrainingMethod
  source_scale: CoarseGrainScaleLevel
  target_scale: CoarseGrainScaleLevel
  source_data: Record<string, unknown>
  parameters: Record<string, number>    // cutoff_radius, grid_resolution, filter_cutoff, n_clusters, etc.
  atom_types: string[]
}

export interface FieldData {
  grid_size: { nx: number; ny: number; nz: number }
  values: number[]
  field_name: string
}

export interface CoarseGrainingMetadata {
  source_scale: CoarseGrainScaleLevel
  target_scale: CoarseGrainScaleLevel
  method: CoarseGrainingMethod
  timestamp: string
}

export interface CoarseGrainingResult {
  success: boolean
  method: CoarseGrainingMethod
  source_points_count: number
  target_points_count: number
  reduction_ratio: number
  computation_time_s: number
  field_data: FieldData
  metadata: CoarseGrainingMetadata
}

export interface MethodRecommendation {
  method: CoarseGrainingMethod
  score: number
  reason: string
  estimated_accuracy: number
}

export interface CoarseGrainingPreset {
  id: string
  name: string
  method: CoarseGrainingMethod
  description: string
}

// ============ API 函数 ============

/**
 * 运行粗粒化分析
 */
export async function runCoarseGraining(config: CoarseGrainingConfig): Promise<CoarseGrainingResult> {
  return invoke<CoarseGrainingResult>('run_coarse_graining', { config })
}

/**
 * 推荐粗粒化方法
 */
export async function recommendMethod(
  sourceScale: CoarseGrainScaleLevel,
  targetScale: CoarseGrainScaleLevel,
  dataDescription: string
): Promise<MethodRecommendation[]> {
  return invoke<MethodRecommendation[]>('recommend_method', { sourceScale, targetScale, dataDescription })
}

/**
 * 对比多种粗粒化方法
 */
export async function compareMethods(configs: CoarseGrainingConfig[]): Promise<CoarseGrainingResult[]> {
  return invoke<CoarseGrainingResult[]>('compare_methods', { configs })
}

/**
 * 获取粗粒化预设方案列表
 */
export async function getCoarseGrainingPresets(): Promise<CoarseGrainingPreset[]> {
  return invoke<CoarseGrainingPreset[]>('get_coarse_graining_presets')
}
