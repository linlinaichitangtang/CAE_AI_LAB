<template>
  <nav class="mobile-bottom-nav safe-area-bottom desktop-only-hide">
    <router-link
      v-for="item in navItems"
      :key="item.path"
      :to="item.path"
      class="nav-tab"
      :class="{ active: isActive(item.path) }"
    >
      <component :is="item.icon" class="tab-icon" />
      <span class="tab-label">{{ item.label }}</span>
    </router-link>
  </nav>
</template>

<script setup lang="ts">
import { h } from 'vue'
import { useRoute } from 'vue-router'

// 简单的 SVG 图标组件（内联，避免依赖外部图标库）
const HomeIcon = { render: () => h('svg', { viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '2' }, [h('path', { d: 'M3 9l9-7 9 7v11a2 2 0 01-2 2H5a2 2 0 01-2-2z' }), h('polyline', { points: '9 22 9 12 15 12 15 22' })]) }
const SimulationIcon = { render: () => h('svg', { viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '2' }, [h('path', { d: 'M12 2L2 7l10 5 10-5-10-5z' }), h('path', { d: 'M2 17l10 5 10-5' }), h('path', { d: 'M2 12l10 5 10-5' })]) }
const NotesIcon = { render: () => h('svg', { viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '2' }, [h('path', { d: 'M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z' }), h('polyline', { points: '14 2 14 8 20 8' }), h('line', { x1: '16', y1: '13', x2: '8', y2: '13' }), h('line', { x1: '16', y1: '17', x2: '8', y2: '17' })]) }
const ModelingIcon = { render: () => h('svg', { viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '2' }, [h('path', { d: 'M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4A2 2 0 003 8v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4A2 2 0 0021 16z' })]) }
const MoreIcon = { render: () => h('svg', { viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '2' }, [h('circle', { cx: '12', cy: '12', r: '1' }), h('circle', { cx: '12', cy: '5', r: '1' }), h('circle', { cx: '12', cy: '19', r: '1' })]) }

const navItems = [
  { path: '/', label: '首页', icon: HomeIcon },
  { path: '/simulation', label: '仿真', icon: SimulationIcon },
  { path: '/modeling', label: '建模', icon: ModelingIcon },
  { path: '/notes', label: '笔记', icon: NotesIcon },
  { path: '/more', label: '更多', icon: MoreIcon },
]

const route = useRoute()

function isActive(path: string) {
  if (path === '/') return route.path === '/'
  return route.path.startsWith(path)
}
</script>

<style scoped>
.mobile-bottom-nav {
  display: none;
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  height: 56px;
  background: var(--bg-primary, #ffffff);
  border-top: 1px solid var(--border-color, #e5e7eb);
  z-index: 1000;
  padding: 0 env(safe-area-inset-bottom, 0px);
}

@media (max-width: 767px) {
  .mobile-bottom-nav {
    display: flex;
    align-items: center;
    justify-content: space-around;
  }
}

.nav-tab {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2px;
  padding: 4px 0;
  min-width: 56px;
  min-height: 44px;
  color: var(--text-tertiary, #9ca3af);
  text-decoration: none;
  transition: color 0.15s ease;
  -webkit-tap-highlight-color: transparent;
}

.nav-tab.active {
  color: var(--primary, #2563EB);
}

.tab-icon {
  width: 22px;
  height: 22px;
}

.tab-label {
  font-size: 10px;
  font-weight: 500;
  line-height: 1;
}
</style>
