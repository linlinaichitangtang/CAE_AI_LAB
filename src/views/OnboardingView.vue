<template>
  <div class="onboarding-page">
    <!-- Skip Button -->
    <button class="skip-btn" @click="skip">跳过</button>

    <!-- Card -->
    <div class="onboarding-card">
      <!-- Step Content -->
      <Transition name="step" mode="out-in">
        <!-- Step 1: 创建项目 -->
        <div v-if="currentStep === 0" key="step-0" class="step-content">
          <div class="step-icon">
            <svg viewBox="0 0 80 80" fill="none" xmlns="http://www.w3.org/2000/svg">
              <!-- Folder base -->
              <rect x="10" y="24" width="60" height="42" rx="4" fill="#1e3a5f" stroke="#2563EB" stroke-width="2"/>
              <!-- Folder tab -->
              <path d="M10 28V20a4 4 0 0 1 4-4h16l6 8H10z" fill="#2563EB" opacity="0.8"/>
              <!-- Plus icon -->
              <line x1="40" y1="38" x2="40" y2="56" stroke="#60a5fa" stroke-width="3" stroke-linecap="round"/>
              <line x1="31" y1="47" x2="49" y2="47" stroke="#60a5fa" stroke-width="3" stroke-linecap="round"/>
              <!-- Decorative dots -->
              <circle cx="22" cy="52" r="2" fill="#60a5fa" opacity="0.4"/>
              <circle cx="58" cy="36" r="1.5" fill="#60a5fa" opacity="0.3"/>
            </svg>
          </div>
          <div class="step-badge">步骤 1 / 3</div>
          <h2 class="step-title">创建你的第一个项目</h2>
          <p class="step-desc">
            在 CAELab 中新建项目，开始你的仿真之旅。支持结构分析、CFD、热耦合等多种仿真类型。
          </p>
        </div>

        <!-- Step 2: 导入模型 -->
        <div v-else-if="currentStep === 1" key="step-1" class="step-content">
          <div class="step-icon">
            <svg viewBox="0 0 80 80" fill="none" xmlns="http://www.w3.org/2000/svg">
              <!-- 3D Cube -->
              <!-- Back face -->
              <path d="M40 14L62 26V50L40 62L18 50V26L40 14Z" fill="#1e3a5f" stroke="#2563EB" stroke-width="2"/>
              <!-- Top face -->
              <path d="M40 14L58 24L40 34L22 24L40 14Z" fill="#2563EB" opacity="0.6"/>
              <!-- Right face -->
              <path d="M58 24L58 46L40 56L40 34L58 24Z" fill="#2563EB" opacity="0.4"/>
              <!-- Left face -->
              <path d="M22 24L22 46L40 56L40 34L22 24Z" fill="#2563EB" opacity="0.3"/>
              <!-- Edges -->
              <line x1="40" y1="34" x2="40" y2="56" stroke="#60a5fa" stroke-width="1.5" opacity="0.6"/>
              <line x1="22" y1="24" x2="40" y2="34" stroke="#60a5fa" stroke-width="1.5" opacity="0.6"/>
              <line x1="58" y1="24" x2="40" y2="34" stroke="#60a5fa" stroke-width="1.5" opacity="0.6"/>
              <!-- Import arrow -->
              <path d="M40 6L40 14" stroke="#60a5fa" stroke-width="2" stroke-linecap="round"/>
              <path d="M35 9L40 4L45 9" stroke="#60a5fa" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
          <div class="step-badge">步骤 2 / 3</div>
          <h2 class="step-title">导入模型与几何体</h2>
          <p class="step-desc">
            支持 STEP、IGES 等格式导入，也可使用内置几何体（长方体、圆柱体、球体）快速开始。
          </p>
        </div>

        <!-- Step 3: 运行仿真 -->
        <div v-else-if="currentStep === 2" key="step-2" class="step-content">
          <div class="step-icon">
            <svg viewBox="0 0 80 80" fill="none" xmlns="http://www.w3.org/2000/svg">
              <!-- Gear / settings circle -->
              <circle cx="40" cy="40" r="24" fill="#1e3a5f" stroke="#2563EB" stroke-width="2"/>
              <circle cx="40" cy="40" r="16" fill="#0f172a" stroke="#2563EB" stroke-width="1.5" opacity="0.8"/>
              <!-- Gear teeth -->
              <rect x="37" y="12" width="6" height="8" rx="2" fill="#2563EB" opacity="0.7"/>
              <rect x="37" y="60" width="6" height="8" rx="2" fill="#2563EB" opacity="0.7"/>
              <rect x="12" y="37" width="8" height="6" rx="2" fill="#2563EB" opacity="0.7"/>
              <rect x="60" y="37" width="8" height="6" rx="2" fill="#2563EB" opacity="0.7"/>
              <rect x="18" y="20" width="6" height="8" rx="2" fill="#2563EB" opacity="0.5" transform="rotate(45 21 24)"/>
              <rect x="56" y="52" width="6" height="8" rx="2" fill="#2563EB" opacity="0.5" transform="rotate(45 59 56)"/>
              <rect x="56" y="20" width="6" height="8" rx="2" fill="#2563EB" opacity="0.5" transform="rotate(-45 59 24)"/>
              <rect x="18" y="52" width="6" height="8" rx="2" fill="#2563EB" opacity="0.5" transform="rotate(-45 21 56)"/>
              <!-- Play button -->
              <path d="M35 32L50 40L35 48V32Z" fill="#60a5fa"/>
              <!-- Completion checkmark (appears on complete) -->
              <Transition name="check">
                <g v-if="showCheckmark">
                  <circle cx="40" cy="40" r="30" fill="#2563EB" opacity="0.15"/>
                  <path d="M28 40L36 48L54 30" stroke="#22c55e" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
                </g>
              </Transition>
            </svg>
          </div>
          <div class="step-badge">步骤 3 / 3</div>
          <h2 class="step-title">运行仿真求解</h2>
          <p class="step-desc">
            选择求解器和分析类型，一键提交仿真。支持 CalculiX 结构分析和 OpenFOAM 流体仿真。
          </p>
        </div>
      </Transition>

      <!-- Progress Dots -->
      <div class="progress-dots">
        <span
          v-for="i in 3"
          :key="i"
          class="dot"
          :class="{ active: currentStep === i - 1 }"
        ></span>
      </div>

      <!-- Navigation Buttons -->
      <div class="nav-buttons">
        <button
          v-if="currentStep > 0"
          class="nav-btn nav-prev"
          @click="prevStep"
        >
          上一步
        </button>
        <div v-else></div>
        <button
          class="nav-btn nav-next"
          @click="nextStep"
        >
          {{ currentStep === 2 ? '完成' : '下一步' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const currentStep = ref(0)
const showCheckmark = ref(false)

function skip() {
  localStorage.setItem('caelab-onboarding-done', 'true')
  router.push('/')
}

function complete() {
  localStorage.setItem('caelab-onboarding-done', 'true')
  router.push('/')
}

function nextStep() {
  if (currentStep.value === 2) {
    // Last step: show checkmark then complete
    showCheckmark.value = true
    setTimeout(() => {
      complete()
    }, 600)
  } else {
    currentStep.value++
  }
}

function prevStep() {
  if (currentStep.value > 0) {
    currentStep.value--
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    skip()
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<style scoped>
.onboarding-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
  padding: 24px;
  position: relative;
}

/* Skip Button */
.skip-btn {
  position: absolute;
  top: 24px;
  left: 24px;
  background: none;
  border: 1px solid rgba(148, 163, 184, 0.3);
  color: #94a3b8;
  padding: 8px 20px;
  border-radius: 8px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s ease;
  z-index: 10;
}

.skip-btn:hover {
  color: #e2e8f0;
  border-color: rgba(148, 163, 184, 0.6);
  background: rgba(148, 163, 184, 0.08);
}

/* Card */
.onboarding-card {
  width: 100%;
  max-width: 520px;
  background: rgba(30, 41, 59, 0.85);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(148, 163, 184, 0.12);
  border-radius: 20px;
  box-shadow:
    0 20px 60px rgba(0, 0, 0, 0.4),
    0 0 0 1px rgba(255, 255, 255, 0.03) inset;
  padding: 48px 40px 36px;
  display: flex;
  flex-direction: column;
  align-items: center;
  min-height: 480px;
}

/* Step Content */
.step-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  flex: 1;
  justify-content: center;
  padding: 8px 0;
}

/* Step Icon */
.step-icon {
  width: 120px;
  height: 120px;
  margin-bottom: 28px;
  position: relative;
}

.step-icon svg {
  width: 100%;
  height: 100%;
}

/* Step Badge */
.step-badge {
  display: inline-block;
  font-size: 12px;
  font-weight: 600;
  color: #60a5fa;
  background: rgba(37, 99, 235, 0.12);
  border: 1px solid rgba(37, 99, 235, 0.2);
  padding: 4px 14px;
  border-radius: 20px;
  margin-bottom: 16px;
  letter-spacing: 0.5px;
}

/* Step Title */
.step-title {
  font-size: 22px;
  font-weight: 700;
  color: #f1f5f9;
  margin: 0 0 12px;
  letter-spacing: -0.3px;
}

/* Step Description */
.step-desc {
  font-size: 14px;
  line-height: 1.7;
  color: #94a3b8;
  margin: 0;
  max-width: 380px;
}

/* Progress Dots */
.progress-dots {
  display: flex;
  gap: 10px;
  margin-top: 36px;
  margin-bottom: 28px;
}

.dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: rgba(148, 163, 184, 0.25);
  transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

.dot.active {
  background: #2563EB;
  box-shadow: 0 0 10px rgba(37, 99, 235, 0.5);
  transform: scale(1.2);
}

/* Navigation Buttons */
.nav-buttons {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  gap: 12px;
}

.nav-btn {
  padding: 10px 28px;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  border: none;
}

.nav-prev {
  background: rgba(148, 163, 184, 0.1);
  border: 1px solid rgba(148, 163, 184, 0.2);
  color: #94a3b8;
}

.nav-prev:hover {
  background: rgba(148, 163, 184, 0.18);
  color: #cbd5e1;
  border-color: rgba(148, 163, 184, 0.35);
}

.nav-next {
  background: linear-gradient(135deg, #2563EB, #3b82f6);
  color: #ffffff;
  box-shadow: 0 4px 14px rgba(37, 99, 235, 0.35);
}

.nav-next:hover {
  box-shadow: 0 6px 20px rgba(37, 99, 235, 0.5);
  transform: translateY(-1px);
}

.nav-next:active {
  transform: translateY(0);
}

/* Step Transition */
.step-enter-active {
  transition: all 0.35s cubic-bezier(0.16, 1, 0.3, 1);
}

.step-leave-active {
  transition: all 0.2s cubic-bezier(0.4, 0, 1, 1);
}

.step-enter-from {
  opacity: 0;
  transform: translateX(30px);
}

.step-leave-to {
  opacity: 0;
  transform: translateX(-30px);
}

/* Checkmark Transition */
.check-enter-active {
  transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}

.check-leave-active {
  transition: all 0.2s ease;
}

.check-enter-from {
  opacity: 0;
  transform: scale(0.5);
}

.check-enter-to {
  opacity: 1;
  transform: scale(1);
}

.check-leave-to {
  opacity: 0;
  transform: scale(1.2);
}

/* Responsive */
@media (max-width: 480px) {
  .onboarding-card {
    padding: 36px 24px 28px;
    border-radius: 16px;
    min-height: 420px;
  }

  .step-icon {
    width: 96px;
    height: 96px;
    margin-bottom: 20px;
  }

  .step-title {
    font-size: 19px;
  }

  .step-desc {
    font-size: 13px;
    line-height: 1.6;
  }

  .skip-btn {
    top: 16px;
    left: 16px;
    padding: 6px 16px;
    font-size: 13px;
  }

  .nav-btn {
    padding: 10px 22px;
    font-size: 13px;
  }

  .progress-dots {
    margin-top: 28px;
    margin-bottom: 22px;
  }
}
</style>
