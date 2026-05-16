/**
 * ComplianceCheckPanel.vue — V4.1-001 合规检查面板
 * 显示仿真结果的合规性判定
 */
<template>
  <div class="compliance-check-panel">
    <div class="panel-header">
      <h3 class="panel-title">📋 合规检查</h3>
      <div class="standard-selector">
        <label
          v-for="std in availableStandards"
          :key="std.id"
          class="std-checkbox"
        >
          <input
            type="checkbox"
            :checked="activeStandards.includes(std.id)"
            @change="toggleStandard(std.id)"
          >
          <span>{{ std.name }}</span>
        </label>
      </div>
    </div>

    <div v-if="isChecking" class="checking-state">
      <span class="spinner">⏳</span>
      <span>正在检查合规性...</span>
    </div>

    <div v-else-if="lastReport" class="report-content">
      <!-- 总体状态 -->
      <div class="overall-status" :class="lastReport.overallStatus">
        <span class="status-icon">{{ statusIcon }}</span>
        <span class="status-text">{{ statusText }}</span>
        <span class="status-detail">
          {{ lastReport.summary.passed }} 通过 / {{ lastReport.summary.failed }} 失败 / {{ lastReport.summary.total }} 总计
        </span>
      </div>

      <!-- 结果列表 -->
      <div class="results-list">
        <div
          v-for="result in lastReport.results"
          :key="result.ruleId"
          class="result-item"
          :class="result.severity"
        >
          <div class="result-header">
            <span class="result-badge" :class="result.passed ? 'pass' : 'fail'">
              {{ result.passed ? '✓' : '✗' }}
            </span>
            <span class="result-standard">{{ result.rule.standardName }}</span>
            <span class="result-clause">条款 {{ result.rule.clause }}</span>
          </div>
          <p class="result-desc">{{ result.rule.description }}</p>
          <div class="result-values">
            <span class="actual-value">
              实际值: <strong>{{ result.actualValue.toFixed(2) }}</strong> {{ result.rule.unit }}
            </span>
            <span class="threshold-value">
              阈值: {{ result.rule.operator }} {{ result.rule.threshold }} {{ result.rule.unit }}
            </span>
            <span class="margin-value" :class="result.margin >= 0 ? 'safe' : 'unsafe'">
              裕度: {{ (result.margin * 100).toFixed(1) }}%
            </span>
          </div>
          <p class="result-message">{{ result.message }}</p>
        </div>
      </div>
    </div>

    <div v-else class="empty-state">
      <span class="empty-icon">📋</span>
      <p>暂无合规检查数据</p>
      <p class="empty-hint">运行仿真后将自动进行合规检查</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useComplianceCheck } from '@/composables/useComplianceCheck'

const {
  isChecking,
  lastReport,
  activeStandards,
  toggleStandard
} = useComplianceCheck()

const availableStandards = [
  { id: 'BMS7-368E', name: 'BMS7-368E (航空)' },
  { id: 'VDA-2300', name: 'VDA 2300 (汽车)' },
  { id: 'GB/T-6398', name: 'GB/T 6398 (国标)' },
  { id: 'GB/T-50017', name: 'GB/T 50017 (钢结构)' }
]

const statusIcon = computed(() => {
  switch (lastReport.value?.overallStatus) {
    case 'pass': return '✅'
    case 'fail': return '❌'
    case 'partial': return '⚠️'
    default: return '📋'
  }
})

const statusText = computed(() => {
  switch (lastReport.value?.overallStatus) {
    case 'pass': return '全部通过'
    case 'fail': return '未通过'
    case 'partial': return '部分通过'
    default: return '待检查'
  }
})
</script>

<style scoped>
.compliance-check-panel {
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
  flex-wrap: wrap;
  gap: 12px;
}

.panel-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
  margin: 0;
}

.standard-selector {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.std-checkbox {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--text-secondary, #64748b);
  cursor: pointer;
}

.checking-state {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 40px;
  color: var(--text-secondary, #64748b);
}

.spinner {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.overall-status {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  border-radius: 8px;
  font-size: 14px;
}

.overall-status.pass {
  background: #dcfce7;
  color: #166534;
}

.overall-status.fail {
  background: #fee2e2;
  color: #991b1b;
}

.overall-status.partial {
  background: #fef3c7;
  color: #92400e;
}

.status-icon {
  font-size: 20px;
}

.status-text {
  font-weight: 600;
}

.status-detail {
  margin-left: auto;
  font-size: 12px;
  opacity: 0.8;
}

.results-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.result-item {
  padding: 12px;
  border-radius: 8px;
  border-left: 4px solid var(--border-color, #e2e8f0);
  background: var(--bg-elevated, #f8fafc);
}

.result-item.critical {
  border-left-color: #ef4444;
  background: #fef2f2;
}

.result-item.warning {
  border-left-color: #f59e0b;
  background: #fffbeb;
}

.result-item.info {
  border-left-color: #22c55e;
  background: #f0fdf4;
}

.result-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.result-badge {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 600;
}

.result-badge.pass {
  background: #22c55e;
  color: white;
}

.result-badge.fail {
  background: #ef4444;
  color: white;
}

.result-standard {
  font-weight: 600;
  font-size: 13px;
  color: var(--text-primary, #1e293b);
}

.result-clause {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
}

.result-desc {
  font-size: 12px;
  color: var(--text-secondary, #64748b);
  margin: 4px 0;
}

.result-values {
  display: flex;
  gap: 16px;
  margin: 8px 0;
  font-size: 12px;
}

.actual-value strong {
  color: var(--primary-color, #3b82f6);
}

.margin-value.safe {
  color: #22c55e;
}

.margin-value.unsafe {
  color: #ef4444;
}

.result-message {
  font-size: 12px;
  color: var(--text-secondary, #64748b);
  margin: 4px 0 0 0;
}

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
