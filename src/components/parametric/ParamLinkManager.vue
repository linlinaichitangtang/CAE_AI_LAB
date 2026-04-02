<template>
  <div class="param-link-manager" :class="{ 'param-link-manager--dark': isDark }">
    <div class="param-link-manager__header">
      <h3 class="param-link-manager__title">参数链接管理</h3>
      <button
        class="param-link-manager__add-btn"
        @click="showAddDialog = true"
        :disabled="store.allParams.length < 2"
      >
        + 添加链接
      </button>
    </div>

    <!-- Link list -->
    <div class="param-link-manager__list" v-if="store.allLinks.length > 0">
      <div
        v-for="link in store.allLinks"
        :key="link.id"
        class="param-link-manager__item"
      >
        <div class="param-link-manager__link-info">
          <span class="param-link-manager__param-tag source">
            {{ getParamName(link.sourceId) }}
          </span>
          <span class="param-link-manager__transform">
            &times;{{ link.scale }}
            <span v-if="link.offset !== 0">
              {{ link.offset > 0 ? '+' : '' }}{{ link.offset }}
            </span>
          </span>
          <span class="param-link-manager__arrow">&rarr;</span>
          <span class="param-link-manager__param-tag target">
            {{ getParamName(link.targetId) }}
          </span>
        </div>
        <button
          class="param-link-manager__remove-btn"
          @click="removeLink(link.id)"
          title="删除链接"
        >
          &times;
        </button>
      </div>
    </div>

    <!-- Empty state -->
    <div v-else class="param-link-manager__empty">
      <p>暂无参数链接</p>
      <p class="param-link-manager__hint">
        将一个参数作为源，驱动其他参数联动变化
      </p>
    </div>

    <!-- Link visualization (simple tree) -->
    <div
      class="param-link-manager__viz"
      v-if="store.allLinks.length > 0"
    >
      <h4 class="param-link-manager__viz-title">链接关系图</h4>
      <svg
        :viewBox="`0 0 ${vizWidth} ${vizHeight}`"
        class="param-link-manager__svg"
      >
        <!-- Draw links (lines) -->
        <line
          v-for="(edge, idx) in vizEdges"
          :key="'edge-' + idx"
          :x1="edge.x1"
          :y1="edge.y1"
          :x2="edge.x2"
          :y2="edge.y2"
          stroke="var(--primary, #4f46e5)"
          stroke-width="1.5"
          stroke-dasharray="4,3"
          marker-end="url(#arrowhead)"
        />
        <!-- Arrow marker -->
        <defs>
          <marker
            id="arrowhead"
            markerWidth="8"
            markerHeight="6"
            refX="8"
            refY="3"
            orient="auto"
          >
            <polygon
              points="0 0, 8 3, 0 6"
              fill="var(--primary, #4f46e5)"
            />
          </marker>
        </defs>
        <!-- Draw nodes -->
        <g v-for="node in vizNodes" :key="'node-' + node.id">
          <rect
            :x="node.x - 50"
            :y="node.y - 14"
            width="100"
            height="28"
            rx="6"
            :fill="node.isSource ? 'rgba(79, 70, 229, 0.12)' : 'rgba(107, 114, 128, 0.12)'"
            :stroke="node.isSource ? 'var(--primary, #4f46e5)' : 'var(--text-secondary, #9ca3af)'"
            stroke-width="1"
          />
          <text
            :x="node.x"
            :y="node.y + 4"
            font-size="11"
            fill="var(--text-primary, #374151)"
            text-anchor="middle"
          >
            {{ truncate(node.name, 12) }}
          </text>
        </g>
      </svg>
    </div>

    <!-- Add link dialog -->
    <div v-if="showAddDialog" class="param-link-manager__dialog-overlay" @click.self="showAddDialog = false">
      <div class="param-link-manager__dialog">
        <h4 class="param-link-manager__dialog-title">添加参数链接</h4>

        <div class="param-link-manager__form">
          <div class="param-link-manager__form-group">
            <label>源参数（驱动方）</label>
            <select v-model="newLink.sourceId">
              <option value="">-- 选择源参数 --</option>
              <option
                v-for="p in store.allParams"
                :key="p.id"
                :value="p.id"
              >
                {{ p.name }}
              </option>
            </select>
          </div>

          <div class="param-link-manager__form-group">
            <label>目标参数（被驱动方）</label>
            <select v-model="newLink.targetId">
              <option value="">-- 选择目标参数 --</option>
              <option
                v-for="p in availableTargets"
                :key="p.id"
                :value="p.id"
              >
                {{ p.name }}
              </option>
            </select>
          </div>

          <div class="param-link-manager__form-row">
            <div class="param-link-manager__form-group">
              <label>缩放系数 (scale)</label>
              <input type="number" v-model.number="newLink.scale" step="0.1" />
            </div>
            <div class="param-link-manager__form-group">
              <label>偏移量 (offset)</label>
              <input type="number" v-model.number="newLink.offset" step="0.1" />
            </div>
          </div>

          <div class="param-link-manager__formula-preview" v-if="newLink.sourceId && newLink.targetId">
            <span class="param-link-manager__formula-label">变换公式:</span>
            <code>
              {{ getParamName(newLink.targetId) }} = {{ getParamName(newLink.sourceId) }}
              &times; {{ newLink.scale }}
              <span v-if="newLink.offset !== 0">
                {{ newLink.offset > 0 ? '+' : '' }}{{ newLink.offset }}
              </span>
            </code>
          </div>
        </div>

        <div class="param-link-manager__dialog-actions">
          <button class="btn btn-ghost" @click="showAddDialog = false">取消</button>
          <button
            class="btn btn-primary"
            @click="addLink"
            :disabled="!newLink.sourceId || !newLink.targetId"
          >
            添加
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useParametricStore } from '@/stores/parametric'

const store = useParametricStore()
const isDark = ref(false)
const showAddDialog = ref(false)

const newLink = ref({
  sourceId: '',
  targetId: '',
  scale: 1,
  offset: 0,
})

// Available targets exclude the selected source
const availableTargets = computed(() => {
  return store.allParams.filter(p => p.id !== newLink.value.sourceId)
})

function getParamName(id: string): string {
  return store.getParamById(id)?.name ?? id
}

function truncate(str: string, maxLen: number): string {
  return str.length > maxLen ? str.slice(0, maxLen) + '...' : str
}

function addLink() {
  if (!newLink.value.sourceId || !newLink.value.targetId) return
  const result = store.addParamLink(
    newLink.value.sourceId,
    newLink.value.targetId,
    newLink.value.scale,
    newLink.value.offset,
  )
  if (result) {
    showAddDialog.value = false
    newLink.value = { sourceId: '', targetId: '', scale: 1, offset: 0 }
  }
}

function removeLink(linkId: string) {
  store.removeParamLink(linkId)
}

// ========== Visualization ==========

interface VizNode {
  id: string
  name: string
  x: number
  y: number
  isSource: boolean
}

interface VizEdge {
  x1: number
  y1: number
  x2: number
  y2: number
}

const vizWidth = 320
const vizHeight = 200

const vizNodes = computed<VizNode[]>(() => {
  const params = store.allParams
  if (params.length === 0) return []

  const sourceIds = new Set(store.allLinks.map(l => l.sourceId))
  const nodeSpacing = Math.min(60, (vizHeight - 40) / Math.max(params.length - 1, 1))

  return params.map((p, i) => ({
    id: p.id,
    name: p.name,
    x: sourceIds.has(p.id) ? 80 : 240,
    y: 20 + i * nodeSpacing,
    isSource: sourceIds.has(p.id),
  }))
})

const vizEdges = computed<VizEdge[]>(() => {
  const nodeMap = new Map<string, VizNode>()
  for (const n of vizNodes.value) {
    nodeMap.set(n.id, n)
  }

  return store.allLinks
    .map(l => {
      const src = nodeMap.get(l.sourceId)
      const tgt = nodeMap.get(l.targetId)
      if (!src || !tgt) return null
      return {
        x1: src.x + 50,
        y1: src.y,
        x2: tgt.x - 50,
        y2: tgt.y,
      }
    })
    .filter((e): e is VizEdge => e !== null)
})

function checkDarkMode() {
  isDark.value = document.documentElement.classList.contains('dark')
}

onMounted(() => {
  checkDarkMode()
  const observer = new MutationObserver(checkDarkMode)
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] })
  onUnmounted(() => observer.disconnect())
})
</script>

<style scoped>
.param-link-manager {
  padding: 12px 0;
}

.param-link-manager__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.param-link-manager__title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary, #374151);
}

.param-link-manager--dark .param-link-manager__title {
  color: #e5e7eb;
}

.param-link-manager__add-btn {
  padding: 4px 10px;
  font-size: 12px;
  background: var(--primary, #4f46e5);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: opacity 0.15s;
}

.param-link-manager__add-btn:hover:not(:disabled) {
  opacity: 0.9;
}

.param-link-manager__add-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.param-link-manager__list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.param-link-manager__item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 10px;
  background: var(--bg-secondary, #f9fafb);
  border-radius: 8px;
  border: 1px solid var(--border-color, #e5e7eb);
}

.param-link-manager--dark .param-link-manager__item {
  background: #1f2937;
  border-color: #374151;
}

.param-link-manager__link-info {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
}

.param-link-manager__param-tag {
  padding: 2px 8px;
  border-radius: 4px;
  font-weight: 500;
}

.param-link-manager__param-tag.source {
  background: rgba(79, 70, 229, 0.12);
  color: var(--primary, #4f46e5);
}

.param-link-manager__param-tag.target {
  background: rgba(107, 114, 128, 0.12);
  color: var(--text-secondary, #6b7280);
}

.param-link-manager__transform {
  font-size: 11px;
  color: var(--text-secondary, #9ca3af);
  font-family: monospace;
}

.param-link-manager__arrow {
  color: var(--text-secondary, #9ca3af);
}

.param-link-manager__remove-btn {
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: #ef4444;
  font-size: 16px;
  cursor: pointer;
  border-radius: 4px;
  transition: background 0.15s;
}

.param-link-manager__remove-btn:hover {
  background: rgba(239, 68, 68, 0.1);
}

.param-link-manager__empty {
  text-align: center;
  padding: 16px;
  color: var(--text-secondary, #9ca3af);
  font-size: 12px;
}

.param-link-manager__hint {
  font-size: 11px;
  margin-top: 4px;
  opacity: 0.7;
}

.param-link-manager__viz {
  margin-top: 16px;
  padding-top: 12px;
  border-top: 1px solid var(--border-color, #e5e7eb);
}

.param-link-manager__viz-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-primary, #374151);
  margin-bottom: 8px;
}

.param-link-manager--dark .param-link-manager__viz-title {
  color: #d1d5db;
}

.param-link-manager__svg {
  width: 100%;
  height: auto;
  max-height: 220px;
}

/* Dialog */
.param-link-manager__dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.param-link-manager__dialog {
  background: var(--bg-primary, #ffffff);
  border-radius: 12px;
  padding: 20px;
  width: 380px;
  max-width: 90vw;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.15);
}

.param-link-manager--dark .param-link-manager__dialog {
  background: #1f2937;
}

.param-link-manager__dialog-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary, #374151);
  margin-bottom: 16px;
}

.param-link-manager--dark .param-link-manager__dialog-title {
  color: #e5e7eb;
}

.param-link-manager__form {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.param-link-manager__form-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.param-link-manager__form-group label {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary, #6b7280);
}

.param-link-manager__form-group select,
.param-link-manager__form-group input {
  padding: 6px 10px;
  font-size: 13px;
  border: 1px solid var(--border-color, #d1d5db);
  border-radius: 6px;
  background: var(--bg-primary, #ffffff);
  color: var(--text-primary, #374151);
  outline: none;
}

.param-link-manager--dark .param-link-manager__form-group select,
.param-link-manager--dark .param-link-manager__form-group input {
  background: #111827;
  border-color: #4b5563;
  color: #e5e7eb;
}

.param-link-manager__form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
}

.param-link-manager__formula-preview {
  padding: 8px 10px;
  background: var(--bg-secondary, #f3f4f6);
  border-radius: 6px;
  font-size: 12px;
}

.param-link-manager--dark .param-link-manager__formula-preview {
  background: #111827;
}

.param-link-manager__formula-label {
  color: var(--text-secondary, #9ca3af);
  margin-right: 4px;
}

.param-link-manager__formula-preview code {
  color: var(--primary, #4f46e5);
  font-family: monospace;
}

.param-link-manager__dialog-actions {
  display: flex;
  gap: 8px;
  margin-top: 16px;
  justify-content: flex-end;
}

.btn {
  padding: 7px 16px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  border: none;
}

.btn-primary {
  background: var(--primary, #4f46e5);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  opacity: 0.9;
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-ghost {
  background: transparent;
  color: var(--text-secondary, #6b7280);
  border: 1px solid var(--border-color, #d1d5db);
}

.btn-ghost:hover {
  background: var(--bg-secondary, #f3f4f6);
}

.param-link-manager--dark .btn-ghost {
  border-color: #4b5563;
  color: #9ca3af;
}
</style>
