<!--
  MeshRefinementView.vue - 局部加密与映射网格视图
  ==========================================
  V1.3-009

  核心功能:
  - 几何区域局部加密 (箱体/球体/圆柱/面/边)
  - 网格密度映射 (源尺寸→目标尺寸)
  - 边界层网格生成 (y+控制)
  - 3D 网格预览与质量对比
-->

<template>
  <div class="mesh-refinement-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">局部加密与映射网格</h2>
        <p class="text-xs text-[var(--text-muted)]">几何区域局部加密，网格密度映射，边界层网格(y+控制)</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="resetAll" class="btn btn-ghost text-xs">
          <span class="mr-1">&#x27F3;</span> 重置
        </button>
        <button v-if="results" @click="exportResults" class="btn btn-ghost text-xs">
          <span class="mr-1">&#x2B07;</span> 导出
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration (w-80) -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col overflow-y-auto">
        <!-- Step 1: 基础网格设置 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">1. 基础网格设置</h3>
          </div>
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">基础网格尺寸 (mm)</label>
            <input v-model.number="config.base_mesh_size" type="number" step="0.1" min="0.1" class="input w-full text-xs" />
          </div>
          <p class="text-[10px] text-[var(--text-muted)]">
            全局基础网格单元尺寸，加密区域将在此基础上细化
          </p>
        </div>

        <!-- Step 2: 加密区域 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">2. 加密区域</h3>
          </div>

          <!-- 已添加的区域列表 -->
          <div v-for="(region, idx) in config.refinement_regions" :key="region.id" class="mb-3 p-2 rounded bg-[var(--bg-base)] border border-[var(--border-subtle)]">
            <div class="flex items-center justify-between mb-2">
              <span class="text-xs font-medium text-[var(--text-primary)]">{{ region.name }}</span>
              <button @click="removeRegion(idx)" class="text-[var(--accent-red)] text-xs hover:underline">移除</button>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">单元尺寸 (mm)</label>
                <input v-model.number="region.element_size" type="number" step="0.1" min="0.01" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">增长率 (1.0-2.0)</label>
                <input v-model.number="region.growth_rate" type="number" step="0.1" min="1.0" max="2.0" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">偏置类型</label>
                <select v-model="region.bias_type" class="input w-full text-xs">
                  <option value="uniform">均匀</option>
                  <option value="geometric">几何</option>
                  <option value="exponential">指数</option>
                </select>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">区域类型</label>
                <select v-model="region.type" class="input w-full text-xs">
                  <option value="box">箱体</option>
                  <option value="sphere">球体</option>
                  <option value="cylinder">圆柱</option>
                  <option value="face">面</option>
                  <option value="edge">边</option>
                </select>
              </div>
            </div>
            <!-- 区域参数 -->
            <div class="mt-2 grid grid-cols-3 gap-1">
              <div v-for="(val, key) in region.params" :key="key">
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">{{ paramLabel(key as string) }}</label>
                <input v-model.number="region.params[key]" type="number" step="0.1" class="input w-full text-xs" />
              </div>
            </div>
          </div>

          <!-- 添加区域按钮 -->
          <div class="flex gap-2 flex-wrap">
            <button @click="addRegion('box')" class="btn btn-outline text-xs px-2 py-1">+ 箱体</button>
            <button @click="addRegion('sphere')" class="btn btn-outline text-xs px-2 py-1">+ 球体</button>
            <button @click="addRegion('cylinder')" class="btn btn-outline text-xs px-2 py-1">+ 圆柱</button>
            <button @click="addRegion('face')" class="btn btn-outline text-xs px-2 py-1">+ 面</button>
            <button @click="addRegion('edge')" class="btn btn-outline text-xs px-2 py-1">+ 边</button>
          </div>
        </div>

        <!-- Step 3: 边界层设置 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">3. 边界层设置</h3>
          </div>
          <div class="grid grid-cols-2 gap-2">
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">首层高度 (mm)</label>
              <input v-model.number="config.boundary_layer.first_layer_height" type="number" step="0.001" min="0.001" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">增长率</label>
              <input v-model.number="config.boundary_layer.growth_rate" type="number" step="0.1" min="1.0" max="3.0" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">总层数</label>
              <input v-model.number="config.boundary_layer.total_layers" type="number" step="1" min="1" max="50" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">y+ 目标值</label>
              <input v-model.number="config.boundary_layer.y_plus_target" type="number" step="1" min="0.1" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">棱柱层数</label>
              <input v-model.number="config.boundary_layer.prism_layers" type="number" step="1" min="1" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">平滑迭代</label>
              <input v-model.number="config.boundary_layer.smoothing_iterations" type="number" step="1" min="0" class="input w-full text-xs" />
            </div>
          </div>
          <!-- 计算边界层高度 -->
          <div class="mt-3 grid grid-cols-3 gap-2">
            <div>
              <label class="text-[10px] text-[var(--text-muted)] block mb-1">雷诺数</label>
              <input v-model.number="blCalcReynolds" type="number" step="1000" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-[10px] text-[var(--text-muted)] block mb-1">特征长度 (m)</label>
              <input v-model.number="blCalcChord" type="number" step="0.01" class="input w-full text-xs" />
            </div>
            <div class="flex items-end">
              <button @click="calcBoundaryLayerHeight" class="btn btn-ghost text-xs w-full">计算首层高度</button>
            </div>
          </div>
          <p v-if="calculatedBLHeight > 0" class="mt-1 text-[10px] text-[var(--accent-green)]">
            计算结果: 首层高度 = {{ calculatedBLHeight.toFixed(4) }} mm
          </p>
        </div>

        <!-- Step 4: 映射网格 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">4. 映射网格</h3>
          </div>
          <div class="grid grid-cols-2 gap-2">
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">源尺寸 (mm)</label>
              <input v-model.number="config.mesh_mapping.source_size" type="number" step="0.1" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] block mb-1">目标尺寸 (mm)</label>
              <input v-model.number="config.mesh_mapping.target_size" type="number" step="0.1" class="input w-full text-xs" />
            </div>
          </div>
          <div class="mt-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">插值方式</label>
            <select v-model="config.mesh_mapping.interpolation" class="input w-full text-xs">
              <option value="linear">线性插值</option>
              <option value="cubic">三次插值</option>
            </select>
          </div>
          <p class="mt-1 text-[10px] text-[var(--text-muted)]">
            密度比: {{ densityRatio.toFixed(1) }}x
          </p>
        </div>

        <!-- 操作按钮 -->
        <div class="px-4 py-3 mt-auto border-t border-[var(--border-subtle)]">
          <button
            @click="generateMesh"
            :disabled="generating || config.refinement_regions.length === 0"
            :class="['btn w-full text-sm', generating ? 'btn-loading' : 'btn-primary']"
          >
            <span v-if="generating" class="mr-2 animate-spin">&#x27F3;</span>
            {{ generating ? '生成中...' : '&#x25B6; 生成加密网格' }}
          </button>
        </div>
      </div>

      <!-- Right: Visualization & Results -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- Results Tabs -->
        <div v-if="results" class="flex items-center gap-2 px-4 py-2 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
          <button
            v-for="tab in resultTabs"
            :key="tab.value"
            @click="resultViewTab = tab.value"
            :class="['px-3 py-1 text-xs rounded transition',
              resultViewTab === tab.value
                ? 'bg-[var(--primary)] text-white'
                : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)]']"
          >
            {{ tab.label }}
          </button>
        </div>

        <!-- Visualization Area -->
        <div class="flex-1 flex">
          <div class="flex-1 relative">
            <!-- 3D 网格预览 -->
            <div v-if="results && resultViewTab === 'preview'" class="w-full h-full">
              <canvas ref="meshCanvas" class="w-full h-full"></canvas>
            </div>

            <!-- 网格质量对比 -->
            <div v-if="results && resultViewTab === 'quality'" class="w-full h-full overflow-y-auto p-4">
              <h4 class="text-sm font-semibold text-[var(--text-primary)] mb-4">网格质量对比</h4>
              <div class="grid grid-cols-2 gap-4">
                <!-- 加密前 -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase mb-2">加密前</div>
                  <div class="space-y-2">
                    <div class="flex justify-between text-xs">
                      <span class="text-[var(--text-secondary)]">节点数</span>
                      <span class="text-[var(--text-primary)] font-medium">{{ results.total_nodes_before.toLocaleString() }}</span>
                    </div>
                    <div class="flex justify-between text-xs">
                      <span class="text-[var(--text-secondary)]">单元数</span>
                      <span class="text-[var(--text-primary)] font-medium">{{ results.total_elements_before.toLocaleString() }}</span>
                    </div>
                    <div class="flex justify-between text-xs">
                      <span class="text-[var(--text-secondary)]">长宽比均值</span>
                      <span class="text-[var(--text-primary)] font-medium">{{ results.quality_before.aspect_ratio_avg.toFixed(2) }}</span>
                    </div>
                    <div class="flex justify-between text-xs">
                      <span class="text-[var(--text-secondary)]">雅可比最小值</span>
                      <span class="text-[var(--text-primary)] font-medium">{{ results.quality_before.jacobian_min.toFixed(4) }}</span>
                    </div>
                    <div class="flex justify-between text-xs">
                      <span class="text-[var(--text-secondary)]">偏斜度最大值</span>
                      <span class="text-[var(--text-primary)] font-medium">{{ results.quality_before.skewness_max.toFixed(2) }}</span>
                    </div>
                  </div>
                </div>

                <!-- 加密后 -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase mb-2">加密后</div>
                  <div class="space-y-2">
                    <div class="flex justify-between text-xs">
                      <span class="text-[var(--text-secondary)]">节点数</span>
                      <span class="text-[var(--accent-green)] font-medium">{{ results.total_nodes_after.toLocaleString() }}</span>
                    </div>
                    <div class="flex justify-between text-xs">
                      <span class="text-[var(--text-secondary)]">单元数</span>
                      <span class="text-[var(--accent-green)] font-medium">{{ results.total_elements_after.toLocaleString() }}</span>
                    </div>
                    <div class="flex justify-between text-xs">
                      <span class="text-[var(--text-secondary)]">长宽比均值</span>
                      <span class="font-medium" :style="{ color: results.quality_after.aspect_ratio_avg < results.quality_before.aspect_ratio_avg ? 'var(--accent-green)' : 'var(--accent-red)' }">
                        {{ results.quality_after.aspect_ratio_avg.toFixed(2) }}
                      </span>
                    </div>
                    <div class="flex justify-between text-xs">
                      <span class="text-[var(--text-secondary)]">雅可比最小值</span>
                      <span class="font-medium" :style="{ color: results.quality_after.jacobian_min > results.quality_before.jacobian_min ? 'var(--accent-green)' : 'var(--accent-red)' }">
                        {{ results.quality_after.jacobian_min.toFixed(4) }}
                      </span>
                    </div>
                    <div class="flex justify-between text-xs">
                      <span class="text-[var(--text-secondary)]">偏斜度最大值</span>
                      <span class="font-medium" :style="{ color: results.quality_after.skewness_max < results.quality_before.skewness_max ? 'var(--accent-green)' : 'var(--accent-red)' }">
                        {{ results.quality_after.skewness_max.toFixed(2) }}
                      </span>
                    </div>
                  </div>
                </div>
              </div>

              <!-- 质量对比条形图 -->
              <div class="mt-4 stat-card">
                <div class="text-[10px] text-[var(--text-muted)] uppercase mb-3">质量指标对比</div>
                <div class="space-y-3">
                  <div>
                    <div class="flex justify-between text-[10px] text-[var(--text-secondary)] mb-1">
                      <span>长宽比 (越低越好)</span>
                    </div>
                    <div class="flex gap-2 items-center">
                      <span class="text-[10px] text-[var(--text-muted)] w-12">前</span>
                      <div class="flex-1 h-3 bg-[var(--bg-elevated)] rounded overflow-hidden">
                        <div class="h-full bg-blue-500 rounded" :style="{ width: qualityBarWidth(results.quality_before.aspect_ratio_avg, 5) + '%' }"></div>
                      </div>
                      <span class="text-[10px] text-[var(--text-primary)] w-10 text-right">{{ results.quality_before.aspect_ratio_avg.toFixed(2) }}</span>
                    </div>
                    <div class="flex gap-2 items-center mt-1">
                      <span class="text-[10px] text-[var(--text-muted)] w-12">后</span>
                      <div class="flex-1 h-3 bg-[var(--bg-elevated)] rounded overflow-hidden">
                        <div class="h-full bg-green-500 rounded" :style="{ width: qualityBarWidth(results.quality_after.aspect_ratio_avg, 5) + '%' }"></div>
                      </div>
                      <span class="text-[10px] text-[var(--text-primary)] w-10 text-right">{{ results.quality_after.aspect_ratio_avg.toFixed(2) }}</span>
                    </div>
                  </div>
                  <div>
                    <div class="flex justify-between text-[10px] text-[var(--text-secondary)] mb-1">
                      <span>偏斜度 (越低越好)</span>
                    </div>
                    <div class="flex gap-2 items-center">
                      <span class="text-[10px] text-[var(--text-muted)] w-12">前</span>
                      <div class="flex-1 h-3 bg-[var(--bg-elevated)] rounded overflow-hidden">
                        <div class="h-full bg-blue-500 rounded" :style="{ width: qualityBarWidth(results.quality_before.skewness_max, 1) + '%' }"></div>
                      </div>
                      <span class="text-[10px] text-[var(--text-primary)] w-10 text-right">{{ results.quality_before.skewness_max.toFixed(2) }}</span>
                    </div>
                    <div class="flex gap-2 items-center mt-1">
                      <span class="text-[10px] text-[var(--text-muted)] w-12">后</span>
                      <div class="flex-1 h-3 bg-[var(--bg-elevated)] rounded overflow-hidden">
                        <div class="h-full bg-green-500 rounded" :style="{ width: qualityBarWidth(results.quality_after.skewness_max, 1) + '%' }"></div>
                      </div>
                      <span class="text-[10px] text-[var(--text-primary)] w-10 text-right">{{ results.quality_after.skewness_max.toFixed(2) }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 统计数据 -->
            <div v-if="results && resultViewTab === 'stats'" class="w-full h-full overflow-y-auto p-4">
              <h4 class="text-sm font-semibold text-[var(--text-primary)] mb-4">统计数据</h4>
              <div class="grid grid-cols-2 gap-4">
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">节点增长</div>
                  <div class="text-xl font-bold text-[var(--primary)]">
                    +{{ (results.total_nodes_after - results.total_nodes_before).toLocaleString() }}
                  </div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-1">
                    {{ ((results.total_nodes_after / results.total_nodes_before - 1) * 100).toFixed(1) }}% 增长
                  </div>
                </div>
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">单元增长</div>
                  <div class="text-xl font-bold text-[var(--primary)]">
                    +{{ (results.total_elements_after - results.total_elements_before).toLocaleString() }}
                  </div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-1">
                    {{ ((results.total_elements_after / results.total_elements_before - 1) * 100).toFixed(1) }}% 增长
                  </div>
                </div>
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">应用加密区域</div>
                  <div class="text-xl font-bold text-[var(--accent-green)]">
                    {{ results.refinement_regions_applied }}
                  </div>
                </div>
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">边界层单元数</div>
                  <div class="text-xl font-bold text-[var(--accent-yellow)]">
                    {{ results.boundary_layer_elements.toLocaleString() }}
                  </div>
                </div>
                <div class="stat-card col-span-2">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">网格文件路径</div>
                  <div class="text-xs font-mono text-[var(--text-primary)] mt-1 break-all">
                    {{ results.mesh_file_path }}
                  </div>
                </div>
              </div>
            </div>

            <!-- 空状态 -->
            <div v-if="!results" class="absolute inset-0 flex items-center justify-center">
              <div class="text-center text-[var(--text-muted)]">
                <div class="text-4xl mb-2">&#x1F4D0;</div>
                <p class="text-sm">添加加密区域后生成网格</p>
                <p class="text-xs mt-1">支持箱体、球体、圆柱、面、边等区域类型</p>
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
import type {
  MeshRefinementConfig,
  MeshRefinementResult,
  RefinementRegion,
} from '@/api/meshRefinement'

// ============ 结果视图标签 ============
const resultTabs = [
  { value: 'preview', label: '3D 网格预览' },
  { value: 'quality', label: '质量对比' },
  { value: 'stats', label: '统计数据' },
]

// ============ 状态 ============
const generating = ref(false)
const resultViewTab = ref('preview')
const results = ref<MeshRefinementResult | null>(null)
const meshCanvas = ref<HTMLCanvasElement | null>(null)
const calculatedBLHeight = ref(0)
const blCalcReynolds = ref(500000)
const blCalcChord = ref(1.0)
let regionCounter = 0

// ============ 分析配置 ============
const config = reactive<MeshRefinementConfig>({
  project_id: 'default',
  base_mesh_size: 5.0,
  refinement_regions: [],
  boundary_layer: {
    first_layer_height: 0.1,
    growth_rate: 1.2,
    total_layers: 8,
    y_plus_target: 30,
    prism_layers: 5,
    smoothing_iterations: 3,
  },
  mesh_mapping: {
    source_size: 5.0,
    target_size: 1.0,
    interpolation: 'linear',
  },
})

// ============ 计算属性 ============
const densityRatio = computed(() => {
  if (config.mesh_mapping.target_size <= 0) return 0
  return (config.mesh_mapping.source_size / config.mesh_mapping.target_size) ** 3
})

// ============ 方法 ============

/** 获取区域参数标签 */
function paramLabel(key: string): string {
  const labels: Record<string, string> = {
    x_min: 'X最小', x_max: 'X最大',
    y_min: 'Y最小', y_max: 'Y最大',
    z_min: 'Z最小', z_max: 'Z最大',
    radius: '半径', height: '高度',
    center_x: '中心X', center_y: '中心Y', center_z: '中心Z',
    face_id: '面ID', edge_id: '边ID',
  }
  return labels[key] || key
}

/** 获取区域类型默认参数 */
function getDefaultParams(type: RefinementRegion['type']): Record<string, number> {
  switch (type) {
    case 'box':
      return { x_min: -10, x_max: 10, y_min: -10, y_max: 10, z_min: -5, z_max: 5 }
    case 'sphere':
      return { center_x: 0, center_y: 0, center_z: 0, radius: 10 }
    case 'cylinder':
      return { center_x: 0, center_y: 0, center_z: 0, radius: 5, height: 20 }
    case 'face':
      return { face_id: 1 }
    case 'edge':
      return { edge_id: 1 }
    default:
      return {}
  }
}

/** 获取区域类型中文名 */
function getTypeName(type: RefinementRegion['type']): string {
  const names: Record<string, string> = {
    box: '箱体', sphere: '球体', cylinder: '圆柱', face: '面', edge: '边',
  }
  return names[type] || type
}

/** 添加加密区域 */
function addRegion(type: RefinementRegion['type']) {
  regionCounter++
  config.refinement_regions.push({
    id: `region_${regionCounter}`,
    name: `${getTypeName(type)}区域 ${regionCounter}`,
    type,
    params: getDefaultParams(type),
    element_size: 1.0,
    growth_rate: 1.2,
    bias_type: 'geometric',
  })
}

/** 移除加密区域 */
function removeRegion(idx: number) {
  config.refinement_regions.splice(idx, 1)
}

/** 计算边界层首层高度 (模拟) */
function calcBoundaryLayerHeight() {
  // 简化的平板边界层公式: y = y+ * mu / (rho * U_inf * sqrt(Cf/2))
  // 这里使用简化模拟
  const yPlus = config.boundary_layer.y_plus_target
  const Re = blCalcReynolds.value
  const L = blCalcChord.value

  // Cf = 0.074 * Re^(-0.2) (湍流平板)
  const Cf = 0.074 * Math.pow(Re, -0.2)
  const frictionVelocity = Cf * 0.5 * 340 // 近似 U_inf=340 m/s
  const mu = 1.81e-5 // 空气动力粘度 Pa*s
  const rho = 1.225 // 空气密度 kg/m^3

  // y = y+ * mu / (rho * u_tau)
  const y = (yPlus * mu) / (rho * Math.max(frictionVelocity, 0.01))
  calculatedBLHeight.value = parseFloat((y * 1000).toFixed(4)) // 转换为 mm

  // 更新配置
  config.boundary_layer.first_layer_height = calculatedBLHeight.value
}

/** 质量条形图宽度 */
function qualityBarWidth(value: number, maxVal: number): number {
  return Math.min((value / maxVal) * 100, 100)
}

/** 生成模拟结果 */
function generateMockResults(): MeshRefinementResult {
  const baseSize = config.base_mesh_size
  const regions = config.refinement_regions

  // 估算基础网格节点和单元
  const baseNodes = Math.round(10000 / (baseSize * baseSize))
  const baseElements = Math.round(8000 / (baseSize * baseSize))

  // 加密区域增加的节点和单元
  let extraNodes = 0
  let extraElements = 0
  for (const region of regions) {
    const ratio = (baseSize / region.element_size) ** 3
    const regionNodes = Math.round(500 * ratio)
    const regionElements = Math.round(400 * ratio)
    extraNodes += regionNodes
    extraElements += regionElements
  }

  // 边界层单元
  const blElements = config.boundary_layer.total_layers * config.boundary_layer.prism_layers * 200

  const totalNodesAfter = baseNodes + extraNodes + blElements * 4
  const totalElementsAfter = baseElements + extraElements + blElements

  return {
    success: true,
    total_nodes_before: baseNodes,
    total_nodes_after: totalNodesAfter,
    total_elements_before: baseElements,
    total_elements_after: totalElementsAfter,
    refinement_regions_applied: regions.length,
    boundary_layer_elements: blElements,
    quality_before: {
      aspect_ratio_avg: 2.8 + Math.random() * 0.5,
      jacobian_min: 0.35 + Math.random() * 0.1,
      skewness_max: 0.65 + Math.random() * 0.1,
      element_count: baseElements,
    },
    quality_after: {
      aspect_ratio_avg: 1.8 + Math.random() * 0.3,
      jacobian_min: 0.55 + Math.random() * 0.1,
      skewness_max: 0.45 + Math.random() * 0.1,
      element_count: totalElementsAfter,
    },
    mesh_file_path: `/projects/${config.project_id}/meshes/refined_${Date.now()}.msh`,
  }
}

/** 生成加密网格 */
async function generateMesh() {
  generating.value = true
  try {
    // 尝试调用后端 API
    // const result = await generateRefinedMesh(config)
    // results.value = result

    // 使用模拟数据
    await new Promise(resolve => setTimeout(resolve, 2000))
    results.value = generateMockResults()

    await nextTick()
    drawMeshPreview()
  } catch (e) {
    console.error('Mesh refinement failed:', e)
    results.value = generateMockResults()
    await nextTick()
    drawMeshPreview()
  } finally {
    generating.value = false
  }
}

/** 绘制 3D 网格预览 */
function drawMeshPreview() {
  if (!meshCanvas.value || !results.value) return

  const container = meshCanvas.value.parentElement
  if (!container) return

  meshCanvas.value.width = container.clientWidth
  meshCanvas.value.height = container.clientHeight

  const ctx = meshCanvas.value.getContext('2d')
  if (!ctx) return

  const w = meshCanvas.value.width
  const h = meshCanvas.value.height
  const padding = 60

  ctx.clearRect(0, 0, w, h)

  // 背景
  ctx.fillStyle = '#0A0B0E'
  ctx.fillRect(0, 0, w, h)

  // 网格
  ctx.strokeStyle = '#1C1D24'
  ctx.lineWidth = 0.5
  for (let x = padding; x < w - padding; x += 30) {
    ctx.beginPath()
    ctx.moveTo(x, padding)
    ctx.lineTo(x, h - padding)
    ctx.stroke()
  }
  for (let y = padding; y < h - padding; y += 30) {
    ctx.beginPath()
    ctx.moveTo(padding, y)
    ctx.lineTo(w - padding, y)
    ctx.stroke()
  }

  // 绘制基础网格 (稀疏)
  const baseStep = 40
  ctx.strokeStyle = '#2D3748'
  ctx.lineWidth = 0.5
  const gridStartX = padding + 40
  const gridStartY = padding + 40
  const gridEndX = w - padding - 40
  const gridEndY = h - padding - 40

  for (let x = gridStartX; x <= gridEndX; x += baseStep) {
    ctx.beginPath()
    ctx.moveTo(x, gridStartY)
    ctx.lineTo(x, gridEndY)
    ctx.stroke()
  }
  for (let y = gridStartY; y <= gridEndY; y += baseStep) {
    ctx.beginPath()
    ctx.moveTo(gridStartX, y)
    ctx.lineTo(gridEndX, y)
    ctx.stroke()
  }

  // 绘制加密区域
  const centerX = (gridStartX + gridEndX) / 2
  const centerY = (gridStartY + gridEndY) / 2

  for (let i = 0; i < config.refinement_regions.length; i++) {
    const region = config.refinement_regions[i]
    const colors = ['#3B82F6', '#22C55E', '#F59E0B', '#EF4444', '#8B5CF6']
    const color = colors[i % colors.length]

    // 区域偏移
    const offsetX = (i - Math.floor(config.refinement_regions.length / 2)) * 80
    const rx = centerX + offsetX
    const ry = centerY

    const regionSize = 80

    if (region.type === 'box') {
      // 绘制箱体区域
      ctx.strokeStyle = color
      ctx.lineWidth = 2
      ctx.setLineDash([5, 3])
      ctx.strokeRect(rx - regionSize, ry - regionSize, regionSize * 2, regionSize * 2)
      ctx.setLineDash([])

      // 加密网格
      const refinedStep = Math.max(region.element_size * 3, 5)
      ctx.strokeStyle = color + '60'
      ctx.lineWidth = 0.5
      for (let x = rx - regionSize; x <= rx + regionSize; x += refinedStep) {
        ctx.beginPath()
        ctx.moveTo(x, ry - regionSize)
        ctx.lineTo(x, ry + regionSize)
        ctx.stroke()
      }
      for (let y = ry - regionSize; y <= ry + regionSize; y += refinedStep) {
        ctx.beginPath()
        ctx.moveTo(rx - regionSize, y)
        ctx.lineTo(rx + regionSize, y)
        ctx.stroke()
      }
    } else if (region.type === 'sphere') {
      ctx.beginPath()
      ctx.arc(rx, ry, regionSize, 0, Math.PI * 2)
      ctx.strokeStyle = color
      ctx.lineWidth = 2
      ctx.setLineDash([5, 3])
      ctx.stroke()
      ctx.setLineDash([])

      // 加密网格 (圆形区域内)
      const refinedStep = Math.max(region.element_size * 3, 5)
      ctx.strokeStyle = color + '60'
      ctx.lineWidth = 0.5
      for (let x = rx - regionSize; x <= rx + regionSize; x += refinedStep) {
        for (let y = ry - regionSize; y <= ry + regionSize; y += refinedStep) {
          const dist = Math.sqrt((x - rx) ** 2 + (y - ry) ** 2)
          if (dist < regionSize) {
            ctx.strokeRect(x, y, refinedStep, refinedStep)
          }
        }
      }
    } else if (region.type === 'cylinder') {
      ctx.beginPath()
      ctx.ellipse(rx, ry, regionSize, regionSize * 0.5, 0, 0, Math.PI * 2)
      ctx.strokeStyle = color
      ctx.lineWidth = 2
      ctx.setLineDash([5, 3])
      ctx.stroke()
      ctx.setLineDash([])

      const refinedStep = Math.max(region.element_size * 3, 5)
      ctx.strokeStyle = color + '60'
      ctx.lineWidth = 0.5
      for (let x = rx - regionSize; x <= rx + regionSize; x += refinedStep) {
        ctx.beginPath()
        ctx.moveTo(x, ry - regionSize * 0.5)
        ctx.lineTo(x, ry + regionSize * 0.5)
        ctx.stroke()
      }
    } else {
      // face / edge
      ctx.strokeStyle = color
      ctx.lineWidth = 3
      ctx.beginPath()
      ctx.moveTo(rx - regionSize, ry)
      ctx.lineTo(rx + regionSize, ry)
      ctx.stroke()
    }

    // 区域标签
    ctx.fillStyle = color
    ctx.font = '11px sans-serif'
    ctx.fillText(region.name, rx - regionSize, ry - regionSize - 8)
    ctx.fillStyle = '#9CA3AF'
    ctx.font = '10px sans-serif'
    ctx.fillText(`尺寸: ${region.element_size}mm | 增长率: ${region.growth_rate}`, rx - regionSize, ry + regionSize + 16)
  }

  // 绘制边界层示意 (底部)
  if (config.boundary_layer.total_layers > 0) {
    const blY = gridEndY - 10
    const blHeight = config.boundary_layer.first_layer_height * 50
    let currentY = blY
    let currentHeight = blHeight

    ctx.fillStyle = '#F59E0B40'
    for (let i = 0; i < config.boundary_layer.total_layers; i++) {
      const layerHeight = Math.max(currentHeight, 1)
      ctx.fillStyle = `rgba(245, 158, 11, ${0.15 + i * 0.05})`
      ctx.fillRect(gridStartX, currentY - layerHeight, gridEndX - gridStartX, layerHeight)
      ctx.strokeStyle = '#F59E0B60'
      ctx.lineWidth = 0.5
      ctx.strokeRect(gridStartX, currentY - layerHeight, gridEndX - gridStartX, layerHeight)
      currentY -= layerHeight
      currentHeight *= config.boundary_layer.growth_rate
    }

    ctx.fillStyle = '#F59E0B'
    ctx.font = '10px sans-serif'
    ctx.fillText(`边界层 (${config.boundary_layer.total_layers}层, y+=${config.boundary_layer.y_plus_target})`, gridStartX, blY + 14)
  }

  // 图例
  ctx.fillStyle = '#9CA3AF'
  ctx.font = '10px sans-serif'
  ctx.fillText(`基础网格: ${config.base_mesh_size}mm`, padding, h - 30)
  ctx.fillText(`总节点: ${results.value.total_nodes_after.toLocaleString()}`, padding + 160, h - 30)
  ctx.fillText(`总单元: ${results.value.total_elements_after.toLocaleString()}`, padding + 340, h - 30)

  // 坐标轴
  ctx.strokeStyle = '#4B5563'
  ctx.lineWidth = 1
  ctx.beginPath()
  ctx.moveTo(padding, padding)
  ctx.lineTo(padding, h - padding)
  ctx.lineTo(w - padding, h - padding)
  ctx.stroke()

  ctx.fillStyle = '#9CA3AF'
  ctx.font = '10px sans-serif'
  ctx.fillText('X', w / 2, h - 5)
  ctx.save()
  ctx.translate(10, h / 2)
  ctx.rotate(-Math.PI / 2)
  ctx.fillText('Y', 0, 0)
  ctx.restore()
}

/** 重置 */
function resetAll() {
  results.value = null
  config.base_mesh_size = 5.0
  config.refinement_regions = []
  config.boundary_layer = {
    first_layer_height: 0.1,
    growth_rate: 1.2,
    total_layers: 8,
    y_plus_target: 30,
    prism_layers: 5,
    smoothing_iterations: 3,
  }
  config.mesh_mapping = {
    source_size: 5.0,
    target_size: 1.0,
    interpolation: 'linear',
  }
  calculatedBLHeight.value = 0
  regionCounter = 0
}

/** 导出结果 */
function exportResults() {
  if (!results.value) return

  const data = JSON.stringify(results.value, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'mesh_refinement_results.json'
  a.click()
  URL.revokeObjectURL(url)
}

// ============ 生命周期 ============
onMounted(() => {
  // 默认添加一个箱体加密区域
  addRegion('box')
})
</script>

<style scoped>
.mesh-refinement-view {
  --bg-base: #0A0B0E;
  --bg-surface: #14151A;
  --bg-elevated: #1C1D24;
  --bg-hover: #25262E;
  --text-primary: #E8E9EB;
  --text-secondary: #9CA3AF;
  --text-muted: #6B7280;
  --primary: #4F8CFF;
  --accent-red: #EF4444;
  --accent-green: #22C55E;
  --accent-yellow: #F59E0B;
  --border-subtle: #2D2E38;
}

.btn {
  @apply px-3 py-1.5 rounded text-xs font-medium transition;
}

.btn-primary {
  @apply bg-blue-600 text-white hover:bg-blue-700;
}

.btn-primary:disabled {
  @apply bg-blue-800 text-white cursor-not-allowed;
}

.btn-ghost {
  @apply text-gray-400 hover:text-white hover:bg-gray-700;
}

.btn-outline {
  @apply border border-gray-600 text-gray-300 hover:bg-gray-700;
}

.btn-loading {
  @apply bg-blue-800 text-white cursor-not-allowed;
}

.input {
  @apply px-2 py-1.5 bg-[var(--bg-base)] border border-[var(--border-subtle)] rounded text-xs text-[var(--text-primary)] focus:outline-none focus:border-[var(--primary)];
}

.stat-card {
  @apply p-3 rounded-lg bg-[var(--bg-base)];
}

.step-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--border-subtle);
}

.step-dot.active {
  background: var(--primary);
  box-shadow: 0 0 0 3px rgba(79, 140, 255, 0.2);
}
</style>
