/**
 * MeshQualityHeatmap.vue — V4.0-005 网格质量热力图
 * 显示 Jacobian、Aspect Ratio、Skewness 质量热力图
 * 用红黄绿标注问题单元
 */
<template>
  <div class="mesh-quality-heatmap">
    <!-- 质量指标选择 -->
    <div class="quality-tabs">
      <button
        v-for="tab in metricTabs"
        :key="tab.key"
        class="quality-tab"
        :class="{ active: activeMetric === tab.key }"
        @click="activeMetric = tab.key"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- 3D 视图区域 -->
    <div class="viewport-container" ref="viewportRef">
      <div v-if="!hasMesh" class="empty-state">
        <span class="empty-icon">🔲</span>
        <p>暂无网格数据</p>
        <p class="empty-hint">请先生成网格后再查看质量热力图</p>
      </div>

      <canvas v-show="hasMesh" ref="canvasRef" class="heatmap-canvas" />
    </div>

    <!-- 颜色图例 -->
    <div class="color-legend">
      <div class="legend-bar">
        <div class="gradient-bar" :style="gradientStyle" />
        <div class="legend-labels">
          <span>{{ metricConfig.minLabel }}</span>
          <span>{{ metricConfig.maxLabel }}</span>
        </div>
      </div>
      <div class="legend-description">
        <span class="quality-bad">■ 不合格</span>
        <span class="quality-warning">■ 警告</span>
        <span class="quality-good">■ 合格</span>
      </div>
    </div>

    <!-- 质量统计 -->
    <div class="quality-stats">
      <div class="stat-card">
        <span class="stat-value bad">{{ stats.badCount }}</span>
        <span class="stat-label">不合格</span>
      </div>
      <div class="stat-card">
        <span class="stat-value warning">{{ stats.warningCount }}</span>
        <span class="stat-label">警告</span>
      </div>
      <div class="stat-card">
        <span class="stat-value good">{{ stats.goodCount }}</span>
        <span class="stat-label">合格</span>
      </div>
      <div class="stat-card highlight">
        <span class="stat-value">{{ stats.averageQuality.toFixed(2) }}</span>
        <span class="stat-label">平均质量</span>
      </div>
    </div>

    <!-- 问题单元列表 -->
    <div v-if="problemElements.length > 0" class="problem-list">
      <h4>需要关注的单元 ({{ problemElements.length }})</h4>
      <div class="problem-items">
        <div
          v-for="elem in problemElements.slice(0, 10)"
          :key="elem.id"
          class="problem-item"
          @click="highlightElement(elem.id)"
        >
          <span class="elem-id">单元 #{{ elem.id }}</span>
          <span class="elem-quality" :class="getQualityClass(elem.quality)">
            {{ elem.quality.toFixed(3) }}
          </span>
          <span class="elem-metric">{{ activeMetric }}</span>
        </div>
      </div>
      <button v-if="problemElements.length > 10" class="show-more">
        显示更多 ({{ problemElements.length - 10 }})
      </button>
    </div>

    <!-- 导出按钮 -->
    <div class="action-bar">
      <button class="btn-export" @click="exportReport">
        📊 导出质检报告
      </button>
      <button class="btn-refresh" @click="refreshMesh">
        🔄 重新计算
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'

// 质量指标类型
type MetricKey = 'jacobian' | 'aspect_ratio' | 'skewness' | 'quality'

interface MeshElement {
  id: number
  type: string
  quality: number
  nodes: number[]
}

interface MeshData {
  nodes: Array<{ id: number; x: number; y: number; z: number }>
  elements: MeshElement[]
}

const props = defineProps<{
  mesh?: MeshData | null
}>()

const emit = defineEmits<{
  'element-select': [elementId: number]
  'export-report': []
}>()

const viewportRef = ref<HTMLDivElement>()
const canvasRef = ref<HTMLCanvasElement>()
const activeMetric = ref<MetricKey>('quality')

// 质量指标配置
const metricTabs = [
  { key: 'quality' as MetricKey, label: '综合质量' },
  { key: 'jacobian' as MetricKey, label: '雅可比' },
  { key: 'aspect_ratio' as MetricKey, label: '长宽比' },
  { key: 'skewness' as MetricKey, label: '歪斜度' }
]

const metricConfig = computed(() => {
  switch (activeMetric.value) {
    case 'jacobian':
      return {
        minLabel: '0 (翻转)',
        maxLabel: '1 (理想)',
        getQuality: (e: MeshElement) => e.quality,
        thresholds: { good: 0.7, warning: 0.3 }
      }
    case 'aspect_ratio':
      return {
        minLabel: '1 (理想)',
        maxLabel: '>10 (畸变)',
        getQuality: (e: MeshElement) => 1 / Math.min(e.quality, 10),
        thresholds: { good: 0.7, warning: 0.3 }
      }
    case 'skewness':
      return {
        minLabel: '0 (理想)',
        maxLabel: '1 (最大畸变)',
        getQuality: (e: MeshElement) => 1 - e.quality,
        thresholds: { good: 0.75, warning: 0.5 }
      }
    default:
      return {
        minLabel: '差',
        maxLabel: '优',
        getQuality: (e: MeshElement) => e.quality,
        thresholds: { good: 0.7, warning: 0.4 }
      }
  }
})

// 颜色渐变
const gradientStyle = computed(() => {
  const metric = activeMetric.value
  if (metric === 'jacobian' || metric === 'quality') {
    return { background: 'linear-gradient(to right, #ef4444, #f59e0b, #22c55e)' }
  } else if (metric === 'aspect_ratio') {
    return { background: 'linear-gradient(to right, #22c55e, #f59e0b, #ef4444)' }
  } else {
    return { background: 'linear-gradient(to right, #22c55e, #f59e0b, #ef4444)' }
  }
})

// 是否有网格数据
const hasMesh = computed(() => props.mesh && props.mesh.elements.length > 0)

// 统计数据
const stats = computed(() => {
  if (!hasMesh.value) {
    return { badCount: 0, warningCount: 0, goodCount: 0, averageQuality: 0 }
  }

  const { thresholds } = metricConfig.value
  const elements = props.mesh!.elements
  let badCount = 0
  let warningCount = 0
  let goodCount = 0
  let totalQuality = 0

  for (const elem of elements) {
    const quality = metricConfig.value.getQuality(elem)
    totalQuality += quality
    if (quality < thresholds.warning) badCount++
    else if (quality < thresholds.good) warningCount++
    else goodCount++
  }

  return {
    badCount,
    warningCount,
    goodCount,
    averageQuality: totalQuality / elements.length
  }
})

// 问题单元列表
const problemElements = computed(() => {
  if (!hasMesh.value) return []

  const { thresholds } = metricConfig.value
  return props.mesh!.elements
    .filter(elem => metricConfig.value.getQuality(elem) < thresholds.good)
    .sort((a, b) => metricConfig.value.getQuality(a) - metricConfig.value.getQuality(b))
})

// 质量等级颜色类
function getQualityClass(quality: number): string {
  const { thresholds } = metricConfig.value
  if (quality < thresholds.warning) return 'bad'
  if (quality < thresholds.good) return 'warning'
  return 'good'
}

// 获取元素颜色
function getElementColor(quality: number): string {
  const { thresholds } = metricConfig.value
  if (quality < thresholds.warning) return '#ef4444' // 红色
  if (quality < thresholds.good) return '#f59e0b'    // 黄色
  return '#22c55e'                                  // 绿色
}

// 高亮选中单元
function highlightElement(elementId: number) {
  emit('element-select', elementId)
}

// 导出报告
function exportReport() {
  emit('export-report')

  // 生成文本报告
  const report = generateReport()
  const blob = new Blob([report], { type: 'text/plain' })
  const link = document.createElement('a')
  link.download = `mesh_quality_report_${Date.now()}.txt`
  link.href = URL.createObjectURL(blob)
  link.click()
}

// 生成报告文本
function generateReport(): string {
  const lines: string[] = [
    '====================================',
    '        网格质量检测报告',
    '====================================',
    '',
    `生成时间: ${new Date().toLocaleString('zh-CN')}`,
    `质量指标: ${activeMetric.value}`,
    '',
    '【统计摘要】',
    `  总单元数: ${props.mesh?.elements.length ?? 0}`,
    `  合格单元: ${stats.value.goodCount} (${((stats.value.goodCount / (props.mesh?.elements.length ?? 1)) * 100).toFixed(1)}%)`,
    `  警告单元: ${stats.value.warningCount} (${((stats.value.warningCount / (props.mesh?.elements.length ?? 1)) * 100).toFixed(1)}%)`,
    `  不合格单元: ${stats.value.badCount} (${((stats.value.badCount / (props.mesh?.elements.length ?? 1)) * 100).toFixed(1)}%)`,
    `  平均质量: ${stats.value.averageQuality.toFixed(3)}`,
    '',
    '【阈值标准】',
    `  合格阈值: >= ${metricConfig.value.thresholds.good}`,
    `  警告阈值: >= ${metricConfig.value.thresholds.warning}`,
    '',
  ]

  if (problemElements.value.length > 0) {
    lines.push('【问题单元列表 (前20个)】')
    for (const elem of problemElements.value.slice(0, 20)) {
      lines.push(`  单元 #${elem.id}: ${elem.type}, 质量=${elem.quality.toFixed(3)}`)
    }
  }

  lines.push('')
  lines.push('====================================')

  return lines.join('\n')
}

// 刷新/重新计算
function refreshMesh() {
  // TODO: 触发网格重新计算
  console.log('Refresh mesh quality calculation')
}

// 绘制热力图
function drawHeatmap() {
  const canvas = canvasRef.value
  if (!canvas || !hasMesh.value) return

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const dpr = window.devicePixelRatio || 1
  const rect = canvas.getBoundingClientRect()

  canvas.width = rect.width * dpr
  canvas.height = rect.height * dpr
  ctx.scale(dpr, dpr)

  const w = rect.width
  const h = rect.height

  // 清空画布
  ctx.fillStyle = '#1e293b'
  ctx.fillRect(0, 0, w, h)

  // 获取网格范围
  const nodes = props.mesh!.nodes
  const elements = props.mesh!.elements

  const minX = Math.min(...nodes.map(n => n.x))
  const maxX = Math.max(...nodes.map(n => n.x))
  const minY = Math.min(...nodes.map(n => n.y))
  const maxY = Math.max(...nodes.map(n => n.y))

  const rangeX = maxX - minX || 1
  const rangeY = maxY - minY || 1

  // 缩放到画布
  const scale = Math.min(w / rangeX, h / rangeY) * 0.9
  const offsetX = w / 2 - ((minX + maxX) / 2) * scale
  const offsetY = h / 2 - ((minY + maxY) / 2) * scale

  // 绘制单元
  for (const elem of elements) {
    const quality = metricConfig.value.getQuality(elem)
    ctx.fillStyle = getElementColor(quality)
    ctx.strokeStyle = 'rgba(255,255,255,0.2)'
    ctx.lineWidth = 0.5

    if (elem.type === 'tet4' || elem.type === 'tet10') {
      // 四面体 - 绘制三个面
      drawTetrahedron(ctx, elem, scale, offsetX, offsetY)
    } else if (elem.type === 'hex8' || elem.type === 'hex20') {
      // 六面体 - 绘制为四边形
      drawHexahedron(ctx, elem, scale, offsetX, offsetY)
    }
  }

  // 绘制节点
  for (const node of nodes) {
    const x = node.x * scale + offsetX
    const y = h - (node.y * scale + offsetY)

    ctx.beginPath()
    ctx.fillStyle = 'rgba(255,255,255,0.5)'
    ctx.arc(x, y, 1, 0, Math.PI * 2)
    ctx.fill()
  }
}

// 绘制四面体单元
function drawTetrahedron(
  ctx: CanvasRenderingContext2D,
  elem: MeshElement,
  scale: number,
  offsetX: number,
  offsetY: number
) {
  const h = canvasRef.value?.height ?? 0
  const nodeCoords = elem.nodes.slice(0, 4).map((nodeId, i) => {
    const node = props.mesh!.nodes.find(n => n.id === nodeId) ?? props.mesh!.nodes[i]
    return {
      x: node.x * scale + offsetX,
      y: h - (node.y * scale + offsetY)
    }
  })

  if (nodeCoords.length >= 4) {
    ctx.beginPath()
    ctx.moveTo(nodeCoords[0].x, nodeCoords[0].y)
    for (let i = 1; i < 4; i++) {
      ctx.lineTo(nodeCoords[i].x, nodeCoords[i].y)
    }
    ctx.closePath()
    ctx.fill()
    ctx.stroke()
  }
}

// 绘制六面体单元（简化为四边形）
function drawHexahedron(
  ctx: CanvasRenderingContext2D,
  elem: MeshElement,
  scale: number,
  offsetX: number,
  offsetY: number
) {
  const h = canvasRef.value?.height ?? 0
  const nodeCoords = elem.nodes.slice(0, 4).map((nodeId, i) => {
    const node = props.mesh!.nodes.find(n => n.id === nodeId) ?? props.mesh!.nodes[i]
    return {
      x: node.x * scale + offsetX,
      y: h - (node.y * scale + offsetY)
    }
  })

  if (nodeCoords.length >= 4) {
    ctx.beginPath()
    ctx.moveTo(nodeCoords[0].x, nodeCoords[0].y)
    for (let i = 1; i < 4; i++) {
      ctx.lineTo(nodeCoords[i].x, nodeCoords[i].y)
    }
    ctx.closePath()
    ctx.fill()
    ctx.stroke()
  }
}

// 监听数据变化重绘
watch(() => props.mesh, () => {
  drawHeatmap()
}, { deep: true })

watch(activeMetric, () => {
  drawHeatmap()
})

onMounted(() => {
  if (hasMesh.value) {
    drawHeatmap()
  }
})
</script>

<style scoped>
.mesh-quality-heatmap {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 16px;
  background: white;
  border-radius: 12px;
}

.quality-tabs {
  display: flex;
  gap: 8px;
}

.quality-tab {
  padding: 8px 16px;
  border-radius: 8px;
  border: 1px solid var(--border-color, #e2e8f0);
  background: white;
  color: var(--text-secondary, #64748b);
  cursor: pointer;
  transition: all 0.2s;
}

.quality-tab.active {
  background: var(--primary-color, #3b82f6);
  color: white;
  border-color: var(--primary-color, #3b82f6);
}

.viewport-container {
  position: relative;
  height: 300px;
  background: #1e293b;
  border-radius: 8px;
  overflow: hidden;
}

.heatmap-canvas {
  width: 100%;
  height: 100%;
}

.empty-state {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #94a3b8;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 12px;
}

.empty-hint {
  font-size: 12px;
  margin-top: 4px;
}

.color-legend {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.legend-bar {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.gradient-bar {
  height: 12px;
  border-radius: 6px;
}

.legend-labels {
  display: flex;
  justify-content: space-between;
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
}

.legend-description {
  display: flex;
  gap: 16px;
  font-size: 12px;
}

.quality-bad {
  color: #ef4444;
}

.quality-warning {
  color: #f59e0b;
}

.quality-good {
  color: #22c55e;
}

.quality-stats {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
}

.stat-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px;
  background: var(--bg-elevated, #f8fafc);
  border-radius: 8px;
}

.stat-card.highlight {
  background: var(--primary-color, #3b82f6);
}

.stat-card.highlight .stat-value {
  color: white;
}

.stat-card.highlight .stat-label {
  color: rgba(255, 255, 255, 0.8);
}

.stat-value {
  font-size: 24px;
  font-weight: 700;
}

.stat-value.bad {
  color: #ef4444;
}

.stat-value.warning {
  color: #f59e0b;
}

.stat-value.good {
  color: #22c55e;
}

.stat-label {
  font-size: 12px;
  color: var(--text-muted, #94a3b8);
}

.problem-list {
  background: var(--bg-elevated, #f8fafc);
  border-radius: 8px;
  padding: 12px;
}

.problem-list h4 {
  font-size: 13px;
  color: var(--text-secondary, #64748b);
  margin-bottom: 8px;
}

.problem-items {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.problem-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 6px 8px;
  background: white;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.problem-item:hover {
  background: var(--bg-ground, #f1f5f9);
}

.elem-id {
  font-size: 12px;
  font-family: monospace;
}

.elem-quality {
  font-size: 12px;
  font-weight: 600;
}

.elem-quality.bad {
  color: #ef4444;
}

.elem-quality.warning {
  color: #f59e0b;
}

.elem-quality.good {
  color: #22c55e;
}

.elem-metric {
  font-size: 10px;
  color: var(--text-muted, #94a3b8);
}

.show-more {
  width: 100%;
  padding: 8px;
  margin-top: 8px;
  border-radius: 6px;
  border: 1px solid var(--border-color, #e2e8f0);
  background: white;
  color: var(--text-secondary, #64748b);
  cursor: pointer;
  font-size: 12px;
}

.action-bar {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

.btn-export,
.btn-refresh {
  padding: 8px 16px;
  border-radius: 8px;
  border: 1px solid var(--border-color, #e2e8f0);
  background: white;
  color: var(--text-secondary, #64748b);
  cursor: pointer;
  transition: all 0.2s;
}

.btn-export:hover,
.btn-refresh:hover {
  background: var(--bg-elevated, #f1f5f9);
}
</style>