# 移动端 (Android & iOS)

> 移动端为 Android 和 iOS 平台特别设计，使用 Dioxus Native Blitz 渲染器，提供触摸友好的 OpenTTD 版本管理和 APK/IPA 安装管理体验。

## 基础技术

- **UI 框架**：Dioxus 0.7 + Dioxus Native
- **渲染引擎**：Blitz (WGPU + Vello) — 原生 GPU 渲染，**无需 WebView**
- **窗口系统**：Android Activity / iOS UIKit
- **构建工具**：`dx` CLI — `dx serve --platform android` / `dx serve --platform ios`
- **配置**：`Dioxus.toml` — 统一管理 AndroidManifest.xml、Info.plist、权限等

## Android

### 核心功能

#### APK 下载管理

- 内置 OpenTTD Android 版本列表
- 自动获取最新 APK 版本信息
- 下载进度显示
- 下载完成通知

#### APK 安装

- 调用系统 PackageInstaller 安装 APK
- 请求 `REQUEST_INSTALL_PACKAGES` 权限
- 安装进度显示
- 安装完成通知

#### 存储管理

- 使用 SAF (Storage Access Framework) 管理文件
- 查看已下载的 APK 文件
- 清理下载缓存

### APK 版本源

| 来源 | 说明 |
|------|------|
| GitHub Releases | 从 OpenTTD GitHub 发布页获取 APK |
| F-Droid | 从 F-Droid 仓库获取 |
| 自定义 URL | 用户输入的 APK 下载链接 |

### 权限管理

| 权限 | 用途 | 请求时机 |
|------|------|---------|
| `INTERNET` | 网络下载 | 安装时自动（Dioxus.toml 配置） |
| `WRITE_EXTERNAL_STORAGE` | 下载文件存储 | Android 10 以下 |
| `REQUEST_INSTALL_PACKAGES` | 安装 APK | 安装时请求 |
| `POST_NOTIFICATIONS` | 下载/安装通知 | Android 13+ |

## iOS

### 核心功能

iOS 版本受限于 Apple 沙盒政策，功能范围有所不同：

- **版本浏览** — 查看 OpenTTD 各版本信息
- **IPA 管理** — 管理已安装的 OpenTTD IPA 包
- **配置同步** — 通过 iCloud 或文件共享同步配置
- **跨平台** — 与桌面端共享配置和模组

### 限制

- **无法直接安装 IPA** — iOS 应用必须通过 App Store 分发
- **沙盒文件系统** — 无法访问其他应用的文件夹
- **后台下载受限** — 需要 Background Tasks 框架

## 触摸优化 UI

### 布局

```
┌─────────────────────────┐
│     状态栏 + 标题栏      │
├─────────────────────────┤
│                         │
│     内容区域             │
│     (版本列表 /          │
│      下载管理 /          │
│      设置)              │
│                         │
│                         │
├─────────────────────────┤
│ 首页 │ 版本 │ 下载 │ 设置 │  ← 底部导航栏
└─────────────────────────┘
```

### 交互设计

- 底部导航栏（Material Design 3 / iOS 风格）
- 大按钮和触摸区域（≥44pt）
- 滑动操作（删除、刷新）
- 下拉刷新
- 底部弹出菜单（Action Sheet）

## 构建与部署

### Android

```bash
# 开发
dx serve --platform android

# 发布构建
dx build --platform android --release

# 打包 APK
dx bundle --platform android --release
```

### iOS

```bash
# 开发（需 Xcode）
dx serve --platform ios

# 发布构建
dx build --platform ios --release

# 打包 IPA
dx bundle --platform ios --release
```

## 注意事项

- **权限处理**：Android 权限系统复杂，需妥善处理权限请求和拒绝情况
- **后台下载**：Android 使用 Foreground Service 确保后台下载不被系统杀死
- **GPU 兼容性**：Blitz 渲染器需要 Vulkan (Android) 或 Metal (iOS) 支持
- **屏幕适配**：支持手机和平板，自适应布局
- **电量优化**：批量下载时合并网络请求，减少电量消耗
- **存储空间**：下载前检查存储空间是否充足