#Maintainer: Datil <datildev@tuta.com>
#PGP Key: F064B97F19488354D7D51F35B1B3491444097676
pkgname=ironsights
pkgver=1.0.0
pkgrel=1
pkgdesc="Professional ballistics calculator for precision shooting"
arch=('x86_64' 'aarch64')
url="https://github.com/yourusername/ironsights"
license=('MIT')
depends=('gtk3' 'webkit2gtk' 'libusb')
makedepends=('rust' 'cargo' 'pkg-config')
source=("$pkgname-$pkgver.tar.gz::https://github.com/yourusername/$pkgname/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')
validpgpkeys=('YOUR_PGP_FINGERPRINT_HERE')  # Add your full fingerprint

prepare() {
    cd "$pkgname-$pkgver"
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$pkgname-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release -p ironsights-desktop --features arch-linux
}

check() {
    cd "$pkgname-$pkgver"
    cargo test --frozen -p ironsights_core
}

package() {
    cd "$pkgname-$pkgver"
    
    # Install binary
    install -Dm755 "target/release/ironsights" "$pkgdir/usr/bin/ironsights"
    
    # Install desktop file
    install -Dm644 "assets/ironsights.desktop" "$pkgdir/usr/share/applications/ironsights.desktop"
    
    # Install icon
    install -Dm644 "assets/icon.png" "$pkgdir/usr/share/pixmaps/ironsights.png"
    
    # Install license
    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    
    # Install PGP public key
    install -Dm644 "ironsights-signing-key.asc" "$pkgdir/usr/share/$pkgname/signing-key.asc"
}
