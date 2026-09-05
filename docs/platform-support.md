# 跨平台概览

> OpenTTD Manager Plus 设计为跨平台应用，覆盖桌面端和移动端。

## 支持矩阵

| 平台 | 支持级别 | 界面 | 构建方式 | 特殊说明 |
|------|---------|------|---------|---------|
| Windows 10/11 | ✅ 完全支持 | 原生风格 | `dx serve --platform desktop` | Blitz (WGPU/DirectX 12) 原生渲染 |
| Linux (x86_64) | ✅ 完全支持 | 原生风格 | `dx serve --platform desktop` | Blitz (WGPU/Vulkan) 原生渲染 |
| Linux (aarch64) | 🟡 支持 | 原生风格 | `dx serve --platform desktop` | 交叉编译 |
| macOS (Intel) | ✅ 完全支持 | 原生风格 | `dx serve --platform desktop` | Blitz (WGPU/Metal) 原生渲染 |
| macOS (Apple Silicon) | ✅ 完全支持 | 原生风格 | `dx serve --platform desktop` | 原生 ARM64 构建 |
| Android | ✅ 支持 | Material Design | `dx serve --platform android` | Blitz (WGPU/Vulkan) 原生渲染 |
| iOS | ✅ 支持 | iOS 风格 | `dx serve --platform ios` | Blitz (WGPU/Metal) 原生渲染 |
| Web | 🟡 支持 | 响应式 | `dx serve --platform web` | 编译为 WASM (WebSys DOM) |

## 平台差异处理

### 文件系统

| 操作 | Windows | Linux | macOS | Android | iOS |
|------|---------|-------|-------|---------|-----|
| 路径分隔符 | `\` | `/` | `/` | `/` | `/` |
| 数据目录 | `%APPDATA%` | `~/.local/share` | `~/Library/Application Support` | 内部存储 | 沙盒 |
| 缓存目录 | `%LOCALAPPDATA%` | `~/.cache` | `~/Library/Caches` | 缓存目录 | 沙盒 |
| 可执行文件 | `.exe` | 无后缀 | `.app` | `.apk` | `.ipa` |
| 符号链接 | 受限 | 支持 | 支持 | 不支持 | 不支持 |

### 进程管理

| 操作 | Windows | Linux/macOS | Android | iOS |
|------|---------|-------------|---------|-----|
| 渲染后端 | DirectX 12 | Vulkan / Metal | Vulkan | Metal |
| 窗口系统 | Win32 | X11/Wayland / Cocoa | Activity | UIKit |
| 启动进程 | `CreateProcess` | `fork/exec` | `ProcessBuilder` | 受限 |
| 进程监控 | `JobObject` | `waitpid` | `ActivityManager` | 受限 |
| 终止进程 | `TerminateProcess` | `kill` | `killProcess` | 受限 |
| 环境变量 | `SetEnvironmentVariable` | `setenv` | `ProcessBuilder` | `ProcessInfo` |

### 网络

| 特性 | Windows | Linux | macOS | Android | iOS |
|------|---------|-------|-------|---------|-----|
| HTTP 代理 | 系统代理 | 环境变量/系统代理 | 系统代理 | 系统代理 | 系统代理 |
| DNS 解析 | 系统 | 系统 | 系统 | 系统 | 系统 |
| 证书验证 | 系统 | 系统 | 系统 | 系统 | 系统 |