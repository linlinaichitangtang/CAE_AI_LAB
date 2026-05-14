/**
 * contextCompressor.ts — V2.4 上下文压缩器
 *
 * Claude Code 对齐：当对话历史过长时，压缩旧消息而非简单截断
 * 保留最近 maxMessages 条原文，中间消息生成摘要
 */

import type { AgentMessage, CompressionConfig, CompressedContext } from './types'

const DEFAULT_CONFIG: CompressionConfig = {
  maxMessages: 50,
  summaryThreshold: 100,
  aggressiveSummary: false,
}

// 需要永久保留的消息类型（不参与压缩）
const PRESERVE_TYPES = ['user_clarification', 'confirmed_action', 'final_result']

function isPreservedMessage(msg: AgentMessage): boolean {
  if (!msg.metadata) return false
  const msgType = msg.metadata.type as string
  return PRESERVE_TYPES.includes(msgType)
}

export class ContextCompressor {
  private config: CompressionConfig

  constructor(config: Partial<CompressionConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config }
  }

  /**
   * 检查是否需要压缩
   */
  needsCompression(messages: AgentMessage[]): boolean {
    return messages.length > this.config.summaryThreshold
  }

  /**
   * 压缩上下文
   */
  compress(messages: AgentMessage[]): CompressedContext {
    if (!this.needsCompression(messages)) {
      return {
        summary: '',
        messageCount: messages.length,
        compressedCount: 0,
        preservedMessages: messages,
        compressionRatio: 1,
      }
    }

    // 分离保留和可压缩的消息
    const { preserved, compressible } = this.splitMessages(messages)

    // 生成摘要
    const summary = this.generateSummary(compressible)

    return {
      summary,
      messageCount: messages.length,
      compressedCount: compressible.length,
      preservedMessages: preserved,
      compressionRatio: (messages.length - compressible.length) / messages.length,
    }
  }

  /**
   * 分离永久保留消息和可压缩消息
   */
  private splitMessages(messages: AgentMessage[]): {
    preserved: AgentMessage[]
    compressible: AgentMessage[]
  } {
    const preserved: AgentMessage[] = []
    const compressible: AgentMessage[] = []

    for (const msg of messages) {
      if (isPreservedMessage(msg)) {
        preserved.push(msg)
      } else {
        compressible.push(msg)
      }
    }

    // 总是保留最近 maxMessages 条（即使是 compressible 类型）
    if (compressible.length > this.config.maxMessages) {
      const keep = compressible.slice(-this.config.maxMessages)
      const toCompress = compressible.slice(0, -this.config.maxMessages)
      preserved.push(...keep)
      compressible.length = 0
      compressible.push(...toCompress)
    } else {
      preserved.push(...compressible)
      compressible.length = 0
    }

    return { preserved, compressible }
  }

  /**
   * 按 taskId 分组生成摘要
   */
  private generateSummary(messages: AgentMessage[]): string {
    if (messages.length === 0) return ''

    const byTask = new Map<string, AgentMessage[]>()

    for (const msg of messages) {
      const taskId = msg.taskId ?? 'unknown'
      if (!byTask.has(taskId)) byTask.set(taskId, [])
      byTask.get(taskId)!.push(msg)
    }

    const parts: string[] = []

    for (const [taskId, taskMsgs] of byTask) {
      const toolCalls = taskMsgs.filter(m => m.toolCalls && m.toolCalls.length > 0)
      const completed = taskMsgs.filter(
        m => m.role === 'assistant' && m.content.includes('完成')
      )
      const failed = taskMsgs.filter(
        m => m.role === 'assistant' && m.content.includes('失败')
      )

      const outcome = failed.length > 0 ? 'failed' : completed.length > 0 ? 'success' : 'incomplete'
      parts.push(
        `[Task ${taskId}]: ${toolCalls.length} tool calls, outcome: ${outcome}`
      )
    }

    return `[Compressed ${messages.length} messages] ${parts.join(' | ')}`
  }

  /**
   * 更新配置
   */
  updateConfig(config: Partial<CompressionConfig>): void {
    this.config = { ...this.config, ...config }
  }

  /**
   * 获取当前配置
   */
  getConfig(): CompressionConfig {
    return { ...this.config }
  }
}