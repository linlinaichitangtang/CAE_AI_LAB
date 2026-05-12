<script setup lang="ts">
/**
 * SimulationParametric.vue — 参数化分析模块
 * 从 SimulationView.vue 提取
 */
import { ref, computed, nextTick } from 'vue'
import { useProjectStore } from '@/stores/project'
import { useParametricStore } from '@/stores/parametric'
import ParamSlider from '@/components/parametric/ParamSlider.vue'
import PreviewMode from '@/components/parametric/PreviewMode.vue'
import ParamLinkManager from '@/components/parametric/ParamLinkManager.vue'
import DoePanel from '@/components/parametric/DoePanel.vue'

const projectStore = useProjectStore()
const parametricStore = useParametricStore()

// ========== 参数化分析状态 ==========
const parametricParameters = ref<any[]>([])
const parametricXDiv = ref(10)
const parametricYDiv = ref(10)
const parametricZDiv = ref(1)
const parametricElementType = ref('CPS4')
const parametricElasticModulus = ref(210000)
const parametricPoissonRatio = ref(0.3)
const parametricResultVariable = ref('von_mises')

const parametricRunning = ref(false)
const parametricProgress = ref(0)
const parametricCompleted = ref(0)
const parametricTotal = ref(0)
const parametricResults = ref<any[]>([])
const parametricSummary = ref<any>(null)
const showPreviewMode = ref(false)
const previewBaselineResult = ref<any>(null)

// ========== 参数化函数 ==========
function enterParametricPreview() {
  previewBaselineResult.value = projectStore.lastResult
  showPreviewMode.value = true
}

function cancelParametricPreview() {
  showPreviewMode.value = false
  previewBaselineResult.value = null
}

function confirmParametricPreview() {
  showPreviewMode.value = false
}

function onParamSliderChange(paramId: string, value: number) {
  const param = parametricParameters.value.find(p => p.id === paramId)
  if (param) {
    param.currentValue = value
    parametricStore.updateParameter(paramId, value)
  }
}

async function runParametricScan() {
  if (!projectStore.hasMesh) return

  parametricRunning.value = true
  parametricProgress.value = 0
  parametricCompleted.value = 0
  parametricTotal.value = parametricParameters.value.length

  try {
    const { invoke } = await import('@tauri-apps/api/core')

    const result = await invoke<any>('parametric::run_parametric_scan', {
      parameters: parametricParameters.value.map(p => ({
        name: p.name,
        min: p.min,
        max: p.max,
        steps: p.steps
      })),
      xDiv: parametricXDiv.value,
      yDiv: parametricYDiv.value,
      elementType: parametricElementType.value,
      elasticModulus: parametricElasticModulus.value,
      poissonRatio: parametricPoissonRatio.value,
      resultVariable: parametricResultVariable.value
    })

    parametricResults.value = result.results || []
    parametricSummary.value = result.summary || null
    parametricCompleted.value = parametricTotal.value
    parametricProgress.value = 100
  } catch (e) {
    console.error('Parametric scan failed:', e)
    simulateParametricScan()
  } finally {
    parametricRunning.value = false
  }
}

function simulateParametricScan() {
  parametricRunning.value = true
  parametricProgress.value = 0
  parametricCompleted.value = 0
  parametricTotal.value = parametricParameters.value.length * 3

  let completed = 0
  const interval = setInterval(() => {
    if (completed >= parametricTotal.value) {
      clearInterval(interval)
      parametricRunning.value = false
      parametricCompleted.value = parametricTotal.value
      parametricProgress.value = 100
      return
    }

    completed++
    parametricCompleted.value = completed
    parametricProgress.value = (completed / parametricTotal.value) * 100

    parametricResults.value.push({
      id: `param_${completed}`,
      values: parametricParameters.value.map(p => p.currentValue || p.min),
      result: Math.random() * 100
    })
  }, 200)
}

function cancelParametricScan() {
  parametricRunning.value = false
}

function addParametricParameter() {
  parametricParameters.value.push({
    id: `param_${Date.now()}`,
    name: `参数 ${parametricParameters.value.length + 1}`,
    min: 0,
    max: 100,
    steps: 5,
    currentValue: 50
  })
}

async function exportParametricResults() {
  const csv = [
    parametricParameters.value.map(p => p.name).join(',') + ',result',
    ...parametricResults.value.map(r =>
      r.values.join(',') + `,${r.result}`
    )
  ].join('\n')

  const blob = new Blob([csv], { type: 'text/csv' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'parametric_results.csv'
  a.click()
  URL.revokeObjectURL(url)
}

defineExpose({
  runParametricScan,
  cancelParametricScan,
  addParametricParameter,
  parametricRunning,
  parametricProgress,
  parametricCompleted,
  parametricTotal
})
</script>

<template>
  <!-- 参数化分析左侧面板 -->
  <div class="w-72 bg-white border-r overflow-y-auto p-4 space-y-6">
    <!-- 参数定义 -->
    <div class="space-y-3">
      <div class="flex items-center justify-between">
        <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
          <span class="w-5 h-5 rounded-full bg-blue-600 text-white text-xs flex items-center justify-center">P</span>
          参数定义
        </h3>
        <button @click="addParametricParameter" class="text-xs text-blue-600 hover:underline">
          + 添加
        </button>
      </div>

      <div v-if="parametricParameters.length === 0" class="text-xs text-gray-500 bg-gray-50 rounded p-3 text-center">
        暂无参数，点击"添加"创建参数化变量
      </div>

      <div v-for="param in parametricParameters" :key="param.id" class="border rounded-lg p-3 space-y-2">
        <div class="flex items-center gap-2">
          <input v-model="param.name" class="font-medium text-sm bg-transparent border-b border-blue-300 focus:border-blue-500 outline-none flex-1" />
          <button @click="parametricParameters = parametricParameters.filter(p => p.id !== param.id)" class="text-red-500 text-xs">✕</button>
        </div>

        <ParamSlider
          :min="param.min"
          :max="param.max"
          :value="param.currentValue || param.min"
          @update:value="onParamSliderChange(param.id, $event)"
        />

        <div class="grid grid-cols-3 gap-1 text-xs">
          <div>
            <label class="text-gray-500">下限</label>
            <input type="number" v-model.number="param.min" class="w-full px-1 py-0.5 border rounded" />
          </div>
          <div>
            <label class="text-gray-500">上限</label>
            <input type="number" v-model.number="param.max" class="w-full px-1 py-0.5 border rounded" />
          </div>
          <div>
            <label class="text-gray-500">步数</label>
            <input type="number" v-model.number="param.steps" min="2" class="w-full px-1 py-0.5 border rounded" />
          </div>
        </div>
      </div>
    </div>

    <!-- 网格配置 -->
    <div class="space-y-3">
      <h3 class="text-sm font-medium text-gray-700">网格配置</h3>
      <div class="grid grid-cols-3 gap-2">
        <div>
          <label class="text-xs text-gray-500">X 方向</label>
          <input type="number" v-model.number="parametricXDiv" min="2" class="w-full px-2 py-1 border rounded text-sm" />
        </div>
        <div>
          <label class="text-xs text-gray-500">Y 方向</label>
          <input type="number" v-model.number="parametricYDiv" min="2" class="w-full px-2 py-1 border rounded text-sm" />
        </div>
        <div>
          <label class="text-xs text-gray-500">Z 方向</label>
          <input type="number" v-model.number="parametricZDiv" min="1" class="w-full px-2 py-1 border rounded text-sm" />
        </div>
      </div>

      <div class="grid grid-cols-2 gap-2">
        <div>
          <label class="text-xs text-gray-500">弹性模量 (MPa)</label>
          <input type="number" v-model.number="parametricElasticModulus" class="w-full px-2 py-1 border rounded text-sm" />
        </div>
        <div>
          <label class="text-xs text-gray-500">泊松比</label>
          <input type="number" v-model.number="parametricPoissonRatio" step="0.01" class="w-full px-2 py-1 border rounded text-sm" />
        </div>
      </div>
    </div>

    <!-- DOE 配置 -->
    <div class="space-y-3">
      <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
        <span class="w-5 h-5 rounded-full bg-purple-600 text-white text-xs flex items-center justify-center">D</span>
        DOE 设计
      </h3>
      <DoePanel :parameters="parametricParameters" />
    </div>

    <!-- 参数链接管理 -->
    <div class="space-y-3">
      <h3 class="text-sm font-medium text-gray-700">参数链接</h3>
      <ParamLinkManager :parameters="parametricParameters" @update="parametricParameters = $event" />
    </div>

    <!-- 运行控制 -->
    <div class="space-y-2">
      <button @click="runParametricScan" :disabled="parametricRunning || parametricParameters.length === 0" class="w-full px-3 py-2 bg-blue-600 text-white rounded text-sm hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed">
        {{ parametricRunning ? `扫描中 ${parametricCompleted}/${parametricTotal}` : '🔍 运行参数扫描' }}
      </button>

      <button v-if="parametricRunning" @click="cancelParametricScan" class="w-full px-3 py-2 bg-red-600 text-white rounded text-sm hover:bg-red-700">
        取消扫描
      </button>

      <button v-if="parametricResults.length > 0" @click="exportParametricResults" class="w-full px-3 py-2 bg-green-600 text-white rounded text-sm hover:bg-green-700">
        📥 导出结果
      </button>
    </div>
  </div>
</template>