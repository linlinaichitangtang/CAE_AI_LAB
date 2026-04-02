import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import * as authApi from '@/api/auth'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<authApi.UserProfile | null>(null)
  const membership = ref<authApi.MembershipStatus | null>(null)
  const accessToken = ref<string | null>(null)
  const refreshTokenValue = ref<string | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const isFirstLogin = ref(!localStorage.getItem('caelab-onboarding-done'))

  const isAuthenticated = computed(() => !!accessToken.value && !!user.value)
  const isPro = computed(() => membership.value?.tier === 'pro' || membership.value?.tier === 'enterprise')
  const isEnterprise = computed(() => membership.value?.tier === 'enterprise')

  function init() {
    // 从 localStorage 恢复登录状态
    accessToken.value = localStorage.getItem('caelab-access-token')
    refreshTokenValue.value = localStorage.getItem('caelab-refresh-token')
    const savedUser = localStorage.getItem('caelab-user')
    if (savedUser) user.value = JSON.parse(savedUser)
    const savedMembership = localStorage.getItem('caelab-membership')
    if (savedMembership) membership.value = JSON.parse(savedMembership)
    isFirstLogin.value = !localStorage.getItem('caelab-onboarding-done')
  }

  async function register(email: string, password: string, nickname?: string) {
    isLoading.value = true
    error.value = null
    try {
      const res = await authApi.register(email, password, nickname)
      setAuthData(res)
      isFirstLogin.value = true
      localStorage.removeItem('caelab-onboarding-done')
      return res
    } catch (e: any) {
      error.value = e
      throw e
    } finally {
      isLoading.value = false
    }
  }

  async function login(email: string, password: string) {
    isLoading.value = true
    error.value = null
    try {
      const res = await authApi.login(email, password)
      setAuthData(res)
      return res
    } catch (e: any) {
      error.value = e
      throw e
    } finally {
      isLoading.value = false
    }
  }

  function completeOnboarding() {
    isFirstLogin.value = false
    localStorage.setItem('caelab-onboarding-done', 'true')
  }

  function logout() {
    user.value = null
    membership.value = null
    accessToken.value = null
    refreshTokenValue.value = null
    localStorage.removeItem('caelab-access-token')
    localStorage.removeItem('caelab-refresh-token')
    localStorage.removeItem('caelab-user')
    localStorage.removeItem('caelab-membership')
  }

  function setAuthData(res: authApi.AuthResponse) {
    accessToken.value = res.access_token
    refreshTokenValue.value = res.refresh_token
    user.value = res.user
    membership.value = res.membership
    localStorage.setItem('caelab-access-token', res.access_token)
    localStorage.setItem('caelab-refresh-token', res.refresh_token)
    localStorage.setItem('caelab-user', JSON.stringify(res.user))
    localStorage.setItem('caelab-membership', JSON.stringify(res.membership))
  }

  return {
    user, membership, accessToken, refreshToken: refreshTokenValue,
    isLoading, error, isFirstLogin,
    isAuthenticated, isPro, isEnterprise,
    init, register, login, logout, completeOnboarding,
  }
})
