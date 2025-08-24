#!/bin/bash
# setup-android-build.sh

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Setting up Android build environment...${NC}"

# 1. Install cargo-ndk
echo -e "${YELLOW}Installing cargo-ndk...${NC}"
cargo install cargo-ndk

# 2. Install Android targets
echo -e "${YELLOW}Installing Android targets...${NC}"
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android

# 3. Check Android SDK
if [ -z "$ANDROID_HOME" ]; then
    echo -e "${YELLOW}ANDROID_HOME not set. Setting it up...${NC}"
    
    # Try common locations
    if [ -d "$HOME/Android/Sdk" ]; then
        export ANDROID_HOME="$HOME/Android/Sdk"
    elif [ -d "/opt/android-sdk" ]; then
        export ANDROID_HOME="/opt/android-sdk"
    else
        echo "Please set ANDROID_HOME environment variable"
        echo "Add to ~/.bashrc:"
        echo "export ANDROID_HOME=/path/to/android/sdk"
        exit 1
    fi
fi

export ANDROID_SDK_ROOT="$ANDROID_HOME"
export NDK_HOME="$ANDROID_HOME/ndk/26.1.10909125"

echo -e "${GREEN}Android environment configured:${NC}"
echo "ANDROID_HOME: $ANDROID_HOME"
echo "NDK_HOME: $NDK_HOME"

# 4. Add to bashrc for persistence
echo -e "${YELLOW}Adding to ~/.bashrc...${NC}"
cat >> ~/.bashrc << EOF

# Android SDK
export ANDROID_HOME="$ANDROID_HOME"
export ANDROID_SDK_ROOT="\$ANDROID_HOME"
export NDK_HOME="\$ANDROID_HOME/ndk/26.1.10909125"
export PATH="\$PATH:\$ANDROID_HOME/platform-tools:\$ANDROID_HOME/tools"
EOF

echo -e "${GREEN}✓ Setup complete!${NC}"
echo ""
echo "Now you can build for Android:"
echo "  cd ballistics-mobile"
echo "  cargo ndk --target aarch64-linux-android build --release"