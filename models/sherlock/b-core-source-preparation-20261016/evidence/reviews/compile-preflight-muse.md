## Nonsecurity audit

This approval covers one runtime-only, compile-only attempt of the exact candidate. It does not adopt the Rust source, schema, representation, or any public interface.

### Source composition and faithfulness

- The compile closure is exactly `compile_harness.rs`, `types.proposed.rs`, and `functions.proposed.rs`.
- `compile_harness.rs` loads the 104 declarations as a private `#[path]` module and `include!`s the two projection bodies inside another private module. It chooses no installed module path, public API, export, or product wiring.
- `const_row_kind` maps all seven `ConstRow` variants to `UInt`, `Bool`, `Nil`, `Cons`, `Leaf`, `Branch`, and `Marker`, respectively.
- `data_row_kind` maps all six `DataRow` variants to the first six corresponding kinds. There is correctly no Data marker case.
- The structural difference between `ConstRow::Nil { .. }` and unit `DataRow::Nil` is preserved. Payloads, `marker_free`, and field slices are not inspected.
- These bodies faithfully implement only the row-to-kind portion of D-23 `kind(ref)`. They do not implement `ValueRef` dereferencing, bounds checking, `arity`, `sortOfKind`, `shallowDiffer`, `checkedArith`, validation, admission, or reserved-tag handling.
- The matches are exhaustive without a wildcard. A future row variant would therefore require an explicit separately reviewed mapping rather than silently receiving a kind.
- On the supplied syntax evidence, the names, imports, exhaustive matches, edition-2024 source, and absence of external Rust dependencies are consistent with an ordinary stdlib `rustc --crate-type=lib` compilation. This is a reading only; compilation success remains unproved until the authorized attempt.

### Config, adapter, and guard

- `CONFIG.json` consistently pins the seven source-candidate rows, the three-file compile closure, the seven snapshot members, and an empty expected-test list.
- The final manifest composition is the seven candidate rows plus `SOURCE-CANDIDATE.json`, `CONFIG.json`, and `run_arithmetic.py`. The adapter refuses mismatched, duplicate, unnormalized, linked, missing, or extra candidate entries.
- The seven-member snapshot is closed. Only the three declared files may appear in rustc dep-info after compilation; the Cargo manifests, toolchain file, and guard are pinned context or control files, not additional compile inputs.
- Cargo is not invoked. Consequently, the workspace and git dependencies named in the snapshotted Cargo files are not resolved, fetched, compiled, or adopted by this attempt.
- The adapter retains the guard's direct-child execution model: no shell, the fixed minimal environment, the pinned non-proxy rustc path, channel verification against `1.99.0`, one 30-second wall clock per child, process-group termination on timeout, and no retry or resource increase.
- The compile step uses `--crate-type=lib`, edition 2024, and the declared three-file closure. The resulting rlib is only recorded. It is not executed, linked into a product, loaded, or used as a test or producer artifact.
- `compile-tests` and `run-tests` are recorded as `not-selected`. They are not an empty-suite pass, and `expected_tests` remains empty.
- The ordered plan stops dependent compile work at the first failure. The later candidate and guard postflights are integrity rechecks, not dependent execution steps; the terminal state retains the first failed step.
- Preflight binds snapshot members to the reviewed hashes before rustc runs. Candidate postflight rechecks the manifest, runner inputs, candidate membership, and both execution-preflight reports. Guard postflight additionally rechecks the snapshot, checkout, Python binary, and compiler binary.
- Both fresh Muse and Opus execution-preflight reports remain runtime prerequisites. Each must approve this exact manifest exactly once. This audit supplies only the nonsecurity Muse side and does not inherit or replace the Opus security decision.

## Limitations

- No tool, compiler, test, artifact, Cargo command, native component, SDK, model, or proof was executed during this review.
- All byte counts and hashes above are recorded candidate inputs, not independent recomputation.
- Syntax and typechecking are assessed from the supplied source only. Expected compiler warnings are retained and unjudged; they are neither accepted as harmless nor treated here as failures.
- Security eligibility, compile-time attack surface, sandboxing, and C4/native-authority decisions are expressly left to Opus. Native authority remains 0 and C4 remains unchanged within the stated scope.
- Root must still independently inspect the materialized candidate, confirm the review-gate files, allocate the fresh slot, and invoke the runner exactly once. Any byte, membership, report-binding, or first-failure deviation stops the attempt and requires fresh review.

## Remaining adoption decisions

None of the following is approved or decided here: schema freeze, R2 adoption for these Rust bytes, host representation, field spelling, derives, layout, discriminants, tag ordinals, wire ABI, serialization, module placement, visibility, public API, admission, C-INV and reserved-tag enforcement, full `kind(ValueRef)`, or the omitted helper and machine closures.

VERDICT: APPROVE_WITH_SCOPE
CANDIDATE_MANIFEST_SHA256: 426355f97f13123f5e06e7080140446fe59449ce9d6c9dd91b443005c30db3f1
