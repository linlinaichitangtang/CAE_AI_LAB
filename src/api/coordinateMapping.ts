/**
 * Cross-Scale Coordinate Mapping Engine API
 * V1.8-002: 跨尺度坐标映射引擎 (MD box ↔ Phase Field grid ↔ FE mesh) API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type ScaleLevel = 'dft' | 'md' | 'phase_field' | 'fe'

export interface Vec3 {
  x: number
  y: number
  z: number
}

export interface MdBox {
  origin: Vec3
  lattice_vectors: Vec3[]       // 3 vectors
  periodic: [boolean, boolean, boolean]
  n_atoms: number
}

export interface PhaseFieldGrid {
  origin: Vec3
  dimensions: { nx: number; ny: number; nz: number }
  spacing: { dx: number; dy: number; dz: number }
  n_cells: number
}

export interface BoundingBox {
  min: Vec3
  max: Vec3
}

export interface FeMesh {
  bounding_box: BoundingBox
  n_elements: number
  element_type: string
}

export interface MappingOptions {
  align_crystal_orientation: boolean
  align_origin: boolean
  tolerance_A: number          // 容差 (Å)
}

export interface MappingSource {
  scale: ScaleLevel
  data: MdBox | PhaseFieldGrid | FeMesh
}

export interface MappingConfig {
  source: MappingSource
  target: MappingSource
  options: MappingOptions
}

export interface MappingResult {
  success: boolean
  transformation_matrix: number[][]    // 4x4
  alignment_error_deg: number
  origin_error_A: number
  mapped_points_count: number
  warnings: string[]
}

export interface CoordinatePoint {
  source: Vec3
  target: Vec3
  scale: ScaleLevel
}

export interface AlignmentValidation {
  is_valid: boolean
  max_error_A: number
  orientation_error_deg: number
}

// ============ API 函数 ============

/**
 * 创建跨尺度坐标映射
 */
export async function createMapping(config: MappingConfig): Promise<MappingResult> {
  return invoke<MappingResult>('create_mapping', { config })
}

/**
 * 映射坐标点
 */
export async function mapPoints(
  points: Vec3[],
  mappingId: string
): Promise<CoordinatePoint[]> {
  return invoke<CoordinatePoint[]>('map_points', { points, mappingId })
}

/**
 * 获取映射信息
 */
export async function getMappingInfo(mappingId: string): Promise<MappingResult> {
  return invoke<MappingResult>('get_mapping_info', { mappingId })
}

/**
 * 验证映射对齐精度
 */
export async function validateAlignment(mappingId: string): Promise<AlignmentValidation> {
  return invoke<AlignmentValidation>('validate_alignment', { mappingId })
}
