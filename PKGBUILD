# Maintainer: smtdfc <me.smtdfc@gmail.com>

pkgname=bakeryos-update
pkgver=0.1.2
pkgrel=1
pkgdesc="A GTK4 graphical application for BakeryOS that checks for and installs system updates. It lists packages with newer versions, lets users deselect individual packages, and requests administrator authentication when an update starts."
arch=('x86_64')
url="https://github.com/bakeryos-project/bakeryos-update"
license=('GPL-3.0-or-later')
depends=('gtk4' 'libadwaita' 'glib2')
makedepends=('meson' 'rust' 'cargo' 'blueprint-compiler')
source=()
sha256sums=()

build() {
    cd $startdir

    arch-meson . build
    meson compile -C build
}

package() {
    cd $startdir
    meson install -C build --no-rebuild --destdir "$pkgdir"
}
