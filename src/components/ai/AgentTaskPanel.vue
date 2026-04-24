<template>
  <div class="agent-task-panel" v-if="visible">
    <!-- 面板头部 -->
    <div class="panel-header">
      <div class="header-left">
        <span class="agent-icon">🤖</span>
        <span class="panel-title">Agent 任务</span>
        <span v-if="plan" class="status-badge" :class="plan.status">
          {{ statusText }}
        </span>
      </div>
      <div class="header-right">
        <button
          v-if="plan && plan.status === 'executing'"
          class="btn btn-sm"
          @click="$emit('pause')"
          title="暂停"
        >
          ⏸️
        </button>
        <button
          v-if="plan && plan.status === 'paused'"
          class="btn btn-sm"
          @click="$emit('resume')"
          title="恢复"
        >
          ▶️
        </button>
        <button
          v-if="plan && (plan.status === 'executing' || plan.status === 'paused')"
          class="btn btn-sm btn-danger"
          @click="$emit('cancel')"
          title="取消"
        >
          ✕
        </button>
        <button class="btn btn-sm" @click="$emit('close')" title="关闭">
          ×
        </button>
      </div>
    </div>

    <!-- 任务信息 -->
    <div v-if="plan" class="plan-info">
      <div class="plan-query">📋 {{ plan.userQuery }}</div>
      <div class="plan-intent">
        意图: <span class="intent-tag">{{ intentText }}</span>
        <span class="confidence">置信度: {{ (plan.intent.confidence * 100).toFixed(0) }}%</span>
      </div>
    </div>

    <!-- 进度条 -->
    <div v-if="plan" class="progress-section">
      <div class="progress-bar-container">
        <div
          class="progress-bar-fill"
          :style="{ width: progressPercent + '%' }"
          :class="progressClass"
        ></div>
      </div>
      <div class="progress-text">
        {{ completedSteps }}/{{ plan.subTasks.length }} 步骤完成
        ({{ progressPercent }}%)
      </div>
    </div>

    <!-- 步骤列表 -->
    <div v-if="plan" class="steps-list">
      <div
        v-for="(step, index) in plan.subTasks"
        :key="step.id"
        class="step-item"
        :class="{
          'is-active': step.status === 'running',
          'is-done': step.status === 'done',
          'is-failed': step.status === 'failed',
          'is-waiting': step.status === 'waiting_confirmation',
          'is-skipped': step.status === 'skipped'
        }"
      >
        <div class="step-indicator">
          <span v-if="step.status === 'done'" class="step-icon">✅</span>
          <span v-else-if="step.status === 'running'" class="step-icon spinning">⚙️</span>
          <span v-else-if="step.status === 'failed'" class="step-icon">❌</span>
          <span v-else-if="step.status === 'waiting_confirmation'" class="step-icon">⚠️</span>
          <span v-else-if="step.status === 'skipped'" class="step-icon">⏭️</span>
          <span v-else class="step-number">{{ index + 1 }}</span>
        </div>
        <div class="step-content">
          <div class="step-name">{{ step.name }}</div>
          <div class="step-description">{{ step.description }}</div>
          <div v-if="step.toolName" class="step-tool">
            🔧 {{ step.toolName }}
          </div>
          <div v-if="step.error" class="step-error">
            ❌ {{ step.error }}
          </div>
          <div v-if="step.actualTime" class="step-time">
            ⏱️ {{ step.actualTime.toFixed(1) }}s
          </div>
          <!-- 确认按钮 -->
          <button
            v-if="step.status === 'waiting_confirmation'"
            class="btn btn-sm btn-primary"
            @click="$emit('confirm', step.id)"
          >
            ✅ 确认执行
          </button>
        </div>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else class="empty-state">
      <span>暂无任务</span>
    </div>

    <!-- 统计信息 -->
    <div v-if="showMetrics" class="metrics-section">
      <div class="metrics-title">📊 Agent 统计</div>
      <div class="metrics-grid" v-if="metrics">
        <div class="metric-item">
          <div class="metric-value">{{ metrics?.completedTasks ?? 0 }}</div>
          <div class="metric-label">已完成</div>
        </div>
        <div class="metric-item">
          <div class="metric-value">{{ metrics?.failedTasks ?? 0 }}</div>
          <div class="metric-label">失败</div>
        </div>
        <div class="metric-item">
          <div class="metric-value">{{ metrics?.totalToolCalls ?? 0 }}</div>
          <div class="metric-label">工具调用</div>
        </div>
        <div class="metric-item">
          <div class="metric-value">{{ toolSuccessRate }}%</div>
          <div class="metric-label">成功率</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { TaskPlan, AgentMetrics } from '../../agent/types'

const props = defineProps<{
  visible: boolean
  plan: TaskPlan | null
  metrics: AgentMetrics | null
  showMetrics?: boolean
}>()

defineEmits<{
  pause: []
  resume: []
  cancel: []
  close: []
  confirm: [subTaskId: string]
}>()

const statusTextMap: Record<string, string> = {
  planning: '📋 规划中',
  executing: '⚙️ 执行中',
  paused: '⏸️ 已暂停',
  completed: '✅ 已完成',
  failed: '❌ 失败',
  cancelled: '🚫 已取消'
}

const intentTextMap: Record<string, string> = {
  modeling: '建模',
  simulation: '仿真',
  postprocess: '后处理',
  code: '代码',
  analysis: '分析',
  notes: '笔记',
  parametric: '参数化',
  optimization: '优化',
  validation: '验证',
  qa: '问答',
  unknown: '未知'
}

const statusText = computed(() => {
  if (!props.plan) return ''
  return statusTextMap[props.plan.status] || props.plan.status
})

const intentText = computed(() => {
  if (!props.plan) return ''
  return intentTextMap[props.plan.intent.intent] || props.plan.intent.intent
})

const completedSteps = computed(() => {
  if (!props.plan) return 0
  return props.plan.subTasks.filter(s => s.status === 'done' || s.status === 'skipped').length
})

const progressPercent = computed(() => {
  if (!props.plan || props.plan.subTasks.length === 0) return 0
  return Math.round((completedSteps.value / props.plan.subTasks.length) * 100)
})

const progressClass = computed(() => {
  if (!props.plan) return ''
  if (props.plan.status === 'failed') return 'progress-danger'
  if (props.plan.status === 'completed') return 'progress-success'
  return 'progress-active'
})

const toolSuccessRate = computed(() => {
  if (!props.metrics || props.metrics.totalToolCalls === 0) return 0
  return Math.round(props.metrics.toolCallSuccessRate * 100)
})
</script>

<style scoped>
.agent-task-panel {
  background: var(--bg-secondary, #1e1e2e);
  border: 1px solid var(--border-color, #313244);
  border-radius: 12px;
  padding: 16px;
  margin-bottom: 12px;
  font-size: 13px;
  color: var(--text-primary, #cdd6f4);
  max-height: 500px;
  overflow-y: auto;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.agent-icon {
  font-size: 18px;
}

.panel-title {
  font-weight: 600;
  font-size: 14px;
}

.status-badge {
  padding: 2px 8px;
  border-radius: 10px;
  font-size: 11px;
  font-weight: 500;
}

.status-badge.planning { background: #89b4fa33; color: #89b4fa; }
.status-badge.executing { background: #a6e3a133; color: #a6e3a1; }
.status-badge.paused { background: #f9e2af33; color: #f9e2af; }
.status-badge.completed { background: #a6e3a133; color: #a6e3a1; }
.status-badge.failed { background: #f38ba833; color: #f38ba8; }
.status-badge.cancelled { background: #6c708633; color: #6c7086; }

.header-right {
  display: flex;
  gap: 4px;
}

.btn {
  background: transparent;
  border: 1px solid var(--border-color, #313244);
  color: var(--text-primary, #cdd6f4);
  border-radius: 6px;
  cursor: pointer;
  padding: 4px 8px;
  font-size: 12px;
  transition: all 0.2s;
}

.btn:hover { background: var(--bg-hover, #313244); }
.btn-sm { padding: 2px 6px; font-size: 11px; }
.btn-danger { color: #f38ba8; border-color: #f38ba844; }
.btn-danger:hover { background: #f38ba822; }
.btn-primary { color: #89b4fa; border-color: #89b4fa44; }
.btn-primary:hover { background: #89b4fa22; }

.plan-info {
  margin-bottom: 12px;
  padding: 8px;
  background: var(--bg-tertiary, #181825);
  border-radius: 8px;
}

.plan-query {
  margin-bottom: 4px;
  font-weight: 500;
}

.plan-intent {
  font-size: 12px;
  color: var(--text-secondary, #a6adc8);
}

.intent-tag {
  background: #89b4fa22;
  color: #89b4fa;
  padding: 1px 6px;
  border-radius: 4px;
  margin-right: 8px;
}

.confidence {
  font-size: 11px;
}

.progress-section {
  margin-bottom: 12px;
}

.progress-bar-container {
  height: 6px;
  background: var(--bg-tertiary, #181825);
  border-radius: 3px;
  overflow: hidden;
}

.progress-bar-fill {
  height: 100%;
  border-radius: 3px;
  transition: width 0.5s ease;
}

.progress-active { background: linear-gradient(90deg, #89b4fa, #a6e3a1); }
.progress-success { background: #a6e3a1; }
.progress-danger { background: #f38ba8; }

.progress-text {
  font-size: 11px;
  color: var(--text-secondary, #a6adc8);
  margin-top: 4px;
}

.steps-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.step-item {
  display: flex;
  gap: 10px;
  padding: 8px;
  border-radius: 8px;
  transition: background 0.2s;
}

.step-item.is-active {
  background: #89b4fa11;
  border-left: 2px solid #89b4fa;
}

.step-item.is-done { opacity: 0.7; }
.step-item.is-failed { border-left: 2px solid #f38ba8; }
.step-item.is-waiting { border-left: 2px solid #f9e2af; background: #f9e2af08; }

.step-indicator {
  flex-shrink: 0;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.step-icon { font-size: 14px; }
.step-number {
  font-size: 11px;
  color: var(--text-secondary, #a6adc8);
  font-weight: 600;
}

.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.step-content {
  flex: 1;
  min-width: 0;
}

.step-name {
  font-weight: 500;
  font-size: 13px;
}

.step-description {
  font-size: 11px;
  color: var(--text-secondary, #a6adc8);
  margin-top: 2px;
}

.step-tool {
  font-size: 11px;
  color: #89b4fa;
  margin-top: 2px;
}

.step-error {
  font-size: 11px;
  color: #f38ba8;
  margin-top: 2px;
}

.step-time {
  font-size: 11px;
  color: var(--text-secondary, #a6adc8);
  margin-top: 2px;
}

.empty-state {
  text-align: center;
  padding: 20px;
  color: var(--text-secondary, #a6adc8);
}

.metrics-section {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--border-color, #313244);
}

.metrics-title {
  font-weight: 600;
  margin-bottom: 8px;
}

.metrics-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}

.metric-item {
  text-align: center;
  padding: 8px;
  background: var(--bg-tertiary, #181825);
  border-radius: 8px;
}

.metric-value {
  font-size: 18px;
  font-weight: 700;
  color: #89b4fa;
}

.metric-label {
  font-size: 11px;
  color: var(--text-secondary, #a6adc8);
  margin-top: 2px;
}
</style>
