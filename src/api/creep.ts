/**
 * Creep Analysis API - V1.4-003
 * 蠕变分析模块：Norton幂律 / Bailey-Norton / 时间硬化 / 应变硬化 / Andrade / Garofalo
 * 封装蠕变分析、曲线拟合、剩余寿命评估、Larson-Miller参数计算等后端API
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

/** 蠕变模型类型 */
export type CreepModel =
  | 'norton_power'
  | 'bailey_norton'
  | 'time_hardening'
  | 'strain_hardening'
  | 'andrade'
  | 'garofalo'

/** 蠕变材料属性 */
export interface CreepMaterial {
  name: string
  E: number                          // 弹性模量 (Pa)
  nu: number                         // 泊松比
  density: number                    // 密度 (kg/m^3)
  A: number                          // Norton系数
  n: number                          // 应力指数
  Q: number                          // 激活能 (J/mol)
  R: number                          // 气体常数 (J/(mol*K))
  temperature_reference: number      // 参考温度 (K)
  creep_strain_coefficients: Record<string, number>  // 蠕变应变系数
}

/** 蠕变分析配置 */
export interface CreepConfig {
  project_id: string
  model: CreepModel
  material: CreepMaterial
  applied_stress: number             // 施加应力 (Pa)
  temperature: number                // 温度 (K)
  time_range: {
    min: number
    max: number
    step: number
  }
  hardening_type: 'time' | 'strain' | 'both'
}

/** 蠕变曲线数据点 */
export interface CreepCurvePoint {
  time: number
  strain: number
  strain_rate: number
}

/** 蠕变分析结果 */
export interface CreepResult {
  success: boolean
  creep_strain_curve: CreepCurvePoint[]
  steady_state_rate: number          // 稳态蠕变速率 (1/s)
  rupture_time: number               // 断裂时间 (s)
  min_creep_rate: number             // 最小蠕变速率 (1/s)
  time_to_1pct_strain: number        // 达到1%应变的时间 (s)
  time_to_5pct_strain: number        // 达到5%应变的时间 (s)
  larson_miller_parameter: number    // Larson-Miller参数
  remaining_life_fraction: number    // 剩余寿命分数 (0~1)
}

/** 蠕变分析模板 */
export interface CreepTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  model: CreepModel
  typical_application: string
  temperature_range: string
}

/** 蠕变曲线拟合结果 */
export interface CreepFitResult {
  coefficients: Record<string, number>
  r_squared: number
}

/** 剩余寿命预测结果 */
export interface RemainingLifeResult {
  remaining_life: number             // 剩余寿命 (s)
  confidence: number                 // 置信度 (0~1)
}

// ============ API 函数 ============

/**
 * 运行蠕变分析
 * @param config 蠕变分析配置
 * @returns 蠕变分析结果
 */
export async function runCreepAnalysis(config: CreepConfig): Promise<CreepResult> {
  return invoke<CreepResult>('run_creep_analysis', { config })
}

/**
 * 拟合蠕变曲线
 * @param experimental_data 实验数据点
 * @param model 蠕变模型类型
 * @returns 拟合系数和决定系数
 */
export async function fitCreepCurve(
  experimental_data: CreepCurvePoint[],
  model: CreepModel
): Promise<CreepFitResult> {
  return invoke<CreepFitResult>('fit_creep_curve', { experimental_data, model })
}

/**
 * 预测剩余寿命
 * @param config 蠕变分析配置
 * @param current_strain 当前累积应变
 * @returns 剩余寿命和置信度
 */
export async function predictRemainingLife(
  config: CreepConfig,
  current_strain: number
): Promise<RemainingLifeResult> {
  return invoke<RemainingLifeResult>('predict_remaining_life', { config, current_strain })
}

/**
 * 计算Larson-Miller参数
 * @param temperature 温度 (K)
 * @param time 时间 (h)
 * @returns Larson-Miller参数值
 */
export async function calculateLarsonMillerParameter(
  temperature: number,
  time: number
): Promise<number> {
  return invoke<number>('calculate_larson_miller_parameter', { temperature, time })
}

/**
 * 获取蠕变分析模板列表
 * @returns 蠕变分析模板数组
 */
export async function getCreepTemplates(): Promise<CreepTemplate[]> {
  return invoke<CreepTemplate[]>('get_creep_templates')
}

/**
 * 获取蠕变材料库
 * @returns 蠕变材料数组
 */
export async function getCreepMaterialLibrary(): Promise<CreepMaterial[]> {
  return invoke<CreepMaterial[]>('get_creep_material_library')
}
