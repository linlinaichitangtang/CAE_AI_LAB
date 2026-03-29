<template>
  <nav class="h-full bg-[var(--bg-surface)] flex flex-col items-center py-4 border-r border-[var(--border-subtle)]">
    <!-- Main Navigation -->
    <div class="flex flex-col items-center gap-2 flex-1">
      <router-link
        v-for="item in navItems"
        :key="item.path"
        :to="item.path"
        class="nav-item"
        :class="{ 'active': $route.path === item.path }"
      >
        <span class="nav-icon">{{ item.icon }}</span>
        <span class="nav-label">{{ item.label }}</span>
      </router-link>
    </div>
    
    <!-- Bottom Section -->
    <div class="mt-auto pt-4 border-t border-[var(--border-subtle)]">
      <router-link
        to="/settings"
        class="nav-item"
        :class="{ 'active': $route.path === '/settings' }"
      >
        <span class="nav-icon">⚙️</span>
        <span class="nav-label">设置</span>
      </router-link>
    </div>
  </nav>
</template>

<script setup lang="ts">
const navItems = [
  { path: '/', icon: '🏠', label: '首页' },
  { path: '/notes', icon: '📝', label: '笔记' },
  { path: '/modeling', icon: '🎨', label: '建模' },
  { path: '/code', icon: '💻', label: '代码' },
  { path: '/simulation', icon: '🔬', label: '仿真' }
]
</script>