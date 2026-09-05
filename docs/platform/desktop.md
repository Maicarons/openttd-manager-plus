# 桌面端

> 桌面端应用是 OpenTTD Manager Plus 的主要平台，支持 Windows、Linux 和 macOS。

## 基础技术

- **UI 框架**：Dioxus 0.7 + Dioxus Native
- **渲染引擎**：Blitz (WGPU + Vello) — 原生 GPU 渲染，无需 WebView
- **窗口系统**：winit（Win32 / X11 / Wayland / Cocoa）
- **构建工具**：`dx` CLI — `dx serve --platform desktop`
- **打包工具**：`dx bundle`

## 平台打包

### Windows

| 格式 | 命令 | 说明 |
|------|------|------|
| Portable `.exe` | `dx build --platform desktop --release` | 单文件，无需 WebView2 |
| Installer `.msi` | `dx bundle --platform desktop --release` | 安装版 |
| Portable `.zip` | 手动打包 | 绿色版 |

**系统要求**：
- Windows 10 1809+ / Windows 11
- **无需 WebView2 运行时**（Blitz 使用 DirectX 12 直接渲染）
- 4GB RAM
- 15MB 磁盘空间（应用本体）

### Linux

| 格式 | 命令 | 说明 |
|------|------|------|
| AppImage | `dx bundle --platform desktop --release` | 通用格式 |
| `.deb` | `dx bundle --platform desktop --release` | Debian/Ubuntu |
| `.rpm` | `dx bundle --platform desktop --release` | Fedora/RHEL |

**系统要求**：
- Linux Kernel 5.0+
- Vulkan 1.1+ 或 OpenGL 3.3+ 支持
- **无需 WebKitGTK**（Blitz 使用 Vulkan 直接渲染）
- 4GB RAM
- 15MB 磁盘空间

### macOS

| 格式 | 命令 | 说明 |
|------|------|------|
| `.dmg` | `dx bundle --platform desktop --release` | 磁盘映像 |
| `.app` | `dx bundle --platform desktop --release` | .app 包 |

**系统要求**：
- macOS 13 (Ventura)+
- **无需 WKWebView**（Blitz 使用 Metal 直接渲染）
- 4GB RAM
- 15MB 磁盘空间

## 桌面 UI 设计要点

- **窗口布局**：左侧导航栏 + 右侧内容区
- **响应式设计**：支持窗口缩放
- **键盘快捷键**：常用操作快捷键
- **系统集成**：文件关联、系统托盘
- **主题**：浅色/深色模式切换