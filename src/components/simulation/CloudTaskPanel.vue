<template>
  <div class="space-y-4">
    <!-- 云端状态 -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3 flex items-center gap-2">
        <span>☁️</span>
        <span>云端仿真</span>
        <span 
          v-if="cloudConnected" 
          class="ml-auto w-2 h-2 rounded-full bg-green-500"
          title="已连接云端服务"
        ></span>
        <span 
          v-else 
          class="ml-auto w-2 h-2 rounded-full bg-gray-400"
          title="未连接云端服务"
        ></span>
      </h4>
      
      <div class="bg-[var(--bg-elevated)] rounded-lg p-3 space-y-2 text-xs">
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">连接状态</span>
          <span :class="cloudConnected ? 'text-green-600' : 'text-gray-500'">
            {{ cloudConnected ? '已连接' : '未连接' }}
          </span>
        </div>
        <div v-if="cloudConnected" class="flex justify-between">
          <span class="text-[var(--text-muted)]">并发任务数</span>
          <span class="text-[var(--text-primary)]">{{ activeTaskCount }}/{{ maxConcurrentTasks }}</span>
        </div>
      </div>
    </div>

    <!-- 任务列表 -->
    <div class="panel-section">
      <div class="flex justify-between items-center mb-3">
        <h4 class="text-xs font-medium text-[var(--text-secondary)] flex items-center gap-2">
          <span>📋</span>
          <span>云端任务 ({{ cloudTasks.length }})</span>
        </h4>
        <button 
          @click="refreshTasks"
          class="text-[var(--text-muted)] hover:text-[var(--text-primary)] transition p-1"
          title="刷新任务列表"
        >
          <span class="text-xs">⟳</span>
        </button>
      </div>

      <!-- 任务为空 -->
      <div v-if="cloudTasks.length === 0" class="text-center py-4 text-[var(--text-muted)] text-xs">
        暂无云端任务
      </div>

      <!-- 任务列表 -->
      <div v-else class="space-y-2 max-h-64 overflow-y-auto">
        <div 
          v-for="task in cloudTasks" 
          :key="task.id"
          class="bg-[var(--bg-elevated)] rounded-lg p-2 cursor-pointer hover:bg-[var(--bg-hover)] transition"
          :class="{ 'ring-1 ring-blue-400': selectedTaskId === task.id }"
          @click="selectTask(task)"
        >
          <!-- 任务名称和状态 -->
          <div class="flex justify-between items-center mb-1">
            <span class="text-xs font-medium text-[var(--text-primary)] truncate max-w-[120px]">
              {{ task.name }}
            </span>
            <span 
              class="text-[10px] px-1.5 py-0.5 rounded"
              :class="statusClasses[task.status]"
            >
              {{ statusLabels[task.status] }}
            </span>
          </div>

          <!-- 进度条 -->
          <div v-if="task.status === 'running' || task.status === 'queued'" class="mb-1">
            <div class="h-1.5 bg-gray-200 rounded-full overflow-hidden">
              <div 
                class="h-full bg-blue-500 transition-all duration-300"
                :style="{ width: task.progress + '%' }"
              ></div>
            </div>
            <div class="text-[10px] text-[var(--text-muted)] text-right mt-0.5">
              {{ task.progress }}%
            </div>
          </div>

          <!-- 操作按钮 -->
          <div class="flex gap-1 mt-1">
            <button 
              v-if="task.status === 'running' || task.status === 'queued'"
              @click.stop="cancelTask(task.id)"
              class="text-[10px] px-2 py-0.5 bg-red-100 text-red-600 rounded hover:bg-red-200 transition"
            >
              取消
            </button>
            <button 
              v-if="task.status === 'completed'"
              @click.stop="viewResult(task.id)"
              class="text-[10px] px-2 py-0.5 bg-green-100 text-green-600 rounded hover:bg-green-200 transition"
            >
              查看结果
            </button>
            <button 
              @click.stop="deleteTask(task.id)"
              class="text-[10px] px-2 py-0.5 bg-gray-100 text-gray-600 rounded hover:bg-gray-200 transition"
            >
              删除
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 选中任务详情 -->
    <div v-if="selectedTask" class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3 flex items-center gap-2">
        <span>📊</span>
        <span>任务详情</span>
      </h4>
      
      <div class="bg-[var(--bg-elevated)] rounded-lg p-3 space-y-2 text-xs">
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">任务ID</span>
          <span class="text-[var(--text-primary)] font-mono text-[10px]">{{ selectedTask.id.slice(0, 12) }}...</span>
        </div>
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">创建时间</span>
          <span class="text-[var(--text-primary)]">{{ formatTime(selectedTask.createdAt) }}</span>
        </div>
        <div v-if="selectedTask.startedAt" class="flex justify-between">
          <span class="text-[var(--text-muted)]">开始时间</span>
          <span class="text-[var(--text-primary)]">{{ formatTime(selectedTask.startedAt) }}</span>
        </div>
        <div v-if="selectedTask.completedAt" class="flex justify-between">
          <span class="text-[var(--text-muted)]">完成时间</span>
          <span class="text-[var(--text-primary)]">{{ formatTime(selectedTask.completedAt) }}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">耗时</span>
          <span class="text-[var(--text-primary)]">{{ getDuration(selectedTask) }}</span>
        </div>
      </div>

      <!-- 错误信息 -->
      <div v-if="selectedTask.error" class="mt-2 bg-red-50 rounded-lg p-2 text-xs text-red-600">
        <span class="font-medium">错误:</span> {{ selectedTask.error }}
      </div>
    </div>

    <!-- 日志流 -->
    <div v-if="selectedTask && (selectedTask.status === 'running' || selectedTask.logs.length > 0)" class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3 flex items-center gap-2">
        <span>📜</span>
        <span>运行日志</span>
        <span v-if="selectedTask.status === 'running'" class="ml-auto animate-pulse text-[10px] text-green-600">
          ● 实时更新中
        </span>
      </h4>
      
      <div class="bg-[var(--bg-elevated)] rounded-lg p-3 max-h-40 overflow-y-auto font-mono text-[10px] leading-relaxed">
        <div v-if="selectedTask.logs.length === 0" class="text-[var(--text-muted)]">
          暂无日志输出
        </div>
        <div v-else class="space-y-0.5">
          <div 
            v-for="(log, idx) in selectedTask.logs" 
            :key="idx"
            class="text-[var(--text-secondary)]"
          >
            {{ log }}
          </div>
        </div>
      </div>
    </div>

    <!-- 测试连接 -->
    <div class="panel-section">
      <button 
        @click="testConnection"
        :disabled="testingConnection"
        class="w-full px-3 py-2 text-xs border border-blue-300 text-blue-600 rounded hover:bg-blue-50 transition disabled:opacity-50"
      >
        {{ testingConnection ? '测试中...' : '测试云端连接' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { listCloudTasks, getCloudTaskStatus, cancelCloudTask, deleteCloudTask, testCloudConnection } from '@/api/cloud-simulation'
import type { CloudTask } from '@/api/cloud-simulation'

// Props
const props = defineProps<{
  projectId?: string
}>()

// Emits
const emit = defineEmits<{
  (e: 'taskCompleted', taskId: string, result: any): void
  (e: 'taskFailed', taskId: string, error: string): void
  (e: 'viewResult', taskId: string): void
}>()

// 状态
const cloudTasks = ref<CloudTask[]>([])
const cloudConnected = ref(false)
const maxConcurrentTasks = ref(3)
const activeTaskCount = ref(0)
const selectedTaskId = ref<string | null>(null)
const testingConnection = ref(false)

// 轮询定时器
let pollingInterval: number | null = null

// 状态标签和样式
const statusLabels: Record<string, string> = {
  pending: '待处理',
  queued: '排队中',
  running: '运行中',
  completed: '已完成',
  failed: '失败',
  cancelled: '已取消'
}

const statusClasses: Record<string, string> = {
  pending: 'bg-gray-100 text-gray-600',
  queued: 'bg-yellow-100 text-yellow-700',
  running: 'bg-blue-100 text-blue-600',
  completed: 'bg-green-100 text-green-600',
  failed: 'bg-red-100 text-red-600',
  cancelled: 'bg-gray-100 text-gray-400'
}

// 计算属性
const selectedTask = computed(() => {
  return cloudTasks.value.find(t => t.id === selectedTaskId.value) || null
})

// 方法
async function refreshTasks() {
  try {
    const tasks = await listCloudTasks(props.projectId)
    cloudTasks.value = tasks
    
    // 更新活动任务数
    activeTaskCount.value = tasks.filter(
      t => t.status === 'running' || t.status === 'queued'
    ).length
  } catch (e) {
    console.error('刷新任务列表失败:', e)
  }
}

function selectTask(task: CloudTask) {
  selectedTaskId.value = task.id
}

async function cancelTask(taskId: string) {
  try {
    await cancelCloudTask(taskId)
    await refreshTasks()
  } catch (e) {
    console.error('取消任务失败:', e)
  }
}

async function deleteTask(taskId: string) {
  try {
    await deleteCloudTask(taskId)
    if (selectedTaskId.value === taskId) {
      selectedTaskId.value = null
    }
    await refreshTasks()
  } catch (e) {
    console.error('删除任务失败:', e)
  }
}

function viewResult(taskId: string) {
  emit('viewResult', taskId)
}

async function testConnection() {
  testingConnection.value = true
  try {
    const result = await testCloudConnection()
    cloudConnected.value = result.connected
    if (!result.connected) {
      console.warn('云端连接测试失败:', result.message)
    }
  } catch (e) {
    cloudConnected.value = false
    console.error('云端连接测试失败:', e)
  } finally {
    testingConnection.value = false
  }
}

function formatTime(isoString: string): string {
  try {
    const date = new Date(isoString)
    return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit', second: '2-digit' })
  } catch {
    return isoString
  }
}

function getDuration(task: CloudTask): string {
  const start = task.startedAt ? new Date(task.startedAt).getTime() : null
  const end = task.completedAt ? new Date(task.completedAt).getTime() : Date.now()
  
  if (!start) return '-'
  
  const durationMs = end - start
  const seconds = Math.floor(durationMs / 1000)
  
  if (seconds < 60) return `${seconds}秒`
  const minutes = Math.floor(seconds / 60)
  const remainingSeconds = seconds % 60
  return `${minutes}分${remainingSeconds}秒`
}

// 开始轮询活动任务
function startPolling() {
  if (pollingInterval) return
  
  pollingInterval = window.setInterval(async () => {
    // 只轮询活动任务
    const activeTasks = cloudTasks.value.filter(
      t => t.status === 'running' || t.status === 'queued' || t.status === 'pending'
    )
    
    for (const task of activeTasks) {
      try {
        const updatedTask = await getCloudTaskStatus(task.id)
        
        // 更新任务列表中的对应任务
        const index = cloudTasks.value.findIndex(t => t.id === task.id)
        if (index !== -1) {
          const oldStatus = cloudTasks.value[index].status
          cloudTasks.value[index] = updatedTask
          
          // 检测状态变化
          if (oldStatus !== updatedTask.status) {
            if (updatedTask.status === 'completed') {
              emit('taskCompleted', task.id, null)
            } else if (updatedTask.status === 'failed') {
              emit('taskFailed', task.id, updatedTask.error || '未知错误')
            }
          }
        }
      } catch (e) {
        console.error('获取任务状态失败:', e)
      }
    }
    
    // 更新活动任务数
    activeTaskCount.value = cloudTasks.value.filter(
      t => t.status === 'running' || t.status === 'queued'
    ).length
    
    // 如果没有活动任务，停止轮询
    if (activeTasks.length === 0 && pollingInterval) {
      clearInterval(pollingInterval)
      pollingInterval = null
    }
  }, 3000) // 每3秒轮询一次
}

// 停止轮询
function stopPolling() {
  if (pollingInterval) {
    clearInterval(pollingInterval)
    pollingInterval = null
  }
}

// 监听活动任务数变化，动态控制轮询
watch(activeTaskCount, (count) => {
  if (count > 0) {
    startPolling()
  } else {
    stopPolling()
  }
})

// 生命周期
onMounted(async () => {
  await testConnection()
  await refreshTasks()
  
  // 如果有活动任务，开始轮询
  if (activeTaskCount.value > 0) {
    startPolling()
  }
})

onUnmounted(() => {
  stopPolling()
})

// 暴露方法给父组件
defineExpose({
  refreshTasks,
  testConnection
})
</script>

<style scoped>
.panel-section {
  padding-bottom: 16px;
  border-bottom: 1px solid var(--border-subtle);
}

.panel-section:last-child {
  border-bottom: none;
  padding-bottom: 0;
}

/* 日志滚动条样式 */
.max-h-40::-webkit-scrollbar {
  width: 4px;
}

.max-h-40::-webkit-scrollbar-thumb {
  background: var(--border-subtle);
  border-radius: 2px;
}

.max-h-40::-webkit-scrollbar-track {
  background: transparent;
}

/* 任务列表滚动条 */
.max-h-64::-webkit-scrollbar {
  width: 4px;
}

.max-h-64::-webkit-scrollbar-thumb {
  background: var(--border-subtle);
  border-radius: 2px;
}
</style>