#!/bin/bash
# 清理脚本
# 清理构建产物和临时文件

set -e

echo "🧹 开始清理..."

# 清理前端构建产物
echo "📦 清理前端构建..."
rm -rf dist
rm -rf node_modules/.vite

# 清理 Rust 构建产物
echo "📦 清理 Rust 构建..."
cd src-tauri
cargo clean
cd ..

# 清理日志
echo "📦 清理日志文件..."
rm -f *.log

echo "✅ 清理完成!"
