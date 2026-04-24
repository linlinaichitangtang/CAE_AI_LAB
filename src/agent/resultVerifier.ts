/**
 * V2.4-013 结果验证器
 * 检查仿真结果数值范围（如 Von Mises stress < 10 × yield stress）、格式完整性
 * 规则 + LLM 结合
 */

import type { ValidationResult, ValidationRule, ToolResult } from './types'

/** 工程验证规则库 */
const ENGINEERING_RULES: ValidationRule[] = [
  {
    field: 'maxVonMises',
    check: 'range',
    params: { min: 0, max: 10e9, unit: 'Pa' },
    message: 'Von Mises 应力应在 0 ~ 10 GPa 范围内'
  },
  {
    field: 'maxDisplacement',
    check: 'range',
    params: { min: 0, max: 1.0, unit: 'm' },
    message: '最大位移应在 0 ~ 1 m 范围内'
  },
  {
    field: 'convergence',
    check: 'not_empty',
    params: {},
    message: '求解应收敛'
  },
  {
    field: 'meshQuality',
    check: 'range',
    params: { field: 'minJacobian', min: 0.3 },
    message: '网格最小雅可比比应 > 0.3'
  },
  {
    field: 'reactionForce',
    check: 'range',
    params: { min: -1e12, max: 1e12, unit: 'N' },
    message: '反力应在合理范围内'
  }
]

/** 结果验证器 */
export class ResultVerifier {
  /**
   * 验证工具执行结果
   */
  verify(toolResult: ToolResult, toolName: string): ValidationResult {
    if (!toolResult.success) {
      return {
        passed: false,
        rules: [{
          rule: { field: 'execution', check: 'not_empty', params: {}, message: '工具执行失败' },
          passed: false,
          message: `工具 "${toolName}" 执行失败: ${toolResult.error || '未知错误'}`
        }],
        summary: `工具 "${toolName}" 执行失败`
      }
    }

    if (!toolResult.data || typeof toolResult.data !== 'object') {
      return {
        passed: true,
        rules: [],
        summary: '结果格式为基础类型，无需深度验证'
      }
    }

    const data = toolResult.data as Record<string, unknown>
    const results: ValidationResult['rules'] = []

    // 根据工具类型选择验证规则
    switch (toolName) {
      case 'run_simulation':
        results.push(...this.validateSimulationResult(data))
        break
      case 'get_results':
        results.push(...this.validateResultData(data))
        break
      case 'generate_mesh':
      case 'check_mesh_quality':
        results.push(...this.validateMeshQuality(data))
        break
      case 'apply_bc':
      case 'set_material':
        results.push(...this.validateSetupResult(data))
        break
      default:
        // 通用验证：检查数据非空
        results.push({
          rule: { field: 'data', check: 'not_empty', params: {}, message: '返回数据不应为空' },
          passed: Object.keys(data).length > 0,
          actualValue: Object.keys(data).length,
          message: Object.keys(data).length > 0 ? '数据非空' : '返回数据为空'
        })
    }

    const allPassed = results.every(r => r.passed)
    return {
      passed: allPassed,
      rules: results,
      summary: allPassed
        ? `验证通过 (${results.length} 项检查)`
        : `验证失败 (${results.filter(r => !r.passed).length}/${results.length} 项不通过)`
    }
  }

  /**
   * 验证仿真结果
   */
  private validateSimulationResult(data: Record<string, unknown>): ValidationResult['rules'] {
    const results: ValidationResult['rules'] = []

    // 检查收敛性
    const convergence = data.convergence
    results.push({
      rule: { field: 'convergence', check: 'not_empty', params: {}, message: '求解应收敛' },
      passed: convergence === true,
      actualValue: convergence,
      message: convergence === true ? '求解已收敛' : '求解未收敛，结果可能不可靠'
    })

    // 检查迭代次数合理性
    const iterations = data.iterations as number | undefined
    if (iterations !== undefined) {
      results.push({
        rule: { field: 'iterations', check: 'range', params: { min: 1, max: 1000 }, message: '迭代次数应在合理范围内' },
        passed: iterations > 0 && iterations < 1000,
        actualValue: iterations,
        message: iterations > 0 && iterations < 1000
          ? `迭代 ${iterations} 次，正常`
          : `迭代 ${iterations} 次，可能异常`
      })
    }

    return results
  }

  /**
   * 验证结果数据
   */
  private validateResultData(data: Record<string, unknown>): ValidationResult['rules'] {
    const results: ValidationResult['rules'] = []

    // 检查 Von Mises 应力
    const maxVonMises = data.maxVonMises as number | undefined
    if (maxVonMises !== undefined) {
      const isPositive = maxVonMises > 0
      const isReasonable = maxVonMises < 10e9  // < 10 GPa
      results.push({
        rule: ENGINEERING_RULES[0],
        passed: isPositive && isReasonable,
        actualValue: maxVonMises,
        message: isPositive && isReasonable
          ? `Von Mises 应力 ${(maxVonMises / 1e6).toFixed(1)} MPa，合理`
          : `Von Mises 应力 ${(maxVonMises / 1e6).toFixed(1)} MPa，可能异常`
      })
    }

    // 检查位移
    const maxDisplacement = data.maxDisplacement as number | undefined
    if (maxDisplacement !== undefined) {
      const isReasonable = Math.abs(maxDisplacement) < 1.0  // < 1m
      results.push({
        rule: ENGINEERING_RULES[1],
        passed: isReasonable,
        actualValue: maxDisplacement,
        message: isReasonable
          ? `最大位移 ${(maxDisplacement * 1000).toFixed(3)} mm，合理`
          : `最大位移 ${(maxDisplacement * 1000).toFixed(3)} mm，可能异常`
      })
    }

    return results
  }

  /**
   * 验证网格质量
   */
  private validateMeshQuality(data: Record<string, unknown>): ValidationResult['rules'] {
    const results: ValidationResult['rules'] = []

    // 检查节点和单元数
    const nodes = data.nodes as number | undefined
    const elements = data.elements as number | undefined
    if (nodes !== undefined && elements !== undefined) {
      const isPositive = nodes > 0 && elements > 0
      const ratio = nodes / elements
      results.push({
        rule: { field: 'mesh_size', check: 'not_empty', params: {}, message: '网格应有有效节点和单元' },
        passed: isPositive,
        actualValue: { nodes, elements },
        message: isPositive
          ? `网格: ${nodes} 节点, ${elements} 单元 (比 ${ratio.toFixed(1)})`
          : '网格数据异常'
      })
    }

    // 检查质量指标
    const quality = data.quality as Record<string, unknown> | undefined
    if (quality) {
      const minJac = quality.minJacobian as number | undefined
      if (minJac !== undefined) {
        results.push({
          rule: ENGINEERING_RULES[3],
          passed: minJac > 0.3,
          actualValue: minJac,
          message: minJac > 0.3
            ? `最小雅可比比 ${minJac.toFixed(2)}，合格`
            : `最小雅可比比 ${minJac.toFixed(2)}，建议优化网格`
        })
      }
    }

    // 检查质量检查结果
    const passed = data.passed as boolean | undefined
    if (passed !== undefined) {
      results.push({
        rule: { field: 'quality_check', check: 'not_empty', params: {}, message: '网格质量检查应通过' },
        passed: passed,
        actualValue: passed,
        message: passed ? '网格质量检查通过' : '网格质量检查未通过'
      })
    }

    return results
  }

  /**
   * 验证设置结果
   */
  private validateSetupResult(data: Record<string, unknown>): ValidationResult['rules'] {
    const results: ValidationResult['rules'] = []

    const applied = data.applied as boolean | undefined
    const configured = data.configured as boolean | undefined
    const created = data.created as boolean | undefined

    const success = applied === true || configured === true || created === true
    results.push({
      rule: { field: 'setup', check: 'not_empty', params: {}, message: '设置应成功应用' },
      passed: success,
      actualValue: { applied, configured, created },
      message: success ? '设置已成功应用' : '设置可能未正确应用'
    })

    return results
  }
}

/** 全局结果验证器实例 */
export const resultVerifier = new ResultVerifier()
