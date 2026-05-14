/**
 * V2.4 AI Agent - 核心类型定义
 * 将"问答型 AI 助手"升级为"任务型 Agent"
 */

// ============================================================================
// 意图识别 (V2.4-001)
// ============================================================================

/** 意图类别 */
export type IntentType =
  | 'modeling'        // 建模任务
  | 'simulation'      // 仿真任务
  | 'postprocess'     // 后处理任务
  | 'code'            // 代码任务
  | 'analysis'        // 分析任务（结果分析、诊断）
  | 'notes'           // 笔记任务
  | 'parametric'      // 参数化扫描
  | 'optimization'    // 优化任务
  | 'validation'      // 验证任务
  | 'qa'              // 问答（不需要工具）
  | 'unknown'         // 未知意图

/** 意图识别结果 */
export interface IntentResult {
  intent: IntentType
  confidence: number       // 0~1
  keywords: string[]       // 匹配到的关键词
  subIntent?: string       // 子意图（如 simulation.static / simulation.modal）
  requiresConfirmation: boolean  // 是否需要用户确认
}

// ============================================================================
// 任务规划 (V2.4-002)
// ============================================================================

/** 子任务状态 */
export type SubTaskStatus = 'pending' | 'running' | 'done' | 'failed' | 'skipped' | 'waiting_confirmation'

/** 子任务 */
export interface SubTask {
  id: string               // 唯一ID，格式: task-{uuid-short}
  name: string             // 子任务名称
  description: string      // 子任务描述
  toolName?: string        // 要调用的工具名
  toolParams?: Record<string, unknown>  // 工具参数
  status: SubTaskStatus
  result?: ToolResult      // 工具执行结果
  error?: string           // 错误信息
  retryCount: number       // 重试次数
  maxRetries: number       // 最大重试次数
  dependsOn: string[]      // 依赖的子任务ID
  estimatedTime?: number   // 预估耗时（秒）
  actualTime?: number      // 实际耗时（秒）
  createdAt: number
  startedAt?: number
  completedAt?: number
  parallelGroup?: string  // 并行任务组标记，同组任务可并行执行
}

/** 任务计划 */
export interface TaskPlan {
  id: string               // 任务计划ID
  userQuery: string        // 用户原始查询
  intent: IntentResult     // 意图识别结果
  subTasks: SubTask[]      // 子任务列表
  status: 'planning' | 'executing' | 'paused' | 'completed' | 'failed' | 'cancelled'
  currentStepIndex: number // 当前执行到的步骤
  createdAt: number
  updatedAt: number
  completedAt?: number
}

// ============================================================================
// 工具系统 (V2.4-004, V2.4-005, V2.4-006~010)
// ============================================================================

/** 工具参数定义 */
export interface ToolParamSchema {
  name: string
  type: 'string' | 'number' | 'boolean' | 'object' | 'array'
  description: string
  required: boolean
  default?: unknown
  enum?: string[]
  minimum?: number
  maximum?: number
}

/** 工具定义 */
export interface ToolDefinition {
  name: string
  description: string
  category: 'modeling' | 'simulation' | 'postprocess' | 'code' | 'notes' | 'analysis' | 'system'
  params: ToolParamSchema[]
  returnType: string
  requiresConfirmation: boolean  // 是否需要用户确认
  tauriCommand?: string          // 对应的 Tauri 后端命令
  isDestructive: boolean         // 是否有破坏性操作
  examples?: string[]            // 使用示例
}

/** 工具执行结果 */
export interface ToolResult {
  success: boolean
  data?: unknown
  error?: string
  executionTime: number          // 执行耗时（ms）
  metadata?: Record<string, unknown>
}

/** 工具调用记录 */
export interface ToolCallRecord {
  id: string
  toolName: string
  params: Record<string, unknown>
  result: ToolResult
  timestamp: number
  subTaskId: string
}

// ============================================================================
// 状态追踪 (V2.4-003)
// ============================================================================

/** Agent 运行状态 */
export interface AgentState {
  currentPlan: TaskPlan | null
  isAgentMode: boolean       // 是否处于 Agent 模式
  isExecuting: boolean       // 是否正在执行任务
  toolCallHistory: ToolCallRecord[]  // 工具调用历史
  taskHistory: TaskPlan[]    // 历史任务
  interruptedTaskId: string | null  // 被中断的任务ID
  totalTasksCompleted: number
  totalTasksFailed: number
  totalToolCalls: number
  totalToolCallErrors: number
}

// ============================================================================
// 结果验证 (V2.4-013)
// ============================================================================

/** 验证规则 */
export interface ValidationRule {
  field: string
  check: 'range' | 'type' | 'not_empty' | 'custom'
  params: Record<string, unknown>
  message: string
}

/** 验证结果 */
export interface ValidationResult {
  passed: boolean
  rules: {
    rule: ValidationRule
    passed: boolean
    actualValue?: unknown
    message: string
  }[]
  summary: string
}

// ============================================================================
// 自修复 (V2.4-015)
// ============================================================================

/** 修复策略 */
export interface RepairStrategy {
  description: string
  toolName: string
  params: Record<string, unknown>
  reason: string
}

// ============================================================================
// Agent 评估 (V2.4-021)
// ============================================================================

/** 评估指标 */
export interface AgentMetrics {
  intentAccuracy: number       // 意图识别准确率
  toolCallSuccessRate: number  // 工具调用成功率
  taskCompletionRate: number   // 任务完成率
  avgStepsPerTask: number      // 平均每任务步骤数
  avgExecutionTime: number     // 平均执行时间（秒）
  selfRepairSuccessRate: number // 自修复成功率
  totalTasks: number
  completedTasks: number
  failedTasks: number
  totalToolCalls: number
}

// ============================================================================
// 消息扩展 (兼容现有 Message 类型)
// ============================================================================

/** Agent 消息角色 */
export type AgentMessageRole = 'user' | 'assistant' | 'system' | 'tool'

/** Agent 消息（扩展自现有 Message） */
export interface AgentMessage {
  id: string
  role: AgentMessageRole
  content: string
  timestamp: number
  // Agent 扩展字段
  toolCalls?: ToolCallMessage[]
  toolCallId?: string         // tool 角色消息的调用ID
  taskId?: string             // 关联的任务ID
  subTaskId?: string          // 关联的子任务ID
  isStreaming?: boolean
  metadata?: Record<string, unknown>
}

/** 工具调用消息 */
export interface ToolCallMessage {
  id: string
  toolName: string
  params: Record<string, unknown>
  status: 'calling' | 'completed' | 'failed'
  result?: ToolResult
}

// ============================================================================
// ReAct 循环 (V2.4-002)
// ============================================================================

/** ReAct 步骤 */
export interface ReActStep {
  thought: string            // 思考过程
  action: string             // 行动（工具名或 'final_answer'）
  actionInput: Record<string, unknown>  // 行动参数
  observation?: string       // 观察结果
  timestamp: number
}

/** ReAct 循环状态 */
export type ReActStatus = 'thinking' | 'acting' | 'observing' | 'finished' | 'error'

// ============================================================================
// 执行循环增强 (Claude Code 对齐)
// ============================================================================

/** 执行循环阶段 */
export type ExecutionPhase = 'executing' | 'validating' | 'repairing' | 'replanning' | 'completing' | 'terminating'

/** 反思结果 */
export interface ReflectionResult {
  confidence: number           // 0-1
  quality: 'excellent' | 'good' | 'marginal' | 'poor'
  shouldReplan: boolean        // 是否触发重规划
  reason: string
  suggestions: string[]       // 改进建议
}

/** 工具反思数据 */
export interface ToolReflection {
  toolName: string
  result: ToolResult
  validation: ValidationResult
  historicalContext: ToolCallRecord[]
}

/** 上下文压缩配置 */
export interface CompressionConfig {
  maxMessages: number          // 保留最近 N 条原文 (default: 50)
  summaryThreshold: number      // 超过此数量则触发压缩 (default: 100)
  aggressiveSummary: boolean    // 是否激进压缩
}

/** 压缩后的上下文 */
export interface CompressedContext {
  summary: string               // 压缩摘要
  messageCount: number          // 原始消息数
  compressedCount: number       // 被压缩的消息数
  preservedMessages: AgentMessage[]  // 保留的原文消息
  compressionRatio: number       // 压缩率
}

/** 批量执行结果 */
export interface BatchExecuteResult {
  results: Map<string, ToolResult>
  totalTime: number
  errors: string[]
}

// ============================================================================
// SubTask 扩展
// ============================================================================

/** 子任务（扩展） */
export interface SubTask {
  id: string
  name: string
  description: string
  toolName?: string
  toolParams?: Record<string, unknown>
  status: SubTaskStatus
  result?: ToolResult
  error?: string
  retryCount: number
  maxRetries: number
  dependsOn: string[]
  estimatedTime?: number
  actualTime?: number
  createdAt: number
  startedAt?: number
  completedAt?: number
  parallelGroup?: string        // 并行组ID，同组任务可并行执行
}

/** Agent Orchestrator 配置 */
export interface AgentOrchestratorConfig {
  enableSelfRepair: boolean           // 是否启用自修复 (default: true)
  enableGoodEnoughTermination: boolean // 是否启用"足够好"终止 (default: true)
  goodEnoughThreshold: number           // 终止阈值 (default: 0.8)
  maxToolRepeats: number               // 同一工具最大连续调用次数 (default: 5)
  allowPartialSuccess: boolean         // 是否允许部分成功 (default: false)
  replanThreshold: number              // 反思重规划阈值 (default: 0.6)
  summaryThreshold: number             // 上下文压缩阈值 (default: 100)
  maxMessages: number                  // 压缩保留消息数 (default: 50)
}
