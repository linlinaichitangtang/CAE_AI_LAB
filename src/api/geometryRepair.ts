/**
 * Geometry Repair API
 * V1.3-007: 几何清理与修复 (CAD 修复) API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

/** 几何问题类型 */
export type GeometryIssueType =
  | 'sliver_face'
  | 'narrow_face'
  | 'small_hole'
  | 'gap'
  | 'overlap'
  | 'non_manifold'
  | 'duplicate_face'
  | 'short_edge'
  | 't_junction'
  | 'self_intersection'

/** 严重程度 */
export type IssueSeverity = 'critical' | 'warning' | 'info'

/** 修复操作类型 */
export type RepairOperationType =
  | 'fill_sliver'
  | 'remove_hole'
  | 'close_gap'
  | 'remove_overlap'
  | 'fix_non_manifold'
  | 'merge_faces'
  | 'remove_duplicate'
  | 'extend_short_edge'
  | 'fix_t_junction'
  | 'fix_self_intersection'

/** 修复操作状态 */
export type RepairStatus = 'pending' | 'success' | 'failed'

/** 文件格式 */
export type GeometryFileFormat = 'STL' | 'STEP' | 'IGES' | 'BREP'

/** 几何问题 */
export interface GeometryIssue {
  type: GeometryIssueType
  severity: IssueSeverity
  description: string
  face_ids: number[]
  edge_ids: number[]
  position: { x: number; y: number; z: number }
  area?: number
  length?: number
}

/** 修复操作 */
export interface RepairOperation {
  type: RepairOperationType
  description: string
  affected_elements: number[]
  status: RepairStatus
}

/** 几何修复配置 */
export interface GeometryRepairConfig {
  file_path: string
  file_format: GeometryFileFormat
  repair_operations: RepairOperation[]
  tolerance: number
  merge_distance: number
  min_face_area: number
  min_hole_diameter: number
  max_edge_length: number
}

/** 修复统计 */
export interface GeometryRepairStats {
  total_faces_before: number
  total_faces_after: number
  total_edges_before: number
  total_edges_after: number
  total_vertices_before: number
  total_vertices_after: number
  watertight_before: boolean
  watertight_after: boolean
  volume_before: number
  volume_after: number
}

/** 几何修复结果 */
export interface GeometryRepairResult {
  success: boolean
  original_issues: GeometryIssue[]
  repaired_issues: GeometryIssue[]
  operations_performed: RepairOperation[]
  stats: GeometryRepairStats
  repaired_file_path: string
  validation_passed: boolean
}

/** 几何修复模板 */
export interface GeometryTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  file_format: GeometryFileFormat
  typical_issues: GeometryIssueType[]
}

/** 导入几何数据 */
export interface ImportedGeometry {
  nodes: Array<{ x: number; y: number; z: number }>
  elements: Array<{ id: number; node_ids: number[]; type: string }>
  faces: Array<{ id: number; node_ids: number[]; normal: { x: number; y: number; z: number } }>
}

// ============ API 函数 ============

/**
 * 导入几何文件
 * @param filePath 文件路径
 * @param format   文件格式 (STL/STEP/IGES/BREP)
 */
export async function importGeometry(
  filePath: string,
  format: GeometryFileFormat
): Promise<ImportedGeometry> {
  return invoke<ImportedGeometry>('import_geometry', { filePath, format })
}

/**
 * 检测几何问题
 * @param config 修复配置
 */
export async function detectIssues(config: GeometryRepairConfig): Promise<GeometryIssue[]> {
  return invoke<GeometryIssue[]>('detect_geometry_issues', { config })
}

/**
 * 执行几何修复
 * @param config 修复配置
 */
export async function repairGeometry(config: GeometryRepairConfig): Promise<GeometryRepairResult> {
  return invoke<GeometryRepairResult>('repair_geometry', { config })
}

/**
 * 验证几何模型
 * @param filePath 文件路径
 * @param format   文件格式
 */
export async function validateGeometry(
  filePath: string,
  format: GeometryFileFormat
): Promise<GeometryRepairResult> {
  return invoke<GeometryRepairResult>('validate_geometry', { filePath, format })
}

/**
 * 导出几何文件
 * @param data    几何数据
 * @param format  导出格式
 * @param filePath 输出文件路径
 */
export async function exportGeometry(
  data: ImportedGeometry,
  format: GeometryFileFormat,
  filePath: string
): Promise<void> {
  return invoke<void>('export_geometry', { data, format, filePath })
}

/**
 * 获取几何修复模板列表
 */
export async function getGeometryRepairTemplates(): Promise<GeometryTemplate[]> {
  return invoke<GeometryTemplate[]>('get_geometry_repair_templates')
}
