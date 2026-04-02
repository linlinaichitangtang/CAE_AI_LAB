<template>
  <router-view v-slot="{ Component, route }">
    <transition :name="transitionName" mode="out-in">
      <component :is="Component" :key="route.path" />
    </transition>
  </router-view>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { usePlatform } from '@/composables/usePlatform'

const { isTouchDevice } = usePlatform()

const transitionName = computed(() => isTouchDevice.value ? 'slide' : 'fade')
</script>

<style>
/* 移动端滑动过渡 */
.slide-enter-active,
.slide-leave-active {
  transition: transform 0.25s ease, opacity 0.25s ease;
}
.slide-enter-from {
  transform: translateX(20px);
  opacity: 0;
}
.slide-leave-to {
  transform: translateX(-20px);
  opacity: 0;
}

/* 桌面端淡入淡出 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
