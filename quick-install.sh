#!/bin/bash

set -e

echo "========================================"
echo "  DDNS Tool 快速安装"
echo "========================================"
echo ""

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# 检查依赖
echo -e "${BLUE}检查依赖...${NC}"

if ! command -v node &> /dev/null; then
    echo -e "${YELLOW}错误: 未找到 Node.js${NC}"
    echo "请使用 ./install.sh 进行完整安装"
    exit 1
fi

if ! command -v rustc &> /dev/null; then
    echo -e "${YELLOW}错误: 未找到 Rust${NC}"
    echo "请使用 ./install.sh 进行完整安装"
    exit 1
fi

echo -e "${GREEN}✓ 依赖检查通过${NC}"

# 安装 npm 依赖
echo -e "\n${BLUE}安装 npm 依赖...${NC}"
npm install

# 编译项目
echo -e "\n${BLUE}编译项目...${NC}"
npm run tauri build

# 显示结果
echo ""
echo -e "${GREEN}========================================"
echo "  ✓ 编译完成!"
echo "========================================${NC}"
echo ""

DEB_PKG=$(find src-tauri/target/release/bundle/deb -name "*.deb" 2>/dev/null | head -1)
RPM_PKG=$(find src-tauri/target/release/bundle/rpm -name "*.rpm" 2>/dev/null | head -1)

if [ -n "$DEB_PKG" ]; then
    echo -e "${GREEN}DEB 包${NC}: $DEB_PKG"
fi

if [ -n "$RPM_PKG" ]; then
    echo -e "${GREEN}RPM 包${NC}: $RPM_PKG"
fi

echo ""
echo "安装方法:"
echo "  Ubuntu/Debian: sudo dpkg -i $DEB_PKG"
echo "  Fedora/RHEL:   sudo rpm -i $RPM_PKG"
echo ""
