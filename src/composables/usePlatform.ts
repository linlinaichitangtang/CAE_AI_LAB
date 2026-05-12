import { ref, computed } from 'vue'

export type OSType = 'windows' | 'macos' | 'linux' | 'ios' | 'android' | 'harmony' | 'openharmony' | 'unknown'
export type DeviceType = 'phone' | 'tablet' | 'laptop' | 'desktop' | 'foldable'
export type FoldableState = 'unfolded' | 'folded' | 'flat' | 'unknown'

/**
 * 平台检测 composable
 * 检测当前设备类型和输入方式
 * 支持 iOS/Android/鸿蒙NEXT/纯血鸿蒙
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

  // 纯血鸿蒙 NEXT 检测
  const isHarmony = computed(() => {
    const ua = userAgent.toLowerCase()
    // 鸿蒙NEXT特征: HarmonyOS NEXT 或 hmos 但不含 Android
    const isHarmonyUA = ua.includes('harmony') || ua.includes('hmos')
    const isNotAndroid = !ua.includes('android')
    return isHarmonyUA && isNotAndroid
  })

  // Android 检测（排除鸿蒙）
  const isAndroid = computed(() => {
    const ua = userAgent.toLowerCase()
    return ua.includes('android') && !ua.includes('harmony') && !ua.includes('hmos')
  })

  const isIOS = computed(() => /iPhone|iPad|iPod/i.test(userAgent))

  const isOpenHarmony = computed(() => {
    // 纯血鸿蒙 OpenHarmony 特征
    const ua = userAgent.toLowerCase()
    return ua.includes('openharmony') || ua.includes('ohos')
  })

  // Apple Pencil 检测
  const isApplePencilSupported = computed(() => {
    return isIOS.value && isTouchDevice.value
  })

  // 鸿蒙 M-Pencil 检测
  const isHarmonyPencilSupported = computed(() => {
    return isHarmony.value && isTouchDevice.value
  })

  /**
   * 检测操作系统类型
   */
  function detectOS(): OSType {
    const ua = userAgent.toLowerCase()
    if (ua.includes('openharmony') || ua.includes('ohos')) return 'openharmony'
    if (ua.includes('harmony') || ua.includes('hmos')) return 'harmony'
    if (/iphone|ipad|ipod/.test(ua)) return 'ios'
    if (/android/.test(ua)) return 'android'
    if (/windows/.test(ua)) return 'windows'
    if (/macintosh|mac os x/.test(ua)) return 'macos'
    if (/linux/.test(ua)) return 'linux'
    return 'unknown'
  }

  /**
   * 检测设备类型（针对鸿蒙设备细化）
   */
  function detectDeviceType(): DeviceType {
    const ua = userAgent.toLowerCase()
    const screenWidth = typeof window !== 'undefined' ? window.innerWidth : 0

    // 折叠屏检测
    if (ua.includes('fold') || ua.includes('mate x') || ua.includes('p50') || ua.includes('pocket')) {
      return 'foldable'
    }

    // 手机
    if (/iphone|huawei.*mate|huawei.*p|huawei.*nova/i.test(ua)) {
      return 'phone'
    }

    // 平板
    if (/ipad|huawei.*matepad|huawei.*tab/i.test(ua) || (screenWidth >= 768 && isTouchDevice.value)) {
      return 'tablet'
    }

    // 笔记本电脑
    if (/huawei.*matebook|macbook/i.test(ua)) {
      return 'laptop'
    }

    // 桌面
    if (!isTouchDevice.value) {
      return 'desktop'
    }

    return 'tablet'
  }

  const os = detectOS()
  const deviceType = detectDeviceType()

  // 屏幕尺寸相关
  const screenWidth = typeof window !== 'undefined' ? ref(window.innerWidth) : ref(0)
  const screenHeight = typeof window !== 'undefined' ? ref(window.innerHeight) : ref(0)
  const aspectRatio = computed(() => {
    if (screenHeight.value === 0) return 0
    return screenWidth.value / screenHeight.value
  })

  return {
    isTouchDevice,
    isMobile,
    isTablet,
    isDesktop,
    isIOS,
    isAndroid,
    isHarmony,
    isOpenHarmony,
    isApplePencilSupported,
    isHarmonyPencilSupported,
    os,
    deviceType,
    detectOS,
    detectDeviceType,
    screenWidth,
    screenHeight,
    aspectRatio,
  }
}
