/**
 * PDF 报告生成工具
 * 使用 jspdf 原生 API 生成 PDF（不依赖 html2canvas）
 */

export interface PdfReportOptions {
  title: string
  author?: string
  orientation?: 'portrait' | 'landscape'
  format?: 'a4' | 'letter'
}

interface ReportSection {
  heading?: string
  content: string
  table?: Array<{ headers: string[]; rows: string[][] }>
}

/**
 * 将 HTML 内容转换为纯文本段落和表格
 */
function parseHtmlToSections(html: string): ReportSection[] {
  const sections: ReportSection[] = []
  // 按 <h2>, <h3>, <table> 分割
  const parts = html.split(/(?=<h[23]|<table)/i)

  for (const part of parts) {
    if (!part.trim()) continue

    // 提取标题
    const h2Match = part.match(/<h2[^>]*>(.*?)<\/h2>/is)
    const h3Match = part.match(/<h3[^>]*>(.*?)<\/h3>/is)
    const heading = h2Match?.[1] || h3Match?.[1] || undefined

    // 提取表格
    const tableMatch = part.match(/<table[^>]*>(.*?)<\/table>/is)
    let table: Array<{ headers: string[]; rows: string[][] }> | undefined
    if (tableMatch) {
      const rows: string[][] = []
      const trMatches = tableMatch[1].matchAll(/<tr[^>]*>(.*?)<\/tr>/gis)
      for (const tr of trMatches) {
        const cells: string[] = []
        const cellMatches = tr[1].matchAll(/<t[hd][^>]*>(.*?)<\/t[hd]>/gis)
        for (const cell of cellMatches) {
          cells.push(cell[1].replace(/<[^>]*>/g, '').trim())
        }
        if (cells.length > 0) rows.push(cells)
      }
      if (rows.length > 0) {
        table = [{ headers: rows[0], rows: rows.slice(1) }]
      }
    }

    // 提取文本内容
    let content = part
      .replace(/<h[23][^>]*>.*?<\/h[23]>/gis, '')
      .replace(/<table[^>]*>.*?<\/table>/gis, '')
      .replace(/<li[^>]*>/gi, '• ')
      .replace(/<br\s*\/?>/gi, '\n')
      .replace(/<[^>]*>/g, '')
      .replace(/&lt;/g, '<')
      .replace(/&gt;/g, '>')
      .replace(/&amp;/g, '&')
      .replace(/&nbsp;/g, ' ')
      .replace(/&quot;/g, '"')
      .replace(/&#39;/g, "'")
      .replace(/\n{3,}/g, '\n\n')
      .trim()

    if (content || table) {
      sections.push({ heading, content: content || '', table })
    }
  }

  return sections
}

/**
 * 生成 PDF 报告
 */
export async function generatePdfReport(
  htmlContent: string,
  options: PdfReportOptions
): Promise<Blob> {
  const { default: jsPDF } = await import('jspdf')

  const orientation = options.orientation ?? 'portrait'
  const format = options.format ?? 'a4'

  const pdf = new jsPDF({
    orientation,
    unit: 'mm',
    format,
  })

  const pageWidth = pdf.internal.pageSize.getWidth()
  const pageHeight = pdf.internal.pageSize.getHeight()
  const margin = 15
  const contentWidth = pageWidth - margin * 2
  let y = margin

  // 辅助函数：检查是否需要换页
  const checkPageBreak = (needed: number) => {
    if (y + needed > pageHeight - margin) {
      pdf.addPage()
      y = margin
    }
  }

  // 标题
  pdf.setFont('helvetica', 'bold')
  pdf.setFontSize(18)
  checkPageBreak(15)
  pdf.text(options.title, pageWidth / 2, y, { align: 'center' })
  y += 8

  // 作者和日期
  pdf.setFont('helvetica', 'normal')
  pdf.setFontSize(10)
  pdf.setTextColor(100, 100, 100)
  const dateStr = new Date().toLocaleDateString('zh-CN')
  const authorStr = options.author ? `${options.author} | ` : ''
  pdf.text(`${authorStr}${dateStr}`, pageWidth / 2, y, { align: 'center' })
  y += 3
  pdf.setDrawColor(200, 200, 200)
  pdf.line(margin, y, pageWidth - margin, y)
  y += 8
  pdf.setTextColor(0, 0, 0)

  // 解析 HTML 内容
  const sections = parseHtmlToSections(htmlContent)

  for (const section of sections) {
    // 标题
    if (section.heading) {
      checkPageBreak(12)
      pdf.setFont('helvetica', 'bold')
      pdf.setFontSize(14)
      pdf.text(section.heading, margin, y)
      y += 7
    }

    // 文本内容
    if (section.content) {
      pdf.setFont('helvetica', 'normal')
      pdf.setFontSize(10)
      const lines = pdf.splitTextToSize(section.content, contentWidth)
      for (const line of lines) {
        checkPageBreak(5)
        pdf.text(line, margin, y)
        y += 4.5
      }
      y += 3
    }

    // 表格
    if (section.table) {
      for (const tbl of section.table) {
        const colCount = tbl.headers.length
        const colWidth = contentWidth / colCount

        // 表头
        checkPageBreak(8)
        pdf.setFillColor(37, 99, 235) // #2563EB
        pdf.setTextColor(255, 255, 255)
        pdf.setFont('helvetica', 'bold')
        pdf.setFontSize(9)
        pdf.rect(margin, y - 4, contentWidth, 7, 'F')
        for (let i = 0; i < tbl.headers.length; i++) {
          pdf.text(tbl.headers[i], margin + i * colWidth + 1.5, y)
        }
        y += 5

        // 表体
        pdf.setTextColor(0, 0, 0)
        pdf.setFont('helvetica', 'normal')
        pdf.setFontSize(8.5)
        for (let r = 0; r < tbl.rows.length; r++) {
          checkPageBreak(6)
          if (r % 2 === 1) {
            pdf.setFillColor(245, 247, 250)
            pdf.rect(margin, y - 3.5, contentWidth, 5, 'F')
          }
          for (let c = 0; c < tbl.rows[r].length; c++) {
            const cellText = tbl.rows[r][c].substring(0, 30) // 截断过长文本
            pdf.text(cellText, margin + c * colWidth + 1.5, y)
          }
          y += 4.5
        }
        y += 4
      }
    }
  }

  // 页脚
  const totalPages = pdf.getNumberOfPages()
  for (let i = 1; i <= totalPages; i++) {
    pdf.setPage(i)
    pdf.setFont('helvetica', 'normal')
    pdf.setFontSize(8)
    pdf.setTextColor(150, 150, 150)
    pdf.text(
      `CAELab - ${options.title} | Page ${i}/${totalPages}`,
      pageWidth / 2,
      pageHeight - 5,
      { align: 'center' }
    )
  }

  return pdf.output('blob')
}

/**
 * 下载 Blob 文件
 */
export function downloadBlob(blob: Blob, filename: string) {
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = filename
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
}
