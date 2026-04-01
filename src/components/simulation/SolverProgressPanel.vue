<template>
  <div class="solver-progress-panel">
    <!-- 进度条 -->
    <div v-if="isRunning" class="mb-3">
      <div class="flex justify-between text-xs mb-1">
        <span class="text-[var(--text-secondary)]">{{ statusMessage }}</span>
        <span class="text-[var(--text-secondary)]">{{ Math.round(progress) }}%</span>
      </div>
      <div class="h-2 bg-[var(--bg-elevated)] rounded-full overflow-hidden">
        <div
          class="h-full bg-[var(--primary)] transition-all duration-300 rounded-full"
          :style="{ width: progress + '%' }"
        ></div>
      </div>
      <div v-if="eta" class="text-[10px] text-[var(--text-tertiary)] mt-1">
        预计剩余时间: {{ formatEta(eta) }}
      </div>
    </div>

    <!-- 已完成状态 -->
    <div v-if="isCompleted" class="mb-3">
      <div class="flex justify-between text-xs mb-1">
        <span class="text-green-600">求解完成</span>
        <span class="text-green-600">100%</span>
      </div>
      <div class="h-2 bg-[var(--bg-elevated)] rounded-full overflow-hidden">
        <div class="h-full bg-green-500 rounded-full" style="width: 100%"></div>
      </div>
      <div class="text-[10px] text-[var(--text-tertiary)] mt-1">
        耗时: {{ formatDuration(elapsedTime) }}
      </div>
    </div>

    <!-- 已取消状态 -->
    <div v-if="isCancelled" class="mb-3">
      <div class="flex justify-between text-xs mb-1">
        <span class="text-orange-500">求解已取消</span>
        <span class="text-orange-500">{{ Math.round(progress) }}%</span>
      </div>
      <div class="h-2 bg-[var(--bg-elevated)] rounded-full overflow-hidden">
        <div
          class="h-full bg-orange-400 rounded-full"
          :style="{ width: progress + '%' }"
        ></div>
      </div>
    </div>

    <!-- 错误状态 -->
    <div v-if="errorMessage" class="mb-3">
      <div class="text-xs text-red-500 bg-red-50 rounded p-2">
        {{ errorMessage }}
      </div>
    </div>

    <!-- 取消按钮 -->
    <button
      v-if="isRunning"
      @click="doCancel"
      class="w-full px-3 py-1.5 bg-red-500 text-white rounded text-xs hover:bg-red-600 transition"
    >
      取消求解
    </button>

    <!-- 日志区域 -->
    <div v-if="logs.length > 0" class="mt-3">
      <div class="flex items-center justify-between mb-1">
        <span class="text-xs text-[var(--text-secondary)]">
          计算日志 ({{ filteredLogs.length }}/{{ logs.length }})
        </span>
        <div class="flex gap-1">
          <button
            v-for="lvl in (['all', 'warning', 'error'] as const)"
            :key="lvl"
            @click="logFilter = lvl as 'all' | 'warning' | 'error'"
            :class="[
              'text-[10px] px-1.5 py-0.5 rounded transition',
              logFilter === lvl
                ? 'bg-[var(--primary)] text-white'
                : 'text-[var(--text-secondary)] hover:bg-gray-100'
            ]"
          >
            {{ lvl === 'all' ? '全部' : lvl === 'warning' ? '警告' : '错误' }}
          </button>
        </div>
      </div>
      <div
        ref="logContainerRef"
        class="bg-[var(--bg-elevated)] rounded-lg p-2 max-h-40 overflow-y-auto font-mono text-[10px] space-y-0.5"
      >
        <div
          v-for="(log, idx) in filteredLogs"
          :key="idx"
          :class="['px-1 py-0.5 rounded', logClass(log.level)]"
        >
          <span class="text-[var(--text-tertiary)]">[{{ log.level }}]</span>
          {{ log.message }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onUnmounted } from 'vue'
import { onSolverEvent, cancelSolver, type SolverEvent } from '@/api/cae'

// ============ 状态 ============

const progress = ref(0)
const statusMessage = ref('准备中...')
const eta = ref<number | null>(null)
const elapsedTime = ref(0)
const isRunning = ref(false)
const isCompleted = ref(false)
const isCancelled = ref(false)
const errorMessage = ref<string | null>(null)
const logs = ref<Array<{ level: string; message: string }>>([])
const logFilter = ref<'all' | 'warning' | 'error'>('all')
const logContainerRef = ref<HTMLElement | null>(null)

// 用于计算 ETA
let startTime: number | null = null
let unlisten: (() => void) | null = null

// ============ 计算属性 ============

const filteredLogs = computed(() => {
  if (logFilter.value === 'all') return logs.value
  return logs.value.filter((log) => log.level === logFilter.value)
})

// ============ 方法 ============

/** 格式化 ETA 秒数为可读字符串 */
function formatEta(seconds: number): string {
  if (seconds < 60) {
    return `${Math.round(seconds)}秒`
  }
  const minutes = Math.floor(seconds / 60)
  const secs = Math.round(seconds % 60)
  return `${minutes}分${secs}秒`
}

/** 格式化已耗时 */
function formatDuration(seconds: number): string {
  if (seconds < 60) {
    return `${seconds.toFixed(1)}秒`
  }
  const minutes = Math.floor(seconds / 60)
  const secs = Math.round(seconds % 60)
  return `${minutes}分${secs}秒`
}

/** 日志行的 CSS 类 */
function logClass(level: string): string {
  switch (level) {
    case 'error':
      return 'bg-red-50 text-red-600'
    case 'warning':
      return 'bg-yellow-50 text-yellow-700'
    default:
      return 'text-[var(--text-secondary)]'
  }
}

/** 自动滚动日志到底部 */
function scrollToBottom() {
  nextTick(() => {
    if (logContainerRef.value) {
      logContainerRef.value.scrollTop = logContainerRef.value.scrollHeight
    }
  })
}

/** 处理求解器事件 */
function handleSolverEvent(event: SolverEvent) {
  switch (event.type) {
    case 'Progress':
      if (event.percent !== undefined) {
        progress.value = event.percent
      }
      if (event.message) {
        statusMessage.value = event.message
      }
      // 更新 ETA
      if (startTime && progress.value > 0) {
        const elapsed = (Date.now() - startTime) / 1000
        const estimatedTotal = elapsed / (progress.value / 100)
        eta.value = Math.max(0, estimatedTotal - elapsed)
      }
      break

    case 'Log':
      if (event.level && event.message) {
        logs.value.push({
          level: event.level,
          message: event.message,
        })
        // 限制日志数量，避免内存溢出
        if (logs.value.length > 5000) {
          logs.value = logs.value.slice(-3000)
        }
        scrollToBottom()
      }
      break

    case 'Completed':
      isRunning.value = false
      isCompleted.value = true
      progress.value = 100
      statusMessage.value = '求解完成'
      eta.value = null
      if (event.elapsed_time_seconds !== undefined) {
        elapsedTime.value = event.elapsed_time_seconds
      }
      break

    case 'Cancelled':
      isRunning.value = false
      isCancelled.value = true
      statusMessage.value = '求解已取消'
      eta.value = null
      break

    case 'Error':
      isRunning.value = false
      errorMessage.value = event.message || '未知错误'
      statusMessage.value = '求解出错'
      eta.value = null
      break
  }
}

/** 启动求解（带进度） */
async function start(inputPath: string, workingDir: string, numThreads?: number) {
  // 重置状态
  resetState()

  isRunning.value = true
  startTime = Date.now()

  // 注册事件监听
  unlisten = await onSolverEvent(handleSolverEvent)

  try {
    await cancelSolver() // 先重置取消标志
    const result = await import('@/api/cae').then((m) =>
      m.runSolverWithProgress(inputPath, workingDir, numThreads)
    )
    return result
  } catch (e: any) {
    if (e === 'SOLVER_CANCELLED') {
      isRunning.value = false
      isCancelled.value = true
      statusMessage.value = '求解已取消'
      return null
    }
    throw e
  } finally {
    isRunning.value = false
    if (startTime) {
      elapsedTime.value = (Date.now() - startTime) / 1000
    }
  }
}

/** 取消求解 */
async function doCancel() {
  try {
    await cancelSolver()
    statusMessage.value = '正在取消...'
  } catch (e) {
    console.error('Failed to cancel solver:', e)
  }
}

/** 停止并清理 */
function stop() {
  if (unlisten) {
    unlisten()
    unlisten = null
  }
}

/** 重置状态 */
function resetState() {
  progress.value = 0
  statusMessage.value = '准备中...'
  eta.value = null
  elapsedTime.value = 0
  isRunning.value = false
  isCompleted.value = false
  isCancelled.value = false
  errorMessage.value = null
  logs.value = []
  logFilter.value = 'all'
  startTime = null
}

// 组件卸载时清理
onUnmounted(() => {
  stop()
})

// 暴露方法给父组件
defineExpose({
  start,
  stop,
  doCancel,
  resetState,
  isRunning,
  isCompleted,
  isCancelled,
  progress,
  statusMessage,
  logs,
})
</script>
