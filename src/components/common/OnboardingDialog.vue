<template>
  <Teleport to="body">
    <Transition name="onboarding">
      <div v-if="visible && isActive" class="onboarding-overlay">
        <!-- Highlight target element -->
        <div v-if="currentStep?.target" class="onboarding-highlight" :style="highlightStyle">
        </div>

        <!-- Tooltip card -->
        <div
          class="onboarding-card glass"
          :class="positionClass"
          :style="cardStyle"
        >
          <!-- Progress dots -->
          <div class="progress-dots">
            <span
              v-for="(_, index) in steps"
              :key="index"
              class="dot"
              :class="{ active: index === currentStepIndex, completed: index < currentStepIndex }"
            ></span>
          </div>

          <!-- Content -->
          <div class="card-content">
            <h3 class="step-title">{{ currentStep?.title }}</h3>
            <p class="step-description">{{ currentStep?.description }}</p>

            <!-- Optional image placeholder -->
            <div v-if="currentStep?.image" class="step-image">
              <img :src="currentStep.image" :alt="currentStep.title" />
            </div>
          </div>

          <!-- Actions -->
          <div class="card-actions">
            <button v-if="currentStepIndex > 0" class="btn-ghost" @click="$emit('prev')">
              上一步
            </button>
            <button class="btn-ghost skip-btn" @click="$emit('skip')">
              跳过引导
            </button>
            <button v-if="currentStepIndex < steps.length - 1" class="btn-primary" @click="$emit('next')">
              下一步
            </button>
            <button v-else class="btn-premium" @click="$emit('complete')">
              开始使用
            </button>
          </div>
        </div>

        <!-- Skip hint -->
        <div class="skip-hint">
          按 <span class="kbd">Esc</span> 跳过引导
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import type { OnboardingStep } from '@/composables/useOnboarding'

const props = defineProps<{
  visible: boolean
  isActive: boolean
  steps: OnboardingStep[]
  currentStepIndex: number
}>()

const emit = defineEmits<{
  next: []
  prev: []
  skip: []
  complete: []
}>()

const currentStep = computed(() => props.steps[props.currentStepIndex])

const positionClass = computed(() => {
  const pos = currentStep.value?.position || 'center'
  return `position-${pos}`
})

const cardStyle = computed(() => {
  // Dynamic positioning based on step position
  const pos = currentStep.value?.position || 'center'
  const styles: Record<string, string> = {}

  if (pos === 'center') {
    styles.top = '50%'
    styles.left = '50%'
    styles.transform = 'translate(-50%, -50%)'
  } else if (pos === 'bottom') {
    styles.bottom = '24px'
    styles.left = '50%'
    styles.transform = 'translateX(-50%)'
  } else if (pos === 'right') {
    styles.top = '50%'
    styles.right = '24px'
    styles.transform = 'translateY(-50%)'
  }

  return styles
})

const highlightStyle = computed(() => {
  const target = currentStep.value?.target
  if (!target) return {}

  // Find the target element and get its position
  try {
    const el = document.querySelector(target)
    if (el) {
      const rect = el.getBoundingClientRect()
      return {
        top: `${rect.top - 8}px`,
        left: `${rect.left - 8}px`,
        width: `${rect.width + 16}px`,
        height: `${rect.height + 16}px`
      }
    }
  } catch {
    // Ignore selector errors
  }

  return {}
})

// Keyboard handler for Escape
function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    emit('skip')
  }
}

watch(() => props.isActive, (active) => {
  if (active) {
    document.addEventListener('keydown', handleKeydown)
  } else {
    document.removeEventListener('keydown', handleKeydown)
  }
})
</script>

<style scoped>
.onboarding-overlay {
  position: fixed;
  inset: 0;
  z-index: 9999;
  pointer-events: none;
}

.onboarding-highlight {
  position: fixed;
  border: 2px solid var(--primary);
  border-radius: var(--radius-lg);
  box-shadow: 0 0 0 4px var(--primary-glow);
  pointer-events: none;
  animation: pulse-highlight 2s infinite;
}

@keyframes pulse-highlight {
  0%, 100% {
    box-shadow: 0 0 0 4px var(--primary-glow);
  }
  50% {
    box-shadow: 0 0 0 8px rgba(37, 99, 235, 0.2);
  }
}

.onboarding-card {
  position: fixed;
  width: 360px;
  padding: 20px;
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-lg);
  pointer-events: auto;
}

.onboarding-card.position-center {
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.onboarding-card.position-bottom {
  bottom: 24px;
  left: 50%;
  transform: translateX(-50%);
}

.onboarding-card.position-right {
  top: 50%;
  right: 24px;
  transform: translateY(-50%);
}

.progress-dots {
  display: flex;
  justify-content: center;
  gap: 8px;
  margin-bottom: 16px;
}

.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--border-default);
  transition: all var(--duration-fast);
}

.dot.active {
  width: 24px;
  border-radius: 4px;
  background: var(--primary);
}

.dot.completed {
  background: var(--accent-green);
}

.card-content {
  margin-bottom: 16px;
}

.step-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 8px;
}

.step-description {
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.6;
}

.step-image {
  margin-top: 12px;
  border-radius: var(--radius-md);
  overflow: hidden;
  background: var(--bg-elevated);
}

.step-image img {
  width: 100%;
  height: auto;
}

.card-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

.skip-btn {
  color: var(--text-muted);
}

.skip-hint {
  position: fixed;
  bottom: 24px;
  left: 50%;
  transform: translateX(-50%);
  font-size: 12px;
  color: var(--text-muted);
  pointer-events: none;
}

/* Animations */
.onboarding-enter-active {
  transition: all var(--duration-normal) var(--ease-out);
}

.onboarding-leave-active {
  transition: all var(--duration-fast) var(--ease-out);
}

.onboarding-enter-from,
.onboarding-leave-to {
  opacity: 0;
}

.onboarding-enter-from .onboarding-card {
  transform: scale(0.9);
}

.onboarding-leave-to .onboarding-card {
  transform: scale(0.95);
}
</style>