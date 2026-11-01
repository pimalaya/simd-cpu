# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added `SimdCpu`, the SIMD features of the running CPU as a token: `NONE`, `BASELINE` from the build target, `detect` behind the `std` feature, the unsafe `with` and the safe `without`.

- Added `SimdFeature`, the features the Pimalaya codecs run: SSSE3, AVX2, AVX-512 VBMI and VBMI2, BMI1, BMI2, LZCNT and POPCNT on x86, NEON on aarch64.
