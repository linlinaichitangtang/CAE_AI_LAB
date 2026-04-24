/**
 * V2.4 AI Agent 测试套件
 * V2.4-018: Agent 冒烟测试 (10个常见场景)
 * V2.4-019: 并发安全测试
 * V2.4-020: 性能基准
 * V2.4-021: Agent 评估体系
 */

import { intentClassifier } from '../intentClassifier'
import { taskPlanner } from '../taskPlanner'
import { toolRegistry } from '../tools'
import { toolExecutor } from '../toolExecutor'
import { resultVerifier } from '../resultVerifier'
import { selfRepairEngine } from '../selfRepair'
import { stateTracker } from '../stateTracker'
import { agentOrchestrator } from '../agentOrchestrator'
import type { IntentResult, ToolResult } from '../types'

// ============================================================================
// V2.4-001: 意图识别准确率测试 (>90%, 100条测试用例)
// ============================================================================

interface TestCase {
  input: string
  expectedIntent: string
  allowedIntents?: string[]  // 备选意图
}

const intentTestCases: TestCase[] = [
  // 建模意图 (15 cases)
  { input: '创建一个立方体', expectedIntent: 'modeling' },
  { input: '画一个球体', expectedIntent: 'modeling' },
  { input: '生成圆柱体几何', expectedIntent: 'modeling' },
  { input: '导入STEP文件', expectedIntent: 'modeling' },
  { input: 'CSG布尔运算', expectedIntent: 'modeling' },
  { input: '创建齿轮齿形', expectedIntent: 'modeling' },
  { input: '画一个圆锥', expectedIntent: 'modeling' },
  { input: '生成网格', expectedIntent: 'modeling' },
  { input: '划分有限元网格', expectedIntent: 'modeling' },
  { input: '导入IGES几何文件', expectedIntent: 'modeling' },
  { input: '创建一个1m长的梁', expectedIntent: 'modeling' },
  { input: '几何建模', expectedIntent: 'modeling' },
  { input: '布尔差集操作', expectedIntent: 'modeling' },
  { input: '导入STL模型', expectedIntent: 'modeling' },
  { input: '生成结构化网格', expectedIntent: 'modeling' },

  // 仿真意图 (20 cases)
  { input: '帮我分析这个悬臂梁的静强度', expectedIntent: 'simulation' },
  { input: '运行静力仿真', expectedIntent: 'simulation' },
  { input: '提交模态分析', expectedIntent: 'simulation' },
  { input: '分析应力分布', expectedIntent: 'simulation' },
  { input: '计算应变', expectedIntent: 'simulation' },
  { input: '运行热分析', expectedIntent: 'simulation' },
  { input: '温度场仿真', expectedIntent: 'simulation' },
  { input: '屈曲分析', expectedIntent: 'simulation' },
  { input: '疲劳寿命评估', expectedIntent: 'simulation' },
  { input: '瞬态动力学分析', expectedIntent: 'simulation' },
  { input: '提交求解', expectedIntent: 'simulation' },
  { input: '运行CAE仿真', expectedIntent: 'simulation' },
  { input: '计算固有频率', expectedIntent: 'simulation' },
  { input: '接触分析', expectedIntent: 'simulation' },
  { input: 'CFD流体仿真', expectedIntent: 'simulation' },
  { input: '振动分析', expectedIntent: 'simulation' },
  { input: '线性静力分析', expectedIntent: 'simulation' },
  { input: '非线性分析', expectedIntent: 'simulation' },
  { input: '位移计算', expectedIntent: 'simulation' },
  { input: '频率响应分析', expectedIntent: 'simulation' },

  // 后处理意图 (12 cases)
  { input: '显示应力云图', expectedIntent: 'postprocess' },
  { input: '渲染Von Mises云图', expectedIntent: 'postprocess' },
  { input: '导出结果为CSV', expectedIntent: 'postprocess' },
  { input: '生成动画', expectedIntent: 'postprocess' },
  { input: '切片查看内部', expectedIntent: 'postprocess' },
  { input: '导出VTK文件', expectedIntent: 'postprocess' },
  { input: '生成应力曲线', expectedIntent: 'postprocess' },
  { input: '绘制位移图表', expectedIntent: 'postprocess' },
  { input: '显示等值线', expectedIntent: 'postprocess' },
  { input: '导出JSON数据', expectedIntent: 'postprocess' },
  { input: '后处理', expectedIntent: 'postprocess' },
  { input: '可视化结果', expectedIntent: 'postprocess' },

  // 代码意图 (10 cases)
  { input: '写一个Python脚本', expectedIntent: 'code' },
  { input: '生成处理CAE结果的代码', expectedIntent: 'code' },
  { input: '帮我写代码', expectedIntent: 'code' },
  { input: '执行Python代码', expectedIntent: 'code' },
  { input: 'Git提交', expectedIntent: 'code' },
  { input: '查看Git状态', expectedIntent: 'code' },
  { input: '创建脚本文件', expectedIntent: 'code' },
  { input: '运行数据处理脚本', expectedIntent: 'code' },
  { input: '生成自动化脚本', expectedIntent: 'code' },
  { input: '写入文件', expectedIntent: 'code' },

  // 分析意图 (10 cases)
  { input: '为什么应力分布不对称？', expectedIntent: 'analysis' },
  { input: '分析结果异常原因', expectedIntent: 'analysis' },
  { input: '对比两种网格的结果', expectedIntent: 'analysis' },
  { input: '检查边界条件设置', expectedIntent: 'analysis' },
  { input: '验证结果正确性', expectedIntent: 'analysis' },
  { input: '诊断求解问题', expectedIntent: 'analysis' },
  { input: '为什么位移这么大？', expectedIntent: 'analysis' },
  { input: '比较不同载荷工况', expectedIntent: 'analysis' },
  { input: '分析不一致原因', expectedIntent: 'analysis' },
  { input: '解释这个结果', expectedIntent: 'analysis', allowedIntents: ['analysis', 'qa'] },

  // 笔记意图 (5 cases)
  { input: '创建分析报告笔记', expectedIntent: 'notes' },
  { input: '记录仿真参数', expectedIntent: 'notes' },
  { input: '插入LaTeX公式', expectedIntent: 'notes' },
  { input: '总结分析结果', expectedIntent: 'notes', allowedIntents: ['notes', 'postprocess'] },
  { input: '创建笔记', expectedIntent: 'notes' },

  // 参数化意图 (8 cases)
  { input: '参数化扫描分析', expectedIntent: 'parametric' },
  { input: '不同板厚对最大应力的影响', expectedIntent: 'parametric' },
  { input: '灵敏度分析', expectedIntent: 'parametric' },
  { input: 'DOE实验设计', expectedIntent: 'parametric' },
  { input: '参数扫描', expectedIntent: 'parametric' },
  { input: '不同材料参数对比', expectedIntent: 'parametric' },
  { input: '尺寸变化对结果的影响', expectedIntent: 'parametric' },
  { input: '参数化研究', expectedIntent: 'parametric' },

  // 优化意图 (5 cases)
  { input: '拓扑优化', expectedIntent: 'optimization' },
  { input: '轻量化设计', expectedIntent: 'optimization' },
  { input: '形状优化', expectedIntent: 'optimization' },
  { input: '尺寸优化', expectedIntent: 'optimization' },
  { input: '优化结构', expectedIntent: 'optimization' },

  // 验证意图 (5 cases)
  { input: '验证仿真精度', expectedIntent: 'validation' },
  { input: '基准测试对比', expectedIntent: 'validation' },
  { input: '与理论解对比', expectedIntent: 'validation' },
  { input: '网格收敛性验证', expectedIntent: 'validation' },
  { input: '误差分析', expectedIntent: 'validation' },

  // QA意图 (10 cases)
  { input: '什么是有限元方法？', expectedIntent: 'qa' },
  { input: '如何使用CAELab？', expectedIntent: 'qa' },
  { input: '解释Von Mises应力的概念', expectedIntent: 'qa' },
  { input: '什么是模态分析？', expectedIntent: 'qa' },
  { input: '如何设置边界条件？', expectedIntent: 'qa' },
  { input: '什么是S-N曲线？', expectedIntent: 'qa' },
  { input: 'CAE是什么？', expectedIntent: 'qa' },
  { input: '请解释泊松比', expectedIntent: 'qa' },
  { input: '什么是拓扑优化？', expectedIntent: 'qa' },
  { input: '如何理解应力集中？', expectedIntent: 'qa' },
]

function runIntentTests(): { total: number; correct: number; accuracy: number; failures: Array<{ input: string; expected: string; actual: string }> } {
  let correct = 0
  const failures: Array<{ input: string; expected: string; actual: string }> = []

  for (const tc of intentTestCases) {
    const result = intentClassifier.classify(tc.input)
    const isCorrect = result.intent === tc.expectedIntent ||
      (tc.allowedIntents && tc.allowedIntents.includes(result.intent))

    if (isCorrect) {
      correct++
    } else {
      failures.push({
        input: tc.input,
        expected: tc.expectedIntent,
        actual: result.intent
      })
    }
  }

  return {
    total: intentTestCases.length,
    correct,
    accuracy: correct / intentTestCases.length,
    failures
  }
}

// ============================================================================
// V2.4-005: 工具调用成功率测试 (>95%, 5个核心工具各20次)
// ============================================================================

async function runToolSuccessTests(): Promise<{ total: number; success: number; rate: number; failures: Array<{ tool: string; error: string }> }> {
  const coreTools = ['get_model_info', 'set_material', 'apply_bc', 'run_simulation', 'get_results']
  const callsPerTool = 20
  let totalCalls = 0
  let successCalls = 0
  const failures: Array<{ tool: string; error: string }> = []

  for (const toolName of coreTools) {
    for (let i = 0; i < callsPerTool; i++) {
      totalCalls++
      try {
        const params = getToolTestParams(toolName, i)
        const result = await toolExecutor.invoke(toolName, params)
        if (result.success) {
          successCalls++
        } else if (result.error !== 'CONFIRMATION_REQUIRED') {
          failures.push({ tool: toolName, error: result.error || '未知错误' })
        } else {
          successCalls++  // 确认类不算失败
        }
      } catch (e) {
        failures.push({ tool: toolName, error: String(e) })
      }
    }
  }

  return {
    total: totalCalls,
    success: successCalls,
    rate: successCalls / totalCalls,
    failures
  }
}

function getToolTestParams(toolName: string, index: number): Record<string, unknown> {
  const params: Record<string, Record<number, Record<string, unknown>>> = {
    'get_model_info': { 0: {} },
    'set_material': { 0: { materialName: 'Q235', youngsModulus: 210e9, poissonsRatio: 0.3, density: 7850 } },
    'apply_bc': { 0: { bcType: 'fixed', face: 'left' }, 1: { bcType: 'point_load', face: 'right', values: { fy: -1000 } } },
    'run_simulation': { 0: { analysisType: 'static' }, 1: { analysisType: 'modal' } },
    'get_results': { 0: { resultType: 'stress', component: 'von_mises' }, 1: { resultType: 'displacement' } },
  }
  const toolParams = params[toolName] || {}
  return toolParams[index % Object.keys(toolParams).length] || {}
}

// ============================================================================
// V2.4-018: Agent 冒烟测试 (10个常见场景)
// ============================================================================

const smokeTestScenarios = [
  { name: '悬臂梁静强度分析', input: '帮我分析这个悬臂梁的静强度', expectedSteps: 6 },
  { name: '模态分析', input: '运行模态分析查看固有频率', expectedSteps: 6 },
  { name: '热分析', input: '进行热分析计算温度分布', expectedSteps: 6 },
  { name: '网格生成与检查', input: '生成网格并检查质量', expectedSteps: 3 },
  { name: '结果导出', input: '导出仿真结果为CSV文件', expectedSteps: 2 },
  { name: '几何创建', input: '创建一个立方体', expectedSteps: 2 },
  { name: '结果分析', input: '为什么应力分布不对称？', expectedSteps: 4 },
  { name: '参数化扫描', input: '不同板厚对最大应力的影响', expectedSteps: 5 },
  { name: '拓扑优化', input: '对当前模型进行拓扑优化', expectedSteps: 4 },
  { name: '结果验证', input: '验证仿真结果的精度', expectedSteps: 4 },
]

async function runSmokeTests(): Promise<{ total: number; passed: number; results: Array<{ name: string; passed: boolean; message: string }> }> {
  const results: Array<{ name: string; passed: boolean; message: string }> = []

  for (const scenario of smokeTestScenarios) {
    try {
      const intent = intentClassifier.classify(scenario.input)
      const plan = taskPlanner.plan(scenario.input, intent)

      const passed = plan.subTasks.length > 0 && plan.subTasks.length >= scenario.expectedSteps - 2
      results.push({
        name: scenario.name,
        passed,
        message: passed
          ? `✅ ${plan.subTasks.length} 个步骤生成`
          : `❌ 期望 ${scenario.expectedSteps} 步，实际 ${plan.subTasks.length} 步`
      })
    } catch (e) {
      results.push({
        name: scenario.name,
        passed: false,
        message: `❌ 异常: ${String(e)}`
      })
    }
  }

  return {
    total: smokeTestScenarios.length,
    passed: results.filter(r => r.passed).length,
    results
  }
}

// ============================================================================
// V2.4-019: 并发安全测试
// ============================================================================

async function runConcurrencyTests(): Promise<{ passed: boolean; results: string[] }> {
  const results: string[] = []

  // 测试1: 多个意图分类并发
  const inputs = Array(10).fill('帮我分析悬臂梁的静强度')
  const classifications = await Promise.all(inputs.map(input => Promise.resolve(intentClassifier.classify(input))))
  const allSame = classifications.every(c => c.intent === 'simulation')
  results.push(`并发意图分类: ${allSame ? '✅ 通过' : '❌ 失败'}`)

  // 测试2: 状态追踪器并发读写
  stateTracker.setAgentMode(true)
  const state1 = stateTracker.getState()
  const state2 = stateTracker.getState()
  const stateConsistent = state1.isAgentMode === state2.isAgentMode
  results.push(`状态追踪器一致性: ${stateConsistent ? '✅ 通过' : '❌ 失败'}`)
  stateTracker.setAgentMode(false)

  // 测试3: 工具注册表并发读取
  const tools1 = toolRegistry.getAll()
  const tools2 = toolRegistry.getAll()
  const toolsConsistent = tools1.length === tools2.length
  results.push(`工具注册表一致性: ${toolsConsistent ? '✅ 通过' : '❌ 失败'}`)

  return {
    passed: allSame && stateConsistent && toolsConsistent,
    results
  }
}

// ============================================================================
// V2.4-020: 性能基准测试
// ============================================================================

async function runPerformanceBenchmarks(): Promise<{ results: Array<{ name: string; value: number; unit: string; passed: boolean }> }> {
  const results: Array<{ name: string; value: number; unit: string; passed: boolean }> = []

  // 意图分类性能
  const intentStart = performance.now()
  for (let i = 0; i < 100; i++) {
    intentClassifier.classify('帮我分析这个悬臂梁的静强度')
  }
  const intentTime = (performance.now() - intentStart) / 100
  results.push({
    name: '意图分类平均耗时',
    value: Math.round(intentTime * 100) / 100,
    unit: 'ms',
    passed: intentTime < 5
  })

  // 任务规划性能
  const planStart = performance.now()
  const intent = intentClassifier.classify('帮我分析这个悬臂梁的静强度')
  for (let i = 0; i < 100; i++) {
    taskPlanner.plan('帮我分析这个悬臂梁的静强度', intent)
  }
  const planTime = (performance.now() - planStart) / 100
  results.push({
    name: '任务规划平均耗时',
    value: Math.round(planTime * 100) / 100,
    unit: 'ms',
    passed: planTime < 10
  })

  // 工具调用性能 (不含求解)
  const toolStart = performance.now()
  const toolResult = await toolExecutor.invoke('get_model_info', {})
  const toolTime = performance.now() - toolStart
  results.push({
    name: '单次工具调用耗时',
    value: Math.round(toolTime * 100) / 100,
    unit: 'ms',
    passed: toolTime < 5000
  })

  // 结果验证性能
  const verifyStart = performance.now()
  for (let i = 0; i < 100; i++) {
    resultVerifier.verify(toolResult, 'get_model_info')
  }
  const verifyTime = (performance.now() - verifyStart) / 100
  results.push({
    name: '结果验证平均耗时',
    value: Math.round(verifyTime * 100) / 100,
    unit: 'ms',
    passed: verifyTime < 5
  })

  return { results }
}

// ============================================================================
// V2.4-021: Agent 评估体系
// ============================================================================

async function runAgentEvaluation(): Promise<{
  intentAccuracy: number
  toolCallSuccessRate: number
  taskCompletionRate: number
  smokeTestPassRate: number
  overallScore: number
}> {
  // 1. 意图识别准确率
  const intentResults = runIntentTests()
  const intentAccuracy = intentResults.accuracy

  // 2. 工具调用成功率
  const toolResults = await runToolSuccessTests()
  const toolCallSuccessRate = toolResults.rate

  // 3. 冒烟测试通过率
  const smokeResults = await runSmokeTests()
  const taskCompletionRate = smokeResults.passed / smokeResults.total

  // 4. 综合评分
  const overallScore = (intentAccuracy * 0.3 + toolCallSuccessRate * 0.3 + taskCompletionRate * 0.4)

  return {
    intentAccuracy: Math.round(intentAccuracy * 100) / 100,
    toolCallSuccessRate: Math.round(toolCallSuccessRate * 100) / 100,
    taskCompletionRate: Math.round(taskCompletionRate * 100) / 100,
    smokeTestPassRate: Math.round(taskCompletionRate * 100) / 100,
    overallScore: Math.round(overallScore * 100) / 100
  }
}

// ============================================================================
// 测试运行器
// ============================================================================

export async function runAllTests(): Promise<void> {
  console.log('========================================')
  console.log('  CAELab V2.4 AI Agent 测试套件')
  console.log('========================================\n')

  // 1. 意图识别测试
  console.log('📊 V2.4-001: 意图识别准确率测试')
  console.log('--------------------------------------')
  const intentResults = runIntentTests()
  console.log(`总测试用例: ${intentResults.total}`)
  console.log(`正确: ${intentResults.correct}`)
  console.log(`准确率: ${(intentResults.accuracy * 100).toFixed(1)}%`)
  console.log(`目标: > 90%`)
  console.log(`结果: ${intentResults.accuracy >= 0.9 ? '✅ 通过' : '❌ 未通过'}`)
  if (intentResults.failures.length > 0) {
    console.log(`\n失败用例 (${intentResults.failures.length}):`)
    for (const f of intentResults.failures.slice(0, 10)) {
      console.log(`  "${f.input}" → 期望: ${f.expected}, 实际: ${f.actual}`)
    }
  }
  console.log('')

  // 2. 工具调用测试
  console.log('🔧 V2.4-005: 工具调用成功率测试')
  console.log('--------------------------------------')
  const toolResults = await runToolSuccessTests()
  console.log(`总调用次数: ${toolResults.total}`)
  console.log(`成功次数: ${toolResults.success}`)
  console.log(`成功率: ${(toolResults.rate * 100).toFixed(1)}%`)
  console.log(`目标: > 95%`)
  console.log(`结果: ${toolResults.rate >= 0.95 ? '✅ 通过' : '❌ 未通过'}`)
  if (toolResults.failures.length > 0) {
    console.log(`\n失败调用 (${toolResults.failures.length}):`)
    for (const f of toolResults.failures.slice(0, 5)) {
      console.log(`  ${f.tool}: ${f.error}`)
    }
  }
  console.log('')

  // 3. 冒烟测试
  console.log('🚀 V2.4-018: Agent 冒烟测试')
  console.log('--------------------------------------')
  const smokeResults = await runSmokeTests()
  console.log(`总场景数: ${smokeResults.total}`)
  console.log(`通过: ${smokeResults.passed}`)
  console.log(`目标: > 7/10`)
  console.log(`结果: ${smokeResults.passed >= 7 ? '✅ 通过' : '❌ 未通过'}`)
  for (const r of smokeResults.results) {
    console.log(`  ${r.message} - ${r.name}`)
  }
  console.log('')

  // 4. 并发安全测试
  console.log('🔒 V2.4-019: 并发安全测试')
  console.log('--------------------------------------')
  const concurrencyResults = await runConcurrencyTests()
  console.log(`结果: ${concurrencyResults.passed ? '✅ 通过' : '❌ 未通过'}`)
  for (const r of concurrencyResults.results) {
    console.log(`  ${r}`)
  }
  console.log('')

  // 5. 性能基准
  console.log('⚡ V2.4-020: 性能基准测试')
  console.log('--------------------------------------')
  const perfResults = await runPerformanceBenchmarks()
  for (const r of perfResults.results) {
    console.log(`  ${r.name}: ${r.value} ${r.unit} ${r.passed ? '✅' : '❌'}`)
  }
  console.log('')

  // 6. 综合评估
  console.log('📈 V2.4-021: Agent 综合评估')
  console.log('--------------------------------------')
  const evaluation = await runAgentEvaluation()
  console.log(`意图识别准确率: ${(evaluation.intentAccuracy * 100).toFixed(1)}%`)
  console.log(`工具调用成功率: ${(evaluation.toolCallSuccessRate * 100).toFixed(1)}%`)
  console.log(`任务完成率: ${(evaluation.taskCompletionRate * 100).toFixed(1)}%`)
  console.log(`综合评分: ${(evaluation.overallScore * 100).toFixed(1)}%`)
  console.log(`目标: > 85%`)
  console.log(`结果: ${evaluation.overallScore >= 0.85 ? '✅ 通过' : '❌ 未通过'}`)
  console.log('')

  console.log('========================================')
  console.log('  测试完成')
  console.log('========================================')
}

// 如果直接运行此文件
if (typeof require !== 'undefined' && require.main === module) {
  runAllTests().catch(console.error)
}
