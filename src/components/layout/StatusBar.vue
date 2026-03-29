<template>
  <footer class="h-6 bg-white border-t border-gray-200 flex items-center px-4 text-xs text-gray-500 justify-between">
    <span>CAELab v0.1.0</span>
    <span>{{ status }}</span>
  </footer>
</template>

<script setup lang="ts">
import { ref } from 'vue'
const status = ref('就绪')
</script>