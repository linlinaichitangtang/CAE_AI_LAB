/**
 * Cohesive Zone Model (CZM) API
 * V1.3-005: 内聚力模型分析 API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export interface CohesiveMaterial {
  name: string
  normal_stiffness: number       // Knn MPa/mm
  shear_stiffness: number        // Kss/Ktt MPa/mm
  normal_strength: number        // sigma_n MPa
  shear_strength: number         // tau_s MPa
  fracture_energy_gn: number     // Gn GJ/m²
  fracture_energy_gs: number     // Gs GJ/m²
  penalty_stiffness: number      // 罚刚度
}

export interface CohesiveBaseMaterial {
  name: string
  E: number                      // 弹性模量 MPa
  nu: number                     // 泊松比
  density: number                // 密度 kg/m³
}

export interface CohesiveConfig {
  project_id: string
  analysis_type: 'delamination' | 'debonding' | 'crack_propagation'
  cohesive_material: CohesiveMaterial
  base_material: CohesiveBaseMaterial
  interface_type: 'surface-based' | 'element-based'
  element_type: 'COH3D8' | 'COH3D6'
  stabilization: boolean
  viscosity: number
}

export interface CohesiveResult {
  success: boolean
  max_separation: number         // mm
  max_stress: number             // MPa
  interface_status: {
    damaged: number
    failed: number
    active: number
  }
  load_displacement_curve: Array<{
    displacement: number
    load: number
  }>
  damage_field: Array<{
    x: number
    y: number
    z: number
    damage: number
  }>
  interface_energy: number       // mJ
  total_energy: number           // mJ
}

export interface CohesiveTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  interface_type: 'surface-based' | 'element-based'
  default_material: CohesiveMaterial
  typical_application: string
}

// ============ API 函数 ============

/**
 * 运行内聚力模型分析
 */
export async function runCohesiveAnalysis(config: CohesiveConfig): Promise<CohesiveResult> {
  return invoke<CohesiveResult>('run_cohesive_analysis', { config })
}

/**
 * 获取内聚力模型工程模板
 */
export async function getCohesiveTemplates(): Promise<CohesiveTemplate[]> {
  return invoke<CohesiveTemplate[]>('get_cohesive_templates')
}

/**
 * 生成内聚力模型 INP 文件
 */
export async function generateCohesiveInp(config: CohesiveConfig): Promise<string> {
  return invoke<string>('generate_cohesive_inp', { config })
}

/**
 * 计算界面刚度参数
 */
export async function calculateInterfaceStiffness(material: CohesiveMaterial): Promise<{
  kn: number
  ks: number
  penalty: number
}> {
  return invoke<{ kn: number; ks: number; penalty: number }>('calculate_interface_stiffness', { material })
}

/**
 * 获取内聚力材料库
 */
export async function getCohesiveMaterialLibrary(): Promise<CohesiveMaterial[]> {
  return invoke<CohesiveMaterial[]>('get_cohesive_material_library')
}
