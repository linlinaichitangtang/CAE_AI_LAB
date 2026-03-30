// API for Transient Dynamics Analysis
import { invoke } from '@tauri-apps/api/tauri'

// ============ Types ============

export interface LoadPoint {
  time: number
  value: number
}

export type LoadType = 'step' | 'sinusoid' | 'impulse' | 'custom'

export interface LoadCurve {
  id: string
  name: string
  load_type: LoadType
  points: LoadPoint[]
  amplitude: number
  frequency: number
  phase: number
  duration: number
}

export interface RayleighDamping {
  alpha: number
  beta: number
  frequency1: number
  frequency2: number
}

export interface TimeStepControl {
  initial_dt: number
  min_dt: number
  max_dt: number
  total_time: number
  output_interval: number
}

export interface TransientAnalysisConfig {
  name: string
  mesh_id: string
  load_curves: LoadCurve[]
  damping: RayleighDamping
  time_control: TimeStepControl
  initial_conditions: number[]
}

export interface TransientResultStep {
  time: number
  displacements: number[]
  velocities: number[]
  accelerations: number[]
  stresses: number[]
  max_displacement: number
  max_stress: number
}

export interface TransientResults {
  analysis_name: string
  total_time: number
  steps: TransientResultStep[]
  max_displacement_overall: number
  max_stress_overall: number
  natural_frequencies: number[]
}

export interface TutorialExample {
  id: string
  name: string
  description: string
  config: TransientAnalysisConfig
}

// ============ API Functions ============

export async function getTutorialExamples(): Promise<TutorialExample[]> {
  return await invoke<TutorialExample[]>('get_tutorial_examples')
}

export async function generateTransientInpFile(
  config: TransientAnalysisConfig,
  outputPath: string
): Promise<string> {
  return await invoke<string>('generate_transient_inp_file', {
    configJson: JSON.stringify(config),
    outputPath
  })
}

export async function runTransientSimulation(
  config: TransientAnalysisConfig
): Promise<TransientResults> {
  return await invoke<TransientResults>('run_transient_simulation', {
    configJson: JSON.stringify(config)
  })
}

export async function calculateRayleighCoefficients(
  frequency1: number,
  frequency2: number,
  dampingRatio: number
): Promise<RayleighDamping> {
  return await invoke<RayleighDamping>('calculate_rayleigh_coefficients', {
    frequency1,
    frequency2,
    dampingRatio
  })
}

export async function getLoadCurveTemplate(
  templateType: 'step' | 'sinusoid' | 'impulse'
): Promise<LoadCurve> {
  return await invoke<LoadCurve>('get_load_curve_template', {
    templateType
  })
}