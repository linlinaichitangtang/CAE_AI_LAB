/**
 * Simulation Data Asset Management API
 * V1.4-010: 仿真数据资产管理 API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type AssetStatus = 'active' | 'archived' | 'locked'

export type TransformationType = 'mesh' | 'solve' | 'postprocess' | 'export'

export type PermissionLevel = 'read' | 'write' | 'admin'

export interface SimulationAsset {
  id: string
  name: string
  project_id: string
  description: string
  version: string
  tags: string[]
  created_at: string
  modified_at: string
  file_size_mb: number
  file_type: string
  author: string
  status: AssetStatus
  checksum_md5: string
  parent_id: string | null
}

export interface DataLineage {
  asset_id: string
  upstream_ids: string[]
  downstream_ids: string[]
  transformation_type: TransformationType
  created_at: string
}

export interface AssetPermission {
  asset_id: string
  user_id: string
  permission: PermissionLevel
  granted_at: string
}

export interface SearchFilter {
  keyword?: string
  tags?: string[]
  author?: string
  date_range?: { from: string; to: string }
  file_type?: string
  project_id?: string
}

export interface DataAssetConfig {
  asset_name: string
  project_id: string
  description: string
  tags: string[]
  parent_id?: string
  auto_version: boolean
}

// ============ API 函数 ============

/**
 * 创建仿真数据资产
 */
export async function createAsset(config: DataAssetConfig): Promise<SimulationAsset> {
  return invoke<SimulationAsset>('create_asset', { config })
}

/**
 * 获取单个仿真数据资产
 */
export async function getAsset(assetId: string): Promise<SimulationAsset> {
  return invoke<SimulationAsset>('get_asset', { assetId })
}

/**
 * 搜索仿真数据资产
 */
export async function searchAssets(filter: SearchFilter): Promise<SimulationAsset[]> {
  return invoke<SimulationAsset[]>('search_assets', { filter })
}

/**
 * 更新仿真数据资产
 */
export async function updateAsset(
  assetId: string,
  updates: Partial<Omit<SimulationAsset, 'id' | 'created_at'>>
): Promise<SimulationAsset> {
  return invoke<SimulationAsset>('update_asset', { assetId, updates })
}

/**
 * 删除仿真数据资产
 */
export async function deleteAsset(assetId: string): Promise<boolean> {
  return invoke<boolean>('delete_asset', { assetId })
}

/**
 * 获取资产版本历史
 */
export async function getVersionHistory(assetId: string): Promise<SimulationAsset[]> {
  return invoke<SimulationAsset[]>('get_version_history', { assetId })
}

/**
 * 获取资产数据血缘
 */
export async function getAssetLineage(assetId: string): Promise<DataLineage> {
  return invoke<DataLineage>('get_asset_lineage', { assetId })
}

/**
 * 递归获取完整数据血缘
 */
export async function getFullLineage(assetId: string): Promise<DataLineage[]> {
  return invoke<DataLineage[]>('get_full_lineage', { assetId })
}

/**
 * 导出溯源报告
 */
export async function exportLineageReport(assetId: string): Promise<string> {
  return invoke<string>('export_lineage_report', { assetId })
}

/**
 * 设置资产权限
 */
export async function setAssetPermission(
  assetId: string,
  userId: string,
  permission: PermissionLevel
): Promise<AssetPermission> {
  return invoke<AssetPermission>('set_asset_permission', { assetId, userId, permission })
}

/**
 * 获取资产权限列表
 */
export async function getAssetPermissions(assetId: string): Promise<AssetPermission[]> {
  return invoke<AssetPermission[]>('get_asset_permissions', { assetId })
}
