# 变更日志

所有项目重要变更都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

### 计划中
- AWS Route53 DNS 提供商完整实现
- 华为云 DNS 提供商完整实现
- 百度云 DNS 提供商完整实现
- 京东云 DNS 提供商完整实现
- IPv6 完整支持
- CNAME 记录完整支持
- 通配符域名支持
- 系统托盘图标和菜单
- 桌面通知
- 开机自启动
- 深色主题
- 多语言支持

---

## [0.1.0-alpha] - 2025-01-06

### 新增
- 🎉 初始版本发布
- ✨ IP 自动检测功能
  - 支持多种检测方法 (API/DNS/网络接口)
  - 自动降级策略
  - IP 缓存机制 (60秒 TTL)
- ✨ DNS 提供商集成
  - Cloudflare 完整支持
  - 阿里云完整支持
  - 腾讯云 (DNSPod) 完整支持
- ✨ 域名管理功能
  - 添加/编辑/删除域名
  - 启用/禁用域名
  - 手动触发 DNS 更新
  - 查看更新历史
- ✨ 智能调度器
  - 每域名独立更新间隔
  - IP 变化检测
  - 自动 DNS 更新
  - 调度器状态监控
- ✨ 现代化用户界面
  - React 19 + TypeScript
  - TailwindCSS 样式
  - 实时日志查看器
  - 跨平台桌面应用
- ✨ 数据持久化
  - SQLite 数据库
  - 凭证安全存储 (系统密钥链)
  - AES-256-GCM 加密
- ✨ Docker 支持
  - 多阶段构建优化
  - Docker Compose 配置
  - 无头模式运行
- 📚 完整文档
  - README 使用指南
  - CONTRIBUTING 贡献指南
  - PROJECT_SUMMARY 技术总结
  - RELEASE_CHECKLIST 发布清单

### 技术栈
- **前端**: React 19 + TypeScript + Vite + TailwindCSS
- **后端**: Rust + Tauri 2.0 + Tokio
- **数据库**: SQLite + SQLx
- **安全**: tauri-plugin-stronghold

### 支持平台
- Linux
- macOS
- Windows
- Docker (Alpine Linux)

### 已知限制
- DNS 提供商仅完成 3/7 (Cloudflare、阿里云、腾讯云)
- 调度器状态在应用重启后不持久化
- 事件系统基础实现,未完全集成
- 缺少自动化测试

---

## [0.0.1-dev] - 2025-01-01

### 新增
- 项目初始化
- 基础架构搭建
- Tauri 2.0 + React 配置

---

[Unreleased]: https://github.com/yourusername/ddns-tool/compare/v0.1.0-alpha...HEAD
[0.1.0-alpha]: https://github.com/yourusername/ddns-tool/compare/v0.0.1-dev...v0.1.0-alpha
[0.0.1-dev]: https://github.com/yourusername/ddns-tool/releases/tag/v0.0.1-dev
