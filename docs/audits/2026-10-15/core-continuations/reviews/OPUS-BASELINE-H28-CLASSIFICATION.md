# Opus 5.5 classification: H28 coverage check and legacy byte checks

Reviewer: Opus 5.5 (`claude-opus-5-5`), high effort, sole security owner for this scope.
Scope: classifying two bounded sets of ordinary execution commands. This is not feature acceptance. Native authority stays at 0.
Method: I read sources only. I made no Cargo, build, test, fmt, SDK, VM, prove, keygen, ledger, key, SRS or cache calls, and I did not stat or read any private cache. Startup: I loaded the develop skill and AGENTS.md and ran guarded `status --json`, which reported historical SP01.6 blockers that don't affect this scope.

## Verdicts

| Check | Command | Verdict |
|---|---|---|
| A | `cargo test -p mori-ledger --test source_closure_coverage` (H28-PROPOSED-CHECKS.json) | **APPROVE_ORDINARY**, subject to the preconditions in A.4 |
| B1 | baseline-mori `compile` into `legacy-byte-baseline/baseline-export` | **NEEDS_SOURCE** |
| B2 | `cargo test -p mori-backend --test stages -- --ignored --exact legacy_artifact_and_every_ir_file_match_baseline_bytes` | **NEEDS_SOURCE**. It can't run until B1 runs, and its comparator has a residual (B.3) |

## A. `source_closure_coverage`

### A.1 Facts I verified
- The test file `crates/mori-ledger/tests/source_closure_coverage.rs` has SHA-256 `184f35a5…b4e0d5`. That matches the prompt and H28-BASELINE-VERIFICATION.json.
- The 61 `BASELINE` entries in the test match `ordered_members` in H28-BASELINE-VERIFICATION.json exactly and in the same order. The root recorded `exact_baseline_membership_match: true` against `git show fe01bc8b:crates/mori-ledger/src/proving.rs` (sha `1dffbb8b…`). I did not repeat that git read myself.
- The 8-entry `CORE_STAGE` suffix and the total of 69 match the frozen A2 append.

### A.2 What runs (call paths)
- Compile time: `include_str!("../src/proving.rs")` embeds the text of proving.rs. `env!("CARGO_MANIFEST_DIR")` is fixed when the test is compiled.
- Runtime: the test parses the embedded text with pure `&str` operations. It makes no calls into the `mori_ledger` API, the SDK, keys, parameters or native code.
- `resolve` calls `fs::canonicalize` on `mori-ledger/src/<member>` paths. That checks paths and metadata only.
- `rust_files` walks `mori-core/src` with `fs::read_dir` and `is_dir`, both read-only. It never opens file contents.
- The only std modules used are `collections`, `fs` and `path`. There is no `process`, `net`, env mutation or write.
- Source effects: none. Cargo writes only to the shared `CARGO_TARGET_DIR`. `--locked` keeps Cargo.lock unchanged.

### A.3 What gets compiled (build closure)
- The test binary links `mori-ledger`'s library and dependencies: mori-core, the midnight-ledger crates with `proof-verifying`, midnight-zkir-v3, midnight-proofs `=0.8.3`, midnight-curves `=0.3.1`, serde/serde_json, sha2, thiserror, anyhow, rand and libc. Its one dev-dependency is `futures`.
- Linking doesn't execute any of that code. Building does run the third-party build scripts and proc macros pinned in Cargo.lock, offline.
- No workspace build.rs exists in mori, mori-core, mori-backend or mori-ledger.
- The prior CLI build premise (offline, `--locked`, Rust 1.99.0, the same public target dir, no Cargo config) can be reused. That holds only if the root confirms that this premise's compiled closure already contained this `mori-ledger` dependency set at the current Cargo.lock. If any Cargo.lock entry changed since then, that premise doesn't cover it.
- Residual, not waived: build-script and proc-macro execution from the pinned registry. That is the same trust as every prior ordinary build.

### A.4 Preconditions and extra pins (the root must add these)
1. Add the test file and its SHA to the check source manifest. It was excluded from the original CLI-CHECK-SOURCE-PINS.json.
2. Pin the current `crates/mori-ledger/src/proving.rs` SHA, since it is embedded at compile time. Also pin the sorted list of `.rs` files under `crates/mori-core/src`, with their SHAs. That list includes the untracked `stage/` files, and the walk depends on it.
3. Confirm that CLI-CHECK-SOURCE-PINS.json and CLI-CHECK-ADDITIONAL-INPUT-PINS.json still match the bytes on disk just before the run.
4. Confirm that no `.cargo/config*` exists in the worktree, its ancestors or `CARGO_HOME`.
5. Environment: besides the `-u RUSTFLAGS -u MORI_FEE_POOL_CAPTURE_DIR` already in the command, also unset `RUSTC_WRAPPER`, `RUSTC_WORKSPACE_WRAPPER`, `CARGO_BUILD_RUSTC_WRAPPER`, `CARGO_ENCODED_RUSTFLAGS`, `RUSTDOCFLAGS`, and every other `MORI*`, `PROPTEST*` and `CARGO*` variable except the `CARGO_TARGET_DIR` set in argv. Don't print their values. Leave `TMPDIR` alone.
6. The test follows symlinks: `canonicalize` resolves them, and `is_dir` follows them during the walk. Confirm that `crates/mori-core/src` and the member paths contain no symlinks that point outside the worktree.
7. Run the command once. On a timeout or failure, stop and don't re-run it unchanged.

### A.5 What a pass shows
A pass shows only that the declared closure is the frozen baseline followed by the 8 stage files, and that every Core `.rs` file falls inside it. That is declared source identity. It doesn't authenticate the build.

## B. Legacy byte checks

### B.1 Facts I verified
- `legacy-byte-baseline/source.mori` has SHA `4ad04eed…6b85`. That equals `oracle-compat-v1.mori` and the frozen pin.
- `target.json` has SHA `8cd664ce…9624` and contains `profile: ledger-9`, `network: local`, `anchor: legacy-byte-oracle`, ledger revision `04c9c5d9…`, zkir `3.0.0-rc.2`, `max_instructions` 100000 and `max_artifact_bytes` 16777216. It is unsigned (no `signed_policy`). It is a local oracle identity, not a qualified Fenrir identity. Root must record that 8cd664ce… SHA as an extra pin.
- BASELINE-BINARY.json (R53) records that `baseline-mori` has SHA `89ae66c4…f28e`. It was copied from `language-wave1-20261014/cargo-target/debug/mori` and built from source `95df7355…`, with merge base `eb2577d6`. The record says the binary was not executed. I did not hash the binary. The root must hash it before and after.

### B.2 Why B1 is NEEDS_SOURCE
My attempt to read `git show 95df73558ab4fb0b6d80618a76e88c3a52c38888:…` was not approved, so I couldn't inspect the baseline's command code. The current worktree's `crates/mori/src/commands/compile.rs` is modified: it adds a `moriarty/2` staged branch. So it doesn't stand in for the baseline. As instructed, I don't assume that the baseline `compile` behaves like the current one or like the earlier qualified `compile-call`.

For reference, the current code serialises with `serde_json::to_string_pretty`, adding no trailing newline. It writes `{index:03}-{action}.zkir`, then `artifact.json` last, through `proof_io::publish_new`, into a directory it creates with `create_dir`. If the baseline does the same, the comparator's `to_vec_pretty` and file names line up byte for byte, but I haven't confirmed that.

Files needed to settle B1, each at `95df7355` (or `eb2577d6` if they're identical there):
1. `crates/mori/src/commands/compile.rs`, for the unsigned branch, the serialiser, the leaf names and the refusal to overwrite.
2. `crates/mori/src/commands/proof_io.rs`, for `publish_new` (create-new, no symlink following).
3. `crates/mori/src/commands/core_io.rs`, for `read_source` and `read_json` limits.
4. `crates/mori/src/main.rs`, `cli.rs` and `commands/mod.rs`, for the dispatch of `compile` and `--color never`, and to confirm no network, environment or cache side effects.
5. The `Target` and `Artifact` definitions in `crates/mori-backend/src/lib.rs`, to check whether the baseline accepts this exact `target.json` (unknown-field handling, `max_artifact_bytes`) and to see how `Artifact` serialises its fields.
6. A diff of `crates/mori-backend/src` and `crates/mori-core/src` between `95df7355` and `eb2577d6`, to confirm no production change between them. That is the PR52 final binding claim.

If those match the current unsigned path, I expect APPROVE_ORDINARY. Its source effect would be one new directory, `baseline-export`, holding the `.zkir` leaves and `artifact.json`, with nothing else written.

Preconditions for that later approval:
- `baseline-export` doesn't exist yet.
- The binary SHA is `89ae66c4…` before and after the run.
- The source and target SHAs above are unchanged afterwards.
- The environment is scrubbed as in A.4(5).
- Nothing reads a private or proving cache.
- The command runs once.

### B.3 B2: comparator review
I have the test text, so this classification doesn't need B1 to have run; executing B2 does.
- The test reads `MORIARTY_STAGE_V1_BASELINE`, which argv sets. It reads `source.mori`, `target.json`, `artifact.json` and `NNN-<action>.zkir`, and calls only `mori_core::parse` and `mori_backend::compile`. It writes nothing, so the candidate never produces expected bytes.
- The build closure is `mori-backend` plus its dev-deps (`midnight_zkir_v3`, `serde_json`). The same build premise and residual as A.3 apply. The root must confirm `mori-backend`'s dev-dependency closure the same way.
- **Residual (test strength, not waived):** the test opens one `.zkir` file per circuit the candidate produced. It never checks that the fixture's set of leaves equals {`artifact.json`} plus exactly those `.zkir` files. If the manifest doesn't embed circuit identity, a missing or extra baseline leaf could go undetected.
  - Interim fix: after copying, the root records every fixture leaf with its hash and checks that set against the leaves the baseline exported.
  - A set-equality assertion belongs to a non-security author plus Muse. I'm writing no code here.
- Preconditions before running B2:
  - B1 is approved and succeeded.
  - The root copied source, target, `artifact.json` and every exported `.zkir` into the fixture root without overwriting anything, and kept a hash for each leaf. The binary and input SHAs were re-verified after the copy.
  - The root pinned `crates/mori-backend/tests/stages.rs` and the current candidate source closure.
  - The environment is scrubbed as in A.4(5), with `MORIARTY_STAGE_V1_BASELINE` set only by argv.
  - Offline, Rust 1.99.0, the same public target, no Cargo config, no `TMPDIR` override, and a single run.

## Still owed
- The full Muse and Opus reviews of the candidate and the results are still owed.
- Nothing here accepts features, qualifies Fenrir or changes native authority.
- Not waived:
  - build-script and proc-macro execution;
  - the comparator's missing leaf-set check;
  - the B1 baseline source I haven't inspected.
