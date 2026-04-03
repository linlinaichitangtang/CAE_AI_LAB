/**
 * Workflow Template API
 * V1.9-003: 多尺度工作流模板库 + V1.9-004: 模板市场
 */

import { invoke } from '@tauri-apps/api/core'
import type { ScaleStep } from './multiscaleIntegration'

// ============ 类型定义 ============

/** 模板分类 */
export type TemplateCategory =
  | 'solidification'
  | 'creep'
  | 'fatigue'
  | 'phase_transformation'
  | 'fracture'
  | 'diffusion'
  | 'custom'

/** 工作流步骤参数定义 */
export interface WorkflowStepParameter {
  /** 参数名称 */
  name: string
  /** 参数类型 */
  type: string
  /** 默认值 */
  default_value: unknown
  /** 最小值（数值类型） */
  min?: number
  /** 最大值（数值类型） */
  max?: number
  /** 可选值列表 */
  options?: string[]
}

/** 桥接到下一步的配置 */
export interface BridgeToNext {
  /** 桥接方法 */
  method: string
  /** 桥接配置 */
  config: Record<string, unknown>
}

/** 工作流步骤 */
export interface WorkflowStep {
  /** 步骤 ID */
  id: string
  /** 所属尺度 */
  scale: ScaleStep
  /** 步骤名称 */
  name: string
  /** 步骤描述 */
  description: string
  /** 参数列表 */
  parameters: WorkflowStepParameter[]
  /** 到下一步的桥接配置 */
  bridge_to_next: BridgeToNext | null
}

/** 模板评分信息 */
export interface TemplateRating {
  /** 平均评分 */
  avg: number
  /** 评分人数 */
  count: number
}

/** 工作流模板 */
export interface WorkflowTemplate {
  /** 模板 ID */
  id: string
  /** 模板名称 */
  name: string
  /** 中文名称 */
  name_zh: string
  /** 模板分类 */
  category: TemplateCategory
  /** 模板描述 */
  description: string
  /** 作者 */
  author: string
  /** 版本号 */
  version: string
  /** 工作流步骤 */
  steps: WorkflowStep[]
  /** 标签 */
  tags: string[]
  /** 难度等级 */
  difficulty: 'beginner' | 'intermediate' | 'advanced'
  /** 预估耗时（分钟） */
  estimated_time_min: number
  /** 下载次数 */
  downloads: number
  /** 评分信息 */
  rating: TemplateRating
  /** 是否为官方模板 */
  is_official: boolean
  /** 创建时间 */
  created_at: string
  /** 更新时间 */
  updated_at: string
}

/** 模板市场查询参数 */
export interface TemplateMarketQuery {
  /** 按分类筛选 */
  category?: TemplateCategory
  /** 关键词搜索 */
  keyword?: string
  /** 排序方式 */
  sort_by: 'rating' | 'downloads' | 'newest' | 'name'
  /** 返回数量限制 */
  limit?: number
  /** 偏移量 */
  offset?: number
}

/** 模板评价 */
export interface TemplateReview {
  /** 用户 ID */
  user_id: string
  /** 模板 ID */
  template_id: string
  /** 评分 */
  rating: number
  /** 评论内容 */
  comment: string
  /** 创建时间 */
  created_at: string
}

/** 模板提交信息 */
export interface TemplateSubmission {
  /** 模板名称 */
  name: string
  /** 中文名称 */
  name_zh: string
  /** 模板分类 */
  category: TemplateCategory
  /** 模板描述 */
  description: string
  /** 工作流步骤 */
  steps: WorkflowStep[]
  /** 标签 */
  tags: string[]
}

// ============ API 函数 ============

/**
 * 查询模板市场列表
 * @param query - 查询参数
 * @returns 模板列表及总数
 */
export async function listTemplates(
  query: TemplateMarketQuery
): Promise<{ templates: WorkflowTemplate[]; total: number }> {
  return invoke<{ templates: WorkflowTemplate[]; total: number }>('list_templates', { query })
}

/**
 * 获取模板详情
 * @param id - 模板 ID
 * @returns 完整的工作流模板
 */
export async function getTemplateDetail(
  id: string
): Promise<WorkflowTemplate> {
  return invoke<WorkflowTemplate>('get_template_detail', { id })
}

/**
 * 下载模板
 * @param id - 模板 ID
 * @returns 模板文件路径
 */
export async function downloadTemplate(
  id: string
): Promise<{ file_path: string }> {
  return invoke<{ file_path: string }>('download_template', { id })
}

/**
 * 上传自定义模板
 * @param submission - 模板提交信息
 * @returns 模板 ID 及审核状态
 */
export async function uploadTemplate(
  submission: TemplateSubmission
): Promise<{ template_id: string; status: 'pending_review' | 'published' }> {
  return invoke<{ template_id: string; status: 'pending_review' | 'published' }>(
    'upload_template',
    { submission }
  )
}

/**
 * 评价模板
 * @param templateId - 模板 ID
 * @param rating - 评分（1-5）
 * @param comment - 评论内容
 * @returns 评价记录
 */
export async function rateTemplate(
  templateId: string,
  rating: number,
  comment: string
): Promise<TemplateReview> {
  return invoke<TemplateReview>('rate_template', { templateId, rating, comment })
}

/**
 * 获取模板评价列表
 * @param templateId - 模板 ID
 * @returns 评价记录数组
 */
export async function getTemplateReviews(
  templateId: string
): Promise<TemplateReview[]> {
  return invoke<TemplateReview[]>('get_template_reviews', { templateId })
}

/**
 * 从已有流水线创建模板
 * @param pipelineId - 流水线 ID
 * @param name - 模板名称
 * @param category - 模板分类
 * @returns 新创建的模板 ID
 */
export async function createTemplateFromPipeline(
  pipelineId: string,
  name: string,
  category: TemplateCategory
): Promise<{ template_id: string }> {
  return invoke<{ template_id: string }>('create_template_from_pipeline', { pipelineId, name, category })
}
