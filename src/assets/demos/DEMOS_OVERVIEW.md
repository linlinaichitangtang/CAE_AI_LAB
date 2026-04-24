# CAELab V1.0.2 Demo 案例库总索引

> 全部五批案例 | **279** 个 Demo | **19** 个方向 | 三维索引

---

## 按方向索引

| # | 方向 | ID前缀 | 文件数 | 路径 | 批次 |
|---|------|--------|--------|------|------|
| 1 | 📐 结构仿真 | SS | 49 | `structural/` | B1-B4 |
| 2 | 🌡️ 热力耦合 | TC | 40 | `thermal/` | B1-B4 |
| 3 | 🧬 多尺度串联 | MS | 38 | `multiscale/` | B1-B4 |
| 4 | 📊 后处理展示 | PP | 40 | `postprocess/` | B1-B4 |
| 5 | 📋 专项验证 | VL | 14 | `validation/` | B3-B4 |
| 6 | 💼 工程实战 | EN | 8 | `engineering/` | B4 |
| 7 | 🔊 声学振动 | AC | 10 | `acoustic/` | B5 |
| 8 | ⚡ 疲劳Durability | FD | 8 | `fatigue/` | B5 |
| 9 | 🧬 拓扑优化 | TO | 8 | `topology_optimization/` | B5 |
| 10 | 📊 可靠性 | RL | 8 | `reliability/` | B5 |
| 11 | 🦴 生物力学 | BM | 8 | `biomechanics/` | B5 |
| 12 | ⚡ 电磁耦合 | EM | 6 | `electromagnetic/` | B5 |
| 13 | 🔌 电子封装 | EP | 6 | `electronics_pkg/` | B5 |
| 14 | 🖨️ 增材制造 | AM | 6 | `additive_manufacturing/` | B5 |
| 15 | 🌊 流固耦合 | FS | 6 | `fsi/` | B5 |
| 16 | 🛢️ 石油化工 | OS | 6 | `offshore/` | B5 |
| 17 | 🚄 轨道交通 | RW | 6 | `railway/` | B5 |
| 18 | 🎯 多学科优化 | MD | 6 | `mdo/` | B5 |
| 19 | 📖 工程合规 | CP | 6 | `compliance/` | B5 |

---

## 按难度索引

| 难度 | 案例数 | 占比 | 适用人群 |
|------|--------|------|----------|
| ⭐ (1) | 8 | 3% | 入门 |
| ⭐⭐ (2) | 24 | 9% | 初级 |
| ⭐⭐⭐ (3) | 53 | 19% | 中级 |
| ⭐⭐⭐⭐ (4) | 133 | 48% | 高级 |
| ⭐⭐⭐⭐⭐ (5) | 61 | 22% | 专家 |

---

## 按用途索引

| 用途 | 案例数 | 说明 |
|------|--------|------|
| learning | 74 | 学习用：理论验证、公式推导、概念理解 |
| demo | 113 | 演示用：工程场景、多物理场、工业应用 |
| test | 92 | 测试用：收敛性、精度、性能、规范校核 |

---

## 按批次索引

| 批次 | 新增 | 累计 | 特点 |
|------|------|------|------|
| Batch 1 | 49 | 49 | 基础理论验证 |
| Batch 2 | 42 | 91 | 进阶工程应用 |
| Batch 3 | 50 | 141 | 前沿研究+专项验证 |
| Batch 4 | 48 | 189 | 规范校核+工程实战 |
| Batch 5 | 90 | 279 | 多元新兴方向全覆盖 |

---

## 文档索引

| 文档 | 说明 |
|------|------|
| [DEMOS.md](DEMOS.md) | Batch 1 总文档 |
| [DEMOS_BATCH2.md](DEMOS_BATCH2.md) | Batch 2 总文档 |
| [DEMOS_BATCH3.md](DEMOS_BATCH3.md) | Batch 3 总文档 |
| [DEMOS_BATCH4.md](DEMOS_BATCH4.md) | Batch 4 总文档 |
| [DEMOS_BATCH5.md](DEMOS_BATCH5.md) | Batch 5 总文档 |
| [DEMOS_VALIDATION.md](DEMOS_VALIDATION.md) | 专项验证测试说明 |
| [DEMOS_ENGINEERING.md](DEMOS_ENGINEERING.md) | 工程实战案例说明 |

---

## 引用标准

| 标准 | 方向 | 案例 |
|------|------|------|
| ASME VIII / B31.3 / B30 | 结构/工程 | SS-40, SS-41, SS-43 |
| ASTM E1820 / E139 / D5766 | 结构/疲劳 | SS-24, FD-01, FD-06 |
| GB 50010 / 50011 / 50017 / 50009 | 土木/合规 | EN-04, EN-05, CP-03~06 |
| IIW / BS 7608 | 疲劳/焊接 | FD-05, RL-05 |
| ISO 6336 / IPC-2221B / IEC 60747-9 | 机械/电子 | EN-02, EN-07, EN-08 |
| API 650 / API 5L | 石油化工 | OS-01, OS-02 |
| EN 13749 / UIC 60 | 轨道交通 | RW-01~06 |

---

## 文件结构

```
src/assets/demos/
├── DEMOS.md                    ← Batch 1
├── DEMOS_BATCH2.md             ← Batch 2
├── DEMOS_BATCH3.md             ← Batch 3
├── DEMOS_BATCH4.md             ← Batch 4
├── DEMOS_BATCH5.md             ← Batch 5
├── DEMOS_VALIDATION.md         ← 专项验证说明
├── DEMOS_ENGINEERING.md        ← 工程实战说明
├── DEMOS_OVERVIEW.md           ← 本文件（总索引）
│
├── structural/          49 files  (SS-01 ~ SS-49)
├── thermal/             40 files  (TC-01 ~ TC-40)
├── multiscale/          38 files  (MS-01 ~ MS-38)
├── postprocess/         40 files  (PP-01 ~ PP-40)
├── validation/          14 files  (VL-01 ~ VL-14)
├── engineering/          8 files  (EN-01 ~ EN-08)
├── acoustic/            10 files  (AC-01 ~ AC-10)
├── fatigue/              8 files  (FD-01 ~ FD-08)
├── topology_optimization/ 8 files (TO-01 ~ TO-08)
├── reliability/          8 files  (RL-01 ~ RL-08)
├── biomechanics/         8 files  (BM-01 ~ BM-08)
├── electromagnetic/      6 files  (EM-01 ~ EM-06)
├── electronics_pkg/      6 files  (EP-01 ~ EP-06)
├── additive_manufacturing/ 6 files (AM-01 ~ AM-06)
├── fsi/                  6 files  (FS-01 ~ FS-06)
├── offshore/             6 files  (OS-01 ~ OS-06)
├── railway/              6 files  (RW-01 ~ RW-06)
├── mdo/                  6 files  (MD-01 ~ MD-06)
└── compliance/           6 files  (CP-01 ~ CP-06)
```

---

*CAELab V1.0.2 | 多尺度仿真平台 | 案例库版本 5.0 | 279 Demo Cases | 19 Directions*
