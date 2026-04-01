<template>
  <div
    class="embed-card"
    :class="[
      `embed-card--${record.type}`,
      { 'embed-card--updated': isUpdated }
    ]"
    @click="navigateToTarget"
  >
    <div class="embed-card__icon">{{ iconMap[record.type] || '📎' }}</div>
    <div class="embed-card__info">
      <div class="embed-card__type">{{ typeLabels[record.type] || record.type }}</div>
      <div class="embed-card__name">{{ record.targetName }}</div>
      <div class="embed-card__status" :class="statusClass" v-if="statusText">
        <span class="embed-card__status-dot"></span>
        {{ statusText }}
      </div>
    </div>
    <button class="embed-card__remove" @click.stop="remove" title="移除嵌入">×</button>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useProjectStore } from '@/stores/project'

interface EmbedRecord {
  id: string
  type: 'model' | 'code' | 'simulation' | 'fatigue' | 'cfd'
  targetId: string
  targetName: string
  noteId: string
  createdAt: string
}

const props = defineProps<{
  record: EmbedRecord
}>()

const emit = defineEmits<{
  remove: [id: string]
}>()

const router = useRouter()
const projectStore = useProjectStore()

// ============ 图标与标签映射 ============
const iconMap: Record<string, string> = {
  model: '📐',
  code: '📄',
  simulation: '📊',
  fatigue: '🔄',
  cfd: '🌊'
}

const typeLabels: Record<string, string> = {
  model: '3D建模',
  code: '代码',
  simulation: '仿真结果',
  fatigue: '疲劳分析',
  cfd: 'CFD分析'
}

// ============ 实时更新状态追踪 ============

// 记录各类型数据的"已知"状态，用于检测变化
const knownMeshHash = ref<string | null>(null)
const knownResultHash = ref<string | null>(null)
const knownCodeTimestamp = ref<string | null>(null)
const isUpdated = ref(false)
let updateFlashTimer: ReturnType<typeof setTimeout> | null = null

// 计算网格数据的哈希（简化：用节点数+单元数作为标识）
function computeMeshHash(): string | null {
  const mesh = projectStore.currentMesh
  if (!mesh) return null
  return `${mesh.nodes.length}-${mesh.elements.length}`
}

// 计算仿真结果的哈希（简化：用节点数作为标识）
function computeResultHash(): string | null {
  const result = projectStore.lastResult
  if (!result) return null
  // ResultSet 中通常包含节点位移数据，用节点数标识
  const nodeCount = result.node_values?.length ?? 0
  return `result-${nodeCount}`
}

// 获取代码文件的最后修改时间戳
function getCodeTimestamp(): string | null {
  return projectStore.codeLastModified ?? null
}

// ============ 状态计算 ============
const statusText = computed(() => {
  switch (props.record.type) {
    case 'model': {
      const mesh = projectStore.currentMesh
      if (!mesh || mesh.nodes.length === 0) return '暂无数据'
      return isUpdated.value ? '有更新' : '已同步'
    }
    case 'code': {
      const ts = getCodeTimestamp()
      if (!ts) return '暂无数据'
      return isUpdated.value ? '有更新' : '已同步'
    }
    case 'simulation': {
      const result = projectStore.lastResult
      if (!result) return '暂无数据'
      return isUpdated.value ? '有更新' : '已同步'
    }
    case 'fatigue': {
      const result = projectStore.lastBucklingResult
      if (!result) return '暂无数据'
      return isUpdated.value ? '有更新' : '已同步'
    }
    case 'cfd': {
      const result = projectStore.lastTransientResult
      if (!result) return '暂无数据'
      return isUpdated.value ? '有更新' : '已同步'
    }
    default:
      return ''
  }
})

const statusClass = computed(() => {
  if (statusText.value === '暂无数据') return 'embed-card__status--empty'
  if (isUpdated.value) return 'embed-card__status--updated'
  return 'embed-card__status--synced'
})

// ============ 变化检测 watch ============

// 检测模型数据变化
watch(
  () => computeMeshHash(),
  (newHash) => {
    if (props.record.type !== 'model') return
    if (newHash === null) {
      knownMeshHash.value = null
      return
    }
    if (knownMeshHash.value !== null && knownMeshHash.value !== newHash) {
      triggerUpdateFlash()
    }
    knownMeshHash.value = newHash
  }
)

// 检测仿真结果变化
watch(
  () => computeResultHash(),
  (newHash) => {
    if (props.record.type !== 'simulation') return
    if (newHash === null) {
      knownResultHash.value = null
      return
    }
    if (knownResultHash.value !== null && knownResultHash.value !== newHash) {
      triggerUpdateFlash()
    }
    knownResultHash.value = newHash
  }
)

// 检测代码文件变化
watch(
  () => projectStore.codeLastModified,
  (newTimestamp) => {
    if (props.record.type !== 'code') return
    if (!newTimestamp) {
      knownCodeTimestamp.value = null
      return
    }
    if (knownCodeTimestamp.value !== null && knownCodeTimestamp.value !== newTimestamp) {
      triggerUpdateFlash()
    }
    knownCodeTimestamp.value = newTimestamp
  }
)

// 检测疲劳分析结果变化
watch(
  () => projectStore.lastBucklingResult,
  (newVal) => {
    if (props.record.type !== 'fatigue') return
    if (newVal) {
      triggerUpdateFlash()
    }
  }
)

// 检测 CFD/瞬态分析结果变化
watch(
  () => projectStore.lastTransientResult,
  (newVal) => {
    if (props.record.type !== 'cfd') return
    if (newVal) {
      triggerUpdateFlash()
    }
  }
)

// ============ 闪烁动画 ============
function triggerUpdateFlash() {
  isUpdated.value = true
  if (updateFlashTimer) clearTimeout(updateFlashTimer)
  // 3秒后恢复为"已同步"状态
  updateFlashTimer = setTimeout(() => {
    isUpdated.value = false
  }, 3000)
}

// ============ 导航与移除 ============
function navigateToTarget() {
  const routes: Record<string, string> = {
    model: '/modeling',
    code: '/code',
    simulation: '/simulation',
    fatigue: '/simulation',
    cfd: '/transient-dynamics'
  }
  projectStore.setCurrentNoteId(props.record.noteId)
  router.push(routes[props.record.type] || '/')
}

function remove() {
  emit('remove', props.record.id)
}

// ============ 初始化 ============
onMounted(() => {
  // 初始化已知状态，避免首次加载时误报"有更新"
  knownMeshHash.value = computeMeshHash()
  knownResultHash.value = computeResultHash()
  knownCodeTimestamp.value = getCodeTimestamp()
})

onUnmounted(() => {
  if (updateFlashTimer) clearTimeout(updateFlashTimer)
})
</script>

<style scoped>
.embed-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: white;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.embed-card:hover {
  border-color: #60a5fa;
  background: #eff6ff;
}

/* 暗色模式 */
:deep(.dark) .embed-card,
:root.dark .embed-card {
  background: #374151;
  border-color: #4b5563;
}
:deep(.dark) .embed-card:hover,
:root.dark .embed-card:hover {
  border-color: #60a5fa;
  background: rgba(59, 130, 246, 0.1);
}

/* 类型变体 */
.embed-card--model {
  border-left: 3px solid #3b82f6;
}
.embed-card--code {
  border-left: 3px solid #10b981;
}
.embed-card--simulation {
  border-left: 3px solid #f59e0b;
}
.embed-card--fatigue {
  border-left: 3px solid #8b5cf6;
}
.embed-card--cfd {
  border-left: 3px solid #06b6d4;
}

/* 有更新时的闪烁动画 */
.embed-card--updated {
  animation: update-flash 0.6s ease-in-out 3;
}

@keyframes update-flash {
  0%, 100% {
    box-shadow: 0 0 0 0 rgba(59, 130, 246, 0);
  }
  50% {
    box-shadow: 0 0 0 4px rgba(59, 130, 246, 0.3);
  }
}

.embed-card__icon {
  font-size: 1.5rem;
  flex-shrink: 0;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f3f4f6;
  border-radius: 8px;
}

:deep(.dark) .embed-card__icon,
:root.dark .embed-card__icon {
  background: #4b5563;
}

.embed-card__info {
  flex: 1;
  min-width: 0;
}

.embed-card__type {
  font-size: 0.75rem;
  color: #6b7280;
  line-height: 1;
}

:deep(.dark) .embed-card__type,
:root.dark .embed-card__type {
  color: #9ca3af;
}

.embed-card__name {
  font-size: 0.875rem;
  font-weight: 500;
  color: #1f2937;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-top: 2px;
}

:deep(.dark) .embed-card__name,
:root.dark .embed-card__name {
  color: #f9fafb;
}

.embed-card__status {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 0.7rem;
  margin-top: 2px;
}

.embed-card__status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  flex-shrink: 0;
}

.embed-card__status--empty {
  color: #9ca3af;
}
.embed-card__status--empty .embed-card__status-dot {
  background: #9ca3af;
}

.embed-card__status--synced {
  color: #10b981;
}
.embed-card__status--synced .embed-card__status-dot {
  background: #10b981;
}

.embed-card__status--updated {
  color: #3b82f6;
  font-weight: 500;
}
.embed-card__status--updated .embed-card__status-dot {
  background: #3b82f6;
  animation: dot-pulse 1s ease-in-out infinite;
}

@keyframes dot-pulse {
  0%, 100% {
    opacity: 1;
    transform: scale(1);
  }
  50% {
    opacity: 0.5;
    transform: scale(1.3);
  }
}

.embed-card__remove {
  flex-shrink: 0;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: #9ca3af;
  font-size: 1rem;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.15s ease;
  line-height: 1;
  padding: 0;
}

.embed-card__remove:hover {
  color: #ef4444;
  background: #fee2e2;
}

:deep(.dark) .embed-card__remove:hover,
:root.dark .embed-card__remove:hover {
  background: rgba(239, 68, 68, 0.2);
}
</style>
