<template>
  <div id="app" class="h-screen flex flex-col bg-[var(--bg-base)]">
    <!-- Splash Screen -->
    <SplashScreen :duration="1500" />

    <!-- Auth Page: full-screen without layout -->
    <template v-if="isAuthPage">
      <router-view />
    </template>

    <!-- Main App Layout -->
    <template v-else>
      <!-- 桌面端顶栏 -->
      <TopBar
        v-if="isDesktop"
        :current-layout="currentLayout"
        @change-layout="changeLayout"
        @toggle-sidebar="toggleSidebar"
        @toggle-right-panel="toggleRightPanel"
      />

      <!-- 试用 Banner -->
      <div
        v-if="trialStore.isInTrial && trialStore.daysRemaining > 0"
        class="trial-banner"
      >
        <span class="trial-icon">⚡</span>
        <span class="trial-text">
          CAELab Pro 试用剩余 <strong>{{ trialStore.daysRemaining }} 天</strong>
        </span>
        <button class="trial-btn" @click="router.push('/membership')">升级</button>
        <button class="trial-close" @click="trialStore.markBannerDismissed()">✕</button>
      </div>
      <div
        v-else-if="trialStore.showExpiredBanner"
        class="trial-banner trial-expired"
      >
        <span class="trial-icon">⏰</span>
        <span class="trial-text">
          您的 CAELab Pro 试用已结束，高级功能已暂停使用
        </span>
        <button class="trial-btn" @click="router.push('/membership')">立即升级</button>
        <button class="trial-close" @click="trialStore.dismissExpiredBanner()">✕</button>
      </div>

      <!-- 移动端/平板端简化顶栏 -->
      <div v-else class="mobile-topbar safe-area-top">
        <span class="font-semibold text-sm">{{ currentRouteName }}</span>
      </div>

      <!-- 主内容区 -->
      <div class="flex-1 flex overflow-hidden">
        <!-- 桌面端左侧导航 -->
        <LeftNav
          v-if="isDesktop && showSidebar"
          :class="['transition-all duration-300', showSidebar ? 'w-[72px]' : 'w-0']"
        />

        <!-- 主视图 -->
        <main class="flex-1 flex flex-col overflow-hidden" :class="{ 'pb-14': isTouchDevice }">
          <!-- Main Workspace with Optional Pane Split -->
          <div class="flex-1 flex overflow-hidden" v-if="isDesktop && (currentLayout === 'quad' || currentLayout === 'split')">
            <!-- Primary Pane -->
            <div class="flex-1 overflow-hidden border-r border-[var(--border-subtle)]">
              <PageTransition />
            </div>
            <!-- Secondary Pane -->
            <div class="flex-1 overflow-hidden">
              <component :is="secondaryComponent" />
            </div>
          </div>

          <!-- Single View (Focus, Side-by-Side, Tri-Panel, or mobile/tablet) -->
          <div v-else class="flex-1 flex overflow-hidden">
            <div class="flex-1 overflow-hidden">
              <PageTransition />
            </div>
          </div>

          <!-- Optional Right Panel (for Tri-Panel and Side-by-Side, desktop only) -->
          <RightPanel
            v-if="isDesktop && showRightPanel && (currentLayout === 'tri' || currentLayout === 'split')"
            :class="['transition-all duration-300', showRightPanel ? 'w-[280px]' : 'w-0']"
          />
        </main>
      </div>

      <!-- 桌面端状态栏 -->
      <StatusBar v-if="isDesktop" />

      <!-- 移动端底部导航 -->
      <MobileBottomNav v-if="isTouchDevice" />
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import TopBar from './components/layout/TopBar.vue'
import LeftNav from './components/layout/LeftNav.vue'
import RightPanel from './components/layout/RightPanel.vue'
import StatusBar from './components/layout/StatusBar.vue'
import MobileBottomNav from './components/layout/MobileBottomNav.vue'
import PageTransition from './components/common/PageTransition.vue'
import SplashScreen from './components/common/SplashScreen.vue'
import CodeView from './views/CodeView.vue'
import NotesView from './views/NotesView.vue'
import ModelingView from './views/ModelingView.vue'
import SimulationView from './views/SimulationView.vue'
import { usePlatform } from './composables/usePlatform'
import { useOrientation } from './composables/useOrientation'
import { useAuthStore } from './stores/authStore'
import { useTrialStore } from './stores/trial'
import { useHotkeys } from './composables/useHotkeys'

// Platform detection for responsive layout
const { isMobile, isTablet, isDesktop, isTouchDevice } = usePlatform()

// Orientation detection for adaptive layout
const { isPortrait, isLandscape } = useOrientation()

// Auth store
const authStore = useAuthStore()

// Trial store
const trialStore = useTrialStore()

// V1.1-011: Hotkeys
const { registerHotkey, setActiveContext } = useHotkeys()
const router = useRouter()

// Layout mode: 'focus' | 'side' | 'tri' | 'quad'
const currentLayout = ref<string>(
  localStorage.getItem('caelab_layout') || 'tri'
)

// Panel visibility
const showSidebar = ref(true)
const showRightPanel = ref(true)

// Current route for determining secondary view in quad/split mode
const route = useRoute()

// Check if current page is auth page (no layout)
const isAuthPage = computed(() => {
  return route.path === '/login'
})

// Current route name for mobile topbar
const currentRouteName = computed(() => {
  const nameMap: Record<string, string> = {
    '/': '首页',
    '/notes': '笔记',
    '/modeling': '建模',
    '/code': '代码',
    '/simulation': '仿真',
    '/fatigue': '疲劳',
    '/transient': '瞬态',
    '/explicit': '显式',
    '/cfd': 'CFD',
    '/thermal': '热耦合',
    '/topology': '拓扑',
    '/settings': '设置',
    '/more': '更多',
    '/membership': '会员',
    '/profile': '个人资料',
  }
  return nameMap[route.path] || 'CAELab'
})

// Compute secondary component based on current route
const secondaryComponent = computed(() => {
  const path = route.path
  if (path === '/notes') return CodeView
  if (path === '/code') return NotesView
  if (path === '/modeling') return SimulationView
  if (path === '/simulation') return ModelingView
  return CodeView // default
})

// Change layout mode
function changeLayout(layout: string) {
  currentLayout.value = layout
  localStorage.setItem('caelab_layout', layout)

  // Auto-adjust panel visibility based on layout
  switch (layout) {
    case 'focus':
      showSidebar.value = true
      showRightPanel.value = false
      break
    case 'side':
      showSidebar.value = true
      showRightPanel.value = false
      break
    case 'tri':
      showSidebar.value = true
      showRightPanel.value = true
      break
    case 'quad':
      showSidebar.value = true
      showRightPanel.value = false
      break
  }
}

// Toggle panels
function toggleSidebar() {
  showSidebar.value = !showSidebar.value
}

function toggleRightPanel() {
  showRightPanel.value = !showRightPanel.value
}

// Route-to-hotkey-context mapping
const routeContextMap: Record<string, string> = {
  '/notes': 'notes',
  '/modeling': 'modeling',
  '/code': 'code',
  '/simulation': 'simulation',
  '/fatigue': 'simulation',
  '/transient': 'simulation',
  '/explicit': 'simulation',
  '/cfd': 'simulation',
  '/thermal': 'simulation',
  '/topology': 'simulation',
}

// Watch for route changes to update hotkey context
watch(route, (newRoute) => {
  const context = routeContextMap[newRoute.path] || 'global'
  setActiveContext(context)
})

// Register global hotkeys
onMounted(() => {
  authStore.init()

  // Global: Ctrl+S -> Save (prevent default browser save)
  registerHotkey({
    id: 'global.save',
    keys: 'ctrl+s',
    description: '保存',
    category: 'global',
    action: () => {
      // Dispatch a custom event that views can listen to
      window.dispatchEvent(new CustomEvent('caelab:save'))
    },
    enabled: true,
  })

  // Global: Ctrl+, -> Settings
  registerHotkey({
    id: 'global.settings',
    keys: 'ctrl+,',
    description: '设置',
    category: 'global',
    action: () => {
      router.push('/settings')
    },
    enabled: true,
  })

  // Global: Ctrl+K -> Search
  registerHotkey({
    id: 'global.search',
    keys: 'ctrl+k',
    description: '搜索',
    category: 'global',
    action: () => {
      window.dispatchEvent(new CustomEvent('caelab:search'))
    },
    enabled: true,
  })

  // Global: F1 -> Help
  registerHotkey({
    id: 'global.help',
    keys: 'f1',
    description: '帮助',
    category: 'global',
    action: () => {
      router.push('/help')
    },
    enabled: true,
  })

  // Global: Escape -> Close panel
  registerHotkey({
    id: 'global.close-panel',
    keys: 'escape',
    description: '关闭面板/弹窗',
    category: 'global',
    action: () => {
      window.dispatchEvent(new CustomEvent('caelab:close-panel'))
    },
    enabled: true,
  })

  // Set initial context based on current route
  const initialContext = routeContextMap[route.path] || 'global'
  setActiveContext(initialContext)
})
</script>

<style scoped>
.mobile-topbar {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 44px;
  background: var(--bg-surface, #ffffff);
  border-bottom: 1px solid var(--border-default, #e5e7eb);
  color: var(--text-primary, #1f2937);
  flex-shrink: 0;
}

@media (min-width: 768px) and (max-width: 1023px) {
  .mobile-topbar {
    height: 48px;
  }
}

/* 试用 Banner */
.trial-banner {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 8px 16px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #fff;
  font-size: 13px;
  flex-shrink: 0;
}

.trial-banner.trial-expired {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
}

.trial-icon {
  font-size: 14px;
}

.trial-text {
  flex: 1;
  text-align: center;
}

.trial-text strong {
  font-weight: 700;
  color: #fff;
}

.trial-btn {
  padding: 4px 14px;
  background: rgba(255, 255, 255, 0.2);
  border: 1px solid rgba(255, 255, 255, 0.4);
  border-radius: 6px;
  color: #fff;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.2s;
}

.trial-btn:hover {
  background: rgba(255, 255, 255, 0.35);
}

.trial-close {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: rgba(255, 255, 255, 0.7);
  cursor: pointer;
  border-radius: 4px;
  font-size: 12px;
}

.trial-close:hover {
  background: rgba(255, 255, 255, 0.2);
  color: #fff;
}
</style>
