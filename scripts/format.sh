#!/bin/bash
# 代码格式化脚本
# 格式化所有 Rust 和 TypeScript 代码

set -e

echo "🎨 开始格式化代码..."

# 格式化 Rust 代码
echo "📦 格式化 Rust 代码..."
cd src-tauri
cargo fmt
cd ..

# 格式化 TypeScript 代码
echo "📦 格式化 TypeScript 代码..."
npm run format

echo "✅ 代码格式化完成!"
