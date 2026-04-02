<template>
  <div class="preview-mode" :class="{ 'preview-mode--dark': isDark }">
    <!-- Preview badge -->
    <div class="preview-mode__badge">
      <span class="preview-mode__dot"></span>
      预览模式
    </div>

    <!-- Preview summary -->
    <div class="preview-mode__summary">
      <h4 class="preview-mode__title">参数变更预览</h4>
      <div class="preview-mode__changes">
        <div
          v-for="change in changes"
          :key="change.id"
          class="preview-mode__change-item"
        >
          <span class="preview-mode__param-name">{{ change.name }}</span>
          <div class="preview-mode__values">
            <span class="preview-mode__old-value">{{ formatVal(change.baseValue) }}</span>
            <span class="preview-mode__arrow">&rarr;</span>
            <span
              class="preview-mode__new-value"
              :class="change.delta > 0 ? 'positive' : change.delta < 0 ? 'negative' : ''"
            >
              {{ formatVal(change.newValue) }}
            </span>
            <span
              class="preview-mode__delta"
              :class="change.delta > 0 ? 'positive' : change.delta < 0 ? 'negative' : ''"
            >
              ({{ change.delta > 0 ? '+' : '' }}{{ formatDelta(change.delta) }})
            </span>
          </div>
        </div>

        <div v-if="changes.length === 0" class="preview-mode__no-changes">
          参数未发生变化
        </div>
      </div>
    </div>

    <!-- Linear interpolation preview -->
    <div class="preview-mode__trend" v-if="hasBaselineResult">
      <h4 class="preview-mode__title">变形趋势预览</h4>
      <div class="preview-mode__trend-chart">
        <svg viewBox="0 0 280 120" class="preview-mode__svg">
          <!-- Grid lines -->
          <line v-for="i in 5" :key="'grid-' + i"
            :x1="30" :y1="10 + (i - 1) * 25"
            :x2="270" :y2="10 + (i - 1) * 25"
            stroke="var(--border-color, #e5e7eb)" stroke-width="0.5"
          />
          <!-- Y axis labels -->
          <text v-for="i in 5" :key="'label-' + i"
            :x="25" :y="14 + (i - 1) * 25"
            font-size="8" fill="var(--text-secondary, #9ca3af)" text-anchor="end"
          >
            {{ formatVal(trendMax - (i - 1) * trendStep) }}
          </text>
          <!-- Baseline line -->
          <line
            x1="30" y1="110"
            x2="270" y2="110"
            stroke="var(--text-secondary, #9ca3af)" stroke-width="1"
          />
          <!-- Trend line -->
          <polyline
            :points="trendPoints"
            fill="none"
            stroke="var(--primary, #4f46e5)"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
          <!-- Current point -->
          <circle
            :cx="trendCurrentX" :cy="trendCurrentY"
            r="4"
            fill="var(--primary, #4f46e5)"
          />
          <!-- Current point label -->
          <text
            :x="trendCurrentX" :y="trendCurrentY - 8"
            font-size="8" fill="var(--primary, #4f46e5)" text-anchor="middle"
          >
            {{ formatVal(interpolatedResult) }}
          </text>
        </svg>
      </div>
      <p class="preview-mode__trend-note">
        * 基于线性插值的简化趋势预览，实际结果需完整求解
      </p>
    </div>

    <!-- Action buttons -->
    <div class="preview-mode__actions">
      <button
        class="btn btn-primary preview-mode__confirm"
        @click="$emit('confirm')"
        :disabled="changes.length === 0"
      >
        确认求解
      </button>
      <button
        class="btn btn-ghost preview-mode__cancel"
        @click="$emit('cancel')"
      >
        取消
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useParametricStore } from '@/stores/parametric'

const props = defineProps<{
  baselineResult?: number | null
  paramResults?: Array<{ paramValues: Record<string, number>; result: number }>
}>()

defineEmits<{
  confirm: []
  cancel: []
}>()

const store = useParametricStore()
const isDark = ref(false)

interface Change {
  id: string
  name: string
  baseValue: number
  newValue: number
  delta: number
}

const changes = computed<Change[]>(() => {
  const result: Change[] = []
  for (const p of store.params) {
    const base = store.previewBaseValues[p.id]
    if (base === undefined) continue
    const current = store.previewValues[p.id] ?? p.value
    if (Math.abs(current - base) > 1e-9) {
      result.push({
        id: p.id,
        name: p.name,
        baseValue: base,
        newValue: current,
        delta: current - base,
      })
    }
  }
  return result
})

const hasBaselineResult = computed(() => props.baselineResult != null)

// Linear interpolation for trend preview
const interpolatedResult = computed(() => {
  if (!hasBaselineResult.value || changes.value.length === 0) return 0

  // Simple linear interpolation: assume result scales proportionally
  // with the largest relative parameter change
  let maxRelativeChange = 0
  for (const c of changes.value) {
    if (Math.abs(c.baseValue) > 1e-12) {
      const rel = Math.abs(c.delta / c.baseValue)
      if (rel > maxRelativeChange) maxRelativeChange = rel
    }
  }

  // Assume linear scaling
  const sign = changes.value.some(c => c.delta > 0) ? 1 : -1
  return (props.baselineResult ?? 0) * (1 + sign * maxRelativeChange * 0.5)
})

const trendMax = computed(() => {
  const base = props.baselineResult ?? 0
  const interp = interpolatedResult.value
  return Math.max(base, interp) * 1.2 || 1
})

const trendStep = computed(() => trendMax.value / 4)

const trendPoints = computed(() => {
  const base = props.baselineResult ?? 0
  const interp = interpolatedResult.value
  const yBase = 110 - (base / trendMax.value) * 100
  const yInterp = 110 - (interp / trendMax.value) * 100
  return `30,${yBase} 150,${(yBase + yInterp) / 2} 270,${yInterp}`
})

const trendCurrentX = 270
const trendCurrentY = computed(() => {
  const interp = interpolatedResult.value
  return 110 - (interp / trendMax.value) * 100
})

function formatVal(val: number): string {
  if (Math.abs(val) >= 1000) return val.toFixed(1)
  if (Math.abs(val) >= 1) return val.toFixed(2)
  return val.toFixed(4)
}

function formatDelta(delta: number): string {
  if (Math.abs(delta) >= 100) return delta.toFixed(1)
  if (Math.abs(delta) >= 1) return delta.toFixed(2)
  return delta.toFixed(4)
}

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
.preview-mode {
  background: rgba(59, 130, 246, 0.06);
  border: 1px solid rgba(59, 130, 246, 0.2);
  border-radius: 12px;
  padding: 16px;
  margin-top: 12px;
}

.preview-mode--dark {
  background: rgba(59, 130, 246, 0.1);
  border-color: rgba(59, 130, 246, 0.3);
}

.preview-mode__badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 3px 10px;
  background: rgba(59, 130, 246, 0.15);
  color: #3b82f6;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 500;
  margin-bottom: 12px;
}

.preview-mode--dark .preview-mode__badge {
  background: rgba(59, 130, 246, 0.25);
  color: #60a5fa;
}

.preview-mode__dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: #3b82f6;
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.4; }
}

.preview-mode__title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary, #374151);
  margin-bottom: 8px;
}

.preview-mode--dark .preview-mode__title {
  color: #e5e7eb;
}

.preview-mode__changes {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.preview-mode__change-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 4px 8px;
  background: rgba(255, 255, 255, 0.5);
  border-radius: 6px;
}

.preview-mode--dark .preview-mode__change-item {
  background: rgba(255, 255, 255, 0.05);
}

.preview-mode__param-name {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-primary, #374151);
}

.preview-mode--dark .preview-mode__param-name {
  color: #d1d5db;
}

.preview-mode__values {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
}

.preview-mode__old-value {
  color: var(--text-secondary, #9ca3af);
  text-decoration: line-through;
}

.preview-mode__arrow {
  color: var(--text-secondary, #9ca3af);
}

.preview-mode__new-value {
  font-weight: 600;
  color: var(--text-primary, #374151);
}

.preview-mode__new-value.positive {
  color: #16a34a;
}

.preview-mode__new-value.negative {
  color: #dc2626;
}

.preview-mode__delta {
  font-size: 11px;
  color: var(--text-secondary, #9ca3af);
}

.preview-mode__delta.positive {
  color: #16a34a;
}

.preview-mode__delta.negative {
  color: #dc2626;
}

.preview-mode__no-changes {
  font-size: 12px;
  color: var(--text-secondary, #9ca3af);
  text-align: center;
  padding: 8px;
}

.preview-mode__trend {
  margin-top: 16px;
  padding-top: 12px;
  border-top: 1px solid rgba(59, 130, 246, 0.15);
}

.preview-mode__trend-chart {
  background: rgba(255, 255, 255, 0.6);
  border-radius: 8px;
  padding: 8px;
}

.preview-mode--dark .preview-mode__trend-chart {
  background: rgba(255, 255, 255, 0.05);
}

.preview-mode__svg {
  width: 100%;
  height: auto;
}

.preview-mode__trend-note {
  font-size: 10px;
  color: var(--text-secondary, #9ca3af);
  margin-top: 6px;
  text-align: center;
}

.preview-mode__actions {
  display: flex;
  gap: 8px;
  margin-top: 16px;
}

.preview-mode__confirm {
  flex: 1;
}

.preview-mode__cancel {
  flex: 1;
}

.btn {
  padding: 8px 16px;
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

.preview-mode--dark .btn-ghost {
  border-color: #4b5563;
  color: #9ca3af;
}

.preview-mode--dark .btn-ghost:hover {
  background: #374151;
}
</style>
