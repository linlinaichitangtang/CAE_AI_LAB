import { ref, computed, onMounted, onUnmounted } from 'vue'

export type Orientation = 'portrait' | 'landscape'

const orientation = ref<Orientation>(
  typeof window !== 'undefined' && window.innerWidth > window.innerHeight
    ? 'landscape' : 'portrait'
)

export function useOrientation() {
  const isPortrait = computed(() => orientation.value === 'portrait')
  const isLandscape = computed(() => orientation.value === 'landscape')

  function onResize() {
    orientation.value = window.innerWidth > window.innerHeight ? 'landscape' : 'portrait'
  }

  // 也监听 orientationchange 事件（移动端更可靠）
  function onOrientationChange() {
    const screenOrientation = (screen as any).orientation
    if (screenOrientation) {
      orientation.value = screenOrientation.type.includes('portrait') ? 'portrait' : 'landscape'
    } else {
      onResize()
    }
  }

  onMounted(() => {
    window.addEventListener('resize', onResize)
    window.addEventListener('orientationchange', onOrientationChange)
  })

  onUnmounted(() => {
    window.removeEventListener('resize', onResize)
    window.removeEventListener('orientationchange', onOrientationChange)
  })

  return { orientation, isPortrait, isLandscape }
}
