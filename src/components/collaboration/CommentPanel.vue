<template>
  <div class="comment-panel h-full flex flex-col bg-[var(--bg-surface)]">
    <!-- Header -->
    <div class="p-4 border-b border-[var(--border-subtle)] flex items-center justify-between flex-shrink-0">
      <div class="flex items-center gap-2">
        <span class="text-lg">💬</span>
        <h3 class="text-sm font-semibold text-[var(--text-primary)]">评论标注</h3>
        <span v-if="filteredComments.length > 0" class="px-2 py-0.5 text-xs rounded-full bg-[var(--primary-glow)] text-[var(--primary)]">
          {{ filteredComments.length }}
        </span>
      </div>
      <div class="flex items-center gap-1">
        <!-- Filter -->
        <select
          v-model="filterMode"
          class="text-xs px-2 py-1 rounded border border-[var(--border-default)] bg-[var(--bg-elevated)] text-[var(--text-secondary)] cursor-pointer"
        >
          <option value="all">全部评论</option>
          <option value="current">当前笔记</option>
          <option value="unresolved">未解决</option>
          <option value="resolved">已解决</option>
        </select>
        <button @click="$emit('close')" class="icon-btn w-7 h-7">
          <span class="text-sm">✕</span>
        </button>
      </div>
    </div>

    <!-- Add Comment -->
    <div class="p-4 border-b border-[var(--border-subtle)] flex-shrink-0">
      <div class="flex gap-2 mb-2">
        <input
          v-model="authorName"
          type="text"
          class="input text-sm flex-shrink-0"
          style="width: 100px"
          placeholder="你的名称"
        />
      </div>
      <div class="relative">
        <textarea
          v-model="newCommentContent"
          class="input w-full resize-none text-sm"
          rows="3"
          placeholder="添加评论... 使用 @ 提及成员"
          @input="handleMentionInput"
          @keydown="handleKeyDown"
        ></textarea>
        <!-- Mention Dropdown -->
        <div
          v-if="showMentionDropdown && mentionSuggestions.length > 0"
          class="absolute left-0 bottom-full mb-1 bg-[var(--bg-surface)] rounded-lg shadow-lg border border-[var(--border-default)] py-1 z-10 min-w-[160px]"
        >
          <button
            v-for="member in mentionSuggestions"
            :key="member"
            @click="insertMention(member)"
            class="w-full px-3 py-1.5 text-left text-sm text-[var(--text-primary)] hover:bg-[var(--bg-hover)] flex items-center gap-2"
          >
            <span class="w-5 h-5 rounded-full flex items-center justify-center text-[10px] font-semibold text-white flex-shrink-0"
              :style="{ backgroundColor: getAvatarColor(member) }"
            >
              {{ member.charAt(0).toUpperCase() }}
            </span>
            {{ member }}
          </button>
        </div>
      </div>
      <div class="flex justify-end mt-2">
        <button
          @click="handleAddComment"
          :disabled="!newCommentContent.trim() || !authorName.trim() || submitting"
          class="btn btn-primary text-sm flex items-center gap-1"
        >
          <span v-if="submitting" class="animate-spin">⏳</span>
          <span v-else>💬</span>
          <span>发表</span>
        </button>
      </div>
    </div>

    <!-- Comments List -->
    <div class="flex-1 overflow-auto p-4">
      <div v-if="filteredComments.length === 0" class="text-center py-8">
        <div class="w-14 h-14 rounded-xl bg-[var(--bg-elevated)] flex items-center justify-center mb-3 mx-auto">
          <span class="text-2xl opacity-50">💬</span>
        </div>
        <p class="text-sm text-[var(--text-secondary)]">暂无评论</p>
        <p class="text-xs text-[var(--text-muted)] mt-1">在上方输入框添加第一条评论</p>
      </div>

      <div v-else class="space-y-3">
        <div
          v-for="comment in filteredComments"
          :key="comment.id"
          class="p-3 rounded-lg transition-colors"
          :class="comment.resolved ? 'bg-[var(--bg-elevated)] opacity-60' : 'bg-[var(--bg-elevated)]'"
        >
          <div class="flex items-start gap-3">
            <!-- Avatar -->
            <div class="w-8 h-8 rounded-full flex items-center justify-center text-xs font-semibold text-white flex-shrink-0 mt-0.5"
              :style="{ backgroundColor: getAvatarColor(comment.author_name) }"
            >
              {{ comment.author_name.charAt(0).toUpperCase() }}
            </div>

            <div class="flex-1 min-w-0">
              <!-- Author & Time -->
              <div class="flex items-center gap-2 mb-1">
                <span class="text-sm font-medium text-[var(--text-primary)]">
                  {{ comment.author_name }}
                </span>
                <span class="text-xs text-[var(--text-muted)]">
                  {{ formatDate(comment.created_at) }}
                </span>
                <span v-if="comment.note_id" class="text-[10px] px-1.5 py-0.5 rounded bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400">
                  笔记
                </span>
                <span
                  v-if="comment.resolved"
                  class="text-[10px] px-1.5 py-0.5 rounded bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400"
                >
                  已解决
                </span>
              </div>

              <!-- Content -->
              <div class="text-sm text-[var(--text-secondary)] whitespace-pre-wrap break-words">
                <template v-for="(segment, idx) in parseMentions(comment.content)" :key="idx">
                  <span v-if="segment.type === 'mention'" class="text-[var(--primary)] font-medium cursor-pointer hover:underline">
                    {{ segment.text }}
                  </span>
                  <span v-else>{{ segment.text }}</span>
                </template>
              </div>

              <!-- Actions -->
              <div class="flex items-center gap-2 mt-2">
                <button
                  @click="handleToggleResolve(comment)"
                  class="text-xs px-2 py-1 rounded hover:bg-[var(--bg-hover)] transition-colors"
                  :class="comment.resolved ? 'text-[var(--text-muted)]' : 'text-green-600 dark:text-green-400'"
                >
                  {{ comment.resolved ? '标记未解决' : '标记已解决' }}
                </button>
                <button
                  @click="startEdit(comment)"
                  class="text-xs px-2 py-1 rounded text-[var(--text-muted)] hover:bg-[var(--bg-hover)] transition-colors"
                >
                  编辑
                </button>
                <button
                  @click="handleDelete(comment)"
                  class="text-xs px-2 py-1 rounded text-[var(--accent-red)] hover:bg-[var(--accent-red)]/10 transition-colors"
                >
                  删除
                </button>
              </div>

              <!-- Edit Mode -->
              <div v-if="editingCommentId === comment.id" class="mt-2">
                <textarea
                  v-model="editContent"
                  class="input w-full resize-none text-sm"
                  rows="2"
                ></textarea>
                <div class="flex justify-end gap-2 mt-2">
                  <button
                    @click="cancelEdit"
                    class="btn btn-ghost text-xs"
                  >
                    取消
                  </button>
                  <button
                    @click="handleUpdateComment(comment.id)"
                    :disabled="!editContent.trim()"
                    class="btn btn-primary text-xs"
                  >
                    保存
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import {
  addComment,
  listComments,
  updateComment,
  deleteComment,
  resolveComment,
  type Comment,
} from '@/api/collaboration'
import { listProjectShares, type ProjectShare } from '@/api/collaboration'

const props = defineProps<{
  projectId: string
  noteId?: string | null
}>()

defineEmits<{
  close: []
}>()

const comments = ref<Comment[]>([])
const shares = ref<ProjectShare[]>([])
const authorName = ref('')
const newCommentContent = ref('')
const filterMode = ref<'all' | 'current' | 'unresolved' | 'resolved'>('all')
const submitting = ref(false)
const editingCommentId = ref<string | null>(null)
const editContent = ref('')

// Mention state
const showMentionDropdown = ref(false)
const mentionSearch = ref('')
const mentionStartIndex = ref(-1)

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
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(diff / 3600000)

  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes} 分钟前`
  if (hours < 24) return `${hours} 小时前`

  return date.toLocaleDateString('zh-CN', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

// Filtered comments based on filter mode
const filteredComments = computed(() => {
  let result = comments.value

  switch (filterMode.value) {
    case 'current':
      result = result.filter(c => c.note_id === props.noteId)
      break
    case 'unresolved':
      result = result.filter(c => !c.resolved)
      break
    case 'resolved':
      result = result.filter(c => c.resolved)
      break
  }

  return result
})

// Mention suggestions
const mentionSuggestions = computed(() => {
  if (!mentionSearch.value) {
    return shares.value.map(s => s.shared_with_name)
  }
  return shares.value
    .map(s => s.shared_with_name)
    .filter(name => name.toLowerCase().includes(mentionSearch.value.toLowerCase()))
})

function parseMentions(content: string): Array<{ type: 'text' | 'mention'; text: string }> {
  const parts: Array<{ type: 'text' | 'mention'; text: string }> = []
  const regex = /@(\S+)/g
  let lastIndex = 0
  let match

  while ((match = regex.exec(content)) !== null) {
    if (match.index > lastIndex) {
      parts.push({ type: 'text', text: content.slice(lastIndex, match.index) })
    }
    parts.push({ type: 'mention', text: match[0] })
    lastIndex = regex.lastIndex
  }

  if (lastIndex < content.length) {
    parts.push({ type: 'text', text: content.slice(lastIndex) })
  }

  return parts
}

function handleMentionInput() {
  const value = newCommentContent.value
  const cursorPos = value.length

  // Find the last @ before cursor
  let atPos = -1
  for (let i = cursorPos - 1; i >= 0; i--) {
    if (value[i] === '@') {
      atPos = i
      break
    }
    if (value[i] === ' ' || value[i] === '\n') {
      break
    }
  }

  if (atPos >= 0) {
    mentionStartIndex.value = atPos
    mentionSearch.value = value.slice(atPos + 1, cursorPos)
    showMentionDropdown.value = true
  } else {
    showMentionDropdown.value = false
  }
}

function insertMention(name: string) {
  const before = newCommentContent.value.slice(0, mentionStartIndex.value)
  const after = newCommentContent.value.slice(mentionStartIndex.value + mentionSearch.value.length + 1)
  newCommentContent.value = `${before}@${name} ${after}`
  showMentionDropdown.value = false
}

function handleKeyDown(e: KeyboardEvent) {
  if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {
    e.preventDefault()
    handleAddComment()
  }
  if (e.key === 'Escape') {
    showMentionDropdown.value = false
  }
}

// Extract mention IDs from content
function extractMentionIds(content: string): string | null {
  const regex = /@(\S+)/g
  const mentions: string[] = []
  let match
  while ((match = regex.exec(content)) !== null) {
    mentions.push(match[1])
  }
  return mentions.length > 0 ? JSON.stringify(mentions) : null
}

async function loadComments() {
  try {
    comments.value = await listComments(props.projectId)
  } catch (e) {
    console.error('Failed to load comments:', e)
  }
}

async function loadShares() {
  try {
    shares.value = await listProjectShares(props.projectId)
  } catch (e) {
    console.error('Failed to load shares:', e)
  }
}

async function handleAddComment() {
  if (!newCommentContent.value.trim() || !authorName.value.trim()) return

  submitting.value = true
  try {
    const mentionIds = extractMentionIds(newCommentContent.value)
    await addComment(
      props.projectId,
      authorName.value.trim(),
      newCommentContent.value.trim(),
      props.noteId || null,
      mentionIds
    )
    newCommentContent.value = ''
    showMentionDropdown.value = false
    await loadComments()
  } catch (e) {
    console.error('Failed to add comment:', e)
    alert('添加评论失败: ' + String(e))
  } finally {
    submitting.value = false
  }
}

function startEdit(comment: Comment) {
  editingCommentId.value = comment.id
  editContent.value = comment.content
}

function cancelEdit() {
  editingCommentId.value = null
  editContent.value = ''
}

async function handleUpdateComment(commentId: string) {
  if (!editContent.value.trim()) return

  try {
    await updateComment(commentId, editContent.value.trim())
    editingCommentId.value = null
    editContent.value = ''
    await loadComments()
  } catch (e) {
    console.error('Failed to update comment:', e)
    alert('更新评论失败: ' + String(e))
  }
}

async function handleDelete(comment: Comment) {
  if (!confirm('确定要删除这条评论吗？')) return

  try {
    await deleteComment(comment.id)
    await loadComments()
  } catch (e) {
    console.error('Failed to delete comment:', e)
    alert('删除评论失败: ' + String(e))
  }
}

async function handleToggleResolve(comment: Comment) {
  try {
    await resolveComment(comment.id, !comment.resolved)
    await loadComments()
  } catch (e) {
    console.error('Failed to resolve comment:', e)
    alert('操作失败: ' + String(e))
  }
}

// Load saved author name from localStorage
function loadAuthorName() {
  const saved = localStorage.getItem('caelab_comment_author')
  if (saved) {
    authorName.value = saved
  }
}

// Save author name to localStorage
watch(authorName, (val) => {
  if (val.trim()) {
    localStorage.setItem('caelab_comment_author', val.trim())
  }
})

onMounted(() => {
  loadAuthorName()
  loadComments()
  loadShares()
})

// Reload comments when noteId changes
watch(() => props.noteId, () => {
  // Don't reload if in 'all' mode
  if (filterMode.value === 'current') {
    // filteredComments is computed, no reload needed
  }
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
