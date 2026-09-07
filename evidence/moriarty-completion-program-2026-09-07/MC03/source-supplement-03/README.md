# Native source supplement 03: exact API and dependency reconciliation

Status: source-only evidence for independent reconsideration. Candidate
`4902a0f18bd601efa0415cb2f365bf667881217c83b0fe87763d077fafaf0a30` remains unchanged
at frozen worktree commit `85444b00bf12f84d0fbb071ae46d299a27b24ece`.
No Cargo command, Rust typecheck, MockProver, synthesis, proof, download, install,
network operation or containment campaign ran. No approval is asserted or supplied.
Only source/metadata reading, archive decoding, Git blob inspection and hashing
were performed. This supplement contains no other auditor's verdict.

## BF-1: exact source contradicts the two proposed missing dependencies

The selected cached `ppv-lite86 0.2.21` Cargo.toml states:

```toml
[dependencies.zerocopy]
version = "0.8.23"
features = ["simd"]
```

It does not request `derive`. Cached `zerocopy 0.8.56` declares `derive =
["zerocopy-derive"]`, `simd = []`, an optional zerocopy-derive dependency, and a
separate unconditional-looking entry under `target.'cfg(any())'` (an always-false
predicate). The enabled source-model feature set is `default, simd`, with no
`derive`. Therefore the cited ppv-lite86 path does not require zerocopy-derive.

Cached `blst 0.3.17` states:

```toml
[build-dependencies.cc]
version = "1.0"

[target.'cfg(target_env = "msvc")'.build-dependencies.glob]
version = "0.3"
```

The intended host and target are x86_64-unknown-linux-gnu, whose environment is gnu,
not msvc. Thus this build path requires cc and excludes glob. Exact full manifests,
archive/lock checksums and extracted-manifest hashes are in
`selected-registry-manifests.md`. These observations do not depend on recollection
of another upstream version.

`workspace-manifests.md` includes the complete root/aggregation manifests and all
four previously omitted manifests: proofs, circuits, curves and zk_stdlib. It
shows bellman only under zk_stdlib dev-dependencies, ark-std only under curves
dev-dependencies, and the actual normal optional/testing features. Dependency
crates' own tests/examples are not selected by the aggregation example command.
Aggregation's rand_chacha dev-dependency is included because it is an example.

`dependency-closure.json` records a source-only fixed-point model from the
aggregation example's normal/build/dev edges and requested truncated-challenges.
It recursively follows normal/build dependencies, proc-macro dependencies and
feature forwarding/optional activation under Linux GNU target predicates. It
conservatively unifies host and target features, so it may include additional
packages; it does not discard build or proc-macro edges. The exact source
manifests and all edges/feature sets are included for review.

Observed source-model result: 119 reachable packages, comprising 5 workspace
packages, 2 pinned Git packages and 112 checksum-verified cached registry archives;
268 edges, including 4 build edges and the root rand_chacha dev edge. All 112
registry archives occur with matching checksums in the unchanged candidate's
registry inventory. No reachable package is missing. Seven proc-macro packages
are included: bincode_derive, curve25519-dalek-derive, halo2derive, paste,
serde_derive, tracing-attributes and zeroize_derive. All 27 absent lockfile archives
are outside this conservative closure and are explicitly listed in the JSON.

This is not Cargo unit-graph output. The model assumes the stated Linux GNU
host/target, default target features and no ancestor configuration/custom rustflags
changing dependency selection. Its inputs and inference are separable from the
exact manifest facts above. A reviewer may verify those facts and the provided
closure independently; no claim is made that source inspection establishes a
successful offline build, MSRV compatibility or complete runtime feasibility.

## BF-2: the candidate uses the definitions at the exact pin

`exact-api-and-gadgets.md` and `source-excerpts.json` contain exact source blocks,
line intervals and full-file SHA256 values. The decisive definition is
`proofs/src/dev/mod.rs:752`: `MockProver::run(circuit, instance)` has two arguments.
There is no k argument. It invokes `RowSizer::min_k(circuit, instance.clone())` at
line 765. Therefore replacing this call with the conventional three-argument
Halo2 signature would be incorrect at this pin.

The other queried API definitions are present:

| Usage | Exact pinned definition |
|---|---|
| PolynomialCommitmentScheme::srs_monomial_blowup | proofs/src/poly/commitment.rs:121; KZG override proofs/src/poly/kzg/mod.rs:118 |
| KZG no-extension result | KZG override returns 1 without single-h-commitment; candidate enables truncated-challenges only |
| ParamsKZG::read_custom(reader, SerdeFormat) | proofs/src/poly/kzg/params.rs:282 |
| max_k and g_monomial_size | Params trait implementation, proofs/src/poly/kzg/params.rs:59–71; trait imported by harness |
| Accumulator, Msm, Point, BlstrsEmulation, SelfEmulation | public reexports, circuits/src/verifier/mod.rs:45–52 |
| ivc::setup | aggregation/src/ivc/setup.rs:24–67; module export in aggregation/src/ivc/mod.rs |
| IvcError::DeciderFailed / InvalidProof | aggregation/src/ivc/error.rs:9–24 |

The exact source resolves these API-existence/signature concerns. It does not
replace Rust typechecking of the complete harness or claim an executed result.

## Gadget behavior and actual mock-domain sizing

Stdlib instruction implementations delegate range, select, equality and fixed
assignment to NativeGadget/NativeChip. The supplied source blocks show:

- `select(cond,x,y)` constrains `cond*x + (1-cond)*y`, confirming argument order.
- Equality-to-fixed emits both `(x-c)*aux = 1-res` and `(x-c)*res = 0`.
- Fixed assignment uses constant assignment/copy constraints; constants are not
  merely host metadata.
- Strict bounds reduce to power-of-two decomposition and equality constraints.
  For bound 3, the gadget selects x or x-1 and checks the selected value below 2.
  Bound 2 and 2^64 use the direct power-of-two path.
- The inspected range path has no `error_if_known_and` rejection for out-of-range
  witness values. `decompose_in_variable_limbsizes` contains a `debug_assert_eq!`
  on leftover bits; with the unchanged normal `--release` profile debug assertions
  are disabled. Truncated limb reconstruction is then constrained against the
  original assigned value and produces an unsatisfied constraint for overflow.
  Unrelated structural assertions and synthesis errors can still fail the run;
  the harness never relabels such a failure as a successful rejection control.

The statement that all 299 mocks necessarily allocate k17 domains is not supported
by this source. `Some(K)` sets the MidnightCircuit configuration parameter and
its maximum decomposition bit length. MockProver separately synthesizes a
RowSizer and selects the minimum n from maximum assigned row, blinding factors,
positive rotations and minimum circuit rows. Pow2Range loads only queried bit
lengths (plus the disabled tag), not every bit length through the maximum.
Consequently actual mock k and timing require measurement; neither k17 allocation
for every mock nor a smaller concrete k is established by this source inspection.
The 1800-second campaign bound and all execution gates remain unchanged.

## Source identity, config existence and remaining boundaries

`source-identity.json` verifies every one of the 421 backend inventory entries
against actual blob bytes from Git commit
`695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7`, using ls-tree and cat-file --batch.
For every file, commit-blob SHA256, candidate backendFiles SHA256 and current
read-only working-file SHA256 agree. This checks the missing commit-to-inventory
link directly, rather than trusting the candidate's hashes alone. The frozen
candidate worktree remained clean at 85444b0 throughout this supplement.

Only existence was checked for `.cargo/config.toml` and legacy `.cargo/config`
along the current main/frozen worktree and /tmp ancestor paths. None existed on
those checked paths. No config contents or secrets were read. The eventual output
path is not selected here; the orchestrator must inspect its full ancestor chain
before a launch. The source supplement does not alter the runner or widen its
trust boundary.

SRS serialization APIs and exact pin checks are included, but this is no ceremony
trust audit. Native execution still requires genuine independent approvals and
full MC01 package acceptance. Different-VK injection, verifier serialization,
OS/toolchain trust, actual offline build success and recursive resource fit remain
separate unresolved matters. No implementation bug requiring source modification
was established by the two cited API/dependency hypotheses.
