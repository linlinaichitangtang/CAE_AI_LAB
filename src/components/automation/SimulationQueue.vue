<template>
  <div class="simulation-queue">
    <!-- 队列统计栏 -->
    <div class="queue-stats-bar">
      <div class="stat-item">
        <span class="stat-label">总计</span>
        <span class="stat-value">{{ stats.total }}</span>
      </div>
      <div class="stat-item stat-pending">
        <span class="stat-label">等待</span>
        <span class="stat-value">{{ stats.pending }}</span>
      </div>
      <div class="stat-item stat-running">
        <span class="stat-label">运行</span>
        <span class="stat-value">{{ stats.running }}</span>
      </div>
      <div class="stat-item stat-completed">
        <span class="stat-label">完成</span>
        <span class="stat-value">{{ stats.completed }}</span>
      </div>
      <div class="stat-item stat-failed">
        <span class="stat-label">失败</span>
        <span class="stat-value">{{ stats.failed }}</span>
      </div>
      <div class="stat-item stat-eta">
        <span class="stat-label">预计剩余</span>
        <span class="stat-value">{{ formatTime(stats.estimatedTimeRemaining) }}</span>
      </div>
    </div>

    <!-- 操作栏 -->
    <div class="queue-actions">
      <button
        v-if="!store.isQueueRunning"
        @click="startQueue"
        class="action-btn action-start"
        :disabled="stats.pending === 0"
      >
        <span class="action-icon">&#9654;</span>
        <span>开始队列</span>
      </button>
      <button
        v-else
        @click="pauseQueue"
        class="action-btn action-pause"
      >
        <span class="action-icon">&#10074;&#10074;</span>
        <span>暂停</span>
      </button>
      <button
        @click="stopQueue"
        class="action-btn action-stop"
        :disabled="!store.isQueueRunning && stats.running === 0"
      >
        <span class="action-icon">&#9632;</span>
        <span>停止</span>
      </button>
      <button
        @click="clearCompleted"
        class="action-btn action-clear"
        :disabled="stats.completed + stats.cancelled === 0"
      >
        <span class="action-icon">&#10006;</span>
        <span>清除已完成</span>
      </button>
      <button
        @click="addDemoJobs"
        class="action-btn action-add"
      >
        <span class="action-icon">+</span>
        <span>添加示例</span>
      </button>
      <div class="spacer"></div>
      <button
        @click="store.requestNotificationPermission()"
        class="action-btn action-notify"
        title="启用桌面通知"
      >
        <span class="action-icon">&#128276;</span>
      </button>
    </div>

    <!-- 队列列表 -->
    <div class="queue-list" @dragover.prevent="onDragOver" @drop="onDrop">
      <div v-if="store.simulationQueue.length === 0" class="queue-empty">
        <div class="empty-icon">&#128203;</div>
        <p>仿真队列为空</p>
        <p class="empty-hint">点击 "添加示例" 或从脚本面板添加任务</p>
      </div>

      <div
        v-for="(job, index) in store.simulationQueue"
        :key="job.id"
        class="queue-item"
        :class="{
          'is-running': job.status === 'running',
          'is-completed': job.status === 'completed',
          'is-failed': job.status === 'failed',
          'is-cancelled': job.status === 'cancelled',
          'is-dragging': draggedJobId === job.id,
        }"
        draggable="true"
        @dragstart="onDragStart($event, job.id)"
        @dragend="onDragEnd"
        @dragover.prevent="onItemDragOver($event, index)"
        @drop.stop="onItemDrop($event, index)"
      >
        <!-- 拖拽手柄 -->
        <div class="drag-handle" title="拖拽排序">&#9776;</div>

        <!-- 状态图标 -->
        <div class="job-status-icon" :class="`status-${job.status}`">
          <span v-if="job.status === 'pending'">&#9203;</span>
          <span v-else-if="job.status === 'running'" class="spin">&#9881;</span>
          <span v-else-if="job.status === 'completed'">&#10003;</span>
          <span v-else-if="job.status === 'failed'">&#10007;</span>
          <span v-else-if="job.status === 'cancelled'">&#8722;</span>
        </div>

        <!-- 任务信息 -->
        <div class="job-info">
          <div class="job-name">{{ job.name }}</div>
          <div class="job-meta">
            <span class="job-project">ID: {{ job.projectId.slice(0, 8) }}</span>
            <span v-if="job.startTime && job.endTime" class="job-duration">
              {{ formatDuration(job.endTime - job.startTime) }}
            </span>
            <span v-if="job.status === 'running' && job.startTime" class="job-elapsed">
              {{ formatDuration(Date.now() - job.startTime) }}
            </span>
          </div>
          <!-- 进度条 -->
          <div v-if="job.status === 'running'" class="progress-bar">
            <div
              class="progress-fill"
              :style="{ width: job.progress + '%' }"
            ></div>
            <span class="progress-text">{{ job.progress }}%</span>
          </div>
          <!-- 错误信息 -->
          <div v-if="job.error" class="job-error">
            {{ job.error }}
          </div>
        </div>

        <!-- ETA -->
        <div v-if="job.status === 'running'" class="job-eta">
          <span class="eta-label">ETA</span>
          <span class="eta-value">{{ formatTime(job.estimatedTime ?? 0) }}</span>
        </div>

        <!-- 操作按钮 -->
        <div class="job-actions">
          <button
            v-if="job.status === 'pending'"
            @click="store.moveJobUp(job.id)"
            class="job-action-btn"
            title="上移"
            :disabled="index === 0"
          >&#9650;</button>
          <button
            v-if="job.status === 'pending'"
            @click="store.moveJobDown(job.id)"
            class="job-action-btn"
            title="下移"
            :disabled="index === store.simulationQueue.length - 1"
          >&#9660;</button>
          <button
            v-if="job.status === 'running'"
            @click="store.cancelJob(job.id)"
            class="job-action-btn action-cancel"
            title="取消"
          >&#10006;</button>
          <button
            v-if="job.status === 'failed' || job.status === 'cancelled'"
            @click="store.retryJob(job.id)"
            class="job-action-btn action-retry"
            title="重试"
          >&#8635;</button>
          <button
            v-if="job.status !== 'running'"
            @click="store.removeFromQueue(job.id)"
            class="job-action-btn action-remove"
            title="移除"
          >&times;</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useAutomationStore, type SimulationJob } from '../../stores/automation'

const store = useAutomationStore()

// 拖拽状态
const draggedJobId = ref<string | null>(null)
const dragOverIndex = ref<number>(-1)

// 队列统计
const stats = computed(() => store.queueStats)

// 实时更新计时器
let timerHandle: ReturnType<typeof setInterval> | null = null

onMounted(() => {
  // 每秒更新一次 ETA 和运行时间
  timerHandle = setInterval(() => {
    // 触发响应式更新
    store.simulationQueue.forEach(job => {
      if (job.status === 'running' && job.startTime) {
        // 更新 estimatedTime（基于当前进度推算）
        const elapsed = Date.now() - job.startTime
        if (job.progress > 0) {
          job.estimatedTime = (elapsed / job.progress) * (100 - job.progress)
        }
      }
    })
  }, 1000)

  // 请求通知权限
  store.requestNotificationPermission()
})

onUnmounted(() => {
  if (timerHandle) {
    clearInterval(timerHandle)
  }
})

// 队列操作
function startQueue() {
  store.startQueue()
}

function pauseQueue() {
  store.pauseQueue()
}

function stopQueue() {
  store.stopQueue()
}

function clearCompleted() {
  store.clearCompletedJobs()
}

// 添加示例任务
function addDemoJobs() {
  const demoNames = [
    '静力学分析 - 10kN',
    '静力学分析 - 20kN',
    '静力学分析 - 30kN',
    '模态分析 - 前10阶',
    '热传导分析',
  ]
  demoNames.forEach((name, i) => {
    store.addToQueue({
      name,
      projectId: `demo-project-${i + 1}`,
      priority: i + 1,
    })
  })
}

// 拖拽处理
function onDragStart(event: DragEvent, jobId: string) {
  draggedJobId.value = jobId
  if (event.dataTransfer) {
    event.dataTransfer.effectAllowed = 'move'
    event.dataTransfer.setData('text/plain', jobId)
  }
}

function onDragEnd() {
  draggedJobId.value = null
  dragOverIndex.value = -1
}

function onDragOver(event: DragEvent) {
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'move'
  }
}

function onDrop(event: DragEvent) {
  // Container-level drop: find the closest item and delegate
  const target = (event.target as HTMLElement).closest('[data-job-index]')
  if (target) {
    const index = parseInt(target.getAttribute('data-job-index') || '0', 10)
    onItemDrop(event, index)
  }
}

function onItemDragOver(event: DragEvent, index: number) {
  dragOverIndex.value = index
}

function onItemDrop(event: DragEvent, targetIndex: number) {
  const jobId = event.dataTransfer?.getData('text/plain')
  if (!jobId) return

  const jobs = store.simulationQueue
  const sourceIndex = jobs.findIndex(j => j.id === jobId)
  if (sourceIndex === -1 || sourceIndex === targetIndex) return

  // 只允许拖拽 pending 状态的任务
  if (jobs[sourceIndex].status !== 'pending') return

  const jobIds = jobs
    .filter(j => j.status === 'pending')
    .map(j => j.id)

  // 在 pending 列表中重排
  const sourceInPending = jobIds.indexOf(jobId)
  const targetInPending = jobIds.indexOf(jobs[targetIndex].id)

  if (sourceInPending === -1 || targetInPending === -1) return

  jobIds.splice(sourceInPending, 1)
  jobIds.splice(targetInPending, 0, jobId)

  store.reorderQueue(jobIds)
  draggedJobId.value = null
  dragOverIndex.value = -1
}

// 格式化工具
function formatTime(ms: number): string {
  if (!ms || ms <= 0) return '--:--'
  const seconds = Math.round(ms / 1000)
  if (seconds < 60) return `${seconds}s`
  const minutes = Math.floor(seconds / 60)
  const remainingSeconds = seconds % 60
  if (minutes < 60) return `${minutes}m ${remainingSeconds}s`
  const hours = Math.floor(minutes / 60)
  const remainingMinutes = minutes % 60
  return `${hours}h ${remainingMinutes}m`
}

function formatDuration(ms: number): string {
  if (!ms || ms <= 0) return ''
  const seconds = (ms / 1000).toFixed(1)
  return `${seconds}s`
}
</script>

<style scoped>
.simulation-queue {
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: 12px;
}

/* 统计栏 */
.queue-stats-bar {
  display: flex;
  gap: 4px;
  padding: 8px 12px;
  background: var(--bg-elevated, #f8fafc);
  border-radius: 8px;
  border: 1px solid var(--border-subtle, #e2e8f0);
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex: 1;
  padding: 4px 0;
}

.stat-label {
  font-size: 10px;
  color: var(--text-secondary, #64748b);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.stat-value {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
}

.stat-pending .stat-value { color: #f59e0b; }
.stat-running .stat-value { color: #3b82f6; }
.stat-completed .stat-value { color: #10b981; }
.stat-failed .stat-value { color: #ef4444; }
.stat-eta .stat-value { font-size: 13px; color: #6366f1; }

/* 操作栏 */
.queue-actions {
  display: flex;
  gap: 6px;
  align-items: center;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 12px;
  font-size: 12px;
  font-weight: 500;
  border: 1px solid var(--border-subtle, #e2e8f0);
  border-radius: 6px;
  background: white;
  color: var(--text-primary, #1e293b);
  cursor: pointer;
  transition: all 0.2s;
}

.action-btn:hover:not(:disabled) {
  background: var(--bg-elevated, #f8fafc);
}

.action-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.action-icon {
  font-size: 12px;
}

.action-start {
  background: #10b981;
  color: white;
  border-color: #10b981;
}
.action-start:hover:not(:disabled) {
  background: #059669;
}

.action-pause {
  background: #f59e0b;
  color: white;
  border-color: #f59e0b;
}
.action-pause:hover:not(:disabled) {
  background: #d97706;
}

.action-stop {
  background: #ef4444;
  color: white;
  border-color: #ef4444;
}
.action-stop:hover:not(:disabled) {
  background: #dc2626;
}

.action-clear {
  color: #64748b;
}

.action-add {
  color: #3b82f6;
  border-color: #3b82f6;
}

.action-notify {
  padding: 6px 8px;
}

.spacer {
  flex: 1;
}

/* 队列列表 */
.queue-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding-right: 4px;
}

.queue-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 200px;
  color: var(--text-secondary, #64748b);
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 12px;
  opacity: 0.5;
}

.empty-hint {
  font-size: 12px;
  margin-top: 4px;
  opacity: 0.7;
}

/* 队列项 */
.queue-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  background: white;
  border: 1px solid var(--border-subtle, #e2e8f0);
  border-radius: 8px;
  transition: all 0.2s;
  cursor: default;
}

.dark .queue-item {
  background: #1f2937;
  border-color: #374151;
}

.queue-item:hover {
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
}

.queue-item.is-running {
  border-color: #3b82f6;
  background: #eff6ff;
}

.dark .queue-item.is-running {
  background: #1e3a5f;
  border-color: #3b82f6;
}

.queue-item.is-completed {
  opacity: 0.7;
}

.queue-item.is-failed {
  border-color: #fca5a5;
  background: #fef2f2;
}

.dark .queue-item.is-failed {
  background: #3b1c1c;
  border-color: #7f1d1d;
}

.queue-item.is-cancelled {
  opacity: 0.4;
}

.queue-item.is-dragging {
  opacity: 0.5;
  border: 2px dashed #3b82f6;
}

/* 拖拽手柄 */
.drag-handle {
  cursor: grab;
  color: var(--text-secondary, #94a3b8);
  font-size: 14px;
  padding: 2px;
  user-select: none;
}

.drag-handle:active {
  cursor: grabbing;
}

/* 状态图标 */
.job-status-icon {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  font-size: 12px;
  flex-shrink: 0;
}

.status-pending {
  background: #fef3c7;
  color: #d97706;
}

.status-running {
  background: #dbeafe;
  color: #2563eb;
}

.status-completed {
  background: #d1fae5;
  color: #059669;
}

.status-failed {
  background: #fee2e2;
  color: #dc2626;
}

.status-cancelled {
  background: #f1f5f9;
  color: #94a3b8;
}

.spin {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* 任务信息 */
.job-info {
  flex: 1;
  min-width: 0;
}

.job-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary, #1e293b);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.job-meta {
  display: flex;
  gap: 8px;
  font-size: 11px;
  color: var(--text-secondary, #64748b);
  margin-top: 2px;
}

.job-error {
  font-size: 11px;
  color: #dc2626;
  margin-top: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* 进度条 */
.progress-bar {
  position: relative;
  height: 18px;
  background: #e2e8f0;
  border-radius: 9px;
  margin-top: 6px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #3b82f6, #6366f1);
  border-radius: 9px;
  transition: width 0.3s ease;
}

.progress-text {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 10px;
  font-weight: 600;
  color: #1e293b;
}

/* ETA */
.job-eta {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 4px 8px;
  background: #f0f0ff;
  border-radius: 6px;
  flex-shrink: 0;
}

.eta-label {
  font-size: 9px;
  color: #6366f1;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.eta-value {
  font-size: 12px;
  font-weight: 600;
  color: #4338ca;
}

/* 操作按钮 */
.job-actions {
  display: flex;
  gap: 2px;
  flex-shrink: 0;
}

.job-action-btn {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: var(--text-secondary, #64748b);
  cursor: pointer;
  font-size: 12px;
  transition: all 0.15s;
}

.job-action-btn:hover:not(:disabled) {
  background: var(--bg-elevated, #f1f5f9);
  color: var(--text-primary, #1e293b);
}

.job-action-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.action-cancel:hover { color: #dc2626; }
.action-retry:hover { color: #2563eb; }
.action-remove:hover { color: #dc2626; }

/* 深色模式 */
.dark .action-btn {
  background: #374151;
  border-color: #4b5563;
  color: #e5e7eb;
}

.dark .action-btn:hover:not(:disabled) {
  background: #4b5563;
}

.dark .queue-stats-bar {
  background: #1f2937;
  border-color: #374151;
}

.dark .stat-label { color: #9ca3af; }
.dark .stat-value { color: #e5e7eb; }

.dark .job-name { color: #e5e7eb; }
.dark .job-meta { color: #9ca3af; }

.dark .progress-bar { background: #374151; }

.dark .job-eta { background: #1e1b4b; }
.dark .eta-label { color: #818cf8; }
.dark .eta-value { color: #a5b4fc; }

.dark .job-action-btn:hover:not(:disabled) {
  background: #4b5563;
  color: #e5e7eb;
}

/* 滚动条 */
.queue-list::-webkit-scrollbar {
  width: 4px;
}

.queue-list::-webkit-scrollbar-track {
  background: transparent;
}

.queue-list::-webkit-scrollbar-thumb {
  background: var(--border-subtle, #cbd5e1);
  border-radius: 2px;
}
</style>
