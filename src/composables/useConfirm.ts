import { reactive, ref } from 'vue'

interface ConfirmOptions {
  title: string
  message: string
  type?: 'danger' | 'warning' | 'info'
  confirmText?: string
  cancelText?: string
  dontShowAgainKey?: string
}

const confirmState = reactive({
  visible: false,
  title: '',
  message: '',
  type: 'danger' as 'danger' | 'warning' | 'info',
  confirmText: '确认',
  cancelText: '取消',
  dontShowAgainKey: '',
  resolve: null as ((value: boolean) => void) | null,
})

const confirmDialogRef = ref<{ handleConfirm: (dontShow: boolean) => void; handleCancel: () => void } | null>(null)

/**
 * 检查某个操作是否已设置"不再提示"
 * @param key 操作的唯一标识
 */
export function shouldSkipConfirm(key: string): boolean {
  return localStorage.getItem(`caelab_dont_show_${key}`) === 'true'
}

/**
 * 设置"不再提示"
 * @param key 操作的唯一标识
 */
export function setDontShowAgain(key: string): void {
  localStorage.setItem(`caelab_dont_show_${key}`, 'true')
}

/**
 * 重置"不再提示"设置
 * @param key 操作的唯一标识
 */
export function resetDontShowAgain(key: string): void {
  localStorage.removeItem(`caelab_dont_show_${key}`)
}

/**
 * 确认对话框 composable
 *
 * 用法：
 * ```ts
 * const { confirm } = useConfirm()
 *
 * // 基本用法
 * const ok = await confirm({
 *   title: '删除项目',
 *   message: '确定要删除此项目吗？此操作不可撤销。',
 *   type: 'danger',
 * })
 * if (ok) { /* 执行删除 *\/ }
 *
 * // 带"不再提示"
 * const ok = await confirm({
 *   title: '删除项目',
 *   message: '确定要删除此项目吗？',
 *   dontShowAgainKey: 'delete-project',
 * })
 * ```
 */
export function useConfirm() {
  /**
   * 显示确认对话框，返回 Promise<boolean>
   * 用户点击确认返回 true，取消返回 false
   */
  async function confirm(options: ConfirmOptions): Promise<boolean> {
    // 如果已设置"不再提示"，直接返回 true
    if (options.dontShowAgainKey && shouldSkipConfirm(options.dontShowAgainKey)) {
      return true
    }

    confirmState.title = options.title
    confirmState.message = options.message
    confirmState.type = options.type || 'danger'
    confirmState.confirmText = options.confirmText || '确认'
    confirmState.cancelText = options.cancelText || '取消'
    confirmState.dontShowAgainKey = options.dontShowAgainKey || ''
    confirmState.visible = true

    return new Promise<boolean>((resolve) => {
      confirmState.resolve = resolve
    })
  }

  /**
   * 内部方法：用户确认
   */
  function handleConfirm(dontShowAgain: boolean): void {
    confirmState.visible = false
    if (dontShowAgain && confirmState.dontShowAgainKey) {
      setDontShowAgain(confirmState.dontShowAgainKey)
    }
    confirmState.resolve?.(true)
    confirmState.resolve = null
  }

  /**
   * 内部方法：用户取消
   */
  function handleCancel(): void {
    confirmState.visible = false
    confirmState.resolve?.(false)
    confirmState.resolve = null
  }

  return {
    confirmState,
    confirm,
    handleConfirm,
    handleCancel,
    shouldSkipConfirm,
    setDontShowAgain,
    resetDontShowAgain,
  }
}
