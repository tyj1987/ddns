# DDNS 工具 - 快速参考卡片

<div align="center">

**快速开始指南**

[![Version](https://img.shields.io/badge/version-v0.1.0--alpha-blue)](https://github.com/yourusername/ddns-tool)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## 📦 安装

```bash
# 克隆仓库
git clone https://github.com/yourusername/ddns-tool.git
cd ddns-tool

# 安装依赖
npm install

# 启动开发模式
npm run tauri dev
```

---

## 🚀 快速配置

### 1. Cloudflare

```yaml
提供商: Cloudflare
API Token: your_cloudflare_api_token
域名: example.com
子域名: www
```

获取 Token: [Cloudflare Dashboard](https://dash.cloudflare.com/profile/api-tokens)

### 2. 阿里云

```yaml
提供商: 阿里云
Access Key ID: your_access_key_id
Access Key Secret: your_access_key_secret
域名: example.com
```

获取 Key: [阿里云 RAM](https://ram.console.aliyun.com/manage/ak)

### 3. 腾讯云

```yaml
提供商: 腾讯云
Secret ID: your_secret_id
Secret Key: your_secret_key
域名: example.com
```

获取密钥: [腾讯云 CAM](https://console.cloud.tencent.com/cam/capi)

---

## 🔧 常用命令

### 开发

```bash
npm run dev              # 前端开发服务器
npm run tauri:dev        # Tauri 开发模式
npm run build            # 构建前端
npm run tauri:build      # 构建桌面应用
```

### 代码质量

```bash
npm run format           # 格式化代码
npm run lint             # 类型检查
npm run check            # 完整检查
```

### 开发脚本

```bash
./scripts/dev.sh         # 启动开发模式
./scripts/build.sh       # 构建项目
./scripts/check.sh       # 检查代码
./scripts/test.sh        # 运行测试
./scripts/format.sh      # 格式化代码
./scripts/clean.sh       # 清理构建产物
```

---

## 🐳 Docker 部署

```bash
# 构建镜像
docker build -f docker/Dockerfile -t ddns-tool .

# 运行容器
docker run -d \
  -v $(pwd)/docker/config:/config \
  -e DDNS_HEADLESS=true \
  --name ddns \
  ddns-tool

# 查看日志
docker logs -f ddns
```

或使用 Docker Compose:

```bash
docker-compose -f docker/docker-compose.yml up -d
```

---

## 📁 项目结构

```
ddns/
├── src/                    # React 前端
├── src-tauri/              # Rust 后端
├── docker/                 # Docker 配置
├── scripts/                # 开发脚本
├── docs/                   # 文档
└── README.md              # 主文档
```

---

## 📚 重要文档

| 文档 | 说明 |
|------|------|
| [README.md](README.md) | 项目主页 |
| [FAQ.md](FAQ.md) | 常见问题 |
| [CHANGELOG.md](CHANGELOG.md) | 版本变更 |
| [CONTRIBUTING.md](CONTRIBUTING.md) | 贡献指南 |
| [docker/README.md](docker/README.md) | Docker 指南 |

---

## 🔍 故障排查

### 问题: IP 检测失败

**解决方法:**
1. 检查网络连接
2. 更换检测方法 (API → DNS → Interface)
3. 检查代理设置

### 问题: DNS 更新失败

**解决方法:**
1. 验证 API 凭证
2. 检查 DNS 记录是否存在
3. 查看日志中的具体错误

### 问题: Linux GUI 错误

**解决方法:**
```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential
```

---

## 🎯 配置示例

### 环境变量 (.env)

```bash
# 日志级别
RUST_LOG=info

# IP 检测方法
IP_DETECTION_METHOD=api

# 默认更新间隔 (秒)
DEFAULT_UPDATE_INTERVAL=300

# 无头模式
DDNS_HEADLESS=false
```

### Docker 配置 (config.yml)

```yaml
domains:
  - name: example.com
    subdomain: www
    provider: cloudflare
    enabled: true
    interval: 300
    credentials:
      token: your_api_token

scheduler:
  enabled: true

ip_detection:
  method: api
  cache_ttl: 60
```

---

## 📊 监控

### 查看状态

- **应用界面**: 查看调度器状态和 IP
- **日志**: 实时日志查看器
- **Docker**: `docker logs -f ddns`

### 健康检查

```bash
# 检查进程
pgrep -f ddns

# 检查日志
tail -f ~/.config/ddns/logs/ddns.log | grep ERROR

# Docker 状态
docker ps | grep ddns
```

---

## 🔗 快速链接

- 📖 [完整文档](README.md)
- 🐛 [报告问题](https://github.com/yourusername/ddns-tool/issues)
- 💡 [功能建议](https://github.com/yourusername/ddns-tool/discussions)
- 📝 [更新日志](CHANGELOG.md)
- 🔐 [安全政策](SECURITY.md)

---

## 🆘 获取帮助

1. 查看 [FAQ.md](FAQ.md)
2. 搜索 [Issues](https://github.com/yourusername/ddns-tool/issues)
3. 加入 [Discussions](https://github.com/yourusername/ddns-tool/discussions)
4. 创建新 Issue

---

<div align="center">

**⭐ 如果这个项目对您有帮助,请给一个 Star! ⭐**

Made with ❤️ by DDNS Tool Contributors

</div>
