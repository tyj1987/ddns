# ✅ GitHub 仓库设置清单

推送代码后,按照此清单完成 GitHub 仓库的配置。

---

## 📋 推送前检查

### 1. 确认 GitHub 仓库已创建

访问: https://github.com/new

**创建仓库**:
- 仓库所有者: `tyj1987`
- 仓库名称: `ddns`
- 描述: `跨平台 DDNS 自动更新工具 (Tauri 2.0 + Rust + React)`
- 可见性: Public (推荐) 或 Private
- **不要**勾选:
  - ❌ Add a README file
  - ❌ Add .gitignore
  - ❌ Choose a license

点击 **"Create repository"**

---

## 🚀 推送代码

### 方式 1: 使用推送脚本

```bash
cd /home/tyj/ddns
./scripts/simple-push.sh
```

### 方式 2: 手动推送

```bash
cd /home/tyj/ddns

# 如果还未初始化,先初始化
git init
git config user.name "tyj1987"
git config user.email "tuoyongjun1987@qq.com"
git branch -m main

# 添加远程仓库
git remote add origin git@github.com:tyj1987/ddns.git

# 添加所有文件
git add .

# 提交
git commit -m "feat: 初始化 DDNS 工具项目 v0.1.0-alpha"

# 推送
git push -u origin main
```

**如果推送失败**,参考 [PUSH_GUIDE.md](PUSH_GUIDE.md)

---

## ✅ 推送后设置

### 1. 验证推送成功

访问: https://github.com/tyj1987/ddns

应该看到:
- ✅ README.md 在首页显示
- ✅ 所有文件已上传 (97 个文件)
- ✅ 代码行数: 16,525+

### 2. 仓库基本设置

访问: https://github.com/tyj1987/ddns/settings

**General**:
- ✅ Repository name: `ddns`
- ✅ Description: `跨平台 DDNS 自动更新工具 (Tauri 2.0 + Rust + React)`

**Topics (主题标签)**:
添加以下标签:
- `ddns`
- `tauri`
- `rust`
- `react`
- `typescript`
- `dns`
- `cloudflare`
- `aliyun`
- `tencent`
- `docker`
- `cross-platform`

**Features**:
- ✅ Issues (启用)
- ✅ Discussions (启用)
- ✅ Actions (启用)
- ✅ Projects (可选)
- ✅ Wiki (可选)
- ✅ Pages (可选)

### 3. 安全设置

访问: https://github.com/tyj1987/ddns/settings/security

**Security**:
- ✅ Dependabot alerts (启用)
- ✅ Dependabot security updates (启用)
- ✅ Secret scanning (启用)

### 4. 分支保护

访问: https://github.com/tyj1987/ddns/settings/branches

**Branch protection rule** for `main`:
- ✅ Require a pull request before merging
  - Require approvals: 1
- ✅ Require status checks to pass before merging
  - Require branches to be up to date before merging
- ✅ Do not allow bypassing the above settings

(可选,适用于多人协作项目)

---

## 🏷️  创建 Release

### 方式 1: GitHub UI

1. 访问: https://github.com/tyj1987/ddns/releases/new
2. 填写:
   - **Choose a tag**: `v0.1.0-alpha`
   - **Target**: `main`
   - **Release title**: `v0.1.0-alpha - 初始发布`
   - **Description**: 复制以下内容

```markdown
## 🎉 DDNS 工具 v0.1.0-alpha

这是 DDNS 工具的首次公开 Alpha 版本!

### ✨ 主要特性

- 🌍 **多云支持**: Cloudflare, 阿里云, 腾讯云
- 🔄 **自动更新**: 智能 IP 检测和 DNS 更新
- 🔒 **安全可靠**: AES-256-GCM 加密存储
- 🎨 **现代界面**: React 19 + Tauri 2.0
- 🐳 **Docker 支持**: 轻量级容器部署

### 📦 安装

#### 桌面应用
下载对应平台的安装包 (见下方 Assets)

#### Docker
```bash
docker pull ghcr.io/tyj1987/ddns:v0.1.0-alpha
```

### 📚 文档

- [快速开始](https://github.com/tyj1987/ddns/blob/main/GETTING_STARTED.md)
- [完整文档](https://github.com/tyj1987/ddns/blob/main/README.md)
- [FAQ](https://github.com/tyj1987/ddns/blob/main/FAQ.md)

### 🙏 致谢

感谢所有贡献者和用户的支持!

### 📝 完整变更日志

参见 [CHANGELOG.md](https://github.com/tyj1987/ddns/blob/main/CHANGELOG.md)
```

3. 勾选:
   - ✅ Set as the latest release
   - ✅ Set as a pre-release (因为是 Alpha 版本)
4. 点击 **"Publish release"**

### 方式 2: Git 命令

```bash
cd /home/tyj/ddns

# 创建标签
git tag -a v0.1.0-alpha -m "v0.1.0-alpha - 初始发布"

# 推送标签
git push origin v0.1.0-alpha
```

然后访问 GitHub 完成 Release 描述。

---

## 🤖 验证 CI/CD

### 1. 检查 Actions 工作流

访问: https://github.com/tyj1987/ddns/actions

应该看到:
- ✅ **CI** 工作流运行中
  - TypeScript 类型检查
  - Rust 代码检查
  - Docker 构建测试

### 2. 检查 Release 构建状态

创建 Release 后,会触发 **Release** 工作流:
- ✅ macOS 构建 (.dmg, .app)
- ✅ Linux 构建 (.deb, AppImage)
- ✅ Windows 构建 (.msi, .exe)
- ✅ Docker 镜像发布

### 3. Docker 镜像

访问: https://github.com/tyj1987/ddns/pkgs/container/ddns

应该看到 Docker 镜像已发布。

---

## 📊 推送后检查清单

### 仓库内容

- [ ] README.md 正确显示
- [ ] 所有文件已上传 (97 个)
- [ ] LICENSE 显示 MIT License
- [ ] Contributing 显示贡献指南

### CI/CD

- [ ] CI 工作流成功运行
- [ ] Docker 构建测试通过
- [ ] Release 创建成功
- [ ] 跨平台构建成功

### 设置

- [ ] 仓库描述已添加
- [ ] 主题标签已添加
- [ ] Security 设置已启用
- [ ] Dependabot 已启用

### 文档

- [ ] 所有文档链接正确
- [ ] 快速启动指南可访问
- [ ] FAQ 完整显示

---

## 🎉 完成后

### 1. 设置仓库 Stars

告诉朋友和社区为你的项目点 Star!

### 2. 分享项目

- [ ] Twitter / 微博
- [ ] Reddit (r/selfhosted, r/rust)
- [ ] V2EX
- [ ] 中文 Rust 社区
- [ ] GitHub Trending (可能!)

### 3. 监控 Issues

定期检查:
- https://github.com/tyj1987/ddns/issues
- https://github.com/tyj1987/ddns/discussions

### 4. 准备下一个版本

根据反馈开始规划 v0.2.0!

---

## 🔗 快速链接

- 仓库: https://github.com/tyj1987/ddns
- Issues: https://github.com/tyj1987/ddns/issues
- Discussions: https://github.com/tyj1987/ddns/discussions
- Actions: https://github.com/tyj1987/ddns/actions
- Releases: https://github.com/tyj1987/ddns/releases
- Settings: https://github.com/tyj1987/ddns/settings

---

**祝你的项目成功! 🎉**
