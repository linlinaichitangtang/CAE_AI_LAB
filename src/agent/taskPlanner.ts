/**
 * V2.4-002 任务规划器
 * 支持 ReAct 循环，输出结构化 TaskPlan（子任务列表）
 * 复用现有 AI 流式输出
 */

import type { IntentResult, TaskPlan, SubTask, ReActStep } from './types'

/** 生成短UUID */
function shortId(): string {
  return Math.random().toString(36).substring(2, 8)
}

/** 任务规划器 */
export class TaskPlanner {
  /**
   * 根据意图和用户查询生成任务计划
   */
  plan(userQuery: string, intent: IntentResult): TaskPlan {
    const planId = `plan-${shortId()}`
    const subTasks = this.decomposeTask(userQuery, intent)

    return {
      id: planId,
      userQuery,
      intent,
      subTasks,
      status: 'planning',
      currentStepIndex: 0,
      createdAt: Date.now(),
      updatedAt: Date.now()
    }
  }

  /**
   * 将用户任务分解为子任务序列
   */
  private decomposeTask(query: string, intent: IntentResult): SubTask[] {
    const subTasks: SubTask[] = []
    const now = Date.now()

    switch (intent.intent) {
      case 'simulation': {
        const subIntent = intent.subIntent || 'simulation.static'

        // 通用仿真流程
        subTasks.push(this.createSubTask('获取模型信息', '读取当前几何和网格信息', 'get_model_info', {}, [], now))
        subTasks.push(this.createSubTask('设置材料', '为模型分配材料属性', 'set_material', {}, [subTasks[0].id], now))
        subTasks.push(this.createSubTask('施加边界条件', '设置约束和载荷', 'apply_bc', {}, [subTasks[1].id], now))
        subTasks.push(this.createSubTask('提交仿真求解', `运行${this.getAnalysisTypeName(subIntent)}分析`, 'run_simulation', { analysisType: this.mapSubIntentToAnalysisType(subIntent) }, [subTasks[2].id], now))
        subTasks.push(this.createSubTask('获取仿真结果', '读取求解结果数据', 'get_results', { resultType: 'stress', component: 'von_mises' }, [subTasks[3].id], now))
        subTasks.push(this.createSubTask('渲染结果云图', '可视化应力分布', 'render_contour', { field: 'von_mises_stress' }, [subTasks[4].id], now))

        // 特殊分析类型追加步骤
        if (subIntent.includes('fatigue')) {
          subTasks.push(this.createSubTask('疲劳寿命评估', '基于S-N曲线计算疲劳寿命', 'validate_results', { resultType: 'fatigue', criteria: 'engineering' }, [subTasks[5].id], now))
        }
        if (subIntent.includes('modal')) {
          subTasks.push(this.createSubTask('获取模态结果', '读取固有频率和振型', 'get_results', { resultType: 'displacement', component: 'all' }, [subTasks[4].id], now))
        }
        break
      }

      case 'modeling': {
        const subIntent = intent.subIntent || ''

        if (subIntent.includes('mesh')) {
          subTasks.push(this.createSubTask('获取几何信息', '读取当前几何体信息', 'get_model_info', {}, [], now))
          subTasks.push(this.createSubTask('生成网格', '划分有限元网格', 'generate_mesh', { meshType: 'structured' }, [subTasks[0].id], now))
          subTasks.push(this.createSubTask('检查网格质量', '验证网格质量指标', 'check_mesh_quality', {}, [subTasks[1].id], now))
        } else if (subIntent.includes('import')) {
          subTasks.push(this.createSubTask('导入几何文件', '导入外部CAD文件', 'import_geometry', {}, [], now))
          subTasks.push(this.createSubTask('查询几何信息', '获取导入模型的详细信息', 'query_geometry', { property: 'all' }, [subTasks[0].id], now))
        } else {
          subTasks.push(this.createSubTask('创建几何体', '根据参数创建几何模型', 'create_geometry', {}, [], now))
          subTasks.push(this.createSubTask('查询几何信息', '获取几何体属性', 'query_geometry', { property: 'all' }, [subTasks[0].id], now))
        }
        break
      }

      case 'postprocess': {
        subTasks.push(this.createSubTask('获取仿真结果', '读取已有的仿真结果', 'get_results', { resultType: 'all' }, [], now))
        subTasks.push(this.createSubTask('渲染云图', '生成结果可视化', 'render_contour', { field: 'von_mises_stress' }, [subTasks[0].id], now))

        if (query.includes('导出') || query.includes('CSV') || query.includes('JSON')) {
          subTasks.push(this.createSubTask('导出数据', '导出结果数据文件', 'export_data', { format: 'csv', data: 'all' }, [subTasks[0].id], now))
        }
        if (query.includes('切片') || query.includes('截面')) {
          subTasks.push(this.createSubTask('切片查看', '切面查看内部结果', 'slice_model', {}, [subTasks[0].id], now))
        }
        if (query.includes('图表') || query.includes('曲线')) {
          subTasks.push(this.createSubTask('生成图表', '绘制数据图表', 'generate_chart', { chartType: 'line' }, [subTasks[0].id], now))
        }
        break
      }

      case 'code': {
        const subIntent = intent.subIntent || ''

        if (subIntent.includes('generate')) {
          subTasks.push(this.createSubTask('分析需求', '理解代码生成需求', 'get_model_info', {}, [], now))
          subTasks.push(this.createSubTask('生成代码', '生成脚本代码', 'write_file', {}, [subTasks[0].id], now))
          subTasks.push(this.createSubTask('执行验证', '运行代码验证正确性', 'execute_code', { language: 'python' }, [subTasks[1].id], now))
        } else if (subIntent.includes('execute')) {
          subTasks.push(this.createSubTask('执行代码', '运行用户指定的代码', 'execute_code', { language: 'python' }, [], now))
        } else if (subIntent.includes('git')) {
          subTasks.push(this.createSubTask('Git操作', '执行Git命令', 'git_operations', { operation: 'status' }, [], now))
        } else {
          subTasks.push(this.createSubTask('文件操作', '读写文件', 'read_file', {}, [], now))
        }
        break
      }

      case 'analysis': {
        // TC4 / 多模态失效分析 — EBSD + SEM + Load 可并行执行
        if (query.includes('TC4') || query.includes('失效') || query.includes('多模态') || query.includes('三路')) {
          const groupId = `parallel-${shortId()}`
          subTasks.push(this.createSubTask('加载 EBSD 晶格数据', '提取 EBSD 晶格特征 80维', 'tc4_load_ebsd', { material: 'TC4' }, [], now, groupId))
          subTasks.push(this.createSubTask('加载 SEM 图像特征', '提取 SEM 图像特征 768维', 'tc4_load_sem', {}, [], now, groupId))
          subTasks.push(this.createSubTask('加载载荷历史', '提取载荷特征 30维', 'tc4_load_load', {}, [], now, groupId))
          subTasks.push(this.createSubTask('多模态失效预测', 'EBSD+SEM+Load 三路融合预测', 'tc4_predict_failure', {}, [subTasks[0].id, subTasks[1].id, subTasks[2].id], now))
          break
        }
        subTasks.push(this.createSubTask('获取当前结果', '读取仿真结果用于分析', 'get_results', { resultType: 'all' }, [], now))
        subTasks.push(this.createSubTask('获取模型信息', '读取模型和边界条件设置', 'get_model_info', {}, [], now))
        subTasks.push(this.createSubTask('验证结果', '检查结果数值合理性', 'validate_results', { resultType: 'stress', criteria: 'engineering' }, [subTasks[0].id], now))
        subTasks.push(this.createSubTask('生成诊断报告', '输出分析结论', 'create_note', { title: '分析诊断报告' }, [subTasks[2].id], now))
        break
      }

      case 'notes': {
        subTasks.push(this.createSubTask('创建笔记', '创建新笔记记录', 'create_note', {}, [], now))
        break
      }

      case 'parametric': {
        subTasks.push(this.createSubTask('获取模型信息', '读取当前模型参数', 'get_model_info', {}, [], now))
        subTasks.push(this.createSubTask('设置基准参数', '配置参数化扫描基准', 'set_material', {}, [subTasks[0].id], now))
        subTasks.push(this.createSubTask('运行参数化扫描', '执行多组参数仿真', 'run_simulation', { analysisType: 'static' }, [subTasks[1].id], now))
        subTasks.push(this.createSubTask('汇总结果', '收集并对比各组结果', 'get_results', { resultType: 'all' }, [subTasks[2].id], now))
        subTasks.push(this.createSubTask('生成对比图表', '绘制参数-结果曲线', 'generate_chart', { chartType: 'line' }, [subTasks[3].id], now))
        break
      }

      case 'optimization': {
        subTasks.push(this.createSubTask('获取模型信息', '读取当前模型', 'get_model_info', {}, [], now))
        subTasks.push(this.createSubTask('配置优化参数', '设置优化目标和约束', 'set_solver', { analysisType: 'static' }, [subTasks[0].id], now))
        subTasks.push(this.createSubTask('运行优化', '执行拓扑/形状/尺寸优化', 'run_simulation', { analysisType: 'static' }, [subTasks[1].id], now))
        subTasks.push(this.createSubTask('获取优化结果', '读取优化结果', 'get_results', { resultType: 'all' }, [subTasks[2].id], now))
        break
      }

      case 'validation': {
        subTasks.push(this.createSubTask('获取模型信息', '读取验证模型', 'get_model_info', {}, [], now))
        subTasks.push(this.createSubTask('运行仿真', '提交验证仿真', 'run_simulation', { analysisType: 'static' }, [subTasks[0].id], now))
        subTasks.push(this.createSubTask('获取结果', '读取仿真结果', 'get_results', { resultType: 'all' }, [subTasks[1].id], now))
        subTasks.push(this.createSubTask('验证结果', '与理论解对比验证', 'validate_results', { resultType: 'stress', criteria: 'strict' }, [subTasks[2].id], now))
        break
      }

      default: {
        // TC4 / 多模态失效分析 — EBSD + SEM + Load 可并行执行
        if (query.includes('TC4') || query.includes('失效') || query.includes('多模态') || query.includes('三路')) {
          const groupId = `parallel-${shortId()}`
          subTasks.push(this.createSubTask('加载 EBSD 晶格数据', '提取 EBSD 晶格特征 80维', 'tc4_load_ebsd', { material: 'TC4' }, [], now, groupId))
          subTasks.push(this.createSubTask('加载 SEM 图像特征', '提取 SEM 图像特征 768维', 'tc4_load_sem', {}, [], now, groupId))
          subTasks.push(this.createSubTask('加载载荷历史', '提取载荷特征 30维', 'tc4_load_load', {}, [], now, groupId))
          subTasks.push(this.createSubTask('多模态失效预测', 'EBSD+SEM+Load 三路融合预测', 'tc4_predict_failure', {}, [subTasks[0].id, subTasks[1].id, subTasks[2].id], now))
          break
        }
        // QA 和 unknown 意图不需要工具调用，直接返回空计划
        break
      }
    }

    return subTasks
  }

  /**
   * 创建子任务
   */
  private createSubTask(
    name: string,
    description: string,
    toolName: string,
    toolParams: Record<string, unknown>,
    dependsOn: string[],
    createdAt: number,
    parallelGroup?: string
  ): SubTask {
    return {
      id: `task-${shortId()}`,
      name,
      description,
      toolName,
      toolParams,
      status: 'pending',
      retryCount: 0,
      maxRetries: 3,
      dependsOn,
      createdAt,
      estimatedTime: this.estimateTime(toolName),
      parallelGroup,
    }
  }

  /**
   * 估算工具执行时间
   */
  private estimateTime(toolName: string): number {
    const estimates: Record<string, number> = {
      'get_model_info': 1,
      'set_material': 1,
      'apply_bc': 2,
      'run_simulation': 30,  // 仿真求解通常较慢
      'get_results': 2,
      'render_contour': 3,
      'generate_mesh': 5,
      'check_mesh_quality': 2,
      'validate_results': 1,
      'create_note': 1,
      'export_data': 2,
      'generate_chart': 2,
      'slice_model': 2,
      'execute_code': 10,
      'write_file': 1,
      'read_file': 1,
    }
    return estimates[toolName] || 3
  }

  /**
   * 获取分析类型中文名
   */
  private getAnalysisTypeName(subIntent: string): string {
    const names: Record<string, string> = {
      'simulation.static': '静力',
      'simulation.modal': '模态',
      'simulation.thermal': '热',
      'simulation.buckling': '屈曲',
      'simulation.fatigue': '疲劳',
      'simulation.transient': '瞬态动力',
      'simulation.contact': '接触',
      'simulation.cfd': 'CFD流体',
    }
    return names[subIntent] || '仿真'
  }

  /**
   * 映射子意图到分析类型
   */
  private mapSubIntentToAnalysisType(subIntent: string): string {
    const mapping: Record<string, string> = {
      'simulation.static': 'static',
      'simulation.modal': 'modal',
      'simulation.thermal': 'thermal',
      'simulation.buckling': 'buckling',
      'simulation.fatigue': 'static',  // 疲劳先跑静力
      'simulation.transient': 'transient',
      'simulation.contact': 'static',  // 接触先跑静力
      'simulation.cfd': 'static',      // CFD暂用static占位
    }
    return mapping[subIntent] || 'static'
  }

  /**
   * ReAct 思考步骤
   */
  createReActStep(thought: string, action: string, actionInput: Record<string, unknown>): ReActStep {
    return {
      thought,
      action,
      actionInput,
      timestamp: Date.now()
    }
  }
}

/** 全局任务规划器实例 */
export const taskPlanner = new TaskPlanner()
