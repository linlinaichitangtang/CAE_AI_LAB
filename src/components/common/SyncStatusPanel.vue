<template>
  <div class="sync-status-panel">
    <!-- 连接状态头部 -->
    <div class="sync-header">
      <div class="flex items-center gap-2">
        <span
          class="sync-dot"
          :class="statusDotClass"
        ></span>
        <span class="font-medium text-sm">{{ statusText }}</span>
      </div>
      <div class="flex items-center gap-2">
        <button
          v-if="status === 'disconnected' || status === 'error'"
          class="sync-btn sync-btn-primary"
          @click="handleConnect"
        >
          <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M5 12h14M12 5l7 7-7 7"/>
          </svg>
          连接
        </button>
        <button
          v-else
          class="sync-btn sync-btn-danger"
          @click="handleDisconnect"
        >
          <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6L6 18M6 6l12 12"/>
          </svg>
          断开
        </button>
        <button
          class="sync-btn sync-btn-secondary"
          @click="handleSyncNow"
          :disabled="status === 'disconnected' || status === 'error'"
        >
          <svg class="w-3.5 h-3.5" :class="{ 'animate-spin': status === 'syncing' }" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
          </svg>
          同步
        </button>
      </div>
    </div>

    <!-- 设备信息 -->
    <div class="sync-section">
      <div class="sync-section-title">设备信息</div>
      <div class="sync-info-grid">
        <div class="sync-info-item">
          <span class="sync-info-label">设备 ID</span>
          <span class="sync-info-value font-mono text-xs" :title="deviceId">{{ shortDeviceId }}</span>
        </div>
        <div class="sync-info-item">
          <span class="sync-info-label">服务器</span>
          <span class="sync-info-value text-xs truncate" :title="serverUrl">{{ serverUrl }}</span>
        </div>
      </div>
    </div>

    <!-- 同步统计 -->
    <div class="sync-section">
      <div class="sync-section-title">同步统计</div>
      <div class="sync-stats-grid">
        <div class="sync-stat-card">
          <div class="sync-stat-value">{{ stats.totalOpsSent }}</div>
          <div class="sync-stat-label">已发送</div>
        </div>
        <div class="sync-stat-card">
          <div class="sync-stat-value">{{ stats.totalOpsReceived }}</div>
          <div class="sync-stat-label">已接收</div>
        </div>
        <div class="sync-stat-card" :class="{ 'sync-stat-warning': stats.totalConflicts > 0 }">
          <div class="sync-stat-value">{{ stats.totalConflicts }}</div>
          <div class="sync-stat-label">冲突</div>
        </div>
        <div class="sync-stat-card">
          <div class="sync-stat-value">{{ stats.pendingOps }}</div>
          <div class="sync-stat-label">待处理</div>
        </div>
      </div>
    </div>

    <!-- 最后同步时间 -->
    <div v-if="stats.lastSyncTime" class="sync-section">
      <div class="sync-info-item">
        <span class="sync-info-label">最后同步</span>
        <span class="sync-info-value text-xs">{{ formattedLastSyncTime }}</span>
      </div>
    </div>

    <!-- 冲突列表 -->
    <div v-if="conflicts.length > 0" class="sync-section">
      <div class="sync-section-title sync-section-title-warning">
        待解决冲突 ({{ conflicts.length }})
      </div>
      <div class="sync-conflict-list">
        <div
          v-for="conflict in conflicts"
          :key="conflict.id"
          class="sync-conflict-item"
        >
          <div class="flex items-center justify-between mb-1">
            <span class="text-xs font-medium">{{ formatOpType(conflict.opType) }}</span>
            <span class="text-xs text-[var(--text-muted)]">{{ conflict.projectId }}</span>
          </div>
          <div class="flex items-center gap-1 text-xs text-[var(--text-secondary)] mb-2">
            <span>本地: {{ formatTimestamp(conflict.localTimestamp) }}</span>
            <span class="mx-1">|</span>
            <span>远程: {{ formatTimestamp(conflict.remoteTimestamp) }}</span>
          </div>
          <div class="flex items-center gap-1.5">
            <button
              class="sync-btn sync-btn-xs sync-btn-primary"
              @click="resolveConflict(conflict.id, 'local_wins')"
            >
              保留本地
            </button>
            <button
              class="sync-btn sync-btn-xs sync-btn-secondary"
              @click="resolveConflict(conflict.id, 'remote_wins')"
            >
              采用远程
            </button>
            <button
              class="sync-btn sync-btn-xs sync-btn-warning"
              @click="resolveConflictWithMerge(conflict.id)"
            >
              合并
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 最近同步日志 -->
    <div class="sync-section">
      <div class="sync-section-title flex items-center justify-between">
        <span>同步日志</span>
        <button
          class="text-xs text-[var(--text-muted)] hover:text-[var(--text-secondary)]"
          @click="toggleLogExpand"
        >
          {{ logExpanded ? '收起' : '展开' }}
        </button>
      </div>
      <div v-if="logExpanded" class="sync-log-list">
        <div
          v-for="entry in recentLogs"
          :key="entry.id"
          class="sync-log-item"
          :class="`sync-log-${entry.direction}`"
        >
          <span class="sync-log-time">{{ formatLogTime(entry.timestamp) }}</span>
          <span class="sync-log-direction" :class="`sync-log-dir-${entry.direction}`">
            {{ logDirectionLabel(entry.direction) }}
          </span>
          <span class="sync-log-message">{{ entry.message }}</span>
        </div>
        <div v-if="recentLogs.length === 0" class="text-xs text-[var(--text-muted)] py-2">
          暂无日志
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import {
  useSyncService,
  type SyncStatus,
  type SyncConflict,
  type SyncLogEntry,
  type SyncOpType,
} from '@/api/syncService'

const syncService = useSyncService()

const status = computed(() => syncService.status.value)
const conflicts = computed(() => syncService.conflicts.value)
const stats = computed(() => syncService.stats.value)
const deviceId = computed(() => syncService.getDeviceId())

const serverUrl = ref('ws://localhost:3001/ws')
const logExpanded = ref(false)
const recentLogs = ref<SyncLogEntry[]>([])

// 从 localStorage 读取服务器地址
onMounted(() => {
  const savedUrl = localStorage.getItem('caelab-sync-server-url')
  if (savedUrl) serverUrl.value = savedUrl

  // 监听日志事件
  const unlisten = syncService.on('log', (entry: SyncLogEntry) => {
    recentLogs.value.unshift(entry)
    if (recentLogs.value.length > 50) {
      recentLogs.value = recentLogs.value.slice(0, 50)
    }
  })

  onUnmounted(() => {
    unlisten()
  })
})

const shortDeviceId = computed(() => {
  const id = deviceId.value
  if (id.length > 16) return id.substring(0, 8) + '...' + id.substring(id.length - 4)
  return id
})

const statusDotClass = computed(() => ({
  'sync-dot-green': status.value === 'connected',
  'sync-dot-yellow': status.value === 'connecting' || status.value === 'syncing',
  'sync-dot-red': status.value === 'error',
  'sync-dot-gray': status.value === 'disconnected',
}))

const statusText = computed(() => {
  const map: Record<SyncStatus, string> = {
    disconnected: '未连接',
    connecting: '连接中...',
    connected: '已连接',
    syncing: '同步中...',
    error: '连接错误',
  }
  return map[status.value]
})

const formattedLastSyncTime = computed(() => {
  if (!stats.value.lastSyncTime) return ''
  return new Date(stats.value.lastSyncTime).toLocaleString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
  })
})

function handleConnect() {
  syncService.connect()
}

function handleDisconnect() {
  syncService.disconnect()
}

function handleSyncNow() {
  syncService.syncNow()
}

function resolveConflict(conflictId: string, resolution: 'local_wins' | 'remote_wins') {
  syncService.resolveConflict(conflictId, resolution)
}

function resolveConflictWithMerge(conflictId: string) {
  // 使用空对象作为 base 进行合并（简化处理）
  syncService.resolveConflictWithMerge(conflictId, {})
}

function toggleLogExpand() {
  logExpanded.value = !logExpanded.value
}

function formatOpType(type: SyncOpType): string {
  const map: Record<SyncOpType, string> = {
    create_project: '创建项目',
    delete_project: '删除项目',
    rename_project: '重命名项目',
    update_material: '更新材料',
    update_mesh: '更新网格',
    update_boundary_condition: '更新边界条件',
    update_geometry: '更新几何',
    add_result: '添加结果',
    delete_result: '删除结果',
    update_note: '更新笔记',
    add_note: '添加笔记',
    delete_note: '删除笔记',
    update_setting: '更新设置',
  }
  return map[type] || type
}

function formatTimestamp(ts: string): string {
  return new Date(ts).toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
  })
}

function formatLogTime(ts: string): string {
  return new Date(ts).toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
  })
}

function logDirectionLabel(dir: SyncLogEntry['direction']): string {
  const map: Record<SyncLogEntry['direction'], string> = {
    sent: '>>',
    received: '<<',
    conflict: '!!',
    error: 'ER',
    info: '--',
  }
  return map[dir]
}
</script>

<style scoped>
.sync-status-panel {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.sync-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.sync-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.sync-dot-green {
  background: var(--accent-green, #22c55e);
  box-shadow: 0 0 6px rgba(34, 197, 94, 0.4);
}

.sync-dot-yellow {
  background: var(--accent-amber, #f59e0b);
  animation: sync-pulse 1.5s infinite;
}

.sync-dot-red {
  background: var(--accent-red, #ef4444);
}

.sync-dot-gray {
  background: var(--text-muted, #6b7280);
}

@keyframes sync-pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.4; }
}

.sync-btn {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  border: 1px solid transparent;
  transition: all 0.15s ease;
  white-space: nowrap;
}

.sync-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.sync-btn-primary {
  background: var(--accent-primary, #3b82f6);
  color: white;
  border-color: var(--accent-primary, #3b82f6);
}

.sync-btn-primary:hover:not(:disabled) {
  opacity: 0.9;
}

.sync-btn-secondary {
  background: transparent;
  color: var(--text-secondary, #9ca3af);
  border-color: var(--border-subtle, #374151);
}

.sync-btn-secondary:hover:not(:disabled) {
  background: var(--bg-hover, rgba(255, 255, 255, 0.05));
}

.sync-btn-danger {
  background: transparent;
  color: var(--accent-red, #ef4444);
  border-color: var(--border-subtle, #374151);
}

.sync-btn-danger:hover:not(:disabled) {
  background: rgba(239, 68, 68, 0.1);
}

.sync-btn-warning {
  background: transparent;
  color: var(--accent-amber, #f59e0b);
  border-color: var(--border-subtle, #374151);
}

.sync-btn-warning:hover:not(:disabled) {
  background: rgba(245, 158, 11, 0.1);
}

.sync-btn-xs {
  padding: 2px 8px;
  font-size: 11px;
}

.sync-section {
  background: var(--bg-surface, rgba(255, 255, 255, 0.03));
  border: 1px solid var(--border-subtle, rgba(255, 255, 255, 0.08));
  border-radius: 8px;
  padding: 12px;
}

.sync-section-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary, #9ca3af);
  margin-bottom: 10px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.sync-section-title-warning {
  color: var(--accent-amber, #f59e0b);
}

.sync-info-grid {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.sync-info-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.sync-info-label {
  font-size: 12px;
  color: var(--text-muted, #6b7280);
  flex-shrink: 0;
}

.sync-info-value {
  font-size: 12px;
  color: var(--text-primary, #e5e7eb);
  overflow: hidden;
  text-overflow: ellipsis;
}

.sync-stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}

.sync-stat-card {
  text-align: center;
  padding: 8px 4px;
  background: var(--bg-elevated, rgba(255, 255, 255, 0.02));
  border-radius: 6px;
}

.sync-stat-value {
  font-size: 18px;
  font-weight: 700;
  color: var(--text-primary, #e5e7eb);
  line-height: 1.2;
}

.sync-stat-label {
  font-size: 10px;
  color: var(--text-muted, #6b7280);
  margin-top: 2px;
}

.sync-stat-warning .sync-stat-value {
  color: var(--accent-amber, #f59e0b);
}

.sync-conflict-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.sync-conflict-item {
  padding: 8px;
  background: rgba(245, 158, 11, 0.05);
  border: 1px solid rgba(245, 158, 11, 0.15);
  border-radius: 6px;
}

.sync-log-list {
  max-height: 200px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.sync-log-item {
  display: flex;
  align-items: flex-start;
  gap: 6px;
  font-size: 11px;
  padding: 3px 6px;
  border-radius: 4px;
  font-family: monospace;
}

.sync-log-item:hover {
  background: var(--bg-hover, rgba(255, 255, 255, 0.03));
}

.sync-log-time {
  color: var(--text-muted, #6b7280);
  flex-shrink: 0;
}

.sync-log-direction {
  flex-shrink: 0;
  font-weight: 700;
  width: 20px;
  text-align: center;
}

.sync-log-dir-sent {
  color: var(--accent-primary, #3b82f6);
}

.sync-log-dir-received {
  color: var(--accent-green, #22c55e);
}

.sync-log-dir-conflict {
  color: var(--accent-amber, #f59e0b);
}

.sync-log-dir-error {
  color: var(--accent-red, #ef4444);
}

.sync-log-dir-info {
  color: var(--text-muted, #6b7280);
}

.sync-log-message {
  color: var(--text-secondary, #9ca3af);
  word-break: break-all;
}

/* 移动端适配 */
@media (max-width: 768px) {
  .sync-status-panel {
    padding: 12px;
  }

  .sync-stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }

  .sync-header {
    flex-direction: column;
    gap: 8px;
    align-items: flex-start;
  }
}
</style>
