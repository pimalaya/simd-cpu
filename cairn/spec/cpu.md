---
cairn: spec
capability: cpu
status: current
---

# CPU

The SIMD features of the running CPU as one `SimdCpu` token, which an application detects and installs once for every SIMD codec to read, or hands to a codec explicitly.

### Requirement: Only features the CPU has

A `SimdCpu` SHALL hold only features the running CPU supports, so that a codec MAY run their instructions on the strength of the token alone. It SHALL be built only from `SimdCpu::NONE`; `SimdCpu::BASELINE`, the features the build target guarantees through `cfg(target_feature)`; `SimdCpu::detect`, behind the `std` feature; `SimdCpu::with`, unsafe; and `SimdCpu::installed` and `SimdCpu::current`, which return a token an install vouched for. `SimdCpu::without` SHALL remove one feature and leave the others, and is safe.

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

The library SHALL build `#![no_std]` with no dependency under every feature combination, `std` serving only `SimdCpu::detect` and `SimdCpu::install_detected`, and SHALL allocate nothing.

### Requirement: One token per process, installed by the application

The crate SHALL hold one process-wide token, which only `SimdCpu::install` (unsafe) and `SimdCpu::install_detected` (behind `std`) set, once: a second install SHALL change nothing and return the token already installed. No function of the crate SHALL install or detect on its own. `SimdCpu::installed` SHALL return the installed token or none, and `SimdCpu::current` the installed token or `SimdCpu::BASELINE`. On a target without 32-bit atomics, the install functions SHALL not exist, `installed` SHALL return none and `current` `SimdCpu::BASELINE`.

#### Scenario: An application that installs
- GIVEN a process whose `main` calls `SimdCpu::install_detected()` first
- WHEN a codec built afterwards reads `SimdCpu::current()`, and a second `install_detected` is called
- THEN the codec gets the detected token, and the second call returns it as an error, the installed token unchanged

#### Scenario: An application that forgot
- GIVEN a process that never installs
- WHEN a codec reads `SimdCpu::current()`
- THEN it gets `SimdCpu::BASELINE`, with no error and no log
