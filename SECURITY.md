# Security policy

## Reporting a vulnerability

Report it privately, through GitHub's
[private vulnerability reporting](https://github.com/PQCrypta/pqc-binary-format/security/advisories/new)
for this repository, or by email to contact@pqcrypta.com. Please do not open a
public issue for it: a report in the open reaches everyone parsing untrusted
input with this crate before a fix does.

A useful report names the version or commit, the binding (Rust, C, Python,
JavaScript/WebAssembly or Go), and the input that triggers it: the bytes given
to `PqcBinaryFormat::from_bytes()` (or a binding's equivalent) and what happens
(a panic, an allocation out of proportion to the input, metadata read wrongly,
a checksum accepted that should not be). A fuzzer's crash file is welcome.

## What happens next

- An acknowledgement within three days.
- The finding reproduced and assessed, and a fix developed in private.
- A release carrying the fix to every registry the crate is published on
  (crates.io, PyPI, npm and the Go module proxy), and a GitHub security
  advisory that names the affected and fixed versions and credits the reporter
  (unless they would rather not be named).

## Supported versions

Fixes are made on `main` and released as the next version; the latest release
is the supported one. Releases are tagged `v<version>` and listed under
[Releases](https://github.com/PQCrypta/pqc-binary-format/releases).
