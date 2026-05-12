/**
 * Assignment API - V2.9-008 作业面板
 * 教师布置仿真作业、学生提交、结果对比
 */

// ============ 类型定义 ============

export interface Student {
  id: string
  name: string
  email?: string
  avatarColor: string
}

export interface AssignmentSubmission {
  id: string
  assignmentId: string  // 关联的作业ID
  studentId: string
  studentName: string
  submittedAt: string
  projectId: string
  projectSnapshot?: {
    maxDisplacement?: number
    maxStress?: number
    meshNodes?: number
    meshElements?: number
  }
  feedback?: string
  score?: number
}

export interface Assignment {
  id: string
  title: string
  description: string
  templateId?: string  // 关联的模板ID
  templateSnapshot?: any  // 模板数据快照
  dueDate?: string
  createdAt: string
  updatedAt: string
  createdBy: string  // 教师名称
  classId: string
  submissions: AssignmentSubmission[]
  allowLate: boolean
  maxAttempts: number
}

export interface Class {
  id: string
  name: string
  description?: string
  createdAt: string
  students: Student[]
  assignments: Assignment[]
}

export interface ResultComparison {
  metric: string
  studentValue: number
  templateValue: number
  deviation: number  // 百分比
}

// ============ 存储键 ============
const CLASSES_KEY = 'caelab_classes'
const ASSIGNMENTS_KEY = 'caelab_assignments'
const SUBMISSIONS_KEY = 'caelab_submissions'

// ============ 工具函数 ============

function generateId(): string {
  return `asm_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T[] {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : []
}

function setStorage<T>(key: string, data: T[]): void {
  localStorage.setItem(key, JSON.stringify(data))
}

function getAvatarColor(name: string): string {
  const colors = [
    '#3B82F6', '#10B981', '#F59E0B', '#EF4444',
    '#8B5CF6', '#EC4899', '#06B6D4', '#F97316'
  ]
  let hash = 0
  for (let i = 0; i < name.length; i++) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash)
  }
  return colors[Math.abs(hash) % colors.length]
}

// ============ 班级管理 ============

/**
 * 创建班级
 */
export function createClass(name: string, description?: string): Class {
  const cls: Class = {
    id: generateId(),
    name,
    description,
    createdAt: new Date().toISOString(),
    students: [],
    assignments: []
  }

  const classes = getStorage<Class>(CLASSES_KEY)
  classes.push(cls)
  setStorage(CLASSES_KEY, classes)

  return cls
}

/**
 * 获取所有班级
 */
export function getClasses(): Class[] {
  return getStorage<Class>(CLASSES_KEY)
}

/**
 * 获取单个班级
 */
export function getClass(classId: string): Class | undefined {
  const classes = getStorage<Class>(CLASSES_KEY)
  return classes.find(c => c.id === classId)
}

/**
 * 更新班级
 */
export function updateClass(classId: string, updates: Partial<Pick<Class, 'name' | 'description'>>): Class | null {
  const classes = getStorage<Class>(CLASSES_KEY)
  const cls = classes.find(c => c.id === classId)
  if (!cls) return null

  if (updates.name) cls.name = updates.name
  if (updates.description !== undefined) cls.description = updates.description

  setStorage(CLASSES_KEY, classes)
  return cls
}

/**
 * 删除班级
 */
export function deleteClass(classId: string): boolean {
  const classes = getStorage<Class>(CLASSES_KEY)
  const filtered = classes.filter(c => c.id !== classId)
  if (filtered.length === classes.length) return false

  setStorage(CLASSES_KEY, filtered)

  // 同时删除班级的所有作业和提交
  const assignments = getStorage<Assignment>(ASSIGNMENTS_KEY)
  const assignmentIds = assignments.filter(a => a.classId === classId).map(a => a.id)
  setStorage(ASSIGNMENTS_KEY, assignments.filter(a => a.classId !== classId))

  const submissions = getStorage<AssignmentSubmission>(SUBMISSIONS_KEY)
  setStorage(SUBMISSIONS_KEY, submissions.filter(s => !assignmentIds.includes(s.id)))

  return true
}

// ============ 学生管理 ============

/**
 * 添加学生到班级
 */
export function addStudentToClass(classId: string, studentName: string, email?: string): Student | null {
  const classes = getStorage<Class>(CLASSES_KEY)
  const cls = classes.find(c => c.id === classId)
  if (!cls) return null

  const student: Student = {
    id: generateId(),
    name: studentName,
    email,
    avatarColor: getAvatarColor(studentName)
  }

  cls.students.push(student)
  setStorage(CLASSES_KEY, classes)

  return student
}

/**
 * 从班级移除学生
 */
export function removeStudentFromClass(classId: string, studentId: string): boolean {
  const classes = getStorage<Class>(CLASSES_KEY)
  const cls = classes.find(c => c.id === classId)
  if (!cls) return false

  const idx = cls.students.findIndex(s => s.id === studentId)
  if (idx === -1) return false

  cls.students.splice(idx, 1)
  setStorage(CLASSES_KEY, classes)

  // 同时删除该学生的所有提交
  const submissions = getStorage<AssignmentSubmission>(SUBMISSIONS_KEY)
  setStorage(SUBMISSIONS_KEY, submissions.filter(s => s.studentId !== studentId))

  return true
}

/**
 * 更新学生信息
 */
export function updateStudent(classId: string, studentId: string, updates: Partial<Pick<Student, 'name' | 'email'>>): Student | null {
  const classes = getStorage<Class>(CLASSES_KEY)
  const cls = classes.find(c => c.id === classId)
  if (!cls) return null

  const student = cls.students.find(s => s.id === studentId)
  if (!student) return null

  if (updates.name) {
    student.name = updates.name
    student.avatarColor = getAvatarColor(updates.name)
  }
  if (updates.email !== undefined) student.email = updates.email

  setStorage(CLASSES_KEY, classes)
  return student
}

// ============ 作业管理 ============

/**
 * 创建作业
 */
export function createAssignment(
  classId: string,
  title: string,
  description: string,
  createdBy: string,
  options?: {
    templateId?: string
    templateSnapshot?: any
    dueDate?: string
    allowLate?: boolean
    maxAttempts?: number
  }
): Assignment | null {
  const cls = getClass(classId)
  if (!cls) return null

  const assignment: Assignment = {
    id: generateId(),
    title,
    description,
    templateId: options?.templateId,
    templateSnapshot: options?.templateSnapshot,
    dueDate: options?.dueDate,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
    createdBy,
    classId,
    submissions: [],
    allowLate: options?.allowLate ?? false,
    maxAttempts: options?.maxAttempts ?? 3
  }

  const assignments = getStorage<Assignment>(ASSIGNMENTS_KEY)
  assignments.push(assignment)
  setStorage(ASSIGNMENTS_KEY, assignments)

  return assignment
}

/**
 * 获取班级的所有作业
 */
export function getClassAssignments(classId: string): Assignment[] {
  return getStorage<Assignment>(ASSIGNMENTS_KEY).filter(a => a.classId === classId)
}

/**
 * 获取单个作业
 */
export function getAssignment(assignmentId: string): Assignment | undefined {
  return getStorage<Assignment>(ASSIGNMENTS_KEY).find(a => a.id === assignmentId)
}

/**
 * 更新作业
 */
export function updateAssignment(assignmentId: string, updates: Partial<Pick<Assignment, 'title' | 'description' | 'dueDate' | 'allowLate' | 'maxAttempts'>>): Assignment | null {
  const assignments = getStorage<Assignment>(ASSIGNMENTS_KEY)
  const assignment = assignments.find(a => a.id === assignmentId)
  if (!assignment) return null

  if (updates.title) assignment.title = updates.title
  if (updates.description !== undefined) assignment.description = updates.description
  if (updates.dueDate !== undefined) assignment.dueDate = updates.dueDate
  if (updates.allowLate !== undefined) assignment.allowLate = updates.allowLate
  if (updates.maxAttempts !== undefined) assignment.maxAttempts = updates.maxAttempts
  assignment.updatedAt = new Date().toISOString()

  setStorage(ASSIGNMENTS_KEY, assignments)
  return assignment
}

/**
 * 删除作业
 */
export function deleteAssignment(assignmentId: string): boolean {
  const assignments = getStorage<Assignment>(ASSIGNMENTS_KEY)
  const filtered = assignments.filter(a => a.id !== assignmentId)
  if (filtered.length === assignments.length) return false

  setStorage(ASSIGNMENTS_KEY, filtered)

  // 同时删除该作业的所有提交
  const submissions = getStorage<AssignmentSubmission>(SUBMISSIONS_KEY)
  setStorage(SUBMISSIONS_KEY, submissions.filter(s => s.id !== assignmentId))

  return true
}

// ============ 提交管理 ============

/**
 * 学生提交作业
 */
export function submitAssignment(
  assignmentId: string,
  studentId: string,
  studentName: string,
  projectId: string,
  projectSnapshot?: AssignmentSubmission['projectSnapshot']
): AssignmentSubmission | null {
  const assignment = getAssignment(assignmentId)
  if (!assignment) return null

  // 检查是否超过最大尝试次数
  const existingSubmissions = getAssignmentSubmissions(assignmentId)
  const studentSubmissions = existingSubmissions.filter(s => s.studentId === studentId)
  if (studentSubmissions.length >= assignment.maxAttempts) {
    throw new Error(`已达到最大提交次数（${assignment.maxAttempts}次）`)
  }

  const submission: AssignmentSubmission = {
    id: generateId(),
    assignmentId,
    studentId,
    studentName,
    submittedAt: new Date().toISOString(),
    projectId,
    projectSnapshot
  }

  const submissions = getStorage<AssignmentSubmission>(SUBMISSIONS_KEY)
  submissions.push(submission)
  setStorage(SUBMISSIONS_KEY, submissions)

  return submission
}

/**
 * 获取作业的所有提交
 */
export function getAssignmentSubmissions(assignmentId: string): AssignmentSubmission[] {
  return getStorage<AssignmentSubmission>(SUBMISSIONS_KEY)
    .filter(s => s.assignmentId === assignmentId)
    .sort((a, b) => new Date(b.submittedAt).getTime() - new Date(a.submittedAt).getTime())
}

/**
 * 给提交评分/反馈
 */
export function gradeSubmission(
  submissionId: string,
  score: number,
  feedback?: string
): AssignmentSubmission | null {
  const submissions = getStorage<AssignmentSubmission>(SUBMISSIONS_KEY)
  const submission = submissions.find(s => s.id === submissionId)
  if (!submission) return null

  submission.score = score
  submission.feedback = feedback
  setStorage(SUBMISSIONS_KEY, submissions)

  return submission
}

/**
 * 获取学生的某个作业的提交历史
 */
export function getStudentSubmissions(assignmentId: string, studentId: string): AssignmentSubmission[] {
  return getStorage<AssignmentSubmission>(SUBMISSIONS_KEY)
    .filter(s => s.assignmentId === assignmentId && s.studentId === studentId)
    .sort((a, b) => new Date(b.submittedAt).getTime() - new Date(a.submittedAt).getTime())
}

// ============ 结果对比 ============

/**
 * 对比学生结果与模板标准结果
 */
export function compareWithTemplate(
  submission: AssignmentSubmission,
  templateSnapshot?: any
): ResultComparison[] {
  if (!submission.projectSnapshot || !templateSnapshot) return []

  const comparisons: ResultComparison[] = []

  // 对比最大位移
  if (submission.projectSnapshot.maxDisplacement !== undefined && templateSnapshot.maxDisplacement !== undefined) {
    const deviation = Math.abs(
      (submission.projectSnapshot.maxDisplacement - templateSnapshot.maxDisplacement) / templateSnapshot.maxDisplacement * 100
    )
    comparisons.push({
      metric: '最大位移',
      studentValue: submission.projectSnapshot.maxDisplacement,
      templateValue: templateSnapshot.maxDisplacement,
      deviation
    })
  }

  // 对比最大应力
  if (submission.projectSnapshot.maxStress !== undefined && templateSnapshot.maxStress !== undefined) {
    const deviation = Math.abs(
      (submission.projectSnapshot.maxStress - templateSnapshot.maxStress) / templateSnapshot.maxStress * 100
    )
    comparisons.push({
      metric: '最大应力',
      studentValue: submission.projectSnapshot.maxStress,
      templateValue: templateSnapshot.maxStress,
      deviation
    })
  }

  // 对比网格规模
  if (submission.projectSnapshot.meshElements !== undefined && templateSnapshot.meshElements !== undefined) {
    const deviation = Math.abs(
      (submission.projectSnapshot.meshElements - templateSnapshot.meshElements) / templateSnapshot.meshElements * 100
    )
    comparisons.push({
      metric: '网格单元数',
      studentValue: submission.projectSnapshot.meshElements,
      templateValue: templateSnapshot.meshElements,
      deviation
    })
  }

  return comparisons
}

// ============ 批量操作 ============

/**
 * 批量添加学生（通过逗号分隔的名称列表）
 */
export function bulkAddStudents(classId: string, names: string): { added: Student[], failed: string[] } {
  const nameList = names.split(/[,，\n]/).map(n => n.trim()).filter(n => n)
  const added: Student[] = []
  const failed: string[] = []

  for (const name of nameList) {
    try {
      const student = addStudentToClass(classId, name)
      if (student) {
        added.push(student)
      } else {
        failed.push(name)
      }
    } catch {
      failed.push(name)
    }
  }

  return { added, failed }
}

/**
 * 获取班级统计数据
 */
export function getClassStats(classId: string): {
  totalStudents: number
  totalAssignments: number
  totalSubmissions: number
  averageScore: number
  submissionRate: number
} {
  const cls = getClass(classId)
  if (!cls) {
    return { totalStudents: 0, totalAssignments: 0, totalSubmissions: 0, averageScore: 0, submissionRate: 0 }
  }

  const assignments = getClassAssignments(classId)
  const allSubmissions: AssignmentSubmission[] = []
  let totalScore = 0
  let scoredCount = 0

  for (const assignment of assignments) {
    const submissions = getAssignmentSubmissions(assignment.id)
    allSubmissions.push(...submissions)
    for (const sub of submissions) {
      if (sub.score !== undefined) {
        totalScore += sub.score
        scoredCount++
      }
    }
  }

  const totalAssignments = assignments.length
  const submissionCount = allSubmissions.length
  const expectedSubmissions = cls.students.length * totalAssignments

  return {
    totalStudents: cls.students.length,
    totalAssignments,
    totalSubmissions: submissionCount,
    averageScore: scoredCount > 0 ? totalScore / scoredCount : 0,
    submissionRate: expectedSubmissions > 0 ? (submissionCount / expectedSubmissions) * 100 : 0
  }
}

// ============ V3.0-001 自动评分系统 ============

export interface GradingRubric {
  metric: string
  weight: number  // 权重 (0-1)
  tolerance: number  // 容许偏差百分比
 满分: number
}

const DEFAULT_RUBRIC: GradingRubric[] = [
  { metric: 'maxDisplacement', weight: 0.4, tolerance: 10, 满分: 40 },
  { metric: 'maxStress', weight: 0.4, tolerance: 15, 满分: 40 },
  { metric: 'meshElements', weight: 0.2, tolerance: 20, 满分: 20 }
]

/**
 * V3.0-001: 根据 rubric 自动评分
 */
export function autoGrade(
  submission: AssignmentSubmission,
  rubric: GradingRubric[] = DEFAULT_RUBRIC
): { score: number; breakdown: Array<{ metric: string; score: number; maxScore: number; reason: string }> } {
  const breakdown: Array<{ metric: string; score: number; maxScore: number; reason: string }> = []
  let totalScore = 0

  for (const item of rubric) {
    const studentVal = (submission.projectSnapshot as any)?.[item.metric]
    if (studentVal === undefined) {
      breakdown.push({
        metric: item.metric,
        score: 0,
        maxScore: item.满分,
        reason: '无结果数据'
      })
      continue
    }

    // 与标准值的偏差
    const templateVal = (submission.projectSnapshot as any)?.[`${item.metric}_template`]
    if (templateVal === undefined) {
      // 如果没有标准值，给满分
      breakdown.push({
        metric: item.metric,
        score: item.满分,
        maxScore: item.满分,
        reason: '无参考标准，满分'
      })
      totalScore += item.满分
      continue
    }

    const deviation = Math.abs((studentVal - templateVal) / templateVal * 100)

    if (deviation <= item.tolerance) {
      // 在容许范围内，按偏差比例给分
      const scoreRatio = 1 - (deviation / item.tolerance) * 0.3  // 最低给70%
      const score = Math.round(item.满分 * scoreRatio)
      breakdown.push({
        metric: item.metric,
        score,
        maxScore: item.满分,
        reason: `偏差 ${deviation.toFixed(1)}%，在容许范围(${item.tolerance}%)内`
      })
      totalScore += score
    } else {
      // 超出范围，按比例扣分
      const scoreRatio = Math.max(0, 1 - (deviation - item.tolerance) / item.tolerance)
      const score = Math.round(item.满分 * scoreRatio * 0.5)  // 最多扣一半
      breakdown.push({
        metric: item.metric,
        score,
        maxScore: item.满分,
        reason: `偏差 ${deviation.toFixed(1)}%，超过容许范围(${item.tolerance}%)`
      })
      totalScore += score
    }
  }

  return { score: Math.round(totalScore), breakdown }
}

/**
 * V3.0-001: 批量自动评分
 */
export function batchAutoGrade(
  assignmentId: string,
  rubric?: GradingRubric[]
): { graded: number; failed: number } {
  const submissions = getAssignmentSubmissions(assignmentId)
  let graded = 0
  let failed = 0

  for (const submission of submissions) {
    // 跳过已经有分数的
    if (submission.score !== undefined) continue

    try {
      const result = autoGrade(submission, rubric)
      gradeSubmission(submission.id, result.score, generateAutoFeedback(result))
      graded++
    } catch {
      failed++
    }
  }

  return { graded, failed }
}

/**
 * V3.0-001: 生成自动反馈
 */
function generateAutoFeedback(result: { score: number; breakdown: Array<{ metric: string; score: number; maxScore: number; reason: string }> }): string {
  const lines = [`自动评分得分: ${result.score}/100`, '']
  for (const item of result.breakdown) {
    lines.push(`- ${item.metric}: ${item.score}/${item.maxScore} — ${item.reason}`)
  }
  return lines.join('\n')
}

/**
 * V3.0-001: 获取学生待提交的作业列表
 */
export function getStudentAssignments(studentId: string): Array<{ assignment: Assignment; class: Class; submission?: AssignmentSubmission; isOverdue: boolean }> {
  const classes = getClasses()
  const results: Array<{ assignment: Assignment; class: Class; submission?: AssignmentSubmission; isOverdue: boolean }> = []

  for (const cls of classes) {
    const assignments = getClassAssignments(cls.id)
    for (const assignment of assignments) {
      // 检查是否已经在截止日期前提交
      const submissions = getAssignmentSubmissions(assignment.id)
      const studentSubmission = submissions.find(s => s.studentId === studentId)

      const dueDate = assignment.dueDate ? new Date(assignment.dueDate) : null
      const isOverdue = dueDate ? new Date() > dueDate : false

      results.push({
        assignment,
        class: cls,
        submission: studentSubmission,
        isOverdue
      })
    }
  }

  return results.sort((a, b) => {
    // 优先显示未提交的 > 即将到期 > 按截止日期排序
    if (!a.submission && b.submission) return -1
    if (a.submission && !b.submission) return 1
    if (a.isOverdue !== b.isOverdue) return a.isOverdue ? 1 : -1
    return new Date(a.assignment.dueDate || 0).getTime() - new Date(b.assignment.dueDate || 0).getTime()
  })
}