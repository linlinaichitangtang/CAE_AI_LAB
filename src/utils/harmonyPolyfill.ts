/**
 * 纯血鸿蒙 (OpenHarmony) 兼容性 Polyfill
 * 支持 HarmonyOS NEXT / OpenHarmony 全版本
 * 处理 WebView 行为差异
 */

export function applyHarmonyPolyfills() {
  const ua = navigator.userAgent.toLowerCase()
  const isPureHarmony = ua.includes('openharmony') || ua.includes('ohmos') ||
                        (ua.includes('harmony') && !ua.includes('android'))
  const isHarmony = ua.includes('harmony') || ua.includes('hmos')

  if (!isPureHarmony && !isHarmony) return

  console.info('[CAELab] Applying HarmonyOS compatibility polyfills...')

  // 1. 修复 CSS env() 安全区域变量
  // 纯血鸿蒙 WebView 支持 env(safe-area-inset-*) 但需要 viewport-fit=cover
  const viewport = document.querySelector('meta[name="viewport"]')
  if (viewport) {
    const content = viewport.getAttribute('content') || ''
    if (!content.includes('viewport-fit=cover')) {
      viewport.setAttribute('content', content + ', viewport-fit=cover')
    }
  }

  // 2. 修复 PointerEvent 压感支持
  // 鸿蒙 M-Pencil 通过 PointerEvent 传递压感
  const originalAddEventListener = EventTarget.prototype.addEventListener
  EventTarget.prototype.addEventListener = function(
    this: EventTarget,
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | AddEventListenerOptions
  ) {
    if (type === 'pointermove' && typeof listener === 'function') {
      const wrappedListener = function(this: EventTarget, e: Event) {
        const pe = e as PointerEvent
        // 鸿蒙 M-Pencil 压感兼容
        if (pe.pointerType === 'pen' && pe.pressure === 0) {
          const force = (pe as any).webkitForce || (pe as any).force || 0.5
          Object.defineProperty(pe, 'pressure', {
            value: Math.min(force / 3, 1),
            writable: false
          })
        }
        return listener.call(this, pe)
      }
      return originalAddEventListener.call(this, type, wrappedListener, options)
    }
    return originalAddEventListener.call(this, type, listener, options)
  }

  // 3. 修复 Web Worker 兼容性
  // 纯血鸿蒙对 Worker 的 Blob URL 支持有限
  if (typeof Worker !== 'undefined') {
    const OriginalWorker = Worker
    ;(window as any).Worker = class extends OriginalWorker {
      constructor(scriptURL: string | URL, options?: WorkerOptions) {
        if (typeof scriptURL === 'string' && scriptURL.startsWith('blob:')) {
          console.warn('[HarmonyOS] Blob URL Worker may not be supported')
        }
        super(scriptURL, options)
      }
    }
  }

  // 4. 修复 CSS backdrop-filter
  // 纯血鸿蒙 WebView 对 backdrop-filter 支持不完整
  const style = document.createElement('style')
  style.id = 'harmony-compat-fixes'
  style.textContent = `
    @supports not (backdrop-filter: blur(1px)) {
      .backdrop-blur, .backdrop-blur-sm, .backdrop-blur-md, .backdrop-blur-lg {
        backdrop-filter: none !important;
        -webkit-backdrop-filter: none !important;
        background-color: rgba(255, 255, 255, 0.95) !important;
      }
    }
    /* 纯血鸿蒙安全区域 */
    .harmony-safe-area-top { padding-top: env(safe-area-inset-top); }
    .harmony-safe-area-bottom { padding-bottom: env(safe-area-inset-bottom); }
    .harmony-safe-area-left { padding-left: env(safe-area-inset-left); }
    .harmony-safe-area-right { padding-right: env(safe-area-inset-right); }
    /* 折叠屏适配 */
    @media (screen-spanning: fold) {
      .fold-aware { padding: env(foldable-area-inset-top) env(foldable-area-inset-left) env(foldable-area-inset-bottom) env(foldable-area-inset-right); }
    }
    /* 修复 input focus 样式 */
    input:focus, textarea:focus, select:focus {
      outline: 2px solid #007DFF;
      outline-offset: 2px;
    }
    /* 修复滚动弹性 */
    body {
      overscroll-behavior: contain;
      -webkit-overflow-scrolling: touch;
    }
  `
  document.head.appendChild(style)

  // 5. 修复滚动行为
  // 纯血鸿蒙 WebView 的 scrollIntoView 可能需要额外处理
  if (typeof Element !== 'undefined') {
    const originalScrollIntoView = Element.prototype.scrollIntoView
    Element.prototype.scrollIntoView = function(alignToTop?: boolean) {
      try {
        originalScrollIntoView.call(this, alignToTop)
      } catch (e) {
        console.warn('[HarmonyOS] scrollIntoView failed:', e)
      }
    }
  }

  // 6. 修复 localStorage 配额
  // 纯血鸿蒙可能限制 localStorage 大小
  try {
    localStorage.setItem('__harmony_check', '1')
    localStorage.removeItem('__harmony_check')
  } catch (e) {
    console.warn('[HarmonyOS] localStorage may have limited quota')
  }

  // 7. 注册 harmonyready 事件
  window.dispatchEvent(new CustomEvent('harmonyready', {
    detail: { version: navigator.userAgent.match(/openharmony[\s\/]?([\d.]+)/i)?.[1] || 'unknown' }
  }))

  console.info('[CAELab] HarmonyOS compatibility polyfills applied')
}
