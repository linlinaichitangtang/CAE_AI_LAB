import { ref, computed, onMounted, onUnmounted } from 'vue'

const isOnline = ref(navigator.onLine)
const isOfflineMode = ref(false)

export function useOfflineMode() {
  const statusText = computed(() => {
    if (!isOnline.value) return '离线模式'
    if (isOfflineMode.value) return '离线模式（已启用）'
    return '在线'
  })

  const statusColor = computed(() => {
    if (!isOnline.value) return '#ef4444' // red
    if (isOfflineMode.value) return '#f59e0b' // amber
    return '#22c55e' // green
  })

  function toggleOfflineMode() {
    isOfflineMode.value = !isOfflineMode.value
    localStorage.setItem('caelab-offline-mode', String(isOfflineMode.value))
  }

  function onOnline() { isOnline.value = true }
  function onOffline() { isOnline.value = false }

  onMounted(() => {
    isOfflineMode.value = localStorage.getItem('caelab-offline-mode') === 'true'
    window.addEventListener('online', onOnline)
    window.addEventListener('offline', onOffline)
  })

  onUnmounted(() => {
    window.removeEventListener('online', onOnline)
    window.removeEventListener('offline', onOffline)
  })

  return { isOnline, isOfflineMode, statusText, statusColor, toggleOfflineMode }
}
