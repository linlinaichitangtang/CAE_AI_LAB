/**
 * useSimulationCitation.ts — V3.0-005 DOI-like 实验引用
 * 为每次仿真生成可引用的学术格式引用信息
 * 支持 BibTeX、RIS、JSON 等格式
 */

import { ref } from 'vue'
import { useProjectStore } from '@/stores/project'

// ============ 类型定义 ============

export interface SimulationCitation {
  id: string  // 仿真的唯一标识符（类似 DOI）
  title: string
  authors: string[]
  createdAt: string
  software: string
  softwareVersion: string
  simulationType: string  // static/modal/thermal/etc
  mesh: {
    nodes: number
    elements: number
    type: string
  }
  material: {
    name: string
    elasticModulus?: number
    poissonRatio?: number
    density?: number
  }
  boundaryConditions: {
    fixed: number
    loads: number
  }
  results: {
    maxDisplacement?: number
    maxStress?: number
    maxStrain?: number
    firstFrequency?: number
  }
  parameters: Record<string, any>
}

export interface CitationFormat {
  id: 'bibtex' | 'ris' | 'json' | 'text' | 'apa'
  name: string
  description: string
}

// ============ 常量 ============

const CITATION_FORMATS: CitationFormat[] = [
  { id: 'bibtex', name: 'BibTeX', description: 'LaTeX 参考文献格式' },
  { id: 'ris', name: 'RIS', description: 'EndNote/Zotero 导入格式' },
  { id: 'json', name: 'JSON', description: '机器可读的元数据格式' },
  { id: 'apa', name: 'APA', description: '美国心理学会引用格式' },
  { id: 'text', name: 'Plain Text', description: '纯文本引用格式' }
]

const SIMULATION_TYPE_NAMES: Record<string, string> = {
  'static': 'Static Structural Analysis',
  'modal': 'Modal Analysis',
  'buckling': 'Buckling Analysis',
  'thermal': 'Thermal Analysis',
  'transient': 'Transient Dynamics',
  'frequency': 'Frequency Response Analysis',
  'fatigue': 'Fatigue Analysis',
  'cfd': 'Computational Fluid Dynamics',
  'multiscale': 'Multi-scale Simulation'
}

// ============ 工具函数 ============

/**
 * 生成唯一标识符（类似 DOI）
 * 格式: CAE-{year}{month}{day}-{6位随机}
 */
function generateCitationId(): string {
  const now = new Date()
  const year = now.getFullYear()
  const month = String(now.getMonth() + 1).padStart(2, '0')
  const day = String(now.getDate()).padStart(2, '0')
  const random = Math.random().toString(36).substring(2, 8).toUpperCase()
  return `CAE-${year}${month}${day}-${random}`
}

/**
 * 获取默认作者列表
 */
function getDefaultAuthors(): string[] {
  const stored = localStorage.getItem('caelab_username')
  return stored ? [stored] : ['Anonymous']
}

// ============ 主函数 ============

/**
 * 从当前项目状态生成引用信息
 */
export function useSimulationCitation() {
  const projectStore = useProjectStore()
  const citations = ref<SimulationCitation[]>([])
  const currentCitation = ref<SimulationCitation | null>(null)

  /**
   * 创建当前仿真的引用信息
   */
  function createCitation(options?: {
    title?: string
    authors?: string[]
    simulationType?: string
  }): SimulationCitation {
    const citation: SimulationCitation = {
      id: generateCitationId(),
      title: options?.title || `CAELab Simulation ${new Date().toLocaleDateString()}`,
      authors: options?.authors || getDefaultAuthors(),
      createdAt: new Date().toISOString(),
      software: 'CAELab',
      softwareVersion: 'V2.9',
      simulationType: options?.simulationType || 'static',
      mesh: {
        nodes: projectStore.currentMesh?.nodes?.length || 0,
        elements: projectStore.currentMesh?.elements?.length || 0,
        type: 'Finite Element'
      },
      material: {
        name: 'Custom Material',
        elasticModulus: undefined,
        poissonRatio: undefined,
        density: undefined
      },
      boundaryConditions: {
        fixed: projectStore.boundaryConditions.fixedBcs?.length || 0,
        loads: (projectStore.boundaryConditions.pointLoads?.length || 0) +
               (projectStore.boundaryConditions.uniformLoads?.length || 0)
      },
      results: {
        maxDisplacement: (projectStore.lastResult as any)?.max_displacement,
        maxStress: (projectStore.lastResult as any)?.max_stress,
        maxStrain: (projectStore.lastResult as any)?.max_strain
      },
      parameters: {}
    }

    currentCitation.value = citation
    citations.value.unshift(citation)

    // 限制保存的引用数量
    if (citations.value.length > 100) {
      citations.value = citations.value.slice(0, 100)
    }

    // 保存到本地存储
    saveCitationsToStorage()

    return citation
  }

  /**
   * 导出为 BibTeX 格式
   */
  function exportToBibTeX(citation: SimulationCitation): string {
    const authors = citation.authors.join(' and ')
    const year = new Date(citation.createdAt).getFullYear()
    const month = new Date(citation.createdAt).toLocaleString('default', { month: 'short' })
    const key = `${citation.authors[0]?.split(' ').pop() || 'Unknown'}${year}`

    const lines = [
      `@misc{${key}${citation.id.slice(-6)},`,
      `  author = {${authors}},`,
      `  title = {${citation.title}},`,
      `  year = {${year}},`,
      `  month = {${month}},`,
      `  howpublished = {\\url{https://caelab.app/simulation/${citation.id}}},`,
      `  note = {Simulation ID: ${citation.id}},`,
      `  publisher = {CAELab}},`,
      ``,
      `  metadata = {`,
      `    simulation\_type = {${SIMULATION_TYPE_NAMES[citation.simulationType] || citation.simulationType}},`,
      `    mesh\_nodes = {${citation.mesh.nodes}},`,
      `    mesh\_elements = {${citation.mesh.elements}},`,
      `    max\_displacement = {${citation.results.maxDisplacement || 'N/A'}},`,
      `    max\_stress = {${citation.results.maxStress || 'N/A'}}`,
      `  }`,
      `}`
    ]

    return lines.join('\n')
  }

  /**
   * 导出为 RIS 格式
   */
  function exportToRIS(citation: SimulationCitation): string {
    const lines = [
      'TY  - COMP',
      `AU  - ${citation.authors.join('\nAU  - ')}`,
      `TI  - ${citation.title}`,
      `PY  - ${new Date(citation.createdAt).getFullYear()}`,
      `DA  - ${new Date(citation.createdAt).toISOString().split('T')[0]}`,
      `PB  - CAELab`,
      `UR  - https://caelab.app/simulation/${citation.id}`,
      `ID  - ${citation.id}`,
      `N1  - Simulation Type: ${SIMULATION_TYPE_NAMES[citation.simulationType] || citation.simulationType}`,
      `N1  - Mesh: ${citation.mesh.nodes} nodes, ${citation.mesh.elements} elements`,
      `N1  - Max Displacement: ${citation.results.maxDisplacement || 'N/A'}`,
      `N1  - Max Stress: ${citation.results.maxStress || 'N/A'}`,
      'ER  - ',
      ''
    ]
    return lines.join('\n')
  }

  /**
   * 导出为 JSON 格式
   */
  function exportToJSON(citation: SimulationCitation): string {
    return JSON.stringify(citation, null, 2)
  }

  /**
   * 导出为 APA 格式
   */
  function exportToAPA(citation: SimulationCitation): string {
    const author = citation.authors.length > 1
      ? `${citation.authors[0]} et al.`
      : citation.authors[0]
    const year = new Date(citation.createdAt).getFullYear()
    const month = new Date(citation.createdAt).toLocaleString('default', { month: 'long' }).toLowerCase()

    return [
      `${author} (${year}). ${citation.title} [Computer software]. CAELab.`,
      `Retrieved ${month} ${new Date().getDate()}, ${year}, from https://caelab.app/simulation/${citation.id}`,
      ``,
      `Simulation metadata:`,
      `- Type: ${SIMULATION_TYPE_NAMES[citation.simulationType] || citation.simulationType}`,
      `- Mesh: ${citation.mesh.nodes} nodes, ${citation.mesh.elements} elements`,
      `- Results: Max displacement ${citation.results.maxDisplacement || 'N/A'}, Max stress ${citation.results.maxStress || 'N/A'}`,
      `- ID: ${citation.id}`
    ].join('\n')
  }

  /**
   * 导出为纯文本格式
   */
  function exportToText(citation: SimulationCitation): string {
    return [
      `=== CAELab Simulation Citation ===`,
      `ID: ${citation.id}`,
      `Title: ${citation.title}`,
      `Authors: ${citation.authors.join(', ')}`,
      `Date: ${new Date(citation.createdAt).toLocaleString()}`,
      `Software: ${citation.software} ${citation.softwareVersion}`,
      ``,
      `Simulation Details:`,
      `  Type: ${SIMULATION_TYPE_NAMES[citation.simulationType] || citation.simulationType}`,
      `  Mesh: ${citation.mesh.nodes} nodes, ${citation.mesh.elements} elements`,
      `  Boundary Conditions: ${citation.boundaryConditions.fixed} fixed, ${citation.boundaryConditions.loads} loads`,
      ``,
      `Results:`,
      `  Max Displacement: ${citation.results.maxDisplacement?.toFixed(6) || 'N/A'} m`,
      `  Max Stress: ${citation.results.maxStress?.toFixed(2) || 'N/A'} MPa`,
      ``,
      `Reference URL: https://caelab.app/simulation/${citation.id}`
    ].join('\n')
  }

  /**
   * 根据格式导出引用
   */
  function exportCitation(citation: SimulationCitation, format: CitationFormat['id']): string {
    switch (format) {
      case 'bibtex': return exportToBibTeX(citation)
      case 'ris': return exportToRIS(citation)
      case 'json': return exportToJSON(citation)
      case 'apa': return exportToAPA(citation)
      case 'text': return exportToText(citation)
      default: return exportToText(citation)
    }
  }

  /**
   * 复制引用到剪贴板
   */
  async function copyToClipboard(citation: SimulationCitation, format: CitationFormat['id']): Promise<boolean> {
    const text = exportCitation(citation, format)
    try {
      await navigator.clipboard.writeText(text)
      return true
    } catch {
      // Fallback
      const textarea = document.createElement('textarea')
      textarea.value = text
      document.body.appendChild(textarea)
      textarea.select()
      document.execCommand('copy')
      document.body.removeChild(textarea)
      return true
    }
  }

  /**
   * 获取历史引用列表
   */
  function getCitationHistory(): SimulationCitation[] {
    try {
      const stored = localStorage.getItem('caelab_citations')
      if (stored) {
        citations.value = JSON.parse(stored)
      }
    } catch {}
    return citations.value
  }

  /**
   * 保存引用到本地存储
   */
  function saveCitationsToStorage() {
    try {
      localStorage.setItem('caelab_citations', JSON.stringify(citations.value))
    } catch {}
  }

  /**
   * 删除引用
   */
  function deleteCitation(id: string) {
    citations.value = citations.value.filter(c => c.id !== id)
    saveCitationsToStorage()
  }

  /**
   * 清空历史
   */
  function clearHistory() {
    citations.value = []
    saveCitationsToStorage()
  }

  return {
    citations,
    currentCitation,
    citationFormats: CITATION_FORMATS,
    createCitation,
    exportCitation,
    exportToBibTeX,
    exportToRIS,
    exportToJSON,
    exportToAPA,
    exportToText,
    copyToClipboard,
    getCitationHistory,
    deleteCitation,
    clearHistory
  }
}