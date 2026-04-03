<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">跨尺度坐标映射引擎</h2>
        <p class="text-sm" style="color: var(--text-muted)">V1.8-002 | MD box ↔ Phase Field grid ↔ FE mesh 坐标变换</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="loadPreset" class="btn btn-ghost text-xs">加载预设</button>
        <button @click="resetAll" class="btn btn-ghost text-xs">重置</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Source Scale -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            源尺度
          </h4>
          <div class="grid grid-cols-2 gap-1">
            <button
              v-for="s in scaleOptions" :key="s.value"
              @click="sourceScale = s.value"
              class="px-2 py-2 rounded text-xs text-left transition border"
              :style="sourceScale === s.value
                ? 'background: var(--primary); border-color: var(--primary); color: white'
                : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
            >
              <div class="font-medium">{{ s.label }}</div>
              <div class="text-[10px] mt-0.5 opacity-80">{{ s.desc }}</div>
            </button>
          </div>
        </div>

        <!-- Step 2: Source Geometry -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            源几何参数
          </h4>

          <!-- MD Box params -->
          <div v-if="sourceScale === 'md'" class="space-y-2">
            <div class="grid grid-cols-3 gap-1">
              <div><label class="label">Origin X (Å)</label><input v-model.number="mdSource.origin.x" type="number" step="0.1" class="input w-full text-xs" /></div>
              <div><label class="label">Origin Y (Å)</label><input v-model.number="mdSource.origin.y" type="number" step="0.1" class="input w-full text-xs" /></div>
              <div><label class="label">Origin Z (Å)</label><input v-model.number="mdSource.origin.z" type="number" step="0.1" class="input w-full text-xs" /></div>
            </div>
            <div class="text-[10px] font-medium mt-2" style="color: var(--text-muted)">晶格向量</div>
            <div v-for="(vec, idx) in mdSource.lattice_vectors" :key="'slv-'+idx" class="grid grid-cols-3 gap-1">
              <div><label class="label">a{{ idx + 1 }}.x</label><input v-model.number="vec.x" type="number" step="0.1" class="input w-full text-xs" /></div>
              <div><label class="label">a{{ idx + 1 }}.y</label><input v-model.number="vec.y" type="number" step="0.1" class="input w-full text-xs" /></div>
              <div><label class="label">a{{ idx + 1 }}.z</label><input v-model.number="vec.z" type="number" step="0.1" class="input w-full text-xs" /></div>
            </div>
            <div class="flex gap-2">
              <label class="text-[10px] flex items-center gap-1" style="color: var(--text-secondary)">
                <input type="checkbox" v-model="mdSource.periodic[0]" /> P_x
              </label>
              <label class="text-[10px] flex items-center gap-1" style="color: var(--text-secondary)">
                <input type="checkbox" v-model="mdSource.periodic[1]" /> P_y
              </label>
              <label class="text-[10px] flex items-center gap-1" style="color: var(--text-secondary)">
                <input type="checkbox" v-model="mdSource.periodic[2]" /> P_z
              </label>
            </div>
            <div><label class="label">原子数</label><input v-model.number="mdSource.n_atoms" type="number" class="input w-full text-xs" /></div>
          </div>

          <!-- Phase Field Grid params -->
          <div v-if="sourceScale === 'phase_field'" class="space-y-2">
            <div class="grid grid-cols-3 gap-1">
              <div><label class="label">Origin X</label><input v-model.number="pfSource.origin.x" type="number" step="0.1" class="input w-full text-xs" /></div>
              <div><label class="label">Origin Y</label><input v-model.number="pfSource.origin.y" type="number" step="0.1" class="input w-full text-xs" /></div>
              <div><label class="label">Origin Z</label><input v-model.number="pfSource.origin.z" type="number" step="0.1" class="input w-full text-xs" /></div>
            </div>
            <div class="grid grid-cols-3 gap-1">
              <div><label class="label">Nx</label><input v-model.number="pfSource.dimensions.nx" type="number" class="input w-full text-xs" /></div>
              <div><label class="label">Ny</label><input v-model.number="pfSource.dimensions.ny" type="number" class="input w-full text-xs" /></div>
              <div><label class="label">Nz</label><input v-model.number="pfSource.dimensions.nz" type="number" class="input w-full text-xs" /></div>
            </div>
            <div class="grid grid-cols-3 gap-1">
              <div><label class="label">dx (Å)</label><input v-model.number="pfSource.spacing.dx" type="number" step="0.01" class="input w-full text-xs" /></div>
              <div><label class="label">dy (Å)</label><input v-model.number="pfSource.spacing.dy" type="number" step="0.01" class="input w-full text-xs" /></div>
              <div><label class="label">dz (Å)</label><input v-model.number="pfSource.spacing.dz" type="number" step="0.01" class="input w-full text-xs" /></div>
            </div>
          </div>

          <!-- FE Mesh params -->
          <div v-if="sourceScale === 'fe'" class="space-y-2">
            <div class="text-[10px] font-medium" style="color: var(--text-muted)">Bounding Box Min</div>
            <div class="grid grid-cols-3 gap-1">
              <div><label class="label">Min X</label><input v-model.number="feSource.bounding_box.min.x" type="number" step="0.1" class="input w-full text-xs" /></div>
              <div><label class="label">Min Y</label><input v-model.number="feSource.bounding_box.min.y" type="number" step="0.1" class="input w-full text-xs" /></div>
              <div><label class="label">Min Z</label><input v-model.number="feSource.bounding_box.min.z" type="number" step="0.1" class="input w-full text-xs" /></div>
            </div>
            <div class="text-[10px] font-medium" style="color: var(--text-muted)">Bounding Box Max</div>
            <div class="grid grid-cols-3 gap-1">
              <div><label class="label">Max X</label><input v-model.number="feSource.bounding_box.max.x" type="number" step="0.1" class="input w-full text-xs" /></div>
              <div><label class="label">Max Y</label><input v-model.number="feSource.bounding_box.max.y" type="number" step="0.1" class="input w-full text-xs" /></div>
              <div><label class="label">Max Z</label><input v-model.number="feSource.bounding_box.max.z" type="number" step="0.1" class="input w-full text-xs" /></div>
            </div>
            <div><label class="label">单元数</label><input v-model.number="feSource.n_elements" type="number" class="input w-full text-xs" /></div>
            <div><label class="label">单元类型</label>
              <select v-model="feSource.element_type" class="input w-full text-xs">
                <option value="hex8">Hex8</option>
                <option value="tet4">Tet4</option>
                <option value="hex20">Hex20</option>
              </select>
            </div>
          </div>

          <!-- DFT placeholder -->
          <div v-if="sourceScale === 'dft'" class="text-xs p-3 rounded" style="background: var(--bg-elevated); color: var(--text-muted)">
            DFT 尺度暂不支持作为映射源，请选择 MD / Phase Field / FE
          </div>
        </div>

        <!-- Step 3: Target Scale -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            目标尺度
          </h4>
          <div class="grid grid-cols-2 gap-1">
            <button
              v-for="s in scaleOptions" :key="s.value"
              @click="targetScale = s.value"
              class="px-2 py-2 rounded text-xs text-left transition border"
              :style="targetScale === s.value
                ? 'background: var(--accent-green); border-color: var(--accent-green); color: white'
                : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
            >
              <div class="font-medium">{{ s.label }}</div>
              <div class="text-[10px] mt-0.5 opacity-80">{{ s.desc }}</div>
            </button>
          </div>
        </div>

        <!-- Target Geometry (simplified) -->
        <div v-if="targetScale === 'phase_field'" class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--accent-green)">3b</span>
            目标网格参数
          </h4>
          <div class="space-y-2">
            <div class="grid grid-cols-3 gap-1">
              <div><label class="label">Nx</label><input v-model.number="pfTarget.dimensions.nx" type="number" class="input w-full text-xs" /></div>
              <div><label class="label">Ny</label><input v-model.number="pfTarget.dimensions.ny" type="number" class="input w-full text-xs" /></div>
              <div><label class="label">Nz</label><input v-model.number="pfTarget.dimensions.nz" type="number" class="input w-full text-xs" /></div>
            </div>
            <div class="grid grid-cols-3 gap-1">
              <div><label class="label">dx (Å)</label><input v-model.number="pfTarget.spacing.dx" type="number" step="0.01" class="input w-full text-xs" /></div>
              <div><label class="label">dy (Å)</label><input v-model.number="pfTarget.spacing.dy" type="number" step="0.01" class="input w-full text-xs" /></div>
              <div><label class="label">dz (Å)</label><input v-model.number="pfTarget.spacing.dz" type="number" step="0.01" class="input w-full text-xs" /></div>
            </div>
          </div>
        </div>

        <!-- Step 4: Options -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            映射选项
          </h4>
          <div class="space-y-2">
            <label class="flex items-center gap-2 text-xs" style="color: var(--text-secondary)">
              <input type="checkbox" v-model="mappingOptions.align_crystal_orientation" />
              对齐晶体取向
            </label>
            <label class="flex items-center gap-2 text-xs" style="color: var(--text-secondary)">
              <input type="checkbox" v-model="mappingOptions.align_origin" />
              对齐原点
            </label>
            <div>
              <label class="label">容差 (Å)</label>
              <input v-model.number="mappingOptions.tolerance_A" type="number" step="0.01" min="0.001" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Step 5: Execute -->
        <div class="panel-section">
          <button @click="createMapping" class="btn btn-primary text-xs w-full" :disabled="isMapping">
            {{ isMapping ? '映射中...' : '创建映射' }}
          </button>
          <div v-if="mappingResult" class="mt-3 p-3 rounded border space-y-1" style="background: var(--bg-elevated); border-color: var(--border-default)">
            <div class="flex justify-between text-xs">
              <span style="color: var(--text-muted)">取向误差</span>
              <span class="font-mono font-bold" style="color: var(--accent-green)">{{ mappingResult.alignment_error_deg.toFixed(4) }}°</span>
            </div>
            <div class="flex justify-between text-xs">
              <span style="color: var(--text-muted)">原点误差</span>
              <span class="font-mono font-bold" style="color: var(--accent-green)">{{ mappingResult.origin_error_A.toFixed(4) }} Å</span>
            </div>
            <div class="flex justify-between text-xs">
              <span style="color: var(--text-muted)">映射点数</span>
              <span class="font-mono font-bold" style="color: var(--primary)">{{ mappingResult.mapped_points_count.toLocaleString() }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization & Results -->
      <div class="flex-1 overflow-y-auto p-6 space-y-6">

        <!-- Coordinate System Visualization -->
        <div class="p-4 rounded-lg border" style="background: var(--bg-surface); border-color: var(--border-subtle)">
          <h3 class="text-sm font-medium mb-4" style="color: var(--text-primary)">坐标系统可视化</h3>
          <svg viewBox="0 0 560 340" class="w-full" style="max-height: 340px">
            <!-- Background grid -->
            <defs>
              <pattern id="grid" width="20" height="20" patternUnits="userSpaceOnUse">
                <path d="M 20 0 L 0 0 0 20" fill="none" stroke="var(--border-subtle)" stroke-width="0.5"/>
              </pattern>
              <marker id="arrowBlue" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
                <polygon points="0 0, 8 3, 0 6" fill="#4F8EF7"/>
              </marker>
              <marker id="arrowGreen" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
                <polygon points="0 0, 8 3, 0 6" fill="#4FF7B0"/>
              </marker>
              <marker id="arrowOrange" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
                <polygon points="0 0, 8 3, 0 6" fill="#F7A84F"/>
              </marker>
            </defs>
            <rect width="560" height="340" fill="url(#grid)" rx="8"/>

            <!-- Source coordinate system (blue) -->
            <g transform="translate(120, 200)">
              <text x="0" y="-60" fill="#4F8EF7" font-size="11" font-weight="bold">源: {{ sourceScaleLabel }}</text>
              <!-- Origin dot -->
              <circle cx="0" cy="0" r="4" fill="#4F8EF7"/>
              <!-- Axes -->
              <line x1="0" y1="0" x2="80" y2="0" stroke="#4F8EF7" stroke-width="2" marker-end="url(#arrowBlue)"/>
              <line x1="0" y1="0" x2="0" y2="-80" stroke="#4F8EF7" stroke-width="2" marker-end="url(#arrowBlue)"/>
              <line x1="0" y1="0" x2="-40" y2="40" stroke="#4F8EF7" stroke-width="2" marker-end="url(#arrowBlue)"/>
              <text x="85" y="4" fill="#4F8EF7" font-size="10">x</text>
              <text x="4" y="-85" fill="#4F8EF7" font-size="10">y</text>
              <text x="-55" y="48" fill="#4F8EF7" font-size="10">z</text>
              <!-- Box outline for MD -->
              <rect v-if="sourceScale === 'md'" x="-30" y="-50" width="60" height="50" fill="none" stroke="#4F8EF7" stroke-width="1" stroke-dasharray="4,2" opacity="0.5" rx="2"/>
              <text v-if="sourceScale === 'md'" x="-25" y="-35" fill="#4F8EF7" font-size="8" opacity="0.7">MD Box</text>
            </g>

            <!-- Transformation arrow -->
            <g transform="translate(280, 170)">
              <line x1="0" y1="0" x2="0" y2="0" stroke="var(--text-muted)" stroke-width="1"/>
              <path d="M -40 0 C -20 -20, 20 -20, 40 0" fill="none" stroke="#F7A84F" stroke-width="2" marker-end="url(#arrowOrange)"/>
              <text x="0" y="-18" fill="#F7A84F" font-size="10" text-anchor="middle" font-weight="bold">T</text>
            </g>

            <!-- Target coordinate system (green) -->
            <g transform="translate(440, 200)">
              <text x="0" y="-60" fill="#4FF7B0" font-size="11" font-weight="bold">目标: {{ targetScaleLabel }}</text>
              <!-- Origin dot -->
              <circle cx="0" cy="0" r="4" fill="#4FF7B0"/>
              <!-- Axes (slightly rotated to show transformation) -->
              <line x1="0" y1="0" x2="75" y2="10" stroke="#4FF7B0" stroke-width="2" marker-end="url(#arrowGreen)"/>
              <line x1="0" y1="0" x2="-10" y2="-78" stroke="#4FF7B0" stroke-width="2" marker-end="url(#arrowGreen)"/>
              <line x1="0" y1="0" x2="-45" y2="35" stroke="#4FF7B0" stroke-width="2" marker-end="url(#arrowGreen)"/>
              <text x="80" y="16" fill="#4FF7B0" font-size="10">x'</text>
              <text x="-18" y="-83" fill="#4FF7B0" font-size="10">y'</text>
              <text x="-58" y="42" fill="#4FF7B0" font-size="10">z'</text>
              <!-- Grid outline for PF -->
              <rect v-if="targetScale === 'phase_field'" x="-35" y="-55" width="70" height="55" fill="none" stroke="#4FF7B0" stroke-width="1" stroke-dasharray="2,2" opacity="0.5" rx="2"/>
              <text v-if="targetScale === 'phase_field'" x="-30" y="-40" fill="#4FF7B0" font-size="8" opacity="0.7">PF Grid</text>
            </g>

            <!-- Legend -->
            <g transform="translate(20, 310)">
              <circle cx="0" cy="0" r="4" fill="#4F8EF7"/>
              <text x="8" y="4" fill="var(--text-secondary)" font-size="10">源坐标系</text>
              <circle cx="90" cy="0" r="4" fill="#4FF7B0"/>
              <text x="98" y="4" fill="var(--text-secondary)" font-size="10">目标坐标系</text>
              <circle cx="190" cy="0" r="4" fill="#F7A84F"/>
              <text x="198" y="4" fill="var(--text-secondary)" font-size="10">变换 T</text>
            </g>
          </svg>
        </div>

        <!-- Transformation Matrix -->
        <div v-if="mappingResult" class="p-4 rounded-lg border" style="background: var(--bg-surface); border-color: var(--border-subtle)">
          <h3 class="text-sm font-medium mb-3" style="color: var(--text-primary)">变换矩阵 (4×4)</h3>
          <div class="overflow-x-auto">
            <table class="text-xs font-mono">
              <tr v-for="(row, ri) in mappingResult.transformation_matrix" :key="'mat-'+ri">
                <td
                  v-for="(val, ci) in row" :key="'mat-'+ri+'-'+ci"
                  class="px-3 py-1.5 text-center border"
                  style="border-color: var(--border-subtle); color: var(--text-secondary)"
                  :style="ri === 3 && ci === 3 ? 'color: var(--primary); font-weight: bold' : ''"
                >{{ val.toFixed(6) }}</td>
              </tr>
            </table>
          </div>
        </div>

        <!-- Alignment Validation & Warnings -->
        <div v-if="mappingResult" class="grid grid-cols-2 gap-4">
          <!-- Validation Status -->
          <div class="p-4 rounded-lg border" style="background: var(--bg-surface); border-color: var(--border-subtle)">
            <h3 class="text-sm font-medium mb-3" style="color: var(--text-primary)">对齐验证</h3>
            <div class="space-y-2">
              <div class="flex items-center gap-2">
                <span class="w-2 h-2 rounded-full" :style="{ background: mappingResult.alignment_error_deg < 1 ? 'var(--accent-green)' : 'var(--accent-yellow)' }"></span>
                <span class="text-xs" style="color: var(--text-secondary)">
                  取向: {{ mappingResult.alignment_error_deg < 1 ? '通过' : '需检查' }}
                  ({{ mappingResult.alignment_error_deg.toFixed(4) }}°)
                </span>
              </div>
              <div class="flex items-center gap-2">
                <span class="w-2 h-2 rounded-full" :style="{ background: mappingResult.origin_error_A < 0.5 ? 'var(--accent-green)' : 'var(--accent-yellow)' }"></span>
                <span class="text-xs" style="color: var(--text-secondary)">
                  原点: {{ mappingResult.origin_error_A < 0.5 ? '通过' : '需检查' }}
                  ({{ mappingResult.origin_error_A.toFixed(4) }} Å)
                </span>
              </div>
              <div class="flex items-center gap-2">
                <span class="w-2 h-2 rounded-full" style="background: var(--accent-green)"></span>
                <span class="text-xs" style="color: var(--text-secondary)">
                  映射点: {{ mappingResult.mapped_points_count.toLocaleString() }}
                </span>
              </div>
            </div>
          </div>

          <!-- Warnings -->
          <div class="p-4 rounded-lg border" style="background: var(--bg-surface); border-color: var(--border-subtle)">
            <h3 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
              警告
              <span v-if="mappingResult.warnings.length > 0" class="ml-1 px-1.5 py-0.5 rounded text-[10px]" style="background: var(--accent-yellow); color: #000">
                {{ mappingResult.warnings.length }}
              </span>
            </h3>
            <div v-if="mappingResult.warnings.length === 0" class="text-xs" style="color: var(--text-muted)">无警告</div>
            <div v-else class="space-y-1">
              <div v-for="(w, wi) in mappingResult.warnings" :key="'warn-'+wi"
                class="text-xs px-2 py-1 rounded" style="background: var(--bg-elevated); color: var(--accent-yellow)"
              >{{ w }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type { ScaleLevel, MappingConfig, MappingResult } from '../api/coordinateMapping'

// ============ Types ============

interface ScaleOption {
  value: ScaleLevel
  label: string
  desc: string
}

// ============ Scale Options ============

const scaleOptions: ScaleOption[] = [
  { value: 'dft', label: 'DFT', desc: '第一性原理' },
  { value: 'md', label: 'MD', desc: '分子动力学' },
  { value: 'phase_field', label: 'Phase Field', desc: '相场模型' },
  { value: 'fe', label: 'FE', desc: '有限元' },
]

// ============ State ============

const sourceScale = ref<ScaleLevel>('md')
const targetScale = ref<ScaleLevel>('phase_field')
const isMapping = ref(false)
const mappingResult = ref<MappingResult | null>(null)

const mdSource = reactive({
  origin: { x: 0, y: 0, z: 0 },
  lattice_vectors: [
    { x: 10.0, y: 0, z: 0 },
    { x: 0, y: 10.0, z: 0 },
    { x: 0, y: 0, z: 10.0 },
  ],
  periodic: [true, true, true] as [boolean, boolean, boolean],
  n_atoms: 4000,
})

const pfSource = reactive({
  origin: { x: 0, y: 0, z: 0 },
  dimensions: { nx: 50, ny: 50, nz: 50 },
  spacing: { dx: 0.2, dy: 0.2, dz: 0.2 },
})

const feSource = reactive({
  bounding_box: {
    min: { x: 0, y: 0, z: 0 },
    max: { x: 100, y: 100, z: 100 },
  },
  n_elements: 1000000,
  element_type: 'hex8',
})

const pfTarget = reactive({
  origin: { x: 0, y: 0, z: 0 },
  dimensions: { nx: 100, ny: 100, nz: 100 },
  spacing: { dx: 0.1, dy: 0.1, dz: 0.1 },
})

const mappingOptions = reactive({
  align_crystal_orientation: true,
  align_origin: true,
  tolerance_A: 0.01,
})

// ============ Computed ============

const sourceScaleLabel = computed(() => {
  const opt = scaleOptions.find(s => s.value === sourceScale.value)
  return opt ? opt.label : ''
})

const targetScaleLabel = computed(() => {
  const opt = scaleOptions.find(s => s.value === targetScale.value)
  return opt ? opt.label : ''
})

// ============ Methods ============

function createMapping() {
  isMapping.value = true

  // Simulate async mapping with mock result
  setTimeout(() => {
    const sourceLabel = sourceScaleLabel.value
    const targetLabel = targetScaleLabel.value

    // Generate a mock 4x4 transformation matrix
    const identityMatrix: number[][] = [
      [1, 0, 0, 0],
      [0, 1, 0, 0],
      [0, 0, 1, 0],
      [0, 0, 0, 1],
    ]

    // Add slight rotation to make it realistic
    const angle = 0.0035 // ~0.2 degrees
    const cos_a = Math.cos(angle)
    const sin_a = Math.sin(angle)
    const matrix: number[][] = [
      [cos_a, -sin_a, 0, 0.001],
      [sin_a, cos_a, 0, -0.0005],
      [0, 0, 1, 0.0002],
      [0, 0, 0, 1],
    ]

    const warnings: string[] = []
    if (!mappingOptions.align_crystal_orientation) {
      warnings.push('晶体取向未对齐，映射可能存在系统误差')
    }
    if (mappingOptions.tolerance_A > 0.1) {
      warnings.push('容差较大 (>0.1 Å)，边界点可能被跳过')
    }

    mappingResult.value = {
      success: true,
      transformation_matrix: matrix,
      alignment_error_deg: 0.2005,
      origin_error_A: 0.0012,
      mapped_points_count: 1000000,
      warnings,
    }

    isMapping.value = false
  }, 800)
}

function loadPreset() {
  // Load MD → Phase Field preset (FCC Al)
  sourceScale.value = 'md'
  targetScale.value = 'phase_field'

  mdSource.origin = { x: 0, y: 0, z: 0 }
  mdSource.lattice_vectors = [
    { x: 10.0, y: 0, z: 0 },
    { x: 0, y: 10.0, z: 0 },
    { x: 0, y: 0, z: 10.0 },
  ]
  mdSource.periodic = [true, true, true]
  mdSource.n_atoms = 4000

  pfTarget.origin = { x: 0, y: 0, z: 0 }
  pfTarget.dimensions = { nx: 100, ny: 100, nz: 100 }
  pfTarget.spacing = { dx: 0.1, dy: 0.1, dz: 0.1 }

  mappingOptions.align_crystal_orientation = true
  mappingOptions.align_origin = true
  mappingOptions.tolerance_A = 0.01

  mappingResult.value = null
}

function resetAll() {
  sourceScale.value = 'md'
  targetScale.value = 'phase_field'
  mappingResult.value = null
  isMapping.value = false

  mdSource.origin = { x: 0, y: 0, z: 0 }
  mdSource.lattice_vectors = [
    { x: 10.0, y: 0, z: 0 },
    { x: 0, y: 10.0, z: 0 },
    { x: 0, y: 0, z: 10.0 },
  ]
  mdSource.periodic = [true, true, true]
  mdSource.n_atoms = 4000

  mappingOptions.align_crystal_orientation = true
  mappingOptions.align_origin = true
  mappingOptions.tolerance_A = 0.01
}

// ============ Lifecycle ============

onMounted(() => {
  nextTick(() => {
    // Auto-load preset for demo
    loadPreset()
  })
})
</script>
