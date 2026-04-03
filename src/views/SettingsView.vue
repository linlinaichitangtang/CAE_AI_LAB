<template>
  <div class="h-full overflow-y-auto p-6">
    <h2 class="text-xl font-bold mb-6">设置</h2>

    <!-- AI 配置 -->
    <div class="mb-6">
      <h3 class="text-lg font-medium mb-3 flex items-center gap-2">
        AI 配置
      </h3>
      <div class="bg-[var(--bg-surface)] rounded-lg p-4 space-y-4">
        <!-- AI 源选择 -->
        <div>
          <label class="block text-sm font-medium mb-1">AI 源</label>
          <select
            v-model="aiConfig.aiSource"
            class="w-full p-2 rounded border border-[var(--border-subtle)] bg-[var(--bg-base)] focus:outline-none focus:ring-2 focus:ring-[var(--accent-primary)]"
          >
            <option value="third-party">第三方 API（如 OpenAI、Claude 等）</option>
            <option value="local">本地部署 (Ollama 等)</option>
            <option value="platform">平台算力（收费）</option>
          </select>
        </div>

        <!-- API 地址 -->
        <div>
          <label class="block text-sm font-medium mb-1">API 地址</label>
          <input
            v-model="aiConfig.apiUrl"
            type="text"
            placeholder="https://api.openai.com/v1"
            class="w-full p-2 rounded border border-[var(--border-subtle)] bg-[var(--bg-base)] focus:outline-none focus:ring-2 focus:ring-[var(--accent-primary)]"
          />
          <p class="text-xs text-[var(--text-secondary)] mt-1">
            第三方API请填写对应服务的API地址，本地部署请填写本地服务地址
          </p>
        </div>

        <!-- API Key -->
        <div>
          <label class="block text-sm font-medium mb-1">API Key</label>
          <input
            v-model="aiConfig.apiKey"
            type="password"
            placeholder="sk-..."
            class="w-full p-2 rounded border border-[var(--border-subtle)] bg-[var(--bg-base)] focus:outline-none focus:ring-2 focus:ring-[var(--accent-primary)]"
          />
        </div>

        <!-- 模型名称 -->
        <div>
          <label class="block text-sm font-medium mb-1">模型名称</label>
          <input
            v-model="aiConfig.modelName"
            type="text"
            placeholder="gpt-4, gpt-3.5-turbo, claude-3-opus 等"
            class="w-full p-2 rounded border border-[var(--border-subtle)] bg-[var(--bg-base)] focus:outline-none focus:ring-2 focus:ring-[var(--accent-primary)]"
          />
        </div>

        <!-- 高级设置 -->
        <details class="group">
          <summary class="cursor-pointer text-sm text-[var(--accent-primary)] hover:underline">
            高级设置
          </summary>
          <div class="mt-3 space-y-3">
            <!-- 最大 Token 数 -->
            <div>
              <label class="block text-sm font-medium mb-1">最大 Token 数</label>
              <input
                v-model.number="aiConfig.maxTokens"
                type="number"
                min="100"
                max="100000"
                class="w-full p-2 rounded border border-[var(--border-subtle)] bg-[var(--bg-base)] focus:outline-none focus:ring-2 focus:ring-[var(--accent-primary)]"
              />
            </div>

            <!-- Temperature -->
            <div>
              <label class="block text-sm font-medium mb-1">
                Temperature (创造性)
                <span class="text-[var(--text-secondary)]">当前: {{ aiConfig.temperature }}</span>
              </label>
              <input
                v-model.number="aiConfig.temperature"
                type="range"
                min="0"
                max="2"
                step="0.1"
                class="w-full"
              />
              <div class="flex justify-between text-xs text-[var(--text-secondary)]">
                <span>精确</span>
                <span>创意</span>
              </div>
            </div>
          </div>
        </details>

        <!-- 保存按钮 -->
        <div class="flex justify-end gap-2 pt-2">
          <button
            @click="resetAiConfig"
            class="px-4 py-2 rounded border border-[var(--border-subtle)] hover:bg-[var(--bg-hover)]"
          >
            重置
          </button>
          <button
            @click="saveAiConfig"
            class="px-4 py-2 rounded bg-[var(--accent-primary)] text-white hover:opacity-90"
          >
            保存配置
          </button>
        </div>
      </div>
    </div>

    <!-- V1.1-011: 快捷键设置 -->
    <div class="mb-6">
      <HotkeySettings />
    </div>

    <!-- 主题设置 -->
    <div class="mb-6">
      <h3 class="text-lg font-medium mb-3">主题</h3>
      <div class="bg-[var(--bg-surface)] rounded-lg p-4">
        <select class="w-full p-2 rounded border border-[var(--border-subtle)] bg-[var(--bg-base)] focus:outline-none focus:ring-2 focus:ring-[var(--accent-primary)]">
          <option>跟随系统</option>
          <option>浅色</option>
          <option>深色</option>
        </select>
      </div>
    </div>

    <!-- 数据存储 -->
    <div class="mb-6">
      <h3 class="text-lg font-medium mb-3">数据存储</h3>
      <div class="bg-[var(--bg-surface)] rounded-lg p-4">
        <p class="text-sm text-[var(--text-secondary)]">当前：本地存储 (SQLite)</p>
      </div>
    </div>

    <!-- 插件管理 (V1.2-010) -->
    <div class="mb-6">
      <h3 class="text-lg font-medium mb-3">{{ t('plugin.settingsEntry') }}</h3>
      <div class="bg-[var(--bg-surface)] rounded-lg p-4">
        <router-link
          to="/plugins"
          class="flex items-center justify-between p-3 rounded-lg border border-[var(--border-subtle)] hover:bg-[var(--bg-hover)] transition-colors group"
        >
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-purple-500 to-indigo-600 flex items-center justify-center">
              <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 4a2 2 0 114 0v1a1 1 0 001 1h3a1 1 0 011 1v3a1 1 0 01-1 1h-1a2 2 0 100 4h1a1 1 0 011 1v3a1 1 0 01-1 1h-3a1 1 0 01-1-1v-1a2 2 0 10-4 0v1a1 1 0 01-1 1H7a1 1 0 01-1-1v-3a1 1 0 00-1-1H4a2 2 0 110-4h1a1 1 0 001-1V7a1 1 0 011-1h3a1 1 0 001-1V4z" />
              </svg>
            </div>
            <div>
              <div class="text-sm font-medium">{{ t('plugin.title') }}</div>
              <div class="text-xs text-[var(--text-secondary)]">{{ t('plugin.installed') }}</div>
            </div>
          </div>
          <svg class="w-4 h-4 text-[var(--text-secondary)] group-hover:text-[var(--text-primary)] transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
        </router-link>
      </div>
    </div>

    <!-- 求解器管理 -->
    <div class="mb-6">
      <h3 class="text-lg font-medium mb-3">{{ t('installer.title') }}</h3>
      <div class="bg-[var(--bg-surface)] rounded-lg p-4">
        <router-link
          to="/solver-installer"
          class="flex items-center justify-between p-3 rounded-lg border border-[var(--border-subtle)] hover:bg-[var(--bg-hover)] transition-colors group"
        >
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-blue-500 to-indigo-600 flex items-center justify-center">
              <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
              </svg>
            </div>
            <div>
              <div class="text-sm font-medium">{{ t('installer.title') }}</div>
              <div class="text-xs text-[var(--text-secondary)]">LAMMPS, Quantum ESPRESSO, FiPy, CalculiX</div>
            </div>
          </div>
          <svg class="w-4 h-4 text-[var(--text-secondary)] group-hover:text-[var(--text-primary)] transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
        </router-link>
      </div>
    </div>

    <!-- 开发者选项 -->
    <div class="mb-6">
      <h3 class="text-lg font-medium mb-3">开发者选项</h3>
      <div class="bg-[var(--bg-surface)] rounded-lg p-4">
        <router-link
          to="/developer"
          class="flex items-center justify-between p-3 rounded-lg border border-[var(--border-subtle)] hover:bg-[var(--bg-hover)] transition-colors group"
        >
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-green-500 to-emerald-600 flex items-center justify-center">
              <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
              </svg>
            </div>
            <div>
              <div class="text-sm font-medium">REST API 开放平台</div>
              <div class="text-xs text-[var(--text-secondary)]">管理 API 服务器、API Key、查看文档</div>
            </div>
          </div>
          <svg class="w-4 h-4 text-[var(--text-secondary)] group-hover:text-[var(--text-primary)] transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
        </router-link>
      </div>
    </div>

    <!-- 关于 -->
    <div>
      <h3 class="text-lg font-medium mb-3">关于</h3>
      <div class="bg-[var(--bg-surface)] rounded-lg p-4">
        <p class="text-sm"><strong>CAELab</strong> - 创作驱动的科研与工程一体化平台</p>
        <p class="text-sm text-[var(--text-secondary)] mt-1">版本: 0.3.0</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAiStore } from '@/stores/ai'
import type { AiConfig } from '@/stores/ai'
import HotkeySettings from '@/components/settings/HotkeySettings.vue'

const { t } = useI18n()
const aiStore = useAiStore()

// AI配置 - 响应式
const aiConfig = reactive<AiConfig>({
  aiSource: aiStore.config.aiSource,
  apiUrl: aiStore.config.apiUrl,
  apiKey: aiStore.config.apiKey,
  modelName: aiStore.config.modelName,
  temperature: aiStore.config.temperature,
  maxTokens: aiStore.config.maxTokens
})

// 同步初始值
onMounted(() => {
  aiStore.loadConfig()
  Object.assign(aiConfig, aiStore.config)
})

// 保存AI配置
function saveAiConfig() {
  aiStore.updateConfig({
    aiSource: aiConfig.aiSource,
    apiUrl: aiConfig.apiUrl,
    apiKey: aiConfig.apiKey,
    modelName: aiConfig.modelName,
    temperature: aiConfig.temperature,
    maxTokens: aiConfig.maxTokens
  })
  aiStore.saveConfig()
  alert('AI配置已保存！')
}

// 重置AI配置
function resetAiConfig() {
  aiStore.resetConfig()
  Object.assign(aiConfig, aiStore.config)
  alert('AI配置已重置为默认值')
}
</script>
