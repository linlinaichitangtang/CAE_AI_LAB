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

interface SimulationResult {
  nodes: Array<{ id: number; x: number; y: number; z: number }>
  elements: Array<{ id: number; type: string; nodeIds: number[] }>
  displacement?: { step: number; data: Record<string, Record<string, number>> }
  stress?: { step: number; data: Record<string, Record<string, number>> }
  vonMises?: { step: number; data: Record<string, number> }
  deformationScale?: number
}

const props = withDefaults(defineProps<{
  result?: SimulationResult | null
  displayMode?: 'displacement' | 'stress' | 'vonMises'
  showDeformed?: boolean
  deformationScale?: number
  colormap?: 'viridis' | 'plasma' | 'inferno' | 'jet' | 'rainbow'
  showWireframe?: boolean
}>(), {
  displayMode: 'vonMises',
  showDeformed: false,
  deformationScale: 1.0,
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
// eslint-disable-next-line @typescript-eslint/no-explicit-any
let wireframeMesh: any = null

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

function initScene() {
  if (!canvasRef.value || !containerRef.value) return

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

  // Renderer
  renderer = new THREE.WebGLRenderer({
    canvas: canvasRef.value,
    antialias: true
  })
  renderer.setSize(containerRef.value.clientWidth, containerRef.value.clientHeight)
  renderer.setPixelRatio(window.devicePixelRatio)

  // Controls
  controls = new OrbitControls(camera, renderer.domElement)
  controls.enableDamping = true
  controls.dampingFactor = 0.05

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

  // Material with vertex colors
  const material = new THREE.MeshPhongMaterial({
    vertexColors: true,
    side: THREE.DoubleSide,
    shininess: 30
  })

  // Remove old mesh
  if (mesh) scene.remove(mesh)
  mesh = new THREE.Mesh(geometry, material)
  scene.add(mesh)

  // Wireframe overlay
  if (props.showWireframe) {
    const wireGeo = new THREE.WireframeGeometry(geometry)
    const wireMat = new THREE.LineBasicMaterial({ color: 0x333333, opacity: 0.3, transparent: true })
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

watch([() => props.displayMode, () => props.showDeformed, () => props.deformationScale, () => props.colormap, () => props.showWireframe], () => {
  if (props.result) {
    updateMesh(props.result)
  }
})

// Expose color data for legend
defineExpose({
  getColorData: () => mesh?.userData
})
</script>