---
cairn: change
id: a-simd-cpu-token
status: landed
created: 2026-11-01
---

# A SIMD CPU token

## Why

base64-codec and quoted-printable-codec each detected the CPU behind a `std` feature of their own, with a token of their own; base32, UTF-7 and later codecs would add more. An application using several codecs detected once per codec and held one token per codec.

## What

`SimdCpu`, a `Copy` token of the SIMD features the running CPU has, which cannot be forged: `NONE`, `BASELINE` from `cfg(target_feature)`, `detect` behind `std`, `has`, the unsafe `with` and the safe `without`. `SimdFeature` names the features the codecs run today, after their Rust target features, `#[non_exhaustive]`. The token holds features rather than backends, since each codec needs a different set: base64-codec AVX-512 VBMI, AVX2, SSSE3 or NEON; quoted-printable-codec AVX-512 VBMI2 with VBMI, BMI1, BMI2, LZCNT and POPCNT. Each codec turns the features into its own backend once, outside its hot path.

Settled with the user on 2026-11-01: the crate is named simd-cpu. Detection stays behind `std`, through the standard library's detection macros, as the approved report proposed, rather than a CPUID reader of its own, which x86 would allow without std.
