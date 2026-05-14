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