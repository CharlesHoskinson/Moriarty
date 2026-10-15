# Opus backend check classification: two proposed commands

- Reviewer: this session's runtime reports `claude-opus-5-5` (Opus 5.5), high effort, sole security owner for this scope. This session produced no adapter report. Root must record the identity that its own adapter actually returned, not this self-statement.
- Scope: ordinary execution-closure classification of the two argv in `BACKEND-PROPOSED-CHECKS.json`. Neither command was run. Native authority stays at 0. This report accepts no correctness or security property of the feature.
- Startup: read develop `SKILL.md`, `execution-focus.md` (Opus boundary) and `AGENTS.md`. Guarded `status --json` shows the historical SP01.6 queue, which is operational history only and was not dispatched.
- Method: read-only source reads plus `sha256sum`. I ran no Cargo, build, test, SDK, model or native action, and I read no SRS, key or cache file.

## Verdicts

| # | `cargo +1.99.0 --offline --locked test -p mori-backend --test …` | Verdict |
|---|---|---|
| 1 | `stages` | **APPROVE_ORDINARY**, if preconditions C1–C7 hold |
| 2 | `stage_relation` | **APPROVE_ORDINARY**, if preconditions C1–C7 hold |

## Pins

All 48 entries in `BACKEND-CHECK-SOURCE-PINS.json` match the worktree (`sha256sum -c`: ALL_OK). The mori-core `src` hashes are identical to `CORE-CHECK-SOURCE-PINS.json`.

**Missing pins (add before running).** These files are compiled in through `include_str!` or sit in the closure:
- `crates/mori-backend/tests/programs/counter.mori` `8db5630f0fedbadc8ef5824a6f39afcaaec5113049cc0dac45e3bbc69979db52`
- `crates/mori-backend/tests/programs/payment.mori` `f1f4807bc53b62e6ad8f816c44aebb92011925935a63479f4e3afa025a29e786`
- `crates/mori-backend/tests/snapshots/compile__authored_counter_abi.snap` `49cc42f9b64a7eb1dbeb710fd7defc76d4370e59b94ba6167b68365a82ac93cb`
- `crates/mori-backend/tests/snapshots/compile__authored_payment_effects.snap` `5c0c29b8579628905d9e4440fe129dc0eecf7c187dea701b0c49cc9a93d4af14`
- `crates/mori-lexer/Cargo.toml` `1ae623c3…8b68` (the same value as the CORE pin)
- `rust-toolchain.toml` `72f7dd8c…b7af`

A `git diff --stat fe01bc8b` earlier in this session was empty for `tests/programs` and `tests/snapshots`. The session git snapshot shows `mori-lexer`, `mori-diagnostics` and `mori-span` unmodified. A later `git diff` run was blocked by the permission layer, so root must assert `git diff --quiet fe01bc8b -- crates/mori-lexer crates/mori-diagnostics crates/mori-span crates/mori-backend/tests/programs crates/mori-backend/tests/snapshots`.

## Test selection (counted from the code)

- **`stages.rs`:** 6 `#[test]` functions. **5 are selected and 1 is ignored** (`legacy_artifact_and_every_ir_file_match_baseline_bytes`).
  - The argv has no `--ignored` or `--include-ignored`, and libtest has no environment switch that includes ignored tests.
  - The expected summary is `5 passed; 0 failed; 1 ignored`. Any other ignored count is a stop condition.
  - If the ignored test ran by mistake, it would read `MORIARTY_STAGE_V1_BASELINE` and files under that path. Nothing generates or writes a baseline fixture.
- **`stage_relation.rs`:** 4 tests, 0 ignored. The expected summary is `4 passed; 0 failed; 0 ignored`.
- `--test` excludes lib unit tests, doctests and other targets. The library is built without `cfg(test)`, so `proving::task4_native_driver` (`#[cfg(test)]` at `proving.rs:1777`) and the `signed_relation`/`proving` test modules (which read `MORIARTY_*` and write files) are **not compiled**.

## Reachable source effects

**`compile_staged`** (`stage/lower.rs:18`):
1. Refuses a signed Target.
2. Calls `mori_core::stage::load`. This is the CORE-classified pure path (parse_v2 → validate → plan → cost → digest), and its pins are identical. I reuse the CORE premise P1 for exactly that path.
3. Then runs `Target::validate`, `liabilities::registry` and `frame::layouts`.
4. For each segment it calls `lower_action_segment(…, signed=None, Some(stage))`. That function:
   - builds instructions in the `Builder`;
   - calls `frame::emit_pre`, `assert_guard` and `emit_post` (they only push `Instruction`/`Operand`/`Fr`);
   - computes `stage_context_hash` (serde `to_value` → `intent::canonical` → SHA256);
   - calls `authorization::digest_limbs`;
   - builds the `IrSource{…}`, then `serde_json::to_value`/`to_string`, then **`IrSource::load(bytes)`**, then a `PartialEq` round-trip check.
5. Finishes with `proving::hex`, which is pure formatting, and a pretty-JSON fixed-point loop.
6. `_out` is unused, so there is no filesystem access.

**Legacy `compile`** (counter/payment test): `stage::require_core1` → `compile_mode` → `mori_core::validate` → the same `lower_action_segment(…, None, None)` → `signed_relation::context_hash` (serde plus SHA256). `signed_relation::admit` and `append` run only when a signed policy is set, and the Target here is unsigned.

**`legacy_compilers_refuse_staged_programs`:**
- `stage::load(MIXED)` is a pure Core call.
- Then `compile` runs `stage::require_core1` **as its first statement** (`lib.rs:366`). It returns `mori::backend::program` before target validation, lowering, IrSource construction or any other SDK call.
- **Confirmed.**

**Public SDK closure** (pinned checkout `midnight-ledger@04c9c5d`, `zkir-v3/src/ir.rs`):
- `IrSource::load` (`:955`) runs `serde_json::from_reader`, checks the version against 3.0 and calls `from_value`.
- `Operand` serde (`:188–245`) does hex encoding/decoding and calls `Fr::from_le_bytes`/`as_le_bytes` (`transient-crypto/src/curve.rs:346,370`), which are field-representation conversions.
- `tag_enforcement_test!` is `#[cfg(test)]`, so it is not compiled.
- **None of the following is reachable from either test file or the backend paths above:** `.check`, `.model()`, `ir_vm`, `keygen`, `prove`, `PARAMS_VERIFIER`, `ParamsProver`, `PK_CACHE` or base-crypto data-provider fetching. I grepped the stage module and the `lib.rs` call graph.
- The test-side SDK use is `IrSource::load` plus `Instruction` pattern matching (`stages.rs`) and serde round-trips (`stage_relation.rs`). The test-file comments were not used as evidence.

Runtime effects: CPU and memory only. There is no proptest or insta use, so no regression or snapshot files are written.

## Build and link closure: what is new compared with CORE

I reuse these CORE premises unchanged: the flock/timeout wrapper; the installed `+1.99.0` toolchain; no `.cargo/config*` (re-checked for the worktree, `~/Moriarty`, `.worktrees` and `~`); offline/locked; and the public object cache premise P7, which root owns and I did not inspect.

**I do not reuse CORE P4.** mori-backend *links* the SDK. Its normal dependencies are `midnight-zkir-v3` (rc.2, which brings `midnight-proofs 0.8.3`, `midnight-circuits 7.2.4`, `midnight-zk-stdlib 2.3.5` and `transient-crypto[cli]` → `base-crypto[cli]` with indicatif, reqwest/rustls, flate2 and atomic-write-file), `midnight-transient-crypto`, `midnight-curves 0.3.1` (blst), `futures`, `rand 0.8` and `libc`.

Residuals, which I accept as ordinary for this classification:
- **R1, compile-time SRS embedding.** `transient-crypto/src/proofs.rs:121` contains `include_bytes!("../static/bls_midnight_2p14")`. rustc reads this public upstream verifier parameter file from the pinned git checkout and embeds it in the rlib. It is decoded only through the lazy `PARAMS_VERIFIER`, and only verify paths dereference that. Those paths are unreachable here, so nothing is loaded or deserialized and no degree/k is computed at runtime. This is not a prover SRS, PK or proving cache. If root counts any compile-time embedding as an "SRS action", treat it as **DENY** instead.
- **R2, host build scripts** (only if objects are stale): blst `build.rs` (cc C/asm build plus a `libblst.a` stat in its own crate directory), libc, and the CORE-listed aws-lc-sys, ring and others. They read `CC`, `CFLAGS`, `CC_*`, `AR`, `BLST_TEST_NO_STD` and `CARGO_ENCODED_RUSTFLAGS`. Proc macros: the midnight derives (CORE-checked), serde, thiserror, zeroize, fake and pastey. I did not audit the registry C sources line by line.
- **R3, linked-but-unreached SDK code.** reqwest, rustls and the data-provider code may be linked. If aws-lc is retained, its C static initializer may run CPU feature detection, which honors `OPENSSL_ia32cap`. It performs no I/O or network action.
- **R4, cargo-home writes.** Cargo writes package-cache/git locks. The git checkouts `04c9c5d` and `a01a1ea` exist locally. Cargo fails closed if anything is missing.

## Preconditions (each must hold, or the classification is redone)

- **C1:** All 48 pins, the 6 added pins and the baseline diff above match at execution time. The SDK is the lock-pinned rev `04c9c5d` with the registry checksums from `Cargo.lock`.
- **C2:** **The argv alone is insufficient.** It unsets only `RUSTFLAGS` and `MORI_FEE_POOL_CAPTURE_DIR`. Root must launch from a sanitized parent environment. Clear these without dumping their values:
  - all `MORI_*`, `MORIARTY_*`, `PROPTEST_*`, `INSTA_*`, `RUST_TEST_*` and `OPENSSL_*` variables;
  - every `CARGO_*` except the argv `CARGO_TARGET_DIR`;
  - `RUSTC`, `RUSTDOC`, `RUSTC_WRAPPER`, `RUSTC_WORKSPACE_WRAPPER`, `RUSTDOCFLAGS` and `RUSTUP_TOOLCHAIN`;
  - `CC`, `CXX`, `AR`, `CFLAGS`, `CXXFLAGS`, `CC_*`, `CFLAGS_*`, `CRATE_CC_NO_DEFAULTS` and `BLST_TEST_NO_STD`.

  Leave `HOME` and `CODEX_HOME` unchanged.
- **C3:** Exactly these two argv run in sequence, stopping at the first failure, under the 300s timeout and the 120s lock wait.
- **C4:** The summaries show 5/1 ignored and 4/0 ignored, and the ignored test's name appears as ignored. Never pass `--ignored` or `--include-ignored`.
- **C5:** No `/home/.cargo/config*` or `/.cargo/config*` exists (carried over from CORE P5).
- **C6:** The target directory holds only public compiled objects (root's P7).
- **C7:** Root re-verifies the pins and the configuration after the run and reports any new untracked file.

## Not established

- Compiler correctness, relation soundness and staged-artifact security.
- Event authentication, consumption and the gates (all ten remain unperformed).
- Baseline byte equivalence: the ignored comparator needs a separate Opus-classified fixture generation.
- These results are not acceptance evidence. The Core 32-pass result is not backend evidence.

Full candidate/result reviews by Muse and Opus are still owed. Native, proof and target gates keep their full scope.
