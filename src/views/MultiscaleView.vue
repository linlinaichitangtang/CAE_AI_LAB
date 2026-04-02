<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">多尺度桥接 (MD/相场 → FE)</h2>
        <p class="text-sm" style="color: var(--text-muted)">粗粒化参数配置，等效边界条件生成，多尺度数据流可视化</p>
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

        <!-- Step 1: Bridge Method -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            桥接方法
          </h4>
          <div class="flex flex-col gap-1">
            <button
              v-for="m in bridgeMethods"
              :key="m.value"
              @click="config.method = m.value"
              :class="['px-3 py-2 rounded text-xs text-left transition border', config.method === m.value ? 'text-white' : '']"
              :style="config.method === m.value
                ? 'background: var(--primary); border-color: var(--primary)'
                : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
            >
              <div class="font-medium">{{ m.label }}</div>
              <div class="text-[10px] mt-0.5 opacity-80">{{ m.desc }}</div>
            </button>
          </div>
        </div>

        <!-- Step 2: Source Data Input -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            细观数据输入
          </h4>

          <!-- Atomistic Data -->
          <div v-if="isAtomisticMethod" class="space-y-2">
            <div>
              <label class="label">原子数</label>
              <input v-model.number="atomisticData.num_atoms" type="number" step="1000" class="input w-full text-xs" />
            </div>
            <div class="grid grid-cols-3 gap-2">
              <div>
                <label class="label">盒子 X (A)</label>
                <input v-model.number="atomisticData.box_size.x" type="number" step="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">盒子 Y (A)</label>
                <input v-model.number="atomisticData.box_size.y" type="number" step="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">盒子 Z (A)</label>
                <input v-model.number="atomisticData.box_size.z" type="number" step="1" class="input w-full text-xs" />
              </div>
            </div>
            <div>
              <label class="label">势函数</label>
              <select v-model="atomisticData.potential_type" class="input w-full text-xs">
                <option value="eam">EAM (嵌入原子法)</option>
                <option value="lj">Lennard-Jones</option>
                <option value="tersoff">Tersoff</option>
                <option value="reaxff">ReaxFF (反应力场)</option>
              </select>
            </div>
            <div>
              <label class="label">原子种类 (逗号分隔)</label>
              <input v-model="speciesStr" type="text" class="input w-full text-xs" placeholder="Cu, Al" />
            </div>
            <div class="grid grid-cols-3 gap-2">
              <div>
                <label class="label">温度 (K)</label>
                <input v-model.number="atomisticData.temperature" type="number" step="10" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">时间步 (fs)</label>
                <input v-model.number="atomisticData.time_step_fs" type="number" step="0.1" class="input w-full text-xs" />
              </div>
            </div>
            <div>
              <label class="label">应力张量 (xx,yy,zz,xy,yz,xz) GPa</label>
              <input v-model="stressStr" type="text" class="input w-full text-xs" placeholder="0,0,0,0,0,0" />
            </div>
            <div>
              <label class="label">应变张量 (xx,yy,zz,xy,yz,xz)</label>
              <input v-model="strainStr" type="text" class="input w-full text-xs" placeholder="0,0,0,0,0,0" />
            </div>
          </div>

          <!-- Phase Field Data -->
          <div v-else class="space-y-2">
            <div class="grid grid-cols-3 gap-2">
              <div>
                <label class="label">网格 Nx</label>
                <input v-model.number="phaseFieldData.grid_size.nx" type="number" step="10" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">网格 Ny</label>
                <input v-model.number="phaseFieldData.grid_size.ny" type="number" step="10" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">网格 Nz</label>
                <input v-model.number="phaseFieldData.grid_size.nz" type="number" step="10" class="input w-full text-xs" />
              </div>
            </div>
            <div>
              <label class="label">场变量 (逗号分隔)</label>
              <input v-model="fieldVarsStr" type="text" class="input w-full text-xs" placeholder="eta, phi, mu" />
            </div>
            <div>
              <label class="label">界面宽度</label>
              <input v-model.number="phaseFieldData.interface_width" type="number" step="0.1" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">迁移率</label>
              <input v-model.number="phaseFieldData.mobility" type="number" step="0.01" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">自由能密度 (J/m³)</label>
              <input v-model.number="phaseFieldData.free_energy_density" type="number" step="1e3" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">晶界数量</label>
              <input v-model.number="grainBoundaryCount" type="number" step="1" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Step 3: Target Settings -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            目标设置
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">目标网格尺寸 (nm)</label>
              <input v-model.number="config.target_mesh_size" type="number" step="1" min="1" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">耦合类型</label>
              <div class="flex gap-1">
                <button
                  v-for="ct in couplingTypes"
                  :key="ct.value"
                  @click="config.coupling_type = ct.value"
                  :class="['flex-1 px-2 py-1.5 rounded text-xs transition border', config.coupling_type === ct.value ? 'text-white' : '']"
                  :style="config.coupling_type === ct.value
                    ? 'background: var(--primary); border-color: var(--primary)'
                    : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
                >
                  {{ ct.label }}
                </button>
              </div>
            </div>
            <div>
              <label class="label">重叠区域 (nm)</label>
              <input v-model.number="config.overlap_region" type="number" step="0.5" min="0" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">松弛迭代次数</label>
              <input v-model.number="config.relaxation_iterations" type="number" step="10" min="1" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="space-y-2">
          <button
            @click="runBridge"
            :disabled="running"
            class="btn btn-primary text-xs w-full"
          >
            <span v-if="running" class="mr-1 animate-spin">&#10227;</span>
            {{ running ? '桥接计算中...' : '运行多尺度桥接' }}
          </button>
          <button
            @click="runCoarseGraining"
            :disabled="coarseGraining"
            class="btn btn-ghost text-xs w-full"
          >
            <span v-if="coarseGraining" class="mr-1 animate-spin">&#10227;</span>
            {{ coarseGraining ? '粗粒化中...' : '粗粒化分析' }}
          </button>
        </div>
      </div>

      <!-- Right Panel: Visualization -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- No results placeholder -->
        <div v-if="!results && !coarseResult" class="flex-1 flex items-center justify-center" style="color: var(--text-muted)">
          <div class="text-center">
            <div class="text-4xl mb-2">&#128202;</div>
            <div class="text-sm">配置参数后运行多尺度桥接分析</div>
          </div>
        </div>

        <!-- Results Content -->
        <template v-else>
          <div class="flex-1 flex overflow-hidden">
            <!-- Main Visualization Area -->
            <div class="flex-1 flex flex-col overflow-hidden">
              <!-- Data Flow Visualization Canvas -->
              <div class="flex-1 relative">
                <canvas ref="flowCanvas" class="w-full h-full" style="background: var(--bg-base)"></canvas>
              </div>

              <!-- Bottom Panels -->
              <div class="h-64 flex border-t" style="border-color: var(--border-subtle)">
                <!-- Equivalent BC Preview -->
                <div class="flex-1 overflow-y-auto p-3 border-r" style="border-color: var(--border-subtle); background: var(--bg-surface)">
                  <h4 class="text-xs font-medium mb-2" style="color: var(--text-secondary)">等效边界条件预览</h4>
                  <div v-if="results" class="space-y-1">
                    <div class="text-[10px]" style="color: var(--text-muted)">
                      映射节点数: {{ results.equivalent_boundary_conditions.length }}
                    </div>
                    <div class="grid grid-cols-4 gap-1 text-[10px] font-mono" style="color: var(--text-primary)">
                      <div class="font-medium" style="color: var(--text-muted)">节点</div>
                      <div class="font-medium" style="color: var(--text-muted)">自由度</div>
                      <div class="font-medium" style="color: var(--text-muted)">值</div>
                      <div class="font-medium" style="color: var(--text-muted)">单位</div>
                      <template v-for="bc in results.equivalent_boundary_conditions.slice(0, 8)" :key="bc.node_id + bc.dof">
                        <div>{{ bc.node_id }}</div>
                        <div>{{ bc.dof }}</div>
                        <div>{{ bc.value.toFixed(4) }}</div>
                        <div>{{ bc.dof.startsWith('U') ? 'nm' : 'nN' }}</div>
                      </template>
                    </div>
                  </div>
                </div>

                <!-- Cost Comparison Bar Chart -->
                <div class="w-72 overflow-y-auto p-3" style="background: var(--bg-surface)">
                  <h4 class="text-xs font-medium mb-2" style="color: var(--text-secondary)">计算成本对比</h4>
                  <div v-if="results" class="space-y-3">
                    <div>
                      <div class="flex justify-between text-[10px] mb-1">
                        <span style="color: var(--text-secondary)">全原子模拟</span>
                        <span class="font-mono" style="color: var(--accent-red)">{{ results.computational_cost_reduction.full_atomistic_time.toFixed(1) }} h</span>
                      </div>
                      <div class="w-full h-4 rounded-full" style="background: var(--bg-elevated)">
                        <div class="h-4 rounded-full transition-all" style="background: var(--accent-red); width: 100%"></div>
                      </div>
                    </div>
                    <div>
                      <div class="flex justify-between text-[10px] mb-1">
                        <span style="color: var(--text-secondary)">多尺度方法</span>
                        <span class="font-mono" style="color: var(--accent-green)">{{ results.computational_cost_reduction.multiscale_time.toFixed(1) }} h</span>
                      </div>
                      <div class="w-full h-4 rounded-full" style="background: var(--bg-elevated)">
                        <div
                          class="h-4 rounded-full transition-all"
                          :style="{ background: 'var(--accent-green)', width: costBarWidth + '%' }"
                        ></div>
                      </div>
                    </div>
                    <div class="pt-2 border-t" style="border-color: var(--border-subtle)">
                      <div class="flex justify-between text-[10px]">
                        <span style="color: var(--text-secondary)">加速比</span>
                        <span class="font-mono font-semibold" style="color: var(--primary)">{{ results.computational_cost_reduction.speedup_factor.toFixed(1) }}x</span>
                      </div>
                      <div class="flex justify-between text-[10px] mt-1">
                        <span style="color: var(--text-secondary)">精度保留</span>
                        <span class="font-mono font-semibold" style="color: var(--accent-green)">{{ (results.computational_cost_reduction.accuracy_retention * 100).toFixed(1) }}%</span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Right Sidebar: Coarse-grained Parameters -->
            <div class="w-72 overflow-y-auto border-l p-4 space-y-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <!-- Coarse-grained Parameters -->
              <div class="result-card" v-if="results">
                <span class="result-label">粗粒化参数</span>
                <div class="space-y-2 mt-2">
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">等效模量</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ formatSci(results.coarse_grained_parameters.equivalent_modulus) }} Pa</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">等效强度</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ formatSci(results.coarse_grained_parameters.equivalent_strength) }} Pa</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">等效泊松比</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ results.coarse_grained_parameters.equivalent_poisson_ratio.toFixed(4) }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">等效密度</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ results.coarse_grained_parameters.equivalent_density.toFixed(1) }} kg/m³</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">等效硬化参数</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ formatSci(results.coarse_grained_parameters.equivalent_hardening) }} Pa</span>
                  </div>
                </div>
              </div>

              <!-- Mapped Stress/Strain Summary -->
              <div class="result-card" v-if="results">
                <span class="result-label">映射场统计</span>
                <div class="space-y-2 mt-2">
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">应力场节点数</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ results.mapped_stress_field.length }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">应变场节点数</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ results.mapped_strain_field.length }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">最大 von-Mises</span>
                    <span class="text-xs font-mono" style="color: var(--accent-red)">{{ maxVonMises.toFixed(1) }} MPa</span>
                  </div>
                </div>
              </div>

              <!-- Coarse Grain Result -->
              <div class="result-card" v-if="coarseResult">
                <span class="result-label">粗粒化结果</span>
                <div class="space-y-2 mt-2">
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">代表体积尺寸</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ coarseResult.representative_volume_size.toFixed(2) }} nm</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">均匀化方法</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ coarseResult.homogenization_method }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">迭代次数</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ coarseResult.convergence_info.iterations }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">残差</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ coarseResult.convergence_info.residual.toExponential(2) }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">收敛状态</span>
                    <span class="text-xs font-mono" :style="{ color: coarseResult.convergence_info.converged ? 'var(--accent-green)' : 'var(--accent-red)' }">
                      {{ coarseResult.convergence_info.converged ? '已收敛' : '未收敛' }}
                    </span>
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
import { ref, reactive, computed, onMounted, nextTick, watch } from 'vue'
import type {
  BridgeMethod,
  CouplingType,
  AtomisticData,
  PhaseFieldData,
  MultiscaleBridgeConfig,
  MultiscaleBridgeResult,
  CoarseGrainedResult,
  MultiscaleTemplate,
  ScaleLevel,
} from '../api/multiscale'

// ============ Constants ============

const bridgeMethods = [
  { value: 'atomistic_to_fe' as BridgeMethod, label: '原子 → FE', desc: 'MD 分子动力学到有限元' },
  { value: 'phase_field_to_fe' as BridgeMethod, label: '相场 → FE', desc: '相场模型到有限元' },
  { value: 'coarse_graining' as BridgeMethod, label: '粗粒化', desc: '统计粗粒化降尺度' },
  { value: 'parameter_passing' as BridgeMethod, label: '参数传递', desc: '跨尺度参数传递' },
  { value: 'concurrent' as BridgeMethod, label: '并发耦合', desc: '并发多尺度耦合' },
]

const couplingTypes = [
  { value: 'sequential' as CouplingType, label: '顺序' },
  { value: 'concurrent' as CouplingType, label: '并发' },
  { value: 'hierarchical' as CouplingType, label: '分层' },
]

const quickTemplates: MultiscaleTemplate[] = [
  {
    id: 'metal_plasticity',
    name: '金属塑性',
    name_en: 'Metal Plasticity (Dislocation → Continuum)',
    category: 'plasticity',
    description: '位错动力学到连续体塑性桥接',
    method: 'atomistic_to_fe',
    typical_application: '金属塑性成形',
  },
  {
    id: 'crack_propagation',
    name: '裂纹扩展',
    name_en: 'Crack Propagation (Atomistic → Phase Field → FE)',
    category: 'fracture',
    description: '原子→相场→FE 多尺度裂纹扩展',
    method: 'phase_field_to_fe',
    typical_application: '断裂力学',
  },
  {
    id: 'nanocomposite',
    name: '纳米复合材料',
    name_en: 'Nanocomposite',
    category: 'composite',
    description: '纳米复合材料多尺度建模',
    method: 'coarse_graining',
    typical_application: '复合材料力学',
  },
  {
    id: 'polycrystal',
    name: '多晶变形',
    name_en: 'Polycrystal Deformation',
    category: 'crystal',
    description: '多晶体塑性变形多尺度分析',
    method: 'concurrent',
    typical_application: '晶体塑性',
  },
]

// ============ State ============

const running = ref(false)
const coarseGraining = ref(false)
const results = ref<MultiscaleBridgeResult | null>(null)
const coarseResult = ref<CoarseGrainedResult | null>(null)
const flowCanvas = ref<HTMLCanvasElement | null>(null)

const speciesStr = ref('Cu, Al')
const stressStr = ref('1.2, 0.8, 0.9, 0.1, 0.05, 0.02')
const strainStr = ref('0.01, 0.008, 0.009, 0.002, 0.001, 0.0005')
const fieldVarsStr = ref('eta, phi, mu')
const grainBoundaryCount = ref(12)

const atomisticData = reactive<AtomisticData>({
  num_atoms: 1024000,
  box_size: { x: 50, y: 50, z: 50 },
  potential_type: 'eam',
  species: ['Cu', 'Al'],
  stress_tensor: [1.2, 0.8, 0.9, 0.1, 0.05, 0.02],
  strain_tensor: [0.01, 0.008, 0.009, 0.002, 0.001, 0.0005],
  temperature: 300,
  time_step_fs: 1.0,
})

const phaseFieldData = reactive<PhaseFieldData>({
  grid_size: { nx: 128, ny: 128, nz: 64 },
  field_variables: ['eta', 'phi', 'mu'],
  interface_width: 2.5,
  mobility: 1.0,
  free_energy_density: 5.0e5,
  grain_boundaries: Array.from({ length: 12 }, (_, i) => i),
})

const config = reactive<MultiscaleBridgeConfig>({
  project_id: '',
  method: 'atomistic_to_fe',
  source_data: atomisticData,
  target_mesh_size: 5,
  coupling_type: 'sequential',
  overlap_region: 2.0,
  relaxation_iterations: 100,
})

// ============ Computed ============

const isAtomisticMethod = computed(() => {
  return config.method === 'atomistic_to_fe' || config.method === 'coarse_graining' || config.method === 'parameter_passing'
})

const costBarWidth = computed(() => {
  if (!results.value) return 0
  const ratio = results.value.computational_cost_reduction.multiscale_time
    / results.value.computational_cost_reduction.full_atomistic_time
  return Math.max(2, ratio * 100)
})

const maxVonMises = computed(() => {
  if (!results.value) return 0
  let max = 0
  for (const s of results.value.mapped_stress_field) {
    const vm = Math.sqrt(
      0.5 * ((s.sxx - s.syy) ** 2 + (s.syy - s.szz) ** 2 + (s.szz - s.sxx) ** 2 + 6 * (s.sxy ** 2 + s.syz ** 2 + s.sxz ** 2))
    )
    if (vm > max) max = vm
  }
  return max
})

// ============ Watchers ============

watch(speciesStr, (val) => {
  atomisticData.species = val.split(',').map(s => s.trim()).filter(Boolean)
})

watch(stressStr, (val) => {
  atomisticData.stress_tensor = val.split(',').map(s => parseFloat(s.trim()) || 0)
})

watch(strainStr, (val) => {
  atomisticData.strain_tensor = val.split(',').map(s => parseFloat(s.trim()) || 0)
})

watch(fieldVarsStr, (val) => {
  phaseFieldData.field_variables = val.split(',').map(s => s.trim()).filter(Boolean)
})

watch(grainBoundaryCount, (val) => {
  phaseFieldData.grain_boundaries = Array.from({ length: val }, (_, i) => i)
})

watch(() => config.method, (method) => {
  config.source_data = isAtomisticMethod.value ? atomisticData : phaseFieldData
})

// ============ Methods ============

function formatSci(val: number): string {
  if (Math.abs(val) < 1e-15) return '0.00e+0'
  if (Math.abs(val) >= 1e6 || Math.abs(val) < 0.01) {
    return val.toExponential(2)
  }
  return val.toFixed(4)
}

function applyTemplate(tpl: MultiscaleTemplate) {
  config.method = tpl.method
  switch (tpl.id) {
    case 'metal_plasticity':
      config.method = 'atomistic_to_fe'
      atomisticData.num_atoms = 2048000
      atomisticData.box_size = { x: 80, y: 80, z: 80 }
      atomisticData.potential_type = 'eam'
      speciesStr.value = 'Cu, Al'
      atomisticData.temperature = 300
      config.target_mesh_size = 10
      config.coupling_type = 'sequential'
      config.overlap_region = 5.0
      config.relaxation_iterations = 200
      break
    case 'crack_propagation':
      config.method = 'phase_field_to_fe'
      phaseFieldData.grid_size = { nx: 256, ny: 256, nz: 1 }
      fieldVarsStr.value = 'crack_phase, displacement, stress'
      phaseFieldData.interface_width = 1.5
      phaseFieldData.mobility = 2.0
      config.target_mesh_size = 3
      config.coupling_type = 'hierarchical'
      config.overlap_region = 1.0
      config.relaxation_iterations = 50
      break
    case 'nanocomposite':
      config.method = 'coarse_graining'
      atomisticData.num_atoms = 512000
      atomisticData.box_size = { x: 40, y: 40, z: 40 }
      atomisticData.potential_type = 'lj'
      speciesStr.value = 'C, H, O'
      config.target_mesh_size = 8
      config.coupling_type = 'sequential'
      config.overlap_region = 3.0
      config.relaxation_iterations = 150
      break
    case 'polycrystal':
      config.method = 'concurrent'
      atomisticData.num_atoms = 4096000
      atomisticData.box_size = { x: 100, y: 100, z: 100 }
      atomisticData.potential_type = 'eam'
      speciesStr.value = 'Fe, Ni, Cr'
      config.target_mesh_size = 15
      config.coupling_type = 'concurrent'
      config.overlap_region = 8.0
      config.relaxation_iterations = 300
      break
  }
  config.source_data = isAtomisticMethod.value ? atomisticData : phaseFieldData
}

function resetAll() {
  results.value = null
  coarseResult.value = null
  config.method = 'atomistic_to_fe'
  config.target_mesh_size = 5
  config.coupling_type = 'sequential'
  config.overlap_region = 2.0
  config.relaxation_iterations = 100
  atomisticData.num_atoms = 1024000
  atomisticData.box_size = { x: 50, y: 50, z: 50 }
  atomisticData.potential_type = 'eam'
  atomisticData.temperature = 300
  atomisticData.time_step_fs = 1.0
  speciesStr.value = 'Cu, Al'
  stressStr.value = '1.2, 0.8, 0.9, 0.1, 0.05, 0.02'
  strainStr.value = '0.01, 0.008, 0.009, 0.002, 0.001, 0.0005'
  phaseFieldData.grid_size = { nx: 128, ny: 128, nz: 64 }
  fieldVarsStr.value = 'eta, phi, mu'
  phaseFieldData.interface_width = 2.5
  phaseFieldData.mobility = 1.0
  phaseFieldData.free_energy_density = 5.0e5
  grainBoundaryCount.value = 12
  config.source_data = atomisticData
}

function generateMockResults(): MultiscaleBridgeResult {
  const numNodes = 64
  const equivalentBCs: MultiscaleBridgeResult['equivalent_boundary_conditions'] = []
  const dofs = ['U1', 'U2', 'U3', 'F1', 'F2', 'F3']

  for (let i = 0; i < numNodes; i++) {
    const dof = dofs[i % dofs.length]
    const value = dof.startsWith('U')
      ? (Math.random() - 0.5) * 0.1
      : (Math.random() - 0.5) * 100
    equivalentBCs.push({ node_id: i + 1, dof, value })
  }

  const stressField: MultiscaleBridgeResult['mapped_stress_field'] = []
  const strainField: MultiscaleBridgeResult['mapped_strain_field'] = []
  const gridSize = 8

  for (let ix = 0; ix < gridSize; ix++) {
    for (let iy = 0; iy < gridSize; iy++) {
      const x = ix * config.target_mesh_size
      const y = iy * config.target_mesh_size
      const z = 0
      const distFactor = Math.sqrt((ix - gridSize / 2) ** 2 + (iy - gridSize / 2) ** 2) / (gridSize / 2)

      stressField.push({
        x, y, z,
        sxx: 100 + Math.random() * 50 - distFactor * 30,
        syy: 80 + Math.random() * 40 - distFactor * 20,
        szz: 60 + Math.random() * 30 - distFactor * 15,
        sxy: (Math.random() - 0.5) * 20,
        syz: (Math.random() - 0.5) * 10,
        sxz: (Math.random() - 0.5) * 10,
      })

      strainField.push({
        x, y, z,
        exx: 0.001 + Math.random() * 0.0005,
        eyy: 0.0008 + Math.random() * 0.0004,
        ezz: 0.0006 + Math.random() * 0.0003,
        exy: (Math.random() - 0.5) * 0.0002,
        eyz: (Math.random() - 0.5) * 0.0001,
        exz: (Math.random() - 0.5) * 0.0001,
      })
    }
  }

  const baseModulus = config.method === 'atomistic_to_fe' ? 110e9 : 200e9
  const baseStrength = config.method === 'atomistic_to_fe' ? 250e6 : 800e6

  return {
    success: true,
    equivalent_boundary_conditions: equivalentBCs,
    mapped_stress_field: stressField,
    mapped_strain_field: strainField,
    coarse_grained_parameters: {
      equivalent_modulus: baseModulus * (0.85 + Math.random() * 0.3),
      equivalent_strength: baseStrength * (0.8 + Math.random() * 0.4),
      equivalent_poisson_ratio: 0.3 + Math.random() * 0.05,
      equivalent_density: 2700 + Math.random() * 2000,
      equivalent_hardening: baseModulus * 0.01 * (0.5 + Math.random()),
    },
    data_flow_visualization: {
      scales: [
        { level: 'quantum' as ScaleLevel, label: '量子尺度', data_size: 0, description: 'DFT 电子结构' },
        { level: 'atomistic' as ScaleLevel, label: '原子尺度', data_size: atomisticData.num_atoms, description: 'MD 分子动力学' },
        { level: 'meso' as ScaleLevel, label: '介观尺度', data_size: phaseFieldData.grid_size.nx * phaseFieldData.grid_size.ny, description: '相场/晶体塑性' },
        { level: 'macro' as ScaleLevel, label: '宏观尺度', data_size: numNodes, description: 'FE 有限元' },
      ],
      connections: [
        { from: 'quantum' as ScaleLevel, to: 'atomistic' as ScaleLevel, method: '势函数拟合', data_transfer: 'EAM/LJ 参数' },
        { from: 'atomistic' as ScaleLevel, to: 'meso' as ScaleLevel, method: '粗粒化', data_transfer: '本构参数' },
        { from: 'meso' as ScaleLevel, to: 'macro' as ScaleLevel, method: '均匀化', data_transfer: '等效边界条件' },
        { from: 'atomistic' as ScaleLevel, to: 'macro' as ScaleLevel, method: '直接桥接', data_transfer: '应力/应变场' },
      ],
    },
    computational_cost_reduction: {
      full_atomistic_time: 240 + Math.random() * 120,
      multiscale_time: 8 + Math.random() * 12,
      speedup_factor: 15 + Math.random() * 25,
      accuracy_retention: 0.92 + Math.random() * 0.06,
    },
  }
}

function generateMockCoarseResult(): CoarseGrainedResult {
  const baseE = config.method === 'atomistic_to_fe' ? 110e9 : 200e9
  const stressStrain: Array<{ strain: number; stress: number }> = []
  for (let i = 0; i <= 30; i++) {
    const strain = i * 0.005
    const stress = baseE * strain * (1 - Math.exp(-strain * 500)) * (1 + 0.1 * Math.random())
    stressStrain.push({ strain, stress })
  }

  return {
    success: true,
    representative_volume_size: config.target_mesh_size * (0.8 + Math.random() * 0.4),
    equivalent_properties: {
      E11: baseE * (0.95 + Math.random() * 0.1),
      E22: baseE * (0.9 + Math.random() * 0.15),
      E33: baseE * (0.88 + Math.random() * 0.2),
      G12: baseE * 0.38 * (0.9 + Math.random() * 0.15),
      G23: baseE * 0.35 * (0.85 + Math.random() * 0.2),
      G13: baseE * 0.37 * (0.88 + Math.random() * 0.18),
      nu12: 0.3 + Math.random() * 0.04,
      nu23: 0.28 + Math.random() * 0.05,
      nu13: 0.29 + Math.random() * 0.04,
    },
    stress_strain_response: stressStrain,
    homogenization_method: 'Mori-Tanaka',
    convergence_info: {
      iterations: 15 + Math.floor(Math.random() * 20),
      residual: 1e-6 + Math.random() * 1e-4,
      converged: true,
    },
  }
}

/**
 * Draw multiscale data flow visualization on canvas
 */
function drawDataFlow() {
  if (!flowCanvas.value || (!results.value && !coarseResult.value)) return

  const canvas = flowCanvas.value
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

  ctx.clearRect(0, 0, w, h)
  ctx.fillStyle = '#0A0B0E'
  ctx.fillRect(0, 0, w, h)

  // Define scale nodes
  const scales = [
    { level: 'quantum', label: '量子尺度', sublabel: 'DFT / 电子结构', color: '#a855f7', x: 0, y: 0 },
    { level: 'atomistic', label: '原子尺度', sublabel: 'MD / 分子动力学', color: '#4F8CFF', x: 0, y: 0 },
    { level: 'meso', label: '介观尺度', sublabel: '相场 / 晶体塑性', color: '#22c55e', x: 0, y: 0 },
    { level: 'macro', label: '宏观尺度', sublabel: 'FE / 有限元', color: '#ef4444', x: 0, y: 0 },
  ]

  // Position nodes in a flow layout
  const nodeW = 140
  const nodeH = 80
  const gapX = 40
  const totalWidth = scales.length * nodeW + (scales.length - 1) * gapX
  const startX = (w - totalWidth) / 2
  const centerY = h * 0.42

  for (let i = 0; i < scales.length; i++) {
    scales[i].x = startX + i * (nodeW + gapX)
    scales[i].y = centerY - nodeH / 2
  }

  // Draw connections (arrows)
  const connections = [
    { from: 0, to: 1, method: '势函数拟合', data: 'EAM/LJ 参数' },
    { from: 1, to: 2, method: '粗粒化', data: '本构参数' },
    { from: 2, to: 3, method: '均匀化', data: '等效边界条件' },
    { from: 1, to: 3, method: '直接桥接', data: '应力/应变场' },
  ]

  for (const conn of connections) {
    const fromNode = scales[conn.from]
    const toNode = scales[conn.to]

    const fromX = fromNode.x + nodeW
    const fromY = fromNode.y + nodeH / 2
    const toX = toNode.x
    const toY = toNode.y + nodeH / 2

    // Draw curved arrow
    const midX = (fromX + toX) / 2
    const isDiagonal = Math.abs(conn.from - conn.to) > 1
    const offsetY = isDiagonal ? -40 : 0

    ctx.strokeStyle = 'rgba(255, 255, 255, 0.15)'
    ctx.lineWidth = 2
    ctx.setLineDash(isDiagonal ? [6, 4] : [])
    ctx.beginPath()
    ctx.moveTo(fromX, fromY)
    ctx.quadraticCurveTo(midX, fromY + offsetY, toX, toY + offsetY)
    ctx.stroke()
    ctx.setLineDash([])

    // Arrow head
    const arrowSize = 8
    const angle = Math.atan2(toY + offsetY - (fromY + offsetY), toX - fromX)
    ctx.fillStyle = 'rgba(255, 255, 255, 0.3)'
    ctx.beginPath()
    ctx.moveTo(toX, toY + offsetY)
    ctx.lineTo(toX - arrowSize * Math.cos(angle - 0.4), toY + offsetY - arrowSize * Math.sin(angle - 0.4))
    ctx.lineTo(toX - arrowSize * Math.cos(angle + 0.4), toY + offsetY - arrowSize * Math.sin(angle + 0.4))
    ctx.closePath()
    ctx.fill()

    // Connection label
    const labelX = midX
    const labelY = fromY + offsetY - 12
    ctx.fillStyle = 'rgba(255, 255, 255, 0.5)'
    ctx.font = '10px sans-serif'
    ctx.textAlign = 'center'
    ctx.fillText(conn.method, labelX, labelY)
    ctx.fillStyle = 'rgba(255, 255, 255, 0.3)'
    ctx.font = '9px sans-serif'
    ctx.fillText(conn.data, labelX, labelY + 14)
  }

  // Draw nodes
  for (const node of scales) {
    // Node background
    ctx.fillStyle = 'rgba(20, 21, 26, 0.9)'
    ctx.strokeStyle = node.color
    ctx.lineWidth = 2
    roundRect(ctx, node.x, node.y, nodeW, nodeH, 8)
    ctx.fill()
    ctx.stroke()

    // Glow effect
    ctx.shadowColor = node.color
    ctx.shadowBlur = 15
    ctx.strokeStyle = node.color
    ctx.lineWidth = 1
    roundRect(ctx, node.x, node.y, nodeW, nodeH, 8)
    ctx.stroke()
    ctx.shadowBlur = 0

    // Node label
    ctx.fillStyle = node.color
    ctx.font = 'bold 13px sans-serif'
    ctx.textAlign = 'center'
    ctx.fillText(node.label, node.x + nodeW / 2, node.y + 30)

    // Node sublabel
    ctx.fillStyle = 'rgba(255, 255, 255, 0.6)'
    ctx.font = '10px sans-serif'
    ctx.fillText(node.sublabel, node.x + nodeW / 2, node.y + 50)

    // Data size indicator
    if (node.level === 'atomistic') {
      const dataSize = atomisticData.num_atoms
      ctx.fillStyle = 'rgba(255, 255, 255, 0.4)'
      ctx.font = '9px monospace'
      ctx.fillText(`${(dataSize / 1e6).toFixed(1)}M atoms`, node.x + nodeW / 2, node.y + 68)
    } else if (node.level === 'meso') {
      const gridSize = phaseFieldData.grid_size.nx * phaseFieldData.grid_size.ny
      ctx.fillStyle = 'rgba(255, 255, 255, 0.4)'
      ctx.font = '9px monospace'
      ctx.fillText(`${(gridSize / 1e3).toFixed(0)}K grid`, node.x + nodeW / 2, node.y + 68)
    } else if (node.level === 'macro') {
      const meshSize = config.target_mesh_size
      ctx.fillStyle = 'rgba(255, 255, 255, 0.4)'
      ctx.font = '9px monospace'
      ctx.fillText(`${meshSize} nm mesh`, node.x + nodeW / 2, node.y + 68)
    } else {
      ctx.fillStyle = 'rgba(255, 255, 255, 0.4)'
      ctx.font = '9px monospace'
      ctx.fillText('DFT', node.x + nodeW / 2, node.y + 68)
    }
  }

  // Title
  ctx.fillStyle = '#E8E9EB'
  ctx.font = '13px sans-serif'
  ctx.textAlign = 'center'
  ctx.fillText('多尺度数据流可视化', w / 2, 24)

  // Method indicator
  const methodLabel = bridgeMethods.find(m => m.value === config.method)?.label ?? config.method
  ctx.fillStyle = 'var(--primary)'
  ctx.font = '11px sans-serif'
  ctx.fillText(`当前方法: ${methodLabel}`, w / 2, h - 16)

  // Draw cost comparison mini bar chart at bottom
  if (results.value) {
    const cost = results.value.computational_cost_reduction
    const barY = h - 60
    const barH = 20
    const barMaxW = 200
    const barX = w - barMaxW - 40

    ctx.fillStyle = 'rgba(255, 255, 255, 0.4)'
    ctx.font = '9px sans-serif'
    ctx.textAlign = 'right'
    ctx.fillText('全原子:', barX - 8, barY + 14)
    ctx.fillStyle = '#ef4444'
    roundRect(ctx, barX, barY, barMaxW, barH, 3)
    ctx.fill()

    ctx.fillStyle = 'rgba(255, 255, 255, 0.4)'
    ctx.textAlign = 'right'
    ctx.fillText('多尺度:', barX - 8, barY + barH + 14)
    ctx.fillStyle = '#22c55e'
    const msWidth = Math.max(4, (cost.multiscale_time / cost.full_atomistic_time) * barMaxW)
    roundRect(ctx, barX, barY + barH + 4, msWidth, barH, 3)
    ctx.fill()
  }
}

function roundRect(ctx: CanvasRenderingContext2D, x: number, y: number, w: number, h: number, r: number) {
  ctx.beginPath()
  ctx.moveTo(x + r, y)
  ctx.lineTo(x + w - r, y)
  ctx.arcTo(x + w, y, x + w, y + r, r)
  ctx.lineTo(x + w, y + h - r)
  ctx.arcTo(x + w, y + h, x + w - r, y + h, r)
  ctx.lineTo(x + r, y + h)
  ctx.arcTo(x, y + h, x, y + h - r, r)
  ctx.lineTo(x, y + r)
  ctx.arcTo(x, y, x + r, y, r)
  ctx.closePath()
}

async function runBridge() {
  running.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 2000))
    results.value = generateMockResults()
    await nextTick()
    drawDataFlow()
  } catch (e) {
    console.error('Multiscale bridge failed:', e)
  } finally {
    running.value = false
  }
}

async function runCoarseGraining() {
  coarseGraining.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 1500))
    coarseResult.value = generateMockCoarseResult()
    if (!results.value) {
      results.value = generateMockResults()
    }
    await nextTick()
    drawDataFlow()
  } catch (e) {
    console.error('Coarse graining failed:', e)
  } finally {
    coarseGraining.value = false
  }
}

function exportResults() {
  if (!results.value) return
  const data = JSON.stringify(results.value, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `multiscale_bridge_${config.method}.json`
  a.click()
  URL.revokeObjectURL(url)
}

// ============ Lifecycle ============

onMounted(() => {
  config.source_data = atomisticData
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
