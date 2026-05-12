/**
 * useLMSIntegration.ts — V3.0-002 LMS 集成 (Canvas/Blackboard)
 * 教育平台集成：作业推送、成绩回传
 */

import { ref } from 'vue'

// ============ 类型定义 ============

export type LMSProvider = 'canvas' | 'blackboard' | 'moodle' | 'custom'

export interface LMSConfig {
  provider: LMSProvider
  baseUrl: string
  apiToken: string
  courseId?: string
  enabled: boolean
}

export interface LMSAssignment {
  id: string
  lmsId: string
  title: string
  description: string
  dueDate?: string
  pointsPossible: number
  submittedAt?: string
  score?: number
  status: 'pending' | 'submitted' | 'graded' | 'returned'
}

export interface LMSGrade {
  studentId: string
  assignmentId: string
  grade: number
  comment?: string
  submittedAt: string
}

export interface LMSCourse {
  id: string
  lmsId: string
  name: string
  code: string
  students: number
}

// ============ 常量 ============

const LMS_STORAGE_KEY = 'caelab_lms_config'
const LMS_GRADES_KEY = 'caelab_lms_grades'

// ============ 工具函数 ============

function generateId(): string {
  return `lms_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

// ============ Canvas API 封装 ============

/**
 * Canvas LMS API 调用封装
 */
async function canvasApiCall(config: LMSConfig, endpoint: string, options?: RequestInit): Promise<any> {
  const url = `${config.baseUrl}/api/v1${endpoint}`
  const response = await fetch(url, {
    ...options,
    headers: {
      'Authorization': `Bearer ${config.apiToken}`,
      'Content-Type': 'application/json',
      ...options?.headers
    }
  })

  if (!response.ok) {
    throw new Error(`Canvas API Error: ${response.status} ${response.statusText}`)
  }

  return response.json()
}

// ============ Blackboard API 封装 ============

/**
 * Blackboard LMS API 调用封装
 */
async function blackboardApiCall(config: LMSConfig, endpoint: string, options?: RequestInit): Promise<any> {
  const url = `${config.baseUrl}/api/v1${endpoint}`
  const response = await fetch(url, {
    ...options,
    headers: {
      'Authorization': `Bearer ${config.apiToken}`,
      'Content-Type': 'application/json',
      ...options?.headers
    }
  })

  if (!response.ok) {
    throw new Error(`Blackboard API Error: ${response.status} ${response.statusText}`)
  }

  return response.json()
}

// ============ 主 Composable ============

export function useLMSIntegration() {
  const config = ref<LMSConfig | null>(null)
  const isConnected = ref(false)
  const isSyncing = ref(false)
  const lastSyncTime = ref<string | null>(null)
  const syncError = ref<string | null>(null)

  // 加载保存的配置
  function loadConfig(): LMSConfig | null {
    const saved = getStorage<LMSConfig>(LMS_STORAGE_KEY)
    if (saved) {
      config.value = saved
      isConnected.value = saved.enabled
    }
    return saved
  }

  // 保存配置
  function saveConfig(newConfig: LMSConfig): void {
    config.value = newConfig
    setStorage(LMS_STORAGE_KEY, newConfig)
    isConnected.value = newConfig.enabled
  }

  // 测试连接
  async function testConnection(newConfig: LMSConfig): Promise<{ success: boolean; error?: string; courses?: LMSCourse[] }> {
    try {
      let courses: any[] = []

      if (newConfig.provider === 'canvas') {
        courses = await canvasApiCall(newConfig, '/courses', { method: 'GET' })
      } else if (newConfig.provider === 'blackboard') {
        courses = await blackboardApiCall(newConfig, '/courses', { method: 'GET' })
      }

      // 转换课程格式
      const formattedCourses: LMSCourse[] = courses.slice(0, 20).map((c: any) => ({
        id: generateId(),
        lmsId: c.id?.toString() || c.uuid || '',
        name: c.name || c.course_name || 'Unnamed Course',
        code: c.course_code || c.course_id || '',
        students: c.students?.length || c.enrollments?.length || 0
      }))

      return { success: true, courses: formattedCourses }
    } catch (e: any) {
      return { success: false, error: e.message || '连接失败' }
    }
  }

  // 连接 LMS
  async function connect(newConfig: LMSConfig): Promise<boolean> {
    const result = await testConnection(newConfig)
    if (result.success) {
      saveConfig({ ...newConfig, enabled: true })
      return true
    }
    syncError.value = result.error || '连接失败'
    return false
  }

  // 断开连接
  function disconnect(): void {
    if (config.value) {
      saveConfig({ ...config.value, enabled: false })
    }
    isConnected.value = false
    syncError.value = null
  }

  // 获取课程列表
  async function getCourses(): Promise<LMSCourse[]> {
    if (!config.value?.enabled) return []

    try {
      let courses: any[] = []
      if (config.value.provider === 'canvas') {
        courses = await canvasApiCall(config.value, '/courses?per_page=50')
      } else if (config.value.provider === 'blackboard') {
        courses = await blackboardApiCall(config.value, '/courses')
      }

      return courses.map((c: any) => ({
        id: generateId(),
        lmsId: c.id?.toString() || c.uuid || '',
        name: c.name || c.course_name || 'Unnamed Course',
        code: c.course_code || c.course_id || '',
        students: c.students?.length || 0
      }))
    } catch (e: any) {
      syncError.value = e.message
      return []
    }
  }

  // 获取作业列表
  async function getAssignments(courseId?: string): Promise<LMSAssignment[]> {
    if (!config.value?.enabled) return []

    const targetCourseId = courseId || config.value.courseId
    if (!targetCourseId) return []

    try {
      let assignments: any[] = []
      if (config.value.provider === 'canvas') {
        assignments = await canvasApiCall(config.value, `/courses/${targetCourseId}/assignments?per_page=50`)
      } else if (config.value.provider === 'blackboard') {
        assignments = await blackboardApiCall(config.value, `/courses/${targetCourseId}/assignments`)
      }

      return assignments.map((a: any) => ({
        id: generateId(),
        lmsId: a.id?.toString() || '',
        title: a.name || a.title || 'Unnamed Assignment',
        description: a.description || '',
        dueDate: a.due_at || a.due_date,
        pointsPossible: a.points_possible || 100,
        submittedAt: a.submitted_at,
        score: a.score,
        status: agraded(a)
      }))
    } catch (e: any) {
      syncError.value = e.message
      return []
    }
  }

  // 辅助函数：判断作业状态
  function agraded(a: any): LMSAssignment['status'] {
    if (a.workflow_state === 'graded' || a.status === 'graded') return 'graded'
    if (a.workflow_state === 'submitted' || a.submitted_at) return 'submitted'
    return 'pending'
  }

  // 提交成绩到 LMS
  async function submitGrade(assignmentId: string, studentId: string, grade: number, comment?: string): Promise<boolean> {
    if (!config.value?.enabled) return false

    try {
      const targetCourseId = config.value.courseId
      if (!targetCourseId) return false

      if (config.value.provider === 'canvas') {
        await canvasApiCall(config.value, `/courses/${targetCourseId}/assignments/${assignmentId}/submissions/${studentId}`, {
          method: 'PUT',
          body: JSON.stringify({
            submission: {
              grade,
              comment
            }
          })
        })
      } else if (config.value.provider === 'blackboard') {
        await blackboardApiCall(config.value, `/courses/${targetCourseId}/assignments/${assignmentId}/submissions/${studentId}`, {
          method: 'PATCH',
          body: JSON.stringify({
            score: grade,
            feedback: comment
          })
        })
      }

      // 保存本地记录
      saveGradeRecord({
        studentId,
        assignmentId,
        grade,
        comment,
        submittedAt: new Date().toISOString()
      })

      return true
    } catch (e: any) {
      syncError.value = e.message
      return false
    }
  }

  // 批量提交成绩
  async function batchSubmitGrades(grades: Array<{ assignmentId: string; studentId: string; grade: number; comment?: string }>): Promise<{ success: number; failed: number }> {
    let success = 0
    let failed = 0

    for (const g of grades) {
      try {
        const ok = await submitGrade(g.assignmentId, g.studentId, g.grade, g.comment)
        if (ok) {
          success++
        } else {
          failed++
        }
      } catch {
        failed++
      }
    }

    return { success, failed }
  }

  // 保存成绩记录
  function saveGradeRecord(grade: LMSGrade): void {
    const records = getStorage<LMSGrade[]>(LMS_GRADES_KEY) || []
    records.push(grade)
    // 只保留最近 500 条
    if (records.length > 500) {
      records.splice(0, records.length - 500)
    }
    setStorage(LMS_GRADES_KEY, records)
  }

  // 获取成绩历史
  function getGradeHistory(): LMSGrade[] {
    return getStorage<LMSGrade[]>(LMS_GRADES_KEY) || []
  }

  // 同步作业
  async function syncAssignments(): Promise<LMSAssignment[]> {
    isSyncing.value = true
    syncError.value = null

    try {
      const assignments = await getAssignments()
      lastSyncTime.value = new Date().toISOString()
      return assignments
    } catch (e: any) {
      syncError.value = e.message
      return []
    } finally {
      isSyncing.value = false
    }
  }

  // 获取同步状态
  function getSyncStatus(): { isConnected: boolean; lastSync: string | null; error: string | null } {
    return {
      isConnected: isConnected.value,
      lastSync: lastSyncTime.value,
      error: syncError.value
    }
  }

  // 初始化时加载配置
  loadConfig()

  return {
    config,
    isConnected,
    isSyncing,
    lastSyncTime,
    syncError,
    connect,
    disconnect,
    testConnection,
    getCourses,
    getAssignments,
    submitGrade,
    batchSubmitGrades,
    syncAssignments,
    getGradeHistory,
    getSyncStatus
  }
}

// ============ LMS 配置向导组件的数据 ============

export const LMS_PROVIDERS: Array<{ id: LMSProvider; name: string; icon: string; description: string }> = [
  {
    id: 'canvas',
    name: 'Canvas LMS',
    icon: '🎓',
    description: 'Instructure Canvas 云端版或自托管版'
  },
  {
    id: 'blackboard',
    name: 'Blackboard Learn',
    icon: '📚',
    description: 'Blackboard Learn Ultra 或 Classic'
  },
  {
    id: 'moodle',
    name: 'Moodle',
    icon: '🌐',
    description: 'Moodle 开源版或托管版'
  },
  {
    id: 'custom',
    name: '自定义 API',
    icon: '⚙️',
    description: '支持任何符合 REST API 规范的教育平台'
  }
]

// ============ 作业推送格式转换 ============

/**
 * 将 CAELab 作业转换为 LMS 格式
 */
export function convertAssignmentToLMS(assignment: any, format: LMSProvider): any {
  const base = {
    name: assignment.title,
    description: assignment.description,
    points_possible: assignment.totalPoints || 100,
    due_at: assignment.dueDate
  }

  switch (format) {
    case 'canvas':
      return {
        ...base,
        submission_types: ['online_upload'],
        workflow_state: 'unpublished'
      }
    case 'blackboard':
      return {
        ...base,
        type: 'Assignment',
        content: assignment.description
      }
    default:
      return base
  }
}