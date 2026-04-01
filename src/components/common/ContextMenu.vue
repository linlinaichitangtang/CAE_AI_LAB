<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition duration-100 ease-out"
      enter-from-class="opacity-0 scale-95"
      enter-to-class="opacity-100 scale-100"
      leave-active-class="transition duration-75 ease-in"
      leave-from-class="opacity-100 scale-100"
      leave-to-class="opacity-0 scale-95"
    >
      <div
        v-if="visible"
        ref="menuRef"
        class="fixed z-[9999] min-w-[180px] max-w-[280px] max-h-[360px] overflow-y-auto
               bg-white dark:bg-gray-800 rounded-lg shadow-xl shadow-black/10 dark:shadow-black/30
               border border-gray-200 dark:border-gray-700 py-1.5
               ring-1 ring-black/5 dark:ring-white/5"
        :style="menuStyle"
        tabindex="-1"
        @keydown="onKeyDown"
      >
        <template v-for="(item, index) in items" :key="index">
          <!-- 分割线 -->
          <div
            v-if="item.divider"
            class="my-1 border-t border-gray-200 dark:border-gray-600"
          />
          <!-- 菜单项 -->
          <div
            v-else
            class="relative"
            @mouseenter="openSubmenu(index, $event)"
            @mouseleave="closeSubmenu(index)"
          >
            <button
              :ref="(el: any) => { if (el) itemRefs[index] = el }"
              :disabled="item.disabled"
              :class="[
                'w-full flex items-center gap-2.5 px-3 py-1.5 text-sm text-left transition-colors duration-75',
                'focus:outline-none',
                focusedIndex === index
                  ? 'bg-blue-50 dark:bg-blue-900/40 text-blue-700 dark:text-blue-200'
                  : item.disabled
                    ? 'text-gray-400 dark:text-gray-500 cursor-not-allowed'
                    : item.danger
                      ? 'text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/30'
                      : 'text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700'
              ]"
              @click="onItemClick(item)"
            >
              <!-- 图标 -->
              <span v-if="item.icon" class="w-4 h-4 flex-shrink-0 flex items-center justify-center text-sm">
                {{ item.icon }}
              </span>
              <span v-else class="w-4 flex-shrink-0" />
              <!-- 标签 -->
              <span class="flex-1 truncate">{{ item.label }}</span>
              <!-- 快捷键 -->
              <span
                v-if="item.shortcut"
                class="ml-auto text-xs text-gray-400 dark:text-gray-500 flex-shrink-0"
              >
                {{ item.shortcut }}
              </span>
              <!-- 子菜单箭头 -->
              <span
                v-if="item.children && item.children.length > 0"
                class="ml-auto text-xs text-gray-400 dark:text-gray-500 flex-shrink-0"
              >
                &#9656;
              </span>
            </button>
            <!-- 子菜单 -->
            <div
              v-if="item.children && item.children.length > 0 && activeSubmenuIndex === index"
              class="absolute left-full top-0 ml-1 min-w-[160px] max-w-[240px] max-h-[300px] overflow-y-auto
                     bg-white dark:bg-gray-800 rounded-lg shadow-xl shadow-black/10 dark:shadow-black/30
                     border border-gray-200 dark:border-gray-700 py-1.5
                     ring-1 ring-black/5 dark:ring-white/5 z-[10000]"
            >
              <template v-for="(child, childIndex) in item.children" :key="childIndex">
                <div
                  v-if="child.divider"
                  class="my-1 border-t border-gray-200 dark:border-gray-600"
                />
                <button
                  v-else
                  :disabled="child.disabled"
                  :class="[
                    'w-full flex items-center gap-2.5 px-3 py-1.5 text-sm text-left transition-colors duration-75',
                    'focus:outline-none',
                    child.disabled
                      ? 'text-gray-400 dark:text-gray-500 cursor-not-allowed'
                      : child.danger
                        ? 'text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/30'
                        : 'text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700'
                  ]"
                  @click.stop="onChildItemClick(child)"
                >
                  <span v-if="child.icon" class="w-4 h-4 flex-shrink-0 flex items-center justify-center text-sm">
                    {{ child.icon }}
                  </span>
                  <span v-else class="w-4 flex-shrink-0" />
                  <span class="flex-1 truncate">{{ child.label }}</span>
                </button>
              </template>
            </div>
          </div>
        </template>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted, onBeforeUnmount } from 'vue'

export interface MenuItem {
  label: string
  icon?: string
  shortcut?: string
  danger?: boolean
  divider?: boolean
  disabled?: boolean
  children?: MenuItem[]
  action?: () => void
}

const props = defineProps<{
  items: MenuItem[]
  x: number
  y: number
  visible: boolean
}>()

const emit = defineEmits<{
  close: []
}>()

const menuRef = ref<HTMLElement | null>(null)
const focusedIndex = ref(-1)
const activeSubmenuIndex = ref(-1)
const itemRefs: Record<number, HTMLElement> = {}

// 计算菜单位置，确保不超出视口
const menuStyle = computed(() => {
  const padding = 8
  let left = props.x + padding
  let top = props.y + padding

  // 如果菜单还没有渲染，先返回初始位置
  if (!menuRef.value) {
    return { left: `${left}px`, top: `${top}px` }
  }

  const menuRect = menuRef.value.getBoundingClientRect()
  const viewportWidth = window.innerWidth
  const viewportHeight = window.innerHeight

  // 右边超出
  if (left + menuRect.width > viewportWidth) {
    left = props.x - menuRect.width - padding
  }
  // 底部超出
  if (top + menuRect.height > viewportHeight) {
    top = props.y - menuRect.height - padding
  }
  // 左边超出
  if (left < padding) {
    left = padding
  }
  // 顶部超出
  if (top < padding) {
    top = padding
  }

  return { left: `${left}px`, top: `${top}px` }
})

// 点击菜单项
function onItemClick(item: MenuItem) {
  if (item.disabled || item.divider) return
  if (item.children && item.children.length > 0) {
    // Toggle submenu for items with children
    activeSubmenuIndex.value = activeSubmenuIndex.value === -1 ? -1 : -1
    return
  }
  item.action?.()
  emit('close')
}

// 点击子菜单项
function onChildItemClick(child: MenuItem) {
  if (child.disabled || child.divider) return
  child.action?.()
  emit('close')
}

// 打开子菜单
function openSubmenu(index: number, _event: MouseEvent) {
  focusedIndex.value = index
  const item = props.items[index]
  if (item && item.children && item.children.length > 0) {
    activeSubmenuIndex.value = index
  } else {
    activeSubmenuIndex.value = -1
  }
}

// 关闭子菜单
function closeSubmenu(index: number) {
  if (activeSubmenuIndex.value === index) {
    activeSubmenuIndex.value = -1
  }
}

// 键盘导航
function onKeyDown(e: KeyboardEvent) {
  const actionableItems = props.items.filter(i => !i.divider && !i.disabled)

  if (e.key === 'Escape') {
    e.preventDefault()
    emit('close')
    return
  }

  if (e.key === 'ArrowDown') {
    e.preventDefault()
    focusedIndex.value = getNextFocusableIndex(focusedIndex.value, 1)
    scrollIntoView()
    return
  }

  if (e.key === 'ArrowUp') {
    e.preventDefault()
    focusedIndex.value = getNextFocusableIndex(focusedIndex.value, -1)
    scrollIntoView()
    return
  }

  if (e.key === 'Enter' || e.key === ' ') {
    e.preventDefault()
    const item = props.items[focusedIndex.value]
    if (item && !item.disabled && !item.divider) {
      onItemClick(item)
    }
    return
  }
}

function getNextFocusableIndex(current: number, direction: 1 | -1): number {
  const len = props.items.length
  let next = current

  for (let i = 0; i < len; i++) {
    next = (next + direction + len) % len
    const item = props.items[next]
    if (item && !item.divider && !item.disabled) {
      return next
    }
  }

  return current
}

function scrollIntoView() {
  nextTick(() => {
    const el = itemRefs[focusedIndex.value]
    if (el) {
      el.scrollIntoView({ block: 'nearest' })
    }
  })
}

// 点击外部关闭
function onDocumentClick(e: MouseEvent) {
  if (!props.visible) return
  if (menuRef.value && !menuRef.value.contains(e.target as Node)) {
    emit('close')
  }
}

// 按 Esc 关闭
function onDocumentKeyDown(e: KeyboardEvent) {
  if (!props.visible) return
  if (e.key === 'Escape') {
    emit('close')
  }
}

// 菜单打开时聚焦并重置键盘索引
watch(() => props.visible, async (val) => {
  if (val) {
    focusedIndex.value = -1
    activeSubmenuIndex.value = -1
    Object.keys(itemRefs).forEach(k => delete itemRefs[Number(k)])
    await nextTick()
    // 聚焦菜单以接收键盘事件
    menuRef.value?.focus()
  }
})

onMounted(() => {
  document.addEventListener('click', onDocumentClick, true)
  document.addEventListener('keydown', onDocumentKeyDown, true)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', onDocumentClick, true)
  document.removeEventListener('keydown', onDocumentKeyDown, true)
})
</script>
