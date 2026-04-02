<template>
  <div class="sync-settings">
    <!-- 标题 -->
    <h3 class="sync-settings-title">
      <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M1.42 9a16.06 16.06 0 0 1 21.16 0"/>
        <path d="M5 12.55a11 11 0 0 1 14.08 0"/>
        <path d="M8.53 16.11a6 6 0 0 1 6.95 0"/>
        <circle cx="12" cy="20" r="1" fill="currentColor"/>
      </svg>
      跨端同步设置
    </h3>

    <!-- 服务器配置 -->
    <div class="sync-settings-section">
      <div class="sync-settings-section-title">服务器配置</div>

      <!-- 服务器地址 -->
      <div class="sync-field">
        <label class="sync-field-label">同步服务器地址</label>
        <div class="sync-field-row">
          <input
            v-model="serverUrl"
            type="text"
            placeholder="ws://localhost:3001/ws"
            class="sync-input"
            :disabled="isConnected"
          />
          <button
            class="sync-btn sync-btn-sm"
            @click="testConnection"
            :disabled="isTesting || !serverUrl"
          >
            <svg v-if="isTesting" class="w-3.5 h-3.5 animate-spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
            </svg>
            <span v-else>测试</span>
          </button>
        </div>
        <p v-if="testResult" class="sync-field-hint" :class="testResultClass">
          {{ testResult }}
        </p>
        <p v-else class="sync-field-hint">
          WebSocket 服务器地址，用于桌面端与移动端之间的实时同步
        </p>
      </div>

      <!-- 认证 Token -->
      <div class="sync-field">
        <label class="sync-field-label">认证 Token（可选）</label>
        <input
          v-model="authToken"
          type="password"
          placeholder="留空表示不使用认证"
          class="sync-input"
        />
        <p class="sync-field-hint">如果服务器启用了认证，请填写 Token</p>
      </div>
    </div>

    <!-- 设备信息 -->
    <div class="sync-settings-section">
      <div class="sync-settings-section-title">设备信息</div>

      <div class="sync-field">
        <label class="sync-field-label">设备 ID</label>
        <div class="sync-field-row">
          <code class="sync-code">{{ deviceId }}</code>
          <button class="sync-btn sync-btn-sm sync-btn-danger-text" @click="handleResetDeviceId">
            重置
          </button>
        </div>
        <p class="sync-field-hint">用于唯一标识当前设备，重置后将生成新的设备 ID</p>
      </div>
    </div>

    <!-- 同步模式 -->
    <div class="sync-settings-section">
      <div class="sync-settings-section-title">同步模式</div>

      <!-- 自动同步开关 -->
      <div class="sync-field">
        <div class="sync-toggle-row">
          <div>
            <div class="sync-field-label">自动同步</div>
            <p class="sync-field-hint">启用后将在检测到变更时自动同步</p>
          </div>
          <label class="sync-toggle">
            <input type="checkbox" v-model="autoSync" />
            <span class="sync-toggle-slider"></span>
          </label>
        </div>
      </div>

      <!-- 同步间隔 -->
      <div v-if="autoSync" class="sync-field">
        <label class="sync-field-label">
          同步间隔
          <span class="text-[var(--text-muted)]">当前: {{ syncIntervalLabel }}</span>
        </label>
        <input
          v-model.number="syncInterval"
          type="range"
          min="0"
          max="60000"
          step="1000"
          class="sync-range"
        />
        <div class="sync-range-labels">
          <span>实时</span>
          <span>30秒</span>
          <span>60秒</span>
        </div>
        <p class="sync-field-hint">设为 0 表示实时同步（每次变更立即推送）</p>
      </div>
    </div>

    <!-- 冲突解决策略 -->
    <div class="sync-settings-section">
      <div class="sync-settings-section-title">冲突解决策略</div>

      <div class="sync-strategy-options">
        <label
          v-for="option in strategyOptions"
          :key="option.value"
          class="sync-strategy-option"
          :class="{ 'sync-strategy-active': conflictStrategy === option.value }"
        >
          <input
            type="radio"
            :value="option.value"
            v-model="conflictStrategy"
            class="sr-only"
          />
          <div class="sync-strategy-radio">
            <svg v-if="conflictStrategy === option.value" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
              <polyline points="20 6 9 17 4 12"/>
            </svg>
          </div>
          <div>
            <div class="sync-strategy-name">{{ option.label }}</div>
            <div class="sync-strategy-desc">{{ option.description }}</div>
          </div>
        </label>
      </div>
    </div>

    <!-- 同步日志 -->
    <div class="sync-settings-section">
      <div class="sync-settings-section-title flex items-center justify-between">
        <span>同步日志</span>
        <button
          class="text-xs text-[var(--text-muted)] hover:text-[var(--text-secondary)]"
          @click="clearLog"
        >
          清除日志
        </button>
      </div>

      <div class="sync-log-container">
        <div v-if="logs.length === 0" class="sync-log-empty">
          暂无同步日志
        </div>
        <div
          v-for="entry in logs"
          :key="entry.id"
          class="sync-log-entry"
          :class="`sync-log-${entry.direction}`"
        >
          <span class="sync-log-time">{{ formatLogTime(entry.timestamp) }}</span>
          <span class="sync-log-badge" :class="`sync-log-badge-${entry.direction}`">
            {{ logBadgeText(entry.direction) }}
          </span>
          <span class="sync-log-msg">{{ entry.message }}</span>
        </div>
      </div>
    </div>

    <!-- 危险操作 -->
    <div class="sync-settings-section sync-settings-danger">
      <div class="sync-settings-section-title">危险操作</div>

      <div class="sync-danger-actions">
        <button
          class="sync-btn sync-btn-danger-full"
          @click="handleClearSyncData"
        >
          <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="3 6 5 6 21 6"/>
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
          </svg>
          清除所有同步数据
        </button>
        <p class="sync-field-hint">
          将清除向量时钟、待处理操作和同步日志。此操作不可撤销。
        </p>
      </div>
    </div>

    <!-- 保存按钮 -->
    <div class="sync-settings-actions">
      <button class="sync-btn sync-btn-secondary" @click="handleReset">
        重置默认
      </button>
      <button class="sync-btn sync-btn-primary" @click="handleSave">
        保存设置
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import {
  useSyncService,
  type SyncLogEntry,
  type ConflictResolutionStrategy,
} from '@/api/syncService'

const syncService = useSyncService()

// 表单状态
const serverUrl = ref('ws://localhost:3001/ws')
const authToken = ref('')
const autoSync = ref(true)
const syncInterval = ref(0)
const conflictStrategy = ref<ConflictResolutionStrategy>('ask')

// UI 状态
const isTesting = ref(false)
const testResult = ref('')
const testSuccess = ref(false)
const logs = ref<SyncLogEntry[]>([])

const deviceId = computed(() => syncService.getDeviceId())
const isConnected = computed(() =>
  syncService.status.value === 'connected' || syncService.status.value === 'syncing'
)

const syncIntervalLabel = computed(() => {
  if (syncInterval.value === 0) return '实时'
  if (syncInterval.value < 1000) return `${syncInterval.value}ms`
  return `${syncInterval.value / 1000}秒`
})

const testResultClass = computed(() => ({
  'sync-hint-success': testSuccess.value,
  'sync-hint-error': !testSuccess.value,
}))

const strategyOptions = [
  {
    value: 'ask' as ConflictResolutionStrategy,
    label: '手动解决',
    description: '检测到冲突时弹出提示，由用户选择保留哪个版本或合并',
  },
  {
    value: 'local_wins' as ConflictResolutionStrategy,
    label: '本地优先',
    description: '冲突时自动保留本地版本，远程版本被忽略',
  },
  {
    value: 'remote_wins' as ConflictResolutionStrategy,
    label: '远程优先',
    description: '冲突时自动采用远程版本，本地版本被覆盖',
  },
]

onMounted(() => {
  // 从 localStorage 恢复设置
  const savedUrl = localStorage.getItem('caelab-sync-server-url')
  if (savedUrl) serverUrl.value = savedUrl

  const savedToken = localStorage.getItem('caelab-sync-auth-token')
  if (savedToken) authToken.value = savedToken

  const savedAutoSync = localStorage.getItem('caelab-sync-auto')
  if (savedAutoSync !== null) autoSync.value = savedAutoSync === 'true'

  const savedInterval = localStorage.getItem('caelab-sync-interval')
  if (savedInterval) syncInterval.value = parseInt(savedInterval, 10)

  conflictStrategy.value = syncService.getConflictStrategy()

  // 加载日志
  logs.value = syncService.getSyncLog()

  // 监听新日志
  const unlisten = syncService.on('log', (entry: SyncLogEntry) => {
    logs.value.unshift(entry)
    if (logs.value.length > 100) {
      logs.value = logs.value.slice(0, 100)
    }
  })

  onUnmounted(() => {
    unlisten()
  })
})

function testConnection() {
  isTesting.value = true
  testResult.value = ''

  try {
    const ws = new WebSocket(serverUrl.value)
    const timeout = setTimeout(() => {
      ws.close()
      testResult.value = '连接超时（5秒），请检查服务器地址'
      testSuccess.value = false
      isTesting.value = false
    }, 5000)

    ws.onopen = () => {
      clearTimeout(timeout)
      ws.close()
      testResult.value = '连接成功！服务器可用'
      testSuccess.value = true
      isTesting.value = false
    }

    ws.onerror = () => {
      clearTimeout(timeout)
      testResult.value = '连接失败，请检查服务器地址是否正确'
      testSuccess.value = false
      isTesting.value = false
    }
  } catch {
    testResult.value = '地址格式无效，请输入有效的 WebSocket 地址'
    testSuccess.value = false
    isTesting.value = false
  }
}

function handleResetDeviceId() {
  if (confirm('确定要重置设备 ID 吗？重置后其他设备将无法识别此设备。')) {
    syncService.resetDeviceId()
  }
}

function clearLog() {
  syncService.clearSyncLog()
  logs.value = []
}

function handleClearSyncData() {
  if (confirm('确定要清除所有同步数据吗？此操作不可撤销。')) {
    syncService.clearSyncData()
    logs.value = []
  }
}

function handleSave() {
  // 保存到 localStorage
  localStorage.setItem('caelab-sync-server-url', serverUrl.value)
  localStorage.setItem('caelab-sync-auth-token', authToken.value)
  localStorage.setItem('caelab-sync-auto', String(autoSync.value))
  localStorage.setItem('caelab-sync-interval', String(syncInterval.value))

  // 更新同步服务配置
  syncService.setConflictStrategy(conflictStrategy.value)

  alert('同步设置已保存')
}

function handleReset() {
  serverUrl.value = 'ws://localhost:3001/ws'
  authToken.value = ''
  autoSync.value = true
  syncInterval.value = 0
  conflictStrategy.value = 'ask'
}

function formatLogTime(ts: string): string {
  return new Date(ts).toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
  })
}

function logBadgeText(dir: SyncLogEntry['direction']): string {
  const map: Record<SyncLogEntry['direction'], string> = {
    sent: '发送',
    received: '接收',
    conflict: '冲突',
    error: '错误',
    info: '信息',
  }
  return map[dir]
}
</script>

<style scoped>
.sync-settings {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.sync-settings-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #e5e7eb);
}

.sync-settings-section {
  background: var(--bg-surface, rgba(255, 255, 255, 0.03));
  border: 1px solid var(--border-subtle, rgba(255, 255, 255, 0.08));
  border-radius: 10px;
  padding: 16px;
}

.sync-settings-danger {
  border-color: rgba(239, 68, 68, 0.2);
}

.sync-settings-section-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary, #9ca3af);
  margin-bottom: 12px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.sync-field {
  margin-bottom: 14px;
}

.sync-field:last-child {
  margin-bottom: 0;
}

.sync-field-label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary, #e5e7eb);
  margin-bottom: 6px;
}

.sync-field-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.sync-input {
  flex: 1;
  padding: 8px 12px;
  border-radius: 8px;
  border: 1px solid var(--border-subtle, rgba(255, 255, 255, 0.12));
  background: var(--bg-base, #1a1a2e);
  color: var(--text-primary, #e5e7eb);
  font-size: 13px;
  outline: none;
  transition: border-color 0.15s;
}

.sync-input:focus {
  border-color: var(--accent-primary, #3b82f6);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.15);
}

.sync-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.sync-field-hint {
  font-size: 11px;
  color: var(--text-muted, #6b7280);
  margin-top: 4px;
}

.sync-hint-success {
  color: var(--accent-green, #22c55e) !important;
}

.sync-hint-error {
  color: var(--accent-red, #ef4444) !important;
}

.sync-code {
  flex: 1;
  padding: 6px 10px;
  border-radius: 6px;
  background: var(--bg-elevated, rgba(255, 255, 255, 0.05));
  color: var(--text-secondary, #9ca3af);
  font-size: 12px;
  font-family: 'SF Mono', 'Fira Code', monospace;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* Toggle 开关 */
.sync-toggle-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.sync-toggle {
  position: relative;
  display: inline-block;
  width: 40px;
  height: 22px;
  flex-shrink: 0;
  cursor: pointer;
}

.sync-toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.sync-toggle-slider {
  position: absolute;
  inset: 0;
  background: var(--bg-elevated, rgba(255, 255, 255, 0.1));
  border-radius: 11px;
  transition: background 0.2s;
}

.sync-toggle-slider::before {
  content: '';
  position: absolute;
  width: 16px;
  height: 16px;
  left: 3px;
  bottom: 3px;
  background: var(--text-muted, #6b7280);
  border-radius: 50%;
  transition: all 0.2s;
}

.sync-toggle input:checked + .sync-toggle-slider {
  background: var(--accent-primary, #3b82f6);
}

.sync-toggle input:checked + .sync-toggle-slider::before {
  transform: translateX(18px);
  background: white;
}

/* Range 滑块 */
.sync-range {
  width: 100%;
  height: 4px;
  -webkit-appearance: none;
  appearance: none;
  background: var(--bg-elevated, rgba(255, 255, 255, 0.1));
  border-radius: 2px;
  outline: none;
}

.sync-range::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: var(--accent-primary, #3b82f6);
  cursor: pointer;
  border: 2px solid white;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
}

.sync-range-labels {
  display: flex;
  justify-content: space-between;
  font-size: 10px;
  color: var(--text-muted, #6b7280);
  margin-top: 4px;
}

/* 策略选项 */
.sync-strategy-options {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.sync-strategy-option {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 8px;
  border: 1px solid var(--border-subtle, rgba(255, 255, 255, 0.08));
  cursor: pointer;
  transition: all 0.15s;
}

.sync-strategy-option:hover {
  background: var(--bg-hover, rgba(255, 255, 255, 0.03));
}

.sync-strategy-active {
  border-color: var(--accent-primary, #3b82f6);
  background: rgba(59, 130, 246, 0.05);
}

.sync-strategy-radio {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  border: 2px solid var(--border-subtle, rgba(255, 255, 255, 0.2));
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  margin-top: 1px;
  transition: border-color 0.15s;
}

.sync-strategy-active .sync-strategy-radio {
  border-color: var(--accent-primary, #3b82f6);
  color: var(--accent-primary, #3b82f6);
}

.sync-strategy-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary, #e5e7eb);
}

.sync-strategy-desc {
  font-size: 11px;
  color: var(--text-muted, #6b7280);
  margin-top: 2px;
}

/* 日志 */
.sync-log-container {
  max-height: 240px;
  overflow-y: auto;
  border-radius: 6px;
  background: var(--bg-base, #1a1a2e);
  border: 1px solid var(--border-subtle, rgba(255, 255, 255, 0.06));
}

.sync-log-empty {
  padding: 20px;
  text-align: center;
  font-size: 12px;
  color: var(--text-muted, #6b7280);
}

.sync-log-entry {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 5px 10px;
  font-size: 11px;
  font-family: 'SF Mono', 'Fira Code', monospace;
  border-bottom: 1px solid rgba(255, 255, 255, 0.03);
}

.sync-log-entry:last-child {
  border-bottom: none;
}

.sync-log-time {
  color: var(--text-muted, #6b7280);
  flex-shrink: 0;
}

.sync-log-badge {
  flex-shrink: 0;
  padding: 1px 6px;
  border-radius: 3px;
  font-size: 10px;
  font-weight: 600;
}

.sync-log-badge-sent {
  background: rgba(59, 130, 246, 0.15);
  color: var(--accent-primary, #3b82f6);
}

.sync-log-badge-received {
  background: rgba(34, 197, 94, 0.15);
  color: var(--accent-green, #22c55e);
}

.sync-log-badge-conflict {
  background: rgba(245, 158, 11, 0.15);
  color: var(--accent-amber, #f59e0b);
}

.sync-log-badge-error {
  background: rgba(239, 68, 68, 0.15);
  color: var(--accent-red, #ef4444);
}

.sync-log-badge-info {
  background: rgba(107, 114, 128, 0.15);
  color: var(--text-muted, #6b7280);
}

.sync-log-msg {
  color: var(--text-secondary, #9ca3af);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 按钮 */
.sync-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  padding: 6px 14px;
  border-radius: 8px;
  font-size: 13px;
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

.sync-btn-sm {
  padding: 6px 12px;
  font-size: 12px;
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
  border-color: var(--border-subtle, rgba(255, 255, 255, 0.12));
}

.sync-btn-secondary:hover:not(:disabled) {
  background: var(--bg-hover, rgba(255, 255, 255, 0.05));
}

.sync-btn-danger-text {
  color: var(--accent-red, #ef4444);
  border-color: rgba(239, 68, 68, 0.2);
}

.sync-btn-danger-text:hover:not(:disabled) {
  background: rgba(239, 68, 68, 0.1);
}

.sync-btn-danger-full {
  width: 100%;
  padding: 10px;
  background: rgba(239, 68, 68, 0.08);
  color: var(--accent-red, #ef4444);
  border-color: rgba(239, 68, 68, 0.2);
}

.sync-btn-danger-full:hover:not(:disabled) {
  background: rgba(239, 68, 68, 0.15);
}

.sync-danger-actions {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.sync-settings-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding-top: 4px;
}

/* 移动端适配 */
@media (max-width: 768px) {
  .sync-settings {
    gap: 16px;
  }

  .sync-settings-section {
    padding: 12px;
  }

  .sync-field-row {
    flex-direction: column;
    align-items: stretch;
  }

  .sync-settings-actions {
    flex-direction: column;
  }

  .sync-settings-actions .sync-btn {
    width: 100%;
  }
}
</style>
