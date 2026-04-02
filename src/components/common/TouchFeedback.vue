<template>
  <div
    class="touch-feedback"
    :class="{ pressed: isPressed }"
    @pointerdown="isPressed = true"
    @pointerup="isPressed = false"
    @pointerleave="isPressed = false"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const isPressed = ref(false)
</script>

<style scoped>
.touch-feedback {
  transition: transform 0.1s ease, opacity 0.1s ease;
  -webkit-tap-highlight-color: transparent;
}
.touch-feedback.pressed {
  transform: scale(0.97);
  opacity: 0.7;
}
</style>
