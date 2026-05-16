/**
 * CADSyncPanel.vue — V4.1-005 CAD 双向同步面板
 */
<template>
  <div class="cad-sync-panel">
    <div class="panel-header">
      <h3 class="panel-title">🔗 CAD 同步</h3>
      <div class="sync-status-badge" :class="overallStatus">
        {{ statusText }}
      </div>
    </div>

    <!-- 导入区 -->
    <div class="import-zone">
      <input
        ref="fileInput"
        type="file"
        accept=".step,.stp,.iges,.igs,.x_t,.stl,.obj"
        style="display: none"
        @change="handleFileSelect"
      >
      <button class="btn-import" @click="fileInput?.click()">
        📁 导入 CAD 文件
      </button>
      <span class="format-hint">支持 STEP, IGES, Parasolid, STL, OBJ</span>
    </div>

    <!-- 同步中状态 -->
    <div v-if="isSyncing" class="syncing-state">
      <span class="spinner">⏳</span>
      <span>正在同步...</span>
    </div>

    <!-- 错误提示 -->
    <div v-if="lastError" class="error-alert">
      ❌ {{ lastError }}
    </div>

    <!-- CAD 链接列表 -->
    <div class="links-list">
      <div v-for="link in cadLinks" :key="link.id" class="link-card">
        <div class="link-header">
          <div class="link-info">
            <span class="link-format">{{ formatLabel(link.format) }}</span>
            <span class="link-name">{{ link.cadFilePath }}</span>
            <span class="link-version">v{{ link.version }}</span>
          </div>
          <div class="link-status">
            <span class="status-dot" :class="link.status" />
            <span class="status-label">{{ statusLabel(link.status) }}</span>
          </div>
        </div>

        <!-- 几何体列表 -->
        <div class="link-geometries">
          <div
            v-for="geo in link.geometries"
            :key="geo.id"
            class="geo-item"
          >
            <span class="geo-icon">{{ geoTypeIcon(geo.type) }}</span>
            <span class="geo-name">{{ geo.name }}</span>
            <span class="geo-visibility" @click="geo.visible = !geo.visible">
              {{ geo.visible ? '👁️' : '🚫' }}
            </span>
          </div>
        </div>

        <!-- 参数表 -->
        <div v-if="link.parameters.length > 0" class="link-params">
          <table class="params-table">
            <thead>
              <tr>
                <th>参数</th>
                <th>值</th>
                <th>单位</th>
                <th>状态</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="param in link.parameters" :key="param.name">
                <td>{{ param.name }}</td>
                <td>
                  <input
                    v-model.number="param.value"
                    type="number"
                    class="param-input"
                    @change="handleParamChange(link.id, param)"
                  >
                </td>
                <td>{{ param.unit }}</td>
                <td>
                  <span class="param-sync" :class="param.synced ? 'synced' : 'unsynced'">
                    {{ param.synced ? '✓ 已同步' : '⟳ 待同步' }}
                  </span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <!-- 操作按钮 -->
        <div class="link-actions">
          <button
            class="btn-sync"
            :disabled="isSyncing || link.status === 'synced'"
            @click="syncFromCAD(link.id)"
          >
            🔄 CAD → CAE
          </button>
          <button
            class="btn-push"
            :disabled="isSyncing || !hasUnsyncedParams(link)"
            @click="pushParamChanges(link)"
          >
            ⬆️ CAE → CAD
          </button>
          <button v-if="link.status === 'conflict'" class="btn-resolve" @click="resolveConflict(link.id, true)">
            ⚖️ 解决冲突
          </button>
          <button class="btn-remove" @click="removeLink(link.id)">
            🗑️ 移除
          </button>
        </div>

        <!-- 变更历史 -->
        <div v-if="link.changeLog.length > 0" class="link-history">
          <div class="history-toggle" @click="toggleHistory(link.id)">
            📋 变更历史 ({{ link.changeLog.length }})
            <span>{{ expandedHistory[link.id] ? '▼' : '▶' }}</span>
          </div>
          <div v-if="expandedHistory[link.id]" class="history-list">
            <div
              v-for="entry in link.changeLog.slice().reverse()"
              :key="entry.id"
              class="history-entry"
              :class="{ conflict: entry.conflict, unmerged: !entry.merged }"
            >
              <span class="entry-time">{{ formatTime(entry.timestamp) }}</span>
              <span class="entry-source" :class="entry.source">{{ entry.source.toUpperCase() }}</span>
              <span class="entry-type">{{ typeLabel(entry.type) }}</span>
              <span class="entry-desc">{{ entry.description }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-if="cadLinks.length === 0" class="empty-state">
      <span class="empty-icon">📐</span>
      <p>暂无 CAD 链接</p>
      <p class="empty-hint">导入 CAD 文件以开始双向同步</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useCADSync, type CADParameter, type CADLink } from '@/composables/useCADSync'

const {
  cadLinks,
  isSyncing,
  lastError,
  FORMAT_LABELS,
  importCAD,
  syncFromCAD,
  pushToCAD,
  updateParameter,
  removeLink,
  resolveConflict
} = useCADSync()

const fileInput = ref<HTMLInputElement | null>(null)
const expandedHistory = ref<Record<string, boolean>>({})

const overallStatus = computed(() => {
  if (cadLinks.value.some(l => l.status === 'conflict')) return 'conflict'
  if (cadLinks.value.some(l => l.status === 'error')) return 'error'
  if (cadLinks.value.some(l => l.status === 'dirty')) return 'dirty'
  if (cadLinks.value.length === 0) return 'empty'
  return 'synced'
})

const statusText = computed(() => {
  switch (overallStatus.value) {
    case 'conflict': return '存在冲突'
    case 'error': return '同步错误'
    case 'dirty': return '待同步'
    case 'empty': return '未连接'
    default: return '已同步'
  }
})

async function handleFileSelect(e: Event) {
  const input = e.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  try {
    await importCAD(file)
  } catch (err) {
    alert(err instanceof Error ? err.message : '导入失败')
  }
  input.value = ''
}

function handleParamChange(linkId: string, param: CADParameter) {
  updateParameter(linkId, param.name, param.value)
}

function hasUnsyncedParams(link: CADLink): boolean {
  return link.parameters.some(p => !p.synced)
}

async function pushParamChanges(link: CADLink) {
  const changes = link.parameters
    .filter(p => !p.synced)
    .map(p => ({ name: p.name, value: p.value }))
  if (changes.length === 0) return
  await pushToCAD(link.id, changes)
}

function formatLabel(fmt: string): string {
  return FORMAT_LABELS[fmt as keyof typeof FORMAT_LABELS] || fmt
}

function statusLabel(status: string): string {
  const map: Record<string, string> = {
    synced: '已同步',
    dirty: '待同步',
    conflict: '冲突',
    error: '错误',
    offline: '离线'
  }
  return map[status] || status
}

function geoTypeIcon(type: string): string {
  const map: Record<string, string> = {
    solid: '▣',
    surface: '▢',
    curve: '∿',
    point: '•'
  }
  return map[type] || '?'
}

function typeLabel(type: string): string {
  const map: Record<string, string> = {
    parameter: '参数',
    geometry: '几何',
    material: '材料',
    feature: '特征'
  }
  return map[type] || type
}

function formatTime(ts: string): string {
  return new Date(ts).toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit', second: '2-digit' })
}

function toggleHistory(linkId: string) {
  expandedHistory.value[linkId] = !expandedHistory.value[linkId]
}
</script>

<style scoped>
.cad-sync-panel {
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

.sync-status-badge {
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 600;
}

.sync-status-badge.synced {
  background: #dcfce7;
  color: #166534;
}

.sync-status-badge.dirty {
  background: #fef3c7;
  color: #92400e;
}

.sync-status-badge.conflict {
  background: #fee2e2;
  color: #991b1b;
}

.sync-status-badge.error {
  background: #fee2e2;
  color: #991b1b;
}

.sync-status-badge.empty {
  background: #f1f5f9;
  color: #64748b;
}

.import-zone {
  display: flex;
  align-items: center;
  gap: 12px;
}

.btn-import {
  padding: 10px 20px;
  background: var(--primary-color, #3b82f6);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-import:hover {
  background: #2563eb;
}

.format-hint {
  font-size: 12px;
  color: var(--text-muted, #94a3b8);
}

.syncing-state {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 20px;
  color: var(--text-secondary, #64748b);
}

.spinner {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.error-alert {
  padding: 10px 14px;
  background: #fef2f2;
  color: #991b1b;
  border-radius: 8px;
  font-size: 13px;
}

.links-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.link-card {
  padding: 14px;
  border-radius: 10px;
  border: 1px solid var(--border-color, #e2e8f0);
  background: var(--bg-elevated, #f8fafc);
}

.link-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
}

.link-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.link-format {
  font-size: 11px;
  padding: 2px 8px;
  background: #eff6ff;
  color: #1e40af;
  border-radius: 4px;
}

.link-name {
  font-size: 13px;
  font-weight: 600;
}

.link-version {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
}

.link-status {
  display: flex;
  align-items: center;
  gap: 6px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.status-dot.synced {
  background: #22c55e;
}

.status-dot.dirty {
  background: #f59e0b;
}

.status-dot.conflict {
  background: #ef4444;
}

.status-dot.error {
  background: #ef4444;
}

.status-dot.offline {
  background: #94a3b8;
}

.status-label {
  font-size: 12px;
  color: var(--text-secondary, #64748b);
}

.link-geometries {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-bottom: 10px;
}

.geo-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  background: white;
  border-radius: 6px;
  border: 1px solid var(--border-color, #e2e8f0);
  font-size: 12px;
}

.geo-icon {
  font-size: 14px;
}

.geo-visibility {
  cursor: pointer;
  font-size: 12px;
  margin-left: 4px;
}

.link-params {
  margin-bottom: 10px;
}

.params-table {
  width: 100%;
  font-size: 12px;
  border-collapse: collapse;
  background: white;
  border-radius: 6px;
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

.param-input {
  width: 80px;
  padding: 4px 8px;
  border: 1px solid var(--border-color, #e2e8f0);
  border-radius: 4px;
  font-size: 12px;
}

.param-sync {
  font-size: 11px;
  padding: 2px 6px;
  border-radius: 4px;
}

.param-sync.synced {
  background: #dcfce7;
  color: #166534;
}

.param-sync.unsynced {
  background: #fef3c7;
  color: #92400e;
}

.link-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.link-actions button {
  padding: 6px 14px;
  border-radius: 6px;
  border: none;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-sync {
  background: #3b82f6;
  color: white;
}

.btn-push {
  background: #8b5cf6;
  color: white;
}

.btn-resolve {
  background: #f59e0b;
  color: white;
}

.btn-remove {
  background: #ef4444;
  color: white;
}

.link-actions button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.link-history {
  margin-top: 10px;
  padding-top: 10px;
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

.history-entry {
  display: flex;
  gap: 8px;
  align-items: center;
  padding: 6px 10px;
  background: white;
  border-radius: 6px;
  font-size: 11px;
}

.history-entry.conflict {
  background: #fef2f2;
  border: 1px solid #fecaca;
}

.history-entry.unmerged {
  opacity: 0.7;
}

.entry-time {
  color: var(--text-muted, #94a3b8);
  white-space: nowrap;
}

.entry-source {
  padding: 1px 6px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 600;
}

.entry-source.cad {
  background: #dbeafe;
  color: #1e40af;
}

.entry-source.cae {
  background: #dcfce7;
  color: #166534;
}

.entry-type {
  color: var(--text-secondary, #64748b);
}

.entry-desc {
  flex: 1;
  color: var(--text-primary, #1e293b);
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
