<template>
  <div
    class="convergence-chart"
    ref="chartContainer"
    @mousedown="onMouseDown"
    @mousemove="onMouseMove"
    @mouseup="onMouseUp"
    @mouseleave="onMouseUp"
    @wheel.prevent="onWheel"
    @dblclick="resetView"
  >
    <!-- SVG Chart -->
    <svg
      :width="svgWidth"
      :height="svgHeight"
      :viewBox="`0 0 ${svgWidth} ${svgHeight}`"
      class="w-full h-full"
    >
      <defs>
        <linearGradient id="complianceGrad" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" style="stop-color: var(--primary, #ec4899); stop-opacity: 0.3" />
          <stop offset="100%" style="stop-color: var(--primary, #ec4899); stop-opacity: 0.02" />
        </linearGradient>
        <linearGradient id="volumeGrad" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" style="stop-color: #3b82f6; stop-opacity: 0.2" />
          <stop offset="100%" style="stop-color: #3b82f6; stop-opacity: 0.02" />
        </linearGradient>
        <clipPath :id="clipId">
          <rect :x="plotArea.x" :y="plotArea.y" :width="plotArea.width" :height="plotArea.height" />
        </clipPath>
      </defs>

      <!-- Background -->
      <rect
        :x="plotArea.x"
        :y="plotArea.y"
        :width="plotArea.width"
        :height="plotArea.height"
        class="chart-bg"
      />

      <!-- Grid lines -->
      <g :clip-path="`url(#${clipId})`">
        <!-- Horizontal grid lines -->
        <line
          v-for="(y, idx) in gridLinesY"
          :key="'gy-' + idx"
          :x1="plotArea.x"
          :y1="y"
          :x2="plotArea.x + plotArea.width"
          :y2="y"
          class="grid-line"
        />
        <!-- Vertical grid lines -->
        <line
          v-for="(x, idx) in gridLinesX"
          :key="'gx-' + idx"
          :x1="x"
          :y1="plotArea.y"
          :x2="x"
          :y2="plotArea.y + plotArea.height"
          class="grid-line"
        />

        <!-- Volume constraint line -->
        <line
          v-if="showVolumeConstraint && volumeConstraintValue != null"
          :x1="plotArea.x"
          :x2="plotArea.x + plotArea.width"
          :y1="volumeConstraintY"
          :y2="volumeConstraintY"
          stroke="#3b82f6"
          stroke-width="1.5"
          stroke-dasharray="6,4"
          opacity="0.7"
        />
        <text
          v-if="showVolumeConstraint && volumeConstraintValue != null"
          :x="plotArea.x + plotArea.width - 4"
          :y="volumeConstraintY - 4"
          fill="#3b82f6"
          font-size="10"
          text-anchor="end"
          class="chart-label"
        >
          Vol: {{ (volumeConstraintValue * 100).toFixed(1) }}%
        </text>

        <!-- Displacement constraint line -->
        <line
          v-if="showDisplacementConstraint && displacementConstraintValue != null"
          :x1="plotArea.x"
          :x2="plotArea.x + plotArea.width"
          :y1="displacementConstraintY"
          :y2="displacementConstraintY"
          stroke="#f59e0b"
          stroke-width="1.5"
          stroke-dasharray="6,4"
          opacity="0.7"
        />
        <text
          v-if="showDisplacementConstraint && displacementConstraintValue != null"
          :x="plotArea.x + plotArea.width - 4"
          :y="displacementConstraintY - 4"
          fill="#f59e0b"
          font-size="10"
          text-anchor="end"
          class="chart-label"
        >
          Disp: {{ displacementConstraintValue.toFixed(4) }}
        </text>

        <!-- Volume fraction area fill -->
        <path
          v-if="showVolume && volumeData.length > 1"
          :d="volumeAreaPath"
          fill="url(#volumeGrad)"
          :clip-path="`url(#${clipId})`"
        />
        <!-- Volume fraction line -->
        <polyline
          v-if="showVolume && volumeData.length > 1"
          :points="volumeLinePoints"
          fill="none"
          stroke="#3b82f6"
          stroke-width="2"
          stroke-linejoin="round"
          :clip-path="`url(#${clipId})`"
        />

        <!-- Compliance area fill -->
        <path
          v-if="showCompliance && complianceData.length > 1"
          :d="complianceAreaPath"
          fill="url(#complianceGrad)"
          :clip-path="`url(#${clipId})`"
        />
        <!-- Compliance line -->
        <polyline
          v-if="showCompliance && complianceData.length > 1"
          :points="complianceLinePoints"
          fill="none"
          stroke="var(--primary, #ec4899)"
          stroke-width="2"
          stroke-linejoin="round"
          :clip-path="`url(#${clipId})`"
        />

        <!-- Data points -->
        <circle
          v-for="(pt, idx) in complianceData"
          :key="'cp-' + idx"
          :cx="pt.x"
          :cy="pt.y"
          r="3"
          fill="var(--primary, #ec4899)"
          stroke="var(--bg-surface, #fff)"
          stroke-width="1.5"
          :clip-path="`url(#${clipId})`"
          style="cursor: pointer"
          @mouseenter="showTooltip($event, idx, 'compliance')"
          @mouseleave="hideTooltip"
        />
        <circle
          v-for="(pt, idx) in volumeData"
          :key="'vp-' + idx"
          :cx="pt.x"
          :cy="pt.y"
          r="3"
          fill="#3b82f6"
          stroke="var(--bg-surface, #fff)"
          stroke-width="1.5"
          :clip-path="`url(#${clipId})`"
          style="cursor: pointer"
          @mouseenter="showTooltip($event, idx, 'volume')"
          @mouseleave="hideTooltip"
        />
      </g>

      <!-- Axes -->
      <line
        :x1="plotArea.x"
        :y1="plotArea.y + plotArea.height"
        :x2="plotArea.x + plotArea.width"
        :y2="plotArea.y + plotArea.height"
        stroke="var(--border-subtle, #e5e7eb)"
        stroke-width="1"
      />
      <line
        :x1="plotArea.x"
        :y1="plotArea.y"
        :x2="plotArea.x"
        :y2="plotArea.y + plotArea.height"
        stroke="var(--border-subtle, #e5e7eb)"
        stroke-width="1"
      />

      <!-- Y-axis labels -->
      <text
        v-for="(label, idx) in yAxisLabels"
        :key="'yl-' + idx"
        :x="plotArea.x - 6"
        :y="label.y + 3"
        fill="var(--text-muted, #9ca3af)"
        font-size="10"
        text-anchor="end"
        class="chart-label"
      >
        {{ label.text }}
      </text>

      <!-- X-axis labels -->
      <text
        v-for="(label, idx) in xAxisLabels"
        :key="'xl-' + idx"
        :x="label.x"
        :y="plotArea.y + plotArea.height + 16"
        fill="var(--text-muted, #9ca3af)"
        font-size="10"
        text-anchor="middle"
        class="chart-label"
      >
        {{ label.text }}
      </text>

      <!-- Axis titles -->
      <text
        :x="plotArea.x + plotArea.width / 2"
        :y="plotArea.y + plotArea.height + 32"
        fill="var(--text-secondary, #6b7280)"
        font-size="11"
        text-anchor="middle"
        class="chart-label"
      >
        Iterations
      </text>
      <text
        :x="12"
        :y="plotArea.y + plotArea.height / 2"
        fill="var(--text-secondary, #6b7280)"
        font-size="11"
        text-anchor="middle"
        class="chart-label"
        transform="rotate(-90, 12, 0)"
      >
        {{ yAxisTitle }}
      </text>
    </svg>

    <!-- Tooltip -->
    <div
      v-if="tooltip.visible"
      class="chart-tooltip"
      :style="{ left: tooltip.x + 'px', top: tooltip.y + 'px' }"
    >
      <div class="tooltip-title">Iteration {{ tooltip.iteration }}</div>
      <div v-if="tooltip.type === 'compliance'" class="tooltip-row">
        <span class="tooltip-dot" style="background: var(--primary, #ec4899)"></span>
        Compliance: {{ tooltip.value }}
      </div>
      <div v-if="tooltip.type === 'volume'" class="tooltip-row">
        <span class="tooltip-dot" style="background: #3b82f6"></span>
        Volume: {{ tooltip.value }}
      </div>
    </div>

    <!-- Legend -->
    <div class="chart-legend">
      <label v-if="showCompliance" class="legend-item">
        <input type="checkbox" v-model="showCompliance" class="sr-only" />
        <span class="legend-line" style="background: var(--primary, #ec4899)"></span>
        Compliance
      </label>
      <label v-if="showVolume" class="legend-item">
        <input type="checkbox" v-model="showVolume" class="sr-only" />
        <span class="legend-line" style="background: #3b82f6"></span>
        Volume Fraction
      </label>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'

export interface ChartDataPoint {
  iteration: number
  compliance?: number
  volume?: number
}

const props = withDefaults(defineProps<{
  data: ChartDataPoint[]
  showVolumeConstraint?: boolean
  volumeConstraintValue?: number | null
  showDisplacementConstraint?: boolean
  displacementConstraintValue?: number | null
  yAxisTitle?: string
  height?: number
}>(), {
  showVolumeConstraint: false,
  volumeConstraintValue: null,
  showDisplacementConstraint: false,
  displacementConstraintValue: null,
  yAxisTitle: 'Objective Value',
  height: 220,
})

const chartContainer = ref<HTMLDivElement | null>(null)
const clipId = 'chart-clip-' + Math.random().toString(36).slice(2, 8)
const svgWidth = ref(600)
const svgHeight = ref(props.height)

const showCompliance = ref(true)
const showVolume = ref(true)

// View state for zoom/pan
const viewState = ref({
  xMin: 0,
  xMax: 100,
  yMin: 0,
  yMax: 100,
})

const isDragging = ref(false)
const dragStart = ref({ x: 0, y: 0, xMin: 0, xMax: 0, yMin: 0, yMax: 0 })

const tooltip = ref({
  visible: false,
  x: 0,
  y: 0,
  iteration: 0,
  type: 'compliance' as 'compliance' | 'volume',
  value: '',
})

const padding = { top: 10, right: 20, bottom: 40, left: 55 }

const plotArea = computed(() => ({
  x: padding.left,
  y: padding.top,
  width: svgWidth.value - padding.left - padding.right,
  height: svgHeight.value - padding.top - padding.bottom,
}))

// Compute data ranges
const dataRange = computed(() => {
  if (props.data.length === 0) {
    return { xMin: 0, xMax: 10, yMin: 0, yMax: 1 }
  }

  let xMin = Infinity, xMax = -Infinity
  let yMin = Infinity, yMax = -Infinity

  for (const pt of props.data) {
    if (pt.iteration < xMin) xMin = pt.iteration
    if (pt.iteration > xMax) xMax = pt.iteration
    if (pt.compliance != null) {
      if (pt.compliance < yMin) yMin = pt.compliance
      if (pt.compliance > yMax) yMax = pt.compliance
    }
    if (pt.volume != null) {
      if (pt.volume < yMin) yMin = pt.volume
      if (pt.volume > yMax) yMax = pt.volume
    }
  }

  // Add margins
  const xRange = xMax - xMin || 10
  const yRange = yMax - yMin || 1
  xMin = Math.max(0, xMin - xRange * 0.05)
  xMax = xMax + xRange * 0.05
  yMin = Math.max(0, yMin - yRange * 0.1)
  yMax = yMax + yRange * 0.1

  return { xMin, xMax, yMin, yMax }
})

// Map data to SVG coordinates
function mapX(iteration: number): number {
  const { xMin, xMax } = viewState.value
  const range = xMax - xMin || 1
  return plotArea.value.x + ((iteration - xMin) / range) * plotArea.value.width
}

function mapY(value: number): number {
  const { yMin, yMax } = viewState.value
  const range = yMax - yMin || 1
  return plotArea.value.y + plotArea.value.height - ((value - yMin) / range) * plotArea.value.height
}

function unmapY(y: number): number {
  const { yMin, yMax } = viewState.value
  const range = yMax - yMin || 1
  return yMin + ((plotArea.value.y + plotArea.value.height - y) / plotArea.value.height) * range
}

const complianceData = computed(() =>
  props.data
    .filter(d => d.compliance != null)
    .map(d => ({ x: mapX(d.iteration), y: mapY(d.compliance!), iteration: d.iteration, value: d.compliance! }))
)

const volumeData = computed(() =>
  props.data
    .filter(d => d.volume != null)
    .map(d => ({ x: mapX(d.iteration), y: mapY(d.volume!), iteration: d.iteration, value: d.volume! }))
)

const complianceLinePoints = computed(() =>
  complianceData.value.map(p => `${p.x},${p.y}`).join(' ')
)

const volumeLinePoints = computed(() =>
  volumeData.value.map(p => `${p.x},${p.y}`).join(' ')
)

const complianceAreaPath = computed(() => {
  const pts = complianceData.value
  if (pts.length < 2) return ''
  const baseline = plotArea.value.y + plotArea.value.height
  let d = `M ${pts[0].x},${baseline} L ${pts[0].x},${pts[0].y}`
  for (let i = 1; i < pts.length; i++) {
    d += ` L ${pts[i].x},${pts[i].y}`
  }
  d += ` L ${pts[pts.length - 1].x},${baseline} Z`
  return d
})

const volumeAreaPath = computed(() => {
  const pts = volumeData.value
  if (pts.length < 2) return ''
  const baseline = plotArea.value.y + plotArea.value.height
  let d = `M ${pts[0].x},${baseline} L ${pts[0].x},${pts[0].y}`
  for (let i = 1; i < pts.length; i++) {
    d += ` L ${pts[i].x},${pts[i].y}`
  }
  d += ` L ${pts[pts.length - 1].x},${baseline} Z`
  return d
})

const volumeConstraintY = computed(() => {
  if (props.volumeConstraintValue == null) return 0
  return mapY(props.volumeConstraintValue)
})

const displacementConstraintY = computed(() => {
  if (props.displacementConstraintValue == null) return 0
  return mapY(props.displacementConstraintValue)
})

// Grid lines
const gridLinesY = computed(() => {
  const { yMin, yMax } = viewState.value
  const range = yMax - yMin
  const step = niceStep(range, 5)
  const lines: number[] = []
  let y = Math.ceil(yMin / step) * step
  while (y <= yMax) {
    lines.push(mapY(y))
    y += step
  }
  return lines
})

const gridLinesX = computed(() => {
  const { xMin, xMax } = viewState.value
  const range = xMax - xMin
  const step = niceStep(range, 6)
  const lines: number[] = []
  let x = Math.ceil(xMin / step) * step
  while (x <= xMax) {
    lines.push(mapX(x))
    x += step
  }
  return lines
})

const yAxisLabels = computed(() => {
  const { yMin, yMax } = viewState.value
  const range = yMax - yMin
  const step = niceStep(range, 5)
  const labels: Array<{ y: number; text: string }> = []
  let y = Math.ceil(yMin / step) * step
  while (y <= yMax) {
    labels.push({ y: mapY(y), text: formatNumber(y) })
    y += step
  }
  return labels
})

const xAxisLabels = computed(() => {
  const { xMin, xMax } = viewState.value
  const range = xMax - xMin
  const step = niceStep(range, 6)
  const labels: Array<{ x: number; text: string }> = []
  let x = Math.ceil(xMin / step) * step
  while (x <= xMax) {
    labels.push({ x: mapX(x), text: Math.round(x).toString() })
    x += step
  }
  return labels
})

function niceStep(range: number, targetSteps: number): number {
  const rough = range / targetSteps
  const pow = Math.pow(10, Math.floor(Math.log10(rough)))
  const normalized = rough / pow
  let nice: number
  if (normalized <= 1.5) nice = 1
  else if (normalized <= 3) nice = 2
  else if (normalized <= 7) nice = 5
  else nice = 10
  return nice * pow
}

function formatNumber(n: number): string {
  if (Math.abs(n) < 0.001) return n.toExponential(1)
  if (Math.abs(n) >= 10000) return n.toExponential(1)
  return n.toFixed(2)
}

// Tooltip
function showTooltip(event: MouseEvent, idx: number, type: 'compliance' | 'volume') {
  const rect = chartContainer.value?.getBoundingClientRect()
  if (!rect) return
  const pt = type === 'compliance' ? complianceData.value[idx] : volumeData.value[idx]
  if (!pt) return
  tooltip.value = {
    visible: true,
    x: event.clientX - rect.left + 12,
    y: event.clientY - rect.top - 10,
    iteration: pt.iteration,
    type,
    value: type === 'compliance' ? pt.value.toFixed(4) : (pt.value * 100).toFixed(1) + '%',
  }
}

function hideTooltip() {
  tooltip.value.visible = false
}

// Zoom & Pan
function onWheel(event: WheelEvent) {
  const factor = event.deltaY > 0 ? 1.1 : 0.9
  const { xMin, xMax, yMin, yMax } = viewState.value
  const xRange = xMax - xMin
  const yRange = yMax - yMin
  const newXRange = xRange * factor
  const newYRange = yRange * factor
  const xCenter = (xMin + xMax) / 2
  const yCenter = (yMin + yMax) / 2
  viewState.value = {
    xMin: xCenter - newXRange / 2,
    xMax: xCenter + newXRange / 2,
    yMin: yCenter - newYRange / 2,
    yMax: yCenter + newYRange / 2,
  }
}

function onMouseDown(event: MouseEvent) {
  isDragging.value = true
  dragStart.value = {
    x: event.clientX,
    y: event.clientY,
    ...viewState.value,
  }
}

function onMouseMove(event: MouseEvent) {
  if (!isDragging.value) return
  const dx = event.clientX - dragStart.value.x
  const dy = event.clientY - dragStart.value.y
  const xRange = dragStart.value.xMax - dragStart.value.xMin
  const yRange = dragStart.value.yMax - dragStart.value.yMin
  const xShift = -(dx / svgWidth.value) * xRange
  const yShift = (dy / svgHeight.value) * yRange
  viewState.value = {
    xMin: dragStart.value.xMin + xShift,
    xMax: dragStart.value.xMax + xShift,
    yMin: dragStart.value.yMin + yShift,
    yMax: dragStart.value.yMax + yShift,
  }
}

function onMouseUp() {
  isDragging.value = false
}

function resetView() {
  viewState.value = { ...dataRange.value }
}

// Resize observer
let resizeObserver: ResizeObserver | null = null

function updateSize() {
  if (chartContainer.value) {
    svgWidth.value = chartContainer.value.clientWidth
    svgHeight.value = props.height
  }
}

onMounted(() => {
  updateSize()
  viewState.value = { ...dataRange.value }
  if (chartContainer.value) {
    resizeObserver = new ResizeObserver(updateSize)
    resizeObserver.observe(chartContainer.value)
  }
})

onUnmounted(() => {
  resizeObserver?.disconnect()
})

// Auto-fit when data changes
watch(() => props.data, () => {
  if (!isDragging.value) {
    viewState.value = { ...dataRange.value }
  }
}, { deep: true })
</script>

<style scoped>
.convergence-chart {
  position: relative;
  width: 100%;
  height: 100%;
  user-select: none;
  cursor: grab;
}

.convergence-chart:active {
  cursor: grabbing;
}

.chart-bg {
  fill: var(--bg-base, #f9fafb);
  rx: 2;
}

.dark .chart-bg {
  fill: var(--bg-base, #111827);
}

.grid-line {
  stroke: var(--border-subtle, #e5e7eb);
  stroke-width: 0.5;
  opacity: 0.6;
}

.dark .grid-line {
  stroke: var(--border-subtle, #374151);
}

.chart-label {
  fill: var(--text-muted, #9ca3af);
}

.chart-tooltip {
  position: absolute;
  background: var(--bg-surface, #fff);
  border: 1px solid var(--border-subtle, #e5e7eb);
  border-radius: 6px;
  padding: 6px 10px;
  font-size: 12px;
  pointer-events: none;
  z-index: 10;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.12);
}

.dark .chart-tooltip {
  background: var(--bg-surface, #1f2937);
  border-color: var(--border-subtle, #374151);
}

.tooltip-title {
  font-weight: 600;
  color: var(--text-primary, #111827);
  margin-bottom: 4px;
}

.tooltip-row {
  display: flex;
  align-items: center;
  gap: 6px;
  color: var(--text-secondary, #6b7280);
}

.tooltip-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.chart-legend {
  position: absolute;
  top: 8px;
  right: 8px;
  display: flex;
  gap: 12px;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
  color: var(--text-secondary, #6b7280);
  cursor: pointer;
}

.legend-line {
  width: 16px;
  height: 2px;
  border-radius: 1px;
}
</style>
