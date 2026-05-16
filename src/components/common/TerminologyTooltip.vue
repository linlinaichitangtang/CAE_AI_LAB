/**
 * TerminologyTooltip.vue — V4.0-004 术语解释悬浮窗
 * 鼠标悬停到术语上弹出中文解释 + LaTeX 公式 + 图示
 */
<template>
  <div class="terminology-tooltip" v-if="visible" :style="tooltipStyle">
    <div class="tooltip-header">
      <span class="term-name">{{ currentTerm?.term }}</span>
      <span class="term-category">{{ currentTerm?.category }}</span>
    </div>

    <div class="tooltip-body">
      <p class="term-definition">{{ currentTerm?.definition }}</p>

      <div v-if="currentTerm?.formula" class="term-formula">
        <span class="formula-label">公式:</span>
        <code class="formula-text">{{ currentTerm.formula }}</code>
      </div>

      <div v-if="currentTerm?.latex" class="term-latex" v-html="renderLatex(currentTerm.latex)" />

      <div v-if="currentTerm?.example" class="term-example">
        <span class="example-label">示例:</span>
        <p class="example-text">{{ currentTerm.example }}</p>
      </div>

      <div v-if="currentTerm?.relatedTerms?.length" class="term-related">
        <span class="related-label">相关术语:</span>
        <div class="related-tags">
          <span
            v-for="related in currentTerm.relatedTerms"
            :key="related"
            class="related-tag"
            @click="navigateTo(related)"
          >
            {{ related }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

export interface TermDefinition {
  term: string
  category: string
  definition: string
  formula?: string
  latex?: string
  example?: string
  relatedTerms?: string[]
  icon?: string
}

interface TooltipProps {
  term: TermDefinition | null
  visible: boolean
  position: { x: number; y: number }
}

const props = defineProps<TooltipProps>()

const tooltipStyle = computed(() => ({
  left: `${props.position.x + 10}px`,
  top: `${props.position.y + 10}px`
}))

const currentTerm = computed(() => props.term)

function navigateTo(term: string) {
  // Emit event to navigate to related term
  console.log('Navigate to:', term)
}

// 简化的 LaTeX 渲染（实际项目中建议使用 KaTeX 或 MathJax）
function renderLatex(latex: string): string {
  // 基本转换，实际项目需要完整 LaTeX 支持
  return latex
    .replace(/sigma/g, 'σ')
    .replace(/epsilon/g, 'ε')
    .replace(/delta/g, 'δ')
    .replace(/lambda/g, 'λ')
    .replace(/\^2/g, '²')
    .replace(/\^3/g, '³')
    .replace(/\^n/g, 'ⁿ')
    .replace(/_/g, '₍')
}

// 预定义的 CAE 术语库
export const CAE_TERMS: Record<string, TermDefinition> = {
  'von_mises': {
    term: 'von Mises 应力',
    category: '应力分析',
    definition: 'von Mises 应力是一种等效应力准则，用于判断材料是否进入塑性状态。在多轴应力状态下，将复杂应力状态简化为等效的单轴拉伸应力。',
    formula: 'σ_v = √[(σ₁-σ₂)² + (σ₂-σ₃)² + (σ₃-σ₁)²] / 2',
    latex: '\\sigma_v = \\sqrt{\\frac{(\\sigma_1-\\\sigma_2)^2 + (\\sigma_2-\\\sigma_3)^2 + (\\sigma_3-\\sigma_1)^2}{2}}',
    example: 'σ_v < σ_y 时材料处于弹性状态；σ_v ≥ σ_y 时材料开始屈服',
    relatedTerms: ['主应力', '屈服条件', '应力张量'],
    icon: '⚡'
  },
  'poissons_ratio': {
    term: '泊松比 ν',
    category: '材料参数',
    definition: '泊松比是材料在拉伸时横向应变与轴向应变的比值，反映材料的横向变形特性。对于大多数金属材料，ν ≈ 0.3。',
    formula: 'ν = -ε_lat / ε_ax',
    example: '橡胶 ν ≈ 0.5（不可压缩）；混凝土 ν ≈ 0.2；软木 ν ≈ 0',
    relatedTerms: ['弹性模量', '剪切模量', '体积模量'],
    icon: '📐'
  },
  'elastic_modulus': {
    term: '弹性模量 E',
    category: '材料参数',
    definition: '弹性模量也称杨氏模量，是材料抵抗弹性变形的能力的度量。E 越大，材料越"刚"，越不容易发生弹性变形。',
    formula: 'E = σ / ε',
    example: '钢材 E ≈ 206 GPa；铝材 E ≈ 69 GPa；橡胶 E ≈ 0.001 GPa',
    relatedTerms: ['泊松比', '屈服强度', '胡克定律'],
    icon: '🔩'
  },
  'yield_strength': {
    term: '屈服强度 σy',
    category: '材料参数',
    definition: '屈服强度是材料开始产生塑性变形的应力阈值。当应力达到屈服强度时，材料从弹性行为转变为塑性行为。',
    formula: 'σ_y',
    example: 'Q235 钢 σ_y = 235 MPa；Q345 钢 σ_y = 345 MPa；6061-T6 铝 σ_y = 276 MPa',
    relatedTerms: ['弹性模量', '抗拉强度', '安全系数'],
    icon: '⚓'
  },
  'jacobian': {
    term: '雅可比矩阵 J',
    category: '网格质量',
    definition: '雅可比矩阵描述单元从自然坐标到实际坐标的映射关系。雅可比行列式表示映射的缩放因子，用于评估网格单元的畸变程度。',
    formula: 'J = ∂x/∂ξ',
    example: 'J > 0 表示单元合法；J < 0 单元翻转（不合格）；理想值 J = 1（等参单元）',
    relatedTerms: ['网格质量', '单元畸变', '等参单元'],
    icon: '📊'
  },
  'aspect_ratio': {
    term: '长宽比 Aspect Ratio',
    category: '网格质量',
    definition: '长宽比是单元最长边与最短边的比值，用于评估网格单元的形状质量。比值越接近1，网格质量越好。',
    formula: 'AR = L_max / L_min',
    example: 'AR = 1 表示完美正方形/正四面体；AR > 3 可能影响计算精度；AR > 10 单元严重畸变',
    relatedTerms: ['网格质量', '雅可比矩阵', '网格收敛性'],
    icon: '🔲'
  },
  'skewness': {
    term: '歪斜度 Skewness',
    category: '网格质量',
    definition: '歪斜度衡量单元与其理想形状（等边/正交）的偏差程度。歪斜度越大，单元形状越差，计算误差越大。',
    formula: 'Skew = (θ_max - θ_equil) / (180° - θ_equil)',
    example: 'Skew = 0 表示完美单元；Skew < 0.25 优质；Skew > 0.5 需改进；Skew > 0.9 不合格',
    relatedTerms: ['网格质量', '雅可比矩阵', '网格优化'],
    icon: '📐'
  },
  'stress_concentration': {
    term: '应力集中',
    category: '应力分析',
    definition: '应力集中是指在几何不连续处（如孔、槽、突变截面）应力显著升高的现象。应力集中系数 K_t 定义为最大应力与名义应力的比值。',
    formula: 'K_t = σ_max / σ_nominal',
    example: '圆孔板 K_t ≈ 3；缺口梁 K_t 可达 5-10；圆角过渡可降低 K_t',
    relatedTerms: ['von Mises 应力', '疲劳寿命', '应力集中系数'],
    icon: '⚠️'
  },
  'modal_analysis': {
    term: '模态分析',
    category: '动力学分析',
    definition: '模态分析用于确定结构的固有频率和振型，是进行响应谱分析、谐响应分析和瞬态动力学分析的基础。',
    formula: '[K]{φ} = λ[M]{φ}',
    example: '一阶模态频率约 50 Hz；二阶模态频率约 120 Hz',
    relatedTerms: ['固有频率', '振型', '动力分析'],
    icon: '📳'
  },
  'buckling': {
    term: '屈曲分析',
    category: '稳定性分析',
    definition: '屈曲分析用于确定结构在压力作用下失去稳定性（发生突然变形）的临界载荷。分为线性屈曲和非线性屈曲。',
    formula: 'P_cr = π²EI / (KL)²',
    example: '压杆临界载荷 P_cr；超过此值结构发生屈曲失稳',
    relatedTerms: ['临界载荷', '压杆稳定性', '欧拉公式'],
    icon: '📉'
  },
  'fatigue': {
    term: '疲劳分析',
    category: '耐久性分析',
    definition: '疲劳是指材料在循环载荷作用下逐渐产生裂纹并最终断裂的现象。S-N 曲线描述应力幅与循环次数的关系。',
    formula: 'σ_a = σ_f\'(2N)^b',
    example: '高周疲劳 N > 10^4 次；低周疲劳 ε_p > 0.01',
    relatedTerms: ['S-N曲线', '疲劳极限', '裂纹扩展'],
    icon: '🔄'
  },
  'thermal_stress': {
    term: '热应力',
    category: '热分析',
    definition: '热应力是由于温度变化导致材料膨胀或收缩受到约束而产生的应力。热应变与温度变化成正比，比例系数为热膨胀系数。',
    formula: 'σ_T = EαΔT / (1-ν)',
    example: '温度升高 ΔT > 0 产生压应力；温度降低 ΔT < 0 产生拉应力',
    relatedTerms: ['热膨胀系数', '温度载荷', '热应变'],
    icon: '🌡️'
  },
  'mesh_convergence': {
    term: '网格收敛性',
    category: '数值方法',
    definition: '网格收敛性是指随着网格不断加密，仿真结果逐渐接近真实解的过程。通过对比不同网格密度下的结果，验证结果的可靠性。',
    formula: '|δ_{n+1} - δ_n| / δ_n < ε',
    example: '加密网格后应力变化 < 5% 认为收敛',
    relatedTerms: ['网格密度', '结果验证', '自适应网格'],
    icon: '✅'
  },
  'principal_stress': {
    term: '主应力',
    category: '应力分析',
    definition: '主应力是材料内部某点处相互垂直的三个法向应力分量。按代数值大小排序：σ₁ ≥ σ₂ ≥ σ₃。',
    formula: 'σ₁, σ₂, σ₃ (特征值)',
    example: '单向拉伸时 σ₁ = σ_axial, σ₂ = σ₃ = 0',
    relatedTerms: ['von Mises 应力', '应力张量', '摩尔圆'],
    icon: '📊'
  },
  'displacement': {
    term: '位移',
    category: '结构响应',
    definition: '位移是结构在载荷作用下各点的位置变化。最大位移通常出现在载荷作用点或结构刚度最弱处。',
    formula: 'u(x,y,z)',
    example: '悬臂梁自由端最大挠度 δ = FL³/(3EI)',
    relatedTerms: ['变形', '挠度', '刚度矩阵'],
    icon: '↔️'
  }
}

// 导出术语库
export function getTermDefinition(termKey: string): TermDefinition | null {
  return CAE_TERMS[termKey] || CAE_TERMS[termKey.toLowerCase()] || null
}

// 搜索术语
export function searchTerms(query: string): TermDefinition[] {
  const q = query.toLowerCase()
  return Object.values(CAE_TERMS).filter(term =>
    term.term.toLowerCase().includes(q) ||
    term.definition.includes(q) ||
    term.category.includes(q)
  )
}
</script>

<style scoped>
.terminology-tooltip {
  position: fixed;
  z-index: 9999;
  width: 320px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  border: 1px solid var(--border-color, #e2e8f0);
  overflow: hidden;
  pointer-events: none;
}

.tooltip-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--bg-elevated, #f8fafc);
  border-bottom: 1px solid var(--border-color, #e2e8f0);
}

.term-name {
  font-weight: 600;
  color: var(--text-primary, #1e293b);
}

.term-category {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 10px;
  background: var(--primary-color, #3b82f6);
  color: white;
}

.tooltip-body {
  padding: 12px 16px;
  max-height: 300px;
  overflow-y: auto;
}

.term-definition {
  font-size: 13px;
  line-height: 1.6;
  color: var(--text-secondary, #475569);
  margin-bottom: 12px;
}

.term-formula {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
}

.formula-label {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
}

.formula-text {
  font-family: 'Courier New', monospace;
  font-size: 12px;
  background: var(--bg-elevated, #f1f5f9);
  padding: 4px 8px;
  border-radius: 4px;
  color: var(--primary-color, #3b82f6);
}

.term-latex {
  margin-bottom: 12px;
  text-align: center;
  font-size: 14px;
}

.term-example {
  margin-bottom: 12px;
}

.example-label {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
  display: block;
  margin-bottom: 4px;
}

.example-text {
  font-size: 12px;
  color: var(--text-secondary, #475569);
  background: var(--bg-elevated, #f8fafc);
  padding: 8px;
  border-radius: 6px;
}

.term-related {
  border-top: 1px solid var(--border-color, #e2e8f0);
  padding-top: 10px;
}

.related-label {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
  display: block;
  margin-bottom: 6px;
}

.related-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.related-tag {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 10px;
  background: var(--bg-elevated, #f1f5f9);
  color: var(--text-secondary, #64748b);
  cursor: pointer;
  pointer-events: auto;
  transition: all 0.2s;
}

.related-tag:hover {
  background: var(--primary-color, #3b82f6);
  color: white;
}
</style>