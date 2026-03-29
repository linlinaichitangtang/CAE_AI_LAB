# CAELab - 科研与工程创作一体化工作台

## 项目简介

CAELab 是一款科研与工程创作一体化工作台，旨在打破传统工具割裂：
- **仿真即创作**：CAE仿真不再是孤立的分析工具，而是创作过程的一部分
- **创作即记录**：笔记、代码、模型、仿真结果形成有机整体
- **记录即知识**：双向链接和知识网络让内容沉淀为知识

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
| 总负责 | 麟工 | 全局统筹 |
| 架构师 | 麟构 | 技术选型、框架设计 |
| 前端 | 麟现 | Vue3 UI开发 |
| 后端 | 麟基 | Tauri/Rust后端 |
| 算法 | 麟算 | CAE求解器 |
| 运维 | 麟维 | CI/CD |
| 测试 | 麟测 | 自动化测试 |

## 开发里程碑

| 阶段 | 目标 | 交付物 |
|------|------|--------|
| Phase 1 | 核心框架 | 用户系统、笔记、建模、代码、仿真、联动 |
| Phase 2 | 能力增强 | CFD、热传导、材料库、协作功能 |
| Phase 3 | 平台扩展 | 多物理场、移动端、API |

## 许可证

MIT License