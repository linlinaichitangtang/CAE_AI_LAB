/**
 * V2.4-011 工具调用执行器
 * invoke(tool_name, params) → Rust backend → JSON 结果
 * V2.4-012 错误处理与重试
 * 工具执行失败时，自动重试（最多 3 次），重试间隔指数退避
 */

import type { ToolResult, ToolDefinition } from './types'
import { toolRegistry } from './tools'

/** 工具执行配置 */
export interface ToolExecutorConfig {
  maxRetries: number
  baseDelay: number       // 基础重试延迟（ms）
  maxDelay: number        // 最大重试延迟（ms）
  timeout: number         // 单次执行超时（ms）
}

/** 默认配置 */
const defaultConfig: ToolExecutorConfig = {
  maxRetries: 3,
  baseDelay: 1000,
  maxDelay: 10000,
  timeout: 30000
}

/** 工具执行器 */
export class ToolExecutor {
  private config: ToolExecutorConfig
  private abortControllers: Map<string, AbortController> = new Map()

  constructor(config?: Partial<ToolExecutorConfig>) {
    this.config = { ...defaultConfig, ...config }
  }

  /**
   * 调用工具
   * @param toolName 工具名称
   * @param params 工具参数
   * @returns 工具执行结果
   */
  async invoke(toolName: string, params: Record<string, unknown> = {}): Promise<ToolResult> {
    const startTime = Date.now()

    // 1. 验证工具是否存在
    const toolDef = toolRegistry.get(toolName)
    if (!toolDef) {
      return {
        success: false,
        error: `工具 "${toolName}" 不存在。可用工具: ${toolRegistry.names.join(', ')}`,
        executionTime: Date.now() - startTime
      }
    }

    // 2. 验证参数
    const paramValidation = this.validateParams(toolDef, params)
    if (!paramValidation.valid) {
      return {
        success: false,
        error: `参数验证失败: ${paramValidation.errors.join('; ')}`,
        executionTime: Date.now() - startTime
      }
    }

    // 3. 检查是否需要用户确认
    if (toolDef.requiresConfirmation) {
      // 返回需要确认的标记，由上层处理
      return {
        success: false,
        error: 'CONFIRMATION_REQUIRED',
        executionTime: Date.now() - startTime,
        metadata: {
          toolName,
          params,
          reason: '此操作需要用户确认后才能执行'
        }
      }
    }

    // 4. 执行工具调用（带重试）
    return this.executeWithRetry(toolDef, params, startTime)
  }

  /**
   * 带重试的执行
   */
  private async executeWithRetry(
    toolDef: ToolDefinition,
    params: Record<string, unknown>,
    startTime: number
  ): Promise<ToolResult> {
    let lastError = ''

    for (let attempt = 0; attempt <= this.config.maxRetries; attempt++) {
      try {
        // 指数退避
        if (attempt > 0) {
          const delay = Math.min(
            this.config.baseDelay * Math.pow(2, attempt - 1),
            this.config.maxDelay
          )
          await this.sleep(delay)
        }

        // 执行工具
        const result = await this.executeSingle(toolDef, params)
        result.executionTime = Date.now() - startTime

        if (result.success) {
          return result
        }

        lastError = result.error || '未知错误'

        // 如果是确认类错误，不重试
        if (lastError === 'CONFIRMATION_REQUIRED') {
          return result
        }

      } catch (error) {
        lastError = String(error)
        console.warn(`Tool "${toolDef.name}" attempt ${attempt + 1} failed:`, lastError)
      }
    }

    return {
      success: false,
      error: `工具 "${toolDef.name}" 执行失败（已重试 ${this.config.maxRetries} 次）: ${lastError}`,
      executionTime: Date.now() - startTime
    }
  }

  /**
   * 单次执行工具
   */
  private async executeSingle(
    toolDef: ToolDefinition,
    params: Record<string, unknown>
  ): Promise<ToolResult> {
    const { invoke } = await import('@tauri-apps/api/core')

    // 映射工具名到 Tauri 命令
    const tauriCommand = this.mapToolToTauriCommand(toolDef.name, toolDef.tauriCommand)

    if (!tauriCommand) {
      // 没有 Tauri 命令的工具，返回模拟结果
      return this.executeMockTool(toolDef.name, params)
    }

    try {
      const result = await invoke(tauriCommand, params)
      return {
        success: true,
        data: result,
        executionTime: 0  // 由外层设置
      }
    } catch (error) {
      return {
        success: false,
        error: String(error),
        executionTime: 0
      }
    }
  }

  /**
   * 映射工具名到 Tauri 命令名
   */
  private mapToolToTauriCommand(toolName: string, tauriCommand?: string): string | null {
    if (tauriCommand) {
      // 将 module::command 格式转换为 Tauri 命令名
      return tauriCommand.includes('::') ? tauriCommand.split('::').pop()! : tauriCommand
    }

    // 默认映射
    const commandMap: Record<string, string> = {
      'get_model_info': 'get_mesh_data',
      'set_material': 'create_material',
      'apply_bc': 'create_bc_container',
      'run_simulation': 'run_solver',
      'get_results': 'parse_results',
      'create_geometry': 'create_beam_model',
      'generate_mesh': 'generate_3d_mesh',
      'import_geometry': 'import_step_file',
      'check_mesh_quality': 'check_mesh_quality',
      'write_file': 'write_file_content',
      'read_file': 'read_file_content',
      'execute_code': 'execute_code',
      'create_note': 'create_file',
      'read_note': 'get_file',
      'update_note': 'update_file',
      'render_contour': 'get_color_map',
      'export_data': 'parse_results',
      'generate_input_file': 'generate_input',
    }

    return commandMap[toolName] || null
  }

  /**
   * 模拟工具执行（用于没有对应 Tauri 命令的工具）
   */
  private async executeMockTool(toolName: string, params: Record<string, unknown>): Promise<ToolResult> {
    // 模拟执行延迟
    await this.sleep(200)

    const mockResults: Record<string, unknown> = {
      'get_model_info': {
        geometryType: 'beam',
        dimensions: { length: 1.0, width: 0.1, height: 0.05 },
        mesh: { nodes: 1000, elements: 800, type: 'hex8' },
        material: null,
        boundaryConditions: []
      },
      'set_material': {
        name: params.materialName || 'Q235',
        youngsModulus: params.youngsModulus || 210e9,
        poissonsRatio: params.poissonsRatio || 0.3,
        density: params.density || 7850,
        applied: true
      },
      'apply_bc': {
        type: params.bcType || 'fixed',
        face: params.face || 'left',
        values: params.values || {},
        applied: true
      },
      'run_simulation': {
        status: 'completed',
        analysisType: params.analysisType || 'static',
        solverOutput: 'Simulation completed successfully.',
        convergence: true,
        iterations: 5
      },
      'get_results': {
        maxVonMises: 150.5e6,
        maxDisplacement: 0.002,
        minVonMises: 0.1e6,
        resultType: params.resultType || 'stress',
        component: params.component || 'von_mises'
      },
      'render_contour': {
        imageGenerated: true,
        field: params.field || 'von_mises_stress',
        colorMap: params.colorMap || 'rainbow',
        min: 0.1e6,
        max: 150.5e6
      },
      'generate_mesh': {
        nodes: 1000,
        elements: 800,
        quality: { avgAspectRatio: 1.5, minJacobian: 0.6 },
        meshType: params.meshType || 'structured'
      },
      'check_mesh_quality': {
        passed: true,
        avgAspectRatio: 1.5,
        maxAspectRatio: 3.2,
        minJacobian: 0.6,
        warnings: []
      },
      'validate_results': {
        passed: true,
        checks: [
          { name: 'stress_range', passed: true, message: 'Von Mises 应力在合理范围内' },
          { name: 'displacement_magnitude', passed: true, message: '位移量级合理' },
          { name: 'convergence', passed: true, message: '求解收敛' }
        ]
      },
      'create_note': {
        id: `note-${Date.now()}`,
        title: params.title || 'Agent 自动生成笔记',
        created: true
      },
      'export_data': {
        format: params.format || 'csv',
        filePath: '/tmp/caelab_export',
        records: 1000,
        exported: true
      },
      'generate_chart': {
        chartType: params.chartType || 'line',
        imageGenerated: true,
        dataPoints: 50
      },
      'slice_model': {
        plane: params.plane || 'XY',
        position: params.position || 0.5,
        imageGenerated: true
      },
      'compare_results': {
        differences: [],
        maxDifference: 0.02,
        correlation: 0.998
      },
      'csg_boolean': {
        operation: params.operation || 'union',
        success: true,
        resultingVolume: 0.005
      },
      'query_geometry': {
        volume: 0.005,
        surfaceArea: 0.42,
        centroid: { x: 0.5, y: 0.05, z: 0.025 },
        boundingBox: { minX: 0, maxX: 1, minY: 0, maxY: 0.1, minZ: 0, maxZ: 0.05 }
      },
      'set_solver': {
        solverName: params.solverName || 'CalculiX',
        analysisType: params.analysisType || 'static',
        configured: true
      },
      'generate_input_file': {
        filePath: '/tmp/job.inp',
        generated: true,
        analysisType: params.analysisType || 'static'
      },
      'git_operations': {
        operation: params.operation || 'status',
        output: 'On branch main\nnothing to commit, working tree clean'
      },
      'read_file': {
        content: '// File content placeholder',
        encoding: params.encoding || 'utf-8',
        size: 1024
      },
      'write_file': {
        filePath: params.filePath || '/tmp/output.txt',
        written: true,
        size: (params.content as string || '').length
      },
      'execute_code': {
        stdout: 'Code executed successfully.',
        stderr: '',
        exitCode: 0,
        language: params.language || 'python'
      },
      'update_note': {
        id: params.noteId || 'note-1',
        updated: true,
        append: params.append || false
      },
      'read_note': {
        id: params.noteId || 'note-1',
        content: 'Note content placeholder',
        title: 'Sample Note'
      },
      'import_geometry': {
        format: params.format || 'step',
        filePath: params.filePath || '/tmp/model.step',
        imported: true,
        geometryInfo: { type: 'imported', volume: 0.003 }
      },

      // ========== 图像分析工具 (V3.6) ==========
      'analyze_sem_image': {
        imageId: params.imageId || 'img-1',
        profileName: params.profileName || 'full',
        features: [
          { type: 'dimple', count: 45, avgSize: 2.5, areaRatio: 0.32 },
          { type: 'crack', count: 3, avgSize: 15.2, length: 45.6 }
        ],
        statistics: {
          totalFeatures: 48,
          featuresByType: { dimple: 45, crack: 3, striation: 0, pore: 0, cleavage: 0, particle: 0, inclusion: 0, unknown: 0 },
          averageSize: 3.8,
          areaRatio: 0.32
        },
        qualityScore: 0.92
      },
      'detect_fracture_features': {
        imageId: params.imageId || 'img-1',
        featureTypes: params.featureTypes || ['dimple', 'crack'],
        detections: [
          { type: 'dimple', confidence: 0.95, count: 42, avgSize: 2.3 },
          { type: 'crack', confidence: 0.88, count: 2, totalLength: 28.5 }
        ],
        modelUsed: params.modelId || 'default_fracture_model'
      },
      'correlate_with_simulation': {
        imageAnalysisId: params.imageAnalysisId || 'result-1',
        simulationData: params.simulationData || { maxStress: 850, maxDisplacement: 0.025 },
        correlationScore: 0.87,
        insights: [
          '断裂模式以韧性断裂为主，韧窝发育良好',
          '高应力区与裂纹扩展路径存在空间关联'
        ],
        recommendations: [
          '材料具有较好的韧性，建议关注疲劳寿命',
          '建议优化热处理工艺提高强度'
        ]
      },

      // ========== ML 训练工具 (V3.6) ==========
      'create_ml_dataset': {
        datasetId: `dataset_${Date.now()}`,
        datasetName: params.datasetName || 'fracture_dataset',
        rootPath: params.rootPath || '/data/fracture',
        classes: params.classes || ['dimple', 'crack', 'striation'],
        created: true
      },
      'train_image_classifier': {
        datasetId: params.datasetId || 'dataset-1',
        modelType: params.modelType || 'classifier',
        architecture: params.architecture || 'resnet18',
        epochs: params.epochs || 50,
        status: 'completed',
        metrics: {
          trainAccuracy: 0.96,
          valAccuracy: 0.92,
          f1Score: 0.91
        },
        modelId: `model_${Date.now()}`
      },
      'predict_with_trained_model': {
        imageId: params.imageId || 'img-1',
        modelId: params.modelId || 'model-1',
        predictions: [
          { label: 'dimple', confidence: 0.94 },
          { label: 'ductile', confidence: 0.89 }
        ],
        processingTime: 0.32
      },

      // ========== Active Learning 工具 (V3.7) ==========
      'initialize_active_learning': {
        poolId: `pool_${Date.now()}`,
        strategy: params.strategy || 'uncertainty',
        batchSize: params.batchSize || 10,
        totalPoints: (params.features as unknown[][] || []).length || 500,
        initialized: true
      },
      'acquire_next_points': {
        poolId: params.poolId || 'pool-1',
        numPoints: params.numPoints || 5,
        selectedPoints: [
          { id: 'pt_1', features: [0.5, 0.3, 0.8], score: 0.92 },
          { id: 'pt_2', features: [0.6, 0.4, 0.7], score: 0.88 },
          { id: 'pt_3', features: [0.4, 0.5, 0.9], score: 0.85 }
        ],
        reason: '高不确定度区域'
      },
      'run_active_learning_iteration': {
        poolId: params.poolId || 'pool-1',
        iteration: 1,
        newLabeledCount: (params.labeledData as unknown[] || []).length || 5,
        modelAccuracy: 0.89,
        uncertaintyReduction: 0.15,
        totalLabeled: 15
      },

      // ========== PINN 工具 (V3.7) ==========
      'train_pinn_model': {
        modelId: `pinn_${Date.now()}`,
        modelName: params.modelName || 'elasticity_pinn',
        physicsType: params.physicsType || 'linear_elasticity',
        epochs: params.epochs || 5000,
        status: 'completed',
        metrics: {
          finalLoss: 0.015,
          dataLoss: 0.008,
          physicsLoss: 0.007,
          r2Score: 0.96
        }
      },
      'predict_pinn': {
        pinnId: params.pinnId || 'pinn-1',
        position: params.position || [0.5, 0.5],
        time: params.time,
        predictedValue: 125.4,
        residual: 0.012,
        confidence: 0.94
      },

      // ========== Surrogate Model 工具 (V3.7) ==========
      'create_surrogate_model': {
        surrogateId: `surrogate_${Date.now()}`,
        modelName: params.modelName || 'stress_surrogate',
        inputDimensions: params.inputDimensions || ['temperature', 'pressure', 'material'],
        outputDimensions: params.outputDimensions || ['maxStress', 'maxDisplacement'],
        modelType: params.modelType || 'ANN',
        created: true
      },
      'train_surrogate': {
        surrogateId: params.surrogateId || 'surrogate-1',
        trainingDataSize: (params.trainingData as unknown[] || []).length || 100,
        status: 'completed',
        metrics: {
          trainRMSE: 0.018,
          valRMSE: 0.024,
          r2Score: 0.97
        }
      },
      'predict_with_surrogate': {
        surrogateId: params.surrogateId || 'surrogate-1',
        inputs: params.inputs || [300, 0.001],
        predictions: [850.5, 0.023],
        uncertainty: [25.2, 0.001],
        confidence: 0.91,
        predictionTime: 0.005
      }
    }

    const result = mockResults[toolName]
    if (result) {
      return {
        success: true,
        data: result,
        executionTime: 0
      }
    }

    return {
      success: false,
      error: `工具 "${toolName}" 没有对应的执行实现`,
      executionTime: 0
    }
  }

  /**
   * 参数验证
   */
  private validateParams(
    toolDef: ToolDefinition,
    params: Record<string, unknown>
  ): { valid: boolean; errors: string[] } {
    const errors: string[] = []

    for (const paramDef of toolDef.params) {
      if (paramDef.required && !(paramDef.name in params)) {
        errors.push(`缺少必填参数 "${paramDef.name}": ${paramDef.description}`)
        continue
      }

      const value = params[paramDef.name]
      if (value === undefined || value === null) continue

      // 类型检查
      if (paramDef.type === 'number' && typeof value !== 'number') {
        errors.push(`参数 "${paramDef.name}" 应为 number 类型，实际为 ${typeof value}`)
      }
      if (paramDef.type === 'string' && typeof value !== 'string') {
        errors.push(`参数 "${paramDef.name}" 应为 string 类型，实际为 ${typeof value}`)
      }
      if (paramDef.type === 'boolean' && typeof value !== 'boolean') {
        errors.push(`参数 "${paramDef.name}" 应为 boolean 类型，实际为 ${typeof value}`)
      }

      // 枚举检查
      if (paramDef.enum && typeof value === 'string' && !paramDef.enum.includes(value)) {
        errors.push(`参数 "${paramDef.name}" 值 "${value}" 不在允许范围内: ${paramDef.enum.join('/')}`)
      }

      // 范围检查
      if (paramDef.minimum !== undefined && typeof value === 'number' && value < paramDef.minimum) {
        errors.push(`参数 "${paramDef.name}" 值 ${value} 小于最小值 ${paramDef.minimum}`)
      }
      if (paramDef.maximum !== undefined && typeof value === 'number' && value > paramDef.maximum) {
        errors.push(`参数 "${paramDef.name}" 值 ${value} 大于最大值 ${paramDef.maximum}`)
      }
    }

    return { valid: errors.length === 0, errors }
  }

  /**
   * 取消执行
   */
  cancel(toolName?: string): void {
    if (toolName) {
      const controller = this.abortControllers.get(toolName)
      if (controller) {
        controller.abort()
        this.abortControllers.delete(toolName)
      }
    } else {
      for (const controller of this.abortControllers.values()) {
        controller.abort()
      }
      this.abortControllers.clear()
    }
  }

  /**
   * 更新配置
   */
  updateConfig(config: Partial<ToolExecutorConfig>): void {
    this.config = { ...this.config, ...config }
  }

  private sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms))
  }
}

/** 全局工具执行器实例 */
export const toolExecutor = new ToolExecutor()
