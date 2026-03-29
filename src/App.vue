<template>
  <div id="app" class="h-screen flex flex-col bg-[var(--bg-base)]">
    <!-- Top Bar -->
    <TopBar 
      :current-layout="currentLayout"
      @change-layout="changeLayout"
      @toggle-sidebar="toggleSidebar"
      @toggle-right-panel="toggleRightPanel"
    />
    
    <!-- Main Content Area -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Navigation -->
      <LeftNav 
        v-if="showSidebar"
        :class="['transition-all duration-300', showSidebar ? 'w-[72px]' : 'w-0']"
      />
      
      <!-- Workspace Area -->
      <main class="flex-1 flex flex-col overflow-hidden" :class="rightPanelClass">
        <!-- Main Workspace with Optional Pane Split -->
        <div class="flex-1 flex overflow-hidden" v-if="currentLayout === 'quad' || currentLayout === 'split'">
          <!-- Primary Pane -->
          <div class="flex-1 overflow-hidden border-r border-[var(--border-subtle)]">
            <router-view />
          </div>
          <!-- Secondary Pane -->
          <div class="flex-1 overflow-hidden">
            <component :is="secondaryComponent" />
          </div>
        </div>
        
        <!-- Single View (Focus, Side-by-Side, Tri-Panel) -->
        <div v-else class="flex-1 flex overflow-hidden">
          <div class="flex-1 overflow-hidden">
            <router-view />
          </div>
        </div>
        
        <!-- Optional Right Panel (for Tri-Panel and Side-by-Side) -->
        <RightPanel 
          v-if="showRightPanel && (currentLayout === 'tri' || currentLayout === 'split')"
          :class="['transition-all duration-300', showRightPanel ? 'w-[280px]' : 'w-0']"
        />
      </main>
    </div>
    
    <!-- Status Bar -->
    <StatusBar />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRoute } from 'vue-router'
import TopBar from './components/layout/TopBar.vue'
import LeftNav from './components/layout/LeftNav.vue'
import RightPanel from './components/layout/RightPanel.vue'
import StatusBar from './components/layout/StatusBar.vue'
import CodeView from './views/CodeView.vue'
import NotesView from './views/NotesView.vue'
import ModelingView from './views/ModelingView.vue'
import SimulationView from './views/SimulationView.vue'

// Layout mode: 'focus' | 'side' | 'tri' | 'quad'
const currentLayout = ref<string>(
  localStorage.getItem('caelab_layout') || 'tri'
)

// Panel visibility
const showSidebar = ref(true)
const showRightPanel = ref(true)

// Current route for determining secondary view in quad/split mode
const route = useRoute()

// Compute secondary component based on current route
const secondaryComponent = computed(() => {
  const path = route.path
  if (path === '/notes') return CodeView
  if (path === '/code') return NotesView
  if (path === '/modeling') return SimulationView
  if (path === '/simulation') return ModelingView
  return CodeView // default
})

// Right panel class
const rightPanelClass = computed(() => {
  if (currentLayout.value === 'quad' || currentLayout.value === 'split') {
    return 'flex-row'
  }
  return ''
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

// Watch for route changes to sync layout
watch(route, () => {
  // Could sync layout based on route if needed
})
</script>

<style scoped>
/* Additional app-specific styles if needed */
</style>