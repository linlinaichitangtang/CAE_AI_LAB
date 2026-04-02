const CACHE_NAME = 'caelab-v1'
const STATIC_ASSETS = [
  '/',
  '/index.html',
]

// 安装：缓存核心资源
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => {
      return cache.addAll(STATIC_ASSETS)
    })
  )
  self.skipWaiting()
})

// 激活：清理旧缓存
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((keys) => {
      return Promise.all(
        keys.filter((key) => key !== CACHE_NAME).map((key) => caches.delete(key))
      )
    })
  )
  self.clients.claim()
})

// 请求拦截：网络优先，离线时回退缓存
self.addEventListener('fetch', (event) => {
  // 仅处理 GET 请求
  if (event.request.method !== 'GET') return

  // API 请求不缓存
  if (event.request.url.includes('/api/')) return

  event.respondWith(
    fetch(event.request)
      .then((response) => {
        // 成功响应：缓存副本
        const clone = response.clone()
        caches.open(CACHE_NAME).then((cache) => {
          cache.put(event.request, clone)
        })
        return response
      })
      .catch(() => {
        // 网络失败：回退缓存
        return caches.match(event.request).then((cached) => {
          return cached || new Response('离线模式', { status: 503 })
        })
      })
  )
})
