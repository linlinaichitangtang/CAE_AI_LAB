/**
 * useBenchmark.ts — V3.1-001 Benchmark 发布
 * 发布与实验数据的对比基准，建立工程可信度
 */

import { ref } from 'vue'

// ============ 类型定义 ============

export interface BenchmarkCase {
  id: string
  name: string
  description: string
  category: 'cantilever' | 'plate' | 'shell' | 'impact' | 'thermal' | 'modal'
  difficulty: 'basic' | 'intermediate' | 'advanced'
  meshSizes: string[]  // ['coarse', 'medium', 'fine']
  referenceData: ReferenceData
  tolerance: ToleranceConfig
  createdAt: string
  author: string
  verified: boolean
}

export interface ReferenceData {
  source: string  // 论文/实验报告
  maxDisplacement?: number
  maxDisplacementUnit?: string
  maxStress?: number
  maxStressUnit?: string
  firstFrequency?: number
  frequencyUnit?: string
  stressDistribution?: { location: string; value: number }[]
  displacementField?: { x: number; y: number; z: number; value: number }[]
}

export interface ToleranceConfig {
  displacement: number  // 允许误差百分比
  stress: number
  frequency: number
  maxAllowedError: number  // 默认 5%
}

export interface BenchmarkResult {
  caseId: string
  userResult: UserResult
  comparison: ComparisonResult
  passed: boolean
  executedAt: string
  executionTime: number  // ms
}

export interface UserResult {
  maxDisplacement?: number
  maxStress?: number
  firstFrequency?: number
  meshNodes?: number
  meshElements?: number
}

export interface ComparisonResult {
  displacementError?: number  // 百分比
  stressError?: number
  frequencyError?: number
  overallScore: number  // 0-100
  details: string[]
}

// ============ 预定义 Benchmark 算例 ============

export const BENCHMARK_CASES: BenchmarkCase[] = [
  {
    id: 'cantilever-static-001',
    name: '悬臂梁静力学分析',
    description: '经典悬臂梁端部受集中力，与 Euler-Bernoulli 梁理论解对比',
    category: 'cantilever',
    difficulty: 'basic',
    meshSizes: ['coarse', 'medium', 'fine'],
    referenceData: {
      source: 'Timoshenko, S. Strength of Materials',
      maxDisplacement: 0.030,
      maxDisplacementUnit: 'm',
      maxStress: 75.0,
      maxStressUnit: 'MPa'
    },
    tolerance: {
      displacement: 10,
      stress: 15,
      frequency: 10,
      maxAllowedError: 5
    },
    createdAt: '2026-01-15',
    author: 'CAELab Team',
    verified: true
  },
  {
    id: 'cantilever-modal-001',
    name: '悬臂梁模态分析',
    description: '悬臂梁前四阶固有频率，与解析解对比',
    category: 'modal',
    difficulty: 'basic',
    meshSizes: ['coarse', 'medium', 'fine'],
    referenceData: {
      source: 'Meirovitch, L. Fundamentals of Vibrations',
      firstFrequency: 18.6,
      frequencyUnit: 'Hz'
    },
    tolerance: {
      displacement: 10,
      stress: 15,
      frequency: 5,
      maxAllowedError: 5
    },
    createdAt: '2026-01-20',
    author: 'CAELab Team',
    verified: true
  },
  {
    id: 'simply-supported-plate-001',
    name: '简支板弯曲分析',
    description: '均布载荷下简支矩形板的挠度和应力，与 Navier 解析解对比',
    category: 'plate',
    difficulty: 'intermediate',
    meshSizes: ['coarse', 'medium', 'fine'],
    referenceData: {
      source: 'Timoshenko, S. & Woinowsky-Krieger, S. Theory of Plates and Shells',
      maxDisplacement: 0.012,
      maxDisplacementUnit: 'm',
      maxStress: 45.0,
      maxStressUnit: 'MPa'
    },
    tolerance: {
      displacement: 10,
      stress: 12,
      frequency: 10,
      maxAllowedError: 5
    },
    createdAt: '2026-02-01',
    author: 'CAELab Team',
    verified: true
  },
  {
    id: 'shell-bending-001',
    name: '圆柱壳弯曲',
    description: '轴向压缩下圆柱壳的屈曲载荷，与经典公式对比',
    category: 'shell',
    difficulty: 'advanced',
    meshSizes: ['coarse', 'medium', 'fine'],
    referenceData: {
      source: 'Brush, D.O. & Almroth, B.O. Buckling of Bars, Plates, and Shells',
      maxStress: 120.0,
      maxStressUnit: 'MPa'
    },
    tolerance: {
      displacement: 15,
      stress: 10,
      frequency: 10,
      maxAllowedError: 8
    },
    createdAt: '2026-02-10',
    author: 'CAELab Team',
    verified: false
  },
  {
    id: 'impact-dynamic-001',
    name: '冲击动力学分析',
    description: '落锤冲击梁的动态响应，与实验数据对比',
    category: 'impact',
    difficulty: 'advanced',
    meshSizes: ['coarse', 'medium', 'fine'],
    referenceData: {
      source: 'Jones, N. Structural Impact',
      maxDisplacement: 0.085,
      maxDisplacementUnit: 'm'
    },
    tolerance: {
      displacement: 12,
      stress: 15,
      frequency: 10,
      maxAllowedError: 7
    },
    createdAt: '2026-02-20',
    author: 'CAELab Team',
    verified: false
  },
  {
    id: 'thermal-steady-001',
    name: '稳态热传导分析',
    description: '带孔板稳态热传导，温度场与解析解对比',
    category: 'thermal',
    difficulty: 'intermediate',
    meshSizes: ['coarse', 'medium', 'fine'],
    referenceData: {
      source: 'Incropera, F. Fundamentals of Heat and Mass Transfer',
    },
    tolerance: {
      displacement: 10,
      stress: 15,
      frequency: 10,
      maxAllowedError: 5
    },
    createdAt: '2026-03-01',
    author: 'CAELab Team',
    verified: false
  }
]

// ============ 工具函数 ============

function generateId(): string {
  return `bench_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T[] {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : []
}

function setStorage<T>(key: string, data: T[]): void {
  localStorage.setItem(key, JSON.stringify(data))
}

function calculateError(userValue: number, referenceValue: number): number {
  return Math.abs((userValue - referenceValue) / referenceValue * 100)
}

// ============ 主 Composable ============

export function useBenchmark() {
  const cases = ref<BenchmarkCase[]>([...BENCHMARK_CASES])
  const results = ref<BenchmarkResult[]>([])
  const currentCase = ref<BenchmarkCase | null>(null)
  const isRunning = ref(false)

  // 加载历史结果
  function loadResults(): BenchmarkResult[] {
    const stored = getStorage<BenchmarkResult>('caelab_benchmark_results')
    results.value = stored
    return stored
  }

  // 获取算例
  function getCases(category?: BenchmarkCase['category']): BenchmarkCase[] {
    if (category) {
      return cases.value.filter(c => c.category === category)
    }
    return cases.value
  }

  // 获取单个算例
  function getCase(caseId: string): BenchmarkCase | undefined {
    return cases.value.find(c => c.id === caseId)
  }

  // 运行 Benchmark
  function runBenchmark(
    caseId: string,
    userResult: UserResult
  ): BenchmarkResult | null {
    const benchmarkCase = getCase(caseId)
    if (!benchmarkCase) return null

    const startTime = Date.now()
    const comparison: ComparisonResult = {
      overallScore: 0,
      details: []
    }

    let totalError = 0
    let errorCount = 0

    // 对比位移
    if (userResult.maxDisplacement && benchmarkCase.referenceData.maxDisplacement) {
      const error = calculateError(
        userResult.maxDisplacement,
        benchmarkCase.referenceData.maxDisplacement
      )
      comparison.displacementError = error
      totalError += error
      errorCount++
      comparison.details.push(`最大位移误差: ${error.toFixed(2)}%`)
    }

    // 对比应力
    if (userResult.maxStress && benchmarkCase.referenceData.maxStress) {
      const error = calculateError(
        userResult.maxStress,
        benchmarkCase.referenceData.maxStress
      )
      comparison.stressError = error
      totalError += error
      errorCount++
      comparison.details.push(`最大应力误差: ${error.toFixed(2)}%`)
    }

    // 对比频率
    if (userResult.firstFrequency && benchmarkCase.referenceData.firstFrequency) {
      const error = calculateError(
        userResult.firstFrequency,
        benchmarkCase.referenceData.firstFrequency
      )
      comparison.frequencyError = error
      totalError += error
      errorCount++
      comparison.details.push(`固有频率误差: ${error.toFixed(2)}%`)
    }

    // 计算总分 (100 - 平均误差)
    const avgError = errorCount > 0 ? totalError / errorCount : 100
    comparison.overallScore = Math.max(0, 100 - avgError)

    // 判断是否通过
    const passed = avgError <= benchmarkCase.tolerance.maxAllowedError

    const result: BenchmarkResult = {
      caseId,
      userResult,
      comparison,
      passed,
      executedAt: new Date().toISOString(),
      executionTime: Date.now() - startTime
    }

    // 保存结果
    results.value.unshift(result)
    if (results.value.length > 100) {
      results.value.splice(100)
    }
    saveResults()

    return result
  }

  // 保存结果
  function saveResults(): void {
    setStorage('caelab_benchmark_results', results.value)
  }

  // 获取某算例的历史结果
  function getCaseResults(caseId: string): BenchmarkResult[] {
    return results.value.filter(r => r.caseId === caseId)
  }

  // 获取最佳结果
  function getBestResult(caseId: string): BenchmarkResult | undefined {
    const caseResults = getCaseResults(caseId)
    return caseResults.sort((a, b) => b.comparison.overallScore - a.comparison.overallScore)[0]
  }

  // 获取统计信息
  function getStats(): {
    totalCases: number
    verifiedCases: number
    totalRuns: number
    averageScore: number
    passRate: number
  } {
    const totalRuns = results.value.length
    const passedRuns = results.value.filter(r => r.passed).length
    const avgScore = totalRuns > 0
      ? results.value.reduce((sum, r) => sum + r.comparison.overallScore, 0) / totalRuns
      : 0

    return {
      totalCases: cases.value.length,
      verifiedCases: cases.value.filter(c => c.verified).length,
      totalRuns,
      averageScore: Math.round(avgScore * 10) / 10,
      passRate: totalRuns > 0 ? Math.round((passedRuns / totalRuns) * 100) : 0
    }
  }

  // 添加自定义算例
  function addCase(newCase: Omit<BenchmarkCase, 'id' | 'createdAt'>): BenchmarkCase {
    const benchmarkCase: BenchmarkCase = {
      ...newCase,
      id: generateId(),
      createdAt: new Date().toISOString()
    }
    cases.value.push(benchmarkCase)
    return benchmarkCase
  }

  // 清空历史
  function clearResults(): void {
    results.value = []
    saveResults()
  }

  // 初始化
  loadResults()

  return {
    cases,
    results,
    currentCase,
    isRunning,
    getCases,
    getCase,
    runBenchmark,
    getCaseResults,
    getBestResult,
    getStats,
    addCase,
    clearResults,
    BENCHMARK_CATEGORIES: ['cantilever', 'plate', 'shell', 'impact', 'thermal', 'modal'] as const
  }
}

// ============ Benchmark 分类信息 ============

export const BENCHMARK_CATEGORY_INFO: Record<BenchmarkCase['category'], { name: string; icon: string; description: string }> = {
  cantilever: {
    name: '悬臂梁',
    icon: '📐',
    description: '端部载荷悬臂结构，验证弯曲和扭转'
  },
  plate: {
    name: '板结构',
    icon: '🔲',
    description: '平面应力/应变板，验证膜行为'
  },
  shell: {
    name: '壳结构',
    icon: '🥚',
    description: '薄壳结构，验证屈曲和失稳'
  },
  impact: {
    name: '冲击',
    icon: '💥',
    description: '动态冲击载荷，验证非线性响应'
  },
  thermal: {
    name: '热分析',
    icon: '🌡️',
    description: '稳态/瞬态热传导'
  },
  modal: {
    name: '模态',
    icon: '📳',
    description: '固有频率和振型分析'
  }
}