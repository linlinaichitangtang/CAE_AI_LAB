/**
 * V2.4 AI Agent 模块
 * 将"问答型 AI 助手"升级为"任务型 Agent"
 */

// 类型导出
export type {
  IntentType,
  IntentResult,
  SubTaskStatus,
  SubTask,
  TaskPlan,
  ToolParamSchema,
  ToolDefinition,
  ToolResult,
  ToolCallRecord,
  AgentState,
  ValidationRule,
  ValidationResult,
  RepairStrategy,
  AgentMetrics,
  AgentMessageRole,
  AgentMessage,
  ToolCallMessage,
  ReActStep,
  ReActStatus,
} from './types'

// 核心组件导出
export { intentClassifier, IntentClassifier } from './intentClassifier'
export { taskPlanner, TaskPlanner } from './taskPlanner'
export { stateTracker, StateTracker } from './stateTracker'
export { toolRegistry } from './tools'
export { toolExecutor, ToolExecutor, type ToolExecutorConfig } from './toolExecutor'
export { resultVerifier, ResultVerifier } from './resultVerifier'
export { selfRepairEngine, SelfRepairEngine } from './selfRepair'
export { agentOrchestrator, AgentOrchestrator, type AgentOrchestratorConfig, type AgentEventType, type AgentEvent } from './agentOrchestrator'

// Claude Code 对齐新增组件
export { ReflectionEngine } from './reflectionEngine'
export { ContextCompressor } from './contextCompressor'

// 新增类型导出
export type {
  ExecutionPhase,
  ReflectionResult,
  ToolReflection,
  CompressionConfig,
  CompressedContext,
  BatchExecuteResult,
} from './types'
