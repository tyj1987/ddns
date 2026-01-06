#!/bin/bash
# 简单的 GitHub 推送脚本

echo "🚀 开始推送到 GitHub..."
echo ""

# 设置 SSH 远程地址
git remote set-url origin git@github.com:tyj1987/ddns.git

echo "📡 远程仓库: git@github.com:tyj1987/ddns.git"
echo ""

# 推送
echo "⏳ 正在推送..."
git push -u origin main

if [ $? -eq 0 ]; then
  echo ""
  echo "✅ 推送成功!"
  echo ""
  echo "🔗 访问仓库: https://github.com/tyj1987/ddns"
  echo "📊 CI/CD: https://github.com/tyj1987/ddns/actions"
else
  echo ""
  echo "❌ 推送失败!"
  echo ""
  echo "💡 请参考 PUSH_GUIDE.md 配置 SSH 密钥或使用 Token"
  echo ""
  echo "📖 查看完整推送指南:"
  echo "   cat PUSH_GUIDE.md"
fi
