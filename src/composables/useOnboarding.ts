/**
 * useOnboarding.ts — V3.8-001 新用户引导
 * 引导新用户了解 CAELab 主要功能
 */
import { ref, computed } from 'vue'

export interface OnboardingStep {
  id: string
  title: string
  description: string
  image?: string
  position: 'center' | 'bottom' | 'top' | 'left' | 'right'
  target?: string  // CSS selector for the element to highlight
  action?: {
    label: string
    handler: () => void
  }
}

export interface OnboardingConfig {
  steps: OnboardingStep[]
  onComplete?: () => void
  onSkip?: () => void
}

const STORAGE_KEY = 'caelab_onboarding_completed'

export function useOnboarding() {
  const isActive = ref(false)
  const currentStepIndex = ref(0)
  const isCompleted = ref(false)

  // 检查是否已完成过引导
  function checkCompleted(): boolean {
    try {
      return localStorage.getItem(STORAGE_KEY) === 'true'
    } catch {
      return false
    }
  }

  // 标记完成
  function markCompleted() {
    try {
      localStorage.setItem(STORAGE_KEY, 'true')
      isCompleted.value = true
    } catch (e) {
      console.error('Failed to save onboarding state:', e)
    }
  }

  // 开始引导
  function startOnboarding() {
    isActive.value = true
    currentStepIndex.value = 0
  }

  // 跳过引导
  function skipOnboarding() {
    isActive.value = false
    markCompleted()
  }

  // 下一步
  function nextStep() {
    currentStepIndex.value++
  }

  // 上一步
  function prevStep() {
    if (currentStepIndex.value > 0) {
      currentStepIndex.value--
    }
  }

  // 完成引导
  function completeOnboarding() {
    isActive.value = false
    markCompleted()
  }

  // 重置引导（用于测试或重新展示）
  function resetOnboarding() {
    isCompleted.value = false
    localStorage.removeItem(STORAGE_KEY)
  }

  const currentStep = computed(() => {
    return null  // Will be set by the component using this composable
  })

  return {
    isActive,
    currentStepIndex,
    isCompleted: computed(() => checkCompleted()),
    startOnboarding,
    skipOnboarding,
    nextStep,
    prevStep,
    completeOnboarding,
    resetOnboarding
  }
}

// 默认引导步骤
export const defaultOnboardingSteps: OnboardingStep[] = [
  {
    id: 'welcome',
    title: '欢迎使用 CAELab',
    description: '这是您的智能有限元分析工作台。让我带您快速了解一下主要功能。',
    position: 'center',
  },
  {
    id: 'modeling',
    title: '几何建模',
    description: '在建模模块中，您可以创建几何体、设置材料属性、定义边界条件。支持 CSG 布尔运算和 STEP 文件导入。',
    position: 'right',
    target: '.nav-item[data-route="/modeling"]'
  },
  {
    id: 'simulation',
    title: '仿真分析',
    description: '选择分析类型（静力学、模态、屈曲等），生成网格，运行仿真。支持多物理场耦合仿真。',
    position: 'right',
    target: '.nav-item[data-route="/simulation"]'
  },
  {
    id: 'results',
    title: '后处理',
    description: '查看应力、位移、应变等结果云图。支持等值线切片、数据导出和报告生成。',
    position: 'right',
    target: '.nav-item[data-route="/postprocess"]'
  },
  {
    id: 'ai-assistant',
    title: 'AI 助手',
    description: '使用自然语言描述您的需求，AI 将自动帮您设置边界条件、分析结果或编写代码。按 Ctrl+K 快速唤起。',
    position: 'bottom',
  },
  {
    id: 'notes',
    title: '智能笔记',
    description: '在笔记模块中记录仿真参数、结果分析。内置 Markdown 编辑器和公式支持。',
    position: 'right',
    target: '.nav-item[data-route="/notes"]'
  },
  {
    id: 'complete',
    title: '开始使用',
    description: '您已了解 CAELab 的主要功能。现在开始您的第一个仿真项目吧！',
    position: 'center',
    action: {
      label: '开始仿真',
      handler: () => {
        // Navigate to simulation
      }
    }
  }
]