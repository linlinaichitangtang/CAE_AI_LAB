/**
 * Multiscale Benchmark Library API
 * V1.8-005: 多尺度标准算例库 (10+ validated benchmarks)
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type BenchmarkCategory =
  | 'solidification'
  | 'phase_transformation'
  | 'fracture'
  | 'creep'
  | 'diffusion'
  | 'elasticity'
  | 'thermal'

export type ScaleCoverage =
  | 'dft'
  | 'md'
  | 'phase_field'
  | 'fe'
  | 'multiscale'

export interface BenchmarkCase {
  id: string
  name: string
  name_zh: string
  category: BenchmarkCategory
  description: string
  scales: ScaleCoverage[]
  reference_data: {
    doi: string
    year: number
    authors: string
  }
  expected_results: {
    quantity: string
    value: number
    tolerance: number
    unit: string
  }[]
  difficulty: 'beginner' | 'intermediate' | 'advanced'
  estimated_time_min: number
}

export interface BenchmarkRun {
  benchmark_id: string
  status: 'pending' | 'running' | 'completed' | 'failed'
  results: {
    quantity: string
    computed: number
    reference: number
    error_percent: number
  }[]
  total_time_s: number
  passed: boolean
}

export interface BenchmarkRunConfig {
  benchmark_id: string
  parameters: Record<string, unknown>
  max_time_s: number
}

// ============ API 函数 ============

/**
 * 获取标准算例列表（可按类别筛选）
 */
export async function listBenchmarks(
  category?: BenchmarkCategory
): Promise<BenchmarkCase[]> {
  return invoke<BenchmarkCase[]>('list_benchmarks', { category })
}

/**
 * 获取算例详情
 */
export async function getBenchmarkDetail(
  id: string
): Promise<BenchmarkCase> {
  return invoke<BenchmarkCase>('get_benchmark_detail', { id })
}

/**
 * 运行标准算例
 */
export async function runBenchmark(
  config: BenchmarkRunConfig
): Promise<BenchmarkRun> {
  return invoke<BenchmarkRun>('run_benchmark', { config })
}

/**
 * 与参考结果对比
 */
export async function compareWithReference(
  benchmark_id: string,
  results: Record<string, number>
): Promise<{
  passed: boolean
  max_error_percent: number
  details: { quantity: string; error: number }[]
}> {
  return invoke<{
    passed: boolean
    max_error_percent: number
    details: { quantity: string; error: number }[]
  }>('compare_with_reference', { benchmarkId: benchmark_id, results })
}

/**
 * 获取算例库统计信息
 */
export async function getBenchmarkStatistics(): Promise<{
  total: number
  by_category: Record<string, number>
  pass_rate: number
}> {
  return invoke<{
    total: number
    by_category: Record<string, number>
    pass_rate: number
  }>('get_benchmark_statistics')
}
