/**
 * V2.4-003 状态追踪器
 * 每个子任务有唯一 ID + 状态 + 结果，可中断/恢复
 * 持久化到 aiState
 */

import type { AgentState, TaskPlan, SubTask, SubTaskStatus, ToolCallRecord, ToolResult, AgentMetrics } from './types'

/** 短UUID */
function shortId(): string {
  return Math.random().toString(36).substring(2, 8)
}

/** 默认 Agent 状态 */
const defaultAgentState: AgentState = {
  currentPlan: null,
  isAgentMode: false,
  isExecuting: false,
  toolCallHistory: [],
  taskHistory: [],
  interruptedTaskId: null,
  totalTasksCompleted: 0,
  totalTasksFailed: 0,
  totalToolCalls: 0,
  totalToolCallErrors: 0
}

/** 状态追踪器 */
export class StateTracker {
  private state: AgentState
  private listeners: Map<string, Array<(state: AgentState) => void>> = new Map()

  constructor() {
    this.state = this.loadState()
  }

  /**
   * 获取当前状态
   */
  getState(): AgentState {
    return { ...this.state }
  }

  /**
   * 获取当前任务计划
   */
  getCurrentPlan(): TaskPlan | null {
    return this.state.currentPlan
  }

  /**
   * 是否处于 Agent 模式
   */
  get isAgentMode(): boolean {
    return this.state.isAgentMode
  }

  /**
   * 是否正在执行
   */
  get isExecuting(): boolean {
    return this.state.isExecuting
  }

  // ============================================================================
  // 任务计划管理
  // ============================================================================

  /**
   * 设置当前任务计划
   */
  setCurrentPlan(plan: TaskPlan): void {
    this.state.currentPlan = plan
    this.state.isExecuting = false
    this.persistState()
    this.emitChange()
  }

  /**
   * 更新任务计划状态
   */
  updatePlanStatus(status: TaskPlan['status']): void {
    if (!this.state.currentPlan) return
    this.state.currentPlan.status = status
    this.state.currentPlan.updatedAt = Date.now()
    if (status === 'completed' || status === 'failed' || status === 'cancelled') {
      this.state.currentPlan.completedAt = Date.now()
      this.state.isExecuting = false
      // 归档到历史
      if (this.state.currentPlan) {
        this.state.taskHistory.push({ ...this.state.currentPlan })
      }
      if (status === 'completed') {
        this.state.totalTasksCompleted++
      } else if (status === 'failed') {
        this.state.totalTasksFailed++
      }
    }
    this.persistState()
    this.emitChange()
  }

  /**
   * 开始执行任务
   */
  startExecution(): void {
    this.state.isExecuting = true
    if (this.state.currentPlan) {
      this.state.currentPlan.status = 'executing'
      this.state.currentPlan.updatedAt = Date.now()
    }
    this.persistState()
    this.emitChange()
  }

  /**
   * 暂停任务执行
   */
  pauseExecution(): void {
    this.state.isExecuting = false
    if (this.state.currentPlan) {
      this.state.currentPlan.status = 'paused'
      this.state.interruptedTaskId = this.state.currentPlan.id
      this.state.currentPlan.updatedAt = Date.now()
    }
    this.persistState()
    this.emitChange()
  }

  /**
   * 恢复任务执行
   */
  resumeExecution(): void {
    this.state.isExecuting = true
    if (this.state.currentPlan) {
      this.state.currentPlan.status = 'executing'
      this.state.currentPlan.updatedAt = Date.now()
    }
    this.state.interruptedTaskId = null
    this.persistState()
    this.emitChange()
  }

  /**
   * 取消任务执行
   */
  cancelExecution(): void {
    this.state.isExecuting = false
    if (this.state.currentPlan) {
      this.state.currentPlan.status = 'cancelled'
      this.state.currentPlan.completedAt = Date.now()
      this.state.currentPlan.updatedAt = Date.now()
      this.state.taskHistory.push({ ...this.state.currentPlan })
      this.state.currentPlan = null
    }
    this.persistState()
    this.emitChange()
  }

  // ============================================================================
  // 子任务状态管理
  // ============================================================================

  /**
   * 更新子任务状态
   */
  updateSubTaskStatus(subTaskId: string, status: SubTaskStatus, result?: ToolResult, error?: string): void {
    if (!this.state.currentPlan) return

    const subTask = this.state.currentPlan.subTasks.find(t => t.id === subTaskId)
    if (!subTask) return

    subTask.status = status

    if (status === 'running') {
      subTask.startedAt = Date.now()
    } else if (status === 'done') {
      subTask.completedAt = Date.now()
      if (subTask.startedAt) {
        subTask.actualTime = (Date.now() - subTask.startedAt) / 1000
      }
      if (result) {
        subTask.result = result
      }
    } else if (status === 'failed') {
      subTask.completedAt = Date.now()
      if (error) {
        subTask.error = error
      }
    }

    // 更新当前步骤索引
    const runningIndex = this.state.currentPlan.subTasks.findIndex(t => t.status === 'running')
    if (runningIndex >= 0) {
      this.state.currentPlan.currentStepIndex = runningIndex
    }

    this.state.currentPlan.updatedAt = Date.now()
    this.persistState()
    this.emitChange()
  }

  /**
   * 增加子任务重试次数
   */
  incrementRetry(subTaskId: string): number {
    if (!this.state.currentPlan) return 0
    const subTask = this.state.currentPlan.subTasks.find(t => t.id === subTaskId)
    if (!subTask) return 0
    subTask.retryCount++
    this.persistState()
    this.emitChange()
    return subTask.retryCount
  }

  /**
   * 获取下一个待执行的子任务
   */
  getNextPendingSubTask(): SubTask | null {
    if (!this.state.currentPlan) return null

    for (const subTask of this.state.currentPlan.subTasks) {
      if (subTask.status === 'pending') {
        // 检查依赖是否都已完成
        const depsSatisfied = subTask.dependsOn.every(depId => {
          const dep = this.state.currentPlan!.subTasks.find(t => t.id === depId)
          return dep && dep.status === 'done'
        })
        if (depsSatisfied) {
          return subTask
        }
      }
    }
    return null
  }

  /**
   * 检查是否所有子任务都已完成
   */
  isAllSubTasksCompleted(): boolean {
    if (!this.state.currentPlan) return false
    return this.state.currentPlan.subTasks.every(
      t => t.status === 'done' || t.status === 'skipped'
    )
  }

  /**
   * 检查是否有失败的子任务
   */
  hasFailedSubTasks(): boolean {
    if (!this.state.currentPlan) return false
    return this.state.currentPlan.subTasks.some(t => t.status === 'failed')
  }

  // ============================================================================
  // 工具调用记录
  // ============================================================================

  /**
   * 记录工具调用
   */
  recordToolCall(toolName: string, params: Record<string, unknown>, result: ToolResult, subTaskId: string): void {
    const record: ToolCallRecord = {
      id: `call-${shortId()}`,
      toolName,
      params,
      result,
      timestamp: Date.now(),
      subTaskId
    }
    this.state.toolCallHistory.push(record)
    this.state.totalToolCalls++
    if (!result.success) {
      this.state.totalToolCallErrors++
    }
    this.persistState()
    this.emitChange()
  }

  /**
   * 获取工具调用历史
   */
  getToolCallHistory(): ToolCallRecord[] {
    return [...this.state.toolCallHistory]
  }

  // ============================================================================
  // Agent 模式管理
  // ============================================================================

  /**
   * 切换 Agent 模式
   */
  setAgentMode(enabled: boolean): void {
    this.state.isAgentMode = enabled
    if (!enabled && this.state.isExecuting) {
      this.pauseExecution()
    }
    this.persistState()
    this.emitChange()
  }

  // ============================================================================
  // 评估指标
  // ============================================================================

  /**
   * 获取 Agent 评估指标
   */
  getMetrics(): AgentMetrics {
    const totalTasks = this.state.totalTasksCompleted + this.state.totalTasksFailed
    return {
      intentAccuracy: 0,  // 由外部更新
      toolCallSuccessRate: this.state.totalToolCalls > 0
        ? (this.state.totalToolCalls - this.state.totalToolCallErrors) / this.state.totalToolCalls
        : 0,
      taskCompletionRate: totalTasks > 0
        ? this.state.totalTasksCompleted / totalTasks
        : 0,
      avgStepsPerTask: totalTasks > 0
        ? this.state.taskHistory.reduce((sum, t) => sum + t.subTasks.length, 0) / totalTasks
        : 0,
      avgExecutionTime: totalTasks > 0
        ? this.state.taskHistory.reduce((sum, t) => {
            if (t.completedAt && t.createdAt) {
              return sum + (t.completedAt - t.createdAt) / 1000
            }
            return sum
          }, 0) / totalTasks
        : 0,
      selfRepairSuccessRate: 0,  // 由外部更新
      totalTasks,
      completedTasks: this.state.totalTasksCompleted,
      failedTasks: this.state.totalTasksFailed,
      totalToolCalls: this.state.totalToolCalls
    }
  }

  // ============================================================================
  // 持久化
  // ============================================================================

  /**
   * 保存状态到 localStorage
   */
  private persistState(): void {
    try {
      const serializable = {
        ...this.state,
        currentPlan: this.state.currentPlan ? {
          ...this.state.currentPlan,
          subTasks: this.state.currentPlan.subTasks.map(st => ({
            ...st,
            // 移除不可序列化的字段
          }))
        } : null
      }
      localStorage.setItem('caelab_agent_state', JSON.stringify(serializable))
    } catch (e) {
      console.error('Failed to persist agent state:', e)
    }
  }

  /**
   * 从 localStorage 加载状态
   */
  private loadState(): AgentState {
    try {
      const saved = localStorage.getItem('caelab_agent_state')
      if (saved) {
        const parsed = JSON.parse(saved)
        return {
          ...defaultAgentState,
          ...parsed,
          isExecuting: false,  // 恢复时不自动执行
          currentPlan: parsed.currentPlan && parsed.currentPlan.status !== 'completed' && parsed.currentPlan.status !== 'cancelled'
            ? { ...parsed.currentPlan, status: 'paused' as const }
            : null
        }
      }
    } catch (e) {
      console.error('Failed to load agent state:', e)
    }
    return { ...defaultAgentState }
  }

  /**
   * 清除所有状态
   */
  clearState(): void {
    this.state = { ...defaultAgentState }
    localStorage.removeItem('caelab_agent_state')
    this.emitChange()
  }

  // ============================================================================
  // 事件监听
  // ============================================================================

  /**
   * 注册状态变更监听器
   */
  onChange(listener: (state: AgentState) => void): () => void {
    const id = shortId()
    if (!this.listeners.has('change')) {
      this.listeners.set('change', [])
    }
    this.listeners.get('change')!.push(listener)
    return () => {
      const arr = this.listeners.get('change')
      if (arr) {
        const idx = arr.indexOf(listener)
        if (idx >= 0) arr.splice(idx, 1)
      }
    }
  }

  /**
   * 触发状态变更事件
   */
  private emitChange(): void {
    const listeners = this.listeners.get('change')
    if (listeners) {
      const snapshot = this.getState()
      for (const listener of listeners) {
        try {
          listener(snapshot)
        } catch (e) {
          console.error('State listener error:', e)
        }
      }
    }
  }
}

/** 全局状态追踪器实例 */
export const stateTracker = new StateTracker()
