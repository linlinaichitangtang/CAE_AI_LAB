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
          🧹 橡皮擦
        </button>
      </div>

      <!-- 画布 -->
      <div class="flex-1 p-4 bg-gray-100 dark:bg-gray-900 flex justify-center items-center">
        <canvas
          ref="canvasRef"
          class="bg-white rounded shadow-lg cursor-crosshair"
          :width="canvasWidth"
          :height="canvasHeight"
          @mousedown="startDrawing"
          @mousemove="draw"
          @mouseup="stopDrawing"
          @mouseleave="stopDrawing"
          @touchstart.prevent="startDrawing"
          @touchmove.prevent="draw"
          @touchend.prevent="stopDrawing"
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
import { ref, onMounted } from 'vue'

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

interface Stroke {
  points: { x: number; y: number }[]
  width: number
  color: string
  isEraser: boolean
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

const redrawCanvas = () => {
  const context = getContext()
  if (!context || !canvasRef.value) return
  
  context.fillStyle = '#ffffff'
  context.fillRect(0, 0, canvasWidth, canvasHeight)
  
  for (const stroke of strokes.value) {
    if (stroke.points.length < 2) continue
    
    context.beginPath()
    context.lineWidth = stroke.width
    context.lineCap = 'round'
    context.lineJoin = 'round'
    context.strokeStyle = stroke.isEraser ? '#ffffff' : stroke.color
    
    context.moveTo(stroke.points[0].x, stroke.points[0].y)
    for (let i = 1; i < stroke.points.length; i++) {
      context.lineTo(stroke.points[i].x, stroke.points[i].y)
    }
    context.stroke()
  }
}

const getCanvasCoords = (e: MouseEvent | TouchEvent): { x: number; y: number } => {
  const canvas = canvasRef.value
  if (!canvas) return { x: 0, y: 0 }
  
  const rect = canvas.getBoundingClientRect()
  let clientX: number, clientY: number
  
  if (e instanceof TouchEvent) {
    clientX = e.touches[0].clientX
    clientY = e.touches[0].clientY
  } else {
    clientX = e.clientX
    clientY = e.clientY
  }
  
  return {
    x: clientX - rect.left,
    y: clientY - rect.top
  }
}

const startDrawing = (e: MouseEvent | TouchEvent) => {
  isDrawing.value = true
  const coords = getCanvasCoords(e)
  
  currentStroke.value = {
    points: [coords],
    width: isEraser.value ? strokeWidth.value * 3 : strokeWidth.value,
    color: strokeColor.value,
    isEraser: isEraser.value
  }
}

const draw = (e: MouseEvent | TouchEvent) => {
  if (!isDrawing.value || !currentStroke.value) return
  
  const coords = getCanvasCoords(e)
  currentStroke.value.points.push(coords)
  
  redrawCanvas()
  
  const context = getContext()
  if (!context || !currentStroke.value) return
  
  const points = currentStroke.value.points
  if (points.length >= 2) {
    context.beginPath()
    context.lineWidth = currentStroke.value.width
    context.lineCap = 'round'
    context.lineJoin = 'round'
    context.strokeStyle = currentStroke.value.isEraser ? '#ffffff' : currentStroke.value.color
    
    const prev = points[points.length - 2]
    const curr = points[points.length - 1]
    
    context.moveTo(prev.x, prev.y)
    context.lineTo(curr.x, curr.y)
    context.stroke()
  }
}

const stopDrawing = () => {
  if (currentStroke.value && currentStroke.value.points.length > 0) {
    strokes.value.push(currentStroke.value)
  }
  currentStroke.value = null
  isDrawing.value = false
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
  redrawCanvas()
})
</script>

<style scoped>
canvas {
  touch-action: none;
}
</style>