/**
 * V2.7 多尺度 Surrogate API - 前端封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============================================================================
// 类型定义
// ============================================================================

export interface MicrostructureImage {
  id: string
  projectId?: string
  fileName: string
  filePath: string
  imageType: string
  width: number
  height: number
  magnification?: number
  scaleBarUm?: number
  materialName?: string
  processingParams?: string
  uploadedAt: string
}

export interface ImageImportRequest {
  projectId?: string
  fileName: string
  filePath: string
  imageType: string
  width: number
  height: number
  magnification?: number
  scaleBarUm?: number
  materialName?: string
  processingParams?: Record<string, unknown>
}

export interface SegmentationRequest {
  imageId: string
  segmentationTarget: string
  useGpu: boolean
}

export interface SegmentationStatistics {
  phaseFractions: Record<string, number>
  porosity: number
  averageGrainSizeUm?: number
  grainSizeStdUm?: number
  iouScore: number
}

export interface SegmentationResult {
  imageId: string
  segmentationTarget: string
  maskBase64?: string
  statistics: SegmentationStatistics
  inferenceTimeMs: number
  isMock: boolean
}

export interface MicrostructureFeatures {
  id: string
  imageId: string
  materialName?: string
  phaseFractions: Record<string, number>
  porosity: number
  avgGrainSizeUm: number
  grainSizeStdUm: number
  shapeFactor: number
  anisotropyRatio: number
  featureVector: number[]
  extractedAt: string
}

export interface MacroPropertyPredictionRequest {
  imageId?: string
  processingParams?: Record<string, unknown>
  featureVector?: number[]
  targetProperties: string[]
  withUncertainty: boolean
}

export interface MacroPropertyPrediction {
  propertyName: string
  predictedValue: number
  unit: string
  confidenceLower: number
  confidenceUpper: number
  uncertainty: number
  needsVerification: boolean
}

export interface MacroPropertyPredictionResponse {
  modelName: string
  isMock: boolean
  inferenceTimeMs: number
  predictions: MacroPropertyPrediction[]
}

export interface TrainingDatasetInfo {
  name: string
  totalSamples: number
  featureDim: number
  targetProperties: string[]
  source: string
  createdAt: string
}

export interface MultiscaleTrainingConfig {
  modelName: string
  trainingDataSource: string
  targetProperties: string[]
  epochs: number
  batchSize: number
  learningRate: number
  hiddenDim: number
  numLayers: number
  useUncertainty: boolean
  useGpu: boolean
}

export interface MultiscaleTrainingResult {
  modelName: string
  success: boolean
  isMock: boolean
  validationErrors: Record<string, number>
  trainingTimeSec: number
  modelPath: string
  lossHistory: Array<{ epoch: number; trainLoss: number; valLoss: number }>
}

export interface OnnxExportResult {
  modelName: string
  onnxFilePath: string
  fileSizeBytes: number
  inputNames: string[]
  outputNames: string[]
  isMock: boolean
}

// ============================================================================
// API 函数
// ============================================================================

export async function importMicrostructureImage(request: ImageImportRequest): Promise<MicrostructureImage> {
  return invoke('import_microstructure_image', { request })
}

export async function segmentMicrostructure(request: SegmentationRequest): Promise<SegmentationResult> {
  return invoke('segment_microstructure', { request })
}

export async function extractMicrostructureFeatures(imageId: string, segmentationTarget?: string): Promise<MicrostructureFeatures> {
  return invoke('extract_microstructure_features', { imageId, segmentationTarget })
}

export async function predictMacroProperties(request: MacroPropertyPredictionRequest): Promise<MacroPropertyPredictionResponse> {
  return invoke('predict_macro_properties', { request })
}

export async function getMultiscaleDatasetInfo(): Promise<TrainingDatasetInfo> {
  return invoke('get_multiscale_dataset_info')
}

export async function trainMultiscaleSurrogate(config: MultiscaleTrainingConfig): Promise<MultiscaleTrainingResult> {
  return invoke('train_multiscale_surrogate', { config })
}

export async function exportMultiscaleOnnx(modelName: string): Promise<OnnxExportResult> {
  return invoke('export_multiscale_onnx', { modelName })
}

export async function verifyWithCaelab(imageId: string, predictedProperties: Record<string, number>): Promise<Record<string, unknown>> {
  return invoke('verify_with_caelab', { imageId, predictedProperties })
}
