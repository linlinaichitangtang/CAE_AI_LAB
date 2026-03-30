<template>
  <div class="editor-toolbar flex items-center gap-1 p-2 bg-gray-100 dark:bg-gray-800 rounded-t border-b border-gray-200 dark:border-gray-700 flex-wrap">
    <!-- 文本格式 -->
    <div class="toolbar-group flex items-center gap-1 border-r border-gray-300 dark:border-gray-600 pr-2">
      <button @click="emit('bold')" :class="active.bold ? 'bg-blue-500 text-white' : ''" class="toolbar-btn" title="粗体">
        <span class="font-bold">B</span>
      </button>
      <button @click="emit('italic')" :class="active.italic ? 'bg-blue-500 text-white' : ''" class="toolbar-btn" title="斜体">
        <span class="italic">I</span>
      </button>
      <button @click="emit('strike')" :class="active.strike ? 'bg-blue-500 text-white' : ''" class="toolbar-btn" title="删除线">
        <span class="line-through">S</span>
      </button>
      <button @click="emit('code')" :class="active.code ? 'bg-blue-500 text-white' : ''" class="toolbar-btn font-mono text-sm" title="行内代码">
        &lt;/&gt;
      </button>
    </div>

    <!-- 标题 -->
    <div class="toolbar-group flex items-center gap-1 border-r border-gray-300 dark:border-gray-600 pr-2">
      <button @click="emit('h1')" class="toolbar-btn text-sm font-bold" title="标题1">H1</button>
      <button @click="emit('h2')" class="toolbar-btn text-sm font-bold" title="标题2">H2</button>
      <button @click="emit('h3')" class="toolbar-btn text-sm font-bold" title="标题3">H3</button>
    </div>

    <!-- 列表 -->
    <div class="toolbar-group flex items-center gap-1 border-r border-gray-300 dark:border-gray-600 pr-2">
      <button @click="emit('bulletList')" :class="active.bulletList ? 'bg-blue-500 text-white' : ''" class="toolbar-btn" title="无序列表">
        <span class="text-lg">•</span>
      </button>
      <button @click="emit('orderedList')" :class="active.orderedList ? 'bg-blue-500 text-white' : ''" class="toolbar-btn" title="有序列表">
        <span class="text-lg">1.</span>
      </button>
      <button @click="emit('blockquote')" :class="active.blockquote ? 'bg-blue-500 text-white' : ''" class="toolbar-btn" title="引用块">
        "
      </button>
    </div>

    <!-- 代码块 -->
    <div class="toolbar-group flex items-center gap-1 border-r border-gray-300 dark:border-gray-600 pr-2">
      <button @click="emit('codeBlock')" :class="active.codeBlock ? 'bg-blue-500 text-white' : ''" class="toolbar-btn" title="代码块">
        { }
      </button>
    </div>

    <!-- 插入 -->
    <div class="toolbar-group flex items-center gap-1 border-r border-gray-300 dark:border-gray-600 pr-2">
      <button @click="emit('setLink')" class="toolbar-btn" title="插入链接">
        🔗
      </button>
      <button @click="emit('insertImage')" class="toolbar-btn" title="插入图片">
        🖼️
      </button>
      <button @click="emit('insertMath')" class="toolbar-btn" title="插入公式">
        ∑
      </button>
    </div>

    <!-- 特殊功能 -->
    <div class="toolbar-group flex items-center gap-1">
      <button @click="emit('insertHandwriting')" class="toolbar-btn" title="手写">
        ✏️
      </button>
      <button @click="emit('insertEmbed')" class="toolbar-btn" title="嵌入对象">
        📦
      </button>
      <button @click="emit('insertNoteLink')" class="toolbar-btn" title="引用笔记">
        📝
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  active?: {
    bold: boolean
    italic: boolean
    strike: boolean
    code: boolean
    bulletList: boolean
    orderedList: boolean
    blockquote: boolean
    codeBlock: boolean
  }
  hasLinks?: boolean
}

withDefaults(defineProps<Props>(), {
  active: () => ({
    bold: false,
    italic: false,
    strike: false,
    code: false,
    bulletList: false,
    orderedList: false,
    blockquote: false,
    codeBlock: false
  }),
  hasLinks: false
})

const emit = defineEmits<{
  (e: 'bold'): void
  (e: 'italic'): void
  (e: 'strike'): void
  (e: 'code'): void
  (e: 'h1'): void
  (e: 'h2'): void
  (e: 'h3'): void
  (e: 'bulletList'): void
  (e: 'orderedList'): void
  (e: 'blockquote'): void
  (e: 'codeBlock'): void
  (e: 'setLink'): void
  (e: 'insertImage'): void
  (e: 'insertMath'): void
  (e: 'insertHandwriting'): void
  (e: 'insertEmbed'): void
  (e: 'insertNoteLink'): void
}>()
</script>

<style scoped>
.toolbar-btn {
  @apply px-2 py-1 rounded text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors text-sm;
  min-width: 28px;
}

.toolbar-group {
  padding-right: 8px;
}
</style>