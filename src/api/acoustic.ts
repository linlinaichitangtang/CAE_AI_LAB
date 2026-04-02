/**
 * Acoustic-Structural Coupling API
 * V1.3-010: 声-结构耦合分析 API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

/** 声学材料属性 */
export interface AcousticMaterial {
  name: string
  density: number                   // kg/m^3
  sound_speed: number               // m/s
  absorption_coefficient: number    // 0-1
  impedance: number                 // Pa*s/m (声阻抗)
}

/** 结构材料属性 */
export interface StructuralMaterial {
  name: string
  E: number                         // Pa (弹性模量)
  nu: number                        // 泊松比
  density: number                   // kg/m^3
  loss_factor: number               // 损耗因子
}

/** 声腔属性 */
export interface CavityProperties {
  volume: number                    // m^3
  surface_area: number              // m^2
}

/** 频率范围 */
export interface FrequencyRange {
  min: number                       // Hz
  max: number                       // Hz
  step: number                      // Hz
}

/** 声-结构耦合分析配置 */
export interface AcousticConfig {
  project_id: string
  analysis_type: 'modal_coupling' | 'harmonic_response' | 'transient_acoustic'
  acoustic_material: AcousticMaterial
  structural_material: StructuralMaterial
  cavity_properties: CavityProperties
  frequency_range: FrequencyRange
  damping_ratio: number
  boundary_condition: 'rigid' | 'absorbent' | 'impedance'
}

/** 模态振型 */
export interface ModeShape {
  frequency: number                 // Hz
  pressure_field: number[][]
  displacement_field: number[][]
}

/** 频率响应数据点 */
export interface FrequencyResponsePoint {
  frequency: number                 // Hz
  spl: number                       // dB
  displacement: number              // mm
}

/** 声-结构耦合分析结果 */
export interface AcousticResult {
  success: boolean
  natural_frequencies: number[]     // Hz
  mode_shapes: ModeShape[]
  frequency_response: FrequencyResponsePoint[]
  max_spl: number                   // dB
  avg_spl: number                   // dB
  radiation_efficiency: number
  transmission_loss: number         // dB
}

/** 声学分析模板 */
export interface AcousticTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  typical_frequency_range: [number, number]
}

// ============ API 函数 ============

/**
 * 运行声-结构耦合分析
 */
export async function runAcousticAnalysis(config: AcousticConfig): Promise<AcousticResult> {
  return invoke<AcousticResult>('run_acoustic_analysis', { config })
}

/**
 * 获取声学分析模板列表
 */
export async function getAcousticTemplates(): Promise<AcousticTemplate[]> {
  return invoke<AcousticTemplate[]>('get_acoustic_templates')
}

/**
 * 计算声压级 (SPL)
 * @param pressure 声压 (Pa)
 */
export async function calculateSoundPressureLevel(pressure: number): Promise<number> {
  return invoke<number>('calculate_sound_pressure_level', { pressure })
}

/**
 * 获取声学材料库
 */
export async function getAcousticMaterialLibrary(): Promise<AcousticMaterial[]> {
  return invoke<AcousticMaterial[]>('get_acoustic_material_library')
}
