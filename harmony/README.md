# 鸿蒙 NEXT 壳应用开发指南

## 概述
CAELab 鸿蒙版通过 ArkTS 壳应用 + WebView 加载前端 H5 资源实现。

## 前置要求
- DevEco Studio 5.0+
- HarmonyOS NEXT 5.0+ SDK
- 华为开发者账号

## 开发步骤
1. 使用 DevEco Studio 创建 Empty Ability 项目
2. 在 entry/src/main/ets/pages/Index.ets 中配置 Web 组件
3. 将 Vite 构建产物（dist/）复制到 resources/rawfile/
4. 配置 Web 组件加载本地 HTML

## Web 组件配置示例
```typescript
import { webview } from '@kit.ArkWeb'

@Entry
@Component
struct Index {
  controller: webview.WebviewController = new webview.WebviewController()

  build() {
    Column() {
      Web({ src: $rawfile('index.html'), controller: this.controller })
        .width('100%')
        .height('100%')
        .javaScriptAccess(true)
        .domStorageAccess(true)
        .fileAccess(true)
        .cacheMode(webview.CacheMode.Default)
        .mixedMode(webview.MixedMode.All Compatible)
        .onConsole((event) => {
          console.log(`[CAELab Web] ${event.message}`)
          return false
        })
        .onErrorReceive((event) => {
          console.error(`[CAELab Web Error] ${event.error.getErrorInfo()}`)
        })
    }
  }
}
```

## 构建前端资源
```bash
cd caelab
npm run build:harmony
```

构建产物将输出到 `harmony/dist/` 目录，将该目录下的所有文件复制到鸿蒙项目的 `resources/rawfile/` 目录即可。

## 注意事项
- 鸿蒙 WebView 不支持 Service Worker 的 file:// 协议
- 鸿蒙 WebView 的安全区域通过 viewport-fit=cover + env() 实现
- M-Pencil 压感通过 PointerEvent.pressure 传递
- 文件访问需要配置 module.json5 的 requestPermissions
- 前端已内置鸿蒙兼容性 Polyfill，自动处理安全区域、压感、Worker 等差异
