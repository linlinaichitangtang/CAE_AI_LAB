<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">多尺度综合报告生成</h2>
        <p class="text-sm" style="color: var(--text-muted)">V2.0-007 | 多尺度分析结果自动汇总与报告导出</p>
      </div>
      <div class="flex items-center gap-2">
        <button class="btn btn-ghost text-xs" @click="resetConfig">重置</button>
        <button class="btn btn-primary text-xs" @click="handlePreview">预览</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Execution & Basic Info -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            执行与基本信息
          </h4>
          <div class="space-y-2">
            <label class="label">关联执行</label>
            <select class="input w-full text-xs" v-model="config.execution_id">
              <option value="">选择执行...</option>
              <option v-for="ex in executionOptions" :key="ex.id" :value="ex.id">{{ ex.label }}</option>
            </select>
            <label class="label">报告标题</label>
            <input class="input w-full text-xs" v-model="config.title" placeholder="输入报告标题" />
            <label class="label">作者</label>
            <input class="input w-full text-xs" v-model="config.author" placeholder="输入作者名称" />
          </div>
        </div>

        <!-- Step 2: Report Sections -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            报告章节
          </h4>
          <div class="space-y-2">
            <div
              v-for="(section, idx) in config.sections"
              :key="idx"
              class="p-2.5 rounded border space-y-1.5"
              style="background: var(--bg-elevated); border-color: var(--border-default)"
            >
              <div class="flex items-center justify-between">
                <span class="text-xs font-medium" style="color: var(--text-secondary)">章节 {{ idx + 1 }}</span>
                <button class="text-[10px] px-1.5 py-0.5 rounded" style="color: var(--accent-red); background: rgba(239,68,68,0.1)" @click="removeSection(idx)">删除</button>
              </div>
              <input class="input w-full text-xs" v-model="section.title" placeholder="章节标题" />
              <select class="input w-full text-xs" v-model="section.scale">
                <option :value="null">全局</option>
                <option v-for="s in scaleOptions" :key="s" :value="s">{{ s }}</option>
              </select>
              <select class="input w-full text-xs" v-model="section.content_type">
                <option value="text">文本</option>
                <option value="table">表格</option>
                <option value="chart">图表</option>
                <option value="image">图片</option>
              </select>
            </div>
            <button class="btn btn-ghost text-xs w-full" @click="addSection">+ 添加章节</button>
          </div>
        </div>

        <!-- Step 3: Options -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            选项
          </h4>
          <div class="space-y-2">
            <label class="flex items-center gap-2 text-xs cursor-pointer" style="color: var(--text-secondary)">
              <input type="checkbox" v-model="config.include_methodology" class="rounded" />
              包含方法论章节
            </label>
            <label class="flex items-center gap-2 text-xs cursor-pointer" style="color: var(--text-secondary)">
              <input type="checkbox" v-model="config.include_raw_data" class="rounded" />
              包含原始数据
            </label>
            <label class="flex items-center gap-2 text-xs cursor-pointer" style="color: var(--text-secondary)">
              <input type="checkbox" v-model="config.include_references" class="rounded" />
              包含参考文献
            </label>
          </div>
        </div>

        <!-- Step 4: Format & Generate -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            格式与生成
          </h4>
          <div class="space-y-2">
            <label class="label">导出格式</label>
            <div class="flex gap-1">
              <button
                v-for="fmt in formatOptions"
                :key="fmt.value"
                class="flex-1 py-1.5 text-xs rounded border transition"
                :style="config.format === fmt.value
                  ? 'background: var(--primary); border-color: var(--primary); color: #fff'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
                @click="config.format = fmt.value as RgReportFormat"
              >{{ fmt.label }}</button>
            </div>
            <button class="btn btn-ghost text-xs w-full" @click="handlePreview">预览</button>
            <button class="btn btn-primary text-xs w-full" @click="handleGenerate">生成报告</button>
          </div>
        </div>

        <!-- Step 5: Report History -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            生成历史
          </h4>
          <div class="space-y-2">
            <div
              v-for="report in reportHistory"
              :key="report.report_id"
              class="p-2.5 rounded border"
              style="background: var(--bg-elevated); border-color: var(--border-default)"
            >
              <div class="text-xs font-medium" style="color: var(--text-primary)">{{ report.title }}</div>
              <div class="flex items-center gap-2 mt-1 text-[10px]" style="color: var(--text-muted)">
                <span class="px-1.5 py-0.5 rounded" style="background: var(--bg-base)">{{ report.format.toUpperCase() }}</span>
                <span>{{ report.page_count }} 页</span>
                <span>{{ formatFileSize(report.file_size_bytes) }}</span>
                <span>{{ report.generated_at.slice(0, 10) }}</span>
              </div>
              <button class="btn btn-ghost text-[10px] mt-1.5 w-full" @click="handleDownload(report.report_id)">下载</button>
            </div>
          </div>
        </div>

      </div>

      <!-- Right Panel: Visualization & Results -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4" style="background: var(--bg-base)">

        <!-- Report Preview -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">报告预览</h4>
          <div class="rounded-lg border p-6" style="background: var(--bg-surface); border-color: var(--border-default)">
            <!-- Simulated PDF layout -->
            <div class="max-w-2xl mx-auto space-y-4">
              <!-- Title page -->
              <div class="text-center py-8 border-b" style="border-color: var(--border-subtle)">
                <div class="text-[10px] uppercase tracking-widest mb-2" style="color: var(--text-muted)">CAELab V2.0 多尺度分析报告</div>
                <div class="text-xl font-bold" style="color: var(--text-primary)">{{ config.title || '未命名报告' }}</div>
                <div class="text-xs mt-2" style="color: var(--text-secondary)">{{ config.author || '未知作者' }} | {{ new Date().toLocaleDateString('zh-CN') }}</div>
              </div>
              <!-- Sections preview -->
              <div v-for="(section, idx) in config.sections" :key="idx" class="border-t pt-4" style="border-color: var(--border-subtle)">
                <div class="flex items-center gap-2 mb-2">
                  <span class="text-sm font-semibold" style="color: var(--text-primary)">{{ section.title || `章节 ${idx + 1}` }}</span>
                  <span
                    v-if="section.scale"
                    class="px-1.5 py-0.5 rounded text-[10px]"
                    style="background: var(--primary); color: #fff; opacity: 0.85"
                  >{{ section.scale }}</span>
                  <span
                    class="px-1.5 py-0.5 rounded text-[10px]"
                    style="background: var(--bg-elevated); color: var(--text-muted)"
                  >{{ contentTypeLabel(section.content_type) }}</span>
                </div>
                <!-- Placeholder content based on type -->
                <div v-if="section.content_type === 'text'" class="h-16 rounded" style="background: var(--bg-elevated)">
                  <div class="p-2 space-y-1">
                    <div class="h-2 rounded" style="background: var(--border-default); width: 90%"></div>
                    <div class="h-2 rounded" style="background: var(--border-default); width: 75%"></div>
                    <div class="h-2 rounded" style="background: var(--border-default); width: 85%"></div>
                  </div>
                </div>
                <div v-else-if="section.content_type === 'table'" class="h-20 rounded p-2" style="background: var(--bg-elevated)">
                  <div class="grid grid-cols-3 gap-1 text-[10px]" style="color: var(--text-muted)">
                    <div class="h-6 rounded flex items-center justify-center" style="background: var(--border-subtle)">数据列 1</div>
                    <div class="h-6 rounded flex items-center justify-center" style="background: var(--border-subtle)">数据列 2</div>
                    <div class="h-6 rounded flex items-center justify-center" style="background: var(--border-subtle)">数据列 3</div>
                    <div class="h-5 rounded flex items-center justify-center" style="background: var(--bg-base)">1.234</div>
                    <div class="h-5 rounded flex items-center justify-center" style="background: var(--bg-base)">5.678</div>
                    <div class="h-5 rounded flex items-center justify-center" style="background: var(--bg-base)">9.012</div>
                  </div>
                </div>
                <div v-else-if="section.content_type === 'chart'" class="h-24 rounded flex items-center justify-center" style="background: var(--bg-elevated)">
                  <svg width="200" height="80" viewBox="0 0 200 80">
                    <polyline points="10,60 50,40 90,50 130,20 170,30 190,15" fill="none" stroke="var(--primary)" stroke-width="2" />
                    <polyline points="10,65 50,55 90,45 130,50 170,35 190,40" fill="none" stroke="var(--accent-green)" stroke-width="2" stroke-dasharray="4,3" />
                    <line x1="10" y1="70" x2="190" y2="70" stroke="var(--border-default)" stroke-width="1" />
                    <line x1="10" y1="10" x2="10" y2="70" stroke="var(--border-default)" stroke-width="1" />
                  </svg>
                </div>
                <div v-else-if="section.content_type === 'image'" class="h-24 rounded flex items-center justify-center" style="background: var(--bg-elevated)">
                  <div class="text-[10px]" style="color: var(--text-muted)">[图片占位符]</div>
                </div>
              </div>
              <!-- Methodology / Raw Data / References indicators -->
              <div v-if="config.include_methodology || config.include_raw_data || config.include_references" class="border-t pt-4" style="border-color: var(--border-subtle)">
                <div class="flex flex-wrap gap-1">
                  <span v-if="config.include_methodology" class="px-2 py-1 rounded text-[10px]" style="background: var(--bg-elevated); color: var(--text-muted)">+ 方法论章节</span>
                  <span v-if="config.include_raw_data" class="px-2 py-1 rounded text-[10px]" style="background: var(--bg-elevated); color: var(--text-muted)">+ 原始数据附录</span>
                  <span v-if="config.include_references" class="px-2 py-1 rounded text-[10px]" style="background: var(--bg-elevated); color: var(--text-muted)">+ 参考文献</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Report Template Gallery -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">报告模板库</h4>
          <div class="grid grid-cols-3 gap-3">
            <div
              v-for="tpl in templates"
              :key="tpl.id"
              class="p-3 rounded-lg border cursor-pointer transition"
              :style="selectedTemplateId === tpl.id
                ? 'border-color: var(--primary); background: rgba(59,130,246,0.05)'
                : 'border-color: var(--border-default); background: var(--bg-surface)'"
              @click="applyTemplate(tpl)"
            >
              <div class="text-xs font-medium mb-1" style="color: var(--text-primary)">{{ tpl.name }}</div>
              <div class="text-[10px] mb-2" style="color: var(--text-muted)">{{ tpl.description }}</div>
              <div class="flex flex-wrap gap-1">
                <span
                  v-for="(sec, sIdx) in tpl.sections.slice(0, 3)"
                  :key="sIdx"
                  class="px-1 py-0.5 rounded text-[9px]"
                  style="background: var(--bg-elevated); color: var(--text-secondary)"
                >{{ sec.title }}</span>
                <span v-if="tpl.sections.length > 3" class="px-1 py-0.5 rounded text-[9px]" style="color: var(--text-muted)">+{{ tpl.sections.length - 3 }}</span>
              </div>
              <div v-if="tpl.is_default" class="mt-2">
                <span class="px-1.5 py-0.5 rounded text-[9px]" style="background: var(--primary); color: #fff">默认</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Section Editor -->
        <div v-if="editingSectionIndex !== null && config.sections[editingSectionIndex]">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">章节编辑器</h4>
          <div class="rounded-lg border p-4 space-y-3" style="background: var(--bg-surface); border-color: var(--border-default)">
            <div class="flex items-center gap-3">
              <label class="label shrink-0">标题</label>
              <input class="input w-full text-xs" v-model="config.sections[editingSectionIndex].title" placeholder="章节标题" />
            </div>
            <div class="flex items-center gap-3">
              <label class="label shrink-0">尺度</label>
              <span
                v-if="config.sections[editingSectionIndex].scale"
                class="px-2 py-0.5 rounded text-xs"
                style="background: var(--primary); color: #fff"
              >{{ config.sections[editingSectionIndex].scale }}</span>
              <span v-else class="text-xs" style="color: var(--text-muted)">全局</span>
            </div>
            <div class="flex items-center gap-3">
              <label class="label shrink-0">内容类型</label>
              <span class="text-xs" style="color: var(--text-secondary)">{{ contentTypeLabel(config.sections[editingSectionIndex].content_type) }}</span>
            </div>
            <div>
              <label class="label">数据预览</label>
              <div class="rounded p-3 text-xs font-mono" style="background: var(--bg-elevated); color: var(--text-secondary)">
                <pre class="whitespace-pre-wrap text-[10px]">{{ JSON.stringify(config.sections[editingSectionIndex].data, null, 2) || '{ "placeholder": "暂无数据" }' }}</pre>
              </div>
            </div>
          </div>
        </div>

      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import type { RgReportSection, RgReportConfig, RgGeneratedReport, RgTemplate } from '../api/reportGenerator'
import type { RgReportFormat, RgContentType } from '../api/reportGenerator'

// ============ Types & Constants ============

interface ExecutionOption {
  id: string
  label: string
}

interface FormatOption {
  value: RgReportFormat
  label: string
}

const scaleOptions = ['宏观 FE', '介观 Phase Field', '微观 MD', '量子 DFT']

const formatOptions: FormatOption[] = [
  { value: 'pdf', label: 'PDF' },
  { value: 'html', label: 'HTML' },
  { value: 'markdown', label: 'Markdown' },
]

const executionOptions: ExecutionOption[] = [
  { id: 'exec-001', label: 'EXEC-001: 四尺度串联验证' },
  { id: 'exec-002', label: 'EXEC-002: 复合材料多尺度分析' },
  { id: 'exec-003', label: 'EXEC-003: 热力耦合自适应精度' },
]

// ============ Reactive State ============

const config = reactive<RgReportConfig>({
  execution_id: '',
  title: '多尺度综合分析报告',
  author: 'CAELab 研究组',
  sections: [],
  include_methodology: true,
  include_raw_data: false,
  include_references: true,
  format: 'pdf',
})

const selectedTemplateId = ref<string | null>(null)
const editingSectionIndex = ref<number | null>(null)
const reportHistory = ref<RgGeneratedReport[]>([])
const templates = ref<RgTemplate[]>([])

// ============ Computed ============

const sectionCount = computed(() => config.sections.length)

// ============ Mock Data ============

function generateMockSections(): RgReportSection[] {
  return [
    { title: '概述与分析目标', scale: null, content_type: 'text' as RgContentType, data: { summary: '本报告汇总了多尺度分析的关键结果...' }, order: 0 },
    { title: '宏观有限元分析', scale: '宏观 FE', content_type: 'chart' as RgContentType, data: { chart_type: 'stress_distribution' }, order: 1 },
    { title: '介观相场演化', scale: '介观 Phase Field', content_type: 'image' as RgContentType, data: { image_type: 'phase_field_snapshot' }, order: 2 },
    { title: '微观分子动力学', scale: '微观 MD', content_type: 'table' as RgContentType, data: { table_type: 'radial_distribution' }, order: 3 },
    { title: '量子密度泛函', scale: '量子 DFT', content_type: 'chart' as RgContentType, data: { chart_type: 'band_structure' }, order: 4 },
    { title: '跨尺度关联分析', scale: null, content_type: 'text' as RgContentType, data: { summary: '各尺度结果的一致性验证...' }, order: 5 },
  ]
}

function generateMockTemplates(): RgTemplate[] {
  return [
    {
      id: 'tpl-default',
      name: '默认模板',
      description: '标准多尺度报告，包含所有尺度的概要分析',
      is_default: true,
      sections: [
        { title: '概述', scale: null, content_type: 'text' as RgContentType, data: {}, order: 0 },
        { title: '宏观分析', scale: '宏观 FE', content_type: 'chart' as RgContentType, data: {}, order: 1 },
        { title: '结论', scale: null, content_type: 'text' as RgContentType, data: {}, order: 2 },
      ],
    },
    {
      id: 'tpl-detailed',
      name: '详细模板',
      description: '包含完整方法论、原始数据和参考文献',
      is_default: false,
      sections: [
        { title: '方法论', scale: null, content_type: 'text' as RgContentType, data: {}, order: 0 },
        { title: '宏观分析', scale: '宏观 FE', content_type: 'chart' as RgContentType, data: {}, order: 1 },
        { title: '介观分析', scale: '介观 Phase Field', content_type: 'image' as RgContentType, data: {}, order: 2 },
        { title: '微观分析', scale: '微观 MD', content_type: 'table' as RgContentType, data: {}, order: 3 },
        { title: '量子分析', scale: '量子 DFT', content_type: 'chart' as RgContentType, data: {}, order: 4 },
        { title: '结论与建议', scale: null, content_type: 'text' as RgContentType, data: {}, order: 5 },
      ],
    },
  ]
}

function generateMockHistory(): RgGeneratedReport[] {
  return [
    {
      report_id: 'rpt-001',
      title: '四尺度串联验证报告',
      format: 'pdf',
      file_path: '/reports/rpt-001.pdf',
      page_count: 24,
      generated_at: '2026-03-28T14:30:00Z',
      file_size_bytes: 2_456_789,
      sections: generateMockSections(),
    },
    {
      report_id: 'rpt-002',
      title: '复合材料多尺度分析摘要',
      format: 'html',
      file_path: '/reports/rpt-002.html',
      page_count: 12,
      generated_at: '2026-03-25T09:15:00Z',
      file_size_bytes: 856_432,
      sections: generateMockSections().slice(0, 3),
    },
    {
      report_id: 'rpt-003',
      title: '热力耦合精度验证',
      format: 'markdown',
      file_path: '/reports/rpt-003.md',
      page_count: 8,
      generated_at: '2026-03-20T16:45:00Z',
      file_size_bytes: 124_567,
      sections: generateMockSections().slice(0, 4),
    },
  ]
}

// ============ Methods ============

function addSection() {
  const newSection: RgReportSection = {
    title: '',
    scale: null,
    content_type: 'text' as RgContentType,
    data: {},
    order: config.sections.length,
  }
  config.sections.push(newSection)
  editingSectionIndex.value = config.sections.length - 1
}

function removeSection(idx: number) {
  config.sections.splice(idx, 1)
  if (editingSectionIndex.value === idx) {
    editingSectionIndex.value = null
  } else if (editingSectionIndex.value !== null && editingSectionIndex.value > idx) {
    editingSectionIndex.value = editingSectionIndex.value - 1
  }
}

function applyTemplate(tpl: RgTemplate) {
  selectedTemplateId.value = tpl.id
  config.sections = tpl.sections.map((s, i) => ({
    title: s.title,
    scale: s.scale,
    content_type: s.content_type,
    data: { ...s.data },
    order: i,
  }))
}

function contentTypeLabel(ct: RgContentType): string {
  const map: Record<RgContentType, string> = {
    text: '文本',
    table: '表格',
    chart: '图表',
    image: '图片',
  }
  return map[ct] || ct
}

function formatFileSize(bytes: number): string {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1_048_576) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / 1_048_576).toFixed(1) + ' MB'
}

function handlePreview() {
  // In production, call previewReport(config)
  editingSectionIndex.value = 0
}

function handleGenerate() {
  // In production, call generateReport(config)
  const newReport: RgGeneratedReport = {
    report_id: 'rpt-' + String(Date.now()).slice(-6),
    title: config.title,
    format: config.format,
    file_path: `/reports/${config.title}.${config.format}`,
    page_count: config.sections.length * 4 + 2,
    generated_at: new Date().toISOString(),
    file_size_bytes: 1_500_000 + Math.floor(Math.random() * 2_000_000),
    sections: [...config.sections],
  }
  reportHistory.value.unshift(newReport)
}

function handleDownload(reportId: string) {
  // In production, call downloadReport(reportId)
  const report = reportHistory.value.find(r => r.report_id === reportId)
  if (report) {
    const path = report.file_path
    // Simulate download
  }
}

function resetConfig() {
  config.execution_id = ''
  config.title = ''
  config.author = ''
  config.sections = []
  config.include_methodology = true
  config.include_raw_data = false
  config.include_references = true
  config.format = 'pdf'
  selectedTemplateId.value = null
  editingSectionIndex.value = null
}

// ============ Lifecycle ============

onMounted(() => {
  config.sections = generateMockSections()
  reportHistory.value = generateMockHistory()
  templates.value = generateMockTemplates()
  selectedTemplateId.value = 'tpl-default'
})
</script>
