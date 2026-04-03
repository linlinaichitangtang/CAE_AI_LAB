<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">跨尺度数据可视化映射</h2>
        <p class="text-sm" style="color: var(--text-muted)">V1.8-008 | 原子/相场/FE 多层叠加，场映射着色，动画生成</p>
      </div>
      <div class="flex items-center gap-2">
        <button class="btn btn-ghost text-xs" @click="resetViz">重置</button>
        <button class="btn btn-primary text-xs" @click="loadPreset">加载预设</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Layer Management -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            图层管理
          </h4>
          <div class="space-y-3">
            <div v-for="layer in layers" :key="layer.type" class="p-2.5 rounded" style="background: var(--bg-elevated)">
              <div class="flex items-center justify-between mb-2">
                <label class="flex items-center gap-2 text-xs cursor-pointer" style="color: var(--text-primary)">
                  <input type="checkbox" v-model="layer.visible" class="rounded" />
                  <span class="font-medium">{{ layerLabel(layer.type) }}</span>
                </label>
                <span class="text-[10px] px-1.5 py-0.5 rounded" style="background: var(--bg-base); color: var(--text-muted)">
                  {{ layer.type }}
                </span>
              </div>
              <div class="space-y-1.5">
                <div>
                  <div class="flex justify-between text-[10px] mb-0.5">
                    <span style="color: var(--text-muted)">不透明度</span>
                    <span style="color: var(--text-secondary)">{{ layer.opacity }}%</span>
                  </div>
                  <input type="range" min="0" max="100" v-model.number="layer.opacity" class="w-full h-1.5 rounded-full appearance-none cursor-pointer" style="accent-color: var(--primary)" />
                </div>
                <div>
                  <label class="text-[10px]" style="color: var(--text-muted)">颜色映射</label>
                  <select class="input w-full text-xs mt-0.5" v-model="layer.colorMap">
                    <option v-for="cm in colorMaps" :key="cm.id" :value="cm.id">{{ cm.name }}</option>
                  </select>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 2: Viewport Controls -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            视口控制
          </h4>
          <div class="space-y-2">
            <div class="grid grid-cols-3 gap-1.5">
              <div>
                <label class="text-[10px]" style="color: var(--text-muted)">Center X</label>
                <input class="input w-full text-xs" type="number" v-model.number="viewport.center.x" step="0.5" />
              </div>
              <div>
                <label class="text-[10px]" style="color: var(--text-muted)">Center Y</label>
                <input class="input w-full text-xs" type="number" v-model.number="viewport.center.y" step="0.5" />
              </div>
              <div>
                <label class="text-[10px]" style="color: var(--text-muted)">Center Z</label>
                <input class="input w-full text-xs" type="number" v-model.number="viewport.center.z" step="0.5" />
              </div>
            </div>
            <div>
              <div class="flex justify-between text-[10px] mb-0.5">
                <span style="color: var(--text-muted)">缩放</span>
                <span style="color: var(--text-secondary)">{{ viewport.zoom.toFixed(1) }}x</span>
              </div>
              <input type="range" min="0.1" max="5" step="0.1" v-model.number="viewport.zoom" class="w-full h-1.5 rounded-full appearance-none cursor-pointer" style="accent-color: var(--primary)" />
            </div>
            <div>
              <div class="flex justify-between text-[10px] mb-0.5">
                <span style="color: var(--text-muted)">旋转 Pitch</span>
                <span style="color: var(--text-secondary)">{{ viewport.rotation.pitch }}deg</span>
              </div>
              <input type="range" min="-180" max="180" v-model.number="viewport.rotation.pitch" class="w-full h-1.5 rounded-full appearance-none cursor-pointer" style="accent-color: var(--primary)" />
            </div>
            <div>
              <div class="flex justify-between text-[10px] mb-0.5">
                <span style="color: var(--text-muted)">旋转 Yaw</span>
                <span style="color: var(--text-secondary)">{{ viewport.rotation.yaw }}deg</span>
              </div>
              <input type="range" min="-180" max="180" v-model.number="viewport.rotation.yaw" class="w-full h-1.5 rounded-full appearance-none cursor-pointer" style="accent-color: var(--primary)" />
            </div>
          </div>
        </div>

        <!-- Step 3: Overlay Mode -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            叠加模式
          </h4>
          <div class="grid grid-cols-2 gap-1.5">
            <button
              v-for="mode in overlayModes"
              :key="mode.value"
              @click="overlayMode = mode.value as typeof overlayMode.value"
              class="px-2 py-2 rounded text-xs text-center transition border"
              :style="overlayMode === mode.value
                ? 'background: var(--primary); border-color: var(--primary); color: #fff'
                : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
            >
              {{ mode.label }}
            </button>
          </div>
        </div>

        <!-- Step 4: Atom-to-Field Mapping -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            原子-场映射
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">场名称</label>
              <input class="input w-full text-xs" type="text" v-model="atomMapping.fieldName" placeholder="von_mises_stress" />
            </div>
            <div>
              <label class="label">颜色映射</label>
              <select class="input w-full text-xs" v-model="atomMapping.colorMap">
                <option v-for="cm in colorMaps" :key="cm.id" :value="cm.id">{{ cm.name }}</option>
              </select>
            </div>
            <div class="grid grid-cols-2 gap-1.5">
              <div>
                <label class="label">范围 Min</label>
                <input class="input w-full text-xs" type="number" v-model.number="atomMapping.rangeMin" step="0.1" />
              </div>
              <div>
                <label class="label">范围 Max</label>
                <input class="input w-full text-xs" type="number" v-model.number="atomMapping.rangeMax" step="0.1" />
              </div>
            </div>
          </div>
        </div>

        <!-- Step 5: Generate -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            生成
          </h4>
          <div class="space-y-2">
            <button class="btn btn-primary text-xs w-full" @click="generateVisualization">生成可视化</button>
            <button class="btn btn-ghost text-xs w-full" @click="generateAnimation">生成动画</button>
          </div>
          <div v-if="vizResult" class="mt-3 p-3 rounded" style="background: var(--bg-elevated)">
            <div class="space-y-1 text-xs">
              <div class="flex justify-between">
                <span style="color: var(--text-muted)">总数据点</span>
                <span style="color: var(--text-primary)">{{ vizResult.total_points.toLocaleString() }}</span>
              </div>
              <div class="flex justify-between">
                <span style="color: var(--text-muted)">渲染耗时</span>
                <span style="color: var(--text-primary)">{{ vizResult.rendering_time_ms }}ms</span>
              </div>
              <div class="flex justify-between">
                <span style="color: var(--text-muted)">状态</span>
                <span :style="{ color: vizResult.success ? 'var(--accent-green)' : 'var(--accent-red)' }">
                  {{ vizResult.success ? '成功' : '失败' }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4">

        <!-- Multi-Layer Visualization -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">多层可视化</h4>
          <div class="p-4 rounded" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
            <svg width="100%" height="420" viewBox="0 0 600 420">
              <defs>
                <!-- Phase field gradient -->
                <linearGradient id="phaseFieldGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                  <stop offset="0%" style="stop-color:#3b82f6;stop-opacity:0.3" />
                  <stop offset="50%" style="stop-color:#8b5cf6;stop-opacity:0.5" />
                  <stop offset="100%" style="stop-color:#ec4899;stop-opacity:0.3" />
                </linearGradient>
                <!-- Atom glow filter -->
                <filter id="atomGlow" x="-50%" y="-50%" width="200%" height="200%">
                  <feGaussianBlur stdDeviation="2" result="blur" />
                  <feMerge>
                    <feMergeNode in="blur" />
                    <feMergeNode in="SourceGraphic" />
                  </feMerge>
                </filter>
              </defs>

              <!-- FE Mesh Grid (background) -->
              <g v-if="isLayerVisible('fe_mesh')" :opacity="getLayerOpacity('fe_mesh')">
                <rect x="40" y="20" width="520" height="380" fill="none" stroke="var(--border-default)" stroke-width="0.5" />
                <!-- Grid lines -->
                <line v-for="i in 13" :key="'h'+i" x1="40" :y1="20 + i * 29.2" x2="560" :y2="20 + i * 29.2" stroke="var(--border-subtle)" stroke-width="0.5" />
                <line v-for="i in 18" :key="'v'+i" :x1="40 + i * 30.6" :y1="20" :x2="40 + i * 30.6" :y2="400" stroke="var(--border-subtle)" stroke-width="0.5" />
                <!-- FE nodes -->
                <circle v-for="node in feNodes" :key="'fe-'+node.id" :cx="node.x" :cy="node.y" r="2" fill="var(--border-default)" />
              </g>

              <!-- Phase Field Contours -->
              <g v-if="isLayerVisible('phase_field')" :opacity="getLayerOpacity('phase_field')">
                <!-- Contour paths -->
                <path d="M60,100 Q150,60 250,120 T450,100 T560,150" fill="none" stroke="#3b82f6" stroke-width="1.5" opacity="0.6" />
                <path d="M60,160 Q180,120 280,180 T480,160 T560,200" fill="none" stroke="#6366f1" stroke-width="1.5" opacity="0.6" />
                <path d="M60,220 Q200,180 300,240 T500,220 T560,260" fill="none" stroke="#8b5cf6" stroke-width="1.5" opacity="0.6" />
                <path d="M60,280 Q220,240 320,300 T520,280 T560,320" fill="none" stroke="#a855f7" stroke-width="1.5" opacity="0.6" />
                <path d="M60,340 Q240,300 340,360 T540,340 T560,380" fill="none" stroke="#ec4899" stroke-width="1.5" opacity="0.6" />
                <!-- Filled regions -->
                <path d="M60,100 Q150,60 250,120 T450,100 T560,150 L560,200 T480,160 T280,180 Q180,120 60,160 Z" fill="url(#phaseFieldGrad)" opacity="0.15" />
                <path d="M60,220 Q200,180 300,240 T500,220 T560,260 L560,320 T520,280 T320,300 Q220,240 60,280 Z" fill="url(#phaseFieldGrad)" opacity="0.2" />
              </g>

              <!-- Atoms -->
              <g v-if="isLayerVisible('atoms')" :opacity="getLayerOpacity('atoms')" filter="url(#atomGlow)">
                <circle
                  v-for="atom in atomPositions"
                  :key="'atom-'+atom.id"
                  :cx="atom.x"
                  :cy="atom.y"
                  :r="atom.r"
                  :fill="atom.color"
                  :opacity="0.85"
                />
              </g>

              <!-- Error Bars -->
              <g v-if="isLayerVisible('error_bars')" :opacity="getLayerOpacity('error_bars')">
                <g v-for="bar in errorBars" :key="'err-'+bar.id">
                  <line :x1="bar.x" :y1="bar.y - bar.halfH" :x2="bar.x" :y2="bar.y + bar.halfH" stroke="var(--accent-red)" stroke-width="1.5" opacity="0.7" />
                  <line :x1="bar.x - 4" :y1="bar.y - bar.halfH" :x2="bar.x + 4" :y2="bar.y - bar.halfH" stroke="var(--accent-red)" stroke-width="1.5" opacity="0.7" />
                  <line :x1="bar.x - 4" :y1="bar.y + bar.halfH" :x2="bar.x + 4" :y2="bar.y + bar.halfH" stroke="var(--accent-red)" stroke-width="1.5" opacity="0.7" />
                  <circle :cx="bar.x" :cy="bar.y" r="2" fill="var(--accent-red)" opacity="0.8" />
                </g>
              </g>

              <!-- Streamlines -->
              <g v-if="isLayerVisible('streamlines')" :opacity="getLayerOpacity('streamlines')">
                <path v-for="sl in streamlines" :key="'sl-'+sl.id" :d="sl.path" fill="none" stroke="var(--accent-green)" stroke-width="1" opacity="0.5" />
              </g>

              <!-- Legend -->
              <g transform="translate(460, 10)">
                <rect x="0" y="0" width="95" height="80" rx="4" fill="var(--bg-elevated)" stroke="var(--border-default)" stroke-width="0.5" />
                <circle cx="12" cy="16" r="4" fill="#3b82f6" />
                <text x="22" y="19" fill="var(--text-secondary)" font-size="9">原子</text>
                <line x1="8" y1="34" x2="16" y2="34" stroke="#8b5cf6" stroke-width="2" />
                <text x="22" y="37" fill="var(--text-secondary)" font-size="9">相场</text>
                <rect x="8" y="46" width="8" height="8" fill="none" stroke="var(--border-default)" stroke-width="0.5" />
                <text x="22" y="54" fill="var(--text-secondary)" font-size="9">FE 网格</text>
                <line x1="8" y1="68" x2="16" y2="68" stroke="var(--accent-red)" stroke-width="1.5" />
                <text x="22" y="71" fill="var(--text-secondary)" font-size="9">误差条</text>
              </g>
            </svg>
          </div>
        </div>

        <!-- Color Map Preview -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">颜色映射预览</h4>
          <div class="p-4 rounded" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
            <svg width="100%" height="60" viewBox="0 0 500 60">
              <defs>
                <linearGradient id="colorMapPreview" x1="0%" y1="0%" x2="100%" y2="0%">
                  <stop offset="0%" style="stop-color:#00008B" />
                  <stop offset="25%" style="stop-color:#0000FF" />
                  <stop offset="50%" style="stop-color:#00FF00" />
                  <stop offset="75%" style="stop-color:#FFFF00" />
                  <stop offset="100%" style="stop-color:#FF0000" />
                </linearGradient>
              </defs>
              <rect x="40" y="10" width="420" height="20" rx="3" fill="url(#colorMapPreview)" />
              <text x="40" y="48" fill="var(--text-muted)" font-size="10">{{ atomMapping.rangeMin }}</text>
              <text x="250" y="48" fill="var(--text-muted)" font-size="10" text-anchor="middle">{{ ((atomMapping.rangeMin + atomMapping.rangeMax) / 2).toFixed(1) }}</text>
              <text x="460" y="48" fill="var(--text-muted)" font-size="10" text-anchor="end">{{ atomMapping.rangeMax }}</text>
              <text x="250" y="58" fill="var(--text-muted)" font-size="9" text-anchor="middle">{{ activeColorMapName }}</text>
            </svg>
          </div>
        </div>

        <!-- Layer Composition Info Cards -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">图层信息</h4>
          <div class="grid grid-cols-3 gap-3">
            <div
              v-for="info in layerInfoCards"
              :key="info.type"
              class="p-3 rounded"
              style="background: var(--bg-surface); border: 1px solid var(--border-subtle)"
            >
              <div class="flex items-center justify-between mb-2">
                <span class="text-xs font-medium" style="color: var(--text-primary)">{{ info.label }}</span>
                <span
                  class="text-[10px] px-1.5 py-0.5 rounded-full"
                  :style="info.visible
                    ? 'background: rgba(34,197,94,0.15); color: var(--accent-green)'
                    : 'background: rgba(156,163,175,0.15); color: var(--text-muted)'"
                >{{ info.visible ? '可见' : '隐藏' }}</span>
              </div>
              <div class="space-y-1 text-[10px]">
                <div class="flex justify-between">
                  <span style="color: var(--text-muted)">数据点</span>
                  <span style="color: var(--text-secondary)">{{ info.points.toLocaleString() }}</span>
                </div>
                <div class="flex justify-between">
                  <span style="color: var(--text-muted)">范围</span>
                  <span style="color: var(--text-secondary)">{{ info.bounds }}</span>
                </div>
                <div class="flex justify-between">
                  <span style="color: var(--text-muted)">不透明度</span>
                  <span style="color: var(--text-secondary)">{{ info.opacity }}%</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type { VizLayer, CrossScaleVizConfig, CrossScaleVizResult } from '../api/crossScaleViz'

// ============ Refs ============
const vizResult = ref<CrossScaleVizResult | null>(null)
const overlayMode = ref<'transparent' | 'contour' | 'isosurface' | 'cross_section'>('contour')

// ============ Layer Config ============
const layers = reactive([
  { type: 'atoms' as VizLayer, visible: true, opacity: 80, colorMap: 'jet' },
  { type: 'phase_field' as VizLayer, visible: true, opacity: 60, colorMap: 'viridis' },
  { type: 'fe_mesh' as VizLayer, visible: true, opacity: 40, colorMap: 'grayscale' },
  { type: 'error_bars' as VizLayer, visible: true, opacity: 70, colorMap: 'hot' },
  { type: 'streamlines' as VizLayer, visible: false, opacity: 50, colorMap: 'coolwarm' }
])

// ============ Viewport ============
const viewport = reactive({
  center: { x: 25.0, y: 25.0, z: 0.0 },
  zoom: 1.0,
  rotation: { pitch: 30, yaw: 45 }
})

// ============ Atom Mapping ============
const atomMapping = reactive({
  fieldName: 'von_mises_stress',
  colorMap: 'jet',
  rangeMin: 0.0,
  rangeMax: 500.0
})

// ============ Color Maps ============
const colorMaps = [
  { id: 'jet', name: 'Jet' },
  { id: 'viridis', name: 'Viridis' },
  { id: 'plasma', name: 'Plasma' },
  { id: 'inferno', name: 'Inferno' },
  { id: 'hot', name: 'Hot' },
  { id: 'coolwarm', name: 'Coolwarm' },
  { id: 'grayscale', name: 'Grayscale' },
  { id: 'turbo', name: 'Turbo' }
]

// ============ Overlay Modes ============
const overlayModes = [
  { value: 'transparent', label: '透明' },
  { value: 'contour', label: '等高线' },
  { value: 'isosurface', label: '等值面' },
  { value: 'cross_section', label: '截面' }
]

// ============ Mock Data: Atom Positions ============
const atomPositions = ref<Array<{ id: number; x: number; y: number; r: number; color: string }>>([])

function generateAtomPositions() {
  const atoms: Array<{ id: number; x: number; y: number; r: number; color: string }> = []
  const colors = ['#3b82f6', '#6366f1', '#8b5cf6', '#a855f7', '#ec4899', '#f97316', '#eab308', '#22c55e']
  let id = 0
  for (let row = 0; row < 12; row++) {
    for (let col = 0; col < 16; col++) {
      const jitterX = (Math.random() - 0.5) * 8
      const jitterY = (Math.random() - 0.5) * 8
      atoms.push({
        id: id++,
        x: 60 + col * 32 + jitterX,
        y: 40 + row * 30 + jitterY,
        r: 3 + Math.random() * 3,
        color: colors[Math.floor(Math.random() * colors.length)]
      })
    }
  }
  return atoms
}

// ============ Mock Data: FE Nodes ============
const feNodes = ref<Array<{ id: number; x: number; y: number }>>([])

function generateFENodes() {
  const nodes: Array<{ id: number; x: number; y: number }> = []
  let id = 0
  for (let row = 0; row <= 13; row++) {
    for (let col = 0; col <= 18; col++) {
      nodes.push({
        id: id++,
        x: 40 + col * 30.6,
        y: 20 + row * 29.2
      })
    }
  }
  return nodes
}

// ============ Mock Data: Error Bars ============
const errorBars = ref<Array<{ id: number; x: number; y: number; halfH: number }>>([])

function generateErrorBars() {
  const bars: Array<{ id: number; x: number; y: number; halfH: number }> = []
  for (let i = 0; i < 20; i++) {
    bars.push({
      id: i,
      x: 80 + i * 24,
      y: 100 + Math.random() * 250,
      halfH: 8 + Math.random() * 25
    })
  }
  return bars
}

// ============ Mock Data: Streamlines ============
const streamlines = ref<Array<{ id: number; path: string }>>([])

function generateStreamlines() {
  const lines: Array<{ id: number; path: string }> = []
  for (let i = 0; i < 8; i++) {
    const startX = 60 + Math.random() * 100
    const startY = 40 + i * 45 + Math.random() * 20
    let d = `M${startX},${startY}`
    let cx = startX
    let cy = startY
    for (let j = 0; j < 6; j++) {
      const cpx = cx + 60 + Math.random() * 40
      const cpy = cy + (Math.random() - 0.5) * 40
      cx = cx + 80 + Math.random() * 40
      cy = cy + (Math.random() - 0.5) * 30
      d += ` Q${cpx},${cpy} ${cx},${cy}`
    }
    lines.push({ id: i, path: d })
  }
  return lines
}

// ============ Computed ============
const activeColorMapName = computed(() => {
  const found = colorMaps.find(cm => cm.id === atomMapping.colorMap)
  return found ? found.name : 'Jet'
})

const layerInfoCards = computed(() => {
  const info = [
    { type: 'atoms' as VizLayer, label: '原子', points: 192, bounds: '0-50 A', opacity: 80 },
    { type: 'phase_field' as VizLayer, label: '相场', points: 4096, bounds: '0-50 A', opacity: 60 },
    { type: 'fe_mesh' as VizLayer, label: 'FE 网格', points: 266, bounds: '0-50 mm', opacity: 40 },
    { type: 'error_bars' as VizLayer, label: '误差条', points: 20, bounds: '0-500 MPa', opacity: 70 },
    { type: 'streamlines' as VizLayer, label: '流线', points: 48, bounds: '0-50 A', opacity: 50 }
  ]
  return info.map(item => {
    const layer = layers.find(l => l.type === item.type)
    return {
      ...item,
      visible: layer ? layer.visible : false,
      opacity: layer ? layer.opacity : 0
    }
  })
})

// ============ Methods ============
function layerLabel(type: VizLayer): string {
  const map: Record<VizLayer, string> = {
    atoms: '原子层',
    phase_field: '相场层',
    fe_mesh: 'FE 网格层',
    error_bars: '误差条层',
    streamlines: '流线层'
  }
  return map[type]
}

function isLayerVisible(type: VizLayer): boolean {
  const layer = layers.find(l => l.type === type)
  return layer ? layer.visible : false
}

function getLayerOpacity(type: VizLayer): number {
  const layer = layers.find(l => l.type === type)
  return layer ? layer.opacity / 100 : 0
}

async function generateVisualization() {
  const config: CrossScaleVizConfig = {
    layers: layers.filter(l => l.visible).map(l => ({
      type: l.type,
      visible: l.visible,
      opacity: l.opacity / 100,
      color_map: l.colorMap,
      data_source: `source_${l.type}`
    })),
    viewport: {
      center: { ...viewport.center },
      zoom: viewport.zoom,
      rotation: { ...viewport.rotation }
    },
    mapping_id: `map-${Date.now()}`,
    overlay_mode: overlayMode.value
  }

  // Simulate generation
  await nextTick()

  const visibleLayers = layers.filter(l => l.visible)
  vizResult.value = {
    success: true,
    layers: visibleLayers.map(l => ({
      type: l.type,
      data_url: '',
      bounds: {
        min: { x: 0, y: 0, z: 0 },
        max: { x: 50, y: 50, z: 50 }
      }
    })),
    total_points: 4626,
    rendering_time_ms: 142
  }
}

async function generateAnimation() {
  // Simulate animation generation
  await nextTick()
  alert('动画生成中... 预计 30 帧 @ 10fps')
}

function resetViz() {
  layers.forEach(l => {
    l.visible = true
    l.opacity = l.type === 'atoms' ? 80 : l.type === 'phase_field' ? 60 : l.type === 'fe_mesh' ? 40 : l.type === 'error_bars' ? 70 : 50
  })
  viewport.center = { x: 25.0, y: 25.0, z: 0.0 }
  viewport.zoom = 1.0
  viewport.rotation = { pitch: 30, yaw: 45 }
  overlayMode.value = 'contour'
  atomMapping.fieldName = 'von_mises_stress'
  atomMapping.colorMap = 'jet'
  atomMapping.rangeMin = 0.0
  atomMapping.rangeMax = 500.0
  vizResult.value = null
}

function loadPreset() {
  layers[0].visible = true
  layers[0].opacity = 85
  layers[0].colorMap = 'jet'
  layers[1].visible = true
  layers[1].opacity = 65
  layers[1].colorMap = 'viridis'
  layers[2].visible = true
  layers[2].opacity = 35
  layers[2].colorMap = 'grayscale'
  layers[3].visible = true
  layers[3].opacity = 75
  layers[3].colorMap = 'hot'
  layers[4].visible = false
  overlayMode.value = 'contour'
  atomMapping.fieldName = 'von_mises_stress'
  atomMapping.colorMap = 'jet'
  atomMapping.rangeMin = 0.0
  atomMapping.rangeMax = 500.0
  viewport.center = { x: 25.0, y: 25.0, z: 0.0 }
  viewport.zoom = 1.2
  viewport.rotation = { pitch: 25, yaw: 40 }
  generateVisualization()
}

// ============ Lifecycle ============
onMounted(() => {
  atomPositions.value = generateAtomPositions()
  feNodes.value = generateFENodes()
  errorBars.value = generateErrorBars()
  streamlines.value = generateStreamlines()
  vizResult.value = {
    success: true,
    layers: [
      { type: 'atoms', data_url: '', bounds: { min: { x: 0, y: 0, z: 0 }, max: { x: 50, y: 50, z: 50 } } },
      { type: 'phase_field', data_url: '', bounds: { min: { x: 0, y: 0, z: 0 }, max: { x: 50, y: 50, z: 50 } } },
      { type: 'error_bars', data_url: '', bounds: { min: { x: 0, y: 0, z: 0 }, max: { x: 50, y: 50, z: 50 } } }
    ],
    total_points: 4626,
    rendering_time_ms: 142
  }
})
</script>
