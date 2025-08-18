#!/bin/bash
# scripts/create-pkgbuild.sh - Create PKGBUILD for Arch Linux

set -e

VERSION="${1:-1.0.0}"
RELEASE="${2:-1}"

mkdir -p pkg

cat > pkg/PKGBUILD << EOF
# Maintainer: Ballistics Analyzer Contributors <support@datildev.com>
pkgname=ballistics-analyzer
pkgver=$VERSION
pkgrel=$RELEASE
pkgdesc="Professional ballistics calculator with privacy-first design"
arch=('x86_64' 'aarch64')
url="https://github.com/DatilDev/Ballistics-Analyzer"
license=('MIT')
depends=(
    'gtk3'
    'webkit2gtk'
    'libappindicator-gtk3'
    'librsvg'
    'libx11'
    'libxcb'
    'libxkbcommon'
    'mesa'
    'wayland'
    'openssl'
)
makedepends=(
    'rust'
    'cargo'
    'pkg-config'
    'git'
)
provides=('ballistics-analyzer')
conflicts=('ballistics-analyzer-git' 'ballistics-analyzer-bin')
source=("https://github.com/DatilDev/Ballistics-Analyzer/archive/v\${pkgver}.tar.gz")
sha256sums=('SKIP')

prepare() {
    cd "\$srcdir/Ballistics-Analyzer-\${pkgver}"
    
    # Update Cargo.lock if needed
    cargo update
}

build() {
    cd "\$srcdir/Ballistics-Analyzer-\${pkgver}"
    
    # Build core library
    cd ballistics_core
    cargo build --release --locked
    
    # Build desktop application
    cd ../ballistics-desktop
    cargo build --release --locked
}

check() {
    cd "\$srcdir/Ballistics-Analyzer-\${pkgver}"
    
    # Run tests
    cargo test --release --locked
}

package() {
    cd "\$srcdir/Ballistics-Analyzer-\${pkgver}"
    
    # Install binary
    install -Dm755 "target/release/ballistics-analyzer" "\$pkgdir/usr/bin/ballistics-analyzer"
    
    # Install desktop entry
    install -Dm644 "pkg/ballistics-analyzer.desktop" "\$pkgdir/usr/share/applications/ballistics-analyzer.desktop"
    
    # Install icon
    install -Dm644 "ballistics-desktop/assets/icon.png" "\$pkgdir/usr/share/icons/hicolor/256x256/apps/ballistics-analyzer.png"
    
    # Install different icon sizes
    for size in 16 32 64 128; do
        install -Dm644 "ballistics-desktop/assets/icon.png" \
            "\$pkgdir/usr/share/icons/hicolor/\${size}x\${size}/apps/ballistics-analyzer.png"
    done
    
    # Install documentation
    install -Dm644 "README.md" "\$pkgdir/usr/share/doc/ballistics-analyzer/README.md"
    install -Dm644 "PRIVACY_POLICY.md" "\$pkgdir/usr/share/doc/ballistics-analyzer/PRIVACY_POLICY.md"
    
    # Install license
    install -Dm644 "LICENSE" "\$pkgdir/usr/share/licenses/ballistics-analyzer/LICENSE"
    
    # Install man page (if exists)
    if [ -f "docs/ballistics-analyzer.1" ]; then
        install -Dm644 "docs/ballistics-analyzer.1" "\$pkgdir/usr/share/man/man1/ballistics-analyzer.1"
    fi
}
EOF

# Create desktop entry
cat > pkg/ballistics-analyzer.desktop << EOF
[Desktop Entry]
Name=Ballistics Analyzer
Comment=Professional ballistics calculator with privacy-first design
GenericName=Ballistics Calculator
Exec=ballistics-analyzer
Icon=ballistics-analyzer
Type=Application
Categories=Education;Science;Utility;
Terminal=false
StartupNotify=true
Keywords=ballistics;calculator;shooting;trajectory;privacy;
Actions=new-calculation;

[Desktop Action new-calculation]
Name=New Calculation
Exec=ballistics-analyzer --new
EOF

# Create .SRCINFO for AUR
cat > pkg/.SRCINFO << EOF
pkgbase = ballistics-analyzer
	pkgdesc = Professional ballistics calculator with privacy-first design
	pkgver = $VERSION
	pkgrel = $RELEASE
	url = https://github.com/DatilDev/Ballistics-Analyzer
	arch = x86_64
	arch = aarch64
	license = MIT
	makedepends = rust
	makedepends = cargo
	makedepends = pkg-config
	makedepends = git
	depends = gtk3
	depends = webkit2gtk
	depends = libappindicator-gtk3
	depends = librsvg
	depends = libx11
	depends = libxcb
	depends = libxkbcommon
	depends = mesa
	depends = wayland
	depends = openssl
	provides = ballistics-analyzer
	conflicts = ballistics-analyzer-git
	conflicts = ballistics-analyzer-bin
	source = https://github.com/DatilDev/Ballistics-Analyzer/archive/v$VERSION.tar.gz
	sha256sums = SKIP

pkgname = ballistics-analyzer
EOF

echo "✓ PKGBUILD created in pkg/PKGBUILD"
echo "To build the package, run: cd pkg && makepkg -si"