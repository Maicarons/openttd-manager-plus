# 构建与发布

> 本章描述 OpenTTD Manager Plus 的构建系统和发布流程。

## 构建系统

Dioxus 0.7 使用 `dx` CLI 作为统一的构建和打包工具。Blitz 原生渲染器使用系统 GPU 管线，无需 WebView 依赖。

### 工作空间构建

```bash
# 安装 Dioxus CLI
curl -fsSL https://dioxuslabs.com/install.sh | bash

# 桌面端开发（Blitz/WGPU 原生渲染）
dx serve --platform desktop

# 桌面端发布构建
dx build --platform desktop --release

# 打包桌面端安装包
dx bundle --platform desktop --release
```

### 桌面端构建

#### Windows

```bash
# 构建 + 打包
dx bundle --platform desktop --release

# 输出位置: target/desktop/release/openttd-manager-plus.msi
# 无需 WebView2 运行时，应用本体约 15MB
```

#### Linux

```bash
# 安装 Linux 依赖（Vulkan 驱动）
sudo apt install mesa-vulkan-drivers

# 构建 + 打包 AppImage
dx bundle --platform desktop --release
```

#### macOS

```bash
# 构建 + 打包 .dmg
dx bundle --platform desktop --release
```

### Android 构建

```bash
# 添加 Android 目标
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android

# 开发
dx serve --platform android

# 发布构建 APK
dx build --platform android --release

# 打包 APK 或 AAB
dx bundle --platform android --release
```

### iOS 构建 (需 macOS + Xcode)

```bash
# 添加 iOS 目标
rustup target add aarch64-apple-ios

# 开发 (iOS 模拟器)
dx serve --platform ios

# 发布构建 IPA
dx build --platform ios --release

# 打包
dx bundle --platform ios --release
```

### Web 构建

```bash
# 构建 WASM 版本（使用 dioxus-web 渲染器）
dx build --platform web --release
```

## CI/CD (GitHub Actions)

### 工作流配置

```yaml
# .github/workflows/build.yml
name: Build

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]
  release:
    types: [created]

jobs:
  build-desktop:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - name: Install Dioxus CLI
        run: curl -fsSL https://dioxuslabs.com/install.sh | bash
      - name: Build
        run: dx build --platform desktop --release
      - uses: actions/upload-artifact@v4
        with:
          name: desktop-${{ matrix.os }}
          path: target/desktop/release/*

  build-mobile:
    strategy:
      matrix:
        platform: [android, ios]
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - name: Install Dioxus CLI
        run: curl -fsSL https://dioxuslabs.com/install.sh | bash
      - name: Add targets
        run: |
          rustup target add aarch64-linux-android
          rustup target add aarch64-apple-ios
      - name: Build
        run: dx build --platform ${{ matrix.platform }} --release
      - uses: actions/upload-artifact@v4
        with:
          name: mobile-${{ matrix.platform }}
          path: target/${{ matrix.platform }}/release/*
```

## 版本号规范

遵循语义化版本 2.0：

```
主版本.次版本.修订号 [-预发布号]
```

- **主版本**：不兼容的 API 变更
- **次版本**：向下兼容的功能新增
- **修订号**：向下兼容的问题修正
- **预发布号**：alpha, beta, rc

## 发布流程

```
1. 创建发布分支 (release/vX.Y.Z)
2. 更新版本号 (Cargo.toml)
3. 更新 CHANGELOG
4. 运行完整测试套件
5. 创建 GitHub Release
6. CI/CD 自动构建所有平台产物
7. 上传构建产物到 Release
8. 合并回 main 分支
9. 打 Git Tag
```

## 发布渠道

| 渠道 | 说明 | 更新频率 |
|------|------|---------|
| Stable | 稳定发布版 | 1-2 月 |
| Beta | 测试版 | 2-4 周 |
| Nightly | 每日构建 | 每天 (CI) |
| GitHub Releases | 所有版本 | 按需 |

## Dioxus.toml 配置

```toml
[application]
name = "OpenTTD Manager Plus"
default_platform = "desktop"

[web]
title = "OpenTTD Manager Plus"

[desktop]
width = 1200
height = 800
resizable = true
min_width = 800
min_height = 600

[mobile]
# 移动端配置通过 Dioxus.toml 统一管理
# 包括 AndroidManifest.xml、Info.plist、权限等
```