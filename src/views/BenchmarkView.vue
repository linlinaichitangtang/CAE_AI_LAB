<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">多尺度标准算例库</h2>
        <p class="text-sm" style="color: var(--text-muted)">V1.8-005 | 验证算例，计算精度对比，回归测试</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="resetAll" class="btn btn-ghost text-xs">重置</button>
        <button v-if="hasRunResults" @click="exportResults" class="btn btn-primary text-xs">导出结果</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Category & Difficulty Filter -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            算例筛选
          </h4>
          <div class="mb-3">
            <label class="label">类别</label>
            <div class="flex flex-wrap gap-1">
              <button
                v-for="cat in categoryFilters"
                :key="cat.value"
                @click="selectedCategory = cat.value"
                class="px-2 py-1 rounded text-xs transition border"
                :style="selectedCategory === cat.value
                  ? 'background: var(--primary); border-color: var(--primary); color: white'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
              >
                {{ cat.label }}
              </button>
            </div>
          </div>
          <div>
            <label class="label">难度</label>
            <div class="flex gap-1">
              <button
                v-for="d in difficultyFilters"
                :key="d.value"
                @click="selectedDifficulty = d.value"
                class="px-2 py-1 rounded text-xs transition border"
                :style="selectedDifficulty === d.value
                  ? 'background: var(--primary); border-color: var(--primary); color: white'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
              >
                {{ d.label }}
              </button>
            </div>
          </div>
        </div>

        <!-- Step 2: Benchmark Case List -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            算例列表
            <span class="ml-1 text-[10px] font-normal" style="color: var(--text-muted)">({{ filteredCases.length }})</span>
          </h4>
          <div class="space-y-2 max-h-48 overflow-y-auto">
            <div
              v-for="b in filteredCases"
              :key="b.id"
              @click="selectedCaseId = b.id"
              class="p-2 rounded border cursor-pointer transition"
              :style="selectedCaseId === b.id
                ? 'background: var(--bg-base); border-color: var(--primary); box-shadow: 0 0 0 1px var(--primary-glow)'
                : 'background: var(--bg-elevated); border-color: var(--border-default)'"
            >
              <div class="text-xs font-medium" style="color: var(--text-primary)">{{ b.name_zh }}</div>
              <div class="text-[10px] mt-0.5" style="color: var(--text-muted)">{{ b.name }}</div>
              <div class="flex items-center gap-1 mt-1.5 flex-wrap">
                <span v-for="s in b.scales" :key="s" class="px-1.5 py-0.5 rounded text-[9px]" style="background: var(--bg-base); color: var(--text-secondary); border: 1px solid var(--border-subtle)">
                  {{ scaleLabels[s] || s }}
                </span>
                <span class="ml-auto px-1.5 py-0.5 rounded text-[9px]" :style="difficultyStyle(b.difficulty)">
                  {{ difficultyLabels[b.difficulty] }}
                </span>
              </div>
              <div class="text-[10px] mt-1" style="color: var(--text-muted)">~{{ b.estimated_time_min }} min</div>
            </div>
          </div>
        </div>

        <!-- Step 3: Selected Benchmark Detail -->
        <div v-if="selectedCase" class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            算例详情
          </h4>
          <p class="text-xs mb-2" style="color: var(--text-secondary)">{{ selectedCase.description }}</p>
          <div class="text-[10px] mb-3" style="color: var(--text-muted)">
            参考文献: {{ selectedCase.reference_data.authors }} ({{ selectedCase.reference_data.year }})
            <br />DOI: {{ selectedCase.reference_data.doi }}
          </div>
          <div class="text-[10px] font-medium mb-1" style="color: var(--text-muted)">预期结果</div>
          <div class="rounded border overflow-hidden" style="border-color: var(--border-default)">
            <table class="w-full text-[10px]">
              <thead>
                <tr style="background: var(--bg-elevated)">
                  <th class="px-2 py-1 text-left" style="color: var(--text-muted)">物理量</th>
                  <th class="px-2 py-1 text-right" style="color: var(--text-muted)">参考值</th>
                  <th class="px-2 py-1 text-right" style="color: var(--text-muted)">容差</th>
                  <th class="px-2 py-1 text-right" style="color: var(--text-muted)">单位</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="r in selectedCase.expected_results" :key="r.quantity" style="border-top: 1px solid var(--border-subtle)">
                  <td class="px-2 py-1" style="color: var(--text-secondary)">{{ r.quantity }}</td>
                  <td class="px-2 py-1 text-right" style="color: var(--text-primary)">{{ r.value }}</td>
                  <td class="px-2 py-1 text-right" style="color: var(--text-muted)">+/- {{ r.tolerance }}%</td>
                  <td class="px-2 py-1 text-right" style="color: var(--text-muted)">{{ r.unit }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- Step 4: Run Config -->
        <div v-if="selectedCase" class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            运行配置
          </h4>
          <div class="mb-3">
            <label class="label">最大运行时间 (s)</label>
            <input v-model.number="runConfig.max_time_s" type="number" step="60" min="60" class="input w-full text-xs" />
          </div>
          <button @click="runBenchmark" :disabled="running" class="btn btn-primary text-xs w-full">
            {{ running ? '运行中...' : '运行算例' }}
          </button>
        </div>

        <!-- Step 5: Run Results -->
        <div v-if="runResult" class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" :style="{ background: runResult.passed ? 'var(--accent-green)' : 'var(--accent-red)' }">5</span>
            运行结果
          </h4>
          <div class="rounded border overflow-hidden mb-2" style="border-color: var(--border-default)">
            <table class="w-full text-[10px]">
              <thead>
                <tr style="background: var(--bg-elevated)">
                  <th class="px-2 py-1 text-left" style="color: var(--text-muted)">物理量</th>
                  <th class="px-2 py-1 text-right" style="color: var(--text-muted)">计算值</th>
                  <th class="px-2 py-1 text-right" style="color: var(--text-muted)">参考值</th>
                  <th class="px-2 py-1 text-right" style="color: var(--text-muted)">误差</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="r in runResult.results" :key="r.quantity" style="border-top: 1px solid var(--border-subtle)">
                  <td class="px-2 py-1" style="color: var(--text-secondary)">{{ r.quantity }}</td>
                  <td class="px-2 py-1 text-right" style="color: var(--text-primary)">{{ r.computed.toFixed(4) }}</td>
                  <td class="px-2 py-1 text-right" style="color: var(--text-muted)">{{ r.reference.toFixed(4) }}</td>
                  <td class="px-2 py-1 text-right font-medium" :style="{ color: r.error_percent <= 5 ? 'var(--accent-green)' : r.error_percent <= 10 ? 'var(--accent-yellow)' : 'var(--accent-red)' }">
                    {{ r.error_percent.toFixed(2) }}%
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
          <div class="flex items-center gap-2 p-2 rounded" :style="{ background: runResult.passed ? 'rgba(34,197,94,0.1)' : 'rgba(239,68,68,0.1)' }">
            <span class="text-xs font-medium" :style="{ color: runResult.passed ? 'var(--accent-green)' : 'var(--accent-red)' }">
              {{ runResult.passed ? 'PASS' : 'FAIL' }}
            </span>
            <span class="text-[10px]" style="color: var(--text-muted)">耗时 {{ runResult.total_time_s.toFixed(1) }}s</span>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization & Results -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <div class="flex-1 overflow-y-auto p-4 space-y-4">

          <!-- Donut Chart: Category Distribution -->
          <div class="p-4 rounded-lg border" style="background: var(--bg-surface); border-color: var(--border-subtle)">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">类别分布</h4>
            <div class="flex items-center gap-6">
              <svg :viewBox="`0 0 ${donutSize} ${donutSize}`" :width="donutSize" :height="donutSize" style="max-width: 200px; max-height: 200px">
                <g v-for="(seg, idx) in donutSegments" :key="seg.label">
                  <circle
                    :cx="donutSize / 2"
                    :cy="donutSize / 2"
                    :r="donutRadius"
                    fill="none"
                    :stroke="donutColors[idx % donutColors.length]"
                    :stroke-width="donutThickness"
                    :stroke-dasharray="`${seg.length} ${donutCircumference - seg.length}`"
                    :stroke-dashoffset="-seg.offset"
                    stroke-linecap="round"
                    opacity="0.85" />
                </g>
                <text :x="donutSize / 2" :y="donutSize / 2 - 6" text-anchor="middle" fill="var(--text-primary)" font-size="20" font-weight="600">
                  {{ allCases.length }}
                </text>
                <text :x="donutSize / 2" :y="donutSize / 2 + 12" text-anchor="middle" fill="var(--text-muted)" font-size="10">
                  总算例
                </text>
              </svg>
              <div class="space-y-1.5">
                <div v-for="(seg, idx) in donutSegments" :key="'legend-' + seg.label" class="flex items-center gap-2">
                  <span class="w-3 h-3 rounded-sm" :style="{ background: donutColors[idx % donutColors.length] }"></span>
                  <span class="text-xs" style="color: var(--text-secondary)">{{ seg.label }}</span>
                  <span class="text-xs font-medium" style="color: var(--text-primary)">{{ seg.count }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Comparison Chart: Computed vs Reference -->
          <div v-if="runResult" class="p-4 rounded-lg border" style="background: var(--bg-surface); border-color: var(--border-subtle)">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">计算值 vs 参考值对比</h4>
            <svg :viewBox="`0 0 ${compareWidth} ${compareHeight}`" class="w-full" style="max-height: 240px">
              <!-- Grid -->
              <line v-for="i in 5" :key="'cg-' + i"
                :x1="padding" :y1="padding + (compareHeight - 2 * padding) * (1 - i / 5)"
                :x2="compareWidth - padding" :y2="padding + (compareHeight - 2 * padding) * (1 - i / 5)"
                stroke="var(--border-subtle)" stroke-dasharray="4,4" />
              <!-- Y axis labels -->
              <text v-for="i in 5" :key="'cyl-' + i"
                :x="padding - 4" :y="padding + (compareHeight - 2 * padding) * (1 - i / 5) + 4"
                text-anchor="end" fill="var(--text-muted)" font-size="9">
                {{ (compareMaxVal * i / 5).toFixed(1) }}
              </text>
              <!-- Bars -->
              <g v-for="(r, idx) in runResult.results" :key="r.quantity">
                <rect
                  :x="padding + idx * compareGroupWidth + 4"
                  :y="padding + (compareHeight - 2 * padding) * (1 - r.reference / compareMaxVal)"
                  :width="compareBarWidth"
                  :height="(compareHeight - 2 * padding) * (r.reference / compareMaxVal)"
                  rx="2"
                  fill="var(--primary)"
                  opacity="0.6" />
                <rect
                  :x="padding + idx * compareGroupWidth + 4 + compareBarWidth + 2"
                  :y="padding + (compareHeight - 2 * padding) * (1 - r.computed / compareMaxVal)"
                  :width="compareBarWidth"
                  :height="(compareHeight - 2 * padding) * (r.computed / compareMaxVal)"
                  rx="2"
                  :fill="r.error_percent <= 5 ? 'var(--accent-green)' : r.error_percent <= 10 ? 'var(--accent-yellow)' : 'var(--accent-red)'"
                  opacity="0.8" />
                <!-- X label -->
                <text
                  :x="padding + idx * compareGroupWidth + compareGroupWidth / 2"
                  :y="compareHeight - 4"
                  text-anchor="middle" fill="var(--text-muted)" font-size="8">
                  {{ r.quantity.length > 8 ? r.quantity.slice(0, 8) + '..' : r.quantity }}
                </text>
              </g>
              <!-- Legend -->
              <rect x="compareWidth - 140" y="8" width="10" height="10" rx="2" fill="var(--primary)" opacity="0.6" />
              <text x="compareWidth - 126" y="17" fill="var(--text-muted)" font-size="9">参考值</text>
              <rect x="compareWidth - 70" y="8" width="10" height="10" rx="2" fill="var(--accent-green)" opacity="0.8" />
              <text x="compareWidth - 56" y="17" fill="var(--text-muted)" font-size="9">计算值</text>
            </svg>
          </div>

          <!-- Pass/Fail Summary -->
          <div class="p-4 rounded-lg border" style="background: var(--bg-surface); border-color: var(--border-subtle)">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">通过/失败统计</h4>
            <div class="grid grid-cols-4 gap-3">
              <div class="p-3 rounded-lg text-center border" style="background: var(--bg-elevated); border-color: var(--border-default)">
                <div class="text-2xl font-bold" style="color: var(--text-primary)">{{ summaryStats.total }}</div>
                <div class="text-[10px] mt-1" style="color: var(--text-muted)">总算例</div>
              </div>
              <div class="p-3 rounded-lg text-center border" style="background: rgba(34,197,94,0.08); border-color: rgba(34,197,94,0.2)">
                <div class="text-2xl font-bold" style="color: var(--accent-green)">{{ summaryStats.passed }}</div>
                <div class="text-[10px] mt-1" style="color: var(--text-muted)">通过</div>
              </div>
              <div class="p-3 rounded-lg text-center border" style="background: rgba(239,68,68,0.08); border-color: rgba(239,68,68,0.2)">
                <div class="text-2xl font-bold" style="color: var(--accent-red)">{{ summaryStats.failed }}</div>
                <div class="text-[10px] mt-1" style="color: var(--text-muted)">失败</div>
              </div>
              <div class="p-3 rounded-lg text-center border" :style="{ background: summaryStats.passRate >= 80 ? 'rgba(34,197,94,0.08)' : 'rgba(239,68,68,0.08)', borderColor: summaryStats.passRate >= 80 ? 'rgba(34,197,94,0.2)' : 'rgba(239,68,68,0.2)' }">
                <div class="text-2xl font-bold" :style="{ color: summaryStats.passRate >= 80 ? 'var(--accent-green)' : 'var(--accent-red)' }">{{ summaryStats.passRate }}%</div>
                <div class="text-[10px] mt-1" style="color: var(--text-muted)">通过率</div>
              </div>
            </div>
            <!-- Per-case status list -->
            <div class="mt-3 space-y-1">
              <div v-for="run in allRunResults" :key="run.benchmark_id" class="flex items-center justify-between px-3 py-1.5 rounded text-xs" style="background: var(--bg-elevated)">
                <span style="color: var(--text-secondary)">{{ getCaseName(run.benchmark_id) }}</span>
                <div class="flex items-center gap-3">
                  <span style="color: var(--text-muted)">{{ run.total_time_s.toFixed(1) }}s</span>
                  <span class="px-2 py-0.5 rounded text-[10px] font-medium" :style="{ background: run.passed ? 'rgba(34,197,94,0.15)' : 'rgba(239,68,68,0.15)', color: run.passed ? 'var(--accent-green)' : 'var(--accent-red)' }">
                    {{ run.passed ? 'PASS' : 'FAIL' }}
                  </span>
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
import type { BenchmarkCategory, BenchmarkCase, BenchmarkRun, BenchmarkRunConfig } from '../api/benchmark'

// ============ Constants ============

const categoryFilters = [
  { value: 'all', label: '全部' },
  { value: 'solidification', label: '凝固' },
  { value: 'phase_transformation', label: '相变' },
  { value: 'fracture', label: '断裂' },
  { value: 'creep', label: '蠕变' },
  { value: 'diffusion', label: '扩散' },
  { value: 'elasticity', label: '弹性' },
  { value: 'thermal', label: '热学' }
]

const difficultyFilters = [
  { value: 'all', label: '全部' },
  { value: 'beginner', label: '初级' },
  { value: 'intermediate', label: '中级' },
  { value: 'advanced', label: '高级' }
]

const difficultyLabels: Record<string, string> = {
  beginner: '初级',
  intermediate: '中级',
  advanced: '高级'
}

const scaleLabels: Record<string, string> = {
  dft: 'DFT',
  md: 'MD',
  phase_field: '相场',
  fe: 'FE',
  multiscale: '多尺度'
}

const donutColors = ['#6366f1', '#8b5cf6', '#06b6d4', '#10b981', '#f59e0b', '#ef4444', '#ec4899']

// ============ State ============

const selectedCategory = ref('all')
const selectedDifficulty = ref('all')
const selectedCaseId = ref<string | null>(null)
const running = ref(false)
const runResult = ref<BenchmarkRun | null>(null)

const runConfig = reactive<BenchmarkRunConfig>({
  benchmark_id: '',
  parameters: {},
  max_time_s: 600
})

// ============ Mock Benchmark Cases ============

const allCases = ref<BenchmarkCase[]>([
  {
    id: 'bm-001',
    name: 'Laminate Solidification',
    name_zh: '层片凝固过程',
    category: 'solidification',
    description: '二元合金定向凝固层片生长过程，验证相场模型对层片间距选择的预测精度。',
    scales: ['phase_field', 'fe'],
    reference_data: { doi: '10.1016/j.actamat.2019.01.023', year: 2019, authors: 'Karma et al.' },
    expected_results: [
      { quantity: '层片间距', value: 12.5, tolerance: 5, unit: 'um' },
      { quantity: '生长速度', value: 0.85, tolerance: 8, unit: 'mm/s' },
      { quantity: '溶质浓度峰值', value: 4.2, tolerance: 6, unit: 'wt%' }
    ],
    difficulty: 'intermediate',
    estimated_time_min: 45
  },
  {
    id: 'bm-002',
    name: 'Martensitic Transformation',
    name_zh: '马氏体相变',
    category: 'phase_transformation',
    description: 'NiTi形状记忆合金马氏体相变，验证相场模型对相变温度滞后和微观组织演化的预测。',
    scales: ['phase_field', 'md'],
    reference_data: { doi: '10.1016/j.msea.2020.139815', year: 2020, authors: 'Levitas et al.' },
    expected_results: [
      { quantity: 'Ms温度', value: 320, tolerance: 3, unit: 'K' },
      { quantity: 'Mf温度', value: 280, tolerance: 4, unit: 'K' },
      { quantity: '滞后宽度', value: 40, tolerance: 8, unit: 'K' }
    ],
    difficulty: 'advanced',
    estimated_time_min: 90
  },
  {
    id: 'bm-003',
    name: 'Crack Propagation CTOD',
    name_zh: '裂纹扩展 CTOD',
    category: 'fracture',
    description: '单边缺口试样三点弯曲断裂试验，验证裂纹尖端张开位移(CTOD)的有限元预测精度。',
    scales: ['fe'],
    reference_data: { doi: '10.1016/j.engfracmech.2018.11.023', year: 2018, authors: 'Anderson et al.' },
    expected_results: [
      { quantity: '临界CTOD', value: 0.28, tolerance: 5, unit: 'mm' },
      { quantity: 'J积分', value: 125, tolerance: 6, unit: 'kJ/m^2' },
      { quantity: '极限载荷', value: 45.2, tolerance: 4, unit: 'kN' }
    ],
    difficulty: 'beginner',
    estimated_time_min: 20
  },
  {
    id: 'bm-004',
    name: 'Norton Creep Law',
    name_zh: 'Norton蠕变定律',
    category: 'creep',
    description: '高温合金稳态蠕变验证，验证Norton幂律蠕变本构模型的有限元实现。',
    scales: ['fe'],
    reference_data: { doi: '10.1016/j.ijplas.2017.03.012', year: 2017, authors: 'Frost & Ashby' },
    expected_results: [
      { quantity: '稳态蠕变率', value: 2.5e-4, tolerance: 10, unit: '1/s' },
      { quantity: '应力指数n', value: 4.5, tolerance: 5, unit: '-' },
      { quantity: '激活能Q', value: 280, tolerance: 8, unit: 'kJ/mol' }
    ],
    difficulty: 'beginner',
    estimated_time_min: 15
  },
  {
    id: 'bm-005',
    name: 'Fickian Diffusion',
    name_zh: 'Fick扩散过程',
    category: 'diffusion',
    description: '一维Fick扩散解析解对比，验证扩散方程数值求解器的精度。',
    scales: ['fe', 'md'],
    reference_data: { doi: '10.1016/j.commatsci.2019.109291', year: 2019, authors: 'Crank' },
    expected_results: [
      { quantity: '扩散系数D', value: 3.2e-10, tolerance: 5, unit: 'm^2/s' },
      { quantity: '穿透深度', value: 0.45, tolerance: 6, unit: 'mm' },
      { quantity: '浓度峰值偏差', value: 0.02, tolerance: 8, unit: '-' }
    ],
    difficulty: 'beginner',
    estimated_time_min: 10
  },
  {
    id: 'bm-006',
    name: 'Linear Elastic Notch',
    name_zh: '线弹性缺口应力集中',
    category: 'elasticity',
    description: '带圆孔平板单向拉伸应力集中系数验证，对比Kirsch解析解。',
    scales: ['fe'],
    reference_data: { doi: '10.1016/j.ijsolstr.2016.08.015', year: 2016, authors: 'Timoshenko' },
    expected_results: [
      { quantity: '应力集中系数Kt', value: 3.0, tolerance: 2, unit: '-' },
      { quantity: '孔边最大应力', value: 300, tolerance: 3, unit: 'MPa' },
      { quantity: '远场应力', value: 100, tolerance: 1, unit: 'MPa' }
    ],
    difficulty: 'beginner',
    estimated_time_min: 5
  },
  {
    id: 'bm-007',
    name: 'Steady State Heat Conduction',
    name_zh: '稳态热传导',
    category: 'thermal',
    description: '多层复合壁稳态热传导，验证界面热阻和温度分布的有限元预测。',
    scales: ['fe'],
    reference_data: { doi: '10.1016/j.ijheatmasstransfer.2018.05.032', year: 2018, authors: 'Incropera et al.' },
    expected_results: [
      { quantity: '热流密度', value: 1500, tolerance: 4, unit: 'W/m^2' },
      { quantity: '界面温度', value: 425, tolerance: 3, unit: 'K' },
      { quantity: '热阻', value: 0.035, tolerance: 5, unit: 'm^2K/W' }
    ],
    difficulty: 'beginner',
    estimated_time_min: 8
  },
  {
    id: 'bm-008',
    name: 'DFT Lattice Constant',
    name_zh: 'DFT晶格常数计算',
    category: 'elasticity',
    description: 'FCC铝晶格常数DFT计算，验证PAW赝势和k点采样的收敛性。',
    scales: ['dft'],
    reference_data: { doi: '10.1103/PhysRevB.73.045112', year: 2006, authors: 'Kresse & Furthmuller' },
    expected_results: [
      { quantity: '晶格常数a', value: 4.04, tolerance: 2, unit: 'A' },
      { quantity: '体模量B', value: 77, tolerance: 5, unit: 'GPa' },
      { quantity: '形成能', value: -3.74, tolerance: 3, unit: 'eV/atom' }
    ],
    difficulty: 'intermediate',
    estimated_time_min: 60
  },
  {
    id: 'bm-009',
    name: 'MD Thermal Conductivity',
    name_zh: 'MD热导率计算',
    category: 'thermal',
    description: '硅的热导率分子动力学计算，使用Green-Kubo方法验证。',
    scales: ['md'],
    reference_data: { doi: '10.1103/PhysRevB.84.085204', year: 2011, authors: 'Schelling et al.' },
    expected_results: [
      { quantity: '热导率k', value: 148, tolerance: 10, unit: 'W/mK' },
      { quantity: '声子MFP', value: 260, tolerance: 15, unit: 'nm' },
      { quantity: '比热Cv', value: 1.66, tolerance: 5, unit: 'MJ/m^3K' }
    ],
    difficulty: 'advanced',
    estimated_time_min: 120
  },
  {
    id: 'bm-010',
    name: 'Multiscale Homogenization',
    name_zh: '多尺度均匀化',
    category: 'elasticity',
    description: '纤维增强复合材料RVE均匀化，验证等效弹性模量的FE^2多尺度预测。',
    scales: ['fe', 'multiscale'],
    reference_data: { doi: '10.1016/j.cma.2019.112705', year: 2019, authors: 'Geers et al.' },
    expected_results: [
      { quantity: '等效E1', value: 142, tolerance: 6, unit: 'GPa' },
      { quantity: '等效E2', value: 10.3, tolerance: 8, unit: 'GPa' },
      { quantity: '等效G12', value: 5.8, tolerance: 10, unit: 'GPa' }
    ],
    difficulty: 'advanced',
    estimated_time_min: 180
  },
  {
    id: 'bm-011',
    name: 'Dendrite Growth Competition',
    name_zh: '枝晶竞争生长',
    category: 'solidification',
    description: '多枝晶竞争生长选择过程，验证相场模型对枝晶取向选择的预测。',
    scales: ['phase_field', 'multiscale'],
    reference_data: { doi: '10.1038/ncomms11208', year: 2016, authors: 'Tourret et al.' },
    expected_results: [
      { quantity: '优选取向偏差', value: 2.5, tolerance: 12, unit: 'deg' },
      { quantity: '尖端半径', value: 0.35, tolerance: 10, unit: 'um' },
      { quantity: '生长速度', value: 1.2, tolerance: 8, unit: 'mm/s' }
    ],
    difficulty: 'advanced',
    estimated_time_min: 150
  },
  {
    id: 'bm-012',
    name: 'Hydrogen Embrittlement',
    name_zh: '氢脆断裂',
    category: 'fracture',
    description: '氢辅助裂纹扩展模拟，DFT计算氢-铁结合能 + 相场断裂预测。',
    scales: ['dft', 'phase_field', 'fe'],
    reference_data: { doi: '10.1016/j.actamat.2020.09.058', year: 2020, authors: 'Jiang et al.' },
    expected_results: [
      { quantity: 'H-Fe结合能', value: -2.78, tolerance: 5, unit: 'eV' },
      { quantity: '临界应力强度', value: 18.5, tolerance: 10, unit: 'MPa*m^0.5' },
      { quantity: '氢扩散系数', value: 1.2e-6, tolerance: 12, unit: 'cm^2/s' }
    ],
    difficulty: 'advanced',
    estimated_time_min: 200
  }
])

// ============ Mock Run Results ============

const allRunResults = ref<BenchmarkRun[]>([])

function buildMockRun(bm: BenchmarkCase): BenchmarkRun {
  const results = bm.expected_results.map(er => {
    const errorFactor = 1 + (Math.random() - 0.4) * (er.tolerance / 100) * 2
    const computed = er.value * errorFactor
    const errorPercent = Math.abs(computed - er.value) / er.value * 100
    return {
      quantity: er.quantity,
      computed,
      reference: er.value,
      error_percent: errorPercent
    }
  })
  const maxError = Math.max(...results.map(r => r.error_percent))
  const passed = maxError <= bm.expected_results[0].tolerance * 1.2
  return {
    benchmark_id: bm.id,
    status: 'completed',
    results,
    total_time_s: bm.estimated_time_min * 0.6 + Math.random() * bm.estimated_time_min * 0.3,
    passed
  }
}

// ============ Computed ============

const filteredCases = computed(() => {
  return allCases.value.filter(b => {
    if (selectedCategory.value !== 'all' && b.category !== selectedCategory.value) return false
    if (selectedDifficulty.value !== 'all' && b.difficulty !== selectedDifficulty.value) return false
    return true
  })
})

const selectedCase = computed(() => {
  if (!selectedCaseId.value) return null
  return allCases.value.find(b => b.id === selectedCaseId.value) || null
})

const hasRunResults = computed(() => {
  return allRunResults.value.length > 0
})

const summaryStats = computed(() => {
  const total = allRunResults.value.length
  const passed = allRunResults.value.filter(r => r.passed).length
  const failed = total - passed
  const passRate = total > 0 ? Math.round(passed / total * 100) : 0
  return { total, passed, failed, passRate }
})

// ============ Donut Chart ============

const donutSize = 180
const donutRadius = 65
const donutThickness = 24
const donutCircumference = 2 * Math.PI * donutRadius

const donutSegments = computed(() => {
  const catMap: Record<string, number> = {}
  const catLabelMap: Record<string, string> = {
    solidification: '凝固',
    phase_transformation: '相变',
    fracture: '断裂',
    creep: '蠕变',
    diffusion: '扩散',
    elasticity: '弹性',
    thermal: '热学'
  }
  for (const c of allCases.value) {
    catMap[c.category] = (catMap[c.category] || 0) + 1
  }
  const entries = Object.entries(catMap)
  const total = entries.reduce((s, [, v]) => s + v, 0)
  let offset = 0
  return entries.map(([cat, count]) => {
    const fraction = count / total
    const length = fraction * donutCircumference
    const seg = {
      label: catLabelMap[cat] || cat,
      count,
      length,
      offset
    }
    offset += length
    return seg
  })
})

// ============ Comparison Chart ============

const compareWidth = 600
const compareHeight = 240
const compareBarWidth = 16
const compareGroupWidth = 48
const padding = 20

const compareMaxVal = computed(() => {
  if (!runResult.value) return 1
  return Math.max(
    ...runResult.value.results.map(r => Math.max(r.computed, r.reference))
  ) * 1.15
})

// ============ Actions ============

function runBenchmark() {
  if (!selectedCase.value) return
  running.value = true
  runConfig.benchmark_id = selectedCase.value.id

  setTimeout(() => {
    const result = buildMockRun(selectedCase.value!)
    runResult.value = result
    // Add to all results if not already there
    const existing = allRunResults.value.findIndex(r => r.benchmark_id === result.benchmark_id)
    if (existing >= 0) {
      allRunResults.value[existing] = result
    } else {
      allRunResults.value.push(result)
    }
    running.value = false
  }, 1200)
}

function resetAll() {
  selectedCategory.value = 'all'
  selectedDifficulty.value = 'all'
  selectedCaseId.value = null
  runResult.value = null
  allRunResults.value = []
  running.value = false
  runConfig.max_time_s = 600
}

function exportResults() {
  const data = JSON.stringify(allRunResults.value, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'benchmark_results.json'
  a.click()
  URL.revokeObjectURL(url)
}

function getCaseName(benchmarkId: string): string {
  const found = allCases.value.find(b => b.id === benchmarkId)
  return found ? found.name_zh : benchmarkId
}

function difficultyStyle(diff: string): Record<string, string> {
  switch (diff) {
    case 'beginner':
      return { background: 'rgba(34,197,94,0.15)', color: 'var(--accent-green)' }
    case 'intermediate':
      return { background: 'rgba(245,158,11,0.15)', color: 'var(--accent-yellow)' }
    case 'advanced':
      return { background: 'rgba(239,68,68,0.15)', color: 'var(--accent-red)' }
    default:
      return { background: 'var(--bg-elevated)', color: 'var(--text-muted)' }
  }
}

// ============ Lifecycle ============

onMounted(() => {
  // Pre-run some mock results for demonstration
  const preRunIds = ['bm-001', 'bm-003', 'bm-004', 'bm-005', 'bm-006', 'bm-007']
  for (const id of preRunIds) {
    const bm = allCases.value.find(b => b.id === id)
    if (bm) {
      allRunResults.value.push(buildMockRun(bm))
    }
  }
  selectedCaseId.value = 'bm-001'
  runConfig.benchmark_id = 'bm-001'
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
