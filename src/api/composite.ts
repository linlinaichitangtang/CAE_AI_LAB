/**
 * Composite Laminate Analysis API
 * V1.4-001: 复合材料层合板分析 API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export interface PlyProperty {
  material_name: string
  thickness: number              // mm
  fiber_angle: number            // deg
  E1: number                     // Pa - 纵向弹性模量
  E2: number                     // Pa - 横向弹性模量
  G12: number                    // Pa - 面内剪切模量
  G23: number                    // Pa - 横向剪切模量
  nu12: number                   // 主泊松比
  nu21: number                   // 次泊松比
  Xt: number                     // Pa - X向拉伸强度
  Xc: number                     // Pa - X向压缩强度
  Yt: number                     // Pa - Y向拉伸强度
  Yc: number                     // Pa - Y向压缩强度
  S: number                      // Pa - 剪切强度
  density: number                // kg/m³
}

export interface LaminateConfig {
  plies: PlyProperty[]
  symmetry: 'none' | 'symmetric' | 'anti-symmetric'
  analysis_type: 'classical_lamination' | 'tsai_hill' | 'tsai_wu' | 'first_ply_failure'
  loading: {
    Nx: number                   // N/m - 面内法向力 X
    Ny: number                   // N/m - 面内法向力 Y
    Nxy: number                  // N/m - 面内剪力
    Mx: number                   // N - 弯矩 X
    My: number                   // N - 弯矩 Y
    Mxy: number                  // N - 扭矩
  }
  temperature_delta: number      // degC - 温差
}

export interface ABDMatrix {
  A: number[]                    // [A11, A12, A16, A22, A26, A66]
  B: number[]                    // [B11, B12, B16, B22, B26, B66]
  D: number[]                    // [D11, D12, D16, D22, D26, D66]
}

export interface PlyResult {
  ply_index: number
  top_stress: { xx: number; yy: number; xy: number }
  bottom_stress: { xx: number; yy: number; xy: number }
  midplane_strain: { exx: number; eyy: number; gxy: number }
  failure_index: number
}

export interface FailureIndex {
  ply_index: number
  tsai_hill: number
  tsai_wu: number
  max_strain: number
  max_stress: number
  failed: boolean
}

export interface LaminateResult {
  ABD_matrix: ABDMatrix
  midplane_strains: {
    exx: number
    eyy: number
    gxy: number
    kxx: number
    kyy: number
    kxy: number
  }
  ply_stresses: PlyResult[]
  failure_indices: FailureIndex[]
  first_ply_failure_load: number
  margin_of_safety: number
}

export interface CompositeTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  plies: PlyProperty[]
  typical_application: string
}

// ============ API 函数 ============

/**
 * 运行层合板分析
 */
export async function runLaminateAnalysis(config: LaminateConfig): Promise<LaminateResult> {
  return invoke<LaminateResult>('run_laminate_analysis', { config })
}

/**
 * 计算 ABD 矩阵
 */
export async function calculateABDMatrix(
  plies: PlyProperty[],
  symmetry: 'none' | 'symmetric' | 'anti-symmetric'
): Promise<ABDMatrix> {
  return invoke<ABDMatrix>('calculate_abd_matrix', { plies, symmetry })
}

/**
 * 计算各铺层应力
 */
export async function calculatePlyStresses(
  abd: ABDMatrix,
  loading: LaminateConfig['loading']
): Promise<PlyResult[]> {
  return invoke<PlyResult[]>('calculate_ply_stresses', { abd, loading })
}

/**
 * 校核失效准则
 */
export async function checkFailureCriteria(
  stresses: PlyResult[],
  strengths: PlyProperty[]
): Promise<FailureIndex[]> {
  return invoke<FailureIndex[]>('check_failure_criteria', { stresses, strengths })
}

/**
 * 铺层顺序优化
 */
export async function optimizeLayup(
  config: LaminateConfig,
  objective: 'min_weight' | 'max_strength' | 'balanced'
): Promise<{ optimal_sequence: number[]; min_weight: number; max_strength: number }> {
  return invoke<{ optimal_sequence: number[]; min_weight: number; max_strength: number }>(
    'optimize_layup', { config, objective }
  )
}

/**
 * 获取复合材料工程模板
 */
export async function getCompositeTemplates(): Promise<CompositeTemplate[]> {
  return invoke<CompositeTemplate[]>('get_composite_templates')
}

/**
 * 获取复合材料材料库
 */
export async function getCompositeMaterialLibrary(): Promise<PlyProperty[]> {
  return invoke<PlyProperty[]>('get_composite_material_library')
}
