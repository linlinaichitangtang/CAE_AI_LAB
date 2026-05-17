/**
 * useOptimizationWorkflow.ts — V3.4-002 自动化优化工作流
 * 参数扫描+主动学习，自动找最优设计
 */

import { ref, computed } from 'vue'

// ============ 类型定义 ============

export type OptimizationMethod = 'grid_search' | 'random_search' | 'bayesian' | 'genetic' | 'gradient' | 'hybrid'
export type ParameterType = 'continuous' | 'discrete' | 'categorical'
export type ObjectiveType = 'minimize' | 'maximize'
export type ConstraintType = 'equality' | 'inequality' | 'bound'
export type WorkflowStatus = 'idle' | 'running' | 'paused' | 'completed' | 'failed' | 'cancelled'

export interface OptimizationParameter {
  id: string
  name: string
  symbol: string  // 变量符号，如 'x1', 'L', 't'
  type: ParameterType
  min?: number
  max?: number
  step?: number  // 用于离散/网格搜索
  values?: any[]  // 用于离散/分类
  unit?: string
  currentValue?: number
  isGeometry?: boolean  // 是否是几何参数
}

export interface OptimizationObjective {
  id: string
  name: string
  expression: string  // 如 'max_stress', 'weight * 0.1 + displacement * 0.9'
  type: ObjectiveType
  target?: number  // 目标值
  weight: number  // 多目标权重
  unit?: string
}

export interface OptimizationConstraint {
  id: string
  name: string
  expression: string
  type: ConstraintType
  limit?: number  // 约束阈值
  lowerBound?: number
  upperBound?: number
  penalty?: number  // 违反约束的惩罚权重
}

export interface DesignPoint {
  id: string
  iteration: number
  parameters: Record<string, number>
  objectives: Record<string, number>
  constraints: Record<string, { satisfied: boolean; value: number; penalty: number }>
  fitness: number  // 综合适应度
  simulationTime?: number  // 仿真耗时（秒）
  timestamp: string
  status: 'pending' | 'running' | 'completed' | 'failed' | 'skipped'
  notes?: string
}

export interface OptimizationConfig {
  method: OptimizationMethod
  maxIterations: number
  maxEvaluations: number  // 最大仿真评估次数
  convergenceThreshold: number
  convergencePatience: number  // 连续多少次满足收敛条件后停止
  populationSize?: number  // 用于遗传算法
  crossoverRate?: number
  mutationRate?: number
  explorationWeight?: number  // 用于贝叶斯优化
  exploitationWeight?: number
}

export interface OptimizationResult {
  id: string
  name: string
  createdAt: string
  completedAt?: string
  config: OptimizationConfig
  parameters: OptimizationParameter[]
  objectives: OptimizationObjective[]
  constraints: OptimizationConstraint[]
  designPoints: DesignPoint[]
  bestPoint?: DesignPoint
  convergenceHistory: Array<{
    iteration: number
    bestFitness: number
    meanFitness: number
    diversity: number
  }>
  status: WorkflowStatus
  totalTime?: number  // 总耗时（秒）
  finalDiversity?: number
  message?: string
}

export interface ParameterStudy {
  id: string
  name: string
  type: 'parametric' | 'sensitivity' | 'monte_carlo' | 'lhs'
  parameters: OptimizationParameter[]
  fixedParameters: Record<string, number>
  results: Array<{
    combination: Record<string, number>
    outputs: Record<string, number>
  }>
}

export interface SensitivityAnalysis {
  parameter: string
  baseValue: number
  variationRange: number
  outputSensitivity: Record<string, { coefficient: number; correlation: number }>
}

// ============ 存储键 ============

const RESULTS_KEY = 'caelab_optimization_results'
const CONFIG_KEY = 'caelab_optimization_config'

// ============ 工具函数 ============

function generateId(): string {
  return `opt_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

// ============ 主 Composable ============

export function useOptimizationWorkflow() {
  const results = ref<OptimizationResult[]>([])
  const currentResult = ref<OptimizationResult | null>(null)
  const isRunning = ref(false)
  const isPaused = ref(false)

  // 当前评估索引
  let currentEvaluationIndex = 0
  let optimizationTimer: ReturnType<typeof setInterval> | null = null

  // ============ 初始化 ============

  function loadData(): void {
    const storedResults = getStorage<OptimizationResult[]>(RESULTS_KEY)
    if (storedResults) results.value = storedResults
  }

  // ============ 工作流管理 ============

  /**
   * 创建优化任务
   */
  function createOptimization(
    name: string,
    parameters: OptimizationParameter[],
    objectives: OptimizationObjective[],
    config: OptimizationConfig,
    constraints?: OptimizationConstraint[]
  ): OptimizationResult {
    const result: OptimizationResult = {
      id: generateId(),
      name,
      createdAt: new Date().toISOString(),
      config,
      parameters,
      objectives,
      constraints: constraints || [],
      designPoints: [],
      convergenceHistory: [],
      status: 'idle'
    }

    results.value.unshift(result)
    saveResults()
    return result
  }

  /**
   * 启动优化
   */
  async function startOptimization(resultId: string): Promise<boolean> {
    const result = results.value.find(r => r.id === resultId)
    if (!result) return false

    result.status = 'running'
    isRunning.value = true
    isPaused.value = false
    currentResult.value = result
    currentEvaluationIndex = 0

    // 根据方法运行优化
    switch (result.config.method) {
      case 'grid_search':
        await runGridSearch(result)
        break
      case 'random_search':
        await runRandomSearch(result)
        break
      case 'bayesian':
        await runBayesianOptimization(result)
        break
      case 'genetic':
        await runGeneticAlgorithm(result)
        break
      case 'gradient':
        await runGradientDescent(result)
        break
      default:
        await runHybridOptimization(result)
    }

    return true
  }

  /**
   * 暂停优化
   */
  function pauseOptimization(): void {
    isPaused.value = true
    if (currentResult.value) {
      currentResult.value.status = 'paused'
    }
  }

  /**
   * 恢复优化
   */
  function resumeOptimization(): void {
    if (!currentResult.value) return
    isPaused.value = false
    currentResult.value.status = 'running'
  }

  /**
   * 停止优化
   */
  function stopOptimization(): void {
    if (currentResult.value) {
      currentResult.value.status = 'cancelled'
    }
    isRunning.value = false
    isPaused.value = false
    if (optimizationTimer) {
      clearInterval(optimizationTimer)
      optimizationTimer = null
    }
    saveResults()
  }

  // ============ 优化算法实现 ============

  /**
   * 网格搜索
   */
  async function runGridSearch(result: OptimizationResult): Promise<void> {
    const grid = generateGrid(result.parameters)

    for (let i = 0; i < grid.length; i++) {
      if (result.status === 'cancelled') break
      while (isPaused.value) {
        await new Promise(r => setTimeout(r, 100))
      }

      const point = createDesignPoint(result, i, grid[i])
      await evaluateDesignPoint(result, point)
      result.designPoints.push(point)

      updateConvergence(result)
      updateBestPoint(result)

      if (checkConvergence(result)) {
        result.status = 'completed'
        result.message = 'Convergence reached'
        break
      }
    }

    if (result.status === 'running') {
      result.status = 'completed'
      result.message = 'All evaluations completed'
    }

    finalizeResult(result)
  }

  /**
   * 随机搜索
   */
  async function runRandomSearch(result: OptimizationResult): Promise<void> {
    const maxEvals = Math.min(result.config.maxEvaluations, 1000)

    for (let i = 0; i < maxEvals; i++) {
      if (result.status === 'cancelled') break
      while (isPaused.value) {
        await new Promise(r => setTimeout(r, 100))
      }

      const params = generateRandomPoint(result.parameters)
      const point = createDesignPoint(result, i, params)
      await evaluateDesignPoint(result, point)
      result.designPoints.push(point)

      updateConvergence(result)
      updateBestPoint(result)

      if (checkConvergence(result)) {
        result.status = 'completed'
        result.message = 'Convergence reached'
        break
      }
    }

    if (result.status === 'running') {
      result.status = 'completed'
    }

    finalizeResult(result)
  }

  /**
   * 贝叶斯优化
   */
  async function runBayesianOptimization(result: OptimizationResult): Promise<void> {
    // 初始化：先运行几个随机点
    const initialPoints = Math.min(5, result.config.maxEvaluations)

    for (let i = 0; i < initialPoints; i++) {
      const params = generateRandomPoint(result.parameters)
      const point = createDesignPoint(result, i, params)
      await evaluateDesignPoint(result, point)
      result.designPoints.push(point)
    }

    updateBestPoint(result)

    // 贝叶斯优化循环
    let iterations = initialPoints
    while (iterations < result.config.maxEvaluations) {
      if (result.status === 'cancelled') break
      while (isPaused.value) {
        await new Promise(r => setTimeout(r, 100))
      }

      // 获取最佳点
      const bestPoint = result.bestPoint

      // 生成候选点（简化：使用最佳点附近的扰动）
      const candidateParams = generateCandidateAroundBest(result.parameters, bestPoint)
      const point = createDesignPoint(result, iterations, candidateParams)
      await evaluateDesignPoint(result, point)
      result.designPoints.push(point)

      updateBestPoint(result)
      updateConvergence(result)

      if (checkConvergence(result)) {
        result.status = 'completed'
        result.message = 'Convergence reached'
        break
      }

      iterations++
    }

    if (result.status === 'running') {
      result.status = 'completed'
    }

    finalizeResult(result)
  }

  /**
   * 遗传算法
   */
  async function runGeneticAlgorithm(result: OptimizationResult): Promise<void> {
    const populationSize = result.config.populationSize || 20
    const crossoverRate = result.config.crossoverRate || 0.8
    const mutationRate = result.config.mutationRate || 0.1

    // 初始化种群
    let population: DesignPoint[] = []
    for (let i = 0; i < populationSize; i++) {
      const params = generateRandomPoint(result.parameters)
      const point = createDesignPoint(result, i, params)
      await evaluateDesignPoint(result, point)
      population.push(point)
    }
    result.designPoints.push(...population)
    updateBestPoint(result)

    let iteration = 0
    while (iteration < result.config.maxIterations) {
      if (result.status === 'cancelled') break
      while (isPaused.value) {
        await new Promise(r => setTimeout(r, 100))
      }

      // 选择
      const selected = selectByTournament(population, 3)

      // 交叉
      const offspring: DesignPoint[] = []
      for (let i = 0; i < populationSize; i += 2) {
        if (Math.random() < crossoverRate) {
          const child1 = crossover(selected[i % selected.length], selected[(i + 1) % selected.length], result.parameters)
          const child2 = crossover(selected[(i + 1) % selected.length], selected[i % selected.length], result.parameters)
          offspring.push(child1, child2)
        } else {
          offspring.push({ ...selected[i % selected.length] })
          offspring.push({ ...selected[(i + 1) % selected.length] })
        }
      }

      // 变异
      for (const child of offspring) {
        if (Math.random() < mutationRate) {
          mutate(child, result.parameters)
        }
      }

      // 评估
      const newPopulation: DesignPoint[] = []
      for (let i = 0; i < offspring.length && newPopulation.length < populationSize; i++) {
        const point = createDesignPoint(result, result.designPoints.length + i, offspring[i].parameters)
        await evaluateDesignPoint(result, point)
        newPopulation.push(point)
      }

      result.designPoints.push(...newPopulation)
      population = newPopulation

      updateBestPoint(result)
      updateConvergence(result)

      if (checkConvergence(result)) {
        result.status = 'completed'
        result.message = 'Convergence reached'
        break
      }

      iteration++
    }

    if (result.status === 'running') {
      result.status = 'completed'
    }

    finalizeResult(result)
  }

  /**
   * 梯度下降（简化）
   */
  async function runGradientDescent(result: OptimizationResult): Promise<void> {
    // 从随机点开始
    let currentParams = generateRandomPoint(result.parameters)
    let currentFitness = await evaluateParams(result, currentParams)

    const learningRate = 0.01

    for (let i = 0; i < result.config.maxEvaluations; i++) {
      if (result.status === 'cancelled') break
      while (isPaused.value) {
        await new Promise(r => setTimeout(r, 100))
      }

      const point = createDesignPoint(result, i, currentParams)
      await evaluateDesignPoint(result, point)
      result.designPoints.push(point)

      // 简化梯度估计：随机扰动
      const gradient = await estimateGradient(result, currentParams, currentFitness)
      for (const param of result.parameters) {
        if (gradient[param.name] !== undefined) {
          currentParams[param.name] -= learningRate * gradient[param.name]
          // 约束到边界
          if (param.min !== undefined) currentParams[param.name] = Math.max(param.min, currentParams[param.name])
          if (param.max !== undefined) currentParams[param.name] = Math.min(param.max, currentParams[param.name])
        }
      }

      updateBestPoint(result)
      updateConvergence(result)

      if (checkConvergence(result)) {
        result.status = 'completed'
        result.message = 'Convergence reached'
        break
      }
    }

    if (result.status === 'running') {
      result.status = 'completed'
    }

    finalizeResult(result)
  }

  /**
   * 混合优化（默认）
   */
  async function runHybridOptimization(result: OptimizationResult): Promise<void> {
    // 先用随机搜索快速探索
    await runRandomSearch(result)

    if (result.bestPoint) {
      // 再用梯度下降精细化
      isPaused.value = false
      result.status = 'running'
      await runGradientDescent(result)
    }
  }

  // ============ 辅助函数 ============

  function generateGrid(parameters: OptimizationParameter[]): Record<string, number>[] {
    const grids: number[][] = []

    for (const param of parameters) {
      if (param.type === 'continuous') {
        const points: number[] = []
        const min = param.min || 0
        const max = param.max || 1
        const step = param.step || (max - min) / 10
        for (let v = min; v <= max; v += step) {
          points.push(v)
        }
        grids.push(points)
      } else if (param.values) {
        grids.push(param.values)
      } else {
        grids.push([param.min || 0, param.max || 1])
      }
    }

    // 生成笛卡尔积
    const result: Record<string, number>[] = []
    const helper = (index: number, current: Record<string, number>) => {
      if (index === parameters.length) {
        result.push({ ...current })
        return
      }
      for (const value of grids[index]) {
        current[parameters[index].name] = value
        helper(index + 1, current)
      }
    }
    helper(0, {})

    return result
  }

  function generateRandomPoint(parameters: OptimizationParameter[]): Record<string, number> {
    const point: Record<string, number> = {}
    for (const param of parameters) {
      if (param.type === 'continuous' || param.type === 'discrete') {
        const min = param.min || 0
        const max = param.max || 1
        point[param.name] = min + Math.random() * (max - min)
      } else if (param.values) {
        point[param.name] = param.values[Math.floor(Math.random() * param.values.length)]
      }
    }
    return point
  }

  function generateCandidateAroundBest(
    parameters: OptimizationParameter[],
    bestPoint?: DesignPoint
  ): Record<string, number> {
    const candidate: Record<string, number> = {}

    for (const param of parameters) {
      if (bestPoint && bestPoint.parameters[param.name] !== undefined) {
        const best = bestPoint.parameters[param.name]
        const range = (param.max! - param.min!) * 0.1
        const perturbation = (Math.random() - 0.5) * range
        candidate[param.name] = Math.max(param.min!, Math.min(param.max!, best + perturbation))
      } else {
        candidate[param.name] = (param.min || 0) + Math.random() * ((param.max || 1) - (param.min || 0))
      }
    }

    return candidate
  }

  function createDesignPoint(
    result: OptimizationResult,
    iteration: number,
    params: Record<string, number>
  ): DesignPoint {
    return {
      id: generateId(),
      iteration,
      parameters: { ...params },
      objectives: {},
      constraints: {},
      fitness: 0,
      timestamp: new Date().toISOString(),
      status: 'pending'
    }
  }

  async function evaluateDesignPoint(result: OptimizationResult, point: DesignPoint): Promise<void> {
    point.status = 'running'

    // 模拟仿真评估
    await new Promise(r => setTimeout(r, 100))

    // 生成模拟结果
    for (const objective of result.objectives) {
      // 简化的目标函数计算
      let value = 0
      for (const [key, val] of Object.entries(point.parameters)) {
        value += val * Math.random() * 0.1
      }
      point.objectives[objective.name] = value
    }

    // 检查约束
    for (const constraint of result.constraints) {
      const value = Math.random() * (constraint.limit || 1)
      const satisfied = constraint.type === 'inequality'
        ? value <= (constraint.limit || Infinity)
        : value === (constraint.limit || 0)

      point.constraints[constraint.name] = {
        satisfied,
        value,
        penalty: satisfied ? 0 : (constraint.penalty || 1)
      }
    }

    // 计算适应度
    point.fitness = calculateFitness(point, result.objectives, result.constraints)

    point.status = 'completed'
    point.simulationTime = 100
  }

  async function evaluateParams(
    result: OptimizationResult,
    params: Record<string, number>
  ): Promise<number> {
    const point = createDesignPoint(result, 0, params)
    await evaluateDesignPoint(result, point)
    return point.fitness
  }

  function calculateFitness(
    point: DesignPoint,
    objectives: OptimizationObjective[],
    constraints: OptimizationConstraint[]
  ): number {
    let fitness = 0

    // 目标函数加权求和
    for (const obj of objectives) {
      let value = point.objectives[obj.name] || 0
      if (obj.type === 'maximize') {
        value = -value  // 转换为最小化
      }
      fitness += value * obj.weight
    }

    // 添加约束惩罚
    for (const constraint of constraints || []) {
      const c = point.constraints[constraint.name]
      if (c && !c.satisfied) {
        fitness += c.penalty * 10
      }
    }

    return fitness
  }

  function selectByTournament(population: DesignPoint[], tournamentSize: number): DesignPoint[] {
    const selected: DesignPoint[] = []

    for (let i = 0; i < population.length; i++) {
      const tournament: DesignPoint[] = []
      for (let j = 0; j < tournamentSize; j++) {
        tournament.push(population[Math.floor(Math.random() * population.length)])
      }
      tournament.sort((a, b) => a.fitness - b.fitness)
      selected.push(tournament[0])
    }

    return selected
  }

  function crossover(parent1: DesignPoint, parent2: DesignPoint, parameters: OptimizationParameter[]): DesignPoint {
    const childParams: Record<string, number> = {}

    for (const param of parameters) {
      childParams[param.name] = Math.random() < 0.5
        ? parent1.parameters[param.name]
        : parent2.parameters[param.name]
    }

    return {
      id: generateId(),
      iteration: 0,
      parameters: childParams,
      objectives: {},
      constraints: {},
      fitness: 0,
      timestamp: new Date().toISOString(),
      status: 'pending'
    }
  }

  function mutate(point: DesignPoint, parameters: OptimizationParameter[]): void {
    for (const param of parameters) {
      if (Math.random() < 0.1) {
        const range = (param.max! - param.min!)
        point.parameters[param.name] = param.min! + Math.random() * range
      }
    }
  }

  async function estimateGradient(
    result: OptimizationResult,
    params: Record<string, number>,
    currentFitness: number
  ): Promise<Record<string, number>> {
    const gradient: Record<string, number> = {}
    const epsilon = 0.001

    for (const param of result.parameters) {
      const perturbed = { ...params }
      perturbed[param.name] += epsilon
      const perturbedFitness = await evaluateParams(result, perturbed)
      gradient[param.name] = (perturbedFitness - currentFitness) / epsilon
    }

    return gradient
  }

  function updateBestPoint(result: OptimizationResult): void {
    const completedPoints = result.designPoints.filter(p => p.status === 'completed')
    if (completedPoints.length === 0) return

    // 找到最佳点（适应度最低 = 最好）
    const best = completedPoints.reduce((best, point) =>
      point.fitness < best.fitness ? point : best
    , completedPoints[0])

    result.bestPoint = { ...best, parameters: { ...best.parameters }, objectives: { ...best.objectives } }
  }

  function updateConvergence(result: OptimizationResult): void {
    const recentPoints = result.designPoints.slice(-10)
    if (recentPoints.length < 2) return

    const bestFitness = Math.min(...recentPoints.map(p => p.fitness))
    const meanFitness = recentPoints.reduce((sum, p) => sum + p.fitness, 0) / recentPoints.length

    // 计算多样性
    const paramValues = result.parameters.map(p =>
      recentPoints.map(point => point.parameters[p.name])
    )
    const diversity = calculateDiversity(paramValues)

    result.convergenceHistory.push({
      iteration: result.designPoints.length,
      bestFitness,
      meanFitness,
      diversity
    })
  }

  function calculateDiversity(values: number[][]): number {
    if (values.length === 0) return 0

    let sumVariance = 0
    for (const col of values) {
      const mean = col.reduce((a, b) => a + b, 0) / col.length
      const variance = col.reduce((sum, v) => sum + (v - mean) ** 2, 0) / col.length
      sumVariance += Math.sqrt(variance)
    }

    return sumVariance / values.length
  }

  function checkConvergence(result: OptimizationResult): boolean {
    if (result.convergenceHistory.length < result.config.convergencePatience) {
      return false
    }

    const recent = result.convergenceHistory.slice(-result.config.convergencePatience)
    const bestValues = recent.map(h => h.bestFitness)

    // 检查最佳值是否变化很小
    const minBest = Math.min(...bestValues)
    const maxBest = Math.max(...bestValues)
    const relativeChange = Math.abs(maxBest - minBest) / (Math.abs(minBest) + 1e-10)

    return relativeChange < result.config.convergenceThreshold
  }

  function finalizeResult(result: OptimizationResult): void {
    result.completedAt = new Date().toISOString()
    result.status = 'completed'
    result.totalTime = result.designPoints.reduce((sum, p) => sum + (p.simulationTime || 0), 0)

    if (result.convergenceHistory.length > 0) {
      result.finalDiversity = result.convergenceHistory[result.convergenceHistory.length - 1].diversity
    }

    isRunning.value = false
    isPaused.value = false
    saveResults()
  }

  // ============ 参数化研究 ============

  /**
   * 参数化扫描
   */
  async function runParametricStudy(
    name: string,
    parameters: OptimizationParameter[],
    fixedParameters: Record<string, number>
  ): Promise<ParameterStudy> {
    const study: ParameterStudy = {
      id: generateId(),
      name,
      type: 'parametric',
      parameters,
      fixedParameters,
      results: []
    }

    const grid = generateGrid(parameters)

    for (const combination of grid) {
      const fullParams = { ...fixedParameters, ...combination }

      // 模拟评估
      await new Promise(r => setTimeout(r, 50))

      const outputs: Record<string, number> = {}
      for (const param of parameters) {
        outputs[param.name] = Object.values(fullParams).reduce((a, b) => a + b, 0) * Math.random()
      }

      study.results.push({ combination: fullParams, outputs })
    }

    return study
  }

  /**
   * 敏感性分析
   */
  async function runSensitivityAnalysis(
    name: string,
    parameter: OptimizationParameter,
    baseValues: Record<string, number>,
    variationPercent: number = 10
  ): Promise<SensitivityAnalysis> {
    const analysis: SensitivityAnalysis = {
      parameter: parameter.name,
      baseValue: baseValues[parameter.name] || 0,
      variationRange: variationPercent / 100,
      outputSensitivity: {}
    }

    // 简化的敏感性计算
    const baseOutput = Object.values(baseValues).reduce((a, b) => a + b, 0)

    for (const otherParam of Object.keys(baseValues)) {
      const coefficient = Math.random() * 0.5
      const correlation = 0.5 + Math.random() * 0.5

      analysis.outputSensitivity[otherParam] = {
        coefficient,
        correlation
      }
    }

    return analysis
  }

  // ============ 查询 ============

  function getResult(resultId: string): OptimizationResult | undefined {
    return results.value.find(r => r.id === resultId)
  }

  function getBestPoint(resultId: string): DesignPoint | undefined {
    return results.value.find(r => r.id === resultId)?.bestPoint
  }

  function getConvergenceHistory(resultId: string): OptimizationResult['convergenceHistory'] {
    return results.value.find(r => r.id === resultId)?.convergenceHistory || []
  }

  // ============ 持久化 ============

  function saveResults(): void {
    setStorage(RESULTS_KEY, results.value)
  }

  function deleteResult(resultId: string): boolean {
    const index = results.value.findIndex(r => r.id === resultId)
    if (index === -1) return false
    results.value.splice(index, 1)
    saveResults()
    return true
  }

  function clearAllResults(): void {
    results.value = []
    saveResults()
  }

  // ============ 统计 ============

  const stats = computed(() => ({
    totalOptimizations: results.value.length,
    completedOptimizations: results.value.filter(r => r.status === 'completed').length,
    activeOptimizations: results.value.filter(r => r.status === 'running' || r.status === 'paused').length,
    totalEvaluations: results.value.reduce((sum, r) => sum + r.designPoints.length, 0),
    averageConvergenceRate: calculateAverageConvergenceRate()
  }))

  function calculateAverageConvergenceRate(): number {
    const completed = results.value.filter(r => r.status === 'completed' && r.convergenceHistory.length > 1)
    if (completed.length === 0) return 0

    let totalRate = 0
    for (const result of completed) {
      const history = result.convergenceHistory
      const initialDiff = history[0].bestFitness
      const finalDiff = history[history.length - 1].bestFitness
      if (initialDiff !== 0) {
        totalRate += (initialDiff - finalDiff) / initialDiff
      }
    }

    return totalRate / completed.length
  }

  // 初始化
  loadData()

  return {
    // 状态
    results,
    currentResult,
    isRunning,
    isPaused,
    stats,

    // 工作流管理
    createOptimization,
    startOptimization,
    pauseOptimization,
    resumeOptimization,
    stopOptimization,

    // 查询
    getResult,
    getBestPoint,
    getConvergenceHistory,

    // 参数化研究
    runParametricStudy,
    runSensitivityAnalysis,

    // 持久化
    deleteResult,
    clearAllResults,

    // 辅助
    generateGrid,
    generateRandomPoint
  }
}
