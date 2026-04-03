<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">自适应精度控制</h2>
        <p class="text-sm" style="color: var(--text-muted)">V2.0-008 | 多尺度分析精度自动评估与迭代优化</p>
      </div>
      <div class="flex items-center gap-2">
        <span
          v-if="controlResult"
          class="px-2 py-1 rounded text-[10px] font-medium"
          :style="statusBadgeStyle(controlResult.overall_status)"
        >{{ statusLabel(controlResult.overall_status) }}</span>
        <button class="btn btn-ghost text-xs" @click="resetConfig">重置</button>
        <button class="btn btn-primary text-xs" @click="handleEvaluate">评估精度</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Execution & Parameters -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            执行与参数
          </h4>
          <div class="space-y-2">
            <label class="label">关联执行</label>
            <select class="input w-full text-xs" v-model="config.execution_id">
              <option value="">选择执行...</option>
              <option v-for="ex in executionOptions" :key="ex.id" :value="ex.id">{{ ex.label }}</option>
            </select>
            <label class="flex items-center gap-2 text-xs cursor-pointer" style="color: var(--text-secondary)">
              <input type="checkbox" v-model="config.enable" class="rounded" />
              启用自适应精度控制
            </label>
            <label class="label">容差 (%)</label>
            <input class="input w-full text-xs" type="number" v-model.number="config.tolerance_percent" min="0.1" max="50" step="0.1" />
            <label class="label">最大迭代次数</label>
            <input class="input w-full text-xs" type="number" v-model.number="config.max_iterations" min="1" max="20" step="1" />
          </div>
        </div>

        <!-- Step 2: Strategy Selection -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            精度策略
          </h4>
          <div class="space-y-1.5">
            <label
              v-for="strat in strategyOptions"
              :key="strat.value"
              class="flex items-center gap-2 p-2 rounded cursor-pointer transition border"
              :style="config.strategies.includes(strat.value)
                ? 'background: rgba(59,130,246,0.08); border-color: var(--primary)'
                : 'background: var(--bg-elevated); border-color: var(--border-default)'"
            >
              <input type="checkbox" :value="strat.value" v-model="config.strategies" class="rounded" />
              <div>
                <div class="text-xs font-medium" style="color: var(--text-primary)">{{ strat.label }}</div>
                <div class="text-[10px]" style="color: var(--text-muted)">{{ strat.desc }}</div>
              </div>
            </label>
          </div>
        </div>

        <!-- Step 3: Metric Checks -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            精度指标
          </h4>
          <div class="space-y-2">
            <div
              v-for="check in metricChecks"
              :key="check.metric_name"
              class="p-2 rounded border"
              :style="check.is_within_tolerance
                ? 'border-color: var(--accent-green); background: rgba(34,197,94,0.05)'
                : 'border-color: var(--accent-red); background: rgba(239,68,68,0.05)'"
            >
              <div class="flex items-center justify-between">
                <span class="text-xs font-medium" style="color: var(--text-primary)">{{ check.metric_name }}</span>
                <span
                  class="px-1.5 py-0.5 rounded text-[9px] font-medium"
                  :style="check.is_within_tolerance
                    ? 'background: var(--accent-green); color: #fff'
                    : 'background: var(--accent-red); color: #fff'"
                >{{ check.is_within_tolerance ? '合格' : '超差' }}</span>
              </div>
              <div class="flex items-center gap-3 mt-1 text-[10px]" style="color: var(--text-muted)">
                <span>当前: {{ check.current_value.toFixed(3) }}</span>
                <span>阈值: {{ check.threshold.toFixed(3) }}</span>
                <span>偏差: {{ check.deviation_percent.toFixed(1) }}%</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 4: Adjustments -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            调整记录
          </h4>
          <div class="space-y-2">
            <div
              v-for="(adj, idx) in adjustments"
              :key="idx"
              class="p-2 rounded border"
              style="background: var(--bg-elevated); border-color: var(--border-default)"
            >
              <div class="text-xs font-medium" style="color: var(--text-primary)">{{ adj.parameter }}</div>
              <div class="flex items-center gap-2 mt-1 text-[10px]" style="color: var(--text-muted)">
                <span>{{ adj.node_id }}</span>
                <span>{{ adj.scale }}</span>
              </div>
              <div class="flex items-center gap-2 mt-1 text-[10px]">
                <span style="color: var(--accent-red)">{{ adj.old_value.toFixed(4) }}</span>
                <span style="color: var(--text-muted)">&rarr;</span>
                <span style="color: var(--accent-green)">{{ adj.new_value.toFixed(4) }}</span>
                <span class="px-1 py-0.5 rounded" style="background: var(--bg-base); color: var(--text-secondary)">{{ strategyLabel(adj.strategy) }}</span>
              </div>
              <div class="text-[10px] mt-1" style="color: var(--text-muted)">{{ adj.reason }}</div>
            </div>
          </div>
        </div>

        <!-- Step 5: Run Adaptive Loop -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            自适应循环
          </h4>
          <div class="space-y-2">
            <label class="flex items-center gap-2 text-xs cursor-pointer" style="color: var(--text-secondary)">
              <input type="checkbox" v-model="config.auto_apply" class="rounded" />
              自动应用调整
            </label>
            <button class="btn btn-primary text-xs w-full" @click="handleRunLoop">运行自适应循环</button>
            <!-- Iteration progress -->
            <div v-if="controlResult" class="p-2 rounded" style="background: var(--bg-elevated)">
              <div class="flex items-center justify-between text-[10px] mb-1" style="color: var(--text-muted)">
                <span>迭代进度</span>
                <span>{{ controlResult.iteration }} / {{ controlResult.max_iterations }}</span>
              </div>
              <div class="w-full h-2 rounded-full overflow-hidden" style="background: var(--border-subtle)">
                <div
                  class="h-full rounded-full transition-all"
                  :style="{
                    width: (controlResult.iteration / controlResult.max_iterations * 100) + '%',
                    background: controlResult.overall_status === 'converged' ? 'var(--accent-green)' : 'var(--primary)'
                  }"
                ></div>
              </div>
            </div>
          </div>
        </div>

      </div>

      <!-- Right Panel: Visualization & Results -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4" style="background: var(--bg-base)">

        <!-- Precision Convergence Chart -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">精度收敛曲线</h4>
          <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-default)">
            <svg width="100%" height="220" viewBox="0 0 600 220" preserveAspectRatio="xMidYMid meet">
              <!-- Grid lines -->
              <line v-for="i in 5" :key="'h'+i" :x1="50" :y1="20 + i * 36" x2="580" :y2="20 + i * 36" stroke="var(--border-subtle)" stroke-width="0.5" />
              <line v-for="i in 6" :key="'v'+i" :x1="50 + i * 88" :y1="20" :x2="50 + i * 88" :y2="200" stroke="var(--border-subtle)" stroke-width="0.5" />
              <!-- Axes -->
              <line x1="50" y1="200" x2="580" y2="200" stroke="var(--border-default)" stroke-width="1" />
              <line x1="50" y1="20" x2="50" y2="200" stroke="var(--border-default)" stroke-width="1" />
              <!-- Y-axis labels -->
              <text x="45" y="24" text-anchor="end" fill="var(--text-muted)" font-size="9">20%</text>
              <text x="45" y="96" text-anchor="end" fill="var(--text-muted)" font-size="9">10%</text>
              <text x="45" y="168" text-anchor="end" fill="var(--text-muted)" font-size="9">2%</text>
              <text x="45" y="204" text-anchor="end" fill="var(--text-muted)" font-size="9">0%</text>
              <!-- X-axis labels -->
              <text x="50" y="216" text-anchor="middle" fill="var(--text-muted)" font-size="9">0</text>
              <text x="138" y="216" text-anchor="middle" fill="var(--text-muted)" font-size="9">1</text>
              <text x="226" y="216" text-anchor="middle" fill="var(--text-muted)" font-size="9">2</text>
              <text x="314" y="216" text-anchor="middle" fill="var(--text-muted)" font-size="9">3</text>
              <text x="402" y="216" text-anchor="middle" fill="var(--text-muted)" font-size="9">4</text>
              <text x="490" y="216" text-anchor="middle" fill="var(--text-muted)" font-size="9">5</text>
              <!-- Axis titles -->
              <text x="315" y="216" text-anchor="middle" fill="var(--text-secondary)" font-size="10" dy="14">迭代次数</text>
              <text x="14" y="110" text-anchor="middle" fill="var(--text-secondary)" font-size="10" transform="rotate(-90, 14, 110)">总体偏差 (%)</text>
              <!-- Threshold line -->
              <line x1="50" y1="152" x2="580" y2="152" stroke="var(--accent-yellow)" stroke-width="1.5" stroke-dasharray="6,4" />
              <text x="584" y="155" fill="var(--accent-yellow)" font-size="9">阈值 5%</text>
              <!-- Convergence line -->
              <polyline
                :points="convergencePoints"
                fill="none"
                stroke="var(--primary)"
                stroke-width="2.5"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
              <!-- Data points -->
              <circle
                v-for="(pt, idx) in convergenceDataPoints"
                :key="idx"
                :cx="pt.x"
                :cy="pt.y"
                r="4"
                fill="var(--primary)"
                stroke="var(--bg-surface)"
                stroke-width="2"
              />
              <!-- Converged marker -->
              <circle v-if="controlResult && controlResult.overall_status === 'converged'" :cx="convergenceDataPoints[convergenceDataPoints.length - 1].x" :cy="convergenceDataPoints[convergenceDataPoints.length - 1].y" r="8" fill="none" stroke="var(--accent-green)" stroke-width="2" opacity="0.7">
                <animate attributeName="r" values="6;10;6" dur="2s" repeatCount="indefinite" />
                <animate attributeName="opacity" values="0.7;0.3;0.7" dur="2s" repeatCount="indefinite" />
              </circle>
            </svg>
          </div>
        </div>

        <!-- Metric Checks Visualization -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">指标检查可视化</h4>
          <div class="rounded-lg border p-4 space-y-3" style="background: var(--bg-surface); border-color: var(--border-default)">
            <div v-for="check in metricChecks" :key="check.metric_name" class="space-y-1">
              <div class="flex items-center justify-between">
                <span class="text-xs" style="color: var(--text-primary)">{{ check.metric_name }}</span>
                <div class="flex items-center gap-2 text-[10px]" style="color: var(--text-muted)">
                  <span>{{ check.current_value.toFixed(3) }} / {{ check.threshold.toFixed(3) }}</span>
                  <span
                    class="px-1 py-0.5 rounded font-medium"
                    :style="check.is_within_tolerance
                      ? 'background: rgba(34,197,94,0.15); color: var(--accent-green)'
                      : 'background: rgba(239,68,68,0.15); color: var(--accent-red)'"
                  >{{ check.deviation_percent.toFixed(1) }}%</span>
                </div>
              </div>
              <!-- Horizontal bar: threshold (background) vs current (foreground) -->
              <div class="relative w-full h-3 rounded-full overflow-hidden" style="background: var(--border-subtle)">
                <div
                  class="absolute top-0 left-0 h-full rounded-full transition-all"
                  :style="{
                    width: Math.min(check.current_value / (check.threshold * 1.5) * 100, 100) + '%',
                    background: check.is_within_tolerance ? 'var(--accent-green)' : 'var(--accent-red)'
                  }"
                ></div>
                <!-- Threshold marker -->
                <div
                  class="absolute top-0 h-full w-0.5"
                  :style="{
                    left: (check.threshold / (check.threshold * 1.5) * 100) + '%',
                    background: 'var(--accent-yellow)'
                  }"
                ></div>
              </div>
            </div>
          </div>
        </div>

        <!-- Adjustment History Timeline -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">调整历史时间线</h4>
          <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-default)">
            <div class="space-y-4">
              <div v-for="(iter, iterIdx) in iterationHistory" :key="iterIdx" class="flex gap-3">
                <!-- Timeline node -->
                <div class="flex flex-col items-center">
                  <div
                    class="w-6 h-6 rounded-full flex items-center justify-center text-[10px] font-bold shrink-0"
                    :style="iter.overall_status === 'converged'
                      ? 'background: var(--accent-green); color: #fff'
                      : 'background: var(--primary); color: #fff'"
                  >{{ iter.iteration }}</div>
                  <div v-if="iterIdx < iterationHistory.length - 1" class="w-0.5 flex-1 mt-1" style="background: var(--border-default)"></div>
                </div>
                <!-- Iteration details -->
                <div class="flex-1 pb-4">
                  <div class="flex items-center gap-2 mb-1">
                    <span class="text-xs font-medium" style="color: var(--text-primary)">迭代 {{ iter.iteration }}</span>
                    <span
                      class="px-1.5 py-0.5 rounded text-[9px]"
                      :style="statusBadgeStyle(iter.overall_status)"
                    >{{ statusLabel(iter.overall_status) }}</span>
                  </div>
                  <!-- Checks summary -->
                  <div class="text-[10px] mb-1" style="color: var(--text-muted)">
                    指标: {{ iter.checks.filter(c => c.is_within_tolerance).length }}/{{ iter.checks.length }} 合格
                  </div>
                  <!-- Adjustments in this iteration -->
                  <div v-if="iter.adjustments.length > 0" class="space-y-1">
                    <div
                      v-for="(adj, adjIdx) in iter.adjustments"
                      :key="adjIdx"
                      class="p-1.5 rounded text-[10px] flex items-center gap-2"
                      style="background: var(--bg-elevated)"
                    >
                      <span style="color: var(--text-secondary)">{{ adj.parameter }}</span>
                      <span style="color: var(--accent-red)">{{ adj.old_value.toFixed(4) }}</span>
                      <span style="color: var(--text-muted)">&rarr;</span>
                      <span style="color: var(--accent-green)">{{ adj.new_value.toFixed(4) }}</span>
                      <span class="px-1 py-0.5 rounded" style="background: var(--bg-base); color: var(--text-muted)">{{ strategyLabel(adj.strategy) }}</span>
                    </div>
                  </div>
                  <div v-else class="text-[10px]" style="color: var(--text-muted)">无调整</div>
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
import { ref, reactive, computed, onMounted } from 'vue'
import type { ApStrategy, ApMetricCheck, ApAdjustment, ApControlResult, ApControlConfig } from '../api/adaptivePrecision'
import type { ApOverallStatus } from '../api/adaptivePrecision'

// ============ Types & Constants ============

interface ExecutionOption {
  id: string
  label: string
}

interface StrategyOption {
  value: ApStrategy
  label: string
  desc: string
}

const executionOptions: ExecutionOption[] = [
  { id: 'exec-001', label: 'EXEC-001: 四尺度串联验证' },
  { id: 'exec-002', label: 'EXEC-002: 复合材料多尺度分析' },
  { id: 'exec-003', label: 'EXEC-003: 热力耦合自适应精度' },
]

const strategyOptions: StrategyOption[] = [
  { value: 'auto_refine', label: '自动细化', desc: '根据误差分布自动细化网格' },
  { value: 'increase_sampling', label: '增加采样', desc: '提高采样点密度以降低统计误差' },
  { value: 'adaptive_mesh', label: '自适应网格', desc: '基于梯度自适应调整网格分辨率' },
  { value: 'ensemble', label: '集成方法', desc: '多模型集成降低系统偏差' },
]

// ============ Reactive State ============

const config = reactive<ApControlConfig>({
  execution_id: '',
  enable: true,
  tolerance_percent: 5.0,
  max_iterations: 5,
  strategies: ['auto_refine', 'adaptive_mesh'],
  auto_apply: false,
})

const controlResult = ref<ApControlResult | null>(null)
const metricChecks = ref<ApMetricCheck[]>([])
const adjustments = ref<ApAdjustment[]>([])
const iterationHistory = ref<ApControlResult[]>([])

// ============ Computed ============

const convergenceDataPoints = computed(() => {
  const data = iterationHistory.value
  if (data.length === 0) return []
  const maxDeviation = 20
  const chartLeft = 50
  const chartRight = 580
  const chartTop = 20
  const chartBottom = 200
  const maxIter = Math.max(data.length - 1, 1)
  const xStep = (chartRight - chartLeft) / 5

  return data.map((result, idx) => {
    const overallDeviation = computeOverallDeviation(result.checks)
    const x = chartLeft + idx * xStep
    const y = chartBottom - (overallDeviation / maxDeviation) * (chartBottom - chartTop)
    return { x, y }
  })
})

const convergencePoints = computed(() => {
  const pts = convergenceDataPoints.value
  if (pts.length === 0) return ''
  return pts.map(p => `${p.x},${p.y}`).join(' ')
})

// ============ Mock Data ============

function generateMockMetricChecks(): ApMetricCheck[] {
  return [
    { metric_name: '应力误差 (宏观)', current_value: 3.21, threshold: 5.0, is_within_tolerance: true, deviation_percent: 2.8 },
    { metric_name: '能量收敛 (微观)', current_value: 4.87, threshold: 5.0, is_within_tolerance: true, deviation_percent: 4.2 },
    { metric_name: '相场梯度 (介观)', current_value: 7.53, threshold: 5.0, is_within_tolerance: false, deviation_percent: 7.1 },
    { metric_name: '带隙偏差 (量子)', current_value: 2.14, threshold: 5.0, is_within_tolerance: true, deviation_percent: 1.9 },
    { metric_name: '跨尺度一致性', current_value: 6.32, threshold: 5.0, is_within_tolerance: false, deviation_percent: 5.8 },
  ]
}

function generateMockAdjustments(): ApAdjustment[] {
  return [
    {
      node_id: 'node-mesh-012',
      scale: '介观 Phase Field',
      strategy: 'adaptive_mesh' as ApStrategy,
      parameter: '网格尺寸',
      old_value: 0.05,
      new_value: 0.025,
      reason: '相场梯度超差，细化网格以捕捉界面特征',
    },
    {
      node_id: 'node-sampling-008',
      scale: '微观 MD',
      strategy: 'increase_sampling' as ApStrategy,
      parameter: '采样步数',
      old_value: 50000,
      new_value: 100000,
      reason: '能量收敛接近阈值边界，增加采样以提高统计精度',
    },
    {
      node_id: 'node-coupling-003',
      scale: '跨尺度',
      strategy: 'auto_refine' as ApStrategy,
      parameter: '耦合容差',
      old_value: 0.01,
      new_value: 0.005,
      reason: '跨尺度一致性超差，收紧耦合容差',
    },
  ]
}

function generateMockIterationHistory(): ApControlResult[] {
  return [
    {
      execution_id: 'exec-001',
      iteration: 0,
      max_iterations: 5,
      overall_status: 'adjusting' as ApOverallStatus,
      checks: [
        { metric_name: '应力误差 (宏观)', current_value: 8.45, threshold: 5.0, is_within_tolerance: false, deviation_percent: 8.2 },
        { metric_name: '能量收敛 (微观)', current_value: 9.12, threshold: 5.0, is_within_tolerance: false, deviation_percent: 9.0 },
        { metric_name: '相场梯度 (介观)', current_value: 12.34, threshold: 5.0, is_within_tolerance: false, deviation_percent: 11.5 },
        { metric_name: '带隙偏差 (量子)', current_value: 3.87, threshold: 5.0, is_within_tolerance: true, deviation_percent: 3.4 },
        { metric_name: '跨尺度一致性', current_value: 11.56, threshold: 5.0, is_within_tolerance: false, deviation_percent: 10.8 },
      ],
      adjustments: [
        {
          node_id: 'node-mesh-012',
          scale: '介观 Phase Field',
          strategy: 'adaptive_mesh' as ApStrategy,
          parameter: '网格尺寸',
          old_value: 0.05,
          new_value: 0.025,
          reason: '相场梯度超差，细化网格以捕捉界面特征',
        },
        {
          node_id: 'node-sampling-008',
          scale: '微观 MD',
          strategy: 'increase_sampling' as ApStrategy,
          parameter: '采样步数',
          old_value: 50000,
          new_value: 100000,
          reason: '能量收敛超差，增加采样以提高统计精度',
        },
      ],
    },
    {
      execution_id: 'exec-001',
      iteration: 1,
      max_iterations: 5,
      overall_status: 'adjusting' as ApOverallStatus,
      checks: [
        { metric_name: '应力误差 (宏观)', current_value: 4.56, threshold: 5.0, is_within_tolerance: true, deviation_percent: 4.1 },
        { metric_name: '能量收敛 (微观)', current_value: 5.23, threshold: 5.0, is_within_tolerance: false, deviation_percent: 4.8 },
        { metric_name: '相场梯度 (介观)', current_value: 7.89, threshold: 5.0, is_within_tolerance: false, deviation_percent: 7.2 },
        { metric_name: '带隙偏差 (量子)', current_value: 2.45, threshold: 5.0, is_within_tolerance: true, deviation_percent: 2.1 },
        { metric_name: '跨尺度一致性', current_value: 8.12, threshold: 5.0, is_within_tolerance: false, deviation_percent: 7.6 },
      ],
      adjustments: [
        {
          node_id: 'node-coupling-003',
          scale: '跨尺度',
          strategy: 'auto_refine' as ApStrategy,
          parameter: '耦合容差',
          old_value: 0.01,
          new_value: 0.005,
          reason: '跨尺度一致性超差，收紧耦合容差',
        },
      ],
    },
    {
      execution_id: 'exec-001',
      iteration: 2,
      max_iterations: 5,
      overall_status: 'converged' as ApOverallStatus,
      checks: [
        { metric_name: '应力误差 (宏观)', current_value: 3.21, threshold: 5.0, is_within_tolerance: true, deviation_percent: 2.8 },
        { metric_name: '能量收敛 (微观)', current_value: 4.87, threshold: 5.0, is_within_tolerance: true, deviation_percent: 4.2 },
        { metric_name: '相场梯度 (介观)', current_value: 4.56, threshold: 5.0, is_within_tolerance: true, deviation_percent: 3.9 },
        { metric_name: '带隙偏差 (量子)', current_value: 2.14, threshold: 5.0, is_within_tolerance: true, deviation_percent: 1.9 },
        { metric_name: '跨尺度一致性', current_value: 4.78, threshold: 5.0, is_within_tolerance: true, deviation_percent: 4.5 },
      ],
      adjustments: [],
    },
  ]
}

// ============ Methods ============

function computeOverallDeviation(checks: ApMetricCheck[]): number {
  if (checks.length === 0) return 0
  const sum = checks.reduce((acc, c) => acc + c.deviation_percent, 0)
  return sum / checks.length
}

function strategyLabel(strategy: ApStrategy): string {
  const map: Record<ApStrategy, string> = {
    auto_refine: '自动细化',
    increase_sampling: '增加采样',
    adaptive_mesh: '自适应网格',
    ensemble: '集成方法',
  }
  return map[strategy] || strategy
}

function statusLabel(status: ApOverallStatus): string {
  const map: Record<ApOverallStatus, string> = {
    acceptable: '可接受',
    adjusting: '调整中',
    converged: '已收敛',
    failed: '失败',
  }
  return map[status] || status
}

function statusBadgeStyle(status: ApOverallStatus): string {
  switch (status) {
    case 'converged':
      return 'background: rgba(34,197,94,0.15); color: var(--accent-green)'
    case 'acceptable':
      return 'background: rgba(59,130,246,0.15); color: var(--primary)'
    case 'adjusting':
      return 'background: rgba(234,179,8,0.15); color: var(--accent-yellow)'
    case 'failed':
      return 'background: rgba(239,68,68,0.15); color: var(--accent-red)'
    default:
      return 'background: var(--bg-elevated); color: var(--text-muted)'
  }
}

function handleEvaluate() {
  // In production, call evaluatePrecision(config.execution_id)
  const checks = generateMockMetricChecks()
  metricChecks.value = checks
  adjustments.value = generateMockAdjustments()

  const withinCount = checks.filter(c => c.is_within_tolerance).length
  let status: ApOverallStatus = 'acceptable' as ApOverallStatus
  if (withinCount === checks.length) {
    status = 'converged' as ApOverallStatus
  } else if (withinCount === 0) {
    status = 'failed' as ApOverallStatus
  } else {
    status = 'adjusting' as ApOverallStatus
  }

  controlResult.value = {
    execution_id: config.execution_id,
    checks: checks,
    adjustments: adjustments.value,
    overall_status: status,
    iteration: 0,
    max_iterations: config.max_iterations,
  }
}

function handleRunLoop() {
  // In production, call runAdaptiveLoop(config)
  const history = generateMockIterationHistory()
  iterationHistory.value = history

  const lastResult = history[history.length - 1]
  if (lastResult) {
    controlResult.value = lastResult
    metricChecks.value = lastResult.checks
    adjustments.value = lastResult.adjustments
  }
}

function resetConfig() {
  config.execution_id = ''
  config.enable = true
  config.tolerance_percent = 5.0
  config.max_iterations = 5
  config.strategies = ['auto_refine', 'adaptive_mesh']
  config.auto_apply = false
  controlResult.value = null
  metricChecks.value = []
  adjustments.value = []
  iterationHistory.value = []
}

// ============ Lifecycle ============

onMounted(() => {
  handleEvaluate()
  handleRunLoop()
})
</script>
