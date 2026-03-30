<template>
  <div class="relative w-full h-full" ref="containerRef">
    <canvas ref="canvasRef" class="w-full h-full"></canvas>
    <!-- Loading overlay -->
    <div v-if="loading" class="absolute inset-0 bg-white/80 flex items-center justify-center">
      <div class="text-gray-500">加载中...</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from 'vue'
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'

interface ThermalResult {
  nodes: Array<{ id: number; x: number; y: number; z: number }>
  elements: Array<{ id: number; type: string; nodeIds: number[] }>
  temperature?: { step: number; data: Record<string, number> }  // 节点温度
  heatFlux?: { step: number; data: Record<string, { x: number; y: number; z: number }> }  // 热流矢量
}

const props = withDefaults(defineProps<{
  result?: ThermalResult | null
  displayMode?: 'temperature' | 'heatFlux'
  showStreamlines?: boolean
  colormap?: 'viridis' | 'plasma' | 'inferno' | 'jet' | 'rainbow'
  showWireframe?: boolean
}>(), {
  displayMode: 'temperature',
  showStreamlines: false,
  colormap: 'viridis',
  showWireframe: false
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
let wireframeMesh: THREE.LineSegments | null = null

// Colormap definitions (RGB values 0-255) - 热力图专用
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

function initScene() {
  if (!canvasRef.value || !containerRef.value) return

  scene = new THREE.Scene()
  scene.background = new THREE.Color(0xf0f0f0)

  camera = new THREE.PerspectiveCamera(
    60,
    containerRef.value.clientWidth / containerRef.value.clientHeight,
    0.01,
    1000
  )
  camera.position.set(5, 5, 5)

  renderer = new THREE.WebGLRenderer({
    canvas: canvasRef.value,
    antialias: true
  })
  renderer.setSize(containerRef.value.clientWidth, containerRef.value.clientHeight)
  renderer.setPixelRatio(window.devicePixelRatio)

  controls = new OrbitControls(camera, renderer.domElement)
  controls.enableDamping = true
  controls.dampingFactor = 0.05

  const ambient = new THREE.AmbientLight(0xffffff, 0.6)
  scene.add(ambient)

  const directional = new THREE.DirectionalLight(0xffffff, 0.8)
  directional.position.set(5, 10, 5)
  scene.add(directional)

  const grid = new THREE.GridHelper(10, 20, 0xcccccc, 0xe0e0e0)
  scene.add(grid)

  const axes = new THREE.AxesHelper(1)
  scene.add(axes)
}

function buildGeometry(result: ThermalResult) {
  const nodeMap = new Map<number, THREE.Vector3>()
  result.nodes.forEach(n => {
    nodeMap.set(n.id, new THREE.Vector3(n.x, n.y, n.z))
  })

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

    if (el.type === 'TET4' || el.type === 'TETRA') {
      const faces = [[0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]]
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

  return { positions, normals, indices, nodeIndexMap }
}

function computeVertexValues(result: ThermalResult, nodeIndexMap: Map<number, number>): Map<number, number> {
  const values = new Map<number, number>()

  if (props.displayMode === 'temperature' && result.temperature) {
    Object.entries(result.temperature.data).forEach(([nid, val]) => {
      const idx = nodeIndexMap.get(parseInt(nid))
      if (idx !== undefined) values.set(idx, val)
    })
  } else if (props.displayMode === 'heatFlux' && result.heatFlux) {
    // 热流模长
    Object.entries(result.heatFlux.data).forEach(([nid, flux]) => {
      const idx = nodeIndexMap.get(parseInt(nid))
      if (idx !== undefined) {
        const magnitude = Math.sqrt(flux.x ** 2 + flux.y ** 2 + flux.z ** 2)
        values.set(idx, magnitude)
      }
    })
  }

  return values
}

function updateMesh(result: ThermalResult) {
  if (!result || result.nodes.length === 0) {
    if (mesh) scene.remove(mesh)
    if (wireframeMesh) scene.remove(wireframeMesh)
    return
  }

  const { positions, normals, indices, nodeIndexMap } = buildGeometry(result)

  const vertexValues = computeVertexValues(result, nodeIndexMap)

  let minVal = Infinity, maxVal = -Infinity
  vertexValues.forEach(v => {
    if (v < minVal) minVal = v
    if (v > maxVal) maxVal = v
  })

  const range = maxVal - minVal || 1

  const colors: number[] = []

  for (let i = 0; i < positions.length / 3; i++) {
    const val = vertexValues.get(i) || 0
    const normalized = (val - minVal) / range
    const color = interpolateColor(normalized, props.colormap)
    colors.push(color.r, color.g, color.b)
  }

  const geometry = new THREE.BufferGeometry()
  geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3))
  geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3))
  geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3))
  geometry.setIndex(indices)
  geometry.computeVertexNormals()

  const material = new THREE.MeshPhongMaterial({
    vertexColors: true,
    side: THREE.DoubleSide,
    shininess: 30
  })

  if (mesh) scene.remove(mesh)
  mesh = new THREE.Mesh(geometry, material)
  scene.add(mesh)

  if (props.showWireframe) {
    const wireGeo = new THREE.WireframeGeometry(geometry)
    const wireMat = new THREE.LineBasicMaterial({ color: 0x333333, opacity: 0.3, transparent: true })
    if (wireframeMesh) scene.remove(wireframeMesh)
    wireframeMesh = new THREE.LineSegments(wireGeo, wireMat)
    scene.add(wireframeMesh)
  } else if (wireframeMesh) {
    scene.remove(wireframeMesh)
    wireframeMesh = null
  }

  mesh.userData = { minVal, maxVal }
}

function animate() {
  requestAnimationFrame(animate)
  controls.update()
  renderer.render(scene, camera)
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
    updateMesh(props.result)
    loading.value = false
    emit('ready')
  }
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
  renderer?.dispose()
})

watch(() => props.result, (newResult) => {
  if (newResult) {
    loading.value = true
    nextTick(() => {
      updateMesh(newResult)
      loading.value = false
    })
  }
}, { deep: true })

watch([() => props.displayMode, () => props.colormap, () => props.showWireframe], () => {
  if (props.result) {
    updateMesh(props.result)
  }
})

defineExpose({
  getColorData: () => mesh?.userData
})
</script>
