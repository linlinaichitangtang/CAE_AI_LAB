<template>
  <div class="h-full flex flex-col bg-gray-50">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-white border-b">
      <div>
        <h2 class="text-lg font-semibold text-gray-800">仿真分析</h2>
        <p class="text-sm text-gray-500">结果可视化与后处理</p>
      </div>
      <div class="flex items-center gap-2">
        <button 
          @click="loadSampleData"
          class="px-3 py-1.5 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 transition"
        >
          加载示例数据
        </button>
        <button 
          @click="exportScreenshot"
          class="px-3 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50 transition"
        >
          导出图片
        </button>
      </div>
    </div>

    <!-- Main content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- 3D Viewer -->
      <div class="flex-1 relative bg-gray-100">
        <ResultViewer
          ref="viewerRef"
          :result="currentResult"
          :display-mode="displayMode"
          :show-deformed="showDeformed"
          :deformation-scale="deformationScale"
          :colormap="colormap"
          :show-wireframe="showWireframe"
          @node-click="onNodeClick"
          @ready="onViewerReady"
        />

        <!-- Color legend overlay -->
        <div 
          v-if="currentResult"
          class="absolute bottom-4 right-4 bg-white/90 backdrop-blur rounded-lg p-3 shadow-lg"
        >
          <ColorLegend
            :min="colorMin"
            :max="colorMax"
            :title="legendTitle"
            :colormap="colormap"
            :unit="legendUnit"
          />
        </div>

        <!-- No data placeholder -->
        <div 
          v-if="!currentResult"
          class="absolute inset-0 flex items-center justify-center"
        >
          <div class="text-center text-gray-400">
            <div class="text-4xl mb-2">📊</div>
            <p>暂无仿真结果</p>
            <p class="text-sm mt-1">运行仿真后，结果将显示在这里</p>
          </div>
        </div>
      </div>

      <!-- Control Panel -->
      <div class="w-72 bg-white border-l overflow-y-auto">
        <div class="p-4 space-y-6">
          <!-- Display Mode -->
          <div>
            <h3 class="text-sm font-medium text-gray-700 mb-2">显示模式</h3>
            <div class="space-y-2">
              <label class="flex items-center gap-2 cursor-pointer">
                <input 
                  type="radio" 
                  v-model="displayMode" 
                  value="vonMises"
                  class="text-blue-600"
                />
                <span class="text-sm">Von Mises 应力</span>
              </label>
              <label class="flex items-center gap-2 cursor-pointer">
                <input 
                  type="radio" 
                  v-model="displayMode" 
                  value="displacement"
                  class="text-blue-600"
                />
                <span class="text-sm">位移</span>
              </label>
              <label class="flex items-center gap-2 cursor-pointer">
                <input 
                  type="radio" 
                  v-model="displayMode" 
                  value="stress"
                  class="text-blue-600"
                />
                <span class="text-sm">应力分量</span>
              </label>
            </div>
          </div>

          <!-- Deformation -->
          <div>
            <h3 class="text-sm font-medium text-gray-700 mb-2">变形显示</h3>
            <div class="space-y-3">
              <label class="flex items-center gap-2 cursor-pointer">
                <input 
                  type="checkbox" 
                  v-model="showDeformed"
                  class="rounded text-blue-600"
                />
                <span class="text-sm">显示变形后形状</span>
              </label>
              <div v-if="showDeformed">
                <label class="text-xs text-gray-500 mb-1 block">变形比例: {{ deformationScale.toFixed(1) }}x</label>
                <input 
                  type="range" 
                  v-model.number="deformationScale"
                  min="0.1" 
                  max="10" 
                  step="0.1"
                  class="w-full"
                />
              </div>
            </div>
          </div>

          <!-- Colormap -->
          <div>
            <h3 class="text-sm font-medium text-gray-700 mb-2">颜色图例</h3>
            <select 
              v-model="colormap"
              class="w-full px-3 py-2 border rounded text-sm"
            >
              <option value="viridis">Viridis</option>
              <option value="plasma">Plasma</option>
              <option value="inferno">Inferno</option>
              <option value="jet">Jet</option>
              <option value="rainbow">Rainbow</option>
            </select>
          </div>

          <!-- Display Options -->
          <div>
            <h3 class="text-sm font-medium text-gray-700 mb-2">显示选项</h3>
            <div class="space-y-2">
              <label class="flex items-center gap-2 cursor-pointer">
                <input 
                  type="checkbox" 
                  v-model="showWireframe"
                  class="rounded text-blue-600"
                />
                <span class="text-sm">显示网格线</span>
              </label>
              <label class="flex items-center gap-2 cursor-pointer">
                <input 
                  type="checkbox" 
                  v-model="showAxes"
                  class="rounded text-blue-600"
                />
                <span class="text-sm">显示坐标轴</span>
              </label>
            </div>
          </div>

          <!-- Statistics -->
          <div v-if="currentResult">
            <h3 class="text-sm font-medium text-gray-700 mb-2">统计信息</h3>
            <div class="bg-gray-50 rounded p-3 text-xs space-y-1">
              <div class="flex justify-between">
                <span class="text-gray-500">节点数</span>
                <span class="font-medium">{{ currentResult.nodes.length }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-500">单元数</span>
                <span class="font-medium">{{ currentResult.elements.length }}</span>
              </div>
              <div v-if="currentResult.vonMises">
                <div class="flex justify-between mt-2 pt-2 border-t">
                  <span class="text-gray-500">最大 Von Mises</span>
                  <span class="font-medium">{{ formatValue(colorMax) }} MPa</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-500">最小 Von Mises</span>
                  <span class="font-medium">{{ formatValue(colorMin) }} MPa</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Node Info -->
          <div v-if="selectedNode">
            <h3 class="text-sm font-medium text-gray-700 mb-2">选中节点</h3>
            <div class="bg-gray-50 rounded p-3 text-xs space-y-1">
              <div class="flex justify-between">
                <span class="text-gray-500">ID</span>
                <span class="font-medium">{{ selectedNode.id }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-500">X</span>
                <span>{{ selectedNode.x.toFixed(4) }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-500">Y</span>
                <span>{{ selectedNode.y.toFixed(4) }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-500">Z</span>
                <span>{{ selectedNode.z.toFixed(4) }}</span>
              </div>
              <div v-if="selectedValue !== null" class="flex justify-between mt-2 pt-2 border-t">
                <span class="text-gray-500">{{ legendTitle }}</span>
                <span class="font-medium">{{ formatValue(selectedValue) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import ResultViewer from '../components/simulation/ResultViewer.vue'
import ColorLegend from '../components/simulation/ColorLegend.vue'
import { generateSampleResult } from '../components/simulation/simulationParser'
import type { SimulationResult } from '../types'

const viewerRef = ref<InstanceType<typeof ResultViewer>>()
const currentResult = ref<SimulationResult | null>(null)
const displayMode = ref<'displacement' | 'stress' | 'vonMises'>('vonMises')
const showDeformed = ref(false)
const deformationScale = ref(1.0)
const colormap = ref<'viridis' | 'plasma' | 'inferno' | 'jet' | 'rainbow'>('viridis')
const showWireframe = ref(false)
const showAxes = ref(true)
const selectedNode = ref<{ id: number; x: number; y: number; z: number } | null>(null)
const selectedValue = ref<number | null>(null)

const legendTitle = computed(() => {
  switch (displayMode.value) {
    case 'vonMises': return 'Von Mises 应力'
    case 'displacement': return '位移'
    case 'stress': return '应力'
    default: return '值'
  }
})

const legendUnit = computed(() => {
  return displayMode.value === 'displacement' ? 'mm' : 'MPa'
})

const colorMin = computed(() => {
  const data = viewerRef.value?.getColorData?.()
  return data?.minVal ?? 0
})

const colorMax = computed(() => {
  const data = viewerRef.value?.getColorData?.()
  return data?.maxVal ?? 100
})

const formatValue = (v: number): string => {
  if (Math.abs(v) >= 1e6) return (v / 1e6).toFixed(2) + 'M'
  if (Math.abs(v) >= 1e3) return (v / 1e3).toFixed(2) + 'k'
  return v.toFixed(2)
}

function loadSampleData() {
  currentResult.value = generateSampleResult()
}

function onViewerReady() {
  console.log('Result viewer ready')
}

function onNodeClick(nodeId: number, value: number) {
  const node = currentResult.value?.nodes.find(n => n.id === nodeId)
  if (node) {
    selectedNode.value = node
    selectedValue.value = value
  }
}

function exportScreenshot() {
  // TODO: Implement screenshot export
  console.log('Export screenshot - to be implemented with canvas capture')
}

// Watch display mode change
import { watch } from 'vue'
watch(displayMode, () => {
  if (currentResult.value) {
    // Result viewer will update automatically via props
  }
})
</script>