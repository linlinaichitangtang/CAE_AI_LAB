/**
 * V2.6 ML 势函数 API - 前端封装
 * 对接 Rust 后端 ml_potential 命令
 */

import { invoke } from '@tauri-apps/api/core'

// ============================================================================
// 类型定义
// ============================================================================

export type MLPotentialType = 'nequip' | 'mace' | 'nep' | 'mtp' | 'ace' | 'gap' | 'chgnet' | 'sevennet'

/** ML 势函数信息 */
export interface MLPotentialInfo {
  name: string
  potentialType: string
  description: string
  supportedElements: string[]
  energyRmse?: number
  forceRmse?: number
  cutoff: number
  modelPath?: string
  trainingDataSize?: number
  isReady: boolean
  trainingTimeSec?: number
  createdAt?: string
}

/** ML 势函数推理请求 */
export interface MLPotentialComputeRequest {
  potentialName: string
  positions: number[][]
  atomTypes: string[]
  cell?: number[][]
  computeForces: boolean
  computeStress: boolean
}

/** ML 势函数推理结果 */
export interface MLPotentialComputeResult {
  potentialName: string
  totalEnergy: number
  energyPerAtom: number
  forces?: number[][]
  stress?: number[][]
  inferenceTimeMs: number
  numAtoms: number
}

/** 势函数选择请求 */
export interface PotentialSelectionRequest {
  elements: string[]
  numAtoms: number
  accuracyRequirement: 'high' | 'medium' | 'low'
  hasGpu: boolean
}

/** 势函数选择结果 */
export interface PotentialSelectionResult {
  recommendedType: string
  reason: string
  needsTraining: boolean
  estimatedTrainingMinutes?: number
  estimatedVramGb?: number
  alternatives: AlternativePotential[]
}

export interface AlternativePotential {
  potentialType: string
  reason: string
}

/** GPU 信息 */
export interface GpuInfo {
  name: string
  totalMemoryGb: number
  availableMemoryGb: number
  isAvailable: boolean
  cudaVersion?: string
}

/** GPU 资源状态 */
export interface GpuResourceStatus {
  gpus: GpuInfo[]
  totalVramGb: number
  usedVramGb: number
  runningTasks: number
  queuedTasks: number
}

/** 训练配置 */
export interface TrainingConfig {
  potentialType: string
  trainingDataPath: string
  validationDataPath?: string
  cutoff: number
  maxNeuron: number
  epochs: number
  batchSize: number
  learningRate: number
  energyWeight: number
  forceWeight: number
  stressWeight?: number
  useGpu: boolean
}

/** 训练结果 */
export interface TrainingResult {
  potentialName: string
  potentialType: string
  success: boolean
  finalEnergyRmse: number
  finalForceRmse: number
  bestEpoch: number
  modelPath: string
  trainingTimeSec: number
  lossHistory: LossRecord[]
}

export interface LossRecord {
  epoch: number
  energyRmse: number
  forceRmse: number
  valEnergyRmse?: number
  valForceRmse?: number
}

/** 验证请求 */
export interface ValidationRequest {
  potentialName: string
  testDataPath: string
}

/** 验证结果 */
export interface ValidationResult {
  potentialName: string
  energyRmse: number
  forceRmse: number
  energyMae: number
  forceMae: number
  numTestStructures: number
  passed: boolean
  details: StructureValidationDetail[]
}

export interface StructureValidationDetail {
  structureId: string
  dftEnergy: number
  mlEnergy: number
  energyError: number
  maxForceError: number
}

/** 超参数推荐请求 */
export interface HyperparamRecommendationRequest {
  numAtoms: number
  numSpecies: number
  numTrainingStructures: number
  potentialType: string
}

/** 超参数推荐结果 */
export interface HyperparamRecommendation {
  cutoff: number
  maxNeuron: number
  epochs: number
  batchSize: number
  learningRate: number
  energyWeight: number
  forceWeight: number
  estimatedTrainingTimeMinutes: number
  estimatedVramGb: number
  reasoning: string
}

// ============================================================================
// API 函数
// ============================================================================

/** 列出可用的 ML 势函数 */
export async function listMlPotentials(): Promise<MLPotentialInfo[]> {
  return invoke<MLPotentialInfo[]>('list_ml_potentials')
}

/** ML 势函数推理 */
export async function computeMlPotential(request: MLPotentialComputeRequest): Promise<MLPotentialComputeResult> {
  return invoke<MLPotentialComputeResult>('compute_ml_potential', { request })
}

/** 势函数自动选择 */
export async function autoSelectPotential(request: PotentialSelectionRequest): Promise<PotentialSelectionResult> {
  return invoke<PotentialSelectionResult>('auto_select_potential', { request })
}

/** 获取 GPU 资源状态 */
export async function getGpuStatus(): Promise<GpuResourceStatus> {
  return invoke<GpuResourceStatus>('get_gpu_status')
}

/** 推荐训练超参数 */
export async function recommendTrainingHyperparams(request: HyperparamRecommendationRequest): Promise<HyperparamRecommendation> {
  return invoke<HyperparamRecommendation>('recommend_training_hyperparams', { request })
}

/** 提交训练任务 */
export async function submitTrainingJob(config: TrainingConfig): Promise<TrainingResult> {
  return invoke<TrainingResult>('submit_training_job', { config })
}

/** 验证势函数质量 */
export async function validateMlPotential(request: ValidationRequest): Promise<ValidationResult> {
  return invoke<ValidationResult>('validate_ml_potential', { request })
}

/** 使用 ML 势运行 MD 模拟 */
export async function runMdWithMlPotential(params: {
  potentialName: string
  positions: number[][]
  atomTypes: string[]
  cell?: number[][]
  ensemble: string
  temperature?: number
  numSteps: number
  timestepFs: number
}): Promise<Record<string, unknown>> {
  return invoke<Record<string, unknown>>('run_md_with_ml_potential', { ...params })
}
