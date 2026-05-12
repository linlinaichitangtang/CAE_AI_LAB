/**
 * useMilestoneAssignments.ts — V3.3-003 分阶段提交
 * 布置渐进式作业（建模→网格→求解→后处理），每步检查
 */

import { ref, computed } from 'vue'

// ============ 类型定义 ============

export interface MilestoneTemplate {
  id: string
  name: string
  description: string
  order: number
  expectedDuration?: number  // 预期时长（分钟）
  checkpoints: MilestoneCheckpoint[]
}

export interface MilestoneCheckpoint {
  id: string
  name: string
  description: string
  order: number
  maxScore: number
  autoGradingRules?: AutoGradingRule[]
  requiredFiles?: string[]
  validationScript?: string
}

export interface AutoGradingRule {
  id: string
  type: 'mesh_quality' | 'result_range' | 'file_exists' | 'parameter_match' | 'convergence'
  params: Record<string, any>
  points: number
  errorMessage: string
}

export interface MilestoneAssignment {
  id: string
  title: string
  description?: string
  courseId?: string
  createdAt: string
  dueDates: Record<string, string>  // milestoneId -> due date ISO string
  totalScore: number
  templateId?: string
  milestones: AssignmentMilestone[]
  settings: AssignmentSettings
}

export interface AssignmentMilestone {
  id: string
  templateId?: string
  name: string
  description: string
  order: number
  checkpoints: AssignmentCheckpoint[]
  unlockCondition?: 'previous_complete' | 'previous_passed' | 'manual' | 'immediate'
  passingScore?: number  // 解锁后续 milestone 需要的分数
}

export interface AssignmentCheckpoint {
  id: string
  name: string
  description: string
  order: number
  maxScore: number
  autoGradingRules?: AutoGradingRule[]
  requiredFiles?: string[]
  validationScript?: string
}

export interface MilestoneSubmission {
  id: string
  assignmentId: string
  milestoneId: string
  studentId: string
  checkpointSubmissions: CheckpointSubmission[]
  status: 'locked' | 'in_progress' | 'submitted' | 'graded'
  startedAt?: string
  submittedAt?: string
  totalScore?: number
  autoScore?: number
  manualScore?: number
  feedback?: string
  gradedAt?: string
  gradedBy?: string
}

export interface CheckpointSubmission {
  id: string
  checkpointId: string
  status: 'not_started' | 'in_progress' | 'submitted' | 'passed' | 'failed'
  submittedAt?: string
  score?: number
  autoScore?: number
  manualScore?: number
  validationResults?: ValidationResult[]
  files?: string[]
  notes?: string
}

export interface ValidationResult {
  ruleId: string
  passed: boolean
  actualValue?: any
  expectedValue?: any
  message: string
  pointsAwarded: number
}

export interface AssignmentSettings {
  allowLateSubmission: boolean
  latePenaltyPercent: number  // 0-100
  allowResubmit: boolean
  maxResubmitCount: number
  showCorrectAnswer: boolean
  showHintsAfterDue: boolean
  notifyOnSubmission: boolean
  notifyOnGrading: boolean
}

export interface StudentProgress {
  studentId: string
  assignmentId: string
  milestoneProgress: Record<string, {
    status: 'locked' | 'in_progress' | 'submitted' | 'graded'
    score?: number
    completedCheckpoints: number
    totalCheckpoints: number
  }>
  overallScore: number
  overallProgress: number  // 0-100
  isComplete: boolean
  isLate: boolean
  timeSpent?: number  // 分钟
}

// ============ 预设模板 ============

export const MILESTONE_TEMPLATES: Omit<MilestoneTemplate, 'id'>[] = [
  {
    name: '基础有限元分析',
    description: '四阶段完整仿真流程',
    order: 0,
    expectedDuration: 120,
    checkpoints: [
      { id: 'm1-c1', name: '几何建模', description: '创建几何模型', order: 0, maxScore: 25 },
      { id: 'm1-c2', name: '材料定义', description: '设置材料属性', order: 1, maxScore: 25 }
    ]
  },
  {
    name: '完整仿真流程',
    description: '建模→网格→求解→后处理四阶段',
    order: 1,
    expectedDuration: 180,
    checkpoints: [
      {
        id: 'ms1-c1', name: '几何建模', description: '创建几何体', order: 0, maxScore: 20,
        autoGradingRules: [
          { id: 'vr1', type: 'file_exists', params: { pattern: '*.step' }, points: 10, errorMessage: '几何文件未找到' }
        ]
      },
      {
        id: 'ms1-c2', name: '网格划分', description: '生成网格', order: 1, maxScore: 25,
        autoGradingRules: [
          { id: 'vr2', type: 'mesh_quality', params: { minQuality: 0.3, minElements: 100 }, points: 15, errorMessage: '网格质量不达标' }
        ]
      },
      {
        id: 'ms1-c3', name: '仿真求解', description: '运行仿真', order: 2, maxScore: 30,
        autoGradingRules: [
          { id: 'vr3', type: 'convergence', params: { maxIterations: 500 }, points: 20, errorMessage: '求解未收敛' },
          { id: 'vr4', type: 'result_range', params: { field: 'maxStress', min: 0, max: 1000 }, points: 10, errorMessage: '结果超出合理范围' }
        ]
      },
      {
        id: 'ms1-c4', name: '后处理', description: '结果可视化与报告', order: 3, maxScore: 25,
        requiredFiles: ['results.pdf', 'displacement.csv']
      }
    ]
  },
  {
    name: '高级非线性分析',
    description: '包含非线性材料和接触分析',
    order: 2,
    expectedDuration: 240,
    checkpoints: [
      { id: 'adv1-c1', name: '几何建模', description: '创建复杂几何', order: 0, maxScore: 15 },
      { id: 'adv1-c2', name: '非线性材料', description: '弹塑性材料定义', order: 1, maxScore: 25 },
      { id: 'adv1-c3', name: '接触设置', description: '定义接触对', order: 2, maxScore: 25 },
      { id: 'adv1-c4', name: '非线性求解', description: '非线性求解器配置', order: 3, maxScore: 20 },
      { id: 'adv1-c5', name: '结果分析', description: '后处理与验证', order: 4, maxScore: 15 }
    ]
  }
]

// ============ 存储键 ============

const TEMPLATES_KEY = 'caelab_milestone_templates'
const ASSIGNMENTS_KEY = 'caelab_milestone_assignments'
const SUBMISSIONS_KEY = 'caelab_milestone_submissions'

// ============ 工具函数 ============

function generateId(): string {
  return `milestone_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

// ============ 主 Composable ============

export function useMilestoneAssignments() {
  const templates = ref<MilestoneTemplate[]>([])
  const assignments = ref<MilestoneAssignment[]>([])
  const submissions = ref<MilestoneSubmission[]>([])

  // ============ 初始化 ============

  function loadData(): void {
    const storedTemplates = getStorage<MilestoneTemplate[]>(TEMPLATES_KEY)
    templates.value = storedTemplates || MILESTONE_TEMPLATES.map(t => ({ ...t, id: generateId() }))

    const storedAssignments = getStorage<MilestoneAssignment[]>(ASSIGNMENTS_KEY)
    if (storedAssignments) assignments.value = storedAssignments

    const storedSubmissions = getStorage<MilestoneSubmission[]>(SUBMISSIONS_KEY)
    if (storedSubmissions) submissions.value = storedSubmissions
  }

  // ============ 模板管理 ============

  function createTemplate(
    name: string,
    description: string,
    checkpoints: Omit<MilestoneCheckpoint, 'id'>[]
  ): MilestoneTemplate {
    const template: MilestoneTemplate = {
      id: generateId(),
      name,
      description,
      order: templates.value.length,
      checkpoints: checkpoints.map((c, i) => ({ ...c, id: generateId(), order: i }))
    }
    templates.value.push(template)
    saveData()
    return template
  }

  function createFromTemplate(templateId: string, dueDates: Record<string, string>): MilestoneTemplate | undefined {
    const template = templates.value.find(t => t.id === templateId)
    if (!template) return undefined

    return {
      ...template,
      id: generateId(),
      checkpoints: template.checkpoints.map(c => ({ ...c, id: generateId() }))
    }
  }

  // ============ 作业创建 ============

  function createAssignment(
    title: string,
    milestones: Omit<AssignmentMilestone, 'id'>[],
    options?: {
      description?: string
      courseId?: string
      dueDates?: Record<string, string>
      templateId?: string
      settings?: Partial<AssignmentSettings>
    }
  ): MilestoneAssignment {
    const defaultSettings: AssignmentSettings = {
      allowLateSubmission: true,
      latePenaltyPercent: 10,
      allowResubmit: false,
      maxResubmitCount: 3,
      showCorrectAnswer: false,
      showHintsAfterDue: true,
      notifyOnSubmission: true,
      notifyOnGrading: true
    }

    const assignment: MilestoneAssignment = {
      id: generateId(),
      title,
      description: options?.description,
      courseId: options?.courseId,
      createdAt: new Date().toISOString(),
      dueDates: options?.dueDates || {},
      totalScore: 0,
      templateId: options?.templateId,
      milestones: milestones.map((m, i) => ({
        ...m,
        id: generateId(),
        order: i,
        checkpoints: m.checkpoints.map((c, j) => ({
          ...c,
          id: generateId(),
          order: j
        }))
      })),
      settings: { ...defaultSettings, ...options?.settings }
    }

    // 计算总分
    assignment.totalScore = assignment.milestones.reduce(
      (sum, ms) => sum + ms.checkpoints.reduce((cs, c) => cs + c.maxScore, 0),
      0
    )

    assignments.value.push(assignment)
    saveData()
    return assignment
  }

  /**
   * 从预设模板创建作业
   */
  function createFromPresetTemplate(
    templateId: string,
    title: string,
    dueDates: Record<string, string>
  ): MilestoneAssignment | null {
    const template = templates.value.find(t => t.id === templateId)
    if (!template) return null

    return createAssignment(
      title,
      [{
        name: template.name,
        description: template.description,
        order: 0,
        checkpoints: template.checkpoints.map(c => ({
          name: c.name,
          description: c.description,
          order: c.order,
          maxScore: c.maxScore,
          autoGradingRules: c.autoGradingRules,
          requiredFiles: c.requiredFiles,
          validationScript: c.validationScript
        })),
        unlockCondition: 'immediate' as const
      }],
      { dueDates, templateId }
    )
  }

  function updateAssignment(assignmentId: string, updates: Partial<MilestoneAssignment>): boolean {
    const assignment = assignments.value.find(a => a.id === assignmentId)
    if (!assignment) return false
    Object.assign(assignment, updates)
    saveData()
    return true
  }

  function deleteAssignment(assignmentId: string): boolean {
    const index = assignments.value.findIndex(a => a.id === assignmentId)
    if (index === -1) return false
    assignments.value.splice(index, 1)
    submissions.value = submissions.value.filter(s => s.assignmentId !== assignmentId)
    saveData()
    return true
  }

  // ============ 提交管理 ============

  /**
   * 创建或获取学生提交记录
   */
  function getOrCreateSubmission(
    assignmentId: string,
    studentId: string
  ): MilestoneSubmission | null {
    const assignment = assignments.value.find(a => a.id === assignmentId)
    if (!assignment) return null

    // 查找现有提交
    let submission = submissions.value.find(
      s => s.assignmentId === assignmentId && s.studentId === studentId
    )

    if (!submission) {
      // 创建新提交
      submission = {
        id: generateId(),
        assignmentId,
        milestoneId: '',  // 第一个 milestone
        studentId,
        checkpointSubmissions: [],
        status: 'in_progress',
        startedAt: new Date().toISOString()
      }

      // 为每个 milestone 和 checkpoint 创建初始记录
      for (const milestone of assignment.milestones) {
        for (const checkpoint of milestone.checkpoints) {
          submission.checkpointSubmissions.push({
            id: generateId(),
            checkpointId: checkpoint.id,
            status: milestone.order === 0 ? 'not_started' : 'locked'
          })
        }
      }

      submissions.value.push(submission)
      saveData()
    }

    return submission
  }

  /**
   * 获取学生的作业进度
   */
  function getStudentProgress(assignmentId: string, studentId: string): StudentProgress | null {
    const assignment = assignments.value.find(a => a.id === assignmentId)
    const submission = submissions.value.find(
      s => s.assignmentId === assignmentId && s.studentId === studentId
    )

    if (!assignment) return null

    const milestoneProgress: Record<string, any> = {}
    let completedCheckpoints = 0
    let totalCheckpoints = 0
    let totalScore = 0

    for (const milestone of assignment.milestones) {
      const milestoneSubmissions = submission?.checkpointSubmissions.filter(
        cs => milestone.checkpoints.some(c => c.id === cs.checkpointId)
      ) || []

      const msTotal = milestone.checkpoints.length
      const msCompleted = milestoneSubmissions.filter(cs => cs.status === 'passed' || cs.status === 'submitted').length

      completedCheckpoints += msCompleted
      totalCheckpoints += msTotal

      const msScore = milestoneSubmissions.reduce((sum, cs) => sum + (cs.score || 0), 0)
      totalScore += msScore

      let status: StudentProgress['milestoneProgress'][string]['status'] = 'locked'
      if (submission) {
        if (msCompleted === msTotal) {
          status = 'submitted'
        } else if (msCompleted > 0) {
          status = 'in_progress'
        }
      }

      milestoneProgress[milestone.id] = {
        status,
        score: msScore,
        completedCheckpoints: msCompleted,
        totalCheckpoints: msTotal
      }
    }

    const overallProgress = totalCheckpoints > 0
      ? Math.round((completedCheckpoints / totalCheckpoints) * 100)
      : 0

    // 检查是否逾期
    const now = new Date()
    let isLate = false
    for (const [milestoneId, dueDate] of Object.entries(assignment.dueDates)) {
      if (dueDate && new Date(dueDate) < now) {
        const milestone = assignment.milestones.find(m => m.id === milestoneId)
        if (milestone && milestoneProgress[milestoneId]?.status !== 'submitted') {
          isLate = true
          break
        }
      }
    }

    return {
      studentId,
      assignmentId,
      milestoneProgress,
      overallScore: totalScore,
      overallProgress,
      isComplete: overallProgress === 100,
      isLate
    }
  }

  /**
   * 更新检查点状态
   */
  function updateCheckpoint(
    submissionId: string,
    checkpointId: string,
    updates: Partial<CheckpointSubmission>
  ): boolean {
    const submission = submissions.value.find(s => s.id === submissionId)
    if (!submission) return false

    const checkpoint = submission.checkpointSubmissions.find(cs => cs.checkpointId === checkpointId)
    if (!checkpoint) return false

    Object.assign(checkpoint, updates)
    saveData()
    return true
  }

  /**
   * 提交检查点
   */
  function submitCheckpoint(
    submissionId: string,
    checkpointId: string,
    files?: string[],
    notes?: string
  ): boolean {
    const submission = submissions.value.find(s => s.id === submissionId)
    if (!submission) return false

    const checkpoint = submission.checkpointSubmissions.find(cs => cs.checkpointId === checkpointId)
    if (!checkpoint || checkpoint.status === 'locked') return false

    checkpoint.status = 'submitted'
    checkpoint.submittedAt = new Date().toISOString()
    checkpoint.files = files
    checkpoint.notes = notes

    // 检查是否所有检查点都完成
    const allComplete = submission.checkpointSubmissions.every(
      cs => cs.status === 'submitted' || cs.status === 'passed' || cs.status === 'failed'
    )
    if (allComplete) {
      submission.status = 'submitted'
      submission.submittedAt = new Date().toISOString()
    }

    saveData()
    return true
  }

  /**
   * 自动评分
   */
  function autoGradeCheckpoint(
    submissionId: string,
    checkpointId: string,
    validationResults: ValidationResult[]
  ): boolean {
    const submission = submissions.value.find(s => s.id === submissionId)
    if (!submission) return false

    const checkpoint = submission.checkpointSubmissions.find(cs => cs.checkpointId === checkpointId)
    if (!checkpoint) return false

    checkpoint.validationResults = validationResults
    checkpoint.autoScore = validationResults.reduce((sum, r) => sum + r.pointsAwarded, 0)

    // 判断是否通过
    const allPassed = validationResults.every(r => r.passed)
    checkpoint.status = allPassed ? 'passed' : 'failed'

    saveData()
    return true
  }

  /**
   * 手动评分
   */
  function gradeCheckpoint(
    submissionId: string,
    checkpointId: string,
    score: number,
    feedback?: string,
    gradedBy?: string
  ): boolean {
    const submission = submissions.value.find(s => s.id === submissionId)
    if (!submission) return false

    const checkpoint = submission.checkpointSubmissions.find(cs => cs.checkpointId === checkpointId)
    if (!checkpoint) return false

    checkpoint.manualScore = score
    checkpoint.score = score
    if (feedback) checkpoint.notes = (checkpoint.notes || '') + `\n[Feedback] ${feedback}`

    // 计算总分
    submission.totalScore = submission.checkpointSubmissions.reduce((sum, cs) => sum + (cs.score || 0), 0)

    submission.gradedAt = new Date().toISOString()
    submission.gradedBy = gradedBy

    // 检查是否所有检查点都评分完成
    const allGraded = submission.checkpointSubmissions.every(
      cs => cs.manualScore !== undefined || cs.status === 'failed'
    )
    if (allGraded) {
      submission.status = 'graded'
    }

    saveData()
    return true
  }

  /**
   * 解锁下一阶段
   */
  function unlockNextMilestone(submissionId: string, currentMilestoneId: string): boolean {
    const submission = submissions.value.find(s => s.id === submissionId)
    if (!submission) return false

    const assignment = assignments.value.find(a => a.id === submission.assignmentId)
    if (!assignment) return false

    const currentMilestone = assignment.milestones.find(m => m.id === currentMilestoneId)
    if (!currentMilestone) return false

    // 找到下一个 milestone
    const nextMilestone = assignment.milestones.find(m => m.order === currentMilestone.order + 1)
    if (!nextMilestone) return false

    // 解锁下一个 milestone 的所有检查点
    for (const checkpoint of nextMilestone.checkpoints) {
      const cs = submission.checkpointSubmissions.find(c => c.checkpointId === checkpoint.id)
      if (cs && cs.status === 'locked') {
        cs.status = 'not_started'
      }
    }

    submission.milestoneId = nextMilestone.id
    saveData()
    return true
  }

  // ============ 查询 ============

  function getAssignment(assignmentId: string): MilestoneAssignment | undefined {
    return assignments.value.find(a => a.id === assignmentId)
  }

  function getTemplate(templateId: string): MilestoneTemplate | undefined {
    return templates.value.find(t => t.id === templateId)
  }

  function getAssignmentSubmissions(assignmentId: string): MilestoneSubmission[] {
    return submissions.value.filter(s => s.assignmentId === assignmentId)
  }

  function getStudentSubmissions(studentId: string): MilestoneSubmission[] {
    return submissions.value.filter(s => s.studentId === studentId)
  }

  // ============ 统计 ============

  function getAssignmentStats(assignmentId: string): {
    totalSubmissions: number
    completedSubmissions: number
    averageScore: number
    passRate: number
    averageProgress: number
    lateSubmissions: number
  } {
    const assignment = assignments.value.find(a => a.id === assignmentId)
    if (!assignment) {
      return {
        totalSubmissions: 0,
        completedSubmissions: 0,
        averageScore: 0,
        passRate: 0,
        averageProgress: 0,
        lateSubmissions: 0
      }
    }

    const assignmentSubs = submissions.value.filter(s => s.assignmentId === assignmentId)
    const now = new Date()

    const completed = assignmentSubs.filter(s => s.status === 'graded' || s.status === 'submitted').length
    const graded = assignmentSubs.filter(s => s.status === 'graded')
    const averageScore = graded.length > 0
      ? graded.reduce((sum, s) => sum + (s.totalScore || 0), 0) / graded.length
      : 0
    const passRate = graded.length > 0
      ? (graded.filter(s => (s.totalScore || 0) >= assignment.totalScore * 0.6).length / graded.length) * 100
      : 0

    const lateSubmissions = assignmentSubs.filter(s => {
      if (!s.submittedAt) return false
      const lastDue = Object.values(assignment.dueDates).sort().pop()
      return lastDue && new Date(s.submittedAt) > new Date(lastDue)
    }).length

    // 平均进度
    let totalProgress = 0
    for (const sub of assignmentSubs) {
      const progress = getStudentProgress(assignmentId, sub.studentId)
      if (progress) totalProgress += progress.overallProgress
    }
    const averageProgress = assignmentSubs.length > 0 ? totalProgress / assignmentSubs.length : 0

    return {
      totalSubmissions: assignmentSubs.length,
      completedSubmissions: completed,
      averageScore: Math.round(averageScore),
      passRate: Math.round(passRate),
      averageProgress: Math.round(averageProgress),
      lateSubmissions
    }
  }

  // ============ 导出 ============

  function exportGrades(assignmentId: string): string {
    const assignment = assignments.value.find(a => a.id === assignmentId)
    if (!assignment) return ''

    const lines: string[] = []
    lines.push(`Assignment: ${assignment.title}`)
    lines.push(`Total Score: ${assignment.totalScore}`)
    lines.push(`Due Dates: ${JSON.stringify(assignment.dueDates)}`)
    lines.push('')

    // 表头
    const headers = ['Student ID', 'Milestones Completed', 'Total Score', 'Progress', 'Status', 'Submitted At']
    lines.push(headers.join(','))

    // 数据行
    const subs = getAssignmentSubmissions(assignmentId)
    for (const sub of subs) {
      const progress = getStudentProgress(assignmentId, sub.studentId)
      const completedMs = Object.values(progress?.milestoneProgress || {})
        .filter(m => m.status === 'submitted' || m.status === 'graded').length

      lines.push([
        sub.studentId,
        `${completedMs}/${assignment.milestones.length}`,
        sub.totalScore || 0,
        `${progress?.overallProgress || 0}%`,
        sub.status,
        sub.submittedAt || ''
      ].join(','))
    }

    return lines.join('\n')
  }

  // ============ 持久化 ============

  function saveData(): void {
    setStorage(TEMPLATES_KEY, templates.value)
    setStorage(ASSIGNMENTS_KEY, assignments.value)
    setStorage(SUBMISSIONS_KEY, submissions.value)
  }

  // 初始化
  loadData()

  return {
    // 状态
    templates,
    assignments,
    submissions,

    // 模板管理
    createTemplate,
    createFromTemplate,
    MILESTONE_TEMPLATES,

    // 作业管理
    createAssignment,
    createFromPresetTemplate,
    updateAssignment,
    deleteAssignment,
    getAssignment,

    // 提交管理
    getOrCreateSubmission,
    getStudentProgress,
    updateCheckpoint,
    submitCheckpoint,
    autoGradeCheckpoint,
    gradeCheckpoint,
    unlockNextMilestone,

    // 查询
    getTemplate,
    getAssignmentSubmissions,
    getStudentSubmissions,

    // 统计
    getAssignmentStats,

    // 导出
    exportGrades,

    // 持久化
    saveData
  }
}
