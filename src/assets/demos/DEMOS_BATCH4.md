# CAELab V1.0.2 Demo 案例库 — 第四批 (Batch 4)

> 多尺度仿真平台标准算例与演示案例集（收官篇）| 新增 **48** 个案例 | 四批合计 **189** 个

---

## 与前三批的衔接路径

```
Batch 1-3 (基础→进阶→前沿)           Batch 4 (规范/盲区/实战)
────────────────────────           ──────────────────────
SS-35 球罐 (ASME VIII) ──────────→ SS-40 开孔补强校核
SS-41 管道振动 ──────────────────→ SS-41 管道柔性 B31.3
SS-45 软硬复合 ──────────────────→ SS-45 超材料/构型力学
SS-47 In718蠕变(Batch3) ─────────→ SS-47 Inconel 617 1100°C
SS-48 疲劳裂纹(Batch3) ─────────→ SS-48 Rice-Tracey延性断裂

TC-28 发动机冷却 ────────────────→ TC-33 锅炉壁温计算
TC-29 建筑幕墙 ──────────────────→ TC-34 换热器场协同
TC-25 LPBF ─────────────────────→ TC-35 微波/TC-36 电阻加热
TC-37 凝固收缩 ─────────────────→ TC-37 Scheil溶质再分配
TC-31 液氢深冷 ─────────────────→ TC-40 LNG船舱绝热

MS-25 In718全链路 ──────────────→ MS-31 石墨烯/MS-32 剪切带
MS-29 ML力场 ───────────────────→ MS-38 数据驱动多尺度
                               → MS-37 分子马达ATP合酶

PP-28 DOE可视化 ────────────────→ PP-34 敏感度热图
PP-29 拓扑后处理 ──────────────→ PP-35 Pareto前沿
PP-31 云原生 ───────────────────→ PP-38 3D PDF报告
                               → PP-39 网格质量雷达图

VL-05 材料模型(Batch3) ─────────→ VL-09 超弹性三模型对比
VL-08 单元类型(Batch3) ─────────→ VL-10 屈服准则三模型对比
                               → VL-11 蠕变本构三模型对比
                               → VL-12 强化模型对比
                               → VL-13 求解器对比
                               → VL-14 积分方案对比

(新方向) ──────────────────────→ EN-01~08 工程实战案例
```

---

## 目录

- [一、结构仿真 Structural Batch 4 (+10)](#一结构仿真-structural-batch-4-10)
- [二、热力耦合 Thermal Coupling Batch 4 (+8)](#二热力耦合-thermal-coupling-batch-4-8)
- [三、多尺度串联 Multiscale Chain Batch 4 (+8)](#三多尺度串联-multiscale-chain-batch-4-8)
- [四、后处理展示 Postprocessing Batch 4 (+8)](#四后处理展示-postprocessing-batch-4-8)
- [五、专项验证测试 Validation Batch 4 (+6)](#五专项验证测试-validation-batch-4-6)
- [六、工程实战案例 Engineering (新增 +8)](#六工程实战案例-engineering-新增-8)
- [四批合计统计](#四批合计统计)

---

## 一、结构仿真 Structural Batch 4 (+10)

路径：`src/assets/demos/structural/batch4-*.json`

### 规范校核类 (4)

| ID | 文件 | 名称 | 难度 | 规范 | 关键指标 |
|----|------|------|------|------|----------|
| SS-40 | `batch4-SS-40-asme-viii-nozzle-reinforcement.json` | ASME VIII 开孔补强 | ⭐⭐⭐⭐ | ASME VIII Div.2 | σ_hoop=250MPa |
| SS-41 | `batch4-SS-41-pipe-flexibility-b313.json` | 管道柔性 B31.3 | ⭐⭐⭐⭐ | ASME B31.3 | S_E=240MPa |
| SS-42 | `batch4-SS-42-lifting-dynamics.json` | 吊装工况动力学 | ⭐⭐⭐⭐ | ASME B30 | F_sling=21.2kN |
| SS-43 | `batch4-SS-43-anchor-bolt-group.json` | 地脚螺栓组设计 | ⭐⭐⭐⭐ | ACI 318 | T_max=47.2kN |

### 前沿/特殊工况 (4)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| SS-44 | `batch4-SS-44-metamaterial-structure.json` | 超材料结构 | ⭐⭐⭐⭐⭐ | ν_eff=-0.5 | 均质化验证 |
| SS-45 | `batch4-SS-45-soft-hard-composite.json` | 软硬复合大变形 | ⭐⭐⭐⭐⭐ | Mooney-Rivlin | 200% 应变 |
| SS-46 | `batch4-SS-46-compliance-minimization.json` | MBB 构型力学 | ⭐⭐⭐⭐⭐ | SIMP 拓扑优化 | C_min=8.5N·m |
| SS-47 | `batch4-SS-47-inconel617-creep.json` | Inconel 617 蠕变 | ⭐⭐⭐⭐⭐ | Norton n=5.5 | 1373K, 50MPa |

### 极限破坏测试 (2)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| SS-48 | `batch4-SS-48-ductile-fracture-rice-tracey.json` | Rice-Tracey 延性断裂 | ⭐⭐⭐⭐⭐ | 空洞生长模型 | ε_f=0.85 |
| SS-49 | `batch4-SS-49-brittle-collapse-rock.json` | 脆性崩塌 | ⭐⭐⭐⭐⭐ | 最大拉应力准则 | σ_c=30MPa |

---

## 二、热力耦合 Thermal Coupling Batch 4 (+8)

路径：`src/assets/demos/thermal/batch4-*.json`

| ID | 文件 | 名称 | 难度 | 类别 | 关键指标 |
|----|------|------|------|------|----------|
| TC-33 | `batch4-TC-33-boiler-wall-temperature.json` | 锅炉壁温 | ⭐⭐⭐⭐ | 规范 | T_outer=673K |
| TC-34 | `batch4-TC-34-heat-exchanger-optimization.json` | 换热器场协同 | ⭐⭐⭐⭐ | 规范 | ε=0.75 |
| TC-35 | `batch4-TC-35-microwave-heating.json` | 微波加热 | ⭐⭐⭐⭐ | 前沿 | Q=5.13MW/m³ |
| TC-36 | `batch4-TC-36-joule-heating.json` | 电阻焦耳热 | ⭐⭐⭐⭐ | 前沿 | 各向异性电阻 |
| TC-37 | `batch4-TC-37-solidification-solute-redistribution.json` | Scheil 溶质再分配 | ⭐⭐⭐⭐⭐ | 前沿 | f_E=8.2% |
| TC-38 | `batch4-TC-38-supercritical-water-oxidation.json` | SCWO 超临界水氧化 | ⭐⭐⭐⭐⭐ | 前沿 | 873K/25MPa |
| TC-39 | `batch4-TC-39-spacecraft-thermal-control.json` | 航天器热控 | ⭐⭐⭐⭐⭐ | 极端 | ±50K 温摆 |
| TC-40 | `batch4-TC-40-lng-tank-insulation.json` | LNG 船舱绝热 | ⭐⭐⭐⭐⭐ | 极端 | BOG<0.1%/day |

---

## 三、多尺度串联 Multiscale Chain Batch 4 (+8)

路径：`src/assets/demos/multiscale/batch4-*.json`

| ID | 文件 | 名称 | 难度 | 尺度链路 | 关键指标 |
|----|------|------|------|----------|----------|
| MS-31 | `batch4-MS-31-graphene-composite.json` | 石墨烯增强复合材料 | ⭐⭐⭐⭐⭐ | DFT→MD→FE | γ_int=0.52J/m² |
| MS-32 | `batch4-MS-32-shear-band-evolution.json` | 剪切带演化 | ⭐⭐⭐⭐⭐ | DFT→MD→PF→FE | 绝热温升750K |
| MS-33 | `batch4-MS-33-electrochemical-deposition.json` | 电化学沉积 | ⭐⭐⭐⭐⭐ | DFT→MD→PF→FE | Butler-Volmer |
| MS-34 | `batch4-MS-34-bioinspired-interface.json` | 生物仿生骨矿化 | ⭐⭐⭐⭐⭐ | DFT→MD→PF→FE | Wolff 定律 |
| MS-35 | `batch4-MS-35-rough-surface-contact.json` | 粗糙表面接触 | ⭐⭐⭐⭐ | DFT→MD→PF→FE | D_f=2.3 |
| MS-36 | `batch4-MS-36-interface-crack-delamination.json` | 界面裂纹脱层 | ⭐⭐⭐⭐⭐ | DFT→MD→PF→FE | G_c=3.2J/m² |
| MS-37 | `batch4-MS-37-molecular-motor-atp.json` | 分子马达 ATP 合酶 | ⭐⭐⭐⭐⭐ | DFT→MD→PF→FE | ΔG=-48.5kJ/mol |
| MS-38 | `batch4-MS-38-data-driven-multiscale.json` | 数据驱动多尺度 | ⭐⭐⭐⭐ | DFT→ML-MD→FE | GAP MAE=0.068eV/Å |

---

## 四、后处理展示 Postprocessing Batch 4 (+8)

路径：`src/assets/demos/postprocess/batch4-*.json`

| ID | 文件 | 名称 | 难度 | 类别 |
|----|------|------|------|------|
| PP-33 | `batch4-PP-33-version-comparison.json` | 版本对比 | ⭐⭐⭐⭐ | 报告 |
| PP-34 | `batch4-PP-34-sensitivity-heatmap.json` | 敏感度热图 | ⭐⭐⭐⭐ | 报告 |
| PP-35 | `batch4-PP-35-design-space-exploration.json` | Pareto 前沿 | ⭐⭐⭐⭐ | 报告 |
| PP-36 | `batch4-PP-36-symmetry-half-model.json` | 对称半模型 | ⭐⭐⭐ | 特殊场景 |
| PP-37 | `batch4-PP-37-animation-playback-control.json` | 动画时程控制 | ⭐⭐⭐ | 特殊场景 |
| PP-38 | `batch4-PP-38-3d-pdf-report.json` | 3D PDF 报告 | ⭐⭐⭐⭐ | 特殊场景 |
| PP-39 | `batch4-PP-39-mesh-quality-radar.json` | 网格质量雷达图 | ⭐⭐⭐⭐ | 性能 |
| PP-40 | `batch4-PP-40-resource-estimation.json` | 计算资源预估 | ⭐⭐⭐ | 性能 |

---

## 五、专项验证测试 Validation Batch 4 (+6)

路径：`src/assets/demos/validation/batch4-*.json`

> 详见 [DEMOS_VALIDATION.md](DEMOS_VALIDATION.md)

| ID | 文件 | 名称 | 难度 | 验证类型 | 对比模型 |
|----|------|------|------|----------|----------|
| VL-09 | `batch4-VL-09-hyperelastic-comparison.json` | 超弹性模型对比 | ⭐⭐⭐⭐ | 本构 | Linear/MR/Ogden |
| VL-10 | `batch4-VL-10-yield-criteria-comparison.json` | 屈服准则对比 | ⭐⭐⭐⭐ | 本构 | VM/Tresca/DP |
| VL-11 | `batch4-VL-11-creep-law-comparison.json` | 蠕变本构对比 | ⭐⭐⭐⭐⭐ | 本构 | Norton/Garofalo/θ |
| VL-12 | `batch4-VL-12-hardening-comparison.json` | 强化模型对比 | ⭐⭐⭐⭐⭐ | 本构 | 各向同性/随动/混合 |
| VL-13 | `batch4-VL-13-direct-vs-iterative-solver.json` | 求解器对比 | ⭐⭐⭐⭐ | 算法 | Cholesky/PCG/AMG |
| VL-14 | `batch4-VL-14-reduced-vs-full-integration.json` | 积分方案对比 | ⭐⭐⭐⭐ | 算法 | C3D8R/C3D8/C3D8I |

---

## 六、工程实战案例 Engineering (新增 +8)

> 详见 [DEMOS_ENGINEERING.md](DEMOS_ENGINEERING.md)

路径：`src/assets/demos/engineering/batch4-*.json`

| ID | 文件 | 名称 | 领域 | 难度 | 规范 |
|----|------|------|------|------|------|
| EN-01 | `batch4-EN-01-shaft-fatigue-life.json` | 阶梯轴疲劳寿命 | 机械 | ⭐⭐⭐⭐ | ISO 6336 |
| EN-02 | `batch4-EN-02-gear-tooth-bending.json` | 齿轮齿根弯曲 | 机械 | ⭐⭐⭐⭐ | Lewis 公式 |
| EN-03 | `batch4-EN-03-spring-combination.json` | 弹簧系统组合 | 机械 | ⭐⭐⭐ | GB/T 1239 |
| EN-04 | `batch4-EN-04-rc-beam-cracking.json` | RC 梁开裂 | 土木 | ⭐⭐⭐⭐ | GB 50010 |
| EN-05 | `batch4-EN-05-steel-beam-column-connection.json` | 钢节点抗震 | 土木 | ⭐⭐⭐⭐ | GB 50017 |
| EN-06 | `batch4-EN-06-retaining-wall-earth-pressure.json` | 挡土墙土压力 | 土木 | ⭐⭐⭐ | Rankine/Coulomb |
| EN-07 | `batch4-EN-07-pcb-thermal-analysis.json` | PCB 热分析 | 电子 | ⭐⭐⭐⭐ | IPC-2221B |
| EN-08 | `batch4-EN-08-igbt-module-thermal.json` | IGBT 模块热分析 | 电子 | ⭐⭐⭐⭐ | IEC 60747-9 |

---

## 四批合计统计

### 按方向统计

| 方向 | B1 | B2 | B3 | B4 | 合计 |
|------|-----|-----|-----|-----|------|
| 📐 结构仿真 | 15 | 12 | 12 | 10 | **49** |
| 🌡️ 热力耦合 | 12 | 10 | 10 | 8 | **40** |
| 🧬 多尺度串联 | 10 | 10 | 10 | 8 | **38** |
| 📊 后处理展示 | 12 | 10 | 10 | 8 | **40** |
| 📋 专项验证 | — | — | 8 | 6 | **14** |
| 💼 工程实战 | — | — | — | 8 | **8** |
| **合计** | **49** | **42** | **50** | **48** | **189** |

### 按用途统计

| 用途 | B1 | B2 | B3 | B4 | 合计 |
|------|-----|-----|-----|-----|------|
| 学习用 | 18 | 16 | 16 | 2 | **52** |
| 演示用 | 17 | 15 | 15 | 22 | **69** |
| 测试用 | 14 | 11 | 19 | 24 | **68** |
| **合计** | **49** | **42** | **50** | **48** | **189** |

### 按难度统计

| 难度 | 合计 | 占比 |
|------|------|------|
| ⭐ (1) | 8 | 4% |
| ⭐⭐ (2) | 18 | 10% |
| ⭐⭐⭐ (3) | 30 | 16% |
| ⭐⭐⭐⭐ (4) | 90 | 48% |
| ⭐⭐⭐⭐⭐ (5) | 43 | 23% |
| **合计** | **189** | 100% |

---

*CAELab V1.0.2 | 多尺度仿真平台 | 案例库版本 4.0 (Final)*
