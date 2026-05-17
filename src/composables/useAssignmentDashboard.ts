/**
 * useAssignmentDashboard.ts — V3.3-001 班级作业热力图
 * 学生整体vs个体对比，直观发现薄弱点
 */

import { ref, computed } from 'vue'

// ============ 类型定义 ============

export interface Student {
  id: string
  name: string
  email: string
  avatarColor: string
  groupId?: string
}

export interface Assignment {
  id: string
  title: string
  description?: string
  dueDate: string
  createdAt: string
  totalPoints: number
  type: 'modeling' | 'mesh' | 'simulation' | 'postprocess' | 'combined'
  milestones?: AssignmentMilestone[]
}

export interface AssignmentMilestone {
  id: string
  name: string
  description: string
  order: number
  points: number
}

export interface Submission {
  id: string
  assignmentId: string
  studentId: string
  submittedAt: string
  score?: number
  status: 'not_started' | 'in_progress' | 'submitted' | 'graded'
  feedback?: string

  // 分阶段提交状态
  milestoneProgress?: Record<string, {
    completed: boolean
    submittedAt?: string
    score?: number
  }>

  // 操作统计
  operations?: OperationRecord[]

  // 时间追踪
  timeSpent?: number  // 秒
  lastActiveAt?: string
}

export interface OperationRecord {
  id: string
  timestamp: string
  type: 'modeling' | 'mesh' | 'simulation' | 'postprocess' | 'comment' | 'save'
  action: string
  details?: string
  duration?: number  // 操作耗时（秒）
}

export interface ClassHeatmapData {
  studentId: string
  studentName: string
  assignmentId: string
  assignmentTitle: string
  completionRate: number  // 0-100
  score?: number
  submittedAt?: string
  overdue: boolean
  weakAreas: string[]  // 薄弱环节
}

export interface WeakAreaReport {
  area: 'modeling' | 'mesh' | 'simulation' | 'postprocess'
  affectedStudents: number
  totalStudents: number
  severity: 'low' | 'medium' | 'high'
  recommendations: string[]
}

export interface DashboardStats {
  totalStudents: number
  totalAssignments: number
  averageCompletion: number
  averageScore: number
  overdueCount: number
  classWeakAreas: WeakAreaReport[]
}

// ============ 存储键 ============

const STUDENTS_KEY = 'caelab_students'
const ASSIGNMENTS_KEY = 'caelab_assignments'
const SUBMISSIONS_KEY = 'caelab_submissions'
const OPERATION_LOGS_KEY = 'caelab_operation_logs'

// ============ 工具函数 ============

function generateId(): string {
  return `assign_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getAvatarColor(name: string): string {
  const colors = ['#3B82F6', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#EC4899', '#06B6D4', '#F97316']
  let hash = 0
  for (let i = 0; i < name.length; i++) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash)
  }
  return colors[Math.abs(hash) % colors.length]
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

function formatDuration(seconds: number): string {
  const hours = Math.floor(seconds / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  if (hours > 0) return `${hours}h ${minutes}m`
  return `${minutes}m`
}

// ============ 主 Composable ============

export function useAssignmentDashboard() {
  const students = ref<Student[]>([])
  const assignments = ref<Assignment[]>([])
  const submissions = ref<Submission[]>([])
  const operationLogs = ref<Record<string, OperationRecord[]>>({})  // key: submissionId

  // ============ 初始化 ============

  function loadData(): void {
    const storedStudents = getStorage<Student[]>(STUDENTS_KEY)
    if (storedStudents) students.value = storedStudents

    const storedAssignments = getStorage<Assignment[]>(ASSIGNMENTS_KEY)
    if (storedAssignments) assignments.value = storedAssignments

    const storedSubmissions = getStorage<Submission[]>(SUBMISSIONS_KEY)
    if (storedSubmissions) submissions.value = storedSubmissions

    const storedLogs = getStorage<Record<string, OperationRecord[]>>(OPERATION_LOGS_KEY)
    if (storedLogs) operationLogs.value = storedLogs
  }

  // ============ 学生管理 ============

  function addStudent(name: string, email: string, groupId?: string): Student {
    const student: Student = {
      id: generateId(),
      name,
      email,
      avatarColor: getAvatarColor(name),
      groupId
    }
    students.value.push(student)
    saveData()
    return student
  }

  function addStudents(studentsData: Array<{ name: string; email: string; groupId?: string }>): Student[] {
    const newStudents = studentsData.map(s => ({
      id: generateId(),
      name: s.name,
      email: s.email,
      avatarColor: getAvatarColor(s.name),
      groupId: s.groupId
    }))
    students.value.push(...newStudents)
    saveData()
    return newStudents
  }

  function removeStudent(studentId: string): boolean {
    const index = students.value.findIndex(s => s.id === studentId)
    if (index === -1) return false
    students.value.splice(index, 1)
    // 删除相关提交
    submissions.value = submissions.value.filter(sub => sub.studentId !== studentId)
    saveData()
    return true
  }

  function getStudent(studentId: string): Student | undefined {
    return students.value.find(s => s.id === studentId)
  }

  // ============ 作业管理 ============

  function createAssignment(
    title: string,
    dueDate: string,
    options?: {
      description?: string
      totalPoints?: number
      type?: Assignment['type']
      milestones?: Omit<AssignmentMilestone, 'id'>[]
    }
  ): Assignment {
    const assignment: Assignment = {
      id: generateId(),
      title,
      description: options?.description,
      dueDate,
      createdAt: new Date().toISOString(),
      totalPoints: options?.totalPoints || 100,
      type: options?.type || 'combined',
      milestones: options?.milestones?.map(m => ({
        ...m,
        id: generateId()
      }))
    }
    assignments.value.push(assignment)
    saveData()
    return assignment
  }

  function updateAssignment(assignmentId: string, updates: Partial<Assignment>): Assignment | null {
    const assignment = assignments.value.find(a => a.id === assignmentId)
    if (!assignment) return null
    Object.assign(assignment, updates)
    saveData()
    return assignment
  }

  function deleteAssignment(assignmentId: string): boolean {
    const index = assignments.value.findIndex(a => a.id === assignmentId)
    if (index === -1) return false
    assignments.value.splice(index, 1)
    submissions.value = submissions.value.filter(s => s.assignmentId !== assignmentId)
    saveData()
    return true
  }

  function getAssignment(assignmentId: string): Assignment | undefined {
    return assignments.value.find(a => a.id === assignmentId)
  }

  // ============ 提交管理 ============

  function createSubmission(studentId: string, assignmentId: string): Submission {
    const submission: Submission = {
      id: generateId(),
      assignmentId,
      studentId,
      submittedAt: new Date().toISOString(),
      status: 'not_started'
    }
    submissions.value.push(submission)
    saveData()
    return submission
  }

  function getSubmission(studentId: string, assignmentId: string): Submission | undefined {
    return submissions.value.find(s => s.studentId === studentId && s.assignmentId === assignmentId)
  }

  function updateSubmission(submissionId: string, updates: Partial<Submission>): Submission | null {
    const submission = submissions.value.find(s => s.id === submissionId)
    if (!submission) return null
    Object.assign(submission, updates)
    saveData()
    return submission
  }

  function submitForGrading(submissionId: string): boolean {
    const submission = submissions.value.find(s => s.id === submissionId)
    if (!submission) return false
    submission.status = 'submitted'
    submission.submittedAt = new Date().toISOString()
    saveData()
    return true
  }

  function gradeSubmission(submissionId: string, score: number, feedback?: string): boolean {
    const submission = submissions.value.find(s => s.id === submissionId)
    if (!submission) return false
    submission.score = score
    submission.status = 'graded'
    submission.feedback = feedback
    saveData()
    return true
  }

  // ============ 分阶段提交 ============

  function updateMilestoneProgress(
    submissionId: string,
    milestoneId: string,
    progress: { completed: boolean; submittedAt?: string; score?: number }
  ): boolean {
    const submission = submissions.value.find(s => s.id === submissionId)
    if (!submission) return false

    if (!submission.milestoneProgress) {
      submission.milestoneProgress = {}
    }
    submission.milestoneProgress[milestoneId] = progress

    if (progress.completed) {
      submission.status = 'in_progress'
    }

    saveData()
    return true
  }

  // ============ 操作记录 ============

  function logOperation(
    submissionId: string,
    operation: Omit<OperationRecord, 'id' | 'timestamp'>
  ): void {
    if (!operationLogs.value[submissionId]) {
      operationLogs.value[submissionId] = []
    }
    operationLogs.value[submissionId].push({
      ...operation,
      id: generateId(),
      timestamp: new Date().toISOString()
    })
    saveData()
  }

  function getOperationLog(submissionId: string): OperationRecord[] {
    return operationLogs.value[submissionId] || []
  }

  function getOperationsByType(submissionId: string, type: OperationRecord['type']): OperationRecord[] {
    return (operationLogs.value[submissionId] || []).filter(op => op.type === type)
  }

  // ============ 热力图数据 ============

  function generateHeatmapData(assignmentId: string): ClassHeatmapData[] {
    const assignment = assignments.value.find(a => a.id === assignmentId)
    if (!assignment) return []

    const data: ClassHeatmapData[] = []
    const now = new Date()
    const dueDate = new Date(assignment.dueDate)

    for (const student of students.value) {
      const submission = getSubmission(student.id, assignmentId)

      let completionRate = 0
      let weakAreas: string[] = []

      if (submission) {
        if (assignment.milestones && assignment.milestones.length > 0) {
          // 分阶段计算完成率
          const milestoneCount = assignment.milestones.length
          const completedCount = Object.values(submission.milestoneProgress || {})
            .filter(m => m.completed).length
          completionRate = Math.round((completedCount / milestoneCount) * 100)

          // 分析薄弱环节
          assignment.milestones.forEach(m => {
            const progress = submission.milestoneProgress?.[m.id]
            if (!progress?.completed) {
              weakAreas.push(m.name)
            }
          })
        } else {
          // 整体作业
          switch (submission.status) {
            case 'not_started': completionRate = 0; break
            case 'in_progress': completionRate = 50; break
            case 'submitted': completionRate = 80; break
            case 'graded': completionRate = 100; break
          }

          // 根据操作记录分析薄弱环节
          const ops = getOperationsByType(submission.id, submission.status === 'not_started' ? 'modeling' : submission.status === 'in_progress' ? 'mesh' : 'simulation')
          const postprocessOps = getOperationsByType(submission.id, 'postprocess')
          if (ops.length === 0) weakAreas.push('操作记录缺失')
          if (postprocessOps.length === 0 && submission.status === 'graded') weakAreas.push('后处理缺失')
        }
      } else {
        completionRate = 0
        weakAreas.push('未提交')
      }

      data.push({
        studentId: student.id,
        studentName: student.name,
        assignmentId,
        assignmentTitle: assignment.title,
        completionRate,
        score: submission?.score,
        submittedAt: submission?.submittedAt,
        overdue: now > dueDate && completionRate < 100,
        weakAreas
      })
    }

    return data.sort((a, b) => a.studentName.localeCompare(b.studentName))
  }

  function generateAllHeatmaps(): Record<string, ClassHeatmapData[]> {
    const result: Record<string, ClassHeatmapData[]> = {}
    for (const assignment of assignments.value) {
      result[assignment.id] = generateHeatmapData(assignment.id)
    }
    return result
  }

  // ============ 薄弱点分析 ============

  function analyzeWeakAreas(assignmentId?: string): WeakAreaReport[] {
    const reports: WeakAreaReport[] = []
    const areas: Array<Assignment['type']> = ['modeling', 'mesh', 'simulation', 'postprocess']

    const relevantAssignments = assignmentId
      ? assignments.value.filter(a => a.id === assignmentId)
      : assignments.value

    for (const area of areas) {
      let affectedStudents = 0
      let totalGraded = 0
      const recommendations: string[] = []

      for (const student of students.value) {
        for (const assignment of relevantAssignments) {
          const submission = getSubmission(student.id, assignment.id)
          if (submission?.status === 'graded') {
            totalGraded++
            const ops = getOperationsByType(submission.id, (area as any))
            if (ops.length === 0) {
              affectedStudents++
            }
          }
        }
      }

      if (totalGraded > 0) {
        const rate = affectedStudents / totalGraded
        let severity: WeakAreaReport['severity'] = 'low'
        if (rate > 0.5) severity = 'high'
        else if (rate > 0.3) severity = 'medium'

        if (severity !== 'low') {
          switch (area) {
            case 'modeling':
              recommendations.push('建议增加几何建模练习')
              recommendations.push('提供建模步骤视频教程')
              break
            case 'mesh':
              recommendations.push('提供网格质量检查清单')
              recommendations.push('增加网格收敛验证练习')
              break
            case 'simulation':
              recommendations.push('增加边界条件设置训练')
              recommendations.push('提供常见错误案例分析')
              break
            case 'postprocess':
              recommendations.push('演示后处理标准流程')
              recommendations.push('提供结果解读指南')
              break
          }
        }

        reports.push({
          area: area as 'simulation' | 'modeling' | 'postprocess' | 'mesh',
          affectedStudents,
          totalStudents: students.value.length,
          severity,
          recommendations
        })
      }
    }

    return reports.sort((a, b) => {
      const order = { high: 0, medium: 1, low: 2 }
      return order[a.severity] - order[b.severity]
    })
  }

  // ============ 统计数据 ============

  const stats = computed<DashboardStats>(() => {
    const now = new Date()
    let totalCompletion = 0
    let totalScore = 0
    let gradedCount = 0
    let overdueCount = 0

    for (const student of students.value) {
      for (const assignment of assignments.value) {
        const submission = getSubmission(student.id, assignment.id)
        const dueDate = new Date(assignment.dueDate)

        if (submission) {
          // 计算完成率
          if (assignment.milestones && assignment.milestones.length > 0) {
            const completedCount = Object.values(submission.milestoneProgress || {})
              .filter(m => m.completed).length
            totalCompletion += (completedCount / assignment.milestones.length) * 100
          } else {
            switch (submission.status) {
              case 'not_started': totalCompletion += 0; break
              case 'in_progress': totalCompletion += 50; break
              case 'submitted': totalCompletion += 80; break
              case 'graded': totalCompletion += 100; break
            }
          }

          // 分数
          if (submission.score !== undefined) {
            totalScore += submission.score
            gradedCount++
          }

          // 逾期
          if (now > dueDate && submission.status !== 'graded') {
            overdueCount++
          }
        } else {
          if (now > dueDate) overdueCount++
        }
      }
    }

    const totalItems = students.value.length * assignments.value.length
    const classWeakAreas = analyzeWeakAreas()

    return {
      totalStudents: students.value.length,
      totalAssignments: assignments.value.length,
      averageCompletion: totalItems > 0 ? Math.round(totalCompletion / totalItems) : 0,
      averageScore: gradedCount > 0 ? Math.round(totalScore / gradedCount) : 0,
      overdueCount,
      classWeakAreas
    }
  })

  // ============ 学生对比视图 ============

  function getStudentComparison(studentId: string): {
    student: Student
    classRank: number
    completionRate: number
    averageScore: number
    weakAreas: string[]
    recentPerformance: Array<{ assignmentTitle: string; score?: number; date: string }>
  } | null {
    const student = getStudent(studentId)
    if (!student) return null

    // 计算班级排名
    const rankings: Array<{ studentId: string; score: number }> = []
    for (const s of students.value) {
      let totalScore = 0
      let count = 0
      for (const a of assignments.value) {
        const sub = getSubmission(s.id, a.id)
        if (sub?.score !== undefined) {
          totalScore += sub.score
          count++
        }
      }
      rankings.push({ studentId: s.id, score: count > 0 ? totalScore / count : 0 })
    }
    rankings.sort((a, b) => b.score - a.score)
    const classRank = rankings.findIndex(r => r.studentId === studentId) + 1

    // 计算完成率
    let completionRate = 0
    let totalScore = 0
    let scoreCount = 0
    const weakAreas: string[] = []
    const recentPerformance: Array<{ assignmentTitle: string; score?: number; date: string }> = []

    for (const assignment of assignments.value) {
      const submission = getSubmission(studentId, assignment.id)

      if (submission) {
        if (assignment.milestones) {
          const completed = Object.values(submission.milestoneProgress || {})
            .filter(m => m.completed).length
          completionRate += (completed / assignment.milestones.length) * 100
        } else {
          switch (submission.status) {
            case 'not_started': completionRate += 0; break
            case 'in_progress': completionRate += 50; break
            case 'submitted': completionRate += 80; break
            case 'graded': completionRate += 100; break
          }
        }

        if (submission.score !== undefined) {
          totalScore += submission.score
          scoreCount++
          recentPerformance.push({
            assignmentTitle: assignment.title,
            score: submission.score,
            date: submission.submittedAt || assignment.dueDate
          })
        }
      }
    }

    const count = assignments.value.length
    completionRate = count > 0 ? Math.round(completionRate / count) : 0
    const averageScore = scoreCount > 0 ? Math.round(totalScore / scoreCount) : 0

    // 识别薄弱点
    const weakAreaTypes = ['modeling', 'mesh', 'simulation', 'postprocess'] as const
    for (const area of weakAreaTypes) {
      let hasOps = false
      for (const assignment of assignments.value) {
        const sub = getSubmission(studentId, assignment.id)
        if (sub) {
          const ops = getOperationsByType(sub.id, area)
          if (ops.length > 0) {
            hasOps = true
            break
          }
        }
      }
      if (!hasOps) {
        weakAreas.push(area)
      }
    }

    return {
      student,
      classRank,
      completionRate,
      averageScore,
      weakAreas,
      recentPerformance: recentPerformance.slice(-5).reverse()
    }
  }

  // ============ 导出报告 ============

  function generateClassReport(assignmentId: string): string {
    const assignment = getAssignment(assignmentId)
    if (!assignment) return ''

    const heatmap = generateHeatmapData(assignmentId)
    const weakAreas = analyzeWeakAreas(assignmentId)

    const lines: string[] = []
    lines.push('=' .repeat(60))
    lines.push('CLASS ASSIGNMENT REPORT')
    lines.push('=' .repeat(60))
    lines.push('')
    lines.push(`Assignment: ${assignment.title}`)
    lines.push(`Due Date: ${new Date(assignment.dueDate).toLocaleDateString()}`)
    lines.push(`Total Points: ${assignment.totalPoints}`)
    lines.push(`Total Students: ${students.value.length}`)
    lines.push('')

    // 完成率统计
    const completed = heatmap.filter(h => h.completionRate === 100).length
    const inProgress = heatmap.filter(h => h.completionRate > 0 && h.completionRate < 100).length
    const notStarted = heatmap.filter(h => h.completionRate === 0).length

    lines.push('COMPLETION STATISTICS:')
    lines.push(`  Completed: ${completed} (${Math.round(completed / heatmap.length * 100)}%)`)
    lines.push(`  In Progress: ${inProgress} (${Math.round(inProgress / heatmap.length * 100)}%)`)
    lines.push(`  Not Started: ${notStarted} (${Math.round(notStarted / heatmap.length * 100)}%)`)
    lines.push('')

    // 薄弱环节
    if (weakAreas.length > 0) {
      lines.push('WEAK AREAS:')
      for (const area of weakAreas) {
        if (area.severity !== 'low') {
          lines.push(`  [${area.severity.toUpperCase()}] ${area.area}: ${area.affectedStudents}/${area.totalStudents} students`)
          area.recommendations.forEach(r => lines.push(`    - ${r}`))
        }
      }
      lines.push('')
    }

    // 学生列表
    lines.push('STUDENT DETAILS:')
    heatmap.forEach(h => {
      const scoreStr = h.score !== undefined ? `Score: ${h.score}/${assignment.totalPoints}` : 'Not graded'
      const statusStr = h.overdue ? ' [OVERDUE]' : ''
      lines.push(`  ${h.studentName}: ${h.completionRate}% ${scoreStr}${statusStr}`)
      if (h.weakAreas.length > 0) {
        lines.push(`    Weak areas: ${h.weakAreas.join(', ')}`)
      }
    })

    return lines.join('\n')
  }

  // ============ 持久化 ============

  function saveData(): void {
    setStorage(STUDENTS_KEY, students.value)
    setStorage(ASSIGNMENTS_KEY, assignments.value)
    setStorage(SUBMISSIONS_KEY, submissions.value)
    setStorage(OPERATION_LOGS_KEY, operationLogs.value)
  }

  function clearAllData(): void {
    students.value = []
    assignments.value = []
    submissions.value = []
    operationLogs.value = {}
    saveData()
  }

  // 初始化
  loadData()

  return {
    // 状态
    students,
    assignments,
    submissions,

    // 学生管理
    addStudent,
    addStudents,
    removeStudent,
    getStudent,

    // 作业管理
    createAssignment,
    updateAssignment,
    deleteAssignment,
    getAssignment,

    // 提交管理
    createSubmission,
    getSubmission,
    updateSubmission,
    submitForGrading,
    gradeSubmission,

    // 分阶段提交
    updateMilestoneProgress,

    // 操作记录
    logOperation,
    getOperationLog,
    getOperationsByType,

    // 热力图
    generateHeatmapData,
    generateAllHeatmaps,

    // 分析
    analyzeWeakAreas,
    getStudentComparison,

    // 统计
    stats,

    // 导出
    generateClassReport,

    // 持久化
    clearAllData
  }
}
