/**
 * CAELab V2.0 - 多尺度综合报告生成 API
 * V2.0-007: 多尺度工作流编排器 - 报告生成模块
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

/** 报告章节内容类型 */
export type RgContentType = 'text' | 'table' | 'chart' | 'image'

/** 报告章节 */
export interface RgReportSection {
  /** 章节标题 */
  title: string
  /** 关联的分析尺度，null 表示全局章节 */
  scale: string | null
  /** 内容类型 */
  content_type: RgContentType
  /** 章节数据 */
  data: Record<string, unknown>
  /** 排序序号 */
  order: number
}

/** 报告导出格式 */
export type RgReportFormat = 'pdf' | 'html' | 'markdown'

/** 报告生成配置 */
export interface RgReportConfig {
  /** 关联的执行 ID */
  execution_id: string
  /** 报告标题 */
  title: string
  /** 作者 */
  author: string
  /** 报告章节列表 */
  sections: RgReportSection[]
  /** 是否包含方法论章节 */
  include_methodology: boolean
  /** 是否包含原始数据 */
  include_raw_data: boolean
  /** 是否包含参考文献 */
  include_references: boolean
  /** 导出格式 */
  format: RgReportFormat
}

/** 已生成的报告 */
export interface RgGeneratedReport {
  /** 报告 ID */
  report_id: string
  /** 报告标题 */
  title: string
  /** 导出格式 */
  format: string
  /** 文件存储路径 */
  file_path: string
  /** 页数 */
  page_count: number
  /** 生成时间 (ISO 8601) */
  generated_at: string
  /** 文件大小（字节） */
  file_size_bytes: number
  /** 报告章节列表 */
  sections: RgReportSection[]
}

/** 报告模板 */
export interface RgTemplate {
  /** 模板 ID */
  id: string
  /** 模板名称 */
  name: string
  /** 模板描述 */
  description: string
  /** 预定义章节列表 */
  sections: RgReportSection[]
  /** 是否为默认模板 */
  is_default: boolean
}

// ============ API 函数 ============

/**
 * 生成多尺度综合报告
 * @param config - 报告生成配置
 * @returns 生成的报告信息
 */
export async function generateReport(config: RgReportConfig): Promise<RgGeneratedReport> {
  return invoke<RgGeneratedReport>('generate_report', { config })
}

/**
 * 预览报告（返回 HTML 内容）
 * @param config - 报告生成配置
 * @returns HTML 预览内容
 */
export async function previewReport(config: RgReportConfig): Promise<{ html_content: string }> {
  return invoke<{ html_content: string }>('preview_report', { config })
}

/**
 * 获取可用的报告模板列表
 * @returns 报告模板数组
 */
export async function getReportTemplates(): Promise<RgTemplate[]> {
  return invoke<RgTemplate[]>('get_report_templates')
}

/**
 * 保存自定义报告模板
 * @param template - 报告模板
 * @returns 保存后的模板 ID
 */
export async function saveReportTemplate(template: RgTemplate): Promise<{ template_id: string }> {
  return invoke<{ template_id: string }>('save_report_template', { template })
}

/**
 * 获取报告生成历史
 * @param limit - 返回条数上限，默认不限制
 * @returns 历史报告列表
 */
export async function getReportHistory(limit?: number): Promise<RgGeneratedReport[]> {
  return invoke<RgGeneratedReport[]>('get_report_history', { limit })
}

/**
 * 下载已生成的报告文件
 * @param reportId - 报告 ID
 * @returns 文件存储路径
 */
export async function downloadReport(reportId: string): Promise<{ file_path: string }> {
  return invoke<{ file_path: string }>('download_report', { reportId })
}
