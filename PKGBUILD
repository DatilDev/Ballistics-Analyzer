pkgname=ballistics-analyzer
pkgver=1.0.0
pkgrel=1
pkgdesc="Professional ballistics calculator - Arch Linux build"
arch=('x86_64' 'aarch64')
url="https://github.com/yourusername/ballistics-analyzer"
license=('MIT')
depends=('gtk3' 'webkit2gtk' 'openssl')
makedepends=('rust' 'cargo' 'pkg-config')
source=("$pkgname-$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$srcdir"
    cargo build --release -p ballistics-desktop --features arch-linux
}

package() {
    cd "$srcdir"
    install -Dm755 "target/release/ballistics-analyzer" "$pkgdir/usr/bin/ballistics-analyzer"
    install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
}