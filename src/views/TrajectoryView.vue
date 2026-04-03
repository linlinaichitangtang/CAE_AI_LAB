<!--
  TrajectoryView.vue - 轨迹可视化视图 (AtomEye 风格)
  ==========================================
  V1.5-006: 轨迹可视化
  V1.5-008: 应力/能量时序曲线

  核心功能:
  - 3D 原子渲染 (canvas 2D 投影)
  - 原子着色 (元素/速度/应力/位移/类型/电荷/动能/势能)
  - 键显示 (距离/角度/序参数判据)
  - 鼠标旋转/缩放/平移
  - 时序曲线 (温度/能量/压力/体积)
  - 轨迹统计 (平均温度/压力/能量漂移/密度)
  - 帧播放控制
-->

<template>
  <div class="trajectory-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">轨迹可视化 (AtomEye 风格)</h2>
        <p class="text-xs text-[var(--text-muted)]">原子着色（元素/速度/应力），bonds 显示，旋转/缩放/平移</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="handleLoadDemo" class="btn btn-primary text-xs">
          <span class="mr-1">&#x25B6;</span> 加载演示数据
        </button>
        <button @click="handleExportFrame" :disabled="!trajectoryResult" class="btn btn-ghost text-xs">
          <span class="mr-1">&#x1F4F7;</span> 导出帧图像
        </button>
        <button @click="handleExportCSV" :disabled="!timeSeriesData" class="btn btn-ghost text-xs">
          <span class="mr-1">&#x1F4CA;</span> 导出CSV
        </button>
        <button @click="handleScreenshot" :disabled="!trajectoryResult" class="btn btn-ghost text-xs">
          <span class="mr-1">&#x1F5B1;</span> 截图
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel (w-80) -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col overflow-y-auto">

        <!-- 文件加载 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">文件加载</h3>
          </div>
          <div
            class="border-2 border-dashed border-[var(--border-default)] rounded-lg p-4 text-center cursor-pointer hover:border-[var(--primary)] transition-colors"
            @dragover.prevent="onDragOver"
            @dragleave.prevent="onDragLeave"
            @drop.prevent="onDrop"
            @click="onFileClick"
          >
            <div class="text-2xl mb-1">&#x1F4C2;</div>
            <p class="text-xs text-[var(--text-secondary)]">拖拽上传 dump/xyz/pdb/dcd 文件</p>
            <p class="text-[10px] text-[var(--text-muted)] mt-1">或点击选择文件</p>
          </div>
          <input
            ref="fileInput"
            type="file"
            accept=".dump,.xyz,.pdb,.dcd"
            class="hidden"
            @change="onFileSelected"
          />
          <div v-if="trajectoryResult" class="mt-2 p-2 bg-[var(--bg-elevated)] rounded text-xs text-[var(--text-secondary)]">
            已加载: {{ trajectoryResult.total_frames }} 帧, {{ trajectoryResult.total_atoms }} 原子
          </div>
        </div>

        <!-- 着色模式 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">着色模式</h3>
          </div>
          <div class="grid grid-cols-2 gap-1.5">
            <button
              v-for="mode in coloringModes"
              :key="mode.value"
              @click="coloringMode = mode.value"
              :class="['px-2 py-1.5 rounded text-xs transition',
                coloringMode === mode.value
                  ? 'bg-[var(--primary)] text-white'
                  : 'bg-[var(--bg-base)] text-[var(--text-secondary)] hover:bg-[var(--bg-elevated)]']"
            >
              {{ mode.label }}
            </button>
          </div>
        </div>

        <!-- 键显示 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">键显示</h3>
          </div>
          <div class="flex items-center justify-between mb-2">
            <span class="text-xs text-[var(--text-secondary)]">显示键</span>
            <div
              :class="['toggle', showBonds ? 'active' : '']"
              @click="showBonds = !showBonds"
            >
              <div class="toggle-knob"></div>
            </div>
          </div>
          <div v-if="showBonds">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">判据</label>
            <select v-model="bondCriterion" class="input w-full text-xs">
              <option value="distance">距离</option>
              <option value="angle">角度</option>
              <option value="order_parameter">序参数</option>
            </select>
            <label class="text-xs text-[var(--text-secondary)] block mb-1 mt-2">截断距离 (A)</label>
            <input v-model.number="bondCutoff" type="number" step="0.1" min="0.5" max="10" class="input w-full text-xs" />
          </div>
        </div>

        <!-- 显示设置 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">显示设置</h3>
          </div>
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">原子缩放: {{ atomScale.toFixed(1) }}</label>
            <input v-model.number="atomScale" type="range" min="0.1" max="3.0" step="0.1" class="slider w-full" />
          </div>
          <div class="flex items-center justify-between">
            <span class="text-xs text-[var(--text-secondary)]">盒子线框</span>
            <div
              :class="['toggle', showBox ? 'active' : '']"
              @click="showBox = !showBox"
            >
              <div class="toggle-knob"></div>
            </div>
          </div>
        </div>

        <!-- 帧控制 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">帧控制</h3>
          </div>
          <div class="text-center mb-2">
            <span class="text-lg font-bold text-[var(--primary)]">{{ currentFrameIndex + 1 }}</span>
            <span class="text-xs text-[var(--text-muted)]"> / {{ totalFrames }}</span>
          </div>
          <div class="flex items-center justify-center gap-2 mb-2">
            <button @click="prevFrame" class="btn btn-ghost text-xs px-2 py-1" :disabled="currentFrameIndex <= 0">
              &#x25C0;
            </button>
            <button @click="togglePlay" :class="['btn text-xs px-3 py-1', isPlaying ? 'btn-ghost' : 'btn-primary']">
              {{ isPlaying ? '&#x23F8; 暂停' : '&#x25B6; 播放' }}
            </button>
            <button @click="nextFrame" class="btn btn-ghost text-xs px-2 py-1" :disabled="currentFrameIndex >= totalFrames - 1">
              &#x25B6;
            </button>
          </div>
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">播放速度: {{ playbackSpeed.toFixed(1) }}x</label>
            <input v-model.number="playbackSpeed" type="range" min="0.1" max="5.0" step="0.1" class="slider w-full" />
          </div>
          <div>
            <label class="text-xs text-[var(--text-secondary)] block mb-1">帧滑块</label>
            <input
              v-model.number="currentFrameIndex"
              type="range"
              min="0"
              :max="totalFrames - 1"
              step="1"
              class="slider w-full"
            />
          </div>
        </div>

      </div>

      <!-- Right Area -->
      <div class="flex-1 flex flex-col overflow-hidden">

        <!-- 3D 原子渲染 (上半) -->
        <div class="flex-1 relative min-h-0">
          <canvas
            ref="atomCanvas"
            class="w-full h-full"
            @mousedown="onCanvasMouseDown"
            @mousemove="onCanvasMouseMove"
            @mouseup="onCanvasMouseUp"
            @mouseleave="onCanvasMouseUp"
            @wheel.prevent="onCanvasWheel"
          ></canvas>

          <!-- 颜色图例 -->
          <div
            v-if="trajectoryResult && coloringMode !== 'element'"
            class="absolute top-3 right-3 bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-2 shadow-sm"
          >
            <div class="text-[10px] text-[var(--text-muted)] mb-1">{{ coloringModeLabel }}</div>
            <div class="w-24 h-3 rounded" :style="{ background: colorGradient }"></div>
            <div class="flex justify-between text-[10px] text-[var(--text-muted)] mt-0.5">
              <span>{{ colorMin }}</span>
              <span>{{ colorMax }}</span>
            </div>
          </div>

          <!-- 元素图例 -->
          <div
            v-if="trajectoryResult && coloringMode === 'element'"
            class="absolute top-3 right-3 bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-2 shadow-sm"
          >
            <div class="text-[10px] text-[var(--text-muted)] mb-1">元素</div>
            <div v-for="el in elementLegend" :key="el.name" class="flex items-center gap-1.5 mb-0.5">
              <div class="w-2.5 h-2.5 rounded-full" :style="{ backgroundColor: el.color }"></div>
              <span class="text-[10px] text-[var(--text-secondary)]">{{ el.name }}</span>
            </div>
          </div>

          <!-- 空状态 -->
          <div v-if="!trajectoryResult" class="absolute inset-0 flex items-center justify-center">
            <div class="text-center text-[var(--text-muted)]">
              <div class="text-4xl mb-2">&#x1F52C;</div>
              <p class="text-sm">点击"加载演示数据"或上传轨迹文件</p>
              <p class="text-xs mt-1">支持 LAMMPS dump / XYZ / PDB / DCD 格式</p>
            </div>
          </div>
        </div>

        <!-- 时序曲线 (下半) -->
        <div v-if="timeSeriesData" class="h-52 border-t border-[var(--border-subtle)] bg-[var(--bg-surface)] flex-shrink-0">
          <div class="grid grid-cols-4 h-full gap-0 divide-x divide-[var(--border-subtle)]">
            <!-- 温度 vs 时间 -->
            <div class="p-2 flex flex-col">
              <div class="text-[10px] font-semibold text-[var(--text-secondary)] mb-1">温度 vs 时间</div>
              <div class="flex-1 relative">
                <svg ref="tempSvg" class="w-full h-full"></svg>
              </div>
            </div>
            <!-- 能量 vs 时间 -->
            <div class="p-2 flex flex-col">
              <div class="text-[10px] font-semibold text-[var(--text-secondary)] mb-1">能量 vs 时间</div>
              <div class="flex-1 relative">
                <svg ref="energySvg" class="w-full h-full"></svg>
              </div>
            </div>
            <!-- 压力 vs 时间 -->
            <div class="p-2 flex flex-col">
              <div class="text-[10px] font-semibold text-[var(--text-secondary)] mb-1">压力 vs 时间</div>
              <div class="flex-1 relative">
                <svg ref="pressureSvg" class="w-full h-full"></svg>
              </div>
            </div>
            <!-- 体积 vs 时间 -->
            <div class="p-2 flex flex-col">
              <div class="text-[10px] font-semibold text-[var(--text-secondary)] mb-1">体积 vs 时间</div>
              <div class="flex-1 relative">
                <svg ref="volumeSvg" class="w-full h-full"></svg>
              </div>
            </div>
          </div>
        </div>

        <!-- 底部统计 -->
        <div v-if="trajectoryStats" class="border-t border-[var(--border-subtle)] bg-[var(--bg-elevated)] px-4 py-2 flex-shrink-0">
          <div class="flex items-center gap-6 text-xs">
            <div class="flex items-center gap-1.5">
              <span class="text-[var(--text-muted)]">平均温度:</span>
              <span class="font-semibold text-[var(--accent-red)]">{{ trajectoryStats.avg_temp.toFixed(1) }} K</span>
            </div>
            <div class="flex items-center gap-1.5">
              <span class="text-[var(--text-muted)]">平均压力:</span>
              <span class="font-semibold text-[var(--primary)]">{{ trajectoryStats.avg_pressure.toFixed(2) }} GPa</span>
            </div>
            <div class="flex items-center gap-1.5">
              <span class="text-[var(--text-muted)]">能量漂移:</span>
              <span class="font-semibold text-[var(--accent-yellow)]">{{ trajectoryStats.energy_drift.toExponential(3) }} eV</span>
            </div>
            <div class="flex items-center gap-1.5">
              <span class="text-[var(--text-muted)]">密度:</span>
              <span class="font-semibold text-[var(--accent-green)]">{{ trajectoryStats.density.toFixed(3) }} g/cm&sup3;</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, onUnmounted, watch, nextTick } from 'vue'
import type {
  TrajectoryFrame,
  TrajectoryResult,
  TrajectoryConfig,
  ColoringMode,
  BondCriterion,
  TimeSeriesData,
  StressEnergyResult,
  TrajectoryStats,
  AtomData,
} from '@/api/trajectoryViewer'

// ============ 响应式状态 ============

const trajectoryResult = ref<TrajectoryResult | null>(null)
const currentFrameIndex = ref(0)
const isPlaying = ref(false)
const playbackSpeed = ref(1.0)
const coloringMode = ref<ColoringMode>('element')
const showBonds = ref(true)
const bondCriterion = ref<BondCriterion>('distance')
const bondCutoff = ref(2.86)
const atomScale = ref(1.0)
const showBox = ref(true)
const timeSeriesData = ref<StressEnergyResult | null>(null)
const trajectoryStats = ref<TrajectoryStats | null>(null)

// Canvas refs
const atomCanvas = ref<HTMLCanvasElement | null>(null)
const fileInput = ref<HTMLInputElement | null>(null)
const tempSvg = ref<SVGSVGElement | null>(null)
const energySvg = ref<SVGSVGElement | null>(null)
const pressureSvg = ref<SVGSVGElement | null>(null)
const volumeSvg = ref<SVGSVGElement | null>(null)

// 3D 相机状态
const camera = reactive({
  rotX: -25,
  rotY: 35,
  zoom: 1.0,
  panX: 0,
  panY: 0,
})

// 鼠标拖拽
let isDragging = false
let dragStartX = 0
let dragStartY = 0
let dragStartRotX = 0
let dragStartRotY = 0
let dragStartPanX = 0
let dragStartPanY = 0
let isRightDrag = false

// 播放定时器
let playTimer: ReturnType<typeof setInterval> | null = null

// ============ 常量 ============

const coloringModes: { value: ColoringMode; label: string }[] = [
  { value: 'element', label: '元素' },
  { value: 'velocity', label: '速度' },
  { value: 'stress', label: '应力' },
  { value: 'displacement', label: '位移' },
  { value: 'type', label: '类型' },
  { value: 'charge', label: '电荷' },
  { value: 'kinetic_energy', label: '动能' },
  { value: 'potential_energy', label: '势能' },
]

// 元素颜色表 (CPK 风格)
const elementColors: Record<string, string> = {
  Al: '#B0B0B0',
  Cu: '#D4760A',
  Fe: '#E06633',
  Ni: '#50D050',
  Au: '#FFD123',
  Ag: '#C0C0C0',
  Pt: '#D0D0E0',
  Si: '#F0C8A0',
  C: '#404040',
  O: '#FF2020',
  N: '#3050F8',
  H: '#FFFFFF',
  Mg: '#8AFF00',
  Ti: '#BFC2C7',
  Zn: '#7D80B0',
  default: '#808080',
}

// ============ 计算属性 ============

const totalFrames = computed(() => trajectoryResult.value?.total_frames ?? 0)

const currentFrame = computed<TrajectoryFrame | null>(() => {
  if (!trajectoryResult.value) return null
  return trajectoryResult.value.frames[currentFrameIndex.value] ?? null
})

const coloringModeLabel = computed(() => {
  const map: Record<ColoringMode, string> = {
    element: '元素',
    velocity: '速度 (A/fs)',
    stress: '应力 (GPa)',
    displacement: '位移 (A)',
    type: '类型',
    charge: '电荷 (e)',
    kinetic_energy: '动能 (eV)',
    potential_energy: '势能 (eV)',
  }
  return map[coloringMode.value]
})

const colorGradient = computed(() => {
  return 'linear-gradient(to right, #3B82F6, #22C55E, #F59E0B, #EF4444)'
})

const colorMin = computed(() => '0.00')
const colorMax = computed(() => '1.00')

const elementLegend = computed(() => {
  if (!trajectoryResult.value) return []
  return trajectoryResult.value.elements.map(el => ({
    name: el,
    color: elementColors[el] || elementColors.default,
  }))
})

// ============ 模拟数据生成 ============

function generateFCCAluminum(): TrajectoryResult {
  const a = 4.05 // FCC Al 晶格常数 (A)
  const nx = 5
  const ny = 5
  const nz = 4
  const numFrames = 100

  // FCC 基矢
  const basis: [number, number, number][] = [
    [0, 0, 0],
    [0.5, 0.5, 0],
    [0.5, 0, 0.5],
    [0, 0.5, 0.5],
  ]

  // 生成初始原子位置
  const baseAtoms: { x: number; y: number; z: number }[] = []
  let atomId = 0
  for (let ix = 0; ix < nx; ix++) {
    for (let iy = 0; iy < ny; iy++) {
      for (let iz = 0; iz < nz; iz++) {
        for (const [bx, by, bz] of basis) {
          baseAtoms.push({
            x: (ix + bx) * a,
            y: (iy + by) * a,
            z: (iz + bz) * a,
          })
          atomId++
        }
      }
    }
  }

  const numAtoms = baseAtoms.length
  const boxLx = nx * a
  const boxLy = ny * a
  const boxLz = nz * a
  const dt = 1.0 // fs

  // 简谐振动参数
  const amplitude = 0.08 // A
  const omega = 0.05 // rad/fs

  // 为每个原子生成随机相位
  const phases = baseAtoms.map(() => ({
    px: Math.random() * Math.PI * 2,
    py: Math.random() * Math.PI * 2,
    pz: Math.random() * Math.PI * 2,
    fx: Math.random() * Math.PI * 2,
    fy: Math.random() * Math.PI * 2,
    fz: Math.random() * Math.PI * 2,
  }))

  const frames: TrajectoryFrame[] = []
  const kB = 8.617333e-5 // eV/K
  const targetTemp = 300 // K
  const mass = 26.98 // g/mol
  const massAmu = mass / 1.66054 // amu -> g/mol to amu

  for (let f = 0; f < numFrames; f++) {
    const t = f * dt
    const atoms: AtomData[] = []

    let totalKE = 0
    let totalPE = 0

    for (let i = 0; i < numAtoms; i++) {
      const base = baseAtoms[i]
      const ph = phases[i]

      // 简谐振动位移
      const dx = amplitude * Math.sin(omega * t + ph.px)
      const dy = amplitude * Math.sin(omega * t + ph.py)
      const dz = amplitude * Math.sin(omega * t + ph.pz)

      // 速度
      const vx = amplitude * omega * Math.cos(omega * t + ph.fx)
      const vy = amplitude * omega * Math.cos(omega * t + ph.fy)
      const vz = amplitude * omega * Math.cos(omega * t + ph.fz)

      // 力 (简谐: F = -k*x)
      const k = massAmu * omega * omega // 力常数
      const fx = -k * dx
      const fy = -k * dy
      const fz = -k * dz

      // 动能
      const ke = 0.5 * massAmu * (vx * vx + vy * vy + vz * vz)
      totalKE += ke

      // 势能 (简谐: PE = 0.5*k*r^2)
      const pe = 0.5 * k * (dx * dx + dy * dy + dz * dz)
      totalPE += pe

      atoms.push({
        id: i + 1,
        element: 'Al',
        x: base.x + dx,
        y: base.y + dy,
        z: base.z + dz,
        vx, vy, vz,
        fx, fy, fz,
      })
    }

    // 温度 (从动能计算)
    const dof = 3 * numAtoms - 3
    const temperature = (2 * totalKE) / (dof * kB)

    // 压力 (理想气体近似 + 维里修正)
    const volume = boxLx * boxLy * boxLz
    const pressure = (numAtoms * kB * temperature) / volume * 1.602176634e4 // eV/A^3 -> GPa

    // 能量漂移 (添加微小线性漂移)
    const drift = f * 1e-6

    frames.push({
      frame_number: f,
      num_atoms: numAtoms,
      atoms,
      box: { lx: boxLx, ly: boxLy, lz: boxLz },
      time_fs: t,
      temperature,
      total_energy: totalKE + totalPE + drift,
      kinetic_energy: totalKE,
      potential_energy: totalPE + drift,
      pressure,
      volume,
    })
  }

  return {
    success: true,
    frames,
    total_frames: numFrames,
    total_atoms: numAtoms,
    box_size: { lx: boxLx, ly: boxLy, lz: boxLz },
    time_range: { min: 0, max: (numFrames - 1) * dt },
    elements: ['Al'],
  }
}

function generateTimeSeries(result: TrajectoryResult): StressEnergyResult {
  const tempData: { step: number; time_fs: number; value: number }[] = []
  const presData: { step: number; time_fs: number; value: number }[] = []
  const totalEData: { step: number; time_fs: number; value: number }[] = []
  const kinEData: { step: number; time_fs: number; value: number }[] = []
  const potEData: { step: number; time_fs: number; value: number }[] = []
  const volData: { step: number; time_fs: number; value: number }[] = []
  const virialData: { step: number; time_fs: number; value: number }[] = []

  for (const frame of result.frames) {
    const step = frame.frame_number
    const t = frame.time_fs
    tempData.push({ step, time_fs: t, value: frame.temperature })
    presData.push({ step, time_fs: t, value: frame.pressure })
    totalEData.push({ step, time_fs: t, value: frame.total_energy })
    kinEData.push({ step, time_fs: t, value: frame.kinetic_energy })
    potEData.push({ step, time_fs: t, value: frame.potential_energy })
    volData.push({ step, time_fs: t, value: frame.volume })
    // 维里应力近似
    virialData.push({ step, time_fs: t, value: frame.pressure * 0.95 + (Math.random() - 0.5) * 0.02 })
  }

  return {
    temperature_series: { label: '温度', unit: 'K', data: tempData },
    pressure_series: { label: '压力', unit: 'GPa', data: presData },
    total_energy_series: { label: '总能量', unit: 'eV', data: totalEData },
    kinetic_energy_series: { label: '动能', unit: 'eV', data: kinEData },
    potential_energy_series: { label: '势能', unit: 'eV', data: potEData },
    volume_series: { label: '体积', unit: 'A^3', data: volData },
    virial_stress_series: { label: '维里应力', unit: 'GPa', data: virialData },
  }
}

function computeStats(result: TrajectoryResult): TrajectoryStats {
  let sumTemp = 0
  let sumPressure = 0
  let firstEnergy = 0
  let lastEnergy = 0
  const n = result.frames.length

  for (let i = 0; i < n; i++) {
    const f = result.frames[i]
    sumTemp += f.temperature
    sumPressure += f.pressure
    if (i === 0) firstEnergy = f.total_energy
    if (i === n - 1) lastEnergy = f.total_energy
  }

  const volume = result.box_size.lx * result.box_size.ly * result.box_size.lz
  const massTotal = result.total_atoms * 26.98 / 6.02214076e23 // g
  const density = (massTotal / volume) * 1e24 // g/cm^3

  return {
    avg_temp: sumTemp / n,
    avg_pressure: sumPressure / n,
    energy_drift: Math.abs(lastEnergy - firstEnergy),
    density,
  }
}

// ============ 操作方法 ============

function handleLoadDemo() {
  const result = generateFCCAluminum()
  trajectoryResult.value = result
  currentFrameIndex.value = 0
  timeSeriesData.value = generateTimeSeries(result)
  trajectoryStats.value = computeStats(result)

  nextTick(() => {
    renderAtoms()
    renderTimeSeriesCharts()
  })
}

function handleExportFrame() {
  if (!currentFrame.value || !atomCanvas.value) return
  const link = document.createElement('a')
  link.download = `frame_${currentFrame.value.frame_number}.png`
  link.href = atomCanvas.value.toDataURL('image/png')
  link.click()
}

function handleExportCSV() {
  if (!timeSeriesData.value) return
  const ts = timeSeriesData.value
  let csv = 'step,time_fs,temperature,pressure,total_energy,kinetic_energy,potential_energy,volume\n'
  for (let i = 0; i < ts.temperature_series.data.length; i++) {
    csv += `${ts.temperature_series.data[i].step},` +
      `${ts.temperature_series.data[i].time_fs},` +
      `${ts.temperature_series.data[i].value},` +
      `${ts.pressure_series.data[i].value},` +
      `${ts.total_energy_series.data[i].value},` +
      `${ts.kinetic_energy_series.data[i].value},` +
      `${ts.potential_energy_series.data[i].value},` +
      `${ts.volume_series.data[i].value}\n`
  }
  const blob = new Blob([csv], { type: 'text/csv' })
  const link = document.createElement('a')
  link.download = 'timeseries.csv'
  link.href = URL.createObjectURL(blob)
  link.click()
  URL.revokeObjectURL(link.href)
}

function handleScreenshot() {
  if (!atomCanvas.value) return
  const link = document.createElement('a')
  link.download = `trajectory_screenshot_${Date.now()}.png`
  link.href = atomCanvas.value.toDataURL('image/png')
  link.click()
}

function togglePlay() {
  isPlaying.value = !isPlaying.value
  if (isPlaying.value) {
    startPlayback()
  } else {
    stopPlayback()
  }
}

function startPlayback() {
  stopPlayback()
  const intervalMs = Math.max(16, 100 / playbackSpeed.value)
  playTimer = setInterval(() => {
    if (currentFrameIndex.value < totalFrames.value - 1) {
      currentFrameIndex.value++
      renderAtoms()
    } else {
      currentFrameIndex.value = 0
      renderAtoms()
    }
  }, intervalMs)
}

function stopPlayback() {
  if (playTimer) {
    clearInterval(playTimer)
    playTimer = null
  }
}

function prevFrame() {
  if (currentFrameIndex.value > 0) {
    currentFrameIndex.value--
    renderAtoms()
  }
}

function nextFrame() {
  if (currentFrameIndex.value < totalFrames.value - 1) {
    currentFrameIndex.value++
    renderAtoms()
  }
}

// ============ 文件拖拽 ============

function onDragOver(e: DragEvent) {
  (e.currentTarget as HTMLElement).style.borderColor = 'var(--primary)'
}

function onDragLeave(e: DragEvent) {
  (e.currentTarget as HTMLElement).style.borderColor = 'var(--border-default)'
}

function onDrop(e: DragEvent) {
  (e.currentTarget as HTMLElement).style.borderColor = 'var(--border-default)'
  const files = e.dataTransfer?.files
  if (files && files.length > 0) {
    handleLoadDemo() // 演示模式: 直接加载模拟数据
  }
}

function onFileClick() {
  fileInput.value?.click()
}

function onFileSelected(e: Event) {
  const input = e.target as HTMLInputElement
  if (input.files && input.files.length > 0) {
    handleLoadDemo() // 演示模式: 直接加载模拟数据
  }
}

// ============ Canvas 鼠标交互 ============

function onCanvasMouseDown(e: MouseEvent) {
  isDragging = true
  isRightDrag = e.button === 2
  dragStartX = e.clientX
  dragStartY = e.clientY
  dragStartRotX = camera.rotX
  dragStartRotY = camera.rotY
  dragStartPanX = camera.panX
  dragStartPanY = camera.panY
}

function onCanvasMouseMove(e: MouseEvent) {
  if (!isDragging) return
  const dx = e.clientX - dragStartX
  const dy = e.clientY - dragStartY

  if (isRightDrag) {
    camera.panX = dragStartPanX + dx * 0.5
    camera.panY = dragStartPanY + dy * 0.5
  } else {
    camera.rotY = dragStartRotY + dx * 0.5
    camera.rotX = dragStartRotX + dy * 0.5
    camera.rotX = Math.max(-90, Math.min(90, camera.rotX))
  }

  renderAtoms()
}

function onCanvasMouseUp() {
  isDragging = false
  isRightDrag = false
}

function onCanvasWheel(e: WheelEvent) {
  const delta = e.deltaY > 0 ? 0.9 : 1.1
  camera.zoom = Math.max(0.1, Math.min(10, camera.zoom * delta))
  renderAtoms()
}

// ============ 3D 渲染 ============

function project3D(x: number, y: number, z: number, cx: number, cy: number, scale: number): { px: number; py: number; depth: number } {
  // 中心化
  const bx = trajectoryResult.value ? trajectoryResult.value.box_size.lx / 2 : 0
  const by = trajectoryResult.value ? trajectoryResult.value.box_size.ly / 2 : 0
  const bz = trajectoryResult.value ? trajectoryResult.value.box_size.lz / 2 : 0
  const rx = x - bx
  const ry = y - by
  const rz = z - bz

  // 旋转 (Y轴)
  const radY = (camera.rotY * Math.PI) / 180
  const cosY = Math.cos(radY)
  const sinY = Math.sin(radY)
  const x1 = rx * cosY + rz * sinY
  const z1 = -rx * sinY + rz * cosY

  // 旋转 (X轴)
  const radX = (camera.rotX * Math.PI) / 180
  const cosX = Math.cos(radX)
  const sinX = Math.sin(radX)
  const y1 = ry * cosX - z1 * sinX
  const z2 = ry * sinX + z1 * cosX

  // 透视投影
  const perspective = 800
  const s = perspective / (perspective + z2 * scale * camera.zoom)

  return {
    px: cx + x1 * scale * camera.zoom * s + camera.panX,
    py: cy - y1 * scale * camera.zoom * s + camera.panY,
    depth: z2,
  }
}

function getAtomColor(atom: AtomData, frame: TrajectoryFrame): string {
  if (coloringMode.value === 'element') {
    return elementColors[atom.element] || elementColors.default
  }

  // 计算着色值
  let value = 0
  switch (coloringMode.value) {
    case 'velocity': {
      const v = Math.sqrt(atom.vx * atom.vx + atom.vy * atom.vy + atom.vz * atom.vz)
      value = v
      break
    }
    case 'stress': {
      const f = Math.sqrt(atom.fx * atom.fx + atom.fy * atom.fy + atom.fz * atom.fz)
      value = f * 0.1
      break
    }
    case 'displacement': {
      const bx = trajectoryResult.value ? trajectoryResult.value.box_size.lx / 2 : 0
      const by = trajectoryResult.value ? trajectoryResult.value.box_size.ly / 2 : 0
      const bz = trajectoryResult.value ? trajectoryResult.value.box_size.lz / 2 : 0
      // 假设平衡位置在晶格点上，位移就是偏离量
      const dx = atom.x - Math.round(atom.x / 4.05) * 4.05
      const dy = atom.y - Math.round(atom.y / 4.05) * 4.05
      const dz = atom.z - Math.round(atom.z / 4.05) * 4.05
      value = Math.sqrt(dx * dx + dy * dy + dz * dz)
      break
    }
    case 'type': {
      value = (atom.id % 5) / 4
      break
    }
    case 'charge': {
      value = 0.5 + 0.5 * Math.sin(atom.id * 0.1)
      break
    }
    case 'kinetic_energy': {
      const v = Math.sqrt(atom.vx * atom.vx + atom.vy * atom.vy + atom.vz * atom.vz)
      value = 0.5 * 26.98 / 6.02214076e23 * 1e3 * v * v
      break
    }
    case 'potential_energy': {
      value = 0.5 + 0.5 * Math.cos(atom.id * 0.05)
      break
    }
  }

  // 归一化到 [0, 1]
  const maxVal = 0.02
  const t = Math.min(1, Math.max(0, value / maxVal))

  // 蓝 -> 绿 -> 黄 -> 红
  return valueToColor(t)
}

function valueToColor(t: number): string {
  let r: number, g: number, b: number
  if (t < 0.33) {
    const s = t / 0.33
    r = Math.round(59 + (34 - 59) * s)
    g = Math.round(130 + (197 - 130) * s)
    b = Math.round(246 + (94 - 246) * s)
  } else if (t < 0.66) {
    const s = (t - 0.33) / 0.33
    r = Math.round(34 + (245 - 34) * s)
    g = Math.round(197 + (158 - 197) * s)
    b = Math.round(94 + (11 - 94) * s)
  } else {
    const s = (t - 0.66) / 0.34
    r = Math.round(245 + (239 - 245) * s)
    g = Math.round(158 + (68 - 158) * s)
    b = Math.round(11 + (68 - 11) * s)
  }
  return `rgb(${r},${g},${b})`
}

function renderAtoms() {
  const canvas = atomCanvas.value
  if (!canvas || !currentFrame.value || !trajectoryResult.value) return

  const rect = canvas.parentElement!.getBoundingClientRect()
  const dpr = window.devicePixelRatio || 1
  canvas.width = rect.width * dpr
  canvas.height = rect.height * dpr
  canvas.style.width = rect.width + 'px'
  canvas.style.height = rect.height + 'px'

  const ctx = canvas.getContext('2d')!
  ctx.scale(dpr, dpr)

  const w = rect.width
  const h = rect.height
  const cx = w / 2
  const cy = h / 2

  // 计算缩放
  const boxDiag = Math.sqrt(
    trajectoryResult.value.box_size.lx ** 2 +
    trajectoryResult.value.box_size.ly ** 2 +
    trajectoryResult.value.box_size.lz ** 2
  )
  const scale = (Math.min(w, h) * 0.6) / boxDiag

  // 清空
  ctx.fillStyle = '#1a1a2e'
  ctx.fillRect(0, 0, w, h)

  const frame = currentFrame.value

  // 绘制盒子线框
  if (showBox.value) {
    const { lx, ly, lz } = frame.box
    const corners: [number, number, number][] = [
      [0, 0, 0], [lx, 0, 0], [lx, ly, 0], [0, ly, 0],
      [0, 0, lz], [lx, 0, lz], [lx, ly, lz], [0, ly, lz],
    ]
    const edges: [number, number][] = [
      [0, 1], [1, 2], [2, 3], [3, 0],
      [4, 5], [5, 6], [6, 7], [7, 4],
      [0, 4], [1, 5], [2, 6], [3, 7],
    ]

    ctx.strokeStyle = 'rgba(255, 255, 255, 0.3)'
    ctx.lineWidth = 1
    ctx.beginPath()
    for (const [a, b] of edges) {
      const pa = project3D(corners[a][0], corners[a][1], corners[a][2], cx, cy, scale)
      const pb = project3D(corners[b][0], corners[b][1], corners[b][2], cx, cy, scale)
      ctx.moveTo(pa.px, pa.py)
      ctx.lineTo(pb.px, pb.py)
    }
    ctx.stroke()
  }

  // 投影所有原子
  const projectedAtoms = frame.atoms.map(atom => {
    const p = project3D(atom.x, atom.y, atom.z, cx, cy, scale)
    return { atom, ...p, color: getAtomColor(atom, frame) }
  })

  // 按深度排序 (远的先画)
  projectedAtoms.sort((a, b) => a.depth - b.depth)

  // 绘制键
  if (showBonds.value) {
    const cutoff2 = bondCutoff.value * bondCutoff.value
    ctx.strokeStyle = 'rgba(180, 180, 180, 0.25)'
    ctx.lineWidth = 0.5

    // 仅检查近邻 (简单 O(n^2) 限制)
    const atoms = frame.atoms
    const n = Math.min(atoms.length, 2000) // 性能限制
    for (let i = 0; i < n; i++) {
      for (let j = i + 1; j < n; j++) {
        const dx = atoms[i].x - atoms[j].x
        const dy = atoms[i].y - atoms[j].y
        const dz = atoms[i].z - atoms[j].z
        const d2 = dx * dx + dy * dy + dz * dz
        if (d2 < cutoff2) {
          const pi = project3D(atoms[i].x, atoms[i].y, atoms[i].z, cx, cy, scale)
          const pj = project3D(atoms[j].x, atoms[j].y, atoms[j].z, cx, cy, scale)
          ctx.beginPath()
          ctx.moveTo(pi.px, pi.py)
          ctx.lineTo(pj.px, pj.py)
          ctx.stroke()
        }
      }
    }
  }

  // 绘制原子
  const baseRadius = scale * 0.8 * atomScale.value
  for (const pa of projectedAtoms) {
    const r = Math.max(1, baseRadius * (800 / (800 + pa.depth * scale * camera.zoom)))

    // 球体渐变效果
    const gradient = ctx.createRadialGradient(
      pa.px - r * 0.3, pa.py - r * 0.3, r * 0.1,
      pa.px, pa.py, r
    )
    gradient.addColorStop(0, lightenColor(pa.color, 60))
    gradient.addColorStop(0.7, pa.color)
    gradient.addColorStop(1, darkenColor(pa.color, 40))

    ctx.beginPath()
    ctx.arc(pa.px, pa.py, r, 0, Math.PI * 2)
    ctx.fillStyle = gradient
    ctx.fill()
  }

  // 帧信息
  ctx.fillStyle = 'rgba(255, 255, 255, 0.7)'
  ctx.font = '11px monospace'
  ctx.fillText(`Frame: ${frame.frame_number}  Time: ${frame.time_fs.toFixed(1)} fs  Atoms: ${frame.num_atoms}`, 10, 20)
  ctx.fillText(`T: ${frame.temperature.toFixed(1)} K  P: ${frame.pressure.toFixed(4)} GPa  E: ${frame.total_energy.toFixed(3)} eV`, 10, 36)
}

function lightenColor(color: string, amount: number): string {
  const rgb = parseColor(color)
  return `rgb(${Math.min(255, rgb.r + amount)},${Math.min(255, rgb.g + amount)},${Math.min(255, rgb.b + amount)})`
}

function darkenColor(color: string, amount: number): string {
  const rgb = parseColor(color)
  return `rgb(${Math.max(0, rgb.r - amount)},${Math.max(0, rgb.g - amount)},${Math.max(0, rgb.b - amount)})`
}

function parseColor(color: string): { r: number; g: number; b: number } {
  if (color.startsWith('rgb(')) {
    const parts = color.slice(4, -1).split(',').map(Number)
    return { r: parts[0], g: parts[1], b: parts[2] }
  }
  if (color.startsWith('#')) {
    const hex = color.slice(1)
    return {
      r: parseInt(hex.slice(0, 2), 16),
      g: parseInt(hex.slice(2, 4), 16),
      b: parseInt(hex.slice(4, 6), 16),
    }
  }
  return { r: 128, g: 128, b: 128 }
}

// ============ 时序曲线 SVG 渲染 ============

function renderTimeSeriesCharts() {
  if (!timeSeriesData.value) return
  const ts = timeSeriesData.value
  nextTick(() => {
    renderSingleChart(
      tempSvg.value,
      [ts.temperature_series],
      ['#EF4444'],
      'K'
    )
    renderSingleChart(
      energySvg.value,
      [
        ts.total_energy_series,
        ts.kinetic_energy_series,
        ts.potential_energy_series,
      ],
      ['#3B82F6', '#22C55E', '#F59E0B'],
      'eV'
    )
    renderSingleChart(
      pressureSvg.value,
      [ts.pressure_series],
      ['#8B5CF6'],
      'GPa'
    )
    renderSingleChart(
      volumeSvg.value,
      [ts.volume_series],
      ['#06B6D4'],
      'A^3'
    )
  })
}

function renderSingleChart(
  svgEl: SVGSVGElement | null,
  series: TimeSeriesData[],
  colors: string[],
  unit: string
) {
  if (!svgEl || series.length === 0) return

  const rect = svgEl.parentElement!.getBoundingClientRect()
  const w = rect.width
  const h = rect.height
  const margin = { top: 4, right: 4, bottom: 14, left: 32 }
  const plotW = w - margin.left - margin.right
  const plotH = h - margin.top - margin.bottom

  svgEl.setAttribute('width', String(w))
  svgEl.setAttribute('height', String(h))
  svgEl.setAttribute('viewBox', `0 0 ${w} ${h}`)

  // 数据范围
  let allMin = Infinity
  let allMax = -Infinity
  let xMin = Infinity
  let xMax = -Infinity

  for (const s of series) {
    for (const p of s.data) {
      if (p.value < allMin) allMin = p.value
      if (p.value > allMax) allMax = p.value
      if (p.time_fs < xMin) xMin = p.time_fs
      if (p.time_fs > xMax) xMax = p.time_fs
    }
  }

  const yRange = allMax - allMin || 1
  const yPad = yRange * 0.1
  allMin -= yPad
  allMax += yPad
  const xRange = xMax - xMin || 1

  const toX = (v: number) => margin.left + ((v - xMin) / xRange) * plotW
  const toY = (v: number) => margin.top + plotH - ((v - allMin) / (allMax - allMin)) * plotH

  let paths = ''

  // 绘制网格线
  paths += `<line x1="${margin.left}" y1="${margin.top}" x2="${margin.left}" y2="${margin.top + plotH}" stroke="var(--border-subtle)" stroke-width="0.5"/>`
  paths += `<line x1="${margin.left}" y1="${margin.top + plotH}" x2="${margin.left + plotW}" y2="${margin.top + plotH}" stroke="var(--border-subtle)" stroke-width="0.5"/>`

  // Y轴刻度
  const yTicks = 3
  for (let i = 0; i <= yTicks; i++) {
    const val = allMin + (allMax - allMin) * (i / yTicks)
    const y = toY(val)
    paths += `<line x1="${margin.left}" y1="${y}" x2="${margin.left + plotW}" y2="${y}" stroke="var(--border-subtle)" stroke-width="0.3" stroke-dasharray="2,2"/>`
    const label = val >= 1000 || val <= -1000 ? val.toExponential(1) : val.toFixed(2)
    paths += `<text x="${margin.left - 2}" y="${y + 3}" text-anchor="end" fill="var(--text-muted)" font-size="7" font-family="monospace">${label}</text>`
  }

  // X轴刻度
  const xTicks = 3
  for (let i = 0; i <= xTicks; i++) {
    const val = xMin + xRange * (i / xTicks)
    const x = toX(val)
    paths += `<text x="${x}" y="${margin.top + plotH + 10}" text-anchor="middle" fill="var(--text-muted)" font-size="7" font-family="monospace">${val.toFixed(0)}</text>`
  }

  // 绘制数据线
  for (let si = 0; si < series.length; si++) {
    const s = series[si]
    const color = colors[si % colors.length]
    const step = Math.max(1, Math.floor(s.data.length / 200)) // 降采样

    let d = ''
    for (let i = 0; i < s.data.length; i += step) {
      const p = s.data[i]
      const x = toX(p.time_fs)
      const y = toY(p.value)
      d += (i === 0 ? 'M' : 'L') + `${x.toFixed(1)},${y.toFixed(1)}`
    }

    paths += `<path d="${d}" fill="none" stroke="${color}" stroke-width="1" stroke-linejoin="round"/>`
  }

  // 单位标签
  paths += `<text x="${margin.left + 2}" y="${margin.top + 8}" fill="var(--text-muted)" font-size="6" font-family="monospace">${unit}</text>`

  svgEl.innerHTML = paths
}

// ============ 侦听器 ============

watch(currentFrameIndex, () => {
  renderAtoms()
})

watch(coloringMode, () => {
  renderAtoms()
})

watch(atomScale, () => {
  renderAtoms()
})

watch(showBonds, () => {
  renderAtoms()
})

watch(bondCutoff, () => {
  renderAtoms()
})

watch(showBox, () => {
  renderAtoms()
})

watch(playbackSpeed, () => {
  if (isPlaying.value) {
    stopPlayback()
    startPlayback()
  }
})

// ============ 生命周期 ============

onMounted(() => {
  // 禁用右键菜单
  atomCanvas.value?.addEventListener('contextmenu', (e) => e.preventDefault())
})

onUnmounted(() => {
  stopPlayback()
})
</script>

<style scoped>
.trajectory-view {
  font-family: var(--font-ui);
}
</style>
