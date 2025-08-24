#!/bin/bash
# build.sh - Build script for all platforms

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building IronSights${NC}"

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Build for desktop (native)
build_desktop() {
    echo -e "${YELLOW}Building desktop application...${NC}"
    cargo build --release
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Desktop build complete${NC}"
        echo "Binary location: target/release/ironsights"
    else
        echo -e "${RED}✗ Desktop build failed${NC}"
        exit 1
    fi
}

# Build for web (WASM)
build_web() {
    echo -e "${YELLOW}Building web application...${NC}"
    
    # Check if wasm-pack is installed
    if ! command_exists wasm-pack; then
        echo -e "${YELLOW}Installing wasm-pack...${NC}"
        curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    fi
    
    # Build WASM
    wasm-pack build --target web --out-dir dist/pkg
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ WASM build complete${NC}"
        
        # Create index.html if it doesn't exist
        if [ ! -f "dist/index.html" ]; then
            echo -e "${YELLOW}Creating index.html...${NC}"
            cat > dist/index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>IronSights</title>
    <link rel="manifest" href="manifest.json">
    <meta name="theme-color" content="#1a1a1a">
    <style>
        html, body {
            margin: 0;
            padding: 0;
            width: 100%;
            height: 100%;
            overflow: hidden;
            background-color: #1a1a1a;
        }
        #loading {
            position: fixed;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            color: white;
            font-family: sans-serif;
            font-size: 24px;
        }
        canvas {
            width: 100% !important;
            height: 100% !important;
        }
    </style>
</head>
<body>
    <div id="loading">Loading IronSights...</div>
    <canvas id="ballistics_canvas"></canvas>
    <script type="module">
        import init from './pkg/ironsights.js';
        
        async function run() {
            await init();
            document.getElementById('loading').style.display = 'none';
        }
        
        run();
    </script>
</body>
</html>
EOF
        fi
        
        # Create manifest.json for PWA
        if [ ! -f "dist/manifest.json" ]; then
            echo -e "${YELLOW}Creating manifest.json...${NC}"
            cat > dist/manifest.json << 'EOF'
{
    "name": "IronSights",
    "short_name": "Ballistics",
    "description": "Professional ballistics calculator with hardware integration",
    "start_url": "/",
    "display": "standalone",
    "background_color": "#1a1a1a",
    "theme_color": "#1a1a1a",
    "orientation": "any",
    "icons": [
        {
            "src": "icon-192.png",
            "sizes": "192x192",
            "type": "image/png"
        },
        {
            "src": "icon-512.png",
            "sizes": "512x512",
            "type": "image/png"
        }
    ]
}
EOF
        fi
        
        # Create service worker for offline support
        if [ ! -f "dist/sw.js" ]; then
            echo -e "${YELLOW}Creating service worker...${NC}"
            cat > dist/sw.js << 'EOF'
const CACHE_NAME = 'ballistics-v1';
const urlsToCache = [
    '/',
    '/index.html',
    '/pkg/ironsights.js',
    '/pkg/ironsights_bg.wasm',
    '/manifest.json'
];

self.addEventListener('install', event => {
    event.waitUntil(
        caches.open(CACHE_NAME)
            .then(cache => cache.addAll(urlsToCache))
    );
});

self.addEventListener('fetch', event => {
    event.respondWith(
        caches.match(event.request)
            .then(response => response || fetch(event.request))
    );
});
EOF
        fi
        
        echo -e "${GREEN}✓ Web build complete${NC}"
        echo "Web files location: dist/"
    else
        echo -e "${RED}✗ Web build failed${NC}"
        exit 1
    fi
}

# Build for Android (using Tauri)
build_android() {
    echo -e "${YELLOW}Android build requires Tauri setup${NC}"
    echo "Please follow: https://tauri.app/v1/guides/getting-started/prerequisites"
}

# Build for iOS (using Tauri)
build_ios() {
    echo -e "${YELLOW}iOS build requires Tauri setup and macOS${NC}"
    echo "Please follow: https://tauri.app/v1/guides/getting-started/prerequisites"
}

# Main build logic
case "${1:-all}" in
    desktop)
        build_desktop
        ;;
    web)
        build_web
        ;;
    android)
        build_android
        ;;
    ios)
        build_ios
        ;;
    all)
        build_desktop
        build_web
        ;;
    *)
        echo "Usage: $0 {desktop|web|android|ios|all}"
        exit 1
        ;;
esac

echo -e "${GREEN}Build complete!${NC}"

# --- Makefile ---
# Save this as 'Makefile' in your project root

.PHONY: all desktop web run run-web test clean doc fmt check

# Default target
all: desktop web

# Build desktop application
desktop:
	@echo "Building desktop application..."
	@cargo build --release --features desktop

# Build web application
web:
	@echo "Building web application..."
	@wasm-pack build --target web --out-dir dist/pkg

# Run desktop application
run:
	@echo "Running desktop application..."
	@cargo run --features desktop

# Run web application (requires Python 3)
run-web: web
	@echo "Starting web server on http://localhost:8000"
	@cd dist && python3 -m http.server 8000

# Run tests
test:
	@echo "Running tests..."
	@cargo test --all-features

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf dist/pkg

# Generate documentation
doc:
	@echo "Generating documentation..."
	@cargo doc --no-deps --open

# Format code
fmt:
	@echo "Formatting code..."
	@cargo fmt

# Check code (lint)
check:
	@echo "Checking code..."
	@cargo clippy -- -D warnings

# Install development dependencies
install-deps:
	@echo "Installing development dependencies..."
	@rustup target add wasm32-unknown-unknown
	@cargo install wasm-pack
	@cargo install cargo-watch

# Development mode with auto-reload
dev:
	@cargo watch -x "run --features desktop"

# Development mode for web
dev-web:
	@cargo watch -s "make web && make run-web"

# --- .gitignore ---
# Save this as '.gitignore' in your project root

# Rust
/target/
**/*.rs.bk
Cargo.lock

# Web build
/dist/
/pkg/
*.wasm

# IDE
.idea/
*.iml
.vscode/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Local data
*.db
*.db-journal
*.db-wal

# Logs
*.log

# Environment
.env
.env.local

# Testing
tarpaulin-report.html
cobertura.xml

# --- rust-toolchain.toml ---
# Save this as 'rust-toolchain.toml' in your project root

[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
targets = ["wasm32-unknown-unknown"]

# --- .cargo/config.toml ---
# Save this as '.cargo/config.toml' in your project root

[target.wasm32-unknown-unknown]
runner = "wasm-bindgen-test-runner"

[build]
target-dir = "target"

[net]
git-fetch-with-cli = true

[profile.release]
lto = true
opt-level = 3
codegen-units = 1

[profile.wasm-release]
inherits = "release"
opt-level = "z"
lto = "fat"
panic = "abort"