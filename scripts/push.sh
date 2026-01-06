#!/bin/bash
# GitHub 推送脚本

set -e

echo "🚀 DDNS 工具 - GitHub 推送助手"
echo "================================"
echo ""
echo "📋 当前状态:"
git status -sb
echo ""

echo "🔧 远程仓库:"
git remote -v
echo ""

echo "📊 最近提交:"
git log -1 --oneline
echo ""

# 选择推送方式
echo "请选择推送方式:"
echo "1) SSH (推荐 - 需要配置 SSH 密钥)"
echo "2) HTTPS + Token (需要 Personal Access Token)"
echo "3) GitHub CLI (需要安装 gh)"
echo ""
read -p "请输入选项 (1/2/3): " choice

case $choice in
  1)
    echo ""
    echo "📝 使用 SSH 方式推送"
    echo "如果失败,请参考 PUSH_GUIDE.md 配置 SSH 密钥"
    echo ""

    # 设置为 SSH URL
    git remote set-url origin git@github.com:tyj1987/ddns.git

    # 推送
    echo "⏳ 正在推送到 GitHub..."
    git push -u origin main
    ;;
  2)
    echo ""
    echo "📝 使用 HTTPS + Token 方式推送"
    echo "请确保已创建 Personal Access Token"
    echo "Token: https://github.com/settings/tokens"
    echo ""

    # 设置为 HTTPS URL
    git remote set-url origin https://github.com/tyj1987/ddns.git

    # 推送
    echo "⏳ 正在推送到 GitHub..."
    echo "提示: 用户名输入 tyj1987,密码输入 Token (不是 GitHub 密码)"
    git push -u origin main
    ;;
  3)
    echo ""
    echo "📝 使用 GitHub CLI 方式推送"
    echo "确保已安装 gh 并登录: gh auth login"
    echo ""

    # 检查 gh 是否安装
    if ! command -v gh &> /dev/null; then
      echo "❌ GitHub CLI (gh) 未安装"
      echo "安装指南: https://github.com/cli/cli#installation"
      exit 1
    fi

    # 推送
    echo "⏳ 正在推送到 GitHub..."
    git push -u origin main
    ;;
  *)
    echo "❌ 无效的选项"
    exit 1
    ;;
esac

echo ""
echo "✅ 推送成功!"
echo ""
echo "🔗 访问你的仓库:"
echo "https://github.com/tyj1987/ddns"
echo ""
echo "📊 查看 CI/CD 状态:"
echo "https://github.com/tyj1987/ddns/actions"
echo ""
echo "🏷️  创建 Release:"
echo "https://github.com/tyj1987/ddns/releases/new"
