/**
 * Project Store - 全局项目数据存储
 * 实现多工具之间的数据共享和联动：
 * - 3D建模生成网格 → 传递给仿真设置
 * - 仿真设置生成INP → 传递给求解器
 * - 求解器结果 → 传回3D视图显示云图
 * - 所有工具都可以引用当前项目的数据
 */

import { defineStore } from 'pinia'
import type { Project } from '@/api/types'
import type { Node, Element, MeshApiResult, FixedBc, PointLoad, UniformLoad, ResultSet } from '@/api/cae'
import type { BucklingResult } from '@/api/cae'
import type { TransientResults } from '@/api/transient_dynamics'

interface EmbedRecord {
  id: string
  type: 'model' | 'code' | 'simulation'
  targetId: string
  targetName: string
  noteId: string
  createdAt: string
}

interface ProjectState {
  // 当前打开的项目
  currentProject: Project | null
  // 当前项目的网格数据（建模生成）
  currentMesh: {
    nodes: Node[]
    elements: Element[]
  } | null
  // 当前设置的边界条件
  boundaryConditions: {
    fixedBcs: FixedBc[]
    pointLoads: PointLoad[]
    uniformLoads: UniformLoad[]
  }
  // 最近一次求解结果
  lastResult: ResultSet | null
  // 屈曲分析结果
  lastBucklingResult: BucklingResult | null
  // 动力学瞬态分析结果
  lastTransientResult: TransientResults | null
  // 当前活跃的工具视图
  activeTool: 'notes' | 'modeling' | 'code' | 'simulation'
  // 嵌入记录列表
  embedRecords: EmbedRecord[]
  // 当前打开的笔记ID（用于嵌入操作）
  currentNoteId: string | null
}

export const useProjectStore = defineStore('project', {
  state: (): ProjectState => ({
    currentProject: null,
    currentMesh: null,
    boundaryConditions: {
      fixedBcs: [],
      pointLoads: [],
      uniformLoads: []
    },
    lastResult: null,
    lastBucklingResult: null,
    lastTransientResult: null,
    activeTool: 'notes',
    embedRecords: [],
    currentNoteId: null
  }),

  getters: {
    hasMesh: (state) => state.currentMesh !== null && state.currentMesh.nodes.length > 0,
    hasBoundaryConditions: (state) => 
      state.boundaryConditions.fixedBcs.length > 0 || 
      state.boundaryConditions.pointLoads.length > 0 ||
      state.boundaryConditions.uniformLoads.length > 0,
    hasResult: (state) => state.lastResult !== null,
    hasBucklingResult: (state) => state.lastBucklingResult !== null,
    hasTransientResult: (state) => state.lastTransientResult !== null
  },

  actions: {
    /** 设置当前项目 */
    setCurrentProject(project: Project) {
      this.currentProject = project
    },

    /** 清除当前项目 */
    clearCurrentProject() {
      this.currentProject = null
      this.clearMesh()
      this.clearBoundaryConditions()
      this.clearResult()
    },

    /** 更新网格数据（建模完成后调用） */
    setMesh(mesh: MeshApiResult) {
      this.currentMesh = {
        nodes: mesh.nodes,
        elements: mesh.elements
      }
    },

    /** 清除网格数据 */
    clearMesh() {
      this.currentMesh = null
    },

    /** 添加固定边界条件 */
    addFixedBc(bc: FixedBc) {
      this.boundaryConditions.fixedBcs.push(bc)
    },

    /** 更新固定边界条件 */
    updateFixedBc(index: number, bc: FixedBc) {
      if (index >= 0 && index < this.boundaryConditions.fixedBcs.length) {
        this.boundaryConditions.fixedBcs[index] = bc
      }
    },

    /** 删除固定边界条件 */
    removeFixedBc(index: number) {
      if (index >= 0 && index < this.boundaryConditions.fixedBcs.length) {
        this.boundaryConditions.fixedBcs.splice(index, 1)
      }
    },

    /** 添加点荷载 */
    addPointLoad(load: PointLoad) {
      this.boundaryConditions.pointLoads.push(load)
    },

    /** 删除点荷载 */
    removePointLoad(index: number) {
      if (index >= 0 && index < this.boundaryConditions.pointLoads.length) {
        this.boundaryConditions.pointLoads.splice(index, 1)
      }
    },

    /** 添加均布荷载 */
    addUniformLoad(load: UniformLoad) {
      this.boundaryConditions.uniformLoads.push(load)
    },

    /** 删除均布荷载 */
    removeUniformLoad(index: number) {
      if (index >= 0 && index < this.boundaryConditions.uniformLoads.length) {
        this.boundaryConditions.uniformLoads.splice(index, 1)
      }
    },

    /** 清空所有边界条件 */
    clearBoundaryConditions() {
      this.boundaryConditions.fixedBcs = []
      this.boundaryConditions.pointLoads = []
      this.boundaryConditions.uniformLoads = []
    },

    /** 设置求解结果 */
    setResult(result: ResultSet) {
      this.lastResult = result
    },

    /** 清除求解结果 */
    clearResult() {
      this.lastResult = null
    },

    /** 设置屈曲分析结果 */
    setBucklingResult(result: BucklingResult) {
      this.lastBucklingResult = result
    },

    /** 清除屈曲分析结果 */
    clearBucklingResult() {
      this.lastBucklingResult = null
    },

    /** 设置动力学瞬态分析结果 */
    setTransientResult(result: TransientResults) {
      this.lastTransientResult = result
    },

    /** 清除动力学瞬态分析结果 */
    clearTransientResult() {
      this.lastTransientResult = null
    },

    /** 设置当前活跃工具 */
    setActiveTool(tool: ProjectState['activeTool']) {
      this.activeTool = tool
    },

    /** 跳转到仿真视图并带入网格数据 */
    jumpToSimulationFromModeling() {
      this.activeTool = 'simulation'
      // 网格数据已经在store里了，仿真视图直接读取
    },

    /** 跳转到3D视图并带入结果数据 */
    jumpToModelingFromSimulation() {
      this.activeTool = 'modeling'
      // 结果数据已经在store里了，3D视图直接读取显示云图
    },

    /** 设置当前笔记ID（用于嵌入操作） */
    setCurrentNoteId(noteId: string | null) {
      this.currentNoteId = noteId
    },

    /** 嵌入对象到笔记 */
    addEmbedRecord(record: Omit<EmbedRecord, 'id' | 'createdAt'>) {
      const newRecord: EmbedRecord = {
        ...record,
        id: `embed-${Date.now()}`,
        createdAt: new Date().toISOString()
      }
      this.embedRecords.push(newRecord)
      return newRecord
    },

    /** 获取指定笔记的所有嵌入记录 */
    getEmbedRecordsByNoteId(noteId: string): EmbedRecord[] {
      return this.embedRecords.filter(r => r.noteId === noteId)
    },

    /** 获取指定类型的所有嵌入记录 */
    getEmbedRecordsByType(type: EmbedRecord['type']): EmbedRecord[] {
      return this.embedRecords.filter(r => r.type === type)
    },

    /** 删除嵌入记录 */
    removeEmbedRecord(id: string) {
      const index = this.embedRecords.findIndex(r => r.id === id)
      if (index !== -1) {
        this.embedRecords.splice(index, 1)
      }
    },

    /** 清除指定笔记的所有嵌入记录 */
    clearEmbedRecordsByNoteId(noteId: string) {
      this.embedRecords = this.embedRecords.filter(r => r.noteId !== noteId)
    },

    /** 获取嵌入对象详情（从store中查找） */
    getEmbedTargetInfo(type: EmbedRecord['type'], _targetId: string) {
      switch (type) {
        case 'model':
          // 从建模数据中获取
          return {
            name: this.currentMesh ? '当前网格模型' : '未知模型',
            hasData: this.currentMesh !== null
          }
        case 'simulation':
          return {
            name: this.lastResult ? '仿真结果' : '无结果',
            hasData: this.lastResult !== null,
            stats: null
          }
        case 'code':
          return {
            name: '代码文件',
            hasData: false
          }
        default:
          return { name: '未知', hasData: false }
      }
    }
  }
})
