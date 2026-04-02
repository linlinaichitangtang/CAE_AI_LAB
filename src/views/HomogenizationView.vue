<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">均匀化方法 (FE²)</h2>
        <p class="text-sm" style="color: var(--text-muted)">微观 RVE → 宏观等效模量，闭环均匀化迭代</p>
      </div>
      <div class="flex items-center gap-2">
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
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration (w-80) -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Homogenization Method -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            均匀化方法
          </h4>
          <div class="grid grid-cols-2 gap-1">
            <button
              v-for="m in methods"
              :key="m.value"
              @click="config.method = m.value"
              :class="['px-2 py-2 rounded text-xs text-left transition border', config.method === m.value ? 'text-white' : '']"
              :style="config.method === m.value
                ? 'background: var(--primary); border-color: var(--primary)'
                : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
            >
              <div class="font-medium">{{ m.label }}</div>
              <div class="text-[10px] mt-0.5 opacity-80">{{ m.desc }}</div>
            </button>
          </div>
        </div>

        <!-- Step 2: RVE Input -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            RVE 输入
          </h4>
          <div class="space-y-2">
            <div class="flex items-center justify-between mb-2">
              <label class="label mb-0">导入已有 RVE</label>
              <button
                @click="importRve = !importRve"
                class="relative w-10 h-5 rounded-full transition-colors"
                :style="importRve ? 'background: var(--primary)' : 'background: var(--border-default)'"
              >
                <span
                  class="absolute top-0.5 w-4 h-4 rounded-full bg-white transition-transform"
                  :style="{ left: importRve ? '22px' : '2px' }"
                ></span>
              </button>
            </div>
            <div v-if="!importRve" class="space-y-2">
              <div>
                <label class="label">基体材料 E (GPa)</label>
                <input v-model.number="rveParams.matrixE_GPa" type="number" step="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">基体材料 nu</label>
                <input v-model.number="rveParams.matrixNu" type="number" step="0.01" min="0" max="0.5" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">包含体材料 E (GPa)</label>
                <input v-model.number="rveParams.inclusionE_GPa" type="number" step="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">包含体材料 nu</label>
                <input v-model.number="rveParams.inclusionNu" type="number" step="0.01" min="0" max="0.5" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">包含体体积分数 (%)</label>
                <input v-model.number="rveParams.inclusionVF" type="number" step="1" min="1" max="80" class="input w-full text-xs" />
              </div>
            </div>
            <div v-else class="text-xs p-3 rounded" style="background: var(--bg-base); color: var(--text-muted)">
              已导入 RVE 网格数据 ({{ rveImportInfo }})
            </div>
          </div>
        </div>

        <!-- Step 3: Macro Strain -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            宏观应变
          </h4>
          <div class="space-y-2">
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">exx</label>
                <input v-model.number="macroStrain.exx" type="number" step="0.001" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">eyy</label>
                <input v-model.number="macroStrain.eyy" type="number" step="0.001" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">ezz</label>
                <input v-model.number="macroStrain.ezz" type="number" step="0.001" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">gxy</label>
                <input v-model.number="macroStrain.gxy" type="number" step="0.001" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">gyz</label>
                <input v-model.number="macroStrain.gyz" type="number" step="0.001" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">gxz</label>
                <input v-model.number="macroStrain.gxz" type="number" step="0.001" class="input w-full text-xs" />
              </div>
            </div>
          </div>
        </div>

        <!-- Step 4: Solver Control -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            求解控制
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">收敛容差</label>
              <input v-model.number="config.convergence_tolerance" type="number" step="1e-6" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">最大迭代次数</label>
              <input v-model.number="config.max_iterations" type="number" step="1" min="1" max="1000" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">温度 (degC)</label>
              <input v-model.number="config.temperature" type="number" step="1" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="space-y-2">
          <button
            @click="runHomogenization"
            :disabled="running"
            class="btn btn-primary text-xs w-full"
          >
            <span v-if="running" class="mr-1 animate-spin">&#10227;</span>
            {{ running ? '计算中...' : '运行均匀化' }}
          </button>
        </div>
      </div>

      <!-- Right Panel: Results -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- No results placeholder -->
        <div v-if="!results" class="flex-1 flex items-center justify-center" style="color: var(--text-muted)">
          <div class="text-center">
            <div class="text-4xl mb-2">&#9670;</div>
            <div class="text-sm">配置参数后运行均匀化分析</div>
          </div>
        </div>

        <!-- Results Content -->
        <template v-else>
          <div class="flex-1 overflow-y-auto p-4 space-y-4">
            <!-- Effective Properties -->
            <div class="result-card">
              <span class="result-label">等效力学性能</span>
              <div class="grid grid-cols-3 gap-3 mt-3">
                <div v-for="prop in effectiveProps" :key="prop.key" class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                  <div class="text-[10px]" style="color: var(--text-muted)">{{ prop.label }}</div>
                  <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ formatSci(prop.value) }}</div>
                  <div class="text-[10px]" style="color: var(--text-muted)">{{ prop.unit }}</div>
                </div>
              </div>
            </div>

            <!-- Stiffness Matrix C -->
            <div class="result-card">
              <span class="result-label">刚度矩阵 C (6x6)</span>
              <div class="mt-2 overflow-x-auto">
                <table class="matrix-table">
                  <thead>
                    <tr>
                      <th class="matrix-header"></th>
                      <th v-for="j in 6" :key="j" class="matrix-header">{{ strainLabels[j - 1] }}</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="i in 6" :key="i">
                      <td class="matrix-header">{{ strainLabels[i - 1] }}</td>
                      <td
                        v-for="j in 6"
                        :key="j"
                        class="matrix-cell"
                        :class="{ 'matrix-diagonal': i === j }"
                      >
                        {{ formatMatrixCell(results.stiffness_matrix_C[i - 1][j - 1]) }}
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>

            <!-- Compliance Matrix S -->
            <div class="result-card">
              <span class="result-label">柔度矩阵 S (6x6)</span>
              <div class="mt-2 overflow-x-auto">
                <table class="matrix-table">
                  <thead>
                    <tr>
                      <th class="matrix-header"></th>
                      <th v-for="j in 6" :key="j" class="matrix-header">{{ strainLabels[j - 1] }}</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="i in 6" :key="i">
                      <td class="matrix-header">{{ strainLabels[i - 1] }}</td>
                      <td
                        v-for="j in 6"
                        :key="j"
                        class="matrix-cell"
                        :class="{ 'matrix-diagonal': i === j }"
                      >
                        {{ formatMatrixCell(results.compliance_matrix_S[i - 1][j - 1]) }}
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>

            <!-- Convergence Curve & Voigt/Reuss/Hill Comparison -->
            <div class="grid grid-cols-2 gap-4">
              <!-- Convergence Curve -->
              <div class="result-card">
                <span class="result-label">收敛曲线</span>
                <div class="mt-2">
                  <svg viewBox="0 0 300 180" class="w-full" style="background: var(--bg-base); border-radius: var(--radius-md)">
                    <!-- Grid lines -->
                    <line v-for="i in 5" :key="'h' + i" x1="40" y1="20 + i * 28" x2="290" y2="20 + i * 28" stroke="rgba(255,255,255,0.06)" stroke-width="0.5" />
                    <line v-for="i in 5" :key="'v' + i" x1="40 + i * 50" y1="20" x2="40 + i * 50" y2="160" stroke="rgba(255,255,255,0.06)" stroke-width="0.5" />

                    <!-- Y axis labels -->
                    <text x="35" y="24" fill="var(--text-muted)" font-size="8" text-anchor="end">1e0</text>
                    <text x="35" y="80" fill="var(--text-muted)" font-size="8" text-anchor="end">1e-4</text>
                    <text x="35" y="136" fill="var(--text-muted)" font-size="8" text-anchor="end">1e-8</text>
                    <text x="35" y="164" fill="var(--text-muted)" font-size="8" text-anchor="end">1e-10</text>

                    <!-- X axis labels -->
                    <text x="40" y="175" fill="var(--text-muted)" font-size="8" text-anchor="middle">0</text>
                    <text x="165" y="175" fill="var(--text-muted)" font-size="8" text-anchor="middle">{{ Math.floor(results.convergence_history.length / 2) }}</text>
                    <text x="290" y="175" fill="var(--text-muted)" font-size="8" text-anchor="middle">{{ results.convergence_history.length }}</text>

                    <!-- Convergence line -->
                    <polyline
                      :points="convergencePoints"
                      fill="none"
                      stroke="var(--primary)"
                      stroke-width="1.5"
                    />

                    <!-- Tolerance line -->
                    <line
                      x1="40" y1="140" x2="290" y2="140"
                      stroke="var(--accent-red)"
                      stroke-width="0.8"
                      stroke-dasharray="4,3"
                    />
                    <text x="292" y="143" fill="var(--accent-red)" font-size="7">tol</text>

                    <!-- Axis labels -->
                    <text x="165" y="12" fill="var(--text-secondary)" font-size="9" text-anchor="middle">Error vs Iteration</text>
                  </svg>
                </div>
              </div>

              <!-- Voigt/Reuss/Hill Comparison -->
              <div class="result-card">
                <span class="result-label">Voigt/Reuss/Hill 上下界</span>
                <div class="mt-2 space-y-3">
                  <div v-for="comp in boundComparisons" :key="comp.key">
                    <div class="text-[10px] mb-1" style="color: var(--text-secondary)">{{ comp.label }}</div>
                    <div class="space-y-0.5">
                      <div class="flex items-center gap-1">
                        <span class="text-[9px] w-8" style="color: var(--accent-green)">Voigt</span>
                        <div class="flex-1 h-2 rounded-full" style="background: var(--bg-base)">
                          <div class="h-2 rounded-full" style="background: var(--accent-green)" :style="{ width: comp.voigtPercent + '%' }"></div>
                        </div>
                        <span class="text-[9px] w-16 text-right font-mono" style="color: var(--text-primary)">{{ formatSci(comp.voigt) }}</span>
                      </div>
                      <div class="flex items-center gap-1">
                        <span class="text-[9px] w-8" style="color: var(--accent-red)">Reuss</span>
                        <div class="flex-1 h-2 rounded-full" style="background: var(--bg-base)">
                          <div class="h-2 rounded-full" style="background: var(--accent-red)" :style="{ width: comp.reussPercent + '%' }"></div>
                        </div>
                        <span class="text-[9px] w-16 text-right font-mono" style="color: var(--text-primary)">{{ formatSci(comp.reuss) }}</span>
                      </div>
                      <div class="flex items-center gap-1">
                        <span class="text-[9px] w-8" style="color: var(--primary)">Hill</span>
                        <div class="flex-1 h-2 rounded-full" style="background: var(--bg-base)">
                          <div class="h-2 rounded-full" style="background: var(--primary)" :style="{ width: comp.hillPercent + '%' }"></div>
                        </div>
                        <span class="text-[9px] w-16 text-right font-mono" style="color: var(--text-primary)">{{ formatSci(comp.hill) }}</span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Volume Averaged Stress/Strain -->
            <div class="grid grid-cols-2 gap-4">
              <div class="result-card">
                <span class="result-label">体积平均应力</span>
                <div class="space-y-1 mt-2">
                  <div v-for="comp in stressComponents" :key="comp.key" class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">{{ comp.label }}</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ comp.value.toExponential(3) }}</span>
                  </div>
                </div>
              </div>
              <div class="result-card">
                <span class="result-label">体积平均应变</span>
                <div class="space-y-1 mt-2">
                  <div v-for="comp in strainComponents" :key="comp.key" class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">{{ comp.label }}</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ comp.value.toFixed(6) }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import type {
  HomogenizationMethod,
  HomogenizationConfig,
  HomogenizationResult,
  HomogenizationTemplate,
  EffectiveProperties,
  MacroStrain,
} from '../api/homogenization'

// ============ Constants ============

const strainLabels = ['11', '22', '33', '12', '23', '13']

const methods = [
  { value: 'voigt' as HomogenizationMethod, label: 'Voigt 上界', desc: '等应变假设' },
  { value: 'reuss' as HomogenizationMethod, label: 'Reuss 下界', desc: '等应力假设' },
  { value: 'hill' as HomogenizationMethod, label: 'Hill 平均', desc: 'Voigt-Reuss平均' },
  { value: 'mori_tanaka' as HomogenizationMethod, label: 'Mori-Tanaka', desc: '稀疏近似' },
  { value: 'self_consistent' as HomogenizationMethod, label: '自洽', desc: 'Self-Consistent' },
  { value: 'fe2' as HomogenizationMethod, label: 'FE²', desc: '全尺度有限元' },
]

const quickTemplates: HomogenizationTemplate[] = [
  {
    id: 'ud_fiber_composite',
    name: '单向纤维复合材料',
    name_en: 'UD Fiber Composite',
    category: 'composite',
    description: '单向纤维增强复合材料均匀化',
    method: 'mori_tanaka',
    typical_application: '航空航天结构件',
  },
  {
    id: 'particle_reinforced_metal',
    name: '颗粒增强金属',
    name_en: 'Particle Reinforced Metal',
    category: 'metal_matrix',
    description: '颗粒增强金属基复合材料均匀化',
    method: 'mori_tanaka',
    typical_application: '汽车轻量化部件',
  },
  {
    id: 'porous_material',
    name: '多孔材料',
    name_en: 'Porous Material',
    category: 'foam',
    description: '多孔材料等效性能计算',
    method: 'self_consistent',
    typical_application: '轻量化夹层结构',
  },
  {
    id: 'woven_composite',
    name: '编织复合材料',
    name_en: 'Woven Composite',
    category: 'composite',
    description: '编织复合材料FE²均匀化',
    method: 'fe2',
    typical_application: '航空发动机部件',
  },
]

// ============ State ============

const running = ref(false)
const importRve = ref(false)
const rveImportInfo = ref('碳纤维/环氧 RVE, 12450 节点, 8920 单元')
const results = ref<HomogenizationResult | null>(null)

const config = reactive<HomogenizationConfig>({
  project_id: '',
  method: 'mori_tanaka',
  rve_mesh_data: {},
  macro_strains: [{ exx: 0.01, eyy: 0.0, ezz: 0.0, gxy: 0.0, gyz: 0.0, gxz: 0.0 }],
  convergence_tolerance: 1e-6,
  max_iterations: 50,
  temperature: 25,
})

const macroStrain = reactive<MacroStrain>({
  exx: 0.01,
  eyy: 0.0,
  ezz: 0.0,
  gxy: 0.0,
  gyz: 0.0,
  gxz: 0.0,
})

const rveParams = reactive({
  matrixE_GPa: 3.5,
  matrixNu: 0.35,
  inclusionE_GPa: 230,
  inclusionNu: 0.2,
  inclusionVF: 60,
})

// ============ Computed ============

const effectiveProps = computed(() => {
  if (!results.value) return []
  const ep = results.value.effective_properties
  return [
    { key: 'Ex', label: 'Ex', value: ep.Ex, unit: 'Pa' },
    { key: 'Ey', label: 'Ey', value: ep.Ey, unit: 'Pa' },
    { key: 'Ez', label: 'Ez', value: ep.Ez, unit: 'Pa' },
    { key: 'Gxy', label: 'Gxy', value: ep.Gxy, unit: 'Pa' },
    { key: 'Gyz', label: 'Gyz', value: ep.Gyz, unit: 'Pa' },
    { key: 'Gxz', label: 'Gxz', value: ep.Gxz, unit: 'Pa' },
    { key: 'nu_xy', label: 'nu_xy', value: ep.nu_xy, unit: '' },
    { key: 'nu_yz', label: 'nu_yz', value: ep.nu_yz, unit: '' },
    { key: 'nu_xz', label: 'nu_xz', value: ep.nu_xz, unit: '' },
  ]
})

const convergencePoints = computed(() => {
  if (!results.value || results.value.convergence_history.length === 0) return ''
  const history = results.value.convergence_history
  const maxIter = history.length
  const maxError = Math.log10(Math.max(history[0].error, 1e-15))
  const minError = Math.log10(Math.max(config.convergence_tolerance * 0.1, 1e-15))

  const points = history.map((record, idx) => {
    const x = 40 + (idx / Math.max(maxIter - 1, 1)) * 250
    const logErr = Math.log10(Math.max(record.error, 1e-15))
    const normalized = (logErr - minError) / (maxError - minError + 1e-10)
    const y = 160 - Math.max(0, Math.min(1, normalized)) * 140
    return `${x},${y}`
  })

  return points.join(' ')
})

const boundComparisons = computed(() => {
  if (!results.value) return []

  const ep = results.value.effective_properties
  const Em = rveParams.matrixE_GPa * 1e9
  const Ei = rveParams.inclusionE_GPa * 1e9
  const vf = rveParams.inclusionVF / 100
  const vm = 1 - vf

  // Voigt (upper bound): arithmetic mean
  const voigtE = Em * vm + Ei * vf
  // Reuss (lower bound): harmonic mean
  const reussE = 1 / (vm / Em + vf / Ei)
  // Hill: average of Voigt and Reuss
  const hillE = (voigtE + reussE) / 2

  const maxE = Ei

  return [
    {
      key: 'E1',
      label: '纵向模量 E1',
      voigt: voigtE,
      reuss: reussE,
      hill: hillE,
      voigtPercent: Math.min((voigtE / maxE) * 100, 100),
      reussPercent: Math.min((reussE / maxE) * 100, 100),
      hillPercent: Math.min((hillE / maxE) * 100, 100),
    },
    {
      key: 'G12',
      label: '剪切模量 G12',
      voigt: voigtE * 0.3,
      reuss: reussE * 0.3,
      hill: hillE * 0.3,
      voigtPercent: Math.min((voigtE * 0.3 / maxE) * 100, 100),
      reussPercent: Math.min((reussE * 0.3 / maxE) * 100, 100),
      hillPercent: Math.min((hillE * 0.3 / maxE) * 100, 100),
    },
    {
      key: 'nu12',
      label: '泊松比 nu12',
      voigt: rveParams.matrixNu * vm + rveParams.inclusionNu * vf,
      reuss: rveParams.matrixNu * vm + rveParams.inclusionNu * vf,
      hill: rveParams.matrixNu * vm + rveParams.inclusionNu * vf,
      voigtPercent: 50,
      reussPercent: 50,
      hillPercent: 50,
    },
  ]
})

const stressComponents = computed(() => {
  if (!results.value) return []
  const s = results.value.volume_averaged_stress
  return [
    { key: 'sxx', label: 'sigma_xx', value: s.exx },
    { key: 'syy', label: 'sigma_yy', value: s.eyy },
    { key: 'szz', label: 'sigma_zz', value: s.ezz },
    { key: 'sxy', label: 'tau_xy', value: s.gxy },
    { key: 'syz', label: 'tau_yz', value: s.gyz },
    { key: 'sxz', label: 'tau_xz', value: s.gxz },
  ]
})

const strainComponents = computed(() => {
  if (!results.value) return []
  const e = results.value.volume_averaged_strain
  return [
    { key: 'exx', label: 'exx', value: e.exx },
    { key: 'eyy', label: 'eyy', value: e.eyy },
    { key: 'ezz', label: 'ezz', value: e.ezz },
    { key: 'gxy', label: 'gxy', value: e.gxy },
    { key: 'gyz', label: 'gyz', value: e.gyz },
    { key: 'gxz', label: 'gxz', value: e.gxz },
  ]
})

// ============ Methods ============

function applyTemplate(tpl: HomogenizationTemplate) {
  config.method = tpl.method
  switch (tpl.id) {
    case 'ud_fiber_composite':
      config.method = 'mori_tanaka'
      rveParams.matrixE_GPa = 3.5
      rveParams.matrixNu = 0.35
      rveParams.inclusionE_GPa = 230
      rveParams.inclusionNu = 0.2
      rveParams.inclusionVF = 60
      macroStrain.exx = 0.01
      macroStrain.eyy = 0.0
      macroStrain.ezz = 0.0
      macroStrain.gxy = 0.0
      macroStrain.gyz = 0.0
      macroStrain.gxz = 0.0
      config.convergence_tolerance = 1e-6
      config.max_iterations = 50
      config.temperature = 25
      break
    case 'particle_reinforced_metal':
      config.method = 'mori_tanaka'
      rveParams.matrixE_GPa = 70
      rveParams.matrixNu = 0.33
      rveParams.inclusionE_GPa = 410
      rveParams.inclusionNu = 0.17
      rveParams.inclusionVF = 15
      macroStrain.exx = 0.005
      macroStrain.eyy = 0.0
      macroStrain.ezz = 0.0
      macroStrain.gxy = 0.002
      macroStrain.gyz = 0.0
      macroStrain.gxz = 0.0
      config.convergence_tolerance = 1e-6
      config.max_iterations = 50
      config.temperature = 25
      break
    case 'porous_material':
      config.method = 'self_consistent'
      rveParams.matrixE_GPa = 70
      rveParams.matrixNu = 0.33
      rveParams.inclusionE_GPa = 0.001
      rveParams.inclusionNu = 0.49
      rveParams.inclusionVF = 30
      macroStrain.exx = 0.01
      macroStrain.eyy = 0.01
      macroStrain.ezz = 0.01
      macroStrain.gxy = 0.0
      macroStrain.gyz = 0.0
      macroStrain.gxz = 0.0
      config.convergence_tolerance = 1e-8
      config.max_iterations = 100
      config.temperature = 25
      break
    case 'woven_composite':
      config.method = 'fe2'
      rveParams.matrixE_GPa = 3.5
      rveParams.matrixNu = 0.35
      rveParams.inclusionE_GPa = 230
      rveParams.inclusionNu = 0.2
      rveParams.inclusionVF = 50
      macroStrain.exx = 0.01
      macroStrain.eyy = 0.01
      macroStrain.ezz = 0.0
      macroStrain.gxy = 0.005
      macroStrain.gyz = 0.0
      macroStrain.gxz = 0.0
      config.convergence_tolerance = 1e-6
      config.max_iterations = 100
      config.temperature = 25
      break
  }
}

function resetAll() {
  results.value = null
  config.method = 'mori_tanaka'
  config.convergence_tolerance = 1e-6
  config.max_iterations = 50
  config.temperature = 25
  macroStrain.exx = 0.01
  macroStrain.eyy = 0.0
  macroStrain.ezz = 0.0
  macroStrain.gxy = 0.0
  macroStrain.gyz = 0.0
  macroStrain.gxz = 0.0
  rveParams.matrixE_GPa = 3.5
  rveParams.matrixNu = 0.35
  rveParams.inclusionE_GPa = 230
  rveParams.inclusionNu = 0.2
  rveParams.inclusionVF = 60
  importRve.value = false
}

function formatSci(val: number): string {
  if (Math.abs(val) < 1e-15) return '0.00'
  if (Math.abs(val) >= 1e6 || Math.abs(val) < 0.01) {
    return val.toExponential(2)
  }
  return val.toFixed(4)
}

function formatMatrixCell(val: number): string {
  if (Math.abs(val) < 1e-20) return '0.00'
  return val.toExponential(2)
}

/**
 * Compute effective properties using Mori-Tanaka method
 */
function computeMoriTanaka(
  Em: number, vm: number, num: number,
  Ei: number, vi: number, nui: number
): EffectiveProperties {
  const Km = Em / (3 * (1 - 2 * num))
  const Gm = Em / (2 * (1 + num))
  const Ki = Ei / (3 * (1 - 2 * nui))
  const Gi = Ei / (2 * (1 + nui))

  // Mori-Tanaka estimates (iterative to avoid circular reference)
  const Keff = Ki + vm * (Km - Ki) / (1 + vi * (Km - Ki) / (Km + 4 * Gm / 3))
  let Geff = Gi + vm * (Gm - Gi) / (1 + vi * (Gm - Gi) / (Gm + Gi * (9 * Km + 8 * Gm) / (6 * (Km + 2 * Gm))))
  for (let iter = 0; iter < 20; iter++) {
    const denom = Gm + Geff * (9 * Km + 8 * Gm) / (6 * (Km + 2 * Gm))
    const GeffNew = Gi + vm * (Gm - Gi) / (1 + vi * (Gm - Gi) / denom)
    if (Math.abs(GeffNew - Geff) < 1e-6) { Geff = GeffNew; break }
    Geff = GeffNew
  }

  const Eeff = 9 * Keff * Geff / (3 * Keff + Geff)
  const nueff = (3 * Keff - 2 * Geff) / (2 * (3 * Keff + Geff))

  // For UD composite: transversely isotropic
  const E1 = Ei * vi + Em * vm
  const nu12 = nui * vi + num * vm
  const E2 = Eeff
  const G12 = Geff

  return {
    Ex: E1,
    Ey: E2,
    Ez: E2,
    Gxy: G12,
    Gyz: G12 * 0.8,
    Gxz: G12 * 0.8,
    nu_xy: nu12,
    nu_yz: nueff,
    nu_xz: nu12,
  }
}

/**
 * Compute effective properties using Voigt method
 */
function computeVoigt(
  Em: number, vm: number, num: number,
  Ei: number, vi: number, nui: number
): EffectiveProperties {
  const E = Em * vm + Ei * vi
  const nu = num * vm + nui * vi
  const G = Em / (2 * (1 + num)) * vm + Ei / (2 * (1 + nui)) * vi

  return {
    Ex: E,
    Ey: E,
    Ez: E,
    Gxy: G,
    Gyz: G,
    Gxz: G,
    nu_xy: nu,
    nu_yz: nu,
    nu_xz: nu,
  }
}

/**
 * Compute effective properties using Reuss method
 */
function computeReuss(
  Em: number, vm: number, num: number,
  Ei: number, vi: number, nui: number
): EffectiveProperties {
  const E = 1 / (vm / Em + vi / Ei)
  const G = 1 / (vm / (Em / (2 * (1 + num))) + vi / (Ei / (2 * (1 + nui))))
  const nu = num * vm + nui * vi

  return {
    Ex: E,
    Ey: E,
    Ez: E,
    Gxy: G,
    Gyz: G,
    Gxz: G,
    nu_xy: nu,
    nu_yz: nu,
    nu_xz: nu,
  }
}

/**
 * Compute effective properties using Hill average
 */
function computeHill(
  Em: number, vm: number, num: number,
  Ei: number, vi: number, nui: number
): EffectiveProperties {
  const voigt = computeVoigt(Em, vm, num, Ei, vi, nui)
  const reuss = computeReuss(Em, vm, num, Ei, vi, nui)

  return {
    Ex: (voigt.Ex + reuss.Ex) / 2,
    Ey: (voigt.Ey + reuss.Ey) / 2,
    Ez: (voigt.Ez + reuss.Ez) / 2,
    Gxy: (voigt.Gxy + reuss.Gxy) / 2,
    Gyz: (voigt.Gyz + reuss.Gyz) / 2,
    Gxz: (voigt.Gxz + reuss.Gxz) / 2,
    nu_xy: (voigt.nu_xy + reuss.nu_xy) / 2,
    nu_yz: (voigt.nu_yz + reuss.nu_yz) / 2,
    nu_xz: (voigt.nu_xz + reuss.nu_xz) / 2,
  }
}

/**
 * Compute effective properties using self-consistent method
 */
function computeSelfConsistent(
  Em: number, vm: number, num: number,
  Ei: number, vi: number, nui: number
): EffectiveProperties {
  // Iterative self-consistent scheme
  let Keff = vm * Em / (3 * (1 - 2 * num)) + vi * Ei / (3 * (1 - 2 * nui))
  let Geff = vm * Em / (2 * (1 + num)) + vi * Ei / (2 * (1 + nui))

  for (let iter = 0; iter < 30; iter++) {
    const Km = Em / (3 * (1 - 2 * num))
    const Gm = Em / (2 * (1 + num))
    const Ki = Ei / (3 * (1 - 2 * nui))
    const Gi = Ei / (2 * (1 + nui))

    const alpha = (3 * Keff) / (3 * Keff + 4 * Geff)
    const beta = (6 * (Keff + 2 * Geff)) / (5 * (3 * Keff + 4 * Geff))

    const Knew = vm * Km / (1 + alpha * (Km - Keff) / Keff) + vi * Ki / (1 + alpha * (Ki - Keff) / Keff)
    const Gnew = vm * Gm / (1 + beta * (Gm - Geff) / Geff) + vi * Gi / (1 + beta * (Gi - Geff) / Geff)

    Keff = Knew
    Geff = Gnew
  }

  const Eeff = 9 * Keff * Geff / (3 * Keff + Geff)
  const nueff = (3 * Keff - 2 * Geff) / (2 * (3 * Keff + Geff))

  return {
    Ex: Eeff,
    Ey: Eeff,
    Ez: Eeff,
    Gxy: Geff,
    Gyz: Geff,
    Gxz: Geff,
    nu_xy: nueff,
    nu_yz: nueff,
    nu_xz: nueff,
  }
}

/**
 * Build 6x6 stiffness matrix from effective properties
 */
function buildStiffnessMatrix(ep: EffectiveProperties): number[][] {
  const C: number[][] = Array.from({ length: 6 }, () => Array(6).fill(0))

  const nu_yx = ep.nu_xy * ep.Ey / ep.Ex
  const delta = 1 - ep.nu_xy * nu_yx - ep.nu_yz * ep.nu_xz - ep.nu_xz * nu_yx * ep.nu_yz
    - 2 * ep.nu_xy * ep.nu_yz * ep.nu_xz

  const invDelta = 1 / delta

  C[0][0] = ep.Ex * (1 - ep.nu_yz * ep.nu_xz) * invDelta
  C[0][1] = ep.nu_xy * (1 + ep.nu_yz * ep.nu_xz) * invDelta * ep.Ex
  C[0][2] = ep.nu_xz * (1 + ep.nu_xy * ep.nu_yz) * invDelta * ep.Ex
  C[1][1] = ep.Ey * (1 - ep.nu_yz * ep.nu_xz) * invDelta
  C[1][2] = ep.nu_yz * (1 + ep.nu_xy * ep.nu_yz) * invDelta * ep.Ey
  C[2][2] = ep.Ez * (1 - ep.nu_xy * ep.nu_yz) * invDelta
  C[3][3] = ep.Gxy
  C[4][4] = ep.Gyz
  C[5][5] = ep.Gxz

  // Symmetric
  C[1][0] = C[0][1]
  C[2][0] = C[0][2]
  C[2][1] = C[1][2]

  return C
}

/**
 * Invert 6x6 matrix using Gaussian elimination
 */
function invertMatrix6x6(A: number[][]): number[][] {
  const n = 6
  // Augment with identity
  const aug: number[][] = A.map((row, i) => {
    const newRow = [...row]
    for (let j = 0; j < n; j++) {
      newRow.push(i === j ? 1 : 0)
    }
    return newRow
  })

  for (let col = 0; col < n; col++) {
    // Find pivot
    let maxRow = col
    let maxVal = Math.abs(aug[col][col])
    for (let row = col + 1; row < n; row++) {
      if (Math.abs(aug[row][col]) > maxVal) {
        maxVal = Math.abs(aug[row][col])
        maxRow = row
      }
    }

    // Swap rows
    [aug[col], aug[maxRow]] = [aug[maxRow], aug[col]]

    // Scale pivot row
    const pivot = aug[col][col]
    for (let j = 0; j < 2 * n; j++) {
      aug[col][j] /= pivot
    }

    // Eliminate column
    for (let row = 0; row < n; row++) {
      if (row === col) continue
      const factor = aug[row][col]
      for (let j = 0; j < 2 * n; j++) {
        aug[row][j] -= factor * aug[col][j]
      }
    }
  }

  return aug.map(row => row.slice(n))
}

/**
 * Generate convergence history
 */
function generateConvergenceHistory(
  method: HomogenizationMethod,
  tolerance: number,
  maxIter: number
): Array<{ iteration: number; error: number }> {
  const history: Array<{ iteration: number; error: number }> = []

  let initialError: number
  let decayRate: number

  switch (method) {
    case 'voigt':
    case 'reuss':
      // Analytical: converges in 1 iteration
      initialError = 1e-2
      decayRate = 0.01
      break
    case 'hill':
      initialError = 1e-2
      decayRate = 0.01
      break
    case 'mori_tanaka':
      initialError = 1e-1
      decayRate = 0.3
      break
    case 'self_consistent':
      initialError = 1e-1
      decayRate = 0.5
      break
    case 'fe2':
      initialError = 1
      decayRate = 0.6
      break
    default:
      initialError = 1e-1
      decayRate = 0.4
  }

  let error = initialError
  let converged = false

  for (let i = 1; i <= maxIter; i++) {
    error = initialError * Math.pow(decayRate, i - 1)
    history.push({ iteration: i, error })

    if (error < tolerance) {
      converged = true
      break
    }
  }

  // Ensure we have at least a few points
  if (history.length < 3) {
    for (let i = history.length + 1; i <= 5; i++) {
      history.push({ iteration: i, error: tolerance * 0.1 })
    }
  }

  return history
}

/**
 * Generate mock homogenization results
 */
function generateMockResults(): HomogenizationResult {
  const Em = rveParams.matrixE_GPa * 1e9
  const Ei = rveParams.inclusionE_GPa * 1e9
  const vm = 1 - rveParams.inclusionVF / 100
  const vi = rveParams.inclusionVF / 100

  let effectiveProps: EffectiveProperties

  switch (config.method) {
    case 'voigt':
      effectiveProps = computeVoigt(Em, vm, rveParams.matrixNu, Ei, vi, rveParams.inclusionNu)
      break
    case 'reuss':
      effectiveProps = computeReuss(Em, vm, rveParams.matrixNu, Ei, vi, rveParams.inclusionNu)
      break
    case 'hill':
      effectiveProps = computeHill(Em, vm, rveParams.matrixNu, Ei, vi, rveParams.inclusionNu)
      break
    case 'mori_tanaka':
      effectiveProps = computeMoriTanaka(Em, vm, rveParams.matrixNu, Ei, vi, rveParams.inclusionNu)
      break
    case 'self_consistent':
      effectiveProps = computeSelfConsistent(Em, vm, rveParams.matrixNu, Ei, vi, rveParams.inclusionNu)
      break
    case 'fe2':
      // FE2 uses Mori-Tanaka as base but with FE refinement
      effectiveProps = computeMoriTanaka(Em, vm, rveParams.matrixNu, Ei, vi, rveParams.inclusionNu)
      // Add small perturbation to simulate FE2 refinement
      effectiveProps.Ex *= (1 + 0.02 * (Math.random() - 0.5))
      effectiveProps.Ey *= (1 + 0.03 * (Math.random() - 0.5))
      effectiveProps.Ez *= (1 + 0.03 * (Math.random() - 0.5))
      effectiveProps.Gxy *= (1 + 0.04 * (Math.random() - 0.5))
      effectiveProps.Gyz *= (1 + 0.05 * (Math.random() - 0.5))
      effectiveProps.Gxz *= (1 + 0.05 * (Math.random() - 0.5))
      break
    default:
      effectiveProps = computeMoriTanaka(Em, vm, rveParams.matrixNu, Ei, vi, rveParams.inclusionNu)
  }

  const C = buildStiffnessMatrix(effectiveProps)
  const S = invertMatrix6x6(C)

  const convergenceHistory = generateConvergenceHistory(
    config.method,
    config.convergence_tolerance,
    config.max_iterations
  )

  // Volume averaged stress from macro strain and stiffness
  const strainVec = [macroStrain.exx, macroStrain.eyy, macroStrain.ezz, macroStrain.gxy, macroStrain.gyz, macroStrain.gxz]
  const stressVec = strainVec.map((_, i) =>
    C[i].reduce((sum, cij, j) => sum + cij * strainVec[j], 0)
  )

  // Strain concentration tensors (simplified: identity + perturbation)
  const strainConcentration = Array.from({ length: 2 }, () =>
    Array.from({ length: 6 }, () =>
      Array.from({ length: 6 }, (_, j) => (j < 3 ? 1.0 : 0.0) + (Math.random() - 0.5) * 0.1)
    )
  )

  // Stress concentration tensors
  const stressConcentration = Array.from({ length: 2 }, () =>
    Array.from({ length: 6 }, () =>
      Array.from({ length: 6 }, () => C[0][0] * (0.5 + Math.random()))
    )
  )

  return {
    success: true,
    effective_properties: effectiveProps,
    stiffness_matrix_C: C,
    compliance_matrix_S: S,
    strain_concentration_tensors: strainConcentration,
    stress_concentration_tensors: stressConcentration,
    convergence_history: convergenceHistory,
    volume_averaged_stress: {
      exx: stressVec[0],
      eyy: stressVec[1],
      ezz: stressVec[2],
      gxy: stressVec[3],
      gyz: stressVec[4],
      gxz: stressVec[5],
    },
    volume_averaged_strain: {
      exx: macroStrain.exx,
      eyy: macroStrain.eyy,
      ezz: macroStrain.ezz,
      gxy: macroStrain.gxy,
      gyz: macroStrain.gyz,
      gxz: macroStrain.gxz,
    },
  }
}

async function runHomogenization() {
  running.value = true
  try {
    config.macro_strains = [{ ...macroStrain }]
    await new Promise(resolve => setTimeout(resolve, 2000))
    results.value = generateMockResults()
  } catch (e) {
    console.error('Homogenization failed:', e)
  } finally {
    running.value = false
  }
}
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

.matrix-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 10px;
}

.matrix-header {
  padding: 3px 5px;
  color: var(--text-muted);
  font-weight: 500;
  text-align: center;
  white-space: nowrap;
}

.matrix-cell {
  padding: 3px 5px;
  color: var(--text-secondary);
  font-family: monospace;
  font-size: 9px;
  text-align: right;
  white-space: nowrap;
}

.matrix-diagonal {
  color: var(--primary);
  font-weight: 600;
}
</style>
