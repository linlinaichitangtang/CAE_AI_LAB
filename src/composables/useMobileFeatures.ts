import { computed } from 'vue'
import { usePlatform } from './usePlatform'

/**
 * 移动端功能裁剪
 * 定义在移动端/平板哪些功能可用/不可用
 * 主要针对平板优化 (iPad / Android Pad / Harmony Pad)
 */
export function useMobileFeatures() {
  const { isTouchDevice, isMobile, isTablet, isIOS, isAndroid, isHarmony } = usePlatform()

  // 平板功能配置（iPad/Android Pad/Harmony Pad）
  const tabletFeatures = computed(() => ({
    viewResults: true,
    notes: true,
    lightSimulation: true,
    basicModeling: true,
    codeEditor: true,
    fullSimulation: true,
    cfdAnalysis: true,
    topologyOptimization: true,
    explicitDynamics: false,
    advancedModeling: true,
    parameterScan: true,
    scriptAutomation: true,
    comparison: true,
    stylusSupport: isIOS.value || isHarmony.value,
    splitScreen: true,
    arPreview: true,
  }))

  // 手机功能配置
  const phoneFeatures = computed(() => ({
    viewResults: true,
    notes: true,
    lightSimulation: true,
    basicModeling: false,
    codeEditor: false,
    fullSimulation: false,
    cfdAnalysis: false,
    topologyOptimization: false,
    explicitDynamics: false,
    advancedModeling: false,
    parameterScan: false,
    scriptAutomation: false,
    comparison: false,
    stylusSupport: false,
    splitScreen: false,
    arPreview: true,
  }))

  // 根据设备类型选择功能集
  const availableFeatures = computed(() => {
    if (isTablet.value) return tabletFeatures.value
    if (isMobile.value) return phoneFeatures.value
    return {
      viewResults: true,
      notes: true,
      lightSimulation: true,
      basicModeling: true,
      codeEditor: true,
      fullSimulation: true,
      cfdAnalysis: true,
      topologyOptimization: true,
      explicitDynamics: true,
      advancedModeling: true,
      parameterScan: true,
      scriptAutomation: true,
      comparison: true,
      stylusSupport: false,
      splitScreen: false,
      arPreview: true,
    }
  })

  // 平板导航项（功能完整）
  const tabletNavItems = computed(() => [
    { path: '/', label: '首页', icon: 'home' },
    { path: '/simulation', label: '仿真', icon: 'layers' },
    { path: '/modeling', label: '建模', icon: 'box' },
    { path: '/notes', label: '笔记', icon: 'file-text' },
    { path: '/code', label: '代码', icon: 'code' },
    { path: '/topology', label: '拓扑', icon: 'grid' },
  ])

  // 手机导航项（精简）
  const mobileNavItems = computed(() => [
    { path: '/', label: '首页', icon: 'home' },
    { path: '/simulation', label: '仿真', icon: 'box' },
    { path: '/notes', label: '笔记', icon: 'file-text' },
  ])

  const navItems = computed(() => {
    if (isTablet.value) return tabletNavItems.value
    if (isMobile.value) return mobileNavItems.value
    return desktopNavItems.value
  })

  // 桌面端完整导航项
  const desktopNavItems = computed(() => [
    { path: '/', label: '首页', icon: 'home' },
    { path: '/notes', label: '笔记', icon: 'file-text' },
    { path: '/modeling', label: '建模', icon: 'box' },
    { path: '/code', label: '代码', icon: 'code' },
    { path: '/simulation', label: '仿真', icon: 'layers' },
    { path: '/fatigue', label: '疲劳', icon: 'activity' },
    { path: '/transient', label: '瞬态', icon: 'zap' },
    { path: '/cfd', label: 'CFD', icon: 'wind' },
    { path: '/thermal', label: '热耦合', icon: 'thermometer' },
    { path: '/topology', label: '拓扑', icon: 'grid' },
  ])

  return {
    availableFeatures,
    tabletFeatures,
    phoneFeatures,
    mobileNavItems,
    tabletNavItems,
    desktopNavItems,
    navItems,
  }
}
