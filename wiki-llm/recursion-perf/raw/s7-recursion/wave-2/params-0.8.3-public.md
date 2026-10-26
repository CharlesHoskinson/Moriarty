# Midnight proofs docs.rs source attempt redirected to crate page
> Source: https://docs.rs/midnight-proofs/0.8.3/src/midnight_proofs/poly/kzg/params.rs.html
> Collected: 2026-10-26
> Published: Unknown

SOURCE — Redirected to the crate landing page; this capture is not evidence of parameter downsizing code. Use params-0.8.3-source.md instead.

midnight-proofs 0.8.3 - Docs.rs

[Docs.rs](/)

* [midnight-proofs-0.8.3](/crate/midnight-proofs/0.8.3 "Fast PLONK-based zero-knowledge proving system
  ")

* [docs.rs](#)
  + [About docs.rs](/about)
  + [Badges](/about/badges)
  + [Builds](/about/builds)
  + [Metadata](/about/metadata)
  + [Shorthand URLs](/about/redirections)
  + [Download](/about/download)
  + [Rustdoc JSON](/about/rustdoc-json)
  + [Build queue](/releases/queue)
  + [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

* [Rust](#)
  + [Rust website](https://www.rust-lang.org/)
  + [The Book](https://doc.rust-lang.org/book/)
  + [Standard Library API Reference](https://doc.rust-lang.org/std/)
  + [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  + [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  + [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

midnight-proofs 0.8.3
=====================

Fast PLONK-based zero-knowledge proving system

* [Crate](/crate/midnight-proofs/0.8.3)
* [Source](/crate/midnight-proofs/0.8.3/source/)
* [Builds](/crate/midnight-proofs/0.8.3/builds)
* [Feature flags](/crate/midnight-proofs/0.8.3/features)

* Size
* Source code size: 906.5 kB
  This is the summed size of all the files inside the crates.io package for this release.
* Ø build duration
* all releases: 15s
  Average build duration of **successful** builds in releases after 2024-10-23.
* Links
* [crates.io](https://crates.io/crates/midnight-proofs "See midnight-proofs on crates.io")
* Dependencies
* + [blake2b\_simd ^1
    *normal*](/crate/blake2b_simd/^1)
  + [criterion ^0.7
    *normal*
    *optional*](/crate/criterion/^0.7)
  + [ff ^0.13
    *normal*](/crate/ff/^0.13)
  + [group ^0.13
    *normal*](/crate/group/^0.13)
  + [itertools ^0.14
    *normal*](/crate/itertools/^0.14)
  + [midnight-curves ^0.3.1
    *normal*](/crate/midnight-curves/^0.3.1)
  + [num-bigint ^0.4
    *normal*](/crate/num-bigint/^0.4)
  + [rand\_chacha ^0.3
    *normal*](/crate/rand_chacha/^0.3)
  + [rand\_core ^0.6
    *normal*](/crate/rand_core/^0.6)
  + [rayon ^1.10
    *normal*](/crate/rayon/^1.10)
  + [serde ^1.0
    *normal*](/crate/serde/^1.0)
  + [serde\_derive ^1
    *normal*](/crate/serde_derive/^1)
  + [tabbycat ^0.1
    *normal*
    *optional*](/crate/tabbycat/^0.1)
  + [tracing ^0.1
    *normal*](/crate/tracing/^0.1)
  + [assert\_matches ^1.5
    *dev*](/crate/assert_matches/^1.5)
  + [criterion ^0.7
    *dev*](/crate/criterion/^0.7)
  + [proptest ^1
    *dev*](/crate/proptest/^1)
  + [rand ^0.8
    *dev*](/crate/rand/^0.8)
  + [serde\_json ^1
    *dev*](/crate/serde_json/^1)
  + [sha2 ^0.10.9
    *dev*](/crate/sha2/^0.10.9)
  + [getrandom ^0.2
    *normal*](/crate/getrandom/^0.2)
  + [midnight-curves ^0.3.0
    *normal*](/crate/midnight-curves/^0.3.0)
* Versions
* + [**0.8.3** (2026-10-01)](/crate/midnight-proofs/0.8.3 "docs.rs failed to build midnight-proofs-0.8.3")
  + [**0.8.2** (2026-07-24)](/crate/midnight-proofs/0.8.2 "docs.rs failed to build midnight-proofs-0.8.2")
  + [**0.8.1** (2026-06-22)](/crate/midnight-proofs/0.8.1 "docs.rs failed to build midnight-proofs-0.8.1")
  + [**0.8.0** (2026-04-30)](/crate/midnight-proofs/0.8.0 "docs.rs failed to build midnight-proofs-0.8.0")
  + [**0.7.3** (2026-07-23)](/crate/midnight-proofs/0.7.3 "docs.rs failed to build midnight-proofs-0.7.3")
  + [**0.7.2** (2026-06-22)](/crate/midnight-proofs/0.7.2 "docs.rs failed to build midnight-proofs-0.7.2")
  + [**0.7.1** (2026-04-01)](/crate/midnight-proofs/0.7.1 "docs.rs failed to build midnight-proofs-0.7.1")
  + [**0.7.0** (2025-12-19)](/crate/midnight-proofs/0.7.0 "docs.rs failed to build midnight-proofs-0.7.0")
  + [**0.0.0** (2025-10-28)](/crate/midnight-proofs/0.0.0)
* Owners
* [![MidnightCI](https://avatars.githubusercontent.com/u/133698592?v=4)](https://crates.io/users/MidnightCI)

docs.rs failed to build midnight-proofs-0.8.3
  
Please check the
[build logs](/crate/midnight-proofs/0.8.3/builds) for more information.
  
See [Builds](/about/builds) for ideas on how to fix a failed build,
or [Metadata](/about/metadata) for how to configure docs.rs builds.
  
If you believe this is docs.rs' fault, [open an issue](https://github.com/rust-lang/docs.rs/issues/new/choose).

Visit the last successful build:
[midnight-proofs-0.0.0](/crate/midnight-proofs/0.0.0)

Midnight Proofs
===============

[![Crates.io Version](https://img.shields.io/crates/v/midnight-proofs?label=midnight-proofs)](https://crates.io/crates/midnight-proofs)

Implementation of Plonk proof system with KZG commitments. This repo initially started
as a fork of [`halo2`](https://github.com/privacy-scaling-explorations/halo2) v0.3.0 –
by the Privacy Scaling Explorations (PSE) team, itself originally derived from the
[Zcash Sapling proving system](https://github.com/zcash/halo2).

### Summary of Changes

* **Generic proof system interface**  
  The proof system is now built on top of a generic trait, `PolynomialCommitmentScheme`, with
  a simpler and more generic interface. At the moment, we provide an implementation using
  KZG commitments with the original Halo2 [multi-open argument](https://zcash.github.io/halo2/design/proving-system/multipoint-opening.html).
* **Simplified transcript interface**  
  The `Transcript` API has been modified for easier use in and off-circuit.
* **Additive selector support**  
  Enabled additive selectors (via the `trash` argument), allowing selectors that do not increase  
  the degree of the proof system.
* **Committed instances**  
  Added support for committed instances behind the `committed-instances` feature flag.  
  See Section 4.2 of the [aPLONK paper](https://eprint.iacr.org/2022/1352.pdf) for details.
* **Truncated challenges for recursion**  
  To enable efficient recursion, Fiat–Shamir challenges can now be truncated to 128 bits  
  (via the `truncate-challenges` feature). This halves the size of scalar multiplications  
  in-circuit, resulting in considerable circuit size gains for in-circuit proof verificaiton.

Minimum Supported Rust Version
------------------------------

Requires Rust **1.85.0** or higher.

Minimum supported Rust version can be changed in the future, but it will be done with a
minor version bump.

Controlling parallelism
-----------------------

*Midnight Proofs* currently uses [rayon](https://github.com/rayon-rs/rayon) for parallel
computation. The `RAYON_NUM_THREADS` environment variable can be used to set the number of
threads.

When compiling to WASM-targets, notice that since version `1.7`, `rayon` will fallback automatically (with no need to handle features) to require `getrandom` in order to be able to work. For more info related to WASM-compilation.

See [Rayon: Usage with WebAssembly](https://github.com/rayon-rs/rayon#usage-with-webassembly) for more details.

License
-------

See root directory for Licensing. We have copied the license files of the original [Zcash Sapling proving system](https://github.com/zcash/halo2).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.