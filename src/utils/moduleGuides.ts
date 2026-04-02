/**
 * V1.1-010: 模块级引导步骤定义
 * 为每个功能模块定义渐进式引导步骤
 */

export interface GuideStep {
  /** 目标元素的 CSS 选择器 */
  target: string
  /** 引导标题 */
  title: string
  /** 引导描述 */
  description: string
  /** 气泡位置 */
  position: 'top' | 'bottom' | 'left' | 'right'
  /** 建议用户执行的操作 */
  action?: string
}

// ============ 仿真模块引导 ============

export const simulationGuideSteps: GuideStep[] = [
  {
    target: '.analysis-type-selector',
    title: '选择分析类型',
    description: '根据你的工程问题选择合适的分析类型：结构分析、模态分析、热传导等。不同类型对应不同的求解策略。',
    position: 'bottom',
  },
  {
    target: '.mesh-config-panel',
    title: '配置网格',
    description: '设置网格维度、范围和密度。网格密度直接影响计算精度和速度，建议先粗后细逐步加密。',
    position: 'right',
  },
  {
    target: '.material-config-panel',
    title: '设置材料参数',
    description: '定义材料的弹性模量、泊松比等参数。可从内置材料库快速选择，也支持自定义输入。',
    position: 'right',
  },
  {
    target: '.boundary-condition-panel',
    title: '设置边界条件',
    description: '定义固定约束和载荷，描述实际工况。正确的边界条件是获得可靠结果的关键。',
    position: 'right',
  },
  {
    target: '.solver-run-button',
    title: '运行求解',
    description: '设置完成后点击运行求解器。可实时查看求解进度，完成后自动显示结果云图。',
    position: 'bottom',
  },
]

// ============ 建模模块引导 ============

export const modelingGuideSteps: GuideStep[] = [
  {
    target: '.geometry-add-button',
    title: '添加几何体',
    description: '点击添加基本几何体（长方体、圆柱体、球体等），或导入外部 CAD 模型文件。',
    position: 'bottom',
  },
  {
    target: '.scene-tree-panel',
    title: '场景树',
    description: '场景树显示当前所有几何体。点击选中，支持多选和批量操作。',
    position: 'right',
  },
  {
    target: '.modeling-3d-viewport',
    title: '3D 视图',
    description: '在 3D 视图中查看和操作几何体。支持旋转、平移、缩放等视图操作。',
    position: 'left',
  },
  {
    target: '.modeling-transform-panel',
    title: '变换工具',
    description: '使用移动、旋转、缩放等工具精确调整几何体的位置和形态。',
    position: 'right',
  },
  {
    target: '.modeling-export-button',
    title: '导出与联动',
    description: '将建模结果传递给仿真模块进行网格生成和求解分析，实现建模-仿真一体化工作流。',
    position: 'bottom',
  },
]

// ============ 笔记模块引导 ============

export const notesGuideSteps: GuideStep[] = [
  {
    target: '.notes-project-selector',
    title: '选择项目',
    description: '选择或创建项目来组织你的笔记。每个项目可以包含多份笔记和仿真数据。',
    position: 'bottom',
  },
  {
    target: '.notes-editor-area',
    title: '富文本编辑器',
    description: '支持 Markdown 语法、数学公式、代码块等。可嵌入仿真结果、3D 模型等富媒体内容。',
    position: 'left',
  },
  {
    target: '.notes-ai-optimize-button',
    title: 'AI 辅助写作',
    description: '使用 AI 优化笔记内容、生成摘要或扩展思路。支持多种写作辅助模式。',
    position: 'bottom',
  },
  {
    target: '.notes-embed-button',
    title: '嵌入仿真结果',
    description: '将仿真结果、3D 模型等直接嵌入笔记中，形成完整的工程分析报告。',
    position: 'bottom',
  },
]

// ============ 代码模块引导 ============

export const codeGuideSteps: GuideStep[] = [
  {
    target: '.code-language-selector',
    title: '选择语言',
    description: '选择编程语言以获得语法高亮和智能提示支持。支持 Python、JavaScript、MATLAB 等。',
    position: 'bottom',
  },
  {
    target: '.code-editor-area',
    title: '代码编辑器',
    description: '基于 Monaco Editor 的代码编辑器，提供语法高亮、自动补全、代码格式化等功能。',
    position: 'left',
  },
  {
    target: '.code-run-button',
    title: '运行代码',
    description: '编写脚本后可直接运行，支持与仿真模块的数据交互，实现参数化仿真和自动化分析。',
    position: 'bottom',
  },
]

/**
 * 根据模块 key 获取引导步骤
 */
export function getGuideSteps(moduleKey: string): GuideStep[] {
  const map: Record<string, GuideStep[]> = {
    simulation: simulationGuideSteps,
    modeling: modelingGuideSteps,
    notes: notesGuideSteps,
    code: codeGuideSteps,
  }
  return map[moduleKey] || []
}
