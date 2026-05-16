/**
 * useResultProvenance.ts — V4.1-006 仿真结果溯源系统
 * 追踪结果来源：谁、何时、用什么参数、运行了什么求解器
 */
import { ref, computed } from 'vue'

export type ProvenanceAction = 'create' | 'modify' | 'run' | 'export' | 'import' | 'delete' | 'approve' | 'reject'
export type ProvenanceEntity = 'simulation' | 'geometry' | 'material' | 'mesh' | 'boundary_condition' | 'solver_config' | 'result'

export interface ProvenanceActor {
  id: string
  name: string
  email?: string
  role: string
  organization?: string
}

export interface ProvenanceParameter {
  name: string
  value: number | string | boolean
  unit?: string
  previousValue?: number | string | boolean
}

export interface ProvenanceRecord {
  id: string
  timestamp: string
  action: ProvenanceAction
  entity: ProvenanceEntity
  entityId: string
  entityName: string
  actor: ProvenanceActor
  parameters?: ProvenanceParameter[]
  description: string
  sourceFile?: string
  checksum?: string
  parentRecordId?: string
  tags: string[]
  verificationStatus: 'unverified' | 'verified' | 'questioned'
  verifier?: ProvenanceActor
  verificationTime?: string
  verificationNotes?: string
}

export interface ResultLineage {
  resultId: string
  resultName: string
  createdAt: string
  creator: ProvenanceActor
  sourceSimulationId: string
  parameterHistory: ProvenanceParameter[]
  executionChain: ProvenanceRecord[]
  verificationChain: ProvenanceRecord[]
  derivedResults: string[]
  confidenceScore: number
  reproducibilityIndex: number
}

export interface ReproducibilityReport {
  resultId: string
  score: number
  missingInfo: string[]
  warnings: string[]
  canReproduce: boolean
  requiredSteps: string[]
}

export function useResultProvenance() {
  const records = ref<ProvenanceRecord[]>([])
  const currentActor = ref<ProvenanceActor>({
    id: 'user-001',
    name: '当前用户',
    role: 'engineer'
  })

  const allLineages = computed((): ResultLineage[] => {
    const resultRecords = records.value.filter(r => r.entity === 'result' && r.action === 'create')
    return resultRecords.map(r => buildLineage(r.entityId))
  })

  /**
   * 记录操作
   */
  function logAction(
    action: ProvenanceAction,
    entity: ProvenanceEntity,
    entityId: string,
    entityName: string,
    options?: {
      parameters?: ProvenanceParameter[]
      description?: string
      sourceFile?: string
      checksum?: string
      parentRecordId?: string
      tags?: string[]
    }
  ): ProvenanceRecord {
    const record: ProvenanceRecord = {
      id: `prov-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`,
      timestamp: new Date().toISOString(),
      action,
      entity,
      entityId,
      entityName,
      actor: { ...currentActor.value },
      parameters: options?.parameters,
      description: options?.description || `${action} ${entity}: ${entityName}`,
      sourceFile: options?.sourceFile,
      checksum: options?.checksum,
      parentRecordId: options?.parentRecordId,
      tags: options?.tags || [],
      verificationStatus: 'unverified'
    }
    records.value.push(record)
    return record
  }

  /**
   * 记录仿真运行
   */
  function logSimulationRun(
    simulationId: string,
    simulationName: string,
    config: {
      solverType: string
      solverVersion: string
      meshElements: number
      meshNodes: number
      materialName: string
      boundaryConditions: string[]
      parameters: ProvenanceParameter[]
    }
  ): ProvenanceRecord {
    return logAction('run', 'simulation', simulationId, simulationName, {
      parameters: [
        ...config.parameters,
        { name: 'solver_type', value: config.solverType },
        { name: 'solver_version', value: config.solverVersion },
        { name: 'mesh_elements', value: config.meshElements },
        { name: 'mesh_nodes', value: config.meshNodes },
        { name: 'material', value: config.materialName },
        { name: 'boundary_conditions', value: config.boundaryConditions.join(', ') }
      ],
      description: `运行仿真: ${config.solverType} v${config.solverVersion}, ${config.meshElements} 单元`,
      tags: ['simulation', 'execution']
    })
  }

  /**
   * 记录结果导出
   */
  function logResultExport(
    resultId: string,
    resultName: string,
    format: string,
    destination: string
  ): ProvenanceRecord {
    return logAction('export', 'result', resultId, resultName, {
      parameters: [
        { name: 'format', value: format },
        { name: 'destination', value: destination }
      ],
      description: `导出结果到 ${destination} (${format})`,
      tags: ['export', format]
    })
  }

  /**
   * 记录结果验证
   */
  function verifyResult(
    resultId: string,
    status: 'verified' | 'questioned',
    notes?: string
  ): void {
    const record = records.value.find(r => r.entityId === resultId && r.entity === 'result')
    if (record) {
      record.verificationStatus = status
      record.verifier = { ...currentActor.value }
      record.verificationTime = new Date().toISOString()
      record.verificationNotes = notes
    }
  }

  /**
   * 构建结果谱系
   */
  function buildLineage(resultId: string): ResultLineage {
    const resultRecord = records.value.find(r => r.entityId === resultId && r.entity === 'result')
    if (!resultRecord) {
      return {
        resultId,
        resultName: 'Unknown',
        createdAt: '',
        creator: currentActor.value,
        sourceSimulationId: '',
        parameterHistory: [],
        executionChain: [],
        verificationChain: [],
        derivedResults: [],
        confidenceScore: 0,
        reproducibilityIndex: 0
      }
    }

    // 追踪执行链：从结果回溯到创建
    const executionChain: ProvenanceRecord[] = []
    let current: ProvenanceRecord | undefined = resultRecord
    const visited = new Set<string>()

    while (current && !visited.has(current.id)) {
      visited.add(current.id)
      executionChain.unshift(current)
      if (current.parentRecordId) {
        current = records.value.find(r => r.id === current!.parentRecordId)
      } else {
        current = records.value.find(r =>
          r.entityId === resultRecord.entityId &&
          r.id !== current!.id &&
          new Date(r.timestamp) < new Date(current!.timestamp)
        )
      }
    }

    // 收集所有参数变更
    const parameterHistory: ProvenanceParameter[] = []
    for (const record of executionChain) {
      if (record.parameters) {
        parameterHistory.push(...record.parameters)
      }
    }

    // 验证链
    const verificationChain = executionChain.filter(r => r.verificationStatus !== 'unverified')

    // 派生结果
    const derivedResults = records.value
      .filter(r => r.parentRecordId === resultRecord.id && r.entity === 'result')
      .map(r => r.entityId)

    // 计算置信度和可复现性
    const confidenceScore = calculateConfidenceScore(executionChain)
    const reproducibilityIndex = calculateReproducibility(executionChain, parameterHistory)

    return {
      resultId,
      resultName: resultRecord.entityName,
      createdAt: resultRecord.timestamp,
      creator: resultRecord.actor,
      sourceSimulationId: resultRecord.parentRecordId || '',
      parameterHistory,
      executionChain,
      verificationChain,
      derivedResults,
      confidenceScore,
      reproducibilityIndex
    }
  }

  /**
   * 生成可复现性报告
   */
  function generateReproducibilityReport(resultId: string): ReproducibilityReport {
    const lineage = buildLineage(resultId)
    const missingInfo: string[] = []
    const warnings: string[] = []
    const requiredSteps: string[] = []

    // 检查必要信息
    if (lineage.parameterHistory.length === 0) {
      missingInfo.push('缺少参数历史记录')
    }

    const hasSolverInfo = lineage.executionChain.some(r =>
      r.parameters?.some(p => p.name === 'solver_type')
    )
    if (!hasSolverInfo) {
      missingInfo.push('缺少求解器信息')
    }

    const hasMeshInfo = lineage.executionChain.some(r =>
      r.parameters?.some(p => p.name === 'mesh_elements')
    )
    if (!hasMeshInfo) {
      warnings.push('网格信息不完整')
    }

    // 构建复现步骤
    requiredSteps.push(`1. 加载仿真配置: ${lineage.resultName}`)
    for (const param of lineage.parameterHistory) {
      if (param.name !== 'solver_type' && param.name !== 'solver_version') {
        requiredSteps.push(`   - 设置 ${param.name} = ${param.value}${param.unit ? ' ' + param.unit : ''}`)
      }
    }
    requiredSteps.push('2. 运行求解器')
    requiredSteps.push('3. 对比结果与原始记录')

    const score = Math.max(0, 100 - missingInfo.length * 25 - warnings.length * 10)

    return {
      resultId,
      score,
      missingInfo,
      warnings,
      canReproduce: score >= 60,
      requiredSteps
    }
  }

  /**
   * 获取实体的完整历史
   */
  function getEntityHistory(entityId: string, entity?: ProvenanceEntity): ProvenanceRecord[] {
    return records.value
      .filter(r => r.entityId === entityId && (!entity || r.entity === entity))
      .sort((a, b) => new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime())
  }

  /**
   * 搜索溯源记录
   */
  function searchRecords(query: string): ProvenanceRecord[] {
    const q = query.toLowerCase()
    return records.value.filter(r =>
      r.entityName.toLowerCase().includes(q) ||
      r.description.toLowerCase().includes(q) ||
      r.actor.name.toLowerCase().includes(q) ||
      r.tags.some(t => t.toLowerCase().includes(q))
    )
  }

  /**
   * 导出溯源链为 JSON
   */
  function exportProvenanceChain(resultId: string): string {
    const lineage = buildLineage(resultId)
    return JSON.stringify(lineage, null, 2)
  }

  /**
   * 计算置信度分数
   */
  function calculateConfidenceScore(chain: ProvenanceRecord[]): number {
    let score = 0.5

    // 有验证记录加分
    const verified = chain.filter(r => r.verificationStatus === 'verified').length
    score += verified * 0.1

    // 有 checksum 加分
    const hasChecksum = chain.some(r => r.checksum)
    if (hasChecksum) score += 0.1

    // 链越长越可信（更多上下文）
    score += Math.min(0.2, chain.length * 0.02)

    // 有参数记录加分
    const hasParams = chain.some(r => r.parameters && r.parameters.length > 0)
    if (hasParams) score += 0.1

    return Math.min(1, score)
  }

  /**
   * 计算可复现性指数
   */
  function calculateReproducibility(chain: ProvenanceRecord[], params: ProvenanceParameter[]): number {
    let score = 0.3

    // 参数完整度
    const requiredParams = ['solver_type', 'mesh_elements', 'material']
    const hasRequired = requiredParams.every(name =>
      params.some(p => p.name === name)
    )
    if (hasRequired) score += 0.3

    // 边界条件记录
    const hasBC = params.some(p => p.name === 'boundary_conditions')
    if (hasBC) score += 0.15

    // 有源文件记录
    const hasSource = chain.some(r => r.sourceFile)
    if (hasSource) score += 0.15

    // 有校验和
    const hasChecksum = chain.some(r => r.checksum)
    if (hasChecksum) score += 0.1

    return Math.min(1, score)
  }

  return {
    records,
    currentActor,
    allLineages,
    logAction,
    logSimulationRun,
    logResultExport,
    verifyResult,
    buildLineage,
    generateReproducibilityReport,
    getEntityHistory,
    searchRecords,
    exportProvenanceChain
  }
}
