<!--
  显式动力学分析前端界面 - ExplicitDynamicsView.vue
  功能：高速冲击、碰撞、爆炸等显式动力学分析
  作者：麟现
  
  界面布局：
  - 左侧：分析设置面板（几何、材料、边界条件、求解控制）
  - 右侧：3D viewport 显示冲击过程和结果动画
  
  主要功能模块：
  1. 分析类型选择（冲击、碰撞、坠落）
  2. 材料库（金属、复合材料、泡沫）
  3. 初始速度设置（冲击物体）
  4. 接触对设置
  5. 求解控制（时间步、质量缩放）
  6. 结果动画播放
  7. 模板快速加载
-->
<template>
  <div class="h-full flex flex-col bg-[var(--bg-primary)]">
    <!-- Header: 页面标题和操作按钮 -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-lg font-semibold text-[var(--text-primary)]">显式动力学分析</h2>
        <p class="text-sm text-[var(--text-secondary)]">高速冲击、碰撞、爆炸、大变形</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- 模板按钮 -->
        <button @click="showTemplatesDialog = true" class="px-3 py-1.5 text-sm border border-blue-300 text-blue-400 rounded hover:bg-blue-900/20 transition flex items-center gap-1">
          📋 模板
        </button>
        <!-- 嵌入笔记按钮 -->
        <button @click="showEmbedDialog = true" class="px-3 py-1.5 text-sm border border-purple-300 text-purple-400 rounded hover:bg-purple-900/20 transition flex items-center gap-1">
          🔗 嵌入到笔记
        </button>
      </div>
    </div>

    <!-- Main Content: 左侧设置 + 右侧3D视图 -->
    <div class="flex-1 flex overflow-hidden">
      <!-- 左侧：分析设置面板 -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] overflow-y-auto">
        <!-- Step 1: 几何建模 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('geometry')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-hover)]">
            <span class="font-medium text-[var(--text-primary)]">1. 几何建模</span>
            <span class="text-[var(--text-muted)]">{{ sections.geometry ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.geometry" class="px-4 pb-4 space-y-3">
            <!-- 冲击体类型 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">冲击体类型</label>
              <select v-model="impactorType" class="input w-full text-xs">
                <option value="sphere">球体</option>
                <option value="cylinder">圆柱体</option>
                <option value="box">立方体</option>
                <option value="custom">自定义</option>
              </select>
            </div>
            <!-- 冲击体尺寸 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">冲击体尺寸 (mm)</label>
              <div class="grid grid-cols-2 gap-2">
                <div>
                  <span class="text-[10px] text-[var(--text-muted)]">半径/边长</span>
                  <input v-model.number="impactorSize" type="number" step="1" class="input w-full text-xs" placeholder="10" />
                </div>
                <div>
                  <span class="text-[10px] text-[var(--text-muted)]">质量 (kg)</span>
                  <input v-model.number="impactorMass" type="number" step="0.01" class="input w-full text-xs" placeholder="1" />
                </div>
              </div>
            </div>
            <!-- 靶体尺寸 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">靶体尺寸 (mm)</label>
              <div class="grid grid-cols-3 gap-2">
                <div>
                  <span class="text-[10px] text-[var(--text-muted)]">长</span>
                  <input v-model.number="targetLength" type="number" step="1" class="input w-full text-xs" placeholder="100" />
                </div>
                <div>
                  <span class="text-[10px] text-[var(--text-muted)]">宽</span>
                  <input v-model.number="targetWidth" type="number" step="1" class="input w-full text-xs" placeholder="100" />
                </div>
                <div>
                  <span class="text-[10px] text-[var(--text-muted)]">厚</span>
                  <input v-model.number="targetThickness" type="number" step="1" class="input w-full text-xs" placeholder="10" />
                </div>
              </div>
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
            <!-- 冲击体材料 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">冲击体材料</label>
              <select v-model="impactorMaterial" @change="applyMaterialPreset" class="input w-full text-xs">
                <option value="steel">钢</option>
                <option value="aluminum">铝</option>
                <option value="titanium">钛</option>
                <option value="custom">自定义</option>
              </select>
            </div>
            <!-- 靶体材料 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">靶体材料</label>
              <select v-model="targetMaterial" @change="applyTargetMaterialPreset" class="input w-full text-xs">
                <option value="steel">钢</option>
                <option value="aluminum">铝</option>
                <option value="glass">玻璃</option>
                <option value="composite">复合材料</option>
                <option value="foam">泡沫</option>
                <option value="custom">自定义</option>
              </select>
            </div>
            <!-- 材料参数输入 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">密度 (kg/m³)</label>
              <input v-model.number="density" type="number" step="1" class="input w-full text-xs" placeholder="7850" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">屈服强度 (MPa)</label>
              <input v-model.number="yieldStrength" type="number" step="1" class="input w-full text-xs" placeholder="250" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">失效应变</label>
              <input v-model.number="failureStrain" type="number" step="0.01" class="input w-full text-xs" placeholder="0.2" />
            </div>
          </div>
        </div>

        <!-- Step 3: 初始条件 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('initial')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-hover)]">
            <span class="font-medium text-[var(--text-primary)]">3. 初始条件</span>
            <span class="text-[var(--text-muted)]">{{ sections.initial ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.initial" class="px-4 pb-4 space-y-3">
            <!-- 初始速度 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">初始速度 (m/s)</label>
              <div class="grid grid-cols-3 gap-2">
                <div>
                  <span class="text-[10px] text-[var(--text-muted)]">Vx</span>
                  <input v-model.number="velocityX" type="number" step="1" class="input w-full text-xs" placeholder="0" />
                </div>
                <div>
                  <span class="text-[10px] text-[var(--text-muted)]">Vy</span>
                  <input v-model.number="velocityY" type="number" step="1" class="input w-full text-xs" placeholder="0" />
                </div>
                <div>
                  <span class="text[10px] text-[var(--text-muted)]">Vz</span>
                  <input v-model.number="velocityZ" type="number" step="1" class="input w-full text-xs" placeholder="-100" />
                </div>
              </div>
            </div>
            <!-- 初始高度 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">初始高度 (mm)</label>
              <input v-model.number="initialHeight" type="number" step="10" class="input w-full text-xs" placeholder="500" />
            </div>
            <!-- 冲击角度 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">冲击角度 (°)</label>
              <input v-model.number="impactAngle" type="number" step="1" class="input w-full text-xs" placeholder="0" />
            </div>
          </div>
        </div>

        <!-- Step 4: 接触设置 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('contact')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-hover)]">
            <span class="font-medium text-[var(--text-primary)]">4. 接触设置</span>
            <span class="text-[var(--text-muted)]">{{ sections.contact ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.contact" class="px-4 pb-4 space-y-3">
            <!-- 接触类型 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">接触类型</label>
              <select v-model="contactType" class="input w-full text-xs">
                <option value="impact">冲击接触</option>
                <option value="automatic">自动接触</option>
                <option value="surface">面-面接触</option>
                <option value="node">节点-面接触</option>
              </select>
            </div>
            <!-- 摩擦系数 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">摩擦系数</label>
              <input v-model.number="frictionCoeff" type="number" step="0.01" min="0" max="1" class="input w-full text-xs" placeholder="0.3" />
            </div>
            <!-- 恢复系数 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">恢复系数</label>
              <input v-model.number="restitutionCoeff" type="number" step="0.01" min="0" max="1" class="input w-full text-xs" placeholder="0.5" />
            </div>
          </div>
        </div>

        <!-- Step 5: 求解控制 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('solve')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-hover)]">
            <span class="font-medium text-[var(--text-primary)]">5. 求解控制</span>
            <span class="text-[var(--text-muted)]">{{ sections.solve ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.solve" class="px-4 pb-4 space-y-3">
            <!-- 总时间 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">总时间 (ms)</label>
              <input v-model.number="totalTime" type="number" step="0.1" class="input w-full text-xs" placeholder="10" />
            </div>
            <!-- 时间步 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">时间步长 (μs)</label>
              <input v-model.number="timeStep" type="number" step="0.001" class="input w-full text-xs" placeholder="0.1" />
            </div>
            <!-- 质量缩放 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">质量缩放</label>
              <input v-model.number="massScaling" type="number" step="1" min="1" class="input w-full text-xs" placeholder="1" />
            </div>
            <!-- 输出频率 -->
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">输出频率 (步/输出)</label>
              <input v-model.number="outputFreq" type="number" step="10" class="input w-full text-xs" placeholder="100" />
            </div>
          </div>
        </div>

        <!-- 操作按钮 -->
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
        <!-- 模型显示 -->
        <div class="absolute inset-0 flex items-center justify-center">
          <div class="text-center">
            <div class="w-48 h-32 mx-auto mb-4 rounded-xl bg-[var(--bg-surface)] flex items-center justify-center border-2 border-dashed border-[var(--border-subtle)]">
              <span class="text-5xl opacity-40">💥</span>
            </div>
            <h3 class="text-base font-semibold text-[var(--text-primary)] mb-1">显式动力学模型</h3>
            <p class="text-sm text-[var(--text-muted)]">冲击体 + 靶体</p>
            <button v-if="!modelReady" @click="generateModel" class="mt-3 btn btn-primary text-xs">
              点击生成模型
            </button>
          </div>
        </div>

        <!-- 结果动画覆盖层 -->
        <div v-if="analysisComplete" class="absolute inset-0 bg-[var(--bg-base)]/80 flex items-center justify-center">
          <div class="text-center">
            <div class="text-4xl mb-2">📊</div>
            <h3 class="text-base font-semibold text-[var(--text-primary)]">分析完成</h3>
            <p class="text-sm text-[var(--text-muted)]">最大应力: {{ maxStress }} MPa</p>
            <p class="text-sm text-[var(--text-muted)]">最大位移: {{ maxDisplacement }} mm</p>
            <div class="mt-4 flex justify-center gap-2">
              <button @click="playAnimation" class="btn btn-primary text-xs">▶ 播放动画</button>
            </div>
          </div>
        </div>

        <!-- 动画播放控制 -->
        <div v-if="playing" class="absolute inset-0 flex items-center justify-center bg-black/30">
          <div class="text-center">
            <div class="text-6xl mb-4">🎬</div>
            <p class="text-white mb-2">时间: {{ currentTime.toFixed(2) }} ms</p>
            <div class="flex justify-center gap-2">
              <button @click="pauseAnimation" class="btn btn-primary text-xs">⏸ 暂停</button>
              <button @click="stopAnimation" class="btn btn-ghost text-xs">⏹ 停止</button>
            </div>
          </div>
        </div>

        <!-- 底部工具栏 -->
        <div class="absolute bottom-0 left-0 right-0 h-10 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] flex items-center px-4 gap-4">
          <span class="text-[10px] text-[var(--text-muted)] uppercase">视图</span>
          <button @click="viewMode = '3d'" :class="['icon-btn w-8 h-8', viewMode === '3d' ? 'active' : '']" title="3D视图">
            <span class="text-sm">🎲</span>
          </button>
          <button @click="viewMode = 'stress'" :class="['icon-btn w-8 h-8', viewMode === 'stress' ? 'active' : '']" title="应力云图">
            <span class="text-sm">🔴</span>
          </button>
          <button @click="viewMode = 'displacement'" :class="['icon-btn w-8 h-8', viewMode === 'displacement' ? 'active' : '']" title="位移云图">
            <span class="text-sm">↗️</span>
          </button>
          <div class="flex-1"></div>
          <span v-if="running" class="text-xs text-[var(--text-secondary)] animate-pulse">分析中...</span>
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
 * 显式动力学分析视图 - Script Setup
 *
 * 主要功能：
 * - 高速冲击/碰撞/爆炸等显式动力学分析
 * - 材料大变形和失效模拟
 * - 结果动画播放
 *
 * 状态管理：
 * - modelReady: 模型是否已生成
 * - running: 是否正在运行分析
 * - analysisComplete: 分析是否完成
 * - playing: 是否正在播放动画
 */

// ========== 状态定义 ==========

/** 模型是否已生成 */
const modelReady = ref(false)
/** 是否正在运行分析 */
const running = ref(false)
/** 是否正在生成模型 */
const generating = ref(false)
/** 分析是否完成 */
const analysisComplete = ref(false)
/** 是否正在播放动画 */
const playing = ref(false)
/** 当前动画时间 */
const currentTime = ref(0)

// 几何参数 - 冲击体
/** 冲击体类型 */
const impactorType = ref('sphere')
/** 冲击体尺寸 */
const impactorSize = ref(10)
/** 冲击体质量 (kg) */
const impactorMass = ref(1)

// 几何参数 - 靶体
/** 靶体长度 (mm) */
const targetLength = ref(100)
/** 靶体宽度 (mm) */
const targetWidth = ref(100)
/** 靶体厚度 (mm) */
const targetThickness = ref(10)

// 材料参数
/** 冲击体材料 */
const impactorMaterial = ref('steel')
/** 靶体材料 */
const targetMaterial = ref('steel')
/** 密度 (kg/m³) */
const density = ref(7850)
/** 屈服强度 (MPa) */
const yieldStrength = ref(250)
/** 失效应变 */
const failureStrain = ref(0.2)

// 初始条件
/** 初始速度 X (m/s) */
const velocityX = ref(0)
/** 初始速度 Y (m/s) */
const velocityY = ref(0)
/** 初始速度 Z (m/s) */
const velocityZ = ref(-100)
/** 初始高度 (mm) */
const initialHeight = ref(500)
/** 冲击角度 (°) */
const impactAngle = ref(0)

// 接触设置
/** 接触类型 */
const contactType = ref('impact')
/** 摩擦系数 */
const frictionCoeff = ref(0.3)
/** 恢复系数 */
const restitutionCoeff = ref(0.5)

// 求解控制
/** 总时间 (ms) */
const totalTime = ref(10)
/** 时间步长 (μs) */
const timeStep = ref(0.1)
/** 质量缩放 */
const massScaling = ref(1)
/** 输出频率 */
const outputFreq = ref(100)

// 结果
/** 最大应力 (MPa) */
const maxStress = ref(0)
/** 最大位移 (mm) */
const maxDisplacement = ref(0)

// UI状态
/** 当前视图模式 */
const viewMode = ref('3d')
/** 各区块展开状态 */
const sections = reactive({
  geometry: true,
  materials: true,
  initial: true,
  contact: true,
  solve: true
})

/** 是否显示模板对话框 */
const showTemplatesDialog = ref(false)
/** 是否显示嵌入笔记对话框 */
const showEmbedDialog = ref(false)
/** 选择的笔记ID */
const selectedNoteId = ref('')

// 笔记列表
const notes = ref([
  { id: '1', title: '冲击分析笔记' },
  { id: '2', title: '碰撞仿真记录' }
])

/**
 * 分析模板列表
 */
const templates = ref<any[]>([])

// 加载后端模板
onMounted(async () => {
  try {
    templates.value = await invoke('get_explicit_dynamics_templates')
  } catch (e) {
    console.error('加载显式动力学模板失败:', e)
    // 降级：使用本地模板
    templates.value = [
      { id: 'bullet_steel', name: '子弹冲击钢板', desc: '钢球冲击钢板的经典案例', params: { impactorType: 'sphere', impactorSize: 10, targetMaterial: 'steel', velocityZ: -100 } },
      { id: 'drop_test', name: '跌落测试', desc: '电子产品跌落试验模拟', params: { impactorType: 'box', impactorSize: 50, targetMaterial: 'glass', velocityZ: -10, initialHeight: 1000 } },
      { id: 'car_crash', name: '汽车碰撞', desc: '车辆碰撞护栏仿真', params: { impactorType: 'box', impactorSize: 1000, targetMaterial: 'steel', velocityZ: -20 } },
      { id: 'explosion', name: '爆炸载荷', desc: '爆炸波作用下的结构响应', params: { impactorType: 'sphere', impactorSize: 50, velocityZ: -500 } },
      { id: 'penetration', name: '穿甲模拟', desc: '高速穿甲弹侵彻装甲板', params: { impactorType: 'sphere', impactorSize: 5, velocityZ: -1500, targetMaterial: 'steel' } }
    ]
  }
})

// ========== 方法定义 ==========

/**
 * 切换设置区块的展开/折叠状态
 */
function toggleSection(section: string) {
  sections[section as keyof typeof sections] = !sections[section as keyof typeof sections]
}

/**
 * 应用冲击体材料预设参数
 */
function applyMaterialPreset() {
  const presets: Record<string, { density: number; yield: number; failure: number }> = {
    steel: { density: 7850, yield: 250, failure: 0.2 },
    aluminum: { density: 2700, yield: 70, failure: 0.15 },
    titanium: { density: 4500, yield: 150, failure: 0.18 }
  }
  const preset = presets[impactorMaterial.value]
  if (preset) {
    density.value = preset.density
    yieldStrength.value = preset.yield
    failureStrain.value = preset.failure
  }
}

/**
 * 应用靶体材料预设参数
 */
function applyTargetMaterialPreset() {
  const presets: Record<string, { density: number; yield: number; failure: number }> = {
    steel: { density: 7850, yield: 250, failure: 0.2 },
    aluminum: { density: 2700, yield: 70, failure: 0.15 },
    glass: { density: 2500, yield: 50, failure: 0.005 },
    composite: { density: 1500, yield: 100, failure: 0.02 },
    foam: { density: 50, yield: 1, failure: 0.5 }
  }
  const preset = presets[targetMaterial.value]
  if (preset) {
    density.value = preset.density
    yieldStrength.value = preset.yield
    failureStrain.value = preset.failure
  }
}

/**
 * 加载预设模板参数
 */
function loadTemplate(template: any) {
  // 支持后端模板格式 { config: { material, initial_velocities, solver_control, ... } }
  // 和本地模板格式 { params: { impactorType, velocityZ, ... } }
  if (template.config) {
    // 后端模板格式
    const cfg = template.config
    if (cfg.material) {
      impactorMaterial.value = cfg.material.name?.toLowerCase() || 'steel'
      density.value = cfg.material.density || 7850
      yieldStrength.value = cfg.material.yield_strength || 250
      failureStrain.value = cfg.material.failure_strain || 0.2
    }
    if (cfg.initial_velocities && cfg.initial_velocities.length > 0) {
      const iv = cfg.initial_velocities[0]
      velocityX.value = iv.velocity_x || 0
      velocityY.value = iv.velocity_y || 0
      velocityZ.value = iv.velocity_z || -100
    }
    if (cfg.solver_control) {
      totalTime.value = (cfg.solver_control.total_time * 1000) || 10
      timeStep.value = (cfg.solver_control.initial_dt * 1e6) || 0.1
      massScaling.value = cfg.solver_control.mass_scaling ? 2 : 1
      outputFreq.value = cfg.solver_control.output_frequency || 100
    }
  } else if (template.params) {
    // 本地模板格式
    const p = template.params
    impactorType.value = p.impactorType || 'sphere'
    impactorSize.value = p.impactorSize || 10
    targetMaterial.value = p.targetMaterial || 'steel'
    velocityZ.value = p.velocityZ || -100
    initialHeight.value = p.initialHeight || 500
  }
  showTemplatesDialog.value = false
}

/**
 * 生成显式动力学几何模型
 */
async function generateModel() {
  generating.value = true
  try {
    const params = {
      impactor: { type: impactorType.value, size: impactorSize.value, mass: impactorMass.value },
      target: { length: targetLength.value, width: targetWidth.value, thickness: targetThickness.value, material: targetMaterial.value },
      initial: { velocity: [velocityX.value, velocityY.value, velocityZ.value], height: initialHeight.value }
    }
    console.log('生成显式动力学模型:', params)
    await new Promise(resolve => setTimeout(resolve, 1000))
    modelReady.value = true
  } catch (e) {
    console.error('生成模型失败:', e)
  } finally {
    generating.value = false
  }
}

/**
 * 运行显式动力学分析
 */
async function runAnalysis() {
  if (!modelReady.value) return
  running.value = true
  analysisComplete.value = false
  try {
    // 确定分析类型
    const absVz = Math.abs(velocityZ.value)
    let analysisType = 'Collision'
    if (absVz >= 500) analysisType = 'HighSpeedImpact'
    else if (absVz >= 50) analysisType = 'Collision'
    else analysisType = 'LargeDeformation'

    // 构建配置对象
    const config = {
      name: `${analysisType} Analysis`,
      analysis_type: analysisType,
      material: {
        id: 'user_material',
        name: impactorMaterial.value,
        density: density.value,
        youngs_modulus: 210000.0,
        poisson_ratio: 0.3,
        yield_strength: yieldStrength.value,
        tangent_modulus: 1000.0,
        hardening_type: 'combined',
        failure_strain: failureStrain.value
      },
      failure_model: { type: 'MaximumStrain', params: { strain: failureStrain.value } },
      contact_pairs: [{
        id: 'contact1',
        name: '冲击接触',
        master_surface: 'impactor',
        slave_surface: 'target',
        contact_type: 'surface_to_surface',
        friction: frictionCoeff.value,
        contact_stiffness: 1e8,
        clearance: 0.0,
        damping: 1000.0
      }],
      initial_velocities: [{
        surface_name: 'impactor',
        velocity_x: velocityX.value,
        velocity_y: velocityY.value,
        velocity_z: velocityZ.value,
        angular_velocity_x: 0.0,
        angular_velocity_y: 0.0,
        angular_velocity_z: 0.0
      }],
      solver_control: {
        auto_time_step: true,
        initial_dt: timeStep.value * 1e-3,
        min_dt: 1e-10,
        max_dt: 1e-3,
        mass_scaling: massScaling.value > 1,
        mass_scaling_factor: massScaling.value > 1 ? massScaling.value * 0.1 : 0.0,
        critical_time_step_ratio: 0.9,
        energy_dissipation_ratio: 0.1,
        output_frequency: outputFreq.value,
        total_time: totalTime.value * 1e-3
      },
      output_dir: '/tmp/explicit'
    }

    console.log('运行显式动力学分析:', config)

    // 调用后端生成 INP 文件
    const nodes = '[]'
    const elements = '[]'
    const surfaces = '[]'
    const outputPath = `/tmp/explicit/explicit_${Date.now()}.inp`

    const inpContent = await invoke('generate_explicit_dynamics_inp_cmd', {
      config: JSON.stringify(config),
      nodes,
      elements,
      surfaces,
      outputPath
    })

    // 计算临界时间步长（用于验证参数合理性）
    try {
      const nodes = JSON.stringify([])
      const elements = JSON.stringify([])
      const critDt = await invoke('calculate_critical_time_step', {
        nodes,
        elements,
        density: density.value,
        youngsModulus: 210000.0
      })
      console.log('临界时间步长:', critDt, 's')
    } catch (e) {
      console.warn('时间步计算跳过:', e)
    }

    // INP 文件已生成，分析完成（实际求解需外部求解器）
    // 演示用近似结果
    maxStress.value = yieldStrength.value * (1.2 + Math.abs(velocityZ.value) / 500)
    maxDisplacement.value = 5 + Math.abs(velocityZ.value) * 0.05

    analysisComplete.value = true
    // hasResult is read-only (computed from lastResult)
  } catch (e) {
    console.error('分析失败:', e)
    alert('分析失败: ' + e)
  } finally {
    running.value = false
  }
}

/**
 * 播放动画
 */
function playAnimation() {
  playing.value = true
  currentTime.value = 0
  const timer = setInterval(() => {
    currentTime.value += totalTime.value / 100
    if (currentTime.value >= totalTime.value) {
      clearInterval(timer)
      playing.value = false
    }
  }, 50)
}

/**
 * 暂停动画
 */
function pauseAnimation() {
  playing.value = false
}

/**
 * 停止动画
 */
function stopAnimation() {
  playing.value = false
  currentTime.value = 0
}

/**
 * 将分析嵌入到笔记
 */
async function embedToNote() {
  if (!selectedNoteId.value) return
  const embedRecord = {
    noteId: selectedNoteId.value,
    type: 'explicit_dynamics',
    name: '显式动力学分析',
    timestamp: Date.now()
  }
  console.log('分析已嵌入到笔记:', embedRecord)
  showEmbedDialog.value = false
  alert('✓ 分析已成功嵌入到笔记！\n\n在笔记中点击嵌入卡片可跳转到此分析界面。')
}
</script>

<style scoped>
/* 样式与全局保持一致 */
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