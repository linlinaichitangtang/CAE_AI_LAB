/**
 * Multiscale Integration API
 * V1.9-001: 四尺度串联集成测试 + V1.9-002: 端到端标准算例
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

/** 尺度步骤类型 */
export type ScaleStep = 'dft' | 'md' | 'phase_field' | 'fe'

/** 集成测试场景 */
export type IntegrationScenario =
  | 'high_strength_steel'
  | 'mg_alloy_creep'
  | 'ceramic_am'

/** 单步配置 */
export interface StepConfig {
  /** 尺度步骤 */
  step: ScaleStep
  /** 是否启用 */
  enabled: boolean
  /** 步骤参数 */
  parameters: Record<string, unknown>
  /** 超时时间（秒） */
  timeout_s: number
}

/** 集成流水线配置 */
export interface IntegrationPipeline {
  /** 测试场景 */
  scenario: IntegrationScenario
  /** 流水线名称 */
  name: string
  /** 步骤列表 */
  steps: StepConfig[]
  /** 是否自动桥接 */
  auto_bridge: boolean
  /** 错误处理策略 */
  error_handling: 'stop' | 'skip' | 'retry'
}

/** 单步执行结果 */
export interface StepResult {
  /** 尺度步骤 */
  step: ScaleStep
  /** 执行状态 */
  status: 'pending' | 'running' | 'completed' | 'failed'
  /** 开始时间 */
  start_time: string
  /** 结束时间 */
  end_time: string
  /** 持续时间（秒） */
  duration_s: number
  /** 步骤输出 */
  output: Record<string, unknown>
  /** 错误信息 */
  error: string | null
  /** 桥接输出 */
  bridge_output: Record<string, unknown> | null
}

/** 最终度量 */
export interface FinalMetric {
  /** 物理量名称 */
  quantity: string
  /** 计算值 */
  value: number
  /** 单位 */
  unit: string
  /** 不确定度 */
  uncertainty: number
}

/** 文献对比条目 */
export interface LiteratureComparison {
  /** 物理量名称 */
  quantity: string
  /** 计算值 */
  computed: number
  /** 参考值 */
  reference: number
  /** 误差百分比 */
  error_percent: number
  /** 文献来源 */
  source: string
}

/** 集成流水线结果 */
export interface IntegrationResult {
  /** 流水线 ID */
  pipeline_id: string
  /** 测试场景 */
  scenario: IntegrationScenario
  /** 整体状态 */
  status: 'running' | 'completed' | 'failed'
  /** 各步骤结果 */
  steps: StepResult[]
  /** 总耗时（秒） */
  total_time_s: number
  /** 最终度量 */
  final_metrics: FinalMetric[]
  /** 文献对比 */
  literature_comparison: LiteratureComparison[]
}

/** 文献引用信息 */
export interface CaseReference {
  /** DOI */
  doi: string
  /** 作者 */
  authors: string
  /** 发表年份 */
  year: number
  /** 期刊 */
  journal: string
}

/** 预期度量 */
export interface ExpectedMetric {
  /** 物理量名称 */
  quantity: string
  /** 期望值 */
  value: number
  /** 单位 */
  unit: string
  /** 容差 */
  tolerance: number
}

/** 端到端标准算例 */
export interface EndToEndCase {
  /** 算例 ID */
  id: string
  /** 算例名称 */
  name: string
  /** 中文名称 */
  name_zh: string
  /** 所属场景 */
  scenario: IntegrationScenario
  /** 算例描述 */
  description: string
  /** 材料体系 */
  material_system: string
  /** 涉及尺度 */
  scales: ScaleStep[]
  /** 文献引用 */
  reference: CaseReference
  /** 预期度量 */
  expected_metrics: ExpectedMetric[]
  /** 难度等级 */
  difficulty: 'beginner' | 'intermediate' | 'advanced'
}

// ============ API 函数 ============

/**
 * 运行多尺度集成流水线
 * @param config - 集成流水线配置
 * @returns 包含 pipeline_id 的结果
 */
export async function runIntegrationPipeline(
  config: IntegrationPipeline
): Promise<{ pipeline_id: string }> {
  return invoke<{ pipeline_id: string }>('run_integration_pipeline', { config })
}

/**
 * 获取流水线执行状态
 * @param pipelineId - 流水线 ID
 * @returns 集成结果
 */
export async function getPipelineStatus(
  pipelineId: string
): Promise<IntegrationResult> {
  return invoke<IntegrationResult>('get_pipeline_status', { pipelineId })
}

/**
 * 获取指定步骤的执行结果
 * @param pipelineId - 流水线 ID
 * @param step - 尺度步骤
 * @returns 步骤结果
 */
export async function getStepResult(
  pipelineId: string,
  step: ScaleStep
): Promise<StepResult> {
  return invoke<StepResult>('get_step_result', { pipelineId, step })
}

/**
 * 重试失败的步骤
 * @param pipelineId - 流水线 ID
 * @param step - 尺度步骤
 * @returns 重试后的步骤结果
 */
export async function retryStep(
  pipelineId: string,
  step: ScaleStep
): Promise<StepResult> {
  return invoke<StepResult>('retry_step', { pipelineId, step })
}

/**
 * 获取端到端标准算例列表
 * @returns 端到端算例数组
 */
export async function listEndToEndCases(): Promise<EndToEndCase[]> {
  return invoke<EndToEndCase[]>('list_end_to_end_cases')
}

/**
 * 运行端到端标准算例
 * @param caseId - 算例 ID
 * @param customParams - 自定义参数（可选）
 * @returns 集成结果
 */
export async function runEndToEndCase(
  caseId: string,
  customParams?: Record<string, unknown>
): Promise<IntegrationResult> {
  return invoke<IntegrationResult>('run_end_to_end_case', { caseId, customParams })
}

/**
 * 与文献数据进行对比
 * @param pipelineId - 流水线 ID
 * @returns 文献对比结果数组
 */
export async function compareWithLiterature(
  pipelineId: string
): Promise<IntegrationResult['literature_comparison']> {
  return invoke<IntegrationResult['literature_comparison']>('compare_with_literature', { pipelineId })
}
