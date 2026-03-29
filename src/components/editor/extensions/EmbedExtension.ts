import { Node, mergeAttributes } from '@tiptap/core'
import type { EmbedItem } from '@/types'

export interface EmbedOptions {
  HTMLAttributes: Record<string, any>
}

declare module '@tiptap/core' {
  interface Commands<ReturnType> {
    embed: {
      insertEmbed: (item: EmbedItem) => ReturnType
    }
  }
}

export const EmbedExtension = Node.create({
  name: 'embed',
  
  group: 'block',
  
  atom: true,
  
  addAttributes() {
    return {
      itemId: {
        default: null,
      },
      itemType: {
        default: 'model', // 'model' | 'code' | 'simulation'
      },
      itemName: {
        default: '',
      },
      itemData: {
        default: null, // Additional data for rendering
      }
    }
  },
  
  parseHTML() {
    return [
      {
        tag: 'div[data-embed]',
      },
    ]
  },
  
  renderHTML({ HTMLAttributes }) {
    const icon = getEmbedIcon(HTMLAttributes.itemType)
    const title = HTMLAttributes.itemName || getEmbedTitle(HTMLAttributes.itemType)
    
    return [
      'div',
      mergeAttributes(HTMLAttributes, { 'data-embed': '' }),
      [
        'div',
        {
          class: 'embed-block flex items-center gap-3 p-4 bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg hover:border-blue-300 dark:hover:border-blue-600 cursor-pointer transition-colors'
        },
        ['span', { class: 'text-3xl' }, icon],
        ['div', { class: 'flex-1' },
          ['div', { class: 'font-medium text-gray-800 dark:text-white' }, title],
          ['div', { class: 'text-sm text-gray-500 dark:text-gray-400' }, getEmbedDesc(HTMLAttributes.itemType)]
        ],
        ['span', { class: 'text-gray-400' }, '→']
      ]
    ]
  },
  
  addCommands() {
    return {
      insertEmbed: (item) => ({ commands }) => {
        return commands.insertContent({
          type: this.name,
          attrs: {
            itemId: item.id,
            itemType: item.type,
            itemName: item.name,
            itemData: item.data || null
          },
        })
      },
    }
  },
})

function getEmbedIcon(type: string): string {
  switch (type) {
    case 'model': return '📐'
    case 'code': return '📄'
    case 'simulation': return '📊'
    default: return '📦'
  }
}

function getEmbedTitle(type: string): string {
  switch (type) {
    case 'model': return '3D模型'
    case 'code': return '代码块'
    case 'simulation': return '仿真结果'
    default: return '嵌入对象'
  }
}

function getEmbedDesc(type: string): string {
  switch (type) {
    case 'model': return '点击跳转到建模模块'
    case 'code': return '点击跳转到代码模块'
    case 'simulation': return '点击跳转到仿真结果'
    default: return ''
  }
}