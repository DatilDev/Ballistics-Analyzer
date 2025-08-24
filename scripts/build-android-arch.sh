#!/bin/bash
set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}Building for Android and Arch Linux only${NC}"

# Detect platform
if [ -f /etc/arch-release ]; then
    PLATFORM="arch"
elif [ -n "$ANDROID_HOME" ]; then
    PLATFORM="android-host"
else
    echo -e "${RED}Error: This build only supports Arch Linux and Android${NC}"
    exit 1
fi

# Build based on platform
case $PLATFORM in
    arch)
        echo -e "${YELLOW}Building Arch Linux package...${NC}"
        cd ballistics-desktop
        cargo build --release --features arch-linux
        cd ..
        makepkg -si
        ;;
    android-host)
        echo -e "${YELLOW}Building Android APK...${NC}"
        cd ballistics-mobile
        cargo ndk --target aarch64-linux-android --target armv7-linux-androideabi build --release
        cd android
        ./gradlew assembleRelease
        ;;
esac

echo -e "${GREEN}Build complete for $PLATFORM${NC}"