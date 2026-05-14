/**
 * V2.4 Agent 编排器
 * 核心调度引擎，串联意图识别→任务规划→工具执行→状态追踪→结果验证→自修复
 * Claude Code 对齐：反思机制 / 并行执行 / 上下文压缩 / 智能终止
 */

import type {
  TaskPlan, SubTask, AgentMessage,
  ToolResult, AgentMetrics, ExecutionPhase, AgentOrchestratorConfig as AgentOrchConfig,
} from './types'
import { intentClassifier } from './intentClassifier'
import { taskPlanner } from './taskPlanner'
import { stateTracker } from './stateTracker'
import { toolExecutor } from './toolExecutor'
import { resultVerifier } from './resultVerifier'
import { selfRepairEngine } from './selfRepair'
import { ReflectionEngine } from './reflectionEngine'
import { ContextCompressor } from './contextCompressor'

/** Agent 编排器事件 */
export type AgentEventType =
  | 'intent_classified'
  | 'plan_created'
  | 'subtask_started'
  | 'subtask_completed'
  | 'subtask_failed'
  | 'tool_called'
  | 'tool_result'
  | 'verification_passed'
  | 'verification_failed'
  | 'repair_attempted'
  | 'plan_completed'
  | 'plan_failed'
  | 'plan_paused'
  | 'confirmation_required'
  | 'message'
  | 'reflection_completed'
  | 'context_compressed'
  | 'infinite_loop_detected'
  | 'replan_recommended'

/** Agent 事件 */
export interface AgentEvent {
  type: AgentEventType
  data: unknown
  timestamp: number
}

/** Agent 编排器配置（扩展） */
export interface AgentOrchestratorConfig extends AgentOrchConfig {
  enableSelfRepair: boolean
  maxRepairAttempts: number
  streamingEnabled: boolean
  autoExecute: boolean
  enableGoodEnoughTermination: boolean
  goodEnoughThreshold: number
  maxToolRepeats: number
  allowPartialSuccess: boolean
  replanThreshold: number
  summaryThreshold: number
  maxMessages: number
}

/** 默认配置 */
const defaultConfig: AgentOrchestratorConfig = {
  enableSelfRepair: true,
  maxRepairAttempts: 2,
  streamingEnabled: true,
  autoExecute: true,
  enableGoodEnoughTermination: true,
  goodEnoughThreshold: 0.8,
  maxToolRepeats: 5,
  allowPartialSuccess: false,
  replanThreshold: 0.6,
  summaryThreshold: 100,
  maxMessages: 50,
}

/** Agent 编排器 */
export class AgentOrchestrator {
  private config: AgentOrchestratorConfig
  private eventListeners: Map<AgentEventType, Array<(event: AgentEvent) => void>> = new Map()
  private messageHistory: AgentMessage[] = []
  private reflectionEngine: ReflectionEngine
  private contextCompressor: ContextCompressor
  private consecutiveFailures = 0
  private MAX_CONSECUTIVE_FAILURES = 3

  constructor(config?: Partial<AgentOrchestratorConfig>) {
    this.config = { ...defaultConfig, ...config } as AgentOrchestratorConfig
    this.reflectionEngine = new ReflectionEngine()
    this.contextCompressor = new ContextCompressor({
      maxMessages: this.config.maxMessages,
      summaryThreshold: this.config.summaryThreshold,
    })
  }

  /**
   * 处理用户输入（主入口）
   */
  async processUserInput(input: string): Promise<AgentMessage[]> {
    const messages: AgentMessage[] = []

    // 1. 添加用户消息
    const userMsg = this.createMessage('user', input)
    messages.push(userMsg)
    this.addMessage(userMsg)

    // 2. 意图识别
    const intent = intentClassifier.classify(input)
    this.emitEvent('intent_classified', intent)

    // 3. 如果是 QA 或 unknown，直接走问答模式
    if (intent.intent === 'qa' || intent.intent === 'unknown') {
      const qaMsg = this.createMessage('assistant',
        intent.intent === 'qa'
          ? '这是一个知识问答，我将直接回答您的问题。'
          : '我不太确定您的意图，您是想进行建模、仿真、后处理还是其他操作？'
      )
      messages.push(qaMsg)
      this.addMessage(qaMsg)
      return messages
    }

    // 4. 任务规划
    const plan = taskPlanner.plan(input, intent)

    // 如果没有子任务，说明不需要工具
    if (plan.subTasks.length === 0) {
      const noToolMsg = this.createMessage('assistant',
        `我理解您想进行${this.getIntentName(intent.intent)}相关操作。请提供更具体的需求，例如具体的参数或目标。`
      )
      messages.push(noToolMsg)
      this.addMessage(noToolMsg)
      return messages
    }

    // 重置执行状态
    this.consecutiveFailures = 0
    this.reflectionEngine.resetRepeatCount()

    stateTracker.setCurrentPlan(plan)
    this.emitEvent('plan_created', plan)

    const planMsg = this.createMessage('assistant',
      `任务计划已生成（${plan.subTasks.length} 个步骤）:\n${plan.subTasks.map((t, i) => `${i + 1}. ${t.name}`).join('\n')}\n\n开始执行...`,
      { taskId: plan.id }
    )
    messages.push(planMsg)
    this.addMessage(planMsg)

    // 5. 如果需要确认且不是自动执行
    if (intent.requiresConfirmation && !this.config.autoExecute) {
      const confirmMsg = this.createMessage('assistant',
        '请确认是否开始执行此任务计划？',
        { taskId: plan.id }
      )
      messages.push(confirmMsg)
      this.addMessage(confirmMsg)
      return messages
    }

    // 6. 执行任务计划（Phase-based 状态机）
    const executionMessages = await this.executePlan(plan)
    messages.push(...executionMessages)

    return messages
  }

  /**
   * 执行任务计划 — Phase-based 状态机
   */
  private async executePlan(plan: TaskPlan): Promise<AgentMessage[]> {
    const messages: AgentMessage[] = []
    stateTracker.startExecution()

    try {
      let phase: ExecutionPhase = 'executing' as ExecutionPhase

      while (phase !== 'terminating') {
        switch (phase) {
          case 'executing': {
            // 获取下一批可执行的任务（支持并行）
            const batch = this.getNextBatch(plan)
            if (batch.length === 0) {
              phase = 'completing'
              continue
            }

            // 执行这一批任务
            const batchResults = await this.executeBatch(batch)
            messages.push(...batchResults.messages)

            // 更新失败计数
            if (batchResults.hasFailures) {
              this.consecutiveFailures++
            } else {
              this.consecutiveFailures = 0
            }

            // 检查无限循环
            if (this.checkInfiniteLoop(plan)) {
              this.emitEvent('infinite_loop_detected', null)
              const loopMsg = this.createMessage('assistant',
                `检测到重复执行相同工具超过 ${this.config.maxToolRepeats} 次，自动终止循环。`
              )
              messages.push(loopMsg)
              this.addMessage(loopMsg)
              phase = 'terminating'
              continue
            }

            phase = 'validating'
            break
          }

          case 'validating': {
            // 检查是否需要修复
            if (this.config.enableSelfRepair && this.hasNextRepairableTask()) {
              phase = 'repairing'
            } else if (this.consecutiveFailures >= this.MAX_CONSECUTIVE_FAILURES) {
              // 连续失败超限，终止
              stateTracker.updatePlanStatus('failed')
              this.emitEvent('plan_failed', plan)
              const failMsg = this.createMessage('assistant',
                `任务执行失败：连续 ${this.consecutiveFailures} 个步骤失败。`
              )
              messages.push(failMsg)
              this.addMessage(failMsg)
              phase = 'terminating'
            } else if (this.shouldTerminateEarly(plan)) {
              phase = 'completing'
            } else {
              phase = 'executing'
            }
            break
          }

          case 'repairing': {
            const repairMessages = await this.attemptBatchRepair(plan)
            messages.push(...repairMessages)
            this.consecutiveFailures = 0
            phase = 'executing'
            break
          }

          case 'completing': {
            if (this.isResultGoodEnough(plan)) {
              stateTracker.updatePlanStatus('completed')
              this.emitEvent('plan_completed', plan)
              const completeMsg = this.createMessage('assistant',
                `任务${this.config.allowPartialSuccess ? '（部分）' : ''}完成！共 ${plan.subTasks.length} 个步骤。`,
                { taskId: plan.id }
              )
              messages.push(completeMsg)
              this.addMessage(completeMsg)
            } else {
              stateTracker.updatePlanStatus('failed')
              this.emitEvent('plan_failed', plan)
              const failMsg = this.createMessage('assistant',
                `任务未达到完成标准（${this.getCompletionRatio(plan) * 100}% < ${this.config.goodEnoughThreshold * 100}%）。`
              )
              messages.push(failMsg)
              this.addMessage(failMsg)
            }
            phase = 'terminating'
            break
          }

          default: {
            phase = 'terminating' as ExecutionPhase
            break
          }
        }
      }
    } catch (error) {
      stateTracker.updatePlanStatus('failed')
      const errorMsg = this.createMessage('assistant',
        `任务执行异常: ${String(error)}`,
        { taskId: plan.id }
      )
      messages.push(errorMsg)
      this.addMessage(errorMsg)
    }

    return messages
  }

  /**
   * 获取下一批可并行执行的任务
   */
  private getNextBatch(plan: TaskPlan): SubTask[] {
    const pending = plan.subTasks.filter(t => t.status === 'pending')
    if (pending.length === 0) return []

    // 找出所有没有未满足依赖的 pending 任务
    const ready: SubTask[] = []
    for (const task of pending) {
      const depsSatisfied = task.dependsOn.every(depId => {
        const dep = plan.subTasks.find(s => s.id === depId)
        return dep && (dep.status === 'done' || dep.status === 'skipped')
      })
      if (depsSatisfied) ready.push(task)
    }

    if (ready.length === 0) return []

    // 找出有 parallelGroup 的任务，同组可并行
    const grouped = ready.filter(t => t.parallelGroup)
    if (grouped.length > 0) {
      // 取第一个 parallelGroup 的所有任务
      const groupId = grouped[0].parallelGroup!
      return ready.filter(t => t.parallelGroup === groupId)
    }

    // 无并行组，只取第一个
    return [ready[0]]
  }

  /**
   * 执行一批任务（支持并行）
   */
  private async executeBatch(tasks: SubTask[]): Promise<{ messages: AgentMessage[]; hasFailures: boolean }> {
    const messages: AgentMessage[] = []
    let hasFailures = false

    if (tasks.length === 1) {
      // 单任务直接执行
      const taskMessages = await this.executeSubTask(tasks[0])
      messages.push(...taskMessages)
      hasFailures = tasks[0].status === 'failed'
    } else {
      // 多任务并行执行
      const startMsg = this.createMessage('assistant',
        `并行执行 ${tasks.length} 个任务: ${tasks.map(t => t.name).join(', ')}...`
      )
      messages.push(startMsg)
      this.addMessage(startMsg)

      const promises = tasks.map(async (task) => {
        return this.executeSubTask(task)
      })

      const results = await Promise.allSettled(promises)

      for (let i = 0; i < results.length; i++) {
        const result = results[i]
        if (result.status === 'fulfilled') {
          messages.push(...result.value)
          if (tasks[i].status === 'failed') hasFailures = true
        } else {
          const failMsg = this.createMessage('assistant',
            `${tasks[i].name} 执行异常: ${String(result.reason)}`
          )
          messages.push(failMsg)
          this.addMessage(failMsg)
          hasFailures = true
        }
      }

      const doneMsg = this.createMessage('assistant',
        `${tasks.length} 个任务执行完成。`
      )
      messages.push(doneMsg)
      this.addMessage(doneMsg)
    }

    return { messages, hasFailures }
  }

  /**
   * 执行单个子任务
   */
  private async executeSubTask(subTask: SubTask): Promise<AgentMessage[]> {
    const messages: AgentMessage[] = []

    // 更新状态为运行中
    stateTracker.updateSubTaskStatus(subTask.id, 'running')
    this.emitEvent('subtask_started', subTask)

    const startMsg = this.createMessage('assistant',
      `正在执行: ${subTask.name}...`,
      { subTaskId: subTask.id, taskId: stateTracker.getCurrentPlan()?.id }
    )
    messages.push(startMsg)
    this.addMessage(startMsg)

    if (!subTask.toolName) {
      stateTracker.updateSubTaskStatus(subTask.id, 'done')
      this.emitEvent('subtask_completed', subTask)
      return messages
    }

    // 记录工具调用（用于无限循环检测）
    this.reflectionEngine.incrementRepeatCount(subTask.toolName)

    // 调用工具
    this.emitEvent('tool_called', { toolName: subTask.toolName, params: subTask.toolParams })
    const result = await toolExecutor.invoke(subTask.toolName, subTask.toolParams || {})
    this.emitEvent('tool_result', result)

    // 检查是否需要用户确认
    if (result.error === 'CONFIRMATION_REQUIRED') {
      stateTracker.updateSubTaskStatus(subTask.id, 'waiting_confirmation')
      this.emitEvent('confirmation_required', result.metadata)
      const confirmMsg = this.createMessage('assistant',
        `操作 "${subTask.name}" 需要您的确认才能执行。是否继续？`,
        { subTaskId: subTask.id, toolCalls: [{ id: `tc-${Date.now()}`, toolName: subTask.toolName!, params: subTask.toolParams || {}, status: 'calling' }] }
      )
      messages.push(confirmMsg)
      this.addMessage(confirmMsg)
      return messages
    }

    // 记录工具调用
    stateTracker.recordToolCall(subTask.toolName, subTask.toolParams || {}, result, subTask.id)

    // 验证结果
    const validation = resultVerifier.verify(result, subTask.toolName)

    // 反思评估
    const reflection = this.reflectionEngine.reflect({
      toolName: subTask.toolName,
      result,
      validation,
      historicalContext: stateTracker.getToolCallHistory(),
    })
    this.emitEvent('reflection_completed', reflection)

    if (reflection.shouldReplan) {
      this.emitEvent('replan_recommended', reflection)
      const replanMsg = this.createMessage('assistant',
        `[反思] ${subTask.name} 结果置信度 ${(reflection.confidence * 100).toFixed(0)}%，建议重新规划。`
      )
      messages.push(replanMsg)
      this.addMessage(replanMsg)
    }

    if (validation.passed) {
      stateTracker.updateSubTaskStatus(subTask.id, 'done', result)
      this.emitEvent('verification_passed', validation)
      this.emitEvent('subtask_completed', subTask)

      const resultSummary = this.summarizeResult(result, subTask.toolName)
      const doneMsg = this.createMessage('assistant',
        `${subTask.name} 完成。${resultSummary}`,
        { subTaskId: subTask.id, taskId: stateTracker.getCurrentPlan()?.id }
      )
      messages.push(doneMsg)
      this.addMessage(doneMsg)
    } else {
      this.emitEvent('verification_failed', validation)

      if (this.config.enableSelfRepair) {
        const repairStrategies = selfRepairEngine.analyzeFailure(subTask.toolName!, result, subTask)

        if (repairStrategies.length > 0 && subTask.retryCount < this.config.maxRepairAttempts) {
          const strategy = repairStrategies[0]
          this.emitEvent('repair_attempted', strategy)

          const repairedTask = selfRepairEngine.applyStrategy(subTask, strategy)
          stateTracker.updateSubTaskStatus(subTask.id, 'pending')
          stateTracker.incrementRetry(subTask.id)

          const repairMsg = this.createMessage('assistant',
            `${subTask.name} 验证未通过，尝试自动修复: ${strategy.description}（原因: ${strategy.reason}）`,
            { subTaskId: subTask.id }
          )
          messages.push(repairMsg)
          this.addMessage(repairMsg)

          const repairMessages = await this.executeSubTask(repairedTask)
          messages.push(...repairMessages)
        } else {
          stateTracker.updateSubTaskStatus(subTask.id, 'failed', result, validation.summary)
          this.emitEvent('subtask_failed', subTask)

          const failMsg = this.createMessage('assistant',
            `${subTask.name} 失败: ${validation.summary}`,
            { subTaskId: subTask.id }
          )
          messages.push(failMsg)
          this.addMessage(failMsg)
        }
      } else {
        stateTracker.updateSubTaskStatus(subTask.id, 'failed', result, validation.summary)
        this.emitEvent('subtask_failed', subTask)

        const failMsg = this.createMessage('assistant',
          `${subTask.name} 失败: ${validation.summary}`,
          { subTaskId: subTask.id }
        )
        messages.push(failMsg)
        this.addMessage(failMsg)
      }
    }

    return messages
  }

  /**
   * 确认等待中的操作
   */
  async confirmPendingAction(subTaskId: string): Promise<AgentMessage[]> {
    const plan = stateTracker.getCurrentPlan()
    if (!plan) return []

    const subTask = plan.subTasks.find(t => t.id === subTaskId)
    if (!subTask || subTask.status !== 'waiting_confirmation') return []

    const messages: AgentMessage[] = []
    stateTracker.updateSubTaskStatus(subTask.id, 'running')

    const result = await toolExecutor.invoke(subTask.toolName!, subTask.toolParams || {})
    stateTracker.recordToolCall(subTask.toolName!, subTask.toolParams || {}, result, subTask.id)

    const validation = resultVerifier.verify(result, subTask.toolName!)
    if (validation.passed) {
      stateTracker.updateSubTaskStatus(subTask.id, 'done', result)
      const msg = this.createMessage('assistant', `${subTask.name} 已确认并执行完成。`, { subTaskId })
      messages.push(msg)
      this.addMessage(msg)
    } else {
      stateTracker.updateSubTaskStatus(subTask.id, 'failed', result, validation.summary)
      const msg = this.createMessage('assistant', `${subTask.name} 执行失败: ${validation.summary}`, { subTaskId })
      messages.push(msg)
      this.addMessage(msg)
    }

    if (stateTracker.isExecuting) {
      const remainingMessages = await this.executePlan(plan)
      messages.push(...remainingMessages)
    }

    return messages
  }

  /** 暂停执行 */
  pause(): void {
    stateTracker.pauseExecution()
  }

  /** 恢复执行 */
  async resume(): Promise<AgentMessage[]> {
    const plan = stateTracker.getCurrentPlan()
    if (!plan) return []
    stateTracker.resumeExecution()
    return this.executePlan(plan)
  }

  /** 取消执行 */
  cancel(): void {
    stateTracker.cancelExecution()
  }

  /** 获取评估指标 */
  getMetrics(): AgentMetrics {
    return stateTracker.getMetrics()
  }

  /** 切换 Agent 模式 */
  setAgentMode(enabled: boolean): void {
    stateTracker.setAgentMode(enabled)
  }

  /** 获取消息历史 */
  getMessages(): AgentMessage[] {
    return [...this.messageHistory]
  }

  // ============================================================================
  // 事件系统
  // ============================================================================

  on(event: AgentEventType, listener: (event: AgentEvent) => void): () => void {
    if (!this.eventListeners.has(event)) {
      this.eventListeners.set(event, [])
    }
    this.eventListeners.get(event)!.push(listener)
    return () => {
      const arr = this.eventListeners.get(event)
      if (arr) {
        const idx = arr.indexOf(listener)
        if (idx >= 0) arr.splice(idx, 1)
      }
    }
  }

  private emitEvent(type: AgentEventType, data: unknown): void {
    const event: AgentEvent = { type, data, timestamp: Date.now() }
    const listeners = this.eventListeners.get(type)
    if (listeners) {
      for (const listener of listeners) {
        try { listener(event) } catch (e) { console.error('Event listener error:', e) }
      }
    }
  }

  // ============================================================================
  // 辅助方法
  // ============================================================================

  private createMessage(role: AgentMessage['role'], content: string, extra?: Partial<AgentMessage>): AgentMessage {
    return {
      id: `msg-${Date.now()}-${Math.random().toString(36).substring(2, 6)}`,
      role,
      content,
      timestamp: Date.now(),
      ...extra
    }
  }

  private addMessage(msg: AgentMessage): void {
    this.messageHistory.push(msg)

    // 上下文压缩检查
    if (this.contextCompressor.needsCompression(this.messageHistory)) {
      const compressed = this.contextCompressor.compress(this.messageHistory)
      if (compressed.compressedCount > 0) {
        this.emitEvent('context_compressed', compressed)
        // 替换历史为压缩后的（保留的 + 摘要消息）
        const summaryMsg = this.createMessage('system', compressed.summary, {
          metadata: { type: 'compressed_summary', ...compressed }
        })
        this.messageHistory = [...compressed.preservedMessages, summaryMsg]
      }
    }
  }

  private summarizeResult(result: ToolResult, toolName: string): string {
    if (!result.data || typeof result.data !== 'object') return ''
    const data = result.data as Record<string, unknown>

    switch (toolName) {
      case 'get_model_info': {
        const mesh = data.mesh as { nodes?: number; elements?: number } | undefined
        return `几何: ${data.geometryType || 'N/A'}, 网格: ${mesh?.nodes || 0} 节点 / ${mesh?.elements || 0} 单元`
      }
      case 'set_material':
        return `材料: ${data.name || 'N/A'}, E = ${data.youngsModulus || 'N/A'} Pa`
      case 'apply_bc':
        return `边界条件类型: ${data.type || 'N/A'}, 施加面: ${data.face || 'N/A'}`
      case 'run_simulation':
        return `求解${data.convergence ? '收敛' : '未收敛'}, 迭代 ${data.iterations || 'N/A'} 次`
      case 'get_results':
        return `Von Mises: ${((data.maxVonMises as number) / 1e6 || 0).toFixed(1)} MPa, 位移: ${((data.maxDisplacement as number) || 0) * 1000} mm`
      case 'render_contour':
        return `云图已生成 (${data.field || 'N/A'})`
      case 'generate_mesh':
        return `${data.nodes || 0} 节点, ${data.elements || 0} 单元`
      case 'check_mesh_quality':
        return data.passed ? '网格质量合格' : '网格质量需优化'
      case 'validate_results':
        return data.passed ? '结果验证通过' : '结果验证未通过'
      case 'tc4_multimodal_predict':
        return `失效模式: ${data.failure_mode || 'N/A'}, 寿命: ${data.cycles || 'N/A'} cycles`
      default:
        return '执行完成'
    }
  }

  private getIntentName(intent: string): string {
    const names: Record<string, string> = {
      modeling: '建模', simulation: '仿真', postprocess: '后处理',
      code: '代码', analysis: '分析', notes: '笔记',
      parametric: '参数化', optimization: '优化', validation: '验证'
    }
    return names[intent] || intent
  }

  private hasNextRepairableTask(): boolean {
    const plan = stateTracker.getCurrentPlan()
    if (!plan) return false
    return plan.subTasks.some(t =>
      (t.status === 'pending') ||
      (t.status === 'failed' && t.retryCount < this.config.maxRepairAttempts)
    )
  }

  private checkInfiniteLoop(plan: TaskPlan): boolean {
    for (const task of plan.subTasks) {
      if (task.toolName) {
        const count = this.reflectionEngine.incrementRepeatCount(task.toolName)
        if (count >= this.config.maxToolRepeats) return true
      }
    }
    return false
  }

  private shouldTerminateEarly(_plan: TaskPlan): boolean {
    // 检查是否全部完成
    if (stateTracker.isAllSubTasksCompleted()) return true
    // 检查是否有失败且无修复选项
    if (stateTracker.hasFailedSubTasks() && !this.hasNextRepairableTask()) return true
    return false
  }

  private isResultGoodEnough(plan: TaskPlan): boolean {
    if (this.config.enableGoodEnoughTermination) {
      const ratio = this.getCompletionRatio(plan)
      if (ratio >= this.config.goodEnoughThreshold) {
        // 检查 critical 任务（无依赖）是否全完成
        const criticalTasks = plan.subTasks.filter(t => t.dependsOn.length === 0)
        const criticalDone = criticalTasks.every(t => t.status === 'done')
        return criticalDone
      }
      return false
    }
    return stateTracker.isAllSubTasksCompleted()
  }

  private getCompletionRatio(plan: TaskPlan): number {
    const completed = plan.subTasks.filter(t => t.status === 'done').length
    return completed / plan.subTasks.length
  }

  private async attemptBatchRepair(plan: TaskPlan): Promise<AgentMessage[]> {
    const messages: AgentMessage[] = []
    const repairMsg = this.createMessage('assistant', '开始批量修复...')
    messages.push(repairMsg)
    this.addMessage(repairMsg)

    const failedTasks = plan.subTasks.filter(t => t.status === 'failed' && t.retryCount < this.config.maxRepairAttempts)
    for (const task of failedTasks) {
      const repairStrategies = selfRepairEngine.analyzeFailure(task.toolName!, task.result!, task)
      if (repairStrategies.length > 0) {
        const strategy = repairStrategies[0]
        selfRepairEngine.applyStrategy(task, strategy)
        stateTracker.updateSubTaskStatus(task.id, 'pending')
        stateTracker.incrementRetry(task.id)

        const taskRepairMsg = this.createMessage('assistant',
          `修复 ${task.name}: ${strategy.description}`
        )
        messages.push(taskRepairMsg)
        this.addMessage(taskRepairMsg)
      }
    }

    return messages
  }
}

/** 全局 Agent 编排器实例 */
export const agentOrchestrator = new AgentOrchestrator()
