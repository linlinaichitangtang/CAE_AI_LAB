/**
 * useAPIDocumentation.ts — V3.1-003 REST API 文档
 * 公开 API，支撑 CI/CD 集成
 */

import { ref } from 'vue'

// ============ 类型定义 ============

export interface APIEndpoint {
  id: string
  method: 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE'
  path: string
  description: string
  category: string
  requiresAuth: boolean
  parameters?: APIParameter[]
  requestBody?: APIRequestBody
  responses: APIResponse[]
  example?: {
    request?: string
    response?: string
  }
  tags: string[]
  deprecated?: boolean
}

export interface APIParameter {
  name: string
  type: string
  required: boolean
  description: string
  location: 'path' | 'query' | 'header'
  default?: string
  enum?: string[]
}

export interface APIRequestBody {
  contentType: string
  schema: Record<string, any>
  example?: any
  required?: boolean
}

export interface APIResponse {
  statusCode: number
  description: string
  contentType?: string
  schema?: Record<string, any>
  example?: any
}

// ============ API 端点定义 ============

export const API_ENDPOINTS: APIEndpoint[] = [
  // ============ 项目管理 ============
  {
    id: 'projects-list',
    method: 'GET',
    path: '/api/v1/projects',
    description: '获取当前用户的所有项目列表',
    category: 'Projects',
    requiresAuth: true,
    parameters: [
      { name: 'page', type: 'integer', required: false, description: '页码 (默认: 1)', location: 'query', default: '1' },
      { name: 'limit', type: 'integer', required: false, description: '每页数量 (默认: 20)', location: 'query', default: '20' },
      { name: 'sort', type: 'string', required: false, description: '排序字段 (created_at, updated_at, name)', location: 'query', default: 'updated_at' }
    ],
    responses: [
      { statusCode: 200, description: '成功返回项目列表', contentType: 'application/json', example: { data: [{ id: 'proj_xxx', name: 'My Project', created_at: '2026-01-01' }], total: 100, page: 1 } },
      { statusCode: 401, description: '未授权' }
    ],
    tags: ['Projects', 'CRUD']
  },
  {
    id: 'projects-create',
    method: 'POST',
    path: '/api/v1/projects',
    description: '创建新项目',
    category: 'Projects',
    requiresAuth: true,
    requestBody: {
      contentType: 'application/json',
      required: true,
      schema: { name: 'string', description: 'string', geometry_type: 'string' },
      example: { name: 'Cantilever Beam Analysis', description: 'Static structural analysis', geometry_type: '3d' }
    },
    responses: [
      { statusCode: 201, description: '项目创建成功', example: { id: 'proj_xxx', name: 'Cantilever Beam Analysis' } },
      { statusCode: 400, description: '请求参数错误' },
      { statusCode: 401, description: '未授权' }
    ],
    tags: ['Projects', 'CRUD']
  },
  {
    id: 'projects-get',
    method: 'GET',
    path: '/api/v1/projects/{projectId}',
    description: '获取单个项目详情',
    category: 'Projects',
    requiresAuth: true,
    parameters: [
      { name: 'projectId', type: 'string', required: true, description: '项目 ID', location: 'path' }
    ],
    responses: [
      { statusCode: 200, description: '成功返回项目详情' },
      { statusCode: 404, description: '项目不存在' }
    ],
    tags: ['Projects', 'CRUD']
  },
  {
    id: 'projects-update',
    method: 'PUT',
    path: '/api/v1/projects/{projectId}',
    description: '更新项目信息',
    category: 'Projects',
    requiresAuth: true,
    parameters: [
      { name: 'projectId', type: 'string', required: true, description: '项目 ID', location: 'path' }
    ],
    requestBody: {
      contentType: 'application/json',
      required: true,
      schema: { name: 'string', description: 'string' }
    },
    responses: [
      { statusCode: 200, description: '更新成功' },
      { statusCode: 404, description: '项目不存在' }
    ],
    tags: ['Projects', 'CRUD']
  },
  {
    id: 'projects-delete',
    method: 'DELETE',
    path: '/api/v1/projects/{projectId}',
    description: '删除项目',
    category: 'Projects',
    requiresAuth: true,
    parameters: [
      { name: 'projectId', type: 'string', required: true, description: '项目 ID', location: 'path' }
    ],
    responses: [
      { statusCode: 204, description: '删除成功' },
      { statusCode: 404, description: '项目不存在' }
    ],
    tags: ['Projects', 'CRUD']
  },

  // ============ 仿真分析 ============
  {
    id: 'simulations-run',
    method: 'POST',
    path: '/api/v1/simulations',
    description: '提交仿真分析任务',
    category: 'Simulations',
    requiresAuth: true,
    requestBody: {
      contentType: 'application/json',
      required: true,
      schema: {
        project_id: 'string',
        analysis_type: 'string',
        mesh_size: 'string',
        boundary_conditions: 'array'
      },
      example: {
        project_id: 'proj_xxx',
        analysis_type: 'static',
        mesh_size: 'medium',
        boundary_conditions: [
          { type: 'fixed', location: [0, 0, 0] },
          { type: 'force', location: [1, 0, 0], magnitude: 1000 }
        ]
      }
    },
    responses: [
      { statusCode: 202, description: '任务已提交', example: { job_id: 'job_xxx', status: 'queued' } },
      { statusCode: 400, description: '参数错误' }
    ],
    tags: ['Simulations', 'Analysis']
  },
  {
    id: 'simulations-status',
    method: 'GET',
    path: '/api/v1/simulations/{jobId}',
    description: '查询仿真任务状态',
    category: 'Simulations',
    requiresAuth: true,
    parameters: [
      { name: 'jobId', type: 'string', required: true, description: '任务 ID', location: 'path' }
    ],
    responses: [
      { statusCode: 200, description: '返回任务状态和结果', example: { job_id: 'job_xxx', status: 'completed', result: { max_displacement: 0.023, max_stress: 75.2 } } },
      { statusCode: 404, description: '任务不存在' }
    ],
    tags: ['Simulations', 'Analysis']
  },
  {
    id: 'simulations-cancel',
    method: 'DELETE',
    path: '/api/v1/simulations/{jobId}',
    description: '取消运行中的仿真任务',
    category: 'Simulations',
    requiresAuth: true,
    parameters: [
      { name: 'jobId', type: 'string', required: true, description: '任务 ID', location: 'path' }
    ],
    responses: [
      { statusCode: 204, description: '取消成功' },
      { statusCode: 400, description: '任务已完成，无法取消' }
    ],
    tags: ['Simulations', 'Analysis']
  },

  // ============ 网格处理 ============
  {
    id: 'mesh-generate',
    method: 'POST',
    path: '/api/v1/mesh/generate',
    description: '生成有限元网格',
    category: 'Mesh',
    requiresAuth: true,
    requestBody: {
      contentType: 'application/json',
      required: true,
      schema: {
        geometry_id: 'string',
        element_type: 'string',
        mesh_size: 'number'
      },
      example: { geometry_id: 'geom_xxx', element_type: 'tet4', mesh_size: 0.01 }
    },
    responses: [
      { statusCode: 201, description: '网格生成成功', example: { mesh_id: 'mesh_xxx', nodes: 5000, elements: 3200 } }
    ],
    tags: ['Mesh', 'Preprocessing']
  },
  {
    id: 'mesh-quality',
    method: 'GET',
    path: '/api/v1/mesh/{meshId}/quality',
    description: '检查网格质量',
    category: 'Mesh',
    requiresAuth: true,
    parameters: [
      { name: 'meshId', type: 'string', required: true, description: '网格 ID', location: 'path' }
    ],
    responses: [
      { statusCode: 200, description: '返回网格质量报告', example: { aspect_ratio: 1.5, jacobian: 0.95, quality_score: 92 } }
    ],
    tags: ['Mesh', 'Preprocessing']
  },

  // ============ 结果后处理 ============
  {
    id: 'results-displacement',
    method: 'GET',
    path: '/api/v1/results/{simulationId}/displacement',
    description: '获取位移场结果',
    category: 'Results',
    requiresAuth: true,
    parameters: [
      { name: 'simulationId', type: 'string', required: true, description: '仿真 ID', location: 'path' }
    ],
    responses: [
      { statusCode: 200, description: '返回位移场数据', example: { max: 0.023, min: 0, field: [{ x: 0, y: 0, z: 0, displacement: 0 }] } }
    ],
    tags: ['Results', 'Postprocessing']
  },
  {
    id: 'results-stress',
    method: 'GET',
    path: '/api/v1/results/{simulationId}/stress',
    description: '获取应力分布结果',
    category: 'Results',
    requiresAuth: true,
    parameters: [
      { name: 'simulationId', type: 'string', required: true, description: '仿真 ID', location: 'path' },
      { name: 'component', type: 'string', required: false, description: '应力分量 (von_mises, max_principal, etc.)', location: 'query', default: 'von_mises' }
    ],
    responses: [
      { statusCode: 200, description: '返回应力数据', example: { max: 75.2, min: 0, unit: 'MPa' } }
    ],
    tags: ['Results', 'Postprocessing']
  },

  // ============ 多尺度分析 ============
  {
    id: 'multiscale-run',
    method: 'POST',
    path: '/api/v1/multiscale',
    description: '运行多尺度仿真工作流',
    category: 'Multiscale',
    requiresAuth: true,
    requestBody: {
      contentType: 'application/json',
      required: true,
      schema: {
        project_id: 'string',
        scales: 'array',
        coupling_method: 'string'
      },
      example: {
        project_id: 'proj_xxx',
        scales: ['md', 'phase_field', 'fe'],
        coupling_method: 'hierarchical'
      }
    },
    responses: [
      { statusCode: 202, description: '多尺度分析已提交' },
      { statusCode: 400, description: '参数错误' }
    ],
    tags: ['Multiscale', 'Advanced']
  },
  {
    id: 'multiscale-status',
    method: 'GET',
    path: '/api/v1/multiscale/{workflowId}',
    description: '查询多尺度工作流状态',
    category: 'Multiscale',
    requiresAuth: true,
    parameters: [
      { name: 'workflowId', type: 'string', required: true, description: '工作流 ID', location: 'path' }
    ],
    responses: [
      { statusCode: 200, description: '返回工作流状态', example: { current_scale: 'md', progress: 45, estimated_time: 120 } }
    ],
    tags: ['Multiscale', 'Advanced']
  },

  // ============ 模板和分享 ============
  {
    id: 'templates-list',
    method: 'GET',
    path: '/api/v1/templates',
    description: '获取公开模板列表',
    category: 'Templates',
    requiresAuth: false,
    parameters: [
      { name: 'category', type: 'string', required: false, description: '分类筛选', location: 'query' },
      { name: 'page', type: 'integer', required: false, description: '页码', location: 'query', default: '1' }
    ],
    responses: [
      { statusCode: 200, description: '返回模板列表', example: { data: [], total: 50 } }
    ],
    tags: ['Templates', 'Sharing']
  },
  {
    id: 'templates-clone',
    method: 'POST',
    path: '/api/v1/templates/{templateId}/clone',
    description: '克隆模板创建新项目',
    category: 'Templates',
    requiresAuth: true,
    parameters: [
      { name: 'templateId', type: 'string', required: true, description: '模板 ID', location: 'path' }
    ],
    requestBody: {
      contentType: 'application/json',
      required: false,
      schema: { name: 'string' },
      example: { name: 'My Cloned Project' }
    },
    responses: [
      { statusCode: 201, description: '克隆成功', example: { project_id: 'proj_xxx' } },
      { statusCode: 404, description: '模板不存在' }
    ],
    tags: ['Templates', 'Sharing']
  },

  // ============ 作业管理 (教育) ============
  {
    id: 'assignments-list',
    method: 'GET',
    path: '/api/v1/assignments',
    description: '获取作业列表 (教师) 或我的作业 (学生)',
    category: 'Education',
    requiresAuth: true,
    parameters: [
      { name: 'role', type: 'string', required: false, description: 'teacher 或 student', location: 'query', default: 'student' },
      { name: 'class_id', type: 'string', required: false, description: '班级 ID', location: 'query' }
    ],
    responses: [
      { statusCode: 200, description: '返回作业列表', example: { data: [] } }
    ],
    tags: ['Education', 'Assignments']
  },
  {
    id: 'assignments-submit',
    method: 'POST',
    path: '/api/v1/assignments/{assignmentId}/submit',
    description: '学生提交作业',
    category: 'Education',
    requiresAuth: true,
    parameters: [
      { name: 'assignmentId', type: 'string', required: true, description: '作业 ID', location: 'path' }
    ],
    requestBody: {
      contentType: 'application/json',
      required: true,
      schema: { project_id: 'string', submission_note: 'string' },
      example: { project_id: 'proj_xxx', submission_note: 'Initial submission' }
    },
    responses: [
      { statusCode: 201, description: '提交成功', example: { submission_id: 'sub_xxx', submitted_at: '2026-01-01T00:00:00Z' } }
    ],
    tags: ['Education', 'Assignments']
  },
  {
    id: 'assignments-grade',
    method: 'POST',
    path: '/api/v1/assignments/{assignmentId}/submissions/{submissionId}/grade',
    description: '教师评分作业',
    category: 'Education',
    requiresAuth: true,
    parameters: [
      { name: 'assignmentId', type: 'string', required: true, description: '作业 ID', location: 'path' },
      { name: 'submissionId', type: 'string', required: true, description: '提交 ID', location: 'path' }
    ],
    requestBody: {
      contentType: 'application/json',
      required: true,
      schema: { score: 'number', feedback: 'string' },
      example: { score: 85, feedback: 'Good analysis, minor mesh issues' }
    },
    responses: [
      { statusCode: 200, description: '评分成功' },
      { statusCode: 403, description: '无权限' }
    ],
    tags: ['Education', 'Assignments']
  }
]

// ============ API 分类 ============

export const API_CATEGORIES = [
  { id: 'Projects', name: '项目管理', icon: '📁', description: '项目的 CRUD 操作' },
  { id: 'Simulations', name: '仿真分析', icon: '🔬', description: '提交和管理仿真任务' },
  { id: 'Mesh', name: '网格处理', icon: '🕸️', description: '网格生成和质量检查' },
  { id: 'Results', name: '结果后处理', icon: '📊', description: '获取和分析仿真结果' },
  { id: 'Multiscale', name: '多尺度分析', icon: '🔬', description: '多尺度仿真工作流' },
  { id: 'Templates', name: '模板管理', icon: '📋', description: '模板的浏览和克隆' },
  { id: 'Education', name: '教育功能', icon: '🎓', description: '作业管理和评分' }
]

// ============ 工具函数 ============

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

// ============ 主 Composable ============

export function useAPIDocumentation() {
  const endpoints = ref<APIEndpoint[]>([...API_ENDPOINTS])
  const selectedCategory = ref<string>('all')
  const searchQuery = ref('')

  // 获取端点列表
  function getEndpoints(category?: string, search?: string): APIEndpoint[] {
    let result = [...endpoints.value]

    if (category && category !== 'all') {
      result = result.filter(e => e.category === category)
    }

    if (search) {
      const query = search.toLowerCase()
      result = result.filter(e =>
        e.path.toLowerCase().includes(query) ||
        e.description.toLowerCase().includes(query) ||
        e.tags.some(t => t.toLowerCase().includes(query))
      )
    }

    return result
  }

  // 获取单个端点
  function getEndpoint(endpointId: string): APIEndpoint | undefined {
    return endpoints.value.find(e => e.id === endpointId)
  }

  // 获取分类统计
  function getCategoryStats(): Record<string, number> {
    const stats: Record<string, number> = {}
    for (const endpoint of endpoints.value) {
      stats[endpoint.category] = (stats[endpoint.category] || 0) + 1
    }
    return stats
  }

  // 生成 curl 示例
  function generateCurl(endpoint: APIEndpoint, baseUrl: string = 'https://api.caelab.app'): string {
    let curl = `curl -X ${endpoint.method} '${baseUrl}${endpoint.path}'`

    if (endpoint.requiresAuth) {
      curl += ` \\\n  -H 'Authorization: Bearer YOUR_API_TOKEN'`
    }

    if (endpoint.requestBody) {
      curl += ` \\\n  -H 'Content-Type: ${endpoint.requestBody.contentType}'`
      if (endpoint.requestBody.example) {
        curl += ` \\\n  -d '${JSON.stringify(endpoint.requestBody.example, null, 2)}'`
      }
    }

    return curl
  }

  // 生成 Python 示例
  function generatePython(endpoint: APIEndpoint, baseUrl: string = 'https://api.caelab.app'): string {
    const lines: string[] = []

    lines.push('import requests')
    lines.push('')

    if (endpoint.requestBody?.example) {
      lines.push(`payload = ${JSON.stringify(endpoint.requestBody.example, null, 4)}`)
      lines.push('')
    }

    const headers: string[] = []
    if (endpoint.requiresAuth) {
      headers.push("'Authorization': 'Bearer YOUR_API_TOKEN'")
    }
    if (endpoint.requestBody) {
      headers.push(`'Content-Type': '${endpoint.requestBody.contentType}'`)
    }

    if (headers.length > 0) {
      lines.push(`headers = {${headers.join(', ')}}`)
      lines.push('')
    }

    const methodLower = endpoint.method.toLowerCase()
    const url = `${baseUrl}${endpoint.path}`

    if (endpoint.requestBody?.example) {
      lines.push(`response = requests.${methodLower}(`)
      lines.push(`    '${url}',`)
      lines.push(`    headers=headers,`)
      lines.push(`    json=payload`)
      lines.push(')')
    } else {
      lines.push(`response = requests.${methodLower}('${url}'${endpoint.requiresAuth ? ', headers=headers' : ''})`)
    }

    lines.push('')
    lines.push('print(response.json())')

    return lines.join('\n')
  }

  // 生成 JavaScript 示例
  function generateJavaScript(endpoint: APIEndpoint, baseUrl: string = 'https://api.caelab.app'): string {
    const lines: string[] = []

    lines.push('// Using fetch API')
    lines.push('')

    const options: string[] = []
    options.push(`  method: '${endpoint.method}'`)

    const headers: string[] = []
    if (endpoint.requiresAuth) {
      headers.push("'Authorization': 'Bearer YOUR_API_TOKEN'")
    }
    if (endpoint.requestBody) {
      headers.push(`'Content-Type': '${endpoint.requestBody.contentType}'`)
    }
    if (headers.length > 0) {
      options.push(`  headers: {${headers.join(', ')}}`)
    }

    if (endpoint.requestBody?.example) {
      options.push(`  body: JSON.stringify(${JSON.stringify(endpoint.requestBody.example, null, 2)})`)
    }

    lines.push(`const response = await fetch('${baseUrl}${endpoint.path}', {`)
    lines.push(options.join(',\n'))
    lines.push('})')
    lines.push('')
    lines.push('const data = await response.json()')
    lines.push('console.log(data)')

    return lines.join('\n')
  }

  // 保存 API key (加密存储)
  function saveAPIKey(key: string): void {
    localStorage.setItem('caelab_api_key', key)
  }

  function getAPIKey(): string | null {
    return localStorage.getItem('caelab_api_key')
  }

  function clearAPIKey(): void {
    localStorage.removeItem('caelab_api_key')
  }

  // 获取 API 使用统计
  function getUsageStats(): { requests: number; errors: number; lastUsed: string | null } {
    const stats = getStorage<{ requests: number; errors: number; lastUsed: string | null }>('caelab_api_usage') || {
      requests: 0,
      errors: 0,
      lastUsed: null
    }
    return stats
  }

  function incrementUsage(success: boolean): void {
    const stats = getUsageStats()
    stats.requests++
    if (!success) stats.errors++
    stats.lastUsed = new Date().toISOString()
    setStorage('caelab_api_usage', stats)
  }

  return {
    endpoints,
    selectedCategory,
    searchQuery,
    getEndpoints,
    getEndpoint,
    getCategoryStats,
    generateCurl,
    generatePython,
    generateJavaScript,
    saveAPIKey,
    getAPIKey,
    clearAPIKey,
    getUsageStats,
    incrementUsage,
    API_CATEGORIES
  }
}