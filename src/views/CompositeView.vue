<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">复合材料层合板分析</h2>
        <p class="text-sm" style="color: var(--text-muted)">经典层合板理论 / Tsai-Hill / Tsai-Wu 失效准则，铺层顺序优化</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- Template Buttons -->
        <button
          v-for="tpl in quickTemplates"
          :key="tpl.id"
          @click="applyTemplate(tpl)"
          class="px-3 py-1.5 text-xs border rounded transition"
          style="border-color: var(--border-default); background: var(--bg-elevated); color: var(--text-secondary)"
        >
          {{ tpl.name }}
        </button>
        <button @click="resetAll" class="px-3 py-1.5 text-xs border rounded transition" style="border-color: var(--border-default); color: var(--text-secondary)">
          重置
        </button>
        <button v-if="results" @click="exportResults" class="px-3 py-1.5 text-xs border rounded transition" style="border-color: var(--accent-green); color: var(--accent-green)">
          导出结果
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration (w-80) -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Ply Definition -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            铺层定义
          </h4>
          <!-- Material Selection -->
          <div class="mb-3">
            <label class="label">材料选择</label>
            <select v-model="selectedMaterial" @change="onMaterialChange" class="input w-full text-xs">
              <option value="t300_epoxy">碳纤维 T300/环氧</option>
              <option value="glass_e">玻璃纤维 E</option>
              <option value="kevlar49">凯夫拉 49</option>
              <option value="boron">硼纤维</option>
            </select>
          </div>
          <!-- Fiber Angle -->
          <div class="mb-3">
            <label class="label">纤维角度 (deg)</label>
            <div class="flex gap-1 flex-wrap">
              <button
                v-for="angle in [0, 45, -45, 90]"
                :key="angle"
                @click="newPlyAngle = angle"
                :class="['px-2 py-1 text-xs rounded border transition', newPlyAngle === angle ? 'text-white' : '']"
                :style="newPlyAngle === angle
                  ? 'background: var(--primary); border-color: var(--primary)'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
              >
                {{ angle }}&deg;
              </button>
            </div>
          </div>
          <!-- Thickness -->
          <div class="mb-3">
            <label class="label">铺层厚度 (mm)</label>
            <input v-model.number="newPlyThickness" type="number" step="0.01" min="0.01" class="input w-full text-xs" />
          </div>
          <!-- Add Ply -->
          <button @click="addPly" class="btn btn-primary text-xs w-full mb-2">添加铺层</button>
          <!-- Ply List -->
          <div class="space-y-1 max-h-48 overflow-y-auto">
            <div
              v-for="(ply, idx) in config.plies"
              :key="idx"
              class="flex items-center justify-between px-2 py-1.5 rounded text-xs"
              style="background: var(--bg-elevated)"
            >
              <span style="color: var(--text-secondary)">
                #{{ idx + 1 }} {{ ply.material_name }} {{ ply.fiber_angle }}&deg;
              </span>
              <div class="flex items-center gap-1">
                <button @click="movePly(idx, -1)" :disabled="idx === 0" class="px-1 rounded hover:opacity-80" style="color: var(--text-muted)">&uarr;</button>
                <button @click="movePly(idx, 1)" :disabled="idx === config.plies.length - 1" class="px-1 rounded hover:opacity-80" style="color: var(--text-muted)">&darr;</button>
                <button @click="removePly(idx)" class="px-1 rounded hover:opacity-80" style="color: var(--accent-red)">&times;</button>
              </div>
            </div>
          </div>
          <!-- Symmetry -->
          <div class="mt-3">
            <label class="label">对称选项</label>
            <div class="flex gap-1">
              <button
                v-for="sym in symmetryOptions"
                :key="sym.value"
                @click="config.symmetry = sym.value"
                :class="['flex-1 px-2 py-1.5 text-xs rounded border transition', config.symmetry === sym.value ? 'text-white' : '']"
                :style="config.symmetry === sym.value
                  ? 'background: var(--primary); border-color: var(--primary)'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
              >
                {{ sym.label }}
              </button>
            </div>
          </div>
        </div>

        <!-- Step 2: Loading -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            载荷定义
          </h4>
          <div class="space-y-2">
            <div class="grid grid-cols-3 gap-2">
              <div>
                <label class="label">Nx (N/m)</label>
                <input v-model.number="config.loading.Nx" type="number" step="100" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">Ny (N/m)</label>
                <input v-model.number="config.loading.Ny" type="number" step="100" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">Nxy (N/m)</label>
                <input v-model.number="config.loading.Nxy" type="number" step="100" class="input w-full text-xs" />
              </div>
            </div>
            <div class="grid grid-cols-3 gap-2">
              <div>
                <label class="label">Mx (N)</label>
                <input v-model.number="config.loading.Mx" type="number" step="10" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">My (N)</label>
                <input v-model.number="config.loading.My" type="number" step="10" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">Mxy (N)</label>
                <input v-model.number="config.loading.Mxy" type="number" step="10" class="input w-full text-xs" />
              </div>
            </div>
            <div>
              <label class="label">温差 (degC)</label>
              <input v-model.number="config.temperature_delta" type="number" step="5" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Step 3: Analysis Settings -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            分析设置
          </h4>
          <div class="space-y-2">
            <label class="label">分析类型</label>
            <div class="flex flex-col gap-1">
              <button
                v-for="at in analysisTypes"
                :key="at.value"
                @click="config.analysis_type = at.value"
                :class="['px-3 py-2 rounded text-xs text-left transition border', config.analysis_type === at.value ? 'text-white' : '']"
                :style="config.analysis_type === at.value
                  ? 'background: var(--primary); border-color: var(--primary)'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
              >
                <div class="font-medium">{{ at.label }}</div>
                <div class="text-[10px] mt-0.5 opacity-80">{{ at.desc }}</div>
              </button>
            </div>
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="space-y-2">
          <button
            @click="runAnalysis"
            :disabled="analyzing || config.plies.length === 0"
            class="btn btn-primary text-xs w-full"
          >
            <span v-if="analyzing" class="mr-1 animate-spin">&#10227;</span>
            {{ analyzing ? '分析中...' : '运行分析' }}
          </button>
          <button
            @click="runOptimization"
            :disabled="optimizing || config.plies.length === 0"
            class="btn btn-ghost text-xs w-full"
          >
            <span v-if="optimizing" class="mr-1 animate-spin">&#10227;</span>
            {{ optimizing ? '优化中...' : '铺层优化' }}
          </button>
        </div>
      </div>

      <!-- Right Panel: Results -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- No results placeholder -->
        <div v-if="!results" class="flex-1 flex items-center justify-center" style="color: var(--text-muted)">
          <div class="text-center">
            <div class="text-4xl mb-2">&#128202;</div>
            <div class="text-sm">配置铺层参数后运行层合板分析</div>
          </div>
        </div>

        <!-- Results Content -->
        <template v-else>
          <div class="flex-1 overflow-y-auto p-4 space-y-4">
            <!-- ABD Matrix Display -->
            <div class="grid grid-cols-3 gap-3">
              <div v-for="mat in abdMatrices" :key="mat.label" class="result-card">
                <span class="result-label">{{ mat.label }} 矩阵</span>
                <div class="grid grid-cols-3 gap-1 mt-1">
                  <div v-for="(val, vi) in mat.values" :key="vi" class="text-xs font-mono text-right px-1 py-0.5 rounded" style="background: var(--bg-elevated); color: var(--text-primary)">
                    {{ formatSci(val) }}
                  </div>
                </div>
              </div>
            </div>

            <!-- Ply Stress Heatmap + Failure Envelope side by side -->
            <div class="grid grid-cols-2 gap-3">
              <!-- Ply Stress Heatmap -->
              <div class="result-card">
                <span class="result-label">铺层失效指数云图</span>
                <div class="mt-2">
                  <canvas ref="heatmapCanvas" class="w-full rounded" style="height: 200px; background: var(--bg-elevated)"></canvas>
                </div>
              </div>
              <!-- Failure Envelope SVG -->
              <div class="result-card">
                <span class="result-label">Tsai-Wu 失效包络</span>
                <div class="mt-2">
                  <svg ref="envelopeSvg" :viewBox="`0 0 ${envSvgW} ${envSvgH}`" class="w-full rounded" style="height: 200px; background: var(--bg-elevated)">
                    <!-- Grid -->
                    <line v-for="i in 5" :key="'eh'+i"
                      :x1="envPad" :y1="envPad + (i-1) * (envSvgH - 2*envPad) / 4"
                      :x2="envSvgW - envPad" :y2="envPad + (i-1) * (envSvgH - 2*envPad) / 4"
                      stroke="#2D2E38" stroke-width="0.5"
                    />
                    <line v-for="i in 5" :key="'ev'+i"
                      :x1="envPad + (i-1) * (envSvgW - 2*envPad) / 4" :y1="envPad"
                      :x2="envPad + (i-1) * (envSvgW - 2*envPad) / 4" :y2="envSvgH - envPad"
                      stroke="#2D2E38" stroke-width="0.5"
                    />
                    <!-- Axes -->
                    <line :x1="envSvgW/2" :y1="envPad" :x2="envSvgW/2" :y2="envSvgH - envPad" stroke="#6B7280" stroke-width="1" />
                    <line :x1="envPad" :y1="envSvgH/2" :x2="envSvgW - envPad" :y2="envSvgH/2" stroke="#6B7280" stroke-width="1" />
                    <!-- Envelope ellipse -->
                    <ellipse
                      :cx="envSvgW/2" :cy="envSvgH/2"
                      :rx="envelopeRx" :ry="envelopeRy"
                      fill="rgba(79,140,255,0.08)" stroke="#4F8CFF" stroke-width="1.5" stroke-dasharray="4,2"
                    />
                    <!-- Current load point -->
                    <circle
                      :cx="loadPointX" :cy="loadPointY"
                      r="4" :fill="isInsideEnvelope ? '#22c55e' : '#ef4444'"
                    />
                    <!-- Labels -->
                    <text :x="envSvgW/2" :y="envSvgH - 2" fill="#9CA3AF" font-size="9" text-anchor="middle">sigma_x (MPa)</text>
                    <text :x="8" :y="envSvgH/2" fill="#9CA3AF" font-size="9" text-anchor="middle" transform="rotate(-90, 8, 100)">sigma_y (MPa)</text>
                  </svg>
                </div>
              </div>
            </div>

            <!-- Result Statistics -->
            <div class="grid grid-cols-4 gap-3">
              <div class="result-card">
                <span class="result-label">首层失效载荷</span>
                <span class="result-value">{{ formatSci(results.first_ply_failure_load) }} N/m</span>
              </div>
              <div class="result-card">
                <span class="result-label">安全裕度</span>
                <span class="result-value" :style="{ color: results.margin_of_safety > 1 ? 'var(--accent-green)' : 'var(--accent-red)' }">
                  {{ results.margin_of_safety.toFixed(3) }}
                </span>
              </div>
              <div class="result-card">
                <span class="result-label">铺层总数</span>
                <span class="result-value">{{ results.ply_stresses.length }}</span>
              </div>
              <div class="result-card">
                <span class="result-label">最大失效指数</span>
                <span class="result-value" :style="{ color: maxFI > 1 ? 'var(--accent-red)' : maxFI > 0.6 ? 'var(--accent-yellow)' : 'var(--accent-green)' }">
                  {{ maxFI.toFixed(4) }}
                </span>
              </div>
            </div>

            <!-- Midplane Strains -->
            <div class="result-card">
              <span class="result-label">中面应变 / 曲率</span>
                            <div class="grid grid-cols-6 gap-2 mt-2">
                <div v-for="key in strainKeys" :key="key" class="text-center">
                  <div class="text-[10px]" style="color: var(--text-muted)">{{ strainLabels[key] }}</div>
                  <div class="text-xs font-mono" style="color: var(--text-primary)">
                    {{ formatSci(results.midplane_strains[key]) }}
                  </div>
                </div>
              </div>
            </div>

            <!-- Failure Index Table -->
            <div class="result-card">
              <span class="result-label">各层失效指数</span>
              <div class="mt-2 overflow-x-auto">
                <table class="w-full text-xs">
                  <thead>
                    <tr style="color: var(--text-muted)">
                      <th class="text-left py-1 px-2">铺层</th>
                      <th class="text-left py-1 px-2">角度</th>
                      <th class="text-right py-1 px-2">Tsai-Hill</th>
                      <th class="text-right py-1 px-2">Tsai-Wu</th>
                      <th class="text-right py-1 px-2">Max Strain</th>
                      <th class="text-right py-1 px-2">Max Stress</th>
                      <th class="text-center py-1 px-2">状态</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="fi in results.failure_indices" :key="fi.ply_index" style="border-top: 1px solid var(--border-subtle)">
                      <td class="py-1 px-2" style="color: var(--text-secondary)">#{{ fi.ply_index + 1 }}</td>
                      <td class="py-1 px-2" style="color: var(--text-secondary)">{{ config.plies[fi.ply_index]?.fiber_angle ?? 0 }}&deg;</td>
                      <td class="text-right py-1 px-2 font-mono" :style="{ color: fiColor(fi.tsai_hill) }">{{ fi.tsai_hill.toFixed(4) }}</td>
                      <td class="text-right py-1 px-2 font-mono" :style="{ color: fiColor(fi.tsai_wu) }">{{ fi.tsai_wu.toFixed(4) }}</td>
                      <td class="text-right py-1 px-2 font-mono" :style="{ color: fiColor(fi.max_strain) }">{{ fi.max_strain.toFixed(4) }}</td>
                      <td class="text-right py-1 px-2 font-mono" :style="{ color: fiColor(fi.max_stress) }">{{ fi.max_stress.toFixed(4) }}</td>
                      <td class="text-center py-1 px-2">
                        <span :class="['inline-block px-2 py-0.5 rounded-full text-[10px]', fi.failed ? 'text-white' : '']"
                          :style="{ background: fi.failed ? 'var(--accent-red)' : 'var(--accent-green)', color: fi.failed ? '#fff' : 'var(--text-primary)' }">
                          {{ fi.failed ? '失效' : '安全' }}
                        </span>
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type {
  PlyProperty,
  LaminateConfig,
  LaminateResult,
  ABDMatrix,
  FailureIndex,
} from '../api/composite'

// ============ Constants ============

const materialLibrary: Record<string, PlyProperty> = {
  t300_epoxy: {
    material_name: '碳纤维T300/环氧',
    thickness: 0.125,
    fiber_angle: 0,
    E1: 181e9, E2: 10.3e9, G12: 7.17e9, G23: 3.78e9,
    nu12: 0.28, nu21: 0.0159,
    Xt: 1500e6, Xc: 1500e6, Yt: 40e6, Yc: 246e6, S: 68e6,
    density: 1600,
  },
  glass_e: {
    material_name: '玻璃纤维E',
    thickness: 0.2,
    fiber_angle: 0,
    E1: 38.6e9, E2: 8.27e9, G12: 4.14e9, G23: 3.24e9,
    nu12: 0.26, nu21: 0.0557,
    Xt: 1062e6, Xc: 610e6, Yt: 31e6, Yc: 118e6, S: 72e6,
    density: 2000,
  },
  kevlar49: {
    material_name: '凯夫拉49',
    thickness: 0.13,
    fiber_angle: 0,
    E1: 76e9, E2: 5.5e9, G12: 2.3e9, G23: 2.1e9,
    nu12: 0.34, nu21: 0.0246,
    Xt: 1380e6, Xc: 235e6, Yt: 29e6, Yc: 138e6, S: 44e6,
    density: 1440,
  },
  boron: {
    material_name: '硼纤维',
    thickness: 0.13,
    fiber_angle: 0,
    E1: 208e9, E2: 25.4e9, G12: 7.24e9, G23: 6.96e9,
    nu12: 0.1677, nu21: 0.0205,
    Xt: 1290e6, Xc: 2410e6, Yt: 69e6, Yc: 172e6, S: 96e6,
    density: 1940,
  },
}

const analysisTypes = [
  { value: 'classical_lamination' as const, label: '经典层合板理论 (CLPT)', desc: '计算 ABD 矩阵与中面应变' },
  { value: 'tsai_hill' as const, label: 'Tsai-Hill 准则', desc: '基于 Hill 屈服准则的失效分析' },
  { value: 'tsai_wu' as const, label: 'Tsai-Wu 准则', desc: '张量多项式交互失效准则' },
  { value: 'first_ply_failure' as const, label: '首层失效分析', desc: '渐进失效的首层失效载荷' },
]

const symmetryOptions = [
  { value: 'none' as const, label: '非对称' },
  { value: 'symmetric' as const, label: '对称' },
  { value: 'anti-symmetric' as const, label: '反对称' },
]

const quickTemplates = [
  {
    id: '0_90s',
    name: '[0/90]s',
    angles: [0, 90],
  },
  {
    id: '0_45_90s',
    name: '[0/±45/90]s',
    angles: [0, 45, -45, 90],
  },
  {
    id: 'quasi_iso',
    name: '准各向同性',
    angles: [0, 45, -45, 90],
  },
  {
    id: '45s',
    name: '[±45]s',
    angles: [45, -45],
  },
]

const strainLabels = {
  exx: 'epsilon_xx',
  eyy: 'epsilon_yy',
  gxy: 'gamma_xy',
  kxx: 'kappa_xx',
  kyy: 'kappa_yy',
  kxy: 'kappa_xy',
}

type StrainKey = keyof typeof strainLabels

const strainKeys: StrainKey[] = ['exx', 'eyy', 'gxy', 'kxx', 'kyy', 'kxy']

// ============ State ============

const analyzing = ref(false)
const optimizing = ref(false)
const selectedMaterial = ref('t300_epoxy')
const newPlyAngle = ref(0)
const newPlyThickness = ref(0.125)
const results = ref<LaminateResult | null>(null)
const heatmapCanvas = ref<HTMLCanvasElement | null>(null)
const envelopeSvg = ref<SVGSVGElement | null>(null)

const envSvgW = 300
const envSvgH = 200
const envPad = 30

const config = reactive<LaminateConfig>({
  plies: [],
  symmetry: 'symmetric',
  analysis_type: 'classical_lamination',
  loading: { Nx: 1000, Ny: 0, Nxy: 500, Mx: 0, My: 0, Mxy: 0 },
  temperature_delta: -50,
})

// ============ Computed ============

const abdMatrices = computed(() => {
  if (!results.value) return []
  const abd = results.value.ABD_matrix
  return [
    { label: 'A (N/m)', values: abd.A },
    { label: 'B (N)', values: abd.B },
    { label: 'D (N*m)', values: abd.D },
  ]
})

const maxFI = computed(() => {
  if (!results.value) return 0
  let max = 0
  for (const fi of results.value.failure_indices) {
    max = Math.max(max, fi.tsai_hill, fi.tsai_wu, fi.max_strain, fi.max_stress)
  }
  return max
})

const envelopeRx = computed(() => (envSvgW - 2 * envPad) / 2 * 0.85)
const envelopeRy = computed(() => (envSvgH - 2 * envPad) / 2 * 0.85)

const loadPointX = computed(() => {
  if (!results.value) return envSvgW / 2
  const nx = config.loading.Nx
  const ny = config.loading.Ny
  const maxN = Math.max(Math.abs(nx), Math.abs(ny), 1)
  return envSvgW / 2 + (nx / maxN) * envelopeRx.value * 0.7
})

const loadPointY = computed(() => {
  if (!results.value) return envSvgH / 2
  const nx = config.loading.Nx
  const ny = config.loading.Ny
  const maxN = Math.max(Math.abs(nx), Math.abs(ny), 1)
  return envSvgH / 2 - (ny / maxN) * envelopeRy.value * 0.7
})

const isInsideEnvelope = computed(() => {
  if (!results.value) return true
  const nx = config.loading.Nx
  const ny = config.loading.Ny
  const maxN = Math.max(Math.abs(nx), Math.abs(ny), 1)
  const rx = envelopeRx.value * 0.85
  const ry = envelopeRy.value * 0.85
  const px = (nx / maxN) * rx
  const py = (ny / maxN) * ry
  return (px * px) / (rx * rx) + (py * py) / (ry * ry) <= 1
})

// ============ Methods ============

function onMaterialChange() {
  const mat = materialLibrary[selectedMaterial.value]
  if (mat) {
    newPlyThickness.value = mat.thickness
  }
}

function addPly() {
  const mat = materialLibrary[selectedMaterial.value]
  if (!mat) return
  const ply: PlyProperty = {
    ...mat,
    fiber_angle: newPlyAngle.value,
    thickness: newPlyThickness.value,
  }
  config.plies.push(ply)
}

function removePly(idx: number) {
  config.plies.splice(idx, 1)
}

function movePly(idx: number, dir: number) {
  const newIdx = idx + dir
  if (newIdx < 0 || newIdx >= config.plies.length) return
  const temp = config.plies[idx]
  config.plies[idx] = config.plies[newIdx]
  config.plies[newIdx] = temp
}

function applyTemplate(tpl: typeof quickTemplates[0]) {
  config.plies = []
  const mat = materialLibrary[selectedMaterial.value]
  if (!mat) return

  const halfPlies = tpl.angles.map(a => ({
    ...mat,
    fiber_angle: a,
    thickness: newPlyThickness.value,
  }))

  if (config.symmetry === 'symmetric') {
    config.plies = [...halfPlies, ...halfPlies.slice().reverse()]
  } else {
    config.plies = [...halfPlies, ...halfPlies.slice().reverse()]
  }
}

function resetAll() {
  results.value = null
  config.plies = []
  config.symmetry = 'symmetric'
  config.analysis_type = 'classical_lamination'
  config.loading = { Nx: 1000, Ny: 0, Nxy: 500, Mx: 0, My: 0, Mxy: 0 }
  config.temperature_delta = -50
  selectedMaterial.value = 't300_epoxy'
  newPlyAngle.value = 0
  newPlyThickness.value = 0.125
  onMaterialChange()
}

function formatSci(val: number): string {
  if (Math.abs(val) < 1e-15) return '0.00e+0'
  if (Math.abs(val) >= 1e6 || Math.abs(val) < 0.01) {
    return val.toExponential(2)
  }
  return val.toFixed(4)
}

function fiColor(val: number): string {
  if (val > 1) return 'var(--accent-red)'
  if (val > 0.6) return 'var(--accent-yellow)'
  return 'var(--accent-green)'
}

/**
 * Build effective plies considering symmetry
 */
function getEffectivePlies(): PlyProperty[] {
  if (config.symmetry === 'symmetric') {
    return [...config.plies, ...config.plies.slice().reverse()]
  }
  if (config.symmetry === 'anti-symmetric') {
    const reversed = config.plies.slice().reverse().map(p => ({
      ...p,
      fiber_angle: -p.fiber_angle,
    }))
    return [...config.plies, ...reversed]
  }
  return [...config.plies]
}

/**
 * Transform reduced stiffness matrix Q to laminate coordinates
 */
function transformQ(Q11: number, Q12: number, Q22: number, Q66: number, theta: number): { Q11b: number; Q12b: number; Q22b: number; Q16b: number; Q26b: number; Q66b: number } {
  const c = Math.cos(theta * Math.PI / 180)
  const s = Math.sin(theta * Math.PI / 180)
  const c2 = c * c
  const s2 = s * s
  const c4 = c2 * c2
  const s4 = s2 * s2
  const cs = c * s
  const c2s2 = c2 * s2

  return {
    Q11b: Q11 * c4 + 2 * (Q12 + 2 * Q66) * c2s2 + Q22 * s4,
    Q12b: (Q11 + Q22 - 4 * Q66) * c2s2 + Q12 * (c4 + s4),
    Q22b: Q11 * s4 + 2 * (Q12 + 2 * Q66) * c2s2 + Q22 * c4,
    Q16b: (Q11 - Q12 - 2 * Q66) * c2 * cs - (Q22 - Q12 - 2 * Q66) * s2 * cs,
    Q26b: (Q11 - Q12 - 2 * Q66) * s2 * cs - (Q22 - Q12 - 2 * Q66) * c2 * cs,
    Q66b: (Q11 + Q22 - 2 * Q12 - 2 * Q66) * c2s2 + Q66 * (c4 + s4),
  }
}

/**
 * Compute ABD matrix using Classical Lamination Theory
 */
function computeABD(plies: PlyProperty[]): ABDMatrix {
  const n = plies.length
  const totalH = plies.reduce((s, p) => s + p.thickness, 0)

  // Compute ply boundaries (z coordinates from bottom)
  const zBounds: number[] = [0]
  for (let i = 0; i < n; i++) {
    zBounds.push(zBounds[i] + plies[i].thickness)
  }
  // Shift so midplane is at z=0
  const mid = totalH / 2
  const zBot: number[] = []
  const zTop: number[] = []
  for (let i = 0; i < n; i++) {
    zBot.push(zBounds[i] - mid)
    zTop.push(zBounds[i + 1] - mid)
  }

  let A11 = 0, A12 = 0, A16 = 0, A22 = 0, A26 = 0, A66 = 0
  let B11 = 0, B12 = 0, B16 = 0, B22 = 0, B26 = 0, B66 = 0
  let D11 = 0, D12 = 0, D16 = 0, D22 = 0, D26 = 0, D66 = 0

  for (let i = 0; i < n; i++) {
    const p = plies[i]
    const nu21 = p.E2 * p.nu12 / p.E1
    const denom = 1 - p.nu12 * nu21
    const Q11 = p.E1 / denom
    const Q22 = p.E2 / denom
    const Q12 = p.nu12 * p.E2 / denom
    const Q66 = p.G12

    const tb = transformQ(Q11, Q12, Q22, Q66, p.fiber_angle)
    const h = zTop[i] - zBot[i]

    A11 += tb.Q11b * h
    A12 += tb.Q12b * h
    A16 += tb.Q16b * h
    A22 += tb.Q22b * h
    A26 += tb.Q26b * h
    A66 += tb.Q66b * h

    B11 += tb.Q11b * (zTop[i] * zTop[i] - zBot[i] * zBot[i]) / 2
    B12 += tb.Q12b * (zTop[i] * zTop[i] - zBot[i] * zBot[i]) / 2
    B16 += tb.Q16b * (zTop[i] * zTop[i] - zBot[i] * zBot[i]) / 2
    B22 += tb.Q22b * (zTop[i] * zTop[i] - zBot[i] * zBot[i]) / 2
    B26 += tb.Q26b * (zTop[i] * zTop[i] - zBot[i] * zBot[i]) / 2
    B66 += tb.Q66b * (zTop[i] * zTop[i] - zBot[i] * zBot[i]) / 2

    D11 += tb.Q11b * (zTop[i] ** 3 - zBot[i] ** 3) / 3
    D12 += tb.Q12b * (zTop[i] ** 3 - zBot[i] ** 3) / 3
    D16 += tb.Q16b * (zTop[i] ** 3 - zBot[i] ** 3) / 3
    D22 += tb.Q22b * (zTop[i] ** 3 - zBot[i] ** 3) / 3
    D26 += tb.Q26b * (zTop[i] ** 3 - zBot[i] ** 3) / 3
    D66 += tb.Q66b * (zTop[i] ** 3 - zBot[i] ** 3) / 3
  }

  return {
    A: [A11, A12, A16, A22, A26, A66],
    B: [B11, B12, B16, B22, B26, B66],
    D: [D11, D12, D16, D22, D26, D66],
  }
}

/**
 * Solve 6x6 system [ABD]{result} = {load}
 * Simple Gaussian elimination
 */
function solve6x6(mat: number[][], rhs: number[]): number[] {
  const n = 6
  const aug = mat.map((row, i) => [...row, rhs[i]])

  for (let col = 0; col < n; col++) {
    let maxRow = col
    for (let row = col + 1; row < n; row++) {
      if (Math.abs(aug[row][col]) > Math.abs(aug[maxRow][col])) maxRow = row
    }
    ;[aug[col], aug[maxRow]] = [aug[maxRow], aug[col]]

    const pivot = aug[col][col]
    if (Math.abs(pivot) < 1e-30) continue

    for (let j = col; j <= n; j++) aug[col][j] /= pivot
    for (let row = 0; row < n; row++) {
      if (row === col) continue
      const factor = aug[row][col]
      for (let j = col; j <= n; j++) aug[row][j] -= factor * aug[col][j]
    }
  }

  return aug.map(row => row[n])
}

/**
 * Compute Tsai-Hill failure index
 */
function tsaiHillFI(sxx: number, syy: number, sxy: number, ply: PlyProperty): number {
  const Xt = ply.Xt
  const Xc = ply.Xc
  const Yt = ply.Yt
  const Yc = ply.Yc
  const S = ply.S

  const sx2 = sxx > 0 ? (sxx / Xt) ** 2 : (sxx / Xc) ** 2
  const sy2 = syy > 0 ? (syy / Yt) ** 2 : (syy / Yc) ** 2
  const sxy2 = (sxy / S) ** 2

  return sx2 - (sxx * syy) / (Xt * Xt) + sy2 + sxy2
}

/**
 * Compute Tsai-Wu failure index
 */
function tsaiWuFI(sxx: number, syy: number, sxy: number, ply: PlyProperty): number {
  const F1 = 1 / ply.Xt - 1 / ply.Xc
  const F2 = 1 / ply.Yt - 1 / ply.Yc
  const F11 = 1 / (ply.Xt * ply.Xc)
  const F22 = 1 / (ply.Yt * ply.Yc)
  const F66 = 1 / (ply.S * ply.S)
  const F12 = -0.5 * Math.sqrt(F11 * F22)

  return F1 * sxx + F2 * syy + F11 * sxx * sxx + F22 * syy * syy + F66 * sxy * sxy + 2 * F12 * sxx * syy
}

/**
 * Compute max strain failure index
 */
function maxStrainFI(sxx: number, syy: number, sxy: number, ply: PlyProperty): number {
  const nu21 = ply.E2 * ply.nu12 / ply.E1
  const exx = sxx / ply.E1 - nu21 * syy / ply.E1
  const eyy = -ply.nu12 * sxx / ply.E1 + syy / ply.E2
  const gxy = sxy / ply.G12

  const eXt = ply.Xt / ply.E1
  const eXc = ply.Xc / ply.E1
  const eYt = ply.Yt / ply.E2
  const eYc = ply.Yc / ply.E2
  const eS = ply.S / ply.G12

  const fi1 = exx > 0 ? Math.abs(exx / eXt) : Math.abs(exx / eXc)
  const fi2 = eyy > 0 ? Math.abs(eyy / eYt) : Math.abs(eyy / eYc)
  const fi3 = Math.abs(gxy / eS)

  return Math.max(fi1, fi2, fi3)
}

/**
 * Compute max stress failure index
 */
function maxStressFI(sxx: number, syy: number, sxy: number, ply: PlyProperty): number {
  const fi1 = sxx > 0 ? Math.abs(sxx / ply.Xt) : Math.abs(sxx / ply.Xc)
  const fi2 = syy > 0 ? Math.abs(syy / ply.Yt) : Math.abs(syy / ply.Yc)
  const fi3 = Math.abs(sxy / ply.S)
  return Math.max(fi1, fi2, fi3)
}

/**
 * Generate mock analysis results using CLPT formulas
 */
function generateMockResults(): LaminateResult {
  const effectivePlies = getEffectivePlies()
  const n = effectivePlies.length

  if (n === 0) {
    return {
      ABD_matrix: { A: [0, 0, 0, 0, 0, 0], B: [0, 0, 0, 0, 0, 0], D: [0, 0, 0, 0, 0, 0] },
      midplane_strains: { exx: 0, eyy: 0, gxy: 0, kxx: 0, kyy: 0, kxy: 0 },
      ply_stresses: [],
      failure_indices: [],
      first_ply_failure_load: 0,
      margin_of_safety: 0,
    }
  }

  const abd = computeABD(effectivePlies)

  // Build 6x6 ABD matrix
  const mat6x6: number[][] = [
    [abd.A[0], abd.A[1], abd.A[2], abd.B[0], abd.B[1], abd.B[2]],
    [abd.A[1], abd.A[3], abd.A[4], abd.B[1], abd.B[3], abd.B[4]],
    [abd.A[2], abd.A[4], abd.A[5], abd.B[2], abd.B[4], abd.B[5]],
    [abd.B[0], abd.B[1], abd.B[2], abd.D[0], abd.D[1], abd.D[2]],
    [abd.B[1], abd.B[3], abd.B[4], abd.D[1], abd.D[3], abd.D[4]],
    [abd.B[2], abd.B[4], abd.B[5], abd.D[2], abd.D[4], abd.D[5]],
  ]

  const loadVec = [
    config.loading.Nx,
    config.loading.Ny,
    config.loading.Nxy,
    config.loading.Mx,
    config.loading.My,
    config.loading.Mxy,
  ]

  const result = solve6x6(mat6x6, loadVec)

  const midplaneStrains = {
    exx: result[0],
    eyy: result[1],
    gxy: result[2],
    kxx: result[3],
    kyy: result[4],
    kxy: result[5],
  }

  // Compute ply stresses
  const totalH = effectivePlies.reduce((s, p) => s + p.thickness, 0)
  const mid = totalH / 2
  let zAccum = 0
  const zBotArr: number[] = []
  const zTopArr: number[] = []
  for (let i = 0; i < n; i++) {
    zBotArr.push(zAccum - mid)
    zAccum += effectivePlies[i].thickness
    zTopArr.push(zAccum - mid)
  }

  const plyStresses: LaminateResult['ply_stresses'] = []
  const failureIndices: FailureIndex[] = []
  let maxFI = 0
  let criticalLoad = Infinity

  for (let i = 0; i < n; i++) {
    const p = effectivePlies[i]
    const zBot = zBotArr[i]
    const zTop = zTopArr[i]
    const zMid = (zBot + zTop) / 2

    // Strain at top and bottom of ply
    const exxTop = midplaneStrains.exx + zTop * midplaneStrains.kxx
    const eyyTop = midplaneStrains.eyy + zTop * midplaneStrains.kyy
    const gxyTop = midplaneStrains.gxy + zTop * midplaneStrains.kxy

    const exxBot = midplaneStrains.exx + zBot * midplaneStrains.kxx
    const eyyBot = midplaneStrains.eyy + zBot * midplaneStrains.kyy
    const gxyBot = midplaneStrains.gxy + zBot * midplaneStrains.kxy

    const exxMid = midplaneStrains.exx + zMid * midplaneStrains.kxx
    const eyyMid = midplaneStrains.eyy + zMid * midplaneStrains.kyy
    const gxyMid = midplaneStrains.gxy + zMid * midplaneStrains.kxy

    // Transform Qbar back to compute stresses in material coords
    const nu21 = p.E2 * p.nu12 / p.E1
    const denom = 1 - p.nu12 * nu21
    const Q11 = p.E1 / denom
    const Q22 = p.E2 / denom
    const Q12 = p.nu12 * p.E2 / denom
    const Q66 = p.G12

    const tb = transformQ(Q11, Q12, Q22, Q66, p.fiber_angle)

    // Stress in laminate coords at top
    const sxxTop = tb.Q11b * exxTop + tb.Q12b * eyyTop + tb.Q16b * gxyTop
    const syyTop = tb.Q12b * exxTop + tb.Q22b * eyyTop + tb.Q26b * gxyTop
    const sxyTop = tb.Q16b * exxTop + tb.Q26b * eyyTop + tb.Q66b * gxyTop

    // Stress in laminate coords at bottom
    const sxxBot = tb.Q11b * exxBot + tb.Q12b * eyyBot + tb.Q16b * gxyBot
    const syyBot = tb.Q12b * exxBot + tb.Q22b * eyyBot + tb.Q26b * gxyBot
    const sxyBot = tb.Q16b * exxBot + tb.Q26b * eyyBot + tb.Q66b * gxyBot

    // Transform stress to material coordinates for failure check
    const theta = p.fiber_angle * Math.PI / 180
    const c = Math.cos(theta)
    const s = Math.sin(theta)

    // Top stress in material coords
    const s1Top = c * c * sxxTop + s * s * syyTop + 2 * c * s * sxyTop
    const s2Top = s * s * sxxTop + c * c * syyTop - 2 * c * s * sxyTop
    const s12Top = -c * s * sxxTop + c * s * syyTop + (c * c - s * s) * sxyTop

    // Bottom stress in material coords
    const s1Bot = c * c * sxxBot + s * s * syyBot + 2 * c * s * sxyBot
    const s2Bot = s * s * sxxBot + c * c * syyBot - 2 * c * s * sxyBot
    const s12Bot = -c * s * sxxBot + c * s * syyBot + (c * c - s * s) * sxyBot

    // Use the more critical (higher stress magnitude) for failure check
    const s1 = Math.abs(s1Top) > Math.abs(s1Bot) ? s1Top : s1Bot
    const s2 = Math.abs(s2Top) > Math.abs(s2Bot) ? s2Top : s2Bot
    const s12 = Math.abs(s12Top) > Math.abs(s12Bot) ? s12Top : s12Bot

    const thFI = tsaiHillFI(s1, s2, s12, p)
    const twFI = tsaiWuFI(s1, s2, s12, p)
    const msFI = maxStrainFI(s1, s2, s12, p)
    const mtFI = maxStressFI(s1, s2, s12, p)

    const plyMaxFI = Math.max(thFI, twFI, msFI, mtFI)
    const failed = plyMaxFI >= 1.0

    if (plyMaxFI > maxFI) maxFI = plyMaxFI
    if (plyMaxFI > 0 && criticalLoad > 1 / plyMaxFI) {
      criticalLoad = 1 / plyMaxFI
    }

    plyStresses.push({
      ply_index: i,
      top_stress: { xx: sxxTop, yy: syyTop, xy: sxyTop },
      bottom_stress: { xx: sxxBot, yy: syyBot, xy: sxyBot },
      midplane_strain: { exx: exxMid, eyy: eyyMid, gxy: gxyMid },
      failure_index: plyMaxFI,
    })

    failureIndices.push({
      ply_index: i,
      tsai_hill: thFI,
      tsai_wu: twFI,
      max_strain: msFI,
      max_stress: mtFI,
      failed,
    })
  }

  // First ply failure load: scale current load by 1/maxFI
  const loadMagnitude = Math.sqrt(
    config.loading.Nx ** 2 + config.loading.Ny ** 2 + config.loading.Nxy ** 2
  )
  const fpfLoad = maxFI > 0 ? loadMagnitude / maxFI : Infinity
  const mos = maxFI > 0 ? 1 / maxFI - 1 : Infinity

  return {
    ABD_matrix: abd,
    midplane_strains: midplaneStrains,
    ply_stresses: plyStresses,
    failure_indices: failureIndices,
    first_ply_failure_load: isFinite(fpfLoad) ? fpfLoad : 0,
    margin_of_safety: isFinite(mos) ? mos : 999,
  }
}

function drawHeatmap() {
  if (!heatmapCanvas.value || !results.value) return

  const canvas = heatmapCanvas.value
  const rect = canvas.parentElement?.getBoundingClientRect()
  if (!rect) return

  canvas.width = rect.width * window.devicePixelRatio
  canvas.height = 200 * window.devicePixelRatio
  canvas.style.width = rect.width + 'px'
  canvas.style.height = '200px'

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const scale = window.devicePixelRatio
  ctx.scale(scale, scale)

  const w = rect.width
  const h = 200
  const padding = 40

  ctx.clearRect(0, 0, w, h)

  const fis = results.value.failure_indices
  if (fis.length === 0) return

  const maxVal = Math.max(...fis.map(f => Math.max(f.tsai_hill, f.tsai_wu)), 1)
  const barH = Math.min(20, (h - 2 * padding) / fis.length)
  const barW = w - 2 * padding

  for (let i = 0; i < fis.length; i++) {
    const fi = fis[i]
    const val = Math.max(fi.tsai_hill, fi.tsai_wu)
    const y = padding + i * barH

    // Background bar
    ctx.fillStyle = '#1A1B23'
    ctx.fillRect(padding, y, barW, barH - 2)

    // Colored fill based on failure index
    const ratio = Math.min(val / maxVal, 1)
    let r: number, g: number, b: number
    if (ratio < 0.5) {
      const t = ratio * 2
      r = Math.round(34 + t * (234 - 34))
      g = Math.round(197 + t * (179 - 197))
      b = Math.round(94 + t * (8 - 94))
    } else {
      const t = (ratio - 0.5) * 2
      r = Math.round(234 + t * (239 - 234))
      g = Math.round(179 + t * (68 - 179))
      b = Math.round(8 + t * (68 - 8))
    }

    ctx.fillStyle = `rgb(${r},${g},${b})`
    ctx.fillRect(padding, y, barW * ratio, barH - 2)

    // Label
    ctx.fillStyle = '#9CA3AF'
    ctx.font = '9px monospace'
    ctx.textAlign = 'right'
    ctx.textBaseline = 'middle'
    ctx.fillText(`#${i + 1}`, padding - 4, y + barH / 2)

    // Value
    ctx.fillStyle = '#E8E9EB'
    ctx.textAlign = 'left'
    ctx.fillText(val.toFixed(3), padding + barW * ratio + 4, y + barH / 2)
  }

  // Title
  ctx.fillStyle = '#E8E9EB'
  ctx.font = '11px sans-serif'
  ctx.textAlign = 'center'
  ctx.fillText('Failure Index (Tsai-Hill / Tsai-Wu)', w / 2, 16)
}

async function runAnalysis() {
  if (config.plies.length === 0) return
  analyzing.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 1200))
    results.value = generateMockResults()
    await nextTick()
    drawHeatmap()
  } catch (e) {
    console.error('Composite analysis failed:', e)
  } finally {
    analyzing.value = false
  }
}

async function runOptimization() {
  if (config.plies.length === 0) return
  optimizing.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 2000))
    // Simulate optimization: shuffle plies and re-analyze
    const original = [...config.plies]
    // Simple greedy: sort by angle magnitude for balanced layup
    config.plies.sort((a, b) => Math.abs(a.fiber_angle) - Math.abs(b.fiber_angle))
    results.value = generateMockResults()
    await nextTick()
    drawHeatmap()
  } catch (e) {
    console.error('Optimization failed:', e)
  } finally {
    optimizing.value = false
  }
}

function exportResults() {
  if (!results.value) return
  const data = JSON.stringify(results.value, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'composite_laminate_results.json'
  a.click()
  URL.revokeObjectURL(url)
}

// ============ Lifecycle ============

onMounted(() => {
  onMaterialChange()
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

.result-card {
  display: flex;
  flex-direction: column;
  padding: 12px;
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
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
