<template>
  <div class="skeleton" :class="[variant, { animated: animate }]" :style="customStyle"></div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  variant?: 'text' | 'circle' | 'rect' | 'card'
  width?: string
  height?: string
  animate?: boolean
  radius?: string
}>(), {
  variant: 'text',
  animate: true,
})

const customStyle = computed(() => ({
  width: props.width || (props.variant === 'circle' ? '40px' : '100%'),
  height: props.height || (props.variant === 'text' ? '16px' : props.variant === 'circle' ? '40px' : '80px'),
  borderRadius: props.radius || (props.variant === 'circle' ? '50%' : props.variant === 'card' ? '8px' : '4px'),
}))
</script>

<style scoped>
.skeleton {
  background: var(--bg-muted, #f0f0f0);
}
.skeleton.animated {
  animation: skeleton-pulse 1.5s ease-in-out infinite;
}
@keyframes skeleton-pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.4; }
}
</style>
