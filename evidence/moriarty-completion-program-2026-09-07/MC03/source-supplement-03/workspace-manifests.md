# Complete workspace package manifests

Exact pinned source. Dependency categories and features are preserved verbatim; no Cargo invocation was used. Root Cargo.toml and aggregation are included along with all four formerly omitted package manifests.


## Cargo.toml

SHA256 `23a26a010b70c7ccec3f7db7ddff13735f5b4fe9de83950729f4bb98eba12a6d`.

```toml
[workspace]
resolver = "2"

members = ["proofs", "curves", "circuits", "aggregation", "zk_stdlib"]

[workspace.package]
license-file = "LICENSE"
edition = "2024"
rust-version = "1.90.0"

[workspace.dependencies]
ff = "0.13"
group = "0.13"
rand = { version = "0.8" }
rand_core = { version = "0.6", default-features = false }
tracing = "0.1"
blake2b_simd = "1"                                        # MSRV 1.66.0
rand_chacha = "0.3"
rayon = "1.10"
serde = { version = "1.0", features = ["derive"] }
serde_derive = { version = "1" }
serde_json = { version = "1" }
num-bigint = { version = "0.4" }
sha2 = "0.10.9"
sha3 = "0.12.0"
ripemd = "0.2.0"

# Avoids type unification errors when third-party repos are integrated to midnight-circuits. Since
# both such repos and midnight-circuits import midnight-proofs, they end up with an isomorphic
# (but different) copy of their midnight-proofs dependency. The below version enforces that they
# both get use the local crate.
[patch.crates-io]
midnight-proofs = { path = "proofs" }
midnight-curves = { path = "curves" }
midnight-circuits = { path = "circuits" }

# TODO: Try removing this.
[patch."https://github.com/midnightntwrk/midnight-zk"]
midnight-proofs = { path = "proofs" }
midnight-curves = { path = "curves" }
midnight-circuits = { path = "circuits" }

## Benchmarks

[profile.bench]
opt-level = 3
debug = false
debug-assertions = false
overflow-checks = false
lto = true
incremental = false
codegen-units = 1

# Allows to extract debug symbols to do mem/perf profiling while aiming for maximum performance
[profile.heap_profiling]
inherits = "release"
opt-level = 3
debug = 1
```


## aggregation/Cargo.toml

SHA256 `4e8382b6fad723cc538dbad5bee90e69fa7440577fc69373a31260366ed6fece`.

```toml
[package]
name = "midnight-aggregation"
version = "0.1.2"
edition.workspace = true
rust-version.workspace = true
description = "Toolkit for proof aggregation of midnight-proofs"
license = "MIT OR Apache-2.0"
categories = ["cryptography"]
keywords = ["halo", "proofs", "zkp", "zkSNARKs", "aggregation"]

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs", "--html-in-header", "katex-header.html"]

[dependencies]
group = { workspace = true }
ff = { workspace = true }
midnight-curves = "0.3.0"
midnight-proofs = { version = "0.8.0", default-features = false, features = [
    "circuit-params",
    "committed-instances",
] }
midnight-circuits = { version = "7.0.0", default-features = false, features = [
    "testing",
] }
midnight-zk-stdlib = { path = "../zk_stdlib", version = "2.1.0" }

rand = { workspace = true }
blake2b_simd = { workspace = true }
sha2 = { workspace = true }

[dev-dependencies]
rand_chacha = { workspace = true }

[[example]]
name = "ivc"
required-features = ["truncated-challenges"]

[[example]]
name = "single_circuit_aggregation"
required-features = ["truncated-challenges"]

[[example]]
name = "multi_circuit_aggregation"
required-features = ["truncated-challenges"]

[features]
truncated-challenges = [
    "midnight-proofs/truncated-challenges",
    "midnight-circuits/truncated-challenges",
]
single-h-commitment = [
    "midnight-proofs/single-h-commitment",
    "midnight-circuits/single-h-commitment",
    "midnight-zk-stdlib/single-h-commitment",
]
fewer-point-sets = [
    "midnight-proofs/fewer-point-sets",
    "midnight-circuits/fewer-point-sets",
]
```


## proofs/Cargo.toml

SHA256 `3ac04788e8bda2e1a6cf9f9964886139f8ea446c61920e77dfcc00624a1f6743`.

```toml
[package]
name = "midnight-proofs"
version = "0.8.0"
edition.workspace = true
rust-version.workspace = true
description = """
Fast PLONK-based zero-knowledge proving system
"""
license = "MIT OR Apache-2.0"
readme = "README.md"
categories = ["cryptography"]
keywords = ["halo", "proofs", "zkp", "zkSNARKs"]

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs", "--html-in-header", "katex-header.html"]

[[bench]]
name = "commit_zk"
harness = false

[[bench]]
name = "plonk"
harness = false

[[bench]]
name = "zswap_output"
required-features = ["bench-internal"]
harness = false

[dependencies]
ff = { workspace = true }
group = { workspace = true }
midnight-curves = "0.3.0"
rand_core = { workspace = true, features = ["getrandom"] }
tracing = { workspace = true }
blake2b_simd = { workspace = true }
rand_chacha = { workspace = true }
itertools = "0.15"
serde = { workspace = true }
serde_derive = { workspace = true }
rayon = { workspace = true }
criterion = { version = "0.8", optional = true }
rustc-hash = "2"

# Developer tooling dependencies
tabbycat = { version = "0.1", features = ["attributes"], optional = true }
num-bigint = { workspace = true }

[dev-dependencies]
assert_matches = "1.5"
proptest = "1"
serde_json = { workspace = true }
midnight-circuits = { path = "../circuits" }
midnight-zk-stdlib = { path = "../zk_stdlib" }
sha2 = { workspace = true }
rand = { workspace = true }
criterion = "0.8"

[target.'cfg(all(target_arch = "wasm32", target_os = "unknown"))'.dependencies]
getrandom = { version = "0.2", features = ["js"] }

[target.'cfg(ci_build)'.dependencies]
midnight-curves = { version = "0.3.0", features = [
  "portable",
] }

[features]
default = ["committed-instances"]
circuit-params = []
dev-curves = ["midnight-curves/dev-curves"]

# This feature truncates challenges to half the size of the scalar field.
truncated-challenges = []

# This feature adds a new argument to prover and verifier functions, which
# represents instances in committed form (e.g. in the form of a polynomial
# commitment instead of a vector of scalars).
# The verifier may not know what the content of the commitment is, but can
# be sure that the content is constrained according to the circuit rules.
#
# This feature is very powerful for proving statements on committed data.
committed-instances = []

# Feature to expose internal prover types and benchmarking utilities.
bench-internal = ["criterion"]

# Feature to commit to the quotient polynomial H as a single polynomial
# commitment instead of splitting it into degree-(n-1) limbs. Requires the
# KZG parameters to support polynomials of degree up to (d-1)*(n-1), where d
# is the maximum constraint degree and n is the circuit domain size (i.e., the
# params must be generated with k >= log2(n * (d-1))).
single-h-commitment = []

# Enables a heuristic to reduce the number of distinct multi-open point sets,
# lowering the verifier MSM length. This can be particularly beneficial for
# in-circuit verifiers. The heuristic works by adding dummy queries for some
# polynomials, which may slightly increase proof size.
fewer-point-sets = []

[lib]
bench = false
```


## circuits/Cargo.toml

SHA256 `9bc2ec291559f01293b5862277fed08ef551e4eca31f7fc9cfc08546b142147f`.

```toml
[package]
name = "midnight-circuits"
version = "7.1.0"
edition.workspace = true
rust-version.workspace = true
license-file.workspace = true
description = "Circuit and gadget implementations for Midnight zero-knowledge proofs"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
group = { workspace = true }
ff = { workspace = true }
rustc-hash = "2"
midnight-curves = "0.3.0"
midnight-proofs = { version = "0.8.0", default-features = false, features = [
    "circuit-params",
    "committed-instances",
] }

serde = { workspace = true, optional = true }
serde_json = { workspace = true, optional = true }
sha2 = { workspace = true }
ripemd = { workspace = true }
rand = { workspace = true }

num-bigint = { version = "0.4" }
num-traits = "0.2"
num-integer = "0.1"
subtle = "2.6"
base64 = "0.13.1"
lazy_static = "1.5.0"
goldenfile = { version = "=1.8.0", optional = true }
rand_chacha = { workspace = true, optional = true }

[dev-dependencies]
criterion = "0.8"
rand_chacha = "0.3.1"
goldenfile = "=1.8.0"
num-bigint = { workspace = true, features = ["rand"] }
serde_json = { workspace = true }
itertools = "0.15"
dhat = "0.3"

[target.'cfg(ci_build)'.dependencies]
midnight-curves = { version = "0.3.0", features = ["portable"] }

[features]
default = []
# Feature that exposes some testing functionality, such as load from scratch or instantiable
testing = ["num-bigint/rand", "goldenfile", "serde_json", "rand_chacha"]
heap_profiling = []
# Enable truncated challenges. This makes recursive proofs more efficient
truncated-challenges = ["midnight-proofs/truncated-challenges"]
# Commit to the quotient polynomial H as a single commitment (see midnight-proofs)
single-h-commitment = ["midnight-proofs/single-h-commitment"]
# Enables a heuristic to reduce the number of distinct multi-open point sets,
# lowering the verifier MSM length (see midnight-proofs for details).
fewer-point-sets = ["midnight-proofs/fewer-point-sets"]
# Enable development curves (BN256, Pasta)
dev-curves = ["midnight-curves/dev-curves", "midnight-proofs/dev-curves"]


[lib]
bench = false

[[bench]]
name = "poseidon_cpu"
harness = false
```


## curves/Cargo.toml

SHA256 `24db8b1531f4b85ae02312e4a02e7fe70da685adf3fc20a57cb4e0b00df7da4d`.

```toml
[package]
name = "midnight-curves"
description = "Implementation of the BLS12-381, JubJub, secp256k1, secp256r1 and Curve25519 curves."
version = "0.3.0"
authors = ["dignifiedquire <me@dignifiedquire.com>", "Midnight team"]
edition.workspace = true
rust-version.workspace = true
license = "MIT/Apache-2.0"
repository = "https://github.com/midnightntwrk/midnight-zk/"
categories = ["cryptography", "algorithms"]
readme = "README.md"

[package.metadata.docs.rs]
rustdoc-args = ["--html-in-header", "katex-header.html"]

[dependencies]
blst = { version = "0.3.15", default-features = true }
pairing = { version = "0.23" }
subtle = "2.6"
bitvec = "1.0.1"
byte-slice-cast = "1.2.3"
num-bigint = "0.4.6"

rand_core = { workspace = true }
ff = { workspace = true }
group = { workspace = true, features = ["tests"] }
rayon = "1.10"
paste = "1.0"
lazy_static = "1.4"

# For curve25519 implementation
curve25519-dalek = { version = "4.1.3", features = ["group"] }

# For BN256 (dev-curves only)
halo2derive = { version = "0.2.0", optional = true }

# Needed for feature unification of `generic-array` required by k256/p256
# point serialization; not referenced directly.
sha2 = "0.10"
digest = "0.10"

# k256 for secp256k1
k256 = { version = "0.13.4", features = ["expose-field", "bits", "alloc"], default-features = false }

# p256 for secp256r1 / NIST P-256
p256 = { version = "0.13.2", features = ["expose-field", "bits", "alloc", "arithmetic"], default-features = false }
primeorder = "0.13"

[dev-dependencies]
rand_xorshift = "0.3.0"
rand = "0.8"
criterion = { version = "0.8", features = ["html_reports"] }
ark-std = "0.6"
halo2derive = "0.2.0"
[target.'cfg(all(target_arch = "wasm32", target_os = "unknown"))'.dependencies]
getrandom = { version = "0.2", features = ["js"] }

[features]
default = ["std"]
std = []
portable = ["blst/portable"]
dev-curves = ["dep:halo2derive"]

[[bench]]
name = "field_arith"
harness = false

[[bench]]
name = "ec"
harness = false

[[bench]]
name = "msm"
harness = false

[[bench]]
name = "pairing"
harness = false
```


## zk_stdlib/Cargo.toml

SHA256 `c5b408102a9814ebab04c6b81d21b494f33edaf4f8f8a7644c3f6ea87b802c5f`.

```toml
[package]
name = "midnight-zk-stdlib"
version = "2.2.0"
edition.workspace = true
rust-version.workspace = true
license-file.workspace = true
description = "Standard library of circuits and utilities for Midnight zero-knowledge proofs"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
group = { workspace = true }
ff = { workspace = true }
blake2b_simd = { workspace = true }
midnight-circuits = { version = "7.0.0", features = ["testing"] }
midnight-curves = { version = "0.3.0" }
midnight-proofs = { version = "0.8.0", default-features = false, features = [
    "circuit-params",
    "committed-instances",
] }

keccak_sha3 = { package = "sha3-circuit", git = "https://github.com/alexandroszacharakis8/sha3-circuit", rev = "1a3ef7d" }
blake2b = { package = "blake2b_halo2", git = "https://github.com/eryxcoop/blake2b_halo2", rev = "257ae18" }

serde = { workspace = true, optional = true }
sha2 = { workspace = true }
rand = { workspace = true }
rayon = { workspace = true }

num-bigint = { version = "0.4" }
num-traits = "0.2"
base64 = "0.13.1"
bincode = "2.0"
hex-literal = "1.1"

[dev-dependencies]
criterion = "0.8"
rand_chacha = { workspace = true }
goldenfile = "=1.8.0"
num-bigint = { workspace = true, features = ["rand"] }
serde_json = { workspace = true }
itertools = "0.15"
dhat = "0.3"
bellman = "0.14.0"
serial_test = "3.2.0"
sha3 = "0.12.0"
blake2 = "0.10.6"

[target.'cfg(ci_build)'.dependencies]
midnight-curves = { version = "0.3.0", features = ["portable"] }

[features]
default = []
# Feature that exposes some testing functionality, such as load from scratch or instantiable
testing = ["num-bigint/rand"]
heap_profiling = []
# Enable truncated challenges. This makes recursive proofs more efficient
truncated-challenges = [
    "midnight-proofs/truncated-challenges",
    "midnight-circuits/truncated-challenges",
]
# Commit to the quotient polynomial H as a single commitment (see midnight-proofs)
single-h-commitment = [
    "midnight-proofs/single-h-commitment",
    "midnight-circuits/single-h-commitment",
]

[lib]
bench = false

[[bench]]
name = "verify"
harness = false

[[example]]
name = "cred_full"
path = "examples/identity/jwt/full_credential.rs"

[[example]]
name = "cred_enrollment"
path = "examples/identity/jwt/enrollment.rs"

[[example]]
name = "cred_property"
path = "examples/identity/jwt/property_check.rs"
```
