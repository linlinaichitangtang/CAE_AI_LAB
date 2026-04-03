# Changelog

所有重要变更均记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

---

## [V1.9-dev] - 2026-04-03

### 新增

- **V1.9-001 四尺度串联集成测试**：新增 `multiscaleIntegration.ts` API + `MultiscaleIntegrationView.vue` 视图，DFT→MD→相场→FE 全流程自动串联，步骤状态监控，自动桥接
- **V1.9-002 端到端标准算例**：3 个真实场景（高强钢析出相/镁合金蠕变/陶瓷增材制造），文献对比验证，误差分析
- **V1.9-003 工作流模板库**：新增 `workflowTemplate.ts` API + `WorkflowTemplateView.vue` 视图，10+ 模板（凝固/蠕变/疲劳/相变/断裂），参数化配置
- **V1.9-004 模板市场**：用户上传/下载/评分工作流模板，审核机制，版本管理，搜索筛选
- **V1.9-005 参数扫描自动化**：新增 `highThroughput.ts` API + `HighThroughputView.vue` 视图，DOE 矩阵生成（全因子/LHS/Sobol/随机/CCD），批量任务提交
- **V1.9-006 高通量筛选结果数据库**：自动入库，查询构建器，导出 CSV/Parquet/JSON，统计与灵敏度分析
- **V1.9-007 AI 参数推荐**：新增 `aiRecommend.ts` API + `AiRecommendView.vue` 视图，基于历史数据推荐各尺度最优参数，置信度评估，反馈循环
- **V1.9-008 四尺度集成 CI**：新增 `nightlyCI.ts` API + `NightlyCIView.vue` 视图，Nightly Build 自动跑完整算例，失败自动提 issue，Slack/邮件通知
- **V1.9-009 多尺度仿真最佳实践文档**：5 个典型场景 step-by-step 教程规划（文档类，后续补充）
- **V1.9-010 多尺度 UI 整合**：新增 `multiscaleWorkspace.ts` API + `MultiscaleWorkspaceView.vue` 视图，单一项目内管理 DFT/MD/相场/FE 四种任务，任务图可视化，共享数据

### 变更

- **路由**：新增 6 条路由（`/integration`、`/workflow-template`、`/high-throughput`、`/ai-recommend`、`/nightly-ci`、`/multiscale-workspace`）
- **导航**：左侧导航栏新增 6 个模块入口（集成测试、模板、高通量、AI推荐、Nightly、工作区）
- **国际化**：中英文语言包新增 V1.9 模块翻译
- **移动端**：新增模块加入移动端路由限制列表

---

## [V1.8-dev] - 2026-04-03

### 新增

- **V1.8-001 多尺度物理量本体库**：新增 `ontology.ts` API + `OntologyView.vue` 视图，50+ 物理量定义，SI/原子单位/cgs 自动转换，分类搜索，批量换算
- **V1.8-002 跨尺度坐标映射引擎**：新增 `coordinateMapping.ts` API + `CoordinateMappingView.vue` 视图，MD 原子盒 ↔ 相场网格 ↔ FE 网格自动对齐，晶向/原点/周期性边界映射
- **V1.8-003 粗粒化策略库**：新增 `coarseGraining.ts` API + `CoarseGrainingView.vue` 视图，QC/MQC/径向平均/傅里叶滤波/ML映射/Voronoi 6 种方法，自动推荐
- **V1.8-004 误差传播追踪框架**：新增 `errorTracking.ts` API + `ErrorTrackingView.vue` 视图，每步变换记录 Δpropagation，累积不确定度，蒙特卡洛采样，灵敏度分析
- **V1.8-005 多尺度标准算例库**：新增 `benchmark.ts` API + `BenchmarkView.vue` 视图，10+ 已验证算例（凝固/相变/断裂/蠕变/扩散/弹性/热学），含 DFT/MD/相场/FE 四级数据
- **V1.8-006 多尺度回归测试 CI**：新增 `regressionCI.ts` API + `RegressionCIView.vue` 视图，自动跑标准算例，误差超标阻断，Slack/邮件/Webhook 通知
- **V1.8-007 仿真结果审计日志**：新增 `auditLog.ts` API + `AuditLogView.vue` 视图，hash chain 不可篡改，可导出 ISO 9001/AS9100 审计报告
- **V1.8-008 跨尺度数据可视化映射**：新增 `crossScaleViz.ts` API + `CrossScaleVizView.vue` 视图，原子颜色映射到场变量，相场浓度云图叠加原子轨迹，尺度桥接动画

### 变更

- **路由**：新增 8 条路由（`/ontology`、`/coordinate-mapping`、`/coarse-graining`、`/error-tracking`、`/benchmark`、`/regression-ci`、`/audit-log`、`/cross-scale-viz`）
- **导航**：左侧导航栏新增 7 个模块入口（本体、坐标映射、粗粒化、误差追踪、算例库、审计、跨尺度可视化）
- **国际化**：中英文语言包新增 V1.8 模块翻译
- **移动端**：新增模块加入移动端路由限制列表

---

## [V1.7-dev] - 2026-04-03

### 新增

- **V1.7-001 VASP 输入生成器**：新增 `dftInput.ts` API + `DftInputView.vue` 视图，支持 POSCAR/INCAR/KPOINTS 可视化编辑，LDA+U 参数，6 种模板
- **V1.7-002 QE 输入生成器**：支持 PW/CP/PP 类型，Control/System/Electrons 块编辑，原子种类/坐标表，文件实时预览
- **V1.7-003 DFT 任务管理器**：新增 `dftTask.ts` API + `DftTaskView.vue` 视图，任务队列/状态监控，SLURM/PBS 脚本生成
- **V1.7-004 DFT 结果解析器**：新增 `dftPostProcess.ts` API，支持 VASP/QE/ABINIT 输出解析，能量/力/DOS/能带/电荷密度
- **V1.7-005 能量/力验证**：5 个标准算例 (Si/Al/Fe/Cu/Diamond)，误差 < meV/atom 验证
- **V1.7-006 DFT→MD 势函数训练**：新增 `dftBridge.ts` API + `DftBridgeView.vue` 视图，支持 NEP/MTP/EAM/MEAM，训练/验证/测试集划分，损失曲线
- **V1.7-007 DFT→相场 GL 参数**：Ginzburg-Landau 系数提取，T-x 二元相图计算，化学势对齐
- **V1.7-008 DFT 云端提交**：SLURM/PBS 脚本生成，队列配置，分区/账户/QoS 设置
- **V1.7-009 能带/DOS 可视化**：新增 `DftPostProcessView.vue` 视图，能带图(SVG)+费米能级标注，总DOS+PDOS叠加，电荷密度热力图

### 变更

- **路由**：新增 4 条路由（`/dft-input`、`/dft-task`、`/dft-postprocess`、`/dft-bridge`）
- **导航**：左侧导航栏新增 1 个模块入口（DFT）
- **国际化**：中英文语言包新增 V1.7 模块翻译
- **移动端**：新增模块加入移动端路由限制列表

---

## [V1.6-dev] - 2026-04-03

### 新增

- **V1.6-001 相场方程求解器**：新增 `phaseField.ts` API + `PhaseFieldView.vue` 视图，支持 Cahn-Hilliard/Allen-Cahn/相场晶体/Karma 模型，双势阱/多项式/Landau 自由能，显式/隐式/半隐式/RK4 时间积分
- **V1.6-002 组织初始化器**：随机/形核/层状/圆形/导入 5 种初始条件，形核位点管理
- **V1.6-003 相场-温度耦合**：单向/双向热耦合，潜热释放，热焓项
- **V1.6-004 相场-力学耦合**：新增 `phaseFieldPostProcess.ts` API，弹性能驱动组织演变，应力辅助析出，von Mises 场
- **V1.6-005 场可视化**：新增 `PhaseFieldAnalysisView.vue` 视图，5 种色谱，等高线，晶界叠加，Z 切片
- **V1.6-006 晶粒统计**：晶粒尺寸分布直方图，取向分布，Grain ID 可视化，尺寸 vs 时间曲线
- **V1.6-007 GPU 加速**：新增 `phaseFieldGpu.ts` API + `PhaseFieldGpuView.vue` 视图，CUDA/OpenCL/WebGPU/WASM SIMD 后端，基准测试，性能分析
- **V1.6-008 MD→相场桥接**：新增 `phaseFieldBridge.ts` API + `PhaseFieldBridgeView.vue` 视图，原子密度/键序参量/Voronoi 粗粒化
- **V1.6-009 相场→FE 均匀化**：Voigt/Reuss/Mori-Tanaka/自洽/数值方法，6×6 弹性张量，应力-应变曲线，屈服面 π 平面投影
- **V1.6-010 相场模板库**：晶粒长大/相分离/枝晶凝固/沉淀析出/裂纹扩展 5 种模板

### 变更

- **路由**：新增 4 条路由（`/phase-field`、`/phase-field-analysis`、`/phase-field-gpu`、`/phase-field-bridge`）
- **导航**：左侧导航栏新增 1 个模块入口（相场）
- **国际化**：中英文语言包新增 V1.6 模块翻译
- **移动端**：新增模块加入移动端路由限制列表

---

## [V1.5-dev] - 2026-04-02

### 新增

- **V1.5-001 MD 模块框架 + 任务管理**：新增 `molecularDynamics.ts` API + `MolecularDynamicsView.vue` 视图，MD 工作区与 FE/CFD 并列，支持 NVE/NVT/NPT/NPH/UM 系综
- **V1.5-002 LAMMPS 集成**：Python API 调用 LAMMPS，支持 EAM/LJ/MEAM/Tersoff/ReaxFF/Morse/Buckingham 势函数，Nosé-Hoover/Berendsen 恒温器，Parrinello-Rahman 恒压器
- **V1.5-003 原子结构建模器**：新增 `atomBuilder.ts` API + `AtomBuilderView.vue` 视图，支持 FCC/BCC/HCP/金刚石/SC 超胞、非晶、界面、缺陷生成
- **V1.5-004 特殊结构生成**：碳纳米管（手性向量公式）、团簇（二十面体/十面体/FCC/BCC/球形）、位错线建模
- **V1.5-005 MD 模拟类型**：NVE/NVT/NPT/NPH/UM 系综，控温控压方法，实时温度/压力/能量监控
- **V1.5-006 轨迹可视化 (AtomEye 风格)**：新增 `trajectoryViewer.ts` API + `TrajectoryView.vue` 视图，原子着色（元素/速度/应力/位移/电荷），bonds 显示，旋转/缩放/平移，帧播放控制
- **V1.5-007 RDF/MSD/扩散系数**：新增 `mdPostProcess.ts` API，径向分布函数、均方位移、扩散系数、速度自相关函数(VACF)、振动态密度(VDOS)
- **V1.5-008 应力/能量时序曲线**：温度/压力/总能量/动能/势能/体积时序，导出 CSV
- **V1.5-009 MD→相场数据接口**：QC/MQC/密度场/序参量粗粒化方法，原子构型→连续场变量
- **V1.5-010 MD→FE 边界条件映射**：Virial/Hardy/Irving-Kirkwood 应力平均，原子应力→连续体应力

### 变更

- **路由**：新增 4 条路由（`/md`、`/atom-builder`、`/trajectory`、`/md-postprocess`）
- **导航**：左侧导航栏新增 2 个模块入口（MD、原子建模）
- **国际化**：中英文语言包新增 V1.5 模块翻译
- **移动端**：新增模块加入移动端路由限制列表

---

## [V1.4-dev] - 2026-04-02

### 新增

- **V1.4-001 复合材料层合板分析**：新增 `composite.ts` API + `CompositeView.vue` 视图，支持经典层合板理论、Tsai-Hill/Tsai-Wu 失效准则、ABD矩阵计算、铺层顺序优化
- **V1.4-002 蜂窝/点阵结构生成器**：新增 `cellularStructure.ts` API + `CellularStructureView.vue` 视图，支持8种胞元类型、Gibson-Ashby等效性能、参数化生成
- **V1.4-003 蠕变分析**：新增 `creep.ts` API + `CreepView.vue` 视图，支持Norton/Bailey-Norton/时间硬化/应变硬化模型、Larson-Miller参数、剩余寿命评估
- **V1.4-004 高级材料模型**：新增 `advancedMaterial.ts` API + `AdvancedMaterialView.vue` 视图，支持Perzyna/Chaboche/Anand粘塑性、NiTi形状记忆合金超弹性滞回
- **V1.4-005 云端HPC提交**：新增 `cloudHpc.ts` API + `CloudHpcView.vue` 视图，支持任务提交/排队/进度推送/结果下载
- **V1.4-006 HPC集群管理**：新增 `hpcCluster.ts` API + `HpcClusterView.vue` 视图，支持节点监控/队列管理/告警/资源趋势
- **V1.4-007 RVE建模器**：新增 `rve.ts` API + `RveView.vue` 视图，支持随机颗粒/纤维/晶粒RVE生成、周期性边界条件
- **V1.4-008 均匀化方法FE²**：新增 `homogenization.ts` API + `HomogenizationView.vue` 视图，支持Voigt/Reuss/Mori-Tanaka/自洽/FE²方法
- **V1.4-009 多尺度桥接**：新增 `multiscale.ts` API + `MultiscaleView.vue` 视图，支持MD/相场→FE粗粒化、等效边界条件生成
- **V1.4-010 仿真数据资产管理**：新增 `dataAsset.ts` API + `DataAssetView.vue` 视图，支持版本化存储/标签检索/数据血缘追踪/溯源报告
- **V1.4-011 ISO/ASME认证报告**：新增 `certification.ts` API + `CertificationView.vue` 视图，支持ASME BPVC/ISO 2553等6种标准、应力校核/安全系数/合规检查

### 变更

- **路由**：新增 11 条路由（`/composite`、`/cellular`、`/creep`、`/advanced-material`、`/cloud-hpc`、`/hpc-cluster`、`/rve`、`/homogenization`、`/multiscale`、`/data-asset`、`/certification`）
- **导航**：左侧导航栏新增 7 个模块入口（复合、蠕变、云HPC、多尺度、数据、认证）
- **国际化**：中英文语言包新增 V1.4 模块翻译
- **移动端**：新增模块加入移动端路由限制列表

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
