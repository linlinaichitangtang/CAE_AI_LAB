<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">多尺度结果对比面板</h2>
        <p class="text-sm" style="color: var(--text-muted)">V2.0-006 | Mg-Al 蠕变多尺度计算结果一致性分析与对比</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="generateReport" class="btn btn-primary text-xs">生成对比</button>
        <button @click="exportData" class="btn btn-ghost text-xs">导出</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Execution Selector -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            执行选择
          </h4>
          <div>
            <label class="label">已完成执行</label>
            <select v-model="selectedExecutionId" class="input w-full text-xs">
              <option v-for="exec in completedExecutions" :key="exec.id" :value="exec.id">{{ exec.label }}</option>
            </select>
          </div>
        </div>

        <!-- Step 2: Scale Summaries -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            尺度结果摘要
          </h4>
          <div class="space-y-2">
            <div v-for="summary in report.scale_summaries" :key="summary.scale"
              class="rounded p-2 border" style="background: var(--bg-elevated); border-color: var(--border-default)">
              <div class="flex items-center justify-between mb-1">
                <span class="text-xs font-medium" :style="{ color: getScaleColor(summary.scale) }">{{ summary.scale }}</span>
                <span class="text-[10px] px-1.5 py-0.5 rounded"
                  :style="summary.status === 'completed'
                    ? 'background: var(--accent-green); color: white'
                    : 'background: var(--accent-yellow); color: white'">
                  {{ summary.status === 'completed' ? '完成' : '运行中' }}
                </span>
              </div>
              <div v-for="result in summary.key_results" :key="result.quantity"
                class="flex items-center justify-between text-[10px]" style="color: var(--text-secondary)">
                <span>{{ result.quantity }}</span>
                <span>{{ result.value }} {{ result.unit }}</span>
              </div>
              <div class="flex items-center justify-between text-[10px] mt-1 pt-1" style="border-top: 1px solid var(--border-subtle); color: var(--text-muted)">
                <span>耗时: {{ summary.computation_time_s.toFixed(1) }}s</span>
                <span>数据: {{ summary.data_size_mb.toFixed(1) }} MB</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 3: Comparison Metrics -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            对比指标
          </h4>
          <div class="space-y-2">
            <div v-for="metric in report.metrics" :key="metric.quantity"
              class="rounded p-2 border" style="background: var(--bg-elevated); border-color: var(--border-default)">
              <div class="flex items-center justify-between mb-1">
                <span class="text-xs font-medium" style="color: var(--text-primary)">{{ metric.quantity }}</span>
                <span class="text-[10px] px-1.5 py-0.5 rounded"
                  :style="metric.is_consistent
                    ? 'background: var(--accent-green); color: white'
                    : 'background: var(--accent-red); color: white'">
                  {{ metric.is_consistent ? '一致' : '偏差' }}
                </span>
              </div>
              <div v-for="val in metric.values" :key="val.scale"
                class="flex items-center justify-between text-[10px]" style="color: var(--text-secondary)">
                <span :style="{ color: getScaleColor(val.scale) }">{{ val.scale }}</span>
                <span>{{ val.value }} {{ val.unit }}</span>
              </div>
              <div class="text-[10px] mt-1 text-right" style="color: var(--text-muted)">
                最大偏差: {{ metric.max_deviation_percent.toFixed(1) }}%
              </div>
            </div>
          </div>
        </div>

        <!-- Step 4: Cross-Scale Links -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            跨尺度关联
          </h4>
          <div class="space-y-1">
            <div v-for="link in report.cross_scale_links" :key="link.source_scale + '-' + link.target_scale + '-' + link.source_quantity"
              class="rounded p-1.5 border" style="background: var(--bg-elevated); border-color: var(--border-default)">
              <div class="flex items-center gap-1 text-[10px]" style="color: var(--text-secondary)">
                <span :style="{ color: getScaleColor(link.source_scale) }">{{ link.source_scale }}</span>
                <span style="color: var(--text-muted)">→</span>
                <span :style="{ color: getScaleColor(link.target_scale) }">{{ link.target_scale }}</span>
                <span class="mx-1">|</span>
                <span>{{ link.source_quantity }} → {{ link.target_quantity }}</span>
              </div>
              <div class="flex items-center justify-between text-[10px] mt-0.5" style="color: var(--text-muted)">
                <span>{{ getLinkTypeLabel(link.link_type) }}</span>
                <span>{{ (link.confidence * 100).toFixed(0) }}%</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 5: Export -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            导出设置
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">导出格式</label>
              <select v-model="exportFormat" class="input w-full text-xs">
                <option value="csv">CSV</option>
                <option value="json">JSON</option>
                <option value="pdf">PDF</option>
              </select>
            </div>
            <button @click="exportData" class="btn btn-primary text-xs w-full">导出</button>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4">

        <!-- Top: Multi-Scale Comparison Chart -->
        <div class="rounded border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">多尺度对比图</h4>
          <svg viewBox="0 0 800 300" class="w-full" style="max-height: 300px">
            <defs>
              <linearGradient id="barGradDft" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stop-color="#3b82f6" stop-opacity="0.9" />
                <stop offset="100%" stop-color="#3b82f6" stop-opacity="0.6" />
              </linearGradient>
              <linearGradient id="barGradMd" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stop-color="#10b981" stop-opacity="0.9" />
                <stop offset="100%" stop-color="#10b981" stop-opacity="0.6" />
              </linearGradient>
              <linearGradient id="barGradPf" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stop-color="#f59e0b" stop-opacity="0.9" />
                <stop offset="100%" stop-color="#f59e0b" stop-opacity="0.6" />
              </linearGradient>
              <linearGradient id="barGradFe" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stop-color="#ef4444" stop-opacity="0.9" />
                <stop offset="100%" stop-color="#ef4444" stop-opacity="0.6" />
              </linearGradient>
            </defs>
            <!-- Y axis -->
            <line x1="60" y1="20" x2="60" y2="260" stroke="var(--border-default)" stroke-width="1" />
            <line x1="60" y1="260" x2="780" y2="260" stroke="var(--border-default)" stroke-width="1" />
            <!-- Y axis labels -->
            <text x="55" y="26" text-anchor="end" fill="var(--text-muted)" font-size="9">100%</text>
            <text x="55" y="88" text-anchor="end" fill="var(--text-muted)" font-size="9">75%</text>
            <text x="55" y="150" text-anchor="end" fill="var(--text-muted)" font-size="9">50%</text>
            <text x="55" y="212" text-anchor="end" fill="var(--text-muted)" font-size="9">25%</text>
            <text x="55" y="264" text-anchor="end" fill="var(--text-muted)" font-size="9">0%</text>
            <!-- Grid lines -->
            <line x1="60" y1="85" x2="780" y2="85" stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
            <line x1="60" y1="147" x2="780" y2="147" stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
            <line x1="60" y1="209" x2="780" y2="209" stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
            <!-- Bars for each metric -->
            <g v-for="(metric, midx) in report.metrics" :key="metric.quantity">
              <text :x="60 + midx * 120 + 50" :y="278" text-anchor="middle" fill="var(--text-secondary)" font-size="9" :transform="'rotate(-25, ' + (60 + midx * 120 + 50) + ', 278)'">{{ metric.quantity }}</text>
              <rect v-for="(val, vidx) in metric.values" :key="val.scale"
                :x="60 + midx * 120 + vidx * 25"
                :y="260 - (val.value / maxMetricValue * 240)"
                width="20"
                :height="val.value / maxMetricValue * 240"
                rx="2"
                :fill="getBarGradient(val.scale)" />
            </g>
            <!-- Legend -->
            <g transform="translate(620, 20)">
              <rect x="0" y="0" width="12" height="12" rx="2" fill="#3b82f6" />
              <text x="16" y="10" fill="var(--text-secondary)" font-size="10">DFT</text>
              <rect x="50" y="0" width="12" height="12" rx="2" fill="#10b981" />
              <text x="66" y="10" fill="var(--text-secondary)" font-size="10">MD</text>
              <rect x="90" y="0" width="12" height="12" rx="2" fill="#f59e0b" />
              <text x="106" y="10" fill="var(--text-secondary)" font-size="10">Phase-Field</text>
            </g>
          </svg>
        </div>

        <!-- Middle: Consistency Score Gauge -->
        <div class="rounded border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">整体一致性评分</h4>
          <div class="flex items-center justify-center">
            <svg viewBox="0 0 300 180" width="300" height="180">
              <defs>
                <linearGradient id="gaugeGrad" x1="0" y1="0" x2="1" y2="0">
                  <stop offset="0%" stop-color="var(--accent-red)" />
                  <stop offset="50%" stop-color="var(--accent-yellow)" />
                  <stop offset="100%" stop-color="var(--accent-green)" />
                </linearGradient>
              </defs>
              <!-- Background arc -->
              <path d="M 30 150 A 120 120 0 0 1 270 150" fill="none" stroke="var(--bg-elevated)" stroke-width="20" stroke-linecap="round" />
              <!-- Filled arc -->
              <path :d="'M 30 150 A 120 120 0 0 1 ' + getGaugeArcEnd(consistencyPercent) + ' 150'"
                fill="none" stroke="url(#gaugeGrad)" stroke-width="20" stroke-linecap="round" />
              <!-- Needle -->
              <line x1="150" y1="150" :x2="getNeedleX(consistencyPercent)" :y2="getNeedleY(consistencyPercent)"
                stroke="var(--text-primary)" stroke-width="2" />
              <circle cx="150" cy="150" r="6" fill="var(--primary)" />
              <!-- Score text -->
              <text x="150" y="135" text-anchor="middle" :fill="getConsistencyColor(consistencyPercent)" font-size="28" font-weight="bold">{{ consistencyPercent }}%</text>
              <text x="150" y="155" text-anchor="middle" fill="var(--text-muted)" font-size="10">一致性得分</text>
              <!-- Scale labels -->
              <text x="25" y="170" text-anchor="middle" fill="var(--text-muted)" font-size="9">0</text>
              <text x="150" y="25" text-anchor="middle" fill="var(--text-muted)" font-size="9">50</text>
              <text x="275" y="170" text-anchor="middle" fill="var(--text-muted)" font-size="9">100</text>
              <!-- Warnings -->
              <g v-if="report.warnings.length > 0">
                <text x="150" y="178" text-anchor="middle" fill="var(--accent-yellow)" font-size="8">
                  {{ report.warnings.length }} 个警告
                </text>
              </g>
            </svg>
          </div>
          <div v-if="report.warnings.length > 0" class="mt-2 space-y-1">
            <div v-for="(warning, widx) in report.warnings" :key="widx"
              class="text-[10px] px-2 py-1 rounded" style="background: rgba(245,158,11,0.1); color: var(--accent-yellow)">
              {{ warning }}
            </div>
          </div>
        </div>

        <!-- Bottom: Drill-Down Section -->
        <div class="rounded border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">穿透查看</h4>
          <div class="flex items-end gap-2 mb-4">
            <div>
              <label class="label">源尺度</label>
              <select v-model="drillDownSource.scale" class="input w-full text-xs">
                <option value="DFT">DFT</option>
                <option value="MD">MD</option>
                <option value="Phase-Field">Phase-Field</option>
                <option value="FE">FE</option>
              </select>
            </div>
            <div>
              <label class="label">源物理量</label>
              <select v-model="drillDownSource.quantity" class="input w-full text-xs">
                <option v-for="metric in report.metrics" :key="metric.quantity" :value="metric.quantity">{{ metric.quantity }}</option>
              </select>
            </div>
            <div>
              <label class="label">目标尺度</label>
              <select v-model="drillDownSource.target_scale" class="input w-full text-xs">
                <option value="DFT">DFT</option>
                <option value="MD">MD</option>
                <option value="Phase-Field">Phase-Field</option>
                <option value="FE">FE</option>
              </select>
            </div>
            <button @click="runDrillDown" class="btn btn-primary text-xs">穿透查看</button>
          </div>
          <div v-if="drillDownResult" class="rounded p-3" style="background: var(--bg-elevated)">
            <div class="text-xs font-medium mb-2" style="color: var(--text-primary)">
              {{ drillDownSource.scale }} / {{ drillDownSource.quantity }} → {{ drillDownSource.target_scale }}
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div class="rounded p-2" style="background: var(--bg-base)">
                <div class="text-[10px]" style="color: var(--text-muted)">目标数据</div>
                <div v-for="(value, key) in drillDownResult.target_data" :key="key"
                  class="text-[10px]" style="color: var(--text-secondary)">
                  {{ key }}: {{ String(value) }}
                </div>
              </div>
              <div class="rounded p-2" style="background: var(--bg-base)">
                <div class="text-[10px]" style="color: var(--text-muted)">关联节点</div>
                <div v-for="node in drillDownResult.related_nodes" :key="node"
                  class="text-[10px]" style="color: var(--text-secondary)">
                  {{ node }}
                </div>
              </div>
            </div>
          </div>
          <div v-else class="text-center py-4 text-xs" style="color: var(--text-muted)">选择源尺度和物理量后点击"穿透查看"</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type { ComparisonMetric, ScaleResultSummary, CrossScaleLink, ComparisonReport, DrillDownRequest } from '../api/resultComparison'

// ============ Types ============

type RcLinkType = 'direct' | 'derived' | 'validated'
type RcExportFormat = 'csv' | 'json' | 'pdf'

// ============ State ============

const selectedExecutionId = ref('exec-001')
const exportFormat = ref<RcExportFormat>('json')

const drillDownSource = reactive({
  scale: 'DFT',
  quantity: '',
  target_scale: 'MD'
})

const drillDownResult = ref<{ target_data: Record<string, unknown>; related_nodes: string[] } | null>(null)

const completedExecutions = ref([
  { id: 'exec-001', label: 'Mg-Al 蠕变 - 完整多尺度 (2026-04-02)' },
  { id: 'exec-002', label: 'Ni基合金 - 蠕变分析 (2026-04-01)' },
  { id: 'exec-003', label: 'Al-Si - 凝固过程 (2026-03-30)' }
])

const report = reactive<ComparisonReport>({
  execution_id: 'exec-001',
  scale_summaries: [],
  metrics: [],
  cross_scale_links: [],
  consistency_score: 0.87,
  warnings: [],
  generated_at: ''
})

// ============ Computed ============

const consistencyPercent = computed(() => Math.round(report.consistency_score * 100))

const maxMetricValue = computed(() => {
  let maxVal = 0
  for (const metric of report.metrics) {
    for (const val of metric.values) {
      if (val.value > maxVal) maxVal = val.value
    }
  }
  return maxVal > 0 ? maxVal : 100
})

// ============ Mock Data ============

function generateMockReport(): ComparisonReport {
  return {
    execution_id: 'exec-001',
    scale_summaries: [
      {
        scale: 'DFT',
        node_id: 'node_dft_001',
        status: 'completed',
        key_results: [
          { quantity: 'C11', value: 62.5, unit: 'GPa' },
          { quantity: 'C12', value: 25.3, unit: 'GPa' },
          { quantity: 'C44', value: 18.7, unit: 'GPa' }
        ],
        computation_time_s: 1245.6,
        data_size_mb: 12.3
      },
      {
        scale: 'MD',
        node_id: 'node_md_001',
        status: 'completed',
        key_results: [
          { quantity: 'diffusivity', value: 2.34e-13, unit: 'm2/s' },
          { quantity: 'creep_rate', value: 1.56e-8, unit: '1/s' },
          { quantity: 'avg_stress', value: 312.5, unit: 'MPa' }
        ],
        computation_time_s: 4520.8,
        data_size_mb: 256.7
      },
      {
        scale: 'Phase-Field',
        node_id: 'node_pf_001',
        status: 'completed',
        key_results: [
          { quantity: 'precipitate_radius', value: 15.2, unit: 'nm' },
          { quantity: 'volume_fraction', value: 0.045, unit: '-' },
          { quantity: 'interface_width', value: 2.8, unit: 'nm' }
        ],
        computation_time_s: 8920.3,
        data_size_mb: 1024.5
      },
      {
        scale: 'FE',
        node_id: 'node_fe_001',
        status: 'completed',
        key_results: [
          { quantity: 'max_stress', value: 285.4, unit: 'MPa' },
          { quantity: 'creep_strain', value: 0.0032, unit: '-' },
          { quantity: 'rupture_time', value: 482.5, unit: 'h' }
        ],
        computation_time_s: 3200.1,
        data_size_mb: 45.8
      }
    ],
    metrics: [
      {
        quantity: '杨氏模量',
        values: [
          { scale: 'DFT', value: 45.2, unit: 'GPa', uncertainty: 1.2 },
          { scale: 'MD', value: 43.8, unit: 'GPa', uncertainty: 2.1 },
          { scale: 'Phase-Field', value: 44.5, unit: 'GPa', uncertainty: 3.5 },
          { scale: 'FE', value: 42.1, unit: 'GPa', uncertainty: 1.8 }
        ],
        is_consistent: true,
        max_deviation_percent: 7.3
      },
      {
        quantity: '扩散系数',
        values: [
          { scale: 'DFT', value: 2.10, unit: 'e-13 m2/s', uncertainty: 0.15 },
          { scale: 'MD', value: 2.34, unit: 'e-13 m2/s', uncertainty: 0.28 },
          { scale: 'Phase-Field', value: 2.52, unit: 'e-13 m2/s', uncertainty: 0.35 },
          { scale: 'FE', value: 2.80, unit: 'e-13 m2/s', uncertainty: 0.42 }
        ],
        is_consistent: false,
        max_deviation_percent: 33.3
      },
      {
        quantity: '界面能',
        values: [
          { scale: 'DFT', value: 0.32, unit: 'J/m2', uncertainty: 0.02 },
          { scale: 'MD', value: 0.35, unit: 'J/m2', uncertainty: 0.04 },
          { scale: 'Phase-Field', value: 0.30, unit: 'J/m2', uncertainty: 0.05 },
          { scale: 'FE', value: 0.28, unit: 'J/m2', uncertainty: 0.03 }
        ],
        is_consistent: true,
        max_deviation_percent: 25.0
      },
      {
        quantity: '蠕变速率',
        values: [
          { scale: 'DFT', value: 1.20, unit: 'e-8 1/s', uncertainty: 0.15 },
          { scale: 'MD', value: 1.56, unit: 'e-8 1/s', uncertainty: 0.22 },
          { scale: 'Phase-Field', value: 1.80, unit: 'e-8 1/s', uncertainty: 0.30 },
          { scale: 'FE', value: 2.10, unit: 'e-8 1/s', uncertainty: 0.18 }
        ],
        is_consistent: false,
        max_deviation_percent: 75.0
      },
      {
        quantity: '晶格常数',
        values: [
          { scale: 'DFT', value: 3.21, unit: 'A', uncertainty: 0.005 },
          { scale: 'MD', value: 3.20, unit: 'A', uncertainty: 0.008 },
          { scale: 'Phase-Field', value: 3.22, unit: 'A', uncertainty: 0.01 },
          { scale: 'FE', value: 3.20, unit: 'A', uncertainty: 0.006 }
        ],
        is_consistent: true,
        max_deviation_percent: 0.6
      },
      {
        quantity: '屈服强度',
        values: [
          { scale: 'DFT', value: 285, unit: 'MPa', uncertainty: 12 },
          { scale: 'MD', value: 278, unit: 'MPa', uncertainty: 18 },
          { scale: 'Phase-Field', value: 295, unit: 'MPa', uncertainty: 25 },
          { scale: 'FE', value: 270, unit: 'MPa', uncertainty: 15 }
        ],
        is_consistent: true,
        max_deviation_percent: 9.3
      }
    ],
    cross_scale_links: [
      { source_scale: 'DFT', target_scale: 'MD', source_quantity: 'C11', target_quantity: 'eam_epsilon', link_type: 'direct' as RcLinkType, confidence: 0.92 },
      { source_scale: 'DFT', target_scale: 'MD', source_quantity: 'formation_energy', target_quantity: 'lattice_constant', link_type: 'derived' as RcLinkType, confidence: 0.78 },
      { source_scale: 'MD', target_scale: 'Phase-Field', source_quantity: 'diffusivity', target_quantity: 'mobility', link_type: 'direct' as RcLinkType, confidence: 0.86 },
      { source_scale: 'MD', target_scale: 'Phase-Field', source_quantity: 'potential_energy', target_quantity: 'driving_force', link_type: 'derived' as RcLinkType, confidence: 0.72 },
      { source_scale: 'Phase-Field', target_scale: 'FE', source_quantity: 'gradient_coefficient', target_quantity: 'yield_stress', link_type: 'validated' as RcLinkType, confidence: 0.88 },
      { source_scale: 'Phase-Field', target_scale: 'FE', source_quantity: 'interface_energy', target_quantity: 'hardening_law', link_type: 'derived' as RcLinkType, confidence: 0.65 },
      { source_scale: 'DFT', target_scale: 'FE', source_quantity: 'C11', target_quantity: 'youngs_modulus', link_type: 'validated' as RcLinkType, confidence: 0.95 }
    ],
    consistency_score: 0.87,
    warnings: [
      '扩散系数在FE尺度偏差较大 (33.3%)，建议检查边界条件',
      '蠕变速率跨尺度偏差显著 (75.0%)，可能需要优化映射参数'
    ],
    generated_at: '2026-04-03T14:30:00Z'
  }
}

// ============ Helpers ============

function getScaleColor(scale: string): string {
  const map: Record<string, string> = {
    'DFT': '#3b82f6',
    'MD': '#10b981',
    'Phase-Field': '#f59e0b',
    'FE': '#ef4444'
  }
  return map[scale] || 'var(--text-secondary)'
}

function getBarGradient(scale: string): string {
  const map: Record<string, string> = {
    'DFT': 'url(#barGradDft)',
    'MD': 'url(#barGradMd)',
    'Phase-Field': 'url(#barGradPf)',
    'FE': 'url(#barGradFe)'
  }
  return map[scale] || '#6b7280'
}

function getLinkTypeLabel(linkType: RcLinkType): string {
  const map: Record<RcLinkType, string> = {
    direct: '直接关联',
    derived: '推导关联',
    validated: '已验证'
  }
  return map[linkType] || linkType
}

function getConsistencyColor(percent: number): string {
  if (percent >= 80) return 'var(--accent-green)'
  if (percent >= 60) return 'var(--accent-yellow)'
  return 'var(--accent-red)'
}

function getGaugeArcEnd(percent: number): string {
  const angle = (percent / 100) * Math.PI
  const x = 150 + 120 * Math.cos(Math.PI - angle)
  return String(x)
}

function getNeedleX(percent: number): number {
  const angle = (percent / 100) * Math.PI
  return 150 + 100 * Math.cos(Math.PI - angle)
}

function getNeedleY(percent: number): number {
  const angle = (percent / 100) * Math.PI
  return 150 - 100 * Math.sin(angle)
}

// ============ Actions ============

function generateReport() {
  const mockReport = generateMockReport()
  mockReport.execution_id = selectedExecutionId.value
  mockReport.generated_at = new Date().toISOString()
  report.execution_id = mockReport.execution_id
  report.scale_summaries = mockReport.scale_summaries
  report.metrics = mockReport.metrics
  report.cross_scale_links = mockReport.cross_scale_links
  report.consistency_score = mockReport.consistency_score
  report.warnings = mockReport.warnings
  report.generated_at = mockReport.generated_at
  drillDownResult.value = null
}

function runDrillDown() {
  if (!drillDownSource.quantity) return
  const request: DrillDownRequest = {
    source_scale: drillDownSource.scale,
    source_quantity: drillDownSource.quantity,
    source_value: null,
    target_scale: drillDownSource.target_scale
  }
  drillDownResult.value = {
    target_data: {
      value: 43.8,
      unit: 'GPa',
      uncertainty: 2.1,
      source_node: 'node_' + drillDownSource.scale.toLowerCase() + '_001',
      mapping_rule: 'pm-' + String(Math.floor(Math.random() * 12) + 1).padStart(3, '0'),
      transfer_error: 0.035
    },
    related_nodes: [
      'node_' + drillDownSource.scale.toLowerCase() + '_001',
      'node_' + drillDownSource.target_scale.toLowerCase() + '_001',
      'node_mapping_' + drillDownSource.scale.toLowerCase() + '_to_' + drillDownSource.target_scale.toLowerCase()
    ]
  }
  console.log('Drill down request:', request)
}

function exportData() {
  const format = exportFormat.value
  const data = JSON.stringify(report, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'comparison_report.' + format
  a.click()
  URL.revokeObjectURL(url)
}

// ============ Lifecycle ============

onMounted(() => {
  const mockReport = generateMockReport()
  report.execution_id = mockReport.execution_id
  report.scale_summaries = mockReport.scale_summaries
  report.metrics = mockReport.metrics
  report.cross_scale_links = mockReport.cross_scale_links
  report.consistency_score = mockReport.consistency_score
  report.warnings = mockReport.warnings
  report.generated_at = mockReport.generated_at

  if (report.metrics.length > 0) {
    drillDownSource.quantity = report.metrics[0].quantity
  }

  const score = report.consistency_score
  const metricCount = report.metrics.length
  nextTick(() => {
    console.log('Comparison report loaded: consistency', Math.round(score * 100) + '%,', metricCount, 'metrics')
  })
})
</script>
