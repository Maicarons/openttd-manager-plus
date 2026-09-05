# OpenTTD Manager Plus 项目规范

## 分支策略

- `main` — 稳定发布版本
- `develop` — 开发主分支
- `feature/<name>` — 功能分支
- `fix/<name>` — Bug 修复分支
- `release/vX.Y.Z` — 发布分支

## 提交信息规范

遵循 [Conventional Commits](https://www.conventionalcommits.org/)：

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

类型：
- `feat` — 新功能
- `fix` — Bug 修复
- `docs` — 文档
- `style` — 代码格式
- `refactor` — 重构
- `test` — 测试
- `chore` — 构建/工具/CI
- `perf` — 性能优化
- `security` — 安全加固

## 版本号规范

遵循语义化版本 2.0：`主版本.次版本.修订号[-预发布号]`

## 发布流程

1. 从 `develop` 创建 `release/vX.Y.Z` 分支
2. 更新版本号（Cargo.toml）
3. 更新 CHANGELOG.md
4. 运行 `cargo test --workspace` 确认所有测试通过
5. 创建 GitHub Release
6. CI/CD 自动构建所有平台产物
7. 合并到 `main` 分支
8. 打 Git Tag

## 目录结构

```
openttd-manager-plus/
├── .github/workflows/     # CI/CD 配置
├── apps/                  # 应用入口
│   ├── desktop/           # 桌面端
│   └── mobile/            # 移动端
├── core/                  # 核心库
│   ├── manager/           # 版本管理
│   ├── downloader/        # 下载引擎
│   ├── config/            # 配置管理
│   └── launcher/          # 启动器
├── docs/                  # 文档 (VitePress)
├── scripts/               # 构建脚本
├── Cargo.toml             # 工作空间定义
├── Dioxus.toml            # Dioxus 平台配置
├── CHANGELOG.md           # 变更日志
└── LICENSE                # AGPL-3.0
```