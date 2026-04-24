# CAELab V1.0.2 Demo 案例库 — 第三批 (Batch 3)

> 多尺度仿真平台标准算例与演示案例集（前沿篇）| 新增 **50** 个案例 | 三批合计 **141** 个

---

## 与前两批的衔接路径

```
Batch 1 (基础)          Batch 2 (进阶)           Batch 3 (前沿)
─────────────          ─────────────            ─────────────
SS-01 悬臂梁静力学 ──→ SS-16 偏心拉伸 ────────→ SS-28 FGM功能梯度梁
SS-09 弹塑性梁     ──→ SS-24 CT试样J积分 ────→ SS-30 Paris疲劳裂纹
SS-12 薄板屈曲     ──→ SS-21 翼梁后屈曲  ────→ SS-31 低速冲击损伤
SS-25 蠕变分析     ──→ SS-25 蠕变(Batch2) ───→ SS-36 22000h长期蠕变
                       SS-37 疲劳-蠕变耦合
SS-38 LEFM K因子(B2)→ SS-38 K因子+1/r奇异性
                       SS-39 SPH/EFG过渡

TC-01 一维导热     ──→ TC-13 半无限体  ───────→ TC-23 Phonon/Kapitza
TC-11 相变潜热     ──→ TC-21 凝固收缩  ───────→ TC-24 PCM储能单元
TC-18 激光焊接     ──→ TC-18 激光(Batch2) ───→ TC-25 粉末床熔化LPBF
TC-22 热障涂层     ──→ TC-22 TBC(Batch2) ────→ TC-26 核反应堆包壳
                                               TC-30 烧蚀防热
                                               TC-31 液氢深冷

MS-05 Fe相变全链路 ──→ MS-15 Ti64全链路 ─────→ MS-25 In718全链路
MS-12 Fe体模量     ──→ MS-12 Fe(Batch2) ─────→ MS-21 位错芯Peierls-Nabarro
MS-14 纳米压痕     ──→ MS-14 纳米压痕(B2) ──→ MS-23 裂纹尖端HRR场
                                               MS-24 Ostwald熟化
                                               MS-28 全固态电池

PP-01 位移云图     ──→ PP-13 主应力轨迹线 ───→ PP-23 应力三轴度
PP-15 安全因子     ──→ PP-15 FOS(Batch2) ───→ PP-24 累积损伤等值面
PP-17 拓扑优化     ──→ PP-17 拓扑(Batch2) ──→ PP-29 拓扑结果处理+CAD
PP-21 百万节点渲染 ──→ PP-21 渲染(Batch2) ───→ PP-31 云原生WebAssembly
```

---

## 目录

- [一、结构仿真 Structural Batch 3 (+12)](#一结构仿真-structural-batch-3-12)
- [二、热力耦合 Thermal Coupling Batch 3 (+10)](#二热力耦合-thermal-coupling-batch-3-10)
- [三、多尺度串联 Multiscale Chain Batch 3 (+10)](#三多尺度串联-multiscale-chain-batch-3-10)
- [四、后处理展示 Postprocessing Batch 3 (+10)](#四后处理展示-postprocessing-batch-3-10)
- [五、专项验证测试 Validation (新增 +8)](#五专项验证测试-validation-新增-8)
- [三批合计统计](#三批合计统计)

---

## 一、结构仿真 Structural Batch 3 (+12)

路径：`src/assets/demos/structural/batch3-*.json`

### 前沿研究用 (4)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| SS-28 | `batch3-SS-28-fgm-beam.json` | FGM 功能梯度梁 | ⭐⭐⭐⭐ | E(x)=E_m+(E_c-E_m)(x/h)^p | E_eff≈225GPa, δ≈0.047mm |
| SS-29 | `batch3-SS-29-piezo-beam.json` | 压电智能梁 | ⭐⭐⭐⭐ | V=d₃₁·σ·t/E | d₃₁=-171pC/N |
| SS-30 | `batch3-SS-30-fatigue-crack-growth.json` | Paris 疲劳裂纹扩展 | ⭐⭐⭐⭐ | da/dN=C(ΔK)^m | C=2e-12, m=3.0 |
| SS-31 | `batch3-SS-31-low-velocity-impact.json` | CFRP 低速冲击 | ⭐⭐⭐⭐ | 分层损伤阈值 | 15J 冲击 |

### 工程实战用 (4)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| SS-32 | `batch3-SS-32-bridge-seismic-retrofit.json` | 桥梁抗震加固 | ⭐⭐⭐⭐ | f_cc=f_co+3.5f_l | CFRP t=3mm |
| SS-33 | `batch3-SS-33-pipe-vibration.json` | 管道振动 | ⭐⭐⭐⭐ | 固有频率避开激励 | 避开 50Hz |
| SS-34 | `batch3-SS-34-wind-turbine-blade.json` | 风电叶片预弯 | ⭐⭐⭐⭐ | 离心力预应力 | L=40m, 12rpm |
| SS-35 | `batch3-SS-35-spherical-tank.json` | 球罐支柱 | ⭐⭐⭐⭐ | σ=pR/(2t), ASME VIII | σ=200MPa |

### 极限工况用 (4)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| SS-36 | `batch3-SS-36-long-term-creep.json` | Cr-Mo 钢 22000h 蠕变 | ⭐⭐⭐⭐⭐ | Larson-Miller 参数 | 813K, 100MPa |
| SS-37 | `batch3-SS-37-creep-fatigue-coupling.json` | 疲劳-蠕变耦合 | ⭐⭐⭐⭐⭐ | D_f+D_c≤1 | In718, 923K |
| SS-38 | `batch3-SS-38-lefm-stress-intensity.json` | K 因子断裂力学 | ⭐⭐⭐⭐⭐ | 1/√r 奇异性 | K_I=σ√(πa)·F(a/W) |
| SS-39 | `batch3-SS-39-sph-efg-transition.json` | SPH/EFG 过渡 | ⭐⭐⭐⭐⭐ | 连续体→离散体 | FEM vs SPH vs EFG |

---

## 二、热力耦合 Thermal Coupling Batch 3 (+10)

路径：`src/assets/demos/thermal/batch3-*.json`

### 前沿研究用 (4)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| TC-23 | `batch3-TC-23-phonon-interface-resistance.json` | 微纳尺度导热 | ⭐⭐⭐⭐ | Kapitza 热阻 | R_K=2e-8 K·m²/W |
| TC-24 | `batch3-TC-24-pcm-energy-storage.json` | 相变储能单元 | ⭐⭐⭐⭐ | Stefan 问题 | L=174kJ/kg |
| TC-25 | `batch3-TC-25-laser-powder-bed.json` | 激光快速成形 LPBF | ⭐⭐⭐⭐⭐ | 粉末床熔化 | T~3500K |
| TC-26 | `batch3-TC-26-reactor-cladding.json` | 核反应堆包壳 | ⭐⭐⭐⭐ | Zr-4 氧化层 | 10000h 寿期 |

### 工程实战用 (3)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| TC-27 | `batch3-TC-27-gas-turbine-life.json` | 燃气轮机寿命 | ⭐⭐⭐⭐⭐ | 蠕变-疲劳交互 | N_f≈5000 |
| TC-28 | `batch3-TC-28-engine-block-cooling.json` | 发动机缸体冷却 | ⭐⭐⭐⭐ | 流道+导热联合 | T_max<573K |
| TC-29 | `batch3-TC-29-building-curtain-wall.json` | 建筑节能幕墙 | ⭐⭐⭐ | U 值计算 | U<1.5 W/(m²·K) |

### 极端工况用 (3)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| TC-30 | `batch3-TC-30-ablation-heat-shield.json` | 烧蚀防热 | ⭐⭐⭐⭐⭐ | ṁ=q/(Cp·ΔT+h_ab) | v_abl≈0.15mm/s |
| TC-31 | `batch3-TC-31-lh2-cryogenic-tank.json` | 液氢储罐深冷 | ⭐⭐⭐⭐⭐ | -253°C 热应力 | Q_leak<1W |
| TC-32 | `batch3-TC-32-ultra-high-speed-friction.json` | 超高速摩擦 | ⭐⭐⭐⭐⭐ | q=6e7 W/m² | dT/dt≈6000K/s |

---

## 三、多尺度串联 Multiscale Chain Batch 3 (+10)

路径：`src/assets/demos/multiscale/batch3-*.json`

### 前沿研究用 (4)

| ID | 文件 | 名称 | 难度 | 尺度链路 | 关键指标 |
|----|------|------|------|----------|----------|
| MS-21 | `batch3-MS-21-dislocation-core-structure.json` | 位错芯 Peierls-Nabarro | ⭐⭐⭐⭐ | DFT→MD→FE | τ_P≈300MPa |
| MS-22 | `batch3-MS-22-grain-boundary-migration.json` | 晶界迁移 CSL | ⭐⭐⭐⭐ | DFT→MD→PF | Burke-Turner |
| MS-23 | `batch3-MS-23-crack-tip-plastic-zone.json` | 裂纹尖端 HRR 场 | ⭐⭐⭐⭐⭐ | DFT→MD→PF→FE | r_p=(K/σ_y)²/6π |
| MS-24 | `batch3-MS-24-nanostructure-coarsening.json` | Ostwald 熟化 | ⭐⭐⭐⭐ | DFT→MD→PF→FE | LSW: <r>³∝t |

### 工业级演示用 (4)

| ID | 文件 | 名称 | 难度 | 尺度链路 | 关键指标 |
|----|------|------|------|----------|----------|
| MS-25 | `batch3-MS-25-in718-full-chain.json` | Inconel 718 全链路 | ⭐⭐⭐⭐⭐ | DFT→MD→PF→FE | γ_ISF≈20mJ/m² |
| MS-26 | `batch3-MS-26-photoresist-lithography.json` | 高分子光刻胶 | ⭐⭐⭐⭐⭐ | DFT→MD→PF→FE | EUV 微图案 |
| MS-27 | `batch3-MS-27-ti64-bone-implant.json` | 骨植入物 Ti-6Al-4V | ⭐⭐⭐⭐ | DFT→MD→PF→FE | 应力遮挡优化 |
| MS-28 | `batch3-MS-28-solid-state-battery.json` | 全固态电池 | ⭐⭐⭐⭐⭐ | DFT→MD→PF→FE | σ≈1e-3 S/cm |

### 高级测试用 (2)

| ID | 文件 | 名称 | 难度 | 尺度链路 | 关键指标 |
|----|------|------|------|----------|----------|
| MS-29 | `batch3-MS-29-ml-force-field.json` | ML 力场耦合 | ⭐⭐⭐⭐ | DFT→ML-MD | 加速 >10x |
| MS-30 | `batch3-MS-30-heterogeneous-computing.json` | 异构计算耦合 | ⭐⭐⭐⭐ | GPU-MD+CPU-FE | 延迟 <1ms |

---

## 四、后处理展示 Postprocessing Batch 3 (+10)

路径：`src/assets/demos/postprocess/batch3-*.json`

### 前沿研究用 (4)

| ID | 文件 | 名称 | 难度 | 可视化类型 | 关键指标 |
|----|------|------|------|------------|----------|
| PP-23 | `batch3-PP-23-stress-triaxiality.json` | 应力三轴度分布 | ⭐⭐⭐⭐ | contour | η=σ_m/σ_eq |
| PP-24 | `batch3-PP-24-cumulative-damage-contour.json` | 累积损伤等值面 | ⭐⭐⭐⭐ | isosurface | D=Σ(n_i/N_i) |
| PP-25 | `batch3-PP-25-heat-flux-vectors.json` | 热流矢量图 | ⭐⭐⭐ | vector | q=-k∇T |
| PP-26 | `batch3-PP-26-inherent-strain-contour.json` | 固有应变等值线 | ⭐⭐⭐⭐ | contour | 焊接等效载荷 |

### 工程实战用 (4)

| ID | 文件 | 名称 | 难度 | 可视化类型 | 关键指标 |
|----|------|------|------|------------|----------|
| PP-27 | `batch3-PP-27-design-checklist.json` | 设计点检表 | ⭐⭐⭐⭐ | design_check | ASME 自动校核 |
| PP-28 | `batch3-PP-28-doe-visualization.json` | DOE 结果可视化 | ⭐⭐⭐⭐ | doe | 响应面+敏感性 |
| PP-29 | `batch3-PP-29-topo-opt-postprocess.json` | 拓扑优化后处理 | ⭐⭐⭐⭐ | comparison | 密度→CAD导出 |
| PP-30 | `batch3-PP-30-auto-report-generation.json` | 报告自动生成 | ⭐⭐⭐⭐ | report | Word/PDF 一键 |

### 高级测试用 (2)

| ID | 文件 | 名称 | 难度 | 可视化类型 | 关键指标 |
|----|------|------|------|------------|----------|
| PP-31 | `batch3-PP-31-cloud-native-postprocess.json` | 云原生后处理 | ⭐⭐⭐⭐⭐ | cloud_native | WebAssembly |
| PP-32 | `batch3-PP-32-collaborative-annotation.json` | 多人协同标注 | ⭐⭐⭐⭐⭐ | collaborative | 实时同步 |

---

## 五、专项验证测试 Validation (新增 +8)

路径：`src/assets/demos/validation/batch3-*.json`

> 详见 [DEMOS_VALIDATION.md](DEMOS_VALIDATION.md)

| ID | 文件 | 名称 | 难度 | 验证类型 | 对比组数 |
|----|------|------|------|----------|----------|
| VL-01 | `batch3-VL-01-boundary-combinations.json` | 边界条件组合验证 | ⭐⭐⭐⭐ | boundary_condition | 6 种 BC |
| VL-02 | `batch3-VL-02-symmetry-conditions.json` | 对称/反对称验证 | ⭐⭐⭐ | boundary_condition | 2 种对称 |
| VL-03 | `batch3-VL-03-periodic-boundary.json` | 周期边界条件 | ⭐⭐⭐⭐ | boundary_condition | 单胞 vs 阵列 |
| VL-04 | `batch3-VL-04-interface-modeling.json` | 界面建模对比 | ⭐⭐⭐⭐ | boundary_condition | 接触/绑定/连续 |
| VL-05 | `batch3-VL-05-material-model-comparison.json` | 材料模型对比 | ⭐⭐⭐⭐ | material_model | 4 种本构 |
| VL-06 | `batch3-VL-06-steady-vs-transient.json` | 稳态vs瞬态 | ⭐⭐⭐ | temporal | 4 个时刻 |
| VL-07 | `batch3-VL-07-mesh-convergence.json` | 网格收敛性 | ⭐⭐⭐ | mesh | 5 种密度 |
| VL-08 | `batch3-VL-08-element-type-comparison.json` | 单元类型对比 | ⭐⭐⭐⭐ | element_type | 4 种单元 |

---

## 三批合计统计

### 按方向统计

| 方向 | Batch 1 | Batch 2 | Batch 3 | 合计 |
|------|---------|---------|---------|------|
| 📐 结构仿真 | 15 | 12 | 12 | **39** |
| 🌡️ 热力耦合 | 12 | 10 | 10 | **32** |
| 🧬 多尺度串联 | 10 | 10 | 10 | **30** |
| 📊 后处理展示 | 12 | 10 | 10 | **32** |
| 📋 专项验证 | — | — | 8 | **8** |
| **合计** | **49** | **42** | **50** | **141** |

### 按用途统计

| 用途 | Batch 1 | Batch 2 | Batch 3 | 合计 |
|------|---------|---------|---------|------|
| 学习用 | 18 | 16 | 16 | **50** |
| 演示用 | 17 | 15 | 15 | **47** |
| 测试用 | 14 | 11 | 19 | **44** |
| **合计** | **49** | **42** | **50** | **141** |

### 按难度统计

| 难度 | Batch 1 | Batch 2 | Batch 3 | 合计 |
|------|---------|---------|---------|------|
| ⭐ (1) | 8 | 0 | 0 | **8** |
| ⭐⭐ (2) | 10 | 8 | 0 | **18** |
| ⭐⭐⭐ (3) | 13 | 10 | 4 | **27** |
| ⭐⭐⭐⭐ (4) | 12 | 16 | 30 | **58** |
| ⭐⭐⭐⭐⭐ (5) | 6 | 8 | 16 | **30** |
| **合计** | **49** | **42** | **50** | **141** |

---

*CAELab V1.0.2 | 多尺度仿真平台 | 案例库版本 3.0 (Batch 1 + Batch 2 + Batch 3)*
