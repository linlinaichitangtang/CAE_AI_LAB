/**
 * useSimulationDatabase.ts — V3.2-001 仿真数据库
 * 仿真结果时间序列存储、标签搜索、批量对比
 */

import { ref, computed } from 'vue'

// ============ 类型定义 ============

export interface SimulationRecord {
  id: string
  projectId: string
  projectName: string
  name: string
  description?: string
  createdAt: string
  updatedAt: string

  // 标签
  tags: string[]
  category?: string

  // 仿真参数
  simulationType: 'static' | 'modal' | 'thermal' | 'transient' | 'buckling' | 'cfd' | 'multiscale'
  meshInfo: {
    nodes: number
    elements: number
    elementType: string
  }
  materialName?: string

  // 结果摘要
  results: {
    maxDisplacement?: number
    maxStress?: number
    maxStrain?: number
    firstFrequency?: number
    safetyFactor?: number
  }

  // 文件引用
  files: SimulationFile[]

  // 元数据
  author: string
  version: string
  isFavorite: boolean
  isArchived: boolean
}

export interface SimulationFile {
  id: string
  type: 'mesh' | 'results' | 'report' | 'input' | 'output' | 'image' | 'video'
  name: string
  size: number
  path: string
  createdAt: string
}

export interface SearchFilters {
  query?: string
  tags?: string[]
  categories?: string[]
  simulationTypes?: SimulationRecord['simulationType'][]
  dateRange?: { start: string; end: string }
  author?: string
  hasResults?: boolean
}

export interface ComparisonResult {
  recordId: string
  recordName: string
  metric: string
  value: number
  deviation?: number  // 与基准的偏差百分比
}

// ============ 存储键 ============

const DB_KEY = 'caelab_simulation_db'
const TAGS_KEY = 'caelab_simulation_tags'
const CATEGORIES_KEY = 'caelab_simulation_categories'

// ============ 工具函数 ============

function generateId(): string {
  return `sim_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

// ============ 主 Composable ============

export function useSimulationDatabase() {
  const records = ref<SimulationRecord[]>([])
  const allTags = ref<string[]>([])
  const allCategories = ref<string[]>([])

  // 加载数据
  function loadDatabase(): SimulationRecord[] {
    const stored = getStorage<SimulationRecord[]>(DB_KEY)
    if (stored) {
      records.value = stored
      updateTagAndCategoryLists()
    }
    return records.value
  }

  // 更新标签和分类列表
  function updateTagAndCategoryLists() {
    const tags = new Set<string>()
    const categories = new Set<string>()

    for (const record of records.value) {
      record.tags.forEach(tag => tags.add(tag))
      if (record.category) categories.add(record.category)
    }

    allTags.value = Array.from(tags).sort()
    allCategories.value = Array.from(categories).sort()
  }

  // 保存数据
  function saveDatabase(): void {
    setStorage(DB_KEY, records.value)
    updateTagAndCategoryLists()
  }

  // ============ CRUD 操作 ============

  /**
   * 添加仿真记录
   */
  function addRecord(data: Omit<SimulationRecord, 'id' | 'createdAt' | 'updatedAt' | 'isFavorite' | 'isArchived'>): SimulationRecord {
    const record: SimulationRecord = {
      ...data,
      id: generateId(),
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      isFavorite: false,
      isArchived: false
    }

    records.value.unshift(record)
    saveDatabase()

    return record
  }

  /**
   * 更新仿真记录
   */
  function updateRecord(recordId: string, updates: Partial<SimulationRecord>): SimulationRecord | null {
    const index = records.value.findIndex(r => r.id === recordId)
    if (index === -1) return null

    records.value[index] = {
      ...records.value[index],
      ...updates,
      updatedAt: new Date().toISOString()
    }

    saveDatabase()
    return records.value[index]
  }

  /**
   * 删除仿真记录
   */
  function deleteRecord(recordId: string): boolean {
    const index = records.value.findIndex(r => r.id === recordId)
    if (index === -1) return false

    records.value.splice(index, 1)
    saveDatabase()
    return true
  }

  /**
   * 批量删除
   */
  function deleteRecords(recordIds: string[]): number {
    let deleted = 0
    for (const id of recordIds) {
      if (deleteRecord(id)) deleted++
    }
    return deleted
  }

  // ============ 查询操作 ============

  /**
   * 搜索仿真记录
   */
  function searchRecords(filters: SearchFilters): SimulationRecord[] {
    let result = [...records.value]

    // 文本搜索
    if (filters.query) {
      const query = filters.query.toLowerCase()
      result = result.filter(r =>
        r.name.toLowerCase().includes(query) ||
        r.projectName.toLowerCase().includes(query) ||
        r.description?.toLowerCase().includes(query) ||
        r.tags.some(t => t.toLowerCase().includes(query))
      )
    }

    // 标签过滤
    if (filters.tags?.length) {
      result = result.filter(r =>
        filters.tags!.every(tag => r.tags.includes(tag))
      )
    }

    // 分类过滤
    if (filters.categories?.length) {
      result = result.filter(r =>
        r.category && filters.categories!.includes(r.category)
      )
    }

    // 仿真类型过滤
    if (filters.simulationTypes?.length) {
      result = result.filter(r =>
        filters.simulationTypes!.includes(r.simulationType)
      )
    }

    // 日期范围
    if (filters.dateRange) {
      const start = new Date(filters.dateRange.start).getTime()
      const end = new Date(filters.dateRange.end).getTime()
      result = result.filter(r => {
        const created = new Date(r.createdAt).getTime()
        return created >= start && created <= end
      })
    }

    // 作者过滤
    if (filters.author) {
      result = result.filter(r =>
        r.author.toLowerCase().includes(filters.author!.toLowerCase())
      )
    }

    // 结果过滤
    if (filters.hasResults !== undefined) {
      result = result.filter(r =>
        filters.hasResults
          ? (r.results.maxDisplacement !== undefined || r.results.maxStress !== undefined)
          : (r.results.maxDisplacement === undefined && r.results.maxStress === undefined)
      )
    }

    return result
  }

  /**
   * 获取单个记录
   */
  function getRecord(recordId: string): SimulationRecord | undefined {
    return records.value.find(r => r.id === recordId)
  }

  /**
   * 获取项目所有仿真记录
   */
  function getProjectRecords(projectId: string): SimulationRecord[] {
    return records.value.filter(r => r.projectId === projectId)
  }

  // ============ 收藏和归档 ============

  function toggleFavorite(recordId: string): boolean {
    const record = records.value.find(r => r.id === recordId)
    if (!record) return false

    record.isFavorite = !record.isFavorite
    saveDatabase()
    return record.isFavorite
  }

  function toggleArchive(recordId: string): boolean {
    const record = records.value.find(r => r.id === recordId)
    if (!record) return false

    record.isArchived = !record.isArchived
    saveDatabase()
    return record.isArchived
  }

  // ============ 时间序列分析 ============

  /**
   * 获取某指标的时间序列数据
   */
  function getTimeSeries(
    metric: 'maxDisplacement' | 'maxStress' | 'maxStrain' | 'firstFrequency',
    filters?: SearchFilters
  ): Array<{ date: string; value: number; recordId: string; recordName: string }> {
    const records = filters ? searchRecords(filters) : records.value

    return records
      .filter(r => r.results[metric] !== undefined && !r.isArchived)
      .map(r => ({
        date: r.createdAt,
        value: r.results[metric] as number,
        recordId: r.id,
        recordName: r.name
      }))
      .sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())
  }

  // ============ 对比分析 ============

  /**
   * 对比多条仿真记录
   */
  function compareRecords(recordIds: string[], metrics: Array<'maxDisplacement' | 'maxStress' | 'maxStrain' | 'firstFrequency'>): ComparisonResult[] {
    const results: ComparisonResult[] = []

    for (const id of recordIds) {
      const record = records.value.find(r => r.id === id)
      if (!record) continue

      for (const metric of metrics) {
        if (record.results[metric] !== undefined) {
          results.push({
            recordId: record.id,
            recordName: record.name,
            metric,
            value: record.results[metric] as number
          })
        }
      }
    }

    return results
  }

  /**
   * 计算偏差百分比（相对于基准）
   */
  function calculateDeviations(recordIds: string[], baselineId: string, metric: 'maxDisplacement' | 'maxStress' | 'firstFrequency'): Array<{ recordId: string; recordName: string; value: number; deviation: number }> {
    const baseline = records.value.find(r => r.id === baselineId)
    if (!baseline || baseline.results[metric] === undefined) return []

    const baselineValue = baseline.results[metric] as number
    const recordsToCompare = records.value.filter(r => recordIds.includes(r.id) && r.results[metric] !== undefined)

    return recordsToCompare.map(r => ({
      recordId: r.id,
      recordName: r.name,
      value: r.results[metric] as number,
      deviation: ((r.results[metric] as number - baselineValue) / baselineValue) * 100
    }))
  }

  // ============ 统计信息 ============

  function getStats(): {
    totalRecords: number
    totalProjects: number
    totalTags: number
    favoritesCount: number
    archivedCount: number
    byType: Record<string, number>
    byMonth: Record<string, number>
  } {
    const byType: Record<string, number> = {}
    const byMonth: Record<string, number> = {}

    for (const record of records.value) {
      byType[record.simulationType] = (byType[record.simulationType] || 0) + 1

      const month = record.createdAt.slice(0, 7) // YYYY-MM
      byMonth[month] = (byMonth[month] || 0) + 1
    }

    return {
      totalRecords: records.value.length,
      totalProjects: new Set(records.value.map(r => r.projectId)).size,
      totalTags: allTags.value.length,
      favoritesCount: records.value.filter(r => r.isFavorite).length,
      archivedCount: records.value.filter(r => r.isArchived).length,
      byType,
      byMonth
    }
  }

  // ============ 标签管理 ============

  function addTag(recordId: string, tag: string): boolean {
    const record = records.value.find(r => r.id === recordId)
    if (!record) return false

    if (!record.tags.includes(tag)) {
      record.tags.push(tag)
      saveDatabase()
    }

    return true
  }

  function removeTag(recordId: string, tag: string): boolean {
    const record = records.value.find(r => r.id === recordId)
    if (!record) return false

    record.tags = record.tags.filter(t => t !== tag)
    saveDatabase()
    return true
  }

  function getRelatedTags(tag: string): string[] {
    // 找到使用该标签的所有记录，返回这些记录的其他标签
    const relatedRecords = records.value.filter(r => r.tags.includes(tag))
    const relatedTags = new Set<string>()

    for (const r of relatedRecords) {
      for (const t of r.tags) {
        if (t !== tag) relatedTags.add(t)
      }
    }

    return Array.from(relatedTags)
  }

  // ============ 导出/导入 ============

  function exportRecord(recordId: string): string | null {
    const record = records.value.find(r => r.id === recordId)
    if (!record) return null
    return JSON.stringify(record, null, 2)
  }

  function importRecord(jsonString: string): SimulationRecord | null {
    try {
      const data = JSON.parse(jsonString)
      // 重置 ID 和时间戳
      const record = addRecord({
        ...data,
        id: undefined,
        createdAt: undefined,
        updatedAt: undefined
      } as any)
      return record
    } catch {
      return null
    }
  }

  // ============ 初始化 ============

  loadDatabase()

  return {
    records,
    allTags,
    allCategories,

    // CRUD
    addRecord,
    updateRecord,
    deleteRecord,
    deleteRecords,
    getRecord,
    getProjectRecords,
    searchRecords,

    // 收藏/归档
    toggleFavorite,
    toggleArchive,

    // 时间序列
    getTimeSeries,

    // 对比分析
    compareRecords,
    calculateDeviations,

    // 统计
    getStats,

    // 标签
    addTag,
    removeTag,
    getRelatedTags,

    // 导出导入
    exportRecord,
    importRecord
  }
}

// ============ 预定义标签 ============

export const PRESET_TAGS = [
  // 状态
  'draft', 'validated', 'production', 'archived',
  // 类型
  'cantilever', 'plate', 'shell', 'bracket', 'beam',
  // 行业
  'aerospace', 'automotive', 'civil', 'biomedical',
  // 状态
  'optimized', 'failed', 'converged',
  // 优先级
  'urgent', 'review-needed', 'approved'
]

export const PRESET_CATEGORIES = [
  '结构分析',
  '热分析',
  '模态分析',
  '流体分析',
  '多尺度',
  '优化设计',
  '失效分析',
  '参数扫描'
]