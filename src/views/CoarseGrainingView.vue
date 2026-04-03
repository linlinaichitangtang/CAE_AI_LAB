<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">粗粒化策略库</h2>
        <p class="text-sm" style="color: var(--text-muted)">V1.8-003 | QC/MQC/CG 多尺度粗粒化方法对比与推荐</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="compareAllMethods" class="btn btn-ghost text-xs">全部对比</button>
        <button @click="resetAll" class="btn btn-ghost text-xs">重置</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Scale Selectors -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            尺度选择
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">源尺度</label>
              <select v-model="sourceScale" class="input w-full text-xs">
                <option value="dft">DFT (第一性原理)</option>
                <option value="md">MD (分子动力学)</option>
                <option value="phase_field">Phase Field (相场)</option>
              </select>
            </div>
            <div class="text-center text-lg" style="color: var(--text-muted)">→</div>
            <div>
              <label class="label">目标尺度</label>
              <select v-model="targetScale" class="input w-full text-xs">
                <option value="md">MD (分子动力学)</option>
                <option value="phase_field">Phase Field (相场)</option>
                <option value="fe">FE (有限元)</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Step 2: Method Selector -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            粗粒化方法
          </h4>
          <div class="grid grid-cols-2 gap-1">
            <button
              v-for="m in methodOptions" :key="m.value"
              @click="selectedMethod = m.value"
              class="px-2 py-2 rounded text-xs text-left transition border relative"
              :style="selectedMethod === m.value
                ? 'background: var(--primary); border-color: var(--primary); color: white'
                : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
            >
              <div class="font-medium">{{ m.label }}</div>
              <div class="text-[10px] mt-0.5 opacity-80">{{ m.desc }}</div>
              <span
                class="absolute top-1 right-1 px-1 rounded text-[9px] font-bold"
                :style="selectedMethod === m.value
                  ? 'background: rgba(255,255,255,0.3); color: white'
                  : 'background: var(--primary); color: white'"
              >{{ m.score }}</span>
            </button>
          </div>
        </div>

        <!-- Step 3: Method Parameters -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            方法参数
          </h4>
          <div class="space-y-2">
            <!-- Quasicontinuum / MQC params -->
            <div v-if="selectedMethod === 'quasicontinuum' || selectedMethod === 'mqc'">
              <label class="label">截断半径 (Å)</label>
              <input v-model.number="params.cutoff_radius" type="number" step="0.5" min="1" class="input w-full text-xs" />
            </div>
            <div v-if="selectedMethod === 'quasicontinuum'">
              <label class="label">代表原子数</label>
              <input v-model.number="params.n_representative" type="number" min="10" class="input w-full text-xs" />
            </div>

            <!-- Radial Average params -->
            <div v-if="selectedMethod === 'radial_average'">
              <label class="label">截断半径 (Å)</label>
              <input v-model.number="params.cutoff_radius" type="number" step="0.5" min="1" class="input w-full text-xs" />
              <label class="label">径向分箱数</label>
              <input v-model.number="params.n_bins" type="number" min="10" class="input w-full text-xs" />
            </div>

            <!-- Fourier Filter params -->
            <div v-if="selectedMethod === 'fourier_filter'">
              <label class="label">网格分辨率 (Å)</label>
              <input v-model.number="params.grid_resolution" type="number" step="0.1" min="0.1" class="input w-full text-xs" />
              <label class="label">滤波截止 (1/Å)</label>
              <input v-model.number="params.filter_cutoff" type="number" step="0.1" min="0.1" class="input w-full text-xs" />
            </div>

            <!-- ML Mapping params -->
            <div v-if="selectedMethod === 'ml_mapping'">
              <label class="label">聚类数 (n_clusters)</label>
              <input v-model.number="params.n_clusters" type="number" min="2" max="1000" class="input w-full text-xs" />
              <label class="label">网格分辨率 (Å)</label>
              <input v-model.number="params.grid_resolution" type="number" step="0.1" min="0.1" class="input w-full text-xs" />
            </div>

            <!-- Voronoi params -->
            <div v-if="selectedMethod === 'voronoi'">
              <label class="label">种子点数</label>
              <input v-model.number="params.n_clusters" type="number" min="2" max="10000" class="input w-full text-xs" />
              <label class="label">截断半径 (Å)</label>
              <input v-model.number="params.cutoff_radius" type="number" step="0.5" min="1" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Step 4: Actions -->
        <div class="panel-section space-y-2">
          <button @click="getRecommendation" class="btn btn-ghost text-xs w-full" :disabled="isRecommending">
            {{ isRecommending ? '分析中...' : '推荐方法' }}
          </button>
          <button @click="executeCoarseGraining" class="btn btn-primary text-xs w-full" :disabled="isExecuting">
            {{ isExecuting ? '计算中...' : '执行粗粒化' }}
          </button>

          <!-- Recommendation Cards -->
          <div v-if="recommendations.length > 0" class="space-y-2 mt-3">
            <div class="text-[10px] font-medium" style="color: var(--text-muted)">推荐结果</div>
            <div
              v-for="rec in recommendations" :key="rec.method"
              class="p-2 rounded border"
              style="background: var(--bg-elevated); border-color: var(--border-default)"
            >
              <div class="flex items-center justify-between">
                <span class="text-xs font-medium" style="color: var(--text-primary)">{{ methodLabels[rec.method] }}</span>
                <span class="text-xs font-mono font-bold" :style="{ color: rec.score >= 0.8 ? 'var(--accent-green)' : rec.score >= 0.6 ? 'var(--accent-yellow)' : 'var(--accent-red)' }">
                  {{ (rec.score * 100).toFixed(0) }}%
                </span>
              </div>
              <div class="text-[10px] mt-1" style="color: var(--text-muted)">{{ rec.reason }}</div>
              <div class="text-[10px] mt-0.5" style="color: var(--text-secondary)">
                预估精度: {{ (rec.estimated_accuracy * 100).toFixed(1) }}%
              </div>
            </div>
          </div>
        </div>

        <!-- Step 5: Result Summary -->
        <div v-if="cgResult" class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            结果摘要
          </h4>
          <div class="p-3 rounded border space-y-2" style="background: var(--bg-elevated); border-color: var(--border-default)">
            <div class="flex justify-between text-xs">
              <span style="color: var(--text-muted)">源点数</span>
              <span class="font-mono" style="color: var(--text-primary)">{{ cgResult.source_points_count.toLocaleString() }}</span>
            </div>
            <div class="flex justify-between text-xs">
              <span style="color: var(--text-muted)">目标点数</span>
              <span class="font-mono" style="color: var(--text-primary)">{{ cgResult.target_points_count.toLocaleString() }}</span>
            </div>
            <div class="flex justify-between text-xs">
              <span style="color: var(--text-muted)">压缩比</span>
              <span class="font-mono font-bold" style="color: var(--primary)">{{ cgResult.reduction_ratio.toFixed(1) }}x</span>
            </div>
            <div class="flex justify-between text-xs">
              <span style="color: var(--text-muted)">计算时间</span>
              <span class="font-mono" style="color: var(--text-primary)">{{ cgResult.computation_time_s.toFixed(2) }}s</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization & Results -->
      <div class="flex-1 overflow-y-auto p-6 space-y-6">

        <!-- Source → Coarse-Grained Visualization -->
        <div class="p-4 rounded-lg border" style="background: var(--bg-surface); border-color: var(--border-subtle)">
          <h3 class="text-sm font-medium mb-4" style="color: var(--text-primary)">粗粒化可视化</h3>
          <svg viewBox="0 0 560 300" class="w-full" style="max-height: 300px">
            <defs>
              <linearGradient id="fieldGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#4F8EF7;stop-opacity:0.8" />
                <stop offset="50%" style="stop-color:#4FF7B0;stop-opacity:0.6" />
                <stop offset="100%" style="stop-color:#F7D84F;stop-opacity:0.8" />
              </linearGradient>
            </defs>

            <!-- Source atoms (left side) -->
            <text x="100" y="20" fill="var(--text-secondary)" font-size="11" text-anchor="middle" font-weight="bold">源: 原子/粒子</text>
            <rect x="20" y="30" width="160" height="240" fill="var(--bg-elevated)" rx="4" stroke="var(--border-subtle)"/>
            <!-- Random dots representing atoms -->
            <g>
              <circle v-for="(atom, i) in sourceAtoms" :key="'atom-'+i"
                :cx="atom.x" :cy="atom.y" :r="atom.r"
                :fill="atom.color" opacity="0.7"
              />
            </g>
            <text x="100" y="285" fill="var(--text-muted)" font-size="10" text-anchor="middle">{{ sourcePointsLabel }} 个点</text>

            <!-- Arrow -->
            <g transform="translate(280, 150)">
              <path d="M -50 0 C -30 -15, 30 -15, 50 0" fill="none" stroke="var(--primary)" stroke-width="2"/>
              <polygon points="50,-4 58,0 50,4" fill="var(--primary)"/>
              <text x="0" y="-18" fill="var(--primary)" font-size="10" text-anchor="middle" font-weight="bold">CG</text>
            </g>

            <!-- Coarse-grained field (right side) -->
            <text x="440" y="20" fill="var(--text-secondary)" font-size="11" text-anchor="middle" font-weight="bold">目标: 粗粒化场</text>
            <rect x="360" y="30" width="180" height="240" fill="var(--bg-elevated)" rx="4" stroke="var(--border-subtle)"/>
            <!-- Color grid cells -->
            <g>
              <rect v-for="(cell, i) in cgFieldCells" :key="'cell-'+i"
                :x="cell.x" :y="cell.y" :width="cell.w" :height="cell.h"
                :fill="cell.color" :rx="2" opacity="0.8"
              />
            </g>
            <text x="450" y="285" fill="var(--text-muted)" font-size="10" text-anchor="middle">{{ targetPointsLabel }} 个单元</text>
          </svg>
        </div>

        <!-- Method Comparison Table -->
        <div class="p-4 rounded-lg border" style="background: var(--bg-surface); border-color: var(--border-subtle)">
          <h3 class="text-sm font-medium mb-4" style="color: var(--text-primary)">方法对比</h3>
          <div class="overflow-x-auto">
            <table class="w-full text-xs">
              <thead>
                <tr style="border-bottom: 1px solid var(--border-subtle)">
                  <th class="text-left py-2 px-3 font-medium" style="color: var(--text-muted)">方法</th>
                  <th class="text-left py-2 px-3 font-medium" style="color: var(--text-muted)">压缩比</th>
                  <th class="text-left py-2 px-3 font-medium" style="color: var(--text-muted)">精度 (%)</th>
                  <th class="text-left py-2 px-3 font-medium" style="color: var(--text-muted)">时间 (s)</th>
                  <th class="text-left py-2 px-3 font-medium" style="color: var(--text-muted)">评分</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(cmp, idx) in comparisonData" :key="idx"
                  style="border-bottom: 1px solid var(--border-subtle)"
                  :style="selectedMethod === cmp.method ? 'background: var(--bg-elevated)' : ''"
                >
                  <td class="py-2 px-3 font-medium" style="color: var(--text-primary)">{{ methodLabels[cmp.method] }}</td>
                  <td class="py-2 px-3 font-mono" style="color: var(--text-secondary)">{{ cmp.reduction.toFixed(1) }}x</td>
                  <td class="py-2 px-3 font-mono" :style="{ color: cmp.accuracy >= 95 ? 'var(--accent-green)' : cmp.accuracy >= 85 ? 'var(--accent-yellow)' : 'var(--accent-red)' }">
                    {{ cmp.accuracy.toFixed(1) }}
                  </td>
                  <td class="py-2 px-3 font-mono" style="color: var(--text-secondary)">{{ cmp.time.toFixed(2) }}</td>
                  <td class="py-2 px-3">
                    <span class="px-2 py-0.5 rounded text-[10px] font-bold"
                      :style="{
                        background: cmp.score >= 0.8 ? 'var(--accent-green)' : cmp.score >= 0.6 ? 'var(--accent-yellow)' : 'var(--accent-red)',
                        color: cmp.score >= 0.6 ? '#000' : '#fff'
                      }"
                    >{{ (cmp.score * 100).toFixed(0) }}</span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type { CoarseGrainingMethod, CoarseGrainingConfig, CoarseGrainingResult, MethodRecommendation } from '../api/coarseGraining'

// ============ Types ============

interface MethodOption {
  value: CoarseGrainingMethod
  label: string
  desc: string
  score: number
}

interface ComparisonEntry {
  method: CoarseGrainingMethod
  reduction: number
  accuracy: number
  time: number
  score: number
}

interface SourceAtom {
  x: number
  y: number
  r: number
  color: string
}

interface CgFieldCell {
  x: number
  y: number
  w: number
  h: number
  color: string
}

// ============ Constants ============

const methodLabels: Record<CoarseGrainingMethod, string> = {
  quasicontinuum: '准连续介质 (QC)',
  mqc: 'MQC',
  radial_average: '径向平均',
  fourier_filter: '傅里叶滤波',
  ml_mapping: 'ML 映射',
  voronoi: 'Voronoi',
}

const methodOptions: MethodOption[] = [
  { value: 'quasicontinuum', label: '准连续介质', desc: 'QC 方法', score: 92 },
  { value: 'mqc', label: 'MQC', desc: '多尺度 QC', score: 88 },
  { value: 'radial_average', label: '径向平均', desc: 'Radial Avg', score: 75 },
  { value: 'fourier_filter', label: '傅里叶滤波', desc: 'Fourier', score: 80 },
  { value: 'ml_mapping', label: 'ML 映射', desc: '机器学习', score: 85 },
  { value: 'voronoi', label: 'Voronoi', desc: 'Voronoi 分区', score: 78 },
]

// ============ State ============

const sourceScale = ref<string>('md')
const targetScale = ref<string>('phase_field')
const selectedMethod = ref<CoarseGrainingMethod>('quasicontinuum')
const isRecommending = ref(false)
const isExecuting = ref(false)
const cgResult = ref<CoarseGrainingResult | null>(null)
const recommendations = ref<MethodRecommendation[]>([])

const params = reactive({
  cutoff_radius: 5.0,
  grid_resolution: 0.5,
  filter_cutoff: 0.5,
  n_clusters: 100,
  n_bins: 50,
  n_representative: 500,
})

// ============ Mock Data ============

const comparisonData: ComparisonEntry[] = [
  { method: 'quasicontinuum', reduction: 156.3, accuracy: 97.2, time: 12.45, score: 0.92 },
  { method: 'mqc', reduction: 142.8, accuracy: 96.5, time: 15.30, score: 0.88 },
  { method: 'radial_average', reduction: 200.0, accuracy: 82.1, time: 3.20, score: 0.75 },
  { method: 'fourier_filter', reduction: 178.5, accuracy: 89.4, time: 5.80, score: 0.80 },
  { method: 'ml_mapping', reduction: 165.2, accuracy: 94.8, time: 28.60, score: 0.85 },
  { method: 'voronoi', reduction: 190.0, accuracy: 86.3, time: 8.90, score: 0.78 },
]

// Generate source atoms for visualization
function generateSourceAtoms(): SourceAtom[] {
  const atoms: SourceAtom[] = []
  const colors = ['#4F8EF7', '#4F8EF7', '#4F8EF7', '#6BA3F9', '#3D7DE5']
  // Use a seeded-like approach for deterministic random
  let seed = 42
  function pseudoRandom(): number {
    seed = (seed * 16807 + 0) % 2147483647
    return seed / 2147483647
  }
  for (let i = 0; i < 120; i++) {
    atoms.push({
      x: 25 + pseudoRandom() * 150,
      y: 40 + pseudoRandom() * 220,
      r: 1.5 + pseudoRandom() * 1.5,
      color: colors[Math.floor(pseudoRandom() * colors.length)],
    })
  }
  return atoms
}

// Generate CG field cells for visualization
function generateCgFieldCells(): CgFieldCell[] {
  const cells: CgFieldCell[] = []
  const colors = ['#4F8EF7', '#4FA8F7', '#4FC8F7', '#4FF7B0', '#8EF74F', '#F7D84F', '#F7A84F', '#F7734F']
  let seed = 123
  function pseudoRandom(): number {
    seed = (seed * 16807 + 0) % 2147483647
    return seed / 2147483647
  }
  const cols = 8
  const rows = 10
  const cellW = 170 / cols
  const cellH = 220 / rows
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      cells.push({
        x: 365 + c * cellW + 1,
        y: 35 + r * cellH + 1,
        w: cellW - 2,
        h: cellH - 2,
        color: colors[Math.floor(pseudoRandom() * colors.length)],
      })
    }
  }
  return cells
}

const sourceAtoms = generateSourceAtoms()
const cgFieldCells = generateCgFieldCells()

// ============ Computed ============

const sourcePointsLabel = computed(() => {
  if (cgResult.value) return cgResult.value.source_points_count.toLocaleString()
  return '4,000'
})

const targetPointsLabel = computed(() => {
  if (cgResult.value) return cgResult.value.target_points_count.toLocaleString()
  return '80'
})

// ============ Methods ============

function getRecommendation() {
  isRecommending.value = true
  recommendations.value = []

  setTimeout(() => {
    // Generate mock recommendations based on source/target scale
    const mockRecs: MethodRecommendation[] = [
      {
        method: 'quasicontinuum',
        score: 0.92,
        reason: 'QC 方法最适合 MD→Phase Field 的跨尺度粗粒化，兼顾精度与效率',
        estimated_accuracy: 0.972,
      },
      {
        method: 'mqc',
        score: 0.88,
        reason: 'MQC 在 QC 基础上增加了多尺度耦合，适合复杂缺陷结构',
        estimated_accuracy: 0.965,
      },
      {
        method: 'ml_mapping',
        score: 0.85,
        reason: 'ML 映射方法精度高但需要训练数据，适合已有数据集的场景',
        estimated_accuracy: 0.948,
      },
    ]

    recommendations.value = mockRecs
    isRecommending.value = false
  }, 600)
}

function executeCoarseGraining() {
  isExecuting.value = true

  setTimeout(() => {
    const methodEntry = comparisonData.find(c => c.method === selectedMethod.value)
    const sourceCount = 4000
    const reduction = methodEntry ? methodEntry.reduction : 100
    const targetCount = Math.round(sourceCount / reduction)

    // Generate mock field data
    const gridSize = { nx: 10, ny: 10, nz: 10 }
    const totalCells = gridSize.nx * gridSize.ny * gridSize.nz
    const values: number[] = []
    for (let i = 0; i < totalCells; i++) {
      values.push(Math.random())
    }

    cgResult.value = {
      success: true,
      method: selectedMethod.value,
      source_points_count: sourceCount,
      target_points_count: targetCount,
      reduction_ratio: reduction,
      computation_time_s: methodEntry ? methodEntry.time : 10,
      field_data: {
        grid_size: gridSize,
        values,
        field_name: 'coarse_grained_field',
      },
      metadata: {
        source_scale: sourceScale.value as CoarseGrainingConfig['source_scale'],
        target_scale: targetScale.value as CoarseGrainingConfig['target_scale'],
        method: selectedMethod.value,
        timestamp: new Date().toISOString(),
      },
    }

    isExecuting.value = false
  }, 1000)
}

function compareAllMethods() {
  // Simulate running all methods
  isExecuting.value = true

  setTimeout(() => {
    cgResult.value = {
      success: true,
      method: selectedMethod.value,
      source_points_count: 4000,
      target_points_count: 80,
      reduction_ratio: 50,
      computation_time_s: 12.45,
      field_data: {
        grid_size: { nx: 10, ny: 10, nz: 10 },
        values: Array(1000).fill(0).map(() => Math.random()),
        field_name: 'coarse_grained_field',
      },
      metadata: {
        source_scale: sourceScale.value as CoarseGrainingConfig['source_scale'],
        target_scale: targetScale.value as CoarseGrainingConfig['target_scale'],
        method: selectedMethod.value,
        timestamp: new Date().toISOString(),
      },
    }
    isExecuting.value = false
  }, 1200)
}

function resetAll() {
  sourceScale.value = 'md'
  targetScale.value = 'phase_field'
  selectedMethod.value = 'quasicontinuum'
  cgResult.value = null
  recommendations.value = []
  isRecommending.value = false
  isExecuting.value = false

  params.cutoff_radius = 5.0
  params.grid_resolution = 0.5
  params.filter_cutoff = 0.5
  params.n_clusters = 100
  params.n_bins = 50
  params.n_representative = 500
}

// ============ Lifecycle ============

onMounted(() => {
  nextTick(() => {
    // Pre-load recommendation for demo
    const cachedMethod = selectedMethod.value
    getRecommendation()
    // Restore selected method (recommendation doesn't change it)
    selectedMethod.value = cachedMethod
  })
})
</script>
