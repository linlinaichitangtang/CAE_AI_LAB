/**
 * Undo/Redo Store - 全局撤销/重做系统
 * 基于 Command Pattern 实现，支持最多 50 步撤销历史
 */

import { defineStore } from 'pinia'

/** 撤销命令接口 */
export interface UndoCommand {
  /** 命令唯一标识 */
  id: string
  /** 命令描述，用于 UI 显示（如 "删除笔记"） */
  description: string
  /** 执行命令 */
  execute(): void
  /** 撤销命令 */
  undo(): void
}

interface UndoState {
  undoStack: UndoCommand[]
  redoStack: UndoCommand[]
}

/** undo 栈最大步数 */
const MAX_UNDO_STACK_SIZE = 50

export const useUndoStore = defineStore('undo', {
  state: (): UndoState => ({
    undoStack: [],
    redoStack: []
  }),

  getters: {
    /** 是否可以撤销 */
    canUndo: (state): boolean => state.undoStack.length > 0,

    /** 是否可以重做 */
    canRedo: (state): boolean => state.redoStack.length > 0,

    /** 最后一个撤销命令的描述（用于 tooltip 显示） */
    lastUndoDescription: (state): string | null => {
      if (state.undoStack.length === 0) return null
      return state.undoStack[state.undoStack.length - 1].description
    },

    /** 最后一个重做命令的描述（用于 tooltip 显示） */
    lastRedoDescription: (state): string | null => {
      if (state.redoStack.length === 0) return null
      return state.redoStack[state.redoStack.length - 1].description
    }
  },

  actions: {
    /**
     * 执行命令并压入 undo 栈
     * 执行新命令时会清空 redo 栈
     */
    execute(command: UndoCommand) {
      // 执行命令
      command.execute()

      // 压入 undo 栈
      this.undoStack.push(command)

      // 超出最大步数时丢弃最早的记录
      if (this.undoStack.length > MAX_UNDO_STACK_SIZE) {
        this.undoStack.shift()
      }

      // 执行新命令时清空 redo 栈
      this.redoStack = []
    },

    /**
     * 撤销最后一个命令
     * 将命令从 undo 栈弹出，执行 undo，然后压入 redo 栈
     */
    undo() {
      if (this.undoStack.length === 0) return

      const command = this.undoStack.pop()!
      command.undo()
      this.redoStack.push(command)
    },

    /**
     * 重做最后一个撤销的命令
     * 将命令从 redo 栈弹出，执行 execute，然后压入 undo 栈
     */
    redo() {
      if (this.redoStack.length === 0) return

      const command = this.redoStack.pop()!
      command.execute()
      this.undoStack.push(command)
    },

    /** 清空所有撤销/重做栈 */
    clear() {
      this.undoStack = []
      this.redoStack = []
    }
  }
})
