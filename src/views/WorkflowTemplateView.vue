<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">工作流模板库</h2>
        <p class="text-sm" style="color: var(--text-muted)">V1.9-003/004 | 多尺度工作流模板浏览、下载与市场管理</p>
      </div>
      <div class="flex items-center gap-2">
        <button class="btn btn-ghost text-xs" @click="resetAll">重置</button>
        <button class="btn btn-primary text-xs" @click="showUploadForm = !showUploadForm">
          {{ showUploadForm ? '关闭上传' : '上传模板' }}
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Search & Filter -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            搜索与筛选
          </h4>
          <div class="space-y-2">
            <input
              class="input w-full text-xs"
              type="text"
              v-model="query.keyword"
              placeholder="搜索模板名称、标签..."
            />
            <div>
              <label class="label">分类</label>
              <select class="input w-full text-xs" v-model="query.category">
                <option value="">全部分类</option>
                <option v-for="cat in categoryOptions" :key="cat.value" :value="cat.value">{{ cat.label }}</option>
              </select>
            </div>
            <div>
              <label class="label">排序</label>
              <select class="input w-full text-xs" v-model="query.sort_by">
                <option value="rating">按评分</option>
                <option value="downloads">按下载量</option>
                <option value="newest">按最新</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Step 2: Template List -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            模板列表
            <span class="text-[10px] ml-1" style="color: var(--text-muted)">({{ filteredTemplates.length }})</span>
          </h4>
          <div class="space-y-2">
            <div
              v-for="tpl in filteredTemplates"
              :key="tpl.id"
              @click="selectTemplate(tpl)"
              class="p-2.5 rounded cursor-pointer transition border"
              :style="selectedTemplate?.id === tpl.id
                ? 'background: var(--primary); border-color: var(--primary); color: #fff'
                : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
            >
              <div class="flex items-center justify-between mb-1">
                <span class="text-xs font-medium truncate" style="max-width: 160px">{{ tpl.name_zh }}</span>
                <span
                  class="text-[9px] px-1.5 py-0.5 rounded shrink-0"
                  :style="selectedTemplate?.id === tpl.id
                    ? 'background: rgba(255,255,255,0.2); color: #fff'
                    : 'background: var(--bg-base); color: var(--text-muted)'"
                >{{ categoryLabel(tpl.category) }}</span>
              </div>
              <div class="flex items-center gap-2 text-[10px]" :style="selectedTemplate?.id === tpl.id ? 'opacity: 0.85' : 'color: var(--text-muted)'">
                <span>{{ difficultyLabel(tpl.difficulty) }}</span>
                <span>{{ renderStars(tpl.rating.avg) }}</span>
                <span>{{ tpl.downloads }} 下载</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 3: Selected Template Detail -->
        <div v-if="selectedTemplate">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            模板详情
          </h4>
          <div class="p-3 rounded space-y-2" style="background: var(--bg-elevated)">
            <div class="text-xs font-medium" style="color: var(--text-primary)">{{ selectedTemplate.name_zh }}</div>
            <div class="text-[10px]" style="color: var(--text-muted)">{{ selectedTemplate.description }}</div>
            <div class="grid grid-cols-2 gap-1 text-[10px]">
              <div>
                <span style="color: var(--text-muted)">作者: </span>
                <span style="color: var(--text-secondary)">{{ selectedTemplate.author }}</span>
              </div>
              <div>
                <span style="color: var(--text-muted)">版本: </span>
                <span style="color: var(--text-secondary)">{{ selectedTemplate.version }}</span>
              </div>
            </div>
            <div>
              <div class="text-[10px] mb-1" style="color: var(--text-muted)">工作流步骤</div>
              <div class="space-y-1">
                <div
                  v-for="(step, idx) in selectedTemplate.steps"
                  :key="step.id"
                  class="flex items-center gap-1.5 text-[10px]"
                >
                  <span
                    class="px-1 rounded"
                    :style="scaleBadgeStyle(step.scale)"
                  >{{ scaleLabel(step.scale) }}</span>
                  <span style="color: var(--text-secondary)">{{ step.name }}</span>
                </div>
              </div>
            </div>
            <div class="flex flex-wrap gap-1">
              <span
                v-for="tag in selectedTemplate.tags"
                :key="tag"
                class="text-[9px] px-1.5 py-0.5 rounded"
                style="background: var(--bg-base); color: var(--text-muted)"
              >{{ tag }}</span>
            </div>
          </div>
        </div>

        <!-- Step 4: Actions -->
        <div v-if="selectedTemplate">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            操作
          </h4>
          <div class="space-y-2">
            <button class="btn btn-primary text-xs w-full" @click="downloadTpl">下载模板</button>
            <button class="btn btn-ghost text-xs w-full" @click="showUploadForm = true; uploadForm.category = selectedTemplate.category">上传模板</button>
            <button class="btn btn-ghost text-xs w-full" @click="createFromPipeline">从管线创建</button>
          </div>
        </div>

        <!-- Step 5: Upload Form & Review -->
        <div v-if="showUploadForm">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            上传模板
          </h4>
          <div class="p-3 rounded space-y-2" style="background: var(--bg-elevated)">
            <div>
              <label class="label">模板名称</label>
              <input class="input w-full text-xs" type="text" v-model="uploadForm.name" placeholder="Template name" />
            </div>
            <div>
              <label class="label">中文名称</label>
              <input class="input w-full text-xs" type="text" v-model="uploadForm.name_zh" placeholder="模板中文名称" />
            </div>
            <div>
              <label class="label">分类</label>
              <select class="input w-full text-xs" v-model="uploadForm.category">
                <option v-for="cat in categoryOptions" :key="cat.value" :value="cat.value">{{ cat.label }}</option>
              </select>
            </div>
            <div>
              <label class="label">描述</label>
              <textarea class="input w-full text-xs" rows="3" v-model="uploadForm.description" placeholder="模板描述..." />
            </div>
            <div>
              <label class="label">标签 (逗号分隔)</label>
              <input class="input w-full text-xs" type="text" v-model="uploadForm.tagsStr" placeholder="标签1, 标签2" />
            </div>
            <button class="btn btn-primary text-xs w-full" @click="submitUpload">提交模板</button>
          </div>

          <!-- Review Section -->
          <div class="mt-3 p-3 rounded space-y-2" style="background: var(--bg-elevated)">
            <div class="text-xs font-medium" style="color: var(--text-primary)">评价模板</div>
            <div class="flex items-center gap-1">
              <span
                v-for="star in 5"
                :key="star"
                @click="reviewForm.rating = star"
                class="cursor-pointer text-lg"
                :style="{ color: star <= reviewForm.rating ? '#f59e0b' : 'var(--text-muted)' }"
              >&#9733;</span>
              <span class="text-[10px] ml-1" style="color: var(--text-muted)">{{ reviewForm.rating }}/5</span>
            </div>
            <textarea class="input w-full text-xs" rows="2" v-model="reviewForm.comment" placeholder="写下你的评价..." />
            <button class="btn btn-ghost text-xs w-full" @click="submitReview">提交评价</button>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4">

        <!-- Category Distribution Bar Chart -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">模板分类分布</h4>
          <div class="p-4 rounded" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
            <svg width="100%" height="220" viewBox="0 0 560 220">
              <g v-for="(bar, idx) in categoryBars" :key="bar.label">
                <rect
                  :x="bar.x" :y="bar.y" :width="bar.width" :height="bar.height"
                  :fill="bar.color" rx="3"
                  opacity="0.85"
                />
                <text :x="bar.x + bar.width / 2" :y="bar.y + bar.height + 14" text-anchor="middle" fill="var(--text-secondary)" font-size="10">
                  {{ bar.label }}
                </text>
                <text :x="bar.x + bar.width / 2" :y="bar.y - 6" text-anchor="middle" fill="var(--text-primary)" font-size="11" font-weight="600">
                  {{ bar.count }}
                </text>
              </g>
              <line x1="40" y1="190" x2="540" y2="190" stroke="var(--border-subtle)" stroke-width="1" />
            </svg>
          </div>
        </div>

        <!-- Workflow Diagram -->
        <div v-if="selectedTemplate">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">工作流图</h4>
          <div class="p-4 rounded" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
            <svg width="100%" :height="Math.max(80, selectedTemplate.steps.length * 70 + 20)" :viewBox="'0 0 560 ' + (selectedTemplate.steps.length * 70 + 20)">
              <defs>
                <marker id="wf-arrow" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
                  <polygon points="0 0, 8 3, 0 6" fill="var(--text-muted)" />
                </marker>
              </defs>
              <g v-for="(step, idx) in selectedTemplate.steps" :key="step.id">
                <rect
                  :x="60" :y="idx * 70 + 10" width="180" height="48" rx="6"
                  :fill="scaleColor(step.scale)"
                  stroke="var(--border-default)"
                  stroke-width="1"
                />
                <text :x="150" :y="idx * 70 + 30" text-anchor="middle" fill="white" font-size="11" font-weight="600">
                  {{ step.name }}
                </text>
                <text :x="150" :y="idx * 70 + 46" text-anchor="middle" fill="rgba(255,255,255,0.75)" font-size="9">
                  {{ scaleLabel(step.scale) }}
                </text>
                <!-- Bridge arrow -->
                <line
                  v-if="step.bridge_to_next && idx < selectedTemplate.steps.length - 1"
                  :x1="150" :y1="idx * 70 + 58"
                  :x2="150" :y2="(idx + 1) * 70 + 10"
                  stroke="var(--text-muted)"
                  stroke-width="1.5"
                  stroke-dasharray="4 2"
                  marker-end="url(#wf-arrow)"
                />
                <text
                  v-if="step.bridge_to_next && idx < selectedTemplate.steps.length - 1"
                  :x="170" :y="idx * 70 + 70"
                  fill="var(--text-muted)"
                  font-size="8"
                >{{ step.bridge_to_next.method }}</text>
                <!-- Data output labels -->
                <text :x="260" :y="idx * 70 + 38" fill="var(--text-muted)" font-size="9">
                  {{ step.parameters.map(p => p.name).slice(0, 3).join(', ') }}
                </text>
              </g>
            </svg>
          </div>
        </div>

        <!-- Reviews List -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            用户评价
            <span class="text-[10px] ml-1" style="color: var(--text-muted)">({{ reviews.length }})</span>
          </h4>
          <div class="space-y-2">
            <div
              v-for="rev in reviews"
              :key="rev.user_id + rev.created_at"
              class="p-3 rounded"
              style="background: var(--bg-surface); border: 1px solid var(--border-subtle)"
            >
              <div class="flex items-center justify-between mb-1">
                <div class="flex items-center gap-2">
                  <span class="text-xs font-medium" style="color: var(--text-primary)">{{ rev.user_id }}</span>
                  <span class="text-[10px]" style="color: #f59e0b">{{ renderStars(rev.rating) }}</span>
                </div>
                <span class="text-[10px]" style="color: var(--text-muted)">{{ formatDate(rev.created_at) }}</span>
              </div>
              <div class="text-[11px]" style="color: var(--text-secondary)">{{ rev.comment }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import type { TemplateCategory, WorkflowTemplate, TemplateMarketQuery, TemplateReview } from '../api/workflowTemplate'

// ============ Category Options ============
const categoryOptions = [
  { value: 'solidification', label: '凝固' },
  { value: 'creep', label: '蠕变' },
  { value: 'fatigue', label: '疲劳' },
  { value: 'phase_transformation', label: '相变' },
  { value: 'fracture', label: '断裂' },
  { value: 'diffusion', label: '扩散' },
  { value: 'custom', label: '自定义' }
]

// ============ State ============
const query = reactive<TemplateMarketQuery>({
  keyword: '',
  category: undefined,
  sort_by: 'rating',
  limit: 50,
  offset: 0
})

const selectedTemplate = ref<WorkflowTemplate | null>(null)
const showUploadForm = ref(false)
const templates = ref<WorkflowTemplate[]>([])
const reviews = ref<TemplateReview[]>([])

const uploadForm = reactive({
  name: '',
  name_zh: '',
  category: 'custom' as TemplateCategory,
  description: '',
  tagsStr: ''
})

const reviewForm = reactive({
  rating: 5,
  comment: ''
})

// ============ Computed ============
const filteredTemplates = computed(() => {
  let list = [...templates.value]
  if (query.keyword) {
    const kw = query.keyword.toLowerCase()
    list = list.filter(t =>
      t.name.toLowerCase().includes(kw) ||
      t.name_zh.includes(kw) ||
      t.tags.some(tag => tag.toLowerCase().includes(kw))
    )
  }
  if (query.category) {
    list = list.filter(t => t.category === query.category)
  }
  if (query.sort_by === 'rating') {
    list.sort((a, b) => b.rating.avg - a.rating.avg)
  } else if (query.sort_by === 'downloads') {
    list.sort((a, b) => b.downloads - a.downloads)
  } else if (query.sort_by === 'newest') {
    list.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
  }
  return list
})

const categoryBars = computed(() => {
  const catMap: Record<string, number> = {}
  for (const tpl of templates.value) {
    catMap[tpl.category] = (catMap[tpl.category] || 0) + 1
  }
  const colors: Record<string, string> = {
    solidification: '#3b82f6',
    creep: '#8b5cf6',
    fatigue: '#ec4899',
    phase_transformation: '#f59e0b',
    fracture: '#ef4444',
    diffusion: '#06b6d4',
    custom: '#6b7280'
  }
  const entries = Object.entries(catMap).sort((a, b) => b[1] - a[1])
  const maxCount = Math.max(...entries.map(e => e[1]), 1)
  const barWidth = Math.min(60, (480 / entries.length) - 10)
  const spacing = (480 - entries.length * barWidth) / (entries.length + 1)
  return entries.map(([cat, count], idx) => {
    const label = categoryOptions.find(c => c.value === cat)?.label || cat
    const barHeight = (count / maxCount) * 150
    return {
      label,
      count,
      color: colors[cat] || '#6b7280',
      x: 40 + spacing * (idx + 1) + barWidth * idx,
      y: 180 - barHeight,
      width: barWidth,
      height: barHeight
    }
  })
})

// ============ Helpers ============
function categoryLabel(cat: TemplateCategory): string {
  return categoryOptions.find(c => c.value === cat)?.label || cat
}

function difficultyLabel(d: string): string {
  if (d === 'beginner') return '入门'
  if (d === 'intermediate') return '中级'
  return '高级'
}

function scaleLabel(scale: string): string {
  const map: Record<string, string> = {
    dft: 'DFT',
    md: 'MD',
    phase_field: 'Phase Field',
    fe: 'FE'
  }
  return map[scale] || scale
}

function scaleColor(scale: string): string {
  const map: Record<string, string> = {
    dft: '#3b82f6',
    md: '#8b5cf6',
    phase_field: '#f59e0b',
    fe: '#22c55e'
  }
  return map[scale] || '#6b7280'
}

function scaleBadgeStyle(scale: string): string {
  return `background: ${scaleColor(scale)}22; color: ${scaleColor(scale)}`
}

function renderStars(rating: number): string {
  const full = Math.floor(rating)
  const half = rating - full >= 0.5 ? 1 : 0
  return '\u2605'.repeat(full) + (half ? '\u00BD' : '') + '\u2606'.repeat(5 - full - half)
}

function formatDate(dateStr: string): string {
  const d = new Date(dateStr)
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`
}

// ============ Mock Data ============
const mockTemplates: WorkflowTemplate[] = [
  {
    id: 'tpl-001', name: 'Mg Alloy Creep Full Chain', name_zh: '镁合金蠕变全链条',
    category: 'creep', description: 'DFT层错能→MD位错攀移→相场晶界滑移→FE蠕变预测完整工作流',
    author: 'CAELab Team', version: '1.2.0',
    steps: [
      { id: 's1', scale: 'dft', name: '层错能计算', description: 'VASP PBE计算ISF/ESF能量', parameters: [{ name: 'xc_functional', type: 'string', default_value: 'PBE' }], bridge_to_next: { method: 'EAM Fitting', config: {} } },
      { id: 's2', scale: 'md', name: '位错攀移模拟', description: 'LAMMPS NPT蠕变模拟', parameters: [{ name: 'temperature', type: 'number', default_value: 573, min: 300, max: 800 }], bridge_to_next: { method: 'Mobility Extraction', config: {} } },
      { id: 's3', scale: 'phase_field', name: '晶界滑移相场', description: 'Allen-Cahn晶粒演化', parameters: [{ name: 'grid_size', type: 'number', default_value: 256 }], bridge_to_next: { method: 'Homogenization', config: {} } },
      { id: 's4', scale: 'fe', name: '宏观蠕变FE', description: 'Abaqus蠕变本构', parameters: [{ name: 'element_type', type: 'string', default_value: 'C3D8' }], bridge_to_next: null }
    ],
    tags: ['镁合金', '蠕变', '四尺度', 'DFT-MD-PF-FE'], difficulty: 'advanced',
    estimated_time_min: 480, downloads: 342, rating: { avg: 4.8, count: 56 }, is_official: true,
    created_at: '2026-01-15T10:00:00Z', updated_at: '2026-03-20T14:30:00Z'
  },
  {
    id: 'tpl-002', name: 'Steel Solidification Chain', name_zh: '高强钢凝固全流程',
    category: 'solidification', description: 'DFT形核势垒→MD原子扩散→相场枝晶生长→FE铸造应力分析',
    author: 'Prof. Wang', version: '1.0.1',
    steps: [
      { id: 's1', scale: 'dft', name: '形核势垒', description: '计算界面能和形核自由能', parameters: [{ name: 'k_points', type: 'string', default_value: '6x6x6' }], bridge_to_next: { method: 'Interfacial Energy', config: {} } },
      { id: 's2', scale: 'md', name: '液态扩散', description: '液态金属原子扩散系数', parameters: [{ name: 'temperature', type: 'number', default_value: 1800 }], bridge_to_next: { method: 'Diffusivity Transfer', config: {} } },
      { id: 's3', scale: 'phase_field', name: '枝晶生长', description: 'KKS相场枝晶模拟', parameters: [{ name: 'undercooling', type: 'number', default_value: 20 }], bridge_to_next: { method: 'Micro-Macro Bridge', config: {} } },
      { id: 's4', scale: 'fe', name: '铸造应力', description: '热应力与变形分析', parameters: [{ name: 'mesh_size', type: 'number', default_value: 1.0 }], bridge_to_next: null }
    ],
    tags: ['高强钢', '凝固', '枝晶', '铸造'], difficulty: 'advanced',
    estimated_time_min: 600, downloads: 218, rating: { avg: 4.5, count: 38 }, is_official: true,
    created_at: '2026-02-10T08:00:00Z', updated_at: '2026-03-15T16:00:00Z'
  },
  {
    id: 'tpl-003', name: 'Fatigue Crack Initiation', name_zh: '疲劳裂纹萌生',
    category: 'fatigue', description: 'DFT层错能→MD位错发射→相场损伤演化→FE疲劳寿命预测',
    author: 'Dr. Chen', version: '1.1.0',
    steps: [
      { id: 's1', scale: 'dft', name: '层错能计算', description: '不同取向层错能', parameters: [{ name: 'supercell', type: 'string', default_value: '2x2x12' }], bridge_to_next: { method: 'SFE Transfer', config: {} } },
      { id: 's2', scale: 'md', name: '位错发射', description: '裂纹尖端位错发射模拟', parameters: [{ name: 'strain_rate', type: 'number', default_value: 1e9 }], bridge_to_next: { method: 'Damage Parameter', config: {} } },
      { id: 's3', scale: 'phase_field', name: '损伤演化', description: '相场疲劳损伤累积', parameters: [{ name: 'cycle_blocks', type: 'number', default_value: 100 }], bridge_to_next: { method: 'S-N Parameters', config: {} } },
      { id: 's4', scale: 'fe', name: '疲劳寿命', description: 'Paris law裂纹扩展', parameters: [{ name: 'stress_ratio', type: 'number', default_value: 0.1 }], bridge_to_next: null }
    ],
    tags: ['疲劳', '裂纹', 'Paris law', '寿命预测'], difficulty: 'intermediate',
    estimated_time_min: 360, downloads: 156, rating: { avg: 4.3, count: 24 }, is_official: false,
    created_at: '2026-02-20T12:00:00Z', updated_at: '2026-03-18T09:00:00Z'
  },
  {
    id: 'tpl-004', name: 'Phase Transformation NiTi', name_zh: 'NiTi相变模拟',
    category: 'phase_transformation', description: 'DFT马氏体稳定性→MD相变路径→相场微观组织→FE超弹性响应',
    author: 'Prof. Li', version: '1.0.0',
    steps: [
      { id: 's1', scale: 'dft', name: '相稳定性', description: 'B2/B19\'能量面计算', parameters: [{ name: 'xc_functional', type: 'string', default_value: 'PBEsol' }], bridge_to_next: { method: 'Landau Params', config: {} } },
      { id: 's2', scale: 'md', name: '相变路径', description: '分子动力学相变过程', parameters: [{ name: 'cooling_rate', type: 'number', default_value: 1e12 }], bridge_to_next: { method: 'Kinetic Coeff', config: {} } },
      { id: 's3', scale: 'phase_field', name: '微观组织', description: '马氏体变体选择', parameters: [{ name: 'variants', type: 'number', default_value: 12 }], bridge_to_next: { method: 'Constitutive Law', config: {} } },
      { id: 's4', scale: 'fe', name: '超弹性', description: 'SMA超弹性行为', parameters: [{ name: 'max_strain', type: 'number', default_value: 0.06 }], bridge_to_next: null }
    ],
    tags: ['NiTi', '形状记忆', '相变', '超弹性'], difficulty: 'advanced',
    estimated_time_min: 420, downloads: 189, rating: { avg: 4.6, count: 31 }, is_official: true,
    created_at: '2026-01-25T14:00:00Z', updated_at: '2026-03-22T11:00:00Z'
  },
  {
    id: 'tpl-005', name: 'Fracture Toughness SiC', name_zh: 'SiC断裂韧性',
    category: 'fracture', description: 'DFT解理能→MD裂纹扩展→相场断裂模拟→FE断裂韧性预测',
    author: 'Dr. Zhang', version: '0.9.0',
    steps: [
      { id: 's1', scale: 'dft', name: '解理能', description: '不同晶面解理能', parameters: [{ name: 'surface', type: 'string', default_value: '(111)' }], bridge_to_next: { method: 'Cohesive Zone', config: {} } },
      { id: 's2', scale: 'md', name: '裂纹扩展', description: '原子尺度裂纹传播', parameters: [{ name: 'crack_velocity', type: 'number', default_value: 1000 }], bridge_to_next: { method: 'Fracture Energy', config: {} } },
      { id: 's3', scale: 'phase_field', name: '断裂相场', description: 'AT2相场断裂模型', parameters: [{ name: 'regularization', type: 'number', default_value: 0.02 }], bridge_to_next: { method: 'J-Integral', config: {} } },
      { id: 's4', scale: 'fe', name: '韧性预测', description: 'CTOD和J积分', parameters: [{ name: 'specimen_type', type: 'string', default_value: 'SENB' }], bridge_to_next: null }
    ],
    tags: ['SiC', '陶瓷', '断裂', '韧性'], difficulty: 'intermediate',
    estimated_time_min: 300, downloads: 98, rating: { avg: 4.1, count: 15 }, is_official: false,
    created_at: '2026-03-01T09:00:00Z', updated_at: '2026-03-25T17:00:00Z'
  },
  {
    id: 'tpl-006', name: 'Hydrogen Diffusion Steel', name_zh: '钢中氢扩散',
    category: 'diffusion', description: 'DFT氢陷阱能→MD氢扩散系数→相场氢浓度分布→FE氢致开裂评估',
    author: 'Dr. Liu', version: '1.0.0',
    steps: [
      { id: 's1', scale: 'dft', name: '氢陷阱能', description: '空位/晶界氢结合能', parameters: [{ name: 'trap_site', type: 'string', default_value: 'vacancy' }], bridge_to_next: { method: 'Trap Parameters', config: {} } },
      { id: 's2', scale: 'md', name: '氢扩散', description: '氢原子扩散路径', parameters: [{ name: 'temperature', type: 'number', default_value: 300 }], bridge_to_next: { method: 'Diffusivity', config: {} } },
      { id: 's3', scale: 'phase_field', name: '浓度场', description: '氢浓度时空分布', parameters: [{ name: 'initial_conc', type: 'number', default_value: 1.0 }], bridge_to_next: { method: 'HEDE Model', config: {} } },
      { id: 's4', scale: 'fe', name: '开裂评估', description: '氢致开裂临界条件', parameters: [{ name: 'threshold_H', type: 'number', default_value: 2.0 }], bridge_to_next: null }
    ],
    tags: ['氢脆', '扩散', '陷阱', 'HEDE'], difficulty: 'intermediate',
    estimated_time_min: 240, downloads: 127, rating: { avg: 4.4, count: 20 }, is_official: false,
    created_at: '2026-02-05T11:00:00Z', updated_at: '2026-03-19T13:00:00Z'
  },
  {
    id: 'tpl-007', name: 'Al Alloy Precipitation', name_zh: '铝合金析出强化',
    category: 'solidification', description: 'DFT析出相形成能→MD原子有序化→相场析出长大→FE时效强化预测',
    author: 'CAELab Team', version: '1.3.0',
    steps: [
      { id: 's1', scale: 'dft', name: '形成能', description: 'GP区/析出相形成能', parameters: [{ name: 'supercell', type: 'string', default_value: '3x3x3' }], bridge_to_next: { method: 'Nucleation Barrier', config: {} } },
      { id: 's2', scale: 'md', name: '有序化', description: '溶质原子偏聚', parameters: [{ name: 'concentration', type: 'number', default_value: 0.05 }], bridge_to_next: { method: 'Mobility', config: {} } },
      { id: 's3', scale: 'phase_field', name: '析出长大', description: '多组元析出动力学', parameters: [{ name: 'n_precipitates', type: 'number', default_value: 50 }], bridge_to_next: { method: 'Orowan Params', config: {} } },
      { id: 's4', scale: 'fe', name: '强化预测', description: 'Orowan强化模型', parameters: [{ name: 'bypass_mechanism', type: 'string', default_value: 'orowan' }], bridge_to_next: null }
    ],
    tags: ['铝合金', '析出', '时效', 'Orowan'], difficulty: 'advanced',
    estimated_time_min: 540, downloads: 276, rating: { avg: 4.7, count: 42 }, is_official: true,
    created_at: '2026-01-10T08:00:00Z', updated_at: '2026-03-21T10:00:00Z'
  },
  {
    id: 'tpl-008', name: 'Creep Zr Alloy', name_zh: '锆合金蠕变',
    category: 'creep', description: 'DFT堆垛层错→MD位错滑移→相场晶粒长大→FE反应堆组件蠕变',
    author: 'Dr. Yang', version: '1.0.0',
    steps: [
      { id: 's1', scale: 'dft', name: '层错能', description: 'Zr HCP层错能', parameters: [{ name: 'xc_functional', type: 'string', default_value: 'PBE' }], bridge_to_next: { method: 'Potential Fit', config: {} } },
      { id: 's2', scale: 'md', name: '位错滑移', description: '柱面/基面滑移', parameters: [{ name: 'temperature', type: 'number', default_value: 673 }], bridge_to_next: { method: 'CRSS Transfer', config: {} } },
      { id: 's3', scale: 'phase_field', name: '晶粒长大', description: '辐照下晶粒演化', parameters: [{ name: 'irradiation_dose', type: 'number', default_value: 1.0 }], bridge_to_next: { method: 'IP Model', config: {} } },
      { id: 's4', scale: 'fe', name: '组件蠕变', description: '包壳管蠕变分析', parameters: [{ name: 'internal_pressure', type: 'number', default_value: 15.0 }], bridge_to_next: null }
    ],
    tags: ['锆合金', '核用', '辐照', '蠕变'], difficulty: 'advanced',
    estimated_time_min: 500, downloads: 87, rating: { avg: 4.2, count: 12 }, is_official: false,
    created_at: '2026-03-05T16:00:00Z', updated_at: '2026-03-26T08:00:00Z'
  },
  {
    id: 'tpl-009', name: 'Fatigue Ti Alloy', name_zh: '钛合金疲劳',
    category: 'fatigue', description: 'DFT alpha/beta相稳定性→MD位错-裂纹交互→相场疲劳条带→FE部件寿命',
    author: 'Prof. Zhao', version: '1.0.0',
    steps: [
      { id: 's1', scale: 'dft', name: '相稳定性', description: 'HCP/BCC相能量', parameters: [{ name: 'alloying_element', type: 'string', default_value: 'V' }], bridge_to_next: { method: 'Phase Fraction', config: {} } },
      { id: 's2', scale: 'md', name: '位错交互', description: 'alpha/beta界面位错', parameters: [{ name: 'misfit_dislocation', type: 'boolean', default_value: true }], bridge_to_next: { method: 'Backstress', config: {} } },
      { id: 's3', scale: 'phase_field', name: '疲劳条带', description: 'PSB形成与演化', parameters: [{ name: 'loading_freq', type: 'number', default_value: 10 }], bridge_to_next: { method: 'Damage Law', config: {} } },
      { id: 's4', scale: 'fe', name: '部件寿命', description: '涡轮叶片寿命', parameters: [{ name: 'notch_factor', type: 'number', default_value: 1.5 }], bridge_to_next: null }
    ],
    tags: ['钛合金', '涡轮', '疲劳', 'PSB'], difficulty: 'advanced',
    estimated_time_min: 450, downloads: 134, rating: { avg: 4.5, count: 22 }, is_official: true,
    created_at: '2026-02-15T10:00:00Z', updated_at: '2026-03-24T15:00:00Z'
  },
  {
    id: 'tpl-010', name: 'Martensitic Transform Steel', name_zh: '钢马氏体相变',
    category: 'phase_transformation', description: 'DFT Ms温度预测→MD切变机制→相场马氏体形貌→FE淬火变形',
    author: 'Dr. Sun', version: '0.8.0',
    steps: [
      { id: 's1', scale: 'dft', name: 'Ms温度', description: '化学驱动力计算', parameters: [{ name: 'composition', type: 'string', default_value: 'Fe-0.4C' }], bridge_to_next: { method: 'Thermodynamic Params', config: {} } },
      { id: 's2', scale: 'md', name: '切变机制', description: 'Bain路径模拟', parameters: [{ name: 'cooling_rate', type: 'number', default_value: 1e13 }], bridge_to_next: { method: 'Shear Modulus', config: {} } },
      { id: 's3', scale: 'phase_field', name: '马氏体形貌', description: '板条马氏体形态', parameters: [{ name: 'variant_pairs', type: 'number', default_value: 24 }], bridge_to_next: { method: 'Transformation Strain', config: {} } },
      { id: 's4', scale: 'fe', name: '淬火变形', description: '热-相变耦合分析', parameters: [{ name: 'quench_medium', type: 'string', default_value: 'water' }], bridge_to_next: null }
    ],
    tags: ['马氏体', '淬火', 'Bain', '变形'], difficulty: 'intermediate',
    estimated_time_min: 380, downloads: 112, rating: { avg: 4.0, count: 18 }, is_official: false,
    created_at: '2026-03-10T14:00:00Z', updated_at: '2026-03-27T09:00:00Z'
  },
  {
    id: 'tpl-011', name: 'Composite Interface', name_zh: '复合材料界面',
    category: 'fracture', description: 'DFT界面结合能→MD界面脱粘→相场界面裂纹→FE层合板渐进损伤',
    author: 'Dr. Wu', version: '1.0.0',
    steps: [
      { id: 's1', scale: 'dft', name: '界面结合能', description: '纤维/基体界面', parameters: [{ name: 'fiber_type', type: 'string', default_value: 'carbon' }], bridge_to_next: { method: 'Cohesive Law', config: {} } },
      { id: 's2', scale: 'md', name: '界面脱粘', description: 'Pull-out模拟', parameters: [{ name: 'pull_rate', type: 'number', default_value: 1e8 }], bridge_to_next: { method: 'Traction-Separation', config: {} } },
      { id: 's3', scale: 'phase_field', name: '界面裂纹', description: '层间裂纹扩展', parameters: [{ name: 'interface_strength', type: 'number', default_value: 50 }], bridge_to_next: { method: 'Delamination Params', config: {} } },
      { id: 's4', scale: 'fe', name: '渐进损伤', description: 'Hashin准则', parameters: [{ name: 'ply_count', type: 'number', default_value: 16 }], bridge_to_next: null }
    ],
    tags: ['复合材料', '界面', '脱粘', '层合板'], difficulty: 'intermediate',
    estimated_time_min: 320, downloads: 95, rating: { avg: 4.3, count: 16 }, is_official: false,
    created_at: '2026-02-28T09:00:00Z', updated_at: '2026-03-23T12:00:00Z'
  },
  {
    id: 'tpl-012', name: 'Custom Multi-Scale', name_zh: '自定义多尺度模板',
    category: 'custom', description: '用户自定义多尺度工作流模板，支持任意尺度组合与桥接方法配置',
    author: 'Community', version: '1.0.0',
    steps: [
      { id: 's1', scale: 'dft', name: '自定义DFT', description: '用户自定义DFT计算', parameters: [{ name: 'input_file', type: 'string', default_value: 'POSCAR' }], bridge_to_next: { method: 'Custom Bridge', config: {} } },
      { id: 's2', scale: 'md', name: '自定义MD', description: '用户自定义MD模拟', parameters: [{ name: 'input_file', type: 'string', default_value: 'in.lammps' }], bridge_to_next: { method: 'Custom Bridge', config: {} } },
      { id: 's3', scale: 'phase_field', name: '自定义相场', description: '用户自定义相场模拟', parameters: [{ name: 'config_file', type: 'string', default_value: 'params.json' }], bridge_to_next: { method: 'Custom Bridge', config: {} } },
      { id: 's4', scale: 'fe', name: '自定义FE', description: '用户自定义FE分析', parameters: [{ name: 'input_file', type: 'string', default_value: 'job.inp' }], bridge_to_next: null }
    ],
    tags: ['自定义', '通用', '模板', '灵活'], difficulty: 'beginner',
    estimated_time_min: 60, downloads: 423, rating: { avg: 3.9, count: 67 }, is_official: false,
    created_at: '2026-01-05T08:00:00Z', updated_at: '2026-03-28T10:00:00Z'
  }
]

const mockReviews: TemplateReview[] = [
  { user_id: 'user_001', template_id: 'tpl-001', rating: 5, comment: '非常完整的镁合金蠕变工作流，从DFT到FE全链条打通，结果与文献吻合很好。', created_at: '2026-03-20T14:30:00Z' },
  { user_id: 'user_002', template_id: 'tpl-001', rating: 5, comment: '参数设置合理，桥接方法选择恰当，适合作为教学案例。', created_at: '2026-03-18T09:15:00Z' },
  { user_id: 'user_003', template_id: 'tpl-001', rating: 4, comment: '整体很好，但MD步骤计算量较大，建议提供GPU加速选项。', created_at: '2026-03-15T16:45:00Z' },
  { user_id: 'user_004', template_id: 'tpl-002', rating: 4, comment: '凝固过程模拟效果不错，枝晶形貌与实验对比合理。', created_at: '2026-03-22T11:20:00Z' },
  { user_id: 'user_005', template_id: 'tpl-004', rating: 5, comment: 'NiTi相变模拟非常专业，Landau参数提取方法很巧妙。', created_at: '2026-03-19T13:00:00Z' },
  { user_id: 'user_006', template_id: 'tpl-007', rating: 5, comment: '铝合金析出强化模板是做时效研究的必备工具，强烈推荐。', created_at: '2026-03-21T10:30:00Z' },
  { user_id: 'user_007', template_id: 'tpl-012', rating: 4, comment: '自定义模板很灵活，但文档可以更详细一些。', created_at: '2026-03-28T10:00:00Z' },
  { user_id: 'user_008', template_id: 'tpl-006', rating: 4, comment: '氢扩散模板对氢脆研究很有帮助，陷阱能计算准确。', created_at: '2026-03-25T17:30:00Z' }
]

// ============ Actions ============
function resetAll() {
  query.keyword = ''
  query.category = undefined
  query.sort_by = 'rating'
  selectedTemplate.value = null
  showUploadForm.value = false
  reviewForm.rating = 5
  reviewForm.comment = ''
}

function selectTemplate(tpl: WorkflowTemplate) {
  selectedTemplate.value = tpl
  reviews.value = mockReviews.filter(r => r.template_id === tpl.id)
  if (reviews.value.length === 0) {
    reviews.value = mockReviews.slice(0, 3)
  }
}

function downloadTpl() {
  if (!selectedTemplate.value) return
  alert(`正在下载模板: ${selectedTemplate.value.name_zh}`)
}

function createFromPipeline() {
  if (!selectedTemplate.value) return
  alert(`从管线创建模板: ${selectedTemplate.value.name_zh}`)
}

function submitUpload() {
  if (!uploadForm.name || !uploadForm.name_zh) return
  const tags = uploadForm.tagsStr.split(',').map(t => t.trim()).filter(Boolean)
  alert(`模板 "${uploadForm.name_zh}" 已提交审核`)
  uploadForm.name = ''
  uploadForm.name_zh = ''
  uploadForm.description = ''
  uploadForm.tagsStr = ''
}

function submitReview() {
  if (!selectedTemplate.value || !reviewForm.comment) return
  const newReview: TemplateReview = {
    user_id: 'current_user',
    template_id: selectedTemplate.value.id,
    rating: reviewForm.rating,
    comment: reviewForm.comment,
    created_at: new Date().toISOString()
  }
  reviews.value.unshift(newReview)
  reviewForm.comment = ''
  reviewForm.rating = 5
}

// ============ Lifecycle ============
onMounted(() => {
  templates.value = mockTemplates
  reviews.value = mockReviews.slice(0, 3)
})
</script>
