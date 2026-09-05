# 技术栈

> 本章详细说明 OpenTTD Manager Plus 所使用的技术栈及其选型理由。

## 前端框架：Dioxus 0.7 + Native (Blitz)

[![Dioxus](https://img.shields.io/badge/Dioxus-0.7%20Native-blueviolet)](https://dioxuslabs.com)

Dioxus 是一个 Rust 编写的声明式 UI 框架，灵感来源于 React。选择 Dioxus 的原因：

| 特性 | 说明 |
|------|------|
| **跨平台** | 同一套代码可编译为桌面端、移动端、Web 端 |
| **原生自渲染** | 使用 Blitz — WGPU 驱动的 HTML/CSS 渲染器，**无需 WebView** |
| **高性能** | Rust 编译，无 GC，极小的运行时开销 |
| **声明式** | 类 React 的组件化开发模式 |
| **原生能力** | 直接访问文件系统、进程等系统 API |
| **热补丁** | 支持 Subsecond 运行时热补丁，修改 Rust 代码无需重启 |
| **活跃社区** | 快速迭代，v0.7.10 为最新稳定版（2026-07-30） |

### 平台渲染架构

| 平台 | 渲染器 | 技术栈 |
|------|--------|--------|
| Windows | **Blitz Native** (WGPU + Vello) | DirectX 12 / Vulkan 后端 |
| Linux | **Blitz Native** (WGPU + Vello) | Vulkan / OpenGL 后端 |
| macOS | **Blitz Native** (WGPU + Vello) | Metal 后端 |
| Android | **Blitz Native** (WGPU + Vello) | Vulkan 后端 |
| iOS | **Blitz Native** (WGPU + Vello) | Metal 后端 |
| Web | Dioxus-Web (WASM/Web-Sys) | 编译到 WASM 操作真实 DOM |

> **Blitz 原生渲染器**：Dioxus 0.7 引入的 WGPU 驱动的 HTML/CSS 渲染器，基于 Vello 计算着色器管线。无需系统 WebView，在所有平台上使用 GPU 加速渲染，为 Rust 跨平台 UI 提供了真正意义上的原生自渲染方案。

### 渲染管线

```
┌─────────────────────────────────────────────────────┐
│                  Dioxus VirtualDom                    │
│  (rsx! macro → VNode tree → diff → patch)           │
└───────────────────────┬─────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────┐
│                  dioxus-native-dom                    │
│  (DioxusDocument: bridges VirtualDom → Blitz DOM)   │
└───────────────────────┬─────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────┐
│                    Blitz DOM                          │
│  (HTML/CSS 解析、布局计算、样式级联)                 │
│  ┌─────────┐ ┌──────────┐ ┌───────────┐             │
│  │ Stylo   │ │ Taffy    │ │ Parley    │             │
│  │ (CSS)   │ │ (布局)   │ │ (文本)    │             │
│  └─────────┘ └──────────┘ └───────────┘             │
└───────────────────────┬─────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────┐
│                  Blitz Paint                          │
│  (Vello compute shader → WGPU → GPU rasterization)  │
└───────────────────────┬─────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────┐
│              Platform Windowing (winit)              │
│  Win32 / X11 / Wayland / Cocoa / Android / UIKit    │
└─────────────────────────────────────────────────────┘
```

## 后端语言：Rust

[![Rust](https://img.shields.io/badge/Rust-1.80+-orange)](https://rust-lang.org)

| 优势 | 说明 |
|------|------|
| **性能** | 零成本抽象，适合网络请求、文件解压等密集型操作 |
| **安全** | 所有权系统保证内存安全，无空指针、数据竞争 |
| **跨平台** | 一等公民的跨平台编译支持 |
| **生态** | Cargo 包管理器，丰富的库生态 |

## 核心依赖

### Dioxus 生态

| 库 | 版本 | 用途 |
|---|------|------|
| `dioxus-native` | 0.7.10 | 原生 Blitz/WGPU 渲染器 |
| `dioxus-core` | 0.7 | 虚拟 DOM 核心 |
| `dioxus-hooks` | 0.7 | 生命周期、状态管理钩子 |
| `dioxus-html` | 0.7 | HTML 元素和事件类型 |
| `dioxus-signals` | 0.7 | 响应式信号状态管理 |
| `dioxus-router` | 0.7 | 声明式路由 |
| `dioxus-logger` | 0.7 | 前端日志 |
| `manganis` | 0.7 | 资源资产管理 |

### 异步运行时

| 库 | 用途 |
|---|------|
| `tokio` | 异步运行时，处理网络请求、文件 I/O |
| `reqwest` | HTTP 客户端，用于下载和 API 请求 |

### 数据处理

| 库 | 用途 |
|---|------|
| `serde` / `serde_json` | 序列化/反序列化 JSON 配置和元数据 |
| `semver` | 语义化版本解析和比较 |
| `chrono` | 日期时间处理 |

### 文件处理

| 库 | 用途 |
|---|------|
| `zip` | 处理 ZIP 格式的 OpenTTD 压缩包 |
| `tar` / `flate2` | 处理 tar.gz 格式的 Linux 发布包 |
| `sha2` | SHA-256 校验和验证 |

### 下载引擎

| 库 | 用途 |
|---|------|
| `reqwest` (stream) | 流式下载，支持断点续传 |
| `indicatif` | 进度条显示 |
| `sha2` | 下载完整性验证 |

### 开发工具

| 工具 | 用途 |
|------|------|
| `dx` CLI | Dioxus 官方 CLI：构建、热重载、打包 |
| `cargo` | 构建系统、依赖管理 |
| `vitepress` | 文档系统 |

## 为什么不是其他方案？

### 为什么不选 Tauri？

Tauri 使用 Rust 后端 + Web 前端（WebView 渲染），需要额外的 Node.js 工具链和 Web 前端技术栈。Dioxus Native 使用 WGPU 直接渲染，不依赖任何系统 WebView，体积更小、性能更好。

### 为什么不选 Electron？

Electron 体积大（>100MB），内存占用高（>200MB），内嵌 Chromium 渲染引擎。Dioxus Native 使用 WGPU 管线，应用体积通常在 5-15MB，内存占用远低于 Electron。

### 为什么不选 Flutter？

Flutter 使用 Dart 语言和 Skia/Impeller 渲染引擎，与 Rust 的 FFI 调用相对复杂。Dioxus Native 允许纯 Rust 开发，Rust 代码可以直接在 UI 层运行，无需跨语言调用开销。

### 为什么不选 Qt (Rust bindings)？

Qt 的 Rust 绑定（如 CXX-Qt）成熟度不及 Dioxus，且 Qt 的许可协议（GPL/LGPL 合规）存在潜在冲突。Dioxus Native 使用 MIT/Apache-2.0 双重许可，更加友好。