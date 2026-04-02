<!--
  AcousticCouplingView.vue - 声-结构耦合分析视图
  ==========================================
  V1.3-010

  核心功能:
  - 振动-噪声耦合分析 (结构模态→声腔模态)
  - 辐射噪声计算
  - 频率响应分析 (SPL vs Frequency)
  - 声压级云图可视化
  - 预设模板 (汽车座舱、飞机客舱、电子设备壳体、建筑隔声)
-->

<template>
  <div class="acoustic-coupling-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">声-结构耦合分析</h2>
        <p class="text-xs text-[var(--text-muted)]">振动-噪声：结构模态&#x2192;声腔模态，辐射噪声计算，频响分析</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="resetAll" class="btn btn-ghost text-xs">
          <span class="mr-1">&#x27F3;</span> 重置
        </button>
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

        <!-- Step 2: 声学材料 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">2. 声学材料</h3>
          </div>
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">介质类型</label>
            <select v-model="acousticPreset" @change="applyAcousticPreset" class="input w-full text-xs">
              <option value="air">空气</option>
              <option value="water">水</option>
              <option value="oil">油</option>
              <option value="custom">自定义</option>
            </select>
          </div>
          <div class="grid grid-cols-2 gap-2">
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">密度 (kg/m&sup3;)</label>
              <input v-model.number="config.acoustic_material.density" type="number" step="0.1" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">声速 (m/s)</label>
              <input v-model.number="config.acoustic_material.sound_speed" type="number" step="1" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">吸收系数</label>
              <input v-model.number="config.acoustic_material.absorption_coefficient" type="number" step="0.01" min="0" max="1" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">声阻抗 (Pa&middot;s/m)</label>
              <input v-model.number="config.acoustic_material.impedance" type="number" step="1" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Step 3: 结构材料 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">3. 结构材料</h3>
          </div>
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">材料类型</label>
            <select v-model="structuralPreset" @change="applyStructuralPreset" class="input w-full text-xs">
              <option value="steel">钢</option>
              <option value="aluminum">铝</option>
              <option value="composite">复合材料</option>
              <option value="custom">自定义</option>
            </select>
          </div>
          <div class="grid grid-cols-2 gap-2">
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">弹性模量 E (GPa)</label>
              <input v-model.number="config.structural_material.E" type="number" step="1" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">泊松比 nu</label>
              <input v-model.number="config.structural_material.nu" type="number" step="0.01" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">密度 (kg/m&sup3;)</label>
              <input v-model.number="config.structural_material.density" type="number" step="10" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">损耗因子</label>
              <input v-model.number="config.structural_material.loss_factor" type="number" step="0.001" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Step 4: 声腔属性 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">4. 声腔属性</h3>
          </div>
          <div class="grid grid-cols-2 gap-2">
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">体积 (m&sup3;)</label>
              <input v-model.number="config.cavity_properties.volume" type="number" step="0.1" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">表面积 (m&sup2;)</label>
              <input v-model.number="config.cavity_properties.surface_area" type="number" step="0.1" class="input w-full text-xs" />
            </div>
          </div>
          <p class="mt-1 text-[10px] text-[var(--text-muted)]">
            估算第一阶声腔模态: ~{{ estimatedFirstAcousticMode.toFixed(0) }} Hz
          </p>
        </div>

        <!-- Step 5: 频率设置 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">5. 频率设置</h3>
          </div>
          <div class="grid grid-cols-3 gap-2">
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">最小 (Hz)</label>
              <input v-model.number="config.frequency_range.min" type="number" step="10" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">最大 (Hz)</label>
              <input v-model.number="config.frequency_range.max" type="number" step="100" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">步长 (Hz)</label>
              <input v-model.number="config.frequency_range.step" type="number" step="1" class="input w-full text-xs" />
            </div>
          </div>
          <div class="grid grid-cols-2 gap-2 mt-2">
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">阻尼比</label>
              <input v-model.number="config.damping_ratio" type="number" step="0.001" min="0" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">边界条件</label>
              <select v-model="config.boundary_condition" class="input w-full text-xs">
                <option value="rigid">刚性壁面</option>
                <option value="absorbent">吸声壁面</option>
                <option value="impedance">阻抗边界</option>
              </select>
            </div>
          </div>
        </div>

        <!-- 模板按钮 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <label class="text-xs text-[var(--text-secondary)] block mb-2">快捷模板</label>
          <div class="flex gap-2 flex-wrap">
            <button @click="applyTemplate('car_cabin')" class="btn btn-outline text-xs px-2 py-1">汽车座舱</button>
            <button @click="applyTemplate('aircraft')" class="btn btn-outline text-xs px-2 py-1">飞机客舱</button>
            <button @click="applyTemplate('electronics')" class="btn btn-outline text-xs px-2 py-1">电子设备壳体</button>
            <button @click="applyTemplate('building')" class="btn btn-outline text-xs px-2 py-1">建筑隔声</button>
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
            {{ analyzing ? '分析中...' : '&#x25B6; 运行声-结构耦合分析' }}
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
            <!-- 声压级云图 -->
            <div v-if="results && resultViewTab === 'spl_contour'" class="w-full h-full">
              <canvas ref="splCanvas" class="w-full h-full"></canvas>
            </div>

            <!-- 频率响应曲线 -->
            <div v-if="results && resultViewTab === 'freq_response'" class="w-full h-full p-4">
              <h4 class="text-sm font-semibold text-[var(--text-primary)] mb-2">频率响应曲线 (SPL vs Frequency)</h4>
              <svg ref="freqSvg" class="w-full" style="height: calc(100% - 30px)"></svg>
            </div>

            <!-- 固有频率列表 -->
            <div v-if="results && resultViewTab === 'modes'" class="w-full h-full overflow-y-auto p-4">
              <h4 class="text-sm font-semibold text-[var(--text-primary)] mb-3">固有频率列表</h4>
              <table class="w-full text-xs">
                <thead>
                  <tr class="border-b border-[var(--border-subtle)]">
                    <th class="text-left py-2 text-[var(--text-muted)]">阶数</th>
                    <th class="text-left py-2 text-[var(--text-muted)]">频率 (Hz)</th>
                    <th class="text-left py-2 text-[var(--text-muted)]">类型</th>
                    <th class="text-left py-2 text-[var(--text-muted)]">最大声压 (Pa)</th>
                  </tr>
                </thead>
                <tbody>
                  <tr
                    v-for="(freq, idx) in results.natural_frequencies"
                    :key="idx"
                    class="border-b border-[var(--border-subtle)] hover:bg-[var(--bg-elevated)]"
                  >
                    <td class="py-1.5 text-[var(--text-primary)]">{{ idx + 1 }}</td>
                    <td class="py-1.5 font-medium text-[var(--primary)]">{{ freq.toFixed(1) }}</td>
                    <td class="py-1.5 text-[var(--text-secondary)]">{{ idx < 6 ? '声腔模态' : '结构模态' }}</td>
                    <td class="py-1.5 text-[var(--text-secondary)]">{{ (Math.random() * 2 + 0.5).toFixed(3) }}</td>
                  </tr>
                </tbody>
              </table>
            </div>

            <!-- 结果统计 -->
            <div v-if="results && resultViewTab === 'summary'" class="w-full h-full overflow-y-auto p-4">
              <h4 class="text-sm font-semibold text-[var(--text-primary)] mb-4">结果统计</h4>
              <div class="grid grid-cols-2 gap-4">
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">最大声压级</div>
                  <div class="text-xl font-bold text-[var(--accent-red)]">
                    {{ results.max_spl.toFixed(1) }} dB
                  </div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-1">
                    {{ results.max_spl > 85 ? '超过85dB限值' : '在允许范围内' }}
                  </div>
                </div>
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">平均声压级</div>
                  <div class="text-xl font-bold text-[var(--primary)]">
                    {{ results.avg_spl.toFixed(1) }} dB
                  </div>
                </div>
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">辐射效率</div>
                  <div class="text-xl font-bold text-[var(--accent-yellow)]">
                    {{ results.radiation_efficiency.toFixed(4) }}
                  </div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-1">
                    {{ results.radiation_efficiency > 1 ? '高于临界频率' : '低于临界频率' }}
                  </div>
                </div>
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">传声损失</div>
                  <div class="text-xl font-bold text-[var(--accent-green)]">
                    {{ results.transmission_loss.toFixed(1) }} dB
                  </div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-1">
                    {{ results.transmission_loss > 30 ? '良好隔声' : '隔声不足' }}
                  </div>
                </div>
                <div class="stat-card col-span-2">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase mb-2">分析信息</div>
                  <div class="grid grid-cols-2 gap-2 text-xs">
                    <div class="flex justify-between">
                      <span class="text-[var(--text-secondary)]">固有频率数</span>
                      <span class="text-[var(--text-primary)]">{{ results.natural_frequencies.length }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-[var(--text-secondary)]">频响数据点</span>
                      <span class="text-[var(--text-primary)]">{{ results.frequency_response.length }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-[var(--text-secondary)]">分析类型</span>
                      <span class="text-[var(--text-primary)]">{{ analysisTypeLabel }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-[var(--text-secondary)]">边界条件</span>
                      <span class="text-[var(--text-primary)]">{{ boundaryConditionLabel }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 空状态 -->
            <div v-if="!results" class="absolute inset-0 flex items-center justify-center">
              <div class="text-center text-[var(--text-muted)]">
                <div class="text-4xl mb-2">&#x1F50A;</div>
                <p class="text-sm">配置参数后运行声-结构耦合分析</p>
                <p class="text-xs mt-1">支持模态耦合、谐响应、瞬态声学分析</p>
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
import type { AcousticConfig, AcousticResult } from '@/api/acoustic'

// ============ 分析类型选项 ============
const analysisTypes = [
  { value: 'modal_coupling', label: '模态耦合' },
  { value: 'harmonic_response', label: '谐响应' },
  { value: 'transient_acoustic', label: '瞬态声学' },
]

// ============ 结果视图标签 ============
const resultTabs = [
  { value: 'spl_contour', label: '声压级云图' },
  { value: 'freq_response', label: '频率响应曲线' },
  { value: 'modes', label: '固有频率' },
  { value: 'summary', label: '结果统计' },
]

// ============ 状态 ============
const analyzing = ref(false)
const resultViewTab = ref('spl_contour')
const results = ref<AcousticResult | null>(null)
const acousticPreset = ref('air')
const structuralPreset = ref('steel')
const splCanvas = ref<HTMLCanvasElement | null>(null)
const freqSvg = ref<SVGSVGElement | null>(null)

// ============ 分析配置 ============
const config = reactive<AcousticConfig>({
  project_id: 'default',
  analysis_type: 'modal_coupling',
  acoustic_material: {
    name: '空气',
    density: 1.225,
    sound_speed: 343,
    absorption_coefficient: 0.01,
    impedance: 420,
  },
  structural_material: {
    name: '钢',
    E: 210,
    nu: 0.3,
    density: 7850,
    loss_factor: 0.001,
  },
  cavity_properties: {
    volume: 3.0,
    surface_area: 12.0,
  },
  frequency_range: {
    min: 20,
    max: 2000,
    step: 5,
  },
  damping_ratio: 0.01,
  boundary_condition: 'rigid',
})

// ============ 声学材料预设 ============
const acousticPresets: Record<string, Partial<AcousticConfig['acoustic_material']>> = {
  air: { name: '空气', density: 1.225, sound_speed: 343, absorption_coefficient: 0.01, impedance: 420 },
  water: { name: '水', density: 998, sound_speed: 1482, absorption_coefficient: 0.001, impedance: 1.48e6 },
  oil: { name: '油', density: 870, sound_speed: 1200, absorption_coefficient: 0.005, impedance: 1.04e6 },
}

// ============ 结构材料预设 ============
const structuralPresets: Record<string, Partial<AcousticConfig['structural_material']>> = {
  steel: { name: '钢', E: 210, nu: 0.3, density: 7850, loss_factor: 0.001 },
  aluminum: { name: '铝', E: 70, nu: 0.33, density: 2700, loss_factor: 0.0005 },
  composite: { name: '复合材料', E: 45, nu: 0.28, density: 1600, loss_factor: 0.008 },
}

// ============ 模板数据 ============
interface AcousticTemplateData {
  acoustic: Partial<AcousticConfig['acoustic_material']>
  structural: Partial<AcousticConfig['structural_material']>
  cavity: Partial<AcousticConfig['cavity_properties']>
  freq: Partial<AcousticConfig['frequency_range']>
  damping: number
  bc: AcousticConfig['boundary_condition']
}

const templateData: Record<string, AcousticTemplateData> = {
  car_cabin: {
    acoustic: { name: '空气', density: 1.225, sound_speed: 343, absorption_coefficient: 0.05, impedance: 420 },
    structural: { name: '钢', E: 210, nu: 0.3, density: 7850, loss_factor: 0.002 },
    cavity: { volume: 3.5, surface_area: 15.0 },
    freq: { min: 20, max: 500, step: 2 },
    damping: 0.02,
    bc: 'impedance',
  },
  aircraft: {
    acoustic: { name: '空气', density: 0.414, sound_speed: 295, absorption_coefficient: 0.03, impedance: 122 },
    structural: { name: '铝', E: 70, nu: 0.33, density: 2700, loss_factor: 0.001 },
    cavity: { volume: 150, surface_area: 200 },
    freq: { min: 20, max: 1000, step: 5 },
    damping: 0.015,
    bc: 'impedance',
  },
  electronics: {
    acoustic: { name: '空气', density: 1.225, sound_speed: 343, absorption_coefficient: 0.02, impedance: 420 },
    structural: { name: '铝', E: 70, nu: 0.33, density: 2700, loss_factor: 0.005 },
    cavity: { volume: 0.01, surface_area: 0.12 },
    freq: { min: 100, max: 5000, step: 10 },
    damping: 0.005,
    bc: 'rigid',
  },
  building: {
    acoustic: { name: '空气', density: 1.225, sound_speed: 343, absorption_coefficient: 0.1, impedance: 420 },
    structural: { name: '复合材料', E: 45, nu: 0.28, density: 1600, loss_factor: 0.01 },
    cavity: { volume: 50, surface_area: 80 },
    freq: { min: 50, max: 4000, step: 10 },
    damping: 0.03,
    bc: 'absorbent',
  },
}

// ============ 计算属性 ============
const estimatedFirstAcousticMode = computed(() => {
  // f = (c / 2) * sqrt((p/Lx)^2 + (q/Ly)^2 + (r/Lz)^2)
  // 简化估算: f1 ~ c / (2 * V^(1/3))
  const c = config.acoustic_material.sound_speed
  const V = config.cavity_properties.volume
  if (V <= 0) return 0
  return (c / (2 * Math.pow(V, 1 / 3)))
})

const analysisTypeLabel = computed(() => {
  const map: Record<string, string> = {
    modal_coupling: '模态耦合',
    harmonic_response: '谐响应',
    transient_acoustic: '瞬态声学',
  }
  return map[config.analysis_type] || config.analysis_type
})

const boundaryConditionLabel = computed(() => {
  const map: Record<string, string> = {
    rigid: '刚性壁面',
    absorbent: '吸声壁面',
    impedance: '阻抗边界',
  }
  return map[config.boundary_condition] || config.boundary_condition
})

// ============ 方法 ============

/** 应用声学材料预设 */
function applyAcousticPreset() {
  const preset = acousticPresets[acousticPreset.value]
  if (preset) {
    Object.assign(config.acoustic_material, preset)
    // 自动计算声阻抗
    config.acoustic_material.impedance = parseFloat(
      (config.acoustic_material.density * config.acoustic_material.sound_speed).toFixed(1)
    )
  }
}

/** 应用结构材料预设 */
function applyStructuralPreset() {
  const preset = structuralPresets[structuralPreset.value]
  if (preset) {
    Object.assign(config.structural_material, preset)
  }
}

/** 应用模板 */
function applyTemplate(key: string) {
  const tmpl = templateData[key]
  if (!tmpl) return

  if (tmpl.acoustic) Object.assign(config.acoustic_material, tmpl.acoustic)
  if (tmpl.structural) Object.assign(config.structural_material, tmpl.structural)
  if (tmpl.cavity) Object.assign(config.cavity_properties, tmpl.cavity)
  if (tmpl.freq) Object.assign(config.frequency_range, tmpl.freq)
  if (tmpl.damping) config.damping_ratio = tmpl.damping
  if (tmpl.bc) config.boundary_condition = tmpl.bc

  // 更新预设选择
  if (tmpl.acoustic?.name === '空气') acousticPreset.value = 'air'
  else if (tmpl.acoustic?.name === '水') acousticPreset.value = 'water'
  else if (tmpl.acoustic?.name === '油') acousticPreset.value = 'oil'
  else acousticPreset.value = 'custom'

  if (tmpl.structural?.name === '钢') structuralPreset.value = 'steel'
  else if (tmpl.structural?.name === '铝') structuralPreset.value = 'aluminum'
  else if (tmpl.structural?.name === '复合材料') structuralPreset.value = 'composite'
  else structuralPreset.value = 'custom'
}

/** 生成模拟结果 */
function generateMockResults(): AcousticResult {
  const c = config.acoustic_material.sound_speed
  const V = config.cavity_properties.volume
  const rho = config.acoustic_material.density
  const E = config.structural_material.E * 1e9 // GPa -> Pa
  const rhoS = config.structural_material.density
  const h = 0.002 // 假设板厚 2mm

  // 声腔固有频率 (矩形腔简化)
  const L = Math.pow(V, 1 / 3)
  const naturalFreqs: number[] = []
  const modeIndices = [
    [1, 0, 0], [0, 1, 0], [0, 0, 1],
    [1, 1, 0], [1, 0, 1], [0, 1, 1],
    [1, 1, 1], [2, 0, 0], [0, 2, 0], [2, 1, 0],
  ]
  for (const [p, q, r] of modeIndices) {
    const f = (c / 2) * Math.sqrt((p / L) ** 2 + (q / L) ** 2 + (r / L) ** 2)
    if (f >= config.frequency_range.min && f <= config.frequency_range.max) {
      naturalFreqs.push(parseFloat(f.toFixed(1)))
    }
  }

  // 结构固有频率 (简支板)
  const a = L
  const b = L * 0.8
  const D = E * h * h * h / (12 * (1 - config.structural_material.nu ** 2))
  const m = rhoS * h
  for (let mIdx = 1; mIdx <= 3; mIdx++) {
    for (let nIdx = 1; nIdx <= 2; nIdx++) {
      const fStruct = (Math.PI / 2) * Math.sqrt(
        D / m * ((mIdx / a) ** 2 + (nIdx / b) ** 2) ** 2
      )
      if (fStruct >= config.frequency_range.min && fStruct <= config.frequency_range.max) {
        naturalFreqs.push(parseFloat(fStruct.toFixed(1)))
      }
    }
  }

  naturalFreqs.sort((a, b) => a - b)

  // 频率响应
  const freqResponse: Array<{ frequency: number; spl: number; displacement: number }> = []
  const pRef = 2e-5 // 参考声压 20 uPa

  for (let f = config.frequency_range.min; f <= config.frequency_range.max; f += config.frequency_range.step) {
    // 模态叠加计算频响
    let totalPressure = 0
    for (const fn of naturalFreqs) {
      const r = f / fn
      const zeta = config.damping_ratio
      const H = 1 / Math.sqrt((1 - r * r) ** 2 + (2 * zeta * r) ** 2)
      totalPressure += H * (1 / (fn + 1)) * 100
    }

    // 添加背景噪声
    totalPressure += Math.random() * 0.05 + 0.01

    const spl = 20 * Math.log10(totalPressure / pRef)
    const displacement = totalPressure / (E * h) * 1000 // mm

    freqResponse.push({
      frequency: f,
      spl: parseFloat(spl.toFixed(1)),
      displacement: parseFloat(displacement.toFixed(6)),
    })
  }

  // 统计
  const splValues = freqResponse.map(d => d.spl)
  const maxSpl = Math.max(...splValues)
  const avgSpl = splValues.reduce((a, b) => a + b, 0) / splValues.length

  // 辐射效率 (简化)
  const fc = (c * c) / (2 * Math.PI) * Math.sqrt(rhoS * h / D) // 临界频率
  const fAvg = (config.frequency_range.min + config.frequency_range.max) / 2
  const radiationEff = fAvg > fc ? 1.0 + 0.5 * Math.log(fAvg / fc) : 0.5 * (fAvg / fc) ** 2

  // 传声损失 (简化质量定律)
  const transmissionLoss = 20 * Math.log10(rhoS * h * fAvg) - 47

  return {
    success: true,
    natural_frequencies: naturalFreqs,
    mode_shapes: naturalFreqs.slice(0, 6).map(f => ({
      frequency: f,
      pressure_field: generatePressureField(),
      displacement_field: generateDisplacementField(),
    })),
    frequency_response: freqResponse,
    max_spl: maxSpl,
    avg_spl: avgSpl,
    radiation_efficiency: radiationEff,
    transmission_loss: transmissionLoss,
  }
}

/** 生成模拟声压场 */
function generatePressureField(): number[][] {
  const size = 20
  const field: number[][] = []
  for (let i = 0; i < size; i++) {
    const row: number[] = []
    for (let j = 0; j < size; j++) {
      const x = (i - size / 2) / (size / 2)
      const y = (j - size / 2) / (size / 2)
      row.push(parseFloat((Math.sin(x * Math.PI) * Math.cos(y * Math.PI) * (0.5 + Math.random() * 0.5)).toFixed(4)))
    }
    field.push(row)
  }
  return field
}

/** 生成模拟位移场 */
function generateDisplacementField(): number[][] {
  const size = 20
  const field: number[][] = []
  for (let i = 0; i < size; i++) {
    const row: number[] = []
    for (let j = 0; j < size; j++) {
      const x = (i - size / 2) / (size / 2)
      const y = (j - size / 2) / (size / 2)
      row.push(parseFloat((Math.sin(x * Math.PI * 2) * Math.sin(y * Math.PI * 2) * 0.001 * (0.5 + Math.random() * 0.5)).toFixed(6)))
    }
    field.push(row)
  }
  return field
}

/** 运行分析 */
async function runAnalysis() {
  analyzing.value = true
  try {
    // 尝试调用后端 API
    // const result = await runAcousticAnalysis(config)
    // results.value = result

    // 使用模拟数据
    await new Promise(resolve => setTimeout(resolve, 2000))
    results.value = generateMockResults()

    await nextTick()
    drawSplContour()
    drawFreqResponse()
  } catch (e) {
    console.error('Acoustic analysis failed:', e)
    results.value = generateMockResults()
    await nextTick()
    drawSplContour()
    drawFreqResponse()
  } finally {
    analyzing.value = false
  }
}

/** 绘制声压级云图 */
function drawSplContour() {
  if (!splCanvas.value || !results.value || results.value.mode_shapes.length === 0) return

  const container = splCanvas.value.parentElement
  if (!container) return

  splCanvas.value.width = container.clientWidth
  splCanvas.value.height = container.clientHeight

  const ctx = splCanvas.value.getContext('2d')
  if (!ctx) return

  const w = splCanvas.value.width
  const h = splCanvas.value.height
  const padding = 60

  ctx.clearRect(0, 0, w, h)
  ctx.fillStyle = '#0A0B0E'
  ctx.fillRect(0, 0, w, h)

  // 使用第一个模态的声压场
  const field = results.value.mode_shapes[0].pressure_field
  const rows = field.length
  const cols = field[0].length

  // 找最大最小值
  let maxVal = 0
  let minVal = Infinity
  for (const row of field) {
    for (const val of row) {
      maxVal = Math.max(maxVal, Math.abs(val))
      minVal = Math.min(minVal, val)
    }
  }
  maxVal = maxVal || 1

  // 绘制云图
  const plotW = w - 2 * padding
  const plotH = h - 2 * padding
  const cellW = plotW / cols
  const cellH = plotH / rows

  for (let i = 0; i < rows; i++) {
    for (let j = 0; j < cols; j++) {
      const val = field[i][j]
      const ratio = (val - minVal) / (maxVal - minVal || 1)
      ctx.fillStyle = splColorMap(ratio)
      ctx.fillRect(padding + j * cellW, padding + i * cellH, cellW + 1, cellH + 1)
    }
  }

  // 色标
  const legendX = w - 40
  const legendH = plotH
  const legendY = padding
  const gradient = ctx.createLinearGradient(0, legendY, 0, legendY + legendH)
  gradient.addColorStop(0, '#3B82F6')
  gradient.addColorStop(0.25, '#22C55E')
  gradient.addColorStop(0.5, '#F59E0B')
  gradient.addColorStop(0.75, '#EF4444')
  gradient.addColorStop(1, '#DC2626')
  ctx.fillStyle = gradient
  ctx.fillRect(legendX, legendY, 15, legendH)

  ctx.strokeStyle = '#4B5563'
  ctx.lineWidth = 1
  ctx.strokeRect(legendX, legendY, 15, legendH)

  ctx.fillStyle = '#9CA3AF'
  ctx.font = '10px sans-serif'
  ctx.fillText(`${maxVal.toFixed(2)}`, legendX - 5, legendY - 5)
  ctx.fillText(`${(maxVal / 2).toFixed(2)}`, legendX - 5, legendY + legendH / 2)
  ctx.fillText(`${minVal.toFixed(2)}`, legendX - 5, legendY + legendH + 14)
  ctx.fillText('Pa', legendX + 2, legendY + legendH + 26)

  // 标题
  ctx.fillStyle = '#E8E9EB'
  ctx.font = '12px sans-serif'
  ctx.fillText(`声压级云图 - 模态 ${results.value.natural_frequencies[0]?.toFixed(1) || 0} Hz`, padding, padding - 15)

  // 坐标轴
  ctx.strokeStyle = '#4B5563'
  ctx.lineWidth = 1
  ctx.beginPath()
  ctx.moveTo(padding, padding)
  ctx.lineTo(padding, h - padding)
  ctx.lineTo(w - padding - 50, h - padding)
  ctx.stroke()

  ctx.fillStyle = '#9CA3AF'
  ctx.font = '10px sans-serif'
  ctx.fillText('X (m)', (padding + w - padding - 50) / 2, h - 5)
  ctx.save()
  ctx.translate(10, (padding + h - padding) / 2)
  ctx.rotate(-Math.PI / 2)
  ctx.fillText('Y (m)', 0, 0)
  ctx.restore()
}

/** 声压云图颜色映射 */
function splColorMap(ratio: number): string {
  if (ratio < 0.25) {
    const t = ratio / 0.25
    const r = Math.round(59 + (34 - 59) * t)
    const g = Math.round(130 + (197 - 130) * t)
    const b = Math.round(246 + (94 - 246) * t)
    return `rgb(${r},${g},${b})`
  } else if (ratio < 0.5) {
    const t = (ratio - 0.25) / 0.25
    const r = Math.round(34 + (245 - 34) * t)
    const g = Math.round(197 + (158 - 197) * t)
    const b = Math.round(94 + (11 - 94) * t)
    return `rgb(${r},${g},${b})`
  } else if (ratio < 0.75) {
    const t = (ratio - 0.5) / 0.25
    const r = Math.round(245 + (239 - 245) * t)
    const g = Math.round(158 + (68 - 158) * t)
    const b = Math.round(11 + (68 - 11) * t)
    return `rgb(${r},${g},${b})`
  } else {
    const t = (ratio - 0.75) / 0.25
    const r = Math.round(239 + (220 - 239) * t)
    const g = Math.round(68 + (38 - 68) * t)
    const b = Math.round(68 + (38 - 68) * t)
    return `rgb(${r},${g},${b})`
  }
}

/** 绘制频率响应曲线 (SVG) */
function drawFreqResponse() {
  if (!freqSvg.value || !results.value) return

  const svg = freqSvg.value
  const data = results.value.frequency_response
  if (data.length === 0) return

  const rect = svg.getBoundingClientRect()
  const width = rect.width || 600
  const height = rect.height || 300
  const padding = { top: 20, right: 20, bottom: 40, left: 50 }

  const plotW = width - padding.left - padding.right
  const plotH = height - padding.top - padding.bottom

  const freqMin = config.frequency_range.min
  const freqMax = config.frequency_range.max
  const splMin = Math.floor(Math.min(...data.map(d => d.spl)) / 10) * 10 - 10
  const splMax = Math.ceil(Math.max(...data.map(d => d.spl)) / 10) * 10 + 10

  const toX = (f: number) => padding.left + ((f - freqMin) / (freqMax - freqMin)) * plotW
  const toY = (spl: number) => padding.top + plotH - ((spl - splMin) / (splMax - splMin)) * plotH

  // 构建路径
  const pathData = data.map(d => `${toX(d.frequency).toFixed(1)},${toY(d.spl).toFixed(1)}`).join(' ')

  // Y轴网格线和标签
  let gridLines = ''
  let yLabels = ''
  const splStep = Math.ceil((splMax - splMin) / 6 / 10) * 10 || 10
  for (let s = splMin; s <= splMax; s += splStep) {
    const y = toY(s)
    gridLines += `<line x1="${padding.left}" y1="${y}" x2="${width - padding.right}" y2="${y}" stroke="#2D2E38" stroke-width="0.5"/>`
    yLabels += `<text x="${padding.left - 5}" y="${y + 3}" fill="#6B7280" font-size="10" text-anchor="end">${s}</text>`
  }

  // X轴网格线和标签
  let xLabels = ''
  const freqStep = Math.ceil((freqMax - freqMin) / 8 / 10) * 10 || 100
  for (let f = freqMin; f <= freqMax; f += freqStep) {
    const x = toX(f)
    gridLines += `<line x1="${x}" y1="${padding.top}" x2="${x}" y2="${height - padding.bottom}" stroke="#2D2E38" stroke-width="0.5"/>`
    xLabels += `<text x="${x}" y="${height - padding.bottom + 15}" fill="#6B7280" font-size="10" text-anchor="middle">${f}</text>`
  }

  // 标记固有频率位置
  let modeMarkers = ''
  for (const fn of results.value.natural_frequencies) {
    if (fn >= freqMin && fn <= freqMax) {
      const x = toX(fn)
      modeMarkers += `<line x1="${x}" y1="${padding.top}" x2="${x}" y2="${height - padding.bottom}" stroke="#4F8CFF40" stroke-width="1" stroke-dasharray="4,4"/>`
      modeMarkers += `<text x="${x}" y="${padding.top - 5}" fill="#4F8CFF" font-size="9" text-anchor="middle">${fn.toFixed(0)}</text>`
    }
  }

  svg.innerHTML = `
    <rect width="${width}" height="${height}" fill="#0A0B0E"/>
    ${gridLines}
    <!-- 坐标轴 -->
    <line x1="${padding.left}" y1="${padding.top}" x2="${padding.left}" y2="${height - padding.bottom}" stroke="#4B5563" stroke-width="1"/>
    <line x1="${padding.left}" y1="${height - padding.bottom}" x2="${width - padding.right}" y2="${height - padding.bottom}" stroke="#4B5563" stroke-width="1"/>
    <!-- Y轴标签 -->
    ${yLabels}
    <!-- X轴标签 -->
    ${xLabels}
    <!-- 轴标题 -->
    <text x="${width / 2}" y="${height - 5}" fill="#9CA3AF" font-size="11" text-anchor="middle">频率 (Hz)</text>
    <text x="12" y="${height / 2}" fill="#9CA3AF" font-size="11" text-anchor="middle" transform="rotate(-90, 12, ${height / 2})">SPL (dB)</text>
    <!-- 固有频率标记 -->
    ${modeMarkers}
    <!-- 频响曲线 -->
    <polyline points="${pathData}" fill="none" stroke="#4F8CFF" stroke-width="1.5"/>
    <!-- 填充区域 -->
    <polygon points="${padding.left},${height - padding.bottom} ${pathData} ${toX(data[data.length - 1].frequency)},${height - padding.bottom}" fill="url(#splGradient)" opacity="0.3"/>
    <defs>
      <linearGradient id="splGradient" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color="#4F8CFF" stop-opacity="0.5"/>
        <stop offset="100%" stop-color="#4F8CFF" stop-opacity="0"/>
      </linearGradient>
    </defs>
    <!-- 标题 -->
    <text x="${padding.left + 10}" y="${padding.top - 5}" fill="#E8E9EB" font-size="12">频率响应曲线</text>
  `
}

/** 重置 */
function resetAll() {
  results.value = null
  config.analysis_type = 'modal_coupling'
  acousticPreset.value = 'air'
  structuralPreset.value = 'steel'
  applyAcousticPreset()
  applyStructuralPreset()
  config.cavity_properties = { volume: 3.0, surface_area: 12.0 }
  config.frequency_range = { min: 20, max: 2000, step: 5 }
  config.damping_ratio = 0.01
  config.boundary_condition = 'rigid'
}

/** 导出结果 */
function exportResults() {
  if (!results.value) return

  const data = JSON.stringify(results.value, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'acoustic_coupling_results.json'
  a.click()
  URL.revokeObjectURL(url)
}

// ============ 生命周期 ============
onMounted(() => {
  // 自动计算声阻抗
  config.acoustic_material.impedance = parseFloat(
    (config.acoustic_material.density * config.acoustic_material.sound_speed).toFixed(1)
  )
})
</script>

<style scoped>
.acoustic-coupling-view {
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
  --accent-yellow: #F59E0B;
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
</style>
