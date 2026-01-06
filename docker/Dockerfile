# DDNS 工具 Docker 镜像
# 多阶段构建,减小最终镜像大小

# 构建阶段
FROM rust:1.75-alpine AS builder

# 安装构建依赖
RUN apk add --no-cache \
    musl-dev \
    sqlite-dev \
    pkgconfig \
    openssl-dev

WORKDIR /build

# 复制 Cargo 配置
COPY src-tauri/Cargo.toml src-tauri/Cargo.lock ./

# 创建虚拟源码目录以缓存依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# 复制实际源码
COPY src-tauri/src ./src

# 构建应用(仅构建后端逻辑,不包含 GUI)
RUN cargo build --release && strip target/release/ddns

# 运行阶段
FROM alpine:3.19

# 安装运行时依赖
RUN apk add --no-cache \
    sqlite \
    ca-certificates

# 创建非 root 用户
RUN addgroup -g 1000 ddns && \
    adduser -D -u 1000 -G ddns ddns

WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /build/target/release/ddns /usr/local/bin/

# 创建数据目录
RUN mkdir -p /config && \
    chown -R ddns:ddns /config

USER ddns

# 环境变量
ENV DDNS_CONFIG_PATH=/config
ENV DDNS_HEADLESS=true

# 暴露端口(如果将来需要 Web UI)
EXPOSE 8080

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD pgrep -x ddns || exit 1

# 启动应用
ENTRYPOINT ["/usr/local/bin/ddns"]

# 默认命令
CMD ["--config", "/config/ddns.yml"]
