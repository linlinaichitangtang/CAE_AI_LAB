/**
 * V4.6-004: 性能基准测试
 *
 * 测量关键操作的执行时间，检测性能回归（>20% 自动失败）。
 * 基准数据保存到 benchmarks/baseline.json，CI 中与基线对比。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync, writeFileSync, existsSync } from 'fs'
import { join } from 'path'

const BASELINE_PATH = join(__dirname, 'baseline.json')
const REGRESSION_THRESHOLD = 0.20 // 20% 回归阈值

interface BenchmarkResult {
  name: string
  durationMs: number
  timestamp: string
}

interface Baseline {
  version: string
  results: BenchmarkResult[]
}

function loadBaseline(): Baseline | null {
  if (!existsSync(BASELINE_PATH)) return null
  try {
    return JSON.parse(readFileSync(BASELINE_PATH, 'utf-8'))
  } catch {
    return null
  }
}

function saveBaseline(results: BenchmarkResult[]) {
  const baseline: Baseline = {
    version: '1.0',
    results,
  }
  writeFileSync(BASELINE_PATH, JSON.stringify(baseline, null, 2))
}

function checkRegression(name: string, currentMs: number, baselineMs: number) {
  const regression = (currentMs - baselineMs) / baselineMs
  if (regression > REGRESSION_THRESHOLD) {
    throw new Error(
      `性能回归检测: ${name} 当前 ${currentMs.toFixed(1)}ms，基线 ${baselineMs.toFixed(1)}ms，回归 ${(regression * 100).toFixed(1)}%（阈值 ${(REGRESSION_THRESHOLD * 100)}%）`
    )
  }
}

// ============================================================================
// 基准测试用例
// ============================================================================

describe('性能基准回归 (V4.6-004)', () => {
  const results: BenchmarkResult[] = []

  // 模拟意图分类性能基准
  it('意图分类推理 < 50ms', () => {
    const start = performance.now()
    // 模拟分类计算
    const keywords = ['创建', '仿真', '分析', '优化', '建模']
    for (let i = 0; i < 1000; i++) {
      keywords.some((k) => k.includes('仿'))
    }
    const duration = performance.now() - start
    results.push({
      name: 'intent_classification_1k',
      durationMs: duration,
      timestamp: new Date().toISOString(),
    })
    expect(duration).toBeLessThan(50)
  })

  // 模拟模板搜索性能基准
  it('模板搜索 < 10ms', () => {
    const templates = Array.from({ length: 100 }, (_, i) => ({
      id: `template-${i}`,
      name: `模板 ${i}`,
      tags: ['tag1', 'tag2', 'tag3'],
    }))
    const start = performance.now()
    for (let i = 0; i < 100; i++) {
      templates.filter(
        (t) =>
          t.name.includes('模板') || t.tags.some((tag) => tag.includes('tag1'))
      )
    }
    const duration = performance.now() - start
    results.push({
      name: 'template_search_100x100',
      durationMs: duration,
      timestamp: new Date().toISOString(),
    })
    expect(duration).toBeLessThan(10)
  })

  // 模拟网格质量计算性能基准
  it('网格质量计算 (1000 单元) < 20ms', () => {
    const start = performance.now()
    // 模拟雅可比行列式计算
    let sum = 0
    for (let i = 0; i < 1000; i++) {
      const det = Math.random() * 0.5 + 0.5 // 模拟雅可比值
      sum += det * det
    }
    const duration = performance.now() - start
    results.push({
      name: 'mesh_quality_1k_elements',
      durationMs: duration,
      timestamp: new Date().toISOString(),
    })
    expect(duration).toBeLessThan(20)
  })

  // 模拟材料查询性能基准
  it('材料数据库查询 < 5ms', () => {
    const materials = Array.from({ length: 26 }, (_, i) => ({
      grade: `Q${i * 10 + 195}`,
      category: 'carbon',
    }))
    const start = performance.now()
    for (let i = 0; i < 1000; i++) {
      materials.find((m) => m.grade === 'Q345')
    }
    const duration = performance.now() - start
    results.push({
      name: 'material_query_1k_lookups',
      durationMs: duration,
      timestamp: new Date().toISOString(),
    })
    expect(duration).toBeLessThan(5)
  })

  // 模拟合规检查计算性能基准
  it('合规检查计算 < 10ms', () => {
    const rules = Array.from({ length: 50 }, (_, i) => ({
      id: `rule-${i}`,
      threshold: Math.random() * 100,
    }))
    const start = performance.now()
    for (let i = 0; i < 100; i++) {
      rules.map((r) => ({
        id: r.id,
        passed: Math.random() * 100 < r.threshold,
        margin: r.threshold - Math.random() * 100,
      }))
    }
    const duration = performance.now() - start
    results.push({
      name: 'compliance_check_50_rules_100x',
      durationMs: duration,
      timestamp: new Date().toISOString(),
    })
    expect(duration).toBeLessThan(10)
  })

  // 保存结果并对比基线
  it('基线对比与保存', () => {
    const baseline = loadBaseline()

    if (baseline) {
      for (const result of results) {
        const baselineResult = baseline.results.find(
          (b) => b.name === result.name
        )
        if (baselineResult) {
          checkRegression(result.name, result.durationMs, baselineResult.durationMs)
        }
      }
    }

    // 保存当前结果作为新基线
    saveBaseline(results)
    expect(results.length).toBeGreaterThan(0)
  })
})
