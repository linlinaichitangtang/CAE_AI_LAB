/**
 * ProvenanceTracker.vue — V4.1-006 仿真结果溯源面板
 */
<template>
  <div class="provenance-tracker">
    <div class="panel-header">
      <h3 class="panel-title">📜 结果溯源</h3>
      <div class="search-box">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="搜索记录（名称/操作人/标签）"
        >
      </div>
    </div>

    <!-- 统计概览 -->
    <div class="stats-bar">
      <div class="stat-item">
        <span class="stat-value">{{ records.length }}</span>
        <span class="stat-label">总记录</span>
      </div>
      <div class="stat-item">
        <span class="stat-value">{{ verifiedCount }}</span>
        <span class="stat-label">已验证</span>
      </div>
      <div class="stat-item">
        <span class="stat-value">{{ questionedCount }}</span>
        <span class="stat-label">存疑</span>
      </div>
      <div class="stat-item">
        <span class="stat-value">{{ avgReproducibility }}%</span>
        <span class="stat-label">平均可复现性</span>
      </div>
    </div>

    <!-- 谱系列表 -->
    <div class="lineages-list">
      <div
        v-for="lineage in filteredLineages"
        :key="lineage.resultId"
        class="lineage-card"
        @click="selectedLineage = lineage"
      >
        <div class="lineage-header">
          <div class="lineage-info">
            <span class="lineage-name">{{ lineage.resultName }}</span>
            <span class="lineage-id">ID: {{ lineage.resultId.slice(0, 8) }}</span>
          </div>
          <div class="lineage-scores">
            <span class="score-badge confidence" :class="scoreClass(lineage.confidenceScore)">
              置信度 {{ (lineage.confidenceScore * 100).toFixed(0) }}%
            </span>
            <span class="score-badge reproducibility" :class="scoreClass(lineage.reproducibilityIndex)">
              可复现 {{ (lineage.reproducibilityIndex * 100).toFixed(0) }}%
            </span>
          </div>
        </div>

        <div class="lineage-meta">
          <span class="meta-item">
            👤 {{ lineage.creator.name }}
          </span>
          <span class="meta-item">
            🕐 {{ formatDate(lineage.createdAt) }}
          </span>
          <span class="meta-item">
            🔗 {{ lineage.executionChain.length }} 步操作
          </span>
          <span v-if="lineage.derivedResults.length > 0" class="meta-item">
            📎 {{ lineage.derivedResults.length }} 个派生结果
          </span>
        </div>
      </div>
    </div>

    <!-- 详情面板 -->
    <div v-if="selectedLineage" class="detail-panel">
      <div class="detail-header">
        <h4>📋 {{ selectedLineage.resultName }}</h4>
        <button class="btn-close" @click="selectedLineage = null">✕</button>
      </div>

      <!-- 可复现性报告 -->
      <div v-if="reproducibilityReport" class="reproducibility-section">
        <div class="repo-score" :class="reproducibilityReport.canReproduce ? 'good' : 'poor'">
          <span class="repo-value">{{ reproducibilityReport.score }}</span>
          <span class="repo-label">可复现性评分</span>
          <span class="repo-status">{{ reproducibilityReport.canReproduce ? '✅ 可复现' : '⚠️ 信息不足' }}</span>
        </div>

        <div v-if="reproducibilityReport.missingInfo.length > 0" class="repo-section">
          <h5>❌ 缺失信息</h5>
          <ul>
            <li v-for="(item, i) in reproducibilityReport.missingInfo" :key="i">{{ item }}</li>
          </ul>
        </div>

        <div v-if="reproducibilityReport.warnings.length > 0" class="repo-section">
          <h5>⚠️ 警告</h5>
          <ul>
            <li v-for="(item, i) in reproducibilityReport.warnings" :key="i">{{ item }}</li>
          </ul>
        </div>

        <div class="repo-section">
          <h5>📝 复现步骤</h5>
          <ol>
            <li v-for="(step, i) in reproducibilityReport.requiredSteps" :key="i">{{ step }}</li>
          </ol>
        </div>
      </div>

      <!-- 执行链时间线 -->
      <div class="timeline-section">
        <h5>⏱️ 执行时间线</h5>
        <div class="timeline">
          <div
            v-for="record in selectedLineage.executionChain"
            :key="record.id"
            class="timeline-item"
            :class="record.verificationStatus"
          >
            <div class="timeline-dot" :class="record.action" />
            <div class="timeline-content">
              <div class="timeline-header">
                <span class="timeline-action">{{ actionLabel(record.action) }}</span>
                <span class="timeline-entity">{{ entityLabel(record.entity) }}</span>
                <span class="timeline-time">{{ formatTime(record.timestamp) }}</span>
              </div>
              <p class="timeline-desc">{{ record.description }}</p>
              <div v-if="record.parameters && record.parameters.length > 0" class="timeline-params">
                <span
                  v-for="param in record.parameters"
                  :key="param.name"
                  class="param-chip"
                >
                  {{ param.name }}: {{ param.value }}{{ param.unit ? param.unit : '' }}
                </span>
              </div>
              <div class="timeline-actor">
                👤 {{ record.actor.name }} ({{ record.actor.role }})
              </div>
              <div v-if="record.verificationStatus !== 'unverified'" class="verification-badge" :class="record.verificationStatus">
                {{ record.verificationStatus === 'verified' ? '✓ 已验证' : '? 存疑' }}
                <span v-if="record.verifier">by {{ record.verifier.name }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 参数历史 -->
      <div v-if="selectedLineage.parameterHistory.length > 0" class="params-section">
        <h5>🔧 参数历史</h5>
        <table class="params-table">
          <thead>
            <tr>
              <th>参数</th>
              <th>值</th>
              <th>单位</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(param, i) in selectedLineage.parameterHistory" :key="i">
              <td>{{ param.name }}</td>
              <td>{{ param.value }}</td>
              <td>{{ param.unit || '-' }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- 操作按钮 -->
      <div class="detail-actions">
        <button class="btn-verify" @click="verifySelected('verified')">
          ✓ 标记为已验证
        </button>
        <button class="btn-question" @click="verifySelected('questioned')">
          ? 标记为存疑
        </button>
        <button class="btn-export" @click="exportSelected">
          📤 导出溯源链
        </button>
      </div>
    </div>

    <div v-if="filteredLineages.length === 0" class="empty-state">
      <span class="empty-icon">📜</span>
      <p>暂无溯源记录</p>
      <p class="empty-hint">运行仿真后将自动记录操作历史</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useResultProvenance, type ResultLineage } from '@/composables/useResultProvenance'

const {
  records,
  allLineages,
  verifyResult,
  generateReproducibilityReport,
  exportProvenanceChain
} = useResultProvenance()

const searchQuery = ref('')
const selectedLineage = ref<ResultLineage | null>(null)

const filteredLineages = computed(() => {
  if (!searchQuery.value) return allLineages.value
  const q = searchQuery.value.toLowerCase()
  return allLineages.value.filter(l =>
    l.resultName.toLowerCase().includes(q) ||
    l.creator.name.toLowerCase().includes(q)
  )
})

const verifiedCount = computed(() =>
  records.value.filter(r => r.verificationStatus === 'verified').length
)

const questionedCount = computed(() =>
  records.value.filter(r => r.verificationStatus === 'questioned').length
)

const avgReproducibility = computed(() => {
  if (allLineages.value.length === 0) return 0
  const sum = allLineages.value.reduce((acc, l) => acc + l.reproducibilityIndex, 0)
  return Math.round((sum / allLineages.value.length) * 100)
})

const reproducibilityReport = computed(() => {
  if (!selectedLineage.value) return null
  return generateReproducibilityReport(selectedLineage.value.resultId)
})

function scoreClass(value: number): string {
  if (value >= 0.8) return 'excellent'
  if (value >= 0.6) return 'good'
  if (value >= 0.4) return 'fair'
  return 'poor'
}

function actionLabel(action: string): string {
  const map: Record<string, string> = {
    create: '创建',
    modify: '修改',
    run: '运行',
    export: '导出',
    import: '导入',
    delete: '删除',
    approve: '批准',
    reject: '拒绝'
  }
  return map[action] || action
}

function entityLabel(entity: string): string {
  const map: Record<string, string> = {
    simulation: '仿真',
    geometry: '几何',
    material: '材料',
    mesh: '网格',
    boundary_condition: '边界条件',
    solver_config: '求解器配置',
    result: '结果'
  }
  return map[entity] || entity
}

function formatDate(ts: string): string {
  if (!ts) return '-'
  return new Date(ts).toLocaleDateString('zh-CN')
}

function formatTime(ts: string): string {
  if (!ts) return '-'
  return new Date(ts).toLocaleString('zh-CN')
}

function verifySelected(status: 'verified' | 'questioned') {
  if (!selectedLineage.value) return
  verifyResult(selectedLineage.value.resultId, status)
}

function exportSelected() {
  if (!selectedLineage.value) return
  const json = exportProvenanceChain(selectedLineage.value.resultId)
  const blob = new Blob([json], { type: 'application/json' })
  const link = document.createElement('a')
  link.download = `provenance_${selectedLineage.value.resultId.slice(0, 8)}.json`
  link.href = URL.createObjectURL(blob)
  link.click()
}
</script>

<style scoped>
.provenance-tracker {
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

.search-box input {
  padding: 8px 12px;
  border: 1px solid var(--border-color, #e2e8f0);
  border-radius: 8px;
  font-size: 13px;
  min-width: 200px;
}

.stats-bar {
  display: flex;
  gap: 16px;
  padding: 12px;
  background: var(--bg-elevated, #f8fafc);
  border-radius: 10px;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  min-width: 80px;
}

.stat-value {
  font-size: 20px;
  font-weight: 700;
  color: var(--primary-color, #3b82f6);
}

.stat-label {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
}

.lineages-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 300px;
  overflow-y: auto;
}

.lineage-card {
  padding: 14px;
  border-radius: 10px;
  border: 1px solid var(--border-color, #e2e8f0);
  cursor: pointer;
  transition: all 0.2s;
}

.lineage-card:hover {
  border-color: var(--primary-color, #3b82f6);
  background: var(--bg-elevated, #f8fafc);
}

.lineage-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.lineage-name {
  font-size: 14px;
  font-weight: 600;
}

.lineage-id {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
  margin-left: 8px;
}

.lineage-scores {
  display: flex;
  gap: 6px;
}

.score-badge {
  font-size: 11px;
  padding: 2px 10px;
  border-radius: 10px;
  font-weight: 600;
}

.score-badge.excellent {
  background: #dcfce7;
  color: #166534;
}

.score-badge.good {
  background: #dbeafe;
  color: #1e40af;
}

.score-badge.fair {
  background: #fef3c7;
  color: #92400e;
}

.score-badge.poor {
  background: #fee2e2;
  color: #991b1b;
}

.lineage-meta {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.meta-item {
  font-size: 12px;
  color: var(--text-secondary, #64748b);
}

/* Detail panel */
.detail-panel {
  padding: 16px;
  background: var(--bg-elevated, #f8fafc);
  border-radius: 10px;
  border: 1px solid var(--border-color, #e2e8f0);
}

.detail-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.detail-header h4 {
  margin: 0;
  font-size: 15px;
}

.btn-close {
  background: none;
  border: none;
  font-size: 16px;
  cursor: pointer;
  color: var(--text-muted, #94a3b8);
}

/* Reproducibility */
.reproducibility-section {
  margin-bottom: 16px;
  padding: 14px;
  background: white;
  border-radius: 8px;
}

.repo-score {
  display: flex;
  align-items: baseline;
  gap: 8px;
  margin-bottom: 12px;
}

.repo-value {
  font-size: 32px;
  font-weight: 700;
}

.repo-score.good .repo-value {
  color: #22c55e;
}

.repo-score.poor .repo-value {
  color: #ef4444;
}

.repo-label {
  font-size: 13px;
  color: var(--text-secondary, #64748b);
}

.repo-status {
  margin-left: auto;
  font-size: 12px;
  font-weight: 600;
}

.repo-section {
  margin-top: 10px;
}

.repo-section h5 {
  font-size: 12px;
  margin: 0 0 6px 0;
  color: var(--text-primary, #1e293b);
}

.repo-section ul,
.repo-section ol {
  margin: 0;
  padding-left: 18px;
  font-size: 12px;
  color: var(--text-secondary, #64748b);
  line-height: 1.6;
}

/* Timeline */
.timeline-section {
  margin-bottom: 16px;
}

.timeline-section h5 {
  font-size: 13px;
  margin: 0 0 10px 0;
}

.timeline {
  position: relative;
  padding-left: 20px;
}

.timeline::before {
  content: '';
  position: absolute;
  left: 6px;
  top: 0;
  bottom: 0;
  width: 2px;
  background: var(--border-color, #e2e8f0);
}

.timeline-item {
  position: relative;
  padding-bottom: 16px;
}

.timeline-dot {
  position: absolute;
  left: -18px;
  top: 4px;
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: var(--primary-color, #3b82f6);
  border: 2px solid white;
}

.timeline-dot.create {
  background: #22c55e;
}

.timeline-dot.run {
  background: #3b82f6;
}

.timeline-dot.modify {
  background: #f59e0b;
}

.timeline-dot.export {
  background: #8b5cf6;
}

.timeline-content {
  padding: 10px;
  background: white;
  border-radius: 8px;
  border: 1px solid var(--border-color, #e2e8f0);
}

.timeline-header {
  display: flex;
  gap: 8px;
  align-items: center;
  margin-bottom: 4px;
}

.timeline-action {
  font-size: 12px;
  font-weight: 600;
  color: var(--primary-color, #3b82f6);
}

.timeline-entity {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
}

.timeline-time {
  margin-left: auto;
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
}

.timeline-desc {
  font-size: 12px;
  color: var(--text-secondary, #64748b);
  margin: 4px 0;
}

.timeline-params {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  margin: 6px 0;
}

.param-chip {
  font-size: 10px;
  padding: 2px 8px;
  background: #f1f5f9;
  color: #475569;
  border-radius: 4px;
  font-family: monospace;
}

.timeline-actor {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
}

.verification-badge {
  display: inline-block;
  margin-top: 6px;
  padding: 2px 10px;
  border-radius: 10px;
  font-size: 11px;
  font-weight: 600;
}

.verification-badge.verified {
  background: #dcfce7;
  color: #166534;
}

.verification-badge.questioned {
  background: #fef3c7;
  color: #92400e;
}

/* Params table */
.params-section {
  margin-bottom: 16px;
}

.params-section h5 {
  font-size: 13px;
  margin: 0 0 10px 0;
}

.params-table {
  width: 100%;
  font-size: 12px;
  border-collapse: collapse;
  background: white;
  border-radius: 8px;
  overflow: hidden;
}

.params-table th,
.params-table td {
  padding: 8px;
  text-align: left;
  border-bottom: 1px solid var(--border-color, #e2e8f0);
}

.params-table th {
  background: #f8fafc;
  font-weight: 600;
  color: var(--text-secondary, #64748b);
}

/* Detail actions */
.detail-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.detail-actions button {
  padding: 8px 16px;
  border-radius: 6px;
  border: none;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-verify {
  background: #22c55e;
  color: white;
}

.btn-question {
  background: #f59e0b;
  color: white;
}

.btn-export {
  background: #3b82f6;
  color: white;
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
