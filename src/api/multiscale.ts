/**
 * Multiscale Bridge API
 * V1.4-009: MD/相场 → FE 桥接 API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type ScaleLevel = 'quantum' | 'atomistic' | 'meso' | 'macro'

export type BridgeMethod =
  | 'atomistic_to_fe'
  | 'phase_field_to_fe'
  | 'coarse_graining'
  | 'parameter_passing'
  | 'concurrent'

export interface AtomisticData {
  num_atoms: number
  box_size: { x: number; y: number; z: number }
  potential_type: 'eam' | 'lj' | 'tersoff' | 'reaxff'
  species: string[]
  stress_tensor: number[]       // 6 components: xx, yy, zz, xy, yz, xz
  strain_tensor: number[]       // 6 components: xx, yy, zz, xy, yz, xz
  temperature: number           // K
  time_step_fs: number          // femtoseconds
}

export interface PhaseFieldData {
  grid_size: { nx: number; ny: number; nz: number }
  field_variables: string[]
  interface_width: number
  mobility: number
  free_energy_density: number
  grain_boundaries: number[]
}

export type CouplingType = 'sequential' | 'concurrent' | 'hierarchical'

export interface MultiscaleBridgeConfig {
  project_id: string
  method: BridgeMethod
  source_data: AtomisticData | PhaseFieldData
  target_mesh_size: number
  coupling_type: CouplingType
  overlap_region: number
  relaxation_iterations: number
}

export interface MultiscaleBridgeResult {
  success: boolean
  equivalent_boundary_conditions: Array<{
    node_id: number
    dof: string
    value: number
  }>
  mapped_stress_field: Array<{
    x: number
    y: number
    z: number
    sxx: number
    syy: number
    szz: number
    sxy: number
    syz: number
    sxz: number
  }>
  mapped_strain_field: Array<{
    x: number
    y: number
    z: number
    exx: number
    eyy: number
    ezz: number
    exy: number
    eyz: number
    exz: number
  }>
  coarse_grained_parameters: {
    equivalent_modulus: number
    equivalent_strength: number
    equivalent_poisson_ratio: number
    equivalent_density: number
    equivalent_hardening: number
  }
  data_flow_visualization: {
    scales: Array<{
      level: ScaleLevel
      label: string
      data_size: number
      description: string
    }>
    connections: Array<{
      from: ScaleLevel
      to: ScaleLevel
      method: string
      data_transfer: string
    }>
  }
  computational_cost_reduction: {
    full_atomistic_time: number
    multiscale_time: number
    speedup_factor: number
    accuracy_retention: number
  }
}

export interface CoarseGrainedResult {
  success: boolean
  representative_volume_size: number
  equivalent_properties: {
    E11: number
    E22: number
    E33: number
    G12: number
    G23: number
    G13: number
    nu12: number
    nu23: number
    nu13: number
  }
  stress_strain_response: Array<{
    strain: number
    stress: number
  }>
  homogenization_method: string
  convergence_info: {
    iterations: number
    residual: number
    converged: boolean
  }
}

export interface EquivalentBC {
  success: boolean
  boundary_type: 'displacement' | 'traction' | 'mixed'
  nodes: Array<{
    id: number
    x: number
    y: number
    z: number
  }>
  conditions: Array<{
    node_id: number
    dof: number
    value: number
  }>
  stiffness_matrix: number[][]
  load_vector: number[]
}

export interface MultiscaleTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  method: BridgeMethod
  typical_application: string
}

// ============ API 函数 ============

/**
 * 运行多尺度桥接分析
 */
export async function runMultiscaleBridge(config: MultiscaleBridgeConfig): Promise<MultiscaleBridgeResult> {
  return invoke<MultiscaleBridgeResult>('run_multiscale_bridge', { config })
}

/**
 * 粗粒化分子动力学数据
 */
export async function coarseGrainMD(
  atomistic_data: AtomisticData,
  target_size: number
): Promise<CoarseGrainedResult> {
  return invoke<CoarseGrainedResult>('coarse_grain_md', { atomisticData: atomistic_data, targetSize: target_size })
}

/**
 * 生成等效边界条件
 */
export async function generateEquivalentBC(
  micro_result: MultiscaleBridgeResult,
  macro_mesh: { nodes: number; elements: number }
): Promise<EquivalentBC> {
  return invoke<EquivalentBC>('generate_equivalent_bc', { microResult: micro_result, macroMesh: macro_mesh })
}

/**
 * 获取多尺度桥接模板
 */
export async function getMultiscaleTemplates(): Promise<MultiscaleTemplate[]> {
  return invoke<MultiscaleTemplate[]>('get_multiscale_templates')
}
