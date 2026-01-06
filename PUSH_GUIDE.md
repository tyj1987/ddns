# 🚀 GitHub 推送完整指南

## 📋 当前状态

✅ Git 仓库已初始化
✅ 所有文件已提交 (97 个文件, 16,525 行代码)
✅ 远程仓库已配置: `git@github.com:tyj1987/ddns.git`
⏳ 等待推送到 GitHub

---

## 🔑 推送方式 (选择一种)

### 方式 1: 使用 SSH 密钥 (推荐)

#### 步骤 1: 生成 SSH 密钥

```bash
# 生成新的 SSH 密钥
ssh-keygen -t ed25519 -C "tuoyongjun1987@qq.com"

# 按提示操作:
# 1. 保存位置: 默认 (~/.ssh/id_ed25519)
# 2. 密码: 可选,直接回车跳过
```

#### 步骤 2: 启动 ssh-agent 并添加密钥

```bash
# 启动 ssh-agent
eval "$(ssh-agent -s)"

# 添加私钥
ssh-add ~/.ssh/id_ed25519
```

#### 步骤 3: 添加公钥到 GitHub

```bash
# 复制公钥内容
cat ~/.ssh/id_ed25519.pub
```

然后:
1. 访问: https://github.com/settings/keys
2. 点击 **"New SSH key"**
3. Title: `DDNS Tool Development`
4. Key: 粘贴刚才复制的公钥内容
5. 点击 **"Add SSH key"**

#### 步骤 4: 测试连接

```bash
ssh -T git@github.com
```

如果看到 `Hi tyj1987! You've successfully authenticated...` 则成功!

#### 步骤 5: 推送代码

```bash
cd /home/tyj/ddns
git push -u origin main
```

---

### 方式 2: 使用 Personal Access Token

#### 步骤 1: 创建 Token

1. 访问: https://github.com/settings/tokens
2. 点击 **"Generate new token"** → **"Generate new token (classic)"**
3. 设置:
   - Note: `DDNS Tool Development`
   - Expiration: 选择过期时间 (建议 90 天)
   - 勾选权限:
     - ✅ repo (完整仓库访问权限)
     - ✅ workflow (GitHub Actions)
4. 点击 **"Generate token"**
5. **重要**: 复制 token (只显示一次!)

#### 步骤 2: 切换远程 URL 为 HTTPS

```bash
cd /home/tyj/ddns
git remote set-url origin https://github.com/tyj1987/ddns.git
```

#### 步骤 3: 推送 (使用 Token)

```bash
git push -u origin main
```

当提示输入用户名和密码时:
- **Username**: `tyj1987`
- **Password**: 粘贴刚才的 Token (不是你的 GitHub 密码!)

---

### 方式 3: 使用 GitHub CLI (gh)

#### 步骤 1: 安装 GitHub CLI

```bash
# Ubuntu/Debian
sudo apt install gh

# macOS
brew install gh

# 或访问: https://github.com/cli/cli#installation
```

#### 步骤 2: 登录

```bash
gh auth login
```

按提示操作:
1. 选择 `GitHub.com`
2. 选择 `HTTPS`
3. 选择 `Yes` (登录)
4. 按回车打开浏览器或输入一次性代码

#### 步骤 3: 推送

```bash
cd /home/tyj/ddns
git push -u origin main
```

---

## ✅ 推送成功后

### 1. 验证仓库

访问: https://github.com/tyj1987/ddns

你应该看到:
- ✅ 所有文件已上传
- ✅ README.md 显示在首页
- ✅ 97 个文件, 16,525+ 行代码

### 2. 自动触发 CI/CD

GitHub Actions 会自动运行:

#### CI 工作流 (.github/workflows/ci.yml)
- ✅ TypeScript 类型检查
- ✅ Rust 代码检查
- ✅ Docker 构建测试

查看: https://github.com/tyj1987/ddns/actions

### 3. 创建 Release

#### 选项 A: 通过 GitHub UI

1. 访问: https://github.com/tyj1987/ddns/releases/new
2. 标签: `v0.1.0-alpha`
3. 标题: `v0.1.0-alpha - 初始发布`
4. 描述: 复制 [CHANGELOG.md](CHANGELOG.md) 的内容
5. 勾选 **"Set as the latest release"**
6. 点击 **"Publish release"**

这会自动触发:
- ✅ 跨平台构建 (Windows, macOS, Linux)
- ✅ Docker 镜像构建和发布

#### 选项 B: 通过 Git 命令

```bash
cd /home/tyj/ddns

# 创建标签
git tag -a v0.1.0-alpha -m "v0.1.0-alpha - 初始发布"

# 推送标签
git push origin v0.1.0-alpha
```

然后访问上面的链接完成 Release。

---

## 🎯 推送后检查清单

- [ ] 代码已推送到 GitHub
- [ ] README.md 正确显示
- [ ] CI 工作流成功运行
- [ ] 创建了第一个 Release
- [ ] 跨平台构建成功
- [ ] Docker 镜像发布成功

---

## 🔧 故障排查

### 问题 1: Permission denied (publickey)

**原因**: SSH 密钥未配置或未添加到 GitHub

**解决**: 使用"方式 1"配置 SSH 密钥

### 问题 2: Authentication failed

**原因**: Token 过期或无效

**解决**: 使用"方式 2"创建新的 Token

### 问题 3: 推送失败 - remote rejected

**原因**: 可能需要先在 GitHub 上创建仓库

**解决**:
1. 访问: https://github.com/new
2. 仓库名: `ddns`
3. 设为 Public 或 Private
4. **不要**勾选 "Add a README file"
5. 点击 "Create repository"
6. 重新执行推送命令

### 问题 4: 连接超时

**原因**: 网络问题或需要代理

**解决**:
```bash
# 配置代理 (如果使用)
git config --global http.proxy http://127.0.0.1:7890
git config --global https.proxy http://127.0.0.1:7890
```

---

## 📊 推送后项目统计

推送成功后,你的仓库将包含:

| 项目 | 数量 |
|------|------|
| 文件 | 97 个 |
| 代码 | 16,525+ 行 |
| 文档 | 22 个 |
| 开发脚本 | 6 个 |
| CI/CD 工作流 | 3 个 |
| Issue 模板 | 3 个 |
| PR 模板 | 1 个 |

---

## 🎉 下一步

推送成功后:

1. **设置仓库描述**:
   - 访问: https://github.com/tyj1987/ddns
   - 点击 ⚙️ Settings
   - Description: `跨平台 DDNS 自动更新工具 (Tauri 2.0 + Rust + React)`

2. **添加主题标签**:
   - 在 Settings 页面
   - Topics: `ddns`, `tauri`, `rust`, `react`, `dns`, `cloudflare`, `aliyun`

3. **启用功能**:
   - ✅ Issues
   - ✅ Discussions
   - ✅ Actions
   - ✅ Wiki (可选)
   - ✅ Pages (可选)

4. **分享项目**:
   - 在社交媒体分享
   - 提交到中文 Rust 社区
   - 提交到 V2EX

---

**准备好了吗? 选择一种方式开始推送吧!** 🚀

如果遇到问题,可以查看:
- [GitHub SSH 文档](https://docs.github.com/zh/authentication/connecting-to-github-with-ssh)
- [GitHub Personal Access Tokens](https://docs.github.com/zh/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token)
