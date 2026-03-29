<template>
  <div class="notes-view h-full flex flex-col">
    <!-- 顶部工具栏 -->
    <div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900">
      <div class="flex items-center gap-4">
        <h2 class="text-xl font-bold text-gray-800 dark:text-white">笔记创作</h2>
        <div class="flex items-center gap-2 text-sm">
          <button 
            @click="currentNoteId = null"
            :class="currentNoteId === null ? 'bg-blue-500 text-white' : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300'"
            class="px-3 py-1 rounded hover:bg-gray-200 dark:hover:bg-gray-600"
          >
            新建
          </button>
          <button 
            @click="saveNote"
            :disabled="!currentNoteId"
            class="px-3 py-1 bg-green-500 text-white rounded hover:bg-green-600 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            保存
          </button>
          <button 
            @click="deleteNote"
            :disabled="!currentNoteId"
            class="px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            删除
          </button>
        </div>
      </div>
      <div class="flex items-center gap-2">
        <span class="text-sm text-gray-500 dark:text-gray-400">
          {{ currentNoteId ? '编辑中' : '新建笔记' }}
        </span>
      </div>
    </div>

    <div class="flex-1 flex overflow-hidden">
      <!-- 左侧笔记列表 -->
      <div class="w-64 border-r border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 overflow-y-auto">
        <div class="p-3">
          <h3 class="text-sm font-medium text-gray-600 dark:text-gray-400 mb-2">笔记本列表</h3>
          <div v-if="notes.length === 0" class="text-sm text-gray-400 text-center py-4">
            暂无笔记
          </div>
          <div v-else class="space-y-1">
            <div
              v-for="note in notes"
              :key="note.id"
              @click="selectNote(note)"
              :class="currentNoteId === note.id ? 'bg-blue-50 dark:bg-blue-900/30 border-blue-300' : ''"
              class="p-2 rounded cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700 border border-transparent"
            >
              <div class="text-sm font-medium text-gray-800 dark:text-white truncate">{{ note.title || '无标题' }}</div>
              <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                {{ formatDate(note.updatedAt) }}
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧编辑器区域 -->
      <div class="flex-1 flex flex-col overflow-hidden">
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
          <TipTapEditor
            v-model="noteContent"
            placeholder="开始创作你的笔记..."
            :editable="true"
            @embed="handleEmbed"
          />
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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import TipTapEditor from '@/components/editor/TipTapEditor.vue'
import type { Note, EmbedItem } from '@/types'
import { useRouter } from 'vue-router'

const router = useRouter()

const notes = ref<Note[]>([])
const currentNoteId = ref<string | null>(null)
const noteTitle = ref('')
const noteContent = ref('')
const embedPreviewItem = ref<EmbedItem | null>(null)

const loadNotes = () => {
  const savedNotes = localStorage.getItem('caelab_notes')
  if (savedNotes) {
    try {
      notes.value = JSON.parse(savedNotes)
    } catch {
      notes.value = []
    }
  }
  
  if (notes.value.length === 0) {
    notes.value = [
      {
        id: 'note-1',
        projectId: 'default',
        title: '欢迎使用CAELab',
        content: '<h1>欢迎使用CAELab</h1><p>这是一个科研与工程创作一体化工作台，支持富文本编辑、LaTeX公式、手写输入和内嵌3D模型等功能。</p>',
        createdAt: new Date(),
        updatedAt: new Date()
      }
    ]
    saveNotes()
  }
}

const selectNote = (note: Note) => {
  currentNoteId.value = note.id
  noteTitle.value = note.title
  noteContent.value = note.content
}

const saveNote = () => {
  if (!currentNoteId.value) return
  
  const noteIndex = notes.value.findIndex(n => n.id === currentNoteId.value)
  if (noteIndex !== -1) {
    notes.value[noteIndex].title = noteTitle.value
    notes.value[noteIndex].content = noteContent.value
    notes.value[noteIndex].updatedAt = new Date()
    saveNotes()
    console.log('笔记已保存')
  }
}

const deleteNote = () => {
  if (!currentNoteId.value) return
  
  if (confirm('确定要删除这篇笔记吗？')) {
    notes.value = notes.value.filter(n => n.id !== currentNoteId.value)
    currentNoteId.value = null
    noteTitle.value = ''
    noteContent.value = ''
    saveNotes()
  }
}

const saveNotes = () => {
  localStorage.setItem('caelab_notes', JSON.stringify(notes.value))
}

const handleEmbed = (item: EmbedItem) => {
  embedPreviewItem.value = item
}

const navigateToEmbed = () => {
  if (!embedPreviewItem.value) return
  
  const routes: Record<string, string> = {
    model: '/modeling',
    code: '/code',
    simulation: '/simulation'
  }
  
  router.push(routes[embedPreviewItem.value.type] || '/')
  embedPreviewItem.value = null
}

const getTypeName = (type: string) => {
  const names: Record<string, string> = {
    model: '3D建模',
    code: '代码编辑',
    simulation: '仿真分析'
  }
  return names[type] || type
}

const formatDate = (date: Date | string) => {
  const d = typeof date === 'string' ? new Date(date) : date
  return d.toLocaleDateString('zh-CN', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

onMounted(() => {
  loadNotes()
})
</script>

<style scoped>
.notes-view {
  height: 100%;
}
</style>