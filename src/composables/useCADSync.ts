/**
 * useCADSync.ts — V4.1-005 CAD 双向同步基础
 * 支持 STEP/IGES/Parasolid 导入、参数同步、变更追踪
 */
import { ref, computed } from 'vue'

export type CADFormat = 'step' | 'iges' | 'x_t' | 'stl' | 'obj'
export type SyncStatus = 'synced' | 'dirty' | 'conflict' | 'error' | 'offline'

export interface CADParameter {
  name: string
  value: number
  unit: string
  expression?: string
  synced: boolean
}

export interface CADGeometry {
  id: string
  name: string
  type: 'solid' | 'surface' | 'curve' | 'point'
  visible: boolean
  importedAt: string
  lastModified: string
}

export interface CADLink {
  id: string
  cadFilePath: string
  format: CADFormat
  version: string
  importedAt: string
  lastSyncAt: string | null
  status: SyncStatus
  parameters: CADParameter[]
  geometries: CADGeometry[]
  changeLog: CADChangeEntry[]
  associatedSimulationId?: string
}

export interface CADChangeEntry {
  id: string
  timestamp: string
  type: 'parameter' | 'geometry' | 'material' | 'feature'
  description: string
  source: 'cad' | 'cae'
  merged: boolean
  conflict?: boolean
}

export interface SyncPreview {
  additions: CADGeometry[]
  modifications: Array<{ old: CADGeometry; new: CADGeometry }>
  deletions: CADGeometry[]
  parameterChanges: Array<{ name: string; old: number; new: number }>
}

const FORMAT_LABELS: Record<CADFormat, string> = {
  step: 'STEP (.stp/.step)',
  iges: 'IGES (.igs/.iges)',
  x_t: 'Parasolid (.x_t)',
  stl: 'STL (.stl)',
  obj: 'Wavefront (.obj)'
}

export function useCADSync() {
  const cadLinks = ref<CADLink[]>([])
  const isSyncing = ref(false)
  const lastError = ref<string | null>(null)
  const syncPreview = ref<SyncPreview | null>(null)

  const activeLinks = computed(() => cadLinks.value.filter(l => l.status !== 'offline'))
  const dirtyLinks = computed(() => cadLinks.value.filter(l => l.status === 'dirty'))
  const conflictLinks = computed(() => cadLinks.value.filter(l => l.status === 'conflict'))

  /**
   * 导入 CAD 文件
   */
  async function importCAD(file: File): Promise<CADLink> {
    const format = detectFormat(file.name)
    if (!format) {
      throw new Error('不支持的 CAD 文件格式')
    }

    // 模拟解析过程（实际项目调用后端解析）
    await simulateDelay(800)

    const link: CADLink = {
      id: `cad-${Date.now()}`,
      cadFilePath: file.name,
      format,
      version: '1.0',
      importedAt: new Date().toISOString(),
      lastSyncAt: null,
      status: 'synced',
      parameters: [],
      geometries: await parseGeometries(file, format),
      changeLog: [{
        id: `chg-${Date.now()}`,
        timestamp: new Date().toISOString(),
        type: 'geometry',
        description: `导入 ${FORMAT_LABELS[format]}: ${file.name}`,
        source: 'cad',
        merged: true
      }],
      associatedSimulationId: undefined
    }

    cadLinks.value.push(link)
    return link
  }

  /**
   * 检测文件格式
   */
  function detectFormat(filename: string): CADFormat | null {
    const lower = filename.toLowerCase()
    if (lower.endsWith('.step') || lower.endsWith('.stp')) return 'step'
    if (lower.endsWith('.iges') || lower.endsWith('.igs')) return 'iges'
    if (lower.endsWith('.x_t') || lower.endsWith('.x_b')) return 'x_t'
    if (lower.endsWith('.stl')) return 'stl'
    if (lower.endsWith('.obj')) return 'obj'
    return null
  }

  /**
   * 模拟解析几何体（实际应由后端返回）
   */
  async function parseGeometries(_file: File, _format: CADFormat): Promise<CADGeometry[]> {
    await simulateDelay(300)
    return [
      { id: 'geo-1', name: 'Body-1', type: 'solid', visible: true, importedAt: new Date().toISOString(), lastModified: new Date().toISOString() },
      { id: 'geo-2', name: 'Body-2', type: 'solid', visible: true, importedAt: new Date().toISOString(), lastModified: new Date().toISOString() }
    ]
  }

  /**
   * 同步 CAD → CAE
   */
  async function syncFromCAD(linkId: string): Promise<void> {
    const link = cadLinks.value.find(l => l.id === linkId)
    if (!link) return

    isSyncing.value = true
    lastError.value = null

    try {
      // 生成同步预览
      syncPreview.value = await generateSyncPreview(link)

      // 用户确认后执行同步（简化：直接执行）
      await simulateDelay(600)

      // 更新参数
      link.parameters = link.parameters.map(p => ({ ...p, synced: true }))
      link.geometries = link.geometries.map(g => ({ ...g, lastModified: new Date().toISOString() }))
      link.lastSyncAt = new Date().toISOString()
      link.status = 'synced'
      link.version = incrementVersion(link.version)

      link.changeLog.push({
        id: `chg-${Date.now()}`,
        timestamp: new Date().toISOString(),
        type: 'geometry',
        description: `从 CAD 同步: ${link.cadFilePath}`,
        source: 'cad',
        merged: true
      })
    } catch (err) {
      lastError.value = err instanceof Error ? err.message : '同步失败'
      link.status = 'error'
    } finally {
      isSyncing.value = false
      syncPreview.value = null
    }
  }

  /**
   * 推送 CAE → CAD（参数变更）
   */
  async function pushToCAD(linkId: string, parameterChanges: Array<{ name: string; value: number }>): Promise<void> {
    const link = cadLinks.value.find(l => l.id === linkId)
    if (!link) return

    isSyncing.value = true
    lastError.value = null

    try {
      await simulateDelay(500)

      for (const change of parameterChanges) {
        const param = link.parameters.find(p => p.name === change.name)
        if (param) {
          param.value = change.value
          param.synced = true
        }
      }

      link.lastSyncAt = new Date().toISOString()
      link.status = 'synced'
      link.version = incrementVersion(link.version)

      link.changeLog.push({
        id: `chg-${Date.now()}`,
        timestamp: new Date().toISOString(),
        type: 'parameter',
        description: `CAE → CAD 参数推送: ${parameterChanges.map(c => c.name).join(', ')}`,
        source: 'cae',
        merged: true
      })
    } catch (err) {
      lastError.value = err instanceof Error ? err.message : '推送失败'
      link.status = 'error'
    } finally {
      isSyncing.value = false
    }
  }

  /**
   * 生成同步预览
   */
  async function generateSyncPreview(link: CADLink): Promise<SyncPreview> {
    await simulateDelay(200)
    return {
      additions: [],
      modifications: link.geometries.map(g => ({
        old: g,
        new: { ...g, lastModified: new Date().toISOString() }
      })),
      deletions: [],
      parameterChanges: link.parameters.filter(p => !p.synced).map(p => ({
        name: p.name,
        old: p.value,
        new: p.value
      }))
    }
  }

  /**
   * 更新参数（CAE 侧修改）
   */
  function updateParameter(linkId: string, name: string, value: number): void {
    const link = cadLinks.value.find(l => l.id === linkId)
    if (!link) return

    const param = link.parameters.find(p => p.name === name)
    if (param) {
      param.value = value
      param.synced = false
      link.status = 'dirty'
      link.changeLog.push({
        id: `chg-${Date.now()}`,
        timestamp: new Date().toISOString(),
        type: 'parameter',
        description: `CAE 修改参数: ${name} = ${value}`,
        source: 'cae',
        merged: false
      })
    }
  }

  /**
   * 关联仿真
   */
  function associateSimulation(linkId: string, simulationId: string): void {
    const link = cadLinks.value.find(l => l.id === linkId)
    if (link) {
      link.associatedSimulationId = simulationId
    }
  }

  /**
   * 移除 CAD 链接
   */
  function removeLink(linkId: string): void {
    cadLinks.value = cadLinks.value.filter(l => l.id !== linkId)
  }

  /**
   * 清除冲突
   */
  function resolveConflict(linkId: string, useCAD: boolean): void {
    const link = cadLinks.value.find(l => l.id === linkId)
    if (!link) return

    link.status = 'synced'
    link.changeLog.push({
      id: `chg-${Date.now()}`,
      timestamp: new Date().toISOString(),
      type: 'parameter',
      description: `冲突已解决: 采用 ${useCAD ? 'CAD' : 'CAE'} 版本`,
      source: useCAD ? 'cad' : 'cae',
      merged: true,
      conflict: false
    })
  }

  function incrementVersion(v: string): string {
    const parts = v.split('.')
    const last = parseInt(parts[parts.length - 1], 10) || 0
    parts[parts.length - 1] = (last + 1).toString()
    return parts.join('.')
  }

  function simulateDelay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms))
  }

  return {
    cadLinks,
    isSyncing,
    lastError,
    syncPreview,
    activeLinks,
    dirtyLinks,
    conflictLinks,
    FORMAT_LABELS,
    importCAD,
    syncFromCAD,
    pushToCAD,
    updateParameter,
    associateSimulation,
    removeLink,
    resolveConflict,
    detectFormat
  }
}
