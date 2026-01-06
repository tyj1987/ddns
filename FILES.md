# 项目文件清单

本文档列出 DDNS 工具项目的所有重要文件。

**最后更新**: 2025年1月6日
**版本**: v0.1.0-alpha

---

## 📁 根目录文件

### 核心文档 (11个)

| 文件 | 说明 | 行数 |
|------|------|------|
| [README.md](README.md) | 项目主页,功能介绍和快速开始 | ~310 |
| [CHANGELOG.md](CHANGELOG.md) | 版本变更记录 | ~60 |
| [FAQ.md](FAQ.md) | 常见问题解答 | ~400 |
| [CONTRIBUTING.md](CONTRIBUTING.md) | 贡献者指南 | ~300 |
| [CLAUDE.md](CLAUDE.md) | 开发者指南 (Claude Code 专用) | ~400 |
| [TESTING_GUIDE.md](TESTING_GUIDE.md) | 测试指南 | ~500 |
| [RELEASE_CHECKLIST.md](RELEASE_CHECKLIST.md) | 发布检查清单 | ~400 |
| [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) | 技术总结 | ~300 |
| [FINAL_REPORT.md](FINAL_REPORT.md) | 项目完成报告 | ~600 |
| [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) | 项目结构说明 | ~200 |
| [COMPLETION_REPORT.md](COMPLETION_REPORT.md) | 完成度报告 | ~250 |

### 配置文件 (7个)

| 文件 | 说明 |
|------|------|
| [.gitignore](.gitignore) | Git 忽略规则 |
| [LICENSE](LICENSE) | MIT 开源许可证 |
| [.env.example](.env.example) | 环境变量配置示例 |
| [package.json](package.json) | Node.js 依赖和脚本 |
| [tsconfig.json](tsconfig.json) | TypeScript 配置 |
| [vite.config.ts](vite.config.ts) | Vite 构建配置 |
| [tailwind.config.js](tailwind.config.js) | TailwindCSS 配置 |

### 开发脚本 (5个)

| 脚本 | 说明 |
|------|------|
| [scripts/dev.sh](scripts/dev.sh) | 启动开发模式 |
| [scripts/build.sh](scripts/build.sh) | 构建项目 |
| [scripts/check.sh](scripts/check.sh) | 检查代码 |
| [scripts/test.sh](scripts/test.sh) | 运行测试 |
| [scripts/format.sh](scripts/format.sh) | 格式化代码 |
| [scripts/clean.sh](scripts/clean.sh) | 清理构建产物 |

---

## 📁 源代码目录

### 前端源码 (src/)

```
src/
├── components/           # React 组件
│   ├── DomainList.tsx   # 域名列表组件
│   ├── DomainForm.tsx   # 域名表单组件
│   ├── LogViewer.tsx    # 日志查看器
│   ├── ProviderForms/   # DNS 提供商表单
│   │   ├── CloudflareForm.tsx
│   │   ├── AliyunForm.tsx
│   │   ├── TencentForm.tsx
│   │   └── index.ts
│   └── Settings.tsx     # 设置页面
├── hooks/               # React Hooks
│   └── useDomains.ts    # 域名管理 Hook
├── lib/                 # 工具库
│   ├── api.ts          # Tauri API 封装
│   └── store.ts        # Zustand 状态管理
├── types/               # TypeScript 类型定义
│   └── index.ts
└── App.tsx             # 主应用组件
```

**代码统计**: ~2,500 行 TypeScript/TSX

### 后端源码 (src-tauri/)

```
src-tauri/
├── src/
│   ├── commands/        # Tauri IPC 命令
│   │   ├── mod.rs
│   │   ├── domains.rs    # 域名管理命令
│   │   ├── scheduler.rs  # 调度器命令
│   │   ├── logs.rs       # 日志命令
│   │   └── settings.rs   # 设置命令
│   ├── providers/       # DNS 提供商
│   │   ├── mod.rs
│   │   ├── provider_trait.rs  # Trait 定义
│   │   ├── cloudflare/        # Cloudflare 实现
│   │   ├── aliyun/            # 阿里云实现
│   │   ├── tencent/           # 腾讯云实现
│   │   ├── aws/               # AWS 占位符
│   │   ├── huawei/            # 华为云占位符
│   │   ├── baidu/             # 百度云占位符
│   │   └── jdcloud/           # 京东云占位符
│   ├── services/        # 业务逻辑服务
│   │   ├── ip_detector.rs     # IP 检测服务
│   │   ├── scheduler.rs       # 调度器服务
│   │   └── dns_updater.rs     # DNS 更新服务
│   ├── storage/         # 数据层
│   │   ├── database.rs        # SQLite 数据库
│   │   └── secure_store.rs    # 安全凭证存储
│   ├── models/           # 数据模型
│   │   ├── domain.rs
│   │   ├── config.rs
│   │   └── credentials.rs
│   ├── app_state.rs     # 应用状态管理
│   ├── error.rs         # 错误类型定义
│   └── main.rs          # 入口文件
├── migrations/          # 数据库迁移
│   ├── 001_initial.sql
│   ├── 002_add_logs.sql
│   └── 003_add_settings.sql
├── Cargo.toml           # Rust 依赖
├── tauri.conf.json      # Tauri 配置
└── icons/               # 应用图标
```

**代码统计**: ~4,500 行 Rust

---

## 📁 Docker 配置 (docker/)

| 文件 | 说明 |
|------|------|
| [Dockerfile](docker/Dockerfile) | 多阶段构建配置 |
| [docker-compose.yml](docker/docker-compose.yml) | Docker Compose 编排 |
| [config.example.yml](docker/config.example.yml) | 配置文件示例 |
| [README.md](docker/README.md) | Docker 部署指南 |

---

## 📁 CI/CD 配置 (.github/workflows/)

| 文件 | 说明 |
|------|------|
| [ci.yml](.github/workflows/ci.yml) | 持续集成工作流 |
| [docker-publish.yml](.github/workflows/docker-publish.yml) | Docker 发布工作流 |
| [release.yml](.github/workflows/release.yml) | 跨平台构建和发布 |

---

## 📊 文件统计

### 按类型分类

| 类型 | 文件数 | 说明 |
|------|--------|------|
| **Markdown 文档** | 19 | .md 文件 |
| **配置文件** | 10 | .json, .toml, .yml 等 |
| **Shell 脚本** | 6 | .sh 文件 |
| **TypeScript 代码** | 18 | .ts, .tsx 文件 |
| **Rust 代码** | 25 | .rs 文件 |
| **SQL 迁移** | 3 | .sql 文件 |
| **其他** | 5 | 图标, 样式等 |
| **总计** | **86** | |

### 按大小分类

| 分类 | 文件数 | 代码行数 (估算) |
|------|--------|----------------|
| **文档** | 19 | ~4,000 |
| **前端代码** | 18 | ~2,500 |
| **后端代码** | 25 | ~4,500 |
| **配置** | 10 | ~500 |
| **脚本** | 6 | ~200 |
| **SQL** | 3 | ~100 |
| **总计** | **81** | **~11,800** |

---

## 🔍 快速导航

### 用户相关
- [README.md](README.md) - 从这里开始!
- [FAQ.md](FAQ.md) - 遇到问题?
- [docker/README.md](docker/README.md) - Docker 部署

### 开发相关
- [CONTRIBUTING.md](CONTRIBUTING.md) - 如何贡献
- [CLAUDE.md](CLAUDE.md) - 开发者指南
- [TESTING_GUIDE.md](TESTING_GUIDE.md) - 测试指南
- [scripts/](scripts/) - 开发脚本

### 项目管理
- [CHANGELOG.md](CHANGELOG.md) - 版本历史
- [RELEASE_CHECKLIST.md](RELEASE_CHECKLIST.md) - 发布检查
- [FINAL_REPORT.md](FINAL_REPORT.md) - 项目总结

### 技术文档
- [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) - 技术总结
- [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) - 项目结构
- [.env.example](.env.example) - 环境变量

---

## 📝 维护说明

### 文档更新优先级

1. **高优先级** (必须及时更新)
   - README.md
   - CHANGELOG.md
   - FAQ.md

2. **中优先级** (版本发布时更新)
   - RELEASE_CHECKLIST.md
   - FINAL_REPORT.md
   - .env.example

3. **低优先级** (重大变更时更新)
   - PROJECT_SUMMARY.md
   - TESTING_GUIDE.md
   - CONTRIBUTING.md

### 添加新文件时

1. 更新本文档 (FILES.md)
2. 更新 .gitignore (如需要)
3. 更新相关文档
4. 运行代码格式化

---

**文档版本**: v1.0
**最后更新**: 2025年1月6日
**维护者**: DDNS Tool 开发团队
