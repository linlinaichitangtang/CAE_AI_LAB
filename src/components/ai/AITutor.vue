<template>
  <Teleport to="body">
    <Transition name="tutor-slide">
      <div v-if="isOpen" class="ai-tutor-overlay" @click.self="close">
        <!-- 侧边栏容器 -->
        <div class="tutor-panel" :class="{ 'is-collapsed': isCollapsed }">
          <!-- 头部 -->
          <div class="tutor-header">
            <div class="header-left">
              <div class="avatar-container">
                <div class="avatar" :class="{ speaking: isSpeaking }">
                  🤖
                </div>
                <span class="status-dot" :class="{ active: isSpeaking }"></span>
              </div>
              <div class="header-info">
                <span class="tutor-name">AI 助教</span>
                <span class="tutor-status">{{ tutorStatusText }}</span>
              </div>
            </div>
            <div class="header-actions">
              <button class="icon-btn" @click="toggleCollapse" :title="isCollapsed ? '展开' : '收起'">
                <svg v-if="!isCollapsed" width="16" height="16" viewBox="0 0 24 24" fill="none">
                  <path d="M15 19l-7-7 7-7" stroke="currentColor" stroke-width="2"/>
                </svg>
                <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none">
                  <path d="M9 5l7 7-7 7" stroke="currentColor" stroke-width="2"/>
                </svg>
              </button>
              <button class="icon-btn" @click="close">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                  <path d="M6 6l12 12M6 18L18 6" stroke="currentColor" stroke-width="2"/>
                </svg>
              </button>
            </div>
          </div>

          <!-- 展开时显示的内容 -->
          <div v-if="!isCollapsed" class="tutor-content">
            <!-- 模式切换 -->
            <div class="mode-tabs">
              <button
                v-for="mode in tutoringModes"
                :key="mode.id"
                class="mode-tab"
                :class="{ active: currentMode === mode.id }"
                @click="switchMode(mode.id)"
              >
                <span class="mode-icon">{{ mode.icon }}</span>
                <span class="mode-label">{{ mode.label }}</span>
              </button>
            </div>

            <!-- 学习进度 -->
            <div v-if="currentLesson" class="lesson-progress">
              <div class="lesson-header">
                <span class="lesson-title">{{ currentLesson.title }}</span>
                <span class="lesson-step">{{ currentStepIndex + 1 }} / {{ totalSteps }}</span>
              </div>
              <div class="progress-bar">
                <div class="progress-fill" :style="{ width: progressPercent + '%' }"></div>
              </div>
            </div>

            <!-- 讲解区域 -->
            <div class="teaching-area">
              <!-- 步骤卡片 -->
              <div v-if="currentStep" class="step-card">
                <div class="step-header">
                  <span class="step-badge">步骤 {{ currentStepIndex + 1 }}</span>
                  <span v-if="currentStep.duration" class="step-duration">{{ currentStep.duration }}秒</span>
                </div>
                <h3 class="step-title">{{ currentStep.title }}</h3>
                <p class="step-description">{{ currentStep.description }}</p>

                <!-- 可视化元素 -->
                <div v-if="currentStep.visualElement" class="visual-element">
                  <div class="visual-placeholder" :class="currentStep.visualElement.type">
                    <span class="visual-icon">{{ getVisualIcon(currentStep.visualElement.type) }}</span>
                    <span class="visual-label">{{ currentStep.visualElement.type }}</span>
                  </div>
                </div>

                <!-- 关键点 -->
                <div v-if="currentStep.keyPoints?.length" class="key-points">
                  <span class="points-label">💡 关键点</span>
                  <ul class="points-list">
                    <li v-for="point in currentStep.keyPoints" :key="point">{{ point }}</li>
                  </ul>
                </div>
              </div>

              <!-- 播放控制 -->
              <div class="playback-controls">
                <button class="control-btn" @click="previousStep" :disabled="currentStepIndex === 0">
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                    <path d="M15 19l-7-7 7-7" stroke="currentColor" stroke-width="2"/>
                  </svg>
                </button>
                <button class="control-btn primary" @click="toggleSpeech" :class="{ active: isSpeaking }">
                  <svg v-if="!isSpeaking" width="24" height="24" viewBox="0 0 24 24" fill="none">
                    <path d="M8 5v14l11-7z" fill="currentColor"/>
                  </svg>
                  <svg v-else width="24" height="24" viewBox="0 0 24 24" fill="none">
                    <rect x="6" y="4" width="4" height="16" fill="currentColor"/>
                    <rect x="14" y="4" width="4" height="16" fill="currentColor"/>
                  </svg>
                </button>
                <button class="control-btn" @click="nextStep" :disabled="currentStepIndex >= totalSteps - 1">
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                    <path d="M9 5l7 7-7 7" stroke="currentColor" stroke-width="2"/>
                  </svg>
                </button>
                <div class="speech-rate">
                  <span class="rate-label">语速</span>
                  <input type="range" min="0.5" max="2" step="0.1" v-model="speechRate" class="rate-slider" />
                  <span class="rate-value">{{ speechRate }}x</span>
                </div>
              </div>

              <!-- 语音输入 -->
              <div class="voice-input-area">
                <button
                  class="voice-btn"
                  :class="{ listening: isListening }"
                  @click="toggleVoiceInput"
                >
                  <svg v-if="!isListening" width="24" height="24" viewBox="0 0 24 24" fill="none">
                    <path d="M12 14c1.66 0 3-1.34 3-3V5c0-1.66-1.34-3-3-3S9 3.34 9 5v6c0 1.66 1.34 3 3 3z" fill="currentColor"/>
                    <path d="M17 11c0 2.76-2.24 5-5 5s-5-2.24-5-5H5c0 3.53 2.61 6.43 6 6.92V21h2v-3.08c3.39-.49 6-3.39 6-6.92h-2z" fill="currentColor"/>
                  </svg>
                  <svg v-else width="24" height="24" viewBox="0 0 24 24" fill="none" class="pulse-animation">
                    <rect x="6" y="6" width="12" height="12" rx="2" fill="currentColor"/>
                  </svg>
                </button>
                <span class="voice-hint">{{ isListening ? '正在聆听...' : '点击说话' }}</span>
              </div>
            </div>

            <!-- 课程选择 -->
            <div class="lesson-list">
              <div class="section-title">📚 选择课程</div>
              <div class="lessons">
                <button
                  v-for="lesson in availableLessons"
                  :key="lesson.id"
                  class="lesson-btn"
                  :class="{ active: currentLesson?.id === lesson.id }"
                  @click="selectLesson(lesson.id)"
                >
                  <span class="lesson-icon">{{ lesson.icon }}</span>
                  <div class="lesson-info">
                    <span class="lesson-name">{{ lesson.title }}</span>
                    <span class="lesson-meta">{{ lesson.steps.length }} 步骤 · {{ lesson.difficulty }}</span>
                  </div>
                </button>
              </div>
            </div>

            <!-- 对话历史 -->
            <div class="conversation-area">
              <div class="section-title">💬 对话</div>
              <div class="messages">
                <div
                  v-for="(msg, index) in conversationHistory"
                  :key="index"
                  class="message"
                  :class="msg.role"
                >
                  <span class="message-avatar">{{ msg.role === 'ai' ? '🤖' : '👤' }}</span>
                  <span class="message-content">{{ msg.content }}</span>
                </div>
              </div>
              <div class="input-area">
                <input
                  v-model="inputMessage"
                  type="text"
                  placeholder="输入问题..."
                  class="message-input"
                  @keydown.enter="sendMessage"
                />
                <button class="send-btn" @click="sendMessage">
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                    <path d="M22 2L11 13M22 2l-7 20-4-9-9-4 20-7z" stroke="currentColor" stroke-width="2"/>
                  </svg>
                </button>
              </div>
            </div>
          </div>

          <!-- 收起时的最小化视图 -->
          <div v-else class="collapsed-view">
            <button class="expand-btn" @click="toggleCollapse">
              <span class="expand-icon">📖</span>
              <span class="expand-text">AI 助教</span>
            </button>
            <div v-if="currentLesson" class="mini-progress">
              <div class="mini-progress-bar" :style="{ width: progressPercent + '%' }"></div>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useAITutor } from '@/composables/useAITutor'

const props = defineProps<{
  isOpen: boolean
}>()

const emit = defineEmits<{
  close: []
}>()

const {
  tutoringMode,
  difficultyLevel,
  isSpeaking,
  isListening,
  currentLessonSteps,
  currentStepIndex,
  conversationHistory,
  lessonLibrary,
  speak,
  stopSpeaking,
  initSpeechRecognition,
  startLesson,
  nextStep,
  previousStep,
  currentStep,
  totalSteps,
  progressPercent
} = useAITutor()

// 状态
const isCollapsed = ref(false)
const currentMode = ref<'explanation' | 'demonstration' | 'practice' | 'quiz'>('explanation')
const speechRate = ref(1.0)
const inputMessage = ref('')

// 模式定义
const tutoringModes = [
  { id: 'explanation', icon: '📖', label: '讲解' },
  { id: 'demonstration', icon: '🎬', label: '演示' },
  { id: 'practice', icon: '✏️', label: '练习' },
  { id: 'quiz', icon: '❓', label: '测验' }
]

// 计算属性
const currentLesson = computed(() => {
  if (currentLessonSteps.value.length > 0) {
    const lessonId = currentLessonSteps.value[0]?.id?.split('-')[0]
    const lessons = Object.values(lessonLibrary)
    return lessons.find(l => l.id === lessonId)
  }
  return null
})

const availableLessons = computed(() => {
  return Object.values(lessonLibrary).map(lesson => ({
    id: lesson.id,
    title: lesson.title,
    icon: lesson.id === 'beam_analysis' ? '🏛️' : lesson.id === 'mesh_generation' ? '🔲' : '⚙️',
    steps: lesson.steps,
    difficulty: lesson.difficulty
  }))
})

const tutorStatusText = computed(() => {
  if (isSpeaking.value) return '正在讲解...'
  if (isListening.value) return '正在聆听...'
  if (currentLesson.value) return `学习 ${currentLesson.value.title}`
  return '随时帮你'
})

// 方法
function close() {
  emit('close')
}

function toggleCollapse() {
  isCollapsed.value = !isCollapsed.value
}

function switchMode(mode: 'explanation' | 'demonstration' | 'practice' | 'quiz') {
  currentMode.value = mode
  tutoringMode.value = mode
}

function selectLesson(lessonId: string) {
  startLesson(lessonId)
}

function toggleSpeech() {
  if (isSpeaking.value) {
    stopSpeaking()
  } else if (currentStep.value?.voiceScript) {
    speak(currentStep.value.voiceScript, { rate: speechRate.value })
  }
}

function toggleVoiceInput() {
  if (isListening.value) {
    // 停止聆听
  } else {
    // 开始聆听
    initSpeechRecognition().then(supported => {
      if (supported) {
        // 开始语音识别
      }
    })
  }
}

function sendMessage() {
  if (!inputMessage.value.trim()) return

  conversationHistory.value.push({
    role: 'student',
    content: inputMessage.value,
    time: new Date()
  })

  // 模拟 AI 回复
  setTimeout(() => {
    conversationHistory.value.push({
      role: 'ai',
      content: '这是一个很好的问题！让我来为你解答...',
      time: new Date()
    })
  }, 500)

  inputMessage.value = ''
}

function getVisualIcon(type: string): string {
  const icons: Record<string, string> = {
    diagram: '📊',
    animation: '🎬',
    model: '🏗️',
    code: '💻',
    chart: '📈'
  }
  return icons[type] || '📄'
}

// 监听语音速率变化
watch(speechRate, (newRate) => {
  // 可以在 speak 时传入这个值
})

onMounted(() => {
  // 初始化语音合成
  if (typeof window !== 'undefined' && window.speechSynthesis) {
    window.speechSynthesis.getVoices()
  }
})
</script>

<style scoped>
.ai-tutor-overlay {
  position: fixed;
  inset: 0;
  z-index: 9998;
  background: rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(4px);
}

.tutor-panel {
  position: fixed;
  top: 0;
  right: 0;
  width: 400px;
  height: 100vh;
  background: var(--bg-surface);
  box-shadow: -4px 0 24px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  transition: width var(--duration-normal) var(--ease-out);
}

.tutor-panel.is-collapsed {
  width: 60px;
}

/* 头部 */
.tutor-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  border-bottom: 1px solid var(--border-subtle);
  background: linear-gradient(135deg, var(--primary) 0%, #6366F1 100%);
  color: white;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.avatar-container {
  position: relative;
}

.avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.2);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
}

.avatar.speaking {
  animation: pulse-glow 1.5s infinite;
}

@keyframes pulse-glow {
  0%, 100% { box-shadow: 0 0 0 0 rgba(255, 255, 255, 0.4); }
  50% { box-shadow: 0 0 0 8px rgba(255, 255, 255, 0); }
}

.status-dot {
  position: absolute;
  bottom: 2px;
  right: 2px;
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.5);
  border: 2px solid white;
}

.status-dot.active {
  background: var(--accent-green);
}

.header-info {
  display: flex;
  flex-direction: column;
}

.tutor-name {
  font-weight: 600;
  font-size: 14px;
}

.tutor-status {
  font-size: 11px;
  opacity: 0.8;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.header-actions .icon-btn {
  color: white;
}

.header-actions .icon-btn:hover {
  background: rgba(255, 255, 255, 0.2);
}

/* 内容区 */
.tutor-content {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 16px;
}

/* 模式标签 */
.mode-tabs {
  display: flex;
  gap: 8px;
  padding: 4px;
  background: var(--bg-elevated);
  border-radius: var(--radius-lg);
}

.mode-tab {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  padding: 10px 8px;
  border: none;
  background: transparent;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--duration-fast);
}

.mode-tab:hover {
  background: var(--bg-surface);
}

.mode-tab.active {
  background: var(--bg-surface);
  box-shadow: var(--shadow-sm);
}

.mode-icon {
  font-size: 18px;
}

.mode-label {
  font-size: 11px;
  color: var(--text-secondary);
}

.mode-tab.active .mode-label {
  color: var(--primary);
  font-weight: 500;
}

/* 课程进度 */
.lesson-progress {
  padding: 12px;
  background: var(--bg-elevated);
  border-radius: var(--radius-lg);
}

.lesson-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
}

.lesson-title {
  font-weight: 600;
  font-size: 13px;
  color: var(--text-primary);
}

.lesson-step {
  font-size: 12px;
  color: var(--text-muted);
}

.progress-bar {
  height: 4px;
  background: var(--border-default);
  border-radius: 2px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--primary) 0%, var(--accent-cyan) 100%);
  transition: width var(--duration-normal);
}

/* 教学区域 */
.teaching-area {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

/* 步骤卡片 */
.step-card {
  padding: 16px;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-xl);
}

.step-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
}

.step-badge {
  font-size: 11px;
  padding: 2px 8px;
  background: var(--primary-glow);
  color: var(--primary);
  border-radius: var(--radius-full);
}

.step-duration {
  font-size: 11px;
  color: var(--text-muted);
}

.step-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 8px;
}

.step-description {
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.6;
}

/* 可视化元素 */
.visual-element {
  margin-top: 12px;
}

.visual-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 24px;
  background: var(--bg-elevated);
  border-radius: var(--radius-lg);
  min-height: 100px;
}

.visual-icon {
  font-size: 32px;
}

.visual-label {
  font-size: 11px;
  color: var(--text-muted);
  text-transform: uppercase;
}

/* 关键点 */
.key-points {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--border-subtle);
}

.points-label {
  font-size: 12px;
  color: var(--text-secondary);
  font-weight: 500;
}

.points-list {
  margin-top: 8px;
  padding-left: 16px;
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.8;
}

/* 播放控制 */
.playback-controls {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 16px;
  background: var(--bg-elevated);
  border-radius: var(--radius-xl);
}

.control-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border: none;
  background: var(--bg-surface);
  border-radius: 50%;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--duration-fast);
}

.control-btn:hover:not(:disabled) {
  background: var(--primary-glow);
  color: var(--primary);
}

.control-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.control-btn.primary {
  width: 56px;
  height: 56px;
  background: var(--primary);
  color: white;
}

.control-btn.primary:hover {
  background: var(--primary-hover);
}

.control-btn.primary.active {
  background: var(--accent-red);
}

.speech-rate {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-left: 16px;
}

.rate-label {
  font-size: 11px;
  color: var(--text-muted);
}

.rate-slider {
  width: 60px;
  accent-color: var(--primary);
}

.rate-value {
  font-size: 11px;
  color: var(--text-secondary);
  min-width: 30px;
}

/* 语音输入 */
.voice-input-area {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.voice-btn {
  width: 48px;
  height: 48px;
  border: none;
  background: var(--bg-elevated);
  border-radius: 50%;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--duration-fast);
}

.voice-btn:hover {
  background: var(--primary-glow);
  color: var(--primary);
}

.voice-btn.listening {
  background: var(--primary);
  color: white;
  animation: pulse 1s infinite;
}

.voice-hint {
  font-size: 11px;
  color: var(--text-muted);
}

/* 课程列表 */
.lesson-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.section-title {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.lessons {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.lesson-btn {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all var(--duration-fast);
  text-align: left;
}

.lesson-btn:hover {
  background: var(--bg-elevated);
  border-color: var(--primary);
}

.lesson-btn.active {
  background: var(--primary-glow);
  border-color: var(--primary);
}

.lesson-icon {
  font-size: 24px;
}

.lesson-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.lesson-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.lesson-meta {
  font-size: 11px;
  color: var(--text-muted);
}

/* 对话区 */
.conversation-area {
  display: flex;
  flex-direction: column;
  gap: 8px;
  flex: 1;
  min-height: 200px;
}

.messages {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 150px;
}

.message {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  padding: 8px;
  border-radius: var(--radius-lg);
}

.message.ai {
  background: var(--bg-elevated);
}

.message.student {
  background: var(--primary-glow);
}

.message-avatar {
  font-size: 16px;
}

.message-content {
  font-size: 13px;
  color: var(--text-primary);
  line-height: 1.4;
}

.input-area {
  display: flex;
  gap: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--border-subtle);
}

.message-input {
  flex: 1;
  padding: 10px 12px;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  font-size: 13px;
  outline: none;
}

.message-input:focus {
  border-color: var(--primary);
}

.send-btn {
  padding: 10px;
  background: var(--primary);
  color: white;
  border: none;
  border-radius: var(--radius-lg);
  cursor: pointer;
}

.send-btn:hover {
  background: var(--primary-hover);
}

/* 收起状态 */
.collapsed-view {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 16px;
  gap: 12px;
}

.expand-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  border: none;
  background: transparent;
  cursor: pointer;
  color: white;
}

.expand-icon {
  font-size: 24px;
}

.expand-text {
  font-size: 11px;
}

.mini-progress {
  width: 100%;
  height: 3px;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 2px;
  overflow: hidden;
}

.mini-progress-bar {
  height: 100%;
  background: white;
}

/* 动画 */
.tutor-slide-enter-active,
.tutor-slide-leave-active {
  transition: all var(--duration-normal) var(--ease-out);
}

.tutor-slide-enter-from,
.tutor-slide-leave-to {
  opacity: 0;
}

.tutor-slide-enter-from .tutor-panel {
  transform: translateX(100%);
}

.tutor-slide-leave-to .tutor-panel {
  transform: translateX(100%);
}

@keyframes pulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.05); }
}
</style>