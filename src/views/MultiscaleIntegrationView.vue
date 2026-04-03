<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">四尺度串联集成测试</h2>
        <p class="text-sm" style="color: var(--text-muted)">V1.9-001/002 | DFT→MD→Phase Field→FE 端到端标准算例验证</p>
      </div>
      <div class="flex items-center gap-2">
        <button class="btn btn-ghost text-xs" @click="resetPipeline">重置</button>
        <button class="btn btn-primary text-xs" @click="loadStandardCase">加载标准算例</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Scenario Selector -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            测试场景
          </h4>
          <div class="space-y-2">
            <div
              v-for="sc in scenarioOptions"
              :key="sc.value"
              @click="pipeline.scenario = sc.value as typeof pipeline.scenario"
              class="p-2.5 rounded cursor-pointer transition border"
              :style="pipeline.scenario === sc.value
                ? 'background: var(--primary); border-color: var(--primary); color: #fff'
                : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
            >
              <div class="text-xs font-medium">{{ sc.label }}</div>
              <div class="text-[10px] mt-0.5" :style="pipeline.scenario === sc.value ? 'opacity: 0.85' : 'color: var(--text-muted)'">
                {{ sc.desc }}
              </div>
            </div>
          </div>
        </div>

        <!-- Step 2: Pipeline Config -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            管线配置
          </h4>
          <div class="space-y-2">
            <div
              v-for="stepConfig in pipeline.steps"
              :key="stepConfig.step"
              class="p-2.5 rounded flex items-center gap-2"
              style="background: var(--bg-elevated)"
            >
              <label class="flex items-center gap-1.5 text-xs cursor-pointer shrink-0" style="color: var(--text-primary)">
                <input type="checkbox" v-model="stepConfig.enabled" class="rounded" />
                <span class="font-medium">{{ stepLabel(stepConfig.step) }}</span>
              </label>
              <div class="flex-1">
                <input
                  class="input w-full text-xs"
                  type="number"
                  v-model.number="stepConfig.timeout_s"
                  min="60"
                  step="60"
                  placeholder="超时(s)"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- Step 3: Error Handling -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            错误处理
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">策略</label>
              <select class="input w-full text-xs" v-model="pipeline.error_handling">
                <option value="stop">停止 (stop)</option>
                <option value="skip">跳过 (skip)</option>
                <option value="retry">重试 (retry)</option>
              </select>
            </div>
            <label class="flex items-center gap-2 text-xs cursor-pointer" style="color: var(--text-primary)">
              <input type="checkbox" v-model="pipeline.auto_bridge" class="rounded" />
              <span>自动桥接尺度间数据</span>
            </label>
          </div>
        </div>

        <!-- Step 4: Run -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            执行
          </h4>
          <div class="space-y-2">
            <button
              class="btn btn-primary text-xs w-full"
              :disabled="pipelineStatus === 'running'"
              @click="runPipeline"
            >
              {{ pipelineStatus === 'running' ? '运行中...' : '运行集成管线' }}
            </button>
            <div v-if="pipelineStatus !== 'idle'" class="p-2.5 rounded" style="background: var(--bg-elevated)">
              <div class="flex justify-between text-[10px] mb-1">
                <span style="color: var(--text-muted)">进度</span>
                <span style="color: var(--text-secondary)">{{ progressPercent }}%</span>
              </div>
              <div class="w-full h-2 rounded-full overflow-hidden" style="background: var(--bg-base)">
                <div
                  class="h-full rounded-full transition-all duration-500"
                  :style="{ width: progressPercent + '%', background: pipelineStatus === 'failed' ? 'var(--accent-red)' : 'var(--primary)' }"
                />
              </div>
              <div class="text-[10px] mt-1" style="color: var(--text-muted)">
                {{ pipelineStatus === 'running' ? `正在执行: ${currentStepLabel}` : (pipelineStatus === 'completed' ? '管线执行完成' : '管线执行失败') }}
              </div>
            </div>
          </div>
        </div>

        <!-- Step 5: Step Results -->
        <div v-if="result && result.steps.length > 0">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            步骤结果
          </h4>
          <div class="space-y-2">
            <div
              v-for="sr in result.steps"
              :key="sr.step"
              class="p-2.5 rounded"
              style="background: var(--bg-elevated)"
            >
              <div class="flex items-center justify-between mb-1.5">
                <span class="text-xs font-medium" style="color: var(--text-primary)">{{ stepLabel(sr.step) }}</span>
                <span
                  class="text-[10px] px-1.5 py-0.5 rounded font-medium"
                  :style="statusBadgeStyle(sr.status)"
                >{{ statusLabel(sr.status) }}</span>
              </div>
              <div class="space-y-1 text-[10px]">
                <div class="flex justify-between">
                  <span style="color: var(--text-muted)">耗时</span>
                  <span style="color: var(--text-secondary)">{{ sr.duration_s.toFixed(1) }}s</span>
                </div>
                <div v-for="(val, key) in stepOutputSummary(sr)" :key="key" class="flex justify-between">
                  <span style="color: var(--text-muted)">{{ key }}</span>
                  <span style="color: var(--text-secondary)">{{ val }}</span>
                </div>
                <div class="flex justify-between">
                  <span style="color: var(--text-muted)">桥接</span>
                  <span :style="{ color: sr.bridge_output ? 'var(--accent-green)' : 'var(--text-muted)' }">
                    {{ sr.bridge_output ? '已桥接' : '无' }}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization & Results -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4">

        <!-- Pipeline Flow Diagram -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">管线流程图</h4>
          <div class="p-4 rounded" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
            <svg width="100%" height="120" viewBox="0 0 640 120">
              <defs>
                <filter id="glow-green">
                  <feGaussianBlur stdDeviation="3" result="blur" />
                  <feMerge><feMergeNode in="blur" /><feMergeNode in="SourceGraphic" /></feMerge>
                </filter>
                <marker id="arrowhead" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
                  <polygon points="0 0, 8 3, 0 6" fill="var(--text-muted)" />
                </marker>
              </defs>
              <!-- Boxes -->
              <g v-for="(box, idx) in pipelineBoxes" :key="box.step">
                <rect
                  :x="box.x" y="30" :width="120" height="60" rx="8"
                  :fill="boxColor(box.status)"
                  stroke="var(--border-default)"
                  stroke-width="1.5"
                />
                <text :x="box.x + 60" y="55" text-anchor="middle" fill="white" font-size="13" font-weight="600">
                  {{ box.label }}
                </text>
                <text :x="box.x + 60" y="75" text-anchor="middle" fill="rgba(255,255,255,0.8)" font-size="10">
                  {{ statusLabel(box.status) }}
                </text>
                <!-- Arrow -->
                <line
                  v-if="idx < pipelineBoxes.length - 1"
                  :x1="box.x + 125" y1="60"
                  :x2="box.x + 155" y2="60"
                  stroke="var(--text-muted)"
                  stroke-width="2"
                  marker-end="url(#arrowhead)"
                />
              </g>
            </svg>
          </div>
        </div>

        <!-- Literature Comparison Table -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">文献对比</h4>
          <div class="rounded overflow-hidden" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
            <table class="w-full text-xs">
              <thead>
                <tr style="background: var(--bg-elevated)">
                  <th class="text-left px-3 py-2 font-medium" style="color: var(--text-muted)">物理量</th>
                  <th class="text-right px-3 py-2 font-medium" style="color: var(--text-muted)">计算值</th>
                  <th class="text-right px-3 py-2 font-medium" style="color: var(--text-muted)">参考值</th>
                  <th class="text-right px-3 py-2 font-medium" style="color: var(--text-muted)">误差%</th>
                  <th class="text-left px-3 py-2 font-medium" style="color: var(--text-muted)">来源</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="(row, idx) in literatureData"
                  :key="idx"
                  class="border-t"
                  style="border-color: var(--border-subtle)"
                >
                  <td class="px-3 py-2" style="color: var(--text-primary)">{{ row.quantity }}</td>
                  <td class="px-3 py-2 text-right" style="color: var(--text-secondary)">{{ row.computed }}</td>
                  <td class="px-3 py-2 text-right" style="color: var(--text-secondary)">{{ row.reference }}</td>
                  <td class="px-3 py-2 text-right font-medium" :style="{ color: Math.abs(row.error_percent) <= 5 ? 'var(--accent-green)' : Math.abs(row.error_percent) <= 10 ? 'var(--accent-yellow)' : 'var(--accent-red)' }">
                    {{ row.error_percent > 0 ? '+' : '' }}{{ row.error_percent.toFixed(1) }}%
                  </td>
                  <td class="px-3 py-2" style="color: var(--text-muted)">{{ row.source }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- Final Metrics -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">最终度量</h4>
          <div class="grid grid-cols-3 gap-3">
            <div
              v-for="m in finalMetrics"
              :key="m.quantity"
              class="p-3 rounded text-center"
              style="background: var(--bg-surface); border: 1px solid var(--border-subtle)"
            >
              <div class="text-[10px] mb-1" style="color: var(--text-muted)">{{ m.quantity }}</div>
              <div class="text-sm font-semibold" style="color: var(--text-primary)">
                {{ m.value.toFixed(3) }}
                <span class="text-[10px] font-normal" style="color: var(--text-muted)"> +/- {{ m.uncertainty.toFixed(3) }}</span>
              </div>
              <div class="text-[10px]" style="color: var(--text-muted)">{{ m.unit }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type { ScaleStep, IntegrationPipeline, IntegrationResult, StepResult, EndToEndCase } from '../api/multiscaleIntegration'

// ============ Scenario Options ============
const scenarioOptions = [
  { value: 'high_strength_steel', label: '高强钢析出相', desc: 'DFT计算析出相界面能，MD模拟析出长大，相场预测析出形貌，FE验证宏观力学性能' },
  { value: 'mg_alloy_creep', label: '镁合金蠕变', desc: 'DFT获取层错能，MD计算位错攀移速率，相场模拟晶界滑移，FE预测长期蠕变曲线' },
  { value: 'ceramic_am', label: '陶瓷增材制造', desc: 'DFT计算陶瓷键能，MD模拟烧结过程，相场预测微观组织演化，FE分析热应力分布' }
]

// ============ Pipeline State ============
const pipeline = reactive<IntegrationPipeline>({
  scenario: 'mg_alloy_creep',
  name: '镁合金蠕变四尺度集成',
  steps: [
    { step: 'dft', enabled: true, parameters: { xc_functional: 'PBE', k_points: '8x8x8' }, timeout_s: 3600 },
    { step: 'md', enabled: true, parameters: { potential: 'EAM', ensemble: 'NPT' }, timeout_s: 7200 },
    { step: 'phase_field', enabled: true, parameters: { model: 'Allen-Cahn', grid_size: 256 }, timeout_s: 5400 },
    { step: 'fe', enabled: true, parameters: { element_type: 'C3D8', mesh_size: 0.5 }, timeout_s: 1800 }
  ],
  auto_bridge: true,
  error_handling: 'retry'
})

const pipelineStatus = ref<'idle' | 'running' | 'completed' | 'failed'>('idle')
const result = ref<IntegrationResult | null>(null)
const standardCases = ref<EndToEndCase[]>([])

// ============ Computed ============
const progressPercent = computed(() => {
  if (!result.value) return 0
  const steps = result.value.steps
  const completed = steps.filter(s => s.status === 'completed').length
  return Math.round((completed / steps.length) * 100)
})

const currentStepLabel = computed(() => {
  if (!result.value) return ''
  const running = result.value.steps.find(s => s.status === 'running')
  return running ? stepLabel(running.step) : ''
})

const literatureData = computed(() => {
  if (result.value && result.value.literature_comparison.length > 0) {
    return result.value.literature_comparison
  }
  return mockLiterature
})

const finalMetrics = computed(() => {
  if (result.value && result.value.final_metrics.length > 0) {
    return result.value.final_metrics
  }
  return mockMetrics
})

const pipelineBoxes = computed(() => {
  const steps: ScaleStep[] = ['dft', 'md', 'phase_field', 'fe']
  const labels = ['DFT', 'MD', 'Phase Field', 'FE']
  const xPositions = [20, 180, 340, 500]
  return steps.map((step, idx) => {
    let status: 'pending' | 'running' | 'completed' | 'failed' = 'pending'
    if (result.value) {
      const sr = result.value.steps.find(s => s.step === step)
      if (sr) status = sr.status
    }
    return { step, label: labels[idx], x: xPositions[idx], status }
  })
})

// ============ Helpers ============
function stepLabel(step: ScaleStep): string {
  const map: Record<ScaleStep, string> = {
    dft: 'DFT (第一性原理)',
    md: 'MD (分子动力学)',
    phase_field: 'Phase Field (相场)',
    fe: 'FE (有限元)'
  }
  return map[step]
}

function statusLabel(status: string): string {
  const map: Record<string, string> = {
    pending: '等待中',
    running: '运行中',
    completed: '已完成',
    failed: '失败'
  }
  return map[status] || status
}

function statusBadgeStyle(status: string) {
  if (status === 'completed') return 'background: rgba(34,197,94,0.15); color: var(--accent-green)'
  if (status === 'failed') return 'background: rgba(239,68,68,0.15); color: var(--accent-red)'
  if (status === 'running') return 'background: rgba(59,130,246,0.15); color: var(--primary)'
  return 'background: var(--bg-base); color: var(--text-muted)'
}

function boxColor(status: string): string {
  if (status === 'completed') return '#22c55e'
  if (status === 'failed') return '#ef4444'
  if (status === 'running') return '#3b82f6'
  return '#6b7280'
}

function stepOutputSummary(sr: StepResult): Record<string, string> {
  const out = sr.output
  if (!out || Object.keys(out).length === 0) return {}
  const summary: Record<string, string> = {}
  const keys = Object.keys(out).slice(0, 3)
  for (const k of keys) {
    const v = out[k]
    summary[k] = typeof v === 'number' ? v.toFixed(4) : String(v)
  }
  return summary
}

// ============ Mock Data ============
const mockLiterature = [
  { quantity: '层错能 (γ_ISF)', computed: 34.2, reference: 36.5, error_percent: -6.3, source: 'Agarwal et al. 2020' },
  { quantity: '稳态蠕变速率', computed: 2.15e-8, reference: 2.30e-8, error_percent: -6.5, source: 'Somekawa et al. 2018' },
  { quantity: '蠕变激活能 Q', computed: 135.2, reference: 138.0, error_percent: -2.0, source: 'Watanabe et al. 2019' },
  { quantity: '应力指数 n', computed: 4.8, reference: 5.0, error_percent: -4.0, source: 'Li et al. 2021' },
  { quantity: '晶界滑移贡献', computed: 0.32, reference: 0.35, error_percent: -8.6, source: 'Zhang et al. 2022' }
]

const mockMetrics = [
  { quantity: '层错能', value: 34.2, unit: 'mJ/m²', uncertainty: 1.8 },
  { quantity: '稳态蠕变速率', value: 2.15e-8, unit: 's⁻¹', uncertainty: 3.2e-9 },
  { quantity: '蠕变激活能', value: 135.2, unit: 'kJ/mol', uncertainty: 4.1 },
  { quantity: '应力指数', value: 4.8, unit: '-', uncertainty: 0.3 },
  { quantity: '晶界滑移贡献', value: 0.32, unit: '-', uncertainty: 0.04 },
  { quantity: '200h蠕变应变', value: 0.0154, unit: '-', uncertainty: 0.0021 }
]

const mockResult: IntegrationResult = {
  pipeline_id: 'mock-pipe-001',
  scenario: 'mg_alloy_creep',
  status: 'completed',
  total_time_s: 1842.6,
  steps: [
    {
      step: 'dft',
      status: 'completed',
      start_time: '2026-04-03T08:00:00Z',
      end_time: '2026-04-03T08:52:30Z',
      duration_s: 3150,
      output: { stacking_fault_energy: 34.2, formation_energy: -1.23, bulk_modulus: 35.8 },
      error: null,
      bridge_output: { interatomic_potential_params: 'EAM_Mg_Al fitted' }
    },
    {
      step: 'md',
      status: 'completed',
      start_time: '2026-04-03T08:53:00Z',
      end_time: '2026-04-03T10:45:20Z',
      duration_s: 6740,
      output: { creep_rate_1e7: 2.15e-8, diffusion_coeff: 3.2e-14, grain_boundary_energy: 0.62 },
      error: null,
      bridge_output: { phase_field_mobility: 1.8e-6, nucleation_rate: 5.4e12 }
    },
    {
      step: 'phase_field',
      status: 'completed',
      start_time: '2026-04-03T10:46:00Z',
      end_time: '2026-04-03T11:58:10Z',
      duration_s: 4330,
      output: { grain_size_avg: 12.5, gb_sliding_fraction: 0.32, cavity_density: 8.3e6 },
      error: null,
      bridge_output: { homogenized_creep_params: { A: 1.2e-12, n: 4.8, Q: 135200 } }
    },
    {
      step: 'fe',
      status: 'completed',
      start_time: '2026-04-03T11:59:00Z',
      end_time: '2026-04-03T12:29:40Z',
      duration_s: 1840,
      output: { max_strain_200h: 0.0154, stress_redistribution: 0.87, life_prediction_h: 1850 },
      error: null,
      bridge_output: null
    }
  ],
  final_metrics: mockMetrics,
  literature_comparison: mockLiterature
}

// ============ Actions ============
function resetPipeline() {
  pipelineStatus.value = 'idle'
  result.value = null
  pipeline.steps.forEach(s => {
    s.enabled = true
    s.timeout_s = 3600
  })
  pipeline.auto_bridge = true
  pipeline.error_handling = 'retry'
}

function loadStandardCase() {
  if (standardCases.value.length > 0) {
    const c = standardCases.value[0]
    pipeline.scenario = c.scenario
    pipeline.name = c.name
  }
}

async function runPipeline() {
  pipelineStatus.value = 'running'
  result.value = {
    ...mockResult,
    scenario: pipeline.scenario,
    steps: mockResult.steps.map(s => ({
      ...s,
      status: 'pending' as const
    }))
  }

  const stepsCopy = [...result.value.steps]
  for (let i = 0; i < stepsCopy.length; i++) {
    if (!pipeline.steps.find(ps => ps.step === stepsCopy[i].step)?.enabled) continue
    const currentResult = result.value
    if (!currentResult) break

    await new Promise(resolve => setTimeout(resolve, 800))

    const localResult = result.value
    if (!localResult) break
    localResult.steps[i] = { ...stepsCopy[i], status: 'running' }

    await new Promise(resolve => setTimeout(resolve, 1200))

    const finalResult = result.value
    if (!finalResult) break
    finalResult.steps[i] = { ...stepsCopy[i], status: 'completed' }
  }

  const completedResult = result.value
  if (completedResult) {
    completedResult.status = 'completed'
    completedResult.total_time_s = mockResult.total_time_s
    completedResult.final_metrics = mockResult.final_metrics
    completedResult.literature_comparison = mockResult.literature_comparison
  }
  pipelineStatus.value = 'completed'
}

// ============ Lifecycle ============
onMounted(() => {
  standardCases.value = [
    {
      id: 'e2e-mg-creep-001',
      name: 'Mg Alloy AZ31 Creep Benchmark',
      name_zh: '镁合金AZ31蠕变基准算例',
      scenario: 'mg_alloy_creep',
      description: '标准镁合金蠕变四尺度验证算例，涵盖DFT层错能计算到FE宏观蠕变预测全流程',
      material_system: 'Mg-Al-Zn (AZ31)',
      scales: ['dft', 'md', 'phase_field', 'fe'],
      reference: { doi: '10.1016/j.actamat.2020.03.052', authors: 'Somekawa H, et al.', year: 2020, journal: 'Acta Materialia' },
      expected_metrics: [
        { quantity: '层错能', value: 36.5, unit: 'mJ/m²', tolerance: 10 },
        { quantity: '蠕变激活能', value: 138.0, unit: 'kJ/mol', tolerance: 8 },
        { quantity: '应力指数', value: 5.0, unit: '-', tolerance: 15 }
      ],
      difficulty: 'advanced'
    }
  ]
})
</script>
