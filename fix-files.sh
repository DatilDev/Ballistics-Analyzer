#!/bin/bash
# fix-files.sh - Fix problematic filenames for cross-platform compatibility

echo "Fixing file naming issues..."

# Remove the problematic wildcard file if it exists
if [ -f "assets/icon-*.png" ]; then
    echo "Removing invalid wildcard filename..."
    rm "assets/icon-*.png"
fi

# Create assets directory if it doesn't exist
mkdir -p assets

# Create placeholder icon files with proper names
echo "Creating placeholder icon files..."

# Create a simple placeholder icon script (1x1 transparent PNG)
create_placeholder_png() {
    local filename=$1
    # This creates a minimal valid PNG file (1x1 transparent pixel)
    printf '\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x06\x00\x00\x00\x1f\x15\xc4\x89\x00\x00\x00\rIDATx\x9cc\xf8\x0f\x00\x00\x01\x01\x00\x05\xb8\x91\x8d\x18\x00\x00\x00\x00IEND\xaeB`\x82' > "$filename"
}

# Create standard PWA icon sizes
for size in 72 96 128 144 152 192 384 512; do
    if [ ! -f "assets/icon-${size}x${size}.png" ]; then
        create_placeholder_png "assets/icon-${size}x${size}.png"
        echo "Created assets/icon-${size}x${size}.png"
    fi
done

# Create other standard icons
if [ ! -f "assets/favicon.ico" ]; then
    create_placeholder_png "assets/favicon.ico"
    echo "Created assets/favicon.ico"
fi

if [ ! -f "assets/apple-touch-icon.png" ]; then
    create_placeholder_png "assets/apple-touch-icon.png"
    echo "Created assets/apple-touch-icon.png"
fi

echo "File issues fixed!"
echo ""
echo "Note: These are placeholder icons. Replace them with actual icons for production."