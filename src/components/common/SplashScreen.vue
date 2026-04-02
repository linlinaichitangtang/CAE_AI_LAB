<template>
  <transition name="splash">
    <div v-if="visible" class="splash-screen">
      <div class="splash-content">
        <div class="logo-icon">
          <svg class="w-10 h-10 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2L2 7l10 5 10-5-10-5z"/>
            <path d="M2 17l10 5 10-5"/>
            <path d="M2 12l10 5 10-5"/>
          </svg>
        </div>
        <div class="logo">CAELab</div>
        <div class="tagline">科研与工程创作一体化工作台</div>
        <div class="loading-dots">
          <span></span><span></span><span></span>
        </div>
      </div>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const props = defineProps<{ duration?: number }>()
const visible = ref(true)

onMounted(() => {
  setTimeout(() => { visible.value = false }, props.duration ?? 1500)
})
</script>

<style scoped>
.splash-screen {
  position: fixed;
  inset: 0;
  z-index: 9999;
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
  display: flex;
  align-items: center;
  justify-content: center;
}

.splash-content {
  text-align: center;
}

.logo-icon {
  width: 64px;
  height: 64px;
  border-radius: 16px;
  background: linear-gradient(135deg, #2563EB, #3B82F6);
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 20px;
  box-shadow: 0 8px 24px rgba(37, 99, 235, 0.4);
}

.logo {
  font-size: 48px;
  font-weight: 800;
  color: #3b82f6;
  letter-spacing: -2px;
}

.tagline {
  font-size: 14px;
  color: #94a3b8;
  margin-top: 8px;
}

.loading-dots {
  margin-top: 32px;
}

.loading-dots span {
  display: inline-block;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #3b82f6;
  margin: 0 4px;
  animation: bounce 1.4s infinite ease-in-out both;
}

.loading-dots span:nth-child(1) { animation-delay: -0.32s; }
.loading-dots span:nth-child(2) { animation-delay: -0.16s; }

@keyframes bounce {
  0%, 80%, 100% { transform: scale(0); }
  40% { transform: scale(1); }
}

.splash-leave-active {
  transition: opacity 0.3s ease;
}

.splash-leave-to {
  opacity: 0;
}
</style>
