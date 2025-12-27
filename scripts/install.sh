#!/bin/bash
# ZeroTier MCP Server 安装脚本
# 用法: curl -fsSL https://raw.githubusercontent.com/fromsko/zerotier-sdk-rust-mcp/main/scripts/install.sh | bash

set -e

REPO="fromsko/zerotier-sdk-rust-mcp"
BINARY_NAME="zerotier-mcp"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# 颜色
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# 检测系统和架构
detect_platform() {
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)

    case "$OS" in
        linux)
            case "$ARCH" in
                x86_64) PLATFORM="x86_64-unknown-linux-gnu" ;;
                *) error "不支持的架构: $ARCH" ;;
            esac
            EXT="tar.gz"
            ;;
        darwin)
            case "$ARCH" in
                x86_64) PLATFORM="x86_64-apple-darwin" ;;
                arm64) PLATFORM="aarch64-apple-darwin" ;;
                *) error "不支持的架构: $ARCH" ;;
            esac
            EXT="tar.gz"
            ;;
        *)
            error "不支持的操作系统: $OS"
            ;;
    esac
}

# 获取最新版本
get_latest_version() {
    VERSION=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')
    if [ -z "$VERSION" ]; then
        error "无法获取最新版本"
    fi
    info "最新版本: $VERSION"
}

# 下载并安装
install() {
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$BINARY_NAME-$PLATFORM.$EXT"
    
    info "下载: $DOWNLOAD_URL"
    
    TMP_DIR=$(mktemp -d)
    trap "rm -rf $TMP_DIR" EXIT
    
    curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/archive.$EXT"
    
    info "解压..."
    cd "$TMP_DIR"
    tar xzf "archive.$EXT"
    
    info "安装到 $INSTALL_DIR"
    mkdir -p "$INSTALL_DIR"
    mv "$BINARY_NAME" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
    
    info "安装完成!"
    echo ""
    echo "可执行文件位置: $INSTALL_DIR/$BINARY_NAME"
    echo ""
    
    # 检查 PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        warn "$INSTALL_DIR 不在 PATH 中"
        echo "请添加到 ~/.bashrc 或 ~/.zshrc:"
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
    fi
}

main() {
    info "ZeroTier MCP Server 安装脚本"
    echo ""
    
    detect_platform
    info "检测到平台: $PLATFORM"
    
    get_latest_version
    install
    
    echo ""
    info "MCP 配置示例:"
    cat << 'EOF'
{
  "mcpServers": {
    "zerotier": {
      "command": "zerotier-mcp",
      "env": {
        "ZEROTIER_CENTRAL_TOKEN": "your_token"
      }
    }
  }
}
EOF
}

main
