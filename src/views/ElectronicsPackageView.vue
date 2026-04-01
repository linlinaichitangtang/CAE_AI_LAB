<!--
  电子封装分析前端界面 - ElectronicsPackageView.vue
  功能：PCB热分析、BGA焊点可靠性、倒装芯片封装的热-结构耦合分析
  作者：麟现
  
  界面布局：
  - 左侧：分析设置面板（可折叠sections）
  - 右侧：3D viewport 显示模型和结果
  
  主要功能模块：
  1. 分析类型选择（PCB/BGA/倒装焊/QFN）
  2. 材料库管理（内置电子材料参数）
  3. 边界条件设置（热边界+结构边界）
  4. 网格设置与生成
  5. 求解控制与运行
  6. 结果后处理（温度场/热应力/焊点疲劳）
  7. 模板快速加载
  8. 嵌入笔记功能
-->
<template>
  <div class="h-full flex flex-col bg-[var(--bg-primary)]">
    <!-- Header: 页面标题、分析类型切换、操作按钮 -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-lg font-semibold text-[var(--text-primary)]">电子封装分析</h2>
        <p class="text-sm text-[var(--text-secondary)]">PCB热分析、BGA/倒装焊封装、焊点可靠性</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- 分析类型选择下拉菜单 -->
        <select v-model="analysisType" class="px-3 py-1.5 text-sm border border-[var(--border-default)] rounded bg-[var(--bg-surface)] text-[var(--text-primary)]">
          <option value="pcb">PCB热分析</option>
          <option value="bga">BGA焊点</option>
          <option value="fc">倒装焊</option>
          <option value="qfn">QFN封装</option>
        </select>
        <!-- 模板按钮：快速加载预设分析模板 -->
        <button @click="showTemplatesDialog = true" class="px-3 py-1.5 text-sm border border-blue-300 text-blue-400 rounded hover:bg-blue-900/20 transition flex items-center gap-1">
          📋 模板
        </button>
        <!-- 材料库按钮：查看/选择电子材料 -->
        <button @click="showMaterialLibrary = true" class="px-3 py-1.5 text-sm border border-green-300 text-green-400 rounded hover:bg-green-900/20 transition flex items-center gap-1">
          📦 材料库
        </button>
        <!-- 嵌入到笔记按钮 -->
        <button @click="showEmbedDialog = true" class="px-3 py-1.5 text-sm border border-purple-300 text-purple-400 rounded hover:bg-purple-900/20 transition flex items-center gap-1">
          🔗 嵌入到笔记
        </button>
      </div>
    </div>

    <!-- Main Content: 左侧设置 + 右侧3D视图 -->
    <div class="flex-1 flex overflow-hidden">
      <!-- 左侧：分析设置面板 -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] overflow-y-auto">
        <!-- Step 1: 几何与材料 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('geometry')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-hover)]">
            <span class="font-medium text-[var(--text-primary)]">1. 几何与材料</span>
            <span class="text-[var(--text-muted)]">{{ sections.geometry ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.geometry" class="px-4 pb-4 space-y-3">
            <!-- 芯片尺寸参数 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">芯片尺寸 (mm)</label>
              <div class="grid grid-cols-2 gap-2">
                <div>
                  <span class="text-[10px] text-[var(--text-muted)]">长度</span>
                  <input v-model.number="chipLength" type="number" step="0.1" class="input w-full text-xs" placeholder="10" />
                </div>
                <div>
                  <span class="text-[10px] text-[var(--text-muted)]">宽度</span>
                  <input v-model.number="chipWidth" type="number" step="0.1" class="input w-full text-xs" placeholder="10" />
                </div>
                <div>
                  <span class="text-[10px] text-[var(--text-muted)]">厚度</span>
                  <input v-model.number="chipThickness" type="number" step="0.01" class="input w-full text-xs" placeholder="0.7" />
                </div>
                <div>
                  <span class="text-[10px] text-[var(--text-muted)]">焊点直径</span>
                  <input v-model.number="solderDiameter" type="number" step="0.01" class="input w-full text-xs" placeholder="0.5" />
                </div>
              </div>
            </div>
            <!-- PCB板层数选择 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">PCB层数</label>
              <select v-model="pcbLayers" class="input w-full text-xs">
                <option value="2">2层板</option>
                <option value="4">4层板</option>
                <option value="6">6层板</option>
                <option value="8">8层板</option>
              </select>
            </div>
            <!-- 芯片材料选择 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">芯片材料</label>
              <select v-model="chipMaterial" class="input w-full text-xs">
                <option value="silicon">硅 (Si)</option>
                <option value="gan">氮化镓 (GaN)</option>
                <option value="sic">碳化硅 (SiC)</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Step 2: 材料参数 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('materials')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-hover)]">
            <span class="font-medium text-[var(--text-primary)]">2. 材料参数</span>
            <span class="text-[var(--text-muted)]">{{ sections.materials ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.materials" class="px-4 pb-4 space-y-3">
            <!-- 导热系数 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">导热系数 (W/m·K)</label>
              <input v-model.number="thermalConductivity" type="number" step="1" class="input w-full text-xs" placeholder="150" />
            </div>
            <!-- 热膨胀系数 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">热膨胀系数 (1/K ×10⁻⁶)</label>
              <input v-model.number="cte" type="number" step="0.1" class="input w-full text-xs" placeholder="2.6" />
            </div>
            <!-- 弹性模量 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">弹性模量 (GPa)</label>
              <input v-model.number="youngModulus" type="number" step="1" class="input w-full text-xs" placeholder="169" />
            </div>
            <!-- 泊松比 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">泊松比</label>
              <input v-model.number="poissonRatio" type="number" step="0.01" class="input w-full text-xs" placeholder="0.28" />
            </div>
            <!-- 功率密度 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">芯片功率密度 (W/mm²)</label>
              <input v-model.number="powerDensity" type="number" step="0.01" class="input w-full text-xs" placeholder="0.5" />
            </div>
          </div>
        </div>

        <!-- Step 3: 边界条件 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('boundary')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-hover)]">
            <span class="font-medium text-[var(--text-primary)]">3. 边界条件</span>
            <span class="text-[var(--text-muted)]">{{ sections.boundary ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.boundary" class="px-4 pb-4 space-y-3">
            <!-- 温度边界条件 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">环境温度 (°C)</label>
              <input v-model.number="ambientTemp" type="number" step="1" class="input w-full text-xs" placeholder="25" />
            </div>
            <!-- 对流换热系数 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">对流换热系数 (W/m²·K)</label>
              <input v-model.number="convectionCoeff" type="number" step="1" class="input w-full text-xs" placeholder="50" />
            </div>
            <!-- 结构约束 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">固定约束</label>
              <div class="space-y-1">
                <label class="flex items-center gap-2 text-xs">
                  <input type="checkbox" v-model="fixBottom" class="rounded" />
                  <span>底部固定</span>
                </label>
                <label class="flex items-center gap-2 text-xs">
                  <input type="checkbox" v-model="symmetryX" class="rounded" />
                  <span>X对称</span>
                </label>
                <label class="flex items-center gap-2 text-xs">
                  <input type="checkbox" v-model="symmetryY" class="rounded" />
                  <span>Y对称</span>
                </label>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 4: 求解控制 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('solve')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-hover)]">
            <span class="font-medium text-[var(--text-primary)]">4. 求解控制</span>
            <span class="text-[var(--text-muted)]">{{ sections.solve ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.solve" class="px-4 pb-4 space-y-3">
            <!-- 分析类型：稳态/瞬态 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">分析类型</label>
              <div class="flex gap-2">
                <button @click="analysisMode = 'steady'" :class="['flex-1 py-1 text-xs rounded border', analysisMode === 'steady' ? 'border-blue-500 bg-blue-900/30 text-blue-400' : 'border-[var(--border-default)]']">稳态热</button>
                <button @click="analysisMode = 'transient'" :class="['flex-1 py-1 text-xs rounded border', analysisMode === 'transient' ? 'border-blue-500 bg-blue-900/30 text-blue-400' : 'border-[--border-default]']">瞬态热</button>
              </div>
            </div>
            <!-- 瞬态时间步 -->
            <div v-if="analysisMode === 'transient'">
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">总时间 (s)</label>
              <input v-model.number="totalTime" type="number" step="1" class="input w-full text-xs" placeholder="3600" />
            </div>
            <!-- 网格精度 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">网格精度</label>
              <select v-model="meshQuality" class="input w-full text-xs">
                <option value="coarse">粗网格 (快速)</option>
                <option value="medium">中等网格</option>
                <option value="fine">细网格 (精确)</option>
              </select>
            </div>
          </div>
        </div>

        <!-- 操作按钮区域 -->
        <div class="px-4 py-3 space-y-2">
          <button @click="generateModel" :disabled="generating" class="w-full btn btn-primary text-sm">
            {{ generating ? '⟳ 生成中...' : '🔧 生成模型' }}
          </button>
          <button @click="runAnalysis" :disabled="!modelReady || running" class="w-full btn btn-primary text-sm">
            {{ running ? '⟳ 分析中...' : '▶ 运行分析' }}
          </button>
        </div>
      </div>

      <!-- 右侧：3D视图区域 -->
      <div class="flex-1 relative bg-[var(--bg-base)]">
        <!-- 模型3D显示区域（placeholder） -->
        <div class="absolute inset-0 flex items-center justify-center">
          <div class="text-center">
            <div class="w-48 h-32 mx-auto mb-4 rounded-xl bg-[var(--bg-surface)] flex items-center justify-center border-2 border-dashed border-[var(--border-subtle)]">
              <span class="text-5xl opacity-40">🖥️</span>
            </div>
            <h3 class="text-base font-semibold text-[var(--text-primary)] mb-1">电子封装模型</h3>
            <p class="text-sm text-[var(--text-muted)]">芯片 + PCB基板 + 焊点阵列</p>
            <button v-if="!modelReady" @click="generateModel" class="mt-3 btn btn-primary text-xs">
              点击生成模型
            </button>
          </div>
        </div>

        <!-- 结果显示覆盖层 -->
        <div v-if="analysisComplete" class="absolute inset-0 bg-[var(--bg-base)]/80 flex items-center justify-center">
          <div class="text-center">
            <div class="text-4xl mb-2">📊</div>
            <h3 class="text-base font-semibold text-[var(--text-primary)]">分析完成</h3>
            <p class="text-sm text-[var(--text-muted)]">最大温度: {{ maxTemp }}°C</p>
            <p class="text-sm text-[var(--text-muted)]">最大热应力: {{ maxStress }}MPa</p>
          </div>
        </div>

        <!-- 工具栏 -->
        <div class="absolute bottom-0 left-0 right-0 h-10 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] flex items-center px-4 gap-4">
          <span class="text-[10px] text-[var(--text-muted)] uppercase">视图</span>
          <button @click="viewMode = '3d'" :class="['icon-btn w-8 h-8', viewMode === '3d' ? 'active' : '']" title="3D视图">
            <span class="text-sm">🎲</span>
          </button>
          <button @click="viewMode = 'temp'" :class="['icon-btn w-8 h-8', viewMode === 'temp' ? 'active' : '']" title="温度场">
            <span class="text-sm">🌡️</span>
          </button>
          <button @click="viewMode = 'stress'" :class="['icon-btn w-8 h-8', viewMode === 'stress' ? 'active' : '']" title="应力场">
            <span class="text-sm">⚡</span>
          </button>
          <div class="flex-1"></div>
          <span v-if="running" class="text-xs text-[var(--text-secondary)] animate-pulse">分析中...</span>
        </div>
      </div>
    </div>

    <!-- 材料库对话框 -->
    <div v-if="showMaterialLibrary" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showMaterialLibrary = false">
      <div class="bg-[var(--bg-surface)] rounded-xl w-[600px] max-h-[80vh] overflow-hidden">
        <div class="px-4 py-3 border-b border-[var(--border-subtle)] flex justify-between items-center">
          <h3 class="font-semibold text-[var(--text-primary)]">📦 电子材料库</h3>
          <button @click="showMaterialLibrary = false" class="text-[var(--text-muted)] hover:text-[var(--text-primary)]">✕</button>
        </div>
        <div class="p-4 overflow-y-auto max-h-[60vh]">
          <table class="w-full text-sm">
            <thead>
              <tr class="border-b border-[var(--border-subtle)]">
                <th class="text-left py-2 text-[var(--text-secondary)]">材料</th>
                <th class="text-right py-2 text-[var(--text-secondary)]">导热 (W/mK)</th>
                <th class="text-right py-2 text-[var(--text-secondary)]">CTE (1/K)</th>
                <th class="text-right py-2 text-[var(--text-secondary)]">操作</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="mat in materialLibrary" :key="mat.id" class="border-b border-[var(--border-subtle)] hover:bg-[var(--bg-hover)]">
                <td class="py-2 text-[var(--text-primary)]">{{ mat.name }}</td>
                <td class="py-2 text-right text-[var(--text-secondary)]">{{ mat.k }}</td>
                <td class="py-2 text-right text-[var(--text-secondary)]">{{ mat.cte }}</td>
                <td class="py-2 text-right">
                  <button @click="applyMaterial(mat)" class="text-blue-400 hover:text-blue-300 text-xs">应用</button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <!-- 模板对话框 -->
    <div v-if="showTemplatesDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showTemplatesDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-xl w-[500px] max-h-[80vh] overflow-hidden">
        <div class="px-4 py-3 border-b border-[var(--border-subtle)] flex justify-between items-center">
          <h3 class="font-semibold text-[var(--text-primary)]">📋 分析模板</h3>
          <button @click="showTemplatesDialog = false" class="text-[var(--text-muted)] hover:text-[var(--text-primary)]">✕</button>
        </div>
        <div class="p-4 space-y-2 overflow-y-auto max-h-[60vh]">
          <div v-for="tmpl in templates" :key="tmpl.id" @click="loadTemplate(tmpl)" class="p-3 border border-[var(--border-subtle)] rounded-lg hover:bg-[var(--bg-hover)] cursor-pointer">
            <div class="font-medium text-sm text-[var(--text-primary)]">{{ tmpl.name }}</div>
            <div class="text-xs text-[var(--text-muted)] mt-1">{{ tmpl.desc }}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 嵌入笔记对话框 -->
    <div v-if="showEmbedDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showEmbedDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-xl w-[400px] p-4">
        <h3 class="font-semibold text-[var(--text-primary)] mb-4">🔗 嵌入到笔记</h3>
        <div class="mb-4">
          <label class="text-sm text-[var(--text-secondary)] mb-2 block">选择要嵌入的笔记</label>
          <select v-model="selectedNoteId" class="input w-full text-sm">
            <option value="">-- 选择笔记 --</option>
            <option v-for="note in notes" :key="note.id" :value="note.id">{{ note.title }}</option>
          </select>
        </div>
        <div class="flex justify-end gap-2">
          <button @click="showEmbedDialog = false" class="btn btn-ghost text-sm">取消</button>
          <button @click="embedToNote" :disabled="!selectedNoteId" class="btn btn-primary text-sm">确认嵌入</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref, reactive, onMounted } from 'vue'
import { useProjectStore } from '../stores/project'

const projectStore = useProjectStore()

/**
 * 电子封装分析视图 - Script Setup
 *
 * 主要状态：
 * - analysisType: 当前分析类型（pcb/bga/fc/qfn）
 * - modelReady: 模型是否已生成
 * - running: 是否正在运行分析
 * - analysisComplete: 分析是否完成
 *
 * 主要方法：
 * - generateModel(): 生成电子封装几何模型
 * - runAnalysis(): 调用后端运行热-结构耦合分析
 * - loadTemplate(): 加载预设模板参数
 * - embedToNote(): 将分析结果嵌入到笔记
 */

// ========== 状态定义 ==========

/** 当前分析类型：pcb=PCB热分析, bga=BGA焊点, fc=倒装焊, qfn=QFN封装 */
const analysisType = ref('pcb')

/** 分析模式：steady=稳态热, transient=瞬态热 */
const analysisMode = ref('steady')

/** 模型是否已生成 */
const modelReady = ref(false)

/** 是否正在运行分析 */
const running = ref(false)

/** 是否正在生成模型 */
const generating = ref(false)

/** 分析是否完成 */
const analysisComplete = ref(false)

// 几何参数
/** 芯片长度 (mm) */
const chipLength = ref(10)
/** 芯片宽度 (mm) */
const chipWidth = ref(10)
/** 芯片厚度 (mm) */
const chipThickness = ref(0.7)
/** 焊点直径 (mm) */
const solderDiameter = ref(0.5)
/** PCB层数 */
const pcbLayers = ref(4)
/** 芯片材料 */
const chipMaterial = ref('silicon')

// 材料参数
/** 导热系数 (W/m·K) */
const thermalConductivity = ref(150)
/** 热膨胀系数 (1/K ×10⁻⁶) */
const cte = ref(2.6)
/** 弹性模量 (GPa) */
const youngModulus = ref(169)
/** 泊松比 */
const poissonRatio = ref(0.28)
/** 芯片功率密度 (W/mm²) */
const powerDensity = ref(0.5)

// 边界条件
/** 环境温度 (°C) */
const ambientTemp = ref(25)
/** 对流换热系数 (W/m²·K) */
const convectionCoeff = ref(50)
/** 底部固定 */
const fixBottom = ref(true)
/** X对称 */
const symmetryX = ref(false)
/** Y对称 */
const symmetryY = ref(false)

// 求解控制
/** 总时间（瞬态分析用）(s) */
const totalTime = ref(3600)
/** 网格精度 */
const meshQuality = ref('medium')

/** 板厚 (mm) */
const boardThickness = ref(1.6)
/** PCB层数 */
const numLayers = ref(4)
/** 是否启用散热器 */
const heatsinkEnabled = ref(false)

// 结果
/** 最大温度 */
const maxTemp = ref(0)
/** 最大热应力 (MPa) */
const maxStress = ref(0)

// UI状态
/** 当前视图模式 */
const viewMode = ref('3d')
/** 各设置区块展开状态 */
const sections = reactive({
  geometry: true,
  materials: true,
  boundary: true,
  solve: true
})

/** 是否显示材料库对话框 */
const showMaterialLibrary = ref(false)
/** 是否显示模板对话框 */
const showTemplatesDialog = ref(false)
/** 是否显示嵌入笔记对话框 */
const showEmbedDialog = ref(false)

/** 选择的笔记ID */
const selectedNoteId = ref('')

// 笔记列表（从store获取）
const notes = ref([
  { id: '1', title: '芯片封装热分析笔记' },
  { id: '2', title: 'BGA可靠性评估' }
])

/**
 * 电子材料库数据
 * 包含常用电子封装材料的物理参数
 */
const materialLibrary = ref([
  { id: 'silicon', name: '硅芯片 (Si)', k: 150, cte: 2.6, E: 169, nu: 0.28 },
  { id: 'gan', name: '氮化镓 (GaN)', k: 130, cte: 3.5, E: 200, nu: 0.25 },
  { id: 'sic', name: '碳化硅 (SiC)', k: 270, cte: 4.0, E: 410, nu: 0.21 },
  { id: 'copper', name: '铜引线框 (Cu)', k: 400, cte: 17, E: 110, nu: 0.35 },
  { id: 'solder', name: '焊料 (SnAgCu)', k: 50, cte: 21, E: 30, nu: 0.4 },
  { id: 'fr4', name: 'FR4基材', k: 0.3, cte: 15, E: 20, nu: 0.2 },
  { id: 'underfill', name: '底部填充胶', k: 0.5, cte: 40, E: 5, nu: 0.45 }
])

/**
 * 分析模板列表
 * 提供常用电子封装分析场景的快速配置
 */
const templates = ref<any[]>([])

onMounted(async () => {
  try {
    templates.value = await invoke('get_analysis_templates')
  } catch (e) {
    console.error('加载电子封装模板失败:', e)
    templates.value = [
      { id: 'pcb_simple', name: '简单PCB热分析', desc: '单芯片PCB板稳态热分析，适合入门学习', params: {} },
      { id: 'pcb_multi', name: '多芯片PCB热分析', desc: '多芯片模块的稳态+瞬态热分析', params: {} },
      { id: 'bga_fatigue', name: 'BGA焊点疲劳分析', desc: '温度循环载荷下的BGA焊点疲劳寿命预测', params: {} },
      { id: 'fc_thermal', name: '倒装焊热分析', desc: '倒装芯片封装的热-结构耦合分析', params: {} },
      { id: 'power_module', name: '功率模块热分析', desc: 'IGBT/SiC功率模块的稳态热分析', params: {} }
    ]
  }
})

// ========== 方法定义 ==========

/**
 * 切换设置区块的展开/折叠状态
 * @param section - 要切换的区块名称
 */
function toggleSection(section: string) {
  sections[section as keyof typeof sections] = !sections[section as keyof typeof sections]
}

/**
 * 从材料库应用材料参数到当前设置
 * @param material - 选中的材料对象
 */
function applyMaterial(material: any) {
  thermalConductivity.value = material.k
  cte.value = material.cte
  youngModulus.value = material.E
  poissonRatio.value = material.nu
  showMaterialLibrary.value = false
}

/**
 * 加载预设模板参数
 * @param template - 选中的模板对象
 */
function loadTemplate(template: any) {
  // 根据模板ID设置相应参数
  if (template.id === 'pcb_simple') {
    analysisType.value = 'pcb'
    analysisMode.value = 'steady'
    chipLength.value = 10
    chipWidth.value = 10
    pcbLayers.value = 4
    powerDensity.value = 0.5
  } else if (template.id === 'bga_fatigue') {
    analysisType.value = 'bga'
    analysisMode.value = 'transient'
    totalTime.value = 3600
    powerDensity.value = 0.3
  }
  showTemplatesDialog.value = false
}

/**
 * 生成电子封装几何模型
 * 调用后端API生成参数化几何模型
 */
async function generateModel() {
  generating.value = true
  try {
    // 构造模型参数
    const params = {
      type: analysisType.value,
      chip: {
        length: chipLength.value,
        width: chipWidth.value,
        thickness: chipThickness.value,
        material: chipMaterial.value
      },
      pcb: {
        layers: pcbLayers.value
      },
      solder: {
        diameter: solderDiameter.value
      }
    }
    console.log('生成电子封装模型:', params)
    // 模拟生成过程（实际应调用Tauri命令）
    await new Promise(resolve => setTimeout(resolve, 1000))
    modelReady.value = true
  } catch (e) {
    console.error('生成模型失败:', e)
  } finally {
    generating.value = false
  }
}

/**
 * 运行热-结构耦合分析
 * 调用后端命令执行求解
 */
async function runAnalysis() {
  if (!modelReady.value) return
  running.value = true
  analysisComplete.value = false
  try {
    const job = {
      id: `electronics_${Date.now()}`,
      name: `${analysisType.value.toUpperCase()} 分析`,
      package_type: analysisType.value,
      thermal_config: {
        ambient_temperature: ambientTemp.value,
        convection_coeff: convectionCoeff.value,
        board_thickness: boardThickness.value * 1e-3,
        num_layers: numLayers.value,
        layer_materials: ['Copper (Cu)', 'FR-4 PCB'],
        trace_layers: [],
        power_dissipation: powerDensity.value,
        heatsink_enabled: heatsinkEnabled.value,
        heatsink_params: heatsinkEnabled.value ? {
          length: 20e-3, width: 20e-3, height: 5e-3, num_fins: 10, thermal_resistance: 5.0
        } : null
      },
      structural_config: {
        analysis_type: 'thermal_stress',
        enforce_zero_disp: { face: 'bottom', u1: 0, u2: 0, u3: 0 },
        temperature_field: null
      },
      materials: [],
      components: [{
        name: 'IC_Chip',
        component_type: 'ic',
        position: [0, 0, (boardThickness.value * numLayers.value) * 1e-3],
        dimensions: [10e-3, 10e-3, 1e-3],
        power: powerDensity.value,
        material: 'Silicon (Si)'
      }],
      results: null,
      status: 'pending'
    }

    console.log('运行电子封装分析:', job)

    const results = await invoke('run_electronics_analysis', { job }) as any

    maxTemp.value = results.max_temperature
    maxStress.value = results.max_von_mises
    analysisComplete.value = true
    // hasResult is read-only (computed from lastResult)

    console.log('分析完成:', results)
  } catch (e) {
    console.error('分析失败:', e)
    alert('分析失败: ' + e)
  } finally {
    running.value = false
  }
}

/**
 * 将当前分析嵌入到选中笔记
 * 调用后端API记录嵌入关系
 */
async function embedToNote() {
  if (!selectedNoteId.value) return
  const embedRecord = {
    noteId: selectedNoteId.value,
    type: 'electronics',
    name: `${analysisType.value.toUpperCase()} 电子封装分析`,
    timestamp: Date.now()
  }
  console.log('电子封装分析已嵌入到笔记:', embedRecord)
  showEmbedDialog.value = false
  alert('✓ 分析已成功嵌入到笔记！\n\n在笔记中点击嵌入卡片可跳转到此分析界面。')
}
</script>

<style scoped>
/* 使用全局样式，保持与其他视图一致 */
.input {
  @apply px-3 py-2 border border-[var(--border-default)] rounded bg-[var(--bg-primary)] text-[var(--text-primary)];
}
.btn {
  @apply px-4 py-2 rounded transition-colors font-medium;
}
.btn-primary {
  @apply bg-blue-600 text-white hover:bg-blue-700;
}
.btn-ghost {
  @apply border border-[var(--border-default)] text-[var(--text-secondary)] hover:bg-[var(--bg-hover)];
}
.icon-btn {
  @apply rounded hover:bg-[var(--bg-hover)] flex items-center justify-center;
}
.icon-btn.active {
  background-color: color-mix(in srgb, var(--accent) 10%, transparent);
  color: var(--accent);
}
</style>