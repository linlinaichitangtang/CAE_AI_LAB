/**
 * Cloud HPC API - 云端 HPC 集群计算资源调度
 * V1.4-005: HPC 作业提交、状态查询、集群信息、模板管理
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export interface HpcJob {
  id: string
  name: string
  project_id: string
  status: 'queued' | 'running' | 'completed' | 'failed' | 'cancelled'
  submit_time: string
  start_time: string
  end_time: string
  priority: number // 1-10
  node_count: number
  cores_per_node: number
  memory_gb: number
  wall_time_hours: number
  estimated_time: string
  progress: number // 0-100
  input_file_path: string
  output_file_path: string
  log_url: string
  error_message: string
}

export interface HpcSubmitConfig {
  project_id: string
  job_name: string
  input_files: string[]
  solver_type: 'calculix' | 'openfoam' | 'custom'
  node_count: number
  cores_per_node: number
  memory_gb: number
  wall_time_hours: number
  priority: number
  callback_url: string
  notification_email: string
}

export interface HpcNode {
  id: string
  name: string
  status: 'online' | 'offline' | 'busy' | 'idle' | 'maintenance'
  cpu_cores: number
  cpu_usage: number // 0-100
  memory_total_gb: number
  memory_used_gb: number
  gpu_count: number
  gpu_type: string
  jobs_running: number
  queue_position: number
  temperature: number
  uptime_hours: number
}

export interface HpcClusterInfo {
  name: string
  total_nodes: number
  online_nodes: number
  total_cores: number
  available_cores: number
  total_memory_gb: number
  available_memory_gb: number
  queue_length: number
  avg_wait_time_minutes: number
}

export interface HpcSubmitResult {
  success: boolean
  job_id: string
  queue_position: number
  estimated_start_time: string
  estimated_completion_time: string
}

export interface HpcTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  solver_type: 'calculix' | 'openfoam' | 'custom'
  recommended_nodes: number
  recommended_cores: number
}

// ============ API 函数 ============

/** 提交 HPC 作业到云端集群 */
export async function submitHpcJob(config: HpcSubmitConfig): Promise<HpcSubmitResult> {
  return await invoke<HpcSubmitResult>('submit_hpc_job', { config })
}

/** 取消 HPC 作业 */
export async function cancelHpcJob(jobId: string): Promise<boolean> {
  return await invoke<boolean>('cancel_hpc_job', { jobId })
}

/** 获取 HPC 作业状态 */
export async function getHpcJobStatus(jobId: string): Promise<HpcJob> {
  return await invoke<HpcJob>('get_hpc_job_status', { jobId })
}

/** 获取 HPC 作业列表 */
export async function getHpcJobList(filters?: {
  project_id?: string
  status?: HpcJob['status']
}): Promise<HpcJob[]> {
  return await invoke<HpcJob[]>('get_hpc_job_list', { filters: filters ?? {} })
}

/** 获取 HPC 集群概览信息 */
export async function getHpcClusterInfo(): Promise<HpcClusterInfo> {
  return await invoke<HpcClusterInfo>('get_hpc_cluster_info')
}

/** 获取 HPC 节点列表 */
export async function getHpcNodeList(): Promise<HpcNode[]> {
  return await invoke<HpcNode[]>('get_hpc_node_list')
}

/** 下载 HPC 作业结果 */
export async function downloadHpcResult(jobId: string, localPath: string): Promise<string> {
  return await invoke<string>('download_hpc_result', { jobId, localPath })
}

/** 获取 HPC 作业模板列表 */
export async function getHpcTemplates(): Promise<HpcTemplate[]> {
  return await invoke<HpcTemplate[]>('get_hpc_templates')
}
