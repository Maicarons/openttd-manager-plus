# 贡献指南

> 感谢您对 OpenTTD Manager Plus 的兴趣！我们欢迎各种形式的贡献。

## 行为准则

本项目采用 [Contributor Covenant](https://www.contributor-covenant.org/) 行为准则。请阅读并遵守。

## 如何贡献

### 报告 Bug

1. 搜索 [Issues](https://github.com/user/openttd-manager-plus/issues) 确认是否已存在
2. 使用 Bug 报告模板创建 Issue
3. 提供尽可能详细的信息：
   - 操作系统和版本
   - OpenTTD Manager Plus 版本
   - 复现步骤
   - 预期行为和实际行为
   - 日志文件（如果有）

### 提交功能请求

1. 搜索 Issues 确认是否已存在
2. 使用功能请求模板创建 Issue
3. 描述功能详情和使用场景

### 提交代码

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交修改 (`git commit -m 'feat: add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 开发环境设置

```bash
# 克隆仓库
git clone https://github.com/user/openttd-manager-plus.git
cd openttd-manager-plus

# 构建所有工作空间
cargo build --workspace

# 构建桌面端
cargo build -p otmp-desktop

# 运行文档
cd docs
npm install
npm run docs:dev
```

## 编码规范

### Rust

- 遵循 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 所有公共 API 必须有文档注释
- 添加适当的单元测试和集成测试

### 提交信息规范

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
- `chore` — 构建/工具

### 分支命名

- `feature/<name>` — 新功能
- `fix/<name>` — Bug 修复
- `docs/<name>` — 文档
- `release/<version>` — 发布分支

## Pull Request 流程

1. PR 标题使用 Conventional Commits 格式
2. 关联相关 Issue
3. 确保 CI 通过
4. 至少一位维护者审查
5. 审查通过后合并

## 测试

```bash
# 运行所有测试
cargo test --workspace

# 运行特定 crate 测试
cargo test -p otmp-core-manager

# 运行文档测试
cargo test --doc
```