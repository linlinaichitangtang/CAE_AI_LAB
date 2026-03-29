<template>
  <div class="flex flex-col items-start gap-1">
    <div class="text-xs font-medium text-gray-600">{{ title }}</div>
    <div class="flex items-center gap-1">
      <span class="text-xs text-gray-500 min-w-[50px] text-right">{{ minLabel }}</span>
      <div 
        class="w-[120px] h-3 rounded-sm"
        :style="{ background: gradientStyle }"
      ></div>
      <span class="text-xs text-gray-500 min-w-[50px]">{{ maxLabel }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  min?: number
  max?: number
  title?: string
  colormap?: 'viridis' | 'plasma' | 'inferno' | 'jet' | 'rainbow'
  unit?: string
}>(), {
  min: 0,
  max: 100,
  title: 'Von Mises Stress',
  colormap: 'viridis',
  unit: 'MPa'
})

const formatValue = (v: number): string => {
  if (Math.abs(v) >= 1e6) return (v / 1e6).toFixed(1) + 'M'
  if (Math.abs(v) >= 1e3) return (v / 1e3).toFixed(1) + 'k'
  return v.toFixed(v < 1 ? 3 : 1)
}

const minLabel = computed(() => formatValue(props.min))
const maxLabel = computed(() => formatValue(props.max))

const gradients: Record<string, string> = {
  viridis: 'linear-gradient(to right, #440154, #3b528b, #21918c, #5ec962, #fde725)',
  plasma: 'linear-gradient(to right, #0d0887, #7e03a8, #cc4778, #f89540, #f0f921)',
  inferno: 'linear-gradient(to right, #000004, #420a68, #932667, #dd513a, #fca50a, #fcffa4)',
  jet: 'linear-gradient(to right, #00007f, #0000ff, #00ffff, #00ff00, #ffff00, #ff0000, #7f0000)',
  rainbow: 'linear-gradient(to right, #ff0000, #ff7f00, #ffff00, #00ff00, #00ffff, #0000ff, #7f00ff)'
}

const gradientStyle = computed(() => gradients[props.colormap] || gradients.viridis)
</script>