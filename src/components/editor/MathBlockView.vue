<template>
  <node-view-wrapper class="math-block-wrapper">
    <div 
      v-if="editor?.isEditable"
      class="math-block-editor flex items-center gap-2 p-3 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded"
    >
      <span class="text-blue-500">∑</span>
      <span class="flex-1 text-sm text-gray-600 dark:text-gray-400 font-mono truncate">
        {{ node.attrs.latex || '点击编辑公式' }}
      </span>
      <button @click="startEdit" class="px-2 py-1 text-sm bg-blue-500 text-white rounded hover:bg-blue-600">
        编辑
      </button>
    </div>
    <div 
      v-else 
      class="math-block-rendered text-center py-2"
      v-html="renderedContent"
    ></div>
  </node-view-wrapper>
</template>

<script setup lang="ts">
import { NodeViewWrapper } from '@tiptap/vue-3'
import { NodeViewProps } from '@tiptap/core'
import katex from 'katex'
import { computed } from 'vue'

const props = defineProps<NodeViewProps>()

const renderedContent = computed(() => {
  if (!props.node.attrs.latex) return ''
  try {
    return katex.renderToString(props.node.attrs.latex, {
      displayMode: props.node.attrs.display === 'block',
      throwOnError: false
    })
  } catch {
    return '<span class="text-red-500">公式渲染错误</span>'
  }
})

const startEdit = () => {
  console.log('Open math editor for:', props.node.attrs.latex)
}
</script>

<style scoped>
.math-block-wrapper {
  margin: 1rem 0;
}

.math-block-rendered :deep(.katex) {
  font-size: 1.2em;
}
</style>