<!--
  相场 GPU 加速视图 - PhaseFieldGpuView.vue
  V1.6-007: CUDA / OpenCL / WebGPU，实时可视化 > 30fps
-->
<template>
  <div class="h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-lg font-semibold text-[var(--text-primary)]">相场 GPU 加速</h2>
        <p class="text-sm text-[var(--text-secondary)]">CUDA / OpenCL / WebGPU，实时可视化 > 30fps</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="handleDetectDevices" :disabled="detecting" class="btn btn-ghost text-xs">
          {{ detecting ? '检测中...' : '检测设备' }}
        </button>
        <button @click="handleRunBenchmark" :disabled="benchmarking" class="btn btn-ghost text-xs">
          {{ benchmarking ? '测试中...' : '运行基准测试' }}
        </button>
        <button @click="handleConfigureSolver" :disabled="configuring" class="btn btn-primary text-xs">
          {{ configuring ? '配置中...' : '配置求解器' }}
        </button>
      </div>
    </div>

    <div class="flex-1 flex overflow-hidden">
      <!-- 左侧面板 -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] overflow-y-auto flex-shrink-0">
        <!-- GPU 设备列表 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('devices')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-elevated)]">
            <span class="font-medium text-[var(--text-primary)] text-sm">GPU 设备列表</span>
            <span class="text-[var(--text-muted)] text-xs">{{ sections.devices ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.devices" class="px-4 pb-4 space-y-2">
            <div v-for="(device, idx) in devices" :key="idx"
              @click="selectDevice(idx)"
              :class="['rounded-lg border p-3 cursor-pointer transition-colors',
                selectedDeviceIdx === idx
                  ? 'border-[var(--primary)] bg-[var(--primary)]/5'
                  : 'border-[var(--border-subtle)] hover:border-[var(--border-default)] bg-[var(--bg-elevated)]']">
              <div class="flex items-center justify-between mb-1">
                <span class="text-xs font-medium text-[var(--text-primary)]">{{ device.name }}</span>
                <span class="text-[10px] text-[var(--text-muted)]">ID: {{ idx }}</span>
              </div>
              <div class="text-[10px] text-[var(--text-secondary)] mb-1">{{ device.vendor }}</div>
              <div class="grid grid-cols-2 gap-1 text-[10px]">
                <div class="text-[var(--text-muted)]">显存</div>
                <div class="text-[var(--text-secondary)]">{{ device.available_memory_mb }} / {{ device.total_memory_mb }} MB</div>
                <div class="text-[var(--text-muted)]">算力</div>
                <div class="text-[var(--text-secondary)]">{{ device.compute_capability }}</div>
                <div class="text-[var(--text-muted)]">驱动</div>
                <div class="text-[var(--text-secondary)]">{{ device.driver_version }}</div>
              </div>
            </div>
            <div v-if="devices.length === 0" class="py-4 text-center text-xs text-[var(--text-muted)]">
              点击"检测设备"以扫描 GPU
            </div>
          </div>
        </div>

        <!-- 求解器配置 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('solver')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-elevated)]">
            <span class="font-medium text-[var(--text-primary)] text-sm">求解器配置</span>
            <span class="text-[var(--text-muted)] text-xs">{{ sections.solver ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.solver" class="px-4 pb-4 space-y-3">
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">计算后端</label>
              <select v-model="solverConfig.backend" class="input w-full text-xs">
                <option value="cuda">CUDA</option>
                <option value="opencl">OpenCL</option>
                <option value="webgpu">WebGPU</option>
                <option value="wasm_simd">WASM SIMD</option>
              </select>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">块大小 X</label>
                <input v-model.number="solverConfig.block_size.x" type="number" min="1" max="1024" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">块大小 Y</label>
                <input v-model.number="solverConfig.block_size.y" type="number" min="1" max="1024" class="input w-full text-xs" />
              </div>
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">精度</label>
              <select v-model="solverConfig.precision" class="input w-full text-xs">
                <option value="float32">float32 (单精度)</option>
                <option value="float64">float64 (双精度)</option>
              </select>
            </div>
            <div>
              <label class="text-[10px] text-[var(--text-muted)]">设备 ID</label>
              <input v-model.number="solverConfig.device_id" type="number" min="0" class="input w-full text-xs" />
            </div>
            <div class="space-y-2">
              <label class="flex items-center gap-2 cursor-pointer">
                <input type="checkbox" v-model="solverConfig.use_shared_memory" class="rounded" />
                <span class="text-xs text-[var(--text-secondary)]">共享内存</span>
              </label>
              <label class="flex items-center gap-2 cursor-pointer">
                <input type="checkbox" v-model="solverConfig.use_texture_memory" class="rounded" />
                <span class="text-xs text-[var(--text-secondary)]">纹理内存</span>
              </label>
              <label class="flex items-center gap-2 cursor-pointer">
                <input type="checkbox" v-model="solverConfig.async_compute" class="rounded" />
                <span class="text-xs text-[var(--text-secondary)]">异步计算</span>
              </label>
            </div>
          </div>
        </div>

        <!-- 基准测试配置 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('benchmark')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-elevated)]">
            <span class="font-medium text-[var(--text-primary)] text-sm">基准测试</span>
            <span class="text-[var(--text-muted)] text-xs">{{ sections.benchmark ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.benchmark" class="px-4 pb-4 space-y-3">
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">网格大小</label>
              <div class="grid grid-cols-4 gap-1">
                <button v-for="size in gridSizes" :key="size.label"
                  @click="benchmarkGridSize = size.value"
                  :class="['px-2 py-1.5 text-[10px] rounded border transition-colors',
                    benchmarkGridSize === size.value
                      ? 'border-[var(--primary)] bg-[var(--primary)]/10 text-[var(--primary)]'
                      : 'border-[var(--border-subtle)] text-[var(--text-secondary)] hover:border-[var(--border-default)]']">
                  {{ size.label }}
                </button>
              </div>
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">步数</label>
              <input v-model.number="benchmarkSteps" type="number" min="100" max="100000" step="100" class="input w-full text-xs" />
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧区域 -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- 性能对比面板 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-3">性能对比</h3>
          <div class="space-y-2">
            <!-- CPU vs GPU 条形图 -->
            <div v-for="result in benchmarkResults" :key="result.backend" class="flex items-center gap-3">
              <span class="text-[10px] text-[var(--text-muted)] w-20 text-right">{{ backendLabel(result.backend) }}</span>
              <div class="flex-1 flex items-center gap-1">
                <div class="flex-1 h-5 bg-[var(--bg-elevated)] rounded overflow-hidden relative">
                  <div class="h-full bg-[var(--accent-yellow)] rounded-l transition-all duration-500 absolute left-0 top-0"
                    :style="{ width: cpuBarWidth(result) + '%' }"></div>
                  <div class="h-full bg-[var(--primary)] rounded-r transition-all duration-500 absolute left-0 top-0"
                    :style="{ width: gpuBarWidth(result) + '%', opacity: 0.85 }"></div>
                </div>
              </div>
              <span class="text-[10px] text-[var(--accent-green)] w-16 text-right font-medium">{{ result.speedup_vs_cpu.toFixed(1) }}x</span>
              <span class="text-[10px] text-[var(--text-secondary)] w-20 text-right">{{ result.steps_per_second.toFixed(0) }} steps/s</span>
            </div>
            <div v-if="benchmarkResults.length === 0" class="py-4 text-center text-xs text-[var(--text-muted)]">
              运行基准测试后显示对比数据
            </div>
          </div>
          <!-- 图例 -->
          <div v-if="benchmarkResults.length > 0" class="flex items-center gap-4 mt-2">
            <div class="flex items-center gap-1">
              <div class="w-3 h-3 bg-[var(--accent-yellow)] rounded-sm"></div>
              <span class="text-[10px] text-[var(--text-muted)]">CPU</span>
            </div>
            <div class="flex items-center gap-1">
              <div class="w-3 h-3 bg-[var(--primary)] rounded-sm"></div>
              <span class="text-[10px] text-[var(--text-muted)]">GPU</span>
            </div>
          </div>
        </div>

        <!-- 实时性能指标 + GPU 详细信息 -->
        <div class="flex-1 overflow-auto px-4 py-3">
          <div class="grid grid-cols-2 gap-4">
            <!-- 实时性能指标 -->
            <div class="bg-[var(--bg-surface)] rounded-lg border border-[var(--border-subtle)] p-4">
              <h4 class="text-xs font-semibold text-[var(--text-primary)] mb-3">实时性能指标</h4>
              <div class="grid grid-cols-2 gap-3">
                <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
                  <div class="text-[10px] text-[var(--text-muted)]">Steps/s</div>
                  <div class="text-lg font-semibold text-[var(--primary)]">{{ realtimeMetrics.stepsPerSecond.toFixed(0) }}</div>
                </div>
                <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
                  <div class="text-[10px] text-[var(--text-muted)]">显存使用</div>
                  <div class="text-lg font-semibold text-[var(--accent-yellow)]">{{ realtimeMetrics.memoryUsedMb.toFixed(0) }} MB</div>
                </div>
                <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
                  <div class="text-[10px] text-[var(--text-muted)]">加速比</div>
                  <div class="text-lg font-semibold text-[var(--accent-green)]">{{ realtimeMetrics.speedup.toFixed(1) }}x</div>
                </div>
                <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
                  <div class="text-[10px] text-[var(--text-muted)]">温度</div>
                  <div class="text-lg font-semibold" :class="realtimeMetrics.temperature > 75 ? 'text-[var(--accent-red)]' : 'text-[var(--text-primary)]'">
                    {{ realtimeMetrics.temperature.toFixed(0) }}°C
                  </div>
                </div>
              </div>
            </div>

            <!-- GPU 详细信息卡片 -->
            <div class="bg-[var(--bg-surface)] rounded-lg border border-[var(--border-subtle)] p-4">
              <h4 class="text-xs font-semibold text-[var(--text-primary)] mb-3">GPU 详细信息</h4>
              <div class="space-y-3">
                <div>
                  <div class="flex items-center justify-between text-[10px] mb-1">
                    <span class="text-[var(--text-muted)]">占用率</span>
                    <span class="text-[var(--text-secondary)]">{{ performanceProfile.occupancy.toFixed(1) }}%</span>
                  </div>
                  <div class="w-full h-2 bg-[var(--bg-base)] rounded-full overflow-hidden">
                    <div class="h-full rounded-full bg-[var(--primary)] transition-all duration-500"
                      :style="{ width: performanceProfile.occupancy + '%' }"></div>
                  </div>
                </div>
                <div>
                  <div class="flex items-center justify-between text-[10px] mb-1">
                    <span class="text-[var(--text-muted)]">缓存命中率</span>
                    <span class="text-[var(--text-secondary)]">{{ performanceProfile.cache_hit_rate.toFixed(1) }}%</span>
                  </div>
                  <div class="w-full h-2 bg-[var(--bg-base)] rounded-full overflow-hidden">
                    <div class="h-full rounded-full bg-[var(--accent-green)] transition-all duration-500"
                      :style="{ width: performanceProfile.cache_hit_rate + '%' }"></div>
                  </div>
                </div>
                <div>
                  <div class="flex items-center justify-between text-[10px] mb-1">
                    <span class="text-[var(--text-muted)]">内存带宽</span>
                    <span class="text-[var(--text-secondary)]">{{ performanceProfile.memory_bandwidth_gb_s.toFixed(1) }} GB/s</span>
                  </div>
                  <div class="w-full h-2 bg-[var(--bg-base)] rounded-full overflow-hidden">
                    <div class="h-full rounded-full bg-[var(--accent-yellow)] transition-all duration-500"
                      :style="{ width: Math.min(100, performanceProfile.memory_bandwidth_gb_s / 8 * 100) + '%' }"></div>
                  </div>
                </div>
                <div class="grid grid-cols-3 gap-2 pt-1">
                  <div class="text-center">
                    <div class="text-[10px] text-[var(--text-muted)]">内核时间</div>
                    <div class="text-xs font-medium text-[var(--text-primary)]">{{ performanceProfile.kernel_time_ms.toFixed(2) }} ms</div>
                  </div>
                  <div class="text-center">
                    <div class="text-[10px] text-[var(--text-muted)]">传输时间</div>
                    <div class="text-xs font-medium text-[var(--text-primary)]">{{ performanceProfile.memory_transfer_ms.toFixed(2) }} ms</div>
                  </div>
                  <div class="text-center">
                    <div class="text-[10px] text-[var(--text-muted)]">总时间</div>
                    <div class="text-xs font-medium text-[var(--text-primary)]">{{ performanceProfile.total_time_ms.toFixed(2) }} ms</div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 性能分析饼图 -->
          <div class="mt-4 bg-[var(--bg-surface)] rounded-lg border border-[var(--border-subtle)] p-4">
            <h4 class="text-xs font-semibold text-[var(--text-primary)] mb-3">性能分析</h4>
            <div class="flex items-center gap-8">
              <svg width="160" height="160" viewBox="0 0 160 160" class="flex-shrink-0">
                <circle cx="80" cy="80" r="60" fill="none" stroke="var(--bg-elevated)" stroke-width="20" />
                <!-- 内核时间 -->
                <circle cx="80" cy="80" r="60" fill="none"
                  :stroke="'var(--primary)'"
                  stroke-width="20"
                  :stroke-dasharray="kernelArc"
                  stroke-dashoffset="0"
                  transform="rotate(-90 80 80)"
                  stroke-linecap="round" />
                <!-- 内存传输时间 -->
                <circle cx="80" cy="80" r="60" fill="none"
                  :stroke="'var(--accent-yellow)'"
                  stroke-width="20"
                  :stroke-dasharray="transferArc"
                  :stroke-dashoffset="-(kernelArcLength)"
                  transform="rotate(-90 80 80)"
                  stroke-linecap="round" />
                <text x="80" y="76" text-anchor="middle" class="fill-[var(--text-primary)]" font-size="12" font-weight="600">
                  {{ performanceProfile.total_time_ms.toFixed(1) }}ms
                </text>
                <text x="80" y="92" text-anchor="middle" class="fill-[var(--text-muted)]" font-size="9">
                  总耗时
                </text>
              </svg>
              <div class="flex-1 space-y-3">
                <div class="flex items-center gap-3">
                  <div class="w-3 h-3 rounded-sm bg-[var(--primary)] flex-shrink-0"></div>
                  <div class="flex-1">
                    <div class="flex items-center justify-between text-xs">
                      <span class="text-[var(--text-secondary)]">内核计算时间</span>
                      <span class="text-[var(--text-primary)] font-medium">{{ performanceProfile.kernel_time_ms.toFixed(2) }} ms</span>
                    </div>
                    <div class="text-[10px] text-[var(--text-muted)]">{{ kernelPercent.toFixed(1) }}%</div>
                  </div>
                </div>
                <div class="flex items-center gap-3">
                  <div class="w-3 h-3 rounded-sm bg-[var(--accent-yellow)] flex-shrink-0"></div>
                  <div class="flex-1">
                    <div class="flex items-center justify-between text-xs">
                      <span class="text-[var(--text-secondary)]">内存传输时间</span>
                      <span class="text-[var(--text-primary)] font-medium">{{ performanceProfile.memory_transfer_ms.toFixed(2) }} ms</span>
                    </div>
                    <div class="text-[10px] text-[var(--text-muted)]">{{ transferPercent.toFixed(1) }}%</div>
                  </div>
                </div>
                <div class="flex items-center gap-3">
                  <div class="w-3 h-3 rounded-sm bg-[var(--bg-elevated)] border border-[var(--border-subtle)] flex-shrink-0"></div>
                  <div class="flex-1">
                    <div class="flex items-center justify-between text-xs">
                      <span class="text-[var(--text-secondary)]">其他开销</span>
                      <span class="text-[var(--text-primary)] font-medium">{{ overheadTime.toFixed(2) }} ms</span>
                    </div>
                    <div class="text-[10px] text-[var(--text-muted)]">{{ overheadPercent.toFixed(1) }}%</div>
                  </div>
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
import { ref, computed, onMounted } from 'vue'
import type {
  GpuBackend,
  GpuDeviceInfo,
  GpuBenchmarkResult,
  GpuSolverConfig,
  GpuPerformanceProfile
} from '../api/phaseFieldGpu'

// ============ 响应式状态 ============

const sections = ref({
  devices: true,
  solver: true,
  benchmark: true
})

const detecting = ref(false)
const benchmarking = ref(false)
const configuring = ref(false)
const selectedDeviceIdx = ref<number | null>(null)

const devices = ref<GpuDeviceInfo[]>([])
const benchmarkResults = ref<GpuBenchmarkResult[]>([])
const solverConfig = ref<GpuSolverConfig>({
  backend: 'cuda',
  block_size: { x: 16, y: 16 },
  use_shared_memory: true,
  use_texture_memory: false,
  precision: 'float32',
  async_compute: true,
  device_id: 0
})

const benchmarkGridSize = ref<{ nx: number; ny: number }>({ nx: 256, ny: 256 })
const benchmarkSteps = ref(1000)

const gridSizes = [
  { label: '64²', value: { nx: 64, ny: 64 } },
  { label: '128²', value: { nx: 128, ny: 128 } },
  { label: '256²', value: { nx: 256, ny: 256 } },
  { label: '512²', value: { nx: 512, ny: 512 } }
]

const performanceProfile = ref<GpuPerformanceProfile>({
  kernel_time_ms: 2.35,
  memory_transfer_ms: 0.87,
  total_time_ms: 3.52,
  occupancy: 78.5,
  cache_hit_rate: 92.3,
  memory_bandwidth_gb_s: 612.4
})

const realtimeMetrics = ref({
  stepsPerSecond: 12450,
  memoryUsedMb: 1024,
  speedup: 28.5,
  temperature: 68
})

// ============ 计算属性 ============

const kernelPercent = computed(() => {
  const total = performanceProfile.value.total_time_ms
  if (total <= 0) return 0
  return (performanceProfile.value.kernel_time_ms / total) * 100
})

const transferPercent = computed(() => {
  const total = performanceProfile.value.total_time_ms
  if (total <= 0) return 0
  return (performanceProfile.value.memory_transfer_ms / total) * 100
})

const overheadTime = computed(() => {
  return Math.max(0, performanceProfile.value.total_time_ms
    - performanceProfile.value.kernel_time_ms
    - performanceProfile.value.memory_transfer_ms)
})

const overheadPercent = computed(() => {
  const total = performanceProfile.value.total_time_ms
  if (total <= 0) return 0
  return (overheadTime.value / total) * 100
})

const circumference = 2 * Math.PI * 60

const kernelArcLength = computed(() => {
  return (kernelPercent.value / 100) * circumference
})

const kernelArc = computed(() => {
  return `${kernelArcLength.value} ${circumference}`
})

const transferArc = computed(() => {
  const len = (transferPercent.value / 100) * circumference
  return `${len} ${circumference}`
})

// ============ 方法 ============

function toggleSection(key: keyof typeof sections.value) {
  sections.value[key] = !sections.value[key]
}

function backendLabel(backend: GpuBackend): string {
  const map: Record<GpuBackend, string> = {
    cuda: 'CUDA',
    opencl: 'OpenCL',
    webgpu: 'WebGPU',
    wasm_simd: 'WASM SIMD'
  }
  return map[backend] || backend
}

function cpuBarWidth(result: GpuBenchmarkResult): number {
  const cpuSteps = result.steps_per_second / result.speedup_vs_cpu
  const maxSteps = Math.max(...benchmarkResults.value.map(r => r.steps_per_second), 1)
  return Math.max(2, (cpuSteps / maxSteps) * 100)
}

function gpuBarWidth(result: GpuBenchmarkResult): number {
  const maxSteps = Math.max(...benchmarkResults.value.map(r => r.steps_per_second), 1)
  return Math.max(2, (result.steps_per_second / maxSteps) * 100)
}

function selectDevice(idx: number) {
  selectedDeviceIdx.value = idx
  solverConfig.value.device_id = idx
}

function generateMockDevices(): GpuDeviceInfo[] {
  return [
    {
      name: 'NVIDIA GeForce RTX 4090',
      vendor: 'NVIDIA Corporation',
      compute_capability: '8.9',
      total_memory_mb: 24576,
      available_memory_mb: 20480,
      max_threads_per_block: 1024,
      max_grid_size: { x: 2147483647, y: 65535 },
      clock_speed_mhz: 2520,
      driver_version: '545.29.06'
    },
    {
      name: 'NVIDIA GeForce RTX 3080 Ti',
      vendor: 'NVIDIA Corporation',
      compute_capability: '8.6',
      total_memory_mb: 12288,
      available_memory_mb: 10240,
      max_threads_per_block: 1024,
      max_grid_size: { x: 2147483647, y: 65535 },
      clock_speed_mhz: 1860,
      driver_version: '545.29.06'
    },
    {
      name: 'Apple M2 Ultra GPU',
      vendor: 'Apple Inc.',
      compute_capability: 'Metal 3.1',
      total_memory_mb: 65536,
      available_memory_mb: 52428,
      max_threads_per_block: 1024,
      max_grid_size: { x: 65535, y: 65535 },
      clock_speed_mhz: 1398,
      driver_version: 'macOS 15.3'
    }
  ]
}

function generateMockBenchmarkResults(gridSize: { nx: number; ny: number }, steps: number): GpuBenchmarkResult[] {
  const baseStepsPerSecond = (64 / gridSize.nx) * (64 / gridSize.ny) * 15000
  return [
    {
      backend: 'cuda',
      grid_size: gridSize,
      steps_per_second: baseStepsPerSecond * 1.0,
      memory_used_mb: (gridSize.nx * gridSize.ny * 8) / (1024 * 1024) * 2.5,
      speedup_vs_cpu: 28.5,
      energy_consumption_mj: steps * 0.012,
      temperature_C: 68
    },
    {
      backend: 'opencl',
      grid_size: gridSize,
      steps_per_second: baseStepsPerSecond * 0.82,
      memory_used_mb: (gridSize.nx * gridSize.ny * 8) / (1024 * 1024) * 2.8,
      speedup_vs_cpu: 23.4,
      energy_consumption_mj: steps * 0.015,
      temperature_C: 72
    },
    {
      backend: 'webgpu',
      grid_size: gridSize,
      steps_per_second: baseStepsPerSecond * 0.65,
      memory_used_mb: (gridSize.nx * gridSize.ny * 8) / (1024 * 1024) * 3.0,
      speedup_vs_cpu: 18.5,
      energy_consumption_mj: steps * 0.018,
      temperature_C: 65
    },
    {
      backend: 'wasm_simd',
      grid_size: gridSize,
      steps_per_second: baseStepsPerSecond * 0.35,
      memory_used_mb: (gridSize.nx * gridSize.ny * 8) / (1024 * 1024) * 3.5,
      speedup_vs_cpu: 9.8,
      energy_consumption_mj: steps * 0.025,
      temperature_C: 58
    }
  ]
}

function generateMockPerformanceProfile(): GpuPerformanceProfile {
  return {
    kernel_time_ms: 1.8 + Math.random() * 1.2,
    memory_transfer_ms: 0.5 + Math.random() * 0.8,
    total_time_ms: 0,
    occupancy: 65 + Math.random() * 25,
    cache_hit_rate: 85 + Math.random() * 12,
    memory_bandwidth_gb_s: 400 + Math.random() * 400
  }
}

async function handleDetectDevices() {
  detecting.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 800))
    devices.value = generateMockDevices()
    if (devices.value.length > 0 && selectedDeviceIdx.value === null) {
      selectedDeviceIdx.value = 0
      solverConfig.value.device_id = 0
    }
  } finally {
    detecting.value = false
  }
}

async function handleRunBenchmark() {
  benchmarking.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 1500))
    benchmarkResults.value = generateMockBenchmarkResults(benchmarkGridSize.value, benchmarkSteps.value)

    const profile = generateMockPerformanceProfile()
    profile.total_time_ms = profile.kernel_time_ms + profile.memory_transfer_ms + 0.3
    performanceProfile.value = profile

    if (benchmarkResults.value.length > 0) {
      const bestResult = benchmarkResults.value[0]
      realtimeMetrics.value = {
        stepsPerSecond: bestResult.steps_per_second,
        memoryUsedMb: bestResult.memory_used_mb,
        speedup: bestResult.speedup_vs_cpu,
        temperature: bestResult.temperature_C
      }
    }
  } finally {
    benchmarking.value = false
  }
}

async function handleConfigureSolver() {
  configuring.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 600))
    const warnings: string[] = []
    if (solverConfig.value.precision === 'float64' && solverConfig.value.backend === 'webgpu') {
      warnings.push('WebGPU 后端对 float64 支持有限，建议使用 float32')
    }
    if (solverConfig.value.use_shared_memory && solverConfig.value.block_size.x > 32) {
      warnings.push('块大小较大时共享内存可能溢出，建议减小块大小')
    }
    if (solverConfig.value.use_texture_memory && solverConfig.value.precision === 'float64') {
      warnings.push('纹理内存不支持 float64 精度，已自动禁用')
    }
    if (warnings.length === 0) {
      warnings.push('配置已成功应用')
    }
    alert(warnings.join('\n'))
  } finally {
    configuring.value = false
  }
}

onMounted(() => {
  devices.value = generateMockDevices()
  selectedDeviceIdx.value = 0
  benchmarkResults.value = generateMockBenchmarkResults(benchmarkGridSize.value, benchmarkSteps.value)
  const profile = generateMockPerformanceProfile()
  profile.total_time_ms = profile.kernel_time_ms + profile.memory_transfer_ms + 0.3
  performanceProfile.value = profile
  if (benchmarkResults.value.length > 0) {
    const bestResult = benchmarkResults.value[0]
    realtimeMetrics.value = {
      stepsPerSecond: bestResult.steps_per_second,
      memoryUsedMb: bestResult.memory_used_mb,
      speedup: bestResult.speedup_vs_cpu,
      temperature: bestResult.temperature_C
    }
  }
})
</script>
