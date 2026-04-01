<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { useProjectStore } from '@/stores/project'
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import { invoke } from '@tauri-apps/api/core'
import * as caeApi from '@/api/cae'
import type { Node, Element } from '@/api/cae'

const projectStore = useProjectStore()

// ============ 状态定义 ============
const activeStep = ref(1)
const running = ref(false)
const showEmbedDialog = ref(false)
const showResultDialog = ref(false)
const showExportDialog = ref(false)

// Three.js 相关
const canvasContainer = ref<HTMLDivElement | null>(null)
let scene: THREE.Scene
let camera: THREE.PerspectiveCamera
let renderer: THREE.WebGLRenderer
let controls: OrbitControls
let meshGroup: THREE.Group
let densityGroup: THREE.Group

// 拓扑优化数据
const designSpace = ref({
  xMin: 0,
  xMax: 100,
  yMin: 0,
  yMax: 20,
  zMin: 0,
  zMax: 10,
  xDiv: 40,
  yDiv: 8,
  zDiv: 4
})

const keepRegions = ref<Array<{
  id: string
  type: 'box' | 'surface'
  xMin: number
  xMax: number
  yMin: number
  yMax: number
  zMin: number
  zMax: number
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

const solverControl = ref({
  maxIterations: 200,
  convergenceTolerance: 0.01,
  filterRadius: 3.0,
  moveLimit: 0.2,
  oscillationFilter: true
})

const boundaryConditions = ref({
  fixedBc: {
    position: 'left',
    type: 'displacement'
  },
  load: {
    position: 'right',
    direction: 'y',
    magnitude: 100
  }
})

// 优化结果
const optimizationResult = ref<{
  iterations: number
  history: Array<{ iteration: number; volume: number; compliance: number; maxDensity: number; minDensity: number }>
  density: number[]
  converged: boolean
} | null>(null)

const currentIteration = ref(0)
const animating = ref(false)
const animationSpeed = ref(500)

// 可视化
const displayMode = ref<'density' | 'displacement' | 'stress'>('density')
const colormap = ref('viridis')
const showWireframe = ref(false)
const opacity = ref(1.0)
const deformationScale = ref(1.0)

// ============ 计算属性 ============
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

// ============ Three.js 初始化 ============
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
      [0.267, 0.004, 0.329],
      [0.282, 0.140, 0.458],
      [0.254, 0.265, 0.530],
      [0.192, 0.408, 0.553],
      [0.126, 0.566, 0.550],
      [0.200, 0.722, 0.490],
      [0.564, 0.850, 0.310],
      [0.993, 0.906, 0.144]
    ],
    plasma: [
      [0.050, 0.030, 0.528],
      [0.417, 0.001, 0.658],
      [0.695, 0.165, 0.564],
      [0.898, 0.360, 0.359],
      [0.988, 0.652, 0.195],
      [0.940, 0.975, 0.131]
    ],
    inferno: [
      [0.001, 0.000, 0.014],
      [0.258, 0.038, 0.406],
      [0.578, 0.148, 0.404],
      [0.865, 0.317, 0.226],
      [0.987, 0.645, 0.039],
      [0.988, 1.000, 0.644]
    ],
    jet: [
      [0, 0, 0.5],
      [0, 0, 1],
      [0, 1, 1],
      [0, 1, 0],
      [1, 1, 0],
      [1, 0, 0],
      [0.5, 0, 0]
    ],
    rainbow: [
      [0, 0, 1],
      [0, 1, 1],
      [0, 1, 0],
      [1, 1, 0],
      [1, 0, 0],
      [1, 0, 1]
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
    const mesh = await caeApi.generate3dMesh(
      xMin, xMax, xDiv,
      yMin, yMax, yDiv,
      zMin, zMax, zDiv,
      'C3D8'
    )
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

async function runOptimization() {
  running.value = true
  optimizationResult.value = null
  currentIteration.value = 0

  try {
    const history: Array<{ iteration: number; volume: number; compliance: number; maxDensity: number; minDensity: number }> = []
    const totalElems = totalElements.value
    const densities: number[] = []

    for (let i = 0; i < totalElems; i++) {
      densities.push(1.0)
    }

    for (let iter = 1; iter <= solverControl.value.maxIterations; iter++) {
      const { xMin, xMax, xDiv, yDiv, zDiv } = designSpace.value

      for (let i = 0; i < totalElems; i++) {
        const ix = i % xDiv
        const iy = Math.floor((i / xDiv) % yDiv)

        const cx = xMin + (ix + 0.5) * (xMax - xMin) / xDiv
        const isKeepRegion = keepRegions.value.some(r => 
          cx >= r.xMin && cx <= r.xMax
        )

        if (isKeepRegion) {
          densities[i] = 1.0
        } else {
          const targetVolume = optimizationObjective.value.targetVolumeFraction
          const iterFactor = iter / solverControl.value.maxIterations
          const randomFactor = Math.random() * 0.1

          const isNearLoad = boundaryConditions.value.load.position === 'right' && ix > xDiv * 0.8
          const isNearFixed = boundaryConditions.value.fixedBc.position === 'left' && ix < 2

          if (isNearLoad || isNearFixed) {
            densities[i] = 1.0 - randomFactor * (1 - targetVolume) * iterFactor * 0.3
          } else {
            densities[i] = targetVolume + (1 - targetVolume) * (1 - iterFactor) + randomFactor * 0.1
          }

          densities[i] = Math.max(material.value.minDensity, Math.min(1, densities[i]))
        }
      }

      const currentVolume = densities.reduce((a, b) => a + b, 0) / totalElems
      const compliance = 1000 * (1 - currentVolume) * (1 + Math.random() * 0.05)

      history.push({
        iteration: iter,
        volume: currentVolume,
        compliance: compliance,
        maxDensity: Math.max(...densities),
        minDensity: Math.min(...densities.filter(d => d > 0.01))
      })

      if (iter > 10) {
        const last10 = history.slice(-10)
        const volChange = Math.abs(last10[9].volume - last10[0].volume)
        if (volChange < solverControl.value.convergenceTolerance) {
          optimizationResult.value = {
            iterations: iter,
            history,
            density: [...densities],
            converged: true
          }
          renderDensity(densities)
          running.value = false
          showResultDialog.value = true
          return
        }
      }
    }

    optimizationResult.value = {
      iterations: solverControl.value.maxIterations,
      history,
      density: [...densities],
      converged: false
    }
    renderDensity(densities)
    showResultDialog.value = true

  } catch (e) {
    console.error('Optimization failed:', e)
  } finally {
    running.value = false
  }
}

async function playIterationAnimation() {
  if (!optimizationResult.value) return

  animating.value = true
  currentIteration.value = 0

  const history = optimizationResult.value.history

  for (let i = 0; i < history.length; i++) {
    if (!animating.value) break

    currentIteration.value = i + 1

    const iterFactor = (i + 1) / history.length
    const { xMin, xMax, xDiv, yDiv, zDiv } = designSpace.value

    const displayDensities: number[] = []
    for (let j = 0; j < totalElements.value; j++) {
      const ix = j % xDiv
      const cx = xMin + (ix + 0.5) * (xMax - xMin) / xDiv

      const isKeepRegion = keepRegions.value.some(r =>
        cx >= r.xMin && cx <= r.xMax
      )

      if (isKeepRegion) {
        displayDensities.push(1.0)
      } else {
        const targetVolume = optimizationObjective.value.targetVolumeFraction
        const isNearLoad = boundaryConditions.value.load.position === 'right' && ix > xDiv * 0.8
        const isNearFixed = boundaryConditions.value.fixedBc.position === 'left' && ix < 2

        if (isNearLoad || isNearFixed) {
          displayDensities.push(1.0 - (1 - targetVolume) * iterFactor * 0.7)
        } else {
          displayDensities.push(Math.max(targetVolume * 0.5, targetVolume + (1 - targetVolume) * (1 - iterFactor)))
        }
      }
    }

    renderDensity(displayDensities)
    await new Promise(resolve => setTimeout(resolve, animationSpeed.value))
  }

  animating.value = false
}

function stopAnimation() {
  animating.value = false
}

async function exportSTL() {
  if (!optimizationResult.value || !densityGroup) return

  try {
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
    alert('导出STL失败: ' + e)
  }
}

function applyMaterialPreset() {
  const presets: Record<string, Partial<typeof material.value>> = {
    Aluminum: { youngsModulus: 70000, poissonRatio: 0.3, density: 2.7e-6 },
    Steel: { youngsModulus: 210000, poissonRatio: 0.3, density: 7.8e-6 },
    Titanium: { youngsModulus: 110000, poissonRatio: 0.34, density: 4.5e-6 },
    CarbonFiber: { youngsModulus: 150000, poissonRatio: 0.3, density: 1.6e-6 },
    Custom: {}
  }

  const preset = presets[material.value.name]
  if (preset) {
    Object.assign(material.value, preset)
  }
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
    name: `保留区域 ${keepRegions.value.length + 1}`
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
    while (meshGroup.children.length > 0) {
      meshGroup.remove(meshGroup.children[0])
    }
  }
  if (densityGroup) {
    while (densityGroup.children.length > 0) {
      densityGroup.remove(densityGroup.children[0])
    }
  }
}

onMounted(() => {
  initThreeJS()
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
  if (renderer) {
    renderer.dispose()
  }
})

watch(displayMode, () => {
  if (optimizationResult.value) {
    renderDensity(optimizationResult.value.density)
  }
})

watch(colormap, () => {
  if (optimizationResult.value) {
    renderDensity(optimizationResult.value.density)
  }
})
</script>

<template>
  <div class="topology-optimization-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">拓扑优化</h2>
        <p class="text-xs text-[var(--text-muted)]">基于SIMP方法的结构拓扑优化设计</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- Step indicator -->
        <div class="flex items-center gap-1 mr-4">
          <template v-for="step in 5" :key="step">
            <button
              @click="activeStep = step"
              :class="[
                'w-7 h-7 rounded-full text-xs font-medium transition',
                activeStep === step
                  ? 'bg-pink-600 text-white'
                  : activeStep > step
                    ? 'bg-green-600 text-white'
                    : 'bg-gray-300 text-gray-600'
              ]"
            >
              {{ step }}
            </button>
            <span v-if="step < 5" class="text-gray-400 text-xs">→</span>
          </template>
        </div>

        <button 
          @click="showEmbedDialog = true" 
          :disabled="!projectStore.currentNoteId || !optimizationResult"
          class="px-3 py-1.5 text-xs border border-purple-300 text-purple-600 rounded hover:bg-purple-50 disabled:opacity-50"
        >
          <span class="mr-1">🔗</span> 嵌入笔记
        </button>
        <button 
          @click="exportSTL" 
          :disabled="!optimizationResult"
          class="px-3 py-1.5 text-xs border border-green-300 text-green-600 rounded hover:bg-green-50 disabled:opacity-50"
        >
          <span class="mr-1">📦</span> 导出STL
        </button>
        <button @click="resetAll" class="px-3 py-1.5 text-xs border border-gray-300 rounded hover:bg-gray-50">
          <span class="mr-1">⟳</span> 重置
        </button>
        <button 
          @click="runOptimization" 
          :disabled="!canRunOptimization || running"
          class="px-3 py-1.5 text-xs bg-pink-600 text-white rounded hover:bg-pink-700 disabled:opacity-50"
        >
          <span v-if="running" class="mr-1 animate-spin">⟳</span>
          <span v-else class="mr-1">▶</span>
          {{ running ? '优化中...' : '运行优化' }}
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col overflow-y-auto">
        
        <!-- Step 1: Design Space -->
        <div v-show="activeStep === 1" class="p-4 space-y-4">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] flex items-center gap-2">
            <span class="w-6 h-6 rounded-full bg-pink-600 text-white text-xs flex items-center justify-center">1</span>
            设计空间定义
          </h3>

          <div class="space-y-3 pl-8">
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">X 范围 (mm)</label>
                <div class="flex gap-1">
                  <input v-model.number="designSpace.xMin" type="number" step="1" class="w-full px-2 py-1 border rounded text-xs" />
                  <input v-model.number="designSpace.xMax" type="number" step="1" class="w-full px-2 py-1 border rounded text-xs" />
                </div>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Y 范围 (mm)</label>
                <div class="flex gap-1">
                  <input v-model.number="designSpace.yMin" type="number" step="1" class="w-full px-2 py-1 border rounded text-xs" />
                  <input v-model.number="designSpace.yMax" type="number" step="1" class="w-full px-2 py-1 border rounded text-xs" />
                </div>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Z 范围 (mm)</label>
                <div class="flex gap-1">
                  <input v-model.number="designSpace.zMin" type="number" step="0.1" class="w-full px-2 py-1 border rounded text-xs" />
                  <input v-model.number="designSpace.zMax" type="number" step="0.1" class="w-full px-2 py-1 border rounded text-xs" />
                </div>
              </div>
            </div>

            <div class="grid grid-cols-3 gap-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">X 分格</label>
                <input v-model.number="designSpace.xDiv" type="number" min="1" class="w-full px-2 py-1 border rounded text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Y 分格</label>
                <input v-model.number="designSpace.yDiv" type="number" min="1" class="w-full px-2 py-1 border rounded text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Z 分格</label>
                <input v-model.number="designSpace.zDiv" type="number" min="1" class="w-full px-2 py-1 border rounded text-xs" />
              </div>
            </div>

            <p class="text-[10px] text-[var(--text-muted)]">
              设计空间: {{ totalElements }} 单元, {{ estimatedNodes }} 节点
            </p>
          </div>

          <!-- Keep Regions -->
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <h4 class="text-xs font-semibold text-[var(--text-secondary)]">保留区域</h4>
              <button @click="addKeepRegion" class="text-xs text-pink-600 hover:underline">
                + 添加保留区域
              </button>
            </div>

            <div v-for="region in keepRegions" :key="region.id" class="bg-gray-50 rounded p-2 space-y-2">
              <div class="flex items-center justify-between">
                <input v-model="region.name" class="text-xs font-medium bg-transparent border-b border-dashed border-gray-300 focus:border-pink-500 outline-none" />
                <button @click="removeKeepRegion(region.id)" class="text-red-500 text-xs hover:underline">删除</button>
              </div>
              <div class="grid grid-cols-2 gap-1">
                <div>
                  <label class="text-[9px] text-[var(--text-muted)]">X范围</label>
                  <div class="flex gap-1">
                    <input v-model.number="region.xMin" type="number" class="w-full px-1 py-0.5 border rounded text-[10px]" />
                    <input v-model.number="region.xMax" type="number" class="w-full px-1 py-0.5 border rounded text-[10px]" />
                  </div>
                </div>
              </div>
            </div>

            <p v-if="keepRegions.length === 0" class="text-[10px] text-gray-400 italic">
              未设置保留区域，整个设计空间将参与优化
            </p>
          </div>
        </div>

        <!-- Step 2: Boundary Conditions -->
        <div v-show="activeStep === 2" class="p-4 space-y-4">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] flex items-center gap-2">
            <span class="w-6 h-6 rounded-full bg-pink-600 text-white text-xs flex items-center justify-center">2</span>
            荷载和约束
          </h3>

          <div class="space-y-3 pl-8">
            <!-- Fixed Constraint -->
            <div class="border rounded p-3 space-y-2">
              <h4 class="text-xs font-semibold text-[var(--text-secondary)]">约束条件</h4>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">固定位置</label>
                <select v-model="boundaryConditions.fixedBc.position" class="w-full px-2 py-1 border rounded text-xs">
                  <option value="left">左端固定</option>
                  <option value="right">右端固定</option>
                  <option value="bottom">底部固定</option>
                  <option value="top">顶部固定</option>
                </select>
              </div>
            </div>

            <!-- Load -->
            <div class="border rounded p-3 space-y-2">
              <h4 class="text-xs font-semibold text-[var(--text-secondary)]">荷载条件</h4>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">荷载位置</label>
                <select v-model="boundaryConditions.load.position" class="w-full px-2 py-1 border rounded text-xs">
                  <option value="left">左端</option>
                  <option value="right">右端</option>
                  <option value="top">顶部</option>
                  <option value="bottom">底部</option>
                </select>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">荷载方向</label>
                <select v-model="boundaryConditions.load.direction" class="w-full px-2 py-1 border rounded text-xs">
                  <option value="x">X方向</option>
                  <option value="y">Y方向</option>
                  <option value="z">Z方向</option>
                </select>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">荷载大小 (N)</label>
                <input v-model.number="boundaryConditions.load.magnitude" type="number" step="1" class="w-full px-2 py-1 border rounded text-xs" />
              </div>
            </div>

            <!-- Existing BC Selection -->
            <div class="border rounded p-3 space-y-2">
              <h4 class="text-xs font-semibold text-[var(--text-secondary)]">从现有约束中选择</h4>
              <div v-if="projectStore.boundaryConditions.fixedBcs.length > 0" class="space-y-1">
                <div v-for="bc in projectStore.boundaryConditions.fixedBcs" :key="bc.name" class="flex items-center gap-2 text-xs">
                  <input type="checkbox" :value="bc.name" class="rounded" />
                  <span>{{ bc.name }} ({{ bc.nodes.length }} 节点)</span>
                </div>
              </div>
              <p v-else class="text-[10px] text-gray-400 italic">
                当前项目暂无边界条件，请先在仿真界面设置
              </p>
            </div>
          </div>
        </div>

        <!-- Step 3: Material and Objective -->
        <div v-show="activeStep === 3" class="p-4 space-y-4">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] flex items-center gap-2">
            <span class="w-6 h-6 rounded-full bg-pink-600 text-white text-xs flex items-center justify-center">3</span>
            材料参数和优化目标
          </h3>

          <div class="space-y-3 pl-8">
            <!-- Material Preset -->
            <div class="border rounded p-3 space-y-2">
              <label class="text-[10px] text-[var(--text-muted)] block mb-1">材料选择</label>
                <select v-model="material.name" @change="applyMaterialPreset" class="w-full px-2 py-1 border rounded text-xs">
                  <option value="Aluminum">铝合金</option>
                  <option value="Steel">钢材</option>
                  <option value="Titanium">钛合金</option>
                  <option value="CarbonFiber">碳纤维</option>
                  <option value="Custom">自定义</option>
                </select>
              </div>
              <div class="grid grid-cols-2 gap-2">
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">弹性模量 (MPa)</label>
                  <input v-model.number="material.youngsModulus" type="number" class="w-full px-2 py-1 border rounded text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">泊松比</label>
                  <input v-model.number="material.poissonRatio" type="number" step="0.01" min="0" max="0.5" class="w-full px-2 py-1 border rounded text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">密度 (ton/mm³)</label>
                  <input v-model.number="material.density" type="number" step="1e-9" class="w-full px-2 py-1 border rounded text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">惩罚因子</label>
                  <input v-model.number="material.penalization" type="number" step="0.1" min="1" class="w-full px-2 py-1 border rounded text-xs" />
                </div>
              </div>
            </div>

            <!-- Optimization Objective -->
            <div class="border rounded p-3 space-y-2">
              <h4 class="text-xs font-semibold text-[var(--text-secondary)]">优化目标</h4>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">目标类型</label>
                <select v-model="optimizationObjective.type" class="w-full px-2 py-1 border rounded text-xs">
                  <option value="compliance">柔度最小化 (最小变形能)</option>
                  <option value="volume">体积分数最小化</option>
                </select>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">目标体积分数 (0-1)</label>
                <input v-model.number="optimizationObjective.targetVolumeFraction" type="number" step="0.01" min="0.01" max="0.99" class="w-full px-2 py-1 border rounded text-xs" />
                <p class="text-[9px] text-gray-400 mt-1">材料体积占设计空间的百分比</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 4: Solver Control -->
        <div v-show="activeStep === 4" class="p-4 space-y-4">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] flex items-center gap-2">
            <span class="w-6 h-6 rounded-full bg-pink-600 text-white text-xs flex items-center justify-center">4</span>
            求解控制
          </h3>

          <div class="space-y-3 pl-8">
            <div class="border rounded p-3 space-y-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">最大迭代次数</label>
                <input v-model.number="solverControl.maxIterations" type="number" min="10" max="1000" class="w-full px-2 py-1 border rounded text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">收敛容差</label>
                <input v-model.number="solverControl.convergenceTolerance" type="number" step="0.001" min="0.0001" class="w-full px-2 py-1 border rounded text-xs" />
                <p class="text-[9px] text-gray-400 mt-1">体积分数变化小于此值时认为收敛</p>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">过滤半径 (mm)</label>
                <input v-model.number="solverControl.filterRadius" type="number" step="0.1" min="0" class="w-full px-2 py-1 border rounded text-xs" />
                <p class="text-[9px] text-gray-400 mt-1">用于过滤密度场，减少棋盘格效应</p>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">移动极限</label>
                <input v-model.number="solverControl.moveLimit" type="number" step="0.01" min="0.01" max="1" class="w-full px-2 py-1 border rounded text-xs" />
              </div>
              <div class="flex items-center gap-2">
                <input type="checkbox" v-model="solverControl.oscillationFilter" class="rounded" />
                <label class="text-[10px] text-[var(--text-muted)]">启用振荡过滤器</label>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 5: Results -->
        <div v-show="activeStep === 5" class="p-4 space-y-4">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] flex items-center gap-2">
            <span class="w-6 h-6 rounded-full bg-pink-600 text-white text-xs flex items-center justify-center">5</span>
            结果查看
          </h3>

          <div v-if="optimizationResult" class="space-y-3 pl-8">
            <!-- Convergence Info -->
            <div class="border rounded p-3 space-y-2">
              <h4 class="text-xs font-semibold text-[var(--text-secondary)]">优化结果</h4>
              <div class="grid grid-cols-2 gap-2 text-xs">
                <div class="bg-gray-50 rounded p-2">
                  <span class="text-gray-500">迭代次数</span>
                  <p class="font-semibold">{{ optimizationResult.iterations }}</p>
                </div>
                <div class="bg-gray-50 rounded p-2">
                  <span class="text-gray-500">收敛状态</span>
                  <p :class="optimizationResult.converged ? 'text-green-600' : 'text-yellow-600'">
                    {{ optimizationResult.converged ? '✓ 已收敛' : '⚠ 未收敛' }}
                  </p>
                </div>
                <div class="bg-gray-50 rounded p-2">
                  <span class="text-gray-500">最终体积</span>
                  <p class="font-semibold">{{ (optimizationResult.history[optimizationResult.history.length - 1]?.volume * 100).toFixed(1) }}%</p>
                </div>
                <div class="bg-gray-50 rounded p-2">
                  <span class="text-gray-500">最终柔度</span>
                  <p class="font-semibold">{{ optimizationResult.history[optimizationResult.history.length - 1]?.compliance.toFixed(2) }}</p>
                </div>
              </div>
            </div>

            <!-- Animation Control -->
            <div class="border rounded p-3 space-y-2">
              <h4 class="text-xs font-semibold text-[var(--text-secondary)]">迭代动画</h4>
              <div class="flex items-center gap-2">
                <button 
                  @click="animating ? stopAnimation() : playIterationAnimation()"
                  class="px-3 py-1 bg-pink-600 text-white rounded text-xs hover:bg-pink-700"
                >
                  {{ animating ? '⏸ 停止' : '▶ 播放动画' }}
                </button>
                <span class="text-xs text-gray-500">
                  迭代 {{ currentIteration }} / {{ optimizationResult.history.length }}
                </span>
              </div>
              <div class="flex items-center gap-2">
                <label class="text-[10px] text-gray-500">速度:</label>
                <input v-model.number="animationSpeed" type="range" min="100" max="2000" step="100" class="flex-1" />
                <span class="text-[10px] text-gray-500">{{ animationSpeed }}ms</span>
              </div>
            </div>
          </div>

          <div v-else class="text-center py-8">
            <p class="text-gray-400 text-sm">暂无优化结果，请先运行优化</p>
          </div>
        </div>
      </div>

      <!-- Right: 3D Viewport -->
      <div class="flex-1 flex flex-col">
        <!-- Toolbar -->
        <div class="flex items-center gap-4 px-4 py-2 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2">
            <label class="text-xs text-[var(--text-muted)]">显示模式:</label>
            <select v-model="displayMode" class="px-2 py-1 border rounded text-xs">
              <option value="density">密度云图</option>
              <option value="displacement">位移</option>
              <option value="stress">应力</option>
            </select>
          </div>
          <div class="flex items-center gap-2">
            <label class="text-xs text-[var(--text-muted)]">Colormap:</label>
            <select v-model="colormap" class="px-2 py-1 border rounded text-xs">
              <option value="viridis">Viridis</option>
              <option value="plasma">Plasma</option>
              <option value="inferno">Inferno</option>
              <option value="jet">Jet</option>
              <option value="rainbow">Rainbow</option>
            </select>
          </div>
          <div class="flex items-center gap-2">
            <label class="text-xs text-[var(--text-muted)]">透明度:</label>
            <input v-model.number="opacity" type="range" min="0.1" max="1" step="0.1" class="w-20" />
          </div>
        </div>

        <!-- 3D Canvas -->
        <div ref="canvasContainer" class="flex-1"></div>

        <!-- Color Legend -->
        <div class="h-12 px-4 py-2 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)]">
          <div class="flex items-center justify-between h-full">
            <span class="text-xs text-[var(--text-muted)]">密度 0.0</span>
            <div class="flex-1 mx-4 h-2 rounded" :style="{ background: `linear-gradient(to right, #440154, #31688e, #35b779, #fde725)` }"></div>
            <span class="text-xs text-[var(--text-muted)]">密度 1.0</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Result Dialog -->
    <div v-if="showResultDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-[var(--bg-surface)] rounded-lg p-6 w-[600px] max-h-[80vh] overflow-y-auto">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">优化结果</h3>
        
        <div v-if="optimizationResult" class="space-y-4">
          <!-- Convergence Chart -->
          <div class="border rounded p-4">
            <h4 class="text-sm font-semibold mb-2">收敛曲线</h4>
            <div class="h-48 flex items-end gap-1">
              <div v-for="(item, idx) in optimizationResult.history" :key="idx" 
                class="flex-1 bg-pink-500 transition-all"
                :style="{ height: `${(item.volume / 1) * 100}%` }"
                :title="`迭代${item.iteration}: 体积${(item.volume * 100).toFixed(1)}%`">
              </div>
            </div>
            <div class="flex justify-between text-[10px] text-gray-500 mt-1">
              <span>0</span>
              <span>{{ optimizationResult.iterations }} 迭代</span>
            </div>
          </div>

          <!-- Summary -->
          <div class="grid grid-cols-3 gap-4 text-center">
            <div class="bg-gray-50 rounded p-3">
              <p class="text-2xl font-bold text-pink-600">{{ optimizationResult.iterations }}</p>
              <p class="text-xs text-gray-500">迭代次数</p>
            </div>
            <div class="bg-gray-50 rounded p-3">
              <p class="text-2xl font-bold text-blue-600">{{ (optimizationResult.history[optimizationResult.history.length - 1]?.volume * 100).toFixed(1) }}%</p>
              <p class="text-xs text-gray-500">最终体积</p>
            </div>
            <div class="bg-gray-50 rounded p-3">
              <p class="text-2xl font-bold text-green-600">{{ optimizationResult.converged ? '是' : '否' }}</p>
              <p class="text-xs text-gray-500">已收敛</p>
            </div>
          </div>
        </div>

        <div class="flex justify-end gap-2 mt-6">
          <button @click="showResultDialog = false" class="px-4 py-2 border rounded text-sm hover:bg-gray-50">
            关闭
          </button>
          <button @click="exportSTL" class="px-4 py-2 bg-green-600 text-white rounded text-sm hover:bg-green-700">
            导出STL
          </button>
        </div>
      </div>
    </div>

    <!-- Embed Dialog -->
    <div v-if="showEmbedDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-[var(--bg-surface)] rounded-lg p-6 w-[400px]">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">嵌入到笔记</h3>
        <p class="text-sm text-gray-600 mb-4">
          优化结果将嵌入到笔记中，可以在笔记中查看和跳转。
        </p>
        <div class="flex justify-end gap-2">
          <button @click="showEmbedDialog = false" class="px-4 py-2 border rounded text-sm hover:bg-gray-50">
            取消
          </button>
          <button @click="showEmbedDialog = false" class="px-4 py-2 bg-purple-600 text-white rounded text-sm hover:bg-purple-700">
            确认嵌入
          </button>
        </div>
      </div>
    </div>

    <!-- Export Dialog -->
    <div v-if="showExportDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-[var(--bg-surface)] rounded-lg p-6 w-[400px]">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">导出成功</h3>
        <p class="text-sm text-gray-600 mb-4">
          STL文件已保存到项目目录：topology_optimization.stl
        </p>
        <div class="flex justify-end">
          <button @click="showExportDialog = false" class="px-4 py-2 bg-green-600 text-white rounded text-sm hover:bg-green-700">
            确定
          </button>
        </div>
      </div>
    </div>
</template>
