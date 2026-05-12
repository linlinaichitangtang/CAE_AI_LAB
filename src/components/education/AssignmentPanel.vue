<script setup lang="ts">
/**
 * AssignmentPanel.vue — V2.9-008 作业面板
 * 教师布置仿真作业、学生提交、结果对比
 */
import { ref, computed, onMounted } from 'vue'
import {
  type Class,
  type Assignment,
  type Student,
  type AssignmentSubmission,
  createClass,
  getClasses,
  deleteClass,
  updateClass,
  addStudentToClass,
  removeStudentFromClass,
  bulkAddStudents,
  createAssignment,
  getClassAssignments,
  deleteAssignment,
  updateAssignment,
  submitAssignment,
  getAssignmentSubmissions,
  gradeSubmission,
  compareWithTemplate,
  getClassStats,
  getAssignment
} from '@/api/assignment'

// ============ 状态 ============
const classes = ref<Class[]>([])
const selectedClassId = ref<string | null>(null)
const selectedAssignmentId = ref<string | null>(null)
const activeTab = ref<'classes' | 'assignments' | 'submissions'>('classes')
const currentUser = ref('教师')  // 实际应从用户系统获取

// 模态框状态
const showCreateClassDialog = ref(false)
const showAddStudentDialog = ref(false)
const showCreateAssignmentDialog = ref(false)
const showGradeDialog = ref(false)
const showResultCompareDialog = ref(false)

// 表单数据
const newClassForm = ref({ name: '', description: '' })
const newStudentForm = ref({ names: '', email: '' })
const newAssignmentForm = ref({
  title: '',
  description: '',
  templateId: '',
  dueDate: '',
  allowLate: false,
  maxAttempts: 3
})
const gradingForm = ref({ score: 0, feedback: '' })
const selectedSubmission = ref<AssignmentSubmission | null>(null)

// ============ 计算属性 ============
const selectedClass = computed(() =>
  classes.value.find(c => c.id === selectedClassId.value) || null
)

const selectedAssignment = computed(() =>
  selectedAssignmentId.value ? getAssignment(selectedAssignmentId.value) : null
)

const selectedAssignmentSubmissions = computed(() =>
  selectedAssignmentId.value ? getAssignmentSubmissions(selectedAssignmentId.value) : []
)

const classStats = computed(() =>
  selectedClassId.value ? getClassStats(selectedClassId.value) : null
)

// ============ 生命周期 ============
function loadClasses() {
  classes.value = getClasses()
}

// ============ 班级管理 ============
function handleCreateClass() {
  if (!newClassForm.value.name.trim()) return
  const cls = createClass(newClassForm.value.name, newClassForm.value.description)
  classes.value.push(cls)
  selectedClassId.value = cls.id
  showCreateClassDialog.value = false
  newClassForm.value = { name: '', description: '' }
}

function handleDeleteClass(classId: string) {
  if (!confirm('确定要删除该班级吗？所有作业和提交记录将被一并删除。')) return
  deleteClass(classId)
  classes.value = getClasses()
  if (selectedClassId.value === classId) {
    selectedClassId.value = null
    selectedAssignmentId.value = null
  }
}

function handleAddStudent() {
  if (!selectedClassId.value || !newStudentForm.value.names.trim()) return
  const result = bulkAddStudents(selectedClassId.value, newStudentForm.value.names)
  classes.value = getClasses()
  showAddStudentDialog.value = false
  newStudentForm.value = { names: '', email: '' }
  if (result.failed.length > 0) {
    alert(`以下学生添加失败：${result.failed.join(', ')}`)
  }
}

function handleRemoveStudent(classId: string, studentId: string) {
  removeStudentFromClass(classId, studentId)
  classes.value = getClasses()
}

// ============ 作业管理 ============
function handleCreateAssignment() {
  if (!selectedClassId.value || !newAssignmentForm.value.title.trim()) return
  const assignment = createAssignment(
    selectedClassId.value,
    newAssignmentForm.value.title,
    newAssignmentForm.value.description,
    currentUser.value,
    {
      templateId: newAssignmentForm.value.templateId || undefined,
      dueDate: newAssignmentForm.value.dueDate || undefined,
      allowLate: newAssignmentForm.value.allowLate,
      maxAttempts: newAssignmentForm.value.maxAttempts
    }
  )
  if (assignment) {
    classes.value = getClasses()
    selectedAssignmentId.value = assignment.id
  }
  showCreateAssignmentDialog.value = false
  newAssignmentForm.value = {
    title: '',
    description: '',
    templateId: '',
    dueDate: '',
    allowLate: false,
    maxAttempts: 3
  }
}

function handleDeleteAssignment(assignmentId: string) {
  if (!confirm('确定要删除该作业吗？')) return
  deleteAssignment(assignmentId)
  classes.value = getClasses()
  if (selectedAssignmentId.value === assignmentId) {
    selectedAssignmentId.value = null
  }
}

function handleGrade(submission: AssignmentSubmission) {
  selectedSubmission.value = submission
  gradingForm.value = {
    score: submission.score ?? 0,
    feedback: submission.feedback ?? ''
  }
  showGradeDialog.value = true
}

function saveGrade() {
  if (!selectedSubmission.value) return
  gradeSubmission(
    selectedSubmission.value.id,
    gradingForm.value.score,
    gradingForm.value.feedback
  )
  classes.value = getClasses()
  showGradeDialog.value = false
}

function handleCompareResults(submission: AssignmentSubmission) {
  selectedSubmission.value = submission
  showResultCompareDialog.value = true
}

// ============ 提交模拟（学生视角）==========
function handleSubmitHomework(assignmentId: string) {
  const studentId = 'student_demo'
  const studentName = '张三'
  submitAssignment(assignmentId, studentId, studentName, 'demo_project', {
    maxDisplacement: Math.random() * 0.01,
    maxStress: Math.random() * 200 + 50,
    meshNodes: 200 + Math.floor(Math.random() * 100),
    meshElements: 100 + Math.floor(Math.random() * 50)
  })
  classes.value = getClasses()
  alert('提交成功！')
}

// ============ 工具函数 ============
function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleString('zh-CN', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

function formatDueDate(dueDate?: string): { text: string, isOverdue: boolean } {
  if (!dueDate) return { text: '无截止日期', isOverdue: false }
  const due = new Date(dueDate)
  const now = new Date()
  return {
    text: due.toLocaleString('zh-CN', { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' }),
    isOverdue: due < now
  }
}

function getScoreColor(score?: number): string {
  if (score === undefined) return 'text-gray-400'
  if (score >= 90) return 'text-green-600'
  if (score >= 70) return 'text-blue-600'
  if (score >= 60) return 'text-yellow-600'
  return 'text-red-600'
}

onMounted(() => {
  loadClasses()
})
</script>

<template>
  <div class="assignment-panel h-full flex bg-[var(--bg-ground)]">
    <!-- 左侧：班级列表 -->
    <div class="w-72 bg-[var(--bg-surface)] border-r flex flex-col">
      <div class="p-4 border-b border-[var(--border-subtle)]">
        <h2 class="font-semibold text-[var(--text-primary)] flex items-center gap-2 mb-3">
          <span>🎓</span>
          <span>教学管理</span>
        </h2>
        <button
          @click="showCreateClassDialog = true"
          class="w-full py-2 px-3 bg-[var(--primary)] text-white rounded-lg text-sm hover:opacity-90 transition-opacity flex items-center justify-center gap-2"
        >
          <span>+</span>
          <span>新建班级</span>
        </button>
      </div>

      <!-- 班级列表 -->
      <div class="flex-1 overflow-y-auto p-2">
        <div v-if="classes.length === 0" class="text-center py-8 text-[var(--text-muted)]">
          <div class="text-4xl mb-2 opacity-50">📚</div>
          <p class="text-sm">暂无班级</p>
          <p class="text-xs mt-1">点击上方按钮创建班级</p>
        </div>
        <div
          v-for="cls in classes"
          :key="cls.id"
          class="mb-2"
        >
          <!-- 班级项 -->
          <div
            class="p-3 rounded-lg cursor-pointer transition-colors border"
            :class="selectedClassId === cls.id
              ? 'bg-[var(--primary-glow)] border-[var(--primary)]'
              : 'bg-[var(--bg-elevated)] hover:bg-[var(--bg-hover)] border-transparent'"
            @click="selectedClassId = cls.id; selectedAssignmentId = null"
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span class="text-lg">🏫</span>
                <span class="font-medium text-sm text-[var(--text-primary)]">{{ cls.name }}</span>
              </div>
              <span class="text-xs text-[var(--text-muted)]">{{ cls.students.length }}人</span>
            </div>
            <p v-if="cls.description" class="text-xs text-[var(--text-muted)] mt-1 truncate">
              {{ cls.description }}
            </p>
          </div>

          <!-- 班级的作业列表 -->
          <div v-if="selectedClassId === cls.id" class="ml-4 mt-1 space-y-1">
            <div
              v-for="assignment in getClassAssignments(cls.id)"
              :key="assignment.id"
              class="p-2 rounded cursor-pointer text-xs transition-colors flex items-center justify-between"
              :class="selectedAssignmentId === assignment.id
                ? 'bg-blue-50 text-blue-700'
                : 'bg-[var(--bg-elevated)] hover:bg-[var(--bg-hover)] text-[var(--text-secondary)]'"
              @click.stop="selectedAssignmentId = assignment.id"
            >
              <div class="flex items-center gap-2">
                <span>📋</span>
                <span class="truncate max-w-[120px]">{{ assignment.title }}</span>
              </div>
              <span class="text-[10px]" :class="formatDueDate(assignment.dueDate).isOverdue ? 'text-red-500' : 'text-[var(--text-muted)]'">
                {{ formatDueDate(assignment.dueDate).text }}
              </span>
            </div>
            <button
              v-if="selectedClassId === cls.id"
              @click.stop="activeTab = 'assignments'"
              class="w-full py-1.5 text-xs text-[var(--primary)] hover:bg-[var(--bg-hover)] rounded transition-colors"
            >
              + 创建作业
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 右侧：详情面板 -->
    <div class="flex-1 flex flex-col overflow-hidden">
      <!-- 无选中班级 -->
      <div v-if="!selectedClassId" class="flex-1 flex items-center justify-center text-[var(--text-muted)]">
        <div class="text-center">
          <div class="text-6xl mb-4 opacity-30">👈</div>
          <p class="text-lg">选择一个班级查看详情</p>
          <p class="text-sm mt-1">或创建一个新班级开始</p>
        </div>
      </div>

      <!-- 班级详情 -->
      <div v-else-if="selectedClass && !selectedAssignmentId" class="flex-1 flex flex-col">
        <!-- 班级头部 -->
        <div class="p-6 border-b border-[var(--border-subtle)]">
          <div class="flex items-center justify-between mb-4">
            <div>
              <h2 class="text-xl font-bold text-[var(--text-primary)]">{{ selectedClass.name }}</h2>
              <p v-if="selectedClass.description" class="text-sm text-[var(--text-muted)] mt-1">
                {{ selectedClass.description }}
              </p>
            </div>
            <div class="flex gap-2">
              <button
                @click="showAddStudentDialog = true"
                class="px-3 py-1.5 bg-[var(--primary)] text-white rounded text-sm hover:opacity-90"
              >
                添加学生
              </button>
              <button
                @click="activeTab = 'assignments'"
                class="px-3 py-1.5 border border-[var(--border-subtle)] rounded text-sm hover:bg-[var(--bg-hover)]"
              >
                布置作业
              </button>
              <button
                @click="handleDeleteClass(selectedClass.id)"
                class="px-3 py-1.5 text-red-500 hover:bg-red-50 rounded text-sm"
              >
                删除班级
              </button>
            </div>
          </div>

          <!-- 统计卡片 -->
          <div v-if="classStats" class="grid grid-cols-4 gap-4">
            <div class="bg-[var(--bg-elevated)] rounded-lg p-3 text-center">
              <p class="text-2xl font-bold text-[var(--primary)]">{{ classStats.totalStudents }}</p>
              <p class="text-xs text-[var(--text-muted)]">学生数</p>
            </div>
            <div class="bg-[var(--bg-elevated)] rounded-lg p-3 text-center">
              <p class="text-2xl font-bold text-[var(--primary)]">{{ classStats.totalAssignments }}</p>
              <p class="text-xs text-[var(--text-muted)]">作业数</p>
            </div>
            <div class="bg-[var(--bg-elevated)] rounded-lg p-3 text-center">
              <p class="text-2xl font-bold text-[var(--primary)]">{{ classStats.totalSubmissions }}</p>
              <p class="text-xs text-[var(--text-muted)]">提交数</p>
            </div>
            <div class="bg-[var(--bg-elevated)] rounded-lg p-3 text-center">
              <p class="text-2xl font-bold" :class="classStats.averageScore >= 60 ? 'text-green-600' : 'text-red-500'">
                {{ classStats.averageScore > 0 ? classStats.averageScore.toFixed(1) : '--' }}
              </p>
              <p class="text-xs text-[var(--text-muted)]">平均分</p>
            </div>
          </div>
        </div>

        <!-- 学生列表 -->
        <div class="flex-1 overflow-y-auto p-6">
          <h3 class="font-semibold text-[var(--text-primary)] mb-4">学生列表 ({{ selectedClass.students.length }})</h3>
          <div v-if="selectedClass.students.length === 0" class="text-center py-8 text-[var(--text-muted)]">
            <p class="text-sm">暂无学生</p>
            <button
              @click="showAddStudentDialog = true"
              class="mt-2 text-sm text-[var(--primary)] hover:underline"
            >
              点击添加学生
            </button>
          </div>
          <div v-else class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3">
            <div
              v-for="student in selectedClass.students"
              :key="student.id"
              class="bg-[var(--bg-elevated)] rounded-lg p-3 flex items-center gap-3 group"
            >
              <div
                class="w-10 h-10 rounded-full flex items-center justify-center text-white font-semibold text-sm"
                :style="{ backgroundColor: student.avatarColor }"
              >
                {{ student.name.charAt(0) }}
              </div>
              <div class="flex-1 min-w-0">
                <p class="font-medium text-sm text-[var(--text-primary)] truncate">{{ student.name }}</p>
                <p v-if="student.email" class="text-xs text-[var(--text-muted)] truncate">{{ student.email }}</p>
              </div>
              <button
                @click="handleRemoveStudent(selectedClass.id, student.id)"
                class="opacity-0 group-hover:opacity-100 text-red-400 hover:text-red-600 text-sm transition-opacity"
              >
                &times;
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 作业详情 -->
      <div v-else-if="selectedAssignment" class="flex-1 flex flex-col overflow-hidden">
        <div class="p-6 border-b border-[var(--border-subtle)]">
          <div class="flex items-center justify-between">
            <div>
              <div class="flex items-center gap-3">
                <h2 class="text-xl font-bold text-[var(--text-primary)]">{{ selectedAssignment.title }}</h2>
                <span
                  class="px-2 py-0.5 rounded text-xs"
                  :class="formatDueDate(selectedAssignment.dueDate).isOverdue ? 'bg-red-100 text-red-600' : 'bg-green-100 text-green-600'"
                >
                  {{ formatDueDate(selectedAssignment.dueDate).text }}
                </span>
              </div>
              <p v-if="selectedAssignment.description" class="text-sm text-[var(--text-muted)] mt-1">
                {{ selectedAssignment.description }}
              </p>
              <p class="text-xs text-[var(--text-muted)] mt-1">
                创建者: {{ selectedAssignment.createdBy }} |
                允许提交 {{ selectedAssignment.maxAttempts }} 次 |
                {{ selectedAssignment.allowLate ? '允许迟交' : '不允许迟交' }}
              </p>
            </div>
            <div class="flex gap-2">
              <button
                @click="handleSubmitHomework(selectedAssignment.id)"
                class="px-3 py-1.5 bg-green-600 text-white rounded text-sm hover:bg-green-700"
              >
                模拟学生提交
              </button>
              <button
                @click="handleDeleteAssignment(selectedAssignment.id)"
                class="px-3 py-1.5 text-red-500 hover:bg-red-50 rounded text-sm"
              >
                删除作业
              </button>
            </div>
          </div>
        </div>

        <!-- 提交列表 -->
        <div class="flex-1 overflow-y-auto p-6">
          <h3 class="font-semibold text-[var(--text-primary)] mb-4">
            提交列表 ({{ selectedAssignmentSubmissions.length }})
          </h3>
          <div v-if="selectedAssignmentSubmissions.length === 0" class="text-center py-8 text-[var(--text-muted)]">
            <div class="text-4xl mb-2 opacity-50">📋</div>
            <p class="text-sm">暂无提交</p>
            <p class="text-xs mt-1">学生提交作业后将显示在这里</p>
          </div>
          <div v-else class="space-y-3">
            <div
              v-for="submission in selectedAssignmentSubmissions"
              :key="submission.id"
              class="bg-[var(--bg-elevated)] rounded-lg p-4"
            >
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <div class="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center text-blue-600 font-semibold text-sm">
                    {{ submission.studentName.charAt(0) }}
                  </div>
                  <div>
                    <p class="font-medium text-sm text-[var(--text-primary)]">{{ submission.studentName }}</p>
                    <p class="text-xs text-[var(--text-muted)]">{{ formatDate(submission.submittedAt) }}</p>
                  </div>
                </div>
                <div class="flex items-center gap-4">
                  <!-- 分数 -->
                  <div v-if="submission.score !== undefined" class="text-right">
                    <p class="text-lg font-bold" :class="getScoreColor(submission.score)">
                      {{ submission.score }}
                    </p>
                    <p class="text-xs text-[var(--text-muted)]">分</p>
                  </div>
                  <div v-else class="text-[var(--text-muted)] text-sm">
                    未评分
                  </div>
                  <!-- 操作按钮 -->
                  <div class="flex gap-1">
                    <button
                      @click="handleCompareResults(submission)"
                      class="px-2 py-1 text-xs text-blue-600 hover:bg-blue-50 rounded transition-colors"
                      title="对比结果"
                    >
                      📊
                    </button>
                    <button
                      @click="handleGrade(submission)"
                      class="px-2 py-1 text-xs bg-[var(--primary)] text-white rounded hover:opacity-90"
                    >
                      {{ submission.score !== undefined ? '修改评分' : '评分' }}
                    </button>
                  </div>
                </div>
              </div>
              <!-- 结果预览 -->
              <div v-if="submission.projectSnapshot" class="mt-3 flex gap-4 text-xs text-[var(--text-muted)]">
                <span v-if="submission.projectSnapshot.maxDisplacement">
                  最大位移: {{ submission.projectSnapshot.maxDisplacement.toFixed(4) }} m
                </span>
                <span v-if="submission.projectSnapshot.maxStress">
                  最大应力: {{ submission.projectSnapshot.maxStress.toFixed(0) }} MPa
                </span>
                <span v-if="submission.projectSnapshot.meshElements">
                  网格: {{ submission.projectSnapshot.meshElements }} 单元
                </span>
              </div>
              <!-- 反馈 -->
              <div v-if="submission.feedback" class="mt-2 text-xs text-[var(--text-secondary)] bg-[var(--bg-surface)] rounded p-2">
                📝 {{ submission.feedback }}
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 无选中作业 -->
      <div v-else class="flex-1 flex items-center justify-center text-[var(--text-muted)]">
        <div class="text-center">
          <div class="text-6xl mb-4 opacity-30">📋</div>
          <p class="text-lg">选择一个作业查看提交情况</p>
        </div>
      </div>
    </div>

    <!-- ========== 模态框 ========== -->

    <!-- 创建班级 -->
    <div v-if="showCreateClassDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showCreateClassDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-xl p-6 w-full max-w-md shadow-lg">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">新建班级</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2">班级名称</label>
            <input
              v-model="newClassForm.name"
              type="text"
              class="input w-full"
              placeholder="例如：力学2024班"
            />
          </div>
          <div>
            <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2">班级描述（可选）</label>
            <textarea
              v-model="newClassForm.description"
              rows="2"
              class="input w-full resize-none"
              placeholder="班级的简短描述"
            />
          </div>
        </div>
        <div class="flex justify-end gap-3 mt-6">
          <button @click="showCreateClassDialog = false" class="btn btn-ghost">取消</button>
          <button @click="handleCreateClass" class="btn btn-primary">创建</button>
        </div>
      </div>
    </div>

    <!-- 添加学生 -->
    <div v-if="showAddStudentDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showAddStudentDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-xl p-6 w-full max-w-md shadow-lg">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">添加学生</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2">学生姓名（多个用逗号分隔）</label>
            <textarea
              v-model="newStudentForm.names"
              rows="4"
              class="input w-full resize-none"
              placeholder="张三, 李四, 王五"
            />
          </div>
        </div>
        <div class="flex justify-end gap-3 mt-6">
          <button @click="showAddStudentDialog = false" class="btn btn-ghost">取消</button>
          <button @click="handleAddStudent" class="btn btn-primary">添加</button>
        </div>
      </div>
    </div>

    <!-- 创建作业 -->
    <div v-if="showCreateAssignmentDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showCreateAssignmentDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-xl p-6 w-full max-w-lg shadow-lg">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">布置作业</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2">作业标题</label>
            <input v-model="newAssignmentForm.title" type="text" class="input w-full" placeholder="例如：悬臂梁静力学分析" />
          </div>
          <div>
            <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2">作业描述</label>
            <textarea v-model="newAssignmentForm.description" rows="3" class="input w-full resize-none" placeholder="详细说明作业要求..." />
          </div>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2">截止日期</label>
              <input v-model="newAssignmentForm.dueDate" type="datetime-local" class="input w-full" />
            </div>
            <div>
              <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2">最大提交次数</label>
              <input v-model.number="newAssignmentForm.maxAttempts" type="number" min="1" max="10" class="input w-full" />
            </div>
          </div>
          <div class="flex items-center gap-2">
            <input v-model="newAssignmentForm.allowLate" type="checkbox" id="allowLate" class="rounded" />
            <label for="allowLate" class="text-sm text-[var(--text-secondary)]">允许迟交</label>
          </div>
        </div>
        <div class="flex justify-end gap-3 mt-6">
          <button @click="showCreateAssignmentDialog = false" class="btn btn-ghost">取消</button>
          <button @click="handleCreateAssignment" class="btn btn-primary">创建作业</button>
        </div>
      </div>
    </div>

    <!-- 评分 -->
    <div v-if="showGradeDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showGradeDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-xl p-6 w-full max-w-md shadow-lg">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">评分</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2">分数</label>
            <input v-model.number="gradingForm.score" type="number" min="0" max="100" class="input w-full" />
          </div>
          <div>
            <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2">评语</label>
            <textarea v-model="gradingForm.feedback" rows="3" class="input w-full resize-none" placeholder="给予学生反馈..." />
          </div>
        </div>
        <div class="flex justify-end gap-3 mt-6">
          <button @click="showGradeDialog = false" class="btn btn-ghost">取消</button>
          <button @click="saveGrade" class="btn btn-primary">保存</button>
        </div>
      </div>
    </div>

    <!-- 结果对比 -->
    <div v-if="showResultCompareDialog && selectedSubmission" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showResultCompareDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-xl p-6 w-full max-w-lg shadow-lg">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">结果对比</h3>
        <div class="space-y-4">
          <div class="bg-[var(--bg-elevated)] rounded-lg p-4">
            <p class="text-sm text-[var(--text-secondary)] mb-2">学生结果</p>
            <div class="grid grid-cols-2 gap-4 text-sm">
              <div>
                <p class="text-xs text-[var(--text-muted)]">最大位移</p>
                <p class="font-medium">{{ selectedSubmission.projectSnapshot?.maxDisplacement?.toFixed(6) || '--' }} m</p>
              </div>
              <div>
                <p class="text-xs text-[var(--text-muted)]">最大应力</p>
                <p class="font-medium">{{ selectedSubmission.projectSnapshot?.maxStress?.toFixed(2) || '--' }} MPa</p>
              </div>
              <div>
                <p class="text-xs text-[var(--text-muted)]">网格节点</p>
                <p class="font-medium">{{ selectedSubmission.projectSnapshot?.meshNodes || '--' }}</p>
              </div>
              <div>
                <p class="text-xs text-[var(--text-muted)]">网格单元</p>
                <p class="font-medium">{{ selectedSubmission.projectSnapshot?.meshElements || '--' }}</p>
              </div>
            </div>
          </div>
        </div>
        <div class="flex justify-end gap-3 mt-6">
          <button @click="showResultCompareDialog = false" class="btn btn-primary">关闭</button>
        </div>
      </div>
    </div>
  </div>
</template>