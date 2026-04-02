<template>
  <div class="hotkey-settings space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-medium flex items-center gap-2">
        <span>&#x2328;&#xFE0F;</span>
        快捷键设置
      </h3>
      <button
        @click="handleResetAll"
        class="text-xs px-3 py-1.5 rounded border border-[var(--border-subtle)] text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] transition"
      >
        全部重置
      </button>
    </div>

    <!-- Search -->
    <div class="relative">
      <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-[var(--text-muted)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8" />
        <path d="m21 21-4.35-4.35" />
      </svg>
      <input
        v-model="searchQuery"
        type="text"
        placeholder="搜索快捷键..."
        class="w-full pl-9 pr-3 py-2 rounded-lg border border-[var(--border-subtle)] bg-[var(--bg-base)] text-sm text-[var(--text-primary)] placeholder:text-[var(--text-muted)] focus:outline-none focus:ring-2 focus:ring-[var(--primary)]"
      />
    </div>

    <!-- Categories -->
    <div v-for="category in filteredCategories" :key="category" class="space-y-2">
      <div class="flex items-center gap-2 mb-2">
        <span class="text-sm">{{ getCategoryIcon(category) }}</span>
        <h4 class="text-sm font-semibold text-[var(--text-primary)]">{{ getCategoryLabel(category) }}</h4>
        <span class="text-xs text-[var(--text-muted)]">
          ({{ getHotkeysByCategory(category).length }})
        </span>
      </div>

      <div class="bg-[var(--bg-surface)] rounded-lg border border-[var(--border-subtle)] divide-y divide-[var(--border-subtle)]">
        <div
          v-for="hk in getHotkeysByCategory(category)"
          :key="hk.id"
          class="flex items-center justify-between px-4 py-2.5"
        >
          <!-- Description -->
          <span class="text-sm text-[var(--text-primary)]">{{ hk.description }}</span>

          <!-- Key binding -->
          <div class="flex items-center gap-2">
            <!-- Capturing mode -->
            <template v-if="isCapturing && capturingId === hk.id">
              <div class="flex items-center gap-1">
                <kbd
                  class="hotkey-capturing"
                  :class="{ 'hotkey-conflict': captureConflict }"
                >
                  {{ capturedKeys || '按下快捷键...' }}
                </kbd>
                <span v-if="captureConflict" class="text-xs text-red-500">
                  冲突
                </span>
              </div>
              <button
                @click="stopCapture"
                class="text-xs text-[var(--text-muted)] hover:text-[var(--text-primary)] transition"
              >
                取消
              </button>
            </template>

            <!-- Display mode -->
            <template v-else>
              <kbd class="hotkey-badge">
                {{ formatKeys(getEffectiveKeys(hk.id)) }}
              </kbd>
              <button
                @click="startCapture(hk.id)"
                class="text-xs text-[var(--primary)] hover:underline transition"
                title="自定义快捷键"
              >
                自定义
              </button>
              <button
                v-if="customKeys[hk.id]"
                @click="resetHotkey(hk.id)"
                class="text-xs text-[var(--text-muted)] hover:text-[var(--text-primary)] transition"
                title="重置为默认"
              >
                重置
              </button>
            </template>
          </div>
        </div>
      </div>
    </div>

    <!-- No results -->
    <div v-if="filteredCategories.length === 0" class="text-center py-8">
      <p class="text-sm text-[var(--text-muted)]">未找到匹配的快捷键</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onUnmounted } from 'vue'
import { useHotkeys } from '@/composables/useHotkeys'
import {
  defaultHotkeys,
  getCategoryLabel,
  getCategoryIcon,
  type HotkeyConfig,
} from '@/utils/defaultHotkeys'

const {
  isCapturing,
  capturedKeys,
  capturingId,
  customKeys,
  startCapture,
  stopCapture,
  customizeHotkey,
  resetHotkey,
  resetAllHotkeys,
  getEffectiveKeys,
  isHotkeyConflict,
  getConflictInfo,
} = useHotkeys()

const searchQuery = ref('')

// ============ 过滤 ============

const allCategories = ['global', 'simulation', 'modeling', 'notes', 'code'] as const

const filteredCategories = computed(() => {
  if (!searchQuery.value.trim()) return [...allCategories]

  const query = searchQuery.value.toLowerCase()
  return allCategories.filter((cat) => {
    const hotkeys = defaultHotkeys.filter((hk) => hk.category === cat)
    return hotkeys.some(
      (hk) =>
        hk.description.toLowerCase().includes(query) ||
        hk.id.toLowerCase().includes(query) ||
        getEffectiveKeys(hk.id).toLowerCase().includes(query)
    )
  })
})

function getHotkeysByCategory(category: string): HotkeyConfig[] {
  let hotkeys = defaultHotkeys.filter((hk) => hk.category === category)

  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase()
    hotkeys = hotkeys.filter(
      (hk) =>
        hk.description.toLowerCase().includes(query) ||
        hk.id.toLowerCase().includes(query) ||
        getEffectiveKeys(hk.id).toLowerCase().includes(query)
    )
  }

  return hotkeys
}

// ============ 捕获冲突检测 ============

const captureConflict = computed(() => {
  if (!isCapturing.value || !capturedKeys.value || !capturingId.value) return false
  return isHotkeyConflict(capturedKeys.value, capturingId.value)
})

// ============ 捕获完成处理 ============

watch(capturedKeys, (newKeys) => {
  if (newKeys && capturingId.value) {
    // 延迟一点以让用户看到按键
    setTimeout(() => {
      if (!newKeys) return

      const conflict = getConflictInfo(newKeys, capturingId.value!)
      if (conflict) {
        // 有冲突，不自动应用，让用户决定
        return
      }

      // 无冲突，应用
      customizeHotkey(capturingId.value!, newKeys)
      stopCapture()
    }, 300)
  }
})

// ============ 操作 ============

function handleResetAll() {
  if (confirm('确定要将所有快捷键重置为默认值吗？')) {
    resetAllHotkeys()
  }
}

// ============ 工具 ============

function formatKeys(keys: string): string {
  if (!keys) return '-'
  return keys
    .split('+')
    .map((k) => {
      const labels: Record<string, string> = {
        ctrl: 'Ctrl',
        alt: 'Alt',
        shift: 'Shift',
        meta: 'Cmd',
        space: 'Space',
        escape: 'Esc',
        delete: 'Del',
        enter: 'Enter',
        tab: 'Tab',
        backspace: 'Back',
        arrowup: '\u2191',
        arrowdown: '\u2193',
        arrowleft: '\u2190',
        arrowright: '\u2192',
        ',': ',',
        '/': '/',
        '.': '.',
      }
      return labels[k] || k.toUpperCase()
    })
    .join(' + ')
}

// 清理
onUnmounted(() => {
  if (isCapturing.value) {
    stopCapture()
  }
})
</script>

<style scoped>
.hotkey-badge {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  padding: 2px 8px;
  border-radius: 4px;
  background: var(--bg-elevated, #f1f5f9);
  border: 1px solid var(--border-subtle, #e2e8f0);
  font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
  font-size: 11px;
  font-weight: 500;
  color: var(--text-primary, #0f172a);
  min-width: 60px;
  justify-content: center;
  white-space: nowrap;
}

.hotkey-capturing {
  display: inline-flex;
  align-items: center;
  padding: 2px 8px;
  border-radius: 4px;
  background: var(--bg-elevated, #f1f5f9);
  border: 2px solid var(--primary, #4f46e5);
  font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
  font-size: 11px;
  font-weight: 500;
  color: var(--text-primary, #0f172a);
  min-width: 80px;
  justify-content: center;
  animation: pulse-border 1s ease-in-out infinite;
}

.hotkey-conflict {
  border-color: #ef4444;
  color: #ef4444;
}

@keyframes pulse-border {
  0%, 100% {
    border-color: var(--primary, #4f46e5);
  }
  50% {
    border-color: var(--primary-hover, #4338ca);
  }
}
</style>
