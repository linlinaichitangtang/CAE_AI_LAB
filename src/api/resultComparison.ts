/**
 * V2.0-006: 多尺度结果对比面板
 * CAELab V2.0 - 多尺度工作流编排器
 *
 * 提供多尺度计算结果的对比分析、一致性检查、下钻追踪及数据导出功能。
 */
import { invoke } from '@tauri-apps/api/core';

// ============ 类型定义 ============

/** 对比指标中各尺度的数值 */
export interface RcMetricValue {
  /** 尺度名称 */
  scale: string;
  /** 数值 */
  value: number;
  /** 单位 */
  unit: string;
  /** 不确定度 */
  uncertainty: number;
}

/** 对比指标 */
export interface ComparisonMetric {
  /** 物理量名称 */
  quantity: string;
  /** 各尺度的数值列表 */
  values: RcMetricValue[];
  /** 各尺度结果是否一致 */
  is_consistent: boolean;
  /** 最大偏差百分比 */
  max_deviation_percent: number;
}

/** 单个尺度的结果摘要 */
export interface ScaleResultSummary {
  /** 尺度名称 */
  scale: string;
  /** 对应的节点 ID */
  node_id: string;
  /** 计算状态 */
  status: string;
  /** 关键结果列表 */
  key_results: { quantity: string; value: number; unit: string }[];
  /** 计算耗时（秒） */
  computation_time_s: number;
  /** 数据大小（MB） */
  data_size_mb: number;
}

/** 跨尺度关联链接类型 */
export type RcLinkType = 'direct' | 'derived' | 'validated';

/** 跨尺度关联链接 */
export interface CrossScaleLink {
  /** 源尺度 */
  source_scale: string;
  /** 目标尺度 */
  target_scale: string;
  /** 源物理量 */
  source_quantity: string;
  /** 目标物理量 */
  target_quantity: string;
  /** 关联类型 */
  link_type: RcLinkType;
  /** 关联置信度 (0~1) */
  confidence: number;
}

/** 对比报告 */
export interface ComparisonReport {
  /** 执行 ID */
  execution_id: string;
  /** 各尺度结果摘要 */
  scale_summaries: ScaleResultSummary[];
  /** 对比指标列表 */
  metrics: ComparisonMetric[];
  /** 跨尺度关联链接 */
  cross_scale_links: CrossScaleLink[];
  /** 整体一致性得分 (0~1) */
  consistency_score: number;
  /** 警告信息列表 */
  warnings: string[];
  /** 报告生成时间 (ISO 8601) */
  generated_at: string;
}

/** 下钻请求参数 */
export interface DrillDownRequest {
  /** 源尺度 */
  source_scale: string;
  /** 源物理量 */
  source_quantity: string;
  /** 源数值 */
  source_value: unknown;
  /** 目标尺度 */
  target_scale: string;
}

/** 导出格式 */
export type RcExportFormat = 'csv' | 'json' | 'pdf';

// ============ API 函数 ============

/**
 * 生成多尺度结果对比报告
 * 对指定执行的所有尺度计算结果进行一致性分析和对比。
 * @param executionId - 工作流执行 ID
 * @returns 完整的对比报告
 */
export async function generateComparison(executionId: string): Promise<ComparisonReport> {
  return invoke<ComparisonReport>('generate_comparison', { executionId });
}

/**
 * 获取指定尺度的结果摘要
 * @param executionId - 工作流执行 ID
 * @param scale - 尺度名称
 * @returns 该尺度的结果摘要
 */
export async function getScaleResultSummary(
  executionId: string,
  scale: string
): Promise<ScaleResultSummary> {
  return invoke<ScaleResultSummary>('get_scale_result_summary', { executionId, scale });
}

/**
 * 下钻追踪：从源尺度的某个数值追溯到目标尺度的详细数据
 * 用于定位跨尺度传递中的一致性问题来源。
 * @param request - 下钻请求参数
 * @returns 目标尺度的详细数据及相关节点列表
 */
export async function drillDown(
  request: DrillDownRequest
): Promise<{ target_data: Record<string, unknown>; related_nodes: string[] }> {
  return invoke<{ target_data: Record<string, unknown>; related_nodes: string[] }>('drill_down', {
    request,
  });
}

/**
 * 获取指定执行的跨尺度关联链接
 * @param executionId - 工作流执行 ID
 * @returns 跨尺度关联链接列表
 */
export async function getCrossScaleLinks(executionId: string): Promise<CrossScaleLink[]> {
  return invoke<CrossScaleLink[]>('get_cross_scale_links', { executionId });
}

/**
 * 导出对比数据为指定格式文件
 * @param executionId - 工作流执行 ID
 * @param format - 导出格式：csv / json / pdf
 * @returns 导出文件的路径
 */
export async function exportComparisonData(
  executionId: string,
  format: RcExportFormat
): Promise<{ file_path: string }> {
  return invoke<{ file_path: string }>('export_comparison_data', { executionId, format });
}
