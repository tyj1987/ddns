#!/bin/bash

set -e

echo "========================================"
echo "  DDNS Tool 自动安装脚本"
echo "========================================"
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 检测操作系统
detect_os() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        OS=$ID
        OS_VERSION=$VERSION_ID
    elif [ -f /etc/redhat-release ]; then
        OS="rhel"
    else
        OS=$(uname -s)
    fi
    echo -e "${GREEN}检测到操作系统: $OS${NC}"
}

# 检查命令是否存在
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# 检查 Node.js
check_nodejs() {
    echo -e "\n${BLUE}[1/6] 检查 Node.js...${NC}"
    if command_exists node; then
        NODE_VERSION=$(node -v)
        echo -e "${GREEN}✓ Node.js 已安装: $NODE_VERSION${NC}"
        return 0
    else
        echo -e "${YELLOW}✗ Node.js 未安装${NC}"
        return 1
    fi
}

# 安装 Node.js
install_nodejs() {
    echo -e "${YELLOW}正在安装 Node.js...${NC}"

    case $OS in
        ubuntu|debian)
            curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
            sudo apt-get install -y nodejs
            ;;
        fedora|rhel|centos)
            sudo dnf install -y nodejs
            ;;
        arch|manjaro)
            sudo pacman -S --noconfirm nodejs npm
            ;;
        *)
            echo -e "${RED}不支持的操作系统: $OS${NC}"
            echo "请手动安装 Node.js: https://nodejs.org/"
            exit 1
            ;;
    esac

    if command_exists node; then
        echo -e "${GREEN}✓ Node.js 安装成功: $(node -v)${NC}"
    else
        echo -e "${RED}✗ Node.js 安装失败${NC}"
        exit 1
    fi
}

# 检查 Rust
check_rust() {
    echo -e "\n${BLUE}[2/6] 检查 Rust...${NC}"
    if command_exists rustc; then
        RUST_VERSION=$(rustc --version)
        echo -e "${GREEN}✓ Rust 已安装: $RUST_VERSION${NC}"
        return 0
    else
        echo -e "${YELLOW}✗ Rust 未安装${NC}"
        return 1
    fi
}

# 安装 Rust
install_rust() {
    echo -e "${YELLOW}正在安装 Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env

    if command_exists rustc; then
        echo -e "${GREEN}✓ Rust 安装成功: $(rustc --version)${NC}"
    else
        echo -e "${RED}✗ Rust 安装失败${NC}"
        exit 1
    fi
}

# 检查系统依赖
check_system_deps() {
    echo -e "\n${BLUE}[3/6] 检查系统依赖...${NC}"

    MISSING_DEPS=0

    case $OS in
        ubuntu|debian)
            dpkg -l | grep -q libwebkit2gtk-4.1-dev || { echo -e "${YELLOW}✗ 缺少 libwebkit2gtk-4.1-dev${NC}"; MISSING_DEPS=1; }
            dpkg -l | grep -q build-essential || { echo -e "${YELLOW}✗ 缺少 build-essential${NC}"; MISSING_DEPS=1; }
            dpkg -l | grep -q libayatana-appindicator3-dev || { echo -e "${YELLOW}✗ 缺少 libayatana-appindicator3-dev${NC}"; MISSING_DEPS=1; }
            ;;
        fedora)
            rpm -q webkit2gtk4.1-devel >/dev/null 2>&1 || { echo -e "${YELLOW}✗ 缺少 webkit2gtk4.1-devel${NC}"; MISSING_DEPS=1; }
            rpm -q libappindicator-gtk3-devel >/dev/null 2>&1 || { echo -e "${YELLOW}✗ 缺少 libappindicator-gtk3-devel${NC}"; MISSING_DEPS=1; }
            ;;
        arch|manjaro)
            pacman -Q webkit2gtk-4.1 >/dev/null 2>&1 || { echo -e "${YELLOW}✗ 缺少 webkit2gtk-4.1${NC}"; MISSING_DEPS=1; }
            pacman -Q libappindicator-gtk3 >/dev/null 2>&1 || { echo -e "${YELLOW}✗ 缺少 libappindicator-gtk3${NC}"; MISSING_DEPS=1; }
            ;;
    esac

    if [ $MISSING_DEPS -eq 0 ]; then
        echo -e "${GREEN}✓ 所有系统依赖已安装${NC}"
        return 0
    else
        return 1
    fi
}

# 安装系统依赖
install_system_deps() {
    echo -e "${YELLOW}正在安装系统依赖...${NC}"

    case $OS in
        ubuntu|debian)
            echo "需要 sudo 权限来安装依赖..."
            sudo apt-get update
            sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential libayatana-appindicator3-dev curl
            ;;
        fedora|rhel|centos)
            echo "需要 sudo 权限来安装依赖..."
            sudo dnf install -y webkit2gtk4.1-devel libappindicator-gtk3-devel curl
            ;;
        arch|manjaro)
            echo "需要 sudo 权限来安装依赖..."
            sudo pacman -S --noconfirm webkit2gtk-4.1 libappindicator-gtk3 curl
            ;;
        *)
            echo -e "${RED}不支持的操作系统: $OS${NC}"
            echo "请手动安装系统依赖"
            exit 1
            ;;
    esac

    echo -e "${GREEN}✓ 系统依赖安装完成${NC}"
}

# 安装 npm 依赖
install_npm_deps() {
    echo -e "\n${BLUE}[4/6] 安装 npm 依赖...${NC}"
    if [ -f "package.json" ]; then
        npm install
        echo -e "${GREEN}✓ npm 依赖安装完成${NC}"
    else
        echo -e "${RED}✗ 未找到 package.json${NC}"
        exit 1
    fi
}

# 编译项目
build_project() {
    echo -e "\n${BLUE}[5/6] 编译项目...${NC}"
    echo "这可能需要几分钟时间..."

    if npm run tauri build; then
        echo -e "${GREEN}✓ 项目编译成功${NC}"
    else
        echo -e "${RED}✗ 项目编译失败${NC}"
        exit 1
    fi
}

# 显示安装结果
show_result() {
    echo -e "\n${BLUE}[6/6] 安装结果...${NC}"
    echo ""
    echo -e "${GREEN}========================================"
    echo "  ✓ 安装完成!"
    echo "========================================${NC}"
    echo ""
    echo "生成的安装包:"
    echo ""

    if [ -d "src-tauri/target/release/bundle/deb" ]; then
        DEB_PKG=$(find src-tauri/target/release/bundle/deb -name "*.deb" | head -1)
        if [ -n "$DEB_PKG" ]; then
            echo -e "${GREEN}DEB 包${NC}: $DEB_PKG"
            echo "  安装命令: sudo dpkg -i $DEB_PKG"
        fi
    fi

    echo ""
    if [ -d "src-tauri/target/release/bundle/rpm" ]; then
        RPM_PKG=$(find src-tauri/target/release/bundle/rpm -name "*.rpm" | head -1)
        if [ -n "$RPM_PKG" ]; then
            echo -e "${GREEN}RPM 包${NC}: $RPM_PKG"
            echo "  安装命令: sudo rpm -i $RPM_PKG"
        fi
    fi

    echo ""
    echo "也可以直接运行编译后的程序:"
    echo "  ./src-tauri/target/release/ddns"
    echo ""
}

# 主流程
main() {
    # 检测操作系统
    detect_os

    # 检查并安装 Node.js
    if ! check_nodejs; then
        install_nodejs
    fi

    # 检查并安装 Rust
    if ! check_rust; then
        install_rust
        # 重新加载环境变量
        source $HOME/.cargo/env
    fi

    # 检查并安装系统依赖
    if ! check_system_deps; then
        install_system_deps
    fi

    # 安装 npm 依赖
    install_npm_deps

    # 编译项目
    build_project

    # 显示结果
    show_result
}

# 运行主流程
main
