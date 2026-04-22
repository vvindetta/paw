# Building and installing PAW

This document describes how to build `paw` from source and how to generate
installable packages for each supported Linux distribution. All packaging
recipes live under [`packaging/`](../packaging) and are exercised in CI by the
[`Build packages`](../.github/workflows/packages.yml) workflow, which
publishes the resulting artifacts for every commit.

- [From source](#from-source)
- [Debian / Ubuntu (`.deb`)](#debian--ubuntu-deb)
- [Fedora / RHEL (`.rpm`)](#fedora--rhel-rpm)
- [Arch Linux / AUR (`PKGBUILD`)](#arch-linux--aur-pkgbuild)
- [File layout](#file-layout)

## From source

```bash
# Debian/Ubuntu
sudo apt-get install -y cargo rustc libpam0g-dev

# Fedora
sudo dnf install -y cargo rust pam-devel

# Arch
sudo pacman -S --needed rust cargo pam

git clone https://github.com/vvindetta/paw.git
cd paw
cargo build --release --workspace
cargo test --workspace
```

Built artifacts are placed in `target/release/` as `libhost.so`,
`libpaw_password.so`, `libpaw_fingerprint.so`, and the `password_hasher`
helper binary. The [`packaging/install-artifacts.sh`](../packaging/install-artifacts.sh)
script installs them into a staging root with the final names (`pam_paw.so`,
`paw_password.so`, `paw_fingerprint.so`, `paw-password-hasher`).

## Debian / Ubuntu (`.deb`)

```bash
sudo apt-get install -y build-essential debhelper devscripts fakeroot \
    cargo rustc libpam0g-dev

cp -r packaging/debian debian
dpkg-buildpackage -us -uc -b
# Resulting paw_*.deb ends up one directory above the source tree.
```

See [`packaging/debian/README.md`](../packaging/debian/README.md) for details.

## Fedora / RHEL (`.rpm`)

```bash
sudo dnf install -y rpm-build rpmdevtools cargo rust pam-devel gcc
rpmdev-setuptree
git archive --prefix=paw-0.1.0/ -o ~/rpmbuild/SOURCES/paw-0.1.0.tar.gz HEAD
cp packaging/rpm/paw.spec ~/rpmbuild/SPECS/paw.spec
rpmbuild -ba ~/rpmbuild/SPECS/paw.spec
```

See [`packaging/rpm/README.md`](../packaging/rpm/README.md) for details.

## Arch Linux / AUR (`PKGBUILD`)

```bash
sudo pacman -S --needed base-devel rust cargo pam
cp packaging/arch/PKGBUILD PKGBUILD
makepkg -sf
```

See [`packaging/arch/README.md`](../packaging/arch/README.md) for details,
including notes on publishing to the AUR.

## File layout

All packages install the same set of files using the distribution-appropriate
multiarch/library directory:

| File                                        | Role                                  |
| ------------------------------------------- | ------------------------------------- |
| `<pam-security-dir>/pam_paw.so`             | Main PAM entry point (`libhost.so`).  |
| `<pam-security-dir>/paw/paw_password.so`    | Password submodule.                   |
| `<pam-security-dir>/paw/paw_fingerprint.so` | Fingerprint submodule.                |
| `/usr/bin/paw-password-hasher`              | CLI helper to generate Argon2 hashes. |

Typical values for `<pam-security-dir>`:

- Debian/Ubuntu: `/usr/lib/<multiarch-triplet>/security` (for example
  `/usr/lib/x86_64-linux-gnu/security`).
- Fedora/RHEL:   `%{_libdir}/security` — usually `/usr/lib64/security`.
- Arch Linux:    `/usr/lib/security`.
