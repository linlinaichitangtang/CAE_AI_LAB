# CAELab 纯血鸿蒙 (OpenHarmony) 适配指南

## 概述

CAELab 针对纯血鸿蒙 (OpenHarmony) 进行深度适配，支持以下设备：

| 设备类型 | 支持状态 | 说明 |
|----------|----------|------|
| 手机 | ✅ | 华为 Mate/P系列/Nova 等 |
| 平板 | ✅ | 华为 MatePad 系列 |
| 折叠屏 | ✅ | Mate X系列 (展开/折叠态自适应) |
| 笔记本 | ✅ | MateBook 系列 |
| 桌面 | ✅ | 鸿蒙 PC 版 |

## 技术架构

```
┌─────────────────────────────────────────────────────────┐
│                    CAELab 前端 (Vue 3)                  │
│  ┌─────────────────────────────────────────────────┐   │
│  │           useHarmonyAdaptation.ts               │   │
│  │  - 设备类型检测 (手机/平板/折叠屏/PC)            │   │
│  │  - 折叠态/展开态 布局切换                        │   │
│  │  - M-Pencil 压感支持                             │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │           usePlatform.ts                        │   │
│  │  - OpenHarmony 版本检测                         │   │
│  │  - 纯血鸿蒙 vs 鸿蒙Android兼容层区分             │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │           harmonyPolyfill.ts                   │   │
│  │  - CSS 安全区域修复                             │   │
│  │  - PointerEvent 压感修复                        │   │
│  │  - WebWorker 兼容性                            │   │
│  │  - backdrop-filter 降级                        │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│              OpenHarmony WebView (ArkWeb)              │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│           OpenHarmony 应用框架 (ArkTS)                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │ 手机/平板    │  │  折叠屏      │  │  PC         │    │
│  │ 适配层       │  │  适配层      │  │  适配层      │    │
│  └─────────────┘  └─────────────┘  └─────────────┘    │
└─────────────────────────────────────────────────────────┘
```

## 前端适配Composables

### 1. usePlatform.ts - 平台检测

```typescript
const { 
  isHarmony,        // 鸿蒙NEXT (含Android兼容层)
  isOpenHarmony,    // 纯血鸿蒙 (openharmony/ohos)
  isPureHarmony,    // 纯血鸿蒙 (不含Android)
  deviceType,       // phone | tablet | foldable | laptop | desktop
  isHarmonyPencilSupported, // M-Pencil 支持
} = usePlatform()
```

### 2. useHarmonyAdaptation.ts - 鸿蒙特性适配

```typescript
const {
  isPureHarmony,      // 是否纯血鸿蒙
  displayInfo,        // 设备信息 (category, foldableState, screenSize)
  windowConfig,      // 窗口配置 (isLandscape, isSplitScreen)
  layoutConfig,      // 布局配置 (navigation, sidePanel, maxContentWidth)
  pencilConfig,      // 触控笔配置 (pressure, tilt, palmRejection)
  multiWindowConfig, // 多窗口配置
} = useHarmonyAdaptation()
```

### 3. useMobileFeatures.ts - 功能开关

```typescript
const { availableFeatures } = useMobileFeatures()
// 纯血鸿蒙平板特性:
availableFeatures.value.mPencil        // M-Pencil 支持
availableFeatures.value.multiWindow    // 多窗口
availableFeatures.value.foldableSupport // 折叠屏支持
```

## 折叠屏适配

### 布局切换规则

| 设备状态 | 屏幕宽度 | 导航方式 | 侧边栏 | 说明 |
|----------|----------|----------|--------|------|
| 手机 | < 768px | 底部导航 | 无 | 紧凑模式 |
| 折叠屏-折叠 | < 768px | 底部导航 | 无 | 手机形态 |
| 折叠屏-展开 | ≥ 1024px | 侧边导航 | 有 | 平板形态 |
| 平板横屏 | ≥ 1024px | 侧边导航 | 有 | 完整功能 |
| 平板竖屏 | 768-1024px | 底部导航 | 无 | 竖屏适配 |

### CSS 折叠屏适配

```css
/* 纯血鸿蒙折叠屏媒体查询 */
@media (screen-spanning: fold) {
  .fold-aware {
    padding: env(foldable-area-inset-top)
             env(foldable-area-inset-left)
             env(foldable-area-inset-bottom)
             env(foldable-area-inset-right);
  }
}
```

## M-Pencil 压感配置

```typescript
const pencilConfig = {
  enabled: true,
  pressureEnabled: true,    // 压感
  tiltEnabled: true,        // 倾斜
  palmRejection: true,      // 防误触
  pressureCurve: 'natural'  // 压感曲线
}
```

压感值范围: `0.0 - 1.0`，通过 `PointerEvent.pressure` 获取。

## 多窗口支持

纯血鸿蒙支持自由窗口 (多任务)，CAELab 响应式适配：

- 最小窗口尺寸: 手机 320×480, 平板 480×320
- 内容区域最大宽度自适应
- 布局模式切换: side ↔ bottom

## 构建输出

```bash
# 构建鸿蒙版
npm run build:harmony

# 输出目录: harmony/dist/
# 包含: index.html, assets/, resources/
```

## 已知限制

1. **Service Worker**: 纯血鸿蒙 WebView 不支持 `file://` 协议的 Service Worker
2. **WebGL**: 部分 OpenHarmony 版本 WebGL 性能受限
3. **摄像头/麦克风**: 需要在 ArkTS 侧申请权限

## 兼容性矩阵

| OpenHarmony 版本 | 支持状态 | 备注 |
|------------------|----------|------|
| 4.0 | ✅ | 主流版本 |
| 4.1 | ✅ | |
| 5.0 | ✅ | NEXT 系列 |
| 4.0 (麒麟) | ⚠️ | 可能存在 WebGL 问题 |