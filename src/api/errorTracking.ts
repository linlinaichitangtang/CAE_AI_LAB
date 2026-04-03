/**
 * Error Propagation Tracking Framework API
 * V1.8-004: 误差传播追踪框架 (error bars, uncertainty propagation)
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type ErrorSource =
  | 'numerical'
  | 'discretization'
  | 'coarse_graining'
  | 'unit_conversion'
  | 'model'
  | 'measurement'

export interface ErrorContribution {
  source: ErrorSource
  scale: string
  step_id: string
  description: string
  absolute_error: number
  relative_error: number
  confidence_interval: [number, number]
}

export interface PropagationStep {
  step_id: string
  from_scale: string
  to_scale: string
  method: string
  input_uncertainty: number
  output_uncertainty: number
  contributions: ErrorContribution[]
  timestamp: string
}

export interface ErrorReport {
  pipeline_id: string
  total_steps: number
  cumulative_uncertainty: number
  steps: PropagationStep[]
  final_result: {
    value: number
    uncertainty: number
    confidence_95: [number, number]
  }
  sensitivity_analysis: {
    parameter: string
    sensitivity: number
  }[]
}

export interface ErrorTrackingConfig {
  pipeline_name: string
  scales: string[]
  enable_monte_carlo: boolean
  mc_samples: number
}

// ============ API 函数 ============

/**
 * 创建误差追踪管线
 */
export async function createErrorPipeline(
  config: ErrorTrackingConfig
): Promise<{ pipeline_id: string }> {
  return invoke<{ pipeline_id: string }>('create_error_pipeline', { config })
}

/**
 * 添加误差贡献项
 */
export async function addErrorContribution(
  pipeline_id: string,
  contribution: ErrorContribution
): Promise<void> {
  return invoke<void>('add_error_contribution', { pipelineId: pipeline_id, contribution })
}

/**
 * 获取传播步骤列表
 */
export async function getPropagationSteps(
  pipeline_id: string
): Promise<PropagationStep[]> {
  return invoke<PropagationStep[]>('get_propagation_steps', { pipelineId: pipeline_id })
}

/**
 * 生成误差报告
 */
export async function generateErrorReport(
  pipeline_id: string
): Promise<ErrorReport> {
  return invoke<ErrorReport>('generate_error_report', { pipelineId: pipeline_id })
}

/**
 * 运行灵敏度分析
 */
export async function runSensitivityAnalysis(
  pipeline_id: string,
  parameters: string[]
): Promise<{ parameter: string; sensitivity: number; rank: number }[]> {
  return invoke<{ parameter: string; sensitivity: number; rank: number }[]>(
    'run_sensitivity_analysis',
    { pipelineId: pipeline_id, parameters }
  )
}
