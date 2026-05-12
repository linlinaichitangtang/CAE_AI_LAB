/**
 * useNaturalLanguageControl.ts — V3.4-005 自然语言控制
 * "帮我分析这个梁的受力" → 自动执行并解读结果
 */

import { ref, computed } from 'vue'

// ============ 类型定义 ============

export type NLProvider = 'openai' | 'anthropic' | 'azure' | 'ollama' | 'custom'
export type IntentType =
  | 'analysis_request'
  | 'geometry_create'
  | 'mesh_generate'
  | 'simulation_run'
  | 'results_query'
  | 'comparison'
  | 'optimization'
  | 'export'
  | 'help'
  | 'clarification'
  | 'unknown'

export type EntityType =
  | 'geometry'
  | 'material'
  | 'boundary_condition'
  | 'mesh_parameter'
  | 'simulation_type'
  | 'result_type'
  | 'time_range'
  | 'numeric_value'

export interface NLEntity {
  type: EntityType
  value: string
  confidence: number
  startIndex: number
  endIndex: number
  synonyms?: string[]
}

export interface ParsedIntent {
  intent: IntentType
  confidence: number
  entities: NLEntity[]
  parameters: Record<string, any>
  originalText: string
  language: 'zh' | 'en'
  requiresClarification: boolean
  clarificationQuestions?: string[]
}

export interface NLAction {
  id: string
  type: string
  description: string
  status: 'pending' | 'executing' | 'completed' | 'failed'
  result?: any
  error?: string
  startTime: string
  endTime?: string
  steps: ActionStep[]
}

export interface ActionStep {
  name: string
  status: 'pending' | 'running' | 'completed' | 'failed'
  result?: any
  error?: string
  timestamp: string
}

export interface ChatMessage {
  id: string
  role: 'user' | 'assistant' | 'system'
  content: string
  timestamp: string
  intent?: ParsedIntent
  actions?: NLAction[]
  metadata?: Record<string, any>
}

export interface NLProviderConfig {
  provider: NLProvider
  apiKey?: string
  baseUrl?: string
  model?: string
  temperature?: number
  maxTokens?: number
  enabled: boolean
}

export interface ConversationContext {
  projectId?: string
  currentGeometry?: string
  currentMaterial?: string
  recentIntents: IntentType[]
  variables: Record<string, any>  // 用户定义的变量
}

// ============ 意图模式定义 ============

const INTENT_PATTERNS: Record<IntentType, {
  zh: string[]
  en: string[]
  priority: number
}> = {
  analysis_request: {
    zh: ['分析', '求解', '计算', '仿真', '受力', '变形', '应力', '应变', '模态', '热分析'],
    en: ['analyze', 'solve', 'calculate', 'simulate', 'stress', 'strain', 'displacement', 'modal', 'thermal'],
    priority: 10
  },
  geometry_create: {
    zh: ['创建', '建模', '画', '生成', '添加', '新的几何'],
    en: ['create', 'model', 'draw', 'generate', 'add', 'new geometry'],
    priority: 9
  },
  mesh_generate: {
    zh: ['网格', '划分网格', '剖分', '网格化'],
    en: ['mesh', 'generate mesh', 'discretize', 'mesh refinement'],
    priority: 8
  },
  simulation_run: {
    zh: ['运行', '开始仿真', '执行', '提交'],
    en: ['run', 'start simulation', 'execute', 'submit'],
    priority: 8
  },
  results_query: {
    zh: ['结果', '查看', '显示', '获取结果', '最大值', '最小值'],
    en: ['results', 'show', 'display', 'get results', 'max', 'min', 'value'],
    priority: 7
  },
  comparison: {
    zh: ['对比', '比较', '差异', '不同'],
    en: ['compare', 'difference', 'vs', 'versus', 'between'],
    priority: 6
  },
  optimization: {
    zh: ['优化', '最小化', '最大化', '找最优', '参数优化'],
    en: ['optimize', 'minimize', 'maximize', 'find optimal', 'parameter optimization'],
    priority: 7
  },
  export: {
    zh: ['导出', '下载', '保存为', '输出'],
    en: ['export', 'download', 'save as', 'output'],
    priority: 6
  },
  help: {
    zh: ['帮助', '如何使用', '怎么', '什么', '教我', '说明'],
    en: ['help', 'how to', 'what', 'teach me', 'explain'],
    priority: 5
  },
  clarification: {
    zh: ['不确定', '哪个', '怎么选', '建议'],
    en: ['not sure', 'which', 'how to choose', 'suggest'],
    priority: 4
  },
  unknown: {
    zh: [],
    en: [],
    priority: 0
  }
}

// ============ 实体模式定义 ============

const ENTITY_PATTERNS: Record<EntityType, {
  zh: RegExp[]
  en: RegExp[]
}> = {
  geometry: {
    zh: [/梁|beam/i, /板|plate/i, /壳|shell/i, /块|block/i, /圆柱|cylinder/i, /球|sphere/i],
    en: [/beam/i, /plate/i, /shell/i, /block/i, /cylinder/i, /sphere/i]
  },
  material: {
    zh: [/钢|steel/i, /铝|aluminum/i, /铜|copper/i, /塑料|plastic/i, /钛|titanium/i],
    en: [/steel/i, /aluminum|aluminium/i, /copper/i, /plastic/i, /titanium/i]
  },
  boundary_condition: {
    zh: [/固定|constraint/i, /载荷|load/i, /力|force/i, /压力|pressure/i, /位移|displacement/i],
    en: [/fixed|constraint/i, /load/i, /force/i, /pressure/i, /displacement/i]
  },
  mesh_parameter: {
    zh: [/粗|coarse/i, /中|medium/i, /细|fine/i, /网格尺寸|mesh size/i],
    en: [/coarse/i, /medium/i, /fine/i, /mesh size/i]
  },
  simulation_type: {
    zh: [/静态|static/i, /模态|modal/i, /热|thermal/i, /屈曲|buckling/i, /瞬态|transient/i],
    en: [/static/i, /modal/i, /thermal/i, /buckling/i, /transient/i]
  },
  result_type: {
    zh: [/应力|stress/i, /应变|strain/i, /位移|displacement/i, /变形|deformation/i],
    en: [/stress/i, /strain/i, /displacement/i, /deformation/i, /von mises/i]
  },
  time_range: {
    zh: [/\d+\s*(秒|分钟|小时|min|sec|hour)/i, /时间|time/i],
    en: [/\d+\s*(second|minute|hour|min|sec)/i, /time/i]
  },
  numeric_value: {
    zh: [/\d+\.?\d*\s*(mm|cm|m|MPa|GPa|N|kN)/i, /\d+\.?\d*/],
    en: [/\d+\.?\d*\s*(mm|cm|m|MPa|GPa|N|kN)/i, /\d+\.?\d*/]
  }
}

// ============ 工具函数 ============

function generateId(): string {
  return `nl_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

function detectLanguage(text: string): 'zh' | 'en' {
  // 简单检测：中文 Unicode 范围
  const chineseRegex = /[一-鿿]/
  return chineseRegex.test(text) ? 'zh' : 'en'
}

function extractNumbers(text: string): number[] {
  const matches = text.match(/\d+\.?\d*/g)
  return matches ? matches.map(Number) : []
}

// ============ 主 Composable ============

export function useNaturalLanguageControl() {
  const config = ref<NLProviderConfig>({
    provider: 'openai',
    model: 'gpt-4',
    temperature: 0.7,
    maxTokens: 2000,
    enabled: true
  })

  const conversationHistory = ref<ChatMessage[]>([])
  const currentContext = ref<ConversationContext>({
    recentIntents: [],
    variables: {}
  })

  const isProcessing = ref(false)
  const currentActions = ref<NLAction[]>([])

  // ============ 初始化 ============

  function loadData(): void {
    const storedConfig = getStorage<NLProviderConfig>('caelab_nl_config')
    if (storedConfig) config.value = storedConfig

    const storedHistory = getStorage<ChatMessage[]>('caelab_nl_history')
    if (storedHistory) conversationHistory.value = storedHistory

    const storedContext = getStorage<ConversationContext>('caelab_nl_context')
    if (storedContext) currentContext.value = storedContext
  }

  // ============ 意图识别 ============

  /**
   * 解析自然语言输入
   */
  function parseInput(text: string): ParsedIntent {
    const language = detectLanguage(text)
    const entities = extractEntities(text, language)
    const intent = classifyIntent(text, language, entities)
    const parameters = extractParameters(text, intent, entities, language)

    // 检查是否需要澄清
    const requiresClarification = checkRequiresClarification(intent, parameters)

    const result: ParsedIntent = {
      intent,
      confidence: calculateIntentConfidence(intent, entities),
      entities,
      parameters,
      originalText: text,
      language,
      requiresClarification,
      clarificationQuestions: requiresClarification
        ? generateClarificationQuestions(intent, parameters, language)
        : undefined
    }

    // 更新上下文
    currentContext.value.recentIntents.push(intent)
    if (currentContext.value.recentIntents.length > 10) {
      currentContext.value.recentIntents.shift()
    }

    return result
  }

  /**
   * 意图分类
   */
  function classifyIntent(
    text: string,
    language: 'zh' | 'en',
    entities: NLEntity[]
  ): IntentType {
    const scores: Record<IntentType, number> = {} as Record<IntentType, number>

    for (const [intent, patterns] of Object.entries(INTENT_PATTERNS)) {
      const patternList = language === 'zh' ? patterns.zh : patterns.en
      let matchCount = 0

      for (const pattern of patternList) {
        if (text.includes(pattern) || new RegExp(pattern, 'i').test(text)) {
          matchCount++
        }
      }

      scores[intent] = matchCount * patterns.priority
    }

    // 找到最高分
    let bestIntent: IntentType = 'unknown'
    let bestScore = 0

    for (const [intent, score] of Object.entries(scores)) {
      if (score > bestScore) {
        bestScore = score
        bestIntent = intent as IntentType
      }
    }

    return bestIntent
  }

  /**
   * 实体提取
   */
  function extractEntities(text: string, language: 'zh' | 'en'): NLEntity[] {
    const entities: NLEntity[] = []

    for (const [type, patterns] of Object.entries(ENTITY_PATTERNS)) {
      const patternList = language === 'zh' ? patterns.zh : patterns.en

      for (const pattern of patternList) {
        const regex = new RegExp(pattern, 'gi')
        let match

        while ((match = regex.exec(text)) !== null) {
          entities.push({
            type: type as EntityType,
            value: match[0],
            confidence: 0.8,
            startIndex: match.index,
            endIndex: match.index + match[0].length
          })
        }
      }
    }

    // 按起始位置排序
    return entities.sort((a, b) => a.startIndex - b.startIndex)
  }

  /**
   * 参数提取
   */
  function extractParameters(
    text: string,
    intent: IntentType,
    entities: NLEntity[],
    language: 'zh' | 'en'
  ): Record<string, any> {
    const params: Record<string, any> = {}
    const numbers = extractNumbers(text)

    // 几何类型
    const geometryEntity = entities.find(e => e.type === 'geometry')
    if (geometryEntity) {
      params.geometryType = mapGeometryType(geometryEntity.value, language)
    }

    // 材料
    const materialEntity = entities.find(e => e.type === 'material')
    if (materialEntity) {
      params.material = mapMaterial(materialEntity.value)
    }

    // 仿真类型
    const simEntity = entities.find(e => e.type === 'simulation_type')
    if (simEntity) {
      params.simulationType = mapSimulationType(simEntity.value)
    }

    // 结果类型
    const resultEntity = entities.find(e => e.type === 'result_type')
    if (resultEntity) {
      params.resultType = mapResultType(resultEntity.value)
    }

    // 边界条件
    const bcEntity = entities.find(e => e.type === 'boundary_condition')
    if (bcEntity) {
      params.boundaryCondition = mapBoundaryCondition(bcEntity.value)
    }

    // 数值参数
    if (numbers.length > 0) {
      params.values = numbers
    }

    // 网格尺寸
    if (text.includes('mesh') || text.includes('网格')) {
      if (text.includes('coarse') || text.includes('粗')) {
        params.meshSize = 'coarse'
      } else if (text.includes('fine') || text.includes('细')) {
        params.meshSize = 'fine'
      } else if (numbers.length > 0) {
        params.meshSize = numbers[0]
      } else {
        params.meshSize = 'medium'
      }
    }

    return params
  }

  function mapGeometryType(value: string, language: 'zh' | 'en'): string {
    const map: Record<string, string> = {
      '梁': 'beam',
      'beam': 'beam',
      '板': 'plate',
      'plate': 'plate',
      '壳': 'shell',
      'shell': 'shell',
      '块': 'block',
      'block': 'block',
      '圆柱': 'cylinder',
      'cylinder': 'cylinder',
      '球': 'sphere',
      'sphere': 'sphere'
    }
    return map[value.toLowerCase()] || value
  }

  function mapMaterial(value: string): string {
    const map: Record<string, string> = {
      '钢': 'steel',
      'steel': 'steel',
      '铝': 'aluminum',
      'aluminum': 'aluminum',
      '铝合金': 'aluminum_alloy',
      '铜': 'copper',
      'copper': 'copper',
      '塑料': 'plastic',
      'plastic': 'plastic',
      '钛': 'titanium',
      'titanium': 'titanium'
    }
    return map[value.toLowerCase()] || value
  }

  function mapSimulationType(value: string): string {
    const map: Record<string, string> = {
      '静态': 'static',
      'static': 'static',
      '模态': 'modal',
      'modal': 'modal',
      '热': 'thermal',
      'thermal': 'thermal',
      '屈曲': 'buckling',
      'buckling': 'buckling',
      '瞬态': 'transient',
      'transient': 'transient'
    }
    return map[value.toLowerCase()] || value
  }

  function mapResultType(value: string): string {
    const map: Record<string, string> = {
      '应力': 'stress',
      'stress': 'stress',
      'von mises': 'von_mises',
      '应变': 'strain',
      'strain': 'strain',
      '位移': 'displacement',
      'displacement': 'displacement',
      '变形': 'deformation',
      'deformation': 'deformation'
    }
    return map[value.toLowerCase()]] || value
  }

  function mapBoundaryCondition(value: string): string {
    const map: Record<string, string> = {
      '固定': 'fixed',
      'fixed': 'fixed',
      '约束': 'constraint',
      'constraint': 'constraint',
      '载荷': 'load',
      'load': 'load',
      '力': 'force',
      'force': 'force',
      '压力': 'pressure',
      'pressure': 'pressure'
    }
    return map[value.toLowerCase()] || value
  }

  /**
   * 计算意图置信度
   */
  function calculateIntentConfidence(intent: IntentType, entities: NLEntity[]): number {
    if (intent === 'unknown') return 0

    let confidence = 0.5  // 基础置信度

    // 实体越多，置信度越高
    confidence += entities.length * 0.1

    // 特定意图关键词匹配
    if (INTENT_PATTERNS[intent].zh.some(k => intent.includes(k)) ||
        INTENT_PATTERNS[intent].en.some(k => intent.includes(k))) {
      confidence += 0.3
    }

    return Math.min(1, confidence)
  }

  /**
   * 检查是否需要澄清
   */
  function checkRequiresClarification(intent: IntentType, parameters: Record<string, any>): boolean {
    // 缺少必要参数时需要澄清
    switch (intent) {
      case 'analysis_request':
      case 'simulation_run':
        return !parameters.geometryType && !parameters.simulationType
      case 'geometry_create':
        return !parameters.geometryType
      default:
        return false
    }
  }

  /**
   * 生成澄清问题
   */
  function generateClarificationQuestions(
    intent: IntentType,
    parameters: Record<string, any>,
    language: 'zh' | 'en'
  ): string[] {
    const questions: string[] = []

    if (!parameters.geometryType) {
      questions.push(language === 'zh'
        ? '请问您的几何模型是什么类型？（梁、板、壳等）'
        : 'What type of geometry are you working with? (beam, plate, shell, etc.)')
    }

    if (!parameters.simulationType && (intent === 'analysis_request' || intent === 'simulation_run')) {
      questions.push(language === 'zh'
        ? '您想进行什么类型的分析？（静态、模态、热分析等）'
        : 'What type of analysis would you like to perform? (static, modal, thermal, etc.)')
    }

    if (!parameters.material) {
      questions.push(language === 'zh'
        ? '请问使用什么材料？'
        : 'What material are you using?')
    }

    return questions
  }

  // ============ 自然语言处理 ============

  /**
   * 处理用户输入
   */
  async function processInput(text: string): Promise<{
    intent: ParsedIntent
    response: string
    actions: NLAction[]
  }> {
    isProcessing.value = true

    try {
      // 解析意图
      const intent = parseInput(text)

      // 添加到历史
      const userMessage: ChatMessage = {
        id: generateId(),
        role: 'user',
        content: text,
        timestamp: new Date().toISOString(),
        intent
      }
      conversationHistory.value.push(userMessage)

      // 生成响应和执行动作
      const { response, actions } = await generateResponse(intent)

      // 添加助手消息
      const assistantMessage: ChatMessage = {
        id: generateId(),
        role: 'assistant',
        content: response,
        timestamp: new Date().toISOString(),
        intent,
        actions
      }
      conversationHistory.value.push(assistantMessage)

      // 保存历史
      saveHistory()

      return { intent, response, actions }
    } finally {
      isProcessing.value = false
    }
  }

  /**
   * 生成响应
   */
  async function generateResponse(intent: ParsedIntent): Promise<{
    response: string
    actions: NLAction[]
  }> {
    const actions: NLAction[] = []

    // 需要澄清
    if (intent.requiresClarification && intent.clarificationQuestions) {
      return {
        response: intent.clarificationQuestions.join('\n'),
        actions: []
      }
    }

    // 根据意图生成响应和动作
    switch (intent.intent) {
      case 'analysis_request':
        return handleAnalysisRequest(intent)
      case 'geometry_create':
        return handleGeometryCreate(intent)
      case 'mesh_generate':
        return handleMeshGenerate(intent)
      case 'simulation_run':
        return handleSimulationRun(intent)
      case 'results_query':
        return handleResultsQuery(intent)
      case 'help':
        return handleHelp(intent)
      default:
        return {
          response: intent.language === 'zh'
            ? '抱歉，我不太理解您的意思。请尝试更具体地描述您的需求。'
            : 'Sorry, I don\'t understand. Please try being more specific.',
          actions: []
        }
    }
  }

  // ============ 意图处理器 ============

  async function handleAnalysisRequest(intent: ParsedIntent): Promise<{ response: string; actions: NLAction[] }> {
    const actions: NLAction[] = []
    const params = intent.parameters

    // 动作1: 确定几何
    actions.push({
      id: generateId(),
      type: 'identify_geometry',
      description: intent.language === 'zh' ? '识别几何模型' : 'Identifying geometry',
      status: 'completed',
      result: { geometryType: params.geometryType },
      startTime: new Date().toISOString()
    })

    // 动作2: 确定仿真类型
    actions.push({
      id: generateId(),
      type: 'setup_simulation',
      description: intent.language === 'zh' ? '设置仿真参数' : 'Setting up simulation',
      status: 'pending',
      steps: [],
      startTime: new Date().toISOString()
    })

    // 动作3: 运行仿真
    actions.push({
      id: generateId(),
      type: 'run_simulation',
      description: intent.language === 'zh' ? '运行仿真分析' : 'Running simulation',
      status: 'pending',
      steps: [],
      startTime: new Date().toISOString()
    })

    const response = intent.language === 'zh'
      ? `好的，我来帮您分析${params.geometryType || '几何模型'}的${params.simulationType || '静态'}性能。\n\n我将执行以下步骤:\n1. 识别几何模型\n2. 设置边界条件和载荷\n3. 生成网格\n4. 运行仿真分析\n\n开始执行...`
      : `OK, I'll help you analyze the ${params.simulationType || 'static'} performance of your ${params.geometryType || 'geometry'}.\n\nI'll execute:\n1. Identify geometry\n2. Set boundary conditions and loads\n3. Generate mesh\n4. Run simulation\n\nStarting...`

    return { response, actions }
  }

  async function handleGeometryCreate(intent: ParsedIntent): Promise<{ response: string; actions: NLAction[] }> {
    const params = intent.parameters

    const response = intent.language === 'zh'
      ? `好的，我来帮您创建${params.geometryType || '几何'}模型。请告诉我具体的尺寸参数（长度、宽度、高度等）。`
      : `OK, I'll help you create a ${params.geometryType || 'geometry'} model. Please provide the dimensions (length, width, height, etc.).`

    return {
      response,
      actions: [{
        id: generateId(),
        type: 'create_geometry',
        description: intent.language === 'zh' ? '创建几何模型' : 'Creating geometry',
        status: 'pending',
        startTime: new Date().toISOString()
      }]
    }
  }

  async function handleMeshGenerate(intent: ParsedIntent): Promise<{ response: string; actions: NLAction[] }> {
    const params = intent.parameters

    const response = intent.language === 'zh'
      ? `好的，我将为您生成网格。\n\n网格设置:\n- 类型: ${params.meshSize || 'medium（中等）'}\n- 尺寸: ${params.meshSize === 'coarse' ? '粗' : params.meshSize === 'fine' ? '细' : '中等'}\n\n开始生成...`
      : `OK, I'll generate the mesh for you.\n\nMesh settings:\n- Type: ${params.meshSize || 'medium'}\n- Size: ${params.meshSize === 'coarse' ? 'coarse' : params.meshSize === 'fine' ? 'fine' : 'medium'}\n\nGenerating...`

    return {
      response,
      actions: [{
        id: generateId(),
        type: 'generate_mesh',
        description: intent.language === 'zh' ? '生成网格' : 'Generating mesh',
        status: 'pending',
        startTime: new Date().toISOString()
      }]
    }
  }

  async function handleSimulationRun(intent: ParsedIntent): Promise<{ response: string; actions: NLAction[] }> {
    const response = intent.language === 'zh'
      ? '好的，我将运行仿真分析。这可能需要几分钟时间，请稍候...'
      : 'OK, I'll run the simulation analysis. This may take a few minutes, please wait...'

    return {
      response,
      actions: [{
        id: generateId(),
        type: 'run_simulation',
        description: intent.language === 'zh' ? '运行仿真' : 'Running simulation',
        status: 'pending',
        startTime: new Date().toISOString()
      }]
    }
  }

  async function handleResultsQuery(intent: ParsedIntent): Promise<{ response: string; actions: NLAction[] }> {
    const params = intent.parameters

    // 模拟查询结果
    const mockResults = {
      maxDisplacement: (Math.random() * 10).toFixed(4),
      maxStress: (Math.random() * 200).toFixed(2),
      maxStrain: (Math.random() * 0.001).toFixed(6)
    }

    const resultType = params.resultType || 'stress'
    let value: string

    switch (resultType) {
      case 'displacement':
        value = `${mockResults.maxDisplacement} mm`
        break
      case 'strain':
        value = `${mockResults.maxStrain}`
        break
      default:
        value = `${mockResults.maxStress} MPa`
    }

    const response = intent.language === 'zh'
      ? `根据分析结果，最大${params.resultType || '应力'}为: **${value}**`
      : `Based on the analysis, the maximum ${params.resultType || 'stress'} is: **${value}**`

    return {
      response,
      actions: [{
        id: generateId(),
        type: 'query_results',
        description: intent.language === 'zh' ? '查询结果' : 'Querying results',
        status: 'completed',
        result: { [resultType]: value },
        startTime: new Date().toISOString()
      }]
    }
  }

  async function handleHelp(intent: ParsedIntent): Promise<{ response: string; actions: NLAction[] }> {
    const response = intent.language === 'zh'
      ? `您好！我是 CAELab 的智能助手。我可以帮您:\n\n**仿真分析**\n- "帮我分析这个梁的受力"\n- "运行静态分析"\n- "计算应力分布"\n\n**几何建模**\n- "创建一个圆柱体"\n- "画一个板模型"\n\n**网格生成**\n- "生成细网格"\n- "划分中等密度网格"\n\n**结果查询**\n- "最大位移是多少"\n- "显示应力云图"\n\n**其他**\n- "帮我生成报告"\n- "导出为 PDF"\n\n请告诉我您想做什么？`
      : `Hello! I'm CAELab's AI assistant. I can help you with:\n\n**Simulation Analysis**\n- "Analyze stress on this beam"\n- "Run static analysis"\n- "Calculate stress distribution"\n\n**Geometry Modeling**\n- "Create a cylinder"\n- "Draw a plate model"\n\n**Mesh Generation**\n- "Generate fine mesh"\n- "Create medium density mesh"\n\n**Results Query**\n- "What's the maximum displacement"\n- "Show stress contour"\n\n**Other**\n- "Generate a report"\n- "Export to PDF"\n\nWhat would you like to do?`

    return { response, actions: [] }
  }

  // ============ 动作执行 ============

  /**
   * 执行动作序列
   */
  async function executeAction(action: NLAction): Promise<void> {
    action.status = 'executing'
    action.startTime = new Date().toISOString()

    try {
      // 模拟执行
      await new Promise(resolve => setTimeout(resolve, 1000))

      action.status = 'completed'
      action.endTime = new Date().toISOString()
    } catch (e: any) {
      action.status = 'failed'
      action.error = e.message
      action.endTime = new Date().toISOString()
    }
  }

  // ============ 对话管理 ============

  function clearHistory(): void {
    conversationHistory.value = []
    currentContext.value = {
      recentIntents: [],
      variables: {}
    }
    saveHistory()
    saveContext()
  }

  function setContextVariable(name: string, value: any): void {
    currentContext.value.variables[name] = value
    saveContext()
  }

  // ============ 持久化 ============

  function saveHistory(): void {
    setStorage('caelab_nl_history', conversationHistory.value.slice(-100))  // 保留最近100条
  }

  function saveContext(): void {
    setStorage('caelab_nl_context', currentContext.value)
  }

  function saveConfig(): void {
    setStorage('caelab_nl_config', config.value)
  }

  // ============ 统计 ============

  const stats = computed(() => ({
    totalConversations: conversationHistory.value.length,
    userMessages: conversationHistory.value.filter(m => m.role === 'user').length,
    intentDistribution: countIntentDistribution(),
    averageConfidence: calculateAverageConfidence()
  }))

  function countIntentDistribution(): Record<IntentType, number> {
    const distribution: Record<string, number> = {}

    for (const msg of conversationHistory.value) {
      if (msg.intent) {
        const intent = msg.intent.intent
        distribution[intent] = (distribution[intent] || 0) + 1
      }
    }

    return distribution as Record<IntentType, number>
  }

  function calculateAverageConfidence(): number {
    const intents = conversationHistory.value
      .filter(m => m.intent)
      .map(m => m.intent!.confidence)

    if (intents.length === 0) return 0
    return intents.reduce((a, b) => a + b, 0) / intents.length
  }

  // 初始化
  loadData()

  return {
    // 状态
    config,
    conversationHistory,
    currentContext,
    isProcessing,
    currentActions,

    // 解析
    parseInput,

    // 处理
    processInput,
    executeAction,

    // 配置
    updateConfig: (updates: Partial<NLProviderConfig>) => {
      Object.assign(config.value, updates)
      saveConfig()
    },

    // 对话管理
    clearHistory,
    setContextVariable,

    // 工具
    detectLanguage,
    extractEntities,

    // 统计
    stats,

    // 常量
    INTENT_PATTERNS,
    ENTITY_PATTERNS
  }
}
