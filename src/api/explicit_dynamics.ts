/**
 * Explicit Dynamics Analysis API - Frontend wrapper for backend explicit dynamics commands
 * 封装显式动力学分析相关的后端API
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export interface ElastoPlasticMaterial {
  id: string
  name: string
  density: number
  youngs_modulus: number
  poisson_ratio: number
  yield_strength: number
  tangent_modulus: number
  hardening_type: string
  failure_strain: number | null
}

export interface FailureModel {
  type: string
  params?: Record<string, number>
}

export interface ContactPair {
  id: string
  name: string
  master_surface: string
  slave_surface: string
  contact_type: string
  friction: number
  contact_stiffness: number
  clearance: number
  damping: number
}

export interface InitialVelocity {
  surface_name: string
  velocity_x: number
  velocity_y: number
  velocity_z: number
  angular_velocity_x?: number
  angular_velocity_y?: number
  angular_velocity_z?: number
}

export interface SolverControl {
  auto_time_step: boolean
  initial_dt: number
  min_dt: number
  max_dt: number
  mass_scaling: boolean
  mass_scaling_factor: number
  critical_time_step_ratio: number
  energy_dissipation_ratio: number
  output_frequency: number
  total_time: number
}

export interface ExplicitDynamicsConfig {
  name: string
  analysis_type: string
  material: ElastoPlasticMaterial
  failure_model: FailureModel
  contact_pairs: ContactPair[]
  initial_velocities: InitialVelocity[]
  solver_control: SolverControl
  output_dir: string
}

export interface ExplicitResultStep {
  time: number
  frame_index: number
  displacements: number[]
  velocities: number[]
  accelerations: number[]
  stresses: number[]
  strains: number[]
  kinetic_energy: number
  internal_energy: number
  max_displacement: number
  max_velocity: number
  max_stress: number
  max_plastic_strain: number
  damage: number | null
}

export interface ExplicitEnergyBalance {
  initial_kinetic_energy: number
  final_kinetic_energy: number
  max_kinetic_energy: number
  internal_energy_gained: number
  energy_dissipated: number
  hourglass_energy: number
  artificial_energy: number
}

export interface FailureLocation {
  node_id: number
  time: number
  frame: number
  damage: number
  plastic_strain: number
  position: [number, number, number]
}

export interface ExplicitDynamicsResults {
  analysis_name: string
  analysis_type: string
  total_time: number
  num_frames: number
  steps: ExplicitResultStep[]
  max_displacement: number
  max_velocity: number
  max_stress: number
  max_plastic_strain: number
  energy_balance: ExplicitEnergyBalance
  failure_locations: FailureLocation[]
}

export interface TemplateExample {
  id: string
  name: string
  description: string
  config: ExplicitDynamicsConfig
}

// ============ API 函数 ============

/**
 * 获取显式动力学模板列表
 */
export async function getExplicitDynamicsTemplates(): Promise<TemplateExample[]> {
  return invoke('get_explicit_dynamics_templates')
}

/**
 * 获取指定模板的详细配置
 */
export async function getExplicitDynamicsTemplate(id: string): Promise<TemplateExample | null> {
  return invoke('get_explicit_dynamics_template', { id })
}

/**
 * 生成显式动力学INP文件
 */
export async function generateExplicitDynamicsInp(
  config: string,
  nodes: string,
  elements: string,
  surfaces: string,
  outputPath: string
): Promise<string> {
  return invoke('generate_explicit_dynamics_inp_cmd', { config, nodes, elements, surfaces, outputPath })
}

/**
 * 生成带动画步的显式动力学INP文件
 */
export async function generateExplicitAnimationInp(
  config: string,
  nodes: string,
  elements: string,
  numSteps: number,
  outputPath: string
): Promise<string> {
  return invoke('generate_explicit_animation_inp_cmd', { config, nodes, elements, numSteps, outputPath })
}

/**
 * 计算临界时间步长
 */
export async function calculateCriticalTimeStep(
  nodes: string,
  elements: string,
  density: number,
  youngsModulus: number
): Promise<number> {
  return invoke('calculate_critical_time_step', { nodes, elements, density, youngsModulus })
}

/**
 * 计算质量缩放建议
 */
export async function calculateMassScalingSuggestion(
  originalDt: number,
  targetDt: number
): Promise<number> {
  return invoke('calculate_mass_scaling_suggestion', { originalDt, targetDt })
}
