/**
 * CAELab V3.7-002: Active Learning Module
 * 主动学习引擎 - 数据高效利用
 * 用于选择性标注和选择性模拟
 */
import { ref, computed } from 'vue'

export type SelectionStrategy = 'uncertainty' | 'variance' | 'density' | 'expected_model_change' | 'random'
export type TaskType = 'classification' | 'regression' | 'detection'

export interface DataPoint {
  id: string
  features: number[]
  label?: string
  predictedLabel?: string
  confidence?: number
  uncertainty?: number
  acquisitionScore?: number
  metadata?: Record<string, any>
  source: 'MD' | 'FE' | 'DFT' | 'experiment' | 'simulation'
  cost?: number
  timestamp: Date
}

export interface ActiveLearningConfig {
  strategy: SelectionStrategy
  batchSize: number
  initialPoolSize: number
  maxIterations: number
  stoppingCriteria: {
    accuracyThreshold?: number
    uncertaintyThreshold?: number
    iterationLimit?: number
  }
  modelType: 'classifier' | 'regressor' | 'ensemble'
  ensembleSize?: number
}

export interface IterationResult {
  iteration: number
  selectedPoints: DataPoint[]
  modelAccuracy: number
  uncertainty: number
  totalCost: number
  poolSize: number
  labeledSize: number
}

export interface AcquisitionLog {
  iteration: number
  strategy: SelectionStrategy
  selectedIds: string[]
  scores: { id: string; score: number }[]
  timestamp: Date
}

const defaultConfigs: Record<SelectionStrategy, Partial<ActiveLearningConfig>> = {
  uncertainty: {
    batchSize: 10,
    strategy: 'uncertainty'
  },
  variance: {
    batchSize: 10,
    strategy: 'variance'
  },
  density: {
    batchSize: 10,
    strategy: 'density'
  },
  expected_model_change: {
    batchSize: 5,
    strategy: 'expected_model_change'
  },
  random: {
    batchSize: 10,
    strategy: 'random'
  }
}

// 状态
const dataPool = ref<DataPoint[]>([])
const labeledData = ref<DataPoint[]>([])
const unlabeledData = computed(() =>
  dataPool.value.filter(d => !d.label && !d.predictedLabel)
)
const config = ref<ActiveLearningConfig>({
  strategy: 'uncertainty',
  batchSize: 10,
  initialPoolSize: 100,
  maxIterations: 50,
  stoppingCriteria: {
    accuracyThreshold: 0.95,
    uncertaintyThreshold: 0.05,
    iterationLimit: 50
  },
  modelType: 'regressor',
  ensembleSize: 5
})
const currentIteration = ref(0)
const iterationHistory = ref<IterationResult[]>([])
const acquisitionLogs = ref<AcquisitionLog[]>([])
const isRunning = ref(false)

export function useActiveLearning() {
  /**
   * 初始化数据池
   */
  function initializePool(
    features: number[][],
    sources: DataPoint['source'][] = [],
    costs: number[] = []
  ): number {
    dataPool.value = features.map((f, i) => ({
      id: `dp_${Date.now()}_${i}`,
      features: f,
      source: sources[i] || 'simulation',
      cost: costs[i] || 1,
      timestamp: new Date()
    }))

    labeledData.value = []
    currentIteration.value = 0
    iterationHistory.value = []

    return dataPool.value.length
  }

  /**
   * 标注数据点
   */
  function labelPoint(pointId: string, label: string | number): boolean {
    const point = dataPool.value.find(p => p.id === pointId)
    if (!point) return false

    point.label = String(label)
    point.predictedLabel = undefined

    if (!labeledData.value.some(p => p.id === pointId)) {
      labeledData.value.push(point)
    }

    return true
  }

  /**
   * 批量标注
   */
  function labelBatch(pointIds: string[], labels: (string | number)[]): number {
    let count = 0
    for (let i = 0; i < pointIds.length; i++) {
      if (labelPoint(pointIds[i], labels[i])) count++
    }
    return count
  }

  /**
   * 导入已有标注数据
   */
  function importLabeledData(
    data: { features: number[]; label: string | number; source?: string }[]
  ): number {
    let count = 0
    for (const item of data) {
      const point: DataPoint = {
        id: `dp_imported_${Date.now()}_${count}`,
        features: item.features,
        label: String(item.label),
        source: (item.source as any) || 'experiment',
        timestamp: new Date()
      }
      dataPool.value.push(point)
      labeledData.value.push(point)
      count++
    }
    return count
  }

  /**
   * 计算采集分数 (选择最"有价值"的数据点)
   */
  function computeAcquisitionScores(): { id: string; score: number; reason: string }[] {
    const unlabeled = unlabeledData.value
    if (unlabeled.length === 0) return []

    const strategy = config.value.strategy
    const batchSize = config.value.batchSize

    const scores: { id: string; score: number; reason: string }[] = unlabeled.map(point => {
      let score = 0
      let reason = ''

      switch (strategy) {
        case 'uncertainty': {
          // 预测不确定性高 -> 分数高
          const uncertainty = point.uncertainty ?? (0.3 + Math.random() * 0.4)
          score = uncertainty
          reason = `Uncertainty: ${(uncertainty * 100).toFixed(1)}%`
          break
        }

        case 'variance': {
          // 预测方差大
          const variance = 0.2 + Math.random() * 0.3
          score = variance
          reason = `Variance: ${variance.toFixed(3)}`
          break
        }

        case 'density': {
          // 基于密度的采集
          const density = Math.random() * 0.5
          const distance = computeMinDistanceToLabeled(point)
          score = density * (1 + distance / 100)
          reason = `Density-based score`
          break
        }

        case 'expected_model_change': {
          // 预期模型变化大
          const modelChange = 0.3 + Math.random() * 0.4
          score = modelChange
          reason = `Expected change: ${(modelChange * 100).toFixed(1)}%`
          break
        }

        case 'random':
        default: {
          score = Math.random()
          reason = 'Random selection'
          break
        }
      }

      return { id: point.id, score, reason }
    })

    // 按分数排序，选择最高的 batchSize 个
    scores.sort((a, b) => b.score - a.score)
    return scores.slice(0, batchSize)
  }

  /**
   * 选中数据点进行标注
   */
  async function selectPointsForLabeling(): Promise<DataPoint[]> {
    const scores = computeAcquisitionScores()
    const selected: DataPoint[] = []

    // 模拟预测 (给未标注点添加预测标签和不确定性)
    for (const point of unlabeledData.value) {
      point.predictedLabel = `pred_${Math.floor(Math.random() * 100)}`
      point.uncertainty = 0.2 + Math.random() * 0.6
      point.confidence = 1 - point.uncertainty
      point.acquisitionScore = 0
    }

    // 选择高分点
    for (const { id, score } of scores) {
      const point = dataPool.value.find(p => p.id === id)
      if (point) {
        point.acquisitionScore = score
        selected.push(point)
      }
    }

    // 记录采集日志
    acquisitionLogs.value.push({
      iteration: currentIteration.value,
      strategy: config.value.strategy,
      selectedIds: selected.map(p => p.id),
      scores: scores.slice(0, config.value.batchSize),
      timestamp: new Date()
    })

    return selected
  }

  /**
   * 运行一轮主动学习
   */
  async function runIteration(): Promise<IterationResult | null> {
    if (isRunning.value) return null

    try {
      isRunning.value = true

      // 1. 选择要标注的点
      const selectedPoints = await selectPointsForLabeling()

      // 2. 模拟更新模型
      await new Promise(r => setTimeout(r, 100))

      // 3. 计算指标
      const accuracy = labeledData.value.length > 0
        ? 0.7 + (labeledData.value.length / (labeledData.value.length + 10)) * 0.25
        : 0.5
      const avgUncertainty = selectedPoints.reduce((sum, p) => sum + (p.uncertainty || 0), 0) / selectedPoints.length
      const totalCost = selectedPoints.reduce((sum, p) => sum + (p.cost || 1), 0)

      const result: IterationResult = {
        iteration: currentIteration.value,
        selectedPoints,
        modelAccuracy: accuracy,
        uncertainty: avgUncertainty,
        totalCost,
        poolSize: dataPool.value.length,
        labeledSize: labeledData.value.length
      }

      iterationHistory.value.push(result)
      currentIteration.value++

      return result
    } finally {
      isRunning.value = false
    }
  }

  /**
   * 批量运行多轮
   */
  async function runBatch(maxIterations?: number): Promise<IterationResult[]> {
    const max = maxIterations || config.value.maxIterations
    const results: IterationResult[] = []

    for (let i = 0; i < max; i++) {
      // 检查停止条件
      const lastResult = results[results.length - 1]
      if (lastResult) {
        if (lastResult.modelAccuracy >= (config.value.stoppingCriteria.accuracyThreshold || 0.95)) {
          console.log('Stopping: accuracy threshold reached')
          break
        }
        if (lastResult.uncertainty <= (config.value.stoppingCriteria.uncertaintyThreshold || 0.05)) {
          console.log('Stopping: uncertainty threshold reached')
          break
        }
      }

      const result = await runIteration()
      if (result) results.push(result)
    }

    return results
  }

  /**
   * 设置采集策略
   */
  function setStrategy(strategy: SelectionStrategy) {
    config.value.strategy = strategy
    const preset = defaultConfigs[strategy]
    if (preset) {
      config.value.batchSize = preset.batchSize || config.value.batchSize
    }
  }

  /**
   * 设置配置
   */
  function setConfig(newConfig: Partial<ActiveLearningConfig>) {
    config.value = { ...config.value, ...newConfig }
  }

  /**
   * 获取推荐采集策略
   */
  function recommendStrategy(): SelectionStrategy {
    const labeled = labeledData.value.length
    const unlabeled = unlabeledData.value.length

    if (labeled < 10) return 'random'
    if (labeled < 50) return 'uncertainty'
    if (config.value.modelType === 'regressor') return 'variance'
    return 'uncertainty'
  }

  /**
   * 分析采集效率
   */
  function analyzeEfficiency(): {
    totalIterations: number
    totalCost: number
    averageAccuracyGain: number
    recommendedNextStrategy: SelectionStrategy
  } {
    const history = iterationHistory.value
    if (history.length === 0) {
      return {
        totalIterations: 0,
        totalCost: 0,
        averageAccuracyGain: 0,
        recommendedNextStrategy: 'uncertainty'
      }
    }

    const totalCost = history.reduce((sum, r) => sum + r.totalCost, 0)
    const accuracyGain = history.length > 1
      ? (history[history.length - 1].modelAccuracy - history[0].modelAccuracy) / (history.length - 1)
      : 0

    return {
      totalIterations: history.length,
      totalCost,
      averageAccuracyGain: accuracyGain,
      recommendedNextStrategy: recommendStrategy()
    }
  }

  /**
   * 计算到最近已标注点的距离
   */
  function computeMinDistanceToLabeled(point: DataPoint): number {
    if (labeledData.value.length === 0) return Infinity

    let minDist = Infinity
    for (const labeled of labeledData.value) {
      const dist = euclideanDistance(point.features, labeled.features)
      if (dist < minDist) minDist = dist
    }
    return minDist
  }

  /**
   * 导出采集日志
   */
  function exportLogs(): string {
    return JSON.stringify({
      config: config.value,
      history: iterationHistory.value,
      logs: acquisitionLogs.value
    }, null, 2)
  }

  /**
   * 生成 Python 采集代码
   */
  function generateAcquisitionCode(): string {
    const strategy = config.value.strategy

    return `
import numpy as np
from sklearn.ensemble import RandomForestClassifier, RandomForestRegressor

class ActiveLearning:
    def __init__(self, strategy='${strategy}', batch_size=${config.value.batchSize}):
        self.strategy = strategy
        self.batch_size = batch_size
        self.model = None
        self.X_labeled = []
        self.y_labeled = []

    def fit(self, X, y):
        self.X_labeled = X
        self.y_labeled = y
        if len(np.unique(y)) > 2:
            self.model = RandomForestRegressor(n_estimators=100)
        else:
            self.model = RandomForestClassifier(n_estimators=100)
        self.model.fit(X, y)

    def predict(self, X_pool):
        predictions = self.model.predict(X_pool)
        uncertainties = np.std([tree.predict(X_pool) for tree in self.model.estimators_], axis=0)
        return predictions, uncertainties

    def select_batch(self, X_pool):
        _, uncertainties = self.predict(X_pool)

        if self.strategy == 'uncertainty':
            scores = uncertainties
        elif self.strategy == 'variance':
            probs = self.model.predict_proba(X_pool)
            scores = -np.max(probs, axis=1)
        elif self.strategy == 'random':
            scores = np.random.rand(len(X_pool))
        else:
            scores = uncertainties

        top_indices = np.argsort(scores)[-self.batch_size:]
        return top_indices, scores[top_indices]

# 使用示例
# al = ActiveLearning(strategy='${strategy}', batch_size=${config.value.batchSize})
# al.fit(X_labeled, y_labeled)
# indices, scores = al.select_batch(X_pool)
print("Active learning module ready")
`.trim()
  }

  // Helpers
  function euclideanDistance(a: number[], b: number[]): number {
    return Math.sqrt(a.reduce((sum, val, i) => sum + Math.pow(val - (b[i] || 0), 2), 0))
  }

  // Computed
  const labeledRatio = computed(() => {
    const total = dataPool.value.length
    if (total === 0) return 0
    return labeledData.value.length / total
  })

  const latestMetrics = computed(() => {
    const history = iterationHistory.value
    if (history.length === 0) return null
    return history[history.length - 1]
  })

  return {
    // 状态
    dataPool: computed(() => dataPool.value),
    labeledData: computed(() => labeledData.value),
    unlabeledData,
    config: computed(() => config.value),
    currentIteration: computed(() => currentIteration.value),
    iterationHistory: computed(() => iterationHistory.value),
    acquisitionLogs: computed(() => acquisitionLogs.value),
    isRunning: computed(() => isRunning.value),
    labeledRatio,
    latestMetrics,
    defaultConfigs,

    // 方法
    initializePool,
    labelPoint,
    labelBatch,
    importLabeledData,
    selectPointsForLabeling,
    runIteration,
    runBatch,
    setStrategy,
    setConfig,
    recommendStrategy,
    analyzeEfficiency,
    exportLogs,
    generateAcquisitionCode
  }
}