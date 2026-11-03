---
cairn: log
change: the-first-release
landed: 2026-11-03
---

# The first release

Prepared simd-cpu 0.1.0, the first crates.io release, ahead of base64-codec 0.1.0 and quoted-printable-codec 0.1.0, which depend on it: crates.io refuses a git dependency, so simd-cpu is published first and the codecs then take it from the registry.

The manifest, package.nix and default.nix read 0.1.0, CHANGELOG.md has its 0.1.0 section, and SECURITY.md supports 0.1.x. The README gained the docs.rs badge, an Examples section and the Contributing link; src/lib.rs opens with its crate attributes before the header, per the Pimalaya guidelines.

No behaviour moved. Capabilities moved: none.
