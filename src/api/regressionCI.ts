/**
 * Multiscale Regression Testing CI API
 * V1.8-006: 多尺度回归测试 CI
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type CITestStatus =
  | 'passed'
  | 'failed'
  | 'skipped'
  | 'running'
  | 'timeout'

export interface CITestResult {
  test_id: string
  benchmark_id: string
  status: CITestStatus
  computed: number
  reference: number
  error_percent: number
  tolerance_percent: number
  execution_time_s: number
  timestamp: string
}

export interface CIPipeline {
  pipeline_id: string
  status: 'queued' | 'running' | 'completed' | 'failed'
  triggered_by: string
  started_at: string
  completed_at: string
  results: CITestResult[]
  summary: {
    total: number
    passed: number
    failed: number
    skipped: number
    total_time_s: number
  }
}

export interface CINotificationConfig {
  type: 'slack' | 'email' | 'webhook'
  enabled: boolean
  url: string
  on_failure_only: boolean
}

export interface CITriggerConfig {
  benchmark_ids: string[]
  tolerance_override: number
  max_parallel: number
  notification: CINotificationConfig
}

// ============ API 函数 ============

/**
 * 触发 CI 回归测试管线
 */
export async function triggerCIPipeline(
  config: CITriggerConfig
): Promise<{ pipeline_id: string }> {
  return invoke<{ pipeline_id: string }>('trigger_ci_pipeline', { config })
}

/**
 * 获取管线执行状态
 */
export async function getPipelineStatus(
  pipeline_id: string
): Promise<CIPipeline> {
  return invoke<CIPipeline>('get_pipeline_status', { pipelineId: pipeline_id })
}

/**
 * 获取管线执行历史
 */
export async function getPipelineHistory(
  limit?: number
): Promise<CIPipeline[]> {
  return invoke<CIPipeline[]>('get_pipeline_history', { limit })
}

/**
 * 获取单个测试详情
 */
export async function getCITestDetail(
  test_id: string
): Promise<CITestResult> {
  return invoke<CITestResult>('get_ci_test_detail', { testId: test_id })
}

/**
 * 更新通知配置
 */
export async function updateNotificationConfig(
  config: CINotificationConfig
): Promise<void> {
  return invoke<void>('update_notification_config', { config })
}

/**
 * 获取 CI 统计信息
 */
export async function getCIStatistics(): Promise<{
  total_pipelines: number
  pass_rate: number
  avg_time_s: number
  last_run: string
}> {
  return invoke<{
    total_pipelines: number
    pass_rate: number
    avg_time_s: number
    last_run: string
  }>('get_ci_statistics')
}
