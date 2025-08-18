#!/bin/bash
# scripts/build-android.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building Ballistics Analyzer for Android...${NC}"

# Set environment variables
export ANDROID_SDK_ROOT="${ANDROID_SDK_ROOT:-$ANDROID_HOME}"
export ANDROID_NDK_ROOT="${ANDROID_NDK_ROOT:-$ANDROID_SDK_ROOT/ndk/26.1.10909125}"

if [ -z "$ANDROID_SDK_ROOT" ]; then
    echo -e "${RED}Error: ANDROID_SDK_ROOT is not set${NC}"
    exit 1
fi

# Change to mobile directory
cd ballistics-mobile

# Build Rust libraries for all Android architectures
echo -e "${YELLOW}Building Rust libraries...${NC}"
cargo ndk \
    --target aarch64-linux-android \
    --target armv7-linux-androideabi \
    --target x86_64-linux-android \
    --target i686-linux-android \
    --platform 21 \
    build --release

# Copy built libraries to Android jniLibs directory
echo -e "${YELLOW}Copying native libraries...${NC}"
mkdir -p android/app/src/main/jniLibs/{arm64-v8a,armeabi-v7a,x86_64,x86}

cp ../target/aarch64-linux-android/release/libballistics_mobile.so \
    android/app/src/main/jniLibs/arm64-v8a/

cp ../target/armv7-linux-androideabi/release/libballistics_mobile.so \
    android/app/src/main/jniLibs/armeabi-v7a/

cp ../target/x86_64-linux-android/release/libballistics_mobile.so \
    android/app/src/main/jniLibs/x86_64/

cp ../target/i686-linux-android/release/libballistics_mobile.so \
    android/app/src/main/jniLibs/x86/

# Change to Android directory
cd android

# Clean previous builds
echo -e "${YELLOW}Cleaning previous builds...${NC}"
./gradlew clean

# Build APK
echo -e "${YELLOW}Building APK...${NC}"
./gradlew assembleRelease

# Copy APK to output directory
echo -e "${YELLOW}Copying APK to output directory...${NC}"
mkdir -p ../../build/android
cp app/build/outputs/apk/release/app-release-unsigned.apk \
    ../../build/android/ballistics-analyzer-unsigned.apk

# Sign APK if keystore exists
if [ -f "release-keystore.jks" ]; then
    echo -e "${YELLOW}Signing APK...${NC}"
    
    # Use apksigner from build-tools
    $ANDROID_SDK_ROOT/build-tools/34.0.0/apksigner sign \
        --ks release-keystore.jks \
        --ks-key-alias ballistics \
        --out ../../build/android/ballistics-analyzer.apk \
        ../../build/android/ballistics-analyzer-unsigned.apk
    
    echo -e "${GREEN}✓ Signed APK created: build/android/ballistics-analyzer.apk${NC}"
else
    echo -e "${YELLOW}No keystore found. APK is unsigned.${NC}"
    echo -e "${YELLOW}To sign the APK, create a keystore with:${NC}"
    echo "keytool -genkey -v -keystore ballistics-mobile/android/release-keystore.jks -alias ballistics -keyalg RSA -keysize 2048 -validity 10000"
fi

# Generate AAB (Android App Bundle) for Play Store
echo -e "${YELLOW}Building AAB for Play Store...${NC}"
./gradlew bundleRelease

cp app/build/outputs/bundle/release/app-release.aab \
    ../../build/android/ballistics-analyzer.aab

echo -e "${GREEN}✓ Build complete!${NC}"
echo -e "${GREEN}APK location: build/android/ballistics-analyzer-unsigned.apk${NC}"
echo -e "${GREEN}AAB location: build/android/ballistics-analyzer.aab${NC}"

cd ../..

# Display APK info
if command -v aapt &> /dev/null; then
    echo -e "\n${YELLOW}APK Information:${NC}"
    aapt dump badging build/android/ballistics-analyzer-unsigned.apk | grep -E "package:|sdkVersion:|application-label:"
fi