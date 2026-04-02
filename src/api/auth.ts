import { invoke } from '@tauri-apps/api/core'

export interface UserProfile {
  id: string
  email: string
  nickname: string | null
  avatar_url: string | null
  company: string | null
  position: string | null
  created_at: string
}

export interface MembershipStatus {
  tier: 'free' | 'pro' | 'enterprise'
  expires_at: string | null
  is_active: boolean
}

export interface AuthResponse {
  access_token: string
  refresh_token: string
  user: UserProfile
  membership: MembershipStatus
}

export interface DeviceInfo {
  id: string
  device_name: string
  device_type: string
  ip_address: string
  last_active_at: string
  is_current: boolean
}

export async function register(email: string, password: string, nickname?: string): Promise<AuthResponse> {
  return invoke('register', { email, password, nickname: nickname || null })
}

export async function login(email: string, password: string): Promise<AuthResponse> {
  return invoke('login', { email, password })
}

export async function refreshToken(refreshToken: string): Promise<AuthResponse> {
  return invoke('refresh_token', { refreshToken })
}

export async function getProfile(userId: string): Promise<UserProfile> {
  return invoke('get_profile', { userId })
}

export async function updateProfile(userId: string, data: Partial<UserProfile>): Promise<UserProfile> {
  return invoke('update_profile', { userId, ...data })
}

export async function changePassword(userId: string, oldPassword: string, newPassword: string): Promise<void> {
  return invoke('change_password', { userId, oldPassword, newPassword })
}

export async function sendVerificationCode(email: string, purpose: string): Promise<void> {
  return invoke('send_verification_code', { email, purpose })
}

export async function verifyCode(email: string, code: string, purpose: string): Promise<boolean> {
  return invoke('verify_code', { email, code, purpose })
}

export async function listDevices(userId: string): Promise<DeviceInfo[]> {
  return invoke('list_devices', { userId })
}

export async function logoutDevice(userId: string, deviceId: string): Promise<void> {
  return invoke('logout_device', { userId, deviceId })
}

export async function getMembershipStatus(userId: string): Promise<MembershipStatus> {
  return invoke('get_membership_status', { userId })
}

export async function updateMembership(userId: string, tier: string, expiresAt: string): Promise<MembershipStatus> {
  return invoke('update_membership', { userId, tier, expiresAt })
}
