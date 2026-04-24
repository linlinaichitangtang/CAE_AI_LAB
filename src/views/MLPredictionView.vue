<template>
  <div class="ml-prediction-view">
    <div class="view-header">
      <h1 class="view-title">🧠 ML 材料性能预测</h1>
      <p class="view-desc">输入材料成分，使用 ML Surrogate Model 快速预测力学/热学性能</p>
    </div>

    <div class="view-content">
      <!-- 左侧: 预测面板 -->
      <div class="main-panel">
        <MaterialPredictionPanel @verify="handleVerify" />
      </div>

      <!-- 右侧: 精度评测 -->
      <div class="side-panel">
        <div class="benchmark-section">
          <h3 class="section-title">📊 精度评测</h3>
          <p class="section-desc">对比 ML 预测 vs CAELab 仿真结果</p>

          <div class="benchmark-controls">
            <button
              class="benchmark-btn"
              @click="runBenchmark"
              :disabled="isBenchmarking"
            >
              {{ isBenchmarking ? '评测中...' : '运行精度评测' }}
            </button>
          </div>

          <div v-if="benchmarkResults.length > 0" class="benchmark-results">
            <table class="benchmark-table">
              <thead>
                <tr>
                  <th>材料</th>
                  <th>属性</th>
                  <th>预测值</th>
                  <th>参考值</th>
                  <th>误差</th>
                  <th>状态</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="(r, i) in benchmarkResults"
                  :key="i"
                  :class="{ fail: !r.passed }"
                >
                  <td>{{ r.materialName }}</td>
                  <td>{{ r.propertyName }}</td>
                  <td>{{ formatBenchmarkValue(r.predictedValue, r.propertyName) }}</td>
                  <td>{{ formatBenchmarkValue(r.referenceValue, r.propertyName) }}</td>
                  <td>{{ r.relativeErrorPercent.toFixed(1) }}%</td>
                  <td>
                    <span :class="r.passed ? 'pass-badge' : 'fail-badge'">
                      {{ r.passed ? '✓ 通过' : '✗ 超标' }}
                    </span>
                  </td>
                </tr>
              </tbody>
            </table>

            <div class="benchmark-summary">
              <span>通过: {{ passedCount }}/{{ benchmarkResults.length }}</span>
              <span>平均误差: {{ avgError.toFixed(1) }}%</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 验证对话框 -->
    <div v-if="verifyResult" class="verify-overlay" @click.self="verifyResult = null">
      <div class="verify-dialog">
        <h3>🔬 CAELab 仿真验证</h3>
        <p>确认使用以下预测参数提交 CAELab 仿真？</p>
        <div class="verify-params">
          <div v-for="pred in verifyResult.predictions" :key="pred.propertyName" class="verify-param">
            <span class="param-name">{{ pred.propertyName }}</span>
            <span class="param-value">{{ pred.predictedValue.toExponential(3) }} {{ pred.unit }}</span>
          </div>
        </div>
        <div class="verify-actions">
          <button class="btn-cancel" @click="verifyResult = null">取消</button>
          <button class="btn-confirm" @click="submitVerification">确认提交仿真</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import MaterialPredictionPanel from '../components/ai/MaterialPredictionPanel.vue'
import { runAccuracyBenchmark, type PredictionResponse, type BenchmarkResult } from '../api/mlPredict'

const verifyResult = ref<PredictionResponse | null>(null)
const benchmarkResults = ref<BenchmarkResult[]>([])
const isBenchmarking = ref(false)

const passedCount = computed(() => benchmarkResults.value.filter(r => r.passed).length)
const avgError = computed(() => {
  if (benchmarkResults.value.length === 0) return 0
  const sum = benchmarkResults.value.reduce((s, r) => s + r.relativeErrorPercent, 0)
  return sum / benchmarkResults.value.length
})

function handleVerify(result: PredictionResponse) {
  verifyResult.value = result
}

function formatBenchmarkValue(value: number, property: string): string {
  if (property === 'poissons_ratio') return value.toFixed(3)
  if (value >= 1e9) return (value / 1e9).toFixed(1) + ' GPa'
  if (value >= 1e6) return (value / 1e6).toFixed(0) + ' MPa'
  return value.toFixed(1)
}

async function runBenchmark() {
  isBenchmarking.value = true
  try {
    benchmarkResults.value = await runAccuracyBenchmark({
      materialNames: ['Q235', '304不锈钢', 'Al6061', 'Ti6Al4V', '纯铜'],
      targetProperties: ['elastic_modulus', 'yield_strength', 'thermal_conductivity'],
    })
  } catch (e) {
    console.error('Benchmark failed:', e)
  } finally {
    isBenchmarking.value = false
  }
}

function submitVerification() {
  // V2.5-004: 通过 Agent 自动提交 CAELab 仿真
  // 这里预留接口，实际逻辑在 Agent 工具中实现
  alert('仿真验证请求已提交（Mock 模式下仅展示流程）')
  verifyResult.value = null
}
</script>

<style scoped>
.ml-prediction-view {
  padding: 20px;
  max-width: 1400px;
  margin: 0 auto;
}

.view-header {
  margin-bottom: 20px;
}
.view-title {
  font-size: 24px;
  font-weight: 700;
  color: var(--text-primary, #cdd6f4);
  margin: 0 0 4px 0;
}
.view-desc {
  font-size: 14px;
  color: var(--text-secondary, #a6adc8);
  margin: 0;
}

.view-content {
  display: grid;
  grid-template-columns: 1fr 380px;
  gap: 16px;
}

.side-panel {
  position: sticky;
  top: 20px;
  align-self: start;
}

.benchmark-section {
  background: var(--bg-secondary, #1e1e2e);
  border: 1px solid var(--border-color, #313244);
  border-radius: 12px;
  padding: 16px;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary, #cdd6f4);
  margin: 0 0 4px 0;
}
.section-desc {
  font-size: 12px;
  color: var(--text-secondary, #a6adc8);
  margin: 0 0 12px 0;
}

.benchmark-btn {
  width: 100%;
  padding: 8px;
  background: var(--bg-primary, #11111b);
  border: 1px solid var(--border-color, #313244);
  border-radius: 6px;
  color: var(--text-primary, #cdd6f4);
  font-size: 13px;
  cursor: pointer;
  margin-bottom: 12px;
}
.benchmark-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.benchmark-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 11px;
}
.benchmark-table th {
  text-align: left;
  padding: 4px 6px;
  color: var(--text-secondary, #a6adc8);
  border-bottom: 1px solid var(--border-color, #313244);
}
.benchmark-table td {
  padding: 4px 6px;
  color: var(--text-primary, #cdd6f4);
  border-bottom: 1px solid rgba(49, 50, 68, 0.5);
}
.benchmark-table tr.fail td {
  color: #f38ba8;
}

.pass-badge {
  color: #a6e3a1;
  font-size: 11px;
}
.fail-badge {
  color: #f38ba8;
  font-size: 11px;
}

.benchmark-summary {
  display: flex;
  justify-content: space-between;
  margin-top: 8px;
  font-size: 12px;
  color: var(--text-secondary, #a6adc8);
}

.verify-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}
.verify-dialog {
  background: var(--bg-secondary, #1e1e2e);
  border: 1px solid var(--border-color, #313244);
  border-radius: 12px;
  padding: 20px;
  max-width: 400px;
  width: 90%;
}
.verify-dialog h3 {
  margin: 0 0 8px 0;
  font-size: 16px;
  color: var(--text-primary, #cdd6f4);
}
.verify-dialog p {
  margin: 0 0 12px 0;
  font-size: 13px;
  color: var(--text-secondary, #a6adc8);
}

.verify-params {
  background: var(--bg-primary, #11111b);
  border-radius: 8px;
  padding: 10px;
  margin-bottom: 16px;
}
.verify-param {
  display: flex;
  justify-content: space-between;
  padding: 4px 0;
  font-size: 13px;
}
.param-name {
  color: var(--text-secondary, #a6adc8);
}
.param-value {
  color: var(--accent-color, #89b4fa);
  font-family: monospace;
}

.verify-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}
.btn-cancel {
  padding: 6px 16px;
  background: transparent;
  border: 1px solid var(--border-color, #313244);
  border-radius: 6px;
  color: var(--text-secondary, #a6adc8);
  cursor: pointer;
}
.btn-confirm {
  padding: 6px 16px;
  background: rgba(166, 227, 161, 0.15);
  border: 1px solid rgba(166, 227, 161, 0.3);
  border-radius: 6px;
  color: #a6e3a1;
  cursor: pointer;
}

@media (max-width: 900px) {
  .view-content {
    grid-template-columns: 1fr;
  }
}
</style>
