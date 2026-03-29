export interface Project {
  id: string
  name: string
  description: string
  createdAt: Date
  updatedAt: Date
  modules: ProjectModule[]
}

export interface ProjectModule {
  type: 'notes' | 'modeling' | 'code' | 'simulation'
  enabled: boolean
}

export interface Note {
  id: string
  projectId: string
  title: string
  content: string
  createdAt: Date
  updatedAt: Date
}

export interface Model {
  id: string
  projectId: string
  name: string
  geometry: GeometryData
  createdAt: Date
  updatedAt: Date
}

export interface GeometryData {
  vertices: number[]
  faces: number[]
}

export interface Simulation {
  id: string
  projectId: string
  modelId: string
  config: SimulationConfig
  results?: SimulationResults
  status: 'pending' | 'running' | 'completed' | 'failed'
}

export interface SimulationConfig {
  meshSize: number
  boundaryConditions: BoundaryCondition[]
  material: Material
}

export interface BoundaryCondition {
  type: 'fixed' | 'force' | 'pressure'
  surface: string
  value: number
}

export interface Material {
  name: string
  youngsModulus: number
  poissonsRatio: number
}

export interface SimulationResults {
  displacement: number[]
  stress: number[]
  strain: number[]
}

// Simulation Result Viewer types
export interface SimulationResult {
  nodes: Array<{ id: number; x: number; y: number; z: number }>
  elements: Array<{ id: number; type: string; nodeIds: number[] }>
  displacement?: { step: number; data: Record<string, Record<string, number>> }
  stress?: { step: number; data: Record<string, Record<string, number>> }
  vonMises?: { step: number; data: Record<string, number> }
  deformationScale?: number
}

export interface EmbeddedObject {
  type: '3d-model' | 'code' | 'simulation-result'
  refId: string
  preview?: string
  position: { x: number; y: number }
  size: { width: number; height: number }
}

export interface EmbedItem {
  id: string
  type: 'model' | 'code' | 'simulation'
  name: string
  data?: any
}