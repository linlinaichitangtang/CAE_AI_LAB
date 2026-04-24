/**
 * V2.4 Agent 编排器
 * 核心调度引擎，串联意图识别→任务规划→工具执行→状态追踪→结果验证→自修复
 */

import type { IntentResult, TaskPlan, AgentMessage, ToolCallMessage, ToolResult, AgentMetrics } from './types'
import { intentClassifier } from './intentClassifier'
import { taskPlanner } from './taskPlanner'
import { stateTracker } from './stateTracker'
import { toolExecutor } from './toolExecutor'
import { resultVerifier } from './resultVerifier'
import { selfRepairEngine } from './selfRepair'

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

/** Agent 事件 */
export interface AgentEvent {
  type: AgentEventType
  data: unknown
  timestamp: number
}

/** Agent 编排器配置 */
export interface AgentOrchestratorConfig {
  enableSelfRepair: boolean
  maxRepairAttempts: number
  streamingEnabled: boolean
  autoExecute: boolean  // 是否自动执行（false时需用户确认每步）
}

/** 默认配置 */
const defaultConfig: AgentOrchestratorConfig = {
  enableSelfRepair: true,
  maxRepairAttempts: 2,
  streamingEnabled: true,
  autoExecute: true
}

/** Agent 编排器 */
export class AgentOrchestrator {
  private config: AgentOrchestratorConfig
  private eventListeners: Map<AgentEventType, Array<(event: AgentEvent) => void>> = new Map()
  private messageHistory: AgentMessage[] = []

  constructor(config?: Partial<AgentOrchestratorConfig>) {
    this.config = { ...defaultConfig, ...config }
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

    // 6. 执行任务计划
    const executionMessages = await this.executePlan(plan)
    messages.push(...executionMessages)

    return messages
  }

  /**
   * 执行任务计划
   */
  private async executePlan(plan: TaskPlan): Promise<AgentMessage[]> {
    const messages: AgentMessage[] = []
    stateTracker.startExecution()

    try {
      while (true) {
        // 获取下一个待执行子任务
        const nextTask = stateTracker.getNextPendingSubTask()
        if (!nextTask) break

        // 执行子任务
        const taskMessages = await this.executeSubTask(nextTask)
        messages.push(...taskMessages)

        // 检查是否被暂停
        if (!stateTracker.isExecuting) {
          stateTracker.updatePlanStatus('paused')
          const pauseMsg = this.createMessage('assistant', '任务已暂停。', { taskId: plan.id })
          messages.push(pauseMsg)
          this.addMessage(pauseMsg)
          break
        }

        // 检查是否全部完成
        if (stateTracker.isAllSubTasksCompleted()) {
          stateTracker.updatePlanStatus('completed')
          this.emitEvent('plan_completed', plan)
          const completeMsg = this.createMessage('assistant',
            `任务完成！共 ${plan.subTasks.length} 个步骤全部执行成功。`,
            { taskId: plan.id }
          )
          messages.push(completeMsg)
          this.addMessage(completeMsg)
          break
        }

        // 检查是否有失败且无法修复的子任务
        if (stateTracker.hasFailedSubTasks() && !this.hasNextRepairableTask()) {
          stateTracker.updatePlanStatus('failed')
          this.emitEvent('plan_failed', plan)
          const failMsg = this.createMessage('assistant',
            `任务执行失败。部分步骤无法完成，请检查错误信息。`,
            { taskId: plan.id }
          )
          messages.push(failMsg)
          this.addMessage(failMsg)
          break
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
   * 执行单个子任务
   */
  private async executeSubTask(subTask: import('./types').SubTask): Promise<AgentMessage[]> {
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
      // 没有工具的子任务直接标记完成
      stateTracker.updateSubTaskStatus(subTask.id, 'done')
      this.emitEvent('subtask_completed', subTask)
      return messages
    }

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

    if (validation.passed) {
      // 验证通过
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
      // 验证失败
      this.emitEvent('verification_failed', validation)

      if (this.config.enableSelfRepair) {
        // 尝试自修复
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

          // 递归执行修复后的任务
          const repairMessages = await this.executeSubTask(repairedTask)
          messages.push(...repairMessages)
        } else {
          // 修复失败或超过重试次数
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

    // 继续执行后续任务
    if (stateTracker.isExecuting) {
      const remainingMessages = await this.executePlan(plan)
      messages.push(...remainingMessages)
    }

    return messages
  }

  /**
   * 暂停执行
   */
  pause(): void {
    stateTracker.pauseExecution()
  }

  /**
   * 恢复执行
   */
  async resume(): Promise<AgentMessage[]> {
    const plan = stateTracker.getCurrentPlan()
    if (!plan) return []

    stateTracker.resumeExecution()
    return this.executePlan(plan)
  }

  /**
   * 取消执行
   */
  cancel(): void {
    stateTracker.cancelExecution()
  }

  /**
   * 获取评估指标
   */
  getMetrics(): AgentMetrics {
    return stateTracker.getMetrics()
  }

  /**
   * 切换 Agent 模式
   */
  setAgentMode(enabled: boolean): void {
    stateTracker.setAgentMode(enabled)
  }

  /**
   * 获取消息历史
   */
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
  }

  private summarizeResult(result: ToolResult, toolName: string): string {
    if (!result.data || typeof result.data !== 'object') return ''
    const data = result.data as Record<string, unknown>

    interface MeshData { nodes?: number; elements?: number }

    switch (toolName) {
      case 'get_model_info': {
        const mesh = data.mesh as MeshData | undefined
        return `几何: ${data.geometryType || 'N/A'}, 网格: ${mesh?.nodes || 0} 节点 / ${mesh?.elements || 0} 单元`
      }
      case 'set_material':
        return `材料: ${data.name || 'N/A'}, E = ${data.youngsModulus || 'N/A'} Pa`
      case 'apply_bc':
        return `边界条件类型: ${data.type || 'N/A'}, 施加面: ${data.face || 'N/A'}`
      case 'run_simulation':
        return `求解${data.convergence ? '收敛' : '未收敛'}, 迭代 ${data.iterations || 'N/A'} 次`
      case 'get_results':
        return `Von Mises: ${(data.maxVonMises as number / 1e6 || 0).toFixed(1)} MPa, 位移: ${((data.maxDisplacement as number || 0) * 1000).toFixed(3)} mm`
      case 'render_contour':
        return `云图已生成 (${data.field || 'N/A'})`
      case 'generate_mesh':
        return `${data.nodes || 0} 节点, ${data.elements || 0} 单元`
      case 'check_mesh_quality':
        return data.passed ? '网格质量合格' : '网格质量需优化'
      case 'validate_results':
        return data.passed ? '结果验证通过' : '结果验证未通过'
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
    return plan.subTasks.some(t => t.status === 'pending' || (t.status === 'failed' && t.retryCount < this.config.maxRepairAttempts))
  }
}

/** 全局 Agent 编排器实例 */
export const agentOrchestrator = new AgentOrchestrator()
