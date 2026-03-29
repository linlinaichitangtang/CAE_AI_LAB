<template>
  <header class="h-12 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)] flex items-center px-4 gap-4">
    <!-- Logo & Title -->
    <div class="flex items-center gap-3">
      <div class="w-8 h-8 rounded-lg bg-[var(--primary)] flex items-center justify-center">
        <span class="text-white font-bold text-sm">CA</span>
      </div>
      <span class="text-sm font-semibold text-[var(--text-primary)] hidden md:inline">CAELab</span>
    </div>
    
    <!-- Breadcrumb -->
    <div class="flex items-center gap-2 text-sm">
      <span class="text-[var(--text-muted)]">/</span>
      <span class="text-[var(--text-primary)]">{{ currentModule }}</span>
    </div>
    
    <!-- Spacer -->
    <div class="flex-1"></div>
    
    <!-- Layout Switcher -->
    <div class="flex items-center gap-1 bg-[var(--bg-elevated)] rounded-lg p-1">
      <button
        v-for="layout in layouts"
        :key="layout.id"
        class="icon-btn w-8 h-8"
        :class="{ 'active': currentLayout === layout.id }"
        :title="layout.name"
        @click="$emit('change-layout', layout.id)"
      >
        <span class="text-sm">{{ layout.icon }}</span>
      </button>
    </div>
    
    <!-- Current Layout Badge -->
    <div class="layout-badge hidden md:flex">
      <span>{{ layoutDisplayName }}</span>
    </div>
    
    <!-- Right Actions -->
    <div class="flex items-center gap-2">
      <!-- Toggle Sidebar -->
      <button class="icon-btn" @click="$emit('toggle-sidebar')" title="切换侧边栏">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
        </svg>
      </button>
      
      <!-- Toggle Right Panel -->
      <button class="icon-btn" @click="$emit('toggle-right-panel')" title="切换属性面板">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17V7m0 10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2h2a2 2 0 012 2m0 10a2 2 0 002 2h2a2 2 0 002-2M9 7a2 2 0 012-2h2a2 2 0 012 2m0 10V7m0 10a2 2 0 002 2h2a2 2 0 002-2V7a2 2 0 00-2-2h-2a2 2 0 00-2 2"/>
        </svg>
      </button>
      
      <!-- More Menu -->
      <button class="icon-btn">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z"/>
        </svg>
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'

const props = defineProps<{
  currentLayout: string
}>()

defineEmits(['change-layout', 'toggle-sidebar', 'toggle-right-panel'])

const route = useRoute()

const layouts = [
  { id: 'focus', icon: '◉', name: '专注模式' },
  { id: 'side', icon: '◧', name: '并排模式' },
  { id: 'tri', icon: '⊞', name: '三分模式' },
  { id: 'quad', icon: '⊟', name: '四分模式' }
]

const currentModule = computed(() => {
  const pathMap: Record<string, string> = {
    '/': '首页',
    '/notes': '笔记',
    '/modeling': '建模',
    '/code': '代码',
    '/simulation': '仿真',
    '/settings': '设置'
  }
  return pathMap[route.path] || 'CAELab'
})

const layoutDisplayName = computed(() => {
  const layoutMap: Record<string, string> = {
    'focus': '专注',
    'side': '并排',
    'tri': '三分',
    'quad': '四分'
  }
  return layoutMap[props.currentLayout] || '三分'
})
</script>

<style scoped>
/* Component-specific styles */
</style>