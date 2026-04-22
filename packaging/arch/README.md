# Arch Linux / AUR packaging

This directory contains a `PKGBUILD` for building `paw` on Arch Linux.

## Build locally from a source checkout

```bash
sudo pacman -S --needed base-devel rust cargo pam

# From the repository root, stage the PKGBUILD alongside the source:
cp packaging/arch/PKGBUILD ./PKGBUILD

# Build (use -f to allow building from a dirty working tree)
makepkg -sf

# The resulting package will be named paw-<version>-<rel>-<arch>.pkg.tar.zst
ls paw-*.pkg.tar.*
```

## Publishing to AUR

Once uploaded to the AUR, the `source=()` array should be replaced with the
upstream release tarball URL plus the matching `sha256sums=()` entry, so
`makepkg` can fetch the sources automatically.
