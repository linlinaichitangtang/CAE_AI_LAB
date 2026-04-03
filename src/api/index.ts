/**
 * CAELab Frontend API Module
 * 与后端Tauri命令对接的API封装 (Tauri v1)
 */

import { invoke } from '@tauri-apps/api/core'

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

export interface KnowledgeGraphNode {
  id: string
  title: string
  file_type: string
}

export interface KnowledgeGraphEdge {
  source: string
  target: string
}

export interface KnowledgeGraph {
  nodes: KnowledgeGraphNode[]
  edges: KnowledgeGraphEdge[]
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

// Knowledge graph API
export async function getKnowledgeGraph(projectId: string): Promise<KnowledgeGraph> {
  return await invoke<KnowledgeGraph>('get_knowledge_graph', { projectId })
}

// ============ 项目分享 API (Phase2) ============

export * from './share'
export type {
  ExportData,
  ImportOptions,
  ImportResult,
  ExportProgress,
  ImportProgress,
  ProjectTemplate
} from './share'

// ============ CFD API ============

export interface CfdSetup {
  project_id: number
  domain_regions: DomainRegion[]
  material: FluidMaterial
  boundary_conditions: BoundaryCondition[]
  turbulence_model: TurbulenceModel
  convergence_tolerance: number
  max_iterations: number
}

export interface DomainRegion {
  region_type: 'Fluid' | 'Solid'
  face_ids: number[]
}

export type FluidMaterial = 
  | { Air: null }
  | { Water: null }
  | { Oil: null }
  | { Custom: { density: number; viscosity: number } }

export interface BoundaryCondition {
  id: number
  boundary_type: 'VelocityInlet' | 'PressureOutlet' | 'Wall' | 'Symmetry' | 'Outlet'
  velocity?: number
  turbulence_intensity?: number
  hydraulic_diameter?: number
  gauge_pressure?: number
  wall_slip?: boolean
  face_ids: number[]
}

export type TurbulenceModel = 'Laminar' | 'KEpsilon' | 'KOmegaSST'

export interface CfdResultStats {
  iterations: number
  converged: boolean
  cl?: number
  cd?: number
  cm?: number
}

/** 生成OpenFOAM案例 */
export async function generateOpenfoamCase(setup: CfdSetup): Promise<string> {
  return await invoke<string>('generate_openfoam_case', { setup })
}

/** 解析OpenFOAM日志获取结果统计 */
export async function parseOpenfoamLog(logPath: string): Promise<CfdResultStats> {
  return await invoke<CfdResultStats>('parse_openfoam_log', { logPath })
}

/** 下载OpenFOAM案例（zip格式） */
export async function downloadOpenfoamCase(projectId: number): Promise<string> {
  return await invoke<string>('download_openfoam_case', { projectId })
}

/** 导入CFD几何文件（STL） */
export async function importCfdGeometry(projectId: number, filePath: string): Promise<string> {
  return await invoke<string>('import_cfd_geometry', { projectId, filePath })
}

/** 生成CFD报告 */
export async function generateCfdReport(projectId: number): Promise<string> {
  return await invoke<string>('generate_cfd_report', { projectId })
}

// ============ 动力学瞬态分析 API ============

export * from './transient_dynamics'

// ============ 疲劳分析 API ============

export * from './fatigue'

// ============ 显式动力学 API ============

export * from './explicit_dynamics'

// ============ 电子封装分析 API ============

export * from './electronics'

// ============ 嵌入记录 API ============

export interface EmbedRecord {
  id: string
  note_id: string
  target_type: string
  target_id: string
  target_name: string
  config: string | null
  created_at: string
}

export async function addEmbedRecord(noteId: string, targetType: string, targetId: string, targetName: string, config?: string) {
  return invoke<EmbedRecord>('add_embed_record', { noteId, targetType, targetId, targetName, config: config || null })
}

export async function getEmbedRecords(noteId: string) {
  return invoke<EmbedRecord[]>('get_embed_records', { noteId })
}

export async function deleteEmbedRecord(id: string) {
  return invoke('delete_embed_record', { id })
}

export async function updateEmbedRecord(id: string, targetName: string, config?: string) {
  return invoke('update_embed_record', { id, targetName, config: config || null })
}

// ============ 分类管理 API ============

export async function updateFileCategory(fileId: string, category: string) {
  return invoke('update_file_category', { fileId, category })
}

export async function getFileCategories(projectId: string) {
  return invoke<string[]>('get_file_categories', { projectId })
}

// ============ 生物力学分析 API ============

export * from './biomechanics'

// ============ DFT 输入生成器 API ============

export * from './dftInput'

// ============ 代码执行 API ============

export interface CodeOutput {
  stdout: string
  stderr: string
  exit_code: number
}

export async function executeCode(language: string, code: string, workingDir?: string): Promise<CodeOutput> {
  return invoke<CodeOutput>('execute_code', { language, code, workingDir: workingDir || null })
}

// ============ V1.8 多尺度数据管理与验证 API ============

// V1.8-001: 多尺度物理量本体库
export * from './ontology'

// V1.8-002: 跨尺度坐标映射引擎
export * from './coordinateMapping'

// V1.8-003: 粗粒化策略库
export * from './coarseGraining'