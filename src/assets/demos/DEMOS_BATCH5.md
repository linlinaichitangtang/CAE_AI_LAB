# CAELab V1.0.2 Demo 案例库 — 第五批 (Batch 5)

> 多尺度仿真平台 | 新增 **90** 个案例 | 13 个全新方向 | 五批合计 **279** 个

---

## 新增方向一览

| # | 方向 | ID前缀 | 文件数 | 路径 |
|---|------|--------|--------|------|
| 1 | 🔊 声学与振动噪声 | AC | 10 | `acoustic/` |
| 2 | ⚡ 疲劳与Durability | FD | 8 | `fatigue/` |
| 3 | 🧬 拓扑优化 | TO | 8 | `topology_optimization/` |
| 4 | 📊 可靠性与不确定性 | RL | 8 | `reliability/` |
| 5 | 🦴 生物力学 | BM | 8 | `biomechanics/` |
| 6 | ⚡ 电磁-结构耦合 | EM | 6 | `electromagnetic/` |
| 7 | 🔌 电子封装与功率模块 | EP | 6 | `electronics_pkg/` |
| 8 | 🖨️ 增材制造AM | AM | 6 | `additive_manufacturing/` |
| 9 | 🌊 流固耦合FSI | FS | 6 | `fsi/` |
| 10 | 🛢️ 石油化工/海洋工程 | OS | 6 | `offshore/` |
| 11 | 🚄 轨道交通 | RW | 6 | `railway/` |
| 12 | 🎯 多学科优化MDO | MD | 6 | `mdo/` |
| 13 | 📖 工程合规与规范校核 | CP | 6 | `compliance/` |

---

## 一、声学与振动噪声 🔊 (10)

| ID | 文件 | 名称 | 难度 | 用途 | 关键指标 |
|----|------|------|------|------|----------|
| AC-01 | `batch5-AC-01-tuning-fork-modal.json` | 音叉模态 | ⭐ | learning | f₁=440Hz (A4) |
| AC-02 | `batch5-AC-02-beam-sound-radiation.json` | 简支梁声辐射 | ⭐⭐ | learning | σ_rad~0.012 |
| AC-03 | `batch5-AC-03-helmholtz-resonator.json` | Helmholtz共振器 | ⭐⭐ | learning | f_H=171.5Hz |
| AC-04 | `batch5-AC-04-exhaust-muffler.json` | 消音器传递损失 | ⭐⭐⭐ | demo | TL_max=24.1dB |
| AC-05 | `batch5-AC-05-fan-blade-noise.json` | 风机噪声 | ⭐⭐⭐⭐ | demo | BPF=300Hz |
| AC-06 | `batch5-AC-06-vehicle-nvh.json` | 车内NVH | ⭐⭐⭐⭐ | demo | 避开25/50/75Hz |
| AC-07 | `batch5-AC-07-railway-floor-isolation.json` | 浮置地板隔振 | ⭐⭐⭐⭐ | demo | IL>20dB@50Hz |
| AC-08 | `batch5-AC-08-acoustic-structural-coupling.json` | 声-结构耦合 | ⭐⭐⭐⭐⭐ | test | 全耦合验证 |
| AC-09 | `batch5-AC-09-sea-analysis.json` | SEA统计能量分析 | ⭐⭐⭐⭐⭐ | test | 500-4000Hz |
| AC-10 | `batch5-AC-10-ultrasonic-ndt.json` | 超声无损检测 | ⭐⭐⭐⭐⭐ | test | TOF=1.695μs |

## 二、疲劳与Durability ⚡ (8)

| ID | 文件 | 名称 | 难度 | 用途 | 关键指标 |
|----|------|------|------|------|----------|
| FD-01 | `batch5-FD-01-sn-curve-basquin.json` | S-N曲线Basquin | ⭐⭐ | learning | N_f(300MPa)=2.5e5 |
| FD-02 | `batch5-FD-02-en-curve-coffin-manson.json` | ε-N曲线C-M | ⭐⭐ | learning | N_t=4700 |
| FD-03 | `batch5-FD-03-fatigue-notch-neuber.json` | 疲劳缺口Neuber | ⭐⭐⭐ | learning | Kf=2.2 |
| FD-04 | `batch5-FD-04-control-arm-fatigue.json` | 控制臂随机疲劳 | ⭐⭐⭐⭐ | demo | Miner D≤1 |
| FD-05 | `batch5-FD-05-welded-joint-fatigue.json` | 焊接接头IIW | ⭐⭐⭐⭐ | demo | FAT80 |
| FD-06 | `batch5-FD-06-fuselage-skin-crack-growth.json` | 蒙皮裂纹扩展 | ⭐⭐⭐⭐ | demo | Paris m=3.3 |
| FD-07 | `batch5-FD-07-variable-amplitude-spectrum.json` | TWIST变幅载荷 | ⭐⭐⭐⭐⭐ | test | 200 flights |
| FD-08 | `batch5-FD-08-thermo-mechanical-fatigue.json` | TMF热-机械疲劳 | ⭐⭐⭐⭐⭐ | test | N_f=320 |

## 三、拓扑优化 🧬 (8)

| ID | 文件 | 名称 | 难度 | 用途 |
|----|------|------|------|------|
| TO-01 | `batch5-TO-01-mbbc-compliance.json` | MBB梁最小柔度 | ⭐⭐⭐ | learning |
| TO-02 | `batch5-TO-02-stress-constraint.json` | 应力约束优化 | ⭐⭐⭐ | learning |
| TO-03 | `batch5-TO-03-frequency-maximization.json` | 频率最大化 | ⭐⭐⭐ | learning |
| TO-04 | `batch5-TO-04-wing-rib-lightweight.json` | 翼肋轻量化 | ⭐⭐⭐⭐ | demo |
| TO-05 | `batch5-TO-05-am-support-design.json` | AM支撑设计 | ⭐⭐⭐⭐ | demo |
| TO-06 | `batch5-TO-06-thermal-path-optimization.json` | 热传导路径优化 | ⭐⭐⭐⭐ | demo |
| TO-07 | `batch5-TO-07-multiscale-topology.json` | 多尺度拓扑优化 | ⭐⭐⭐⭐⭐ | test |
| TO-08 | `batch5-TO-08-dynamic-topology.json` | 动态拓扑优化 | ⭐⭐⭐⭐⭐ | test |

## 四、可靠性与不确定性 📊 (8)

| ID | 文件 | 名称 | 难度 | 用途 |
|----|------|------|------|------|
| RL-01 | `batch5-RL-01-monte-carlo-sampling.json` | 蒙特卡洛抽样 | ⭐⭐⭐ | learning |
| RL-02 | `batch5-RL-02-response-surface-rsm.json` | 响应面法RSM | ⭐⭐⭐ | learning |
| RL-03 | `batch5-RL-03-form-reliability.json` | FORM一次可靠度 | ⭐⭐⭐⭐ | learning |
| RL-04 | `batch5-RL-04-gear-bending-reliability.json` | 齿轮可靠度 | ⭐⭐⭐⭐ | demo |
| RL-05 | `batch5-RL-05-welded-joint-reliability.json` | 焊接接头可靠度 | ⭐⭐⭐⭐ | demo |
| RL-06 | `batch5-RL-06-seismic-reliability.json` | 抗震可靠度 | ⭐⭐⭐⭐⭐ | demo |
| RL-07 | `batch5-RL-07-multiphysics-uncertainty.json` | 多物理场不确定性 | ⭐⭐⭐⭐⭐ | test |
| RL-08 | `batch5-RL-08-data-driven-reliability.json` | 数据驱动可靠性 | ⭐⭐⭐⭐⭐ | test |

## 五、生物力学 🦴 (8)

| ID | 文件 | 名称 | 难度 | 用途 |
|----|------|------|------|------|
| BM-01 | `batch5-BM-01-internal-fixator-bending.json` | 内固定器弯曲 | ⭐⭐ | learning |
| BM-02 | `batch5-BM-02-femoral-neck-fracture.json` | 股骨颈骨折 | ⭐⭐⭐ | learning |
| BM-03 | `batch5-BM-03-artery-wall-stress.json` | 血管壁应力 | ⭐⭐⭐ | learning |
| BM-04 | `batch5-BM-04-hip-implant-wear.json` | 髋关节假体磨损 | ⭐⭐⭐⭐ | demo |
| BM-05 | `batch5-BM-05-spine-disc-herniation.json` | 椎间盘突出 | ⭐⭐⭐⭐ | demo |
| BM-06 | `batch5-BM-06-stent-expansion.json` | 血管支架扩张 | ⭐⭐⭐⭐ | demo |
| BM-07 | `batch5-BM-07-bone-remodeling-wolff.json` | Wolff定律重塑 | ⭐⭐⭐⭐⭐ | test |
| BM-08 | `batch5-BM-08-trabecular-microfe.json` | 松质骨微FE | ⭐⭐⭐⭐⭐ | test |

## 六~十三、其余方向 (48)

| 方向 | ID | 文件数 | 难度范围 |
|------|-----|--------|----------|
| ⚡ 电磁耦合 EM | EM-01~06 | 6 | ⭐⭐⭐~⭐⭐⭐⭐⭐ |
| 🔌 电子封装 EP | EP-01~06 | 6 | ⭐⭐~⭐⭐⭐⭐⭐ |
| 🖨️ 增材制造 AM | AM-01~06 | 6 | ⭐⭐~⭐⭐⭐⭐⭐ |
| 🌊 流固耦合 FS | FS-01~06 | 6 | ⭐⭐⭐~⭐⭐⭐⭐⭐ |
| 🛢️ 石油化工 OS | OS-01~06 | 6 | ⭐⭐⭐~⭐⭐⭐⭐⭐ |
| 🚄 轨道交通 RW | RW-01~06 | 6 | ⭐⭐⭐~⭐⭐⭐⭐⭐ |
| 🎯 多学科优化 MD | MD-01~06 | 6 | ⭐⭐⭐~⭐⭐⭐⭐⭐ |
| 📖 工程合规 CP | CP-01~06 | 6 | ⭐⭐~⭐⭐⭐⭐ |

---

## 五批合计统计

| 方向 | B1 | B2 | B3 | B4 | B5 | 合计 |
|------|-----|-----|-----|-----|-----|------|
| 📐 结构仿真 | 15 | 12 | 12 | 10 | — | **49** |
| 🌡️ 热力耦合 | 12 | 10 | 10 | 8 | — | **40** |
| 🧬 多尺度串联 | 10 | 10 | 10 | 8 | — | **38** |
| 📊 后处理展示 | 12 | 10 | 10 | 8 | — | **40** |
| 📋 专项验证 | — | — | 8 | 6 | — | **14** |
| 💼 工程实战 | — | — | — | 8 | — | **8** |
| 🔊 声学振动 | — | — | — | — | 10 | **10** |
| ⚡ 疲劳 | — | — | — | — | 8 | **8** |
| 🧬 拓扑优化 | — | — | — | — | 8 | **8** |
| 📊 可靠性 | — | — | — | — | 8 | **8** |
| 🦴 生物力学 | — | — | — | — | 8 | **8** |
| ⚡ 电磁耦合 | — | — | — | — | 6 | **6** |
| 🔌 电子封装 | — | — | — | — | 6 | **6** |
| 🖨️ 增材制造 | — | — | — | — | 6 | **6** |
| 🌊 流固耦合 | — | — | — | — | 6 | **6** |
| 🛢️ 石油化工 | — | — | — | — | 6 | **6** |
| 🚄 轨道交通 | — | — | — | — | 6 | **6** |
| 🎯 多学科优化 | — | — | — | — | 6 | **6** |
| 📖 工程合规 | — | — | — | — | 6 | **6** |
| **合计** | **49** | **42** | **50** | **48** | **90** | **279** |

---

*CAELab V1.0.2 | 多尺度仿真平台 | 案例库版本 5.0 (Final)*
