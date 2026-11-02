//! # simd-cpu
//!
//! The SIMD features of the running CPU as one token, which SIMD codecs take
//! to pick their backend, `no_std`, with no dependency and no I/O.
//!
//! A [`SimdCpu`] names only features the running CPU has: [`SimdCpu::BASELINE`]
//! holds the ones the build target guarantees, `SimdCpu::detect`, behind the
//! `std` feature, asks the running CPU, and the unsafe [`SimdCpu::with`] adds
//! one the caller vouches for. [`SimdCpu::without`] removes one, which is
//! always sound, to run a narrower backend.
//!
//! An application installs one token at start, usually through
//! `SimdCpu::install_detected`, first thing in `main`, and every codec built
//! afterwards reads it with [`SimdCpu::current`], picking its backend from
//! it once; a codec can also take an explicit token instead. An application
//! that installs nothing gets [`SimdCpu::BASELINE`]: safe, and as fast as the
//! build target allows. Libraries never install: the token is the
//! application's, set once, as a rustls crypto provider is.
//!
//! The installed token lives in a static of this crate, so two copies of the
//! crate in one build, from two versions, hold two tokens, and an install
//! reaches only its own.
//!
//! A codec turns the features into its own backend, so a new codec needing a
//! new feature only adds a variant to [`SimdFeature`].
//!
//! ```
//! use simd_cpu::{SimdCpu, SimdFeature};
//!
//! let cpu = SimdCpu::BASELINE.without(SimdFeature::Avx2);
//! assert!(!cpu.has(SimdFeature::Avx2));
//! assert!(!SimdCpu::NONE.has(SimdFeature::Neon));
//!
//! // NOTE: an application that installed nothing reads the baseline.
//! let cpu = SimdCpu::installed().unwrap_or(SimdCpu::BASELINE);
//! assert_eq!(SimdCpu::current(), cpu);
//! ```

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(target_has_atomic = "32")]
use core::sync::atomic::{AtomicU32, Ordering};

/// The installed token's features, with [`MARK`] once a token is installed,
/// zero before.
#[cfg(target_has_atomic = "32")]
static INSTALLED: AtomicU32 = AtomicU32::new(0);

/// The bit of [`INSTALLED`] telling an installed token from none, past every
/// feature bit, so [`SimdFeature`] holds at most 31 variants.
const MARK: u32 = 1 << 31;

/// The SIMD features of the running CPU, by construction ones it supports.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SimdCpu {
    features: u32,
}

impl SimdCpu {
    /// No feature, for the portable backends alone.
    pub const NONE: Self = Self { features: 0 };

    /// The features the build target guarantees, read from
    /// `cfg(target_feature)`: none on a default x86_64 build, NEON on aarch64.
    pub const BASELINE: Self = Self::from_flags([
        (SimdFeature::Ssse3, cfg!(target_feature = "ssse3")),
        (SimdFeature::Avx2, cfg!(target_feature = "avx2")),
        (SimdFeature::Avx512Vbmi, cfg!(target_feature = "avx512vbmi")),
        (
            SimdFeature::Avx512Vbmi2,
            cfg!(target_feature = "avx512vbmi2"),
        ),
        (SimdFeature::Bmi1, cfg!(target_feature = "bmi1")),
        (SimdFeature::Bmi2, cfg!(target_feature = "bmi2")),
        (SimdFeature::Lzcnt, cfg!(target_feature = "lzcnt")),
        (SimdFeature::Popcnt, cfg!(target_feature = "popcnt")),
        (SimdFeature::Neon, cfg!(target_feature = "neon")),
    ]);

    /// The features the running CPU supports, detected at run time, the
    /// build target's included.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn detect() -> Self {
        detected()
    }

    /// Installs this token as the process's, once: a second install changes
    /// nothing and returns the token already installed. Only the application
    /// installs, before building any codec; a codec built earlier keeps the
    /// token it read.
    ///
    /// # Safety
    ///
    /// The running CPU must support every feature of the token: every codec
    /// reading [`current`](Self::current) runs their instructions on the
    /// strength of the token alone.
    #[cfg(target_has_atomic = "32")]
    #[cfg_attr(docsrs, doc(cfg(target_has_atomic = "32")))]
    pub unsafe fn install(self) -> Result<(), Self> {
        // NOTE: the token publishes no other memory, so relaxed ordering
        // gives every reader either none or the whole token.
        INSTALLED
            .compare_exchange(
                0,
                self.features | MARK,
                Ordering::Relaxed,
                Ordering::Relaxed,
            )
            .map(|_| ())
            .map_err(|word| Self {
                features: word & !MARK,
            })
    }

    /// Detects the running CPU and installs the result as the process's
    /// token, returning it, or returns the token already installed. The one
    /// call an application makes, first thing in `main`.
    ///
    /// ```
    /// use simd_cpu::SimdCpu;
    ///
    /// let cpu = SimdCpu::install_detected().unwrap_or_else(|installed| installed);
    /// assert_eq!(SimdCpu::current(), cpu);
    /// ```
    #[cfg(all(feature = "std", target_has_atomic = "32"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn install_detected() -> Result<Self, Self> {
        let cpu = detected();

        // SAFETY: detection holds only features the running CPU supports.
        unsafe { cpu.install() }.map(|()| cpu)
    }

    /// The token the application installed, or none.
    #[inline]
    pub fn installed() -> Option<Self> {
        let word = installed_word();

        match word & MARK {
            0 => None,
            _ => Some(Self {
                features: word & !MARK,
            }),
        }
    }

    /// The token the application installed, or [`BASELINE`](Self::BASELINE)
    /// when it installed none: what a codec runs unless given a token.
    #[inline]
    pub fn current() -> Self {
        Self::installed().unwrap_or(Self::BASELINE)
    }

    /// Whether the token holds `feature`.
    pub const fn has(self, feature: SimdFeature) -> bool {
        self.features & feature.bit() != 0
    }

    /// The token with `feature` added.
    ///
    /// # Safety
    ///
    /// The running CPU must support `feature`: a codec runs its instructions
    /// on the strength of the token alone.
    pub const unsafe fn with(self, feature: SimdFeature) -> Self {
        Self {
            features: self.features | feature.bit(),
        }
    }

    /// The token without `feature`, to run a narrower backend.
    pub const fn without(self, feature: SimdFeature) -> Self {
        Self {
            features: self.features & !feature.bit(),
        }
    }

    const fn from_flags<const N: usize>(flags: [(SimdFeature, bool); N]) -> Self {
        let mut cpu = Self::NONE;
        let mut i = 0;

        while i < N {
            let (feature, present) = flags[i];

            if present {
                cpu.features |= feature.bit();
            }

            i += 1;
        }

        cpu
    }
}

impl Default for SimdCpu {
    fn default() -> Self {
        Self::BASELINE
    }
}

/// A CPU feature some SIMD codec runs, named after its Rust target feature.
///
/// A feature implies the ones its target feature implies: a CPU with
/// AVX-512 VBMI has AVX2 and SSSE3, whether or not the token holds them.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SimdFeature {
    /// SSSE3, on x86.
    Ssse3,
    /// AVX2, on x86.
    Avx2,
    /// AVX-512 VBMI, the byte permutes, on x86.
    Avx512Vbmi,
    /// AVX-512 VBMI2, the byte compress and expand, on x86.
    Avx512Vbmi2,
    /// BMI1, on x86.
    Bmi1,
    /// BMI2, on x86.
    Bmi2,
    /// LZCNT, on x86.
    Lzcnt,
    /// POPCNT, on x86.
    Popcnt,
    /// NEON, on aarch64.
    Neon,
}

impl SimdFeature {
    const fn bit(self) -> u32 {
        1 << self as u32
    }
}

#[cfg(target_has_atomic = "32")]
#[inline]
fn installed_word() -> u32 {
    INSTALLED.load(Ordering::Relaxed)
}

/// No install without 32-bit atomics, which only targets with no SIMD
/// feature to install lack.
#[cfg(not(target_has_atomic = "32"))]
#[inline]
fn installed_word() -> u32 {
    0
}

#[cfg(all(
    any(feature = "std", test),
    any(target_arch = "x86", target_arch = "x86_64")
))]
fn detected() -> SimdCpu {
    use std::arch::is_x86_feature_detected as has;

    SimdCpu::from_flags([
        (SimdFeature::Ssse3, has!("ssse3")),
        (SimdFeature::Avx2, has!("avx2")),
        (SimdFeature::Avx512Vbmi, has!("avx512vbmi")),
        (SimdFeature::Avx512Vbmi2, has!("avx512vbmi2")),
        (SimdFeature::Bmi1, has!("bmi1")),
        (SimdFeature::Bmi2, has!("bmi2")),
        (SimdFeature::Lzcnt, has!("lzcnt")),
        (SimdFeature::Popcnt, has!("popcnt")),
    ])
}

#[cfg(all(any(feature = "std", test), target_arch = "aarch64"))]
fn detected() -> SimdCpu {
    use std::arch::is_aarch64_feature_detected as has;

    SimdCpu::from_flags([(SimdFeature::Neon, has!("neon"))])
}

#[cfg(all(
    any(feature = "std", test),
    not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))
))]
fn detected() -> SimdCpu {
    SimdCpu::BASELINE
}

#[cfg(test)]
mod tests {
    use crate::{SimdCpu, SimdFeature, detected};

    /// Run with `-C target-cpu=native` too, where the baseline is not empty:
    /// it pairs every feature with the same name in both lists.
    #[test]
    fn detection_holds_the_baseline() {
        let cpu = detected();

        for feature in [
            SimdFeature::Ssse3,
            SimdFeature::Avx2,
            SimdFeature::Avx512Vbmi,
            SimdFeature::Avx512Vbmi2,
            SimdFeature::Bmi1,
            SimdFeature::Bmi2,
            SimdFeature::Lzcnt,
            SimdFeature::Popcnt,
            SimdFeature::Neon,
        ] {
            if SimdCpu::BASELINE.has(feature) {
                assert!(cpu.has(feature), "{feature:?}");
            }
        }
    }

    /// The whole install lifecycle in one test, the token being the process's.
    #[test]
    fn a_token_installs_once() {
        assert_eq!(SimdCpu::installed(), None);
        assert_eq!(SimdCpu::current(), SimdCpu::BASELINE);

        let cpu = detected().without(SimdFeature::Avx2);

        // SAFETY: detection holds only features the running CPU supports, and
        // removing one keeps that.
        assert_eq!(unsafe { cpu.install() }, Ok(()));
        assert_eq!(SimdCpu::installed(), Some(cpu));
        assert_eq!(SimdCpu::current(), cpu);

        // SAFETY: NONE holds no feature.
        assert_eq!(unsafe { SimdCpu::NONE.install() }, Err(cpu));
        assert_eq!(SimdCpu::current(), cpu);
    }
}
