<template>
  <div class="xfem-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">断裂力学分析 (XFEM)</h2>
        <p class="text-xs text-[var(--text-muted)]">扩展有限元：裂纹扩展追踪，J积分/K因子计算，无需网格重剖</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- 模板选择 -->
        <select v-model="selectedTemplate" @change="loadTemplate" class="input w-auto text-xs">
          <option value="">选择模板</option>
          <option v-for="t in templates" :key="t.id" :value="t.id">{{ t.name }}</option>
        </select>
        <!-- 重置 -->
        <button @click="resetAll" class="btn btn-ghost text-xs">
          <span class="mr-1">&#x27F3;</span> 重置
        </button>
        <!-- 导出 -->
        <button v-if="results" @click="exportResults" class="btn btn-ghost text-xs">
          <span class="mr-1">&#x2B07;</span> 导出
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration (w-80) -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col overflow-y-auto">
        <!-- Step 1: 分析类型 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">1. 分析类型</h3>
          </div>
          <div class="flex gap-2 flex-wrap">
            <button
              v-for="type in analysisTypes"
              :key="type.value"
              @click="config.analysis_type = type.value"
              :class="['px-3 py-1.5 rounded text-xs transition', config.analysis_type === type.value ? 'bg-[var(--primary)] text-white' : 'bg-[var(--bg-base)] text-[var(--text-secondary)]']"
            >
              {{ type.label }}
            </button>
          </div>
        </div>

        <!-- Step 2: 材料属性 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">2. 材料属性</h3>
          </div>

          <!-- 材料库选择 -->
          <div class="mb-3">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">材料库</label>
            <select v-model="materialPreset" @change="applyMaterialPreset" class="input w-full text-xs">
              <option value="steel">结构钢 (Q235)</option>
              <option value="aluminum">铝合金 (7075-T6)</option>
              <option value="titanium">钛合金 (Ti-6Al-4V)</option>
              <option value="ceramic">陶瓷 (Al2O3)</option>
              <option value="custom">自定义</option>
            </select>
          </div>

          <!-- 弹性模量 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">弹性模量 E (GPa)</label>
            <input v-model.number="config.material.E" type="number" step="1" class="input w-full text-xs" />
          </div>

          <!-- 泊松比 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">泊松比 nu</label>
            <input v-model.number="config.material.nu" type="number" step="0.01" class="input w-full text-xs" />
          </div>

          <!-- 断裂韧性 KIc -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">断裂韧性 KIc (MPa&radic;m)</label>
            <input v-model.number="config.material.fracture_toughness" type="number" step="0.1" class="input w-full text-xs" />
          </div>

          <!-- 屈服强度 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">屈服强度 (MPa)</label>
            <input v-model.number="config.material.yield_strength" type="number" step="10" class="input w-full text-xs" />
          </div>

          <!-- 抗拉强度 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">抗拉强度 (MPa)</label>
            <input v-model.number="config.material.tensile_strength" type="number" step="10" class="input w-full text-xs" />
          </div>
        </div>

        <!-- Step 3: 裂纹定义 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">3. 裂纹定义</h3>
          </div>

          <!-- 裂纹类型 -->
          <div class="mb-3">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">裂纹类型</label>
            <select v-model="config.crack.type" class="input w-full text-xs">
              <option value="edge">边缘裂纹</option>
              <option value="center">中心裂纹</option>
              <option value="corner">角裂纹</option>
            </select>
          </div>

          <!-- 初始裂纹长度 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">初始裂纹长度 a (mm)</label>
            <input v-model.number="config.crack.initial_length" type="number" step="0.1" class="input w-full text-xs" />
          </div>

          <!-- 方向角 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">方向角 (deg)</label>
            <input v-model.number="config.crack.orientation_angle" type="number" step="1" class="input w-full text-xs" />
          </div>

          <!-- 裂纹位置 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">裂纹位置</label>
            <div class="grid grid-cols-3 gap-1">
              <div>
                <span class="text-[10px] text-[var(--text-muted)]">X (mm)</span>
                <input v-model.number="config.crack.position.x" type="number" step="0.1" class="input w-full text-xs" />
              </div>
              <div>
                <span class="text-[10px] text-[var(--text-muted)]">Y (mm)</span>
                <input v-model.number="config.crack.position.y" type="number" step="0.1" class="input w-full text-xs" />
              </div>
              <div>
                <span class="text-[10px] text-[var(--text-muted)]">Z (mm)</span>
                <input v-model.number="config.crack.position.z" type="number" step="0.1" class="input w-full text-xs" />
              </div>
            </div>
          </div>

          <!-- 富集类型 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">富集类型</label>
            <select v-model="config.crack.enrichment_type" class="input w-full text-xs">
              <option value="standard">标准 XFEM</option>
              <option value="phantom_node">Phantom Node</option>
              <option value="vietnam">Vietnam (修正)</option>
            </select>
          </div>
        </div>

        <!-- Step 4: 求解设置 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">4. 求解设置</h3>
          </div>

          <!-- 网格尺寸 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">网格尺寸 (mm)</label>
            <input v-model.number="config.mesh_size" type="number" step="0.1" class="input w-full text-xs" />
          </div>

          <!-- 最大增量步 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">最大增量步</label>
            <input v-model.number="config.max_increments" type="number" step="10" class="input w-full text-xs" />
          </div>

          <!-- 富集半径 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">富集半径 (mm)</label>
            <input v-model.number="config.enrichment_radius" type="number" step="0.5" class="input w-full text-xs" />
          </div>

          <!-- 积分类型 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">积分类型</label>
            <select v-model="config.domain_integral_type" class="input w-full text-xs">
              <option value="interaction">相互作用积分</option>
              <option value="j_integral">J积分</option>
              <option value="cstar">C* 积分</option>
            </select>
          </div>

          <!-- 扩展判据 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">裂纹扩展判据</label>
            <select v-model="config.growth_criterion" class="input w-full text-xs">
              <option value="max_circumferential_stress">最大周向应力</option>
              <option value="max_energy_release_rate">最大能量释放率</option>
              <option value="wilson">Wilson 判据</option>
            </select>
          </div>
        </div>

        <!-- 快捷模板按钮 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <label class="text-xs text-[var(--text-secondary)] block mb-2">快捷模板</label>
          <div class="flex gap-2 flex-wrap">
            <button @click="applyQuickTemplate('ct')" class="btn btn-outline text-xs px-2 py-1">中心裂纹板 (CT)</button>
            <button @click="applyQuickTemplate('senb')" class="btn btn-outline text-xs px-2 py-1">单边缺口梁 (SENB)</button>
            <button @click="applyQuickTemplate('ctod')" class="btn btn-outline text-xs px-2 py-1">紧凑拉伸 (CTOD)</button>
          </div>
        </div>

        <!-- 运行分析按钮 -->
        <div class="px-4 py-3 mt-auto border-t border-[var(--border-subtle)]">
          <button
            @click="runAnalysis"
            :disabled="analyzing"
            :class="['btn w-full text-sm', analyzing ? 'btn-loading' : 'btn-primary']"
          >
            <span v-if="analyzing" class="mr-2 animate-spin">&#x27F3;</span>
            {{ analyzing ? '分析中...' : '&#x25B6; 运行断裂分析' }}
          </button>
        </div>
      </div>

      <!-- Right: Visualization & Results -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- Results Tabs -->
        <div v-if="results" class="flex items-center gap-2 px-4 py-2 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
          <button
            v-for="tab in resultTabs"
            :key="tab.value"
            @click="resultViewTab = tab.value"
            :class="['px-3 py-1 text-xs rounded transition',
              resultViewTab === tab.value
                ? 'bg-[var(--primary)] text-white'
                : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)]']"
          >
            {{ tab.label }}
          </button>
        </div>

        <!-- Visualization Area -->
        <div class="flex-1 flex">
          <div class="flex-1 relative">
            <!-- 裂纹扩展路径 Canvas -->
            <div v-if="results && resultViewTab === 'crack_path'" class="w-full h-full">
              <canvas ref="crackCanvas" class="w-full h-full"></canvas>
            </div>
            <!-- 应力场云图 Canvas -->
            <div v-if="results && resultViewTab === 'stress_field'" class="w-full h-full">
              <canvas ref="stressCanvas" class="w-full h-full"></canvas>
            </div>
            <!-- K因子/J积分结果面板 -->
            <div v-if="results && resultViewTab === 'kfactors'" class="w-full h-full overflow-y-auto p-4">
              <div class="grid grid-cols-2 gap-4">
                <!-- KI -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">KI (I型)</div>
                  <div class="text-xl font-bold" :style="{ color: results.stress_intensity_factor.KI > config.material.fracture_toughness ? 'var(--accent-red)' : 'var(--primary)' }">
                    {{ results.stress_intensity_factor.KI.toFixed(2) }} MPa&radic;m
                  </div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-1">
                    KIc = {{ config.material.fracture_toughness }} MPa&radic;m
                  </div>
                </div>
                <!-- KII -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">KII (II型)</div>
                  <div class="text-xl font-bold text-[var(--text-primary)]">
                    {{ results.stress_intensity_factor.KII.toFixed(2) }} MPa&radic;m
                  </div>
                </div>
                <!-- KIII -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">KIII (III型)</div>
                  <div class="text-xl font-bold text-[var(--text-primary)]">
                    {{ results.stress_intensity_factor.KIII.toFixed(2) }} MPa&radic;m
                  </div>
                </div>
                <!-- 等效 K -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">等效 Keff</div>
                  <div class="text-xl font-bold" :style="{ color: effectiveK > config.material.fracture_toughness ? 'var(--accent-red)' : 'var(--accent-green)' }">
                    {{ effectiveK.toFixed(2) }} MPa&radic;m
                  </div>
                  <div class="text-[10px] mt-1" :style="{ color: effectiveK > config.material.fracture_toughness ? 'var(--accent-red)' : 'var(--accent-green)' }">
                    {{ effectiveK > config.material.fracture_toughness ? '裂纹失稳' : '裂纹稳定' }}
                  </div>
                </div>
                <!-- J积分 -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">J积分</div>
                  <div class="text-xl font-bold text-[var(--primary)]">
                    {{ results.j_integral.toFixed(2) }} N/m
                  </div>
                </div>
                <!-- 能量释放率 G -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">能量释放率 G</div>
                  <div class="text-xl font-bold text-[var(--text-primary)]">
                    {{ results.energy_release_rate.toFixed(4) }} N/m
                  </div>
                </div>
                <!-- 裂纹长度 -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">裂纹长度</div>
                  <div class="text-xl font-bold text-[var(--text-primary)]">
                    {{ (results.crack_length * 1000).toFixed(2) }} mm
                  </div>
                </div>
                <!-- 裂尖位置 -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">裂尖位置</div>
                  <div class="text-sm font-semibold text-[var(--text-primary)]">
                    ({{ results.crack_tip_position.x.toFixed(2) }},
                    {{ results.crack_tip_position.y.toFixed(2) }},
                    {{ results.crack_tip_position.z.toFixed(2) }}) mm
                  </div>
                </div>
              </div>

              <!-- 收敛信息 -->
              <div class="mt-4 p-3 rounded-lg" :class="results.converged ? 'bg-green-50 border border-green-200' : 'bg-red-50 border border-red-200'">
                <div class="flex items-center gap-2">
                  <span>{{ results.converged ? '&#x2705;' : '&#x26A0;&#xFE0F;' }}</span>
                  <span class="text-xs font-medium" :class="results.converged ? 'text-green-700' : 'text-red-700'">
                    {{ results.converged ? '分析收敛' : '分析未收敛' }}
                  </span>
                  <span class="text-xs text-[var(--text-muted)]">
                    | 增量步: {{ results.num_steps }}
                  </span>
                </div>
              </div>
            </div>
            <!-- 裂纹路径坐标表 -->
            <div v-if="results && resultViewTab === 'path_table'" class="w-full h-full overflow-y-auto p-4">
              <h4 class="text-sm font-semibold text-[var(--text-primary)] mb-3">裂纹路径坐标</h4>
              <table class="w-full text-xs">
                <thead>
                  <tr class="border-b border-[var(--border-subtle)]">
                    <th class="text-left py-2 text-[var(--text-muted)]">Step</th>
                    <th class="text-left py-2 text-[var(--text-muted)]">X (mm)</th>
                    <th class="text-left py-2 text-[var(--text-muted)]">Y (mm)</th>
                    <th class="text-left py-2 text-[var(--text-muted)]">Z (mm)</th>
                  </tr>
                </thead>
                <tbody>
                  <tr
                    v-for="(pt, idx) in results.crack_path"
                    :key="idx"
                    class="border-b border-[var(--border-subtle)] hover:bg-[var(--bg-elevated)]"
                  >
                    <td class="py-1.5">{{ pt.step }}</td>
                    <td class="py-1.5">{{ pt.x.toFixed(3) }}</td>
                    <td class="py-1.5">{{ pt.y.toFixed(3) }}</td>
                    <td class="py-1.5">{{ pt.z.toFixed(3) }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
            <!-- 空状态 -->
            <div v-if="!results" class="absolute inset-0 flex items-center justify-center">
              <div class="text-center text-[var(--text-muted)]">
                <div class="text-4xl mb-2">&#x1F4CF;</div>
                <p class="text-sm">配置参数后运行断裂力学分析</p>
              </div>
            </div>
          </div>

          <!-- Color Legend (应力场模式) -->
          <div v-if="results && resultViewTab === 'stress_field'" class="w-16 bg-[var(--bg-surface)] border-l border-[var(--border-subtle)] p-2">
            <div class="text-[10px] text-[var(--text-muted)] mb-1">应力 (MPa)</div>
            <div class="h-64 w-4 mx-auto" :style="{ background: stressLegendGradient }"></div>
            <div class="flex flex-col justify-between h-64 text-[10px] text-[var(--text-muted)] mt-1">
              <span>{{ stressLegendMax.toFixed(0) }}</span>
              <span>{{ (stressLegendMax / 2).toFixed(0) }}</span>
              <span>0</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type { XfemConfig, XfemResult, XfemTemplate } from '@/api/xfem'

// ============ 分析类型选项 ============
const analysisTypes: { value: 'static' | 'fatigue_crack_growth' | 'dynamic'; label: string }[] = [
  { value: 'static', label: '静态断裂' },
  { value: 'fatigue_crack_growth', label: '疲劳裂纹扩展' },
  { value: 'dynamic', label: '动态断裂' },
]

// ============ 结果视图标签 ============
const resultTabs = [
  { value: 'crack_path', label: '裂纹扩展路径' },
  { value: 'stress_field', label: '裂尖应力场' },
  { value: 'kfactors', label: 'K因子/J积分' },
  { value: 'path_table', label: '路径坐标表' },
]

// ============ 状态 ============
const analyzing = ref(false)
const selectedTemplate = ref('')
const templates = ref<XfemTemplate[]>([])
const resultViewTab = ref('crack_path')
const results = ref<XfemResult | null>(null)
const materialPreset = ref('steel')

// Canvas refs
const crackCanvas = ref<HTMLCanvasElement | null>(null)
const stressCanvas = ref<HTMLCanvasElement | null>(null)

// ============ 分析配置 ============
const config = reactive<XfemConfig>({
  project_id: 'default',
  analysis_type: 'static',
  material: {
    name: '结构钢',
    E: 200,          // GPa (UI显示用)
    nu: 0.3,
    density: 7850,   // kg/m^3
    yield_strength: 235,   // MPa
    fracture_toughness: 50, // MPa*sqrt(m)
    tensile_strength: 400,  // MPa
  },
  crack: {
    id: 'crack_1',
    type: 'edge',
    initial_length: 10,     // mm (UI显示用)
    orientation_angle: 0,   // deg
    position: { x: 0, y: 0, z: 0 },
    propagation_direction: 'auto',
    enrichment_type: 'standard',
  },
  mesh_size: 1.0,           // mm
  max_increments: 100,
  enrichment_radius: 5.0,   // mm
  domain_integral_type: 'interaction',
  growth_criterion: 'max_circumferential_stress',
})

// ============ 材料预设数据 ============
const materialPresets: Record<string, Partial<XfemConfig['material']>> = {
  steel: {
    name: '结构钢 (Q235)',
    E: 200,
    nu: 0.3,
    density: 7850,
    yield_strength: 235,
    fracture_toughness: 50,
    tensile_strength: 400,
  },
  aluminum: {
    name: '铝合金 (7075-T6)',
    E: 71,
    nu: 0.33,
    density: 2810,
    yield_strength: 503,
    fracture_toughness: 29,
    tensile_strength: 572,
  },
  titanium: {
    name: '钛合金 (Ti-6Al-4V)',
    E: 114,
    nu: 0.34,
    density: 4430,
    yield_strength: 880,
    fracture_toughness: 66,
    tensile_strength: 950,
  },
  ceramic: {
    name: '陶瓷 (Al2O3)',
    E: 370,
    nu: 0.22,
    density: 3900,
    yield_strength: 300,
    fracture_toughness: 4,
    tensile_strength: 300,
  },
}

// ============ 模板数据 ============
const templateData: XfemTemplate[] = [
  {
    id: 'ct',
    name: '中心裂纹板 (CT)',
    name_en: 'Center Crack Tension',
    category: 'fracture',
    description: 'ASTM E399 标准中心裂纹拉伸试样',
    crack_type: 'center',
    default_material: { name: '结构钢', E: 200, fracture_toughness: 50 },
    typical_KIc_range: [30, 100],
  },
  {
    id: 'senb',
    name: '单边缺口梁 (SENB)',
    name_en: 'Single Edge Notch Beam',
    category: 'fracture',
    description: 'ASTM D5045 标准单边缺口弯曲试样',
    crack_type: 'edge',
    default_material: { name: '结构钢', E: 200, fracture_toughness: 50 },
    typical_KIc_range: [20, 80],
  },
  {
    id: 'ctod',
    name: '紧凑拉伸 (CTOD)',
    name_en: 'Crack Tip Opening Displacement',
    category: 'fracture',
    description: 'BS 7448 标准裂纹尖端张开位移试样',
    crack_type: 'edge',
    default_material: { name: '结构钢', E: 200, fracture_toughness: 50 },
    typical_KIc_range: [40, 120],
  },
]

// ============ 计算属性 ============
const effectiveK = computed(() => {
  if (!results.value) return 0
  const { KI, KII, KIII } = results.value.stress_intensity_factor
  return Math.sqrt(KI * KI + KII * KII + KIII * KIII)
})

const stressLegendMax = computed(() => {
  if (!results.value || results.value.stress_field.length === 0) return 500
  let maxVal = 0
  for (const pt of results.value.stress_field) {
    maxVal = Math.max(maxVal, Math.abs(pt.sigma_xx), Math.abs(pt.sigma_yy), Math.abs(pt.tau_xy))
  }
  return maxVal
})

const stressLegendGradient = computed(() => {
  return 'linear-gradient(to bottom, #EF4444, #F59E0B, #22C55E, #3B82F6, #6366F1)'
})

// ============ 方法 ============

/** 应用材料预设 */
function applyMaterialPreset() {
  const preset = materialPresets[materialPreset.value]
  if (preset) {
    Object.assign(config.material, preset)
  }
}

/** 加载模板 */
function loadTemplate() {
  if (!selectedTemplate.value) return
  const tmpl = templateData.find(t => t.id === selectedTemplate.value)
  if (!tmpl) return

  config.crack.type = tmpl.crack_type
  if (tmpl.default_material) {
    if (tmpl.default_material.E !== undefined) config.material.E = tmpl.default_material.E
    if (tmpl.default_material.fracture_toughness !== undefined) {
      config.material.fracture_toughness = tmpl.default_material.fracture_toughness
    }
    if (tmpl.default_material.name) config.material.name = tmpl.default_material.name
  }

  // 根据模板设置典型参数
  switch (tmpl.id) {
    case 'ct':
      config.crack.type = 'center'
      config.crack.initial_length = 15
      config.crack.orientation_angle = 90
      config.mesh_size = 0.8
      config.enrichment_radius = 6
      break
    case 'senb':
      config.crack.type = 'edge'
      config.crack.initial_length = 8
      config.crack.orientation_angle = 0
      config.mesh_size = 0.5
      config.enrichment_radius = 4
      break
    case 'ctod':
      config.crack.type = 'edge'
      config.crack.initial_length = 12
      config.crack.orientation_angle = 0
      config.mesh_size = 0.6
      config.enrichment_radius = 5
      break
  }
}

/** 快捷模板 */
function applyQuickTemplate(type: 'ct' | 'senb' | 'ctod') {
  selectedTemplate.value = type
  loadTemplate()
}

/** 重置 */
function resetAll() {
  results.value = null
  selectedTemplate.value = ''
  materialPreset.value = 'steel'
  config.analysis_type = 'static'
  applyMaterialPreset()
  config.crack = {
    id: 'crack_1',
    type: 'edge',
    initial_length: 10,
    orientation_angle: 0,
    position: { x: 0, y: 0, z: 0 },
    propagation_direction: 'auto',
    enrichment_type: 'standard',
  }
  config.mesh_size = 1.0
  config.max_increments = 100
  config.enrichment_radius = 5.0
  config.domain_integral_type = 'interaction'
  config.growth_criterion = 'max_circumferential_stress'
}

/** 生成模拟结果 (参考 ASTM 标准值) */
function generateMockResults(): XfemResult {
  const a = config.crack.initial_length / 1000  // mm -> m
  const KIc = config.material.fracture_toughness
  const E = config.material.E * 1e9  // GPa -> Pa

  // 基于 ASTM E399 公式模拟 KI 值
  // KI = sigma * sqrt(pi * a) * F(a/W)
  const sigma = config.material.yield_strength * 0.4  // 施加40%屈服应力
  const F_factor = 1.12 - 0.231 * (a / 0.05) + 10.55 * Math.pow(a / 0.05, 2) - 21.72 * Math.pow(a / 0.05, 3) + 30.39 * Math.pow(a / 0.05, 4)
  const KI = sigma * Math.sqrt(Math.PI * a) * Math.max(F_factor, 1.0)
  const KII = KI * 0.15 * (Math.random() * 0.5 + 0.5)
  const KIII = KI * 0.05 * (Math.random() * 0.5 + 0.5)

  // J积分: J = KI^2 / E (平面应力)
  const j_integral = (KI * KI) / E * 1000  // N/m

  // 能量释放率 G = J (线弹性)
  const energy_release_rate = j_integral

  // 裂纹路径生成
  const numSteps = config.analysis_type === 'fatigue_crack_growth' ? 20 : 10
  const crackPath: Array<{ x: number; y: number; z: number; step: number }> = []
  let cx = config.crack.position.x
  let cy = config.crack.position.y
  const angleRad = (config.crack.orientation_angle * Math.PI) / 180

  for (let i = 0; i <= numSteps; i++) {
    const stepGrowth = (config.crack.initial_length / numSteps) * (1 + 0.1 * Math.sin(i * 0.5))
    cx += stepGrowth * Math.cos(angleRad) + (Math.random() - 0.5) * 0.2
    cy += stepGrowth * Math.sin(angleRad) + (Math.random() - 0.5) * 0.2
    crackPath.push({
      x: parseFloat(cx.toFixed(3)),
      y: parseFloat(cy.toFixed(3)),
      z: config.crack.position.z,
      step: i,
    })
  }

  // 应力场生成 (裂尖奇异场)
  const stressField: Array<{
    x: number; y: number; z: number
    sigma_xx: number; sigma_yy: number; tau_xy: number
  }> = []
  const tipX = crackPath[crackPath.length - 1].x
  const tipY = crackPath[crackPath.length - 1].y
  const gridSize = 15

  for (let i = -gridSize; i <= gridSize; i++) {
    for (let j = -gridSize; j <= gridSize; j++) {
      const px = tipX + i * config.mesh_size * 0.5
      const py = tipY + j * config.mesh_size * 0.5
      const r = Math.sqrt((px - tipX) ** 2 + (py - tipY) ** 2)
      if (r < 0.01) continue

      const theta = Math.atan2(py - tipY, px - tipX)
      const sqrtR = Math.sqrt(r * 1000) // mm

      // Williams 展开 (I型主导)
      const factor = KI / Math.sqrt(2 * Math.PI * r * 1000)
      const cosHalf = Math.cos(theta / 2)
      const sinHalf = Math.sin(theta / 2)

      const sigma_xx = factor * cosHalf * (1 - sinHalf * Math.sin(1.5 * theta))
      const sigma_yy = factor * cosHalf * (1 + sinHalf * Math.sin(1.5 * theta))
      const tau_xy = factor * sinHalf * cosHalf * Math.cos(1.5 * theta)

      stressField.push({
        x: parseFloat(px.toFixed(3)),
        y: parseFloat(py.toFixed(3)),
        z: config.crack.position.z,
        sigma_xx: parseFloat(sigma_xx.toFixed(2)),
        sigma_yy: parseFloat(sigma_yy.toFixed(2)),
        tau_xy: parseFloat(tau_xy.toFixed(2)),
      })
    }
  }

  const finalCrackLength = Math.sqrt(
    (crackPath[crackPath.length - 1].x - crackPath[0].x) ** 2 +
    (crackPath[crackPath.length - 1].y - crackPath[0].y) ** 2
  )

  return {
    success: true,
    stress_intensity_factor: {
      KI: parseFloat(KI.toFixed(2)),
      KII: parseFloat(KII.toFixed(2)),
      KIII: parseFloat(KIII.toFixed(2)),
    },
    j_integral: parseFloat(j_integral.toFixed(4)),
    energy_release_rate: parseFloat(energy_release_rate.toFixed(4)),
    crack_tip_position: {
      x: crackPath[crackPath.length - 1].x,
      y: crackPath[crackPath.length - 1].y,
      z: config.crack.position.z,
    },
    crack_length: finalCrackLength,
    crack_path: crackPath,
    stress_field: stressField,
    num_steps: numSteps,
    converged: true,
  }
}

/** 运行分析 */
async function runAnalysis() {
  analyzing.value = true
  try {
    // 尝试调用后端 API
    // const result = await runXfemAnalysis(config)
    // results.value = result

    // 使用模拟数据
    await new Promise(resolve => setTimeout(resolve, 1500))
    results.value = generateMockResults()

    await nextTick()
    drawCrackPath()
    drawStressField()
  } catch (e) {
    console.error('XFEM analysis failed:', e)
    // 降级为模拟数据
    results.value = generateMockResults()
    await nextTick()
    drawCrackPath()
    drawStressField()
  } finally {
    analyzing.value = false
  }
}

/** 绘制裂纹扩展路径 */
function drawCrackPath() {
  if (!crackCanvas.value || !results.value) return

  const container = crackCanvas.value.parentElement
  if (!container) return

  crackCanvas.value.width = container.clientWidth
  crackCanvas.value.height = container.clientHeight

  const ctx = crackCanvas.value.getContext('2d')
  if (!ctx) return

  const w = crackCanvas.value.width
  const h = crackCanvas.value.height
  const padding = 60

  ctx.clearRect(0, 0, w, h)

  // 背景
  ctx.fillStyle = '#0A0B0E'
  ctx.fillRect(0, 0, w, h)

  // 网格
  ctx.strokeStyle = '#1C1D24'
  ctx.lineWidth = 0.5
  for (let x = padding; x < w - padding; x += 30) {
    ctx.beginPath()
    ctx.moveTo(x, padding)
    ctx.lineTo(x, h - padding)
    ctx.stroke()
  }
  for (let y = padding; y < h - padding; y += 30) {
    ctx.beginPath()
    ctx.moveTo(padding, y)
    ctx.lineTo(w - padding, y)
    ctx.stroke()
  }

  const path = results.value.crack_path
  if (path.length === 0) return

  // 计算边界
  let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity
  for (const pt of path) {
    minX = Math.min(minX, pt.x)
    maxX = Math.max(maxX, pt.x)
    minY = Math.min(minY, pt.y)
    maxY = Math.max(maxY, pt.y)
  }

  const rangeX = maxX - minX || 1
  const rangeY = maxY - minY || 1
  const scaleX = (w - 2 * padding) / rangeX
  const scaleY = (h - 2 * padding) / rangeY
  const scale = Math.min(scaleX, scaleY) * 0.8

  const offsetX = (w - rangeX * scale) / 2 - minX * scale
  const offsetY = (h - rangeY * scale) / 2 - minY * scale

  const toScreen = (x: number, y: number) => ({
    sx: x * scale + offsetX,
    sy: h - (y * scale + offsetY),
  })

  // 绘制裂纹体 (矩形示意)
  ctx.strokeStyle = '#374151'
  ctx.lineWidth = 1
  ctx.setLineDash([5, 5])
  const bodyPad = 20
  const bx1 = minX * scale + offsetX - bodyPad
  const by1 = minY * scale + offsetY - bodyPad
  const bx2 = maxX * scale + offsetX + bodyPad
  const by2 = maxY * scale + offsetY + bodyPad
  ctx.strokeRect(bx1, h - by2, bx2 - bx1, by2 - by1)
  ctx.setLineDash([])

  // 绘制裂纹路径
  ctx.beginPath()
  ctx.strokeStyle = '#EF4444'
  ctx.lineWidth = 3
  for (let i = 0; i < path.length; i++) {
    const { sx, sy } = toScreen(path[i].x, path[i].y)
    if (i === 0) ctx.moveTo(sx, sy)
    else ctx.lineTo(sx, sy)
  }
  ctx.stroke()

  // 绘制路径节点
  for (let i = 0; i < path.length; i++) {
    const { sx, sy } = toScreen(path[i].x, path[i].y)
    ctx.beginPath()
    ctx.arc(sx, sy, i === 0 || i === path.length - 1 ? 5 : 2, 0, Math.PI * 2)
    ctx.fillStyle = i === path.length - 1 ? '#F59E0B' : '#EF4444'
    ctx.fill()
  }

  // 裂尖标注
  const tip = path[path.length - 1]
  const tipScreen = toScreen(tip.x, tip.y)
  ctx.beginPath()
  ctx.arc(tipScreen.sx, tipScreen.sy, 8, 0, Math.PI * 2)
  ctx.strokeStyle = '#F59E0B'
  ctx.lineWidth = 2
  ctx.stroke()

  ctx.fillStyle = '#F59E0B'
  ctx.font = '11px sans-serif'
  ctx.fillText(`裂尖 (${tip.x.toFixed(1)}, ${tip.y.toFixed(1)})`, tipScreen.sx + 12, tipScreen.sy - 8)

  // 起始点标注
  const start = path[0]
  const startScreen = toScreen(start.x, start.y)
  ctx.fillStyle = '#22C55E'
  ctx.font = '11px sans-serif'
  ctx.fillText(`起点 (${start.x.toFixed(1)}, ${start.y.toFixed(1)})`, startScreen.sx + 12, startScreen.sy + 16)

  // 图例
  ctx.fillStyle = '#9CA3AF'
  ctx.font = '10px sans-serif'
  ctx.fillText(`裂纹类型: ${config.crack.type === 'edge' ? '边缘裂纹' : config.crack.type === 'center' ? '中心裂纹' : '角裂纹'}`, padding, h - 20)
  ctx.fillText(`扩展步数: ${path.length - 1}`, padding + 200, h - 20)

  // 坐标轴
  ctx.strokeStyle = '#4B5563'
  ctx.lineWidth = 1
  ctx.beginPath()
  ctx.moveTo(padding, padding)
  ctx.lineTo(padding, h - padding)
  ctx.lineTo(w - padding, h - padding)
  ctx.stroke()

  ctx.fillStyle = '#9CA3AF'
  ctx.font = '10px sans-serif'
  ctx.fillText('X (mm)', w / 2, h - 5)
  ctx.save()
  ctx.translate(10, h / 2)
  ctx.rotate(-Math.PI / 2)
  ctx.fillText('Y (mm)', 0, 0)
  ctx.restore()
}

/** 绘制裂尖应力场云图 */
function drawStressField() {
  if (!stressCanvas.value || !results.value) return

  const container = stressCanvas.value.parentElement
  if (!container) return

  stressCanvas.value.width = container.clientWidth
  stressCanvas.value.height = container.clientHeight

  const ctx = stressCanvas.value.getContext('2d')
  if (!ctx) return

  const w = stressCanvas.value.width
  const h = stressCanvas.value.height
  const padding = 60

  ctx.clearRect(0, 0, w, h)
  ctx.fillStyle = '#0A0B0E'
  ctx.fillRect(0, 0, w, h)

  const field = results.value.stress_field
  if (field.length === 0) return

  // 计算边界
  let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity
  let maxStress = 0
  for (const pt of field) {
    minX = Math.min(minX, pt.x)
    maxX = Math.max(maxX, pt.x)
    minY = Math.min(minY, pt.y)
    maxY = Math.max(maxY, pt.y)
    maxStress = Math.max(maxStress, Math.abs(pt.sigma_yy))
  }
  maxStress = maxStress || 1

  const rangeX = maxX - minX || 1
  const rangeY = maxY - minY || 1
  const scaleX = (w - 2 * padding) / rangeX
  const scaleY = (h - 2 * padding) / rangeY
  const scale = Math.min(scaleX, scaleY) * 0.9

  const offsetX = (w - rangeX * scale) / 2 - minX * scale
  const offsetY = (h - rangeY * scale) / 2 - minY * scale

  // 绘制应力场 (sigma_yy)
  const cellSize = Math.max(scale * config.mesh_size * 0.5, 2)
  for (const pt of field) {
    const sx = pt.x * scale + offsetX
    const sy = h - (pt.y * scale + offsetY)
    const ratio = Math.min(Math.abs(pt.sigma_yy) / maxStress, 1)

    ctx.fillStyle = stressColorMap(ratio)
    ctx.fillRect(sx - cellSize / 2, sy - cellSize / 2, cellSize, cellSize)
  }

  // 裂纹路径叠加
  const path = results.value.crack_path
  if (path.length > 1) {
    ctx.beginPath()
    ctx.strokeStyle = '#000000'
    ctx.lineWidth = 2
    for (let i = 0; i < path.length; i++) {
      const sx = path[i].x * scale + offsetX
      const sy = h - (path[i].y * scale + offsetY)
      if (i === 0) ctx.moveTo(sx, sy)
      else ctx.lineTo(sx, sy)
    }
    ctx.stroke()
  }

  // 裂尖标记
  const tip = results.value.crack_tip_position
  const tipSx = tip.x * scale + offsetX
  const tipSy = h - (tip.y * scale + offsetY)
  ctx.beginPath()
  ctx.arc(tipSx, tipSy, 6, 0, Math.PI * 2)
  ctx.strokeStyle = '#FFFFFF'
  ctx.lineWidth = 2
  ctx.stroke()

  // 坐标轴
  ctx.strokeStyle = '#4B5563'
  ctx.lineWidth = 1
  ctx.beginPath()
  ctx.moveTo(padding, padding)
  ctx.lineTo(padding, h - padding)
  ctx.lineTo(w - padding, h - padding)
  ctx.stroke()

  ctx.fillStyle = '#9CA3AF'
  ctx.font = '10px sans-serif'
  ctx.fillText('X (mm)', w / 2, h - 5)
  ctx.save()
  ctx.translate(10, h / 2)
  ctx.rotate(-Math.PI / 2)
  ctx.fillText('Y (mm)', 0, 0)
  ctx.restore()

  // 标题
  ctx.fillStyle = '#E8E9EB'
  ctx.font = '12px sans-serif'
  ctx.fillText('sigma_yy 应力场 (MPa)', padding + 10, padding - 10)
}

/** 应力云图颜色映射 */
function stressColorMap(ratio: number): string {
  // 蓝 -> 青 -> 绿 -> 黄 -> 红
  if (ratio < 0.25) {
    const t = ratio / 0.25
    const r = Math.round(59 + (6 - 59) * t)
    const g = Math.round(130 + (182 - 130) * t)
    const b = Math.round(246 + (212 - 246) * t)
    return `rgb(${r},${g},${b})`
  } else if (ratio < 0.5) {
    const t = (ratio - 0.25) / 0.25
    const r = Math.round(6 + (34 - 6) * t)
    const g = Math.round(182 + (197 - 182) * t)
    const b = Math.round(212 + (94 - 212) * t)
    return `rgb(${r},${g},${b})`
  } else if (ratio < 0.75) {
    const t = (ratio - 0.5) / 0.25
    const r = Math.round(34 + (245 - 34) * t)
    const g = Math.round(197 + (158 - 197) * t)
    const b = Math.round(94 + (11 - 94) * t)
    return `rgb(${r},${g},${b})`
  } else {
    const t = (ratio - 0.75) / 0.25
    const r = Math.round(245 + (239 - 245) * t)
    const g = Math.round(158 + (68 - 158) * t)
    const b = Math.round(11 + (68 - 11) * t)
    return `rgb(${r},${g},${b})`
  }
}

/** 导出结果 */
function exportResults() {
  if (!results.value) return

  const data = JSON.stringify(results.value, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'xfem_results.json'
  a.click()
  URL.revokeObjectURL(url)
}

// ============ 生命周期 ============
onMounted(() => {
  templates.value = templateData
})
</script>

<style scoped>
.xfem-view {
  --bg-base: #0A0B0E;
  --bg-surface: #14151A;
  --bg-elevated: #1C1D24;
  --bg-hover: #25262E;
  --text-primary: #E8E9EB;
  --text-secondary: #9CA3AF;
  --text-muted: #6B7280;
  --primary: #4F8CFF;
  --accent-red: #EF4444;
  --accent-green: #22C55E;
  --accent-amber: #F59E0B;
  --border-subtle: #2D2E38;
}

.btn {
  @apply px-3 py-1.5 rounded text-xs font-medium transition;
}

.btn-primary {
  @apply bg-blue-600 text-white hover:bg-blue-700;
}

.btn-primary:disabled {
  @apply bg-blue-800 text-white cursor-not-allowed;
}

.btn-ghost {
  @apply text-gray-400 hover:text-white hover:bg-gray-700;
}

.btn-outline {
  @apply border border-gray-600 text-gray-300 hover:bg-gray-700;
}

.btn-loading {
  @apply bg-blue-800 text-white cursor-not-allowed;
}

.input {
  @apply px-2 py-1.5 bg-[var(--bg-base)] border border-[var(--border-subtle)] rounded text-xs text-[var(--text-primary)] focus:outline-none focus:border-[var(--primary)];
}

.stat-card {
  @apply p-3 rounded-lg bg-[var(--bg-base)];
}

.step-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--border-subtle);
}

.step-dot.active {
  background: var(--primary);
  box-shadow: 0 0 0 3px rgba(79, 140, 255, 0.2);
}

.step-dot.completed {
  background: var(--accent-green);
}
</style>
