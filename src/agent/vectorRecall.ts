/**
 * V3.10 Agent RAG 向量推理服务
 * 基于 Rust 后端 vector_store 命令
 * 集成到 Agent 执行上下文中 — 主动回忆 + 跨会话知识积累
 */

import type { VectorSearchRequest, VectorSearchResult, ActiveRecallRequest, ActiveRecallResult, CrossSessionKnowledgeSummary, EmbeddingRecord, KnowledgeSourceType } from './types'

/** 获取 Tauri 引用 (类型安全) */
function getTauri() {
  return (window as unknown as {
    __TAURI__?: {
      core: {
        invoke: (cmd: string, args?: Record<string, unknown>) => Promise<unknown>
      }
    }
  }).__TAURI__
}

// ============================================================================
// V3.10-001: 向量写入
// ============================================================================

/** 写入嵌入记录 */
export async function writeEmbeddingRecord(request: {
  source_type: KnowledgeSourceType
  project_id?: string
  user_id?: string
  content: string
  tags: string[]
  metadata_json?: string
}): Promise<EmbeddingRecord | null> {
  const tauri = getTauri()
  if (!tauri) return null
  try {
    return await tauri.core.invoke('write_embedding_record', { request }) as EmbeddingRecord
  } catch (e) {
    console.warn('write_embedding_record failed:', e)
    return null
  }
}

/** 批量写入嵌入记录 */
export async function batchWriteEmbeddingRecords(requests: {
  source_type: KnowledgeSourceType
  project_id?: string
  user_id?: string
  content: string
  tags: string[]
  metadata_json?: string
}[]): Promise<number> {
  const tauri = getTauri()
  if (!tauri) return 0
  try {
    return await tauri.core.invoke('batch_write_embeddings', { requests }) as number
  } catch (e) {
    console.warn('batch_write_embeddings failed:', e)
    return 0
  }
}

/** 删除嵌入记录 */
export async function deleteEmbeddingRecord(id: string): Promise<boolean> {
  const tauri = getTauri()
  if (!tauri) return false
  try {
    await tauri.core.invoke('delete_embedding_record', { id })
    return true
  } catch (e) {
    console.warn('delete_embedding_record failed:', e)
    return false
  }
}

// ============================================================================
// V3.10-002: 向量搜索
// ============================================================================

/** 向量搜索 */
export async function searchVectorStore(request: VectorSearchRequest): Promise<VectorSearchResult[]> {
  const tauri = getTauri()
  if (!tauri) return []
  try {
    const result = await tauri.core.invoke('search_vector_store', {
      query: request.query,
      scope: request.scope,
      project_id: request.project_id,
      user_id: request.user_id,
      limit: request.limit ?? 5,
      similarity_threshold: request.similarity_threshold ?? 0.5,
      source_types: request.source_types,
    })
    return (result as VectorSearchResult[]).map(r => ({
      ...r,
      record: {
        ...r.record,
        embedding: [], // 不传输原始向量
      }
    }))
  } catch (e) {
    console.warn('search_vector_store failed:', e)
    return []
  }
}

// ============================================================================
// V3.10-003: 主动回忆
// ============================================================================

/** 主动回忆 — 仿真前自动检索相关历史 */
export async function activeRecall(request: ActiveRecallRequest): Promise<ActiveRecallResult | null> {
  const tauri = getTauri()
  if (!tauri) return null
  try {
    return await tauri.core.invoke('active_recall', {
      simulation_type: request.simulation_type,
      material: request.material,
      project_id: request.project_id,
      user_id: request.user_id,
      limit: request.limit ?? 5,
    }) as ActiveRecallResult
  } catch (e) {
    console.warn('active_recall failed:', e)
    return null
  }
}

/** 跨会话知识摘要 */
export async function getCrossSessionSummary(userId: string): Promise<CrossSessionKnowledgeSummary | null> {
  const tauri = getTauri()
  if (!tauri) return null
  try {
    return await tauri.core.invoke('get_cross_session_summary', { user_id: userId }) as CrossSessionKnowledgeSummary
  } catch (e) {
    console.warn('get_cross_session_summary failed:', e)
    return null
  }
}

// ============================================================================
// V3.10-004: 知识自动积累
// ============================================================================

/** 从仿真结果中自动提取知识并写入向量库 */
export async function autoArchiveToVectorStore(params: {
  simulationType: string
  projectId?: string
  userId: string
  material?: string
  result: {
    maxStress?: number
    maxDisplacement?: number
    convergence?: boolean
    iterations?: number
    analysisType?: string
  }
  notes?: string
}): Promise<boolean> {
  const { simulationType, projectId, userId, material, result, notes } = params

  // 构建知识内容
  const contentParts: string[] = []
  contentParts.push(`仿真类型: ${simulationType}`)
  if (material) contentParts.push(`材料: ${material}`)
  if (result.analysisType) contentParts.push(`分析类型: ${result.analysisType}`)
  if (result.maxStress) contentParts.push(`最大应力: ${result.maxStress.toFixed(2)} MPa`)
  if (result.maxDisplacement) contentParts.push(`最大位移: ${(result.maxDisplacement * 1000).toFixed(3)} mm`)
  if (result.convergence !== undefined) contentParts.push(`收敛: ${result.convergence ? '是' : '否'}`)
  if (result.iterations) contentParts.push(`迭代次数: ${result.iterations}`)
  if (notes) contentParts.push(`备注: ${notes}`)

  const content = contentParts.join(' | ')
  const tags = [simulationType]
  if (material) tags.push(material)
  if (result.analysisType) tags.push(result.analysisType)
  if (result.convergence) tags.push('converged')
  else tags.push('not_converged')

  try {
    await writeEmbeddingRecord({
      source_type: 'simulation_result',
      project_id: projectId,
      user_id: userId,
      content,
      tags,
      metadata_json: JSON.stringify({ result }),
    })
    return true
  } catch (e) {
    console.warn('autoArchiveToVectorStore failed:', e)
    return false
  }
}

/** 从材料数据中提取知识并写入向量库 */
export async function archiveMaterialKnowledge(params: {
  materialId: string
  materialName: string
  userId: string
  properties: {
    elastic_modulus?: number
    yield_strength?: number
    fatigue_params?: string
    thermal_params?: string
    mesh_guidelines?: string
  }
}): Promise<boolean> {
  const { materialId, materialName, userId, properties } = params

  const contentParts: string[] = []
  contentParts.push(`材料: ${materialName}`)
  if (properties.elastic_modulus) contentParts.push(`弹性模量: ${properties.elastic_modulus} GPa`)
  if (properties.yield_strength) contentParts.push(`屈服强度: ${properties.yield_strength} MPa`)
  if (properties.fatigue_params) contentParts.push(`疲劳参数: ${properties.fatigue_params}`)
  if (properties.thermal_params) contentParts.push(`热参数: ${properties.thermal_params}`)
  if (properties.mesh_guidelines) contentParts.push(`网格指南: ${properties.mesh_guidelines}`)

  const content = contentParts.join(' | ')
  const tags = [materialName, 'material', 'material_properties']

  try {
    await writeEmbeddingRecord({
      source_type: 'material_data',
      user_id: userId,
      content,
      tags,
      metadata_json: JSON.stringify({ materialId, properties }),
    })
    return true
  } catch (e) {
    console.warn('archiveMaterialKnowledge failed:', e)
    return false
  }
}

// ============================================================================
// V3.10-005: 主动回忆集成
// ============================================================================

/**
 * 根据仿真类型触发主动回忆
 * 返回格式化的知识提示文本
 */
export async function recallBeforeSimulation(params: {
  simulationType: string
  material?: string
  projectId?: string
  userId: string
}): Promise<string> {
  const { simulationType, material, projectId, userId } = params

  const result = await activeRecall({
    simulation_type: simulationType,
    material,
    project_id: projectId,
    user_id: userId,
    limit: 3,
  })

  if (!result || !result.records || result.records.length === 0) {
    return ''
  }

  // 构建知识提示
  let hint = `[主动回忆 — 基于历史知识]\n`
  hint += `上下文完整性: ${(result.context_completeness * 100).toFixed(0)}%\n\n`

  for (const record of result.records) {
    // 截取内容片段
    const snippet = record.snippet.length > 200
      ? record.snippet.substring(0, 200) + '...'
      : record.snippet
    hint += `• [${record.record.source_type}] ${snippet}\n`
  }

  if (result.suggested_directions && result.suggested_directions.length > 0) {
    hint += `\n建议补充: ${result.suggested_directions.join(', ')}`
  }

  return hint
}