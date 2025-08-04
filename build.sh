#!/bin/bash

echo "Building Ballistics Analyzer for all platforms..."

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

# Create output directory
mkdir -p dist

# Build for Web (PWA)
echo -e "${BLUE}Building PWA...${NC}"
wasm-pack build --target web --out-dir pkg --release
rollup pkg/ballistics_analyzer.js --format iife --file pkg/bundle.js
cp index.html dist/
cp manifest.json dist/
cp sw.js dist/
cp -r assets dist/
cp -r pkg dist/

# Build for Desktop platforms
echo -e "${BLUE}Building for Windows...${NC}"
cargo build --release --target x86_64-pc-windows-msvc

echo -e "${BLUE}Building for macOS...${NC}"
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
lipo -create -output dist/ballistics-analyzer-macos \
  target/x86_64-apple-darwin/release/ballistics-analyzer \
  target/aarch64-apple-darwin/release/ballistics-analyzer

echo -e "${BLUE}Building for Linux...${NC}"
cargo build --release --target x86_64-unknown-linux-gnu

# Build for Mobile platforms
echo -e "${BLUE}Building for Android...${NC}"
cargo ndk -t arm64-v8a -t armeabi-v7a build --release

echo -e "${BLUE}Building for iOS...${NC}"
cargo lipo --release
cargo lipo --release --targets aarch64-apple-ios

echo -e "${GREEN}Build complete! Check the dist/ directory for all builds.${NC}"
*/