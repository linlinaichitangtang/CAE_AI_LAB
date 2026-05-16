/**
 * ConvergenceDiagnosticsPanel.vue — V4.2-004 非线性收敛自动修复面板
 */
<template>
  <div class="convergence-panel">
    <div class="panel-header">
      <h3 class="panel-title">🔧 收敛诊断</h3>
      <div class="diag-status" :class="statusClass">
        {{ statusText }}
      </div>
    </div>

    <!-- 分析状态 -->
    <div v-if="isAnalyzing" class="analyzing-state">
      <span class="spinner">🔍</span>
      <span>正在分析收敛问题...</span>
    </div>

    <!-- 诊断结果 -->
    <div v-else-if="recommendation" class="diag-result">
      <!-- 场景标签 -->
      <div class="scenario-badge" :class="recommendation.primary.scenario">
        <span class="scenario-icon">{{ scenarioIcon(recommendation.primary.scenario) }}</span>
        <span class="scenario-label">{{ scenarioLabel(recommendation.primary.scenario) }}</span>
      </div>

      <!-- 修复概率 -->
      <div class="fix-probability">
        <div class="prob-bar">
          <div
            class="prob-fill"
            :class="probClass(recommendation.estimatedFixProbability)"
            :style="{ width: `${recommendation.estimatedFixProbability * 100}%` }"
          />
        </div>
        <span class="prob-value">
          修复概率 {{ (recommendation.estimatedFixProbability * 100).toFixed(0) }}%
        </span>
      </div>

      <!-- 综合建议 -->
      <div v-if="recommendation.combinedStrategy" class="combined-hint">
        💡 {{ recommendation.combinedStrategy }}
      </div>

      <!-- 主要策略 -->
      <div class="strategy-card primary">
        <div class="strategy-header">
          <span class="strategy-badge">首选</span>
          <span class="strategy-label">{{ recommendation.primary.label }}</span>
        </div>
        <p class="strategy-desc">{{ recommendation.primary.description }}</p>
        <div class="strategy-action">
          <span class="action-icon">⚙️</span>
          <code>{{ recommendation.primary.action }}</code>
        </div>
        <div class="strategy-meta">
          <span class="risk-badge" :class="recommendation.primary.riskLevel">
            风险: {{ riskLabel(recommendation.primary.riskLevel) }}
          </span>
        </div>
        <div class="strategy-actions">
          <button class="btn-apply" @click="applyStrategy('primary')">
            ✓ 一键应用
          </button>
          <button class="btn-copy" @click="copyAction(recommendation.primary.action)">
            📋 复制
          </button>
        </div>
      </div>

      <!-- 备选策略 -->
      <div v-if="recommendation.alternatives.length > 0" class="alternatives-section">
        <h4>备选方案</h4>
        <div
          v-for="(alt, i) in recommendation.alternatives"
          :key="alt.id"
          class="strategy-card alt"
        >
          <div class="strategy-header">
            <span class="strategy-badge alt">{{ i + 2 }}</span>
            <span class="strategy-label">{{ alt.label }}</span>
          </div>
          <p class="strategy-desc">{{ alt.description }}</p>
          <div class="strategy-action">
            <span class="action-icon">⚙️</span>
            <code>{{ alt.action }}</code>
          </div>
          <div class="strategy-actions">
            <button class="btn-apply alt" @click="applyStrategy('alt', i)">
              应用
            </button>
          </div>
        </div>
      </div>

      <!-- 收敛历史图 -->
      <div v-if="historyPoints.length > 0" class="convergence-history">
        <h4>📈 收敛历史</h4>
        <div class="history-chart">
          <svg ref="chartRef" class="chart-svg" :viewBox="`0 0 ${chartWidth} ${chartHeight}`">
            <!-- 网格线 -->
            <g class="grid-lines">
              <line
                v-for="y in gridLines"
                :key="y"
                :x1="padding.left"
                :y1="y"
                :x2="chartWidth - padding.right"
                :y2="y"
                class="grid-line"
              />
            </g>
            <!-- 残差曲线 -->
            <path :d="residualPath" class="curve residual" />
            <!-- 增量曲线 -->
            <path :d="incrementPath" class="curve increment" />
            <!-- 图例 -->
            <g class="legend" :transform="`translate(${chartWidth - padding.right - 100}, ${padding.top})`">
              <line x1="0" y1="0" x2="20" y2="0" class="curve residual" />
              <text x="25" y="4" class="legend-text">残差</text>
              <line x1="0" y1="20" x2="20" y2="20" class="curve increment" />
              <text x="25" y="24" class="legend-text">增量</text>
            </g>
          </svg>
        </div>
      </div>

      <!-- 历史记录 -->
      <div v-if="diagHistory.length > 0" class="history-section">
        <div class="history-toggle" @click="showHistory = !showHistory">
          📋 历史记录 ({{ diagHistory.length }})
          <span>{{ showHistory ? '▼' : '▶' }}</span>
        </div>
        <div v-if="showHistory" class="history-list">
          <div
            v-for="record in diagHistory"
            :key="record.jobId"
            class="history-record"
          >
            <span class="record-time">{{ formatTime(record.timestamp) }}</span>
            <span class="record-scenario">{{ scenarioLabel(record.scenario) }}</span>
            <span class="record-iterations">{{ record.iterationCount }} 次迭代</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else class="empty-state">
      <span class="empty-icon">🔧</span>
      <p>暂无诊断数据</p>
      <p class="empty-hint">仿真失败后将自动触发收敛诊断</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useConvergenceDiagnostics, type ConvergenceScenario, type RepairStrategy, type ConvergenceDiagRecord } from '@/composables/useConvergenceDiagnostics'

const props = defineProps<{
  errorMessage?: string
  residualHistory?: number[]
  jobId?: string
  autoAnalyze?: boolean
}>()

const emit = defineEmits<{
  applyStrategy: [strategy: RepairStrategy, action: 'primary' | 'alt', index?: number]
  generateReport: [report: string]
}>()

const {
  isAnalyzing,
  lastDiag,
  history,
  analyzeConvergenceFailure,
  scenarioLabel,
  parseConvergenceHistory,
  generateDiagReport,
  getAllStrategies
} = useConvergenceDiagnostics()

const recommendation = ref<Awaited<ReturnType<typeof analyzeConvergenceFailure>> | null>(null)
const showHistory = ref(false)
const diagHistory = ref<ConvergenceDiagRecord[]>([])
const chartRef = ref<SVGElement | null>(null)

// 模拟残差历史（实际应从求解器获取）
const mockResidualHistory = computed(() =>
  props.residualHistory || Array.from({ length: 20 }, (_, i) =>
    100 * Math.exp(-0.3 * i) * (1 + 0.2 * Math.sin(i * 0.5))
  )
)

const historyPoints = computed(() => {
  const residuals = mockResidualHistory.value
  return residuals.map((r, i) => ({
    step: Math.floor(i / 10),
    iteration: i % 10,
    residual: r,
    increment: r * (0.1 + Math.random() * 0.2)
  }))
})

// 图表配置
const chartWidth = 380
const chartHeight = 160
const padding = { top: 20, right: 20, bottom: 30, left: 50 }

const gridLines = computed(() => {
  const lines = []
  for (let i = 0; i <= 4; i++) {
    lines.push(padding.top + (chartHeight - padding.top - padding.bottom) * i / 4)
  }
  return lines
})

// 残差曲线路径
const residualPath = computed(() => {
  const points = historyPoints.value
  if (points.length === 0) return ''
  const maxRes = Math.max(...points.map(p => p.residual))
  const minRes = Math.min(...points.map(p => p.residual))
  const range = maxRes - minRes || 1

  const w = chartWidth - padding.left - padding.right
  const h = chartHeight - padding.top - padding.bottom

  return points.map((p, i) => {
    const x = padding.left + (i / (points.length - 1 || 1)) * w
    const y = padding.top + (1 - (p.residual - minRes) / range) * h
    return `${i === 0 ? 'M' : 'L'}${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

// 增量曲线路径
const incrementPath = computed(() => {
  const points = historyPoints.value
  if (points.length === 0) return ''
  const maxRes = Math.max(...points.map(p => p.residual))
  const minRes = Math.min(...points.map(p => p.residual))
  const range = maxRes - minRes || 1

  const w = chartWidth - padding.left - padding.right
  const h = chartHeight - padding.top - padding.bottom

  return points.map((p, i) => {
    const x = padding.left + (i / (points.length - 1 || 1)) * w
    const y = padding.top + (1 - (p.increment - minRes) / range) * h
    return `${i === 0 ? 'M' : 'L'}${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

// 状态
const statusClass = computed(() => {
  if (isAnalyzing.value) return 'analyzing'
  if (recommendation.value) return 'found'
  return 'idle'
})

const statusText = computed(() => {
  if (isAnalyzing.value) return '分析中'
  if (recommendation.value) return '已诊断'
  return '待机'
})

// 概率等级
function probClass(prob: number): string {
  if (prob >= 0.7) return 'high'
  if (prob >= 0.4) return 'medium'
  return 'low'
}

// 风险等级标签
function riskLabel(level: string): string {
  const map: Record<string, string> = { low: '低', medium: '中', high: '高' }
  return map[level] || level
}

// 场景图标
function scenarioIcon(scenario: ConvergenceScenario): string {
  const map: Record<ConvergenceScenario, string> = {
    contact_penetration: '🤝',
    large_strain: '📐',
    material_nonlinearity: '🔴',
    geometric_instability: '⚠️',
    thermal_stiffening: '🌡️',
    mesh_distortion: '🔲',
    ill_conditioned: '📊',
    load_step_too_large: '⏱️',
    excessive_dof: '🔢',
    mixed_convergence: '🔀'
  }
  return map[scenario] || '❓'
}

// 自动分析
watch(() => [props.errorMessage, props.autoAnalyze] as const, async ([msg, auto]) => {
  if (msg && auto) {
    const result = await analyzeConvergenceFailure(msg, props.residualHistory || [], props.jobId)
    recommendation.value = result
    if (lastDiag.value) {
      diagHistory.value = [lastDiag.value, ...diagHistory.value.slice(0, 9)]
    }
  }
}, { immediate: true })

// 应用策略
function applyStrategy(type: 'primary' | 'alt', index = 0) {
  if (!recommendation.value) return
  const strategy = type === 'primary'
    ? recommendation.value.primary
    : recommendation.value.alternatives[index]
  if (strategy) {
    emit('applyStrategy', strategy, type, index)
  }
}

// 复制操作
function copyAction(action: string) {
  navigator.clipboard.writeText(action)
}

// 格式化时间
function formatTime(ts: string): string {
  if (!ts) return '-'
  return new Date(ts).toLocaleTimeString('zh-CN')
}
</script>

<style scoped>
.convergence-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 20px;
  background: white;
  border-radius: 12px;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.panel-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
  margin: 0;
}

.diag-status {
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 600;
}

.diag-status.idle {
  background: #f1f5f9;
  color: #64748b;
}

.diag-status.analyzing {
  background: #fef3c7;
  color: #92400e;
}

.diag-status.found {
  background: #dcfce7;
  color: #166534;
}

.analyzing-state {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 30px;
  color: var(--text-secondary, #64748b);
}

.spinner {
  animation: spin 1s linear infinite;
  font-size: 20px;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* Scenario badge */
.scenario-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 14px;
  border-radius: 16px;
  font-size: 13px;
  font-weight: 600;
}

.scenario-badge.contact_penetration { background: #fef3c7; color: #92400e; }
.scenario-badge.large_strain { background: #dbeafe; color: #1e40af; }
.scenario-badge.material_nonlinearity { background: #fee2e2; color: #991b1b; }
.scenario-badge.geometric_instability { background: #f3e8ff; color: #6b21a8; }
.scenario-badge.thermal_stiffening { background: #ffe4e6; color: #9f1239; }
.scenario-badge.mesh_distortion { background: #fef9c3; color: #854d0e; }
.scenario-badge.ill_conditioned { background: #f1f5f9; color: #475569; }
.scenario-badge.load_step_too_large { background: #e0f2fe; color: #0369a1; }
.scenario-badge.excessive_dof { background: #f1f5f9; color: #475569; }
.scenario-badge.mixed_convergence { background: #f3f4f6; color: #374151; }

.scenario-icon {
  font-size: 16px;
}

/* Fix probability */
.fix-probability {
  display: flex;
  align-items: center;
  gap: 10px;
}

.prob-bar {
  flex: 1;
  height: 8px;
  background: var(--bg-elevated, #f1f5f9);
  border-radius: 4px;
  overflow: hidden;
}

.prob-fill {
  height: 100%;
  border-radius: 4px;
  transition: width 0.5s ease;
}

.prob-fill.high { background: #22c55e; }
.prob-fill.medium { background: #f59e0b; }
.prob-fill.low { background: #ef4444; }

.prob-value {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary, #64748b);
  min-width: 100px;
}

/* Combined hint */
.combined-hint {
  padding: 10px 14px;
  background: #eff6ff;
  border-radius: 8px;
  font-size: 12px;
  color: #1e40af;
}

/* Strategy cards */
.strategy-card {
  padding: 14px;
  border-radius: 10px;
  border: 1px solid var(--border-color, #e2e8f0);
  background: var(--bg-elevated, #f8fafc);
}

.strategy-card.primary {
  border-color: var(--primary-color, #3b82f6);
  background: #eff6ff;
}

.strategy-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.strategy-badge {
  padding: 2px 8px;
  border-radius: 10px;
  font-size: 10px;
  font-weight: 700;
  background: var(--primary-color, #3b82f6);
  color: white;
}

.strategy-badge.alt {
  background: var(--text-muted, #94a3b8);
}

.strategy-label {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
}

.strategy-desc {
  font-size: 12px;
  color: var(--text-secondary, #64748b);
  margin: 0 0 10px 0;
  line-height: 1.5;
}

.strategy-action {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  padding: 8px 10px;
  background: white;
  border-radius: 6px;
  margin-bottom: 10px;
}

.action-icon {
  font-size: 14px;
  flex-shrink: 0;
}

.strategy-action code {
  font-size: 11px;
  font-family: 'Monaco', 'Menlo', monospace;
  color: var(--text-primary, #1e293b);
  line-height: 1.4;
}

.strategy-meta {
  margin-bottom: 10px;
}

.risk-badge {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 10px;
}

.risk-badge.low { background: #dcfce7; color: #166534; }
.risk-badge.medium { background: #fef3c7; color: #92400e; }
.risk-badge.high { background: #fee2e2; color: #991b1b; }

.strategy-actions {
  display: flex;
  gap: 8px;
}

.strategy-actions button {
  padding: 6px 16px;
  border-radius: 6px;
  border: none;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-apply {
  background: var(--primary-color, #3b82f6);
  color: white;
}

.btn-apply.alt {
  background: var(--text-secondary, #64748b);
  color: white;
}

.btn-copy {
  background: white;
  color: var(--text-secondary, #64748b);
  border: 1px solid var(--border-color, #e2e8f0);
}

/* Alternatives */
.alternatives-section {
  margin-top: 12px;
}

.alternatives-section h4 {
  font-size: 12px;
  color: var(--text-muted, #94a3b8);
  margin: 0 0 8px 0;
}

.strategy-card.alt {
  margin-bottom: 8px;
}

/* Convergence history chart */
.convergence-history {
  margin-top: 12px;
}

.convergence-history h4 {
  font-size: 12px;
  color: var(--text-secondary, #64748b);
  margin: 0 0 8px 0;
}

.history-chart {
  background: white;
  border-radius: 8px;
  padding: 8px;
}

.chart-svg {
  width: 100%;
  height: auto;
}

.grid-line {
  stroke: var(--border-color, #e2e8f0);
  stroke-width: 1;
  stroke-dasharray: 3 3;
}

.curve {
  fill: none;
  stroke-width: 2;
}

.curve.residual {
  stroke: #3b82f6;
}

.curve.increment {
  stroke: #f59e0b;
}

.legend-text {
  font-size: 10px;
  fill: var(--text-secondary, #64748b);
}

/* History section */
.history-section {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--border-color, #e2e8f0);
}

.history-toggle {
  font-size: 12px;
  color: var(--text-secondary, #64748b);
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-top: 8px;
}

.history-record {
  display: flex;
  gap: 8px;
  font-size: 11px;
  padding: 6px 10px;
  background: var(--bg-elevated, #f8fafc);
  border-radius: 6px;
}

.record-time {
  color: var(--text-muted, #94a3b8);
  white-space: nowrap;
}

.record-scenario {
  color: var(--text-primary, #1e293b);
  flex: 1;
}

.record-iterations {
  color: var(--text-muted, #94a3b8);
}

/* Empty state */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 40px;
  color: var(--text-muted, #94a3b8);
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 12px;
}

.empty-hint {
  font-size: 12px;
}
</style>