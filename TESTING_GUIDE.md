# 测试指南

本文档提供 DDNS 工具的完整测试指南。

## 📋 测试环境要求

### 开发环境测试

**必需软件:**
- Node.js 18+
- Rust 1.75+
- 包管理器: npm 或 yarn

**Linux GUI 测试额外需求:**
```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.1-dev build-essential \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev

# Fedora
sudo dnf install webkit2gtk3-devel gcc openssl-devel \
  gtk3-devel libappindicator-gtk3-devel librsvg2-devel

# Arch Linux
sudo pacman -S webkit2gtk base-devel openssl \
  gtk3 libappindicator-gtk3 librsvg
```

### Docker 测试环境

**必需软件:**
- Docker 20.10+
- Docker Compose 2.0+

---

## 🧪 测试流程

### 第一阶段: 前端测试

#### 1.1 前端开发服务器测试

```bash
# 启动前端开发服务器
npm run dev
```

**验证项:**
- [ ] 服务器在 http://localhost:5173 启动
- [ ] 页面加载无错误
- [ ] 所有组件正常渲染
- [ ] 热模块替换(HMR)工作正常

**预期结果:**
- 浏览器自动打开
- 看到 DDNS 工具主界面
- 控制台无错误

#### 1.2 TypeScript 类型检查

```bash
# 类型检查
npx tsc --noEmit
```

**验证项:**
- [ ] 无类型错误
- [ ] 所有导入正确解析

**预期结果:**
- 输出: `✓ Type checking completed`
- 无错误或警告

#### 1.3 前端构建测试

```bash
# 构建前端
npm run build
```

**验证项:**
- [ ] 构建成功
- [ ] 生成 dist 目录
- [ ] 构建产物大小合理 (< 500KB)

**预期结果:**
```
dist/index.html                  1.2 kB
dist/assets/index-xxx.css        15 kB
dist/assets/index-xxx.js        227 kB
```

---

### 第二阶段: 后端测试

#### 2.1 Rust 代码编译检查

**注意:** Linux 系统跳过 GUI 构建是正常的。

```bash
# 检查 Rust 代码 (不构建 GUI)
cd src-tauri

# 方式1: 如果有 GUI 库
cargo check

# 方式2: 跳过 GUI 相关检查 (仅验证逻辑)
cargo check --lib
```

**验证项:**
- [ ] 代码编译通过
- [ ] 无严重警告
- [ ] 所有依赖正确解析

#### 2.2 Rust 单元测试

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test ip_detector
cargo test database
cargo test providers
```

**验证项:**
- [ ] 所有测试通过
- [ ] 测试覆盖率合理

#### 2.3 后端代码格式化

```bash
# 格式化代码
cargo fmt

# 检查格式是否正确
cargo fmt -- --check
```

#### 2.4 Linter 检查

```bash
# 运行 clippy (需要 GUI 库)
cargo clippy

# 如果没有 GUI 库,跳过此步骤
```

---

### 第三阶段: 集成测试 (Tauri)

#### 3.1 Tauri 开发模式测试

**前提条件:** 已安装 GUI 库 (Linux) 或在 macOS/Windows 上

```bash
# 启动 Tauri 开发模式
npm run tauri dev
```

**验证项:**
- [ ] 应用窗口启动
- [ ] 前后端通信正常
- [ ] IPC 命令正常工作

#### 3.2 功能集成测试

**在运行的应用中执行以下测试:**

**测试 1: IP 检测**
1. 打开应用
2. 查看首页 "IP 地址" 面板
3. 点击 "刷新 IP" 按钮

验证:
- [ ] 显示当前公网 IP
- [ ] IPv4 地址格式正确 (如 192.168.1.1)
- [ ] 控制台日志显示检测方法
- [ ] 日志中无错误

**测试 2: 添加 Cloudflare 域名**
1. 点击 "域名管理" 标签
2. 点击 "添加域名" 按钮
3. 填写表单:
   - 域名提供商: `Cloudflare`
   - 域名: `example.com`
   - 子域名: `www`
   - 记录类型: `A`
   - API Token: `<your_cloudflare_token>`
4. 点击 "保存"

验证:
- [ ] 域名出现在列表中
- [ ] 显示完整域名 (www.example.com)
- [ ] 状态为 "已启用"
- [ ] 日志显示 "域名添加成功"

**测试 3: 手动 DNS 更新**
1. 在域名列表中找到刚添加的域名
2. 点击 "更新" 按钮
3. 观察日志输出

验证:
- [ ] 日志显示 "开始检测 IP"
- [ ] 日志显示 "IP 已变化" 或 "IP 未变化"
- [ ] 如果 IP 变化,显示 "DNS 更新成功"
- [ ] 域名卡片显示最后更新时间

**测试 4: 启动调度器**
1. 在页面顶部找到调度器状态
2. 点击 "启动" 按钮
3. 等待 30 秒观察自动更新

验证:
- [ ] 调度器状态变为 "运行中"
- [ ] 显示活动任务数 > 0
- [ ] 每隔设定间隔自动检测 IP
- [ ] 日志显示定时检测记录

**测试 5: 日志查看**
1. 点击 "日志查看器" 标签
2. 观察实时日志
3. 尝试更改日志级别筛选
4. 点击 "清空日志"

验证:
- [ ] 日志每 3 秒自动刷新
- [ ] 级别筛选正常工作
- [ ] 清空日志成功
- [ ] 日志显示时间戳、级别、消息

**测试 6: 域名编辑和删除**
1. 点击域名卡片上的 "编辑"
2. 修改更新间隔为 60 秒
3. 保存更改
4. 点击 "删除"

验证:
- [ ] 编辑弹窗显示当前配置
- [ ] 修改成功保存
- [ ] 删除确认对话框正常
- [ ] 域名从列表移除

**测试 7: 添加阿里云域名**
1. 重复测试 2 步骤
2. 填写:
   - 域名提供商: `阿里云`
   - 域名: `your-domain.com`
   - Access Key ID: `<your_aliyun_key>`
   - Access Key Secret: `<your_aliyun_secret>`

验证:
- [ ] 阿里云域名添加成功
- [ ] 手动更新 DNS 成功

**测试 8: 添加腾讯云域名**
1. 重复测试 2 步骤
2. 填写:
   - 域名提供商: `腾讯云`
   - 域名: `your-domain.com`
   - Secret ID: `<your_tencent_secret_id>`
   - Secret Key: `<your_tencent_secret_key>`

验证:
- [ ] 腾讯云域名添加成功
- [ ] 手动更新 DNS 成功

#### 3.3 错误处理测试

**测试 9: 无效凭证**
1. 添加 Cloudflare 域名
2. 输入错误的 API Token
3. 尝试手动更新

验证:
- [ ] 显示错误提示
- [ ] 日志记录错误信息
- [ ] 凭证未暴露在日志中

**测试 10: 网络错误**
1. 断开网络连接
2. 尝试检测 IP
3. 尝试 DNS 更新

验证:
- [ ] 显示网络错误提示
- [ ] 错误消息友好清晰
- [ ] 应用不崩溃

---

### 第四阶段: Docker 测试

#### 4.1 Docker 镜像构建

```bash
# 构建镜像
docker build -f docker/Dockerfile -t ddns-tool:dev .

# 查看镜像大小
docker images ddns-tool:dev
```

**验证项:**
- [ ] 构建成功
- [ ] 镜像大小合理 (< 100MB)
- [ ] 无构建错误

**预期结果:**
```
REPOSITORY   TAG          SIZE
ddns-tool    dev          85.3 MB
```

#### 4.2 Docker Compose 启动

```bash
# 创建配置目录
mkdir -p docker/config

# 复制配置文件
cp docker/config.example.yml docker/config/ddns.yml

# 编辑配置文件,添加真实凭证
vim docker/config/ddns.yml

# 启动服务
docker-compose -f docker/docker-compose.yml up -d

# 查看日志
docker-compose -f docker/docker-compose.yml logs -f
```

**验证项:**
- [ ] 容器成功启动
- [ ] 日志显示 "DDNS 工具启动"
- [ ] IP 检测成功
- [ ] DNS 自动更新工作

**预期日志输出:**
```
ddns-1  | [INFO] DDNS 工具启动
ddns-1  | [INFO] 检测到公网 IP: 1.2.3.4
ddns-1  | [INFO] 调度器已启动,管理 1 个任务
ddns-1  | [INFO] [example.com] IP 未变化,跳过更新
```

#### 4.3 Docker 健康检查

```bash
# 检查容器状态
docker-compose -f docker/docker-compose.yml ps

# 查看资源使用
docker stats ddns-1
```

**验证项:**
- [ ] 容器状态为 "Up"
- [ ] 内存使用合理 (< 100MB)
- [ ] CPU 使用正常

#### 4.4 配置文件测试

修改 `docker/config/ddns.yml`:
1. 添加新域名
2. 重启容器
3. 观察日志

```bash
# 重启容器
docker-compose -f docker/docker-compose.yml restart

# 查看日志
docker-compose -f docker/docker-compose.yml logs -f
```

**验证项:**
- [ ] 新域名被加载
- [ ] 配置生效
- [ ] 无配置错误

---

### 第五阶段: 跨平台构建测试

#### 5.1 Linux 构建

```bash
# 构建 Linux 二进制
npm run tauri build -- --target x86_64-unknown-linux-gnu

# 检查构建产物
ls -lh src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/
```

**验证项:**
- [ ] 生成 .deb 包
- [ ] 生成 AppImage
- [ ] 可以安装运行

#### 5.2 macOS 构建 (如果环境支持)

```bash
# 构建 macOS 应用
npm run tauri build -- --target x86_64-apple-darwin

# 检查构建产物
ls -lh src-tauri/target/x86_64-apple-darwin/release/bundle/
```

**验证项:**
- [ ] 生成 .dmg 文件
- [ ] 生成 .app 包
- [ ] 可以打开运行

#### 5.3 Windows 构建 (如果环境支持)

```bash
# 构建 Windows 应用
npm run tauri build -- --target x86_64-pc-windows-msvc

# 检查构建产物
ls -lh src-tauri/target/x86_64-pc-windows-msvc/release/bundle/
```

**验证项:**
- [ ] 生成 .msi 安装包
- [ ] 生成 .exe 可执行文件
- [ ] 可以安装运行

---

## 🔍 性能测试

### 测试 11: 内存使用监控

1. 启动应用
2. 添加 10 个域名
3. 启动调度器
4. 运行 30 分钟
5. 监控内存使用

**工具:**
- Linux: `htop` 或 `ps`
- macOS: Activity Monitor
- Windows: Task Manager

**预期结果:**
- 启动内存: < 150MB
- 稳定内存: < 200MB
- 无内存泄漏

### 测试 12: IP 检测性能

1. 手动触发 IP 检测 10 次
2. 记录每次响应时间

**预期结果:**
- API 方法: < 3 秒
- DNS 方法: < 2 秒
- 降级策略工作正常

### 测试 13: 并发更新测试

1. 添加 5 个不同提供商的域名
2. 同时触发手动更新
3. 观察更新过程

**预期结果:**
- 所有更新并发执行
- 无死锁
- 总完成时间 < 10 秒

---

## 🔒 安全测试

### 测试 14: 凭证安全

1. 添加域名并保存凭证
2. 打开日志文件
3. 搜索凭证关键字

**验证项:**
- [ ] 日志中无明文凭证
- [ ] 数据库中凭证已加密
- [ ] 错误消息不泄露凭证

### 测试 15: SQL 注入测试

1. 添加域名,名称填写: `test'; DROP TABLE domains; --`
2. 尝试保存

**验证项:**
- [ ] 输入被拒绝或正确转义
- [ ] 数据库未受影响
- [ ] 应用无崩溃

---

## 📊 测试结果记录

### 测试检查清单

使用以下清单记录测试结果:

```
[ ] P1-前端测试 (4/4 通过)
[ ] P2-后端测试 (4/4 通过)
[ ] P3-集成测试 (10/10 通过)
[ ] P4-Docker测试 (4/4 通过)
[ ] P5-跨平台测试 (1/3 通过)
[ ] 性能测试 (3/3 通过)
[ ] 安全测试 (2/2 通过)
```

### 发现的问题

记录测试中发现的问题:

| ID | 严重性 | 描述 | 状态 |
|----|--------|------|------|
|   |        |      |      |

---

## 🐛 已知问题

根据 [RELEASE_CHECKLIST.md](RELEASE_CHECKLIST.md) 中的已知限制:

1. **DNS 提供商**: 仅完成 3/7
2. **调度器持久化**: 应用重启后需手动重启
3. **事件系统**: 基础实现,未完全集成

---

## ✅ 测试完成标准

项目可以发布的标准:

- [ ] 所有 P1-P3 测试通过
- [ ] Docker 测试通过
- [ ] 至少一个平台构建成功
- [ ] 无严重 (P0/P1) bug
- [ ] 性能测试通过
- [ ] 安全测试通过

---

## 📝 测试报告模板

完成测试后,填写此报告:

```markdown
## 测试报告

**测试日期**: YYYY-MM-DD
**测试人员**: Your Name
**测试环境**: OS + Version
**应用版本**: v0.1.0-alpha

### 测试结果摘要
- 总测试数: 28
- 通过: XX
- 失败: XX
- 跳过: XX

### 失败测试详情
1. [测试名称]
   - 预期: ...
   - 实际: ...
   - 严重性: High/Medium/Low

### 建议和反馈
...
```

---

**最后更新**: 2025年1月6日
