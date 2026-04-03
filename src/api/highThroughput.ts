/**
 * CAE API - High Throughput Parameter Scan & Screening
 * V1.9-005: 参数扫描自动化
 * V1.9-006: 高通量筛选结果数据库
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

/** 实验设计方法 */
export type DoEMethod = 'full_factorial' | 'latin_hypercube' | 'sobol' | 'random' | 'central_composite'

/** 参数扫描范围 */
export interface ParamRange {
  /** 参数名称 */
  name: string
  /** 最小值 */
  min: number
  /** 最大值 */
  max: number
  /** 步长（连续参数） */
  step?: number
  /** 离散取值列表 */
  values?: number[]
  /** 参数类型 */
  type: 'continuous' | 'discrete'
}

/** 参数扫描配置 */
export interface ScanConfig {
  /** 扫描名称 */
  name: string
  /** 实验设计方法 */
  doe_method: DoEMethod
  /** 参数范围列表 */
  parameters: ParamRange[]
  /** 总组合数 */
  total_combinations: number
  /** 最大并行数 */
  max_parallel: number
  /** 模板 ID */
  template_id?: string
  /** 单次运行超时时间（秒） */
  timeout_per_run_s: number
}

/** 单个扫描任务 */
export interface ScanTask {
  /** 任务 ID */
  task_id: string
  /** 参数组合 */
  params: Record<string, number>
  /** 任务状态 */
  status: 'queued' | 'running' | 'completed' | 'failed'
  /** 运行结果 */
  result: Record<string, unknown> | null
  /** 错误信息 */
  error: string | null
  /** 运行时长（秒） */
  duration_s: number
  /** 开始时间 */
  started_at: string
  /** 完成时间 */
  completed_at: string
}

/** 扫描进度信息 */
export interface ScanProgress {
  /** 已完成数 */
  completed: number
  /** 失败数 */
  failed: number
  /** 总数 */
  total: number
  /** 完成百分比 */
  percent: number
}

/** 扫描结果 */
export interface ScanResult {
  /** 扫描 ID */
  scan_id: string
  /** 扫描配置 */
  config: ScanConfig
  /** 扫描状态 */
  status: 'queued' | 'running' | 'completed' | 'partial'
  /** 任务列表 */
  tasks: ScanTask[]
  /** 进度信息 */
  progress: ScanProgress
  /** 总耗时（秒） */
  total_time_s: number
  /** 结果数据库路径 */
  results_db_path: string
}

/** 查询配置 */
export interface QueryConfig {
  /** 过滤条件列表 */
  filters: { param_name: string; min: number; max: number }[]
  /** 排序字段 */
  sort_by: string
  /** 排序方向 */
  sort_order: 'asc' | 'desc'
  /** 返回行数上限 */
  limit: number
  /** 查询列名列表 */
  columns: string[]
}

/** 查询结果 */
export interface QueryResult {
  /** 数据行 */
  rows: Record<string, unknown>[]
  /** 总行数 */
  total_rows: number
  /** 列名列表 */
  columns: string[]
  /** 查询耗时（毫秒） */
  query_time_ms: number
}

/** 导出配置 */
export interface ExportConfig {
  /** 扫描 ID */
  scan_id: string
  /** 导出格式 */
  format: 'csv' | 'parquet' | 'json' | 'hdf5'
  /** 指定导出列 */
  columns?: string[]
  /** 过滤条件 */
  filters?: Record<string, unknown>
}

/** 参数灵敏度统计 */
export interface ScanStatistics {
  /** 参数名称 */
  param_name: string
  /** 相关系数 */
  correlation: number
  /** 最优值 */
  optimal_value: number
  /** 灵敏度 */
  sensitivity: number
}

/** 扫描仪表盘可视化 URL */
export interface ScanDashboard {
  /** 平行坐标图 URL */
  parallel_coords_url: string
  /** 热力图 URL */
  heatmap_url: string
  /** 散点图 URL */
  scatter_url: string
}

// ============ API 函数 ============

/**
 * 创建参数扫描任务
 * 根据实验设计方法（DOE）生成参数组合并启动批量扫描
 */
export async function createParameterScan(
  config: ScanConfig
): Promise<{ scan_id: string }> {
  return invoke<{ scan_id: string }>('create_parameter_scan', { config })
}

/**
 * 获取扫描状态
 * 返回扫描任务的完整状态，包括所有子任务和进度信息
 */
export async function getScanStatus(
  scan_id: string
): Promise<ScanResult> {
  return invoke<ScanResult>('get_scan_status', { scanId: scan_id })
}

/**
 * 取消正在运行的扫描任务
 * 已完成的子任务结果将保留，未开始的任务将被取消
 */
export async function cancelScan(
  scan_id: string
): Promise<void> {
  return invoke<void>('cancel_scan', { scanId: scan_id })
}

/**
 * 查询扫描结果
 * 支持按参数范围过滤、排序和列选择，从结果数据库中检索数据
 */
export async function queryScanResults(
  query: QueryConfig
): Promise<QueryResult> {
  return invoke<QueryResult>('query_scan_results', { query })
}

/**
 * 导出扫描结果
 * 将筛选结果导出为指定格式文件（CSV / Parquet / JSON / HDF5）
 */
export async function exportScanResults(
  config: ExportConfig
): Promise<{ file_path: string }> {
  return invoke<{ file_path: string }>('export_scan_results', { config })
}

/**
 * 获取扫描统计分析
 * 返回各参数的相关系数、最优值和灵敏度分析结果
 */
export async function getScanStatistics(
  scan_id: string
): Promise<ScanStatistics[]> {
  return invoke<ScanStatistics[]>('get_scan_statistics', { scanId: scan_id })
}

/**
 * 获取扫描仪表盘
 * 返回平行坐标图、热力图和散点图的可视化 URL
 */
export async function getScanDashboard(
  scan_id: string
): Promise<ScanDashboard> {
  return invoke<ScanDashboard>('get_scan_dashboard', { scanId: scan_id })
}
