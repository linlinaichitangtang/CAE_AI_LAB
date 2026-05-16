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
// 工具定义 - 图像分析工具 (V3.6)
// ============================================================================

const analyze_sem_image: ToolDefinition = {
  name: 'analyze_sem_image',
  description: '分析 SEM/TEM/光学 断裂面图像，提取特征（韧窝/条纹/裂纹/孔洞），输出特征统计和尺寸分布，用于材料失效分析',
  category: 'analysis',
  params: [
    { name: 'imageId', type: 'string', description: '图像 ID', required: true },
    { name: 'profileName', type: 'string', description: '分析配置: dimple/striation/cleavage/crack/full', required: false, default: 'full' },
    { name: 'pixelCalibration', type: 'number', description: '像素校准 (nm/pixel)', required: false, default: 1.0 }
  ],
  returnType: 'ImageAnalysisResult',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['分析SEM断裂面图像', '检测韧窝特征', '提取疲劳条纹间距']
}

const detect_fracture_features: ToolDefinition = {
  name: 'detect_fracture_features',
  description: '使用 ML 模型检测断裂面特征（韧窝/解理面/疲劳条带/二次裂纹），并量化特征尺寸和密度',
  category: 'analysis',
  params: [
    { name: 'imageId', type: 'string', description: '图像 ID', required: true },
    { name: 'featureTypes', type: 'array', description: '要检测的特征类型: dimple/striation/cleavage/crack/pore', required: false, default: ['dimple', 'crack'] },
    { name: 'modelId', type: 'string', description: 'ML 模型 ID（可选，使用默认模型）', required: false }
  ],
  returnType: 'FeatureDetectionResult',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['检测断裂面韧窝', '识别疲劳条带', '量化裂纹密度']
}

const correlate_with_simulation: ToolDefinition = {
  name: 'correlate_with_simulation',
  description: '将图像分析结果与 MD/FE 模拟数据进行跨尺度关联分析，输出断裂机制判断和材料性能预测',
  category: 'analysis',
  params: [
    { name: 'imageAnalysisId', type: 'string', description: '图像分析结果 ID', required: true },
    { name: 'simulationData', type: 'object', description: '模拟数据 {maxStress, maxDisplacement, strainEnergy}', required: false },
    { name: 'materialProperty', type: 'string', description: '材料属性名称', required: false, default: 'TC4 Titanium Alloy' }
  ],
  returnType: 'CrossScaleCorrelation',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['关联SEM和MD模拟结果', '跨尺度分析断裂机制']
}

// ============================================================================
// 工具定义 - ML 训练工具 (V3.6)
// ============================================================================

const create_ml_dataset: ToolDefinition = {
  name: 'create_ml_dataset',
  description: '创建 ML 训练数据集，管理 SEM/TEM 图像和标注数据，支持分类/检测/分割任务',
  category: 'system',
  params: [
    { name: 'datasetName', type: 'string', description: '数据集名称', required: true },
    { name: 'rootPath', type: 'string', description: '数据根目录路径', required: true },
    { name: 'classes', type: 'array', description: '分类类别列表', required: true }
  ],
  returnType: 'DatasetInfo',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['创建断裂特征数据集', '准备SEM图像训练集']
}

const train_image_classifier: ToolDefinition = {
  name: 'train_image_classifier',
  description: '训练图像分类/检测/分割模型（ResNet/UNet/YOLOv8），支持数据增强和迁移学习',
  category: 'system',
  params: [
    { name: 'datasetId', type: 'string', description: '数据集 ID', required: true },
    { name: 'modelType', type: 'string', description: '模型类型: classifier/detector/segmenter', required: true },
    { name: 'architecture', type: 'string', description: '网络架构: resnet18/resnet50/unet/yolov8', required: false, default: 'resnet18' },
    { name: 'epochs', type: 'number', description: '训练轮数', required: false, default: 50 },
    { name: 'transferLearning', type: 'boolean', description: '是否使用迁移学习', required: false, default: true }
  ],
  returnType: 'TrainingResult',
  requiresConfirmation: true,
  isDestructive: false,
  examples: ['训练韧窝分类器', '训练裂纹检测模型', '训练SEM图像分割模型']
}

const predict_with_trained_model: ToolDefinition = {
  name: 'predict_with_trained_model',
  description: '使用训练好的模型对新图像进行预测（分类/检测/分割）',
  category: 'system',
  params: [
    { name: 'imageId', type: 'string', description: '待预测图像 ID', required: true },
    { name: 'modelId', type: 'string', description: '训练好的模型 ID', required: true }
  ],
  returnType: 'PredictionResult',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['用训练模型预测SEM图像', '识别断裂特征类型']
}

// ============================================================================
// 工具定义 - Active Learning 工具 (V3.7)
// ============================================================================

const initialize_active_learning: ToolDefinition = {
  name: 'initialize_active_learning',
  description: '初始化主动学习数据池，准备采集策略（uncertainty/variance/density）',
  category: 'system',
  params: [
    { name: 'features', type: 'array', description: '特征矩阵 [[f1, f2, ...], ...]', required: true },
    { name: 'strategy', type: 'string', description: '采集策略: uncertainty/variance/density/expected_model_change/random', required: false, default: 'uncertainty' },
    { name: 'batchSize', type: 'number', description: '每轮采集数量', required: false, default: 10 }
  ],
  returnType: 'ActiveLearningPool',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['初始化主动学习', '设置不确定度采集策略']
}

const acquire_next_points: ToolDefinition = {
  name: 'acquire_next_points',
  description: '根据当前采集策略推荐最有价值的数据点进行标注/实验',
  category: 'system',
  params: [
    { name: 'poolId', type: 'string', description: '数据池 ID', required: true },
    { name: 'numPoints', type: 'number', description: '采集点数', required: false, default: 5 }
  ],
  returnType: 'AcquisitionResult',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['推荐下一个实验点', '选择最有价值的SEM图像']
}

const run_active_learning_iteration: ToolDefinition = {
  name: 'run_active_learning_iteration',
  description: '执行一轮主动学习：选择点 → 更新模型 → 评估指标',
  category: 'system',
  params: [
    { name: 'poolId', type: 'string', description: '数据池 ID', required: true },
    { name: 'labeledData', type: 'array', description: '新标注数据 [{features, label}]', required: true }
  ],
  returnType: 'ActiveLearningIteration',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['执行主动学习迭代', '更新模型并选择下一批点']
}

// ============================================================================
// 工具定义 - PINN 工具 (V3.7)
// ============================================================================

const train_pinn_model: ToolDefinition = {
  name: 'train_pinn_model',
  description: '训练物理约束神经网络 (PINN)，将物理定律嵌入训练过程，满足边界条件和平衡方程',
  category: 'system',
  params: [
    { name: 'modelName', type: 'string', description: '模型名称', required: true },
    { name: 'physicsType', type: 'string', description: '物理类型: linear_elasticity/nonlinear_elasticity/heat_equation/navier_stokes/diffusion', required: true },
    { name: 'dimension', type: 'number', description: '维度: 1/2/3', required: false, default: 2 },
    { name: 'epochs', type: 'number', description: '训练轮数', required: false, default: 5000 },
    { name: 'physicsWeight', type: 'number', description: '物理损失权重', required: false, default: 0.1 }
  ],
  returnType: 'PINNTrainingResult',
  requiresConfirmation: true,
  isDestructive: false,
  examples: ['训练线性弹性PINN', '训练热传导物理约束模型']
}

const predict_pinn: ToolDefinition = {
  name: 'predict_pinn',
  description: '使用训练好的 PINN 模型进行预测，结果满足物理约束',
  category: 'system',
  params: [
    { name: 'pinnId', type: 'string', description: 'PINN 模型 ID', required: true },
    { name: 'position', type: 'array', description: '位置坐标 [x] 或 [x, y] 或 [x, y, z]', required: true },
    { name: 'time', type: 'number', description: '时间（仅对瞬态问题）', required: false }
  ],
  returnType: 'PINNPrediction',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['PINN预测应力分布', '预测温度场']
}

// ============================================================================
// 工具定义 - Surrogate Model 工具 (V3.7)
// ============================================================================

const create_surrogate_model: ToolDefinition = {
  name: 'create_surrogate_model',
  description: '创建代理模型，用 ML 模型替代完整 MD/FE 求解器，加速预测',
  category: 'system',
  params: [
    { name: 'modelName', type: 'string', description: '模型名称', required: true },
    { name: 'inputDimensions', type: 'array', description: '输入维度名称列表', required: true },
    { name: 'outputDimensions', type: 'array', description: '输出维度名称列表', required: true },
    { name: 'modelType', type: 'string', description: '模型类型: ANN/GNN/KNN/SVR/RandomForest/GaussianProcess', required: false, default: 'ANN' }
  ],
  returnType: 'SurrogateModel',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['创建应力代理模型', '创建位移预测代理模型']
}

const train_surrogate: ToolDefinition = {
  name: 'train_surrogate',
  description: '从 MD/FE 模拟数据训练代理模型，学习输入参数到输出的映射',
  category: 'system',
  params: [
    { name: 'surrogateId', type: 'string', description: '代理模型 ID', required: true },
    { name: 'trainingData', type: 'array', description: '训练数据 [{inputs: [], outputs: []}]', required: true },
    { name: 'epochs', type: 'number', description: '训练轮数', required: false, default: 300 }
  ],
  returnType: 'SurrogateTrainingResult',
  requiresConfirmation: true,
  isDestructive: false,
  examples: ['训练Surrogate代理模型', '从MD数据训练替代模型']
}

const predict_with_surrogate: ToolDefinition = {
  name: 'predict_with_surrogate',
  description: '使用训练好的代理模型快速预测，无需运行完整 MD/FE 模拟',
  category: 'system',
  params: [
    { name: 'surrogateId', type: 'string', description: '代理模型 ID', required: true },
    { name: 'inputs', type: 'array', description: '输入参数 [温度, 压力, 材料属性, ...]', required: true }
  ],
  returnType: 'SurrogatePrediction',
  requiresConfirmation: false,
  isDestructive: false,
  examples: ['代理模型快速预测应力', '秒级预测代替完整模拟']
}

// ============================================================================
// TC4 失效分析工具 (V3.8)
// ============================================================================

const tc4_generate_synthetic_data: ToolDefinition = {
  name: 'tc4_generate_synthetic_data',
  description: '生成 TC4 合成数据，用于验证多模态失效分析流程。生成 ebsd_synthetic.csv, sem_synthetic.csv, load_synthetic.csv, failure_labels.csv 四个文件',
  category: 'simulation',
  params: [
    { name: 'num_samples', type: 'number', description: '样本数量', required: false, default: 100 },
    { name: 'output_dir', type: 'string', description: '输出目录', required: false, default: './tc4_synthetic' },
    { name: 'use_tc4', type: 'boolean', description: '是否使用 TC4 HCP 参数（false 用 Al FCC 先跑通）', required: false, default: true },
    { name: 'seed', type: 'number', description: '随机种子', required: false, default: 42 }
  ],
  returnType: 'string',
  requiresConfirmation: false,
  tauriCommand: 'tc4_failure::tc4_generate_synthetic',
  isDestructive: false,
  examples: ['生成 100 个 TC4 合成样本', '生成 Al FCC 测试数据']
}

const tc4_load_ebsd: ToolDefinition = {
  name: 'tc4_load_ebsd',
  description: '加载 EBSD 晶格数据，从 .ctf/.ang 文件或 CSV 提取 80 维晶格特征向量',
  category: 'simulation',
  params: [
    { name: 'ebsd_path', type: 'string', description: 'EBSD 文件路径（.ctf/.ang/.csv）', required: true },
    { name: 'material', type: 'string', description: '材料类型（TC4 或 Al）', required: false, default: 'TC4' }
  ],
  returnType: 'EbsdFeatures',
  requiresConfirmation: false,
  tauriCommand: 'tc4_failure::tc4_load_ebsd',
  isDestructive: false,
  examples: ['加载 EBSD 数据', '读取 TC4 晶格特征']
}

const tc4_load_sem: ToolDefinition = {
  name: 'tc4_load_sem',
  description: '加载 SEM 断口图像，提取 768 维 ViT 特征向量',
  category: 'simulation',
  params: [
    { name: 'sem_image_path', type: 'string', description: 'SEM 图像路径（.jpg/.png/.csv）', required: true }
  ],
  returnType: 'SemFeatures',
  requiresConfirmation: false,
  tauriCommand: 'tc4_failure::tc4_load_sem',
  isDestructive: false,
  examples: ['加载 SEM 图像', '提取断口图像特征']
}

const tc4_load_load: ToolDefinition = {
  name: 'tc4_load_load',
  description: '加载载荷历史数据，从 CSV 提取 30 维载荷特征向量',
  category: 'simulation',
  params: [
    { name: 'load_path', type: 'string', description: '载荷数据文件路径（.csv）', required: true }
  ],
  returnType: 'LoadFeatures',
  requiresConfirmation: false,
  tauriCommand: 'tc4_failure::tc4_load_load',
  isDestructive: false,
  examples: ['加载载荷数据', '读取载荷历史']
}

const tc4_predict_failure: ToolDefinition = {
  name: 'tc4_predict_failure',
  description: '多模态失效预测 - 三路输入（EBSD + SEM + Load）预测失效模式和疲劳寿命',
  category: 'simulation',
  params: [
    { name: 'ebsd', type: 'object', description: 'EBSD 晶格特征 {sample_id, features, a, c, c_to_a}', required: true },
    { name: 'sem', type: 'object', description: 'SEM 图像特征 {sample_id, features, quality_score}', required: true },
    { name: 'load', type: 'object', description: 'Load 载荷特征 {sample_id, features, load_type}', required: true }
  ],
  returnType: 'MultimodalPrediction',
  requiresConfirmation: false,
  tauriCommand: 'tc4_failure::tc4_multimodal_predict',
  isDestructive: false,
  examples: ['预测 TC4 失效模式', '多模态预测疲劳寿命']
}

const tc4_md_validation: ToolDefinition = {
  name: 'tc4_md_validation',
  description: 'LAMMPS 分子动力学验证裂纹扩展或位错演化，输入晶格参数和裂纹配置，输出裂纹速度、位错密度、断裂能等',
  category: 'simulation',
  params: [
    { name: 'lattice_a', type: 'number', description: '晶格参数 a (Å)', required: true },
    { name: 'lattice_c', type: 'number', description: '晶格参数 c (Å)', required: true },
    { name: 'crack_plane', type: 'string', description: '裂纹面 (如 (0001), (10-10), (11-20))', required: true },
    { name: 'crack_direction', type: 'string', description: '裂纹方向 (如 [11-20], [1-100])', required: true },
    { name: 'validation_type', type: 'string', description: '验证类型 (crack_propagation / dislocation_evolution)', required: true }
  ],
  returnType: 'string',
  requiresConfirmation: false,
  tauriCommand: 'tc4_failure::tc4_md_validation',
  isDestructive: false,
  examples: ['MD 验证裂纹扩展', '模拟位错演化']
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
    // 图像分析工具 (V3.6)
    analyze_sem_image, detect_fracture_features, correlate_with_simulation,
    // ML 训练工具 (V3.6)
    create_ml_dataset, train_image_classifier, predict_with_trained_model,
    // Active Learning 工具 (V3.7)
    initialize_active_learning, acquire_next_points, run_active_learning_iteration,
    // PINN 工具 (V3.7)
    train_pinn_model, predict_pinn,
    // Surrogate Model 工具 (V3.7)
    create_surrogate_model, train_surrogate, predict_with_surrogate,
    // TC4 失效分析工具 (V3.8)
    tc4_generate_synthetic_data, tc4_load_ebsd, tc4_load_sem, tc4_load_load, tc4_predict_failure, tc4_md_validation,
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
