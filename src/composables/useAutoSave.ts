/**
 * 自动保存 Composable
 * 定期或在关键操作后自动保存项目状态
 */
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { useProjectStore } from '@/stores/project'

export interface AutoSaveConfig {
  /** 自动保存间隔（毫秒），默认 30000 (30秒) */
  interval: number
  /** 是否启用自动保存 */
  enabled: boolean
  /** 最大版本历史数量，默认 20 */
  maxVersions: number
}

export interface VersionSnapshot {
  id: string
  timestamp: string
  label: string
  /** 保存时的数据摘要 */
  summary: {
    hasMesh: boolean
    nodeCount: number
    elementCount: number
    hasBoundaryConditions: boolean
    hasResult: boolean
  }
  /** 完整的项目状态快照 */
  data: {
    mesh: any | null
    boundaryConditions: any
    lastResult: any | null
  }
}

export function useAutoSave(config?: Partial<AutoSaveConfig>) {
  const projectStore = useProjectStore()

  // 合并配置
  const fullConfig: AutoSaveConfig = {
    interval: 30000,
    enabled: true,
    maxVersions: 20,
    ...config,
  }

  const isAutoSaving = ref(false)
  const lastSaveTime = ref<Date | null>(null)
  const versions = ref<VersionSnapshot[]>([])
  let timer: ReturnType<typeof setInterval> | null = null
  let debounceTimer: ReturnType<typeof setTimeout> | null = null

  /** 获取 localStorage key */
  function getStorageKey(): string {
    const projectId = projectStore.currentProject?.id || 'default'
    return `caelab_versions_${projectId}`
  }

  /** 从 localStorage 加载版本历史 */
  function loadVersions() {
    try {
      const key = getStorageKey()
      const raw = localStorage.getItem(key)
      if (raw) {
        versions.value = JSON.parse(raw) as VersionSnapshot[]
      }
    } catch (e) {
      console.error('加载版本历史失败:', e)
      versions.value = []
    }
  }

  /** 将版本历史持久化到 localStorage */
  function persistVersions() {
    try {
      const key = getStorageKey()
      localStorage.setItem(key, JSON.stringify(versions.value))
    } catch (e) {
      console.error('保存版本历史失败:', e)
      // 如果是存储空间不足，尝试删除最旧的版本后重试
      if (e instanceof DOMException && e.name === 'QuotaExceededError') {
        if (versions.value.length > 1) {
          versions.value.shift()
          persistVersions()
        }
      }
    }
  }

  /** 生成唯一版本 ID */
  function generateVersionId(): string {
    return `ver_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
  }

  /** 生成版本标签 */
  function generateLabel(customLabel?: string): string {
    if (customLabel) return customLabel

    const now = new Date()
    const timeStr = now.toLocaleTimeString('zh-CN', {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: false,
    })

    // 根据当前状态生成描述性标签
    const parts: string[] = []
    if (projectStore.currentMesh) {
      parts.push('网格')
    }
    if (projectStore.hasBoundaryConditions) {
      parts.push('BC')
    }
    if (projectStore.hasResult) {
      parts.push('结果')
    }

    const stateDesc = parts.length > 0 ? parts.join('+') : '空项目'
    return `自动保存 [${stateDesc}] ${timeStr}`
  }

  /** 保存当前状态到版本历史 */
  function saveVersion(label?: string): VersionSnapshot {
    const snapshot: VersionSnapshot = {
      id: generateVersionId(),
      timestamp: new Date().toISOString(),
      label: generateLabel(label),
      summary: {
        hasMesh: projectStore.currentMesh !== null && projectStore.currentMesh.nodes.length > 0,
        nodeCount: projectStore.currentMesh?.nodes.length || 0,
        elementCount: projectStore.currentMesh?.elements.length || 0,
        hasBoundaryConditions: projectStore.hasBoundaryConditions,
        hasResult: projectStore.hasResult,
      },
      data: {
        mesh: projectStore.currentMesh ? JSON.parse(JSON.stringify(projectStore.currentMesh)) : null,
        boundaryConditions: JSON.parse(JSON.stringify(projectStore.boundaryConditions)),
        lastResult: projectStore.lastResult ? JSON.parse(JSON.stringify(projectStore.lastResult)) : null,
      },
    }

    // 添加到版本列表头部（最新的在前）
    versions.value.unshift(snapshot)

    // 限制最大版本数量
    if (versions.value.length > fullConfig.maxVersions) {
      versions.value = versions.value.slice(0, fullConfig.maxVersions)
    }

    // 持久化
    persistVersions()

    // 更新最后保存时间
    lastSaveTime.value = new Date()

    return snapshot
  }

  /** 恢复到指定版本 */
  function restoreVersion(versionId: string): boolean {
    const version = versions.value.find((v) => v.id === versionId)
    if (!version) {
      console.error('版本不存在:', versionId)
      return false
    }

    try {
      // 恢复网格数据
      if (version.data.mesh) {
        projectStore.currentMesh = JSON.parse(JSON.stringify(version.data.mesh))
      } else {
        projectStore.clearMesh()
      }

      // 恢复边界条件
      if (version.data.boundaryConditions) {
        projectStore.boundaryConditions = JSON.parse(JSON.stringify(version.data.boundaryConditions))
      } else {
        projectStore.clearBoundaryConditions()
      }

      // 恢复求解结果
      if (version.data.lastResult) {
        projectStore.lastResult = JSON.parse(JSON.stringify(version.data.lastResult))
      } else {
        projectStore.clearResult()
      }

      return true
    } catch (e) {
      console.error('恢复版本失败:', e)
      return false
    }
  }

  /** 删除指定版本 */
  function deleteVersion(versionId: string): void {
    const index = versions.value.findIndex((v) => v.id === versionId)
    if (index !== -1) {
      versions.value.splice(index, 1)
      persistVersions()
    }
  }

  /** 清除所有版本历史 */
  function clearVersions(): void {
    versions.value = []
    try {
      localStorage.removeItem(getStorageKey())
    } catch (e) {
      console.error('清除版本历史失败:', e)
    }
  }

  /** 自动保存触发 */
  function autoSave() {
    if (!fullConfig.enabled) return
    if (isAutoSaving.value) return

    // 如果没有任何数据变化，跳过保存
    if (
      !projectStore.currentMesh &&
      !projectStore.hasBoundaryConditions &&
      !projectStore.hasResult
    ) {
      return
    }

    isAutoSaving.value = true
    try {
      saveVersion()
    } finally {
      isAutoSaving.value = false
    }
  }

  /** 防抖自动保存 */
  function debouncedAutoSave() {
    if (debounceTimer) {
      clearTimeout(debounceTimer)
    }
    debounceTimer = setTimeout(() => {
      autoSave()
      debounceTimer = null
    }, 2000) // 防抖 2 秒
  }

  /** 启动定时自动保存 */
  function startAutoSave() {
    stopAutoSave()
    if (fullConfig.enabled) {
      timer = setInterval(() => {
        autoSave()
      }, fullConfig.interval)
    }
  }

  /** 停止定时自动保存 */
  function stopAutoSave() {
    if (timer) {
      clearInterval(timer)
      timer = null
    }
    if (debounceTimer) {
      clearTimeout(debounceTimer)
      debounceTimer = null
    }
  }

  /** 更新配置 */
  function updateConfig(newConfig: Partial<AutoSaveConfig>) {
    Object.assign(fullConfig, newConfig)
    // 如果间隔或启用状态改变，重新启动定时器
    if (newConfig.interval !== undefined || newConfig.enabled !== undefined) {
      startAutoSave()
    }
  }

  // 监听 store 变化，在关键操作后触发保存
  watch(
    () => [
      projectStore.currentMesh,
      projectStore.boundaryConditions,
      projectStore.lastResult,
    ],
    () => {
      if (fullConfig.enabled) {
        // 防抖 2 秒后自动保存
        debouncedAutoSave()
      }
    },
    { deep: true }
  )

  onMounted(() => {
    loadVersions()
    startAutoSave()
  })

  onUnmounted(() => {
    stopAutoSave()
  })

  return {
    isAutoSaving,
    lastSaveTime,
    versions,
    saveVersion,
    restoreVersion,
    deleteVersion,
    clearVersions,
    startAutoSave,
    stopAutoSave,
    updateConfig,
  }
}
