# CAELab - 科研与工程创作一体化工作台

## 项目简介

CAELab（CAE Lab）是科研与工程创作一体化工作台，核心壁垒是**多尺度贯通**——从第一性原理（DFT）到分子动力学（MD）到相场（Phase Field）到有限元（FE），一条链路的四个尺度全部自研打通。

- **多尺度仿真**：DFT → MD → 相场 → FE，一键串联，全自动数据传递
- **仿真即创作**：CAE仿真是创作过程的一部分，不是孤立工具
- **全尺度覆盖**：原子级（DFT/MD）→ 介观（相场）→ 连续体（FE），一个平台全搞定

## 技术栈

| 层级 | 技术 | 用途 |
|------|------|------|
| 跨平台框架 | Tauri 2.0 | 桌面应用框架 |
| 前端框架 | Vue 3 + TypeScript | UI开发 |
| 构建工具 | Vite | 开发服务器、构建 |
| 3D引擎 | Three.js | 3D建模与可视化 |
| 富文本编辑器 | TipTap | 笔记编辑 |
| 代码编辑器 | Monaco Editor | 代码编辑 |
| 数学渲染 | KaTeX | LaTeX公式 |
| 样式方案 | Tailwind CSS | UI样式 |
| 状态管理 | Pinia | 状态管理 |
| 数据库 | SQLite | 本地存储 |
| 后端服务 | Rust (Actix-web) | 本地API服务 |
| CAE求解器 | CalculiX | 结构力学求解 |

## 快速开始

### 环境要求
- Node.js 18+
- Rust 1.70+
- Git

### 安装步骤
```bash
# 1. 克隆仓库
git clone <repository-url>
cd CAELab

# 2. 安装前端依赖
npm install

# 3. 安装Rust依赖
cd src-tauri && cargo fetch && cd ..

# 4. 启动开发服务器
npm run dev
```

### 构建发布版本
```bash
# 构建macOS版本
npm run build:mac

# 构建Windows版本
npm run build:win

# 构建Linux版本
npm run build:linux
```

## 开发指南

### 项目结构
```
CAELab/
├── src/                  # Vue 3 前端源码
├── src-tauri/            # Tauri/Rust 后端源码
├── docs/                 # 项目文档
├── solver/               # CAE求解器
└── scripts/              # 构建脚本
```

### 代码规范
- TypeScript：严格模式，所有类型必须显式定义
- ESLint：代码风格检查
- Prettier：代码格式化
- Rust：遵循 rustfmt 规范

### Git提交规范
```
<type>(<scope>): <description>
类型: feat, fix, docs, style, refactor, test, chore
```

## 开发团队

| 角色 | 工程师 | 职责 |
|------|--------|------|
| 总负责人 | 麟昭（钳多多/龚小钳） | 全局统筹、任务拆解、跨部门协调 |
| 量化部门 | 麟御 | 量化策略、需求方 |
| 研发部门 | 麟造 | 产品设计、需求方 |
| 工程部门 | 麟工 | 代码实现（麟构/麟算/麟基/麟现/麟维/麟测） |

## 开发里程碑

| 版本 | 阶段 | 核心内容 | 状态 |
|------|------|---------|------|
| V0.1 ~ V0.9 | 基础能力 | 核心框架、建模、仿真、CFD | ✅ |
| V0.10 | 用户体系 | 注册/登录/会员/云同步 | ✅ |
| V1.0 | 协作分享 | 项目协作、数据共享 | 🚧 |
| V1.1 | 参数化优化 | 参数驱动、敏感性分析 | 📋 |
| V1.2 | 自动化与 API | 工作流自动化、外部API | 🚧 |
| V1.3 ~ V1.4 | 高级仿真 | 多物理场耦合、先进材料、云服务 | ✅ |
| V1.5 | Phase 3 — MD | 分子动力学、LAMMPS集成、轨迹可视化 | ✅ |
| V1.6 | Phase 3 — 相场 | CH/AC方程、热力耦合、GPU加速 | ✅ |
| V1.7 | Phase 3 — DFT | VASP/QE接口、势函数训练 | ✅ |
| V1.8 | Phase 3 — 数据框架 | 本体库、粗粒化、误差追踪、审计 | ✅ |
| V1.9 | Phase 3 — 集成 | 工作流编排、高通量、AI推荐、集成CI | ✅ |
| V2.0 | 平台护城河 | 多尺度工作流编排器 UI（前端完成） | ✅ |
| V2.0.1 | Phase 3 后端 | MD/相场/DFT Rust 命令实现（进行中） | 🚧 |

## 许可证

MIT License