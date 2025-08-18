#!/bin/bash
# scripts/build-all.sh - Build for all supported platforms

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Ballistics Analyzer - Build All      ${NC}"
echo -e "${BLUE}========================================${NC}"

# Parse arguments
BUILD_TYPE="${1:-release}"
PLATFORMS="${2:-all}"

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Detect current platform
CURRENT_OS="unknown"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    CURRENT_OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    CURRENT_OS="macos"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    CURRENT_OS="windows"
fi

echo -e "${GREEN}Current platform: $CURRENT_OS${NC}"
echo -e "${GREEN}Build type: $BUILD_TYPE${NC}"
echo ""

# Build function
build_platform() {
    local platform=$1
    echo -e "${YELLOW}Building for $platform...${NC}"
    
    case $platform in
        desktop)
            if [ -f "scripts/build-desktop.sh" ]; then
                chmod +x scripts/build-desktop.sh
                BUILD_TYPE=$BUILD_TYPE ./scripts/build-desktop.sh
            elif [ "$CURRENT_OS" == "windows" ]; then
                powershell -ExecutionPolicy Bypass -File scripts/build-desktop.ps1 -BuildType $BUILD_TYPE
            else
                echo -e "${RED}Desktop build script not found${NC}"
                return 1
            fi
            ;;
            
        android)
            if [ -f "scripts/build-android.sh" ]; then
                chmod +x scripts/build-android.sh
                ./scripts/build-android.sh
            else
                echo -e "${RED}Android build script not found${NC}"
                return 1
            fi
            ;;
            
        ios)
            if [ "$CURRENT_OS" != "macos" ]; then
                echo -e "${YELLOW}iOS can only be built on macOS${NC}"
                return 1
            fi
            if [ -f "scripts/build-ios.sh" ]; then
                chmod +x scripts/build-ios.sh
                ./scripts/build-ios.sh
            else
                echo -e "${RED}iOS build script not found${NC}"
                return 1
            fi
            ;;
            
        wasm)
            if [ -f "scripts/build-wasm.sh" ]; then
                chmod +x scripts/build-wasm.sh
                ./scripts/build-wasm.sh
            else
                echo -e "${YELLOW}WASM build temporarily disabled${NC}"
            fi
            ;;
            
        docker)
            if command_exists docker; then
                echo -e "${YELLOW}Building Docker image...${NC}"
                docker build -t ballistics-analyzer:latest .
            else
                echo -e "${RED}Docker not found${NC}"
                return 1
            fi
            ;;
            
        *)
            echo -e "${RED}Unknown platform: $platform${NC}"
            return 1
            ;;
    esac
    
    echo -e "${GREEN}✓ $platform build complete${NC}"
    echo ""
}

# Create build directory
mkdir -p build

# Build based on platforms argument
if [ "$PLATFORMS" == "all" ]; then
    # Build everything possible on current platform
    echo -e "${BLUE}Building all available platforms...${NC}"
    echo ""
    
    # Always build desktop
    build_platform desktop
    
    # Build mobile if tools available
    if command_exists cargo && command_exists rustup; then
        if rustup target list | grep -q "aarch64-linux-android"; then
            build_platform android
        fi
    fi
    
    # Build iOS on macOS only
    if [ "$CURRENT_OS" == "macos" ]; then
        build_platform ios
    fi
    
    # Build Docker if available
    if command_exists docker; then
        build_platform docker
    fi
    
else
    # Build specific platforms
    IFS=',' read -ra PLATFORM_ARRAY <<< "$PLATFORMS"
    for platform in "${PLATFORM_ARRAY[@]}"; do
        build_platform "$platform"
    done
fi

# Summary
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Build Summary                         ${NC}"
echo -e "${BLUE}========================================${NC}"

echo -e "${GREEN}Build artifacts:${NC}"

# List desktop builds
if [ -d "build/linux" ]; then
    echo -e "  ${YELLOW}Linux:${NC}"
    ls -lh build/linux/ballistics-analyzer 2>/dev/null || true
    ls -lh *.AppImage 2>/dev/null || true
fi

if [ -d "build/macos" ]; then
    echo -e "  ${YELLOW}macOS:${NC}"
    ls -lhd "build/macos/Ballistics Analyzer.app" 2>/dev/null || true
    ls -lh build/macos/*.dmg 2>/dev/null || true
fi

if [ -d "build/windows" ]; then
    echo -e "  ${YELLOW}Windows:${NC}"
    ls -lh build/windows/ballistics-analyzer.exe 2>/dev/null || true
    ls -lh build/windows/*-setup.exe 2>/dev/null || true
fi

# List mobile builds
if [ -d "build/android" ]; then
    echo -e "  ${YELLOW}Android:${NC}"
    ls -lh build/android/*.apk 2>/dev/null || true
    ls -lh build/android/*.aab 2>/dev/null || true
fi

if [ -d "build/ios" ]; then
    echo -e "  ${YELLOW}iOS:${NC}"
    ls -lh build/ios/*.ipa 2>/dev/null || true
fi

# Docker
if command_exists docker; then
    echo -e "  ${YELLOW}Docker:${NC}"
    docker images ballistics-analyzer:latest --format "table {{.Repository}}:{{.Tag}}\t{{.Size}}"
fi

echo ""
echo -e "${GREEN}✓ All builds complete!${NC}"