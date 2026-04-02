<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch, nextTick } from 'vue'
import { useProjectStore } from '@/stores/project'
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import { invoke } from '@tauri-apps/api/core'
import * as caeApi from '@/api/cae'
import type { Node, Element } from '@/api/cae'
import * as optApi from '@/api/optimization'
import ConvergenceChart from '@/components/optimization/ConvergenceChart.vue'
import type { ChartDataPoint } from '@/components/optimization/ConvergenceChart.vue'
import ShapeOptimizationPanel from '@/components/optimization/ShapeOptimizationPanel.vue'

const projectStore = useProjectStore()

// ============ State ============
const activeStep = ref(1)
const running = ref(false)
const showEmbedDialog = ref(false)
const showResultDialog = ref(false)
const showExportDialog = ref(false)
const activeTab = ref<'topology' | 'shape'>('topology')

// Three.js
const canvasContainer = ref<HTMLDivElement | null>(null)
let scene: THREE.Scene
let camera: THREE.PerspectiveCamera
let renderer: THREE.WebGLRenderer
let controls: OrbitControls
let meshGroup: THREE.Group
let densityGroup: THREE.Group

// Design space
const designSpace = ref({
  xMin: 0, xMax: 100,
  yMin: 0, yMax: 20,
  zMin: 0, zMax: 10,
  xDiv: 40, yDiv: 8, zDiv: 4
})

const keepRegions = ref<Array<{
  id: string; type: 'box' | 'surface'
  xMin: number; xMax: number; yMin: number; yMax: number; zMin: number; zMax: number
  name: string
}>>([])

const material = ref({
  name: 'Aluminum',
  youngsModulus: 70000,
  poissonRatio: 0.3,
  density: 2.7e-6,
  penalization: 3.0,
  minDensity: 0.001
})

const optimizationObjective = ref({
  type: 'compliance',
  targetVolumeFraction: 0.3,
  weightVolume: 0.5,
  weightCompliance: 0.5
})

const constraintType = ref<'volume_only' | 'volume_displacement'>('volume_only')
const maxDisplacement = ref(0.5)

const solverControl = ref({
  maxIterations: 200,
  convergenceTolerance: 0.01,
  filterRadius: 3.0,
  moveLimit: 0.2,
  oscillationFilter: true
})

const boundaryConditions = ref({
  fixedBc: { position: 'left', type: 'displacement' },
  load: { position: 'right', direction: 'y', magnitude: 100 }
})

// Optimization result (from backend)
const optimizationResult = ref<{
  success: boolean
  iterations: Array<{
    iteration: number
    objective_value: number
    volume_fraction: number
    max_stress: number | null
    max_density: number
    min_density: number
    converged: boolean
  }>
  final_objective: number
  final_volume: number
  final_density_field: number[]
  elapsed_time_seconds: number
  error_message?: string
} | null>(null)

const currentIteration = ref(0)
const animating = ref(false)
const animationSpeed = ref(200) // ms per frame
const animationFrameId = ref<number>(0)

// Visualization
const displayMode = ref<'density' | 'displacement' | 'stress'>('density')
const colormap = ref('viridis')
const showWireframe = ref(false)
const opacity = ref(1.0)
const deformationScale = ref(1.0)

// Convergence chart data
const chartData = computed<ChartDataPoint[]>(() => {
  if (!optimizationResult.value) return []
  return optimizationResult.value.iterations.map(iter => ({
    iteration: iter.iteration,
    compliance: iter.objective_value,
    volume: iter.volume_fraction,
  }))
})

// ============ Computed ============
const canRunOptimization = computed(() => {
  return designSpace.value.xDiv > 0 &&
    designSpace.value.yDiv > 0 &&
    material.value.youngsModulus > 0 &&
    optimizationObjective.value.targetVolumeFraction > 0 &&
    optimizationObjective.value.targetVolumeFraction < 1
})

const totalElements = computed(() => {
  return designSpace.value.xDiv * designSpace.value.yDiv * designSpace.value.zDiv
})

const estimatedNodes = computed(() => {
  return (designSpace.value.xDiv + 1) * (designSpace.value.yDiv + 1) * (designSpace.value.zDiv + 1)
})

// ============ Three.js Init ============
function initThreeJS() {
  if (!canvasContainer.value) return

  const width = canvasContainer.value.clientWidth
  const height = canvasContainer.value.clientHeight

  scene = new THREE.Scene()
  scene.background = new THREE.Color(0x0A0B0E)

  camera = new THREE.PerspectiveCamera(60, width / height, 0.1, 1000)
  camera.position.set(150, 100, 150)

  renderer = new THREE.WebGLRenderer({ antialias: true })
  renderer.setSize(width, height)
  renderer.setPixelRatio(window.devicePixelRatio)
  canvasContainer.value.appendChild(renderer.domElement)

  controls = new OrbitControls(camera, renderer.domElement)
  controls.enableDamping = true
  controls.dampingFactor = 0.05

  meshGroup = new THREE.Group()
  densityGroup = new THREE.Group()
  scene.add(meshGroup)
  scene.add(densityGroup)

  const ambientLight = new THREE.AmbientLight(0xffffff, 0.6)
  scene.add(ambientLight)

  const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8)
  directionalLight.position.set(100, 100, 50)
  scene.add(directionalLight)

  const directionalLight2 = new THREE.DirectionalLight(0xffffff, 0.4)
  directionalLight2.position.set(-50, 50, -50)
  scene.add(directionalLight2)

  const gridHelper = new THREE.GridHelper(200, 20, 0x333333, 0x222222)
  gridHelper.position.y = -0.1
  scene.add(gridHelper)

  const axesHelper = new THREE.AxesHelper(50)
  scene.add(axesHelper)

  animate()
}

function animate() {
  if (!renderer) return
  requestAnimationFrame(animate)
  controls?.update()
  renderer.render(scene, camera)
}

function handleResize() {
  if (!canvasContainer.value || !camera || !renderer) return
  const width = canvasContainer.value.clientWidth
  const height = canvasContainer.value.clientHeight
  camera.aspect = width / height
  camera.updateProjectionMatrix()
  renderer.setSize(width, height)
}

function getColormapColor(value: number, mapName: string): THREE.Color {
  const t = Math.max(0, Math.min(1, value))

  const colormaps: Record<string, Array<[number, number, number]>> = {
    viridis: [
      [0.267, 0.004, 0.329], [0.282, 0.140, 0.458], [0.254, 0.265, 0.530],
      [0.192, 0.408, 0.553], [0.126, 0.566, 0.550], [0.200, 0.722, 0.490],
      [0.564, 0.850, 0.310], [0.993, 0.906, 0.144]
    ],
    plasma: [
      [0.050, 0.030, 0.528], [0.417, 0.001, 0.658], [0.695, 0.165, 0.564],
      [0.898, 0.360, 0.359], [0.988, 0.652, 0.195], [0.940, 0.975, 0.131]
    ],
    inferno: [
      [0.001, 0.000, 0.014], [0.258, 0.038, 0.406], [0.578, 0.148, 0.404],
      [0.865, 0.317, 0.226], [0.987, 0.645, 0.039], [0.988, 1.000, 0.644]
    ],
    jet: [
      [0, 0, 0.5], [0, 0, 1], [0, 1, 1], [0, 1, 0], [1, 1, 0], [1, 0, 0], [0.5, 0, 0]
    ],
    rainbow: [
      [0, 0, 1], [0, 1, 1], [0, 1, 0], [1, 1, 0], [1, 0, 0], [1, 0, 1]
    ]
  }

  const colors = colormaps[mapName] || colormaps.viridis
  const idx = t * (colors.length - 1)
  const i = Math.floor(idx)
  const f = idx - i

  if (i >= colors.length - 1) {
    return new THREE.Color(colors[colors.length - 1][0], colors[colors.length - 1][1], colors[colors.length - 1][2])
  }

  const c1 = colors[i]
  const c2 = colors[i + 1]
  return new THREE.Color(
    c1[0] + (c2[0] - c1[0]) * f,
    c1[1] + (c2[1] - c1[1]) * f,
    c1[2] + (c2[2] - c1[2]) * f
  )
}

function renderMesh(mesh: { nodes: Node[]; elements: Element[] }) {
  if (!meshGroup) return

  while (meshGroup.children.length > 0) {
    meshGroup.remove(meshGroup.children[0])
  }

  const { xMin, xMax, yMin, yMax, zMin, zMax, xDiv, yDiv, zDiv } = designSpace.value

  mesh.elements.forEach((elem) => {
    const nodes = elem.node_ids.map(id => mesh.nodes.find(n => n.id === id))
    if (nodes.some(n => !n)) return

    const cx = nodes.reduce((sum, n) => sum + n!.x, 0) / nodes.length
    const cy = nodes.reduce((sum, n) => sum + n!.y, 0) / nodes.length
    const cz = nodes.reduce((sum, n) => sum + n!.z, 0) / nodes.length

    const boxGeo = new THREE.BoxGeometry(
      (xMax - xMin) / xDiv,
      (yMax - yMin) / yDiv,
      (zMax - zMin) / zDiv
    )
    boxGeo.translate(cx, cy, cz)

    const color = getColormapColor(cx / xMax, colormap.value)
    const materialObj = new THREE.MeshLambertMaterial({
      color: color,
      transparent: true,
      opacity: opacity.value
    })

    const meshObj = new THREE.Mesh(boxGeo, materialObj)
    meshGroup.add(meshObj)
  })
}

async function generateMesh() {
  running.value = true
  try {
    const { xMin, xMax, yMin, yMax, zMin, zMax, xDiv, yDiv, zDiv } = designSpace.value
    const mesh = await caeApi.generate3dMesh(xMin, xMax, xDiv, yMin, yMax, yDiv, zMin, zMax, zDiv, 'C3D8')
    projectStore.setMesh(mesh)
    renderMesh(mesh)
  } catch (e) {
    console.error('Mesh generation failed:', e)
  } finally {
    running.value = false
  }
}

function renderDensity(densityArray: number[]) {
  if (!densityGroup) return

  while (densityGroup.children.length > 0) {
    densityGroup.remove(densityGroup.children[0])
  }

  const { xMin, xMax, yMin, yMax, zMin, zMax, xDiv, yDiv, zDiv } = designSpace.value

  for (let i = 0; i < densityArray.length; i++) {
    const ix = i % xDiv
    const iy = Math.floor((i / xDiv) % yDiv)
    const iz = Math.floor(i / (xDiv * yDiv))

    const cx = xMin + (ix + 0.5) * (xMax - xMin) / xDiv
    const cy = yMin + (iy + 0.5) * (yMax - yMin) / yDiv
    const cz = zMin + (iz + 0.5) * (zMax - zMin) / zDiv

    const density = densityArray[i]

    const boxGeo = new THREE.BoxGeometry(
      (xMax - xMin) / xDiv * 0.95,
      (yMax - yMin) / yDiv * 0.95,
      (zMax - zMin) / zDiv * 0.95
    )
    boxGeo.translate(cx, cy, cz)

    const color = getColormapColor(density, colormap.value)
    const materialObj = new THREE.MeshLambertMaterial({
      color: color,
      transparent: true,
      opacity: density > 0.01 ? Math.min(1, density + 0.2) : 0.05
    })

    const mesh = new THREE.Mesh(boxGeo, materialObj)
    densityGroup.add(mesh)
  }
}

// ============ Backend Integration ============
async function runOptimization() {
  running.value = true
  optimizationResult.value = null
  currentIteration.value = 0

  try {
    // Reset optimizer state
    await optApi.resetOptimizer()

    const { xMin, xMax, yMin, yMax, zMin, zMax, xDiv, yDiv, zDiv } = designSpace.value

    // Generate mesh if not already present
    let mesh = projectStore.currentMesh
    if (!mesh || mesh.nodes.length === 0) {
      mesh = await caeApi.generate3dMesh(xMin, xMax, xDiv, yMin, yMax, yDiv, zMin, zMax, zDiv, 'C3D8')
      projectStore.setMesh(mesh)
      renderMesh(mesh)
    }

    // Build topology config for backend
    const config = {
      optimization_type: 'Topology',
      objective: optimizationObjective.value.type === 'compliance' ? 'MinCompliance' : 'MinMass',
      constraints: [
        {
          constraint_type: 'VolumeFraction',
          upper_bound: optimizationObjective.value.targetVolumeFraction,
        },
      ],
      max_iterations: solverControl.value.maxIterations,
      convergence_tolerance: solverControl.value.convergenceTolerance,
      design_domain: {
        x_min: xMin, x_max: xMax,
        y_min: yMin, y_max: yMax,
        z_min: zMin, z_max: zMax,
      },
      penalization_factor: material.value.penalization,
      min_density: material.value.minDensity,
      filter_radius: solverControl.value.filterRadius,
      oc_beta: 1.5,
      oc_move_limit: solverControl.value.moveLimit,
    }

    const materialObj = {
      name: material.value.name,
      youngs_modulus: material.value.youngsModulus,
      poisson_ratio: material.value.poissonRatio,
      density: material.value.density,
    }

    // Build boundary conditions
    const bcContainer = {
      fixed_bcs: [],
      loads: [],
    }

    // Call real backend
    const meshData = mesh!
    const result = await optApi.runTopologyOptimizationFull({
      config,
      nodes: meshData.nodes.map((n: { id: number; x: number; y: number; z: number }) => ({ id: n.id, x: n.x, y: n.y, z: n.z })),
      elements: meshData.elements.map((e: { id: number; node_ids: number[] }) => ({ id: e.id, node_ids: e.node_ids })),
      boundary_conditions: bcContainer,
      material: materialObj,
    })

    optimizationResult.value = result

    // Render final density field
    if (result.final_density_field && result.final_density_field.length > 0) {
      renderDensity(result.final_density_field)
    }

    // Auto-switch to results step
    activeStep.value = 5
    showResultDialog.value = true

  } catch (e) {
    console.error('Optimization failed:', e)
    optimizationResult.value = {
      success: false,
      iterations: [],
      final_objective: 0,
      final_volume: 0,
      final_density_field: [],
      elapsed_time_seconds: 0,
      error_message: String(e),
    }
  } finally {
    running.value = false
  }
}

// ============ Iteration Animation ============
async function playIterationAnimation() {
  if (!optimizationResult.value || optimizationResult.value.iterations.length === 0) return

  animating.value = true
  const totalIters = optimizationResult.value.iterations.length

  for (let i = 0; i < totalIters; i++) {
    if (!animating.value) break

    currentIteration.value = i + 1

    try {
      const densityField = await optApi.getIterationDensityField(i)
      if (densityField && densityField.length > 0) {
        renderDensity(densityField)
      }
    } catch (e) {
      // If backend doesn't have the iteration data, generate from final field
      console.warn(`Could not get iteration ${i} density:`, e)
    }

    await new Promise(resolve => {
      animationFrameId.value = window.setTimeout(resolve, animationSpeed.value)
    })
  }

  animating.value = false
}

function stepForward() {
  if (!optimizationResult.value) return
  const maxIter = optimizationResult.value.iterations.length
  if (currentIteration.value < maxIter) {
    currentIteration.value++
    loadIterationDensity(currentIteration.value - 1)
  }
}

function stepBackward() {
  if (!optimizationResult.value) return
  if (currentIteration.value > 1) {
    currentIteration.value--
    loadIterationDensity(currentIteration.value - 1)
  }
}

async function loadIterationDensity(iterIndex: number) {
  try {
    const densityField = await optApi.getIterationDensityField(iterIndex)
    if (densityField && densityField.length > 0) {
      renderDensity(densityField)
    }
  } catch (e) {
    console.warn(`Could not load iteration ${iterIndex}:`, e)
  }
}

function stopAnimation() {
  animating.value = false
  if (animationFrameId.value) {
    clearTimeout(animationFrameId.value)
  }
}

// ============ STL Export ============
async function exportSTL() {
  if (!optimizationResult.value || !densityGroup) return

  try {
    const { xMin, xMax, yMin, yMax, zMin, zMax, xDiv, yDiv, zDiv } = designSpace.value
    let stlContent = 'solid topology_optimization\n'

    densityGroup.children.forEach((child) => {
      if (child instanceof THREE.Mesh && child.geometry instanceof THREE.BoxGeometry) {
        const geometry = child.geometry
        const positionAttr = geometry.attributes.position
        const matrix = child.matrixWorld

        const vertices: THREE.Vector3[] = []
        for (let i = 0; i < 8; i++) {
          const v = new THREE.Vector3(
            positionAttr.getX(i),
            positionAttr.getY(i),
            positionAttr.getZ(i)
          )
          v.applyMatrix4(matrix)
          vertices.push(v)
        }

        const faces = [
          [0, 1, 2, 0, 2, 3],
          [4, 6, 5, 4, 7, 6],
          [0, 4, 5, 0, 5, 1],
          [2, 6, 7, 2, 7, 3],
          [0, 3, 7, 0, 7, 4],
          [1, 5, 6, 1, 6, 2]
        ]

        faces.forEach(face => {
          const v0 = vertices[face[0]]
          const v1 = vertices[face[1]]
          const v2 = vertices[face[2]]

          const edge1 = new THREE.Vector3().subVectors(v1, v0)
          const edge2 = new THREE.Vector3().subVectors(v2, v0)
          const normal = new THREE.Vector3().crossVectors(edge1, edge2).normalize()

          stlContent += `  facet normal ${normal.x.toExponential(6)} ${normal.y.toExponential(6)} ${normal.z.toExponential(6)}\n`
          stlContent += `    outer loop\n`
          stlContent += `      vertex ${v0.x.toExponential(6)} ${v0.y.toExponential(6)} ${v0.z.toExponential(6)}\n`
          stlContent += `      vertex ${v1.x.toExponential(6)} ${v1.y.toExponential(6)} ${v1.z.toExponential(6)}\n`
          stlContent += `      vertex ${v2.x.toExponential(6)} ${v2.y.toExponential(6)} ${v2.z.toExponential(6)}\n`
          stlContent += `    endloop\n`
          stlContent += `  endfacet\n`
        })
      }
    })

    stlContent += 'endsolid topology_optimization\n'

    await invoke('save_file_content', {
      path: 'topology_optimization.stl',
      content: stlContent
    })

    showExportDialog.value = true
  } catch (e) {
    console.error('Export STL failed:', e)
    alert('Export STL failed: ' + e)
  }
}

// ============ Helpers ============
function applyMaterialPreset() {
  const presets: Record<string, Partial<typeof material.value>> = {
    Aluminum: { youngsModulus: 70000, poissonRatio: 0.3, density: 2.7e-6 },
    Steel: { youngsModulus: 210000, poissonRatio: 0.3, density: 7.8e-6 },
    Titanium: { youngsModulus: 110000, poissonRatio: 0.34, density: 4.5e-6 },
    CarbonFiber: { youngsModulus: 150000, poissonRatio: 0.3, density: 1.6e-6 },
    Custom: {}
  }
  const preset = presets[material.value.name]
  if (preset) Object.assign(material.value, preset)
}

function addKeepRegion() {
  keepRegions.value.push({
    id: Date.now().toString(),
    type: 'box',
    xMin: designSpace.value.xMin,
    xMax: designSpace.value.xMin + 10,
    yMin: designSpace.value.yMin,
    yMax: designSpace.value.yMax,
    zMin: designSpace.value.zMin,
    zMax: designSpace.value.zMax,
    name: `Keep Region ${keepRegions.value.length + 1}`
  })
}

function removeKeepRegion(id: string) {
  keepRegions.value = keepRegions.value.filter(r => r.id !== id)
}

function resetAll() {
  activeStep.value = 1
  optimizationResult.value = null
  currentIteration.value = 0
  keepRegions.value = []
  if (meshGroup) {
    while (meshGroup.children.length > 0) meshGroup.remove(meshGroup.children[0])
  }
  if (densityGroup) {
    while (densityGroup.children.length > 0) densityGroup.remove(densityGroup.children[0])
  }
}

onMounted(() => {
  initThreeJS()
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
  stopAnimation()
  if (renderer) renderer.dispose()
})

watch(displayMode, () => {
  if (optimizationResult.value) renderDensity(optimizationResult.value.final_density_field)
})

watch(colormap, () => {
  if (optimizationResult.value) renderDensity(optimizationResult.value.final_density_field)
})
</script>

<template>
  <div class="topology-optimization-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">Topology Optimization</h2>
        <p class="text-xs text-[var(--text-muted)]">SIMP method structural topology optimization</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- Tab switch: Topology / Shape -->
        <div class="bg-gray-100 dark:bg-gray-700 rounded-lg p-0.5 flex mr-2">
          <button
            @click="activeTab = 'topology'"
            :class="[
              'px-3 py-1 rounded-md text-xs transition-colors',
              activeTab === 'topology'
                ? 'bg-white dark:bg-gray-600 shadow text-pink-600 dark:text-pink-400'
                : 'text-gray-600 dark:text-gray-400 hover:text-gray-900'
            ]"
          >
            Topology
          </button>
          <button
            @click="activeTab = 'shape'"
            :class="[
              'px-3 py-1 rounded-md text-xs transition-colors',
              activeTab === 'shape'
                ? 'bg-white dark:bg-gray-600 shadow text-pink-600 dark:text-pink-400'
                : 'text-gray-600 dark:text-gray-400 hover:text-gray-900'
            ]"
          >
            Shape
          </button>
        </div>

        <!-- Step indicator -->
        <div class="flex items-center gap-1 mr-2">
          <template v-for="step in 5" :key="step">
            <button
              @click="activeStep = step"
              :class="[
                'w-7 h-7 rounded-full text-xs font-medium transition',
                activeStep === step
                  ? 'bg-pink-600 text-white'
                  : activeStep > step
                    ? 'bg-green-600 text-white'
                    : 'bg-gray-300 dark:bg-gray-600 text-gray-600 dark:text-gray-400'
              ]"
            >
              {{ step }}
            </button>
            <span v-if="step < 5" class="text-gray-400 text-xs">-></span>
          </template>
        </div>

        <button
          @click="showEmbedDialog = true"
          :disabled="!projectStore.currentNoteId || !optimizationResult"
          class="px-3 py-1.5 text-xs border border-purple-300 text-purple-600 rounded hover:bg-purple-50 disabled:opacity-50"
        >
          Embed
        </button>
        <button
          @click="exportSTL"
          :disabled="!optimizationResult"
          class="px-3 py-1.5 text-xs border border-green-300 text-green-600 rounded hover:bg-green-50 disabled:opacity-50"
        >
          Export STL
        </button>
        <button @click="resetAll" class="px-3 py-1.5 text-xs border border-gray-300 rounded hover:bg-gray-50">
          Reset
        </button>
        <button
          @click="runOptimization"
          :disabled="!canRunOptimization || running"
          class="px-3 py-1.5 text-xs bg-pink-600 text-white rounded hover:bg-pink-700 disabled:opacity-50"
        >
          <span v-if="running" class="mr-1 animate-spin">~</span>
          <span v-else class="mr-1">></span>
          {{ running ? 'Optimizing...' : 'Run' }}
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col overflow-y-auto">

        <!-- Shape Optimization Panel (when shape tab active) -->
        <ShapeOptimizationPanel v-if="activeTab === 'shape'" />

        <!-- Topology Optimization Steps (when topology tab active) -->
        <template v-if="activeTab === 'topology'">
        <!-- Step 1: Design Space -->
        <div v-show="activeStep === 1" class="p-4 space-y-4">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] flex items-center gap-2">
            <span class="w-6 h-6 rounded-full bg-pink-600 text-white text-xs flex items-center justify-center">1</span>
            Design Space
          </h3>

          <div class="space-y-3 pl-8">
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">X Range (mm)</label>
                <div class="flex gap-1">
                  <input v-model.number="designSpace.xMin" type="number" step="1" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
                  <input v-model.number="designSpace.xMax" type="number" step="1" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
                </div>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Y Range (mm)</label>
                <div class="flex gap-1">
                  <input v-model.number="designSpace.yMin" type="number" step="1" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
                  <input v-model.number="designSpace.yMax" type="number" step="1" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
                </div>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Z Range (mm)</label>
                <div class="flex gap-1">
                  <input v-model.number="designSpace.zMin" type="number" step="0.1" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
                  <input v-model.number="designSpace.zMax" type="number" step="0.1" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
                </div>
              </div>
            </div>

            <div class="grid grid-cols-3 gap-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">X Div</label>
                <input v-model.number="designSpace.xDiv" type="number" min="1" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Y Div</label>
                <input v-model.number="designSpace.yDiv" type="number" min="1" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Z Div</label>
                <input v-model.number="designSpace.zDiv" type="number" min="1" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
              </div>
            </div>

            <p class="text-[10px] text-[var(--text-muted)]">
              {{ totalElements }} elements, {{ estimatedNodes }} nodes
            </p>
          </div>

          <!-- Keep Regions -->
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <h4 class="text-xs font-semibold text-[var(--text-secondary)]">Keep Regions</h4>
              <button @click="addKeepRegion" class="text-xs text-pink-600 hover:underline">+ Add</button>
            </div>
            <div v-for="region in keepRegions" :key="region.id" class="bg-gray-50 dark:bg-gray-800 rounded p-2 space-y-2">
              <div class="flex items-center justify-between">
                <input v-model="region.name" class="text-xs font-medium bg-transparent border-b border-dashed border-gray-300 focus:border-pink-500 outline-none text-[var(--text-primary)]" />
                <button @click="removeKeepRegion(region.id)" class="text-red-500 text-xs hover:underline">Delete</button>
              </div>
              <div class="grid grid-cols-2 gap-1">
                <div>
                  <label class="text-[9px] text-[var(--text-muted)]">X Range</label>
                  <div class="flex gap-1">
                    <input v-model.number="region.xMin" type="number" class="w-full px-1 py-0.5 border rounded text-[10px] bg-[var(--bg-base)] text-[var(--text-primary)]" />
                    <input v-model.number="region.xMax" type="number" class="w-full px-1 py-0.5 border rounded text-[10px] bg-[var(--bg-base)] text-[var(--text-primary)]" />
                  </div>
                </div>
              </div>
            </div>
            <p v-if="keepRegions.length === 0" class="text-[10px] text-gray-400 italic">
              No keep regions defined. Entire design space will participate.
            </p>
          </div>
        </div>

        <!-- Step 2: Boundary Conditions -->
        <div v-show="activeStep === 2" class="p-4 space-y-4">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] flex items-center gap-2">
            <span class="w-6 h-6 rounded-full bg-pink-600 text-white text-xs flex items-center justify-center">2</span>
            Loads & Constraints
          </h3>

          <div class="space-y-3 pl-8">
            <div class="border rounded p-3 space-y-2">
              <h4 class="text-xs font-semibold text-[var(--text-secondary)]">Constraint</h4>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Fixed Position</label>
                <select v-model="boundaryConditions.fixedBc.position" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]">
                  <option value="left">Left</option>
                  <option value="right">Right</option>
                  <option value="bottom">Bottom</option>
                  <option value="top">Top</option>
                </select>
              </div>
            </div>

            <div class="border rounded p-3 space-y-2">
              <h4 class="text-xs font-semibold text-[var(--text-secondary)]">Load</h4>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Position</label>
                <select v-model="boundaryConditions.load.position" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]">
                  <option value="left">Left</option>
                  <option value="right">Right</option>
                  <option value="top">Top</option>
                  <option value="bottom">Bottom</option>
                </select>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Direction</label>
                <select v-model="boundaryConditions.load.direction" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]">
                  <option value="x">X</option>
                  <option value="y">Y</option>
                  <option value="z">Z</option>
                </select>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Magnitude (N)</label>
                <input v-model.number="boundaryConditions.load.magnitude" type="number" step="1" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
              </div>
            </div>
          </div>
        </div>

        <!-- Step 3: Material and Objective -->
        <div v-show="activeStep === 3" class="p-4 space-y-4">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] flex items-center gap-2">
            <span class="w-6 h-6 rounded-full bg-pink-600 text-white text-xs flex items-center justify-center">3</span>
            Material & Objective
          </h3>

          <div class="space-y-3 pl-8">
            <div class="border rounded p-3 space-y-2">
              <label class="text-[10px] text-[var(--text-muted)] block mb-1">Material</label>
              <select v-model="material.name" @change="applyMaterialPreset" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]">
                <option value="Aluminum">Aluminum</option>
                <option value="Steel">Steel</option>
                <option value="Titanium">Titanium</option>
                <option value="CarbonFiber">Carbon Fiber</option>
                <option value="Custom">Custom</option>
              </select>
              <div class="grid grid-cols-2 gap-2">
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">E (MPa)</label>
                  <input v-model.number="material.youngsModulus" type="number" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">Poisson</label>
                  <input v-model.number="material.poissonRatio" type="number" step="0.01" min="0" max="0.5" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">Density</label>
                  <input v-model.number="material.density" type="number" step="1e-9" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">Penalization</label>
                  <input v-model.number="material.penalization" type="number" step="0.1" min="1" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
                </div>
              </div>
            </div>

            <div class="border rounded p-3 space-y-2">
              <h4 class="text-xs font-semibold text-[var(--text-secondary)]">Objective</h4>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Type</label>
                <select v-model="optimizationObjective.type" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]">
                  <option value="compliance">Min Compliance</option>
                  <option value="volume">Min Volume</option>
                </select>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Target Volume Fraction (0-1)</label>
                <input v-model.number="optimizationObjective.targetVolumeFraction" type="number" step="0.01" min="0.01" max="0.99" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
              </div>
            </div>

            <!-- Constraint type selection -->
            <div class="border rounded p-3 space-y-2">
              <h4 class="text-xs font-semibold text-[var(--text-secondary)]">Constraints</h4>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Constraint Type</label>
                <select v-model="constraintType" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]">
                  <option value="volume_only">Volume Only</option>
                  <option value="volume_displacement">Volume + Displacement</option>
                </select>
              </div>
              <div v-if="constraintType === 'volume_displacement'">
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Max Displacement (mm)</label>
                <input v-model.number="maxDisplacement" type="number" step="0.01" min="0.001" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
              </div>
            </div>
          </div>
        </div>

        <!-- Step 4: Solver Control -->
        <div v-show="activeStep === 4" class="p-4 space-y-4">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] flex items-center gap-2">
            <span class="w-6 h-6 rounded-full bg-pink-600 text-white text-xs flex items-center justify-center">4</span>
            Solver Control
          </h3>

          <div class="space-y-3 pl-8">
            <div class="border rounded p-3 space-y-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Max Iterations</label>
                <input v-model.number="solverControl.maxIterations" type="number" min="10" max="1000" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Convergence Tolerance</label>
                <input v-model.number="solverControl.convergenceTolerance" type="number" step="0.001" min="0.0001" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Filter Radius (mm)</label>
                <input v-model.number="solverControl.filterRadius" type="number" step="0.1" min="0" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Move Limit</label>
                <input v-model.number="solverControl.moveLimit" type="number" step="0.01" min="0.01" max="1" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
              </div>
              <div class="flex items-center gap-2">
                <input type="checkbox" v-model="solverControl.oscillationFilter" class="rounded" />
                <label class="text-[10px] text-[var(--text-muted)]">Oscillation Filter</label>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 5: Results -->
        <div v-show="activeStep === 5" class="p-4 space-y-4">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] flex items-center gap-2">
            <span class="w-6 h-6 rounded-full bg-pink-600 text-white text-xs flex items-center justify-center">5</span>
            Results
          </h3>

          <div v-if="optimizationResult" class="space-y-3 pl-8">
            <!-- Summary -->
            <div class="border rounded p-3 space-y-2">
              <h4 class="text-xs font-semibold text-[var(--text-secondary)]">Summary</h4>
              <div class="grid grid-cols-2 gap-2 text-xs">
                <div class="bg-gray-50 dark:bg-gray-800 rounded p-2">
                  <span class="text-gray-500 dark:text-gray-400">Iterations</span>
                  <p class="font-semibold text-[var(--text-primary)]">{{ optimizationResult.iterations.length }}</p>
                </div>
                <div class="bg-gray-50 dark:bg-gray-800 rounded p-2">
                  <span class="text-gray-500 dark:text-gray-400">Converged</span>
                  <p :class="optimizationResult.iterations[optimizationResult.iterations.length - 1]?.converged ? 'text-green-600' : 'text-yellow-600'">
                    {{ optimizationResult.iterations[optimizationResult.iterations.length - 1]?.converged ? 'Yes' : 'No' }}
                  </p>
                </div>
                <div class="bg-gray-50 dark:bg-gray-800 rounded p-2">
                  <span class="text-gray-500 dark:text-gray-400">Final Volume</span>
                  <p class="font-semibold text-[var(--text-primary)]">{{ (optimizationResult.final_volume * 100).toFixed(1) }}%</p>
                </div>
                <div class="bg-gray-50 dark:bg-gray-800 rounded p-2">
                  <span class="text-gray-500 dark:text-gray-400">Final Compliance</span>
                  <p class="font-semibold text-[var(--text-primary)]">{{ optimizationResult.final_objective.toFixed(4) }}</p>
                </div>
                <div class="bg-gray-50 dark:bg-gray-800 rounded p-2">
                  <span class="text-gray-500 dark:text-gray-400">Elapsed</span>
                  <p class="font-semibold text-[var(--text-primary)]">{{ optimizationResult.elapsed_time_seconds.toFixed(2) }}s</p>
                </div>
              </div>
            </div>

            <!-- Animation Control -->
            <div class="border rounded p-3 space-y-2">
              <h4 class="text-xs font-semibold text-[var(--text-secondary)]">Iteration Animation</h4>
              <div class="flex items-center gap-2">
                <button
                  @click="animating ? stopAnimation() : playIterationAnimation()"
                  class="px-3 py-1 bg-pink-600 text-white rounded text-xs hover:bg-pink-700"
                >
                  {{ animating ? 'Pause' : 'Play' }}
                </button>
                <button @click="stepBackward" :disabled="currentIteration <= 1" class="px-2 py-1 border rounded text-xs hover:bg-gray-50 disabled:opacity-50">
                  |<
                </button>
                <button @click="stepForward" :disabled="currentIteration >= (optimizationResult?.iterations.length || 0)" class="px-2 py-1 border rounded text-xs hover:bg-gray-50 disabled:opacity-50">
                  >|
                </button>
                <span class="text-xs text-gray-500">
                  {{ currentIteration }} / {{ optimizationResult.iterations.length }}
                </span>
              </div>
              <div class="flex items-center gap-2">
                <label class="text-[10px] text-gray-500">Speed:</label>
                <input v-model.number="animationSpeed" type="range" min="50" max="1000" step="50" class="flex-1" />
                <span class="text-[10px] text-gray-500">{{ animationSpeed }}ms</span>
              </div>
            </div>

            <!-- Convergence Chart -->
            <div class="border rounded p-3 space-y-2">
              <h4 class="text-xs font-semibold text-[var(--text-secondary)]">Convergence</h4>
              <ConvergenceChart
                :data="chartData"
                :show-volume-constraint="true"
                :volume-constraint-value="optimizationObjective.targetVolumeFraction"
                :show-displacement-constraint="constraintType === 'volume_displacement'"
                :displacement-constraint-value="constraintType === 'volume_displacement' ? maxDisplacement : null"
                :height="200"
              />
            </div>
          </div>

          <div v-else class="text-center py-8">
            <p class="text-gray-400 text-sm">No results yet. Run optimization first.</p>
          </div>
        </div>
        </template>
      </div>

      <!-- Right: 3D Viewport -->
      <div class="flex-1 flex flex-col">
        <!-- Toolbar -->
        <div class="flex items-center gap-4 px-4 py-2 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2">
            <label class="text-xs text-[var(--text-muted)]">Display:</label>
            <select v-model="displayMode" class="px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]">
              <option value="density">Density</option>
              <option value="displacement">Displacement</option>
              <option value="stress">Stress</option>
            </select>
          </div>
          <div class="flex items-center gap-2">
            <label class="text-xs text-[var(--text-muted)]">Colormap:</label>
            <select v-model="colormap" class="px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]">
              <option value="viridis">Viridis</option>
              <option value="plasma">Plasma</option>
              <option value="inferno">Inferno</option>
              <option value="jet">Jet</option>
              <option value="rainbow">Rainbow</option>
            </select>
          </div>
          <div class="flex items-center gap-2">
            <label class="text-xs text-[var(--text-muted)]">Opacity:</label>
            <input v-model.number="opacity" type="range" min="0.1" max="1" step="0.1" class="w-20" />
          </div>
        </div>

        <!-- 3D Canvas -->
        <div ref="canvasContainer" class="flex-1"></div>

        <!-- Color Legend -->
        <div class="h-12 px-4 py-2 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)]">
          <div class="flex items-center justify-between h-full">
            <span class="text-xs text-[var(--text-muted)]">0.0</span>
            <div class="flex-1 mx-4 h-2 rounded" :style="{ background: 'linear-gradient(to right, #440154, #31688e, #35b779, #fde725)' }"></div>
            <span class="text-xs text-[var(--text-muted)]">1.0</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Result Dialog -->
    <div v-if="showResultDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-[var(--bg-surface)] rounded-lg p-6 w-[700px] max-h-[80vh] overflow-y-auto">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">Optimization Result</h3>

        <div v-if="optimizationResult" class="space-y-4">
          <!-- Convergence Chart in dialog -->
          <div class="border rounded p-4">
            <h4 class="text-sm font-semibold mb-2 text-[var(--text-primary)]">Convergence Chart</h4>
            <ConvergenceChart
              :data="chartData"
              :show-volume-constraint="true"
              :volume-constraint-value="optimizationObjective.targetVolumeFraction"
              :show-displacement-constraint="constraintType === 'volume_displacement'"
              :displacement-constraint-value="constraintType === 'volume_displacement' ? maxDisplacement : null"
              :height="220"
            />
          </div>

          <!-- Summary -->
          <div class="grid grid-cols-4 gap-4 text-center">
            <div class="bg-gray-50 dark:bg-gray-800 rounded p-3">
              <p class="text-2xl font-bold text-pink-600">{{ optimizationResult.iterations.length }}</p>
              <p class="text-xs text-gray-500">Iterations</p>
            </div>
            <div class="bg-gray-50 dark:bg-gray-800 rounded p-3">
              <p class="text-2xl font-bold text-blue-600">{{ (optimizationResult.final_volume * 100).toFixed(1) }}%</p>
              <p class="text-xs text-gray-500">Final Volume</p>
            </div>
            <div class="bg-gray-50 dark:bg-gray-800 rounded p-3">
              <p class="text-2xl font-bold text-green-600">{{ optimizationResult.final_objective.toFixed(2) }}</p>
              <p class="text-xs text-gray-500">Compliance</p>
            </div>
            <div class="bg-gray-50 dark:bg-gray-800 rounded p-3">
              <p class="text-2xl font-bold" :class="optimizationResult.iterations[optimizationResult.iterations.length - 1]?.converged ? 'text-green-600' : 'text-yellow-600'">
                {{ optimizationResult.iterations[optimizationResult.iterations.length - 1]?.converged ? 'Yes' : 'No' }}
              </p>
              <p class="text-xs text-gray-500">Converged</p>
            </div>
          </div>
        </div>

        <div class="flex justify-end gap-2 mt-6">
          <button @click="showResultDialog = false" class="px-4 py-2 border rounded text-sm hover:bg-gray-50 text-[var(--text-primary)]">
            Close
          </button>
          <button @click="exportSTL" class="px-4 py-2 bg-green-600 text-white rounded text-sm hover:bg-green-700">
            Export STL
          </button>
        </div>
      </div>
    </div>

    <!-- Embed Dialog -->
    <div v-if="showEmbedDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-[var(--bg-surface)] rounded-lg p-6 w-[400px]">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">Embed to Notes</h3>
        <p class="text-sm text-gray-600 mb-4">Results will be embedded into notes.</p>
        <div class="flex justify-end gap-2">
          <button @click="showEmbedDialog = false" class="px-4 py-2 border rounded text-sm hover:bg-gray-50 text-[var(--text-primary)]">Cancel</button>
          <button @click="showEmbedDialog = false" class="px-4 py-2 bg-purple-600 text-white rounded text-sm hover:bg-purple-700">Confirm</button>
        </div>
      </div>
    </div>

    <!-- Export Dialog -->
    <div v-if="showExportDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-[var(--bg-surface)] rounded-lg p-6 w-[400px]">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">Export Success</h3>
        <p class="text-sm text-gray-600 mb-4">STL file saved: topology_optimization.stl</p>
        <div class="flex justify-end">
          <button @click="showExportDialog = false" class="px-4 py-2 bg-green-600 text-white rounded text-sm hover:bg-green-700">OK</button>
        </div>
      </div>
    </div>
  </div>
</template>
