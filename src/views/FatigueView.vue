<template>
  <div class="fatigue-analysis">
    <!-- Header -->
    <div class="header-section">
      <h2 class="text-lg font-semibold">疲劳分析</h2>
      <p class="text-xs text-[var(--text-muted)]">高周/低周疲劳分析,寿命预测,损伤累积</p>
    </div>

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

    <!-- Content -->
    <div class="content">
      <!-- S-N Curve Analysis (High Cycle Fatigue) -->
      <div v-if="currentTab === 'sn'" class="panel-section">
        <h4 class="text-sm font-medium mb-3">高周疲劳 S-N 曲线</h4>
        
        <!-- Stress Ratio R -->
        <div class="mb-3">
          <label class="label">应力比 R (σmin/σmax)</label>
          <input v-model.number="params.sn.stress_ratio" type="number" step="0.1" class="input" />
        </div>

        <!-- S-N Data Points -->
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
        <h4 class="text-sm font-medium mb-3">低周疲劳 E-N 曲线</h4>
        
        <!-- Strain Amplitude -->
        <div class="mb-3">
          <label class="label">应变幅 Δε/2</label>
          <input v-model.number="params.en.strain_amplitude" type="number" step="0.0001" class="input" />
        </div>

        <!-- Cyclic Properties -->
        <div class="mb-3">
          <label class="label">循环应力系数 K'</label>
          <input v-model.number="params.en.cyclic_stress_coeff" type="number" step="1" class="input" />
        </div>
        <div class="mb-3">
          <label class="label">循环应变硬��指数 n'</label>
          <input v-model.number="params.en.cyclic_exponent" type="number" step="0.01" class="input" />
        </div>

        <!-- Neuber Correction -->
        <div class="mb-3">
          <label class="label">
            <input v-model.boolean="params.en.use_neuber" type="checkbox" class="checkbox" />
            启用 Neuber 修正
          </label>
        </div>
      </div>

      <!-- Random Vibration Fatigue (PSD) -->
      <div v-if="currentTab === 'psd'" class="panel-section">
        <h4 class="text-sm font-medium mb-3">随机振动疲劳 PSD谱分析</h4>
        
        <!-- PSD Data Import -->
        <div class="mb-3">
          <label class="label">PSD 谱数据 (频率-功率谱密度)</label>
          <textarea v-model="params.psd.psd_data" class="input textarea" rows="5" placeholder="频率, PSD值&#10;10, 0.001&#10;20, 0.002&#10;..."></textarea>
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
        <h4 class="text-sm font-medium mb-3">载荷谱输入</h4>
        
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

        <!-- Stress Concentration -->
        <div class="mb-3">
          <label class="label">应力集中系数 Kt</label>
          <input v-model.number="params.kt" type="number" step="0.1" class="input" />
        </div>

        <!-- Surface Treatment -->
        <div class="mb-3">
          <label class="label">表面处理</label>
          <select v-model="params.surface_treatment" class="input">
            <option value="machined">机加工</option>
            <option value="as_cast">铸态</option>
            <option value="polished">抛光</option>
            <option value="shot_peened">喷丸</option>
          </select>
        </div>
      </div>

      <!-- Analysis Results -->
      <div v-if="results" class="results-section">
        <h4 class="text-sm font-medium mb-3">分析结果</h4>
        
        <!-- Damage Accumulation -->
        <div class="result-card">
          <span class="result-label">损伤累积 D</span>
          <span :class="['result-value', results.damage > 1 ? 'danger' : 'safe']">
            {{ results.damage.toFixed(4) }}
          </span>
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
      </div>

      <!-- Run Analysis Button -->
      <button @click="runAnalysis" :disabled="analyzing" class="btn-primary">
        {{ analyzing ? '分析中...' : '运行疲劳分析' }}
      </button>

      <!-- Export Results -->
      <button v-if="results" @click="exportResults" class="btn-secondary">
        导出结果
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Analysis type tabs
const analysisTypes = [
  { value: 'sn', label: 'S-N曲线' },
  { value: 'en', label: 'E-N曲线' },
  { value: 'psd', label: 'PSD谱' }
]

const currentTab = ref('sn')
const analyzing = ref(false)

// Parameters
const params = reactive({
  // S-N Curve params
  sn: {
    stress_ratio: -1,
    dataPoints: [
      { stress: 300, cycles: 1e4 },
      { stress: 200, cycles: 1e5 },
      { stress: 150, cycles: 1e6 }
    ],
    fatigue_limit: 100,
    mean_stress_correction: 'none'
  },
  // E-N Curve params
  en: {
    strain_amplitude: 0.01,
    cyclic_stress_coeff: 1000,
    cyclic_exponent: 0.2,
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
    stress_amplitude: 150
  },
  kt: 1.0,
  surface_treatment: 'machined'
})

// Results
const results = ref<{
  damage: number
  life_cycles: number
  safety_factor: number
} | null>(null)

// S-N data points management
const addSNPoint = () => {
  params.sn.dataPoints.push({ stress: 0, cycles: 0 })
}

const removeSNPoint = (idx: number) => {
  params.sn.dataPoints.splice(idx, 1)
}

// Format number
const formatNumber = (n: number): string => {
  if (n >= 1e6) return (n / 1e6).toFixed(2) + 'M'
  if (n >= 1e3) return (n / 1e3).toFixed(2) + 'K'
  return n.toFixed(0)
}

// Run analysis
const runAnalysis = async () => {
  analyzing.value = true
  try {
    results.value = await invoke('fatigue_analysis', {
      params: {
        analysis_type: currentTab.value,
        sn_params: params.sn,
        en_params: params.en,
        psd_params: params.psd,
        load_type: params.load_type,
        load: params.load,
        kt: params.kt,
        surface_treatment: params.surface_treatment
      }
    })
  } catch (e) {
    console.error('Fatigue analysis failed:', e)
  } finally {
    analyzing.value = false
  }
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
</script>

<style scoped>
.fatigue-analysis {
  padding: 16px;
  color: var(--text-primary);
}

.header-section {
  margin-bottom: 16px;
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

.results-section {
  margin: 16px 0;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
}

.result-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid var(--border-color);
}

.result-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.result-value {
  font-size: 14px;
  font-weight: 600;
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
  padding: 12px;
  background: transparent;
  border: 1px solid var(--accent-color);
  border-radius: 4px;
  color: var(--accent-color);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
}
</style>