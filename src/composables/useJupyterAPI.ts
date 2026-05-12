/**
 * useJupyterAPI.ts — V3.2-005 Jupyter API 增强
 * notebook 直接调用 CAELab API 执行仿真
 */

import { ref } from 'vue'

// ============ 类型定义 ============

export interface CAELabAPIResponse {
  success: boolean
  data?: any
  error?: string
  executionTime: number
}

export interface SimulationJob {
  id: string
  projectId: string
  simulationType: string
  status: 'queued' | 'running' | 'completed' | 'failed'
  submittedAt: string
  completedAt?: string
  result?: SimulationResult
}

export interface SimulationResult {
  maxDisplacement?: number
  maxStress?: number
  maxStrain?: number
  firstFrequency?: number
  meshInfo?: {
    nodes: number
    elements: number
  }
  outputFiles?: string[]
}

export interface JupyterIntegrationConfig {
  serverUrl: string
  apiToken?: string
  autoConnect: boolean
  showInlinePlots: boolean
}

// ============ CAELab API 封装 ============

/**
 * CAELab API 客户端类（供 Jupyter 调用）
 */
export class CAELabAPIClient {
  private baseUrl: string
  private apiToken?: string

  constructor(config: { baseUrl: string; apiToken?: string }) {
    this.baseUrl = config.baseUrl
    this.apiToken = config.apiToken
  }

  private async request(endpoint: string, options?: RequestInit): Promise<any> {
    const url = `${this.baseUrl}${endpoint}`
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      ...(this.apiToken ? { 'Authorization': `Bearer ${this.apiToken}` } : {})
    }

    const response = await fetch(url, { ...options, headers })
    if (!response.ok) {
      throw new Error(`API Error: ${response.status} ${response.statusText}`)
    }
    return response.json()
  }

  // ============ 项目管理 ============

  async listProjects(): Promise<CAELabAPIResponse> {
    const start = Date.now()
    try {
      const data = await this.request('/api/v1/projects')
      return { success: true, data, executionTime: Date.now() - start }
    } catch (e: any) {
      return { success: false, error: e.message, executionTime: Date.now() - start }
    }
  }

  async getProject(projectId: string): Promise<CAELabAPIResponse> {
    const start = Date.now()
    try {
      const data = await this.request(`/api/v1/projects/${projectId}`)
      return { success: true, data, executionTime: Date.now() - start }
    } catch (e: any) {
      return { success: false, error: e.message, executionTime: Date.now() - start }
    }
  }

  async createProject(name: string, description?: string): Promise<CAELabAPIResponse> {
    const start = Date.now()
    try {
      const data = await this.request('/api/v1/projects', {
        method: 'POST',
        body: JSON.stringify({ name, description })
      })
      return { success: true, data, executionTime: Date.now() - start }
    } catch (e: any) {
      return { success: false, error: e.message, executionTime: Date.now() - start }
    }
  }

  // ============ 仿真执行 ============

  async runSimulation(
    projectId: string,
    simulationType: 'static' | 'modal' | 'thermal' | 'buckling' | 'transient',
    options?: {
      meshSize?: 'coarse' | 'medium' | 'fine'
      boundaryConditions?: any[]
    }
  ): Promise<CAELabAPIResponse> {
    const start = Date.now()
    try {
      const data = await this.request('/api/v1/simulations', {
        method: 'POST',
        body: JSON.stringify({
          project_id: projectId,
          analysis_type: simulationType,
          mesh_size: options?.meshSize || 'medium',
          boundary_conditions: options?.boundaryConditions || []
        })
      })
      return { success: true, data, executionTime: Date.now() - start }
    } catch (e: any) {
      return { success: false, error: e.message, executionTime: Date.now() - start }
    }
  }

  async getSimulationStatus(jobId: string): Promise<CAELabAPIResponse> {
    const start = Date.now()
    try {
      const data = await this.request(`/api/v1/simulations/${jobId}`)
      return { success: true, data, executionTime: Date.now() - start }
    } catch (e: any) {
      return { success: false, error: e.message, executionTime: Date.now() - start }
    }
  }

  async getResults(simulationId: string): Promise<CAELabAPIResponse> {
    const start = Date.now()
    try {
      const [disp, stress] = await Promise.all([
        this.request(`/api/v1/results/${simulationId}/displacement`),
        this.request(`/api/v1/results/${simulationId}/stress`)
      ])
      return {
        success: true,
        data: { displacement: disp, stress },
        executionTime: Date.now() - start
      }
    } catch (e: any) {
      return { success: false, error: e.message, executionTime: Date.now() - start }
    }
  }

  // ============ 网格操作 ============

  async generateMesh(
    geometryId: string,
    elementType: string,
    meshSize: number
  ): Promise<CAELabAPIResponse> {
    const start = Date.now()
    try {
      const data = await this.request('/api/v1/mesh/generate', {
        method: 'POST',
        body: JSON.stringify({
          geometry_id: geometryId,
          element_type: elementType,
          mesh_size: meshSize
        })
      })
      return { success: true, data, executionTime: Date.now() - start }
    } catch (e: any) {
      return { success: false, error: e.message, executionTime: Date.now() - start }
    }
  }

  async checkMeshQuality(meshId: string): Promise<CAELabAPIResponse> {
    const start = Date.now()
    try {
      const data = await this.request(`/api/v1/mesh/${meshId}/quality`)
      return { success: true, data, executionTime: Date.now() - start }
    } catch (e: any) {
      return { success: false, error: e.message, executionTime: Date.now() - start }
    }
  }

  // ============ 模板操作 ============

  async listTemplates(category?: string): Promise<CAELabAPIResponse> {
    const start = Date.now()
    try {
      const endpoint = category ? `/api/v1/templates?category=${category}` : '/api/v1/templates'
      const data = await this.request(endpoint)
      return { success: true, data, executionTime: Date.now() - start }
    } catch (e: any) {
      return { success: false, error: e.message, executionTime: Date.now() - start }
    }
  }

  async cloneTemplate(templateId: string, newName?: string): Promise<CAELabAPIResponse> {
    const start = Date.now()
    try {
      const data = await this.request(`/api/v1/templates/${templateId}/clone`, {
        method: 'POST',
        body: JSON.stringify({ name: newName })
      })
      return { success: true, data, executionTime: Date.now() - start }
    } catch (e: any) {
      return { success: false, error: e.message, executionTime: Date.now() - start }
    }
  }

  // ============ 多尺度操作 ============

  async runMultiscale(
    projectId: string,
    scales: string[],
    couplingMethod: string
  ): Promise<CAELabAPIResponse> {
    const start = Date.now()
    try {
      const data = await this.request('/api/v1/multiscale', {
        method: 'POST',
        body: JSON.stringify({
          project_id: projectId,
          scales,
          coupling_method: couplingMethod
        })
      })
      return { success: true, data, executionTime: Date.now() - start }
    } catch (e: any) {
      return { success: false, error: e.message, executionTime: Date.now() - start }
    }
  }
}

// ============ Jupyter 宏命令生成 ============

export function generateJupyterMacros(): string {
  return `
# CAELab Jupyter Integration Macros
# 在 Jupyter Notebook 中使用以下命令调用 CAELab API

# 初始化 CAELab 客户端
caelab = CAELabAPIClient('http://localhost:3000')

# 列出所有项目
projects = caelab.list_projects()

# 运行静态仿真
job = caelab.run_simulation(project_id='your_project_id', simulation_type='static')

# 获取仿真状态
status = caelab.get_simulation_status(job_id=job['job_id'])

# 获取结果
results = caelab.get_results(simulation_id='your_simulation_id')

# 生成网格
mesh = caelab.generate_mesh(geometry_id='your_geometry_id', element_type='tet4', mesh_size=0.01)

# 检查网格质量
quality = caelab.check_mesh_quality(mesh_id=mesh['mesh_id'])

# 列出模板
templates = caelab.list_templates(category='structure')

# 克隆模板
new_project = caelab.clone_template(template_id='your_template_id', new_name='My Project')

# 运行多尺度仿真
ms_job = caelab.run_multiscale(project_id='your_project_id', scales=['md', 'fe'], coupling_method='hierarchical')
`
}

// ============ 主 Composable ============

export function useJupyterAPI() {
  const isConnected = ref(false)
  const isConnecting = ref(false)
  const config = ref<JupyterIntegrationConfig>({
    serverUrl: 'http://localhost:3000',
    apiToken: undefined,
    autoConnect: false,
    showInlinePlots: true
  })

  const client = ref<CAELabAPIClient | null>(null)
  const connectionError = ref<string | null>(null)

  // ============ 连接管理 ============

  async function connect(url?: string, token?: string): Promise<boolean> {
    isConnecting.value = true
    connectionError.value = null

    try {
      const serverUrl = url || config.value.serverUrl

      client.value = new CAELabAPIClient({
        baseUrl: serverUrl,
        apiToken: token || config.value.apiToken
      })

      // 测试连接
      const result = await client.value.listProjects()

      if (result.success) {
        config.value.serverUrl = serverUrl
        if (token) config.value.apiToken = token
        isConnected.value = true
        return true
      } else {
        connectionError.value = result.error || 'Connection failed'
        client.value = null
        return false
      }
    } catch (e: any) {
      connectionError.value = e.message || 'Failed to connect'
      client.value = null
      return false
    } finally {
      isConnecting.value = false
    }
  }

  function disconnect(): void {
    client.value = null
    isConnected.value = false
    connectionError.value = null
  }

  // ============ 便捷方法 ============

  async function runStaticSimulation(projectId: string, options?: any): Promise<CAELabAPIResponse> {
    if (!client.value) {
      return { success: false, error: 'Not connected', executionTime: 0 }
    }
    return client.value.runSimulation(projectId, 'static', options)
  }

  async function runModalAnalysis(projectId: string, options?: any): Promise<CAELabAPIResponse> {
    if (!client.value) {
      return { success: false, error: 'Not connected', executionTime: 0 }
    }
    return client.value.runSimulation(projectId, 'modal', options)
  }

  async function runThermalAnalysis(projectId: string, options?: any): Promise<CAELabAPIResponse> {
    if (!client.value) {
      return { success: false, error: 'Not connected', executionTime: 0 }
    }
    return client.value.runSimulation(projectId, 'thermal', options)
  }

  async function getResults(simulationId: string): Promise<CAELabAPIResponse> {
    if (!client.value) {
      return { success: false, error: 'Not connected', executionTime: 0 }
    }
    return client.value.getResults(simulationId)
  }

  async function listProjects(): Promise<CAELabAPIResponse> {
    if (!client.value) {
      return { success: false, error: 'Not connected', executionTime: 0 }
    }
    return client.value.listProjects()
  }

  // ============ 状态轮询 ============

  function pollSimulationStatus(
    jobId: string,
    interval: number = 2000,
    maxAttempts: number = 30
  ): Promise<CAELabAPIResponse> {
    return new Promise(async (resolve) => {
      let attempts = 0

      const poll = async () => {
        if (!client.value) {
          resolve({ success: false, error: 'Disconnected', executionTime: 0 })
          return
        }

        const result = await client.value.getSimulationStatus(jobId)

        if (!result.success) {
          resolve(result)
          return
        }

        const status = result.data?.status
        if (status === 'completed' || status === 'failed' || attempts >= maxAttempts) {
          resolve(result)
          return
        }

        attempts++
        setTimeout(poll, interval)
      }

      await poll()
    })
  }

  // ============ 宏命令导出 ============

  function getMacros(): string {
    return generateJupyterMacros()
  }

  // ============ 配置保存 ============

  function saveConfig(): void {
    localStorage.setItem('caelab_jupyter_config', JSON.stringify(config.value))
  }

  function loadConfig(): void {
    const stored = localStorage.getItem('caelab_jupyter_config')
    if (stored) {
      try {
        const parsed = JSON.parse(stored)
        config.value = { ...config.value, ...parsed }
      } catch {}
    }
  }

  // ============ 初始化 ============

  loadConfig()

  return {
    // 状态
    isConnected,
    isConnecting,
    config,
    connectionError,

    // 连接
    connect,
    disconnect,

    // 便捷方法
    runStaticSimulation,
    runModalAnalysis,
    runThermalAnalysis,
    getResults,
    listProjects,

    // 轮询
    pollSimulationStatus,

    // 宏命令
    getMacros,

    // 客户端访问
    getClient: () => client.value
  }
}