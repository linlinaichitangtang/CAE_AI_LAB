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
  category: 'modeling' | 'simulation' | 'postprocess' | 'code' | 'notes' | 'analysis' | 'system' | 'ml'
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

// ============================================================================
// V3.9 Agent 记忆系统 (五层记忆架构)
// ============================================================================

/** 疲劳 S-N 曲线参数 */
export interface FatigueSNCurve {
  material: string
  stressRatio: number
  curveType: 'basquin' | 'whler' | 'custom'
  basquinB?: number
  basquinM?: number
  fatigueLimit?: number
  cutoffCycles?: number
  coefficientA?: number
  thresholdB?: number
  exponentC?: number
  standardSource?: string
  testConditions?: string
}

/** 热性能参数 */
export interface ThermalParams {
  thermalConductivity?: number
  specificHeat?: number
  thermalExpansion: number
  maxTemperature?: number
  thermalFatigueFactor?: string
}

/** 网格划分指南 */
export interface MeshGuidelines {
  elementType: 'tet10' | 'hex20' | 'tet4' | 'hex8' | 'mixed'
  minSize?: number
  maxSize?: number
  localRefinement?: number
  qualityTarget?: number
  boundaryLayerRatio?: number
  transitionStrategy?: 'gradual' | 'single' | 'hierarchical'
}

/** 求解器推荐设置 */
export interface SolverSettings {
  solverType: 'direct' | 'iterative' | 'multigrid'
  maxIterations?: number
  tolerance?: number
  preconditioner?: 'amg' | 'ilu' | 'jacobi' | 'none'
  linearSolver?: string
  nonlinearStrategy?: 'newton' | 'arc_length' | 'modified_newton'
  timeStepStrategy?: 'fixed' | 'adaptive' | 'automatic'
}

/** 材料知识库 */
export interface MaterialKnowledge {
  material_id: string
  name: string
  category: 'steel' | 'aluminum' | 'titanium' | 'polymer' | 'composite' | 'ceramic' | 'other'
  elastic_modulus: number
  poissons_ratio: number
  density: number
  yield_strength: number
  ultimate_strength?: number
  fatigue_params?: FatigueSNCurve[]
  thermal_params?: ThermalParams
  mesh_guidelines?: MeshGuidelines
  solver_settings?: SolverSettings
  common_errors?: string[]
  last_analysis?: number
  project_ids?: string[]
  notes?: string
}

/** 设计规范来源 */
export type DesignStandardSource = 'BMS7-368E' | 'VDA' | 'GB/T' | 'ASTM' | 'ISO' | 'JIS' | 'enterprise'

/** 载荷工况 */
export interface LoadCase {
  name: string
  type: string
  description?: string
}

/** 设计规范规则 */
export interface DesignStandard {
  id: string
  name: string
  version: string
  source: DesignStandardSource
  scope: string
  safety_factor: number
  allowable_stress?: number
  applicable_load_cases?: LoadCase[]
  rules?: string
  industry?: string
  issue_date?: string
  notes?: string
}

/** 失效模式类型 */
export type FailureModeType = 'HCF' | 'LCF' | 'TMF' | 'CREEP' | 'OVERLOAD' | 'FOD' | 'CORROSION' | 'WEAR'

/** 失效机制描述 */
export interface FailureMechanism {
  name: string
  drivingFactors: string[]
  predictors: string[]
  mitigationStrategies: string[]
  criticalThresholds?: Record<string, number>
}

/** 失效模式图谱节点 */
export interface FailureModeGraph {
  id: string
  mode_type: FailureModeType
  description: string
  applicable_materials?: string[]
  temperatureRange?: { min: number; max: number }
  mechanisms: FailureMechanism[]
  uncertainty: 'high' | 'medium' | 'low'
  related_standards?: string[]
  case_study_ids?: string[]
  priority: number
}

/** 用户偏好 */
export interface UserPreference {
  unitSystem: 'metric' | 'imperial'
  language: string
  theme?: string
  frequentMaterials?: string[]
  frequentSimulationTypes?: string[]
  preferredPostprocessor?: string
}

/** 项目历史摘要 */
export interface ProjectHistoryEntry {
  project_id: string
  project_name: string
  description?: string
  created_at: number
  last_accessed_at: number
  tags?: string[]
  completed_analyses?: number
}

/** 用户画像 */
export interface UserProfile {
  user_id: string
  nickname?: string
  email?: string
  company?: string
  position?: string
  preferences: UserPreference
  project_history: ProjectHistoryEntry[]
  expertise_tags?: string[]
  created_at: number
  last_login_at: number
}

/** 知识召回请求 */
export interface KnowledgeRecallRequest {
  keywords: string[]
  context_type?: 'material' | 'standard' | 'failure_mode' | 'mesh' | 'solver' | 'all'
  project_id?: string
  simulation_type?: string
  limit?: number
}

/** 知识召回结果 */
export interface KnowledgeRecallResult {
  materials?: MaterialKnowledge[]
  standards?: DesignStandard[]
  failure_modes?: FailureModeGraph[]
  confidence: number
  hit_description: string
  source: 'local' | 'cross_session' | 'long_term'
}

// ============================================================================
// V3.10 Agent RAG 向量推理 (Knowledge Retrieval-Augmented Generation)
// ============================================================================

/** 向量嵌入维度 (TF-IDF 特征数) */
export const VECTOR_DIM = 128

/** 嵌入向量类型 (固定维度实数向量) */
export type EmbeddingVector = number[]

/** 知识片段来源类型 */
export type KnowledgeSourceType = 'simulation_result' | 'material_data' | 'design_standard' | 'failure_analysis' | 'user_note' | 'chat_history' | 'project_doc'

/** 向量嵌入记录 */
export interface EmbeddingRecord {
  id: string
  /** 来源类型 */
  source_type: KnowledgeSourceType
  /** 关联项目 ID */
  project_id?: string
  /** 关联用户 ID */
  user_id?: string
  /** 知识片段文本 */
  content: string
  /** TF-IDF 嵌入向量 (VECTOR_DIM 维) */
  embedding: EmbeddingVector
  /** 关键词标签 */
  tags: string[]
  /** 创建时间 */
  created_at: number
  /** 最后被检索时间 */
  last_accessed_at?: number
  /** 检索次数 */
  access_count: number
  /** 元数据 (JSON) */
  metadata_json?: string
}

/** 向量搜索结果 */
export interface VectorSearchResult {
  /** 匹配的嵌入记录 */
  record: EmbeddingRecord
  /** 余弦相似度 (0-1) */
  similarity: number
  /** 排名 (1-based) */
  rank: number
  /** 命中描述 */
  snippet: string
}

/** 向量搜索请求 */
export interface VectorSearchRequest {
  /** 查询文本 */
  query: string
  /** 查询嵌入向量 (可选，不填则自动生成) */
  query_embedding?: EmbeddingVector
  /** 搜索范围: 'project' | 'user' | 'global' */
  scope: 'project' | 'user' | 'global'
  /** 关联项目 ID (scope=project 时必填) */
  project_id?: string
  /** 关联用户 ID (scope=user 时必填) */
  user_id?: string
  /** 返回数量上限 */
  limit?: number
  /** 相似度阈值 (低于此值不返回) */
  similarity_threshold?: number
  /** 过滤来源类型 */
  source_types?: KnowledgeSourceType[]
}

/** 主动回忆请求 */
export interface ActiveRecallRequest {
  /** 当前仿真类型 */
  simulation_type: string
  /** 当前材料 */
  material?: string
  /** 当前项目 ID */
  project_id?: string
  /** 关联用户 ID */
  user_id?: string
  /** 召回数量 */
  limit?: number
}

/** 主动回忆结果 */
export interface ActiveRecallResult {
  /** 检索到的知识片段列表 */
  records: VectorSearchResult[]
  /** 上下文完整性评分 (0-1) */
  context_completeness: number
  /** 建议补充的方向 */
  suggested_directions: string[]
  /** 来源汇总 */
  source_summary: string
}

/** 跨会话知识摘要 */
export interface CrossSessionKnowledgeSummary {
  total_records: number
  top_sources: KnowledgeSourceType[]
  recent_topics: string[]
  mastery_level: 'beginner' | 'intermediate' | 'advanced' | 'expert'
  recommendations: string[]
}
