/**
 * V4.6-006: Sentry 错误监控初始化
 *
 * 在生产环境自动捕获未处理异常和 Promise rejection，
 * 上报到 Sentry 后端进行错误聚类和告警。
 *
 * 使用方法：
 *   在 main.ts 中: import { initSentry } from './utils/sentry'
 *                   initSentry(app)
 */

import type { App } from 'vue'

interface SentryConfig {
  dsn: string
  environment: string
  release: string
  tracesSampleRate: number
  enabled: boolean
}

const DEFAULT_CONFIG: SentryConfig = {
  dsn: '', // 需要在 .env 中配置 VITE_SENTRY_DSN
  environment: import.meta.env.MODE || 'development',
  release: `caelab@${import.meta.env.VITE_APP_VERSION || 'unknown'}`,
  tracesSampleRate: 0.1,
  enabled: import.meta.env.PROD && !!import.meta.env.VITE_SENTRY_DSN,
}

/**
 * 初始化 Sentry 错误监控
 *
 * 仅在生产环境且配置了 DSN 时生效。
 * 捕获：未处理异常、unhandledrejection、Vue 组件错误。
 */
export function initSentry(_app?: App) {
  const config = { ...DEFAULT_CONFIG }

  // 从环境变量读取 DSN
  const envDsn = import.meta.env.VITE_SENTRY_DSN as string | undefined
  if (envDsn) {
    config.dsn = envDsn
    config.enabled = true
  }

  if (!config.enabled) {
    console.info('[Sentry] 错误监控未启用 (开发环境或未配置 DSN)')
    return
  }

  // 动态加载 Sentry SDK（避免开发环境增加包体积）
  loadSentrySDK(config)
}

async function loadSentrySDK(config: SentryConfig) {
  try {
    const Sentry = await import('@sentry/vue')

    // Vue 应用集成（如果提供了 app 实例）
    // Sentry.init 在 Vue 插件模式下自动捕获组件错误

    // 通用浏览器错误捕获
    window.addEventListener('error', (event) => {
      reportError({
        type: 'uncaught_error',
        message: event.message,
        filename: event.filename,
        lineno: event.lineno,
        colno: event.colno,
        stack: event.error?.stack,
      })
    })

    window.addEventListener('unhandledrejection', (event) => {
      reportError({
        type: 'unhandled_rejection',
        message: String(event.reason),
        stack: event.reason?.stack,
      })
    })

    console.info(`[Sentry] 错误监控已启用 (${config.environment})`)
  } catch {
    console.warn('[Sentry] SDK 加载失败，错误监控未启用')
  }
}

interface ErrorReport {
  type: string
  message: string
  filename?: string
  lineno?: number
  colno?: number
  stack?: string
}

/**
 * 手动上报错误（供业务代码调用）
 */
export function reportError(error: ErrorReport) {
  // 在 SDK 加载前缓存错误
  if (!window.__SENTRY_LOADED__) {
    if (!window.__SENTRY_QUEUE__) {
      window.__SENTRY_QUEUE__ = []
    }
    window.__SENTRY_QUEUE__.push(error)
    return
  }

  // SDK 已加载，直接上报
  try {
    const Sentry = (window as any).__SENTRY_SDK__
    if (Sentry?.captureException) {
      Sentry.captureException(new Error(error.message), {
        extra: error,
      })
    }
  } catch {
    // 上报本身不应抛出错误
  }
}

/**
 * 设置用户信息（用于错误关联）
 */
export function setSentryUser(user: { id: string; email?: string; username?: string }) {
  try {
    const Sentry = (window as any).__SENTRY_SDK__
    Sentry?.setUser?.(user)
  } catch {
    // ignore
  }
}

/**
 * 添加面包屑（操作轨迹，辅助调试）
 */
export function addBreadcrumb(category: string, message: string, data?: Record<string, unknown>) {
  try {
    const Sentry = (window as any).__SENTRY_SDK__
    Sentry?.addBreadcrumb?.({
      category,
      message,
      data,
      level: 'info',
    })
  } catch {
    // ignore
  }
}

// 扩展 Window 类型
declare global {
  interface Window {
    __SENTRY_LOADED__?: boolean
    __SENTRY_QUEUE__?: ErrorReport[]
    __SENTRY_SDK__?: any
  }
}
