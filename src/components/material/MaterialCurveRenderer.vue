/**
 * MaterialCurveRenderer.vue — V4.0-003 可视化材料编辑器
 * 实时绘制应力-应变曲线、S-N曲线、热膨胀曲线
 * 支持拖拽调整参数即时预览
 */
<template>
  <div class="material-curve-renderer">
    <!-- 曲线类型选择 -->
    <div class="curve-tabs">
      <button
        v-for="tab in curveTabs"
        :key="tab.type"
        class="curve-tab"
        :class="{ active: activeCurve === tab.type }"
        @click="activeCurve = tab.type"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- 画布区域 -->
    <div class="canvas-container" ref="containerRef">
      <canvas ref="canvasRef" class="curve-canvas" />

      <!-- 坐标标注 -->
      <div class="axis-label x-axis">{{ xAxisLabel }}</div>
      <div class="axis-label y-axis">{{ yAxisLabel }}</div>

      <!-- 图例 -->
      <div class="legend">
        <div v-for="(item, i) in legendItems" :key="i" class="legend-item">
          <span class="legend-color" :style="{ background: item.color }" />
          <span class="legend-text">{{ item.label }}</span>
        </div>
      </div>
    </div>

    <!-- 参数控制面板 -->
    <div class="param-panel">
      <div class="param-row">
        <label>弹性模量 E</label>
        <div class="slider-group">
          <input
            type="range"
            :min="50"
            :max="400"
            step="1"
            :value="params.elasticModulus"
            @input="updateParam('elasticModulus', ($event.target as HTMLInputElement).valueAsNumber)"
          />
          <input
            type="number"
            :value="params.elasticModulus"
            @input="updateParam('elasticModulus', ($event.target as HTMLInputElement).valueAsNumber)"
          />
          <span class="unit">GPa</span>
        </div>
      </div>

      <div class="param-row">
        <label>泊松比 ν</label>
        <div class="slider-group">
          <input
            type="range"
            :min="0.1"
            :max="0.4"
            step="0.01"
            :value="params.poissonsRatio"
            @input="updateParam('poissonsRatio', ($event.target as HTMLInputElement).valueAsNumber)"
          />
          <input
            type="number"
            :value="params.poissonsRatio"
            @input="updateParam('poissonsRatio', ($event.target as HTMLInputElement).valueAsNumber)"
          />
        </div>
      </div>

      <div class="param-row">
        <label>屈服强度 σy</label>
        <div class="slider-group">
          <input
            type="range"
            :min="100"
            :max="500"
            step="1"
            :value="params.yieldStrength"
            @input="updateParam('yieldStrength', ($event.target as HTMLInputElement).valueAsNumber)"
          />
          <input
            type="number"
            :value="params.yieldStrength"
            @input="updateParam('yieldStrength', ($event.target as HTMLInputElement).valueAsNumber)"
          />
          <span class="unit">MPa</span>
        </div>
      </div>

      <div v-if="activeCurve === 'fatigue'" class="param-row">
        <label>疲劳极限 σa</label>
        <div class="slider-group">
          <input
            type="range"
            :min="50"
            :max="300"
            step="1"
            :value="params.fatigueLimit"
            @input="updateParam('fatigueLimit', ($event.target as HTMLInputElement).valueAsNumber)"
          />
          <input
            type="number"
            :value="params.fatigueLimit"
            @input="updateParam('fatigueLimit', ($event.target as HTMLInputElement).valueAsNumber)"
          />
          <span class="unit">MPa</span>
        </div>
      </div>

      <div v-if="activeCurve === 'thermal'" class="param-row">
        <label>热膨胀系数 α</label>
        <div class="slider-group">
          <input
            type="range"
            :min="5"
            :max="25"
            step="0.1"
            :value="params.thermalExpansion"
            @input="updateParam('thermalExpansion', ($event.target as HTMLInputElement).valueAsNumber)"
          />
          <input
            type="number"
            :value="params.thermalExpansion"
            @input="updateParam('thermalExpansion', ($event.target as HTMLInputElement).valueAsNumber)"
          />
          <span class="unit">×10⁻⁶ /K</span>
        </div>
      </div>

      <div class="param-row">
        <label>硬化指数 n</label>
        <div class="slider-group">
          <input
            type="range"
            :min="0.05"
            :max="0.5"
            step="0.01"
            :value="params.hardeningExponent"
            @input="updateParam('hardeningExponent', ($event.target as HTMLInputElement).valueAsNumber)"
          />
          <input
            type="number"
            :value="params.hardeningExponent"
            @input="updateParam('hardeningExponent', ($event.target as HTMLInputElement).valueAsNumber)"
          />
        </div>
      </div>
    </div>

    <!-- 导出按钮 -->
    <div class="action-bar">
      <button class="btn-export" @click="exportImage">
        📷 导出图片
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'

interface MaterialParams {
  elasticModulus: number   // GPa
  poissonsRatio: number
  yieldStrength: number   // MPa
  fatigueLimit: number    // MPa (for S-N curve)
  thermalExpansion: number // ×10⁻⁶ /K
  hardeningExponent: number
}

const props = defineProps<{
  materialName?: string
  initialParams?: Partial<MaterialParams>
}>()

const emit = defineEmits<{
  'params-change': [params: MaterialParams]
}>()

// 曲线类型
type CurveType = 'stress-strain' | 'fatigue' | 'thermal'

const curveTabs = [
  { type: 'stress-strain' as CurveType, label: '应力-应变曲线' },
  { type: 'fatigue' as CurveType, label: 'S-N 疲劳曲线' },
  { type: 'thermal' as CurveType, label: '热膨胀曲线' }
]

const activeCurve = ref<CurveType>('stress-strain')
const containerRef = ref<HTMLDivElement>()
const canvasRef = ref<HTMLCanvasElement>()

// 材料参数默认值
const defaultParams: MaterialParams = {
  elasticModulus: 206,      // 钢结构
  poissonsRatio: 0.3,
  yieldStrength: 235,     // Q235
  fatigueLimit: 165,      // 疲劳极限约为屈服强度的0.7倍
  thermalExpansion: 12,   // 钢材热膨胀系数
  hardeningExponent: 0.2
}

const params = ref<MaterialParams>({
  ...defaultParams,
  ...props.initialParams
})

// 坐标轴标签
const xAxisLabel = computed(() => {
  switch (activeCurve.value) {
    case 'stress-strain': return '应变 ε (%)'
    case 'fatigue': return '循环次数 N'
    case 'thermal': return '温度 T (°C)'
  }
})

const yAxisLabel = computed(() => {
  switch (activeCurve.value) {
    case 'stress-strain': return '应力 σ (MPa)'
    case 'fatigue': return '应力幅 σa (MPa)'
    case 'thermal': return '应变 ε (×10⁻³)'
  }
})

// 图例
const legendItems = computed(() => {
  switch (activeCurve.value) {
    case 'stress-strain':
      return [
        { label: '应力-应变曲线', color: '#3b82f6' },
        { label: `屈服点 (${params.value.yieldStrength} MPa)`, color: '#f59e0b' }
      ]
    case 'fatigue':
      return [
        { label: 'S-N 曲线', color: '#3b82f6' },
        { label: `疲劳极限 (${params.value.fatigueLimit} MPa)`, color: '#22c55e' }
      ]
    case 'thermal':
      return [
        { label: '热膨胀曲线', color: '#3b82f6' },
        { label: `α = ${params.value.thermalExpansion}×10⁻⁶/K`, color: '#f59e0b' }
      ]
  }
})

function updateParam(key: keyof MaterialParams, value: number) {
  params.value[key] = value
  emit('params-change', params.value)
  drawCurve()
}

// 绘制曲线
function drawCurve() {
  const canvas = canvasRef.value
  if (!canvas) return

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const dpr = window.devicePixelRatio || 1
  const rect = canvas.getBoundingClientRect()

  canvas.width = rect.width * dpr
  canvas.height = rect.height * dpr
  ctx.scale(dpr, dpr)

  const w = rect.width
  const h = rect.height
  const padding = { top: 30, right: 30, bottom: 50, left: 60 }
  const plotW = w - padding.left - padding.right
  const plotH = h - padding.top - padding.bottom

  // 清空画布
  ctx.fillStyle = '#ffffff'
  ctx.fillRect(0, 0, w, h)

  switch (activeCurve.value) {
    case 'stress-strain':
      drawStressStrainCurve(ctx, padding, plotW, plotH)
      break
    case 'fatigue':
      drawFatigueCurve(ctx, padding, plotW, plotH)
      break
    case 'thermal':
      drawThermalCurve(ctx, padding, plotW, plotH)
      break
  }

  // 绘制坐标轴
  drawAxes(ctx, padding, plotW, plotH)
}

function drawStressStrainCurve(ctx: CanvasRenderingContext2D, padding: { top: number; right: number; bottom: number; left: number }, plotW: number, plotH: number) {
  const { elasticModulus, yieldStrength, hardeningExponent } = params.value

  // 转换为 MPa
  const E = elasticModulus * 1000 // MPa
  const sigmaY = yieldStrength
  const n = hardeningExponent

  // 绘制曲线
  ctx.beginPath()
  ctx.strokeStyle = '#3b82f6'
  ctx.lineWidth = 2

  const maxStrain = 0.15 // 15% 最大应变
  const maxStress = E * maxStrain

  for (let i = 0; i <= 200; i++) {
    const epsilon = (i / 200) * maxStrain
    let sigma: number

    if (epsilon <= sigmaY / E) {
      // 线弹性阶段
      sigma = E * epsilon
    } else {
      // 塑性阶段 (幂硬化)
      const epsilonP = epsilon - sigmaY / E
      const K = sigmaY / (sigmaY / E) ** n
      sigma = sigmaY + K * Math.pow(epsilonP, n)
    }

    const x = padding.left + (epsilon / maxStrain) * plotW
    const y = padding.top + plotH - (sigma / (maxStress * 1.2)) * plotH

    if (i === 0) ctx.moveTo(x, y)
    else ctx.lineTo(x, y)
  }
  ctx.stroke()

  // 绘制屈服点
  const yieldX = padding.left + (sigmaY / E / maxStrain) * plotW
  const yieldY = padding.top + plotH - (sigmaY / (maxStress * 1.2)) * plotH

  ctx.beginPath()
  ctx.fillStyle = '#f59e0b'
  ctx.arc(yieldX, yieldY, 5, 0, Math.PI * 2)
  ctx.fill()

  // 屈服点标注
  ctx.fillStyle = '#64748b'
  ctx.font = '12px sans-serif'
  ctx.fillText(`σy = ${sigmaY} MPa`, yieldX + 8, yieldY - 8)
}

function drawFatigueCurve(ctx: CanvasRenderingContext2D, padding: { top: number; right: number; bottom: number; left: number }, plotW: number, plotH: number) {
  const { yieldStrength, fatigueLimit } = params.value

  // Basquin 公式: sigma_a = sigma_f' * (2N)^b
  // 取疲劳极限对应 10^7 次循环
  const sigmaF = yieldStrength * 1.5 // 疲劳强度系数
  const b = -0.1 // 疲劳指数

  ctx.beginPath()
  ctx.strokeStyle = '#3b82f6'
  ctx.lineWidth = 2

  const maxN = 1e8
  const maxSigma = yieldStrength

  for (let i = 0; i <= 200; i++) {
    const N = Math.pow(10, 3 + (i / 200) * 5) // 10^3 到 10^8
    let sigmaA: number

    if (N < 1e4) {
      // 低周疲劳 (超过屈服)
      sigmaA = sigmaF * Math.pow(2 * N, b)
      sigmaA = Math.min(sigmaA, yieldStrength * 0.95)
    } else {
      // 高周疲劳
      sigmaA = sigmaF * Math.pow(2 * N, b)
      sigmaA = Math.max(sigmaA, fatigueLimit)
    }

    const x = padding.left + (Math.log10(N) - 3) / 5 * plotW
    const y = padding.top + plotH - (sigmaA / maxSigma) * plotH

    if (i === 0) ctx.moveTo(x, y)
    else ctx.lineTo(x, y)
  }
  ctx.stroke()

  // 绘制疲劳极限线
  const fatigueY = padding.top + plotH - (fatigueLimit / maxSigma) * plotH
  ctx.beginPath()
  ctx.strokeStyle = '#22c55e'
  ctx.setLineDash([5, 5])
  ctx.moveTo(padding.left, fatigueY)
  ctx.lineTo(padding.left + plotW, fatigueY)
  ctx.stroke()
  ctx.setLineDash([])

  ctx.fillStyle = '#64748b'
  ctx.font = '12px sans-serif'
  ctx.fillText(`σa = ${fatigueLimit} MPa`, padding.left + plotW * 0.7, fatigueY - 8)
}

function drawThermalCurve(ctx: CanvasRenderingContext2D, padding: { top: number; right: number; bottom: number; left: number }, plotW: number, plotH: number) {
  const { thermalExpansion, yieldStrength, elasticModulus } = params.value

  // 热应变: epsilon = alpha * deltaT
  const alpha = thermalExpansion * 1e-6 // /K
  const E = elasticModulus * 1e3 // MPa/K
  const maxTemp = 600 // °C

  ctx.beginPath()
  ctx.strokeStyle = '#3b82f6'
  ctx.lineWidth = 2

  for (let i = 0; i <= 200; i++) {
    const T = (i / 200) * maxTemp
    const epsilon_th = alpha * T
    const sigma = yieldStrength * 0.8 * Math.exp(-T / 200) // 热应力简化模型
    const epsilon_e = sigma / E

    const x = padding.left + (T / maxTemp) * plotW
    const y = padding.top + plotH - (epsilon_th * 1000 / 6) * plotH // 缩放显示

    if (i === 0) ctx.moveTo(x, y)
    else ctx.lineTo(x, y)
  }
  ctx.stroke()

  // 绘制 α 标注
  ctx.fillStyle = '#64748b'
  ctx.font = '12px sans-serif'
  ctx.fillText(`α = ${thermalExpansion}×10⁻⁶/K`, padding.left + plotW * 0.6, padding.top + 30)
}

function drawAxes(ctx: CanvasRenderingContext2D, padding: { top: number; right: number; bottom: number; left: number }, plotW: number, plotH: number) {
  ctx.strokeStyle = '#e2e8f0'
  ctx.lineWidth = 1

  // X 轴
  ctx.beginPath()
  ctx.moveTo(padding.left, padding.top + plotH)
  ctx.lineTo(padding.left + plotW, padding.top + plotH)
  ctx.stroke()

  // Y 轴
  ctx.beginPath()
  ctx.moveTo(padding.left, padding.top)
  ctx.lineTo(padding.left, padding.top + plotH)
  ctx.stroke()

  // 网格线
  ctx.strokeStyle = '#f1f5f9'
  ctx.setLineDash([2, 2])
  for (let i = 1; i <= 4; i++) {
    const x = padding.left + (plotW / 4) * i
    ctx.beginPath()
    ctx.moveTo(x, padding.top)
    ctx.lineTo(x, padding.top + plotH)
    ctx.stroke()

    const y = padding.top + (plotH / 4) * i
    ctx.beginPath()
    ctx.moveTo(padding.left, y)
    ctx.lineTo(padding.left + plotW, y)
    ctx.stroke()
  }
  ctx.setLineDash([])
}

// 导出图片
function exportImage() {
  const canvas = canvasRef.value
  if (!canvas) return

  const link = document.createElement('a')
  link.download = `material_curve_${activeCurve.value}_${Date.now()}.png`
  link.href = canvas.toDataURL('image/png')
  link.click()
}

// 响应式重绘
let resizeObserver: ResizeObserver | null = null

onMounted(() => {
  drawCurve()

  if (containerRef.value) {
    resizeObserver = new ResizeObserver(() => {
      drawCurve()
    })
    resizeObserver.observe(containerRef.value)
  }
})

onUnmounted(() => {
  resizeObserver?.disconnect()
})

// 监听曲线类型变化重绘
watch(activeCurve, () => {
  drawCurve()
})

// 监听参数变化重绘
watch(params, () => {
  drawCurve()
}, { deep: true })
</script>

<style scoped>
.material-curve-renderer {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 16px;
  background: white;
  border-radius: 12px;
}

.curve-tabs {
  display: flex;
  gap: 8px;
}

.curve-tab {
  padding: 8px 16px;
  border-radius: 8px;
  border: 1px solid var(--border-color, #e2e8f0);
  background: white;
  color: var(--text-secondary, #64748b);
  cursor: pointer;
  transition: all 0.2s;
}

.curve-tab.active {
  background: var(--primary-color, #3b82f6);
  color: white;
  border-color: var(--primary-color, #3b82f6);
}

.canvas-container {
  position: relative;
  height: 300px;
  background: #fafafa;
  border-radius: 8px;
}

.curve-canvas {
  width: 100%;
  height: 100%;
}

.axis-label {
  position: absolute;
  font-size: 12px;
  color: var(--text-muted, #94a3b8);
}

.axis-label.x-axis {
  bottom: 8px;
  left: 50%;
  transform: translateX(-50%);
}

.axis-label.y-axis {
  left: 8px;
  top: 50%;
  transform: translateY(-50%) rotate(-90deg);
  transform-origin: center;
  white-space: nowrap;
}

.legend {
  position: absolute;
  top: 8px;
  right: 8px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  background: rgba(255, 255, 255, 0.9);
  padding: 8px;
  border-radius: 6px;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
}

.legend-color {
  width: 12px;
  height: 3px;
  border-radius: 2px;
}

.legend-text {
  color: var(--text-secondary, #64748b);
}

.param-panel {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 16px;
  background: var(--bg-elevated, #f8fafc);
  border-radius: 8px;
}

.param-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.param-row label {
  font-size: 13px;
  color: var(--text-secondary, #64748b);
  min-width: 80px;
}

.slider-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.slider-group input[type="range"] {
  width: 120px;
}

.slider-group input[type="number"] {
  width: 60px;
  padding: 4px 8px;
  border: 1px solid #e2e8f0;
  border-radius: 4px;
  text-align: right;
  font-size: 13px;
}

.unit {
  font-size: 12px;
  color: var(--text-muted, #94a3b8);
}

.action-bar {
  display: flex;
  justify-content: flex-end;
}

.btn-export {
  padding: 8px 16px;
  border-radius: 8px;
  border: 1px solid var(--border-color, #e2e8f0);
  background: white;
  color: var(--text-secondary, #64748b);
  cursor: pointer;
  transition: all 0.2s;
}

.btn-export:hover {
  background: var(--bg-elevated, #f1f5f9);
}
</style>