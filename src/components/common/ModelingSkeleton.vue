<template>
  <div class="h-full flex flex-col bg-[var(--bg-base)] animate-pulse">
    <!-- Header skeleton -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <div class="h-5 w-20 bg-gray-200 dark:bg-gray-700 rounded mb-1"></div>
        <div class="h-3 w-40 bg-gray-200 dark:bg-gray-700 rounded"></div>
      </div>
      <div class="flex items-center gap-2">
        <div class="h-8 w-16 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div class="h-8 w-16 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div class="h-8 w-24 bg-gray-200 dark:bg-gray-700 rounded"></div>
      </div>
    </div>

    <!-- Main content skeleton -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left: Scene tree skeleton -->
      <div class="w-56 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col flex-shrink-0">
        <div class="px-3 py-2 border-b border-[var(--border-subtle)]">
          <div class="h-3 w-12 bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
        <div class="flex-1 p-3 space-y-2">
          <div class="h-10 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-10 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-10 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-10 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
        <div class="p-3 border-t border-[var(--border-subtle)]">
          <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
      </div>

      <!-- Center: 3D Viewport skeleton -->
      <div class="flex-1 flex flex-col">
        <div class="flex-1 bg-gray-100 dark:bg-gray-900 flex items-center justify-center">
          <ThreeJSWireframeSkeleton :size="200" />
        </div>
        <!-- Toolbar skeleton -->
        <div class="h-10 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] flex items-center px-4 gap-4">
          <div class="h-6 w-16 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-6 w-16 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-6 w-16 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="flex-1"></div>
          <div class="h-6 w-20 bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
      </div>

      <!-- Right: Properties panel skeleton -->
      <div class="w-64 bg-[var(--bg-surface)] border-l border-[var(--border-subtle)] flex-shrink-0 p-4">
        <div class="h-4 w-16 bg-gray-200 dark:bg-gray-700 rounded mb-3"></div>
        <div class="space-y-3">
          <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
        <div class="mt-4 pt-4 border-t border-[var(--border-subtle)]">
          <div class="h-4 w-20 bg-gray-200 dark:bg-gray-700 rounded mb-3"></div>
          <div class="space-y-3">
            <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
            <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
            <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import ThreeJSWireframeSkeleton from './ThreeJSWireframeSkeleton.vue'
</script>
