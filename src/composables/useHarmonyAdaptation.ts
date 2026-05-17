/**
 * CAELab 纯血鸿蒙 (OpenHarmony) 适配
 * 支持手机、平板、折叠屏
 */
import { ref, computed, watch, onMounted } from 'vue'
import { usePlatform, type DeviceType, type FoldableState } from './usePlatform'

export type HarmonyDeviceCategory = 'phone' | 'tablet' | 'foldable' | 'laptop' | 'desktop'

export interface HarmonyDisplayInfo {
  category: HarmonyDeviceCategory
  foldableState: FoldableState
  screenWidth: number
  screenHeight: number
  isLargeScreen: boolean
  supportsMultiWindow: boolean
  supportsMpencil: boolean
}

export interface HarmonyWindowConfig {
  maxWidth: number
  maxHeight: number
  isLandscape: boolean
  isSplitScreen: boolean
}

const displayInfo = ref<HarmonyDisplayInfo>({
  category: 'phone',
  foldableState: 'unknown',
  screenWidth: 375,
  screenHeight: 812,
  isLargeScreen: false,
  supportsMultiWindow: false,
  supportsMpencil: false
})

const windowConfig = ref<HarmonyWindowConfig>({
  maxWidth: 375,
  maxHeight: 812,
  isLandscape: false,
  isSplitScreen: false
})

const isPureHarmony = ref(false)
const openHarmonyVersion = ref<string | null>(null)

export function useHarmonyAdaptation() {
  const { isHarmony, isOpenHarmony, isHarmonyPencilSupported, deviceType, screenWidth, screenHeight, isTouchDevice } = usePlatform()

  /**
   * 初始化鸿蒙设备信息
   */
  const initHarmonyDevice = () => {
    const ua = navigator.userAgent.toLowerCase()

    // 检测纯血鸿蒙
    isPureHarmony.value = isOpenHarmony.value || isHarmony.value

    // 提取 OpenHarmony 版本
    const ohMatch = ua.match(/openharmony[\s\/]?([\d.]+)/i)
    if (ohMatch) {
      openHarmonyVersion.value = ohMatch[1]
    }

    // 检测折叠屏状态
    const foldableStates: FoldableState[] = ['folded', 'unfolded', 'flat']
    const currentFoldableState = foldableStates.find(state => ua.includes(state)) || 'unknown'

    // 确定设备类别
    let category: HarmonyDeviceCategory = 'phone'
    if (deviceType === 'foldable') {
      category = 'foldable'
    } else if (deviceType === 'laptop') {
      category = 'laptop'
    } else if (deviceType === 'tablet') {
      category = 'tablet'
    } else if (!isTouchDevice.value) {
      category = 'desktop'
    } else if (window.innerWidth >= 768) {
      category = 'tablet'
    }

    displayInfo.value = {
      category,
      foldableState: currentFoldableState,
      screenWidth: window.innerWidth,
      screenHeight: window.innerHeight,
      isLargeScreen: window.innerWidth >= 768,
      supportsMultiWindow: true, // 鸿蒙支持多窗口
      supportsMpencil: isHarmonyPencilSupported.value
    }

    // 更新窗口配置
    updateWindowConfig()
  }

  /**
   * 更新窗口配置
   */
  const updateWindowConfig = () => {
    const width = window.innerWidth
    const height = window.innerHeight

    windowConfig.value = {
      maxWidth: width,
      maxHeight: height,
      isLandscape: width > height,
      isSplitScreen: width < height * 0.8
    }
  }

  /**
   * 获取针对当前设备的布局配置
   */
  const getLayoutConfig = computed(() => {
    const info = displayInfo.value
    const win = windowConfig.value

    // 手机布局
    if (info.category === 'phone') {
      return {
        navigation: 'bottom' as const,
        sidePanel: false,
        maxContentWidth: 375,
        showLabels: false,
        compactMode: true
      }
    }

    // 折叠屏展开态
    if (info.category === 'foldable' && info.foldableState === 'unfolded') {
      return {
        navigation: 'side' as const,
        sidePanel: true,
        maxContentWidth: 1200,
        showLabels: true,
        compactMode: false
      }
    }

    // 折叠屏折叠态
    if (info.category === 'foldable' && info.foldableState === 'folded') {
      return {
        navigation: 'bottom' as const,
        sidePanel: false,
        maxContentWidth: 375,
        showLabels: false,
        compactMode: true
      }
    }

    // 平板横屏
    if (info.category === 'tablet' && win.isLandscape) {
      return {
        navigation: 'side' as const,
        sidePanel: true,
        maxContentWidth: 1024,
        showLabels: true,
        compactMode: false
      }
    }

    // 平板竖屏
    if (info.category === 'tablet' && !win.isLandscape) {
      return {
        navigation: 'bottom' as const,
        sidePanel: false,
        maxContentWidth: 768,
        showLabels: true,
        compactMode: false
      }
    }

    // 笔记本/桌面
    return {
      navigation: 'side' as const,
      sidePanel: true,
      maxContentWidth: 1400,
      showLabels: true,
      compactMode: false
    }
  })

  /**
   * 获取触控笔配置
   */
  const getPencilConfig = computed(() => ({
    enabled: displayInfo.value.supportsMpencil,
    pressureEnabled: true,
    tiltEnabled: true,
    palmRejection: true,
    // 鸿蒙 M-Pencil 特有的压感曲线
    pressureCurve: 'natural'
  }))

  /**
   * 获取多窗口配置
   */
  const getMultiWindowConfig = computed(() => ({
    supported: displayInfo.value.supportsMultiWindow,
    minWidth: displayInfo.value.category === 'phone' ? 320 : 480,
    minHeight: displayInfo.value.category === 'phone' ? 480 : 320,
    isCurrentlySplit: windowConfig.value.isSplitScreen
  }))

  /**
   * 监听屏幕变化
   */
  const setupResizeListener = () => {
    window.addEventListener('resize', () => {
      displayInfo.value.screenWidth = window.innerWidth
      displayInfo.value.screenHeight = window.innerHeight
      displayInfo.value.isLargeScreen = window.innerWidth >= 768
      updateWindowConfig()
    })

    // 监听折叠状态变化
    window.matchMedia('(orientation: landscape)').addEventListener('change', () => {
      updateWindowConfig()
    })
  }

  onMounted(() => {
    if (isPureHarmony.value) {
      initHarmonyDevice()
      setupResizeListener()
    }
  })

  const isHarmonyDevice = computed(() => isPureHarmony.value)

  return {
    // 状态
    isPureHarmony: isHarmonyDevice,
    isOpenHarmony,
    displayInfo: computed(() => displayInfo.value),
    windowConfig: computed(() => windowConfig.value),
    layoutConfig: getLayoutConfig,
    pencilConfig: getPencilConfig,
    multiWindowConfig: getMultiWindowConfig,

    // 方法
    initHarmonyDevice,
    updateWindowConfig,
  }
}