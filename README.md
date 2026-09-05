# OpenTTD Manager Plus

> 一个跨平台的开源 OpenTTD 版本管理器与启动器，灵感来源于 Minecraft 启动器（如 HMCL、PCL、Prism Launcher）。

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Built with Dioxus](https://img.shields.io/badge/Built%20with-Dioxus-0.7-blueviolet.svg)](https://dioxuslabs.com)
[![CI](https://github.com/user/openttd-manager-plus/actions/workflows/ci.yml/badge.svg)](https://github.com/user/openttd-manager-plus/actions/workflows/ci.yml)
[![Build](https://github.com/user/openttd-manager-plus/actions/workflows/build.yml/badge.svg)](https://github.com/user/openttd-manager-plus/actions/workflows/build.yml)
[![Tests](https://img.shields.io/badge/tests-223%20passed-green.svg)](https://github.com/user/openttd-manager-plus)
[![Rust](https://img.shields.io/badge/Rust-1.80+-orange.svg)](https://rust-lang.org)

## 概述

OpenTTD Manager Plus 是一个使用 **Rust + Dioxus Native** 构建的跨平台 OpenTTD 版本管理器。它采用 **Blitz/WGPU** 原生渲染引擎（无需 WebView），支持桌面端（Windows/Linux/macOS）和移动端（Android/iOS），提供统一的版本管理、模组管理、配置管理和游戏启动体验。

## 项目状态

| Phase | 内容 | 状态 |
|-------|------|------|
| Phase 1 | 基础框架 (MVP) — 核心库、下载引擎、启动器、桌面骨架 | ✅ 完成 |
| Phase 2 | 核心功能 — 多版本源、下载队列、配置管理、UI 连接 | ✅ 完成 |
| Phase 3 | 增强功能 — 自定义导入、存档管理、国际化、主题系统 | ✅ 完成 |
| Phase 4 | 移动端与优化 — Android/iOS 应用、性能优化、安全加固 | ✅ 完成 |
| Phase 5 | 发布准备 — CI/CD 流水线、打包脚本、文档完善 | ✅ 完成 |

## 特性

| 特性 | 说明 |
|------|------|
| 📦 多版本管理 | 官方 OpenTTD、JGRPP、CityMania Client（CMClient）等 fork 版本 |
| ⚡ 镜像加速下载 | 自动选择最优镜像源（ghproxy/ghfast），断点续传，SHA-256 校验 |
| 🔧 配置管理 | 独立/共享/混合三种配置方案，openttd.cfg 图形化编辑器 |
| 🎨 模组管理 | 在线浏览 BaNaNaS 模组（NewGRF/AI/GameScript/音轨集） |
| 🚀 启动器 | 一键启动，参数构建，多实例运行，进程监控 |
| 📱 移动端 | Android APK 下载安装，iOS 版本浏览，触摸优化 UI |
| 🗺️ 存档管理 | 浏览、导入、导出、备份、恢复、搜索存档 |
| 🌐 国际化 | 中文（简体）和 English 支持，可切换 |
| 🎨 主题系统 | 浅色/深色主题切换 |
| 🛡️ 安全加固 | 路径遍历防护、URL 验证、文件校验、依赖审计 |

## 快速开始

```bash
# 安装 Dioxus CLI
curl -fsSL https://dioxuslabs.com/install.sh | bash

# 构建并运行桌面版
dx serve --platform desktop

# 构建 Android 版
dx serve --platform android

# 构建 iOS 版（需 macOS + Xcode）
dx serve --platform ios

# 运行测试
cargo test --workspace
```

## 构建与打包

```bash
# 开发构建
cargo build -p otmp-desktop

# 发布构建
dx build --platform desktop --release

# 打包安装包
dx bundle --platform desktop --release

# 使用脚本一键构建
# Linux/macOS:
bash scripts/release.sh
# Windows:
scripts\release.bat
```

## 技术栈

| 技术 | 版本 | 用途 |
|------|------|------|
| Rust | 1.80+ | 编译语言 |
| Dioxus | 0.7.10 | UI 框架 |
| Blitz/WGPU | 0.7 | 原生渲染引擎（无需 WebView） |
| Tokio | 1.x | 异步运行时 |
| reqwest | 0.12 | HTTP 客户端 |
| VitePress | 1.x | 文档系统 |

## 项目结构

```
openttd-manager-plus/
├── .github/workflows/     # CI/CD 流水线
├── apps/                  # 应用入口
│   ├── desktop/           # 桌面端（Win/Linux/macOS）
│   └── mobile/            # 移动端（Android/iOS）
├── core/                  # 核心库
│   ├── manager/           # 版本管理（52 测试）
│   ├── downloader/        # 下载引擎（58 测试）
│   ├── config/            # 配置管理（80 测试）
│   └── launcher/          # 启动器（35 测试）
├── docs/                  # 文档（VitePress）
├── scripts/               # 构建脚本
├── Cargo.toml             # 工作空间定义
├── Dioxus.toml            # Dioxus 平台配置
├── CHANGELOG.md           # 变更日志
├── CONTRIBUTING.md        # 贡献指南
└── LICENSE                # AGPL-3.0
```

## 测试

```bash
# 运行所有测试
cargo test --workspace

# 运行特定模块测试
cargo test -p otmp-core-manager
cargo test -p otmp-core-downloader
cargo test -p otmp-core-launcher
cargo test -p otmp-core-config
```

**当前测试覆盖：223 个测试，全部通过**

| 模块 | 测试数 | 状态 |
|------|--------|------|
| otmp-core-config | 80 | ✅ |
| otmp-core-downloader | 58 | ✅ |
| otmp-core-manager | 52 | ✅ |
| otmp-core-launcher | 33 | ✅ |

## 许可

Copyright (C) 2026 OpenTTD Manager Plus Contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

See [LICENSE](./LICENSE) for details.