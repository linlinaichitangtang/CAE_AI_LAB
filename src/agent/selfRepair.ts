/**
 * V2.4-015 自主修复机制
 * 验证失败时，Agent 分析原因并尝试修复（如调整网格密度/修改载荷）
 * 需要 LLM 理解错误日志
 */

import type { ToolResult, RepairStrategy, SubTask } from './types'
import { resultVerifier } from './resultVerifier'

/** 自修复策略库 */
const REPAIR_STRATEGIES: Record<string, (error: string, params: Record<string, unknown>) => RepairStrategy[]> = {
  'run_simulation': (error, params) => {
    const strategies: RepairStrategy[] = []

    if (error.includes('不收敛') || error.includes('not converged') || error.includes('convergence')) {
      strategies.push({
        description: '增加最大迭代次数',
        toolName: 'run_simulation',
        params: { ...params, maxIterations: ((params.maxIterations as number) || 100) * 2 },
        reason: '求解未收敛，尝试增加迭代次数'
      })
      strategies.push({
        description: '细化网格',
        toolName: 'generate_mesh',
        params: { meshType: 'structured', elementSize: 0.005 },
        reason: '求解不收敛可能是网格太粗导致'
      })
    }

    if (error.includes('singular') || error.includes('singularity') || error.includes('刚体位移')) {
      strategies.push({
        description: '检查并补充边界条件',
        toolName: 'apply_bc',
        params: { bcType: 'fixed', face: 'bottom', values: {} },
        reason: '可能存在刚体位移，需要补充约束'
      })
    }

    if (error.includes('材料') || error.includes('material')) {
      strategies.push({
        description: '重新设置材料属性',
        toolName: 'set_material',
        params: { materialName: 'Q235' },
        reason: '材料设置可能有误，使用默认材料重试'
      })
    }

    // 默认策略
    if (strategies.length === 0) {
      strategies.push({
        description: '使用默认参数重试',
        toolName: 'run_simulation',
        params: { ...params, analysisType: 'static', maxIterations: 200 },
        reason: '使用更保守的参数重试'
      })
    }

    return strategies
  },

  'generate_mesh': (error, params) => {
    const strategies: RepairStrategy[] = []

    if (error.includes('质量') || error.includes('quality')) {
      strategies.push({
        description: '使用更小的单元尺寸',
        toolName: 'generate_mesh',
        params: { ...params, meshType: 'structured', elementSize: ((params.elementSize as number) || 0.01) * 0.5 },
        reason: '网格质量不满足要求，尝试减小单元尺寸'
      })
    }

    strategies.push({
      description: '切换为非结构化网格',
      toolName: 'generate_mesh',
      params: { ...params, meshType: 'tetrahedral' },
      reason: '结构化网格可能不适用当前几何，尝试非结构化网格'
    })

    return strategies
  },

  'get_results': (_error, params) => {
    return [{
      description: '重新获取结果',
      toolName: 'get_results',
      params: { ...params, resultType: 'all' },
      reason: '尝试获取全部结果数据'
    }]
  },

  'apply_bc': (_error, _params) => {
    return [{
      description: '使用默认边界条件',
      toolName: 'apply_bc',
      params: { bcType: 'fixed', face: 'left', values: {} },
      reason: '边界条件设置失败，使用默认固定约束'
    }]
  }
}

/** 自修复引擎 */
export class SelfRepairEngine {
  /**
   * 分析失败原因并生成修复策略
   */
  analyzeFailure(
    toolName: string,
    toolResult: ToolResult,
    subTask: SubTask
  ): RepairStrategy[] {
    const error = toolResult.error || '未知错误'

    // 1. 先验证结果
    const validation = resultVerifier.verify(toolResult, toolName)
    if (validation.passed) {
      return []  // 结果验证通过，无需修复
    }

    // 2. 查找匹配的修复策略
    const strategyGenerator = REPAIR_STRATEGIES[toolName]
    if (strategyGenerator) {
      const strategies = strategyGenerator(error, subTask.toolParams || {})
      return strategies.slice(0, 3)  // 最多返回3个策略
    }

    // 3. 通用修复策略
    return [{
      description: '使用默认参数重试',
      toolName,
      params: subTask.toolParams || {},
      reason: `工具执行失败: ${error}，使用默认参数重试`
    }]
  }

  /**
   * 应用修复策略（修改子任务参数）
   */
  applyStrategy(subTask: SubTask, strategy: RepairStrategy): SubTask {
    return {
      ...subTask,
      toolParams: { ...subTask.toolParams, ...strategy.params },
      toolName: strategy.toolName,
      status: 'pending',
      retryCount: subTask.retryCount,
      error: undefined,
      result: undefined
    }
  }
}

/** 全局自修复引擎实例 */
export const selfRepairEngine = new SelfRepairEngine()
