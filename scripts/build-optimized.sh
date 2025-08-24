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
# Build what we can
for platform in $CAN_BUILD; do
    case $platform in
        arch)
            make build-arch
            ;;
        android)
            make build-android
            ;;
    esac
done
done

echo -e "${GREEN}Build complete for:$CAN_BUILD${NC}"