#!/bin/bash
# fix-workspace-deps.sh

echo "Finding all workspace dependencies..."

# Create a temporary file to collect all dependencies
DEPS_FILE="/tmp/workspace_deps.txt"
> "$DEPS_FILE"

# Find all workspace = true dependencies
for cargo_file in ironsights_core/Cargo.toml ironsights-desktop/Cargo.toml ironsights-mobile/Cargo.toml; do
    if [ -f "$cargo_file" ]; then
        echo "Checking $cargo_file..."
        grep "workspace = true" "$cargo_file" | while read -r line; do
            # Extract the dependency name
            dep=$(echo "$line" | sed -n 's/^\([a-z_-]*\).*/\1/p')
            if [ ! -z "$dep" ]; then
                echo "$dep" >> "$DEPS_FILE"
            fi
        done
    fi
done

# Get unique dependencies
UNIQUE_DEPS=$(sort "$DEPS_FILE" | uniq)

echo "Found dependencies that need workspace definitions:"
echo "$UNIQUE_DEPS"

# Now create a complete Cargo.toml with all dependencies
cat > Cargo.toml << 'EOF'
[workspace]
members = [
    "ironsights_core",
    "ironsights-desktop",
    "ironsights-mobile"
]

[workspace.package]
version = "1.0.0"
edition = "2021"
authors = ["IronSights Contributors"]
license = "MIT"

[workspace.dependencies]
# Core dependencies
anyhow = "1.0"
base64 = "0.22"
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.5", features = ["v4", "serde"] }

# GUI dependencies
egui = "0.28"
eframe = { version = "0.28", default-features = false }
egui_extras = { version = "0.28", features = ["image"] }
egui_plot = "0.28"

# Database
rusqlite = { version = "0.31", features = ["bundled"] }
r2d2 = "0.8"
r2d2_sqlite = "0.24"

# Async
tokio = { version = "1.35", features = ["rt-multi-thread", "macros", "time"] }

# Nostr
nostr-sdk = { version = "0.35", default-features = false, features = ["sqlite", "nip04"] }

# Android
android-activity = "0.5"
android_logger = "0.14"
ndk = "0.9"
ndk-context = "0.1"
ndk-glue = "0.7"
jni = "0.21"

# iOS
objc = "0.2"
objc-foundation = "0.1"
core-foundation = "0.9"
mobile-entry-point = "0.1"

# Logging
log = "0.4"
env_logger = "0.11"

# File/System
dirs = "5.0"
image = { version = "0.24", default-features = false, features = ["png", "jpeg"] }
rfd = "0.14"
webbrowser = "0.8"

# Utilities
lazy_static = "1.4"
once_cell = "1.19"

# Web/WASM
wasm-bindgen = "0.2"
web-sys = "0.3"
getrandom = { version = "0.2", features = ["js"] }

# Hardware
serialport = { version = "4.3", optional = true }

# Build dependencies
winres = "0.1"
EOF

echo "✓ Created comprehensive Cargo.toml"

# Try to build
cargo build