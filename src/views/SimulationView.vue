<template>
  <div class="h-full flex flex-col bg-gray-50">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-white border-b">
      <div>
        <h2 class="text-lg font-semibold text-gray-800">仿真分析</h2>
        <p class="text-sm text-gray-500">网格生成、边界条件设置、求解、结果可视化</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- 🔗 嵌入到笔记 -->
        <button 
          @click="showEmbedToNoteDialog"
          :disabled="!projectStore.currentNoteId || !projectStore.hasResult"
          class="px-3 py-1.5 text-sm border border-orange-300 text-orange-600 rounded hover:bg-orange-50 transition disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-1"
        >
          <span>🔗</span>
          <span>嵌入到笔记</span>
        </button>
        <button 
          @click="resetAll"
          class="px-3 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50 transition"
        >
          重置
        </button>
        <button 
          @click="exportScreenshot"
          class="px-3 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50 transition"
        >
          导出图片
        </button>
      </div>
    </div>

    <!-- Main content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Mesh Generation & BC Setup -->
      <div class="w-80 bg-white border-r overflow-y-auto p-4 space-y-6">
        <!-- Step 1: Mesh Generation -->
        <div class="space-y-3">
          <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
            <span class="w-5 h-5 rounded-full bg-blue-600 text-white text-xs flex items-center justify-center">1</span>
            网格生成
          </h3>
          
          <div class="space-y-2">
            <div>
              <label class="text-xs text-gray-600 mb-1 block">维度</label>
              <select v-model="meshDimension" class="w-full px-2 py-1.5 border rounded text-sm">
                <option value="2d">2D 网格</option>
                <option value="3d">3D 网格</option>
              </select>
            </div>

            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-xs text-gray-600 mb-1 block">X 范围 (min)</label>
                <input type="number" v-model.number="meshXMin" class="w-full px-2 py-1 border rounded text-sm" />
              </div>
              <div>
                <label class="text-xs text-gray-600 mb-1 block">X 范围 (max)</label>
                <input type="number" v-model.number="meshXMax" class="w-full px-2 py-1 border rounded text-sm" />
              </div>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-xs text-gray-600 mb-1 block">Y 范围 (min)</label>
                <input type="number" v-model.number="meshYMin" class="w-full px-2 py-1 border rounded text-sm" />
              </div>
              <div>
                <label class="text-xs text-gray-600 mb-1 block">Y 范围 (max)</label>
                <input type="number" v-model.number="meshYMax" class="w-full px-2 py-1 border rounded text-sm" />
              </div>
            </div>

            <div v-if="meshDimension === '3d'" class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-xs text-gray-600 mb-1 block">Z 范围 (min)</label>
                <input type="number" v-model.number="meshZMin" class="w-full px-2 py-1 border rounded text-sm" />
              </div>
              <div>
                <label class="text-xs text-gray-600 mb-1 block">Z 范围 (max)</label>
                <input type="number" v-model.number="meshZMax" class="w-full px-2 py-1 border rounded text-sm" />
              </div>
            </div>

            <div class="grid grid-cols-3 gap-2">
              <div>
                <label class="text-xs text-gray-600 mb-1 block">X 分格</label>
                <input type="number" v-model.number="meshXDiv" class="w-full px-2 py-1 border rounded text-sm" min="1" />
              </div>
              <div>
                <label class="text-xs text-gray-600 mb-1 block">Y 分格</label>
                <input type="number" v-model.number="meshYDiv" class="w-full px-2 py-1 border rounded text-sm" min="1" />
              </div>
              <div v-if="meshDimension === '3d'">
                <label class="text-xs text-gray-600 mb-1 block">Z 分格</label>
                <input type="number" v-model.number="meshZDiv" class="w-full px-2 py-1 border rounded text-sm" min="1" />
              </div>
            </div>

            <button 
              @click="generateMesh" 
              :disabled="generatingMesh"
              class="w-full px-3 py-2 bg-blue-600 text-white rounded text-sm hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {{ generatingMesh ? '生成中...' : '生成网格' }}
            </button>

            <div v-if="projectStore.hasMesh" class="text-xs text-green-600 bg-green-50 rounded p-2">
              ✓ 网格已生成: {{ projectStore.currentMesh!.nodes.length }} 节点, {{ projectStore.currentMesh!.elements.length }} 单元
            </div>
          </div>
        </div>

        <!-- Step 2: Material -->
        <div class="space-y-3">
          <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
            <span class="w-5 h-5 rounded-full bg-blue-600 text-white text-xs flex items-center justify-center">2</span>
            材料参数
          </h3>
          <div>
            <label class="text-xs text-gray-600 mb-1 block">弹性模量 (MPa)</label>
            <input type="number" v-model.number="materialE" class="w-full px-2 py-1 border rounded text-sm" />
          </div>
          <div>
            <label class="text-xs text-gray-600 mb-1 block">泊松比</label>
            <input type="number" v-model.number="materialNu" step="0.01" min="0" max="0.5" class="w-full px-2 py-1 border rounded text-sm" />
          </div>
          <div>
            <label class="text-xs text-gray-600 mb-1 block">密度 (ton/mm³)</label>
            <input type="number" v-model.number="materialDensity" step="1e-9" class="w-full px-2 py-1 border rounded text-sm" />
          </div>
        </div>

        <!-- Step 3: Boundary Conditions -->
        <div class="space-y-3">
          <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
            <span class="w-5 h-5 rounded-full bg-blue-600 text-white text-xs flex items-center justify-center">3</span>
            边界条件
          </h3>
          
          <div v-if="!projectStore.hasMesh" class="text-xs text-gray-500 italic">
            请先生成网格
          </div>
          <div v-else class="space-y-3">
            <!-- Quick Preset: Cantilever Beam -->
            <button @click="applyCantileverPreset" class="w-full px-2 py-1.5 border border-blue-300 text-blue-600 rounded text-xs hover:bg-blue-50 transition">
              ⚡ 快速应用：悬臂梁预设
            </button>
            
            <div class="divider border-t"></div>

            <!-- Fixed BC List -->
            <div>
              <div class="flex justify-between items-center mb-1">
                <label class="text-xs text-gray-600">固定约束 ({{ projectStore.boundaryConditions.fixedBcs.length }})</label>
              </div>
              <div v-for="(bc, idx) in projectStore.boundaryConditions.fixedBcs" :key="idx" class="flex items-center justify-between text-xs p-2 bg-gray-50 rounded mb-1">
                <span>{{ bc.name }} ({{ bc.nodes.length }} 节点)</span>
                <button @click="projectStore.removeFixedBc(idx)" class="text-red-500 hover:text-red-700">删除</button>
              </div>
            </div>

            <!-- Point Load List -->
            <div>
              <div class="flex justify-between items-center mb-1">
                <label class="text-xs text-gray-600">点荷载 ({{ projectStore.boundaryConditions.pointLoads.length }})</label>
              </div>
              <div v-for="(load, idx) in projectStore.boundaryConditions.pointLoads" :key="idx" class="flex items-center justify-between text-xs p-2 bg-gray-50 rounded mb-1">
                <span>{{ load.name }} = {{ load.magnitude }}</span>
                <button @click="projectStore.removePointLoad(idx)" class="text-red-500 hover:text-red-700">删除</button>
              </div>
            </div>

            <!-- Uniform Load List -->
            <div>
              <div class="flex justify-between items-center mb-1">
                <label class="text-xs text-gray-600">均布荷载 ({{ projectStore.boundaryConditions.uniformLoads.length }})</label>
              </div>
              <div v-for="(load, idx) in projectStore.boundaryConditions.uniformLoads" :key="idx" class="flex items-center justify-between text-xs p-2 bg-gray-50 rounded mb-1">
                <span>{{ load.name }} = {{ load.magnitude }}</span>
                <button @click="projectStore.removeUniformLoad(idx)" class="text-red-500 hover:text-red-700">删除</button>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 4: Run Solver -->
        <div class="space-y-3">
          <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
            <span class="w-5 h-5 rounded-full bg-blue-600 text-white text-xs flex items-center justify-center">4</span>
            运行求解
          </h3>

          <button 
            @click="runSolver" 
            :disabled="!canRunSolver || runningSolver"
            class="w-full px-3 py-2 bg-green-600 text-white rounded text-sm hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ runningSolver ? '求解中...' : '运行求解器' }}
          </button>

          <div v-if="projectStore.hasResult" class="text-xs text-green-600 bg-green-50 rounded p-2">
            ✓ 求解完成，结果已加载
          </div>

          <div v-if="lastError" class="text-xs text-red-600 bg-red-50 rounded p-2">
            ✗ {{ lastError }}
          </div>
        </div>
      </div>

      <!-- 3D Viewer -->
      <div class="flex-1 relative bg-gray-100">
        <ResultViewer
          ref="viewerRef"
          :result="currentResult"
          :display-mode="displayMode"
          :show-deformed="showDeformed"
          :deformation-scale="deformationScale"
          :colormap="colormap"
          :show-wireframe="showWireframe"
          @node-click="onNodeClick"
          @ready="onViewerReady"
        />

        <!-- Color legend overlay -->
        <div 
          v-if="currentResult"
          class="absolute bottom-4 right-4 bg-white/90 backdrop-blur rounded-lg p-3 shadow-lg"
        >
          <ColorLegend
            :min="colorMin"
            :max="colorMax"
            :title="legendTitle"
            :colormap="colormap"
            :unit="legendUnit"
          />
        </div>

        <!-- No data placeholder -->
        <div 
          v-if="!currentResult"
          class="absolute inset-0 flex items-center justify-center"
        >
          <div class="text-center text-gray-400">
            <div class="text-4xl mb-2">📊</div>
            <p>暂无仿真结果</p>
            <p class="text-sm mt-1">运行仿真后，结果将显示在这里</p>
          </div>
        </div>
      </div>

      <!-- Control Panel -->
      <div class="w-72 bg-white border-l overflow-y-auto">
        <div class="p-4 space-y-6">
          <!-- Display Mode -->
          <div>
            <h3 class="text-sm font-medium text-gray-700 mb-2">显示模式</h3>
            <div class="space-y-2">
              <label class="flex items-center gap-2 cursor-pointer">
                <input 
                  type="radio" 
                  v-model="displayMode" 
                  value="vonMises"
                  class="text-blue-600"
                />
                <span class="text-sm">Von Mises 应力</span>
              </label>
              <label class="flex items-center gap-2 cursor-pointer">
                <input 
                  type="radio" 
                  v-model="displayMode" 
                  value="displacement"
                  class="text-blue-600"
                />
                <span class="text-sm">位移</span>
              </label>
              <label class="flex items-center gap-2 cursor-pointer">
                <input 
                  type="radio" 
                  v-model="displayMode" 
                  value="stress"
                  class="text-blue-600"
                />
                <span class="text-sm">应力分量</span>
              </label>
            </div>
          </div>

          <!-- Deformation -->
          <div>
            <h3 class="text-sm font-medium text-gray-700 mb-2">变形显示</h3>
            <div class="space-y-3">
              <label class="flex items-center gap-2 cursor-pointer">
                <input 
                  type="checkbox" 
                  v-model="showDeformed"
                  class="rounded text-blue-600"
                />
                <span class="text-sm">显示变形后形状</span>
              </label>
              <div v-if="showDeformed">
                <label class="text-xs text-gray-500 mb-1 block">变形比例: {{ deformationScale.toFixed(1) }}x</label>
                <input 
                  type="range" 
                  v-model.number="deformationScale"
                  min="0.1" 
                  max="10" 
                  step="0.1"
                  class="w-full"
                />
              </div>
            </div>
          </div>

          <!-- Colormap -->
          <div>
            <h3 class="text-sm font-medium text-gray-700 mb-2">颜色图例</h3>
            <select 
              v-model="colormap"
              class="w-full px-3 py-2 border rounded text-sm"
            >
              <option value="viridis">Viridis</option>
              <option value="plasma">Plasma</option>
              <option value="inferno">Inferno</option>
              <option value="jet">Jet</option>
              <option value="rainbow">Rainbow</option>
            </select>
          </div>

          <!-- Display Options -->
          <div>
            <h3 class="text-sm font-medium text-gray-700 mb-2">显示选项</h3>
            <div class="space-y-2">
              <label class="flex items-center gap-2 cursor-pointer">
                <input 
                  type="checkbox" 
                  v-model="showWireframe"
                  class="rounded text-blue-600"
                />
                <span class="text-sm">显示网格线</span>
              </label>
              <label class="flex items-center gap-2 cursor-pointer">
                <input 
                  type="checkbox" 
                  v-model="showAxes"
                  class="rounded text-blue-600"
                />
                <span class="text-sm">显示坐标轴</span>
              </label>
            </div>
          </div>

          <!-- Statistics -->
          <div v-if="currentResult">
            <h3 class="text-sm font-medium text-gray-700 mb-2">统计信息</h3>
            <div class="bg-gray-50 rounded p-3 text-xs space-y-1">
              <div class="flex justify-between">
                <span class="text-gray-500">节点数</span>
                <span class="font-medium">{{ currentResult.nodes.length }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-500">单元数</span>
                <span class="font-medium">{{ currentResult.elements.length }}</span>
              </div>
              <div v-if="currentResult.vonMises">
                <div class="flex justify-between mt-2 pt-2 border-t">
                  <span class="text-gray-500">最大 Von Mises</span>
                  <span class="font-medium">{{ formatValue(colorMax) }} MPa</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-500">最小 Von Mises</span>
                  <span class="font-medium">{{ formatValue(colorMin) }} MPa</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Node Info -->
          <div v-if="selectedNode">
            <h3 class="text-sm font-medium text-gray-700 mb-2">选中节点</h3>
            <div class="bg-gray-50 rounded p-3 text-xs space-y-1">
              <div class="flex justify-between">
                <span class="text-gray-500">ID</span>
                <span class="font-medium">{{ selectedNode.id }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-500">X</span>
                <span>{{ selectedNode.x.toFixed(4) }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-500">Y</span>
                <span>{{ selectedNode.y.toFixed(4) }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-500">Z</span>
                <span>{{ selectedNode.z.toFixed(4) }}</span>
              </div>
              <div v-if="selectedValue !== null" class="flex justify-between mt-2 pt-2 border-t">
                <span class="text-gray-500">{{ legendTitle }}</span>
                <span class="font-medium">{{ formatValue(selectedValue) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 🔗 嵌入到笔记对话框 -->
      <div v-if="showEmbedDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showEmbedDialog = false">
        <div class="bg-white rounded-lg shadow-xl w-[500px]">
          <div class="p-4 border-b flex justify-between items-center">
            <h3 class="text-lg font-semibold text-gray-800 flex items-center gap-2">
              <span>🔗</span>
              <span>嵌入到笔记</span>
            </h3>
            <button @click="showEmbedDialog = false" class="text-gray-500 hover:text-gray-700">✕</button>
          </div>
          <div class="p-4">
            <div class="mb-4">
              <label class="text-sm font-medium text-gray-700 mb-2 block">选择要嵌入的笔记</label>
              <select 
                v-model="selectedEmbedNoteId"
                class="w-full px-3 py-2 border border-gray-300 rounded text-sm"
              >
                <option value="">-- 选择笔记 --</option>
                <option v-for="note in simFiles" :key="note.id" :value="note.id">
                  {{ note.file_name || '无标题笔记' }}
                </option>
              </select>
            </div>
            
            <!-- 嵌入预览 -->
            <div class="bg-gray-50 rounded p-3">
              <div class="flex items-center gap-3">
                <span class="text-2xl">📊</span>
                <div>
                  <div class="text-sm font-medium text-gray-800">
                    {{ projectStore.lastResult ? `仿真结果 (已求解)` : '无仿真结果' }}
                  </div>
                  <div class="text-xs text-gray-500">
                    {{ new Date().toLocaleDateString() }}
                  </div>
                </div>
              </div>
            </div>
          </div>
          <div class="p-4 border-t flex justify-end gap-3">
            <button @click="showEmbedDialog = false" class="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300 text-sm">
              取消
            </button>
            <button 
              @click="embedSimulationToNote" 
              :disabled="!selectedEmbedNoteId || !projectStore.hasResult"
              class="px-4 py-2 bg-orange-500 text-white rounded hover:bg-orange-600 text-sm disabled:opacity-50 disabled:cursor-not-allowed"
            >
              确认嵌入
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import ResultViewer from '../components/simulation/ResultViewer.vue'
import ColorLegend from '../components/simulation/ColorLegend.vue'
import { useProjectStore } from '@/stores/project'
import { useAiStore } from '@/stores/ai'
import * as caeApi from '@/api/cae'
import type { SimulationResult } from '../types'
import type { Material, MeshApiResult } from '@/api/cae'

// Development mode: uncomment to load sample data
// import { generateSampleResult } from '../components/simulation/simulationParser'

const projectStore = useProjectStore()
const aiStore = useAiStore()

onMounted(() => {
  // Update AI context with current simulation state
  aiStore.updateContext({
    currentTool: 'simulation',
    ...(projectStore.hasMesh && {
      simulationMesh: {
        nodeCount: projectStore.currentMesh!.nodes.length,
        elementCount: projectStore.currentMesh!.elements.length
      }
    })
  })
})
const viewerRef = ref<InstanceType<typeof ResultViewer>>()
const currentResult = ref<SimulationResult | null>(null)

// Mesh generation state
const meshDimension = ref<'2d' | '3d'>('3d')
const meshXMin = ref(0)
const meshXMax = ref(100)
const meshYMin = ref(0)
const meshYMax = ref(20)
const meshZMin = ref(0)
const meshZMax = ref(10)
const meshXDiv = ref(20)
const meshYDiv = ref(4)
const meshZDiv = ref(2)
const generatingMesh = ref(false)

// Material state
const materialE = ref(200000) // MPa = 200 GPa
const materialNu = ref(0.3)
const materialDensity = ref(7.85e-9)

// Running state
const runningSolver = ref(false)
const lastError = ref<string | null>(null)

const canRunSolver = computed(() => {
  return projectStore.hasMesh && projectStore.boundaryConditions.fixedBcs.length > 0
})

// Generate mesh
async function generateMesh() {
  generatingMesh.value = true
  lastError.value = null
  
  try {
    let result: MeshApiResult
    
    if (meshDimension.value === '2d') {
      result = await caeApi.generate2dMesh(
        meshXMin.value, meshXMax.value, meshXDiv.value,
        meshYMin.value, meshYMax.value, meshYDiv.value,
        'C2D4'
      )
    } else {
      result = await caeApi.generate3dMesh(
        meshXMin.value, meshXMax.value, meshXDiv.value,
        meshYMin.value, meshYMax.value, meshYDiv.value,
        meshZMin.value, meshZMax.value, meshZDiv.value,
        'C3D8'
      )
    }
    
    projectStore.setMesh(result)
    // Clear old BCs when generating new mesh
    projectStore.clearBoundaryConditions()
    projectStore.clearResult()
  } catch (e: any) {
    lastError.value = String(e)
  } finally {
    generatingMesh.value = false
  }
}

// Apply cantilever beam preset
async function applyCantileverPreset() {
  if (!projectStore.currentMesh) return
  
  try {
    // Create fixed BC on left face (x=0)
    const fixedBc = await caeApi.createCantileverFixedBc(projectStore.currentMesh.nodes)
    projectStore.addFixedBc(fixedBc)
    
    // Create point load on right face top
    const load = await caeApi.createCantileverPointLoad(
      projectStore.currentMesh.nodes,
      meshXMax.value,
      1000 // 1000 N
    )
    projectStore.addPointLoad(load)
  } catch (e: any) {
    lastError.value = String(e)
  }
}

// Get current material
function getCurrentMaterial(): Material {
  return {
    id: 1,
    name: 'Steel',
    elastic_modulus: materialE.value,
    poisson_ratio: materialNu.value,
    density: materialDensity.value
  }
}

// Run solver
async function runSolver() {
  if (!projectStore.currentMesh) return
  
  runningSolver.value = true
  lastError.value = null
  
  try {
    // Get all data from store
    const nodes = projectStore.currentMesh.nodes
    const elements = projectStore.currentMesh.elements
    const material = getCurrentMaterial()
    const fixedBc = projectStore.boundaryConditions.fixedBcs[0] // for simplicity, take first
    const pointLoad = projectStore.boundaryConditions.pointLoads[0] || null
    const uniformLoads = projectStore.boundaryConditions.uniformLoads
    
    // Generate INP to temp path
    const tempPath = `/tmp/caelab_job.inp`
    const inpResult = await caeApi.generateCompleteInp(
      nodes, elements, material, fixedBc, pointLoad, uniformLoads, tempPath
    )
    
    console.log('INP generated:', inpResult)
    
    // Run solver
    const workingDir = '/tmp'
    const solverResult = await caeApi.runSolver(tempPath, workingDir)
    
    if (!solverResult.success) {
      throw new Error(solverResult.errors.join(', '))
    }
    
    // Parse FRD result
    const frdPath = `/tmp/caelab_job.frd`
    const resultSet = await caeApi.parseFrdFile(frdPath)
    
    projectStore.setResult(resultSet)
    
    // TODO: Convert ResultSet to SimulationResult for viewer
    // For now, still use sample data structure
    // currentResult.value = convertResultSet(resultSet)
  } catch (e: any) {
    lastError.value = String(e)
  } finally {
    runningSolver.value = false
  }
}

// Reset all
function resetAll() {
  projectStore.clearMesh()
  projectStore.clearBoundaryConditions()
  projectStore.clearResult()
  currentResult.value = null
  lastError.value = null
}
const displayMode = ref<'displacement' | 'stress' | 'vonMises'>('vonMises')
const showDeformed = ref(false)
const deformationScale = ref(1.0)
const colormap = ref<'viridis' | 'plasma' | 'inferno' | 'jet' | 'rainbow'>('viridis')
const showWireframe = ref(false)
const showAxes = ref(true)
const selectedNode = ref<{ id: number; x: number; y: number; z: number } | null>(null)
const selectedValue = ref<number | null>(null)

const legendTitle = computed(() => {
  switch (displayMode.value) {
    case 'vonMises': return 'Von Mises 应力'
    case 'displacement': return '位移'
    case 'stress': return '应力'
    default: return '值'
  }
})

const legendUnit = computed(() => {
  return displayMode.value === 'displacement' ? 'mm' : 'MPa'
})

const colorMin = computed(() => {
  const data = viewerRef.value?.getColorData?.()
  return data?.minVal ?? 0
})

const colorMax = computed(() => {
  const data = viewerRef.value?.getColorData?.()
  return data?.maxVal ?? 100
})

const formatValue = (v: number): string => {
  if (Math.abs(v) >= 1e6) return (v / 1e6).toFixed(2) + 'M'
  if (Math.abs(v) >= 1e3) return (v / 1e3).toFixed(2) + 'k'
  return v.toFixed(2)
}

function onViewerReady() {
  console.log('Result viewer ready')
}

function onNodeClick(nodeId: number, value: number) {
  const node = currentResult.value?.nodes.find(n => n.id === nodeId)
  if (node) {
    selectedNode.value = node
    selectedValue.value = value
  }
}

function exportScreenshot() {
  // TODO: Implement screenshot export
  console.log('Export screenshot - to be implemented with canvas capture')
}

// ========== 嵌入到笔记功能 ==========
const showEmbedDialog = ref(false)
const selectedEmbedNoteId = ref<string>('')
const simFiles = ref<any[]>([])

async function loadFilesForEmbed() {
  if (projectStore.currentProject) {
    try {
      const { listFiles } = await import('@/api')
      const allFiles = await listFiles(projectStore.currentProject.id)
      simFiles.value = allFiles.filter((f: any) => f.file_type === 'note')
    } catch (e) {
      console.error('加载笔记列表失败:', e)
    }
  }
}

function showEmbedToNoteDialog() {
  if (simFiles.value.length === 0) {
    loadFilesForEmbed()
  }
  if (!projectStore.currentNoteId && simFiles.value.length > 0) {
    selectedEmbedNoteId.value = simFiles.value[0]?.id || ''
  } else {
    selectedEmbedNoteId.value = projectStore.currentNoteId || ''
  }
  showEmbedDialog.value = true
}

async function embedSimulationToNote() {
  if (!selectedEmbedNoteId.value) return

  const embedRecord = projectStore.addEmbedRecord({
    type: 'simulation',
    targetId: 'current-simulation',
    targetName: projectStore.lastResult 
      ? `仿真结果 (已求解)`
      : '无仿真结果',
    noteId: selectedEmbedNoteId.value
  })

  console.log('仿真结果已嵌入到笔记:', embedRecord)
  showEmbedDialog.value = false
  
  alert('✓ 仿真结果已成功嵌入到笔记！\n\n在笔记中点击嵌入卡片可跳转到仿真界面。')
}

// Watch for project changes
watch(() => projectStore.currentProject, (project) => {
  if (project) {
    loadFilesForEmbed()
  }
})

// Watch display mode change
watch(displayMode, () => {
  if (currentResult.value) {
    // Result viewer will update automatically via props
  }
})
</script>