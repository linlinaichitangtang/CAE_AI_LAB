<template>
  <div class="mobile-ai-chat">
    <!-- Header -->
    <div class="chat-header">
      <button class="back-btn" @click="$emit('close')" title="返回">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
        </svg>
      </button>
      <div class="header-info">
        <h3 class="header-title">AI 助手</h3>
        <span class="header-status" :class="{ active: !aiStore.isGenerating }">
          {{ aiStore.isGenerating ? '思考中...' : '在线' }}
        </span>
      </div>
      <button class="clear-btn" @click="clearConfirm = true" :disabled="!aiStore.hasMessages">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
        </svg>
      </button>
    </div>

    <!-- Messages -->
    <div ref="messagesContainer" class="messages-area">
      <!-- Empty state -->
      <div v-if="!aiStore.hasMessages && !aiStore.isGenerating" class="empty-state">
        <div class="empty-icon">AI</div>
        <p class="empty-title">CAELab AI 助手</p>
        <p class="empty-desc">支持笔记、建模、仿真、代码全场景辅助</p>

        <!-- Quick questions -->
        <div class="quick-questions">
          <button
            v-for="q in quickQuestions"
            :key="q.text"
            class="quick-btn"
            @click="insertQuestion(q.text)"
          >
            <span class="quick-icon">{{ q.icon }}</span>
            <span>{{ q.label }}</span>
          </button>
        </div>
      </div>

      <!-- Message list -->
      <div
        v-for="(msg, idx) in aiStore.messages"
        :key="idx"
        :class="['message-bubble', msg.role === 'user' ? 'user-bubble' : 'ai-bubble']"
      >
        <div class="bubble-avatar">
          {{ msg.role === 'user' ? 'U' : 'AI' }}
        </div>
        <div class="bubble-content">
          <pre class="bubble-text">{{ msg.content }}</pre>
          <span class="bubble-time">{{ formatTime(msg.timestamp) }}</span>
        </div>
      </div>

      <!-- Streaming message -->
      <div v-if="aiStore.isGenerating && aiStore.currentStreamingText" class="message-bubble ai-bubble">
        <div class="bubble-avatar">AI</div>
        <div class="bubble-content">
          <pre class="bubble-text">{{ aiStore.currentStreamingText }}</pre>
          <span class="typing-indicator">
            <span></span><span></span><span></span>
          </span>
        </div>
      </div>

      <!-- Loading indicator -->
      <div v-if="aiStore.isGenerating && !aiStore.currentStreamingText" class="loading-indicator">
        <div class="loading-dots">
          <span></span><span></span><span></span>
        </div>
        <span>AI 正在思考...</span>
      </div>
    </div>

    <!-- Voice input display -->
    <div v-if="isListening" class="voice-input-bar">
      <div class="voice-pulse"></div>
      <span class="voice-text">{{ transcript || '正在聆听...' }}</span>
      <button class="voice-stop-btn" @click="stopVoice">停止</button>
    </div>

    <!-- Input area -->
    <div class="input-area">
      <!-- Voice button -->
      <button
        v-if="voiceSupported"
        class="voice-btn"
        :class="{ listening: isListening }"
        @click="() => toggleVoice()"
        :title="isListening ? '停止语音' : '语音输入'"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"/>
        </svg>
      </button>

      <!-- Text input -->
      <div class="input-wrapper">
        <textarea
          ref="inputRef"
          v-model="inputText"
          @keyup.enter.exact="sendMessage"
          placeholder="输入问题..."
          rows="1"
          class="chat-input"
          :disabled="aiStore.isGenerating"
          @input="autoResize"
        />
      </div>

      <!-- Send button -->
      <button
        class="send-btn"
        :disabled="!canSend"
        @click="sendMessage"
      >
        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
        </svg>
      </button>
    </div>

    <!-- Clear confirm modal -->
    <div v-if="clearConfirm" class="modal-overlay" @click.self="clearConfirm = false">
      <div class="modal-content">
        <h4>确认清空对话?</h4>
        <p>此操作不可撤销</p>
        <div class="modal-actions">
          <button class="modal-cancel" @click="clearConfirm = false">取消</button>
          <button class="modal-confirm" @click="doClear">确认清空</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted } from 'vue'
import { useAiStore } from '@/stores/ai'
import { useSpeechRecognition } from '@/composables/useSpeechRecognition'
import { invoke } from '@tauri-apps/api/core'

defineEmits<{
  'close': []
}>()

const aiStore = useAiStore()
const {
  isListening,
  isSupported: voiceSupported,
  transcript,
  toggle: toggleVoice,
  stop: stopVoice,
  clearTranscript,
} = useSpeechRecognition()

const messagesContainer = ref<HTMLDivElement | null>(null)
const inputRef = ref<HTMLTextAreaElement | null>(null)
const inputText = ref('')
const clearConfirm = ref(false)

// 快捷提问
const quickQuestions = [
  { icon: 'beam', label: '悬臂梁仿真', text: '帮我设计一个悬臂梁的仿真实例' },
  { icon: 'stress', label: 'Von Mises', text: '解释一下Von Mises应力是什么意思' },
  { icon: 'code', label: 'Python脚本', text: '帮我写一个Python处理CAE结果的脚本' },
  { icon: 'mesh', label: '网格质量', text: '如何评估CAE网格质量？有哪些指标？' },
]

onMounted(() => {
  aiStore.loadConfig()
})

const canSend = computed(() => {
  return inputText.value.trim().length > 0 && !aiStore.isGenerating
})

function formatTime(ts: number): string {
  return new Date(ts).toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
}

function scrollToBottom() {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
    }
  })
}

watch(() => aiStore.messages.length, scrollToBottom)
watch(() => aiStore.currentStreamingText, scrollToBottom)

// 当语音识别结束时，将文本填入输入框
watch(isListening, (listening) => {
  if (!listening && transcript.value) {
    inputText.value += transcript.value
    clearTranscript()
    nextTick(() => autoResize())
  }
})

function insertQuestion(text: string) {
  inputText.value = text
  nextTick(() => {
    inputRef.value?.focus()
    autoResize()
  })
}

function autoResize() {
  if (!inputRef.value) return
  inputRef.value.style.height = 'auto'
  inputRef.value.style.height = Math.min(inputRef.value.scrollHeight, 120) + 'px'
}

async function sendMessage() {
  if (!canSend.value) return

  const content = inputText.value.trim()
  inputText.value = ''
  nextTick(() => {
    if (inputRef.value) inputRef.value.style.height = 'auto'
  })

  aiStore.addUserMessage(content)
  aiStore.startGeneration()

  try {
    const messages = aiStore.fullMessagesWithContext
    const response = await invoke<string>('ai_chat_completion', {
      messages: messages.map(m => ({ role: m.role, content: m.content })),
      config: aiStore.config
    })
    aiStore.updateStreamingText(response)
    aiStore.finishGeneration()
  } catch (e: any) {
    console.error('AI request failed:', e)
    aiStore.addAssistantMessage(`请求失败: ${String(e)}`)
    aiStore.finishGeneration()
  }
}

function doClear() {
  aiStore.clearMessages()
  clearConfirm.value = false
}
</script>

<style scoped>
.mobile-ai-chat {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #f5f5f5;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

/* Header */
.chat-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: #fff;
  border-bottom: 1px solid #e5e5e5;
  min-height: 56px;
}
.back-btn, .clear-btn {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: none;
  color: #666;
  border-radius: 8px;
  -webkit-tap-highlight-color: transparent;
}
.back-btn:active, .clear-btn:active {
  background: #f0f0f0;
}
.header-info {
  flex: 1;
}
.header-title {
  font-size: 16px;
  font-weight: 600;
  color: #1a1a1a;
  margin: 0;
}
.header-status {
  font-size: 11px;
  color: #999;
}
.header-status.active {
  color: #22c55e;
}

/* Messages area */
.messages-area {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  -webkit-overflow-scrolling: touch;
}

/* Empty state */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 40px 20px;
  text-align: center;
}
.empty-icon {
  width: 56px;
  height: 56px;
  border-radius: 16px;
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  font-weight: 700;
  margin-bottom: 16px;
}
.empty-title {
  font-size: 18px;
  font-weight: 600;
  color: #1a1a1a;
  margin: 0 0 4px;
}
.empty-desc {
  font-size: 13px;
  color: #999;
  margin: 0 0 24px;
}
.quick-questions {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
  width: 100%;
  max-width: 320px;
}
.quick-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  border: 1px solid #e5e5e5;
  border-radius: 12px;
  background: #fff;
  font-size: 12px;
  color: #333;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  transition: all 0.15s;
}
.quick-btn:active {
  background: #f0f0f0;
  transform: scale(0.97);
}
.quick-icon {
  font-size: 16px;
}

/* Message bubbles */
.message-bubble {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
  max-width: 100%;
}
.message-bubble.user-bubble {
  flex-direction: row-reverse;
}
.bubble-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 11px;
  font-weight: 600;
  flex-shrink: 0;
}
.user-bubble .bubble-avatar {
  background: #3b82f6;
  color: #fff;
}
.ai-bubble .bubble-avatar {
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  color: #fff;
}
.bubble-content {
  max-width: 80%;
  min-width: 0;
}
.bubble-text {
  margin: 0;
  padding: 10px 14px;
  border-radius: 16px;
  font-size: 14px;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-word;
  font-family: inherit;
}
.user-bubble .bubble-text {
  background: #3b82f6;
  color: #fff;
  border-bottom-right-radius: 4px;
}
.ai-bubble .bubble-text {
  background: #fff;
  color: #333;
  border-bottom-left-radius: 4px;
  box-shadow: 0 1px 2px rgba(0,0,0,0.06);
}
.bubble-time {
  display: block;
  font-size: 10px;
  color: #999;
  margin-top: 4px;
  padding: 0 4px;
}
.user-bubble .bubble-time {
  text-align: right;
}

/* Typing indicator */
.typing-indicator {
  display: inline-flex;
  gap: 3px;
  padding: 4px 0 0 4px;
}
.typing-indicator span {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: #999;
  animation: typing 1.4s infinite;
}
.typing-indicator span:nth-child(2) { animation-delay: 0.2s; }
.typing-indicator span:nth-child(3) { animation-delay: 0.4s; }
@keyframes typing {
  0%, 60%, 100% { opacity: 0.3; transform: scale(0.8); }
  30% { opacity: 1; transform: scale(1); }
}

/* Loading indicator */
.loading-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  color: #999;
  font-size: 13px;
}
.loading-dots {
  display: flex;
  gap: 4px;
}
.loading-dots span {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #3b82f6;
  animation: bounce 1.4s infinite;
}
.loading-dots span:nth-child(2) { animation-delay: 0.2s; }
.loading-dots span:nth-child(3) { animation-delay: 0.4s; }
@keyframes bounce {
  0%, 80%, 100% { transform: scale(0.6); opacity: 0.4; }
  40% { transform: scale(1); opacity: 1; }
}

/* Voice input bar */
.voice-input-bar {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 16px;
  background: #1a1a2e;
  color: #fff;
}
.voice-pulse {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: #ef4444;
  animation: pulse 1.5s infinite;
}
@keyframes pulse {
  0%, 100% { box-shadow: 0 0 0 0 rgba(239, 68, 68, 0.6); }
  50% { box-shadow: 0 0 0 8px rgba(239, 68, 68, 0); }
}
.voice-text {
  flex: 1;
  font-size: 14px;
  opacity: 0.9;
}
.voice-stop-btn {
  padding: 4px 12px;
  border: 1px solid rgba(255,255,255,0.3);
  border-radius: 16px;
  background: none;
  color: #fff;
  font-size: 12px;
  -webkit-tap-highlight-color: transparent;
}

/* Input area */
.input-area {
  display: flex;
  align-items: flex-end;
  gap: 8px;
  padding: 8px 12px;
  padding-bottom: max(8px, env(safe-area-inset-bottom));
  background: #fff;
  border-top: 1px solid #e5e5e5;
}
.voice-btn {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: #f5f5f5;
  color: #666;
  border-radius: 50%;
  flex-shrink: 0;
  -webkit-tap-highlight-color: transparent;
}
.voice-btn.listening {
  background: #ef4444;
  color: #fff;
  animation: pulse 1.5s infinite;
}
.voice-btn:active {
  transform: scale(0.92);
}
.input-wrapper {
  flex: 1;
  min-width: 0;
}
.chat-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #e5e5e5;
  border-radius: 20px;
  font-size: 14px;
  line-height: 1.4;
  resize: none;
  outline: none;
  font-family: inherit;
  max-height: 120px;
  background: #f5f5f5;
  box-sizing: border-box;
}
.chat-input:focus {
  border-color: #3b82f6;
  background: #fff;
}
.send-btn {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: #3b82f6;
  color: #fff;
  border-radius: 50%;
  flex-shrink: 0;
  -webkit-tap-highlight-color: transparent;
  transition: opacity 0.15s;
}
.send-btn:disabled {
  opacity: 0.4;
}
.send-btn:active:not(:disabled) {
  transform: scale(0.92);
}

/* Modal */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0,0,0,0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}
.modal-content {
  background: #fff;
  border-radius: 16px;
  padding: 24px;
  width: 280px;
  text-align: center;
}
.modal-content h4 {
  font-size: 16px;
  font-weight: 600;
  color: #1a1a1a;
  margin: 0 0 8px;
}
.modal-content p {
  font-size: 13px;
  color: #999;
  margin: 0 0 20px;
}
.modal-actions {
  display: flex;
  gap: 12px;
}
.modal-cancel, .modal-confirm {
  flex: 1;
  padding: 10px;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}
.modal-cancel {
  background: #f5f5f5;
  color: #666;
}
.modal-confirm {
  background: #ef4444;
  color: #fff;
}
</style>
