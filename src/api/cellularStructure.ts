/**
 * Cellular / Lattice Structure API
 * V1.4-002: 蜂窝/点阵结构生成 API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type CellType =
  | 'hexagonal'
  | 'triangular'
  | 'square'
  | 're-entrant'
  | 'chiral'
  | 'lattice_bcc'
  | 'lattice_fcc'
  | 'lattice_body_centered'

export interface BaseMaterial {
  name: string
  E: number                     // Pa - 弹性模量
  nu: number                    // 泊松比
  density: number               // kg/m³
}

export interface CellularConfig {
  cell_type: CellType
  cell_size: number              // mm - 胞元尺寸
  wall_thickness: number         // mm - 壁厚
  num_cells_x: number
  num_cells_y: number
  num_cells_z: number
  base_material: BaseMaterial
  relative_density_target: number
}

export interface EquivalentProperties {
  Ex: number                     // Pa
  Ey: number                     // Pa
  Ez: number                     // Pa
  Gxy: number                    // Pa
  Gyz: number                    // Pa
  Gxz: number                    // Pa
  nu_xy: number
  nu_yz: number
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

export interface MeshData {
  nodes: MeshNode[]
  elements: MeshElement[]
}

export interface CellularResult {
  success: boolean
  relative_density: number
  equivalent_properties: EquivalentProperties
  mass: number                   // kg
  volume: number                 // mm³
  nodes: number
  elements: number
  cell_count: number
  mesh_data: MeshData
}

export interface OptimizationResult {
  optimal_params: {
    cell_size: number
    wall_thickness: number
  }
  target_met: boolean
  max_stress: number             // Pa
  equivalent_stiffness: number   // Pa
}

export interface CellularTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  cell_type: CellType
  typical_application: string
}

// ============ API 函数 ============

/**
 * 生成蜂窝/点阵结构
 */
export async function generateCellularStructure(config: CellularConfig): Promise<CellularResult> {
  return invoke<CellularResult>('generate_cellular_structure', { config })
}

/**
 * 计算等效力学性能
 */
export async function calculateEquivalentProperties(config: CellularConfig): Promise<EquivalentProperties> {
  return invoke<EquivalentProperties>('calculate_equivalent_properties', { config })
}

/**
 * 优化蜂窝/点阵参数
 */
export async function optimizeCellularParams(
  config: CellularConfig,
  target: 'min_weight' | 'max_stiffness' | 'target_density'
): Promise<OptimizationResult> {
  return invoke<OptimizationResult>('optimize_cellular_params', { config, target })
}

/**
 * 获取蜂窝/点阵结构模板
 */
export async function getCellularTemplates(): Promise<CellularTemplate[]> {
  return invoke<CellularTemplate[]>('get_cellular_templates')
}

/**
 * 导出蜂窝/点阵网格
 */
export async function exportCellularMesh(
  data: MeshData,
  format: 'stl' | 'obj' | 'inp' | 'vtk'
): Promise<string> {
  return invoke<string>('export_cellular_mesh', { data, format })
}
