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
      <!-- 左侧笔记列表 -->
      <div class="w-64 border-r border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 overflow-y-auto">
        <div class="p-3">
          <h3 class="text-sm font-medium text-gray-600 dark:text-gray-400 mb-2">笔记列表</h3>
          <div v-if="!currentProjectId" class="text-sm text-gray-400 text-center py-4">
            请先选择一个项目
          </div>
          <div v-else-if="files.length === 0" class="text-sm text-gray-400 text-center py-4">
            暂无笔记，点击新建开始创作
          </div>
          <div v-else class="space-y-1">
            <div
              v-for="file in files"
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
              @embed="handleEmbed"
            />
          </div>
        </template>
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
import { useRoute } from 'vue-router'
import TipTapEditor from '@/components/editor/TipTapEditor.vue'
import type { EmbedItem, Project, ProjectFile } from '@/api'
import { listProjects, listFiles, createFile, updateFile, deleteFile, readFileContent, getFileTypeIcon } from '@/api'
import { formatProjectDate } from '@/api'
import { useProjectStore } from '@/stores/project'

const route = useRoute()
const router = useRouter()
const projectStore = useProjectStore()

// 获取当前笔记的嵌入记录
const currentEmbedRecords = computed(() => {
  if (!currentFileId.value) return []
  return projectStore.getEmbedRecordsByNoteId(currentFileId.value)
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

// 导入router
import { useRouter } from 'vue-router'
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