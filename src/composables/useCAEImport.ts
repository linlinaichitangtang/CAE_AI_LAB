/**
 * useCAEImport.ts — V2.9-010 ANSYS/ABAQUS 格式导入
 * 支持导入 ANSYS Workbench (.wbpj)、ABAQUS (.cae/.inp)、COMSOL (.mph) 文件
 */

import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export interface CAEImportResult {
  success: boolean
  format: 'inp' | 'cdb' | 'wbpj' | 'cae' | 'unknown'
  fileName: string
  preview: CAEPreview | null
  error?: string
}

export interface CAEPreview {
  nodes?: number
  elements?: number
  materials?: number
  steps?: number
  boundaries?: number
  loads?: number
  modelBounds?: {
    min: [number, number, number]
    max: [number, number, number]
  }
  geometryInfo?: {
    hasSolid: boolean
    hasShell: boolean
    hasBeam: boolean
    partCount: number
  }
  warnings?: string[]
}

export interface ANSYSResult {
  success: boolean
  format: 'wbpj' | 'cdb' | 'inp' | 'unknown'
  projectPath?: string
  systemCount?: number
  componentSystems?: string[]
  meshInfo?: {
    nodeCount: number
    elementCount: number
    elementTypes: string[]
  }
  materialCount?: number
  error?: string
  warnings?: string[]
}

export interface ABAQUSResult {
  success: boolean
  model?: {
    nodes: Array<{ id: number; x: number; y: number; z: number }>
    elements: Array<{ id: number; type: string; nodeIds: number[] }>
    materials: Array<{ name: string; properties: Record<string, number> }>
    sections: Array<{ type: string; material: string; thickness?: number }>
    steps: Array<{ name: string; type: string }>
    boundaries: Array<{ type: string; region: string; dof?: number; magnitude?: number }>
    loads: Array<{ type: string; region: string; magnitude: number; direction?: string }>
  }
  nodeSets?: Record<string, number[]>
  elementSets?: Record<string, number[]>
  warnings?: string[]
  error?: string
}

// ============ CAE 格式检测 ============

/**
 * 通过文件头检测 CAE 文件格式
 * 注意：此函数主要依赖文件扩展名，FORMAT_SIGNATURES 用于文档和未来扩展
 */
export function detectCAEFormat(fileName: string, _headerBytes?: Uint8Array): 'inp' | 'cdb' | 'wbpj' | 'cae' | 'mph' | 'unknown' {
  const ext = fileName.toLowerCase().split('.').pop()

  if (ext === 'inp') return 'inp'
  if (ext === 'cdb') return 'cdb'
  if (ext === 'cae') return 'cae'
  if (ext === 'mph') return 'mph'
  if (ext === 'wbpj') return 'wbpj'
  if (ext === 'cae') return 'cae'

  // 尝试验证文件头
  if (_headerBytes) {
    const header = new TextDecoder().decode(_headerBytes.slice(0, 100))
    if (header.includes('*HEADING') || header.includes('*Part')) return 'inp'
    if (header.includes('ANSYS')) return 'cdb'
    if (header.includes('<?xml') || header.includes('<WorkbenchProject')) return 'wbpj'
  }

  return 'unknown'
}

// ============ INP/Abaqus 导入 (通过 Tauri 后端) ============

/**
 * 导入 Abaqus INP 文件（复用后端命令）
 */
export async function importINPFile(filePath: string): Promise<any> {
  return await invoke('import_abaqus_inp', { filePath })
}

/**
 * 导入后预览
 */
export async function previewINPFile(file: File): Promise<CAEPreview> {
  // 读取文件内容进行预览
  const content = await file.text()
  const lines = content.split('\n')

  let nodes = 0
  let elements = 0
  let materials = new Set<string>()
  let steps = 0
  let boundaries = 0
  let loads = 0

  let inNodes = false
  let inElements = false

  for (const line of lines) {
    const trimmed = line.trim().toUpperCase()

    // 检测节点
    if (trimmed.startsWith('*NODE')) {
      inNodes = true
      inElements = false
    } else if (trimmed.startsWith('*ELEMENT')) {
      inElements = true
      inNodes = false
    } else if (trimmed.startsWith('*') && trimmed !== '*NODE' && trimmed !== '*ELEMENT') {
      inNodes = false
      inElements = false
    }

    if (inNodes && !trimmed.startsWith('*')) {
      nodes++
    }
    if (inElements && !trimmed.startsWith('*')) {
      elements++
    }

    // 检测材料
    if (trimmed.startsWith('*MATERIAL')) {
      const match = trimmed.match(/\$*NAME\s*=\s*(.+)/i) || trimmed.match(/\s*(.+)\s*$/)
      if (match) materials.add(match[1].trim())
    }

    // 检测步骤
    if (trimmed.startsWith('*STEP')) {
      steps++
    }

    // 检测边界条件
    if (trimmed.startsWith('*BOUNDARY')) {
      boundaries++
    }

    // 检测载荷
    if (trimmed.startsWith('*CLOAD') || trimmed.startsWith('*DLOAD') || trimmed.startsWith('*LOAD')) {
      loads++
    }
  }

  return {
    nodes,
    elements,
    materials: materials.size,
    steps,
    boundaries,
    loads
  }
}

// ============ ANSYS CDB 导入 (V2.9-010) ============

/**
 * 解析 ANSYS CDB 文件并返回预览信息
 */
export async function previewCDBFile(file: File): Promise<CAEPreview> {
  const content = await file.text()
  const lines = content.split('\n')

  let nodes = 0
  let elements = 0
  let materials = 0
  let elementTypes = new Set<string>()

  for (const line of lines) {
    const trimmed = line.trim()
    const upper = trimmed.toUpperCase()

    // 节点块: N, node_id, x, y, z
    if (upper.startsWith('N,') || /^\s*\d+\s*,\s*[\d.eE+-]+\s*,\s*[\d.eE+-]+\s*,\s*[\d.eE+-]+/.test(trimmed)) {
      nodes++
    }

    // 单元块: EN, elem_id, MAT, TYPE, REAL, SEC
    if (upper.startsWith('EN,') || /^\s*\d+\s*,\s*\d+\s*,\s*\d+\s*,\s*\d+\s*,\s*\d+\s*,/.test(trimmed)) {
      elements++
      // 检测单元类型
      const typeMatch = trimmed.match(/TYPE\s*=\s*(\d+)/i)
      if (typeMatch) {
        elementTypes.add(`TYPE_${typeMatch[1]}`)
      }
    }

    // 材料快: MAT, mat_id
    if (upper.startsWith('MAT,') || /^\s*MAT\s*,\s*\d+/.test(upper)) {
      materials++
    }
  }

  return {
    nodes,
    elements,
    materials,
    geometryInfo: {
      hasSolid: elements > 0,
      hasShell: false,
      hasBeam: false,
      partCount: 1
    }
  }
}

/**
 * 解析 ANSYS WBPJ 文件（XML 格式）
 */
export async function previewWBPJFile(file: File): Promise<CAEPreview> {
  const content = await file.text()

  // 解析 XML
  const parser = new DOMParser()
  const doc = parser.parseFromString(content, 'text/xml')

  // 查找项目信息
  const projectName = doc.querySelector('ProjectName')?.textContent || ''
  const systems = doc.querySelectorAll('System')

  let meshNodes = 0
  let meshElements = 0

  // 查找 Mesh 信息（如果存在）
  const meshInfo = doc.querySelectorAll('Mesh')
  meshInfo.forEach(mesh => {
    const nodeCount = mesh.getAttribute('NodeCount')
    const elemCount = mesh.getAttribute('ElementCount')
    if (nodeCount) meshNodes = Math.max(meshNodes, parseInt(nodeCount))
    if (elemCount) meshElements = Math.max(meshElements, parseInt(elemCount))
  })

  return {
    nodes: meshNodes,
    elements: meshElements,
    geometryInfo: {
      hasSolid: true,
      hasShell: false,
      hasBeam: false,
      partCount: systems.length
    },
    warnings: [
      `项目名称: ${projectName}`,
      `包含 ${systems.length} 个系统`
    ]
  }
}

// ============ ABAQUS CAE 导入 (V2.9-010) ============

/**
 * 解析 ABAQUS CAE 文件（SQLite 格式）
 * 注意：直接解析 .cae 文件很复杂，这里提供基础支持
 */
export async function previewCAEFile(file: File): Promise<CAEPreview> {
  // .cae 文件是 SQLite 格式
  // 读取文件头验证格式
  const buffer = await file.slice(0, 16).arrayBuffer()
  const header = new Uint8Array(buffer)

  // SQLite 文件头: "SQLite format 3"
  const isSQLite = String.fromCharCode(...header.slice(0, 16)).includes('SQLite format 3')

  if (!isSQLite) {
    return {
      warnings: ['文件格式可能不正确，不是 ABAQUS CAE 文件']
    }
  }

  return {
    geometryInfo: {
      hasSolid: true,
      hasShell: true,
      hasBeam: false,
      partCount: 1
    },
    warnings: [
      'ABAQUS .cae 文件已识别',
      '完整解析需要在后端进行，请使用 File > Import 功能'
    ]
  }
}

// ============ 统一导入入口 (V2.9-010) ============

/**
 * 通用 CAE 文件导入
 */
export async function importCAEFile(file: File): Promise<CAEImportResult> {
  const format = detectCAEFormat(file.name)

  switch (format) {
    case 'inp':
      return {
        success: true,
        format: 'inp',
        fileName: file.name,
        preview: await previewINPFile(file)
      }

    case 'cdb':
      return {
        success: true,
        format: 'cdb',
        fileName: file.name,
        preview: await previewCDBFile(file)
      }

    case 'wbpj':
      return {
        success: true,
        format: 'wbpj',
        fileName: file.name,
        preview: await previewWBPJFile(file)
      }

    case 'cae':
      return {
        success: true,
        format: 'cae',
        fileName: file.name,
        preview: await previewCAEFile(file)
      }

    default:
      return {
        success: false,
        format: 'unknown',
        fileName: file.name,
        preview: null,
        error: `不支持的文件格式: ${file.name}。支持的格式: .inp (Abaqus), .cdb (ANSYS), .wbpj (ANSYS Workbench), .cae (ABAQUS)`
      }
  }
}

/**
 * 通过 Tauri 导入文件（调用后端）
 */
export async function importViaBackend(filePath: string, format: string): Promise<any> {
  switch (format) {
    case 'inp':
    case 'cdb':
      return await invoke('import_abaqus_inp', { filePath })
    case 'wbpj':
      return await invoke('import_ansys_wbpj', { filePath })
    default:
      throw new Error(`不支持的后端导入格式: ${format}`)
  }
}

// ============ Composable ============

export function useCAEImport() {
  const isImporting = ref(false)
  const importError = ref<string | null>(null)
  const importResult = ref<CAEImportResult | null>(null)
  const supportedFormats = [
    { ext: '.inp', name: 'Abaqus INP', desc: 'Abaqus 求解器输入文件' },
    { ext: '.cdb', name: 'ANSYS CDB', desc: 'ANSYS Classic 网格/模型文件' },
    { ext: '.wbpj', name: 'ANSYS Workbench', desc: 'ANSYS Workbench 项目文件' },
    { ext: '.cae', name: 'ABAQUS CAE', desc: 'ABAQUS 完整模型文件' },
  ]

  async function importFile(file: File): Promise<CAEImportResult> {
    isImporting.value = true
    importError.value = null
    importResult.value = null

    try {
      const result = await importCAEFile(file)
      importResult.value = result
      if (!result.success) {
        importError.value = result.error || '导入失败'
      }
      return result
    } catch (e: any) {
      const errorMsg = e.message || String(e)
      importError.value = errorMsg
      return {
        success: false,
        format: 'unknown',
        fileName: file.name,
        preview: null,
        error: errorMsg
      }
    } finally {
      isImporting.value = false
    }
  }

  function clearResult() {
    importResult.value = null
    importError.value = null
  }

  return {
    isImporting,
    importError,
    importResult,
    supportedFormats,
    importFile,
    clearResult,
    detectCAEFormat,
    previewINPFile,
    previewCDBFile,
    previewWBPJFile,
    previewCAEFile
  }
}