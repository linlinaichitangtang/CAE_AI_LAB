/**
 * useTerminologyTooltip.ts — V4.0-004 术语悬浮窗逻辑
 * 管理术语识别、悬浮窗显示与定位
 */
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { getTermDefinition } from '../components/common/TerminologyTooltip.vue'
import type { TermDefinition } from '../components/common/TerminologyTooltip.vue'
// @ts-ignore - CAE_TERMS exported from Vue SFC
import { CAE_TERMS } from '../components/common/TerminologyTooltip.vue'

export function useTerminologyTooltip() {
  const visible = ref(false)
  const position = ref({ x: 0, y: 0 })
  const currentTerm = ref<TermDefinition | null>(null)
  const searchQuery = ref('')

  // 搜索结果
  const searchResults = computed(() => {
    if (!searchQuery.value) return []
    const q = searchQuery.value.toLowerCase()
    return Object.values(CAE_TERMS).filter(term =>
      term.term.toLowerCase().includes(q) ||
      term.definition.toLowerCase().includes(q) ||
      term.category.toLowerCase().includes(q)
    ).slice(0, 10)
  })

  // 显示悬浮窗
  function showTooltip(termKey: string, event?: MouseEvent) {
    const term = getTermDefinition(termKey)
    if (term) {
      currentTerm.value = term
      if (event) {
        position.value = { x: event.clientX, y: event.clientY }
      }
      visible.value = true
    }
  }

  // 隐藏悬浮窗
  function hideTooltip() {
    visible.value = false
    currentTerm.value = null
  }

  // 更新位置
  function updatePosition(x: number, y: number) {
    position.value = { x, y }
  }

  // 搜索术语
  function search(query: string): TermDefinition[] {
    searchQuery.value = query
    return searchResults.value
  }

  // 高亮文本中的术语
  function highlightTerms(text: string): string {
    let result = text
    for (const term of Object.values(CAE_TERMS)) {
      // 替换术语为带标记的 HTML
      const regex = new RegExp(`(${term.term})`, 'gi')
      result = result.replace(regex, `<span class="term-highlight" data-term="${term.term}">$1</span>`)
    }
    return result
  }

  // 处理鼠标移动（全局）
  function handleMouseMove(event: MouseEvent) {
    if (visible.value) {
      updatePosition(event.clientX, event.clientY)
    }
  }

  // 处理术语点击
  function handleTermClick(event: Event) {
    const target = event.target as HTMLElement
    if (target.classList.contains('term-highlight')) {
      const termKey = target.dataset.term
      if (termKey) {
        showTooltip(termKey, event as unknown as MouseEvent)
      }
    }
  }

  // 全局事件监听
  onMounted(() => {
    document.addEventListener('mousemove', handleMouseMove)
    document.addEventListener('click', handleTermClick)
  })

  onUnmounted(() => {
    document.removeEventListener('mousemove', handleMouseMove)
    document.removeEventListener('click', handleTermClick)
  })

  return {
    visible,
    position,
    currentTerm,
    searchQuery,
    searchResults,
    showTooltip,
    hideTooltip,
    updatePosition,
    search,
    highlightTerms
  }
}

// 预定义的术语正则表达式（用于实时检测）
export const TERM_PATTERNS: Array<{ pattern: RegExp; termKey: string }> = [
  { pattern: /von\s*Mises|vonMises/i, termKey: 'von_mises' },
  { pattern: /泊松比/i, termKey: 'poissons_ratio' },
  { pattern: /弹性模量|杨氏模量|E\s*=/i, termKey: 'elastic_modulus' },
  { pattern: /屈服强度|σ_y/i, termKey: 'yield_strength' },
  { pattern: /雅可比| Jacobian/i, termKey: 'jacobian' },
  { pattern: /长宽比|Aspect\s*Ratio/i, termKey: 'aspect_ratio' },
  { pattern: /歪斜度|Skewness/i, termKey: 'skewness' },
  { pattern: /应力集中/i, termKey: 'stress_concentration' },
  { pattern: /模态分析|固有频率/i, termKey: 'modal_analysis' },
  { pattern: /屈曲|临界载荷/i, termKey: 'buckling' },
  { pattern: /疲劳|S-N/i, termKey: 'fatigue' },
  { pattern: /热应力|热膨胀/i, termKey: 'thermal_stress' },
  { pattern: /网格收敛/i, termKey: 'mesh_convergence' },
  { pattern: /主应力|σ_1/i, termKey: 'principal_stress' },
  { pattern: /位移|挠度/i, termKey: 'displacement' }
]

// 检测文本中的术语
export function detectTerms(text: string): Array<{ term: TermDefinition; start: number; end: number }> {
  const results: Array<{ term: TermDefinition; start: number; end: number }> = []

  for (const { pattern, termKey } of TERM_PATTERNS) {
    const match = pattern.exec(text)
    if (match) {
      const term = getTermDefinition(termKey)
      if (term) {
        results.push({ term, start: match.index, end: match.index + match[0].length })
      }
    }
  }

  return results.sort((a, b) => a.start - b.start)
}