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

```bash
# openSUSE (Tumbleweed / Leap)
sudo zypper install -y gcc pkgconf-pkg-config pam-devel fprintd
```

```bash
# Alpine Linux
sudo apk add build-base pkgconf linux-pam-dev fprintd
```

```bash
# Gentoo
sudo emerge --ask virtual/pkgconfig sys-libs/pam sys-auth/fprintd
```

## Build

```bash
cargo build --release
```

Build output directory:

- `target/release`

Main artifacts:

- `libpaw.so`
- `libpaw_password.so`
- `libpaw_fingerprint.so`
- `password_hasher`
