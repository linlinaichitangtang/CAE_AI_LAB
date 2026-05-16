/**
 * ReportGeneratorPanel.vue — V4.1-002 报告生成面板
 * 一键生成 PDF/Word/Markdown 格式的仿真分析报告
 */
<template>
  <div class="report-generator-panel">
    <div class="panel-header">
      <h3 class="panel-title">📑 报告生成</h3>
    </div>

    <div class="report-form">
      <div class="form-row">
        <label>项目名称</label>
        <input v-model="form.projectName" type="text" placeholder="输入项目名称">
      </div>

      <div class="form-row">
        <label>仿真名称</label>
        <input v-model="form.simulationName" type="text" placeholder="输入仿真名称">
      </div>

      <div class="form-row">
        <label>报告人</label>
        <input v-model="form.author" type="text" placeholder="输入报告人姓名">
      </div>

      <div class="form-row">
        <label>包含合规检查</label>
        <input v-model="form.includeCompliance" type="checkbox">
      </div>
    </div>

    <div class="export-actions">
      <button class="btn-export pdf" @click="exportPDF" :disabled="isGenerating">
        📄 导出 PDF
      </button>
      <button class="btn-export md" @click="exportMarkdown" :disabled="isGenerating">
        📝 导出 Markdown
      </button>
      <button class="btn-export html" @click="exportHTML" :disabled="isGenerating">
        🌐 导出 HTML
      </button>
    </div>

    <div v-if="isGenerating" class="generating-state">
      <span class="spinner">⏳</span>
      <span>正在生成报告...</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useComplianceCheck } from '@/composables/useComplianceCheck'

const props = defineProps<{
  simulationResults?: {
    maxStress?: number
    maxDisplacement?: number
    safetyFactor?: number
    meshElements?: number
    meshNodes?: number
    materialName?: string
    analysisType?: string
  }
}>()

const { lastReport: complianceReport } = useComplianceCheck()
const isGenerating = ref(false)

const form = reactive({
  projectName: 'CAELab 仿真项目',
  simulationName: '结构静力分析',
  author: '工程师',
  includeCompliance: true
})

function buildReportData() {
  return {
    projectName: form.projectName,
    simulationName: form.simulationName,
    author: form.author,
    date: new Date().toLocaleDateString('zh-CN'),
    results: props.simulationResults || {},
    complianceReport: form.includeCompliance && complianceReport.value
      ? {
          overallStatus: complianceReport.value.overallStatus,
          results: complianceReport.value.results
        }
      : undefined
  }
}

function generateHTMLContent(): string {
  const data = buildReportData()
  const r = data.results

  let html = `<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<title>${data.projectName} — 仿真分析报告</title>
<style>
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; max-width: 800px; margin: 40px auto; padding: 20px; color: #1e293b; }
h1 { font-size: 24px; border-bottom: 2px solid #3b82f6; padding-bottom: 10px; }
h2 { font-size: 18px; color: #3b82f6; margin-top: 30px; }
table { width: 100%; border-collapse: collapse; margin: 16px 0; }
th, td { padding: 10px; border: 1px solid #e2e8f0; text-align: left; }
th { background: #f8fafc; font-weight: 600; }
.pass { background: #dcfce7; color: #166534; }
.fail { background: #fee2e2; color: #991b1b; }
.partial { background: #fef3c7; color: #92400e; }
.footer { margin-top: 40px; padding-top: 20px; border-top: 1px solid #e2e8f0; font-size: 12px; color: #94a3b8; }
</style>
</head>
<body>
<h1>${data.projectName}</h1>
<p><strong>仿真名称：</strong>${data.simulationName}</p>
<p><strong>报告人：</strong>${data.author}</p>
<p><strong>日期：</strong>${data.date}</p>

<h2>1. 模型信息</h2>
<table>
<tr><th>参数</th><th>数值</th></tr>
<tr><td>分析类型</td><td>${r.analysisType || '静力分析'}</td></tr>
<tr><td>材料</td><td>${r.materialName || '-'}</td></tr>
<tr><td>网格单元数</td><td>${r.meshElements?.toLocaleString() || '-'}</td></tr>
<tr><td>网格节点数</td><td>${r.meshNodes?.toLocaleString() || '-'}</td></tr>
</table>

<h2>2. 仿真结果</h2>
<table>
<tr><th>结果项</th><th>数值</th><th>单位</th></tr>
<tr><td>最大应力</td><td>${r.maxStress?.toFixed(2) || '-'}</td><td>MPa</td></tr>
<tr><td>最大位移</td><td>${r.maxDisplacement?.toFixed(4) || '-'}</td><td>mm</td></tr>
<tr><td>安全系数</td><td>${r.safetyFactor?.toFixed(2) || '-'}</td><td>-</td></tr>
</table>`

  if (data.complianceReport) {
    const statusClass = data.complianceReport.overallStatus
    const statusText = data.complianceReport.overallStatus === 'pass' ? '通过' : data.complianceReport.overallStatus === 'fail' ? '未通过' : '部分通过'
    html += `
<h2>3. 合规检查</h2>
<p>总体状态：<span class="${statusClass}">${statusText}</span></p>
<table>
<tr><th>标准</th><th>条款</th><th>实际值</th><th>结果</th></tr>`
    for (const result of data.complianceReport.results) {
      html += `<tr><td>${result.rule.standardName}</td><td>${result.rule.clause}</td><td>${result.actualValue.toFixed(2)}</td><td class="${result.passed ? 'pass' : 'fail'}">${result.passed ? '通过' : '未通过'}</td></tr>`
    }
    html += `</table>`
  }

  html += `
<div class="footer">
<p>本报告由 CAELab 自动生成</p>
<p>报告ID：RPT-${Date.now()}</p>
</div>
</body>
</html>`

  return html
}

function generateMarkdown(): string {
  const data = buildReportData()
  const r = data.results

  let md = `# ${data.projectName} — 仿真分析报告\n\n`
  md += `**仿真名称**: ${data.simulationName}\n\n`
  md += `**报告人**: ${data.author}\n\n`
  md += `**日期**: ${data.date}\n\n`
  md += `---\n\n`
  md += `## 1. 模型信息\n\n`
  md += `| 参数 | 数值 |\n`
  md += `|------|------|\n`
  md += `| 分析类型 | ${r.analysisType || '静力分析'} |\n`
  md += `| 材料 | ${r.materialName || '-'} |\n`
  md += `| 网格单元数 | ${r.meshElements?.toLocaleString() || '-'} |\n`
  md += `| 网格节点数 | ${r.meshNodes?.toLocaleString() || '-'} |\n\n`
  md += `## 2. 仿真结果\n\n`
  md += `| 结果项 | 数值 | 单位 |\n`
  md += `|--------|------|------|\n`
  md += `| 最大应力 | ${r.maxStress?.toFixed(2) || '-'} | MPa |\n`
  md += `| 最大位移 | ${r.maxDisplacement?.toFixed(4) || '-'} | mm |\n`
  md += `| 安全系数 | ${r.safetyFactor?.toFixed(2) || '-'} | - |\n\n`

  if (data.complianceReport) {
    md += `## 3. 合规检查\n\n`
    md += `**总体状态**: ${data.complianceReport.overallStatus}\n\n`
    md += `| 标准 | 条款 | 实际值 | 结果 |\n`
    md += `|------|------|--------|------|\n`
    for (const result of data.complianceReport.results) {
      md += `| ${result.rule.standardName} | ${result.rule.clause} | ${result.actualValue.toFixed(2)} | ${result.passed ? '通过' : '未通过'} |\n`
    }
    md += `\n`
  }

  md += `---\n\n*本报告由 CAELab 自动生成*\n`
  return md
}

async function exportPDF() {
  isGenerating.value = true
  try {
    const html = generateHTMLContent()
    const printWindow = window.open('', '_blank')
    if (!printWindow) {
      alert('请允许弹窗以生成报告')
      return
    }
    printWindow.document.write(html)
    printWindow.document.close()
    setTimeout(() => {
      printWindow.print()
    }, 500)
  } finally {
    isGenerating.value = false
  }
}

async function exportMarkdown() {
  isGenerating.value = true
  try {
    const md = generateMarkdown()
    const blob = new Blob([md], { type: 'text/markdown' })
    const link = document.createElement('a')
    link.download = `${form.projectName}_report_${Date.now()}.md`
    link.href = URL.createObjectURL(blob)
    link.click()
  } finally {
    isGenerating.value = false
  }
}

async function exportHTML() {
  isGenerating.value = true
  try {
    const html = generateHTMLContent()
    const blob = new Blob([html], { type: 'text/html' })
    const link = document.createElement('a')
    link.download = `${form.projectName}_report_${Date.now()}.html`
    link.href = URL.createObjectURL(blob)
    link.click()
  } finally {
    isGenerating.value = false
  }
}
</script>

<style scoped>
.report-generator-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 20px;
  background: white;
  border-radius: 12px;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.panel-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
  margin: 0;
}

.report-form {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.form-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.form-row label {
  font-size: 13px;
  color: var(--text-secondary, #64748b);
  min-width: 80px;
}

.form-row input[type="text"] {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--border-color, #e2e8f0);
  border-radius: 6px;
  font-size: 13px;
}

.form-row input[type="checkbox"] {
  width: 18px;
  height: 18px;
  accent-color: var(--primary-color, #3b82f6);
}

.export-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.btn-export {
  padding: 10px 20px;
  border-radius: 8px;
  border: none;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-export.pdf {
  background: #ef4444;
  color: white;
}

.btn-export.md {
  background: #374151;
  color: white;
}

.btn-export.html {
  background: #3b82f6;
  color: white;
}

.btn-export:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.generating-state {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 20px;
  color: var(--text-secondary, #64748b);
}

.spinner {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
