/**
 * CAELab V2.0 - 自适应精度控制 API
 * V2.0-008: 多尺度工作流编排器 - 精度控制模块
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

/** 自适应精度策略 */
export type ApStrategy = 'auto_refine' | 'increase_sampling' | 'adaptive_mesh' | 'ensemble'

/** 精度指标检查结果 */
export interface ApMetricCheck {
  /** 指标名称 */
  metric_name: string
  /** 当前值 */
  current_value: number
  /** 阈值 */
  threshold: number
  /** 是否在容差范围内 */
  is_within_tolerance: boolean
  /** 偏差百分比 */
  deviation_percent: number
}

/** 精度调整动作 */
export interface ApAdjustment {
  /** 节点 ID */
  node_id: string
  /** 关联尺度 */
  scale: string
  /** 采用的策略 */
  strategy: ApStrategy
  /** 调整的参数名 */
  parameter: string
  /** 调整前的值 */
  old_value: number
  /** 调整后的值 */
  new_value: number
  /** 调整原因 */
  reason: string
}

/** 精度控制整体状态 */
export type ApOverallStatus = 'acceptable' | 'adjusting' | 'converged' | 'failed'

/** 精度控制结果 */
export interface ApControlResult {
  /** 关联的执行 ID */
  execution_id: string
  /** 指标检查结果列表 */
  checks: ApMetricCheck[]
  /** 执行的调整列表 */
  adjustments: ApAdjustment[]
  /** 整体状态 */
  overall_status: ApOverallStatus
  /** 当前迭代次数 */
  iteration: number
  /** 最大迭代次数 */
  max_iterations: number
}

/** 精度控制配置 */
export interface ApControlConfig {
  /** 关联的执行 ID */
  execution_id: string
  /** 是否启用自适应精度控制 */
  enable: boolean
  /** 容差百分比 */
  tolerance_percent: number
  /** 最大迭代次数 */
  max_iterations: number
  /** 允许使用的策略列表 */
  strategies: ApStrategy[]
  /** 是否自动应用调整 */
  auto_apply: boolean
}

// ============ API 函数 ============

/**
 * 评估当前执行精度
 * @param executionId - 执行 ID
 * @returns 精度控制结果
 */
export async function evaluatePrecision(executionId: string): Promise<ApControlResult> {
  return invoke<ApControlResult>('evaluate_precision', { executionId })
}

/**
 * 手动应用精度调整
 * @param executionId - 执行 ID
 * @param adjustments - 调整列表
 * @returns 应用结果
 */
export async function applyAdjustments(
  executionId: string,
  adjustments: ApAdjustment[]
): Promise<{ success: boolean; applied_count: number }> {
  return invoke<{ success: boolean; applied_count: number }>('apply_adjustments', {
    executionId,
    adjustments,
  })
}

/**
 * 运行自适应精度循环
 * @param config - 精度控制配置
 * @returns 每次迭代的控制结果列表
 */
export async function runAdaptiveLoop(config: ApControlConfig): Promise<ApControlResult[]> {
  return invoke<ApControlResult[]>('run_adaptive_loop', { config })
}

/**
 * 获取精度控制历史记录
 * @param executionId - 执行 ID
 * @returns 历史控制结果列表
 */
export async function getPrecisionHistory(executionId: string): Promise<ApControlResult[]> {
  return invoke<ApControlResult[]>('get_precision_history', { executionId })
}

/**
 * 更新精度控制配置
 * @param config - 新的精度控制配置
 */
export async function updateControlConfig(config: ApControlConfig): Promise<void> {
  return invoke<void>('update_control_config', { config })
}

/**
 * 获取推荐的精度策略
 * @param scale - 分析尺度
 * @param metric - 指标名称
 * @returns 推荐策略及原因
 */
export async function getRecommendedStrategy(
  scale: string,
  metric: string
): Promise<{ strategy: ApStrategy; reason: string }> {
  return invoke<{ strategy: ApStrategy; reason: string }>('get_recommended_strategy', {
    scale,
    metric,
  })
}
