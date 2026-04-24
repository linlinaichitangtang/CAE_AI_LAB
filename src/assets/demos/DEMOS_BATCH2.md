# CAELab V1.0.2 Demo 案例库 — 第二批 (Batch 2)

> 多尺度仿真平台标准算例与演示案例集（进阶篇）| 新增 **42** 个案例 | 与第一批合计 **91** 个

---

## 与第一批的衔接路径

```
第一批 (Batch 1)                    第二批 (Batch 2)
─────────────────                   ─────────────────
SS-01 悬臂梁静力学 ──────────────→ SS-16 偏心拉伸 (组合变形)
SS-02 简支梁弯曲   ──────────────→ SS-17 组合变形梁 (弯+扭+剪)
SS-04 薄壁圆筒     ──────────────→ SS-18 厚壁圆筒 (Lamé公式)
SS-07 框架模态     ──────────────→ SS-19 旋转圆盘 (离心力场)
SS-09 弹塑性梁     ──────────────→ SS-24 CT试样 J积分
SS-12 薄板屈曲     ──────────────→ SS-21 翼梁后屈曲
SS-15 复合材料层合板 ────────────→ SS-26 复合材料开口板

TC-01 一维导热     ──────────────→ TC-13 半无限大体非稳态导热
TC-02 热膨胀梁     ──────────────→ TC-14 热冲击瞬态应力
TC-03 板坯冷却     ──────────────→ TC-21 凝固收缩
TC-06 焊接瞬态     ──────────────→ TC-18 激光焊接
TC-08 喷嘴热防护   ──────────────→ TC-22 热障涂层 TBC

MS-01 原子→连续体  ──────────────→ MS-11 Si弹性常数 DFT→FE
MS-05 Fe相变全链路 ──────────────→ MS-15 Ti-6Al-4V全链路
MS-08 陶瓷增韧     ──────────────→ MS-18 形状记忆合金

PP-01 位移云图     ──────────────→ PP-13 主应力轨迹线
PP-02 频响曲线     ──────────────→ PP-16 响应谱分析
PP-07 切片等值面   ──────────────→ PP-20 任意剖面切片
PP-09 大模型渲染   ──────────────→ PP-21 百万节点并行渲染
```

---

## 目录

- [一、结构仿真 Structural Batch 2 (+12)](#一结构仿真-structural-batch-2-12)
- [二、热力耦合 Thermal Coupling Batch 2 (+10)](#二热力耦合-thermal-coupling-batch-2-10)
- [三、多尺度串联 Multiscale Chain Batch 2 (+10)](#三多尺度串联-multiscale-chain-batch-2-10)
- [四、后处理展示 Postprocessing Batch 2 (+10)](#四后处理展示-postprocessing-batch-2-10)
- [两批合计统计](#两批合计统计)

---

## 一、结构仿真 Structural Batch 2 (+12)

路径：`src/assets/demos/structural/batch2-*.json`

### 进阶学习用 (4)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| SS-16 | `batch2-SS-16-eccentric-tension.json` | 偏心拉伸分析 | ⭐⭐ | σ=P/A±Pe·c/I | σ_max=39.2MPa |
| SS-17 | `batch2-SS-17-combined-deformation-beam.json` | 组合变形梁分析 | ⭐⭐⭐ | σ_eq=√(σ²+3τ²) | σ_eq=92.5MPa |
| SS-18 | `batch2-SS-18-thick-walled-cylinder.json` | 厚壁圆筒 Lamé 分析 | ⭐⭐⭐ | σ_θ=p_iR_i²/(R_o²-R_i²)(1+R_o²/r²) | σ_θ_max=83.3MPa |
| SS-19 | `batch2-SS-19-rotating-disk.json` | 旋转圆盘应力分析 | ⭐⭐⭐ | σ_θ=ρω²(3+ν)R²/8 | σ_θ_max=129.4MPa |

### 工业级演示用 (4)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| SS-20 | `batch2-SS-20-connecting-rod-dynamics.json` | 汽车连杆动力学 | ⭐⭐⭐⭐ | 惯性力+Goodman疲劳 | F_inertia=12.6kN, FOS=2.1 |
| SS-21 | `batch2-SS-21-wing-structure-buckling.json` | 航空翼梁结构屈曲 | ⭐⭐⭐⭐ | 加筋板压缩失稳 | Pcr=211.8kN/m |
| SS-22 | `batch2-SS-22-pressure-vessel-nozzle.json` | 压力容器接管 | ⭐⭐⭐⭐ | ASME VIII应力集中 | SCF=2.5, σ_max=625MPa |
| SS-23 | `batch2-SS-23-bolted-joint-group.json` | 螺栓连接组 | ⭐⭐⭐⭐ | 预紧力+载荷分配 | F_v=20kN/bolt |

### 高级测试用 (4)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| SS-24 | `batch2-SS-24-epfm-ct-specimen.json` | CT 试样 J 积分 | ⭐⭐⭐⭐⭐ | J 积分路径独立性 | J=17.7kJ/m², 5 paths |
| SS-25 | `batch2-SS-25-creep-analysis.json` | 高温蠕变分析 | ⭐⭐⭐⭐⭐ | Norton 幂律 ε̇=Aσⁿ | ε̇_ss=2.92e-9/s |
| SS-26 | `batch2-SS-26-composite-open-hole.json` | 复合材料开口板 | ⭐⭐⭐⭐⭐ | 应力集中+层间剥离 | Kt=2.8, OHT=450MPa |
| SS-27 | `batch2-SS-27-impact-dynamics.json` | 冲击动力学 | ⭐⭐⭐⭐⭐ | 应力波传播 | c=5048m/s |

---

## 二、热力耦合 Thermal Coupling Batch 2 (+10)

路径：`src/assets/demos/thermal/batch2-*.json`

### 进阶学习用 (4)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| TC-13 | `batch2-TC-13-transient-half-space.json` | 非稳态导热(半无限大体) | ⭐⭐ | T(x,t)=T_s+(T_i-T_s)·erf(x/2√αt) | T(0.01m,100s)=527.4K |
| TC-14 | `batch2-TC-14-thermal-shock.json` | 热冲击 | ⭐⭐⭐ | R=σ_f(1-ν)/(Eα) | R=71.4K, Bi=1.0 |
| TC-15 | `batch2-TC-15-radiation-heat-transfer.json` | 辐射换热 | ⭐⭐⭐ | q=σ(T₁⁴-T₂⁴)/(1/ε₁+1/ε₂-1) | q=3078W/m² |
| TC-16 | `batch2-TC-16-contact-thermal-resistance.json` | 接触热阻 | ⭐⭐ | R_contact=t/k | R=8.33e-4 K·m²/W |

### 工业级演示用 (3)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| TC-17 | `batch2-TC-17-turbine-blade-cooling.json` | 涡轮叶片冷却 | ⭐⭐⭐⭐ | 气膜冷却+内部通道 | η=0.5, T_s=1200K |
| TC-18 | `batch2-TC-18-laser-welding.json` | 激光焊接 | ⭐⭐⭐⭐ | Goldak 双椭球热源 | T_peak=3000K |
| TC-19 | `batch2-TC-19-chip-packaging.json` | 电子芯片封装 | ⭐⭐⭐⭐ | 多层热阻网络 | R_ja=0.8K/W, T_j=403K |

### 高级测试用 (3)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| TC-20 | `batch2-TC-20-thermal-topology-optimization.json` | 热结构拓扑优化 | ⭐⭐⭐⭐⭐ | SIMP 方法 | 体积分数 30% |
| TC-21 | `batch2-TC-21-solidification-shrinkage.json` | 凝固收缩 | ⭐⭐⭐⭐⭐ | 潜热+缩孔预测 | t_s=10.7s, L=389kJ/kg |
| TC-22 | `batch2-TC-22-thermal-barrier-coating.json` | 热障涂层 TBC | ⭐⭐⭐⭐⭐ | YSZ+NiCrAlY 界面 | T_s=1300K |

---

## 三、多尺度串联 Multiscale Chain Batch 2 (+10)

路径：`src/assets/demos/multiscale/batch2-*.json`

### 进阶学习用 (4)

| ID | 文件 | 名称 | 难度 | 尺度链路 | 关键指标 |
|----|------|------|------|----------|----------|
| MS-11 | `batch2-MS-11-si-elastic-constants.json` | Si 晶体弹性常数 | ⭐⭐ | DFT → FE | C₁₁=165.7, C₁₂=63.9, C₄₄=79.6 GPa |
| MS-12 | `batch2-MS-12-fe-lattice-bulk-modulus.json` | Fe 晶格常数+体模量 | ⭐⭐ | DFT → MD | B₀=170GPa, a=2.87Å |
| MS-13 | `batch2-MS-13-polymer-rouse-model.json` | 高分子链节 Rouse 模型 | ⭐⭐⭐ | DFT → MD → FE | τ_R=ζN²b²/(3π²kBT) |
| MS-14 | `batch2-MS-14-nanoindentation.json` | 纳米压痕 | ⭐⭐⭐ | MD → PF → FE | H≈0.5GPa (Oliver-Pharr) |

### 工业级演示用 (4)

| ID | 文件 | 名称 | 难度 | 尺度链路 | 关键指标 |
|----|------|------|------|----------|----------|
| MS-15 | `batch2-MS-15-ti64-full-chain.json` | Ti-6Al-4V 全链路 | ⭐⭐⭐⭐⭐ | DFT→MD→PF→FE | ε>300% 超塑性 |
| MS-16 | `batch2-MS-16-mg-we43-degradation.json` | 镁合金 WE43 降解 | ⭐⭐⭐⭐⭐ | DFT→MD→PF→FE | 腐蚀寿命预测 |
| MS-17 | `batch2-MS-17-concrete-chloride-ingress.json` | 混凝土氯离子侵蚀 | ⭐⭐⭐⭐ | DFT→MD→PF→FE | 锈蚀膨胀比≈3 |
| MS-18 | `batch2-MS-18-shape-memory-alloy.json` | 形状记忆合金 NiTi | ⭐⭐⭐⭐⭐ | DFT→MD→PF→FE | 拟弹性滞后环 |

### 高级测试用 (2)

| ID | 文件 | 名称 | 难度 | 尺度链路 | 关键指标 |
|----|------|------|------|----------|----------|
| MS-19 | `batch2-MS-19-uncertainty-propagation.json` | 跨尺度不确定性传播 | ⭐⭐⭐⭐ | DFT→MD→PF→FE | Sobol 灵敏度, MC=1000 |
| MS-20 | `batch2-MS-20-adaptive-mesh-coupling.json` | 自适应网格尺度耦合 | ⭐⭐⭐⭐ | DFT→MD→PF→FE | η<5% 误差指标 |

---

## 四、后处理展示 Postprocessing Batch 2 (+10)

路径：`src/assets/demos/postprocess/batch2-*.json`

### 进阶学习用 (4)

| ID | 文件 | 名称 | 难度 | 可视化类型 | 关键指标 |
|----|------|------|------|------------|----------|
| PP-13 | `batch2-PP-13-principal-stress-trajectories.json` | 主应力轨迹线 | ⭐⭐ | trajectory | Kirsch 解, σ=100MPa |
| PP-14 | `batch2-PP-14-strain-energy-density.json` | 应变能密度云图 | ⭐⭐ | contour | U=σ²/(2E) |
| PP-15 | `batch2-PP-15-safety-factor-contour.json` | 安全因子云图 | ⭐⭐ | contour | FOS=σ_y/σ_eqv |
| PP-16 | `batch2-PP-16-response-spectrum.json` | 响应谱分析 | ⭐⭐⭐ | spectrum | GB50011 设计谱 |

### 高级演示用 (4)

| ID | 文件 | 名称 | 难度 | 可视化类型 | 关键指标 |
|----|------|------|------|------------|----------|
| PP-17 | `batch2-PP-17-topology-optimization-animation.json` | 拓扑优化迭代动画 | ⭐⭐⭐⭐ | topology | SIMP, 50 迭代 |
| PP-18 | `batch2-PP-18-fsi-bridge-vibration.json` | 流固耦合 FSI | ⭐⭐⭐⭐ | fsi | 涡振 f≈0.2Hz |
| PP-19 | `batch2-PP-19-multiphysics-coupled.json` | 多物理场耦合结果 | ⭐⭐⭐⭐ | multiphysics | T+σ+u 同步展示 |
| PP-20 | `batch2-PP-20-arbitrary-section-slice.json` | 任意剖面切片 | ⭐⭐⭐⭐ | slice | 3 任意切面 |

### 高级测试用 (2)

| ID | 文件 | 名称 | 难度 | 可视化类型 | 关键指标 |
|----|------|------|------|------------|----------|
| PP-21 | `batch2-PP-21-mega-scale-rendering.json` | 百万节点并行渲染 | ⭐⭐⭐⭐⭐ | performance | 1M nodes, >30fps |
| PP-22 | `batch2-PP-22-vr-ar-export.json` | VR/AR 后处理导出 | ⭐⭐⭐⭐⭐ | vr_ar | USDZ, <50MB |

---

## 两批合计统计

### 按方向统计

| 方向 | Batch 1 | Batch 2 | 合计 |
|------|---------|---------|------|
| 📐 结构仿真 | 15 | 12 | **27** |
| 🌡️ 热力耦合 | 12 | 10 | **22** |
| 🧬 多尺度串联 | 10 | 10 | **20** |
| 📊 后处理展示 | 12 | 10 | **22** |
| **合计** | **49** | **42** | **91** |

### 按用途统计

| 用途 | Batch 1 | Batch 2 | 合计 |
|------|---------|---------|------|
| 学习用 | 18 | 16 | **34** |
| 演示用 | 17 | 15 | **32** |
| 测试用 | 14 | 11 | **25** |
| **合计** | **49** | **42** | **91** |

### 按难度统计

| 难度 | Batch 1 | Batch 2 | 合计 |
|------|---------|---------|------|
| ⭐ (1) | 8 | 0 | **8** |
| ⭐⭐ (2) | 10 | 8 | **18** |
| ⭐⭐⭐ (3) | 13 | 10 | **23** |
| ⭐⭐⭐⭐ (4) | 12 | 16 | **28** |
| ⭐⭐⭐⭐⭐ (5) | 6 | 8 | **14** |
| **合计** | **49** | **42** | **91** |

### 标准参考

| 案例 | 标准/规范 |
|------|-----------|
| SS-22 | ASME BPVC Section VIII Division 2 |
| SS-24 | ASTM E1820 (J-integral test) |
| SS-25 | ASTM E139 (creep test) |
| SS-26 | ASTM D5766 (OHT strength) |
| TC-17 | NASA CMC Turbine Blade Design |
| TC-21 | ASTM A36/A36M (casting) |
| PP-16 | GB 50011-2010 (seismic design) |

---

*CAELab V1.0.2 | 多尺度仿真平台 | 案例库版本 2.1 (Batch 1 + Batch 2)*
