# 贡献指南

感谢您对 DDNS 工具的关注! 我们欢迎各种形式的贡献。

## 🤝 如何贡献

### 报告问题

如果您发现了 Bug 或有功能建议:

1. 在 [Issues](https://github.com/yourusername/ddns-tool/issues) 中搜索是否已有相关问题
2. 如果没有,创建新的 Issue 并提供:
   - 清晰的标题
   - 详细的描述
   - 复现步骤
   - 环境信息 (OS、版本号等)
   - 截图或日志(如果适用)

### 提交代码

#### 开发环境设置

1. **Fork 项目**
   ```bash
   # 在 GitHub 上 Fork 项目到您的账号
   git clone https://github.com/YOUR_USERNAME/ddns-tool.git
   cd ddns-tool
   ```

2. **安装依赖**
   ```bash
   npm install
   cd src-tauri
   cargo build
   ```

3. **创建功能分支**
   ```bash
   git checkout -b feature/your-feature-name
   ```

#### 代码规范

**Rust 代码:**
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 Rust 命名规范
- 添加必要的文档注释
- 编写单元测试

```bash
# 格式化代码
cargo fmt

# 代码检查
cargo clippy -- -D warnings

# 运行测试
cargo test
```

**TypeScript 代码:**
- 使用 ESLint 和 Prettier
- 遵循 TypeScript 严格模式
- 使用有意义的变量和函数名
- 添加 JSDoc 注释

```bash
# 格式化代码
npm run format

# 代码检查
npm run lint
```

#### 提交规范

**Commit Message 格式:**
```
<type>: <subject>

<body>

<footer>
```

**Type 类型:**
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式调整(不影响功能)
- `refactor`: 重构(既不是新功能也不是修复)
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动

**示例:**
```
feat: 添加 AWS Route53 DNS 提供商支持

- 实现 AWS Route53 API 集成
- 添加签名认证
- 编写单元测试

Closes #123
```

#### Pull Request 流程

1. **更新分支**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **推送分支**
   ```bash
   git push origin feature/your-feature-name
   ```

3. **创建 Pull Request**
   - 在 GitHub 上创建 Pull Request
   - 填写 PR 模板
   - 等待 Code Review

4. **代码审查**
   - 响应 Reviewer 的意见
   - 进行必要的修改
   - 保持 Commit 历史整洁

#### PR 检查清单

在提交 PR 前,请确保:

- [ ] 代码通过所有测试 (`cargo test`)
- [ ] 代码通过 linter 检查 (`cargo clippy`, `npm run lint`)
- [ ] 代码已格式化 (`cargo fmt`, `npm run format`)
- [ ] 添加了必要的测试
- [ ] 更新了相关文档
- [ ] Commit Message 遵循规范
- [ ] 功能分支基于最新的 main 分支

## 🎯 开发重点

我们需要贡献的主要领域:

### 1. DNS 提供商实现

优先级: 高

当前状态:
- ✅ Cloudflare - 已完成
- ✅ 阿里云 - 已完成
- ✅ 腾讯云 - 已完成
- 🔲 AWS Route53 - 待实现
- 🔲 华为云 - 待实现
- 🔲 百度云 - 待实现
- 🔲 京东云 - 待实现

实现参考: [src-tauri/src/providers/cloudflare/mod.rs](src-tauri/src/providers/cloudflare/mod.rs)

### 2. 功能增强

- [ ] IPv6 完整支持
- [ ] CNAME 记录支持
- [ ] 通配符域名支持
- [ ] 批量域名导入导出
- [ ] 域名分组管理

### 3. 用户体验

- [ ] 系统托盘菜单
- [ ] 桌面通知
- [ ] 开机自启动
- [ ] 深色主题
- [ ] 多语言支持

### 4. 测试

- [ ] 单元测试
- [ ] 集成测试
- [ ] E2E 测试
- [ ] 性能测试

### 5. 文档

- [ ] API 文档
- [ ] 架构文档
- [ ] 部署指南
- [ ] 故障排除指南

## 📝 开发资源

### 项目结构

```
ddns-tool/
├── src/                    # React 前端
│   ├── components/         # UI 组件
│   ├── lib/               # API 客户端
│   └── types/             # TypeScript 类型
├── src-tauri/             # Rust 后端
│   ├── src/
│   │   ├── commands/      # Tauri 命令
│   │   ├── services/      # 业务逻辑
│   │   ├── providers/     # DNS 提供商
│   │   ├── models/        # 数据模型
│   │   ├── storage/       # 数据库
│   │   └── error.rs       # 错误处理
│   └── migrations/        # SQL 迁移
└── docker/                # Docker 配置
```

### 重要文件

- **CLAUDE.md** - 项目开发指南
- **README.md** - 项目介绍和使用指南
- **PROJECT_SUMMARY.md** - 技术总结
- **CONTRIBUTING.md** - 本文件

### 技术栈

**前端:**
- React 19 + TypeScript
- Tauri 2.0
- TailwindCSS

**后端:**
- Rust
- Tauri 2.0
- Tokio
- SQLx

## 🐛 Bug 报告模板

```markdown
### 描述
简要描述问题

### 复现步骤
1. 步骤 1
2. 步骤 2
3. ...

### 预期行为
描述预期的正确行为

### 实际行为
描述实际发生的错误行为

### 环境信息
- OS: [例如: Ubuntu 22.04]
- 版本: [例如: v0.1.0]
- Node.js: [例如: v18.0.0]
- Rust: [例如: 1.75.0]

### 日志/截图
粘贴相关日志或截图
```

## ✨ 功能请求模板

```markdown
### 功能描述
清晰描述您希望添加的功能

### 使用场景
描述该功能的实际使用场景

### 实现建议(可选)
如果您有实现思路,请描述

### 替代方案
描述可能的替代解决方案
```

## 📜 许可证

通过贡献代码,您同意您的贡献将在 [MIT License](LICENSE) 下发布。

## 💬 讨论

在开始大型功能开发前,建议先通过 [Discussions](https://github.com/yourusername/ddns-tool/discussions) 讨论设计思路。

## 🙏 致谢

感谢所有贡献者的付出!

---

最后更新: 2025年1月6日
