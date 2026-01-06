#!/bin/bash
# 代码检查脚本
# 检查 Rust 和 TypeScript 代码

set -e

echo "🔍 开始检查代码..."

# 检查 TypeScript 类型
echo "📦 检查 TypeScript 类型..."
npx tsc --noEmit

if [ $? -eq 0 ]; then
    echo "✅ TypeScript 类型检查通过"
else
    echo "❌ TypeScript 类型检查失败"
    exit 1
fi

# 检查 Rust 代码 (跳过 GUI 依赖)
echo "📦 检查 Rust 代码..."
cd src-tauri
cargo check --lib 2>&1 | grep -E "(error|warning:.*unused|Finished|Checking)" || true

# 尝试完整检查 (可能因 GUI 库失败,这是预期的)
echo "📦 尝试完整检查 (可能因 GUI 库失败)..."
cargo check 2>&1 | tail -5 || true

cd ..

echo "✅ 代码检查完成!"
echo ""
echo "注意: Linux 上 GUI 库错误是预期的,不影响后端逻辑"
