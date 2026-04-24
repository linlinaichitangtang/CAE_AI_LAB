# CAELab V1.0.2 Demo 案例库

> 多尺度仿真平台标准算例与演示案例集 | 共 **49** 个案例 | 覆盖 4 大方向

---

## 目录

- [一、结构仿真 Structural (15)](#一结构仿真-structural-15)
- [二、热力耦合 Thermal Coupling (12)](#二热力耦合-thermal-coupling-12)
- [三、多尺度串联 Multiscale Chain (10)](#三多尺度串联-multiscale-chain-10)
- [四、后处理展示 Postprocessing (12)](#四后处理展示-postprocessing-12)
- [使用说明](#使用说明)
- [难度说明](#难度说明)

---

## 一、结构仿真 Structural (15)

路径：`src/assets/demos/structural/`

### 学习用 Learning (6)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| SS-01 | `SS-01-cantilever-static.json` | 悬臂梁静力学 | ⭐ | δ=FL³/3EI | δ=0.200mm, σ=6.0MPa |
| SS-02 | `SS-02-simply-supported-bending.json` | 简支梁弯曲 | ⭐ | δ=FL³/48EI | δ=0.0125mm, σ=1.5MPa |
| SS-03 | `SS-03-torsion-shaft.json` | 扭转杆 | ⭐ | τ=Tr/J, J=πR⁴/2 | τ_max=20.37MPa |
| SS-04 | `SS-04-thin-cylinder-compression.json` | 薄壁圆筒压杆 | ⭐ | 应力均匀化假设 | σ_axial=63.66MPa |
| SS-05 | `SS-05-plane-stress-plate.json` | 平面应力正方形板 | ⭐ | σ₁=σ, σ₂=0 | σ₁=100MPa |
| SS-06 | `SS-06-cantilever-thermal.json` | 悬臂梁温度载荷 | ⭐⭐ | σ=EαΔT | σ=240MPa, ΔT=100°C |

### 演示用 Demo (5)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| SS-07 | `SS-07-frame-modal.json` | 多楼层框架模态 | ⭐⭐⭐ | Euler-Bernoulli 梁理论 | f₁=9.56Hz, f₂=26.77Hz, f₃=38.69Hz |
| SS-08 | `SS-08-hole-stress-concentration.json` | 孔口应力集中 | ⭐⭐⭐ | Kirsch 解 σ_max=3σ | σ_max=300MPa, Kt=3.0 |
| SS-09 | `SS-09-elastoplastic-beam.json` | 非线性弹塑性梁 | ⭐⭐⭐⭐ | 塑性铰理论 | My=5208N·m, Mp=7813N·m |
| SS-10 | `SS-10-welding-residual-stress.json` | 焊接接头残余应力 | ⭐⭐⭐⭐ | 焊接热影响区 | σ_res=345MPa |
| SS-11 | `SS-11-sandwich-panel.json` | 夹层板弯曲 | ⭐⭐⭐ | 夹层板理论 | δ=0.338mm |

### 测试用 Test (4)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| SS-12 | `SS-12-plate-buckling.json` | 薄板屈曲 | ⭐⭐⭐⭐ | Pcr=π²D/a² | Pcr=11569N |
| SS-13 | `SS-13-block-contact.json` | 两个块体接触 | ⭐⭐⭐⭐⭐ | 接触力学 | p=100MPa, F=1.0MN |
| SS-14 | `SS-14-cyclic-symmetry.json` | 循环对称结构 | ⭐⭐⭐⭐ | 周期边界条件 | σ_hoop=10.83MPa |
| SS-15 | `SS-15-composite-laminate.json` | 复合材料层合板 | ⭐⭐⭐⭐⭐ | [ABD] 刚度矩阵 | A₁₁=59.87MN/m |

---

## 二、热力耦合 Thermal Coupling (12)

路径：`src/assets/demos/thermal/`

### 学习用 Learning (4)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| TC-01 | `TC-01-1d-heat-conduction.json` | 一维导热杆 | ⭐ | Fourier 定律 q=kAΔT/L | q=80W, 线性分布 |
| TC-02 | `TC-02-thermal-expansion-beam.json` | 热膨胀梁 | ⭐ | σ=EαΔT | σ=120MPa, δ=0.6mm |
| TC-03 | `TC-03-slab-cooling.json` | 板坯冷却 | ⭐⭐ | Bi 数与 Fo 数 | Bi=0.017, τ=1921s |
| TC-04 | `TC-04-forced-convection-plate.json` | 对流换热 | ⭐⭐ | Newton 冷却 q=hΔT | q=1825W/m² |

### 演示用 Demo (4)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| TC-05 | `TC-05-heatsink.json` | 电子散热器 | ⭐⭐⭐ | 热阻网络 | R_th=0.91K/W, T_max=345.5K |
| TC-06 | `TC-06-welding-transient.json` | 焊接过程瞬态热分析 | ⭐⭐⭐⭐ | 移动高斯热源 | T_peak=2500K |
| TC-07 | `TC-07-brake-disc-coupled.json` | 刹车盘热力耦合 | ⭐⭐⭐⭐ | 摩擦生热+结构变形 | σ=200MPa |
| TC-08 | `TC-08-rocket-nozzle-thermal.json` | 火箭发动机喷嘴热防护 | ⭐⭐⭐⭐⭐ | 烧蚀+主动冷却 | q=8.5MW/m² |

### 测试用 Test (4)

| ID | 文件 | 名称 | 难度 | 核心理论 | 关键指标 |
|----|------|------|------|----------|----------|
| TC-09 | `TC-09-convergence-comparison.json` | 热力耦合收敛性 | ⭐⭐⭐⭐ | NR vs 增量法 | δ=0.24mm |
| TC-10 | `TC-10-anisotropic-conductivity.json` | 各向异性导热 | ⭐⭐⭐⭐ | K 矩阵对角占优 | K=diag(50,5,1) |
| TC-11 | `TC-11-phase-change-latent-heat.json` | 相变潜热 | ⭐⭐⭐⭐⭐ | Enthalpy 方法 | s=14.6mm@1000s |
| TC-12 | `TC-12-thermal-fatigue.json` | 热疲劳 | ⭐⭐⭐⭐⭐ | Coffin-Manson | N_f=1000 cycles |

---

## 三、多尺度串联 Multiscale Chain (10)

路径：`src/assets/demos/multiscale/`

### 学习用 Learning (4)

| ID | 文件 | 名称 | 难度 | 尺度链路 | 关键指标 |
|----|------|------|------|----------|----------|
| MS-01 | `MS-01-atom-vibration-to-continuum.json` | 原子振动→连续体频率 | ⭐ | DFT → FE | Al Debye freq vs f₁ |
| MS-02 | `MS-02-molecular-force-constant.json` | 分子结构→力常数 | ⭐⭐ | DFT → MD → FE | H₂O k_bond=500N/m |
| MS-03 | `MS-03-md-snapshot-to-phasefield.json` | MD快照→相场初始场 | ⭐⭐ | MD → Phase Field | 4000 atoms → η(x,y,z) |
| MS-04 | `MS-04-fe-results-to-postprocess.json` | 有限元结果→云图后处理 | ⭐ | FE → FE | U=σ²/(2E) contour |

### 演示用 Demo (4)

| ID | 文件 | 名称 | 难度 | 尺度链路 | 关键指标 |
|----|------|------|------|----------|----------|
| MS-05 | `MS-05-fe-bcc-phase-transformation.json` | Fe BCC 相变全链路 | ⭐⭐⭐⭐⭐ | DFT→MD→PF→FE | HV~800 |
| MS-06 | `MS-06-si-crystal-machining.json` | Si 晶体加工全链路 | ⭐⭐⭐⭐⭐ | DFT→MD→PF→FE | 切削深度 5nm |
| MS-07 | `MS-07-al-alloy-creep.json` | Al 合金蠕变全链路 | ⭐⭐⭐⭐ | DFT→MD→PF→FE | ε̇=Aσⁿ |
| MS-08 | `MS-08-ceramic-toughening.json` | 陶瓷增韧全链路 | ⭐⭐⭐⭐⭐ | DFT→MD→PF→FE | KIC~7 MPa√m |

### 测试用 Test (2)

| ID | 文件 | 名称 | 难度 | 尺度链路 | 关键指标 |
|----|------|------|------|----------|----------|
| MS-09 | `MS-09-scale-interface-mapping.json` | 尺度接口数据映射 | ⭐⭐⭐ | DFT→MD→PF→FE | 映射误差 <5% |
| MS-10 | `MS-10-multimaterial-parameter-lib.json` | 多材料体系参数库 | ⭐⭐⭐⭐ | DFT→MD→PF→FE | 9 种材料完整参数 |

---

## 四、后处理展示 Postprocessing (12)

路径：`src/assets/demos/postprocess/`

### 学习用 Learning (4)

| ID | 文件 | 名称 | 难度 | 可视化类型 | 关键指标 |
|----|------|------|------|------------|----------|
| PP-01 | `PP-01-displacement-contour.json` | 悬臂梁位移云图 | ⭐ | contour | δ_max=2mm, jet colormap |
| PP-02 | `PP-02-sdof-frf.json` | 单自由度频响曲线 | ⭐⭐ | frf | f_n=5.03Hz, ζ=0.025 |
| PP-03 | `PP-03-principal-strain-direction.json` | 主应变方向 | ⭐⭐ | tensor | σ₁=100MPa, σ₂=0 |
| PP-04 | `PP-04-animation-basics.json` | 动画基础 | ⭐ | animation | 1st/2nd/3rd mode |

### 演示用 Demo (4)

| ID | 文件 | 名称 | 难度 | 可视化类型 | 关键指标 |
|----|------|------|------|------------|----------|
| PP-05 | `PP-05-modal-shapes-3d.json` | 模态振型动画 | ⭐⭐⭐ | animation | 前 6 阶, 3D 旋转 |
| PP-06 | `PP-06-frf-waterfall.json` | 频响函数瀑布图 | ⭐⭐⭐ | waterfall | 5 种阻尼比叠加 |
| PP-07 | `PP-07-slice-isosurface.json` | 切片云图+等值面 | ⭐⭐⭐⭐ | slice+isosurface | 3 正交切片 + σ_VM=200MPa |
| PP-08 | `PP-08-transient-response-animation.json` | 瞬态响应动画 | ⭐⭐⭐⭐ | animation | 500μs, 应力波传播 |

### 测试用 Test (4)

| ID | 文件 | 名称 | 难度 | 可视化类型 | 关键指标 |
|----|------|------|------|------------|----------|
| PP-09 | `PP-09-large-model-rendering.json` | 大模型渲染 | ⭐⭐⭐⭐⭐ | performance | 50 万节点, >30fps |
| PP-10 | `PP-10-animation-export.json` | 多帧导出动画 | ⭐⭐⭐⭐ | export | VTK→PNG→GIF, 360 帧 |
| PP-11 | `PP-11-stress-criterion-comparison.json` | 应力奇异评估 | ⭐⭐⭐⭐⭐ | comparison | VM/Tresca/Max Principal |
| PP-12 | `PP-12-result-comparison-tool.json` | 结果对比工具 | ⭐⭐⭐⭐ | comparison | 修改前后差异可视化 |

---

## 使用说明

### 加载案例

```typescript
import demoData from '@/assets/demos/structural/SS-01-cantilever-static.json'

// 或通过动态加载
async function loadDemo(category: string, id: string) {
  const module = await import(`@/assets/demos/${category}/${id}.json`)
  return module.default
}
```

### 案例文件结构

每个 JSON 文件包含以下核心字段：

| 字段 | 说明 |
|------|------|
| `id` | 唯一标识 (SS/TC/MS/PP-XX) |
| `name` / `nameEn` | 中英文名称 |
| `category` | 分类方向 |
| `usage` | 用途：learning / demo / test |
| `difficulty` | 难度等级 1-5 |
| `geometry` | 几何参数 |
| `mesh` | 网格参数 |
| `material` | 材料参数 |
| `boundaryConditions` | 边界条件 |
| `solver` | 求解器配置 |
| `theoretical` | 理论解与公式 |
| `acceptance` | 验收标准 |
| `expected_results` | 期望结果 |

### 理论参考

| 案例 | 参考文献 |
|------|----------|
| SS-01~02, SS-07 | Timoshenko, S.P. "Strength of Materials", Part I |
| SS-03 | Roark, R.J. "Formulas for Stress and Strain", 7th Ed. |
| SS-05 | Timoshenko, "Theory of Plates and Shells", Eq. 29 |
| SS-08 | Kirsch, G. "Die Theorie der Elastizität und die Bedürfnisse der Festigkeitslehre" |
| SS-12 | Timoshenko & Gere, "Theory of Elastic Stability" |
| SS-15 | Jones, R.M. "Mechanics of Composite Materials" |
| TC-01 | Incropera, F.P. "Fundamentals of Heat and Mass Transfer" |
| TC-11 | Stefan, J. "Über die Theorie der Eisbildung" |
| TC-12 | Coffin, L.F. "A Study of the Effects of Cyclic Thermal Stresses on a Ductile Metal" |
| MS-05 | Olson, G.B. "Computational Design of Hierarchically Structured Materials" |

---

## 难度说明

| 等级 | 标识 | 适用人群 | 说明 |
|------|------|----------|------|
| ⭐ | 1 | 入门 | 基础理论验证，单一物理场 |
| ⭐⭐ | 2 | 初级 | 含温度/瞬态等扩展概念 |
| ⭐⭐⭐ | 3 | 中级 | 多物理场耦合或复杂边界 |
| ⭐⭐⭐⭐ | 4 | 高级 | 非线性/多尺度/工程应用 |
| ⭐⭐⭐⭐⭐ | 5 | 专家 | 全链路多尺度/极限工况/性能测试 |

---

## 统计

| 方向 | 学习 | 演示 | 测试 | 合计 |
|------|------|------|------|------|
| 结构仿真 | 6 | 5 | 4 | **15** |
| 热力耦合 | 4 | 4 | 4 | **12** |
| 多尺度串联 | 4 | 4 | 2 | **10** |
| 后处理展示 | 4 | 4 | 4 | **12** |
| **合计** | **18** | **17** | **14** | **49** |

---

*CAELab V1.0.2 | 多尺度仿真平台 | 案例库版本 2.0*
