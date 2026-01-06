# 安全政策

## 🛡️ 安全承诺

DDNS 工具致力于保护用户的安全和隐私。我们重视社区的安全研究员和用户在帮助我们发现和修复安全问题方面的贡献。

## 📋 支持的版本

目前只对以下版本提供安全更新:

| 版本 | 支持状态 |
|------|----------|
| v0.1.0-alpha (当前) | ✅ 支持 |
| v0.0.x | ❌ 不支持 |

## 🔍 报告安全漏洞

如果你发现了安全漏洞,**请不要**公开创建 issue。

### 报告方式

请发送电子邮件至: [security@your-username.com](mailto:security@your-username.com)

邮件应包含:
- 漏洞描述
- 受影响的版本
- 复现步骤
- 潜在影响
- 如果可能,提供修复建议

### 响应承诺

- 我们会在 **48 小时内**确认收到报告
- 我们会在 **7 天内**评估漏洞的严重性
- 我们会在 **14 天内**发布修复或提供详细的修复计划
- 我们会与安全研究员保持沟通

### 期望行为

我们期望安全研究员:
- 给我们合理的时间修复问题后再公开披露
- 遵守负责任的披露原则
- 不利用漏洞访问、修改或删除用户数据
- 以测试漏洞为目的,仅进行必要的操作

作为回报,我们承诺:
- 认可你的贡献 (除非你要求匿名)
- 在我们的安全公告中感谢你

## 🔒 安全最佳实践

### 用户安全

1. **凭证管理**
   - ✅ 使用 API Token 而非主账号密钥
   - ✅ 定期轮换 API 凭证
   - ✅ 为 DDNS 服务创建最小权限的角色
   - ❌ 不要在日志或配置文件中暴露凭证

2. **网络安全**
   - ✅ 确保应用从官方来源下载
   - ✅ 保持应用更新到最新版本
   - ✅ 在防火墙后运行 (如可能)
   - ❌ 不要在公共网络暴露管理界面

3. **数据保护**
   - ✅ 定期备份数据库和配置
   - ✅ 使用系统密钥链存储凭证
   - ✅ 设置适当的文件权限

### 开发者安全

1. **依赖管理**
   - 定期更新依赖
   - 监控安全公告
   - 使用 Dependabot 自动检测

2. **代码审查**
   - 所有代码变更需要审查
   - 使用静态分析工具
   - 运行安全测试

3. **凭证处理**
   - 永远不要记录凭证
   - 永远不要在错误消息中暴露凭证
   - 使用环境变量或密钥管理系统

## 🔐 已知安全风险

### 凭证存储

**风险**: 凭证存储在系统密钥链中

**缓解措施**:
- 使用 AES-256-GCM 加密
- 凭证不在数据库明文存储
- 凭证不在日志中显示

**用户建议**:
- 使用受限的 API Token
- 定期轮换凭证
- 使用专门的 DDNS 账号

### 网络通信

**风险**: 应用通过 HTTP/HTTPS 与外部 API 通信

**缓解措施**:
- 强制使用 TLS/SSL
- 使用 rustls (无 OpenSSL 依赖)
- 验证 SSL 证书

**用户建议**:
- 不要修改 TLS 设置
- 检查网络连接安全性

### SQL 注入

**风险**: 用户输入可能用于数据库查询

**缓解措施**:
- 使用 SQLx 编译时查询检查
- 所有查询都是参数化的
- 输入验证和清理

**状态**: ✅ 已缓解

## 🛠️ 安全特性

DDNS 工具实现了以下安全特性:

1. **加密存储**
   - AES-256-GCM 加密凭证
   - 系统密钥链集成

2. **网络安全**
   - 强制 HTTPS/TLS
   - 证书验证
   - rustls 实现

3. **代码安全**
   - 类型安全 (Rust + TypeScript)
   - SQL 注入防护
   - 输入验证

4. **日志安全**
   - 凭证自动脱敏
   - 结构化日志
   - 安全的错误消息

## 📢 安全公告

安全公告会通过以下方式发布:

1. **GitHub Security Advisories** - 官方安全公告
2. **Release Notes** - 版本更新说明
3. **README.md** - 项目主页 (严重漏洞)

订阅方式:
- Watch GitHub repository (选择 "Releases only")
- 关注 GitHub Security Advisories

## 🔍 第三方审计

此项目目前**未经过**第三方安全审计。

如果你或你的公司愿意进行安全审计,请通过以下方式联系:
- Email: [security@your-username.com](mailto:security@your-username.com)
- GitHub Discussions

## 📧 安全相关问题

### 非漏洞安全问题

对于非安全漏洞的问题:
- 配置安全问题 → [GitHub Discussions](https://github.com/yourusername/ddns-tool/discussions)
- 文档安全问题 → 创建 [Documentation Issue](https://github.com/yourusername/ddns-tool/issues/new?template=documentation.md)

### 安全功能请求

如果你想要新的安全功能:
1. 创建 [Feature Request](https://github.com/yourusername/ddns-tool/issues/new?template=feature_request.md)
2. 在标题中使用 `[SECURITY]` 前缀
3. 详细说明安全用例和威胁模型

## 📚 资源

- [GitHub Security](https://github.com/security)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CVE Database](https://cve.mitre.org/)

---

**最后更新**: 2025年1月6日

**联系我们**: [security@your-username.com](mailto:security@your-username.com)

---

## ⏱️ 时间线

| 日期 | 事件 |
|------|------|
| 2025-01-06 | 安全政策创建 |

---

## 🙏 致谢

感谢所有帮助我们发现和修复安全问题的安全研究员和用户!

如果你发现了安全漏洞,请遵循上述报告方式。你的帮助对于保持 DDNS 工具的安全非常重要。
