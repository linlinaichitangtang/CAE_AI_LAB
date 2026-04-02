/**
 * Mesh Refinement API
 * V1.3-009: 局部加密与映射网格 API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

/** 加密区域定义 */
export interface RefinementRegion {
  id: string
  name: string
  type: 'box' | 'sphere' | 'cylinder' | 'face' | 'edge'
  params: Record<string, number>
  element_size: number              // mm
  growth_rate: number               // 1.0 - 2.0
  bias_type: 'uniform' | 'geometric' | 'exponential'
}

/** 边界层配置 */
export interface BoundaryLayerConfig {
  first_layer_height: number        // mm
  growth_rate: number               // 1.0 - 3.0
  total_layers: number
  y_plus_target: number
  prism_layers: number
  smoothing_iterations: number
}

/** 网格质量指标 */
export interface MeshQuality {
  aspect_ratio_avg: number
  jacobian_min: number
  skewness_max: number
  element_count: number
}

/** 网格映射配置 */
export interface MeshMappingConfig {
  source_size: number               // mm
  target_size: number               // mm
  interpolation: 'linear' | 'cubic'
}

/** 网格加密配置 */
export interface MeshRefinementConfig {
  project_id: string
  base_mesh_size: number            // mm
  refinement_regions: RefinementRegion[]
  boundary_layer: BoundaryLayerConfig
  mesh_mapping: MeshMappingConfig
}

/** 网格加密结果 */
export interface MeshRefinementResult {
  success: boolean
  total_nodes_before: number
  total_nodes_after: number
  total_elements_before: number
  total_elements_after: number
  refinement_regions_applied: number
  boundary_layer_elements: number
  quality_before: MeshQuality
  quality_after: MeshQuality
  mesh_file_path: string
}

/** 加密模板 */
export interface RefinementTemplate {
  id: string
  name: string
  description: string
  regions: RefinementRegion[]
}

// ============ API 函数 ============

/**
 * 生成加密网格
 */
export async function generateRefinedMesh(config: MeshRefinementConfig): Promise<MeshRefinementResult> {
  return invoke<MeshRefinementResult>('generate_refined_mesh', { config })
}

/**
 * 计算边界层首层高度 (基于 y+ 目标值)
 * @param yPlus       y+ 目标值
 * @param reynolds    雷诺数
 * @param chordLength 特征长度 (m)
 */
export async function calculateBoundaryLayerHeight(
  yPlus: number,
  reynolds: number,
  chordLength: number
): Promise<number> {
  return invoke<number>('calculate_boundary_layer_height', {
    yPlus,
    reynolds,
    chordLength,
  })
}

/**
 * 验证网格质量
 */
export async function validateMeshQuality(meshData: unknown): Promise<MeshQuality> {
  return invoke<MeshQuality>('validate_mesh_quality', { meshData })
}

/**
 * 获取加密模板列表
 */
export async function getRefinementTemplates(): Promise<RefinementTemplate[]> {
  return invoke<RefinementTemplate[]>('get_refinement_templates')
}
