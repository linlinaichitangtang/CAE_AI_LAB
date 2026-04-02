/**
 * Automation Store - 脚本自动化存储
 * 管理脚本录制、编辑、回放状态 + 仿真队列管理
 */

import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
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

// ============ 仿真队列类型 ============

export type JobStatus = 'pending' | 'running' | 'completed' | 'failed' | 'cancelled'

export interface SimulationJob {
  id: string
  name: string
  projectId: string
  scriptId?: string
  status: JobStatus
  progress: number
  startTime?: number
  endTime?: number
  estimatedTime?: number
  result?: any
  error?: string
  priority: number
}

export interface QueueStats {
  total: number
  completed: number
  failed: number
  pending: number
  running: number
  cancelled: number
  estimatedTimeRemaining: number
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
  // 仿真队列
  simulationQueue: SimulationJob[]
  // 队列是否正在自动运行
  isQueueRunning: boolean
  // 当前正在运行的 job ID
  currentJobId: string | null
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
    builtInTemplates: [],
    simulationQueue: [],
    isQueueRunning: false,
    currentJobId: null,
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
    canPlay: (state) => state.currentScript !== null && !state.isPlaying && !state.isRecording,

    // 队列中待处理的任务
    pendingJobs: (state) => state.simulationQueue.filter(j => j.status === 'pending'),

    // 队列中正在运行的任务
    runningJobs: (state) => state.simulationQueue.filter(j => j.status === 'running'),

    // 队列中已完成的任务
    completedJobs: (state) => state.simulationQueue.filter(j => j.status === 'completed'),

    // 队列中失败的任务
    failedJobs: (state) => state.simulationQueue.filter(j => j.status === 'failed'),

    // 当前正在运行的 job
    currentJob: (state) =>
      state.simulationQueue.find(j => j.id === state.currentJobId) || null,

    // 队列统计
    queueStats: (state): QueueStats => {
      const jobs = state.simulationQueue
      const completed = jobs.filter(j => j.status === 'completed')
      const failed = jobs.filter(j => j.status === 'failed')
      const pending = jobs.filter(j => j.status === 'pending')
      const running = jobs.filter(j => j.status === 'running')
      const cancelled = jobs.filter(j => j.status === 'cancelled')

      // 估算剩余时间
      const completedTimes = completed
        .filter(j => j.startTime && j.endTime)
        .map(j => (j.endTime! - j.startTime!) / 1000)
      const avgTime = completedTimes.length > 0
        ? completedTimes.reduce((a, b) => a + b, 0) / completedTimes.length
        : 30 // 默认 30 秒
      const remaining = (pending.length + running.length) * avgTime * 1000

      return {
        total: jobs.length,
        completed: completed.length,
        failed: failed.length,
        pending: pending.length,
        running: running.length,
        cancelled: cancelled.length,
        estimatedTimeRemaining: remaining,
      }
    },
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

    /** 删除循环 */
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

            // 执行步骤
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
          case 'mesh': {
            // 调用网格生成 API
            output = await invoke('generate_mesh', {
              projectId: params.projectId,
              elementType: params.elementType || 'C3D8',
              xDiv: params.xDiv || 10,
              yDiv: params.yDiv || 10,
              zDiv: params.zDiv || 10,
              xMin: params.xMin ?? 0,
              xMax: params.xMax ?? 1,
              yMin: params.yMin ?? 0,
              yMax: params.yMax ?? 1,
              zMin: params.zMin ?? 0,
              zMax: params.zMax ?? 1,
            })
            break
          }
          case 'material': {
            // 调用材料设置 API
            output = await invoke('set_material', {
              projectId: params.projectId,
              name: params.name || 'Default',
              material: {
                youngs_modulus: params.elastic_modulus || params.youngs_modulus || 210000,
                poisson_ratio: params.poisson_ratio || 0.3,
                density: params.density || 7.85e-9,
                thermal_conductivity: params.thermal_conductivity,
                thermal_expansion: params.thermal_expansion,
                yield_strength: params.yield_strength,
                ultimate_strength: params.ultimate_strength,
              },
            })
            break
          }
          case 'boundary': {
            // 调用边界条件 API
            output = await invoke('add_boundary_condition', {
              projectId: params.projectId,
              bcType: params.bc_type || 'fixed',
              surfaces: params.surfaces || [],
              nodeIds: params.node_ids || [],
              ux: params.ux,
              uy: params.uy,
              uz: params.uz,
            })
            break
          }
          case 'load': {
            // 调用荷载 API
            output = await invoke('add_load', {
              projectId: params.projectId,
              loadType: params.load_type || 'force',
              surfaces: params.surfaces || [],
              nodeIds: params.node_ids || [],
              magnitude: params.magnitude || 0,
              direction: params.direction || 'z',
              values: params.values || [],
            })
            break
          }
          case 'solve': {
            // 调用求解 API
            output = await invoke('run_simulation', {
              projectId: params.projectId,
              analysisType: params.analysisType || 'static',
              numModes: params.numModes || 10,
              maxIterations: params.maxIterations || 100,
              convergenceTolerance: params.convergenceTolerance || 1e-6,
            })
            break
          }
          case 'result': {
            // 调用结果获取 API
            output = await invoke('get_simulation_result', {
              projectId: params.projectId,
              resultTypes: params.resultTypes || ['displacement', 'stress'],
              format: params.format || 'json',
              exportPath: params.exportPath || null,
            })
            break
          }
          case 'wait': {
            // 等待指定时间
            await new Promise(resolve =>
              setTimeout(resolve, params.duration || 1000)
            )
            output = { waited: params.duration || 1000 }
            break
          }
          case 'custom': {
            // 执行自定义代码
            output = await invoke('execute_code', {
              language: params.language || 'python',
              code: params.code || '',
              workingDir: params.workingDir || null,
            })
            break
          }
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
          error: error.message || String(error),
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

    // ============ 仿真队列管理 ============

    /** 添加任务到队列 */
    addToQueue(job: Omit<SimulationJob, 'id' | 'status' | 'progress'>): SimulationJob {
      const newJob: SimulationJob = {
        ...job,
        id: nanoid(),
        status: 'pending',
        progress: 0,
      }
      this.simulationQueue.push(newJob)
      return newJob
    },

    /** 从队列移除任务 */
    removeFromQueue(jobId: string) {
      const job = this.simulationQueue.find(j => j.id === jobId)
      if (job && job.status === 'running') {
        throw new Error('Cannot remove a running job from the queue')
      }
      const index = this.simulationQueue.findIndex(j => j.id === jobId)
      if (index !== -1) {
        this.simulationQueue.splice(index, 1)
      }
    },

    /** 拖拽重排队列 */
    reorderQueue(jobIds: string[]) {
      const jobMap = new Map(this.simulationQueue.map(j => [j.id, j]))
      const reordered: SimulationJob[] = []
      for (const id of jobIds) {
        const job = jobMap.get(id)
        if (job) reordered.push(job)
      }
      // 保留不在 jobIds 中的任务（如正在运行的）
      for (const job of this.simulationQueue) {
        if (!jobIds.includes(job.id)) {
          reordered.push(job)
        }
      }
      this.simulationQueue = reordered
    },

    /** 上移任务 */
    moveJobUp(jobId: string) {
      const index = this.simulationQueue.findIndex(j => j.id === jobId)
      if (index > 0) {
        const [job] = this.simulationQueue.splice(index, 1)
        this.simulationQueue.splice(index - 1, 0, job)
      }
    },

    /** 下移任务 */
    moveJobDown(jobId: string) {
      const index = this.simulationQueue.findIndex(j => j.id === jobId)
      if (index < this.simulationQueue.length - 1) {
        const [job] = this.simulationQueue.splice(index, 1)
        this.simulationQueue.splice(index + 1, 0, job)
      }
    },

    /** 取消任务 */
    cancelJob(jobId: string) {
      const job = this.simulationQueue.find(j => j.id === jobId)
      if (job) {
        if (job.status === 'running') {
          job.status = 'cancelled'
          job.endTime = Date.now()
          this.currentJobId = null
          // 如果队列正在运行，继续下一个
          if (this.isQueueRunning) {
            this.startNextJob()
          }
        } else if (job.status === 'pending') {
          job.status = 'cancelled'
        }
      }
    },

    /** 重试失败的任务 */
    retryJob(jobId: string) {
      const job = this.simulationQueue.find(j => j.id === jobId)
      if (job && (job.status === 'failed' || job.status === 'cancelled')) {
        job.status = 'pending'
        job.progress = 0
        job.error = undefined
        job.result = undefined
        job.startTime = undefined
        job.endTime = undefined
        job.estimatedTime = undefined
      }
    },

    /** 清除已完成的任务 */
    clearCompletedJobs() {
      this.simulationQueue = this.simulationQueue.filter(
        j => j.status !== 'completed' && j.status !== 'cancelled'
      )
    },

    /** 清空整个队列 */
    clearQueue() {
      // 只保留正在运行的任务
      this.simulationQueue = this.simulationQueue.filter(j => j.status === 'running')
    },

    /** 开始自动运行队列 */
    async startQueue() {
      if (this.isQueueRunning) return
      this.isQueueRunning = true
      await this.startNextJob()
    },

    /** 暂停队列 */
    pauseQueue() {
      this.isQueueRunning = false
    },

    /** 停止队列 */
    stopQueue() {
      this.isQueueRunning = false
      // 取消所有 pending 任务
      for (const job of this.simulationQueue) {
        if (job.status === 'pending') {
          job.status = 'cancelled'
        }
      }
    },

    /** 执行下一个队列任务 */
    async startNextJob() {
      if (!this.isQueueRunning) return

      // 找到下一个 pending 任务
      const nextJob = this.simulationQueue.find(j => j.status === 'pending')
      if (!nextJob) {
        // 没有更多任务，停止队列
        this.isQueueRunning = false
        this.currentJobId = null
        // 发送完成通知
        this.notifyQueueComplete()
        return
      }

      // 更新任务状态
      nextJob.status = 'running'
      nextJob.startTime = Date.now()
      nextJob.progress = 0
      this.currentJobId = nextJob.id

      try {
        // 如果任务关联了脚本，执行脚本
        if (nextJob.scriptId) {
          const script = this.scripts.find(s => s.id === nextJob.scriptId)
          if (script) {
            for (let i = 0; i < script.steps.length; i++) {
              if (!this.isQueueRunning) break

              const step = script.steps[i]
              if (!step.enabled) continue

              // 更新进度
              nextJob.progress = Math.round(((i + 1) / script.steps.length) * 100)

              const resolvedParams = this.resolveParams({
                ...step.params,
                projectId: nextJob.projectId,
              })

              const result = await this.executeStep(step, resolvedParams)

              if (!result.success) {
                throw new Error(result.error || 'Step execution failed')
              }
            }
          }
        } else {
          // 无脚本关联，直接运行仿真
          const result = await invoke('run_simulation', {
            projectId: nextJob.projectId,
            analysisType: 'static',
          })
          nextJob.result = result
          nextJob.progress = 100
        }

        // 任务完成
        nextJob.status = 'completed'
        nextJob.endTime = Date.now()
        nextJob.progress = 100

        // 发送桌面通知
        this.notifyJobComplete(nextJob)

        // 自动开始下一个
        if (this.isQueueRunning) {
          await this.startNextJob()
        }
      } catch (error: any) {
        nextJob.status = 'failed'
        nextJob.endTime = Date.now()
        nextJob.error = error.message || String(error)

        // 发送失败通知
        this.notifyJobFailed(nextJob)

        // 即使失败也继续下一个
        if (this.isQueueRunning) {
          await this.startNextJob()
        }
      } finally {
        this.currentJobId = null
      }
    },

    /** 发送桌面通知 — 任务完成 */
    notifyJobComplete(job: SimulationJob) {
      try {
        if ('Notification' in window && Notification.permission === 'granted') {
          new Notification('CAELab - 任务完成', {
            body: `"${job.name}" 已成功完成`,
            icon: '/icon.png',
          })
        }
      } catch {
        // 通知不可用时静默忽略
      }
    },

    /** 发送桌面通知 — 任务失败 */
    notifyJobFailed(job: SimulationJob) {
      try {
        if ('Notification' in window && Notification.permission === 'granted') {
          new Notification('CAELab - 任务失败', {
            body: `"${job.name}" 运行失败: ${job.error || '未知错误'}`,
            icon: '/icon.png',
          })
        }
      } catch {
        // 通知不可用时静默忽略
      }
    },

    /** 发送桌面通知 — 队列全部完成 */
    notifyQueueComplete() {
      try {
        if ('Notification' in window && Notification.permission === 'granted') {
          const stats = this.queueStats
          new Notification('CAELab - 队列完成', {
            body: `所有任务已完成: ${stats.completed} 成功, ${stats.failed} 失败`,
            icon: '/icon.png',
          })
        }
      } catch {
        // 通知不可用时静默忽略
      }
    },

    /** 请求通知权限 */
    requestNotificationPermission() {
      try {
        if ('Notification' in window && Notification.permission === 'default') {
          Notification.requestPermission()
        }
      } catch {
        // 静默忽略
      }
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
        description: '完整的静力学分析步骤：网格->材料->边界->荷载->求解->结果',
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
        description: '参数化分析：定义参数->循环求解->汇总结果',
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
