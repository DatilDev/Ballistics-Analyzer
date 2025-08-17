#!/bin/bash
# debug-build.sh - Debug and fix build issues

echo "=== Debugging Cargo Build Issues ==="
echo ""

# Check current Cargo.toml
echo "1. Checking for version conflicts in Cargo.toml..."
grep -n "eframe" Cargo.toml || echo "No eframe found in Cargo.toml"
echo ""

# Clean everything
echo "2. Cleaning build artifacts..."
cargo clean
rm -rf pkg dist target
echo "Clean complete."
echo ""

# Update dependencies
echo "3. Updating Cargo index..."
cargo update
echo ""

# Check for conflicting dependencies
echo "4. Checking dependency tree for eframe..."
cargo tree -p eframe 2>/dev/null || echo "eframe not in dependency tree yet"
echo ""

# Try building with verbose output
echo "5. Attempting build with verbose output..."
echo ""
echo "Building for desktop..."
cargo build --release --features desktop --verbose 2>&1 | head -50

echo ""
echo "=== Quick Fix Options ==="
echo ""
echo "Option 1: Use the minimal working Cargo.toml (copy from artifact above)"
echo ""
echo "Option 2: Manually fix version conflicts:"
echo "  - Ensure all egui-related crates use the same version"
echo "  - Remove any duplicate eframe entries"
echo "  - Use consistent version numbers (0.29 for all egui crates)"
echo ""
echo "Option 3: Force clean rebuild:"
echo "  rm -rf ~/.cargo/registry/cache"
echo "  rm -rf ~/.cargo/registry/index"
echo "  cargo clean"
echo "  cargo build --release --features desktop"