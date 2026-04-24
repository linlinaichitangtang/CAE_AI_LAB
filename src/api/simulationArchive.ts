/**
 * V2.5 仿真结果归档 API - 前端封装
 * 对接 Rust 后端 simulation_archive 命令
 */

import { invoke } from '@tauri-apps/api/core'

// ============================================================================
// 类型定义
// ============================================================================

/** 仿真结果归档记录 */
export interface SimulationArchive {
  id: string
  projectId?: string
  simulationType: string
  materialName?: string
  composition?: string
  inputParams: string
  outputResults: string
  meshStats?: string
  solverName?: string
  solveTimeSec?: number
  converged?: boolean
  iterations?: number
  archivedAt: string
  notes?: string
}

/** 归档请求 */
export interface ArchiveRequest {
  projectId?: string
  simulationType: string
  materialName?: string
  composition?: Record<string, number>
  inputParams: Record<string, unknown>
  outputResults: Record<string, unknown>
  meshStats?: Record<string, unknown>
  solverName?: string
  solveTimeSec?: number
  converged?: boolean
  iterations?: number
  notes?: string
}

/** 查询过滤条件 */
export interface ArchiveFilter {
  projectId?: string
  simulationType?: string
  materialName?: string
  dateFrom?: string
  dateTo?: string
  limit?: number
  offset?: number
}

/** 数据导出请求 */
export interface ExportRequest {
  format: 'json' | 'csv'
  filter?: ArchiveFilter
}

/** 导出结果 */
export interface ExportResult {
  format: string
  recordCount: number
  filePath: string
  fileSizeBytes: number
}

/** 归档统计 */
export interface ArchiveStats {
  totalRecords: number
  bySimulationType: Record<string, number>
  byMaterial: Record<string, number>
  latestArchiveAt?: string
}

// ============================================================================
// API 函数
// ============================================================================

/** 归档仿真结果 */
export async function archiveSimulationResult(
  request: ArchiveRequest
): Promise<SimulationArchive> {
  return invoke<SimulationArchive>('archive_simulation_result', { request })
}

/** 查询归档记录 */
export async function querySimulationArchives(
  filter: ArchiveFilter
): Promise<SimulationArchive[]> {
  return invoke<SimulationArchive[]>('query_simulation_archives', { filter })
}

/** 获取归档统计 */
export async function getArchiveStatistics(): Promise<ArchiveStats> {
  return invoke<ArchiveStats>('get_archive_statistics')
}

/** 导出归档数据 */
export async function exportSimulationArchives(
  request: ExportRequest
): Promise<ExportResult> {
  return invoke<ExportResult>('export_simulation_archives', { request })
}

/** 删除归档记录 */
export async function deleteSimulationArchive(
  id: string
): Promise<boolean> {
  return invoke<boolean>('delete_simulation_archive', { id })
}
