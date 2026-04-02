<template>
  <div v-if="visible" class="fixed inset-0 bg-black/40 flex items-center justify-center z-50">
    <div class="bg-white dark:bg-gray-800 rounded-xl shadow-2xl p-6 w-80 space-y-4">
      <!-- 标题 -->
      <div class="flex items-center justify-between">
        <h3 class="text-sm font-semibold text-[var(--text-primary)]">{{ message || '网格加载中' }}</h3>
        <button
          v-if="cancelable"
          @click="$emit('cancel')"
          class="text-xs text-gray-400 hover:text-red-500 transition-colors"
        >
          取消
        </button>
      </div>

      <!-- 进度条 -->
      <div class="space-y-1">
        <div class="flex justify-between text-xs text-[var(--text-secondary)]">
          <span>加载进度</span>
          <span>{{ progress.percentage.toFixed(1) }}%</span>
        </div>
        <div class="w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
          <div
            class="h-full bg-[var(--primary)] rounded-full transition-all duration-300 ease-out"
            :style="{ width: progress.percentage + '%' }"
          ></div>
        </div>
      </div>

      <!-- 统计信息 -->
      <div class="grid grid-cols-2 gap-2 text-xs">
        <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-2">
          <div class="text-[var(--text-muted)]">已加载块</div>
          <div class="font-mono text-[var(--text-primary)]">
            {{ progress.loaded }} / {{ progress.total }}
          </div>
        </div>
        <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-2">
          <div class="text-[var(--text-muted)]">节点数</div>
          <div class="font-mono text-[var(--text-primary)]">
            {{ formatNumber(progress.loadedNodes) }}
          </div>
        </div>
        <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-2">
          <div class="text-[var(--text-muted)]">单元数</div>
          <div class="font-mono text-[var(--text-primary)]">
            {{ formatNumber(progress.loadedElements) }}
          </div>
        </div>
        <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-2">
          <div class="text-[var(--text-muted)]">预计剩余</div>
          <div class="font-mono text-[var(--text-primary)]">
            {{ formatTime(progress.estimatedTimeRemaining) }}
          </div>
        </div>
      </div>

      <!-- 线框骨架预览提示 -->
      <div class="text-center text-[10px] text-[var(--text-muted)]">
        低精度预览已就绪，高精度模型加载中...
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { LoadProgress } from '@/utils/meshStreaming'

interface Props {
  visible: boolean
  progress: LoadProgress
  cancelable?: boolean
  message?: string
}

withDefaults(defineProps<Props>(), {
  cancelable: true,
  message: '',
})

defineEmits<{
  (e: 'cancel'): void
}>()

function formatNumber(n: number): string {
  if (n >= 1000000) return (n / 1000000).toFixed(1) + 'M'
  if (n >= 1000) return (n / 1000).toFixed(1) + 'K'
  return String(n)
}

function formatTime(seconds: number): string {
  if (seconds <= 0) return '--'
  if (seconds < 1) return '<1s'
  if (seconds < 60) return Math.round(seconds) + 's'
  const min = Math.floor(seconds / 60)
  const sec = Math.round(seconds % 60)
  return `${min}m ${sec}s`
}
</script>
