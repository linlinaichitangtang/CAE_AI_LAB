/**
 * Fatigue Analysis API - Frontend wrapper for backend fatigue commands
 * 封装疲劳分析相关的后端API
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export interface SNDataPoint {
  stress: number
  cycles: number
}

export interface SNParams {
  stress_ratio: number
  data_points: SNDataPoint[]
  fatigue_limit: number
  mean_stress_correction: string
}

export interface ENParams {
  strain_amplitude: number
  cyclic_stress_coeff: number
  cyclic_exponent: number
  use_neuber: boolean
}

export interface PSDParams {
  psd_data: string
  rms_stress: number
  target_life: number
}

export interface LoadParams {
  stress_amplitude: number
}

export interface FatigueParams {
  analysis_type: string
  sn_params: SNParams | null
  en_params: ENParams | null
  psd_params: PSDParams | null
  load_type: string
  load: LoadParams
  kt: number
  surface_treatment: string
}

export interface FatigueResults {
  damage: number
  life_cycles: number
  safety_factor: number
  max_stress?: number
  dangerous_node?: number
  allowable_cycles?: number
  damage_distribution?: number[]
}

export interface RainflowCycle {
  amplitude: number
  mean: number
  count: number
}

export interface FatigueTemplate {
  name: string
  description: string
  template_type: string
  params: FatigueParams
}

// ============ API 函数 ============

/**
 * 执行疲劳分析
 */
export async function fatigueAnalysis(params: FatigueParams): Promise<FatigueResults> {
  return invoke('fatigue_analysis', { params })
}

/**
 * 雨流循环计数
 */
export async function rainflowAnalysis(loads: number[]): Promise<RainflowCycle[]> {
  return invoke('rainflow_analysis', { loads })
}

/**
 * 从实验数据拟合S-N曲线
 */
export async function fitSNCurve(
  dataPoints: SNDataPoint[],
  fatigueLimit: number
): Promise<{ log_a: number; m: number; fatigue_limit: number }> {
  return invoke('fit_sn_curve', { dataPoints, fatigueLimit })
}

/**
 * 计算每个节点的损伤值
 */
export async function calculateNodeDamage(
  stressResults: number[],
  snCurveParams: SNParams
): Promise<number[]> {
  return invoke('calculate_node_damage', { stressResults, snCurveParams })
}

/**
 * 获取疲劳分析模板
 */
export async function getFatigueTemplates(): Promise<FatigueTemplate[]> {
  return invoke('get_fatigue_templates')
}

/**
 * 生成疲劳分析INP文件
 */
export async function generateFatigueInpFile(
  meshData: Record<string, unknown>,
  bcData: Record<string, unknown>,
  loadData: Record<string, unknown>,
  fatigueParams: FatigueParams
): Promise<string> {
  return invoke('generate_fatigue_inp_file', { meshData, bcData, loadData, fatigueParams })
}

/**
 * 运行疲劳仿真
 */
export async function runFatigueSimulation(
  inpContent: string,
  workDir: string
): Promise<Record<string, unknown>> {
  return invoke('run_fatigue_simulation', { inpContent, workDir })
}
