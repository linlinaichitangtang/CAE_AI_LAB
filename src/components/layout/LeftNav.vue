<template>
  <nav class="w-16 bg-white border-r border-gray-200 flex flex-col items-center py-4 gap-4">
    <router-link
      v-for="item in navItems"
      :key="item.path"
      :to="item.path"
      class="w-12 h-12 flex flex-col items-center justify-center rounded-lg hover:bg-gray-100 transition-colors"
      :class="{ 'bg-primary-50 text-primary-600': $route.path === item.path }"
    >
      <span class="text-xl">{{ item.icon }}</span>
      <span class="text-[10px] mt-0.5">{{ item.label }}</span>
    </router-link>
  </nav>
</template>

<script setup lang="ts">
const navItems = [
  { path: '/', icon: '🏠', label: '首页' },
  { path: '/notes', icon: '📝', label: '笔记' },
  { path: '/modeling', icon: '🎨', label: '建模' },
  { path: '/code', icon: '💻', label: '代码' },
  { path: '/simulation', icon: '🔬', label: '仿真' },
  { path: '/settings', icon: '⚙️', label: '设置' }
]
</script>