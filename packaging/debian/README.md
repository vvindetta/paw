# Debian / Ubuntu packaging

This directory contains the Debian packaging recipe for `paw`. To build a
`.deb` package from a source checkout:

```bash
# Install build dependencies
sudo apt-get install -y build-essential debhelper devscripts cargo rustc libpam0g-dev

# Stage the packaging files at the repository root
cp -r packaging/debian debian

# Build the package
dpkg-buildpackage -us -uc -b

# The resulting .deb will be written one directory above the source tree
ls ../paw_*.deb
```

The `debian/rules` file invokes `cargo build --release --workspace` and then
calls `packaging/install-artifacts.sh` to lay out the files under
`/usr/lib/<multiarch>/security/` and `/usr/bin/`.
