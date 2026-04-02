# Changelog

所有重要变更均记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

---

## [V1.3-dev] - 2026-04-02

### 新增

- **V1.3-003 热-流耦合分析 (CHT)**：新增 `ThermalFluidView.vue` 视图，支持共轭传热分析、散热片参数优化、温度场/速度场可视化
- **V1.3-005 内聚力模型分析 (CZM)**：新增 `cohesive.ts` API + `CohesiveZoneView.vue` 视图，支持分层/脱粘/裂纹扩展分析、双线性内聚力模型、载荷-位移曲线
- **V1.3-006 断裂力学分析 (XFEM)**：新增 `xfem.ts` API + `XfemView.vue` 视图，支持裂纹扩展追踪、J积分/K因子计算、Williams 奇异场展开
- **V1.3-007 几何清理与修复**：新增 `geometryRepair.ts` API + `GeometryRepairView.vue` 视图，支持 STL/STEP/IGES 导入、问题检测与自动修复
- **V1.3-008 高级网格工具**：新增 `advancedMesh.ts` API + `AdvancedMeshView.vue` 视图，支持梁/壳/实体混合网格、7种梁截面、截面属性计算、网格质量评分
- **V1.3-009 局部加密与映射网格**：新增 `meshRefinement.ts` API + `MeshRefinementView.vue` 视图，支持多区域加密、边界层网格(y+控制)、网格密度映射
- **V1.3-010 声-结构耦合分析**：新增 `acoustic.ts` API + `AcousticCouplingView.vue` 视图，支持模态耦合/谐响应/瞬态声学、频率响应曲线、辐射噪声计算

### 变更

- **路由**：新增 7 条路由（`/thermal-fluid`、`/cohesive`、`/xfem`、`/geometry-repair`、`/advanced-mesh`、`/mesh-refinement`、`/acoustic`）
- **导航**：左侧导航栏新增 7 个模块入口（热流、内聚力、XFEM、几何修复、高级网格、网格加密、声学）
- **国际化**：中英文语言包新增 V1.3 模块翻译
- **移动端**：新增强分析模块加入移动端路由限制列表

---

## [0.5.0] - 2026-04-02

### 新增

- **标准算例验证**：新增简支矩形板均布载荷和悬臂梁模态分析标准算例，共 5 个标准算例
- **局部网格加密**：支持边/面/体选择加密，渐变过渡算法
- **网格质量检查**：长宽比/雅可比比/偏斜度/翘曲角指标，可视化质量分布
- **求解进度条**：实时进度 + ETA + 取消功能 + 计算日志查看
- **剖切面工具**：XY/XZ/YZ 方向剖切，滑块调整位置
- **变形动画**：多时间步连续动画回放，0.25x~2x 速度控制
- **自动保存**：30 秒定时自动保存 + 版本历史（最多 20 个版本）
- **非线性材料**：弹塑性（Von Mises/Tresca/Drucker-Prager）、粘弹性（Maxwell/Kelvin/Prony）、超弹性（Neo-Hookean/Mooney-Rivlin/Ogden/Yeoh）
- **STEP/IGES 导入**：通过 GMSH/FreeCAD 转换，支持 STEP/STP/IGES/IGS 格式
- **材料库扩充**：50 种预置材料（结构钢/不锈钢/铝合金/钛合金/铜合金/高温合金/聚合物/复合材料/混凝土/木材/橡胶等）
- **帮助文档**：6 篇内置帮助文章 + 4 步新手引导
- **PDF 报告**：仿真报告 PDF 导出（基于 jsPDF）
- **多语言**：中文/英文双语支持（vue-i18n）
- **版本历史面板**：版本列表/恢复/删除/自动保存开关
- **验证报告组件**：理论解 vs 数值解对比，PASS/FAIL 判定

### 优化

- **性能优化**：Vite 代码分割（5 个独立 chunk）+ gzip/brotli 压缩
- **接触分析**：ContactResults 对接真实求解结果
- **打包配置**：三平台安装包完善（NSIS/deb/AppImage）

### 修复

- 修复 Rust 编译错误（E0432/E0373/E0382/E0689）
- 修复 TypeScript 类型错误（可选链/类型断言/重复标识符）
- 修复 PDF 生成构建失败（html2pdf.js 替换为 jsPDF 原生 API）

---

## [0.4.0] - 2026-04-02

### 新增

- 非线性材料 INP 生成（弹塑性/粘弹性/超弹性）
- STEP/IGES 文件导入后端
- 材料库 50 种预置材料
- 帮助文档系统
- PDF 报告生成
- 多语言 i18n 框架
- 性能优化（代码分割 + 压缩）
- 打包配置完善

---

## [0.3.0] - 2026-04-02

### 新增

- 局部网格加密
- 网格质量检查
- 求解进度条 + ETA + 取消
- 计算日志面板
- 自动保存 + 版本历史
- 剖切面工具
- 变形动画
- 悬臂梁标准算例（3 个）

---

## [0.2.0] - 初始版本

### 新增

- 基础仿真框架
- 3D 建模
- 笔记编辑器
- 代码编辑器
- 项目管理
