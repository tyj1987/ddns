# 常见问题解答 (FAQ)

本文档回答了 DDNS 工具使用中的常见问题。

---

## 📋 目录

- [安装问题](#安装问题)
- [配置问题](#配置问题)
- [使用问题](#使用问题)
- [DNS 提供商问题](#dns-提供商问题)
- [Docker 相关](#docker-相关)
- [错误排查](#错误排查)
- [高级问题](#高级问题)

---

## 安装问题

### Q1: Linux 上安装失败,提示缺少 webkit 库?

**错误信息:**
```
error: failed to run custom build command for `webkit2gtk-sys`
Package `webkit2gtk-4.1` was not found
```

**解决方案:**

Ubuntu/Debian:
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev
```

Fedora:
```bash
sudo dnf install webkit2gtk3-devel gcc openssl-devel \
  gtk3-devel libappindicator-gtk3-devel librsvg-devel
```

Arch Linux:
```bash
sudo pacman -S webkit2gtk base-devel openssl \
  gtk3 libappindicator-gtk3 librsvg
```

### Q2: macOS 上无法打开应用?

**可能原因:** 安全设置阻止了未签名应用

**解决方案:**
1. 右键点击应用 → "打开"
2. 或在终端运行:
```bash
xattr -cr /Applications/DDNS.app
```

### Q3: Windows 杀毒软件报警?

**原因:** 未签名的应用可能被误报

**解决方案:**
1. 添加到杀毒软件白名单
2. 或使用 Windows Defender 排除项

---

## 配置问题

### Q4: 如何获取 Cloudflare API Token?

**步骤:**

1. 登录 [Cloudflare Dashboard](https://dash.cloudflare.com/)
2. 点击右上角头像 → "My Profile"
3. 选择 "API Tokens" → "Create Token"
4. 使用模板 "Edit zone DNS"
5. 配置权限:
   - Zone → DNS → Edit
   - Zone → Zone → Read
   - 包含你的域名
6. 创建并复制 Token

### Q5: 如何获取阿里云 Access Key?

**步骤:**

1. 登录 [阿里云控制台](https://ram.console.aliyun.com/manage/ak)
2. 创建 AccessKey:
   - 推荐使用子账号 AccessKey
   - 或使用 RAM 角色授予最小权限
3. 权限要求:
   - `AliyunDNSFullAccess` (完整权限)
   - 或自定义策略: 只允许操作 DNS 记录
4. 复制 Access Key ID 和 Secret

**安全建议:** 不要使用主账号 Access Key!

### Q6: 如何获取腾讯云 API 密钥?

**步骤:**

1. 登录 [腾讯云访问管理](https://console.cloud.tencent.com/cam/capi)
2. 创建密钥或使用已有密钥
3. 权限要求:
   - `QcloudDNSPodFullAccess` (DNSPod 完整权限)
4. 复制 Secret ID 和 Secret Key

### Q7: 凭证存储在哪里?

**位置:**
- **macOS**: 系统密钥链 (Keychain)
- **Windows**: 凭证管理器 (Credential Manager)
- **Linux**: Secret Service (gnome-keyring / kwallet)
- **加密方式**: AES-256-GCM

**注意:** 凭证不在数据库中明文存储,也不在日志中显示。

---

## 使用问题

### Q8: 为什么 DNS 没有更新?

**可能原因:**

1. **IP 未变化**
   - 检查日志: `IP 未变化,跳过更新`
   - 这是正常的,避免不必要的 API 调用

2. **域名配置错误**
   - 检查域名拼写
   - 检查子域名是否正确
   - 确认 DNS 记录已存在于提供商

3. **凭证无效**
   - 检查 API Token/Key 是否正确
   - 检查凭证权限是否足够
   - 查看日志中的错误信息

4. **网络问题**
   - 检查网络连接
   - 检查防火墙设置

### Q9: 如何查看当前 IP?

**方法:**

1. **应用界面**: 首页 "IP 地址" 面板
2. **日志**: 查看最新的 IP 检测记录
3. **手动刷新**: 点击 "刷新 IP" 按钮

### Q10: 调度器多久检测一次 IP?

**默认:** 5 分钟 (300 秒)

**自定义:**
- 编辑域名 → "更新间隔"
- 范围: 30 秒 - 24 小时
- 每个域名独立设置

**注意:** 频繁检测可能触发 API 限流!

### Q11: 支持哪些 DNS 记录类型?

**当前支持:**
- ✅ **A 记录** (IPv4 地址) - 完整支持
- ⚠️ **AAAA 记录** (IPv6 地址) - 部分支持
- ⚠️ **CNAME 记录** - 部分支持

**计划支持:** MX, TXT, SRV 等

---

## DNS 提供商问题

### Q12: 支持哪些 DNS 提供商?

**已完整实现:**
- ✅ Cloudflare
- ✅ 阿里云
- ✅ 腾讯云 (DNSPod)

**计划实现:**
- ⏳ AWS Route53
- ⏳ 华为云
- ⏳ 百度云
- ⏳ 京东云

### Q13: Cloudflare 代理设置会影响 DDNS 吗?

**回答:** 是的,可能有影响

**说明:**
- 如果 DNS 记录启用了 Cloudflare 代理 (橙色云朵图标)
- 更新的是 Cloudflare 的 IP,不是源站 IP
- 建议对 DDNS 记录关闭代理 (仅 DNS)

### Q14: 阿里云/腾讯云 API 请求频率限制?

**限制:**

**阿里云:**
- 限制: 每秒 20 次
- 超过返回: `Throttling`

**腾讯云:**
- 限制: 每秒 20 次
- 超过返回: `RequestLimitExceeded`

**建议:**
- 更新间隔至少 60 秒
- 多个域名错开检测时间

---

## Docker 相关

### Q15: 如何在 Docker 中运行?

**快速开始:**

```bash
# 1. 构建镜像
docker build -f docker/Dockerfile -t ddns-tool .

# 2. 创建配置目录
mkdir -p docker/config

# 3. 复制配置文件
cp docker/config.example.yml docker/config/ddns.yml

# 4. 编辑配置
vim docker/config/ddns.yml

# 5. 运行容器
docker run -d \
  -v $(pwd)/docker/config:/config \
  -e DDNS_HEADLESS=true \
  --name ddns \
  ddns-tool
```

### Q16: Docker 容器重启后数据会丢失吗?

**回答:** 不会

**原因:**
- 配置文件通过卷挂载持久化
- 数据库文件在容器内,重启不会丢失
- 建议定期备份 `docker/config` 目录

### Q17: 如何查看 Docker 日志?

```bash
# 查看实时日志
docker logs -f ddns

# 查看最近 100 行
docker logs --tail 100 ddns

# 查看特定时间日志
docker logs --since 1h ddns
```

---

## 错误排查

### Q18: "无法检测 IP" 错误?

**排查步骤:**

1. **检查网络连接**
   ```bash
   ping -c 3 8.8.8.8
   ```

2. **手动测试 API**
   ```bash
   curl https://api.ipify.org
   ```

3. **检查代理设置**
   - 某些代理可能阻止 API 请求
   - 尝试关闭代理

4. **更改检测方法**
   - 设置 → IP 检测方法
   - 切换到 DNS 或接口模式

### Q19: "DNS 更新失败" 错误?

**常见原因:**

1. **凭证无效或过期**
   - 检查 API Token/Key
   - 重新添加域名

2. **记录不存在**
   - 先在 DNS 提供商控制台手动创建记录
   - 然后使用 DDNS 工具更新

3. **权限不足**
   - 检查 API Token 权限
   - 确保有 "编辑 DNS" 权限

4. **API 限流**
   - 降低更新频率
   - 等待一段时间后重试

### Q20: "数据库错误"?

**可能原因:**

1. **数据库文件损坏**
   ```bash
   # 重置数据库
   rm ~/.config/ddns/data.db
   # 应用会自动创建新数据库
   ```

2. **权限问题**
   ```bash
   # 检查权限
   ls -la ~/.config/ddns/
   ```

3. **多个实例冲突**
   ```bash
   # 确保只有一个实例运行
   pkill ddns
   ```

---

## 高级问题

### Q21: 如何实现开机自启动?

**Linux (systemd):**

```bash
# 1. 创建服务文件
sudo vim /etc/systemd/system/ddns.service
```

内容:
```ini
[Unit]
Description=DDNS Tool
After=network.target

[Service]
Type=simple
User=your-username
WorkingDirectory=/path/to/ddns
ExecStart=/path/to/ddns/src-tauri/target/release/ddns
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

启动:
```bash
sudo systemctl enable ddns
sudo systemctl start ddns
```

**macOS (launchd):**

创建 `~/Library/LaunchAgents/com.ddns.tool.plist`

**Windows:**
- 将快捷方式放入启动文件夹
- `Win + R` → `shell:startup`

### Q22: 如何监控 DDNS 状态?

**方法:**

1. **应用界面**: 查看调度器状态和最后更新时间
2. **日志**: 实时日志查看器
3. **脚本监控**:
   ```bash
   # 检查进程是否运行
   pgrep -f ddns

   # 检查日志中的错误
   tail -f ~/.config/ddns/logs/ddns.log | grep ERROR
   ```

### Q23: 如何批量导入域名?

**当前状态:** 暂不支持批量导入

**替代方案:**
- 手动逐个添加
- 使用配置文件 (Docker 模式)
- 或等待后续版本支持

### Q24: 数据库文件位置?

**位置:**

- **Linux**: `~/.config/ddns/data.db`
- **macOS**: `~/Library/Application Support/ddns/data.db`
- **Windows**: `%APPDATA%\ddns\data.db`

**备份:**
```bash
# 备份数据库
cp ~/.config/ddns/data.db ~/ddns-backup-$(date +%Y%m%d).db
```

### Q25: 如何完全卸载?

**步骤:**

1. **关闭应用**
2. **删除应用文件**
3. **删除数据**:
   ```bash
   # Linux/macOS
   rm -rf ~/.config/ddns

   # Windows
   rmdir %APPDATA%\ddns
   ```
4. **删除凭证**:
   - Linux/macOS: 从密钥链删除
   - Windows: 从凭证管理器删除

---

## 仍需帮助?

如果以上 FAQ 没有解决你的问题:

1. **查看文档**:
   - [README.md](README.md) - 快速开始
   - [TESTING_GUIDE.md](TESTING_GUIDE.md) - 测试指南
   - [docker/README.md](docker/README.md) - Docker 部署

2. **搜索 Issues**:
   - [GitHub Issues](https://github.com/yourusername/ddns-tool/issues)

3. **创建新 Issue**:
   - 描述问题
   - 附上日志
   - 说明环境 (OS, 版本)

4. **社区讨论**:
   - GitHub Discussions
   - 提交 Pull Request

---

**最后更新**: 2025年1月6日
