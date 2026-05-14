/**
 * tc4Failure.ts — V3.8 TC4 多模态失效分析 API
 *
 * 三路输入：EBSD 晶格 + SEM 图像 + Load 载荷
 * 输出：失效模式分类 + 疲劳寿命预测
 */

import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export interface EbsdFeatures {
  sample_id: string
  features: number[]
  a?: number
  c?: number
  c_to_a?: number
}

export interface SemFeatures {
  sample_id: string
  features: number[]
  quality_score: number
}

export interface LoadFeatures {
  sample_id: string
  features: number[]
  load_type: string
}

export type FailureMode = 'HCF' | 'LCF' | 'TMF' | 'CREEP' | 'OVERLOAD' | 'FOD'

export interface MultimodalPrediction {
  sample_id: string
  failure_mode: FailureMode
  class_confidence: number
  log_cycles: number
  cycles: number
  cycle_confidence: number
  fused_features: number[]
}

export interface Tc4SyntheticConfig {
  num_samples: number
  output_dir: string
  use_tc4: boolean
  seed?: number
}

export interface Tc4MdValidationConfig {
  lattice_a: number
  lattice_c: number
  crack_plane: string
  crack_direction: string
  validation_type: string
}

// ============ API 调用 ============

export async function tc4GenerateSynthetic(config: Tc4SyntheticConfig): Promise<string> {
  return invoke('tc4_generate_synthetic', { config })
}

export async function tc4LoadEbsd(ebsdPath: string, material: string): Promise<EbsdFeatures> {
  return invoke('tc4_load_ebsd', { ebsdPath, material })
}

export async function tc4LoadSem(semImagePath: string): Promise<SemFeatures> {
  return invoke('tc4_load_sem', { semImagePath })
}

export async function tc4LoadLoad(loadPath: string): Promise<LoadFeatures> {
  return invoke('tc4_load_load', { loadPath })
}

export async function tc4MultimodalPredict(
  ebsd: EbsdFeatures,
  sem: SemFeatures,
  load: LoadFeatures
): Promise<MultimodalPrediction> {
  return invoke('tc4_multimodal_predict', { ebsd, sem, load })
}

export async function tc4MdValidation(config: Tc4MdValidationConfig): Promise<string> {
  return invoke('tc4_md_validation', { config })
}

// ============ Pinia Store ============

export const useTc4FailureStore = defineStore('tc4Failure', {
  state: () => ({
    // 数据状态
    ebsd: null as EbsdFeatures | null,
    sem: null as SemFeatures | null,
    load: null as LoadFeatures | null,
    prediction: null as MultimodalPrediction | null,

    // 状态
    isLoading: false,
    isGenerating: false,
    error: null as string | null,

    // 历史记录
    history: [] as MultimodalPrediction[],

    // MD 验证结果
    mdResult: null as string | null,
  }),

  getters: {
    hasAllInputs: (state) => state.ebsd !== null && state.sem !== null && state.load !== null,

    failureModeLabel: (state) => {
      if (!state.prediction) return ''
      const labels: Record<FailureMode, string> = {
        HCF: '高周疲劳',
        LCF: '低周疲劳',
        TMF: '热机械疲劳',
        CREEP: '蠕变',
        OVERLOAD: '过载',
        FOD: '外来物损伤',
      }
      return labels[state.prediction.failure_mode] || state.prediction.failure_mode
    },

    cycleRange: (state) => {
      if (!state.prediction) return { min: 0, max: 0 }
      const cycles = state.prediction.cycles
      if (cycles < 1000) return { min: 0, max: 10000 }
      if (cycles < 1e6) return { min: 0, max: 1e7 }
      return { min: 0, max: 1e10 }
    },
  },

  actions: {
    async generateSynthetic(config: Tc4SyntheticConfig) {
      this.isGenerating = true
      this.error = null
      try {
        const result = await tc4GenerateSynthetic(config)
        return result
      } catch (e) {
        this.error = String(e)
        throw e
      } finally {
        this.isGenerating = false
      }
    },

    async loadEbsd(ebsdPath: string, material: string = 'TC4') {
      this.isLoading = true
      this.error = null
      try {
        this.ebsd = await tc4LoadEbsd(ebsdPath, material)
        return this.ebsd
      } catch (e) {
        this.error = String(e)
        throw e
      } finally {
        this.isLoading = false
      }
    },

    async loadSem(semImagePath: string) {
      this.isLoading = true
      this.error = null
      try {
        this.sem = await tc4LoadSem(semImagePath)
        return this.sem
      } catch (e) {
        this.error = String(e)
        throw e
      } finally {
        this.isLoading = false
      }
    },

    async loadLoad(loadPath: string) {
      this.isLoading = true
      this.error = null
      try {
        this.load = await tc4LoadLoad(loadPath)
        return this.load
      } catch (e) {
        this.error = String(e)
        throw e
      } finally {
        this.isLoading = false
      }
    },

    async predict() {
      if (!this.ebsd || !this.sem || !this.load) {
        throw new Error('请先加载所有三路输入数据')
      }
      this.isLoading = true
      this.error = null
      try {
        this.prediction = await tc4MultimodalPredict(this.ebsd, this.sem, this.load)
        if (this.prediction) {
          this.history.unshift(this.prediction)
          if (this.history.length > 50) this.history.pop()
        }
        return this.prediction
      } catch (e) {
        this.error = String(e)
        throw e
      } finally {
        this.isLoading = false
      }
    },

    async runMdValidation(config: Tc4MdValidationConfig) {
      this.isLoading = true
      this.error = null
      try {
        this.mdResult = await tc4MdValidation(config)
        return this.mdResult
      } catch (e) {
        this.error = String(e)
        throw e
      } finally {
        this.isLoading = false
      }
    },

    clearInputs() {
      this.ebsd = null
      this.sem = null
      this.load = null
      this.prediction = null
      this.error = null
    },

    clearHistory() {
      this.history = []
    },
  },
})