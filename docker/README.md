# DDNS 工具 Docker 部署指南

## 快速开始

### 1. 使用 Docker Compose (推荐)

```bash
# 克隆项目
git clone <repository-url>
cd ddns

# 创建配置目录
mkdir -p docker/config

# 复制示例配置
cp docker/config.example.yml docker/config/ddns.yml

# 编辑配置文件,添加您的域名和凭证
nano docker/config/ddns.yml

# 启动服务
cd docker
docker-compose up -d

# 查看日志
docker-compose logs -f

# 停止服务
docker-compose down
```

### 2. 使用 Docker 命令

```bash
# 构建镜像
docker build -f docker/Dockerfile -t ddns-tool .

# 运行容器
docker run -d \
  --name ddns-tool \
  --restart unless-stopped \
  -v $(pwd)/docker/config:/config \
  -e RUST_LOG=info \
  ddns-tool

# 查看日志
docker logs -f ddns-tool

# 停止容器
docker stop ddns-tool

# 删除容器
docker rm ddns-tool
```

## 配置说明

### 环境变量

- `DDNS_CONFIG_PATH`: 配置文件路径(默认: `/config`)
- `DDNS_HEADLESS`: 无头模式运行(默认: `true`)
- `RUST_LOG`: 日志级别(默认: `info`)

### 卷挂载

- `/config`: 配置文件目录(必需)
- `/app/data`: 数据库文件目录(可选)

## 支持的 DNS 提供商

### Cloudflare

```yaml
domains:
  - name: "example.com"
    provider: "cloudflare"
    subdomain: "www"
    credentials:
      api_token: "your-api-token"
```

### 阿里云

```yaml
domains:
  - name: "example.com"
    provider: "aliyun"
    subdomain: "www"
    credentials:
      access_key_id: "your-access-key-id"
      access_key_secret: "your-access-key-secret"
```

### 腾讯云

```yaml
domains:
  - name: "example.com"
    provider: "tencent"
    subdomain: "www"
    credentials:
      secret_id: "your-secret-id"
      secret_key: "your-secret-key"
```

## 监控和维护

### 查看日志

```bash
# Docker Compose
docker-compose logs -f

# Docker
docker logs -f ddns-tool
```

### 查看数据库

```bash
docker exec -it ddns-tool sh
sqlite3 /app/data/data.db
```

### 备份数据

```bash
docker cp ddns-tool:/app/data/data.db ./backup-$(date +%Y%m%d).db
```

## 故障排除

### 容器无法启动

1. 检查日志: `docker logs ddns-tool`
2. 验证配置文件语法
3. 确认凭证正确性

### DNS 更新失败

1. 检查网络连接
2. 验证 API 凭证
3. 查看 DNS 提供商限制(速率限制等)

### IP 检测失败

1. 检查容器网络访问
2. 尝试更改 `ip_detection_method` 配置

## 生产环境建议

1. **使用环境变量存储敏感信息**
   ```bash
   docker run -d \
     -e CLOUDFLARE_API_TOKEN=xxx \
     -e ALIYUN_ACCESS_KEY_ID=xxx \
     ddns-tool
   ```

2. **定期备份数据库**
   ```bash
   # 添加到 crontab
   0 2 * * * docker cp ddns-tool:/app/data/data.db /backup/ddns-$(date +\%Y\%m\%d).db
   ```

3. **监控容器健康**
   ```bash
   docker ps --filter "name=ddns-tool"
   docker inspect --format='{{.State.Health.Status}}' ddns-tool
   ```

4. **使用特定版本标签**
   ```dockerfile
   FROM ddns-tool:0.1.0
   ```

## 更新

```bash
# 拉取最新代码
git pull

# 重新构建镜像
docker-compose build

# 重启服务
docker-compose up -d
```
