/**
 * Workflow Scheduler API
 * V2.0-003: 工作流调度器（DAG 执行）
 */

import { invoke } from '@tauri-apps/api/core'
import type { WorkflowScaleLevel } from './workflowEditor'

// ============ 类型定义 ============

/** 执行状态 */
export type ExecutionStatus =
  | 'pending'
  | 'queued'
  | 'running'
  | 'paused'
  | 'completed'
  | 'failed'
  | 'cancelled'
  | 'rollback'

/** 节点执行记录 */
export interface NodeExecution {
  node_id: string
  scale: WorkflowScaleLevel
  status: ExecutionStatus
  started_at: string | null
  completed_at: string | null
  duration_s: number
  input_data: Record<string, unknown> | null
  output_data: Record<string, unknown> | null
  error: string | null
  retry_count: number
}

/** 执行检查点 */
export interface ExecutionCheckpoint {
  node_id: string
  step_index: number
  timestamp: string
  state: Record<string, unknown>
}

/** 工作流执行记录 */
export interface WorkflowExecution {
  execution_id: string
  graph_id: string
  status: ExecutionStatus
  nodes: NodeExecution[]
  checkpoints: ExecutionCheckpoint[]
  started_at: string
  completed_at: string | null
  total_time_s: number
  error: string | null
}

/** 执行配置 */
export interface ExecutionConfig {
  graph_id: string
  resume_from_node?: string
  max_retries: number
  timeout_per_node_s: number
  parallel_nodes: number
  enable_rollback: boolean
  notification_on_complete: boolean
}

// ============ API 函数 ============

/**
 * 启动工作流执行
 * @param config - 执行配置
 * @returns 执行 ID
 */
export async function startExecution(
  config: ExecutionConfig
): Promise<{ execution_id: string }> {
  return invoke<{ execution_id: string }>('start_execution', { config })
}

/**
 * 获取工作流执行状态
 * @param executionId - 执行 ID
 * @returns 完整的工作流执行记录
 */
export async function getExecutionStatus(
  executionId: string
): Promise<WorkflowExecution> {
  return invoke<WorkflowExecution>('get_execution_status', { executionId })
}

/**
 * 暂停正在执行的工作流
 * @param executionId - 执行 ID
 */
export async function pauseExecution(
  executionId: string
): Promise<void> {
  return invoke<void>('pause_execution', { executionId })
}

/**
 * 恢复已暂停的工作流执行
 * @param executionId - 执行 ID
 */
export async function resumeExecution(
  executionId: string
): Promise<void> {
  return invoke<void>('resume_execution', { executionId })
}

/**
 * 取消工作流执行
 * @param executionId - 执行 ID
 */
export async function cancelExecution(
  executionId: string
): Promise<void> {
  return invoke<void>('cancel_execution', { executionId })
}

/**
 * 从指定节点重试执行
 * @param executionId - 执行 ID
 * @param nodeId - 重试起始节点 ID
 */
export async function retryFromNode(
  executionId: string,
  nodeId: string
): Promise<void> {
  return invoke<void>('retry_from_node', { executionId, nodeId })
}

/**
 * 获取工作流执行的检查点列表
 * @param executionId - 执行 ID
 * @returns 检查点列表
 */
export async function getExecutionCheckpoints(
  executionId: string
): Promise<ExecutionCheckpoint[]> {
  return invoke<ExecutionCheckpoint[]>('get_execution_checkpoints', { executionId })
}

/**
 * 列出工作流执行记录
 * @param graphId - 可选，按工作流图 ID 筛选
 * @param status - 可选，按执行状态筛选
 * @param limit - 可选，返回记录数量上限
 * @returns 工作流执行记录列表
 */
export async function listExecutions(
  graphId?: string,
  status?: ExecutionStatus,
  limit?: number
): Promise<WorkflowExecution[]> {
  return invoke<WorkflowExecution[]>('list_executions', { graphId, status, limit })
}
