/**
 * useContainerizedExecution.ts — V3.0-004 容器化执行环境
 * Docker 化仿真环境，确保所有学生环境一致、结果可复现
 */

import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export interface ContainerImage {
  id: string
  name: string
  tag: string
  fullName: string
  size: number
  created: string
  description?: string
}

export interface ContainerInstance {
  id: string
  imageId: string
  imageName: string
  status: 'starting' | 'running' | 'stopped' | 'error'
  createdAt: string
  ports?: { host: number; container: number }[]
  volumeMounts?: { host: string; container: string }[]
}

export interface ExecutionEnvironment {
  id: string
  name: string
  image: string
  description: string
  packages: string[]
  envVars: Record<string, string>
  workingDir: string
}

export interface ExecutionResult {
  success: boolean
  stdout: string
  stderr: string
  exitCode: number
  duration: number
  memoryUsed?: number
}

export interface ExecutionRequest {
  code: string
  language: 'python' | 'julia' | 'octave' | 'fortran'
  timeout?: number
  environmentId?: string
}

// ============ 常量 ============

const CONTAINER_IMAGES_KEY = 'caelab_container_images'
const EXECUTION_ENVIRONMENTS_KEY = 'caelab_execution_environments'

// 预定义的执行环境
export const PREDEFINED_ENVIRONMENTS: ExecutionEnvironment[] = [
  {
    id: 'cae-base',
    name: 'CAE Base',
    image: 'caelab/cae-base:v2.9',
    description: '基础 CAE 环境，包含 Python、NumPy、SciPy、Matplotlib',
    packages: ['python>=3.10', 'numpy', 'scipy', 'matplotlib', 'pandas', 'meshio', 'pyvista'],
    envVars: {},
    workingDir: '/workspace'
  },
  {
    id: 'cae-advanced',
    name: 'CAE Advanced',
    image: 'caelab/cae-advanced:v2.9',
    description: '高级 CAE 环境，包含 FEA、分子动力学、相场模拟工具',
    packages: ['python>=3.10', 'numpy', 'scipy', 'fenics', 'petsc4py', 'pyamg', 'matplotlib', 'pandas'],
    envVars: { 'OMP_NUM_THREADS': '4' },
    workingDir: '/workspace'
  },
  {
    id: 'cae-multiscale',
    name: 'Multi-scale Suite',
    image: 'caelab/cae-multiscale:v2.9',
    description: '多尺度模拟专用环境 (DFT/MD/Phase Field/FE)',
    packages: ['python>=3.10', 'numpy', 'scipy', 'ase', 'mdanalysis', 'fenics', 'py-pde'],
    envVars: { 'OPENMM_CPU_THREADS': '4', 'OMP_NUM_THREADS': '4' },
    workingDir: '/workspace'
  },
  {
    id: 'cae-education',
    name: 'Education',
    image: 'caelab/cae-education:v2.9',
    description: '教学环境，简化配置，适合学生作业',
    packages: ['python>=3.10', 'numpy', 'scipy', 'matplotlib', 'pandas', 'jupyter'],
    envVars: {},
    workingDir: '/home/student'
  }
]

// ============ 工具函数 ============

function generateId(): string {
  return `cnt_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`
}

// ============ 主 Composable ============

export function useContainerizedExecution() {
  // 状态
  const availableImages = ref<ContainerImage[]>([])
  const runningContainers = ref<ContainerInstance[]>([])
  const currentEnvironment = ref<ExecutionEnvironment | null>(null)
  const isDockerAvailable = ref<boolean | null>(null)
  const isLoading = ref(false)
  const lastError = ref<string | null>(null)

  // 执行状态
  const isExecuting = ref(false)
  const executionHistory = ref<Array<{
    id: string
    code: string
    language: string
    result: ExecutionResult
    timestamp: string
  }>>([])

  // ============ Docker 检测 ============

  /**
   * 检测 Docker 是否可用
   */
  async function checkDockerAvailability(): Promise<boolean> {
    try {
      // 尝试通过 Tauri 后端检测 Docker
      const result = await invoke('check_docker_available').catch(() => null)

      // Fallback: 尝试直接检测
      if (result === null) {
        try {
          const response = await fetch('/api/docker/info', { method: 'GET' })
          isDockerAvailable.value = response.ok
        } catch {
          // 可能是 WebSocket 或后端检测
          isDockerAvailable.value = false
        }
      } else {
        isDockerAvailable.value = result as boolean
      }

      return isDockerAvailable.value ?? false
    } catch {
      isDockerAvailable.value = false
      return false
    }
  }

  // ============ 镜像管理 ============

  /**
   * 获取可用镜像列表
   */
  async function listImages(): Promise<ContainerImage[]> {
    isLoading.value = true
    lastError.value = null

    try {
      // 通过后端获取镜像列表
      const images = await invoke('list_container_images').catch(() => null)

      if (images) {
        availableImages.value = images as ContainerImage[]
      } else {
        // Fallback: 从本地存储加载预定义镜像
        availableImages.value = PREDEFINED_ENVIRONMENTS.map((env, idx) => ({
          id: `img_${idx}`,
          name: env.image.split(':')[0],
          tag: env.image.split(':')[1] || 'latest',
          fullName: env.image,
          size: 0,
          created: new Date().toISOString(),
          description: env.description
        }))
      }

      return availableImages.value
    } catch (e: any) {
      lastError.value = e.message || '获取镜像列表失败'
      return []
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 拉取镜像
   */
  async function pullImage(imageName: string, onProgress?: (status: string) => void): Promise<boolean> {
    isLoading.value = true
    lastError.value = null

    try {
      await invoke('pull_container_image', { imageName })
      await listImages() // 刷新列表
      return true
    } catch (e: any) {
      lastError.value = e.message || '拉取镜像失败'
      return false
    } finally {
      isLoading.value = false
    }
  }

  // ============ 容器管理 ============

  /**
   * 启动容器
   */
  async function startContainer(environment: ExecutionEnvironment): Promise<ContainerInstance | null> {
    isLoading.value = true
    lastError.value = null

    try {
      const container = await invoke('start_container', {
        imageName: environment.image,
        environmentId: environment.id
      }).catch(() => null)

      if (container) {
        const instance = container as ContainerInstance
        runningContainers.value.push(instance)
        currentEnvironment.value = environment
        saveContainerState()
        return instance
      }

      // Fallback: 创建本地实例
      const fallback: ContainerInstance = {
        id: generateId(),
        imageId: environment.id,
        imageName: environment.image,
        status: 'running',
        createdAt: new Date().toISOString()
      }
      runningContainers.value.push(fallback)
      currentEnvironment.value = environment
      saveContainerState()
      return fallback
    } catch (e: any) {
      lastError.value = e.message || '启动容器失败'
      return null
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 停止容器
   */
  async function stopContainer(containerId: string): Promise<boolean> {
    try {
      await invoke('stop_container', { containerId }).catch(() => null)

      const container = runningContainers.value.find(c => c.id === containerId)
      if (container) {
        container.status = 'stopped'
      }
      saveContainerState()
      return true
    } catch (e: any) {
      lastError.value = e.message || '停止容器失败'
      return false
    }
  }

  /**
   * 删除容器
   */
  async function removeContainer(containerId: string): Promise<boolean> {
    try {
      await invoke('remove_container', { containerId }).catch(() => null)

      runningContainers.value = runningContainers.value.filter(c => c.id !== containerId)
      if (currentEnvironment.value && runningContainers.value.length === 0) {
        currentEnvironment.value = null
      }
      saveContainerState()
      return true
    } catch (e: any) {
      lastError.value = e.message || '删除容器失败'
      return false
    }
  }

  // ============ 代码执行 ============

  /**
   * 在容器中执行代码
   */
  async function executeCode(request: ExecutionRequest): Promise<ExecutionResult> {
    const startTime = Date.now()
    isExecuting.value = true
    lastError.value = null

    // 确保有可用的执行环境
    if (runningContainers.value.length === 0 && !currentEnvironment.value) {
      // 自动启动默认环境
      const env = PREDEFINED_ENVIRONMENTS.find(e => e.id === 'cae-education') || PREDEFINED_ENVIRONMENTS[0]
      const container = await startContainer(env)
      if (!container) {
        return {
          success: false,
          stdout: '',
          stderr: '无法启动执行环境',
          exitCode: -1,
          duration: 0
        }
      }
    }

    try {
      // 通过后端执行代码
      const result = await invoke('execute_in_container', {
        code: request.code,
        language: request.language,
        timeout: request.timeout || 60,
        environmentId: request.environmentId || currentEnvironment.value?.id
      }).catch(() => null)

      const executionResult = result as ExecutionResult || {
        success: false,
        stdout: '',
        stderr: '执行环境不可用，请检查 Docker 是否运行',
        exitCode: -1,
        duration: Date.now() - startTime
      }

      // 记录执行历史
      executionHistory.value.unshift({
        id: generateId(),
        code: request.code,
        language: request.language,
        result: executionResult,
        timestamp: new Date().toISOString()
      })

      // 限制历史记录数量
      if (executionHistory.value.length > 50) {
        executionHistory.value.splice(50)
      }
      saveExecutionHistory()

      return executionResult
    } catch (e: any) {
      lastError.value = e.message || '执行失败'
      return {
        success: false,
        stdout: '',
        stderr: e.message || '执行失败',
        exitCode: -1,
        duration: Date.now() - startTime
      }
    } finally {
      isExecuting.value = false
    }
  }

  /**
   * 执行 Python 代码（快捷方法）
   */
  async function executePython(code: string, timeout?: number): Promise<ExecutionResult> {
    return executeCode({
      code,
      language: 'python',
      timeout
    })
  }

  /**
   * 执行 Julia 代码（快捷方法）
   */
  async function executeJulia(code: string, timeout?: number): Promise<ExecutionResult> {
    return executeCode({
      code,
      language: 'julia',
      timeout
    })
  }

  // ============ 环境管理 ============

  /**
   * 获取所有预定义环境
   */
  function getEnvironments(): ExecutionEnvironment[] {
    return PREDEFINED_ENVIRONMENTS
  }

  /**
   * 获取当前环境信息
   */
  function getCurrentEnvironment(): ExecutionEnvironment | null {
    return currentEnvironment.value
  }

  /**
   * 获取运行中的容器列表
   */
  function getRunningContainers(): ContainerInstance[] {
    return runningContainers.value.filter(c => c.status === 'running')
  }

  // ============ 状态持久化 ============

  function saveContainerState() {
    const state = {
      containers: runningContainers.value,
      currentEnvironment: currentEnvironment.value
    }
    setStorage('caelab_container_state', state)
  }

  function loadContainerState() {
    const state = getStorage<{
      containers: ContainerInstance[]
      currentEnvironment: ExecutionEnvironment | null
    }>('caelab_container_state')

    if (state) {
      runningContainers.value = state.containers || []
      currentEnvironment.value = state.currentEnvironment
    }
  }

  function saveExecutionHistory() {
    setStorage('caelab_execution_history', executionHistory.value)
  }

  function loadExecutionHistory() {
    const history = getStorage<typeof executionHistory.value>('caelab_execution_history')
    if (history) {
      executionHistory.value = history
    }
  }

  // ============ 清理 ============

  /**
   * 清理所有已停止的容器
   */
  async function cleanupStoppedContainers(): Promise<number> {
    const stopped = runningContainers.value.filter(c => c.status === 'stopped')
    let cleaned = 0

    for (const container of stopped) {
      const ok = await removeContainer(container.id)
      if (ok) cleaned++
    }

    return cleaned
  }

  /**
   * 清理执行历史
   */
  function clearExecutionHistory(): void {
    executionHistory.value = []
    saveExecutionHistory()
  }

  // ============ 初始化 ============

  async function initialize(): Promise<void> {
    await checkDockerAvailability()
    loadContainerState()
    loadExecutionHistory()
    if (isDockerAvailable.value) {
      await listImages()
    }
  }

  // 初始化
  initialize()

  return {
    // 状态
    availableImages,
    runningContainers,
    currentEnvironment,
    isDockerAvailable,
    isLoading,
    lastError,
    isExecuting,
    executionHistory,

    // 方法
    checkDockerAvailability,
    listImages,
    pullImage,
    startContainer,
    stopContainer,
    removeContainer,
    executeCode,
    executePython,
    executeJulia,
    getEnvironments,
    getCurrentEnvironment,
    getRunningContainers,
    cleanupStoppedContainers,
    clearExecutionHistory,

    // 工具
    formatBytes,
    PREDEFINED_ENVIRONMENTS
  }
}

// ============ 快捷环境选择器数据 ============

export const ENVIRONMENT_ICONS: Record<string, string> = {
  'cae-base': '📦',
  'cae-advanced': '🔬',
  'cae-multiscale': '🔬',
  'cae-education': '🎓'
}

export function getEnvironmentIcon(envId: string): string {
  return ENVIRONMENT_ICONS[envId] || '📦'
}