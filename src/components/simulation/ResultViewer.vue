<template>
  <div class="relative w-full h-full" ref="containerRef">
    <canvas ref="canvasRef" class="w-full h-full"></canvas>
    <!-- Loading overlay -->
    <div v-if="loading" class="absolute inset-0 bg-white/80 flex items-center justify-center">
      <div class="text-gray-500">加载中...</div>
    </div>

    <!-- 剖切面控制 -->
    <div v-if="result" class="absolute top-2 right-2 bg-white/90 dark:bg-gray-800/90 rounded-lg shadow-lg p-2 text-xs space-y-1.5 z-10">
      <label class="flex items-center gap-1.5">
        <input type="checkbox" v-model="clippingOn" />
        <span>剖切面</span>
      </label>
      <template v-if="clippingOn">
        <!-- 方向选择 -->
        <div class="flex gap-1">
          <button v-for="dir in (['XY', 'XZ', 'YZ'] as const)" :key="dir"
                  @click="clipDirection = dir as 'XY' | 'XZ' | 'YZ'"
                  :class="['px-1.5 py-0.5 rounded', clipDirection === dir ? 'bg-blue-500 text-white' : 'bg-gray-100 dark:bg-gray-700 dark:text-gray-200']">
            {{ dir }}
          </button>
        </div>
        <!-- 位置滑块 -->
        <div>
          <span>位置: {{ clipPosition.toFixed(2) }}</span>
          <input type="range" :min="clipMin" :max="clipMax" :step="0.01" v-model.number="clipPosition" class="w-full" />
        </div>
      </template>
    </div>

    <!-- 动画控制栏 -->
    <div v-if="animationFrames && animationFrames.length > 1"
         class="absolute bottom-2 left-1/2 -translate-x-1/2 bg-white/90 dark:bg-gray-800/90 rounded-lg shadow-lg px-3 py-2 flex items-center gap-2 z-10">
      <!-- 播放/暂停 -->
      <button @click="isPlaying ? pauseAnimation() : playAnimation()"
              class="w-6 h-6 flex items-center justify-center rounded bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 text-xs">
        {{ isPlaying ? '⏸' : '▶' }}
      </button>
      <!-- 进度条 -->
      <input type="range" :min="0" :max="animationFrames.length - 1" :step="1"
             v-model.number="currentFrame" class="w-32" />
      <!-- 帧信息 -->
      <span class="text-[10px] text-[var(--text-secondary)] whitespace-nowrap">
        {{ currentFrame + 1 }}/{{ animationFrames.length }}
        t={{ animationFrames[currentFrame]?.time?.toFixed(3) }}s
      </span>
      <!-- 速度控制 -->
      <select v-model.number="playbackSpeed" class="text-[10px] bg-transparent dark:text-gray-200">
        <option :value="0.25">0.25x</option>
        <option :value="0.5">0.5x</option>
        <option :value="1">1x</option>
        <option :value="2">2x</option>
      </select>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from 'vue'
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'

// 移动端检测
function isMobileDevice(): boolean {
  if (typeof window === 'undefined') return false
  return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)
    || window.innerWidth < 768
}

interface SimulationResult {
  nodes: Array<{ id: number; x: number; y: number; z: number }>
  elements: Array<{ id: number; type: string; nodeIds: number[] }>
  displacement?: { step: number; data: Record<string, Record<string, number>> }
  stress?: { step: number; data: Record<string, Record<string, number>> }
  vonMises?: { step: number; data: Record<string, number> }
  deformationScale?: number
}

interface ModeShapeData {
  node_id: number
  x: number
  y: number
  z: number
  ux: number
  uy: number
  uz: number
  magnitude: number
}

interface AnimationFrame {
  step: number
  time: number
  node_values: Array<{ node_id: number; value: number }>
  displacement_data?: Record<string, Record<string, number>>
}

const props = withDefaults(defineProps<{
  result?: SimulationResult | null
  modeShape?: ModeShapeData[] | null  // For modal animation
  densityField?: number[] | null  // For topology optimization density visualization
  displayMode?: 'displacement' | 'stress' | 'vonMises'
  showDeformed?: boolean
  deformationScale?: number
  colormap?: 'viridis' | 'plasma' | 'inferno' | 'jet' | 'rainbow'
  showWireframe?: boolean
  clippingEnabled?: boolean
  clippingPlane?: { normal: [number, number, number]; constant: number }
  animationFrames?: AnimationFrame[]
}>(), {
  displayMode: 'vonMises',
  showDeformed: false,
  deformationScale: 1.0,
  colormap: 'viridis',
  showWireframe: false,
  modeShape: null,
  densityField: null,
  clippingEnabled: false,
  clippingPlane: undefined,
  animationFrames: undefined
})

const emit = defineEmits<{
  (e: 'nodeClick', nodeId: number, value: number): void
  (e: 'ready'): void
}>()

const containerRef = ref<HTMLDivElement>()
const canvasRef = ref<HTMLCanvasElement>()
const loading = ref(true)

// Three.js objects
let scene: THREE.Scene
let camera: THREE.PerspectiveCamera
let renderer: THREE.WebGLRenderer
let controls: OrbitControls
let mesh: THREE.Mesh
// eslint-disable-next-line @typescript-eslint/no-explicit-any
let wireframeMesh: any = null

// ---- Clipping plane state ----
const clippingOn = ref(false)
const clipDirection = ref<'XY' | 'XZ' | 'YZ'>('XZ')
const clipPosition = ref(0)
let clippingPlaneObj: THREE.Plane | null = null
let planeHelperObj: THREE.PlaneHelper | null = null
let planeVisualObj: THREE.Mesh | null = null

// Clipping slider bounds (updated based on model geometry)
const clipMin = ref(-5)
const clipMax = ref(5)

// ---- Animation state ----
const isPlaying = ref(false)
const currentFrame = ref(0)
const playbackSpeed = ref(1.0)
let animationTimer: number | null = null

// Colormap definitions (RGB values 0-255)
const colormaps: Record<string, Array<[number, number, number]>> = {
  viridis: [
    [68, 1, 84], [72, 35, 116], [64, 67, 135], [52, 94, 141],
    [41, 120, 142], [32, 145, 140], [34, 167, 132], [68, 190, 112],
    [121, 209, 81], [181, 222, 43], [253, 231, 37]
  ],
  plasma: [
    [13, 8, 135], [75, 3, 161], [126, 0, 180], [160, 9, 173],
    [186, 30, 151], [209, 60, 120], [222, 93, 84], [232, 121, 50],
    [241, 153, 23], [249, 189, 15], [252, 229, 33]
  ],
  inferno: [
    [0, 0, 4], [40, 11, 84], [101, 21, 110], [147, 34, 99],
    [189, 55, 68], [219, 83, 39], [244, 117, 16], [253, 159, 6],
    [252, 202, 21], [252, 249, 44], [252, 255, 163]
  ],
  jet: [
    [0, 0, 131], [0, 0, 255], [0, 63, 255], [0, 127, 255],
    [0, 191, 255], [63, 255, 191], [127, 255, 127], [191, 255, 63],
    [255, 255, 0], [255, 127, 0], [255, 0, 0], [127, 0, 0]
  ],
  rainbow: [
    [255, 0, 0], [255, 127, 0], [255, 255, 0], [0, 255, 0],
    [0, 255, 255], [0, 0, 255], [127, 0, 255]
  ]
}

function interpolateColor(value: number, colormapName: string): THREE.Color {
  const cmap = colormaps[colormapName] || colormaps.viridis
  const idx = Math.min(Math.floor(value * (cmap.length - 1)), cmap.length - 2)
  const t = (value * (cmap.length - 1)) - idx
  const c1 = cmap[idx]
  const c2 = cmap[idx + 1]
  return new THREE.Color(
    (c1[0] + (c2[0] - c1[0]) * t) / 255,
    (c1[1] + (c2[1] - c1[1]) * t) / 255,
    (c1[2] + (c2[2] - c1[2]) * t) / 255
  )
}

// ---- Clipping plane helpers ----

function getDirectionNormal(dir: 'XY' | 'XZ' | 'YZ'): THREE.Vector3 {
  switch (dir) {
    case 'XY': return new THREE.Vector3(0, 0, -1)
    case 'XZ': return new THREE.Vector3(0, -1, 0)
    case 'YZ': return new THREE.Vector3(-1, 0, 0)
  }
}

function computeModelBounds(result: SimulationResult): { min: THREE.Vector3; max: THREE.Vector3 } {
  const min = new THREE.Vector3(Infinity, Infinity, Infinity)
  const max = new THREE.Vector3(-Infinity, -Infinity, -Infinity)
  result.nodes.forEach(n => {
    min.x = Math.min(min.x, n.x)
    min.y = Math.min(min.y, n.y)
    min.z = Math.min(min.z, n.z)
    max.x = Math.max(max.x, n.x)
    max.y = Math.max(max.y, n.y)
    max.z = Math.max(max.z, n.z)
  })
  return { min, max }
}

function updateClippingPlane() {
  if (!clippingPlaneObj || !props.result) return

  const normal = getDirectionNormal(clipDirection.value)
  clippingPlaneObj.normal.copy(normal)
  clippingPlaneObj.constant = clipPosition.value

  // Update plane helper
  if (planeHelperObj) {
    scene.remove(planeHelperObj)
    planeHelperObj.dispose()
  }

  if (clippingOn.value) {
    planeHelperObj = new THREE.PlaneHelper(clippingPlaneObj, 10, 0x4F8CFF)
    scene.add(planeHelperObj)

    // Update semi-transparent plane visual
    updatePlaneVisual()
  } else {
    planeHelperObj = null
    removePlaneVisual()
  }

  // Rebuild mesh to apply/remove clipping planes
  updateMesh(props.result)
}

function updatePlaneVisual() {
  removePlaneVisual()

  if (!clippingOn.value || !clippingPlaneObj) return

  const size = 10
  const planeGeo = new THREE.PlaneGeometry(size, size)
  const planeMat = new THREE.MeshBasicMaterial({
    color: 0x4F8CFF,
    transparent: true,
    opacity: 0.08,
    side: THREE.DoubleSide,
    depthWrite: false
  })
  planeVisualObj = new THREE.Mesh(planeGeo, planeMat)

  // Orient the visual plane to match the clipping plane
  const normal = clippingPlaneObj.normal.clone().normalize()
  const up = new THREE.Vector3(0, 1, 0)
  const quaternion = new THREE.Quaternion()
  quaternion.setFromUnitVectors(new THREE.Vector3(0, 0, 1), normal)
  planeVisualObj.quaternion.copy(quaternion)

  // Position the visual plane at the clipping plane location
  const planePoint = normal.clone().multiplyScalar(-clippingPlaneObj.constant)
  planeVisualObj.position.copy(planePoint)

  scene.add(planeVisualObj)
}

function removePlaneVisual() {
  if (planeVisualObj) {
    scene.remove(planeVisualObj)
    planeVisualObj.geometry.dispose()
    ;(planeVisualObj.material as THREE.Material).dispose()
    planeVisualObj = null
  }
}

function getClippingPlanes(): THREE.Plane[] {
  if (!clippingOn.value || !clippingPlaneObj) return []
  return [clippingPlaneObj]
}

// ---- Animation helpers ----

function playAnimation() {
  if (!props.animationFrames || props.animationFrames.length === 0) return
  isPlaying.value = true
  const interval = 1000 / playbackSpeed.value // ms per frame

  animationTimer = window.setInterval(() => {
    currentFrame.value++
    if (currentFrame.value >= props.animationFrames!.length) {
      currentFrame.value = 0 // loop
    }
    updateMeshForFrame(currentFrame.value)
  }, interval)
}

function pauseAnimation() {
  isPlaying.value = false
  if (animationTimer) {
    clearInterval(animationTimer)
    animationTimer = null
  }
}

function updateMeshForFrame(frameIndex: number) {
  const frame = props.animationFrames?.[frameIndex]
  if (!frame || !props.result) return

  const result = props.result

  // Build node map
  const nodeMap = new Map<number, THREE.Vector3>()
  result.nodes.forEach(n => {
    nodeMap.set(n.id, new THREE.Vector3(n.x, n.y, n.z))
  })

  // Build node value map from animation frame
  const frameNodeValues = new Map<number, number>()
  frame.node_values.forEach(nv => {
    frameNodeValues.set(nv.node_id, nv.value)
  })

  // Build geometry
  const positions: number[] = []
  const normals: number[] = []
  const indices: number[] = []
  let idx = 0
  const nodeIndexMap = new Map<number, number>()

  result.elements.forEach(el => {
    const nodeIds = el.nodeIds
    const positionsLocal: THREE.Vector3[] = []

    nodeIds.forEach(nid => {
      if (!nodeIndexMap.has(nid)) {
        nodeIndexMap.set(nid, idx++)
        positionsLocal.push(nodeMap.get(nid) || new THREE.Vector3())
      }
    })

    // Apply displacement from animation frame if available
    if (frame.displacement_data) {
      positionsLocal.forEach((pos, localIdx) => {
        const nid = nodeIds[localIdx]
        const ux = (frame.displacement_data!['ux']?.[nid] || 0) * props.deformationScale
        const uy = (frame.displacement_data!['uy']?.[nid] || 0) * props.deformationScale
        const uz = (frame.displacement_data!['uz']?.[nid] || 0) * props.deformationScale
        pos.x += ux
        pos.y += uy
        pos.z += uz
      })
    }

    // Triangulate based on element type
    if (el.type === 'TET4' || el.type === 'TETRA') {
      const faces = [
        [0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]
      ]
      faces.forEach(f => {
        const a = positionsLocal[f[0]]
        const b = positionsLocal[f[1]]
        const c = positionsLocal[f[2]]

        const ab = new THREE.Vector3().subVectors(b, a)
        const ac = new THREE.Vector3().subVectors(c, a)
        const normal = new THREE.Vector3().crossVectors(ab, ac).normalize()

        ;[a, b, c].forEach(v => {
          positions.push(v.x, v.y, v.z)
          normals.push(normal.x, normal.y, normal.z)
        })
        indices.push(idx - 3, idx - 2, idx - 1)
        idx += 3
      })
    } else if (el.type === 'HEX8' || el.type === 'BRICK') {
      const v = positionsLocal
      const faces = [
        [0, 1, 2], [0, 2, 3],
        [4, 6, 5], [4, 7, 6],
        [0, 3, 7], [0, 7, 4],
        [1, 5, 6], [1, 6, 2],
        [0, 4, 5], [0, 5, 1],
        [3, 2, 6], [3, 6, 7]
      ]
      faces.forEach(fi => {
        const a = v[fi[0]]
        const b = v[fi[1]]
        const c = v[fi[2]]

        const ab = new THREE.Vector3().subVectors(b, a)
        const ac = new THREE.Vector3().subVectors(c, a)
        const normal = new THREE.Vector3().crossVectors(ab, ac).normalize()

        ;[a, b, c].forEach(v => {
          positions.push(v.x, v.y, v.z)
          normals.push(normal.x, normal.y, normal.z)
        })
        indices.push(idx, idx + 1, idx + 2)
        idx += 3
      })
    } else {
      for (let i = 1; i < nodeIds.length - 1; i++) {
        const a = positionsLocal[0]
        const b = positionsLocal[i]
        const c = positionsLocal[i + 1]

        const ab = new THREE.Vector3().subVectors(b, a)
        const ac = new THREE.Vector3().subVectors(c, a)
        const normal = new THREE.Vector3().crossVectors(ab, ac).normalize()

        ;[a, b, c].forEach(v => {
          positions.push(v.x, v.y, v.z)
          normals.push(normal.x, normal.y, normal.z)
        })
        indices.push(idx, idx + 1, idx + 2)
        idx += 3
      }
    }
  })

  // Compute vertex values from animation frame
  const vertexValues = new Map<number, number>()
  result.nodes.forEach(n => {
    const mapIdx = nodeIndexMap.get(n.id)
    if (mapIdx !== undefined) {
      vertexValues.set(mapIdx, frameNodeValues.get(n.id) || 0)
    }
  })

  // Find min/max for normalization
  let minVal = Infinity, maxVal = -Infinity
  vertexValues.forEach(v => {
    if (v < minVal) minVal = v
    if (v > maxVal) maxVal = v
  })
  const range = maxVal - minVal || 1

  // Create colors
  const colors: number[] = []
  const colorValues: number[] = []
  for (let i = 0; i < positions.length / 3; i++) {
    const val = vertexValues.get(i) || 0
    const normalized = (val - minVal) / range
    colorValues.push(val)
    const color = interpolateColor(normalized, props.colormap)
    colors.push(color.r, color.g, color.b)
  }

  // Build geometry
  const geometry = new THREE.BufferGeometry()
  geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3))
  geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3))
  geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3))
  geometry.setIndex(indices)
  geometry.computeVertexNormals()

  // Material with clipping support
  const clippingPlanes = getClippingPlanes()
  const material = new THREE.MeshPhongMaterial({
    vertexColors: true,
    side: THREE.DoubleSide,
    shininess: 30,
    clippingPlanes: clippingPlanes,
    clipShadows: true
  })

  // Update scene
  if (mesh) scene.remove(mesh)
  mesh = new THREE.Mesh(geometry, material)
  scene.add(mesh)

  // Wireframe
  if (props.showWireframe) {
    const wireGeo = new THREE.WireframeGeometry(geometry)
    const wireMat = new THREE.LineBasicMaterial({
      color: 0x333333,
      opacity: 0.3,
      transparent: true,
      clippingPlanes: clippingPlanes
    })
    if (wireframeMesh) scene.remove(wireframeMesh)
    wireframeMesh = new THREE.LineSegments(wireGeo, wireMat)
    scene.add(wireframeMesh)
  } else if (wireframeMesh) {
    scene.remove(wireframeMesh)
    wireframeMesh = undefined as any
  }

  mesh.userData = { minVal, maxVal, colorValues }
}

// ---- Scene init ----

function initScene() {
  if (!canvasRef.value || !containerRef.value) return

  const mobile = isMobileDevice()

  // Scene
  scene = new THREE.Scene()
  scene.background = new THREE.Color(0xf0f0f0)

  // Camera
  camera = new THREE.PerspectiveCamera(
    60,
    containerRef.value.clientWidth / containerRef.value.clientHeight,
    0.01,
    1000
  )
  camera.position.set(5, 5, 5)

  // Renderer with mobile optimizations
  const pixelRatio = mobile
    ? Math.min(window.devicePixelRatio, 2) // 移动端限制 pixelRatio 最大为 2
    : window.devicePixelRatio

  renderer = new THREE.WebGLRenderer({
    canvas: canvasRef.value,
    antialias: !mobile, // 移动端关闭抗锯齿以提升性能
    powerPreference: 'high-performance',
  })
  renderer.setSize(containerRef.value.clientWidth, containerRef.value.clientHeight)
  renderer.setPixelRatio(pixelRatio)
  renderer.localClippingEnabled = true

  // 手动管理渲染器信息重置，减少内存开销
  renderer.info.autoReset = false

  // Controls
  controls = new OrbitControls(camera, renderer.domElement)
  controls.enableDamping = true
  controls.dampingFactor = 0.08

  // 触控优化：配置触控手势
  controls.touches = {
    ONE: THREE.TOUCH.ROTATE,
    TWO: THREE.TOUCH.DOLLY_PAN,
  }
  controls.minDistance = 0.5
  controls.maxDistance = 100

  // 防止触控事件与页面滚动冲突
  renderer.domElement.style.touchAction = 'none'

  // 双击重置视角
  renderer.domElement.addEventListener('dblclick', resetCamera)

  // 记录默认相机位置
  const defaultCameraPosition = new THREE.Vector3(5, 5, 5)
  const defaultCameraTarget = new THREE.Vector3(0, 0, 0)

  function resetCamera() {
    // 平滑过渡到默认视角
    const startPosition = camera.position.clone()
    const startTarget = controls.target.clone()
    const duration = 500 // ms
    const startTime = performance.now()

    function animateReset(now: number) {
      const elapsed = now - startTime
      const t = Math.min(elapsed / duration, 1)
      // ease-out cubic
      const ease = 1 - Math.pow(1 - t, 3)

      camera.position.lerpVectors(startPosition, defaultCameraPosition, ease)
      controls.target.lerpVectors(startTarget, defaultCameraTarget, ease)
      controls.update()

      if (t < 1) {
        requestAnimationFrame(animateReset)
      }
    }
    requestAnimationFrame(animateReset)
  }

  // Lights
  const ambient = new THREE.AmbientLight(0xffffff, 0.6)
  scene.add(ambient)

  const directional = new THREE.DirectionalLight(0xffffff, 0.8)
  directional.position.set(5, 10, 5)
  scene.add(directional)

  // Grid helper
  const grid = new THREE.GridHelper(10, 20, 0xcccccc, 0xe0e0e0)
  scene.add(grid)

  // Axes helper
  const axes = new THREE.AxesHelper(1)
  scene.add(axes)

  // Initialize clipping plane object
  clippingPlaneObj = new THREE.Plane(new THREE.Vector3(0, -1, 0), 0)
}

function buildGeometry(result: SimulationResult) {
  // Build node map
  const nodeMap = new Map<number, THREE.Vector3>()
  result.nodes.forEach(n => {
    nodeMap.set(n.id, new THREE.Vector3(n.x, n.y, n.z))
  })

  // Build geometry from elements
  const positions: number[] = []
  const normals: number[] = []
  const indices: number[] = []

  let idx = 0
  const nodeIndexMap = new Map<number, number>()

  result.elements.forEach(el => {
    const nodeIds = el.nodeIds
    const positionsLocal: THREE.Vector3[] = []

    nodeIds.forEach(nid => {
      if (!nodeIndexMap.has(nid)) {
        nodeIndexMap.set(nid, idx++)
        positionsLocal.push(nodeMap.get(nid) || new THREE.Vector3())
      }
    })

    // Triangulate based on element type
    if (el.type === 'TET4' || el.type === 'TETRA') {
      // Tetrahedron: 4 nodes -> 4 triangles
      const faces = [
        [0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]
      ]
      faces.forEach(f => {
        const a = positionsLocal[f[0]]
        const b = positionsLocal[f[1]]
        const c = positionsLocal[f[2]]

        const ab = new THREE.Vector3().subVectors(b, a)
        const ac = new THREE.Vector3().subVectors(c, a)
        const normal = new THREE.Vector3().crossVectors(ab, ac).normalize()

        ;[a, b, c].forEach(v => {
          positions.push(v.x, v.y, v.z)
          normals.push(normal.x, normal.y, normal.z)
        })
        indices.push(idx - 3, idx - 2, idx - 1)
        idx += 3
      })
    } else if (el.type === 'HEX8' || el.type === 'BRICK') {
      // Hexahedron: 8 nodes -> 12 triangles (2 per face, 6 faces)
      const v = positionsLocal
      const faces = [
        [0, 1, 2], [0, 2, 3], // bottom
        [4, 6, 5], [4, 7, 6], // top
        [0, 3, 7], [0, 7, 4], // left
        [1, 5, 6], [1, 6, 2], // right
        [0, 4, 5], [0, 5, 1], // front
        [3, 2, 6], [3, 6, 7]  // back
      ]
      faces.forEach(fi => {
        const a = v[fi[0]]
        const b = v[fi[1]]
        const c = v[fi[2]]

        const ab = new THREE.Vector3().subVectors(b, a)
        const ac = new THREE.Vector3().subVectors(c, a)
        const normal = new THREE.Vector3().crossVectors(ab, ac).normalize()

        ;[a, b, c].forEach(v => {
          positions.push(v.x, v.y, v.z)
          normals.push(normal.x, normal.y, normal.z)
        })
        indices.push(idx, idx + 1, idx + 2)
        idx += 3
      })
    } else {
      // TRI3, QUAD4, etc - triangulate directly
      for (let i = 1; i < nodeIds.length - 1; i++) {
        const a = positionsLocal[0]
        const b = positionsLocal[i]
        const c = positionsLocal[i + 1]

        const ab = new THREE.Vector3().subVectors(b, a)
        const ac = new THREE.Vector3().subVectors(c, a)
        const normal = new THREE.Vector3().crossVectors(ab, ac).normalize()

        ;[a, b, c].forEach(v => {
          positions.push(v.x, v.y, v.z)
          normals.push(normal.x, normal.y, normal.z)
        })
        indices.push(idx, idx + 1, idx + 2)
        idx += 3
      }
    }
  })

  return { positions, normals, indices, nodeIndexMap }
}

function applyDisplacement(result: SimulationResult, scale: number, positions: number[], nodeIndexMap: Map<number, number>) {
  if (!result.displacement) return positions

  const disp = result.displacement.data
  const nodes = result.nodes

  nodes.forEach(n => {
    const mapIdx = nodeIndexMap.get(n.id)
    if (mapIdx === undefined) return

    const ux = (disp['ux']?.[n.id] || 0) * scale
    const uy = (disp['uy']?.[n.id] || 0) * scale
    const uz = (disp['uz']?.[n.id] || 0) * scale

    positions[mapIdx * 3] += ux
    positions[mapIdx * 3 + 1] += uy
    positions[mapIdx * 3 + 2] += uz
  })

  return positions
}

function computeVertexValues(result: SimulationResult, nodeIndexMap: Map<number, number>, _indices: number[]): Map<number, number> {
  const values = new Map<number, number>()

  if (props.displayMode === 'vonMises' && result.vonMises) {
    Object.entries(result.vonMises.data).forEach(([nid, val]) => {
      const idx = nodeIndexMap.get(parseInt(nid))
      if (idx !== undefined) values.set(idx, val)
    })
  } else if (props.displayMode === 'displacement' && result.displacement) {
    const ux = result.displacement.data['ux'] || {}
    const uy = result.displacement.data['uy'] || {}
    const uz = result.displacement.data['uz'] || {}
    result.nodes.forEach(n => {
      const idx = nodeIndexMap.get(n.id)
      if (idx !== undefined) {
        const dx = ux[String(n.id)] || 0
        const dy = uy[String(n.id)] || 0
        const dz = uz[String(n.id)] || 0
        values.set(idx, Math.sqrt(dx*dx + dy*dy + dz*dz))
      }
    })
  } else if (props.displayMode === 'stress' && result.stress) {
    // Use first stress component as example
    const keys = Object.keys(result.stress.data)
    if (keys.length > 0) {
      const component = result.stress.data[keys[0]]
      Object.entries(component).forEach(([nid, val]) => {
        const idx = nodeIndexMap.get(parseInt(nid))
        if (idx !== undefined) values.set(idx, Math.abs(val))
      })
    }
  }

  return values
}

function updateMesh(result: SimulationResult) {
  if (!result || result.nodes.length === 0) {
    if (mesh) scene.remove(mesh)
    if (wireframeMesh) scene.remove(wireframeMesh)
    return
  }

  const { positions, normals, indices, nodeIndexMap } = buildGeometry(result)

  // Apply deformation if enabled
  let finalPositions = [...positions]
  if (props.showDeformed && result.displacement) {
    finalPositions = applyDisplacement(result, props.deformationScale, finalPositions, nodeIndexMap)
  }

  // Compute values for coloring
  const vertexValues = computeVertexValues(result, nodeIndexMap, indices)

  // Find min/max for normalization
  let minVal = Infinity, maxVal = -Infinity
  vertexValues.forEach(v => {
    if (v < minVal) minVal = v
    if (v > maxVal) maxVal = v
  })

  const range = maxVal - minVal || 1

  // Create colors array
  const colors: number[] = []
  const colorValues: number[] = []

  for (let i = 0; i < finalPositions.length / 3; i++) {
    const val = vertexValues.get(i) || 0
    const normalized = (val - minVal) / range
    colorValues.push(val)
    const color = interpolateColor(normalized, props.colormap)
    colors.push(color.r, color.g, color.b)
  }

  // Build geometry
  const geometry = new THREE.BufferGeometry()
  geometry.setAttribute('position', new THREE.Float32BufferAttribute(finalPositions, 3))
  geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3))
  geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3))
  geometry.setIndex(indices)
  geometry.computeVertexNormals()

  // Get clipping planes
  const clippingPlanes = getClippingPlanes()

  // Material with vertex colors and clipping support
  const material = new THREE.MeshPhongMaterial({
    vertexColors: true,
    side: THREE.DoubleSide,
    shininess: 30,
    clippingPlanes: clippingPlanes,
    clipShadows: true
  })

  // Remove old mesh
  if (mesh) scene.remove(mesh)
  mesh = new THREE.Mesh(geometry, material)
  scene.add(mesh)

  // Wireframe overlay
  if (props.showWireframe) {
    const wireGeo = new THREE.WireframeGeometry(geometry)
    const wireMat = new THREE.LineBasicMaterial({
      color: 0x333333,
      opacity: 0.3,
      transparent: true,
      clippingPlanes: clippingPlanes
    })
    if (wireframeMesh) scene.remove(wireframeMesh)
    wireframeMesh = new THREE.LineSegments(wireGeo, wireMat)
    scene.add(wireframeMesh)
  } else if (wireframeMesh) {
    scene.remove(wireframeMesh)
    wireframeMesh = undefined as any
  }

  // Store info for colorbar
  mesh.userData = { minVal, maxVal, colorValues }
}

function animate() {
  requestAnimationFrame(animate)
  controls.update()
  renderer.render(scene, camera)
  // 手动重置渲染器信息
  renderer.info.reset()
}

function handleResize() {
  if (!containerRef.value || !camera || !renderer) return
  const w = containerRef.value.clientWidth
  const h = containerRef.value.clientHeight
  camera.aspect = w / h
  camera.updateProjectionMatrix()
  renderer.setSize(w, h)
}

onMounted(() => {
  initScene()
  animate()
  window.addEventListener('resize', handleResize)

  if (props.result) {
    // Compute model bounds for clipping slider range
    const bounds = computeModelBounds(props.result)
    clipMin.value = Math.min(bounds.min.x, bounds.min.y, bounds.min.z) - 0.5
    clipMax.value = Math.max(bounds.max.x, bounds.max.y, bounds.max.z) + 0.5
    clipPosition.value = (clipMin.value + clipMax.value) / 2

    updateMesh(props.result)
    loading.value = false
    emit('ready')
  }
})

onUnmounted(() => {
  pauseAnimation()
  window.removeEventListener('resize', handleResize)
  removePlaneVisual()
  if (planeHelperObj) {
    planeHelperObj.dispose()
    planeHelperObj = null
  }
  renderer?.dispose()
})

watch(() => props.result, (newResult) => {
  if (newResult) {
    loading.value = true
    nextTick(() => {
      // Recompute model bounds for clipping slider range
      const bounds = computeModelBounds(newResult)
      clipMin.value = Math.min(bounds.min.x, bounds.min.y, bounds.min.z) - 0.5
      clipMax.value = Math.max(bounds.max.x, bounds.max.y, bounds.max.z) + 0.5
      clipPosition.value = (clipMin.value + clipMax.value) / 2

      updateMesh(newResult)
      loading.value = false
    })
  }
}, { deep: true })

watch([() => props.displayMode, () => props.showDeformed, () => props.deformationScale, () => props.colormap, () => props.showWireframe], () => {
  if (props.result) {
    updateMesh(props.result)
  }
})

// Watch for mode shape changes (modal animation)
watch(() => props.modeShape, (newModeShape) => {
  if (newModeShape && props.result) {
    updateMeshWithModeShape(props.result, newModeShape)
  }
}, { deep: true })

// ---- Clipping watchers ----

// Watch clipping toggle
watch(clippingOn, () => {
  updateClippingPlane()
})

// Watch clip direction change
watch(clipDirection, () => {
  updateClippingPlane()
})

// Watch clip position change
watch(clipPosition, () => {
  if (!clippingPlaneObj) return
  clippingPlaneObj.constant = clipPosition.value

  // Update plane helper
  if (planeHelperObj) {
    scene.remove(planeHelperObj)
    planeHelperObj.dispose()
    planeHelperObj = new THREE.PlaneHelper(clippingPlaneObj, 10, 0x4F8CFF)
    scene.add(planeHelperObj)
  }

  // Update plane visual position
  updatePlaneVisual()

  // Update clipping on existing mesh material without full rebuild
  if (mesh && mesh.material) {
    const mat = mesh.material as THREE.MeshPhongMaterial
    if (clippingOn.value) {
      mat.clippingPlanes = [clippingPlaneObj]
      mat.needsUpdate = true
    } else {
      mat.clippingPlanes = []
      mat.needsUpdate = true
    }
  }
  if (wireframeMesh && wireframeMesh.material) {
    const wireMat = wireframeMesh.material as THREE.LineBasicMaterial
    if (clippingOn.value) {
      wireMat.clippingPlanes = [clippingPlaneObj]
      wireMat.needsUpdate = true
    } else {
      wireMat.clippingPlanes = []
      wireMat.needsUpdate = true
    }
  }
})

// Watch external clipping props
watch(() => props.clippingEnabled, (enabled) => {
  if (enabled !== undefined) {
    clippingOn.value = enabled
  }
})

watch(() => props.clippingPlane, (planeConfig) => {
  if (planeConfig && clippingPlaneObj) {
    clippingPlaneObj.normal.set(planeConfig.normal[0], planeConfig.normal[1], planeConfig.normal[2])
    clippingPlaneObj.constant = planeConfig.constant
    clipPosition.value = planeConfig.constant

    // Infer direction from normal
    const n = planeConfig.normal
    if (Math.abs(n[2]) > Math.abs(n[1]) && Math.abs(n[2]) > Math.abs(n[0])) {
      clipDirection.value = 'XY'
    } else if (Math.abs(n[1]) > Math.abs(n[0])) {
      clipDirection.value = 'XZ'
    } else {
      clipDirection.value = 'YZ'
    }

    clippingOn.value = true
    updateClippingPlane()
  }
}, { deep: true })

// ---- Animation watchers ----

watch(currentFrame, (newFrame) => {
  // When user manually drags the slider, update the mesh
  if (!isPlaying.value && props.animationFrames && props.animationFrames.length > 0) {
    updateMeshForFrame(newFrame)
  }
})

watch(playbackSpeed, () => {
  // If currently playing, restart with new speed
  if (isPlaying.value) {
    pauseAnimation()
    playAnimation()
  }
})

watch(() => props.animationFrames, (newFrames) => {
  if (newFrames && newFrames.length > 0) {
    currentFrame.value = 0
    if (newFrames.length > 1 && !isPlaying.value) {
      updateMeshForFrame(0)
    }
  } else {
    pauseAnimation()
  }
}, { deep: true })

// Update mesh with mode shape animation
function updateMeshWithModeShape(result: SimulationResult, modeShapeData: ModeShapeData[]) {
  if (!result || result.nodes.length === 0) return

  // Build node position map with displacement applied
  const nodePositions = new Map<number, { x: number; y: number; z: number }>()
  const nodeMagnitudes = new Map<number, number>()

  modeShapeData.forEach(d => {
    nodePositions.set(d.node_id, { x: d.x + d.ux, y: d.y + d.uy, z: d.z + d.uz })
    nodeMagnitudes.set(d.node_id, d.magnitude)
  })

  // Build positions and vertex values from mode shape data
  const positions: number[] = []
  const normals: number[] = []
  const indices: number[] = []
  const vertexValues: number[] = []

  let idx = 0

  result.elements.forEach(el => {
    const nodeIds = el.nodeIds

    // Get displaced positions for this element's nodes
    const positionsLocal: Array<{ x: number; y: number; z: number }> = []
    const valuesLocal: number[] = []

    nodeIds.forEach(nid => {
      const pos = nodePositions.get(nid)
      if (pos) {
        positionsLocal.push(pos)
        valuesLocal.push(nodeMagnitudes.get(nid) || 0)
      } else {
        const node = result.nodes.find(n => n.id === nid)
        if (node) {
          positionsLocal.push({ x: node.x, y: node.y, z: node.z })
          valuesLocal.push(0)
        }
      }
    })

    // Triangulate
    for (let i = 1; i < nodeIds.length - 1; i++) {
      const a = positionsLocal[0]
      const b = positionsLocal[i]
      const c = positionsLocal[i + 1]

      const ab = new THREE.Vector3().subVectors(
        new THREE.Vector3(b.x, b.y, b.z),
        new THREE.Vector3(a.x, a.y, a.z)
      )
      const ac = new THREE.Vector3().subVectors(
        new THREE.Vector3(c.x, c.y, c.z),
        new THREE.Vector3(a.x, a.y, a.z)
      )
      const normal = new THREE.Vector3().crossVectors(ab, ac).normalize()

      const valA = valuesLocal[0]
      const valB = valuesLocal[i]
      const valC = valuesLocal[i + 1]

      ;[a, b, c].forEach(v => {
        positions.push(v.x, v.y, v.z)
        normals.push(normal.x, normal.y, normal.z)
      })

      const avgVal = (valA + valB + valC) / 3
      vertexValues.push(avgVal, avgVal, avgVal)

      indices.push(idx, idx + 1, idx + 2)
      idx += 3
    }
  })

  // Find min/max for normalization
  let minVal = Infinity, maxVal = -Infinity
  vertexValues.forEach(v => {
    if (v < minVal) minVal = v
    if (v > maxVal) maxVal = v
  })

  const range = maxVal - minVal || 1

  // Create colors
  const colors: number[] = []
  for (let i = 0; i < vertexValues.length; i++) {
    const normalized = Math.max(0, Math.min(1, (vertexValues[i] - minVal) / range))
    const color = interpolateColor(normalized, props.colormap)
    colors.push(color.r, color.g, color.b)
  }

  // Build geometry
  const geometry = new THREE.BufferGeometry()
  geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3))
  geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3))
  geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3))
  geometry.setIndex(indices)

  // Get clipping planes
  const clippingPlanes = getClippingPlanes()

  // Material
  const material = new THREE.MeshPhongMaterial({
    vertexColors: true,
    side: THREE.DoubleSide,
    shininess: 30,
    clippingPlanes: clippingPlanes,
    clipShadows: true
  })

  // Update scene
  if (mesh) scene.remove(mesh)
  mesh = new THREE.Mesh(geometry, material)
  scene.add(mesh)

  mesh.userData = { minVal, maxVal, colorValues: vertexValues }
}

// Expose color data for legend and animation controls
defineExpose({
  getColorData: () => mesh?.userData,
  playAnimation,
  pauseAnimation,
  isPlaying,
  currentFrame
})
</script>
