#!/bin/bash
# scripts/create-appimage.sh - Create AppImage for Linux distribution

set -e

TARGET="${1:-x86_64-unknown-linux-gnu}"
VERSION="${2:-1.0.0}"

echo "Creating AppImage for Ballistics Analyzer..."

# Download AppImage tools if not present
if [ ! -f "appimagetool-x86_64.AppImage" ]; then
    echo "Downloading AppImage tools..."
    wget -q https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
    chmod +x appimagetool-x86_64.AppImage
fi

# Create AppDir structure
APPDIR="AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
mkdir -p "$APPDIR/usr/share/applications"
mkdir -p "$APPDIR/usr/share/icons/hicolor/256x256/apps"
mkdir -p "$APPDIR/usr/share/icons/hicolor/128x128/apps"
mkdir -p "$APPDIR/usr/share/icons/hicolor/64x64/apps"
mkdir -p "$APPDIR/usr/share/icons/hicolor/32x32/apps"
mkdir -p "$APPDIR/usr/share/icons/hicolor/16x16/apps"
mkdir -p "$APPDIR/usr/lib"

# Copy binary
cp "target/$TARGET/release/ballistics-analyzer" "$APPDIR/usr/bin/"

# Copy assets
cp -r ballistics-desktop/assets "$APPDIR/usr/share/"

# Create desktop entry
cat > "$APPDIR/usr/share/applications/ballistics-analyzer.desktop" << EOF
[Desktop Entry]
Name=Ballistics Analyzer
Comment=Professional ballistics calculator with privacy-first design
Exec=ballistics-analyzer
Icon=ballistics-analyzer
Type=Application
Categories=Education;Science;Utility;
Terminal=false
StartupNotify=true
MimeType=application/x-ballistics;
Keywords=ballistics;calculator;shooting;trajectory;
EOF

# Copy icons (assuming icon.png exists)
if [ -f "ballistics-desktop/assets/icon.png" ]; then
    cp ballistics-desktop/assets/icon.png "$APPDIR/usr/share/icons/hicolor/256x256/apps/ballistics-analyzer.png"
    
    # Create different sizes
    for size in 128 64 32 16; do
        convert ballistics-desktop/assets/icon.png -resize ${size}x${size} \
            "$APPDIR/usr/share/icons/hicolor/${size}x${size}/apps/ballistics-analyzer.png" 2>/dev/null || \
        cp ballistics-desktop/assets/icon.png "$APPDIR/usr/share/icons/hicolor/${size}x${size}/apps/ballistics-analyzer.png"
    done
fi

# Create AppRun script
cat > "$APPDIR/AppRun" << 'EOF'
#!/bin/bash
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"

# Privacy: Ensure app runs in isolated environment
export XDG_CONFIG_HOME="${HOME}/.config"
export XDG_DATA_HOME="${HOME}/.local/share"
export XDG_CACHE_HOME="${HOME}/.cache"

exec "${HERE}/usr/bin/ballistics-analyzer" "$@"
EOF

chmod +x "$APPDIR/AppRun"

# Link desktop and icon for AppImage
ln -sf usr/share/applications/ballistics-analyzer.desktop "$APPDIR/ballistics-analyzer.desktop"
ln -sf usr/share/icons/hicolor/256x256/apps/ballistics-analyzer.png "$APPDIR/ballistics-analyzer.png"

# Copy required libraries
echo "Copying required libraries..."
LIBS=(
    libgtk-3.so.0
    libgdk-3.so.0
    libatk-1.0.so.0
    libcairo-gobject.so.2
    libcairo.so.2
    libgdk_pixbuf-2.0.so.0
    libgio-2.0.so.0
    libglib-2.0.so.0
    libgobject-2.0.so.0
    libpango-1.0.so.0
    libpangocairo-1.0.so.0
    libharfbuzz.so.0
    libfontconfig.so.1
    libfreetype.so.6
)

for lib in "${LIBS[@]}"; do
    LIB_PATH=$(ldconfig -p | grep "$lib" | head -1 | awk '{print $NF}')
    if [ -f "$LIB_PATH" ]; then
        cp "$LIB_PATH" "$APPDIR/usr/lib/" 2>/dev/null || true
    fi
done

# Create AppImage
ARCH=$(uname -m)
OUTPUT_NAME="Ballistics-Analyzer-${VERSION}-${ARCH}.AppImage"

./appimagetool-x86_64.AppImage "$APPDIR" "$OUTPUT_NAME"

echo "✓ AppImage created: $OUTPUT_NAME"

# Verify AppImage
chmod +x "$OUTPUT_NAME"
./"$OUTPUT_NAME" --version || true

# Clean up
rm -rf "$APPDIR"