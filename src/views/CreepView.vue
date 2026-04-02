<template>
  <div class="h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-lg font-semibold text-[var(--text-primary)]">蠕变分析</h2>
        <p class="text-sm text-[var(--text-muted)]">Norton/Bailey-Norton 蠕变模型，时间硬化/应变硬化，剩余寿命评估</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- 模板按钮 -->
        <button
          v-for="tpl in quickTemplates"
          :key="tpl.id"
          @click="applyTemplate(tpl)"
          class="btn btn-ghost text-xs"
        >
          {{ tpl.name }}
        </button>
        <!-- 重置 -->
        <button @click="resetAll" class="btn btn-ghost text-xs">重置</button>
        <!-- 导出 -->
        <button v-if="results" @click="exportResults" class="btn btn-ghost text-xs" style="color: var(--accent-green); border-color: var(--accent-green);">
          导出结果
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration (w-80) -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] overflow-y-auto p-4 space-y-4">

        <!-- Step 1: 蠕变模型选择 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">1</span>
            蠕变模型选择
          </h4>
          <div class="flex flex-col gap-1.5">
            <button
              v-for="model in creepModels"
              :key="model.value"
              @click="config.model = model.value"
              :class="['px-3 py-2 rounded text-xs text-left transition border',
                config.model === model.value
                  ? 'bg-blue-600 text-white border-blue-600'
                  : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)] border-[var(--border-default)] hover:bg-[var(--bg-hover)]']"
            >
              <div class="font-medium">{{ model.label }}</div>
              <div class="text-[10px] mt-0.5 opacity-80">{{ model.desc }}</div>
            </button>
          </div>
        </div>

        <!-- Step 2: 材料属性 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">2</span>
            材料属性
          </h4>
          <div class="mb-3">
            <label class="label">材料库</label>
            <select v-model="materialPreset" @change="applyMaterialPreset" class="input w-full text-xs">
              <option value="ss304">304不锈钢</option>
              <option value="inconel718">Inconel 718</option>
              <option value="p91">P91钢</option>
              <option value="al6061">Al 6061</option>
              <option value="custom">自定义</option>
            </select>
          </div>
          <div class="space-y-2">
            <div>
              <label class="label">弹性模量 E (Pa)</label>
              <input v-model.number="config.material.E" type="number" step="1e9" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">泊松比 nu</label>
              <input v-model.number="config.material.nu" type="number" step="0.01" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">Norton系数 A</label>
              <input v-model.number="config.material.A" type="number" step="1e-20" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">应力指数 n</label>
              <input v-model.number="config.material.n" type="number" step="0.1" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">激活能 Q (J/mol)</label>
              <input v-model.number="config.material.Q" type="number" step="1000" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">参考温度 (K)</label>
              <input v-model.number="config.material.temperature_reference" type="number" step="10" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Step 3: 载荷与温度 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">3</span>
            载荷与温度
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">施加应力 (Pa)</label>
              <input v-model.number="config.applied_stress" type="number" step="1e6" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">温度 (K)</label>
              <input v-model.number="config.temperature" type="number" step="10" class="input w-full text-xs" />
            </div>
            <div class="grid grid-cols-3 gap-2">
              <div>
                <label class="label">时间最小 (h)</label>
                <input v-model.number="config.time_range.min" type="number" step="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">时间最大 (h)</label>
                <input v-model.number="config.time_range.max" type="number" step="100" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">步长 (h)</label>
                <input v-model.number="config.time_range.step" type="number" step="1" class="input w-full text-xs" />
              </div>
            </div>
          </div>
        </div>

        <!-- Step 4: 硬化类型 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">4</span>
            硬化类型
          </h4>
          <div class="flex flex-col gap-1.5">
            <button
              v-for="ht in hardeningTypes"
              :key="ht.value"
              @click="config.hardening_type = ht.value"
              :class="['px-3 py-2 rounded text-xs text-left transition border',
                config.hardening_type === ht.value
                  ? 'bg-blue-600 text-white border-blue-600'
                  : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)] border-[var(--border-default)] hover:bg-[var(--bg-hover)]']"
            >
              {{ ht.label }}
            </button>
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="space-y-2">
          <button @click="runAnalysis" :disabled="analyzing" class="btn btn-primary text-xs w-full">
            {{ analyzing ? '分析中...' : '运行蠕变分析' }}
          </button>
          <button @click="runLifePrediction" :disabled="!results" class="btn btn-ghost text-xs w-full" style="border: 1px solid var(--accent-yellow); color: var(--accent-yellow);">
            剩余寿命预测
          </button>
        </div>
      </div>

      <!-- Right Panel: Results Visualization -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- Results Tabs -->
        <div v-if="results" class="flex items-center gap-2 px-4 py-2 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
          <button
            v-for="tab in resultTabs"
            :key="tab.value"
            @click="resultViewTab = tab.value"
            :class="['px-3 py-1 text-xs rounded transition',
              resultViewTab === tab.value
                ? 'bg-[var(--primary)] text-white'
                : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)]']"
          >
            {{ tab.label }}
          </button>
        </div>

        <!-- Visualization Area -->
        <div class="flex-1 overflow-y-auto p-4">
          <div v-if="results">
            <!-- 蠕变曲线图 -->
            <div v-show="resultViewTab === 'creep_curve'" class="mb-4">
              <h4 class="text-sm font-medium mb-2 text-[var(--text-primary)]">蠕变应变曲线 (三阶段)</h4>
              <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-4">
                <svg ref="creepCurveSvg" viewBox="0 0 600 300" class="w-full" style="max-height: 300px;">
                  <!-- Grid -->
                  <line v-for="i in 6" :key="'h'+i" :x1="60" :y1="30 + i * 45" :x2="580" :y2="30 + i * 45" stroke="var(--border-subtle)" stroke-width="0.5" />
                  <line v-for="i in 6" :key="'v'+i" :x1="60 + i * 86.6" :y1="30" :x2="60 + i * 86.6" :y2="300" stroke="var(--border-subtle)" stroke-width="0.5" />
                  <!-- Axes -->
                  <line x1="60" y1="300" x2="580" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                  <line x1="60" y1="30" x2="60" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                  <!-- Axis Labels -->
                  <text x="320" y="295" text-anchor="middle" fill="var(--text-muted)" font-size="10">时间 (h)</text>
                  <text x="20" y="165" text-anchor="middle" fill="var(--text-muted)" font-size="10" transform="rotate(-90, 20, 165)">蠕变应变 (%)</text>
                  <!-- Tick labels X -->
                  <text v-for="(tick, i) in creepXTicks" :key="'tx'+i" :x="tick.x" :y="315" text-anchor="middle" fill="var(--text-muted)" font-size="9">{{ tick.label }}</text>
                  <!-- Tick labels Y -->
                  <text v-for="(tick, i) in creepYTicks" :key="'ty'+i" :x="55" :y="tick.y + 3" text-anchor="end" fill="var(--text-muted)" font-size="9">{{ tick.label }}</text>
                  <!-- Phase annotations -->
                  <rect x="60" y="30" :width="phase1Width" height="270" fill="rgba(37,99,235,0.04)" />
                  <rect :x="60 + phase1Width" y="30" :width="phase2Width" height="270" fill="rgba(34,197,94,0.04)" />
                  <rect :x="60 + phase1Width + phase2Width" y="30" :width="phase3Width" height="270" fill="rgba(239,68,68,0.04)" />
                  <text :x="60 + phase1Width / 2" y="48" text-anchor="middle" fill="var(--primary)" font-size="9" font-weight="500">第一阶段</text>
                  <text :x="60 + phase1Width + phase2Width / 2" y="48" text-anchor="middle" fill="var(--accent-green)" font-size="9" font-weight="500">第二阶段</text>
                  <text :x="60 + phase1Width + phase2Width + phase3Width / 2" y="48" text-anchor="middle" fill="var(--accent-red)" font-size="9" font-weight="500">第三阶段</text>
                  <!-- Creep curve path -->
                  <polyline
                    :points="creepCurvePoints"
                    fill="none"
                    stroke="var(--primary)"
                    stroke-width="2"
                    stroke-linejoin="round"
                  />
                  <!-- Data points -->
                  <circle
                    v-for="(pt, i) in creepCurveSamplePoints"
                    :key="'cp'+i"
                    :cx="pt.x"
                    :cy="pt.y"
                    r="2.5"
                    fill="var(--primary)"
                  />
                </svg>
              </div>
            </div>

            <!-- 应变速率曲线 -->
            <div v-show="resultViewTab === 'strain_rate'" class="mb-4">
              <h4 class="text-sm font-medium mb-2 text-[var(--text-primary)]">应变速率曲线</h4>
              <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-4">
                <svg viewBox="0 0 600 300" class="w-full" style="max-height: 300px;">
                  <!-- Grid -->
                  <line v-for="i in 6" :key="'rh'+i" :x1="60" :y1="30 + i * 45" :x2="580" :y2="30 + i * 45" stroke="var(--border-subtle)" stroke-width="0.5" />
                  <line v-for="i in 6" :key="'rv'+i" :x1="60 + i * 86.6" :y1="30" :x2="60 + i * 86.6" :y2="300" stroke="var(--border-subtle)" stroke-width="0.5" />
                  <!-- Axes -->
                  <line x1="60" y1="300" x2="580" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                  <line x1="60" y1="30" x2="60" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                  <!-- Axis Labels -->
                  <text x="320" y="295" text-anchor="middle" fill="var(--text-muted)" font-size="10">时间 (h)</text>
                  <text x="20" y="165" text-anchor="middle" fill="var(--text-muted)" font-size="10" transform="rotate(-90, 20, 165)">应变速率 (1/s)</text>
                  <!-- Strain rate curve -->
                  <polyline
                    :points="strainRatePoints"
                    fill="none"
                    stroke="var(--accent-green)"
                    stroke-width="2"
                    stroke-linejoin="round"
                  />
                  <!-- Steady state line -->
                  <line
                    :x1="60"
                    :y1="steadyStateLineY"
                    x2="580"
                    :y2="steadyStateLineY"
                    stroke="var(--accent-yellow)"
                    stroke-width="1"
                    stroke-dasharray="6,3"
                  />
                  <text x="585" :y="steadyStateLineY + 3" fill="var(--accent-yellow)" font-size="9">稳态</text>
                </svg>
              </div>
            </div>

            <!-- Larson-Miller参数图 -->
            <div v-show="resultViewTab === 'lmp'" class="mb-4">
              <h4 class="text-sm font-medium mb-2 text-[var(--text-primary)]">Larson-Miller 参数图 (应力 vs LMP)</h4>
              <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-4">
                <svg viewBox="0 0 600 300" class="w-full" style="max-height: 300px;">
                  <!-- Grid -->
                  <line v-for="i in 6" :key="'lh'+i" :x1="60" :y1="30 + i * 45" :x2="580" :y2="30 + i * 45" stroke="var(--border-subtle)" stroke-width="0.5" />
                  <line v-for="i in 6" :key="'lv'+i" :x1="60 + i * 86.6" :y1="30" :x2="60 + i * 86.6" :y2="300" stroke="var(--border-subtle)" stroke-width="0.5" />
                  <!-- Axes -->
                  <line x1="60" y1="300" x2="580" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                  <line x1="60" y1="30" x2="60" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                  <!-- Axis Labels -->
                  <text x="320" y="295" text-anchor="middle" fill="var(--text-muted)" font-size="10">LMP = T(C + log t)</text>
                  <text x="20" y="165" text-anchor="middle" fill="var(--text-muted)" font-size="10" transform="rotate(-90, 20, 165)">应力 (MPa)</text>
                  <!-- LMP data points -->
                  <polyline
                    :points="lmpCurvePoints"
                    fill="none"
                    stroke="var(--accent-red)"
                    stroke-width="2"
                    stroke-linejoin="round"
                  />
                  <circle
                    v-for="(pt, i) in lmpCurvePoints.split(' ')"
                    :key="'lmp'+i"
                    :cx="parseFloat(pt.split(',')[0])"
                    :cy="parseFloat(pt.split(',')[1])"
                    r="3"
                    fill="var(--accent-red)"
                  />
                </svg>
              </div>
            </div>

            <!-- 结果统计 -->
            <div v-show="resultViewTab === 'stats'" class="mb-4">
              <h4 class="text-sm font-medium mb-2 text-[var(--text-primary)]">结果统计</h4>
              <div class="grid grid-cols-2 lg:grid-cols-4 gap-3">
                <div class="stat-card">
                  <span class="stat-label">稳态蠕变速率</span>
                  <span class="stat-value">{{ formatScientific(results.steady_state_rate) }} /s</span>
                </div>
                <div class="stat-card">
                  <span class="stat-label">断裂时间</span>
                  <span class="stat-value">{{ formatTime(results.rupture_time) }}</span>
                </div>
                <div class="stat-card">
                  <span class="stat-label">最小蠕变速率</span>
                  <span class="stat-value">{{ formatScientific(results.min_creep_rate) }} /s</span>
                </div>
                <div class="stat-card">
                  <span class="stat-label">Larson-Miller参数</span>
                  <span class="stat-value">{{ results.larson_miller_parameter.toFixed(1) }}</span>
                </div>
                <div class="stat-card">
                  <span class="stat-label">1%应变时间</span>
                  <span class="stat-value">{{ formatTime(results.time_to_1pct_strain) }}</span>
                </div>
                <div class="stat-card">
                  <span class="stat-label">5%应变时间</span>
                  <span class="stat-value">{{ formatTime(results.time_to_5pct_strain) }}</span>
                </div>
                <div class="stat-card">
                  <span class="stat-label">剩余寿命分数</span>
                  <span :class="['stat-value', results.remaining_life_fraction < 0.3 ? 'text-red-500' : 'text-green-500']">
                    {{ (results.remaining_life_fraction * 100).toFixed(1) }}%
                  </span>
                </div>
                <div class="stat-card">
                  <span class="stat-label">当前蠕变模型</span>
                  <span class="stat-value text-sm">{{ modelLabel }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Empty State -->
          <div v-else class="w-full h-full flex items-center justify-center text-[var(--text-muted)]">
            <div class="text-center">
              <div class="text-5xl mb-3 opacity-30" style="font-family: var(--font-code);">&#x03B5;</div>
              <div class="text-sm">配置参数后运行蠕变分析</div>
              <div class="text-xs mt-1 opacity-60">支持 Norton 幂律、Bailey-Norton、时间/应变硬化等模型</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import type { CreepModel, CreepResult, CreepCurvePoint } from '../api/creep'

// ============ 常量 ============

const creepModels = [
  { value: 'norton_power' as CreepModel, label: 'Norton 幂律', desc: 'epsilon = A * sigma^n * t^m' },
  { value: 'bailey_norton' as CreepModel, label: 'Bailey-Norton', desc: 'epsilon = A * sigma^n * t^m (改进)' },
  { value: 'time_hardening' as CreepModel, label: '时间硬化', desc: 'd_epsilon/dt = A * sigma^n * t^m' },
  { value: 'strain_hardening' as CreepModel, label: '应变硬化', desc: 'd_epsilon/dt = A * sigma^n * epsilon^m' },
  { value: 'andrade' as CreepModel, label: 'Andrade', desc: 'epsilon = beta * t^(1/3) + kappa * t' },
  { value: 'garofalo' as CreepModel, label: 'Garofalo', desc: 'epsilon_dot = A * [sinh(alpha*sigma)]^n * exp(-Q/RT)' },
]

const hardeningTypes = [
  { value: 'time' as const, label: '时间硬化' },
  { value: 'strain' as const, label: '应变硬化' },
  { value: 'both' as const, label: '两者' },
]

const quickTemplates = [
  { id: 'pipe', name: '高温管道', model: 'norton_power' as CreepModel, stress: 80e6, temp: 873, preset: 'p91' },
  { id: 'blade', name: '涡轮叶片', model: 'garofalo' as CreepModel, stress: 200e6, temp: 1173, preset: 'inconel718' },
  { id: 'vessel', name: '压力容器', model: 'time_hardening' as CreepModel, stress: 150e6, temp: 773, preset: 'ss304' },
  { id: 'weld', name: '焊接接头', model: 'bailey_norton' as CreepModel, stress: 100e6, temp: 823, preset: 'p91' },
]

const materialPresets: Record<string, { E: number; nu: number; A: number; n: number; Q: number; Tref: number }> = {
  ss304: { E: 193e9, nu: 0.29, A: 3.5e-17, n: 5.2, Q: 280000, Tref: 873 },
  inconel718: { E: 200e9, nu: 0.3, A: 1.2e-30, n: 7.5, Q: 420000, Tref: 973 },
  p91: { E: 213e9, nu: 0.3, A: 5.8e-22, n: 6.8, Q: 350000, Tref: 873 },
  al6061: { E: 68.9e9, nu: 0.33, A: 1.1e-12, n: 3.2, Q: 150000, Tref: 573 },
}

const resultTabs = [
  { value: 'creep_curve', label: '蠕变曲线' },
  { value: 'strain_rate', label: '应变速率' },
  { value: 'lmp', label: 'L-M参数' },
  { value: 'stats', label: '结果统计' },
]

// ============ 响应式状态 ============

const analyzing = ref(false)
const materialPreset = ref('ss304')
const resultViewTab = ref('creep_curve')
const results = ref<CreepResult | null>(null)

const config = reactive({
  model: 'norton_power' as CreepModel,
  material: {
    E: 193e9,
    nu: 0.29,
    density: 8000,
    A: 3.5e-17,
    n: 5.2,
    Q: 280000,
    R: 8.314,
    temperature_reference: 873,
    creep_strain_coefficients: {},
  },
  applied_stress: 80e6,
  temperature: 873,
  time_range: { min: 0, max: 1000, step: 5 },
  hardening_type: 'time' as 'time' | 'strain' | 'both',
})

// ============ 计算属性 ============

const modelLabel = computed(() => {
  const found = creepModels.find(m => m.value === config.model)
  return found ? found.label : config.model
})

// 生成模拟蠕变曲线数据 (基于 Norton 幂律: epsilon = A * sigma^n * t^m)
function generateCreepData(): CreepCurvePoint[] {
  const { A, n } = config.material
  const sigma = config.applied_stress
  const T = config.temperature
  const Tref = config.material.temperature_reference
  const Q = config.material.Q
  const R = config.material.R
  const tMin = config.time_range.min
  const tMax = config.time_range.max
  const tStep = config.time_range.step

  const points: CreepCurvePoint[] = []
  const tRupture = tMax * 0.95

  // Arrhenius温度修正
  const tempFactor = Math.exp(-Q / R * (1 / T - 1 / Tref))

  for (let t = tMin; t <= tMax; t += tStep) {
    const tSec = t * 3600
    const tRuptSec = tRupture * 3600

    // Norton幂律: epsilon = A * sigma^n * t^m
    const baseRate = A * Math.pow(sigma, n) * tempFactor

    // 三阶段蠕变: m随时间变化
    let m: number
    const ratio = tSec / tRuptSec
    if (ratio < 0.1) {
      // 第一阶段: 减速蠕变 (m从0.5递减到0.1)
      m = 0.5 - 4.0 * ratio
    } else if (ratio < 0.8) {
      // 第二阶段: 稳态蠕变 (m ~ 1)
      m = 1.0
    } else {
      // 第三阶段: 加速蠕变 (m从1递增到4)
      m = 1.0 + 15.0 * Math.pow((ratio - 0.8) / 0.2, 2)
    }

    const strain = baseRate * Math.pow(Math.max(tSec, 1), m)
    const strainRate = m > 0 ? baseRate * m * Math.pow(Math.max(tSec, 1), m - 1) : 0

    points.push({
      time: t,
      strain: strain * 100, // 转换为百分比
      strain_rate: strainRate,
    })
  }
  return points
}

// 蠕变曲线 SVG 坐标点
const creepCurvePoints = computed(() => {
  if (!results.value) return ''
  const data = results.value.creep_strain_curve
  if (!data || data.length === 0) return ''

  const maxTime = Math.max(...data.map(d => d.time))
  const maxStrain = Math.max(...data.map(d => d.strain))
  const plotW = 520
  const plotH = 270
  const offsetX = 60
  const offsetY = 30

  return data.map(d => {
    const x = offsetX + (d.time / maxTime) * plotW
    const y = offsetY + plotH - (d.strain / maxStrain) * plotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

// 采样点 (每隔N个取一个)
const creepCurveSamplePoints = computed(() => {
  if (!results.value) return []
  const data = results.value.creep_strain_curve
  if (!data || data.length === 0) return []

  const maxTime = Math.max(...data.map(d => d.time))
  const maxStrain = Math.max(...data.map(d => d.strain))
  const plotW = 520
  const plotH = 270
  const offsetX = 60
  const offsetY = 30
  const step = Math.max(1, Math.floor(data.length / 20))

  return data.filter((_, i) => i % step === 0).map(d => ({
    x: offsetX + (d.time / maxTime) * plotW,
    y: offsetY + plotH - (d.strain / maxStrain) * plotH,
  }))
})

// X轴刻度
const creepXTicks = computed(() => {
  if (!results.value) return []
  const data = results.value.creep_strain_curve
  const maxTime = data.length > 0 ? Math.max(...data.map(d => d.time)) : 1000
  const plotW = 520
  const offsetX = 60
  const ticks = 5
  return Array.from({ length: ticks + 1 }, (_, i) => ({
    x: offsetX + (i / ticks) * plotW,
    label: ((i / ticks) * maxTime).toFixed(0),
  }))
})

// Y轴刻度
const creepYTicks = computed(() => {
  if (!results.value) return []
  const data = results.value.creep_strain_curve
  const maxStrain = data.length > 0 ? Math.max(...data.map(d => d.strain)) : 10
  const plotH = 270
  const offsetY = 30
  const ticks = 5
  return Array.from({ length: ticks + 1 }, (_, i) => ({
    y: offsetY + plotH - (i / ticks) * plotH,
    label: ((i / ticks) * maxStrain).toFixed(2),
  }))
})

// 三阶段宽度
const phase1Width = computed(() => 520 * 0.1)
const phase2Width = computed(() => 520 * 0.7)
const phase3Width = computed(() => 520 * 0.2)

// 应变速率曲线 SVG 坐标点
const strainRatePoints = computed(() => {
  if (!results.value) return ''
  const data = results.value.creep_strain_curve
  if (!data || data.length === 0) return ''

  const maxTime = Math.max(...data.map(d => d.time))
  const maxRate = Math.max(...data.map(d => d.strain_rate))
  const plotW = 520
  const plotH = 270
  const offsetX = 60
  const offsetY = 30

  return data.map(d => {
    const x = offsetX + (d.time / maxTime) * plotW
    const y = offsetY + plotH - (d.strain_rate / maxRate) * plotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

// 稳态线Y坐标
const steadyStateLineY = computed(() => {
  if (!results.value) return 165
  const data = results.value.creep_strain_curve
  const maxRate = Math.max(...data.map(d => d.strain_rate))
  const plotH = 270
  const offsetY = 30
  // 稳态速率约为最小速率的1.5倍
  const steadyRate = results.value.min_creep_rate * 1.5
  return offsetY + plotH - (steadyRate / maxRate) * plotH
})

// Larson-Miller 参数图数据
const lmpCurvePoints = computed(() => {
  // 模拟 Larson-Miller 参数图: 应力 vs LMP
  const stresses = [300, 250, 200, 150, 120, 100, 80, 60, 50, 40]
  const lmpValues = [18000, 19000, 20000, 21000, 22000, 23000, 24000, 25000, 26000, 27000]
  const plotW = 520
  const plotH = 270
  const offsetX = 60
  const offsetY = 30
  const minLMP = 17000
  const maxLMP = 28000
  const minStress = 30
  const maxStress = 320

  return stresses.map((s, i) => {
    const x = offsetX + ((lmpValues[i] - minLMP) / (maxLMP - minLMP)) * plotW
    const y = offsetY + plotH - ((s - minStress) / (maxStress - minStress)) * plotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

// ============ 方法 ============

function applyMaterialPreset() {
  const preset = materialPresets[materialPreset.value]
  if (preset) {
    config.material.E = preset.E
    config.material.nu = preset.nu
    config.material.A = preset.A
    config.material.n = preset.n
    config.material.Q = preset.Q
    config.material.temperature_reference = preset.Tref
  }
}

function applyTemplate(tpl: typeof quickTemplates[0]) {
  config.model = tpl.model
  config.applied_stress = tpl.stress
  config.temperature = tpl.temp
  materialPreset.value = tpl.preset
  applyMaterialPreset()
}

function resetAll() {
  results.value = null
  config.model = 'norton_power'
  config.applied_stress = 80e6
  config.temperature = 873
  config.time_range = { min: 0, max: 1000, step: 5 }
  config.hardening_type = 'time'
  materialPreset.value = 'ss304'
  applyMaterialPreset()
}

function runAnalysis() {
  analyzing.value = true
  // 使用 setTimeout 模拟异步分析
  setTimeout(() => {
    try {
      const curveData = generateCreepData()
      const rates = curveData.map(d => d.strain_rate)
      const minRate = Math.min(...rates)
      const steadyRate = minRate * 1.5
      const strains = curveData.map(d => d.strain)
      const maxStrain = Math.max(...strains)

      // 计算1%和5%应变时间
      let time1pct = curveData[curveData.length - 1].time
      let time5pct = curveData[curveData.length - 1].time
      for (const pt of curveData) {
        if (pt.strain >= 1 && time1pct === curveData[curveData.length - 1].time) time1pct = pt.time
        if (pt.strain >= 5 && time5pct === curveData[curveData.length - 1].time) time5pct = pt.time
      }

      // Larson-Miller参数: LMP = T(C + log10(tr))
      const C = 20
      const ruptureTimeHours = config.time_range.max * 0.95
      const lmp = config.temperature * (C + Math.log10(ruptureTimeHours))

      results.value = {
        success: true,
        creep_strain_curve: curveData,
        steady_state_rate: steadyRate,
        rupture_time: ruptureTimeHours * 3600,
        min_creep_rate: minRate,
        time_to_1pct_strain: time1pct * 3600,
        time_to_5pct_strain: time5pct * 3600,
        larson_miller_parameter: lmp,
        remaining_life_fraction: 0.72,
      }
    } catch (e) {
      console.error('Creep analysis failed:', e)
    } finally {
      analyzing.value = false
    }
  }, 800)
}

function runLifePrediction() {
  if (!results.value) return
  // 模拟剩余寿命预测
  const currentStrain = results.value.creep_strain_curve[
    Math.floor(results.value.creep_strain_curve.length * 0.3)
  ]?.strain || 0.5
  const remainingFraction = Math.max(0, 1 - currentStrain / Math.max(...results.value.creep_strain_curve.map(d => d.strain)))
  results.value.remaining_life_fraction = remainingFraction
}

function formatScientific(val: number): string {
  if (!val || !isFinite(val)) return 'N/A'
  if (val === 0) return '0'
  const exp = Math.floor(Math.log10(Math.abs(val)))
  const mantissa = val / Math.pow(10, exp)
  return `${mantissa.toFixed(2)}e${exp}`
}

function formatTime(seconds: number): string {
  if (!seconds || !isFinite(seconds)) return 'N/A'
  const hours = seconds / 3600
  if (hours >= 10000) return `${(hours / 1000).toFixed(1)} kh`
  if (hours >= 1) return `${hours.toFixed(1)} h`
  const minutes = seconds / 60
  return `${minutes.toFixed(1)} min`
}

function exportResults() {
  if (!results.value) return
  const data = JSON.stringify(results.value, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'creep_results.json'
  a.click()
  URL.revokeObjectURL(url)
}
</script>

<style scoped>
.panel-section {
  padding: 12px;
  background: var(--bg-elevated);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-subtle);
}

.label {
  display: block;
  font-size: 10px;
  color: var(--text-muted);
  margin-bottom: 4px;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.stat-card {
  display: flex;
  flex-direction: column;
  padding: 12px;
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
}

.stat-label {
  font-size: 10px;
  color: var(--text-muted);
  text-transform: uppercase;
  margin-bottom: 4px;
}

.stat-value {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}
</style>
