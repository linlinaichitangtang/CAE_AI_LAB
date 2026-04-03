<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">多尺度物理量本体库</h2>
        <p class="text-sm" style="color: var(--text-muted)">V1.8-001 | 跨尺度物理量检索、维度分析与单位转换</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="exportHistory" class="btn btn-ghost text-xs">导出记录</button>
        <button @click="resetAll" class="btn btn-ghost text-xs">重置</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Search -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            搜索物理量
          </h4>
          <div class="space-y-2">
            <input
              v-model="searchKeyword"
              type="text"
              placeholder="输入关键词..."
              class="input w-full text-xs"
              @input="filterQuantities"
            />
            <div>
              <label class="label">分类筛选</label>
              <select v-model="selectedCategory" class="input w-full text-xs" @change="filterQuantities">
                <option value="">全部分类</option>
                <option v-for="cat in categories" :key="cat" :value="cat">{{ cat }}</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Step 2: Quantity List -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            物理量列表
            <span class="text-xs ml-2" style="color: var(--text-muted)">({{ filteredQuantities.length }})</span>
          </h4>
          <div class="space-y-1 max-h-48 overflow-y-auto">
            <button
              v-for="q in filteredQuantities"
              :key="q.id"
              @click="selectQuantity(q)"
              class="w-full text-left px-3 py-2 rounded text-xs border transition"
              :style="selectedQuantity && selectedQuantity.id === q.id
                ? 'background: var(--primary); border-color: var(--primary); color: white'
                : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
            >
              <span class="font-mono font-bold">{{ q.symbol }}</span>
              <span class="ml-1">{{ q.name_zh }}</span>
              <span class="ml-1 opacity-60">{{ q.default_unit }}</span>
            </button>
          </div>
        </div>

        <!-- Step 3: Quantity Detail -->
        <div v-if="selectedQuantity" class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            物理量详情
          </h4>
          <div class="p-3 rounded border space-y-2" style="background: var(--bg-elevated); border-color: var(--border-default)">
            <div class="text-xs font-medium" style="color: var(--text-primary)">
              {{ selectedQuantity.symbol }} - {{ selectedQuantity.name_zh }} ({{ selectedQuantity.name_en }})
            </div>
            <div class="text-[10px]" style="color: var(--text-muted)">
              量纲: L<sup>{{ selectedQuantity.dimension.L }}</sup>
              M<sup>{{ selectedQuantity.dimension.M }}</sup>
              T<sup>{{ selectedQuantity.dimension.T }}</sup>
              &Theta;<sup>{{ selectedQuantity.dimension['Θ'] }}</sup>
              I<sup>{{ selectedQuantity.dimension.I }}</sup>
              N<sup>{{ selectedQuantity.dimension.N }}</sup>
              J<sup>{{ selectedQuantity.dimension.J }}</sup>
            </div>
            <div class="border-t pt-2 mt-2" style="border-color: var(--border-subtle)">
              <div class="text-[10px] mb-1" style="color: var(--text-muted)">可用单位</div>
              <div class="space-y-1">
                <div
                  v-for="u in selectedQuantity.units"
                  :key="u.symbol"
                  class="flex justify-between text-[10px] px-2 py-1 rounded"
                  style="background: var(--bg-base); color: var(--text-secondary)"
                >
                  <span>{{ u.name }} ({{ u.symbol }})</span>
                  <span class="font-mono">×{{ u.to_si_factor }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 4: Unit Converter -->
        <div v-if="selectedQuantity" class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            单位转换
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">数值</label>
              <input v-model.number="converter.value" type="number" step="any" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">源单位</label>
              <select v-model="converter.fromUnit" class="input w-full text-xs">
                <option v-for="u in selectedQuantity.units" :key="u.symbol" :value="u.symbol">{{ u.name }} ({{ u.symbol }})</option>
              </select>
            </div>
            <div>
              <label class="label">目标单位</label>
              <select v-model="converter.toUnit" class="input w-full text-xs">
                <option v-for="u in selectedQuantity.units" :key="u.symbol" :value="u.symbol">{{ u.name }} ({{ u.symbol }})</option>
              </select>
            </div>
            <button @click="doConvert" class="btn btn-primary text-xs w-full">转换</button>
            <div v-if="conversionResult" class="p-3 rounded border text-center" style="background: var(--bg-elevated); border-color: var(--border-default)">
              <div class="text-lg font-bold font-mono" style="color: var(--primary)">{{ conversionResult.output_value }}</div>
              <div class="text-[10px] mt-1" style="color: var(--text-muted)">
                {{ converter.value }} {{ converter.fromUnit }} = {{ conversionResult.output_value }} {{ converter.toUnit }}
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization & Results -->
      <div class="flex-1 overflow-y-auto p-6 space-y-6">

        <!-- Category Distribution Chart -->
        <div class="p-4 rounded-lg border" style="background: var(--bg-surface); border-color: var(--border-subtle)">
          <h3 class="text-sm font-medium mb-4" style="color: var(--text-primary)">分类分布</h3>
          <svg :viewBox="`0 0 ${chartWidth} ${chartHeight}`" class="w-full" style="max-height: 280px">
            <!-- Grid lines -->
            <line v-for="i in 5" :key="'grid-'+i"
              :x1="60" :y1="chartHeight - 40 - (i - 1) * ((chartHeight - 80) / 4)"
              :x2="chartWidth - 20" :y2="chartHeight - 40 - (i - 1) * ((chartHeight - 80) / 4)"
              stroke="var(--border-subtle)" stroke-width="1" stroke-dasharray="4,4"
            />
            <!-- Y-axis labels -->
            <text v-for="i in 5" :key="'ylab-'+i"
              :x="55" :y="chartHeight - 36 - (i - 1) * ((chartHeight - 80) / 4)"
              text-anchor="end" fill="var(--text-muted)" font-size="10"
            >{{ Math.round(maxCategoryCount * (i - 1) / 4) }}</text>
            <!-- Bars -->
            <g v-for="(cat, idx) in categoryStats" :key="cat.name">
              <rect
                :x="70 + idx * barWidth"
                :y="chartHeight - 40 - (cat.count / maxCategoryCount) * (chartHeight - 80)"
                :width="barWidth - 12"
                :height="(cat.count / maxCategoryCount) * (chartHeight - 80)"
                :rx="4"
                :fill="barColors[idx % barColors.length]"
                opacity="0.85"
              />
              <text
                :x="70 + idx * barWidth + (barWidth - 12) / 2"
                :y="chartHeight - 24"
                text-anchor="middle" fill="var(--text-secondary)" font-size="10"
              >{{ cat.name }}</text>
              <text
                :x="70 + idx * barWidth + (barWidth - 12) / 2"
                :y="chartHeight - 48 - (cat.count / maxCategoryCount) * (chartHeight - 80)"
                text-anchor="middle" fill="var(--text-primary)" font-size="11" font-weight="bold"
              >{{ cat.count }}</text>
            </g>
          </svg>
        </div>

        <!-- Conversion History Table -->
        <div class="p-4 rounded-lg border" style="background: var(--bg-surface); border-color: var(--border-subtle)">
          <h3 class="text-sm font-medium mb-4" style="color: var(--text-primary)">最近转换记录</h3>
          <div class="overflow-x-auto">
            <table class="w-full text-xs">
              <thead>
                <tr style="border-bottom: 1px solid var(--border-subtle)">
                  <th class="text-left py-2 px-3 font-medium" style="color: var(--text-muted)">物理量</th>
                  <th class="text-left py-2 px-3 font-medium" style="color: var(--text-muted)">源</th>
                  <th class="text-left py-2 px-3 font-medium" style="color: var(--text-muted)">目标</th>
                  <th class="text-left py-2 px-3 font-medium" style="color: var(--text-muted)">结果</th>
                  <th class="text-left py-2 px-3 font-medium" style="color: var(--text-muted)">时间</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(rec, idx) in conversionHistory" :key="idx"
                  style="border-bottom: 1px solid var(--border-subtle)"
                >
                  <td class="py-2 px-3 font-mono" style="color: var(--text-primary)">{{ rec.quantity }}</td>
                  <td class="py-2 px-3" style="color: var(--text-secondary)">{{ rec.from }}</td>
                  <td class="py-2 px-3" style="color: var(--text-secondary)">{{ rec.to }}</td>
                  <td class="py-2 px-3 font-mono font-bold" style="color: var(--primary)">{{ rec.result }}</td>
                  <td class="py-2 px-3" style="color: var(--text-muted)">{{ rec.timestamp }}</td>
                </tr>
                <tr v-if="conversionHistory.length === 0">
                  <td colspan="5" class="py-4 text-center" style="color: var(--text-muted)">暂无转换记录</td>
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
import type { PhysicalQuantity, ConversionRequest, ConversionResult, OntologySearchResult } from '../api/ontology'

// ============ Types ============

interface ConversionRecord {
  quantity: string
  from: string
  to: string
  result: string
  timestamp: string
}

// ============ Mock Data ============

const categories = ['力学', '热学', '电磁', '光学', '量子化学', '材料']

const mockQuantities: PhysicalQuantity[] = [
  {
    id: 'stress', name_zh: '应力', name_en: 'Stress', symbol: 'σ',
    dimension: { L: -1, M: 1, T: -2, 'Θ': 0, I: 0, N: 0, J: 0 },
    default_unit: 'Pa',
    units: [
      { name: '帕斯卡', symbol: 'Pa', to_si_factor: 1 },
      { name: '千帕', symbol: 'kPa', to_si_factor: 1e3 },
      { name: '兆帕', symbol: 'MPa', to_si_factor: 1e6 },
      { name: '吉帕', symbol: 'GPa', to_si_factor: 1e9 },
      { name: '巴', symbol: 'bar', to_si_factor: 1e5 },
      { name: 'psi', symbol: 'psi', to_si_factor: 6894.76 },
    ]
  },
  {
    id: 'strain', name_zh: '应变', name_en: 'Strain', symbol: 'ε',
    dimension: { L: 0, M: 0, T: 0, 'Θ': 0, I: 0, N: 0, J: 0 },
    default_unit: '',
    units: [
      { name: '无量纲', symbol: '-', to_si_factor: 1 },
      { name: '百分数', symbol: '%', to_si_factor: 0.01 },
      { name: '微应变', symbol: 'με', to_si_factor: 1e-6 },
    ]
  },
  {
    id: 'elastic_modulus', name_zh: '弹性模量', name_en: 'Elastic Modulus', symbol: 'E',
    dimension: { L: -1, M: 1, T: -2, 'Θ': 0, I: 0, N: 0, J: 0 },
    default_unit: 'GPa',
    units: [
      { name: '帕斯卡', symbol: 'Pa', to_si_factor: 1 },
      { name: '吉帕', symbol: 'GPa', to_si_factor: 1e9 },
      { name: 'Mbar', symbol: 'Mbar', to_si_factor: 1e11 },
    ]
  },
  {
    id: 'temperature', name_zh: '温度', name_en: 'Temperature', symbol: 'T',
    dimension: { L: 0, M: 0, T: 0, 'Θ': 1, I: 0, N: 0, J: 0 },
    default_unit: 'K',
    units: [
      { name: '开尔文', symbol: 'K', to_si_factor: 1 },
      { name: '摄氏度', symbol: '°C', to_si_factor: 1 },
      { name: '华氏度', symbol: '°F', to_si_factor: 0.5556 },
    ]
  },
  {
    id: 'thermal_conductivity', name_zh: '热导率', name_en: 'Thermal Conductivity', symbol: 'κ',
    dimension: { L: 1, M: 1, T: -3, 'Θ': -1, I: 0, N: 0, J: 0 },
    default_unit: 'W/(m·K)',
    units: [
      { name: 'W/(m·K)', symbol: 'W/(m·K)', to_si_factor: 1 },
      { name: 'cal/(s·cm·K)', symbol: 'cal/(s·cm·K)', to_si_factor: 418.68 },
      { name: 'BTU/(h·ft·°F)', symbol: 'BTU/(h·ft·°F)', to_si_factor: 1.7307 },
    ]
  },
  {
    id: 'specific_heat', name_zh: '比热容', name_en: 'Specific Heat', symbol: 'c_p',
    dimension: { L: 2, M: 0, T: -2, 'Θ': -1, I: 0, N: 0, J: 0 },
    default_unit: 'J/(kg·K)',
    units: [
      { name: 'J/(kg·K)', symbol: 'J/(kg·K)', to_si_factor: 1 },
      { name: 'cal/(g·K)', symbol: 'cal/(g·K)', to_si_factor: 4184 },
      { name: 'eV/(atom·K)', symbol: 'eV/(atom·K)', to_si_factor: 96485.3 },
    ]
  },
  {
    id: 'electric_field', name_zh: '电场强度', name_en: 'Electric Field', symbol: 'E',
    dimension: { L: 1, M: 1, T: -3, 'Θ': 0, I: -1, N: 0, J: 0 },
    default_unit: 'V/m',
    units: [
      { name: 'V/m', symbol: 'V/m', to_si_factor: 1 },
      { name: 'kV/cm', symbol: 'kV/cm', to_si_factor: 1e5 },
      { name: 'V/Å', symbol: 'V/Å', to_si_factor: 1e10 },
    ]
  },
  {
    id: 'magnetic_flux', name_zh: '磁通量', name_en: 'Magnetic Flux', symbol: 'Φ',
    dimension: { L: 2, M: 1, T: -2, 'Θ': 0, I: -1, N: 0, J: 0 },
    default_unit: 'Wb',
    units: [
      { name: '韦伯', symbol: 'Wb', to_si_factor: 1 },
      { name: '麦克斯韦', symbol: 'Mx', to_si_factor: 1e-8 },
      { name: 'T·m²', symbol: 'T·m²', to_si_factor: 1 },
    ]
  },
  {
    id: 'refractive_index', name_zh: '折射率', name_en: 'Refractive Index', symbol: 'n',
    dimension: { L: 0, M: 0, T: 0, 'Θ': 0, I: 0, N: 0, J: 0 },
    default_unit: '',
    units: [
      { name: '无量纲', symbol: '-', to_si_factor: 1 },
    ]
  },
  {
    id: 'wavelength', name_zh: '波长', name_en: 'Wavelength', symbol: 'λ',
    dimension: { L: 1, M: 0, T: 0, 'Θ': 0, I: 0, N: 0, J: 0 },
    default_unit: 'nm',
    units: [
      { name: '米', symbol: 'm', to_si_factor: 1 },
      { name: '毫米', symbol: 'mm', to_si_factor: 1e-3 },
      { name: '微米', symbol: 'μm', to_si_factor: 1e-6 },
      { name: '纳米', symbol: 'nm', to_si_factor: 1e-9 },
      { name: '埃', symbol: 'Å', to_si_factor: 1e-10 },
    ]
  },
  {
    id: 'energy', name_zh: '能量', name_en: 'Energy', symbol: 'E',
    dimension: { L: 2, M: 1, T: -2, 'Θ': 0, I: 0, N: 0, J: 0 },
    default_unit: 'eV',
    units: [
      { name: '焦耳', symbol: 'J', to_si_factor: 1 },
      { name: '电子伏特', symbol: 'eV', to_si_factor: 1.602e-19 },
      { name: '千焦/摩尔', symbol: 'kJ/mol', to_si_factor: 1.6605e-21 },
      { name: 'Hartree', symbol: 'Ha', to_si_factor: 4.3597e-18 },
      { name: 'Rydberg', symbol: 'Ry', to_si_factor: 2.1799e-18 },
      { name: 'kcal/mol', symbol: 'kcal/mol', to_si_factor: 6.9477e-21 },
    ]
  },
  {
    id: 'force', name_zh: '力', name_en: 'Force', symbol: 'F',
    dimension: { L: 1, M: 1, T: -2, 'Θ': 0, I: 0, N: 0, J: 0 },
    default_unit: 'N',
    units: [
      { name: '牛顿', symbol: 'N', to_si_factor: 1 },
      { name: '千牛', symbol: 'kN', to_si_factor: 1e3 },
      { name: '达因', symbol: 'dyn', to_si_factor: 1e-5 },
      { name: 'eV/Å', symbol: 'eV/Å', to_si_factor: 1.602e-9 },
    ]
  },
  {
    id: 'viscosity', name_zh: '粘度', name_en: 'Viscosity', symbol: 'η',
    dimension: { L: -1, M: 1, T: -1, 'Θ': 0, I: 0, N: 0, J: 0 },
    default_unit: 'Pa·s',
    units: [
      { name: 'Pa·s', symbol: 'Pa·s', to_si_factor: 1 },
      { name: 'mPa·s', symbol: 'mPa·s', to_si_factor: 1e-3 },
      { name: 'cP', symbol: 'cP', to_si_factor: 1e-3 },
      { name: 'Poise', symbol: 'P', to_si_factor: 0.1 },
    ]
  },
  {
    id: 'diffusion_coeff', name_zh: '扩散系数', name_en: 'Diffusion Coefficient', symbol: 'D',
    dimension: { L: 2, M: 0, T: -1, 'Θ': 0, I: 0, N: 0, J: 0 },
    default_unit: 'm²/s',
    units: [
      { name: 'm²/s', symbol: 'm²/s', to_si_factor: 1 },
      { name: 'cm²/s', symbol: 'cm²/s', to_si_factor: 1e-4 },
      { name: 'mm²/s', symbol: 'mm²/s', to_si_factor: 1e-6 },
      { name: 'Å²/ps', symbol: 'Å²/ps', to_si_factor: 1e-8 },
    ]
  },
  {
    id: 'surface_energy', name_zh: '表面能', name_en: 'Surface Energy', symbol: 'γ',
    dimension: { L: 0, M: 1, T: -2, 'Θ': 0, I: 0, N: 0, J: 0 },
    default_unit: 'J/m²',
    units: [
      { name: 'J/m²', symbol: 'J/m²', to_si_factor: 1 },
      { name: 'erg/cm²', symbol: 'erg/cm²', to_si_factor: 1e-3 },
      { name: 'eV/Å²', symbol: 'eV/Å²', to_si_factor: 16.02 },
      { name: 'N/m', symbol: 'N/m', to_si_factor: 1 },
    ]
  },
  {
    id: 'charge_density', name_zh: '电荷密度', name_en: 'Charge Density', symbol: 'ρ',
    dimension: { L: -3, M: 0, T: 1, 'Θ': 0, I: 1, N: 0, J: 0 },
    default_unit: 'C/m³',
    units: [
      { name: 'C/m³', symbol: 'C/m³', to_si_factor: 1 },
      { name: 'e/Å³', symbol: 'e/Å³', to_si_factor: 1.602e27 },
      { name: 'e/nm³', symbol: 'e/nm³', to_si_factor: 1.602e18 },
    ]
  },
]

// Category mapping for mock data
const quantityCategoryMap: Record<string, string> = {
  stress: '力学', strain: '力学', elastic_modulus: '力学', force: '力学',
  viscosity: '材料', surface_energy: '材料', diffusion_coeff: '材料',
  temperature: '热学', thermal_conductivity: '热学', specific_heat: '热学',
  electric_field: '电磁', magnetic_flux: '电磁', charge_density: '电磁',
  refractive_index: '光学', wavelength: '光学',
  energy: '量子化学',
}

// ============ State ============

const searchKeyword = ref('')
const selectedCategory = ref('')
const selectedQuantity = ref<PhysicalQuantity | null>(null)
const conversionResult = ref<ConversionResult | null>(null)
const conversionHistory = ref<ConversionRecord[]>([])

const converter = reactive({
  value: 1,
  fromUnit: '',
  toUnit: '',
})

// ============ Chart Config ============

const chartWidth = 600
const chartHeight = 240
const barColors = ['#4F8EF7', '#F7734F', '#4FF7B0', '#F7D84F', '#B04FF7', '#F74F8E']

// ============ Computed ============

const filteredQuantities = computed(() => {
  let result = mockQuantities
  if (selectedCategory.value) {
    result = result.filter(q => quantityCategoryMap[q.id] === selectedCategory.value)
  }
  if (searchKeyword.value.trim()) {
    const kw = searchKeyword.value.trim().toLowerCase()
    result = result.filter(q =>
      q.symbol.toLowerCase().includes(kw) ||
      q.name_zh.includes(kw) ||
      q.name_en.toLowerCase().includes(kw)
    )
  }
  return result
})

const categoryStats = computed(() => {
  const counts: Record<string, number> = {}
  for (const cat of categories) {
    counts[cat] = 0
  }
  for (const q of mockQuantities) {
    const cat = quantityCategoryMap[q.id]
    if (cat && counts[cat] !== undefined) {
      counts[cat]++
    }
  }
  return categories.map(name => ({ name, count: counts[name] || 0 }))
})

const maxCategoryCount = computed(() => {
  return Math.max(...categoryStats.value.map(c => c.count), 1)
})

const barWidth = computed(() => {
  return (chartWidth - 90) / categories.length
})

// ============ Methods ============

function filterQuantities() {
  // Computed handles filtering automatically
}

function selectQuantity(q: PhysicalQuantity) {
  selectedQuantity.value = q
  converter.fromUnit = q.units[0]?.symbol || ''
  converter.toUnit = q.units[1]?.symbol || q.units[0]?.symbol || ''
  converter.value = 1
  conversionResult.value = null
}

function doConvert() {
  if (!selectedQuantity.value) return

  const fromDef = selectedQuantity.value.units.find(u => u.symbol === converter.fromUnit)
  const toDef = selectedQuantity.value.units.find(u => u.symbol === converter.toUnit)
  if (!fromDef || !toDef) return

  // Convert: value * fromFactor -> SI -> / toFactor
  const valueInSI = converter.value * fromDef.to_si_factor
  const resultValue = valueInSI / toDef.to_si_factor

  conversionResult.value = {
    input_value: converter.value,
    output_value: parseFloat(resultValue.toPrecision(8)),
    from_unit: converter.fromUnit,
    to_unit: converter.toUnit,
    quantity: selectedQuantity.value.name_zh,
    uncertainty: 0,
  }

  // Add to history
  const now = new Date()
  const timestamp = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:${now.getSeconds().toString().padStart(2, '0')}`
  conversionHistory.value.unshift({
    quantity: `${selectedQuantity.value.symbol} ${selectedQuantity.value.name_zh}`,
    from: `${converter.value} ${converter.fromUnit}`,
    to: converter.toUnit,
    result: conversionResult.value.output_value.toString(),
    timestamp,
  })

  // Keep only last 20 records
  if (conversionHistory.value.length > 20) {
    conversionHistory.value = conversionHistory.value.slice(0, 20)
  }
}

function exportHistory() {
  if (conversionHistory.value.length === 0) return
  const csv = ['物理量,源,目标,结果,时间']
  for (const rec of conversionHistory.value) {
    csv.push(`${rec.quantity},${rec.from},${rec.to},${rec.result},${rec.timestamp}`)
  }
  const blob = new Blob([csv.join('\n')], { type: 'text/csv' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'conversion_history.csv'
  a.click()
  URL.revokeObjectURL(url)
}

function resetAll() {
  searchKeyword.value = ''
  selectedCategory.value = ''
  selectedQuantity.value = null
  conversionResult.value = null
  converter.value = 1
  converter.fromUnit = ''
  converter.toUnit = ''
}

// ============ Lifecycle ============

onMounted(() => {
  nextTick(() => {
    // Pre-populate some conversion history for demo
    const demoHistory: ConversionRecord[] = [
      { quantity: 'σ 应力', from: '100 MPa', to: 'bar', result: '1000', timestamp: '14:32:05' },
      { quantity: 'E 能量', from: '1 eV', to: 'J', result: '1.602e-19', timestamp: '14:28:11' },
      { quantity: 'λ 波长', from: '532 nm', to: 'Å', result: '5320', timestamp: '14:15:42' },
      { quantity: 'κ 热导率', from: '200 W/(m·K)', to: 'cal/(s·cm·K)', result: '0.4777', timestamp: '13:55:20' },
      { quantity: 'D 扩散系数', from: '1e-5 cm²/s', to: 'Å²/ps', result: '1', timestamp: '13:40:08' },
    ]
    conversionHistory.value = demoHistory
  })
})
</script>
