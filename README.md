# CAELab - 科研与工程创作一体化工作台

## 项目简介

CAELab（CAE Lab）是科研与工程创作一体化工作台，核心壁垒是**多尺度贯通**——从第一性原理（DFT）到分子动力学（MD）到相场（Phase Field）到有限元（FE），一条链路的四个尺度全部自研打通。

- **多尺度仿真**：DFT → MD → 相场 → FE，一键串联，全自动数据传递
- **仿真即创作**：CAE仿真是创作过程的一部分，不是孤立工具
- **全尺度覆盖**：原子级（DFT/MD）→ 介观（相场）→ 连续体（FE），一个平台全搞定
- **全平台支持**：桌面 (Windows/macOS/Linux)、移动 (iOS/Android)、纯血鸿蒙 (OpenHarmony)

## 技术栈

| 层级 | 技术 | 用途 |
|------|------|
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

| 平台 | 命令 | 输出 |
|------|------|------|
| macOS | `npm run tauri build -- --target dmg` | `.dmg` 安装包 |
| Windows | `npm run tauri build` | `.exe` (NSIS) |
| Linux | `npm run tauri build -- --target deb` | `.deb` 包 |
| iOS | `npm run tauri build -- --target ipa` | `.ipa` (需 macOS + Xcode) |
| Android | `npm run tauri build -- --target aab` | `.aab` (华为/Google Play) |
| 鸿蒙 NEXT | `npm run build:harmony` | `harmony/dist/` (WebView 资源) |

## 移动端适配

### 支持的平台

| 平台 | 手机 | 平板 | 折叠屏 | PC |
|------|------|------|--------|-----|
| iOS | ✅ | ✅ | - | - |
| Android | ✅ | ✅ | ✅ | - |
| 纯血鸿蒙 (OpenHarmony) | ✅ | ✅ | ✅ | ✅ |

### 平板功能完整支持
- 完整仿真 (CFD/拓扑优化/高级建模)
- M-Pencil / Apple Pencil 手写笔支持
- 多窗口/分屏操作
- AR/VR 预览

### 纯血鸿蒙适配
- OpenHarmony 版本检测与适配
- 折叠屏展开/折叠态自适应布局
- M-Pencil 压感支持
- 华为 AppGallery / 华为应用市场集成

## 开发指南

### 项目结构
```
CAELab/
├── src/                  # Vue 3 前端源码
│   ├── composables/      # Vue Composables (43个)
│   ├── views/            # 视图组件 (168个)
│   ├── components/       # 通用组件
│   └── utils/            # 工具函数
├── src-tauri/            # Tauri/Rust 后端源码
├── harmony/              # 纯血鸿蒙适配文档
├── docs/                 # 项目文档
└── ROADMAP.md            # 版本路线图
```

### 技术规范
- TypeScript：严格模式，所有类型必须显式定义
- ESLint：代码风格检查
- Prettier：代码格式化
- Rust：遵循 rustfmt 规范

### Git提交规范
```
<type>(<scope>): <description>
类型: feat, fix, docs, style, refactor, test, chore
```

## 已完成版本

| 版本 | 主题 | 核心功能 | 状态 |
|------|------|---------|------|
| V0.1 ~ V0.9 | 基础能力 | 核心框架、建模、仿真、CFD | ✅ |
| V1.0 ~ V1.4 | 高级仿真 | 多物理场耦合、先进材料、云服务、多尺度 | ✅ |
| V2.0 ~ V2.9 | 平台护城河 | 多尺度工作流编排器 UI（前端完成） | ✅ |
| V3.0 | 教学集成 | 班级管理、模板市场、LMS集成、容器化 | ✅ |
| V3.1 | 离线支持 | 离线模式、Benchmark、CAE导入 | ✅ |
| V3.2 | 数据中台 | 仿真数据库、论文导出、版本控制、Jupyter API | ✅ |
| V3.3 | 作业系统 | 作业仪表盘、操作回放、分阶段提交、教材模板 | ✅ |
| V3.4 | 行业落地 | 报告生成、优化工作流、Docker部署、AI助手 | ✅ |
| V3.5 | 工业标准 | ASME/NRC/IEC合规检查、AR/VR、API监控 | ✅ |

**当前版本**: V3.5 (2026-05-12)

## 开发团队

| 角色 | 工程师 | 职责 |
|------|--------|------|
| 总负责人 | 麟昭（钳多多/龚小钳） | 全局统筹、任务拆解、跨部门协调 |
| 量化部门 | 麟御 | 量化策略、需求方 |
| 研发部门 | 麟造 | 产品设计、需求方 |
| 工程部门 | 麟工 | 代码实现（麟构/麟算/麟基/麟现/麟维/麟测） |

## 许可证

MIT License