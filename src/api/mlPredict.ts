/**
 * V2.5 ML 预测 API - 前端封装
 * 对接 Rust 后端 ml_predict 命令
 */

import { invoke } from '@tauri-apps/api/core'

// ============================================================================
// 类型定义
// ============================================================================

/** 材料成分 (元素 -> 原子百分比) */
export type Composition = Record<string, number>

/** ML 模型预测请求 */
export interface PredictionRequest {
  composition: Composition
  targetProperties: string[]
  modelName?: string
}

/** 单个属性预测结果 */
export interface PropertyPrediction {
  propertyName: string
  predictedValue: number
  unit: string
  confidenceLower: number
  confidenceUpper: number
  uncertainty: number
}

/** ML 模型预测响应 */
export interface PredictionResponse {
  modelName: string
  isMock: boolean
  inferenceTimeMs: number
  predictions: PropertyPrediction[]
}

/** 可用模型信息 */
export interface ModelInfo {
  name: string
  description: string
  supportedProperties: string[]
  isLoaded: boolean
  modelType: string
}

/** 精度评测请求 */
export interface BenchmarkRequest {
  materialNames: string[]
  targetProperties: string[]
}

/** 精度评测结果 */
export interface BenchmarkResult {
  materialName: string
  propertyName: string
  predictedValue: number
  referenceValue: number
  absoluteError: number
  relativeErrorPercent: number
  passed: boolean
}

// ============================================================================
// API 函数
// ============================================================================

/** ML 属性预测 */
export async function predictMaterialProperties(
  request: PredictionRequest
): Promise<PredictionResponse> {
  return invoke<PredictionResponse>('predict_material_properties', {
    request,
  })
}

/** 获取可用 ML 模型列表 */
export async function listMlModels(): Promise<ModelInfo[]> {
  return invoke<ModelInfo[]>('list_ml_models')
}

/** 运行精度评测 */
export async function runAccuracyBenchmark(
  request: BenchmarkRequest
): Promise<BenchmarkResult[]> {
  return invoke<BenchmarkResult[]>('run_accuracy_benchmark', {
    request,
  })
}
