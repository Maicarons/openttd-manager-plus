# 下载引擎

> 下载引擎负责高效、可靠地获取 OpenTTD 版本文件和相关资源。

## 核心特性

### 多线程下载

- 分片下载，每个分片独立线程
- 动态调整并发数
- 内存缓冲 + 文件写入分离

### 断点续传

- 下载过程中记录已下载字节数
- 中断后自动恢复，跳过已完成部分
- 支持服务器 ETag 验证

### 进度报告

- 实时速度显示（MB/s）
- 预计剩余时间（ETA）
- 已完成百分比
- 下载队列总体进度

## 镜像加速

### 镜像源列表

| 镜像源 | 域名 | 适用场景 |
|--------|------|---------|
| GitHub 官方 | `github.com` | 海外用户直连 |
| ghproxy | `ghproxy.com` | GitHub Release 代理 |
| 镜像站 (由 GitHub Action 同步) | 自建 | 国内加速 |

### 自动选择策略

```
1. 延迟测试 → 对每个镜像发起 HEAD 请求
2. 排序 → 按延迟升序排列
3. 选取 → 选取最快镜像
4. 监控 → 下载过程中监控速度
5. 切换 → 速度低于阈值时自动切换到下一个
```

### 故障转移

- 镜像不可用时自动尝试下一个
- 下载过程中断时自动恢复
- 所有镜像失败后提示用户

## 下载队列

- 支持多个下载任务排队
- 优先级管理（版本下载 > 模组下载 > 资源下载）
- 并发控制（最多同时下载 3 个任务）
- 任务状态：等待中 / 下载中 / 已完成 / 失败

## 数据结构

```rust
/// 下载任务
struct DownloadTask {
    id: Uuid,
    url: String,
    destination: PathBuf,
    expected_size: Option<u64>,
    expected_checksum: Option<String>,
    status: DownloadStatus,
    progress: DownloadProgress,
    created_at: NaiveDateTime,
}

/// 下载状态
enum DownloadStatus {
    Pending,
    Downloading,
    Paused,
    Completed,
    Failed(String),
    Cancelled,
}

/// 下载进度
struct DownloadProgress {
    bytes_downloaded: u64,
    total_bytes: Option<u64>,
    speed: f64,           // bytes per second
    eta: Option<Duration>,
}
```