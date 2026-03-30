<template>
  <div class="tiptap-editor-wrapper">
    <EditorToolbar 
      :active="toolbarActive"
      @bold="toggleBold"
      @italic="toggleItalic"
      @strike="toggleStrike"
      @code="toggleCode"
      @h1="toggleHeading(1)"
      @h2="toggleHeading(2)"
      @h3="toggleHeading(3)"
      @bullet-list="toggleBulletList"
      @ordered-list="toggleOrderedList"
      @blockquote="toggleBlockquote"
      @code-block="toggleCodeBlock"
      @set-link="setLink"
      @insert-image="insertImage"
      @insert-math="insertMathBlock"
      @insert-handwriting="insertHandwriting"
      @insert-embed="insertEmbed"
      @insert-note-link="showNoteLinkSelector = true"
    />
    
    <div class="editor-content-wrapper min-h-[300px] border border-gray-200 dark:border-gray-700 rounded-b bg-white dark:bg-gray-900">
      <EditorContent :editor="editor ?? undefined" class="prose dark:prose-invert max-w-none p-4" />
    </div>

    <!-- Math Editor Modal -->
    <MathEditor
      v-if="showMathEditor"
      @save="handleMathSave"
      @cancel="showMathEditor = false"
    />

    <!-- Handwriting Canvas -->
    <HandwritingCanvas
      v-if="showHandwriting"
      @save="handleHandwritingSave"
      @cancel="showHandwriting = false"
    />

    <!-- Embed Selector -->
    <EmbedSelector
      v-if="showEmbedSelector"
      @select="handleEmbedSelect"
      @cancel="showEmbedSelector = false"
    />

    <!-- Note Link Selector -->
    <div v-if="showNoteLinkSelector" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showNoteLinkSelector = false">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-[500px] max-h-[400px] flex flex-col">
        <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
          <h3 class="text-lg font-semibold text-gray-800 dark:text-white">
            📝 引用笔记
          </h3>
          <button @click="showNoteLinkSelector = false" class="text-gray-500 hover:text-gray-700 dark:text-gray-400">✕</button>
        </div>
        <div class="p-4 flex-1 overflow-auto">
          <div v-if="availableNotes.length === 0" class="text-center py-8 text-gray-500">
            暂无可引用的笔记
          </div>
          <div v-else class="space-y-2">
            <div
              v-for="note in availableNotes"
              :key="note.id"
              @click="insertNoteLink(note)"
              class="p-3 bg-gray-50 dark:bg-gray-700 rounded cursor-pointer hover:bg-blue-50 dark:hover:bg-blue-900/30 hover:border-blue-300 transition"
            >
              <div class="text-sm font-medium text-gray-800 dark:text-white">{{ note.title }}</div>
              <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">{{ note.updated }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, shallowRef, computed, onMounted, onUnmounted, watch } from 'vue'
import { Editor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'
import Link from '@tiptap/extension-link'
import Image from '@tiptap/extension-image'
import CodeBlockLowlight from '@tiptap/extension-code-block-lowlight'
import { createLowlight, common } from 'lowlight'
import katex from 'katex'
import EditorToolbar from './EditorToolbar.vue'
import MathEditor from './MathEditor.vue'
import HandwritingCanvas from './HandwritingCanvas.vue'
import EmbedSelector from './EmbedSelector.vue'
import type { EmbedItem } from '@/types'

const lowlight = createLowlight(common)

// 笔记列表项
interface NoteItem {
  id: string
  title: string
  updated: string
}

interface Props {
  modelValue?: string
  placeholder?: string
  editable?: boolean
  notes?: NoteItem[]
  currentNoteId?: string
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  placeholder: '开始创作...',
  editable: true,
  notes: () => []
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'embed', item: EmbedItem): void
  (e: 'linkNote', noteId: string, noteTitle: string): void
}>()

const editor = shallowRef<Editor | null>(null)
const showMathEditor = ref(false)
const showHandwriting = ref(false)
const showEmbedSelector = ref(false)
const showNoteLinkSelector = ref(false)

// 可引用的笔记列表（排除当前笔记）
const availableNotes = computed(() => {
  return props.notes.filter(n => n.id !== props.currentNoteId)
})

const toolbarActive = computed(() => {
  if (!editor.value) {
    return {
      bold: false, italic: false, strike: false, code: false,
      bulletList: false, orderedList: false, blockquote: false, codeBlock: false
    }
  }
  return {
    bold: editor.value.isActive('bold'),
    italic: editor.value.isActive('italic'),
    strike: editor.value.isActive('strike'),
    code: editor.value.isActive('code'),
    bulletList: editor.value.isActive('bulletList'),
    orderedList: editor.value.isActive('orderedList'),
    blockquote: editor.value.isActive('blockquote'),
    codeBlock: editor.value.isActive('codeBlock')
  }
})

const initEditor = () => {
  editor.value = new Editor({
    extensions: [
      StarterKit.configure({
        codeBlock: false
      }),
      Placeholder.configure({
        placeholder: props.placeholder
      }),
      Link.configure({
        openOnClick: false,
        HTMLAttributes: {
          class: 'text-blue-500 underline cursor-pointer'
        }
      }),
      Image.configure({
        inline: true,
        HTMLAttributes: {
          class: 'max-w-full rounded'
        }
      }),
      CodeBlockLowlight.configure({
        lowlight
      })
    ],
    content: props.modelValue,
    editable: props.editable,
    onUpdate: ({ editor }) => {
      emit('update:modelValue', editor.getHTML())
    }
  })
}

const toggleBold = () => editor.value?.chain().focus().toggleBold().run()
const toggleItalic = () => editor.value?.chain().focus().toggleItalic().run()
const toggleStrike = () => editor.value?.chain().focus().toggleStrike().run()
const toggleCode = () => editor.value?.chain().focus().toggleCode().run()
const toggleCodeBlock = () => editor.value?.chain().focus().toggleCodeBlock().run()

const toggleHeading = (level: 1 | 2 | 3) => {
  editor.value?.chain().focus().toggleHeading({ level }).run()
}

const toggleBulletList = () => editor.value?.chain().focus().toggleBulletList().run()
const toggleOrderedList = () => editor.value?.chain().focus().toggleOrderedList().run()
const toggleBlockquote = () => editor.value?.chain().focus().toggleBlockquote().run()

const setLink = () => {
  const url = window.prompt('输入链接地址:')
  if (url) {
    editor.value?.chain().focus().setLink({ href: url }).run()
  }
}

const insertImage = () => {
  const url = window.prompt('输入图片地址:')
  if (url) {
    editor.value?.chain().focus().setImage({ src: url }).run()
  }
}

const insertMathBlock = () => {
  showMathEditor.value = true
}

const insertHandwriting = () => {
  showHandwriting.value = true
}

const insertEmbed = () => {
  showEmbedSelector.value = true
}

const handleMathSave = (latex: string, display: 'inline' | 'block') => {
  try {
    const rendered = katex.renderToString(latex, {
      displayMode: display === 'block',
      throwOnError: false
    })
    const wrapper = display === 'block' 
      ? `<div class="math-block" data-latex="${escapeAttr(latex)}">${rendered}</div>`
      : `<span class="math-inline" data-latex="${escapeAttr(latex)}">${rendered}</span>`
    editor.value?.chain().focus().insertContent(wrapper).run()
  } catch (e) {
    console.error('Failed to render math:', e)
  }
  showMathEditor.value = false
}

const handleHandwritingSave = (dataUrl: string) => {
  editor.value?.chain().focus().setImage({ src: dataUrl }).run()
  showHandwriting.value = false
}

const handleEmbedSelect = (item: EmbedItem) => {
  emit('embed', item)
  showEmbedSelector.value = false
}

// 插入笔记链接
const insertNoteLink = (note: NoteItem) => {
  // 插入链接样式的内容
  const linkHtml = `<a href="#" class="note-link" data-note-id="${note.id}" onclick="event.preventDefault(); window.dispatchEvent(new CustomEvent('note-link-click', {detail: '${note.id}'}))">📝 ${note.title}</a>`
  editor.value?.chain().focus().insertContent(linkHtml).run()
  
  // 通知父组件创建链接记录
  emit('linkNote', note.id, note.title)
  
  showNoteLinkSelector.value = false
}

const escapeAttr = (text: string) => {
  return text.replace(/&/g, '&amp;').replace(/"/g, '&quot;')
}

watch(() => props.modelValue, (newVal) => {
  if (editor.value && editor.value.getHTML() !== newVal) {
    editor.value.commands.setContent(newVal)
  }
})

watch(() => props.editable, (newVal) => {
  editor.value?.setEditable(newVal)
})

onMounted(() => {
  initEditor()
})

onUnmounted(() => {
  editor.value?.destroy()
})
</script>

<style>
.tiptap-editor-wrapper :deep(.ProseMirror) {
  outline: none;
  min-height: 280px;
}

.tiptap-editor-wrapper :deep(.ProseMirror p.is-editor-empty:first-child::before) {
  content: attr(data-placeholder);
  color: #9ca3af;
  pointer-events: none;
  float: left;
  height: 0;
}

.tiptap-editor-wrapper :deep(.ProseMirror h1) {
  font-size: 2em;
  font-weight: bold;
  margin: 1em 0 0.5em;
}

.tiptap-editor-wrapper :deep(.ProseMirror h2) {
  font-size: 1.5em;
  font-weight: bold;
  margin: 1em 0 0.5em;
}

.tiptap-editor-wrapper :deep(.ProseMirror h3) {
  font-size: 1.25em;
  font-weight: bold;
  margin: 1em 0 0.5em;
}

.tiptap-editor-wrapper :deep(.ProseMirror code) {
  background: #f1f5f9;
  padding: 0.2em 0.4em;
  border-radius: 3px;
  font-family: monospace;
  font-size: 0.9em;
}

.tiptap-editor-wrapper :deep(.ProseMirror pre) {
  background: #1e1e1e;
  color: #d4d4d4;
  padding: 1em;
  border-radius: 8px;
  overflow-x: auto;
  margin: 1em 0;
}

.tiptap-editor-wrapper :deep(.ProseMirror pre code) {
  background: transparent;
  padding: 0;
  color: inherit;
}

.tiptap-editor-wrapper :deep(.ProseMirror blockquote) {
  border-left: 3px solid #3b82f6;
  padding-left: 1em;
  margin: 1em 0;
  color: #6b7280;
}

.tiptap-editor-wrapper :deep(.ProseMirror ul),
.tiptap-editor-wrapper :deep(.ProseMirror ol) {
  padding-left: 1.5em;
  margin: 1em 0;
}

.tiptap-editor-wrapper :deep(.ProseMirror li) {
  margin: 0.25em 0;
}

.tiptap-editor-wrapper :deep(.ProseMirror img) {
  max-width: 100%;
  border-radius: 8px;
  margin: 1em 0;
}

.tiptap-editor-wrapper :deep(.ProseMirror a) {
  color: #3b82f6;
  text-decoration: underline;
  cursor: pointer;
}

.tiptap-editor-wrapper :deep(.math-block) {
  background: #eff6ff;
  border: 1px solid #bfdbfe;
  border-radius: 8px;
  padding: 1em;
  margin: 1em 0;
  text-align: center;
}

.tiptap-editor-wrapper :deep(.math-inline) {
  background: #eff6ff;
  padding: 0 0.25em;
  border-radius: 4px;
}

.dark .tiptap-editor-wrapper :deep(.math-block) {
  background: #1e3a5f;
  border-color: #3b82f6;
}

.dark .tiptap-editor-wrapper :deep(.math-inline) {
  background: #1e3a5f;
}

.dark .tiptap-editor-wrapper :deep(.ProseMirror code) {
  background: #374151;
}

.dark .tiptap-editor-wrapper :deep(.ProseMirror blockquote) {
  border-color: #60a5fa;
  color: #9ca3af;
}
</style>