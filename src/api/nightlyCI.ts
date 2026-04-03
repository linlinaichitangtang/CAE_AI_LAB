/**
 * Nightly CI / Nightly Build API
 * V1.9-008: 四尺度集成 CI / Nightly Build
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type NightlyStatus =
  | 'scheduled'
  | 'running'
  | 'completed'
  | 'failed'
  | 'partial'

export interface NightlyConfig {
  scenarios: string[]
  trigger_time: string
  max_duration_s: number
  on_failure: 'notify_only' | 'create_issue' | 'block_release'
  notification_channels: Array<{
    type: 'slack' | 'email' | 'webhook'
    url: string
  }>
}

export interface NightlyScenarioResult {
  scenario: string
  status: 'passed' | 'failed' | 'skipped'
  duration_s: number
  steps: Array<{
    scale: string
    status: string
    error: string | null
  }>
  metrics: Array<{
    quantity: string
    value: number
    reference: number
    error_percent: number
  }>
}

export interface NightlyBuild {
  build_id: string
  status: NightlyStatus
  triggered_at: string
  completed_at: string
  duration_s: number
  scenarios: NightlyScenarioResult[]
  summary: {
    total: number
    passed: number
    failed: number
    skipped: number
  }
  report_url: string
  issues_created: number
}

export interface NightlyHistoryQuery {
  date_range?: {
    start: string
    end: string
  }
  status?: NightlyStatus
  limit?: number
}

// ============ API 函数 ============

/**
 * 触发 Nightly Build
 */
export async function triggerNightlyBuild(): Promise<{ build_id: string }> {
  return invoke<{ build_id: string }>('trigger_nightly_build')
}

/**
 * 获取 Nightly Build 状态
 */
export async function getNightlyStatus(
  build_id: string
): Promise<NightlyBuild> {
  return invoke<NightlyBuild>('get_nightly_status', { buildId: build_id })
}

/**
 * 获取 Nightly Build 历史记录
 */
export async function getNightlyHistory(
  query: NightlyHistoryQuery
): Promise<NightlyBuild[]> {
  return invoke<NightlyBuild[]>('get_nightly_history', { query })
}

/**
 * 更新 Nightly Build 配置
 */
export async function updateNightlyConfig(
  config: NightlyConfig
): Promise<void> {
  return invoke<void>('update_nightly_config', { config })
}

/**
 * 获取 Nightly Build 配置
 */
export async function getNightlyConfig(): Promise<NightlyConfig> {
  return invoke<NightlyConfig>('get_nightly_config')
}

/**
 * 获取 Nightly Build 统计信息
 */
export async function getNightlyStatistics(): Promise<{
  total_builds: number
  pass_rate: number
  avg_duration_s: number
  last_success: string
  last_failure: string
  consecutive_failures: number
}> {
  return invoke<{
    total_builds: number
    pass_rate: number
    avg_duration_s: number
    last_success: string
    last_failure: string
    consecutive_failures: number
  }>('get_nightly_statistics')
}

/**
 * 获取 Nightly Build 报告
 */
export async function getNightlyReport(
  build_id: string
): Promise<{ report_url: string }> {
  return invoke<{ report_url: string }>('get_nightly_report', { buildId: build_id })
}
