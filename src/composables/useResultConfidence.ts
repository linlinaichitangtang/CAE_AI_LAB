/**
 * useResultConfidence.ts — V4.1-003 结果置信度系统
 * 每个仿真结果附带误差估计：网格收敛性、模型不确定性、边界条件敏感度
 */
import { ref, computed } from 'vue'

export interface ConfidenceDimension {
  name: string
  label: string
  value: number      // 0-1，越高越好
  errorEstimate: number // 百分比误差估计
  description: string
}

export interface ConfidenceReport {
  overallConfidence: number   // 综合置信度 0-1
  confidenceInterval: {       // 置信区间
    lower: number
    upper: number
    unit: string
  }
  dimensions: ConfidenceDimension[]
  recommendations: string[]
}

export function useResultConfidence() {
  const isAnalyzing = ref(false)
  const lastReport = ref<ConfidenceReport | null>(null)

  /**
   * 分析结果置信度
   * @param results 仿真结果
   * @param meshData 网格数据
   * @param history 历史收敛数据（可选）
   */
  async function analyzeConfidence(
    results: {
      maxStress?: number
      maxDisplacement?: number
      value?: number        // 通用值
      unit?: string
    },
    meshData?: {
      elementCount: number
      elementType: string
      maxAspectRatio: number
      minJacobian: number
      avgSkewness: number
    },
    history?: Array<{
      meshDensity: number   // 单元数或特征尺寸
      resultValue: number
    }>
  ): Promise<ConfidenceReport> {
    isAnalyzing.value = true

    try {
      const dimensions: ConfidenceDimension[] = []
      const recommendations: string[] = []

      // 1. 网格收敛性分析
      const meshConfidence = calculateMeshConfidence(meshData, history)
      dimensions.push(meshConfidence)

      if (meshConfidence.value < 0.7) {
        recommendations.push('建议加密网格，当前网格密度可能不足以捕捉应力梯度')
      }
      if (meshConfidence.errorEstimate > 10) {
        recommendations.push('网格收敛误差超过 10%，建议进行网格收敛性测试')
      }

      // 2. 网格质量分析
      const qualityConfidence = calculateQualityConfidence(meshData)
      dimensions.push(qualityConfidence)

      if (qualityConfidence.value < 0.6) {
        recommendations.push('部分单元质量较差，建议优化网格或使用自适应网格加密')
      }

      // 3. 模型不确定性（简化估计）
      const modelConfidence = calculateModelConfidence(results)
      dimensions.push(modelConfidence)

      if (modelConfidence.value < 0.8) {
        recommendations.push('模型简化可能引入较大误差，建议对比更精细的模型')
      }

      // 4. 边界条件敏感度
      const bcConfidence = calculateBCConfidence(results)
      dimensions.push(bcConfidence)

      if (bcConfidence.value < 0.7) {
        recommendations.push('边界条件可能对结果有显著影响，建议进行敏感度分析')
      }

      // 计算综合置信度（加权平均）
      const weights = { mesh: 0.35, quality: 0.25, model: 0.2, bc: 0.2 }
      const overallConfidence =
        meshConfidence.value * weights.mesh +
        qualityConfidence.value * weights.quality +
        modelConfidence.value * weights.model +
        bcConfidence.value * weights.bc

      // 计算综合误差估计
      const totalError = Math.sqrt(
        Math.pow(meshConfidence.errorEstimate, 2) +
        Math.pow(qualityConfidence.errorEstimate, 2) +
        Math.pow(modelConfidence.errorEstimate, 2) +
        Math.pow(bcConfidence.errorEstimate, 2)
      )

      const baseValue = results.value ?? results.maxStress ?? 0
      const halfInterval = (baseValue * totalError) / 200

      const report: ConfidenceReport = {
        overallConfidence,
        confidenceInterval: {
          lower: Math.max(0, baseValue - halfInterval),
          upper: baseValue + halfInterval,
          unit: results.unit ?? 'MPa'
        },
        dimensions,
        recommendations
      }

      lastReport.value = report
      return report
    } finally {
      isAnalyzing.value = false
    }
  }

  /**
   * 计算网格收敛性置信度
   */
  function calculateMeshConfidence(
    meshData?: {
      elementCount: number
      elementType: string
      maxAspectRatio: number
      minJacobian: number
      avgSkewness: number
    },
    history?: Array<{ meshDensity: number; resultValue: number }>
  ): ConfidenceDimension {
    if (!meshData) {
      return {
        name: 'mesh_convergence',
        label: '网格收敛性',
        value: 0.3,
        errorEstimate: 25,
        description: '网格信息缺失，无法评估收敛性'
      }
    }

    // 基于单元数量估算
    let value = 0.5
    let errorEstimate = 15

    if (meshData.elementCount > 100000) {
      value = 0.85
      errorEstimate = 3
    } else if (meshData.elementCount > 50000) {
      value = 0.75
      errorEstimate = 5
    } else if (meshData.elementCount > 10000) {
      value = 0.65
      errorEstimate = 8
    } else if (meshData.elementCount > 5000) {
      value = 0.55
      errorEstimate = 12
    } else {
      value = 0.4
      errorEstimate = 20
    }

    // 如果有历史收敛数据，计算收敛阶数
    if (history && history.length >= 2) {
      const sorted = [...history].sort((a, b) => a.meshDensity - b.meshDensity)
      const lastTwo = sorted.slice(-2)
      const ratio = lastTwo[1].meshDensity / lastTwo[0].meshDensity
      const resultChange = Math.abs(
        (lastTwo[1].resultValue - lastTwo[0].resultValue) / lastTwo[0].resultValue
      )

      if (resultChange < 0.02) {
        value = Math.min(0.95, value + 0.15)
        errorEstimate = Math.max(2, errorEstimate - 3)
      } else if (resultChange < 0.05) {
        value = Math.min(0.9, value + 0.1)
        errorEstimate = Math.max(3, errorEstimate - 2)
      } else {
        value = Math.max(0.3, value - 0.1)
        errorEstimate = Math.min(25, errorEstimate + 5)
      }
    }

    // 二阶单元精度更高
    if (meshData.elementType.includes('tet10') || meshData.elementType.includes('hex20')) {
      value = Math.min(0.95, value + 0.1)
      errorEstimate = Math.max(2, errorEstimate - 2)
    }

    return {
      name: 'mesh_convergence',
      label: '网格收敛性',
      value: Math.min(1, Math.max(0.1, value)),
      errorEstimate: Math.min(30, Math.max(2, errorEstimate)),
      description: history && history.length >= 2
        ? `基于 ${history.length} 次网格加密测试，收敛趋势${errorEstimate < 5 ? '良好' : errorEstimate < 10 ? '一般' : '需改进'}`
        : `基于网格规模估算（${meshData.elementCount} 单元），未进行收敛性测试`
    }
  }

  /**
   * 计算网格质量置信度
   */
  function calculateQualityConfidence(
    meshData?: {
      elementCount: number
      maxAspectRatio: number
      minJacobian: number
      avgSkewness: number
    }
  ): ConfidenceDimension {
    if (!meshData) {
      return {
        name: 'mesh_quality',
        label: '网格质量',
        value: 0.3,
        errorEstimate: 20,
        description: '网格质量信息缺失'
      }
    }

    // 根据各项指标评估
    let value = 0.7
    let errorEstimate = 8

    // Jacobian 检查
    if (meshData.minJacobian < 0) {
      value -= 0.3
      errorEstimate += 10
    } else if (meshData.minJacobian < 0.3) {
      value -= 0.15
      errorEstimate += 5
    }

    // 长宽比检查
    if (meshData.maxAspectRatio > 10) {
      value -= 0.2
      errorEstimate += 8
    } else if (meshData.maxAspectRatio > 5) {
      value -= 0.1
      errorEstimate += 4
    }

    // 歪斜度检查
    if (meshData.avgSkewness > 0.5) {
      value -= 0.15
      errorEstimate += 5
    } else if (meshData.avgSkewness > 0.3) {
      value -= 0.05
      errorEstimate += 2
    }

    const finalValue = Math.min(1, Math.max(0.2, value))
    const finalError = Math.min(25, Math.max(3, errorEstimate))

    return {
      name: 'mesh_quality',
      label: '网格质量',
      value: finalValue,
      errorEstimate: finalError,
      description: `Jacobian 最小值: ${meshData.minJacobian.toFixed(3)}, 最大长宽比: ${meshData.maxAspectRatio.toFixed(1)}, 平均歪斜度: ${meshData.avgSkewness.toFixed(3)}`
    }
  }

  /**
   * 计算模型不确定性
   */
  function calculateModelConfidence(
    _results: { maxStress?: number; maxDisplacement?: number; value?: number }
  ): ConfidenceDimension {
    // 简化模型：基于常见简化假设的误差估计
    const simplificationErrors = [
      { name: '材料线性假设', error: 3 },
      { name: '小变形假设', error: 2 },
      { name: '理想边界条件', error: 5 },
      { name: '忽略温度效应', error: 2 }
    ]

    const totalModelError = Math.sqrt(
      simplificationErrors.reduce((sum, item) => sum + Math.pow(item.error, 2), 0)
    )

    const value = Math.max(0.3, 1 - totalModelError / 30)

    return {
      name: 'model_uncertainty',
      label: '模型不确定性',
      value,
      errorEstimate: totalModelError,
      description: `包含材料线性假设(±3%)、小变形假设(±2%)、边界条件理想化(±5%)等误差源`
    }
  }

  /**
   * 计算边界条件敏感度
   */
  function calculateBCConfidence(
    _results: { maxStress?: number; maxDisplacement?: number; value?: number }
  ): ConfidenceDimension {
    // 边界条件通常引入 5-15% 的误差
    const typicalBCErrors = {
      fixed: 5,      // 固定约束可能过刚
      displacement: 8,  // 位移约束
      force: 10,     // 集中力简化
      pressure: 6,   // 均布压力
      contact: 15    // 接触最复杂
    }

    const avgError = Object.values(typicalBCErrors).reduce((a, b) => a + b, 0) / Object.values(typicalBCErrors).length
    const value = Math.max(0.4, 1 - avgError / 25)

    return {
      name: 'bc_sensitivity',
      label: '边界条件敏感度',
      value,
      errorEstimate: avgError,
      description: '基于典型边界条件误差估计，固定约束通常引入 ±5% 误差，接触分析可能高达 ±15%'
    }
  }

  return {
    isAnalyzing,
    lastReport,
    analyzeConfidence
  }
}
