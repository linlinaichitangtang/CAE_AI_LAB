/**
 * ResultConfidenceBadge.vue — V4.1-003 结果置信度显示
 * 显示每个仿真结果的误差估计和置信区间
 */
<template>
  <div class="result-confidence-badge">
    <div class="confidence-header" @click="isExpanded = !isExpanded">
      <span class="confidence-icon">📊</span>
      <span class="confidence-label">结果置信度</span>
      <span class="confidence-score" :class="scoreClass">
        {{ (report.overallConfidence * 100).toFixed(0) }}%
      </span>
      <span class="expand-icon">{{ isExpanded ? '▼' : '▶' }}</span>
    </div>

    <div v-if="isExpanded" class="confidence-details">
      <!-- 置信区间 -->
      <div class="confidence-interval">
        <span class="interval-label">置信区间 (95%)</span>
        <div class="interval-bar">
          <div class="interval-range" :style="intervalStyle">
            <span class="interval-value">{{ report.confidenceInterval.lower.toFixed(1) }} ~ {{ report.confidenceInterval.upper.toFixed(1) }}</span>
          </div>
        </div>
        <span class="interval-unit">{{ report.confidenceInterval.unit }}</span>
      </div>

      <!-- 各维度详情 -->
      <div class="dimensions-list">
        <div
          v-for="dim in report.dimensions"
          :key="dim.name"
          class="dimension-item"
        >
          <div class="dim-header">
            <span class="dim-name">{{ dim.label }}</span>
            <span class="dim-value" :class="getScoreClass(dim.value)">
              {{ (dim.value * 100).toFixed(0) }}%
            </span>
            <span class="dim-error">±{{ dim.errorEstimate.toFixed(1) }}%</span>
          </div>
          <div class="dim-bar">
            <div class="dim-fill" :class="getScoreClass(dim.value)" :style="{ width: `${dim.value * 100}%` }" />
          </div>
          <p class="dim-desc">{{ dim.description }}</p>
        </div>
      </div>

      <!-- 建议 -->
      <div v-if="report.recommendations.length > 0" class="recommendations">
        <h4>💡 改进建议</h4>
        <ul>
          <li v-for="(rec, i) in report.recommendations" :key="i">{{ rec }}</li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { ConfidenceReport } from '@/composables/useResultConfidence'

const props = defineProps<{
  report: ConfidenceReport
}>()

const isExpanded = ref(false)

const scoreClass = computed(() => getScoreClass(props.report.overallConfidence))

const intervalStyle = computed(() => {
  const { lower, upper } = props.report.confidenceInterval
  const center = (lower + upper) / 2
  const range = upper - lower
  const scale = center > 0 ? 100 / (center * 2) : 1
  const left = Math.max(0, (center - range / 2) * scale)
  const width = Math.min(100, range * scale)
  return { left: `${left}%`, width: `${width}%` }
})

function getScoreClass(value: number): string {
  if (value >= 0.8) return 'excellent'
  if (value >= 0.6) return 'good'
  if (value >= 0.4) return 'fair'
  return 'poor'
}
</script>

<style scoped>
.result-confidence-badge {
  background: white;
  border-radius: 10px;
  border: 1px solid var(--border-color, #e2e8f0);
  overflow: hidden;
}

.confidence-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  cursor: pointer;
  transition: background 0.2s;
}

.confidence-header:hover {
  background: var(--bg-elevated, #f8fafc);
}

.confidence-icon {
  font-size: 16px;
}

.confidence-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
  flex: 1;
}

.confidence-score {
  font-size: 14px;
  font-weight: 700;
  padding: 2px 8px;
  border-radius: 10px;
}

.confidence-score.excellent {
  background: #dcfce7;
  color: #166534;
}

.confidence-score.good {
  background: #dbeafe;
  color: #1e40af;
}

.confidence-score.fair {
  background: #fef3c7;
  color: #92400e;
}

.confidence-score.poor {
  background: #fee2e2;
  color: #991b1b;
}

.expand-icon {
  font-size: 10px;
  color: var(--text-muted, #94a3b8);
}

.confidence-details {
  padding: 0 14px 14px;
  border-top: 1px solid var(--border-color, #e2e8f0);
}

.confidence-interval {
  padding: 12px 0;
}

.interval-label {
  font-size: 12px;
  color: var(--text-muted, #94a3b8);
}

.interval-bar {
  position: relative;
  height: 24px;
  background: var(--bg-elevated, #f1f5f9);
  border-radius: 12px;
  margin: 6px 0;
  overflow: hidden;
}

.interval-range {
  position: absolute;
  top: 0;
  bottom: 0;
  background: var(--primary-color, #3b82f6);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 60px;
}

.interval-value {
  font-size: 11px;
  color: white;
  font-weight: 600;
  white-space: nowrap;
}

.interval-unit {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
}

.dimensions-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.dimension-item {
  padding: 8px 0;
  border-bottom: 1px solid var(--border-color, #e2e8f0);
}

.dimension-item:last-child {
  border-bottom: none;
}

.dim-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}

.dim-name {
  font-size: 12px;
  color: var(--text-secondary, #64748b);
}

.dim-value {
  font-size: 12px;
  font-weight: 600;
}

.dim-value.excellent { color: #22c55e; }
.dim-value.good { color: #3b82f6; }
.dim-value.fair { color: #f59e0b; }
.dim-value.poor { color: #ef4444; }

.dim-error {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
}

.dim-bar {
  height: 6px;
  background: var(--bg-elevated, #f1f5f9);
  border-radius: 3px;
  overflow: hidden;
}

.dim-fill {
  height: 100%;
  border-radius: 3px;
  transition: width 0.5s ease;
}

.dim-fill.excellent { background: #22c55e; }
.dim-fill.good { background: #3b82f6; }
.dim-fill.fair { background: #f59e0b; }
.dim-fill.poor { background: #ef4444; }

.dim-desc {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
  margin: 4px 0 0 0;
  line-height: 1.4;
}

.recommendations {
  margin-top: 12px;
  padding: 10px;
  background: #eff6ff;
  border-radius: 8px;
}

.recommendations h4 {
  font-size: 12px;
  color: var(--primary-color, #3b82f6);
  margin: 0 0 6px 0;
}

.recommendations ul {
  margin: 0;
  padding-left: 16px;
}

.recommendations li {
  font-size: 11px;
  color: var(--text-secondary, #475569);
  line-height: 1.6;
}
</style>