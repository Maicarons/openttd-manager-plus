# OpenTTD Manager Plus

> 一个跨平台的开源 OpenTTD 版本管理器与启动器，灵感来源于 Minecraft 启动器（如 HMCL、PCL、Prism Launcher）。

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Built with Dioxus](https://img.shields.io/badge/Built%20with-Dioxus-0.7-blueviolet.svg)](https://dioxuslabs.com)

## 特性

- **多版本管理** — 支持官方 OpenTTD、JGRPP、CityMania Client（CMClient）等 fork 版本
- **独立/共享配置** — 每个版本可拥有独立的 `openttd.cfg`、模组、图像/音频库，也可共享
- **镜像加速下载** — 自动选择最优镜像源，加速版本与资源下载
- **自定义版本下载** — 支持从自定义 URL 或本地文件导入版本
- **跨平台桌面端** — Windows / Linux / macOS
- **移动端支持** — Android（APK 安装管理）+ iOS
- **模组管理** — 在线浏览、下载、管理 NewGRF、AI、GameScript、音轨集
- **存档管理** — 浏览、导入、导出、备份存档与场景
- **多语言** — 支持国际化（i18n）

## 快速开始

```bash
# 安装 Dioxus CLI
curl -fsSL https://dioxuslabs.com/install.sh | bash

# 构建桌面版
dx serve --platform desktop

# 构建 Android 版
dx serve --platform android
```

> 详细文档请参阅 [docs/](./docs/) 目录。

## 项目结构

```
openttd-manager-plus/
├── apps/                   # 应用入口
│   ├── desktop/           # 桌面端（Win/Linux/macOS）
│   └── mobile/            # 移动端（Android/iOS）
├── core/                  # 核心库
│   ├── manager/           # 版本管理逻辑
│   ├── downloader/        # 下载引擎
│   ├── config/            # 配置管理
│   └── launcher/          # 启动器核心
├── docs/                  # 文档（VitePress）
├── Cargo.toml             # 工作空间定义
├── Dioxus.toml            # Dioxus 平台配置
└── LICENSE                # AGPL v3
```

## 许可

Copyright (C) 2026 OpenTTD Manager Plus Contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

See [LICENSE](./LICENSE) for details.