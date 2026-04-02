<template>
  <div class="lite-code-editor" ref="editorContainer">
    <div class="editor-toolbar">
      <button @click="undo" title="撤销">&#x21A9;</button>
      <button @click="redo" title="重做">&#x21AA;</button>
      <span class="separator"></span>
      <button @click="formatCode" title="格式化">&#x2261;</button>
      <button @click="toggleComment" title="注释">#</button>
      <span class="separator"></span>
      <select v-model="language" @change="changeLanguage" class="lang-select">
        <option value="python">Python</option>
        <option value="javascript">JavaScript</option>
        <option value="typescript">TypeScript</option>
        <option value="json">JSON</option>
        <option value="xml">XML</option>
        <option value="plaintext">Text</option>
      </select>
      <span class="flex-1"></span>
      <span class="line-info">{{ cursorLine }}:{{ cursorCol }}</span>
    </div>
    <div class="editor-content" ref="editorRef"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { EditorView, basicSetup } from 'codemirror'
import { python } from '@codemirror/lang-python'
import { javascript } from '@codemirror/lang-javascript'
import { json } from '@codemirror/lang-json'
import { xml } from '@codemirror/lang-xml'
import { oneDark } from '@codemirror/theme-one-dark'
import { EditorState, Compartment } from '@codemirror/state'
import { undo as cmUndo, redo as cmRedo } from '@codemirror/commands'

const props = withDefaults(defineProps<{
  modelValue?: string
  language?: string
  readOnly?: boolean
  lineNumbers?: boolean
  height?: string
}>(), {
  modelValue: '',
  language: 'python',
  readOnly: false,
  lineNumbers: true,
  height: '100%',
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
  'save': [value: string]
}>()

const editorContainer = ref<HTMLElement>()
const editorRef = ref<HTMLElement>()
const language = ref(props.language)
const cursorLine = ref(1)
const cursorCol = ref(1)

let view: EditorView | null = null
const languageCompartment = new Compartment()
const readOnlyCompartment = new Compartment()

function getLanguageExtension() {
  switch (language.value) {
    case 'python': return python()
    case 'javascript': return javascript()
    case 'typescript': return javascript({ typescript: true })
    case 'json': return json()
    case 'xml': return xml()
    default: return []
  }
}

onMounted(() => {
  if (!editorRef.value) return

  const state = EditorState.create({
    doc: props.modelValue,
    extensions: [
      basicSetup,
      languageCompartment.of(getLanguageExtension()),
      oneDark,
      readOnlyCompartment.of(props.readOnly ? [EditorState.readOnly.of(true)] : []),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          emit('update:modelValue', update.state.doc.toString())
        }
        const pos = update.state.selection.main.head
        const line = update.state.doc.lineAt(pos)
        cursorLine.value = line.number
        cursorCol.value = pos - line.from + 1
      }),
      EditorView.theme({
        '&': { fontSize: '14px', height: props.height },
        '.cm-scroller': { overflow: 'auto' },
        '.cm-content': { fontFamily: "'Fira Code', 'Cascadia Code', 'JetBrains Mono', monospace" },
      }),
      // 移动端触控优化
      EditorView.theme({
        '.cm-gutters': { minWidth: '30px' },
        '.cm-lineNumbers': { fontSize: '11px' },
      }),
    ],
  })

  view = new EditorView({
    state,
    parent: editorRef.value,
  })
})

onUnmounted(() => {
  view?.destroy()
})

watch(() => props.modelValue, (val) => {
  if (view && val !== view.state.doc.toString()) {
    view.dispatch({
      changes: { from: 0, to: view.state.doc.length, insert: val },
    })
  }
})

watch(() => props.readOnly, (val) => {
  if (!view) return
  view.dispatch({
    effects: readOnlyCompartment.reconfigure(
      val ? [EditorState.readOnly.of(true)] : []
    ),
  })
})

function changeLanguage() {
  if (!view) return
  view.dispatch({
    effects: languageCompartment.reconfigure(getLanguageExtension()),
  })
}

function undo() {
  if (view) cmUndo(view)
}

function redo() {
  if (view) cmRedo(view)
}

function formatCode() {
  if (!view) return
  const code = view.state.doc.toString()
  const formatted = code.replace(/\n{3,}/g, '\n\n').trim()
  view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: formatted } })
}

function toggleComment() {
  if (!view) return
  const { from, to } = view.state.selection.main
  const line = view.state.doc.lineAt(from)
  const text = line.text
  const commentChar = language.value === 'python' ? '# ' : '// '
  const toggled = text.startsWith(commentChar) ? text.slice(commentChar.length) : commentChar + text
  view.dispatch({ changes: { from: line.from, to: line.to, insert: toggled } })
}

defineExpose({ undo, redo, formatCode, toggleComment })
</script>

<style scoped>
.lite-code-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  border: 1px solid var(--border-color, #e5e7eb);
  border-radius: 8px;
  overflow: hidden;
  background: #282c34;
}
.editor-toolbar {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  background: #21252b;
  border-bottom: 1px solid #3a3f4b;
  min-height: 36px;
}
.editor-toolbar button {
  padding: 4px 8px;
  background: none;
  border: none;
  color: #abb2bf;
  cursor: pointer;
  border-radius: 4px;
  font-size: 14px;
  min-width: 32px;
  min-height: 32px;
  -webkit-tap-highlight-color: transparent;
}
.editor-toolbar button:active {
  background: #4f5b66;
}
@media (hover: hover) {
  .editor-toolbar button:hover {
    background: #3a3f4b;
  }
}
.separator {
  width: 1px;
  height: 20px;
  background: #3a3f4b;
  margin: 0 4px;
}
.lang-select {
  background: #3a3f4b;
  color: #abb2bf;
  border: none;
  border-radius: 4px;
  padding: 4px 8px;
  font-size: 12px;
  min-height: 32px;
  -webkit-appearance: none;
  appearance: none;
}
.line-info {
  color: #636d83;
  font-size: 11px;
  font-family: monospace;
}
.editor-content {
  flex: 1;
  overflow: hidden;
}
.flex-1 {
  flex: 1;
}

/* 移动端适配 */
@media (max-width: 768px) {
  .editor-toolbar {
    padding: 2px 4px;
    gap: 2px;
    min-height: 32px;
  }
  .editor-toolbar button {
    padding: 4px 6px;
    min-width: 28px;
    min-height: 28px;
    font-size: 13px;
  }
  .lang-select {
    font-size: 11px;
    padding: 2px 4px;
    min-height: 28px;
  }
}
</style>
