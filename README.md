# PAW is a multi-factor authentication module for Linux [BETA]
**More precisely, it is a PAM module with submodules that you can mix in any combination.**

![screenshot.jpg](demo.jpg)

It helps protect your primary system password in untrusted environments.
You can define policies such as `LONG_PASSWORD OR (SHORT_PASSWORD + FINGERPRINT)` and require a minimum number of successful factors. It's like in GrapheneOS.


## Why not just stock PAM config?

PAM is Linux's standard authentication framework, but practical `OR` flows are hard to express in typical configs.  
PAW adds a dedicated policy layer for this.

---

### PAW is in beta. Roadmap:
  - Minimum number of successfully passed modules option
  - Rewrite Roadmap
  
  Modules:
  - ~~Password~~
  - ~~Fingerprint~~
  - Face ID
  - NFC
  - Trusted Bluetooth/Wi-Fi networks

---

## Install

### 1. Get release binaries

- [GitHub](https://github.com/vvindetta/paw/releases) / [Codeberg](https://codeberg.org/vvindetta/paw/releases)

Release files:

- `libpaw.so` - main PAM module
- `libpaw_password.so` - password submodule
- `libpaw_fingerprint.so` - fingerprint submodule
- `password_hasher` - helper tool to generate `/etc/paw_shadow`

### Build from source (optional)

See [BUILD.md](BUILD.md) for dependencies and build commands.

### 2. Resolve PAM module directory

```bash
# Debian / Ubuntu (amd64)
PAM_DIR=/usr/lib/x86_64-linux-gnu/security

# Debian / Ubuntu (arm64)
PAM_DIR=/usr/lib/aarch64-linux-gnu/security

# Older Debian / Ubuntu releases
PAM_DIR=/lib/x86_64-linux-gnu/security

# Arch / Alpine
PAM_DIR=/usr/lib/security

# Fedora / RHEL / CentOS / Alma / Rocky
PAM_DIR=/usr/lib64/security

# openSUSE
PAM_DIR=/lib64/security
```

### 3. Install PAW and submodules

```bash
# If you downloaded release binaries into the current directory:
sudo install -D -m 0644 ./libpaw.so "$PAM_DIR/libpaw.so"
sudo install -D -m 0644 ./libpaw_password.so "$PAM_DIR/paw/libpaw_password.so"
sudo install -D -m 0644 ./libpaw_fingerprint.so "$PAM_DIR/paw/libpaw_fingerprint.so"
```

```bash
# If you built from source:
sudo install -D -m 0644 target/release/libpaw.so "$PAM_DIR/libpaw.so"
sudo install -D -m 0644 target/release/libpaw_password.so "$PAM_DIR/paw/libpaw_password.so"
sudo install -D -m 0644 target/release/libpaw_fingerprint.so "$PAM_DIR/paw/libpaw_fingerprint.so"
```

### 4. Configure `/etc/paw.conf`

```bash
sudo touch /etc/paw.conf
sudo $EDITOR /etc/paw.conf
```

Format (one module per line):

```text
path_to_paw_module attempts_number
```

`attempts_number` defaults to `3` if omitted. Module order in this file is execution order.

Use the same base path as your selected `PAM_DIR` value:

```bash
echo "PAM_DIR=$PAM_DIR"
```

Config example:
```text
YOUR_PAM_DIR/paw/libpaw_password.so
YOUR_PAM_DIR/paw/libpaw_fingerprint.so 5
```

## Modules configuration

### Password module

```bash
# If you downloaded release binaries:
./password_hasher "YOUR_LONG_PASSWORD" | sudo tee /etc/paw_shadow >/dev/null

# If you built from source:
./target/release/password_hasher "YOUR_LONG_PASSWORD" | sudo tee /etc/paw_shadow >/dev/null

sudo chmod 600 /etc/paw_shadow
sudo chown root:root /etc/paw_shadow
```

### Fingerprint module

Enroll a fingerprint for this user:
```bash
fprintd-enroll "$USER"
```

Distro-specific commands:

```bash
# Fedora / RHEL / Alma / Rocky 
sudo authselect disable-feature with-fingerprint
authselect apply-changes
```

```bash
# Debian / Ubuntu
sudo pam-auth-update --disable fprintd
```

```bash
# openSUSE
sudo pam-config -q --fprintd
sudo pam-config -d --fprintd
```

```bash
# Arch
btw, you're an Arch user!
```

### Test with `pamtester` before changing real services

Install `pamtester` with your package manager, then create `/etc/pam.d/paw_testing`:

```text
auth    required    libpaw.so
account required    pam_permit.so
```

Run:

```bash
pamtester paw_testing "$USER" authenticate
```

### Enable in a real PAM service

After successful testing, add `libpaw.so` to the relevant `/etc/pam.d/*` service file (for example `/etc/pam.d/sudo`):

```text
auth    sufficient    libpaw.so
```

Control flag priority:

- PAM evaluates rules top to bottom.
- `required`: must pass; failure is returned after the stack is processed.
- `requisite`: like `required`, but fails immediately.
- `sufficient`: if it passes (and no earlier `required`/`requisite` failed), PAM returns success immediately.
- `optional`: usually ignored unless it is the only rule affecting that stack.

[PAM docs](https://man7.org/linux/man-pages/man5/pam.d.5.html)

## Contributing

Contributions are welcome: code review, and new authentication modules. Please check the [roadmap](#paw-is-in-beta-roadmap).
