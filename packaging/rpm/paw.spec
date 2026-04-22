Name:           paw
Version:        0.1.0
Release:        1%{?dist}
Summary:        Multi-factor PAM authentication with pluggable modules

License:        See LICENSE
URL:            https://github.com/vvindetta/paw
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  gcc
BuildRequires:  pam-devel
Requires:       pam

%global _pam_moduledir %{_libdir}/security
%global _paw_submoduledir %{_pam_moduledir}/paw

%description
PAW provides a PAM module and a set of submodules that allow combining
multiple authentication factors (password, fingerprint, etc.) for GNU/Linux
authentication. It is designed to avoid exposing the main system password in
unsafe environments by combining a secondary password with additional factors.

%prep
%setup -q

%build
cargo build --release --workspace

%check
cargo test --release --workspace

%install
rm -rf %{buildroot}
install -d -m 0755 %{buildroot}%{_pam_moduledir}
install -d -m 0755 %{buildroot}%{_paw_submoduledir}
install -d -m 0755 %{buildroot}%{_bindir}
install -m 0644 target/release/libhost.so            %{buildroot}%{_pam_moduledir}/pam_paw.so
install -m 0644 target/release/libpaw_password.so    %{buildroot}%{_paw_submoduledir}/paw_password.so
install -m 0644 target/release/libpaw_fingerprint.so %{buildroot}%{_paw_submoduledir}/paw_fingerprint.so
install -m 0755 target/release/password_hasher       %{buildroot}%{_bindir}/paw-password-hasher

%files
%license LICENSE
%doc README.md
%{_pam_moduledir}/pam_paw.so
%dir %{_paw_submoduledir}
%{_paw_submoduledir}/paw_password.so
%{_paw_submoduledir}/paw_fingerprint.so
%{_bindir}/paw-password-hasher

%changelog
* Thu Jan 01 2026 PAW Maintainers <maintainers@example.com> - 0.1.0-1
- Initial RPM packaging.
