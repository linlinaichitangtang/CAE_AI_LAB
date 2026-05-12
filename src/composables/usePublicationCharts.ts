/**
 * usePublicationCharts.ts — V3.2-002 论文级图表导出
 * 矢量图、300dpi、Nature/Science/Elsevier 格式模板
 */

import { ref } from 'vue'

// ============ 类型定义 ============

export type JournalFormat = 'nature' | 'science' | 'elsevier' | 'ieee' | 'springer' | 'wiley' | 'custom'

export interface ChartConfig {
  title: string
  subtitle?: string
  xLabel: string
  yLabel: string
  legendPosition: 'top' | 'bottom' | 'left' | 'right' | 'none'
  showGrid: boolean
  showLegend: boolean
  fontSize: {
    title: number
    label: number
    tick: number
    legend: number
  }
  colors: string[]
  lineWidth: number
  markerSize: number
}

export interface ExportOptions {
  format: 'svg' | 'png' | 'pdf' | 'eps'
  dpi: number
  width: number  // mm
  height: number // mm
  journal: JournalFormat
  background: 'white' | 'transparent' | 'auto'
  fontFamily: string
}

export interface ChartDataSeries {
  name: string
  data: Array<{ x: number; y: number }>
  color?: string
  lineStyle?: 'solid' | 'dashed' | 'dotted'
  marker?: 'circle' | 'square' | 'triangle' | 'diamond' | 'none'
}

// ============ 期刊格式预设 ============

export const JOURNAL_PRESETS: Record<JournalFormat, {
  name: string
  fontFamily: string
  fontSize: { title: number; label: number; tick: number; legend: number }
  colors: string[]
  lineWidth: number
  markerSize: number
  requirements: string
}> = {
  nature: {
    name: 'Nature',
    fontFamily: 'Arial, sans-serif',
    fontSize: { title: 9, label: 8, tick: 7, legend: 7 },
    colors: ['#000000', '#3366CC', '#DC3912', '#FF9900', '#109618', '#990099', '#0099C6', '#DD4477'],
    lineWidth: 0.75,
    markerSize: 3,
    requirements: '单栏 89mm, 双栏 183mm, 字体 Arial'
  },
  science: {
    name: 'Science',
    fontFamily: 'Arial, sans-serif',
    fontSize: { title: 10, label: 9, tick: 8, legend: 8 },
    colors: ['#000000', '#E41A1C', '#377EB8', '#4DAF4A', '#984EA3', '#FF7F00', '#FFFF33', '#A65628'],
    lineWidth: 1,
    markerSize: 4,
    requirements: '单栏 56mm, 双栏 120mm, 字体 Arial'
  },
  elsevier: {
    name: 'Elsevier',
    fontFamily: 'Times New Roman, serif',
    fontSize: { title: 10, label: 9, tick: 8, legend: 8 },
    colors: ['#000000', '#003366', '#CC0000', '#006600', '#660099', '#FF6600', '#999999', '#336699'],
    lineWidth: 0.75,
    markerSize: 3,
    requirements: '单栏 80mm, 双栏 170mm, 字体 Times New Roman'
  },
  ieee: {
    name: 'IEEE',
    fontFamily: 'Times New Roman, serif',
    fontSize: { title: 8, label: 7, tick: 6, legend: 6 },
    colors: ['#000000', '#0000FF', '#FF0000', '#00AA00', '#AA00AA', '#FFAA00', '#880000', '#008800'],
    lineWidth: 0.5,
    markerSize: 2,
    requirements: '单栏 86mm, 双栏 183mm'
  },
  springer: {
    name: 'Springer',
    fontFamily: 'Times New Roman, serif',
    fontSize: { title: 9, label: 8, tick: 7, legend: 7 },
    colors: ['#000000', '#1F497D', '#C0504D', '#9BBB59', '#8064A2', '#4BACC6', '#F79646', '#C9C9C9'],
    lineWidth: 0.8,
    markerSize: 3,
    requirements: '单栏 84mm, 双栏 174mm'
  },
  wiley: {
    name: 'Wiley',
    fontFamily: 'Arial, sans-serif',
    fontSize: { title: 9, label: 8, tick: 7, legend: 7 },
    colors: ['#000000', '#990000', '#0066CC', '#009933', '#993300', '#660066', '#CC6600', '#333333'],
    lineWidth: 0.75,
    markerSize: 3,
    requirements: '单栏 89mm, 双栏 183mm'
  },
  custom: {
    name: '自定义',
    fontFamily: 'Arial, sans-serif',
    fontSize: { title: 10, label: 9, tick: 8, legend: 8 },
    colors: ['#000000', '#3366CC', '#DC3912', '#FF9900', '#109618', '#990099', '#0099C6', '#DD4477'],
    lineWidth: 1,
    markerSize: 4,
    requirements: '自定义格式'
  }
}

// ============ 默认图表配置 ============

export const DEFAULT_CHART_CONFIG: ChartConfig = {
  title: '',
  subtitle: '',
  xLabel: 'X',
  yLabel: 'Y',
  legendPosition: 'right',
  showGrid: true,
  showLegend: true,
  fontSize: { title: 10, label: 9, tick: 8, legend: 8 },
  colors: ['#000000', '#3366CC', '#DC3912', '#FF9900', '#109618', '#990099', '#0099C6', '#DD4477'],
  lineWidth: 1,
  markerSize: 4
}

// ============ 期刊尺寸预设 (mm) ============

export const JOURNAL_SIZES = {
  'nature-single': { width: 89, height: 60 },
  'nature-double': { width: 183, height: 80 },
  'science-single': { width: 56, height: 50 },
  'science-double': { width: 120, height: 80 },
  'elsevier-single': { width: 80, height: 55 },
  'elsevier-double': { width: 170, height: 100 },
  'ieee-single': { width: 86, height: 60 },
  'ieee-double': { width: 183, height: 100 },
  'a4-full': { width: 180, height: 120 },
  'slide-16:9': { width: 160, height: 90 },
  'slide-4:3': { width: 140, height: 105 }
}

// ============ 主 Composable ============

export function usePublicationCharts() {
  const currentConfig = ref<ChartConfig>({ ...DEFAULT_CHART_CONFIG })
  const currentJournal = ref<JournalFormat>('custom')
  const currentSize = ref<keyof typeof JOURNAL_SIZES>('a4-full')
  const exportFormat = ref<ExportOptions['format']>('svg')

  // 设置期刊格式
  function setJournal(journal: JournalFormat): void {
    currentJournal.value = journal
    const preset = JOURNAL_PRESETS[journal]
    currentConfig.value.fontSize = { ...preset.fontSize }
    currentConfig.value.colors = [...preset.colors]
    currentConfig.value.lineWidth = preset.lineWidth
    currentConfig.value.markerSize = preset.markerSize
  }

  // 设置尺寸
  function setSize(size: keyof typeof JOURNAL_SIZES): void {
    currentSize.value = size
  }

  // 更新配置
  function updateConfig(updates: Partial<ChartConfig>): void {
    Object.assign(currentConfig.value, updates)
  }

  // 生成 SVG
  function generateSVG(
    series: ChartDataSeries[],
    config: ChartConfig,
    width: number,
    height: number
  ): string {
    const { colors, fontSize, lineWidth, markerSize, showGrid, showLegend, legendPosition } = config

    // 计算坐标区域
    const margin = { top: 20, right: 20, bottom: 40, left: 50 }
    const plotWidth = width - margin.left - margin.right
    const plotHeight = height - margin.top - margin.bottom

    // 计算数据范围
    let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity
    for (const s of series) {
      for (const p of s.data) {
        minX = Math.min(minX, p.x)
        maxX = Math.max(maxX, p.x)
        minY = Math.min(minY, p.y)
        maxY = Math.max(maxY, p.y)
      }
    }

    // 添加边距
    const xRange = maxX - minX || 1
    const yRange = maxY - minY || 1
    minX -= xRange * 0.1
    maxX += xRange * 0.1
    minY -= yRange * 0.1
    maxY += yRange * 0.1

    // 坐标转换
    const scaleX = (x: number) => margin.left + ((x - minX) / (maxX - minX)) * plotWidth
    const scaleY = (y: number) => margin.top + plotHeight - ((y - minY) / (maxY - minY)) * plotHeight

    // 生成 SVG
    const lines: string[] = []

    // 背景
    if (config.background !== 'transparent') {
      lines.push(`<rect x="0" y="0" width="${width}" height="${height}" fill="white"/>`)
    }

    // 网格
    if (showGrid) {
      const gridLines = 5
      lines.push('<g stroke="#e0e0e0" stroke-width="0.5">')
      for (let i = 0; i <= gridLines; i++) {
        const y = margin.top + (i / gridLines) * plotHeight
        lines.push(`<line x1="${margin.left}" y1="${y}" x2="${width - margin.right}" y2="${y}"/>`)
      }
      for (let i = 0; i <= gridLines; i++) {
        const x = margin.left + (i / gridLines) * plotWidth
        lines.push(`<line x1="${x}" y1="${margin.top}" x2="${x}" y2="${height - margin.bottom}"/>`)
      }
      lines.push('</g>')
    }

    // 数据线
    for (let si = 0; si < series.length; si++) {
      const s = series[si]
      const color = s.color || colors[si % colors.length]

      // 线
      if (s.lineStyle !== 'none') {
        const strokeDash = s.lineStyle === 'dashed' ? 'stroke-dasharray="5,3"' : s.lineStyle === 'dotted' ? 'stroke-dasharray="2,2"' : ''
        lines.push(`<path d="${s.data.map((p, i) => `${i === 0 ? 'M' : 'L'} ${scaleX(p.x).toFixed(2)} ${scaleY(p.y).toFixed(2)}`).join(' ')}" fill="none" stroke="${color}" stroke-width="${lineWidth}" ${strokeDash}/>`)
      }

      // 标记
      if (s.marker && s.marker !== 'none') {
        const markerPath = (x: number, y: number) => {
          const size = markerSize
          switch (s.marker) {
            case 'circle': return `<circle cx="${x}" cy="${y}" r="${size / 2}"/>`
            case 'square': return `<rect x="${x - size / 2}" y="${y - size / 2}" width="${size}" height="${size}"/>`
            case 'triangle': return `<polygon points="${x},${y - size / 2} ${x - size / 2},${y + size / 2} ${x + size / 2},${y + size / 2}"/>`
            case 'diamond': return `<polygon points="${x},${y - size / 2} ${x + size / 2},${y} ${x},${y + size / 2} ${x - size / 2},${y}"/>`
            default: return ''
          }
        }
        lines.push(`<g fill="${color}">${s.data.map(p => markerPath(scaleX(p.x), scaleY(p.y))).join('')}</g>`)
      }
    }

    // 轴
    lines.push(`<g stroke="#000" stroke-width="0.5">`)
    lines.push(`<line x1="${margin.left}" y1="${height - margin.bottom}" x2="${width - margin.right}" y2="${height - margin.bottom}"/>`)
    lines.push(`<line x1="${margin.left}" y1="${margin.top}" x2="${margin.left}" y2="${height - margin.bottom}"/>`)
    lines.push('</g>')

    // 标题
    if (config.title) {
      lines.push(`<text x="${width / 2}" y="12" text-anchor="middle" font-family="${JOURNAL_PRESETS[currentJournal.value].fontFamily}" font-size="${fontSize.title}" font-weight="bold">${config.title}</text>`)
    }

    // 轴标签
    lines.push(`<text x="${width / 2}" y="${height - 5}" text-anchor="middle" font-family="${JOURNAL_PRESETS[currentJournal.value].fontFamily}" font-size="${fontSize.label}">${config.xLabel}</text>`)
    lines.push(`<text x="12" y="${height / 2}" text-anchor="middle" transform="rotate(-90, 12, ${height / 2})" font-family="${JOURNAL_PRESETS[currentJournal.value].fontFamily}" font-size="${fontSize.label}">${config.yLabel}</text>`)

    // 刻度
    lines.push(`<g font-family="${JOURNAL_PRESETS[currentJournal.value].fontFamily}" font-size="${fontSize.tick}" fill="#000">`)
    for (let i = 0; i <= 5; i++) {
      const xVal = minX + (i / 5) * (maxX - minX)
      const xPos = scaleX(xVal)
      lines.push(`<text x="${xPos}" y="${height - margin.bottom + 12}" text-anchor="middle">${xVal.toFixed(2)}</text>`)
    }
    for (let i = 0; i <= 5; i++) {
      const yVal = minY + (i / 5) * (maxY - minY)
      const yPos = scaleY(yVal)
      lines.push(`<text x="${margin.left - 8}" y="${yPos + 3}" text-anchor="end">${yVal.toFixed(2)}</text>`)
    }
    lines.push('</g>')

    // 图例
    if (showLegend && legendPosition !== 'none') {
      const legendX = legendPosition === 'right' ? width - margin.right - 20 : legendPosition === 'left' ? margin.left + 10 : width / 2 - (series.length * 30) / 2
      const legendY = legendPosition === 'top' ? margin.top : legendPosition === 'bottom' ? height - margin.bottom + 30 : height / 2

      lines.push(`<g font-family="${JOURNAL_PRESETS[currentJournal.value].fontFamily}" font-size="${fontSize.legend}" fill="#000">`)
      series.forEach((s, i) => {
        const color = s.color || colors[i % colors.length]
        const lx = legendX + i * 60
        lines.push(`<line x1="${lx}" y1="${legendY}" x2="${lx + 15}" y2="${legendY}" stroke="${color}" stroke-width="${lineWidth}"/>`)
        lines.push(`<text x="${lx + 20}" y="${legendY + 3}">${s.name}</text>`)
      })
      lines.push('</g>')
    }

    return `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="${width}mm" height="${height}mm" viewBox="0 0 ${width} ${height}">
${lines.join('\n')}
</svg>`
  }

  // 导出图表
  async function exportChart(
    series: ChartDataSeries[],
    options: ExportOptions
  ): Promise<Blob> {
    const config = { ...currentConfig.value, ...options }
    const size = JOURNAL_SIZES[options.width as keyof typeof JOURNAL_SIZES] || { width: options.width, height: options.height }

    let content: string

    if (options.format === 'svg') {
      content = generateSVG(series, config as ChartConfig, size.width, size.height)
      return new Blob([content], { type: 'image/svg+xml' })
    } else if (options.format === 'png' || options.format === 'pdf') {
      // PNG/PDF 需要通过 Canvas 转换
      const svg = generateSVG(series, config as ChartConfig, size.width, size.height)
      const canvas = document.createElement('canvas')
      const scale = options.dpi / 96 * 3.78 // 96 DPI 基准, 3.78 = 96/25.4
      canvas.width = size.width * scale
      canvas.height = size.height * scale

      const ctx = canvas.getContext('2d')!
      const img = new Image()

      return new Promise((resolve, reject) => {
        img.onload = () => {
          ctx.fillStyle = options.background === 'white' ? '#fff' : options.background === 'transparent' ? 'transparent' : '#fff'
          ctx.fillRect(0, 0, canvas.width, canvas.height)
          ctx.drawImage(img, 0, 0)
          canvas.toBlob(blob => {
            if (blob) resolve(blob)
            else reject(new Error('Failed to create blob'))
          }, options.format === 'png' ? 'image/png' : 'application/pdf')
        }
        img.onerror = reject
        img.src = 'data:image/svg+xml;base64,' + btoa(svg)
      })
    }

    throw new Error(`Unsupported format: ${options.format}`)
  }

  // 下载图表
  function downloadChart(series: ChartDataSeries[], filename: string, options: ExportOptions): void {
    exportChart(series, options).then(blob => {
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `${filename}.${options.format}`
      a.click()
      URL.revokeObjectURL(url)
    })
  }

  // 复制到剪贴板 (SVG)
  async function copyToClipboard(series: ChartDataSeries[]): Promise<boolean> {
    const svg = generateSVG(series, currentConfig.value, 180, 120)
    try {
      await navigator.clipboard.writeText(svg)
      return true
    } catch {
      return false
    }
  }

  // 生成数据表格 (CSV)
  function exportDataTable(series: ChartDataSeries[]): string {
    if (series.length === 0) return ''

    const headers = ['Index', ...series.map(s => s.name)]
    const rows: string[][] = []

    const maxLen = Math.max(...series.map(s => s.data.length))
    for (let i = 0; i < maxLen; i++) {
      const row = [String(i + 1)]
      for (const s of series) {
        row.push(s.data[i] ? `${s.data[i].x},${s.data[i].y}` : '')
      }
      rows.push(row)
    }

    return [headers.join(','), ...rows.map(r => r.join(','))].join('\n')
  }

  return {
    currentConfig,
    currentJournal,
    currentSize,
    exportFormat,

    setJournal,
    setSize,
    updateConfig,

    generateSVG,
    exportChart,
    downloadChart,
    copyToClipboard,
    exportDataTable,

    JOURNAL_PRESETS,
    JOURNAL_SIZES,
    DEFAULT_CHART_CONFIG
  }
}