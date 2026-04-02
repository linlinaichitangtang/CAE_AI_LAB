<template>
  <div class="handwriting-modal fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="emit('cancel')">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-[800px] max-h-[90vh] flex flex-col">
      <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
        <h3 class="text-lg font-semibold text-gray-800 dark:text-white">手写区域</h3>
        <div class="flex gap-2">
          <button @click="clearCanvas" class="px-3 py-1 text-sm bg-gray-100 dark:bg-gray-700 rounded hover:bg-gray-200 dark:hover:bg-gray-600">
            清空
          </button>
          <button @click="undoStroke" class="px-3 py-1 text-sm bg-gray-100 dark:bg-gray-700 rounded hover:bg-gray-200 dark:hover:bg-gray-600" :disabled="strokes.length === 0">
            撤销
          </button>
        </div>
      </div>

      <!-- 工具栏 -->
      <div class="p-3 border-b border-gray-200 dark:border-gray-700 flex items-center gap-6">
        <!-- 笔刷大小 -->
        <div class="flex items-center gap-2">
          <span class="text-sm text-gray-600 dark:text-gray-400">粗细:</span>
          <input type="range" v-model="strokeWidth" min="1" max="20" class="w-24" />
          <span class="text-sm text-gray-500 w-8">{{ strokeWidth }}px</span>
        </div>

        <!-- 颜色 -->
        <div class="flex items-center gap-2">
          <span class="text-sm text-gray-600 dark:text-gray-400">颜色:</span>
          <div class="flex gap-1">
            <button v-for="color in colors" :key="color" @click="strokeColor = color"
              :class="strokeColor === color ? 'ring-2 ring-blue-500' : ''"
              class="w-6 h-6 rounded-full border-2 border-gray-300">
            </button>
          </div>
        </div>

        <!-- 橡皮擦 -->
        <button @click="toggleEraser" :class="isEraser ? 'bg-red-100 border-red-300' : ''"
          class="px-3 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded hover:bg-gray-100 dark:hover:bg-gray-700">
          橡皮擦
        </button>

        <!-- 压感指示器 -->
        <div v-if="activePointerType === 'pen'" class="flex items-center gap-2">
          <span class="text-sm text-gray-600 dark:text-gray-400">压感:</span>
          <div class="w-16 h-2 bg-gray-200 dark:bg-gray-600 rounded-full overflow-hidden">
            <div class="h-full bg-blue-500 rounded-full transition-all duration-75"
                 :style="{ width: (currentPressure * 100) + '%' }"></div>
          </div>
          <span class="text-xs text-gray-500 w-8">{{ currentPressure.toFixed(2) }}</span>
        </div>
      </div>

      <!-- 画布 -->
      <div class="flex-1 p-4 bg-gray-100 dark:bg-gray-900 flex justify-center items-center">
        <canvas
          ref="canvasRef"
          class="bg-white rounded shadow-lg cursor-crosshair"
          :width="canvasWidth"
          :height="canvasHeight"
          @pointerdown="onPointerDown"
          @pointermove="onPointerMove"
          @pointerup="onPointerUp"
          @pointerleave="onPointerUp"
        ></canvas>
      </div>

      <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-3">
        <button @click="emit('cancel')" class="px-4 py-2 text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded">
          取消
        </button>
        <button @click="handleSave" class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
          插入笔记
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

const emit = defineEmits<{
  (e: 'save', dataUrl: string): void
  (e: 'cancel'): void
}>()

const canvasRef = ref<HTMLCanvasElement | null>(null)
const canvasWidth = 700
const canvasHeight = 400
const strokeWidth = ref(3)
const strokeColor = ref('#000000')
const isEraser = ref(false)
const colors = ['#000000', '#2563eb', '#dc2626', '#16a34a', '#9333ea']

// 压感相关状态
const currentPressure = ref(0)
const activePointerType = ref<'pen' | 'touch' | 'mouse' | ''>('')

interface StrokePoint {
  x: number
  y: number
  pressure: number
}

interface Stroke {
  points: StrokePoint[]
  width: number
  color: string
  isEraser: boolean
  hasPressure: boolean  // 标记是否包含压感数据
}

const strokes = ref<Stroke[]>([])
const currentStroke = ref<Stroke | null>(null)
const isDrawing = ref(false)
const ctx = ref<CanvasRenderingContext2D | null>(null)

const getContext = () => {
  if (!canvasRef.value) return null
  if (!ctx.value) {
    ctx.value = canvasRef.value.getContext('2d')
  }
  return ctx.value
}

/**
 * 根据压感值计算笔画宽度
 * 基础宽度由用户设置，压感值 (0~1) 在此基础上缩放
 */
function getPressureWidth(baseWidth: number, pressure: number): number {
  // 基础宽度 * (0.5 + pressure * 0.5)，最小 1px
  return Math.max(1, baseWidth * (0.5 + pressure * 0.5))
}

/**
 * 根据压感值计算笔画透明度
 * 基础透明度 0.6，最大 1.0
 */
function getPressureOpacity(pressure: number): number {
  return 0.6 + pressure * 0.4
}

/**
 * 绘制带压感的笔画段
 */
function drawStrokeSegment(
  context: CanvasRenderingContext2D,
  from: StrokePoint,
  to: StrokePoint,
  stroke: Stroke
) {
  context.beginPath()

  if (stroke.hasPressure && !stroke.isEraser) {
    // 压感模式：根据压感动态调整宽度和透明度
    const width = getPressureWidth(stroke.width, to.pressure)
    const opacity = getPressureOpacity(to.pressure)
    context.lineWidth = width
    context.globalAlpha = opacity
  } else {
    // 非压感模式：使用固定宽度
    context.lineWidth = stroke.width
    context.globalAlpha = 1.0
  }

  context.lineCap = 'round'
  context.lineJoin = 'round'
  context.strokeStyle = stroke.isEraser ? '#ffffff' : stroke.color

  context.moveTo(from.x, from.y)
  context.lineTo(to.x, to.y)
  context.stroke()

  // 重置透明度
  context.globalAlpha = 1.0
}

/**
 * 重绘整个画布（用于撤销和笔画历史回放）
 */
const redrawCanvas = () => {
  const context = getContext()
  if (!context || !canvasRef.value) return

  context.fillStyle = '#ffffff'
  context.fillRect(0, 0, canvasWidth, canvasHeight)

  for (const stroke of strokes.value) {
    if (stroke.points.length < 2) continue

    if (!stroke.hasPressure || stroke.isEraser) {
      // 非压感笔画：一次性绘制整条路径
      context.beginPath()
      context.lineWidth = stroke.width
      context.lineCap = 'round'
      context.lineJoin = 'round'
      context.strokeStyle = stroke.isEraser ? '#ffffff' : stroke.color
      context.globalAlpha = 1.0

      context.moveTo(stroke.points[0].x, stroke.points[0].y)
      for (let i = 1; i < stroke.points.length; i++) {
        context.lineTo(stroke.points[i].x, stroke.points[i].y)
      }
      context.stroke()
    } else {
      // 压感笔画：逐段绘制以体现压感变化
      for (let i = 1; i < stroke.points.length; i++) {
        drawStrokeSegment(context, stroke.points[i - 1], stroke.points[i], stroke)
      }
    }
  }
}

/**
 * 获取 PointerEvent 在画布上的坐标
 */
const getCanvasCoords = (e: PointerEvent): { x: number; y: number } => {
  const canvas = canvasRef.value
  if (!canvas) return { x: 0, y: 0 }

  const rect = canvas.getBoundingClientRect()
  return {
    x: e.clientX - rect.left,
    y: e.clientY - rect.top
  }
}

/**
 * PointerEvent 事件处理：开始绘制
 */
const onPointerDown = (e: PointerEvent) => {
  // 捕获指针，确保在画布外也能接收到事件
  const canvas = canvasRef.value
  if (canvas) {
    canvas.setPointerCapture(e.pointerId)
  }

  isDrawing.value = true
  activePointerType.value = e.pointerType as 'pen' | 'touch' | 'mouse'

  const coords = getCanvasCoords(e)
  const pressure = e.pressure

  // 更新压感显示
  currentPressure.value = pressure

  // 判断是否为压感输入（仅 pen 类型使用压感）
  const hasPressure = e.pointerType === 'pen' && pressure > 0

  currentStroke.value = {
    points: [{ ...coords, pressure }],
    width: isEraser.value ? strokeWidth.value * 3 : strokeWidth.value,
    color: strokeColor.value,
    isEraser: isEraser.value,
    hasPressure
  }
}

/**
 * PointerEvent 事件处理：绘制中
 */
const onPointerMove = (e: PointerEvent) => {
  // 更新压感显示
  if (e.pointerType === 'pen') {
    currentPressure.value = e.pressure
  }

  if (!isDrawing.value || !currentStroke.value) return
  if (e.buttons === 0) return

  const coords = getCanvasCoords(e)
  const pressure = e.pressure

  currentStroke.value.points.push({ ...coords, pressure })

  // 实时绘制当前笔画段
  const context = getContext()
  if (!context || !currentStroke.value) return

  const points = currentStroke.value.points
  if (points.length >= 2) {
    drawStrokeSegment(context, points[points.length - 2], points[points.length - 1], currentStroke.value)
  }
}

/**
 * PointerEvent 事件处理：停止绘制
 */
const onPointerUp = (e: PointerEvent) => {
  if (currentStroke.value && currentStroke.value.points.length > 0) {
    strokes.value.push(currentStroke.value)
  }
  currentStroke.value = null
  isDrawing.value = false
  currentPressure.value = 0
  activePointerType.value = ''
}

const clearCanvas = () => {
  strokes.value = []
  redrawCanvas()
}

const undoStroke = () => {
  strokes.value.pop()
  redrawCanvas()
}

const toggleEraser = () => {
  isEraser.value = !isEraser.value
}

const handleSave = () => {
  if (!canvasRef.value) return
  const dataUrl = canvasRef.value.toDataURL('image/png')
  emit('save', dataUrl)
}

onMounted(() => {
  // 设置画布的 touch-action 为 none，防止浏览器默认触控行为
  if (canvasRef.value) {
    canvasRef.value.style.touchAction = 'none'
  }
  redrawCanvas()
})

onUnmounted(() => {
  // 释放资源
  ctx.value = null
})
</script>

<style scoped>
canvas {
  touch-action: none;
}
</style>
