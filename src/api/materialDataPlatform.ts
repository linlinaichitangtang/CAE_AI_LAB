/**
 * V2.8 材料数据中台 + 主动学习 API - 前端封装
 */

import { invoke } from '@tauri-apps/api/core'

export interface MaterialPropertyRecord {
  id: string
  projectId?: string
  materialName: string
  composition: string
  processingParams: string
  properties: string
  dataSource: string
  validationStatus: string
  uncertainty?: string
  modelName?: string
  notes?: string
  createdAt: string
}

export interface MaterialDataWriteRequest {
  projectId?: string
  materialName: string
  composition: Record<string, unknown>
  processingParams?: Record<string, unknown>
  properties: Record<string, unknown>
  dataSource: string
  validationStatus?: string
  uncertainty?: Record<string, unknown>
  modelName?: string
  notes?: string
}

export interface MaterialDataQuery {
  materialName?: string
  element?: string
  dataSource?: string
  validationStatus?: string
  limit?: number
  offset?: number
  orderBy?: string
  orderDir?: 'asc' | 'desc'
}

export interface CoverageAnalysis {
  totalRecords: number
  numElements: number
  numMaterials: number
  compositionCoverage: number
  processingCoverage: number
  propertyCoverage: number
  overallCoverage: number
  blindSpots: BlindSpot[]
  qualityScore: number
}

export interface BlindSpot {
  description: string
  elements: string[]
  priority: number
  expectedImprovement: number
}

export interface UncertaintyAnalysisRequest {
  targetProperty: string
  compositionSpace: Array<{ element: string; min: number; max: number }>
  numSamples: number
}

export interface UncertaintyAnalysisResult {
  targetProperty: string
  numSamples: number
  meanUncertainty: number
  maxUncertainty: number
  highUncertaintyRegions: UncertainRegion[]
}

export interface UncertainRegion {
  description: string
  centerComposition: Record<string, number>
  uncertainty: number
  predictedValue: number
}

export interface ActiveLearningRequest {
  targetProperty: string
  numRecommendations: number
  strategy: 'bayesian' | 'greedy' | 'random'
  constraints?: Record<string, unknown>
}

export interface ActiveLearningRecommendation {
  recommendations: RecommendedPoint[]
  expectedImprovement: number
  strategy: string
  currentCoverage: number
  expectedCoverage: number
}

export interface RecommendedPoint {
  rank: number
  materialName: string
  composition: Record<string, number>
  processingParams: Record<string, number>
  expectedInformationGain: number
  predictedUncertainty: number
  reason: string
}

export interface ClosedLoopRequest {
  recommendationId: string
  materialName: string
  composition: Record<string, unknown>
  processingParams: Record<string, unknown>
  targetProperty: string
}

export interface ClosedLoopResult {
  success: boolean
  recommendationId: string
  simulationResult?: string
  mlPredictedValue?: number
  caelabSimulatedValue?: number
  error?: number
  accuracyChange?: number
  coverageChange?: number
  status: string
}

export interface ActiveLearningReport {
  title: string
  generatedAt: string
  initialDataCount: number
  currentDataCount: number
  newDataCount: number
  initialRmse: number
  currentRmse: number
  accuracyImprovementPercent: number
  initialCoverage: number
  currentCoverage: number
  recommendationHistory: RecommendationRecord[]
}

export interface RecommendationRecord {
  round: number
  materialName: string
  composition: Record<string, number>
  mlPrediction: number
  caelabResult: number
  error: number
  timestamp: string
}

export async function writeMaterialProperty(request: MaterialDataWriteRequest): Promise<MaterialPropertyRecord> {
  return invoke('write_material_property', { request })
}

export async function queryMaterialProperties(query: MaterialDataQuery): Promise<MaterialPropertyRecord[]> {
  return invoke('query_material_properties', { query })
}

export async function batchImportMaterialData(records: MaterialDataWriteRequest[]): Promise<number> {
  return invoke('batch_import_material_data', { records })
}

export async function analyzeDataCoverage(): Promise<CoverageAnalysis> {
  return invoke('analyze_data_coverage')
}

export async function analyzeUncertainty(request: UncertaintyAnalysisRequest): Promise<UncertaintyAnalysisResult> {
  return invoke('analyze_uncertainty', { request })
}

export async function getActiveLearningRecommendations(request: ActiveLearningRequest): Promise<ActiveLearningRecommendation> {
  return invoke('get_active_learning_recommendations', { request })
}

export async function runClosedLoopVerification(request: ClosedLoopRequest): Promise<ClosedLoopResult> {
  return invoke('run_closed_loop_verification', { request })
}

export async function generateActiveLearningReport(): Promise<ActiveLearningReport> {
  return invoke('generate_active_learning_report')
}
