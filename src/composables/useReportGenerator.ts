/**
 * useReportGenerator.ts — V3.4-001 一键报告 PPT/PDF
 * 客户/管理层汇报用，自动生成专业报告
 */

import { ref } from 'vue'

// ============ 类型定义 ============

export type ReportFormat = 'pdf' | 'pptx' | 'html' | 'docx'
export type ReportStyle = 'professional' | 'academic' | 'executive' | 'compact'
export type SlideLayout = 'title' | 'content' | 'two_column' | 'image_left' | 'image_right' | 'full_image' | 'data_table'

export interface ReportProject {
  id: string
  name: string
  description?: string
  createdAt: string
  modifiedAt: string
}

export interface ReportSection {
  id: string
  title: string
  type: 'title' | 'summary' | 'geometry' | 'mesh' | 'results' | 'chart' | 'table' | 'image' | 'text' | 'conclusion'
  layout: SlideLayout
  content: SectionContent
  notes?: string  // 演讲者备注
}

export interface SectionContent {
  // 通用
  text?: string
  imageUrl?: string
  imageCaption?: string

  // 几何信息
  geometryInfo?: {
    type: string
    dimensions?: Record<string, number>
    volume?: number
    surfaceArea?: number
  }

  // 网格信息
  meshInfo?: {
    elementType: string
    nodeCount: number
    elementCount: number
    qualityMetric?: number
  }

  // 仿真结果
  results?: {
    analysisType: string
    maxDisplacement?: number
    maxStress?: number
    maxStrain?: number
    maxTemperature?: number
    firstFrequency?: number
    criticalLoad?: number
    dataPoints?: Array<{ label: string; value: number; unit: string }>
  }

  // 图表数据
  chartData?: {
    type: 'bar' | 'line' | 'pie' | 'scatter' | 'heatmap'
    title: string
    series: Array<{
      name: string
      data: Array<{ x: number; y: number } | number>
      color?: string
    }>
    xLabel?: string
    yLabel?: string
  }

  // 表格数据
  tableData?: {
    headers: string[]
    rows: string[][]
    caption?: string
  }
}

export interface ReportTemplate {
  id: string
  name: string
  description: string
  category: 'standard' | 'academic' | 'executive' | 'custom'
  sections: Omit<ReportSection, 'id'>[]
  style: ReportStyle
  logoUrl?: string
  companyName?: string
  accentColor?: string
}

export interface ReportGenerationOptions {
  format: ReportFormat
  template?: string
  style?: ReportStyle
  includeSections?: string[]
  language?: 'zh' | 'en'
  pageSize?: 'A4' | 'Letter' | 'Legal'
  orientation?: 'portrait' | 'landscape'
  showPageNumbers?: boolean
  showDate?: boolean
  showLogo?: boolean
}

export interface GeneratedReport {
  id: string
  name: string
  format: ReportFormat
  createdAt: string
  fileSize?: number
  downloadUrl?: string
  previewUrl?: string
}

export interface ReportProgress {
  stage: 'preparing' | 'generating' | 'finalizing' | 'complete' | 'error'
  progress: number  // 0-100
  currentSection?: string
  errorMessage?: string
}

// ============ 预设模板 ============

export const REPORT_TEMPLATES: ReportTemplate[] = [
  {
    id: 'standard',
    name: '标准仿真报告',
    description: '包含完整仿真流程的几何、网格、结果和分析',
    category: 'standard',
    style: 'professional',
    sections: [
      { title: '封面', type: 'title', layout: 'title', content: {} },
      { title: '摘要', type: 'summary', layout: 'content', content: {} },
      { title: '几何模型', type: 'geometry', layout: 'image_left', content: {} },
      { title: '网格划分', type: 'mesh', layout: 'two_column', content: {} },
      { title: '仿真结果', type: 'results', layout: 'content', content: {} },
      { title: '结果分析', type: 'chart', layout: 'content', content: {} },
      { title: '结论', type: 'conclusion', layout: 'content', content: {} }
    ]
  },
  {
    id: 'executive',
    name: '管理层汇报',
    description: '简洁扼要，适合管理层快速了解项目概况',
    category: 'executive',
    style: 'executive',
    sections: [
      { title: '封面', type: 'title', layout: 'title', content: {} },
      { title: '项目概况', type: 'summary', layout: 'content', content: {} },
      { title: '关键结果', type: 'results', layout: 'data_table', content: {} },
      { title: '结论与建议', type: 'conclusion', layout: 'content', content: {} }
    ]
  },
  {
    id: 'academic',
    name: '学术论文格式',
    description: '符合学术规范的仿真报告格式',
    category: 'academic',
    style: 'academic',
    sections: [
      { title: '标题页', type: 'title', layout: 'title', content: {} },
      { title: '摘要', type: 'summary', layout: 'content', content: {} },
      { title: '引言', type: 'text', layout: 'content', content: {} },
      { title: '方法', type: 'geometry', layout: 'two_column', content: {} },
      { title: '结果', type: 'results', layout: 'content', content: {} },
      { title: '图表', type: 'chart', layout: 'content', content: {} },
      { title: '讨论', type: 'text', layout: 'content', content: {} },
      { title: '结论', type: 'conclusion', layout: 'content', content: {} }
    ]
  },
  {
    id: 'comparison',
    name: '对比分析报告',
    description: '多个设计方案或工况的对比分析',
    category: 'standard',
    style: 'professional',
    sections: [
      { title: '封面', type: 'title', layout: 'title', content: {} },
      { title: '概述', type: 'summary', layout: 'content', content: {} },
      { title: '方案对比', type: 'chart', layout: 'two_column', content: {} },
      { title: '数据对比', type: 'table', layout: 'data_table', content: {} },
      { title: '结论', type: 'conclusion', layout: 'content', content: {} }
    ]
  }
]

// ============ 样式预设 ============

export const REPORT_STYLES: Record<ReportStyle, {
  name: string
  fontFamily: string
  headingColor: string
  accentColor: string
  backgroundColor: string
  textColor: string
  borderColor: string
}> = {
  professional: {
    name: '专业风格',
    fontFamily: 'Inter, Arial, sans-serif',
    headingColor: '#1a365d',
    accentColor: '#2563eb',
    backgroundColor: '#ffffff',
    textColor: '#374151',
    borderColor: '#e5e7eb'
  },
  academic: {
    name: '学术风格',
    fontFamily: 'Times New Roman, serif',
    headingColor: '#1e293b',
    accentColor: '#dc2626',
    backgroundColor: '#fefefe',
    textColor: '#334155',
    borderColor: '#cbd5e1'
  },
  executive: {
    name: '商务风格',
    fontFamily: 'Arial, sans-serif',
    headingColor: '#0f172a',
    accentColor: '#059669',
    backgroundColor: '#f8fafc',
    textColor: '#475569',
    borderColor: '#e2e8f0'
  },
  compact: {
    name: '紧凑风格',
    fontFamily: 'Arial, sans-serif',
    headingColor: '#1e293b',
    accentColor: '#7c3aed',
    backgroundColor: '#ffffff',
    textColor: '#334155',
    borderColor: '#e2e8f0'
  }
}

// ============ 存储键 ============

const TEMPLATES_KEY = 'caelab_report_templates'
const PROJECTS_KEY = 'caelab_report_projects'
const GENERATED_KEY = 'caelab_generated_reports'

// ============ 工具函数 ============

function generateId(): string {
  return `report_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

// ============ 主 Composable ============

export function useReportGenerator() {
  const templates = ref<ReportTemplate[]>([])
  const projects = ref<ReportProject[]>([])
  const generatedReports = ref<GeneratedReport[]>([])

  const generationProgress = ref<ReportProgress>({
    stage: 'preparing',
    progress: 0
  })

  // ============ 初始化 ============

  function loadData(): void {
    const storedTemplates = getStorage<ReportTemplate[]>(TEMPLATES_KEY)
    templates.value = storedTemplates && storedTemplates.length > 0
      ? storedTemplates
      : REPORT_TEMPLATES

    const storedProjects = getStorage<ReportProject[]>(PROJECTS_KEY)
    if (storedProjects) projects.value = storedProjects

    const storedReports = getStorage<GeneratedReport[]>(GENERATED_KEY)
    if (storedReports) generatedReports.value = storedReports
  }

  // ============ 模板管理 ============

  function createTemplate(template: Omit<ReportTemplate, 'id'>): ReportTemplate {
    const newTemplate: ReportTemplate = {
      ...template,
      id: generateId()
    }
    templates.value.push(newTemplate)
    saveTemplates()
    return newTemplate
  }

  function updateTemplate(templateId: string, updates: Partial<ReportTemplate>): boolean {
    const template = templates.value.find(t => t.id === templateId)
    if (!template) return false
    Object.assign(template, updates)
    saveTemplates()
    return true
  }

  function deleteTemplate(templateId: string): boolean {
    const index = templates.value.findIndex(t => t.id === templateId)
    if (index === -1) return false
    templates.value.splice(index, 1)
    saveTemplates()
    return true
  }

  function getTemplate(templateId: string): ReportTemplate | undefined {
    return templates.value.find(t => t.id === templateId)
  }

  function cloneTemplate(templateId: string, newName: string): ReportTemplate | null {
    const template = templates.value.find(t => t.id === templateId)
    if (!template) return null

    return createTemplate({
      ...template,
      name: newName,
      category: 'custom',
      sections: template.sections.map(s => ({ ...s }))
    })
  }

  // ============ 报告项目管理 ============

  function createReportProject(name: string, description?: string): ReportProject {
    const project: ReportProject = {
      id: generateId(),
      name,
      description,
      createdAt: new Date().toISOString(),
      modifiedAt: new Date().toISOString()
    }
    projects.value.unshift(project)
    saveProjects()
    return project
  }

  function updateReportProject(projectId: string, updates: Partial<ReportProject>): boolean {
    const project = projects.value.find(p => p.id === projectId)
    if (!project) return false
    Object.assign(project, updates, { modifiedAt: new Date().toISOString() })
    saveProjects()
    return true
  }

  function deleteReportProject(projectId: string): boolean {
    const index = projects.value.findIndex(p => p.id === projectId)
    if (index === -1) return false
    projects.value.splice(index, 1)
    saveProjects()
    return true
  }

  // ============ 报告生成 ============

  /**
   * 生成 PDF 报告
   */
  async function generatePDF(
    sections: ReportSection[],
    options: ReportGenerationOptions
  ): Promise<Blob> {
    generationProgress.value = { stage: 'generating', progress: 0 }

    // 模拟生成过程
    const totalSections = sections.length

    for (let i = 0; i < totalSections; i++) {
      await new Promise(resolve => setTimeout(resolve, 200))
      generationProgress.value = {
        stage: 'generating',
        progress: Math.round(((i + 1) / totalSections) * 80),
        currentSection: sections[i].title
      }
    }

    generationProgress.value = { stage: 'finalizing', progress: 90 }

    // 生成 HTML 内容（简化版，实际应该用 jsPDF）
    const html = generateHTML(sections, options)

    generationProgress.value = { stage: 'complete', progress: 100 }

    return new Blob([html], { type: 'text/html' })
  }

  /**
   * 生成 PPTX 报告（简化实现）
   */
  async function generatePPTX(
    sections: ReportSection[],
    options: ReportGenerationOptions
  ): Promise<Blob> {
    generationProgress.value = { stage: 'generating', progress: 0 }

    const totalSections = sections.length

    for (let i = 0; i < totalSections; i++) {
      await new Promise(resolve => setTimeout(resolve, 150))
      generationProgress.value = {
        stage: 'generating',
        progress: Math.round(((i + 1) / totalSections) * 80),
        currentSection: sections[i].title
      }
    }

    generationProgress.value = { stage: 'complete', progress: 100 }

    // 返回模拟的 PPTX（实际应该用 pptxgenjs）
    const content = generatePPTXContent(sections, options)
    return new Blob([content], { type: 'application/vnd.openxmlformats-officedocument.presentationml.presentation' })
  }

  /**
   * 生成 HTML 报告
   */
  async function generateHTMLReport(
    sections: ReportSection[],
    options: ReportGenerationOptions
  ): Promise<Blob> {
    generationProgress.value = { stage: 'generating', progress: 50 }
    const html = generateHTML(sections, options)
    generationProgress.value = { stage: 'complete', progress: 100 }
    return new Blob([html], { type: 'text/html' })
  }

  /**
   * 生成 DOCX 报告（简化实现）
   */
  async function generateDOCX(
    sections: ReportSection[],
    options: ReportGenerationOptions
  ): Promise<Blob> {
    generationProgress.value = { stage: 'generating', progress: 50 }
    // 实际应该用 docx library
    const html = generateHTML(sections, options)
    generationProgress.value = { stage: 'complete', progress: 100 }
    return new Blob([html], { type: 'text/html' })
  }

  /**
   * 根据选项生成报告
   */
  async function generateReport(
    sections: ReportSection[],
    options: ReportGenerationOptions,
    filename?: string
  ): Promise<GeneratedReport> {
    let blob: Blob

    switch (options.format) {
      case 'pdf':
        blob = await generatePDF(sections, options)
        break
      case 'pptx':
        blob = await generatePPTX(sections, options)
        break
      case 'docx':
        blob = await generateDOCX(sections, options)
        break
      case 'html':
      default:
        blob = await generateHTMLReport(sections, options)
        break
    }

    const report: GeneratedReport = {
      id: generateId(),
      name: filename || `Report_${new Date().toISOString().slice(0, 10)}`,
      format: options.format,
      createdAt: new Date().toISOString(),
      fileSize: blob.size,
      downloadUrl: URL.createObjectURL(blob)
    }

    generatedReports.value.unshift(report)
    saveGeneratedReports()

    return report
  }

  // ============ HTML 生成器 ============

  function generateHTML(sections: ReportSection[], options: ReportGenerationOptions): string {
    const style = REPORT_STYLES[options.style || 'professional']
    const pageSize = options.pageSize || 'A4'
    const orientation = options.orientation || 'portrait'

    const pageStyles: Record<string, string> = {
      A4: orientation === 'landscape' ? '297mm' : '210mm',
      Letter: orientation === 'landscape' ? '11in' : '8.5in',
      Legal: orientation === 'landscape' ? '14in' : '8.5in'
    }

    const pageWidth = pageStyles[pageSize]

    let html = `<!DOCTYPE html>
<html lang="${options.language || 'zh'}">
<head>
  <meta charset="UTF-8">
  <title>${sections.find(s => s.type === 'title')?.content?.text || 'Simulation Report'}</title>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body {
      font-family: ${style.fontFamily};
      color: ${style.textColor};
      background: ${style.backgroundColor};
      line-height: 1.6;
    }
    .page {
      width: ${pageWidth};
      min-height: ${orientation === 'landscape' ? '210mm' : '297mm'};
      margin: 0 auto;
      padding: 20mm;
      page-break-after: always;
      border-bottom: 1px solid ${style.borderColor};
    }
    @media print {
      .page { border: none; }
    }
    h1 { font-size: 28px; color: ${style.headingColor}; margin-bottom: 20px; }
    h2 { font-size: 22px; color: ${style.headingColor}; margin: 20px 0 15px; border-bottom: 2px solid ${style.accentColor}; padding-bottom: 8px; }
    h3 { font-size: 16px; color: ${style.headingColor}; margin: 15px 0 10px; }
    p { margin-bottom: 12px; }
    .cover { text-align: center; padding-top: 80px; }
    .cover h1 { font-size: 36px; margin-bottom: 30px; }
    .cover .subtitle { font-size: 18px; color: #666; margin-bottom: 60px; }
    .cover .meta { font-size: 14px; color: #999; margin-top: 100px; }
    .summary-box { background: #f8f9fa; padding: 20px; border-left: 4px solid ${style.accentColor}; margin: 20px 0; }
    .info-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 15px; margin: 20px 0; }
    .info-item { background: #f8f9fa; padding: 12px; border-radius: 4px; }
    .info-item .label { font-size: 12px; color: #666; margin-bottom: 4px; }
    .info-item .value { font-size: 16px; font-weight: bold; color: ${style.headingColor}; }
    .chart-container { margin: 20px 0; text-align: center; }
    .chart-container img { max-width: 100%; height: auto; }
    .data-table { width: 100%; border-collapse: collapse; margin: 20px 0; }
    .data-table th { background: ${style.headingColor}; color: white; padding: 12px; text-align: left; }
    .data-table td { padding: 10px 12px; border-bottom: 1px solid ${style.borderColor}; }
    .data-table tr:hover { background: #f8f9fa; }
    .two-column { display: grid; grid-template-columns: 1fr 1fr; gap: 30px; }
    .image-caption { font-size: 12px; color: #666; text-align: center; margin-top: 8px; }
    .page-number { text-align: right; font-size: 12px; color: #999; margin-top: 20px; }
    .conclusion { background: linear-gradient(135deg, ${style.accentColor}15, ${style.accentColor}05); padding: 20px; border-radius: 8px; margin: 20px 0; }
  </style>
</head>
<body>
`

    for (let i = 0; i < sections.length; i++) {
      const section = sections[i]
      const pageNum = options.showPageNumbers ? `<div class="page-number">${i + 1}</div>` : ''

      switch (section.type) {
        case 'title':
          html += `  <div class="page cover">\n`
          html += `    <h1>${section.content.text || 'Simulation Report'}</h1>\n`
          if (options.showDate) {
            html += `    <div class="meta">${new Date().toLocaleDateString()}</div>\n`
          }
          html += `    ${pageNum}\n`
          html += `  </div>\n`
          break

        case 'summary':
          html += `  <div class="page">\n`
          html += `    <h2>${section.title}</h2>\n`
          html += `    <div class="summary-box">${section.content.text || 'Summary content'}</div>\n`
          html += `    ${pageNum}\n`
          html += `  </div>\n`
          break

        case 'geometry':
        case 'mesh':
          html += `  <div class="page">\n`
          html += `    <h2>${section.title}</h2>\n`
          html += `    <div class="info-grid">\n`
          if (section.content.geometryInfo) {
            html += `      <div class="info-item"><div class="label">类型</div><div class="value">${section.content.geometryInfo.type}</div></div>\n`
            if (section.content.geometryInfo.volume) {
              html += `      <div class="info-item"><div class="label">体积</div><div class="value">${section.content.geometryInfo.volume.toFixed(2)} mm³</div></div>\n`
            }
          }
          if (section.content.meshInfo) {
            html += `      <div class="info-item"><div class="label">单元类型</div><div class="value">${section.content.meshInfo.elementType}</div></div>\n`
            html += `      <div class="info-item"><div class="label">节点数</div><div class="value">${section.content.meshInfo.nodeCount.toLocaleString()}</div></div>\n`
            html += `      <div class="info-item"><div class="label">单元数</div><div class="value">${section.content.meshInfo.elementCount.toLocaleString()}</div></div>\n`
          }
          html += `    </div>\n`
          if (section.content.text) {
            html += `    <p>${section.content.text}</p>\n`
          }
          html += `    ${pageNum}\n`
          html += `  </div>\n`
          break

        case 'results':
          html += `  <div class="page">\n`
          html += `    <h2>${section.title}</h2>\n`
          html += `    <div class="info-grid">\n`
          if (section.content.results) {
            const r = section.content.results
            if (r.maxDisplacement !== undefined) {
              html += `      <div class="info-item"><div class="label">最大位移</div><div class="value">${r.maxDisplacement.toFixed(4)} mm</div></div>\n`
            }
            if (r.maxStress !== undefined) {
              html += `      <div class="info-item"><div class="label">最大应力</div><div class="value">${r.maxStress.toFixed(2)} MPa</div></div>\n`
            }
            if (r.maxStrain !== undefined) {
              html += `      <div class="info-item"><div class="label">最大应变</div><div class="value">${r.maxStrain.toFixed(6)}</div></div>\n`
            }
            if (r.firstFrequency !== undefined) {
              html += `      <div class="info-item"><div class="label">第一阶频率</div><div class="value">${r.firstFrequency.toFixed(2)} Hz</div></div>\n`
            }
          }
          html += `    </div>\n`
          html += `    ${pageNum}\n`
          html += `  </div>\n`
          break

        case 'table':
          if (section.content.tableData) {
            html += `  <div class="page">\n`
            html += `    <h2>${section.title}</h2>\n`
            html += `    <table class="data-table">\n`
            html += `      <thead><tr>${section.content.tableData.headers.map(h => `<th>${h}</th>`).join('')}</tr></thead>\n`
            html += `      <tbody>\n`
            for (const row of section.content.tableData.rows) {
              html += `        <tr>${row.map(cell => `<td>${cell}</td>`).join('')}</tr>\n`
            }
            html += `      </tbody>\n`
            html += `    </table>\n`
            if (section.content.tableData.caption) {
              html += `    <p class="image-caption">${section.content.tableData.caption}</p>\n`
            }
            html += `    ${pageNum}\n`
            html += `  </div>\n`
          }
          break

        case 'conclusion':
          html += `  <div class="page">\n`
          html += `    <h2>${section.title}</h2>\n`
          html += `    <div class="conclusion">${section.content.text || 'Conclusion content'}</div>\n`
          html += `    ${pageNum}\n`
          html += `  </div>\n`
          break

        default:
          html += `  <div class="page">\n`
          html += `    <h2>${section.title}</h2>\n`
          html += `    <p>${section.content.text || ''}</p>\n`
          html += `    ${pageNum}\n`
          html += `  </div>\n`
      }
    }

    html += `</body>\n</html>`
    return html
  }

  function generatePPTXContent(sections: ReportSection[], options: ReportGenerationOptions): string {
    // 简化的 PPTX XML 表示
    let xml = `<?xml version="1.0" encoding="UTF-8"?>
<pptx>
  <title>Simulation Report</title>
  <slides>
`

    for (const section of sections) {
      xml += `    <slide type="${section.type}" layout="${section.layout}">\n`
      xml += `      <title>${section.title}</title>\n`
      if (section.content.text) {
        xml += `      <content>${section.content.text}</content>\n`
      }
      xml += `    </slide>\n`
    }

    xml += `  </slides>\n`
    xml += `</pptx>`
    return xml
  }

  // ============ 下载和导出 ============

  function downloadReport(reportId: string): void {
    const report = generatedReports.value.find(r => r.id === reportId)
    if (!report || !report.downloadUrl) return

    const a = document.createElement('a')
    a.href = report.downloadUrl
    a.download = `${report.name}.${report.format}`
    a.click()
  }

  function deleteGeneratedReport(reportId: string): boolean {
    const index = generatedReports.value.findIndex(r => r.id === reportId)
    if (index === -1) return false

    const report = generatedReports.value[index]
    if (report.downloadUrl) {
      URL.revokeObjectURL(report.downloadUrl)
    }

    generatedReports.value.splice(index, 1)
    saveGeneratedReports()
    return true
  }

  // ============ 从项目创建报告 ============

  function createReportFromProject(
    project: {
      name: string
      geometry?: any
      mesh?: any
      results?: any
    },
    templateId?: string
  ): ReportSection[] {
    const template = templateId ? getTemplate(templateId) : templates.value[0]
    if (!template) return []

    return template.sections.map(section => {
      const newSection: ReportSection = {
        ...section,
        id: generateId(),
        content: { ...section.content }
      }

      switch (section.type) {
        case 'title':
          newSection.content.text = project.name
          break
        case 'geometry':
          if (project.geometry) {
            newSection.content.geometryInfo = {
              type: project.geometry.type || 'Unknown',
              dimensions: project.geometry.dimensions,
              volume: project.geometry.volume,
              surfaceArea: project.geometry.surfaceArea
            }
          }
          break
        case 'mesh':
          if (project.mesh) {
            newSection.content.meshInfo = {
              elementType: project.mesh.elementType || 'Unknown',
              nodeCount: project.mesh.nodeCount || 0,
              elementCount: project.mesh.elementCount || 0,
              qualityMetric: project.mesh.qualityMetric
            }
          }
          break
        case 'results':
          if (project.results) {
            newSection.content.results = project.results
          }
          break
      }

      return newSection
    })
  }

  // ============ 持久化 ============

  function saveTemplates(): void {
    setStorage(TEMPLATES_KEY, templates.value)
  }

  function saveProjects(): void {
    setStorage(PROJECTS_KEY, projects.value)
  }

  function saveGeneratedReports(): void {
    setStorage(GENERATED_KEY, generatedReports.value)
  }

  // 初始化
  loadData()

  return {
    // 状态
    templates,
    projects,
    generatedReports,
    generationProgress,

    // 模板管理
    createTemplate,
    updateTemplate,
    deleteTemplate,
    getTemplate,
    cloneTemplate,

    // 报告项目管理
    createReportProject,
    updateReportProject,
    deleteReportProject,

    // 报告生成
    generateReport,
    generatePDF,
    generatePPTX,
    generateHTMLReport,
    generateDOCX,

    // 下载导出
    downloadReport,
    deleteGeneratedReport,

    // 从项目创建
    createReportFromProject,

    // 数据
    REPORT_TEMPLATES,
    REPORT_STYLES,

    // ========== V4.1-002 新增：合规检查集成报告 ==========

    /**
     * 从仿真结果快速生成含合规检查的分析报告
     */
    generateComplianceReport,

    /**
     * 导出为 Markdown
     */
    exportMarkdown
  }
}

// ========== V4.1-002 合规检查报告生成 ==========

export interface ComplianceReportData {
  projectName: string
  simulationName: string
  author: string
  date: string
  results: {
    maxStress?: number
    maxDisplacement?: number
    safetyFactor?: number
    meshElements?: number
    meshNodes?: number
    materialName?: string
    analysisType?: string
  }
  complianceReport?: {
    overallStatus: string
    results: Array<{
      rule: { standardName: string; clause: string; description: string }
      actualValue: number
      passed: boolean
      margin: number
      message: string
    }>
  }
}

function generateComplianceReport(data: ComplianceReportData): string {
  const lines: string[] = [
    `# ${data.projectName} — 仿真分析报告`,
    '',
    `**仿真名称**: ${data.simulationName}`,
    `**报告人**: ${data.author}`,
    `**日期**: ${data.date}`,
    '',
    '---',
    '',
    '## 1. 项目概述',
    '',
    `本项目对 ${data.simulationName} 进行有限元仿真分析，评估结构在指定载荷条件下的力学响应。`,
    '',
    '## 2. 模型信息',
    '',
    '| 参数 | 数值 |',
    '|------|------|',
    `| 分析类型 | ${data.results.analysisType || '静力分析'} |`,
    `| 材料 | ${data.results.materialName || '结构钢'} |`,
    `| 网格单元数 | ${data.results.meshElements || '-'} |`,
    `| 网格节点数 | ${data.results.meshNodes || '-'} |`,
    '',
    '## 3. 仿真结果',
    '',
    '| 结果项 | 数值 | 单位 |',
    '|--------|------|------|',
    `| 最大 von Mises 应力 | ${data.results.maxStress ? data.results.maxStress.toFixed(2) : '-'} | MPa |`,
    `| 最大位移 | ${data.results.maxDisplacement ? (data.results.maxDisplacement * 1000).toFixed(3) : '-'} | mm |`,
    `| 安全系数 | ${data.results.safetyFactor ? data.results.safetyFactor.toFixed(2) : '-'} | - |`,
    ''
  ]

  if (data.complianceReport) {
    lines.push('## 4. 合规检查')
    lines.push('')
    const status = data.complianceReport.overallStatus
    lines.push(`**总体状态**: ${status === 'pass' ? '✅ 通过' : status === 'fail' ? '❌ 未通过' : '⚠️ 部分通过'}`)
    lines.push('')
    for (const r of data.complianceReport.results) {
      lines.push(`- ${r.passed ? '✅' : '❌'} **${r.rule.standardName} 条款 ${r.rule.clause}**`)
      lines.push(`  - ${r.message}`)
      lines.push(`  - 实际值: ${r.actualValue.toFixed(2)}, 裕度: ${(r.margin * 100).toFixed(1)}%`)
      lines.push('')
    }
    lines.push('## 5. 结论')
  } else {
    lines.push('## 4. 结论')
  }

  lines.push('')
  lines.push(data.complianceReport
    ? `仿真分析已完成。合规检查总体状态：${data.complianceReport.overallStatus === 'pass' ? '通过' : data.complianceReport.overallStatus === 'fail' ? '未通过' : '部分通过'}。详见上文。`
    : '仿真分析已完成，结果已汇总。')
  lines.push('')
  lines.push('---')
  lines.push('')
  lines.push('*本报告由 CAELab 自动生成 | 仅供工程参考*')

  return lines.join('\n')
}

function exportMarkdown(data: ComplianceReportData): void {
  const md = generateComplianceReport(data)
  const blob = new Blob([md], { type: 'text/markdown' })
  const link = document.createElement('a')
  link.download = `${data.projectName}_report_${Date.now()}.md`
  link.href = URL.createObjectURL(blob)
  link.click()
}
