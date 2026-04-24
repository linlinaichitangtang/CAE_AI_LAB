<template>
  <div class="archive-view">
    <div class="view-header">
      <h1 class="view-title">📦 仿真历史归档</h1>
      <p class="view-desc">所有仿真结果自动归档，支持查询、过滤和导出</p>
    </div>

    <!-- 统计概览 -->
    <div v-if="stats" class="stats-bar">
      <div class="stat-item">
        <span class="stat-value">{{ stats.totalRecords }}</span>
        <span class="stat-label">总记录</span>
      </div>
      <div v-for="(count, type) in stats.bySimulationType" :key="type" class="stat-item">
        <span class="stat-value">{{ count }}</span>
        <span class="stat-label">{{ type }}</span>
      </div>
    </div>

    <!-- 过滤栏 -->
    <div class="filter-bar">
      <select v-model="filter.simulationType" class="filter-select" @change="loadArchives">
        <option value="">全部类型</option>
        <option value="static">静力学</option>
        <option value="modal">模态分析</option>
        <option value="thermal">热分析</option>
        <option value="buckling">屈曲分析</option>
        <option value="frequency_response">频响分析</option>
        <option value="transient">瞬态动力学</option>
        <option value="contact">接触分析</option>
        <option value="cfd">CFD</option>
      </select>
      <input
        v-model="filter.materialName"
        placeholder="材料名称过滤"
        class="filter-input"
        @input="loadArchives"
      />
      <input
        v-model="filter.dateFrom"
        type="date"
        class="filter-input"
        @change="loadArchives"
      />
      <span class="filter-sep">~</span>
      <input
        v-model="filter.dateTo"
        type="date"
        class="filter-input"
        @change="loadArchives"
      />
      <div class="filter-actions">
        <button class="export-btn" @click="handleExport('json')">导出 JSON</button>
        <button class="export-btn" @click="handleExport('csv')">导出 CSV</button>
      </div>
    </div>

    <!-- 归档列表 -->
    <div class="archive-list">
      <div v-if="archives.length === 0 && !isLoading" class="empty-state">
        暂无仿真归档记录
      </div>
      <div
        v-for="archive in archives"
        :key="archive.id"
        class="archive-card"
      >
        <div class="card-top">
          <div class="card-type-badge" :class="archive.simulationType">
            {{ getTypeLabel(archive.simulationType) }}
          </div>
          <span class="card-date">{{ formatDate(archive.archivedAt) }}</span>
        </div>
        <div class="card-body">
          <div class="card-info-row">
            <span class="info-label">材料:</span>
            <span class="info-value">{{ archive.materialName || '未指定' }}</span>
          </div>
          <div class="card-info-row">
            <span class="info-label">求解器:</span>
            <span class="info-value">{{ archive.solverName || 'N/A' }}</span>
          </div>
          <div class="card-info-row">
            <span class="info-label">耗时:</span>
            <span class="info-value">{{ archive.solveTimeSec ? archive.solveTimeSec.toFixed(1) + 's' : 'N/A' }}</span>
          </div>
          <div class="card-info-row">
            <span class="info-label">收敛:</span>
            <span class="info-value" :class="archive.converged ? 'converged' : 'not-converged'">
              {{ archive.converged === true ? '✓ 是' : archive.converged === false ? '✗ 否' : 'N/A' }}
            </span>
          </div>
          <div v-if="parsedResults" class="card-results">
            <div v-for="(value, key) in parsedResults" :key="key" class="result-item">
              <span class="result-key">{{ key }}</span>
              <span class="result-value">{{ formatResultValue(value) }}</span>
            </div>
          </div>
        </div>
        <div class="card-actions">
          <button class="action-btn" @click="viewDetail(archive)">查看详情</button>
          <button class="action-btn danger" @click="handleDelete(archive.id)">删除</button>
        </div>
      </div>
    </div>

    <!-- 详情对话框 -->
    <div v-if="detailArchive" class="detail-overlay" @click.self="detailArchive = null">
      <div class="detail-dialog">
        <div class="detail-header">
          <h3>仿真详情</h3>
          <button class="close-btn" @click="detailArchive = null">✕</button>
        </div>
        <div class="detail-body">
          <div class="detail-section">
            <h4>基本信息</h4>
            <div class="detail-row"><span>ID:</span><span>{{ detailArchive.id }}</span></div>
            <div class="detail-row"><span>类型:</span><span>{{ detailArchive.simulationType }}</span></div>
            <div class="detail-row"><span>材料:</span><span>{{ detailArchive.materialName || 'N/A' }}</span></div>
            <div class="detail-row"><span>归档时间:</span><span>{{ detailArchive.archivedAt }}</span></div>
          </div>
          <div class="detail-section">
            <h4>输入参数</h4>
            <pre class="json-block">{{ formatJson(detailArchive.inputParams) }}</pre>
          </div>
          <div class="detail-section">
            <h4>输出结果</h4>
            <pre class="json-block">{{ formatJson(detailArchive.outputResults) }}</pre>
          </div>
          <div v-if="detailArchive.meshStats" class="detail-section">
            <h4>网格统计</h4>
            <pre class="json-block">{{ formatJson(detailArchive.meshStats) }}</pre>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import {
  querySimulationArchives,
  getArchiveStatistics,
  exportSimulationArchives,
  deleteSimulationArchive,
  type SimulationArchive,
  type ArchiveStats,
  type ArchiveFilter,
} from '../api/simulationArchive'

const archives = ref<SimulationArchive[]>([])
const stats = ref<ArchiveStats | null>(null)
const isLoading = ref(false)
const detailArchive = ref<SimulationArchive | null>(null)

const filter = reactive<ArchiveFilter>({
  simulationType: '',
  materialName: '',
  dateFrom: '',
  dateTo: '',
  limit: 50,
  offset: 0,
})

const typeLabels: Record<string, string> = {
  static: '静力学',
  modal: '模态',
  thermal: '热分析',
  buckling: '屈曲',
  frequency_response: '频响',
  transient: '瞬态',
  contact: '接触',
  cfd: 'CFD',
}

function getTypeLabel(type: string): string {
  return typeLabels[type] || type
}

function formatDate(dateStr: string): string {
  try {
    const d = new Date(dateStr)
    return d.toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' })
  } catch {
    return dateStr
  }
}

function formatJson(str: string): string {
  try {
    return JSON.stringify(JSON.parse(str), null, 2)
  } catch {
    return str
  }
}

function formatResultValue(value: unknown): string {
  if (typeof value === 'number') {
    if (value >= 1e9) return (value / 1e9).toFixed(2) + ' GPa'
    if (value >= 1e6) return (value / 1e6).toFixed(1) + ' MPa'
    if (value >= 1e3) return (value / 1e3).toFixed(1) + ' kPa'
    return value.toFixed(2)
  }
  return String(value)
}

const parsedResults = computed(() => {
  if (!detailArchive.value) return null
  return null
})

async function loadArchives() {
  isLoading.value = true
  try {
    const f: ArchiveFilter = { ...filter }
    if (!f.simulationType) delete f.simulationType
    if (!f.materialName) delete f.materialName
    if (!f.dateFrom) delete f.dateFrom
    if (!f.dateTo) delete f.dateTo
    archives.value = await querySimulationArchives(f)
  } catch (e) {
    console.error('加载归档失败:', e)
  } finally {
    isLoading.value = false
  }
}

async function loadStats() {
  try {
    stats.value = await getArchiveStatistics()
  } catch (e) {
    console.error('加载统计失败:', e)
  }
}

function viewDetail(archive: SimulationArchive) {
  detailArchive.value = archive
}

async function handleDelete(id: string) {
  if (!confirm('确认删除此归档记录？')) return
  try {
    await deleteSimulationArchive(id)
    archives.value = archives.value.filter((a: SimulationArchive) => a.id !== id)
    await loadStats()
  } catch (e) {
    console.error('删除失败:', e)
  }
}

async function handleExport(format: 'json' | 'csv') {
  try {
    const result = await exportSimulationArchives({ format, filter: { ...filter } })
    alert(`导出成功: ${result.recordCount} 条记录\n文件: ${result.filePath}\n大小: ${(result.fileSizeBytes / 1024).toFixed(1)} KB`)
  } catch (e) {
    console.error('导出失败:', e)
    alert('导出失败')
  }
}

onMounted(() => {
  loadArchives()
  loadStats()
})
</script>

<style scoped>
.archive-view {
  padding: 20px;
  max-width: 1200px;
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

.stats-bar {
  display: flex;
  gap: 16px;
  margin-bottom: 16px;
  padding: 12px 16px;
  background: var(--bg-secondary, #1e1e2e);
  border: 1px solid var(--border-color, #313244);
  border-radius: 10px;
}
.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.stat-value {
  font-size: 20px;
  font-weight: 700;
  color: var(--accent-color, #89b4fa);
}
.stat-label {
  font-size: 11px;
  color: var(--text-secondary, #a6adc8);
}

.filter-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 16px;
  flex-wrap: wrap;
}
.filter-select, .filter-input {
  background: var(--bg-primary, #11111b);
  border: 1px solid var(--border-color, #313244);
  border-radius: 6px;
  color: var(--text-primary, #cdd6f4);
  padding: 6px 10px;
  font-size: 13px;
}
.filter-sep {
  color: var(--text-secondary, #a6adc8);
}
.filter-actions {
  margin-left: auto;
  display: flex;
  gap: 6px;
}
.export-btn {
  padding: 6px 12px;
  background: var(--bg-primary, #11111b);
  border: 1px solid var(--border-color, #313244);
  border-radius: 6px;
  color: var(--text-primary, #cdd6f4);
  font-size: 12px;
  cursor: pointer;
}
.export-btn:hover {
  border-color: var(--accent-color, #89b4fa);
}

.archive-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 12px;
}

.empty-state {
  text-align: center;
  padding: 40px;
  color: var(--text-secondary, #a6adc8);
  font-size: 14px;
}

.archive-card {
  background: var(--bg-secondary, #1e1e2e);
  border: 1px solid var(--border-color, #313244);
  border-radius: 10px;
  padding: 14px;
  transition: border-color 0.2s;
}
.archive-card:hover {
  border-color: var(--accent-color, #89b4fa);
}

.card-top {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
}

.card-type-badge {
  font-size: 11px;
  font-weight: 600;
  padding: 2px 8px;
  border-radius: 4px;
  background: rgba(137, 180, 250, 0.15);
  color: #89b4fa;
}
.card-type-badge.thermal { background: rgba(250, 179, 135, 0.15); color: #fab387; }
.card-type-badge.modal { background: rgba(203, 166, 247, 0.15); color: #cba6f7; }
.card-type-badge.cfd { background: rgba(148, 226, 213, 0.15); color: #94e2d5; }

.card-date {
  font-size: 11px;
  color: var(--text-secondary, #a6adc8);
}

.card-body {
  margin-bottom: 10px;
}

.card-info-row {
  display: flex;
  gap: 8px;
  font-size: 12px;
  padding: 2px 0;
}
.info-label {
  color: var(--text-secondary, #a6adc8);
  min-width: 48px;
}
.info-value {
  color: var(--text-primary, #cdd6f4);
}
.converged { color: #a6e3a1; }
.not-converged { color: #f38ba8; }

.card-results {
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--border-color, #313244);
}
.result-item {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  padding: 2px 0;
}
.result-key {
  color: var(--text-secondary, #a6adc8);
}
.result-value {
  color: var(--accent-color, #89b4fa);
  font-family: monospace;
}

.card-actions {
  display: flex;
  gap: 6px;
  justify-content: flex-end;
}
.action-btn {
  padding: 4px 10px;
  background: transparent;
  border: 1px solid var(--border-color, #313244);
  border-radius: 4px;
  color: var(--text-secondary, #a6adc8);
  font-size: 11px;
  cursor: pointer;
}
.action-btn:hover {
  border-color: var(--accent-color, #89b4fa);
  color: var(--accent-color, #89b4fa);
}
.action-btn.danger:hover {
  border-color: #f38ba8;
  color: #f38ba8;
}

.detail-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}
.detail-dialog {
  background: var(--bg-secondary, #1e1e2e);
  border: 1px solid var(--border-color, #313244);
  border-radius: 12px;
  padding: 20px;
  max-width: 600px;
  width: 90%;
  max-height: 80vh;
  overflow-y: auto;
}
.detail-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}
.detail-header h3 {
  margin: 0;
  font-size: 16px;
  color: var(--text-primary, #cdd6f4);
}
.close-btn {
  background: none;
  border: none;
  color: var(--text-secondary, #a6adc8);
  font-size: 16px;
  cursor: pointer;
}

.detail-section {
  margin-bottom: 16px;
}
.detail-section h4 {
  font-size: 13px;
  color: var(--text-secondary, #a6adc8);
  margin: 0 0 8px 0;
}
.detail-row {
  display: flex;
  gap: 8px;
  font-size: 12px;
  padding: 2px 0;
}
.detail-row span:first-child {
  color: var(--text-secondary, #a6adc8);
  min-width: 80px;
}

.json-block {
  background: var(--bg-primary, #11111b);
  border: 1px solid var(--border-color, #313244);
  border-radius: 6px;
  padding: 10px;
  font-size: 11px;
  color: var(--text-primary, #cdd6f4);
  overflow-x: auto;
  margin: 0;
  font-family: monospace;
  max-height: 200px;
  overflow-y: auto;
}
</style>
