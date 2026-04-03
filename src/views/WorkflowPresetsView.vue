<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">典型工作流模板</h2>
        <p class="text-sm" style="color: var(--text-muted)">V2.0-005 | 蠕变/凝固/析出/断裂/扩散 五种典型材料计算工作流</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="showCreateFromGraph = !showCreateFromGraph" class="btn btn-ghost text-xs">从图创建模板</button>
        <button @click="resetFilters" class="btn btn-ghost text-xs">重置筛选</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Category & Difficulty Filter -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            筛选模板
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">类别</label>
              <div class="flex flex-wrap gap-1">
                <button
                  v-for="cat in categoryOptions" :key="cat.value"
                  @click="selectedCategory = cat.value"
                  class="px-2 py-1 rounded text-[10px] transition border"
                  :style="selectedCategory === cat.value
                    ? 'background: var(--primary); border-color: var(--primary); color: white'
                    : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
                >{{ cat.label }}</button>
              </div>
            </div>
            <div>
              <label class="label">难度</label>
              <div class="flex gap-1">
                <button
                  v-for="diff in difficultyOptions" :key="diff.value"
                  @click="selectedDifficulty = diff.value"
                  class="px-2 py-1 rounded text-[10px] transition border"
                  :style="selectedDifficulty === diff.value
                    ? 'background: var(--primary); border-color: var(--primary); color: white'
                    : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
                >{{ diff.label }}</button>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 2: Preset Cards -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            模板列表 ({{ filteredPresets.length }})
          </h4>
          <div class="space-y-2">
            <div
              v-for="preset in filteredPresets" :key="preset.id"
              class="rounded p-2 border cursor-pointer transition"
              :style="selectedPreset && selectedPreset.id === preset.id
                ? 'background: var(--bg-elevated); border-color: var(--primary)'
                : 'background: var(--bg-elevated); border-color: var(--border-default)'"
              @click="selectedPreset = preset"
            >
              <div class="flex items-center justify-between mb-1">
                <span class="text-xs font-medium" style="color: var(--text-primary)">{{ preset.name_zh }}</span>
                <span class="text-[10px] px-1.5 py-0.5 rounded" :style="getCategoryBadgeStyle(preset.category)">{{ getCategoryLabel(preset.category) }}</span>
              </div>
              <div class="flex items-center gap-2 text-[10px]" style="color: var(--text-muted)">
                <span :style="{ color: getDifficultyColor(preset.difficulty) }">{{ getDifficultyLabel(preset.difficulty) }}</span>
                <span>|</span>
                <span>{{ preset.estimated_time_min }} min</span>
                <span>|</span>
                <span>{{ preset.steps.length }} 步</span>
              </div>
              <div class="text-[10px] mt-1" style="color: var(--text-secondary)">
                材料: {{ preset.material_examples.slice(0, 2).join(', ') }}
              </div>
              <button @click.stop="applyPresetDirect(preset)" class="btn btn-primary text-xs w-full mt-2">应用</button>
            </div>
          </div>
        </div>

        <!-- Step 3: Selected Preset Detail -->
        <div v-if="selectedPreset" class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            模板详情: {{ selectedPreset.name_zh }}
          </h4>
          <div class="space-y-2">
            <p class="text-[10px]" style="color: var(--text-secondary)">{{ selectedPreset.description }}</p>
            <div class="text-[10px] font-medium" style="color: var(--text-muted)">工作流步骤:</div>
            <div v-for="(step, idx) in selectedPreset.steps" :key="idx" class="rounded p-2 border" style="background: var(--bg-base); border-color: var(--border-default)">
              <div class="flex items-center gap-1 mb-1">
                <span class="text-[10px] font-medium" style="color: var(--text-primary)">{{ step.name }}</span>
                <span class="text-[10px] px-1 py-0.5 rounded" style="background: var(--primary); color: white; opacity: 0.8">{{ step.scale }}</span>
              </div>
              <div class="text-[10px]" style="color: var(--text-muted)">{{ step.description }}</div>
              <div class="text-[10px] mt-1" style="color: var(--text-secondary)">
                参数: {{ step.key_parameters.map((p: PresetKeyParameter) => p.name).join(', ') }}
              </div>
              <div class="text-[10px]" style="color: var(--text-secondary)">
                输出: {{ step.expected_output.map((o: PresetExpectedOutput) => o.quantity + ' (' + o.unit + ')').join(', ') }}
              </div>
            </div>
          </div>
        </div>

        <!-- Step 4: Customization -->
        <div v-if="selectedPreset" class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            自定义配置
          </h4>
          <div class="space-y-2">
            <div class="text-[10px] font-medium" style="color: var(--text-muted)">参数覆盖:</div>
            <div v-for="step in selectedPreset.steps" :key="'ov-'+step.name" class="space-y-1">
              <div v-for="param in step.key_parameters" :key="'ovp-'+param.name" class="flex items-center gap-1">
                <label class="text-[10px] w-24 truncate" style="color: var(--text-secondary)">{{ param.name }}</label>
                <input v-model="customOverrides[param.name]" class="input w-full text-xs" :placeholder="String(param.default_value)" />
              </div>
            </div>
            <div class="text-[10px] font-medium mt-2" style="color: var(--text-muted)">跳过步骤:</div>
            <div v-for="step in selectedPreset.steps" :key="'skip-'+step.name" class="flex items-center gap-1">
              <input type="checkbox" v-model="skipSteps" :value="step.name" />
              <span class="text-[10px]" style="color: var(--text-secondary)">{{ step.name }}</span>
            </div>
            <button @click="applyCustomized" class="btn btn-primary text-xs w-full">应用模板</button>
          </div>
        </div>

        <!-- Step 5: Create from Graph -->
        <div v-if="showCreateFromGraph" class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            从图创建模板
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">工作流图 ID</label>
              <input v-model="createFromGraph.graphId" class="input w-full text-xs" placeholder="graph_xxx" />
            </div>
            <div>
              <label class="label">模板名称</label>
              <input v-model="createFromGraph.name" class="input w-full text-xs" placeholder="自定义模板" />
            </div>
            <div>
              <label class="label">类别</label>
              <select v-model="createFromGraph.category" class="input w-full text-xs">
                <option value="creep">蠕变</option>
                <option value="solidification">凝固</option>
                <option value="precipitation">析出</option>
                <option value="fracture">断裂</option>
                <option value="diffusion">扩散</option>
              </select>
            </div>
            <button @click="createPresetFromGraphAction" class="btn btn-primary text-xs w-full">创建模板</button>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4">

        <!-- Top: Preset Workflow Diagram -->
        <div class="rounded border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">工作流模板示意图</h4>
          <svg viewBox="0 0 800 160" class="w-full" style="max-height: 160px">
            <defs>
              <marker id="arrowWf" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
                <polygon points="0 0, 8 3, 0 6" fill="var(--primary)" />
              </marker>
            </defs>
            <!-- 5 preset workflow patterns -->
            <g v-for="(preset, pidx) in presets" :key="preset.id">
              <text :x="80" :y="15 + pidx * 30" fill="var(--text-secondary)" font-size="10" font-weight="bold">{{ preset.name_zh }}</text>
              <g v-for="(step, sidx) in preset.steps.slice(0, 5)" :key="sidx">
                <rect :x="140 + sidx * 120" :y="4 + pidx * 30" width="100" height="20" rx="4"
                  :fill="getScaleColor(step.scale)" opacity="0.8" />
                <text :x="190 + sidx * 120" :y="18 + pidx * 30" text-anchor="middle" fill="white" font-size="8">{{ step.name }}</text>
                <line v-if="sidx < preset.steps.slice(0, 5).length - 1"
                  :x1="240 + sidx * 120" :y1="14 + pidx * 30"
                  :x2="140 + (sidx + 1) * 120" :y2="14 + pidx * 30"
                  stroke="var(--primary)" stroke-width="1" marker-end="url(#arrowWf)" />
              </g>
            </g>
          </svg>
        </div>

        <!-- Middle: Preset Comparison Table -->
        <div class="rounded border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">模板对比表</h4>
          <table class="w-full text-xs">
            <thead>
              <tr style="color: var(--text-muted)">
                <th class="text-left py-2 px-2">模板</th>
                <th class="text-left py-2 px-2">类别</th>
                <th class="text-center py-2 px-2">步骤数</th>
                <th class="text-center py-2 px-2">难度</th>
                <th class="text-center py-2 px-2">预估时间</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="preset in presets" :key="preset.id" style="border-top: 1px solid var(--border-subtle)">
                <td class="py-2 px-2 font-medium" style="color: var(--text-primary)">{{ preset.name_zh }}</td>
                <td class="py-2 px-2">
                  <span class="text-[10px] px-1.5 py-0.5 rounded" :style="getCategoryBadgeStyle(preset.category)">{{ getCategoryLabel(preset.category) }}</span>
                </td>
                <td class="py-2 px-2 text-center" style="color: var(--text-secondary)">{{ preset.steps.length }}</td>
                <td class="py-2 px-2 text-center">
                  <span :style="{ color: getDifficultyColor(preset.difficulty) }">{{ getDifficultyLabel(preset.difficulty) }}</span>
                </td>
                <td class="py-2 px-2 text-center" style="color: var(--text-secondary)">{{ preset.estimated_time_min }} min</td>
              </tr>
            </tbody>
          </table>
        </div>

        <!-- Bottom: Material Examples per Preset -->
        <div class="rounded border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">适用材料体系</h4>
          <div class="grid grid-cols-1 gap-2">
            <div v-for="preset in presets" :key="'mat-'+preset.id" class="rounded p-2 border" style="background: var(--bg-elevated); border-color: var(--border-default)">
              <div class="flex items-center gap-2 mb-1">
                <span class="text-xs font-medium" style="color: var(--text-primary)">{{ preset.name_zh }}</span>
                <span class="text-[10px] px-1.5 py-0.5 rounded" :style="getCategoryBadgeStyle(preset.category)">{{ getCategoryLabel(preset.category) }}</span>
              </div>
              <div class="flex flex-wrap gap-1">
                <span v-for="mat in preset.material_examples" :key="mat"
                  class="text-[10px] px-2 py-0.5 rounded" style="background: var(--bg-base); color: var(--text-secondary); border: 1px solid var(--border-default)">
                  {{ mat }}
                </span>
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
import type { PresetCategory, PresetStep, WorkflowPreset, PresetCustomization, PresetKeyParameter, PresetExpectedOutput } from '../api/workflowPresets'

// ============ Types & Constants ============

type PresetDifficulty = 'beginner' | 'intermediate' | 'advanced'

const categoryOptions = [
  { label: '全部', value: '' },
  { label: '蠕变', value: 'creep' },
  { label: '凝固', value: 'solidification' },
  { label: '析出', value: 'precipitation' },
  { label: '断裂', value: 'fracture' },
  { label: '扩散', value: 'diffusion' }
]

const difficultyOptions = [
  { label: '全部', value: '' },
  { label: '初级', value: 'beginner' },
  { label: '中级', value: 'intermediate' },
  { label: '高级', value: 'advanced' }
]

// ============ State ============

const selectedCategory = ref('')
const selectedDifficulty = ref('')
const selectedPreset = ref<WorkflowPreset | null>(null)
const showCreateFromGraph = ref(false)
const customOverrides = reactive<Record<string, string>>({})
const skipSteps = ref<string[]>([])

const createFromGraph = reactive({
  graphId: '',
  name: '',
  category: 'creep' as PresetCategory
})

// ============ Mock Data ============

const presets = ref<WorkflowPreset[]>([])

function generateMockPresets(): WorkflowPreset[] {
  return [
    {
      id: 'preset-creep-001',
      name: 'Ni-based Superalloy Creep',
      name_zh: '镍基高温合金蠕变分析',
      category: 'creep' as PresetCategory,
      description: '从DFT弹性常数出发，经MD原子尺度蠕变模拟，到相场微观组织演化，最终FE宏观蠕变寿命预测的完整多尺度蠕变工作流。',
      material_examples: ['Ni-based superalloy', 'Co-based superalloy', 'TiAl alloy'],
      steps: [
        { scale: 'DFT', name: '弹性常数计算', description: '计算C11, C12, C44弹性常数', key_parameters: [{ name: 'kpoints', default_value: '12x12x12', description: 'k点网格密度' }, { name: 'ecut', default_value: 520, description: '截断能(eV)' }], expected_output: [{ quantity: 'C11', unit: 'GPa' }, { quantity: 'C12', unit: 'GPa' }] },
        { scale: 'MD', name: 'EAM势拟合', description: '基于DFT数据拟合EAM势函数', key_parameters: [{ name: 'potential_type', default_value: 'EAM', description: '势函数类型' }, { name: 'fitting_method', default_value: 'force-matching', description: '拟合方法' }], expected_output: [{ quantity: 'eam_params', unit: '-' }] },
        { scale: 'MD', name: '高温蠕变模拟', description: '不同温度应力下蠕变模拟', key_parameters: [{ name: 'temperature', default_value: 1200, description: '温度(K)' }, { name: 'stress_MPa', default_value: 300, description: '施加应力(MPa)' }], expected_output: [{ quantity: 'creep_rate', unit: '1/s' }] },
        { scale: 'Phase-Field', name: 'gamma/gamma\'演化', description: '相场模拟微观组织演化', key_parameters: [{ name: 'grid_size', default_value: 256, description: '网格尺寸' }, { name: 'mobility', default_value: 1e-10, description: '迁移率' }], expected_output: [{ quantity: 'rafting_time', unit: 'h' }] },
        { scale: 'FE', name: '宏观蠕变寿命', description: '有限元蠕变寿命预测', key_parameters: [{ name: 'element_type', default_value: 'C3D8R', description: '单元类型' }, { name: 'norton_n', default_value: 5.0, description: 'Norton指数' }], expected_output: [{ quantity: 'rupture_time', unit: 'h' }] }
      ],
      estimated_time_min: 480,
      difficulty: 'advanced' as PresetDifficulty
    },
    {
      id: 'preset-solidification-001',
      name: 'Al Alloy Solidification',
      name_zh: '铝合金凝固过程模拟',
      category: 'solidification' as PresetCategory,
      description: '从DFT热力学数据到相场枝晶生长模拟，再到FE宏观铸造应力分析的多尺度凝固工作流。',
      material_examples: ['Al-Si alloy', 'Al-Cu alloy', 'Al-Mg-Si alloy'],
      steps: [
        { scale: 'DFT', name: '热力学性质', description: '计算形成能、混合焓', key_parameters: [{ name: 'supercell', default_value: '2x2x2', description: '超胞大小' }], expected_output: [{ quantity: 'mixing_enthalpy', unit: 'eV/atom' }] },
        { scale: 'MD', name: '液态结构', description: '液态铝合金分子动力学', key_parameters: [{ name: 'temperature', default_value: 1000, description: '温度(K)' }], expected_output: [{ quantity: 'pair_distribution', unit: '-' }] },
        { scale: 'Phase-Field', name: '枝晶生长', description: '相场枝晶生长模拟', key_parameters: [{ name: 'undercooling', default_value: 10, description: '过冷度(K)' }], expected_output: [{ quantity: 'dendrite_arm_spacing', unit: 'um' }] },
        { scale: 'FE', name: '铸造应力', description: '宏观铸造残余应力', key_parameters: [{ name: 'casting_speed', default_value: 0.01, description: '铸造速度(m/s)' }], expected_output: [{ quantity: 'residual_stress', unit: 'MPa' }] }
      ],
      estimated_time_min: 360,
      difficulty: 'intermediate' as PresetDifficulty
    },
    {
      id: 'preset-precipitation-001',
      name: 'Al Alloy Precipitation Hardening',
      name_zh: '铝合金析出强化模拟',
      category: 'precipitation' as PresetCategory,
      description: '从DFT析出相稳定性分析到相场析出动力学模拟，再到FE宏观力学性能预测的析出强化工作流。',
      material_examples: ['Al-Cu alloy', 'Al-Zn-Mg-Cu', 'Ni-based superalloy'],
      steps: [
        { scale: 'DFT', name: '析出相稳定性', description: '计算析出相形成能', key_parameters: [{ name: 'phase', default_value: 'theta', description: '析出相类型' }], expected_output: [{ quantity: 'formation_energy', unit: 'eV/atom' }] },
        { scale: 'MD', name: '原子扩散', description: '溶质原子扩散系数', key_parameters: [{ name: 'temperature', default_value: 473, description: '时效温度(K)' }], expected_output: [{ quantity: 'diffusivity', unit: 'm2/s' }] },
        { scale: 'Phase-Field', name: '析出动力学', description: '相场析出过程模拟', key_parameters: [{ name: 'aging_time', default_value: 24, description: '时效时间(h)' }], expected_output: [{ quantity: 'precipitate_radius', unit: 'nm' }] },
        { scale: 'FE', name: '强化预测', description: '宏观力学性能预测', key_parameters: [{ name: 'volume_fraction', default_value: 0.05, description: '析出相体积分数' }], expected_output: [{ quantity: 'yield_strength', unit: 'MPa' }] }
      ],
      estimated_time_min: 300,
      difficulty: 'intermediate' as PresetDifficulty
    },
    {
      id: 'preset-fracture-001',
      name: 'Steel Ductile Fracture',
      name_zh: '钢材延性断裂模拟',
      category: 'fracture' as PresetCategory,
      description: '从DFT层错能计算到MD裂纹萌生模拟，再到相场断裂和FE宏观断裂韧性预测的多尺度断裂工作流。',
      material_examples: ['High-strength steel', 'Stainless steel 304', 'HSLA steel'],
      steps: [
        { scale: 'DFT', name: '层错能计算', description: '计算层错能和 stacking fault energy', key_parameters: [{ name: 'magnetism', default_value: 'ferro', description: '磁性设置' }], expected_output: [{ quantity: 'stacking_fault_energy', unit: 'mJ/m2' }] },
        { scale: 'MD', name: '裂纹萌生', description: '原子尺度裂纹萌生模拟', key_parameters: [{ name: 'crack_mode', default_value: 'I', description: '裂纹类型' }], expected_output: [{ quantity: 'critical_stress_intensity', unit: 'MPa*m0.5' }] },
        { scale: 'Phase-Field', name: '裂纹扩展', description: '相场裂纹扩展模拟', key_parameters: [{ name: 'fracture_energy', default_value: 50, description: '断裂能(J/m2)' }], expected_output: [{ quantity: 'crack_path', unit: '-' }] },
        { scale: 'FE', name: '断裂韧性', description: '宏观断裂韧性预测', key_parameters: [{ name: 'specimen_type', default_value: 'CT', description: '试样类型' }], expected_output: [{ quantity: 'K_IC', unit: 'MPa*m0.5' }] }
      ],
      estimated_time_min: 420,
      difficulty: 'advanced' as PresetDifficulty
    },
    {
      id: 'preset-diffusion-001',
      name: 'Mg-Al Interdiffusion',
      name_zh: 'Mg-Al互扩散模拟',
      category: 'diffusion' as PresetCategory,
      description: '从DFT扩散势垒到MD扩散系数，再到相场浓度场演化和FE宏观扩散模拟的完整多尺度扩散工作流。',
      material_examples: ['Mg-Al alloy', 'Mg-RE alloy', 'Li-ion battery materials'],
      steps: [
        { scale: 'DFT', name: '扩散势垒', description: 'NEB方法计算扩散势垒', key_parameters: [{ name: 'nimages', default_value: 7, description: 'NEB像数' }], expected_output: [{ quantity: 'migration_barrier', unit: 'eV' }] },
        { scale: 'MD', name: '扩散系数', description: 'MSD方法计算扩散系数', key_parameters: [{ name: 'temperature', default_value: 673, description: '温度(K)' }], expected_output: [{ quantity: 'diffusivity', unit: 'm2/s' }] },
        { scale: 'Phase-Field', name: '浓度场演化', description: '相场浓度场模拟', key_parameters: [{ name: 'initial_concentration', default_value: 0.1, description: '初始浓度' }], expected_output: [{ quantity: 'concentration_profile', unit: 'at%' }] },
        { scale: 'FE', name: '宏观扩散', description: 'Fick定律宏观扩散', key_parameters: [{ name: 'boundary_condition', default_value: 'dirichlet', description: '边界条件' }], expected_output: [{ quantity: 'diffusion_depth', unit: 'um' }] }
      ],
      estimated_time_min: 240,
      difficulty: 'beginner' as PresetDifficulty
    }
  ]
}

// ============ Computed ============

const filteredPresets = computed(() => {
  return presets.value.filter((p: WorkflowPreset) => {
    if (selectedCategory.value && p.category !== selectedCategory.value) return false
    if (selectedDifficulty.value && p.difficulty !== selectedDifficulty.value) return false
    return true
  })
})

// ============ Helpers ============

function getCategoryLabel(cat: PresetCategory): string {
  const map: Record<PresetCategory, string> = {
    creep: '蠕变',
    solidification: '凝固',
    precipitation: '析出',
    fracture: '断裂',
    diffusion: '扩散'
  }
  return map[cat] || cat
}

function getCategoryBadgeStyle(cat: PresetCategory): string {
  const colors: Record<PresetCategory, string> = {
    creep: 'background: #ef4444; color: white',
    solidification: 'background: #3b82f6; color: white',
    precipitation: 'background: #10b981; color: white',
    fracture: 'background: #f59e0b; color: white',
    diffusion: 'background: #8b5cf6; color: white'
  }
  return colors[cat] || 'background: var(--bg-base); color: var(--text-secondary)'
}

function getDifficultyLabel(diff: PresetDifficulty): string {
  const map: Record<PresetDifficulty, string> = {
    beginner: '初级',
    intermediate: '中级',
    advanced: '高级'
  }
  return map[diff] || diff
}

function getDifficultyColor(diff: PresetDifficulty): string {
  const map: Record<PresetDifficulty, string> = {
    beginner: 'var(--accent-green)',
    intermediate: 'var(--accent-yellow)',
    advanced: 'var(--accent-red)'
  }
  return map[diff] || 'var(--text-secondary)'
}

function getScaleColor(scale: string): string {
  const map: Record<string, string> = {
    'DFT': '#3b82f6',
    'MD': '#10b981',
    'Phase-Field': '#f59e0b',
    'FE': '#ef4444'
  }
  return map[scale] || '#6b7280'
}

// ============ Actions ============

function resetFilters() {
  selectedCategory.value = ''
  selectedDifficulty.value = ''
  selectedPreset.value = null
}

function applyPresetDirect(preset: WorkflowPreset) {
  selectedPreset.value = preset
  console.log('Applying preset:', preset.id, preset.name_zh)
}

function applyCustomized() {
  if (!selectedPreset.value) return
  const customization: PresetCustomization = {
    preset_id: selectedPreset.value.id,
    parameter_overrides: { ...customOverrides },
    skip_steps: [...skipSteps.value],
    additional_steps: []
  }
  console.log('Applying customized preset:', customization.preset_id, 'overrides:', Object.keys(customization.parameter_overrides).length, 'skip:', customization.skip_steps)
}

function createPresetFromGraphAction() {
  if (!createFromGraph.graphId || !createFromGraph.name) return
  console.log('Creating preset from graph:', createFromGraph.graphId, createFromGraph.name, createFromGraph.category)
  showCreateFromGraph.value = false
  createFromGraph.graphId = ''
  createFromGraph.name = ''
}

// ============ Lifecycle ============

onMounted(() => {
  const data = generateMockPresets()
  presets.value = data
  const count = presets.value.length
  nextTick(() => {
    console.log('Workflow presets loaded:', count, 'presets')
  })
})
</script>
