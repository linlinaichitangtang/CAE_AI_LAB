# CAELab V1.0.2 工程实战案例说明

> 真实工程场景案例集 | 8 个案例 | `src/assets/demos/engineering/`

---

## 概述

工程实战案例（Engineering）是 CAELab 案例库中最贴近工业应用的案例方向，覆盖**机械通用、土木建筑、电子半导体**三大领域。每个案例均基于真实工程设计规范，包含完整的几何参数、材料属性、载荷条件和验收标准。

---

## 案例总览

### 机械通用 (3)

| ID | 案例 | 规范 | 核心公式 | 验收标准 |
|----|------|------|----------|----------|
| EN-01 | 阶梯轴疲劳寿命 | ISO 6336 / DIN 743 | Soderberg: σ_a/S_e + σ_m/S_ut ≤ 1 | N_f > 10⁶ cycles |
| EN-02 | 齿轮齿根弯曲 | ISO 6336 / Lewis | σ = F_t/(b·m·Y) | σ ≤ σ_FP |
| EN-03 | 弹簧系统组合 | GB/T 1239 | k = Gd⁴/(8D³n) | 刚度误差 <5% |

### 土木建筑 (3)

| ID | 案例 | 规范 | 核心公式 | 验收标准 |
|----|------|------|----------|----------|
| EN-04 | RC 梁开裂 | GB 50010 | w_max = α_cr·ψ·σ_s/Es·(1.9c+0.08d_eq/ρ_te) | w_max ≤ 0.2mm |
| EN-05 | 钢节点抗震 | GB 50017 | V_p ≤ 0.55·f_v·h_c·t_w | 强节点弱构件 |
| EN-06 | 挡土墙土压力 | GB 50007 | Ka = tan²(45°-φ/2) | Pa ≤ [Pa] |

### 电子半导体 (2)

| ID | 案例 | 规范 | 核心公式 | 验收标准 |
|----|------|------|----------|----------|
| EN-07 | PCB 热分析 | IPC-2221B | R_ja = R_jc + R_cb + R_ba | T_j ≤ 398K |
| EN-08 | IGBT 热分析 | IEC 60747-9 | R_th(j-c) = ΔT/P | ΔR_th/R_th < 10% |

---

## 详细说明

### EN-01 阶梯轴疲劳寿命

**场景**：电机传动轴，D=80mm→d=50mm 过渡，圆角 r=5mm
- 材质：45钢调质，σ_b=750MPa，σ_-1=350MPa
- 载荷：弯矩 M=500N·m（旋转弯曲），轴向力 F_a=50kN
- 应力集中：Kt=2.0（查 Peterson 图表）
- 疲劳极限：σ_-1K = σ_-1/Kt·ε·β = 350/2.0×0.85×0.9 = 133.9MPa
- 安全系数：n = σ_-1K / σ_a = 133.9/80 = 1.67

### EN-02 齿轮齿根弯曲疲劳

**场景**：直齿圆柱齿轮，m=5mm, z=30, b=30mm
- 材质：20CrMnTi 渗碳淬火，σ_Flim=500MPa
- 载荷：T=200N·m, F_t = 2T/(m·z) = 2667N
- Lewis 系数：Y=0.42（z=30 标准齿形）
- 齿根应力：σ_F = F_t/(b·m·Y) = 2667/(0.03×0.005×0.42) = 42.3MPa
- 安全系数：S_F = σ_Flim/σ_F = 500/42.3 = 11.8

### EN-03 弹簧系统组合

**场景**：螺旋弹簧 + 碟形弹簧组合减振
- 螺旋弹簧：d=5mm, D=30mm, n=8, G=80GPa → k₁=23.1kN/m
- 碟形弹簧：D=50mm, d=25mm, t=2mm, h₀=1.5mm → k₂=150kN/m
- 并联组合：k_parallel = k₁+k₂ = 173.1kN/m
- 串联组合：k_series = 1/(1/k₁+1/k₂) = 20.1kN/m

### EN-04 钢筋混凝土梁开裂

**场景**：简支梁 L=4m, b×h=200×400mm, 3φ16 HRB400
- 混凝土：C30, f_tk=2.01MPa, E_c=30GPa
- 钢筋：A_s=603mm², ρ_te=0.015
- 裂缝宽度：w_max = 1.9·ψ·σ_s/Es·(30+0.08d_eq/ρ_te)
- 验收：w_max ≤ 0.2mm（一类环境）/ 0.3mm（二类）

### EN-05 钢结构梁柱节点

**场景**：H型钢梁柱焊接节点，抗震设计
- 梁：H300×150×6.5×9, 柱：H350×350×12×19
- 节点域剪力：V_p = (M_bL+M_bR)/h_b - V_col
- 验收：V_p ≤ 0.55·f_v·h_c·t_w（强节点弱构件）
- 塑性铰：梁端形成，节点域保持弹性

### EN-06 挡土墙主动土压力

**场景**：悬臂式挡土墙 H=4m
- 回填：φ=30°, γ=18kN/m³, c=0
- Rankine 主动土压力系数：Ka = tan²(45°-30°/2) = 0.333
- 总主动土压力：Pa = γH²Ka/2 = 18×16×0.333/2 = 48kN/m
- 作用点：H/3 = 1.33m from base

### EN-07 PCB 板级热分析

**场景**：8层 FR4 PCB, BGA 封装 Q=5W
- PCB 尺寸：200mm×150mm×1.6mm
- 热阻网络：R_jc(chip→board) + R_cb(board→ambient)
- 铜层导热：k_Cu=385W/(m·K), FR4: k=0.3W/(m·K)
- 结温：T_j = T_a + Q×R_ja = 323 + 5×25 = 448K → 需散热片

### EN-08 IGBT 模块热分析

**场景**：SiC IGBT 功率模块，Q=200W
- 芯片：12mm×12mm SiC, T_jmax=448K(175°C)
- 基板：AlN 陶瓷 k=170W/(m·K), Cu 底板
- 循环可靠性：ΔR_th/R_th < 10% per 1000 cycles
- 热循环：-40°C↔150°C, ΔT=190K

---

## 使用方式

```typescript
import demoData from '@/assets/demos/engineering/batch4-EN-01-shaft-fatigue-life.json'

// 获取所有工程案例
const engineeringCases = [
  import('@/assets/demos/engineering/batch4-EN-01-shaft-fatigue-life.json'),
  import('@/assets/demos/engineering/batch4-EN-02-gear-tooth-bending.json'),
  // ...
]
```

---

*CAELab V1.0.2 | 工程实战案例 | 版本 1.0*
