<template>
  <div class="h-full flex flex-col p-6 overflow-auto">
    <!-- 分享链接导入（保留原有逻辑） -->
    <div v-if="loading && !importMode" class="flex-1 flex items-center justify-center">
      <div class="text-center">
        <div class="w-12 h-12 border-4 border-[var(--primary)] border-t-transparent rounded-full animate-spin mx-auto mb-4"></div>
        <p class="text-[var(--text-secondary)]">正在解析分享链接...</p>
      </div>
    </div>

    <div v-else-if="error && !importMode" class="flex-1 flex items-center justify-center">
      <div class="text-center max-w-md">
        <div class="w-16 h-16 rounded-xl bg-[var(--accent-red)]/20 flex items-center justify-center mx-auto mb-4">
          <span class="text-3xl">❌</span>
        </div>
        <h2 class="text-xl font-semibold text-[var(--text-primary)] mb-2">导入失败</h2>
        <p class="text-[var(--text-secondary)] mb-6">{{ error }}</p>
        <div class="flex gap-3 justify-center">
          <button @click="switchToFileImport" class="btn btn-primary">
            从文件导入
          </button>
          <button @click="goHome" class="btn btn-ghost">
            返回首页
          </button>
        </div>
      </div>
    </div>

    <div v-else-if="success && !importMode" class="flex-1 flex items-center justify-center">
      <div class="text-center max-w-md">
        <div class="w-16 h-16 rounded-xl bg-[var(--primary)]/20 flex items-center justify-center mx-auto mb-4">
          <span class="text-3xl">✅</span>
        </div>
        <h2 class="text-xl font-semibold text-[var(--text-primary)] mb-2">项目导入成功</h2>
        <p class="text-[var(--text-secondary)] mb-6">「{{ projectName }}」已成功导入，现在开始你的创作之旅吧！</p>
        <div class="flex gap-3 justify-center">
          <button @click="goToNotes" class="btn btn-primary">
            前往笔记
          </button>
          <button @click="goHome" class="btn btn-ghost">
            返回首页
          </button>
        </div>
      </div>
    </div>

    <!-- .caelabzip 文件导入模式 -->
    <div v-else-if="importMode" class="flex-1 flex flex-col">
      <!-- 文件选择区域 -->
      <div v-if="!importing && !importDone" class="flex-1 flex items-center justify-center">
        <div class="w-full max-w-lg">
          <div class="text-center mb-8">
            <h2 class="text-2xl font-bold text-[var(--text-primary)] mb-2">导入 CAELab 项目</h2>
            <p class="text-[var(--text-secondary)]">从 .caelabzip 文件导入完整项目数据</p>
          </div>

          <!-- 拖拽上传区域 -->
          <div
            class="border-2 border-dashed border-[var(--border-subtle)] rounded-xl p-12 text-center cursor-pointer hover:border-[var(--primary)] transition-colors"
            :class="{ 'border-[var(--primary)] bg-[var(--primary)]/5': isDragging }"
            @click="triggerFileSelect"
            @dragover.prevent="isDragging = true"
            @dragleave.prevent="isDragging = false"
            @drop.prevent="handleDrop"
          >
            <span class="text-5xl block mb-4">📦</span>
            <p class="text-[var(--text-primary)] mb-2 text-lg">点击或拖拽 .caelabzip 文件到此处</p>
            <p class="text-sm text-[var(--text-muted)]">支持 .caelabzip 格式的 CAELab 项目压缩包</p>
          </div>

          <input
            ref="fileInputRef"
            type="file"
            accept=".caelabzip"
            class="hidden"
            @change="handleFileSelect"
          />

          <!-- 已选文件信息 -->
          <div v-if="selectedFile" class="mt-4 card p-4">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-lg bg-[var(--primary-glow)] flex items-center justify-center flex-shrink-0">
                <span class="text-lg">📦</span>
              </div>
              <div class="flex-1 min-w-0">
                <p class="font-semibold text-[var(--text-primary)] truncate">{{ selectedFile.name }}</p>
                <p class="text-xs text-[var(--text-muted)]">{{ formatFileSize(selectedFile.size) }}</p>
              </div>
              <button @click="selectedFile = null" class="icon-btn text-[var(--text-muted)] hover:text-[var(--accent-red)]">
                ✕
              </button>
            </div>

            <!-- 可选：自定义项目名称 -->
            <div class="mt-4 space-y-3">
              <div>
                <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2 uppercase tracking-wider">项目名称（可选）</label>
                <input
                  v-model="importOptions.name"
                  type="text"
                  class="input w-full"
                  placeholder="留空则使用原项目名称"
                />
              </div>
              <div>
                <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2 uppercase tracking-wider">项目描述（可选）</label>
                <textarea
                  v-model="importOptions.description"
                  rows="2"
                  class="input w-full resize-none"
                  placeholder="留空则使用原项目描述"
                ></textarea>
              </div>
            </div>

            <button
              @click="startImport"
              class="btn btn-primary w-full mt-4"
            >
              开始导入
            </button>
          </div>

          <div class="text-center mt-6">
            <button @click="goHome" class="btn btn-ghost">
              返回首页
            </button>
          </div>
        </div>
      </div>

      <!-- 导入进度 -->
      <div v-else-if="importing" class="flex-1 flex items-center justify-center">
        <div class="w-full max-w-md text-center">
          <div class="w-16 h-16 rounded-xl bg-[var(--primary)]/20 flex items-center justify-center mx-auto mb-6">
            <div class="w-8 h-8 border-4 border-[var(--primary)] border-t-transparent rounded-full animate-spin"></div>
          </div>
          <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-2">{{ importProgress.message }}</h3>

          <!-- 进度条 -->
          <div class="w-full bg-[var(--bg-elevated)] rounded-full h-2 mt-4 mb-2">
            <div
              class="bg-[var(--primary)] h-2 rounded-full transition-all duration-300"
              :style="{ width: progressPercent + '%' }"
            ></div>
          </div>
          <p class="text-sm text-[var(--text-muted)]">
            {{ importProgress.current }} / {{ importProgress.total }}
          </p>

          <!-- 阶段指示器 -->
          <div class="flex justify-center gap-2 mt-6">
            <div
              v-for="stage in importStages"
              :key="stage.key"
              class="flex items-center gap-1 text-xs"
              :class="getStageClass(stage.key)"
            >
              <span>{{ stage.icon }}</span>
              <span>{{ stage.label }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 导入完成 -->
      <div v-else-if="importDone" class="flex-1 flex items-center justify-center">
        <div class="text-center max-w-md">
          <div class="w-16 h-16 rounded-xl bg-[var(--primary)]/20 flex items-center justify-center mx-auto mb-4">
            <span class="text-3xl">✅</span>
          </div>
          <h2 class="text-xl font-semibold text-[var(--text-primary)] mb-2">项目导入成功</h2>
          <p class="text-[var(--text-secondary)] mb-4">
            「{{ projectName }}」已成功导入
          </p>

          <!-- 导入统计 -->
          <div v-if="importStats" class="card p-4 mb-6 text-left">
            <h4 class="text-sm font-semibold text-[var(--text-secondary)] uppercase tracking-wider mb-3">导入统计</h4>
            <div class="grid grid-cols-2 gap-3">
              <div class="flex items-center gap-2">
                <span>📄</span>
                <span class="text-sm text-[var(--text-primary)]">文件: {{ importStats.filesRestored }}</span>
              </div>
              <div class="flex items-center gap-2">
                <span>📜</span>
                <span class="text-sm text-[var(--text-primary)]">版本历史: {{ importStats.versionsRestored }}</span>
              </div>
              <div class="flex items-center gap-2">
                <span>🔗</span>
                <span class="text-sm text-[var(--text-primary)]">双向链接: {{ importStats.linksRestored }}</span>
              </div>
              <div class="flex items-center gap-2">
                <span>📎</span>
                <span class="text-sm text-[var(--text-primary)]">嵌入记录: {{ importStats.embedsRestored }}</span>
              </div>
            </div>
          </div>

          <div class="flex gap-3 justify-center">
            <button @click="goToNotes" class="btn btn-primary">
              前往笔记
            </button>
            <button @click="goHome" class="btn btn-ghost">
              返回首页
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 默认入口：选择导入方式 -->
    <div v-else class="flex-1 flex items-center justify-center">
      <div class="text-center max-w-md">
        <div class="w-20 h-20 rounded-2xl bg-[var(--bg-surface)] flex items-center justify-center mb-6">
          <span class="text-5xl">📦</span>
        </div>
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-2">导入 CAELab 项目</h3>
        <p class="text-[var(--text-secondary)] mb-6">选择导入方式</p>
        <div class="space-y-3">
          <button @click="switchToFileImport" class="btn btn-primary w-full flex items-center justify-center gap-2">
            <span>📦</span>
            <span>从 .caelabzip 文件导入</span>
          </button>
          <button @click="goHome" class="btn btn-ghost w-full">
            返回首页
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { parseShareLink, importProjectFromZip, type ImportProgress } from '@/api/share'

const router = useRouter()
const route = useRoute()

// State
const loading = ref(true)
const error = ref('')
const success = ref(false)
const projectName = ref('')
const importMode = ref(false)
const importing = ref(false)
const importDone = ref(false)

// File import state
const fileInputRef = ref<HTMLInputElement | null>(null)
const selectedFile = ref<File | null>(null)
const isDragging = ref(false)
const importOptions = ref({
  name: '',
  description: ''
})

// Progress state
const importProgress = ref<ImportProgress>({
  stage: 'validating',
  current: 0,
  total: 1,
  message: '准备中...'
})
const importStats = ref<{
  filesRestored: number
  versionsRestored: number
  linksRestored: number
  embedsRestored: number
} | null>(null)

const importStages = [
  { key: 'validating', icon: '🔍', label: '验证' },
  { key: 'creating', icon: '📁', label: '创建' },
  { key: 'files', icon: '📄', label: '文件' },
  { key: 'versions', icon: '📜', label: '版本' },
  { key: 'links', icon: '🔗', label: '链接' },
  { key: 'embeds', icon: '📎', label: '嵌入' },
  { key: 'done', icon: '✅', label: '完成' }
]

const progressPercent = computed(() => {
  if (importProgress.value.total === 0) return 0
  return Math.round((importProgress.value.current / importProgress.value.total) * 100)
})

function getStageClass(stageKey: string): string {
  const stageOrder = ['validating', 'creating', 'files', 'versions', 'links', 'embeds', 'done']
  const currentIndex = stageOrder.indexOf(importProgress.value.stage)
  const stageIndex = stageOrder.indexOf(stageKey)

  if (stageIndex < currentIndex) {
    return 'text-[var(--primary)]'
  } else if (stageIndex === currentIndex) {
    return 'text-[var(--text-primary)] font-semibold'
  }
  return 'text-[var(--text-muted)]'
}

function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

onMounted(async () => {
  const data = route.query.data as string

  if (data) {
    // 尝试解析分享链接
    const projectId = parseShareLink(`${window.location.origin}/share?data=${data}`)

    if (!projectId) {
      error.value = '无效的分享链接格式'
      loading.value = false
      return
    }

    // 分享链接功能需要后端服务器支持
    error.value = '分享链接功能需要后端服务器支持存储项目数据。请使用 .caelabzip 文件导入。'
    loading.value = false
  } else {
    // 没有分享链接数据，直接进入文件导入模式
    loading.value = false
    importMode.value = true
  }
})

function switchToFileImport() {
  importMode.value = true
  error.value = ''
  loading.value = false
}

function triggerFileSelect() {
  fileInputRef.value?.click()
}

function handleFileSelect(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (file) {
    selectedFile.value = file
    importOptions.value = { name: '', description: '' }
  }
  // Reset input
  target.value = ''
}

function handleDrop(event: DragEvent) {
  isDragging.value = false
  const file = event.dataTransfer?.files?.[0]
  if (file && file.name.endsWith('.caelabzip')) {
    selectedFile.value = file
    importOptions.value = { name: '', description: '' }
  } else if (file) {
    alert('请选择 .caelabzip 格式的文件')
  }
}

async function startImport() {
  if (!selectedFile.value) return

  importing.value = true
  importDone.value = false
  importStats.value = null

  try {
    const result = await importProjectFromZip(selectedFile.value, {
      name: importOptions.value.name || undefined,
      description: importOptions.value.description || undefined,
      onProgress: (progress) => {
        importProgress.value = progress
      }
    })

    projectName.value = result.project.name
    importStats.value = result.stats
    importing.value = false
    importDone.value = true
    success.value = true
  } catch (err) {
    importing.value = false
    importMode.value = true
    selectedFile.value = null
    alert('导入失败: ' + (err as Error).message)
  }
}

function goHome() {
  router.push('/')
}

function goToNotes() {
  router.push('/notes')
}
</script>
