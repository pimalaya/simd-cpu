---
cairn: log
change: a-simd-cpu-token
landed: 2026-11-01
---

# A SIMD CPU token

The user asked whether the CPU detection base64-codec and quoted-printable-codec each carried should move to a small shared crate, ahead of base32, UTF-7 and later codecs. Settled with them: yes, named simd-cpu, holding features rather than backends.

What landed:

- `SimdCpu`, a `Copy` token with a private bit set: `NONE`, `BASELINE` read from `cfg(target_feature)` (the method of simdutf8, credited in ATTRIBUTION.md), `detect` behind `std` through `is_x86_feature_detected!` and `is_aarch64_feature_detected!`, the baseline on other targets, `has`, the unsafe `with`, the safe `without`, and `Default` as `BASELINE`.
- `SimdFeature`, `#[non_exhaustive]`: SSSE3, AVX2, AVX-512 VBMI, AVX-512 VBMI2, BMI1, BMI2, LZCNT, POPCNT and NEON, the union of what the two codecs run. Variants are not gated by architecture: a feature of another architecture is never held.
- One test, detection holding the baseline, which pairs each feature with the same name in both lists when run with `-C target-cpu=native`.
- base64-codec and quoted-printable-codec moved to it the same day, each through its change a-shared-cpu-token, quoted-printable-codec once its AVX-512 pass had landed.

Checked: tests on x86_64 with and without `-C target-cpu=native`; clippy with `-D warnings` for all targets and for the library without default features, on x86_64 and aarch64-unknown-linux-musl; the test under qemu-aarch64 on neoverse-n1, where `BASELINE` holds NEON and detection agrees; missing docs and rustdoc with `-D warnings`. The repository is not under git yet.

Capabilities moved: cpu (new).
