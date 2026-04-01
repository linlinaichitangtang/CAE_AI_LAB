/**
 * Cloud Simulation API - 云端仿真任务提交与状态管理
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export interface CloudTask {
  id: string
  name: string
  status: 'pending' | 'queued' | 'running' | 'completed' | 'failed' | 'cancelled'
  progress: number // 0-100
  createdAt: string
  startedAt?: string
  completedAt?: string
  error?: string
  resultUrl?: string
  logs: string[]
}

export interface CloudTaskConfig {
  projectId: string
  projectName: string
  meshData: {
    nodes: Array<{ id: number; x: number; y: number; z: number }>
    elements: Array<{ id: number; type: string; nodeIds: number[] }>
  }
  material: {
    elastic_modulus: number
    poisson_ratio: number
    density: number
  }
  boundaryConditions: {
    fixedBcs: Array<{ name: string; nodes: number[]; bc_type: string }>
    pointLoads: Array<{ name: string; node: number; magnitude: number; direction: string }>
    uniformLoads: Array<{ name: string; surface_name: string; magnitude: number; load_type: string }>
  }
  solverSettings?: {
    maxIterations?: number
    convergenceTolerance?: number
    numThreads?: number
  }
}

export interface CloudTaskResult {
  success: boolean
  taskId: string
  message?: string
}

// ============ API 函数 ============

/** 提交云端仿真任务 */
export async function submitCloudTask(config: CloudTaskConfig): Promise<CloudTaskResult> {
  return await invoke<CloudTaskResult>('submit_cloud_simulation', { config })
}

/** 获取任务状态 */
export async function getCloudTaskStatus(taskId: string): Promise<CloudTask> {
  return await invoke<CloudTask>('get_cloud_task_status', { taskId })
}

/** 列出当前用户的所有云端任务 */
export async function listCloudTasks(projectId?: string): Promise<CloudTask[]> {
  return await invoke<CloudTask[]>('list_cloud_tasks', { projectId })
}

/** 取消云端任务 */
export async function cancelCloudTask(taskId: string): Promise<boolean> {
  return await invoke<boolean>('cancel_cloud_task', { taskId })
}

/** 删除云端任务 */
export async function deleteCloudTask(taskId: string): Promise<boolean> {
  return await invoke<boolean>('delete_cloud_task', { taskId })
}

/** 获取任务结果数据 */
export async function getCloudTaskResult(taskId: string): Promise<any> {
  return await invoke<any>('get_cloud_task_result', { taskId })
}

/** 获取任务日志流URL */
export async function getCloudTaskLogStream(taskId: string): Promise<string> {
  return await invoke<string>('get_cloud_task_log_stream', { taskId })
}

/** 测试云端连接 */
export async function testCloudConnection(): Promise<{ connected: boolean; message: string }> {
  return await invoke<{ connected: boolean; message: string }>('test_cloud_connection')
}

/** 获取云端服务配置 */
export async function getCloudConfig(): Promise<{
  apiUrl: string
  enabled: boolean
  maxConcurrentTasks: number
  queuePosition?: number
}> {
  return await invoke('get_cloud_config')
}