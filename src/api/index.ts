/**
 * CAELab Frontend API Module
 * 与后端Tauri命令对接的API封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型导出 ============

// 重新导出所有类型
export type {
  Project,
  ProjectFile,
  FileType,
  CreateProjectInput,
  UpdateProjectInput,
  CreateFileInput,
  UpdateFileInput,
  Settings,
  EmbedItem
} from './types'

// 也从types导入接口
import type {
  Project,
  ProjectFile,
  FileType,
  CreateProjectInput,
  UpdateProjectInput,
  CreateFileInput,
  UpdateFileInput
} from './types'

// ============ 项目管理 API ============

/**
 * 创建新项目
 */
export async function createProject(input: CreateProjectInput): Promise<Project> {
  return await invoke<Project>('create_project', { input })
}

/**
 * 获取所有项目列表
 */
export async function listProjects(): Promise<Project[]> {
  return await invoke<Project[]>('list_projects')
}

/**
 * 获取单个项目详情
 */
export async function getProject(id: string): Promise<Project> {
  return await invoke<Project>('get_project', { id })
}

/**
 * 更新项目信息
 */
export async function updateProject(input: UpdateProjectInput): Promise<Project> {
  return await invoke<Project>('update_project', { input })
}

/**
 * 删除项目
 */
export async function deleteProject(id: string): Promise<void> {
  await invoke('delete_project', { id })
}

// ============ 文件管理 API ============

/**
 * 创建新文件
 */
export async function createFile(input: CreateFileInput): Promise<ProjectFile> {
  return await invoke<ProjectFile>('create_file', { input })
}

/**
 * 获取项目的所有文件
 */
export async function listFiles(projectId: string): Promise<ProjectFile[]> {
  return await invoke<ProjectFile[]>('list_files', { projectId })
}

/**
 * 获取单个文件详情
 */
export async function getFile(id: string): Promise<ProjectFile> {
  return await invoke<ProjectFile>('get_file', { id })
}

/**
 * 更新文件
 */
export async function updateFile(input: UpdateFileInput): Promise<ProjectFile> {
  return await invoke<ProjectFile>('update_file', { input })
}

/**
 * 删除文件
 */
export async function deleteFile(id: string): Promise<void> {
  await invoke('delete_file', { id })
}

/**
 * 读取文件内容
 */
export async function readFileContent(id: string): Promise<string> {
  return await invoke<string>('read_file_content', { id })
}

/**
 * 写入文件内容
 */
export async function writeFileContent(id: string, content: string): Promise<void> {
  await invoke('write_file_content', { id, content })
}

// ============ 设置 API ============

/**
 * 保存项目设置
 */
export async function saveSettings(projectId: string, settingsJson: string): Promise<void> {
  await invoke('save_settings', { projectId, settingsJson })
}

/**
 * 获取项目设置
 */
export async function getSettings(projectId: string): Promise<any> {
  return await invoke('get_settings', { projectId })
}

/**
 * 删除项目设置
 */
export async function deleteSettings(projectId: string): Promise<void> {
  await invoke('delete_settings', { projectId })
}

// ============ 辅助函数 ============

/**
 * 格式化项目创建时间
 */
export function formatProjectDate(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

/**
 * 获取文件类型显示名称
 */
export function getFileTypeLabel(type: FileType): string {
  const labels: Record<FileType, string> = {
    'note': '笔记',
    'modeling_data': '建模数据',
    'code_file': '代码文件'
  }
  return labels[type] || type
}

/**
 * 获取文件类型图标
 */
export function getFileTypeIcon(type: FileType): string {
  const icons: Record<FileType, string> = {
    'note': '📝',
    'modeling_data': '🎨',
    'code_file': '💻'
  }
  return icons[type] || '📄'
}