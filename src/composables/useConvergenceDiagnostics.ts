/**
 * useConvergenceDiagnostics.ts — V4.2-004 非线性收敛自动修复
 * 分析求解器收敛失败原因，自动推荐修复策略
 */
import { ref, computed } from 'vue'

/** 收敛诊断场景 */
export type ConvergenceScenario =
  | 'excessive_dof'           // 自由度过多
  | 'contact_penetration'     // 接触穿透
  | 'large_strain'            // 大应变/大变形
  | 'material_nonlinearity'   // 材料非线性
  | 'geometric_instability'   // 几何不稳定（屈曲）
  | 'thermal_stiffening'       // 热软化/热刚化
  | 'mesh_distortion'         // 网格畸变
  | 'ill_conditioned'         // 病态方程组
  | 'load_step_too_large'     // 载荷步过大
  | 'mixed_convergence'        // 混合收敛问题

/** 修复策略 */
export interface RepairStrategy {
  id: string
  scenario: ConvergenceScenario
  priority: number         // 1=首选, 2=次选, 3=备选
  label: string
  description: string
  action: string            // 用户可执行的操作描述
  inpModifier?: (input: string) => string  // 可选：直接修改 INP 内容
  expectedImprovement: string
  riskLevel: 'low' | 'medium' | 'high'
}

/** 诊断记录 */
export interface ConvergenceDiagRecord {
  timestamp: string
  jobId: string
  scenario: ConvergenceScenario
  residualHistory: number[]
  incrementHistory: number[]
  iterationCount: number
  lastError: string
  appliedStrategies: string[]
  stepHistory?: number[]
}

/** 收敛历史点 */
export interface ConvergenceHistoryPoint {
  step: number
  iteration: number
  residual: number
  increment: number
  ratio: number
}

/** 推荐的修复策略列表 */
export interface RepairRecommendation {
  primary: RepairStrategy
  alternatives: RepairStrategy[]
  autoApplyAvailable: boolean
  estimatedFixProbability: number  // 0-1
  combinedStrategy?: string        // 可组合多个策略
}

const REPAIR_STRATEGIES: RepairStrategy[] = [
  // 接触穿透
  {
    id: 'contact-adjust-mortars',
    scenario: 'contact_penetration',
    priority: 1,
    label: '调整接触算法',
    description: '将接触算法从 Penalty 切换到 Lagrange 或增强拉格朗日法，减少穿透',
    action: '修改 *CONTACT PAIR 或 *CONTACT FORMULATION 为 LAGRANGIAN 或 ENHANCED',
    expectedImprovement: '穿透量减少 50-80%',
    riskLevel: 'low'
  },
  {
    id: 'contact-reduce-penalty',
    scenario: 'contact_penetration',
    priority: 2,
    label: '减小 Penalty 刚度',
    description: '减小接触刚度（C青年），避免过约束导致的不收敛',
    action: '设置 *CONTACT PAIR 中 C青年 参数为原值的 1/10',
    expectedImprovement: '穿透量减少但精度降低',
    riskLevel: 'medium'
  },
  {
    id: 'contact-friction-zero',
    scenario: 'contact_penetration',
    priority: 3,
    label: '临时移除摩擦',
    description: '暂时将摩擦系数设为 0，先使接触建立，再逐步恢复摩擦',
    action: '设置 *CONTACT PAIR 中 FRICTION 为 0',
    expectedImprovement: '建立稳定接触后恢复',
    riskLevel: 'low'
  },
  {
    id: 'contact-add stabilization',
    scenario: 'contact_penetration',
    priority: 2,
    label: '打开接触稳定化',
    description: '启用弹性滑移稳定化，允许小幅度的接触滑移',
    action: '添加 *CONTACT STABILIZATION 参数',
    expectedImprovement: '改善初期接触建立',
    riskLevel: 'low'
  },

  // 大应变/几何非线性
  {
    id: 'nlgeom-on',
    scenario: 'large_strain',
    priority: 1,
    label: '打开几何非线性',
    description: '开启 NLGEOM 选项，精确考虑大变形和大应变效应',
    action: '添加 *STEP, NLGEOM=YES',
    expectedImprovement: '正确处理大变形问题',
    riskLevel: 'low'
  },
  {
    id: 'refine-load-step',
    scenario: 'large_strain',
    priority: 1,
    label: '减小初始载荷步',
    description: '将初始载荷步从默认的 1.0 减小到 0.1 或更小',
    action: '设置 *STATIC 中 DLFLOAD=0.1 或使用 *DYNAMIC',
    expectedImprovement: '避免过冲导致的不收敛',
    riskLevel: 'low'
  },
  {
    id: 'riks-analysis',
    scenario: 'large_strain',
    priority: 2,
    label: '使用 Riks 分析（弧长法）',
    description: '对于加载超过极限点的问题，使用 Riks 弧长法追踪下降路径',
    action: '使用 *STATIC, RIKS 而不是普通 *STATIC',
    expectedImprovement: '追踪极限点后的软化响应',
    riskLevel: 'medium'
  },

  // 材料非线性
  {
    id: 'plastic-refine-curve',
    scenario: 'material_nonlinearity',
    priority: 1,
    label: '细化塑性曲线',
    description: '在塑性区添加更多数据点，使材料模型更平滑',
    action: '在 *MATERIAL 中添加更多塑性数据点',
    expectedImprovement: '避免屈服转折处的数值不稳定性',
    riskLevel: 'low'
  },
  {
    id: 'plastic-hardening-model',
    scenario: 'material_nonlinearity',
    priority: 2,
    label: '使用等向强化模型',
    description: '如果模型含复杂屈服面，建议简化为等向强化',
    action: '设置 *PLASTIC 中 HARDENING=ISOTROPIC',
    expectedImprovement: '简化计算改善收敛',
    riskLevel: 'medium'
  },
  {
    id: 'creep-solid-size',
    scenario: 'material_nonlinearity',
    priority: 3,
    label: '设置蠕变单元类型',
    description: '对于蠕变分析，需明确指定 CREEP 单元属性',
    action: '在 *ELEMENT 中设置 SOLUTIOM=TYPE 或使用 CREEP 表',
    expectedImprovement: '正确处理时间相关变形',
    riskLevel: 'medium'
  },

  // 几何不稳定（屈曲/失稳）
  {
    id: 'buckle-preload',
    scenario: 'geometric_instability',
    priority: 1,
    label: '添加几何缺陷',
    description: '为屈曲分析添加小量几何缺陷（总厚的 1/1000），打破对称性',
    action: '使用 *IMPERFECTION 或手动扰动节点坐标',
    expectedImprovement: '消除理想结构的奇异性',
    riskLevel: 'medium'
  },
  {
    id: 'buckle-increment-control',
    scenario: 'geometric_instability',
    priority: 2,
    label: '精细增量步控制',
    description: '使用自动增量步，控制每个增量步的最小/最大迭代次数',
    action: '使用 *CONTROLS, PARAMETER=FIELD 或 AUTO=YES',
    expectedImprovement: '自动调整载荷增量适应结构响应',
    riskLevel: 'low'
  },
  {
    id: 'stabilization-factor',
    scenario: 'geometric_instability',
    priority: 3,
    label: '添加稳定因子',
    description: '对无载荷的自由度施加微小弹性约束，避免刚体位移',
    action: '添加 *STABILIZE 或 *CONSTRAINT 等效',
    expectedImprovement: '避免刚体模式导致的数值问题',
    riskLevel: 'medium'
  },

  // 载荷步过大
  {
    id: 'step-refine',
    scenario: 'load_step_too_large',
    priority: 1,
    label: '减小时间/载荷增量',
    description: '将分析步的总时间或总载荷减小，同时增加增量步数',
    action: '在 *STEP 中设置 TIME 或 DLOAD 参数',
    expectedImprovement: '更平滑的载荷施加路径',
    riskLevel: 'low'
  },
  {
    id: 'initial-delta',
    scenario: 'load_step_too_large',
    priority: 2,
    label: '减小初始增量步',
    description: '设置较小的初始增量（Initial Delta），避免初期过冲',
    action: '在 *STATIC 中设置 DIRECT 或 TIME 参数组合',
    expectedImprovement: '避免初期迭代发散',
    riskLevel: 'low'
  },

  // 病态方程组
  {
    id: 'solver-direct',
    scenario: 'ill_conditioned',
    priority: 1,
    label: '切换到直接求解器',
    description: '对于病态方程，改用直接求解器（SPOOLES）代替迭代求解器',
    action: '在 *STEP 中设置 SOLVER=SPOOLES',
    expectedImprovement: '直接求解器对病态系统更稳定',
    riskLevel: 'low'
  },
  {
    id: 'mesh-quality-improve',
    scenario: 'ill_conditioned',
    priority: 2,
    label: '改善网格质量',
    description: '提高网格整体质量，减少长宽比和歪斜度',
    action: '细化畸形单元，使用高阶单元或局部加密',
    expectedImprovement: '改善方程组条件数',
    riskLevel: 'medium'
  },
  {
    id: 'equation-scaling',
    scenario: 'ill_conditioned',
    priority: 3,
    label: '启用方程缩放',
    description: '启用方程缩放改善数值精度',
    action: '设置 *CONTROLS 或通过 Solver 选项启用',
    expectedImprovement: '改善舍入误差',
    riskLevel: 'low'
  },

  // 网格畸变
  {
    id: 'mesh-distortion-mapped',
    scenario: 'mesh_distortion',
    priority: 1,
    label: '使用映射网格',
    description: '对规则几何使用映射网格，避免自由网格的畸形单元',
    action: '在 *ELEMENT 中指定 ELTYPE 或使用 Mapped Meshing',
    expectedImprovement: '消除畸形单元',
    riskLevel: 'medium'
  },
  {
    id: 'mesh-adaptive',
    scenario: 'mesh_distortion',
    priority: 2,
    label: '自适应网格重划分',
    description: '开启网格重划分选项，在畸变超过阈值时自动重新划分',
    action: '添加 *ALE, MESH 或 *MESH, MOTION=ADAPTIVE',
    expectedImprovement: '避免大变形后的网格畸变',
    riskLevel: 'medium'
  },

  // 自由度过多
  {
    id: 'dof-reduce-mesh',
    scenario: 'excessive_dof',
    priority: 1,
    label: '粗化全局网格',
    description: '将全局网格尺寸增大 1.5-2 倍，减少总自由度数',
    action: '在 *ELEMENT 中增大 ELTYPE 对应的网格尺寸参数',
    expectedImprovement: '自由度减半，收敛更快速',
    riskLevel: 'medium'
  },
  {
    id: 'dof-submodel',
    scenario: 'excessive_dof',
    priority: 2,
    label: '使用子模型技术',
    description: '仅对关注区域精细建模，通过子模型边界传递位移',
    action: '使用 *SUBMODEL 和 *STEP, SUBSTRACTION',
    expectedImprovement: '减少计算量同时保持精度',
    riskLevel: 'high'
  },
  {
    id: 'dof-superelement',
    scenario: 'excessive_dof',
    priority: 3,
    label: '使用超单元',
    description: '将低应力区域的网格凝聚为超单元，减少自由度',
    action: '使用 *SUBSTRATE 和 *SE GENERATE',
    expectedImprovement: '大幅减少自由度',
    riskLevel: 'high'
  },

  // 热刚化/热软化
  {
    id: 'thermal-coupled-step',
    scenario: 'thermal_stiffening',
    priority: 1,
    label: '使用耦合分析步',
    description: '对于热-结构耦合问题，使用 COUPLED 类型分析步',
    action: '使用 *COUPLED TEMPERATURE-DISPLACEMENT',
    expectedImprovement: '正确处理热应力',
    riskLevel: 'low'
  },
  {
    id: 'thermal-refine-dt',
    scenario: 'thermal_stiffening',
    priority: 2,
    label: '细化热分析增量步',
    description: '减小温度变化速率，避免温度梯度过大',
    action: '在 *HEAT TRANSFER 中使用更小的 TIME',
    expectedImprovement: '平滑温度场变化',
    riskLevel: 'low'
  },

  // 混合收敛问题
  {
    id: 'mixed-increment-control',
    scenario: 'mixed_convergence',
    priority: 1,
    label: '精细增量步控制',
    description: '启用自动增量步，允许求解器根据收敛情况自适应调整',
    action: '在 *STATIC 中设置 DIRECT 或 AUTO=YES',
    expectedImprovement: '求解器自动寻找最佳载荷增量',
    riskLevel: 'low'
  },
  {
    id: 'mixed-increase-iterations',
    scenario: 'mixed_convergence',
    priority: 2,
    label: '增加迭代次数上限',
    description: '如果迭代次数不足，增加每个增量步的最大迭代次数',
    action: '在 *CONTROLS 中设置 ITERATIONS 参数',
    expectedImprovement: '给予非线性求解更多机会',
    riskLevel: 'low'
  },
  {
    id: 'mixed-relaxation',
    scenario: 'mixed_convergence',
    priority: 3,
    label: '应用松弛因子',
    description: '对位移/力应用松弛因子，减缓更新速率',
    action: '在 *CONTROLS 中设置 RELAXATION 参数',
    expectedImprovement: '改善迭代稳定性',
    riskLevel: 'medium'
  }
]

export function useConvergenceDiagnostics() {
  const isAnalyzing = ref(false)
  const lastDiag = ref<ConvergenceDiagRecord | null>(null)
  const history = ref<ConvergenceHistoryPoint[]>([])

  /**
   * 分析收敛失败原因
   * @param errorMessage 求解器返回的错误信息
   * @param residualHistory 残差历史 [step0_iter0, step0_iter1, ...]
   * @param jobId 任务ID
   */
  async function analyzeConvergenceFailure(
    errorMessage: string,
    residualHistory: number[] = [],
    jobId?: string
  ): Promise<RepairRecommendation> {
    isAnalyzing.value = true

    try {
      // 解析错误信息，识别场景
      const scenarios = detectScenarios(errorMessage, residualHistory)

      // 获取主要场景的修复策略
      const primaryScenario = scenarios[0]
      const primaryStrategies = REPAIR_STRATEGIES.filter(s => s.scenario === primaryScenario)
        .sort((a, b) => a.priority - b.priority)

      if (primaryStrategies.length === 0) {
        // 回退到默认策略
        return {
          primary: {
            id: 'fallback-diagnostic',
            scenario: 'mixed_convergence',
            priority: 1,
            label: '建议手动诊断',
            description: '无法自动识别问题类型，建议检查以下内容：1) 网格质量；2) 边界条件完整性；3) 材料属性正确性',
            action: '手动审查模型设置',
            expectedImprovement: '取决于手动修正',
            riskLevel: 'medium'
          },
          alternatives: [],
          autoApplyAvailable: false,
          estimatedFixProbability: 0.1
        }
      }

      const primary = primaryStrategies[0]
      const alternatives = primaryStrategies.slice(1, 4)

      // 计算修复概率
      const fixProbability = estimateFixProbability(primary, scenarios)

      // 记录诊断
      const record: ConvergenceDiagRecord = {
        timestamp: new Date().toISOString(),
        jobId: jobId || `diag-${Date.now()}`,
        scenario: primaryScenario,
        residualHistory: residualHistory.slice(-50),
        incrementHistory: [], // 增量历史暂未收集
        iterationCount: residualHistory.length,
        lastError: errorMessage,
        appliedStrategies: []
      }
      lastDiag.value = record

      // 生成组合策略说明
      let combinedStrategy: string | undefined
      if (scenarios.length > 1) {
        combinedStrategy = `检测到多问题（${scenarios.map(s => scenarioLabel(s)).join(' + ')}），建议综合应用修复策略`
      }

      return {
        primary,
        alternatives,
        autoApplyAvailable: primary.inpModifier !== undefined,
        estimatedFixProbability: fixProbability,
        combinedStrategy
      }
    } finally {
      isAnalyzing.value = false
    }
  }

  /**
   * 检测收敛失败的场景类型
   */
  function detectScenarios(
    errorMessage: string,
    residualHistory: number[]
  ): ConvergenceScenario[] {
    const scenarios: ConvergenceScenario[] = []
    const msg = errorMessage.toLowerCase()

    // 基于错误消息关键词识别
    if (msg.includes('contact') || msg.includes('penetration') || msg.includes('overclosure')) {
      scenarios.push('contact_penetration')
    }
    if (msg.includes('large strain') || msg.includes('large deformation') || msg.includes('nlgeom')) {
      scenarios.push('large_strain')
    }
    if (msg.includes('plastic') || msg.includes('yield') || msg.includes('creep')) {
      scenarios.push('material_nonlinearity')
    }
    if (msg.includes('buckling') || msg.includes('instability') || msg.includes('negative pivot')) {
      scenarios.push('geometric_instability')
    }
    if (msg.includes('load') && (msg.includes('step') || msg.includes('increment'))) {
      scenarios.push('load_step_too_large')
    }
    if (msg.includes('ill') || msg.includes('condition') || msg.includes('pivot')) {
      scenarios.push('ill_conditioned')
    }
    if (msg.includes('distort') || msg.includes('jacobian') || msg.includes('element')) {
      scenarios.push('mesh_distortion')
    }
    if (msg.includes('dof') || msg.includes('degree') || msg.includes('memory')) {
      scenarios.push('excessive_dof')
    }
    if (msg.includes('thermal') || msg.includes('temperature') || msg.includes('heat')) {
      scenarios.push('thermal_stiffening')
    }
    if (msg.includes('converge') && scenarios.length === 0) {
      scenarios.push('mixed_convergence')
    }

    // 基于残差历史模式识别
    if (scenarios.length === 0 && residualHistory.length > 5) {
      const lastFew = residualHistory.slice(-5)
      const maxRatio = Math.max(...lastFew.map((r, i) => r / (residualHistory[i - 1] || 1)))
      if (maxRatio > 1.5) {
        scenarios.push('load_step_too_large')
      }
      // 检查是否震荡
      const oscillations = lastFew.filter((r, i) => i > 0 && (r > lastFew[i - 1] * 1.2 || r < lastFew[i - 1] * 0.8)).length
      if (oscillations >= 3) {
        scenarios.push('mixed_convergence')
      }
    }

    // 默认回退
    if (scenarios.length === 0) {
      scenarios.push('mixed_convergence')
    }

    return scenarios
  }

  /**
   * 估算修复概率
   */
  function estimateFixProbability(
    strategy: RepairStrategy,
    _scenarios: ConvergenceScenario[]
  ): number {
    // 基础概率
    let prob = 0.6

    // 根据风险等级调整
    switch (strategy.riskLevel) {
      case 'low': prob += 0.2; break
      case 'medium': break
      case 'high': prob -= 0.15; break
    }

    // 根据优先级调整
    switch (strategy.priority) {
      case 1: prob += 0.1; break
      case 2: prob -= 0.05; break
      case 3: prob -= 0.1; break
    }

    return Math.min(0.95, Math.max(0.2, prob))
  }

  /**
   * 获取指定场景的所有策略
   */
  function getStrategiesForScenario(scenario: ConvergenceScenario): RepairStrategy[] {
    return REPAIR_STRATEGIES.filter(s => s.scenario === scenario)
      .sort((a, b) => a.priority - b.priority)
  }

  /**
   * 获取所有场景的策略映射
   */
  function getAllStrategies(): Map<ConvergenceScenario, RepairStrategy[]> {
    const map = new Map<ConvergenceScenario, RepairStrategy[]>()
    for (const strategy of REPAIR_STRATEGIES) {
      const list = map.get(strategy.scenario) || []
      list.push(strategy)
      map.set(strategy.scenario, list)
    }
    return map
  }

  /**
   * 生成诊断报告
   */
  function generateDiagReport(recommendation: RepairRecommendation): string {
    const lines: string[] = [
      '# 收敛诊断报告',
      '',
      `**时间**: ${new Date().toLocaleString('zh-CN')}`,
      `**场景**: ${scenarioLabel(recommendation.primary.scenario)}`,
      `**推荐策略**: ${recommendation.primary.label}`,
      `**修复概率**: ${(recommendation.estimatedFixProbability * 100).toFixed(0)}%`,
      ''
    ]

    lines.push('## 问题描述', '')
    lines.push(recommendation.primary.description, '')
    lines.push('## 建议操作', '')
    lines.push(recommendation.primary.action, '')

    if (recommendation.alternatives.length > 0) {
      lines.push('')
      lines.push('## 备选方案', '')
      for (const alt of recommendation.alternatives) {
        lines.push(`### ${alt.label}`)
        lines.push(alt.description)
        lines.push(`操作: ${alt.action}`)
        lines.push('')
      }
    }

    if (recommendation.combinedStrategy) {
      lines.push('## 综合建议', '')
      lines.push(recommendation.combinedStrategy, '')
    }

    lines.push('---')
    lines.push('*本报告由 CAELab 收敛诊断系统自动生成*')

    return lines.join('\n')
  }

  /**
   * 场景标签（中英文）
   */
  function scenarioLabel(scenario: ConvergenceScenario): string {
    const map: Record<ConvergenceScenario, string> = {
      contact_penetration: '接触穿透',
      large_strain: '大应变/大变形',
      material_nonlinearity: '材料非线性',
      geometric_instability: '几何不稳定',
      thermal_stiffening: '热刚化/热软化',
      mesh_distortion: '网格畸变',
      ill_conditioned: '病态方程组',
      load_step_too_large: '载荷步过大',
      excessive_dof: '自由度过多',
      mixed_convergence: '混合收敛问题'
    }
    return map[scenario] || scenario
  }

  /**
   * 解析收敛历史并生成可视化数据
   */
  function parseConvergenceHistory(
    rawData: Array<{ step: number; iter: number; residual: number; increment: number }>
  ): ConvergenceHistoryPoint[] {
    return rawData.map(p => ({
      step: p.step,
      iteration: p.iter,
      residual: p.residual,
      increment: p.increment,
      ratio: p.residual > 0 ? p.increment / p.residual : 0
    }))
  }

  return {
    isAnalyzing,
    lastDiag,
    history,
    analyzeConvergenceFailure,
    detectScenarios,
    getStrategiesForScenario,
    getAllStrategies,
    generateDiagReport,
    scenarioLabel,
    parseConvergenceHistory,
    REPAIR_STRATEGIES
  }
}