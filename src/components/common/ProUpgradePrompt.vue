<template>
  <Teleport to="body">
    <Transition name="prompt">
      <div v-if="visible" class="prompt-overlay" @click.self="handleClose">
        <div class="prompt-card">
          <button class="prompt-close" @click="handleClose">
            <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </button>

          <!-- Icon -->
          <div class="prompt-icon">
            <svg class="w-8 h-8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M12 2L2 7l10 5 10-5-10-5z"/>
              <path d="M2 17l10 5 10-5"/>
              <path d="M2 12l10 5 10-5"/>
            </svg>
          </div>

          <h3 class="prompt-title">此功能需要 Pro 会员</h3>
          <p class="prompt-desc">{{ featureDescription || '升级到 Pro 会员以解锁此功能及更多高级特性。' }}</p>

          <!-- Mini Feature Comparison -->
          <div class="prompt-features">
            <div class="prompt-feature-item">
              <svg class="w-4 h-4 text-green-500" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
              <span>结构仿真 200 次/月</span>
            </div>
            <div class="prompt-feature-item">
              <svg class="w-4 h-4 text-green-500" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
              <span>CFD 仿真 50 次/月</span>
            </div>
            <div class="prompt-feature-item">
              <svg class="w-4 h-4 text-green-500" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
              <span>高级仿真（热耦合/疲劳）</span>
            </div>
            <div class="prompt-feature-item">
              <svg class="w-4 h-4 text-green-500" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
              <span>云端存储 10 GB</span>
            </div>
            <div class="prompt-feature-item">
              <svg class="w-4 h-4 text-green-500" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
              <span>Python API</span>
            </div>
          </div>

          <div class="prompt-actions">
            <button class="btn btn-secondary" @click="handleClose">稍后再说</button>
            <button class="btn btn-primary" @click="handleUpgrade">升级到 Pro</button>
          </div>

          <p class="prompt-note">¥99/月起，年付享 8 折优惠</p>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'

const props = defineProps<{
  visible: boolean
  featureDescription?: string
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'upgrade'): void
}>()

const router = useRouter()

function handleClose() {
  emit('close')
}

function handleUpgrade() {
  emit('upgrade')
  router.push('/membership')
}
</script>

<style scoped>
.prompt-overlay {
  position: fixed;
  inset: 0;
  z-index: 1000;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.prompt-card {
  position: relative;
  width: 100%;
  max-width: 380px;
  background: var(--bg-surface, #ffffff);
  border-radius: var(--radius-xl, 16px);
  padding: 32px 28px;
  text-align: center;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.2);
}

.prompt-close {
  position: absolute;
  top: 12px;
  right: 12px;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: var(--text-muted, #9ca3af);
  cursor: pointer;
  border-radius: var(--radius-md, 8px);
}

.prompt-close:hover {
  background: var(--bg-elevated, #f3f4f6);
  color: var(--text-primary, #1f2937);
}

.prompt-icon {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: linear-gradient(135deg, rgba(37, 99, 235, 0.1), rgba(59, 130, 246, 0.1));
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 16px;
  color: var(--primary, #2563EB);
}

.prompt-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary, #1f2937);
  margin-bottom: 8px;
}

.prompt-desc {
  font-size: 14px;
  color: var(--text-secondary, #4b5563);
  margin-bottom: 20px;
  line-height: 1.5;
}

.prompt-features {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 16px;
  background: var(--bg-elevated, #f3f4f6);
  border-radius: var(--radius-md, 8px);
  margin-bottom: 24px;
  text-align: left;
}

.prompt-feature-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--text-primary, #1f2937);
}

.prompt-actions {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
}

.prompt-actions .btn {
  flex: 1;
}

.prompt-note {
  font-size: 12px;
  color: var(--text-muted, #9ca3af);
}

/* Transition */
.prompt-enter-active,
.prompt-leave-active {
  transition: opacity 0.2s ease;
}

.prompt-enter-active .prompt-card,
.prompt-leave-active .prompt-card {
  transition: transform 0.2s var(--ease-out, cubic-bezier(0.16, 1, 0.3, 1));
}

.prompt-enter-from,
.prompt-leave-to {
  opacity: 0;
}

.prompt-enter-from .prompt-card,
.prompt-leave-to .prompt-card {
  transform: scale(0.95) translateY(10px);
}
</style>
