#!/bin/bash
# 开发模式启动脚本
# 同时启动前端开发服务器和 Tauri

set -e

echo "🚀 启动开发模式..."

# 检查是否安装了依赖
if [ ! -d "node_modules" ]; then
    echo "📦 首次运行,安装依赖..."
    npm install
fi

echo "🔥 启动 Tauri 开发模式..."
npm run tauri dev
