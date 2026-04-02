<template>
  <div class="profile-page">
    <div class="profile-layout">
      <!-- Sidebar -->
      <aside class="profile-sidebar">
        <div class="profile-avatar-section">
          <div class="avatar-large">
            <img v-if="authStore.user?.avatar_url" :src="authStore.user.avatar_url" alt="avatar" />
            <span v-else class="avatar-initial">{{ avatarInitial }}</span>
          </div>
          <button class="btn btn-ghost btn-upload-avatar" @click="handleUploadAvatar">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"/>
              <circle cx="12" cy="13" r="4"/>
            </svg>
            更换头像
          </button>
        </div>

        <nav class="profile-nav">
          <button
            v-for="section in sections"
            :key="section.id"
            class="profile-nav-item"
            :class="{ active: activeSection === section.id }"
            @click="activeSection = section.id"
          >
            <component :is="section.icon" class="w-4 h-4" />
            {{ section.label }}
          </button>
        </nav>

        <!-- Membership Card -->
        <div class="membership-mini-card" :class="membershipClass">
          <div class="membership-mini-header">
            <span class="membership-tier-badge">{{ tierLabel }}</span>
            <span v-if="authStore.membership?.expires_at" class="membership-expires">
              {{ formatDate(authStore.membership.expires_at) }} 到期
            </span>
          </div>
          <button class="btn btn-ghost btn-manage-membership" @click="router.push('/membership')">
            管理会员
            <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="9 18 15 12 9 6"/></svg>
          </button>
        </div>
      </aside>

      <!-- Main Content -->
      <main class="profile-main">
        <!-- Profile Info Section -->
        <section v-if="activeSection === 'info'" class="profile-section">
          <h2 class="section-title">个人资料</h2>
          <div class="form-grid">
            <div class="form-group">
              <label class="form-label">昵称</label>
              <input
                v-model="profileForm.nickname"
                type="text"
                class="input"
                placeholder="输入昵称"
              />
            </div>
            <div class="form-group">
              <label class="form-label">邮箱</label>
              <input
                :value="authStore.user?.email"
                type="email"
                class="input"
                disabled
              />
              <span class="form-hint">邮箱不可更改</span>
            </div>
            <div class="form-group">
              <label class="form-label">公司</label>
              <input
                v-model="profileForm.company"
                type="text"
                class="input"
                placeholder="输入公司名称"
              />
            </div>
            <div class="form-group">
              <label class="form-label">职位</label>
              <input
                v-model="profileForm.position"
                type="text"
                class="input"
                placeholder="输入职位"
              />
            </div>
          </div>
          <div class="form-actions">
            <button class="btn btn-primary" @click="handleSaveProfile">保存修改</button>
          </div>
          <div v-if="saveSuccess" class="form-success">保存成功</div>
          <div v-if="saveError" class="form-error">{{ saveError }}</div>
        </section>

        <!-- Security Section -->
        <section v-if="activeSection === 'security'" class="profile-section">
          <h2 class="section-title">安全设置</h2>
          <div class="security-card">
            <h3 class="card-title">修改密码</h3>
            <div class="form-group">
              <label class="form-label">当前密码</label>
              <input
                v-model="passwordForm.oldPassword"
                type="password"
                class="input"
                placeholder="输入当前密码"
              />
            </div>
            <div class="form-group">
              <label class="form-label">新密码</label>
              <input
                v-model="passwordForm.newPassword"
                type="password"
                class="input"
                placeholder="至少 8 位，含大小写和数字"
              />
            </div>
            <div class="form-group">
              <label class="form-label">确认新密码</label>
              <input
                v-model="passwordForm.confirmPassword"
                type="password"
                class="input"
                placeholder="再次输入新密码"
              />
            </div>
            <div class="form-actions">
              <button class="btn btn-primary" @click="handleChangePassword">修改密码</button>
            </div>
            <div v-if="passwordSuccess" class="form-success">密码修改成功</div>
            <div v-if="passwordError" class="form-error">{{ passwordError }}</div>
          </div>
        </section>

        <!-- Devices Section -->
        <section v-if="activeSection === 'devices'" class="profile-section">
          <h2 class="section-title">登录设备</h2>
          <div v-if="devices.length === 0" class="empty-state">
            <p>暂无设备信息</p>
          </div>
          <div v-else class="device-list">
            <div
              v-for="device in devices"
              :key="device.id"
              class="device-item"
            >
              <div class="device-info">
                <div class="device-icon">
                  <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
                    <line x1="8" y1="21" x2="16" y2="21"/>
                    <line x1="12" y1="17" x2="12" y2="21"/>
                  </svg>
                </div>
                <div>
                  <div class="device-name">
                    {{ device.device_name }}
                    <span v-if="device.is_current" class="current-badge">当前设备</span>
                  </div>
                  <div class="device-meta">
                    {{ device.device_type }} &middot; {{ device.ip_address }} &middot; {{ formatDate(device.last_active_at) }}
                  </div>
                </div>
              </div>
              <button
                v-if="!device.is_current"
                class="btn btn-ghost btn-sm"
                @click="handleLogoutDevice(device.id)"
              >
                远程登出
              </button>
            </div>
          </div>
        </section>

        <!-- Logout -->
        <div class="logout-section">
          <button class="btn btn-danger" @click="handleLogout">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>
              <polyline points="16 17 21 12 16 7"/>
              <line x1="21" y1="12" x2="9" y2="12"/>
            </svg>
            退出登录
          </button>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, h } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/authStore'
import * as authApi from '@/api/auth'
import type { DeviceInfo } from '@/api/auth'

const router = useRouter()
const authStore = useAuthStore()

const activeSection = ref('info')
const devices = ref<DeviceInfo[]>([])
const saveSuccess = ref(false)
const saveError = ref<string | null>(null)
const passwordSuccess = ref(false)
const passwordError = ref<string | null>(null)

// Profile form
const profileForm = reactive({
  nickname: authStore.user?.nickname || '',
  company: authStore.user?.company || '',
  position: authStore.user?.position || '',
})

// Password form
const passwordForm = reactive({
  oldPassword: '',
  newPassword: '',
  confirmPassword: '',
})

// Icons
const UserIcon = {
  render: () => h('svg', { class: 'w-4 h-4', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2' }),
    h('circle', { cx: '12', cy: '7', r: '4' })
  ])
}

const ShieldIcon = {
  render: () => h('svg', { class: 'w-4 h-4', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z' })
  ])
}

const MonitorIcon = {
  render: () => h('svg', { class: 'w-4 h-4', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('rect', { x: '2', y: '3', width: '20', height: '14', rx: '2', ry: '2' }),
    h('line', { x1: '8', y1: '21', x2: '16', y2: '21' }),
    h('line', { x1: '12', y1: '17', x2: '12', y2: '21' })
  ])
}

const sections = [
  { id: 'info', label: '个人资料', icon: UserIcon },
  { id: 'security', label: '安全设置', icon: ShieldIcon },
  { id: 'devices', label: '登录设备', icon: MonitorIcon },
]

const avatarInitial = computed(() => {
  const name = authStore.user?.nickname || authStore.user?.email || ''
  return name.charAt(0).toUpperCase()
})

const membershipClass = computed(() => {
  const tier = authStore.membership?.tier || 'free'
  return `tier-${tier}`
})

const tierLabel = computed(() => {
  const labels: Record<string, string> = {
    free: 'Free',
    pro: 'Pro',
    enterprise: 'Enterprise',
  }
  return labels[authStore.membership?.tier || 'free'] || 'Free'
})

function formatDate(dateStr: string) {
  if (!dateStr) return ''
  const date = new Date(dateStr)
  return date.toLocaleDateString('zh-CN', { year: 'numeric', month: 'long', day: 'numeric' })
}

function handleUploadAvatar() {
  // TODO: implement avatar upload with crop
  console.log('Upload avatar placeholder')
}

async function handleSaveProfile() {
  saveSuccess.value = false
  saveError.value = null
  if (!authStore.user?.id) return
  try {
    const updated = await authApi.updateProfile(authStore.user.id, {
      nickname: profileForm.nickname || null,
      company: profileForm.company || null,
      position: profileForm.position || null,
    })
    authStore.user = updated
    saveSuccess.value = true
    setTimeout(() => { saveSuccess.value = false }, 3000)
  } catch (e: any) {
    saveError.value = e
  }
}

async function handleChangePassword() {
  passwordSuccess.value = false
  passwordError.value = null
  if (passwordForm.newPassword !== passwordForm.confirmPassword) {
    passwordError.value = '两次输入的密码不一致'
    return
  }
  if (!authStore.user?.id) return
  try {
    await authApi.changePassword(authStore.user.id, passwordForm.oldPassword, passwordForm.newPassword)
    passwordSuccess.value = true
    passwordForm.oldPassword = ''
    passwordForm.newPassword = ''
    passwordForm.confirmPassword = ''
    setTimeout(() => { passwordSuccess.value = false }, 3000)
  } catch (e: any) {
    passwordError.value = e
  }
}

async function loadDevices() {
  if (!authStore.user?.id) return
  try {
    devices.value = await authApi.listDevices(authStore.user.id)
  } catch {
    // silently fail
  }
}

async function handleLogoutDevice(deviceId: string) {
  if (!authStore.user?.id) return
  try {
    await authApi.logoutDevice(authStore.user.id, deviceId)
    devices.value = devices.value.filter(d => d.id !== deviceId)
  } catch (e: any) {
    passwordError.value = e
  }
}

function handleLogout() {
  authStore.logout()
  router.push('/login')
}

onMounted(() => {
  if (authStore.user) {
    profileForm.nickname = authStore.user.nickname || ''
    profileForm.company = authStore.user.company || ''
    profileForm.position = authStore.user.position || ''
  }
  loadDevices()
})
</script>

<style scoped>
.profile-page {
  max-width: 900px;
  margin: 0 auto;
  padding: 32px 24px;
}

.profile-layout {
  display: grid;
  grid-template-columns: 240px 1fr;
  gap: 24px;
}

/* Sidebar */
.profile-sidebar {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.profile-avatar-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.avatar-large {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: linear-gradient(135deg, var(--primary, #2563EB), var(--primary-hover, #3B82F6));
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.avatar-large img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.avatar-initial {
  font-size: 32px;
  font-weight: 700;
  color: white;
}

.btn-upload-avatar {
  font-size: 12px;
  color: var(--primary, #2563EB);
  gap: 4px;
}

/* Nav */
.profile-nav {
  display: flex;
  flex-direction: column;
  gap: 2px;
  background: var(--bg-surface, #ffffff);
  border: 1px solid var(--border-default, #e5e7eb);
  border-radius: var(--radius-lg, 12px);
  padding: 4px;
}

.profile-nav-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  border: none;
  background: transparent;
  border-radius: var(--radius-md, 8px);
  font-size: 13px;
  color: var(--text-secondary, #4b5563);
  cursor: pointer;
  transition: all var(--duration-fast, 150ms);
  text-align: left;
  width: 100%;
}

.profile-nav-item:hover {
  background: var(--bg-elevated, #f3f4f6);
  color: var(--text-primary, #1f2937);
}

.profile-nav-item.active {
  background: var(--primary-glow, rgba(37, 99, 235, 0.08));
  color: var(--primary, #2563EB);
  font-weight: 500;
}

/* Membership Mini Card */
.membership-mini-card {
  padding: 16px;
  border-radius: var(--radius-lg, 12px);
  border: 1px solid var(--border-default, #e5e7eb);
  background: var(--bg-surface, #ffffff);
}

.membership-mini-card.tier-free {
  border-color: var(--border-default, #e5e7eb);
}

.membership-mini-card.tier-pro {
  border-color: var(--primary, #2563EB);
  background: linear-gradient(135deg, rgba(37, 99, 235, 0.04), rgba(59, 130, 246, 0.04));
}

.membership-mini-card.tier-enterprise {
  border-color: #8b5cf6;
  background: linear-gradient(135deg, rgba(139, 92, 246, 0.04), rgba(168, 85, 247, 0.04));
}

.membership-mini-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}

.membership-tier-badge {
  font-size: 14px;
  font-weight: 700;
  color: var(--text-primary, #1f2937);
}

.tier-pro .membership-tier-badge { color: var(--primary, #2563EB); }
.tier-enterprise .membership-tier-badge { color: #8b5cf6; }

.membership-expires {
  font-size: 11px;
  color: var(--text-muted, #9ca3af);
}

.btn-manage-membership {
  width: 100%;
  font-size: 12px;
  justify-content: center;
  gap: 4px;
  color: var(--text-secondary, #4b5563);
}

/* Main Content */
.profile-main {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.profile-section {
  background: var(--bg-surface, #ffffff);
  border: 1px solid var(--border-default, #e5e7eb);
  border-radius: var(--radius-lg, 12px);
  padding: 24px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #1f2937);
  margin-bottom: 20px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-subtle, #f3f4f6);
}

.card-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary, #1f2937);
  margin-bottom: 16px;
}

/* Form */
.form-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
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

.form-hint {
  font-size: 12px;
  color: var(--text-muted, #9ca3af);
}

.form-actions {
  margin-top: 20px;
  display: flex;
  gap: 8px;
}

.form-success {
  margin-top: 12px;
  padding: 8px 12px;
  background: #f0fdf4;
  border: 1px solid #bbf7d0;
  border-radius: var(--radius-sm, 4px);
  color: #16a34a;
  font-size: 13px;
}

.form-error {
  margin-top: 12px;
  padding: 8px 12px;
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: var(--radius-sm, 4px);
  color: #dc2626;
  font-size: 13px;
}

/* Security Card */
.security-card {
  max-width: 480px;
}

/* Device List */
.device-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.device-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: var(--bg-elevated, #f3f4f6);
  border-radius: var(--radius-md, 8px);
}

.device-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.device-icon {
  width: 36px;
  height: 36px;
  border-radius: var(--radius-md, 8px);
  background: var(--bg-surface, #ffffff);
  border: 1px solid var(--border-default, #e5e7eb);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary, #4b5563);
}

.device-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary, #1f2937);
  display: flex;
  align-items: center;
  gap: 8px;
}

.current-badge {
  font-size: 11px;
  padding: 1px 6px;
  background: #dcfce7;
  color: #16a34a;
  border-radius: 4px;
  font-weight: 500;
}

.device-meta {
  font-size: 12px;
  color: var(--text-muted, #9ca3af);
  margin-top: 2px;
}

.btn-sm {
  padding: 4px 12px;
  font-size: 12px;
  color: var(--accent-red, #ef4444);
}

.btn-sm:hover {
  background: #fef2f2;
}

/* Logout */
.logout-section {
  padding-top: 16px;
  border-top: 1px solid var(--border-subtle, #f3f4f6);
}

.btn-danger {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background: transparent;
  color: var(--accent-red, #ef4444);
  border: 1px solid var(--accent-red, #ef4444);
  border-radius: var(--radius-md, 8px);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--duration-fast, 150ms);
}

.btn-danger:hover {
  background: var(--accent-red, #ef4444);
  color: white;
}

/* Empty State */
.empty-state {
  text-align: center;
  padding: 32px;
  color: var(--text-muted, #9ca3af);
  font-size: 14px;
}

/* Responsive */
@media (max-width: 768px) {
  .profile-layout {
    grid-template-columns: 1fr;
  }

  .profile-sidebar {
    flex-direction: row;
    flex-wrap: wrap;
    align-items: center;
    justify-content: center;
  }

  .profile-avatar-section {
    flex-direction: row;
  }

  .profile-nav {
    flex-direction: row;
  }

  .form-grid {
    grid-template-columns: 1fr;
  }
}
</style>
