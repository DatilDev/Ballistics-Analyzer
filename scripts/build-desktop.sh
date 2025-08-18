#!/bin/bash
# scripts/build-desktop.sh - Universal build script for Linux and macOS

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Detect OS
OS="unknown"
ARCH=$(uname -m)

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
    if [ -f /etc/debian_version ]; then
        DISTRO="debian"
    elif [ -f /etc/arch-release ]; then
        DISTRO="arch"
    elif [ -f /etc/fedora-release ]; then
        DISTRO="fedora"
    elif [ -f /etc/alpine-release ]; then
        DISTRO="alpine"
    else
        DISTRO="unknown"
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
    DISTRO="darwin"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    OS="windows"
    DISTRO="windows"
else
    echo -e "${RED}Unsupported OS: $OSTYPE${NC}"
    exit 1
fi

echo -e "${BLUE}Building Ballistics Analyzer Desktop${NC}"
echo -e "${GREEN}OS: $OS ($DISTRO) - Arch: $ARCH${NC}"

# Function to check command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to install dependencies
install_dependencies() {
    echo -e "${YELLOW}Checking dependencies...${NC}"
    
    case "$DISTRO" in
        debian)
            echo -e "${YELLOW}Installing Debian/Ubuntu dependencies...${NC}"
            sudo apt-get update
            sudo apt-get install -y \
                build-essential \
                pkg-config \
                libssl-dev \
                libgtk-3-dev \
                libwebkit2gtk-4.0-dev \
                libappindicator3-dev \
                librsvg2-dev \
                libx11-dev \
                libxcb1-dev \
                libxcb-render0-dev \
                libxcb-shape0-dev \
                libxcb-xfixes0-dev \
                libxkbcommon-dev \
                libgl1-mesa-dev \
                libegl1-mesa-dev \
                libwayland-dev
            ;;
        arch)
            echo -e "${YELLOW}Installing Arch Linux dependencies...${NC}"
            sudo pacman -Sy --needed \
                base-devel \
                pkg-config \
                openssl \
                gtk3 \
                webkit2gtk \
                libappindicator-gtk3 \
                librsvg \
                libx11 \
                libxcb \
                libxkbcommon \
                mesa \
                wayland
            ;;
        fedora)
            echo -e "${YELLOW}Installing Fedora dependencies...${NC}"
            sudo dnf install -y \
                gcc \
                pkg-config \
                openssl-devel \
                gtk3-devel \
                webkit2gtk3-devel \
                libappindicator-gtk3-devel \
                librsvg2-devel \
                libX11-devel \
                libxcb-devel \
                libxkbcommon-devel \
                mesa-libGL-devel \
                mesa-libEGL-devel \
                wayland-devel
            ;;
        alpine)
            echo -e "${YELLOW}Installing Alpine Linux dependencies...${NC}"
            sudo apk add \
                build-base \
                pkgconf \
                openssl-dev \
                gtk+3.0-dev \
                webkit2gtk-dev \
                librsvg-dev \
                libx11-dev \
                libxcb-dev \
                libxkbcommon-dev \
                mesa-dev \
                wayland-dev
            ;;
        darwin)
            echo -e "${YELLOW}Checking macOS dependencies...${NC}"
            if ! command_exists brew; then
                echo -e "${RED}Homebrew not found. Please install from https://brew.sh${NC}"
                exit 1
            fi
            # macOS dependencies are usually handled by Rust/cargo
            ;;
    esac
}

# Check for Rust
if ! command_exists rustc; then
    echo -e "${RED}Rust not found. Installing...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
fi

# Update Rust
echo -e "${YELLOW}Updating Rust toolchain...${NC}"
rustup update stable

# Install dependencies if requested
if [ "$1" == "--install-deps" ]; then
    install_dependencies
fi

# Set build target based on OS and architecture
TARGET=""
case "$OS-$ARCH" in
    linux-x86_64)
        TARGET="x86_64-unknown-linux-gnu"
        ;;
    linux-aarch64)
        TARGET="aarch64-unknown-linux-gnu"
        ;;
    linux-armv7l)
        TARGET="armv7-unknown-linux-gnueabihf"
        ;;
    macos-x86_64)
        TARGET="x86_64-apple-darwin"
        ;;
    macos-arm64)
        TARGET="aarch64-apple-darwin"
        ;;
    *)
        echo -e "${YELLOW}Using default target${NC}"
        ;;
esac

# Add target if specified
if [ ! -z "$TARGET" ]; then
    echo -e "${YELLOW}Adding target: $TARGET${NC}"
    rustup target add $TARGET
    TARGET_FLAG="--target $TARGET"
else
    TARGET_FLAG=""
fi

# Build type
BUILD_TYPE="${BUILD_TYPE:-release}"
if [ "$BUILD_TYPE" == "release" ]; then
    BUILD_FLAGS="--release"
    OUTPUT_DIR="release"
else
    BUILD_FLAGS=""
    OUTPUT_DIR="debug"
fi

# Clean previous builds
if [ "$1" == "--clean" ] || [ "$2" == "--clean" ]; then
    echo -e "${YELLOW}Cleaning previous builds...${NC}"
    cargo clean
fi

# Build ballistics_core
echo -e "${BLUE}Building ballistics_core...${NC}"
cd ballistics_core
cargo build $BUILD_FLAGS $TARGET_FLAG
cd ..

# Build ballistics-desktop
echo -e "${BLUE}Building ballistics-desktop...${NC}"
cd ballistics-desktop
cargo build $BUILD_FLAGS $TARGET_FLAG
cd ..

# Find output binary
if [ ! -z "$TARGET" ]; then
    BINARY_PATH="target/$TARGET/$OUTPUT_DIR/ballistics-analyzer"
else
    BINARY_PATH="target/$OUTPUT_DIR/ballistics-analyzer"
fi

# Platform-specific post-processing
case "$OS" in
    linux)
        # Strip binary for smaller size
        if [ "$BUILD_TYPE" == "release" ]; then
            echo -e "${YELLOW}Stripping binary...${NC}"
            strip "$BINARY_PATH"
        fi
        
        # Create output directory
        mkdir -p build/linux
        cp "$BINARY_PATH" build/linux/
        
        # Copy assets
        cp -r ballistics-desktop/assets build/linux/
        
        echo -e "${GREEN}✓ Linux binary: build/linux/ballistics-analyzer${NC}"
        ;;
        
    macos)
        # Create app bundle
        echo -e "${YELLOW}Creating macOS app bundle...${NC}"
        ./scripts/create-macos-bundle.sh "$BINARY_PATH"
        echo -e "${GREEN}✓ macOS app: build/macos/Ballistics Analyzer.app${NC}"
        ;;
esac

# Create tarball for distribution
if [ "$BUILD_TYPE" == "release" ]; then
    echo -e "${YELLOW}Creating distribution package...${NC}"
    
    PACKAGE_NAME="ballistics-analyzer-${OS}-${ARCH}"
    mkdir -p "build/$PACKAGE_NAME"
    
    cp "$BINARY_PATH" "build/$PACKAGE_NAME/"
    cp README.md "build/$PACKAGE_NAME/"
    cp LICENSE "build/$PACKAGE_NAME/"
    cp PRIVACY_POLICY.md "build/$PACKAGE_NAME/"
    cp -r ballistics-desktop/assets "build/$PACKAGE_NAME/"
    
    cd build
    tar -czf "$PACKAGE_NAME.tar.gz" "$PACKAGE_NAME"
    cd ..
    
    echo -e "${GREEN}✓ Distribution package: build/$PACKAGE_NAME.tar.gz${NC}"
fi

# Display binary info
echo -e "${BLUE}Build Information:${NC}"
file "$BINARY_PATH"
ls -lh "$BINARY_PATH"

echo -e "${GREEN}✓ Build complete!${NC}"