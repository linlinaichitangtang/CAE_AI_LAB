import { applyHarmonyPolyfills } from './utils/harmonyPolyfill'
applyHarmonyPolyfills()

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import router from './router'
import i18n from './locales'
import App from './App.vue'
import './styles/main.css'

const app = createApp(App)

// Pinia store
const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)
app.use(pinia)

// Router
app.use(router)

// i18n
app.use(i18n)

app.mount('#app')

// 注册 Service Worker（仅生产环境）
if ('serviceWorker' in navigator && import.meta.env.PROD) {
  navigator.serviceWorker.register('/sw.js').catch(() => {
    // Service Worker 注册失败不影响应用运行
  })
}
