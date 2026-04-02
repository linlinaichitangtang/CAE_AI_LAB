<template>
  <footer class="desktop-only h-8 bg-[var(--bg-surface)] border-t border-[var(--border-default)] flex items-center px-4 text-xs gap-4">
    <!-- Solver Status -->
    <div class="flex items-center gap-2">
      <span class="status-indicator" :class="solverStatusClass"></span>
      <span class="text-[var(--text-secondary)]">{{ solverStatusText }}</span>
    </div>

    <!-- Auto Save Status -->
    <div class="flex items-center gap-1.5 text-[var(--text-muted)]" :title="autoSaveTooltip">
      <svg v-if="!isAutoSaving" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
        <polyline points="17 21 17 13 7 13 7 21"/>
        <polyline points="7 3 7 8 15 8"/>
      </svg>
      <svg v-else class="w-3.5 h-3.5 animate-spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
      </svg>
      <span>{{ autoSaveText }}</span>
    </div>

    <!-- Progress Bar (shown when solving) -->
    <div v-if="isSolving" class="flex items-center gap-2 flex-1 max-w-xs">
      <div class="progress-bar">
        <div class="progress-fill" :style="{ width: solveProgress + '%' }"></div>
      </div>
      <span class="text-[var(--text-muted)] w-10">{{ solveProgress }}%</span>
    </div>

    <!-- Current Time Step -->
    <div v-if="isSolving" class="flex items-center gap-1 text-[var(--text-muted)]">
      <span>步:</span>
      <span class="text-[var(--text-secondary)] font-medium">{{ currentStep }}</span>
      <span>/</span>
      <span>{{ totalSteps }}</span>
    </div>

    <!-- Spacer -->
    <div class="flex-1"></div>

    <!-- Right Section Indicators -->
    <div class="flex items-center gap-4">
      <!-- Network Status (Offline Mode Indicator) -->
      <div class="flex items-center gap-1.5 cursor-pointer select-none" @click="toggleOfflineMode" :title="statusText + '（点击切换离线模式）'">
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M1.42 9a16.06 16.06 0 0 1 21.16 0"/>
          <path d="M5 12.55a11 11 0 0 1 14.08 0"/>
          <path d="M8.53 16.11a6 6 0 0 1 6.95 0"/>
          <circle cx="12" cy="20" r="1" fill="currentColor"/>
        </svg>
        <span class="text-[var(--text-muted)]">{{ statusText }}</span>
      </div>

      <!-- Memory Usage -->
      <div class="flex items-center gap-1.5 text-[var(--text-muted)]" :title="'内存: ' + memoryUsage">
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <rect x="2" y="6" width="20" height="12" rx="2"/>
          <path d="M6 12h.01M10 12h.01"/>
        </svg>
        <span>{{ memoryUsage }}</span>
      </div>

      <!-- GPU Status -->
      <div class="flex items-center gap-1.5 text-[var(--text-muted)]" :title="'GPU: ' + gpuStatus">
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <rect x="2" y="4" width="20" height="16" rx="2"/>
          <path d="M6 8h.01M10 8h.01M14 8h.01M18 8h.01"/>
          <path d="M6 12h12"/>
        </svg>
        <span>{{ gpuStatus }}</span>
      </div>

      <!-- CPU Load -->
      <div class="flex items-center gap-1.5 text-[var(--text-muted)]" title="CPU 负载">
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <rect x="4" y="4" width="16" height="16" rx="2"/>
          <rect x="9" y="9" width="6" height="6"/>
          <path d="M9 1v3M15 1v3M9 20v3M15 20v3M1 9h3M20 9h3M1 15h3M20 15h3"/>
        </svg>
        <span>{{ cpuLoad }}</span>
      </div>

      <!-- Divider -->
      <div class="w-px h-3.5 bg-[var(--border-default)]"></div>

      <!-- Current Time -->
      <div class="text-[var(--text-muted)]">
        {{ currentTime }}
      </div>

      <!-- Divider -->
      <div class="w-px h-3.5 bg-[var(--border-default)]"></div>

      <!-- Version -->
      <span class="text-[var(--text-muted)]">v0.2.0</span>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useAutoSave } from '@/composables/useAutoSave'
import { useOfflineMode } from '@/composables/useOfflineMode'

// Auto save state
const { lastSaveTime, isAutoSaving } = useAutoSave()

// Offline mode state
const { statusText, toggleOfflineMode } = useOfflineMode()

const autoSaveText = computed(() => {
  if (isAutoSaving.value) return '保存中...'
  if (lastSaveTime.value) {
    const diff = Date.now() - lastSaveTime.value.getTime()
    const sec = Math.floor(diff / 1000)
    if (sec < 60) return '已保存'
    const min = Math.floor(sec / 60)
    return `${min}分钟前保存`
  }
  return '自动保存'
})

const autoSaveTooltip = computed(() => {
  if (isAutoSaving.value) return '正在自动保存...'
  if (lastSaveTime.value) {
    return `最后保存: ${lastSaveTime.value.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: false })}`
  }
  return '自动保存未启用'
})

// Solver state
const isSolving = ref(false)
const solveProgress = ref(0)
const currentStep = ref(0)
const totalSteps = ref(100)
const solverStatus = ref<'idle' | 'solving' | 'complete' | 'error'>('idle')

// System metrics (mock data - replace with real data in production)
const memoryUsage = ref('2.1 GB')
const gpuStatus = ref('空闲')
const cpuLoad = ref('23%')
const currentTime = ref('')

const solverStatusClass = computed(() => ({
  'status-idle': solverStatus.value === 'idle',
  'status-solving': solverStatus.value === 'solving',
  'status-complete': solverStatus.value === 'complete',
  'status-error': solverStatus.value === 'error'
}))

const solverStatusText = computed(() => {
  const statusMap = {
    'idle': '就绪',
    'solving': '求解中...',
    'complete': '求解完成',
    'error': '求解错误'
  }
  return statusMap[solverStatus.value]
})

// Update time every second
function updateTime() {
  const now = new Date()
  currentTime.value = now.toLocaleTimeString('zh-CN', { 
    hour: '2-digit', 
    minute: '2-digit',
    second: '2-digit',
    hour12: false 
  })
}

// Simulate solver progress (for demo - remove in production)
let solveInterval: ReturnType<typeof setInterval> | null = null

function startSolveDemo() {
  if (solveInterval) return
  solverStatus.value = 'solving'
  isSolving.value = true
  solveProgress.value = 0
  currentStep.value = 0
  
  solveInterval = setInterval(() => {
    if (solveProgress.value < 100) {
      solveProgress.value += Math.random() * 5
      currentStep.value = Math.floor(solveProgress.value)
      if (solveProgress.value >= 100) {
        solveProgress.value = 100
        currentStep.value = totalSteps.value
        solverStatus.value = 'complete'
        setTimeout(() => {
          isSolving.value = false
        }, 1500)
        if (solveInterval) {
          clearInterval(solveInterval)
          solveInterval = null
        }
      }
    }
  }, 200)
}

let timeInterval: ReturnType<typeof setInterval> | null = null

onMounted(() => {
  updateTime()
  timeInterval = setInterval(updateTime, 1000)
  
  // Demo: Click on status bar to start solver demo
  document.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  if (timeInterval) clearInterval(timeInterval)
  if (solveInterval) clearInterval(solveInterval)
  document.removeEventListener('keydown', handleKeydown)
})

function handleKeydown(e: KeyboardEvent) {
  // Press 'S' to start solver demo
  if (e.key === 's' && !isSolving.value) {
    startSolveDemo()
  }
}
</script>

<style scoped>
.status-indicator {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

.status-idle {
  background: var(--accent-green);
}

.status-solving {
  background: var(--accent-amber);
  animation: pulse 1.5s infinite;
}

.status-complete {
  background: var(--accent-green);
}

.status-error {
  background: var(--accent-red);
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.4; }
}

.progress-bar {
  width: 100px;
  height: 4px;
  background: var(--bg-elevated);
  border-radius: 2px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--primary) 0%, var(--primary-hover) 100%);
  border-radius: 2px;
  transition: width 0.2s ease-out;
}
</style>
