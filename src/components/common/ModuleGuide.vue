<template>
  <Teleport to="body">
    <Transition name="guide-fade">
      <div v-if="active && steps.length > 0" class="module-guide-overlay">
        <!-- Semi-transparent mask with cutout -->
        <div
          class="guide-mask"
          :style="maskStyle"
          @click.self="handleSkip"
        ></div>

        <!-- Tooltip -->
        <Transition name="guide-tooltip" mode="out-in">
          <div
            v-if="currentStepData"
            :key="currentStep"
            :class="['guide-tooltip', `guide-tooltip-${currentStepData.position}`]"
            :style="tooltipStyle"
          >
            <!-- Arrow -->
            <div :class="['guide-arrow', `guide-arrow-${currentStepData.position}`]"></div>

            <!-- Content -->
            <div class="guide-tooltip-content">
              <!-- Step badge -->
              <div class="guide-step-badge">
                {{ currentStep + 1 }} / {{ steps.length }}
              </div>

              <!-- Title -->
              <h3 class="guide-tooltip-title">{{ currentStepData.title }}</h3>

              <!-- Description -->
              <p class="guide-tooltip-description">{{ currentStepData.description }}</p>

              <!-- Action hint -->
              <div v-if="currentStepData.action" class="guide-action-hint">
                <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="10" />
                  <path d="M12 16v-4M12 8h.01" />
                </svg>
                <span>提示：{{ currentStepData.action }}</span>
              </div>

              <!-- Navigation -->
              <div class="guide-tooltip-actions">
                <button
                  v-if="currentStep > 0"
                  @click="prevStep"
                  class="guide-btn guide-btn-secondary"
                >
                  上一步
                </button>
                <div class="flex-1"></div>
                <button
                  @click="handleSkip"
                  class="guide-btn guide-btn-ghost"
                >
                  跳过
                </button>
                <button
                  v-if="currentStep < steps.length - 1"
                  @click="nextStep"
                  class="guide-btn guide-btn-primary"
                >
                  下一步
                </button>
                <button
                  v-else
                  @click="handleFinish"
                  class="guide-btn guide-btn-primary"
                >
                  完成
                </button>
              </div>
            </div>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue'
import type { GuideStep } from '@/utils/moduleGuides'

const props = defineProps<{
  steps: GuideStep[]
  active: boolean
  moduleKey: string
}>()

const emit = defineEmits<{
  finish: []
  skip: []
}>()

const STORAGE_PREFIX = 'caelab-guide-'
const currentStep = ref(0)
const targetRect = ref<DOMRect | null>(null)
const resizeObserver = ref<ResizeObserver | null>(null)

const currentStepData = computed(() => {
  return props.steps[currentStep.value] || null
})

// ============ 定位计算 ============

function updateTargetPosition() {
  if (!currentStepData.value) return

  const targetEl = document.querySelector(currentStepData.value.target)
  if (targetEl) {
    targetRect.value = targetEl.getBoundingClientRect()
  } else {
    // 目标元素不存在，使用屏幕中央
    targetRect.value = new DOMRect(
      window.innerWidth / 2 - 100,
      window.innerHeight / 2 - 50,
      200,
      100
    )
  }
}

// 遮罩层样式：使用 box-shadow 实现镂空效果
const maskStyle = computed(() => {
  if (!targetRect.value) return {}

  const rect = targetRect.value
  const padding = 8 // 镂空区域外扩像素

  return {
    boxShadow: `0 0 0 9999px rgba(0, 0, 0, 0.5)`,
    position: 'fixed' as const,
    top: `${rect.top - padding}px`,
    left: `${rect.left - padding}px`,
    width: `${rect.width + padding * 2}px`,
    height: `${rect.height + padding * 2}px`,
    borderRadius: '8px',
    zIndex: '8000',
    transition: 'all 0.3s ease',
  }
})

// 气泡定位
const tooltipStyle = computed(() => {
  if (!targetRect.value || !currentStepData.value) return {}

  const rect = targetRect.value
  const gap = 16
  const position = currentStepData.value.position

  const base: Record<string, string> = {
    position: 'fixed' as unknown as string,
    zIndex: '8001',
  }

  switch (position) {
    case 'bottom':
      base.top = `${rect.bottom + gap}px`
      base.left = `${rect.left + rect.width / 2}px`
      base.transform = 'translateX(-50%)'
      break
    case 'top':
      base.bottom = `${window.innerHeight - rect.top + gap}px`
      base.left = `${rect.left + rect.width / 2}px`
      base.transform = 'translateX(-50%)'
      break
    case 'right':
      base.left = `${rect.right + gap}px`
      base.top = `${rect.top + rect.height / 2}px`
      base.transform = 'translateY(-50%)'
      break
    case 'left':
      base.right = `${window.innerWidth - rect.left + gap}px`
      base.top = `${rect.top + rect.height / 2}px`
      base.transform = 'translateY(-50%)'
      break
  }

  return base
})

// ============ 步骤导航 ============

function nextStep() {
  if (currentStep.value < props.steps.length - 1) {
    currentStep.value++
    nextTick(updateTargetPosition)
  }
}

function prevStep() {
  if (currentStep.value > 0) {
    currentStep.value--
    nextTick(updateTargetPosition)
  }
}

function handleSkip() {
  markCompleted()
  emit('skip')
}

function handleFinish() {
  markCompleted()
  emit('finish')
}

// ============ 持久化 ============

function isCompleted(): boolean {
  return localStorage.getItem(`${STORAGE_PREFIX}${props.moduleKey}-completed`) === 'true'
}

function markCompleted() {
  localStorage.setItem(`${STORAGE_PREFIX}${props.moduleKey}-completed`, 'true')
}

// ============ 键盘支持 ============

function handleKeydown(e: KeyboardEvent) {
  if (!props.active) return

  if (e.key === 'Escape') {
    handleSkip()
  } else if (e.key === 'ArrowRight' || e.key === 'Enter') {
    if (currentStep.value < props.steps.length - 1) {
      nextStep()
    } else {
      handleFinish()
    }
  } else if (e.key === 'ArrowLeft') {
    prevStep()
  }
}

// ============ 窗口调整 ============

function handleResize() {
  updateTargetPosition()
}

// ============ 生命周期 ============

watch(
  () => props.active,
  (newActive) => {
    if (newActive) {
      if (isCompleted()) {
        // 已完成引导，不再显示
        emit('skip')
        return
      }
      currentStep.value = 0
      nextTick(() => {
        updateTargetPosition()
      })
    }
  }
)

watch(currentStep, () => {
  nextTick(updateTargetPosition)
})

onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
  window.addEventListener('resize', handleResize)
  window.addEventListener('scroll', handleResize, true)

  // 监听 DOM 变化以更新目标位置
  resizeObserver.value = new ResizeObserver(handleResize)
  resizeObserver.value.observe(document.body)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
  window.removeEventListener('resize', handleResize)
  window.removeEventListener('scroll', handleResize, true)
  resizeObserver.value?.disconnect()
})
</script>

<style scoped>
.guide-mask {
  pointer-events: auto;
}

.guide-tooltip {
  position: fixed;
  width: 320px;
  max-width: calc(100vw - 32px);
  background: white;
  border-radius: 12px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  overflow: visible;
}

:deep(.dark) .guide-tooltip,
.dark .guide-tooltip {
  background: #1f2937;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
}

.guide-tooltip-content {
  padding: 16px 20px;
}

.guide-step-badge {
  display: inline-block;
  padding: 2px 10px;
  border-radius: 999px;
  font-size: 11px;
  font-weight: 500;
  background: #f1f5f9;
  color: #64748b;
  margin-bottom: 8px;
}

:deep(.dark) .guide-step-badge,
.dark .guide-step-badge {
  background: #374151;
  color: #9ca3af;
}

.guide-tooltip-title {
  font-size: 15px;
  font-weight: 600;
  color: #0f172a;
  margin-bottom: 6px;
}

:deep(.dark) .guide-tooltip-title,
.dark .guide-tooltip-title {
  color: #f1f5f9;
}

.guide-tooltip-description {
  font-size: 13px;
  line-height: 1.6;
  color: #475569;
  margin-bottom: 12px;
}

:deep(.dark) .guide-tooltip-description,
.dark .guide-tooltip-description {
  color: #94a3b8;
}

.guide-action-hint {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: #6366f1;
  background: #eef2ff;
  padding: 6px 10px;
  border-radius: 6px;
  margin-bottom: 12px;
}

:deep(.dark) .guide-action-hint,
.dark .guide-action-hint {
  color: #818cf8;
  background: rgba(99, 102, 241, 0.15);
}

.guide-tooltip-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.guide-btn {
  padding: 6px 14px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  border: none;
  transition: all 0.15s ease;
}

.guide-btn-primary {
  background: #4f46e5;
  color: white;
}

.guide-btn-primary:hover {
  background: #4338ca;
}

.guide-btn-secondary {
  background: #f1f5f9;
  color: #0f172a;
}

.guide-btn-secondary:hover {
  background: #e2e8f0;
}

:deep(.dark) .guide-btn-secondary,
.dark .guide-btn-secondary {
  background: #374151;
  color: #f1f5f9;
}

.guide-btn-ghost {
  background: transparent;
  color: #64748b;
}

.guide-btn-ghost:hover {
  color: #475569;
  background: #f8fafc;
}

/* Arrows */
.guide-arrow {
  position: absolute;
  width: 12px;
  height: 12px;
  background: white;
  transform: rotate(45deg);
  border: 1px solid #e2e8f0;
}

:deep(.dark) .guide-arrow,
.dark .guide-arrow {
  background: #1f2937;
  border-color: #374151;
}

.guide-arrow-bottom {
  top: -7px;
  left: 50%;
  margin-left: -6px;
  border-right: none;
  border-bottom: none;
}

.guide-arrow-top {
  bottom: -7px;
  left: 50%;
  margin-left: -6px;
  border-left: none;
  border-top: none;
}

.guide-arrow-right {
  left: -7px;
  top: 50%;
  margin-top: -6px;
  border-top: none;
  border-right: none;
}

.guide-arrow-left {
  right: -7px;
  top: 50%;
  margin-top: -6px;
  border-bottom: none;
  border-left: none;
}

/* Transitions */
.guide-fade-enter-active {
  transition: opacity 0.3s ease;
}

.guide-fade-leave-active {
  transition: opacity 0.2s ease;
}

.guide-fade-enter-from,
.guide-fade-leave-to {
  opacity: 0;
}

.guide-tooltip-enter-active {
  transition: all 0.25s ease-out;
}

.guide-tooltip-leave-active {
  transition: all 0.15s ease-in;
}

.guide-tooltip-enter-from {
  opacity: 0;
  transform: translateX(-50%) translateY(8px) !important;
}

.guide-tooltip-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(-4px) !important;
}
</style>
