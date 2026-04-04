/**
 * 试用管理 Store
 * 管理14天全功能试用状态
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

const TRIAL_DAYS = 14
const TRIAL_STORAGE_KEY = 'caelab-trial-start'
const TRIAL_DONE_KEY = 'caelab-trial-done'

export const useTrialStore = defineStore('trial', () => {
  // ============================================
  // State
  // ============================================

  /** 试用开始时间戳（毫秒） */
  const trialStartMs = ref<number | null>(loadStartTime())

  /** 试用结束时间戳（毫秒） */
  const trialEndMs = computed(() => {
    if (!trialStartMs.value) return null
    return trialStartMs.value + TRIAL_DAYS * 24 * 60 * 60 * 1000
  })

  /** 是否正在试用 */
  const isInTrial = computed(() => {
    if (!trialStartMs.value || !trialEndMs.value) return false
    const now = Date.now()
    return now >= trialStartMs.value && now < trialEndMs.value
  })

  /** 是否已完成试用（已弹窗引导过） */
  const trialDone = ref(loadTrialDone())

  /** 试用剩余天数（向上取整） */
  const daysRemaining = computed(() => {
    if (!trialEndMs.value) return 0
    const remaining = trialEndMs.value - Date.now()
    if (remaining <= 0) return 0
    return Math.ceil(remaining / (24 * 60 * 60 * 1000))
  })

  /** 是否已过期 */
  const isExpired = computed(() => {
    if (!trialEndMs.value) return false
    return Date.now() >= trialEndMs.value
  })

  /** 是否全功能模式（试用中或永久Pro/Ent） */
  const isFullAccess = computed(() => {
    return isInTrial.value
  })

  /** 是否显示过期 Banner（试用已结束且未引导过） */
  const showExpiredBanner = computed(() => {
    return isExpired.value && !trialDone.value
  })

  /** 试用开始时间 */
  const startDate = computed(() => {
    if (!trialStartMs.value) return null
    return new Date(trialStartMs.value).toLocaleDateString('zh-CN')
  })

  // ============================================
  // Actions
  // ============================================

  /**
   * 开始14天试用
   * 调用时机：用户点击"免费试用14天"
   */
  function startTrial() {
    const now = Date.now()
    trialStartMs.value = now
    trialDone.value = false
    localStorage.setItem(TRIAL_STORAGE_KEY, now.toString())
    localStorage.removeItem(TRIAL_DONE_KEY)
  }

  /**
   * 试用结束，标记为已完成
   * 调用时机：试用期到，或用户主动关闭试用引导
   */
  function markTrialDone() {
    trialDone.value = true
    localStorage.setItem(TRIAL_DONE_KEY, 'true')
  }

  /**
   * 关闭试用到期 Banner
   */
  function dismissExpiredBanner() {
    markTrialDone()
  }

  /**
   * 关闭试用中 Banner（本次不提醒）
   */
  function markBannerDismissed() {
    // 仅标记为不再显示，试用仍然有效
  }

  /**
   * 重置试用（用于测试）
   */
  function resetTrial() {
    trialStartMs.value = null
    trialDone.value = false
    localStorage.removeItem(TRIAL_STORAGE_KEY)
    localStorage.removeItem(TRIAL_DONE_KEY)
  }

  return {
    trialStartMs,
    trialEndMs,
    isInTrial,
    isExpired,
    isFullAccess,
    daysRemaining,
    trialDone,
    showExpiredBanner,
    startDate,
    startTrial,
    markTrialDone,
    dismissExpiredBanner,
    markBannerDismissed,
    resetTrial,
  }
})

// ============================================
// Helpers
// ============================================

function loadStartTime(): number | null {
  const saved = localStorage.getItem(TRIAL_STORAGE_KEY)
  return saved ? parseInt(saved, 10) : null
}

function loadTrialDone(): boolean {
  return localStorage.getItem(TRIAL_DONE_KEY) === 'true'
}
