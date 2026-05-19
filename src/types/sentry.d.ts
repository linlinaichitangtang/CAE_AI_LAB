// Type declarations for optional Sentry integration
// @sentry/vue is loaded dynamically and may not be installed
declare module '@sentry/vue' {
  export function init(options: any): void
  export function captureException(error: any): void
  export function captureMessage(message: string): void
  export const vueRouterInstrumentation: any
  export const browserTracingIntegration: any
}
