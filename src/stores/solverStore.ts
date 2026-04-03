/**
 * 求解器管理 Store
 * 管理求解器检测、安装、卸载、验证等状态
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import {
  detectSolvers,
  checkSolverWorks,
  installSolver as apiInstallSolver,
  uninstallSolver as apiUninstallSolver,
  getInstallMethods,
  type SolverStatus,
  type SolverVerifyResult,
  type InstallResult,
  type UninstallResult,
  type InstallMethod,
} from '@/api/solverManager'

export const useSolverStore = defineStore('solver', () => {
  // ============================================
  // State
  // ============================================

  /** 求解器列表 */
  const solvers = ref<SolverStatus[]>([])

  /** 加载状态 */
  const loading = ref(false)

  /** 正在安装的求解器名称 */
  const installing = ref<string | null>(null)

  /** 安装进度消息 */
  const installProgress = ref('')

  /** 安装日志 */
  const installLogs = ref<string[]>([])

  /** 可用的安装方法 */
  const installMethods = ref<InstallMethod[]>([])

  /** 验证结果映射 */
  const verifyResults = ref<Record<string, SolverVerifyResult>>({})

  /** 选中的求解器名称集合（用于批量安装） */
  const selectedSolvers = ref<Set<string>>(new Set())

  /** 错误信息 */
  const error = ref<string | null>(null)

  // ============================================
  // Getters
  // ============================================

  /** 是否正在安装中 */
  const isInstalling = computed(() => installing.value !== null)

  /** 已安装的求解器 */
  const installedSolvers = computed(() =>
    solvers.value.filter(s => s.installed)
  )

  /** 未安装的求解器 */
  const notInstalledSolvers = computed(() =>
    solvers.value.filter(s => !s.installed)
  )

  /** 选中的未安装求解器数量 */
  const selectedCount = computed(() => {
    return [...selectedSolvers.value].filter(name => {
      const solver = solvers.value.find(s => s.name === name)
      return solver && !solver.installed
    }).length
  })

  /** 是否全部选中 */
  const allSelected = computed(() => {
    if (solvers.value.length === 0) return false
    return solvers.value.every(s => selectedSolvers.value.has(s.name))
  })

  // ============================================
  // Actions
  // ============================================

  /** 检测系统中的求解器 */
  async function detect() {
    loading.value = true
    error.value = null
    try {
      solvers.value = await detectSolvers()
      // 初始化选中状态：未安装的默认选中
      solvers.value.forEach(s => {
        if (!s.installed) {
          selectedSolvers.value.add(s.name)
        }
      })
    } catch (e) {
      error.value = String(e)
      console.error('Failed to detect solvers:', e)
    } finally {
      loading.value = false
    }
  }

  /** 加载可用的安装方法 */
  async function loadInstallMethods() {
    try {
      installMethods.value = await getInstallMethods()
    } catch (e) {
      console.error('Failed to load install methods:', e)
    }
  }

  /** 安装单个求解器 */
  async function install(solverName: string, method: string) {
    installing.value = solverName
    installProgress.value = ''
    installLogs.value = []
    error.value = null

    try {
      const result: InstallResult = await apiInstallSolver(solverName, method)
      if (result.success) {
        installProgress.value = `✓ ${result.message} (${result.duration_s.toFixed(1)}s)`
        installLogs.value.push(`[${solverName}] ${result.message}`)
        // 刷新状态
        await detect()
        // 自动验证
        await verify(solverName)
      } else {
        installProgress.value = `✗ ${result.message}`
        installLogs.value.push(`[${solverName}] FAILED: ${result.message}`)
      }
      return result
    } catch (e) {
      const msg = String(e)
      installProgress.value = `✗ ${msg}`
      installLogs.value.push(`[${solverName}] ERROR: ${msg}`)
      error.value = msg
      return { success: false, message: msg, duration_s: 0 }
    } finally {
      installing.value = null
    }
  }

  /** 批量安装选中的求解器 */
  async function installSelected(getMethod: (name: string) => string) {
    const toInstall = [...selectedSolvers.value].filter(name => {
      const solver = solvers.value.find(s => s.name === name)
      return solver && !solver.installed
    })

    const results: Record<string, InstallResult> = {}
    for (const name of toInstall) {
      const method = getMethod(name)
      results[name] = await install(name, method)
    }
    return results
  }

  /** 卸载求解器 */
  async function uninstall(solverName: string) {
    error.value = null
    try {
      const result: UninstallResult = await apiUninstallSolver(solverName)
      if (result.success) {
        // 刷新状态
        await detect()
        // 清除验证结果
        delete verifyResults.value[solverName]
      } else {
        error.value = result.message
      }
      return result
    } catch (e) {
      const msg = String(e)
      error.value = msg
      return { success: false, message: msg }
    }
  }

  /** 验证求解器 */
  async function verify(solverName: string) {
    try {
      const result = await checkSolverWorks(solverName)
      verifyResults.value[solverName] = result
      return result
    } catch (e) {
      const result: SolverVerifyResult = {
        works: false,
        version: null,
        error_message: String(e),
      }
      verifyResults.value[solverName] = result
      return result
    }
  }

  /** 验证所有已安装的求解器 */
  async function verifyAll() {
    const installed = solvers.value.filter(s => s.installed)
    const results: Record<string, SolverVerifyResult> = {}
    for (const solver of installed) {
      results[solver.name] = await verify(solver.name)
    }
    return results
  }

  /** 切换选中状态 */
  function toggleSelect(solverName: string) {
    if (selectedSolvers.value.has(solverName)) {
      selectedSolvers.value.delete(solverName)
    } else {
      selectedSolvers.value.add(solverName)
    }
  }

  /** 全选 */
  function selectAll() {
    solvers.value.forEach(s => selectedSolvers.value.add(s.name))
  }

  /** 取消全选 */
  function deselectAll() {
    selectedSolvers.value.clear()
  }

  /** 添加安装日志 */
  function addLog(message: string) {
    installLogs.value.push(message)
  }

  /** 清除错误 */
  function clearError() {
    error.value = null
  }

  return {
    // State
    solvers,
    loading,
    installing,
    installProgress,
    installLogs,
    installMethods,
    verifyResults,
    selectedSolvers,
    error,
    // Getters
    isInstalling,
    installedSolvers,
    notInstalledSolvers,
    selectedCount,
    allSelected,
    // Actions
    detect,
    loadInstallMethods,
    install,
    installSelected,
    uninstall,
    verify,
    verifyAll,
    toggleSelect,
    selectAll,
    deselectAll,
    addLog,
    clearError,
  }
})
