<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">误差传播追踪</h2>
        <p class="text-sm" style="color: var(--text-muted)">V1.8-004 | 误差传播追踪框架，Monte Carlo 不确定性量化</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="resetAll" class="btn btn-ghost text-xs">重置</button>
        <button v-if="report" @click="exportReport" class="btn btn-primary text-xs">导出报告</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Pipeline Setup -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            管线配置
          </h4>
          <div class="mb-3">
            <label class="label">管线名称</label>
            <input v-model="config.pipeline_name" type="text" class="input w-full text-xs" placeholder="DFT-MD-PF-FE 多尺度管线" />
          </div>
          <div class="mb-2">
            <label class="label">涉及尺度</label>
            <div class="grid grid-cols-2 gap-1 mt-1">
              <label v-for="s in scaleOptions" :key="s.value" class="flex items-center gap-1.5 px-2 py-1.5 rounded text-xs cursor-pointer border transition" :style="config.scales.includes(s.value) ? 'background: var(--primary); border-color: var(--primary); color: white' : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'">
                <input type="checkbox" :value="s.value" v-model="config.scales" class="sr-only" />
                {{ s.label }}
              </label>
            </div>
          </div>
        </div>

        <!-- Step 2: Monte Carlo Settings -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            Monte Carlo 设置
          </h4>
          <label class="flex items-center gap-2 mb-3 cursor-pointer">
            <input v-model="config.enable_monte_carlo" type="checkbox" class="rounded" />
            <span class="text-xs" style="color: var(--text-secondary)">启用 Monte Carlo 采样</span>
          </label>
          <div v-if="config.enable_monte_carlo">
            <label class="label">采样数</label>
            <input v-model.number="config.mc_samples" type="number" min="100" max="100000" step="100" class="input w-full text-xs" />
            <div class="flex justify-between mt-1">
              <span class="text-[10px]" style="color: var(--text-muted)">100</span>
              <span class="text-[10px]" style="color: var(--text-muted)">100,000</span>
            </div>
          </div>
        </div>

        <!-- Step 3: Create Pipeline -->
        <button @click="createPipeline" :disabled="!config.pipeline_name || config.scales.length === 0" class="btn btn-primary text-xs w-full">
          创建管线
        </button>

        <!-- Step 4: Error Contributions -->
        <div v-if="pipelineCreated" class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            误差贡献项
          </h4>
          <div v-for="(c, idx) in contributions" :key="idx" class="mb-3 p-2 rounded border" style="background: var(--bg-elevated); border-color: var(--border-default)">
            <div class="flex items-center justify-between mb-2">
              <span class="text-xs font-medium" style="color: var(--text-secondary)">贡献 #{{ idx + 1 }}</span>
              <button @click="removeContribution(idx)" class="text-xs px-1.5 py-0.5 rounded" style="color: var(--accent-red)">删除</button>
            </div>
            <div class="space-y-2">
              <div>
                <label class="label">误差来源</label>
                <select v-model="c.source" class="input w-full text-xs">
                  <option v-for="src in errorSourceOptions" :key="src.value" :value="src.value">{{ src.label }}</option>
                </select>
              </div>
              <div>
                <label class="label">尺度</label>
                <select v-model="c.scale" class="input w-full text-xs">
                  <option v-for="s in config.scales" :key="s" :value="s">{{ scaleLabels[s] || s }}</option>
                </select>
              </div>
              <div>
                <label class="label">描述</label>
                <input v-model="c.description" type="text" class="input w-full text-xs" placeholder="误差描述" />
              </div>
              <div class="grid grid-cols-2 gap-2">
                <div>
                  <label class="label">绝对误差</label>
                  <input v-model.number="c.absolute_error" type="number" step="0.001" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">相对误差 (%)</label>
                  <input v-model.number="c.relative_error" type="number" step="0.1" class="input w-full text-xs" />
                </div>
              </div>
            </div>
          </div>
          <button @click="addContribution" class="btn btn-ghost text-xs w-full">+ 添加贡献项</button>
        </div>

        <!-- Step 5: Generate Report -->
        <div v-if="pipelineCreated && contributions.length > 0">
          <button @click="generateReport" :disabled="generating" class="btn btn-primary text-xs w-full mb-3">
            {{ generating ? '生成中...' : '生成报告' }}
          </button>
          <div v-if="report" class="p-3 rounded border" style="background: var(--bg-elevated); border-color: var(--border-default)">
            <h5 class="text-xs font-medium mb-2" style="color: var(--text-primary)">报告摘要</h5>
            <div class="space-y-1">
              <div class="flex justify-between text-xs">
                <span style="color: var(--text-muted)">累积不确定度</span>
                <span style="color: var(--text-primary)">{{ report.cumulative_uncertainty.toFixed(4) }}</span>
              </div>
              <div class="flex justify-between text-xs">
                <span style="color: var(--text-muted)">95% 置信区间</span>
                <span style="color: var(--text-primary)">[{{ report.final_result.confidence_95[0].toFixed(4) }}, {{ report.final_result.confidence_95[1].toFixed(4) }}]</span>
              </div>
              <div class="flex justify-between text-xs">
                <span style="color: var(--text-muted)">最终结果</span>
                <span style="color: var(--text-primary)">{{ report.final_result.value.toFixed(4) }} +/- {{ report.final_result.uncertainty.toFixed(4) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization & Results -->
      <div class="flex-1 flex flex-col overflow-hidden">

        <!-- No data placeholder -->
        <div v-if="!report" class="flex-1 flex items-center justify-center" style="color: var(--text-muted)">
          <div class="text-center">
            <svg width="64" height="64" viewBox="0 0 64 64" class="mx-auto mb-3 opacity-30">
              <rect x="8" y="16" width="12" height="32" rx="2" fill="currentColor" />
              <rect x="26" y="8" width="12" height="40" rx="2" fill="currentColor" />
              <rect x="44" y="24" width="12" height="24" rx="2" fill="currentColor" />
              <line x1="4" y1="52" x2="60" y2="52" stroke="currentColor" stroke-width="1.5" />
            </svg>
            <div class="text-sm">创建管线并添加误差贡献项后生成报告</div>
          </div>
        </div>

        <!-- Visualization -->
        <div v-if="report" class="flex-1 overflow-y-auto p-4 space-y-4">

          <!-- Waterfall Chart: Error Accumulation -->
          <div class="p-4 rounded-lg border" style="background: var(--bg-surface); border-color: var(--border-subtle)">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">误差累积瀑布图</h4>
            <svg :viewBox="`0 0 ${waterfallWidth} ${waterfallHeight}`" class="w-full" style="max-height: 260px">
              <!-- Grid lines -->
              <line v-for="i in 5" :key="'grid-' + i"
                :x1="padding" :y1="padding + (waterfallHeight - 2 * padding) * (1 - i / 5)"
                :x2="waterfallWidth - padding" :y2="padding + (waterfallHeight - 2 * padding) * (1 - i / 5)"
                stroke="var(--border-subtle)" stroke-dasharray="4,4" />
              <!-- Y axis labels -->
              <text v-for="i in 5" :key="'ylab-' + i"
                :x="padding - 4" :y="padding + (waterfallHeight - 2 * padding) * (1 - i / 5) + 4"
                text-anchor="end" fill="var(--text-muted)" font-size="9">
                {{ (maxCumulative * i / 5).toFixed(3) }}
              </text>
              <!-- Bars + error bars -->
              <g v-for="(step, idx) in report.steps" :key="step.step_id">
                <rect
                  :x="padding + idx * barWidth + barWidth * 0.15"
                  :y="padding + (waterfallHeight - 2 * padding) * (1 - step.output_uncertainty / maxCumulative)"
                  :width="barWidth * 0.7"
                  :height="(waterfallHeight - 2 * padding) * (step.output_uncertainty / maxCumulative)"
                  rx="3"
                  :fill="stepColors[idx % stepColors.length]"
                  opacity="0.85" />
                <!-- Error bar -->
                <line
                  :x1="padding + idx * barWidth + barWidth * 0.5"
                  :y1="padding + (waterfallHeight - 2 * padding) * (1 - (step.output_uncertainty + step.output_uncertainty * 0.15) / maxCumulative)"
                  :x2="padding + idx * barWidth + barWidth * 0.5"
                  :y2="padding + (waterfallHeight - 2 * padding) * (1 - Math.max(0, step.output_uncertainty - step.output_uncertainty * 0.15) / maxCumulative)"
                  stroke="var(--text-muted)" stroke-width="1.5" />
                <line
                  :x1="padding + idx * barWidth + barWidth * 0.3"
                  :y1="padding + (waterfallHeight - 2 * padding) * (1 - (step.output_uncertainty + step.output_uncertainty * 0.15) / maxCumulative)"
                  :x2="padding + idx * barWidth + barWidth * 0.7"
                  :y2="padding + (waterfallHeight - 2 * padding) * (1 - (step.output_uncertainty + step.output_uncertainty * 0.15) / maxCumulative)"
                  stroke="var(--text-muted)" stroke-width="1.5" />
                <line
                  :x1="padding + idx * barWidth + barWidth * 0.3"
                  :y1="padding + (waterfallHeight - 2 * padding) * (1 - Math.max(0, step.output_uncertainty - step.output_uncertainty * 0.15) / maxCumulative)"
                  :x2="padding + idx * barWidth + barWidth * 0.7"
                  :y2="padding + (waterfallHeight - 2 * padding) * (1 - Math.max(0, step.output_uncertainty - step.output_uncertainty * 0.15) / maxCumulative)"
                  stroke="var(--text-muted)" stroke-width="1.5" />
                <!-- X label -->
                <text
                  :x="padding + idx * barWidth + barWidth * 0.5"
                  :y="waterfallHeight - 4"
                  text-anchor="middle" fill="var(--text-muted)" font-size="9">
                  {{ step.method }}
                </text>
                <!-- Value label -->
                <text
                  :x="padding + idx * barWidth + barWidth * 0.5"
                  :y="padding + (waterfallHeight - 2 * padding) * (1 - step.output_uncertainty / maxCumulative) - 6"
                  text-anchor="middle" fill="var(--text-primary)" font-size="9" font-weight="500">
                  {{ step.output_uncertainty.toFixed(4) }}
                </text>
              </g>
            </svg>
          </div>

          <!-- Sensitivity Analysis -->
          <div class="p-4 rounded-lg border" style="background: var(--bg-surface); border-color: var(--border-subtle)">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">灵敏度分析</h4>
            <svg :viewBox="`0 0 ${sensitivityWidth} ${sensitivityHeight}`" class="w-full" style="max-height: 220px">
              <g v-for="(item, idx) in report.sensitivity_analysis" :key="item.parameter">
                <text :x="sensitivityWidth - 8" :y="30 + idx * 32 + 12" text-anchor="end" fill="var(--text-secondary)" font-size="10">
                  {{ item.parameter }}
                </text>
                <rect
                  :x="8"
                  :y="30 + idx * 32"
                  :width="Math.max(2, (sensitivityWidth - 120) * item.sensitivity / maxSensitivity)"
                  height="20"
                  rx="3"
                  :fill="sensitivityColors[idx % sensitivityColors.length]"
                  opacity="0.8" />
                <text :x="12" :y="30 + idx * 32 + 14" fill="white" font-size="9" font-weight="500">
                  {{ item.sensitivity.toFixed(3) }}
                </text>
              </g>
            </svg>
          </div>

          <!-- Error Report Cards -->
          <div>
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">分步误差报告</h4>
            <div class="grid grid-cols-1 gap-3">
              <div v-for="(step, idx) in report.steps" :key="step.step_id" class="p-3 rounded-lg border" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="flex items-center justify-between mb-2">
                  <div class="flex items-center gap-2">
                    <span class="inline-flex items-center justify-center w-6 h-6 rounded-full text-white text-xs" :style="{ background: stepColors[idx % stepColors.length] }">{{ idx + 1 }}</span>
                    <div>
                      <div class="text-xs font-medium" style="color: var(--text-primary)">{{ step.method }}</div>
                      <div class="text-[10px]" style="color: var(--text-muted)">{{ step.from_scale }} -> {{ step.to_scale }}</div>
                    </div>
                  </div>
                  <div class="text-right">
                    <div class="text-xs font-medium" style="color: var(--text-primary)">U = {{ step.output_uncertainty.toFixed(4) }}</div>
                    <div class="text-[10px]" style="color: var(--text-muted)">输入 U = {{ step.input_uncertainty.toFixed(4) }}</div>
                  </div>
                </div>
                <div v-if="step.contributions.length > 0" class="mt-2 pt-2 border-t" style="border-color: var(--border-subtle)">
                  <div class="text-[10px] font-medium mb-1" style="color: var(--text-muted)">贡献分解</div>
                  <div v-for="c in step.contributions" :key="c.source + c.description" class="flex items-center justify-between py-0.5">
                    <span class="text-[10px]" style="color: var(--text-secondary)">{{ sourceLabels[c.source] }}: {{ c.description }}</span>
                    <span class="text-[10px]" style="color: var(--text-primary)">abs {{ c.absolute_error.toFixed(4) }} | rel {{ c.relative_error.toFixed(2) }}%</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type { ErrorSource, ErrorContribution, PropagationStep, ErrorReport, ErrorTrackingConfig } from '../api/errorTracking'

// ============ Constants ============

const scaleOptions = [
  { value: 'DFT', label: 'DFT' },
  { value: 'MD', label: 'MD' },
  { value: 'Phase Field', label: 'Phase Field' },
  { value: 'FE', label: 'FE' }
]

const scaleLabels: Record<string, string> = {
  'DFT': 'DFT 量子尺度',
  'MD': 'MD 分子动力学',
  'Phase Field': '相场介观尺度',
  'FE': 'FE 宏观有限元'
}

const errorSourceOptions = [
  { value: 'numerical', label: '数值误差' },
  { value: 'discretization', label: '离散化误差' },
  { value: 'coarse_graining', label: '粗粒化误差' },
  { value: 'unit_conversion', label: '单位转换误差' },
  { value: 'model', label: '模型误差' },
  { value: 'measurement', label: '测量误差' }
]

const sourceLabels: Record<ErrorSource, string> = {
  numerical: '数值',
  discretization: '离散化',
  coarse_graining: '粗粒化',
  unit_conversion: '单位转换',
  model: '模型',
  measurement: '测量'
}

const stepColors = ['#6366f1', '#8b5cf6', '#a855f7', '#d946ef', '#ec4899']
const sensitivityColors = ['#6366f1', '#8b5cf6', '#06b6d4', '#10b981', '#f59e0b', '#ef4444']

// ============ State ============

const config = reactive<ErrorTrackingConfig>({
  pipeline_name: '',
  scales: [],
  enable_monte_carlo: false,
  mc_samples: 10000
})

const pipelineCreated = ref(false)
const generating = ref(false)
const report = ref<ErrorReport | null>(null)

interface LocalContribution {
  source: ErrorSource
  scale: string
  description: string
  absolute_error: number
  relative_error: number
}

const contributions = ref<LocalContribution[]>([])

// ============ SVG Dimensions ============

const padding = 40
const waterfallWidth = 600
const waterfallHeight = 260
const sensitivityWidth = 500
const sensitivityHeight = 220

const barWidth = computed(() => {
  const steps = report.value ? report.value.steps.length : 0
  return steps > 0 ? (waterfallWidth - 2 * padding) / steps : 0
})

const maxCumulative = computed(() => {
  if (!report.value || report.value.steps.length === 0) return 1
  return Math.max(...report.value.steps.map(s => s.output_uncertainty)) * 1.2
})

const maxSensitivity = computed(() => {
  if (!report.value || report.value.sensitivity_analysis.length === 0) return 1
  return Math.max(...report.value.sensitivity_analysis.map(s => s.sensitivity))
})

// ============ Actions ============

function addContribution() {
  contributions.value.push({
    source: 'numerical',
    scale: config.scales[0] || 'DFT',
    description: '',
    absolute_error: 0.0,
    relative_error: 0.0
  })
}

function removeContribution(idx: number) {
  contributions.value.splice(idx, 1)
}

function createPipeline() {
  pipelineCreated.value = true
  if (contributions.value.length === 0) {
    addContribution()
  }
}

function generateReport() {
  generating.value = true
  // Simulate async generation
  setTimeout(() => {
    report.value = buildMockReport()
    generating.value = false
  }, 800)
}

function resetAll() {
  config.pipeline_name = ''
  config.scales = []
  config.enable_monte_carlo = false
  config.mc_samples = 10000
  pipelineCreated.value = false
  report.value = null
  contributions.value = []
}

function exportReport() {
  if (!report.value) return
  const data = JSON.stringify(report.value, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'error_tracking_report.json'
  a.click()
  URL.revokeObjectURL(url)
}

// ============ Mock Data ============

function buildMockReport(): ErrorReport {
  const steps: PropagationStep[] = [
    {
      step_id: 'step-1',
      from_scale: 'DFT',
      to_scale: 'MD',
      method: 'DFT势函数拟合',
      input_uncertainty: 0.0012,
      output_uncertainty: 0.0035,
      contributions: [
        {
          source: 'numerical',
          scale: 'DFT',
          step_id: 'step-1',
          description: '平面波截断能收敛',
          absolute_error: 0.0008,
          relative_error: 0.15,
          confidence_interval: [0.0005, 0.0011]
        },
        {
          source: 'model',
          scale: 'DFT',
          step_id: 'step-1',
          description: '交换关联泛函近似',
          absolute_error: 0.0015,
          relative_error: 0.28,
          confidence_interval: [0.0010, 0.0020]
        }
      ],
      timestamp: new Date().toISOString()
    },
    {
      step_id: 'step-2',
      from_scale: 'MD',
      to_scale: 'MD',
      method: 'MD NVE 系综模拟',
      input_uncertainty: 0.0035,
      output_uncertainty: 0.0082,
      contributions: [
        {
          source: 'numerical',
          scale: 'MD',
          step_id: 'step-2',
          description: '时间步长积分误差',
          absolute_error: 0.0012,
          relative_error: 0.12,
          confidence_interval: [0.0008, 0.0016]
        },
        {
          source: 'discretization',
          scale: 'MD',
          step_id: 'step-2',
          description: '截断半径设置',
          absolute_error: 0.0020,
          relative_error: 0.20,
          confidence_interval: [0.0014, 0.0026]
        },
        {
          source: 'measurement',
          scale: 'MD',
          step_id: 'step-2',
          description: '系综统计采样不足',
          absolute_error: 0.0015,
          relative_error: 0.15,
          confidence_interval: [0.0010, 0.0020]
        }
      ],
      timestamp: new Date().toISOString()
    },
    {
      step_id: 'step-3',
      from_scale: 'MD',
      to_scale: 'Phase Field',
      method: '粗粒化参数映射',
      input_uncertainty: 0.0082,
      output_uncertainty: 0.0156,
      contributions: [
        {
          source: 'coarse_graining',
          scale: 'Phase Field',
          step_id: 'step-3',
          description: '自由能密度函数拟合',
          absolute_error: 0.0040,
          relative_error: 0.22,
          confidence_interval: [0.0028, 0.0052]
        },
        {
          source: 'unit_conversion',
          scale: 'Phase Field',
          step_id: 'step-3',
          description: 'eV/atom 到 J/m^3 转换',
          absolute_error: 0.0006,
          relative_error: 0.03,
          confidence_interval: [0.0003, 0.0009]
        },
        {
          source: 'model',
          scale: 'Phase Field',
          step_id: 'step-3',
          description: '梯度能系数近似',
          absolute_error: 0.0028,
          relative_error: 0.16,
          confidence_interval: [0.0020, 0.0036]
        }
      ],
      timestamp: new Date().toISOString()
    },
    {
      step_id: 'step-4',
      from_scale: 'Phase Field',
      to_scale: 'Phase Field',
      method: '相场演化求解',
      input_uncertainty: 0.0156,
      output_uncertainty: 0.0243,
      contributions: [
        {
          source: 'discretization',
          scale: 'Phase Field',
          step_id: 'step-4',
          description: '网格尺寸敏感性',
          absolute_error: 0.0050,
          relative_error: 0.18,
          confidence_interval: [0.0035, 0.0065]
        },
        {
          source: 'numerical',
          scale: 'Phase Field',
          step_id: 'step-4',
          description: '隐式求解器迭代收敛',
          absolute_error: 0.0037,
          relative_error: 0.13,
          confidence_interval: [0.0025, 0.0049]
        }
      ],
      timestamp: new Date().toISOString()
    },
    {
      step_id: 'step-5',
      from_scale: 'Phase Field',
      to_scale: 'FE',
      method: 'FE 宏观均质化',
      input_uncertainty: 0.0243,
      output_uncertainty: 0.0387,
      contributions: [
        {
          source: 'coarse_graining',
          scale: 'FE',
          step_id: 'step-5',
          description: 'RVE 代表体积元尺寸',
          absolute_error: 0.0080,
          relative_error: 0.20,
          confidence_interval: [0.0055, 0.0105]
        },
        {
          source: 'discretization',
          scale: 'FE',
          step_id: 'step-5',
          description: '有限元网格密度',
          absolute_error: 0.0044,
          relative_error: 0.11,
          confidence_interval: [0.0030, 0.0058]
        },
        {
          source: 'model',
          scale: 'FE',
          step_id: 'step-5',
          description: '本构模型简化',
          absolute_error: 0.0020,
          relative_error: 0.05,
          confidence_interval: [0.0012, 0.0028]
        }
      ],
      timestamp: new Date().toISOString()
    }
  ]

  const cumulativeUncertainty = steps[steps.length - 1].output_uncertainty
  const finalValue = 2.4567
  const finalUncertainty = cumulativeUncertainty * finalValue

  return {
    pipeline_id: 'pipeline-' + Date.now(),
    total_steps: steps.length,
    cumulative_uncertainty: cumulativeUncertainty,
    steps,
    final_result: {
      value: finalValue,
      uncertainty: finalUncertainty,
      confidence_95: [finalValue - 1.96 * finalUncertainty, finalValue + 1.96 * finalUncertainty]
    },
    sensitivity_analysis: [
      { parameter: '势函数参数 epsilon', sensitivity: 0.342 },
      { parameter: '网格尺寸 dx', sensitivity: 0.278 },
      { parameter: '时间步长 dt', sensitivity: 0.195 },
      { parameter: '截断半径 rc', sensitivity: 0.142 },
      { parameter: '梯度能系数 kappa', sensitivity: 0.098 },
      { parameter: 'RVE 尺寸 L', sensitivity: 0.067 }
    ]
  }
}

// ============ Lifecycle ============

onMounted(() => {
  // Pre-fill for demo
  config.pipeline_name = 'DFT-MD-PF-FE 多尺度管线'
  config.scales = ['DFT', 'MD', 'Phase Field', 'FE']
  config.enable_monte_carlo = true
  config.mc_samples = 10000
})
</script>

<style scoped>
.panel-section {
  padding: 12px;
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
  border: 1px solid var(--border-subtle);
}

.label {
  display: block;
  font-size: 10px;
  color: var(--text-muted);
  margin-bottom: 4px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.input {
  padding: 6px 8px;
  background: var(--bg-base);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 12px;
  outline: none;
  transition: border-color 0.15s;
}

.input:focus {
  border-color: var(--primary);
  box-shadow: 0 0 0 2px var(--primary-glow);
}

.btn {
  padding: 6px 12px;
  border-radius: var(--radius-md);
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
  border: 1px solid transparent;
}

.btn-primary {
  background: var(--primary);
  color: white;
  border-color: var(--primary);
}

.btn-primary:hover {
  opacity: 0.9;
  box-shadow: 0 0 8px var(--primary-glow);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  box-shadow: none;
}

.btn-ghost {
  background: transparent;
  color: var(--text-secondary);
  border-color: var(--border-default);
}

.btn-ghost:hover {
  background: var(--bg-elevated);
  color: var(--text-primary);
}
</style>
