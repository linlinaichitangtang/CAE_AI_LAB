/**
 * ParametricBatchPanel.vue — V4.2-005 参数化批处理面板
 */
<template>
  <div class="parametric-batch-panel">
    <div class="panel-header">
      <h3 class="panel-title">🔬 参数化批处理</h3>
      <div class="job-count">{{ batchJobs.length }} 个任务</div>
    </div>

    <!-- 任务选择 -->
    <div class="job-tabs">
      <button
        v-for="job in batchJobs"
        :key="job.id"
        class="job-tab"
        :class="{ active: activeJobId === job.id }"
        @click="activeJobId = job.id"
      >
        <span class="job-name">{{ job.name }}</span>
        <span class="job-progress">{{ job.completedCount }}/{{ job.totalCount }}</span>
      </button>
      <button class="job-tab new" @click="showNewJobDialog = true">
        + 新建
      </button>
    </div>

    <!-- 参数配置区 -->
    <div v-if="!activeJob" class="empty-state">
      <span class="empty-icon">🔬</span>
      <p>暂无批处理任务</p>
      <button class="btn-create" @click="showNewJobDialog = true">创建新任务</button>
    </div>

    <!-- 活跃任务详情 -->
    <div v-else class="job-detail">
      <!-- 进度条 -->
      <div class="progress-section">
        <div class="progress-bar">
          <div
            class="progress-fill"
            :class="{ completed: activeJob.status === 'completed' }"
            :style="{ width: `${(activeJob.completedCount / activeJob.totalCount) * 100}%` }"
          />
        </div>
        <div class="progress-stats">
          <span class="stat completed">{{ activeJob.completedCount }} 成功</span>
          <span class="stat failed">{{ activeJob.failedCount }} 失败</span>
          <span class="stat pending">{{ activeJob.totalCount - activeJob.completedCount - activeJob.failedCount }} 待运行</span>
        </div>
      </div>

      <!-- 控制按钮 -->
      <div class="control-buttons">
        <button
          v-if="activeJob.status === 'pending' || activeJob.status === 'paused'"
          class="btn-start"
          @click="startBatch"
        >
          ▶️ 开始运行
        </button>
        <button
          v-if="activeJob.status === 'running'"
          class="btn-pause"
          @click="pauseBatch"
        >
          ⏸️ 暂停
        </button>
        <button class="btn-export" @click="exportCSV">
          📤 导出 CSV
        </button>
        <button class="btn-view-chart" @click="showChart = !showChart">
          📊 查看图表
        </button>
      </div>

      <!-- 参数组合列表 -->
      <div class="params-section">
        <h4>参数组合 ({{ activeJob.totalCount }} 组)</h4>
        <div class="params-table">
          <table>
            <thead>
              <tr>
                <th>#</th>
                <th
                  v-for="paramName in paramNames"
                  :key="paramName"
                >
                  {{ paramName }}
                </th>
                <th>状态</th>
                <th>结果</th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="result in activeJob.results"
                :key="result.caseIndex"
                :class="result.status"
              >
                <td class="case-index">{{ result.caseIndex + 1 }}</td>
                <td
                  v-for="paramName in paramNames"
                  :key="paramName"
                  class="param-value"
                >
                  {{ result.parameterSet[paramName]?.toFixed(2) || '-' }}
                </td>
                <td class="status-cell">
                  <span class="status-badge" :class="result.status">
                    {{ statusLabel(result.status) }}
                  </span>
                </td>
                <td class="result-cell">
                  <template v-if="result.outputMetrics">
                    <span class="metric">σ={{ result.outputMetrics.maxStress?.toFixed(0) || '-' }}</span>
                    <span class="metric">δ={{ result.outputMetrics.maxDisplacement?.toFixed(3) || '-' }}</span>
                  </template>
                  <span v-else-if="result.error" class="error-msg">{{ result.error }}</span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- 敏感度图表 -->
      <div v-if="showChart && summary" class="chart-section">
        <h4>📊 参数敏感度排名</h4>
        <div class="sensitivity-bars">
          <div
            v-for="item in summary.sensitivityRanking"
            :key="item.parameter"
            class="sensitivity-item"
          >
            <span class="param-name">{{ item.parameter }}</span>
            <div class="sensitivity-bar">
              <div
                class="sensitivity-fill"
                :style="{ width: `${item.sensitivity * 100}%` }"
              />
            </div>
            <span class="sensitivity-value">{{ (item.sensitivity * 100).toFixed(1) }}%</span>
          </div>
        </div>

        <!-- 指标统计 -->
        <div v-if="summary.bestCase" class="best-case">
          <h5>🏆 最优案例</h5>
          <div class="best-params">
            <span
              v-for="(val, key) in summary.bestCase.params"
              :key="key"
              class="param-chip"
            >
              {{ key }}: {{ val }}
            </span>
          </div>
          <p>安全系数: {{ summary.bestCase.value.toFixed(3) }}</p>
        </div>

        <!-- 指标分布 -->
        <div v-if="summary.metrics.maxStress" class="metrics-summary">
          <h5>📈 指标统计</h5>
          <div class="metric-cards">
            <div v-if="summary.metrics.maxStress" class="metric-card">
              <span class="metric-label">最大应力 (MPa)</span>
              <span class="metric-stats">
                {{ summary.metrics.maxStress.min.toFixed(0) }} ~ {{ summary.metrics.maxStress.max.toFixed(0) }}
                (μ={{ summary.metrics.maxStress.mean.toFixed(0) }}, σ={{ summary.metrics.maxStress.std.toFixed(1) }})
              </span>
            </div>
            <div v-if="summary.metrics.safetyFactor" class="metric-card">
              <span class="metric-label">安全系数</span>
              <span class="metric-stats">
                {{ summary.metrics.safetyFactor.min.toFixed(2) }} ~ {{ summary.metrics.safetyFactor.max.toFixed(2) }}
                (μ={{ summary.metrics.safetyFactor.mean.toFixed(2) }})
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 新建任务对话框 -->
    <div v-if="showNewJobDialog" class="dialog-overlay" @click.self="showNewJobDialog = false">
      <div class="dialog">
        <div class="dialog-header">
          <h4>创建参数化批处理任务</h4>
          <button class="btn-close" @click="showNewJobDialog = false">✕</button>
        </div>

        <div class="dialog-body">
          <!-- 任务名称 -->
          <div class="form-group">
            <label>任务名称</label>
            <input v-model="newJobName" type="text" placeholder="输入任务名称">
          </div>

          <!-- DOE 类型选择 -->
          <div class="form-group">
            <label>实验设计类型</label>
            <select v-model="selectedDOEType" class="doe-select">
              <option value="full_factorial">全因子设计 (Full Factorial)</option>
              <option value="fractional_factorial">部分因子设计 (2^(k-1))</option>
              <option value="box_behnken">Box-Behnken 设计</option>
              <option value="central_composite">中心复合设计 (CCD)</option>
              <option value="latin_hypercube">拉丁超立方设计</option>
              <option value="linear">线性扫描</option>
            </select>
          </div>

          <!-- 参数数量 -->
          <div class="form-group">
            <label>扫描参数数量</label>
            <select v-model="paramCount" class="param-count-select">
              <option v-for="n in 5" :key="n" :value="n">{{ n }} 个参数</option>
            </select>
          </div>

          <!-- 参数配置 -->
          <div class="params-config">
            <div
              v-for="i in paramCount"
              :key="i"
              class="param-config"
            >
              <h5>参数 {{ i }}</h5>
              <div class="param-fields">
                <input v-model="paramConfigs[i - 1].name" type="text" placeholder="参数名" class="field-name">
                <input v-model.number="paramConfigs[i - 1].start" type="number" placeholder="起始值" class="field-range">
                <input v-model.number="paramConfigs[i - 1].end" type="number" placeholder="结束值" class="field-range">
                <select v-model="paramConfigs[i - 1].mode" class="field-mode">
                  <option value="linear">线性</option>
                  <option value="log">对数</option>
                </select>
              </div>
            </div>
          </div>

          <!-- DOE 高级选项 -->
          <div v-if="selectedDOEType !== 'linear'" class="advanced-options">
            <div class="form-group">
              <label>因子水平数</label>
              <select v-model.number="doeLevels" class="levels-select">
                <option :value="2">2 水平</option>
                <option :value="3">3 水平</option>
                <option :value="5">5 水平</option>
              </select>
            </div>
          </div>

          <!-- 预估运行次数 -->
          <div class="estimate">
            <span class="estimate-label">预估运行次数:</span>
            <span class="estimate-value">{{ estimatedRuns }} 组</span>
          </div>
        </div>

        <div class="dialog-footer">
          <button class="btn-cancel" @click="showNewJobDialog = false">取消</button>
          <button class="btn-create" @click="createNewJob" :disabled="isCreating">
            {{ isCreating ? '生成中...' : '创建并生成参数组合' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive } from 'vue'
import { useParametricBatch, type DOEType } from '@/composables/useParametricBatch'

const {
  batchJobs,
  activeJobId,
  activeJob,
  currentProgress,
  createBatchJob,
  generateLinearScan,
  generateDOE,
  updateJobStatus,
  generateSummary,
  exportResultsCSV
} = useParametricBatch()

const showNewJobDialog = ref(false)
const showChart = ref(false)
const isCreating = ref(false)
const newJobName = ref('参数扫描')
const selectedDOEType = ref<DOEType | 'linear'>('full_factorial')
const paramCount = ref(2)
const doeLevels = ref(3)
const summary = ref<ReturnType<typeof generateSummary> | null>(null)

const paramConfigs = reactive(
  Array.from({ length: 5 }, () => ({
    name: '',
    start: 0,
    end: 100,
    mode: 'linear' as 'linear' | 'log'
  }))
)

// 预估运行次数
const estimatedRuns = computed(() => {
  const count = paramCount.value
  if (selectedDOEType.value === 'linear') {
    return Math.pow(10, count) // 简化为 10^count 估算
  }
  switch (selectedDOEType.value) {
    case 'full_factorial': return Math.pow(doeLevels.value, count)
    case 'fractional_factorial': return Math.pow(2, count - 1)
    case 'box_behnken': return 2 * count * (count - 1) / 2 + 1
    case 'central_composite': return Math.pow(2, count) + 2 * count + 1
    case 'latin_hypercube': return doeLevels.value
    default: return Math.pow(doeLevels.value, count)
  }
})

// 参数名列表
const paramNames = computed(() => {
  if (!activeJob.value || activeJob.value.results.length === 0) return []
  return Object.keys(activeJob.value.results[0].parameterSet)
})

// 创建新任务
async function createNewJob() {
  isCreating.value = true
  try {
    const ranges = paramConfigs.slice(0, paramCount.value).map((c, i) => ({
      parameterId: c.name || `param${i + 1}`,
      mode: c.mode,
      start: c.start,
      end: c.end
    }))

    let paramSets: Array<Record<string, number>>
    if (selectedDOEType.value === 'linear') {
      paramSets = generateLinearScan(ranges)
    } else {
      paramSets = generateDOE(ranges, {
        type: selectedDOEType.value as any,
        levels: doeLevels.value,
        centerPoints: 1
      })
    }

    const job = createBatchJob(newJobName.value, paramSets)
    activeJobId.value = job.id

    showNewJobDialog.value = false
    resetNewJobForm()
  } finally {
    isCreating.value = false
  }
}

function resetNewJobForm() {
  newJobName.value = '参数扫描'
  selectedDOEType.value = 'full_factorial'
  paramCount.value = 2
  doeLevels.value = 3
  paramConfigs.forEach(c => {
    c.name = ''
    c.start = 0
    c.end = 100
    c.mode = 'linear'
  })
}

// 开始运行
function startBatch() {
  if (!activeJob.value) return
  updateJobStatus(activeJob.value.id, 'running')
}

// 暂停运行
function pauseBatch() {
  if (!activeJob.value) return
  updateJobStatus(activeJob.value.id, 'paused')
}

// 导出 CSV
function exportCSV() {
  if (!activeJob.value) return
  const csv = exportResultsCSV(activeJob.value)
  const blob = new Blob([csv], { type: 'text/csv' })
  const link = document.createElement('a')
  link.download = `${activeJob.value.name}_results_${Date.now()}.csv`
  link.href = URL.createObjectURL(blob)
  link.click()
}

// 状态标签
function statusLabel(status: string): string {
  const map: Record<string, string> = {
    pending: '待运行',
    running: '运行中',
    success: '成功',
    failed: '失败',
    skipped: '跳过'
  }
  return map[status] || status
}
</script>

<style scoped>
.parametric-batch-panel {
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

.job-count {
  font-size: 12px;
  color: var(--text-muted, #94a3b8);
}

/* Job tabs */
.job-tabs {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.job-tab {
  padding: 8px 16px;
  border-radius: 8px;
  border: 1px solid var(--border-color, #e2e8f0);
  background: white;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
}

.job-tab.active {
  border-color: var(--primary-color, #3b82f6);
  background: #eff6ff;
}

.job-tab.new {
  border-style: dashed;
  color: var(--text-muted, #94a3b8);
}

.job-name {
  font-weight: 600;
}

.job-progress {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
}

/* Progress section */
.progress-section {
  margin-bottom: 12px;
}

.progress-bar {
  height: 8px;
  background: var(--bg-elevated, #f1f5f9);
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 8px;
}

.progress-fill {
  height: 100%;
  background: var(--primary-color, #3b82f6);
  border-radius: 4px;
  transition: width 0.3s ease;
}

.progress-fill.completed {
  background: #22c55e;
}

.progress-stats {
  display: flex;
  gap: 16px;
  font-size: 12px;
}

.progress-stats .stat {
  padding: 2px 8px;
  border-radius: 4px;
}

.progress-stats .completed {
  background: #dcfce7;
  color: #166534;
}

.progress-stats .failed {
  background: #fee2e2;
  color: #991b1b;
}

.progress-stats .pending {
  background: var(--bg-elevated, #f1f5f9);
  color: var(--text-muted, #94a3b8);
}

/* Control buttons */
.control-buttons {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  margin-bottom: 16px;
}

.control-buttons button {
  padding: 8px 16px;
  border-radius: 6px;
  border: none;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-start {
  background: #22c55e;
  color: white;
}

.btn-pause {
  background: #f59e0b;
  color: white;
}

.btn-export {
  background: var(--primary-color, #3b82f6);
  color: white;
}

.btn-view-chart {
  background: #8b5cf6;
  color: white;
}

/* Params table */
.params-section {
  margin-bottom: 16px;
}

.params-section h4 {
  font-size: 13px;
  color: var(--text-secondary, #64748b);
  margin: 0 0 8px 0;
}

.params-table {
  overflow-x: auto;
  max-height: 300px;
  overflow-y: auto;
}

.params-table table {
  width: 100%;
  font-size: 12px;
  border-collapse: collapse;
}

.params-table th,
.params-table td {
  padding: 8px;
  text-align: left;
  border-bottom: 1px solid var(--border-color, #e2e8f0);
  white-space: nowrap;
}

.params-table th {
  background: #f8fafc;
  font-weight: 600;
  color: var(--text-secondary, #64748b);
  position: sticky;
  top: 0;
}

.params-table tr.running {
  background: #fef3c7;
}

.params-table tr.success {
  background: #f0fdf4;
}

.params-table tr.failed {
  background: #fef2f2;
}

.case-index {
  color: var(--text-muted, #94a3b8);
  width: 40px;
}

.param-value {
  font-family: monospace;
}

.status-badge {
  padding: 2px 8px;
  border-radius: 10px;
  font-size: 11px;
  font-weight: 600;
}

.status-badge.pending {
  background: #f1f5f9;
  color: #64748b;
}

.status-badge.running {
  background: #fef3c7;
  color: #92400e;
}

.status-badge.success {
  background: #dcfce7;
  color: #166534;
}

.status-badge.failed {
  background: #fee2e2;
  color: #991b1b;
}

.result-cell {
  display: flex;
  gap: 8px;
}

.metric {
  font-size: 11px;
  color: var(--text-secondary, #64748b);
}

.error-msg {
  font-size: 11px;
  color: #991b1b;
}

/* Sensitivity chart */
.chart-section {
  padding: 14px;
  background: var(--bg-elevated, #f8fafc);
  border-radius: 10px;
}

.chart-section h4 {
  font-size: 13px;
  color: var(--text-primary, #1e293b);
  margin: 0 0 12px 0;
}

.sensitivity-bars {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.sensitivity-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.param-name {
  font-size: 12px;
  color: var(--text-secondary, #64748b);
  min-width: 80px;
  font-family: monospace;
}

.sensitivity-bar {
  flex: 1;
  height: 16px;
  background: var(--bg-surface, white);
  border-radius: 8px;
  overflow: hidden;
}

.sensitivity-fill {
  height: 100%;
  background: linear-gradient(90deg, #3b82f6, #8b5cf6);
  border-radius: 8px;
  transition: width 0.5s ease;
}

.sensitivity-value {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
  min-width: 45px;
  text-align: right;
}

.best-case {
  margin-top: 16px;
  padding: 12px;
  background: #f0fdf4;
  border-radius: 8px;
}

.best-case h5 {
  font-size: 12px;
  margin: 0 0 8px 0;
  color: #166534;
}

.best-params {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-bottom: 8px;
}

.param-chip {
  font-size: 11px;
  padding: 2px 8px;
  background: white;
  border-radius: 4px;
  font-family: monospace;
}

.best-case p {
  font-size: 12px;
  color: #166534;
  margin: 0;
}

.metrics-summary {
  margin-top: 12px;
}

.metrics-summary h5 {
  font-size: 12px;
  margin: 0 0 8px 0;
}

.metric-cards {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.metric-card {
  padding: 10px;
  background: white;
  border-radius: 6px;
}

.metric-label {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
  display: block;
  margin-bottom: 4px;
}

.metric-stats {
  font-size: 12px;
  color: var(--text-primary, #1e293b);
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

.btn-create {
  margin-top: 12px;
  padding: 10px 24px;
  background: var(--primary-color, #3b82f6);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
}

/* Dialog */
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.dialog {
  background: white;
  border-radius: 16px;
  width: 480px;
  max-height: 80vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color, #e2e8f0);
}

.dialog-header h4 {
  margin: 0;
  font-size: 15px;
}

.btn-close {
  background: none;
  border: none;
  font-size: 18px;
  cursor: pointer;
  color: var(--text-muted, #94a3b8);
}

.dialog-body {
  padding: 20px;
  overflow-y: auto;
  flex: 1;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
  margin-bottom: 6px;
}

.form-group input,
.form-group select {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--border-color, #e2e8f0);
  border-radius: 8px;
  font-size: 13px;
}

.params-config {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-top: 16px;
}

.param-config {
  padding: 12px;
  background: var(--bg-elevated, #f8fafc);
  border-radius: 8px;
}

.param-config h5 {
  font-size: 12px;
  margin: 0 0 8px 0;
  color: var(--text-secondary, #64748b);
}

.param-fields {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 80px;
  gap: 8px;
}

.param-fields input,
.param-fields select {
  padding: 8px;
  font-size: 12px;
}

.estimate {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: #eff6ff;
  border-radius: 8px;
  margin-top: 16px;
}

.estimate-label {
  font-size: 13px;
  color: var(--text-secondary, #64748b);
}

.estimate-value {
  font-size: 14px;
  font-weight: 700;
  color: var(--primary-color, #3b82f6);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 16px 20px;
  border-top: 1px solid var(--border-color, #e2e8f0);
}

.btn-cancel {
  padding: 8px 16px;
  background: white;
  border: 1px solid var(--border-color, #e2e8f0);
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
}
</style>