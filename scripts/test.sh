#!/bin/bash
# 测试脚本
# 运行所有测试

set -e

echo "🧪 开始运行测试..."

# 运行 Rust 测试
echo "📦 运行 Rust 测试..."
cd src-tauri
cargo test --lib
cd ..

echo ""
echo "✅ 测试完成!"
echo ""
echo "注意: 前端 E2E 测试需要配置,见 TESTING_GUIDE.md"
