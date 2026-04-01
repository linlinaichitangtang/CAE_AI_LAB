import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Home',
    component: () => import('./views/HomeView.vue')
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
    path: '/help',
    name: 'help',
    component: () => import('./views/HelpView.vue')
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router