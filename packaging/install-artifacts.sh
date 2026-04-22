#!/usr/bin/env bash
# Install PAW artifacts from a cargo target directory into a staging root.
#
# Usage:
#   install-artifacts.sh <target-dir> <staging-root> [pam-security-dir]
#
# Example (build a .deb layout):
#   ./install-artifacts.sh target/release debian/paw /usr/lib/x86_64-linux-gnu/security
set -euo pipefail

target_dir=${1:?target-dir is required}
staging_root=${2:?staging-root is required}
pam_security_dir=${3:-/usr/lib/security}

pam_dst="${staging_root}${pam_security_dir}"
paw_submodule_dst="${pam_dst}/paw"
bin_dst="${staging_root}/usr/bin"

install -d -m 0755 "${pam_dst}"
install -d -m 0755 "${paw_submodule_dst}"
install -d -m 0755 "${bin_dst}"

install -m 0644 "${target_dir}/libhost.so"             "${pam_dst}/pam_paw.so"
install -m 0644 "${target_dir}/libpaw_password.so"     "${paw_submodule_dst}/paw_password.so"
install -m 0644 "${target_dir}/libpaw_fingerprint.so"  "${paw_submodule_dst}/paw_fingerprint.so"
install -m 0755 "${target_dir}/password_hasher"        "${bin_dst}/paw-password-hasher"

echo "Installed PAW artifacts under ${staging_root}"
