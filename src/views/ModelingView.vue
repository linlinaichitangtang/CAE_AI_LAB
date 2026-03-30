<template>
  <div class="modeling-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">3D建模</h2>
        <p class="text-xs text-[var(--text-muted)]">几何建模、外部模型导入、参数化编辑</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- 🔗 嵌入到笔记 -->
        <button 
          @click="showEmbedToNoteDialog"
          :disabled="!projectStore.currentNoteId"
          class="btn btn-ghost text-xs border border-orange-300 text-orange-600 hover:bg-orange-50 flex items-center gap-1"
          title="将当前建模内容嵌入到笔记"
        >
          <span>🔗</span>
          <span>嵌入到笔记</span>
        </button>
        <button @click="importModel" class="btn btn-ghost text-xs">
          <span class="mr-1">📁</span> 导入
        </button>
        <button @click="openAiDialog = true" class="btn btn-ghost text-xs border border-purple-300 text-purple-600 hover:bg-purple-50">
          <span class="mr-1">🤖</span> AI生成
        </button>
        <button @click="showAddGeometry = true" class="btn btn-primary text-xs">
          <span class="mr-1">+</span> 添加几何体
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left: Scene Tree -->
      <div class="w-56 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col">
        <div class="px-3 py-2 text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider border-b border-[var(--border-subtle)]">
          场景树
        </div>
        <div class="flex-1 overflow-y-auto p-3">
          <div v-if="geometryItems.length === 0" class="text-center py-8">
            <div class="text-4xl mb-2 opacity-30">📐</div>
            <p class="text-xs text-[var(--text-muted)]">暂无几何体</p>
            <p class="text-[10px] text-[var(--text-muted)] mt-1">点击"添加几何体"开始</p>
          </div>
          <div v-else class="space-y-1">
            <div 
              v-for="(item, idx) in geometryItems" 
              :key="idx"
              @click="selectGeometry(idx)"
              :class="['p-2 rounded cursor-pointer text-xs transition', 
                selectedGeometryIdx === idx 
                  ? 'bg-[var(--accent)]/10 text-[var(--accent)]' 
                  : 'hover:bg-[var(--bg-hover)] text-[var(--text-primary)]'
              ]"
            >
              <div class="flex items-center gap-2">
                <span class="text-base">{{ item.icon }}</span>
                <span class="truncate">{{ item.name }}</span>
              </div>
              <div class="text-[10px] text-[var(--text-muted)] mt-0.5 pl-6">
                {{ item.dimensions }}
              </div>
            </div>
          </div>
        </div>
        <!-- Add geometry button -->
        <div class="p-3 border-t border-[var(--border-subtle)]">
          <select 
            v-model="selectedGeometryType" 
            @change="onGeometryTypeChange"
            class="input w-full text-xs"
          >
            <option value="">添加基本体...</option>
            <option value="box">立方体</option>
            <option value="sphere">球体</option>
            <option value="cylinder">圆柱体</option>
            <option value="cone">圆锥体</option>
            <option value="torus">圆环体</option>
          </select>
        </div>
      </div>

      <!-- Center: 3D Viewport -->
      <div class="flex-1 relative flex flex-col">
        <!-- Canvas Container -->
        <div ref="canvasContainer" class="flex-1"></div>
        
        <!-- Toolbar -->
        <div class="h-10 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] flex items-center px-4 gap-4">
          <span class="text-[10px] text-[var(--text-muted)] uppercase tracking-wider">工具</span>
          <div class="flex items-center gap-1">
            <button 
              @click="currentTool = 'select'" 
              :class="['icon-btn w-8 h-8', currentTool === 'select' ? 'active' : '']" 
              title="选择 (S)"
            >
              <span class="text-sm">⊹</span>
            </button>
            <button 
              @click="currentTool = 'move'" 
              :class="['icon-btn w-8 h-8', currentTool === 'move' ? 'active' : '']" 
              title="移动 (M)"
            >
              <span class="text-sm">✥</span>
            </button>
            <button 
              @click="currentTool = 'rotate'" 
              :class="['icon-btn w-8 h-8', currentTool === 'rotate' ? 'active' : '']" 
              title="旋转 (R)"
            >
              <span class="text-sm">↻</span>
            </button>
            <button 
              @click="currentTool = 'scale'" 
              :class="['icon-btn w-8 h-8', currentTool === 'scale' ? 'active' : '']" 
              title="缩放 (E)"
            >
              <span class="text-sm">⤢</span>
            </button>
          </div>
          <div class="flex-1"></div>
          <button @click="generateMesh" :disabled="!canGenerateMesh || generating" class="btn btn-primary text-xs">
            <span v-if="generating" class="mr-1 animate-spin">⟳</span>
            <span v-else class="mr-1">⚙</span>
            生成网格
          </button>
          <span class="text-[10px] text-[var(--text-muted)]">网格: {{ showGrid ? '开启' : '关闭' }}</span>
          <button @click="showGrid = !showGrid" class="icon-btn w-7 h-7">
            <span class="text-xs">{{ showGrid ? '▦' : '▢' }}</span>
          </button>
        </div>
      </div>

      <!-- Right: Properties Panel -->
      <div class="w-64 bg-[var(--bg-surface)] border-l border-[var(--border-subtle)] flex flex-col">
        <div class="px-3 py-2 text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider border-b border-[var(--border-subtle)]">
          属性
        </div>
        <div class="flex-1 overflow-y-auto p-4">
          <!-- No selection state -->
          <div v-if="selectedGeometryIdx === null" class="text-center py-8">
            <div class="text-3xl mb-2 opacity-30">ℹ️</div>
            <p class="text-xs text-[var(--text-muted)]">选择对象以查看属性</p>
          </div>
          
          <!-- Box Properties -->
          <div v-else-if="currentGeometry?.type === 'box'" class="space-y-4">
            <div>
              <h4 class="text-xs font-medium text-[var(--text-primary)] mb-3">几何参数</h4>
              <div class="space-y-3">
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">长度 X (mm)</label>
                  <input 
                    type="number" 
                    v-model.number="boxParams.width" 
                    @input="updateBoxGeometry"
                    class="input w-full text-xs"
                    min="0.1"
                  />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">宽度 Y (mm)</label>
                  <input 
                    type="number" 
                    v-model.number="boxParams.height" 
                    @input="updateBoxGeometry"
                    class="input w-full text-xs"
                    min="0.1"
                  />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">高度 Z (mm)</label>
                  <input 
                    type="number" 
                    v-model.number="boxParams.depth" 
                    @input="updateBoxGeometry"
                    class="input w-full text-xs"
                    min="0.1"
                  />
                </div>
              </div>
            </div>

            <div class="border-t border-[var(--border-subtle)] pt-4">
              <h4 class="text-xs font-medium text-[var(--text-primary)] mb-3">网格细分</h4>
              <div class="space-y-3">
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">X 方向分段</label>
                  <input 
                    type="number" 
                    v-model.number="boxParams.divX" 
                    @input="updateBoxGeometry"
                    class="input w-full text-xs"
                    min="1"
                    max="100"
                  />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">Y 方向分段</label>
                  <input 
                    type="number" 
                    v-model.number="boxParams.divY" 
                    @input="updateBoxGeometry"
                    class="input w-full text-xs"
                    min="1"
                    max="100"
                  />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">Z 方向分段</label>
                  <input 
                    type="number" 
                    v-model.number="boxParams.divZ" 
                    @input="updateBoxGeometry"
                    class="input w-full text-xs"
                    min="1"
                    max="100"
                  />
                </div>
              </div>
            </div>

            <div class="border-t border-[var(--border-subtle)] pt-4">
              <h4 class="text-xs font-medium text-[var(--text-primary)] mb-3">变换</h4>
              <div class="space-y-3">
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">位置 X</label>
                  <input 
                    type="number" 
                    v-model.number="boxParams.posX" 
                    @input="updateBoxPosition"
                    class="input w-full text-xs"
                  />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">位置 Y</label>
                  <input 
                    type="number" 
                    v-model.number="boxParams.posY" 
                    @input="updateBoxPosition"
                    class="input w-full text-xs"
                  />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">位置 Z</label>
                  <input 
                    type="number" 
                    v-model.number="boxParams.posZ" 
                    @input="updateBoxPosition"
                    class="input w-full text-xs"
                  />
                </div>
                <div class="grid grid-cols-2 gap-2">
                  <div>
                    <label class="text-[10px] text-[var(--text-muted)] block mb-1">旋转 X</label>
                    <input 
                      type="number" 
                      v-model.number="boxParams.rotX" 
                      @input="updateBoxRotation"
                      class="input w-full text-xs"
                    />
                  </div>
                  <div>
                    <label class="text-[10px] text-[var(--text-muted)] block mb-1">旋转 Y</label>
                    <input 
                      type="number" 
                      v-model.number="boxParams.rotY" 
                      @input="updateBoxRotation"
                      class="input w-full text-xs"
                    />
                  </div>
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">旋转 Z</label>
                  <input 
                    type="number" 
                    v-model.number="boxParams.rotZ" 
                    @input="updateBoxRotation"
                    class="input w-full text-xs"
                  />
                </div>
              </div>
            </div>

            <!-- Display Results Overlay -->
            <div v-if="projectStore.hasResult && resultMeshGroup" class="border-t border-[var(--border-subtle)] pt-4">
              <h4 class="text-xs font-medium text-[var(--text-primary)] mb-3">结果显示</h4>
              <div class="space-y-2">
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">显示类型</label>
                  <select v-model="resultDisplayMode" class="input w-full text-xs">
                    <option value="stress">应力云图</option>
                    <option value="displacement">位移云图</option>
                    <option value="none">无</option>
                  </select>
                </div>
                <button 
                  v-if="resultDisplayMode !== 'none'"
                  @click="toggleResultMesh"
                  class="btn btn-ghost text-xs w-full"
                >
                  {{ resultMeshGroup.visible ? '隐藏结果' : '显示结果' }}
                </button>
              </div>
            </div>

            <button 
              @click="deleteCurrentGeometry"
              class="btn btn-ghost text-xs w-full text-red-500 hover:bg-red-50 mt-4"
            >
              删除几何体
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- AI生成几何体对话框 -->
    <div v-if="openAiDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="openAiDialog = false">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-[550px] flex flex-col max-h-[80vh]">
        <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
          <h3 class="text-lg font-semibold text-gray-800 dark:text-white">
            🤖 AI生成几何体
          </h3>
          <button @click="openAiDialog = false" class="text-gray-500 hover:text-gray-700 dark:text-gray-400">✕</button>
        </div>
        <div class="p-4 flex-1 overflow-auto">
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
            输入自然语言描述，AI将自动解析并创建几何体
          </p>
          <textarea
            v-model="aiPrompt"
            placeholder="例如：帮我创建一个长100宽20高10的悬臂梁"
            class="input w-full text-sm min-h-[80px] resize-y mb-4"
            :disabled="aiGenerating"
          />
          <!-- AI思考过程 -->
          <div v-if="aiGenerating" class="bg-gray-50 dark:bg-gray-700 rounded p-3 mb-4">
            <div class="text-xs text-gray-500 dark:text-gray-400 mb-2">AI 思考中...</div>
            <div class="text-xs text-gray-600 dark:text-gray-300 font-mono whitespace-pre-wrap">
              {{ aiThinking }}
            </div>
          </div>
          <!-- 解析结果 -->
          <div v-if="aiParsedResult" class="bg-blue-50 dark:bg-blue-900/20 rounded p-3">
            <div class="text-xs font-semibold text-blue-600 dark:text-blue-400 mb-2">解析结果</div>
            <div class="text-xs text-gray-600 dark:text-gray-300 space-y-1">
              <div>类型: {{ aiParsedResult.type }}</div>
              <div>尺寸: 长{{ aiParsedResult.width }} × 宽{{ aiParsedResult.height }} × 高{{ aiParsedResult.depth }}</div>
              <div>分段: {{ aiParsedResult.divX }} × {{ aiParsedResult.divY }} × {{ aiParsedResult.divZ }}</div>
            </div>
          </div>
          <!-- 错误信息 -->
          <div v-if="aiError" class="bg-red-50 dark:bg-red-900/20 rounded p-3 text-xs text-red-600 dark:text-red-400">
            {{ aiError }}
          </div>
        </div>
        <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-3">
          <button @click="openAiDialog = false" class="btn btn-ghost text-xs">取消</button>
          <button @click="generateFromAi" :disabled="aiGenerating || !aiPrompt.trim()" class="btn btn-primary text-xs">
            <span v-if="aiGenerating">生成中...</span>
            <span v-else>生成几何体</span>
          </button>
        </div>
      </div>
    </div>

    <!-- 🔗 嵌入到笔记对话框 -->
    <div v-if="showEmbedDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showEmbedDialog = false">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-[500px]">
        <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
          <h3 class="text-lg font-semibold text-gray-800 dark:text-white flex items-center gap-2">
            <span>🔗</span>
            <span>嵌入到笔记</span>
          </h3>
          <button @click="showEmbedDialog = false" class="text-gray-500 hover:text-gray-700 dark:text-gray-400">✕</button>
        </div>
        <div class="p-4">
          <div class="mb-4">
            <label class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2 block">选择要嵌入的笔记</label>
            <select 
              v-model="selectedEmbedNoteId"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-800 dark:text-white text-sm"
            >
              <option value="">-- 选择笔记 --</option>
              <option v-for="note in files" :key="note.id" :value="note.id">
                {{ note.file_name || '无标题笔记' }}
              </option>
            </select>
          </div>
          
          <!-- 嵌入预览 -->
          <div class="bg-gray-50 dark:bg-gray-900 rounded p-3">
            <div class="flex items-center gap-3">
              <span class="text-2xl">📐</span>
              <div>
                <div class="text-sm font-medium text-gray-800 dark:text-white">
                  {{ geometryItems.length > 0 ? `3D建模项目 (${geometryItems.length}个几何体)` : '空建模项目' }}
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  建模数据 • {{ new Date().toLocaleDateString() }}
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-3">
          <button @click="showEmbedDialog = false" class="px-4 py-2 bg-gray-200 dark:bg-gray-700 rounded hover:bg-gray-300 dark:hover:bg-gray-600 text-sm">
            取消
          </button>
          <button 
            @click="embedModelToNote" 
            :disabled="!selectedEmbedNoteId"
            class="px-4 py-2 bg-orange-500 text-white rounded hover:bg-orange-600 text-sm disabled:opacity-50 disabled:cursor-not-allowed"
          >
            确认嵌入
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, watch } from 'vue'
import { useRouter } from 'vue-router'
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import { invoke } from '@tauri-apps/api/tauri'
import { useProjectStore } from '@/stores/project'
import { useAiStore } from '@/stores/ai'
import type { GeometryItem } from '../types'

const aiStore = useAiStore()

const router = useRouter()
const projectStore = useProjectStore()

// Canvas and Three.js refs
const canvasContainer = ref<HTMLDivElement | null>(null)
let renderer: THREE.WebGLRenderer | null = null
let scene: THREE.Scene | null = null
let camera: THREE.PerspectiveCamera | null = null
let controls: OrbitControls | null = null
let animationFrameId: number | null = null

// Geometry state
const geometryItems = ref<GeometryItem[]>([])
const selectedGeometryIdx = ref<number | null>(null)
const selectedGeometryType = ref('')
const showAddGeometry = ref(false)

// AI 生成几何体
const openAiDialog = ref(false)
const aiPrompt = ref('')
const aiGenerating = ref(false)
const aiThinking = ref('')
const aiParsedResult = ref<{
  type: string
  width: number
  height: number
  depth: number
  divX: number
  divY: number
  divZ: number
} | null>(null)
const aiError = ref('')

// Box parameters
const boxParams = ref({
  width: 100,
  height: 50,
  depth: 30,
  divX: 10,
  divY: 5,
  divZ: 3,
  posX: 0,
  posY: 0,
  posZ: 0,
  rotX: 0,
  rotY: 0,
  rotZ: 0
})

// Tool state
const currentTool = ref<'select' | 'move' | 'rotate' | 'scale'>('select')
const showGrid = ref(true)

// Generation state
const generating = ref(false)

// Result display
const resultDisplayMode = ref<'stress' | 'displacement' | 'none'>('stress')
let resultMeshGroup: THREE.Group | null = null

// Current geometry computed
const currentGeometry = computed(() => {
  if (selectedGeometryIdx.value !== null) {
    return geometryItems.value[selectedGeometryIdx.value]
  }
  return null
})

// Can generate mesh
const canGenerateMesh = computed(() => {
  return geometryItems.value.length > 0 && !generating.value
})

// Three.js initialization
function initThreeJS() {
  if (!canvasContainer.value) return

  // Scene
  scene = new THREE.Scene()
  scene.background = new THREE.Color(0x1a1a2e)

  // Camera
  const aspect = canvasContainer.value.clientWidth / canvasContainer.value.clientHeight
  camera = new THREE.PerspectiveCamera(45, aspect, 0.1, 10000)
  camera.position.set(200, 150, 200)
  camera.lookAt(0, 0, 0)

  // Renderer
  renderer = new THREE.WebGLRenderer({ antialias: true })
  renderer.setSize(canvasContainer.value.clientWidth, canvasContainer.value.clientHeight)
  renderer.setPixelRatio(window.devicePixelRatio)
  canvasContainer.value.appendChild(renderer.domElement)

  // Controls
  controls = new OrbitControls(camera, renderer.domElement)
  controls.enableDamping = true
  controls.dampingFactor = 0.05
  controls.screenSpacePanning = true
  controls.minDistance = 10
  controls.maxDistance = 2000

  // Lights
  const ambientLight = new THREE.AmbientLight(0x404040, 1)
  scene.add(ambientLight)

  const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8)
  directionalLight.position.set(100, 100, 50)
  scene.add(directionalLight)

  const directionalLight2 = new THREE.DirectionalLight(0xffffff, 0.4)
  directionalLight2.position.set(-100, -50, -50)
  scene.add(directionalLight2)

  // Grid
  createGrid()

  // Axes helper
  const axesHelper = new THREE.AxesHelper(50)
  scene.add(axesHelper)

  // Animation loop
  function animate() {
    animationFrameId = requestAnimationFrame(animate)
    if (controls) {
      controls.update()
    }
    if (renderer && scene && camera) {
      renderer.render(scene, camera)
    }
  }
  animate()

  // Handle resize
  window.addEventListener('resize', onWindowResize)
}

function onWindowResize() {
  if (!canvasContainer.value || !camera || !renderer) return
  
  const width = canvasContainer.value.clientWidth
  const height = canvasContainer.value.clientHeight
  
  camera.aspect = width / height
  camera.updateProjectionMatrix()
  renderer.setSize(width, height)
}

function createGrid() {
  if (!scene) return
  
  const gridHelper = new THREE.GridHelper(500, 50, 0x444444, 0x222222)
  gridHelper.rotation.x = Math.PI / 2
  gridHelper.name = 'grid'
  scene.add(gridHelper)
}

function toggleGrid(visible: boolean) {
  if (!scene) return
  const grid = scene.getObjectByName('grid')
  if (grid) {
    grid.visible = visible
  }
}

watch(showGrid, (val) => {
  toggleGrid(val)
})

// Create geometry
function createBox(params: typeof boxParams.value) {
  const geometry = new THREE.BoxGeometry(params.width, params.height, params.depth)
  
  // Create wireframe for visualization
  const edges = new THREE.EdgesGeometry(geometry)
  const lineMaterial = new THREE.LineBasicMaterial({ color: 0x00ffff, linewidth: 1 })
  const wireframe = new THREE.LineSegments(edges, lineMaterial)
  
  // Create transparent solid for better visibility
  const material = new THREE.MeshPhongMaterial({
    color: 0x00aaff,
    transparent: true,
    opacity: 0.3,
    side: THREE.DoubleSide
  })
  const mesh = new THREE.Mesh(geometry, material)
  
  const group = new THREE.Group()
  group.add(mesh)
  group.add(wireframe)
  group.position.set(params.posX, params.posY, params.posZ)
  group.rotation.set(
    THREE.MathUtils.degToRad(params.rotX),
    THREE.MathUtils.degToRad(params.rotY),
    THREE.MathUtils.degToRad(params.rotZ)
  )
  
  group.userData = {
    type: 'box',
    params: { ...params },
    mesh: mesh,
    wireframe: wireframe,
    geometry: geometry
  }
  
  return group
}

function addGeometryToScene(type: string) {
  if (!scene) return
  
  let group: THREE.Group
  
  switch (type) {
    case 'box':
      group = createBox(boxParams.value)
      geometryItems.value.push({
        type: 'box',
        name: `立方体 ${geometryItems.value.filter(g => g.type === 'box').length + 1}`,
        icon: '⬜',
        dimensions: `${boxParams.value.width}×${boxParams.value.height}×${boxParams.value.depth}`,
        group: group,
        params: { ...boxParams.value }
      })
      break
    default:
      return
  }
  
  scene.add(group)
  
  // Select the new geometry
  selectedGeometryIdx.value = geometryItems.value.length - 1
  selectedGeometryType.value = ''
}

function onGeometryTypeChange() {
  if (selectedGeometryType.value) {
    addGeometryToScene(selectedGeometryType.value)
  }
}

function selectGeometry(idx: number) {
  // Deselect previous
  if (selectedGeometryIdx.value !== null && scene) {
    const prevGeom = geometryItems.value[selectedGeometryIdx.value]
    if (prevGeom.group) {
      prevGeom.group.children.forEach((child: THREE.Object3D) => {
        if (child instanceof THREE.Mesh) {
          (child.material as THREE.MeshPhongMaterial).opacity = 0.3
        }
      })
    }
  }
  
  // Select new
  selectedGeometryIdx.value = idx
  const geom = geometryItems.value[idx]
  
  // Update box params from selected geometry
  if (geom.type === 'box' && geom.params) {
    boxParams.value = { ...geom.params }
  }
  
  // Highlight selected
  if (geom.group) {
    geom.group.children.forEach((child: THREE.Object3D) => {
      if (child instanceof THREE.Mesh) {
        (child.material as THREE.MeshPhongMaterial).opacity = 0.5
      }
    })
  }
  
  // Center camera on selected object
  if (geom.group && camera && controls) {
    const box = new THREE.Box3().setFromObject(geom.group)
    const center = box.getCenter(new THREE.Vector3())
    controls.target.copy(center)
  }
}

function updateBoxGeometry() {
  if (selectedGeometryIdx.value === null) return
  
  const geom = geometryItems.value[selectedGeometryIdx.value]
  if (geom.type !== 'box' || !scene) return
  
  // Remove old group
  scene.remove(geom.group!)
  
  // Create new geometry with updated params
  const newGroup = createBox(boxParams.value)
  scene.add(newGroup)
  
  // Update stored item
  geom.group = newGroup
  geom.dimensions = `${boxParams.value.width}×${boxParams.value.height}×${boxParams.value.depth}`
  geom.params = { ...boxParams.value }
  
  // Select it
  selectGeometry(selectedGeometryIdx.value)
}

function updateBoxPosition() {
  if (selectedGeometryIdx.value === null) return
  
  const geom = geometryItems.value[selectedGeometryIdx.value]
  if (!geom.group) return
  
  geom.group.position.set(boxParams.value.posX, boxParams.value.posY, boxParams.value.posZ)
  geom.params!.posX = boxParams.value.posX
  geom.params!.posY = boxParams.value.posY
  geom.params!.posZ = boxParams.value.posZ
}

function updateBoxRotation() {
  if (selectedGeometryIdx.value === null) return
  
  const geom = geometryItems.value[selectedGeometryIdx.value]
  if (!geom.group) return
  
  geom.group.rotation.set(
    THREE.MathUtils.degToRad(boxParams.value.rotX),
    THREE.MathUtils.degToRad(boxParams.value.rotY),
    THREE.MathUtils.degToRad(boxParams.value.rotZ)
  )
  geom.params!.rotX = boxParams.value.rotX
  geom.params!.rotY = boxParams.value.rotY
  geom.params!.rotZ = boxParams.value.rotZ
}

function deleteCurrentGeometry() {
  if (selectedGeometryIdx.value === null) return
  
  const geom = geometryItems.value[selectedGeometryIdx.value]
  if (geom.group && scene) {
    scene.remove(geom.group)
  }
  
  geometryItems.value.splice(selectedGeometryIdx.value, 1)
  selectedGeometryIdx.value = null
}

function importModel() {
  // TODO: Implement model import via Tauri file dialog
  console.log('Import model - to be implemented')
}

// AI生成几何体
async function generateFromAi() {
  if (!aiPrompt.value.trim() || aiGenerating.value) return

  aiGenerating.value = true
  aiThinking.value = ''
  aiParsedResult.value = null
  aiError.value = ''

  try {
    // 构建提示词，让AI解析几何体参数
    const prompt = `你是3D建模助手。用户想要创建几何体，请从以下描述中提取几何参数：

"${aiPrompt.value}"

请以JSON格式返回以下参数（如果参数未提及，使用合理默认值）：
{
  "type": "box|sphere|cylinder|cone|torus",
  "width": 数值（X方向长度）,
  "height": 数值（Y方向长度）,
  "depth": 数值（Z方向高度）,
  "divX": 数值（X方向分段数）,
  "divY": 数值（Y方向分段数）,
  "divZ": 数值（Z方向分段数）
}

注意：如果描述的是悬臂梁，通常是细长的box，width代表长度，height代表宽度，depth代表高度。只返回JSON，不要其他内容。`

    aiThinking.value = '正在调用AI分析描述...'

    // 调用后端AI
    const response = await invoke<string>('ai_chat_completion', {
      messages: [{ role: 'user', content: prompt }],
      config: aiStore.config
    })

    aiThinking.value = '正在解析响应...'

    // 尝试解析JSON响应
    const jsonMatch = response.match(/\{[\s\S]*\}/)
    if (!jsonMatch) {
      throw new Error('AI响应格式错误，无法解析参数')
    }

    const parsed = JSON.parse(jsonMatch[0])

    // 验证和设置默认值
    const result = {
      type: parsed.type || 'box',
      width: parseFloat(parsed.width) || 100,
      height: parseFloat(parsed.height) || 50,
      depth: parseFloat(parsed.depth) || 30,
      divX: Math.max(1, parseInt(parsed.divX) || 10),
      divY: Math.max(1, parseInt(parsed.divY) || 5),
      divZ: Math.max(1, parseInt(parsed.divZ) || 3)
    }

    aiParsedResult.value = result
    aiThinking.value = '解析完成！'

    // 更新box参数并创建几何体
    boxParams.value = {
      width: result.width,
      height: result.height,
      depth: result.depth,
      divX: result.divX,
      divY: result.divY,
      divZ: result.divZ,
      posX: 0,
      posY: 0,
      posZ: 0,
      rotX: 0,
      rotY: 0,
      rotZ: 0
    }

    // 创建几何体
    if (result.type === 'box' && scene) {
      addGeometryToScene('box')
    }

    // 关闭对话框
    setTimeout(() => {
      openAiDialog.value = false
      aiPrompt.value = ''
      aiParsedResult.value = null
    }, 1500)

  } catch (e: any) {
    console.error('AI生成失败:', e)
    aiError.value = `解析失败: ${String(e)}`
  } finally {
    aiGenerating.value = false
  }
}

// Generate mesh and save to store
async function generateMesh() {
  if (!canGenerateMesh.value || !projectStore.currentMesh) return
  
  generating.value = true
  
  try {
    // Import CAE API
    const { generate3dMesh } = await import('@/api/cae')
    
    // Get the first box geometry for mesh generation
    const firstBox = geometryItems.value.find(g => g.type === 'box')
    if (!firstBox || !firstBox.params) {
      throw new Error('No valid geometry found')
    }
    
    const params = firstBox.params
    
    // Generate 3D mesh via backend API
    const meshResult = await generate3dMesh(
      params.posX, params.posX + params.width, params.divX,
      params.posY, params.posY + params.height, params.divY,
      params.posZ, params.posZ + params.depth, params.divZ,
      'C3D8'
    )
    
    // Save to project store
    projectStore.setMesh(meshResult)
    
    // Clear old results when generating new mesh
    projectStore.clearBoundaryConditions()
    projectStore.clearResult()
    
    // Show success feedback
    console.log(`Mesh generated: ${meshResult.nodes.length} nodes, ${meshResult.elements.length} elements`)
    
    // Navigate to simulation view
    projectStore.setActiveTool('simulation')
    router.push('/simulation')
    
  } catch (error) {
    console.error('Mesh generation failed:', error)
    // TODO: Show error toast
  } finally {
    generating.value = false
  }
}

// Result visualization
function createResultMesh() {
  if (!projectStore.lastResult || !scene) return
  
  // Clear existing result mesh
  if (resultMeshGroup) {
    scene.remove(resultMeshGroup)
    resultMeshGroup = null
  }
  
  // TODO: Create result visualization from projectStore.lastResult
  // This would typically:
  // 1. Read ResultSet data from store
  // 2. Map node values to colors using colormap
  // 3. Create a new mesh with vertex colors
  // 4. Add to scene
  
  // For now, placeholder
  console.log('Creating result mesh from', projectStore.lastResult)
}

// ========== 嵌入到笔记功能 ==========
const showEmbedDialog = ref(false)
const selectedEmbedNoteId = ref<string>('')

function showEmbedToNoteDialog() {
  if (!projectStore.currentNoteId && files.value.length > 0) {
    // 如果没有当前笔记，自动选择第一个
    selectedEmbedNoteId.value = files.value[0]?.id || ''
  } else {
    selectedEmbedNoteId.value = projectStore.currentNoteId || ''
  }
  showEmbedDialog.value = true
}

async function embedModelToNote() {
  if (!selectedEmbedNoteId.value) return

  // 构建嵌入记录
  const embedRecord = projectStore.addEmbedRecord({
    type: 'model',
    targetId: 'current-model',
    targetName: geometryItems.value.length > 0 
      ? `3D模型 (${geometryItems.value.length}个几何体)`
      : '空建模项目',
    noteId: selectedEmbedNoteId.value
  })

  console.log('建模已嵌入到笔记:', embedRecord)
  showEmbedDialog.value = false
  
  // 提示用户
  alert('✓ 建模已成功嵌入到笔记！\n\n在笔记中点击嵌入卡片可跳转到建模界面。')
}

// 监听项目ID变化，尝试加载笔记列表
import { ref as vueRef } from 'vue'
const files = vueRef<any[]>([])
const loadFilesForEmbed = async () => {
  if (projectStore.currentProject) {
    try {
      const { listFiles } = await import('@/api')
      const allFiles = await listFiles(projectStore.currentProject.id)
      files.value = allFiles.filter((f: any) => f.file_type === 'note')
    } catch (e) {
      console.error('加载笔记列表失败:', e)
    }
  }
}

// Watch project changes
watch(() => projectStore.currentProject, (project) => {
  if (project) {
    loadFilesForEmbed()
  }
})

function toggleResultMesh() {
  if (resultMeshGroup) {
    resultMeshGroup.visible = !resultMeshGroup.visible
  }
}

watch(() => projectStore.lastResult, (newResult) => {
  if (newResult) {
    nextTick(() => {
      createResultMesh()
      resultDisplayMode.value = 'stress'
    })
  }
})

watch(resultDisplayMode, (mode) => {
  if (mode === 'none') {
    if (resultMeshGroup) {
      resultMeshGroup.visible = false
    }
  } else if (projectStore.lastResult) {
    createResultMesh()
  }
})

// Lifecycle
onMounted(() => {
  nextTick(() => {
    initThreeJS()
    
    // Update AI context
    aiStore.updateContext({
      currentTool: 'modeling',
      ...(geometryItems.value.length > 0 && geometryItems.value[0].type === 'box' && geometryItems.value[0].params && {
        modelingGeometry: {
          type: 'box',
          dimensions: {
            width: geometryItems.value[0].params.width,
            height: geometryItems.value[0].params.height,
            depth: geometryItems.value[0].params.depth
          },
          divisions: {
            divX: geometryItems.value[0].params.divX,
            divY: geometryItems.value[0].params.divY,
            divZ: geometryItems.value[0].params.divZ
          }
        }
      })
    })
    
    // Check if we have result data to display
    if (projectStore.hasResult) {
      createResultMesh()
    }
  })
})

onUnmounted(() => {
  if (animationFrameId !== null) {
    cancelAnimationFrame(animationFrameId)
  }
  
  window.removeEventListener('resize', onWindowResize)
  
  // Dispose Three.js resources
  if (renderer) {
    renderer.dispose()
    renderer = null
  }
  
  if (scene) {
    scene.traverse(object => {
      if (object instanceof THREE.Mesh) {
        object.geometry.dispose()
        if (Array.isArray(object.material)) {
          object.material.forEach(m => m.dispose())
        } else {
          object.material.dispose()
        }
      }
    })
    scene = null
  }
  
  camera = null
  controls = null
})
</script>

<style scoped>
.modeling-view {
  --accent: #3b82f6;
}

.btn {
  @apply px-3 py-1.5 rounded text-xs font-medium transition-colors;
}

.btn-primary {
  @apply bg-blue-600 text-white hover:bg-blue-700;
}

.btn-ghost {
  @apply bg-transparent hover:bg-gray-100 text-gray-700;
}

.input {
  @apply px-2 py-1.5 border border-[var(--border-subtle)] rounded bg-[var(--bg-base)] text-[var(--text-primary)] text-xs focus:outline-none focus:border-blue-500;
}

.icon-btn {
  @apply flex items-center justify-center rounded transition-colors;
  @apply bg-transparent hover:bg-gray-100 text-[var(--text-secondary)];
}

.icon-btn.active {
  @apply bg-blue-100 text-blue-600;
}
</style>