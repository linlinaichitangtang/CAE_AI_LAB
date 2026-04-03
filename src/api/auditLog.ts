/**
 * Simulation Audit Log API
 * V1.8-007: 仿真结果审计日志 (hash chain, ISO 9001 reports)
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type AuditEventType =
  | 'simulation_start'
  | 'simulation_complete'
  | 'parameter_change'
  | 'result_export'
  | 'data_import'
  | 'scale_bridge'
  | 'coarse_graining'
  | 'validation_run'

export interface AuditEntry {
  entry_id: string
  timestamp: string
  event_type: AuditEventType
  user_id: string
  simulation_id: string
  description: string
  metadata: Record<string, unknown>
  previous_hash: string
  entry_hash: string
  block_index: number
}

export interface AuditChain {
  chain_id: string
  simulation_id: string
  created_at: string
  entries: AuditEntry[]
  is_valid: boolean
  total_entries: number
}

export interface AuditReport {
  chain_id: string
  simulation_id: string
  report_type: 'ISO9001' | 'AS9100' | 'custom'
  generated_at: string
  entries: AuditEntry[]
  summary: {
    total_events: number
    duration_s: number
    scales_involved: string[]
  }
  hash_valid: boolean
}

export interface AuditExportConfig {
  chain_id: string
  report_type: 'ISO9001' | 'AS9100' | 'custom'
  format: 'pdf' | 'html' | 'json'
  include_raw_data: boolean
}

// ============ API 函数 ============

/**
 * 创建审计链
 */
export async function createAuditChain(
  simulationId: string
): Promise<{ chain_id: string }> {
  return invoke<{ chain_id: string }>('create_audit_chain', { simulationId })
}

/**
 * 添加审计条目
 */
export async function addAuditEntry(
  chain_id: string,
  event_type: AuditEventType,
  description: string,
  metadata?: Record<string, unknown>
): Promise<AuditEntry> {
  return invoke<AuditEntry>('add_audit_entry', { chain_id, event_type, description, metadata })
}

/**
 * 获取审计链
 */
export async function getAuditChain(
  chain_id: string
): Promise<AuditChain> {
  return invoke<AuditChain>('get_audit_chain', { chain_id })
}

/**
 * 验证审计链完整性
 */
export async function validateChain(
  chain_id: string
): Promise<{ is_valid: boolean; tampered_entries: number[]; first_tampered_index: number }> {
  return invoke<{ is_valid: boolean; tampered_entries: number[]; first_tampered_index: number }>(
    'validate_chain',
    { chain_id }
  )
}

/**
 * 导出审计报告
 */
export async function exportAuditReport(
  config: AuditExportConfig
): Promise<{ report_path: string }> {
  return invoke<{ report_path: string }>('export_audit_report', { config })
}

/**
 * 搜索审计日志
 */
export async function searchAuditLogs(
  simulation_id: string,
  event_type?: AuditEventType,
  date_range?: { start: string; end: string }
): Promise<AuditEntry[]> {
  return invoke<AuditEntry[]>('search_audit_logs', { simulation_id, event_type, date_range })
}
