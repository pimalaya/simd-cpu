---
cairn: change
id: an-installed-token
status: landed
created: 2026-11-02
---

# An installed token

## Why

Every SIMD codec takes a `SimdCpu`, and the application detects it once. Today the application must then hand the token down through every layer between it and the codecs. mime-rs shows the cost: on 2026-11-02 it grew a `cpu` field on `MimeParseOptions`, `MimeBuildOptions` and every `MimeEntity` (which now takes part in entity equality), and RFC 2047 words fall back to `SimdCpu::BASELINE` because header readers cannot reach an entity. io-pimdir, himalaya's crates and every other layer would grow the same plumbing.

CPU features are a fact about the process with one right answer, not a policy choice, so one process-wide token loses nothing. The standard library already caches its detection in a static, and memchr keeps its chosen kernel in one.

## What

Settled with the user on 2026-11-02: the crate provides the machinery, and the application alone installs, once, like rustls's `CryptoProvider::install_default`. No library installs or detects on its own, so detection stays the application's, as settled on 2026-10-31.

- A private `static` `AtomicU32` holds the installed token: its feature bits and one bit marking it installed. It is an atomic integer rather than a `OnceLock`, so it works without std and without alloc.
- `unsafe fn SimdCpu::install(self) -> Result<(), SimdCpu>`: installs the token once; a second call changes nothing and returns the token already installed. It is unsafe for the reason `with` is: codecs run instructions on the token's word alone, so a wrong token installed is undefined behaviour in every consumer.
- `fn SimdCpu::install_detected() -> Result<SimdCpu, SimdCpu>`, behind `std`: installs `SimdCpu::detect()` and returns it, or returns the token already installed. It is safe, since detection cannot lie. This is the one call an application makes, first thing in `main`.
- `fn SimdCpu::installed() -> Option<SimdCpu>`: the installed token, or none.
- `fn SimdCpu::current() -> SimdCpu`: the installed token, or `SimdCpu::BASELINE` when none is. An application that forgot to install gets safe and slower backends, never an error or a log: on aarch64 the baseline already holds NEON, and on x86_64 the codecs' portable backends. The fallback policy lives here, once, rather than in each codec.
- Relaxed ordering throughout: the token publishes no other memory, and a reader seeing the token either before or after an install gets a sound one.
- `SimdCpu::default()` stays `BASELINE`: it is deterministic and keeps meaning what the build target guarantees.
- Targets without 32-bit atomics have no SIMD feature to install: there `install` and `install_detected` are not compiled, `installed` returns none and `current` returns `BASELINE`.

The docs say who calls what: the application installs, once, before building any codec; libraries call `current` or take an explicit token, and never install. They also say that two copies of simd-cpu in one build hold two statics, so an install reaches only the copy it was called on.

### How codecs use it (their own changes, after this one)

The options a caller writes hold `cpu: Option<SimdCpu>`: `Some` is the caller's explicit choice, for tests, benches and no_std applications; `None` takes `SimdCpu::current()`. A codec resolves that once, when its value is built, and keeps the resolved backend, so no encode or decode call reads the static: per-call resolution would add an atomic load and a backend match to every call, and lose the backend being known to the compiler on small inputs, which the codecs' passes tune at a few cycles. A build that moves a run-time read into a constructor is checked by diffing release assembly function by function, since a run-time `new` inside `Default` once pushed quoted-printable-codec's decoder constructor past the cross-crate inline threshold.

- base64-codec and quoted-printable-codec: their default constructors take `current()`.
- mime-rs: `cpu: Option<SimdCpu>` in `MimeParseOptions` and `MimeBuildOptions`, resolved per leaf decode or per build; the `cpu` field of `MimeEntity` goes, and RFC 2047 words take `current()` too.

## Risks

- Two copies of simd-cpu in a build (two git revs, or two 0.0.x versions once on crates.io) split the static silently. Mitigation: keep the crate tiny, and take it to 1.0 early so one semver line serves every codec.
- An install after a codec value was built does not reach that value, which resolved `current()` before. The docs say to install first; a configuration captures its provider when it is built in rustls too.
- Tests share the process-wide static: the install lifecycle is tested in one test function, and every other test passes explicit tokens.
