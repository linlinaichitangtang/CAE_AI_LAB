/**
 * DFT Task Manager API - DFT 任务管理器
 * V1.7-003: DFT 任务提交、状态查询、队列管理、SLURM/PBS 脚本生成
 * V1.7-008: DFT 云端提交、远程队列调度
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type DftTaskStatus = 'queued' | 'running' | 'completed' | 'failed' | 'cancelled' | 'submitting'

export type DftCode = 'vasp' | 'quantum_espresso' | 'abinit' | 'castep'

export interface DftTask {
  id: string
  name: string
  project_id: string
  code: DftCode
  status: DftTaskStatus
  submit_time: string
  start_time: string
  end_time: string
  priority: number // 1-10
  input_files: string[]
  output_files: string[]
  total_energy_eV: number | null
  convergence_achieved: boolean | null
  num_ionic_steps: number | null
  num_electronic_steps: number | null
  wall_time_seconds: number | null
  error_message: string
  log_url: string
  node_id: string
  queue_position: number
}

export interface DftSubmitConfig {
  project_id: string
  job_name: string
  code: DftCode
  input_directory: string
  num_cores: number
  wall_time_hours: number
  memory_gb: number
  priority: number
  callback_url: string
  notification_email: string
  hpc_queue: 'slurm' | 'pbs' | 'local'
}

export interface DftQueueConfig {
  scheduler: 'slurm' | 'pbs' | 'local'
  partition: string
  account: string
  qos: string
  max_jobs: number
  max_cores_per_job: number
}

export interface SlurmScriptConfig {
  job_name: string
  partition: string
  nodes: number
  ntasks_per_node: number
  cpus_per_task: number
  mem: string
  time: string
  output: string
  error: string
  modules: string[]
  commands: string[]
}

export interface DftSubmitResult {
  success: boolean
  job_id: string
  queue_position: number
  estimated_start_time: string
  submit_script_path: string
}

export interface DftTaskListResult {
  tasks: DftTask[]
  total: number
  running: number
  queued: number
  completed: number
  failed: number
}

// ============ API 函数 ============

/** 提交 DFT 计算任务 */
export async function submitDftJob(config: DftSubmitConfig): Promise<DftSubmitResult> {
  return await invoke<DftSubmitResult>('submit_dft_job', { config })
}

/** 取消 DFT 计算任务 */
export async function cancelDftJob(jobId: string): Promise<boolean> {
  return await invoke<boolean>('cancel_dft_job', { jobId })
}

/** 获取 DFT 任务状态 */
export async function getDftJobStatus(jobId: string): Promise<DftTask> {
  return await invoke<DftTask>('get_dft_job_status', { jobId })
}

/** 获取 DFT 任务列表 */
export async function getDftJobList(filters?: {
  project_id?: string
  status?: DftTaskStatus
  code?: DftCode
}): Promise<DftTaskListResult> {
  return await invoke<DftTaskListResult>('get_dft_job_list', { filters: filters ?? {} })
}

/** 生成 SLURM 提交脚本 */
export async function generateSlurmScript(config: SlurmScriptConfig): Promise<string> {
  return await invoke<string>('generate_slurm_script', { config })
}

/** 生成 PBS 提交脚本 */
export async function generatePbsScript(config: SlurmScriptConfig): Promise<string> {
  return await invoke<string>('generate_pbs_script', { config })
}

/** 配置 DFT 队列参数 */
export async function configureDftQueue(config: DftQueueConfig): Promise<{ success: boolean }> {
  return await invoke<{ success: boolean }>('configure_dft_queue', { config })
}

/** 获取队列状态概览 */
export async function getQueueStatus(): Promise<{
  running: number
  queued: number
  available_cores: number
}> {
  return await invoke<{ running: number; queued: number; available_cores: number }>('get_queue_status')
}
