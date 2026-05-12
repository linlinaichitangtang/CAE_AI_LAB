<script setup lang="ts">
/**
 * LMSConfiguration.vue — V3.0-002 LMS 集成 UI
 * 配置 Canvas/Blackboard/Moodle 等 LMS 连接
 */
import { ref, computed, onMounted } from 'vue'
import { useLMSIntegration, LMS_PROVIDERS, type LMSConfig, type LMSProvider, type LMSCourse } from '@/composables/useLMSIntegration'

// ============ 状态 ============
const {
  config,
  isConnected,
  isSyncing,
  syncError,
  connect,
  disconnect,
  testConnection,
  getCourses,
  getAssignments
} = useLMSIntegration()

// 连接表单
const connectForm = ref({
  provider: 'canvas' as LMSProvider,
  baseUrl: '',
  apiToken: '',
  courseId: ''
})

// 连接状态
const isTesting = ref(false)
const testResult = ref<{ success: boolean; error?: string; courses?: LMSCourse[] } | null>(null)
const showTokenHelp = ref(false)

// 已加载的课程和作业
const courses = ref<LMSCourse[]>([])
const assignments = ref<any[]>([])
const selectedCourseId = ref<string>('')
const isLoadingCourses = ref(false)

// ============ 计算属性 ============
const hasConfig = computed(() => !!config.value)
const canConnect = computed(() => connectForm.value.baseUrl && connectForm.value.apiToken)

// ============ 工具函数 ============
function getProviderInfo(provider: LMSProvider) {
  return LMS_PROVIDERS.find(p => p.id === provider) || LMS_PROVIDERS[3]
}

function formatDate(dateStr?: string): string {
  if (!dateStr) return ''
  return new Date(dateStr).toLocaleString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// ============ 操作函数 ============
async function handleTestConnection() {
  isTesting.value = true
  testResult.value = null

  try {
    const result = await testConnection(connectForm.value as LMSConfig)
    testResult.value = result

    if (result.success && result.courses) {
      courses.value = result.courses
    }
  } catch (e: any) {
    testResult.value = { success: false, error: e.message }
  } finally {
    isTesting.value = false
  }
}

async function handleConnect() {
  if (!canConnect.value) return

  const success = await connect(connectForm.value as LMSConfig)
  if (success) {
    testResult.value = null
    // 加载课程列表
    await loadCourses()
  }
}

function handleDisconnect() {
  disconnect()
  courses.value = []
  assignments.value = []
  selectedCourseId.value = ''
}

async function loadCourses() {
  isLoadingCourses.value = true
  try {
    courses.value = await getCourses()
  } finally {
    isLoadingCourses.value = false
  }
}

async function loadAssignments(courseId: string) {
  selectedCourseId.value = courseId
  assignments.value = await getAssignments(courseId)
}

// ============ 生命周期 ============
onMounted(() => {
  // 如果已有配置，加载课程
  if (config.value?.enabled) {
    loadCourses()
  }

  // 填充表单（如果已有配置）
  if (config.value) {
    connectForm.value = {
      provider: config.value.provider,
      baseUrl: config.value.baseUrl,
      apiToken: config.value.apiToken,
      courseId: config.value.courseId || ''
    }
  }
})
</script>

<template>
  <div class="lms-configuration h-full flex flex-col bg-[var(--bg-ground)]">
    <!-- 顶部栏 -->
    <div class="bg-[var(--bg-surface)] border-b border-[var(--border-subtle)] px-6 py-4">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-xl font-bold text-[var(--text-primary)] flex items-center gap-2">
            <span>🎓</span>
            <span>LMS 集成</span>
          </h1>
          <p class="text-sm text-[var(--text-muted)] mt-1">
            连接 Canvas、Blackboard、Moodle 等教育平台
          </p>
        </div>
        <div class="flex items-center gap-2">
          <span
            class="px-3 py-1.5 rounded-full text-sm font-medium"
            :class="isConnected
              ? 'bg-green-100 text-green-700'
              : 'bg-gray-100 text-gray-500'"
          >
            {{ isConnected ? '🟢 已连接' : '⚫ 未连接' }}
          </span>
          <button
            v-if="isConnected"
            @click="loadCourses"
            :disabled="isLoadingCourses"
            class="px-3 py-1.5 border border-[var(--border-subtle)] rounded text-sm hover:bg-[var(--bg-hover)] disabled:opacity-50"
          >
            🔄 刷新
          </button>
        </div>
      </div>
    </div>

    <!-- 内容区 -->
    <div class="flex-1 overflow-y-auto p-6">
      <!-- 未连接状态：连接表单 -->
      <div v-if="!isConnected" class="max-w-2xl mx-auto">
        <div class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] p-6">
          <h2 class="text-lg font-semibold text-[var(--text-primary)] mb-4">连接教育平台</h2>

          <!-- 选择平台 -->
          <div class="mb-6">
            <label class="block text-sm font-medium text-[var(--text-secondary)] mb-2">选择平台</label>
            <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
              <button
                v-for="provider in LMS_PROVIDERS"
                :key="provider.id"
                @click="connectForm.provider = provider.id"
                class="p-4 rounded-lg border-2 transition-all text-center"
                :class="connectForm.provider === provider.id
                  ? 'border-[var(--primary)] bg-[var(--primary)]/5'
                  : 'border-[var(--border-subtle)] hover:border-[var(--primary)]/50'"
              >
                <div class="text-2xl mb-1">{{ provider.icon }}</div>
                <div class="text-sm font-medium text-[var(--text-primary)]">{{ provider.name }}</div>
                <div class="text-xs text-[var(--text-muted)] mt-1">{{ provider.description }}</div>
              </button>
            </div>
          </div>

          <!-- 平台 URL -->
          <div class="mb-4">
            <label class="block text-sm font-medium text-[var(--text-secondary)] mb-2">
              API 地址
              <span class="text-[var(--text-muted)] ml-1">({{ getProviderInfo(connectForm.provider).name }} 实例 URL)</span>
            </label>
            <input
              v-model="connectForm.baseUrl"
              type="url"
              class="input w-full"
              placeholder="https://your-school.instructure.com"
            />
            <p class="text-xs text-[var(--text-muted)] mt-1">
              {{ connectForm.provider === 'canvas' ? '例如: https://canvas.instructure.com' :
                 connectForm.provider === 'blackboard' ? '例如: https://blackboard.university.edu' :
                 connectForm.provider === 'moodle' ? '例如: https://moodle.university.edu' :
                 '输入 LMS 的 REST API 基础 URL' }}
            </p>
          </div>

          <!-- API Token -->
          <div class="mb-4">
            <label class="block text-sm font-medium text-[var(--text-secondary)] mb-2">
              API Token
              <button
                @click="showTokenHelp = !showTokenHelp"
                class="ml-1 text-xs text-[var(--primary)] hover:underline"
              >
                如何获取？
              </button>
            </label>
            <input
              v-model="connectForm.apiToken"
              type="password"
              class="input w-full"
              placeholder="输入 API Access Token"
            />
            <div v-if="showTokenHelp" class="mt-2 p-3 bg-[var(--bg-elevated)] rounded-lg text-sm">
              <p class="font-medium text-[var(--text-primary)] mb-1">获取 API Token 步骤：</p>
              <ol class="list-decimal list-inside text-[var(--text-secondary)] space-y-1">
                <li>登录 {{ getProviderInfo(connectForm.provider).name }}</li>
                <li>进入「管理员」或「账户设置」</li>
                <li>找到「开发者」或「API」设置</li>
                <li>创建新的 Access Token</li>
                <li>复制生成的 Token 并粘贴到此处</li>
              </ol>
            </div>
          </div>

          <!-- 测试连接按钮 -->
          <div class="flex gap-3 mb-4">
            <button
              @click="handleTestConnection"
              :disabled="!canConnect || isTesting"
              class="px-4 py-2 border border-[var(--border-subtle)] rounded-lg text-sm hover:bg-[var(--bg-hover)] disabled:opacity-50"
            >
              {{ isTesting ? '测试中...' : '🔍 测试连接' }}
            </button>
          </div>

          <!-- 测试结果 -->
          <div v-if="testResult" class="mb-4 p-4 rounded-lg" :class="testResult.success ? 'bg-green-50' : 'bg-red-50'">
            <div v-if="testResult.success" class="text-green-700">
              <p class="font-medium">✅ 连接成功！</p>
              <p class="text-sm mt-1">找到 {{ testResult.courses?.length || 0 }} 门课程</p>
            </div>
            <div v-else class="text-red-700">
              <p class="font-medium">❌ 连接失败</p>
              <p class="text-sm mt-1">{{ testResult.error }}</p>
            </div>
          </div>

          <!-- 连接按钮 -->
          <div class="flex justify-end gap-3 pt-4 border-t border-[var(--border-subtle)]">
            <button
              @click="handleConnect"
              :disabled="!canConnect || isTesting"
              class="px-6 py-2 bg-[var(--primary)] text-white rounded-lg text-sm hover:opacity-90 disabled:opacity-50"
            >
              连接
            </button>
          </div>
        </div>
      </div>

      <!-- 已连接状态：课程和作业 -->
      <div v-else class="space-y-6">
        <!-- 当前配置信息 -->
        <div class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] p-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-lg bg-[var(--primary)]/10 flex items-center justify-center text-xl">
                {{ getProviderInfo(config?.provider || 'canvas').icon }}
              </div>
              <div>
                <p class="font-medium text-[var(--text-primary)]">{{ getProviderInfo(config?.provider || 'canvas').name }}</p>
                <p class="text-sm text-[var(--text-muted)]">{{ config?.baseUrl }}</p>
              </div>
            </div>
            <button
              @click="handleDisconnect"
              class="px-4 py-2 border border-red-200 text-red-600 rounded-lg text-sm hover:bg-red-50"
            >
              断开连接
            </button>
          </div>
        </div>

        <!-- 课程列表 -->
        <div class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
          <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
            <h2 class="font-semibold text-[var(--text-primary)]">📚 我的课程</h2>
          </div>

          <div v-if="isLoadingCourses" class="p-8 text-center text-[var(--text-muted)]">
            加载中...
          </div>

          <div v-else-if="courses.length === 0" class="p-8 text-center text-[var(--text-muted)]">
            暂无可用课程
          </div>

          <div v-else class="divide-y divide-[var(--border-subtle)]">
            <div
              v-for="course in courses"
              :key="course.id"
              class="p-4 hover:bg-[var(--bg-hover)] cursor-pointer"
              :class="selectedCourseId === course.id ? 'bg-[var(--primary)]/5' : ''"
              @click="loadAssignments(course.id)"
            >
              <div class="flex items-center justify-between">
                <div>
                  <p class="font-medium text-[var(--text-primary)]">{{ course.name }}</p>
                  <p class="text-sm text-[var(--text-muted)]">{{ course.code }} • {{ course.students }} 名学生</p>
                </div>
                <span class="text-xs text-[var(--text-muted)]">{{ course.lmsId }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- 作业列表 -->
        <div v-if="selectedCourseId" class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
          <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
            <h2 class="font-semibold text-[var(--text-primary)]">📋 作业列表</h2>
          </div>

          <div v-if="assignments.length === 0" class="p-8 text-center text-[var(--text-muted)]">
            暂无作业
          </div>

          <div v-else class="divide-y divide-[var(--border-subtle)]">
            <div
              v-for="assignment in assignments"
              :key="assignment.id"
              class="p-4"
            >
              <div class="flex items-start justify-between">
                <div>
                  <p class="font-medium text-[var(--text-primary)]">{{ assignment.title }}</p>
                  <p class="text-sm text-[var(--text-muted)] mt-1">{{ assignment.description || '无描述' }}</p>
                  <div class="flex items-center gap-3 mt-2 text-xs text-[var(--text-muted)]">
                    <span>📊 {{ assignment.pointsPossible }} 分</span>
                    <span v-if="assignment.dueDate">📅 {{ formatDate(assignment.dueDate) }}</span>
                    <span
                      class="px-2 py-0.5 rounded-full"
                      :class="{
                        'bg-yellow-100 text-yellow-700': assignment.status === 'pending',
                        'bg-blue-100 text-blue-700': assignment.status === 'submitted',
                        'bg-green-100 text-green-700': assignment.status === 'graded'
                      }"
                    >
                      {{ assignment.status === 'pending' ? '待提交' :
                         assignment.status === 'submitted' ? '已提交' :
                         assignment.status === 'graded' ? '已评分' : assignment.status }}
                    </span>
                  </div>
                </div>
                <div v-if="assignment.score !== undefined" class="text-right">
                  <p class="text-2xl font-bold text-[var(--primary)]">{{ assignment.score }}</p>
                  <p class="text-xs text-[var(--text-muted)]">/ {{ assignment.pointsPossible }}</p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 同步错误提示 -->
        <div v-if="syncError" class="p-4 bg-red-50 border border-red-200 rounded-lg">
          <p class="text-red-700">
            <span class="font-medium">⚠️ 同步错误：</span>
            {{ syncError }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>