/**
 * Multiscale Physical Quantity Ontology API
 * V1.8-001: 多尺度物理量本体库 API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type UnitSystem = 'SI' | 'atomic' | 'cgs'

export interface DimensionPowers {
  L: number    // 长度
  M: number    // 质量
  T: number    // 时间
  Θ: number    // 温度
  I: number    // 电流
  N: number    // 物质的量
  J: number    // 发光强度
}

export interface UnitDefinition {
  name: string
  symbol: string
  to_si_factor: number
}

export interface PhysicalQuantity {
  id: string
  name_zh: string
  name_en: string
  symbol: string
  dimension: DimensionPowers
  default_unit: string
  units: UnitDefinition[]
}

export interface ConversionRequest {
  quantity_id: string
  value: number
  from_unit: string
  to_unit: string
}

export interface ConversionResult {
  input_value: number
  output_value: number
  from_unit: string
  to_unit: string
  quantity: string
  uncertainty: number
}

export interface OntologySearchResult {
  id: string
  name_zh: string
  name_en: string
  symbol: string
  category: string
}

// ============ API 函数 ============

/**
 * 搜索物理量
 */
export async function searchQuantities(
  keyword: string,
  category?: string
): Promise<OntologySearchResult[]> {
  return invoke<OntologySearchResult[]>('search_quantities', { keyword, category: category || null })
}

/**
 * 单位转换
 */
export async function convertUnit(req: ConversionRequest): Promise<ConversionResult> {
  return invoke<ConversionResult>('convert_unit', { req })
}

/**
 * 批量单位转换
 */
export async function batchConvert(requests: ConversionRequest[]): Promise<ConversionResult[]> {
  return invoke<ConversionResult[]>('batch_convert', { requests })
}

/**
 * 获取物理量定义
 */
export async function getQuantityDefinition(quantityId: string): Promise<PhysicalQuantity> {
  return invoke<PhysicalQuantity>('get_quantity_definition', { quantityId })
}

/**
 * 列出所有物理量分类
 */
export async function listCategories(): Promise<string[]> {
  return invoke<string[]>('list_categories')
}
