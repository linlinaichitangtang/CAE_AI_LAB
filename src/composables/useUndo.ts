/**
 * useUndo composable - 全局撤销/重做快捷键 hook
 * 提供 Ctrl+Z / Ctrl+Shift+Z / Cmd+Z / Cmd+Shift+Z 快捷键支持
 */

import { onMounted, onUnmounted } from 'vue'
import { useUndoStore } from '@/stores/undo'

export function useUndo() {
  const store = useUndoStore()

  /** 处理键盘快捷键事件 */
  function handleKeydown(event: KeyboardEvent) {
    const isMod = event.ctrlKey || event.metaKey
    if (!isMod) return

    // Ctrl+Z / Cmd+Z => 撤销
    if (event.key === 'z' && !event.shiftKey) {
      event.preventDefault()
      store.undo()
      return
    }

    // Ctrl+Shift+Z / Cmd+Shift+Z => 重做
    if (event.key === 'z' && event.shiftKey) {
      event.preventDefault()
      store.redo()
      return
    }

    // Ctrl+Y / Cmd+Y => 重做（备选快捷键）
    if (event.key === 'y') {
      event.preventDefault()
      store.redo()
      return
    }
  }

  onMounted(() => {
    document.addEventListener('keydown', handleKeydown)
  })

  onUnmounted(() => {
    document.removeEventListener('keydown', handleKeydown)
  })

  return {
    canUndo: store.canUndo,
    canRedo: store.canRedo,
    lastUndoDescription: store.lastUndoDescription,
    lastRedoDescription: store.lastRedoDescription,
    undo: store.undo,
    redo: store.redo,
    execute: store.execute,
    clear: store.clear
  }
}
