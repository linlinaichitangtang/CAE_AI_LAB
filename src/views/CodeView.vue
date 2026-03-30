<template>
  <div class="code-view flex h-full" @contextmenu.prevent>
    <!-- File Tree Panel -->
    <div class="file-tree w-60 bg-[#1e1e1e] border-r border-[#3c3c3c] flex flex-col overflow-hidden">
      <!-- Tree Header -->
      <div class="tree-header px-3 py-2 text-xs text-[#858585] border-b border-[#3c3c3c] uppercase tracking-wider">
        文件资源管理器
      </div>
      
      <!-- Tree Content -->
      <div class="tree-content flex-1 overflow-y-auto py-1">
        <FileTreeItem
          v-for="node in fileTree"
          :key="node.path"
          :node="node"
          @select="handleFileSelect"
        />
      </div>
    </div>

    <!-- Editor Area -->
    <div class="editor-area flex-1 flex flex-col min-w-0">
      <!-- Tabs Bar -->
      <div class="tabs-bar h-9 bg-[#2d2d2d] flex items-center overflow-x-auto border-b border-[#3c3c3c]"
           style="scrollbar-width: none;">
        <div
          v-for="tab in tabs"
          :key="tab.id"
          class="tab flex items-center h-full px-3 cursor-pointer border-r border-[#3c3c3c] group transition-colors"
          :class="[
            activeTabId === tab.id
              ? 'bg-[#1e1e1e] text-[#d4d4d4]'
              : 'bg-[#2d2d2d] text-[#858585] hover:bg-[#323232]'
          ]"
          @click="switchTab(tab.id)"
          @mousedown.prevent
          @click.right.prevent="showTabContextMenu($event, tab)"
        >
          <span class="text-xs whitespace-nowrap">{{ tab.name }}</span>
          <button
            v-if="tab.dirty"
            class="ml-1.5 w-4 h-4 flex items-center justify-center text-[#858585] hover:text-[#d4d4d4] rounded"
            @click.stop="closeTab(tab.id)"
          >
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>
          <span v-else class="ml-1.5 w-4 h-4 flex items-center justify-center opacity-0 group-hover:opacity-100 cursor-pointer hover:text-[#d4d4d4]"
                @click.stop="closeTab(tab.id)">
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </span>
        </div>

        <!-- Tab Actions -->
        <div class="ml-auto px-2 flex items-center gap-1">
          <!-- 🔗 嵌入到笔记 -->
          <button 
            @click="showEmbedToNoteDialog"
            :disabled="!projectStore.currentNoteId || !activeTabId"
            class="p-1 text-orange-400 hover:text-orange-300 transition-colors border border-orange-400 rounded px-2 flex items-center gap-1 text-xs"
            title="将当前代码嵌入到笔记"
          >
            <span>🔗</span>
            <span>嵌入</span>
          </button>
          <div class="w-px h-4 bg-[#3c3c3c] mx-1"></div>
          <!-- AI功能按钮 -->
          <button @click="openAiExplainDialog" :disabled="!activeTabId" class="p-1 text-purple-400 hover:text-purple-300 transition-colors" title="AI解释代码">
            🤖 解释
          </button>
          <button @click="openAiOptimizeDialog" :disabled="!activeTabId" class="p-1 text-blue-400 hover:text-blue-300 transition-colors" title="AI优化代码">
            🤖 优化
          </button>
          <div class="w-px h-4 bg-[#3c3c3c] mx-1"></div>
          <button class="p-1 text-[#858585] hover:text-[#d4d4d4] transition-colors" @click="toggleTerminal" title="终端">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
            </svg>
          </button>
        </div>
      </div>

      <!-- Editor Container -->
      <div class="editor-container flex-1 relative bg-[#1e1e1e]">
        <div
          v-if="tabs.length === 0"
          class="absolute inset-0 flex flex-col items-center justify-center text-[#858585]"
        >
          <svg class="w-16 h-16 mb-4 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"/>
          </svg>
          <p class="text-sm">从左侧文件树选择一个文件开始编辑</p>
          <p class="text-xs mt-2 opacity-60">或使用 Ctrl+P 快速打开</p>
        </div>
        <div
          v-show="tabs.length > 0"
          ref="editorContainer"
          class="absolute inset-0"
        />
      </div>

      <!-- Terminal Panel (toggleable) -->
      <div
        v-if="showTerminal"
        class="terminal-panel h-48 bg-[#1e1e1e] border-t border-[#3c3c3c] flex flex-col"
      >
        <div class="terminal-header px-3 py-1.5 flex items-center justify-between border-b border-[#3c3c3c]">
          <span class="text-xs text-[#858585]">终端</span>
          <button class="text-[#858585] hover:text-[#d4d4d4] transition-colors" @click="showTerminal = false">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>
        </div>
        <div class="terminal-content flex-1 p-3 font-mono text-xs text-[#d4d4d4] overflow-y-auto">
          <div class="text-[#4ec9b0]">$ caelab dev</div>
          <div class="text-[#858585] mt-1">VITE v5.2.0 ready in 342ms</div>
          <div class="mt-1">➜ <span class="text-[#569cd6]">Local:</span> <span class="text-[#4ec9b0]">http://localhost:1420/</span></div>
        </div>
      </div>
    </div>

    <!-- Right Panel: Outline / Minimap Toggle -->
    <div class="right-panel w-48 bg-[#1e1e1e] border-l border-[#3c3c3c] flex flex-col overflow-hidden">
      <div class="panel-header px-3 py-2 text-xs text-[#858585] border-b border-[#3c3c3c] uppercase tracking-wider">
        大纲
      </div>
      <div class="panel-content flex-1 overflow-y-auto p-3">
        <div v-if="activeTabId" class="text-xs text-[#858585]">
          <div class="text-[#4ec9b0] mb-2">当前文件</div>
          <div class="text-[#d4d4d4]">{{ tabs.find(t => t.id === activeTabId)?.name || '-' }}</div>
          <div class="mt-3 text-[#6a9955]">// 符号导航</div>
          <div class="text-[#c586c0] mt-1">functions: {{ activeTabId ? '...' : '-' }}</div>
        </div>
        <div v-else class="text-xs text-[#858585] text-center mt-8">
          无活动文件
        </div>
      </div>
    </div>

    <!-- AI解释对话框 -->
    <div v-if="showAiExplain" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showAiExplain = false">
      <div class="bg-[#1e1e1e] rounded-lg shadow-xl w-[700px] flex flex-col max-h-[80vh] border border-[#3c3c3c]">
        <div class="p-4 border-b border-[#3c3c3c] flex justify-between items-center">
          <h3 class="text-lg font-semibold text-[#d4d4d4]">
            🤖 AI解释代码
          </h3>
          <button @click="showAiExplain = false" class="text-[#858585] hover:text-[#d4d4d4]">✕</button>
        </div>
        <div class="p-4 flex-1 overflow-auto">
          <p class="text-xs text-[#858585] mb-4">AI将分析当前代码并给出详细解释</p>
          <div v-if="aiExplainThinking" class="bg-[#2d2d2d] rounded p-3 mb-4">
            <div class="text-xs text-[#858585] mb-2">AI 分析中...</div>
            <div class="text-xs text-[#d4d4d4] font-mono whitespace-pre-wrap">
              {{ aiExplainThinking }}
            </div>
          </div>
          <div v-if="aiExplainResult" class="bg-[#2d2d2d] rounded p-4 text-xs text-[#d4d4d4] max-h-64 overflow-auto whitespace-pre-wrap">
            {{ aiExplainResult }}
          </div>
          <div v-if="aiExplainError" class="bg-red-900/30 rounded p-3 text-xs text-red-400">
            {{ aiExplainError }}
          </div>
        </div>
        <div class="p-4 border-t border-[#3c3c3c] flex justify-end gap-3">
          <button @click="showAiExplain = false" class="px-4 py-2 bg-[#3c3c3c] rounded text-[#d4d4d4] hover:bg-[#4c4c4c] text-sm">
            关闭
          </button>
          <button v-if="!aiExplainResult" @click="doAiExplain" :disabled="aiExplaining" class="px-4 py-2 bg-purple-600 text-white rounded hover:bg-purple-700 text-sm disabled:opacity-50">
            {{ aiExplaining ? '分析中...' : '开始分析' }}
          </button>
        </div>
      </div>
    </div>

    <!-- AI优化对话框 -->
    <div v-if="showAiOptimize" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showAiOptimize = false">
      <div class="bg-[#1e1e1e] rounded-lg shadow-xl w-[800px] flex flex-col max-h-[85vh] border border-[#3c3c3c]">
        <div class="p-4 border-b border-[#3c3c3c] flex justify-between items-center">
          <h3 class="text-lg font-semibold text-[#d4d4d4]">
            🤖 AI优化代码
          </h3>
          <button @click="showAiOptimize = false" class="text-[#858585] hover:text-[#d4d4d4]">✕</button>
        </div>
        <div class="p-4 flex-1 overflow-auto">
          <p class="text-xs text-[#858585] mb-4">AI将优化当前代码，包括改善代码结构、提高可读性、修复潜在问题</p>
          <div v-if="aiOptimizing" class="bg-[#2d2d2d] rounded p-3 mb-4">
            <div class="text-xs text-[#858585] mb-2">AI 优化中...</div>
            <div class="text-xs text-[#d4d4d4] font-mono whitespace-pre-wrap">
              {{ aiOptimizeThinking }}
            </div>
          </div>
          <div v-if="aiOptimizedCode" class="mb-4">
            <div class="text-xs font-semibold text-blue-400 mb-2">优化后的代码:</div>
            <pre class="bg-[#2d2d2d] rounded p-4 text-xs text-[#d4d4d4] max-h-64 overflow-auto whitespace-pre-wrap">{{ aiOptimizedCode }}</pre>
          </div>
          <div v-if="aiOptimizeError" class="bg-red-900/30 rounded p-3 text-xs text-red-400">
            {{ aiOptimizeError }}
          </div>
        </div>
        <div class="p-4 border-t border-[#3c3c3c] flex justify-end gap-3">
          <button @click="showAiOptimize = false" class="px-4 py-2 bg-[#3c3c3c] rounded text-[#d4d4d4] hover:bg-[#4c4c4c] text-sm">
            取消
          </button>
          <button v-if="aiOptimizedCode" @click="applyOptimizedCode" class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 text-sm">
            应用优化结果
          </button>
          <button v-else @click="doAiOptimize" :disabled="aiOptimizing" class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 text-sm disabled:opacity-50">
            {{ aiOptimizing ? '优化中...' : '开始优化' }}
          </button>
        </div>
      </div>
    </div>

    <!-- 🔗 嵌入到笔记对话框 -->
    <div v-if="showEmbedDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showEmbedDialog = false">
      <div class="bg-[#1e1e1e] rounded-lg shadow-xl w-[500px] border border-[#3c3c3c]">
        <div class="p-4 border-b border-[#3c3c3c] flex justify-between items-center">
          <h3 class="text-lg font-semibold text-[#d4d4d4] flex items-center gap-2">
            <span>🔗</span>
            <span>嵌入到笔记</span>
          </h3>
          <button @click="showEmbedDialog = false" class="text-[#858585] hover:text-[#d4d4d4]">✕</button>
        </div>
        <div class="p-4">
          <div class="mb-4">
            <label class="text-sm font-medium text-[#d4d4d4] mb-2 block">选择要嵌入的笔记</label>
            <select 
              v-model="selectedEmbedNoteId"
              class="w-full px-3 py-2 border border-[#3c3c3c] rounded bg-[#2d2d2d] text-[#d4d4d4] text-sm"
            >
              <option value="">-- 选择笔记 --</option>
              <option v-for="note in codeFiles" :key="note.id" :value="note.id">
                {{ note.file_name || '无标题笔记' }}
              </option>
            </select>
          </div>
          
          <!-- 嵌入预览 -->
          <div class="bg-[#2d2d2d] rounded p-3">
            <div class="flex items-center gap-3">
              <span class="text-2xl">📄</span>
              <div>
                <div class="text-sm font-medium text-[#d4d4d4]">
                  {{ tabs.find(t => t.id === activeTabId)?.name || '代码文件' }} (代码)
                </div>
                <div class="text-xs text-[#858585]">
                  {{ new Date().toLocaleDateString() }}
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="p-4 border-t border-[#3c3c3c] flex justify-end gap-3">
          <button @click="showEmbedDialog = false" class="px-4 py-2 bg-[#3c3c3c] rounded text-[#d4d4d4] hover:bg-[#4c4c4c] text-sm">
            取消
          </button>
          <button 
            @click="embedCodeToNote" 
            :disabled="!selectedEmbedNoteId || !activeTabId"
            class="px-4 py-2 bg-orange-500 text-white rounded hover:bg-orange-600 text-sm disabled:opacity-50 disabled:cursor-not-allowed"
          >
            确认嵌入
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue'
import { useCodeEditor, type FileTreeNode } from '@/composables/useCodeEditor'
import { invoke } from '@tauri-apps/api/tauri'
import { useAiStore } from '@/stores/ai'
import { useProjectStore } from '@/stores/project'
const aiStore = useAiStore()
const projectStore = useProjectStore()

// Dynamic worker import for Monaco
let monacoLoaded = false

const loadMonacoWorkers = async () => {
  if (monacoLoaded) return
  monacoLoaded = true
  
  // @ts-ignore - Dynamic Vite worker import
  const [editorWorker, tsWorker] = await Promise.all([
    import('monaco-editor/esm/vs/editor/editor.worker?worker'),
    import('monaco-editor/esm/vs/language/typescript/ts.worker?worker')
  ])

  // @ts-ignore
  self.MonacoEnvironment = {
    getWorker(_: any, label: string) {
      if (label === 'typescript' || label === 'javascript') {
        return new tsWorker.default()
      }
      return new editorWorker.default()
    }
  }
}

// File Tree Item Component
import { defineComponent, h, type VNode } from 'vue'

const FileTreeItem = defineComponent({
  name: 'FileTreeItem',
  props: {
    node: { type: Object as () => FileTreeNode, required: true },
    depth: { type: Number, default: 0 }
  },
  emits: ['select'],
  setup(props, { emit }): () => VNode | null {
    const expanded = ref(props.node.expanded || false)

    const toggle = () => {
      if (props.node.type === 'folder') {
        expanded.value = !expanded.value
      }
    }

    const handleClick = () => {
      if (props.node.type === 'file') {
        emit('select', { path: props.node.path, name: props.node.name })
      } else {
        toggle()
      }
    }

    return (): VNode | null => {
      const children: VNode[] = []

      // Indent
      children.push(h('div', { style: { width: `${8 + props.depth * 12}px`, display: 'inline-block' } }))

      // Arrow for folders
      if (props.node.type === 'folder') {
        children.push(
          h('span', {
            class: 'mr-1 text-[#858585] transition-transform inline-block w-3',
            style: { transform: expanded.value ? 'rotate(90deg)' : 'rotate(0deg)' },
            onClick: (e: Event) => { e.stopPropagation(); toggle() }
          }, '▶')
        )
      }

      // Icon
      const icon = props.node.type === 'folder'
        ? '📁'
        : props.node.name.endsWith('.ts') || props.node.name.endsWith('.tsx')
          ? '🔷'
          : props.node.name.endsWith('.vue')
            ? '💚'
            : props.node.name.endsWith('.md')
              ? '📄'
              : props.node.name.endsWith('.json')
                ? '📋'
                : '📄'

      children.push(h('span', { class: 'mr-1.5 text-sm' }, icon))

      // Name
      children.push(h('span', { class: 'text-sm text-[#cccccc]' }, props.node.name))

      // Children
      if (props.node.type === 'folder' && expanded.value && props.node.children) {
        children.push(
          h('div', { class: 'ml-2' },
            props.node.children.map(child =>
              h(FileTreeItem, {
                key: child.path,
                node: child,
                depth: props.depth + 1,
                onSelect: (data: { path: string; name: string }) => emit('select', data)
              })
            )
          )
        )
      }

      return h(
        'div',
        {
          class: 'tree-node flex items-center py-0.5 cursor-pointer hover:bg-[#2a2d2e] text-[#cccccc] text-sm',
          style: { paddingLeft: `${8 + props.depth * 12}px`, paddingRight: '8px' },
          onClick: handleClick
        },
        children
      )
    }
  }
})

// Main component
const editorContainer = ref<HTMLElement | null>(null)
const showTerminal = ref(false)

// AI 功能状态
const showAiExplain = ref(false)
const aiExplaining = ref(false)
const aiExplainThinking = ref('')
const aiExplainResult = ref('')
const aiExplainError = ref('')

const showAiOptimize = ref(false)
const aiOptimizing = ref(false)
const aiOptimizeThinking = ref('')
const aiOptimizedCode = ref('')
const aiOptimizeError = ref('')

// ========== 嵌入到笔记功能 ==========
const showEmbedDialog = ref(false)
const selectedEmbedNoteId = ref<string>('')
const codeFiles = ref<any[]>([])

async function loadFilesForEmbed() {
  if (projectStore.currentProject) {
    try {
      const { listFiles } = await import('@/api')
      const allFiles = await listFiles(projectStore.currentProject.id)
      codeFiles.value = allFiles.filter((f: any) => f.file_type === 'note')
    } catch (e) {
      console.error('加载笔记列表失败:', e)
    }
  }
}

function showEmbedToNoteDialog() {
  if (codeFiles.value.length === 0) {
    loadFilesForEmbed()
  }
  if (!projectStore.currentNoteId && codeFiles.value.length > 0) {
    selectedEmbedNoteId.value = codeFiles.value[0]?.id || ''
  } else {
    selectedEmbedNoteId.value = projectStore.currentNoteId || ''
  }
  showEmbedDialog.value = true
}

async function embedCodeToNote() {
  if (!selectedEmbedNoteId.value) return
  
  const activeTab = tabs.value.find(t => t.id === activeTabId.value)
  const embedRecord = projectStore.addEmbedRecord({
    type: 'code',
    targetId: activeTabId.value || 'current-code',
    targetName: activeTab ? `${activeTab.name} (代码)` : '代码文件',
    noteId: selectedEmbedNoteId.value
  })

  console.log('代码已嵌入到笔记:', embedRecord)
  showEmbedDialog.value = false
  
  alert('✓ 代码已成功嵌入到笔记！\n\n在笔记中点击嵌入卡片可跳转到代码界面。')
}

// Watch for project changes
import { watch } from 'vue'
watch(() => projectStore.currentProject, (project) => {
  if (project) {
    loadFilesForEmbed()
  }
})

const {
  tabs,
  activeTabId,
  fileTree,
  initEditor,
  openFile,
  closeTab,
  switchTab,
  dispose,
  editorRef
} = useCodeEditor()

const handleFileSelect = (data: { path: string; name: string }) => {
  openFile(data.path, data.name)
}

const toggleTerminal = () => {
  showTerminal.value = !showTerminal.value
}

// Keyboard shortcuts
const handleKeydown = (e: KeyboardEvent) => {
  // Ctrl+P: Quick open
  if (e.ctrlKey && e.key === 'p') {
    e.preventDefault()
    // Could implement quick open modal here
  }
  // Ctrl+W: Close current tab
  if (e.ctrlKey && e.key === 'w' && activeTabId.value) {
    e.preventDefault()
    closeTab(activeTabId.value)
  }
  // Ctrl+Tab: Next tab
  if (e.ctrlKey && e.key === 'Tab') {
    e.preventDefault()
    if (tabs.value.length > 1) {
      const currentIndex = tabs.value.findIndex(t => t.id === activeTabId.value)
      const nextIndex = (currentIndex + 1) % tabs.value.length
      switchTab(tabs.value[nextIndex].id)
    }
  }
}

onMounted(async () => {
  await loadMonacoWorkers()
  await nextTick()
  if (editorContainer.value) {
    initEditor(editorContainer.value)
  }
  window.addEventListener('keydown', handleKeydown)
  
  // Update AI context
  const activeTab = tabs.value.find(t => t.id === activeTabId.value)
  aiStore.updateContext({
    currentTool: 'code',
    ...(activeTab && activeTab.content && {
      codeContent: activeTab.content.slice(0, 2000),
      codeLanguage: activeTab.language || 'text'
    })
  })
})

onBeforeUnmount(() => {
  dispose()
  window.removeEventListener('keydown', handleKeydown)
})

// Context menu state (simplified - no actual menu implementation)
const showTabContextMenu = (_e: MouseEvent, _tab: any) => {
  // Could implement context menu here
}

// AI解释代码
function openAiExplainDialog() {
  const activeTab = tabs.value.find(t => t.id === activeTabId.value)
  if (!activeTab || !activeTab.content) {
    aiExplainError.value = '请先打开一个有内容的文件'
    showAiExplain.value = true
    return
  }
  showAiExplain.value = true
  aiExplainResult.value = ''
  aiExplainError.value = ''
  aiExplainThinking.value = ''
}

async function doAiExplain() {
  const activeTab = tabs.value.find(t => t.id === activeTabId.value)
  if (!activeTab || !activeTab.content || aiExplaining.value) return

  aiExplaining.value = true
  aiExplainThinking.value = '正在分析代码...'
  aiExplainError.value = ''

  try {
    const prompt = `你是一位专业的编程导师和代码审查员。请详细解释以下代码：

文件: ${activeTab.name}
语言: ${activeTab.language || 'text'}

代码:
\`\`\`${activeTab.language || ''}
${activeTab.content}
\`\`\`

请提供：
1. 代码的整体功能和目的
2. 主要的代码结构和逻辑
3. 关键函数/方法的说明
4. 代码中可能存在的问题或改进建议
5. 相关的最佳实践建议

请用中文回答，解释要详细但简洁。`

    const response = await invoke<string>('ai_chat_completion', {
      messages: [{ role: 'user', content: prompt }],
      config: aiStore.config
    })

    aiExplainThinking.value = '分析完成！'
    aiExplainResult.value = response

  } catch (e: any) {
    console.error('AI解释失败:', e)
    aiExplainError.value = `分析失败: ${String(e)}`
  } finally {
    aiExplaining.value = false
  }
}

// AI优化代码
function openAiOptimizeDialog() {
  const activeTab = tabs.value.find(t => t.id === activeTabId.value)
  if (!activeTab || !activeTab.content) {
    aiOptimizeError.value = '请先打开一个有内容的文件'
    showAiOptimize.value = true
    return
  }
  showAiOptimize.value = true
  aiOptimizedCode.value = ''
  aiOptimizeError.value = ''
  aiOptimizeThinking.value = ''
}

async function doAiOptimize() {
  const activeTab = tabs.value.find(t => t.id === activeTabId.value)
  if (!activeTab || !activeTab.content || aiOptimizing.value) return

  aiOptimizing.value = true
  aiOptimizeThinking.value = '正在优化代码...'
  aiOptimizeError.value = ''

  try {
    const prompt = `你是一位专业的软件开发工程师。请优化以下代码：

文件: ${activeTab.name}
语言: ${activeTab.language || 'text'}

原始代码:
\`\`\`${activeTab.language || ''}
${activeTab.content}
\`\`\`

请进行以下优化：
1. 改善代码结构和可读性
2. 消除潜在的bug和代码异味
3. 优化性能和资源使用
4. 添加必要的注释
5. 遵循语言的最佳实践和编码规范

请直接返回优化后的完整代码，不要解释过程。`

    const response = await invoke<string>('ai_chat_completion', {
      messages: [{ role: 'user', content: prompt }],
      config: aiStore.config
    })

    aiOptimizeThinking.value = '优化完成！'
    aiOptimizedCode.value = response

  } catch (e: any) {
    console.error('AI优化失败:', e)
    aiOptimizeError.value = `优化失败: ${String(e)}`
  } finally {
    aiOptimizing.value = false
  }
}

function applyOptimizedCode() {
  const activeTab = tabs.value.find(t => t.id === activeTabId.value)
  if (!activeTab || !aiOptimizedCode.value) return

  if (editorRef.value) {
    const model = editorRef.value.getModel()
    if (model) {
      model.setValue(aiOptimizedCode.value)
      const tabIndex = tabs.value.findIndex(t => t.id === activeTabId.value)
      if (tabIndex !== -1) {
        tabs.value[tabIndex].content = aiOptimizedCode.value
        tabs.value[tabIndex].dirty = true
      }
    }
  }

  showAiOptimize.value = false
}
</script>

<style scoped>
.code-view {
  font-family: 'Inter', 'Segoe UI', system-ui, sans-serif;
}

/* Scrollbar styling */
.tree-content::-webkit-scrollbar,
.panel-content::-webkit-scrollbar {
  width: 8px;
}

.tree-content::-webkit-scrollbar-track,
.panel-content::-webkit-scrollbar-track {
  background: #1e1e1e;
}

.tree-content::-webkit-scrollbar-thumb,
.panel-content::-webkit-scrollbar-thumb {
  background: #424242;
  border-radius: 4px;
}

.tree-content::-webkit-scrollbar-thumb:hover,
.panel-content::-webkit-scrollbar-thumb:hover {
  background: #4f4f4f;
}

/* Tab scroll */
.tabs-bar::-webkit-scrollbar {
  height: 0;
}
</style>
