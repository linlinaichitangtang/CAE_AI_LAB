/**
 * Parametric Store - 参数化分析状态管理
 * 管理参数定义、参数链接、预览值等
 */

import { defineStore } from 'pinia'

// ============================================================================
// Types
// ============================================================================

export interface ParamDef {
  id: string
  name: string
  value: number
  min: number
  max: number
  step: number
  unit: string
  category: string
}

export interface ParamLink {
  id: string
  sourceId: string
  targetId: string
  scale: number
  offset: number
}

export interface ParametricState {
  params: ParamDef[]
  links: ParamLink[]
  previewValues: Record<string, number>
  isPreviewMode: boolean
  previewBaseValues: Record<string, number>
}

// ============================================================================
// Helpers
// ============================================================================

function generateId(): string {
  return 'p_' + Date.now().toString(36) + '_' + Math.random().toString(36).slice(2, 8)
}

// ============================================================================
// Store
// ============================================================================

export const useParametricStore = defineStore('parametric', {
  state: (): ParametricState => ({
    params: [],
    links: [],
    previewValues: {},
    isPreviewMode: false,
    previewBaseValues: {},
  }),

  getters: {
    /** 获取所有参数 */
    allParams: (state): ParamDef[] => state.params,

    /** 按类别分组 */
    paramsByCategory(state): Record<string, ParamDef[]> {
      const groups: Record<string, ParamDef[]> = {}
      for (const p of state.params) {
        const cat = p.category || 'default'
        if (!groups[cat]) groups[cat] = []
        groups[cat].push(p)
      }
      return groups
    },

    /** 根据 ID 获取参数 */
    getParamById: (state) => (id: string): ParamDef | undefined => {
      return state.params.find(p => p.id === id)
    },

    /** 获取被链接的目标参数 */
    getLinkedTargets: (state) => (paramId: string): ParamLink[] => {
      return state.links.filter(l => l.sourceId === paramId)
    },

    /** 获取链接到某个目标的所有源 */
    getLinkedSources: (state) => (paramId: string): ParamLink[] => {
      return state.links.filter(l => l.targetId === paramId)
    },

    /** 获取所有链接关系 */
    allLinks: (state): ParamLink[] => state.links,

    /** 是否处于预览模式 */
    previewing: (state): boolean => state.isPreviewMode,
  },

  actions: {
    // ========== 参数管理 ==========

    /** 添加参数 */
    addParam(param: Partial<ParamDef> & { name: string }): ParamDef {
      const newParam: ParamDef = {
        id: generateId(),
        name: param.name,
        value: param.value ?? 0,
        min: param.min ?? 0,
        max: param.max ?? 100,
        step: param.step ?? 1,
        unit: param.unit ?? '',
        category: param.category ?? 'default',
      }
      this.params.push(newParam)
      this.previewValues[newParam.id] = newParam.value
      this.persist()
      return newParam
    },

    /** 更新参数值 */
    updateParamValue(paramId: string, value: number) {
      const param = this.params.find(p => p.id === paramId)
      if (!param) return

      // 钳位到 [min, max]
      const clamped = Math.min(Math.max(value, param.min), param.max)
      // 对齐到 step
      const aligned = Math.round(clamped / param.step) * param.step
      param.value = aligned

      if (this.isPreviewMode) {
        this.previewValues[paramId] = aligned
      }

      // 传播变更到链接的目标参数
      this.propagateChange(paramId, aligned)
      this.persist()
    },

    /** 更新参数定义 */
    updateParam(paramId: string, updates: Partial<Omit<ParamDef, 'id'>>) {
      const param = this.params.find(p => p.id === paramId)
      if (!param) return
      Object.assign(param, updates)
      this.persist()
    },

    /** 删除参数 */
    removeParam(paramId: string) {
      this.params = this.params.filter(p => p.id !== paramId)
      // 同时删除相关链接
      this.links = this.links.filter(l => l.sourceId !== paramId && l.targetId !== paramId)
      delete this.previewValues[paramId]
      this.persist()
    },

    /** 批量设置参数 */
    setParams(params: ParamDef[]) {
      this.params = params
      for (const p of params) {
        this.previewValues[p.id] = p.value
      }
      this.persist()
    },

    /** 清空所有参数 */
    clearParams() {
      this.params = []
      this.links = []
      this.previewValues = {}
      this.isPreviewMode = false
      this.previewBaseValues = {}
      this.persist()
    },

    // ========== 参数链接 (V1.1-003) ==========

    /** 创建参数链接 */
    addParamLink(sourceId: string, targetId: string, scale: number = 1, offset: number = 0): ParamLink | null {
      if (sourceId === targetId) return null

      // 检查循环引用
      if (this.wouldCreateCycle(sourceId, targetId)) return null

      const link: ParamLink = {
        id: generateId(),
        sourceId,
        targetId,
        scale,
        offset,
      }
      this.links.push(link)
      this.persist()
      return link
    },

    /** 删除参数链接 */
    removeParamLink(linkId: string) {
      this.links = this.links.filter(l => l.id !== linkId)
      this.persist()
    },

    /** 获取被链接的参数 */
    getLinkedParams(paramId: string): ParamLink[] {
      return this.links.filter(l => l.sourceId === paramId)
    },

    /** 传播参数变更到所有链接目标 */
    propagateChange(paramId: string, newValue: number) {
      const targets = this.links.filter(l => l.sourceId === paramId)
      for (const link of targets) {
        const targetValue = newValue * link.scale + link.offset
        const targetParam = this.params.find(p => p.id === link.targetId)
        if (targetParam) {
          // 钳位
          const clamped = Math.min(Math.max(targetValue, targetParam.min), targetParam.max)
          targetParam.value = clamped
          if (this.isPreviewMode) {
            this.previewValues[link.targetId] = clamped
          }
          // 递归传播
          this.propagateChange(link.targetId, clamped)
        }
      }
    },

    /** 检查添加链接是否会形成循环 */
    wouldCreateCycle(sourceId: string, targetId: string): boolean {
      // BFS: 从 targetId 出发，看能否到达 sourceId
      const visited = new Set<string>()
      const queue = [targetId]
      while (queue.length > 0) {
        const current = queue.shift()!
        if (current === sourceId) return true
        if (visited.has(current)) continue
        visited.add(current)
        const outLinks = this.links.filter(l => l.sourceId === current)
        for (const link of outLinks) {
          queue.push(link.targetId)
        }
      }
      return false
    },

    // ========== 预览模式 (V1.1-002) ==========

    /** 进入预览模式 */
    enterPreviewMode() {
      this.previewBaseValues = {}
      for (const p of this.params) {
        this.previewBaseValues[p.id] = p.value
        this.previewValues[p.id] = p.value
      }
      this.isPreviewMode = true
    },

    /** 确认预览值（退出预览模式，保留当前值） */
    confirmPreview() {
      this.isPreviewMode = false
      this.previewBaseValues = {}
      this.persist()
    },

    /** 取消预览（恢复到进入预览前的值） */
    cancelPreview() {
      for (const [id, baseValue] of Object.entries(this.previewBaseValues)) {
        const param = this.params.find(p => p.id === id)
        if (param) {
          param.value = baseValue
          this.previewValues[id] = baseValue
        }
      }
      this.isPreviewMode = false
      this.previewBaseValues = {}
    },

    /** 获取参数的预览值 */
    getPreviewValue(paramId: string): number {
      return this.previewValues[paramId] ?? this.getParamById(paramId)?.value ?? 0
    },

    /** 获取参数相对于基准的变化量 */
    getPreviewDelta(paramId: string): number {
      const base = this.previewBaseValues[paramId]
      if (base === undefined) return 0
      return (this.previewValues[paramId] ?? 0) - base
    },

    // ========== 持久化 ==========

    /** 保存到 localStorage */
    persist() {
      try {
        const data = {
          params: this.params,
          links: this.links,
        }
        localStorage.setItem('caelab_parametric', JSON.stringify(data))
      } catch {
        // localStorage 不可用时静默失败
      }
    },

    /** 从 localStorage 恢复 */
    hydrate() {
      try {
        const raw = localStorage.getItem('caelab_parametric')
        if (!raw) return
        const data = JSON.parse(raw)
        if (data.params) {
          this.params = data.params
          for (const p of this.params) {
            this.previewValues[p.id] = p.value
          }
        }
        if (data.links) {
          this.links = data.links
        }
      } catch {
        // 解析失败时静默忽略
      }
    },
  },
})
