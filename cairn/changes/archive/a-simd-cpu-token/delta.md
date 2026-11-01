---
cairn: delta
change: a-simd-cpu-token
---

## ADDED Requirements

### Requirement: Only features the CPU has

A `SimdCpu` SHALL hold only features the running CPU supports, so that a codec MAY run their instructions on the strength of the token alone. It SHALL be built only from `SimdCpu::NONE`; `SimdCpu::BASELINE`, the features the build target guarantees through `cfg(target_feature)`; `SimdCpu::detect`, behind the `std` feature; and `SimdCpu::with`, unsafe. `SimdCpu::without` SHALL remove one feature and leave the others, and is safe.

#### Scenario: A default x86_64 build
- GIVEN the crate built for x86_64 with no `-C target-feature`
- WHEN `SimdCpu::BASELINE` is read
- THEN it holds no feature

#### Scenario: An aarch64 build without std
- GIVEN the crate built for aarch64 with no feature
- WHEN `SimdCpu::BASELINE` is read
- THEN it holds NEON, and no detection code is compiled

### Requirement: Detection holds the baseline

`SimdCpu::detect` SHALL hold every feature the running CPU supports among the `SimdFeature` variants of its architecture, at least those of `SimdCpu::BASELINE`.

#### Scenario: A native build
- GIVEN the crate built with `-C target-cpu=native`
- WHEN the running CPU is detected
- THEN the detected token holds every feature of `SimdCpu::BASELINE`

### Requirement: Features, not backends

The crate SHALL name CPU features only, each after its Rust target feature, and never a codec backend: each codec SHALL pick its own backend from them. `SimdFeature` SHALL stay `#[non_exhaustive]`, so that a feature a new codec needs is added without a breaking change.

### Requirement: No dependency, no std

The library SHALL build `#![no_std]` with no dependency under every feature combination, `std` serving only `SimdCpu::detect`.

## MODIFIED Requirements

## REMOVED Requirements
