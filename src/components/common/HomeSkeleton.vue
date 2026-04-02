<template>
  <div class="h-full flex flex-col p-6 overflow-auto bg-[var(--bg-base)] animate-pulse">
    <!-- Welcome Header skeleton -->
    <div class="flex justify-between items-center mb-8">
      <div>
        <div class="h-7 w-40 bg-gray-200 dark:bg-gray-700 rounded mb-2"></div>
        <div class="h-4 w-56 bg-gray-200 dark:bg-gray-700 rounded"></div>
      </div>
      <div class="flex items-center gap-2">
        <div class="h-9 w-28 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div class="h-9 w-16 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div class="h-9 w-16 bg-gray-200 dark:bg-gray-700 rounded"></div>
      </div>
    </div>

    <!-- Recent Projects skeleton -->
    <div class="mb-8">
      <div class="h-4 w-20 bg-gray-200 dark:bg-gray-700 rounded mb-4"></div>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div v-for="i in 3" :key="i" class="bg-[var(--bg-surface)] rounded-xl p-5 border border-[var(--border-subtle)]">
          <div class="flex items-center gap-3 mb-3">
            <div class="w-10 h-10 rounded-lg bg-gray-200 dark:bg-gray-700"></div>
            <div class="flex-1">
              <div class="h-4 w-28 bg-gray-200 dark:bg-gray-700 rounded mb-1"></div>
              <div class="h-3 w-20 bg-gray-200 dark:bg-gray-700 rounded"></div>
            </div>
          </div>
          <div class="h-4 w-full bg-gray-200 dark:bg-gray-700 rounded mb-2"></div>
          <div class="h-4 w-3/4 bg-gray-200 dark:bg-gray-700 rounded mb-4"></div>
          <div class="flex gap-2 pt-3 border-t border-[var(--border-subtle)]">
            <div class="h-8 w-8 bg-gray-200 dark:bg-gray-700 rounded"></div>
            <div class="h-8 w-8 bg-gray-200 dark:bg-gray-700 rounded"></div>
            <div class="h-8 w-8 bg-gray-200 dark:bg-gray-700 rounded"></div>
            <div class="h-8 w-8 bg-gray-200 dark:bg-gray-700 rounded"></div>
          </div>
        </div>
      </div>
    </div>

    <!-- Module Cards skeleton -->
    <div class="mt-auto pt-8 border-t border-[var(--border-subtle)]">
      <div class="h-4 w-20 bg-gray-200 dark:bg-gray-700 rounded mb-4"></div>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
        <div v-for="i in 4" :key="i" class="bg-[var(--bg-surface)] rounded-xl p-5 border border-[var(--border-subtle)]">
          <div class="w-12 h-12 rounded-xl bg-gray-200 dark:bg-gray-700 mb-3"></div>
          <div class="h-4 w-20 bg-gray-200 dark:bg-gray-700 rounded mb-1"></div>
          <div class="h-3 w-32 bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
</script>
