<template>
  <div class="transient-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">动力学瞬态分析</h2>
        <p class="text-xs text-[var(--text-muted)]">时域响应、冲击分析、振动仿真</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="showTutorialDialog = true" class="btn btn-ghost text-xs">
          <span class="mr-1">📚</span> 示例教程
        </button>
        <button @click="showEmbedDialog = true" class="btn btn-ghost text-xs">
          <span class="mr-1">🔗</span> 嵌入到笔记
        </button>
        <button @click="resetAll" class="btn btn-ghost text-xs">
          <span class="mr-1">⟳</span> 重置
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left: Setup Panel -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col overflow-y-auto">
        <!-- Step 1: Load Curves -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-3">1. 载荷曲线</h3>
          
          <!-- Load Type Selector -->
          <div class="mb-3">
            <label class="text-xs text-[var(--text-secondary)] block mb-2">载荷类型</label>
            <div class="flex gap-2 flex-wrap">
              <button 
                v-for="type in loadTypes" 
                :key="type.value"
                @click="setLoadType(type.value as 'step' | 'sinusoid' | 'impulse' | 'custom')"
                :class="['px-3 py-1.5 rounded text-xs transition', currentLoadType === type.value ? 'bg-[var(--accent)] text-white' : 'bg-[var(--bg-base)] text-[var(--text-secondary)]']"
              >
                {{ type.label }}
              </button>
            </div>
          </div>

          <!-- Template Buttons -->
          <div class="mb-3">
            <label class="text-xs text-[var(--text-secondary)] block mb-2">快捷模板</label>
            <div class="flex gap-2">
              <button @click="applyTemplate('step')" class="btn btn-outline text-xs px-2 py-1">阶跃</button>
              <button @click="applyTemplate('sinusoid')" class="btn btn-outline text-xs px-2 py-1">正弦</button>
              <button @click="applyTemplate('impulse')" class="btn btn-outline text-xs px-2 py-1">冲击</button>
            </div>
          </div>

          <!-- Load Parameters -->
          <div class="space-y-2">
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">幅值 (N)</label>
              <input v-model.number="loadConfig.amplitude" type="number" class="input w-full text-xs" />
            </div>
            <div v-if="currentLoadType === 'sinusoid'">
              <label class="text-xs text-[var(--text-secondary)] block mb-1">频率 (Hz)</label>
              <input v-model.number="loadConfig.frequency" type="number" step="0.1" class="input w-full text-xs" />
            </div>
            <div v-if="currentLoadType === 'sinusoid'">
              <label class="text-xs text-[var(--text-secondary)] block mb-1">相位 (rad)</label>
              <input v-model.number="loadConfig.phase" type="number" step="0.1" class="input w-full text-xs" />
            </div>
          </div>

          <!-- Custom Curve Editor -->
          <div class="mt-4">
            <div class="flex items-center justify-between mb-2">
              <label class="text-xs text-[var(--text-secondary)]">数据点</label>
              <button @click="showCurveEditor = true" class="text-xs text-[var(--accent)] hover:underline">
                ✏️ 编辑曲线
              </button>
            </div>
            <div class="text-xs text-[var(--text-muted)]">
              {{ loadConfig.points.length }} 个数据点
            </div>
          </div>
        </div>

        <!-- Step 2: Damping Settings -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-3">2. Rayleigh阻尼</h3>
          
          <div class="space-y-3">
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">质量阻尼系数 α</label>
              <input v-model.number="dampingConfig.alpha" type="number" step="0.01" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">刚度阻尼系数 β</label>
              <input v-model.number="dampingConfig.beta" type="number" step="0.0001" class="input w-full text-xs" />
            </div>
            <div class="flex gap-2">
              <div class="flex-1">
                <label class="text-xs text-[var(--text-secondary)] block mb-1">频率1 (Hz)</label>
                <input v-model.number="dampingConfig.frequency1" type="number" step="0.1" class="input w-full text-xs" />
              </div>
              <div class="flex-1">
                <label class="text-xs text-[var(--text-secondary)] block mb-1">频率2 (Hz)</label>
                <input v-model.number="dampingConfig.frequency2" type="number" step="0.1" class="input w-full text-xs" />
              </div>
            </div>
            <button @click="calculateDamping" class="btn btn-outline text-xs w-full">
              根据阻尼比计算
            </button>
          </div>
        </div>

        <!-- Step 3: Time Control -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-3">3. 时间步控制</h3>
          
          <div class="space-y-2">
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">初始时间步长 (s)</label>
              <input v-model.number="timeConfig.initial_dt" type="number" step="0.0001" class="input w-full text-xs" />
            </div>
            <div class="flex gap-2">
              <div class="flex-1">
                <label class="text-xs text-[var(--text-secondary)] block mb-1">最小步长 (s)</label>
                <input v-model.number="timeConfig.min_dt" type="number" step="0.000001" class="input w-full text-xs" />
              </div>
              <div class="flex-1">
                <label class="text-xs text-[var(--text-secondary)] block mb-1">最大步长 (s)</label>
                <input v-model.number="timeConfig.max_dt" type="number" step="0.001" class="input w-full text-xs" />
              </div>
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">总分析时间 (s)</label>
              <input v-model.number="timeConfig.total_time" type="number" step="0.1" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">输出间隔 (步)</label>
              <input v-model.number="timeConfig.output_interval" type="number" min="1" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Run Button -->
        <div class="px-4 py-3 mt-auto border-t border-[var(--border-subtle)]">
          <button 
            @click="runSimulation" 
            :disabled="running || !canRun"
            :class="['btn w-full text-sm', running ? 'btn-loading' : 'btn-primary']"
          >
            <span v-if="running" class="mr-2 animate-spin">⟳</span>
            {{ running ? '计算中...' : '▶ 运行瞬态分析' }}
          </button>
          <p v-if="!canRun" class="text-xs text-[var(--text-muted)] mt-2">
            ⚠️ 请先设置载荷参数
          </p>
        </div>
      </div>

      <!-- Center: 3D Viewport -->
      <div class="flex-1 flex flex-col">
        <!-- Canvas -->
        <div ref="canvasContainer" class="flex-1 bg-[var(--bg-base)] relative">
          <div v-if="!hasResult" class="absolute inset-0 flex items-center justify-center">
            <div class="text-center text-[var(--text-muted)]">
              <div class="text-4xl mb-2">📊</div>
              <p class="text-sm">运行分析后显示时域动画</p>
            </div>
          </div>
        </div>

        <!-- Animation Controls -->
        <div v-if="hasResult" class="h-16 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] px-4 flex items-center gap-4">
          <button @click="togglePlay" class="btn btn-ghost w-10 h-10 rounded-full flex items-center justify-center">
            {{ isPlaying ? '⏸' : '▶' }}
          </button>
          <div class="flex-1">
            <input 
              type="range" 
              v-model.number="currentFrame" 
              :min="0" 
              :max="animationFrames.length - 1"
              class="w-full"
              @input="onSliderChange"
            />
          </div>
          <span class="text-xs text-[var(--text-secondary)]">
            t = {{ currentTime.toFixed(4) }} s
          </span>
          <select v-model="displayMode" class="input text-xs w-32">
            <option value="displacement">位移</option>
            <option value="velocity">速度</option>
            <option value="acceleration">加速度</option>
            <option value="stress">应力</option>
          </select>
        </div>
      </div>

      <!-- Right: Results Panel -->
      <div v-if="hasResult" class="w-72 bg-[var(--bg-surface)] border-l border-[var(--border-subtle)] overflow-y-auto">
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <h3 class="text-sm font-semibold text-[var(--text-primary)]">分析结果</h3>
        </div>
        
        <!-- Summary Stats -->
        <div class="px-4 py-3 space-y-3">
          <div class="stat-card">
            <div class="text-[10px] text-[var(--text-muted)] uppercase">最大位移</div>
            <div class="text-xl font-bold text-[var(--accent)]">
              {{ results?.max_displacement_overall.toFixed(6) }} m
            </div>
          </div>
          <div class="stat-card">
            <div class="text-[10px] text-[var(--text-muted)] uppercase">最大应力</div>
            <div class="text-xl font-bold text-[var(--danger)]">
              {{ results?.max_stress_overall.toFixed(2) }} Pa
            </div>
          </div>
          <div class="stat-card">
            <div class="text-[10px] text-[var(--text-muted)] uppercase">固有频率</div>
            <div class="text-lg font-semibold text-[var(--text-primary)]">
              {{ results?.natural_frequencies[0]?.toFixed(2) || 'N/A' }} Hz
            </div>
          </div>
        </div>

        <!-- Time History Chart -->
        <div class="px-4 py-3 border-t border-[var(--border-subtle)]">
          <h4 class="text-xs font-semibold text-[var(--text-secondary)] mb-2">位移时程曲线</h4>
          <div ref="chartContainer" class="h-40 bg-[var(--bg-base)] rounded"></div>
        </div>

        <!-- Step List -->
        <div class="px-4 py-3 border-t border-[var(--border-subtle)]">
          <h4 class="text-xs font-semibold text-[var(--text-secondary)] mb-2">帧列表</h4>
          <div class="space-y-1 max-h-48 overflow-y-auto">
            <button 
              v-for="(step, idx) in results?.steps" 
              :key="idx"
              @click="jumpToFrame(idx)"
              :class="['w-full text-left px-2 py-1 rounded text-xs transition', currentFrame === idx ? 'bg-[var(--accent)] text-white' : 'hover:bg-[var(--bg-base)]']"
            >
              t = {{ step.time.toFixed(4) }}s | {{ step.max_displacement.toFixed(6) }}m
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Curve Editor Dialog -->
    <div v-if="showCurveEditor" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-[var(--bg-surface)] rounded-lg w-[600px] max-h-[80vh] flex flex-col">
        <div class="px-4 py-3 border-b border-[var(--border-subtle)] flex items-center justify-between">
          <h3 class="text-sm font-semibold">编辑载荷曲线</h3>
          <button @click="showCurveEditor = false" class="text-[var(--text-muted)] hover:text-[var(--text-primary)]">×</button>
        </div>
        <div class="p-4 flex-1 overflow-y-auto">
          <!-- Canvas for drawing -->
          <div class="mb-4">
            <canvas 
              ref="curveCanvas" 
              class="w-full h-48 bg-[var(--bg-base)] rounded border border-[var(--border-subtle)] cursor-crosshair"
              @mousedown="startDrawing"
              @mousemove="continueDrawing"
              @mouseup="stopDrawing"
            ></canvas>
          </div>
          <!-- Data Points Table -->
          <div class="mb-4">
            <div class="flex items-center justify-between mb-2">
              <label class="text-xs font-semibold text-[var(--text-secondary)]">数据点表格</label>
              <button @click="addPoint" class="text-xs text-[var(--accent)] hover:underline">+ 添加点</button>
            </div>
            <table class="w-full text-xs">
              <thead>
                <tr class="border-b border-[var(--border-subtle)]">
                  <th class="text-left py-1 text-[var(--text-muted)]">时间 (s)</th>
                  <th class="text-left py-1 text-[var(--text-muted)]">幅值</th>
                  <th class="w-8"></th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(point, idx) in editablePoints" :key="idx">
                  <td class="py-1">
                    <input v-model.number="point.time" type="number" step="0.001" class="input w-full text-xs" />
                  </td>
                  <td class="py-1 px-1">
                    <input v-model.number="point.value" type="number" class="input w-full text-xs" />
                  </td>
                  <td>
                    <button @click="removePoint(idx)" class="text-[var(--text-muted)] hover:text-[var(--danger)]">×</button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
        <div class="px-4 py-3 border-t border-[var(--border-subtle)] flex justify-end gap-2">
          <button @click="showCurveEditor = false" class="btn btn-ghost text-xs">取消</button>
          <button @click="applyCurve" class="btn btn-primary text-xs">应用</button>
        </div>
      </div>
    </div>

    <!-- Tutorial Dialog -->
    <div v-if="showTutorialDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-[var(--bg-surface)] rounded-lg w-[500px] max-h-[80vh] flex flex-col">
        <div class="px-4 py-3 border-b border-[var(--border-subtle)] flex items-center justify-between">
          <h3 class="text-sm font-semibold">示例教程</h3>
          <button @click="showTutorialDialog = false" class="text-[var(--text-muted)] hover:text-[var(--text-primary)]">×</button>
        </div>
        <div class="p-4 flex-1 overflow-y-auto space-y-4">
          <div 
            v-for="example in tutorialExamples" 
            :key="example.id"
            @click="loadTutorial(example)"
            class="p-3 rounded-lg bg-[var(--bg-base)] hover:bg-[var(--bg-hover)] cursor-pointer transition"
          >
            <h4 class="text-sm font-semibold text-[var(--text-primary)]">{{ example.name }}</h4>
            <p class="text-xs text-[var(--text-muted)] mt-1">{{ example.description }}</p>
          </div>
        </div>
        <div class="px-4 py-3 border-t border-[var(--border-subtle)] flex justify-end">
          <button @click="showTutorialDialog = false" class="btn btn-ghost text-xs">关闭</button>
        </div>
      </div>
    </div>

    <!-- Embed Dialog -->
    <div v-if="showEmbedDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-[var(--bg-surface)] rounded-lg w-80">
        <div class="px-4 py-3 border-b border-[var(--border-subtle)] flex items-center justify-between">
          <h3 class="text-sm font-semibold">嵌入到笔记</h3>
          <button @click="showEmbedDialog = false" class="text-[var(--text-muted)] hover:text-[var(--text-primary)]">×</button>
        </div>
        <div class="p-4">
          <p class="text-xs text-[var(--text-muted)] mb-3">选择要嵌入的目标笔记</p>
          <select v-model="targetNoteId" class="input w-full text-xs">
            <option value="">选择笔记...</option>
            <option v-for="note in (projectStore.currentProject ? [{id: projectStore.currentProject.id, title: projectStore.currentProject.name}] : [])" :key="note.id" :value="note.id">
              {{ note.title || '无标题笔记' }}
            </option>
          </select>
        </div>
        <div class="px-4 py-3 border-t border-[var(--border-subtle)] flex justify-end gap-2">
          <button @click="showEmbedDialog = false" class="btn btn-ghost text-xs">取消</button>
          <button @click="embedToNote" :disabled="!targetNoteId" class="btn btn-primary text-xs">嵌入</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { useProjectStore } from '@/stores/project'
import {
  getTutorialExamples,
  runTransientSimulation,
  getLoadCurveTemplate,
  type LoadPoint,
  type RayleighDamping,
  type TimeStepControl,
  type TransientAnalysisConfig,
  type TransientResults,
  type TutorialExample
} from '@/api/transient_dynamics'

const projectStore = useProjectStore()

// Refs
const canvasContainer = ref<HTMLElement | null>(null)
const curveCanvas = ref<HTMLCanvasElement | null>(null)
const chartContainer = ref<HTMLElement | null>(null)

// State
const showCurveEditor = ref(false)
const showTutorialDialog = ref(false)
const showEmbedDialog = ref(false)
const running = ref(false)
const isPlaying = ref(false)
const currentFrame = ref(0)
const currentTime = ref(0)
const displayMode = ref('displacement')
const targetNoteId = ref('')
const tutorialExamples = ref<TutorialExample[]>([])

// 切换载荷类型
function setLoadType(type: 'step' | 'sinusoid' | 'impulse' | 'custom') {
  currentLoadType.value = type
}

// Load config
const loadTypes = [
  { label: '阶跃', value: 'step' },
  { label: '正弦', value: 'sinusoid' },
  { label: '冲击', value: 'impulse' },
  { label: '自定义', value: 'custom' }
]
const currentLoadType = ref<'step' | 'sinusoid' | 'impulse' | 'custom'>('step')
const loadConfig = ref({
  amplitude: 100.0,
  frequency: 10.0,
  phase: 0.0,
  points: [] as LoadPoint[]
})

// Damping config
const dampingConfig = ref<RayleighDamping>({
  alpha: 0.1,
  beta: 0.0001,
  frequency1: 1.0,
  frequency2: 10.0
})

// Time control
const timeConfig = ref<TimeStepControl>({
  initial_dt: 0.001,
  min_dt: 1e-6,
  max_dt: 0.01,
  total_time: 1.0,
  output_interval: 100
})

// Results
const results = ref<TransientResults | null>(null)
const animationFrames = computed(() => results.value?.steps || [])
const hasResult = computed(() => projectStore.hasTransientResult)

// Editable points for curve editor
const editablePoints = ref<LoadPoint[]>([
  { time: 0.0, value: 0.0 },
  { time: 0.1, value: 100.0 },
  { time: 0.2, value: 100.0 },
  { time: 0.3, value: 0.0 }
])

// Drawing state
let isDrawing = false

// Computed
const canRun = computed(() => loadConfig.value.amplitude > 0)

// Methods
function resetAll() {
  loadConfig.value = {
    amplitude: 100.0,
    frequency: 10.0,
    phase: 0.0,
    points: []
  }
  dampingConfig.value = {
    alpha: 0.1,
    beta: 0.0001,
    frequency1: 1.0,
    frequency2: 10.0
  }
  timeConfig.value = {
    initial_dt: 0.001,
    min_dt: 1e-6,
    max_dt: 0.01,
    total_time: 1.0,
    output_interval: 100
  }
  results.value = null
  projectStore.clearTransientResult()
  currentFrame.value = 0
  isPlaying.value = false
}

async function applyTemplate(type: 'step' | 'sinusoid' | 'impulse') {
  try {
    const template = await getLoadCurveTemplate(type)
    loadConfig.value.amplitude = template.amplitude
    loadConfig.value.frequency = template.frequency
    loadConfig.value.phase = template.phase
    loadConfig.value.points = template.points
    currentLoadType.value = type
    editablePoints.value = [...template.points]
  } catch (e) {
    console.error('Failed to load template:', e)
  }
}

async function loadTutorials() {
  try {
    tutorialExamples.value = await getTutorialExamples()
  } catch (e) {
    console.error('Failed to load tutorials:', e)
  }
}

function loadTutorial(example: TutorialExample) {
  loadConfig.value = {
    amplitude: example.config.load_curves[0]?.amplitude || 100,
    frequency: example.config.load_curves[0]?.frequency || 10,
    phase: example.config.load_curves[0]?.phase || 0,
    points: example.config.load_curves[0]?.points || []
  }
  dampingConfig.value = { ...example.config.damping }
  timeConfig.value = { ...example.config.time_control }
  currentLoadType.value = example.config.load_curves[0]?.load_type || 'step'
  showTutorialDialog.value = false
}

function calculateDamping() {
  // Simplified damping calculation
  const freq1 = dampingConfig.value.frequency1
  const freq2 = dampingConfig.value.frequency2
  const dampingRatio = 0.05 // 5% critical damping
  
  const omega1 = 2 * Math.PI * freq1
  const omega2 = 2 * Math.PI * freq2
  
  // ζ = α/(2ω) + βω/2
  // Solve for α and β
  dampingConfig.value.alpha = dampingRatio * 2 * omega1 * omega2 / (omega1 + omega2)
  dampingConfig.value.beta = dampingRatio * 2 / (omega1 + omega2)
}

async function runSimulation() {
  running.value = true
  try {
    const config: TransientAnalysisConfig = {
      name: '瞬态分析',
      mesh_id: projectStore.currentMesh ? 'current_mesh' : 'default',
      load_curves: [{
        id: 'main_load',
        name: '主载荷',
        load_type: currentLoadType.value,
        points: loadConfig.value.points,
        amplitude: loadConfig.value.amplitude,
        frequency: loadConfig.value.frequency,
        phase: loadConfig.value.phase,
        duration: timeConfig.value.total_time
      }],
      damping: dampingConfig.value,
      time_control: timeConfig.value,
      initial_conditions: [0, 0]
    }
    
    const result = await runTransientSimulation(config)
    results.value = result
    projectStore.setTransientResult(result)
    currentFrame.value = 0
    
    await nextTick()
    drawChart()
    drawAnimationFrame()
  } catch (e) {
    console.error('Simulation failed:', e)
  } finally {
    running.value = false
  }
}

function togglePlay() {
  isPlaying.value = !isPlaying.value
  if (isPlaying.value) {
    playAnimation()
  }
}

let animationTimer: number | null = null

function playAnimation() {
  if (!isPlaying.value || !results.value) return
  
  const interval = 50 // 50ms per frame
  animationTimer = window.setInterval(() => {
    if (currentFrame.value < animationFrames.value.length - 1) {
      currentFrame.value++
      currentTime.value = results.value!.steps[currentFrame.value].time
      drawAnimationFrame()
    } else {
      isPlaying.value = false
      if (animationTimer) {
        clearInterval(animationTimer)
        animationTimer = null
      }
    }
  }, interval)
}

function onSliderChange() {
  if (results.value) {
    currentTime.value = results.value.steps[currentFrame.value].time
    drawAnimationFrame()
  }
}

function jumpToFrame(idx: number) {
  currentFrame.value = idx
  if (results.value) {
    currentTime.value = results.value.steps[idx].time
    drawAnimationFrame()
  }
}

function drawChart() {
  if (!chartContainer.value || !results.value) return
  
  const canvas = chartContainer.value.querySelector('canvas') || document.createElement('canvas')
  canvas.width = chartContainer.value.clientWidth
  canvas.height = chartContainer.value.clientHeight
  
  if (!chartContainer.value.contains(canvas)) {
    chartContainer.value.appendChild(canvas)
  }
  
  const ctx = canvas.getContext('2d')
  if (!ctx) return
  
  const width = canvas.width
  const height = canvas.height
  const padding = 30
  
  ctx.clearRect(0, 0, width, height)
  
  // Draw axes
  ctx.strokeStyle = '#374151'
  ctx.lineWidth = 1
  ctx.beginPath()
  ctx.moveTo(padding, padding)
  ctx.lineTo(padding, height - padding)
  ctx.lineTo(width - padding, height - padding)
  ctx.stroke()
  
  // Draw data
  const steps = results.value.steps
  const maxTime = results.value.total_time
  const maxDisp = results.value.max_displacement_overall
  
  ctx.strokeStyle = '#4F8CFF'
  ctx.lineWidth = 2
  ctx.beginPath()
  
  steps.forEach((step, i) => {
    const x = padding + (step.time / maxTime) * (width - 2 * padding)
    const y = height - padding - (step.max_displacement / maxDisp) * (height - 2 * padding)
    
    if (i === 0) ctx.moveTo(x, y)
    else ctx.lineTo(x, y)
  })
  
  ctx.stroke()
  
  // Labels
  ctx.fillStyle = '#9CA3AF'
  ctx.font = '10px sans-serif'
  ctx.fillText('时间 (s)', width / 2, height - 5)
  ctx.save()
  ctx.translate(10, height / 2)
  ctx.rotate(-Math.PI / 2)
  ctx.fillText('位移 (m)', 0, 0)
  ctx.restore()
}

function drawAnimationFrame() {
  // Simple visualization placeholder
  // In production, would render 3D model with displacement
  if (!canvasContainer.value) return
  
  const existingCanvas = canvasContainer.value.querySelector('canvas')
  if (!existingCanvas) {
    const canvas = document.createElement('canvas')
    canvas.style.width = '100%'
    canvas.style.height = '100%'
    canvasContainer.value.appendChild(canvas)
  }
}

function startDrawing(e: MouseEvent) {
  isDrawing = true
  const rect = curveCanvas.value?.getBoundingClientRect()
  if (rect) {
    const x = e.clientX - rect.left
    const y = e.clientY - rect.top
    // Add point based on click position
    const time = (x / rect.width) * timeConfig.value.total_time
    const value = (1 - y / rect.height) * loadConfig.value.amplitude
    editablePoints.value.push({ time, value })
    editablePoints.value.sort((a, b) => a.time - b.time)
  }
}

function continueDrawing(e: MouseEvent) {
  if (!isDrawing || !curveCanvas.value) return
  // Real-time drawing would update preview
}

function stopDrawing() {
  isDrawing = false
}

function addPoint() {
  const lastPoint = editablePoints.value[editablePoints.value.length - 1]
  const newTime = lastPoint ? lastPoint.time + 0.1 : 0
  editablePoints.value.push({ time: newTime, value: 0 })
}

function removePoint(idx: number) {
  editablePoints.value.splice(idx, 1)
}

function applyCurve() {
  loadConfig.value.points = [...editablePoints.value]
  currentLoadType.value = 'custom'
  showCurveEditor.value = false
}

function embedToNote() {
  if (!targetNoteId.value) return
  projectStore.addEmbedRecord({
    type: 'simulation',
    targetId: 'transient_analysis',
    targetName: '动力学瞬态分析结果',
    noteId: targetNoteId.value
  })
  showEmbedDialog.value = false
  targetNoteId.value = ''
}

// Watch for result changes
watch(() => projectStore.lastTransientResult, (newResult) => {
  if (newResult) {
    results.value = newResult
  }
})

// Lifecycle
onMounted(() => {
  loadTutorials()
})

// Cleanup
import { onUnmounted } from 'vue'
onUnmounted(() => {
  if (animationTimer) {
    clearInterval(animationTimer)
  }
})
</script>

<style scoped>
.transient-view {
  --bg-base: #0A0B0E;
  --bg-surface: #14151A;
  --bg-hover: #1C1D24;
  --text-primary: #E8E9EB;
  --text-secondary: #9CA3AF;
  --text-muted: #6B7280;
  --accent: #4F8CFF;
  --danger: #EF4444;
  --border-subtle: #2D2E38;
}

.btn {
  @apply px-3 py-1.5 rounded text-xs font-medium transition;
}

.btn-primary {
  @apply bg-blue-600 text-white hover:bg-blue-700;
}

.btn-ghost {
  @apply text-gray-400 hover:text-white hover:bg-gray-700;
}

.btn-outline {
  @apply border border-gray-600 text-gray-300 hover:bg-gray-700;
}

.btn-loading {
  @apply bg-blue-800 text-white cursor-not-allowed;
}

.input {
  @apply px-2 py-1.5 bg-[var(--bg-base)] border border-[var(--border-subtle)] rounded text-xs text-[var(--text-primary)] focus:outline-none focus:border-[var(--accent)];
}

.stat-card {
  @apply p-3 rounded-lg bg-[var(--bg-base)];
}
</style>