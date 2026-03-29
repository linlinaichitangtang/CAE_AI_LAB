import { ref, shallowRef } from 'vue'
import * as monaco from 'monaco-editor'

export interface CodeTab {
  id: string
  name: string
  path: string
  language: string
  content: string
  dirty: boolean
}

export interface FileTreeNode {
  name: string
  path: string
  type: 'file' | 'folder'
  children?: FileTreeNode[]
  expanded?: boolean
}

const defaultFiles: FileTreeNode[] = [
  {
    name: 'src',
    path: '/src',
    type: 'folder',
    expanded: true,
    children: [
      { name: 'main.ts', path: '/src/main.ts', type: 'file' },
      { name: 'App.vue', path: '/src/App.vue', type: 'file' },
      { name: 'utils', path: '/src/utils', type: 'folder', expanded: false, children: [
        { name: 'helpers.ts', path: '/src/utils/helpers.ts', type: 'file' }
      ]}
    ]
  },
  {
    name: 'docs',
    path: '/docs',
    type: 'folder',
    expanded: false,
    children: [
      { name: 'README.md', path: '/docs/README.md', type: 'file' },
      { name: 'API.md', path: '/docs/API.md', type: 'file' }
    ]
  },
  { name: 'package.json', path: '/package.json', type: 'file' },
  { name: 'tsconfig.json', path: '/tsconfig.json', type: 'file' }
]

const fileContents: Record<string, string> = {
  '/src/main.ts': `import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

const app = createApp(App)
app.use(router)
app.mount('#app')`,
  '/src/App.vue': `<template>
  <div id="app">
    <router-view />
  </div>
</template>

<script setup lang="ts">
// CAELab Application Root
</script>`,
  '/src/utils/helpers.ts': `// Utility helper functions
export function formatDate(date: Date): string {
  return new Intl.DateTimeFormat('zh-CN').format(date)
}

export function debounce<T extends (...args: any[]) => void>(
  fn: T,
  delay: number
): (...args: Parameters<T>) => void {
  let timer: ReturnType<typeof setTimeout>
  return (...args) => {
    clearTimeout(timer)
    timer = setTimeout(() => fn(...args), delay)
  }
}`,
  '/docs/README.md': `# CAELab Documentation

## Overview
CAELab is an integrated research and engineering workstation.

## Features
- Note-taking with rich text editing
- 3D modeling and visualization
- Code editing with syntax highlighting
- Simulation results embedding`,
  '/docs/API.md': `# API Reference

## Endpoints

### GET /api/projects
Returns list of all projects.

### POST /api/projects
Create a new project.`,
  '/package.json': JSON.stringify({
    name: "caelab",
    version: "0.1.0",
    description: "科研与工程创作一体化工作台",
    scripts: {
      dev: "vite",
      build: "vue-tsc --noEmit && vite build"
    }
  }, null, 2),
  '/tsconfig.json': JSON.stringify({
    compilerOptions: {
      target: "ES2021",
      module: "ESNext",
      strict: true,
      jsx: "preserve"
    }
  }, null, 2)
}

function getLanguageFromPath(path: string): string {
  const ext = path.split('.').pop()?.toLowerCase() || ''
  const map: Record<string, string> = {
    ts: 'typescript',
    tsx: 'typescript',
    js: 'javascript',
    jsx: 'javascript',
    vue: 'vue',
    py: 'python',
    rs: 'rust',
    cpp: 'cpp',
    c: 'cpp',
    h: 'cpp',
    hpp: 'cpp',
    cs: 'csharp',
    go: 'go',
    java: 'java',
    md: 'markdown',
    json: 'json',
    html: 'html',
    css: 'css',
    scss: 'scss',
    yaml: 'yaml',
    yml: 'yaml',
    xml: 'xml'
  }
  return map[ext] || 'plaintext'
}

export function useCodeEditor() {
  const editorRef = shallowRef<monaco.editor.IStandaloneCodeEditor | null>(null)
  const tabs = ref<CodeTab[]>([])
  const activeTabId = ref<string | null>(null)
  const fileTree = ref<FileTreeNode[]>(defaultFiles)

  const initEditor = (container: HTMLElement) => {
    const editor = monaco.editor.create(container, {
      value: '',
      language: 'typescript',
      theme: 'vs-dark',
      fontSize: 14,
      fontFamily: "'JetBrains Mono', 'Fira Code', Consolas, monospace",
      minimap: { enabled: true, scale: 1 },
      scrollBeyondLastLine: false,
      automaticLayout: true,
      tabSize: 2,
      insertSpaces: true,
      wordWrap: 'on',
      lineNumbers: 'on',
      renderWhitespace: 'selection',
      bracketPairColorization: { enabled: true },
      cursorBlinking: 'smooth',
      smoothScrolling: true,
      padding: { top: 16 }
    })
    editorRef.value = editor

    editor.onDidChangeModelContent(() => {
      const activeTab = tabs.value.find(t => t.id === activeTabId.value)
      if (activeTab) {
        activeTab.content = editor.getValue()
        activeTab.dirty = true
      }
    })

    return editor
  }

  const openFile = (path: string, name: string) => {
    // Check if already open
    const existing = tabs.value.find(t => t.path === path)
    if (existing) {
      activeTabId.value = existing.id
      if (editorRef.value) {
        const model = monaco.editor.createModel(
          existing.content,
          getLanguageFromPath(path)
        )
        editorRef.value.setModel(model)
      }
      return
    }

    const id = `tab-${Date.now()}`
    const content = fileContents[path] || `// ${name}\n`
    const language = getLanguageFromPath(path)

    tabs.value.push({ id, name, path, language, content, dirty: false })
    activeTabId.value = id

    if (editorRef.value) {
      const model = monaco.editor.createModel(content, language)
      editorRef.value.setModel(model)
    }
  }

  const closeTab = (id: string) => {
    const index = tabs.value.findIndex(t => t.id === id)
    if (index === -1) return

    tabs.value.splice(index, 1)

    if (activeTabId.value === id) {
      if (tabs.value.length > 0) {
        const newActive = tabs.value[Math.min(index, tabs.value.length - 1)]
        activeTabId.value = newActive.id
        if (editorRef.value) {
          const model = monaco.editor.createModel(newActive.content, newActive.language)
          editorRef.value.setModel(model)
        }
      } else {
        activeTabId.value = null
        if (editorRef.value) {
          editorRef.value.setModel(null)
        }
      }
    }
  }

  const switchTab = (id: string) => {
    const tab = tabs.value.find(t => t.id === id)
    if (!tab || !editorRef.value) return

    activeTabId.value = id
    const model = monaco.editor.createModel(tab.content, tab.language)
    editorRef.value.setModel(model)
  }

  const dispose = () => {
    editorRef.value?.dispose()
    editorRef.value = null
  }

  return {
    editorRef,
    tabs,
    activeTabId,
    fileTree,
    initEditor,
    openFile,
    closeTab,
    switchTab,
    dispose
  }
}