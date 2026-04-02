<template>
  <div class="mobile-report-viewer" ref="viewerRef">
    <!-- Fullscreen toggle -->
    <div class="viewer-header" v-if="!isFullscreen">
      <button class="fullscreen-btn" @click="enterFullscreen" title="全屏查看">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4"/>
        </svg>
      </button>
      <span class="header-title">仿真报告</span>
      <div class="header-actions">
        <button class="font-btn" @click="cycleFontSize" title="调整字体">
          <span class="font-size-label">{{ fontSize }}px</span>
        </button>
      </div>
    </div>

    <!-- Fullscreen exit button -->
    <div class="fullscreen-exit" v-if="isFullscreen" @click="exitFullscreen">
      <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
      </svg>
    </div>

    <!-- Report content with swipe navigation -->
    <div
      class="report-pages-container"
      ref="pagesContainer"
      @touchstart="onTouchStart"
      @touchmove="onTouchMove"
      @touchend="onTouchEnd"
    >
      <div
        class="report-pages-track"
        :style="{ transform: `translateX(-${currentPage * 100}%)`, transition: isSwiping ? 'none' : 'transform 0.3s ease' }"
      >
        <div
          v-for="(page, idx) in pages"
          :key="idx"
          class="report-page"
          :style="{ fontSize: fontSize + 'px' }"
        >
          <div class="page-content" v-html="page"></div>
        </div>
      </div>
    </div>

    <!-- Page indicator -->
    <div class="page-indicator" v-if="pages.length > 1">
      <span
        v-for="(_, idx) in pages"
        :key="idx"
        :class="['dot', { active: idx === currentPage }]"
        @click="goToPage(idx)"
      ></span>
    </div>

    <!-- Page navigation -->
    <div class="page-nav" v-if="pages.length > 1">
      <button class="nav-btn" :disabled="currentPage === 0" @click="prevPage">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
        </svg>
      </button>
      <span class="page-info">{{ currentPage + 1 }} / {{ pages.length }}</span>
      <button class="nav-btn" :disabled="currentPage === pages.length - 1" @click="nextPage">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
        </svg>
      </button>
    </div>

    <!-- Bottom PDF export bar -->
    <div class="bottom-bar">
      <button class="export-btn" @click="exportPdf" :disabled="isExporting">
        <svg v-if="isExporting" class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
        </svg>
        <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
        </svg>
        <span>{{ isExporting ? '导出中...' : '导出 PDF' }}</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'

const props = withDefaults(defineProps<{
  reportHtml?: string
}>(), {
  reportHtml: '',
})

const viewerRef = ref<HTMLElement>()
const pagesContainer = ref<HTMLElement>()

const isFullscreen = ref(false)
const currentPage = ref(0)
const fontSize = ref(14)
const isExporting = ref(false)

// Swipe state
const isSwiping = ref(false)
let touchStartX = 0
let touchStartY = 0
let touchDeltaX = 0

// Font size options
const fontSizes = [12, 14, 16, 18, 20]

// Split HTML content into pages
const pages = computed(() => {
  if (!props.reportHtml) return ['<p style="text-align:center;color:#999;padding:40px 0;">暂无报告内容</p>']

  // Split by major sections (h2 tags)
  const sections = props.reportHtml.split(/(?=<h2[^>]*>)/i)
  if (sections.length <= 1) return [props.reportHtml]

  // Group sections into pages (2-3 sections per page)
  const groupedPages: string[] = []
  let currentPageContent = ''

  for (const section of sections) {
    if (!section.trim()) continue
    currentPageContent += section
    // Count h2 tags to decide page breaks
    const h2Count = (currentPageContent.match(/<h2[^>]*>/gi) || []).length
    if (h2Count >= 2) {
      groupedPages.push(currentPageContent)
      currentPageContent = ''
    }
  }
  if (currentPageContent.trim()) {
    groupedPages.push(currentPageContent)
  }

  return groupedPages.length > 0 ? groupedPages : [props.reportHtml]
})

// Navigation
function prevPage() {
  if (currentPage.value > 0) currentPage.value--
}

function nextPage() {
  if (currentPage.value < pages.value.length - 1) currentPage.value++
}

function goToPage(idx: number) {
  currentPage.value = idx
}

// Font size
function cycleFontSize() {
  const currentIdx = fontSizes.indexOf(fontSize.value)
  const nextIdx = (currentIdx + 1) % fontSizes.length
  fontSize.value = fontSizes[nextIdx]
}

// Fullscreen
function enterFullscreen() {
  if (!viewerRef.value) return
  if (viewerRef.value.requestFullscreen) {
    viewerRef.value.requestFullscreen()
  } else if ((viewerRef.value as any).webkitRequestFullscreen) {
    (viewerRef.value as any).webkitRequestFullscreen()
  }
  isFullscreen.value = true
}

function exitFullscreen() {
  if (document.exitFullscreen) {
    document.exitFullscreen()
  } else if ((document as any).webkitExitFullscreen) {
    (document as any).webkitExitFullscreen()
  }
  isFullscreen.value = false
}

function onFullscreenChange() {
  isFullscreen.value = !!document.fullscreenElement
}

// Touch swipe
function onTouchStart(e: TouchEvent) {
  touchStartX = e.touches[0].clientX
  touchStartY = e.touches[0].clientY
  isSwiping.value = true
  touchDeltaX = 0
}

function onTouchMove(e: TouchEvent) {
  if (!isSwiping.value) return
  const dx = e.touches[0].clientX - touchStartX
  const dy = e.touches[0].clientY - touchStartY

  // Only handle horizontal swipes
  if (Math.abs(dx) > Math.abs(dy)) {
    touchDeltaX = dx
    e.preventDefault()
  }
}

function onTouchEnd() {
  if (!isSwiping.value) return
  isSwiping.value = false

  const threshold = 50
  if (touchDeltaX < -threshold && currentPage.value < pages.value.length - 1) {
    currentPage.value++
  } else if (touchDeltaX > threshold && currentPage.value > 0) {
    currentPage.value--
  }
  touchDeltaX = 0
}

// PDF Export
async function exportPdf() {
  if (isExporting.value || !props.reportHtml) return
  isExporting.value = true

  try {
    const { generatePdfReport, downloadBlob } = await import('@/utils/pdfReport')
    const blob = await generatePdfReport(props.reportHtml, {
      title: `CAE仿真报告_${new Date().toLocaleDateString().replace(/\//g, '-')}`,
      orientation: 'portrait',
      format: 'a4',
    })
    downloadBlob(blob, `CAE仿真报告_${new Date().toLocaleDateString().replace(/\//g, '-')}.pdf`)
  } catch (e: any) {
    console.error('导出 PDF 失败:', e)
    alert('导出 PDF 失败: ' + (e.message || e))
  } finally {
    isExporting.value = false
  }
}

onMounted(() => {
  document.addEventListener('fullscreenchange', onFullscreenChange)
  document.addEventListener('webkitfullscreenchange', onFullscreenChange)
})

onUnmounted(() => {
  document.removeEventListener('fullscreenchange', onFullscreenChange)
  document.removeEventListener('webkitfullscreenchange', onFullscreenChange)
  if (isFullscreen.value) exitFullscreen()
})

defineExpose({ enterFullscreen, exitFullscreen })
</script>

<style scoped>
.mobile-report-viewer {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #fff;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  position: relative;
}

/* Header */
.viewer-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 12px;
  background: #fff;
  border-bottom: 1px solid #e5e5e5;
}
.fullscreen-btn, .font-btn {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid #e5e5e5;
  border-radius: 8px;
  background: #fff;
  color: #666;
  -webkit-tap-highlight-color: transparent;
}
.fullscreen-btn:active, .font-btn:active {
  background: #f5f5f5;
}
.header-title {
  flex: 1;
  font-size: 15px;
  font-weight: 600;
  color: #1a1a1a;
}
.font-size-label {
  font-size: 11px;
  font-weight: 600;
}

/* Fullscreen exit */
.fullscreen-exit {
  position: fixed;
  top: 12px;
  right: 12px;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0,0,0,0.5);
  color: #fff;
  border-radius: 50%;
  z-index: 100;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}

/* Report pages */
.report-pages-container {
  flex: 1;
  overflow: hidden;
  position: relative;
}
.report-pages-track {
  display: flex;
  height: 100%;
  width: 100%;
}
.report-page {
  min-width: 100%;
  height: 100%;
  overflow-y: auto;
  padding: 16px;
  -webkit-overflow-scrolling: touch;
  box-sizing: border-box;
}
.page-content {
  color: #333;
  line-height: 1.6;
}
.page-content :deep(h2) {
  font-size: 1.3em;
  font-weight: 700;
  color: #1a1a1a;
  margin: 20px 0 10px;
  padding-bottom: 6px;
  border-bottom: 2px solid #3b82f6;
}
.page-content :deep(h3) {
  font-size: 1.1em;
  font-weight: 600;
  color: #333;
  margin: 16px 0 8px;
}
.page-content :deep(p) {
  margin: 8px 0;
}
.page-content :deep(table) {
  width: 100%;
  border-collapse: collapse;
  margin: 12px 0;
  font-size: 0.9em;
}
.page-content :deep(th),
.page-content :deep(td) {
  border: 1px solid #e5e5e5;
  padding: 6px 8px;
  text-align: left;
}
.page-content :deep(th) {
  background: #f5f5f5;
  font-weight: 600;
}
.page-content :deep(ul),
.page-content :deep(ol) {
  padding-left: 20px;
  margin: 8px 0;
}

/* Page indicator */
.page-indicator {
  display: flex;
  justify-content: center;
  gap: 6px;
  padding: 8px 0;
}
.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #ddd;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  transition: all 0.2s;
}
.dot.active {
  background: #3b82f6;
  width: 20px;
  border-radius: 4px;
}

/* Page navigation */
.page-nav {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 4px 0;
}
.nav-btn {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid #e5e5e5;
  border-radius: 50%;
  background: #fff;
  color: #666;
  -webkit-tap-highlight-color: transparent;
}
.nav-btn:disabled {
  opacity: 0.3;
}
.nav-btn:active:not(:disabled) {
  background: #f5f5f5;
}
.page-info {
  font-size: 13px;
  color: #999;
  min-width: 60px;
  text-align: center;
}

/* Bottom bar */
.bottom-bar {
  padding: 8px 16px;
  padding-bottom: max(8px, env(safe-area-inset-bottom));
  background: #fff;
  border-top: 1px solid #e5e5e5;
}
.export-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
  padding: 12px;
  border: none;
  border-radius: 12px;
  background: #3b82f6;
  color: #fff;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}
.export-btn:active:not(:disabled) {
  background: #2563eb;
}
.export-btn:disabled {
  opacity: 0.6;
}

/* Fullscreen mode adjustments */
:fullscreen .viewer-header,
:-webkit-full-screen .viewer-header {
  display: none;
}
</style>
