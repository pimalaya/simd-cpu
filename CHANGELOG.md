# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-11-03

### Added

- Added `SimdCpu`, the SIMD features of the running CPU as a token.

  `NONE` holds no feature, `BASELINE` the build target's. The safe `without` removes a feature, the unsafe `with` adds one the caller vouches for.

- Added `SimdCpu::detect`, the running CPU's features, behind the `std` feature.
- Added `SimdFeature`: SSSE3, AVX2, AVX-512 VBMI and VBMI2, BMI1, BMI2, LZCNT and POPCNT on x86, NEON on aarch64.
- Added `SimdCpu::install_detected`, behind the `std` feature, and the unsafe `SimdCpu::install`, setting the process-wide token once.
- Added `SimdCpu::installed` and `SimdCpu::current`, reading the process-wide token, `current` falling back on `BASELINE`.

[Unreleased]: https://github.com/pimalaya/simd-cpu/compare/v0.1.0..HEAD
[0.1.0]: https://github.com/pimalaya/simd-cpu/compare/root..v0.1.0
