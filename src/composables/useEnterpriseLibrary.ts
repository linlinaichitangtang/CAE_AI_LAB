/**
 * useEnterpriseLibrary.ts — V4.1-004 企业标准件库与检查清单
 * 标准件库、材料标准映射、仿真前检查清单
 */
import { ref, computed } from 'vue'

export interface StandardPart {
  id: string
  name: string
  category: 'fastener' | 'structural' | 'bearing' | 'seal' | 'custom'
  standard: string      // GB/T, ISO, DIN, etc.
  standardCode: string  // e.g. "GB/T 5783"
  specs: Record<string, number | string>
  material: string
  materialStandard?: string
  icon?: string
}

export interface ChecklistItem {
  id: string
  category: 'geometry' | 'material' | 'mesh' | 'boundary' | 'solver' | 'post'
  label: string
  description: string
  required: boolean
  checked: boolean
  autoCheck?: () => boolean
}

export interface DesignRule {
  id: string
  name: string
  category: string
  condition: string
  threshold: number
  operator: '>' | '<' | '>=' | '<=' | '=='
  unit: string
  severity: 'critical' | 'warning' | 'info'
}

export interface StandardMaterial {
  name: string
  standard: string
  grade: string
  density: number
  elasticModulus: number
  poissonRatio: number
  yieldStrength: number
  tensileStrength: number
  thermalExpansion?: number
  applications: string[]
}

// ========== 内置标准件库 ==========
const builtInStandardParts: StandardPart[] = [
  // 紧固件
  {
    id: 'bolt-m8-8.8',
    name: 'M8×40 六角头螺栓',
    category: 'fastener',
    standard: 'GB/T',
    standardCode: 'GB/T 5783',
    specs: { diameter: 8, length: 40, pitch: 1.25, headHeight: 5.3, headWidth: 13 },
    material: '35CrMo',
    materialStandard: 'GB/T 3077',
    icon: '🔩'
  },
  {
    id: 'bolt-m12-10.9',
    name: 'M12×60 高强度螺栓',
    category: 'fastener',
    standard: 'GB/T',
    standardCode: 'GB/T 5782',
    specs: { diameter: 12, length: 60, pitch: 1.75, headHeight: 7.5, headWidth: 18 },
    material: '40Cr',
    materialStandard: 'GB/T 3077',
    icon: '🔩'
  },
  {
    id: 'nut-m8-8',
    name: 'M8 1型六角螺母',
    category: 'fastener',
    standard: 'GB/T',
    standardCode: 'GB/T 6170',
    specs: { diameter: 8, height: 6.8, width: 13 },
    material: '35CrMo',
    materialStandard: 'GB/T 3077',
    icon: '🔧'
  },
  {
    id: 'washer-m8',
    name: 'M8 平垫圈',
    category: 'fastener',
    standard: 'GB/T',
    standardCode: 'GB/T 97.1',
    specs: { innerDiameter: 8.4, outerDiameter: 16, thickness: 1.6 },
    material: 'Q235',
    materialStandard: 'GB/T 700',
    icon: '⭕'
  },
  // 结构件
  {
    id: 'hbeam-200',
    name: 'HW200×200 H型钢',
    category: 'structural',
    standard: 'GB/T',
    standardCode: 'GB/T 11263',
    specs: { height: 200, width: 200, webThickness: 8, flangeThickness: 12, weight: 49.9 },
    material: 'Q355B',
    materialStandard: 'GB/T 1591',
    icon: '工'
  },
  {
    id: 'channel-160',
    name: '槽钢 160×63',
    category: 'structural',
    standard: 'GB/T',
    standardCode: 'GB/T 707',
    specs: { height: 160, width: 63, webThickness: 6.5, weight: 17.3 },
    material: 'Q235B',
    materialStandard: 'GB/T 700',
    icon: '⊏'
  },
  // 轴承
  {
    id: 'bearing-6205',
    name: '深沟球轴承 6205',
    category: 'bearing',
    standard: 'GB/T',
    standardCode: 'GB/T 276',
    specs: { innerDiameter: 25, outerDiameter: 52, width: 15, dynamicLoad: 14.0, staticLoad: 7.85 },
    material: 'GCr15',
    materialStandard: 'GB/T 18254',
    icon: '◎'
  },
  {
    id: 'bearing-30206',
    name: '圆锥滚子轴承 30206',
    category: 'bearing',
    standard: 'GB/T',
    standardCode: 'GB/T 297',
    specs: { innerDiameter: 30, outerDiameter: 62, width: 17.25, dynamicLoad: 43.2, staticLoad: 50.5 },
    material: 'GCr15',
    materialStandard: 'GB/T 18254',
    icon: '◎'
  }
]

// ========== 标准材料库 ==========
const builtInStandardMaterials: StandardMaterial[] = [
  {
    name: 'Q235B', standard: 'GB/T 700', grade: 'B级',
    density: 7850, elasticModulus: 206000, poissonRatio: 0.3,
    yieldStrength: 235, tensileStrength: 375,
    thermalExpansion: 1.2e-5,
    applications: ['建筑结构', '一般机械零件']
  },
  {
    name: 'Q355B', standard: 'GB/T 1591', grade: 'B级',
    density: 7850, elasticModulus: 206000, poissonRatio: 0.3,
    yieldStrength: 355, tensileStrength: 470,
    thermalExpansion: 1.2e-5,
    applications: ['桥梁', '车辆', '压力容器']
  },
  {
    name: '35CrMo', standard: 'GB/T 3077', grade: '调质',
    density: 7850, elasticModulus: 210000, poissonRatio: 0.3,
    yieldStrength: 835, tensileStrength: 980,
    thermalExpansion: 1.1e-5,
    applications: ['高强度螺栓', '轴类零件', '齿轮']
  },
  {
    name: '40Cr', standard: 'GB/T 3077', grade: '调质',
    density: 7850, elasticModulus: 210000, poissonRatio: 0.3,
    yieldStrength: 785, tensileStrength: 980,
    thermalExpansion: 1.1e-5,
    applications: ['重要轴类', '齿轮', '连杆']
  },
  {
    name: 'GCr15', standard: 'GB/T 18254', grade: '退火',
    density: 7810, elasticModulus: 219000, poissonRatio: 0.3,
    yieldStrength: 350, tensileStrength: 520,
    thermalExpansion: 1.0e-5,
    applications: ['轴承', '精密量具']
  },
  {
    name: 'TC4', standard: 'GB/T 3620.1', grade: 'Ti-6Al-4V',
    density: 4430, elasticModulus: 113800, poissonRatio: 0.342,
    yieldStrength: 880, tensileStrength: 950,
    thermalExpansion: 8.6e-6,
    applications: ['航空发动机', '航天结构', '医疗植入物']
  },
  {
    name: 'Al7075-T6', standard: 'GB/T 3190', grade: 'T6',
    density: 2810, elasticModulus: 71700, poissonRatio: 0.33,
    yieldStrength: 503, tensileStrength: 572,
    thermalExpansion: 2.3e-5,
    applications: ['航空结构', '模具', '自行车']
  }
]

// ========== 设计规则库 ==========
const builtInDesignRules: DesignRule[] = [
  { id: 'dr-001', name: '最小壁厚', category: 'casting', condition: '壁厚', threshold: 3, operator: '>=', unit: 'mm', severity: 'critical' },
  { id: 'dr-002', name: '螺栓间距', category: 'fastener', condition: '间距/直径', threshold: 3, operator: '>=', unit: 'ratio', severity: 'warning' },
  { id: 'dr-003', name: '孔边距', category: 'fastener', condition: '边距/直径', threshold: 1.5, operator: '>=', unit: 'ratio', severity: 'critical' },
  { id: 'dr-004', name: '长细比限制', category: 'buckling', condition: '长细比', threshold: 200, operator: '<=', unit: 'ratio', severity: 'warning' },
  { id: 'dr-005', name: '焊缝间距', category: 'welding', condition: '间距', threshold: 50, operator: '>=', unit: 'mm', severity: 'info' },
  { id: 'dr-006', name: '圆角半径', category: 'stress', condition: '圆角/壁厚', threshold: 0.5, operator: '>=', unit: 'ratio', severity: 'warning' },
  { id: 'dr-007', name: '安全系数', category: 'safety', condition: '安全系数', threshold: 1.5, operator: '>=', unit: 'ratio', severity: 'critical' }
]

export function useEnterpriseLibrary() {
  const standardParts = ref<StandardPart[]>([...builtInStandardParts])
  const standardMaterials = ref<StandardMaterial[]>([...builtInStandardMaterials])
  const designRules = ref<DesignRule[]>([...builtInDesignRules])
  const customParts = ref<StandardPart[]>([])

  const allParts = computed(() => [...standardParts.value, ...customParts.value])

  const partsByCategory = computed(() => {
    const map = new Map<string, StandardPart[]>()
    for (const part of allParts.value) {
      const list = map.get(part.category) || []
      list.push(part)
      map.set(part.category, list)
    }
    return map
  })

  function searchParts(query: string): StandardPart[] {
    const q = query.toLowerCase()
    return allParts.value.filter(p =>
      p.name.toLowerCase().includes(q) ||
      p.standardCode.toLowerCase().includes(q) ||
      p.material.toLowerCase().includes(q)
    )
  }

  function getMaterialByName(name: string): StandardMaterial | undefined {
    return standardMaterials.value.find(m => m.name === name)
  }

  function addCustomPart(part: Omit<StandardPart, 'id'>): StandardPart {
    const newPart: StandardPart = {
      ...part,
      id: `custom-${Date.now()}`
    }
    customParts.value.push(newPart)
    return newPart
  }

  function removeCustomPart(id: string) {
    customParts.value = customParts.value.filter(p => p.id !== id)
  }

  // ========== 检查清单 ==========
  function createChecklist(simulationType: string): ChecklistItem[] {
    const commonItems: ChecklistItem[] = [
      { id: 'chk-geo-1', category: 'geometry', label: '几何模型已清理', description: '移除小边、小面、短边等几何缺陷', required: true, checked: false },
      { id: 'chk-geo-2', category: 'geometry', label: '单位制一致', description: '确认模型单位与材料单位匹配', required: true, checked: false },
      { id: 'chk-mat-1', category: 'material', label: '材料属性完整', description: '密度、弹性模量、泊松比已定义', required: true, checked: false },
      { id: 'chk-mat-2', category: 'material', label: '材料来源可靠', description: '使用标准材料库或实验数据', required: false, checked: false },
      { id: 'chk-mesh-1', category: 'mesh', label: '网格已划分', description: '所有体/面已分配网格', required: true, checked: false },
      { id: 'chk-mesh-2', category: 'mesh', label: '网格质量合格', description: 'Jacobian > 0, 长宽比 < 10', required: true, checked: false },
      { id: 'chk-mesh-3', category: 'mesh', label: '关注区域加密', description: '应力集中区域网格足够精细', required: false, checked: false },
      { id: 'chk-bc-1', category: 'boundary', label: '约束充分', description: '消除刚体位移，不过约束', required: true, checked: false },
      { id: 'chk-bc-2', category: 'boundary', label: '载荷合理', description: '载荷大小、方向、分布符合实际', required: true, checked: false },
      { id: 'chk-sol-1', category: 'solver', label: '求解器设置正确', description: '分析类型、求解步长、收敛准则合适', required: true, checked: false },
      { id: 'chk-sol-2', category: 'solver', label: '输出变量已选择', description: '应力、应变、位移等需要的结果已选', required: false, checked: false },
      { id: 'chk-post-1', category: 'post', label: '结果云图已检查', description: '确认无异常值、无数值奇异', required: true, checked: false }
    ]

    if (simulationType.includes('fatigue') || simulationType.includes('疲劳')) {
      commonItems.push(
        { id: 'chk-fat-1', category: 'material', label: 'S-N曲线已定义', description: '疲劳材料属性包含S-N或E-N曲线', required: true, checked: false },
        { id: 'chk-fat-2', category: 'solver', label: '载荷谱已输入', description: '循环载荷幅值和均值已定义', required: true, checked: false }
      )
    }

    if (simulationType.includes('thermal') || simulationType.includes('热')) {
      commonItems.push(
        { id: 'chk-th-1', category: 'material', label: '热物性完整', description: '导热系数、比热容、热膨胀系数已定义', required: true, checked: false },
        { id: 'chk-th-2', category: 'boundary', label: '热边界条件', description: '温度或热流边界已施加', required: true, checked: false }
      )
    }

    if (simulationType.includes('dynamic') || simulationType.includes('modal')) {
      commonItems.push(
        { id: 'chk-dyn-1', category: 'solver', label: '模态阶数足够', description: '提取模态覆盖频率范围', required: true, checked: false },
        { id: 'chk-dyn-2', category: 'material', label: '质量分布合理', description: '密度或质量点定义正确', required: true, checked: false }
      )
    }

    return commonItems
  }

  function validateChecklist(items: ChecklistItem[]): { passed: boolean; missing: ChecklistItem[]; rate: number } {
    const required = items.filter(i => i.required)
    const checkedRequired = required.filter(i => i.checked)
    const missing = required.filter(i => !i.checked)
    const rate = items.length > 0 ? checkedRequired.length / required.length : 0
    return {
      passed: missing.length === 0,
      missing,
      rate
    }
  }

  // ========== 设计规则检查 ==========
  function checkDesignRule(rule: DesignRule, actualValue: number): { pass: boolean; margin: number } {
    let pass = false
    switch (rule.operator) {
      case '>': pass = actualValue > rule.threshold; break
      case '<': pass = actualValue < rule.threshold; break
      case '>=': pass = actualValue >= rule.threshold; break
      case '<=': pass = actualValue <= rule.threshold; break
      case '==': pass = Math.abs(actualValue - rule.threshold) < 1e-6; break
    }
    const margin = rule.operator.startsWith('>')
      ? (actualValue - rule.threshold) / rule.threshold
      : (rule.threshold - actualValue) / rule.threshold
    return { pass, margin }
  }

  return {
    standardParts,
    customParts,
    allParts,
    partsByCategory,
    standardMaterials,
    designRules,
    searchParts,
    getMaterialByName,
    addCustomPart,
    removeCustomPart,
    createChecklist,
    validateChecklist,
    checkDesignRule
  }
}
