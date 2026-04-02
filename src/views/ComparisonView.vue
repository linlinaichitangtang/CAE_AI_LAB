<template>
  <div class="comparison-view h-full flex bg-[var(--bg-base)]">
    <!-- Left Panel: Result Cards -->
    <div class="w-72 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col overflow-hidden">
      <!-- Panel Header -->
      <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
        <div class="flex items-center justify-between mb-2">
          <h2 class="text-sm font-semibold text-[var(--text-primary)]">Results</h2>
          <span class="text-xs text-[var(--text-muted)]">{{ availableResults.length }} total</span>
        </div>
        <div class="flex items-center gap-2">
          <input
            v-model="searchQuery"
            placeholder="Search results..."
            class="flex-1 px-2 py-1 text-xs border rounded bg-[var(--bg-base)] text-[var(--text-primary)] placeholder:text-[var(--text-muted)]"
          />
          <button @click="addDemoResults" class="px-2 py-1 text-xs bg-pink-600 text-white rounded hover:bg-pink-700">
            + Demo
          </button>
        </div>
      </div>

      <!-- Result Cards List -->
      <div class="flex-1 overflow-y-auto p-2 space-y-1">
        <div
          v-for="result in filteredResults"
          :key="result.id"
          :class="[
            'rounded-lg border transition-all cursor-pointer',
            selectedResultIds.includes(result.id)
              ? 'border-pink-400 bg-pink-50 dark:bg-pink-900/20'
              : 'border-[var(--border-subtle)] bg-white dark:bg-gray-800 hover:border-gray-300'
          ]"
        >
          <div class="flex items-start gap-2 p-2" @click="toggleResultSelection(result.id)">
            <input
              type="checkbox"
              :checked="selectedResultIds.includes(result.id)"
              @click.stop="toggleResultSelection(result.id)"
              class="mt-0.5 rounded text-pink-600"
            />
            <div class="flex-1 min-w-0">
              <div class="flex items-center justify-between">
                <span class="text-xs font-medium text-[var(--text-primary)] truncate">{{ result.name }}</span>
                <button
                  @click.stop="toggleExpand(result.id)"
                  class="text-[var(--text-muted)] hover:text-[var(--text-primary)] ml-1"
                >
                  <svg class="w-3 h-3 transition-transform" :class="{ 'rotate-180': expandedIds.has(result.id) }" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                  </svg>
                </button>
              </div>
              <div class="text-[10px] text-[var(--text-muted)] mt-0.5">
                {{ result.type }} | {{ result.createdAt }}
              </div>
              <!-- Key metrics preview -->
              <div class="flex gap-2 mt-1">
                <span v-if="result.maxVonMises" class="text-[10px] px-1.5 py-0.5 bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400 rounded">
                  {{ result.maxVonMises.toFixed(1) }} MPa
                </span>
                <span v-if="result.maxDisplacement" class="text-[10px] px-1.5 py-0.5 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400 rounded">
                  {{ result.maxDisplacement.toFixed(4) }} mm
                </span>
              </div>
            </div>
          </div>

          <!-- Expanded details -->
          <div v-if="expandedIds.has(result.id)" class="px-2 pb-2 border-t border-[var(--border-subtle)] mt-1 pt-2">
            <div class="space-y-1 text-[10px]">
              <div class="flex justify-between">
                <span class="text-[var(--text-muted)]">Max Stress</span>
                <span class="text-[var(--text-primary)]">{{ result.maxVonMises?.toFixed(2) || '-' }} MPa</span>
              </div>
              <div class="flex justify-between">
                <span class="text-[var(--text-muted)]">Max Displacement</span>
                <span class="text-[var(--text-primary)]">{{ result.maxDisplacement?.toFixed(4) || '-' }} mm</span>
              </div>
              <div class="flex justify-between">
                <span class="text-[var(--text-muted)]">Volume Fraction</span>
                <span class="text-[var(--text-primary)]">{{ result.volumeFraction?.toFixed(3) || '-' }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-[var(--text-muted)]">Compliance</span>
                <span class="text-[var(--text-primary)]">{{ result.compliance?.toFixed(4) || '-' }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-[var(--text-muted)]">Elements</span>
                <span class="text-[var(--text-primary)]">{{ result.elementCount || '-' }}</span>
              </div>
            </div>
          </div>
        </div>

        <div v-if="filteredResults.length === 0" class="text-center py-8">
          <p class="text-sm text-[var(--text-muted)]">No results found</p>
          <p class="text-xs text-[var(--text-muted)] mt-1">Add demo results or save from optimization</p>
        </div>
      </div>

      <!-- Selection Info -->
      <div class="px-4 py-2 border-t border-[var(--border-subtle)] bg-[var(--bg-surface)]">
        <div class="text-[10px] text-[var(--text-muted)]">
          {{ selectedResultIds.length }} selected (select 2 for comparison)
        </div>
      </div>
    </div>

    <!-- Right Panel: Comparison View -->
    <div class="flex-1 flex flex-col overflow-hidden">
      <!-- View Mode Tabs -->
      <div class="flex items-center justify-between px-4 py-2 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
        <div class="flex items-center gap-3">
          <button @click="emit('exit')" class="text-[var(--text-muted)] hover:text-[var(--text-primary)]">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
            </svg>
          </button>
          <h2 class="text-sm font-semibold text-[var(--text-primary)]">Comparison</h2>
          <span class="text-xs text-[var(--text-muted)]">({{ selectedResults.length }} results)</span>
        </div>

        <div class="flex items-center gap-2">
          <div class="bg-gray-100 dark:bg-gray-700 rounded-lg p-0.5 flex">
            <button
              v-for="tab in viewTabs"
              :key="tab.value"
              @click="viewMode = tab.value"
              :class="[
                'px-3 py-1 rounded-md text-xs transition-colors',
                viewMode === tab.value
                  ? 'bg-white dark:bg-gray-600 shadow text-pink-600 dark:text-pink-400'
                  : 'text-gray-600 dark:text-gray-400 hover:text-gray-900'
              ]"
            >
              {{ tab.label }}
            </button>
          </div>
          <button @click="exportReport" class="px-3 py-1.5 bg-blue-600 text-white rounded-lg text-xs hover:bg-blue-700">
            Export
          </button>
        </div>
      </div>

      <!-- Main Comparison Area -->
      <div class="flex-1 overflow-hidden">
        <!-- Empty state -->
        <div v-if="selectedResults.length < 2" class="h-full flex items-center justify-center">
          <div class="text-center">
            <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-gray-100 dark:bg-gray-800 flex items-center justify-center">
              <svg class="w-8 h-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
              </svg>
            </div>
            <p class="text-[var(--text-muted)] text-sm">Select at least 2 results to compare</p>
            <p class="text-[var(--text-muted)] text-xs mt-1">Use checkboxes on the left panel</p>
          </div>
        </div>

        <!-- Slider Comparison (Figma-style) -->
        <div v-else-if="viewMode === 'slider'" class="h-full relative">
          <div class="absolute inset-0 flex">
            <!-- Result A (left side) -->
            <div
              class="h-full overflow-hidden relative"
              :style="{ width: sliderPosition + '%' }"
            >
              <div class="w-full h-full bg-gradient-to-br from-pink-100 to-purple-100 dark:from-pink-900/20 dark:to-purple-900/20 flex items-center justify-center">
                <div class="text-center">
                  <div class="text-2xl font-bold text-pink-600 dark:text-pink-400">{{ selectedResults[0].name }}</div>
                  <div class="mt-2 space-y-1">
                    <div v-if="selectedResults[0].maxVonMises" class="text-sm text-[var(--text-secondary)]">
                      Stress: {{ selectedResults[0].maxVonMises.toFixed(1) }} MPa
                    </div>
                    <div v-if="selectedResults[0].maxDisplacement" class="text-sm text-[var(--text-secondary)]">
                      Disp: {{ selectedResults[0].maxDisplacement.toFixed(4) }} mm
                    </div>
                    <div v-if="selectedResults[0].compliance" class="text-sm text-[var(--text-secondary)]">
                      Compliance: {{ selectedResults[0].compliance.toFixed(4) }}
                    </div>
                  </div>
                  <!-- Simulated 3D viewport placeholder -->
                  <div class="mt-4 w-64 h-48 mx-auto bg-[var(--bg-surface)] rounded-lg border border-[var(--border-subtle)] flex items-center justify-center">
                    <div class="text-center text-[var(--text-muted)]">
                      <svg class="w-8 h-8 mx-auto mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M14 10l-2 1m0 0l-2-1m2 1v2.5M20 7l-2 1m2-1l-2-1m2 1v2.5M14 4l-2-1-2 1M4 7l2-1M4 7l2 1M4 7v2.5M12 21l-2-1m2 1l2-1m-2 1v-2.5M6 18l-2-1v-2.5M18 18l2-1v-2.5" />
                      </svg>
                      <p class="text-xs">3D Cloud Map A</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Result B (right side) -->
            <div
              class="h-full overflow-hidden relative"
              :style="{ width: (100 - sliderPosition) + '%' }"
            >
              <div class="w-full h-full bg-gradient-to-br from-blue-100 to-cyan-100 dark:from-blue-900/20 dark:to-cyan-900/20 flex items-center justify-center">
                <div class="text-center">
                  <div class="text-2xl font-bold text-blue-600 dark:text-blue-400">{{ selectedResults[1].name }}</div>
                  <div class="mt-2 space-y-1">
                    <div v-if="selectedResults[1].maxVonMises" class="text-sm text-[var(--text-secondary)]">
                      Stress: {{ selectedResults[1].maxVonMises.toFixed(1) }} MPa
                    </div>
                    <div v-if="selectedResults[1].maxDisplacement" class="text-sm text-[var(--text-secondary)]">
                      Disp: {{ selectedResults[1].maxDisplacement.toFixed(4) }} mm
                    </div>
                    <div v-if="selectedResults[1].compliance" class="text-sm text-[var(--text-secondary)]">
                      Compliance: {{ selectedResults[1].compliance.toFixed(4) }}
                    </div>
                  </div>
                  <!-- Simulated 3D viewport placeholder -->
                  <div class="mt-4 w-64 h-48 mx-auto bg-[var(--bg-surface)] rounded-lg border border-[var(--border-subtle)] flex items-center justify-center">
                    <div class="text-center text-[var(--text-muted)]">
                      <svg class="w-8 h-8 mx-auto mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M14 10l-2 1m0 0l-2-1m2 1v2.5M20 7l-2 1m2-1l-2-1m2 1v2.5M14 4l-2-1-2 1M4 7l2-1M4 7l2 1M4 7v2.5M12 21l-2-1m2 1l2-1m-2 1v-2.5M6 18l-2-1v-2.5M18 18l2-1v-2.5" />
                      </svg>
                      <p class="text-xs">3D Cloud Map B</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Slider Control -->
          <div
            class="absolute top-0 bottom-0 z-10 cursor-ew-resize"
            :style="{ left: `calc(${sliderPosition}% - 16px)` }"
            @mousedown="startSliderDrag"
          >
            <div class="w-8 h-full flex items-center justify-center">
              <div class="w-1 h-full bg-white shadow-lg rounded relative">
                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-8 h-8 bg-white rounded-full shadow-lg flex items-center justify-center border-2 border-gray-200">
                  <svg class="w-4 h-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l4-4 4 4m0 6l-4 4-4-4" />
                  </svg>
                </div>
              </div>
            </div>
          </div>

          <!-- Labels -->
          <div class="absolute top-3 left-3 px-2 py-1 bg-black/50 text-white text-xs rounded">
            A: {{ selectedResults[0].name }}
          </div>
          <div class="absolute top-3 right-3 px-2 py-1 bg-black/50 text-white text-xs rounded">
            B: {{ selectedResults[1].name }}
          </div>
        </div>

        <!-- Table Comparison -->
        <div v-else-if="viewMode === 'table'" class="h-full overflow-auto p-4">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[var(--border-subtle)] overflow-hidden">
            <table class="w-full">
              <thead class="bg-gray-50 dark:bg-gray-700">
                <tr>
                  <th class="px-4 py-3 text-left text-xs font-medium text-[var(--text-secondary)] uppercase">Metric</th>
                  <th
                    v-for="result in selectedResults"
                    :key="result.id"
                    class="px-4 py-3 text-left text-xs font-medium text-[var(--text-secondary)] uppercase"
                  >
                    {{ result.name }}
                  </th>
                  <th v-if="selectedResults.length === 2" class="px-4 py-3 text-left text-xs font-medium text-[var(--text-secondary)] uppercase">
                    Diff (%)
                  </th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
                <tr v-for="metric in comparisonMetrics" :key="metric.key">
                  <td class="px-4 py-3 text-sm text-[var(--text-primary)] font-medium">{{ metric.label }}</td>
                  <td
                    v-for="result in selectedResults"
                    :key="result.id"
                    class="px-4 py-3 text-sm text-[var(--text-primary)]"
                  >
                    {{ formatMetricValue(result[metric.key as keyof ComparisonResult], metric.unit) }}
                  </td>
                  <td v-if="selectedResults.length === 2" class="px-4 py-3 text-sm">
                    <span
                      :class="getDiffClass(metric.key)"
                    >
                      {{ computeDiff(metric.key) }}
                    </span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- Side-by-side 3D Comparison -->
        <div v-else-if="viewMode === 'sidebyside'" class="h-full flex gap-2 p-2">
          <div
            v-for="result in selectedResults"
            :key="result.id"
            class="flex-1 bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[var(--border-subtle)] flex flex-col overflow-hidden"
          >
            <div class="px-3 py-2 border-b border-[var(--border-subtle)] bg-gray-50 dark:bg-gray-700 flex items-center justify-between">
              <span class="text-xs font-medium text-[var(--text-primary)]">{{ result.name }}</span>
              <span v-if="result.maxVonMises" class="text-[10px] px-2 py-0.5 bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400 rounded">
                {{ result.maxVonMises.toFixed(1) }} MPa
              </span>
            </div>
            <div class="flex-1 bg-[var(--bg-base)] flex items-center justify-center">
              <div class="text-center text-[var(--text-muted)]">
                <svg class="w-12 h-12 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M14 10l-2 1m0 0l-2-1m2 1v2.5M20 7l-2 1m2-1l-2-1m2 1v2.5M14 4l-2-1-2 1M4 7l2-1M4 7l2 1M4 7v2.5M12 21l-2-1m2 1l2-1m-2 1v-2.5M6 18l-2-1v-2.5M18 18l2-1v-2.5" />
                </svg>
                <p class="text-xs">3D Viewport</p>
                <p class="text-[10px] mt-1">Synced OrbitControls</p>
              </div>
            </div>
            <div class="p-2 border-t border-[var(--border-subtle)] text-[10px] text-[var(--text-muted)] flex justify-between">
              <span>Disp: {{ result.maxDisplacement?.toFixed(4) || '-' }}</span>
              <span>{{ result.type }}</span>
            </div>
          </div>
        </div>

        <!-- Bar Chart Comparison (SVG) -->
        <div v-else-if="viewMode === 'chart'" class="h-full p-4 overflow-auto">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[var(--border-subtle)] p-4">
            <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-4">Key Metrics Comparison</h3>
            <svg :width="chartWidth" :height="chartHeight" class="w-full">
              <!-- Background -->
              <rect width="100%" height="100%" fill="transparent" />

              <!-- Grid lines -->
              <line v-for="i in 5" :key="'grid-' + i"
                :x1="60" :y1="20 + (i - 1) * ((chartHeight - 60) / 4)"
                :x2="chartWidth - 20" :y2="20 + (i - 1) * ((chartHeight - 60) / 4)"
                stroke="var(--border-subtle, #e5e7eb)" stroke-width="0.5"
              />

              <!-- Y-axis labels -->
              <text v-for="i in 5" :key="'yl-' + i"
                :x="55" :y="24 + (i - 1) * ((chartHeight - 60) / 4)"
                fill="var(--text-muted, #9ca3af)" font-size="10" text-anchor="end"
              >
                {{ (100 - (i - 1) * 25) }}%
              </text>

              <!-- Bars for each metric -->
              <g v-for="(metric, mIdx) in barMetrics" :key="metric.key">
                <!-- Metric label -->
                <text :x="30" :y="metricY(mIdx)" fill="var(--text-secondary, #6b7280)" font-size="10" text-anchor="middle" transform="rotate(-45, 30, 0)">
                  {{ metric.label }}
                </text>

                <!-- Bars for each result -->
                <rect
                  v-for="(result, rIdx) in selectedResults"
                  :key="result.id"
                  :x="barX(rIdx, selectedResults.length, metricY(mIdx))"
                  :y="barHeight(result, metric.key)"
                  :width="barWidth(selectedResults.length)"
                  :height="chartHeight - 60 - barHeight(result, metric.key)"
                  :fill="barColors[rIdx % barColors.length]"
                  rx="2"
                  opacity="0.8"
                />

                <!-- Value labels -->
                <text
                  v-for="(result, rIdx) in selectedResults"
                  :key="'val-' + result.id"
                  :x="barX(rIdx, selectedResults.length, metricY(mIdx)) + barWidth(selectedResults.length) / 2"
                  :y="barHeight(result, metric.key) - 4"
                  fill="var(--text-secondary, #6b7280)" font-size="9" text-anchor="middle"
                >
                  {{ formatBarValue(result, metric.key) }}
                </text>
              </g>

              <!-- Legend -->
              <g v-for="(result, rIdx) in selectedResults" :key="'legend-' + result.id">
                <rect :x="80 + rIdx * 120" :y="chartHeight - 25" width="12" height="12" :fill="barColors[rIdx % barColors.length]" rx="2" />
                <text :x="96 + rIdx * 120" :y="chartHeight - 15" fill="var(--text-secondary, #6b7280)" font-size="10">{{ result.name }}</text>
              </g>
            </svg>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive, watch, onMounted, onUnmounted } from 'vue'
import { useProjectStore } from '@/stores/project'
import { storeToRefs } from 'pinia'

interface ComparisonResult {
  id: string
  name: string
  type: string
  createdAt: string
  maxVonMises?: number
  maxDisplacement?: number
  volumeFraction?: number
  compliance?: number
  elementCount?: number
  frequencies?: number[]
  bucklingLoads?: number[]
  parameterValue?: number
}

const emit = defineEmits<{
  (e: 'exit'): void
  (e: 'export', data: { results: ComparisonResult[]; viewMode: string }): void
}>()

const projectStore = useProjectStore()
const { lastResult } = storeToRefs(projectStore)

const searchQuery = ref('')
type ViewMode = 'slider' | 'table' | 'sidebyside' | 'chart'

const viewMode = ref<ViewMode>('slider')
const sliderPosition = ref(50)
const isDraggingSlider = ref(false)
const expandedIds = reactive(new Set<string>())

const viewTabs: Array<{ value: ViewMode; label: string }> = [
  { value: 'slider', label: 'Slider' },
  { value: 'table', label: 'Table' },
  { value: 'sidebyside', label: 'Side by Side' },
  { value: 'chart', label: 'Chart' },
]

const barColors = ['#ec4899', '#3b82f6', '#10b981', '#f59e0b', '#8b5cf6']

// Demo data
const availableResults = ref<ComparisonResult[]>([
  {
    id: 'demo-1',
    name: 'Cantilever - 30% Vol',
    type: 'Topology',
    createdAt: '2025-04-01 10:30',
    maxVonMises: 245.8,
    maxDisplacement: 0.342,
    volumeFraction: 0.30,
    compliance: 12.45,
    elementCount: 1280,
  },
  {
    id: 'demo-2',
    name: 'Cantilever - 50% Vol',
    type: 'Topology',
    createdAt: '2025-04-01 11:15',
    maxVonMises: 189.3,
    maxDisplacement: 0.218,
    volumeFraction: 0.50,
    compliance: 8.72,
    elementCount: 1280,
  },
  {
    id: 'demo-3',
    name: 'L-Bracket Opt',
    type: 'Shape',
    createdAt: '2025-04-01 14:00',
    maxVonMises: 312.5,
    maxDisplacement: 0.456,
    volumeFraction: 0.45,
    compliance: 15.67,
    elementCount: 960,
  },
  {
    id: 'demo-4',
    name: 'Simply Supported',
    type: 'Topology',
    createdAt: '2025-04-02 09:00',
    maxVonMises: 178.2,
    maxDisplacement: 0.189,
    volumeFraction: 0.40,
    compliance: 7.34,
    elementCount: 1536,
  },
])

const selectedResultIds = ref<string[]>([])

const filteredResults = computed(() => {
  if (!searchQuery.value) return availableResults.value
  const q = searchQuery.value.toLowerCase()
  return availableResults.value.filter(r =>
    r.name.toLowerCase().includes(q) || r.type.toLowerCase().includes(q)
  )
})

// Auto-select first two
const selectedResults = computed(() =>
  availableResults.value.filter(r => selectedResultIds.value.includes(r.id))
)

watch(() => availableResults.value, (newResults) => {
  if (newResults.length >= 2 && selectedResultIds.value.length === 0) {
    selectedResultIds.value = [newResults[0].id, newResults[1].id]
  }
}, { immediate: true })

function toggleResultSelection(id: string) {
  const idx = selectedResultIds.value.indexOf(id)
  if (idx >= 0) {
    selectedResultIds.value.splice(idx, 1)
  } else {
    if (selectedResultIds.value.length >= 4) {
      selectedResultIds.value.shift()
    }
    selectedResultIds.value.push(id)
  }
}

function toggleExpand(id: string) {
  if (expandedIds.has(id)) {
    expandedIds.delete(id)
  } else {
    expandedIds.add(id)
  }
}

function addDemoResults() {
  const id = 'demo-' + (availableResults.value.length + 1)
  availableResults.value.push({
    id,
    name: `Parametric Run ${availableResults.value.length + 1}`,
    type: 'Structural',
    createdAt: new Date().toLocaleString(),
    maxVonMises: 150 + Math.random() * 200,
    maxDisplacement: 0.1 + Math.random() * 0.5,
    volumeFraction: 0.2 + Math.random() * 0.5,
    compliance: 5 + Math.random() * 15,
    elementCount: 800 + Math.floor(Math.random() * 1000),
  })
}

// Slider drag
function startSliderDrag(event: MouseEvent) {
  isDraggingSlider.value = true
  updateSliderPosition(event)

  const onMove = (e: MouseEvent) => {
    if (isDraggingSlider.value) updateSliderPosition(e)
  }
  const onUp = () => {
    isDraggingSlider.value = false
    window.removeEventListener('mousemove', onMove)
    window.removeEventListener('mouseup', onUp)
  }
  window.addEventListener('mousemove', onMove)
  window.addEventListener('mouseup', onUp)
}

function updateSliderPosition(event: MouseEvent) {
  const container = event.currentTarget as HTMLElement
  if (!container) return
  const rect = container.parentElement?.getBoundingClientRect()
  if (!rect) return
  const x = event.clientX - rect.left
  const pct = Math.max(5, Math.min(95, (x / rect.width) * 100))
  sliderPosition.value = pct
}

// Table comparison
const comparisonMetrics = [
  { key: 'maxVonMises', label: 'Max Von Mises Stress', unit: 'MPa' },
  { key: 'maxDisplacement', label: 'Max Displacement', unit: 'mm' },
  { key: 'volumeFraction', label: 'Volume Fraction', unit: '' },
  { key: 'compliance', label: 'Compliance', unit: '' },
  { key: 'elementCount', label: 'Element Count', unit: '' },
]

function formatMetricValue(value: unknown, unit?: string): string {
  if (value === undefined || value === null) return '-'
  if (typeof value === 'number') {
    return value.toFixed(4) + (unit ? ` ${unit}` : '')
  }
  return String(value)
}

function computeDiff(metricKey: string): string {
  const results = selectedResults.value
  if (results.length < 2) return '-'

  const val0 = results[0][metricKey as keyof ComparisonResult] as unknown as number
  const val1 = results[1][metricKey as keyof ComparisonResult] as unknown as number

  if (typeof val0 === 'number' && typeof val1 === 'number' && val0 !== 0) {
    const diff = ((val1 - val0) / val0 * 100)
    const sign = diff > 0 ? '+' : ''
    return `${sign}${diff.toFixed(1)}%`
  }

  return '-'
}

function getDiffClass(metricKey: string): string {
  const results = selectedResults.value
  if (results.length < 2) return ''

  const val0 = results[0][metricKey as keyof ComparisonResult] as unknown as number
  const val1 = results[1][metricKey as keyof ComparisonResult] as unknown as number

  if (typeof val0 === 'number' && typeof val1 === 'number' && val0 !== 0) {
    const diff = (val1 - val0) / val0 * 100
    if (Math.abs(diff) < 1) return 'text-gray-500'
    if (diff > 0) return 'text-red-600 font-medium'
    return 'text-green-600 font-medium'
  }

  return 'text-gray-500'
}

// Bar chart
const chartWidth = ref(700)
const chartHeight = ref(350)

const barMetrics = computed(() =>
  comparisonMetrics.filter(m => {
    return selectedResults.value.some(r => r[m.key as keyof ComparisonResult] != null)
  })
)

function metricY(idx: number): number {
  const groupHeight = (chartHeight.value - 60) / barMetrics.value.length
  return 40 + idx * groupHeight + groupHeight / 2
}

function barX(resultIdx: number, total: number, _centerY: number): number {
  const groupWidth = (chartWidth.value - 80) / barMetrics.value.length
  const barW = Math.min(30, (groupWidth - 20) / total)
  const groupStart = 60 + Math.floor(_centerY / ((chartHeight.value - 60) / barMetrics.value.length)) * groupWidth
  return groupStart + 10 + resultIdx * (barW + 4)
}

function barWidth(total: number): number {
  const groupWidth = (chartWidth.value - 80) / barMetrics.value.length
  return Math.min(30, (groupWidth - 20) / total)
}

function barHeight(result: ComparisonResult, key: string): number {
  const val = result[key as keyof ComparisonResult] as unknown as number
  if (typeof val !== 'number') return chartHeight.value - 60

  // Normalize: find max across all selected results for this metric
  const maxVal = Math.max(...selectedResults.value.map(r => {
    const v = r[key as keyof ComparisonResult] as unknown as number
    return typeof v === 'number' ? v : 0
  }))

  if (maxVal === 0) return chartHeight.value - 60
  const ratio = val / maxVal
  return chartHeight.value - 60 - ratio * (chartHeight.value - 80)
}

function formatBarValue(result: ComparisonResult, key: string): string {
  const val = result[key as keyof ComparisonResult] as unknown as number
  if (typeof val !== 'number') return '-'
  if (val > 1000) return val.toFixed(0)
  if (val > 10) return val.toFixed(1)
  return val.toFixed(3)
}

function exportReport() {
  emit('export', {
    results: selectedResults.value,
    viewMode: viewMode.value,
  })
}

// Load from store
function loadResultsFromStore() {
  if (lastResult.value) {
    const exists = availableResults.value.some(r => r.id === 'store-current')
    if (!exists) {
      availableResults.value.push({
        id: 'store-current',
        name: 'Current Result',
        type: 'Structural',
        createdAt: new Date().toLocaleString(),
        maxVonMises: lastResult.value.stats?.max,
        maxDisplacement: undefined,
      })
    }
  }
}

onMounted(() => {
  loadResultsFromStore()
})
</script>

<style scoped>
.comparison-view {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}
</style>
