# DDNS 工具

<div align="center">

一个现代化的跨平台动态 DNS 更新工具

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app/)
[![React](https://img.shields.io/badge/React-19-cyan.svg)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.6-blue.svg)](https://www.typescriptlang.org/)
[![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/yourusername/ddns-tool)
[![Code Quality](https://img.shields.io/badge/code%20quality-A-brightgreen.svg)](https://github.com/yourusername/ddns-tool)

**支持平台**: Windows | macOS | Linux | Docker

[English](./README_EN.md) | 简体中文

[功能特性](#-特性) • [快速开始](#-快速开始) • [使用指南](#-使用指南) • [配置说明](#-配置说明) • [部署](#-部署) • [常见问题](#-常见问题)

</div>


---

## ✨ 特性

- 🌍 **多云支持** - 支持 Cloudflare、阿里云、腾讯云、AWS Route53 等主流 DNS 提供商
- 🔄 **自动更新** - IP 变化时自动更新 DNS 记录,支持自定义更新间隔
- 🔒 **安全可靠** - 使用系统密钥链加密存储 API 凭证
- 🎨 **现代界面** - 基于 Tauri 2.0 + React 19 构建的原生桌面应用
- 📊 **实时监控** - 实时显示 IP 地址、更新历史和系统日志
- 🐳 **Docker 支持** - 提供完整的 Docker 镜像,支持服务器部署
- ⚡ **高性能** - 使用 Rust 和 Tokio 异步运行时,低资源占用

## 🚀 快速开始

### 前置要求

- **Node.js** >= 18.0.0
- **Rust** >= 1.70
- **系统依赖**:
  - Linux:
    ```bash
    # Ubuntu/Debian
    sudo apt-get install libwebkit2gtk-4.1-dev build-essential libayatana-appindicator3-dev

    # Fedora/RHEL
    sudo dnf install webkit2gtk4.1-devel libappindicator-gtk3-devel

    # Arch Linux
    sudo pacman -S webkit2gtk-4.1 libappindicator-gtk3
    ```
  - macOS: Xcode 命令行工具
  - Windows: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

### 安装

```bash
# 克隆仓库
git clone https://github.com/tyj1987/ddns.git
cd ddns

# 安装依赖
npm install

# 开发模式运行
npm run tauri dev

# 生产构建
npm run tauri build
```

### Docker 部署

```bash
# 使用 Docker Compose
cd docker
cp config.example.yml config/ddns.yml
# 编辑配置文件
docker-compose up -d
```

详细部署说明请参考 [Docker 部署指南](./docker/README.md)

## 📖 使用指南

### 1. 添加域名

1. 打开应用,切换到"域名管理"标签
2. 点击"添加域名"按钮
3. 填写域名信息:
   - **域名**: example.com
   - **子域名**: www
   - **DNS 提供商**: Cloudflare
   - **记录类型**: A (IPv4)
   - **更新间隔**: 300 秒
4. 填写提供商凭证:
   - Cloudflare: API Token
   - 阿里云: Access Key ID + Secret
5. 点击"添加"

### 2. 配置提供商

#### Cloudflare

1. 登录 [Cloudflare Dashboard](https://dash.cloudflare.com/)
2. 进入 "My Profile" → "API Tokens"
3. 创建具有 "Edit DNS" 权限的 Token

#### 阿里云

1. 登录 [阿里云控制台](https://ram.console.aliyun.com/)
2. 创建 RAM 用户并授权 "AliyunDNSFullAccess"
3. 创建 AccessKey

#### 腾讯云

1. 登录 [腾讯云控制台](https://console.cloud.tencent.com/cam)
2. 创建 API 密钥
3. 授权 DNSPod 权限

### 3. 启动调度器

添加域名后,调度器会自动开始运行。您也可以:

- **手动更新**: 点击域名列表中的"立即更新"按钮
- **启用/禁用**: 通过域名操作菜单切换状态
- **查看日志**: 切换到"日志"标签查看实时日志

## 🔧 配置说明

### IP 检测方法

应用支持多种 IP 检测方法(自动降级):

1. **API 检测** (推荐) - 通过第三方服务获取公网 IP
2. **DNS 检测** - 查询 OpenDNS 的 myip.opendns.com
3. **网络接口** - 直接读取本地网络接口

### 更新间隔

- **最小**: 30 秒
- **最大**: 24 小时
- **推荐**: 5-10 分钟

> 注意: 过于频繁的更新可能触发 DNS 提供商的速率限制

### 日志级别

- **Error**: 仅错误
- **Warn**: 警告及以上
- **Info**: 信息及以上 (推荐)
- **Debug**: 调试 (详细日志)

## 🏗️ 技术架构

### 技术栈

- **前端**:
  - [React 19](https://react.dev/) + [TypeScript](https://www.typescriptlang.org/)
  - [Vite](https://vitejs.dev/) - 构建工具
  - [TailwindCSS](https://tailwindcss.com/) - UI 框架
  - [Tauri 2.0](https://tauri.app/) - 桌面应用框架

- **后端**:
  - [Rust](https://www.rust-lang.org/) - 系统编程语言
  - [Tokio](https://tokio.rs/) - 异步运行时
  - [SQLx](https://github.com/launchbadge/sqlx) - 数据库 ORM
  - [Reqwest](https://docs.rs/reqwest/) - HTTP 客户端

### 项目结构

```
ddns-tool/
├── src/                    # React 前端源码
│   ├── components/         # UI 组件
│   ├── lib/               # API 客户端
│   └── types/             # TypeScript 类型
├── src-tauri/             # Rust 后端源码
│   ├── src/
│   │   ├── commands/      # Tauri 命令
│   │   ├── services/      # 业务逻辑
│   │   ├── providers/     # DNS 提供商实现
│   │   ├── models/        # 数据模型
│   │   ├── storage/       # 数据库和存储
│   │   └── error.rs       # 错误处理
│   └── migrations/        # SQL 迁移
├── docker/                # Docker 配置
├── CLAUDE.md             # 项目指南
└── README.md             # 本文件
```

### 设计模式

- **Trait 抽象**: 统一的 DNS 提供商接口
- **工厂模式**: 动态创建提供商实例
- **依赖注入**: 使用 Tauri State 管理应用状态
- **异步任务**: 基于 Tokio 的并发调度

## 🤝 贡献

欢迎贡献! 请查看 [CONTRIBUTING.md](./CONTRIBUTING.md)

### 开发指南

```bash
# 安装依赖
npm install

# 使用开发脚本
./scripts/dev.sh        # 启动开发模式
./scripts/build.sh      # 构建项目
./scripts/check.sh      # 检查代码
./scripts/test.sh       # 运行测试
./scripts/format.sh     # 格式化代码

# 或使用 npm scripts
npm run dev             # 前端开发服务器
npm run build           # 构建前端
npm run tauri:dev       # Tauri 开发模式
npm run tauri:build     # 构建桌面应用
```

## 📚 文档

- [README](README.md) - 项目介绍
- [CONTRIBUTING.md](CONTRIBUTING.md) - 贡献指南
- [FAQ.md](FAQ.md) - 常见问题
- [TESTING_GUIDE.md](TESTING_GUIDE.md) - 测试指南
- [CHANGELOG.md](CHANGELOG.md) - 版本变更
- [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) - 技术总结
- [docker/README.md](docker/README.md) - Docker 部署
- [CLAUDE.md](CLAUDE.md) - 开发者指南

## 🔗 相关链接

- **问题反馈**: [GitHub Issues](https://github.com/yourusername/ddns-tool/issues)
- **功能建议**: [GitHub Discussions](https://github.com/yourusername/ddns-tool/discussions)
- **更新日志**: [CHANGELOG.md](CHANGELOG.md)
- **Docker Hub**: [ddns-tool](https://hub.docker.com/r/yourusername/ddns-tool)

## 📊 项目状态

[![项目完成度](https://progress-bar.dev/92?title=完成度)](https://github.com/yourusername/ddns-tool/projects)
[![代码质量](https://progress-bar.dev/95?title=代码质量)](https://github.com/yourusername/ddns-tool)
[![文档完整度](https://progress-bar.dev/100?title=文档)](https://github.com/yourusername/ddns-tool)

**当前版本**: v0.1.0-alpha | **最新发布**: [2025-01-06](https://github.com/yourusername/ddns-tool/releases)

## 🛣️ 路线图

### v0.1.0-alpha (当前)
- ✅ 核心功能 (IP 检测, DNS 更新, 调度器)
- ✅ 3个 DNS 提供商 (Cloudflare, 阿里云, 腾讯云)
- ✅ Docker 支持
- ✅ 完整文档

### v0.2.0 (计划中)
- ⏳ 更多 DNS 提供商 (AWS, 华为云, 百度云)
- ⏳ 自动化测试
- ⏳ 调度器状态持久化
- ⏳ CI/CD 完善

### v0.3.0 (计划中)
- ⏳ 系统托盘集成
- ⏳ 桌面通知
- ⏳ 深色主题

## 📝 许可证

[MIT License](./LICENSE) © 2025 DDNS Tool Contributors

## 🙏 致谢

本项目基于以下优秀的开源技术:

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [React](https://react.dev/) - UI 库
- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [TailwindCSS](https://tailwindcss.com/) - CSS 框架
- [SQLx](https://github.com/launchbadge/sqlx) - 数据库库
- [Tokio](https://tokio.rs/) - 异步运行时

感谢所有贡献者的支持!

---

<div align="center">

**⭐ 如果这个项目对您有帮助,请给一个 Star! ⭐**

**💬 欢迎加入讨论和贡献代码!**

Made with ❤️ by [DDNS Tool Contributors](https://github.com/yourusername/ddns-tool/graphs/contributors)

[![GitHub stars](https://img.shields.io/github/stars/yourusername/ddns-tool?style=social)](https://github.com/yourusername/ddns-tool/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/yourusername/ddns-tool?style=social)](https://github.com/yourusername/ddns-tool/network/members)
[![GitHub issues](https://img.shields.io/github/issues/yourusername/ddns-tool)](https://github.com/yourusername/ddns-tool/issues)

</div>
