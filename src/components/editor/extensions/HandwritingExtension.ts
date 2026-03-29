import { Node, mergeAttributes } from '@tiptap/core'
import { nodeViewProps } from '@tiptap/vue-3'
import { VueNodeViewRenderer } from '@tiptap/vue-3'

// Handwriting node view component
const HandwritingNodeView = {
  props: nodeViewProps,
  template: `
    <node-view-wrapper class="handwriting-wrapper">
      <div v-if="editor.isEditable" class="handwriting-editor border border-dashed border-gray-400 dark:border-gray-600 rounded p-4 text-center">
        <img :src="node.attrs.src" class="max-w-full mx-auto rounded" style="max-height: 300px;" />
        <div class="mt-2 text-sm text-gray-500">手写笔记 · <span @click="deleteNode" class="text-red-500 cursor-pointer hover:underline">删除</span></div>
      </div>
      <div v-else class="handwriting-display">
        <img :src="node.attrs.src" class="max-w-full rounded" />
      </div>
    </node-view-wrapper>
  `,
  methods: {
    deleteNode() {
      this.editor.commands.deleteNode()
    }
  } as any
}

export interface HandwritingOptions {
  HTMLAttributes: Record<string, any>
}

export const HandwritingExtension = Node.create({
  name: 'handwriting',
  
  group: 'block',
  
  atom: true,
  
  addAttributes() {
    return {
      src: {
        default: null,
      },
      width: {
        default: '100%',
      },
    }
  },
  
  parseHTML() {
    return [
      {
        tag: 'div[data-handwriting]',
      },
    ]
  },
  
  renderHTML({ HTMLAttributes }) {
    return [
      'div',
      mergeAttributes(HTMLAttributes, { 'data-handwriting': '' }),
      [
        'img',
        {
          src: HTMLAttributes.src,
          class: 'handwriting-image w-full rounded',
          style: 'max-height: 400px; object-fit: contain;'
        }
      ]
    ]
  },
  
  addNodeView() {
    return VueNodeViewRenderer(HandwritingNodeView)
  },
})