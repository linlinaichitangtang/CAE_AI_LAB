<template>
  <footer class="h-8 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] flex items-center px-4 text-xs">
    <!-- Left Section -->
    <div class="flex items-center gap-4 text-[var(--text-muted)]">
      <span class="flex items-center gap-1">
        <span class="text-[var(--accent-green)]">●</span>
        就绪
      </span>
      <span class="hidden md:inline">CAELab v0.1.0</span>
    </div>
    
    <!-- Spacer -->
    <div class="flex-1"></div>
    
    <!-- Right Section -->
    <div class="flex items-center gap-4 text-[var(--text-muted)]">
      <span class="hidden md:inline">本地存储</span>
      <span>{{ cursorPosition }}</span>
      <span>UTF-8</span>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const cursorPosition = ref('行 1, 列 1')
</script>