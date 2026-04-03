/**
 * Workflow Editor API
 * V2.0-001: 工作流图编辑器（节点+连线）
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

/** 工作流尺度级别 */
export type WorkflowScaleLevel = 'dft' | 'md' | 'phase_field' | 'fe'

/** 节点在画布上的位置 */
export interface WfNodePosition {
  x: number
  y: number
}

/** 工作流节点 */
export interface WfNode {
  id: string
  scale: WorkflowScaleLevel
  label: string
  position: WfNodePosition
  config: Record<string, unknown>
  status: 'idle' | 'configured' | 'running' | 'completed' | 'failed'
}

/** 工作流连线（边） */
export interface WfEdge {
  id: string
  source_node_id: string
  target_node_id: string
  data_mapping: Array<{ source_field: string; target_field: string }>
  label: string
}

/** 条件分支 */
export interface WfConditionalBranch {
  edge_id: string
  condition: { field: string; operator: string; value: number }
  target_node_id: string
}

/** 工作流图 */
export interface WfGraph {
  nodes: WfNode[]
  edges: WfEdge[]
  conditional_branches: WfConditionalBranch[]
}

/** 工作流图持久化结构 */
export interface WfGraphSave {
  graph_id: string
  name: string
  description: string
  graph: WfGraph
  created_at: string
  updated_at: string
}

// ============ API 函数 ============

/**
 * 创建新的工作流图
 * @param name - 工作流名称
 * @param description - 工作流描述
 * @returns 保存后的工作流图数据
 */
export async function createGraph(
  name: string,
  description: string
): Promise<WfGraphSave> {
  return invoke<WfGraphSave>('create_graph', { name, description })
}

/**
 * 加载指定的工作流图
 * @param graphId - 工作流图 ID
 * @returns 工作流图数据
 */
export async function loadGraph(
  graphId: string
): Promise<WfGraphSave> {
  return invoke<WfGraphSave>('load_graph', { graphId })
}

/**
 * 保存工作流图
 * @param graph - 工作流图数据
 * @returns 保存结果
 */
export async function saveGraph(
  graph: WfGraphSave
): Promise<{ success: boolean }> {
  return invoke<{ success: boolean }>('save_graph', { graph })
}

/**
 * 向工作流图中添加节点
 * @param graphId - 工作流图 ID
 * @param scale - 尺度级别
 * @param label - 节点标签
 * @param position - 节点位置
 * @returns 新创建的节点
 */
export async function addNode(
  graphId: string,
  scale: WorkflowScaleLevel,
  label: string,
  position: WfNodePosition
): Promise<WfNode> {
  return invoke<WfNode>('add_node', { graphId, scale, label, position })
}

/**
 * 从工作流图中移除节点
 * @param graphId - 工作流图 ID
 * @param nodeId - 节点 ID
 */
export async function removeNode(
  graphId: string,
  nodeId: string
): Promise<void> {
  return invoke<void>('remove_node', { graphId, nodeId })
}

/**
 * 更新节点配置
 * @param graphId - 工作流图 ID
 * @param nodeId - 节点 ID
 * @param config - 新的配置数据
 * @returns 更新后的节点
 */
export async function updateNodeConfig(
  graphId: string,
  nodeId: string,
  config: Record<string, unknown>
): Promise<WfNode> {
  return invoke<WfNode>('update_node_config', { graphId, nodeId, config })
}

/**
 * 向工作流图中添加连线
 * @param graphId - 工作流图 ID
 * @param sourceNodeId - 源节点 ID
 * @param targetNodeId - 目标节点 ID
 * @param dataMapping - 数据映射关系
 * @returns 新创建的连线
 */
export async function addEdge(
  graphId: string,
  sourceNodeId: string,
  targetNodeId: string,
  dataMapping: Array<{ source_field: string; target_field: string }>
): Promise<WfEdge> {
  return invoke<WfEdge>('add_edge', { graphId, sourceNodeId, targetNodeId, dataMapping })
}

/**
 * 从工作流图中移除连线
 * @param graphId - 工作流图 ID
 * @param edgeId - 连线 ID
 */
export async function removeEdge(
  graphId: string,
  edgeId: string
): Promise<void> {
  return invoke<void>('remove_edge', { graphId, edgeId })
}

/**
 * 添加条件分支
 * @param graphId - 工作流图 ID
 * @param branch - 条件分支配置
 */
export async function addConditionalBranch(
  graphId: string,
  branch: WfConditionalBranch
): Promise<void> {
  return invoke<void>('add_conditional_branch', { graphId, branch })
}

/**
 * 验证工作流图的完整性
 * @param graphId - 工作流图 ID
 * @returns 验证结果，包含是否有效、错误列表和警告列表
 */
export async function validateGraph(
  graphId: string
): Promise<{ is_valid: boolean; errors: string[]; warnings: string[] }> {
  return invoke<{ is_valid: boolean; errors: string[]; warnings: string[] }>(
    'validate_graph',
    { graphId }
  )
}

/**
 * 列出所有工作流图
 * @returns 工作流图列表
 */
export async function listGraphs(): Promise<WfGraphSave[]> {
  return invoke<WfGraphSave[]>('list_graphs')
}
