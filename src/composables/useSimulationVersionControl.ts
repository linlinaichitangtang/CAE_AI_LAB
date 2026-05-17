/**
 * useSimulationVersionControl.ts — V3.2-003 仿真版本管理
 * git-like diff (mesh/results 对比)，分支/合并
 */

import { ref, computed } from 'vue'

// ============ 类型定义 ============

export interface VersionSnapshot {
  id: string
  version: string
  label?: string
  description?: string
  createdAt: string
  createdBy: string
  parentId?: string  // 父版本 ID，用于构建版本树

  // 快照数据
  meshSnapshot?: {
    nodes: number
    elements: number
    nodeCoords?: Array<{ id: number; x: number; y: number; z: number }>
    elementConns?: Array<{ id: number; type: string; nodeIds: number[] }>
  }
  boundaryConditionsSnapshot?: any
  resultsSnapshot?: {
    maxDisplacement?: number
    maxStress?: number
    firstFrequency?: number
    [key: string]: any
  }
  parametersSnapshot?: Record<string, any>

  // 元数据
  projectId: string
  simulationType: string
  tags: string[]
  isPinned: boolean
}

export interface VersionDiff {
  hasChanges: boolean
  meshChanges: {
    nodesAdded: number
    nodesRemoved: number
    elementsAdded: number
    elementsRemoved: number
    nodeCoordsChanged: number
    connectivityChanged: number
  }
  resultChanges: {
    maxDisplacementDelta?: number
    maxStressDelta?: number
    firstFrequencyDelta?: number
  }
  parameterChanges: {
    added: string[]
    removed: string[]
    changed: Array<{ key: string; oldValue: any; newValue: any }>
  }
}

export interface VersionBranch {
  id: string
  name: string
  description?: string
  headVersionId: string
  createdAt: string
  createdBy: string
  color: string
}

export interface VersionTag {
  id: string
  name: string
  versionId: string
  createdAt: string
  createdBy: string
}

// ============ 存储键 ============

const VERSIONS_KEY = 'caelab_versions'
const BRANCHES_KEY = 'caelab_branches'
const TAGS_KEY = 'caelab_tags'

// ============ 工具函数 ============

function generateId(): string {
  return `ver_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function generateVersionNumber(major: number, minor: number, patch: number): string {
  return `${major}.${minor}.${patch}`
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

// ============ 主 Composable ============

export function useSimulationVersionControl() {
  const versions = ref<VersionSnapshot[]>([])
  const branches = ref<VersionBranch[]>([])
  const tags = ref<VersionTag[]>([])

  const currentBranch = ref<string>('main')
  const headVersionId = ref<string | null>(null)

  // ============ 初始化 ============

  function loadData(): void {
    const storedVersions = getStorage<VersionSnapshot[]>(VERSIONS_KEY)
    if (storedVersions) versions.value = storedVersions

    const storedBranches = getStorage<VersionBranch[]>(BRANCHES_KEY)
    if (storedBranches) branches.value = storedBranches

    const storedTags = getStorage<VersionTag[]>(TAGS_KEY)
    if (storedTags) tags.value = storedTags

    // 确保 main 分支存在
    if (!branches.value.find(b => b.name === 'main')) {
      createBranch('main', '主分支')
    }
  }

  // ============ 版本管理 ============

  /**
   * 创建新版本快照
   */
  function createSnapshot(
    projectId: string,
    data: {
      label?: string
      description?: string
      meshSnapshot?: VersionSnapshot['meshSnapshot']
      boundaryConditionsSnapshot?: any
      resultsSnapshot?: VersionSnapshot['resultsSnapshot']
      parametersSnapshot?: Record<string, any>
      simulationType: string
      createdBy?: string
    }
  ): VersionSnapshot {
    const latestVersion = getLatestVersion(projectId)
    const [major, minor, patch] = latestVersion
      ? latestVersion.version.split('.').map(Number)
      : [1, 0, 0]

    const newVersion: VersionSnapshot = {
      id: generateId(),
      version: generateVersionNumber(major, minor, patch + 1),
      label: data.label,
      description: data.description,
      createdAt: new Date().toISOString(),
      createdBy: data.createdBy || localStorage.getItem('caelab_username') || 'Unknown',
      parentId: latestVersion?.id,
      projectId,
      simulationType: data.simulationType,
      meshSnapshot: data.meshSnapshot,
      boundaryConditionsSnapshot: data.boundaryConditionsSnapshot,
      resultsSnapshot: data.resultsSnapshot,
      parametersSnapshot: data.parametersSnapshot,
      tags: [],
      isPinned: false
    }

    versions.value.unshift(newVersion)
    headVersionId.value = newVersion.id

    // 更新分支头
    const branch = branches.value.find(b => b.name === currentBranch.value)
    if (branch) {
      branch.headVersionId = newVersion.id
    }

    saveData()

    return newVersion
  }

  /**
   * 获取项目最新版本
   */
  function getLatestVersion(projectId: string): VersionSnapshot | undefined {
    return versions.value.find(v => v.projectId === projectId && v.id === headVersionId.value)
  }

  /**
   * 获取项目所有版本
   */
  function getProjectVersions(projectId: string): VersionSnapshot[] {
    return versions.value
      .filter(v => v.projectId === projectId)
      .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
  }

  /**
   * 获取版本详情
   */
  function getVersion(versionId: string): VersionSnapshot | undefined {
    return versions.value.find(v => v.id === versionId)
  }

  /**
   * 删除版本
   */
  function deleteVersion(versionId: string): boolean {
    const index = versions.value.findIndex(v => v.id === versionId)
    if (index === -1) return false

    versions.value.splice(index, 1)

    // 如果删除的是头版本，需要更新
    if (headVersionId.value === versionId) {
      const projectId = versions.value[index]?.projectId
      const latest = getLatestVersion(projectId)
      headVersionId.value = latest?.id || null
    }

    saveData()
    return true
  }

  /**
   * 标记版本
   */
  function pinVersion(versionId: string): void {
    const version = versions.value.find(v => v.id === versionId)
    if (version) {
      version.isPinned = true
      saveData()
    }
  }

  function unpinVersion(versionId: string): void {
    const version = versions.value.find(v => v.id === versionId)
    if (version) {
      version.isPinned = false
      saveData()
    }
  }

  /**
   * 恢复版本
   */
  function restoreVersion(versionId: string): VersionSnapshot | null {
    const version = versions.value.find(v => v.id === versionId)
    if (!version) return null

    // 创建新版本作为恢复点
    return createSnapshot(version.projectId, {
      label: `Restore to ${version.version}`,
      description: `Restored from version ${version.version}`,
      meshSnapshot: version.meshSnapshot,
      boundaryConditionsSnapshot: version.boundaryConditionsSnapshot,
      resultsSnapshot: version.resultsSnapshot,
      parametersSnapshot: version.parametersSnapshot,
      simulationType: version.simulationType
    })
  }

  // ============ 版本对比 ============

  /**
   * 对比两个版本
   */
  function diffVersions(versionIdA: string, versionIdB: string): VersionDiff {
    const versionA = versions.value.find(v => v.id === versionIdA)
    const versionB = versions.value.find(v => v.id === versionIdB)

    if (!versionA || !versionB) {
      return {
        hasChanges: false,
        meshChanges: { nodesAdded: 0, nodesRemoved: 0, elementsAdded: 0, elementsRemoved: 0, nodeCoordsChanged: 0, connectivityChanged: 0 },
        resultChanges: { maxDisplacementDelta: undefined, maxStressDelta: undefined, firstFrequencyDelta: undefined },
        parameterChanges: { added: [] as string[], removed: [] as string[], changed: [] as any[] }
      } as VersionDiff
    }

    const meshChanges = {
      nodesAdded: (versionB.meshSnapshot?.nodes || 0) - (versionA.meshSnapshot?.nodes || 0),
      nodesRemoved: 0,
      elementsAdded: (versionB.meshSnapshot?.elements || 0) - (versionA.meshSnapshot?.elements || 0),
      elementsRemoved: 0,
      nodeCoordsChanged: 0,
      connectivityChanged: 0
    }

    const resultChanges: VersionDiff['resultChanges'] = {}

    if (versionA.resultsSnapshot?.maxDisplacement !== undefined && versionB.resultsSnapshot?.maxDisplacement !== undefined) {
      resultChanges.maxDisplacementDelta = versionB.resultsSnapshot.maxDisplacement - versionA.resultsSnapshot.maxDisplacement
    }
    if (versionA.resultsSnapshot?.maxStress !== undefined && versionB.resultsSnapshot?.maxStress !== undefined) {
      resultChanges.maxStressDelta = versionB.resultsSnapshot.maxStress - versionA.resultsSnapshot.maxStress
    }
    if (versionA.resultsSnapshot?.firstFrequency !== undefined && versionB.resultsSnapshot?.firstFrequency !== undefined) {
      resultChanges.firstFrequencyDelta = versionB.resultsSnapshot.firstFrequency - versionA.resultsSnapshot.firstFrequency
    }

    // 参数变化
    const paramChanges = {
      added: [] as string[],
      removed: [] as string[],
      changed: [] as Array<{ key: string; oldValue: any; newValue: any }>
    }

    const paramsA = versionA.parametersSnapshot || {}
    const paramsB = versionB.parametersSnapshot || {}

    for (const key of Object.keys(paramsB)) {
      if (!(key in paramsA)) {
        paramChanges.added.push(key)
      } else if (paramsA[key] !== paramsB[key]) {
        paramChanges.changed.push({ key, oldValue: paramsA[key], newValue: paramsB[key] })
      }
    }

    for (const key of Object.keys(paramsA)) {
      if (!(key in paramsB)) {
        paramChanges.removed.push(key)
      }
    }

    return {
      hasChanges: meshChanges.nodesAdded !== 0 || meshChanges.elementsAdded !== 0 || resultChanges.maxDisplacementDelta !== undefined || paramChanges.added.length > 0,
      meshChanges,
      resultChanges,
      parameterChanges: paramChanges
    }
  }

  /**
   * 生成版本对比报告
   */
  function generateDiffReport(versionIdA: string, versionIdB: string): string {
    const diff = diffVersions(versionIdA, versionIdB)
    const versionA = getVersion(versionIdA)
    const versionB = getVersion(versionIdB)

    if (!versionA || !versionB) return ''

    const lines: string[] = []

    lines.push('=' .repeat(60))
    lines.push('VERSION DIFF REPORT')
    lines.push('=' .repeat(60))
    lines.push('')
    lines.push(`From: ${versionA.version} (${versionA.label || 'no label'})`)
    lines.push(`To:   ${versionB.version} (${versionB.label || 'no label'})`)
    lines.push(`Date: ${new Date().toLocaleString()}`)
    lines.push('')

    if (!diff.hasChanges) {
      lines.push('No significant changes detected.')
      return lines.join('\n')
    }

    // 网格变化
    if (diff.meshChanges.nodesAdded !== 0 || diff.meshChanges.elementsAdded !== 0) {
      lines.push('MESH CHANGES:')
      lines.push(`  Nodes: ${diff.meshChanges.nodesAdded > 0 ? '+' : ''}${diff.meshChanges.nodesAdded}`)
      lines.push(`  Elements: ${diff.meshChanges.elementsAdded > 0 ? '+' : ''}${diff.meshChanges.elementsAdded}`)
      lines.push('')
    }

    // 结果变化
    if (diff.resultChanges.maxDisplacementDelta !== undefined) {
      lines.push('RESULTS CHANGES:')
      lines.push(`  Max Displacement: ${diff.resultChanges.maxDisplacementDelta > 0 ? '+' : ''}${diff.resultChanges.maxDisplacementDelta.toFixed(6)}`)
    }
    if (diff.resultChanges.maxStressDelta !== undefined) {
      lines.push(`  Max Stress: ${diff.resultChanges.maxStressDelta > 0 ? '+' : ''}${diff.resultChanges.maxStressDelta.toFixed(2)}`)
    }
    if (diff.resultChanges.firstFrequencyDelta !== undefined) {
      lines.push(`  First Frequency: ${diff.resultChanges.firstFrequencyDelta > 0 ? '+' : ''}${diff.resultChanges.firstFrequencyDelta.toFixed(2)}`)
    }
    if (Object.keys(diff.resultChanges).length > 0) lines.push('')

    // 参数变化
    if (diff.parameterChanges.added.length > 0) {
      lines.push('PARAMETERS ADDED:')
      diff.parameterChanges.added.forEach(key => {
        lines.push(`  + ${key}`)
      })
      lines.push('')
    }
    if (diff.parameterChanges.removed.length > 0) {
      lines.push('PARAMETERS REMOVED:')
      diff.parameterChanges.removed.forEach(key => {
        lines.push(`  - ${key}`)
      })
      lines.push('')
    }
    if (diff.parameterChanges.changed.length > 0) {
      lines.push('PARAMETERS CHANGED:')
      diff.parameterChanges.changed.forEach(({ key, oldValue, newValue }) => {
        lines.push(`  ${key}: ${JSON.stringify(oldValue)} → ${JSON.stringify(newValue)}`)
      })
      lines.push('')
    }

    return lines.join('\n')
  }

  // ============ 分支管理 ============

  /**
   * 创建分支
   */
  function createBranch(name: string, description?: string): VersionBranch {
    const branchColors = ['#3B82F6', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#EC4899']
    const color = branchColors[branches.value.length % branchColors.length]

    const branch: VersionBranch = {
      id: generateId(),
      name,
      description,
      headVersionId: headVersionId.value || '',
      createdAt: new Date().toISOString(),
      createdBy: localStorage.getItem('caelab_username') || 'Unknown',
      color
    }

    branches.value.push(branch)
    saveData()

    return branch
  }

  /**
   * 切换分支
   */
  function switchBranch(branchName: string): boolean {
    const branch = branches.value.find(b => b.name === branchName)
    if (!branch) return false

    currentBranch.value = branchName
    headVersionId.value = branch.headVersionId

    return true
  }

  /**
   * 删除分支
   */
  function deleteBranch(branchName: string): boolean {
    if (branchName === 'main') return false  // 不能删除主分支

    const index = branches.value.findIndex(b => b.name === branchName)
    if (index === -1) return false

    branches.value.splice(index, 1)

    // 如果删除了当前分支，切换到 main
    if (currentBranch.value === branchName) {
      switchBranch('main')
    }

    saveData()
    return true
  }

  /**
   * 合并分支
   */
  function mergeBranch(sourceBranchName: string, targetBranchName: string): { success: boolean; conflicts?: string[] } {
    const sourceBranch = branches.value.find(b => b.name === sourceBranchName)
    const targetBranch = branches.value.find(b => b.name === targetBranchName)

    if (!sourceBranch || !targetBranch) return { success: false, conflicts: ['Branch not found'] }

    // 获取源分支的最新版本
    const sourceVersion = versions.value.find(v => v.id === sourceBranch.headVersionId)
    if (!sourceVersion) return { success: false, conflicts: ['Source branch has no versions'] }

    // 在目标分支创建新版本
    createSnapshot(sourceVersion.projectId, {
      label: `Merge from ${sourceBranchName}`,
      description: `Merged branch ${sourceBranchName} into ${targetBranchName}`,
      meshSnapshot: sourceVersion.meshSnapshot,
      boundaryConditionsSnapshot: sourceVersion.boundaryConditionsSnapshot,
      resultsSnapshot: sourceVersion.resultsSnapshot,
      parametersSnapshot: sourceVersion.parametersSnapshot,
      simulationType: sourceVersion.simulationType
    })

    return { success: true }
  }

  // ============ 标签管理 ============

  /**
   * 添加标签
   */
  function addTag(versionId: string, tagName: string): VersionTag | null {
    const version = versions.value.find(v => v.id === versionId)
    if (!version) return null

    // 检查是否已存在
    if (tags.value.find(t => t.name === tagName && t.versionId === versionId)) {
      return null
    }

    const tag: VersionTag = {
      id: generateId(),
      name: tagName,
      versionId,
      createdAt: new Date().toISOString(),
      createdBy: localStorage.getItem('caelab_username') || 'Unknown'
    }

    tags.value.push(tag)

    // 同时给版本添加标签
    version.tags.push(tagName)

    saveData()
    return tag
  }

  /**
   * 删除标签
   */
  function removeTag(tagId: string): boolean {
    const tag = tags.value.find(t => t.id === tagId)
    if (!tag) return false

    const version = versions.value.find(v => v.id === tag.versionId)
    if (version) {
      version.tags = version.tags.filter(t => t !== tag.name)
    }

    tags.value = tags.value.filter(t => t.id !== tagId)
    saveData()
    return true
  }

  /**
   * 获取版本的标签
   */
  function getVersionTags(versionId: string): VersionTag[] {
    return tags.value.filter(t => t.versionId === versionId)
  }

  // ============ 版本树 ============

  /**
   * 获取版本的祖先链
   */
  function getVersionAncestors(versionId: string): VersionSnapshot[] {
    const ancestors: VersionSnapshot[] = []
    let current = versions.value.find(v => v.id === versionId)

    while (current?.parentId) {
      const parent = versions.value.find(v => v.id === current!.parentId)
      if (parent) {
        ancestors.push(parent)
        current = parent
      } else {
        break
      }
    }

    return ancestors
  }

  /**
   * 获取版本的分支
   */
  function getVersionBranches(versionId: string): VersionBranch[] {
    return branches.value.filter(b => b.headVersionId === versionId)
  }

  // ============ 统计 ============

  function getStats(): {
    totalVersions: number
    totalBranches: number
    totalTags: number
    projectsWithVersions: number
  } {
    return {
      totalVersions: versions.value.length,
      totalBranches: branches.value.length,
      totalTags: tags.value.length,
      projectsWithVersions: new Set(versions.value.map(v => v.projectId)).size
    }
  }

  // ============ 持久化 ============

  function saveData(): void {
    setStorage(VERSIONS_KEY, versions.value)
    setStorage(BRANCHES_KEY, branches.value)
    setStorage(TAGS_KEY, tags.value)
  }

  function clearData(): void {
    versions.value = []
    branches.value = []
    tags.value = []
    saveData()
  }

  // 初始化
  loadData()

  return {
    // 状态
    versions,
    branches,
    tags,
    currentBranch,
    headVersionId,

    // 版本管理
    createSnapshot,
    getLatestVersion,
    getProjectVersions,
    getVersion,
    deleteVersion,
    pinVersion,
    unpinVersion,
    restoreVersion,

    // 版本对比
    diffVersions,
    generateDiffReport,

    // 分支管理
    createBranch,
    switchBranch,
    deleteBranch,
    mergeBranch,

    // 标签管理
    addTag,
    removeTag,
    getVersionTags,

    // 版本树
    getVersionAncestors,
    getVersionBranches,

    // 统计
    getStats,

    // 持久化
    clearData
  }
}