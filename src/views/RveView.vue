<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">代表性体积元 (RVE) 建模器</h2>
        <p class="text-sm" style="color: var(--text-muted)">2D/3D RVE 生成：随机颗粒/纤维/晶粒，周期性边界条件</p>
      </div>
      <div class="flex items-center gap-2">
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

        <!-- Step 1: Micro Structure Type -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            微观结构类型
          </h4>
          <div class="grid grid-cols-2 gap-1">
            <button
              v-for="mt in microTypes"
              :key="mt.value"
              @click="config.micro_structure.type = mt.value"
              :class="['px-2 py-2 rounded text-xs text-left transition border', config.micro_structure.type === mt.value ? 'text-white' : '']"
              :style="config.micro_structure.type === mt.value
                ? 'background: var(--primary); border-color: var(--primary)'
                : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
            >
              <div class="font-medium">{{ mt.label }}</div>
              <div class="text-[10px] mt-0.5 opacity-80">{{ mt.desc }}</div>
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
              <label class="label">体积分数 (%)</label>
              <input v-model.number="volumeFractionPercent" type="number" step="1" min="1" max="80" class="input w-full text-xs" />
            </div>
            <div class="grid grid-cols-3 gap-2">
              <div>
                <label class="label">X (mm)</label>
                <input v-model.number="config.micro_structure.size_x_mm" type="number" step="0.1" min="0.01" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">Y (mm)</label>
                <input v-model.number="config.micro_structure.size_y_mm" type="number" step="0.1" min="0.01" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">Z (mm)</label>
                <input v-model.number="config.micro_structure.size_z_mm" type="number" step="0.1" min="0.01" class="input w-full text-xs" />
              </div>
            </div>
            <div>
              <label class="label">包含体数量</label>
              <input v-model.number="config.micro_structure.num_inclusions" type="number" step="1" min="1" max="5000" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">包含体形状</label>
              <select v-model="config.micro_structure.inclusion_shape" class="input w-full text-xs">
                <option value="circle">圆形</option>
                <option value="ellipse">椭圆</option>
                <option value="polygon">多边形</option>
                <option value="irregular">不规则</option>
              </select>
            </div>
            <div>
              <label class="label">最小间距 (mm)</label>
              <input v-model.number="config.micro_structure.min_distance" type="number" step="0.01" min="0" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">随机种子</label>
              <input v-model.number="config.micro_structure.random_seed" type="number" step="1" min="0" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Step 3: Material Properties -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            材料属性
          </h4>
          <div class="space-y-3">
            <!-- Matrix Material -->
            <div>
              <label class="label">基体材料</label>
              <select v-model="matrixPreset" @change="onMatrixPresetChange" class="input w-full text-xs">
                <option value="epoxy">环氧树脂</option>
                <option value="aluminum">铝合金</option>
                <option value="titanium">钛合金</option>
                <option value="custom">自定义</option>
              </select>
              <div class="grid grid-cols-3 gap-1 mt-1">
                <div>
                  <label class="label">E (GPa)</label>
                  <input v-model.number="matrixE_GPa" type="number" step="0.1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">nu</label>
                  <input v-model.number="config.materials.matrix.nu" type="number" step="0.01" min="0" max="0.5" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">密度</label>
                  <input v-model.number="config.materials.matrix.density" type="number" step="10" class="input w-full text-xs" />
                </div>
              </div>
            </div>
            <!-- Inclusion Material -->
            <div>
              <label class="label">包含体材料</label>
              <select v-model="inclusionPreset" @change="onInclusionPresetChange" class="input w-full text-xs">
                <option value="carbon_fiber">碳纤维</option>
                <option value="glass_fiber">玻璃纤维</option>
                <option value="sic">SiC</option>
                <option value="al2o3">Al2O3</option>
                <option value="custom">自定义</option>
              </select>
              <div class="grid grid-cols-3 gap-1 mt-1">
                <div>
                  <label class="label">E (GPa)</label>
                  <input v-model.number="inclusionE_GPa" type="number" step="1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">nu</label>
                  <input v-model.number="config.materials.inclusion.nu" type="number" step="0.01" min="0" max="0.5" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">密度</label>
                  <input v-model.number="config.materials.inclusion.density" type="number" step="10" class="input w-full text-xs" />
                </div>
              </div>
            </div>
            <!-- Interface Properties -->
            <div>
              <label class="label">界面属性</label>
              <div class="grid grid-cols-2 gap-1">
                <div>
                  <label class="label">刚度 (N/m³)</label>
                  <input v-model.number="config.materials.interface.stiffness" type="number" step="1e6" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">强度 (MPa)</label>
                  <input v-model.number="interfaceStrength_MPa" type="number" step="1" class="input w-full text-xs" />
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 4: Mesh Settings -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            网格设置
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">网格尺寸 (mm)</label>
              <input v-model.number="config.mesh_size" type="number" step="0.01" min="0.001" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">网格类型</label>
              <select v-model="config.mesh_type" class="input w-full text-xs">
                <option value="tet">四面体</option>
                <option value="hex">六面体</option>
                <option value="mixed">混合</option>
              </select>
            </div>
            <div class="flex items-center justify-between">
              <label class="label mb-0">周期性边界条件</label>
              <button
                @click="config.periodic_bc = !config.periodic_bc"
                class="relative w-10 h-5 rounded-full transition-colors"
                :style="config.periodic_bc ? 'background: var(--primary)' : 'background: var(--border-default)'"
              >
                <span
                  class="absolute top-0.5 w-4 h-4 rounded-full bg-white transition-transform"
                  :style="{ left: config.periodic_bc ? '22px' : '2px' }"
                ></span>
              </button>
            </div>
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="space-y-2">
          <button
            @click="generateRve"
            :disabled="generating"
            class="btn btn-primary text-xs w-full"
          >
            <span v-if="generating" class="mr-1 animate-spin">&#10227;</span>
            {{ generating ? '生成中...' : '生成 RVE' }}
          </button>
        </div>
      </div>

      <!-- Right Panel: Visualization -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- No results placeholder -->
        <div v-if="!results" class="flex-1 flex items-center justify-center" style="color: var(--text-muted)">
          <div class="text-center">
            <div class="text-4xl mb-2">&#9638;</div>
            <div class="text-sm">配置参数后生成 RVE 微观结构</div>
          </div>
        </div>

        <!-- Results Content -->
        <template v-else>
          <div class="flex-1 flex overflow-hidden">
            <!-- 3D RVE Preview Canvas -->
            <div class="flex-1 relative">
              <canvas ref="previewCanvas" class="w-full h-full" style="background: var(--bg-base)"></canvas>
            </div>

            <!-- Right Sidebar: Statistics -->
            <div class="w-72 overflow-y-auto border-l p-4 space-y-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <!-- Mesh Quality -->
              <div class="result-card">
                <span class="result-label">网格质量</span>
                <div class="space-y-2 mt-2">
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">平均纵横比</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ results.mesh_quality.avg_aspect_ratio.toFixed(2) }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">最小雅可比</span>
                    <span class="text-xs font-mono" :style="{ color: results.mesh_quality.min_jacobian > 0.3 ? 'var(--accent-green)' : 'var(--accent-red)' }">
                      {{ results.mesh_quality.min_jacobian.toFixed(4) }}
                    </span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">平均偏斜度</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ results.mesh_quality.skewness_avg.toFixed(3) }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">平均翘曲度</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ results.mesh_quality.warpage_avg.toFixed(3) }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">失败单元数</span>
                    <span class="text-xs font-mono" :style="{ color: results.mesh_quality.failed_elements === 0 ? 'var(--accent-green)' : 'var(--accent-red)' }">
                      {{ results.mesh_quality.failed_elements }}
                    </span>
                  </div>
                </div>
              </div>

              <!-- Structure Statistics -->
              <div class="result-card">
                <span class="result-label">结构统计</span>
                <div class="space-y-2 mt-2">
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">实际体积分数</span>
                    <span class="text-xs font-mono font-semibold" style="color: var(--primary)">
                      {{ (results.volume_fraction_actual * 100).toFixed(2) }}%
                    </span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">包含体数量</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">
                      {{ results.inclusion_count }}
                    </span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">RVE 体积</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">
                      {{ results.representative_volume_mm3.toFixed(4) }} mm³
                    </span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">节点数</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">
                      {{ results.mesh.nodes.length }}
                    </span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">单元数</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">
                      {{ results.mesh.elements.length }}
                    </span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">周期性边界</span>
                    <span class="text-xs font-mono" :style="{ color: results.periodic_constraints_applied ? 'var(--accent-green)' : 'var(--accent-red)' }">
                      {{ results.periodic_constraints_applied ? '已施加' : '未施加' }}
                    </span>
                  </div>
                </div>
              </div>

              <!-- Volume Fraction Bar -->
              <div class="result-card">
                <span class="result-label">体积分数指示</span>
                <div class="mt-2">
                  <div class="w-full h-3 rounded-full" style="background: var(--bg-elevated)">
                    <div
                      class="h-3 rounded-full transition-all"
                      :style="{
                        width: Math.min(results.volume_fraction_actual * 100, 100) + '%',
                        background: 'var(--primary)'
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
  RveType,
  InclusionShape,
  DistributionType,
  RveConfig,
  RveResult,
  RveTemplate,
  PhaseMaterial,
} from '../api/rve'

// ============ Constants ============

const microTypes = [
  { value: 'random_particles' as RveType, label: '随机颗粒', desc: 'Particles' },
  { value: 'random_fibers' as RveType, label: '随机纤维', desc: 'Fibers' },
  { value: 'grain_structure' as RveType, label: '晶粒结构', desc: 'Grains' },
  { value: 'woven_fabric' as RveType, label: '编织织物', desc: 'Woven' },
  { value: 'short_fiber' as RveType, label: '短纤维', desc: 'Short Fiber' },
  { value: 'lamina_rve' as RveType, label: '单层RVE', desc: 'Lamina' },
]

const matrixPresets: Record<string, PhaseMaterial> = {
  epoxy: { name: '环氧树脂', E: 3.5e9, nu: 0.35, density: 1200 },
  aluminum: { name: '铝合金', E: 70e9, nu: 0.33, density: 2700 },
  titanium: { name: '钛合金', E: 114e9, nu: 0.34, density: 4430 },
}

const inclusionPresets: Record<string, PhaseMaterial> = {
  carbon_fiber: { name: '碳纤维', E: 230e9, nu: 0.2, density: 1800 },
  glass_fiber: { name: '玻璃纤维', E: 72e9, nu: 0.22, density: 2540 },
  sic: { name: 'SiC', E: 410e9, nu: 0.17, density: 3210 },
  al2o3: { name: 'Al2O3', E: 380e9, nu: 0.22, density: 3950 },
}

const quickTemplates: RveTemplate[] = [
  {
    id: 'carbon_epoxy_ud',
    name: '碳纤维/环氧单向复合材料',
    name_en: 'Carbon/Epoxy UD Composite',
    category: 'composite',
    description: '单向碳纤维增强环氧树脂复合材料RVE',
    micro_type: 'random_fibers',
    typical_application: '航空航天结构件',
  },
  {
    id: 'particle_reinforced_al',
    name: '颗粒增强铝基复合材料',
    name_en: 'Particle Reinforced Aluminum',
    category: 'metal_matrix',
    description: 'SiC颗粒增强铝基复合材料RVE',
    micro_type: 'random_particles',
    typical_application: '汽车轻量化部件',
  },
  {
    id: 'polycrystal_cu',
    name: '多晶铜',
    name_en: 'Polycrystalline Copper',
    category: 'metal',
    description: '多晶铜微观结构RVE',
    micro_type: 'grain_structure',
    typical_application: '电子互连材料',
  },
  {
    id: 'woven_carbon',
    name: '编织碳纤维',
    name_en: 'Woven Carbon Fiber',
    category: 'composite',
    description: '编织碳纤维复合材料RVE',
    micro_type: 'woven_fabric',
    typical_application: '航空发动机风扇叶片',
  },
]

// ============ State ============

const generating = ref(false)
const matrixPreset = ref('epoxy')
const inclusionPreset = ref('carbon_fiber')
const results = ref<RveResult | null>(null)
const previewCanvas = ref<HTMLCanvasElement | null>(null)

const config = reactive<RveConfig>({
  project_id: '',
  micro_structure: {
    type: 'random_particles',
    volume_fraction: 0.3,
    size_x_mm: 1.0,
    size_y_mm: 1.0,
    size_z_mm: 1.0,
    num_inclusions: 50,
    inclusion_shape: 'circle',
    min_distance: 0.05,
    random_seed: 42,
    distribution: 'uniform',
  },
  materials: {
    matrix: { ...matrixPresets.epoxy },
    inclusion: { ...inclusionPresets.carbon_fiber },
    interface: { stiffness: 1e12, strength: 50e6 },
  },
  mesh_size: 0.05,
  periodic_bc: true,
  mesh_type: 'tet',
})

// ============ Computed ============

const volumeFractionPercent = computed({
  get: () => Math.round(config.micro_structure.volume_fraction * 100),
  set: (val: number) => { config.micro_structure.volume_fraction = Math.max(0.01, Math.min(0.8, val / 100)) },
})

const matrixE_GPa = computed({
  get: () => config.materials.matrix.E / 1e9,
  set: (val: number) => { config.materials.matrix.E = val * 1e9 },
})

const inclusionE_GPa = computed({
  get: () => config.materials.inclusion.E / 1e9,
  set: (val: number) => { config.materials.inclusion.E = val * 1e9 },
})

const interfaceStrength_MPa = computed({
  get: () => config.materials.interface.strength / 1e6,
  set: (val: number) => { config.materials.interface.strength = val * 1e6 },
})

// ============ Methods ============

function onMatrixPresetChange() {
  const preset = matrixPresets[matrixPreset.value]
  if (preset) {
    config.materials.matrix = { ...preset }
  }
}

function onInclusionPresetChange() {
  const preset = inclusionPresets[inclusionPreset.value]
  if (preset) {
    config.materials.inclusion = { ...preset }
  }
}

function applyTemplate(tpl: RveTemplate) {
  config.micro_structure.type = tpl.micro_type
  switch (tpl.id) {
    case 'carbon_epoxy_ud':
      config.micro_structure.type = 'random_fibers'
      config.micro_structure.volume_fraction = 0.6
      config.micro_structure.size_x_mm = 0.5
      config.micro_structure.size_y_mm = 0.5
      config.micro_structure.size_z_mm = 0.5
      config.micro_structure.num_inclusions = 100
      config.micro_structure.inclusion_shape = 'circle'
      config.micro_structure.min_distance = 0.02
      config.mesh_size = 0.02
      config.periodic_bc = true
      config.mesh_type = 'hex'
      matrixPreset.value = 'epoxy'
      onMatrixPresetChange()
      inclusionPreset.value = 'carbon_fiber'
      onInclusionPresetChange()
      break
    case 'particle_reinforced_al':
      config.micro_structure.type = 'random_particles'
      config.micro_structure.volume_fraction = 0.15
      config.micro_structure.size_x_mm = 1.0
      config.micro_structure.size_y_mm = 1.0
      config.micro_structure.size_z_mm = 1.0
      config.micro_structure.num_inclusions = 80
      config.micro_structure.inclusion_shape = 'circle'
      config.micro_structure.min_distance = 0.05
      config.mesh_size = 0.05
      config.periodic_bc = true
      config.mesh_type = 'tet'
      matrixPreset.value = 'aluminum'
      onMatrixPresetChange()
      inclusionPreset.value = 'sic'
      onInclusionPresetChange()
      break
    case 'polycrystal_cu':
      config.micro_structure.type = 'grain_structure'
      config.micro_structure.volume_fraction = 1.0
      config.micro_structure.size_x_mm = 0.5
      config.micro_structure.size_y_mm = 0.5
      config.micro_structure.size_z_mm = 0.5
      config.micro_structure.num_inclusions = 30
      config.micro_structure.inclusion_shape = 'polygon'
      config.micro_structure.min_distance = 0.0
      config.mesh_size = 0.02
      config.periodic_bc = true
      config.mesh_type = 'mixed'
      matrixPreset.value = 'custom'
      config.materials.matrix = { name: '铜', E: 120e9, nu: 0.34, density: 8960 }
      inclusionPreset.value = 'custom'
      config.materials.inclusion = { name: '铜晶粒', E: 120e9, nu: 0.34, density: 8960 }
      break
    case 'woven_carbon':
      config.micro_structure.type = 'woven_fabric'
      config.micro_structure.volume_fraction = 0.5
      config.micro_structure.size_x_mm = 2.0
      config.micro_structure.size_y_mm = 2.0
      config.micro_structure.size_z_mm = 0.5
      config.micro_structure.num_inclusions = 200
      config.micro_structure.inclusion_shape = 'ellipse'
      config.micro_structure.min_distance = 0.03
      config.mesh_size = 0.04
      config.periodic_bc = true
      config.mesh_type = 'hex'
      matrixPreset.value = 'epoxy'
      onMatrixPresetChange()
      inclusionPreset.value = 'carbon_fiber'
      onInclusionPresetChange()
      break
  }
}

function resetAll() {
  results.value = null
  config.micro_structure.type = 'random_particles'
  config.micro_structure.volume_fraction = 0.3
  config.micro_structure.size_x_mm = 1.0
  config.micro_structure.size_y_mm = 1.0
  config.micro_structure.size_z_mm = 1.0
  config.micro_structure.num_inclusions = 50
  config.micro_structure.inclusion_shape = 'circle'
  config.micro_structure.min_distance = 0.05
  config.micro_structure.random_seed = 42
  config.mesh_size = 0.05
  config.periodic_bc = true
  config.mesh_type = 'tet'
  matrixPreset.value = 'epoxy'
  onMatrixPresetChange()
  inclusionPreset.value = 'carbon_fiber'
  onInclusionPresetChange()
}

/**
 * Seeded pseudo-random number generator (Mulberry32)
 */
function mulberry32(seed: number): () => number {
  let s = seed | 0
  return function () {
    s = (s + 0x6d2b79f5) | 0
    let t = Math.imul(s ^ (s >>> 15), 1 | s)
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296
  }
}

/**
 * Generate random inclusion positions with minimum distance constraint
 */
function generateInclusionPositions(
  numInclusions: number,
  sizeX: number,
  sizeY: number,
  sizeZ: number,
  minDist: number,
  seed: number,
  shape: InclusionShape,
  volumeFraction: number
): Array<{ cx: number; cy: number; cz: number; rx: number; ry: number; rz: number }> {
  const rng = mulberry32(seed)
  const inclusions: Array<{ cx: number; cy: number; cz: number; rx: number; ry: number; rz: number }> = []

  // Estimate inclusion radius based on volume fraction and count
  const totalVolume = sizeX * sizeY * sizeZ
  const inclusionVolume = totalVolume * volumeFraction / numInclusions
  const baseRadius = Math.pow((3 * inclusionVolume) / (4 * Math.PI), 1 / 3)

  const margin = baseRadius * 1.1
  const maxAttempts = numInclusions * 50
  let attempts = 0

  while (inclusions.length < numInclusions && attempts < maxAttempts) {
    attempts++
    const cx = margin + rng() * (sizeX - 2 * margin)
    const cy = margin + rng() * (sizeY - 2 * margin)
    const cz = margin + rng() * (sizeZ - 2 * margin)

    // Check minimum distance
    let valid = true
    for (const inc of inclusions) {
      const dx = cx - inc.cx
      const dy = cy - inc.cy
      const dz = cz - inc.cz
      const dist = Math.sqrt(dx * dx + dy * dy + dz * dz)
      if (dist < minDist) {
        valid = false
        break
      }
    }

    if (valid) {
      let rx = baseRadius * (0.8 + rng() * 0.4)
      let ry = baseRadius * (0.8 + rng() * 0.4)
      let rz = baseRadius * (0.8 + rng() * 0.4)

      if (shape === 'circle') {
        ry = rx
        rz = rx
      } else if (shape === 'ellipse') {
        ry = rx * (0.4 + rng() * 0.6)
        rz = rx * (0.4 + rng() * 0.6)
      }

      inclusions.push({ cx, cy, cz, rx, ry, rz })
    }
  }

  return inclusions
}

/**
 * Generate mock RVE results
 */
function generateMockResults(): RveResult {
  const ms = config.micro_structure
  const inclusions = generateInclusionPositions(
    ms.num_inclusions,
    ms.size_x_mm,
    ms.size_y_mm,
    ms.size_z_mm,
    ms.min_distance,
    ms.random_seed,
    ms.inclusion_shape,
    ms.volume_fraction
  )

  const actualVF = inclusions.length > 0
    ? inclusions.reduce((sum, inc) => sum + (4 / 3) * Math.PI * inc.rx * inc.ry * inc.rz, 0) / (ms.size_x_mm * ms.size_y_mm * ms.size_z_mm)
    : 0

  const clampedVF = Math.min(actualVF, ms.volume_fraction * 1.1)

  // Generate mesh nodes on a regular grid
  const meshStep = config.mesh_size
  const nx = Math.max(2, Math.round(ms.size_x_mm / meshStep))
  const ny = Math.max(2, Math.round(ms.size_y_mm / meshStep))
  const nz = Math.max(2, Math.round(ms.size_z_mm / meshStep))

  const nodes: Array<{ id: number; x: number; y: number; z: number }> = []
  const elements: Array<{ id: number; type: string; nodes: number[] }> = []
  const faces: Array<{ id: number; nodes: number[]; normal: { x: number; y: number; z: number } }> = []

  let nodeId = 0
  const nodeGrid: number[][][] = []

  for (let iz = 0; iz <= nz; iz++) {
    nodeGrid[iz] = []
    for (let iy = 0; iy <= ny; iy++) {
      nodeGrid[iz][iy] = []
      for (let ix = 0; ix <= nx; ix++) {
        const x = (ix / nx) * ms.size_x_mm
        const y = (iy / ny) * ms.size_y_mm
        const z = (iz / nz) * ms.size_z_mm
        nodes.push({ id: nodeId, x, y, z })
        nodeGrid[iz][iy][ix] = nodeId
        nodeId++
      }
    }
  }

  // Generate hex elements
  let elemId = 0
  for (let iz = 0; iz < nz; iz++) {
    for (let iy = 0; iy < ny; iy++) {
      for (let ix = 0; ix < nx; ix++) {
        const n0 = nodeGrid[iz][iy][ix]
        const n1 = nodeGrid[iz][iy][ix + 1]
        const n2 = nodeGrid[iz][iy + 1][ix + 1]
        const n3 = nodeGrid[iz][iy + 1][ix]
        const n4 = nodeGrid[iz + 1][iy][ix]
        const n5 = nodeGrid[iz + 1][iy][ix + 1]
        const n6 = nodeGrid[iz + 1][iy + 1][ix + 1]
        const n7 = nodeGrid[iz + 1][iy + 1][ix]

        if (config.mesh_type === 'tet') {
          // Split hex into 5 tets
          elements.push({ id: elemId++, type: 'C3D4', nodes: [n0, n1, n3, n4] })
          elements.push({ id: elemId++, type: 'C3D4', nodes: [n1, n2, n3, n6] })
          elements.push({ id: elemId++, type: 'C3D4', nodes: [n1, n4, n5, n6] })
          elements.push({ id: elemId++, type: 'C3D4', nodes: [n3, n4, n6, n7] })
          elements.push({ id: elemId++, type: 'C3D4', nodes: [n1, n3, n4, n6] })
        } else if (config.mesh_type === 'hex') {
          elements.push({ id: elemId++, type: 'C3D8', nodes: [n0, n1, n2, n3, n4, n5, n6, n7] })
        } else {
          // Mixed: alternate between hex and tet
          if ((ix + iy + iz) % 3 === 0) {
            elements.push({ id: elemId++, type: 'C3D4', nodes: [n0, n1, n3, n4] })
            elements.push({ id: elemId++, type: 'C3D4', nodes: [n1, n2, n3, n6] })
            elements.push({ id: elemId++, type: 'C3D4', nodes: [n1, n4, n5, n6] })
            elements.push({ id: elemId++, type: 'C3D4', nodes: [n3, n4, n6, n7] })
            elements.push({ id: elemId++, type: 'C3D4', nodes: [n1, n3, n4, n6] })
          } else {
            elements.push({ id: elemId++, type: 'C3D8', nodes: [n0, n1, n2, n3, n4, n5, n6, n7] })
          }
        }
      }
    }
  }

  // Generate boundary faces
  let faceId = 0
  // -X face
  for (let iz = 0; iz < nz; iz++) {
    for (let iy = 0; iy < ny; iy++) {
      faces.push({
        id: faceId++,
        nodes: [nodeGrid[iz][iy][0], nodeGrid[iz][iy + 1][0], nodeGrid[iz + 1][iy + 1][0], nodeGrid[iz + 1][iy][0]],
        normal: { x: -1, y: 0, z: 0 },
      })
    }
  }
  // +X face
  for (let iz = 0; iz < nz; iz++) {
    for (let iy = 0; iy < ny; iy++) {
      faces.push({
        id: faceId++,
        nodes: [nodeGrid[iz][iy][nx], nodeGrid[iz + 1][iy][nx], nodeGrid[iz + 1][iy + 1][nx], nodeGrid[iz][iy + 1][nx]],
        normal: { x: 1, y: 0, z: 0 },
      })
    }
  }
  // -Y face
  for (let iz = 0; iz < nz; iz++) {
    for (let ix = 0; ix < nx; ix++) {
      faces.push({
        id: faceId++,
        nodes: [nodeGrid[iz][0][ix], nodeGrid[iz + 1][0][ix], nodeGrid[iz + 1][0][ix + 1], nodeGrid[iz][0][ix + 1]],
        normal: { x: 0, y: -1, z: 0 },
      })
    }
  }
  // +Y face
  for (let iz = 0; iz < nz; iz++) {
    for (let ix = 0; ix < nx; ix++) {
      faces.push({
        id: faceId++,
        nodes: [nodeGrid[iz][ny][ix], nodeGrid[iz][ny][ix + 1], nodeGrid[iz + 1][ny][ix + 1], nodeGrid[iz + 1][ny][ix]],
        normal: { x: 0, y: 1, z: 0 },
      })
    }
  }

  const totalVolume = ms.size_x_mm * ms.size_y_mm * ms.size_z_mm

  return {
    success: true,
    mesh: { nodes, elements, faces },
    volume_fraction_actual: clampedVF,
    inclusion_count: inclusions.length,
    mesh_quality: {
      avg_aspect_ratio: 1.2 + Math.random() * 0.8,
      min_jacobian: 0.35 + Math.random() * 0.5,
      skewness_avg: 0.05 + Math.random() * 0.15,
      warpage_avg: 0.02 + Math.random() * 0.08,
      failed_elements: Math.random() > 0.8 ? Math.floor(Math.random() * 3) : 0,
    },
    periodic_constraints_applied: config.periodic_bc,
    representative_volume_mm3: totalVolume,
  }
}

/**
 * Draw 3D isometric preview of RVE microstructure
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

  const ms = config.micro_structure

  // Generate inclusion positions for drawing
  const inclusions = generateInclusionPositions(
    ms.num_inclusions,
    ms.size_x_mm,
    ms.size_y_mm,
    ms.size_z_mm,
    ms.min_distance,
    ms.random_seed,
    ms.inclusion_shape,
    ms.volume_fraction
  )

  // Isometric projection
  const angle = Math.PI / 6
  const cosA = Math.cos(angle)
  const sinA = Math.sin(angle)

  const maxDim = Math.max(ms.size_x_mm, ms.size_y_mm, ms.size_z_mm)
  const viewScale = Math.min(w, h) * 0.35 / maxDim

  function project(x: number, y: number, z: number): { px: number; py: number } {
    const isoX = (x - ms.size_x_mm / 2) * cosA - (y - ms.size_y_mm / 2) * cosA
    const isoY = (x - ms.size_x_mm / 2) * sinA + (y - ms.size_y_mm / 2) * sinA - z
    return {
      px: w / 2 + isoX * viewScale,
      py: h / 2 + isoY * viewScale,
    }
  }

  // Draw RVE bounding box
  const corners = [
    { x: 0, y: 0, z: 0 },
    { x: ms.size_x_mm, y: 0, z: 0 },
    { x: ms.size_x_mm, y: ms.size_y_mm, z: 0 },
    { x: 0, y: ms.size_y_mm, z: 0 },
    { x: 0, y: 0, z: ms.size_z_mm },
    { x: ms.size_x_mm, y: 0, z: ms.size_z_mm },
    { x: ms.size_x_mm, y: ms.size_y_mm, z: ms.size_z_mm },
    { x: 0, y: ms.size_y_mm, z: ms.size_z_mm },
  ]

  const boxEdges = [
    [0, 1], [1, 2], [2, 3], [3, 0],
    [4, 5], [5, 6], [6, 7], [7, 4],
    [0, 4], [1, 5], [2, 6], [3, 7],
  ]

  ctx.strokeStyle = 'rgba(255, 255, 255, 0.15)'
  ctx.lineWidth = 1
  for (const [a, b] of boxEdges) {
    const p1 = project(corners[a].x, corners[a].y, corners[a].z)
    const p2 = project(corners[b].x, corners[b].y, corners[b].z)
    ctx.beginPath()
    ctx.moveTo(p1.px, p1.py)
    ctx.lineTo(p2.px, p2.py)
    ctx.stroke()
  }

  // Sort inclusions by depth for painter's algorithm
  const sortedInclusions = [...inclusions].sort((a, b) => {
    const depthA = a.cz
    const depthB = b.cz
    return depthA - depthB
  })

  // Draw inclusions
  for (const inc of sortedInclusions) {
    const p = project(inc.cx, inc.cy, inc.cz)
    const depth = inc.cz / (ms.size_z_mm + 1e-10)
    const brightness = 0.3 + 0.7 * (1 - depth)
    const radius = Math.max(inc.rx, inc.ry) * viewScale

    // Inclusion color: warm orange/amber
    const r = Math.round(255 * brightness)
    const g = Math.round(140 * brightness)
    const b = Math.round(50 * brightness)

    ctx.fillStyle = `rgba(${r}, ${g}, ${b}, 0.7)`
    ctx.strokeStyle = `rgba(${r}, ${g}, ${b}, 0.9)`
    ctx.lineWidth = 0.5

    if (ms.inclusion_shape === 'circle') {
      ctx.beginPath()
      ctx.arc(p.px, p.py, radius, 0, Math.PI * 2)
      ctx.fill()
      ctx.stroke()
    } else if (ms.inclusion_shape === 'ellipse') {
      ctx.beginPath()
      ctx.ellipse(p.px, p.py, radius, radius * (inc.ry / (inc.rx + 1e-10)), 0, 0, Math.PI * 2)
      ctx.fill()
      ctx.stroke()
    } else if (ms.inclusion_shape === 'polygon') {
      const sides = 6
      ctx.beginPath()
      for (let i = 0; i < sides; i++) {
        const a = (2 * Math.PI / sides) * i
        const px = p.px + radius * Math.cos(a)
        const py = p.py + radius * Math.sin(a)
        if (i === 0) ctx.moveTo(px, py)
        else ctx.lineTo(px, py)
      }
      ctx.closePath()
      ctx.fill()
      ctx.stroke()
    } else {
      // Irregular: random polygon
      const rng = mulberry32(Math.round(inc.cx * 1000 + inc.cy * 100))
      const sides = 5 + Math.floor(rng() * 4)
      ctx.beginPath()
      for (let i = 0; i < sides; i++) {
        const a = (2 * Math.PI / sides) * i
        const rVar = radius * (0.6 + rng() * 0.8)
        const px = p.px + rVar * Math.cos(a)
        const py = p.py + rVar * Math.sin(a)
        if (i === 0) ctx.moveTo(px, py)
        else ctx.lineTo(px, py)
      }
      ctx.closePath()
      ctx.fill()
      ctx.stroke()
    }
  }

  // Draw periodic BC indicators
  if (config.periodic_bc) {
    ctx.setLineDash([4, 4])
    ctx.strokeStyle = 'rgba(79, 140, 255, 0.5)'
    ctx.lineWidth = 1.5

    // Draw arrows on opposite faces
    const arrowPairs = [
      { a: project(0, ms.size_y_mm / 2, ms.size_z_mm / 2), b: project(ms.size_x_mm, ms.size_y_mm / 2, ms.size_z_mm / 2) },
      { a: project(ms.size_x_mm / 2, 0, ms.size_z_mm / 2), b: project(ms.size_x_mm / 2, ms.size_y_mm, ms.size_z_mm / 2) },
      { a: project(ms.size_x_mm / 2, ms.size_y_mm / 2, 0), b: project(ms.size_x_mm / 2, ms.size_y_mm / 2, ms.size_z_mm) },
    ]

    for (const pair of arrowPairs) {
      ctx.beginPath()
      ctx.moveTo(pair.a.px, pair.a.py)
      ctx.lineTo(pair.b.px, pair.b.py)
      ctx.stroke()
    }

    ctx.setLineDash([])
  }

  // Title
  ctx.fillStyle = '#E8E9EB'
  ctx.font = '12px sans-serif'
  ctx.textAlign = 'center'
  const typeLabel = microTypes.find(mt => mt.value === ms.type)?.label ?? ms.type
  ctx.fillText(`${typeLabel} RVE - ${inclusions.length} 包含体`, w / 2, 20)

  // Axis indicator
  const axLen = 40
  const axOrigin = { x: 50, y: h - 50 }
  const axX = project(ms.size_x_mm / 2 + axLen / viewScale, ms.size_y_mm / 2, 0)
  const axY = project(ms.size_x_mm / 2, ms.size_y_mm / 2 + axLen / viewScale, 0)
  const axZ = project(ms.size_x_mm / 2, ms.size_y_mm / 2, axLen / viewScale)

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

  // Legend
  ctx.fillStyle = 'rgba(255, 140, 50, 0.7)'
  ctx.fillRect(w - 140, h - 50, 12, 12)
  ctx.fillStyle = '#E8E9EB'
  ctx.font = '10px sans-serif'
  ctx.textAlign = 'left'
  ctx.fillText('包含体', w - 124, h - 40)

  ctx.fillStyle = 'rgba(79, 140, 255, 0.3)'
  ctx.fillRect(w - 140, h - 32, 12, 12)
  ctx.fillStyle = '#E8E9EB'
  ctx.fillText('基体', w - 124, h - 22)
}

async function generateRve() {
  generating.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 1500))
    results.value = generateMockResults()
    await nextTick()
    drawPreview()
  } catch (e) {
    console.error('RVE generation failed:', e)
  } finally {
    generating.value = false
  }
}

function exportMesh() {
  if (!results.value) return
  const data = JSON.stringify(results.value.mesh, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `rve_mesh_${config.micro_structure.type}.json`
  a.click()
  URL.revokeObjectURL(url)
}

// ============ Lifecycle ============

onMounted(() => {
  onMatrixPresetChange()
  onInclusionPresetChange()
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
