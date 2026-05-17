<script setup lang="ts">
/**
 * SimulationOptimization.vue — 拓扑/形状/尺寸优化模块
 * 从 SimulationView.vue 提取 (~370 行 script + ~340 行 template)
 */
import { ref, computed, watch, nextTick } from 'vue'
import { useProjectStore } from '@/stores/project'
import ResultViewer from '@/components/simulation/ResultViewer.vue'
import ColorLegend from '@/components/simulation/ColorLegend.vue'

const projectStore = useProjectStore()

// ========== 优化参数状态 ==========
const optimizationType = ref<'topology' | 'shape' | 'size'>('topology')
const objectiveFunction = ref<'min_compliance' | 'min_mass' | 'max_stiffness'>('min_compliance')
const volumeConstraint = ref(0.5)
const stressConstraint = ref(250.0)
const maxIterations = ref(100)
const convergenceTolerance = ref(0.01)
const penalizationFactor = ref(3.0)
const minDensity = ref(0.01)
const designDomain = ref({ x_min: 0, x_max: 10, y_min: 0, y_max: 10, z_min: 0, z_max: 1 })

const optimizationRunning = ref(false)
const optimizationProgress = ref(0)
const currentIteration = ref(0)
const optimizationResult = ref<any>(null)
const currentDensityField = ref<number[]>([])
const iterationHistory = ref<any[]>([])
const objectiveHistory = ref<number[]>([])
const volumeHistory = ref<number[]>([])
const optimizationComplete = ref(false)

const sizeParameters = ref<{ name: string; lower_bound: number; upper_bound: number }[]>([])
const historyChartRef = ref<HTMLCanvasElement | null>(null)
const optimViewerRef = ref<any>(null)

const optimizationProgress_pct = computed(() => {
  return maxIterations.value > 0 ? (currentIteration.value / maxIterations.value) * 100 : 0
})

const estimatedCases = computed(() => {
  if (optimizationType.value === 'size') {
    return sizeParameters.value.length > 0
      ? Math.pow(10, sizeParameters.value.length)
      : 0
  }
  return maxIterations.value
})

// ========== 优化函数 ==========
function addSizeParameter() {
  sizeParameters.value.push({ name: '', lower_bound: 0.1, upper_bound: 10.0 })
}

function stopOptimization() {
  optimizationRunning.value = false
}

async function runOptimization() {
  if (!projectStore.hasMesh) return

  optimizationRunning.value = true
  optimizationComplete.value = false
  currentIteration.value = 0
  optimizationProgress.value = 0
  iterationHistory.value = []
  objectiveHistory.value = []
  volumeHistory.value = []
  currentDensityField.value = []

  try {
    const { invoke } = await import('@tauri-apps/api/core')

    const result = await invoke<any>('optimization_commands::run_topology_optimization', {
      config: {
        optimization_type: optimizationType.value,
        objective_function: objectiveFunction.value,
        volume_fraction: volumeConstraint.value,
        max_stress: stressConstraintValue.value,
        max_iterations: maxIterations.value,
        convergence_tolerance: convergenceTolerance.value,
        penalization_factor: penalizationFactor.value,
        min_density: minDensity.value,
        design_domain: designDomain.value,
        mesh_id: (projectStore.currentMesh as any)?.id
      }
    })

    if (result.density_field) {
      currentDensityField.value = result.density_field
    }
    optimizationResult.value = result
    optimizationComplete.value = true
    objectiveHistory.value = result.objective_history || []
    volumeHistory.value = result.volume_history || []
    currentIteration.value = maxIterations.value
    optimizationProgress.value = 100

    await nextTick()
    drawOptimizationHistory()
  } catch (e) {
    console.error('Optimization failed:', e)
    // 模拟优化过程（当后端未实现时）
    simulateOptimization()
  } finally {
    optimizationRunning.value = false
  }
}

function simulateOptimization() {
  // 前端模拟优化迭代（用于演示）
  const total = maxIterations.value
  let iteration = 0

  const interval = setInterval(() => {
    if (!optimizationRunning.value || iteration >= total) {
      clearInterval(interval)
      if (iteration >= total) {
        optimizationComplete.value = true
        drawOptimizationHistory()
      }
      return
    }

    iteration++
    currentIteration.value = iteration
    optimizationProgress.value = (iteration / total) * 100

    // 模拟数据
    const obj = 1000 * Math.exp(-iteration / 30) + 200 + Math.random() * 20
    const vol = volumeConstraint.value - (volumeConstraint.value - 0.3) * (iteration / total)

    objectiveHistory.value.push(obj)
    volumeHistory.value.push(vol)
    iterationHistory.value.push({
      iteration,
      objective_value: obj,
      volume_fraction: vol,
      converged: iteration > 10
    })

    // 模拟密度场
    if (iteration % 10 === 0) {
      const size = projectStore.currentMesh?.nodes?.length || 100
      currentDensityField.value = Array.from({ length: size }, (_, i) =>
        Math.random() > 0.4 ? Math.random() * 0.8 + 0.2 : Math.random() * 0.2
      )
    }

    drawOptimizationHistory()
  }, 100)
}

function drawOptimizationHistory() {
  if (!historyChartRef.value) return
  const ctx = historyChartRef.value.getContext('2d')
  if (!ctx) return

  const w = historyChartRef.value.width
  const h = historyChartRef.value.height

  ctx.clearRect(0, 0, w, h)

  if (objectiveHistory.value.length < 2) return

  const maxObj = Math.max(...objectiveHistory.value)
  const minObj = Math.min(...objectiveHistory.value)
  const objRange = maxObj - minObj || 1

  const padding = 20
  const chartW = w - padding * 2
  const chartH = h - padding * 2

  // 绘制目标函数曲线
  ctx.strokeStyle = '#ec4899'
  ctx.lineWidth = 2
  ctx.beginPath()

  objectiveHistory.value.forEach((obj, i) => {
    const x = padding + (i / (objectiveHistory.value.length - 1)) * chartW
    const y = padding + chartH - ((obj - minObj) / objRange) * chartH
    if (i === 0) ctx.moveTo(x, y)
    else ctx.lineTo(x, y)
  })
  ctx.stroke()

  // 绘制体积分数曲线
  ctx.strokeStyle = '#10b981'
  ctx.setLineDash([5, 5])
  ctx.beginPath()

  volumeHistory.value.forEach((vol, i) => {
    const x = padding + (i / (volumeHistory.value.length - 1)) * chartW
    const y = padding + chartH - (vol / 1) * chartH
    if (i === 0) ctx.moveTo(x, y)
    else ctx.lineTo(x, y)
  })
  ctx.stroke()
  ctx.setLineDash([])

  // 图例
  ctx.font = '10px sans-serif'
  ctx.fillStyle = '#ec4899'
  ctx.fillText('目标函数', padding, 12)
  ctx.fillStyle = '#10b981'
  ctx.fillText('体积分数', padding + 60, 12)
}

async function exportOptimizedGeometry() {
  if (!optimizationResult.value) return

  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('optimization_commands::export_topology_to_stl', {
      densityField: currentDensityField.value,
      threshold: 0.5,
      outputPath: ''
    })
  } catch (e) {
    console.error('Export failed:', e)
  }
}

async function exportOptimizationHistory() {
  const csv = [
    'iteration,objective_value,volume_fraction',
    ...iterationHistory.value.map((h, i) =>
      `${h.iteration},${h.objective_value?.toFixed(4)},${(h.volume_fraction * 100).toFixed(2)}%`
    )
  ].join('\n')

  const blob = new Blob([csv], { type: 'text/csv' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'optimization_history.csv'
  a.click()
  URL.revokeObjectURL(url)
}

// 监听画布尺寸变化
watch(historyChartRef, (canvas) => {
  if (canvas) {
    canvas.width = canvas.offsetWidth
    canvas.height = canvas.offsetHeight
  }
})

const stressConstraintValue = computed({
  get: () => stressConstraint.value,
  set: (v) => { stressConstraint.value = v }
})

defineExpose({
  runOptimization,
  stopOptimization,
  optimizationRunning,
  optimizationProgress,
  currentIteration,
  optimizationComplete,
  currentDensityField
})
</script>

<template>
  <!-- 优化设计左侧面板 -->
  <div class="w-72 bg-white border-r overflow-y-auto p-4 space-y-6">
    <!-- 1. 优化类型选择 -->
    <div class="space-y-3">
      <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
        <span class="w-5 h-5 rounded-full bg-pink-600 text-white text-xs flex items-center justify-center">1</span>
        优化类型
      </h3>

      <div class="space-y-2">
        <label class="flex items-center gap-2 cursor-pointer p-2 rounded hover:bg-pink-50">
          <input type="radio" v-model="optimizationType" value="topology" class="text-pink-600" />
          <span class="text-sm">🔬 拓扑优化 (Topology)</span>
        </label>
        <label class="flex items-center gap-2 cursor-pointer p-2 rounded hover:bg-pink-50">
          <input type="radio" v-model="optimizationType" value="shape" class="text-pink-600" />
          <span class="text-sm">📐 形状优化 (Shape)</span>
        </label>
        <label class="flex items-center gap-2 cursor-pointer p-2 rounded hover:bg-pink-50">
          <input type="radio" v-model="optimizationType" value="size" class="text-pink-600" />
          <span class="text-sm">📏 尺寸优化 (Size)</span>
        </label>
      </div>
    </div>

    <!-- 2. 目标函数设置 -->
    <div class="space-y-3">
      <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
        <span class="w-5 h-5 rounded-full bg-pink-600 text-white text-xs flex items-center justify-center">2</span>
        目标函数
      </h3>

      <select v-model="objectiveFunction" class="w-full px-3 py-2 border border-pink-300 rounded text-sm">
        <option value="min_compliance">最小柔度 (Min Compliance)</option>
        <option value="min_mass">最小质量 (Min Mass)</option>
        <option value="max_stiffness">最大刚度 (Max Stiffness)</option>
      </select>
    </div>

    <!-- 3. 约束条件 -->
    <div class="space-y-3">
      <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
        <span class="w-5 h-5 rounded-full bg-pink-600 text-white text-xs flex items-center justify-center">3</span>
        约束条件
      </h3>

      <div class="space-y-3">
        <div>
          <label class="text-xs text-gray-600 mb-1 block">体积分数上限</label>
          <div class="flex items-center gap-2">
            <input type="range" v-model.number="volumeConstraint" min="0.1" max="1.0" step="0.05" class="flex-1" />
            <span class="text-sm font-medium text-pink-600 w-12">{{ (volumeConstraint * 100).toFixed(0) }}%</span>
          </div>
        </div>

        <div>
          <label class="text-xs text-gray-600 mb-1 block">最大应力限制 (MPa)</label>
          <input type="number" v-model.number="stressConstraint" class="w-full px-2 py-1 border rounded text-sm" placeholder="250" />
        </div>
      </div>
    </div>

    <!-- 4. 算法参数 -->
    <div class="space-y-3">
      <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
        <span class="w-5 h-5 rounded-full bg-pink-600 text-white text-xs flex items-center justify-center">4</span>
        算法参数
      </h3>

      <div class="space-y-2">
        <div>
          <label class="text-xs text-gray-600 mb-1 block">最大迭代次数</label>
          <input type="number" v-model.number="maxIterations" min="10" max="500" class="w-full px-2 py-1 border rounded text-sm" />
        </div>

        <div>
          <label class="text-xs text-gray-600 mb-1 block">收敛精度</label>
          <input type="number" v-model.number="convergenceTolerance" step="0.001" min="0.0001" class="w-full px-2 py-1 border rounded text-sm" />
        </div>

        <!-- 拓扑优化专用参数 -->
        <div v-if="optimizationType === 'topology'" class="p-3 bg-pink-50 rounded-lg space-y-2">
          <div class="text-xs font-medium text-pink-700 mb-2">🎯 拓扑优化参数</div>
          <div>
            <label class="text-xs text-gray-600 mb-1 block">惩罚因子 (SIMP)</label>
            <input type="number" v-model.number="penalizationFactor" min="1.0" max="4.0" step="0.1" class="w-full px-2 py-1 text-xs border rounded" />
          </div>
          <div>
            <label class="text-xs text-gray-600 mb-1 block">最小密度</label>
            <input type="number" v-model.number="minDensity" min="0.001" max="0.3" step="0.01" class="w-full px-2 py-1 text-xs border rounded" />
          </div>
        </div>

        <!-- 尺寸优化专用参数 -->
        <div v-if="optimizationType === 'size'" class="p-3 bg-pink-50 rounded-lg space-y-2">
          <div class="text-xs font-medium text-pink-700 mb-2">📏 尺寸优化参数</div>
          <div v-for="(param, idx) in sizeParameters" :key="idx" class="border border-pink-200 rounded p-2">
            <div class="flex items-center justify-between mb-1">
              <input v-model="param.name" placeholder="参数名称" class="px-2 py-1 text-xs border rounded flex-1 mr-1" />
              <button @click="sizeParameters.splice(idx, 1)" class="text-red-500 text-xs">删除</button>
            </div>
            <div class="grid grid-cols-2 gap-1">
              <div>
                <label class="text-[10px] text-gray-500">下限</label>
                <input type="number" v-model.number="param.lower_bound" class="w-full px-1 py-1 text-xs border rounded" />
              </div>
              <div>
                <label class="text-[10px] text-gray-500">上限</label>
                <input type="number" v-model.number="param.upper_bound" class="w-full px-1 py-1 text-xs border rounded" />
              </div>
            </div>
          </div>
          <button @click="addSizeParameter" class="w-full px-2 py-1 text-xs border border-pink-300 text-pink-600 rounded hover:bg-pink-50">
            + 添加尺寸参数
          </button>
        </div>
      </div>
    </div>

    <!-- 5. 设计区域 -->
    <div class="space-y-3">
      <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
        <span class="w-5 h-5 rounded-full bg-pink-600 text-white text-xs flex items-center justify-center">5</span>
        设计区域
      </h3>

      <div class="grid grid-cols-2 gap-2">
        <div>
          <label class="text-xs text-gray-600 mb-1 block">X 范围</label>
          <div class="flex gap-1">
            <input type="number" v-model.number="designDomain.x_min" class="w-full px-1 py-1 text-xs border rounded" placeholder="Min" />
            <input type="number" v-model.number="designDomain.x_max" class="w-full px-1 py-1 text-xs border rounded" placeholder="Max" />
          </div>
        </div>
        <div>
          <label class="text-xs text-gray-600 mb-1 block">Y 范围</label>
          <div class="flex gap-1">
            <input type="number" v-model.number="designDomain.y_min" class="w-full px-1 py-1 text-xs border rounded" placeholder="Min" />
            <input type="number" v-model.number="designDomain.y_max" class="w-full px-1 py-1 text-xs border rounded" placeholder="Max" />
          </div>
        </div>
      </div>
    </div>

    <!-- 运行优化按钮 -->
    <div class="space-y-2">
      <button @click="runOptimization" :disabled="optimizationRunning || !projectStore.hasMesh" class="w-full px-3 py-2 bg-pink-600 text-white rounded text-sm hover:bg-pink-700 disabled:opacity-50 disabled:cursor-not-allowed">
        {{ optimizationRunning ? '优化中...' : '🚀 开始优化' }}
      </button>

      <button v-if="optimizationRunning" @click="stopOptimization" class="w-full px-3 py-2 bg-red-600 text-white rounded text-sm hover:bg-red-700">
        停止优化
      </button>
    </div>
  </div>
</template>