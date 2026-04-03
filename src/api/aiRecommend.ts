/**
 * CAE API - AI Parameter Recommendation (Multi-scale)
 * V1.9-007: AI 参数推荐（多尺度场景）
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

/** 推荐适用尺度 */
export type RecommendationScope = 'dft' | 'md' | 'phase_field' | 'fe' | 'multiscale'

/** 推荐约束条件 */
export interface RecommendationConstraint {
  /** 约束名称 */
  name: string
  /** 约束上限值 */
  max_value: number
}

/** 历史数据范围 */
export interface HistoricalDataRange {
  /** 已有流水线数量 */
  pipeline_count: number
  /** 日期范围 */
  date_range: { start: string; end: string }
}

/** 推荐上下文 */
export interface RecommendationContext {
  /** 材料体系 */
  material_system: string
  /** 目标性能 */
  target_property: string
  /** 计算尺度 */
  scale: RecommendationScope
  /** 约束条件列表 */
  constraints: RecommendationConstraint[]
  /** 历史数据范围 */
  historical_data_range: HistoricalDataRange
}

/** 备选参数值 */
export interface ParameterAlternative {
  /** 备选值 */
  value: number
  /** 预期性能 */
  expected_performance: number
}

/** 单个参数推荐 */
export interface ParameterRecommendation {
  /** 参数名称 */
  parameter_name: string
  /** 推荐值 */
  recommended_value: number
  /** 置信度 (0-1) */
  confidence: number
  /** 推荐理由 */
  rationale: string
  /** 备选方案列表 */
  alternatives: ParameterAlternative[]
  /** 适用尺度 */
  scale: RecommendationScope
}

/** 推荐结果 */
export interface RecommendationResult {
  /** 参数推荐列表 */
  recommendations: ParameterRecommendation[]
  /** 整体置信度 (0-1) */
  overall_confidence: number
  /** 基于流水线数量 */
  based_on_pipeline_count: number
  /** 预估提升幅度描述 */
  estimated_improvement: string
  /** 警告信息列表 */
  warnings: string[]
}

/** 推荐历史记录 */
export interface RecommendationHistoryItem {
  /** 推荐上下文 */
  context: RecommendationContext
  /** 推荐结果 */
  result: RecommendationResult
  /** 是否已应用 */
  applied: boolean
  /** 实际提升值 */
  actual_improvement: number | null
  /** 记录时间戳 */
  timestamp: string
}

/** 模型性能指标 */
export interface ModelPerformance {
  /** 模型名称 */
  model_name: string
  /** 准确率 (0-1) */
  accuracy: number
  /** 训练流水线数量 */
  training_pipelines: number
  /** 最后训练时间 */
  last_trained: string
  /** 适用尺度列表 */
  scales: RecommendationScope[]
}

/** 用户反馈评分 */
export type FeedbackRating = 'helpful' | 'not_helpful' | 'partially'

// ============ API 函数 ============

/**
 * 获取 AI 参数推荐
 * 基于材料体系、目标性能和历史数据，返回多尺度参数推荐结果
 */
export async function getRecommendations(
  context: RecommendationContext
): Promise<RecommendationResult> {
  return invoke<RecommendationResult>('get_recommendations', { context })
}

/**
 * 获取推荐历史记录
 * 返回历史推荐记录列表，可指定返回数量上限
 */
export async function getRecommendationHistory(
  limit?: number
): Promise<RecommendationHistoryItem[]> {
  return invoke<RecommendationHistoryItem[]>('get_recommendation_history', { limit })
}

/**
 * 应用推荐参数
 * 将推荐参数值应用到指定的流水线配置中，返回更新后的完整配置
 */
export async function applyRecommendation(
  recommendation_id: string,
  pipeline_config: Record<string, unknown>
): Promise<{ updated_config: Record<string, unknown> }> {
  return invoke<{ updated_config: Record<string, unknown> }>(
    'apply_recommendation',
    { recommendationId: recommendation_id, pipelineConfig: pipeline_config }
  )
}

/**
 * 获取推荐模型性能
 * 返回各推荐模型的准确率、训练数据量和适用尺度信息
 */
export async function getModelPerformance(): Promise<ModelPerformance[]> {
  return invoke<ModelPerformance[]>('get_model_performance')
}

/**
 * 反馈推荐效果
 * 对已应用的推荐进行评分，帮助模型持续优化推荐质量
 */
export async function feedbackRecommendation(
  recommendation_id: string,
  rating: FeedbackRating,
  actual_value?: number
): Promise<void> {
  return invoke<void>(
    'feedback_recommendation',
    { recommendationId: recommendation_id, rating, actualValue: actual_value }
  )
}
