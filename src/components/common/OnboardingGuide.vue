<template>
  <Teleport to="body">
    <div class="onboarding-overlay" @click.self="handleSkip">
      <div class="onboarding-container">
        <!-- Progress Bar -->
        <div class="onboarding-progress">
          <div
            v-for="i in totalSteps"
            :key="i"
            :class="['progress-dot', { active: i === currentStep, completed: i < currentStep }]"
          ></div>
        </div>

        <!-- Step Content -->
        <Transition name="slide" mode="out-in">
          <div :key="currentStep" class="onboarding-content">
            <!-- Step Indicator -->
            <div class="step-badge">
              {{ currentStep }} / {{ totalSteps }}
            </div>

            <!-- Illustration -->
            <div class="step-illustration">
              <div :class="['illustration-card', steps[currentStep - 1].color]">
                <div class="illustration-icon">{{ steps[currentStep - 1].icon }}</div>
                <div class="illustration-detail">
                  <div v-for="line in steps[currentStep - 1].detail" :key="line" class="detail-line"></div>
                </div>
              </div>
            </div>

            <!-- Text -->
            <h2 class="step-title">{{ steps[currentStep - 1].title }}</h2>
            <p class="step-description">{{ steps[currentStep - 1].description }}</p>

            <!-- Tips -->
            <div class="step-tips">
              <div v-for="tip in steps[currentStep - 1].tips" :key="tip" class="tip-item">
                <svg class="w-4 h-4 text-[var(--primary)] flex-shrink-0 mt-0.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="20 6 9 17 4 12"/>
                </svg>
                <span>{{ tip }}</span>
              </div>
            </div>
          </div>
        </Transition>

        <!-- Actions -->
        <div class="onboarding-actions">
          <button
            v-if="currentStep > 1"
            @click="prevStep"
            class="btn btn-secondary"
          >
            上一步
          </button>
          <div class="flex-1"></div>
          <button
            @click="handleSkip"
            class="btn btn-ghost"
          >
            跳过引导
          </button>
          <button
            v-if="currentStep < totalSteps"
            @click="nextStep"
            class="btn btn-primary"
          >
            下一步
          </button>
          <button
            v-else
            @click="handleFinish"
            class="btn btn-primary"
          >
            开始使用
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const emit = defineEmits<{
  close: []
}>()

const ONBOARDING_KEY = 'caelab-onboarding-completed'
const totalSteps = 4
const currentStep = ref(1)

const steps = [
  {
    title: '欢迎使用 CAELab',
    description: 'CAELab 是一个科研与工程创作一体化工作台，提供从建模到仿真分析的完整工作流程。让我们快速了解核心功能。',
    icon: '🏗️',
    color: 'color-blue',
    detail: ['line1', 'line2', 'line3'],
    tips: [
      '支持结构分析、模态分析、热传导等多种仿真类型',
      '内置丰富的材料库和标准算例',
      '提供 AI 辅助分析功能'
    ]
  },
  {
    title: '创建项目与生成网格',
    description: '在首页创建新项目后，进入仿真模块生成网格。CAELab 支持 2D/3D 结构化网格，提供 11 种单元类型选择。',
    icon: '🔲',
    color: 'color-green',
    detail: ['line1', 'line2', 'line3'],
    tips: [
      '支持 S4、C3D8 等常用单元类型',
      '可进行局部网格加密以提升关键区域精度',
      '内置网格质量检查工具'
    ]
  },
  {
    title: '设置材料与边界条件',
    description: '为模型定义材料属性和边界条件。CAELab 提供内置材料库，也支持自定义材料参数。',
    icon: '⚙️',
    color: 'color-orange',
    detail: ['line1', 'line2', 'line3'],
    tips: [
      '从材料库快速选择常用材料（钢、铝等）',
      '支持线弹性、弹塑性、超弹性等多种本构模型',
      '灵活设置固定约束、集中力、均布载荷等边界条件'
    ]
  },
  {
    title: '运行求解与查看结果',
    description: '设置完成后，选择分析类型并运行求解器。求解完成后可在 3D 视图中查看应力云图、变形图等结果。',
    icon: '📊',
    color: 'color-purple',
    detail: ['line1', 'line2', 'line3'],
    tips: [
      '实时查看求解进度和计算日志',
      '支持 Von Mises 应力、位移等多种结果显示',
      '可导出 HTML 和 PDF 格式的仿真报告'
    ]
  }
]

onMounted(() => {
  // Auto-show on first visit
  const completed = localStorage.getItem(ONBOARDING_KEY)
  if (completed) {
    emit('close')
  }
})

function nextStep() {
  if (currentStep.value < totalSteps) {
    currentStep.value++
  }
}

function prevStep() {
  if (currentStep.value > 1) {
    currentStep.value--
  }
}

function handleSkip() {
  localStorage.setItem(ONBOARDING_KEY, 'true')
  emit('close')
}

function handleFinish() {
  localStorage.setItem(ONBOARDING_KEY, 'true')
  emit('close')
}
</script>

<style scoped>
.onboarding-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.onboarding-container {
  width: 520px;
  max-height: 90vh;
  background: var(--bg-surface, #ffffff);
  border-radius: 16px;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.onboarding-progress {
  display: flex;
  justify-content: center;
  gap: 8px;
  padding: 20px 24px 0;
}

.progress-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--border-default, #e2e8f0);
  transition: all 0.3s ease;
}

.progress-dot.active {
  width: 24px;
  border-radius: 4px;
  background: var(--primary, #4f46e5);
}

.progress-dot.completed {
  background: var(--primary, #4f46e5);
  opacity: 0.5;
}

.onboarding-content {
  padding: 24px 32px;
  text-align: center;
}

.step-badge {
  display: inline-block;
  padding: 2px 10px;
  border-radius: 999px;
  font-size: 12px;
  font-weight: 500;
  background: var(--bg-elevated, #f1f5f9);
  color: var(--text-muted, #64748b);
  margin-bottom: 20px;
}

.step-illustration {
  margin-bottom: 24px;
}

.illustration-card {
  width: 100%;
  height: 140px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  position: relative;
  overflow: hidden;
}

.illustration-card.color-blue {
  background: linear-gradient(135deg, #dbeafe, #bfdbfe);
}

.illustration-card.color-green {
  background: linear-gradient(135deg, #dcfce7, #bbf7d0);
}

.illustration-card.color-orange {
  background: linear-gradient(135deg, #ffedd5, #fed7aa);
}

.illustration-card.color-purple {
  background: linear-gradient(135deg, #ede9fe, #ddd6fe);
}

.illustration-icon {
  font-size: 48px;
}

.illustration-detail {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.detail-line {
  height: 8px;
  border-radius: 4px;
  background: rgba(255, 255, 255, 0.7);
}

.detail-line:nth-child(1) { width: 120px; }
.detail-line:nth-child(2) { width: 90px; }
.detail-line:nth-child(3) { width: 140px; }

.step-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary, #0f172a);
  margin-bottom: 8px;
}

.step-description {
  font-size: 14px;
  line-height: 1.6;
  color: var(--text-secondary, #475569);
  margin-bottom: 20px;
}

.step-tips {
  text-align: left;
  background: var(--bg-elevated, #f8fafc);
  border-radius: 8px;
  padding: 12px 16px;
  border: 1px solid var(--border-subtle, #f1f5f9);
}

.tip-item {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  font-size: 13px;
  color: var(--text-secondary, #475569);
  line-height: 1.5;
}

.tip-item + .tip-item {
  margin-top: 6px;
}

.onboarding-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 16px 32px 24px;
}

.btn {
  padding: 8px 16px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  border: none;
  transition: all 0.15s ease;
}

.btn-primary {
  background: var(--primary, #4f46e5);
  color: white;
}

.btn-primary:hover {
  background: var(--primary-hover, #4338ca);
}

.btn-secondary {
  background: var(--bg-elevated, #f1f5f9);
  color: var(--text-primary, #0f172a);
}

.btn-secondary:hover {
  background: var(--bg-hover, #e2e8f0);
}

.btn-ghost {
  background: transparent;
  color: var(--text-muted, #64748b);
}

.btn-ghost:hover {
  color: var(--text-secondary, #475569);
  background: var(--bg-elevated, #f8fafc);
}

/* Slide transition */
.slide-enter-active,
.slide-leave-active {
  transition: all 0.25s ease;
}

.slide-enter-from {
  opacity: 0;
  transform: translateX(20px);
}

.slide-leave-to {
  opacity: 0;
  transform: translateX(-20px);
}
</style>
