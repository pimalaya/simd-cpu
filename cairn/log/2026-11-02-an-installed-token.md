---
cairn: log
change: an-installed-token
landed: 2026-11-02
---

# An installed token

Landed the process-wide token: a private `AtomicU32` holding the feature bits and a mark bit, set once by `SimdCpu::install` (unsafe) or `SimdCpu::install_detected` (behind `std`), a second install returning the token already installed, and read by `SimdCpu::installed` (none before an install) and `SimdCpu::current` (`BASELINE` before one). The application installs, first thing in `main`; no function of the crate installs or detects on its own, so detection stays the application's. Relaxed ordering throughout, the token publishing no other memory. `SimdCpu::default()` stays `BASELINE`.

Targets without 32-bit atomics, all bare-metal and none with a feature `SimdFeature` names, have no install functions; there `installed` returns none and `current` `BASELINE`. Checked on thumbv6m-none-eabi and riscv32imc-unknown-none-elf, and every target pimalaya/nix, the Android app and iOS build has 32-bit atomics.

One test runs the whole lifecycle, the static being the process's. The codecs and mime-rs move to it in their own changes: their options take `cpu: Option<SimdCpu>`, `None` meaning `current()`, resolved once when a codec value is built.

Capabilities moved: cpu (ADDED: one token per process, installed by the application; MODIFIED: only features the CPU has, no dependency and no std).
