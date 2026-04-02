/**
 * Parametric API - DOE + Sensitivity Analysis
 */

import { invoke } from '@tauri-apps/api/core'

// ============================================================================
// Types
// ============================================================================

export interface DoeParameter {
  name: string
  min: number
  max: number
  levels?: number
}

export type DoeSamplingMethod =
  | 'FullFactorial'
  | 'LatinHypercube'
  | 'Sobol'
  | 'Random'
  | 'CentralComposite'

export interface DoeConfig {
  parameters: DoeParameter[]
  sampling_method: DoeSamplingMethod
  num_samples: number
}

export interface DoeResult {
  samples: Record<string, number>[]
  sampling_method: string
  total_samples: number
}

export interface SensitivityResult {
  parameter_name: string
  sensitivity_score: number   // -1 to 1
  main_effect: number
  interaction_effects: Array<[string, number]>
}

export interface ScanCaseResult {
  case_id: number
  parameter_values: Record<string, number>
  input_file: string
  output_file?: string
  result_file?: string
  success: boolean
  max_stress?: number
  max_displacement?: number
  max_von_mises?: number
  elapsed_time_seconds?: number
  error_message?: string
}

export interface Parameter {
  name: string
  parameter_type: {
    type: 'Discrete' | 'Range' | 'Linspace'
    values?: number[]
    start?: number
    end?: number
    step?: number
    num_points?: number
  }
}

// ============================================================================
// API Functions
// ============================================================================

/** 运行 DOE 研究 */
export async function runDoeStudy(config: DoeConfig): Promise<DoeResult> {
  return await invoke<DoeResult>('run_doe_study', { config })
}

/** 计算灵敏度分析 */
export async function calculateSensitivity(
  results: ScanCaseResult[],
  parameters: Parameter[]
): Promise<SensitivityResult[]> {
  return await invoke<SensitivityResult[]>('calculate_sensitivity', { results, parameters })
}

// ============================================================================
// Sampling method labels
// ============================================================================

export const SAMPLING_METHOD_LABELS: Record<DoeSamplingMethod, string> = {
  FullFactorial: '全因子设计',
  LatinHypercube: '拉丁超立方',
  Sobol: 'Sobol 序列',
  Random: '均匀随机',
  CentralComposite: '中心复合设计',
}

export const SAMPLING_METHOD_DESCRIPTIONS: Record<DoeSamplingMethod, string> = {
  FullFactorial: '遍历所有参数组合，适合参数较少的情况',
  LatinHypercube: '分层随机采样，高效覆盖参数空间',
  Sobol: '低差异序列采样，空间填充性好',
  Random: '简单均匀随机采样',
  CentralComposite: '中心点 + 轴向点 + 因子点，适合响应面拟合',
}
