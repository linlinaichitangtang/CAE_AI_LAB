/**
 * Project Share API - Export/Import/Template utilities
 * Phase 2: Project sharing and collaboration
 */

import JSZip from 'jszip'
import { saveAs } from 'file-saver'
import { getProject, listFiles, readFileContent, createProject, createFile, type Project, type ProjectFile, type CreateProjectInput } from './index'

// ============ 项目导出 ============

export interface ExportData {
  project: Project
  files: Array<{
    file: ProjectFile
    content: string
  }>
  exportedAt: string
  version: string
}

/**
 * 导出项目为JSON数据
 */
export async function exportProject(projectId: string): Promise<ExportData> {
  const project = await getProject(projectId)
  const files = await listFiles(projectId)
  
  const fileData = await Promise.all(
    files.map(async (file) => {
      let content = ''
      if (file.content === null && file.file_type !== 'modeling_data') {
        // Try to read content from backend
        try {
          content = await readFileContent(file.id)
        } catch {
          content = ''
        }
      } else {
        content = file.content || ''
      }
      return { file, content }
    })
  )
  
  return {
    project,
    files: fileData,
    exportedAt: new Date().toISOString(),
    version: '1.0'
  }
}

/**
 * 导出项目为ZIP文件并下载
 */
export async function exportProjectAsZip(projectId: string): Promise<void> {
  const data = await exportProject(projectId)
  
  const zip = new JSZip()
  
  // Add project metadata
  zip.file('project.json', JSON.stringify({
    project: data.project,
    exportedAt: data.exportedAt,
    version: data.version
  }, null, 2))
  
  // Add files
  const filesFolder = zip.folder('files')
  if (filesFolder) {
    for (const fileData of data.files) {
      const filePath = fileData.file.file_path || `${fileData.file.file_type}/${fileData.file.file_name}`
      filesFolder.file(filePath, fileData.content)
      
      // Add file metadata
      filesFolder.file(`${filePath}.meta.json`, JSON.stringify(fileData.file, null, 2))
    }
  }
  
  // Generate and download ZIP
  const blob = await zip.generateAsync({ type: 'blob' })
  const fileName = `${data.project.name.replace(/[^a-zA-Z0-9\u4e00-\u9fa5]/g, '_')}_${new Date().toISOString().slice(0, 10)}.zip`
  saveAs(blob, fileName)
}

// ============ 项目导入 ============

export interface ImportOptions {
  name?: string
  description?: string
}

/**
 * 从ZIP文件导入项目
 */
export async function importProjectFromZip(file: File, options: ImportOptions = {}): Promise<Project> {
  const arrayBuffer = await file.arrayBuffer()
  const zip = await JSZip.loadAsync(arrayBuffer)
  
  // Read project metadata
  const projectJson = zip.file('project.json')
  if (!projectJson) {
    throw new Error('无效的导入文件：缺少 project.json')
  }
  
  const metadata = JSON.parse(await projectJson.async('string'))
  const originalProject = metadata.project
  
  // Create new project
  const newProjectInput: CreateProjectInput = {
    name: options.name || originalProject.name,
    description: options.description || originalProject.description
  }
  
  const newProject = await createProject(newProjectInput)
  
  // Import files
  const filesFolder = zip.folder('files')
  if (filesFolder) {
    const filePaths = Object.keys(filesFolder.files).filter(p => !p.endsWith('.meta.json') && p !== 'files/')
    
    for (const filePath of filePaths) {
      const file = filesFolder.file(filePath)
      if (!file) continue
      
      // Skip meta files
      if (filePath.endsWith('.meta.json')) continue
      
      const content = await file.async('string')
      
      // Get original file metadata
      const metaPath = `${filePath}.meta.json`
      const metaFile = filesFolder.file(metaPath)
      let originalFile = null
      if (metaFile) {
        originalFile = JSON.parse(await metaFile.async('string'))
      }
      
      await createFile({
        project_id: newProject.id,
        file_type: originalFile?.file_type || 'note',
        file_name: originalFile?.file_name || filePath.split('/').pop() || 'untitled',
        content: content,
        file_path: filePath
      })
    }
  }
  
  return newProject
}

// ============ 项目模板 ============

export interface ProjectTemplate {
  id: string
  name: string
  description: string
  category: string
  icon: string
  projectData?: ExportData
}

/**
 * 保存项目为模板（存储到localStorage）
 */
export async function saveProjectAsTemplate(projectId: string, name: string, description: string, category: string = 'default'): Promise<void> {
  const data = await exportProject(projectId)
  
  const templates = getTemplates()
  const template: ProjectTemplate = {
    id: `template_${Date.now()}`,
    name,
    description,
    category,
    icon: '📋',
    projectData: data
  }
  
  templates.push(template)
  localStorage.setItem('caelab_templates', JSON.stringify(templates))
}

/**
 * 获取所有模板
 */
export function getTemplates(): ProjectTemplate[] {
  const stored = localStorage.getItem('caelab_templates')
  return stored ? JSON.parse(stored) : []
}

/**
 * 从模板创建项目
 */
export async function createProjectFromTemplate(templateId: string, newName?: string, newDescription?: string): Promise<Project> {
  const templates = getTemplates()
  const template = templates.find(t => t.id === templateId)
  
  if (!template || !template.projectData) {
    throw new Error('模板不存在或数据已损坏')
  }
  
  // Create new project
  const newProjectInput: CreateProjectInput = {
    name: newName || template.projectData.project.name,
    description: newDescription || template.projectData.project.description
  }
  
  const newProject = await createProject(newProjectInput)
  
  // Import files
  for (const fileData of template.projectData.files) {
    await createFile({
      project_id: newProject.id,
      file_type: fileData.file.file_type,
      file_name: fileData.file.file_name,
      content: fileData.content,
      file_path: fileData.file.file_path
    })
  }
  
  return newProject
}

/**
 * 删除模板
 */
export function deleteTemplate(templateId: string): void {
  const templates = getTemplates()
  const filtered = templates.filter(t => t.id !== templateId)
  localStorage.setItem('caelab_templates', JSON.stringify(filtered))
}

// ============ 分享链接 ============

/**
 * 生成分享链接（base64编码项目ID）
 */
export function generateShareLink(projectId: string): string {
  const encoded = btoa(projectId)
  return `${window.location.origin}/share?data=${encoded}`
}

/**
 * 解析分享链接
 */
export function parseShareLink(url: string): string | null {
  try {
    const urlObj = new URL(url)
    const data = urlObj.searchParams.get('data')
    if (data) {
      return atob(data)
    }
  } catch {
    // Invalid URL
  }
  return null
}

/**
 * 复制文本到剪贴板
 */
export async function copyToClipboard(text: string): Promise<boolean> {
  try {
    await navigator.clipboard.writeText(text)
    return true
  } catch {
    // Fallback for older browsers
    try {
      const textarea = document.createElement('textarea')
      textarea.value = text
      textarea.style.position = 'fixed'
      textarea.style.opacity = '0'
      document.body.appendChild(textarea)
      textarea.select()
      document.execCommand('copy')
      document.body.removeChild(textarea)
      return true
    } catch {
      return false
    }
  }
}
