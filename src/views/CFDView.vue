<template>
  <div class="cfd-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">计算流体动力学</h2>
        <p class="text-xs text-[var(--text-muted)]">流体力学仿真、风洞分析、管道流动</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="importGeometry" class="btn btn-ghost text-xs">
          <span class="mr-1">📁</span> 导入几何
        </button>
        <button @click="resetSetup" class="btn btn-ghost text-xs">
          <span class="mr-1">⟳</span> 重置
        </button>
        <button @click="showEmbedDialog = true" class="btn btn-ghost text-xs">
          <span class="mr-1">🔗</span> 嵌入到笔记
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left: Setup Wizard -->
      <div class="w-72 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col">
        <div class="px-3 py-2 text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider border-b border-[var(--border-subtle)]">
          仿真设置
        </div>
        <div class="flex-1 overflow-y-auto p-3 space-y-4">
          <!-- Step 1: Domain Definition -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)]">1. 流体域定义</h3>
            <div class="space-y-1">
              <label class="flex items-center gap-2 cursor-pointer text-xs">
                <input type="checkbox" v-model="domainMarkingEnabled" class="rounded text-[var(--accent)]" />
                <span>启用区域标记</span>
              </label>
              <div v-if="domainMarkingEnabled" class="pl-4 pt-1 space-y-2">
                <div>
                  <p class="text-[10px] text-[var(--text-muted)] mb-1">当前标记类型</p>
                  <div class="flex gap-1">
                    <button
                      v-for="type in domainTypes"
                      :key="type.value"
                      @click="currentDomainType = type.value"
                      :class="['px-2 py-1 rounded text-[10px] transition', currentDomainType === type.value ? 'bg-[var(--accent)] text-white' : 'bg-[var(--bg-base)] text-[var(--text-secondary)]']"
                    >
                      {{ type.label }}
                    </button>
                  </div>
                </div>
                <p class="text-[10px] text-[var(--text-muted)]">
                  已标记: {{ markedRegions.length }} 个区域
                </p>
              </div>
            </div>
          </div>

          <!-- Step 2: Material -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)]">2. 流体材料</h3>
            <select v-model="selectedMaterial" class="input w-full text-xs">
              <option value="">选择材料...</option>
              <option v-for="mat in fluidMaterials" :key="mat.value" :value="mat.value">
                {{ mat.label }}
              </option>
            </select>
            <div v-if="selectedMaterial === 'custom'" class="space-y-1 pt-1">
              <div class="flex gap-2">
                <div class="flex-1">
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">密度 (kg/m³)</label>
                  <input v-model.number="customDensity" type="number" class="input w-full text-xs" />
                </div>
                <div class="flex-1">
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">粘度 (Pa·s)</label>
                  <input v-model.number="customViscosity" type="number" step="0.000001" class="input w-full text-xs" />
                </div>
              </div>
            </div>
          </div>

          <!-- Step 3: Boundary Conditions -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)]">3. 边界条件</h3>
            <div class="space-y-1">
              <div v-for="bc in boundaryConditions" :key="bc.id" class="flex items-center justify-between p-2 rounded bg-[var(--bg-base)]">
                <div class="flex-1 min-w-0">
                  <p class="text-xs font-medium text-[var(--text-primary)] truncate">{{ getBoundaryTypeName(bc.type) }}</p>
                  <p class="text-[10px] text-[var(--text-muted)] truncate">{{ bc.faces }} 个面</p>
                </div>
                <button @click="removeBoundary(bc.id)" class="text-[var(--text-muted)] hover:text-[var(--danger)]">×</button>
              </div>
              <button @click="addBoundary" class="btn btn-ghost w-full text-xs">+ 添加边界条件</button>
            </div>
          </div>

          <!-- Step 4: Turbulence Model -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)]">4. 湍流模型</h3>
            <div class="space-y-1">
              <label v-for="model in turbulenceModels" :key="model.value" class="flex items-center gap-2 cursor-pointer text-xs">
                <input type="radio" v-model="turbulenceModel" :value="model.value" class="text-[var(--accent)]" />
                <span>{{ model.label }}</span>
              </label>
            </div>
          </div>

          <!-- Step 5: Solver Control -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)]">5. 求解控制</h3>
            <div class="space-y-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">残差收敛标准 (10^-n)</label>
                <input v-model.number="convergenceTolerance" type="number" min="1" max="10" step="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">最大迭代步数</label>
                <input v-model.number="maxIterations" type="number" step="100" class="input w-full text-xs" />
              </div>
            </div>
          </div>
        </div>
        <!-- Run Button -->
        <div class="p-3 border-t border-[var(--border-subtle)]">
          <button @click="runCFD" :disabled="!canRun || running" class="btn btn-primary w-full text-xs">
            <span v-if="running" class="mr-1 animate-spin">⟳</span>
            <span v-else class="mr-1">▶</span>
            {{ running ? '生成中...' : '生成OpenFOAM案例' }}
          </button>
          <p v-if="!canRun" class="text-[10px] text-[var(--text-muted)] mt-1">
            ⚠️ 请完成所有必需设置
          </p>
        </div>
      </div>

      <!-- Center: 3D Viewport -->
      <div class="flex-1 relative flex flex-col">
        <!-- Canvas Container -->
        <div ref="canvasContainer" class="flex-1 bg-[var(--bg-base)]"></div>

        <!-- Toolbar -->
        <div class="h-10 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] flex items-center px-4 gap-4">
          <span class="text-[10px] text-[var(--text-muted)] uppercase tracking-wider">显示</span>
          <div class="flex items-center gap-1">
            <button
              @click="showVelocityVectors = !showVelocityVectors"
              :class="['icon-btn w-8 h-8', showVelocityVectors ? 'active' : '']"
              title="速度矢量"
            >
              <span class="text-sm">↗</span>
            </button>
            <button
              @click="showPressureContour = !showPressureContour"
              :class="['icon-btn w-8 h-8', showPressureContour ? 'active' : '']"
              title="压力云图"
            >
              <span class="text-sm">◼</span>
            </button>
            <button
              @click="showStreamlines = !showStreamlines"
              :class="['icon-btn w-8 h-8', showStreamlines ? 'active' : '']"
              title="流线"
            >
              <span class="text-sm">~</span>
            </button>
          </div>
          <div class="flex-1"></div>
          <button @click="downloadCase" :disabled="!caseGenerated" class="btn btn-ghost text-xs">
            <span class="mr-1">📥</span> 下载OpenFOAM案例
          </button>
        </div>
      </div>

      <!-- Right: Results -->
      <div class="w-64 bg-[var(--bg-surface)] border-l border-[var(--border-subtle)] flex flex-col" v-if="hasResults">
        <div class="px-3 py-2 text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider border-b border-[var(--border-subtle)]">
          结果统计
        </div>
        <div class="flex-1 overflow-y-auto p-3 space-y-4">
          <div v-if="resultStats" class="space-y-2">
            <div class="bg-[var(--bg-base)] rounded p-2 space-y-1">
              <div class="flex justify-between text-xs">
                <span class="text-[var(--text-muted)]">迭代步数</span>
                <span class="font-medium">{{ resultStats.iterations }}</span>
              </div>
              <div class="flex justify-between text-xs">
                <span class="text-[var(--text-muted)]">收敛</span>
                <span :class="['font-medium', resultStats.converged ? 'text-green-600' : 'text-orange-500']">
                  {{ resultStats.converged ? '是' : '否' }}
                </span>
              </div>
            </div>
            <div class="bg-[var(--bg-base)] rounded p-2 space-y-1">
              <div class="flex justify-between text-xs">
                <span class="text-[var(--text-muted)]">升力系数 Cl</span>
                <span class="font-medium">{{ resultStats.cl?.toFixed(4) ?? '-' }}</span>
              </div>
              <div class="flex justify-between text-xs">
                <span class="text-[var(--text-muted)]">阻力系数 Cd</span>
                <span class="font-medium">{{ resultStats.cd?.toFixed(4) ?? '-' }}</span>
              </div>
              <div class="flex justify-between text-xs">
                <span class="text-[var(--text-muted)]">力矩系数 Cm</span>
                <span class="font-medium">{{ resultStats.cm?.toFixed(4) ?? '-' }}</span>
              </div>
            </div>
          </div>
          <div>
            <h3 class="text-xs font-semibold mb-2">残差历史</h3>
            <div class="w-full h-32 bg-[var(--bg-base)] rounded p-2">
              <!-- Residual plot will be rendered here -->
            </div>
          </div>
        </div>
        <div class="p-3 border-t border-[var(--border-subtle)]">
          <button @click="generateReport" class="btn btn-primary w-full text-xs">
            <span class="mr-1">📄</span> 生成报告
          </button>
        </div>
      </div>
      <div v-else class="w-64 bg-[var(--bg-surface)] border-l border-[var(--border-subtle)] flex items-center justify-center">
        <div class="text-center p-4">
          <div class="text-4xl mb-2 opacity-30">🌊</div>
          <p class="text-xs text-[var(--text-muted)]">运行后显示结果</p>
        </div>
      </div>
    </div>

    <!-- Add Boundary Dialog -->
    <div v-if="showAddBoundaryDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-[var(--bg-surface)] rounded-lg p-4 w-80 max-w-full">
        <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-3">添加边界条件</h3>
        <div class="space-y-3">
          <div>
            <label class="text-xs text-[var(--text-muted)] block mb-1">边界类型</label>
            <select v-model="newBoundaryType" class="input w-full text-xs">
              <option value="velocityInlet">速度入口 (Velocity Inlet)</option>
              <option value="pressureOutlet">压力出口 (Pressure Outlet)</option>
              <option value="wall">壁面 (Wall)</option>
              <option value="symmetry">对称面 (Symmetry)</option>
              <option value="outlet">出流 (Outflow)</option>
            </select>
          </div>
          <div v-if="newBoundaryType === 'velocityInlet'" class="space-y-2">
            <div>
              <label class="text-xs text-[var(--text-muted)] block mb-1">速度大小 (m/s)</label>
              <input v-model.number="newVelocity" type="number" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-muted)] block mb-1">湍流强度 (%)</label>
              <input v-model.number="newTurbulenceIntensity" type="number" min="0.1" max="100" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-muted)] block mb-1">水力直径 (m)</label>
              <input v-model.number="newHydraulicDiameter" type="number" step="0.001" class="input w-full text-xs" />
            </div>
          </div>
          <div v-if="newBoundaryType === 'pressureOutlet'" class="space-y-2">
            <div>
              <label class="text-xs text-[var(--text-muted)] block mb-1">出口静压 (Pa)</label>
              <input v-model.number="newGaugePressure" type="number" class="input w-full text-xs" />
            </div>
          </div>
          <div v-if="['wall'].includes(newBoundaryType)" class="space-y-2">
            <label class="flex items-center gap-2 cursor-pointer text-xs">
              <input type="checkbox" v-model="newWallSlip" class="rounded text-[var(--accent)]" />
              <span>滑移壁面</span>
            </label>
          </div>
          <div class="flex gap-2 pt-2">
            <button @click="cancelAddBoundary" class="btn btn-ghost flex-1 text-xs">取消</button>
            <button @click="confirmAddBoundary" class="btn btn-primary flex-1 text-xs">添加</button>
          </div>
        </div>
      </div>
    </div>

    <!-- 🔗 嵌入到笔记对话框 -->
    <div v-if="showEmbedDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-[var(--bg-surface)] rounded-lg p-4 w-80 max-w-full">
        <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-3">
          <span class="mr-1">🔗</span> 嵌入到笔记
        </h3>
        <div class="space-y-3">
          <div>
            <label class="text-sm font-medium text-[var(--text-muted)] mb-2 block">选择要嵌入的笔记</label>
            <select v-model="embedTargetNoteId" class="input w-full text-sm">
              <option value="">选择笔记...</option>
              <option v-for="note in projectStore.notes" :key="note.id" :value="note.id">
                {{ note.name }}
              </option>
            </select>
          </div>
          <div v-if="embedTargetNoteId" class="bg-[var(--bg-base)] rounded p-2">
            <p class="text-xs text-[var(--text-muted)] mb-1">嵌入预览</p>
            <p class="text-sm text-[var(--text-primary)]">
              📊 CFD仿真: {{ projectStore.projectName }}
            </p>
            <p class="text-xs text-[var(--text-muted)]">
              {{ boundaryConditions.length }} 边界条件 · {{ turbulenceModel }}
            </p>
          </div>
          <div class="flex gap-2 pt-2">
            <button @click="cancelEmbed" class="btn btn-ghost flex-1 text-sm">取消</button>
            <button @click="confirmEmbed" :disabled="!embedTargetNoteId" class="btn btn-primary flex-1 text-sm">
              确认嵌入
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, inject } from 'vue'
import * as THREE from 'three'
import { useProjectStore } from '@/stores/project'

const projectStore = useProjectStore()

// Props and state
const domainMarkingEnabled = ref(false)
const currentDomainType = ref<'fluid' | 'solid'>('fluid')
const markedRegions = ref<any[]>([])
const selectedMaterial = ref('')
const customDensity = ref(1.225)
const customViscosity = ref(1.81e-5)
const boundaryConditions = ref<any[]>([])
const turbulenceModel = ref('kepsilon')
const convergenceTolerance = ref(6)
const maxIterations = ref(1000)
const running = ref(false)
const caseGenerated = ref(false)
const hasResults = ref(false)
const resultStats = ref<any>(null)
const showAddBoundaryDialog = ref(false)
const showEmbedDialog = ref(false)
const showVelocityVectors = ref(true)
const showPressureContour = ref(true)
const showStreamlines = ref(false)
const canvasContainer = ref<HTMLElement | null>(null)

// New boundary
const newBoundaryType = ref('velocityInlet')
const newVelocity = ref(10.0)
const newTurbulenceIntensity = ref(5.0)
const newHydraulicDiameter = ref(0.1)
const newGaugePressure = ref(0)
const newWallSlip = ref(false)

// Embed
const embedTargetNoteId = ref('')

// Domain types
const domainTypes = [
  { value: 'fluid', label: '流体域' },
  { value: 'solid', label: '固体域' },
]

// Fluid materials
const fluidMaterials = [
  { value: 'air', label: '空气 (1.225 kg/m³, 1.81e-5 Pa·s)' },
  { value: 'water', label: '水 (1000 kg/m³, 0.001 Pa·s)' },
  { value: 'oil', label: '机油 (850 kg/m³, 0.08 Pa·s)' },
  { value: 'custom', label: '自定义...' },
]

// Turbulence models
const turbulenceModels = [
  { value: 'laminar', label: '层流 (无湍流)' },
  { value: 'kepsilon', label: 'k-epsilon RANS' },
  { value: 'komegaSST', label: 'k-omega SST' },
]

// Computed
const canRun = computed(() => {
  if (!selectedMaterial.value) return false
  if (boundaryConditions.value.length === 0) return false
  if (!domainMarkingEnabled.value || markedRegions.value.length === 0) return false
  return true
})

// Methods
function getBoundaryTypeName(type: string): string {
  const names: Record<string, string> = {
    velocityInlet: '速度入口',
    pressureOutlet: '压力出口',
    wall: '壁面',
    symmetry: '对称面',
    outlet: '出流',
  }
  return names[type] || type
}

function addBoundary() {
  showAddBoundaryDialog.value = true
}

function cancelAddBoundary() {
  showAddBoundaryDialog.value = false
}

function confirmAddBoundary() {
  boundaryConditions.value.push({
    id: Date.now(),
    type: newBoundaryType.value,
    velocity: newVelocity.value,
    turbulenceIntensity: newTurbulenceIntensity.value,
    hydraulicDiameter: newHydraulicDiameter.value,
    gaugePressure: newGaugePressure.value,
    wallSlip: newWallSlip.value,
    faces: 0, // Will be updated when user selects faces
  })
  showAddBoundaryDialog.value = false
  // Reset
  newBoundaryType.value = 'velocityInlet'
  newVelocity.value = 10.0
  newTurbulenceIntensity.value = 5.0
  newHydraulicDiameter.value = 0.1
  newGaugePressure.value = 0
  newWallSlip.value = false
}

function removeBoundary(id: number) {
  const idx = boundaryConditions.value.findIndex(bc => bc.id === id)
  if (idx >= 0) {
    boundaryConditions.value.splice(idx, 1)
  }
}

function resetSetup() {
  domainMarkingEnabled.value = false
  markedRegions.value = []
  selectedMaterial.value = ''
  boundaryConditions.value = []
  turbulenceModel.value = 'kepsilon'
  convergenceTolerance.value = 6
  maxIterations.value = 1000
  caseGenerated.value = false
  hasResults.value = false
  resultStats.value = null
}

async function runCFD() {
  running.value = true
  try {
    // Call backend to generate OpenFOAM case
    const result = await window.__TAURI__.invoke('generate_openfoam_case', {
      domainMarking: markedRegions.value,
      material: selectedMaterial.value === 'custom'
        ? { density: customDensity.value, viscosity: customViscosity.value }
        : selectedMaterial.value,
      boundaryConditions: boundaryConditions.value,
      turbulenceModel: turbulenceModel.value,
      convergenceTolerance: Math.pow(10, -convergenceTolerance.value),
      maxIterations: maxIterations.value,
    })
    caseGenerated.value = true
    // If we have results, parse them
    if (result && result.stats) {
      resultStats.value = result.stats
      hasResults.value = true
    }
  } catch (e) {
    console.error('Failed to generate OpenFOAM case:', e)
  } finally {
    running.value = false
  }
}

function downloadCase() {
  // Trigger download of generated case
  window.__TAURI__.invoke('download_openfoam_case')
}

function importGeometry() {
  window.__TAURI__.invoke('import_cfd_geometry')
}

function generateReport() {
  window.__TAURI__.invoke('generate_cfd_report')
}

// ========== 嵌入到笔记功能 ==========
function cancelEmbed() {
  showEmbedDialog.value = false
  embedTargetNoteId.value = ''
}

async function confirmEmbed() {
  if (!embedTargetNoteId.value) return

  const embedRecord = {
    type: 'cfd',
    name: `CFD仿真: ${projectStore.projectName}`,
    config: {
      material: selectedMaterial.value,
      turbulence: turbulenceModel.value,
      boundaries: boundaryConditions.value.length,
    },
    timestamp: Date.now(),
  }

  await projectStore.addEmbedToNote(embedTargetNoteId.value, embedRecord)
  console.log('CFD仿真已嵌入到笔记:', embedRecord)
  alert('✓ CFD仿真已成功嵌入到笔记！\n\n在笔记中点击嵌入卡片可跳转到CFD界面。')
  showEmbedDialog.value = false
  embedTargetNoteId.value = ''
}

// Three.js initialization
let scene: THREE.Scene | null = null
let renderer: THREE.WebGLRenderer | null = null

onMounted(() => {
  if (!canvasContainer.value) return

  // Initialize Three.js scene
  const width = canvasContainer.value.clientWidth
  const height = canvasContainer.value.clientHeight

  scene = new THREE.Scene()
  scene.background = null

  const camera = new THREE.PerspectiveCamera(45, width / height, 0.1, 1000)
  camera.position.set(5, 5, 5)
  camera.lookAt(0, 0, 0)

  renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true })
  renderer.setSize(width, height)
  renderer.setPixelRatio(window.devicePixelRatio)
  canvasContainer.value.appendChild(renderer.domElement)

  // Add lights
  const ambientLight = new THREE.AmbientLight(0xffffff, 0.6)
  scene.add(ambientLight)
  const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8)
  directionalLight.position.set(10, 10, 5)
  scene.add(directionalLight)

  // Animation loop
  function animate() {
    requestAnimationFrame(animate)
    if (renderer && scene && camera) {
      renderer.render(scene, camera)
    }
  }
  animate()

  // Handle window resize
  function onResize() {
    if (!canvasContainer.value || !renderer || !camera) return
    const w = canvasContainer.value.clientWidth
    const h = canvasContainer.value.clientHeight
    camera.aspect = w / h
    camera.updateProjectionMatrix()
    renderer.setSize(w, h)
  }
  window.addEventListener('resize', onResize)
})
</script>

<style scoped>
/* Inherit global styles from app */
</style>
