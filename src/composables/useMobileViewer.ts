/**
 * useMobileViewer.ts — V3.3-006 移动端查看器
 * iPad/Android 查看仿真结果、批注、简单操作
 */

import { ref, computed, onMounted, onUnmounted } from 'vue'

// ============ 类型定义 ============

export type DeviceType = 'phone' | 'tablet' | 'desktop'
export type Orientation = 'portrait' | 'landscape'
export type ViewMode = 'results' | 'mesh' | 'model' | 'comments' | 'compare'

export interface MobileConfig {
  enableGestures: boolean
  enableHaptics: boolean
  autoRotate: boolean
  showControls: boolean
  swipeSensitivity: number  // 1-10
  pinchZoomSpeed: number  // 1-10
}

export interface ResultViewport {
  projectId: string
  resultId: string
  position: { x: number; y: number; z: number }
  rotation: { x: number; y: number; z: number }
  zoom: number
  clipPlane?: {
    enabled: boolean
    normal: { x: number; y: number; z: number }
    offset: number
  }
  colorMap: string
  showDeformation: boolean
  deformationScale: number
}

export interface Annotation {
  id: string
  resultId: string
  type: 'point' | 'line' | 'region' | 'text'
  position: { x: number; y: number; z: number }
  content: string
  author: string
  createdAt: string
  color: string
  replies?: AnnotationReply[]
}

export interface AnnotationReply {
  id: string
  author: string
  content: string
  createdAt: string
}

export interface ComparisonSession {
  id: string
  name: string
  resultIds: string[]
  currentIndex: number
  syncViews: boolean
}

// ============ 存储键 ============

const CONFIG_KEY = 'caelab_mobile_config'
const VIEWPORTS_KEY = 'caelab_mobile_viewports'
const ANNOTATIONS_KEY = 'caelab_mobile_annotations'

// ============ 工具函数 ============

function generateId(): string {
  return `mobile_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

// ============ 主 Composable ============

export function useMobileViewer() {
  // 设备信息
  const deviceType = ref<DeviceType>('desktop')
  const orientation = ref<Orientation>('portrait')
  const isMobile = ref(false)
  const isTablet = ref(false)

  // 视图状态
  const viewMode = ref<ViewMode>('results')
  const currentViewport = ref<ResultViewport | null>(null)
  const viewports = ref<ResultViewport[]>([])

  // 移动端配置
  const config = ref<MobileConfig>({
    enableGestures: true,
    enableHaptics: true,
    autoRotate: true,
    showControls: true,
    swipeSensitivity: 5,
    pinchZoomSpeed: 5
  })

  // 批注
  const annotations = ref<Annotation[]>([])

  // 对比会话
  const comparisonSession = ref<ComparisonSession | null>(null)

  // UI 状态
  const showSidebar = ref(false)
  const showToolbar = ref(true)
  const showColorMap = ref(true)
  const isFullscreen = ref(false)

  // 手势状态
  const activeGestures = ref<{
    type: 'none' | 'pan' | 'pinch' | 'rotate' | 'swipe'
    startX: number
    startY: number
    currentX: number
    currentY: number
    scale: number
    rotation: number
  }>({
    type: 'none',
    startX: 0,
    startY: 0,
    currentX: 0,
    currentY: 0,
    scale: 1,
    rotation: 0
  })

  // 触控板状态
  const touchStartTime = ref(0)
  const touchStartDistance = ref(0)

  // ============ 计算属性 ============

  const isLandscape = computed(() => orientation.value === 'landscape')
  const isPortrait = computed(() => orientation.value === 'portrait')

  const viewportStyle = computed(() => {
    if (!currentViewport.value) return {}

    const { position, rotation, zoom } = currentViewport.value

    return {
      transform: `
        translate3d(${position.x}px, ${position.y}px, ${position.z}px)
        rotate3d(${rotation.x}, ${rotation.y}, ${rotation.z}, 0deg)
        scale(${zoom})
      `,
      transition: activeGestures.value.type === 'none' ? 'transform 0.2s ease-out' : 'none'
    }
  })

  // ============ 设备检测 ============

  function detectDevice(): void {
    const width = window.innerWidth
    const height = window.innerHeight

    // 检测设备类型
    const userAgent = navigator.userAgent.toLowerCase()
    const isAndroid = userAgent.includes('android')
    const isIOS = userAgent.includes('iphone') || userAgent.includes('ipad') || userAgent.includes('ipod')

    isMobile.value = isAndroid || isIOS || width < 768
    isTablet.value = (isAndroid || isIOS || width >= 768) && width < 1024

    if (width >= 1024) {
      deviceType.value = 'desktop'
    } else if (width >= 768) {
      deviceType.value = 'tablet'
    } else {
      deviceType.value = 'phone'
    }

    // 检测方向
    orientation.value = width > height ? 'landscape' : 'portrait'
  }

  function handleResize(): void {
    detectDevice()
  }

  // ============ 手势处理 ============

  function handleTouchStart(e: TouchEvent): void {
    if (!config.value.enableGestures) return

    const touch = e.touches[0]
    activeGestures.value.startX = touch.clientX
    activeGestures.value.startY = touch.clientY
    activeGestures.value.currentX = touch.clientX
    activeGestures.value.currentY = touch.clientY

    if (e.touches.length === 2) {
      activeGestures.value.type = 'pinch'
      const dx = e.touches[1].clientX - e.touches[0].clientX
      const dy = e.touches[1].clientY - e.touches[0].clientY
      touchStartDistance.value = Math.sqrt(dx * dx + dy * dy)
    } else if (e.touches.length === 1) {
      activeGestures.value.type = 'pan'
    }

    touchStartTime.value = Date.now()

    // 震动反馈
    if (config.value.enableHaptics && navigator.vibrate) {
      navigator.vibrate(10)
    }
  }

  function handleTouchMove(e: TouchEvent): void {
    if (!config.value.enableGestures || activeGestures.value.type === 'none') return

    const touch = e.touches[0]
    activeGestures.value.currentX = touch.clientX
    activeGestures.value.currentY = touch.clientY

    if (!currentViewport.value) return

    const deltaX = (touch.clientX - activeGestures.value.startX) * config.value.swipeSensitivity * 0.1
    const deltaY = (touch.clientY - activeGestures.value.startY) * config.value.swipeSensitivity * 0.1

    switch (activeGestures.value.type) {
      case 'pan':
        currentViewport.value.position.x += deltaX
        currentViewport.value.position.y += deltaY
        break

      case 'pinch':
        if (e.touches.length === 2) {
          const dx = e.touches[1].clientX - e.touches[0].clientX
          const dy = e.touches[1].clientY - e.touches[0].clientY
          const distance = Math.sqrt(dx * dx + dy * dy)
          const scaleDelta = distance / touchStartDistance.value

          currentViewport.value.zoom = Math.max(0.1, Math.min(10, currentViewport.value.zoom * scaleDelta))
          touchStartDistance.value = distance
        }
        break
    }

    activeGestures.value.startX = touch.clientX
    activeGestures.value.startY = touch.clientY
  }

  function handleTouchEnd(e: TouchEvent): void {
    const duration = Date.now() - touchStartTime.value

    // 检测滑动手势
    if (duration < 300) {
      const deltaX = activeGestures.value.currentX - activeGestures.value.startX
      const deltaY = activeGestures.value.currentY - activeGestures.value.startY

      if (Math.abs(deltaX) > 50 && Math.abs(deltaX) > Math.abs(deltaY)) {
        handleSwipe(deltaX > 0 ? 'left' : 'right')
      } else if (Math.abs(deltaY) > 50 && Math.abs(deltaY) > Math.abs(deltaX)) {
        handleSwipe(deltaY > 0 ? 'up' : 'down')
      }
    }

    // 重置手势状态
    activeGestures.value.type = 'none'
    activeGestures.value.scale = 1
    activeGestures.value.rotation = 0

    saveViewport()
  }

  function handleSwipe(direction: 'up' | 'down' | 'left' | 'right'): void {
    // 震动反馈
    if (config.value.enableHaptics && navigator.vibrate) {
      navigator.vibrate(25)
    }

    switch (direction) {
      case 'left':
        // 下一个结果
        nextResult()
        break
      case 'right':
        // 上一个结果
        previousResult()
        break
      case 'up':
        // 切换视图模式
        cycleViewMode(1)
        break
      case 'down':
        // 打开/关闭侧边栏
        showSidebar.value = !showSidebar.value
        break
    }
  }

  // ============ 鼠标/触控板事件（桌面模拟） ============

  function handleWheel(e: WheelEvent): void {
    if (!currentViewport.value) return

    e.preventDefault()

    const zoomDelta = e.deltaY > 0 ? 0.9 : 1.1
    currentViewport.value.zoom = Math.max(0.1, Math.min(10, currentViewport.value.zoom * zoomDelta))

    saveViewport()
  }

  function handleMouseDown(e: MouseEvent): void {
    if (!config.value.enableGestures) return
    activeGestures.value.type = 'pan'
    activeGestures.value.startX = e.clientX
    activeGestures.value.startY = e.clientY
  }

  function handleMouseMove(e: MouseEvent): void {
    if (activeGestures.value.type !== 'pan' || !currentViewport.value) return

    const deltaX = (e.clientX - activeGestures.value.startX) * config.value.swipeSensitivity * 0.05
    const deltaY = (e.clientY - activeGestures.value.startY) * config.value.swipeSensitivity * 0.05

    currentViewport.value.position.x += deltaX
    currentViewport.value.position.y += deltaY

    activeGestures.value.startX = e.clientX
    activeGestures.value.startY = e.clientY
  }

  function handleMouseUp(): void {
    if (activeGestures.value.type === 'pan') {
      saveViewport()
    }
    activeGestures.value.type = 'none'
  }

  // ============ 视图控制 ============

  function cycleViewMode(direction: 1 | -1 = 1): void {
    const modes: ViewMode[] = ['results', 'mesh', 'model', 'comments', 'compare']
    const currentIndex = modes.indexOf(viewMode.value)
    const newIndex = (currentIndex + direction + modes.length) % modes.length
    viewMode.value = modes[newIndex]
  }

  function nextResult(): void {
    if (comparisonSession.value) {
      comparisonSession.value.currentIndex = (comparisonSession.value.currentIndex + 1) % comparisonSession.value.resultIds.length
    }
  }

  function previousResult(): void {
    if (comparisonSession.value) {
      comparisonSession.value.currentIndex = (comparisonSession.value.currentIndex - 1 + comparisonSession.value.resultIds.length) % comparisonSession.value.resultIds.length
    }
  }

  function resetViewport(): void {
    if (currentViewport.value) {
      currentViewport.value.position = { x: 0, y: 0, z: 0 }
      currentViewport.value.rotation = { x: 0, y: 0, z: 0 }
      currentViewport.value.zoom = 1
    }
  }

  function fitToScreen(): void {
    if (currentViewport.value) {
      currentViewport.value.zoom = 1
      currentViewport.value.position = { x: 0, y: 0, z: 0 }
    }
  }

  function setViewMode(mode: ViewMode): void {
    viewMode.value = mode
  }

  // ============ 批注功能 ============

  function addAnnotation(
    resultId: string,
    type: Annotation['type'],
    position: { x: number; y: number; z: number },
    content: string
  ): Annotation {
    const annotation: Annotation = {
      id: generateId(),
      resultId,
      type,
      position,
      content,
      author: localStorage.getItem('caelab_username') || 'Anonymous',
      createdAt: new Date().toISOString(),
      color: '#3B82F6',
      replies: []
    }

    annotations.value.push(annotation)
    saveAnnotations()

    // 震动反馈
    if (config.value.enableHaptics && navigator.vibrate) {
      navigator.vibrate(30)
    }

    return annotation
  }

  function replyToAnnotation(annotationId: string, content: string): AnnotationReply | null {
    const annotation = annotations.value.find(a => a.id === annotationId)
    if (!annotation) return null

    const reply: AnnotationReply = {
      id: generateId(),
      author: localStorage.getItem('caelab_username') || 'Anonymous',
      content,
      createdAt: new Date().toISOString()
    }

    if (!annotation.replies) {
      annotation.replies = []
    }
    annotation.replies.push(reply)
    saveAnnotations()

    return reply
  }

  function deleteAnnotation(annotationId: string): boolean {
    const index = annotations.value.findIndex(a => a.id === annotationId)
    if (index === -1) return false

    annotations.value.splice(index, 1)
    saveAnnotations()
    return true
  }

  function getResultAnnotations(resultId: string): Annotation[] {
    return annotations.value.filter(a => a.resultId === resultId)
  }

  // ============ 视口管理 ============

  function saveViewport(): void {
    if (!currentViewport.value) return

    const existing = viewports.value.findIndex(v => v.resultId === currentViewport.value!.resultId)
    if (existing >= 0) {
      viewports.value[existing] = { ...currentViewport.value }
    } else {
      viewports.value.push({ ...currentViewport.value })
    }

    setStorage(VIEWPORTS_KEY, viewports.value)
  }

  function loadViewport(resultId: string): ResultViewport | null {
    const viewport = viewports.value.find(v => v.resultId === resultId)
    if (viewport) {
      currentViewport.value = { ...viewport }
      return currentViewport.value
    }

    // 创建新的视口
    const newViewport: ResultViewport = {
      projectId: '',
      resultId,
      position: { x: 0, y: 0, z: 0 },
      rotation: { x: 0, y: 0, z: 0 },
      zoom: 1,
      colorMap: 'viridis',
      showDeformation: false,
      deformationScale: 1
    }

    currentViewport.value = newViewport
    return newViewport
  }

  // ============ 对比功能 ============

  function createComparisonSession(name: string, resultIds: string[]): ComparisonSession {
    const session: ComparisonSession = {
      id: generateId(),
      name,
      resultIds,
      currentIndex: 0,
      syncViews: true
    }

    comparisonSession.value = session
    return session
  }

  function addToComparison(resultId: string): void {
    if (!comparisonSession.value) return

    if (!comparisonSession.value.resultIds.includes(resultId)) {
      comparisonSession.value.resultIds.push(resultId)
    }
  }

  function removeFromComparison(resultId: string): void {
    if (!comparisonSession.value) return

    comparisonSession.value.resultIds = comparisonSession.value.resultIds.filter(id => id !== resultId)
  }

  function toggleSyncViews(): void {
    if (comparisonSession.value) {
      comparisonSession.value.syncViews = !comparisonSession.value.syncViews
    }
  }

  // ============ 全屏控制 ============

  async function enterFullscreen(): Promise<boolean> {
    try {
      await document.documentElement.requestFullscreen()
      isFullscreen.value = true
      return true
    } catch {
      return false
    }
  }

  function exitFullscreen(): void {
    if (document.fullscreenElement) {
      document.exitFullscreen()
    }
    isFullscreen.value = false
  }

  function toggleFullscreen(): void {
    if (isFullscreen.value) {
      exitFullscreen()
    } else {
      enterFullscreen()
    }
  }

  // ============ UI 控制 ============

  function toggleSidebar(): void {
    showSidebar.value = !showSidebar.value
  }

  function toggleToolbar(): void {
    showToolbar.value = !showToolbar.value
  }

  function toggleColorMap(): void {
    showColorMap.value = !showColorMap.value
  }

  // ============ 配置文件 ============

  function updateConfig(updates: Partial<MobileConfig>): void {
    Object.assign(config.value, updates)
    setStorage(CONFIG_KEY, config.value)
  }

  function resetConfig(): void {
    config.value = {
      enableGestures: true,
      enableHaptics: true,
      autoRotate: true,
      showControls: true,
      swipeSensitivity: 5,
      pinchZoomSpeed: 5
    }
    setStorage(CONFIG_KEY, config.value)
  }

  // ============ 持久化 ============

  function saveAnnotations(): void {
    setStorage(ANNOTATIONS_KEY, annotations.value)
  }

  function loadData(): void {
    const storedConfig = getStorage<MobileConfig>(CONFIG_KEY)
    if (storedConfig) config.value = storedConfig

    const storedViewports = getStorage<ResultViewport[]>(VIEWPORTS_KEY)
    if (storedViewports) viewports.value = storedViewports

    const storedAnnotations = getStorage<Annotation[]>(ANNOTATIONS_KEY)
    if (storedAnnotations) annotations.value = storedAnnotations
  }

  // ============ 初始化 ============

  onMounted(() => {
    loadData()
    detectDevice()

    window.addEventListener('resize', handleResize)
    window.addEventListener('orientationchange', handleResize)
    document.addEventListener('fullscreenchange', () => {
      isFullscreen.value = !!document.fullscreenElement
    })
  })

  onUnmounted(() => {
    window.removeEventListener('resize', handleResize)
    window.removeEventListener('orientationchange', handleResize)
  })

  return {
    // 设备信息
    deviceType,
    orientation,
    isMobile,
    isTablet,
    isLandscape,
    isPortrait,

    // 视图状态
    viewMode,
    currentViewport,
    viewportStyle,
    comparisonSession,

    // 配置
    config,

    // UI 状态
    showSidebar,
    showToolbar,
    showColorMap,
    isFullscreen,

    // 批注
    annotations,

    // 手势
    activeGestures,

    // 手势处理
    handleTouchStart,
    handleTouchMove,
    handleTouchEnd,
    handleWheel,
    handleMouseDown,
    handleMouseMove,
    handleMouseUp,

    // 视图控制
    cycleViewMode,
    nextResult,
    previousResult,
    resetViewport,
    fitToScreen,
    setViewMode,

    // 批注
    addAnnotation,
    replyToAnnotation,
    deleteAnnotation,
    getResultAnnotations,

    // 视口
    saveViewport,
    loadViewport,

    // 对比
    createComparisonSession,
    addToComparison,
    removeFromComparison,
    toggleSyncViews,

    // 全屏
    enterFullscreen,
    exitFullscreen,
    toggleFullscreen,

    // UI
    toggleSidebar,
    toggleToolbar,
    toggleColorMap,

    // 配置
    updateConfig,
    resetConfig
  }
}
