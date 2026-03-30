/**
 * CAELab Frontend API Module
 * 与后端Tauri命令对接的API封装 (Tauri v1)
 */

import { invoke } from '@tauri-apps/api/tauri'

// ============ 类型导出 ============

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

// ============ CAE API 导出 ============

export * from './cae'
export type {
  Node,
  Element,
  FixedBc,
  PointLoad,
  UniformLoad,
  MeshApiResult,
  ResultSet
} from './cae'

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

export async function createProject(input: CreateProjectInput): Promise<Project> {
  return await invoke<Project>('create_project', { input })
}

export async function listProjects(): Promise<Project[]> {
  return await invoke<Project[]>('list_projects')
}

export async function getProject(id: string): Promise<Project> {
  return await invoke<Project>('get_project', { id })
}

export async function updateProject(input: UpdateProjectInput): Promise<Project> {
  return await invoke<Project>('update_project', { input })
}

export async function deleteProject(id: string): Promise<void> {
  await invoke('delete_project', { id })
}

// ============ 文件管理 API ============

export async function createFile(input: CreateFileInput): Promise<ProjectFile> {
  return await invoke<ProjectFile>('create_file', { input })
}

export async function listFiles(projectId: string): Promise<ProjectFile[]> {
  return await invoke<ProjectFile[]>('list_files', { projectId })
}

export async function getFile(id: string): Promise<ProjectFile> {
  return await invoke<ProjectFile>('get_file', { id })
}

export async function updateFile(input: UpdateFileInput): Promise<ProjectFile> {
  return await invoke<ProjectFile>('update_file', { input })
}

export async function deleteFile(id: string): Promise<void> {
  await invoke('delete_file', { id })
}

export async function readFileContent(id: string): Promise<string> {
  return await invoke<string>('read_file_content', { id })
}

export async function writeFileContent(id: string, content: string): Promise<void> {
  await invoke('write_file_content', { id, content })
}

// ============ 设置 API ============

export async function saveSettings(projectId: string, settingsJson: string): Promise<void> {
  await invoke('save_settings', { projectId, settingsJson })
}

export async function getSettings(projectId: string): Promise<any> {
  return await invoke('get_settings', { projectId })
}

export async function deleteSettings(projectId: string): Promise<void> {
  await invoke('delete_settings', { projectId })
}

// ============ 辅助函数 ============

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

export function getFileTypeLabel(type: FileType): string {
  const labels: Record<FileType, string> = {
    'note': '笔记',
    'modeling_data': '建模数据',
    'code_file': '代码文件'
  }
  return labels[type] || type
}

export function getFileTypeIcon(type: FileType): string {
  const icons: Record<FileType, string> = {
    'note': '📝',
    'modeling_data': '🎨',
    'code_file': '💻'
  }
  return icons[type] || '📄'
}

// ============ 笔记增强 API (Phase2) ============

export interface NoteVersion {
  id: string
  note_id: string
  title: string
  content: string
  created_at: string
}

export interface NoteLink {
  id: string
  source_note_id: string
  target_note_id: string
  created_at: string
}

export interface SearchResult {
  note_id: string
  title: string
  snippet: string
  score: number
}

// Version history API
export async function saveNoteVersion(noteId: string, title: string, content: string): Promise<NoteVersion> {
  return await invoke<NoteVersion>('save_note_version', { noteId, title, content })
}

export async function getNoteVersions(noteId: string): Promise<NoteVersion[]> {
  return await invoke<NoteVersion[]>('get_note_versions', { noteId })
}

export async function getNoteVersion(versionId: string): Promise<NoteVersion> {
  return await invoke<NoteVersion>('get_note_version', { versionId })
}

export async function restoreNoteVersion(noteId: string, versionId: string): Promise<ProjectFile> {
  return await invoke<ProjectFile>('restore_note_version', { noteId, versionId })
}

export async function deleteNoteVersion(versionId: string): Promise<void> {
  await invoke('delete_note_version', { versionId })
}

// Note link API
export async function createNoteLink(sourceNoteId: string, targetNoteId: string): Promise<NoteLink> {
  return await invoke<NoteLink>('create_note_link', { sourceNoteId, targetNoteId })
}

export async function getNoteLinks(noteId: string): Promise<NoteLink[]> {
  return await invoke<NoteLink[]>('get_note_links', { noteId })
}

export async function getNoteBacklinks(noteId: string): Promise<NoteLink[]> {
  return await invoke<NoteLink[]>('get_note_backlinks', { noteId })
}

export async function deleteNoteLink(linkId: string): Promise<void> {
  await invoke('delete_note_link', { linkId })
}

// Search API
export async function searchNotes(projectId: string, query: string): Promise<SearchResult[]> {
  return await invoke<SearchResult[]>('search_notes', { projectId, query })
}

// ============ 项目分享 API (Phase2) ============

export * from './share'
export type {
  ExportData,
  ImportOptions,
  ProjectTemplate
} from './share'