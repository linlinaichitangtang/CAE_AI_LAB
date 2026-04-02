<template>
  <div class="h-full flex flex-col bg-[var(--bg-primary)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-lg font-semibold text-[var(--text-primary)]">内聚力模型分析 (CZM)</h2>
        <p class="text-sm text-[var(--text-muted)]">分层/粘结界面：双线性内聚力模型，裂纹扩展可视化</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- Template Buttons -->
        <button
          v-for="tpl in quickTemplates"
          :key="tpl.id"
          @click="applyTemplate(tpl)"
          class="px-3 py-1.5 text-xs border border-[var(--border-color)] rounded bg-[var(--bg-tertiary)] text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] transition"
        >
          {{ tpl.name }}
        </button>
        <!-- Reset -->
        <button @click="resetAll" class="px-3 py-1.5 text-xs border border-gray-300 rounded hover:bg-gray-50 transition">
          重置
        </button>
        <!-- Export -->
        <button v-if="results" @click="exportResults" class="px-3 py-1.5 text-xs border border-green-300 text-green-600 rounded hover:bg-green-50 transition">
          导出结果
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration (w-80) -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] overflow-y-auto p-4 space-y-4">

        <!-- Step 1: Analysis Type -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">1</span>
            分析类型
          </h4>
          <div class="flex flex-col gap-2">
            <button
              v-for="type in analysisTypes"
              :key="type.value"
              @click="config.analysis_type = type.value"
              :class="['px-3 py-2 rounded text-xs text-left transition border',
                config.analysis_type === type.value
                  ? 'bg-blue-600 text-white border-blue-600'
                  : 'bg-[var(--bg-tertiary)] text-[var(--text-secondary)] border-[var(--border-color)] hover:bg-[var(--bg-hover)]']"
            >
              <div class="font-medium">{{ type.label }}</div>
              <div class="text-[10px] mt-0.5 opacity-80">{{ type.desc }}</div>
            </button>
          </div>
        </div>

        <!-- Step 2: Base Material -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">2</span>
            基体材料
          </h4>
          <div class="mb-3">
            <label class="label">材料选择</label>
            <select v-model="baseMaterialPreset" @change="applyBaseMaterial" class="input w-full text-xs">
              <option value="carbon_fiber">碳纤维复合材料</option>
              <option value="glass_fiber">玻璃纤维复合材料</option>
              <option value="aluminum">铝合金</option>
              <option value="custom">自定义</option>
            </select>
          </div>
          <div class="space-y-2">
            <div>
              <label class="label">弹性模量 E (MPa)</label>
              <input v-model.number="config.base_material.E" type="number" step="1000" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">泊松比 nu</label>
              <input v-model.number="config.base_material.nu" type="number" step="0.01" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">密度 (kg/m³)</label>
              <input v-model.number="config.base_material.density" type="number" step="10" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Step 3: Interface Properties -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">3</span>
            界面属性
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">法向刚度 Knn (MPa/mm)</label>
              <input v-model.number="config.cohesive_material.normal_stiffness" type="number" step="100" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">剪切刚度 Kss/Ktt (MPa/mm)</label>
              <input v-model.number="config.cohesive_material.shear_stiffness" type="number" step="100" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">法向强度 sigma_n (MPa)</label>
              <input v-model.number="config.cohesive_material.normal_strength" type="number" step="1" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">剪切强度 tau_s (MPa)</label>
              <input v-model.number="config.cohesive_material.shear_strength" type="number" step="1" class="input w-full text-xs" />
            </div>
            <div class="flex gap-2">
              <div class="flex-1">
                <label class="label">断裂能 Gn (GJ/m²)</label>
                <input v-model.number="config.cohesive_material.fracture_energy_gn" type="number" step="0.01" class="input w-full text-xs" />
              </div>
              <div class="flex-1">
                <label class="label">断裂能 Gs (GJ/m²)</label>
                <input v-model.number="config.cohesive_material.fracture_energy_gs" type="number" step="0.01" class="input w-full text-xs" />
              </div>
            </div>
            <div>
              <label class="label">罚刚度</label>
              <input v-model.number="config.cohesive_material.penalty_stiffness" type="number" step="100" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Step 4: Interface & Element Type -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">4</span>
            界面与单元
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">界面类型</label>
              <div class="flex gap-2">
                <button
                  @click="config.interface_type = 'surface-based'"
                  :class="['flex-1 px-2 py-1.5 rounded text-xs transition border',
                    config.interface_type === 'surface-based'
                      ? 'bg-blue-600 text-white border-blue-600'
                      : 'bg-[var(--bg-tertiary)] text-[var(--text-secondary)] border-[var(--border-color)]']"
                >
                  面基 (Surface)
                </button>
                <button
                  @click="config.interface_type = 'element-based'"
                  :class="['flex-1 px-2 py-1.5 rounded text-xs transition border',
                    config.interface_type === 'element-based'
                      ? 'bg-blue-600 text-white border-blue-600'
                      : 'bg-[var(--bg-tertiary)] text-[var(--text-secondary)] border-[var(--border-color)]']"
                >
                  单元基 (Element)
                </button>
              </div>
            </div>
            <div>
              <label class="label">单元类型</label>
              <select v-model="config.element_type" class="input w-full text-xs">
                <option value="COH3D8">COH3D8 (六面体)</option>
                <option value="COH3D6">COH3D6 (楔形体)</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Step 5: Solver Control -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">5</span>
            求解控制
          </h4>
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <label class="label mb-0">稳定化</label>
              <button
                @click="config.stabilization = !config.stabilization"
                :class="['relative w-10 h-5 rounded-full transition-colors',
                  config.stabilization ? 'bg-blue-600' : 'bg-gray-600']"
              >
                <span
                  :class="['absolute top-0.5 w-4 h-4 rounded-full bg-white transition-transform',
                    config.stabilization ? 'left-5' : 'left-0.5']"
                ></span>
              </button>
            </div>
            <div>
              <label class="label">粘性系数</label>
              <input v-model.number="config.viscosity" type="number" step="0.0001" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Run Analysis Button -->
        <button
          @click="runAnalysis"
          :disabled="analyzing"
          class="btn btn-primary text-xs w-full"
        >
          <span v-if="analyzing" class="mr-1 animate-spin">&#10227;</span>
          {{ analyzing ? '分析中...' : '运行内聚力分析' }}
        </button>
      </div>

      <!-- Right Panel: Visualization -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- Visualization Area -->
        <div class="flex-1 flex overflow-hidden">
          <!-- 3D Damage Field Canvas -->
          <div class="flex-1 relative">
            <div v-if="results" class="w-full h-full">
              <canvas ref="damageCanvas" class="w-full h-full"></canvas>
            </div>
            <div v-else class="w-full h-full flex items-center justify-center text-[var(--text-muted)]">
              <div class="text-center">
                <div class="text-4xl mb-2">&#128202;</div>
                <div class="text-sm">配置参数后运行内聚力分析</div>
              </div>
            </div>
          </div>

          <!-- Color Legend -->
          <div v-if="results" class="w-16 bg-[var(--bg-surface)] border-l border-[var(--border-subtle)] p-2">
            <div class="text-xs text-[var(--text-muted)] mb-1">损伤值</div>
            <div class="h-64 w-4 mx-auto rounded" style="background: linear-gradient(to bottom, #22c55e, #eab308, #ef4444, #7f1d1d)"></div>
            <div class="flex flex-col justify-between h-64 text-xs text-[var(--text-muted)] mt-1">
              <span>1.0</span>
              <span>0.75</span>
              <span>0.5</span>
              <span>0.25</span>
              <span>0.0</span>
            </div>
          </div>
        </div>

        <!-- Load-Displacement Curve (SVG) -->
        <div v-if="results" class="h-56 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] px-4 py-3">
          <h4 class="text-xs font-medium mb-2 text-[var(--text-secondary)]">载荷-位移曲线</h4>
          <svg ref="curveSvg" :viewBox="`0 0 ${svgWidth} ${svgHeight}`" class="w-full h-40">
            <!-- Grid lines -->
            <line v-for="i in 5" :key="'h'+i"
              :x1="svgPadding" :y1="svgPadding + (i - 1) * ((svgHeight - 2 * svgPadding) / 4)"
              :x2="svgWidth - svgPadding" :y2="svgPadding + (i - 1) * ((svgHeight - 2 * svgPadding) / 4)"
              stroke="#2D2E38" stroke-width="0.5"
            />
            <line v-for="i in 5" :key="'v'+i"
              :x1="svgPadding + (i - 1) * ((svgWidth - 2 * svgPadding) / 4)" :y1="svgPadding"
              :x2="svgPadding + (i - 1) * ((svgWidth - 2 * svgPadding) / 4)" :y2="svgHeight - svgPadding"
              stroke="#2D2E38" stroke-width="0.5"
            />
            <!-- Axes -->
            <line :x1="svgPadding" :y1="svgPadding" :x2="svgPadding" :y2="svgHeight - svgPadding" stroke="#6B7280" stroke-width="1" />
            <line :x1="svgPadding" :y1="svgHeight - svgPadding" :x2="svgWidth - svgPadding" :y2="svgHeight - svgPadding" stroke="#6B7280" stroke-width="1" />
            <!-- Curve -->
            <polyline
              :points="curvePoints"
              fill="none"
              stroke="#4F8CFF"
              stroke-width="2"
              stroke-linejoin="round"
            />
            <!-- Axis labels -->
            <text :x="svgWidth / 2" :y="svgHeight - 2" fill="#9CA3AF" font-size="10" text-anchor="middle">位移 (mm)</text>
            <text :x="8" :y="svgHeight / 2" fill="#9CA3AF" font-size="10" text-anchor="middle" transform="rotate(-90, 8, 140)">载荷 (N)</text>
            <!-- Peak marker -->
            <circle v-if="peakPoint" :cx="peakPoint.x" :cy="peakPoint.y" r="4" fill="#ef4444" />
            <text v-if="peakPoint" :x="peakPoint.x + 6" :y="peakPoint.y - 6" fill="#ef4444" font-size="9">峰值</text>
          </svg>
        </div>

        <!-- Results Statistics -->
        <div v-if="results" class="h-44 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] overflow-y-auto p-4">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">分析结果统计</h4>
          <div class="grid grid-cols-4 gap-4">
            <div class="result-card">
              <span class="result-label">最大分离量</span>
              <span class="result-value">{{ results.max_separation.toFixed(4) }} mm</span>
            </div>
            <div class="result-card">
              <span class="result-label">最大应力</span>
              <span class="result-value">{{ results.max_stress.toFixed(2) }} MPa</span>
            </div>
            <div class="result-card">
              <span class="result-label">界面能</span>
              <span class="result-value">{{ results.interface_energy.toFixed(2) }} mJ</span>
            </div>
            <div class="result-card">
              <span class="result-label">总能量</span>
              <span class="result-value">{{ results.total_energy.toFixed(2) }} mJ</span>
            </div>
            <div class="result-card">
              <span class="result-label">活跃界面</span>
              <span class="result-value text-green-500">{{ results.interface_status.active }}</span>
            </div>
            <div class="result-card">
              <span class="result-label">损伤界面</span>
              <span class="result-value text-yellow-500">{{ results.interface_status.damaged }}</span>
            </div>
            <div class="result-card">
              <span class="result-label">失效界面</span>
              <span class="result-value text-red-500">{{ results.interface_status.failed }}</span>
            </div>
            <div class="result-card">
              <span class="result-label">损伤比例</span>
              <span :class="['result-value', damageRatio > 0.5 ? 'text-red-500' : 'text-green-500']">
                {{ (damageRatio * 100).toFixed(1) }}%
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type {
  CohesiveConfig,
  CohesiveResult,
  CohesiveTemplate,
  CohesiveMaterial,
} from '../api/cohesive'

// ============ Constants ============

const analysisTypes = [
  { value: 'delamination' as const, label: '分层分析', desc: '复合材料层间分层' },
  { value: 'debonding' as const, label: '脱粘分析', desc: '胶接接头界面脱粘' },
  { value: 'crack_propagation' as const, label: '裂纹扩展', desc: '界面裂纹扩展模拟' },
]

const quickTemplates: CohesiveTemplate[] = [
  {
    id: 'composite_delamination',
    name: '复合材料层合板分层',
    name_en: 'Composite Laminate Delamination',
    category: 'delamination',
    description: '碳纤维/环氧树脂层合板 Mode I/II 分层分析',
    interface_type: 'surface-based',
    default_material: {
      name: 'CFRP/Epoxy Interface',
      normal_stiffness: 1e6,
      shear_stiffness: 1e6,
      normal_strength: 60,
      shear_strength: 90,
      fracture_energy_gn: 0.28,
      fracture_energy_gs: 0.79,
      penalty_stiffness: 1e8,
    },
    typical_application: '航空航天复合材料结构',
  },
  {
    id: 'adhesive_debonding',
    name: '胶接接头脱粘',
    name_en: 'Adhesive Joint Debonding',
    category: 'debonding',
    description: '结构胶接接头界面脱粘分析',
    interface_type: 'element-based',
    default_material: {
      name: 'Epoxy Adhesive',
      normal_stiffness: 5e5,
      shear_stiffness: 5e5,
      normal_strength: 30,
      shear_strength: 25,
      fracture_energy_gn: 0.5,
      fracture_energy_gs: 1.0,
      penalty_stiffness: 5e7,
    },
    typical_application: '汽车/船舶胶接结构',
  },
  {
    id: 'coating_delamination',
    name: '涂层剥离',
    name_en: 'Coating Delamination',
    category: 'crack_propagation',
    description: '涂层/基体界面裂纹扩展分析',
    interface_type: 'surface-based',
    default_material: {
      name: 'Coating Interface',
      normal_stiffness: 2e6,
      shear_stiffness: 2e6,
      normal_strength: 20,
      shear_strength: 15,
      fracture_energy_gn: 0.1,
      fracture_energy_gs: 0.2,
      penalty_stiffness: 2e8,
    },
    typical_application: '防腐涂层/热喷涂涂层',
  },
]

const baseMaterialPresets: Record<string, { E: number; nu: number; density: number }> = {
  carbon_fiber: { E: 138000, nu: 0.3, density: 1600 },
  glass_fiber: { E: 40000, nu: 0.25, density: 2000 },
  aluminum: { E: 71000, nu: 0.33, density: 2700 },
}

// ============ State ============

const analyzing = ref(false)
const baseMaterialPreset = ref('carbon_fiber')
const results = ref<CohesiveResult | null>(null)
const damageCanvas = ref<HTMLCanvasElement | null>(null)
const curveSvg = ref<SVGSVGElement | null>(null)

const svgWidth = 600
const svgHeight = 180
const svgPadding = 30

const config = reactive<CohesiveConfig>({
  project_id: '',
  analysis_type: 'delamination',
  cohesive_material: {
    name: 'CFRP/Epoxy Interface',
    normal_stiffness: 1e6,
    shear_stiffness: 1e6,
    normal_strength: 60,
    shear_strength: 90,
    fracture_energy_gn: 0.28,
    fracture_energy_gs: 0.79,
    penalty_stiffness: 1e8,
  },
  base_material: {
    name: '碳纤维复合材料',
    E: 138000,
    nu: 0.3,
    density: 1600,
  },
  interface_type: 'surface-based',
  element_type: 'COH3D8',
  stabilization: true,
  viscosity: 0.0005,
})

// ============ Computed ============

const damageRatio = computed(() => {
  if (!results.value) return 0
  const total = results.value.interface_status.active
    + results.value.interface_status.damaged
    + results.value.interface_status.failed
  if (total === 0) return 0
  return (results.value.interface_status.damaged + results.value.interface_status.failed) / total
})

const curvePoints = computed(() => {
  if (!results.value) return ''
  const curve = results.value.load_displacement_curve
  if (!curve || curve.length === 0) return ''

  const maxDisp = Math.max(...curve.map(p => p.displacement), 1e-10)
  const maxLoad = Math.max(...curve.map(p => p.load), 1e-10)
  const plotW = svgWidth - 2 * svgPadding
  const plotH = svgHeight - 2 * svgPadding

  return curve
    .map(p => {
      const x = svgPadding + (p.displacement / maxDisp) * plotW
      const y = (svgHeight - svgPadding) - (p.load / maxLoad) * plotH
      return `${x.toFixed(1)},${y.toFixed(1)}`
    })
    .join(' ')
})

const peakPoint = computed(() => {
  if (!results.value) return null
  const curve = results.value.load_displacement_curve
  if (!curve || curve.length === 0) return null

  const maxDisp = Math.max(...curve.map(p => p.displacement), 1e-10)
  const maxLoad = Math.max(...curve.map(p => p.load), 1e-10)
  const plotW = svgWidth - 2 * svgPadding
  const plotH = svgHeight - 2 * svgPadding

  let peakIdx = 0
  for (let i = 1; i < curve.length; i++) {
    if (curve[i].load > curve[peakIdx].load) peakIdx = i
  }

  const p = curve[peakIdx]
  return {
    x: svgPadding + (p.displacement / maxDisp) * plotW,
    y: (svgHeight - svgPadding) - (p.load / maxLoad) * plotH,
  }
})

// ============ Methods ============

function applyBaseMaterial() {
  const preset = baseMaterialPresets[baseMaterialPreset.value]
  if (preset) {
    config.base_material.E = preset.E
    config.base_material.nu = preset.nu
    config.base_material.density = preset.density
    const nameMap: Record<string, string> = {
      carbon_fiber: '碳纤维复合材料',
      glass_fiber: '玻璃纤维复合材料',
      aluminum: '铝合金',
    }
    config.base_material.name = nameMap[baseMaterialPreset.value] || '自定义'
  }
}

function applyTemplate(tpl: CohesiveTemplate) {
  config.analysis_type = tpl.category as CohesiveConfig['analysis_type']
  config.interface_type = tpl.interface_type
  config.cohesive_material = { ...tpl.default_material }
}

function resetAll() {
  results.value = null
  config.analysis_type = 'delamination'
  config.interface_type = 'surface-based'
  config.element_type = 'COH3D8'
  config.stabilization = true
  config.viscosity = 0.0005
  config.cohesive_material = {
    name: 'CFRP/Epoxy Interface',
    normal_stiffness: 1e6,
    shear_stiffness: 1e6,
    normal_strength: 60,
    shear_strength: 90,
    fracture_energy_gn: 0.28,
    fracture_energy_gs: 0.79,
    penalty_stiffness: 1e8,
  }
  baseMaterialPreset.value = 'carbon_fiber'
  applyBaseMaterial()
}

function generateMockResults(): CohesiveResult {
  // Generate bilinear traction-separation curve
  const delta0 = config.cohesive_material.normal_strength / config.cohesive_material.normal_stiffness
  const deltaF = 2 * config.cohesive_material.fracture_energy_gn / config.cohesive_material.normal_strength
  const peakLoad = config.cohesive_material.normal_strength * 10 // Scale for display

  const curvePoints: Array<{ displacement: number; load: number }> = []
  const numPoints = 50
  for (let i = 0; i <= numPoints; i++) {
    const t = i / numPoints
    const delta = t * deltaF * 1.5
    let load: number
    if (delta <= delta0) {
      // Linear ascending
      load = (delta / delta0) * peakLoad
    } else if (delta <= deltaF) {
      // Linear descending (softening)
      load = peakLoad * (1 - (delta - delta0) / (deltaF - delta0))
    } else {
      // Fully separated
      load = 0
    }
    // Add small noise for realism
    load += (Math.random() - 0.5) * peakLoad * 0.02
    load = Math.max(0, load)
    curvePoints.push({ displacement: delta, load })
  }

  // Generate damage field on a grid
  const damageField: Array<{ x: number; y: number; z: number; damage: number }> = []
  const gridSize = 10
  for (let ix = 0; ix < gridSize; ix++) {
    for (let iy = 0; iy < gridSize; iy++) {
      const x = ix * 2.0
      const y = iy * 2.0
      const z = 0
      // Damage concentrates near center with random variation
      const distFromCenter = Math.sqrt((ix - gridSize / 2) ** 2 + (iy - gridSize / 2) ** 2) / (gridSize / 2)
      const damage = Math.min(1, Math.max(0, 1 - distFromCenter + (Math.random() - 0.5) * 0.3))
      damageField.push({ x, y, z, damage })
    }
  }

  const totalInterfaces = gridSize * gridSize
  const failedCount = Math.floor(totalInterfaces * 0.15)
  const damagedCount = Math.floor(totalInterfaces * 0.35)
  const activeCount = totalInterfaces - failedCount - damagedCount

  return {
    success: true,
    max_separation: deltaF * 1.2,
    max_stress: config.cohesive_material.normal_strength * (0.9 + Math.random() * 0.1),
    interface_status: {
      damaged: damagedCount,
      failed: failedCount,
      active: activeCount,
    },
    load_displacement_curve: curvePoints,
    damage_field: damageField,
    interface_energy: config.cohesive_material.fracture_energy_gn * 1000 * (0.8 + Math.random() * 0.2),
    total_energy: config.cohesive_material.fracture_energy_gn * 1000 * (1.2 + Math.random() * 0.3),
  }
}

function drawDamageField() {
  if (!damageCanvas.value || !results.value) return

  const canvas = damageCanvas.value
  const rect = canvas.parentElement?.getBoundingClientRect()
  if (!rect) return

  canvas.width = rect.width * window.devicePixelRatio
  canvas.height = rect.height * window.devicePixelRatio
  canvas.style.width = rect.width + 'px'
  canvas.style.height = rect.height + 'px'

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const scale = window.devicePixelRatio
  ctx.scale(scale, scale)

  const w = rect.width
  const h = rect.height
  const padding = 40

  ctx.clearRect(0, 0, w, h)

  // Background
  ctx.fillStyle = '#0A0B0E'
  ctx.fillRect(0, 0, w, h)

  const field = results.value.damage_field
  if (!field || field.length === 0) return

  // Find bounds
  const xs = field.map(p => p.x)
  const ys = field.map(p => p.y)
  const minX = Math.min(...xs)
  const maxX = Math.max(...xs)
  const minY = Math.min(...ys)
  const maxY = Math.max(...ys)

  const plotW = w - 2 * padding
  const plotH = h - 2 * padding

  // Draw damage field as colored rectangles
  const cellW = plotW / Math.sqrt(field.length)
  const cellH = plotH / Math.sqrt(field.length)
  const gridSize = Math.sqrt(field.length)

  for (let i = 0; i < field.length; i++) {
    const p = field[i]
    const col = i % gridSize
    const row = Math.floor(i / gridSize)

    const cx = padding + col * cellW
    const cy = padding + row * cellH

    // Color map: green -> yellow -> red -> dark red
    const d = p.damage
    let r: number, g: number, b: number
    if (d < 0.5) {
      const t = d * 2
      r = Math.round(34 + t * (234 - 34))
      g = Math.round(197 + t * (179 - 197))
      b = Math.round(94 + t * (8 - 94))
    } else {
      const t = (d - 0.5) * 2
      r = Math.round(234 + t * (127 - 234))
      g = Math.round(179 + t * (29 - 179))
      b = Math.round(8 + t * (29 - 8))
    }

    ctx.fillStyle = `rgb(${r},${g},${b})`
    ctx.fillRect(cx + 1, cy + 1, cellW - 2, cellH - 2)

    // Draw damage value text
    if (cellW > 30) {
      ctx.fillStyle = d > 0.5 ? '#fff' : '#000'
      ctx.font = '9px monospace'
      ctx.textAlign = 'center'
      ctx.textBaseline = 'middle'
      ctx.fillText(d.toFixed(2), cx + cellW / 2, cy + cellH / 2)
    }
  }

  // Draw axes
  ctx.strokeStyle = '#6B7280'
  ctx.lineWidth = 1
  ctx.beginPath()
  ctx.moveTo(padding, padding)
  ctx.lineTo(padding, h - padding)
  ctx.lineTo(w - padding, h - padding)
  ctx.stroke()

  // Axis labels
  ctx.fillStyle = '#9CA3AF'
  ctx.font = '11px sans-serif'
  ctx.textAlign = 'center'
  ctx.fillText('X (mm)', w / 2, h - 8)
  ctx.save()
  ctx.translate(12, h / 2)
  ctx.rotate(-Math.PI / 2)
  ctx.fillText('Y (mm)', 0, 0)
  ctx.restore()

  // Title
  ctx.fillStyle = '#E8E9EB'
  ctx.font = '12px sans-serif'
  ctx.textAlign = 'center'
  ctx.fillText('界面损伤云图 (SDEG)', w / 2, 20)
}

async function runAnalysis() {
  analyzing.value = true
  try {
    // Generate mock results (in production, call API)
    await new Promise(resolve => setTimeout(resolve, 1500))
    results.value = generateMockResults()

    await nextTick()
    drawDamageField()
  } catch (e) {
    console.error('Cohesive analysis failed:', e)
  } finally {
    analyzing.value = false
  }
}

function exportResults() {
  if (!results.value) return
  const data = JSON.stringify(results.value, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'cohesive_results.json'
  a.click()
  URL.revokeObjectURL(url)
}

// ============ Lifecycle ============

onMounted(() => {
  applyBaseMaterial()
})
</script>

<style scoped>
.panel-section {
  margin-bottom: 4px;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
}

.label {
  display: block;
  font-size: 10px;
  color: var(--text-muted);
  margin-bottom: 4px;
  text-transform: uppercase;
}

.input {
  width: 100%;
  padding: 8px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  color: var(--text-primary);
  font-size: 12px;
}

.result-card {
  display: flex;
  flex-direction: column;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
}

.result-label {
  font-size: 10px;
  color: var(--text-muted);
  text-transform: uppercase;
  margin-bottom: 4px;
}

.result-value {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}
</style>
