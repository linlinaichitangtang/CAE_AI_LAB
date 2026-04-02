import { computed } from 'vue'
import { usePlatform } from './usePlatform'

/**
 * 移动端功能裁剪
 * 定义在移动端哪些功能可用/不可用
 */
export function useMobileFeatures() {
  const { isTouchDevice, isMobile } = usePlatform()

  // 移动端可用的功能
  const availableFeatures = computed(() => ({
    // 查看结果 - 完全可用
    viewResults: true,
    // 笔记 - 完全可用
    notes: true,
    // 轻量仿真 - 可用（简化模式）
    lightSimulation: true,
    // 建模 - 简化（仅基本几何体）
    basicModeling: true,
    // 代码编辑 - 简化
    codeEditor: true,

    // 以下功能在移动端不可用或受限
    fullSimulation: !isTouchDevice.value,  // 完整仿真仅桌面端
    cfdAnalysis: !isTouchDevice.value,      // CFD 仅桌面端
    topologyOptimization: !isTouchDevice.value, // 拓扑优化仅桌面端
    explicitDynamics: !isTouchDevice.value, // 显式动力学仅桌面端
    advancedModeling: !isTouchDevice.value, // 高级建模仅桌面端
    parameterScan: !isTouchDevice.value,   // 参数化扫描仅桌面端
    scriptAutomation: !isTouchDevice.value, // 脚本自动化仅桌面端
    comparison: !isTouchDevice.value,       // 结果对比仅桌面端
  }))

  // 移动端导航项（精简版）
  const mobileNavItems = computed(() => [
    { path: '/', label: '首页', icon: 'home' },
    { path: '/simulation', label: '仿真', icon: 'box' },
    { path: '/modeling', label: '建模', icon: 'cube' },
    { path: '/notes', label: '笔记', icon: 'file-text' },
  ])

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

  const navItems = computed(() =>
    isTouchDevice.value ? mobileNavItems.value : desktopNavItems.value
  )

  return {
    availableFeatures,
    mobileNavItems,
    desktopNavItems,
    navItems,
  }
}
