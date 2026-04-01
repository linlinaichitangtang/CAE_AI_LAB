/**
 * Biomechanics Analysis API - Frontend wrapper for backend biomechanics commands
 * 封装生物力学分析相关的后端API
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export interface BiomechanicsMeshConfig {
  x_min: number
  x_max: number
  y_min: number
  y_max: number
  z_min: number
  z_max: number
  x_div: number
  y_div: number
  z_div: number
  mesh_quality_target: number
  stl_import_path: string | null
}

export interface BiomechanicsMaterial {
  name: string
  category: string
  density: number
  youngs_modulus: number
  poissions_ratio: number
  yield_strength: number
  ultimate_strength: number
  fatigue_limit: number
  hardness: number
  bio_compatible: boolean
}

export interface LoadingConfiguration {
  load_type: string
  load_magnitude: number
  load_direction: [number, number, number]
  load_application_area: string
  frequency: number | null
  num_cycles: number | null
}

export interface BiomechanicsBC {
  name: string
  bc_type: string
  face: string
  values: [number, number, number] | null
}

export interface BiomechanicsAnalysisJob {
  id: string
  name: string
  analysis_type: string
  mesh_config: BiomechanicsMeshConfig
  material: BiomechanicsMaterial
  loading_config: LoadingConfiguration
  boundary_conditions: BiomechanicsBC[]
  results: BiomechanicsResults | null
  status: string
}

export interface BiomechanicsResults {
  displacement: [number, [number, number, number]][]
  stress: [number, [number, number, number, number, number, number]][]
  von_mises: [number, number][]
  strain: [number, [number, number, number, number, number, number]][]
  max_displacement: number
  max_stress: number
  max_von_mises: number
  stress_shielding_ratio: number
  interface_micromotion: number
  safety_factor: number
  fatigue_damage: number
}

export interface BiomechanicsTemplate {
  id: string
  name: string
  description: string
  category: string
  config: BiomechanicsAnalysisJob
}

export interface STLGeometry {
  file_name: string
  node_count: number
  element_count: number
  bounding_box: [number, number, number, number, number, number]
  status: string
}

// ============ API 函数 ============

/**
 * 获取生物力学材料库
 */
export async function getBiomechanicsMaterialLibrary(): Promise<BiomechanicsMaterial[]> {
  return invoke('get_bio_material_library')
}

/**
 * 获取生物力学分析模板
 */
export async function getBiomechanicsTemplates(): Promise<BiomechanicsTemplate[]> {
  return invoke('get_bio_templates')
}

/**
 * 运行生物力学分析
 */
export async function runBiomechanicsAnalysis(
  job: BiomechanicsAnalysisJob
): Promise<BiomechanicsResults> {
  return invoke('run_biomechanics_analysis', { job })
}

/**
 * 导入STL几何文件
 */
export async function importStlGeometry(filePath: string): Promise<STLGeometry> {
  return invoke('import_stl_geometry', { filePath })
}
