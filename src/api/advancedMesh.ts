/**
 * Advanced Mesh API
 * V1.3-008: 高级网格工具 (梁/壳/实体混合) API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

/** 梁截面类型 */
export type BeamSectionType =
  | 'circular'
  | 'rectangular'
  | 'i_section'
  | 't_section'
  | 'l_section'
  | 'hollow_circular'
  | 'hollow_rectangular'

/** 梁截面属性 */
export interface BeamSection {
  type: BeamSectionType
  name: string
  dimensions: Record<string, number>
  area: number                // mm^2
  Iy: number                  // mm^4 (绕Y轴惯性矩)
  Iz: number                  // mm^4 (绕Z轴惯性矩)
  J: number                   // mm^4 (扭转常数)
}

/** 壳单元类型 */
export type ShellElementType = 'S4R' | 'S8R' | 'S3' | 'STRI65'

/** 壳偏置类型 */
export type ShellOffsetType = 'top' | 'bottom' | 'middle'

/** 壳单元属性 */
export interface ShellProperties {
  thickness: number           // mm
  element_type: ShellElementType
  integration_points: number
  offset_type: ShellOffsetType
}

/** 实体单元类型 */
export type SolidElementType = 'C3D8R' | 'C3D4' | 'C3D10'

/** 梁单元类型 */
export type BeamElementType = 'B31' | 'B32' | 'PIPE31'

/** 网格质量目标 */
export type MeshQualityTarget = 'high' | 'medium' | 'low'

/** 混合网格配置 */
export interface MixedMeshConfig {
  project_id: string
  beam_sections: BeamSection[]
  shell_properties: ShellProperties
  solid_element_type: SolidElementType
  beam_element_type: BeamElementType
  mesh_size: number           // mm
  transition_elements: boolean
  mesh_quality_target: MeshQualityTarget
}

/** 统计范围 (min/max/avg) */
export interface StatRange {
  min: number
  max: number
  avg: number
}

/** 网格质量指标 */
export interface MeshQualityMetrics {
  aspect_ratio: StatRange
  jacobian: StatRange
  warpage: StatRange
  skewness: StatRange
  taper: StatRange
  element_quality_score: number   // 0-1
  total_elements: number
  failed_elements: number
  warning_elements: number
}

/** 单元统计 */
export interface ElementStats {
  count: number
  nodes: number
}

/** 混合网格结果 */
export interface MixedMeshResult {
  success: boolean
  beam_elements: ElementStats
  shell_elements: ElementStats
  solid_elements: ElementStats
  transition_elements: ElementStats
  total_nodes: number
  total_elements: number
  quality_metrics: MeshQualityMetrics
  mesh_stats: Record<string, number>
}

/** 高级网格模板 */
export interface AdvancedMeshTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  typical_application: string
  default_sections: BeamSection[]
}

// ============ API 函数 ============

/**
 * 生成混合网格 (梁/壳/实体)
 */
export async function generateMixedMesh(config: MixedMeshConfig): Promise<MixedMeshResult> {
  return invoke<MixedMeshResult>('generate_mixed_mesh', { config })
}

/**
 * 计算梁截面属性 (面积、惯性矩、扭转常数)
 */
export async function calculateBeamSectionProperties(section: BeamSection): Promise<BeamSection> {
  return invoke<BeamSection>('calculate_beam_section_properties', { section })
}

/**
 * 计算壳单元属性
 */
export async function calculateShellProperties(props: ShellProperties): Promise<ShellProperties> {
  return invoke<ShellProperties>('calculate_shell_properties', { props })
}

/**
 * 检查网格质量
 */
export async function checkMeshQuality(meshData: MixedMeshResult): Promise<MeshQualityMetrics> {
  return invoke<MeshQualityMetrics>('check_mesh_quality', { meshData })
}

/**
 * 获取高级网格模板列表
 */
export async function getAdvancedMeshTemplates(): Promise<AdvancedMeshTemplate[]> {
  return invoke<AdvancedMeshTemplate[]>('get_advanced_mesh_templates')
}

/**
 * 获取梁截面库
 */
export async function getBeamSectionLibrary(): Promise<BeamSection[]> {
  return invoke<BeamSection[]>('get_beam_section_library')
}

/**
 * 导出网格到 INP 文件
 */
export async function exportMeshToInp(meshData: MixedMeshResult, filePath: string): Promise<string> {
  return invoke<string>('export_mesh_to_inp', { meshData, filePath })
}
