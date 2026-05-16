/**
 * V3.9 Agent 知识召回服务
 * 基于 Rust 后端 knowledge_memory 命令
 * 集成到 Agent 执行上下文中
 */

import type { KnowledgeRecallRequest, KnowledgeRecallResult } from './types'

/** 调用 Rust 后端知识召回 */
export async function recallKnowledge(request: KnowledgeRecallRequest): Promise<KnowledgeRecallResult | null> {
  try {
    const tauri = (window as unknown as { __TAURI__?: { core: { invoke: (cmd: string, args: { request: KnowledgeRecallRequest }) => Promise<unknown> } } }).__TAURI__
    if (!tauri) return null
    const result = await tauri.core.invoke(
      'recall_knowledge',
      { request }
    )
    return result as KnowledgeRecallResult
  } catch (e) {
    console.warn('Knowledge recall failed:', e)
    return null
  }
}

/** 获取材料知识列表 */
export async function getMaterialKnowledgeList(materialId?: string, name?: string, limit = 5) {
  try {
    const tauri = (window as unknown as { __TAURI__?: { core: { invoke: (cmd: string, args: object) => Promise<unknown[]> } } }).__TAURI__
    if (!tauri) return []
    return await tauri.core.invoke(
      'get_material_knowledge_list',
      { material_id: materialId, name, limit }
    ) as unknown[]
  } catch (e) {
    console.warn('get_material_knowledge_list failed:', e)
    return []
  }
}

/** 获取设计规范列表 */
export async function getDesignStandardsList(name?: string, source?: string, limit = 5) {
  try {
    const tauri = (window as unknown as { __TAURI__?: { core: { invoke: (cmd: string, args: object) => Promise<unknown[]> } } }).__TAURI__
    if (!tauri) return []
    return await tauri.core.invoke(
      'get_design_standards_list',
      { name, source, limit }
    ) as unknown[]
  } catch (e) {
    console.warn('get_design_standards_list failed:', e)
    return []
  }
}

/** 获取失效模式列表 */
export async function getFailureModesList(modeType?: string, material?: string, limit = 5) {
  try {
    const tauri = (window as unknown as { __TAURI__?: { core: { invoke: (cmd: string, args: object) => Promise<unknown[]> } } }).__TAURI__
    if (!tauri) return []
    return await tauri.core.invoke(
      'get_failure_modes_list',
      { mode_type: modeType, material, limit }
    ) as unknown[]
  } catch (e) {
    console.warn('get_failure_modes_list failed:', e)
    return []
  }
}

/** 获取用户画像 */
export async function getUserProfile(userId: string) {
  try {
    const tauri = (window as unknown as { __TAURI__?: { core: { invoke: (cmd: string, args: { user_id: string }) => Promise<unknown> } } }).__TAURI__
    if (!tauri) return null
    return await tauri.core.invoke(
      'get_user_profile',
      { user_id: userId }
    )
  } catch (e) {
    console.warn('get_user_profile failed:', e)
    return null
  }
}

/** 保存用户画像 */
export async function saveUserProfile(request: {
  user_id: string
  nickname?: string
  email?: string
  company?: string
  position?: string
  preferences_json: string
  project_history_json: string
  expertise_tags_json?: string
}) {
  try {
    const tauri = (window as unknown as { __TAURI__?: { core: { invoke: (cmd: string, args: { request: unknown }) => Promise<unknown> } } }).__TAURI__
    if (!tauri) return null
    return await tauri.core.invoke(
      'save_user_profile',
      { request }
    )
  } catch (e) {
    console.warn('save_user_profile failed:', e)
    return null
  }
}

/** 根据材料名称召回相关知识 */
export async function recallByMaterial(materialName: string): Promise<{
  material: unknown | null
  standards: unknown[]
  failureModes: unknown[]
} | null> {
  try {
    const [materials, standards, failureModes] = await Promise.all([
      getMaterialKnowledgeList(materialName, undefined, 1),
      getDesignStandardsList(undefined, undefined, 3),
      getFailureModesList(undefined, materialName, 3),
    ])
    return {
      material: materials?.[0] || null,
      standards: standards || [],
      failureModes: failureModes || [],
    }
  } catch (e) {
    console.warn('recallByMaterial failed:', e)
    return null
  }
}

/** 根据仿真类型召回相关知识 */
export async function recallBySimulationType(simType: string): Promise<{
  materials: unknown[]
  standards: unknown[]
  failureModes: unknown[]
} | null> {
  const keywords = getKeywordsForSimulationType(simType)
  if (keywords.length === 0) return null

  try {
    const result = await recallKnowledge({
      keywords,
      context_type: 'all',
      simulation_type: simType,
      limit: 5,
    })
    if (!result) return null

    const materials = result.materials ? JSON.parse(result.materials as unknown as string) : []
    const standards = result.standards ? JSON.parse(result.standards as unknown as string) : []
    const failureModes = result.failure_modes ? JSON.parse(result.failure_modes as unknown as string) : []

    return { materials, standards, failureModes }
  } catch (e) {
    console.warn('recallBySimulationType failed:', e)
    return null
  }
}

/** 从用户输入中提取关键词进行知识召回 */
export async function recallFromUserInput(userInput: string, intent: string): Promise<{
  material: unknown | null
  standards: unknown[]
  failureModes: unknown[]
  knowledgeHint: string
} | null> {
  const keywords = extractKeywordsFromInput(userInput)
  if (keywords.length === 0) return null

  try {
    const result = await recallKnowledge({
      keywords,
      context_type: 'all',
      simulation_type: intent,
      limit: 10,
    })

    if (!result || result.confidence < 0.1) return null

    const materials = result.materials ? JSON.parse(result.materials as unknown as string) : []
    const standards = result.standards ? JSON.parse(result.standards as unknown as string) : []
    const failureModes = result.failure_modes ? JSON.parse(result.failure_modes as unknown as string) : []

    // 构建知识提示文本
    let knowledgeHint = ''
    if (materials.length > 0) {
      const mat = materials[0] as { name?: string; elastic_modulus?: number; yield_strength?: number; common_errors?: string }
      knowledgeHint += `[材料知识] ${mat.name}: E=${mat.elastic_modulus} GPa, σ_s=${mat.yield_strength} MPa`
      if (mat.common_errors) {
        const errors = typeof mat.common_errors === 'string' ? JSON.parse(mat.common_errors) : mat.common_errors
        if (Array.isArray(errors) && errors.length > 0) {
          knowledgeHint += ` | 常见错误: ${errors.slice(0, 2).join(', ')}`
        }
      }
    }
    if (failureModes.length > 0) {
      const fm = failureModes[0] as { mode_type?: string; description?: string; mechanisms?: unknown[] }
      const mech = fm.mechanisms?.[0] as { name?: string; mitigationStrategies?: string[] } | undefined
      knowledgeHint += `\n[失效模式] ${fm.mode_type}: ${fm.description}`
      if (mech?.mitigationStrategies?.[0]) {
        knowledgeHint += ` | 缓解: ${mech.mitigationStrategies[0]}`
      }
    }

    return {
      material: materials[0] || null,
      standards,
      failureModes,
      knowledgeHint,
    }
  } catch (e) {
    console.warn('recallFromUserInput failed:', e)
    return null
  }
}

/** 从用户输入中提取关键词 */
function extractKeywordsFromInput(input: string): string[] {
  const keywords: string[] = []

  // 材料关键词
  const materialPatterns = [
    /\b(TC4|Ti-6Al-4V|AA\s*7075|Q235|Q345|SS\s*316L|Inconel\s*718|A36|S235|S355)\b/gi,
    /\b(钛合金|铝合金|结构钢|不锈钢|高温合金)\b/gi,
  ]
  for (const pattern of materialPatterns) {
    const matches = input.match(pattern)
    if (matches) keywords.push(...matches)
  }

  // 仿真类型关键词
  const simPatterns = [
    /\b(静态|静力|动态|模态|疲劳|热|耦合|屈曲|拓扑优化|拓扑)\b/gi,
    /\b(fatigue|thermal|modal|buckling|static|dynamic|topology)\b/gi,
    /\b(HCF|LCF|TMF|creep|overload|FOD)\b/gi,
  ]
  for (const pattern of simPatterns) {
    const matches = input.match(pattern)
    if (matches) keywords.push(...matches)
  }

  // 失效模式关键词
  const failurePatterns = [
    /\b(高周疲劳|低周疲劳|热疲劳|蠕变|过载|腐蚀|磨损|撞击)\b/gi,
    /\b(fatigue|creep|fracture|corrosion|wear|impact)\b/gi,
  ]
  for (const pattern of failurePatterns) {
    const matches = input.match(pattern)
    if (matches) keywords.push(...matches)
  }

  // 去重
  return [...new Set(keywords)]
}

/** 获取仿真类型对应的关键词 */
function getKeywordsForSimulationType(simType: string): string[] {
  const map: Record<string, string[]> = {
    'simulation.static': ['静态', '静力', 'static'],
    'simulation.dynamic': ['动态', 'dynamic'],
    'simulation.modal': ['模态', 'modal', '振型', '固有频率'],
    'simulation.fatigue': ['疲劳', 'fatigue', 'HCF', 'LCF', 'S-N'],
    'simulation.thermal': ['热', 'thermal', '温度', '热应力'],
    'simulation.buckling': ['屈曲', 'buckling', '失稳'],
    'optimization': ['优化', 'optimization', '拓扑', 'size', 'shape'],
    'analysis': ['分析', 'analysis', '应力', '应变', 'von Mises'],
  }
  return map[simType] || [simType]
}