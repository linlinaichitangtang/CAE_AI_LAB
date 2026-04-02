/**
 * CAE API - Optimization (Topology + Shape)
 */

import { invoke } from '@tauri-apps/api/core'

// ============================================================================
// Topology Optimization
// ============================================================================

/** Generate STL file from optimized density field */
export async function exportTopologyToStl(request: {
  nodes: Array<{ id: number; x: number; y: number; z: number }>
  elements: Array<{ id: number; node_ids: number[] }>
  density_field: number[]
  threshold: number
  output_path: string
}): Promise<{ success: boolean; file_path: string; element_count: number; node_count: number; message?: string }> {
  return await invoke('export_topology_to_stl', { request })
}

/** Export optimization results with STL geometry */
export async function exportOptimizationResults(request: {
  nodes: Array<{ id: number; x: number; y: number; z: number }>
  elements: Array<{ id: number; node_ids: number[] }>
  density_field: number[]
  optimization_config: any
  iterations: Array<{ iteration: number; objective_value: number; volume_fraction: number; density_field?: number[] }>
  output_dir: string
}): Promise<{ success: boolean; stl_path: string; report_path: string; message?: string }> {
  return await invoke('export_optimization_results', { request })
}

/** Load optimization template */
export async function loadOptimizationTemplate(template_name: string): Promise<{
  name: string
  description: string
  optimization_type: string
  objective: string
  volume_fraction: number
  max_iterations: number
  penalization_factor: number
  min_density: number
  design_domain: { x_min: number; x_max: number; y_min: number; y_max: number; z_min: number; z_max: number }
  mesh_config: { x_div: number; y_div: number; z_div: number }
  material: { elastic_modulus: number; poisson_ratio: number }
  boundary_conditions: any
}> {
  return await invoke('load_optimization_template', { template_name })
}

/** Run topology optimization with real iterations */
export async function runTopologyOptimizationFull(request: {
  config: any
  nodes: Array<{ id: number; x: number; y: number; z: number }>
  elements: Array<{ id: number; node_ids: number[] }>
  boundary_conditions: any
  material: any
}): Promise<{
  success: boolean
  iterations: Array<{
    iteration: number
    objective_value: number
    volume_fraction: number
    max_stress: number | null
    max_density: number
    min_density: number
    converged: boolean
  }>
  final_objective: number
  final_volume: number
  final_density_field: number[]
  elapsed_time_seconds: number
  error_message?: string
}> {
  return await invoke('run_topology_optimization_full', { request })
}

/** Get iteration density field for animation */
export async function getIterationDensityField(iteration: number): Promise<number[]> {
  return await invoke('get_iteration_density_field', { iteration })
}

/** Reset optimizer state */
export async function resetOptimizer(): Promise<void> {
  return await invoke('reset_optimizer')
}

/** Calculate stiffness matrix contribution for SIMP */
export async function calculateSimpStiffness(
  base_young: number,
  density: number,
  penalization: number
): Promise<number> {
  return await invoke('calculate_simp_stiffness', { base_young, density, penalization })
}

/** Sensitivity analysis for OC method */
export async function calculateOcSensitivity(
  element_index: number,
  density: number,
  compliance: number,
  beta: number
): Promise<number> {
  return await invoke('calculate_oc_sensitivity', { element_index, density, compliance, beta })
}

/** Export STL as ASCII format for easy reading */
export async function exportStlAscii(request: {
  nodes: Array<{ id: number; x: number; y: number; z: number }>
  elements: Array<{ id: number; node_ids: number[] }>
  density_field: number[]
  threshold: number
  output_path: string
}): Promise<{ success: boolean; file_path: string; triangle_count: number }> {
  return await invoke('export_stl_ascii', { request })
}

// ============================================================================
// Shape Optimization
// ============================================================================

export interface ShapeOptConfig {
  mesh_nodes: Array<[number, number, number]>
  mesh_elements: Array<[number, number, number, number]> // tetrahedra
  boundary_node_ids: number[]
  fixed_node_ids: number[]
  load_node_ids: number[]
  load_values: number[]
  objective: string // "minimize_compliance"
  max_movement: number
  max_iterations: number
  convergence_tolerance: number
}

export interface ShapeOptResult {
  optimized_nodes: Array<[number, number, number]>
  iterations: ShapeOptIteration[]
  converged: boolean
  final_compliance: number
  elapsed_time_seconds: number
  error_message?: string
}

export interface ShapeOptIteration {
  iteration: number
  compliance: number
  max_movement: number
  node_displacements: Array<[number, number, number]>
}

/** Run shape optimization */
export async function runShapeOptimization(config: ShapeOptConfig): Promise<ShapeOptResult> {
  return await invoke('run_shape_optimization_v2', { config })
}

/** Export shape optimization result to STL */
export async function exportShapeOptToStl(request: {
  nodes: Array<[number, number, number]>
  elements: Array<[number, number, number, number]>
  output_path: string
}): Promise<{ success: boolean; file_path: string; triangle_count: number }> {
  return await invoke('export_shape_opt_to_stl', { request })
}
