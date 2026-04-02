/**
 * Diff 工具模块 - 基于 LCS（最长公共子序列）的行级文本对比
 * 不引入额外依赖，纯 TypeScript 实现
 */

export interface DiffLine {
  type: 'add' | 'remove' | 'unchanged'
  content: string
  oldLineNum?: number
  newLineNum?: number
}

export interface DiffResult {
  lines: DiffLine[]
  stats: {
    additions: number
    deletions: number
    unchanged: number
  }
}

/**
 * 将文本按行分割，统一换行符
 */
function splitLines(text: string): string[] {
  if (!text) return []
  // 统一换行符为 \n，然后分割
  return text.replace(/\r\n/g, '\n').replace(/\r/g, '\n').split('\n')
}

/**
 * 计算 LCS（最长公共子序列）的长度矩阵
 * 使用动态规划，空间优化为 O(min(m,n))
 */
function computeLCSLengths(oldLines: string[], newLines: string[]): number[][] {
  const m = oldLines.length
  const n = newLines.length

  // 使用完整矩阵以便回溯
  const dp: number[][] = Array.from({ length: m + 1 }, () =>
    new Array(n + 1).fill(0)
  )

  for (let i = 1; i <= m; i++) {
    for (let j = 1; j <= n; j++) {
      if (oldLines[i - 1] === newLines[j - 1]) {
        dp[i][j] = dp[i - 1][j - 1] + 1
      } else {
        dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1])
      }
    }
  }

  return dp
}

/**
 * 回溯 LCS 矩阵，生成 diff 行列表
 */
function backtrackDiff(
  dp: number[][],
  oldLines: string[],
  newLines: string[]
): DiffLine[] {
  const result: DiffLine[] = []
  let i = oldLines.length
  let j = newLines.length

  // 先反向生成，然后翻转
  const reversed: DiffLine[] = []

  while (i > 0 || j > 0) {
    if (i > 0 && j > 0 && oldLines[i - 1] === newLines[j - 1]) {
      // 两行相同，属于 LCS
      reversed.push({
        type: 'unchanged',
        content: oldLines[i - 1],
        oldLineNum: i,
        newLineNum: j
      })
      i--
      j--
    } else if (j > 0 && (i === 0 || dp[i][j - 1] >= dp[i - 1][j])) {
      // 新增行
      reversed.push({
        type: 'add',
        content: newLines[j - 1],
        newLineNum: j
      })
      j--
    } else if (i > 0) {
      // 删除行
      reversed.push({
        type: 'remove',
        content: oldLines[i - 1],
        oldLineNum: i
      })
      i--
    }
  }

  // 翻转得到正确顺序
  return reversed.reverse()
}

/**
 * 计算两个文本之间的差异
 *
 * @param oldText 旧版本文本
 * @param newText 新版本文本
 * @returns DiffResult 包含差异行和统计信息
 */
export function computeDiff(oldText: string, newText: string): DiffResult {
  const oldLines = splitLines(oldText)
  const newLines = splitLines(newText)

  // 如果任一文本为空，直接返回全增/全删
  if (oldLines.length === 0 && newLines.length === 0) {
    return {
      lines: [],
      stats: { additions: 0, deletions: 0, unchanged: 0 }
    }
  }

  if (oldLines.length === 0) {
    return {
      lines: newLines.map((line, idx) => ({
        type: 'add' as const,
        content: line,
        newLineNum: idx + 1
      })),
      stats: { additions: newLines.length, deletions: 0, unchanged: 0 }
    }
  }

  if (newLines.length === 0) {
    return {
      lines: oldLines.map((line, idx) => ({
        type: 'remove' as const,
        content: line,
        oldLineNum: idx + 1
      })),
      stats: { additions: 0, deletions: oldLines.length, unchanged: 0 }
    }
  }

  // 计算 LCS 矩阵
  const dp = computeLCSLengths(oldLines, newLines)

  // 回溯生成 diff
  const lines = backtrackDiff(dp, oldLines, newLines)

  // 统计
  const stats = {
    additions: 0,
    deletions: 0,
    unchanged: 0
  }

  for (const line of lines) {
    switch (line.type) {
      case 'add':
        stats.additions++
        break
      case 'remove':
        stats.deletions++
        break
      case 'unchanged':
        stats.unchanged++
        break
    }
  }

  return { lines, stats }
}

/**
 * 将 DiffResult 转换为并排对比视图所需的左右行数据
 */
export interface SideBySideLine {
  left: {
    type: 'remove' | 'unchanged' | 'empty'
    content: string
    lineNum?: number
  }
  right: {
    type: 'add' | 'unchanged' | 'empty'
    content: string
    lineNum?: number
  }
}

/**
 * 将 diff 行转换为并排对比格式
 * 删除行和新增行会配对显示，未变行直接对齐
 */
export function toSideBySide(diffResult: DiffResult): SideBySideLine[] {
  const result: SideBySideLine[] = []
  const lines = diffResult.lines

  let i = 0
  while (i < lines.length) {
    const line = lines[i]

    if (line.type === 'unchanged') {
      result.push({
        left: {
          type: 'unchanged',
          content: line.content,
          lineNum: line.oldLineNum
        },
        right: {
          type: 'unchanged',
          content: line.content,
          lineNum: line.newLineNum
        }
      })
      i++
    } else if (line.type === 'remove') {
      // 收集连续的删除行
      const removes: DiffLine[] = []
      while (i < lines.length && lines[i].type === 'remove') {
        removes.push(lines[i])
        i++
      }
      // 收集紧接着的连续新增行
      const adds: DiffLine[] = []
      while (i < lines.length && lines[i].type === 'add') {
        adds.push(lines[i])
        i++
      }

      // 配对删除和新增行
      const maxLen = Math.max(removes.length, adds.length)
      for (let j = 0; j < maxLen; j++) {
        result.push({
          left: j < removes.length
            ? {
                type: 'remove',
                content: removes[j].content,
                lineNum: removes[j].oldLineNum
              }
            : {
                type: 'empty',
                content: ''
              },
          right: j < adds.length
            ? {
                type: 'add',
                content: adds[j].content,
                lineNum: adds[j].newLineNum
              }
            : {
                type: 'empty',
                content: ''
              }
        })
      }
    } else if (line.type === 'add') {
      // 独立的新增行（前面没有删除行）
      result.push({
        left: {
          type: 'empty',
          content: ''
        },
        right: {
          type: 'add',
          content: line.content,
          lineNum: line.newLineNum
        }
      })
      i++
    }
  }

  return result
}

/**
 * 对 HTML 内容进行文本化处理，用于 diff 对比
 * 去除多余空白，保留结构信息
 */
export function normalizeHtmlForDiff(html: string): string {
  if (!html) return ''
  // 将 HTML 标签转为单行，保留标签结构
  return html
    .replace(/\r\n/g, '\n')
    .replace(/\r/g, '\n')
    // 将多个连续空格压缩为一个
    .replace(/[ \t]+/g, ' ')
    // 去除行首行尾空白
    .split('\n')
    .map(line => line.trim())
    .join('\n')
    // 去除多余空行（保留最多一个连续空行）
    .replace(/\n{3,}/g, '\n\n')
    .trim()
}
