<template>
  <div class="remote-solver-panel p-4 space-y-4">
    <!-- 服务器配置 -->
    <div class="space-y-3">
      <h4 class="text-sm font-semibold text-gray-700 dark:text-gray-300">远程求解服务器</h4>

      <div class="space-y-2">
        <label class="block text-xs text-gray-500 dark:text-gray-400">服务器地址</label>
        <input
          v-model="serverUrl"
          type="url"
          placeholder="https://solver.example.com"
          class="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 focus:ring-1 focus:ring-blue-500 focus:border-blue-500 outline-none"
        />
      </div>

      <div class="space-y-2">
        <label class="block text-xs text-gray-500 dark:text-gray-400">认证 Token（可选）</label>
        <input
          v-model="authToken"
          type="password"
          placeholder="Bearer token"
          class="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 focus:ring-1 focus:ring-blue-500 focus:border-blue-500 outline-none"
        />
      </div>

      <button
        @click="testConnection"
        :disabled="isTestingConnection || !serverUrl"
        class="px-3 py-1.5 text-xs border border-gray-300 dark:border-gray-600 rounded hover:bg-gray-100 dark:hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {{ isTestingConnection ? '测试中...' : '测试连接' }}
      </button>
      <span v-if="connectionStatus" class="text-xs ml-2" :class="connectionStatusClass">
        {{ connectionStatus }}
      </span>
    </div>

    <div class="border-t border-gray-200 dark:border-gray-700"></div>

    <!-- 任务提交 -->
    <div class="space-y-3">
      <h4 class="text-sm font-semibold text-gray-700 dark:text-gray-300">提交仿真任务</h4>

      <div class="space-y-2">
        <label class="block text-xs text-gray-500 dark:text-gray-400">项目名称</label>
        <input
          v-model="submitForm.projectName"
          type="text"
          placeholder="输入项目名称"
          class="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 focus:ring-1 focus:ring-blue-500 focus:border-blue-500 outline-none"
        />
      </div>

      <div class="space-y-2">
        <label class="block text-xs text-gray-500 dark:text-gray-400">分析类型</label>
        <select
          v-model="submitForm.analysisType"
          class="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 focus:ring-1 focus:ring-blue-500 focus:border-blue-500 outline-none"
        >
          <option value="static">静力学分析</option>
          <option value="modal">模态分析</option>
          <option value="thermal">热分析</option>
          <option value="transient">瞬态分析</option>
          <option value="buckling">屈曲分析</option>
        </select>
      </div>

      <button
        @click="submitJob"
        :disabled="isSubmitting || !canSubmit"
        class="w-full px-4 py-2 text-sm bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {{ isSubmitting ? '提交中...' : '提交到远程服务器' }}
      </button>
      <p v-if="submitError" class="text-xs text-red-500 mt-1">{{ submitError }}</p>
    </div>

    <div class="border-t border-gray-200 dark:border-gray-700"></div>

    <!-- 任务列表 -->
    <div class="space-y-3">
      <div class="flex justify-between items-center">
        <h4 class="text-sm font-semibold text-gray-700 dark:text-gray-300">任务列表</h4>
        <button
          v-if="jobs.length > 0"
          @click="refreshJobs"
          :disabled="isRefreshing"
          class="text-xs text-blue-500 hover:text-blue-600 disabled:opacity-50"
        >
          {{ isRefreshing ? '刷新中...' : '刷新' }}
        </button>
      </div>

      <div v-if="jobs.length === 0" class="text-xs text-gray-400 dark:text-gray-500 text-center py-4">
        暂无远程任务
      </div>

      <div v-else class="space-y-2 max-h-[300px] overflow-y-auto">
        <div
          v-for="job in jobs"
          :key="job.id"
          class="p-3 border border-gray-200 dark:border-gray-600 rounded-lg space-y-2"
        >
          <!-- 任务头部 -->
          <div class="flex justify-between items-start">
            <div>
              <p class="text-sm font-medium text-gray-800 dark:text-gray-200 truncate max-w-[200px]">
                {{ job.id }}
              </p>
              <p class="text-xs text-gray-400 dark:text-gray-500">
                {{ formatTime(job.submittedAt) }}
              </p>
            </div>
            <span
              class="text-xs px-2 py-0.5 rounded-full font-medium"
              :class="statusBadgeClass(job.status)"
            >
              {{ statusText(job.status) }}
            </span>
          </div>

          <!-- 进度条 -->
          <div v-if="job.status === 'running' || job.status === 'queued'" class="space-y-1">
            <div class="w-full h-1.5 bg-gray-200 dark:bg-gray-600 rounded-full overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-500"
                :class="job.status === 'running' ? 'bg-blue-500' : 'bg-gray-400'"
                :style="{ width: job.progress + '%' }"
              ></div>
            </div>
            <span class="text-xs text-gray-500">{{ job.progress }}%</span>
          </div>

          <!-- 错误信息 -->
          <p v-if="job.error" class="text-xs text-red-500">{{ job.error }}</p>

          <!-- 结果摘要 -->
          <div v-if="job.result" class="text-xs text-gray-600 dark:text-gray-400 space-y-0.5">
            <p>最大位移: {{ job.result.maxDisplacement.toFixed(4) }} mm</p>
            <p>最大应力: {{ job.result.maxStress.toFixed(2) }} MPa</p>
          </div>

          <!-- 操作按钮 -->
          <div class="flex gap-2">
            <button
              v-if="job.status === 'running' || job.status === 'queued'"
              @click="cancelJob(job.id)"
              class="px-2 py-1 text-xs border border-red-300 text-red-500 rounded hover:bg-red-50 dark:hover:bg-red-900/20"
            >
              取消
            </button>
            <button
              v-if="job.status === 'completed' && job.result?.resultFileUrl"
              @click="downloadResult(job)"
              :disabled="isDownloading === job.id"
              class="px-2 py-1 text-xs border border-blue-300 text-blue-500 rounded hover:bg-blue-50 dark:hover:bg-blue-900/20"
            >
              {{ isDownloading === job.id ? '下载中...' : '下载结果' }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive, onMounted } from 'vue'
import {
  type RemoteSolverConfig,
  type RemoteJob,
  type SubmitJobRequest,
  submitRemoteJob,
  getRemoteJobStatus,
  cancelRemoteJob,
  downloadRemoteResult,
  pollJobUntilComplete,
} from '@/api/remoteSolver'

// 服务器配置
const serverUrl = ref('https://solver.example.com')
const authToken = ref('')
const isTestingConnection = ref(false)
const connectionStatus = ref('')
const connectionSuccess = ref(false)

// 提交表单
const submitForm = reactive({
  projectName: '',
  analysisType: 'static',
})
const isSubmitting = ref(false)
const submitError = ref('')

// 任务列表
const jobs = ref<RemoteJob[]>([])
const isRefreshing = ref(false)
const isDownloading = ref<string | null>(null)

// 获取求解器配置
const solverConfig = computed<RemoteSolverConfig>(() => ({
  serverUrl: serverUrl.value,
  authToken: authToken.value || undefined,
}))

const canSubmit = computed(() => {
  return serverUrl.value && submitForm.projectName.trim()
})

const connectionStatusClass = computed(() => ({
  'text-green-500': connectionSuccess.value,
  'text-red-500': !connectionSuccess.value && connectionStatus.value,
}))

/**
 * 测试服务器连接
 */
async function testConnection() {
  isTestingConnection.value = true
  connectionStatus.value = ''
  connectionSuccess.value = false

  try {
    const response = await fetch(`${serverUrl.value}/api/solver/health`, {
      signal: AbortSignal.timeout(5000),
    })
    if (response.ok) {
      connectionStatus.value = '连接成功'
      connectionSuccess.value = true
    } else {
      connectionStatus.value = `服务器返回 ${response.status}`
      connectionSuccess.value = false
    }
  } catch (err) {
    connectionStatus.value = '连接失败'
    connectionSuccess.value = false
  } finally {
    isTestingConnection.value = false
  }
}

/**
 * 提交仿真任务
 */
async function submitJob() {
  if (!canSubmit.value) return

  isSubmitting.value = true
  submitError.value = ''

  // 构建提交请求（使用模拟数据作为示例）
  const request: SubmitJobRequest = {
    projectName: submitForm.projectName,
    analysisType: submitForm.analysisType,
    meshData: {
      nodes: [],
      elements: [],
      element_type: 'TET4',
    },
    material: {
      name: 'Steel',
      elastic_modulus: 210000,
      poisson_ratio: 0.3,
      density: 7.85e-6,
    },
    boundaryConditions: [],
  }

  try {
    const job = await submitRemoteJob(solverConfig.value, request)
    jobs.value.unshift(job)

    // 开始轮询任务状态
    pollJob(job.id)

    // 重置表单
    submitForm.projectName = ''
  } catch (err: any) {
    submitError.value = err.message || '提交失败'
  } finally {
    isSubmitting.value = false
  }
}

/**
 * 轮询单个任务状态
 */
function pollJob(jobId: string) {
  pollJobUntilComplete(
    solverConfig.value,
    jobId,
    (updatedJob) => {
      const index = jobs.value.findIndex(j => j.id === jobId)
      if (index !== -1) {
        jobs.value[index] = updatedJob
      }
    },
    3000,  // 每 3 秒轮询一次
    1800  // 最多轮询 1 小时
  ).catch(() => {
    // 轮询失败不影响 UI
  })
}

/**
 * 刷新所有任务状态
 */
async function refreshJobs() {
  isRefreshing.value = true
  try {
    const updatedJobs = await Promise.all(
      jobs.value
        .filter(j => j.status === 'queued' || j.status === 'running')
        .map(j => getRemoteJobStatus(solverConfig.value, j.id))
    )

    for (const updatedJob of updatedJobs) {
      const index = jobs.value.findIndex(j => j.id === updatedJob.id)
      if (index !== -1) {
        jobs.value[index] = updatedJob
      }
    }
  } catch {
    // 刷新失败静默处理
  } finally {
    isRefreshing.value = false
  }
}

/**
 * 取消任务
 */
async function cancelJob(jobId: string) {
  try {
    await cancelRemoteJob(solverConfig.value, jobId)
    const index = jobs.value.findIndex(j => j.id === jobId)
    if (index !== -1) {
      jobs.value[index].status = 'cancelled'
    }
  } catch (err: any) {
    console.error('取消任务失败:', err)
  }
}

/**
 * 下载任务结果
 */
async function downloadResult(job: RemoteJob) {
  if (!job.result?.resultFileUrl) return

  isDownloading.value = job.id
  try {
    const blob = await downloadRemoteResult(solverConfig.value, job.result.resultFileUrl)
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `result-${job.id}.vtk`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  } catch (err: any) {
    console.error('下载结果失败:', err)
  } finally {
    isDownloading.value = null
  }
}

/**
 * 格式化时间
 */
function formatTime(isoString: string): string {
  try {
    const date = new Date(isoString)
    return date.toLocaleString('zh-CN', {
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      hour12: false,
    })
  } catch {
    return isoString
  }
}

/**
 * 获取状态文本
 */
function statusText(status: RemoteJob['status']): string {
  const map: Record<RemoteJob['status'], string> = {
    queued: '排队中',
    running: '求解中',
    completed: '已完成',
    failed: '失败',
    cancelled: '已取消',
  }
  return map[status]
}

/**
 * 获取状态徽章样式
 */
function statusBadgeClass(status: RemoteJob['status']): string {
  const map: Record<RemoteJob['status'], string> = {
    queued: 'bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-300',
    running: 'bg-blue-100 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400',
    completed: 'bg-green-100 text-green-600 dark:bg-green-900/30 dark:text-green-400',
    failed: 'bg-red-100 text-red-600 dark:bg-red-900/30 dark:text-red-400',
    cancelled: 'bg-gray-100 text-gray-500 dark:bg-gray-700 dark:text-gray-400',
  }
  return map[status]
}

onMounted(() => {
  // 从 localStorage 恢复服务器配置
  const savedUrl = localStorage.getItem('caelab-remote-solver-url')
  if (savedUrl) serverUrl.value = savedUrl
  const savedToken = localStorage.getItem('caelab-remote-solver-token')
  if (savedToken) authToken.value = savedToken
})
</script>
