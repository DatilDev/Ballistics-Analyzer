#!/bin/bash
# scripts/build-android.sh - Build Android APK for Ballistics Analyzer

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Building Ballistics Analyzer Android ${NC}"
echo -e "${GREEN}========================================${NC}"

# Check for Android SDK
export ANDROID_SDK_ROOT="${ANDROID_SDK_ROOT:-$ANDROID_HOME}"
export ANDROID_NDK_ROOT="${ANDROID_NDK_ROOT:-$ANDROID_SDK_ROOT/ndk/26.1.10909125}"

if [ -z "$ANDROID_SDK_ROOT" ]; then
    echo -e "${RED}Error: ANDROID_SDK_ROOT/ANDROID_HOME not set${NC}"
    echo -e "${YELLOW}Please set ANDROID_HOME environment variable${NC}"
    exit 1
fi

echo -e "${BLUE}Android SDK: $ANDROID_SDK_ROOT${NC}"
echo -e "${BLUE}Android NDK: $ANDROID_NDK_ROOT${NC}"

# Check for cargo-ndk
if ! command -v cargo-ndk &> /dev/null; then
    echo -e "${YELLOW}Installing cargo-ndk...${NC}"
    cargo install cargo-ndk
fi

# Change to mobile directory
cd ballistics-mobile

# Build Rust libraries for Android architectures
echo -e "${YELLOW}Building Rust libraries for Android...${NC}"
cargo ndk \
    --target aarch64-linux-android \
    --target armv7-linux-androideabi \
    --platform 21 \
    build --release

# Create JNI libs directory structure
echo -e "${YELLOW}Copying native libraries...${NC}"
mkdir -p android/app/src/main/jniLibs/{arm64-v8a,armeabi-v7a}

# Copy the built libraries
cp ../target/aarch64-linux-android/release/libballistics_mobile.so \
    android/app/src/main/jniLibs/arm64-v8a/

cp ../target/armv7-linux-androideabi/release/libballistics_mobile.so \
    android/app/src/main/jniLibs/armeabi-v7a/

echo -e "${GREEN}✓ Native libraries copied${NC}"

# Change to Android directory
cd android

# Ensure gradlew is executable
chmod +x gradlew

# Clean previous builds
echo -e "${YELLOW}Cleaning previous builds...${NC}"
./gradlew clean

# Build APK
echo -e "${YELLOW}Building APK...${NC}"
./gradlew assembleRelease

# Create output directory
mkdir -p ../../build/android

# Copy unsigned APK
cp app/build/outputs/apk/release/app-release-unsigned.apk \
    ../../build/android/ballistics-analyzer-unsigned.apk

echo -e "${GREEN}✓ Unsigned APK created${NC}"

# Check if keystore exists for signing
KEYSTORE_PATH="release-keystore.jks"
if [ -f "$KEYSTORE_PATH" ]; then
    echo -e "${YELLOW}Signing APK...${NC}"
    
    # Use apksigner from build-tools
    if [ -f "$ANDROID_SDK_ROOT/build-tools/34.0.0/apksigner" ]; then
        $ANDROID_SDK_ROOT/build-tools/34.0.0/apksigner sign \
            --ks "$KEYSTORE_PATH" \
            --ks-key-alias "${KEY_ALIAS:-ballistics}" \
            --out ../../build/android/ballistics-analyzer.apk \
            ../../build/android/ballistics-analyzer-unsigned.apk
        
        echo -e "${GREEN}✓ Signed APK created${NC}"
    else
        echo -e "${YELLOW}Warning: apksigner not found, APK remains unsigned${NC}"
    fi
else
    echo -e "${YELLOW}No keystore found. APK is unsigned.${NC}"
    echo -e "${YELLOW}To create a keystore, run:${NC}"
    echo "keytool -genkey -v -keystore ballistics-mobile/android/release-keystore.jks \\"
    echo "  -alias ballistics -keyalg RSA -keysize 2048 -validity 10000"
fi

# Build AAB for Play Store (optional)
if [ "${BUILD_AAB:-false}" = "true" ]; then
    echo -e "${YELLOW}Building AAB for Play Store...${NC}"
    ./gradlew bundleRelease
    
    cp app/build/outputs/bundle/release/app-release.aab \
        ../../build/android/ballistics-analyzer.aab
    
    echo -e "${GREEN}✓ AAB created${NC}"
fi

# Return to original directory
cd ../..

# Summary
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Build Complete!                       ${NC}"
echo -e "${GREEN}========================================${NC}"
echo -e "${BLUE}Output files:${NC}"
echo -e "  • ${YELLOW}build/android/ballistics-analyzer-unsigned.apk${NC}"

if [ -f "build/android/ballistics-analyzer.apk" ]; then
    echo -e "  • ${YELLOW}build/android/ballistics-analyzer.apk${NC} (signed)"
fi

if [ -f "build/android/ballistics-analyzer.aab" ]; then
    echo -e "  • ${YELLOW}build/android/ballistics-analyzer.aab${NC} (Play Store)"
fi

# Display APK info if aapt is available
if command -v aapt &> /dev/null; then
    echo -e "\n${BLUE}APK Information:${NC}"
    aapt dump badging build/android/ballistics-analyzer-unsigned.apk | \
        grep -E "package:|sdkVersion:|targetSdkVersion:|application-label:" | \
        sed 's/^/  /'
fi

# Display file sizes
echo -e "\n${BLUE}File sizes:${NC}"
ls -lh build/android/*.apk 2>/dev/null | awk '{print "  • "$9": "$5}'

exit 0