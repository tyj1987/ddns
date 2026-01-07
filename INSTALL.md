# 安装说明

DDNS Tool 提供了自动化安装脚本,可以自动检测环境并安装所需依赖。

## 自动安装 (推荐)

### 完整自动安装

适用于首次安装或环境不完整的情况:

```bash
# 1. 克隆仓库
git clone https://github.com/tyj1987/ddns.git
cd ddns

# 2. 运行自动安装脚本
./install.sh
```

**脚本会自动完成以下操作:**
- ✓ 检测操作系统类型
- ✓ 检查并安装 Node.js
- ✓ 检查并安装 Rust
- ✓ 检查并安装系统依赖
- ✓ 安装 npm 依赖
- ✓ 编译项目
- ✓ 生成安装包

**支持的操作系统:**
- Ubuntu / Debian
- Fedora / RHEL / CentOS
- Arch Linux / Manjaro

### 快速安装

适用于已安装 Node.js 和 Rust 的环境:

```bash
./quick-install.sh
```

## 手动安装

如果自动安装脚本遇到问题,可以手动安装:

### 1. 安装系统依赖

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential libayatana-appindicator3-dev
```

**Fedora/RHEL:**
```bash
sudo dnf install -y webkit2gtk4.1-devel libappindicator-gtk3-devel
```

**Arch Linux:**
```bash
sudo pacman -S webkit2gtk-4.1 libappindicator-gtk3
```

### 2. 安装 Node.js

如果系统未安装 Node.js:

```bash
# Ubuntu/Debian (使用 NodeSource 仓库)
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs

# 或使用 nvm 安装
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20
```

### 3. 安装 Rust

如果系统未安装 Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 4. 安装项目依赖并编译

```bash
# 安装 npm 依赖
npm install

# 编译项目
npm run tauri build
```

## 安装生成的包

编译完成后,会在 `src-tauri/target/release/bundle/` 目录下生成安装包:

### Ubuntu/Debian:
```bash
sudo dpkg -i src-tauri/target/release/bundle/deb/DDNS 工具_0.1.0_amd64.deb
```

### Fedora/RHEL:
```bash
sudo rpm -i src-tauri/target/release/bundle/rpm/DDNS 工具-0.1.0-1.x86_64.rpm
```

### 直接运行
```bash
./src-tauri/target/release/ddns
```

## 验证安装

安装完成后,可以运行以下命令验证:

```bash
# 检查版本
ddns --version

# 或直接运行
ddns
```

## 开发模式

如果只想开发测试,不需要编译:

```bash
npm install
npm run tauri dev
```

## 故障排除

### 问题 1: 找不到 Node.js

**错误:** `bash: node: command not found`

**解决:** 运行 `./install.sh` 自动安装,或参考手动安装步骤

### 问题 2: 找不到 Rust

**错误:** `bash: rustc: command not found`

**解决:** 运行 `./install.sh` 自动安装,或手动安装 Rust

### 问题 3: 缺少系统库

**错误:** `Can't detect any appindicator library`

**解决:** 安装 libayatana-appindicator3-dev (见手动安装步骤)

### 问题 4: 编译警告

**说明:** 编译警告是正常的,不影响程序使用

### 问题 5: 权限错误

**错误:** `Permission denied`

**解决:** 确保脚本有执行权限:
```bash
chmod +x install.sh quick-install.sh
```

## 系统要求

- **操作系统:** Linux (Ubuntu 20.04+, Fedora 35+, Arch Linux)
- **内存:** 至少 2GB RAM
- **磁盘空间:** 至少 1GB 可用空间
- **网络:** 需要互联网连接下载依赖

## 卸载

### Ubuntu/Debian:
```bash
sudo dpkg -r ddns
```

### Fedora/RHEL:
```bash
sudo rpm -e ddns
```

### 手动删除:
```bash
rm -rf ~/.config/ddns
rm -rf ~/.local/share/ddns
```

## 更新

```bash
# 拉取最新代码
git pull

# 重新编译
./quick-install.sh

# 或完整重新安装
./install.sh
```

## 下一步

安装完成后,请参考 [README.md](README.md) 了解如何使用 DDNS Tool。
