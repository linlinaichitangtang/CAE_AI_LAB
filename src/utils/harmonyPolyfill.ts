/**
 * 鸿蒙 NEXT WebView 兼容性 Polyfill
 * 鸿蒙 WebView 基于 Chromium，但部分 API 行为有差异
 */

export function applyHarmonyPolyfills() {
  const ua = navigator.userAgent.toLowerCase()
  const isHarmony = ua.includes('harmony') || ua.includes('hmos')
  if (!isHarmony) return

  // 1. 修复 CSS env() 安全区域变量
  // 鸿蒙 WebView 支持 env(safe-area-inset-*) 但需要 viewport-fit=cover
  const viewport = document.querySelector('meta[name="viewport"]')
  if (viewport) {
    const content = viewport.getAttribute('content') || ''
    if (!content.includes('viewport-fit=cover')) {
      viewport.setAttribute('content', content + ', viewport-fit=cover')
    }
  }

  // 2. 修复 PointerEvent 压感支持
  // 鸿蒙 M-Pencil 通过 PointerEvent 传递压感，但 event.pressure 可能始终为 0
  // 需要通过 event.webkitForce 或自定义事件获取
  const originalAddEventListener = EventTarget.prototype.addEventListener
  EventTarget.prototype.addEventListener = function(this: EventTarget, type: string, listener: EventListenerOrEventListenerObject, options?: boolean | AddEventListenerOptions) {
    if (type === 'pointermove' && typeof listener === 'function') {
      const wrappedListener = function(this: EventTarget, e: Event) {
        const pe = e as PointerEvent
        // 鸿蒙 M-Pencil 压感兼容
        if (pe.pointerType === 'pen' && pe.pressure === 0) {
          // 尝试从 webkitForce 获取
          const force = (pe as any).webkitForce || (pe as any).force || 0.5
          Object.defineProperty(pe, 'pressure', { value: Math.min(force / 3, 1), writable: false })
        }
        return listener.call(this, pe)
      }
      return originalAddEventListener.call(this, type, wrappedListener, options)
    }
    return originalAddEventListener.call(this, type, listener, options)
  }

  // 3. 修复 Web Worker 兼容性
  // 鸿蒙 WebView 对 Worker 的 Blob URL 支持有限
  if (typeof Worker !== 'undefined') {
    const OriginalWorker = Worker
    ;(window as any).Worker = class extends OriginalWorker {
      constructor(scriptURL: string | URL, options?: WorkerOptions) {
        if (typeof scriptURL === 'string' && scriptURL.startsWith('blob:')) {
          // 鸿蒙可能不支持 blob: URL 的 Worker
          console.warn('[HarmonyOS] Blob URL Worker may not be supported')
        }
        super(scriptURL, options)
      }
    }
  }

  // 4. 修复 CSS backdrop-filter
  // 鸿蒙 WebView 对 backdrop-filter 支持不完整
  const style = document.createElement('style')
  style.textContent = `
    @supports not (backdrop-filter: blur(1px)) {
      .backdrop-blur, .backdrop-blur-sm, .backdrop-blur-md, .backdrop-blur-lg {
        backdrop-filter: none !important;
        -webkit-backdrop-filter: none !important;
        background-color: rgba(255, 255, 255, 0.95) !important;
      }
    }
  `
  document.head.appendChild(style)

  console.info('[CAELab] HarmonyOS NEXT compatibility polyfills applied')
}
