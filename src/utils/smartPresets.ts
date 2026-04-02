/**
 * V1.1-008: 智能预设置引擎
 * 根据模型特征自动推荐网格和求解器参数
 */

// ============ 类型定义 ============

export interface MeshPreset {
  elementSize: number
  elementOrder: number // 1=linear, 2=quadratic
  meshType: string // 'tet', 'hex', 'mixed'
  reason: string
}

export interface SolverPreset {
  solverType: string
  maxIterations: number
  convergenceTolerance: number
  reason: string
}

export interface SmartPreset {
  mesh: MeshPreset
  solver: SolverPreset
  confidence: number // 0-1
}

export interface ModelFeatures {
  boundingBox: { x: number; y: number; z: number }
  nodeCount: number
  elementCount: number
  analysisType: string // 'static', 'modal', 'thermal', 'frequency', 'buckling', 'cfd'
  materialType: string // 'isotropic', 'orthotropic', 'anisotropic'
  hasContact: boolean
  complexity: 'simple' | 'medium' | 'complex'
}

// ============ 复杂度评估 ============

/**
 * 根据节点数自动评估模型复杂度
 */
export function assessComplexity(nodeCount: number): 'simple' | 'medium' | 'complex' {
  if (nodeCount < 1000) return 'simple'
  if (nodeCount <= 10000) return 'medium'
  return 'complex'
}

/**
 * 从网格数据中提取模型特征
 */
export function extractFeatures(params: {
  nodeCount: number
  elementCount: number
  boundingBox?: { x: number; y: number; z: number }
  analysisType?: string
  materialType?: string
  hasContact?: boolean
}): ModelFeatures {
  const {
    nodeCount,
    elementCount,
    boundingBox = { x: 100, y: 20, z: 10 },
    analysisType = 'static',
    materialType = 'isotropic',
    hasContact = false,
  } = params

  return {
    boundingBox,
    nodeCount,
    elementCount,
    analysisType,
    materialType,
    hasContact,
    complexity: assessComplexity(nodeCount),
  }
}

// ============ 推荐引擎 ============

/**
 * 根据模型特征推荐预设参数
 */
export function recommendPreset(features: ModelFeatures): SmartPreset {
  const mesh = recommendMesh(features)
  const solver = recommendSolver(features)
  const confidence = calculateConfidence(features, mesh, solver)

  return { mesh, solver, confidence }
}

/**
 * 推荐网格参数
 */
function recommendMesh(features: ModelFeatures): MeshPreset {
  const { complexity, nodeCount, analysisType, hasContact, boundingBox } = features

  // 基础推荐
  let elementSize: number
  let elementOrder: number
  let meshType: string
  let reason: string

  switch (complexity) {
    case 'simple':
      elementSize = Math.max(
        boundingBox.x,
        boundingBox.y,
        boundingBox.z
      ) / 10
      elementOrder = 1
      meshType = 'hex'
      reason = `模型包含 ${nodeCount.toLocaleString()} 个节点，结构简单，建议使用线性六面体单元以快速获得结果`
      break

    case 'medium':
      elementSize = Math.max(
        boundingBox.x,
        boundingBox.y,
        boundingBox.z
      ) / 20
      elementOrder = 2
      meshType = 'tet'
      reason = `模型包含 ${nodeCount.toLocaleString()} 个节点，中等复杂度，建议使用二次四面体单元以平衡精度和效率`
      break

    case 'complex':
      elementSize = Math.max(
        boundingBox.x,
        boundingBox.y,
        boundingBox.z
      ) / 40
      elementOrder = 2
      meshType = 'mixed'
      reason = `模型包含 ${nodeCount.toLocaleString()} 个节点，复杂几何体，建议使用混合网格（六面体+四面体）并采用二次单元以提高精度`
      break
  }

  // 接触分析修正：接触区域需要更细的网格
  if (hasContact) {
    elementSize *= 0.5
    reason += '。检测到接触分析，建议对接触区域进行网格加密'
  }

  // 模态分析修正：使用一致质量矩阵
  if (analysisType === 'modal' || analysisType === 'frequency') {
    if (elementOrder === 1) {
      elementOrder = 2
      reason += '。模态/频率分析建议使用二次单元以获得更精确的频率计算'
    }
  }

  // CFD 分析修正
  if (analysisType === 'cfd') {
    meshType = 'hex'
    elementOrder = 1
    reason = `CFD 流场分析建议使用结构化六面体网格，边界层区域需进一步加密`
  }

  // 各向异性材料修正
  if (features.materialType === 'anisotropic') {
    if (elementOrder < 2) {
      elementOrder = 2
      reason += '。各向异性材料建议使用高阶单元以准确捕捉方向相关的力学行为'
    }
  }

  return {
    elementSize: Math.round(elementSize * 1000) / 1000,
    elementOrder,
    meshType,
    reason,
  }
}

/**
 * 推荐求解器参数
 */
function recommendSolver(features: ModelFeatures): SolverPreset {
  const { complexity, analysisType, hasContact, nodeCount } = features

  let solverType: string
  let maxIterations: number
  let convergenceTolerance: number
  let reason: string

  // 根据分析类型推荐
  switch (analysisType) {
    case 'modal':
      solverType = 'Lanczos'
      maxIterations = 500
      convergenceTolerance = 1e-6
      reason = '模态分析推荐使用 Lanczos 特征值求解器，收敛速度快，适合提取多阶模态'
      break

    case 'thermal':
      solverType = 'iterative'
      maxIterations = 1000
      convergenceTolerance = 1e-4
      reason = '热传导分析推荐使用迭代求解器，内存占用低，适合大型热传导问题'
      break

    case 'frequency':
      solverType = 'Lanczos'
      maxIterations = 1000
      convergenceTolerance = 1e-8
      reason = '频率响应分析推荐使用 Lanczos 求解器，高精度收敛以准确捕捉共振频率'
      break

    case 'buckling':
      solverType = 'direct'
      maxIterations = 100
      convergenceTolerance = 1e-6
      reason = '屈曲分析推荐使用直接求解器以确保特征值计算的稳定性'
      break

    case 'cfd':
      solverType = 'coupled'
      maxIterations = 5000
      convergenceTolerance = 1e-5
      reason = 'CFD 分析推荐使用耦合求解器，速度-压力耦合求解以获得稳定收敛'
      break

    default: // static
      if (complexity === 'simple') {
        solverType = 'direct'
        maxIterations = 100
        convergenceTolerance = 1e-6
        reason = '简单模型推荐使用直接求解器，一次性求解，精度最高'
      } else if (complexity === 'medium') {
        solverType = 'iterative'
        maxIterations = 500
        convergenceTolerance = 1e-5
        reason = '中等规模模型推荐使用迭代求解器（PCG），内存效率高，适合 ${nodeCount.toLocaleString()} 节点量级'
      } else {
        solverType = 'iterative'
        maxIterations = 1000
        convergenceTolerance = 1e-6
        reason = '大规模模型推荐使用迭代求解器配合预条件子，可高效处理 ${nodeCount.toLocaleString()} 节点的自由度'
      }
      break
  }

  // 接触分析修正
  if (hasContact) {
    if (analysisType === 'static') {
      solverType = 'direct'
      maxIterations = Math.max(maxIterations, 200)
      convergenceTolerance = Math.min(convergenceTolerance, 1e-6)
      reason += '。接触分析涉及非线性迭代，建议使用直接求解器以保证收敛稳定性'
    }
  }

  return {
    solverType,
    maxIterations,
    convergenceTolerance,
    reason,
  }
}

/**
 * 计算推荐置信度
 */
function calculateConfidence(
  features: ModelFeatures,
  _mesh: MeshPreset,
  _solver: SolverPreset
): number {
  let confidence = 0.7 // 基础置信度

  // 节点数越多，推荐越有依据
  if (features.nodeCount > 100) confidence += 0.1
  if (features.nodeCount > 1000) confidence += 0.05

  // 明确的分析类型提高置信度
  if (['static', 'modal', 'thermal'].includes(features.analysisType)) {
    confidence += 0.1
  }

  // 接触分析降低置信度（情况更复杂）
  if (features.hasContact) confidence -= 0.1

  // 各向异性材料降低置信度
  if (features.materialType === 'anisotropic') confidence -= 0.05

  return Math.min(1, Math.max(0, confidence))
}

/**
 * 获取置信度标签
 */
export function getConfidenceLabel(confidence: number): {
  label: string
  color: string
  bgColor: string
} {
  if (confidence >= 0.8) {
    return {
      label: '高',
      color: 'text-green-700 dark:text-green-400',
      bgColor: 'bg-green-100 dark:bg-green-900/30',
    }
  }
  if (confidence >= 0.5) {
    return {
      label: '中',
      color: 'text-yellow-700 dark:text-yellow-400',
      bgColor: 'bg-yellow-100 dark:bg-yellow-900/30',
    }
  }
  return {
    label: '低',
    color: 'text-red-700 dark:text-red-400',
    bgColor: 'bg-red-100 dark:bg-red-900/30',
  }
}
