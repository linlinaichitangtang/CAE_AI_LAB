/**
 * CAELab Frontend API Types
 * 与后端Tauri命令对接的类型定义
 */

// ============ 项目管理类型 ============

export interface Project {
  id: string
  name: string
  description: string
  created_at: string
  updated_at: string
}

export interface ProjectFile {
  id: string
  project_id: string
  file_type: FileType
  file_name: string
  content: string | null
  file_path: string
  category: string
  created_at: string
  updated_at: string
}

export type FileType = 'note' | 'modeling_data' | 'code_file'

export interface CreateProjectInput {
  name: string
  description: string
}

export interface UpdateProjectInput {
  id: string
  name?: string
  description?: string
}

export interface CreateFileInput {
  project_id: string
  file_type: FileType
  file_name: string
  content?: string
  file_path: string
}

export interface UpdateFileInput {
  id: string
  file_name?: string
  content?: string
}

export interface Settings {
  project_id: string
  settings_json: string
}

// ============ 嵌入对象类型 ============

export interface EmbedItem {
  id: string
  type: 'model' | 'code' | 'simulation'
  name: string
  data?: any
}