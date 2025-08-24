#!/bin/bash
set -e

echo "Testing build configurations..."

# Test minimal build
echo "Testing minimal build..."
cargo check -p ballistics-desktop --no-default-features --features minimal

# Test arch-linux build
echo "Testing Arch Linux build..."
cargo check -p ballistics-desktop --features arch-linux

# Test Android target if available
if command -v rustup &> /dev/null; then
    if rustup target list --installed | grep -q "aarch64-linux-android"; then
        echo "Testing Android build..."
        cargo check -p ballistics-mobile --target aarch64-linux-android
    fi
fi

echo "✓ All build configurations pass"