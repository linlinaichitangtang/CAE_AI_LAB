<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-lg font-semibold text-[var(--text-primary)]">高通量参数扫描</h2>
        <p class="text-sm text-[var(--text-muted)]">V1.9-005 DOE实验设计 / V1.9-006 高通量筛选结果数据库</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="resetAll" class="btn btn-ghost text-xs">重置</button>
        <button v-if="scanResults.length > 0" @click="exportData('csv')" class="btn btn-ghost text-xs">导出 CSV</button>
        <button v-if="scanResults.length > 0" @click="exportData('parquet')" class="btn btn-ghost text-xs">导出 Parquet</button>
        <button v-if="scanResults.length > 0" @click="exportData('json')" class="btn btn-ghost text-xs">导出 JSON</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration (w-80) -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] overflow-y-auto p-4 space-y-4">

        <!-- Step 1: DOE 方法选择 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">1</span>
            DOE 实验设计方法
          </h4>
          <div class="grid grid-cols-2 gap-1.5">
            <button
              v-for="method in doeMethods"
              :key="method.value"
              @click="scanConfig.doe_method = method.value"
              :class="['px-2 py-2 rounded text-xs text-left transition border',
                scanConfig.doe_method === method.value
                  ? 'bg-blue-600 text-white border-blue-600'
                  : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)] border-[var(--border-default)] hover:bg-[var(--bg-hover)]']"
            >
              <div class="font-medium">{{ method.label }}</div>
              <div class="text-[10px] mt-0.5 opacity-80">{{ method.desc }}</div>
            </button>
          </div>
        </div>

        <!-- Step 2: 参数范围 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">2</span>
            参数范围定义
          </h4>
          <div class="space-y-2">
            <div v-for="(param, idx) in paramRanges" :key="idx" class="p-2 rounded border border-[var(--border-subtle)] bg-[var(--bg-elevated)]">
              <div class="flex items-center justify-between mb-1.5">
                <span class="text-xs font-medium text-[var(--text-primary)]">参数 {{ idx + 1 }}</span>
                <button @click="removeParam(idx)" class="text-[var(--accent-red)] text-xs hover:underline">移除</button>
              </div>
              <div class="space-y-1.5">
                <div>
                  <label class="label">参数名</label>
                  <input v-model="param.name" type="text" class="input w-full text-xs" placeholder="e.g. temperature" />
                </div>
                <div class="grid grid-cols-3 gap-1.5">
                  <div>
                    <label class="label">最小值</label>
                    <input v-model.number="param.min" type="number" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">最大值</label>
                    <input v-model.number="param.max" type="number" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">步长</label>
                    <input v-model.number="param.step" type="number" step="0.1" class="input w-full text-xs" />
                  </div>
                </div>
                <div>
                  <label class="label">类型</label>
                  <select v-model="param.type" class="input w-full text-xs">
                    <option value="continuous">连续</option>
                    <option value="discrete">离散</option>
                  </select>
                </div>
              </div>
            </div>
            <button @click="addParam" class="btn btn-ghost text-xs w-full" style="border-style: dashed;">
              + 添加参数
            </button>
          </div>
        </div>

        <!-- Step 3: 扫描设置 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">3</span>
            扫描设置
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">最大并行任务数</label>
              <input v-model.number="scanConfig.max_parallel" type="number" min="1" max="32" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">单次运行超时 (秒)</label>
              <input v-model.number="scanConfig.timeout_per_run_s" type="number" min="10" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">模板 ID (可选)</label>
              <input v-model="scanConfig.template_id" type="text" class="input w-full text-xs" placeholder="留空使用默认模板" />
            </div>
          </div>
        </div>

        <!-- Step 4: 创建扫描 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">4</span>
            扫描执行
          </h4>
          <button @click="createScan" :disabled="scanning" class="btn btn-primary text-xs w-full">
            {{ scanning ? '扫描中...' : '创建扫描' }}
          </button>
          <div v-if="scanTask" class="mt-3 space-y-2">
            <div class="flex items-center justify-between text-xs text-[var(--text-secondary)]">
              <span>进度</span>
              <span>{{ scanProgress.completed }} / {{ scanProgress.total }}</span>
            </div>
            <div class="w-full h-2 rounded-full bg-[var(--bg-elevated)] overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-300"
                :style="{ width: scanProgressPercent + '%', background: 'var(--primary)' }"
              ></div>
            </div>
            <div class="grid grid-cols-3 gap-2 text-xs text-center">
              <div class="p-1.5 rounded bg-[var(--bg-elevated)]">
                <div class="text-[var(--accent-green)] font-medium">{{ scanProgress.completed }}</div>
                <div class="text-[var(--text-muted)]">完成</div>
              </div>
              <div class="p-1.5 rounded bg-[var(--bg-elevated)]">
                <div class="text-[var(--accent-red)] font-medium">{{ scanProgress.failed }}</div>
                <div class="text-[var(--text-muted)]">失败</div>
              </div>
              <div class="p-1.5 rounded bg-[var(--bg-elevated)]">
                <div class="text-[var(--primary)] font-medium">{{ scanProgress.total }}</div>
                <div class="text-[var(--text-muted)]">总计</div>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 5: 结果筛选 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">5</span>
            结果查询
          </h4>
          <div class="space-y-2">
            <div v-for="(filter, idx) in queryConfig.filters" :key="idx" class="flex items-center gap-1.5">
              <input v-model="filter.param_name" type="text" class="input w-full text-xs" placeholder="参数名" />
              <input v-model.number="filter.min" type="number" class="input text-xs" style="width: 70px;" placeholder="最小值" />
              <input v-model.number="filter.max" type="number" class="input text-xs" style="width: 70px;" placeholder="最大值" />
              <button @click="removeFilter(idx)" class="text-[var(--accent-red)] text-xs px-1">x</button>
            </div>
            <button @click="addFilter" class="btn btn-ghost text-xs w-full" style="border-style: dashed;">
              + 添加筛选条件
            </button>
            <button @click="applyQuery" class="btn btn-ghost text-xs w-full">应用筛选</button>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization / Results -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <div class="flex-1 overflow-y-auto p-4 space-y-4">

          <!-- 并行坐标可视化 -->
          <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-4">
            <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">并行坐标 - 参数组合可视化</h4>
            <svg viewBox="0 0 700 280" class="w-full" style="max-height: 280px;">
              <!-- 背景网格 -->
              <line v-for="i in 5" :key="'bg'+i" :x1="80 + i * 130" :y1="20" :x2="80 + i * 130" :y2="240" stroke="var(--border-subtle)" stroke-width="0.5" />
              <!-- 坐标轴标签 -->
              <text v-for="(axis, ai) in parallelAxes" :key="'al'+ai" :x="80 + ai * 130" y="265" text-anchor="middle" fill="var(--text-muted)" font-size="10">{{ axis.label }}</text>
              <!-- 刻度 -->
              <text v-for="(axis, ai) in parallelAxes" :key="'at'+ai" :x="80 + ai * 130 - 12" y="24" text-anchor="end" fill="var(--text-muted)" font-size="8">{{ axis.max }}</text>
              <text v-for="(axis, ai) in parallelAxes" :key="'ab'+ai" :x="80 + ai * 130 - 12" y="244" text-anchor="end" fill="var(--text-muted)" font-size="8">{{ axis.min }}</text>
              <!-- 参数线 -->
              <polyline
                v-for="(line, li) in parallelLines"
                :key="'pl'+li"
                :points="line.points"
                fill="none"
                :stroke="line.color"
                stroke-width="1.5"
                opacity="0.7"
              />
              <!-- 图例 -->
              <circle cx="80" cy="12" r="4" fill="var(--accent-green)" />
              <text x="88" y="15" fill="var(--text-muted)" font-size="9">最优</text>
              <circle cx="130" cy="12" r="4" fill="var(--primary)" />
              <text x="138" y="15" fill="var(--text-muted)" font-size="9">良好</text>
              <circle cx="180" cy="12" r="4" fill="var(--accent-yellow)" />
              <text x="188" y="15" fill="var(--text-muted)" font-size="9">一般</text>
              <circle cx="230" cy="12" r="4" fill="var(--accent-red)" />
              <text x="238" y="15" fill="var(--text-muted)" font-size="9">较差</text>
            </svg>
          </div>

          <!-- 热力图 -->
          <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-4">
            <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">参数热力图 (参数1 vs 参数2 → 结果值)</h4>
            <svg viewBox="0 0 500 320" class="w-full" style="max-height: 320px;">
              <!-- 热力图单元格 -->
              <rect
                v-for="(cell, ci) in heatmapCells"
                :key="'hc'+ci"
                :x="cell.x"
                :y="cell.y"
                :width="cell.w"
                :height="cell.h"
                :fill="cell.color"
                stroke="var(--bg-base)"
                stroke-width="0.5"
              />
              <!-- X 轴标签 -->
              <text v-for="(xt, xi) in heatmapXLabels" :key="'hx'+xi" :x="xt.x" :y="305" text-anchor="middle" fill="var(--text-muted)" font-size="9">{{ xt.label }}</text>
              <!-- Y 轴标签 -->
              <text v-for="(yt, yi) in heatmapYLabels" :key="'hy'+yi" :x="25" :y="yt.y" text-anchor="end" fill="var(--text-muted)" font-size="9">{{ yt.label }}</text>
              <!-- 轴标题 -->
              <text x="250" y="318" text-anchor="middle" fill="var(--text-muted)" font-size="10">温度 (K)</text>
              <text x="8" y="160" text-anchor="middle" fill="var(--text-muted)" font-size="10" transform="rotate(-90, 8, 160)">应力 (MPa)</text>
              <!-- 色标 -->
              <defs>
                <linearGradient id="heatGrad" x1="0" y1="0" x2="1" y2="0">
                  <stop offset="0%" stop-color="#2563eb" />
                  <stop offset="50%" stop-color="#eab308" />
                  <stop offset="100%" stop-color="#dc2626" />
                </linearGradient>
              </defs>
              <rect x="400" y="30" width="80" height="12" fill="url(#heatGrad)" rx="2" />
              <text x="400" y="26" fill="var(--text-muted)" font-size="8">低</text>
              <text x="470" y="26" fill="var(--text-muted)" font-size="8">高</text>
              <text x="440" y="55" text-anchor="middle" fill="var(--text-muted)" font-size="8">蠕变应变率</text>
            </svg>
          </div>

          <!-- 统计卡片 -->
          <div class="grid grid-cols-3 gap-3">
            <!-- 最优参数 -->
            <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-3">
              <h4 class="text-xs font-medium mb-2 text-[var(--text-primary)]">最优参数组合</h4>
              <div class="space-y-1.5">
                <div v-for="(opt, oi) in optimalParams" :key="'op'+oi" class="flex items-center justify-between text-xs">
                  <span class="text-[var(--text-secondary)]">{{ opt.name }}</span>
                  <span class="font-mono text-[var(--accent-green)]">{{ opt.value }}</span>
                </div>
                <div class="pt-1.5 border-t border-[var(--border-subtle)]">
                  <div class="flex items-center justify-between text-xs">
                    <span class="text-[var(--text-secondary)]">目标值</span>
                    <span class="font-mono text-[var(--accent-green)] font-medium">{{ optimalResultValue }}</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- 相关性 -->
            <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-3">
              <h4 class="text-xs font-medium mb-2 text-[var(--text-primary)]">参数相关性</h4>
              <div class="space-y-1.5">
                <div v-for="(corr, ci) in correlations" :key="'cr'+ci" class="flex items-center justify-between text-xs">
                  <span class="text-[var(--text-secondary)]">{{ corr.name }}</span>
                  <div class="flex items-center gap-1.5">
                    <div class="w-16 h-1.5 rounded-full bg-[var(--bg-elevated)] overflow-hidden">
                      <div
                        class="h-full rounded-full"
                        :style="{ width: Math.abs(corr.value) * 100 + '%', background: corr.value > 0 ? 'var(--accent-green)' : 'var(--accent-red)' }"
                      ></div>
                    </div>
                    <span class="font-mono w-10 text-right" :style="{ color: corr.value > 0 ? 'var(--accent-green)' : 'var(--accent-red)' }">{{ corr.value.toFixed(2) }}</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- 灵敏度排名 -->
            <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-3">
              <h4 class="text-xs font-medium mb-2 text-[var(--text-primary)]">灵敏度排名</h4>
              <div class="space-y-1.5">
                <div v-for="(sens, si) in sensitivityRankings" :key="'sr'+si" class="flex items-center gap-2 text-xs">
                  <span class="w-4 h-4 rounded-full flex items-center justify-center text-[10px] font-medium"
                    :style="{ background: si === 0 ? 'var(--accent-yellow)' : 'var(--bg-elevated)', color: si === 0 ? '#000' : 'var(--text-secondary)' }">
                    {{ si + 1 }}
                  </span>
                  <span class="text-[var(--text-secondary)] flex-1">{{ sens.name }}</span>
                  <div class="w-20 h-1.5 rounded-full bg-[var(--bg-elevated)] overflow-hidden">
                    <div class="h-full rounded-full bg-[var(--primary)]" :style="{ width: sens.score + '%' }"></div>
                  </div>
                  <span class="font-mono text-[var(--text-primary)] w-10 text-right">{{ sens.score.toFixed(1) }}%</span>
                </div>
              </div>
            </div>
          </div>

          <!-- 筛选结果表格 -->
          <div v-if="flatRows.length > 0" class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-4">
            <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">筛选结果 ({{ flatRows.length }} 条)</h4>
            <div class="overflow-x-auto">
              <table class="w-full text-xs">
                <thead>
                  <tr class="border-b border-[var(--border-subtle)]">
                    <th v-for="col in resultColumns" :key="col.key" class="text-left py-1.5 px-2 text-[var(--text-muted)] font-medium">{{ col.label }}</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="(row, ri) in flatRows.slice(0, 20)" :key="'rr'+ri" class="border-b border-[var(--border-subtle)]">
                    <td v-for="col in resultColumns" :key="col.key+'-'+ri" class="py-1.5 px-2 text-[var(--text-secondary)]">
                      <span :class="{ 'text-[var(--accent-green)] font-medium': col.key === 'result_value' && row[col.key as keyof typeof row] === minResultValue }">
                        {{ typeof row[col.key as keyof typeof row] === 'number' ? (row[col.key as keyof typeof row] as number).toFixed(4) : row[col.key as keyof typeof row] }}
                      </span>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import type { DoEMethod, ParamRange, ScanConfig, ScanResult, ScanTask, ScanProgress, QueryConfig } from '../api/highThroughput'

// ============ DOE 方法 ============
const doeMethods = ref([
  { value: 'full_factorial' as DoEMethod, label: '全因子', desc: '所有组合穷举' },
  { value: 'latin_hypercube' as DoEMethod, label: 'Latin Hypercube', desc: '分层随机采样' },
  { value: 'sobol' as DoEMethod, label: 'Sobol', desc: '准蒙特卡洛序列' },
  { value: 'random' as DoEMethod, label: '随机', desc: '均匀随机采样' },
  { value: 'central_composite' as DoEMethod, label: '中心复合', desc: 'RSM响应面设计' },
])

// ============ 参数范围 ============
const paramRanges = ref<ParamRange[]>([
  { name: 'temperature', min: 600, max: 1000, step: 50, type: 'continuous' },
  { name: 'stress', min: 50, max: 300, step: 25, type: 'continuous' },
  { name: 'grain_size', min: 10, max: 100, step: 10, type: 'discrete' },
])

function addParam() {
  paramRanges.value.push({ name: '', min: 0, max: 1, step: 0.1, type: 'continuous' })
}

function removeParam(idx: number) {
  paramRanges.value.splice(idx, 1)
}

// ============ 扫描配置 ============
const scanConfig = reactive<ScanConfig>({
  name: '',
  doe_method: 'latin_hypercube',
  parameters: [],
  total_combinations: 0,
  max_parallel: 4,
  timeout_per_run_s: 300,
  template_id: '',
})

// ============ 扫描状态 ============
const scanning = ref(false)
const scanTask = ref<ScanTask | null>(null)

/** 从 scanTask 的上下文获取进度（模拟用） */
const mockProgress = ref<ScanProgress>({ completed: 0, failed: 0, total: 50, percent: 0 })

const scanProgress = computed<ScanProgress>(() => {
  return mockProgress.value
})

const scanProgressPercent = computed(() => {
  if (scanProgress.value.total === 0) return 0
  return Math.round((scanProgress.value.completed / scanProgress.value.total) * 100)
})

// ============ 查询配置 ============
const queryConfig = reactive<QueryConfig>({
  filters: [],
  sort_by: '',
  sort_order: 'asc',
  limit: 100,
  columns: [],
})

function addFilter() {
  queryConfig.filters.push({ param_name: '', min: -Infinity, max: Infinity })
}

function removeFilter(idx: number) {
  queryConfig.filters.splice(idx, 1)
}

// ============ Mock 数据生成 ============
function generateMockResults(): ScanResult[] {
  const results: ScanResult[] = []
  const seed = (n: number) => {
    let x = Math.sin(n * 127.1 + 311.7) * 43758.5453
    return x - Math.floor(x)
  }
  for (let i = 0; i < 50; i++) {
    const temperature = 600 + seed(i * 3) * 400
    const stress = 50 + seed(i * 3 + 1) * 250
    const grain_size = 10 + Math.floor(seed(i * 3 + 2) * 90)
    const norton_n = 3 + seed(i * 5) * 5
    const activation_Q = 200 + seed(i * 5 + 1) * 150
    // Norton creep: strain_rate = A * sigma^n * exp(-Q/RT)
    const R = 8.314
    const A = 1e-12
    const strain_rate = A * Math.pow(stress * 1e6, norton_n) * Math.exp(-activation_Q * 1000 / (R * temperature))
    const result_value = Math.log10(strain_rate)
    results.push({
      scan_id: `scan_${String(i).padStart(4, '0')}`,
      config: {
        name: `scan_${String(i).padStart(4, '0')}`,
        doe_method: 'latin_hypercube',
        parameters: [
          { name: 'temperature', min: 600, max: 1000, type: 'continuous' },
          { name: 'stress', min: 50, max: 300, type: 'continuous' },
          { name: 'grain_size', min: 10, max: 100, type: 'discrete' },
          { name: 'norton_n', min: 3, max: 8, type: 'continuous' },
          { name: 'activation_Q', min: 200, max: 350, type: 'continuous' },
        ],
        total_combinations: 50,
        max_parallel: 4,
        timeout_per_run_s: 300,
      },
      status: 'completed',
      tasks: [{
        task_id: `task_${String(i).padStart(4, '0')}`,
        params: { temperature, stress, grain_size, norton_n, activation_Q },
        status: 'completed',
        result: { result_value },
        error: null,
        duration_s: 2 + seed(i * 7) * 10,
        started_at: new Date().toISOString(),
        completed_at: new Date().toISOString(),
      }],
      progress: { completed: 50, failed: 0, total: 50, percent: 100 },
      total_time_s: 2 + seed(i * 7) * 10,
      results_db_path: '',
    })
  }
  return results
}

const scanResults = ref<ScanResult[]>([])

// ============ 辅助：从 ScanResult 提取参数和结果值 ============
/** 获取 ScanResult 中第一个 task 的参数 */
function getParams(result: ScanResult): Record<string, number> {
  if (result.tasks.length > 0) {
    return result.tasks[0].params
  }
  return {}
}

/** 获取 ScanResult 中第一个 task 的 result_value */
function getResultValue(result: ScanResult): number {
  if (result.tasks.length > 0 && result.tasks[0].result) {
    const rv = result.tasks[0].result!['result_value']
    return typeof rv === 'number' ? rv : 0
  }
  return 0
}

// ============ 筛选结果 ============
const filteredResults = computed(() => {
  if (queryConfig.filters.length === 0) return scanResults.value
  return scanResults.value.filter(result => {
    return queryConfig.filters.every(filter => {
      const val = getParams(result)[filter.param_name]
      if (val === undefined) return true
      return val >= filter.min && val <= filter.max
    })
  })
})

/** 将 ScanResult[] 转换为扁平化的行数据用于表格展示 */
const flatRows = computed(() => {
  return filteredResults.value.map(result => {
    const params = getParams(result)
    const rv = getResultValue(result)
    return {
      scan_id: result.scan_id,
      ...params,
      result_value: rv,
      status: result.status,
    }
  })
})

const resultColumns = computed(() => {
  if (scanResults.value.length === 0) return []
  const firstParams = getParams(scanResults.value[0])
  const paramKeys = Object.keys(firstParams)
  return [
    { key: 'scan_id', label: 'ID' },
    ...paramKeys.map(k => ({ key: k, label: k })),
    { key: 'result_value', label: '结果值' },
    { key: 'status', label: '状态' },
  ]
})

const minResultValue = computed(() => {
  if (scanResults.value.length === 0) return 0
  return Math.min(...scanResults.value.map(r => getResultValue(r)))
})

// ============ 并行坐标可视化 ============
const parallelAxes = computed(() => {
  if (scanResults.value.length === 0) return []
  const firstParams = getParams(scanResults.value[0])
  const paramKeys = Object.keys(firstParams)
  return paramKeys.slice(0, 5).map(key => {
    const values = scanResults.value.map(r => getParams(r)[key] as number)
    return {
      label: key,
      min: Math.min(...values).toFixed(1),
      max: Math.max(...values).toFixed(1),
    }
  })
})

const parallelLines = computed(() => {
  if (scanResults.value.length === 0 || parallelAxes.value.length === 0) return []
  const firstParams = getParams(scanResults.value[0])
  const paramKeys = Object.keys(firstParams).slice(0, 5)
  // 取前5条有代表性的数据
  const sorted = [...scanResults.value].sort((a, b) => getResultValue(a) - getResultValue(b))
  const indices = [0, Math.floor(sorted.length * 0.25), Math.floor(sorted.length * 0.5), Math.floor(sorted.length * 0.75), sorted.length - 1]
  const colors = ['var(--accent-green)', 'var(--primary)', 'var(--accent-yellow)', '#f97316', 'var(--accent-red)']
  return indices.map((idx, li) => {
    const result = sorted[idx]
    const points = paramKeys.map((key, ki) => {
      const values = scanResults.value.map(r => getParams(r)[key] as number)
      const minVal = Math.min(...values)
      const maxVal = Math.max(...values)
      const val = getParams(result)[key] as number
      const normalized = maxVal === minVal ? 0.5 : (val - minVal) / (maxVal - minVal)
      const x = 80 + ki * 130
      const y = 240 - normalized * 220
      return `${x},${y}`
    })
    return { points: points.join(' '), color: colors[li] }
  })
})

// ============ 热力图 ============
const heatmapCells = computed(() => {
  if (scanResults.value.length === 0) return []
  const cells = []
  const xBins = 8
  const yBins = 6
  const cellW = 44
  const cellH = 38
  const offsetX = 40
  const offsetY = 30

  // 收集温度和应力范围
  const temps = scanResults.value.map(r => getParams(r).temperature)
  const stresses = scanResults.value.map(r => getParams(r).stress)
  const tMin = Math.min(...temps)
  const tMax = Math.max(...temps)
  const sMin = Math.min(...stresses)
  const sMax = Math.max(...stresses)
  const results = scanResults.value.map(r => getResultValue(r))
  const rMin = Math.min(...results)
  const rMax = Math.max(...results)

  for (let xi = 0; xi < xBins; xi++) {
    for (let yi = 0; yi < yBins; yi++) {
      const tLow = tMin + (tMax - tMin) * xi / xBins
      const tHigh = tMin + (tMax - tMin) * (xi + 1) / xBins
      const sLow = sMin + (sMax - sMin) * yi / yBins
      const sHigh = sMin + (sMax - sMin) * (yi + 1) / yBins

      const matching = scanResults.value.filter(r =>
        getParams(r).temperature >= tLow && getParams(r).temperature < tHigh &&
        getParams(r).stress >= sLow && getParams(r).stress < sHigh
      )

      const avgResult = matching.length > 0
        ? matching.reduce((sum, r) => sum + getResultValue(r), 0) / matching.length
        : null

      let color = 'var(--bg-elevated)'
      if (avgResult !== null) {
        const norm = rMax === rMin ? 0.5 : (avgResult - rMin) / (rMax - rMin)
        const r = Math.round(norm * 220 + 37)
        const g = Math.round((1 - Math.abs(norm - 0.5) * 2) * 180 + 40)
        const b = Math.round((1 - norm) * 200 + 37)
        color = `rgb(${r}, ${g}, ${b})`
      }

      cells.push({
        x: offsetX + xi * cellW,
        y: offsetY + yi * cellH,
        w: cellW,
        h: cellH,
        color,
      })
    }
  }
  return cells
})

const heatmapXLabels = computed(() => {
  const labels = []
  for (let i = 0; i <= 8; i += 2) {
    const val = 600 + (400 * i / 8)
    labels.push({ x: 40 + i * 44 + 22, label: val.toFixed(0) })
  }
  return labels
})

const heatmapYLabels = computed(() => {
  const labels = []
  for (let i = 0; i <= 6; i += 2) {
    const val = 50 + (250 * i / 6)
    labels.push({ x: 35, y: 30 + i * 38 + 22, label: val.toFixed(0) })
  }
  return labels
})

// ============ 统计分析 ============
const optimalParams = computed(() => {
  if (scanResults.value.length === 0) return []
  const best = scanResults.value.reduce((a, b) => getResultValue(a) < getResultValue(b) ? a : b)
  const params = getParams(best)
  return Object.entries(params).map(([name, value]) => ({
    name,
    value: typeof value === 'number' ? value.toFixed(2) : String(value),
  }))
})

const optimalResultValue = computed(() => {
  if (scanResults.value.length === 0) return '-'
  return Math.min(...scanResults.value.map(r => getResultValue(r))).toFixed(4)
})

const correlations = computed(() => {
  if (scanResults.value.length === 0) return []
  const firstParams = getParams(scanResults.value[0])
  const paramKeys = Object.keys(firstParams)
  const resultValues = scanResults.value.map(r => getResultValue(r))
  const rMean = resultValues.reduce((a, b) => a + b, 0) / resultValues.length

  return paramKeys.slice(0, 4).map(key => {
    const pValues = scanResults.value.map(r => getParams(r)[key] as number)
    const pMean = pValues.reduce((a, b) => a + b, 0) / pValues.length
    let num = 0, denA = 0, denB = 0
    for (let i = 0; i < pValues.length; i++) {
      const da = pValues[i] - pMean
      const db = resultValues[i] - rMean
      num += da * db
      denA += da * da
      denB += db * db
    }
    const corr = denA === 0 || denB === 0 ? 0 : num / Math.sqrt(denA * denB)
    return { name: key, value: corr }
  })
})

const sensitivityRankings = computed(() => {
  if (scanResults.value.length === 0) return []
  const absCorrs = correlations.value.map(c => ({ name: c.name, score: Math.abs(c.value) * 100 }))
  absCorrs.sort((a, b) => b.score - a.score)
  return absCorrs
})

// ============ 操作方法 ============
function createScan() {
  scanning.value = true
  mockProgress.value = { completed: 0, failed: 0, total: 50, percent: 0 }

  scanTask.value = {
    task_id: 'task_001',
    params: {},
    status: 'running',
    result: null,
    error: null,
    duration_s: 0,
    started_at: new Date().toISOString(),
    completed_at: new Date().toISOString(),
  }

  // 模拟扫描过程
  let completed = 0
  const interval = setInterval(() => {
    completed += Math.floor(Math.random() * 5) + 1
    if (completed >= 50) {
      completed = 50
      clearInterval(interval)
      scanning.value = false
      scanResults.value = generateMockResults()
      mockProgress.value.completed = 50
      mockProgress.value.failed = Math.floor(Math.random() * 3)
      mockProgress.value.percent = 100
      if (scanTask.value) {
        scanTask.value.status = 'completed'
      }
    }
    if (completed < 50) {
      mockProgress.value.completed = Math.min(completed, 50)
      mockProgress.value.percent = Math.round((Math.min(completed, 50) / 50) * 100)
    }
  }, 200)
}

function applyQuery() {
  // 触发 computed 重新计算
}

function exportData(format: string) {
  // 导出逻辑
}

function resetAll() {
  scanConfig.name = ''
  scanConfig.doe_method = 'latin_hypercube'
  scanConfig.parameters = []
  scanConfig.total_combinations = 0
  scanConfig.max_parallel = 4
  scanConfig.timeout_per_run_s = 300
  scanConfig.template_id = ''
  paramRanges.value = [
    { name: 'temperature', min: 600, max: 1000, step: 50, type: 'continuous' },
    { name: 'stress', min: 50, max: 300, step: 25, type: 'continuous' },
    { name: 'grain_size', min: 10, max: 100, step: 10, type: 'discrete' },
  ]
  scanResults.value = []
  scanTask.value = null
  mockProgress.value = { completed: 0, failed: 0, total: 50, percent: 0 }
  queryConfig.filters = []
}

onMounted(() => {
  // 初始化
})
</script>
