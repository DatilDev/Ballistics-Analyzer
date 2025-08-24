#!/bin/bash
# fix-workspace.sh

echo "Fixing workspace configuration..."

# 1. Check and rename directories if needed
if [ -d "ballistics_core" ]; then
    mv ballistics_core ironsights_core
    echo "Renamed ballistics_core → ironsights_core"
fi

if [ -d "ballistics-desktop" ]; then
    mv ballistics-desktop ironsights-desktop
    echo "Renamed ballistics-desktop → ironsights-desktop"
fi

if [ -d "ballistics-mobile" ]; then
    mv ballistics-mobile ironsights-mobile
    echo "Renamed ballistics-mobile → ironsights-mobile"
fi

# 2. Fix Cargo.toml
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
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.5", features = ["v4", "serde"] }

# GUI dependencies
egui = "0.28"
eframe = "0.28"
egui_extras = "0.28"
egui_plot = "0.28"

# Database
rusqlite = { version = "0.31", features = ["bundled"] }
r2d2 = "0.8"
r2d2_sqlite = "0.24"

# Async
tokio = { version = "1.35", features = ["rt-multi-thread", "macros"] }

# Nostr
nostr-sdk = { version = "0.35", default-features = false }

# Android
android-activity = "0.5"
android_logger = "0.14"
ndk = "0.9"
ndk-context = "0.1"

# Logging
log = "0.4"
env_logger = "0.11"
EOF

echo "✓ Fixed Cargo.toml"

# 3. Clean and rebuild
cargo clean
cargo build

echo "✓ Workspace fixed!"