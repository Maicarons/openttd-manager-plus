# 模组管理

> 模组管理模块提供 OpenTTD 模组的在线浏览、下载和管理功能。

## OpenTTD 模组类型

| 类型 | 扩展名 | 说明 |
|------|--------|------|
| NewGRF | `.grf` | 游戏图形、功能增强模组 |
| AI | `.nut` | 电脑对手 AI 脚本 |
| GameScript | `.nut` | 游戏规则脚本 |
| 音轨集 | 目录 | 自定义音乐包 |
| 基础集 | `.obg`, `.obm` | 基础图形/音效集 |

## 在线模组浏览器

### 数据来源

- **BaNaNaS** — OpenTTD 官方在线内容平台
- **GitHub** — 社区模组仓库
- **TT-Forums** — 社区论坛模组发布帖

### 浏览功能

- 分类浏览（NewGRF / AI / GameScript / 音轨集）
- 搜索（按名称、作者、标签）
- 排序（按热度、更新时间、评分）
- 筛选（按游戏版本兼容性、类别）

### 模组详情

- 名称、作者、版本号
- 描述和截图
- 兼容的 OpenTTD 版本
- 依赖关系
- 下载量、评分
- 更新日志

## 本地模组管理

### 已安装模组列表

- 显示所有已安装模组
- 按类型、名称、大小排序
- 启用/禁用（通过修改 openttd.cfg）
- 删除模组

### 模组导入

- 从本地文件导入（`.tar`, `.zip`, `.grf`）
- 拖拽导入
- 批量导入

### 模组冲突检测

- 检测同名模组冲突
- 检测版本兼容性冲突
- 检测依赖缺失
- 冲突解决建议

## 数据结构

```rust
/// 模组信息
struct ModInfo {
    id: String,
    name: String,
    version: String,
    mod_type: ModType,
    author: String,
    description: String,
    url: Option<String>,
    dependencies: Vec<String>,
    compatible_versions: Vec<String>,
    installed: bool,
    install_path: Option<PathBuf>,
    enabled: bool,
}

/// 模组类型
enum ModType {
    NewGRF,
    AI,
    GameScript,
    MusicSet,
    BaseSet,
}
```