/**
 * V1.1-011: 全局快捷键管理系统
 * 统一管理所有快捷键注册、冲突检测和上下文感知
 */

import { ref, readonly } from 'vue'
import { defaultHotkeys, type HotkeyConfig } from '@/utils/defaultHotkeys'

// ============ 类型定义 ============

export interface HotkeyBinding {
  id: string
  keys: string
  description: string
  category: string
  action: () => void
  enabled: boolean
}

// ============ 状态 ============

const STORAGE_KEY = 'caelab-hotkeys-custom'

/** 当前活跃的上下文（模块） */
const activeContext = ref<string>('global')

/** 快捷键注册表 */
const hotkeyRegistry = new Map<string, HotkeyBinding>()

/** 自定义快捷键映射（持久化） */
const customKeys = ref<Record<string, string>>({})

/** 是否正在捕获按键 */
const isCapturing = ref(false)

/** 当前捕获到的按键 */
const capturedKeys = ref<string>('')

/** 当前正在自定义的快捷键 ID */
const capturingId = ref<string | null>(null)

// ============ 初始化 ============

/**
 * 从 localStorage 加载自定义快捷键
 */
function loadCustomKeys(): void {
  try {
    const saved = localStorage.getItem(STORAGE_KEY)
    if (saved) {
      customKeys.value = JSON.parse(saved)
    }
  } catch (e) {
    console.error('加载自定义快捷键失败:', e)
    customKeys.value = {}
  }
}

/**
 * 保存自定义快捷键到 localStorage
 */
function saveCustomKeys(): void {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(customKeys.value))
}

// ============ 快捷键解析 ============

/**
 * 解析快捷键字符串为规范化格式
 * 'ctrl+shift+z' -> 'ctrl+shift+z'
 * 'Ctrl+S' -> 'ctrl+s'
 */
function normalizeKeys(keys: string): string {
  return keys
    .toLowerCase()
    .trim()
    .split('+')
    .map((k) => k.trim())
    .sort((a, b) => {
      // 修饰键排在前面
      const order: Record<string, number> = {
        ctrl: 0,
        alt: 1,
        shift: 2,
        meta: 3,
      }
      const aOrder = order[a] ?? 10
      const bOrder = order[b] ?? 10
      return aOrder - bOrder
    })
    .join('+')
}

/**
 * 将 KeyboardEvent 转换为快捷键字符串
 */
function eventToKeys(e: KeyboardEvent): string {
  const parts: string[] = []

  if (e.ctrlKey || e.metaKey) parts.push('ctrl')
  if (e.altKey) parts.push('alt')
  if (e.shiftKey) parts.push('shift')

  // 主键
  let key = e.key.toLowerCase()

  // 规范化功能键
  if (key === ' ') key = 'space'
  if (key === 'escape') key = 'escape'
  if (key.startsWith('f') && key.length <= 3) {
    const num = parseInt(key.slice(1))
    if (num >= 1 && num <= 24) key = `f${num}`
  }
  if (key === ',') key = ','
  if (key === '/') key = '/'
  if (key === '.') key = '.'

  // 如果主键是修饰键本身，忽略
  if (['ctrl', 'alt', 'shift', 'meta'].includes(key)) return ''

  parts.push(key)

  return parts.join('+')
}

/**
 * 检查 KeyboardEvent 是否匹配快捷键字符串
 */
function matchKeys(e: KeyboardEvent, keys: string): boolean {
  const normalized = normalizeKeys(keys)
  const eventKeys = eventToKeys(e)
  return eventKeys === normalized
}

// ============ 核心 API ============

/**
 * 注册快捷键
 */
function registerHotkey(binding: HotkeyBinding): void {
  // 应用自定义快捷键
  const customKey = customKeys.value[binding.id]
  const effectiveKeys = customKey || binding.keys

  hotkeyRegistry.set(binding.id, {
    ...binding,
    keys: normalizeKeys(effectiveKeys),
  })
}

/**
 * 注销快捷键
 */
function unregisterHotkey(id: string): void {
  hotkeyRegistry.delete(id)
}

/**
 * 获取所有已注册的快捷键
 */
function getHotkeys(): HotkeyBinding[] {
  return Array.from(hotkeyRegistry.values())
}

/**
 * 检查快捷键是否冲突
 */
function isHotkeyConflict(keys: string, excludeId?: string): boolean {
  const normalized = normalizeKeys(keys)

  for (const [id, binding] of hotkeyRegistry) {
    if (id === excludeId) continue
    if (normalizeKeys(binding.keys) === normalized) {
      return true
    }
  }

  // 也检查默认快捷键
  for (const defaultHk of defaultHotkeys) {
    if (defaultHk.id === excludeId) continue
    if (normalizeKeys(defaultHk.keys) === normalized) {
      // 如果该默认快捷键已被自定义覆盖，跳过
      if (customKeys.value[defaultHk.id]) continue
      return true
    }
  }

  return false
}

/**
 * 获取冲突的快捷键信息
 */
function getConflictInfo(keys: string, excludeId?: string): { id: string; description: string } | null {
  const normalized = normalizeKeys(keys)

  for (const [id, binding] of hotkeyRegistry) {
    if (id === excludeId) continue
    if (normalizeKeys(binding.keys) === normalized) {
      return { id, description: binding.description }
    }
  }

  for (const defaultHk of defaultHotkeys) {
    if (defaultHk.id === excludeId) continue
    if (normalizeKeys(defaultHk.keys) === normalized) {
      if (customKeys.value[defaultHk.id]) continue
      return { id: defaultHk.id, description: defaultHk.description }
    }
  }

  return null
}

/**
 * 设置活跃上下文
 */
function setActiveContext(context: string): void {
  activeContext.value = context
}

/**
 * 自定义快捷键
 */
function customizeHotkey(id: string, newKeys: string): boolean {
  const normalized = normalizeKeys(newKeys)

  // 检查冲突
  if (isHotkeyConflict(normalized, id)) {
    return false
  }

  customKeys.value[id] = normalized
  saveCustomKeys()

  // 更新注册表
  const binding = hotkeyRegistry.get(id)
  if (binding) {
    binding.keys = normalized
  }

  return true
}

/**
 * 重置单个快捷键为默认
 */
function resetHotkey(id: string): void {
  delete customKeys.value[id]
  saveCustomKeys()

  const defaultHk = defaultHotkeys.find((h) => h.id === id)
  const binding = hotkeyRegistry.get(id)
  if (defaultHk && binding) {
    binding.keys = normalizeKeys(defaultHk.keys)
  }
}

/**
 * 重置所有快捷键为默认
 */
function resetAllHotkeys(): void {
  customKeys.value = {}
  saveCustomKeys()

  // 重新应用默认值
  for (const defaultHk of defaultHotkeys) {
    const binding = hotkeyRegistry.get(defaultHk.id)
    if (binding) {
      binding.keys = normalizeKeys(defaultHk.keys)
    }
  }
}

/**
 * 获取快捷键的有效按键（考虑自定义）
 */
function getEffectiveKeys(id: string): string {
  return customKeys.value[id] || defaultHotkeys.find((h) => h.id === id)?.keys || ''
}

// ============ 按键捕获（用于设置面板） ============

function startCapture(id: string): void {
  isCapturing.value = true
  capturedKeys.value = ''
  capturingId.value = id
}

function stopCapture(): void {
  isCapturing.value = false
  capturedKeys.value = ''
  capturingId.value = null
}

// ============ 全局事件处理 ============

function handleGlobalKeydown(e: KeyboardEvent): void {
  // 如果正在捕获按键
  if (isCapturing.value) {
    e.preventDefault()
    e.stopPropagation()

    const keys = eventToKeys(e)
    if (keys) {
      capturedKeys.value = keys
    }
    return
  }

  // 检查是否匹配任何已注册的快捷键
  for (const [, binding] of hotkeyRegistry) {
    if (!binding.enabled) continue

    // 上下文检查：全局快捷键始终响应，模块级快捷键只在对应上下文响应
    if (binding.category !== 'global' && binding.category !== activeContext.value) {
      continue
    }

    if (matchKeys(e, binding.keys)) {
      e.preventDefault()
      e.stopPropagation()
      binding.action()
      return
    }
  }
}

// ============ 初始化 ============

loadCustomKeys()

// 注册全局事件监听
if (typeof document !== 'undefined') {
  document.addEventListener('keydown', handleGlobalKeydown, true)
}

// ============ 导出 ============

export function useHotkeys() {
  return {
    // 状态
    activeContext: readonly(activeContext),
    isCapturing: readonly(isCapturing),
    capturedKeys: readonly(capturedKeys),
    capturingId: readonly(capturingId),
    customKeys: readonly(customKeys),

    // 注册
    registerHotkey,
    unregisterHotkey,
    getHotkeys,

    // 冲突检测
    isHotkeyConflict,
    getConflictInfo,

    // 上下文
    setActiveContext,

    // 自定义
    customizeHotkey,
    resetHotkey,
    resetAllHotkeys,
    getEffectiveKeys,

    // 捕获
    startCapture,
    stopCapture,

    // 工具
    normalizeKeys,
    eventToKeys,
    matchKeys,
  }
}
