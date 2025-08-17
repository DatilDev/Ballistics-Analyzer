#!/bin/bash
# cleanup-cross-platform.sh - Final version for workspace structure

echo "🧹 Cleaning up cross-platform branch..."

# 1. Remove problematic files
echo "Step 1: Removing problematic files..."
rm -f "assets/icon-*.png" 2>/dev/null && echo "  ✓ Removed invalid wildcard icon"
rm -f "assets/README.md" 2>/dev/null && echo "  ✓ Removed placeholder README"

# 2. Generate proper icon files
echo -e "\nStep 2: Generating icon files..."
if [ -f "fix-files.sh" ]; then
    ./fix-files.sh
else
    echo "  ⚠️  fix-files.sh not found - skipping icon generation"
fi

# 3. Verify workspace structure
echo -e "\nStep 3: Verifying workspace structure..."

# Check root Cargo.toml
if [ -f "Cargo.toml" ]; then
    if grep -q "\[workspace\]" Cargo.toml; then
        echo "  ✓ Root workspace configuration found"
        grep -q "ballistics_core" Cargo.toml && echo "    ✓ ballistics_core in members" || echo "    ⚠️  ballistics_core not in workspace members"
        grep -q "ballistics-wasm" Cargo.toml && echo "    ✓ ballistics-wasm in members" || echo "    ⚠️  ballistics-wasm not in workspace members"
    else
        echo "  ❌ Missing [workspace] section in root Cargo.toml"
    fi
else
    echo "  ❌ Root Cargo.toml not found"
fi

# 4. Check ballistics_core
echo -e "\nStep 4: Checking ballistics_core..."
if [ -d "ballistics_core" ]; then
    echo "  ✓ ballistics_core directory exists"
    [ -f "ballistics_core/Cargo.toml" ] && echo "    ✓ Cargo.toml found" || echo "    ❌ Cargo.toml missing"
    [ -d "ballistics_core/src" ] && echo "    ✓ src directory found" || echo "    ❌ src directory missing"
    
    # Check if build.rs is needed
    if grep -q "rusqlite" ballistics_core/Cargo.toml 2>/dev/null; then
        if [ ! -f "ballistics_core/build.rs" ]; then
            echo "    ⚠️  build.rs might be needed for SQLite bundling"
        else
            echo "    ✓ build.rs found"
        fi
    fi
else
    echo "  ❌ ballistics_core directory not found"
fi

# 5. Check ballistics-wasm
echo -e "\nStep 5: Checking ballistics-wasm..."
if [ -d "ballistics-wasm" ]; then
    echo "  ✓ ballistics-wasm directory exists"
    [ -f "ballistics-wasm/index.html" ] && echo "    ✓ index.html found" || echo "    ⚠️  index.html missing"
    [ -f "ballistics-wasm/manifest.json" ] && echo "    ✓ manifest.json found" || echo "    ⚠️  manifest.json missing"
    [ -f "ballistics-wasm/sw.js" ] && echo "    ✓ sw.js found" || echo "    ⚠️  sw.js missing"
    [ -f "ballistics-wasm/Cargo.toml" ] && echo "    ✓ Cargo.toml found" || echo "    ⚠️  Cargo.toml missing"
else
    echo "  ❌ ballistics-wasm directory not found"
fi

# 6. Summary
echo -e "\n══════════════════════════════════════"
echo "SUMMARY:"
echo "══════════════════════════════════════"
echo "✓ Problematic files cleaned"
echo "✓ Icons generated (if fix-files.sh exists)"
echo ""
echo "Workspace structure:"
echo "  root/"
echo "    ├── Cargo.toml (workspace config)"
echo "    ├── ballistics_core/ (core library)"
echo "    ├── ballistics-wasm/ (web app)"
echo "    └── assets/ (shared assets)"
echo ""
echo "No build.rs needed in root - ballistics_core handles its own build"
echo "══════════════════════════════════════"