#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}Optimized Build for Android & Arch Linux${NC}"

# Detect what we can build
CAN_BUILD=""
if [ -f /etc/arch-release ]; then
    CAN_BUILD="$CAN_BUILD arch"
fi
if [ -n "$ANDROID_HOME" ]; then
    CAN_BUILD="$CAN_BUILD android"
fi

if [ -z "$CAN_BUILD" ]; then
    echo -e "${YELLOW}Building generic binary only${NC}"
    cargo build --release -p ballistics-desktop --no-default-features --features minimal
    exit 0
fi

# Build what we can
for platform in $CAN_BUILD; do
    echo -e "${YELLOW}Building $platform...${NC}"
    make build-$platform
done

echo -e "${GREEN}Build complete for:$CAN_BUILD${NC}"