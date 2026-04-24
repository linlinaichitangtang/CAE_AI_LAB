/**
 * V2.4-001 意图识别引擎
 * 基于规则 + 关键词匹配的意图分类器
 * 目标准确率 > 90%
 */

import type { IntentType, IntentResult } from './types'

/** 意图关键词映射 */
const INTENT_KEYWORDS: Record<IntentType, { keywords: string[]; patterns: RegExp[]; subIntents: Record<string, string[]> }> = {
  modeling: {
    keywords: ['创建', '画', '建模', '几何', '立方体', '球体', '圆柱', '圆锥', '齿轮', 'CSG', '布尔', '拉伸', '旋转', '扫掠', '模型', '体素', '网格', 'mesh', '几何体', '导入', 'STEP', 'IGES', 'STL'],
    patterns: [/创建?\s*(几何|模型|立方体|球体|圆柱|圆锥|齿轮)/i, /画\s*(一个|齿轮|齿形|轮廓|曲线)/i, /导入\s*(STEP|IGES|STL|几何)/i, /CSG\s*(布尔|并|交|差)/i, /生成\s*(网格|mesh)/i],
    subIntents: {
      'modeling.geometry': ['几何', '立方体', '球体', '圆柱', '圆锥', '齿轮', '轮廓', '曲线'],
      'modeling.mesh': ['网格', 'mesh', '划分', '单元'],
      'modeling.import': ['导入', 'STEP', 'IGES', 'STL', 'import'],
      'modeling.csg': ['CSG', '布尔', '并集', '交集', '差集', 'union', 'intersect', 'subtract'],
    }
  },
  simulation: {
    keywords: ['仿真', '分析', '求解', '计算', '应力', '应变', '位移', '模态', '频率', '屈曲', '热', '温度', '流体', 'CFD', '静力', '动力', '瞬态', '稳态', '线性', '非线性', '接触', '疲劳', '振动', 'FEM', 'FEA', 'CAE', 'CalculiX', 'Abaqus'],
    patterns: [/分析?\s*(应力|应变|位移|模态|频率|屈曲|热|温度|疲劳|振动)/i, /运行?\s*(仿真|求解|计算|分析)/i, /提交?\s*(求解|仿真|任务)/i, /静力?\s*(分析|仿真)/i, /模态?\s*(分析|仿真)/i, /热?\s*(分析|仿真|耦合)/i, /疲劳?\s*(分析|寿命|评估)/i, /屈曲?\s*(分析|仿真)/i],
    subIntents: {
      'simulation.static': ['静力', '静强度', '应力', '应变', '位移', 'static'],
      'simulation.modal': ['模态', '频率', '固有', '振动', 'modal', 'natural frequency'],
      'simulation.thermal': ['热', '温度', '热传导', 'thermal', 'temperature'],
      'simulation.buckling': ['屈曲', '失稳', 'buckling'],
      'simulation.fatigue': ['疲劳', '寿命', 'S-N', 'fatigue'],
      'simulation.transient': ['瞬态', '动态', '动力', 'transient', 'dynamic'],
      'simulation.contact': ['接触', 'contact'],
      'simulation.cfd': ['CFD', '流体', 'fluid'],
    }
  },
  postprocess: {
    keywords: ['后处理', '云图', '等值线', '切片', '动画', '导出', 'CSV', 'JSON', 'VTK', '结果', '可视化', '渲染', 'contour', 'plot', '图表', '曲线'],
    patterns: [/显示?\s*(云图|等值线|结果|应力|位移)/i, /导出?\s*(结果|CSV|JSON|VTK|数据)/i, /生成?\s*(图表|曲线|动画|报告)/i, /切片?\s*(查看|显示)/i, /后处理/i],
    subIntents: {
      'postprocess.contour': ['云图', '等值线', 'contour'],
      'postprocess.export': ['导出', 'CSV', 'JSON', 'VTK', 'export'],
      'postprocess.animation': ['动画', 'animation'],
      'postprocess.slice': ['切片', 'slice'],
      'postprocess.chart': ['图表', '曲线', 'plot', 'chart'],
    }
  },
  code: {
    keywords: ['代码', '脚本', 'Python', '脚本', '编程', '函数', '写', '生成代码', '自动生成', 'Git', 'commit', '文件', '读写', '执行', '运行脚本'],
    patterns: [/写?\s*(一个|Python|脚本|代码|函数)/i, /生成?\s*(代码|脚本|Python)/i, /帮我?\s*(写|生成|创建)\s*(代码|脚本)/i, /Git\s*(commit|status|diff)/i, /执行?\s*(代码|脚本|命令)/i],
    subIntents: {
      'code.generate': ['生成', '写', '创建', 'generate'],
      'code.execute': ['执行', '运行', 'execute', 'run'],
      'code.git': ['Git', 'commit', 'status', 'diff'],
      'code.file': ['文件', '读写', 'file'],
    }
  },
  analysis: {
    keywords: ['为什么', '原因', '诊断', '对比', '比较', '检查', '验证', '不一致', '异常', '问题', '分析结果', '解释', '诊断报告'],
    patterns: [/为什么\s*(应力|位移|温度|结果)/i, /分析?\s*(原因|为什么|不一致|异常)/i, /对比?\s*(两种|结果|情况)/i, /检查?\s*(边界条件|几何|网格|设置)/i, /验证?\s*(结果|数值|正确性)/i, /诊断?\s*(问题|错误)/i],
    subIntents: {
      'analysis.diagnose': ['诊断', '为什么', '原因', '问题'],
      'analysis.compare': ['对比', '比较', 'compare'],
      'analysis.validate': ['验证', '检查', 'validate'],
    }
  },
  notes: {
    keywords: ['笔记', '记录', '总结', '公式', 'LaTeX', '文档', '备忘', '笔记'],
    patterns: [/创建?\s*(笔记|记录|文档)/i, /插入?\s*(公式|LaTeX)/i, /总结?\s*(分析|仿真|结果)/i],
    subIntents: {
      'notes.create': ['创建', '新建'],
      'notes.formula': ['公式', 'LaTeX'],
      'notes.summary': ['总结', '归纳'],
    }
  },
  parametric: {
    keywords: ['参数化', '扫描', '参数', '灵敏度', 'DOE', '不同', '影响', '变化', '范围', 'sweep', 'parametric'],
    patterns: [/参数化?\s*(扫描|分析|研究)/i, /不同\s*(厚度|尺寸|参数|材料)\s*(对|的)\s*(影响|应力|结果)/i, /灵敏度?\s*(分析)/i, /DOE\s*(实验|设计|研究)/i],
    subIntents: {}
  },
  optimization: {
    keywords: ['优化', '拓扑', '形状', '尺寸', '轻量化', '目标', '约束', 'SIMP', 'OC', 'optimization', 'topology'],
    patterns: [/优化?\s*(拓扑|形状|尺寸|结构)/i, /拓扑?\s*(优化)/i, /轻量化/i],
    subIntents: {
      'optimization.topology': ['拓扑', 'topology'],
      'optimization.shape': ['形状', 'shape'],
      'optimization.size': ['尺寸', 'size'],
    }
  },
  validation: {
    keywords: ['验证', '基准', 'benchmark', '对比', '理论解', '解析解', '误差', '精度', '收敛', '网格收敛'],
    patterns: [/验证?\s*(结果|精度|网格收敛)/i, /基准?\s*(测试|对比|算例)/i, /对比?\s*(理论解|解析解|解析)/i, /误差?\s*(分析|评估)/i],
    subIntents: {}
  },
  qa: {
    keywords: ['什么是', '怎么', '如何', '为什么是', '解释', '概念', '原理', '定义', '公式推导'],
    patterns: [/什么是\s*(有限元|CAE|应力|应变)/i, /如何?\s*(使用|设置|配置|理解)/i, /解释?\s*(概念|原理|公式)/i],
    subIntents: {}
  },
  unknown: {
    keywords: [],
    patterns: [],
    subIntents: {}
  }
}

/** 意图分类器 */
export class IntentClassifier {
  /**
   * 分类用户输入的意图
   * @param input 用户自然语言输入
   * @returns 意图识别结果
   */
  classify(input: string): IntentResult {
    if (!input || input.trim().length === 0) {
      return {
        intent: 'unknown',
        confidence: 0,
        keywords: [],
        requiresConfirmation: true
      }
    }

    const normalizedInput = input.trim().toLowerCase()
    const scores: Map<IntentType, number> = new Map()

    // 初始化所有意图分数
    const allIntents: IntentType[] = ['modeling', 'simulation', 'postprocess', 'code', 'analysis', 'notes', 'parametric', 'optimization', 'validation', 'qa', 'unknown']
    for (const intent of allIntents) {
      scores.set(intent, 0)
    }

    // 1. 关键词匹配（每个关键词 +0.15）
    for (const [intent, config] of Object.entries(INTENT_KEYWORDS)) {
      if (intent === 'unknown') continue
      let score = 0
      for (const keyword of config.keywords) {
        if (normalizedInput.includes(keyword.toLowerCase())) {
          score += 0.15
        }
      }
      scores.set(intent as IntentType, score)
    }

    // 2. 正则模式匹配（每个匹配 +0.3）
    for (const [intent, config] of Object.entries(INTENT_KEYWORDS)) {
      if (intent === 'unknown') continue
      let score = scores.get(intent as IntentType) || 0
      for (const pattern of config.patterns) {
        if (pattern.test(input)) {
          score += 0.3
        }
      }
      scores.set(intent as IntentType, score)
    }

    // 3. 子意图匹配（加分）
    let bestSubIntent: string | undefined
    let bestSubIntentScore = 0
    for (const [intent, config] of Object.entries(INTENT_KEYWORDS)) {
      if (intent === 'unknown') continue
      for (const [subIntent, subKeywords] of Object.entries(config.subIntents)) {
        for (const kw of subKeywords) {
          if (normalizedInput.includes(kw.toLowerCase())) {
            const currentScore = scores.get(intent as IntentType) || 0
            scores.set(intent as IntentType, currentScore + 0.1)
            if (scores.get(intent as IntentType)! > bestSubIntentScore) {
              bestSubIntentScore = scores.get(intent as IntentType)!
              bestSubIntent = subIntent
            }
          }
        }
      }
    }

    // 4. 找出最高分意图
    let bestIntent: IntentType = 'unknown'
    let bestScore = 0
    for (const [intent, score] of scores.entries()) {
      if (score > bestScore) {
        bestScore = score
        bestIntent = intent
      }
    }

    // 5. 如果最高分太低，归类为 QA 或 unknown
    if (bestScore < 0.15) {
      // 检查是否是问句
      if (/^(什么|怎么|如何|为什么|解释|请问|？|\?)/.test(input)) {
        bestIntent = 'qa'
        bestScore = 0.5
      } else {
        bestIntent = 'unknown'
        bestScore = 0.1
      }
    }

    // 6. 计算置信度
    const totalScore = Array.from(scores.values()).reduce((a, b) => a + b, 0)
    const confidence = totalScore > 0 ? Math.min(bestScore / totalScore * 2, 1) : 0

    // 7. 提取匹配的关键词
    const matchedKeywords: string[] = []
    if (bestIntent !== 'unknown') {
      const config = INTENT_KEYWORDS[bestIntent]
      for (const keyword of config.keywords) {
        if (normalizedInput.includes(keyword.toLowerCase())) {
          matchedKeywords.push(keyword)
        }
      }
    }

    // 8. 判断是否需要确认
    const requiresConfirmation = confidence < 0.5 || bestIntent === 'unknown'

    return {
      intent: bestIntent,
      confidence: Math.round(confidence * 100) / 100,
      keywords: matchedKeywords,
      subIntent: bestSubIntent,
      requiresConfirmation
    }
  }

  /**
   * 批量分类（用于测试）
   */
  classifyBatch(inputs: string[]): IntentResult[] {
    return inputs.map(input => this.classify(input))
  }
}

/** 全局意图分类器实例 */
export const intentClassifier = new IntentClassifier()
