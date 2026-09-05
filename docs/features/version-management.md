# 版本管理

> 版本管理是 OpenTTD Manager Plus 的核心功能，提供多版本来源的统一管理。

## 版本源

### 官方 OpenTTD

| 属性 | 说明 |
|------|------|
| 源 URL | `https://cdn.openttd.org/openttd-releases/` |
| 元数据 | `https://cdn.openttd.org/openttd-releases/latest.yaml` |
| 版本类型 | Stable, RC, Beta, Nightly |
| 发布格式 | Windows: `.zip` / Linux: `.tar.gz` / macOS: `.dmg` |

实现方式：
- 定期拉取 `latest.yaml` 获取版本列表
- 解析版本号，按语义化版本排序
- 缓存元数据到本地，降低重复请求

### JGRPP

| 属性 | 说明 |
|------|------|
| 源 URL | `https://github.com/JGRennison/OpenTTD-patches/releases` |
| API | GitHub Releases API |
| 版本类型 | Release, Pre-release |
| 发布格式 | Windows: `.zip` / Linux: `.tar.gz` / macOS: `.dmg` |

实现方式：
- 使用 GitHub Releases API 获取发布列表
- 支持从 GitHub Release 页面解析资产
- 可通过镜像下载（如 ghproxy.com）

### CityMania Client (CMClient)

| 属性 | 说明 |
|------|------|
| 源 URL | `https://github.com/OpenTTD/OpenTTD-cmclient/releases` |
| API | GitHub Releases API |
| 版本类型 | Release, Pre-release |
| 发布格式 | Windows: `.zip` / Linux: `.tar.gz` / macOS: `.dmg` |

### 自定义版本

支持用户从以下源添加自定义版本：
- 任意 HTTP/HTTPS URL（直接下载链接）
- 本地文件（`.zip` / `.tar.gz` / `.7z`）
- 本地已安装的 OpenTTD 目录（导入现有安装）

## 版本操作

### 安装流程

```
1. 选择版本源 → 获取版本列表
2. 选择版本号 → 获取下载信息
3. 镜像选择 → 自动/手动选择下载源
4. 下载 → 流式下载 + 进度显示
5. 校验 → SHA-256 完整性验证
6. 解压 → 到实例目录
7. 注册 → 添加到版本管理器
```

### 版本切换

- 点击版本即可切换默认启动版本
- 支持右键菜单（设置默认、删除、打开目录）
- 版本状态标签：已安装/可更新/未安装/损坏

### 更新检测

- 启动时自动检查版本更新（可配置）
- 已安装版本列表显示更新提示
- 一键更新到最新版本

## 数据结构

```rust
/// 版本源枚举
enum VersionSource {
    Official,
    Jgrpp,
    CmClient,
    Custom(String),
}

/// 版本信息
struct VersionInfo {
    id: Uuid,
    source: VersionSource,
    version: Version,           // 语义化版本
    version_type: VersionType,  // Stable, RC, Nightly, etc.
    name: String,
    release_date: NaiveDateTime,
    downloads: Vec<DownloadAsset>,
    checksums: HashMap<String, String>,
    changelog: Option<String>,
    installed: bool,
    install_path: Option<PathBuf>,
}

/// 版本类型
enum VersionType {
    Stable,
    ReleaseCandidate,
    Beta,
    Nightly,
    PreRelease,
    Custom,
}
```