/**
 * Data Transfer API
 * V2.0-002: 尺度间数据传递引擎
 */

import { invoke } from '@tauri-apps/api/core'
import type { WorkflowScaleLevel } from './workflowEditor'

// ============ 类型定义 ============

/** 数据传递方向 */
export type DataTransferDirection =
  | 'dft_to_md'
  | 'md_to_phase_field'
  | 'phase_field_to_fe'
  | 'md_to_fe'
  | 'dft_to_phase_field'
  | 'dft_to_fe'

/** 字段模式定义 */
export interface FieldSchema {
  name: string
  type: 'scalar' | 'vector' | 'tensor'
  dimensions: number[]
  unit: string
  description: string
}

/** 尺度输出模式 */
export interface ScaleOutputSchema {
  scale: WorkflowScaleLevel
  version: string
  fields: FieldSchema[]
  required_fields: string[]
}

/** 数据传递规则 */
export interface DataTransferRule {
  source_scale: WorkflowScaleLevel
  target_scale: WorkflowScaleLevel
  direction: DataTransferDirection
  field_mappings: Array<{
    source_field: string
    target_field: string
    transformation: string
    unit_conversion: { factor: number; offset: number }
  }>
  validation: {
    check_dimensions: boolean
    check_units: boolean
    tolerance: number
  }
}

/** 数据传递结果 */
export interface DataTransferResult {
  success: boolean
  direction: DataTransferDirection
  transferred_fields: Array<{
    field: string
    source_value: unknown
    target_value: unknown
    status: 'success' | 'warning' | 'error'
    message: string
  }>
  validation_errors: string[]
  warnings: string[]
  transfer_time_ms: number
}

/** 数据传递配置 */
export interface DataTransferConfig {
  source_node_id: string
  target_node_id: string
  direction: DataTransferDirection
  rules: DataTransferRule[]
  source_data: Record<string, unknown>
}

// ============ API 函数 ============

/**
 * 获取指定尺度的输出数据模式
 * @param scale - 尺度级别
 * @returns 尺度输出模式定义
 */
export async function getScaleOutputSchema(
  scale: WorkflowScaleLevel
): Promise<ScaleOutputSchema> {
  return invoke<ScaleOutputSchema>('get_scale_output_schema', { scale })
}

/**
 * 验证数据传递配置是否合法
 * @param config - 数据传递配置
 * @returns 验证结果，包含是否有效、错误列表和警告列表
 */
export async function validateDataTransfer(
  config: DataTransferConfig
): Promise<{ is_valid: boolean; errors: string[]; warnings: string[] }> {
  return invoke<{ is_valid: boolean; errors: string[]; warnings: string[] }>(
    'validate_data_transfer',
    { config }
  )
}

/**
 * 执行尺度间数据传递
 * @param config - 数据传递配置
 * @returns 数据传递结果
 */
export async function executeDataTransfer(
  config: DataTransferConfig
): Promise<DataTransferResult> {
  return invoke<DataTransferResult>('execute_data_transfer', { config })
}

/**
 * 获取指定方向的数据传递规则
 * @param direction - 数据传递方向
 * @returns 数据传递规则列表
 */
export async function getTransferRules(
  direction: DataTransferDirection
): Promise<DataTransferRule[]> {
  return invoke<DataTransferRule[]>('get_transfer_rules', { direction })
}

/**
 * 更新数据传递规则
 * @param rule - 新的数据传递规则
 */
export async function updateTransferRule(
  rule: DataTransferRule
): Promise<void> {
  return invoke<void>('update_transfer_rule', { rule })
}

/**
 * 获取两个节点之间的数据传递历史记录
 * @param sourceNodeId - 源节点 ID
 * @param targetNodeId - 目标节点 ID
 * @returns 数据传递结果列表
 */
export async function getTransferHistory(
  sourceNodeId: string,
  targetNodeId: string
): Promise<DataTransferResult[]> {
  return invoke<DataTransferResult[]>('get_transfer_history', {
    sourceNodeId,
    targetNodeId,
  })
}
