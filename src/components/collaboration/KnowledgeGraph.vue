<template>
  <div
    ref="containerRef"
    class="knowledge-graph-container"
    @mousedown="onCanvasMouseDown"
    @mousemove="onCanvasMouseMove"
    @mouseup="onCanvasMouseUp"
    @mouseleave="onCanvasMouseUp"
    @wheel.prevent="onWheel"
  >
    <!-- SVG 画布 -->
    <svg
      ref="svgRef"
      :width="svgWidth"
      :height="svgHeight"
      :viewBox="`0 0 ${svgWidth} ${svgHeight}`"
      class="knowledge-graph-svg"
    >
      <defs>
        <!-- 箭头标记 -->
        <marker
          id="arrowhead"
          markerWidth="10"
          markerHeight="7"
          refX="28"
          refY="3.5"
          orient="auto"
          fill="var(--text-secondary, #6b7280)"
        >
          <polygon points="0 0, 10 3.5, 0 7" />
        </marker>
        <marker
          id="arrowhead-highlight"
          markerWidth="10"
          markerHeight="7"
          refX="28"
          refY="3.5"
          orient="auto"
          fill="var(--primary, #3b82f6)"
        >
          <polygon points="0 0, 10 3.5, 0 7" />
        </marker>
      </defs>

      <g :transform="`translate(${panX}, ${panY}) scale(${zoom})`">
        <!-- 边 -->
        <line
          v-for="edge in graphData.edges"
          :key="`edge-${edge.source}-${edge.target}`"
          :x1="getNodeX(edge.source)"
          :y1="getNodeY(edge.source)"
          :x2="getNodeX(edge.target)"
          :y2="getNodeY(edge.target)"
          :stroke="isEdgeHighlighted(edge) ? 'var(--primary, #3b82f6)' : 'var(--text-secondary, #6b7280)'"
          :stroke-width="isEdgeHighlighted(edge) ? 2.5 : 1.5"
          :stroke-opacity="isEdgeHighlighted(edge) ? 1 : 0.5"
          :marker-end="isEdgeHighlighted(edge) ? 'url(#arrowhead-highlight)' : 'url(#arrowhead)'"
          class="kg-edge"
        />

        <!-- 节点 -->
        <g
          v-for="node in graphData.nodes"
          :key="node.id"
          :transform="`translate(${getNodeX(node.id)}, ${getNodeY(node.id)})`"
          class="kg-node"
          :class="{ 'kg-node-current': node.id === currentNoteId }"
          @mousedown.stop="onNodeMouseDown($event, node.id)"
          @mouseenter="hoveredNodeId = node.id"
          @mouseleave="hoveredNodeId = null"
          @click.stop="onNodeClick(node.id)"
        >
          <!-- 节点光晕（当前笔记） -->
          <circle
            v-if="node.id === currentNoteId"
            r="24"
            fill="none"
            :stroke="'var(--primary, #3b82f6)'"
            stroke-width="2"
            stroke-opacity="0.4"
          />
          <!-- 节点圆 -->
          <circle
            r="18"
            :fill="getNodeColor(node.file_type)"
            :stroke="node.id === currentNoteId ? 'var(--primary, #3b82f6)' : 'var(--bg-primary, #fff)'"
            :stroke-width="node.id === currentNoteId ? 3 : 2"
            class="kg-node-circle"
            :class="{ 'kg-node-hovered': hoveredNodeId === node.id }"
          />
          <!-- 节点文字标签 -->
          <text
            :y="30"
            text-anchor="middle"
            :fill="'var(--text-primary, #1f2937)'"
            font-size="11"
            class="kg-node-label"
          >
            {{ truncateTitle(node.title) }}
          </text>
        </g>
      </g>
    </svg>

    <!-- 悬浮提示 -->
    <div
      v-if="hoveredNodeId && hoveredNodeData"
      class="kg-tooltip"
      :style="{ left: tooltipX + 'px', top: tooltipY + 'px' }"
    >
      {{ hoveredNodeData.title }}
    </div>

    <!-- 缩放控制 -->
    <div class="kg-zoom-controls">
      <button @click.stop="zoomIn" class="kg-zoom-btn" title="放大">+</button>
      <button @click.stop="zoomOut" class="kg-zoom-btn" title="缩小">-</button>
      <button @click.stop="resetView" class="kg-zoom-btn" title="重置视图">Reset</button>
    </div>

    <!-- 空状态 -->
    <div v-if="graphData.nodes.length === 0 && !loading" class="kg-empty">
      <div class="kg-empty-icon">Network</div>
      <div>暂无笔记数据</div>
    </div>

    <!-- 加载状态 -->
    <div v-if="loading" class="kg-loading">
      <div class="kg-loading-spinner"></div>
      <div>加载知识网络...</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, onUnmounted, watch, computed, nextTick } from 'vue'
import { getKnowledgeGraph, type KnowledgeGraph, type KnowledgeGraphNode } from '@/api'

interface Props {
  projectId: string
  currentNoteId?: string
}

interface Emits {
  (e: 'select-note', noteId: string): void
}

const props = withDefaults(defineProps<Props>(), {
  currentNoteId: ''
})

const emit = defineEmits<Emits>()

// ============ 状态 ============
const containerRef = ref<HTMLElement | null>(null)
const svgRef = ref<SVGSVGElement | null>(null)
const svgWidth = ref(800)
const svgHeight = ref(600)
const loading = ref(true)
const graphData = reactive<KnowledgeGraph>({ nodes: [], edges: [] })

// 节点位置存储
const nodePositions = reactive<Record<string, { x: number; y: number; vx: number; vy: number }>>({})

// 交互状态
const panX = ref(0)
const panY = ref(0)
const zoom = ref(1)
const isPanning = ref(false)
const panStartX = ref(0)
const panStartY = ref(0)
const panStartPanX = ref(0)
const panStartPanY = ref(0)
const draggingNodeId = ref<string | null>(null)
const hoveredNodeId = ref<string | null>(null)
const tooltipX = ref(0)
const tooltipY = ref(0)

// 力导向模拟
let animationFrameId: number | null = null
let simulationRunning = false
let simulationAlpha = 1.0 // 温度，从1冷却到0

// 力导向参数
const REPULSION_FORCE = 5000 // 斥力常数
const SPRING_FORCE = 0.005 // 弹簧力常数
const SPRING_LENGTH = 150 // 弹簧自然长度
const CENTER_FORCE = 0.01 // 中心引力
const DAMPING = 0.85 // 阻尼系数
const ALPHA_DECAY = 0.998 // 温度衰减
const ALPHA_MIN = 0.001 // 最低温度阈值
const NODE_RADIUS = 18

// ============ 计算属性 ============
const hoveredNodeData = computed<KnowledgeGraphNode | null>(() => {
  if (!hoveredNodeId.value) return null
  return graphData.nodes.find(n => n.id === hoveredNodeId.value) || null
})

// ============ 方法 ============

function getNodeX(id: string): number {
  return nodePositions[id]?.x ?? 0
}

function getNodeY(id: string): number {
  return nodePositions[id]?.y ?? 0
}

function truncateTitle(title: string): string {
  if (!title) return 'Untitled'
  return title.length > 8 ? title.slice(0, 7) + '...' : title
}

function getNodeColor(fileType: string): string {
  switch (fileType) {
    case 'note':
      return 'var(--primary, #3b82f6)'
    case 'modeling_data':
      return '#10b981'
    case 'code_file':
      return '#f59e0b'
    default:
      return 'var(--primary, #3b82f6)'
  }
}

function isEdgeHighlighted(edge: { source: string; target: string }): boolean {
  if (!hoveredNodeId.value && !props.currentNoteId) return false
  const activeId = hoveredNodeId.value || props.currentNoteId
  return edge.source === activeId || edge.target === activeId
}

// ============ 加载数据 ============
async function loadGraph() {
  if (!props.projectId) return

  loading.value = true
  try {
    const data = await getKnowledgeGraph(props.projectId)
    graphData.nodes = data.nodes
    graphData.edges = data.edges

    // 初始化节点位置（圆形布局）
    initializePositions()

    // 启动力导向模拟
    startSimulation()
  } catch (e) {
    console.error('Failed to load knowledge graph:', e)
    graphData.nodes = []
    graphData.edges = []
  } finally {
    loading.value = false
  }
}

function initializePositions() {
  const centerX = svgWidth.value / 2
  const centerY = svgHeight.value / 2
  const count = graphData.nodes.length
  const radius = Math.min(svgWidth.value, svgHeight.value) * 0.3

  graphData.nodes.forEach((node, i) => {
    if (!nodePositions[node.id]) {
      const angle = (2 * Math.PI * i) / Math.max(count, 1)
      nodePositions[node.id] = {
        x: centerX + radius * Math.cos(angle),
        y: centerY + radius * Math.sin(angle),
        vx: 0,
        vy: 0
      }
    }
  })

  // 移除已不存在的节点
  const nodeIds = new Set(graphData.nodes.map(n => n.id))
  for (const key of Object.keys(nodePositions)) {
    if (!nodeIds.has(key)) {
      delete nodePositions[key]
    }
  }
}

// ============ 力导向模拟 ============
function startSimulation() {
  simulationAlpha = 1.0
  if (!simulationRunning) {
    simulationRunning = true
    tick()
  }
}

function tick() {
  if (simulationAlpha < ALPHA_MIN) {
    simulationRunning = false
    animationFrameId = null
    return
  }

  const centerX = svgWidth.value / 2
  const centerY = svgHeight.value / 2
  const nodes = graphData.nodes
  const edges = graphData.edges

  // 构建邻接表
  const adjacency = new Map<string, string[]>()
  for (const edge of edges) {
    if (!adjacency.has(edge.source)) adjacency.set(edge.source, [])
    if (!adjacency.has(edge.target)) adjacency.set(edge.target, [])
    adjacency.get(edge.source)!.push(edge.target)
    adjacency.get(edge.target)!.push(edge.source)
  }

  // 计算力
  for (let i = 0; i < nodes.length; i++) {
    const nodeA = nodePositions[nodes[i].id]
    if (!nodeA) continue
    if (draggingNodeId.value === nodes[i].id) continue

    let fx = 0
    let fy = 0

    // 1. 节点间斥力（库仑力）
    for (let j = 0; j < nodes.length; j++) {
      if (i === j) continue
      const nodeB = nodePositions[nodes[j].id]
      if (!nodeB) continue

      const dx = nodeA.x - nodeB.x
      const dy = nodeA.y - nodeB.y
      const distSq = Math.max(dx * dx + dy * dy, 100) // 避免除零
      const dist = Math.sqrt(distSq)
      const force = REPULSION_FORCE / distSq

      fx += (dx / dist) * force
      fy += (dy / dist) * force
    }

    // 2. 连接节点的弹簧力
    const neighbors = adjacency.get(nodes[i].id) || []
    for (const neighborId of neighbors) {
      const nodeB = nodePositions[neighborId]
      if (!nodeB) continue

      const dx = nodeB.x - nodeA.x
      const dy = nodeB.y - nodeA.y
      const dist = Math.max(Math.sqrt(dx * dx + dy * dy), 1)
      const displacement = dist - SPRING_LENGTH

      fx += (dx / dist) * displacement * SPRING_FORCE
      fy += (dy / dist) * displacement * SPRING_FORCE
    }

    // 3. 中心引力
    fx += (centerX - nodeA.x) * CENTER_FORCE
    fy += (centerY - nodeA.y) * CENTER_FORCE

    // 更新速度（含阻尼）
    nodeA.vx = (nodeA.vx + fx) * DAMPING * simulationAlpha
    nodeA.vy = (nodeA.vy + fy) * DAMPING * simulationAlpha

    // 更新位置
    nodeA.x += nodeA.vx
    nodeA.y += nodeA.vy
  }

  // 温度衰减
  simulationAlpha *= ALPHA_DECAY

  animationFrameId = requestAnimationFrame(tick)
}

function stopSimulation() {
  if (animationFrameId !== null) {
    cancelAnimationFrame(animationFrameId)
    animationFrameId = null
  }
  simulationRunning = false
}

// ============ 交互：拖拽节点 ============
function onNodeMouseDown(event: MouseEvent, nodeId: string) {
  draggingNodeId.value = nodeId
  const pos = nodePositions[nodeId]
  if (pos) {
    pos.vx = 0
    pos.vy = 0
  }
}

function onNodeClick(nodeId: string) {
  emit('select-note', nodeId)
}

// ============ 交互：画布平移 ============
function onCanvasMouseDown(event: MouseEvent) {
  if (draggingNodeId.value) return
  isPanning.value = true
  panStartX.value = event.clientX
  panStartY.value = event.clientY
  panStartPanX.value = panX.value
  panStartPanY.value = panY.value
}

function onCanvasMouseMove(event: MouseEvent) {
  // 更新悬浮提示位置
  if (containerRef.value) {
    const rect = containerRef.value.getBoundingClientRect()
    tooltipX.value = event.clientX - rect.left + 15
    tooltipY.value = event.clientY - rect.top - 10
  }

  // 拖拽节点
  if (draggingNodeId.value) {
    const pos = nodePositions[draggingNodeId.value]
    if (pos) {
      const dx = (event.clientX - panStartX.value) / zoom.value
      const dy = (event.clientY - panStartY.value) / zoom.value
      // 使用上一次的位置 + 增量
      pos.x += (event.movementX) / zoom.value
      pos.y += (event.movementY) / zoom.value
      pos.vx = 0
      pos.vy = 0

      // 拖拽时重新激活模拟
      if (simulationAlpha < 0.1) {
        simulationAlpha = 0.3
        if (!simulationRunning) {
          simulationRunning = true
          tick()
        }
      }
    }
    return
  }

  // 平移画布
  if (isPanning.value) {
    panX.value = panStartPanX.value + (event.clientX - panStartX.value)
    panY.value = panStartPanY.value + (event.clientY - panStartY.value)
  }
}

function onCanvasMouseUp() {
  draggingNodeId.value = null
  isPanning.value = false
}

// ============ 交互：缩放 ============
function onWheel(event: WheelEvent) {
  const delta = event.deltaY > 0 ? 0.9 : 1.1
  const newZoom = Math.max(0.1, Math.min(5, zoom.value * delta))

  // 以鼠标位置为中心缩放
  if (containerRef.value) {
    const rect = containerRef.value.getBoundingClientRect()
    const mouseX = event.clientX - rect.left
    const mouseY = event.clientY - rect.top

    panX.value = mouseX - (mouseX - panX.value) * (newZoom / zoom.value)
    panY.value = mouseY - (mouseY - panY.value) * (newZoom / zoom.value)
  }

  zoom.value = newZoom
}

function zoomIn() {
  zoom.value = Math.min(5, zoom.value * 1.2)
}

function zoomOut() {
  zoom.value = Math.max(0.1, zoom.value * 0.8)
}

function resetView() {
  zoom.value = 1
  panX.value = 0
  panY.value = 0
  // 重新初始化位置并启动模拟
  initializePositions()
  startSimulation()
}

// ============ 尺寸自适应 ============
function updateSize() {
  if (containerRef.value) {
    svgWidth.value = containerRef.value.clientWidth
    svgHeight.value = containerRef.value.clientHeight
  }
}

// ============ 生命周期 ============
onMounted(async () => {
  await nextTick()
  updateSize()
  await loadGraph()
  window.addEventListener('resize', updateSize)
})

onUnmounted(() => {
  stopSimulation()
  window.removeEventListener('resize', updateSize)
})

// 监听项目变化
watch(() => props.projectId, () => {
  stopSimulation()
  // 清空位置
  for (const key of Object.keys(nodePositions)) {
    delete nodePositions[key]
  }
  loadGraph()
})
</script>

<style scoped>
.knowledge-graph-container {
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
  background: var(--bg-primary, #ffffff);
  cursor: grab;
  user-select: none;
}

.knowledge-graph-container:active {
  cursor: grabbing;
}

.knowledge-graph-svg {
  display: block;
}

.kg-node {
  cursor: pointer;
}

.kg-node-circle {
  transition: r 0.15s ease, filter 0.15s ease;
  filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1));
}

.kg-node-circle.kg-node-hovered {
  r: 22;
  filter: drop-shadow(0 2px 8px rgba(0, 0, 0, 0.2));
}

.kg-node-current .kg-node-circle {
  filter: drop-shadow(0 0 8px var(--primary, #3b82f6));
}

.kg-node-label {
  pointer-events: none;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  fill: var(--text-primary, #1f2937);
}

.kg-edge {
  pointer-events: none;
}

.kg-tooltip {
  position: absolute;
  padding: 4px 10px;
  background: var(--bg-secondary, #374151);
  color: var(--text-inverse, #ffffff);
  font-size: 12px;
  border-radius: 4px;
  pointer-events: none;
  white-space: nowrap;
  z-index: 10;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.kg-zoom-controls {
  position: absolute;
  bottom: 16px;
  right: 16px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  z-index: 10;
}

.kg-zoom-btn {
  width: 36px;
  height: 36px;
  border: 1px solid var(--border-color, #e5e7eb);
  background: var(--bg-primary, #ffffff);
  color: var(--text-primary, #1f2937);
  border-radius: 6px;
  font-size: 16px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.15s;
}

.kg-zoom-btn:hover {
  background: var(--bg-secondary, #f3f4f6);
}

.kg-empty {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary, #6b7280);
  font-size: 14px;
  gap: 8px;
}

.kg-empty-icon {
  font-size: 48px;
  opacity: 0.3;
}

.kg-loading {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary, #6b7280);
  font-size: 14px;
  gap: 12px;
}

.kg-loading-spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--border-color, #e5e7eb);
  border-top-color: var(--primary, #3b82f6);
  border-radius: 50%;
  animation: kg-spin 0.8s linear infinite;
}

@keyframes kg-spin {
  to {
    transform: rotate(360deg);
  }
}

/* 深色模式 */
:global(.dark) .kg-zoom-btn {
  border-color: var(--border-color, #4b5563);
  background: var(--bg-primary, #1f2937);
  color: var(--text-primary, #f9fafb);
}

:global(.dark) .kg-zoom-btn:hover {
  background: var(--bg-secondary, #374151);
}
</style>
