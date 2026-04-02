<!--
  显式动力学分析前端界面 - ExplicitDynamicsView.vue
  功能：高速冲击、碰撞、爆炸等显式动力学分析
  V1.3-004: 新增实际求解器、能量守恒曲线、变形动画
-->
<template>
  <div class="h-full flex flex-col bg-[var(--bg-primary)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-lg font-semibold text-[var(--text-primary)]">显式动力学分析</h2>
        <p class="text-sm text-[var(--text-secondary)]">高速冲击、碰撞、爆炸、大变形 | V1.3-004 求解器</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="showTemplatesDialog = true" class="px-3 py-1.5 text-sm border border-blue-300 text-blue-400 rounded hover:bg-blue-900/20 transition">模板</button>
        <button @click="showEmbedDialog = true" class="px-3 py-1.5 text-sm border border-purple-300 text-purple-400 rounded hover:bg-purple-900/20 transition">嵌入笔记</button>
      </div>
    </div>

    <div class="flex-1 flex overflow-hidden">
      <!-- 左侧面板 -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] overflow-y-auto">
        <!-- 1. 几何建模 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('geometry')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-hover)]">
            <span class="font-medium text-[var(--text-primary)]">1. 几何建模</span>
            <span class="text-[var(--text-muted)]">{{ sections.geometry ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.geometry" class="px-4 pb-4 space-y-3">
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">网格尺寸 (N x N x N)</label>
              <div class="grid grid-cols-3 gap-2">
                <div>
                  <span class="text-[10px] text-[var(--text-muted)]">Nx</span>
                  <input v-model.number="meshNx" type="number" min="2" max="20" class="input w-full text-xs" />
                </div>
                <div>
                  <span class="text-[10px] text-[var(--text-muted)]">Ny</span>
                  <input v-model.number="meshNy" type="number" min="2" max="20" class="input w-full text-xs" />
                </div>
                <div>
                  <span class="text-[10px] text-[var(--text-muted)]">Nz</span>
                  <input v-model.number="meshNz" type="number" min="2" max="20" class="input w-full text-xs" />
                </div>
              </div>
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">模型尺寸 (mm)</label>
              <input v-model.number="meshSize" type="number" step="1" class="input w-full text-xs" placeholder="10" />
            </div>
          </div>
        </div>

        <!-- 2. 材料参数 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('materials')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-hover)]">
            <span class="font-medium text-[var(--text-primary)]">2. 材料参数</span>
            <span class="text-[var(--text-muted)]">{{ sections.materials ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.materials" class="px-4 pb-4 space-y-3">
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">材料预设</label>
              <select v-model="materialPreset" @change="applyMaterialPreset" class="input w-full text-xs">
                <option value="steel">钢 (E=210GPa)</option>
                <option value="aluminum">铝 (E=70GPa)</option>
                <option value="titanium">钛 (E=116GPa)</option>
                <option value="rubber">橡胶 (E=0.01GPa)</option>
              </select>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">弹性模量 (MPa)</label>
                <input v-model.number="solverMaterial.youngs_modulus" type="number" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">泊松比</label>
                <input v-model.number="solverMaterial.poisson_ratio" type="number" step="0.01" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">密度 (kg/m3)</label>
                <input v-model.number="solverMaterial.density" type="number" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">屈服应力 (MPa)</label>
                <input v-model.number="solverMaterial.yield_stress" type="number" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">硬化模量 (MPa)</label>
                <input v-model.number="solverMaterial.hardening_modulus" type="number" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">失效应变</label>
                <input v-model.number="solverMaterial.failure_strain" type="number" step="0.01" class="input w-full text-xs" />
              </div>
            </div>
          </div>
        </div>

        <!-- 3. 初始条件 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('initial')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-hover)]">
            <span class="font-medium text-[var(--text-primary)]">3. 初始条件</span>
            <span class="text-[var(--text-muted)]">{{ sections.initial ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.initial" class="px-4 pb-4 space-y-3">
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">初始速度 Z (m/s)</label>
              <input v-model.number="initialVelocityZ" type="number" step="1" class="input w-full text-xs" placeholder="-10" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">阻尼系数</label>
              <input v-model.number="solverDamping" type="number" step="0.1" class="input w-full text-xs" placeholder="0.0" />
            </div>
          </div>
        </div>

        <!-- 4. 求解控制 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('solve')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-hover)]">
            <span class="font-medium text-[var(--text-primary)]">4. 求解控制</span>
            <span class="text-[var(--text-muted)]">{{ sections.solve ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.solve" class="px-4 pb-4 space-y-3">
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">时间步长 (s)</label>
                <input v-model.number="solverTimeStep" type="number" step="1e-7" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">终止时间 (s)</label>
                <input v-model.number="solverEndTime" type="number" step="1e-4" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">输出间隔 (步)</label>
                <input v-model.number="solverOutputInterval" type="number" min="1" class="input w-full text-xs" />
              </div>
            </div>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="px-4 py-3 space-y-2">
          <button @click="generateDemoMesh" :disabled="generating" class="w-full btn btn-primary text-sm">
            {{ generating ? '生成中...' : '生成网格' }}
          </button>
          <button @click="runSolver" :disabled="!modelReady || running" class="w-full btn btn-accent text-sm">
            {{ running ? '求解中...' : '运行求解器' }}
          </button>
          <button v-if="solverResult" @click="playSolverAnimation" :disabled="playing" class="w-full btn btn-ghost text-sm">
            {{ playing ? '播放中...' : '播放变形动画' }}
          </button>
        </div>

        <!-- 求解结果摘要 -->
        <div v-if="solverResult" class="px-4 pb-4 space-y-2 border-t border-[var(--border-subtle)]">
          <h4 class="text-xs font-semibold text-[var(--text-primary)] mt-3">求解结果</h4>
          <div class="text-xs text-[var(--text-secondary)] space-y-1">
            <div>时间步数: {{ solverResult.num_time_steps }}</div>
            <div>输出帧数: {{ solverResult.frames.length }}</div>
            <div>能量误差: <span :class="solverResult.energy_error_percent < 1 ? 'text-green-400' : 'text-red-400'">{{ solverResult.energy_error_percent.toFixed(3) }}%</span></div>
            <div>失效单元: {{ solverResult.num_failed_elements }}</div>
            <div v-if="solverResult.frames.length > 0">
              <div>最大位移: {{ solverResult.frames[solverResult.frames.length-1].max_displacement.toExponential(3) }}</div>
              <div>最大速度: {{ solverResult.frames[solverResult.frames.length-1].max_velocity.toExponential(3) }}</div>
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧视图区域 -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- 上部：可视化区域 -->
        <div class="flex-1 relative bg-[var(--bg-base)]">
          <!-- 无结果时的占位 -->
          <div v-if="!solverResult && !playing" class="absolute inset-0 flex items-center justify-center">
            <div class="text-center">
              <div class="w-48 h-32 mx-auto mb-4 rounded-xl bg-[var(--bg-surface)] flex items-center justify-center border-2 border-dashed border-[var(--border-subtle)]">
                <svg width="64" height="64" viewBox="0 0 64 64" class="opacity-30">
                  <rect x="8" y="8" width="48" height="48" rx="4" fill="none" stroke="currentColor" stroke-width="1.5"/>
                  <line x1="8" y1="24" x2="56" y2="24" stroke="currentColor" stroke-width="0.5" stroke-dasharray="3"/>
                  <line x1="8" y1="40" x2="56" y2="40" stroke="currentColor" stroke-width="0.5" stroke-dasharray="3"/>
                  <line x1="24" y1="8" x2="24" y2="56" stroke="currentColor" stroke-width="0.5" stroke-dasharray="3"/>
                  <line x1="40" y1="8" x2="40" y2="56" stroke="currentColor" stroke-width="0.5" stroke-dasharray="3"/>
                  <polygon points="32,16 38,28 26,28" fill="currentColor" opacity="0.5"/>
                </svg>
              </div>
              <h3 class="text-base font-semibold text-[var(--text-primary)] mb-1">显式动力学求解器</h3>
              <p class="text-sm text-[var(--text-muted)]">中心差分时间积分 | HEX8单元</p>
              <p class="text-xs text-[var(--text-muted)] mt-2">生成网格后点击"运行求解器"</p>
            </div>
          </div>

          <!-- 变形动画 SVG -->
          <div v-if="(playing || showFrame) && currentFrame" class="absolute inset-0 flex items-center justify-center p-4">
            <svg :viewBox="`0 0 ${svgWidth} ${svgHeight}`" class="w-full h-full max-w-2xl">
              <!-- 网格变形显示 -->
              <g v-for="(elem, eidx) in displayElements" :key="eidx">
                <polygon
                  :points="getElementPolygon(elem, currentFrame)"
                  :fill="getElementColor(eidx, currentFrame)"
                  :stroke="isElementFailed(eidx, currentFrame) ? '#ff4444' : 'rgba(255,255,255,0.15)'"
                  :stroke-width="isElementFailed(eidx, currentFrame) ? 2 : 0.5"
                  :opacity="isElementFailed(eidx, currentFrame) ? 0.4 : 0.8"
                />
              </g>
              <!-- 时间标签 -->
              <text :x="svgWidth - 10" :y="20" text-anchor="end" fill="var(--text-primary)" font-size="12">
                t = {{ currentFrame.time.toExponential(3) }} s
              </text>
              <text :x="svgWidth - 10" :y="36" text-anchor="end" fill="var(--text-secondary)" font-size="10">
                KE={{ currentFrame.kinetic_energy.toExponential(2) }} IE={{ currentFrame.internal_energy.toExponential(2) }}
              </text>
            </svg>
          </div>

          <!-- 动画控制栏 -->
          <div v-if="solverResult && solverResult.frames.length > 1" class="absolute bottom-12 left-4 right-4 flex items-center gap-3">
            <button @click="playing ? pauseSolverAnimation() : playSolverAnimation()" class="btn btn-primary text-xs px-3 py-1">
              {{ playing ? '⏸' : '▶' }}
            </button>
            <input type="range" :min="0" :max="solverResult.frames.length - 1" v-model.number="currentFrameIndex"
              class="flex-1 h-1 accent-blue-500" @input="onFrameSliderChange" />
            <span class="text-xs text-[var(--text-muted)] w-16 text-right">
              {{ currentFrameIndex }}/{{ solverResult.frames.length - 1 }}
            </span>
          </div>
        </div>

        <!-- 下部：能量守恒曲线 -->
        <div v-if="solverResult && solverResult.frames.length > 1" class="h-48 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] p-3">
          <div class="flex items-center justify-between mb-1">
            <h4 class="text-xs font-semibold text-[var(--text-primary)]">能量守恒曲线</h4>
            <div class="flex items-center gap-3 text-[10px]">
              <span class="flex items-center gap-1"><span class="w-3 h-0.5 bg-blue-400 inline-block"></span>动能</span>
              <span class="flex items-center gap-1"><span class="w-3 h-0.5 bg-red-400 inline-block"></span>内能</span>
              <span class="flex items-center gap-1"><span class="w-3 h-0.5 bg-green-400 inline-block"></span>总能量</span>
            </div>
          </div>
          <svg :viewBox="`0 0 ${energyChartWidth} ${energyChartHeight}`" class="w-full" preserveAspectRatio="none">
            <!-- 坐标轴 -->
            <line x1="40" y1="0" x2="40" :y2="energyChartHeight - 20" stroke="rgba(255,255,255,0.2)" stroke-width="0.5"/>
            <line x1="40" :y1="energyChartHeight - 20" :x2="energyChartWidth" :y2="energyChartHeight - 20" stroke="rgba(255,255,255,0.2)" stroke-width="0.5"/>
            <!-- Y轴标签 -->
            <text x="35" y="12" text-anchor="end" fill="var(--text-muted)" font-size="8">{{ maxEnergyLabel }}</text>
            <text x="35" :y="energyChartHeight - 22" text-anchor="end" fill="var(--text-muted)" font-size="8">0</text>
            <!-- 动能曲线 -->
            <polyline :points="energyCurvePoints('kinetic')" fill="none" stroke="#60a5fa" stroke-width="1.5"/>
            <!-- 内能曲线 -->
            <polyline :points="energyCurvePoints('internal')" fill="none" stroke="#f87171" stroke-width="1.5"/>
            <!-- 总能量曲线 -->
            <polyline :points="energyCurvePoints('total')" fill="none" stroke="#4ade80" stroke-width="1.5"/>
            <!-- 当前帧指示线 -->
            <line v-if="currentFrame != null" :x1="frameToX(currentFrameIndex)" y1="0"
              :x2="frameToX(currentFrameIndex)" :y2="energyChartHeight - 20"
              stroke="rgba(255,255,255,0.3)" stroke-width="0.5" stroke-dasharray="3"/>
          </svg>
        </div>
      </div>
    </div>

    <!-- 模板对话框 -->
    <div v-if="showTemplatesDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showTemplatesDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-xl w-[500px] max-h-[80vh] overflow-hidden">
        <div class="px-4 py-3 border-b border-[var(--border-subtle)] flex justify-between items-center">
          <h3 class="font-semibold text-[var(--text-primary)]">分析模板</h3>
          <button @click="showTemplatesDialog = false" class="text-[var(--text-muted)] hover:text-[var(--text-primary)]">X</button>
        </div>
        <div class="p-4 space-y-2 overflow-y-auto max-h-[60vh]">
          <div v-for="tmpl in templates" :key="tmpl.id" @click="loadTemplate(tmpl)" class="p-3 border border-[var(--border-subtle)] rounded-lg hover:bg-[var(--bg-hover)] cursor-pointer">
            <div class="font-medium text-sm text-[var(--text-primary)]">{{ tmpl.name }}</div>
            <div class="text-xs text-[var(--text-muted)] mt-1">{{ tmpl.desc }}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 嵌入笔记对话框 -->
    <div v-if="showEmbedDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showEmbedDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-xl w-[400px] p-4">
        <h3 class="font-semibold text-[var(--text-primary)] mb-4">嵌入到笔记</h3>
        <div class="mb-4">
          <label class="text-sm text-[var(--text-secondary)] mb-2 block">选择要嵌入的笔记</label>
          <select v-model="selectedNoteId" class="input w-full text-sm">
            <option value="">-- 选择笔记 --</option>
            <option v-for="note in notes" :key="note.id" :value="note.id">{{ note.title }}</option>
          </select>
        </div>
        <div class="flex justify-end gap-2">
          <button @click="showEmbedDialog = false" class="btn btn-ghost text-sm">取消</button>
          <button @click="embedToNote" :disabled="!selectedNoteId" class="btn btn-primary text-sm">确认嵌入</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref, reactive, computed, onMounted, onUnmounted } from 'vue'
import type { ExplicitSolverResult, ExplicitFrame } from '../api/explicit_dynamics'

// ========== 状态 ==========
const modelReady = ref(false)
const running = ref(false)
const generating = ref(false)
const playing = ref(false)
const showFrame = ref(false)

// 网格参数
const meshNx = ref(4)
const meshNy = ref(4)
const meshNz = ref(4)
const meshSize = ref(10.0)

// 材料预设
const materialPreset = ref('steel')
const solverMaterial = reactive({
  density: 7850.0,
  youngs_modulus: 210000.0,
  poisson_ratio: 0.3,
  yield_stress: 250.0,
  hardening_modulus: 1000.0,
  failure_strain: 0.2,
})

// 求解参数
const initialVelocityZ = ref(-10.0)
const solverDamping = ref(0.0)
const solverTimeStep = ref(1e-5)
const solverEndTime = ref(0.01)
const solverOutputInterval = ref(10)

// 网格数据
const demoMesh = ref<any>(null)

// 求解结果
const solverResult = ref<ExplicitSolverResult | null>(null)
const currentFrameIndex = ref(0)
const animationTimer = ref<number | null>(null)

// UI
const sections = reactive({ geometry: true, materials: true, initial: true, solve: true })
const showTemplatesDialog = ref(false)
const showEmbedDialog = ref(false)
const selectedNoteId = ref('')
const notes = ref([{ id: '1', title: '冲击分析笔记' }])
const templates = ref<any[]>([])

// SVG 尺寸
const svgWidth = 500
const svgHeight = 400
const energyChartWidth = 600
const energyChartHeight = 140

// ========== 计算属性 ==========
const currentFrame = computed<ExplicitFrame | null>(() => {
  if (!solverResult.value) return null
  return solverResult.value.frames[currentFrameIndex.value] || null
})

const displayElements = computed(() => {
  return demoMesh.value?.elements || []
})

const maxEnergy = computed(() => {
  if (!solverResult.value || solverResult.value.frames.length === 0) return 1
  let maxE = 0
  for (const f of solverResult.value.frames) {
    maxE = Math.max(maxE, f.kinetic_energy, f.internal_energy, f.total_energy)
  }
  return maxE || 1
})

const maxEnergyLabel = computed(() => {
  const e = maxEnergy.value
  if (e === 0) return '0'
  return e.toExponential(1)
})

// ========== 方法 ==========
function toggleSection(section: string) {
  sections[section as keyof typeof sections] = !sections[section as keyof typeof sections]
}

function applyMaterialPreset() {
  const presets: Record<string, any> = {
    steel: { density: 7850, youngs_modulus: 210000, poisson_ratio: 0.3, yield_stress: 250, hardening_modulus: 1000, failure_strain: 0.2 },
    aluminum: { density: 2700, youngs_modulus: 70000, poisson_ratio: 0.33, yield_stress: 70, hardening_modulus: 500, failure_strain: 0.15 },
    titanium: { density: 4500, youngs_modulus: 116000, poisson_ratio: 0.34, yield_stress: 150, hardening_modulus: 800, failure_strain: 0.18 },
    rubber: { density: 1100, youngs_modulus: 10, poisson_ratio: 0.49, yield_stress: 5, hardening_modulus: 1, failure_strain: 3.0 },
  }
  const p = presets[materialPreset.value]
  if (p) Object.assign(solverMaterial, p)
}

async function generateDemoMesh() {
  generating.value = true
  try {
    demoMesh.value = await invoke('generate_explicit_demo_mesh', {
      nx: meshNx.value,
      ny: meshNy.value,
      nz: meshNz.value,
      size: meshSize.value,
    })
    modelReady.value = true
    solverResult.value = null
    currentFrameIndex.value = 0
  } catch (e) {
    console.error('生成网格失败:', e)
    alert('生成网格失败: ' + e)
  } finally {
    generating.value = false
  }
}

async function runSolver() {
  if (!demoMesh.value) return
  running.value = true
  solverResult.value = null
  try {
    const config = {
      nodes: demoMesh.value.nodes,
      elements: demoMesh.value.elements,
      node_masses: [],
      material: { ...solverMaterial },
      boundary_conditions: {
        fixed_nodes: demoMesh.value.fixed_nodes,
        prescribed_velocities: [],
        initial_velocities: demoMesh.value.initial_velocities.map((iv: any) => [iv[0], [iv[1][0], iv[1][1], initialVelocityZ.value]]),
      },
      contact_pairs: [],
      time_step: solverTimeStep.value,
      end_time: solverEndTime.value,
      output_interval: solverOutputInterval.value,
      damping: solverDamping.value,
    }
    solverResult.value = await invoke('run_explicit_solver', { config })
    currentFrameIndex.value = 0
    showFrame.value = true
  } catch (e) {
    console.error('求解失败:', e)
    alert('求解失败: ' + e)
  } finally {
    running.value = false
  }
}

function playSolverAnimation() {
  if (!solverResult.value || solverResult.value.frames.length < 2) return
  playing.value = true
  showFrame.value = true
  const totalFrames = solverResult.value.frames.length
  animationTimer.value = window.setInterval(() => {
    currentFrameIndex.value++
    if (currentFrameIndex.value >= totalFrames) {
      currentFrameIndex.value = totalFrames - 1
      pauseSolverAnimation()
    }
  }, 80)
}

function pauseSolverAnimation() {
  playing.value = false
  if (animationTimer.value !== null) {
    clearInterval(animationTimer.value)
    animationTimer.value = null
  }
}

function onFrameSliderChange() {
  showFrame.value = true
}

// SVG 辅助方法
function getElementPolygon(elem: number[], frame: ExplicitFrame): string {
  if (!demoMesh.value || !frame) return ''
  const nodes = demoMesh.value.nodes
  const disps = frame.node_displacements
  const scale = Math.min(svgWidth, svgHeight) / (meshSize.value * 1.5)
  const ox = svgWidth / 2
  const oy = svgHeight / 2

  // 取前面4个节点画2D投影 (XY平面)
  const pts = [0, 1, 2, 3].map(i => {
    const nid = elem[i]
    const x = nodes[nid][0] + (disps[nid] ? disps[nid][0] * scale * 0.1 : 0)
    const y = nodes[nid][1] + (disps[nid] ? disps[nid][1] * scale * 0.1 : 0)
    return `${ox + x * scale},${oy + y * scale}`
  })
  return pts.join(' ')
}

function getElementColor(eidx: number, frame: ExplicitFrame): string {
  if (!frame) return 'rgba(100,100,255,0.3)'
  const stress = frame.element_stresses[eidx] || 0
  const maxStress = Math.max(...frame.element_stresses, 1)
  const ratio = Math.min(stress / maxStress, 1)
  // 蓝 -> 绿 -> 黄 -> 红
  if (ratio < 0.33) {
    const t = ratio / 0.33
    return `rgba(${Math.round(50 + t * 50)}, ${Math.round(100 + t * 155)}, 255, 0.7)`
  } else if (ratio < 0.66) {
    const t = (ratio - 0.33) / 0.33
    return `rgba(${Math.round(100 + t * 155)}, 255, ${Math.round(255 - t * 200)}, 0.7)`
  } else {
    const t = (ratio - 0.66) / 0.34
    return `rgba(255, ${Math.round(255 - t * 200)}, ${Math.round(55 - t * 55)}, 0.8)`
  }
}

function isElementFailed(eidx: number, frame: ExplicitFrame): boolean {
  return frame ? frame.failed_elements.includes(eidx) : false
}

function frameToX(frameIdx: number): number {
  if (!solverResult.value || solverResult.value.frames.length < 2) return 40
  const plotW = energyChartWidth - 50
  return 40 + (frameIdx / (solverResult.value.frames.length - 1)) * plotW
}

function energyCurvePoints(type: 'kinetic' | 'internal' | 'total'): string {
  if (!solverResult.value) return ''
  const frames = solverResult.value.frames
  const plotH = energyChartHeight - 25
  const maxE = maxEnergy.value
  return frames.map((f, i) => {
    const val = type === 'kinetic' ? f.kinetic_energy : type === 'internal' ? f.internal_energy : f.total_energy
    const x = frameToX(i)
    const y = plotH - (val / maxE) * plotH
    return `${x},${y}`
  }).join(' ')
}

function loadTemplate(template: any) {
  if (template.params) {
    const p = template.params
    if (p.meshNx) meshNx.value = p.meshNx
    if (p.meshNy) meshNy.value = p.meshNy
    if (p.meshNz) meshNz.value = p.meshNz
    if (p.meshSize) meshSize.value = p.meshSize
    if (p.velocityZ) initialVelocityZ.value = p.velocityZ
    if (p.material) {
      materialPreset.value = p.material
      applyMaterialPreset()
    }
  }
  showTemplatesDialog.value = false
}

async function embedToNote() {
  if (!selectedNoteId.value) return
  showEmbedDialog.value = false
  alert('分析已嵌入到笔记')
}

onMounted(async () => {
  try {
    templates.value = await invoke('get_explicit_dynamics_templates')
  } catch (e) {
    templates.value = [
      { id: 'cube_impact', name: '立方体冲击', desc: '4x4x4网格，底面固定，顶面初速度', params: { meshNx: 4, meshNy: 4, meshNz: 4, meshSize: 10, velocityZ: -10, material: 'steel' } },
      { id: 'rubber_block', name: '橡胶块压缩', desc: '橡胶材料大变形分析', params: { meshNx: 5, meshNy: 5, meshNz: 5, meshSize: 10, velocityZ: -5, material: 'rubber' } },
      { id: 'aluminum_plate', name: '铝板冲击', desc: '铝板高速冲击', params: { meshNx: 6, meshNy: 6, meshNz: 3, meshSize: 10, velocityZ: -50, material: 'aluminum' } },
    ]
  }
})

onUnmounted(() => {
  if (animationTimer.value !== null) clearInterval(animationTimer.value)
})
</script>

<style scoped>
.input {
  @apply px-3 py-2 border border-[var(--border-default)] rounded bg-[var(--bg-primary)] text-[var(--text-primary)];
}
.btn {
  @apply px-4 py-2 rounded transition-colors font-medium;
}
.btn-primary {
  @apply bg-blue-600 text-white hover:bg-blue-700;
}
.btn-accent {
  @apply bg-emerald-600 text-white hover:bg-emerald-700;
}
.btn-ghost {
  @apply border border-[var(--border-default)] text-[var(--text-secondary)] hover:bg-[var(--bg-hover)];
}
</style>
