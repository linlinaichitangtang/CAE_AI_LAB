<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">相场后处理分析</h2>
        <p class="text-sm" style="color: var(--text-muted)">晶粒统计 / 应力场 / 场可视化，相场-力学耦合</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="handleAnalyzeGrains" :disabled="analyzingGrains" class="btn btn-primary text-xs">
          <span v-if="analyzingGrains" class="mr-1 animate-spin">&#10227;</span>
          {{ analyzingGrains ? '分析中...' : '分析晶粒' }}
        </button>
        <button @click="handleRunMechanical" :disabled="runningMechanical" class="btn btn-primary text-xs">
          <span v-if="runningMechanical" class="mr-1 animate-spin">&#10227;</span>
          {{ runningMechanical ? '计算中...' : '运行力学耦合' }}
        </button>
        <button @click="handleExportCSV" class="btn btn-ghost text-xs">导出CSV</button>
      </div>
    </div>

    <!-- Tab Switcher -->
    <div class="flex px-4 pt-3 gap-1" style="background: var(--bg-surface)">
      <button
        v-for="tab in tabs"
        :key="tab.key"
        @click="activeTab = tab.key"
        class="px-4 py-2 text-xs font-medium rounded-t transition"
        :style="activeTab === tab.key
          ? 'background: var(--bg-base); color: var(--primary); border-bottom: 2px solid var(--primary)'
          : 'color: var(--text-muted); border-bottom: 2px solid transparent'"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Controls -->
      <div class="w-72 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- ===== 场可视化控制 ===== -->
        <template v-if="activeTab === 'visualization'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">场选择</h4>
            <select v-model="vizConfig.field" class="input w-full text-xs">
              <option v-for="f in fieldOptions" :key="f.value" :value="f.value">{{ f.label }}</option>
            </select>
          </div>

          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">色谱选择</h4>
            <select v-model="vizConfig.colormap" class="input w-full text-xs">
              <option v-for="c in colormapOptions" :key="c.value" :value="c.value">{{ c.label }}</option>
            </select>
          </div>

          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">等高线级别</h4>
            <input v-model.number="vizConfig.contour_levels" type="number" min="0" max="20" step="1" class="input w-full text-xs" />
          </div>

          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">晶界叠加</h4>
            <label class="flex items-center gap-2 cursor-pointer">
              <input type="checkbox" v-model="vizConfig.show_grain_boundaries" class="accent-blue-500" />
              <span class="text-xs" style="color: var(--text-secondary)">显示晶界</span>
            </label>
          </div>

          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">Z 切片</h4>
            <input v-model.number="vizConfig.slice_z" type="range" min="0" :max="gridSize - 1" step="1" class="w-full" />
            <div class="text-xs mt-1" style="color: var(--text-muted)">当前: {{ vizConfig.slice_z }}</div>
          </div>

          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">等值面值</h4>
            <input v-model.number="vizConfig.isosurface_value" type="number" step="0.05" min="0" max="1" class="input w-full text-xs" />
          </div>
        </template>

        <!-- ===== 晶粒统计控制 ===== -->
        <template v-if="activeTab === 'grain-stats'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">统计概览</h4>
            <div class="space-y-2">
              <div class="flex items-center justify-between">
                <span class="text-xs" style="color: var(--text-secondary)">晶粒数</span>
                <span class="text-xs font-mono font-semibold" style="color: var(--text-primary)">{{ grainResult?.num_grains ?? '-' }}</span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-xs" style="color: var(--text-secondary)">平均尺寸</span>
                <span class="text-xs font-mono" style="color: var(--text-primary)">{{ grainResult ? grainResult.avg_grain_size.toFixed(2) : '-' }}</span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-xs" style="color: var(--text-secondary)">最大尺寸</span>
                <span class="text-xs font-mono" style="color: var(--text-primary)">{{ grainResult?.size_distribution.max.toFixed(2) ?? '-' }}</span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-xs" style="color: var(--text-secondary)">最小尺寸</span>
                <span class="text-xs font-mono" style="color: var(--text-primary)">{{ grainResult?.size_distribution.min.toFixed(2) ?? '-' }}</span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-xs" style="color: var(--text-secondary)">晶界总长</span>
                <span class="text-xs font-mono" style="color: var(--text-primary)">{{ grainResult ? grainResult.grain_boundary_length.toFixed(1) : '-' }}</span>
              </div>
            </div>
          </div>
        </template>

        <!-- ===== 力学耦合控制 ===== -->
        <template v-if="activeTab === 'mechanical'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">材料属性</h4>
            <div class="space-y-2">
              <div>
                <label class="label">弹性模量 E (GPa)</label>
                <input v-model.number="mechConfig.elastic_modulus_E" type="number" step="10" min="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">泊松比 v</label>
                <input v-model.number="mechConfig.poisson_ratio_nu" type="number" step="0.01" min="0" max="0.5" class="input w-full text-xs" />
              </div>
            </div>
          </div>

          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">本征应变</h4>
            <div class="space-y-2">
              <div>
                <label class="label">本征应变类型</label>
                <select v-model="mechConfig.eigenstrain_type" class="input w-full text-xs">
                  <option value="misfit_strain">沉淀错配</option>
                  <option value="phase_transformation">相变</option>
                  <option value="elastic_energy">热膨胀</option>
                </select>
              </div>
              <div>
                <label class="label">错配应变大小</label>
                <input v-model.number="mechConfig.misfit_strain_magnitude" type="number" step="0.001" min="0" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">耦合强度</label>
                <input v-model.number="mechConfig.coupling_strength" type="number" step="0.1" min="0" max="10" class="input w-full text-xs" />
              </div>
            </div>
          </div>

          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">外加应力</h4>
            <div class="space-y-2">
              <div class="grid grid-cols-3 gap-2">
                <div>
                  <label class="label">sigma_xx</label>
                  <input v-model.number="mechConfig.external_stress.xx" type="number" step="10" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">sigma_yy</label>
                  <input v-model.number="mechConfig.external_stress.yy" type="number" step="10" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">tau_xy</label>
                  <input v-model.number="mechConfig.external_stress.xy" type="number" step="10" class="input w-full text-xs" />
                </div>
              </div>
            </div>
          </div>

          <div class="panel-section" v-if="mechResult">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">结果统计</h4>
            <div class="space-y-2">
              <div class="flex items-center justify-between">
                <span class="text-xs" style="color: var(--text-secondary)">最大应力 (MPa)</span>
                <span class="text-xs font-mono font-semibold" style="color: var(--accent-red)">{{ mechResult.max_stress.toFixed(2) }}</span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-xs" style="color: var(--text-secondary)">总弹性能 (J)</span>
                <span class="text-xs font-mono" style="color: var(--text-primary)">{{ mechResult.total_elastic_energy.toExponential(3) }}</span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-xs" style="color: var(--text-secondary)">von Mises 最大值</span>
                <span class="text-xs font-mono" style="color: var(--accent-yellow)">{{ computeVonMisesMax().toFixed(2) }}</span>
              </div>
            </div>
          </div>
        </template>
      </div>

      <!-- Right Panel: Visualization Area -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4">

        <!-- ===== 场可视化面板 ===== -->
        <template v-if="activeTab === 'visualization'">
          <div class="relative" style="background: var(--bg-elevated); border-radius: var(--radius-md); border: 1px solid var(--border-subtle)">
            <canvas ref="vizCanvas" class="w-full" style="height: 420px; border-radius: var(--radius-md)"></canvas>
          </div>
          <!-- Color Legend -->
          <div class="flex items-center gap-3 px-2">
            <span class="text-xs" style="color: var(--text-muted)">色标:</span>
            <div class="flex-1 h-4 rounded" :style="{ background: legendGradient }"></div>
            <div class="flex justify-between w-full">
              <span class="text-[10px] font-mono" style="color: var(--text-muted)">{{ vizMinVal.toFixed(3) }}</span>
              <span class="text-[10px] font-mono" style="color: var(--text-muted)">{{ vizMaxVal.toFixed(3) }}</span>
            </div>
          </div>
        </template>

        <!-- ===== 晶粒统计面板 ===== -->
        <template v-if="activeTab === 'grain-stats'">
          <!-- Grain Size Distribution Histogram -->
          <div class="panel-section" style="padding: 16px">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">晶粒尺寸分布</h4>
            <svg ref="sizeHistSvg" viewBox="0 0 600 200" class="w-full" style="height: 200px">
              <rect x="0" y="0" width="600" height="200" fill="transparent" />
              <!-- Axes -->
              <line x1="50" y1="180" x2="580" y2="180" stroke="var(--border-default)" stroke-width="1" />
              <line x1="50" y1="10" x2="50" y2="180" stroke="var(--border-default)" stroke-width="1" />
              <!-- Bars -->
              <rect
                v-for="(bar, idx) in sizeHistBars"
                :key="idx"
                :x="bar.x"
                :y="bar.y"
                :width="bar.w"
                :height="bar.h"
                fill="var(--primary)"
                opacity="0.8"
                rx="2"
              />
              <!-- X Labels -->
              <text
                v-for="(lbl, idx) in sizeHistXLabels"
                :key="'xl' + idx"
                :x="lbl.x"
                :y="195"
                text-anchor="middle"
                fill="var(--text-muted)"
                font-size="9"
              >{{ lbl.text }}</text>
              <!-- Y Labels -->
              <text x="45" y="15" text-anchor="end" fill="var(--text-muted)" font-size="9">{{ sizeHistYMax }}</text>
              <text x="45" y="95" text-anchor="end" fill="var(--text-muted)" font-size="9">{{ Math.round(sizeHistYMax / 2) }}</text>
              <text x="45" y="183" text-anchor="end" fill="var(--text-muted)" font-size="9">0</text>
              <!-- Axis titles -->
              <text x="315" y="210" text-anchor="middle" fill="var(--text-secondary)" font-size="10">尺寸 (um)</text>
              <text x="12" y="100" text-anchor="middle" fill="var(--text-secondary)" font-size="10" transform="rotate(-90, 12, 100)">频次</text>
            </svg>
          </div>

          <!-- Orientation Distribution -->
          <div class="panel-section" style="padding: 16px">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">取向分布</h4>
            <svg ref="orientSvg" viewBox="0 0 600 200" class="w-full" style="height: 200px">
              <rect x="0" y="0" width="600" height="200" fill="transparent" />
              <line x1="50" y1="180" x2="580" y2="180" stroke="var(--border-default)" stroke-width="1" />
              <line x1="50" y1="10" x2="50" y2="180" stroke="var(--border-default)" stroke-width="1" />
              <rect
                v-for="(bar, idx) in orientHistBars"
                :key="idx"
                :x="bar.x"
                :y="bar.y"
                :width="bar.w"
                :height="bar.h"
                fill="var(--accent-green)"
                opacity="0.8"
                rx="2"
              />
              <text
                v-for="(lbl, idx) in orientHistXLabels"
                :key="'ol' + idx"
                :x="lbl.x"
                :y="195"
                text-anchor="middle"
                fill="var(--text-muted)"
                font-size="9"
              >{{ lbl.text }}</text>
              <text x="45" y="15" text-anchor="end" fill="var(--text-muted)" font-size="9">{{ orientHistYMax }}</text>
              <text x="45" y="95" text-anchor="end" fill="var(--text-muted)" font-size="9">{{ Math.round(orientHistYMax / 2) }}</text>
              <text x="45" y="183" text-anchor="end" fill="var(--text-muted)" font-size="9">0</text>
              <text x="315" y="210" text-anchor="middle" fill="var(--text-secondary)" font-size="10">取向角 (deg)</text>
              <text x="12" y="100" text-anchor="middle" fill="var(--text-secondary)" font-size="10" transform="rotate(-90, 12, 100)">频次</text>
            </svg>
          </div>

          <!-- Grain Table -->
          <div class="panel-section" style="padding: 16px">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">晶粒列表</h4>
            <div class="overflow-x-auto" style="max-height: 280px; overflow-y: auto">
              <table class="w-full text-xs">
                <thead>
                  <tr style="border-bottom: 1px solid var(--border-default)">
                    <th class="text-left py-2 px-2" style="color: var(--text-muted)">ID</th>
                    <th class="text-right py-2 px-2" style="color: var(--text-muted)">面积</th>
                    <th class="text-right py-2 px-2" style="color: var(--text-muted)">直径</th>
                    <th class="text-right py-2 px-2" style="color: var(--text-muted)">取向</th>
                    <th class="text-right py-2 px-2" style="color: var(--text-muted)">圆度</th>
                    <th class="text-right py-2 px-2" style="color: var(--text-muted)">长宽比</th>
                  </tr>
                </thead>
                <tbody>
                  <tr
                    v-for="g in (grainResult?.grains ?? []).slice(0, 50)"
                    :key="g.grain_id"
                    style="border-bottom: 1px solid var(--border-subtle)"
                  >
                    <td class="py-1.5 px-2 font-mono" style="color: var(--text-primary)">{{ g.grain_id }}</td>
                    <td class="py-1.5 px-2 text-right font-mono" style="color: var(--text-secondary)">{{ g.area.toFixed(1) }}</td>
                    <td class="py-1.5 px-2 text-right font-mono" style="color: var(--text-secondary)">{{ g.equivalent_diameter.toFixed(2) }}</td>
                    <td class="py-1.5 px-2 text-right font-mono" style="color: var(--text-secondary)">{{ g.orientation.toFixed(1) }}</td>
                    <td class="py-1.5 px-2 text-right font-mono" style="color: var(--text-secondary)">{{ g.circularity.toFixed(3) }}</td>
                    <td class="py-1.5 px-2 text-right font-mono" style="color: var(--text-secondary)">{{ g.aspect_ratio.toFixed(3) }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          <!-- Grain Size vs Time -->
          <div class="panel-section" style="padding: 16px">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">晶粒尺寸 vs 时间</h4>
            <svg ref="sizeTimeSvg" viewBox="0 0 600 200" class="w-full" style="height: 200px">
              <rect x="0" y="0" width="600" height="200" fill="transparent" />
              <line x1="50" y1="180" x2="580" y2="180" stroke="var(--border-default)" stroke-width="1" />
              <line x1="50" y1="10" x2="50" y2="180" stroke="var(--border-default)" stroke-width="1" />
              <!-- Grid lines -->
              <line v-for="i in 4" :key="'gl' + i" x1="50" :y1="10 + i * 42.5" x2="580" :y2="10 + i * 42.5" stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
              <!-- Line plot -->
              <polyline
                :points="sizeTimeLinePoints"
                fill="none"
                stroke="var(--accent-yellow)"
                stroke-width="2"
              />
              <!-- Dots -->
              <circle
                v-for="(pt, idx) in sizeTimeDataPoints"
                :key="'st' + idx"
                :cx="pt.x"
                :cy="pt.y"
                r="3"
                fill="var(--accent-yellow)"
              />
              <!-- X Labels -->
              <text
                v-for="(lbl, idx) in sizeTimeXLabels"
                :key="'stl' + idx"
                :x="lbl.x"
                :y="195"
                text-anchor="middle"
                fill="var(--text-muted)"
                font-size="9"
              >{{ lbl.text }}</text>
              <text x="315" y="210" text-anchor="middle" fill="var(--text-secondary)" font-size="10">时间 (s)</text>
              <text x="12" y="100" text-anchor="middle" fill="var(--text-secondary)" font-size="10" transform="rotate(-90, 12, 100)">尺寸 (um)</text>
            </svg>
          </div>
        </template>

        <!-- ===== 力学耦合面板 ===== -->
        <template v-if="activeTab === 'mechanical'">
          <!-- Stress Component Selector -->
          <div class="flex items-center gap-2 mb-2">
            <span class="text-xs" style="color: var(--text-muted)">应力分量:</span>
            <button
              v-for="comp in stressComponents"
              :key="comp.value"
              @click="activeStressComponent = comp.value"
              class="px-3 py-1 text-xs rounded transition"
              :style="activeStressComponent === comp.value
                ? 'background: var(--primary); color: #fff'
                : 'background: var(--bg-elevated); color: var(--text-secondary); border: 1px solid var(--border-default)'"
            >{{ comp.label }}</button>
          </div>

          <!-- Stress Field Canvas -->
          <div class="relative" style="background: var(--bg-elevated); border-radius: var(--radius-md); border: 1px solid var(--border-subtle)">
            <canvas ref="stressCanvas" class="w-full" style="height: 360px; border-radius: var(--radius-md)"></canvas>
          </div>

          <!-- Elastic Energy Density Canvas -->
          <div class="mt-4">
            <h4 class="text-sm font-medium mb-2" style="color: var(--text-primary)">弹性应变能密度</h4>
            <div class="relative" style="background: var(--bg-elevated); border-radius: var(--radius-md); border: 1px solid var(--border-subtle)">
              <canvas ref="energyCanvas" class="w-full" style="height: 240px; border-radius: var(--radius-md)"></canvas>
            </div>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick, watch } from 'vue'
import type {
  FieldType,
  ColormapType,
  StressCouplingType,
  GrainData,
  GrainSizeDistribution,
  OrientationDistribution,
  PfMechanicalConfig,
  PfMechanicalResult,
  PfGrainAnalysisResult,
  FieldVisualizationConfig,
} from '../api/phaseFieldPostProcess'

// ============ Constants ============

const tabs = [
  { key: 'visualization' as const, label: '场可视化' },
  { key: 'grain-stats' as const, label: '晶粒统计' },
  { key: 'mechanical' as const, label: '力学耦合' },
]

const fieldOptions: Array<{ value: FieldType; label: string }> = [
  { value: 'order_parameter', label: '序参量' },
  { value: 'temperature', label: '温度' },
  { value: 'stress', label: '应力' },
  { value: 'free_energy', label: '自由能' },
  { value: 'grain_id', label: '晶粒ID' },
  { value: 'strain', label: '应变' },
]

const colormapOptions: Array<{ value: ColormapType; label: string }> = [
  { value: 'viridis', label: 'Viridis' },
  { value: 'jet', label: 'Jet' },
  { value: 'coolwarm', label: 'Coolwarm' },
  { value: 'grayscale', label: 'Grayscale' },
  { value: 'rainbow', label: 'Rainbow' },
]

const stressComponents = [
  { value: 'xx' as const, label: 'sigma_xx' },
  { value: 'yy' as const, label: 'sigma_yy' },
  { value: 'xy' as const, label: 'tau_xy' },
  { value: 'von_mises' as const, label: 'von Mises' },
]

// ============ State ============

const activeTab = ref<'visualization' | 'grain-stats' | 'mechanical'>('visualization')
const analyzingGrains = ref(false)
const runningMechanical = ref(false)
const gridSize = 128

const vizCanvas = ref<HTMLCanvasElement | null>(null)
const stressCanvas = ref<HTMLCanvasElement | null>(null)
const energyCanvas = ref<HTMLCanvasElement | null>(null)
const sizeHistSvg = ref<SVGSVGElement | null>(null)
const orientSvg = ref<SVGSVGElement | null>(null)
const sizeTimeSvg = ref<SVGSVGElement | null>(null)

const vizConfig = reactive<FieldVisualizationConfig>({
  field: 'order_parameter',
  colormap: 'viridis',
  contour_levels: 5,
  show_grain_boundaries: true,
  slice_z: 0,
  isosurface_value: 0.5,
})

const mechConfig = reactive<PfMechanicalConfig>({
  elastic_modulus_E: 200,
  poisson_ratio_nu: 0.3,
  eigenstrain_type: 'misfit_strain',
  misfit_strain_magnitude: 0.01,
  coupling_strength: 1.0,
  external_stress: { xx: 0, yy: 0, xy: 0 },
})

const activeStressComponent = ref<'xx' | 'yy' | 'xy' | 'von_mises'>('xx')

// Generated data
const orderParamField = ref<number[][]>([])
const grainBoundaryField = ref<number[][]>([])
const grainResult = ref<PfGrainAnalysisResult | null>(null)
const mechResult = ref<PfMechanicalResult | null>(null)

// ============ Colormap Functions ============

function viridis(t: number): [number, number, number] {
  const r = Math.max(0, Math.min(1, -0.37 + 3.5 * t - 2.5 * t * t))
  const g = Math.max(0, Math.min(1, 0.05 + 1.8 * t - 0.8 * t * t))
  const b = Math.max(0, Math.min(1, 0.33 + 0.7 * t - 1.3 * t * t))
  return [Math.round(r * 255), Math.round(g * 255), Math.round(b * 255)]
}

function jet(t: number): [number, number, number] {
  const r = Math.max(0, Math.min(1, 1.5 - Math.abs(4 * t - 3)))
  const g = Math.max(0, Math.min(1, 1.5 - Math.abs(4 * t - 2)))
  const b = Math.max(0, Math.min(1, 1.5 - Math.abs(4 * t - 1)))
  return [Math.round(r * 255), Math.round(g * 255), Math.round(b * 255)]
}

function coolwarm(t: number): [number, number, number] {
  const r = Math.max(0, Math.min(1, 0.5 + 1.5 * (t - 0.5)))
  const b = Math.max(0, Math.min(1, 0.5 + 1.5 * (0.5 - t)))
  const g = 1 - Math.abs(t - 0.5) * 1.5
  return [Math.round(r * 255), Math.round(Math.max(0, g) * 255), Math.round(b * 255)]
}

function grayscale(t: number): [number, number, number] {
  const v = Math.round(t * 255)
  return [v, v, v]
}

function rainbow(t: number): [number, number, number] {
  const h = (1 - t) * 270
  const s = 1.0
  const l = 0.5
  const c = (1 - Math.abs(2 * l - 1)) * s
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1))
  const m = l - c / 2
  let r = 0, g = 0, b = 0
  if (h < 60) { r = c; g = x; }
  else if (h < 120) { r = x; g = c; }
  else if (h < 180) { g = c; b = x; }
  else if (h < 240) { g = x; b = c; }
  else if (h < 300) { r = x; b = c; }
  else { r = c; b = x; }
  return [Math.round((r + m) * 255), Math.round((g + m) * 255), Math.round((b + m) * 255)]
}

function applyColormap(t: number, cmap: ColormapType): [number, number, number] {
  const tc = Math.max(0, Math.min(1, t))
  switch (cmap) {
    case 'viridis': return viridis(tc)
    case 'jet': return jet(tc)
    case 'coolwarm': return coolwarm(tc)
    case 'grayscale': return grayscale(tc)
    case 'rainbow': return rainbow(tc)
    default: return viridis(tc)
  }
}

// ============ Voronoi Grain Generation ============

interface VoronoiSeed {
  x: number
  y: number
  id: number
  orderParam: number
  orientation: number
}

function generateVoronoiField(n: number, numSeeds: number): { field: number[][]; seeds: VoronoiSeed[] } {
  const seeds: VoronoiSeed[] = []
  for (let i = 0; i < numSeeds; i++) {
    seeds.push({
      x: Math.random() * n,
      y: Math.random() * n,
      id: i,
      orderParam: 0.2 + Math.random() * 0.8,
      orientation: Math.random() * 180,
    })
  }

  const field: number[][] = []
  for (let iy = 0; iy < n; iy++) {
    const row: number[] = []
    for (let ix = 0; ix < n; ix++) {
      let minDist = Infinity
      let closestSeed = seeds[0]
      for (const s of seeds) {
        const dx = ix - s.x
        const dy = iy - s.y
        const d = dx * dx + dy * dy
        if (d < minDist) {
          minDist = d
          closestSeed = s
        }
      }
      // Smooth interface using distance to second closest
      let secondDist = Infinity
      for (const s of seeds) {
        if (s.id === closestSeed.id) continue
        const dx = ix - s.x
        const dy = iy - s.y
        const d = dx * dx + dy * dy
        if (d < secondDist) secondDist = d
      }
      const interfaceWidth = 2.5
      const eta = 0.5 * (1 - Math.tanh((Math.sqrt(minDist) - Math.sqrt(secondDist)) / interfaceWidth))
      row.push(eta * closestSeed.orderParam + (1 - eta) * 0.1)
    }
    field.push(row)
  }
  return { field, seeds }
}

function computeGrainBoundaries(field: number[][], threshold: number): number[][] {
  const n = field.length
  const boundaries: number[][] = []
  for (let iy = 0; iy < n; iy++) {
    const row: number[] = []
    for (let ix = 0; ix < n; ix++) {
      let isBoundary = 0
      if (ix > 0 && Math.abs(field[iy][ix] - field[iy][ix - 1]) > threshold) isBoundary = 1
      if (ix < n - 1 && Math.abs(field[iy][ix] - field[iy][ix + 1]) > threshold) isBoundary = 1
      if (iy > 0 && Math.abs(field[iy][ix] - field[iy - 1][ix]) > threshold) isBoundary = 1
      if (iy < n - 1 && Math.abs(field[iy][ix] - field[iy + 1][ix]) > threshold) isBoundary = 1
      row.push(isBoundary)
    }
    boundaries.push(row)
  }
  return boundaries
}

function extractGrainsFromField(field: number[][], seeds: VoronoiSeed[]): GrainData[] {
  const n = field.length
  const grainMap = new Map<number, { pixels: Array<{ x: number; y: number }>; sumX: number; sumY: number }>()

  for (let iy = 0; iy < n; iy++) {
    for (let ix = 0; ix < n; ix++) {
      let minDist = Infinity
      let closestId = 0
      for (const s of seeds) {
        const dx = ix - s.x
        const dy = iy - s.y
        const d = dx * dx + dy * dy
        if (d < minDist) { minDist = d; closestId = s.id }
      }
      if (!grainMap.has(closestId)) grainMap.set(closestId, { pixels: [], sumX: 0, sumY: 0 })
      const g = grainMap.get(closestId)!
      g.pixels.push({ x: ix, y: iy })
      g.sumX += ix
      g.sumY += iy
    }
  }

  const grains: GrainData[] = []
  for (const [id, data] of grainMap) {
    const area = data.pixels.length
    if (area < 20) continue
    const perimeter = computePerimeter(data.pixels, n)
    const diameter = 2 * Math.sqrt(area / Math.PI)
    const seed = seeds.find(s => s.id === id)
    grains.push({
      grain_id: id,
      area,
      perimeter,
      equivalent_diameter: diameter,
      orientation: seed?.orientation ?? Math.random() * 180,
      centroid: { x: data.sumX / area, y: data.sumY / area },
      aspect_ratio: 0.6 + Math.random() * 0.8,
      circularity: (4 * Math.PI * area) / (perimeter * perimeter),
    })
  }
  return grains
}

function computePerimeter(pixels: Array<{ x: number; y: number }>, n: number): number {
  const set = new Set(pixels.map(p => p.y * n + p.x))
  let perimeter = 0
  for (const p of pixels) {
    const neighbors = [[0, 1], [0, -1], [1, 0], [-1, 0]]
    for (const [dx, dy] of neighbors) {
      const nx = p.x + dx
      const ny = p.y + dy
      if (nx < 0 || nx >= n || ny < 0 || ny >= n || !set.has(ny * n + nx)) {
        perimeter++
      }
    }
  }
  return perimeter
}

function computeSizeDistribution(grains: GrainData[]): GrainSizeDistribution {
  const sizes = grains.map(g => g.equivalent_diameter).sort((a, b) => a - b)
  if (sizes.length === 0) return { bins: [], mean: 0, median: 0, std_dev: 0, max: 0, min: 0 }

  const min = sizes[0]
  const max = sizes[sizes.length - 1]
  const mean = sizes.reduce((s, v) => s + v, 0) / sizes.length
  const median = sizes.length % 2 === 0
    ? (sizes[sizes.length / 2 - 1] + sizes[sizes.length / 2]) / 2
    : sizes[Math.floor(sizes.length / 2)]
  const variance = sizes.reduce((s, v) => s + (v - mean) ** 2, 0) / sizes.length
  const std_dev = Math.sqrt(variance)

  const numBins = Math.min(12, Math.max(5, Math.ceil(Math.sqrt(sizes.length))))
  const binWidth = (max - min) / numBins || 1
  const bins = []
  for (let i = 0; i < numBins; i++) {
    const size_min = min + i * binWidth
    const size_max = min + (i + 1) * binWidth
    const count = sizes.filter(s => s >= size_min && (i === numBins - 1 ? s <= size_max : s < size_max)).length
    bins.push({ size_min, size_max, count })
  }

  return { bins, mean, median, std_dev, max, min }
}

function computeOrientationDistribution(grains: GrainData[]): OrientationDistribution {
  const numBins = 18
  const binWidth = 180 / numBins
  const bins = []
  let maxCount = 0
  let preferredAngle = 0

  for (let i = 0; i < numBins; i++) {
    const angle_min = i * binWidth
    const angle_max = (i + 1) * binWidth
    const count = grains.filter(g => g.orientation >= angle_min && g.orientation < angle_max).length
    bins.push({ angle_min, angle_max, count })
    if (count > maxCount) {
      maxCount = count
      preferredAngle = (angle_min + angle_max) / 2
    }
  }

  const totalGrains = grains.length || 1
  const texture_strength = maxCount / totalGrains

  return { bins, preferred_orientation: preferredAngle, texture_strength }
}

function generateGrainSizeVsTime(numPoints: number): Array<{ time: number; size: number }> {
  const data: Array<{ time: number; size: number }> = []
  let size = 5 + Math.random() * 3
  for (let i = 0; i < numPoints; i++) {
    const t = i * 0.5
    size += 0.3 + Math.random() * 0.5
    data.push({ time: t, size })
  }
  return data
}

// ============ Mechanical Coupling Simulation ============

function computeMechanicalResult(field: number[][]): PfMechanicalResult {
  const n = field.length
  const E = mechConfig.elastic_modulus_E * 1e3 // GPa -> MPa
  const eps0 = mechConfig.misfit_strain_magnitude
  const coupling = mechConfig.coupling_strength
  const extStress = mechConfig.external_stress

  const stressXX: number[][] = []
  const stressYY: number[][] = []
  const stressXY: number[][] = []
  const stressZZ: number[][] = []
  const stressYZ: number[][] = []
  const stressXZ: number[][] = []
  const vonMises: number[][] = []
  const energyDensity: number[][] = []

  let maxStress = 0
  let totalEnergy = 0

  for (let iy = 0; iy < n; iy++) {
    const sxxRow: number[] = []
    const syyRow: number[] = []
    const sxyRow: number[] = []
    const szzRow: number[] = []
    const syzRow: number[] = []
    const sxzRow: number[] = []
    const vmRow: number[] = []
    const edRow: number[] = []

    for (let ix = 0; ix < n; ix++) {
      const eta = field[iy][ix]
      // Simplified stress: sigma = E * eps_misfit * eta * coupling + external
      const sxx = E * eps0 * eta * coupling + extStress.xx
      const syy = E * eps0 * eta * coupling * 0.85 + extStress.yy
      const sxy = E * eps0 * eta * coupling * 0.1 + extStress.xy
      const szz = E * eps0 * eta * coupling * 0.3
      const syz = E * eps0 * eta * coupling * 0.05
      const sxz = E * eps0 * eta * coupling * 0.05

      // von Mises
      const vm = Math.sqrt(
        0.5 * ((sxx - syy) ** 2 + (syy - szz) ** 2 + (szz - sxx) ** 2 + 6 * (sxy * sxy + syz * syz + sxz * sxz))
      )

      // Elastic energy density = 0.5 * sigma * epsilon
      const ed = 0.5 * (sxx * eps0 * eta + syy * eps0 * eta * 0.85)

      sxxRow.push(sxx)
      syyRow.push(syy)
      sxyRow.push(sxy)
      szzRow.push(szz)
      syzRow.push(syz)
      sxzRow.push(sxz)
      vmRow.push(vm)
      edRow.push(ed)

      if (Math.abs(sxx) > maxStress) maxStress = Math.abs(sxx)
      if (vm > maxStress) maxStress = vm
      totalEnergy += ed
    }

    stressXX.push(sxxRow)
    stressYY.push(syyRow)
    stressXY.push(sxyRow)
    stressZZ.push(szzRow)
    stressYZ.push(syzRow)
    stressXZ.push(sxzRow)
    vonMises.push(vmRow)
    energyDensity.push(edRow)
  }

  // Strain field (simplified)
  const strainXX = stressXX.map(row => row.map(v => v / E))
  const strainYY = stressYY.map(row => row.map(v => v / E))
  const strainXY = stressXY.map(row => row.map(v => v / (E * 2 * (1 + mechConfig.poisson_ratio_nu))))
  const strainZZ = stressZZ.map(row => row.map(v => -mechConfig.poisson_ratio_nu * v / E))
  const strainYZ = stressYZ.map(row => row.map(() => 0))
  const strainXZ = stressXZ.map(row => row.map(() => 0))

  return {
    success: true,
    stress_field: { xx: stressXX, yy: stressYY, zz: stressZZ, xy: stressXY, yz: stressYZ, xz: stressXZ },
    strain_field: { xx: strainXX, yy: strainYY, zz: strainZZ, xy: strainXY, yz: strainYZ, xz: strainXZ },
    elastic_energy_density: energyDensity,
    total_elastic_energy: totalEnergy,
    max_stress: maxStress,
    von_mises_field: vonMises,
  }
}

// ============ Computed ============

const vizMinVal = computed(() => {
  if (orderParamField.value.length === 0) return 0
  let min = Infinity
  for (const row of orderParamField.value) for (const v of row) if (v < min) min = v
  return min
})

const vizMaxVal = computed(() => {
  if (orderParamField.value.length === 0) return 1
  let max = -Infinity
  for (const row of orderParamField.value) for (const v of row) if (v > max) max = v
  return max
})

const legendGradient = computed(() => {
  const steps = 10
  const colors: string[] = []
  for (let i = 0; i <= steps; i++) {
    const t = i / steps
    const [r, g, b] = applyColormap(t, vizConfig.colormap)
    colors.push(`rgb(${r},${g},${b}) ${t * 100}%`)
  }
  return `linear-gradient(to right, ${colors.join(', ')})`
})

// Size histogram bars
const sizeHistBars = computed(() => {
  const dist = grainResult.value?.size_distribution
  if (!dist || dist.bins.length === 0) return []
  const maxCount = Math.max(...dist.bins.map(b => b.count), 1)
  const plotW = 530
  const plotH = 160
  const barW = plotW / dist.bins.length - 2
  return dist.bins.map((bin, i) => ({
    x: 50 + i * (plotW / dist.bins.length) + 1,
    y: 180 - (bin.count / maxCount) * plotH,
    w: barW,
    h: (bin.count / maxCount) * plotH,
  }))
})

const sizeHistXLabels = computed(() => {
  const dist = grainResult.value?.size_distribution
  if (!dist || dist.bins.length === 0) return []
  const plotW = 530
  const step = plotW / dist.bins.length
  return dist.bins.map((bin, i) => ({
    x: 50 + i * step + step / 2,
    text: bin.size_min.toFixed(1),
  }))
})

const sizeHistYMax = computed(() => {
  const dist = grainResult.value?.size_distribution
  if (!dist || dist.bins.length === 0) return 0
  return Math.max(...dist.bins.map(b => b.count))
})

// Orientation histogram bars
const orientHistBars = computed(() => {
  const dist = grainResult.value?.orientation_distribution
  if (!dist || dist.bins.length === 0) return []
  const maxCount = Math.max(...dist.bins.map(b => b.count), 1)
  const plotW = 530
  const plotH = 160
  const barW = plotW / dist.bins.length - 2
  return dist.bins.map((bin, i) => ({
    x: 50 + i * (plotW / dist.bins.length) + 1,
    y: 180 - (bin.count / maxCount) * plotH,
    w: barW,
    h: (bin.count / maxCount) * plotH,
  }))
})

const orientHistXLabels = computed(() => {
  const dist = grainResult.value?.orientation_distribution
  if (!dist || dist.bins.length === 0) return []
  const plotW = 530
  const step = plotW / dist.bins.length
  return dist.bins.map((bin, i) => ({
    x: 50 + i * step + step / 2,
    text: bin.angle_min.toFixed(0),
  }))
})

const orientHistYMax = computed(() => {
  const dist = grainResult.value?.orientation_distribution
  if (!dist || dist.bins.length === 0) return 0
  return Math.max(...dist.bins.map(b => b.count))
})

// Size vs time chart
const sizeTimeLinePoints = computed(() => {
  const data = grainResult.value?.grain_size_vs_time
  if (!data || data.length === 0) return ''
  const maxT = Math.max(...data.map(d => d.time))
  const minT = Math.min(...data.map(d => d.time))
  const maxS = Math.max(...data.map(d => d.size))
  const minS = Math.min(...data.map(d => d.size))
  const rangeT = maxT - minT || 1
  const rangeS = maxS - minS || 1
  const plotW = 530
  const plotH = 160
  return data.map(d => {
    const px = 50 + ((d.time - minT) / rangeT) * plotW
    const py = 180 - ((d.size - minS) / rangeS) * plotH
    return `${px},${py}`
  }).join(' ')
})

const sizeTimeDataPoints = computed(() => {
  const data = grainResult.value?.grain_size_vs_time
  if (!data || data.length === 0) return []
  const maxT = Math.max(...data.map(d => d.time))
  const minT = Math.min(...data.map(d => d.time))
  const maxS = Math.max(...data.map(d => d.size))
  const minS = Math.min(...data.map(d => d.size))
  const rangeT = maxT - minT || 1
  const rangeS = maxS - minS || 1
  const plotW = 530
  const plotH = 160
  return data.map(d => ({
    x: 50 + ((d.time - minT) / rangeT) * plotW,
    y: 180 - ((d.size - minS) / rangeS) * plotH,
  }))
})

const sizeTimeXLabels = computed(() => {
  const data = grainResult.value?.grain_size_vs_time
  if (!data || data.length === 0) return []
  const maxT = Math.max(...data.map(d => d.time))
  const minT = Math.min(...data.map(d => d.time))
  const plotW = 530
  const numLabels = Math.min(6, data.length)
  const labels = []
  for (let i = 0; i < numLabels; i++) {
    const t = minT + (maxT - minT) * (i / (numLabels - 1))
    labels.push({ x: 50 + (i / (numLabels - 1)) * plotW, text: t.toFixed(1) })
  }
  return labels
})

function computeVonMisesMax(): number {
  if (!mechResult.value) return 0
  let max = 0
  for (const row of mechResult.value.von_mises_field) {
    for (const v of row) if (v > max) max = v
  }
  return max
}

// ============ Canvas Drawing ============

function drawFieldOnCanvas(
  canvas: HTMLCanvasElement,
  field: number[][],
  colormap: ColormapType,
  boundaries: number[][] | null,
  title: string
) {
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
  const n = field.length
  if (n === 0) return

  // Find min/max
  let fMin = Infinity
  let fMax = -Infinity
  for (const row of field) {
    for (const v of row) {
      if (v < fMin) fMin = v
      if (v > fMax) fMax = v
    }
  }
  const range = fMax - fMin || 1

  // Draw heatmap using ImageData
  const imgData = ctx.createImageData(n, n)
  for (let iy = 0; iy < n; iy++) {
    for (let ix = 0; ix < n; ix++) {
      const t = (field[iy][ix] - fMin) / range
      const [r, g, b] = applyColormap(t, colormap)
      const idx = (iy * n + ix) * 4
      imgData.data[idx] = r
      imgData.data[idx + 1] = g
      imgData.data[idx + 2] = b
      imgData.data[idx + 3] = 255
    }
  }

  // Scale and draw
  const tempCanvas = document.createElement('canvas')
  tempCanvas.width = n
  tempCanvas.height = n
  const tempCtx = tempCanvas.getContext('2d')
  if (!tempCtx) return
  tempCtx.putImageData(imgData, 0, 0)

  ctx.imageSmoothingEnabled = true
  ctx.drawImage(tempCanvas, 0, 0, w, h)

  // Draw boundaries
  if (boundaries) {
    const cellW = w / n
    const cellH = h / n
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.6)'
    ctx.lineWidth = 0.5
    for (let iy = 0; iy < n; iy++) {
      for (let ix = 0; ix < n; ix++) {
        if (boundaries[iy][ix] > 0) {
          ctx.fillStyle = 'rgba(255, 255, 255, 0.7)'
          ctx.fillRect(ix * cellW, iy * cellH, cellW, cellH)
        }
      }
    }
  }

  // Draw contour lines
  if (vizConfig.contour_levels > 0) {
    const levels = vizConfig.contour_levels
    ctx.strokeStyle = 'rgba(0, 0, 0, 0.3)'
    ctx.lineWidth = 0.8
    for (let l = 1; l < levels; l++) {
      const threshold = fMin + (range * l) / levels
      const cellW = w / n
      const cellH = h / n
      for (let iy = 0; iy < n - 1; iy++) {
        for (let ix = 0; ix < n - 1; ix++) {
          const v00 = field[iy][ix]
          const v10 = field[iy][ix + 1]
          const v01 = field[iy + 1][ix]
          const v11 = field[iy + 1][ix + 1]
          const corners = [v00, v10, v11, v01]
          let below = 0
          let above = 0
          for (const c of corners) {
            if (c < threshold) below++
            else above++
          }
          if (below > 0 && above > 0) {
            ctx.strokeRect(ix * cellW, iy * cellH, cellW, cellH)
          }
        }
      }
    }
  }

  // Title
  ctx.fillStyle = '#E8E9EB'
  ctx.font = '12px sans-serif'
  ctx.textAlign = 'center'
  ctx.fillText(title, w / 2, 18)

  // Min/Max labels
  ctx.font = '10px monospace'
  ctx.textAlign = 'left'
  ctx.fillText(`min: ${fMin.toFixed(3)}`, 8, h - 8)
  ctx.textAlign = 'right'
  ctx.fillText(`max: ${fMax.toFixed(3)}`, w - 8, h - 8)
}

function drawVisualization() {
  if (!vizCanvas.value || orderParamField.value.length === 0) return
  const titleMap: Record<FieldType, string> = {
    order_parameter: '序参量场',
    temperature: '温度场',
    stress: '应力场',
    free_energy: '自由能场',
    grain_id: '晶粒ID场',
    strain: '应变场',
  }
  drawFieldOnCanvas(
    vizCanvas.value,
    orderParamField.value,
    vizConfig.colormap,
    vizConfig.show_grain_boundaries ? grainBoundaryField.value : null,
    titleMap[vizConfig.field] || '场可视化'
  )
}

function drawStressField() {
  if (!stressCanvas.value || !mechResult.value) return
  let field: number[][] = []
  let title = ''
  switch (activeStressComponent.value) {
    case 'xx':
      field = mechResult.value.stress_field.xx
      title = '应力场 sigma_xx'
      break
    case 'yy':
      field = mechResult.value.stress_field.yy
      title = '应力场 sigma_yy'
      break
    case 'xy':
      field = mechResult.value.stress_field.xy
      title = '应力场 tau_xy'
      break
    case 'von_mises':
      field = mechResult.value.von_mises_field
      title = 'von Mises 等效应力'
      break
  }
  drawFieldOnCanvas(stressCanvas.value, field, 'coolwarm', null, title)
}

function drawEnergyDensity() {
  if (!energyCanvas.value || !mechResult.value) return
  drawFieldOnCanvas(energyCanvas.value, mechResult.value.elastic_energy_density, 'viridis', null, '弹性应变能密度')
}

// ============ Actions ============

async function handleAnalyzeGrains() {
  analyzingGrains.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 1200))
    // Generate mock data
    const numSeeds = 25 + Math.floor(Math.random() * 20)
    const { field, seeds } = generateVoronoiField(gridSize, numSeeds)
    orderParamField.value = field
    grainBoundaryField.value = computeGrainBoundaries(field, 0.15)

    const grains = extractGrainsFromField(field, seeds)
    const sizeDist = computeSizeDistribution(grains)
    const orientDist = computeOrientationDistribution(grains)
    const sizeVsTime = generateGrainSizeVsTime(30)

    grainResult.value = {
      success: true,
      grains,
      size_distribution: sizeDist,
      orientation_distribution: orientDist,
      total_grain_area: grains.reduce((s, g) => s + g.area, 0),
      num_grains: grains.length,
      avg_grain_size: grains.reduce((s, g) => s + g.equivalent_diameter, 0) / grains.length,
      grain_boundary_length: grainBoundaryField.value.flat().reduce((s, v) => s + v, 0),
      grain_size_vs_time: sizeVsTime,
    }

    await nextTick()
    drawVisualization()
  } catch (e) {
    console.error('Grain analysis failed:', e)
  } finally {
    analyzingGrains.value = false
  }
}

async function handleRunMechanical() {
  runningMechanical.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 1500))
    if (orderParamField.value.length === 0) {
      const numSeeds = 25 + Math.floor(Math.random() * 15)
      const { field } = generateVoronoiField(gridSize, numSeeds)
      orderParamField.value = field
    }
    mechResult.value = computeMechanicalResult(orderParamField.value)
    await nextTick()
    drawStressField()
    drawEnergyDensity()
  } catch (e) {
    console.error('Mechanical coupling failed:', e)
  } finally {
    runningMechanical.value = false
  }
}

function handleExportCSV() {
  const field = orderParamField.value
  if (field.length === 0) return
  let csv = ''
  for (const row of field) {
    csv += row.join(',') + '\n'
  }
  const blob = new Blob([csv], { type: 'text/csv' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'phase_field_export.csv'
  a.click()
  URL.revokeObjectURL(url)
}

// ============ Watchers ============

watch([() => vizConfig.field, () => vizConfig.colormap, () => vizConfig.contour_levels, () => vizConfig.show_grain_boundaries], () => {
  if (activeTab.value === 'visualization' && orderParamField.value.length > 0) {
    nextTick(() => drawVisualization())
  }
})

watch(activeStressComponent, () => {
  if (mechResult.value) {
    nextTick(() => drawStressField())
  }
})

watch(activeTab, async () => {
  await nextTick()
  if (activeTab.value === 'visualization' && orderParamField.value.length > 0) {
    drawVisualization()
  }
  if (activeTab.value === 'mechanical' && mechResult.value) {
    drawStressField()
    drawEnergyDensity()
  }
})

// ============ Lifecycle ============

onMounted(async () => {
  // Auto-generate initial data
  const numSeeds = 30
  const { field, seeds } = generateVoronoiField(gridSize, numSeeds)
  orderParamField.value = field
  grainBoundaryField.value = computeGrainBoundaries(field, 0.15)

  const grains = extractGrainsFromField(field, seeds)
  const sizeDist = computeSizeDistribution(grains)
  const orientDist = computeOrientationDistribution(grains)
  const sizeVsTime = generateGrainSizeVsTime(30)

  grainResult.value = {
    success: true,
    grains,
    size_distribution: sizeDist,
    orientation_distribution: orientDist,
    total_grain_area: grains.reduce((s, g) => s + g.area, 0),
    num_grains: grains.length,
    avg_grain_size: grains.reduce((s, g) => s + g.equivalent_diameter, 0) / grains.length,
    grain_boundary_length: grainBoundaryField.value.flat().reduce((s, v) => s + v, 0),
    grain_size_vs_time: sizeVsTime,
  }

  mechResult.value = computeMechanicalResult(field)

  await nextTick()
  drawVisualization()
})
</script>

<style scoped>
.panel-section {
  margin-bottom: 4px;
  padding: 12px;
  background: var(--bg-elevated);
  border-radius: var(--radius-md);
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
  background: var(--bg-base);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 12px;
  transition: border-color var(--duration-fast) var(--ease-out);
}

.input:focus {
  outline: none;
  border-color: var(--primary);
  box-shadow: 0 0 0 2px var(--primary-glow);
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 8px 16px;
  border-radius: var(--radius-md);
  font-weight: 500;
  transition: all var(--duration-fast) var(--ease-out);
  cursor: pointer;
  border: none;
}

.btn-primary {
  background: var(--primary);
  color: #fff;
}

.btn-primary:hover {
  opacity: 0.9;
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-ghost {
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border-default);
}

.btn-ghost:hover {
  background: var(--bg-elevated);
}

.btn-ghost:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Scrollbar styling */
.overflow-y-auto::-webkit-scrollbar {
  width: 4px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: transparent;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background: var(--border-default);
  border-radius: var(--radius-full);
}
</style>
