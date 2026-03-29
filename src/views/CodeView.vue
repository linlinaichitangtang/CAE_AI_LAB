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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue'
import { useCodeEditor, type FileTreeNode } from '@/composables/useCodeEditor'

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

const {
  tabs,
  activeTabId,
  fileTree,
  initEditor,
  openFile,
  closeTab,
  switchTab,
  dispose
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
})

onBeforeUnmount(() => {
  dispose()
  window.removeEventListener('keydown', handleKeydown)
})

// Context menu state (simplified - no actual menu implementation)
const showTabContextMenu = (_e: MouseEvent, _tab: any) => {
  // Could implement context menu here
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