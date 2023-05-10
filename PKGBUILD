# Maintainer: Radek Podgorny <radek@podgorny.cz>
pkgname=rpbatmanager-git
provides=('rpbatmanager')
conflicts=('rpbatmanager')
pkgver=r13.1239d8d
pkgrel=1
pkgdesc="Radek Podgorny's battery manager"
arch=('x86_64')
url="https://github.com/rpodgorny/rpbatmanager-rs"
#license=('PSF')
#depends=('babashka')
makedepends=('git' 'cargo')
#options=(!emptydirs)
#backup=('etc/rpbatmanager.conf')
source=("$pkgname::git+https://github.com/rpodgorny/rpbatmanager-rs")
md5sums=('SKIP')

pkgver() {
	cd "$srcdir/$pkgname"
	printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
	cd "$srcdir/$pkgname"
	cargo build --release --locked
}

package() {
	cd "$srcdir/$pkgname"
	install -D -m 0755 -t $pkgdir/usr/bin/ target/release/rpbatmanager
	install -D -m 0644 -t $pkgdir/usr/lib/systemd/system/ rpbatmanager.service
}
