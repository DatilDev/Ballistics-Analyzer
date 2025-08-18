#!/bin/bash
# scripts/create-macos-bundle.sh - Create macOS app bundle

set -e

BINARY_PATH="$1"
if [ -z "$BINARY_PATH" ]; then
    BINARY_PATH="target/release/ballistics-analyzer"
fi

APP_NAME="Ballistics Analyzer"
BUNDLE_NAME="$APP_NAME.app"
BUNDLE_DIR="build/macos/$BUNDLE_NAME"

echo "Creating macOS app bundle..."

# Create bundle structure
mkdir -p "$BUNDLE_DIR/Contents/MacOS"
mkdir -p "$BUNDLE_DIR/Contents/Resources"
mkdir -p "$BUNDLE_DIR/Contents/Frameworks"

# Copy binary
cp "$BINARY_PATH" "$BUNDLE_DIR/Contents/MacOS/ballistics-analyzer"

# Copy assets
cp -r ballistics-desktop/assets/* "$BUNDLE_DIR/Contents/Resources/"

# Create Info.plist
cat > "$BUNDLE_DIR/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleDisplayName</key>
    <string>Ballistics Analyzer</string>
    <key>CFBundleExecutable</key>
    <string>ballistics-analyzer</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>com.ballistics.analyzer</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>Ballistics Analyzer</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.14</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
    <key>NSRequiresAquaSystemAppearance</key>
    <false/>
    <key>NSCameraUsageDescription</key>
    <string>Ballistics Analyzer needs camera access to attach photos to calculations (stored locally only)</string>
    <key>NSLocationWhenInUseUsageDescription</key>
    <string>Ballistics Analyzer uses your location for environmental calculations (stored locally only)</string>
    <key>NSBluetoothAlwaysUsageDescription</key>
    <string>Ballistics Analyzer needs Bluetooth to connect to rangefinders and weather meters</string>
</dict>
</plist>
EOF

# Create PkgInfo
echo "APPL????" > "$BUNDLE_DIR/Contents/PkgInfo"

# Convert icon to icns if PNG exists
if [ -f "ballistics-desktop/assets/icon.png" ]; then
    echo "Creating macOS icon..."
    
    # Create iconset
    ICONSET="$BUNDLE_DIR/Contents/Resources/AppIcon.iconset"
    mkdir -p "$ICONSET"
    
    # Generate different sizes
    sips -z 16 16     ballistics-desktop/assets/icon.png --out "$ICONSET/icon_16x16.png" > /dev/null 2>&1
    sips -z 32 32     ballistics-desktop/assets/icon.png --out "$ICONSET/icon_16x16@2x.png" > /dev/null 2>&1
    sips -z 32 32     ballistics-desktop/assets/icon.png --out "$ICONSET/icon_32x32.png" > /dev/null 2>&1
    sips -z 64 64     ballistics-desktop/assets/icon.png --out "$ICONSET/icon_32x32@2x.png" > /dev/null 2>&1
    sips -z 128 128   ballistics-desktop/assets/icon.png --out "$ICONSET/icon_128x128.png" > /dev/null 2>&1
    sips -z 256 256   ballistics-desktop/assets/icon.png --out "$ICONSET/icon_128x128@2x.png" > /dev/null 2>&1
    sips -z 256 256   ballistics-desktop/assets/icon.png --out "$ICONSET/icon_256x256.png" > /dev/null 2>&1
    sips -z 512 512   ballistics-desktop/assets/icon.png --out "$ICONSET/icon_256x256@2x.png" > /dev/null 2>&1
    sips -z 512 512   ballistics-desktop/assets/icon.png --out "$ICONSET/icon_512x512.png" > /dev/null 2>&1
    sips -z 1024 1024 ballistics-desktop/assets/icon.png --out "$ICONSET/icon_512x512@2x.png" > /dev/null 2>&1
    
    # Create icns
    iconutil -c icns "$ICONSET" -o "$BUNDLE_DIR/Contents/Resources/AppIcon.icns"
    rm -rf "$ICONSET"
fi

# Sign the app if certificate is available
if security find-identity -p codesigning -v | grep -q "Developer ID Application"; then
    echo "Signing app bundle..."
    codesign --deep --force --verify --verbose --sign "Developer ID Application" "$BUNDLE_DIR"
    
    # Verify signature
    codesign --verify --verbose "$BUNDLE_DIR"
else
    echo "No signing certificate found. App will not be signed."
fi

# Create DMG for distribution
if command -v create-dmg &> /dev/null; then
    echo "Creating DMG..."
    create-dmg \
        --volname "Ballistics Analyzer" \
        --window-pos 200 120 \
        --window-size 600 400 \
        --icon-size 100 \
        --icon "$BUNDLE_NAME" 175 120 \
        --hide-extension "$BUNDLE_NAME" \
        --app-drop-link 425 120 \
        "build/macos/Ballistics-Analyzer.dmg" \
        "$BUNDLE_DIR"
else
    echo "create-dmg not found. Install with: brew install create-dmg"
    
    # Simple DMG creation
    echo "Creating simple DMG..."
    hdiutil create -volname "Ballistics Analyzer" -srcfolder "$BUNDLE_DIR" -ov -format UDZO "build/macos/Ballistics-Analyzer.dmg"
fi

echo "✓ macOS app bundle created: $BUNDLE_DIR"
[ -f "build/macos/Ballistics-Analyzer.dmg" ] && echo "✓ DMG created: build/macos/Ballistics-Analyzer.dmg"