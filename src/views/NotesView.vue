<template>
  <div class="notes-view h-full flex flex-col">
    <!-- 顶部工具栏 -->
    <div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900">
      <div class="flex items-center gap-4">
        <h2 class="text-xl font-bold text-gray-800 dark:text-white">笔记创作</h2>
        
        <!-- 项目选择器 -->
        <div class="flex items-center gap-2">
          <select 
            v-model="currentProjectId"
            @change="loadFiles"
            class="px-3 py-1 border rounded bg-white dark:bg-gray-800 text-sm"
          >
            <option value="">选择项目...</option>
            <option v-for="p in projects" :key="p.id" :value="p.id">{{ p.name }}</option>
          </select>
        </div>

        <div class="flex items-center gap-2 text-sm">
          <button 
            @click="createNewNote"
            class="px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            新建
          </button>
          <button 
            @click="saveNote"
            :disabled="!currentFileId"
            class="px-3 py-1 bg-green-500 text-white rounded hover:bg-green-600 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            保存
          </button>
          <button 
            @click="openAiOptimizeDialog"
            :disabled="!noteContent.trim()"
            class="px-3 py-1 bg-purple-500 text-white rounded hover:bg-purple-600 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            🤖 AI优化
          </button>
          <button 
            @click="showHistory = true"
            :disabled="!currentFileId"
            class="px-3 py-1 bg-orange-500 text-white rounded hover:bg-orange-600 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            📜 历史
          </button>
          <button 
            @click="deleteNote"
            :disabled="!currentFileId"
            class="px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            删除
          </button>
        </div>
      </div>
      <div class="flex items-center gap-2">
        <span v-if="currentProjectId" class="text-sm text-gray-500 dark:text-gray-400">
          {{ currentFileId ? '编辑中' : '新建笔记' }}
        </span>
        <span v-else class="text-sm text-gray-500 dark:text-gray-400">
          请先选择项目
        </span>
      </div>
    </div>

    <div class="flex-1 flex overflow-hidden">
      <!-- 左侧笔记列表 + 搜索 -->
      <div class="w-72 border-r border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 overflow-y-auto">
        <div class="p-3">
          <!-- 搜索框 -->
          <div class="mb-4">
            <div class="relative">
              <input
                v-model="searchQuery"
                type="text"
                placeholder="搜索笔记..."
                class="w-full px-3 py-2 pl-10 border rounded-lg bg-white dark:bg-gray-700 text-sm"
                @input="onSearchInput"
              />
              <span class="absolute left-3 top-2.5 text-gray-400">🔍</span>
              <button 
                v-if="searchQuery"
                @click="clearSearch"
                class="absolute right-2 top-2 text-gray-400 hover:text-gray-600"
              >
                ✕
              </button>
            </div>
            <!-- 搜索结果 -->
            <div v-if="isSearching && searchResults.length > 0" class="mt-2 bg-white dark:bg-gray-700 rounded-lg shadow-lg max-h-64 overflow-y-auto">
              <div class="p-2 text-xs text-gray-500 dark:text-gray-400 border-b">
                找到 {{ searchResults.length }} 条结果
              </div>
              <div
                v-for="result in searchResults"
                :key="result.note_id"
                @click="selectFileById(result.note_id)"
                class="p-3 cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-600 border-b border-gray-100 dark:border-gray-600 last:border-0"
              >
                <div class="text-sm font-medium text-gray-800 dark:text-white">{{ result.title }}</div>
                <div class="text-xs text-gray-500 dark:text-gray-400 mt-1 line-clamp-2" v-html="result.snippet"></div>
              </div>
            </div>
          </div>

          <!-- 双向链接入口 -->
          <button 
            @click="showLinksPanel = !showLinksPanel"
            class="w-full mb-3 px-3 py-2 text-sm bg-indigo-100 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-300 rounded hover:bg-indigo-200 dark:hover:bg-indigo-900/50 flex items-center gap-2"
          >
            <span>🔗</span>
            <span>双向链接</span>
            <span class="ml-auto text-xs">({{ links.length + backlinks.length }})</span>
          </button>

          <!-- 链接面板 -->
          <div v-if="showLinksPanel && currentFileId" class="mb-3 p-3 bg-white dark:bg-gray-700 rounded-lg">
            <div class="text-xs font-medium text-gray-500 dark:text-gray-400 mb-2">引用的笔记</div>
            <div v-if="links.length === 0" class="text-xs text-gray-400 text-center py-2">暂无引用</div>
            <div v-else class="space-y-1">
              <div
                v-for="link in links"
                :key="link.id"
                @click="selectFileById(link.target_note_id)"
                class="flex items-center justify-between p-2 bg-gray-50 dark:bg-gray-600 rounded text-xs cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-500"
              >
                <span class="text-gray-700 dark:text-gray-300">→ {{ getFileName(link.target_note_id) }}</span>
                <button @click.stop="removeLink(link.id)" class="text-red-400 hover:text-red-600">✕</button>
              </div>
            </div>
            <div class="text-xs font-medium text-gray-500 dark:text-gray-400 mb-2 mt-3">被引用的笔记</div>
            <div v-if="backlinks.length === 0" class="text-xs text-gray-400 text-center py-2">暂无被引用</div>
            <div v-else class="space-y-1">
              <div
                v-for="link in backlinks"
                :key="link.id"
                @click="selectFileById(link.source_note_id)"
                class="flex items-center justify-between p-2 bg-gray-50 dark:bg-gray-600 rounded text-xs cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-500"
              >
                <span class="text-gray-700 dark:text-gray-300">← {{ getFileName(link.source_note_id) }}</span>
              </div>
            </div>
            <!-- 添加链接 -->
            <div class="mt-3 border-t border-gray-200 dark:border-gray-600 pt-3">
              <div class="text-xs text-gray-500 dark:text-gray-400 mb-1">添加引用</div>
              <select 
                v-model="newLinkTarget"
                class="w-full px-2 py-1 text-xs border rounded bg-white dark:bg-gray-800"
              >
                <option value="">选择笔记...</option>
                <option v-for="f in otherFiles" :key="f.id" :value="f.id">{{ f.file_name }}</option>
              </select>
              <button 
                @click="addLink"
                :disabled="!newLinkTarget"
                class="mt-2 w-full px-2 py-1 text-xs bg-indigo-500 text-white rounded hover:bg-indigo-600 disabled:opacity-50"
              >
                添加链接
              </button>
            </div>
          </div>

          <h3 class="text-sm font-medium text-gray-600 dark:text-gray-400 mb-2">笔记列表</h3>
          <div v-if="!currentProjectId" class="text-sm text-gray-400 text-center py-4">
            请先选择一个项目
          </div>
          <div v-else-if="displayFiles.length === 0" class="text-sm text-gray-400 text-center py-4">
            {{ searchQuery ? '无搜索结果' : '暂无笔记，点击新建开始创作' }}
          </div>
          <div v-else class="space-y-1">
            <div
              v-for="file in displayFiles"
              :key="file.id"
              @click="selectFile(file)"
              :class="currentFileId === file.id ? 'bg-blue-50 dark:bg-blue-900/30 border-blue-300' : ''"
              class="p-2 rounded cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700 border border-transparent"
            >
              <div class="text-sm font-medium text-gray-800 dark:text-white truncate flex items-center gap-2">
                <span>{{ getFileTypeIcon(file.file_type) }}</span>
                <span>{{ file.file_name || '无标题' }}</span>
              </div>
              <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                {{ formatDate(file.updated_at) }}
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧编辑器区域 -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <div v-if="!currentProjectId" class="flex-1 flex items-center justify-center">
          <div class="text-center text-gray-500">
            <div class="text-4xl mb-4">📝</div>
            <p>请从顶部选择一个项目，或创建新项目</p>
          </div>
        </div>
        
        <template v-else>
          <!-- 笔记标题 -->
          <div class="p-4 border-b border-gray-200 dark:border-gray-700">
            <input
              v-model="noteTitle"
              type="text"
              placeholder="笔记标题..."
              class="w-full text-xl font-bold bg-transparent border-none outline-none text-gray-800 dark:text-white placeholder-gray-400"
            />
          </div>

          <!-- 编辑器 -->
          <div class="flex-1 overflow-auto p-4">
            <!-- 嵌入记录展示 -->
            <div v-if="currentEmbedRecords.length > 0" class="mb-4 bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
              <h4 class="text-sm font-medium text-gray-600 dark:text-gray-400 mb-3 flex items-center gap-2">
                <span>🔗</span>
                <span>已嵌入对象</span>
              </h4>
              <div class="grid grid-cols-2 gap-2">
                <div
                  v-for="record in currentEmbedRecords"
                  :key="record.id"
                  @click="navigateToEmbedItem(record)"
                  class="flex items-center gap-3 p-3 bg-white dark:bg-gray-700 rounded border border-gray-200 dark:border-gray-600 cursor-pointer hover:border-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/30 transition"
                >
                  <span class="text-2xl">
                    {{ record.type === 'model' ? '📐' : record.type === 'code' ? '📄' : '📊' }}
                  </span>
                  <div class="flex-1 min-w-0">
                    <div class="text-sm font-medium text-gray-800 dark:text-white truncate">
                      {{ record.targetName }}
                    </div>
                    <div class="text-xs text-gray-500 dark:text-gray-400">
                      {{ record.type === 'model' ? '3D建模' : record.type === 'code' ? '代码' : '仿真结果' }}
                    </div>
                  </div>
                  <button 
                    @click.stop="removeEmbedRecord(record.id)"
                    class="text-gray-400 hover:text-red-500 text-xs"
                    title="移除嵌入"
                  >
                    ✕
                  </button>
                </div>
              </div>
            </div>

            <TipTapEditor
              v-model="noteContent"
              placeholder="开始创作你的笔记..."
              :editable="true"
              :notes="notesForEditor"
              :current-note-id="currentFileId || ''"
              @embed="handleEmbed"
              @link-note="handleLinkNote"
            />
          </div>
        </template>
      </div>
    </div>

    <!-- 版本历史弹窗 -->
    <div v-if="showHistory && currentFileId" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showHistory = false">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-[600px] max-h-[80vh] flex flex-col">
        <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
          <h3 class="text-lg font-semibold text-gray-800 dark:text-white">
            📜 版本历史
          </h3>
          <button @click="showHistory = false" class="text-gray-500 hover:text-gray-700 dark:text-gray-400">✕</button>
        </div>
        <div class="p-4 flex-1 overflow-auto">
          <div v-if="versions.length === 0" class="text-center py-8 text-gray-500">
            暂无版本记录
          </div>
          <div v-else class="space-y-3">
            <div
              v-for="(version, index) in versions"
              :key="version.id"
              class="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-600 transition"
            >
              <div class="flex justify-between items-start">
                <div>
                  <div class="text-sm font-medium text-gray-800 dark:text-white">
                    {{ version.title }}
                  </div>
                  <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                    {{ formatDate(version.created_at) }}
                    <span v-if="index === 0" class="ml-2 text-green-500 font-medium">最新</span>
                  </div>
                </div>
                <div class="flex gap-2">
                  <button 
                    @click="previewVersion(version)"
                    class="px-3 py-1 text-xs bg-blue-500 text-white rounded hover:bg-blue-600"
                  >
                    预览
                  </button>
                  <button 
                    v-if="index !== 0"
                    @click="confirmRestore(version)"
                    class="px-3 py-1 text-xs bg-orange-500 text-white rounded hover:bg-orange-600"
                  >
                    恢复
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-3">
          <button @click="saveCurrentVersion" class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 text-sm">
            保存当前版本
          </button>
          <button @click="showHistory = false" class="px-4 py-2 bg-gray-200 dark:bg-gray-700 rounded hover:bg-gray-300 dark:hover:bg-gray-600 text-sm">
            关闭
          </button>
        </div>
      </div>
    </div>

    <!-- 版本预览弹窗 -->
    <div v-if="previewVersionData" class="fixed inset-0 bg-black/50 flex items-center justify-center z-[60]" @click.self="previewVersionData = null">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-[700px] max-h-[85vh] flex flex-col">
        <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
          <h3 class="text-lg font-semibold text-gray-800 dark:text-white">
            版本预览 - {{ previewVersionData.title }}
          </h3>
          <button @click="previewVersionData = null" class="text-gray-500 hover:text-gray-700 dark:text-gray-400">✕</button>
        </div>
        <div class="p-4 flex-1 overflow-auto">
          <pre class="whitespace-pre-wrap text-sm text-gray-700 dark:text-gray-300 font-mono">{{ previewVersionData.content }}</pre>
        </div>
        <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-3">
          <button @click="previewVersionData = null" class="px-4 py-2 bg-gray-200 dark:bg-gray-700 rounded hover:bg-gray-300 dark:hover:bg-gray-600 text-sm">
            关闭
          </button>
        </div>
      </div>
    </div>

    <!-- 嵌入对象预览弹窗 -->
    <div v-if="embedPreviewItem" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="embedPreviewItem = null">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-[600px] max-h-[80vh] flex flex-col">
        <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
          <h3 class="text-lg font-semibold text-gray-800 dark:text-white">
            嵌入: {{ embedPreviewItem.name }}
          </h3>
          <button @click="embedPreviewItem = null" class="text-gray-500 hover:text-gray-700">
            ✕
          </button>
        </div>
        <div class="p-4 flex-1 overflow-auto">
          <div class="text-center py-8">
            <div class="text-6xl mb-4">
              {{ embedPreviewItem.type === 'model' ? '📐' : embedPreviewItem.type === 'code' ? '📄' : '📊' }}
            </div>
            <div class="text-gray-600 dark:text-gray-400">
              点击跳转到 {{ getTypeName(embedPreviewItem.type) }} 模块查看详情
            </div>
          </div>
        </div>
        <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-3">
          <button @click="embedPreviewItem = null" class="px-4 py-2 bg-gray-200 dark:bg-gray-700 rounded hover:bg-gray-300 dark:hover:bg-gray-600">
            关闭
          </button>
          <button @click="navigateToEmbed" class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
            跳转查看
          </button>
        </div>
      </div>
    </div>

    <!-- AI优化对话框 -->
    <div v-if="showAiOptimize" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showAiOptimize = false">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-[700px] flex flex-col max-h-[85vh]">
        <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
          <h3 class="text-lg font-semibold text-gray-800 dark:text-white">
            🤖 AI优化笔记
          </h3>
          <button @click="showAiOptimize = false" class="text-gray-500 hover:text-gray-700 dark:text-gray-400">✕</button>
        </div>
        <div class="p-4 flex-1 overflow-auto">
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
            AI将优化当前笔记内容，包括润色、完善格式、补充内容
          </p>
          <!-- AI思考过程 -->
          <div v-if="aiOptimizing" class="bg-gray-50 dark:bg-gray-700 rounded p-3 mb-4">
            <div class="text-xs text-gray-500 dark:text-gray-400 mb-2">AI 优化中...</div>
            <div class="text-xs text-gray-600 dark:text-gray-300 font-mono whitespace-pre-wrap">
              {{ aiOptimizeThinking }}
            </div>
          </div>
          <!-- 原始内容预览 -->
          <div v-if="aiOptimizedContent" class="mb-4">
            <div class="text-xs font-semibold text-gray-500 dark:text-gray-400 mb-2">原始内容预览:</div>
            <div class="bg-gray-100 dark:bg-gray-900 rounded p-3 text-xs text-gray-600 dark:text-gray-300 max-h-32 overflow-auto">
              {{ noteContent.slice(0, 500) }}{{ noteContent.length > 500 ? '...' : '' }}
            </div>
          </div>
          <!-- 优化后的内容 -->
          <div v-if="aiOptimizedContent" class="mb-4">
            <div class="text-xs font-semibold text-blue-600 dark:text-blue-400 mb-2">优化后的内容:</div>
            <div class="bg-blue-50 dark:bg-blue-900/20 rounded p-3 text-xs text-gray-700 dark:text-gray-300 max-h-64 overflow-auto whitespace-pre-wrap">
              {{ aiOptimizedContent }}
            </div>
          </div>
          <!-- 错误信息 -->
          <div v-if="aiOptimizeError" class="bg-red-50 dark:bg-red-900/20 rounded p-3 text-xs text-red-600 dark:text-red-400">
            {{ aiOptimizeError }}
          </div>
        </div>
        <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-3">
          <button @click="showAiOptimize = false" class="px-4 py-2 bg-gray-200 dark:bg-gray-700 rounded hover:bg-gray-300 dark:hover:bg-gray-600 text-sm">
            取消
          </button>
          <button v-if="aiOptimizedContent" @click="applyOptimizedContent" class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 text-sm">
            应用优化结果
          </button>
          <button v-else @click="doAiOptimize" :disabled="aiOptimizing" class="px-4 py-2 bg-purple-500 text-white rounded hover:bg-purple-600 text-sm disabled:opacity-50">
            {{ aiOptimizing ? '优化中...' : '开始优化' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import TipTapEditor from '@/components/editor/TipTapEditor.vue'
import type { EmbedItem, Project, ProjectFile } from '@/api'
import { 
  listProjects, listFiles, createFile, updateFile, deleteFile, readFileContent, getFileTypeIcon,
  saveNoteVersion, getNoteVersions, restoreNoteVersion,
  createNoteLink, getNoteLinks, getNoteBacklinks, deleteNoteLink,
  searchNotes, type NoteVersion, type NoteLink, type SearchResult
} from '@/api'
import { formatProjectDate } from '@/api'
import { useProjectStore } from '@/stores/project'

const route = useRoute()
const router = useRouter()
const projectStore = useProjectStore()

// ============ 双向链接状态 ============
const links = ref<NoteLink[]>([])
const backlinks = ref<NoteLink[]>([])
const showLinksPanel = ref(false)
const newLinkTarget = ref('')

// ============ 版本历史状态 ============
const versions = ref<NoteVersion[]>([])
const showHistory = ref(false)
const previewVersionData = ref<NoteVersion | null>(null)

// ============ 搜索状态 ============
const searchQuery = ref('')
const searchResults = ref<SearchResult[]>([])
const isSearching = ref(false)
let searchTimeout: ReturnType<typeof setTimeout> | null = null

// 获取当前笔记的嵌入记录
const currentEmbedRecords = computed(() => {
  if (!currentFileId.value) return []
  return projectStore.getEmbedRecordsByNoteId(currentFileId.value)
})

// 可引用的笔记列表（排除当前笔记）
const otherFiles = computed(() => {
  return files.value.filter(f => f.id !== currentFileId.value)
})

// 编辑器可用的笔记列表
const notesForEditor = computed(() => {
  return files.value.map(f => ({
    id: f.id,
    title: f.file_name || '无标题',
    updated: formatDate(f.updated_at)
  }))
})

// 显示的文件列表（搜索模式/普通模式）
const displayFiles = computed(() => {
  if (isSearching.value && searchResults.value.length > 0) {
    // 搜索模式下显示匹配的文件
    const matchedIds = new Set(searchResults.value.map(r => r.note_id))
    return files.value.filter(f => matchedIds.has(f.id))
  }
  return files.value
})

// 跳转到嵌入对象
function navigateToEmbedItem(record: { type: 'model' | 'code' | 'simulation'; targetId: string; noteId: string }) {
  const routes: Record<string, string> = {
    model: '/modeling',
    code: '/code',
    simulation: '/simulation'
  }
  projectStore.setCurrentNoteId(currentFileId.value)
  router.push(routes[record.type] || '/')
}

// 移除嵌入记录
function removeEmbedRecord(id: string) {
  projectStore.removeEmbedRecord(id)
}

// ============ 双向链接方法 ============
async function loadLinks() {
  if (!currentFileId.value) {
    links.value = []
    backlinks.value = []
    return
  }
  
  try {
    links.value = await getNoteLinks(currentFileId.value)
    backlinks.value = await getNoteBacklinks(currentFileId.value)
  } catch (e) {
    console.error('Failed to load links:', e)
  }
}

function getFileName(noteId: string): string {
  const file = files.value.find(f => f.id === noteId)
  return file?.file_name || '未知笔记'
}

async function addLink() {
  if (!currentFileId.value || !newLinkTarget.value) return
  
  try {
    await createNoteLink(currentFileId.value, newLinkTarget.value)
    newLinkTarget.value = ''
    await loadLinks()
  } catch (e: any) {
    console.error('Failed to add link:', e)
    if (e.message?.includes('already')) {
      alert('链接已存在')
    }
  }
}

// 从编辑器插入链接（自动创建链接记录）
async function handleLinkNote(targetNoteId: string, _targetTitle: string) {
  if (!currentFileId.value) return
  
  try {
    await createNoteLink(currentFileId.value, targetNoteId)
    await loadLinks()
  } catch (e: any) {
    // 忽略重复链接错误
    if (!e.message?.includes('already')) {
      console.error('Failed to create link:', e)
    }
  }
}

async function removeLink(linkId: string) {
  try {
    await deleteNoteLink(linkId)
    await loadLinks()
  } catch (e) {
    console.error('Failed to remove link:', e)
  }
}

function selectFileById(id: string) {
  const file = files.value.find(f => f.id === id)
  if (file) {
    selectFile(file)
    showLinksPanel.value = false
    searchQuery.value = ''
    isSearching.value = false
  }
}

// ============ 版本历史方法 ============
async function loadVersions() {
  if (!currentFileId.value) return
  
  try {
    versions.value = await getNoteVersions(currentFileId.value)
  } catch (e) {
    console.error('Failed to load versions:', e)
  }
}

async function saveCurrentVersion() {
  if (!currentFileId.value) return
  
  try {
    await saveNoteVersion(currentFileId.value, noteTitle.value, noteContent.value)
    await loadVersions()
    alert('版本已保存')
  } catch (e) {
    console.error('Failed to save version:', e)
    alert('保存失败')
  }
}

function previewVersion(version: NoteVersion) {
  previewVersionData.value = version
}

async function confirmRestore(version: NoteVersion) {
  if (!confirm(`确定要恢复到 "${version.title}" 版本吗？当前内容会先保存为新版本。`)) {
    return
  }
  
  try {
    const restored = await restoreNoteVersion(currentFileId.value!, version.id)
    noteTitle.value = restored.file_name
    noteContent.value = restored.content || ''
    await loadVersions()
    alert('恢复成功')
    showHistory.value = false
  } catch (e) {
    console.error('Failed to restore version:', e)
    alert('恢复失败')
  }
}

// ============ 搜索方法 ============
function onSearchInput() {
  if (searchTimeout) clearTimeout(searchTimeout)
  
  if (!searchQuery.value.trim()) {
    isSearching.value = false
    searchResults.value = []
    return
  }
  
  isSearching.value = true
  searchTimeout = setTimeout(async () => {
    try {
      searchResults.value = await searchNotes(currentProjectId.value, searchQuery.value)
    } catch (e) {
      console.error('Search failed:', e)
      searchResults.value = []
    }
  }, 300)
}

function clearSearch() {
  searchQuery.value = ''
  isSearching.value = false
  searchResults.value = []
}

// 状态
const projects = ref<Project[]>([])
const files = ref<ProjectFile[]>([])
const currentProjectId = ref<string>('')
const currentFileId = ref<string | null>(null)
const noteTitle = ref('')
const noteContent = ref('')
const embedPreviewItem = ref<EmbedItem | null>(null)
const loading = ref(false)

// AI优化状态
const showAiOptimize = ref(false)
const aiOptimizing = ref(false)
const aiOptimizeThinking = ref('')
const aiOptimizedContent = ref('')
const aiOptimizeError = ref('')

// 加载项目列表
async function loadProjects() {
  try {
    projects.value = await listProjects()
    
    // 如果URL有projectId参数，使用它
    const urlProjectId = route.query.projectId as string
    if (urlProjectId) {
      currentProjectId.value = urlProjectId
    } else if (projects.value.length > 0) {
      currentProjectId.value = projects.value[0].id
    }
  } catch (error) {
    console.error('Failed to load projects:', error)
  }
}

// 加载项目的文件列表
async function loadFiles() {
  if (!currentProjectId.value) {
    files.value = []
    return
  }

  try {
    loading.value = true
    const allFiles = await listFiles(currentProjectId.value)
    // 只显示笔记类型的文件
    files.value = allFiles.filter(f => f.file_type === 'note')
    currentFileId.value = null
    noteTitle.value = ''
    noteContent.value = ''
    links.value = []
    backlinks.value = []
  } catch (error) {
    console.error('Failed to load files:', error)
  } finally {
    loading.value = false
  }
}

// 选择文件
async function selectFile(file: ProjectFile) {
  currentFileId.value = file.id
  noteTitle.value = file.file_name
  
  try {
    loading.value = true
    noteContent.value = await readFileContent(file.id)
    // 加载双向链接
    await loadLinks()
  } catch (error) {
    console.error('Failed to read file content:', error)
    noteContent.value = file.content || ''
  } finally {
    loading.value = false
  }
}

// 创建新笔记
async function createNewNote() {
  if (!currentProjectId.value) {
    alert('请先选择一个项目')
    return
  }

  try {
    loading.value = true
    const newFile = await createFile({
      project_id: currentProjectId.value,
      file_type: 'note',
      file_name: '新建笔记',
      content: '<p>开始你的创作...</p>',
      file_path: ''
    })
    
    currentFileId.value = newFile.id
    noteTitle.value = newFile.file_name
    noteContent.value = newFile.content || ''
    
    await loadFiles()
  } catch (error) {
    console.error('Failed to create note:', error)
    alert('创建笔记失败: ' + (error as Error).message)
  } finally {
    loading.value = false
  }
}

// 保存笔记
async function saveNote() {
  if (!currentFileId.value) return

  try {
    loading.value = true
    await updateFile({
      id: currentFileId.value,
      file_name: noteTitle.value,
      content: noteContent.value
    })
    await loadFiles()
    console.log('笔记已保存')
  } catch (error) {
    console.error('Failed to save note:', error)
    alert('保存失败: ' + (error as Error).message)
  } finally {
    loading.value = false
  }
}

// 删除笔记
async function deleteNote() {
  if (!currentFileId.value) return

  if (confirm('确定要删除这篇笔记吗？')) {
    try {
      loading.value = true
      await deleteFile(currentFileId.value)
      currentFileId.value = null
      noteTitle.value = ''
      noteContent.value = ''
      await loadFiles()
    } catch (error) {
      console.error('Failed to delete note:', error)
      alert('删除失败: ' + (error as Error).message)
    } finally {
      loading.value = false
    }
  }
}

// 处理嵌入
function handleEmbed(item: EmbedItem) {
  embedPreviewItem.value = item
  // 如果当前有笔记ID，设置它以便其他工具可以嵌入到此笔记
  if (currentFileId.value) {
    projectStore.setCurrentNoteId(currentFileId.value)
  }
}

// 跳转到嵌入对象
function navigateToEmbed() {
  if (!embedPreviewItem.value) return

  const routes: Record<string, string> = {
    model: '/modeling',
    code: '/code',
    simulation: '/simulation'
  }

  // 设置当前笔记ID，方便从其他工具嵌入
  if (currentFileId.value) {
    projectStore.setCurrentNoteId(currentFileId.value)
  }

  router.push(routes[embedPreviewItem.value.type] || '/')
  embedPreviewItem.value = null
}

function getTypeName(type: string) {
  const names: Record<string, string> = {
    model: '3D建模',
    code: '代码编辑',
    simulation: '仿真分析'
  }
  return names[type] || type
}

function formatDate(dateStr: string) {
  return formatProjectDate(dateStr)
}

import { invoke } from '@tauri-apps/api/tauri'
import { useAiStore } from '@/stores/ai'

const aiStore = useAiStore()

// AI优化笔记
function openAiOptimizeDialog() {
  showAiOptimize.value = true
  aiOptimizedContent.value = ''
  aiOptimizeError.value = ''
  aiOptimizeThinking.value = ''
}

async function doAiOptimize() {
  if (!noteContent.value.trim() || aiOptimizing.value) return

  aiOptimizing.value = true
  aiOptimizeThinking.value = '正在调用AI优化...'
  aiOptimizeError.value = ''

  try {
    const prompt = `你是一位专业的技术文档编辑。请优化以下笔记内容：

要求：
1. 润色语言，使其更加流畅专业
2. 完善格式，使用适当的标题、列表等
3. 补充完善可能遗漏的内容
4. 保持原意，不要改变核心内容
5. 如果有技术术语，确保使用正确

原始内容：
${noteContent.value}

请直接返回优化后的内容，不需要解释。`

    const response = await invoke<string>('ai_chat_completion', {
      messages: [{ role: 'user', content: prompt }],
      config: aiStore.config
    })

    aiOptimizeThinking.value = '优化完成！'
    aiOptimizedContent.value = response

  } catch (e: any) {
    console.error('AI优化失败:', e)
    aiOptimizeError.value = `优化失败: ${String(e)}`
  } finally {
    aiOptimizing.value = false
  }
}

function applyOptimizedContent() {
  if (aiOptimizedContent.value) {
    noteContent.value = aiOptimizedContent.value
    showAiOptimize.value = false
    aiOptimizedContent.value = ''
  }
}

// 监听项目变化
watch(currentProjectId, () => {
  if (currentProjectId.value) {
    loadFiles()
  }
})

onMounted(() => {
  loadProjects()
  // Update AI context
  aiStore.updateContext({
    currentTool: 'notes',
    ...(noteContent.value && {
      noteContent: noteContent.value.slice(0, 2000) // Limit length
    }),
    ...(currentProjectId.value && projects.value.find(p => p.id === currentProjectId.value) && {
      projectName: projects.value.find(p => p.id === currentProjectId.value)!.name
    })
  })
})
</script>

<style scoped>
.notes-view {
  height: 100%;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.animate-spin {
  animation: spin 1s linear infinite;
}
</style>