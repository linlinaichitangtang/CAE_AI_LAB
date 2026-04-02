/**
 * Advanced Material Models API - V1.4-004
 * 粘塑性/形状记忆合金(SMA)模块
 * 封装Perzyna/Chaboche/Anand粘塑性模型和Liang-Rogers/Brinson/Tanaka/Lagoudas SMA模型
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

/** 粘塑性模型类型 */
export type ViscoplasticModel = 'perzyna' | 'chaboche' | 'anand'

/** 形状记忆合金模型类型 */
export type SMAModel = 'liang_rogers' | 'brinson' | 'tanaka' | 'lagoudas'

/** 粘塑性材料属性 */
export interface ViscoplasticMaterial {
  name: string
  E: number                          // 弹性模量 (Pa)
  nu: number                         // 泊松比
  density: number                    // 密度 (kg/m^3)
  yield_stress: number               // 屈服应力 (Pa)
  hardening_modulus: number          // 硬化模量 (Pa)
  n_power: number                    // Perzyna幂指数
  C1: number                         // Chaboche背应力系数 C1 (Pa)
  gamma1: number                     // Chaboche背应力演化速率 gamma1
  Q_inf: number                      // 各向同性硬化饱和值 (Pa)
  b_kinematic: number                // 运动硬化速率参数
  temperature_dependent: boolean     // 是否温度相关
}

/** 形状记忆合金材料属性 */
export interface SMAMaterial {
  name: string
  E_austenite: number                // 奥氏体弹性模量 (Pa)
  E_martensite: number               // 马氏体弹性模量 (Pa)
  nu: number                         // 泊松比
  density: number                    // 密度 (kg/m^3)
  Mf: number                         // 马氏体相变开始温度 (K)
  Ms: number                         // 马氏体相变结束温度 (K)
  As: number                         // 奥氏体相变开始温度 (K)
  Af: number                         // 奥氏体相变结束温度 (K)
  C_M: number                        // 马氏体相变温度-应力斜率 (K/Pa)
  C_A: number                        // 奥氏体相变温度-应力斜率 (K/Pa)
  epsilon_L: number                  // 最大可恢复应变
  sigma_s: number                    // 相变开始应力 (Pa)
  sigma_f: number                    // 相变结束应力 (Pa)
}

/** 粘塑性分析配置 */
export interface ViscoplasticConfig {
  project_id: string
  model: ViscoplasticModel
  material: ViscoplasticMaterial
  applied_stress: number             // 施加应力 (Pa)
  strain_rate: number                // 应变速率 (1/s)
  temperature: number                // 温度 (K)
  cycles: number                     // 循环次数
}

/** SMA分析配置 */
export interface SMAConfig {
  project_id: string
  model: SMAModel
  material: SMAMaterial
  applied_stress: number             // 施加应力 (Pa)
  temperature: number                // 温度 (K)
  cycles: number                     // 循环次数
  max_strain: number                 // 最大应变
}

/** 应力-应变数据点 */
export interface StressStrainPoint {
  stress: number
  strain: number
}

/** 应变速率效应数据点 */
export interface StrainRateEffectPoint {
  strain_rate: number
  yield_stress: number
}

/** 应力松弛数据点 */
export interface RelaxationPoint {
  time: number
  stress: number
}

/** 滞回环数据 */
export interface HysteresisLoop {
  loop_number: number
  loading: StressStrainPoint[]
  unloading: StressStrainPoint[]
}

/** 马氏体体积分数数据点 */
export interface MartensiteFractionPoint {
  temperature: number
  fraction: number
}

/** 粘塑性分析结果 */
export interface ViscoplasticResult {
  success: boolean
  stress_strain_curve: StressStrainPoint[]
  strain_rate_effect: StrainRateEffectPoint[]
  relaxation_curve: RelaxationPoint[]
  creep_ratcheting: Array<{
    cycle: number
    accumulated_strain: number
  }>
  accumulated_damage: number
}

/** SMA分析结果 */
export interface SMAResult {
  success: boolean
  stress_strain_curve: StressStrainPoint[]
  hysteresis_loops: HysteresisLoop[]
  martensite_fraction_curve: MartensiteFractionPoint[]
  transformation_strain: number      // 相变应变
  dissipated_energy: number          // 耗散能 (J/m^3)
  shape_recovery_ratio: number       // 形状恢复率 (0~1)
  cyclic_degradation: Array<{
    cycle: number
    recovery_ratio: number
    dissipated_energy: number
  }>
}

/** 高级材料模板 */
export interface AdvancedMaterialTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  model: ViscoplasticModel | SMAModel
  typical_application: string
}

// ============ API 函数 ============

/**
 * 运行粘塑性分析
 * @param config 粘塑性分析配置
 * @returns 粘塑性分析结果
 */
export async function runViscoplasticAnalysis(config: ViscoplasticConfig): Promise<ViscoplasticResult> {
  return invoke<ViscoplasticResult>('run_viscoplastic_analysis', { config })
}

/**
 * 运行形状记忆合金分析
 * @param config SMA分析配置
 * @returns SMA分析结果
 */
export async function runSMAAnalysis(config: SMAConfig): Promise<SMAResult> {
  return invoke<SMAResult>('run_sma_analysis', { config })
}

/**
 * 获取高级材料分析模板列表
 * @returns 高级材料模板数组
 */
export async function getAdvancedMaterialTemplates(): Promise<AdvancedMaterialTemplate[]> {
  return invoke<AdvancedMaterialTemplate[]>('get_advanced_material_templates')
}

/**
 * 获取粘塑性材料库
 * @returns 粘塑性材料数组
 */
export async function getViscoplasticMaterialLibrary(): Promise<ViscoplasticMaterial[]> {
  return invoke<ViscoplasticMaterial[]>('get_viscoplastic_material_library')
}

/**
 * 获取形状记忆合金材料库
 * @returns SMA材料数组
 */
export async function getSMAMaterialLibrary(): Promise<SMAMaterial[]> {
  return invoke<SMAMaterial[]>('get_sma_material_library')
}
