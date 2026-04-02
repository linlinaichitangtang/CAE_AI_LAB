<template>
  <div class="h-full flex flex-col bg-gray-50 dark:bg-gray-900 animate-pulse">
    <!-- Header skeleton -->
    <div class="flex items-center justify-between px-4 py-3 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
      <div>
        <div class="h-5 w-24 bg-gray-200 dark:bg-gray-700 rounded mb-1"></div>
        <div class="h-3 w-48 bg-gray-200 dark:bg-gray-700 rounded"></div>
      </div>
      <div class="flex items-center gap-2">
        <div class="h-8 w-20 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div class="h-8 w-20 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div class="h-8 w-24 bg-gray-200 dark:bg-gray-700 rounded"></div>
      </div>
    </div>

    <!-- Main content skeleton -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left panel skeleton -->
      <div class="w-72 border-r border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 flex-shrink-0 p-4">
        <!-- Analysis type selector -->
        <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded mb-4"></div>
        <!-- Mesh config -->
        <div class="mb-4">
          <div class="h-4 w-20 bg-gray-200 dark:bg-gray-700 rounded mb-2"></div>
          <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded mb-2"></div>
          <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded mb-2"></div>
          <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
        <!-- Material config -->
        <div class="mb-4">
          <div class="h-4 w-20 bg-gray-200 dark:bg-gray-700 rounded mb-2"></div>
          <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded mb-2"></div>
          <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded mb-2"></div>
          <div class="h-8 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
        <!-- Boundary conditions -->
        <div>
          <div class="h-4 w-24 bg-gray-200 dark:bg-gray-700 rounded mb-2"></div>
          <div class="h-10 w-full bg-gray-200 dark:bg-gray-700 rounded mb-2"></div>
          <div class="h-10 w-full bg-gray-200 dark:bg-gray-700 rounded mb-2"></div>
          <div class="h-10 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
      </div>

      <!-- Center 3D viewport skeleton -->
      <div class="flex-1 bg-gray-100 dark:bg-gray-900 flex items-center justify-center">
        <ThreeJSWireframeSkeleton :size="200" />
      </div>

      <!-- Right panel skeleton (optional) -->
      <div class="w-64 border-l border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 flex-shrink-0 p-4">
        <div class="h-4 w-16 bg-gray-200 dark:bg-gray-700 rounded mb-3"></div>
        <div class="h-32 w-full bg-gray-200 dark:bg-gray-700 rounded mb-3"></div>
        <div class="h-4 w-20 bg-gray-200 dark:bg-gray-700 rounded mb-2"></div>
        <div class="h-24 w-full bg-gray-200 dark:bg-gray-700 rounded mb-3"></div>
        <div class="h-4 w-16 bg-gray-200 dark:bg-gray-700 rounded mb-2"></div>
        <div class="h-20 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
      </div>
    </div>

    <!-- Bottom toolbar skeleton -->
    <div class="h-10 bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 flex items-center px-4 gap-4">
      <div class="h-6 w-16 bg-gray-200 dark:bg-gray-700 rounded"></div>
      <div class="h-6 w-20 bg-gray-200 dark:bg-gray-700 rounded"></div>
      <div class="h-6 w-16 bg-gray-200 dark:bg-gray-700 rounded"></div>
      <div class="flex-1"></div>
      <div class="h-6 w-24 bg-gray-200 dark:bg-gray-700 rounded"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import ThreeJSWireframeSkeleton from './ThreeJSWireframeSkeleton.vue'
</script>
