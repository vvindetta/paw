# Build PAW from source

## Install build dependencies

```bash
# Debian / Ubuntu
sudo apt install -y build-essential pkg-config libpam0g-dev fprintd
```

```bash
# Fedora / RHEL / CentOS Stream / AlmaLinux / Rocky Linux
sudo dnf install -y gcc pkgconf-pkg-config pam-devel fprintd
```

```bash
# Arch Linux
sudo pacman -S --needed base-devel pkgconf pam fprintd
```

## Build

```bash
cargo build --release
cp paw_config.toml target/release/paw_config.toml
```

Build output directory:

- `target/release`

Main artifacts:

- `libpaw.so`
- `libpaw_password.so`
- `libpaw_fingerprint.so`
- `password_hasher`
- `paw_config.toml`
