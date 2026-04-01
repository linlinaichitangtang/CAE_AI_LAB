<!--
/**
 * 生物力学仿真视图组件
 * 
 * 功能说明:
 * - 提供骨科植入物（髋关节、膝关节、骨折钢板等）的力学仿真
 * - 支持软组织（血管、软骨、肌腱等）的超弹性分析
 * - 提供预定义分析模板和生物材料库
 * - 配置边界条件、荷载类型和日常活动载荷
 * - 调用后端CAE进行生物力学求解
 * - 支持AI解释结果和嵌入笔记功能
 * 
 * 主要分析类型:
 * - orthopedic: 骨科植入物分析
 * - soft_tissue: 软组织分析
 */
-->
<template>
  <div class="h-full flex flex-col bg-gray-50">
    <!-- Header: 页面标题、类型切换、操作按钮 -->
    <div class="flex items-center justify-between px-4 py-3 bg-white border-b">
      <div>
        <h2 class="text-lg font-semibold text-gray-800">生物力学仿真</h2>
        <p class="text-sm text-gray-500">骨科植入物、软组织力学分析</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- 分析类型切换 -->
        <select v-model="analysisType" class="px-3 py-1.5 text-sm border border-blue-300 rounded">
          <option value="orthopedic">骨科植入物分析</option>
          <option value="soft_tissue">软组织分析</option>
        </select>
        <!-- 🤖 AI解释结果 -->
        <button 
          @click="showAIResultDialog"
          :disabled="!projectStore.hasResult"
          class="px-3 py-1.5 text-sm border border-purple-300 text-purple-600 rounded hover:bg-purple-50 transition disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-1"
        >
          <span>🤖</span>
          <span>AI解释结果</span>
        </button>
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

    <!-- Main content: 主内容区，三栏布局 -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Biomechanics Setup - 生物力学设置面板 -->
      <div class="w-80 bg-white border-r overflow-y-auto p-4 space-y-6">
        <!-- Step 1: Analysis Type Selection - 分析类型选择 -->
        <div class="space-y-3">
          <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
            <span class="w-5 h-5 rounded-full bg-blue-600 text-white text-xs flex items-center justify-center">1</span>
            分析类型
          </h3>
          
          <div class="space-y-2">
            <select v-model="analysisType" class="w-full px-2 py-1.5 border rounded text-sm">
              <option value="orthopedic">骨科植入物分析</option>
              <option value="soft_tissue">软组织分析</option>
            </select>
          </div>
        </div>

        <!-- Step 2: Template Selection - 分析模板选择 -->
        <div class="space-y-3">
          <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
            <span class="w-5 h-5 rounded-full bg-green-600 text-white text-xs flex items-center justify-center">2</span>
            分析模板
          </h3>
          
          <div class="grid grid-cols-1 gap-2">
            <button
              v-for="template in templates"
              :key="template.id"
              @click="selectTemplate(template)"
              :class="['p-3 border rounded-lg text-left transition', selectedTemplate?.id === template.id ? 'border-blue-500 bg-blue-50' : 'border-gray-200 hover:border-gray-300']"
            >
              <div class="flex items-center gap-2">
                <span class="text-xl">{{ template.icon }}</span>
                <div>
                  <div class="text-sm font-medium text-gray-800">{{ template.name }}</div>
                  <div class="text-xs text-gray-500">{{ template.description }}</div>
                </div>
              </div>
            </button>
          </div>
        </div>

        <!-- Step 3: Material Library - 生物材料库选择 -->
        <div class="space-y-3">
          <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
            <span class="w-5 h-5 rounded-full bg-orange-600 text-white text-xs flex items-center justify-center">3</span>
            生物材料库
          </h3>
          
          <div class="space-y-2">
            <div>
              <label class="text-xs text-gray-600 mb-1 block">选择材料</label>
              <select v-model="selectedMaterial" class="w-full px-2 py-1.5 border rounded text-sm">
                <optgroup v-if="analysisType === 'orthopedic'" label="金属植入物">
                  <option v-for="m in orthopedicMaterials" :key="m.name" :value="m.name">{{ m.name }} ({{ m.E }} MPa)</option>
                </optgroup>
                <optgroup label="生物组织">
                  <option v-for="m in tissueMaterials" :key="m.name" :value="m.name">{{ m.name }} ({{ m.type }})</option>
                </optgroup>
              </select>
            </div>
            
            <!-- Material Properties Display -->
            <div v-if="currentMaterialProps" class="bg-gray-50 rounded-lg p-3 space-y-2">
              <div class="text-xs font-medium text-gray-600 mb-2">材料参数</div>
              <div class="grid grid-cols-2 gap-2 text-xs">
                <div>
                  <span class="text-gray-500">弹性模量:</span>
                  <span class="ml-1 font-medium">{{ currentMaterialProps.E }} MPa</span>
                </div>
                <div>
                  <span class="text-gray-500">泊松比:</span>
                  <span class="ml-1 font-medium">{{ currentMaterialProps.nu }}</span>
                </div>
                <div v-if="currentMaterialProps.density">
                  <span class="text-gray-500">密度:</span>
                  <span class="ml-1 font-medium">{{ currentMaterialProps.density }} g/cm³</span>
                </div>
                <div v-if="currentMaterialProps.yieldStrength">
                  <span class="text-gray-500">屈服强度:</span>
                  <span class="ml-1 font-medium">{{ currentMaterialProps.yieldStrength }} MPa</span>
                </div>
                <div v-if="currentMaterialProps.UTS">
                  <span class="text-gray-500">极限强度:</span>
                  <span class="ml-1 font-medium">{{ currentMaterialProps.UTS }} MPa</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 4: Boundary Conditions - 边界条件配置 -->
        <div class="space-y-3">
          <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
            <span class="w-5 h-5 rounded-full bg-purple-600 text-white text-xs flex items-center justify-center">4</span>
            边界条件
          </h3>
          
          <!-- Constraint Type -->
          <div>
            <label class="text-xs text-gray-600 mb-1 block">约束类型</label>
            <select v-model="constraintType" class="w-full px-2 py-1.5 border rounded text-sm">
              <option value="fixed">固定约束 (All DOFs)</option>
              <option value="pinned">铰接约束 (Translation only)</option>
              <option value="symmetry">对称约���</option>
              <option value="roller">滚支约束 (Y方向释放)</option>
            </select>
          </div>
          
          <!-- Constraint Location -->
          <div>
            <label class="text-xs text-gray-600 mb-1 block">约束位置</label>
            <div class="grid grid-cols-3 gap-2">
              <label class="flex items-center gap-1 cursor-pointer">
                <input type="checkbox" v-model="constraintSides.X" class="text-blue-600" />
                <span class="text-xs">X端</span>
              </label>
              <label class="flex items-center gap-1 cursor-pointer">
                <input type="checkbox" v-model="constraintSides.Y" class="text-blue-600" />
                <span class="text-xs">Y端</span>
              </label>
              <label class="flex items-center gap-1 cursor-pointer">
                <input type="checkbox" v-model="constraintSides.Z" class="text-blue-600" />
                <span class="text-xs">Z端</span>
              </label>
            </div>
          </div>
          
          <!-- Load Type -->
          <div>
            <label class="text-xs text-gray-600 mb-1 block">荷载类型</label>
            <select v-model="loadType" class="w-full px-2 py-1.5 border rounded text-sm">
              <option value="axial">轴向荷载</option>
              <option value="bending">弯曲荷载</option>
              <option value="torsion">扭转荷载</option>
              <option value="complex">复合荷载</option>
            </select>
          </div>
          
          <!-- Load Magnitude -->
          <div>
            <label class="text-xs text-gray-600 mb-1 block">荷载大小 (N)</label>
            <input type="number" v-model.number="loadMagnitude" class="w-full px-2 py-1 border rounded text-sm" />
          </div>
          
          <!-- Load Preset (Daily Activities) -->
          <div>
            <label class="text-xs text-gray-600 mb-1 block">日常活动载荷</label>
            <select v-model="selectedLoadPreset" @change="applyLoadPreset" class="w-full px-2 py-1.5 border rounded text-sm">
              <option value="">-- 自定义 --</option>
              <option value="walking">行走 (400-700 N)</option>
              <option value="running">跑步 (1000-1500 N)</option>
              <option value="stairs">爬楼 (800-1200 N)</option>
              <option value="sit_stand">坐下/站起 (600-900 N)</option>
              <option value="standing">站立 (250-400 N)</option>
            </select>
          </div>
        </div>

        <!-- Step 5: Generate & Solve - 生成与求解 -->
        <div class="space-y-3">
          <h3 class="text-sm font-medium text-gray-700 flex items-center gap-2">
            <span class="w-5 h-5 rounded-full bg-red-600 text-white text-xs flex items-center justify-center">5</span>
            生成与求解
          </h3>
          
          <button 
            @click="generateAndSolve"
            :disabled="solving"
            class="w-full px-3 py-2 bg-blue-600 text-white rounded text-sm hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ solving ? '求解中...' : '生成并求解' }}
          </button>
          
          <div v-if="projectStore.hasResult" class="text-xs text-green-600 bg-green-50 rounded p-2">
            ✓ 求解完成
          </div>
        </div>
      </div>

      <!-- Right Panel: 3D Viewport -->
      <div class="flex-1 flex flex-col">
        <!-- Toolbar -->
        <div class="flex items-center justify-between px-4 py-2 bg-gray-100 border-b">
          <div class="flex items-center gap-4">
            <!-- Display Mode -->
            <div class="flex items-center gap-2">
              <label class="text-xs text-gray-600">显示:</label>
              <select v-model="displayMode" class="px-2 py-1 text-xs border rounded">
                <option value="mesh">网格</option>
                <option value="wireframe">线框</option>
                <option value="solid">实体</option>
                <option value="phantom">透明</option>
              </select>
            </div>
            
            <!-- Result Type -->
            <div class="flex items-center gap-2">
              <label class="text-xs text-gray-600">结果:</label>
              <select v-model="resultType" @change="updateResultDisplay" class="px-2 py-1 text-xs border rounded">
                <option value="displacement">位移</option>
                <option value="stress">von Mises应力</option>
                <option value="strain">应变</option>
                <option value="stress_shielding">应力遮挡</option>
                <option value="micromotion">界面微动</option>
              </select>
            </div>
            
            <!-- Deformation Scale -->
            <div class="flex items-center gap-2">
              <label class="text-xs text-gray-600">变形:</label>
              <input type="range" v-model.number="deformationScale" min="1" max="10" step="1" class="w-20" />
              <span class="text-xs">{{ deformationScale }}x</span>
            </div>
          </div>
          
          <div class="flex items-center gap-2">
            <button @click="resetView" class="px-2 py-1 text-xs border rounded hover:bg-gray-200">重置视图</button>
            <button @click="toggleResultMesh" class="px-2 py-1 text-xs border rounded hover:bg-gray-200">
              {{ showResult ? '隐藏结果' : '显示结果' }}
            </button>
          </div>
        </div>
        
        <!-- 3D Canvas Container -->
        <div ref="canvasContainer" class="flex-1 bg-gray-900 relative">
          <canvas ref="threeCanvas" class="w-full h-full"></canvas>
          
          <!-- Color Legend -->
          <div class="absolute bottom-4 right-4">
            <ColorLegend :colormap="colormap" :minValue="resultMin" :maxValue="resultMax" :resultType="resultType" />
          </div>
          
          <!-- Stats Panel -->
          <div v-if="projectStore.hasResult" class="absolute top-4 left-4 bg-black/70 text-white p-3 rounded text-xs space-y-1">
            <div class="font-medium">结果统计</div>
            <div>最大位移: {{ stats.maxDisplacement.toFixed(4) }} mm</div>
            <div>最大应力: {{ stats.maxStress.toFixed(2) }} MPa</div>
            <div>最大应变: {{ stats.maxStrain.toFixed(4) }}</div>
            <div v-if="resultType === 'stress_shielding'">应力遮挡率: {{ stats.stressShielding.toFixed(1) }}%</div>
            <div v-if="resultType === 'micromotion'">界面微动: {{ stats.micromotion.toFixed(4) }} mm</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 🤖 AI解释结果对话框 -->
    <div v-if="showAIDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showAIDialog = false">
      <div class="bg-white rounded-lg shadow-xl w-[600px] max-h-[80vh] overflow-y-auto">
        <div class="p-4 border-b flex justify-between items-center">
          <h3 class="text-lg font-semibold text-gray-800 flex items-center gap-2">
            <span>🤖</span>
            <span>AI解释结果</span>
          </h3>
          <button @click="showAIDialog = false" class="text-gray-500 hover:text-gray-700">✕</button>
        </div>
        <div class="p-4">
          <div v-if="aiLoading" class="text-center py-8">
            <div class="text-gray-500">AI分析中...</div>
          </div>
          <div v-else-if="aiResult" class="prose prose-sm max-w-none">
            <pre class="whitespace-pre-wrap text-sm text-gray-700 font-sans">{{ aiResult }}</pre>
          </div>
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
              <option v-for="note in notes" :key="note.id" :value="note.id">
                {{ note.file_name || '无标题笔记' }}
              </option>
            </select>
          </div>
          
          <!-- 嵌入预览 -->
          <div class="bg-gray-50 dark:bg-gray-900 rounded p-3">
            <div class="flex items-center gap-3">
              <span class="text-2xl">🦴</span>
              <div>
                <div class="text-sm font-medium text-gray-800 dark:text-white">
                  {{ selectedTemplate?.name || '生物力学分析' }} ({{ analysisType === 'orthopedic' ? '骨科植入物' : '软组织' }})
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  生物力学仿真 • {{ new Date().toLocaleDateString() }}
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
            @click="embedToNote" 
            :disabled="!selectedEmbedNoteId"
            class="px-4 py-2 bg-orange-500 text-white rounded hover:bg-orange-600 text-sm disabled:opacity-50 disabled:cursor-not-allowed"
          >
            嵌入
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import ColorLegend from '../components/simulation/ColorLegend.vue'
import { useProjectStore } from '@/stores/project'
import { useAiStore } from '@/stores/ai'

const projectStore = useProjectStore()
const aiStore = useAiStore()

// ============================================================
// 生物力学仿真视图 - 脚本逻辑
// ============================================================

// ========== Analysis Type: 分析类型 ==========
const analysisType = ref<'orthopedic' | 'soft_tissue'>('orthopedic')

// ========== Templates: 预定义分析模板 ==========
interface Template {
  id: string
  name: string
  description: string
  icon: string
  config: {
    dimension: string
    meshSize: number
    material: string
    loadType: string
    constraintType: string
  }
}

const templates = ref<Template[]>([
  {
    id: 'hip_replacement',
    name: '髋关节置换',
    description: '股骨头假体植入后的力学分析',
    icon: '🦴',
    config: { dimension: '3d', meshSize: 2, material: 'Ti-6Al-4V', loadType: 'axial', constraintType: 'fixed' }
  },
  {
    id: 'knee_replacement',
    name: '膝关节置换',
    description: '膝关节假体接触应力分析',
    icon: '🦿',
    config: { dimension: '3d', meshSize: 2, material: 'CoCrMo', loadType: 'complex', constraintType: 'fixed' }
  },
  {
    id: 'fracture_plate',
    name: '骨折钢板固定',
    description: '钢板螺钉内固定应力分析',
    icon: '🔩',
    config: { dimension: '3d', meshSize: 1.5, material: 'Ti-6Al-4V', loadType: 'bending', constraintType: 'fixed' }
  },
  {
    id: 'spinal_fusion',
    name: '脊柱融合',
    description: '椎体融合器应力分析',
    icon: '🪜',
    config: { dimension: '3d', meshSize: 1, material: 'PEEK', loadType: 'axial', constraintType: 'pinned' }
  },
  {
    id: 'vascular',
    name: '血管变形',
    description: '血管壁软组织超弹性分析',
    icon: '🩸',
    config: { dimension: '3d', meshSize: 1, material: '骨组织', loadType: 'pressure', constraintType: 'symmetry' }
  },
  {
    id: 'cartilage',
    name: '软骨接触',
    description: '关节软骨压应力分析',
    icon: '🫓',
    config: { dimension: '3d', meshSize: 0.5, material: '软骨', loadType: 'complex', constraintType: 'fixed' }
  }
])

const selectedTemplate = ref<Template | null>(null)

// ========== Materials ==========
interface MaterialProps {
  name: string
  type: string
  E?: number
  nu?: number
  density?: number
  yieldStrength?: number
  UTS?: number
  model?: string
}

// ========== Orthopedic implant materials: 骨科植入物材料 ==========
const orthopedicMaterials: MaterialProps[] = [
  { name: 'Ti-6Al-4V', type: '钛合金', E: 110000, nu: 0.34, density: 4.43, yieldStrength: 880, UTS: 950 },
  { name: 'CoCrMo', type: '钴铬合金', E: 210000, nu: 0.30, density: 8.9, yieldStrength: 500, UTS: 900 },
  { name: 'PEEK', type: '高分子', E: 3500, nu: 0.36, density: 1.32, yieldStrength: 100, UTS: 110 },
  { name: 'StainlessSteel', type: '不锈钢', E: 200000, nu: 0.30, density: 7.9, yieldStrength: 250, UTS: 580 },
  { name: 'Zirconia', type: '陶瓷', E: 200000, nu: 0.30, density: 6.0, yieldStrength: 800, UTS: 1000 }
]

// ========== Soft tissue materials: 软组织材料 ==========
const tissueMaterials: MaterialProps[] = [
  { name: '骨组织', type: '各向异性', E: 15000, nu: 0.3, density: 1.8, yieldStrength: 100 },
  { name: '软骨', type: '超弹性', E: 10, nu: 0.45, density: 1.1, model: 'Mooney-Rivlin' },
  { name: '肌腱', type: '超弹性', E: 500, nu: 0.3, density: 1.2, model: 'Ogden' },
  { name: '韧带', type: '超弹性', E: 300, nu: 0.35, density: 1.1, model: 'Ogden' },
  { name: '血管', type: '超弹性', E: 2, nu: 0.45, density: 1.0, model: 'Mooney-Rivlin' }
]

const selectedMaterial = ref('Ti-6Al-4V')

const currentMaterialProps = computed(() => {
  const allMaterials = [...orthopedicMaterials, ...tissueMaterials]
  return allMaterials.find(m => m.name === selectedMaterial.value)
})

// ========== Boundary Conditions: 边界条件配置 ==========
const constraintType = ref<'fixed' | 'pinned' | 'symmetry' | 'roller'>('fixed')
const constraintSides = ref({ X: true, Y: true, Z: false })
const loadType = ref<'axial' | 'bending' | 'torsion' | 'complex'>('axial')
const loadMagnitude = ref(500)
const selectedLoadPreset = ref('')

// ========== 3D Viewer State: 3D查看器状态 ==========
const canvasContainer = ref<HTMLElement | null>(null)
const threeCanvas = ref<HTMLCanvasElement | null>(null)
const displayMode = ref<'mesh' | 'wireframe' | 'solid' | 'phantom'>('mesh')
const resultType = ref<'displacement' | 'stress' | 'strain' | 'stress_shielding' | 'micromotion'>('stress')
const deformationScale = ref(5)
const colormap = ref<'viridis' | 'plasma' | 'inferno' | 'jet' | 'rainbow'>('viridis')
const resultMin = ref(0)
const resultMax = ref(100)
const showResult = ref(true)

// Stats: 结果统计数据
const stats = ref({
  maxDisplacement: 0,
  maxStress: 0,
  maxStrain: 0,
  stressShielding: 0,
  micromotion: 0
})

// ========== Dialog State: 对话框状态 ==========
const showAIDialog = ref(false)
const aiLoading = ref(false)
const aiResult = ref('')
const showEmbedDialog = ref(false)
const selectedEmbedNoteId = ref('')
const notes = ref<any[]>([])

// ========== Solving State: 求解状态 ==========
const solving = ref(false)

// ============================================================
// Functions: 核心功能函数
// ============================================================

/**
 * 应用日常活动载荷预设
 * 根据选择的日常活动（行走、跑步、爬楼等）自动设置相应的荷载大小
 */
function applyLoadPreset() {
  const presets: Record<string, number> = {
    walking: 500,
    running: 1200,
    stairs: 1000,
    sit_stand: 700,
    standing: 300
  }
  if (selectedLoadPreset.value && presets[selectedLoadPreset.value]) {
    loadMagnitude.value = presets[selectedLoadPreset.value]
  }
}

function selectTemplate(template: Template) {
  selectedTemplate.value = template
  if (template.config.material) {
    selectedMaterial.value = template.config.material
  }
  if (template.config.loadType) {
    loadType.value = template.config.loadType as any
  }
  if (template.config.constraintType) {
    constraintType.value = template.config.constraintType as any
  }
}

async function generateAndSolve() {
  solving.value = true
  try {
    // 从模板或当前设置构建分析作业
    const templates_data: any = await invoke('get_bio_templates')
    let jobConfig: any = null

    // 尝试从模板构建
    if (templates_data && templates_data.length > 0) {
      const tmpl = templates_data.find((t: any) =>
        analysisType.value === 'orthopedic'
          ? t.category === 'orthopedic'
          : t.category === 'soft_tissue'
      )
      if (tmpl) {
        jobConfig = { ...tmpl.config }
      }
    }

    // 如果没有模板，用当前参数构建
    if (!jobConfig) {
      const matProps = currentMaterialProps.value
      jobConfig = {
        id: `bio_${Date.now()}`,
        name: `${analysisType.value === 'orthopedic' ? '骨科植入物' : '软组织'}分析`,
        analysis_type: analysisType.value,
        mesh_config: {
          x_min: -50, x_max: 50,
          y_min: -50, y_max: 50,
          z_min: -50, z_max: 50,
          x_div: 20, y_div: 20, z_div: 20,
          mesh_quality_target: 0.8,
          stl_import_path: null
        },
        material: {
          name: selectedMaterial.value,
          category: 'metal_implant',
          density: matProps?.density || 4430,
          youngs_modulus: (matProps?.E || 110e9),
          poissions_ratio: matProps?.nu || 0.33,
          yield_strength: matProps?.yieldStrength || 880e6,
          ultimate_strength: 950e6,
          fatigue_limit: 500e6,
          hardness: 350,
          bio_compatible: true
        },
        loading_config: {
          load_type: loadType.value.toLowerCase().includes('cyclic') ? 'cyclic' : 'static',
          load_magnitude: loadMagnitude.value,
          load_direction: [0, -1, 0],
          load_application_area: 'surface',
          frequency: 1.0,
          num_cycles: 10000000
        },
        boundary_conditions: [{
          name: 'fixed_bottom',
          bc_type: 'fixed',
          face: 'bottom',
          values: [0, 0, 0]
        }],
        results: null,
        status: 'pending'
      }
    }

    console.log('运行生物力学分析:', jobConfig)

    // 调用后端生物力学分析
    const results = await invoke('run_biomechanics_analysis', { job: jobConfig }) as any

    stats.value = {
      maxDisplacement: results.max_displacement || 0,
      maxStress: results.max_stress || 0,
      maxStrain: results.max_von_mises || 0,
      stressShielding: results.stress_shielding_ratio || 0,
      micromotion: results.interface_micromotion || 0
    }

    resultMax.value = Math.max(
      stats.value.maxDisplacement * 1.2,
      stats.value.maxStress * 1.2,
      stats.value.maxStrain * 1.2
    )

    // hasResult is read-only (computed from lastResult)
    console.log('生物力学分析完成:', results)
  } catch (error) {
    console.error('Biomechanics solve error:', error)
    alert('分析失败: ' + error)
  } finally {
    solving.value = false
  }
}

// ========== AI Result Dialog ==========
async function showAIResultDialog() {
  showAIDialog.value = true
  aiLoading.value = true
  aiResult.value = ''
  
  try {
    const prompt = `作为生物力学仿真专家，请分析以下结果并给出专业解释：
    分析类型: ${analysisType.value === 'orthopedic' ? '骨科植入物分析' : '软组织分析'}
    材料: ${selectedMaterial.value}
    最大位移: ${stats.value.maxDisplacement.toFixed(4)} mm
    最大应力: ${stats.value.maxStress.toFixed(2)} MPa
    最大应变: ${stats.value.maxStrain.toFixed(4)}
    ${resultType.value === 'stress_shielding' ? `应力遮挡率: ${stats.value.stressShielding.toFixed(1)}%` : ''}
    ${resultType.value === 'micromotion' ? `界面微动: ${stats.value.micromotion.toFixed(4)} mm` : ''}
    
    请解释这些结果的临床意义，并给出可能的改进建议。`
    
    const result = await aiStore.sendMessage(prompt)
    aiResult.value = result.response || 'AI返回为空'
  } catch (error) {
    aiResult.value = 'AI分析失败: ' + String(error)
  } finally {
    aiLoading.value = false
  }
}

// ========== Embed to Note ==========
const loadNotes = async () => {
  if (projectStore.currentProject) {
    try {
      const { listFiles } = await import('@/api')
      const allFiles = await listFiles(projectStore.currentProject.id)
      notes.value = allFiles.filter((f: any) => f.file_type === 'note')
    } catch (e) {
      console.error('Load notes error:', e)
    }
  }
}

function showEmbedToNoteDialog() {
  if (!projectStore.currentNoteId && notes.value.length > 0) {
    selectedEmbedNoteId.value = notes.value[0]?.id || ''
  } else {
    selectedEmbedNoteId.value = projectStore.currentNoteId || ''
  }
  showEmbedDialog.value = true
}

function embedToNote() {
  if (!selectedEmbedNoteId.value) return
  
  const embedRecord = projectStore.addEmbedRecord({
    type: 'biomechanics' as any,
    targetId: 'current-simulation',
    targetName: selectedTemplate.value?.name || '生物力学分析',
    noteId: selectedEmbedNoteId.value
  })
  
  console.log('Biomechanics embedded to note:', embedRecord)
  showEmbedDialog.value = false
  alert('✓ 生物力学分析已成功嵌入到笔记！\n\n在笔记中点击嵌入卡片可跳转到生物力学界面。')
}

// ========== 3D Viewer Functions ==========
function updateResultDisplay() {
  // Trigger result mesh update in Three.js
  console.log('Update result display:', resultType.value)
}

function toggleResultMesh() {
  showResult.value = !showResult.value
}

function resetView() {
  // Reset Three.js camera
  console.log('Reset view')
}

function exportScreenshot() {
  // Export canvas as image
  if (threeCanvas.value) {
    const link = document.createElement('a')
    link.download = 'biomechanics_result.png'
    link.href = threeCanvas.value.toDataURL('image/png')
    link.click()
  }
}

function resetAll() {
  // Reset all state
  analysisType.value = 'orthopedic'
  selectedTemplate.value = null
  selectedMaterial.value = 'Ti-6Al-4V'
  constraintType.value = 'fixed'
  constraintSides.value = { X: true, Y: true, Z: false }
  loadType.value = 'axial'
  loadMagnitude.value = 500
  selectedLoadPreset.value = ''
  
  projectStore.setMesh(null as any)
  projectStore.lastResult = null
}

// Watch project changes
watch(() => projectStore.currentProject, () => {
  loadNotes()
})

onMounted(async () => {
  // 加载模板
  try {
    templates.value = await invoke('get_bio_templates')
  } catch (e) {
    console.error('加载生物力学模板失败:', e)
  }
  // 加载笔记
  loadNotes()
})
</script>

<style scoped>
.prose pre {
  white-space: pre-wrap;
  word-break: break-word;
}

input[type="range"] {
  -webkit-appearance: none;
  appearance: none;
  height: 4px;
  background: #d1d5db;
  border-radius: 2px;
}

input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 12px;
  height: 12px;
  background: #3b82f6;
  border-radius: 50%;
  cursor: pointer;
}
</style>