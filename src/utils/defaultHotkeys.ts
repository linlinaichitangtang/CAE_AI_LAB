/**
 * V1.1-011: 默认快捷键配置
 * 定义所有预定义的快捷键绑定
 */

export interface HotkeyConfig {
  /** 唯一标识，如 'global.save' */
  id: string
  /** 快捷键组合，如 'ctrl+s', 'alt+s', 'f5' */
  keys: string
  /** 功能描述 */
  description: string
  /** 分类 */
  category: 'global' | 'simulation' | 'modeling' | 'notes' | 'code'
}

export const defaultHotkeys: HotkeyConfig[] = [
  // ===== 全局 =====
  { id: 'global.save', keys: 'ctrl+s', description: '保存', category: 'global' },
  { id: 'global.undo', keys: 'ctrl+z', description: '撤销', category: 'global' },
  { id: 'global.redo', keys: 'ctrl+shift+z', description: '重做', category: 'global' },
  { id: 'global.settings', keys: 'ctrl+,', description: '设置', category: 'global' },
  { id: 'global.search', keys: 'ctrl+k', description: '搜索', category: 'global' },
  { id: 'global.help', keys: 'f1', description: '帮助', category: 'global' },
  { id: 'global.close-panel', keys: 'escape', description: '关闭面板/弹窗', category: 'global' },

  // ===== 仿真 =====
  { id: 'simulation.run', keys: 'f5', description: '运行仿真', category: 'simulation' },
  { id: 'simulation.stop', keys: 'shift+f5', description: '停止仿真', category: 'simulation' },
  { id: 'simulation.mesh', keys: 'ctrl+m', description: '生成网格', category: 'simulation' },
  { id: 'simulation.reset', keys: 'ctrl+shift+r', description: '重置仿真', category: 'simulation' },
  { id: 'simulation.result', keys: 'ctrl+shift+v', description: '查看结果', category: 'simulation' },

  // ===== 建模 =====
  { id: 'modeling.select', keys: 's', description: '选择工具', category: 'modeling' },
  { id: 'modeling.move', keys: 'g', description: '移动工具', category: 'modeling' },
  { id: 'modeling.rotate', keys: 'r', description: '旋转工具', category: 'modeling' },
  { id: 'modeling.scale', keys: 'e', description: '缩放工具', category: 'modeling' },
  { id: 'modeling.add-geometry', keys: 'shift+a', description: '添加几何体', category: 'modeling' },
  { id: 'modeling.delete', keys: 'delete', description: '删除选中', category: 'modeling' },

  // ===== 笔记 =====
  { id: 'notes.bold', keys: 'ctrl+b', description: '加粗', category: 'notes' },
  { id: 'notes.italic', keys: 'ctrl+i', description: '斜体', category: 'notes' },
  { id: 'notes.underline', keys: 'ctrl+u', description: '下划线', category: 'notes' },
  { id: 'notes.new-note', keys: 'ctrl+n', description: '新建笔记', category: 'notes' },
  { id: 'notes.ai-optimize', keys: 'ctrl+shift+a', description: 'AI 优化', category: 'notes' },

  // ===== 代码 =====
  { id: 'code.run', keys: 'ctrl+enter', description: '运行代码', category: 'code' },
  { id: 'code.format', keys: 'shift+alt+f', description: '格式化代码', category: 'code' },
  { id: 'code.save-file', keys: 'ctrl+s', description: '保存文件', category: 'code' },
  { id: 'code.comment', keys: 'ctrl+/', description: '切换注释', category: 'code' },
]

/**
 * 获取分类标签
 */
export function getCategoryLabel(category: string): string {
  const labels: Record<string, string> = {
    global: '全局',
    simulation: '仿真',
    modeling: '建模',
    notes: '笔记',
    code: '代码',
  }
  return labels[category] || category
}

/**
 * 获取分类图标
 */
export function getCategoryIcon(category: string): string {
  const icons: Record<string, string> = {
    global: '\u{1F310}',
    simulation: '\u{1F4CA}',
    modeling: '\u{1F4D0}',
    notes: '\u{1F4DD}',
    code: '\u{1F4BB}',
  }
  return icons[category] || '\u{1F3E0}'
}
