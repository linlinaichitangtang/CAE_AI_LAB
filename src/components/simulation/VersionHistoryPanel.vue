<template>
  <div class="version-history-panel">
    <div class="flex items-center justify-between mb-3">
      <h3 class="text-sm font-semibold text-[var(--text-primary)]">版本历史</h3>
      <div class="flex gap-1">
        <button
          @click="saveNow"
          class="p-1.5 rounded hover:bg-[var(--bg-hover)] text-[var(--text-secondary)] transition"
          title="立即保存"
        >
          <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
            <polyline points="17 21 17 13 7 13 7 21"/>
            <polyline points="7 3 7 8 15 8"/>
          </svg>
        </button>
        <button
          @click="clearAll"
          class="p-1.5 rounded hover:bg-red-50 text-[var(--text-secondary)] hover:text-red-500 transition"
          title="清除历史"
        >
          <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="3 6 5 6 21 6"/>
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
            <line x1="10" y1="11" x2="10" y2="17"/>
            <line x1="14" y1="11" x2="14" y2="17"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- 自动保存状态 -->
    <div class="flex items-center justify-between text-xs mb-2 px-1">
      <span class="text-[var(--text-secondary)]">
        自动保存: {{ enabled ? '开启' : '关闭' }}
      </span>
      <label class="flex items-center gap-1 cursor-pointer">
        <input
          type="checkbox"
          :checked="enabled"
          @change="toggleAutoSave"
          class="rounded text-blue-600 w-3.5 h-3.5"
        />
        <span class="text-[var(--text-secondary)]">30秒</span>
      </label>
    </div>

    <!-- 最后保存时间 -->
    <div v-if="lastSaveTime" class="text-[10px] text-[var(--text-tertiary)] mb-2 px-1">
      最后保存: {{ formatTime(lastSaveTime) }}
    </div>

    <!-- 自动保存中指示 -->
    <div v-if="isAutoSaving" class="flex items-center gap-1 text-[10px] text-blue-500 mb-2 px-1">
      <svg class="w-3 h-3 animate-spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
      </svg>
      <span>正在保存...</span>
    </div>

    <!-- 版本列表 -->
    <div class="space-y-1 max-h-60 overflow-y-auto custom-scrollbar">
      <div
        v-for="ver in sortedVersions"
        :key="ver.id"
        :class="[
          'version-item rounded-lg p-2 cursor-pointer transition border',
          selectedVersion === ver.id
            ? 'bg-blue-50 border-blue-200'
            : 'bg-[var(--bg-elevated)] hover:bg-[var(--bg-hover)] border-transparent',
        ]"
        @click="selectVersion(ver.id)"
      >
        <div class="flex items-center justify-between">
          <span class="text-xs font-medium text-[var(--text-primary)] truncate max-w-[140px]">
            {{ ver.label }}
          </span>
          <span class="text-[10px] text-[var(--text-tertiary)] whitespace-nowrap ml-2">
            {{ formatTime(ver.timestamp) }}
          </span>
        </div>
        <div class="flex gap-2 mt-1 text-[10px] text-[var(--text-secondary)]">
          <span v-if="ver.summary.hasMesh">
            网格 {{ ver.summary.nodeCount }}N/{{ ver.summary.elementCount }}E
          </span>
          <span v-if="ver.summary.hasBoundaryConditions">BC</span>
          <span v-if="ver.summary.hasResult">结果</span>
          <span v-if="!ver.summary.hasMesh && !ver.summary.hasBoundaryConditions && !ver.summary.hasResult" class="text-[var(--text-tertiary)]">
            空项目
          </span>
        </div>
        <!-- 恢复和删除按钮 -->
        <div class="flex gap-1 mt-1.5" v-if="selectedVersion === ver.id">
          <button
            @click.stop="restore(ver.id)"
            class="text-[10px] px-2 py-0.5 bg-blue-600 text-white rounded hover:bg-blue-700 transition"
          >
            恢复此版本
          </button>
          <button
            @click.stop="remove(ver.id)"
            class="text-[10px] px-2 py-0.5 text-red-500 hover:bg-red-50 rounded transition"
          >
            删除
          </button>
        </div>
      </div>

      <div
        v-if="versions.length === 0"
        class="text-center py-4 text-xs text-[var(--text-tertiary)]"
      >
        暂无版本历史
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAutoSave } from '@/composables/useAutoSave'

const {
  isAutoSaving,
  lastSaveTime,
  versions,
  saveVersion,
  restoreVersion,
  deleteVersion,
  clearVersions,
  updateConfig,
} = useAutoSave()

const enabled = ref(true)
const selectedVersion = ref<string | null>(null)

/** 按时间倒序排列的版本列表 */
const sortedVersions = computed(() => {
  return [...versions.value].sort(
    (a, b) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime()
  )
})

/** 格式化时间显示 */
function formatTime(timestamp: string | Date): string {
  const date = timestamp instanceof Date ? timestamp : new Date(timestamp)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffSec = Math.floor(diffMs / 1000)
  const diffMin = Math.floor(diffSec / 60)
  const diffHour = Math.floor(diffMin / 60)
  const diffDay = Math.floor(diffHour / 24)

  if (diffSec < 60) return '刚刚'
  if (diffMin < 60) return `${diffMin}分钟前`
  if (diffHour < 24) return `${diffHour}小时前`
  if (diffDay < 7) return `${diffDay}天前`

  return date.toLocaleDateString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    hour12: false,
  })
}

/** 选择版本 */
function selectVersion(versionId: string) {
  selectedVersion.value = selectedVersion.value === versionId ? null : versionId
}

/** 立即保存 */
function saveNow() {
  saveVersion('手动保存')
}

/** 恢复版本 */
function restore(versionId: string) {
  const success = restoreVersion(versionId)
  if (success) {
    selectedVersion.value = null
  } else {
    alert('恢复版本失败，请查看控制台日志。')
  }
}

/** 删除版本 */
function remove(versionId: string) {
  deleteVersion(versionId)
  if (selectedVersion.value === versionId) {
    selectedVersion.value = null
  }
}

/** 清除所有版本历史 */
function clearAll() {
  if (versions.value.length === 0) return
  if (confirm('确定要清除所有版本历史吗？此操作不可撤销。')) {
    clearVersions()
    selectedVersion.value = null
  }
}

/** 切换自动保存开关 */
function toggleAutoSave() {
  enabled.value = !enabled.value
  updateConfig({ enabled: enabled.value })
}
</script>

<style scoped>
.version-history-panel {
  padding: 12px;
}

.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}

.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}

.custom-scrollbar::-webkit-scrollbar-thumb {
  background: var(--border-default, #e2e8f0);
  border-radius: 2px;
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: var(--text-tertiary, #94a3b8);
}
</style>
