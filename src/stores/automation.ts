/**
 * Automation Store - 脚本自动化存储
 * 管理脚本录制、编辑、回放状态
 */

import { defineStore } from 'pinia'
import { nanoid } from 'nanoid'

// ============ 类型定义 ============

export interface ScriptStep {
  id: string
  type: 'mesh' | 'material' | 'boundary' | 'load' | 'solve' | 'result' | 'wait' | 'custom'
  action: string
  params: Record<string, any>
  description: string
  enabled: boolean
}

export interface ScriptLoop {
  type: 'repeat' | 'forEach' | 'while'
  paramName: string
  start?: number
  end?: number
  step?: number
  values?: string[]
}

export interface Script {
  id: string
  name: string
  description: string
  steps: ScriptStep[]
  loops: ScriptLoop[]
  variables: Record<string, string | number>
  createdAt: string
  updatedAt: string
  isTemplate: boolean
  projectId?: string
}

export interface RecordingSession {
  id: string
  startTime: string
  steps: ScriptStep[]
  isRecording: boolean
}

export interface PlaybackSession {
  scriptId: string
  status: 'idle' | 'running' | 'paused' | 'completed' | 'failed'
  currentStepIndex: number
  loopCount: number
  totalSteps: number
  startTime: string
  endTime?: string
  error?: string
  results: PlaybackResult[]
}

export interface PlaybackResult {
  stepId: string
  success: boolean
  output?: any
  error?: string
  duration: number
}

// ============ 初始状态 ============

interface AutomationState {
  // 当前脚本库
  scripts: Script[]
  // 当前编辑的脚本
  currentScript: Script | null
  // 录制会话
  recordingSession: RecordingSession | null
  // 回放会话
  playbackSession: PlaybackSession | null
  // 是否正在录制
  isRecording: boolean
  // 是否正在回放
  isPlaying: boolean
  // 内置模板
  builtInTemplates: Script[]
}

// ============ Store 定义 ============

export const useAutomationStore = defineStore('automation', {
  state: (): AutomationState => ({
    scripts: [],
    currentScript: null,
    recordingSession: null,
    playbackSession: null,
    isRecording: false,
    isPlaying: false,
    builtInTemplates: []
  }),

  getters: {
    // 获取所有脚本（非模板）
    userScripts: (state) => state.scripts.filter(s => !s.isTemplate),
    
    // 获取项目脚本
    projectScripts: (state) => (projectId: string) => 
      state.scripts.filter(s => s.projectId === projectId),
    
    // 获取模板脚本
    templates: (state) => state.scripts.filter(s => s.isTemplate),
    
    // 当前脚本的步骤数
    currentScriptStepCount: (state) => 
      state.currentScript?.steps.length || 0,
    
    // 是否可以录制
    canRecord: (state) => !state.isRecording && !state.isPlaying,
    
    // 是否可以回放
    canPlay: (state) => state.currentScript !== null && !state.isPlaying && !state.isRecording
  },

  actions: {
    // ============ 脚本管理 ============

    /** 创建新脚本 */
    createScript(name: string, description: string = ''): Script {
      const script: Script = {
        id: nanoid(),
        name,
        description,
        steps: [],
        loops: [],
        variables: {},
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        isTemplate: false
      }
      this.scripts.push(script)
      this.currentScript = script
      return script
    },

    /** 更新脚本 */
    updateScript(id: string, updates: Partial<Script>) {
      const index = this.scripts.findIndex(s => s.id === id)
      if (index !== -1) {
        this.scripts[index] = {
          ...this.scripts[index],
          ...updates,
          updatedAt: new Date().toISOString()
        }
        if (this.currentScript?.id === id) {
          this.currentScript = this.scripts[index]
        }
      }
    },

    /** 删除脚本 */
    deleteScript(id: string) {
      const index = this.scripts.findIndex(s => s.id === id)
      if (index !== -1) {
        this.scripts.splice(index, 1)
        if (this.currentScript?.id === id) {
          this.currentScript = null
        }
      }
    },

    /** 加载脚本 */
    loadScript(id: string) {
      const script = this.scripts.find(s => s.id === id)
      if (script) {
        this.currentScript = JSON.parse(JSON.stringify(script))
      }
    },

    /** 保存当前脚本 */
    saveCurrentScript() {
      if (this.currentScript) {
        this.updateScript(this.currentScript.id, this.currentScript)
      }
    },

    /** 复制脚本 */
    duplicateScript(id: string): Script | null {
      const original = this.scripts.find(s => s.id === id)
      if (!original) return null
      
      const copy: Script = {
        ...JSON.parse(JSON.stringify(original)),
        id: nanoid(),
        name: `${original.name} (副本)`,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        isTemplate: false
      }
      this.scripts.push(copy)
      return copy
    },

    // ============ 步骤管理 ============

    /** 添加步骤 */
    addStep(step: Omit<ScriptStep, 'id'>) {
      if (!this.currentScript) return
      
      const newStep: ScriptStep = {
        ...step,
        id: nanoid()
      }
      this.currentScript.steps.push(newStep)
    },

    /** 更新步骤 */
    updateStep(stepId: string, updates: Partial<ScriptStep>) {
      if (!this.currentScript) return
      
      const index = this.currentScript.steps.findIndex(s => s.id === stepId)
      if (index !== -1) {
        this.currentScript.steps[index] = {
          ...this.currentScript.steps[index],
          ...updates
        }
      }
    },

    /** 删除步骤 */
    deleteStep(stepId: string) {
      if (!this.currentScript) return
      
      const index = this.currentScript.steps.findIndex(s => s.id === stepId)
      if (index !== -1) {
        this.currentScript.steps.splice(index, 1)
      }
    },

    /** 重新排序步骤 */
    reorderSteps(fromIndex: number, toIndex: number) {
      if (!this.currentScript) return
      
      const steps = this.currentScript.steps
      const [moved] = steps.splice(fromIndex, 1)
      steps.splice(toIndex, 0, moved)
    },

    // ============ 变量管理 ============

    /** 设置变量 */
    setVariable(name: string, value: string | number) {
      if (!this.currentScript) return
      this.currentScript.variables[name] = value
    },

    /** 删除变量 */
    deleteVariable(name: string) {
      if (!this.currentScript) return
      delete this.currentScript.variables[name]
    },

    /** 解析变量（替换 ${var} 格式） */
    resolveParams(params: Record<string, any>): Record<string, any> {
      if (!this.currentScript) return params
      
      const resolved: Record<string, any> = {}
      const varPattern = /\$\{([^}]+)\}/
      
      for (const [key, value] of Object.entries(params)) {
        if (typeof value === 'string') {
          const match = value.match(varPattern)
          if (match && this.currentScript.variables[match[1]] !== undefined) {
            resolved[key] = this.currentScript.variables[match[1]]
          } else {
            resolved[key] = value
          }
        } else {
          resolved[key] = value
        }
      }
      
      return resolved
    },

    // ============ 循环管理 ============

    /** 添加循环 */
    addLoop(loop: ScriptLoop) {
      if (!this.currentScript) return
      this.currentScript.loops.push(loop)
    },

    /** ��除循环 */
    deleteLoop(index: number) {
      if (!this.currentScript) return
      this.currentScript.loops.splice(index, 1)
    },

    /** 生成循环序列 */
    generateLoopSequence(loop: ScriptLoop): any[] {
      if (loop.type === 'forEach' && loop.values) {
        return loop.values
      }
      
      const start = loop.start || 0
      const end = loop.end || 10
      const step = loop.step || 1
      const sequence: number[] = []
      
      if (loop.type === 'repeat') {
        for (let i = 0; i < (loop.start || 1); i++) {
          sequence.push(i)
        }
      } else {
        for (let i = start; i <= end; i += step) {
          sequence.push(i)
        }
      }
      
      return sequence
    },

    // ============ 录制功能 ============

    /** 开始录制 */
    startRecording() {
      this.recordingSession = {
        id: nanoid(),
        startTime: new Date().toISOString(),
        steps: [],
        isRecording: true
      }
      this.isRecording = true
    },

    /** 录制步骤 */
    recordStep(step: Omit<ScriptStep, 'id'>) {
      if (!this.recordingSession) return
      
      const newStep: ScriptStep = {
        ...step,
        id: nanoid()
      }
      this.recordingSession.steps.push(newStep)
    },

    /** 停止录制并创建脚本 */
    stopRecording(name: string, description: string = ''): Script | null {
      if (!this.recordingSession) return null
      
      const script: Script = {
        id: nanoid(),
        name,
        description,
        steps: this.recordingSession.steps,
        loops: [],
        variables: {},
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        isTemplate: false
      }
      
      this.scripts.push(script)
      this.currentScript = script
      this.recordingSession = null
      this.isRecording = false
      
      return script
    },

    /** 取消录制 */
    cancelRecording() {
      this.recordingSession = null
      this.isRecording = false
    },

    // ============ 回放功能 ============

    /** 开始回放 */
    async startPlayback(scriptId: string, loopCount: number = 1) {
      const script = this.scripts.find(s => s.id === scriptId)
      if (!script) return
      
      this.playbackSession = {
        scriptId: script.id,
        status: 'running',
        currentStepIndex: 0,
        loopCount: 0,
        totalSteps: script.steps.length,
        startTime: new Date().toISOString(),
        results: []
      }
      this.isPlaying = true
      
      // 逐个执行步骤
      try {
        for (let loop = 0; loop < loopCount; loop++) {
          this.playbackSession.loopCount = loop + 1
          
          for (let i = 0; i < script.steps.length; i++) {
            const step = script.steps[i]
            if (!step.enabled) continue
            
            this.playbackSession.currentStepIndex = i
            
            // 解析参数中的变量
            const resolvedParams = this.resolveParams(step.params)
            
            // 执行步骤（这里会调用实际的操作）
            const result = await this.executeStep(step, resolvedParams)
            
            this.playbackSession.results.push(result)
            
            // 检查是否暂停
            if (this.playbackSession.status === 'paused') {
              return
            }
          }
        }
        
        this.playbackSession.status = 'completed'
        this.playbackSession.endTime = new Date().toISOString()
      } catch (error: any) {
        this.playbackSession.status = 'failed'
        this.playbackSession.error = error.message
      }
      
      this.isPlaying = false
    },

    /** 执行单个步骤 */
    async executeStep(step: ScriptStep, params: Record<string, any>): Promise<PlaybackResult> {
      const startTime = Date.now()
      
      try {
        let output: any = null
        
        // 根据步骤类型执行不同操作
        switch (step.type) {
          case 'mesh':
            // 生成网格
            break
          case 'material':
            // 设置材料
            break
          case 'boundary':
            // 设置边界条件
            break
          case 'load':
            // 设置荷载
            break
          case 'solve':
            // 运行求解器
            break
          case 'result':
            // 获取结果
            break
          case 'wait':
            // 等待指定时间
            await new Promise(resolve => 
              setTimeout(resolve, params.duration || 1000)
            )
            break
          case 'custom':
            // 自定义操作
            break
        }
        
        return {
          stepId: step.id,
          success: true,
          output,
          duration: Date.now() - startTime
        }
      } catch (error: any) {
        return {
          stepId: step.id,
          success: false,
          error: error.message,
          duration: Date.now() - startTime
        }
      }
    },

    /** 暂停回放 */
    pausePlayback() {
      if (this.playbackSession?.status === 'running') {
        this.playbackSession.status = 'paused'
      }
    },

    /** 恢复回放 */
    resumePlayback() {
      if (this.playbackSession?.status === 'paused') {
        this.playbackSession.status = 'running'
      }
    },

    /** 停止回放 */
    stopPlayback() {
      if (this.playbackSession) {
        this.playbackSession.status = 'idle'
        this.playbackSession.endTime = new Date().toISOString()
      }
      this.isPlaying = false
    },

    /** 重置回放状态 */
    resetPlayback() {
      this.playbackSession = null
      this.isPlaying = false
    },

    // ============ 模板管理 ============

    /** 添加内置模板 */
    addBuiltInTemplate(template: Script) {
      this.builtInTemplates.push({
        ...template,
        isTemplate: true
      })
    },

    /** 初始化内置模板 */
    initBuiltInTemplates() {
      // 标准静力学分析流程
      this.addBuiltInTemplate({
        id: 'template-static',
        name: '标准静力学分析流程',
        description: '完整的静力学分析步骤：网格→材料→边界→荷载→求解→结果',
        steps: [
          {
            id: '1',
            type: 'mesh',
            action: 'generate_mesh',
            params: { elementType: 'C3D8', xDiv: 10, yDiv: 10, zDiv: 10 },
            description: '生成3D网格',
            enabled: true
          },
          {
            id: '2',
            type: 'material',
            action: 'set_material',
            params: { elastic_modulus: 210000, poisson_ratio: 0.3, density: 7.85e-9 },
            description: '设置钢材材料',
            enabled: true
          },
          {
            id: '3',
            type: 'boundary',
            action: 'set_boundary',
            params: { surfaces: ['bottom'], bc_type: 'fixed' },
            description: '设置固定边界',
            enabled: true
          },
          {
            id: '4',
            type: 'load',
            action: 'set_load',
            params: { surfaces: ['top'], magnitude: 1000, direction: 'z' },
            description: '施加竖向荷载',
            enabled: true
          },
          {
            id: '5',
            type: 'solve',
            action: 'run_solver',
            params: { analysisType: 'static' },
            description: '运行静力学求解',
            enabled: true
          },
          {
            id: '6',
            type: 'result',
            action: 'get_results',
            params: { resultTypes: ['displacement', 'stress'] },
            description: '获取位移和应力结果',
            enabled: true
          }
        ],
        loops: [],
        variables: {
          elastic_modulus: 210000,
          poisson_ratio: 0.3,
          load_magnitude: 1000
        },
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        isTemplate: true
      })

      // 参数化扫描完整流程
      this.addBuiltInTemplate({
        id: 'template-parametric',
        name: '参数化扫描完整流程',
        description: '参数化分析：定义参数→循环求解→汇总结果',
        steps: [
          {
            id: '1',
            type: 'mesh',
            action: 'generate_mesh',
            params: { elementType: 'C3D8' },
            description: '生成基础网格',
            enabled: true
          },
          {
            id: '2',
            type: 'material',
            action: 'set_material',
            params: { elastic_modulus: '${elastic_modulus}', poisson_ratio: 0.3 },
            description: '设置参数化材料',
            enabled: true
          },
          {
            id: '3',
            type: 'boundary',
            action: 'set_boundary',
            params: { surfaces: ['bottom'], bc_type: 'fixed' },
            description: '设置固定边界',
            enabled: true
          },
          {
            id: '4',
            type: 'load',
            action: 'set_load',
            params: { magnitude: '${load_magnitude}', direction: 'z' },
            description: '施加参数化荷载',
            enabled: true
          },
          {
            id: '5',
            type: 'solve',
            action: 'run_solver',
            params: { analysisType: 'static' },
            description: '运行求解',
            enabled: true
          },
          {
            id: '6',
            type: 'result',
            action: 'export_results',
            params: { format: 'csv' },
            description: '导出结果',
            enabled: true
          }
        ],
        loops: [
          {
            type: 'forEach',
            paramName: 'load_magnitude',
            start: 500,
            end: 2000,
            step: 250
          }
        ],
        variables: {
          elastic_modulus: 210000,
          load_magnitude: 1000
        },
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        isTemplate: true
      })

      // 模态分析流程
      this.addBuiltInTemplate({
        id: 'template-modal',
        name: '模态分析流程',
        description: '提取结构固有频率和振型',
        steps: [
          {
            id: '1',
            type: 'mesh',
            action: 'generate_mesh',
            params: { elementType: 'C3D8', xDiv: 8, yDiv: 8, zDiv: 8 },
            description: '生成3D网格',
            enabled: true
          },
          {
            id: '2',
            type: 'material',
            action: 'set_material',
            params: { elastic_modulus: 210000, poisson_ratio: 0.3, density: 7.85e-9 },
            description: '设置钢材材料',
            enabled: true
          },
          {
            id: '3',
            type: 'boundary',
            action: 'set_boundary',
            params: { surfaces: ['bottom'], bc_type: 'fixed' },
            description: '设置固定边界',
            enabled: true
          },
          {
            id: '4',
            type: 'solve',
            action: 'run_solver',
            params: { analysisType: 'modal', numModes: 10 },
            description: '运行模态求解',
            enabled: true
          },
          {
            id: '5',
            type: 'result',
            action: 'get_results',
            params: { resultTypes: ['frequency', 'modeShape'] },
            description: '获取频率和振型',
            enabled: true
          }
        ],
        loops: [],
        variables: {
          numModes: 10
        },
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        isTemplate: true
      })
    },

    // ============ 导入导出 ============

    /** 导出脚本为JSON */
    exportScript(id: string): string | null {
      const script = this.scripts.find(s => s.id === id)
      if (!script) return null
      return JSON.stringify(script, null, 2)
    },

    /** 从JSON导入脚本 */
    importScript(json: string): Script | null {
      try {
        const imported = JSON.parse(json) as Script
        imported.id = nanoid()
        imported.createdAt = new Date().toISOString()
        imported.updatedAt = new Date().toISOString()
        imported.isTemplate = false
        
        this.scripts.push(imported)
        return imported
      } catch (e) {
        console.error('导入脚本失败:', e)
        return null
      }
    }
  }
})