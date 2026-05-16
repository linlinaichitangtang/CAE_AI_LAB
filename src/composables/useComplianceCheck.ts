/**
 * useComplianceCheck.ts — V4.1-001 自动合规检查引擎
 * 仿真完成后自动比对设计规范，判定合规性
 */
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface ComplianceRule {
  standardId: string
  standardName: string
  clause: string
  description: string
  checkType: 'stress' | 'displacement' | 'fatigue' | 'buckling'
  operator: '<' | '>' | '<=' | '>='
  threshold: number
  unit: string
}

export interface ComplianceResult {
  ruleId: string
  rule: ComplianceRule
  actualValue: number
  passed: boolean
  margin: number // 安全裕度 (正数表示安全，负数表示超标)
  severity: 'critical' | 'warning' | 'info'
  message: string
}

export interface ComplianceReport {
  simulationId: string
  timestamp: string
  overallStatus: 'pass' | 'fail' | 'partial'
  results: ComplianceResult[]
  summary: {
    total: number
    passed: number
    failed: number
    warnings: number
  }
}

// 内置合规规则库
const BUILT_IN_RULES: ComplianceRule[] = [
  {
    standardId: 'BMS7-368E',
    standardName: 'BMS7-368E',
    clause: '4.2',
    description: '最大 von Mises 应力应小于许用应力 / 安全系数',
    checkType: 'stress',
    operator: '<',
    threshold: 147, // 220 / 1.5
    unit: 'MPa'
  },
  {
    standardId: 'BMS7-368E',
    standardName: 'BMS7-368E',
    clause: '5.1',
    description: '疲劳极限安全系数 ≥ 1.5',
    checkType: 'fatigue',
    operator: '>=',
    threshold: 1.5,
    unit: ''
  },
  {
    standardId: 'VDA-2300',
    standardName: 'VDA 2300',
    clause: '3.1',
    description: '最大应力应小于屈服强度的 80%',
    checkType: 'stress',
    operator: '<',
    threshold: 276, // 345 * 0.8
    unit: 'MPa'
  },
  {
    standardId: 'GB/T-6398',
    standardName: 'GB/T 6398-2024',
    clause: '6.3',
    description: '结构安全系数应 ≥ 1.4',
    checkType: 'stress',
    operator: '>=',
    threshold: 1.4,
    unit: ''
  },
  {
    standardId: 'GB/T-50017',
    standardName: 'GB/T 50017',
    clause: '4.1',
    description: '钢结构最大挠度 ≤ L/250',
    checkType: 'displacement',
    operator: '<=',
    threshold: 4, // 1000/250
    unit: 'mm'
  }
]

export function useComplianceCheck() {
  const isChecking = ref(false)
  const lastReport = ref<ComplianceReport | null>(null)
  const activeStandards = ref<string[]>(['BMS7-368E', 'VDA-2300', 'GB/T-6398'])

  // 根据启用的标准筛选规则
  const activeRules = computed(() =>
    BUILT_IN_RULES.filter(rule => activeStandards.value.includes(rule.standardId))
  )

  /**
   * 执行合规检查
   * @param simulationResults 仿真结果数据
   * @param materialProps 材料属性
   */
  async function runComplianceCheck(
    simulationResults: {
      maxVonMises?: number
      maxDisplacement?: number
      safetyFactor?: number
      maxStress?: number
      fatigueSafetyFactor?: number
    },
    materialProps?: {
      yieldStrength?: number
      ultimateStrength?: number
      elasticModulus?: number
    }
  ): Promise<ComplianceReport> {
    isChecking.value = true

    try {
      // 尝试调用后端合规检查引擎
      const backendResult = await invoke<ComplianceReport>('run_compliance_check', {
        results: simulationResults,
        material: materialProps,
        standards: activeStandards.value
      }).catch(() => null)

      if (backendResult) {
        lastReport.value = backendResult
        return backendResult
      }
    } catch {
      // 后端不可用，使用前端本地检查
    }

    // 前端本地合规检查
    const results: ComplianceResult[] = []

    for (const rule of activeRules.value) {
      let actualValue: number | undefined

      switch (rule.checkType) {
        case 'stress':
          actualValue = simulationResults.maxVonMises ?? simulationResults.maxStress
          break
        case 'displacement':
          actualValue = simulationResults.maxDisplacement
          break
        case 'fatigue':
          actualValue = simulationResults.fatigueSafetyFactor ?? simulationResults.safetyFactor
          break
        case 'buckling':
          actualValue = simulationResults.safetyFactor
          break
      }

      if (actualValue === undefined) continue

      let passed = false
      let margin = 0

      switch (rule.operator) {
        case '<':
          passed = actualValue < rule.threshold
          margin = (rule.threshold - actualValue) / rule.threshold
          break
        case '>':
          passed = actualValue > rule.threshold
          margin = (actualValue - rule.threshold) / rule.threshold
          break
        case '<=':
          passed = actualValue <= rule.threshold
          margin = (rule.threshold - actualValue) / rule.threshold
          break
        case '>=':
          passed = actualValue >= rule.threshold
          margin = (actualValue - rule.threshold) / rule.threshold
          break
      }

      const severity: 'critical' | 'warning' | 'info' = !passed
        ? (margin < -0.2 ? 'critical' : 'warning')
        : 'info'

      results.push({
        ruleId: `${rule.standardId}-${rule.clause}`,
        rule,
        actualValue,
        passed,
        margin,
        severity,
        message: passed
          ? `✅ 符合 ${rule.standardName} 条款 ${rule.clause}: ${actualValue.toFixed(1)} ${rule.unit} ${rule.operator} ${rule.threshold} ${rule.unit} (裕度: ${(margin * 100).toFixed(1)}%)`
          : `❌ 不满足 ${rule.standardName} 条款 ${rule.clause}: ${actualValue.toFixed(1)} ${rule.unit} 不满足 ${rule.operator} ${rule.threshold} ${rule.unit} (超标: ${Math.abs(margin * 100).toFixed(1)}%)`
      })
    }

    const failedCount = results.filter(r => !r.passed).length
    const warningCount = results.filter(r => r.passed && r.margin < 0.1).length
    const passedCount = results.filter(r => r.passed && r.margin >= 0.1).length

    const overallStatus: 'pass' | 'fail' | 'partial' =
      failedCount === 0 ? 'pass' : (passedCount === 0 ? 'fail' : 'partial')

    const report: ComplianceReport = {
      simulationId: `sim_${Date.now()}`,
      timestamp: new Date().toISOString(),
      overallStatus,
      results,
      summary: {
        total: results.length,
        passed: passedCount,
        failed: failedCount,
        warnings: warningCount
      }
    }

    lastReport.value = report
    isChecking.value = false
    return report
  }

  /**
   * 获取特定标准的规则
   */
  function getRulesByStandard(standardId: string): ComplianceRule[] {
    return BUILT_IN_RULES.filter(rule => rule.standardId === standardId)
  }

  /**
   * 添加自定义规则
   */
  function addCustomRule(rule: Omit<ComplianceRule, 'standardId'> & { standardId: string }): void {
    BUILT_IN_RULES.push(rule as ComplianceRule)
  }

  /**
   * 启用/禁用标准
   */
  function toggleStandard(standardId: string): void {
    const idx = activeStandards.value.indexOf(standardId)
    if (idx >= 0) {
      activeStandards.value.splice(idx, 1)
    } else {
      activeStandards.value.push(standardId)
    }
  }

  return {
    isChecking,
    lastReport,
    activeStandards,
    activeRules,
    runComplianceCheck,
    getRulesByStandard,
    addCustomRule,
    toggleStandard
  }
}
