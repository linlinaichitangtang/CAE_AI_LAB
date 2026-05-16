<template>
  <div class="simulation-wizard">
    <!-- ========== 阶段 1：模板选择 ========== -->
    <div v-if="state.phase === 'select'" class="template-selection">
      <div class="wizard-header">
        <h1 class="wizard-title">仿真向导</h1>
        <p class="wizard-subtitle">
          选择经典案例，跟随 step-by-step 引导完成你的第一次仿真分析
        </p>
      </div>

      <!-- 筛选标签 -->
      <div class="filter-tabs">
        <button
          v-for="tab in filterTabs"
          :key="tab.key"
          class="filter-tab"
          :class="{ active: activeFilter === tab.key }"
          @click="activeFilter = tab.key"
        >
          {{ tab.label }}
        </button>
      </div>

      <!-- 模板卡片网格 -->
      <div class="template-grid">
        <div
          v-for="template in filteredTemplates"
          :key="template.id"
          class="template-card"
          @click="selectTemplate(template)"
        >
          <div class="card-header">
            <span class="difficulty-badge" :class="template.difficulty">
              {{ difficultyLabel(template.difficulty) }}
            </span>
            <span class="time-estimate">⏱️ {{ template.estimatedTime }}分钟</span>
          </div>
          <h3 class="card-title">{{ template.name }}</h3>
          <p class="card-description">{{ template.description }}</p>
          <div class="card-tags">
            <span v-for="tag in template.tags" :key="tag" class="tag">{{ tag }}</span>
          </div>
          <div class="card-footer">
            <span class="step-count">{{ template.steps.length }} 个步骤</span>
            <button class="start-btn">开始</button>
          </div>
        </div>
      </div>
    </div>

    <!-- ========== 阶段 2：向导流程 ========== -->
    <div v-else-if="state.phase === 'wizard' && template" class="wizard-flow">
      <!-- 顶部进度条 -->
      <div class="wizard-progress-bar">
        <div class="progress-track">
          <div class="progress-fill" :style="{ width: progress + '%' }" />
        </div>
        <div class="step-indicators">
          <div
            v-for="(step, index) in template.steps"
            :key="step.id"
            class="step-dot"
            :class="{
              completed: index < currentStepIndex,
              active: index === currentStepIndex,
              pending: index > currentStepIndex
            }"
            @click="goToStep(index)"
          >
            <span v-if="index < currentStepIndex">✓</span>
            <span v-else>{{ index + 1 }}</span>
          </div>
        </div>
        <div class="step-labels">
          <span
            v-for="(step, index) in template.steps"
            :key="step.id"
            class="step-label"
            :class="{ active: index === currentStepIndex }"
          >
            {{ step.title }}
          </span>
        </div>
      </div>

      <!-- 当前步骤内容 -->
      <div v-if="currentStep" class="step-content">
        <div class="step-header">
          <span class="step-icon">{{ currentStep.icon }}</span>
          <h2 class="step-title">{{ currentStep.title }}</h2>
          <span class="step-counter">{{ currentStepIndex + 1 }} / {{ totalSteps }}</span>
        </div>

        <p class="step-description">{{ currentStep.description }}</p>

        <!-- 指导信息区 -->
        <div v-if="currentStep.guidance" class="guidance-box">
          <div class="guidance-title">💡 指导</div>
          <p class="guidance-text">{{ currentStep.guidance }}</p>
        </div>

        <!-- 步骤主体内容（根据 component 类型渲染） -->
        <div class="step-body">
          <WizardStepContent
            v-if="currentStep"
            :component="currentStep.component"
            :template="template"
            :step-id="currentStep.id"
            :step-data="getStepData(currentStep?.id ?? '')"
            @update="(data: Record<string, unknown>) => currentStep && saveStepData(currentStep.id, data)"
          />
        </div>

        <!-- 技巧提示 -->
        <div v-if="currentStep.tips && currentStep.tips.length > 0" class="tips-box">
          <div class="tips-title">📌 小技巧</div>
          <ul class="tips-list">
            <li v-for="(tip, i) in currentStep.tips" :key="i">{{ tip }}</li>
          </ul>
        </div>
      </div>

      <!-- 底部导航 -->
      <div class="wizard-nav">
        <button
          class="nav-btn secondary"
          :disabled="isFirstStep"
          @click="prevStep"
        >
          ← 上一步
        </button>

        <button
          v-if="!isLastStep"
          class="nav-btn primary"
          :disabled="!canGoNext"
          @click="nextStep"
        >
          下一步 →
        </button>
        <button
          v-else
          class="nav-btn success"
          @click="completeWizard"
        >
          ✓ 完成向导
        </button>
      </div>

      <!-- 计时器 -->
      <div class="elapsed-time">
        已用时: {{ formatTime(elapsedTime) }}
      </div>
    </div>

    <!-- ========== 阶段 3：完成 ========== -->
    <div v-else-if="state.phase === 'complete' && template" class="wizard-complete">
      <div class="complete-icon">🎉</div>
      <h2 class="complete-title">恭喜完成！</h2>
      <p class="complete-message">
        你已完成「{{ template.name }}」仿真向导，用时 {{ formatTime(elapsedTime) }}
      </p>

      <div class="complete-summary">
        <div class="summary-item">
          <span class="summary-label">完成步骤</span>
          <span class="summary-value">{{ totalSteps }} / {{ totalSteps }}</span>
        </div>
        <div class="summary-item">
          <span class="summary-label">预估时间</span>
          <span class="summary-value">{{ template.estimatedTime }} 分钟</span>
        </div>
        <div class="summary-item">
          <span class="summary-label">实际用时</span>
          <span class="summary-value">{{ formatTime(elapsedTime) }}</span>
        </div>
      </div>

      <div class="complete-actions">
        <button class="action-btn primary" @click="handleStartSimulation">
          ▶️ 运行仿真
        </button>
        <button class="action-btn secondary" @click="backToSelection">
          📋 选择其他案例
        </button>
        <button class="action-btn secondary" @click="resetWizard">
          🔄 重新开始
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useSimulationWizard } from '../../composables/useSimulationWizard'
import {
  allTemplates,
  getTemplatesByCategory,
  getRecommendedTemplates
} from '../../data/templates/simulationTemplates'
import WizardStepContent from './WizardStepContent.vue'

const router = useRouter()

const emit = defineEmits<{
  'start-simulation': []
}>()

function handleStartSimulation() {
  emit('start-simulation')
  router.push('/simulation')
}

const {
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
} = useSimulationWizard()

const template = computed(() => state.value.template)

// 筛选
const activeFilter = ref<string>('all')
const filterTabs = [
  { key: 'all', label: '全部' },
  { key: 'recommended', label: '新手推荐' },
  { key: 'structural', label: '结构力学' },
  { key: 'beginner', label: '入门' },
  { key: 'intermediate', label: '进阶' }
]

const filteredTemplates = computed(() => {
  switch (activeFilter.value) {
    case 'recommended': return getRecommendedTemplates()
    case 'beginner': return allTemplates.filter(t => t.difficulty === 'beginner')
    case 'intermediate': return allTemplates.filter(t => t.difficulty === 'intermediate')
    case 'structural': return getTemplatesByCategory('structural')
    default: return allTemplates
  }
})

function difficultyLabel(d: string): string {
  const map: Record<string, string> = {
    beginner: '入门',
    intermediate: '进阶',
    advanced: '高级'
  }
  return map[d] ?? d
}

function formatTime(seconds: number): string {
  const m = Math.floor(seconds / 60)
  const s = seconds % 60
  return m > 0 ? `${m}分${s}秒` : `${s}秒`
}
</script>

<style scoped>
.simulation-wizard {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  background: var(--bg-primary, #f8fafc);
}

/* ===== 模板选择 ===== */
.template-selection {
  max-width: 1200px;
  margin: 0 auto;
  padding: 32px 24px;
}

.wizard-header {
  text-align: center;
  margin-bottom: 32px;
}

.wizard-title {
  font-size: 32px;
  font-weight: 700;
  color: var(--text-primary, #1e293b);
  margin-bottom: 8px;
}

.wizard-subtitle {
  font-size: 16px;
  color: var(--text-secondary, #64748b);
}

.filter-tabs {
  display: flex;
  gap: 8px;
  margin-bottom: 24px;
  flex-wrap: wrap;
}

.filter-tab {
  padding: 8px 16px;
  border-radius: 20px;
  border: 1px solid var(--border-color, #e2e8f0);
  background: white;
  color: var(--text-secondary, #64748b);
  cursor: pointer;
  transition: all 0.2s;
}

.filter-tab.active {
  background: var(--primary-color, #3b82f6);
  color: white;
  border-color: var(--primary-color, #3b82f6);
}

.template-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 20px;
}

.template-card {
  background: white;
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.08);
  cursor: pointer;
  transition: all 0.2s;
  border: 2px solid transparent;
}

.template-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0,0,0,0.12);
  border-color: var(--primary-color, #3b82f6);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.difficulty-badge {
  padding: 4px 10px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 600;
}

.difficulty-badge.beginner {
  background: #dcfce7;
  color: #166534;
}

.difficulty-badge.intermediate {
  background: #fef3c7;
  color: #92400e;
}

.difficulty-badge.advanced {
  background: #fee2e2;
  color: #991b1b;
}

.time-estimate {
  font-size: 12px;
  color: var(--text-secondary, #64748b);
}

.card-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
  margin-bottom: 8px;
}

.card-description {
  font-size: 14px;
  color: var(--text-secondary, #64748b);
  line-height: 1.5;
  margin-bottom: 12px;
}

.card-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-bottom: 16px;
}

.tag {
  padding: 2px 8px;
  border-radius: 4px;
  background: #f1f5f9;
  font-size: 12px;
  color: var(--text-secondary, #64748b);
}

.card-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.step-count {
  font-size: 13px;
  color: var(--text-secondary, #64748b);
}

.start-btn {
  padding: 8px 20px;
  border-radius: 8px;
  background: var(--primary-color, #3b82f6);
  color: white;
  border: none;
  font-weight: 600;
  cursor: pointer;
}

/* ===== 向导流程 ===== */
.wizard-flow {
  max-width: 900px;
  margin: 0 auto;
  padding: 24px;
}

.wizard-progress-bar {
  margin-bottom: 32px;
}

.progress-track {
  height: 4px;
  background: #e2e8f0;
  border-radius: 2px;
  margin-bottom: 12px;
}

.progress-fill {
  height: 100%;
  background: var(--primary-color, #3b82f6);
  border-radius: 2px;
  transition: width 0.3s;
}

.step-indicators {
  display: flex;
  justify-content: space-between;
  position: relative;
}

.step-dot {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.step-dot.completed {
  background: #22c55e;
  color: white;
}

.step-dot.active {
  background: var(--primary-color, #3b82f6);
  color: white;
  box-shadow: 0 0 0 4px rgba(59,130,246,0.2);
}

.step-dot.pending {
  background: #e2e8f0;
  color: #94a3b8;
}

.step-labels {
  display: flex;
  justify-content: space-between;
  margin-top: 8px;
}

.step-label {
  font-size: 12px;
  color: var(--text-secondary, #64748b);
  text-align: center;
  flex: 1;
}

.step-label.active {
  color: var(--primary-color, #3b82f6);
  font-weight: 600;
}

.step-content {
  background: white;
  border-radius: 12px;
  padding: 28px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.08);
  margin-bottom: 20px;
}

.step-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.step-icon {
  font-size: 24px;
}

.step-title {
  font-size: 22px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
  flex: 1;
}

.step-counter {
  font-size: 14px;
  color: var(--text-secondary, #64748b);
}

.step-description {
  font-size: 15px;
  color: var(--text-secondary, #64748b);
  line-height: 1.6;
  margin-bottom: 20px;
}

.guidance-box {
  background: #eff6ff;
  border-left: 4px solid var(--primary-color, #3b82f6);
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 20px;
}

.guidance-title {
  font-weight: 600;
  color: var(--primary-color, #3b82f6);
  margin-bottom: 8px;
}

.guidance-text {
  font-size: 14px;
  color: var(--text-secondary, #475569);
  line-height: 1.6;
}

.step-body {
  margin-bottom: 20px;
}

.tips-box {
  background: #fefce8;
  border-radius: 8px;
  padding: 16px;
}

.tips-title {
  font-weight: 600;
  color: #a16207;
  margin-bottom: 8px;
}

.tips-list {
  margin: 0;
  padding-left: 20px;
  color: var(--text-secondary, #475569);
  font-size: 14px;
  line-height: 1.8;
}

.wizard-nav {
  display: flex;
  justify-content: space-between;
  gap: 12px;
}

.nav-btn {
  padding: 12px 24px;
  border-radius: 8px;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  border: none;
  transition: all 0.2s;
}

.nav-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.nav-btn.primary {
  background: var(--primary-color, #3b82f6);
  color: white;
}

.nav-btn.success {
  background: #22c55e;
  color: white;
}

.nav-btn.secondary {
  background: #f1f5f9;
  color: var(--text-primary, #1e293b);
}

.elapsed-time {
  text-align: center;
  margin-top: 16px;
  font-size: 13px;
  color: var(--text-secondary, #94a3b8);
}

/* ===== 完成页面 ===== */
.wizard-complete {
  max-width: 600px;
  margin: 0 auto;
  padding: 60px 24px;
  text-align: center;
}

.complete-icon {
  font-size: 64px;
  margin-bottom: 16px;
}

.complete-title {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary, #1e293b);
  margin-bottom: 12px;
}

.complete-message {
  font-size: 16px;
  color: var(--text-secondary, #64748b);
  margin-bottom: 32px;
}

.complete-summary {
  display: flex;
  justify-content: center;
  gap: 32px;
  margin-bottom: 32px;
}

.summary-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.summary-label {
  font-size: 13px;
  color: var(--text-secondary, #94a3b8);
}

.summary-value {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
}

.complete-actions {
  display: flex;
  flex-direction: column;
  gap: 12px;
  max-width: 300px;
  margin: 0 auto;
}

.action-btn {
  padding: 14px 24px;
  border-radius: 10px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  border: none;
  transition: all 0.2s;
}

.action-btn.primary {
  background: var(--primary-color, #3b82f6);
  color: white;
}

.action-btn.secondary {
  background: #f1f5f9;
  color: var(--text-primary, #1e293b);
}
</style>
