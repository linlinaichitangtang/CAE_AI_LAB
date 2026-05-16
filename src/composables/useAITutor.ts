/**
 * useAITutor.ts — V3.8-002 AI 教学助手
 * 为学生提供交互式学习体验：语音讲解 + 步骤演示 + 即时反馈
 */
import { ref, computed } from 'vue'

// ============ 类型定义 ============

export type TutoringMode = 'explanation' | 'demonstration' | 'practice' | 'quiz'
export type DifficultyLevel = 'beginner' | 'intermediate' | 'advanced'

export interface TeachingStep {
  id: string
  title: string
  description: string
  visualElement?: {
    type: 'diagram' | 'animation' | 'model' | 'code' | 'chart'
    content: string
  }
  voiceScript?: string  // AI 讲解的语音文本
  duration?: number     // 预计讲解时长（秒）
  keyPoints?: string[]  // 关键知识点
}

export interface StudentProgress {
  currentLesson: string | null
  completedLessons: string[]
  quizScores: Record<string, number>
  timeSpent: number      // 学习时长（分钟）
  strengths: string[]   // 擅长的主题
  weaknesses: string[]   // 薄弱的主题
}

export interface QuizQuestion {
  id: string
  question: string
  options?: string[]
  correctAnswer: string | number
  explanation: string
  difficulty: DifficultyLevel
}

export interface AICapability {
  tts: boolean           // 文字转语音
  stt: boolean           // 语音转文字
  vision: boolean        // 视觉识别
  reasoning: boolean     // 推理能力
}

// ============ 状态 ============

const tutoringMode = ref<TutoringMode>('explanation')
const difficultyLevel = ref<DifficultyLevel>('beginner')
const isSpeaking = ref(false)
const isListening = ref(false)
const currentLessonSteps = ref<TeachingStep[]>([])
const currentStepIndex = ref(0)
const studentProgress = ref<StudentProgress>({
  currentLesson: null,
  completedLessons: [],
  quizScores: {},
  timeSpent: 0,
  strengths: [],
  weaknesses: []
})
const conversationHistory = ref<{ role: 'ai' | 'student'; content: string; time: Date }[]>([])

// ============ 语音合成 ============

let speechSynthesis: SpeechSynthesis | null = null

function initSpeechSynthesis() {
  if (typeof window !== 'undefined' && window.speechSynthesis) {
    speechSynthesis = window.speechSynthesis
  }
}

function speak(text: string, options?: {
  rate?: number
  pitch?: number
  volume?: number
  voice?: string
  lang?: string
}): Promise<void> {
  return new Promise((resolve, reject) => {
    if (!speechSynthesis) {
      initSpeechSynthesis()
    }

    if (!speechSynthesis) {
      reject(new Error('Speech synthesis not available'))
      return
    }

    // 停止之前的语音
    speechSynthesis.cancel()

    const utterance = new SpeechSynthesisUtterance(text)
    utterance.rate = options?.rate || 1.0
    utterance.pitch = options?.pitch || 1.0
    utterance.volume = options?.volume || 1.0
    utterance.lang = options?.lang || 'zh-CN'

    if (options?.voice) {
      const voices = speechSynthesis.getVoices()
      const selectedVoice = voices.find(v => v.name.includes(options.voice!))
      if (selectedVoice) utterance.voice = selectedVoice
    }

    utterance.onstart = () => {
      isSpeaking.value = true
    }

    utterance.onend = () => {
      isSpeaking.value = false
      resolve()
    }

    utterance.onerror = (e) => {
      isSpeaking.value = false
      reject(e)
    }

    speechSynthesis.speak(utterance)
  })
}

function stopSpeaking() {
  if (speechSynthesis) {
    speechSynthesis.cancel()
    isSpeaking.value = false
  }
}

function pauseSpeaking() {
  if (speechSynthesis && isSpeaking.value) {
    speechSynthesis.pause()
  }
}

function resumeSpeaking() {
  if (speechSynthesis) {
    speechSynthesis.resume()
  }
}

// ============ 语音识别 ============

let speechRecognition: any = null

function initSpeechRecognition(): Promise<boolean> {
  return new Promise((resolve) => {
    if (typeof window === 'undefined') {
      resolve(false)
      return
    }

    const SpeechRecognition = (window as any).SpeechRecognition ||
                               (window as any).webkitSpeechRecognition

    if (!SpeechRecognition) {
      resolve(false)
      return
    }

    speechRecognition = new SpeechRecognition()
    speechRecognition.continuous = false
    speechRecognition.interimResults = true
    speechRecognition.lang = 'zh-CN'

    speechRecognition.onstart = () => {
      isListening.value = true
    }

    speechRecognition.onend = () => {
      isListening.value = false
    }

    speechRecognition.onresult = (event: any) => {
      const transcript = event.results[0][0].transcript
      // Handle transcript
      console.log('Recognized:', transcript)
    }

    speechRecognition.onerror = (event: any) => {
      console.error('Speech recognition error:', event.error)
      isListening.value = false
    }

    resolve(true)
  })
}

function startListening(onResult: (transcript: string) => void) {
  if (speechRecognition) {
    speechRecognition.onresult = (event: any) => {
      onResult(event.results[0][0].transcript)
    }
    speechRecognition.start()
  }
}

function stopListening() {
  if (speechRecognition) {
    speechRecognition.stop()
    isListening.value = false
  }
}

// ============ 课程内容 ============

const lessonLibrary = {
  'beam_analysis': {
    id: 'beam_analysis',
    title: '梁的静力学分析',
    description: '学习如何分析悬臂梁和简支梁的应力和位移',
    difficulty: 'beginner',
    steps: [
      {
        id: 'intro',
        title: '什么是梁？',
        description: '梁是工程中最常见的结构元件，用于承受弯曲载荷。',
        voiceScript: '梁是一种长条形的结构元件，主要承受弯曲载荷。在工程中，我们经常遇到悬臂梁和简支梁两种类型。悬臂梁一端固定，另一端自由。简支梁两端都有支撑。',
        keyPoints: ['梁承受弯曲载荷', '悬臂梁一端固定', '简支梁两端支撑']
      },
      {
        id: 'fixed_beam',
        title: '悬臂梁',
        description: '悬臂梁是最简单的静定结构，一端固定，一端自由。',
        visualElement: {
          type: 'diagram',
          content: '悬臂梁示意图：左端固定约束，右端承受集中载荷'
        },
        voiceScript: '看这个悬臂梁，左端固定在墙上，右端施加了一个向下的力。固定端可以阻止梁的转动和移动，所以那里会有弯矩和反力。',
        duration: 30,
        keyPoints: ['固定端有弯矩', '最大应力在固定端', '自由端位移最大']
      },
      {
        id: 'formulas',
        title: '关键公式',
        description: '学习梁分析的三个核心公式：弯矩、应力和挠度。',
        visualElement: {
          type: 'chart',
          content: 'M = -F*L (弯矩), σ = M*y/I (应力), δ = F*L³/(3EI) (挠度)'
        },
        voiceScript: '现在记住这三个公式。弯矩等于负的载荷乘以长度。最大应力等于弯矩乘以距离除以惯性矩。最大挠度等于载荷乘以长度的立方除以三倍的弹性模量乘以惯性矩。',
        duration: 45,
        keyPoints: ['弯矩公式', '应力公式', '挠度公式']
      }
    ]
  },

  'mesh_generation': {
    id: 'mesh_generation',
    title: '网格生成基础',
    description: '理解有限元网格的概念和生成方法',
    difficulty: 'beginner',
    steps: [
      {
        id: 'why_mesh',
        title: '为什么要网格？',
        description: '连续体无法直接用计算机计算，需要离散化。',
        voiceScript: '有限元分析的核心思想是把连续体分成很多小块，每一小块我们可以用简单的公式计算。这些小块就叫做单元，它们的集合就叫做网格。',
        keyPoints: ['连续体离散化', '单元和节点', '网格密度影响精度']
      },
      {
        id: 'element_types',
        title: '单元类型',
        description: '了解不同类型的有限元单元及其应用。',
        visualElement: {
          type: 'diagram',
          content: '一阶单元: 4节点四边形, 8节点六面体'
        },
        voiceScript: '最常用的单元是四边形单元和六面体单元。四边形单元用于二维问题，六面体单元用于三维问题。一阶单元有四个节点，二阶单元有八个节点，能更好地拟合曲线。',
        duration: 60
      }
    ]
  },

  'stress_analysis': {
    id: 'stress_analysis',
    title: '应力分析入门',
    description: '理解应力的概念、分类和计算方法',
    difficulty: 'beginner',
    steps: [
      {
        id: 'what_is_stress',
        title: '应力的定义',
        description: '应力是单位面积上的内力。',
        voiceScript: '应力是材料内部单位面积上的内力。当我们用力除以面积，就得到了应力。单位是帕斯卡，或者常用兆帕。',
        keyPoints: ['应力 = 力 / 面积', '单位: Pa, MPa', '分布不均匀时取最大值']
      },
      {
        id: 'von_mises',
        title: 'Von Mises 应力',
        description: '学习用于判断材料屈服的等效应力。',
        visualElement: {
          type: 'chart',
          content: 'Von Mises 应力云图显示高应力区域'
        },
        voiceScript: 'Von Mises 应力是一个等效应力，它把复杂的三维应力状态转换成一个单一的值。当这个值超过材料的屈服强度时，材料就会屈服。这就是我们判断结构是否安全的主要方法。',
        duration: 90,
        keyPoints: ['等效应力概念', '屈服准则', '安全系数']
      }
    ]
  }
}

// ============ 测验系统 ============

const quizBank: Record<string, QuizQuestion[]> = {
  beam_analysis: [
    {
      id: 'q1',
      question: '悬臂梁的最大弯矩发生在哪个位置？',
      options: ['自由端', '固定端', '跨中', '任意位置'],
      correctAnswer: 1,
      explanation: '悬臂梁的最大弯矩发生在固定端，因为那里距自由端的距离最远，弯矩等于载荷乘以长度。',
      difficulty: 'beginner'
    },
    {
      id: 'q2',
      question: '如果梁的长度增加一倍，最大挠度会增加多少倍？',
      options: ['2倍', '4倍', '8倍', '16倍'],
      correctAnswer: 2,
      explanation: '挠度公式 δ = FL³/(3EI)，长度增加一倍变成 2L，代入公式得 (2L)³ = 8L³，所以挠度增加 8 倍。',
      difficulty: 'intermediate'
    }
  ],
  mesh_generation: [
    {
      id: 'q1',
      question: '二阶单元比一阶单元的主要优势是什么？',
      options: ['节点更少', '计算更快', '能拟合曲线', '内存占用更低'],
      correctAnswer: 2,
      explanation: '二阶单元有中间节点，可以更好地拟合曲线和曲面，提高计算精度，特别适合曲边几何。',
      difficulty: 'intermediate'
    }
  ]
}

// ============ 教学逻辑 ============

function startLesson(lessonId: keyof typeof lessonLibrary) {
  const lesson = lessonLibrary[lessonId]
  if (!lesson) return

  currentLessonSteps.value = [...lesson.steps] as TeachingStep[]
  currentStepIndex.value = 0
  studentProgress.value.currentLesson = lessonId

  // 发送课程开始的对话
  addToConversation('ai', `我们开始学习"${lesson.title}"。${lesson.description}`)
}

function nextStep() {
  if (currentStepIndex.value < currentLessonSteps.value.length - 1) {
    currentStepIndex.value++
    const step = currentLessonSteps.value[currentStepIndex.value]
    if (step.voiceScript) {
      speak(step.voiceScript)
    }
  } else {
    // 课程结束
    completeLesson()
  }
}

function previousStep() {
  if (currentStepIndex.value > 0) {
    currentStepIndex.value--
  }
}

function completeLesson() {
  if (studentProgress.value.currentLesson) {
    studentProgress.value.completedLessons.push(studentProgress.value.currentLesson)
    studentProgress.value.currentLesson = null
  }
  addToConversation('ai', '恭喜你完成了这个课程！还有别的想学的吗？')
}

function addToConversation(role: 'ai' | 'student', content: string) {
  conversationHistory.value.push({
    role,
    content,
    time: new Date()
  })
}

function getQuizForLesson(lessonId: string): QuizQuestion[] {
  return quizBank[lessonId] || []
}

function submitQuizAnswer(questionId: string, answer: string | number): boolean {
  for (const lessonId of Object.keys(quizBank)) {
    const question = quizBank[lessonId].find(q => q.id === questionId)
    if (question) {
      const isCorrect = question.correctAnswer === answer
      if (isCorrect) {
        studentProgress.value.quizScores[questionId] = 1
      } else {
        studentProgress.value.quizScores[questionId] = 0
      }
      return isCorrect
    }
  }
  return false
}

// ============ AI 讲解生成 ============

function generateExplanation(topic: string, _level: DifficultyLevel): string {
  // 基于主题生成讲解内容
  const explanations: Record<string, string> = {
    'stress': '应力是材料内部由于外力作用而产生的内力。当外力作用在构件上时，构件内部会产生抵抗这种作用的力，这就是应力。计算公式是应力 = 力 / 面积，单位是帕斯卡（Pa）。',
    'strain': '应变是材料在外力作用下发生的相对变形。它是一个无量纲的量，通常用百分比或微应变表示。计算公式是应变 = 变形量 / 原始长度。正应变表示拉伸，负应变表示压缩。',
    'displacement': '位移是结构在载荷作用下发生的空间位置变化。在有限元分析中，我们计算每个节点的位移，然后根据位移推算应力和应变。最大位移通常发生在结构的最薄弱处或自由端。'
  }

  return explanations[topic] || '这个主题的讲解内容正在准备中...'
}

// ============ LLM 对话功能 (V4.0-006) ============

interface LLMConfig {
  apiEndpoint?: string
  model?: string
  maxTokens?: number
  temperature?: number
}

const llmConfig = ref<LLMConfig>({
  model: 'gpt-4',
  maxTokens: 500,
  temperature: 0.7
})

const isAILoading = ref(false)
const llmError = ref<string | null>(null)

/**
 * 发送消息给 LLM 并获取回复
 * 集成了 difficultyLevel 和 teachingMode 的影响
 */
async function sendLLMMessage(
  message: string,
  context?: {
    currentLesson?: string
    currentStep?: string
    userQuestion?: string
  }
): Promise<string> {
  isAILoading.value = true
  llmError.value = null

  try {
    // 构建系统提示词，包含难度级别和教学模式
    const systemPrompt = buildSystemPrompt()

    // 构建用户消息
    const userMessage = buildUserMessage(message, context)

    // 调用后端 LLM API
    const response = await callLLMAPI(systemPrompt, userMessage)

    // 添加到对话历史
    addToConversation('student', message)
    addToConversation('ai', response)

    return response
  } catch (error) {
    llmError.value = error instanceof Error ? error.message : 'LLM 调用失败'
    return `抱歉，AI 导师暂时无法回答您的问题。错误: ${llmError.value}`
  } finally {
    isAILoading.value = false
  }
}

/**
 * 根据 difficultyLevel 和 teachingMode 构建系统提示词
 */
function buildSystemPrompt(): string {
  const mode = tutoringMode.value
  const level = difficultyLevel.value

  const modeInstructions: Record<TutoringMode, string> = {
    explanation: '你是 CAELab 的 AI 教学助手，擅长用通俗易懂的语言解释有限元分析和 CAE 概念。请结合图示和例子进行讲解。',
    demonstration: '你是 CAELab 的 AI 教学助手，通过演示和步骤引导帮助学生学习。每个回复应该包含具体的操作步骤。',
    practice: '你是 CAELab 的 AI 教学助手，通过练习和问题引导学生主动思考。不要直接给出答案，而是引导用户自己找到解决方案。',
    quiz: '你是 CAELab 的 AI 教学助手，通过测验检验学生的学习效果。提出有挑战性的问题，并根据用户回答给予反馈。'
  }

  const levelInstructions: Record<DifficultyLevel, string> = {
    beginner: '请使用简单的语言，避免专业术语，假设学生没有任何基础。每解释一个概念都要从最基本的定义开始。',
    intermediate: '可以使用一些专业术语，假设学生有一定的基础知识。可以讨论更深层次的原理和应用。',
    advanced: '请使用精确的专业术语，讨论高级概念和最新研究进展。假设学生有扎实的理论基础和实践经验。'
  }

  return `${modeInstructions[mode]}\n\n${levelInstructions[level]}\n\n注意：\n- 用中文回答\n- 适当使用 Markdown 格式化\n- 如果需要，可以请求查看图示\n- 保持在 500 字以内`
}

/**
 * 构建用户消息，包含上下文信息
 */
function buildUserMessage(
  message: string,
  context?: {
    currentLesson?: string
    currentStep?: string
    userQuestion?: string
  }
): string {
  let fullMessage = message

  if (context?.currentLesson) {
    fullMessage = `[当前课程: ${context.currentLesson}]\n${fullMessage}`
  }

  if (context?.currentStep) {
    fullMessage = `[当前步骤: ${context.currentStep}]\n${fullMessage}`
  }

  return fullMessage
}

/**
 * 调用后端 LLM API
 * 实际实现需要根据后端 API 调整
 */
async function callLLMAPI(systemPrompt: string, userMessage: string): Promise<string> {
  // TODO: 实际调用后端 LLM API
  // 目前返回模拟响应
  try {
    // 模拟 API 调用
    // 实际应该使用 tauri.core.invoke 调用后端命令
    await new Promise(resolve => setTimeout(resolve, 1000))

    // 根据消息内容生成智能回复
    return generateSmartResponse(userMessage, systemPrompt)
  } catch {
    throw new Error('LLM API 调用失败')
  }
}

/**
 * 生成智能回复（当后端 API 不可用时的本地回退）
 */
function generateSmartResponse(message: string, _context: string): string {
  const lowerMessage = message.toLowerCase()

  // 基于关键词生成回复
  if (lowerMessage.includes('von mises') || lowerMessage.includes('应力')) {
    return `von Mises 应力是一种等效应力准则，用于判断材料是否进入塑性状态。

**公式：**
\`σ_v = √[(σ₁-σ₂)² + (σ₂-σ₃)² + (σ₃-σ₁)²] / 2\`

**通俗解释：**
想象你在拉扯一块橡皮泥。如果你向多个方向同时用力，橡皮泥可能会在某个方向先坏掉。von Mises 应力就是把所有方向的力"合并"成一个等效的拉力，让我们能够与材料的屈服强度直接比较。

**判断标准：**
- σ_v < σ_y：材料安全（弹性状态）
- σ_v ≥ σ_y：材料开始屈服（塑性状态）

需要我详细解释某个部分吗？`
  }

  if (lowerMessage.includes('泊松比')) {
    return `泊松比 ν 是描述材料横向变形的重要参数。

**定义：**
ν = - (横向应变 / 轴向应变)

**常见材料的泊松比：**
- 钢材：≈ 0.3
- 铝材：≈ 0.33
- 橡胶：≈ 0.5（接近不可压缩）
- 混凝土：≈ 0.2
- 软木：≈ 0（几乎没有横向收缩）

**物理意义：**
当你拉伸一根橡皮筋时，它会变细。泊松比描述的就是这种"变细"程度相对于"变长"程度的比值。

需要了解泊松比在有限元分析中的应用吗？`
  }

  if (lowerMessage.includes('网格') || lowerMessage.includes('mesh')) {
    return `网格是有限元分析的基础，它将连续的几何模型离散成有限个单元。

**关键概念：**

1. **网格密度**：网格越细，计算精度越高，但计算时间越长

2. **单元类型**：
   - Tet4/Tet10：四面体单元，适合复杂几何
   - Hex8/Hex20：六面体单元，精度高但需要规则几何

3. **网格质量指标**：
   - Jacobian：描述单元畸变程度
   - Aspect Ratio：长宽比
   - Skewness：歪斜度

**实用建议：**
- 应力集中区域需要加密网格
- 网格应该从粗到细逐步过渡
- 使用网格收敛性测试验证结果

需要我演示如何生成高质量网格吗？`
  }

  if (lowerMessage.includes('悬臂梁')) {
    return `悬臂梁是结构力学中最经典的案例之一。

**问题描述：**
梁的一端固定（固定端），另一端自由（自由端）。在自由端施加集中力 F。

**理论解：**
- 最大弯矩：M_max = F × L（发生在固定端）
- 最大剪力：V_max = F
- 最大挠度：δ_max = FL³/(3EI)

**仿真要点：**
1. 固定端需要约束所有自由度（Ux, Uy, Uz, Rx, Ry, Rz）
2. 集中力应施加在自由端面中心
3. 网格在固定端附近需要加密

**验证方法：**
仿真结果应该与理论解相差在 10% 以内。如果偏差过大，检查边界条件和网格质量。

想开始一个悬臂梁分析的实践练习吗？`
  }

  // 默认回复
  return `您的问题是："${message}"

作为 CAELab 的 AI 教学助手，我建议您：

1. 如果您是初学者，可以从"仿真向导"开始，选择"悬臂梁分析"作为第一个案例
2. 如果您想了解特定概念，可以直接询问（如"什么是 von Mises 应力"）
3. 如果您在实际操作中遇到问题，可以描述具体的错误信息

有什么具体问题我可以帮您解答吗？`
}

// 设置 LLM 配置
function setLLMConfig(config: Partial<LLMConfig>) {
  llmConfig.value = { ...llmConfig.value, ...config }
}

// ============ 导出 ============

export function useAITutor() {
  return {
    // 状态
    tutoringMode,
    difficultyLevel,
    isSpeaking,
    isListening,
    currentLessonSteps,
    currentStepIndex,
    studentProgress,
    conversationHistory,
    lessonLibrary,

    // 语音功能
    speak,
    stopSpeaking,
    pauseSpeaking,
    resumeSpeaking,
    initSpeechRecognition,
    startListening,
    stopListening,

    // 课程功能
    startLesson,
    nextStep,
    previousStep,
    completeLesson,
    getQuizForLesson,
    submitQuizAnswer,

    // AI 讲解
    generateExplanation,
    addToConversation,

    // LLM 功能 (V4.0-006)
    sendLLMMessage,
    setLLMConfig,
    isAILoading,
    llmError,

    // 计算属性
    currentStep: computed(() => currentLessonSteps.value[currentStepIndex.value]),
    totalSteps: computed(() => currentLessonSteps.value.length),
    progressPercent: computed(() =>
      currentLessonSteps.value.length > 0
        ? (currentStepIndex.value / (currentLessonSteps.value.length - 1)) * 100
        : 0
    ),

    // 能力检测
    capabilities: computed<AICapability>(() => ({
      tts: typeof window !== 'undefined' && 'speechSynthesis' in window,
      stt: typeof window !== 'undefined' && (
        'SpeechRecognition' in window || 'webkitSpeechRecognition' in window
      ),
      vision: true,  // 假设有视觉能力
      reasoning: true
    }))
  }
}