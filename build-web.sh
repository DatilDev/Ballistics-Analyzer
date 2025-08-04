#!/bin/bash

set -e

echo "Building Ballistics Analyzer PWA..."

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

# Check dependencies
command -v wasm-pack >/dev/null 2>&1 || { 
    echo -e "${RED}wasm-pack is required but not installed.${NC}" 
    echo "Install with: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
}

# Clean previous builds
echo -e "${BLUE}Cleaning previous builds...${NC}"
rm -rf pkg dist

# Build WASM
echo -e "${BLUE}Building WASM module...${NC}"
wasm-pack build --target web --out-dir pkg --release

# Create dist directory
mkdir -p dist
mkdir -p dist/assets
mkdir -p dist/pkg

# Copy files
echo -e "${BLUE}Copying files...${NC}"
cp index.html dist/
cp manifest.json dist/
cp sw.js dist/
cp -r assets/* dist/assets/ 2>/dev/null || :
cp -r pkg/* dist/pkg/

# Generate icons if not present
if [ ! -f "dist/assets/icon-192x192.png" ]; then
    echo -e "${BLUE}Generating PWA icons...${NC}"
    # Create placeholder icons (in production, use proper icon generator)
    mkdir -p dist/assets
    # This would normally use ImageMagick or similar to generate icons
fi

# Optimize for production
echo -e "${BLUE}Optimizing for production...${NC}"
# Add any optimization steps here

echo -e "${GREEN}✓ Build complete!${NC}"
echo -e "To test locally, run: ${BLUE}cd dist && python3 -m http.server 8000${NC}"
echo -e "Then visit: ${BLUE}http://localhost:8000${NC}"