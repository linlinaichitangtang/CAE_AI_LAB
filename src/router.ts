import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

// 预加载核心路由的辅助函数
/* @vite-ignore */
const preload = (importFn: () => Promise<any>) => importFn()

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Home',
    component: () => import('./views/HomeView.vue')
  },
  {
    path: '/onboarding',
    name: 'Onboarding',
    component: () => import('./views/OnboardingView.vue'),
    meta: { public: true }
  },
  {
    path: '/login',
    name: 'Login',
    component: () => import('./views/AuthView.vue'),
    meta: { public: true }
  },
  {
    path: '/membership',
    name: 'Membership',
    component: () => import('./views/MembershipView.vue')
  },
  {
    path: '/profile',
    name: 'Profile',
    component: () => import('./views/ProfileView.vue')
  },
  {
    path: '/share',
    name: 'Share',
    component: () => import('./views/ShareView.vue')
  },
  {
    path: '/notes',
    name: 'Notes',
    component: () => import('./views/NotesView.vue')
  },
  {
    path: '/modeling',
    name: 'Modeling',
    component: () => import('./views/ModelingView.vue')
  },
  {
    path: '/code',
    name: 'Code',
    component: () => import('./views/CodeView.vue')
  },
  {
    path: '/simulation',
    name: 'Simulation',
    component: () => import('./views/SimulationView.vue')
  },
  {
    path: '/fatigue',
    name: 'Fatigue',
    component: () => import('./views/FatigueView.vue')
  },
  {
    path: '/transient',
    name: 'TransientDynamics',
    component: () => import('./views/TransientDynamicsView.vue')
  },
  {
    path: '/cfd',
    name: 'CFD',
    component: () => import('./views/CFDView.vue')
  },
  {
    path: '/thermal',
    name: 'ThermalCoupling',
    component: () => import('./views/ThermalCouplingView.vue')
  },
  {
    path: '/topology',
    name: 'TopologyOptimization',
    component: () => import('./views/TopologyOptimizationView.vue')
  },
  {
    path: '/explicit',
    name: 'ExplicitDynamics',
    component: () => import('./views/ExplicitDynamicsView.vue')
  },
  {
    path: '/electronics',
    name: 'ElectronicsPackage',
    component: () => import('./views/ElectronicsPackageView.vue')
  },
  {
    path: '/biomechanics',
    name: 'BioMechanics',
    component: () => import('./views/BioMechanicsView.vue')
  },
  {
    path: '/comparison',
    name: 'Comparison',
    component: () => import('./views/ComparisonView.vue')
  },
  {
    path: '/ai',
    name: 'AIChat',
    component: () => import('./views/AIChatView.vue')
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('./views/SettingsView.vue')
  },
  {
    path: '/plugins',
    name: 'Plugins',
    component: () => import('./views/PluginManager.vue')
  },
  {
    path: '/developer',
    name: 'DeveloperConsole',
    component: () => import('./views/DeveloperConsole.vue')
  },
  {
    path: '/help',
    name: 'help',
    component: () => import('./views/HelpView.vue')
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes,
  // 滚动行为优化
  scrollBehavior(to, _from, savedPosition) {
    if (savedPosition) {
      return savedPosition
    }
    return { top: 0 }
  }
})

// Auth 路由守卫：未登录用户重定向到 /login
router.beforeEach((to, _from, next) => {
  // 检查是否为公开路由（不需要登录）
  const isPublicRoute = to.meta.public === true

  // 检查登录状态
  const accessToken = localStorage.getItem('caelab-access-token')
  const isAuthenticated = !!accessToken

  // Onboarding redirect: if authenticated user hasn't completed onboarding, redirect to /onboarding
  if (to.path !== '/onboarding' && isAuthenticated && !localStorage.getItem('caelab-onboarding-done')) {
    next('/onboarding')
    return
  }
  // If user is trying to access onboarding but already completed it, redirect to home
  if (to.path === '/onboarding' && localStorage.getItem('caelab-onboarding-done')) {
    next('/')
    return
  }

  if (!isPublicRoute && !isAuthenticated) {
    // 未登录用户访问受保护路由，重定向到登录页
    next('/login')
  } else if (isPublicRoute && isAuthenticated && to.path === '/login') {
    // 已登录用户访问登录页，重定向到首页
    next('/')
  } else {
    next()
  }
})

// 移动端路由守卫：限制访问桌面端专属功能
router.beforeEach((to, _from, next) => {
  const isMobile = typeof window !== 'undefined' && window.innerWidth < 768

  // 移动端不可访问的路由列表
  const mobileRestricted = ['/cfd', '/explicit', '/topology', '/comparison', '/ai']

  if (isMobile && mobileRestricted.includes(to.path)) {
    // 移动端访问受限路由时重定向到首页
    next('/')
  } else {
    next()
  }
})

/**
 * 预加载核心路由组件
 * 在空闲时预加载首页、仿真、笔记、代码等核心路由
 * 移动端使用 requestIdleCallback，桌面端使用 setTimeout
 */
export function prefetchCoreRoutes() {
  const coreRoutes = [
    () => import('./views/SimulationView.vue'),
    () => import('./views/NotesView.vue'),
    () => import('./views/CodeView.vue'),
    () => import('./views/ModelingView.vue'),
  ]

  const schedulePrefetch = typeof requestIdleCallback !== 'undefined'
    ? requestIdleCallback
    : (cb: () => void) => setTimeout(cb, 200)

  schedulePrefetch(() => {
    coreRoutes.forEach((importFn, idx) => {
      setTimeout(() => {
        preload(importFn).catch(() => {
          // 预加载失败不影响正常使用
        })
      }, idx * 300) // 错开 300ms 避免同时请求
    })
  })
}

// 在路由就绪后预加载核心路由
router.isReady().then(() => {
  // 延迟 1 秒后开始预加载，确保首屏渲染完成
  setTimeout(prefetchCoreRoutes, 1000)
})

export default router
