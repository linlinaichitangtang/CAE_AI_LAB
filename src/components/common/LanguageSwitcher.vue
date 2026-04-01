<template>
  <div class="relative" ref="switcherRef">
    <button
      class="topbar-action"
      :title="currentLocale === 'zh-CN' ? 'Switch to English' : '切换到中文'"
      @click="toggleDropdown"
    >
      <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="10"/>
        <line x1="2" y1="12" x2="22" y2="12"/>
        <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/>
      </svg>
    </button>

    <Transition name="dropdown">
      <div v-if="showDropdown" class="lang-dropdown">
        <button
          v-for="locale in locales"
          :key="locale.value"
          :class="['lang-item', { active: currentLocale === locale.value }]"
          @click="switchLocale(locale.value)"
        >
          <span class="lang-flag">{{ locale.flag }}</span>
          <span class="lang-label">{{ locale.label }}</span>
          <svg v-if="currentLocale === locale.value" class="w-4 h-4 text-[var(--primary)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
        </button>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'

const { locale } = useI18n()

const switcherRef = ref<HTMLElement | null>(null)
const showDropdown = ref(false)

const currentLocale = computed(() => locale.value)

const locales = [
  { value: 'zh-CN', label: '简体中文', flag: '🇨🇳' },
  { value: 'en-US', label: 'English', flag: '🇺🇸' },
]

function toggleDropdown() {
  showDropdown.value = !showDropdown.value
}

function switchLocale(newLocale: string) {
  locale.value = newLocale
  localStorage.setItem('caelab-locale', newLocale)
  showDropdown.value = false
}

function handleClickOutside(event: MouseEvent) {
  if (switcherRef.value && !switcherRef.value.contains(event.target as Node)) {
    showDropdown.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.lang-dropdown {
  position: absolute;
  top: calc(100% + 6px);
  right: 0;
  width: 160px;
  background: var(--bg-surface, #ffffff);
  border: 1px solid var(--border-default, #e2e8f0);
  border-radius: var(--radius-lg, 8px);
  box-shadow: var(--shadow-lg, 0 10px 15px -3px rgba(0, 0, 0, 0.1));
  z-index: 100;
  overflow: hidden;
  padding: 4px;
}

.lang-item {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 8px 12px;
  border: none;
  background: transparent;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  color: var(--text-primary, #0f172a);
  transition: background 0.15s ease;
}

.lang-item:hover {
  background: var(--bg-elevated, #f1f5f9);
}

.lang-item.active {
  background: var(--primary-glow, rgba(79, 70, 229, 0.08));
}

.lang-flag {
  font-size: 16px;
}

.lang-label {
  flex: 1;
  text-align: left;
}

/* Dropdown animation */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.15s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
