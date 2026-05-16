/**
 * useParametricBatch.ts — V4.2-005 参数化批处理引擎
 * 可视化设置扫描参数，后台自动批量运行仿真
 */
import { ref, computed, reactive } from 'vue'

/** 参数化扫描参数定义 */
export interface ScanParameter {
  id: string
  name: string
  type: 'material' | 'geometry' | 'load' | 'mesh' | 'solver'
  variable: string       // 参数变量名（对应 INP 文件中的变量名或字段路径）
  unit?: string
  description?: string
}

/** 参数扫描范围 */
export interface ScanRange {
  parameterId: string
  mode: 'linear' | 'log' | 'list'    // 线性/对数/列表
  start: number
  end: number
  step?: number        // 线性模式下步长
  count?: number       // 或指定点数
  values?: number[]    // 列表模式下的具体值
}

/** DOE 实验设计类型 */
export type DOEType = 'full_factorial' | 'fractional_factorial' | 'box_behnken' | 'central_composite' | 'latin_hypercube'

/** DOE 配置 */
export interface DOEConfig {
  type: DOEType
  levels: number       // 因子水平数
  centerPoints: number // 中心点数量
}

/** 批处理任务 */
export interface BatchJob {
  id: string
  name: string
  parameterSets: Array<Record<string, number>>  // 所有参数组合
  totalCount: number
  completedCount: number
  failedCount: number
  status: 'pending' | 'running' | 'paused' | 'completed' | 'failed'
  results: BatchJobResult[]
  createdAt: string
  startedAt?: string
  completedAt?: string
}

/** 批处理任务结果 */
export interface BatchJobResult {
  jobId: string
  parameterSet: Record<string, number>
  caseIndex: number
  status: 'pending' | 'running' | 'success' | 'failed' | 'skipped'
  startTime?: string
  endTime?: string
  duration?: number
  error?: string
  outputMetrics?: {
    maxStress?: number
    maxDisplacement?: number
    safetyFactor?: number
    firstFrequency?: number
    [key: string]: number | undefined
  }
}

/** DOE 生成的实验矩阵行 */
export interface DOERow {
  run: number
  values: Record<string, number>
  isCenterPoint: boolean
  isFactorial: boolean
}

/** 扫描结果汇总 */
export interface ScanSummary {
  totalRuns: number
  successfulRuns: number
  failedRuns: number
  metrics: {
    maxStress?: { min: number; max: number; mean: number; std: number }
    maxDisplacement?: { min: number; max: number; mean: number; std: number }
    safetyFactor?: { min: number; max: number; mean: number; std: number }
    firstFrequency?: { min: number; max: number; mean: number; std: number }
  }
  sensitivityRanking: Array<{ parameter: string; sensitivity: number }>
  bestCase?: { params: Record<string, number>; metric: string; value: number }
}

export function useParametricBatch() {
  const batchJobs = ref<BatchJob[]>([])
  const activeJobId = ref<string | null>(null)
  const isGeneratingDOE = ref(false)
  const currentProgress = ref(0)

  const activeJob = computed(() =>
    batchJobs.value.find(j => j.id === activeJobId.value) || null
  )

  /** 创建新的批处理任务 */
  function createBatchJob(
    name: string,
    parameterSets: Array<Record<string, number>>
  ): BatchJob {
    const job: BatchJob = {
      id: `batch-${Date.now()}`,
      name,
      parameterSets,
      totalCount: parameterSets.length,
      completedCount: 0,
      failedCount: 0,
      status: 'pending',
      results: parameterSets.map((params, i) => ({
        jobId: `batch-${Date.now()}`,
        parameterSet: params,
        caseIndex: i,
        status: 'pending' as const
      })),
      createdAt: new Date().toISOString()
    }
    batchJobs.value.push(job)
    return job
  }

  /** 生成线性扫描参数组合 */
  function generateLinearScan(ranges: ScanRange[]): Array<Record<string, number>> {
    const results: Array<Record<string, number>> = []

    function generateCombination(index: number, current: Record<string, number>) {
      if (index === ranges.length) {
        results.push({ ...current })
        return
      }

      const range = ranges[index]
      const paramId = range.parameterId
      const values: number[] = []

      if (range.mode === 'list' && range.values) {
        values.push(...range.values)
      } else {
        const count = range.count || Math.ceil((range.end - range.start) / (range.step || 1)) + 1
        for (let i = 0; i < count; i++) {
          const t = count > 1 ? i / (count - 1) : 0
          let value: number
          if (range.mode === 'log') {
            const logStart = Math.log10(Math.max(range.start, 1e-10))
            const logEnd = Math.log10(Math.max(range.end, 1e-10))
            value = Math.pow(10, logStart + t * (logEnd - logStart))
          } else {
            value = range.start + t * (range.end - range.start)
          }
          values.push(value)
        }
      }

      for (const value of values) {
        current[paramId] = value
        generateCombination(index + 1, current)
      }
    }

    generateCombination(0, {})
    return results
  }

  /** 生成 DOE 实验矩阵 */
  function generateDOE(
    ranges: ScanRange[],
    config: DOEConfig
  ): Array<Record<string, number>> {
    isGeneratingDOE.value = true

    try {
      const params = ranges.map(r => r.parameterId)
      const levels = config.levels
      const centerPoints = config.centerPoints

      switch (config.type) {
        case 'full_factorial':
          // 全因子设计：每个参数的每个水平与其他参数的每个水平组合
          return generateFullFactorial(params, levels, ranges)

        case 'fractional_factorial':
          // 部分因子设计：2^(k-1) 分辨率
          return generateFractionalFactorial(params, levels, ranges)

        case 'box_behnken':
          // Box-Behnken 设计：适用于 3-7 个因子
          return generateBoxBehnken(params, levels, ranges)

        case 'central_composite':
          // Central Composite Design (CCD)：包含轴向点和中心点
          return generateCentralComposite(params, levels, ranges)

        case 'latin_hypercube':
          // 拉丁超立方设计：每个参数水平在每行只出现一次
          return generateLatinHypercube(params, levels, ranges)

        default:
          return generateFullFactorial(params, levels, ranges)
      }
    } finally {
      isGeneratingDOE.value = false
    }
  }

  /** 全因子设计 */
  function generateFullFactorial(
    params: string[],
    levels: number,
    ranges: ScanRange[]
  ): Array<Record<string, number>> {
    const results: Array<Record<string, number>> = []
    const count = Math.pow(levels, params.length)

    for (let i = 0; i < count; i++) {
      const row: Record<string, number> = {}
      let remainder = i
      for (let j = 0; j < params.length; j++) {
        const levelIndex = remainder % levels
        remainder = Math.floor(remainder / levels)
        const range = ranges[j]
        row[params[j]] = range.start + (levelIndex / (levels - 1)) * (range.end - range.start)
      }
      results.push(row)
    }

    // 添加中心点
    const centerPoint: Record<string, number> = {}
    for (const range of ranges) {
      centerPoint[range.parameterId] = (range.start + range.end) / 2
    }
    results.push(centerPoint)

    return results
  }

  /** 部分因子设计 (2^(k-1)) */
  function generateFractionalFactorial(
    params: string[],
    levels: number,
    ranges: ScanRange[]
  ): Array<Record<string, number>> {
    // 简化为 2 水平的 1/2 分辨率设计
    const k = params.length
    const halfFactor = Math.pow(2, k - 1)

    const results: Array<Record<string, number>> = []
    const generator = params.slice(0, k - 1) // 最后一个因子由前 k-1 个决定

    for (let i = 0; i < halfFactor; i++) {
      const row: Record<string, number> = {}
      let remainder = i
      for (let j = 0; j < params.length - 1; j++) {
        const levelIndex = remainder % 2
        remainder = Math.floor(remainder / 2)
        const range = ranges[j]
        row[params[j]] = levelIndex === 0 ? range.start : range.end
      }
      // 最后一个因子
      const lastRange = ranges[params.length - 1]
      row[params[params.length - 1]] = i % 2 === 0 ? lastRange.start : lastRange.end
      results.push(row)
    }

    return results
  }

  /** Box-Behnken 设计 */
  function generateBoxBehnken(
    params: string[],
    levels: number,
    ranges: ScanRange[]
  ): Array<Record<string, number>> {
    const k = params.length
    const results: Array<Record<string, number>> = []

    // 每个因子与其他因子的中心点组合
    for (let i = 0; i < k; i++) {
      for (let j = i + 1; j < k; j++) {
        // (min, min)
        const row1: Record<string, number> = {}
        for (let m = 0; m < k; m++) {
          row1[params[m]] = ranges[m].start
        }
        row1[params[i]] = ranges[i].start
        row1[params[j]] = ranges[j].start
        results.push(row1)

        // (max, max)
        const row2: Record<string, number> = {}
        for (let m = 0; m < k; m++) {
          row2[params[m]] = ranges[m].end
        }
        row2[params[i]] = ranges[i].end
        row2[params[j]] = ranges[j].end
        results.push(row2)

        // (min, max)
        const row3: Record<string, number> = {}
        for (let m = 0; m < k; m++) {
          row3[params[m]] = (ranges[m].start + ranges[m].end) / 2
        }
        row3[params[i]] = ranges[i].start
        row3[params[j]] = ranges[j].end
        results.push(row3)

        // (max, min)
        const row4: Record<string, number> = {}
        for (let m = 0; m < k; m++) {
          row4[params[m]] = (ranges[m].start + ranges[m].end) / 2
        }
        row4[params[i]] = ranges[i].end
        row4[params[j]] = ranges[j].start
        results.push(row4)
      }
    }

    // 所有中心点
    const center: Record<string, number> = {}
    for (let i = 0; i < k; i++) {
      center[params[i]] = (ranges[i].start + ranges[i].end) / 2
    }
    results.push(center)

    return results
  }

  /** Central Composite Design (CCD) */
  function generateCentralComposite(
    params: string[],
    levels: number,
    ranges: ScanRange[]
  ): Array<Record<string, number>> {
    const k = params.length
    const results: Array<Record<string, number>> = []

    // 立方点：2^k 个全因子
    const factorialPoints = Math.pow(2, k)
    for (let i = 0; i < factorialPoints; i++) {
      const row: Record<string, number> = {}
      let remainder = i
      for (let j = 0; j < k; j++) {
        const levelIndex = remainder % 2
        remainder = Math.floor(remainder / 2)
        row[params[j]] = levelIndex === 0 ? ranges[j].start : ranges[j].end
      }
      results.push(row)
    }

    // 轴向点：每个因子 ±α（α ≈ √k）
    const alpha = Math.pow(k, 0.25) * 2
    for (let i = 0; i < k; i++) {
      // +α 方向
      const rowPos: Record<string, number> = {}
      for (let j = 0; j < k; j++) {
        const center = (ranges[j].start + ranges[j].end) / 2
        const halfRange = (ranges[j].end - ranges[j].start) / 2
        rowPos[params[j]] = center + halfRange * alpha
      }
      results.push(rowPos)

      // -α 方向
      const rowNeg: Record<string, number> = {}
      for (let j = 0; j < k; j++) {
        const center = (ranges[j].start + ranges[j].end) / 2
        const halfRange = (ranges[j].end - ranges[j].start) / 2
        rowNeg[params[j]] = center - halfRange * alpha
      }
      results.push(rowNeg)
    }

    // 中心点
    const center: Record<string, number> = {}
    for (let i = 0; i < k; i++) {
      center[params[i]] = (ranges[i].start + ranges[i].end) / 2
    }
    results.push(center)

    return results
  }

  /** 拉丁超立方设计 */
  function generateLatinHypercube(
    params: string[],
    levels: number,
    ranges: ScanRange[]
  ): Array<Record<string, number>> {
    const k = params.length
    const results: Array<Record<string, number>> = []

    // 生成随机拉丁超立方
    const permutations: number[][] = []
    for (let i = 0; i < k; i++) {
      const perm = Array.from({ length: levels }, (_, j) => j)
      // Fisher-Yates 洗牌
      for (let j = perm.length - 1; j > 0; j--) {
        const rand = Math.floor(Math.random() * (j + 1));
        [perm[j], perm[rand]] = [perm[rand], perm[j]]
      }
      permutations.push(perm)
    }

    for (let i = 0; i < levels; i++) {
      const row: Record<string, number> = {}
      for (let j = 0; j < k; j++) {
        const levelIndex = permutations[j][i]
        const t = levels > 1 ? levelIndex / (levels - 1) : 0
        row[params[j]] = ranges[j].start + t * (ranges[j].end - ranges[j].start)
      }
      results.push(row)
    }

    return results
  }

  /** 更新任务状态 */
  function updateJobStatus(
    jobId: string,
    status: BatchJob['status']
  ) {
    const job = batchJobs.value.find(j => j.id === jobId)
    if (!job) return

    job.status = status
    if (status === 'running' && !job.startedAt) {
      job.startedAt = new Date().toISOString()
    }
    if (status === 'completed' || status === 'failed') {
      job.completedAt = new Date().toISOString()
    }
  }

  /** 更新单个结果 */
  function updateResult(
    jobId: string,
    caseIndex: number,
    result: Partial<BatchJobResult>
  ) {
    const job = batchJobs.value.find(j => j.id === jobId)
    if (!job) return

    const res = job.results[caseIndex]
    if (!res) return

    Object.assign(res, result)

    if (result.status === 'success') job.completedCount++
    if (result.status === 'failed') job.failedCount++

    // 更新进度
    currentProgress.value = job.completedCount / job.totalCount

    // 检查是否完成
    if (job.completedCount + job.failedCount >= job.totalCount) {
      job.status = job.failedCount === 0 ? 'completed' : 'failed'
      job.completedAt = new Date().toISOString()
    }
  }

  /** 生成扫描结果汇总报告 */
  function generateSummary(job: BatchJob): ScanSummary {
    const successfulResults = job.results.filter(r => r.status === 'success')
    const allMetrics = successfulResults
      .filter(r => r.outputMetrics)
      .map(r => r.outputMetrics!)

    if (allMetrics.length === 0) {
      return { totalRuns: job.totalCount, successfulRuns: job.completedCount, failedRuns: job.failedCount, metrics: {}, sensitivityRanking: [] }
    }

    // 计算各指标的统计值
    const metrics: ScanSummary['metrics'] = {}

    const metricKeys = ['maxStress', 'maxDisplacement', 'safetyFactor', 'firstFrequency'] as const
    for (const key of metricKeys) {
      const values = allMetrics.map(m => m[key]).filter((v): v is number => v !== undefined)
      if (values.length > 0) {
        const mean = values.reduce((a, b) => a + b, 0) / values.length
        const variance = values.reduce((sum, v) => sum + Math.pow(v - mean, 2), 0) / values.length
        metrics[key] = {
          min: Math.min(...values),
          max: Math.max(...values),
          mean,
          std: Math.sqrt(variance)
        }
      }
    }

    // 计算敏感度排名（基于方差分析）
    const sensitivityRanking = computeSensitivityRanking(job, metrics)

    // 找到最优案例
    let bestCase: ScanSummary['bestCase'] | undefined
    if (metrics.safetyFactor) {
      // 安全系数越大越好
      const best = successfulResults
        .filter(r => r.outputMetrics?.safetyFactor !== undefined)
        .sort((a, b) => (b.outputMetrics?.safetyFactor || 0) - (a.outputMetrics?.safetyFactor || 0))[0]
      if (best) {
        bestCase = {
          params: best.parameterSet,
          metric: 'safetyFactor',
          value: best.outputMetrics?.safetyFactor || 0
        }
      }
    }

    return {
      totalRuns: job.totalCount,
      successfulRuns: job.completedCount,
      failedRuns: job.failedCount,
      metrics,
      sensitivityRanking,
      bestCase
    }
  }

  /** 计算参数敏感度（简化的方差分析） */
  function computeSensitivityRanking(
    job: BatchJob,
    metrics: ScanSummary['metrics']
  ): Array<{ parameter: string; sensitivity: number }> {
    const results = job.results.filter(r => r.status === 'success' && r.outputMetrics)
    if (results.length === 0) return []

    // 获取所有参数名
    const paramNames = Object.keys(results[0].parameterSet)

    // 对于每个参数，计算其对各指标的影响（用相关系数近似）
    const rankings: Array<{ parameter: string; sensitivity: number }> = []

    for (const paramName of paramNames) {
      const paramValues = results.map(r => r.parameterSet[paramName])
      let totalSensitivity = 0
      let metricCount = 0

      for (const metricKey of Object.keys(metrics) as Array<keyof typeof metrics>) {
        const metricVals = results
          .map(r => r.outputMetrics?.[metricKey])
          .filter((v): v is number => v !== undefined)

        if (paramValues.length === metricVals.length && metricVals.length > 1) {
          // 计算 Pearson 相关系数
          const corr = pearsonCorrelation(paramValues, metricVals)
          totalSensitivity += Math.abs(corr)
          metricCount++
        }
      }

      const avgSensitivity = metricCount > 0 ? totalSensitivity / metricCount : 0
      rankings.push({ parameter: paramName, sensitivity: avgSensitivity })
    }

    // 按敏感度降序排列
    return rankings.sort((a, b) => b.sensitivity - a.sensitivity)
  }

  /** Pearson 相关系数 */
  function pearsonCorrelation(x: number[], y: number[]): number {
    const n = x.length
    if (n !== y.length || n === 0) return 0

    const meanX = x.reduce((a, b) => a + b, 0) / n
    const meanY = y.reduce((a, b) => a + b, 0) / n

    let num = 0
    let denX = 0
    let denY = 0

    for (let i = 0; i < n; i++) {
      const dx = x[i] - meanX
      const dy = y[i] - meanY
      num += dx * dy
      denX += dx * dx
      denY += dy * dy
    }

    const den = Math.sqrt(denX * denY)
    return den === 0 ? 0 : num / den
  }

  /** 删除批处理任务 */
  function deleteJob(jobId: string) {
    batchJobs.value = batchJobs.value.filter(j => j.id !== jobId)
    if (activeJobId.value === jobId) {
      activeJobId.value = null
    }
  }

  /** 导出结果为 CSV */
  function exportResultsCSV(job: BatchJob): string {
    const headers = ['Run', ...Object.keys(job.results[0]?.parameterSet || {}), 'Status', 'Duration(s)', 'MaxStress', 'MaxDisplacement', 'SafetyFactor']
    const lines = [headers.join(',')]

    for (const result of job.results) {
      const row = [
        result.caseIndex + 1,
        ...Object.values(result.parameterSet),
        result.status,
        result.duration?.toFixed(2) || '',
        result.outputMetrics?.maxStress?.toFixed(2) || '',
        result.outputMetrics?.maxDisplacement?.toFixed(4) || '',
        result.outputMetrics?.safetyFactor?.toFixed(3) || ''
      ]
      lines.push(row.join(','))
    }

    return lines.join('\n')
  }

  return {
    batchJobs,
    activeJobId,
    activeJob,
    isGeneratingDOE,
    currentProgress,
    createBatchJob,
    generateLinearScan,
    generateDOE,
    updateJobStatus,
    updateResult,
    generateSummary,
    deleteJob,
    exportResultsCSV
  }
}