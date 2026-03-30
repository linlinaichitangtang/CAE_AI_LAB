<template>
  <div class="script-player-panel">
    <!-- 脚本选择 -->
    <div class="player-script-selector">
      <label class="text-xs text-gray-600 mb-1 block">选择脚本</label>
      <select 
        v-model="selectedScriptId"
        @change="loadScript"
        class="w-full px-2 py-1.5 text-sm border rounded"
      >
        <option value="">请选择脚本...</option>
        <option 
          v-for="script in allScripts" 
          :key="script.id" 
          :value="script.id"
        >
          {{ script.name }}
        </option>
      </select>
    </div>

    <!-- 回放控制 -->
    <div class="playback-controls">
      <div class="control-row">
        <!-- 循环次数 -->
        <div class="loop-count-input">
          <label class="text-xs text-gray-600">循环次数</label>
          <input 
            v-model.number="loopCount"
            type="number"
            min="1"
            max="100"
            class="w-full px-2 py-1 text-sm border rounded"
            :disabled="isPlaying"
          />
        </div>
        
        <!-- 速度 -->
        <div class="speed-input">
          <label class="text-xs text-gray-600">速度</label>
          <select 
            v-model="playbackSpeed"
            class="w-full px-2 py-1 text-sm border rounded"
            :disabled="isPlaying"
          >
            <option value="0.5">0.5x</option>
            <option value="1">1x</option>
            <option value="2">2x</option>
            <option value="5">5x</option>
          </select>
        </div>
      </div>

      <!-- 控制按钮 -->
      <div class="buttons-row">
        <template v-if="!isPlaying">
          <button 
            @click="startPlayback"
            :disabled="!selectedScriptId"
            class="play-btn"
          >
            <span class="play-icon">▶</span>
            <span>开始回放</span>
          </button>
        </template>
        
        <template v-else>
          <button 
            v-if="playbackStatus === 'running'"
            @click="pausePlayback"
            class="pause-btn"
          >
            <span class="pause-icon">⏸</span>
            <span>暂停</span>
          </button>
          
          <button 
            v-else
            @click="resumePlayback"
            class="resume-btn"
          >
            <span class="resume-icon">▶</span>
            <span>继续</span>
          </button>
          
          <button 
            @click="stopPlayback"
            class="stop-btn"
          >
            <span class="stop-icon">⏹</span>
            <span>停止</span>
          </button>
        </template>
        
        <button 
          @click="resetPlayback"
          :disabled="isPlaying"
          class="reset-btn"
        >
          重置
        </button>
      </div>
    </div>

    <!-- 执行进度 -->
    <div v-if="playbackSession" class="playback-progress">
      <div class="progress-header">
        <span class="text-xs font-medium text-gray-700">
          执行进度
        </span>
        <span 
          class="status-badge"
          :class="statusClass"
        >
          {{ statusLabel }}
        </span>
      </div>
      
      <!-- 进度条 -->
      <div class="progress-bar-container">
        <div 
          class="progress-bar"
          :style="{ width: progressPercent + '%' }"
          :class="progressClass"
        ></div>
      </div>
      
      <!-- 进度信息 -->
      <div class="progress-info">
        <span class="text-xs text-gray-500">
          步骤 {{ currentStepIndex + 1 }} / {{ totalSteps }}
        </span>
        <span v-if="playbackSession.loopCount > 1" class="text-xs text-indigo-600">
          第 {{ playbackSession.loopCount }} / {{ loopCount }} 轮
        </span>
      </div>
    </div>

    <!-- 当前执行的步骤 -->
    <div v-if="currentStep && isPlaying" class="current-step">
      <div class="current-step-header">
        <span class="text-xs font-medium text-gray-700">正在执行</span>
        <span class="animate-pulse w-2 h-2 bg-green-500 rounded-full"></span>
      </div>
      <div class="current-step-content">
        <div class="step-type-badge">{{ getStepTypeLabel(currentStep.type) }}</div>
        <div class="step-action">{{ currentStep.action }}</div>
        <div v-if="currentStep.description" class="step-desc">{{ currentStep.description }}</div>
      </div>
    </div>

    <!-- 执行结果列表 -->
    <div v-if="playbackResults.length > 0" class="results-section">
      <h5 class="section-title">执行结果</h5>
      
      <div class="results-list">
        <div 
          v-for="(result, idx) in playbackResults" 
          :key="idx"
          class="result-item"
          :class="{ 'result-success': result.success, 'result-failed': !result.success }"
        >
          <div class="result-icon">
            {{ result.success ? '✓' : '✗' }}
          </div>
          <div class="result-content">
            <div class="result-step">{{ getStepLabel(idx) }}</div>
            <div v-if="result.error" class="result-error">{{ result.error }}</div>
            <div v-else class="result-duration">{{ result.duration }}ms</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 执行统计 -->
    <div v-if="playbackSession && playbackSession.status !== 'running'" class="execution-stats">
      <div class="stat-row">
        <span class="stat-label">总耗时</span>
        <span class="stat-value">{{ totalDuration }}ms</span>
      </div>
      <div class="stat-row">
        <span class="stat-label">成功</span>
        <span class="stat-value stat-success">{{ successCount }}</span>
      </div>
      <div class="stat-row">
        <span class="stat-label">失败</span>
        <span class="stat-value stat-failed">{{ failedCount }}</span>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-if="!selectedScriptId" class="empty-state">
      <p class="text-sm text-gray-500">请选择一个脚本开始回放</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useAutomationStore, type PlaybackSession, type ScriptStep, type PlaybackResult } from '@/stores/automation'

// Store
const automationStore = useAutomationStore()

// 状态
const selectedScriptId = ref<string>('')
const loopCount = ref(1)
const playbackSpeed = ref(1)
const playbackSession = ref<PlaybackSession | null>(null)

// 计算属性
const isPlaying = computed(() => automationStore.isPlaying)
const allScripts = computed(() => [
  ...automationStore.builtInTemplates,
  ...automationStore.scripts
])

const playbackStatus = computed(() => playbackSession.value?.status || 'idle')

const statusLabel = computed(() => {
  const labels: Record<string, string> = {
    idle: '空闲',
    running: '执行中',
    paused: '已暂停',
    completed: '已完成',
    failed: '失败'
  }
  return labels[playbackStatus.value] || '空闲'
})

const statusClass = computed(() => {
  const classes: Record<string, string> = {
    idle: 'bg-gray-100 text-gray-600',
    running: 'bg-blue-100 text-blue-600',
    paused: 'bg-yellow-100 text-yellow-600',
    completed: 'bg-green-100 text-green-600',
    failed: 'bg-red-100 text-red-600'
  }
  return classes[playbackStatus.value] || classes.idle
})

const progressPercent = computed(() => {
  if (!playbackSession.value || playbackSession.value.totalSteps === 0) return 0
  return Math.round(
    ((playbackSession.value.currentStepIndex + 1) / playbackSession.value.totalSteps) * 100
  )
})

const progressClass = computed(() => {
  if (playbackStatus.value === 'failed') return 'bg-red-500'
  if (playbackStatus.value === 'completed') return 'bg-green-500'
  return 'bg-blue-500'
})

const currentStepIndex = computed(() => 
  playbackSession.value?.currentStepIndex || 0
)

const totalSteps = computed(() => 
  playbackSession.value?.totalSteps || 0
)

const currentStep = computed((): ScriptStep | null => {
  const script = allScripts.value.find(s => s.id === selectedScriptId.value)
  if (!script || currentStepIndex.value >= script.steps.length) return null
  return script.steps[currentStepIndex.value]
})

const playbackResults = computed(() => 
  playbackSession.value?.results || []
)

const totalDuration = computed(() => {
  if (!playbackSession.value?.results) return 0
  return playbackSession.value.results.reduce((sum, r) => sum + r.duration, 0)
})

const successCount = computed(() => 
  playbackResults.value.filter(r => r.success).length
)

const failedCount = computed(() => 
  playbackResults.value.filter(r => !r.success).length
)

// 方法
function getStepTypeLabel(type: string): string {
  const labels: Record<string, string> = {
    mesh: '网格',
    material: '材料',
    boundary: '边界',
    load: '荷载',
    solve: '求解',
    result: '结果',
    wait: '等待',
    custom: '自定义'
  }
  return labels[type] || type
}

function getStepLabel(index: number): string {
  const script = allScripts.value.find(s => s.id === selectedScriptId.value)
  if (!script || !script.steps[index]) return `步骤 ${index + 1}`
  return script.steps[index].action || `步骤 ${index + 1}`
}

function loadScript() {
  if (selectedScriptId.value) {
    automationStore.loadScript(selectedScriptId.value)
  }
}

async function startPlayback() {
  if (!selectedScriptId.value) return
  
  await automationStore.startPlayback(selectedScriptId.value, loopCount.value)
  
  // 监听回放状态
  watch(() => automationStore.playbackSession, (session) => {
    playbackSession.value = session
  }, { immediate: true })
}

function pausePlayback() {
  automationStore.pausePlayback()
}

function resumePlayback() {
  automationStore.resumePlayback()
}

function stopPlayback() {
  automationStore.stopPlayback()
}

function resetPlayback() {
  automationStore.resetPlayback()
  playbackSession.value = null
}
</script>

<style scoped>
.script-player-panel {
  background: white;
  border-radius: 8px;
  padding: 12px;
  max-height: 100%;
  overflow-y: auto;
}

.player-script-selector {
  margin-bottom: 12px;
}

.playback-controls {
  padding: 12px;
  background: var(--bg-elevated);
  border-radius: 6px;
  margin-bottom: 12px;
}

.control-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
  margin-bottom: 12px;
}

.loop-count-input,
.speed-input {
  display: flex;
  flex-direction: column;
}

.buttons-row {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.play-btn,
.pause-btn,
.resume-btn,
.stop-btn,
.reset-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  padding: 8px 12px;
  font-size: 12px;
  border-radius: 4px;
  transition: all 0.2s;
}

.play-btn {
  background: #22c55e;
  color: white;
}

.play-btn:hover {
  background: #16a34a;
}

.play-btn:disabled {
  background: #86efac;
  cursor: not-allowed;
}

.pause-btn {
  background: #eab308;
  color: white;
}

.pause-btn:hover {
  background: #ca8a04;
}

.resume-btn {
  background: #22c55e;
  color: white;
}

.resume-btn:hover {
  background: #16a34a;
}

.stop-btn {
  background: #ef4444;
  color: white;
}

.stop-btn:hover {
  background: #dc2626;
}

.reset-btn {
  background: white;
  border: 1px solid var(--border-subtle);
  color: var(--text-secondary);
}

.reset-btn:hover {
  background: var(--bg-hover);
}

.reset-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.play-icon,
.pause-icon,
.resume-icon,
.stop-icon {
  font-size: 10px;
}

.playback-progress {
  padding: 12px;
  background: var(--bg-elevated);
  border-radius: 6px;
  margin-bottom: 12px;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.status-badge {
  padding: 2px 8px;
  font-size: 10px;
  border-radius: 10px;
}

.progress-bar-container {
  height: 8px;
  background: #e5e7eb;
  border-radius: 4px;
  overflow: hidden;
}

.progress-bar {
  height: 100%;
  transition: width 0.3s ease;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  margin-top: 4px;
}

.current-step {
  padding: 12px;
  background: #f0fdf4;
  border: 1px solid #bbf7d0;
  border-radius: 6px;
  margin-bottom: 12px;
}

.current-step-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.current-step-content {
  padding: 8px;
  background: white;
  border-radius: 4px;
}

.step-type-badge {
  display: inline-block;
  padding: 1px 6px;
  background: var(--bg-elevated);
  border-radius: 3px;
  font-size: 10px;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.step-action {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.step-desc {
  font-size: 11px;
  color: var(--text-muted);
  margin-top: 4px;
}

.results-section {
  margin-bottom: 12px;
}

.section-title {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.results-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
  max-height: 150px;
  overflow-y: auto;
}

.result-item {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  padding: 8px;
  border-radius: 4px;
  background: var(--bg-elevated);
}

.result-item.result-success {
  border-left: 3px solid #22c55e;
}

.result-item.result-failed {
  border-left: 3px solid #ef4444;
  background: #fef2f2;
}

.result-icon {
  width: 18px;
  height: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 10px;
  border-radius: 50%;
  flex-shrink: 0;
}

.result-success .result-icon {
  background: #dcfce7;
  color: #16a34a;
}

.result-failed .result-icon {
  background: #fee2e2;
  color: #dc2626;
}

.result-content {
  flex: 1;
  min-width: 0;
}

.result-step {
  font-size: 11px;
  font-weight: 500;
  color: var(--text-primary);
}

.result-error {
  font-size: 10px;
  color: #dc2626;
  margin-top: 2px;
}

.result-duration {
  font-size: 10px;
  color: var(--text-muted);
  margin-top: 2px;
}

.execution-stats {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 8px;
  padding: 12px;
  background: var(--bg-elevated);
  border-radius: 6px;
  margin-bottom: 12px;
}

.stat-row {
  text-align: center;
}

.stat-label {
  display: block;
  font-size: 10px;
  color: var(--text-muted);
  margin-bottom: 2px;
}

.stat-value {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.stat-success {
  color: #16a34a;
}

.stat-failed {
  color: #dc2626;
}

.empty-state {
  padding: 24px;
  text-align: center;
}
</style>