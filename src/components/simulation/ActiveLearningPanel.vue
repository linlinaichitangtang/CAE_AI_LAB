<template>
  <div class="al-panel">
    <!-- 覆盖度概览 -->
    <div class="section">
      <h3 class="section-title">📊 数据覆盖度</h3>
      <div v-if="coverage" class="coverage-grid">
        <div class="cov-card">
          <span class="cov-val">{{ (coverage.overallCoverage * 100).toFixed(0) }}%</span>
          <span class="cov-label">综合覆盖度</span>
          <div class="cov-bar"><div class="cov-fill" :style="{ width: (coverage.overallCoverage * 100) + '%' }"></div></div>
        </div>
        <div class="cov-card">
          <span class="cov-val">{{ (coverage.compositionCoverage * 100).toFixed(0) }}%</span>
          <span class="cov-label">成分空间</span>
          <div class="cov-bar"><div class="cov-fill comp" :style="{ width: (coverage.compositionCoverage * 100) + '%' }"></div></div>
        </div>
        <div class="cov-card">
          <span class="cov-val">{{ (coverage.processingCoverage * 100).toFixed(0) }}%</span>
          <span class="cov-label">工艺空间</span>
          <div class="cov-bar"><div class="cov-fill proc" :style="{ width: (coverage.processingCoverage * 100) + '%' }"></div></div>
        </div>
        <div class="cov-card">
          <span class="cov-val">{{ coverage.totalRecords }}</span>
          <span class="cov-label">总记录数</span>
        </div>
      </div>
      <div class="quality-badge" :class="(coverage?.qualityScore ?? 0) > 70 ? 'good' : (coverage?.qualityScore ?? 0) > 40 ? 'medium' : 'low'">
        数据质量: {{ coverage?.qualityScore?.toFixed(0) ?? 'N/A' }}/100
      </div>
    </div>

    <!-- 数据盲区 -->
    <div class="section">
      <h3 class="section-title">🔍 数据盲区</h3>
      <div class="blind-spots">
        <div v-for="(spot, i) in coverage?.blindSpots" :key="i" class="spot-card" :class="'priority-' + spot.priority">
          <div class="spot-header">
            <span class="spot-rank">P{{ spot.priority }}</span>
            <span class="spot-elements">{{ spot.elements.join(' + ') }}</span>
            <span class="spot-improve">+{{ (spot.expectedImprovement * 100).toFixed(0) }}%</span>
          </div>
          <p class="spot-desc">{{ spot.description }}</p>
        </div>
      </div>
    </div>

    <!-- 主动学习推荐 -->
    <div class="section">
      <h3 class="section-title">🎯 主动学习推荐</h3>
      <div class="al-controls">
        <div class="form-row">
          <label>目标属性</label>
          <select v-model="alRequest.targetProperty" class="form-select">
            <option value="elastic_modulus">弹性模量</option>
            <option value="yield_strength">屈服强度</option>
            <option value="thermal_conductivity">热导率</option>
          </select>
        </div>
        <div class="form-row">
          <label>采样策略</label>
          <select v-model="alRequest.strategy" class="form-select">
            <option value="bayesian">贝叶斯优化</option>
            <option value="greedy">贪心策略</option>
            <option value="random">随机采样</option>
          </select>
        </div>
        <button class="btn-primary" @click="runRecommendation" :disabled="isRecommending">
          {{ isRecommending ? '分析中...' : '⚡ 获取推荐' }}
        </button>
      </div>

      <div v-if="alResult" class="rec-results">
        <div class="rec-summary">
          <span>推荐 {{ alResult.recommendations.length }} 个仿真点</span>
          <span>预期覆盖度: {{ (alResult.currentCoverage * 100).toFixed(0) }}% → {{ (alResult.expectedCoverage * 100).toFixed(0) }}%</span>
          <span>预期精度提升: +{{ (alResult.expectedImprovement * 100).toFixed(0) }}%</span>
        </div>
        <div class="rec-list">
          <div v-for="rec in alResult.recommendations" :key="rec.rank" class="rec-card">
            <div class="rec-header">
              <span class="rec-rank">#{{ rec.rank }}</span>
              <span class="rec-name">{{ rec.materialName }}</span>
              <span class="rec-gain">信息增益: {{ (rec.expectedInformationGain * 100).toFixed(1) }}%</span>
            </div>
            <div class="rec-composition">
              <span v-for="(val, el) in rec.composition" :key="el" class="comp-tag">
                {{ el }}: {{ val.toFixed(1) }}%
              </span>
            </div>
            <p class="rec-reason">{{ rec.reason }}</p>
            <button class="btn-verify" @click="runClosedLoop(rec)">🔬 执行验证</button>
          </div>
        </div>
      </div>
    </div>

    <!-- 实验报告 -->
    <div class="section">
      <h3 class="section-title">📋 实验报告</h3>
      <button class="btn-sm" @click="loadReport">生成报告</button>
      <div v-if="report" class="report-content">
        <div class="report-header">
          <h4>{{ report.title }}</h4>
          <span class="report-date">{{ report.generatedAt.slice(0, 10) }}</span>
        </div>
        <div class="report-stats">
          <div class="stat-item">
            <span class="stat-val">{{ report.newDataCount }}</span>
            <span class="stat-label">新增数据</span>
          </div>
          <div class="stat-item">
            <span class="stat-val">{{ report.accuracyImprovementPercent.toFixed(1) }}%</span>
            <span class="stat-label">精度提升</span>
          </div>
          <div class="stat-item">
            <span class="stat-val">{{ ((report.currentCoverage - report.initialCoverage) * 100).toFixed(0) }}pp</span>
            <span class="stat-label">覆盖度提升</span>
          </div>
          <div class="stat-item">
            <span class="stat-val">{{ report.recommendationHistory.length }}</span>
            <span class="stat-label">验证轮次</span>
          </div>
        </div>
        <div class="report-history">
          <table class="history-table">
            <thead><tr><th>轮次</th><th>材料</th><th>ML预测</th><th>CAELab</th><th>误差</th></tr></thead>
            <tbody>
              <tr v-for="rec in report.recommendationHistory" :key="rec.round">
                <td>{{ rec.round }}</td>
                <td>{{ rec.materialName }}</td>
                <td>{{ formatGpa(rec.mlPrediction) }}</td>
                <td>{{ formatGpa(rec.caelabResult) }}</td>
                <td>{{ formatGpa(rec.error) }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import {
  analyzeDataCoverage,
  getActiveLearningRecommendations,
  runClosedLoopVerification,
  generateActiveLearningReport,
  type CoverageAnalysis,
  type ActiveLearningRecommendation,
  type ActiveLearningReport,
  type RecommendedPoint,
} from '../../api/materialDataPlatform'

const coverage = ref<CoverageAnalysis | null>(null)
const isRecommending = ref(false)
const alResult = ref<ActiveLearningRecommendation | null>(null)
const report = ref<ActiveLearningReport | null>(null)

const alRequest = reactive({
  targetProperty: 'elastic_modulus',
  numRecommendations: 5,
  strategy: 'bayesian' as 'bayesian' | 'greedy' | 'random',
})

function formatGpa(v: number): string {
  return (v / 1e9).toFixed(1) + ' GPa'
}

async function loadCoverage() {
  try { coverage.value = await analyzeDataCoverage() } catch (e) { console.error(e) }
}

async function runRecommendation() {
  isRecommending.value = true
  alResult.value = null
  try {
    alResult.value = await getActiveLearningRecommendations(alRequest)
  } catch (e) { console.error(e) } finally { isRecommending.value = false }
}

async function runClosedLoop(rec: RecommendedPoint) {
  try {
    const result = await runClosedLoopVerification({
      recommendationId: String(rec.rank),
      materialName: rec.materialName,
      composition: rec.composition,
      processingParams: rec.processingParams,
      targetProperty: alRequest.targetProperty,
    })
    alert(`闭环验证完成 (Mock)\nML预测: ${formatGpa(result.mlPredictedValue ?? 0)}\nCAELab: ${formatGpa(result.caelabSimulatedValue ?? 0)}\n误差: ${formatGpa(result.error ?? 0)}\n覆盖度变化: +${((result.coverageChange ?? 0) * 100).toFixed(1)}pp`)
  } catch (e) { console.error(e) }
}

async function loadReport() {
  try { report.value = await generateActiveLearningReport() } catch (e) { console.error(e) }
}

onMounted(loadCoverage)
</script>

<style scoped>
.al-panel { display: flex; flex-direction: column; gap: 12px; }

.section {
  background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #313244);
  border-radius: 12px; padding: 16px;
}
.section-title { font-size: 14px; font-weight: 600; color: var(--text-primary, #cdd6f4); margin: 0 0 12px; }

.coverage-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; margin-bottom: 12px; }
.cov-card { text-align: center; }
.cov-val { display: block; font-size: 22px; font-weight: 700; color: var(--accent-color, #89b4fa); }
.cov-label { font-size: 11px; color: var(--text-secondary, #a6adc8); }
.cov-bar { height: 4px; background: var(--bg-primary, #11111b); border-radius: 2px; margin-top: 6px; }
.cov-fill { height: 100%; background: var(--accent-color, #89b4fa); border-radius: 2px; transition: width 0.3s; }
.cov-fill.comp { background: #cba6f7; }
.cov-fill.proc { background: #fab387; }

.quality-badge {
  display: inline-block; padding: 4px 12px; border-radius: 6px; font-size: 13px; font-weight: 600;
}
.quality-badge.good { background: rgba(166, 227, 161, 0.15); color: #a6e3a1; }
.quality-badge.medium { background: rgba(249, 226, 175, 0.15); color: #f9e2af; }
.quality-badge.low { background: rgba(243, 139, 168, 0.15); color: #f38ba8; }

.blind-spots { display: flex; flex-direction: column; gap: 8px; }
.spot-card {
  background: var(--bg-primary, #11111b); border: 1px solid var(--border-color, #313244);
  border-radius: 8px; padding: 10px; border-left: 3px solid var(--text-secondary, #a6adc8);
}
.spot-card.priority-1 { border-left-color: #f38ba8; }
.spot-card.priority-2 { border-left-color: #f9e2af; }
.spot-card.priority-3 { border-left-color: #89b4fa; }
.spot-header { display: flex; align-items: center; gap: 8px; margin-bottom: 4px; }
.spot-rank { font-size: 12px; font-weight: 700; color: #f38ba8; }
.spot-elements { font-size: 12px; color: var(--text-primary, #cdd6f4); flex: 1; }
.spot-improve { font-size: 11px; color: #a6e3a1; }
.spot-desc { font-size: 12px; color: var(--text-secondary, #a6adc8); margin: 0; }

.al-controls {
  background: var(--bg-primary, #11111b); border: 1px solid var(--border-color, #313244);
  border-radius: 8px; padding: 12px; margin-bottom: 12px;
}
.form-row { display: flex; align-items: center; gap: 8px; margin-bottom: 8px; }
.form-row label { font-size: 12px; color: var(--text-secondary, #a6adc8); min-width: 80px; }
.form-select {
  flex: 1; background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #313244);
  border-radius: 4px; color: var(--text-primary, #cdd6f4); padding: 4px 8px; font-size: 13px;
}
.btn-primary {
  width: 100%; padding: 8px; background: linear-gradient(135deg, #89b4fa, #74c7ec);
  border: none; border-radius: 6px; color: #11111b; font-size: 13px; font-weight: 600; cursor: pointer;
}
.btn-primary:disabled { opacity: 0.5; cursor: not-allowed; }
.btn-sm {
  padding: 4px 10px; background: transparent; border: 1px solid var(--border-color, #313244);
  border-radius: 4px; color: var(--text-secondary, #a6adc8); font-size: 12px; cursor: pointer;
}

.rec-summary {
  display: flex; gap: 16px; padding: 8px 0; font-size: 12px; color: var(--text-secondary, #a6adc8);
  border-bottom: 1px solid var(--border-color, #313244); margin-bottom: 10px;
}
.rec-list { display: flex; flex-direction: column; gap: 8px; }
.rec-card {
  background: var(--bg-primary, #11111b); border: 1px solid var(--border-color, #313244);
  border-radius: 8px; padding: 10px;
}
.rec-header { display: flex; align-items: center; gap: 8px; margin-bottom: 6px; }
.rec-rank { font-size: 14px; font-weight: 700; color: var(--accent-color, #89b4fa); }
.rec-name { font-size: 13px; font-weight: 600; color: var(--text-primary, #cdd6f4); flex: 1; }
.rec-gain { font-size: 11px; color: #a6e3a1; }
.rec-composition { display: flex; gap: 4px; flex-wrap: wrap; margin-bottom: 6px; }
.comp-tag {
  font-size: 11px; background: rgba(137, 180, 250, 0.1); color: #89b4fa;
  padding: 1px 6px; border-radius: 4px;
}
.rec-reason { font-size: 12px; color: var(--text-secondary, #a6adc8); margin: 0 0 8px; }
.btn-verify {
  width: 100%; padding: 6px; background: rgba(166, 227, 161, 0.15);
  border: 1px solid rgba(166, 227, 161, 0.3); border-radius: 6px;
  color: #a6e3a1; font-size: 12px; cursor: pointer;
}

.report-content { margin-top: 12px; }
.report-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
.report-header h4 { margin: 0; font-size: 14px; color: var(--text-primary, #cdd6f4); }
.report-date { font-size: 12px; color: var(--text-secondary, #a6adc8); }
.report-stats { display: flex; gap: 16px; margin-bottom: 12px; }
.stat-item { text-align: center; }
.stat-val { display: block; font-size: 18px; font-weight: 700; color: var(--accent-color, #89b4fa); }
.stat-label { font-size: 10px; color: var(--text-secondary, #a6adc8); }

.history-table { width: 100%; border-collapse: collapse; font-size: 12px; }
.history-table th { text-align: left; padding: 4px 6px; color: var(--text-secondary, #a6adc8); border-bottom: 1px solid var(--border-color, #313244); }
.history-table td { padding: 4px 6px; color: var(--text-primary, #cdd6f4); border-bottom: 1px solid rgba(49, 50, 68, 0.5); }
</style>
