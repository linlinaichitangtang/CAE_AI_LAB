import { ref, computed } from 'vue'

/**
 * 平台检测 composable
 * 检测当前设备类型和输入方式
 */
export function usePlatform() {
  const userAgent = typeof navigator !== 'undefined' ? navigator.userAgent : ''

  const isTouchDevice = ref(
    typeof window !== 'undefined' && (
      'ontouchstart' in window ||
      navigator.maxTouchPoints > 0 ||
      // iPadOS 13+ 桌面模式检测
      (navigator.platform === 'MacIntel' && navigator.maxTouchPoints > 1)
    )
  )

  const isMobile = computed(() => {
    if (typeof window === 'undefined') return false
    return window.innerWidth < 768 || /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(userAgent)
  })

  const isTablet = computed(() => {
    if (typeof window === 'undefined') return false
    const width = window.innerWidth
    return (width >= 768 && width < 1024) || /iPad|Android(?!.*Mobile)/i.test(userAgent)
  })

  const isDesktop = computed(() => !isMobile.value && !isTablet.value)

  const isIOS = computed(() => /iPhone|iPad|iPod/i.test(userAgent))
  const isAndroid = computed(() => /Android/i.test(userAgent))

  const isApplePencilSupported = computed(() => {
    return isIOS.value && isTouchDevice.value
  })

  return {
    isTouchDevice,
    isMobile,
    isTablet,
    isDesktop,
    isIOS,
    isAndroid,
    isApplePencilSupported,
  }
}
