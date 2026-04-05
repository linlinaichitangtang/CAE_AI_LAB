# CAELab V1.0.2 UI 补全任务提示词

## 项目信息
- **项目路径**: `/Users/gongzhaolin/.openclaw/workspace-chief/projects/cae`
- **技术栈**: Tauri 2 (Rust + Vue 3 + TypeScript)
- **UI 设计规范**: `REDESIGN.md`（项目根目录，参考印象笔记+GoodNotes+COMSOL风格）

## 任务目标
完成 Phase 2 / Phase 3 UI 遗留任务，共 7 项。

---

## 任务一：仿真模块标签页化补全（SimulationView.vue）

**文件**: `src/views/SimulationView.vue`

### 1.1 分析类型下拉菜单扩展
当前 `analysisType` 下拉只有：structural / modal / frequency / buckling / thermal / cfd

**需要新增**：
- `transient` → 瞬态分析（调用 `TransientDynamicsView.vue` 的核心逻辑）
- `fatigue` → 疲劳分析（集成 `FatigueView.vue`）
- `thermal_coupling` → 热-结构耦合（集成 `ThermalCouplingView.vue`）

### 1.2 右侧快捷标签页扩展
当前 `commonAnalysisTabs` 只有 5 个快捷按钮（网格/材料/边界/求解/后处理）。

**需要在 SimulationView.vue 内新增三个标签页内容区**：
```
<template v-if="analysisType === 'transient'">
  <!-- 参考 src/views/TransientDynamicsView.vue 的核心参数表单 -->
  <!-- 时间步设置、加载曲线、Newmark-beta 参数 -->
</template>

<template v-if="analysisType === 'fatigue'">
  <!-- 参考 src/views/FatigueView.vue，但内嵌到 SimulationView 右侧面板 -->
  <!-- 高周/低周选择、S-N曲线、应力比、寿命计算 -->
</template>

<template v-if="analysisType === 'thermal_coupling'">
  <!-- 参考 src/views/ThermalCouplingView.vue，但内嵌 -->
  <!-- 顺序耦合/完全耦合、温度场→结构场映射 -->
</template>
```

### 1.3 标签页切换逻辑
```typescript
const analysisType = ref<'structural' | 'modal' | 'frequency' | 'buckling' | 'thermal' | 'cfd' | 'transient' | 'fatigue' | 'thermal_coupling'>('structural')
```

---

## 任务二：后处理全屏 + 图表联动（SimulationView.vue）

**目标**：仿真结果出来后，能一键切换到全屏 3D 云图视图，底部图表浮动可调。

### 2.1 全屏模式
在 SimulationView.vue 的求解完成区域添加"全屏查看"按钮：
```html
<button v-if="projectStore.hasResult" @click="toggleFullscreen" class="tab-btn" title="全屏查看">
  ⛶ 全屏
</button>
```

### 2.2 图表联动
底部响应曲线区域需要与 3D 云图联动：
- 点击曲线上的某点 → 3D 视图中高亮对应位置的节点
- 拖动时间轴滑块 → 3D 云图同步更新到该时间步
- 提供"分离/联动"切换按钮

---

## 任务三：材料/边界条件面板重设计（SimulationView.vue）

**参考**: `REDESIGN.md` 第 4.2 节 COMSOL 风格材料面板

### 3.1 材料面板改造
当前材料输入分散在各 analysisType 分支内，**统一改造为折叠面板**：
```
▼ 材料属性
  材料库 [▼ 选择]  [从项目材料库选择]
  ──────────────────────────────
  名称：铝合金 6061
  弹性模量：68.9 [GPa ▼]  ← 单位下拉
  泊松比：0.33
  密度：2.70 [g/cm³ ▼]
  屈服强度：276 [MPa ▼]
  ──────────────────────────────
  [+ 高级属性] [+ 自定义属性]
```

### 3.2 边界条件面板改造
统一为 COMSOL 风格分类折叠：
```
▼ 边界条件
  ▼ 位移约束
    [固定支撑] [对称边界] [自定义]
  ▼ 荷载
    [分布力] [压力] [加速度] [离心载荷]
```

---

## 任务四：右侧参数面板规范化（SimulationView.vue）

**目标**: 统一右侧面板宽度和行为。

### 4.1 统一宽度
所有 `activeTab` 和 `analysisType` 的右侧面板统一为 `w-72`（288px），不能用 `w-80` 或其他宽度。

### 4.2 面板一致性
- 面板背景：`bg-white`
- 边框：`border-r border-gray-200`
- 内边距：`p-4 space-y-4`
- 折叠面板统一使用 `<details>/<summary>` 或现有 CollapsePanel 组件

---

## 任务五：动效和过渡优化

**目标**: 系统化动效，统一体验。

### 5.1 统一 transition 类
在 `src/style.css` 或 `tailwind.config.js` 中确保有：
```css
.transition-panel { transition: all 200ms ease-out; }
.transition-expand { transition: max-height 200ms ease-out; }
```

### 5.2 标签切换动画
`activeTab` 切换时加滑动下划线指示器（200ms）。

### 5.3 加载骨架屏
求解进行中显示骨架屏（shimmer 动画），不能用 spinner 代替。

---

## 技术约束

1. **不破坏现有功能**：所有现有 analysisType（structural/modal/frequency/buckling/thermal/cfd）必须保持正常工作
2. **Rust 命令不修改**：不做后端 Rust 命令开发，只改 Vue 前端
3. **编译通过**：`npm run build` 和 `cargo build` 零错误
4. **不退化**：不做会导致运行时 panic 的修改
5. **移动端兼容**：新增的标签页和面板需要响应式（考虑 iPad 场景）

---

## 交付标准

1. `analysisType` 包含全部 9 种分析类型
2. 瞬态/疲劳/热耦合三种分析的参数表单内嵌在 SimulationView.vue 右侧面板
3. 后处理全屏模式可切换
4. 材料面板符合 COMSOL 风格（有单位下拉）
5. 右侧面板宽度统一 w-72
6. `git commit` 提交，commit message 以 `feat(ui):` 开头
