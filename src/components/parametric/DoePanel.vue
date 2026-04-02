<template>
  <div class="doe-panel" :class="{ 'doe-panel--dark': isDark }">
    <!-- DOE Configuration -->
    <div class="doe-panel__section">
      <h3 class="doe-panel__section-title">DOE 实验设计</h3>

      <!-- Parameter selection -->
      <div class="doe-panel__form-group">
        <label class="doe-panel__label">实验参数</label>
        <div class="doe-panel__param-list">
          <div
            v-for="(param, idx) in doeParams"
            :key="idx"
            class="doe-panel__param-item"
          >
            <input
              v-model="param.name"
              placeholder="参数名"
              class="doe-panel__input doe-panel__input--name"
            />
            <input
              type="number"
              v-model.number="param.min"
              placeholder="最小值"
              class="doe-panel__input doe-panel__input--num"
            />
            <span class="doe-panel__separator">~</span>
            <input
              type="number"
              v-model.number="param.max"
              placeholder="最大值"
              class="doe-panel__input doe-panel__input--num"
            />
            <button
              class="doe-panel__remove-btn"
              @click="doeParams.splice(idx, 1)"
              :disabled="doeParams.length <= 1"
            >
              &times;
            </button>
          </div>
        </div>
        <button class="doe-panel__add-btn" @click="addDoeParam">+ 添加参数</button>
      </div>

      <!-- Sampling method -->
      <div class="doe-panel__form-group">
        <label class="doe-panel__label">采样方法</label>
        <select v-model="samplingMethod" class="doe-panel__select">
          <option v-for="(label, key) in methodLabels" :key="key" :value="key">
            {{ label }}
          </option>
        </select>
        <p class="doe-panel__hint">{{ methodDescriptions[samplingMethod] }}</p>
      </div>

      <!-- Sample count (for non-factorial methods) -->
      <div class="doe-panel__form-group" v-if="samplingMethod !== 'FullFactorial' && samplingMethod !== 'CentralComposite'">
        <label class="doe-panel__label">样本数量</label>
        <input
          type="number"
          v-model.number="numSamples"
          :min="2"
          :max="10000"
          class="doe-panel__input doe-panel__input--full"
        />
      </div>

      <!-- Levels (for FullFactorial) -->
      <div class="doe-panel__form-group" v-if="samplingMethod === 'FullFactorial'">
        <label class="doe-panel__label">每个参数的层级数</label>
        <input
          type="number"
          v-model.number="factorialLevels"
          :min="2"
          :max="20"
          class="doe-panel__input doe-panel__input--full"
        />
        <p class="doe-panel__hint">
          预计总样本数: {{ estimatedSamples }}
        </p>
      </div>

      <!-- Run button -->
      <button
        class="btn btn-primary doe-panel__run-btn"
        @click="runDoe"
        :disabled="doeRunning || doeParams.length === 0"
      >
        {{ doeRunning ? '生成中...' : '生成 DOE 样本' }}
      </button>
    </div>

    <!-- DOE Results -->
    <div class="doe-panel__section" v-if="doeResult">
      <h3 class="doe-panel__section-title">
        DOE 样本 ({{ doeResult.total_samples }} 个)
      </h3>

      <!-- Samples table -->
      <div class="doe-panel__table-wrapper">
        <table class="doe-panel__table">
          <thead>
            <tr>
              <th>#</th>
              <th v-for="param in doeParams" :key="param.name">{{ param.name }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(sample, idx) in doeResult.samples" :key="idx">
              <td>{{ idx + 1 }}</td>
              <td v-for="param in doeParams" :key="param.name">
                {{ (sample[param.name] ?? 0).toFixed(4) }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Sensitivity Analysis -->
    <div class="doe-panel__section" v-if="sensitivityResults.length > 0">
      <h3 class="doe-panel__section-title">灵敏度分析</h3>

      <!-- Sensitivity bar chart (pure SVG) -->
      <div class="doe-panel__chart">
        <svg :viewBox="`0 0 ${chartWidth} ${chartHeight}`" class="doe-panel__svg">
          <!-- Title -->
          <text :x="chartWidth / 2" y="16" font-size="12" fill="var(--text-primary, #374151)" text-anchor="middle" font-weight="600">
            参数灵敏度分数
          </text>

          <!-- Grid lines -->
          <line
            v-for="i in 5"
            :key="'grid-' + i"
            :x1="chartLeft" :y1="chartTop + (i - 1) * chartRowHeight"
            :x2="chartWidth - 20" :y2="chartTop + (i - 1) * chartRowHeight"
            stroke="var(--border-color, #e5e7eb)" stroke-width="0.5"
          />

          <!-- Zero line -->
          <line
            :x1="chartCenterX" :y1="chartTop - 5"
            :x2="chartCenterX" :y2="chartTop + sensitivityResults.length * chartRowHeight + 5"
            stroke="var(--text-secondary, #9ca3af)" stroke-width="1" stroke-dasharray="4,2"
          />

          <!-- X axis labels -->
          <text :x="chartLeft" :y="chartTop + sensitivityResults.length * chartRowHeight + 18"
            font-size="9" fill="var(--text-secondary, #9ca3af)" text-anchor="middle">-1</text>
          <text :x="chartCenterX" :y="chartTop + sensitivityResults.length * chartRowHeight + 18"
            font-size="9" fill="var(--text-secondary, #9ca3af)" text-anchor="middle">0</text>
          <text :x="chartWidth - 20" :y="chartTop + sensitivityResults.length * chartRowHeight + 18"
            font-size="9" fill="var(--text-secondary, #9ca3af)" text-anchor="middle">+1</text>

          <!-- Bars -->
          <g v-for="(item, idx) in sensitivityResults" :key="item.parameter_name">
            <!-- Label -->
            <text
              :x="chartLeft - 8"
              :y="chartTop + idx * chartRowHeight + chartRowHeight / 2 + 4"
              font-size="11"
              fill="var(--text-primary, #374151)"
              text-anchor="end"
            >
              {{ item.parameter_name }}
            </text>

            <!-- Bar background (full range) -->
            <rect
              :x="chartLeft"
              :y="chartTop + idx * chartRowHeight + 4"
              :width="chartWidth - 20 - chartLeft"
              :height="chartRowHeight - 8"
              rx="3"
              fill="var(--bg-tertiary, #f3f4f6)"
            />

            <!-- Positive bar (green) -->
            <rect
              v-if="item.sensitivity_score > 0"
              :x="chartCenterX"
              :y="chartTop + idx * chartRowHeight + 4"
              :width="Math.abs(item.sensitivity_score) * barHalfWidth"
              :height="chartRowHeight - 8"
              rx="3"
              fill="#16a34a"
              opacity="0.8"
              class="doe-panel__bar"
              @mouseenter="hoveredBar = idx"
              @mouseleave="hoveredBar = -1"
            />

            <!-- Negative bar (red) -->
            <rect
              v-if="item.sensitivity_score < 0"
              :x="chartCenterX - Math.abs(item.sensitivity_score) * barHalfWidth"
              :y="chartTop + idx * chartRowHeight + 4"
              :width="Math.abs(item.sensitivity_score) * barHalfWidth"
              :height="chartRowHeight - 8"
              rx="3"
              fill="#dc2626"
              opacity="0.8"
              class="doe-panel__bar"
              @mouseenter="hoveredBar = idx"
              @mouseleave="hoveredBar = -1"
            />

            <!-- Score label -->
            <text
              :x="item.sensitivity_score >= 0 ? chartCenterX + Math.abs(item.sensitivity_score) * barHalfWidth + 6 : chartCenterX - Math.abs(item.sensitivity_score) * barHalfWidth - 6"
              :y="chartTop + idx * chartRowHeight + chartRowHeight / 2 + 4"
              font-size="10"
              :fill="item.sensitivity_score >= 0 ? '#16a34a' : '#dc2626'"
              :text-anchor="item.sensitivity_score >= 0 ? 'start' : 'end'"
              font-weight="600"
            >
              {{ item.sensitivity_score.toFixed(3) }}
            </text>
          </g>

          <!-- Tooltip -->
          <g v-if="hoveredBar >= 0 && sensitivityResults[hoveredBar]" class="doe-panel__tooltip">
            <rect
              x="10" :y="chartTop + hoveredBar * chartRowHeight - 10"
              width="180" height="50" rx="6"
              fill="var(--bg-primary, #ffffff)"
              stroke="var(--border-color, #d1d5db)"
              stroke-width="1"
              filter="drop-shadow(0 2px 4px rgba(0,0,0,0.1))"
            />
            <text x="18" :y="chartTop + hoveredBar * chartRowHeight + 6" font-size="10" fill="var(--text-primary, #374151)" font-weight="600">
              {{ sensitivityResults[hoveredBar].parameter_name }}
            </text>
            <text x="18" :y="chartTop + hoveredBar * chartRowHeight + 22" font-size="9" fill="var(--text-secondary, #6b7280)">
              主效应: {{ sensitivityResults[hoveredBar].main_effect.toFixed(4) }}
            </text>
            <text x="18" :y="chartTop + hoveredBar * chartRowHeight + 36" font-size="9" fill="var(--text-secondary, #6b7280)">
              灵敏度: {{ sensitivityResults[hoveredBar].sensitivity_score.toFixed(4) }}
            </text>
          </g>
        </svg>
      </div>

      <!-- Interaction effects -->
      <div class="doe-panel__interactions" v-if="hasInteractions">
        <h4 class="doe-panel__sub-title">交互效应</h4>
        <div
          v-for="item in sensitivityResults"
          :key="'inter-' + item.parameter_name"
          class="doe-panel__interaction-group"
        >
          <div
            v-for="[otherName, score] in item.interaction_effects"
            :key="item.parameter_name + '-' + otherName"
            class="doe-panel__interaction-item"
          >
            <span class="doe-panel__interaction-label">
              {{ item.parameter_name }} x {{ otherName }}
            </span>
            <div class="doe-panel__interaction-bar-wrapper">
              <div
                class="doe-panel__interaction-bar"
                :class="score >= 0 ? 'positive' : 'negative'"
                :style="{ width: Math.abs(score) * 100 + '%' }"
              />
            </div>
            <span class="doe-panel__interaction-score" :class="score >= 0 ? 'positive' : 'negative'">
              {{ score.toFixed(3) }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Parameter-Result Scatter Plot (pure SVG) -->
    <div class="doe-panel__section" v-if="scatterData.length > 0">
      <h3 class="doe-panel__section-title">参数-结果散点图</h3>

      <div class="doe-panel__scatter-controls">
        <select v-model="selectedScatterParam" class="doe-panel__select doe-panel__select--sm">
          <option v-for="p in scatterParams" :key="p" :value="p">{{ p }}</option>
        </select>
      </div>

      <div class="doe-panel__chart">
        <svg :viewBox="`0 0 ${scatterWidth} ${scatterHeight}`" class="doe-panel__svg">
          <!-- Grid -->
          <line v-for="i in 5" :key="'sg-' + i"
            :x1="scatterLeft" :y1="scatterTop + (i - 1) * scatterRowH"
            :x2="scatterWidth - 20" :y2="scatterTop + (i - 1) * scatterRowH"
            stroke="var(--border-color, #e5e7eb)" stroke-width="0.5"
          />
          <line v-for="i in 5" :key="'sgv-' + i"
            :x1="scatterLeft + (i - 1) * scatterColW"
            :y1="scatterTop"
            :x2="scatterLeft + (i - 1) * scatterColW"
            :y2="scatterHeight - 30"
            stroke="var(--border-color, #e5e7eb)" stroke-width="0.5"
          />

          <!-- Axis labels -->
          <text :x="scatterLeft" :y="scatterHeight - 10" font-size="9" fill="var(--text-secondary, #9ca3af)" text-anchor="middle">
            {{ scatterXMin.toFixed(1) }}
          </text>
          <text :x="scatterWidth - 20" :y="scatterHeight - 10" font-size="9" fill="var(--text-secondary, #9ca3af)" text-anchor="middle">
            {{ scatterXMax.toFixed(1) }}
          </text>
          <text :x="scatterLeft - 8" :y="scatterTop + 4" font-size="9" fill="var(--text-secondary, #9ca3af)" text-anchor="end">
            {{ scatterYMax.toFixed(2) }}
          </text>
          <text :x="scatterLeft - 8" :y="scatterHeight - 30" font-size="9" fill="var(--text-secondary, #9ca3af)" text-anchor="end">
            {{ scatterYMin.toFixed(2) }}
          </text>

          <!-- Axis titles -->
          <text :x="(scatterLeft + scatterWidth - 20) / 2" :y="scatterHeight - 0" font-size="10" fill="var(--text-primary, #374151)" text-anchor="middle">
            {{ selectedScatterParam }}
          </text>
          <text x="8" :y="(scatterTop + scatterHeight - 30) / 2" font-size="10" fill="var(--text-primary, #374151)" text-anchor="middle"
            transform="rotate(-90, 8, 120)">
            结果值
          </text>

          <!-- Data points -->
          <circle
            v-for="(pt, idx) in currentScatterPoints"
            :key="'pt-' + idx"
            :cx="pt.x"
            :cy="pt.y"
            r="4"
            fill="var(--primary, #4f46e5)"
            opacity="0.7"
            class="doe-panel__scatter-point"
            @mouseenter="hoveredPoint = idx"
            @mouseleave="hoveredPoint = -1"
          />

          <!-- Trend line -->
          <line
            v-if="currentScatterPoints.length >= 2"
            :x1="trendLine.x1" :y1="trendLine.y1"
            :x2="trendLine.x2" :y2="trendLine.y2"
            stroke="var(--primary, #4f46e5)"
            stroke-width="1.5"
            stroke-dasharray="6,3"
            opacity="0.5"
          />

          <!-- Point tooltip -->
          <g v-if="hoveredPoint >= 0 && currentScatterPoints[hoveredPoint]">
            <rect
              :x="currentScatterPoints[hoveredPoint].x + 8"
              :y="currentScatterPoints[hoveredPoint].y - 24"
              width="120" height="32" rx="4"
              fill="var(--bg-primary, #ffffff)"
              stroke="var(--border-color, #d1d5db)"
            />
            <text
              :x="currentScatterPoints[hoveredPoint].x + 14"
              :y="currentScatterPoints[hoveredPoint].y - 10"
              font-size="9" fill="var(--text-primary, #374151)"
            >
              {{ selectedScatterParam }}: {{ scatterData[hoveredPoint].xVal.toFixed(3) }}
            </text>
            <text
              :x="currentScatterPoints[hoveredPoint].x + 14"
              :y="currentScatterPoints[hoveredPoint].y + 2"
              font-size="9" fill="var(--text-primary, #374151)"
            >
              结果: {{ scatterData[hoveredPoint].yVal.toFixed(4) }}
            </text>
          </g>
        </svg>
      </div>
    </div>

    <!-- Calculate sensitivity button -->
    <div class="doe-panel__section" v-if="hasScanResults && sensitivityResults.length === 0">
      <button
        class="btn btn-primary doe-panel__run-btn"
        @click="runSensitivity"
        :disabled="sensitivityRunning"
      >
        {{ sensitivityRunning ? '计算中...' : '计算灵敏度分析' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import {
  runDoeStudy,
  calculateSensitivity,
  SAMPLING_METHOD_LABELS,
  SAMPLING_METHOD_DESCRIPTIONS,
  type DoeConfig,
  type DoeResult,
  type SensitivityResult,
  type DoeSamplingMethod,
  type ScanCaseResult,
  type Parameter,
} from '@/api/parametric'

const props = defineProps<{
  scanResults?: ScanCaseResult[]
  scanParameters?: Parameter[]
}>()

const isDark = ref(false)
const doeRunning = ref(false)
const sensitivityRunning = ref(false)
const hoveredBar = ref(-1)
const hoveredPoint = ref(-1)

// DOE config
const doeParams = ref<Array<{ name: string; min: number; max: number }>>([
  { name: 'param_1', min: 0, max: 100 },
])
const samplingMethod = ref<DoeSamplingMethod>('LatinHypercube')
const numSamples = ref(20)
const factorialLevels = ref(3)

// Results
const doeResult = ref<DoeResult | null>(null)
const sensitivityResults = ref<SensitivityResult[]>([])

// Scatter
const selectedScatterParam = ref('')

const methodLabels = SAMPLING_METHOD_LABELS
const methodDescriptions = SAMPLING_METHOD_DESCRIPTIONS

// Chart dimensions
const chartWidth = 400
const chartHeight = 200
const chartLeft = 80
const chartTop = 28
const chartRowHeight = 32
const chartCenterX = (chartWidth - 20 + chartLeft) / 2
const barHalfWidth = computed(() => (chartWidth - 20 - chartLeft) / 2)

// Scatter dimensions
const scatterWidth = 400
const scatterHeight = 260
const scatterLeft = 50
const scatterTop = 20
const scatterRowH = computed(() => (scatterHeight - 30 - scatterTop) / 4)
const scatterColW = computed(() => (scatterWidth - 20 - scatterLeft) / 4)

// Estimated samples
const estimatedSamples = computed(() => {
  if (samplingMethod.value === 'FullFactorial') {
    return Math.pow(factorialLevels.value, doeParams.value.length)
  }
  if (samplingMethod.value === 'CentralComposite') {
    const k = doeParams.value.length
    return Math.pow(2, k) + 2 * k + 1
  }
  return numSamples.value
})

// Has scan results
const hasScanResults = computed(() => {
  return props.scanResults && props.scanResults.length >= 2
})

// Has interactions
const hasInteractions = computed(() => {
  return sensitivityResults.value.some(r => r.interaction_effects.length > 0)
})

// Scatter data
const scatterData = computed(() => {
  if (!props.scanResults || props.scanResults.length === 0) return []
  return props.scanResults
    .filter(r => r.success)
    .map(r => {
      const xVal = r.parameter_values[selectedScatterParam.value] ?? 0
      const yVal = r.max_stress ?? r.max_displacement ?? r.max_von_mises ?? 0
      return { xVal, yVal }
    })
    .filter(d => !isNaN(d.xVal) && !isNaN(d.yVal))
})

const scatterParams = computed(() => {
  if (!props.scanResults || props.scanResults.length === 0) return []
  const names = new Set<string>()
  for (const r of props.scanResults) {
    for (const key of Object.keys(r.parameter_values)) {
      names.add(key)
    }
  }
  return Array.from(names)
})

const scatterXMin = computed(() => {
  if (scatterData.value.length === 0) return 0
  return Math.min(...scatterData.value.map(d => d.xVal))
})

const scatterXMax = computed(() => {
  if (scatterData.value.length === 0) return 1
  return Math.max(...scatterData.value.map(d => d.xVal))
})

const scatterYMin = computed(() => {
  if (scatterData.value.length === 0) return 0
  return Math.min(...scatterData.value.map(d => d.yVal))
})

const scatterYMax = computed(() => {
  if (scatterData.value.length === 0) return 1
  return Math.max(...scatterData.value.map(d => d.yVal))
})

const currentScatterPoints = computed(() => {
  const xRange = scatterXMax.value - scatterXMin.value || 1
  const yRange = scatterYMax.value - scatterYMin.value || 1
  const plotW = scatterWidth - 20 - scatterLeft
  const plotH = scatterHeight - 30 - scatterTop

  return scatterData.value.map(d => ({
    x: scatterLeft + ((d.xVal - scatterXMin.value) / xRange) * plotW,
    y: scatterTop + plotH - ((d.yVal - scatterYMin.value) / yRange) * plotH,
  }))
})

const trendLine = computed(() => {
  const pts = currentScatterPoints.value
  if (pts.length < 2) return { x1: 0, y1: 0, x2: 0, y2: 0 }

  const n = pts.length
  let sumX = 0, sumY = 0, sumXY = 0, sumX2 = 0
  for (const p of pts) {
    sumX += p.x
    sumY += p.y
    sumXY += p.x * p.y
    sumX2 += p.x * p.x
  }
  const denom = n * sumX2 - sumX * sumX
  if (Math.abs(denom) < 1e-12) return { x1: pts[0].x, y1: pts[0].y, x2: pts[n - 1].x, y2: pts[n - 1].y }

  const slope = (n * sumXY - sumX * sumY) / denom
  const intercept = (sumY - slope * sumX) / n

  const minX = Math.min(...pts.map(p => p.x))
  const maxX = Math.max(...pts.map(p => p.x))

  return {
    x1: minX,
    y1: slope * minX + intercept,
    x2: maxX,
    y2: slope * maxX + intercept,
  }
})

// Watch for scatter params
watch(scatterParams, (params) => {
  if (params.length > 0 && !params.includes(selectedScatterParam.value)) {
    selectedScatterParam.value = params[0]
  }
}, { immediate: true })

function addDoeParam() {
  const idx = doeParams.value.length + 1
  doeParams.value.push({ name: `param_${idx}`, min: 0, max: 100 })
}

async function runDoe() {
  doeRunning.value = true
  try {
    const config: DoeConfig = {
      parameters: doeParams.value.map(p => ({
        name: p.name,
        min: p.min,
        max: p.max,
        levels: samplingMethod.value === 'FullFactorial' ? factorialLevels.value : undefined,
      })),
      sampling_method: samplingMethod.value,
      num_samples: numSamples.value,
    }
    doeResult.value = await runDoeStudy(config)
  } catch (e: any) {
    console.error('DOE study failed:', e)
  } finally {
    doeRunning.value = false
  }
}

async function runSensitivity() {
  if (!props.scanResults || !props.scanParameters) return
  sensitivityRunning.value = true
  try {
    sensitivityResults.value = await calculateSensitivity(props.scanResults, props.scanParameters)
  } catch (e: any) {
    console.error('Sensitivity analysis failed:', e)
  } finally {
    sensitivityRunning.value = false
  }
}

function checkDarkMode() {
  isDark.value = document.documentElement.classList.contains('dark')
}

onMounted(() => {
  checkDarkMode()
  const observer = new MutationObserver(checkDarkMode)
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] })
  onUnmounted(() => observer.disconnect())
})
</script>

<style scoped>
.doe-panel {
  padding: 16px;
}

.doe-panel__section {
  margin-bottom: 20px;
}

.doe-panel__section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary, #374151);
  margin-bottom: 12px;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--border-color, #e5e7eb);
}

.doe-panel--dark .doe-panel__section-title {
  color: #e5e7eb;
  border-color: #374151;
}

.doe-panel__form-group {
  margin-bottom: 12px;
}

.doe-panel__label {
  display: block;
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary, #6b7280);
  margin-bottom: 4px;
}

.doe-panel__hint {
  font-size: 11px;
  color: var(--text-secondary, #9ca3af);
  margin-top: 4px;
}

.doe-panel__param-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 8px;
}

.doe-panel__param-item {
  display: flex;
  align-items: center;
  gap: 6px;
}

.doe-panel__input {
  padding: 5px 8px;
  font-size: 12px;
  border: 1px solid var(--border-color, #d1d5db);
  border-radius: 6px;
  background: var(--bg-primary, #ffffff);
  color: var(--text-primary, #374151);
  outline: none;
}

.doe-panel--dark .doe-panel__input {
  background: #111827;
  border-color: #4b5563;
  color: #e5e7eb;
}

.doe-panel__input:focus {
  border-color: var(--primary, #4f46e5);
}

.doe-panel__input--name {
  flex: 1;
  min-width: 0;
}

.doe-panel__input--num {
  width: 80px;
}

.doe-panel__input--full {
  width: 100%;
  box-sizing: border-box;
}

.doe-panel__separator {
  color: var(--text-secondary, #9ca3af);
  font-size: 12px;
}

.doe-panel__remove-btn {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: #ef4444;
  font-size: 16px;
  cursor: pointer;
  border-radius: 4px;
}

.doe-panel__remove-btn:hover:not(:disabled) {
  background: rgba(239, 68, 68, 0.1);
}

.doe-panel__remove-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.doe-panel__add-btn {
  padding: 4px 10px;
  font-size: 12px;
  background: transparent;
  color: var(--primary, #4f46e5);
  border: 1px dashed var(--primary, #4f46e5);
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s;
}

.doe-panel__add-btn:hover {
  background: rgba(79, 70, 229, 0.06);
}

.doe-panel__select {
  width: 100%;
  padding: 6px 10px;
  font-size: 13px;
  border: 1px solid var(--border-color, #d1d5db);
  border-radius: 6px;
  background: var(--bg-primary, #ffffff);
  color: var(--text-primary, #374151);
  outline: none;
}

.doe-panel--dark .doe-panel__select {
  background: #111827;
  border-color: #4b5563;
  color: #e5e7eb;
}

.doe-panel__select--sm {
  width: auto;
  min-width: 120px;
}

.doe-panel__run-btn {
  width: 100%;
  padding: 10px;
  font-size: 13px;
  margin-top: 8px;
}

.btn {
  padding: 8px 16px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  border: none;
}

.btn-primary {
  background: var(--primary, #4f46e5);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  opacity: 0.9;
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Table */
.doe-panel__table-wrapper {
  max-height: 300px;
  overflow: auto;
  border: 1px solid var(--border-color, #e5e7eb);
  border-radius: 8px;
}

.doe-panel--dark .doe-panel__table-wrapper {
  border-color: #374151;
}

.doe-panel__table {
  width: 100%;
  border-collapse: collapse;
  font-size: 12px;
}

.doe-panel__table th {
  padding: 8px 12px;
  text-align: left;
  background: var(--bg-secondary, #f9fafb);
  color: var(--text-secondary, #6b7280);
  font-weight: 500;
  position: sticky;
  top: 0;
  z-index: 1;
}

.doe-panel--dark .doe-panel__table th {
  background: #1f2937;
  color: #9ca3af;
}

.doe-panel__table td {
  padding: 6px 12px;
  border-top: 1px solid var(--border-color, #e5e7eb);
  color: var(--text-primary, #374151);
}

.doe-panel--dark .doe-panel__table td {
  border-color: #374151;
  color: #d1d5db;
}

.doe-panel__table tr:hover td {
  background: var(--bg-secondary, #f9fafb);
}

.doe-panel--dark .doe-panel__table tr:hover td {
  background: #1f2937;
}

/* Chart */
.doe-panel__chart {
  background: var(--bg-primary, #ffffff);
  border: 1px solid var(--border-color, #e5e7eb);
  border-radius: 8px;
  padding: 8px;
  overflow: hidden;
}

.doe-panel--dark .doe-panel__chart {
  background: #111827;
  border-color: #374151;
}

.doe-panel__svg {
  width: 100%;
  height: auto;
}

.doe-panel__bar {
  transition: opacity 0.15s;
}

.doe-panel__bar:hover {
  opacity: 1;
}

/* Interactions */
.doe-panel__sub-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary, #374151);
  margin-bottom: 8px;
}

.doe-panel--dark .doe-panel__sub-title {
  color: #d1d5db;
}

.doe-panel__interaction-item {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 6px;
}

.doe-panel__interaction-label {
  font-size: 11px;
  color: var(--text-secondary, #6b7280);
  min-width: 120px;
}

.doe-panel__interaction-bar-wrapper {
  flex: 1;
  height: 8px;
  background: var(--bg-tertiary, #f3f4f6);
  border-radius: 4px;
  overflow: hidden;
}

.doe-panel--dark .doe-panel__interaction-bar-wrapper {
  background: #374151;
}

.doe-panel__interaction-bar {
  height: 100%;
  border-radius: 4px;
  transition: width 0.3s ease;
}

.doe-panel__interaction-bar.positive {
  background: #16a34a;
}

.doe-panel__interaction-bar.negative {
  background: #dc2626;
}

.doe-panel__interaction-score {
  font-size: 11px;
  font-weight: 500;
  min-width: 50px;
  text-align: right;
}

.doe-panel__interaction-score.positive {
  color: #16a34a;
}

.doe-panel__interaction-score.negative {
  color: #dc2626;
}

/* Scatter controls */
.doe-panel__scatter-controls {
  margin-bottom: 8px;
}

.doe-panel__scatter-point {
  transition: r 0.15s;
}

.doe-panel__scatter-point:hover {
  r: 6;
}
</style>
