<template>
  <div class="comparison-view h-full flex flex-col bg-gray-50">
    <!-- Header -->
    <div class="bg-white border-b px-4 py-3 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <button @click="exitComparison" class="text-gray-500 hover:text-gray-700">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <h2 class="text-lg font-semibold">结果对比</h2>
        <span class="text-sm text-gray-500">({{ selectedResults.length }} 个结果)</span>
      </div>
      
      <div class="flex items-center gap-2">
        <!-- View mode tabs -->
        <div class="bg-gray-100 rounded-lg p-1 flex">
          <button
            v-for="tab in viewTabs"
            :key="tab.value"
            @click="viewMode = tab.value"
            :class="[
              'px-3 py-1 rounded-md text-sm transition-colors',
              viewMode === tab.value 
                ? 'bg-white shadow text-blue-600' 
                : 'text-gray-600 hover:text-gray-900'
            ]"
          >
            {{ tab.label }}
          </button>
        </div>
        
        <button @click="exportReport" class="px-3 py-1.5 bg-blue-600 text-white rounded-lg text-sm hover:bg-blue-700">
          导出报告
        </button>
      </div>
    </div>

    <!-- Result Selector -->
    <div class="bg-white border-b px-4 py-2 flex items-center gap-4 overflow-x-auto">
      <span class="text-sm text-gray-600 shrink-0">选择对比结果:</span>
      <div class="flex gap-2">
        <label
          v-for="result in availableResults"
          :key="result.id"
          class="flex items-center gap-2 px-3 py-1.5 bg-gray-50 rounded-lg cursor-pointer hover:bg-gray-100 transition-colors"
        >
          <input
            type="checkbox"
            :value="result.id"
            v-model="selectedResultIds"
            class="rounded text-blue-600"
          />
          <div class="flex flex-col">
            <span class="text-sm font-medium">{{ result.name }}</span>
            <span class="text-xs text-gray-500">{{ result.type }}</span>
          </div>
        </label>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 overflow-hidden">
      <!-- Cloud Image Comparison -->
      <div v-if="viewMode === 'cloud'" class="h-full flex">
        <div class="flex-1 flex gap-1 p-2 overflow-auto">
          <div 
            v-for="result in selectedResults" 
            :key="result.id"
            class="flex-1 min-w-[300px] flex flex-col bg-white rounded-lg shadow"
          >
            <div class="px-3 py-2 border-b flex items-center justify-between bg-gray-50 rounded-t-lg">
              <span class="font-medium text-sm">{{ result.name }}</span>
              <span 
                v-if="result.maxVonMises" 
                class="text-xs px-2 py-0.5 bg-red-100 text-red-700 rounded"
              >
                σ_max: {{ result.maxVonMises.toFixed(1) }}
              </span>
            </div>
            <div class="flex-1 relative bg-gray-100 flex items-center justify-center">
              <div class="text-center text-gray-400">
                <div class="text-2xl mb-1">📊</div>
                <p class="text-sm">3D视图</p>
              </div>
            </div>
            <div class="p-2 border-t text-xs text-gray-500 flex justify-between">
              <span>位移: {{ result.maxDisplacement?.toFixed(4) || '-' }}</span>
              <span>类型: {{ result.type }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Table Comparison -->
      <div v-else-if="viewMode === 'table'" class="h-full overflow-auto p-4">
        <table class="w-full bg-white rounded-lg shadow overflow-hidden">
          <thead class="bg-gray-100">
            <tr>
              <th class="px-4 py-3 text-left text-sm font-medium text-gray-700">指标</th>
              <th 
                v-for="result in selectedResults" 
                :key="result.id"
                class="px-4 py-3 text-left text-sm font-medium text-gray-700"
              >
                {{ result.name }}
              </th>
              <th class="px-4 py-3 text-left text-sm font-medium text-gray-700">差异(%)</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200">
            <tr v-for="metric in tableMetrics" :key="metric.key">
              <td class="px-4 py-3 text-sm text-gray-900">{{ metric.label }}</td>
              <td 
                v-for="result in selectedResults" 
                :key="result.id"
                class="px-4 py-3 text-sm"
              >
                {{ formatValue(result[metric.key as keyof ComparisonResult], metric.unit) }}
              </td>
              <td class="px-4 py-3 text-sm text-gray-500">
                {{ computeDifference(metric.key) }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Curve Comparison -->
      <div v-else-if="viewMode === 'curve'" class="h-full p-4">
        <div class="bg-white rounded-lg shadow p-4 h-full flex flex-col">
          <div class="flex items-center justify-between mb-4">
            <h3 class="font-medium">参数-结果曲线</h3>
            <select v-model="curveXAxis" class="text-sm border rounded px-2 py-1">
              <option value="parameter">扫描参数</option>
              <option value="load">载荷大小</option>
              <option value="step">分析步</option>
            </select>
          </div>
          <div class="flex-1 flex items-center justify-center text-gray-400">
            <div class="text-center">
              <div class="text-4xl mb-2">📈</div>
              <p>曲线对比视图</p>
              <p class="text-sm mt-1">显示参数化扫描的曲线对比</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Diff Visualization -->
      <div v-else-if="viewMode === 'diff'" class="h-full flex">
        <div class="flex-1 p-4">
          <div class="bg-white rounded-lg shadow h-full flex flex-col">
            <div class="px-4 py-3 border-b flex items-center justify-between">
              <h3 class="font-medium">差异云图</h3>
              <div class="flex gap-2">
                <button
                  v-for="diffType in diffTypes"
                  :key="diffType.value"
                  @click="diffMode = diffType.value"
                  :class="[
                    'px-2 py-1 text-xs rounded',
                    diffMode === diffType.value 
                      ? 'bg-blue-600 text-white' 
                      : 'bg-gray-100 text-gray-600'
                  ]"
                >
                  {{ diffType.label }}
                </button>
              </div>
            </div>
            <div class="flex-1 flex items-center justify-center text-gray-400">
              <div class="text-center">
                <div class="text-4xl mb-2">🔄</div>
                <p>差异云图</p>
                <p class="text-sm mt-1">显示两个结果之间的差异</p>
              </div>
            </div>
            <div class="p-3 border-t text-sm">
              <div class="flex items-center gap-4">
                <span class="text-gray-600">差异范围:</span>
                <span class="font-medium text-red-600">{{ diffStats.min.toFixed(2) }}</span>
                <span class="text-gray-400">~</span>
                <span class="font-medium text-blue-600">{{ diffStats.max.toFixed(2) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty state -->
    <div v-if="selectedResults.length < 2" class="flex-1 flex items-center justify-center">
      <div class="text-center text-gray-400">
        <div class="text-4xl mb-2">⚖️</div>
        <p>请选择至少2个结果进行对比</p>
        <p class="text-sm mt-1">从上方列表中选择要对比的结果</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useProjectStore } from '@/stores/project'
import { storeToRefs } from 'pinia'

interface ComparisonResult {
  id: string
  name: string
  type: 'structural' | 'modal' | 'buckling' | 'thermal'
  nodes: Array<{ id: number; x: number; y: number; z: number }>
  elements: Array<{ id: number; type: string; nodeIds: number[] }>
  vonMises?: Record<string, number>
  displacement?: Record<string, { ux: number; uy: number; uz: number }>
  maxVonMises?: number
  maxDisplacement?: number
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

// View modes
const viewTabs = [
  { value: 'cloud', label: '云图对比' },
  { value: 'table', label: '表格对比' },
  { value: 'curve', label: '曲线对比' },
  { value: 'diff', label: '差异云图' }
]

const viewMode = ref('cloud')
const curveXAxis = ref('parameter')
const diffMode = ref('absolute')

const diffTypes = [
  { value: 'absolute', label: '绝对差异' },
  { value: 'relative', label: '相对差异(%)' }
]

// Convert store result to comparison format
function convertToComparisonResult(result: any, index: number, paramValue?: number): ComparisonResult {
  const converted: ComparisonResult = {
    id: `result_${index}`,
    name: paramValue !== undefined 
      ? `工况 ${index + 1} (参数=${paramValue})` 
      : `结果 ${index + 1}`,
    type: 'structural',
    nodes: result.nodes || [],
    elements: result.elements || [],
    maxVonMises: result.stats?.max,
    maxDisplacement: undefined,
    parameterValue: paramValue
  }
  
  // Extract vonMises if available
  if (result.node_values && result.node_values.length > 0) {
    const vonMises: Record<string, number> = {}
    result.node_values[0].forEach((nv: { node_id: number; value: number }) => {
      vonMises[String(nv.node_id)] = nv.value
    })
    converted.vonMises = vonMises
  }
  
  return converted
}

// Available results - load from store
const availableResults = ref<ComparisonResult[]>([])

function loadResultsFromStore() {
  const results: ComparisonResult[] = []
  
  // Load current result from store
  if (lastResult.value) {
    results.push(convertToComparisonResult(lastResult.value, 0))
  }
  
  availableResults.value = results
}

const selectedResultIds = ref<string[]>([])

// Auto-select first two results when available
watch(() => availableResults.value, (newResults) => {
  if (newResults.length >= 2 && selectedResultIds.value.length === 0) {
    selectedResultIds.value = [newResults[0].id, newResults[1].id]
  }
}, { immediate: true })

onMounted(() => {
  loadResultsFromStore()
})

const selectedResults = computed(() => 
  availableResults.value.filter(r => selectedResultIds.value.includes(r.id))
)

const tableMetrics = [
  { key: 'maxVonMises', label: '最大Von Mises应力', unit: 'MPa' },
  { key: 'maxDisplacement', label: '最大位移', unit: 'm' },
  { key: 'frequencies', label: '固有频率', unit: 'Hz' },
  { key: 'bucklingLoads', label: '临界载荷', unit: 'kN' }
]

const diffStats = computed(() => {
  const results = selectedResults.value
  if (results.length < 2 || !results[0].vonMises || !results[1].vonMises) {
    return { min: 0, max: 0 }
  }
  
  let min = Infinity
  let max = -Infinity
  
  const vm0 = results[0].vonMises!
  const vm1 = results[1].vonMises!
  
  Object.keys(vm0).forEach(key => {
    const diff = Math.abs((vm0[key] || 0) - (vm1[key] || 0))
    if (diff < min) min = diff
    if (diff > max) max = diff
  })
  
  return { min: min || 0, max: max || 0 }
})

function formatValue(value: unknown, unit?: string): string {
  if (value === undefined || value === null) return '-'
  if (typeof value === 'number') {
    return value.toFixed(4) + (unit ? ` ${unit}` : '')
  }
  if (Array.isArray(value)) {
    return value.slice(0, 3).join(', ') + (value.length > 3 ? '...' : '')
  }
  return String(value)
}

function computeDifference(metricKey: string): string {
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

function exitComparison() {
  emit('exit')
}

function exportReport() {
  emit('export', {
    results: selectedResults.value,
    viewMode: viewMode.value
  })
}
</script>

<style scoped>
.comparison-view {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}
</style>
