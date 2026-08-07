# Security policy

LXDB reads binary datasets and external dictionary sources. Please do not
publish crafted datasets, malicious source archives, or other security issues
in a public issue before maintainers have had a chance to investigate.

Report vulnerabilities privately through the repository's GitHub security
advisory flow. Include the affected crate, Rust toolchain, reproduction steps,
and a minimal sample when possible. Reports without a reproducible example are
still welcome, but may take longer to triage.

Until a fix is available, avoid opening untrusted `.lxdb` files outside the
validated storage APIs.
