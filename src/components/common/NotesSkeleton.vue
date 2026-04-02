<template>
  <div class="h-full flex flex-col bg-gray-50 dark:bg-gray-900 animate-pulse">
    <!-- Header skeleton -->
    <div class="flex items-center justify-between px-4 py-3 bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-700">
      <div class="flex items-center gap-4">
        <div class="h-5 w-20 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div class="h-8 w-32 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div class="flex items-center gap-2">
          <div class="h-8 w-14 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-8 w-14 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-8 w-14 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-8 w-14 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-8 w-14 bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
      </div>
      <div class="flex items-center gap-2">
        <div class="h-8 w-14 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div class="h-8 w-14 bg-gray-200 dark:bg-gray-700 rounded"></div>
      </div>
    </div>

    <!-- Main content skeleton -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left: Notes list skeleton -->
      <div class="w-72 border-r border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 flex-shrink-0 p-3">
        <!-- Search bar skeleton -->
        <div class="h-9 w-full bg-gray-200 dark:bg-gray-700 rounded-lg mb-4"></div>
        <!-- Category tags skeleton -->
        <div class="flex gap-2 mb-4">
          <div class="h-6 w-12 bg-gray-200 dark:bg-gray-700 rounded-full"></div>
          <div class="h-6 w-16 bg-gray-200 dark:bg-gray-700 rounded-full"></div>
          <div class="h-6 w-14 bg-gray-200 dark:bg-gray-700 rounded-full"></div>
        </div>
        <!-- Note items skeleton -->
        <div class="space-y-2">
          <div class="p-2 rounded bg-gray-100 dark:bg-gray-700">
            <div class="h-4 w-32 bg-gray-200 dark:bg-gray-600 rounded mb-1"></div>
            <div class="h-3 w-20 bg-gray-200 dark:bg-gray-600 rounded"></div>
          </div>
          <div class="p-2 rounded bg-gray-100 dark:bg-gray-700">
            <div class="h-4 w-28 bg-gray-200 dark:bg-gray-600 rounded mb-1"></div>
            <div class="h-3 w-16 bg-gray-200 dark:bg-gray-600 rounded"></div>
          </div>
          <div class="p-2 rounded bg-gray-100 dark:bg-gray-700">
            <div class="h-4 w-36 bg-gray-200 dark:bg-gray-600 rounded mb-1"></div>
            <div class="h-3 w-24 bg-gray-200 dark:bg-gray-600 rounded"></div>
          </div>
          <div class="p-2 rounded bg-gray-100 dark:bg-gray-700">
            <div class="h-4 w-24 bg-gray-200 dark:bg-gray-600 rounded mb-1"></div>
            <div class="h-3 w-18 bg-gray-200 dark:bg-gray-600 rounded"></div>
          </div>
          <div class="p-2 rounded bg-gray-100 dark:bg-gray-700">
            <div class="h-4 w-30 bg-gray-200 dark:bg-gray-600 rounded mb-1"></div>
            <div class="h-3 w-22 bg-gray-200 dark:bg-gray-600 rounded"></div>
          </div>
        </div>
      </div>

      <!-- Right: Editor skeleton -->
      <div class="flex-1 flex flex-col bg-white dark:bg-gray-900">
        <!-- Title skeleton -->
        <div class="p-4 border-b border-gray-200 dark:border-gray-700">
          <div class="h-7 w-48 bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
        <!-- Toolbar skeleton -->
        <div class="px-4 py-2 border-b border-gray-200 dark:border-gray-700 flex items-center gap-2">
          <div class="h-7 w-7 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-7 w-7 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-7 w-7 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-7 w-7 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-px h-5 bg-gray-200 dark:bg-gray-700 mx-1"></div>
          <div class="h-7 w-7 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-7 w-7 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-7 w-7 bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
        <!-- Content skeleton -->
        <div class="flex-1 p-4 space-y-3">
          <div class="h-4 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-4 w-5/6 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-4 w-4/5 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-4 w-full bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-4 w-3/4 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-32 w-full bg-gray-200 dark:bg-gray-700 rounded mt-4"></div>
          <div class="h-4 w-full bg-gray-200 dark:bg-gray-700 rounded mt-4"></div>
          <div class="h-4 w-5/6 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-4 w-2/3 bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
</script>
