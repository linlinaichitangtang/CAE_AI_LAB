/**
 * Thermal-Fluid Coupling API
 * V1.3-003: 热-流耦合分析 API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export interface ThermalFluidConfig {
  fluid_domain: string
  solid_domain: string
  inlet_temperature: number      // K
  outlet_pressure: number        // Pa
  heat_source: number            // W/m^3
  fluid_material: FluidMaterialConfig
  solid_material: SolidMaterialConfig
  heat_sink_params?: HeatSinkParams
  max_iterations: number
  convergence_tolerance: number
}

export interface FluidMaterialConfig {
  name: string
  density: number               // kg/m^3
  viscosity: number             // Pa.s
  thermal_conductivity: number  // W/(m.K)
  specific_heat: number         // J/(kg.K)
}

export interface SolidMaterialConfig {
  name: string
  density: number               // kg/m^3
  thermal_conductivity: number  // W/(m.K)
  specific_heat: number         // J/(kg.K)
}

export interface HeatSinkParams {
  num_fins: number
  fin_spacing: number           // mm
  fin_height: number            // mm
  fin_thickness: number         // mm
  base_thickness: number        // mm
}

export interface ThermalFluidResult {
  success: boolean
  max_temperature: number       // K
  min_temperature: number       // K
  avg_temperature: number       // K
  max_velocity: number          // m/s
  heat_dissipation: number      // W
  thermal_resistance: number    // K/W
  pressure_drop: number         // Pa
  efficiency: number            // %
  temperature_field: Array<{ x: number; y: number; z: number; temperature: number }>
  velocity_field: Array<{ x: number; y: number; z: number; vx: number; vy: number; vz: number }>
  iterations: number
  converged: boolean
}

// ============ API 函数 ============

/**
 * 运行热-流耦合分析 (共轭传热)
 */
export async function runThermalFluidAnalysis(config: ThermalFluidConfig): Promise<ThermalFluidResult> {
  return invoke<ThermalFluidResult>('run_conjugate_heat_transfer', { config })
}

/**
 * 优化散热片参数
 */
export async function optimizeHeatSink(params: HeatSinkParams & {
  max_temperature: number
  heat_source_power: number
  inlet_temperature: number
  flow_velocity: number
}): Promise<{
  optimized_params: HeatSinkParams
  estimated_temperature: number
  thermal_resistance: number
  efficiency: number
}> {
  return invoke('optimize_heat_sink', { params })
}
