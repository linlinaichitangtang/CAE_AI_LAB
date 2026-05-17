/**
 * V4.0 仿真向导逻辑
 * 管理向导状态、步骤切换、数据收集
 */

import { ref, computed } from 'vue'
import type { SimulationTemplate, WizardStep } from '../data/templates/simulationTemplates'

export type WizardPhase = 'select' | 'wizard' | 'complete'

export interface WizardState {
  phase: WizardPhase
  template: SimulationTemplate | null
  currentStepIndex: number
  stepData: Record<string, Record<string, unknown>>
  completedSteps: Set<string>
  startTime: number
}

export function useSimulationWizard() {
  const state = ref<WizardState>({
    phase: 'select',
    template: null,
    currentStepIndex: 0,
    stepData: {},
    completedSteps: new Set(),
    startTime: 0
  })

  // ============================================================================
  // 计算属性
  // ============================================================================

  const currentStep = computed<WizardStep | null>(() => {
    const t = state.value.template
    if (!t) return null
    return t.steps[state.value.currentStepIndex] ?? null
  })

  const totalSteps = computed(() => state.value.template?.steps.length ?? 0)

  const currentStepIndex = computed(() => state.value.currentStepIndex)

  const progress = computed(() => {
    if (totalSteps.value === 0) return 0
    return ((state.value.currentStepIndex + 1) / totalSteps.value) * 100
  })

  const isFirstStep = computed(() => state.value.currentStepIndex === 0)

  const isLastStep = computed(() =>
    state.value.currentStepIndex === totalSteps.value - 1
  )

  const canGoNext = computed(() => {
    const step = currentStep.value
    if (!step) return false
    // 最后一步（result）随时可以完成
    if (step.component === 'result') return true
    // 其他步骤需要验证
    const data = state.value.stepData[step.id]
    if (step.validation && data) {
      const result = step.validation(data)
      return result === true
    }
    return true
  })

  const elapsedTime = computed(() => {
    if (state.value.startTime === 0) return 0
    return Math.floor((Date.now() - state.value.startTime) / 1000)
  })

  // ============================================================================
  // 操作方法
  // ============================================================================

  /** 选择模板并启动向导 */
  function selectTemplate(template: SimulationTemplate) {
    state.value = {
      phase: 'wizard',
      template,
      currentStepIndex: 0,
      stepData: {},
      completedSteps: new Set(),
      startTime: Date.now()
    }
  }

  /** 下一步 */
  function nextStep() {
    if (currentStep.value) {
      state.value.completedSteps.add(currentStep.value.id)
    }
    if (!isLastStep.value) {
      state.value.currentStepIndex++
    }
  }

  /** 上一步 */
  function prevStep() {
    if (!isFirstStep.value) {
      state.value.currentStepIndex--
    }
  }

  /** 跳转到指定步骤 */
  function goToStep(index: number) {
    if (index >= 0 && index < totalSteps.value) {
      state.value.currentStepIndex = index
    }
  }

  /** 保存步骤数据 */
  function saveStepData(stepId: string, data: Record<string, unknown>) {
    state.value.stepData[stepId] = { ...state.value.stepData[stepId], ...data }
  }

  /** 获取步骤数据 */
  function getStepData(stepId: string): Record<string, unknown> | undefined {
    return state.value.stepData[stepId]
  }

  /** 完成向导 */
  function completeWizard() {
    state.value.phase = 'complete'
    state.value.completedSteps = new Set(
      state.value.template?.steps.map(s => s.id) ?? []
    )
  }

  /** 重置向导 */
  function resetWizard() {
    state.value = {
      phase: 'select',
      template: null,
      currentStepIndex: 0,
      stepData: {},
      completedSteps: new Set(),
      startTime: 0
    }
  }

  /** 返回模板选择 */
  function backToSelection() {
    resetWizard()
  }

  return {
    state,
    currentStep,
    currentStepIndex,
    totalSteps,
    progress,
    isFirstStep,
    isLastStep,
    canGoNext,
    elapsedTime,
    selectTemplate,
    nextStep,
    prevStep,
    goToStep,
    saveStepData,
    getStepData,
    completeWizard,
    resetWizard,
    backToSelection
  }
}