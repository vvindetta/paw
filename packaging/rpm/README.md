# Fedora / RHEL packaging

This directory contains an RPM spec file for building `paw` on Fedora-based
distributions.

## Build locally

```bash
sudo dnf install -y rpm-build rpmdevtools cargo rust pam-devel gcc
rpmdev-setuptree

# Create a source tarball from the checkout
git archive --prefix=paw-0.1.0/ -o ~/rpmbuild/SOURCES/paw-0.1.0.tar.gz HEAD

# Copy the spec into place
cp packaging/rpm/paw.spec ~/rpmbuild/SPECS/paw.spec

# Build
rpmbuild -ba ~/rpmbuild/SPECS/paw.spec

# The resulting RPM is under ~/rpmbuild/RPMS/<arch>/
ls ~/rpmbuild/RPMS/*/paw-*.rpm
```

The spec file runs `cargo build --release --workspace`, executes the test
suite via `%check`, and installs the PAM modules under
`%{_libdir}/security/` with submodules under `%{_libdir}/security/paw/`.
