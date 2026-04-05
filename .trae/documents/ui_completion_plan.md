# CAELab V1.0.2 UI 补全任务实施计划

## 任务概述
根据 `trae_ui_tasks_prompt.md` 的要求，完成 Phase 2 / Phase 3 UI 遗留任务，共 7 项。

## 任务分解与优先级

### [x] 任务 1：仿真模块标签页化补全（SimulationView.vue）
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - 扩展分析类型下拉菜单，添加 transient、fatigue 和 thermal_coupling
  - 为这三种分析类型添加右侧标签页内容
  - 更新 analysisType 类型定义
- **Success Criteria**:
  - 分析类型下拉菜单包含全部 9 种分析类型
  - 三种新分析类型的参数表单内嵌在 SimulationView.vue 右侧面板
  - 标签页切换逻辑正常工作
- **Test Requirements**:
  - `programmatic` TR-1.1: 分析类型下拉菜单包含 9 个选项
  - `programmatic` TR-1.2: 选择新分析类型时右侧面板显示对应参数表单
  - `human-judgement` TR-1.3: 标签页切换流畅，UI 布局合理

### [x] 任务 2：后处理全屏 + 图表联动（SimulationView.vue）
- **Priority**: P1
- **Depends On**: None
- **Description**:
  - 添加"全屏查看"按钮
  - 实现底部图表与 3D 云图的联动
  - 提供"分离/联动"切换按钮
- **Success Criteria**:
  - 求解完成后显示"全屏查看"按钮
  - 点击曲线上的点，3D 视图中高亮对应节点
  - 拖动时间轴滑块，3D 云图同步更新
- **Test Requirements**:
  - `programmatic` TR-2.1: 求解完成后"全屏查看"按钮可见
  - `programmatic` TR-2.2: 点击曲线点时 3D 视图有响应
  - `human-judgement` TR-2.3: 全屏模式视觉效果良好

### [x] 任务 3：材料/边界条件面板重设计（SimulationView.vue）
- **Priority**: P1
- **Depends On**: None
- **Description**:
  - 统一改造材料面板为折叠面板
  - 统一边界条件面板为 COMSOL 风格分类折叠
  - 添加单位下拉功能
- **Success Criteria**:
  - 材料面板符合 COMSOL 风格，有单位下拉
  - 边界条件面板分类清晰，折叠展开正常
  - 所有分析类型的材料/边界条件面板风格一致
- **Test Requirements**:
  - `programmatic` TR-3.1: 材料面板包含单位下拉选项
  - `programmatic` TR-3.2: 边界条件面板分类折叠功能正常
  - `human-judgement` TR-3.3: 面板视觉效果符合 COMSOL 风格

### [x] 任务 4：右侧参数面板规范化（SimulationView.vue）
- **Priority**: P2
- **Depends On**: 任务 1, 任务 3
- **Description**:
  - 统一所有右侧面板宽度为 w-72
  - 统一面板背景、边框、内边距样式
  - 确保所有分析类型的面板一致性
- **Success Criteria**:
  - 所有右侧面板宽度统一为 w-72
  - 面板样式一致，背景为 bg-white，边框为 border-r border-gray-200
  - 内边距统一为 p-4 space-y-4
- **Test Requirements**:
  - `programmatic` TR-4.1: 所有右侧面板宽度为 288px (w-72)
  - `human-judgement` TR-4.2: 面板样式一致，视觉效果统一

### [x] 任务 5：动效和过渡优化
- **Priority**: P2
- **Depends On**: None
- **Description**:
  - 统一 transition 类
  - 添加标签切换动画（滑动下划线指示器）
  - 实现加载骨架屏
- **Success Criteria**:
  - 统一的过渡动画类
  - 标签切换时有滑动下划线指示器
  - 求解进行中显示骨架屏
- **Test Requirements**:
  - `programmatic` TR-5.1: 存在统一的 transition 类定义
  - `human-judgement` TR-5.2: 标签切换动画流畅
  - `human-judgement` TR-5.3: 加载骨架屏效果良好

## 技术实施细节

### 任务 1 实施细节
1. **更新分析类型定义**:
   - 修改 `analysisType` 类型，添加新的分析类型
   - 更新下拉菜单选项

2. **添加新分析类型的参数表单**:
   - 参考 TransientDynamicsView.vue 添加瞬态分析参数
   - 参考 FatigueView.vue 添加疲劳分析参数
   - 参考 ThermalCouplingView.vue 添加热-结构耦合参数

3. **标签页切换逻辑**:
   - 更新类型定义和切换逻辑
   - 确保新分析类型的标签页显示正确

### 任务 2 实施细节
1. **全屏模式**:
   - 在求解完成区域添加"全屏查看"按钮
   - 实现全屏切换逻辑

2. **图表联动**:
   - 实现曲线点击与 3D 视图高亮的联动
   - 实现时间轴滑块与 3D 云图的同步
   - 添加"分离/联动"切换按钮

### 任务 3 实施细节
1. **材料面板改造**:
   - 统一为折叠面板样式
   - 添加材料库选择
   - 添加单位下拉功能

2. **边界条件面板改造**:
   - 统一为 COMSOL 风格分类折叠
   - 按类型分组边界条件
   - 优化交互体验

### 任务 4 实施细节
1. **统一宽度**:
   - 确保所有右侧面板宽度为 w-72
   - 检查并调整布局

2. **面板一致性**:
   - 统一背景、边框、内边距样式
   - 确保所有分析类型的面板风格一致

### 任务 5 实施细节
1. **统一 transition 类**:
   - 在 style.css 或 tailwind.config.js 中定义统一的过渡类

2. **标签切换动画**:
   - 添加滑动下划线指示器
   - 实现 200ms 过渡动画

3. **加载骨架屏**:
   - 求解进行中显示骨架屏
   - 使用 shimmer 动画效果

## 交付标准
1. `analysisType` 包含全部 9 种分析类型
2. 瞬态/疲劳/热耦合三种分析的参数表单内嵌在 SimulationView.vue 右侧面板
3. 后处理全屏模式可切换
4. 材料面板符合 COMSOL 风格（有单位下拉）
5. 右侧面板宽度统一 w-72
6. `git commit` 提交，commit message 以 `feat(ui):` 开头

## 风险与注意事项
1. **不破坏现有功能**：所有现有 analysisType 必须保持正常工作
2. **Rust 命令不修改**：只改 Vue 前端，不做后端 Rust 命令开发
3. **编译通过**：确保 `npm run build` 和 `cargo build` 零错误
4. **不退化**：不做会导致运行时 panic 的修改
5. **移动端兼容**：新增的标签页和面板需要响应式（考虑 iPad 场景）