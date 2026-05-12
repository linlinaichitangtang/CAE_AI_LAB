/**
 * useErrorTranslator.ts — V2.9-003 错误提示中文化
 * 将 Rust/求解器错误翻译为中文，并提供可能原因和解决建议
 */

import { ref } from 'vue'

export interface TranslatedError {
  message: string       // 中文错误信息
  possibleCause: string // 可能原因
  suggestedAction: string // 建议操作
  severity: 'error' | 'warning' | 'info'
}

// 错误模式映射表
const errorPatterns: Array<{
  pattern: RegExp
  translation: Omit<TranslatedError, 'message'>
  template: string
}> = [
  // 网格相关错误
  {
    pattern: /mesh.*generation.*failed|mesh.*error|网格生成/i,
    template: '网格生成失败',
    translation: {
      possibleCause: '网格参数设置不当或几何模型存在问题',
      suggestedAction: '检查网格划分数量是否合理，尝试降低网格密度',
      severity: 'error'
    }
  },
  {
    pattern: /element.*quality|网格质量|mesh.*quality/i,
    template: '网格质量不佳',
    translation: {
      possibleCause: '单元纵横比过大或雅可比矩阵为负',
      suggestedAction: '尝试使用更细的网格或调整网格疏密分布',
      severity: 'warning'
    }
  },
  {
    pattern: /node.*count|节点.*超|too many nodes/i,
    template: '节点数量超出限制',
    translation: {
      possibleCause: '模型规模太大，超过了当前求解器的处理能力',
      suggestedAction: '减少网格数量，或使用云端 HPC 进行大规模计算',
      severity: 'error'
    }
  },

  // 求解器相关错误
  {
    pattern: /solver.*fail|solver.*error|求解器.*失败|求解失败/i,
    template: '求解器执行失败',
    translation: {
      possibleCause: '求解器配置错误、材料参数无效或边界条件矛盾',
      suggestedAction: '检查材料参数是否合理，确认边界条件是否完整',
      severity: 'error'
    }
  },
  {
    pattern: /ccx.*not.*found|calculix.*未找到|求解器未安装/i,
    template: '求解器未安装',
    translation: {
      possibleCause: 'CalculiX 求解器未正确安装或路径配置错误',
      suggestedAction: '请在设置中安装 CalculiX 求解器',
      severity: 'error'
    }
  },
  {
    pattern: /divergence|不收敛|收敛失败/i,
    template: '求解不收敛',
    translation: {
      possibleCause: '模型存在物理问题（如奇异边界条件）或网格质量差',
      suggestedAction: '检查边界条件是否合理，尝试减少载荷步长或加密网格',
      severity: 'error'
    }
  },
  {
    pattern: /out.*memory|memory.*error|内存.*不足|内存溢出/i,
    template: '内存不足',
    translation: {
      possibleCause: '模型规模太大，当前设备内存不足以完成计算',
      suggestedAction: '减少网格数量、使用更少的单元类型，或切换到云端 HPC',
      severity: 'error'
    }
  },

  // 材料相关错误
  {
    pattern: /material.*error|材料.*错误|elastic.*modulus.*invalid/i,
    template: '材料参数无效',
    translation: {
      possibleCause: '弹性模量、泊松比等参数设置不当',
      suggestedAction: '检查材料参数：弹性模量需为正数，泊松比需在 0~0.5 之间',
      severity: 'error'
    }
  },
  {
    pattern: /yield.*strength.*exceed|屈服强度.*超限/i,
    template: '材料屈服强度超限',
    translation: {
      possibleCause: '计算应力超过材料屈服强度，可能发生塑性变形',
      suggestedAction: '考虑使用弹塑性材料模型或更换更高强度的材料',
      severity: 'warning'
    }
  },

  // 边界条件相关错误
  {
    pattern: /boundary.*condition|边界条件.*错误|boundary.*missing/i,
    template: '边界条件错误或缺失',
    translation: {
      possibleCause: '模型边界条件不完整，存在欠约束或过约束情况',
      suggestedAction: '确保模型有足够的约束（通常需要固定至少一个点/面）',
      severity: 'error'
    }
  },
  {
    pattern: /overconstraint|过约束|redundant.*constraint/i,
    template: '边界条件过约束',
    translation: {
      possibleCause: '部分节点被重复约束，导致刚度矩阵奇异',
      suggestedAction: '移除冗余的边界条件，保留必要的约束',
      severity: 'error'
    }
  },

  // 文件 I/O 相关错误
  {
    pattern: /file.*not.*found|文件未找到|file.*not.*exist/i,
    template: '文件未找到',
    translation: {
      possibleCause: '指定的输入文件或数据文件不存在',
      suggestedAction: '检查文件路径是否正确，确保文件存在且可读',
      severity: 'error'
    }
  },
  {
    pattern: /permission.*denied|权限.*拒绝|无法.*写入/i,
    template: '文件权限错误',
    translation: {
      possibleCause: '没有写入权限或文件被占用',
      suggestedAction: '检查文件是否被其他程序占用，或以管理员身份运行',
      severity: 'error'
    }
  },
  {
    pattern: /file.*corrupt|文件.*损坏|inp.*parse.*error/i,
    template: '文件格式错误或损坏',
    translation: {
      possibleCause: '文件格式不符合预期或内容损坏',
      suggestedAction: '检查文件内容是否完整，尝试重新导入',
      severity: 'error'
    }
  },

  // 接触分析错误
  {
    pattern: /contact.*fail|接触.*失败|接触.*error/i,
    template: '接触分析失败',
    translation: {
      possibleCause: '接触对设置不当或接触刚度参数不合理',
      suggestedAction: '检查接触对定义，降低接触刚度或调整算法',
      severity: 'error'
    }
  },

  // 通用错误
  {
    pattern: /timeout|超时/i,
    template: '计算超时',
    translation: {
      possibleCause: '求解时间过长，超过了预设的超时限制',
      suggestedAction: '简化模型、减少网格数量或增加超时时间',
      severity: 'error'
    }
  },
  {
    pattern: /network.*error|网络.*错误|connection.*fail/i,
    template: '网络连接失败',
    translation: {
      possibleCause: '无法连接到云端服务器或外部服务',
      suggestedAction: '检查网络连接，或切换到离线模式',
      severity: 'error'
    }
  }
]

/**
 * 翻译错误信息
 */
export function useErrorTranslator() {
  const lastError = ref<TranslatedError | null>(null)

  /**
   * 翻译一个错误字符串
   */
  function translateError(error: unknown): TranslatedError {
    const errorStr = String(error)

    // 遍历模式列表，找到匹配的错误模式
    for (const item of errorPatterns) {
      if (item.pattern.test(errorStr)) {
        const result: TranslatedError = {
          message: item.template,
          possibleCause: item.translation.possibleCause,
          suggestedAction: item.translation.suggestedAction,
          severity: item.translation.severity
        }
        lastError.value = result
        return result
      }
    }

    // 未匹配到任何模式，返回通用错误
    const result: TranslatedError = {
      message: '操作失败',
      possibleCause: '发生了未知错误，可能是程序内部异常',
      suggestedAction: '请查看详细错误信息，或联系技术支持',
      severity: 'error'
    }
    lastError.value = result
    return result
  }

  /**
   * 获取用户友好的错误字符串（用于直接显示）
   */
  function getFriendlyError(error: unknown): string {
    const translated = translateError(error)
    return `[${translated.message}] ${translated.possibleCause}。${translated.suggestedAction}`
  }

  /**
   * 清除错误
   */
  function clearError() {
    lastError.value = null
  }

  return {
    lastError,
    translateError,
    getFriendlyError,
    clearError
  }
}