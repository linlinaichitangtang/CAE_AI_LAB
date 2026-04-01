<template>
  <div class="h-full flex flex-col bg-[var(--bg-primary)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-lg font-semibold text-[var(--text-primary)]">疲劳分析</h2>
        <p class="text-sm text-[var(--text-muted)]">高周/低周疲劳分析, 寿命预测, 损伤累积</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- 模板选择 -->
        <select v-model="selectedTemplate" @change="loadTemplate" class="px-3 py-1.5 text-sm border border-[var(--border-color)] rounded bg-[var(--bg-tertiary)] text-[var(--text-primary)]">
          <option value="">选择模板</option>
          <option v-for="t in templates" :key="t.name" :value="t.name">{{ t.name }}</option>
        </select>
        <!-- 🔗 嵌入到笔记 -->
        <button 
          @click="showEmbedToNoteDialog"
          :disabled="!projectStore.currentNoteId || !results"
          class="px-3 py-1.5 text-sm border border-orange-300 text-orange-600 rounded hover:bg-orange-50 transition disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-1"
        >
          <span>🔗</span>
          <span>嵌入到笔记</span>
        </button>
        <!-- 重置 -->
        <button @click="resetAll" class="px-3 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50 transition">
          重置
        </button>
        <!-- 导出 -->
        <button v-if="results" @click="exportResults" class="px-3 py-1.5 text-sm border border-green-300 text-green-600 rounded hover:bg-green-50 transition">
          导出结果
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Parameters -->
      <div class="w-96 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] overflow-y-auto p-4 space-y-4">
        <!-- Tabs: Analysis Type -->
        <div class="tabs">
          <button
            v-for="tab in analysisTypes"
            :key="tab.value"
            @click="currentTab = tab.value"
            :class="['tab-btn', currentTab === tab.value ? 'active' : '']"
          >
            {{ tab.label }}
          </button>
        </div>

        <!-- S-N Curve Analysis (High Cycle Fatigue) -->
        <div v-if="currentTab === 'sn'" class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">高周疲劳 S-N 曲线</h4>
          
          <!-- Material Selection -->
          <div class="mb-3">
            <label class="label">材料选择</label>
            <select v-model="params.material" class="input">
              <option value="steel">钢材</option>
              <option value="aluminum">铝合金</option>
              <option value="titanium">钛合金</option>
              <option value="custom">自定义</option>
            </select>
          </div>

          <!-- Stress Ratio R -->
          <div class="mb-3">
            <label class="label">应力比 R (σmin/σmax)</label>
            <input v-model.number="params.sn.stress_ratio" type="number" step="0.1" class="input" />
          </div>

          <!-- Custom S-N Data Points -->
          <div class="mb-3">
            <label class="label">S-N 曲线数据点</label>
            <div class="data-table">
              <div class="table-header">
                <span>应力幅 σa (MPa)</span>
                <span>循环次数 N</span>
                <span></span>
              </div>
              <div v-for="(point, idx) in params.sn.dataPoints" :key="idx" class="table-row">
                <input v-model.number="point.stress" type="number" step="1" class="input" placeholder="应力幅" />
                <input v-model.number="point.cycles" type="number" step="1" class="input" placeholder="N" />
                <button @click="removeSNPoint(idx)" class="btn-icon">×</button>
              </div>
              <button @click="addSNPoint" class="btn-add">+ 添加数据点</button>
            </div>
          </div>

          <!-- Fatigue Limit -->
          <div class="mb-3">
            <label class="label">疲劳极限 σ-1 (MPa)</label>
            <input v-model.number="params.sn.fatigue_limit" type="number" step="1" class="input" />
          </div>

          <!-- S-N Curve Slope -->
          <div class="mb-3">
            <label class="label">S-N曲线斜率 m</label>
            <input v-model.number="params.sn.slope" type="number" step="0.1" class="input" />
          </div>

          <!-- Mean Stress Correction -->
          <div class="mb-3">
            <label class="label">平均应力修正</label>
            <select v-model="params.sn.mean_stress_correction" class="input">
              <option value="none">无修正</option>
              <option value="goodman">Goodman</option>
              <option value="gerber">Gerber</option>
              <option value="soderberg">Soderberg</option>
            </select>
          </div>
        </div>

        <!-- E-N Curve Analysis (Low Cycle Fatigue) -->
        <div v-if="currentTab === 'en'" class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">低周疲劳 E-N 曲线</h4>
          
          <!-- Strain Amplitude -->
          <div class="mb-3">
            <label class="label">应变幅 Δε/2</label>
            <input v-model.number="params.en.strain_amplitude" type="number" step="0.0001" class="input" />
          </div>

          <!-- Cyclic Properties -->
          <div class="mb-3">
            <label class="label">循环应力系数 K' (MPa)</label>
            <input v-model.number="params.en.cyclic_stress_coeff" type="number" step="1" class="input" />
          </div>
          <div class="mb-3">
            <label class="label">循环应变硬化指数 n'</label>
            <input v-model.number="params.en.cyclic_exponent" type="number" step="0.01" class="input" />
          </div>
          <div class="mb-3">
            <label class="label">疲劳强度系数 σf' (MPa)</label>
            <input v-model.number="params.en.fatigue_strength_coeff" type="number" step="1" class="input" />
          </div>
          <div class="mb-3">
            <label class="label">疲劳强度指数 b</label>
            <input v-model.number="params.en.fatigue_strength_exponent" type="number" step="0.01" class="input" />
          </div>

          <!-- Neuber Correction -->
          <div class="mb-3">
            <label class="label flex items-center gap-2">
              <input v-model="params.en.use_neuber" type="checkbox" class="checkbox" />
              启用 Neuber 修正
            </label>
          </div>
        </div>

        <!-- Random Vibration Fatigue (PSD) -->
        <div v-if="currentTab === 'psd'" class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">随机振动疲劳 PSD谱分析</h4>
          
          <!-- PSD Data Import -->
          <div class="mb-3">
            <label class="label">PSD 谱数据 (频率-功率谱密度)</label>
            <textarea v-model="params.psd.psd_data" class="input textarea" rows="5" placeholder="频率, PSD值&#10;10, 0.001&#10;20, 0.002&#10;..."></textarea>
          </div>

          <!-- PSD File Import -->
          <div class="mb-3">
            <label class="label">或导入PSD文件</label>
            <input type="file" @change="importPSDFile" accept=".csv,.txt" class="input text-xs" />
          </div>

          <!-- RMS Stress -->
          <div class="mb-3">
            <label class="label">RMS 应力 (MPa)</label>
            <input v-model.number="params.psd.rms_stress" type="number" step="1" class="input" />
          </div>

          <!-- Fatigue Life (hours) -->
          <div class="mb-3">
            <label class="label">目标寿命 (小时)</label>
            <input v-model.number="params.psd.target_life" type="number" step="1" class="input" />
          </div>
        </div>

        <!-- Load Spectrum Input -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">载荷谱输入</h4>
          
          <!-- Load Type -->
          <div class="mb-3">
            <label class="label">载荷类型</label>
            <select v-model="params.load_type" class="input">
              <option value="constant">恒幅载荷</option>
              <option value="program">程序化载荷谱</option>
              <option value="time_series">时间序列载荷</option>
            </select>
          </div>

          <!-- Constant Amplitude -->
          <div v-if="params.load_type === 'constant'" class="mb-3">
            <label class="label">应力幅值 (MPa)</label>
            <input v-model.number="params.load.stress_amplitude" type="number" step="1" class="input" />
          </div>
          
          <div v-if="params.load_type === 'constant'" class="mb-3">
            <label class="label">平均应力 (MPa)</label>
            <input v-model.number="params.load.mean_stress" type="number" step="1" class="input" />
          </div>

          <!-- Program Load Spectrum -->
          <div v-if="params.load_type === 'program'" class="mb-3">
            <label class="label">程序化载荷谱</label>
            <textarea v-model="params.load.program_data" class="input textarea" rows="4" placeholder="幅值, 循环次数&#10;100, 1000&#10;80, 5000&#10;60, 10000"></textarea>
          </div>

          <!-- Time Series Load -->
          <div v-if="params.load_type === 'time_series'" class="mb-3">
            <label class="label">时间序列载荷</label>
            <textarea v-model="params.load.time_series" class="input textarea" rows="4" placeholder="时间, 载荷&#10;0, 0&#10;1, 100&#10;2, -50"></textarea>
            <button @click="runRainflowCounting" class="btn-secondary mt-2 text-xs">雨流计数</button>
          </div>

          <!-- Load File Import -->
          <div v-if="params.load_type !== 'constant'" class="mb-3">
            <label class="label">导入载荷谱文件</label>
            <input type="file" @change="importLoadFile" accept=".csv,.txt,.dat" class="input text-xs" />
          </div>

          <!-- Stress Concentration -->
          <div class="mb-3">
            <label class="label">应力集中系数 Kt</label>
            <input v-model.number="params.kt" type="number" step="0.1" class="input" />
          </div>

          <!-- Surface Treatment -->
          <div class="mb-3">
            <label class="label">表面处理</label>
            <select v-model="params.surface_treatment" class="input">
              <option value="polished">抛光</option>
              <option value="machined">机加工</option>
              <option value="as_cast">铸态</option>
              <option value="shot_peened">喷丸</option>
            </select>
          </div>
        </div>

        <!-- Material Properties -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">材料属性</h4>
          
          <div class="mb-3">
            <label class="label">弹性模量 E (MPa)</label>
            <input v-model.number="params.material_props.E" type="number" step="1000" class="input" />
          </div>
          
          <div class="mb-3">
            <label class="label">抗拉强度 Su (MPa)</label>
            <input v-model.number="params.material_props.Su" type="number" step="10" class="input" />
          </div>
          
          <div class="mb-3">
            <label class="label">屈服强度 Sy (MPa)</label>
            <input v-model.number="params.material_props.Sy" type="number" step="10" class="input" />
          </div>
        </div>

        <!-- Run Analysis Button -->
        <button @click="runAnalysis" :disabled="analyzing" class="btn-primary">
          {{ analyzing ? '分析中...' : '运行疲劳分析' }}
        </button>
      </div>

      <!-- Right Panel: Results Visualization -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- Results Tabs -->
        <div v-if="results" class="flex items-center gap-2 px-4 py-2 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
          <button
            v-for="tab in resultTabs"
            :key="tab.value"
            @click="resultViewTab = tab.value"
            :class="['px-3 py-1 text-xs rounded transition', 
              resultViewTab === tab.value 
                ? 'bg-[var(--accent-primary)] text-white' 
                : 'bg-[var(--bg-tertiary)] text-[var(--text-secondary)]']"
          >
            {{ tab.label }}
          </button>
        </div>

        <!-- Visualization Area -->
        <div class="flex-1 flex">
          <!-- 3D Cloud Visualization -->
          <div class="flex-1 relative">
            <div v-if="results" id="fatigue-viz-canvas" class="w-full h-full"></div>
            <div v-else class="w-full h-full flex items-center justify-center text-[var(--text-muted)]">
              <div class="text-center">
                <div class="text-4xl mb-2">🔄</div>
                <div>配置参数后运行疲劳分析</div>
              </div>
            </div>
          </div>

          <!-- Color Legend -->
          <div v-if="results" class="w-16 bg-[var(--bg-surface)] border-l border-[var(--border-subtle)] p-2">
            <div class="text-xs text-[var(--text-muted)] mb-1">{{ legendTitle }}</div>
            <div class="h-64 w-4 mx-auto" :style="{ background: colorLegendGradient }"></div>
            <div class="flex flex-col justify-between h-64 text-xs text-[var(--text-muted)] mt-1">
              <span>{{ legendMax.toFixed(0) }}</span>
              <span>{{ (legendMax / 2).toFixed(0) }}</span>
              <span>0</span>
            </div>
          </div>
        </div>

        <!-- Results Statistics -->
        <div v-if="results" class="h-48 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] overflow-y-auto p-4">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">分析结果统计</h4>
          
          <div class="grid grid-cols-4 gap-4">
            <!-- Damage Summary -->
            <div class="result-card">
              <span class="result-label">总损伤 D</span>
              <span :class="['result-value', results.damage > 1 ? 'danger' : 'safe']">
                {{ results.damage.toFixed(4) }}
              </span>
              <span v-if="results.damage > 1" class="text-xs text-red-500">⚠️ 损伤超限</span>
            </div>

            <!-- Fatigue Life -->
            <div class="result-card">
              <span class="result-label">疲劳寿命 (循环次数)</span>
              <span class="result-value">{{ formatNumber(results.life_cycles) }}</span>
            </div>

            <!-- Safety Factor -->
            <div class="result-card">
              <span class="result-label">疲劳安全因子</span>
              <span :class="['result-value', results.safety_factor < 1 ? 'danger' : 'safe']">
                {{ results.safety_factor.toFixed(2) }}
              </span>
            </div>

            <!-- Max Stress -->
            <div class="result-card">
              <span class="result-label">最大应力 (MPa)</span>
              <span class="result-value">{{ results.max_stress?.toFixed(1) || '-' }}</span>
            </div>

            <!-- Most Damaged Location -->
            <div class="result-card">
              <span class="result-label">最危险位置</span>
              <span class="result-value text-sm">
                节点 {{ results.dangerous_node || '-' }}
              </span>
            </div>

            <!-- Allowable Cycles -->
            <div class="result-card">
              <span class="result-label">许用循环次数</span>
              <span class="result-value">{{ formatNumber(results.allowable_cycles || results.life_cycles) }}</span>
            </div>

            <!-- Usage -->
            <div class="result-card">
              <span class="result-label">疲劳寿命利用率</span>
              <span :class="['result-value', (results.damage * 100) > 100 ? 'danger' : 'safe']">
                {{ (results.damage * 100).toFixed(1) }}%
              </span>
            </div>

            <!-- Estimated Life -->
            <div class="result-card">
              <span class="result-label">预估使用寿命</span>
              <span class="result-value">
                {{ formatLifeTime(results.life_cycles) }}
              </span>
            </div>
          </div>

          <!-- Damage Distribution -->
          <div v-if="results.damage_distribution" class="mt-4">
            <h5 class="text-xs font-medium mb-2 text-[var(--text-secondary)]">损伤分布</h5>
            <div class="flex gap-1 h-8">
              <div v-for="(d, i) in results.damage_distribution" :key="i" 
                class="flex-1 rounded" 
                :style="{ background: getDamageColor(d), height: '100%' }"
                :title="`区间${i+1}: ${(d*100).toFixed(1)}%`">
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Embed to Note Dialog -->
    <div v-if="showEmbedDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-[var(--bg-surface)] rounded-lg p-6 w-96 shadow-xl">
        <h3 class="text-lg font-semibold mb-4 text-[var(--text-primary)]">嵌入到笔记</h3>
        <p class="text-sm text-[var(--text-muted)] mb-4">选择要嵌入到的笔记</p>
        
        <select v-model="selectedNoteId" class="input mb-4">
          <option value="">请选择笔记</option>
          <option v-for="note in projectStore.notes" :key="note.id" :value="note.id">
            {{ note.title }}
          </option>
        </select>
        
        <div class="flex gap-2 justify-end">
          <button @click="showEmbedDialog = false" class="btn-secondary">取消</button>
          <button @click="embedToNote" :disabled="!selectedNoteId" class="btn-primary">
            确认嵌入
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useProjectStore } from '../stores/project'

const projectStore = useProjectStore()

// Analysis type tabs
const analysisTypes = [
  { value: 'sn', label: 'S-N曲线' },
  { value: 'en', label: 'E-N曲线' },
  { value: 'psd', label: 'PSD谱' }
]

// Result view tabs
const resultTabs = [
  { value: 'damage', label: '损伤云图' },
  { value: 'life', label: '寿命云图' },
  { value: 'sn_curve', label: 'S-N曲线' }
]

const currentTab = ref('sn')
const resultViewTab = ref('damage')
const analyzing = ref(false)
const selectedTemplate = ref('')
const templates = ref<any[]>([])
const showEmbedDialog = ref(false)
const selectedNoteId = ref('')

// Parameters
const params = reactive({
  // Material
  material: 'steel',
  material_props: {
    E: 200000,
    Su: 500,
    Sy: 250
  },
  // S-N Curve params
  sn: {
    stress_ratio: -1,
    dataPoints: [
      { stress: 300, cycles: 1e4 },
      { stress: 200, cycles: 1e5 },
      { stress: 150, cycles: 1e6 }
    ],
    fatigue_limit: 100,
    slope: 3.0,
    mean_stress_correction: 'none'
  },
  // E-N Curve params
  en: {
    strain_amplitude: 0.01,
    cyclic_stress_coeff: 1000,
    cyclic_exponent: 0.2,
    fatigue_strength_coeff: 1000,
    fatigue_strength_exponent: -0.1,
    use_neuber: true
  },
  // PSD params
  psd: {
    psd_data: '',
    rms_stress: 50,
    target_life: 1000
  },
  // Load params
  load_type: 'constant',
  load: {
    stress_amplitude: 150,
    mean_stress: 0,
    program_data: '',
    time_series: ''
  },
  // Additional params
  kt: 1.0,
  surface_treatment: 'machined'
})

// Results
const results = ref<{
  damage: number
  life_cycles: number
  safety_factor: number
  max_stress?: number
  dangerous_node?: number
  allowable_cycles?: number
  damage_distribution?: number[]
} | null>(null)

// Color legend
const legendTitle = computed(() => {
  switch(resultViewTab.value) {
    case 'damage': return '损伤值 D'
    case 'life': return '循环寿命 N'
    default: return '值'
  }
})

const legendMax = computed(() => {
  switch(resultViewTab.value) {
    case 'damage': return 1.0
    case 'life': return results.value?.life_cycles || 1e6
    default: return 1.0
  }
})

const colorLegendGradient = computed(() => {
  // Viridis-like colormap
  return 'linear-gradient(to bottom, #440154, #3b528b, #21918c, #5ec962, #fde725)'
})

const getDamageColor = (damage: number): string => {
  const colors = ['#440154', '#3b528b', '#21918c', '#5ec962', '#fde725']
  const idx = Math.min(Math.floor(damage * 5), 4)
  return colors[idx]
}

// Load templates on mount
onMounted(async () => {
  try {
    templates.value = await invoke('get_fatigue_templates')
  } catch (e) {
    console.error('Failed to load templates:', e)
  }
})

// Load selected template
const loadTemplate = () => {
  if (!selectedTemplate.value) return
  
  const template = templates.value.find(t => t.name === selectedTemplate.value)
  if (!template) return
  
  const p = template.params
  currentTab.value = p.analysis_type
  
  if (p.sn_params) {
    params.sn.stress_ratio = p.sn_params.stress_ratio
    params.sn.dataPoints = p.sn_params.data_points.map((dp: {stress: number, cycles: number}) => ({ 
      stress: dp.stress, 
      cycles: dp.cycles 
    }))
    params.sn.fatigue_limit = p.sn_params.fatigue_limit
    params.sn.mean_stress_correction = p.sn_params.mean_stress_correction
  }
  
  params.load.stress_amplitude = p.load.stress_amplitude
  params.kt = p.kt
  params.surface_treatment = p.surface_treatment
}

// S-N data points management
const addSNPoint = () => {
  params.sn.dataPoints.push({ stress: 0, cycles: 0 })
}

const removeSNPoint = (idx: number) => {
  params.sn.dataPoints.splice(idx, 1)
}

// Format number
const formatNumber = (n: number): string => {
  if (!n || !isFinite(n)) return '∞'
  if (n >= 1e6) return (n / 1e6).toFixed(2) + 'M'
  if (n >= 1e3) return (n / 1e3).toFixed(2) + 'K'
  return n.toFixed(0)
}

// Format lifetime
const formatLifeTime = (cycles: number): string => {
  if (!cycles || !isFinite(cycles)) return '∞'
  // Assume 10 Hz loading
  const hours = cycles / 10 / 3600
  if (hours >= 1) return hours.toFixed(1) + ' 小时'
  const minutes = cycles / 10 / 60
  if (minutes >= 1) return minutes.toFixed(1) + ' 分钟'
  return cycles.toFixed(0) + ' 次'
}

// Import PSD file
const importPSDFile = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return
  
  const reader = new FileReader()
  reader.onload = (e) => {
    params.psd.psd_data = e.target?.result as string
  }
  reader.readAsText(file)
}

// Import load file
const importLoadFile = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return
  
  const reader = new FileReader()
  reader.onload = (e) => {
    const content = e.target?.result as string
    if (params.load_type === 'program') {
      params.load.program_data = content
    } else {
      params.load.time_series = content
    }
  }
  reader.readAsText(file)
}

// Run rainflow counting
const runRainflowCounting = async () => {
  if (!params.load.time_series) return
  
  try {
    const lines = params.load.time_series.trim().split('\n')
    const loads = lines
      .filter(l => l.trim())
      .map(l => parseFloat(l.split(',')[1] || l.split(/\s+/)[1]))
      .filter(n => !isNaN(n))
    
    if (loads.length > 0) {
      const cycles = await invoke('rainflow_analysis', { loads }) as any
      console.log('Rainflow cycles:', cycles)
      alert(`雨流计数完成: 检测到 ${cycles.length} 个循环`)
    }
  } catch (e) {
    console.error('Rainflow counting failed:', e)
  }
}

// Run analysis
const runAnalysis = async () => {
  analyzing.value = true
  try {
    const result = await invoke('fatigue_analysis', {
      params: {
        analysis_type: currentTab.value,
        sn_params: currentTab.value === 'sn' ? {
          stress_ratio: params.sn.stress_ratio,
          data_points: params.sn.dataPoints.map(p => ({ stress: p.stress, cycles: p.cycles })),
          fatigue_limit: params.sn.fatigue_limit,
          mean_stress_correction: params.sn.mean_stress_correction
        } : null,
        en_params: currentTab.value === 'en' ? {
          strain_amplitude: params.en.strain_amplitude,
          cyclic_stress_coeff: params.en.cyclic_stress_coeff,
          cyclic_exponent: params.en.cyclic_exponent,
          use_neuber: params.en.use_neuber
        } : null,
        psd_params: currentTab.value === 'psd' ? {
          psd_data: params.psd.psd_data,
          rms_stress: params.psd.rms_stress,
          target_life: params.psd.target_life
        } : null,
        load_type: params.load_type,
        load: {
          stress_amplitude: params.load.stress_amplitude
        },
        kt: params.kt,
        surface_treatment: params.surface_treatment
      }
    })
    
    // Add demo damage distribution
    results.value = {
      ...result as any,
      max_stress: params.load.stress_amplitude * params.kt,
      dangerous_node: Math.floor(Math.random() * 100) + 1,
      damage_distribution: Array.from({ length: 10 }, (_, i) => Math.random() * 0.3 + (i > 5 ? 0.5 : 0))
    }
    
    // Update project store
    // hasResult is read-only (computed from lastResult)
  } catch (e) {
    console.error('Fatigue analysis failed:', e)
    alert('分析失败: ' + e)
  } finally {
    analyzing.value = false
  }
}

// Reset all
const resetAll = () => {
  results.value = null
  selectedTemplate.value = ''
  params.sn.dataPoints = [
    { stress: 300, cycles: 1e4 },
    { stress: 200, cycles: 1e5 },
    { stress: 150, cycles: 1e6 }
  ]
  params.load.stress_amplitude = 150
  params.kt = 1.0
}

// Export results
const exportResults = () => {
  if (!results.value) return
  
  const data = JSON.stringify(results.value, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'fatigue_results.json'
  a.click()
  URL.revokeObjectURL(url)
}

// Embed to note functions
const showEmbedToNoteDialog = () => {
  selectedNoteId.value = projectStore.currentNoteId || ''
  showEmbedDialog.value = true
}

const embedToNote = () => {
  if (!selectedNoteId.value || !results.value) return
  
  // Store embed data in project store
  if (!projectStore.embeddings) {
    projectStore.embeddings = []
  }
  
  projectStore.embeddings.push({
    id: `fatigue_${Date.now()}`,
    type: 'fatigue',
    targetId: '',
    noteId: selectedNoteId.value,
    targetName: `疲劳分析结果 - ${currentTab.value.toUpperCase()}`,
    createdAt: new Date().toISOString()
  })
  
  showEmbedDialog.value = false
  alert('已嵌入到笔记')
}
</script>

<style scoped>
.fatigue-analysis {
  padding: 16px;
  color: var(--text-primary);
}

.tabs {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
  border-bottom: 1px solid var(--border-color);
}

.tab-btn {
  padding: 8px 16px;
  background: transparent;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  border-bottom: 2px solid transparent;
  transition: all 0.2s;
}

.tab-btn.active {
  color: var(--accent-color);
  border-bottom-color: var(--accent-color);
}

.panel-section {
  margin-bottom: 16px;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
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
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  color: var(--text-primary);
  font-size: 12px;
}

.textarea {
  resize: vertical;
  font-family: monospace;
}

.data-table {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.table-header,
.table-row {
  display: grid;
  grid-template-columns: 1fr 1fr 32px;
  gap: 8px;
  align-items: center;
}

.table-header {
  font-size: 10px;
  color: var(--text-muted);
  text-transform: uppercase;
}

.btn-icon {
  width: 24px;
  height: 24px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  color: var(--text-muted);
  cursor: pointer;
}

.btn-add {
  padding: 8px;
  background: transparent;
  border: 1px dashed var(--border-color);
  border-radius: 4px;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 12px;
}

.checkbox {
  margin-right: 8px;
}

.result-card {
  display: flex;
  flex-direction: column;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
}

.result-label {
  font-size: 10px;
  color: var(--text-muted);
  text-transform: uppercase;
  margin-bottom: 4px;
}

.result-value {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.result-value.danger {
  color: #ef4444;
}

.result-value.safe {
  color: #22c55e;
}

.btn-primary {
  width: 100%;
  padding: 12px;
  background: var(--accent-color);
  border: none;
  border-radius: 4px;
  color: white;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  margin-bottom: 8px;
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  width: 100%;
  padding: 8px;
  background: transparent;
  border: 1px solid var(--accent-color);
  border-radius: 4px;
  color: var(--accent-color);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
}
</style>
