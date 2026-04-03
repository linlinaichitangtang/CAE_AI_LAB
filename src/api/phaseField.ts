/**
 * Phase Field Simulation API
 * V1.6-001: 相场方程求解器
 * V1.6-002: 组织初始化器
 * V1.6-003: 相场-温度耦合
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

/** 相场方程类型 */
export type PfEquation =
  | 'cahn_hilliard'
  | 'allen_cahn'
  | 'phase_field_crystal'
  | 'karma_model'

/** 自由能类型 */
export type FreeEnergyType =
  | 'double_well'
  | 'polynomial'
  | 'landau'
  | 'regular_solution'

/** 时间积分方法 */
export type TimeIntegration =
  | 'explicit_euler'
  | 'implicit_euler'
  | 'semi_implicit'
  | 'runge_kutta4'

/** 计算网格 */
export interface PfGrid {
  nx: number
  ny: number
  nz: number
  dx: number
  dy: number
  dz: number
}

/** 相场材料参数 */
export interface PfMaterial {
  name: string
  free_energy_type: FreeEnergyType
  free_energy_coefficients: Record<string, number>
  mobility: number
  gradient_energy_coeff: number
  latent_heat: number
  heat_capacity: number
  thermal_conductivity: number
  interface_width: number
  equilibrium_order_parameter: number
}

/** 初始条件 */
export interface PfInitialCondition {
  type: 'random' | 'nucleation' | 'layered' | 'circular' | 'import'
  params: Record<string, number>
  grain_count: number
  nucleation_sites: Array<{ x: number; y: number; r: number }>
  noise_amplitude: number
}

/** 热耦合配置 */
export interface PfThermalConfig {
  initial_temperature_K: number
  boundary_temperature_K: number
  thermal_coupling: 'none' | 'one_way' | 'two_way'
  latent_heat_release: boolean
  heat_source: Array<{ x: number; y: number; power: number }>
}

/** 求解器配置 */
export interface PfSolverConfig {
  project_id: string
  equation: PfEquation
  grid: PfGrid
  material: PfMaterial
  initial_condition: PfInitialCondition
  thermal: PfThermalConfig
  time_integration: TimeIntegration
  dt: number
  num_steps: number
  output_interval: number
  adaptive_timestep: boolean
  convergence_tolerance: number
}

/** 帧数据 */
export interface PfFrame {
  step: number
  time: number
  order_param_stats: {
    mean: number
    std: number
    min: number
    max: number
  }
  temperature_stats: {
    mean: number
    std: number
    min: number
    max: number
  }
}

/** 求解结果 */
export interface PfResult {
  success: boolean
  total_time: number
  num_steps_completed: number
  order_parameter_field: number[][]
  temperature_field: number[][]
  free_energy_density: number[][]
  grain_boundaries: number[][]
  total_free_energy: number
  interface_area: number
  grain_count: number
  avg_grain_size: number
  max_order_parameter: number
  min_order_parameter: number
  frames: PfFrame[]
}

/** 模板 */
export interface PfTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  equation: PfEquation
  typical_application: string
  reference: string
}

/** 配置验证结果 */
export interface PfValidationResult {
  valid: boolean
  errors: string[]
  warnings: string[]
}

/** 内存估算结果 */
export interface PfMemoryEstimate {
  estimated_mb: number
  recommended_ram: string
}

// ============ API 函数 ============

/**
 * 运行相场模拟
 * V1.6-001: 求解 Cahn-Hilliard / Allen-Cahn / 相场晶体 / Karma 模型
 */
export async function runPhaseFieldSimulation(config: PfSolverConfig): Promise<PfResult> {
  return invoke<PfResult>('run_phase_field_simulation', { config })
}

/**
 * 生成初始条件
 * V1.6-002: 生成随机/形核/层状/圆形初始序参量场
 */
export async function generateInitialCondition(
  config: PfInitialCondition,
  grid: PfGrid
): Promise<number[][]> {
  return invoke<number[][]>('generate_pf_initial_condition', { config, grid })
}

/**
 * 计算自由能密度场
 * V1.6-001: 根据序参量场和材料参数计算自由能密度分布
 */
export async function calculateFreeEnergy(
  orderParam: number[][],
  material: PfMaterial
): Promise<number[][]> {
  return invoke<number[][]>('calculate_pf_free_energy', { orderParam, material })
}

/**
 * 获取相场模拟模板
 */
export async function getPhaseFieldTemplates(): Promise<PfTemplate[]> {
  return invoke<PfTemplate[]>('get_phase_field_templates')
}

/**
 * 验证求解器配置
 */
export async function validatePfConfig(config: PfSolverConfig): Promise<PfValidationResult> {
  return invoke<PfValidationResult>('validate_pf_config', { config })
}

/**
 * 估算内存需求
 */
export async function estimatePfMemory(config: PfSolverConfig): Promise<PfMemoryEstimate> {
  return invoke<PfMemoryEstimate>('estimate_pf_memory', { config })
}
