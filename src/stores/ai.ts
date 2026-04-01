/**
 * AI Store - 全局AI对话和配置存储
 * 支持多种AI源:
 * - third-party: 第三方平台API (OpenAI, 豆包, 通义千问等)
 * - local: 本地部署大模型 (Ollama, Llama.cpp等)
 * - platform: 陛下提供的算力服务 (收费模式)
 */

import { defineStore } from 'pinia'

export type AiSource = 'third-party' | 'local' | 'platform'

export interface Message {
  role: 'user' | 'assistant' | 'system'
  content: string
  timestamp: number
}

export interface AiConfig {
  aiSource: AiSource
  apiUrl: string
  apiKey: string
  modelName: string
  temperature: number
  maxTokens: number
}

export interface ContextData {
  currentTool: 'notes' | 'modeling' | 'code' | 'simulation' | 'ai'
  projectName?: string
  // 笔记上下文
  noteContent?: string
  // 建模上下文
  modelingGeometry?: {
    type: string
    dimensions: { width: number; height: number; depth: number }
    divisions: { divX: number; divY: number; divZ: number }
  }
  // 代码上下文
  codeContent?: string
  codeLanguage?: string
  // 仿真上下文
  simulationMesh?: {
    nodeCount: number
    elementCount: number
  }
  simulationResult?: {
    maxStress: number
    maxDisplacement: number
  }
}

interface AiState {
  messages: Message[]
  config: AiConfig
  context: ContextData
  isGenerating: boolean
  currentStreamingText: string
}

// 默认配置
const defaultConfig: AiConfig = {
  aiSource: 'third-party',
  apiUrl: 'https://api.openai.com/v1/chat/completions',
  apiKey: '',
  modelName: 'gpt-3.5-turbo',
  temperature: 0.7,
  maxTokens: 2000
}

// 默认上下文
const defaultContext: ContextData = {
  currentTool: 'ai'
}

export const useAiStore = defineStore('ai', {
  state: (): AiState => ({
    messages: [],
    config: { ...defaultConfig },
    context: { ...defaultContext },
    isGenerating: false,
    currentStreamingText: ''
  }),

  getters: {
    hasMessages: (state) => state.messages.length > 0,
    isConfigured: (state) => state.config.apiKey.length > 0 || state.config.aiSource !== 'third-party',
    // 获取完整prompt，包含系统提示和上下文
    fullMessagesWithContext: (state): Message[] => {
      const systemPrompt = buildSystemPrompt(state.context)
      const systemMessage: Message = {
        role: 'system',
        content: systemPrompt,
        timestamp: Date.now()
      }
      return [systemMessage, ...state.messages]
    }
  },

  actions: {
    /** 加载配置从本地存储 */
    loadConfig() {
      const saved = localStorage.getItem('caelab_ai_config')
      if (saved) {
        try {
          this.config = { ...defaultConfig, ...JSON.parse(saved) }
        } catch (e) {
          console.error('Failed to load AI config:', e)
          this.config = { ...defaultConfig }
        }
      }
    },

    /** 保存配置到本地存储 */
    saveConfig() {
      localStorage.setItem('caelab_ai_config', JSON.stringify(this.config))
    },

    /** 更新配置 */
    updateConfig(newConfig: Partial<AiConfig>) {
      this.config = { ...this.config, ...newConfig }
      this.saveConfig()
    },

    /** 更新当前上下文 */
    updateContext(newContext: Partial<ContextData>) {
      this.context = { ...this.context, ...newContext }
    },

    /** 添加用户消息 */
    addUserMessage(content: string) {
      this.messages.push({
        role: 'user',
        content,
        timestamp: Date.now()
      })
    },

    /** 添加助手消息 */
    addAssistantMessage(content: string) {
      this.messages.push({
        role: 'assistant',
        content,
        timestamp: Date.now()
      })
    },

    /** 发送消息并获取AI响应 */
    async sendMessage(prompt: string): Promise<{ response: string }> {
      this.addUserMessage(prompt)
      try {
        const { invoke } = await import('@tauri-apps/api/core')
        const response = await invoke('ai_chat', { message: prompt, context: this.context })
        const text = typeof response === 'string' ? response : JSON.stringify(response)
        this.addAssistantMessage(text)
        return { response: text }
      } catch (error) {
        const errorMsg = String(error)
        this.addAssistantMessage(`错误: ${errorMsg}`)
        return { response: '' }
      }
    },

    /** 开始流式生成 */
    startGeneration() {
      this.isGenerating = true
      this.currentStreamingText = ''
    },

    /** 更新流式文本 */
    updateStreamingText(text: string) {
      this.currentStreamingText = text
    },

    /** 结束流式生成，保存最终结果 */
    finishGeneration() {
      if (this.currentStreamingText.trim()) {
        this.addAssistantMessage(this.currentStreamingText)
      }
      this.isGenerating = false
      this.currentStreamingText = ''
    },

    /** 清空对话 */
    clearMessages() {
      this.messages = []
      this.currentStreamingText = ''
      this.isGenerating = false
    },

    /** 取消当前生成 */
    cancelGeneration() {
      this.isGenerating = false
      this.currentStreamingText = ''
    },

    /** 重置为默认配置 */
    resetConfig() {
      this.config = { ...defaultConfig }
      this.saveConfig()
    }
  }
})

/** 构建系统提示词，包含当前上下文 */
function buildSystemPrompt(context: ContextData): string {
  let prompt = `你是CAELab助手，CAELab是一个集成了笔记、3D建模、CAE仿真、代码编辑的一体化科研工程平台。\n`
  prompt += `你正在帮助用户进行工作，请根据当前上下文回答问题，给出专业建议。\n\n`

  prompt += `当前位置: ${getToolName(context.currentTool)}\n`

  if (context.projectName) {
    prompt += `当前项目: ${context.projectName}\n`
  }

  // 根据不同工具添加对应上下文
  switch (context.currentTool) {
    case 'notes':
      if (context.noteContent) {
        prompt += `\n当前笔记内容:\n${context.noteContent}\n`
      }
      break
    case 'modeling':
      if (context.modelingGeometry) {
        const g = context.modelingGeometry
        prompt += `\n当前几何体: ${g.type}, 尺寸: ${g.dimensions.width}×${g.dimensions.height}×${g.dimensions.depth}, 分段: ${g.divisions.divX}×${g.divisions.divY}×${g.divisions.divZ}\n`
      }
      break
    case 'code':
      if (context.codeContent && context.codeLanguage) {
        prompt += `\n当前代码 (${context.codeLanguage}):\n\`\`\`${context.codeLanguage}\n${context.codeContent}\n\`\`\`\n`
      }
      break
    case 'simulation':
      if (context.simulationMesh) {
        prompt += `\n当前网格: ${context.simulationMesh.nodeCount} 节点, ${context.simulationMesh.elementCount} 单元\n`
      }
      if (context.simulationResult) {
        prompt += `计算结果: 最大应力 ${context.simulationResult.maxStress} MPa, 最大位移 ${context.simulationResult.maxDisplacement} mm\n`
      }
      break
  }

  prompt += `\n请专业、简洁地回答用户问题，如果可以，给出具体操作建议或代码。\n`

  return prompt
}

function getToolName(tool: ContextData['currentTool']): string {
  const names: Record<ContextData['currentTool'], string> = {
    notes: '笔记编辑',
    modeling: '3D建模',
    code: '代码编辑',
    simulation: 'CAE仿真',
    ai: 'AI助手'
  }
  return names[tool] || tool
}
