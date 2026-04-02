<template>
  <div class="shape-optimization-panel p-4 space-y-4">
    <h3 class="text-sm font-semibold text-[var(--text-primary)] flex items-center gap-2">
      <span class="w-6 h-6 rounded-full bg-indigo-600 text-white text-xs flex items-center justify-center">S</span>
      Shape Optimization
    </h3>

    <div class="space-y-3 pl-8">
      <!-- Mesh Info -->
      <div class="border rounded p-3 space-y-2">
        <h4 class="text-xs font-semibold text-[var(--text-secondary)]">Mesh</h4>
        <div class="grid grid-cols-2 gap-2 text-xs">
          <div class="bg-gray-50 dark:bg-gray-800 rounded p-2">
            <span class="text-gray-500">Nodes</span>
            <p class="font-semibold text-[var(--text-primary)]">{{ meshInfo.nodes }}</p>
          </div>
          <div class="bg-gray-50 dark:bg-gray-800 rounded p-2">
            <span class="text-gray-500">Elements</span>
            <p class="font-semibold text-[var(--text-primary)]">{{ meshInfo.elements }}</p>
          </div>
        </div>
        <button
          @click="generateDemoMesh"
          class="w-full px-3 py-1.5 text-xs border border-indigo-300 text-indigo-600 rounded hover:bg-indigo-50"
        >
          Generate Demo Mesh
        </button>
      </div>

      <!-- Boundary Selection -->
      <div class="border rounded p-3 space-y-2">
        <h4 class="text-xs font-semibold text-[var(--text-secondary)]">Boundary Nodes</h4>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] block mb-1">Boundary Selection</label>
          <select v-model="boundaryPreset" @change="applyBoundaryPreset" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]">
            <option value="all_surface">All Surface Nodes</option>
            <option value="top_face">Top Face Only</option>
            <option value="custom">Custom (Manual)</option>
          </select>
        </div>
        <div class="text-[10px] text-[var(--text-muted)]">
          {{ shapeConfig.boundary_node_ids.length }} boundary nodes selected
        </div>
      </div>

      <!-- Fixed Nodes -->
      <div class="border rounded p-3 space-y-2">
        <h4 class="text-xs font-semibold text-[var(--text-secondary)]">Fixed Nodes</h4>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] block mb-1">Fixed Face</label>
          <select v-model="fixedPreset" @change="applyFixedPreset" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]">
            <option value="left">Left Face</option>
            <option value="right">Right Face</option>
            <option value="bottom">Bottom Face</option>
          </select>
        </div>
      </div>

      <!-- Load -->
      <div class="border rounded p-3 space-y-2">
        <h4 class="text-xs font-semibold text-[var(--text-secondary)]">Load</h4>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] block mb-1">Load Position</label>
          <select v-model="loadPreset" @change="applyLoadPreset" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]">
            <option value="right">Right Face</option>
            <option value="top">Top Face</option>
            <option value="bottom">Bottom Face</option>
          </select>
        </div>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] block mb-1">Load Magnitude (N)</label>
          <input v-model.number="loadMagnitude" type="number" step="10" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
        </div>
      </div>

      <!-- Optimization Parameters -->
      <div class="border rounded p-3 space-y-2">
        <h4 class="text-xs font-semibold text-[var(--text-secondary)]">Parameters</h4>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] block mb-1">Objective</label>
          <select v-model="shapeConfig.objective" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]">
            <option value="minimize_compliance">Minimize Compliance</option>
          </select>
        </div>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] block mb-1">Max Movement (mm)</label>
          <input v-model.number="shapeConfig.max_movement" type="number" step="0.1" min="0.01" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
        </div>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] block mb-1">Max Iterations</label>
          <input v-model.number="shapeConfig.max_iterations" type="number" min="5" max="200" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
        </div>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] block mb-1">Convergence Tolerance</label>
          <input v-model.number="shapeConfig.convergence_tolerance" type="number" step="0.001" min="0.0001" class="w-full px-2 py-1 border rounded text-xs bg-[var(--bg-base)] text-[var(--text-primary)]" />
        </div>
      </div>

      <!-- Run Button -->
      <button
        @click="runShapeOptimization"
        :disabled="running || shapeConfig.mesh_nodes.length === 0"
        class="w-full px-3 py-2 bg-indigo-600 text-white rounded text-sm hover:bg-indigo-700 disabled:opacity-50"
      >
        <span v-if="running" class="mr-1 animate-spin">~</span>
        <span v-else class="mr-1">></span>
        {{ running ? 'Optimizing...' : 'Run Shape Optimization' }}
      </button>

      <!-- Results -->
      <div v-if="result" class="border rounded p-3 space-y-2">
        <h4 class="text-xs font-semibold text-[var(--text-secondary)]">Results</h4>
        <div class="grid grid-cols-2 gap-2 text-xs">
          <div class="bg-gray-50 dark:bg-gray-800 rounded p-2">
            <span class="text-gray-500">Iterations</span>
            <p class="font-semibold text-[var(--text-primary)]">{{ result.iterations.length }}</p>
          </div>
          <div class="bg-gray-50 dark:bg-gray-800 rounded p-2">
            <span class="text-gray-500">Converged</span>
            <p :class="result.converged ? 'text-green-600' : 'text-yellow-600'">
              {{ result.converged ? 'Yes' : 'No' }}
            </p>
          </div>
          <div class="bg-gray-50 dark:bg-gray-800 rounded p-2">
            <span class="text-gray-500">Final Compliance</span>
            <p class="font-semibold text-[var(--text-primary)]">{{ result.final_compliance.toFixed(4) }}</p>
          </div>
          <div class="bg-gray-50 dark:bg-gray-800 rounded p-2">
            <span class="text-gray-500">Elapsed</span>
            <p class="font-semibold text-[var(--text-primary)]">{{ result.elapsed_time_seconds.toFixed(2) }}s</p>
          </div>
        </div>

        <!-- Compliance History Chart (SVG) -->
        <div class="mt-2">
          <h5 class="text-[10px] text-[var(--text-muted)] mb-1">Compliance History</h5>
          <svg :width="280" :height="100" class="w-full">
            <rect x="0" y="0" :width="280" :height="100" fill="var(--bg-base, #f9fafb)" rx="4" />
            <polyline
              v-if="result.iterations.length > 1"
              :points="complianceChartPoints"
              fill="none"
              stroke="#6366f1"
              stroke-width="1.5"
            />
            <circle
              v-for="(pt, idx) in result.iterations"
              :key="idx"
              :cx="mapChartX(idx, result.iterations.length)"
              :cy="mapChartY(pt.compliance)"
              r="2"
              fill="#6366f1"
            />
          </svg>
        </div>

        <button
          @click="emit('showResult', result)"
          class="w-full px-3 py-1.5 text-xs border border-green-300 text-green-600 rounded hover:bg-green-50"
        >
          Show Before/After
        </button>
        <button
          @click="exportResult"
          class="w-full px-3 py-1.5 text-xs border border-blue-300 text-blue-600 rounded hover:bg-blue-50"
        >
          Export STL
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import { runShapeOptimization as apiRunShapeOptimization, exportShapeOptToStl, type ShapeOptResult } from '@/api/optimization'
import { invoke } from '@tauri-apps/api/core'

const emit = defineEmits<{
  (e: 'showResult', result: ShapeOptResult): void
  (e: 'meshUpdate', nodes: number[][]): void
}>()

const running = ref(false)
const result = ref<ShapeOptResult | null>(null)

const meshInfo = reactive({ nodes: 0, elements: 0 })
const boundaryPreset = ref('all_surface')
const fixedPreset = ref('left')
const loadPreset = ref('right')
const loadMagnitude = ref(100)

const shapeConfig = reactive({
  mesh_nodes: [] as number[][],
  mesh_elements: [] as number[][],
  boundary_node_ids: [] as number[],
  fixed_node_ids: [] as number[],
  load_node_ids: [] as number[],
  load_values: [] as number[],
  objective: 'minimize_compliance',
  max_movement: 1.0,
  max_iterations: 50,
  convergence_tolerance: 0.001,
})

function generateDemoMesh() {
  // Generate a simple tetrahedral mesh for demo
  const nx = 8, ny = 4, nz = 2
  const nodes: number[][] = []
  const elements: number[][] = []
  const xSize = 40, ySize = 20, zSize = 10

  // Generate nodes
  for (let iz = 0; iz <= nz; iz++) {
    for (let iy = 0; iy <= ny; iy++) {
      for (let ix = 0; ix <= nx; ix++) {
        nodes.push([
          (ix / nx) * xSize,
          (iy / ny) * ySize,
          (iz / nz) * zSize,
        ])
      }
    }
  }

  // Generate tetrahedra from hex cells
  const nnx = nx + 1, nny = ny + 1
  for (let iz = 0; iz < nz; iz++) {
    for (let iy = 0; iy < ny; iy++) {
      for (let ix = 0; ix < nx; ix++) {
        const n0 = iz * nnx * nny + iy * nnx + ix
        const n1 = n0 + 1
        const n2 = n0 + nnx
        const n3 = n2 + 1
        const n4 = n0 + nnx * nny
        const n5 = n4 + 1
        const n6 = n4 + nnx
        const n7 = n6 + 1

        // Split hex into 5 tets (standard decomposition)
        elements.push([n0, n1, n2, n4])
        elements.push([n1, n3, n2, n7])
        elements.push([n1, n7, n4, n2])
        elements.push([n1, n5, n7, n4])
        elements.push([n2, n7, n4, n6])
      }
    }
  }

  shapeConfig.mesh_nodes = nodes
  shapeConfig.mesh_elements = elements
  meshInfo.nodes = nodes.length
  meshInfo.elements = elements.length

  applyBoundaryPreset()
  applyFixedPreset()
  applyLoadPreset()
}

function applyBoundaryPreset() {
  if (shapeConfig.mesh_nodes.length === 0) return

  const nnx = 9, nny = 5, nnz = 3
  const boundary: number[] = []

  for (let i = 0; i < shapeConfig.mesh_nodes.length; i++) {
    const [x, y, z] = shapeConfig.mesh_nodes[i]
    const ix = Math.round((x / 40) * 8)
    const iy = Math.round((y / 20) * 4)
    const iz = Math.round((z / 10) * 2)

    if (boundaryPreset.value === 'all_surface') {
      if (ix === 0 || ix === 8 || iy === 0 || iy === 4 || iz === 0 || iz === 2) {
        boundary.push(i)
      }
    } else if (boundaryPreset.value === 'top_face') {
      if (iy === 4) boundary.push(i)
    }
  }

  shapeConfig.boundary_node_ids = boundary
}

function applyFixedPreset() {
  if (shapeConfig.mesh_nodes.length === 0) return

  const fixed: number[] = []
  for (let i = 0; i < shapeConfig.mesh_nodes.length; i++) {
    const [x, y, z] = shapeConfig.mesh_nodes[i]
    if (fixedPreset.value === 'left' && x < 0.1) fixed.push(i)
    else if (fixedPreset.value === 'right' && x > 39.9) fixed.push(i)
    else if (fixedPreset.value === 'bottom' && y < 0.1) fixed.push(i)
  }
  shapeConfig.fixed_node_ids = fixed
}

function applyLoadPreset() {
  if (shapeConfig.mesh_nodes.length === 0) return

  const loadNodes: number[] = []
  for (let i = 0; i < shapeConfig.mesh_nodes.length; i++) {
    const [x, y, z] = shapeConfig.mesh_nodes[i]
    if (loadPreset.value === 'right' && x > 39.9) loadNodes.push(i)
    else if (loadPreset.value === 'top' && y > 19.9) loadNodes.push(i)
    else if (loadPreset.value === 'bottom' && y < 0.1) loadNodes.push(i)
  }
  shapeConfig.load_node_ids = loadNodes
  shapeConfig.load_values = loadNodes.map(() => loadMagnitude.value)
}

async function runShapeOptimization() {
  if (shapeConfig.mesh_nodes.length === 0) return

  running.value = true
  result.value = null

  try {
    const res = await apiRunShapeOptimization({
      mesh_nodes: shapeConfig.mesh_nodes as [number, number, number][],
      mesh_elements: shapeConfig.mesh_elements as [number, number, number, number][],
      boundary_node_ids: shapeConfig.boundary_node_ids,
      fixed_node_ids: shapeConfig.fixed_node_ids,
      load_node_ids: shapeConfig.load_node_ids,
      load_values: shapeConfig.load_values,
      objective: shapeConfig.objective,
      max_movement: shapeConfig.max_movement,
      max_iterations: shapeConfig.max_iterations,
      convergence_tolerance: shapeConfig.convergence_tolerance,
    })

    result.value = res
    emit('meshUpdate', res.optimized_nodes)
  } catch (e) {
    console.error('Shape optimization failed:', e)
  } finally {
    running.value = false
  }
}

async function exportResult() {
  if (!result.value) return
  try {
    const res = await exportShapeOptToStl({
      nodes: result.value.optimized_nodes,
      elements: shapeConfig.mesh_elements as [number, number, number, number][],
      output_path: 'shape_optimization.stl',
    })
    alert(`STL exported: ${res.file_path}`)
  } catch (e) {
    console.error('Export failed:', e)
  }
}

// Chart helpers
const complianceRange = computed(() => {
  if (!result.value || result.value.iterations.length === 0) return { min: 0, max: 1 }
  const values = result.value.iterations.map(i => i.compliance)
  return {
    min: Math.min(...values) * 0.95,
    max: Math.max(...values) * 1.05,
  }
})

function mapChartX(idx: number, total: number): number {
  const padding = 5
  return padding + (idx / Math.max(total - 1, 1)) * (280 - 2 * padding)
}

function mapChartY(value: number): number {
  const padding = 5
  const { min, max } = complianceRange.value
  const range = max - min || 1
  return 95 - padding - ((value - min) / range) * (100 - 2 * padding)
}

const complianceChartPoints = computed(() => {
  if (!result.value) return ''
  return result.value.iterations
    .map((iter, idx) => `${mapChartX(idx, result.value!.iterations.length)},${mapChartY(iter.compliance)}`)
    .join(' ')
})
</script>
