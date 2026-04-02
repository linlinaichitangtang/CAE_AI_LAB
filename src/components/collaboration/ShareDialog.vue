<template>
  <div class="fixed inset-0 bg-black/60 flex items-center justify-center z-50 animate-fade-in" @click.self="$emit('close')">
    <div class="bg-[var(--bg-surface)] rounded-xl shadow-lg w-full max-w-lg animate-slide-in flex flex-col max-h-[80vh]">
      <!-- Header -->
      <div class="p-5 border-b border-[var(--border-subtle)] flex justify-between items-center flex-shrink-0">
        <div class="flex items-center gap-3">
          <div class="w-9 h-9 rounded-lg bg-[var(--primary-glow)] flex items-center justify-center">
            <span class="text-[var(--primary)] text-lg">👥</span>
          </div>
          <div>
            <h3 class="text-lg font-semibold text-[var(--text-primary)]">团队共享</h3>
            <p class="text-xs text-[var(--text-muted)]">{{ projectName }}</p>
          </div>
        </div>
        <button @click="$emit('close')" class="icon-btn">
          <span class="text-lg">✕</span>
        </button>
      </div>

      <!-- Add Member Form -->
      <div class="p-5 border-b border-[var(--border-subtle)] flex-shrink-0">
        <div class="flex gap-2">
          <input
            v-model="newMemberName"
            type="text"
            class="input flex-1"
            placeholder="输入成员名称..."
            @keyup.enter="handleAddMember"
          />
          <select
            v-model="newMemberPermission"
            class="input w-28"
          >
            <option value="read">只读</option>
            <option value="write">可编辑</option>
            <option value="admin">管理员</option>
          </select>
          <button
            @click="handleAddMember"
            :disabled="!newMemberName.trim() || adding"
            class="btn btn-primary flex items-center gap-1"
          >
            <span v-if="adding" class="animate-spin">⏳</span>
            <span v-else>+</span>
            <span>添加</span>
          </button>
        </div>
        <p v-if="error" class="text-xs text-[var(--accent-red)] mt-2">{{ error }}</p>
      </div>

      <!-- Shared Members List -->
      <div class="flex-1 overflow-auto p-5">
        <div v-if="shares.length === 0" class="text-center py-8">
          <div class="w-16 h-16 rounded-xl bg-[var(--bg-elevated)] flex items-center justify-center mb-4 mx-auto">
            <span class="text-3xl opacity-50">👥</span>
          </div>
          <p class="text-[var(--text-secondary)] mb-1">暂无共享成员</p>
          <p class="text-xs text-[var(--text-muted)]">添加团队成员以开始协作</p>
        </div>

        <div v-else class="space-y-3">
          <div class="flex items-center justify-between mb-2">
            <span class="text-xs font-medium text-[var(--text-secondary)] uppercase tracking-wider">
              已共享 ({{ shares.length }} 人)
            </span>
          </div>

          <div
            v-for="share in shares"
            :key="share.id"
            class="flex items-center justify-between p-3 rounded-lg bg-[var(--bg-elevated)] hover:bg-[var(--bg-hover)] transition-colors group"
          >
            <div class="flex items-center gap-3">
              <!-- Avatar -->
              <div class="w-9 h-9 rounded-full flex items-center justify-center text-sm font-semibold text-white flex-shrink-0"
                :style="{ backgroundColor: getAvatarColor(share.shared_with_name) }"
              >
                {{ share.shared_with_name.charAt(0).toUpperCase() }}
              </div>
              <div>
                <div class="text-sm font-medium text-[var(--text-primary)]">
                  {{ share.shared_with_name }}
                </div>
                <div class="text-xs text-[var(--text-muted)]">
                  {{ formatDate(share.created_at) }}
                </div>
              </div>
            </div>

            <div class="flex items-center gap-2">
              <!-- Permission Select -->
              <select
                :value="share.permission"
                @change="handleUpdatePermission(share.id, ($event.target as HTMLSelectElement).value as 'read' | 'write' | 'admin')"
                class="text-xs px-2 py-1 rounded border border-[var(--border-default)] bg-[var(--bg-surface)] text-[var(--text-primary)] cursor-pointer hover:border-[var(--primary)] transition-colors"
              >
                <option value="read">只读</option>
                <option value="write">可编辑</option>
                <option value="admin">管理员</option>
              </select>

              <!-- Remove Button -->
              <button
                @click="handleRemoveMember(share.id, share.shared_with_name)"
                class="w-7 h-7 flex items-center justify-center rounded text-[var(--text-muted)] hover:text-[var(--accent-red)] hover:bg-[var(--accent-red)]/10 transition-colors opacity-0 group-hover:opacity-100"
                title="移除成员"
              >
                <span class="text-sm">✕</span>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="p-4 border-t border-[var(--border-subtle)] flex justify-between items-center flex-shrink-0">
        <div class="flex items-center gap-4 text-xs text-[var(--text-muted)]">
          <span class="flex items-center gap-1">
            <span class="w-2 h-2 rounded-full bg-blue-400"></span>
            只读
          </span>
          <span class="flex items-center gap-1">
            <span class="w-2 h-2 rounded-full bg-green-400"></span>
            可编辑
          </span>
          <span class="flex items-center gap-1">
            <span class="w-2 h-2 rounded-full bg-purple-400"></span>
            管理员
          </span>
        </div>
        <button @click="$emit('close')" class="btn btn-ghost text-sm">
          关闭
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import {
  shareProject,
  listProjectShares,
  removeProjectShare,
  updateSharePermission,
  type ProjectShare,
} from '@/api/collaboration'

const props = defineProps<{
  projectId: string
  projectName: string
}>()

defineEmits<{
  close: []
  updated: []
}>()

const shares = ref<ProjectShare[]>([])
const newMemberName = ref('')
const newMemberPermission = ref<'read' | 'write' | 'admin'>('read')
const adding = ref(false)
const error = ref('')

const avatarColors = [
  '#3B82F6', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6',
  '#EC4899', '#06B6D4', '#F97316', '#14B8A6', '#6366F1',
]

function getAvatarColor(name: string): string {
  let hash = 0
  for (let i = 0; i < name.length; i++) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash)
  }
  return avatarColors[Math.abs(hash) % avatarColors.length]
}

function formatDate(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('zh-CN', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

async function loadShares() {
  try {
    shares.value = await listProjectShares(props.projectId)
  } catch (e) {
    console.error('Failed to load shares:', e)
  }
}

async function handleAddMember() {
  if (!newMemberName.value.trim()) return

  adding.value = true
  error.value = ''

  try {
    await shareProject(props.projectId, newMemberName.value.trim(), newMemberPermission.value)
    newMemberName.value = ''
    newMemberPermission.value = 'read'
    await loadShares()
  } catch (e: any) {
    error.value = String(e)
  } finally {
    adding.value = false
  }
}

async function handleRemoveMember(shareId: string, name: string) {
  if (!confirm(`确定要移除成员「${name}」吗？`)) return

  try {
    await removeProjectShare(shareId)
    await loadShares()
  } catch (e) {
    console.error('Failed to remove member:', e)
    alert('移除失败: ' + String(e))
  }
}

async function handleUpdatePermission(shareId: string, permission: 'read' | 'write' | 'admin') {
  try {
    await updateSharePermission(shareId, permission)
    await loadShares()
  } catch (e) {
    console.error('Failed to update permission:', e)
    alert('更新权限失败: ' + String(e))
  }
}

onMounted(() => {
  loadShares()
})
</script>

<style scoped>
@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.animate-spin {
  animation: spin 1s linear infinite;
}
</style>
