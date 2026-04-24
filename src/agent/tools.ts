/**
 * V2.4-004 工具注册表
 * 统一管理所有可用工具的 schema（名称/描述/参数/返回类型）
 * V2.4-005 基础仿真工具集
 * V2.4-006 建模工具集
 * V2.4-007 仿真工具集扩展
 * V2.4-008 代码工具集
 * V2.4-009 笔记工具集
 * V2.4-010 后处理工具集
 */

import type { ToolDefinition } from './types'

// ============================================================================
// 工具定义 - 基础仿真工具 (V2.4-005)
// ============================================================================

const get_model_info: ToolDefinition = {
  name: 'get_model_info',
  description: '获取当前模型的详细信息，包括几何类型、尺寸、网格统计等',
  category: 'simulation',
  params: [],
  returnType: 'ModelInfo',
  requiresConfirmation: false,
  tauriCommand: 'cae_api::get_mesh_data',
  isDestructive: false,
  examples: ['获取当前模型信息', '查看网格统计']
}

const set_material: ToolDefinition = {
  name: 'set_material',
  description: '为当前模型设置材料属性（弹性模量、泊松比、密度等）',
  category: 'simulation',
  params: [
    { name: 'materialName', type: 'string', description: '材料名称（如 Q235, 304不锈钢, Al6061）', required: true },
    { name: 'youngsModulus', type: 'number', description: '弹性模量 (Pa)', required: false, default: 210e9 },
    { name: 'poissonsRatio', type: 'number', description: '泊松比', required: false, default: 0.3 },
    { name: 'density', type: 'number', description: '密度 (kg/m³)', required: false, default: 7850 },
    { name: 'yieldStrength', type: 'number', description: '屈服强度 (Pa)', required: false },
    { name: 'thermalConductivity', type: 'number', description: '热导率 (W/(m·K))', required: false }
  ],
  returnType: 'MaterialInfo',
  requiresConfirmation: false,
  tauriCommand: 'materials::create_material',
  isDestructive: false,
  examples: ['设置材料为Q235钢', '使用304不锈钢']
}

const apply_bc: ToolDefinition = {
  name: 'apply_bc',
  description: '施加边界条件（固定约束、位移约束、载荷等）',
  category: 'simulation',
  params: [
    { name: 'bcType', type: 'string', description: '边界条件类型', required: true, enum: ['fixed', 'displacement', 'point_load', 'pressure', 'heat_flux', 'temperature'] },
    { name: 'face', type: 'string', description: '施加的面（如 left, right, top, bottom, front, back）', required: true },
    { name: 'values', type: 'object', description: '边界条件值（如 {fx: 0, fy: -1000, fz: 0}）', required: false }
  ],
  returnType: 'BoundaryCondition',
  requiresConfirmation: false,
  tauriCommand: 'cae_api::create_bc_container',
  isDestructive: false,
  examples: ['固定左端面', '在右端施加1000N向下载荷', '顶部施加10MPa压力']
}

const run_simulation: ToolDefinition = {
  name: 'run_simulation',
  description: '提交仿真求解任务（支持静力、模态、热分析等）',
  category: 'simulation',
  params: [
    { name: 'analysisType', type: 'string', description: '分析类型', required: true, enum: ['static', 'modal', 'thermal', 'buckling', 'frequency_response', 'transient'] },
    { name: 'solverName', type: 'string', description: '求解器名称', required: false, default: 'CalculiX' },
    { name: 'maxIterations', type: 'number', description: '最大迭代次数', required: false, default: 100 }
  ],
  returnType: 'SimulationResult',
  requiresConfirmation: true,
  tauriCommand: 'cae_api::run_solver',
  isDestructive: false,
  examples: ['运行静力分析', '提交模态分析求解']
}

const get_results: ToolDefinition = {
  name: 'get_results',
  description: '获取仿真结果数据（应力、位移、应变等）',
  category: 'simulation',
  params: [
    { name: 'resultType', type: 'string', description: '结果类型', required: true, enum: ['stress', 'displacement', 'strain', 'reaction_force', 'temperature', 'all'] },
    { name: 'component', type: 'string', description: '分量（如 von_mises, sx, sy, sz, sxy, mag）', required: false, default: 'von_mises' }
  ],
  returnType: 'ResultData',
  requiresConfirmation: false,
  tauriCommand: 'cae_api::parse_results',
  isDestructive: false,
  examples: ['获取Von Mises应力结果', '查看位移分布']
}

// ============================================================================
// 工具定义 - 建模工具集 (V2.4-006)
// ============================================================================

const create_geometry: ToolDefinition = {
  name: 'create_geometry',
  description: '创建基础几何体（立方体、球体、圆柱体、圆锥体等）',
  category: 'modeling',
  params: [
    { name: 'type', type: 'string', description: '几何体类型', required: true, enum: ['cube', 'sphere', 'cylinder', 'cone', 'torus'] },
    { name: 'width', type: 'number', description: '宽度/直径 (m)', required: false, default: 1.0 },
    { name: 'height', type: 'number', description: '高度 (m)', required: false, default: 1.0 },
    { name: 'depth', type: 'number', description: '深度 (m)', required: false, default: 1.0 },
    { name: 'divisions', type: 'object', description: '分段数 {divX, divY, divZ}', required: false }
  ],
  returnType: 'GeometryInfo',
  requiresConfirmation: false,
  tauriCommand: 'cae_api::create_beam_model',
  isDestructive: false,
  examples: ['创建一个1m×0.1m×0.05m的立方体', '画一个半径0.5m的球']
}

const csg_boolean: ToolDefinition = {
  name: 'csg_boolean',
  description: 'CSG布尔运算（并集、交集、差集）',
  category: 'modeling',
  params: [
    { name: 'operation', type: 'string', description: '布尔运算类型', required: true, enum: ['union', 'intersect', 'subtract'] },
    { name: 'bodyA', type: 'string', description: '几何体A的ID', required: true },
    { name: 'bodyB', type: 'string', description: '几何体B的ID', required: true }
  ],
  returnType: 'GeometryInfo',
  requiresConfirmation: false,
  isDestructive: true,
  examples: ['从立方体中减去圆柱体', '合并两个几何体']
}

const query_geometry: ToolDefinition = {
  name: 'query_geometry',
  description: '查询几何体信息（体积、表面积、质心等）',
  category: 'modeling',
  params: [
    { name: 'geometryId', type: 'string', description: '几何体ID', required: false },
    { name: 'property', type: 'string', description: '查询属性', required: false, enum: ['volume', 'surface_area', 'centroid', 'bounding_box', 'all'] }
  ],
  returnType: 'GeometryProperties',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['查询当前几何体体积', '获取几何体信息']
}

const generate_mesh: ToolDefinition = {
  name: 'generate_mesh',
  description: '生成有限元网格（支持结构化/非结构化网格）',
  category: 'modeling',
  params: [
    { name: 'meshType', type: 'string', description: '网格类型', required: true, enum: ['structured', 'unstructured', 'tetrahedral', 'hexahedral'] },
    { name: 'elementSize', type: 'number', description: '单元尺寸 (m)', required: false },
    { name: 'divisions', type: 'object', description: '分段数 {divX, divY, divZ}', required: false },
    { name: 'elementOrder', type: 'string', description: '单元阶次', required: false, enum: ['linear', 'quadratic'], default: 'linear' }
  ],
  returnType: 'MeshInfo',
  requiresConfirmation: false,
  tauriCommand: 'cae_api::generate_3d_mesh',
  isDestructive: false,
  examples: ['生成结构化网格', '划分网格，单元尺寸0.01m']
}

const import_geometry: ToolDefinition = {
  name: 'import_geometry',
  description: '导入外部几何文件（STEP/IGES/STL）',
  category: 'modeling',
  params: [
    { name: 'filePath', type: 'string', description: '文件路径', required: true },
    { name: 'format', type: 'string', description: '文件格式', required: true, enum: ['step', 'iges', 'stl'] }
  ],
  returnType: 'GeometryInfo',
  requiresConfirmation: false,
  tauriCommand: 'step_import::import_step_file',
  isDestructive: false,
  examples: ['导入STEP文件']
}

// ============================================================================
// 工具定义 - 仿真工具集扩展 (V2.4-007)
// ============================================================================

const set_solver: ToolDefinition = {
  name: 'set_solver',
  description: '设置求解器参数和求解类型',
  category: 'simulation',
  params: [
    { name: 'solverName', type: 'string', description: '求解器名称', required: false, default: 'CalculiX' },
    { name: 'analysisType', type: 'string', description: '分析类型', required: true, enum: ['static', 'modal', 'thermal_steady', 'thermal_transient', 'buckling', 'frequency_response', 'contact', 'explicit_dynamic'] },
    { name: 'options', type: 'object', description: '求解器选项', required: false }
  ],
  returnType: 'SolverConfig',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['设置求解器为CalculiX', '切换到模态分析']
}

const generate_input_file: ToolDefinition = {
  name: 'generate_input_file',
  description: '生成求解器输入文件（.inp）',
  category: 'simulation',
  params: [
    { name: 'analysisType', type: 'string', description: '分析类型', required: true },
    { name: 'outputPath', type: 'string', description: '输出文件路径', required: false }
  ],
  returnType: 'FileInfo',
  requiresConfirmation: false,
  tauriCommand: 'cae_api::generate_input',
  isDestructive: false,
  examples: ['生成inp文件']
}

const check_mesh_quality: ToolDefinition = {
  name: 'check_mesh_quality',
  description: '检查网格质量（纵横比、雅可比、扭曲度等）',
  category: 'simulation',
  params: [
    { name: 'criteria', type: 'string', description: '质量标准', required: false, enum: ['strict', 'normal', 'relaxed'], default: 'normal' }
  ],
  returnType: 'MeshQualityReport',
  requiresConfirmation: false,
  tauriCommand: 'cae_api::check_mesh_quality',
  isDestructive: false,
  examples: ['检查网格质量']
}

// ============================================================================
// 工具定义 - 代码工具集 (V2.4-008)
// ============================================================================

const write_file: ToolDefinition = {
  name: 'write_file',
  description: '写入文件内容',
  category: 'code',
  params: [
    { name: 'filePath', type: 'string', description: '文件路径', required: true },
    { name: 'content', type: 'string', description: '文件内容', required: true },
    { name: 'overwrite', type: 'boolean', description: '是否覆盖已有文件', required: false, default: false }
  ],
  returnType: 'FileInfo',
  requiresConfirmation: true,
  tauriCommand: 'file::write_file_content',
  isDestructive: true,
  examples: ['写入Python脚本', '保存结果到文件']
}

const read_file: ToolDefinition = {
  name: 'read_file',
  description: '读取文件内容',
  category: 'code',
  params: [
    { name: 'filePath', type: 'string', description: '文件路径', required: true },
    { name: 'encoding', type: 'string', description: '文件编码', required: false, default: 'utf-8' }
  ],
  returnType: 'FileContent',
  requiresConfirmation: false,
  tauriCommand: 'file::read_file_content',
  isDestructive: false,
  examples: ['读取结果文件', '查看inp文件内容']
}

const execute_code: ToolDefinition = {
  name: 'execute_code',
  description: '执行代码（Python/Shell等）',
  category: 'code',
  params: [
    { name: 'code', type: 'string', description: '要执行的代码', required: true },
    { name: 'language', type: 'string', description: '编程语言', required: true, enum: ['python', 'shell', 'javascript'] },
    { name: 'timeout', type: 'number', description: '超时时间（秒）', required: false, default: 30 }
  ],
  returnType: 'ExecutionResult',
  requiresConfirmation: true,
  tauriCommand: 'code_exec::execute_code',
  isDestructive: false,
  examples: ['执行Python脚本', '运行数据处理代码']
}

const git_operations: ToolDefinition = {
  name: 'git_operations',
  description: 'Git版本控制操作',
  category: 'code',
  params: [
    { name: 'operation', type: 'string', description: 'Git操作', required: true, enum: ['status', 'commit', 'diff', 'log', 'branch'] },
    { name: 'message', type: 'string', description: '提交消息（commit时需要）', required: false },
    { name: 'branch', type: 'string', description: '分支名', required: false }
  ],
  returnType: 'GitResult',
  requiresConfirmation: true,
  isDestructive: true,
  examples: ['Git提交', '查看Git状态']
}

// ============================================================================
// 工具定义 - 笔记工具集 (V2.4-009)
// ============================================================================

const create_note: ToolDefinition = {
  name: 'create_note',
  description: '创建笔记',
  category: 'notes',
  params: [
    { name: 'title', type: 'string', description: '笔记标题', required: true },
    { name: 'content', type: 'string', description: '笔记内容（支持Markdown和LaTeX）', required: true },
    { name: 'category', type: 'string', description: '笔记分类', required: false }
  ],
  returnType: 'NoteInfo',
  requiresConfirmation: false,
  tauriCommand: 'file::create_file',
  isDestructive: false,
  examples: ['创建分析报告笔记', '记录仿真参数']
}

const read_note: ToolDefinition = {
  name: 'read_note',
  description: '读取笔记内容',
  category: 'notes',
  params: [
    { name: 'noteId', type: 'string', description: '笔记ID', required: true }
  ],
  returnType: 'NoteContent',
  requiresConfirmation: false,
  tauriCommand: 'file::get_file',
  isDestructive: false,
  examples: ['读取笔记']
}

const update_note: ToolDefinition = {
  name: 'update_note',
  description: '更新笔记内容',
  category: 'notes',
  params: [
    { name: 'noteId', type: 'string', description: '笔记ID', required: true },
    { name: 'content', type: 'string', description: '新内容', required: true },
    { name: 'append', type: 'boolean', description: '是否追加而非替换', required: false, default: false }
  ],
  returnType: 'NoteInfo',
  requiresConfirmation: false,
  tauriCommand: 'file::update_file',
  isDestructive: false,
  examples: ['更新笔记内容', '追加分析结果到笔记']
}

// ============================================================================
// 工具定义 - 后处理工具集 (V2.4-010)
// ============================================================================

const render_contour: ToolDefinition = {
  name: 'render_contour',
  description: '渲染结果云图（应力/位移/温度等）',
  category: 'postprocess',
  params: [
    { name: 'field', type: 'string', description: '场变量', required: true, enum: ['von_mises_stress', 'displacement', 'temperature', 'principal_stress', 'strain'] },
    { name: 'colorMap', type: 'string', description: '色图类型', required: false, enum: ['rainbow', 'jet', 'coolwarm', 'viridis'], default: 'rainbow' },
    { name: 'showDeformed', type: 'boolean', description: '是否显示变形', required: false, default: true },
    { name: 'scaleFactor', type: 'number', description: '变形放大系数', required: false, default: 1.0 }
  ],
  returnType: 'ContourImage',
  requiresConfirmation: false,
  tauriCommand: 'cae_api::get_color_map',
  isDestructive: false,
  examples: ['显示Von Mises应力云图', '渲染位移云图']
}

const slice_model: ToolDefinition = {
  name: 'slice_model',
  description: '对模型进行切片查看内部结果',
  category: 'postprocess',
  params: [
    { name: 'plane', type: 'string', description: '切面方向', required: true, enum: ['XY', 'XZ', 'YZ'] },
    { name: 'position', type: 'number', description: '切面位置（归一化 0~1）', required: false, default: 0.5 },
    { name: 'field', type: 'string', description: '场变量', required: false, default: 'von_mises_stress' }
  ],
  returnType: 'SliceImage',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['XY平面切片查看', '在中间位置切片']
}

const export_data: ToolDefinition = {
  name: 'export_data',
  description: '导出仿真数据（CSV/JSON/VTK格式）',
  category: 'postprocess',
  params: [
    { name: 'format', type: 'string', description: '导出格式', required: true, enum: ['csv', 'json', 'vtk'] },
    { name: 'data', type: 'string', description: '数据类型', required: true, enum: ['stress', 'displacement', 'mesh', 'all'] },
    { name: 'filePath', type: 'string', description: '输出文件路径', required: false }
  ],
  returnType: 'ExportResult',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['导出应力数据为CSV', '导出VTK文件']
}

const generate_chart: ToolDefinition = {
  name: 'generate_chart',
  description: '生成数据图表（曲线图/柱状图/散点图等）',
  category: 'postprocess',
  params: [
    { name: 'chartType', type: 'string', description: '图表类型', required: true, enum: ['line', 'bar', 'scatter', 'contour_2d'] },
    { name: 'xData', type: 'array', description: 'X轴数据', required: true },
    { name: 'yData', type: 'array', description: 'Y轴数据', required: true },
    { name: 'title', type: 'string', description: '图表标题', required: false },
    { name: 'xLabel', type: 'string', description: 'X轴标签', required: false },
    { name: 'yLabel', type: 'string', description: 'Y轴标签', required: false }
  ],
  returnType: 'ChartImage',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['生成应力-位移曲线', '绘制频率响应图']
}

// ============================================================================
// 工具定义 - 分析工具集 (V2.4-013 辅助)
// ============================================================================

const validate_results: ToolDefinition = {
  name: 'validate_results',
  description: '验证仿真结果的合理性和精度',
  category: 'analysis',
  params: [
    { name: 'resultType', type: 'string', description: '结果类型', required: true },
    { name: 'values', type: 'object', description: '结果数值', required: true },
    { name: 'criteria', type: 'string', description: '验证标准', required: false, enum: ['engineering', 'strict', 'custom'] }
  ],
  returnType: 'ValidationResult',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['验证应力结果是否合理', '检查结果精度']
}

const compare_results: ToolDefinition = {
  name: 'compare_results',
  description: '对比两组仿真结果',
  category: 'analysis',
  params: [
    { name: 'resultA', type: 'object', description: '结果A', required: true },
    { name: 'resultB', type: 'object', description: '结果B', required: true },
    { name: 'metrics', type: 'array', description: '对比指标', required: false }
  ],
  returnType: 'ComparisonReport',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['对比两种网格的结果', '比较不同载荷工况']
}

// ============================================================================
// 工具定义 - ML 预测工具 (V2.5)
// ============================================================================

const predict_material_properties: ToolDefinition = {
  name: 'predict_material_properties',
  description: '使用 ML Surrogate Model 预测材料性能（弹性模量、屈服强度、热导率等），无需运行仿真即可快速筛选候选材料',
  category: 'analysis',
  params: [
    { name: 'composition', type: 'object', description: '材料成分（元素 -> 原子百分比），如 { Fe: 70, Cr: 19, Ni: 9 }', required: true },
    { name: 'targetProperties', type: 'array', description: '要预测的属性列表', required: true, enum: ['elastic_modulus', 'yield_strength', 'thermal_conductivity', 'density', 'poissons_ratio', 'shear_modulus', 'bulk_modulus'] },
    { name: 'modelName', type: 'string', description: '模型名称', required: false, default: 'mock_chgnet' }
  ],
  returnType: 'PredictionResponse',
  requiresConfirmation: false,
  tauriCommand: 'ml_predict::predict_material_properties',
  isDestructive: false,
  examples: ['预测304不锈钢的弹性模量', '用ML预测Al6061的力学性能']
}

const verify_ml_prediction: ToolDefinition = {
  name: 'verify_ml_prediction',
  description: '对 ML 预测结果进行 CAELab 仿真验证，自动提交仿真任务并对比结果',
  category: 'analysis',
  params: [
    { name: 'composition', type: 'object', description: '材料成分', required: true },
    { name: 'predictedProperties', type: 'object', description: 'ML 预测的属性值', required: true }
  ],
  returnType: 'VerificationResult',
  requiresConfirmation: true,
  tauriCommand: 'ml_predict::predict_material_properties',
  isDestructive: false,
  examples: ['验证ML预测结果', '用CAELab仿真验证预测']
}

// ============================================================================
// 工具定义 - ML 势函数工具 (V2.6)
// ============================================================================

const select_ml_potential: ToolDefinition = {
  name: 'select_ml_potential',
  description: '根据体系元素和原子数自动推荐最合适的 ML 势函数类型（MACE/CHGNet/NequIP/NEP 等），判断是否需要训练',
  category: 'analysis',
  params: [
    { name: 'elements', type: 'array', description: '原子种类列表，如 ["Fe", "Cr", "Ni"]', required: true },
    { name: 'numAtoms', type: 'number', description: '体系原子总数', required: true },
    { name: 'accuracyRequirement', type: 'string', description: '精度要求: high/medium/low', required: false, default: 'medium' }
  ],
  returnType: 'PotentialSelectionResult',
  requiresConfirmation: false,
  tauriCommand: 'ml_potential::auto_select_potential',
  isDestructive: false,
  examples: ['推荐FeCrNi合金的势函数', '10000原子体系用什么势函数']
}

const train_ml_potential: ToolDefinition = {
  name: 'train_ml_potential',
  description: '使用 DFT 数据训练 ML 势函数（NequIP/MACE/NEP），返回训练结果和精度指标',
  category: 'simulation',
  params: [
    { name: 'potentialType', type: 'string', description: '势函数类型: nequip/mace/nep/mtp/ace', required: true },
    { name: 'trainingDataPath', type: 'string', description: '训练数据文件路径 (extxyz 格式)', required: true },
    { name: 'epochs', type: 'number', description: '训练轮数', required: false, default: 500 },
    { name: 'cutoff', type: 'number', description: '截断半径 (Å)', required: false, default: 5.0 }
  ],
  returnType: 'TrainingResult',
  requiresConfirmation: true,
  tauriCommand: 'ml_potential::submit_training_job',
  isDestructive: false,
  examples: ['训练FeCrNi的NequIP势函数', '用DFT数据训练MACE势']
}

const run_md_ml: ToolDefinition = {
  name: 'run_md_with_ml_potential',
  description: '使用 ML 势函数运行分子动力学模拟，支持大体系 (>10万原子) 高效计算',
  category: 'simulation',
  params: [
    { name: 'potentialName', type: 'string', description: 'ML 势函数名称', required: true },
    { name: 'ensemble', type: 'string', description: '系综: NVE/NVT/NPT', required: true },
    { name: 'temperature', type: 'number', description: '温度 (K)', required: false, default: 300 },
    { name: 'numSteps', type: 'number', description: '模拟步数', required: false, default: 10000 },
    { name: 'timestepFs', type: 'number', description: '时间步长 (fs)', required: false, default: 1.0 }
  ],
  returnType: 'MdResult',
  requiresConfirmation: true,
  tauriCommand: 'ml_potential::run_md_with_ml_potential',
  isDestructive: false,
  examples: ['用MACE势跑10万原子NVT模拟', 'ML势函数MD模拟300K']
}

const validate_ml_potential: ToolDefinition = {
  name: 'validate_ml_potential',
  description: '验证 ML 势函数质量，对比 DFT 参考值计算能量和力的 RMSE',
  category: 'analysis',
  params: [
    { name: 'potentialName', type: 'string', description: '势函数名称', required: true },
    { name: 'testDataPath', type: 'string', description: '测试数据路径', required: true }
  ],
  returnType: 'ValidationResult',
  requiresConfirmation: false,
  tauriCommand: 'ml_potential::validate_ml_potential',
  isDestructive: false,
  examples: ['验证MACE势函数精度', '检查训练好的势函数质量']
}

// ============================================================================
// 工具定义 - 多尺度 Surrogate 工具 (V2.7)
// ============================================================================

const segment_microstructure: ToolDefinition = {
  name: 'segment_microstructure',
  description: '使用 UNet 对微观结构图像进行相分割（固相/孔隙/第二相），提取相体积分数、孔隙率、晶粒尺寸等特征',
  category: 'analysis',
  params: [
    { name: 'imageId', type: 'string', description: '微观结构图像 ID', required: true },
    { name: 'segmentationTarget', type: 'string', description: '分割目标: phase/pore/grain', required: false, default: 'phase' }
  ],
  returnType: 'SegmentationResult',
  requiresConfirmation: false,
  tauriCommand: 'multiscale_surrogate::segment_microstructure',
  isDestructive: false,
  examples: ['分割SEM图像提取相分布', '检测微观结构孔隙率']
}

const predict_macro_property: ToolDefinition = {
  name: 'predict_macro_property',
  description: '从微观结构特征直接预测宏观等效性能（弹性模量/热导率/屈服强度），支持不确定性量化',
  category: 'analysis',
  params: [
    { name: 'imageId', type: 'string', description: '微观结构图像 ID', required: false },
    { name: 'processingParams', type: 'object', description: '工艺参数（温度/压力/时间等）', required: false },
    { name: 'targetProperties', type: 'array', description: '预测目标列表', required: true },
    { name: 'withUncertainty', type: 'boolean', description: '是否输出不确定性', required: false, default: true }
  ],
  returnType: 'MacroPropertyPredictionResponse',
  requiresConfirmation: false,
  tauriCommand: 'multiscale_surrogate::predict_macro_properties',
  isDestructive: false,
  examples: ['预测Al6061的宏观弹性模量', '从微观结构预测热导率']
}

const run_multiscale_workflow: ToolDefinition = {
  name: 'run_multiscale_workflow',
  description: '端到端多尺度直通工作流：微观结构图像 → UNet分割 → 特征提取 → ML预测宏观性能 → 可选CAELab验证',
  category: 'simulation',
  params: [
    { name: 'imageId', type: 'string', description: '微观结构图像 ID', required: true },
    { name: 'targetProperties', type: 'array', description: '预测目标', required: true },
    { name: 'autoVerify', type: 'boolean', description: '不确定性高时是否自动触发CAELab验证', required: false, default: false }
  ],
  returnType: 'MultiScaleWorkflowResult',
  requiresConfirmation: true,
  tauriCommand: 'multiscale_surrogate::predict_macro_properties',
  isDestructive: false,
  examples: ['运行完整多尺度工作流', '从SEM图像预测宏观性能并验证']
}

// ============================================================================
// 工具定义 - 主动学习工具 (V2.8)
// ============================================================================

const analyze_data_coverage: ToolDefinition = {
  name: 'analyze_data_coverage',
  description: '分析材料数据库的数据覆盖度，识别成分空间和工艺空间的数据盲区，输出覆盖度评分和盲区列表',
  category: 'analysis',
  params: [],
  returnType: 'CoverageAnalysis',
  requiresConfirmation: false,
  tauriCommand: 'material_data_platform::analyze_data_coverage',
  isDestructive: false,
  examples: ['分析数据覆盖度', '找出数据盲区']
}

const get_active_learning_recommendations: ToolDefinition = {
  name: 'get_active_learning_recommendations',
  description: '基于贝叶斯优化主动推荐下一个最有价值的仿真点，优先选择高不确定性+高期望改进区域，用最少仿真次数获得最高精度',
  category: 'analysis',
  params: [
    { name: 'targetProperty', type: 'string', description: '目标属性: elastic_modulus/yield_strength/thermal_conductivity', required: true },
    { name: 'numRecommendations', type: 'number', description: '推荐数量', required: false, default: 5 },
    { name: 'strategy', type: 'string', description: '采样策略: bayesian/greedy/random', required: false, default: 'bayesian' }
  ],
  returnType: 'ActiveLearningRecommendation',
  requiresConfirmation: false,
  tauriCommand: 'material_data_platform::get_active_learning_recommendations',
  isDestructive: false,
  examples: ['推荐下一个仿真点', '用主动学习优化弹性模量预测']
}

const run_closed_loop: ToolDefinition = {
  name: 'run_closed_loop',
  description: '执行主动学习闭环：推荐点 → CAELab仿真 → 结果写回数据库 → 模型自动更新，全程无需人工介入',
  category: 'simulation',
  params: [
    { name: 'materialName', type: 'string', description: '材料名称', required: true },
    { name: 'composition', type: 'object', description: '材料成分', required: true },
    { name: 'processingParams', type: 'object', description: '工艺参数', required: true },
    { name: 'targetProperty', type: 'string', description: '目标属性', required: true }
  ],
  returnType: 'ClosedLoopResult',
  requiresConfirmation: true,
  tauriCommand: 'material_data_platform::run_closed_loop_verification',
  isDestructive: false,
  examples: ['执行闭环验证', '自动补充数据并更新模型']
}

// ============================================================================
// 工具注册表
// ============================================================================

/** 所有已注册的工具 */
const registeredTools: Map<string, ToolDefinition> = new Map()

/** 初始化工具注册表 */
function initToolRegistry(): void {
  const allTools: ToolDefinition[] = [
    // 基础仿真工具 (V2.4-005)
    get_model_info, set_material, apply_bc, run_simulation, get_results,
    // 建模工具 (V2.4-006)
    create_geometry, csg_boolean, query_geometry, generate_mesh, import_geometry,
    // 仿真扩展 (V2.4-007)
    set_solver, generate_input_file, check_mesh_quality,
    // 代码工具 (V2.4-008)
    write_file, read_file, execute_code, git_operations,
    // 笔记工具 (V2.4-009)
    create_note, read_note, update_note,
    // 后处理工具 (V2.4-010)
    render_contour, slice_model, export_data, generate_chart,
    // 分析工具
    validate_results, compare_results,
    // ML 预测工具 (V2.5)
    predict_material_properties, verify_ml_prediction,
    // ML 势函数工具 (V2.6)
    select_ml_potential, train_ml_potential, run_md_ml, validate_ml_potential,
    // 多尺度 Surrogate 工具 (V2.7)
    segment_microstructure, predict_macro_property, run_multiscale_workflow,
    // 主动学习工具 (V2.8)
    analyze_data_coverage, get_active_learning_recommendations, run_closed_loop,
  ]

  for (const tool of allTools) {
    registeredTools.set(tool.name, tool)
  }
}

// 初始化
initToolRegistry()

// ============================================================================
// 工具注册表 API
// ============================================================================

export const toolRegistry = {
  /**
   * 获取工具定义
   */
  get(name: string): ToolDefinition | undefined {
    return registeredTools.get(name)
  },

  /**
   * 获取所有工具定义
   */
  getAll(): ToolDefinition[] {
    return Array.from(registeredTools.values())
  },

  /**
   * 按分类获取工具
   */
  getByCategory(category: ToolDefinition['category']): ToolDefinition[] {
    return this.getAll().filter(t => t.category === category)
  },

  /**
   * 注册新工具
   */
  register(tool: ToolDefinition): void {
    registeredTools.set(tool.name, tool)
  },

  /**
   * 注销工具
   */
  unregister(name: string): boolean {
    return registeredTools.delete(name)
  },

  /**
   * 检查工具是否存在
   */
  has(name: string): boolean {
    return registeredTools.has(name)
  },

  /**
   * 获取工具数量
   */
  get count(): number {
    return registeredTools.size
  },

  /**
   * 获取工具名称列表
   */
  get names(): string[] {
    return Array.from(registeredTools.keys())
  },

  /**
   * 生成工具描述（用于 LLM prompt）
   */
  getToolDescriptionsForLLM(): string {
    const tools = this.getAll()
    let desc = '可用工具列表:\n\n'
    for (const tool of tools) {
      desc += `### ${tool.name}\n`
      desc += `描述: ${tool.description}\n`
      desc += `分类: ${tool.category}\n`
      if (tool.params.length > 0) {
        desc += '参数:\n'
        for (const param of tool.params) {
          const req = param.required ? '必填' : '可选'
          const def = param.default !== undefined ? `, 默认: ${param.default}` : ''
          const enumVals = param.enum ? `, 可选值: ${param.enum.join('/')}` : ''
          desc += `  - ${param.name} (${param.type}, ${req}${def}${enumVals}): ${param.description}\n`
        }
      }
      if (tool.examples && tool.examples.length > 0) {
        desc += `示例: ${tool.examples.join(', ')}\n`
      }
      desc += '\n'
    }
    return desc
  },

  /**
   * 获取 OpenAI function calling 格式的工具定义
   */
  getOpenAITools(): Array<{ type: 'function'; function: { name: string; description: string; parameters: Record<string, unknown> } }> {
    return this.getAll().map(tool => ({
      type: 'function' as const,
      function: {
        name: tool.name,
        description: tool.description,
        parameters: {
          type: 'object',
          properties: tool.params.reduce((acc, param) => {
            const schema: Record<string, unknown> = {
              type: param.type,
              description: param.description
            }
            if (param.enum) schema.enum = param.enum
            if (param.default !== undefined) schema.default = param.default
            if (param.minimum !== undefined) schema.minimum = param.minimum
            if (param.maximum !== undefined) schema.maximum = param.maximum
            acc[param.name] = schema
            return acc
          }, {} as Record<string, unknown>),
          required: tool.params.filter(p => p.required).map(p => p.name)
        }
      }
    }))
  }
}
