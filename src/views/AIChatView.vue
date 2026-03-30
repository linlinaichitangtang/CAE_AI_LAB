<template>
  <div class="ai-chat-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">AI助手</h2>
        <p class="text-xs text-[var(--text-muted)]">全局智能助手，支持笔记、建模、仿真、代码全场景辅助</p>
      </div>
      <div class="flex items-center gap-2">
        <button 
          @click="clearConfirm = true"
          :disabled="!aiStore.hasMessages || aiStore.isGenerating"
          class="btn btn-ghost text-xs"
        >
          🗑️ 清空对话
        </button>
        <router-link to="/settings">
          <button class="btn btn-ghost text-xs">
            ⚙️ 配置
          </button>
        </router-link>
      </div>
    </div>

    <!-- Main content: messages -->
    <div ref="messagesContainer" class="flex-1 overflow-y-auto p-4 space-y-4">
      <!-- Empty state -->
      <div v-if="!aiStore.hasMessages && !aiStore.isGenerating" class="text-center py-12">
        <div class="text-5xl mb-4 opacity-30">🤖</div>
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-2">CAELab AI助手</h3>
        <p class="text-sm text-[var(--text-muted)] max-w-md mx-auto">
          我可以帮你：<br>
          • 写笔记和总结<br>
          • 根据描述生成3D建模参数<br>
          • 辅助编写和调试代码<br>
          • 解释CAE仿真结果、给出优化建议<br>
          • 回答任何CAE相关问题
        </p>
        <div class="mt-6 flex flex-wrap gap-2 justify-center">
          <button @click="insertQuickQuestion('帮我设计一个悬臂梁的仿真实例')" class="px-3 py-1.5 text-xs border border-blue-300 text-blue-600 rounded-full hover:bg-blue-50 transition">
            帮我设计一个悬臂梁
          </button>
          <button @click="insertQuickQuestion('解释一下Von Mises应力是什么意思')" class="px-3 py-1.5 text-xs border border-blue-300 text-blue-600 rounded-full hover:bg-blue-50 transition">
            解释Von Mises应力
          </button>
          <button @click="insertQuickQuestion('帮我写一个Python处理CAE结果的脚本')" class="px-3 py-1.5 text-xs border border-blue-300 text-blue-600 rounded-full hover:bg-blue-50 transition">
            写Python处理结果脚本
          </button>
        </div>
      </div>

      <!-- Message list -->
      <div 
        v-for="(msg, idx) in aiStore.messages" 
        :key="idx"
        :class="['message', msg.role === 'user' ? 'user-message' : 'assistant-message']"
      >
        <div class="message-header">
          <span class="avatar">{{ msg.role === 'user' ? '👤' : '🤖' }}</span>
          <span class="name">{{ msg.role === 'user' ? '你' : 'AI助手' }}</span>
          <span class="time text-xs opacity-60">{{ formatTime(msg.timestamp) }}</span>
        </div>
        <div class="message-content">
          <pre>{{ msg.content }}</pre>
        </div>
      </div>

      <!-- Streaming message -->
      <div v-if="aiStore.isGenerating && aiStore.currentStreamingText" class="message assistant-message">
        <div class="message-header">
          <span class="avatar">🤖</span>
          <span class="name">AI助手</span>
          <span class="time text-xs opacity-60">正在生成...</span>
        </div>
        <div class="message-content">
          <pre>{{ aiStore.currentStreamingText }}</pre>
        </div>
      </div>

      <!-- Loading indicator -->
      <div v-if="aiStore.isGenerating && !aiStore.currentStreamingText" class="flex items-center gap-2 text-sm text-[var(--text-muted)] p-4">
        <span class="animate-spin">⟳</span>
        AI正在思考中...
      </div>
    </div>

    <!-- Input area -->
    <div class="border-t border-[var(--border-subtle)] p-4 bg-[var(--bg-surface)]">
      <div class="flex flex-col gap-2">
        <textarea
          v-model="inputText"
          @keyup.enter.ctrl="sendMessage"
          @keyup.enter.meta="sendMessage"
          placeholder="输入问题... (Ctrl+Enter 发送)"
          class="input w-full text-sm min-h-[80px] resize-y"
          :disabled="aiStore.isGenerating"
        />
        <div class="flex items-center justify-between">
          <div class="text-xs text-[var(--text-muted)]">
            当前AI源: {{ getAiSourceName(aiStore.config.aiSource) }}
            <span v-if="!aiStore.isConfigured" class="text-yellow-600 ml-2">⚠️ 请先配置API密钥</span>
          </div>
          <div class="flex items-center gap-2">
            <button 
              v-if="aiStore.isGenerating"
              @click="cancelGeneration"
              class="btn btn-ghost text-xs"
            >
              ⏸️ 取消
            </button>
            <button 
              @click="sendMessage"
              :disabled="!canSend || aiStore.isGenerating"
              class="btn btn-primary text-xs"
            >
              <span v-if="!aiStore.isGenerating">发送 ↵</span>
              <span v-else>发送中...</span>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Clear confirm modal -->
    <div v-if="clearConfirm" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg p-6 w-80">
        <h3 class="text-lg font-semibold mb-2">确认清空对话？</h3>
        <p class="text-sm text-gray-500 mb-4">此操作不可撤销</p>
        <div class="flex justify-end gap-2">
          <button @click="clearConfirm = false" class="btn btn-ghost">取消</button>
          <button @click="doClearMessages" class="btn btn-primary text-red-600 bg-red-600">确认清空</button>
        </div>
      </div>
    </div>

    <!-- Context info panel (right side for debug) -->
    <div class="absolute top-16 right-4 w-64 bg-white/90 backdrop-blur rounded shadow-lg border p-3">
      <h4 class="text-xs font-semibold mb-2 text-gray-700">当前上下文</h4>
      <div class="text-[10px] text-gray-600 space-y-1">
        <div><span class="font-medium">工具:</span> {{ aiStore.context.currentTool }}</div>
        <div v-if="aiStore.context.projectName"><span class="font-medium">项目:</span> {{ aiStore.context.projectName }}</div>
        <div v-if="aiStore.context.modelingGeometry">
          <span class="font-medium">几何体:</span> {{ aiStore.context.modelingGeometry.type }}
        </div>
        <div v-if="aiStore.context.simulationResult">
          <span class="font-medium">结果:</span> {{ aiStore.context.simulationResult.maxStress }} MPa
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted } from 'vue'
import { useAiStore } from '@/stores/ai'
import { invoke } from '@tauri-apps/api/tauri'

const aiStore = useAiStore()
const messagesContainer = ref<HTMLDivElement | null>(null)
const inputText = ref('')
const clearConfirm = ref(false)

// Load config on mount
onMounted(() => {
  aiStore.loadConfig()
})

// Can send when have text and not generating
const canSend = computed(() => {
  return inputText.value.trim().length > 0 && !aiStore.isGenerating
})

// Format timestamp
function formatTime(ts: number): string {
  return new Date(ts).toLocaleTimeString('zh-CN')
}

// Get AI source display name
function getAiSourceName(source: string): string {
  const names: Record<string, string> = {
    'third-party': '第三方API',
    'local': '本地模型',
    'platform': '平台算力'
  }
  return names[source] || source
}

// Scroll to bottom when messages change
function scrollToBottom() {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
    }
  })
}

watch(() => aiStore.messages.length, scrollToBottom)
watch(() => aiStore.currentStreamingText, scrollToBottom)

// Insert quick question
function insertQuickQuestion(q: string) {
  inputText.value = q
}

// Send message
async function sendMessage() {
  if (!canSend.value) return

  const content = inputText.value.trim()
  inputText.value = ''

  // Add user message
  aiStore.addUserMessage(content)

  // Start generation
  aiStore.startGeneration()

  try {
    // Get full messages with context
    const messages = aiStore.fullMessagesWithContext

    // Call Tauri backend to invoke AI
    // Backend will handle streaming via events
    const response = await invoke<string>('ai_chat_completion', {
      messages: messages.map(m => ({ role: m.role, content: m.content })),
      config: aiStore.config
    })

    // For now, just set the full response (streaming will be implemented in backend)
    aiStore.updateStreamingText(response)
    aiStore.finishGeneration()

  } catch (e: any) {
    console.error('AI request failed:', e)
    aiStore.addAssistantMessage(`请求失败: ${String(e)}`)
    aiStore.finishGeneration()
  }
}

// Cancel generation
function cancelGeneration() {
  aiStore.cancelGeneration()
}

// Clear messages
function doClearMessages() {
  aiStore.clearMessages()
  clearConfirm.value = false
}
</script>

<style scoped>
.ai-chat-view {
  --accent: #3b82f6;
}

.btn {
  @apply px-3 py-1.5 rounded text-xs font-medium transition-colors;
}

.btn-primary {
  @apply bg-blue-600 text-white hover:bg-blue-700;
}

.btn-primary.bg-red-600 {
  @apply hover:bg-red-700;
}

.btn-ghost {
  @apply bg-transparent hover:bg-gray-100 text-gray-700 border border-gray-300;
}

.input {
  @apply px-3 py-2 border border-[var(--border-subtle)] rounded bg-[var(--bg-base)] text-[var(--text-primary)] focus:outline-none focus:border-blue-500;
}

.message {
  @apply max-w-4xl mx-auto;
}

.message-header {
  @apply flex items-center gap-2 mb-2;
}

.message-header .avatar {
  @apply text-base;
}

.message-header .name {
  @apply text-sm font-medium text-[var(--text-primary)];
}

.message-header .time {
  @apply ml-auto;
}

.message-content {
  @apply rounded-lg p-3;
}

.user-message .message-content {
  @apply bg-blue-500 text-white;
}

.assistant-message .message-content {
  @apply bg-[var(--bg-surface)] border border-[var(--border-subtle)] text-[var(--text-primary)];
}

.message-content pre {
  @apply whitespace-pre-wrap font-sans text-sm m-0;
}

.user-message .message-content pre {
  @apply text-white;
}
</style>
