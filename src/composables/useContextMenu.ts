import { ref } from 'vue'
import type { MenuItem } from '@/components/common/ContextMenu.vue'

export function useContextMenu() {
  const visible = ref(false)
  const x = ref(0)
  const y = ref(0)
  const items = ref<MenuItem[]>([])

  function show(event: MouseEvent, menuItems: MenuItem[]) {
    event.preventDefault()
    event.stopPropagation()
    x.value = event.clientX
    y.value = event.clientY
    items.value = menuItems
    visible.value = true
  }

  function hide() {
    visible.value = false
  }

  return { visible, x, y, items, show, hide }
}
