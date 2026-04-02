/**
 * RVE (Representative Volume Element) Modeling API
 * V1.4-007: RVE 建模器 API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type RveType =
  | 'random_particles'
  | 'random_fibers'
  | 'grain_structure'
  | 'woven_fabric'
  | 'short_fiber'
  | 'lamina_rve'

export type InclusionShape = 'circle' | 'ellipse' | 'polygon' | 'irregular'

export type DistributionType = 'uniform' | 'gaussian' | 'lognormal'

export interface MicroStructure {
  type: RveType
  volume_fraction: number          // 0-1
  size_x_mm: number
  size_y_mm: number
  size_z_mm: number
  num_inclusions: number
  inclusion_shape: InclusionShape
  min_distance: number
  random_seed: number
  distribution: DistributionType
}

export interface PhaseMaterial {
  name: string
  E: number                       // Pa - 弹性模量
  nu: number                      // 泊松比
  density: number                 // kg/m³
}

export interface InterfaceProperty {
  stiffness: number               // N/m³ - 界面刚度
  strength: number                // Pa - 界面强度
}

export interface RveMaterial {
  matrix: PhaseMaterial
  inclusion: PhaseMaterial
  interface: InterfaceProperty
}

export type MeshType = 'tet' | 'hex' | 'mixed'

export interface RveConfig {
  project_id: string
  micro_structure: MicroStructure
  materials: RveMaterial
  mesh_size: number               // mm - 目标网格尺寸
  periodic_bc: boolean
  mesh_type: MeshType
}

export interface MeshNode {
  id: number
  x: number
  y: number
  z: number
}

export interface MeshElement {
  id: number
  type: string
  nodes: number[]
}

export interface MeshFace {
  id: number
  nodes: number[]
  normal: { x: number; y: number; z: number }
}

export interface MeshData {
  nodes: MeshNode[]
  elements: MeshElement[]
  faces: MeshFace[]
}

export interface MeshQuality {
  avg_aspect_ratio: number
  min_jacobian: number
  skewness_avg: number
  warpage_avg: number
  failed_elements: number
}

export interface PeriodicConstraints {
  node_pairs: Array<{ master: number; slave: number; dof: string[] }>
  constraint_count: number
}

export interface RveResult {
  success: boolean
  mesh: MeshData
  volume_fraction_actual: number
  inclusion_count: number
  mesh_quality: MeshQuality
  periodic_constraints_applied: boolean
  representative_volume_mm3: number
}

export interface RveTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  micro_type: RveType
  typical_application: string
}

// ============ API 函数 ============

/**
 * 生成 RVE 微观结构
 */
export async function generateRve(config: RveConfig): Promise<RveResult> {
  return invoke<RveResult>('generate_rve', { config })
}

/**
 * 计算体积分数
 */
export async function calculateVolumeFraction(
  mesh: MeshData,
  inclusion_ids: number[]
): Promise<number> {
  return invoke<number>('calculate_volume_fraction', { mesh, inclusion_ids })
}

/**
 * 施加周期性边界条件
 */
export async function applyPeriodicBC(mesh: MeshData): Promise<PeriodicConstraints> {
  return invoke<PeriodicConstraints>('apply_periodic_bc', { mesh })
}

/**
 * 获取 RVE 模板列表
 */
export async function getRveTemplates(): Promise<RveTemplate[]> {
  return invoke<RveTemplate[]>('get_rve_templates')
}

/**
 * 导出 RVE 网格
 */
export async function exportRveMesh(
  mesh: MeshData,
  format: 'stl' | 'obj' | 'inp' | 'vtk' | 'abaqus'
): Promise<string> {
  return invoke<string>('export_rve_mesh', { mesh, format })
}
