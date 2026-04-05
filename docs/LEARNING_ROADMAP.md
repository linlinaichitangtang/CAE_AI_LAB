# CAELab 四尺度CAE复习路线图 v1.0

> 本文档为 CAELab 用户（龚肇麟）量身定制，融合理论学习与工程实践。每章节配有 CAELab App 内置示例入口，帮助边学边练。

---

## 宏观路线图

```
数学基础（2-3周）
    ↓
有限元 FEM（2-3周）← 当前 CAELab 最成熟模块
    ↓
分子动力学 MD（2-3周）
    ↓
相场方法 Phase Field（2-3周）
    ↓
第一性原理 DFT（2-3周）
```

---

## 阶段一：数学基础

> 所有CAE方法的数学根基。建议结合具体物理问题理解，而非孤立刷题。

### P0｜微积分

**核心内容：**
- 偏微分方程（PDE）——有限元/相场的语言
- 重积分、曲线积分、曲面积分
- Gauss散度定理、Stokes定理 → 弱形式推导必备

**CAELab 示例：**
- 示例位置：`/Users/gongzhaolin/.openclaw/workspace-chief/projects/cae/examples/calculus/`
- `gauss_divergence_example.rs` — 演示散度定理在连续体方程中的应用
- `pde_variational_derivation.ipynb` — 从微分形式到弱形式的推导笔记

### P0｜线性代数

**核心内容：**
- 矩阵运算、特征值/特征向量
- 稀疏矩阵存储格式（CSR/CSC）— 刚度矩阵的核心
- LU分解、共轭梯度法（CG）

**CAELab 示例：**
- 示例位置：`/Users/gongzhaolin/.openclaw/workspace-chief/projects/cae/src/solver/`
- `sparse_matrix.rs` — CAELab 稀疏矩阵实现（CSR格式）
- `cg_solver.rs` — 共轭梯度法求解器源码

### P1｜数值计算

**核心内容：**
- Gauss-Legendre积分 → 有限元单元数值积分
- 数值微分、有限差分
- 插值与形函数（单元刚度矩阵构建的基础）

**CAELab 示例：**
- 示例位置：`/Users/gongzhaolin/.openclaw/workspace-chief/projects/cae/src/elements/`
- `gauss_integration.rs` — 1D/2D Gauss积分实现
- `shape_function.rs` — Lagrange形函数构造

### P2｜变分法

**核心内容：**
- 泛函与变分原理
- Euler-Lagrange方程
- **Galerkin弱形式** — 连接微分方程与有限元的核心桥梁

**CAELab 示例：**
- 示例位置：`/Users/gongzhaolin/.openclaw/workspace-chief/projects/cae/examples/variational/`
- `galerkin_weak_form_derivation.md` — 以弹性杆为例的完整推导
- `poisson_weak_form.rs` — Poisson方程弱形式有限元实现

---

## 阶段二：有限元法（FEM）

> CAELab 当前最成熟的模块（CalculiX求解器已集成）。建议作为第一个深入复习的物理方法。

### 知识链

```
弹性力学基础 → 变分原理 → Galerkin法 → 单元类型 → 
刚度矩阵组装 → 边界条件 → 求解器 → 后处理
```

### P0｜弹性力学基础

**核心内容：**
- 位移场 u(x,y,z)、应变张量 ε、应力张量 σ
- 本构关系：σ = D · ε（胡克定律矩阵 D）
- 平衡方程、边界条件

**教材参考：** 《有限元方法》第3章（Zienkiewicz）

**CAELab 示例：**
- App 内示例：`仿真模块 → 静态力学 → 悬臂梁分析`
- 源码：`/Users/gongzhaolin/.openclaw/workspace-chief/projects/cae/src/structural/`

### P0｜变分原理与Galerkin法

**核心内容：**
- 势能泛函 Π = ∫σ:ε dΩ - ∫f·u dΩ
- 虚功原理 → 弱形式
- Galerkin加权残差法

**CAELab 示例：**
- 示例：`/Users/gongzhaolin/.openclaw/workspace-chief/projects/cae/examples/fem/beam_element.rs`
- 推导笔记：`galerkin_derivation_for_beam.md`

### P0｜单元类型

**核心内容：**
| 单元 | 自由度 | 应用场景 |
|------|--------|----------|
| 杆单元（Truss） | 轴向位移 | 桁架结构 |
| 梁单元（Beam） | 位移+转角 | 梁结构 |
| 三角形单元（TRI3, TRI6） | 位移 | 2D平面问题 |
| 四边形单元（QUAD4, QUAD8） | 位移 | 2D平面问题 |
| 六面体单元（HEX8, HEX20） | 位移 | 3D实体 |

**CAELab 示例：**
- App 内：`仿真模块 → 网格 → 单元类型选择面板`
- 源码：`/Users/gongzhaolin/.openclaw/workspace-chief/projects/cae/src/elements/`

### P1｜刚度矩阵组装

**核心内容：**
- 局部坐标系 → 整体坐标系变换
- 单元刚度矩阵 → 全局刚度矩阵（直接刚度法）
- 边界条件施加（惩罚法、对角元改1法）

**CAELab 示例：**
- 源码：`/Users/gongzhaolin/.openclaw/workspace-chief/projects/cae/src/assembly/`
- `global_stiffness.rs` — 全局刚度矩阵组装

### P1｜求解器

**核心内容：**
- 线性静力分析
- 模态分析（特征值问题）
- 屈曲分析

**CAELab 现状：**
- ✅ CalculiX 求解器已集成
- App 内入口：`仿真模块 → 求解设置 → 选择求解器`

### P2｜非线性

**核心内容：**
- 几何非线性（大变形）
- 材料非线性（弹塑性）
- Newton-Raphson迭代

**CAELab 现状：** 待完善

---

## 阶段三：分子动力学（MD）

> CAELab MD分析模块框架已有，求解器（LAMMPS）集成待完善。

### 知识链

```
Newton运动方程 → Verlet积分 → 势函数 → 
系综理论 → 周期边界 → 轨迹分析
```

### P0｜分子动力学原理

**核心内容：**
- Newton 第二定律：F = ma
- Verlet算法、Velocity-Verlet、Leap-frog
- 时间步长选择（fs量级）

### P0｜势函数

**核心内容：**
| 势函数 | 应用场景 |
|--------|----------|
| Lennard-Jones | 惰性气体、范德华力 |
| Morse | 键合相互作用 |
| EAM/MEAM | 金属 |
| REBO | C-H系统、共价键 |
| Coulomb +  PME | 带电系统、长程相互作用 |

**CAELab 示例：**
- 示例位置：`/Users/gongzhaolin/.openclaw/workspace-chief/projects/cae/examples/md/`
- `lammps_potential_examples/` — 各种势函数的输入文件示例

### P1｜系综

**核心内容：**
| 系综 | 物理条件 | 用途 |
|------|----------|------|
| NVE | 孤立系统，能量守恒 | 平衡态 |
| NVT | 温度恒定 | 热力学平衡 |
| NPT | 温度+压强恒定 | 相变研究 |

### P1｜边界条件

**核心内容：**
- 周期边界条件（Periodic Boundary）
- 最小镜像约定（Minimum Image Convention）
- 非周期性边界

### P2｜分析方法

**核心内容：**
- 径向分布函数 g(r) — 结构分析
- 均方根位移 MSD — 扩散系数
- 应力-应变曲线 — 力学性质

**CAELab 示例：**
- App 内：`MD模块 → 轨迹分析 → g(r)/MSD计算`
- 源码：`/Users/gongzhaolin/.openclaw/workspace-chief/projects/cae/src/md/`

---

## 阶段四：相场方法

> CAELab 相场求解器待开发。

### 知识链

```
Ginzburg-Landau理论 → 序参量 → 
Cahn-Hilliard方程 → Allen-Cahn方程 → 数值离散
```

### P0｜相场模型基础

**核心内容：**
- 序参量 φ（phase field）
- Ginzburg-Landau自由能泛函：F = ∫[f(φ) + (κ/2)|∇φ|²] dΩ
- 双稳态势：f(φ) = (φ²-φ⁴) 型

### P0｜Cahn-Hilliard方程

**核心内容：**
- 相分离、Spinodal分解
- 化学势 μ = δF/δφ
- 方程：∂φ/∂t = M∇²μ

**典型应用：** 金属合金的相分离模拟

### P0｜Allen-Cahn方程

**核心内容：**
- 晶粒长大（Grain Growth）
- 方程：∂φ/∂t = -M(δF/δφ)
- 界面追踪（无需显式界面）

**典型应用：** 金属再结晶、晶粒长大

### P1｜数值方法

**核心内容：**
- 能量极小化
- 有限差分/有限元离散
- 半隐式 Fourier谱方法

### P2｜耦合场

**核心内容：**
- 热力学一致性
- 弹性应变能耦合（相场+力学）

**CAELab 示例：**
- 示例位置：`/Users/gongzhaolin/.openclaw/workspace-chief/projects/cae/examples/phase_field/`
- `cahn_hilliard_2d.rs` — 2D Cahn-Hilliard方程简单实现

---

## 阶段五：第一性原理（DFT）

> CAELab DFT模块框架已有（Quantum ESPRESSO 对接）。

### 知识链

```
Schrödinger方程 → Hartree近似 → 
Hartree-Fock → 密度泛函理论 → Kohn-Sham方程 → 赝势
```

### P0｜量子力学基础

**核心内容：**
- Schrödinger方程：Hψ = Eψ
- Hamiltonian：H = T + V（动能 + 势能）
- 波函数 ψ 的物理意义

### P0｜Hartree-Fock近似

**核心内容：**
- 平均场近似
- 自洽场（SCF）迭代
- 交换相互作用

### P1｜密度泛函理论（DFT）

**核心内容：**
- Kohn-Sham 方程
- 交换-关联泛函：LDA → GGA → meta-GGA
- Hohenberg-Kohn 定理

### P1｜赝势方法

**核心内容：**
| 类型 | 特点 |
|------|------|
| Norm-conserving | 较早，准确但较硬 |
| PAW（Projector Augmented Wave） | VASP标配，更高效 |

### P2｜能带理论

**核心内容：**
- 布洛赫定理
- 倒格子空间
- Brillouin区
- 能带结构计算

**CAELab 示例：**
- App 内：`DFT模块 → 能带计算 → 输入结构`
- 源码：`/Users/gongzhaolin/.openclaw/workspace-chief/projects/cae/src/dft/`

---

## 推荐学习顺序

```
第1步：数学基础（微积分+线代）      → 约2周
第2步：有限元（结合CAELab现有模块）   → 约2-3周 ← 从这里开始！
第3步：分子动力学                   → 约2周
第4步：相场方法                     → 约2周
第5步：第一性原理                   → 约2周
```

---

## CAELab 内置学习入口

| 功能 | 位置 | 说明 |
|------|------|------|
| 有限元示例 | `仿真模块 → 示例工程` | 悬臂梁、孔板等标准算例 |
| MD轨迹示例 | `MD模块 → 示例轨迹` | LAMMPS输出.gro/.xyz |
| DFT输入示例 | `DFT模块 → 示例结构` | .xsf/.cif 结构文件 |
| 公式备忘录 | `帮助 → 数学参考` | 各模块核心公式速查 |

---

## 第一个知识点建议

**从 Galerkin 弱形式 开始**，原因：
1. 它是连接数学基础和有限元的核心桥梁
2. CAELab 所有固体力学求解器都基于此
3. 一旦理解弱形式，有限元其他部分都很直观

**下一步动作：** 朕可以为你生成一份 `galerkin_derivation_for_beam.md`，包含：
- 弹性杆的完整数学推导
- 每一步的物理意义
- 对应的 CAELab 源码位置

---

_文档版本：v1.0 | 2026-04-05 | 由麟昭编制_
