<template>
  <aside class="h-full bg-[var(--bg-surface)] border-l border-[var(--border-subtle)] flex flex-col">
    <!-- Panel Header -->
    <div class="px-4 py-3 border-b border-[var(--border-subtle)] flex items-center justify-between">
      <span class="text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider">
        {{ panelTitle }}
      </span>
      <button class="icon-btn" @click="$emit('close')">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
        </svg>
      </button>
    </div>
    
    <!-- Panel Content -->
    <div class="flex-1 overflow-y-auto p-4">
      <!-- Dynamic content based on current route -->
      <component :is="panelContentComponent" />
    </div>
  </aside>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import NotesPanel from './panels/NotesPanel.vue'
import ModelingPanel from './panels/ModelingPanel.vue'
import CodePanel from './panels/CodePanel.vue'
import SimulationPanel from './panels/SimulationPanel.vue'

defineEmits(['close'])

const route = useRoute()

const panelTitle = computed(() => {
  const pathMap: Record<string, string> = {
    '/notes': '笔记属性',
    '/modeling': '建模属性',
    '/code': '代码属性',
    '/simulation': '仿真参数'
  }
  return pathMap[route.path] || '属性'
})

const panelContentComponent = computed(() => {
  const componentMap: Record<string, any> = {
    '/notes': NotesPanel,
    '/modeling': ModelingPanel,
    '/code': CodePanel,
    '/simulation': SimulationPanel
  }
  return componentMap[route.path] || NotesPanel
})
</script>

<style scoped>
/* Panel-specific styles */
</style>