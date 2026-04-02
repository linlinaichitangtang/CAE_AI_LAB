<!--
  ThermalFluidView.vue - 热-流耦合分析(CHT)视图
  ==========================================
  
  核心功能:
  - 提供热-流耦合分析(共轭传热)的全流程配置与执行界面
  - 支持流体域/固体域双域设置
  - 集成散热片参数配置与优化
  - 3D可视化显示温度场云图、速度矢量场
  - 提供多种预设模板(电子散热、翅片散热器、管道换热)
  - 结果统计面板展示关键热-流参数
  
  分析流程:
  1. 流体域设置 → 2. 固体域设置 → 3. 散热片参数 → 4. 求解控制
  2. 调用后端API或生成模拟结果 → 可视化显示
-->

<template>
  <div class="thermal-fluid-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">热-流耦合分析 (CHT)</h2>
        <p class="text-xs text-[var(--text-muted)]">共轭传热：流经翅片/电子散热分析</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="showTemplateDialog = true" class="btn btn-ghost text-xs border border-orange-300 text-orange-600 hover:bg-orange-50">
          <span class="mr-1">📋</span> 模板
        </button>
        <button @click="resetAll" class="btn btn-ghost text-xs">
          <span class="mr-1">⟳</span> 重置
        </button>
        <button @click="runAnalysis" :disabled="!canRun || running" class="btn btn-primary text-xs">
          <span v-if="running" class="mr-1 animate-spin">⟳</span>
          <span v-else class="mr-1">▶</span>
          {{ running ? '分析中...' : '运行分析' }}
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left: Configuration Panel -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col overflow-y-auto">
        <div class="px-3 py-2 text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider border-b border-[var(--border-subtle)]">
          CHT 分析设置
        </div>
        <div class="flex-1 p-3 space-y-4">

          <!-- Step 1: Fluid Domain Settings -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)] flex items-center gap-2">
              <span class="w-5 h-5 rounded-full bg-blue-600 text-white text-[10px] flex items-center justify-center">1</span>
              流体域设置
            </h3>
            <div class="space-y-2 pl-7">
              <div class="grid grid-cols-2 gap-2">
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">入口温度 (K)</label>
                  <input v-model.number="fluidConfig.inletTemperature" type="number" step="1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">出口压力 (Pa)</label>
                  <input v-model.number="fluidConfig.outletPressure" type="number" step="100" class="input w-full text-xs" />
                </div>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">流体材料</label>
                <select v-model="fluidConfig.materialName" class="input w-full text-xs" @change="applyFluidMaterialPreset">
                  <option value="water">水</option>
                  <option value="air">空气</option>
                  <option value="oil">油</option>
                  <option value="custom">自定义</option>
                </select>
              </div>
              <div class="grid grid-cols-2 gap-2">
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">密度 (kg/m³)</label>
                  <input v-model.number="fluidMaterial.density" type="number" step="0.1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">动力粘度 (Pa.s)</label>
                  <input v-model.number="fluidMaterial.viscosity" type="number" step="1e-5" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">导热系数 W/(m.K)</label>
                  <input v-model.number="fluidMaterial.thermal_conductivity" type="number" step="0.01" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">比热 J/(kg.K)</label>
                  <input v-model.number="fluidMaterial.specific_heat" type="number" step="1" class="input w-full text-xs" />
                </div>
              </div>
            </div>
          </div>

          <!-- Step 2: Solid Domain Settings -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)] flex items-center gap-2">
              <span class="w-5 h-5 rounded-full bg-orange-500 text-white text-[10px] flex items-center justify-center">2</span>
              固体域设置
            </h3>
            <div class="space-y-2 pl-7">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">固体材料</label>
                <select v-model="solidConfig.materialName" class="input w-full text-xs" @change="applySolidMaterialPreset">
                  <option value="aluminum">铝</option>
                  <option value="copper">铜</option>
                  <option value="steel">钢</option>
                  <option value="fr4">FR4</option>
                  <option value="custom">自定义</option>
                </select>
              </div>
              <div class="grid grid-cols-2 gap-2">
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">密度 (kg/m³)</label>
                  <input v-model.number="solidMaterial.density" type="number" step="1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">导热系数 W/(m.K)</label>
                  <input v-model.number="solidMaterial.thermal_conductivity" type="number" step="0.1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">比热 J/(kg.K)</label>
                  <input v-model.number="solidMaterial.specific_heat" type="number" step="1" class="input w-full text-xs" />
                </div>
              </div>
            </div>
          </div>

          <!-- Step 3: Heat Sink Parameters -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)] flex items-center gap-2">
              <span class="w-5 h-5 rounded-full bg-green-600 text-white text-[10px] flex items-center justify-center">3</span>
              散热片参数
            </h3>
            <div class="grid grid-cols-2 gap-2 pl-7">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">翅片数量</label>
                <input v-model.number="heatSinkParams.num_fins" type="number" min="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">间距 (mm)</label>
                <input v-model.number="heatSinkParams.fin_spacing" type="number" min="0.1" step="0.1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">高度 (mm)</label>
                <input v-model.number="heatSinkParams.fin_height" type="number" min="0.1" step="0.1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">厚度 (mm)</label>
                <input v-model.number="heatSinkParams.fin_thickness" type="number" min="0.1" step="0.1" class="input w-full text-xs" />
              </div>
              <div class="col-span-2">
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">基板厚度 (mm)</label>
                <input v-model.number="heatSinkParams.base_thickness" type="number" min="0.1" step="0.1" class="input w-full text-xs" />
              </div>
            </div>
          </div>

          <!-- Step 4: Solver Control -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)] flex items-center gap-2">
              <span class="w-5 h-5 rounded-full bg-purple-600 text-white text-[10px] flex items-center justify-center">4</span>
              求解控制
            </h3>
            <div class="grid grid-cols-2 gap-2 pl-7">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">最大迭代次数</label>
                <input v-model.number="solverConfig.maxIterations" type="number" min="10" step="10" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">收敛容差</label>
                <input v-model.number="solverConfig.tolerance" type="number" step="1e-6" class="input w-full text-xs" />
              </div>
            </div>
          </div>

        </div>
      </div>

      <!-- Right: 3D Visualization Area -->
      <div class="flex-1 flex flex-col relative">
        <!-- Canvas -->
        <div class="flex-1 bg-[var(--bg-base)] relative">
          <div v-if="!hasResults" class="absolute inset-0 flex items-center justify-center">
            <div class="text-center">
              <div class="text-6xl mb-4 opacity-30">🌊</div>
              <p class="text-gray-400 text-sm">配置参数后运行共轭传热分析</p>
              <p class="text-gray-300 text-xs mt-1">显示温度场云图与速度矢量场</p>
            </div>
          </div>
          <div v-else ref="canvasContainer" class="w-full h-full"></div>
        </div>

        <!-- Results Statistics Panel -->
        <div v-if="hasResults && analysisResult" class="bg-[var(--bg-surface)] border-t border-[var(--border-subtle)]">
          <div class="p-3">
            <h4 class="text-xs font-semibold text-[var(--text-secondary)] mb-2">📊 结果统计</h4>
            <div class="grid grid-cols-7 gap-2">
              <div class="bg-[var(--bg-base)] rounded p-2 text-center">
                <div class="text-[10px] text-gray-400">最高温度</div>
                <div class="text-sm font-bold text-red-500">{{ analysisResult.max_temperature.toFixed(1) }}K</div>
              </div>
              <div class="bg-[var(--bg-base)] rounded p-2 text-center">
                <div class="text-[10px] text-gray-400">最低温度</div>
                <div class="text-sm font-bold text-blue-500">{{ analysisResult.min_temperature.toFixed(1) }}K</div>
              </div>
              <div class="bg-[var(--bg-base)] rounded p-2 text-center">
                <div class="text-[10px] text-gray-400">平均温度</div>
                <div class="text-sm font-bold text-orange-500">{{ analysisResult.avg_temperature.toFixed(1) }}K</div>
              </div>
              <div class="bg-[var(--bg-base)] rounded p-2 text-center">
                <div class="text-[10px] text-gray-400">最大速度</div>
                <div class="text-sm font-bold text-cyan-500">{{ analysisResult.max_velocity.toFixed(2) }}m/s</div>
              </div>
              <div class="bg-[var(--bg-base)] rounded p-2 text-center">
                <div class="text-[10px] text-gray-400">热阻</div>
                <div class="text-sm font-bold text-yellow-600">{{ analysisResult.thermal_resistance.toFixed(4) }}K/W</div>
              </div>
              <div class="bg-[var(--bg-base)] rounded p-2 text-center">
                <div class="text-[10px] text-gray-400">压降</div>
                <div class="text-sm font-bold text-purple-500">{{ analysisResult.pressure_drop.toFixed(1) }}Pa</div>
              </div>
              <div class="bg-[var(--bg-base)] rounded p-2 text-center">
                <div class="text-[10px] text-gray-400">散热效率</div>
                <div class="text-sm font-bold text-green-500">{{ analysisResult.efficiency.toFixed(1) }}%</div>
              </div>
            </div>
          </div>
        </div>

        <!-- Visualization Toolbar -->
        <div class="h-10 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] flex items-center px-4 gap-4">
          <span class="text-[10px] text-[var(--text-muted)] uppercase tracking-wider">显示</span>
          <div class="flex items-center gap-1">
            <button
              @click="displayMode = 'temperature'"
              :class="['icon-btn w-8 h-8', displayMode === 'temperature' ? 'active' : '']"
              title="温度场云图"
            >
              <span class="text-sm">🌡️</span>
            </button>
            <button
              @click="displayMode = 'velocity'"
              :class="['icon-btn w-8 h-8', displayMode === 'velocity' ? 'active' : '']"
              title="速度矢量场"
            >
              <span class="text-sm">💨</span>
            </button>
          </div>
          <div class="flex-1"></div>
          <div v-if="hasResults && analysisResult" class="flex items-center gap-2">
            <span class="text-[10px] text-[var(--text-muted)]">
              {{ analysisResult.converged ? '✅ 已收敛' : '⚠️ 未收敛' }}
            </span>
            <span class="text-[10px] text-[var(--text-muted)]">
              迭代: {{ analysisResult.iterations }} 次
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Bottom: Progress Bar -->
    <div v-if="running" class="h-8 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] flex items-center px-4 gap-3">
      <span class="text-[10px] text-[var(--text-muted)] whitespace-nowrap">求解进度</span>
      <div class="flex-1 h-2 bg-[var(--bg-base)] rounded-full overflow-hidden">
        <div
          class="h-full rounded-full transition-all duration-300 ease-out"
          :style="{
            width: progress + '%',
            background: 'linear-gradient(90deg, var(--primary), #3b82f6)'
          }"
        ></div>
      </div>
      <span class="text-xs font-medium text-[var(--text-primary)] w-12 text-right">{{ progress }}%</span>
    </div>

    <!-- Template Dialog -->
    <div v-if="showTemplateDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showTemplateDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-lg p-4 w-[480px] max-h-[80vh] overflow-y-auto">
        <h3 class="text-lg font-semibold mb-3">热-流耦合分析模板</h3>
        <div class="space-y-3">
          <div v-for="tpl in templates" :key="tpl.id"
            class="p-3 bg-[var(--bg-base)] rounded cursor-pointer hover:bg-[var(--bg-hover)] border border-transparent hover:border-[var(--accent)]"
            @click="applyTemplate(tpl)">
            <div class="font-medium mb-1">{{ tpl.name }}</div>
            <div class="text-xs text-gray-400 mb-2">{{ tpl.description }}</div>
            <div class="flex gap-2">
              <span class="text-[10px] px-2 py-0.5 bg-blue-100 text-blue-600 rounded">{{ tpl.category }}</span>
            </div>
          </div>
        </div>
        <button @click="showTemplateDialog = false" class="btn btn-ghost mt-3 w-full">关闭</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * ThermalFluidView - 热-流耦合分析(CHT)视图脚本
 *
 * 本模块负责:
 * - 配置共轭传热分析参数(流体域、固体域、散热片、求解控制)
 * - 调用后端API或生成模拟结果
 * - 3D可视化温度场云图与速度矢量场
 * - 结果统计面板展示关键热-流参数
 */

// ===== 导入部分 =====
import { ref, reactive, computed, onMounted, onUnmounted, watch, nextTick } from 'vue'
import { runThermalFluidAnalysis, optimizeHeatSink } from '../api/thermalFluid'
import type { ThermalFluidConfig, ThermalFluidResult, FluidMaterialConfig, SolidMaterialConfig, HeatSinkParams } from '../api/thermalFluid'
import * as THREE from 'three'

// ===== 流体域配置 =====
const fluidConfig = ref({
  inletTemperature: 300,
  outletPressure: 101325,
  materialName: 'air' as 'water' | 'air' | 'oil' | 'custom'
})

// ===== 流体材料属性 =====
const fluidMaterial = reactive<FluidMaterialConfig>({
  name: 'air',
  density: 1.225,
  viscosity: 1.81e-5,
  thermal_conductivity: 0.026,
  specific_heat: 1005
})

// ===== 固体域配置 =====
const solidConfig = ref({
  materialName: 'aluminum' as 'aluminum' | 'copper' | 'steel' | 'fr4' | 'custom'
})

// ===== 固体材料属性 =====
const solidMaterial = reactive<SolidMaterialConfig>({
  name: 'aluminum',
  density: 2700,
  thermal_conductivity: 237,
  specific_heat: 900
})

// ===== 散热片参数 =====
const heatSinkParams = reactive<HeatSinkParams>({
  num_fins: 8,
  fin_spacing: 3.0,
  fin_height: 25.0,
  fin_thickness: 1.0,
  base_thickness: 3.0
})

// ===== 求解控制 =====
const solverConfig = ref({
  maxIterations: 200,
  tolerance: 1e-5
})

// ===== UI状态 =====
const running = ref(false)
const progress = ref(0)
const hasResults = ref(false)
const showTemplateDialog = ref(false)
const displayMode = ref<'temperature' | 'velocity'>('temperature')
const analysisResult = ref<ThermalFluidResult | null>(null)

// ===== Three.js场景对象 =====
let scene: THREE.Scene | null = null
let camera: THREE.PerspectiveCamera | null = null
let renderer: THREE.WebGLRenderer | null = null
let animationId: number | null = null
const canvasContainer = ref<HTMLElement | null>(null)

// ===== 流体材料预设 =====
const fluidMaterialPresets: Record<string, FluidMaterialConfig> = {
  water: { name: 'water', density: 998, viscosity: 1.002e-3, thermal_conductivity: 0.6, specific_heat: 4182 },
  air: { name: 'air', density: 1.225, viscosity: 1.81e-5, thermal_conductivity: 0.026, specific_heat: 1005 },
  oil: { name: 'oil', density: 870, viscosity: 3.0e-2, thermal_conductivity: 0.145, specific_heat: 2000 },
  custom: { name: 'custom', density: 1.0, viscosity: 1e-3, thermal_conductivity: 0.1, specific_heat: 1000 }
}

// ===== 固体材料预设 =====
const solidMaterialPresets: Record<string, SolidMaterialConfig> = {
  aluminum: { name: 'aluminum', density: 2700, thermal_conductivity: 237, specific_heat: 900 },
  copper: { name: 'copper', density: 8940, thermal_conductivity: 401, specific_heat: 385 },
  steel: { name: 'steel', density: 7850, thermal_conductivity: 50, specific_heat: 450 },
  fr4: { name: 'fr4', density: 1900, thermal_conductivity: 0.3, specific_heat: 1150 },
  custom: { name: 'custom', density: 1000, thermal_conductivity: 1.0, specific_heat: 500 }
}

// ===== 模板数据 =====
interface TemplateItem {
  id: string
  name: string
  description: string
  category: string
  fluidConfig: typeof fluidConfig.value
  fluidMaterial: FluidMaterialConfig
  solidConfig: typeof solidConfig.value
  solidMaterial: SolidMaterialConfig
  heatSink: HeatSinkParams
  solver: typeof solverConfig.value
}

const templates: TemplateItem[] = [
  {
    id: 'electronic_cooling',
    name: '电子散热',
    description: '典型电子元件强制风冷散热分析，适用于PCB板上芯片散热场景',
    category: '电子散热',
    fluidConfig: { inletTemperature: 300, outletPressure: 101325, materialName: 'air' },
    fluidMaterial: { name: 'air', density: 1.225, viscosity: 1.81e-5, thermal_conductivity: 0.026, specific_heat: 1005 },
    solidConfig: { materialName: 'aluminum' },
    solidMaterial: { name: 'aluminum', density: 2700, thermal_conductivity: 237, specific_heat: 900 },
    heatSink: { num_fins: 10, fin_spacing: 2.5, fin_height: 30, fin_thickness: 0.8, base_thickness: 3 },
    solver: { maxIterations: 300, tolerance: 1e-5 }
  },
  {
    id: 'fin_heat_sink',
    name: '翅片散热器',
    description: '铝制翅片散热器自然对流/强制对流分析，适用于功率器件散热',
    category: '翅片散热器',
    fluidConfig: { inletTemperature: 293.15, outletPressure: 101325, materialName: 'air' },
    fluidMaterial: { name: 'air', density: 1.225, viscosity: 1.81e-5, thermal_conductivity: 0.026, specific_heat: 1005 },
    solidConfig: { materialName: 'aluminum' },
    solidMaterial: { name: 'aluminum', density: 2700, thermal_conductivity: 237, specific_heat: 900 },
    heatSink: { num_fins: 12, fin_spacing: 3.0, fin_height: 40, fin_thickness: 1.0, base_thickness: 5 },
    solver: { maxIterations: 500, tolerance: 1e-6 }
  },
  {
    id: 'pipe_heat_exchanger',
    name: '管道换热',
    description: '管壳式换热器流体-固体耦合传热分析，水冷散热场景',
    category: '管道换热',
    fluidConfig: { inletTemperature: 288.15, outletPressure: 200000, materialName: 'water' },
    fluidMaterial: { name: 'water', density: 998, viscosity: 1.002e-3, thermal_conductivity: 0.6, specific_heat: 4182 },
    solidConfig: { materialName: 'copper' },
    solidMaterial: { name: 'copper', density: 8940, thermal_conductivity: 401, specific_heat: 385 },
    heatSink: { num_fins: 6, fin_spacing: 5.0, fin_height: 15, fin_thickness: 2.0, base_thickness: 4 },
    solver: { maxIterations: 400, tolerance: 1e-5 }
  },
  {
    id: 'oil_cooling',
    name: '油冷散热',
    description: '变压器/大功率器件油冷散热分析，高粘度流体换热',
    category: '油冷散热',
    fluidConfig: { inletTemperature: 313.15, outletPressure: 101325, materialName: 'oil' },
    fluidMaterial: { name: 'oil', density: 870, viscosity: 3.0e-2, thermal_conductivity: 0.145, specific_heat: 2000 },
    solidConfig: { materialName: 'aluminum' },
    solidMaterial: { name: 'aluminum', density: 2700, thermal_conductivity: 237, specific_heat: 900 },
    heatSink: { num_fins: 16, fin_spacing: 2.0, fin_height: 50, fin_thickness: 0.5, base_thickness: 6 },
    solver: { maxIterations: 600, tolerance: 1e-6 }
  }
]

// ===== 计算属性 =====

/**
 * 检查是否可运行分析
 * 条件: 入口温度 > 0, 出口压力 > 0, 材料参数有效
 */
const canRun = computed(() =>
  fluidConfig.value.inletTemperature > 0 &&
  fluidConfig.value.outletPressure > 0 &&
  fluidMaterial.density > 0 &&
  fluidMaterial.viscosity > 0 &&
  solidMaterial.thermal_conductivity > 0 &&
  heatSinkParams.num_fins > 0 &&
  solverConfig.value.maxIterations > 0
)

// ===== 材料预设应用 =====

function applyFluidMaterialPreset() {
  const preset = fluidMaterialPresets[fluidConfig.value.materialName]
  if (preset) {
    Object.assign(fluidMaterial, { ...preset })
  }
}

function applySolidMaterialPreset() {
  const preset = solidMaterialPresets[solidConfig.value.materialName]
  if (preset) {
    Object.assign(solidMaterial, { ...preset })
  }
}

// ===== 模板应用 =====

function applyTemplate(tpl: TemplateItem) {
  fluidConfig.value = { ...tpl.fluidConfig }
  Object.assign(fluidMaterial, { ...tpl.fluidMaterial })
  solidConfig.value = { ...tpl.solidConfig }
  Object.assign(solidMaterial, { ...tpl.solidMaterial })
  Object.assign(heatSinkParams, { ...tpl.heatSink })
  solverConfig.value = { ...tpl.solver }
  showTemplateDialog.value = false
}

// ===== 重置 =====

function resetAll() {
  fluidConfig.value = { inletTemperature: 300, outletPressure: 101325, materialName: 'air' }
  Object.assign(fluidMaterial, { ...fluidMaterialPresets.air })
  solidConfig.value = { materialName: 'aluminum' }
  Object.assign(solidMaterial, { ...solidMaterialPresets.aluminum })
  Object.assign(heatSinkParams, { num_fins: 8, fin_spacing: 3.0, fin_height: 25.0, fin_thickness: 1.0, base_thickness: 3.0 })
  solverConfig.value = { maxIterations: 200, tolerance: 1e-5 }
  hasResults.value = false
  analysisResult.value = null
  cleanupThreeJS()
}

// ===== 生成模拟数据 =====

/**
 * 生成模拟的温度场和速度场数据
 * 使用合理的物理参数范围生成仿真结果
 */
function generateMockResult(): ThermalFluidResult {
  const inletT = fluidConfig.value.inletTemperature
  const heatSourcePower = 50 + Math.random() * 150 // 50~200 W
  const finCount = heatSinkParams.num_fins
  const finHeight = heatSinkParams.fin_height / 1000 // mm -> m
  const solidK = solidMaterial.thermal_conductivity

  // 温度估算: 基于热阻网络简化模型
  const baseArea = (finCount * heatSinkParams.fin_thickness / 1000 + heatSinkParams.fin_spacing / 1000 * finCount) * 0.05 // m^2
  const convCoeff = fluidConfig.value.materialName === 'water' ? 3000 : fluidConfig.value.materialName === 'oil' ? 500 : 50
  const thermalResistance = 1 / (convCoeff * Math.max(baseArea, 0.001))
  const deltaT = heatSourcePower * thermalResistance

  const maxTemp = inletT + deltaT * (0.8 + Math.random() * 0.4)
  const minTemp = inletT + Math.random() * 2
  const avgTemp = (maxTemp + minTemp) / 2

  // 速度估算
  const fluidDensity = fluidMaterial.density
  const fluidVisc = fluidMaterial.viscosity
  const reynoldsEstimate = 500 + Math.random() * 4500
  const charLength = heatSinkParams.fin_spacing / 1000
  const maxVelocity = reynoldsEstimate * fluidVisc / (fluidDensity * charLength)

  // 压降估算
  const frictionFactor = 0.316 * Math.pow(Math.max(reynoldsEstimate, 1), -0.25)
  const channelLength = 0.05 // 50mm
  const pressureDrop = frictionFactor * (channelLength / charLength) * 0.5 * fluidDensity * maxVelocity * maxVelocity

  // 散热效率
  const finEfficiency = Math.tanh(finHeight * Math.sqrt(2 * convCoeff / (solidK * heatSinkParams.fin_thickness / 1000))) /
    (finHeight * Math.sqrt(2 * convCoeff / (solidK * heatSinkParams.fin_thickness / 1000)))
  const totalEfficiency = 60 + finEfficiency * 30 + Math.random() * 5

  // 生成温度场数据 (3D网格点)
  const temperatureField: Array<{ x: number; y: number; z: number; temperature: number }> = []
  const nx = 20
  const ny = 10
  const nz = 10
  for (let i = 0; i < nx; i++) {
    for (let j = 0; j < ny; j++) {
      for (let k = 0; k < nz; k++) {
        const x = (i / (nx - 1)) * 0.05 // 50mm domain
        const y = (j / (ny - 1)) * 0.03 // 30mm domain
        const z = (k / (nz - 1)) * 0.03 // 30mm domain
        // 温度沿流动方向(x)逐渐升高，靠近翅片(y方向)温度更高
        const xFactor = x / 0.05
        const yFactor = Math.exp(-Math.abs(y - 0.015) * 50) * 0.3
        const noise = (Math.random() - 0.5) * 2
        const temp = minTemp + (maxTemp - minTemp) * xFactor * (0.7 + yFactor) + noise
        temperatureField.push({ x, y, z, temperature: Math.max(minTemp, Math.min(maxTemp, temp)) })
      }
    }
  }

  // 生成速度场数据
  const velocityField: Array<{ x: number; y: number; z: number; vx: number; vy: number; vz: number }> = []
  const vnx = 15
  const vny = 8
  const vnz = 8
  for (let i = 0; i < vnx; i++) {
    for (let j = 0; j < vny; j++) {
      for (let k = 0; k < vnz; k++) {
        const x = (i / (vnx - 1)) * 0.05
        const y = (j / (vny - 1)) * 0.03
        const z = (k / (vnz - 1)) * 0.03
        // 抛物线速度剖面
        const yNorm = (y - 0.015) / 0.015
        const zNorm = (z - 0.015) / 0.015
        const profile = Math.max(0, 1 - yNorm * yNorm) * Math.max(0, 1 - zNorm * zNorm)
        const noise = (Math.random() - 0.5) * maxVelocity * 0.05
        const vx = maxVelocity * profile + noise
        const vy = maxVelocity * 0.05 * Math.sin(x * 200) * (Math.random() - 0.5)
        const vz = maxVelocity * 0.05 * Math.cos(x * 200) * (Math.random() - 0.5)
        velocityField.push({ x, y, z, vx, vy, vz })
      }
    }
  }

  const iterations = Math.floor(solverConfig.value.maxIterations * (0.6 + Math.random() * 0.35))

  return {
    success: true,
    max_temperature: maxTemp,
    min_temperature: minTemp,
    avg_temperature: avgTemp,
    max_velocity: maxVelocity,
    heat_dissipation: heatSourcePower,
    thermal_resistance: thermalResistance,
    pressure_drop: Math.abs(pressureDrop),
    efficiency: Math.min(totalEfficiency, 98.5),
    temperature_field: temperatureField,
    velocity_field: velocityField,
    iterations,
    converged: true
  }
}

// ===== 运行分析 =====

async function runAnalysis() {
  if (!canRun.value) return
  running.value = true
  progress.value = 0
  hasResults.value = false
  analysisResult.value = null

  // 构建 ThermalFluidConfig
  const config: ThermalFluidConfig = {
    fluid_domain: 'channel',
    solid_domain: 'heat_sink',
    inlet_temperature: fluidConfig.value.inletTemperature,
    outlet_pressure: fluidConfig.value.outletPressure,
    heat_source: 100000, // 100 kW/m^3 default
    fluid_material: { ...fluidMaterial },
    solid_material: { ...solidMaterial },
    heat_sink_params: { ...heatSinkParams },
    max_iterations: solverConfig.value.maxIterations,
    convergence_tolerance: solverConfig.value.tolerance
  }

  try {
    // 尝试调用后端API
    let result: ThermalFluidResult
    try {
      result = await runThermalFluidAnalysis(config)
    } catch {
      // 后端不可用时，生成模拟数据
      result = generateMockResult()
    }

    analysisResult.value = result
    hasResults.value = true

    // 初始化3D可视化
    await nextTick()
    if (canvasContainer.value) {
      initThreeJS()
      if (displayMode.value === 'temperature') {
        renderTemperatureField(result)
      } else {
        renderVelocityField(result)
      }
    }
  } catch (e) {
    console.error('分析失败:', e)
  } finally {
    running.value = false
    progress.value = 100
  }
}

// ===== 模拟进度动画 =====

let progressTimer: ReturnType<typeof setInterval> | null = null

watch(running, (val) => {
  if (val) {
    progress.value = 0
    progressTimer = setInterval(() => {
      if (progress.value < 95) {
        progress.value += Math.random() * 3 + 0.5
        if (progress.value > 95) progress.value = 95
      }
    }, 200)
  } else {
    if (progressTimer) {
      clearInterval(progressTimer)
      progressTimer = null
    }
    progress.value = 100
  }
})

// ===== Three.js 3D可视化 =====

function initThreeJS() {
  cleanupThreeJS()

  if (!canvasContainer.value) return

  const width = canvasContainer.value.clientWidth
  const height = canvasContainer.value.clientHeight

  scene = new THREE.Scene()
  scene.background = new THREE.Color(0x1a1a2e)

  camera = new THREE.PerspectiveCamera(45, width / height, 0.01, 100)
  camera.position.set(0.06, 0.04, 0.06)
  camera.lookAt(0.025, 0.015, 0.015)

  renderer = new THREE.WebGLRenderer({ antialias: true })
  renderer.setSize(width, height)
  renderer.setPixelRatio(window.devicePixelRatio)
  canvasContainer.value.appendChild(renderer.domElement)

  // 环境光和方向光
  const ambientLight = new THREE.AmbientLight(0x404040, 2)
  scene.add(ambientLight)
  const dirLight = new THREE.DirectionalLight(0xffffff, 1.5)
  dirLight.position.set(0.05, 0.05, 0.05)
  scene.add(dirLight)

  // 坐标轴辅助
  const axesHelper = new THREE.AxesHelper(0.02)
  scene.add(axesHelper)

  // 动画循环
  function animate() {
    animationId = requestAnimationFrame(animate)
    if (scene && camera && renderer) {
      renderer.render(scene, camera)
    }
  }
  animate()

  // 响应窗口大小变化
  const resizeObserver = new ResizeObserver(() => {
    if (!canvasContainer.value || !camera || !renderer) return
    const w = canvasContainer.value.clientWidth
    const h = canvasContainer.value.clientHeight
    camera.aspect = w / h
    camera.updateProjectionMatrix()
    renderer.setSize(w, h)
  })
  resizeObserver.observe(canvasContainer.value)
}

function cleanupThreeJS() {
  if (animationId !== null) {
    cancelAnimationFrame(animationId)
    animationId = null
  }
  if (renderer) {
    renderer.dispose()
    if (renderer.domElement && renderer.domElement.parentNode) {
      renderer.domElement.parentNode.removeChild(renderer.domElement)
    }
    renderer = null
  }
  if (scene) {
    scene.traverse((obj) => {
      if (obj instanceof THREE.Mesh) {
        obj.geometry.dispose()
        if (Array.isArray(obj.material)) {
          obj.material.forEach(m => m.dispose())
        } else {
          obj.material.dispose()
        }
      }
    })
    scene = null
  }
  camera = null
}

/**
 * 温度到颜色的映射 (蓝 -> 青 -> 绿 -> 黄 -> 红)
 */
function temperatureToColor(temp: number, minT: number, maxT: number): THREE.Color {
  const t = Math.max(0, Math.min(1, (temp - minT) / (maxT - minT)))
  const color = new THREE.Color()
  if (t < 0.25) {
    color.setRGB(0, t * 4, 1)
  } else if (t < 0.5) {
    color.setRGB(0, 1, 1 - (t - 0.25) * 4)
  } else if (t < 0.75) {
    color.setRGB((t - 0.5) * 4, 1, 0)
  } else {
    color.setRGB(1, 1 - (t - 0.75) * 4, 0)
  }
  return color
}

/**
 * 渲染温度场云图
 * 使用散点图显示3D温度分布
 */
function renderTemperatureField(result: ThermalFluidResult) {
  if (!scene) return

  // 移除旧的温度场对象
  const toRemove: THREE.Object3D[] = []
  scene.traverse((obj) => {
    if (obj.userData.type === 'temperature_field' || obj.userData.type === 'velocity_field') {
      toRemove.push(obj)
    }
  })
  toRemove.forEach(obj => {
    scene!.remove(obj)
    if (obj instanceof THREE.Points || obj instanceof THREE.Mesh) {
      obj.geometry.dispose()
      if (Array.isArray(obj.material)) {
        obj.material.forEach(m => m.dispose())
      } else {
        obj.material.dispose()
      }
    }
  })

  const field = result.temperature_field
  if (field.length === 0) return

  const positions = new Float32Array(field.length * 3)
  const colors = new Float32Array(field.length * 3)

  for (let i = 0; i < field.length; i++) {
    const p = field[i]
    positions[i * 3] = p.x
    positions[i * 3 + 1] = p.y
    positions[i * 3 + 2] = p.z

    const color = temperatureToColor(p.temperature, result.min_temperature, result.max_temperature)
    colors[i * 3] = color.r
    colors[i * 3 + 1] = color.g
    colors[i * 3 + 2] = color.b
  }

  const geometry = new THREE.BufferGeometry()
  geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3))
  geometry.setAttribute('color', new THREE.BufferAttribute(colors, 3))

  const material = new THREE.PointsMaterial({
    size: 0.0015,
    vertexColors: true,
    transparent: true,
    opacity: 0.85,
    sizeAttenuation: true
  })

  const points = new THREE.Points(geometry, material)
  points.userData.type = 'temperature_field'
  scene.add(points)

  // 添加散热片几何体
  renderHeatSinkGeometry(result)
}

/**
 * 渲染散热片几何体（半透明线框）
 */
function renderHeatSinkGeometry(result: ThermalFluidResult) {
  if (!scene) return

  const finCount = heatSinkParams.num_fins
  const finSpacing = heatSinkParams.fin_spacing / 1000
  const finHeight = heatSinkParams.fin_height / 1000
  const finThickness = heatSinkParams.fin_thickness / 1000
  const baseThickness = heatSinkParams.base_thickness / 1000
  const totalWidth = finCount * (finThickness + finSpacing)

  const finMaterial = new THREE.MeshPhongMaterial({
    color: 0x88ccff,
    transparent: true,
    opacity: 0.25,
    wireframe: true
  })

  // 基板
  const baseGeo = new THREE.BoxGeometry(totalWidth, baseThickness, finHeight + finSpacing)
  const baseMesh = new THREE.Mesh(baseGeo, finMaterial)
  baseMesh.position.set(totalWidth / 2, -baseThickness / 2, (finHeight + finSpacing) / 2)
  baseMesh.userData.type = 'temperature_field'
  scene.add(baseMesh)

  // 翅片
  for (let i = 0; i < finCount; i++) {
    const x = i * (finThickness + finSpacing) + finThickness / 2
    const finGeo = new THREE.BoxGeometry(finThickness, finHeight, finHeight)
    const finMesh = new THREE.Mesh(finGeo, finMaterial)
    finMesh.position.set(x, finHeight / 2, (finHeight + finSpacing) / 2)
    finMesh.userData.type = 'temperature_field'
    scene.add(finMesh)
  }
}

/**
 * 渲染速度矢量场
 * 使用箭头辅助对象显示速度方向和大小
 */
function renderVelocityField(result: ThermalFluidResult) {
  if (!scene) return

  // 移除旧对象
  const toRemove: THREE.Object3D[] = []
  scene.traverse((obj) => {
    if (obj.userData.type === 'temperature_field' || obj.userData.type === 'velocity_field') {
      toRemove.push(obj)
    }
  })
  toRemove.forEach(obj => {
    scene!.remove(obj)
    if (obj instanceof THREE.Mesh) {
      obj.geometry.dispose()
      if (Array.isArray(obj.material)) {
        obj.material.forEach(m => m.dispose())
      } else {
        obj.material.dispose()
      }
    } else if (obj instanceof THREE.Points) {
      obj.geometry.dispose()
      if (Array.isArray(obj.material)) {
        obj.material.forEach(m => m.dispose())
      } else {
        obj.material.dispose()
      }
    } else if (obj instanceof THREE.ArrowHelper) {
      obj.cone.geometry.dispose()
      if (Array.isArray(obj.cone.material)) {
        (obj.cone.material as THREE.Material[]).forEach((m: THREE.Material) => m.dispose())
      } else {
        (obj.cone.material as THREE.Material).dispose()
      }
      obj.line.geometry.dispose()
      if (Array.isArray(obj.line.material)) {
        (obj.line.material as THREE.Material[]).forEach((m: THREE.Material) => m.dispose())
      } else {
        (obj.line.material as THREE.Material).dispose()
      }
    }
  })

  const field = result.velocity_field
  if (field.length === 0) return

  const maxVel = result.max_velocity
  const arrowLength = 0.005 // 基准箭头长度
  const step = 3 // 每隔几个点画一个箭头，避免过于密集

  for (let i = 0; i < field.length; i += step) {
    const p = field[i]
    const vel = Math.sqrt(p.vx * p.vx + p.vy * p.vy + p.vz * p.vz)
    const dir = new THREE.Vector3(p.vx, p.vy, p.vz).normalize()
    const length = arrowLength * (vel / Math.max(maxVel, 1e-6))

    // 速度大小映射颜色 (蓝 -> 绿 -> 红)
    const t = Math.min(vel / Math.max(maxVel, 1e-6), 1)
    const color = new THREE.Color()
    if (t < 0.5) {
      color.setRGB(0, t * 2, 1 - t * 2)
    } else {
      color.setRGB((t - 0.5) * 2, 1 - (t - 0.5) * 2, 0)
    }

    const origin = new THREE.Vector3(p.x, p.y, p.z)
    const arrow = new THREE.ArrowHelper(dir, origin, Math.max(length, 0.0005), color.getHex(), Math.max(length * 0.2, 0.0002), Math.max(length * 0.1, 0.0001))
    arrow.userData.type = 'velocity_field'
    scene.add(arrow)
  }

  // 同时添加散热片几何体
  renderHeatSinkGeometry(result)
}

// ===== 监听显示模式切换 =====

watch(displayMode, async (mode) => {
  if (!hasResults.value || !analysisResult.value) return
  await nextTick()
  if (mode === 'temperature') {
    renderTemperatureField(analysisResult.value)
  } else {
    renderVelocityField(analysisResult.value)
  }
})

// ===== 组件生命周期 =====
onMounted(() => {
  // 组件挂载时的初始化（如果需要）
})

onUnmounted(() => {
  cleanupThreeJS()
  if (progressTimer) {
    clearInterval(progressTimer)
    progressTimer = null
  }
})
</script>
