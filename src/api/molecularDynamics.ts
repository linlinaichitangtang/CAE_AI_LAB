/**
 * Molecular Dynamics API
 * V1.5-001: MD 模块框架
 * V1.5-002: LAMMPS 集成
 * V1.5-005: MD 模拟类型
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type MdEnsemble = 'NVE' | 'NVT' | 'NPT' | 'NPH' | 'UM'

export type ThermostatType = 'nosé_hoover' | 'berendsen' | 'velocity_rescaling' | 'andersen'

export type BarostatType = 'parrinello_rahman' | 'berendsen' | 'andersen'

export type PotentialType = 'lj' | 'eam' | 'meam' | 'tersoff' | 'reaxff' | 'morse' | 'buckingham'

export interface MdAtom {
  id: number
  element: string
  position: { x: number; y: number; z: number }
  velocity: { x: number; y: number; z: number }
  force: { x: number; y: number; z: number }
  charge: number
  mass: number
  type_id: number
}

export type MdBoundary = 'periodic' | 'shrink_wrapped' | 'fixed' | 'reflective'

export interface MdConfig {
  project_id: string
  job_name: string
  ensemble: MdEnsemble
  potential: PotentialType
  potential_file_path: string
  atoms: MdAtom[]
  box: { lx: number; ly: number; lz: number; xy: number; xz: number; yz: number }
  boundary: [MdBoundary, MdBoundary, MdBoundary]
  timestep_fs: number
  num_steps: number
  dump_interval: number
  thermostat: ThermostatType
  thermostat_params: {
    target_temp_K: number
    damping_time_fs: number
  }
  barostat: BarostatType
  barostat_params: {
    target_pressure_Pa: number
    damping_time_fs: number
  }
  neighbor_skin_distance: number
  restart_enabled: boolean
}

export interface MdResult {
  success: boolean
  total_energy: number
  kinetic_energy: number
  potential_energy: number
  temperature: number
  pressure: number
  volume: number
  num_atoms: number
  num_steps_completed: number
  final_config: MdAtom[]
  trajectory_file_path: string
  dump_file_path: string
  log_file_path: string
  wall_time_seconds: number
}

export interface MdLammpsConfig extends MdConfig {
  lammps_executable_path: string
  num_procs: number
  custom_commands: string[]
}

export interface MdTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  ensemble: MdEnsemble
  potential: PotentialType
  typical_application: string
  reference: string
}

// ============ API 函数 ============

/**
 * 运行 MD 模拟
 */
export async function runMdSimulation(config: MdConfig): Promise<MdResult> {
  return invoke<MdResult>('run_md_simulation', { config })
}

/**
 * 运行 LAMMPS 模拟
 */
export async function runLammpsSimulation(config: MdLammpsConfig): Promise<MdResult> {
  return invoke<MdResult>('run_lammps_simulation', { config })
}

/**
 * 生成 LAMMPS 输入文件内容
 */
export async function generateLammpsInput(config: MdConfig): Promise<string> {
  return invoke<string>('generate_lammps_input', { config })
}

/**
 * 解析 LAMMPS 输出文件
 */
export async function parseLammpsOutput(output_path: string): Promise<MdResult> {
  return invoke<MdResult>('parse_lammps_output', { outputPath: output_path })
}

/**
 * 获取 MD 模拟模板列表
 */
export async function getMdTemplates(): Promise<MdTemplate[]> {
  return invoke<MdTemplate[]>('get_md_templates')
}

/**
 * 验证 MD 配置
 */
export async function validateMdConfig(config: MdConfig): Promise<{ valid: boolean; errors: string[]; warnings: string[] }> {
  return invoke<{ valid: boolean; errors: string[]; warnings: string[] }>('validate_md_config', { config })
}

/**
 * 估算内存需求
 */
export async function estimateMemory(config: MdConfig): Promise<{ estimated_mb: number; recommended_ram: string }> {
  return invoke<{ estimated_mb: number; recommended_ram: string }>('estimate_memory', { config })
}

/**
 * 检查 LAMMPS 是否可用
 */
export async function checkLammpsAvailable(): Promise<{ available: boolean; version: string; capabilities: string[] }> {
  return invoke<{ available: boolean; version: string; capabilities: string[] }>('check_lammps_available')
}
