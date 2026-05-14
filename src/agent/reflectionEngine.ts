/**
 * reflectionEngine.ts — V2.4 反思引擎
 *
 * Claude Code 对齐：每步执行后评估结果质量，
 * confidence < threshold 时触发重规划
 */

import type { ToolResult, ValidationResult, ToolCallRecord, ReflectionResult, ToolReflection } from './types'

// 工程规则校验得分加成
const RULE_PASS_BONUS = 0.1
const RULE_FAIL_PENALTY = 0.2

// 历史模式加成（同类工具最近3+次成功）
const HISTORY_SUCCESS_BONUS = 0.1
const HISTORY_THRESHOLD = 3

// 结果合理性加成（von_mises < 0.8 × yield 等）
const RESULT_REASONABLENESS_BONUS = 0.1

// 重规划阈值
const REPLAN_THRESHOLD = 0.6

export class ReflectionEngine {
  private toolRepeatCount: Map<string, number> = new Map()

  /**
   * 反思单个工具执行结果
   */
  reflect(reflection: ToolReflection): ReflectionResult {
    const { toolName, result, validation, historicalContext } = reflection

    // 基础分：validation 通过给 0.7，不通过给 0
    let confidence = validation.passed ? 0.7 : 0.0

    // 规则检查加分/减分
    const { rules } = validation
    let passedRules = 0
    let failedRules = 0
    for (const r of rules) {
      if (r.passed) passedRules++
      else failedRules++
    }
    confidence += passedRules * RULE_PASS_BONUS
    confidence -= failedRules * RULE_FAIL_PENALTY

    // 历史同类成功加成
    const recentSameTool = this.getRecentSameToolCalls(toolName, historicalContext)
    if (recentSameTool >= HISTORY_THRESHOLD) {
      confidence += HISTORY_SUCCESS_BONUS
    }

    // 结果合理性加成（仅对成功结果）
    if (result.success && this.isResultReasonable(result)) {
      confidence += RESULT_REASONABLENESS_BONUS
    }

    // 置信度上限 1.0
    confidence = Math.min(confidence, 1.0)

    // 判断质量等级
    let quality: ReflectionResult['quality']
    if (confidence >= 0.9) quality = 'excellent'
    else if (confidence >= 0.7) quality = 'good'
    else if (confidence >= 0.5) quality = 'marginal'
    else quality = 'poor'

    // 是否需要重规划
    const shouldReplan = confidence < REPLAN_THRESHOLD

    // 生成建议
    const suggestions = this.generateSuggestions(toolName, result, validation, confidence)

    return {
      confidence,
      quality,
      shouldReplan,
      reason: this.buildReason(toolName, confidence, validation, recentSameTool),
      suggestions,
    }
  }

  /**
   * 重置工具重复计数（每次新任务开始时调用）
   */
  resetRepeatCount(): void {
    this.toolRepeatCount.clear()
  }

  /**
   * 递增工具重复计数，返回当前累计值
   */
  incrementRepeatCount(toolName: string): number {
    const count = (this.toolRepeatCount.get(toolName) || 0) + 1
    this.toolRepeatCount.set(toolName, count)
    return count
  }

  /**
   * 获取某工具最近的成功调用次数
   */
  private getRecentSameToolCalls(toolName: string, history: ToolCallRecord[]): number {
    const recent = history.slice(-10)
    let count = 0
    for (const record of recent) {
      if (record.toolName === toolName && record.result.success) count++
    }
    return count
  }

  /**
   * 判断结果是否合理（工程意义上）
   */
  private isResultReasonable(result: ToolResult): boolean {
    if (!result.data) return false

    // 仿真结果合理性检查
    const data = result.data as Record<string, unknown>
    if ('von_mises' in data || 'max_stress' in data || 'stress' in data) {
      const stress = Number(data.von_mises ?? data.max_stress ?? data.stress ?? 0)
      if (stress > 1e12) return false // 应力超过 1 TPa 不合理（材料屈服一般在 GPa 以内）
    }

    // 位移结果合理性检查
    if ('max_displacement' in data || 'displacement' in data) {
      const disp = Number(data.max_displacement ?? data.displacement ?? 0)
      if (disp > 100) return false // 位移超过 100m 不合理
    }

    return true
  }

  /**
   * 生成改进建议
   */
  private generateSuggestions(
    toolName: string,
    result: ToolResult,
    validation: ValidationResult,
    confidence: number
  ): string[] {
    const suggestions: string[] = []

    if (!result.success) {
      suggestions.push(`考虑检查 ${toolName} 的参数是否正确`)
      suggestions.push('可以尝试使用更稳健的替代工具')
    }

    if (!validation.passed) {
      const failedRules = validation.rules.filter(r => !r.passed)
      for (const rule of failedRules) {
        suggestions.push(`规则 "${rule.message}" 未通过，建议调整参数`)
      }
    }

    if (confidence < 0.5) {
      suggestions.push('置信度过低，建议重新分析问题')
    } else if (confidence < 0.7) {
      suggestions.push('结果处于边缘区间，建议验证输入数据')
    }

    return suggestions
  }

  /**
   * 构建原因字符串
   */
  private buildReason(
    toolName: string,
    confidence: number,
    validation: ValidationResult,
    recentSameTool: number
  ): string {
    const parts: string[] = []

    parts.push(`工具 ${toolName} 执行`)
    parts.push(`置信度 ${(confidence * 100).toFixed(0)}%`)

    if (validation.passed) {
      parts.push('验证通过')
    } else {
      const failedCount = validation.rules.filter(r => !r.passed).length
      parts.push(`验证失败 (${failedCount} 条规则)`)
    }

    if (recentSameTool >= HISTORY_THRESHOLD) {
      parts.push('历史同类成功')
    }

    return parts.join(' / ')
  }
}