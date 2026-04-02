/**
 * FSI (Fluid-Structure Interaction) API
 * V1.3-002: 流-固耦合分析 API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export interface FsiNodeData {
  node_id: number
  x: number
  y: number
  z: number
  value: number
}

export interface FsiMaterial {
  name: string
  youngs_modulus: number   // Pa
  poisson_ratio: number
  density: number          // kg/m^3
  thickness?: number       // m (for shell/plate)
}

export interface FsiConfig {
  project_id: string
  coupling_type: 'one_way' | 'two_way'
  cfd_pressure_field: FsiNodeData[]
  cfd_temperature_field: FsiNodeData[]
  mapping_method: 'nearest' | 'rbf' | 'conservative'
  structural_material?: FsiMaterial
  structural_bc_type?: 'fixed' | 'simply_supported'
}

export interface FsiResult {
  max_displacement: number
  max_stress: number
  mapped_pressure_nodes: number
  mapping_error: number
  total_force: number
  avg_pressure: number
  max_pressure: number
  displacement_field: FsiNodeData[]
  stress_field: FsiNodeData[]
}

export interface FsiTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  coupling_type: string
  default_mapping_method: string
  default_material: FsiMaterial
  typical_pressure_range: [number, number]
  typical_velocity_range: [number, number]
}

// ============ API 函数 ============

/**
 * 运行 FSI 分析
 */
export async function runFsiAnalysis(config: FsiConfig): Promise<FsiResult> {
  return invoke<FsiResult>('run_fsi_analysis', { config })
}

/**
 * 将 CFD 结果映射到结构网格
 */
export async function mapCfdToStructural(
  cfdNodes: FsiNodeData[],
  structuralNodes: FsiNodeData[],
  method: 'nearest' | 'rbf' | 'conservative'
): Promise<FsiNodeData[]> {
  return invoke<FsiNodeData[]>('map_cfd_to_structural', {
    cfdNodes,
    structuralNodes,
    method,
  })
}

/**
 * 获取 FSI 工程算例模板
 */
export async function getFsiTemplates(): Promise<FsiTemplate[]> {
  return invoke<FsiTemplate[]>('get_fsi_templates')
}
