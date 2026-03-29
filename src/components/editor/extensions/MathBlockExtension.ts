import { Node, mergeAttributes } from '@tiptap/core'
import { VueNodeViewRenderer } from '@tiptap/vue-3'
import katex from 'katex'
import MathBlockView from '../MathBlockView.vue'

export interface MathBlockOptions {
  HTMLAttributes: Record<string, any>
}

declare module '@tiptap/core' {
  interface Commands<ReturnType> {
    mathBlock: {
      insertMathBlock: (options: { latex: string; display: 'inline' | 'block' }) => ReturnType
    }
  }
}

export const MathBlockExtension = Node.create({
  name: 'mathBlock',
  
  group: 'block',
  
  atom: true,
  
  addAttributes() {
    return {
      latex: {
        default: '',
      },
      display: {
        default: 'block',
      },
    }
  },
  
  parseHTML() {
    return [
      {
        tag: 'div[data-math-block]',
      },
    ]
  },
  
  renderHTML({ HTMLAttributes, node }) {
    const displayMode = node.attrs.display === 'block'
    let latexHtml = ''
    try {
      latexHtml = katex.renderToString(node.attrs.latex || '', {
        displayMode,
        throwOnError: false,
      })
    } catch {
      latexHtml = '<span class="text-red-500">公式渲染错误</span>'
    }
    
    return [
      'div',
      mergeAttributes(HTMLAttributes, { 
        'data-math-block': '',
        'class': displayMode ? 'math-block' : 'math-inline'
      }),
      ['span', { innerHTML: latexHtml }]
    ]
  },
  
  addNodeView() {
    return VueNodeViewRenderer(MathBlockView)
  },
  
  addCommands() {
    return {
      insertMathBlock: (options) => ({ commands }) => {
        return commands.insertContent({
          type: this.name,
          attrs: options,
        })
      },
    }
  },
})