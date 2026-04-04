<template>
  <div class="auth-page">
    <div class="auth-card">
      <!-- Logo -->
      <div class="auth-logo">
        <div class="logo-icon">
          <svg class="w-8 h-8 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2L2 7l10 5 10-5-10-5z"/>
            <path d="M2 17l10 5 10-5"/>
            <path d="M2 12l10 5 10-5"/>
          </svg>
        </div>
        <h1 class="logo-text">CAELab</h1>
        <p class="logo-tagline">科研与工程创作一体化工作台</p>
      </div>

      <!-- Tabs -->
      <div class="auth-tabs">
        <button
          class="auth-tab"
          :class="{ active: activeTab === 'login' }"
          @click="activeTab = 'login'"
        >登录</button>
        <button
          class="auth-tab"
          :class="{ active: activeTab === 'register' }"
          @click="activeTab = 'register'"
        >注册</button>
      </div>

      <!-- Login Form -->
      <div v-if="activeTab === 'login'" class="auth-form">
        <div class="form-group">
          <label class="form-label">邮箱</label>
          <input
            v-model="loginForm.email"
            type="email"
            class="input"
            placeholder="your@email.com"
            @keyup.enter="handleLogin"
          />
        </div>

        <div class="form-group">
          <label class="form-label">密码</label>
          <div class="password-input">
            <input
              v-model="loginForm.password"
              :type="showLoginPassword ? 'text' : 'password'"
              class="input"
              placeholder="输入密码"
              @keyup.enter="handleLogin"
            />
            <button class="password-toggle" @click="showLoginPassword = !showLoginPassword" type="button">
              <svg v-if="showLoginPassword" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/>
                <line x1="1" y1="1" x2="23" y2="23"/>
              </svg>
              <svg v-else class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                <circle cx="12" cy="12" r="3"/>
              </svg>
            </button>
          </div>
        </div>

        <div class="form-row">
          <a href="#" class="link" @click.prevent="showForgotPassword = true">忘记密码？</a>
        </div>

        <button
          class="btn btn-primary btn-block"
          :disabled="authStore.isLoading || !loginForm.email || !loginForm.password"
          @click="handleLogin"
        >
          <span v-if="authStore.isLoading" class="btn-spinner"></span>
          <span v-else>登录</span>
        </button>

        <div v-if="authStore.error" class="form-error">{{ authStore.error }}</div>

        <div class="form-footer">
          没有账号？<a href="#" class="link" @click.prevent="activeTab = 'register'">立即注册</a>
        </div>

        <!-- Divider -->
        <div class="auth-divider">
          <span>或</span>
        </div>

        <!-- Social Login -->
        <div class="social-buttons">
          <button class="btn btn-secondary btn-social" @click="handleGoogleLogin">
            <svg class="w-4 h-4" viewBox="0 0 24 24">
              <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92a5.06 5.06 0 0 1-2.2 3.32v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.1z"/>
              <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
              <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
              <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
            </svg>
            Google 登录
          </button>
          <button class="btn btn-secondary btn-social" @click="handleGithubLogin">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
            </svg>
            GitHub 登录
          </button>
        </div>

        <!-- 免费试用入口 -->
        <div class="trial-entry">
          <div class="trial-divider"><span>或</span></div>
          <button
            class="btn btn-trial btn-block"
            @click="handleTrial"
          >
            <span class="trial-icon">⚡</span>
            免费试用 14 天
            <span class="trial-sub">全功能开放，无需注册</span>
          </button>
        </div>
      </div>

      <!-- Register Form -->
      <div v-else class="auth-form">
        <div class="form-group">
          <label class="form-label">邮箱</label>
          <input
            v-model="registerForm.email"
            type="email"
            class="input"
            placeholder="your@email.com"
          />
        </div>

        <div class="form-group">
          <label class="form-label">验证码</label>
          <div class="code-input">
            <input
              v-model="registerForm.code"
              type="text"
              class="input"
              placeholder="6 位验证码"
              maxlength="6"
            />
            <button
              class="btn btn-secondary btn-send-code"
              :disabled="codeCooldown > 0 || !registerForm.email"
              @click="handleSendCode"
            >
              {{ codeCooldown > 0 ? `${codeCooldown}s` : '发送验证码' }}
            </button>
          </div>
        </div>

        <div class="form-group">
          <label class="form-label">昵称 <span class="optional">（可选）</span></label>
          <input
            v-model="registerForm.nickname"
            type="text"
            class="input"
            placeholder="输入昵称"
          />
        </div>

        <div class="form-group">
          <label class="form-label">密码</label>
          <input
            v-model="registerForm.password"
            type="password"
            class="input"
            placeholder="至少 8 位，含大小写和数字"
            @input="checkPasswordStrength"
          />
          <!-- Password Strength Indicator -->
          <div class="password-strength">
            <div class="strength-bars">
              <div class="strength-bar" :class="{ active: passwordStrength >= 1, weak: passwordStrength === 1, medium: passwordStrength === 2, strong: passwordStrength >= 3 }"></div>
              <div class="strength-bar" :class="{ active: passwordStrength >= 2, weak: passwordStrength === 1, medium: passwordStrength === 2, strong: passwordStrength >= 3 }"></div>
              <div class="strength-bar" :class="{ active: passwordStrength >= 3, weak: passwordStrength === 1, medium: passwordStrength === 2, strong: passwordStrength >= 3 }"></div>
              <div class="strength-bar" :class="{ active: passwordStrength >= 4, weak: passwordStrength === 1, medium: passwordStrength === 2, strong: passwordStrength >= 3 }"></div>
            </div>
            <span class="strength-text" :class="strengthClass">{{ strengthLabel }}</span>
          </div>
        </div>

        <div class="form-group">
          <label class="form-label">确认密码</label>
          <input
            v-model="registerForm.confirmPassword"
            type="password"
            class="input"
            :class="{ 'input-error': registerForm.confirmPassword && registerForm.password !== registerForm.confirmPassword }"
            placeholder="再次输入密码"
            @keyup.enter="handleRegister"
          />
          <div v-if="registerForm.confirmPassword && registerForm.password !== registerForm.confirmPassword" class="form-hint error">
            两次输入的密码不一致
          </div>
        </div>

        <div class="form-group">
          <label class="checkbox-label">
            <input type="checkbox" v-model="registerForm.agreeTerms" />
            <span>我已阅读并同意 <a href="#" class="link" @click.prevent>服务条款</a> 和 <a href="#" class="link" @click.prevent>隐私政策</a></span>
          </label>
        </div>

        <button
          class="btn btn-primary btn-block"
          :disabled="authStore.isLoading || !canRegister"
          @click="handleRegister"
        >
          <span v-if="authStore.isLoading" class="btn-spinner"></span>
          <span v-else>注册</span>
        </button>

        <div v-if="authStore.error" class="form-error">{{ authStore.error }}</div>

        <div class="form-footer">
          已有账号？<a href="#" class="link" @click.prevent="activeTab = 'login'">立即登录</a>
        </div>
      </div>
    </div>

    <!-- Forgot Password Modal -->
    <Teleport to="body">
      <Transition name="modal">
        <div v-if="showForgotPassword" class="modal-overlay" @click.self="showForgotPassword = false">
          <div class="modal-card">
            <div class="modal-header">
              <h3>重置密码</h3>
              <button class="modal-close" @click="showForgotPassword = false">
                <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
                </svg>
              </button>
            </div>
            <div class="modal-body">
              <div class="form-group">
                <label class="form-label">邮箱</label>
                <input
                  v-model="forgotForm.email"
                  type="email"
                  class="input"
                  placeholder="your@email.com"
                />
              </div>

              <div class="form-group">
                <label class="form-label">验证码</label>
                <div class="code-input">
                  <input
                    v-model="forgotForm.code"
                    type="text"
                    class="input"
                    placeholder="6 位验证码"
                    maxlength="6"
                  />
                  <button
                    class="btn btn-secondary btn-send-code"
                    :disabled="forgotCodeCooldown > 0 || !forgotForm.email"
                    @click="handleForgotSendCode"
                  >
                    {{ forgotCodeCooldown > 0 ? `${forgotCodeCooldown}s` : '发送验证码' }}
                  </button>
                </div>
              </div>

              <div class="form-group">
                <label class="form-label">新密码</label>
                <input
                  v-model="forgotForm.newPassword"
                  type="password"
                  class="input"
                  placeholder="至少 8 位，含大小写和数字"
                />
              </div>

              <div class="form-group">
                <label class="form-label">确认新密码</label>
                <input
                  v-model="forgotForm.confirmPassword"
                  type="password"
                  class="input"
                  placeholder="再次输入新密码"
                />
              </div>

              <div v-if="forgotError" class="form-error">{{ forgotError }}</div>
              <div v-if="forgotSuccess" class="form-success">密码重置成功，请使用新密码登录</div>
            </div>
            <div class="modal-footer">
              <button class="btn btn-secondary" @click="showForgotPassword = false">取消</button>
              <button
                class="btn btn-primary"
                :disabled="!forgotForm.email || !forgotForm.code || !forgotForm.newPassword || !forgotForm.confirmPassword"
                @click="handleResetPassword"
              >重置密码</button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/authStore'
import { useTrialStore } from '@/stores/trial'
import * as authApi from '@/api/auth'

const router = useRouter()
const authStore = useAuthStore()
const trialStore = useTrialStore()

const activeTab = ref<'login' | 'register'>('login')
const showLoginPassword = ref(false)
const showForgotPassword = ref(false)
const codeCooldown = ref(0)
const forgotCodeCooldown = ref(0)
const passwordStrength = ref(0)
const forgotError = ref<string | null>(null)
const forgotSuccess = ref(false)

// Login form
const loginForm = reactive({
  email: '',
  password: '',
})

// Register form
const registerForm = reactive({
  email: '',
  code: '',
  nickname: '',
  password: '',
  confirmPassword: '',
  agreeTerms: false,
})

// Forgot password form
const forgotForm = reactive({
  email: '',
  code: '',
  newPassword: '',
  confirmPassword: '',
})

const strengthLabel = computed(() => {
  const labels = ['', '弱', '一般', '强', '非常强']
  return labels[passwordStrength.value] || ''
})

const strengthClass = computed(() => {
  const classes = ['', 'weak', 'medium', 'strong', 'strong']
  return classes[passwordStrength.value] || ''
})

const canRegister = computed(() => {
  return (
    registerForm.email &&
    registerForm.code &&
    registerForm.password &&
    registerForm.confirmPassword &&
    registerForm.password === registerForm.confirmPassword &&
    registerForm.agreeTerms &&
    passwordStrength.value >= 2
  )
})

function checkPasswordStrength() {
  const pwd = registerForm.password
  let strength = 0
  if (pwd.length >= 8) strength++
  if (/[a-z]/.test(pwd) && /[A-Z]/.test(pwd)) strength++
  if (/\d/.test(pwd)) strength++
  if (/[^a-zA-Z0-9]/.test(pwd)) strength++
  passwordStrength.value = strength
}

async function handleLogin() {
  try {
    await authStore.login(loginForm.email, loginForm.password)
    router.push('/')
  } catch {
    // error is set in store
  }
}

async function handleSendCode() {
  try {
    await authApi.sendVerificationCode(registerForm.email, 'register')
    codeCooldown.value = 60
    const timer = setInterval(() => {
      codeCooldown.value--
      if (codeCooldown.value <= 0) clearInterval(timer)
    }, 1000)
  } catch (e: any) {
    authStore.error = e
  }
}

async function handleRegister() {
  if (!canRegister.value) return
  try {
    const verified = await authApi.verifyCode(registerForm.email, registerForm.code, 'register')
    if (!verified) {
      authStore.error = '验证码错误'
      return
    }
    await authStore.register(registerForm.email, registerForm.password, registerForm.nickname || undefined)
    router.push('/onboarding')
  } catch {
    // error is set in store
  }
}

function handleGoogleLogin() {
  // Google OAuth flow: redirect to Google's consent screen
  // In production, this would use the actual Google Client ID from .env
  const clientId = import.meta.env.VITE_GOOGLE_CLIENT_ID || ''
  if (!clientId) {
    authStore.error = 'Google 登录暂未配置，请联系管理员'
    return
  }
  const redirectUri = encodeURIComponent(window.location.origin + '/auth/google/callback')
  const scope = encodeURIComponent('openid email profile')
  window.location.href = `https://accounts.google.com/o/oauth2/v2/auth?client_id=${clientId}&redirect_uri=${redirectUri}&response_type=code&scope=${scope}`
}

function handleGithubLogin() {
  // GitHub OAuth flow: redirect to GitHub's authorization page
  const clientId = import.meta.env.VITE_GITHUB_CLIENT_ID || ''
  if (!clientId) {
    authStore.error = 'GitHub 登录暂未配置，请联系管理员'
    return
  }
  const redirectUri = encodeURIComponent(window.location.origin + '/auth/github/callback')
  const scope = encodeURIComponent('read:user user:email')
  window.location.href = `https://github.com/login/oauth/authorize?client_id=${clientId}&redirect_uri=${redirectUri}&scope=${scope}`
}

/**
 * 免费试用：创建本地临时账号 + 启动14天试用
 * 1. 创建 guest 账号（仅本地存储，无需邮箱验证）
 * 2. 启动14天全功能试用
 * 3. 跳转求解器安装引导页
 */
async function handleTrial() {
  authStore.isLoading = true
  authStore.error = null
  try {
    // 创建本地 guest 账号（UUID 形式，无需真实邮箱）
    const guestId = 'guest_' + Math.random().toString(36).substring(2, 15)
    const guestEmail = `guest_${guestId}@trial.caelab.local`
    const guestNickname = '试用用户'

    // 设置试用状态
    trialStore.startTrial()

    // 写入本地认证数据（模拟登录状态）
    localStorage.setItem('caelab-access-token', `trial_${guestId}`)
    localStorage.setItem('caelab-refresh-token', `trial_refresh_${guestId}`)
    localStorage.setItem('caelab-user', JSON.stringify({
      id: guestId,
      email: guestEmail,
      nickname: guestNickname,
      avatar_url: null,
      company: null,
      position: null,
      created_at: new Date().toISOString(),
    }))
    const trialEnd = trialStore.trialEndMs ?? null
    localStorage.setItem('caelab-membership', JSON.stringify({
      tier: 'pro',
      expires_at: trialEnd ? new Date(trialEnd).toISOString() : null,
      is_active: true,
    }))

    // 初始化 store 状态
    authStore.accessToken = `trial_${guestId}`
    authStore.refreshToken = `trial_refresh_${guestId}`
    authStore.user = {
      id: guestId,
      email: guestEmail,
      nickname: guestNickname,
      avatar_url: null,
      company: null,
      position: null,
      created_at: new Date().toISOString(),
    }
    authStore.membership = {
      tier: 'pro',
      expires_at: trialEnd ? new Date(trialEnd).toISOString() : null,
      is_active: true,
    }

    // 跳转到求解器安装引导页
    router.push('/solver-installer')
  } catch (e: any) {
    authStore.error = '试用启动失败，请重试'
  } finally {
    authStore.isLoading = false
  }
}

async function handleForgotSendCode() {
  try {
    await authApi.sendVerificationCode(forgotForm.email, 'reset_password')
    forgotCodeCooldown.value = 60
    const timer = setInterval(() => {
      forgotCodeCooldown.value--
      if (forgotCodeCooldown.value <= 0) clearInterval(timer)
    }, 1000)
  } catch (e: any) {
    forgotError.value = e
  }
}

async function handleResetPassword() {
  forgotError.value = null
  forgotSuccess.value = false
  if (forgotForm.newPassword !== forgotForm.confirmPassword) {
    forgotError.value = '两次输入的密码不一致'
    return
  }
  try {
    const verified = await authApi.verifyCode(forgotForm.email, forgotForm.code, 'reset_password')
    if (!verified) {
      forgotError.value = '验证码错误'
      return
    }
    // TODO: call reset password API when available
    forgotSuccess.value = true
    setTimeout(() => {
      showForgotPassword.value = false
      activeTab.value = 'login'
    }, 2000)
  } catch (e: any) {
    forgotError.value = e
  }
}

onMounted(() => {
  authStore.error = null
})
</script>

<style scoped>
.auth-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
  padding: 24px;
}

.auth-card {
  width: 100%;
  max-width: 420px;
  background: var(--bg-surface, #ffffff);
  border-radius: var(--radius-xl, 16px);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  padding: 32px;
}

/* Logo */
.auth-logo {
  text-align: center;
  margin-bottom: 28px;
}

.logo-icon {
  width: 48px;
  height: 48px;
  border-radius: var(--radius-md, 8px);
  background: linear-gradient(135deg, var(--primary, #2563EB), var(--primary-hover, #3B82F6));
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 12px;
  box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
}

.logo-text {
  font-size: 24px;
  font-weight: 800;
  color: var(--text-primary, #1f2937);
  letter-spacing: -1px;
}

.logo-tagline {
  font-size: 13px;
  color: var(--text-muted, #9ca3af);
  margin-top: 4px;
}

/* Tabs */
.auth-tabs {
  display: flex;
  background: var(--bg-elevated, #f3f4f6);
  border-radius: var(--radius-md, 8px);
  padding: 3px;
  margin-bottom: 24px;
}

.auth-tab {
  flex: 1;
  padding: 8px 16px;
  border: none;
  background: transparent;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary, #4b5563);
  cursor: pointer;
  transition: all var(--duration-fast, 150ms) var(--ease-out, cubic-bezier(0.16, 1, 0.3, 1));
}

.auth-tab.active {
  background: var(--bg-surface, #ffffff);
  color: var(--primary, #2563EB);
  box-shadow: var(--shadow-sm, 0 1px 3px rgba(0, 0, 0, 0.06));
}

/* Form */
.auth-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary, #4b5563);
}

.form-label .optional {
  font-weight: 400;
  color: var(--text-muted, #9ca3af);
}

.form-row {
  display: flex;
  justify-content: flex-end;
}

.form-error {
  padding: 8px 12px;
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: var(--radius-sm, 4px);
  color: #dc2626;
  font-size: 13px;
}

.form-success {
  padding: 8px 12px;
  background: #f0fdf4;
  border: 1px solid #bbf7d0;
  border-radius: var(--radius-sm, 4px);
  color: #16a34a;
  font-size: 13px;
}

.form-hint {
  font-size: 12px;
  color: var(--text-muted, #9ca3af);
}

.form-hint.error {
  color: #dc2626;
}

.form-footer {
  text-align: center;
  font-size: 13px;
  color: var(--text-muted, #9ca3af);
}

.link {
  color: var(--primary, #2563EB);
  text-decoration: none;
  font-weight: 500;
  transition: color var(--duration-fast, 150ms);
}

.link:hover {
  color: var(--primary-hover, #3B82F6);
  text-decoration: underline;
}

/* Password Input */
.password-input {
  position: relative;
}

.password-input .input {
  padding-right: 40px;
}

.password-toggle {
  position: absolute;
  right: 8px;
  top: 50%;
  transform: translateY(-50%);
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: var(--text-muted, #9ca3af);
  cursor: pointer;
  border-radius: var(--radius-sm, 4px);
}

.password-toggle:hover {
  color: var(--text-secondary, #4b5563);
  background: var(--bg-elevated, #f3f4f6);
}

/* Code Input */
.code-input {
  display: flex;
  gap: 8px;
}

.code-input .input {
  flex: 1;
}

.btn-send-code {
  white-space: nowrap;
  min-width: 110px;
  font-size: 12px;
  padding: 8px 12px;
}

/* Password Strength */
.password-strength {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 4px;
}

.strength-bars {
  display: flex;
  gap: 4px;
  flex: 1;
}

.strength-bar {
  height: 3px;
  flex: 1;
  border-radius: 2px;
  background: var(--bg-hover, #e5e7eb);
  transition: background var(--duration-fast, 150ms);
}

.strength-bar.active.weak { background: #ef4444; }
.strength-bar.active.medium { background: #f59e0b; }
.strength-bar.active.strong { background: #22c55e; }

.strength-text {
  font-size: 11px;
  font-weight: 500;
  min-width: 48px;
  text-align: right;
}

.strength-text.weak { color: #ef4444; }
.strength-text.medium { color: #f59e0b; }
.strength-text.strong { color: #22c55e; }

/* Checkbox */
.checkbox-label {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  font-size: 13px;
  color: var(--text-secondary, #4b5563);
  cursor: pointer;
}

.checkbox-label input[type="checkbox"] {
  width: 16px;
  height: 16px;
  margin-top: 1px;
  accent-color: var(--primary, #2563EB);
  cursor: pointer;
}

/* Buttons */
.btn-block {
  width: 100%;
  padding: 10px 16px;
  font-size: 14px;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top-color: white;
  border-radius: 50%;
  animation: spin 0.6s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

/* Divider */
.auth-divider {
  display: flex;
  align-items: center;
  gap: 16px;
  margin: 4px 0;
}

.auth-divider::before,
.auth-divider::after {
  content: '';
  flex: 1;
  height: 1px;
  background: var(--border-default, #e5e7eb);
}

.auth-divider span {
  font-size: 12px;
  color: var(--text-muted, #9ca3af);
}

/* Social Buttons */
.social-buttons {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.btn-social {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 10px 16px;
}

/* Input Error */
.input-error {
  border-color: #ef4444 !important;
}

.input-error:focus {
  box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.08) !important;
}

/* Modal */
.modal-overlay {
  position: fixed;
  inset: 0;
  z-index: 1000;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.modal-card {
  width: 100%;
  max-width: 400px;
  background: var(--bg-surface, #ffffff);
  border-radius: var(--radius-xl, 16px);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.2);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px 0;
}

.modal-header h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #1f2937);
}

.modal-close {
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

.modal-close:hover {
  background: var(--bg-elevated, #f3f4f6);
  color: var(--text-primary, #1f2937);
}

.modal-body {
  padding: 20px 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 0 24px 20px;
}

/* Modal Transition */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-active .modal-card,
.modal-leave-active .modal-card {
  transition: transform 0.2s var(--ease-out, cubic-bezier(0.16, 1, 0.3, 1));
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .modal-card,
.modal-leave-to .modal-card {
  transform: scale(0.95) translateY(10px);
}

/* Trial Button */
.trial-entry {
  margin-top: 12px;
}

.trial-divider {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
  color: var(--text-muted, #9ca3af);
  font-size: 12px;
}
.trial-divider::before,
.trial-divider::after {
  content: '';
  flex: 1;
  height: 1px;
  background: var(--border-color, #e5e7eb);
}

.btn-trial {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: var(--radius-md, 10px);
  padding: 14px 20px;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  transition: all 0.2s ease;
  box-shadow: 0 4px 14px rgba(102, 126, 234, 0.35);
}
.btn-trial:hover {
  transform: translateY(-1px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.45);
}
.btn-trial:active {
  transform: translateY(0);
}
.trial-icon {
  font-size: 18px;
}
.trial-sub {
  font-size: 11px;
  font-weight: 400;
  opacity: 0.85;
}

/* Responsive */
@media (max-width: 480px) {
  .auth-card {
    padding: 24px 20px;
    border-radius: var(--radius-lg, 12px);
  }
}
</style>
