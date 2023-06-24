# COPY INTO OWN DIRECTORY AND RUN makepkg -si
pkgname=swde
pkgver=0.6.0.r0.g6267b2a
pkgrel=1
pkgdesc="A little wrapper around the Shopware devenv environment that gets out of your way"
url="https://github.com/cyl3x/shopware-devenv"
arch=(any)
license=("MIT")
makedepends=(cargo)
provides=("${pkgname}")
source=("git+$url.git#tag=v$pkgver")
options=()
md5sums=('SKIP')

pkgver() {
    (git describe --long --tags || echo "$pkgver") | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g'
}

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}
