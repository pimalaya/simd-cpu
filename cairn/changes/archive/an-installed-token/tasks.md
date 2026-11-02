---
cairn: tasks
change: an-installed-token
---

- [x] The static `AtomicU32`, `install`, `install_detected`, `installed` and `current`, gated on `target_has_atomic = "32"` where they need it
- [x] One test of the lifecycle: none installed, `current` at the baseline, a first install, a second refused with the first returned
- [x] Docs: the crate header and each function say the application installs once, before building any codec, and libraries never install; the two-copies caveat
- [x] Builds: no feature, `std`, and targets without 32-bit atomics (thumbv6m-none-eabi, riscv32imc-unknown-none-elf); clippy, aarch64 too
- [x] CHANGELOG.md, README.md, spec folded, log entry, change archived
- [ ] Follow-up changes proposed in base64-codec, quoted-printable-codec and mime-rs (`cpu: Option<SimdCpu>` resolved at construction), then the rev bumped in each
