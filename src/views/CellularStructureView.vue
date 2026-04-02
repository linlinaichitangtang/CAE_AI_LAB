<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">蜂窝/点阵结构生成器</h2>
        <p class="text-sm" style="color: var(--text-muted)">正六边形/三角形蜂窝，参数化胞元，等效刚度计算</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- Template Buttons -->
        <button
          v-for="tpl in quickTemplates"
          :key="tpl.id"
          @click="applyTemplate(tpl)"
          class="px-3 py-1.5 text-xs border rounded transition"
          style="border-color: var(--border-default); background: var(--bg-elevated); color: var(--text-secondary)"
        >
          {{ tpl.name }}
        </button>
        <button @click="resetAll" class="px-3 py-1.5 text-xs border rounded transition" style="border-color: var(--border-default); color: var(--text-secondary)">
          重置
        </button>
        <button v-if="results" @click="exportMesh" class="px-3 py-1.5 text-xs border rounded transition" style="border-color: var(--accent-green); color: var(--accent-green)">
          导出网格
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration (w-80) -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Cell Type -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            胞元类型
          </h4>
          <div class="grid grid-cols-2 gap-1">
            <button
              v-for="ct in cellTypes"
              :key="ct.value"
              @click="config.cell_type = ct.value"
              :class="['px-2 py-2 rounded text-xs text-left transition border', config.cell_type === ct.value ? 'text-white' : '']"
              :style="config.cell_type === ct.value
                ? 'background: var(--primary); border-color: var(--primary)'
                : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
            >
              <div class="font-medium">{{ ct.label }}</div>
              <div class="text-[10px] mt-0.5 opacity-80">{{ ct.desc }}</div>
            </button>
          </div>
        </div>

        <!-- Step 2: Geometry Parameters -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            几何参数
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">胞元尺寸 (mm)</label>
              <input v-model.number="config.cell_size" type="number" step="0.5" min="1" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">壁厚 (mm)</label>
              <input v-model.number="config.wall_thickness" type="number" step="0.01" min="0.01" class="input w-full text-xs" />
            </div>
            <div class="grid grid-cols-3 gap-2">
              <div>
                <label class="label">X 胞元数</label>
                <input v-model.number="config.num_cells_x" type="number" step="1" min="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">Y 胞元数</label>
                <input v-model.number="config.num_cells_y" type="number" step="1" min="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">Z 胞元数</label>
                <input v-model.number="config.num_cells_z" type="number" step="1" min="1" class="input w-full text-xs" />
              </div>
            </div>
          </div>
        </div>

        <!-- Step 3: Material Properties -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            材料属性
          </h4>
          <div class="mb-3">
            <label class="label">基体材料</label>
            <select v-model="materialPreset" @change="onMaterialPresetChange" class="input w-full text-xs">
              <option value="aluminum">铝合金 (Al 6061)</option>
              <option value="steel">钢 (SS 304)</option>
              <option value="titanium">钛合金 (Ti-6Al-4V)</option>
              <option value="carbon_fiber">碳纤维复合材料</option>
              <option value="custom">自定义</option>
            </select>
          </div>
          <div class="space-y-2">
            <div>
              <label class="label">弹性模量 E (Pa)</label>
              <input v-model.number="config.base_material.E" type="number" step="1e9" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">泊松比 nu</label>
              <input v-model.number="config.base_material.nu" type="number" step="0.01" min="0" max="0.5" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">密度 (kg/m³)</label>
              <input v-model.number="config.base_material.density" type="number" step="10" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Step 4: Optimization Target -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            优化目标
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">目标相对密度</label>
              <input v-model.number="config.relative_density_target" type="number" step="0.01" min="0.01" max="0.99" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">最小刚度约束 (Pa)</label>
              <input v-model.number="minStiffnessConstraint" type="number" step="1e6" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="space-y-2">
          <button
            @click="generateStructure"
            :disabled="generating"
            class="btn btn-primary text-xs w-full"
          >
            <span v-if="generating" class="mr-1 animate-spin">&#10227;</span>
            {{ generating ? '生成中...' : '生成结构' }}
          </button>
          <button
            @click="runOptimization"
            :disabled="optimizing"
            class="btn btn-ghost text-xs w-full"
          >
            <span v-if="optimizing" class="mr-1 animate-spin">&#10227;</span>
            {{ optimizing ? '优化中...' : '优化参数' }}
          </button>
        </div>
      </div>

      <!-- Right Panel: Visualization -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- No results placeholder -->
        <div v-if="!results" class="flex-1 flex items-center justify-center" style="color: var(--text-muted)">
          <div class="text-center">
            <div class="text-4xl mb-2">&#128202;</div>
            <div class="text-sm">配置参数后生成蜂窝/点阵结构</div>
          </div>
        </div>

        <!-- Results Content -->
        <template v-else>
          <div class="flex-1 flex overflow-hidden">
            <!-- 3D Structure Preview Canvas -->
            <div class="flex-1 relative">
              <canvas ref="previewCanvas" class="w-full h-full" style="background: var(--bg-base)"></canvas>
            </div>

            <!-- Right Sidebar: Properties -->
            <div class="w-72 overflow-y-auto border-l p-4 space-y-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <!-- Equivalent Properties -->
              <div class="result-card">
                <span class="result-label">等效力学性能</span>
                <div class="space-y-2 mt-2">
                  <div v-for="prop in equivalentProps" :key="prop.key" class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">{{ prop.label }}</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ formatSci(prop.value) }}</span>
                  </div>
                </div>
              </div>

              <!-- Structure Statistics -->
              <div class="result-card">
                <span class="result-label">结构统计</span>
                <div class="space-y-2 mt-2">
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">相对密度</span>
                    <span class="text-xs font-mono font-semibold" style="color: var(--text-primary)">
                      {{ (results.relative_density * 100).toFixed(2) }}%
                    </span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">质量</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">
                      {{ results.mass.toFixed(4) }} kg
                    </span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">体积</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">
                      {{ formatSci(results.volume) }} mm³
                    </span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">胞元数</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">
                      {{ results.cell_count }}
                    </span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">节点数</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">
                      {{ results.nodes }}
                    </span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">单元数</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">
                      {{ results.elements }}
                    </span>
                  </div>
                </div>
              </div>

              <!-- Density Bar -->
              <div class="result-card">
                <span class="result-label">相对密度指示</span>
                <div class="mt-2">
                  <div class="w-full h-3 rounded-full" style="background: var(--bg-elevated)">
                    <div
                      class="h-3 rounded-full transition-all"
                      :style="{
                        width: Math.min(results.relative_density * 100, 100) + '%',
                        background: densityBarColor
                      }"
                    ></div>
                  </div>
                  <div class="flex justify-between mt-1">
                    <span class="text-[10px]" style="color: var(--text-muted)">0%</span>
                    <span class="text-[10px]" style="color: var(--text-muted)">100%</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type {
  CellType,
  CellularConfig,
  CellularResult,
  CellularTemplate,
  BaseMaterial,
} from '../api/cellularStructure'

// ============ Constants ============

const cellTypes = [
  { value: 'hexagonal' as CellType, label: '正六边形', desc: 'Hexagonal' },
  { value: 'triangular' as CellType, label: '三角形', desc: 'Triangular' },
  { value: 'square' as CellType, label: '方形', desc: 'Square' },
  { value: 're-entrant' as CellType, label: '内凹', desc: 'Re-entrant' },
  { value: 'chiral' as CellType, label: '手性', desc: 'Chiral' },
  { value: 'lattice_bcc' as CellType, label: 'BCC 点阵', desc: 'Body-Centered' },
  { value: 'lattice_fcc' as CellType, label: 'FCC 点阵', desc: 'Face-Centered' },
  { value: 'lattice_body_centered' as CellType, label: '体心立方', desc: 'BCC Cubic' },
]

const materialPresets: Record<string, BaseMaterial> = {
  aluminum: { name: '铝合金 Al 6061', E: 68.9e9, nu: 0.33, density: 2700 },
  steel: { name: '钢 SS 304', E: 193e9, nu: 0.29, density: 8000 },
  titanium: { name: '钛合金 Ti-6Al-4V', E: 113.8e9, nu: 0.342, density: 4430 },
  carbon_fiber: { name: '碳纤维复合材料', E: 138e9, nu: 0.3, density: 1600 },
}

const quickTemplates: CellularTemplate[] = [
  {
    id: 'standard_honeycomb',
    name: '标准蜂窝芯',
    name_en: 'Standard Honeycomb Core',
    category: 'honeycomb',
    description: '标准正六边形蜂窝芯，航空航天夹层板',
    cell_type: 'hexagonal',
    typical_application: '航空航天夹层板',
  },
  {
    id: 'lightweight_sandwich',
    name: '轻量化夹层板',
    name_en: 'Lightweight Sandwich Panel',
    category: 'sandwich',
    description: '轻量化夹层板蜂窝芯',
    cell_type: 'hexagonal',
    typical_application: '汽车轻量化',
  },
  {
    id: 'energy_absorption',
    name: '能量吸收结构',
    name_en: 'Energy Absorption Structure',
    category: 'crash',
    description: '内凹蜂窝能量吸收结构',
    cell_type: 're-entrant',
    typical_application: '碰撞防护',
  },
  {
    id: 'thermal_panel',
    name: '热防护面板',
    name_en: 'Thermal Protection Panel',
    category: 'thermal',
    description: '点阵热防护面板',
    cell_type: 'lattice_bcc',
    typical_application: '热防护系统',
  },
]

// ============ State ============

const generating = ref(false)
const optimizing = ref(false)
const materialPreset = ref('aluminum')
const minStiffnessConstraint = ref(1e6)
const results = ref<CellularResult | null>(null)
const previewCanvas = ref<HTMLCanvasElement | null>(null)

const config = reactive<CellularConfig>({
  cell_type: 'hexagonal',
  cell_size: 10,
  wall_thickness: 0.2,
  num_cells_x: 5,
  num_cells_y: 5,
  num_cells_z: 3,
  base_material: { ...materialPresets.aluminum },
  relative_density_target: 0.15,
})

// ============ Computed ============

const equivalentProps = computed(() => {
  if (!results.value) return []
  const ep = results.value.equivalent_properties
  return [
    { key: 'Ex', label: 'Ex (Pa)', value: ep.Ex },
    { key: 'Ey', label: 'Ey (Pa)', value: ep.Ey },
    { key: 'Ez', label: 'Ez (Pa)', value: ep.Ez },
    { key: 'Gxy', label: 'Gxy (Pa)', value: ep.Gxy },
    { key: 'Gyz', label: 'Gyz (Pa)', value: ep.Gyz },
    { key: 'Gxz', label: 'Gxz (Pa)', value: ep.Gxz },
    { key: 'nu_xy', label: 'nu_xy', value: ep.nu_xy },
    { key: 'nu_yz', label: 'nu_yz', value: ep.nu_yz },
  ]
})

const densityBarColor = computed(() => {
  if (!results.value) return 'var(--primary)'
  const rd = results.value.relative_density
  if (rd < 0.1) return '#22c55e'
  if (rd < 0.3) return '#4F8CFF'
  if (rd < 0.5) return '#eab308'
  return '#ef4444'
})

// ============ Methods ============

function onMaterialPresetChange() {
  const preset = materialPresets[materialPreset.value]
  if (preset) {
    config.base_material = { ...preset }
  }
}

function applyTemplate(tpl: CellularTemplate) {
  config.cell_type = tpl.cell_type
  switch (tpl.id) {
    case 'standard_honeycomb':
      config.cell_size = 10
      config.wall_thickness = 0.2
      config.num_cells_x = 5
      config.num_cells_y = 5
      config.num_cells_z = 3
      config.relative_density_target = 0.12
      materialPreset.value = 'aluminum'
      onMaterialPresetChange()
      break
    case 'lightweight_sandwich':
      config.cell_size = 8
      config.wall_thickness = 0.15
      config.num_cells_x = 6
      config.num_cells_y = 6
      config.num_cells_z = 2
      config.relative_density_target = 0.08
      materialPreset.value = 'carbon_fiber'
      onMaterialPresetChange()
      break
    case 'energy_absorption':
      config.cell_type = 're-entrant'
      config.cell_size = 12
      config.wall_thickness = 0.3
      config.num_cells_x = 4
      config.num_cells_y = 4
      config.num_cells_z = 5
      config.relative_density_target = 0.2
      materialPreset.value = 'aluminum'
      onMaterialPresetChange()
      break
    case 'thermal_panel':
      config.cell_type = 'lattice_bcc'
      config.cell_size = 6
      config.wall_thickness = 0.5
      config.num_cells_x = 8
      config.num_cells_y = 8
      config.num_cells_z = 4
      config.relative_density_target = 0.25
      materialPreset.value = 'titanium'
      onMaterialPresetChange()
      break
  }
}

function resetAll() {
  results.value = null
  config.cell_type = 'hexagonal'
  config.cell_size = 10
  config.wall_thickness = 0.2
  config.num_cells_x = 5
  config.num_cells_y = 5
  config.num_cells_z = 3
  config.relative_density_target = 0.15
  materialPreset.value = 'aluminum'
  onMaterialPresetChange()
  minStiffnessConstraint.value = 1e6
}

function formatSci(val: number): string {
  if (Math.abs(val) < 1e-15) return '0.00e+0'
  if (Math.abs(val) >= 1e6 || Math.abs(val) < 0.01) {
    return val.toExponential(2)
  }
  return val.toFixed(4)
}

/**
 * Compute relative density for hexagonal honeycomb
 * rho* / rho_s = (2/√3) * (t/l)
 */
function computeRelativeDensity(cellType: CellType, t: number, l: number): number {
  const ratio = t / l
  switch (cellType) {
    case 'hexagonal':
      return (2 / Math.sqrt(3)) * ratio
    case 'triangular':
      return 2 * Math.sqrt(3) * ratio
    case 'square':
      return 2 * ratio
    case 're-entrant':
      return (2 / Math.sqrt(3)) * ratio * 1.2
    case 'chiral':
      return 1.5 * ratio
    case 'lattice_bcc':
    case 'lattice_fcc':
    case 'lattice_body_centered':
      return 3 * Math.PI * (ratio / 2) ** 2
    default:
      return ratio
  }
}

/**
 * Compute equivalent properties using Gibson-Ashby model
 */
function computeEquivalentProperties(
  cellType: CellType,
  rd: number,
  baseMaterial: BaseMaterial
): CellularResult['equivalent_properties'] {
  const Es = baseMaterial.E
  const nus = baseMaterial.nu

  let E1: number, E2: number, G12: number, nu12: number

  switch (cellType) {
    case 'hexagonal':
      E1 = Es * rd
      E2 = Es * rd * rd
      G12 = Es * rd * rd / (2 * (1 + nus))
      nu12 = nus * Math.sqrt(rd)
      break
    case 'triangular':
      E1 = Es * rd / 3
      E2 = Es * rd / 3
      G12 = Es * rd / 8
      nu12 = nus / 3
      break
    case 'square':
      E1 = Es * rd / 2
      E2 = Es * rd / 2
      G12 = Es * rd / 4
      nu12 = nus / 2
      break
    case 're-entrant':
      E1 = Es * rd * 0.5
      E2 = Es * rd * rd * 0.8
      G12 = Es * rd * rd * 0.3
      nu12 = -0.3 * Math.sqrt(rd)
      break
    case 'chiral':
      E1 = Es * rd * 0.7
      E2 = Es * rd * 0.7
      G12 = Es * rd * 0.5
      nu12 = 0.1
      break
    case 'lattice_bcc':
    case 'lattice_fcc':
    case 'lattice_body_centered':
      E1 = Es * rd * 0.3
      E2 = Es * rd * 0.3
      G12 = Es * rd * 0.12
      nu12 = nus * 0.5
      break
    default:
      E1 = Es * rd
      E2 = Es * rd * rd
      G12 = Es * rd * rd / (2 * (1 + nus))
      nu12 = nus * Math.sqrt(rd)
  }

  return {
    Ex: E1,
    Ey: E2,
    Ez: E2 * 0.8,
    Gxy: G12,
    Gyz: G12 * 0.7,
    Gxz: G12 * 0.7,
    nu_xy: nu12,
    nu_yz: nus * 0.5,
  }
}

/**
 * Generate hexagonal cell vertices
 */
function generateHexVertices(cx: number, cy: number, size: number): Array<{ x: number; y: number }> {
  const vertices: Array<{ x: number; y: number }> = []
  for (let i = 0; i < 6; i++) {
    const angle = (Math.PI / 3) * i + Math.PI / 6
    vertices.push({
      x: cx + size * Math.cos(angle),
      y: cy + size * Math.sin(angle),
    })
  }
  return vertices
}

/**
 * Generate triangular cell vertices
 */
function generateTriVertices(cx: number, cy: number, size: number): Array<{ x: number; y: number }> {
  const vertices: Array<{ x: number; y: number }> = []
  for (let i = 0; i < 3; i++) {
    const angle = (2 * Math.PI / 3) * i - Math.PI / 2
    vertices.push({
      x: cx + size * Math.cos(angle),
      y: cy + size * Math.sin(angle),
    })
  }
  return vertices
}

/**
 * Generate square cell vertices
 */
function generateSquareVertices(cx: number, cy: number, size: number): Array<{ x: number; y: number }> {
  const h = size / 2
  return [
    { x: cx - h, y: cy - h },
    { x: cx + h, y: cy - h },
    { x: cx + h, y: cy + h },
    { x: cx - h, y: cy + h },
  ]
}

/**
 * Generate mock results
 */
function generateMockResults(): CellularResult {
  const rd = computeRelativeDensity(config.cell_type, config.wall_thickness, config.cell_size)
  const eqProps = computeEquivalentProperties(config.cell_type, rd, config.base_material)

  const totalW = config.num_cells_x * config.cell_size
  const totalH = config.num_cells_y * config.cell_size
  const totalD = config.num_cells_z * config.cell_size
  const volume = totalW * totalH * totalD
  const mass = rd * config.base_material.density * volume * 1e-9

  const cellCount = config.num_cells_x * config.num_cells_y * config.num_cells_z

  // Estimate nodes and elements based on cell type
  let nodesPerCell = 6
  let elemsPerCell = 6
  switch (config.cell_type) {
    case 'hexagonal': nodesPerCell = 6; elemsPerCell = 6; break
    case 'triangular': nodesPerCell = 3; elemsPerCell = 3; break
    case 'square': nodesPerCell = 4; elemsPerCell = 4; break
    case 're-entrant': nodesPerCell = 8; elemsPerCell = 8; break
    case 'chiral': nodesPerCell = 10; elemsPerCell = 10; break
    case 'lattice_bcc':
    case 'lattice_fcc':
    case 'lattice_body_centered':
      nodesPerCell = 9; elemsPerCell = 12; break
  }

  const totalNodes = Math.round(cellCount * nodesPerCell * 0.6)
  const totalElements = Math.round(cellCount * elemsPerCell * 0.8)

  // Generate simple mesh data for visualization
  const meshNodes: Array<{ id: number; x: number; y: number; z: number }> = []
  const meshElements: Array<{ id: number; type: string; nodes: number[] }> = []

  let nodeId = 0
  let elemId = 0

  for (let iz = 0; iz < config.num_cells_z; iz++) {
    for (let iy = 0; iy < config.num_cells_y; iy++) {
      for (let ix = 0; ix < config.num_cells_x; ix++) {
        const cx = ix * config.cell_size + config.cell_size / 2
        const cy = iy * config.cell_size + config.cell_size / 2
        const cz = iz * config.cell_size

        let vertices: Array<{ x: number; y: number }>
        switch (config.cell_type) {
          case 'triangular':
            vertices = generateTriVertices(cx, cy, config.cell_size / 2)
            break
          case 'square':
            vertices = generateSquareVertices(cx, cy, config.cell_size)
            break
          default:
            vertices = generateHexVertices(cx, cy, config.cell_size / 2)
        }

        const nodeIds: number[] = []
        for (const v of vertices) {
          meshNodes.push({ id: nodeId, x: v.x, y: v.y, z: cz })
          nodeIds.push(nodeId)
          nodeId++
        }

        // Create shell elements (quads/tris)
        for (let i = 0; i < vertices.length; i++) {
          const next = (i + 1) % vertices.length
          meshElements.push({
            id: elemId,
            type: 'S3',
            nodes: [nodeIds[i], nodeIds[next], nodeIds[0]],
          })
          elemId++
        }
      }
    }
  }

  return {
    success: true,
    relative_density: rd,
    equivalent_properties: eqProps,
    mass,
    volume,
    nodes: totalNodes,
    elements: totalElements,
    cell_count: cellCount,
    mesh_data: {
      nodes: meshNodes,
      elements: meshElements,
    },
  }
}

/**
 * Draw 3D isometric preview of cellular structure
 */
function drawPreview() {
  if (!previewCanvas.value || !results.value) return

  const canvas = previewCanvas.value
  const rect = canvas.parentElement?.getBoundingClientRect()
  if (!rect) return

  canvas.width = rect.width * window.devicePixelRatio
  canvas.height = rect.height * window.devicePixelRatio
  canvas.style.width = rect.width + 'px'
  canvas.style.height = rect.height + 'px'

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const scale = window.devicePixelRatio
  ctx.scale(scale, scale)

  const w = rect.width
  const h = rect.height

  ctx.clearRect(0, 0, w, h)
  ctx.fillStyle = '#0A0B0E'
  ctx.fillRect(0, 0, w, h)

  const meshData = results.value.mesh_data
  if (!meshData || meshData.nodes.length === 0) return

  // Isometric projection
  const angle = Math.PI / 6
  const cosA = Math.cos(angle)
  const sinA = Math.sin(angle)

  const totalW = config.num_cells_x * config.cell_size
  const totalH = config.num_cells_y * config.cell_size
  const totalD = config.num_cells_z * config.cell_size

  const maxDim = Math.max(totalW, totalH, totalD)
  const viewScale = Math.min(w, h) * 0.35 / maxDim

  function project(x: number, y: number, z: number): { px: number; py: number } {
    const isoX = (x - totalW / 2) * cosA - (y - totalH / 2) * cosA
    const isoY = (x - totalW / 2) * sinA + (y - totalH / 2) * sinA - z
    return {
      px: w / 2 + isoX * viewScale,
      py: h / 2 + isoY * viewScale,
    }
  }

  // Collect all edges from elements
  const edgeSet = new Set<string>()
  const edges: Array<{ x1: number; y1: number; z1: number; x2: number; y2: number; z2: number; z: number }> = []

  for (const elem of meshData.elements) {
    const ns = elem.nodes
    for (let i = 0; i < ns.length; i++) {
      const a = ns[i]
      const b = ns[(i + 1) % ns.length]
      const key = Math.min(a, b) + '-' + Math.max(a, b)
      if (!edgeSet.has(key)) {
        edgeSet.add(key)
        const na = meshData.nodes.find(n => n.id === a)
        const nb = meshData.nodes.find(n => n.id === b)
        if (na && nb) {
          edges.push({
            x1: na.x, y1: na.y, z1: na.z,
            x2: nb.x, y2: nb.y, z2: nb.z,
            z: (na.z + nb.z) / 2,
          })
        }
      }
    }
  }

  // Sort edges by depth (painter's algorithm)
  edges.sort((a, b) => {
    const depthA = (a.z1 + a.z2) / 2
    const depthB = (b.z1 + b.z2) / 2
    return depthA - depthB
  })

  // Draw edges with depth-based coloring
  const minZ = 0
  const maxZ = totalD

  for (const edge of edges) {
    const p1 = project(edge.x1, edge.y1, edge.z1)
    const p2 = project(edge.x2, edge.y2, edge.z2)

    const depth = (edge.z - minZ) / (maxZ - minZ + 1e-10)
    const brightness = 0.3 + 0.7 * (1 - depth)

    const r = Math.round(79 * brightness)
    const g = Math.round(140 * brightness)
    const b = Math.round(255 * brightness)

    ctx.strokeStyle = `rgb(${r},${g},${b})`
    ctx.lineWidth = 1
    ctx.beginPath()
    ctx.moveTo(p1.px, p1.py)
    ctx.lineTo(p2.px, p2.py)
    ctx.stroke()
  }

  // Draw nodes as small dots
  for (const node of meshData.nodes) {
    const p = project(node.x, node.y, node.z)
    const depth = (node.z - minZ) / (maxZ - minZ + 1e-10)
    const brightness = 0.3 + 0.7 * (1 - depth)

    ctx.fillStyle = `rgba(255, 255, 255, ${brightness * 0.5})`
    ctx.beginPath()
    ctx.arc(p.px, p.py, 1.5, 0, Math.PI * 2)
    ctx.fill()
  }

  // Title
  ctx.fillStyle = '#E8E9EB'
  ctx.font = '12px sans-serif'
  ctx.textAlign = 'center'
  const cellLabel = cellTypes.find(ct => ct.value === config.cell_type)?.label ?? config.cell_type
  ctx.fillText(`${cellLabel} 蜂窝/点阵结构 - ${config.num_cells_x}x${config.num_cells_y}x${config.num_cells_z}`, w / 2, 20)

  // Axis indicator
  const axLen = 40
  const axOrigin = { x: 50, y: h - 50 }
  const axX = project(totalW / 2 + axLen, totalH / 2, 0)
  const axY = project(totalW / 2, totalH / 2 + axLen, 0)
  const axZ = project(totalW / 2, totalH / 2, axLen)

  // Normalize axis arrows to fixed length from origin
  function normArrow(target: { px: number; py: number }, origin: { x: number; y: number }, len: number): { ex: number; ey: number } {
    const dx = target.px - origin.x
    const dy = target.py - origin.y
    const mag = Math.sqrt(dx * dx + dy * dy)
    if (mag < 1e-10) return { ex: origin.x, ey: origin.y }
    return { ex: origin.x + dx / mag * len, ey: origin.y + dy / mag * len }
  }

  const axes = [
    { end: axX, color: '#ef4444', label: 'X' },
    { end: axY, color: '#22c55e', label: 'Y' },
    { end: axZ, color: '#4F8CFF', label: 'Z' },
  ]

  for (const axis of axes) {
    const { ex, ey } = normArrow(axis.end, axOrigin, axLen)
    ctx.strokeStyle = axis.color
    ctx.lineWidth = 2
    ctx.beginPath()
    ctx.moveTo(axOrigin.x, axOrigin.y)
    ctx.lineTo(ex, ey)
    ctx.stroke()

    ctx.fillStyle = axis.color
    ctx.font = '10px sans-serif'
    ctx.textAlign = 'center'
    ctx.fillText(axis.label, ex + (ex - axOrigin.x) * 0.3, ey + (ey - axOrigin.y) * 0.3)
  }
}

async function generateStructure() {
  generating.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 1500))
    results.value = generateMockResults()
    await nextTick()
    drawPreview()
  } catch (e) {
    console.error('Structure generation failed:', e)
  } finally {
    generating.value = false
  }
}

async function runOptimization() {
  optimizing.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 2000))

    // Simple optimization: adjust wall_thickness to match target density
    const targetRd = config.relative_density_target
    let bestT = config.wall_thickness
    let bestDiff = Infinity

    for (let t = 0.05; t <= config.cell_size * 0.5; t += 0.01) {
      const rd = computeRelativeDensity(config.cell_type, t, config.cell_size)
      const diff = Math.abs(rd - targetRd)
      if (diff < bestDiff) {
        bestDiff = diff
        bestT = t
      }
    }

    config.wall_thickness = parseFloat(bestT.toFixed(3))
    results.value = generateMockResults()
    await nextTick()
    drawPreview()
  } catch (e) {
    console.error('Optimization failed:', e)
  } finally {
    optimizing.value = false
  }
}

function exportMesh() {
  if (!results.value) return
  const data = JSON.stringify(results.value.mesh_data, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `cellular_mesh_${config.cell_type}.json`
  a.click()
  URL.revokeObjectURL(url)
}

// ============ Lifecycle ============

onMounted(() => {
  onMaterialPresetChange()
})
</script>

<style scoped>
.panel-section {
  margin-bottom: 4px;
  padding: 12px;
  background: var(--bg-elevated);
  border-radius: var(--radius-md);
}

.label {
  display: block;
  font-size: 10px;
  color: var(--text-muted);
  margin-bottom: 4px;
  text-transform: uppercase;
}

.input {
  width: 100%;
  padding: 8px;
  background: var(--bg-base);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 12px;
  transition: border-color var(--duration-fast) var(--ease-out);
}

.input:focus {
  outline: none;
  border-color: var(--primary);
  box-shadow: 0 0 0 2px var(--primary-glow);
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 8px 16px;
  border-radius: var(--radius-md);
  font-weight: 500;
  transition: all var(--duration-fast) var(--ease-out);
  cursor: pointer;
  border: none;
}

.btn-primary {
  background: var(--primary);
  color: #fff;
}

.btn-primary:hover {
  opacity: 0.9;
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-ghost {
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border-default);
}

.btn-ghost:hover {
  background: var(--bg-elevated);
}

.btn-ghost:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.result-card {
  display: flex;
  flex-direction: column;
  padding: 12px;
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
}

.result-label {
  font-size: 10px;
  color: var(--text-muted);
  text-transform: uppercase;
  margin-bottom: 4px;
}

.result-value {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}
</style>
