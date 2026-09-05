# 架构设计

> 本章描述 OpenTTD Manager Plus 的整体架构设计、模块划分和数据流。

## 总体架构

```
┌─────────────────────────────────────────────────────────────────┐
│                    UI Layer (Dioxus Native)                      │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────────┐  │
│  │ 版本管理  │ │ 下载管理 │ │ 配置管理  │ │  模组管理       │  │
│  │  页面     │ │  页面    │ │  页面     │ │  页面           │  │
│  └─────┬────┘ └────┬───┬─┘ └────┬─────┘ └───────┬──────────┘  │
│        │           │   │        │               │             │
│  ┌─────┴───────────┴───┴────────┴───────────────┴──────────┐  │
│  │          State Management (Signals / Context)            │  │
│  └─────────────────────────┬─────────────────────────────────┘  │
│                            │                                     │
│  ┌─────────────────────────▼─────────────────────────────────┐  │
│  │              Blitz Native Renderer Layer                    │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────────┐  │  │
│  │  │Blitz DOM │ │ Stylo    │ │ Taffy    │ │ Vello/WGPU │  │  │
│  │  │(HTML/CSS)│ │ (样式)   │ │ (布局)   │ │ (渲染)     │  │  │
│  │  └──────────┘ └──────────┘ └──────────┘ └────────────┘  │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────┼────────────────────────────────────┐
│                    Core Layer (Rust)                            │
│  ┌────────────────┐ ┌──────┴──────┐ ┌──────────────────┐       │
│  │  otmp-core-     │ │ otmp-core-  │ │ otmp-core-       │       │
│  │  manager        │ │ downloader  │ │ config            │       │
│  │  (版本管理)     │ │ (下载引擎)  │ │ (配置管理)       │       │
│  └───────┬─────────┘ └──────┬──────┘ └────────┬─────────┘       │
│          │                  │                  │                 │
│  ┌───────┴──────────────────┴──────────────────┴──────────┐    │
│  │                    otmp-core-launcher                    │    │
│  │                    (启动器核心)                          │    │
│  └───────────────────────────┬──────────────────────────────┘    │
│                              │                                   │
│  ┌───────────────────────────┴──────────────────────────────┐    │
│  │               Platform Abstraction Layer                  │    │
│  │  (平台特定实现：进程管理、文件路径、权限、APK/IPA 安装)  │    │
│  └──────────────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────────────┘
```

## 渲染架构

```
┌─────────────────────────────────────────────────────┐
│                  Dioxus VirtualDom                    │
│  (rsx! macro → VNode tree → diff → patch)           │
└───────────────────────┬─────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────┐
│              dioxus-native-dom / DioxusDocument       │
│  (Translates VirtualDom changes into DOM tree ops)   │
└───────────────────────┬─────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────┐
│                    Blitz DOM                          │
│  ┌─────────────────────────────────────────────────┐ │
│  │  HTML Parser → CSS Cascade (Stylo) → Layout    │ │
│  │  (Taffy) → Text Layout (Parley)                │ │
│  └─────────────────────────────────────────────────┘ │
└───────────────────────┬─────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────┐
│              Blitz Paint (Vello + WGPU)              │
│  ┌─────────────────────────────────────────────────┐ │
│  │  Compute Shader Pipeline → GPU Rasterization    │ │
│  │  DirectX 12 / Vulkan / Metal 后端               │ │
│  └─────────────────────────────────────────────────┘ │
└───────────────────────┬─────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────┐
│              Platform Windowing (winit)              │
│  Win32 / X11 / Wayland / Cocoa / Android / UIKit    │
└─────────────────────────────────────────────────────┘
```

## 模块划分

### 1. 核心库 (Core Layer)

#### `otmp-core-manager` — 版本管理

```
src/
├── lib.rs                           # 入口
├── sources/                         # 版本源
│   ├── mod.rs
│   ├── official.rs                  # 官方 OpenTTD 源
│   ├── jgrpp.rs                     # JGRPP 源
│   └── cmclient.rs                  # CityMania Client 源
├── version/                         # 版本模型
│   ├── mod.rs
│   ├── model.rs                     # 版本结构体
│   └── compare.rs                   # 版本比较
└── manifest/                        # 版本清单
    ├── mod.rs
    ├── fetch.rs                     # 远程清单获取
    └── cache.rs                     # 本地清单缓存
```

#### `otmp-core-downloader` — 下载引擎

```
src/
├── lib.rs
├── engine/                          # 下载引擎
│   ├── mod.rs
│   ├── download.rs                  # 核心下载逻辑
│   ├── stream.rs                    # 流式下载
│   └── resume.rs                    # 断点续传
├── mirror/                          # 镜像管理
│   ├── mod.rs
│   ├── selector.rs                  # 镜像选择器
│   └── sources.rs                   # 镜像源配置
└── verify/                          # 完整性验证
    ├── mod.rs
    └── checksum.rs                  # 校验和验证
```

#### `otmp-core-config` — 配置管理

```
src/
├── lib.rs
├── profile/                         # 配置方案
│   ├── mod.rs
│   ├── model.rs                     # 配置方案模型
│   └── manager.rs                   # 配置方案管理
├── instance/                        # 实例管理
│   ├── mod.rs
│   ├── model.rs                     # 实例模型
│   └── io.rs                        # 实例 I/O
└── migration/                       # 配置迁移
    ├── mod.rs
    └── converter.rs                 # 配置转换器
```

#### `otmp-core-launcher` — 启动器核心

```
src/
├── lib.rs
├── runner/                          # 游戏运行器
│   ├── mod.rs
│   ├── direct.rs                    # 直接启动
│   └── isolated.rs                  # 隔离启动（沙箱）
├── process/                         # 进程管理
│   ├── mod.rs
│   ├── start.rs                     # 进程启动
│   └── monitor.rs                   # 进程监控
└── args/                            # 命令行参数
    ├── mod.rs
    ├── builder.rs                   # 参数构建器
    └── templates.rs                 # 参数模板
```

### 2. 平台层 (Platform Layer)

每个平台实现特定接口：

| 接口 | Windows | Linux | macOS | Android | iOS |
|------|---------|-------|-------|---------|-----|
| 窗口系统 | Win32 | X11/Wayland | Cocoa | Android Activity | UIKit |
| 文件路径 | `%APPDATA%` | `~/.local/share` | `~/Library/Application Support` | 内部存储 | 沙盒 |
| 进程管理 | `CreateProcess` | `fork/exec` | `NSTask` | `ProcessBuilder` | 受限 |
| 渲染后端 | DirectX 12 | Vulkan | Metal | Vulkan | Metal |
| APK/IPA | N/A | N/A | N/A | `PackageInstaller` | 仅 App Store |

### 3. 应用层 (Apps)

#### 桌面端 (`apps/desktop`)

```
src/
├── main.rs                          # 入口 (dioxus_native::launch)
├── components/                      # UI 组件
│   ├── layout/                      # 布局组件
│   │   ├── sidebar.rs
│   │   ├── header.rs
│   │   └── content.rs
│   ├── version/                     # 版本管理组件
│   │   ├── list.rs
│   │   ├── detail.rs
│   │   └── install.rs
│   ├── download/                    # 下载组件
│   │   ├── progress.rs
│   │   └── queue.rs
│   ├── config/                      # 配置组件
│   │   ├── editor.rs
│   │   └── profile.rs
│   ├── mods/                        # 模组组件
│   │   ├── browser.rs
│   │   ├── detail.rs
│   │   └── manager.rs
│   └── common/                      # 通用组件
│       ├── button.rs
│       ├── modal.rs
│       └── toast.rs
├── pages/                           # 页面
│   ├── home.rs
│   ├── versions.rs
│   ├── downloads.rs
│   ├── configs.rs
│   ├── mods.rs
│   └── settings.rs
└── utils/                           # 工具
    ├── theme.rs
    └── i18n.rs
```

#### 移动端 (`apps/mobile`)

```
src/
├── main.rs                          # 入口 (dioxus_native::launch)
├── components/                      # 移动端适配组件
│   ├── bottom-nav.rs                # 底部导航
│   ├── version-list.rs              # 版本列表
│   ├── apk-installer.rs             # APK 安装器
│   └── ...
├── pages/                           # 页面
│   ├── home.rs
│   ├── versions.rs
│   ├── apk-manager.rs
│   └── settings.rs
└── utils/
    ├── permissions.rs               # 权限管理
    └── platform.rs                  # 平台特定工具
```

## 数据流

### 版本安装流程

```
用户选择版本
    │
    ▼
版本管理器检查本地缓存
    │
    ├── 已缓存 ──→ 校验完整性
    │                  │
    │             ├── 通过 ──→ 解压到实例目录
    │             └── 失败 ──→ 重新下载
    │
    └── 未缓存 ──→ 下载引擎
                       │
                       ├── 镜像选择器 → 选择最优源
                       ├── 多线程下载 → 进度报告
                       └── 完整性验证 → 解压
```

### 游戏启动流程

```
用户点击"启动"
    │
    ▼
启动器核心
    ├── 选择版本 → 确认真实性
    ├── 构建参数 → 加载配置方案
    ├── 设置环境 → 配置工作目录
    └── 启动进程 → 监控运行状态
```

## 数据存储

### 目录结构

```
~/.openttd-manager-plus/          # 程序数据目录
├── instances/                    # 实例目录
│   ├── official-14.1/           # 独立实例
│   │   ├── openttd.cfg
│   │   ├── newgrf/
│   │   ├── gm/
│   │   ├── data/
│   │   ├── saves/
│   │   └── scenario/
│   └── jgrpp-0.59.1/
│       └── ...
├── shared/                       # 共享资源
│   ├── newgrf/                   # 共享 NewGRF
│   ├── gm/                       # 共享音轨
│   └── data/                     # 共享数据文件
├── cache/                        # 缓存
│   ├── downloads/                # 下载缓存
│   └── manifests/                # 版本清单缓存
├── profiles/                     # 配置方案
│   └── default.json
└── config.json                   # 程序配置
```

### 平台特定路径

| 平台 | 数据目录 |
|------|---------|
| Windows | `%APPDATA%/openttd-manager-plus` |
| Linux | `~/.local/share/openttd-manager-plus` |
| macOS | `~/Library/Application Support/openttd-manager-plus` |
| Android | `{internal_storage}/Android/data/openttd-manager-plus` |
| iOS | 沙盒内 `Documents/` 目录 |