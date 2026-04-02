<template>
  <Teleport to="body">
    <Transition name="confirm-fade">
      <div
        v-if="visible"
        class="fixed inset-0 z-[100] flex items-center justify-center"
        @click.self="handleCancel"
      >
        <!-- Backdrop -->
        <div class="absolute inset-0 bg-black/50 backdrop-blur-sm"></div>

        <!-- Dialog -->
        <div
          class="relative bg-white dark:bg-gray-800 rounded-xl shadow-2xl w-full max-w-md mx-4 animate-slide-in"
          @keydown.enter="handleConfirm"
          @keydown.escape="handleCancel"
          tabindex="0"
          ref="dialogRef"
        >
          <!-- Icon + Title -->
          <div class="flex items-start gap-4 p-6 pb-0">
            <!-- Type icon -->
            <div
              class="flex-shrink-0 w-10 h-10 rounded-full flex items-center justify-center"
              :class="iconBgClass"
            >
              <span v-if="icon" class="text-xl">{{ icon }}</span>
              <svg v-else class="w-5 h-5" :class="iconColorClass" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path v-if="type === 'danger'" stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z" />
                <path v-else-if="type === 'warning'" stroke-linecap="round" stroke-linejoin="round" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                <path v-else stroke-linecap="round" stroke-linejoin="round" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>

            <div class="flex-1 min-w-0">
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">{{ title }}</h3>
            </div>
          </div>

          <!-- Message -->
          <div class="px-6 pt-3 pb-2">
            <p class="text-sm text-gray-600 dark:text-gray-400 leading-relaxed" v-html="message"></p>
          </div>

          <!-- "Don't show again" checkbox -->
          <div v-if="showCheckbox" class="px-6 pb-2">
            <label class="flex items-center gap-2 cursor-pointer select-none">
              <input
                type="checkbox"
                v-model="dontShowAgain"
                class="w-4 h-4 rounded border-gray-300 dark:border-gray-600 text-blue-600 dark:text-blue-500 focus:ring-blue-500 dark:bg-gray-700"
              />
              <span class="text-xs text-gray-500 dark:text-gray-400">{{ checkboxText }}</span>
            </label>
          </div>

          <!-- Actions -->
          <div class="flex justify-end gap-3 p-6 pt-4">
            <button
              @click="handleCancel"
              class="px-4 py-2 text-sm font-medium rounded-lg transition-colors bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600"
            >
              {{ cancelText }}
            </button>
            <button
              @click="handleConfirm"
              class="px-4 py-2 text-sm font-medium rounded-lg transition-colors text-white"
              :class="confirmButtonClass"
            >
              {{ confirmText }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'

interface Props {
  visible: boolean
  title: string
  message: string
  confirmText?: string
  cancelText?: string
  type?: 'danger' | 'warning' | 'info'
  showCheckbox?: boolean
  checkboxText?: string
  icon?: string
  dontShowAgainKey?: string
}

const props = withDefaults(defineProps<Props>(), {
  confirmText: '确认',
  cancelText: '取消',
  type: 'danger',
  showCheckbox: false,
  checkboxText: '不再提示此操作',
  icon: '',
  dontShowAgainKey: '',
})

const emit = defineEmits<{
  'confirm': [dontShowAgain: boolean]
  'cancel': []
  'update:visible': [value: boolean]
}>()

const dialogRef = ref<HTMLDivElement | null>(null)
const dontShowAgain = ref(false)

// Focus dialog when visible
watch(() => props.visible, async (val) => {
  if (val) {
    dontShowAgain.value = false
    await nextTick()
    dialogRef.value?.focus()
  }
})

const iconBgClass = computed(() => {
  switch (props.type) {
    case 'danger': return 'bg-red-100 dark:bg-red-900/30'
    case 'warning': return 'bg-yellow-100 dark:bg-yellow-900/30'
    case 'info': return 'bg-blue-100 dark:bg-blue-900/30'
    default: return 'bg-gray-100 dark:bg-gray-700'
  }
})

const iconColorClass = computed(() => {
  switch (props.type) {
    case 'danger': return 'text-red-600 dark:text-red-400'
    case 'warning': return 'text-yellow-600 dark:text-yellow-400'
    case 'info': return 'text-blue-600 dark:text-blue-400'
    default: return 'text-gray-600 dark:text-gray-400'
  }
})

const confirmButtonClass = computed(() => {
  switch (props.type) {
    case 'danger': return 'bg-red-600 hover:bg-red-700 focus:ring-red-500'
    case 'warning': return 'bg-yellow-600 hover:bg-yellow-700 focus:ring-yellow-500'
    case 'info': return 'bg-blue-600 hover:bg-blue-700 focus:ring-blue-500'
    default: return 'bg-gray-600 hover:bg-gray-700'
  }
})

function handleConfirm() {
  if (dontShowAgain.value && props.dontShowAgainKey) {
    localStorage.setItem(`caelab_dont_show_${props.dontShowAgainKey}`, 'true')
  }
  emit('confirm', dontShowAgain.value)
  emit('update:visible', false)
}

function handleCancel() {
  emit('cancel')
  emit('update:visible', false)
}
</script>

<style scoped>
.confirm-fade-enter-active,
.confirm-fade-leave-active {
  transition: opacity 0.2s ease;
}

.confirm-fade-enter-from,
.confirm-fade-leave-to {
  opacity: 0;
}

.animate-slide-in {
  animation: slide-in 0.2s ease-out;
}

@keyframes slide-in {
  from {
    opacity: 0;
    transform: scale(0.95) translateY(-10px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}
</style>
