/**
 * Multiscale Workspace API
 * V1.9-010: 多尺度 UI 整合（单一工作区）
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type TaskScale = 'dft' | 'md' | 'phase_field' | 'fe'

export interface ProjectTask {
  task_id: string
  scale: TaskScale
  name: string
  status: 'created' | 'running' | 'completed' | 'failed' | 'paused'
  created_at: string
  updated_at: string
  parameters: Record<string, unknown>
  result_summary: Record<string, unknown> | null
  bridge_to: string | null
  bridge_from: string | null
}

export interface MultiscaleProject {
  project_id: string
  name: string
  description: string
  created_at: string
  updated_at: string
  tasks: ProjectTask[]
  shared_data: Array<{
    name: string
    type: string
    size_bytes: number
    source_task_id: string
  }>
  status: 'active' | 'archived' | 'completed'
}

export interface ProjectConfig {
  name: string
  description: string
  template_id?: string
}

export interface TaskConnection {
  from_task_id: string
  to_task_id: string
  data_mapping: Array<{
    source_field: string
    target_field: string
  }>
  bridge_method: string
}

export interface ProjectDashboard {
  project: MultiscaleProject
  task_graph: {
    nodes: Array<{
      id: string
      scale: TaskScale
      status: string
    }>
    edges: Array<{
      from: string
      to: string
      data_flow: string
    }>
  }
  progress: {
    total: number
    completed: number
    running: number
    failed: number
  }
  shared_data_summary: {
    total_size_mb: number
    data_types: string[]
  }
}

// ============ API 函数 ============

/**
 * 创建多尺度项目
 */
export async function createProject(
  config: ProjectConfig
): Promise<{ project_id: string }> {
  return invoke<{ project_id: string }>('create_project', { config })
}

/**
 * 获取项目详情
 */
export async function getProject(
  project_id: string
): Promise<MultiscaleProject> {
  return invoke<MultiscaleProject>('get_project', { projectId: project_id })
}

/**
 * 列出所有项目
 */
export async function listProjects(): Promise<MultiscaleProject[]> {
  return invoke<MultiscaleProject[]>('list_projects')
}

/**
 * 向项目添加任务
 */
export async function addTask(
  project_id: string,
  scale: TaskScale,
  name: string,
  parameters: Record<string, unknown>
): Promise<ProjectTask> {
  return invoke<ProjectTask>('add_task', {
    projectId: project_id,
    scale,
    name,
    parameters,
  })
}

/**
 * 连接两个任务（建立数据桥接）
 */
export async function connectTasks(
  connection: TaskConnection
): Promise<void> {
  return invoke<void>('connect_tasks', { connection })
}

/**
 * 获取项目仪表盘
 */
export async function getProjectDashboard(
  project_id: string
): Promise<ProjectDashboard> {
  return invoke<ProjectDashboard>('get_project_dashboard', { projectId: project_id })
}

/**
 * 运行项目（执行所有任务）
 */
export async function runProject(
  project_id: string
): Promise<{ execution_id: string }> {
  return invoke<{ execution_id: string }>('run_project', { projectId: project_id })
}

/**
 * 归档项目
 */
export async function archiveProject(
  project_id: string
): Promise<void> {
  return invoke<void>('archive_project', { projectId: project_id })
}
