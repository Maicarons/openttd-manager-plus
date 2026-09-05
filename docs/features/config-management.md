# 配置管理

> 配置管理模块负责管理 OpenTTD 的配置文件、模组和资源。

## 配置方案

### 三种模式

#### 1. 独立配置（Isolated）

每个版本实例拥有自己完整的配置目录：

```
instances/official-14.1/
├── openttd.cfg
├── newgrf/
├── gm/
├── data/
├── saves/
└── scenario/
```

**适用场景**：不同版本需要不同模组和配置。

#### 2. 共享配置（Shared）

多个版本共享同一个配置目录：

```
shared/
├── openttd.cfg        # 共享配置
├── newgrf/            # 共享模组
├── gm/                # 共享音轨
├── data/              # 共享数据
├── saves/             # 共享存档
└── scenario/          # 共享场景

instances/official-14.1/  → 符号链接到 shared/
instances/jgrpp-0.59.1/  → 符号链接到 shared/
```

**适用场景**：多个版本使用相同配置和模组。

#### 3. 混合模式（Hybrid）

部分资源共享，部分独立：

```
instances/official-14.1/
├── openttd.cfg        # 独立配置文件
├── newgrf/            # 独立模组
├── gm/                → 共享 (shared/gm/)
├── data/              → 共享 (shared/data/)
├── saves/             # 独立存档
└── scenario/          # 独立场景
```

**适用场景**：需要独立配置和新模组，但共享基础资源。

### 配置方案管理

- 创建、编辑、删除配置方案
- 配置方案导入/导出
- 默认方案设置

## openttd.cfg 编辑器

### 图形化编辑

- 分类浏览配置项
- 搜索配置项
- 修改和保存配置
- 配置验证

### 配置预设

- 新手推荐配置
- 高性能配置
- 联机优化配置
- 自定义配置模板

### 备份与恢复

- 自动备份（修改前生成备份）
- 手动备份点
- 配置版本历史
- 一键恢复

## 资源路径管理

### 路径类型

| 路径 | 说明 | 默认值 |
|------|------|--------|
| `newgrf` | NewGRF 模组目录 | `{instance_dir}/newgrf` |
| `gm` | 音轨集目录 | `{instance_dir}/gm` |
| `data` | 数据文件目录 | `{instance_dir}/data` |
| `saves` | 存档目录 | `{instance_dir}/saves` |
| `scenario` | 场景目录 | `{instance_dir}/scenario` |
| `baseset` | 基础集目录 | `{instance_dir}/baseset` |

### 路径管理

- 图形化路径修改
- 路径验证（有效性检查）
- 相对路径/绝对路径切换
- 路径重置到默认值