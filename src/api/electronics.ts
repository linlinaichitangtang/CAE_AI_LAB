/**
 * Electronics Package Analysis API - Frontend wrapper for backend electronics commands
 * 封装电子封装分析相关的后端API
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export interface ElectronicsThermalConfig {
  ambient_temperature: number
  convection_coeff: number
  board_thickness: number
  num_layers: number
  layer_materials: string[]
  trace_layers: TraceLayer[]
  power_dissipation: number
  heatsink_enabled: boolean
  heatsink_params: HeatsinkParams | null
}

export interface TraceLayer {
  layer_id: number
  thickness: number
  copper_weight: number
  trace_pattern: string
}

export interface HeatsinkParams {
  length: number
  width: number
  height: number
  num_fins: number
  thermal_resistance: number
}

export interface ElectronicsStructuralConfig {
  analysis_type: string
  enforce_zero_disp: BoundaryConfig | null
  temperature_field: [number, number][] | null
}

export interface BoundaryConfig {
  face: string
  u1: number
  u2: number
  u3: number
}

export interface ElectronicsMaterial {
  name: string
  category: string
  thermal_conductivity: number
  specific_heat: number
  density: number
  youngs_modulus: number
  poissions_ratio: number
  cte: number
  electrical_conductivity: number
}

export interface ElectronicComponent {
  name: string
  component_type: string
  position: [number, number, number]
  dimensions: [number, number, number]
  power: number
  material: string
}

export interface HotSpot {
  location: [number, number, number]
  temperature: number
  component: string
}

export interface SolderJointResult {
  joint_id: string
  location: [number, number, number]
  shear_stress: number
  fatigue_life_cycles: number
  reliability_score: number
}

export interface ElectronicsResults {
  temperature_field: [number, number][]
  displacement: [number, [number, number, number]][]
  stress: [number, [number, number, number, number, number, number]][]
  von_mises: [number, number][]
  max_temperature: number
  max_displacement: number
  max_stress: number
  max_von_mises: number
  hotspot_locations: HotSpot[]
  solder_joint_stress: SolderJointResult[]
}

export interface ElectronicsAnalysisJob {
  id: string
  name: string
  package_type: string
  thermal_config: ElectronicsThermalConfig
  structural_config: ElectronicsStructuralConfig
  materials: ElectronicsMaterial[]
  components: ElectronicComponent[]
  results: ElectronicsResults | null
  status: string
}

export interface ElectronicsTemplate {
  id: string
  name: string
  description: string
  category: string
  config: ElectronicsAnalysisJob
}

// ============ API 函数 ============

/**
 * 获取电子材料库
 */
export async function getElectronicsMaterialLibrary(): Promise<ElectronicsMaterial[]> {
  return invoke('get_material_library')
}

/**
 * 获取电子封装分析模板
 */
export async function getElectronicsTemplates(): Promise<ElectronicsTemplate[]> {
  return invoke('get_analysis_templates')
}

/**
 * 运行电子封装分析
 */
export async function runElectronicsAnalysis(
  job: ElectronicsAnalysisJob
): Promise<ElectronicsResults> {
  return invoke('run_electronics_analysis', { job })
}
