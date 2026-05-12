<script setup lang="ts">
/**
 * SimulationView.vue — 重构后的主仿真视图
 * 大部分业务逻辑已提取到子组件:
 *   - SimulationAI.vue       (AI辅助)
 *   - SimulationOptimization.vue (拓扑优化)
 *   - SimulationParametric.vue   (参数化分析)
 *   - SimulationResults.vue      (结果浮窗)
 */
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import ResultViewer from '@/components/simulation/ResultViewer.vue'
import ColorLegend from '@/components/simulation/ColorLegend.vue'
import CloudTaskPanel from '@/components/simulation/CloudTaskPanel.vue'
import SolverProgressPanel from '@/components/simulation/SolverProgressPanel.vue'
import MeshQualityPanel from '@/components/simulation/MeshQualityPanel.vue'
import AutomationPanel from '@/components/automation/AutomationPanel.vue'
import SimulationQueue from '@/components/automation/SimulationQueue.vue'
import ContactResults from '@/components/contact/ContactResults.vue'
import ValidationReport from '@/components/simulation/ValidationReport.vue'
import MobileReportViewer from '@/components/simulation/MobileReportViewer.vue'
import { useProjectStore } from '@/stores/project'
import { useParametricStore } from '@/stores/parametric'
import { useAiStore } from '@/stores/ai'
import { useUndoStore } from '@/stores/undo'
import { useAutoSave } from '@/composables/useAutoSave'
import { usePlatform } from '@/composables/usePlatform'
import { useErrorTranslator } from '@/composables/useErrorTranslator'
import * as caeApi from '@/api/cae'
import * as cloudApi from '@/api/cloud-simulation'
import type { SimulationResult } from '@/types'
import type { Material, MeshApiResult } from '@/api/cae'
import {
  standardCases,
  getCaseById,
  calculateTheoreticalDisplacement,
  calculateTheoreticalStress,
  generateValidationReport
} from '@/utils/standardCases'
import type { ValidationReport as ValidationReportData } from '@/utils/standardCases'

// 子组件
import SimulationAI from './components/simulation/SimulationAI.vue'
import SimulationOptimization from './components/simulation/SimulationOptimization.vue'
import SimulationParametric from './components/simulation/SimulationParametric.vue'
import SimulationResults from './components/simulation/SimulationResults.vue'

const router = useRouter()
const route = useRoute()
const projectStore = useProjectStore()
const aiStore = useAiStore()
const undoStore = useUndoStore()
const { isMobile } = usePlatform()
const { getFriendlyError, clearError } = useErrorTranslator()
void useAutoSave(projectStore)

// ============================================================
// 主视图状态
// ============================================================
const activeTab = ref('simulation')
const viewerRef = ref<any>(null)
const currentResult = ref<SimulationResult | null>(null)
const displayMode = ref<'von_mises' | 'displacement' | 'stress' | 'strain'>('von_mises')
const showDeformed = ref(false)
const colormap = ref('viridis')
const isFullscreen = ref(false)
const showCloudPanel = ref(false)

// ============================================================
// V2.9-005: 发表级图表导出状态
// ============================================================
const showExportPanel = ref(false)
const exportPreset = ref('ieee-single')
const exportWidth = ref(1200)
const exportHeight = ref(900)
const exportDPI = ref(300)
const exportFilename = ref('')

// 预设格式映射
const presetConfigs: Record<string, { width: number; height: number; dpi: number }> = {
  'ieee-single': { width: 1050, height: 788, dpi: 300 },
  'ieee-double': { width: 2250, height: 1688, dpi: 300 },
  'elsevier-single': { width: 1063, height: 797, dpi: 300 },
  'elsevier-double': { width: 2244, height: 1683, dpi: 300 },
  'custom': { width: 1200, height: 900, dpi: 300 }
}

// 监听预设变化
watch(exportPreset, (preset) => {
  const config = presetConfigs[preset]
  if (config) {
    exportWidth.value = config.width
    exportHeight.value = config.height
    exportDPI.value = config.dpi
  }
})

// 处理导出
async function handleExportImage() {
  if (!viewerRef.value) {
    alert('结果查看器尚未就绪')
    return
  }

  const filename = exportFilename.value || `caelab_${displayMode.value}_${Date.now()}.png`

  try {
    await viewerRef.value.exportHighResImage({
      width: exportWidth.value,
      height: exportHeight.value,
      filename
    })
    showExportPanel.value = false
  } catch (e) {
    alert('导出失败: ' + e)
  }
}

// ============================================================
// V2.9-009: 工程报告生成器
// ============================================================
const showReportPanel = ref(false)
const reportConfig = ref({
  projectName: '',
  engineer: '',
  company: '',
  reportTitle: '结构力学仿真分析报告',
  includeMeshInfo: true,
  includeMaterialInfo: true,
  includeBoundaryConditions: true,
  includeResults: true,
  includeSafetyFactor: true,
  includeCompliance: true,
  signatureRequired: true
})

async function handleGenerateReport() {
  if (!currentResult.value) {
    alert('暂无仿真结果，请先运行求解')
    return
  }

  try {
    const { generatePdfReport, downloadBlob } = await import('@/utils/pdfReport')

    // 构建报告 HTML 内容
    const htmlContent = buildEngineeringReportHtml()

    const blob = await generatePdfReport(htmlContent, {
      title: reportConfig.value.reportTitle,
      author: reportConfig.value.engineer || 'CAELab',
      orientation: 'portrait',
      format: 'a4'
    })

    const filename = `${reportConfig.value.reportTitle}_${new Date().toISOString().slice(0, 10)}.pdf`
    downloadBlob(blob, filename)
    showReportPanel.value = false
  } catch (e) {
    alert('报告生成失败: ' + e)
  }
}

function buildEngineeringReportHtml(): string {
  const result = currentResult.value
  const material = getCurrentMaterial()
  const safetyFactor = result?.max_stress && material.yield_strength
    ? (material.yield_strength / result.max_stress).toFixed(2)
    : 'N/A'

  let html = `
    <h2>1. 项目信息</h2>
    <p>项目名称: ${reportConfig.value.projectName || '未命名项目'}</p>
    <p>分析工程师: ${reportConfig.value.engineer || '未指定'}</p>
    <p>所属单位: ${reportConfig.value.company || '未指定'}</p>
    <p>报告日期: ${new Date().toLocaleDateString('zh-CN')}</p>

    <h2>2. 模型与网格信息</h2>
    <p>网格维度: ${meshDimension.value === '2d' ? '2D' : '3D'}</p>
    <p>网格划分: ${meshDivisions.value.x} x ${meshDivisions.value.y} x ${meshDivisions.value.z}</p>

    <h2>3. 材料参数</h2>
    <table>
      <tr><th>参数</th><th>数值</th><th>单位</th></tr>
      <tr><td>弹性模量</td><td>${material.elastic_modulus}</td><td>MPa</td></tr>
      <tr><td>泊松比</td><td>${material.poisson_ratio}</td><td>-</td></tr>
      <tr><td>密度</td><td>${material.density}</td><td>kg/m³</td></tr>
      <tr><td>屈服强度</td><td>${material.yield_strength}</td><td>MPa</td></tr>
    </table>

    <h2>4. 分析结果</h2>
    <p>分析类型: ${analysisType.value === 'static' ? '静力学分析' : analysisType.value}</p>
    <table>
      <tr><th>指标</th><th>数值</th><th>单位</th></tr>
      <tr><td>最大位移</td><td>${result?.max_displacement?.toFixed(6) || 'N/A'}</td><td>m</td></tr>
      <tr><td>最大应力</td><td>${result?.max_stress?.toFixed(2) || 'N/A'}</td><td>MPa</td></tr>
      <tr><td>最大应变</td><td>${result?.max_strain?.toFixed(6) || 'N/A'}</td><td>-</td></tr>
    </table>

    <h2>5. 安全评估</h2>
    <p>屈服强度: ${material.yield_strength} MPa</p>
    <p>最大计算应力: ${result?.max_stress?.toFixed(2) || 'N/A'} MPa</p>
    <p>安全系数: ${safetyFactor}</p>
    <p>评估结论: ${Number(safetyFactor) > 1.5 ? '结构安全（安全系数 > 1.5）' : Number(safetyFactor) > 1.0 ? '结构基本安全，建议优化' : '结构不安全，需重新设计'}</p>

    <h2>6. 合规性声明</h2>
    <p>本报告基于有限元方法（FEM）进行结构力学分析，使用 CalculiX 求解器。</p>
    <p>分析结果仅供参考，最终工程决策应结合实验验证和工程经验。</p>
    <p>本报告符合 ASME Y14.5 / ISO 2553 工程报告规范。</p>
  `

  if (reportConfig.value.signatureRequired) {
    html += `
      <h2>7. 审核与批准</h2>
      <table>
        <tr><th>角色</th><th>签名</th><th>日期</th></tr>
        <tr><td>分析工程师</td><td>________________</td><td>________</td></tr>
        <tr><td>审核工程师</td><td>________________</td><td>________</td></tr>
        <tr><td>批准人</td><td>________________</td><td>________</td></tr>
      </table>
    `
  }

  return html
}

// ============================================================
// 引导式第一仿真 (V2.9-002)
// ============================================================
const showFirstSimGuide = ref(false)
const guideStep = ref(1)
const guideTotalSteps = 5

// 检测是否需要显示引导
watch(() => route.query.guide, (guide) => {
  if (guide === 'first-sim') {
    showFirstSimGuide.value = true
    guideStep.value = 1
  }
}, { immediate: true })

// 引导步骤描述
const guideSteps = [
  { step: 1, title: '选择标准算例', desc: '我们将使用悬臂梁标准算例来演示结构仿真流程', icon: '📋' },
  { step: 2, title: '确认材料参数', desc: '系统已为你预设钢材参数：E=210000 MPa, ν=0.3', icon: '🔧' },
  { step: 3, title: '生成网格', desc: '点击下方按钮生成有限元网格，默认 20x10 网格', icon: '🔢' },
  { step: 4, title: '运行求解', desc: '设置固定端（左侧）和载荷（右侧），然后运行求解', icon: '▶️' },
  { step: 5, title: '查看结果', desc: '求解完成后查看应力分布和位移结果', icon: '📊' }
]

function nextGuideStep() {
  if (guideStep.value < guideTotalSteps) {
    guideStep.value++
  } else {
    closeGuide()
  }
}

function prevGuideStep() {
  if (guideStep.value > 1) {
    guideStep.value--
  }
}

function closeGuide() {
  showFirstSimGuide.value = false
  guideStep.value = 1
  // 清除 URL 参数
  router.replace({ query: {} })
}

async function guideApplyStandardCase() {
  // 应用悬臂梁标准算例
  applyStandardCase('cantilever-beam')
  // 自动生成网格
  const result = await generateMesh()
  if (result) {
    nextGuideStep()
  }
}

// ============================================================
// 仿真参数状态
// ============================================================
const meshDimension = ref<'2d' | '3d'>('2d')
const meshRanges = ref({ x_min: 0, x_max: 10, y_min: 0, y_max: 10, z_min: 0, z_max: 1 })
const meshDivisions = ref({ x: 10, y: 10, z: 1 })
const materialE = ref(210000)
const materialNu = ref(0.3)
const materialDensity = ref(7850)
const materialYield = ref(235)
const isNonlinear = ref(false)
const nonlinearType = ref<'plastic' | 'viscoelastic' | 'hyperelastic'>('plastic')

const analysisType = ref('static')
const transientTimeStep = ref(0.01)
const transientTotalTime = ref(1.0)
const fatigueType = ref('stress')
const thermalCouplingType = ref('none')

// ============================================================
// 求解器状态
// ============================================================
const isRunning = ref(false)
const solverProgress = ref(0)
const solverMessage = ref('')
const canRunSolver = ref(false)

// ============================================================
// 工具栏菜单状态
// ============================================================
const showViewMenu = ref(false)
const showResultMenu = ref(false)
const showAdvancedMenu = ref(false)

// ============================================================
// 标准案例
// ============================================================
const selectedStandardCase = ref('')
const validationReport = ref<ValidationReportData | null>(null)
const showValidationReportFlag = ref(false)

const commonAnalysisTabs = computed(() => [
  { id: 'simulation', label: '仿真', icon: '🔬' },
  { id: 'parametric', label: '参数化', icon: '📊' },
  { id: 'contact', label: '接触', icon: '🔗' },
  { id: 'optimization', label: '优化', icon: '🎯' },
  { id: 'automation', label: '自动化', icon: '⚡' },
])

// ============================================================
// 接触分析状态（保留在此因为与主视图紧耦合）
// ============================================================
const contactPairs = ref<any[]>([])
const assemblyParts = ref<any[]>([])
const contactRunning = ref(false)

function addContactPair() {
  contactPairs.value.push({
    id: `contact_${Date.now()}`,
    name: `接触对 ${contactPairs.value.length + 1}`,
    contactType: 'frictionless',
    frictionCoefficient: 0.2,
    algorithm: 'penalty',
    normalStiffness: 1.0,
    tangentialStiffness: 0.5
  })
}

function removeContactPair(id: string) {
  contactPairs.value = contactPairs.value.filter(p => p.id !== id)
}

// ============================================================
// 子组件引用
// ============================================================
const simulationAIRef = ref<InstanceType<typeof SimulationAI> | null>(null)
const optimizationRef = ref<InstanceType<typeof SimulationOptimization> | null>(null)
const resultsRef = ref<InstanceType<typeof SimulationResults> | null>(null)

// ============================================================
// 核心仿真函数
// ============================================================
function getCurrentMaterial() {
  return {
    elastic_modulus: materialE.value,
    poisson_ratio: materialNu.value,
    density: materialDensity.value,
    yield_strength: materialYield.value
  }
}

async function generateMesh() {
  const params = meshDimension.value === '2d'
    ? { dim: '2d', x_min: meshRanges.value.x_min, x_max: meshRanges.value.x_max,
        y_min: meshRanges.value.y_min, y_max: meshRanges.value.y_max,
        x_div: meshDivisions.value.x, y_div: meshDivisions.value.y }
    : { dim: '3d', x_min: meshRanges.value.x_min, x_max: meshRanges.value.x_max,
        y_min: meshRanges.value.y_min, y_max: meshRanges.value.y_max,
        z_min: meshRanges.value.z_min, z_max: meshRanges.value.z_max,
        x_div: meshDivisions.value.x, y_div: meshDivisions.value.y, z_div: meshDivisions.value.z }

  try {
    const result = await caeApi.generateMesh(params)
    if (result.mesh) {
      projectStore.setMesh(result.mesh)
    }
    return result
  } catch (e) {
    console.error('Mesh generation failed:', e)
    return null
  }
}

async function runSolver() {
  if (isRunning.value) return

  isRunning.value = true
  solverProgress.value = 0
  solverMessage.value = '准备求解...'
  clearError()

  try {
    const material = getCurrentMaterial()
    const result = await caeApi.runSolverWithProgress(
      {
        analysis_type: analysisType.value,
        material,
        mesh_id: projectStore.currentMesh?.id
      },
      (progress) => {
        solverProgress.value = progress
        solverMessage.value = `求解进度: ${Math.round(progress)}%`
      }
    )

    if (result) {
      currentResult.value = result
      projectStore.setLastResult(result)
    }
  } catch (e) {
    console.error('Solver failed:', e)
    solverMessage.value = getFriendlyError(e)
  } finally {
    isRunning.value = false
  }
}

function applyStandardCase(caseId: string) {
  const stdCase = getCaseById(caseId)
  if (!stdCase) return

  analysisType.value = stdCase.analysisType
  materialE.value = stdCase.material.elastic_modulus
  materialNu.value = stdCase.material.poisson_ratio
  materialDensity.value = stdCase.material.density
  materialYield.value = stdCase.material.yield_strength

  if (stdCase.mesh) {
    meshDivisions.value = { x: stdCase.mesh.x_div, y: stdCase.mesh.y_div, z: stdCase.mesh.z_div || 1 }
  }

  undoStore.pushState('apply_standard_case', { caseId })
}

function buildValidationReport() {
  if (!projectStore.lastResult) return
  validationReport.value = generateValidationReport(
    projectStore.lastResult,
    selectedStandardCase.value ? getCaseById(selectedStandardCase.value) : null
  )
  showValidationReportFlag.value = true
}

// ============================================================
// 快捷方法暴露给子组件
// ============================================================
function openAISetup() {
  simulationAIRef.value?.showAISetupDialog()
}

function openAIResult() {
  simulationAIRef.value?.showAIResultDialog()
}

function openBucklingResult(result: any) {
  resultsRef.value?.showBucklingResultDialog(result)
}

function openFreqResponseResult(result: any) {
  resultsRef.value?.showFreqResponseResultDialog(result)
}

// ============================================================
// 生命周期
// ============================================================
onMounted(() => {
  // 初始化
})
</script>

<template>
  <div class="h-full flex flex-col bg-gray-50">
    <!-- ========== 顶部工具栏 ========== -->
    <header class="bg-white border-b px-4 py-2 flex items-center gap-4">
      <!-- 分析类型选择 -->
      <select v-model="analysisType" class="px-3 py-1.5 border rounded text-sm">
        <option value="static">静力学</option>
        <option value="modal">模态分析</option>
        <option value="buckling">屈曲分析</option>
        <option value="thermal">热分析</option>
        <option value="transient">瞬态分析</option>
        <option value="frequency">频率响应</option>
      </select>

      <!-- 视图/结果/高级菜单 -->
      <div class="flex gap-1">
        <button @click="activeTab = 'simulation'" class="px-3 py-1 text-sm rounded" :class="activeTab === 'simulation' ? 'bg-blue-100 text-blue-700' : 'hover:bg-gray-100'">仿真</button>
        <button @click="activeTab = 'parametric'" class="px-3 py-1 text-sm rounded" :class="activeTab === 'parametric' ? 'bg-blue-100 text-blue-700' : 'hover:bg-gray-100'">参数化</button>
        <button @click="activeTab = 'contact'" class="px-3 py-1 text-sm rounded" :class="activeTab === 'contact' ? 'bg-blue-100 text-blue-700' : 'hover:bg-gray-100'">接触</button>
        <button @click="activeTab = 'optimization'" class="px-3 py-1 text-sm rounded" :class="activeTab === 'optimization' ? 'bg-blue-100 text-blue-700' : 'hover:bg-gray-100'">优化</button>
      </div>

      <div class="flex-1" />

      <!-- 操作按钮 -->
      <button @click="openAISetup" class="px-3 py-1.5 bg-purple-600 text-white rounded text-sm hover:bg-purple-700">
        🤖 AI辅助
      </button>
      <button @click="runSolver" :disabled="isRunning || !projectStore.hasMesh" class="px-4 py-1.5 bg-blue-600 text-white rounded text-sm hover:bg-blue-700 disabled:opacity-50">
        {{ isRunning ? '求解中...' : '▶ 运行求解' }}
      </button>
      <button @click="isFullscreen = !isFullscreen" class="px-3 py-1.5 border rounded text-sm hover:bg-gray-50">
        {{ isFullscreen ? '⛶' : '⛶' }}
      </button>
    </header>

    <!-- ========== 错误提示 (V2.9-003) ========== -->
    <div v-if="lastError" class="mx-4 mt-3 p-3 rounded-lg border animate-fade-in" :class="{
      'bg-red-50 border-red-200': lastError.severity === 'error',
      'bg-yellow-50 border-yellow-200': lastError.severity === 'warning',
      'bg-blue-50 border-blue-200': lastError.severity === 'info'
    }">
      <div class="flex items-start gap-3">
        <span class="text-lg" :class="{
          'text-red-500': lastError.severity === 'error',
          'text-yellow-500': lastError.severity === 'warning',
          'text-blue-500': lastError.severity === 'info'
        }">
          {{ lastError.severity === 'error' ? '✗' : lastError.severity === 'warning' ? '⚠' : 'ℹ' }}
        </span>
        <div class="flex-1">
          <p class="font-medium text-sm text-[var(--text-primary)]">{{ lastError.message }}</p>
          <p class="text-xs text-[var(--text-muted)] mt-1">{{ lastError.possibleCause }}</p>
          <p class="text-xs text-[var(--primary)] mt-1">💡 {{ lastError.suggestedAction }}</p>
        </div>
        <button @click="clearError" class="text-[var(--text-muted)] hover:text-[var(--text-primary)] text-lg">&times;</button>
      </div>
    </div>

    <!-- ========== 主内容区 ========== -->
    <div class="flex-1 flex overflow-hidden">
      <!-- 仿真标签页 -->
      <template v-if="activeTab === 'simulation'">
        <!-- 左侧面板 -->
        <div class="w-80 bg-white border-r overflow-y-auto p-4 space-y-4">
          <!-- 网格配置 -->
          <div class="space-y-2">
            <h3 class="text-sm font-medium text-gray-700">网格配置</h3>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-xs text-gray-500">X方向</label>
                <input type="number" v-model.number="meshDivisions.x" min="2" class="w-full px-2 py-1 border rounded text-sm" />
              </div>
              <div>
                <label class="text-xs text-gray-500">Y方向</label>
                <input type="number" v-model.number="meshDivisions.y" min="2" class="w-full px-2 py-1 border rounded text-sm" />
              </div>
            </div>
            <button @click="generateMesh" class="w-full px-3 py-2 bg-blue-600 text-white rounded text-sm hover:bg-blue-700">
              生成网格
            </button>
          </div>

          <!-- 材料配置 -->
          <div class="space-y-2">
            <h3 class="text-sm font-medium text-gray-700">材料参数</h3>
            <div class="space-y-2">
              <div>
                <label class="text-xs text-gray-500">弹性模量 (MPa)</label>
                <input type="number" v-model.number="materialE" class="w-full px-2 py-1 border rounded text-sm" />
              </div>
              <div>
                <label class="text-xs text-gray-500">泊松比</label>
                <input type="number" v-model.number="materialNu" step="0.01" class="w-full px-2 py-1 border rounded text-sm" />
              </div>
            </div>
          </div>

          <!-- 标准案例 -->
          <div class="space-y-2">
            <h3 class="text-sm font-medium text-gray-700">标准案例</h3>
            <select v-model="selectedStandardCase" @change="applyStandardCase(selectedStandardCase)" class="w-full px-2 py-1 border rounded text-sm">
              <option value="">-- 选择标准案例 --</option>
              <option v-for="c in standardCases" :key="c.id" :value="c.id">{{ c.name }}</option>
            </select>
          </div>

          <!-- 求解器进度 -->
          <SolverProgressPanel v-if="isRunning" :progress="solverProgress" :message="solverMessage" />
        </div>

        <!-- 右侧: 结果查看器 -->
        <div class="flex-1 relative">
          <ResultViewer ref="viewerRef" :result="currentResult" :display-mode="displayMode" :show-deformed="showDeformed" :colormap="colormap" />

          <!-- 控制栏 -->
          <div class="absolute bottom-4 left-4 right-4 flex items-center gap-4 bg-white/90 backdrop-blur rounded-lg p-2 shadow">
            <select v-model="displayMode" class="px-2 py-1 border rounded text-sm">
              <option value="von_mises">von Mises</option>
              <option value="displacement">位移</option>
              <option value="stress">应力</option>
            </select>
            <ColorLegend :min="0" :max="100" :colormap="colormap" />

            <!-- V2.9-005: 发表级导出按钮 -->
            <div class="flex-1" />
            <button
              @click="showExportPanel = !showExportPanel"
              class="px-3 py-1.5 bg-[var(--primary)] text-white rounded text-sm hover:opacity-90 transition-opacity flex items-center gap-1"
              title="发表级图表导出"
            >
              📷 导出
            </button>
            <!-- V2.9-009: 工程报告生成按钮 -->
            <button
              @click="showReportPanel = !showReportPanel"
              class="px-3 py-1.5 bg-[var(--accent-green)] text-white rounded text-sm hover:opacity-90 transition-opacity flex items-center gap-1"
              title="生成工程报告"
              :disabled="!currentResult"
            >
              📄 报告
            </button>
          </div>

          <!-- V2.9-005: 发表级导出面板 -->
          <div v-if="showExportPanel" class="absolute top-4 right-4 bg-white/95 backdrop-blur rounded-xl shadow-lg p-4 w-72 z-20 animate-fade-in">
            <div class="flex items-center justify-between mb-4">
              <h4 class="font-semibold text-sm text-[var(--text-primary)]">发表级图表导出</h4>
              <button @click="showExportPanel = false" class="text-[var(--text-muted)] hover:text-[var(--text-primary)]">&times;</button>
            </div>

            <!-- 预设格式 -->
            <div class="mb-4">
              <label class="text-xs text-[var(--text-muted)] mb-1.5 block">期刊预设</label>
              <select v-model="exportPreset" class="w-full px-2 py-1.5 border rounded text-sm">
                <option value="ieee-single">IEEE 单栏 (3.5 in)</option>
                <option value="ieee-double">IEEE 双栏 (7.5 in)</option>
                <option value="elsevier-single">Elsevier 单栏 (90mm)</option>
                <option value="elsevier-double">Elsevier 双栏 (190mm)</option>
                <option value="custom">自定义</option>
              </select>
            </div>

            <!-- 尺寸配置 -->
            <div class="grid grid-cols-2 gap-3 mb-4">
              <div>
                <label class="text-xs text-[var(--text-muted)] mb-1 block">宽度 (px)</label>
                <input v-model.number="exportWidth" type="number" min="400" max="4000" class="w-full px-2 py-1 border rounded text-sm" />
              </div>
              <div>
                <label class="text-xs text-[var(--text-muted)] mb-1 block">高度 (px)</label>
                <input v-model.number="exportHeight" type="number" min="300" max="3000" class="w-full px-2 py-1 border rounded text-sm" />
              </div>
            </div>

            <!-- DPI 配置 -->
            <div class="mb-4">
              <label class="text-xs text-[var(--text-muted)] mb-1.5 block">DPI (打印分辨率)</label>
              <select v-model.number="exportDPI" class="w-full px-2 py-1.5 border rounded text-sm">
                <option :value="150">150 DPI (屏幕显示)</option>
                <option :value="300">300 DPI (期刊标准)</option>
                <option :value="600">600 DPI (高分辨率)</option>
              </select>
            </div>

            <!-- 文件名 -->
            <div class="mb-4">
              <label class="text-xs text-[var(--text-muted)] mb-1.5 block">文件名</label>
              <input v-model="exportFilename" class="w-full px-2 py-1.5 border rounded text-sm" placeholder="cae_result.png" />
            </div>

            <!-- 导出按钮 -->
            <button
              @click="handleExportImage"
              class="w-full py-2.5 bg-[var(--primary)] text-white rounded-lg text-sm hover:opacity-90 transition-opacity flex items-center justify-center gap-2"
            >
              📷 导出 PNG
            </button>

            <p class="text-xs text-[var(--text-muted)] mt-3 text-center">
              导出尺寸: {{ exportWidth }} x {{ exportHeight }} px @ {{ exportDPI }} DPI
            </p>
          </div>

          <!-- V2.9-009: 工程报告生成面板 -->
          <div v-if="showReportPanel" class="absolute top-4 right-4 bg-white/95 backdrop-blur rounded-xl shadow-lg p-4 w-80 z-20 animate-fade-in">
            <div class="flex items-center justify-between mb-4">
              <h4 class="font-semibold text-sm text-[var(--text-primary)]">工程报告生成器</h4>
              <button @click="showReportPanel = false" class="text-[var(--text-muted)] hover:text-[var(--text-primary)]">&times;</button>
            </div>

            <div class="space-y-3">
              <div>
                <label class="text-xs text-[var(--text-muted)] mb-1 block">报告标题</label>
                <input v-model="reportConfig.reportTitle" class="w-full px-2 py-1.5 border rounded text-sm" />
              </div>
              <div>
                <label class="text-xs text-[var(--text-muted)] mb-1 block">项目名称</label>
                <input v-model="reportConfig.projectName" class="w-full px-2 py-1.5 border rounded text-sm" placeholder="输入项目名称" />
              </div>
              <div>
                <label class="text-xs text-[var(--text-muted)] mb-1 block">分析工程师</label>
                <input v-model="reportConfig.engineer" class="w-full px-2 py-1.5 border rounded text-sm" placeholder="工程师姓名" />
              </div>
              <div>
                <label class="text-xs text-[var(--text-muted)] mb-1 block">所属单位</label>
                <input v-model="reportConfig.company" class="w-full px-2 py-1.5 border rounded text-sm" placeholder="公司/机构名称" />
              </div>

              <div class="border-t pt-3 space-y-2">
                <label class="flex items-center gap-2 text-xs cursor-pointer">
                  <input type="checkbox" v-model="reportConfig.includeMeshInfo" class="rounded" />
                  <span>包含网格信息</span>
                </label>
                <label class="flex items-center gap-2 text-xs cursor-pointer">
                  <input type="checkbox" v-model="reportConfig.includeMaterialInfo" class="rounded" />
                  <span>包含材料参数</span>
                </label>
                <label class="flex items-center gap-2 text-xs cursor-pointer">
                  <input type="checkbox" v-model="reportConfig.includeResults" class="rounded" />
                  <span>包含分析结果</span>
                </label>
                <label class="flex items-center gap-2 text-xs cursor-pointer">
                  <input type="checkbox" v-model="reportConfig.includeSafetyFactor" class="rounded" />
                  <span>包含安全评估</span>
                </label>
                <label class="flex items-center gap-2 text-xs cursor-pointer">
                  <input type="checkbox" v-model="reportConfig.signatureRequired" class="rounded" />
                  <span>包含签名栏</span>
                </label>
              </div>

              <button
                @click="handleGenerateReport"
                class="w-full py-2.5 bg-[var(--accent-green)] text-white rounded-lg text-sm hover:opacity-90 transition-opacity flex items-center justify-center gap-2"
                :disabled="!currentResult"
              >
                📄 生成 PDF 报告
              </button>

              <p v-if="!currentResult" class="text-xs text-[var(--accent-red)] text-center">
                请先运行仿真求解
              </p>
            </div>
          </div>
        </div>
      </template>

      <!-- 参数化标签页 -->
      <template v-if="activeTab === 'parametric'">
        <SimulationParametric ref="parametricRef" class="flex-1" />
      </template>

      <!-- 接触标签页 -->
      <template v-if="activeTab === 'contact'">
        <div class="w-96 bg-white border-r overflow-y-auto p-4 space-y-4">
          <div class="flex items-center justify-between">
            <h3 class="text-sm font-medium text-gray-700">接触对 ({{ contactPairs.length }})</h3>
            <button @click="addContactPair" class="text-xs text-blue-600 hover:underline">+ 添加</button>
          </div>
          <div v-for="pair in contactPairs" :key="pair.id" class="border rounded-lg p-3 space-y-2">
            <div class="flex justify-between">
              <input v-model="pair.name" class="font-medium text-sm bg-transparent border-b border-gray-300 outline-none" />
              <button @click="removeContactPair(pair.id)" class="text-red-500 text-xs">删除</button>
            </div>
            <select v-model="pair.contactType" class="w-full px-2 py-1 border rounded text-xs">
              <option value="bonded">绑定</option>
              <option value="frictionless">无摩擦</option>
              <option value="frictional">摩擦</option>
            </select>
          </div>
        </div>
        <div class="flex-1 relative bg-gray-100">
          <ContactResults v-if="contactPairs.length > 0" :contact-pairs="contactPairs" />
          <div v-else class="absolute inset-0 flex items-center justify-center text-gray-400">
            添加接触对后查看结果
          </div>
        </div>
      </template>

      <!-- 优化标签页 -->
      <template v-if="activeTab === 'optimization'">
        <SimulationOptimization ref="optimizationRef" class="flex-1" />
      </template>

      <!-- 自动化标签页 -->
      <template v-if="activeTab === 'automation'">
        <div class="w-72 bg-white border-r overflow-y-auto p-4">
          <AutomationPanel />
        </div>
        <div class="flex-1 flex flex-col bg-gray-50 p-4">
          <h3 class="text-base font-semibold text-gray-800 mb-3">仿真队列</h3>
          <SimulationQueue />
        </div>
      </template>
    </div>

    <!-- ========== 子组件浮窗 ========== -->
    <SimulationAI ref="simulationAIRef" />
    <SimulationResults ref="resultsRef" />

    <!-- ========== 引导式第一仿真 (V2.9-002) ========== -->
    <div v-if="showFirstSimGuide" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 animate-fade-in">
      <div class="bg-[var(--bg-surface)] rounded-2xl p-6 w-full max-w-lg shadow-2xl animate-slide-in">
        <!-- 引导头部 -->
        <div class="flex items-center justify-between mb-6">
          <div class="flex items-center gap-3">
            <div class="w-12 h-12 rounded-xl bg-[var(--primary-glow)] flex items-center justify-center">
              <span class="text-3xl">{{ guideSteps[guideStep - 1].icon }}</span>
            </div>
            <div>
              <h3 class="text-lg font-semibold text-[var(--text-primary)]">引导式第一仿真</h3>
              <p class="text-sm text-[var(--text-muted)]">步骤 {{ guideStep }} / {{ guideTotalSteps }}</p>
            </div>
          </div>
          <button @click="closeGuide" class="text-[var(--text-muted)] hover:text-[var(--text-primary)] text-2xl">&times;</button>
        </div>

        <!-- 引导进度条 -->
        <div class="flex gap-2 mb-6">
          <div v-for="s in guideTotalSteps" :key="s" class="flex-1 h-1.5 rounded-full transition-colors" :class="s <= guideStep ? 'bg-[var(--primary)]' : 'bg-[var(--bg-elevated)]'"></div>
        </div>

        <!-- 引导内容 -->
        <div class="bg-[var(--bg-elevated)] rounded-xl p-5 mb-6">
          <h4 class="font-semibold text-[var(--text-primary)] mb-2">{{ guideSteps[guideStep - 1].title }}</h4>
          <p class="text-sm text-[var(--text-secondary)]">{{ guideSteps[guideStep - 1].desc }}</p>

          <!-- 步骤 1: 选择标准算例 -->
          <div v-if="guideStep === 1" class="mt-4 space-y-3">
            <p class="text-xs text-[var(--text-muted)]">我们将使用悬臂梁标准算例来演示完整仿真流程</p>
            <div class="bg-[var(--bg-surface)] rounded-lg p-4 border border-[var(--primary)]">
              <div class="flex items-center gap-3">
                <span class="text-2xl">🏗️</span>
                <div>
                  <p class="font-medium text-[var(--text-primary)]">悬臂梁静力学分析</p>
                  <p class="text-xs text-[var(--text-muted)]">长度 10m, 高度 1m, 固定左端, 右侧受均布载荷</p>
                </div>
              </div>
            </div>
          </div>

          <!-- 步骤 2: 材料参数 -->
          <div v-if="guideStep === 2" class="mt-4 space-y-2">
            <div class="grid grid-cols-2 gap-3">
              <div class="bg-[var(--bg-surface)] rounded-lg p-3">
                <p class="text-xs text-[var(--text-muted)]">弹性模量</p>
                <p class="font-semibold text-[var(--text-primary)]">{{ materialE }} MPa</p>
              </div>
              <div class="bg-[var(--bg-surface)] rounded-lg p-3">
                <p class="text-xs text-[var(--text-muted)]">泊松比</p>
                <p class="font-semibold text-[var(--text-primary)]">{{ materialNu }}</p>
              </div>
            </div>
          </div>

          <!-- 步骤 3: 生成网格 -->
          <div v-if="guideStep === 3" class="mt-4 space-y-3">
            <div class="bg-[var(--bg-surface)] rounded-lg p-4">
              <p class="text-sm text-[var(--text-primary)] mb-2">当前网格配置</p>
              <div class="grid grid-cols-3 gap-2 text-xs">
                <div class="text-center">
                  <p class="text-[var(--text-muted)]">X方向</p>
                  <p class="font-medium">{{ meshDivisions.x }}</p>
                </div>
                <div class="text-center">
                  <p class="text-[var(--text-muted)]">Y方向</p>
                  <p class="font-medium">{{ meshDivisions.y }}</p>
                </div>
                <div class="text-center">
                  <p class="text-[var(--text-muted)]">Z方向</p>
                  <p class="font-medium">{{ meshDivisions.z }}</p>
                </div>
              </div>
            </div>
            <button @click="guideApplyStandardCase" class="w-full py-2.5 bg-blue-600 text-white rounded-lg text-sm hover:bg-blue-700 transition-colors">
              应用标准算例并生成网格
            </button>
          </div>

          <!-- 步骤 4: 运行求解 -->
          <div v-if="guideStep === 4" class="mt-4 space-y-3">
            <div class="bg-[var(--bg-surface)] rounded-lg p-4">
              <p class="text-xs text-[var(--text-muted)] mb-2">边界条件</p>
              <div class="flex gap-4">
                <div class="flex items-center gap-2">
                  <span class="w-3 h-3 bg-green-500 rounded-full"></span>
                  <span class="text-sm">固定端: x=0</span>
                </div>
                <div class="flex items-center gap-2">
                  <span class="w-3 h-3 bg-red-500 rounded-full"></span>
                  <span class="text-sm">载荷端: x=10</span>
                </div>
              </div>
            </div>
            <button @click="runSolver" :disabled="isRunning" class="w-full py-2.5 bg-blue-600 text-white rounded-lg text-sm hover:bg-blue-700 disabled:opacity-50 transition-colors flex items-center justify-center gap-2">
              <span v-if="isRunning">求解中... {{ solverProgress }}%</span>
              <span v-else>▶ 运行求解</span>
            </button>
          </div>

          <!-- 步骤 5: 查看结果 -->
          <div v-if="guideStep === 5 && currentResult" class="mt-4 space-y-3">
            <div class="bg-[var(--bg-surface)] rounded-lg p-4">
              <p class="text-sm text-[var(--text-primary)] mb-2">仿真完成！</p>
              <div class="grid grid-cols-2 gap-2 text-xs">
                <div class="bg-[var(--bg-elevated)] rounded p-2">
                  <p class="text-[var(--text-muted)]">最大位移</p>
                  <p class="font-medium">{{ currentResult.max_displacement?.toFixed(4) }} m</p>
                </div>
                <div class="bg-[var(--bg-elevated)] rounded p-2">
                  <p class="text-[var(--text-muted)]">最大应力</p>
                  <p class="font-medium">{{ currentResult.max_stress?.toFixed(0) }} MPa</p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 引导按钮 -->
        <div class="flex justify-between gap-3">
          <button v-if="guideStep > 1" @click="prevGuideStep" class="px-4 py-2 border rounded-lg text-sm hover:bg-[var(--bg-hover)] transition-colors">
            ← 上一步
          </button>
          <div v-else></div>
          <button v-if="guideStep < guideTotalSteps" @click="nextGuideStep" class="px-4 py-2 bg-[var(--primary)] text-white rounded-lg text-sm hover:opacity-90 transition-colors">
            下一步 →
          </button>
          <button v-else @click="closeGuide" class="px-4 py-2 bg-[var(--primary)] text-white rounded-lg text-sm hover:opacity-90 transition-colors">
            完成
          </button>
        </div>
      </div>
    </div>
  </div>
</template>