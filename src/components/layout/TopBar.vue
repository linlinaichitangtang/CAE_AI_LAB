<template>
  <header class="h-12 bg-[var(--bg-surface)] border-b border-[var(--border-default)] flex items-center px-4 gap-3">
    <!-- Logo & Product Name -->
    <div class="flex items-center gap-2.5">
      <div class="w-7 h-7 rounded-md bg-gradient-to-br from-[var(--primary)] to-[var(--primary-hover)] flex items-center justify-center shadow-sm">
        <svg class="w-4 h-4 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 2L2 7l10 5 10-5-10-5z"/>
          <path d="M2 17l10 5 10-5"/>
          <path d="M2 12l10 5 10-5"/>
        </svg>
      </div>
      <span class="text-sm font-semibold text-[var(--text-primary)] tracking-tight">CAELab</span>
    </div>

    <!-- Divider -->
    <div class="w-px h-5 bg-[var(--border-default)]"></div>

    <!-- Breadcrumb / Current Module -->
    <div class="flex items-center gap-1.5 text-sm">
      <span class="text-[var(--text-muted)]">/</span>
      <span class="text-[var(--text-primary)] font-medium">{{ currentModule }}</span>
    </div>

    <!-- Divider -->
    <div class="w-px h-5 bg-[var(--border-default)]"></div>

    <!-- Undo / Redo -->
    <div class="flex items-center gap-0.5">
      <button
        class="topbar-action"
        :class="{ 'topbar-action--disabled': !canUndo }"
        :disabled="!canUndo"
        :title="undoTooltip"
        @click="undo"
      >
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="1 4 1 10 7 10"/>
          <path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10"/>
        </svg>
      </button>
      <button
        class="topbar-action"
        :class="{ 'topbar-action--disabled': !canRedo }"
        :disabled="!canRedo"
        :title="redoTooltip"
        @click="redo"
      >
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="23 4 23 10 17 10"/>
          <path d="M20.49 15a9 9 0 1 1-2.13-9.36L23 10"/>
        </svg>
      </button>
    </div>

    <!-- Spacer -->
    <div class="flex-1"></div>

    <!-- Layout Switcher (Compact Tabs) -->
    <div class="flex items-center gap-1 bg-[var(--bg-elevated)] rounded-md p-0.5">
      <button 
        v-for="layout in layouts" 
        :key="layout.id"
        class="layout-tab"
        :class="{ 'active': currentLayout === layout.id }"
        :title="layout.name"
        @click="$emit('change-layout', layout.id)"
      >
        <component :is="layout.icon" class="w-3.5 h-3.5" />
      </button>
    </div>

    <!-- Layout Badge -->
    <div class="hidden lg:flex items-center gap-1.5 px-2.5 py-1 bg-[var(--bg-elevated)] rounded-md">
      <span class="text-xs text-[var(--text-secondary)]">{{ layoutDisplayName }}</span>
    </div>

    <!-- Action Buttons -->
    <div class="flex items-center gap-0.5">
      <!-- Notifications -->
      <button class="topbar-action" title="通知">
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/>
          <path d="M13.73 21a2 2 0 0 1-3.46 0"/>
        </svg>
        <span class="notification-dot"></span>
      </button>

      <!-- Help -->
      <button class="topbar-action" title="帮助文档" @click="router.push('/help')">
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <circle cx="12" cy="12" r="10"/>
          <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/>
          <line x1="12" y1="17" x2="12.01" y2="17"/>
        </svg>
      </button>

      <!-- Language Switcher -->
      <LanguageSwitcher />

      <!-- Settings -->
      <button class="topbar-action" title="设置">
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <circle cx="12" cy="12" r="3"/>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
        </svg>
      </button>

      <!-- Divider -->
      <div class="w-px h-5 bg-[var(--border-default)] mx-1"></div>

      <!-- User Menu -->
      <div class="relative" ref="userMenuRef">
        <button class="user-avatar" @click="toggleUserMenu">
          <span class="text-xs font-medium">龚</span>
        </button>
        
        <!-- Dropdown -->
        <Transition name="dropdown">
          <div v-if="showUserMenu" class="user-dropdown">
            <div class="dropdown-header">
              <div class="w-8 h-8 rounded-full bg-[var(--primary)] flex items-center justify-center">
                <span class="text-white text-sm font-medium">龚</span>
              </div>
              <div class="flex-1 min-w-0">
                <div class="text-sm font-medium text-[var(--text-primary)] truncate">龚肇麟</div>
                <div class="text-xs text-[var(--text-muted)] truncate">Free Plan</div>
              </div>
            </div>
            <div class="dropdown-divider"></div>
            <button class="dropdown-item" @click="handleMenuAction('profile')">
              <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
                <circle cx="12" cy="7" r="4"/>
              </svg>
              个人资料
            </button>
            <button class="dropdown-item" @click="handleMenuAction('license')">
              <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
              </svg>
              许可证
            </button>
            <div class="dropdown-divider"></div>
            <button class="dropdown-item text-[var(--accent-red)]" @click="handleMenuAction('logout')">
              <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>
                <polyline points="16 17 21 12 16 7"/>
                <line x1="21" y1="12" x2="9" y2="12"/>
              </svg>
              退出登录
            </button>
          </div>
        </Transition>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useUndo } from '@/composables/useUndo'
import { useAuthStore } from '@/stores/authStore'
import LanguageSwitcher from '@/components/common/LanguageSwitcher.vue'

const props = defineProps<{
  currentLayout: string
}>()

defineEmits(['change-layout', 'toggle-sidebar', 'toggle-right-panel'])

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()
const showUserMenu = ref(false)
const userMenuRef = ref<HTMLElement | null>(null)

// Undo / Redo
const { canUndo, canRedo, lastUndoDescription, lastRedoDescription, undo, redo } = useUndo()

const undoTooltip = computed(() => {
  const desc = lastUndoDescription
  return desc ? `撤销: ${desc} (Ctrl+Z)` : '撤销 (Ctrl+Z)'
})

const redoTooltip = computed(() => {
  const desc = lastRedoDescription
  return desc ? `重做: ${desc} (Ctrl+Shift+Z)` : '重做 (Ctrl+Shift+Z)'
})

// SVG Icon Components for Layout Switcher
const FocusIcon = {
  template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
    <rect x="3" y="3" width="18" height="18" rx="2"/>
    <circle cx="12" cy="12" r="3"/>
  </svg>`
}

const SideIcon = {
  template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
    <rect x="3" y="3" width="8" height="18" rx="1"/>
    <rect x="13" y="3" width="8" height="18" rx="1"/>
  </svg>`
}

const TriIcon = {
  template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
    <rect x="3" y="3" width="5" height="18" rx="1"/>
    <rect x="10" y="3" width="11" height="18" rx="1"/>
  </svg>`
}

const QuadIcon = {
  template: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
    <rect x="3" y="3" width="8" height="8" rx="1"/>
    <rect x="13" y="3" width="8" height="8" rx="1"/>
    <rect x="3" y="13" width="8" height="8" rx="1"/>
    <rect x="13" y="13" width="8" height="8" rx="1"/>
  </svg>`
}

const layouts = [
  { id: 'focus', name: '专注模式', icon: FocusIcon },
  { id: 'side', name: '并排模式', icon: SideIcon },
  { id: 'tri', name: '三分模式', icon: TriIcon },
  { id: 'quad', name: '四分模式', icon: QuadIcon }
]

const currentModule = computed(() => {
  const pathMap: Record<string, string> = {
    '/': '首页',
    '/notes': '笔记',
    '/modeling': '建模',
    '/code': '代码',
    '/simulation': '仿真',
    '/fatigue': '疲劳分析',
    '/transient': '瞬态分析',
    '/explicit': '显式动力学',
    '/cfd': 'CFD',
    '/thermal': '热耦合',
    '/topology': '拓扑优化',
    '/comparison': '结果对比',
    '/settings': '设置'
  }
  return pathMap[route.path] || 'CAELab'
})

const layoutDisplayName = computed(() => {
  const layoutMap: Record<string, string> = {
    'focus': '专注模式',
    'side': '并排模式',
    'tri': '三分模式',
    'quad': '四分模式'
  }
  return layoutMap[props.currentLayout] || '三分模式'
})

function toggleUserMenu() {
  showUserMenu.value = !showUserMenu.value
}

function handleMenuAction(action: string) {
  showUserMenu.value = false
  if (action === 'profile') {
    router.push('/profile')
  } else if (action === 'license') {
    router.push('/membership')
  } else if (action === 'logout') {
    authStore.logout()
    router.push('/login')
  }
}

function handleClickOutside(event: MouseEvent) {
  if (userMenuRef.value && !userMenuRef.value.contains(event.target as Node)) {
    showUserMenu.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.layout-tab {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 24px;
  border-radius: 4px;
  color: var(--text-muted);
  transition: all var(--duration-fast) var(--ease-out);
  cursor: pointer;
  border: none;
  background: transparent;
}

.layout-tab:hover {
  color: var(--text-secondary);
  background: var(--bg-hover);
}

.layout-tab.active {
  color: var(--primary);
  background: var(--primary-glow);
}

.topbar-action {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  transition: all var(--duration-fast) var(--ease-out);
  cursor: pointer;
  border: none;
  background: transparent;
}

.topbar-action:hover {
  color: var(--text-primary);
  background: var(--bg-elevated);
}

.topbar-action--disabled {
  color: var(--text-muted);
  opacity: 0.4;
  cursor: not-allowed;
  pointer-events: none;
}

.notification-dot {
  position: absolute;
  top: 6px;
  right: 6px;
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--accent-red);
  border: 1.5px solid var(--bg-surface);
}

.user-avatar {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: var(--radius-md);
  background: var(--primary-glow);
  color: var(--primary);
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  border: none;
  transition: all var(--duration-fast) var(--ease-out);
}

.user-avatar:hover {
  background: var(--primary);
  color: white;
}

.user-dropdown {
  position: absolute;
  top: calc(100% + 8px);
  right: 0;
  width: 220px;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  z-index: 100;
  overflow: hidden;
}

.dropdown-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: var(--bg-elevated);
}

.dropdown-divider {
  height: 1px;
  background: var(--border-subtle);
}

.dropdown-item {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 10px 16px;
  color: var(--text-primary);
  font-size: 13px;
  text-align: left;
  background: transparent;
  border: none;
  cursor: pointer;
  transition: background var(--duration-fast);
}

.dropdown-item:hover {
  background: var(--bg-elevated);
}

.dropdown-item.text-\[var\(--accent-red\)\] {
  color: var(--accent-red);
}

/* Dropdown animation */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all var(--duration-fast) var(--ease-out);
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
