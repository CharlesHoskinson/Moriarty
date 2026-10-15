# Opus CLI check classification: `core_continuations`

- Reviewer: Claude Opus 5.5 (`claude-opus-5-5`), high effort, sole security owner for this scope. Terminal status: complete.
- Scope: the ordinary execution-closure classification of the single command in `CLI-PROPOSED-CHECKS.json`. This is not candidate acceptance. Native authority stays at 0. The command was not run. Reads were read-only source and checkout reads, plus `sha256sum` over 9 pinned paths. No Cargo, build, test, SDK, model or native action was taken.
- Startup: the guidance from the prior core classification (develop `SKILL.md`, `execution-focus.md`, `AGENTS.md`) applies unchanged. I reused only its named build premises (wrapper, toolchain, no `.cargo/config*`, offline lock), and I state that reuse explicitly here.
- Inspect patch: `inspect.rs` sha256 = `5cef6e03…adebef`, which matches the pin. I classify only its effects. I do not judge its correctness.

## Verdict: **APPROVE_ORDINARY** (conditional on prerequisites C1–C6)

Command: `flock -w 120 /tmp/moriarty-language-wave1-build.lock timeout --signal=TERM --kill-after=10s 300s env -u RUSTFLAGS -u MORI_FEE_POOL_CAPTURE_DIR CARGO_TARGET_DIR=…/language-wave1-20261014/cargo-target cargo +1.99.0 --offline --locked test -p mori --test core_continuations -j2 -- --test-threads=1`

## Pins

- Spot-verified 9 of the 113 pins byte for byte: `inspect.rs`, `core_continuations.rs`, `mori-backend/src/lib.rs`, `stage/lower.rs`, `compile.rs`, `simulate.rs`, `delivery.mori`, `atomic-compatibility.mori`, `Cargo.lock`. The sandbox denied a full scripted check, so root must verify all 113 before and after (C1).
- All `.rs` files under `mori-core/src`, `mori-backend/src` and `mori-ledger/src`, and all reached `mori/src` files, are pinned. `escrow.mori` has the same hash as `delivery.mori`.
- **Missing pins (add them; they do not block this classification):** `crates/mori-core/Cargo.toml`, `crates/mori-backend/Cargo.toml`, `crates/mori-ledger/Cargo.toml` and `rust-toolchain.toml`. Also add `git diff --quiet fe01bc8b -- crates/mori-lexer crates/mori-diagnostics crates/mori-span crates/mori-ast crates/mori-parser crates/mori-checker crates/mori-fmt`. All of these crates are linked into the `mori` binary, and `check` calls `mori_lexer::first_token_text`. The sandbox denied my git diff, so this is unverified here.

## Build/link closure

- **Targets:** the `mori` binary (non-test profile, so no `cfg(test)` code; the `core_io` child-respawn tests are excluded) plus one integration-test binary. No lib units, no doctests, no `--workspace`.
- **Binary linking:** all of `mori-ledger`, `midnight-ledger-v9` (`proof-verifying`), the zkir-v3/transient-crypto/proofs crates and the base-crypto reqwest stack are linked. Command selection is an explicit clap `match` in `commands/mod.rs`. Startup is `Cli::parse` → `Shell::new` → `commands::run`. There is no logger, panic hook, clap `env=` attribute, `ctor`/`inventory`/`linkme` crate, or life-before-main. The only env reads in workspace `src` are `cfg(test)` code (`core_io.rs`, `signed_relation.rs:1704` `MORIARTY_SIGNED_MEASUREMENT_DIR`) and `task4_native_driver.rs`, which is not declared in `lib.rs`. anstream/miette may read `NO_COLOR`/`TERM`-class variables for rendering only, and `--color never` is passed.
- **Test-binary linking:** `mori_backend::Target::ledger9` (a pure constructor), `stage::StagedArtifact` Deserialize (with `decode_registry_*` and `decode_artifact_target` → `Target::validate`) and serde_json. The dev-dependency Midnight crates are unreferenced, so they are compiled but not loaded.
- **Build scripts:** no `build.rs` in any workspace crate. In checkout `04c9c5d`, only `onchain-runtime/build.rs` exists. It spawns `scheme` only when the `vendored` feature is off. `vendored` is in `default`, and no `default-features = false` applies to it, so it does not run. Registry build scripts (aws-lc-rs/sys, ring, blst, and the probes) are ordinary offline C/asm compiles when artifacts are stale. Proc macros are as in the prior classification. The mori binary graph is larger than mori-core's, and I did not enumerate every registry build script in it (residual R1).

## Runtime closure (7 tests, serial)

Every test spawns `env!("CARGO_BIN_EXE_mori") --color never <cmd> <tmp>/source.mori`. Each test creates its fixture dir with `create_dir` at `temp_dir()/mori-core-continuations-<pid>-<n>`. It writes only public source, scenario, continuation, event and target JSON there, and removes the dir in `Drop`.

1. **`check --json`** (`check.rs`): `read_source` (bounded to 1 MiB) → `first_token_text` → `stage::load` → `print_stage_report` → `json::export` to stdout.
2. **`inspect`** with no `--target` (`inspect.rs:18-38`): `read_source` → `stage::load` → `print_stage_report`. No backend, target or compile. The `/2`+`--target` branch is not exercised.
3. **`simulate` /2 start** (`run_stage_start`): `stage::load` → `decode::read_bounded(scenario, 1 MiB)` → `json::parse` → `stage::start`.
4. **`simulate --resume`** (`run_resume`): `read_bounded` on the continuation and event files → strict `decode::continuation`/`event` → `stage::resume`. Nothing is written, and the test re-reads `continuation.json` unchanged.
5. **`simulate` /1** (`atomic-compatibility.mori`): `mori_core::simulate_metered` or `simulate`. `--trace` is not passed, so `replay::generate`, `--target` and authorization are unreachable.
6. **`compile` /2** (`compile.rs:19-35`):
   - `read_json(target)` → `compile_staged`. The signed Target is refused. Then `stage::load` → `Target::validate` → `liabilities::registry` → `frame::layouts` → per segment, `lower_action_segment(…, signed=None, Some(stage))`.
   - Builder/Instruction/Operand construction → `frame::emit_pre`/`emit_post` → `stage::context::stage_context_hash`. `signed_relation::append` is unreachable because `signed=None`.
   - `serde_json::to_string` → **`midnight_zkir_v3::IrSource::load`**, which is `serde_json::from_reader` on in-memory bytes plus a version rewrite plus serde Deserialize, followed by a `PartialEq` round-trip.
   - Then the fixed-point `to_vec_pretty`. `_out` is unused inside the backend.
   - Then the CLI runs `create_dir(out)` → `publish_new` per circuit and the manifest last. `publish_new` writes a `create_new` staging file `.mori-proof-<pid>-<n>.tmp` in `out`, syncs it, hard-links it into place and removes it on `Drop`.
   - The test reopens the artifact in-process.

**Blocked SDK paths are not reachable.** `IrSource::check` sits only in `signed_relation::check_relation`. Its non-test callers are `witness` ← `accept_intent` (verify-intent) and `proving.rs:265` (prove). No selected command reaches either. No model, VM, degree/k, keygen, prove, native verify, parameters/SRS, data provider, ledger or deploy code executes. `zkir-v3/src/main.rs` (keygen/fetch) is a separate binary target and is not linked. **Classification: the zkir-v3 IR parse and serde is an ordinary public, CPU-only path.**

## Prerequisites (each must hold, or the command needs reclassification)

- **C1:** root verifies all 113 pins, the 4 added Cargo/toolchain pins and the crate baseline diff before and after the run, and stops on any mismatch.
- **C2:** child environment. The proposed argv itself clears only `RUSTFLAGS` and `MORI_FEE_POOL_CAPTURE_DIR`. Root must additionally unset, by name and without dumping the environment:
  - all `MORI_*`, `MORIARTY_*` (recommended) and `PROPTEST_*` variables;
  - every `CARGO_*` except the argv `CARGO_TARGET_DIR`;
  - `RUSTC`, `RUSTDOC`, `RUSTC_WRAPPER`, `RUSTC_WORKSPACE_WRAPPER`, `RUSTDOCFLAGS` and `RUSTUP_TOOLCHAIN`.
  - Leave `HOME`, `CODEX_HOME` and `RUSTUP_HOME` untouched.
  - Note: clearing `CARGO_HOME` when it is set to a non-default location makes offline resolution fail closed. That failure is acceptable, but it is not a pass.
- **C3:** `TMPDIR` must be unset or point to the OS temp directory, not to a repo, key or cache directory.
- **C4:** exactly this argv runs once, with `-j2` and `--test-threads=1`. On failure, stop. Do not retry, and do not run the full lib or workspace.
- **C5:** reused premises P5 and P7 from `OPUS-CORE-CHECK-CLASSIFICATION.md`: no `/home/.cargo/config*` or `/.cargo/config*`, the installed 1.99.0 toolchain, and a shared target directory that holds only public compiled objects (root's premise). The git checkouts `04c9c5d` and `a01a1ea` must be present, or the build fails closed.
- **C6:** after the run, root confirms no `mori-core-continuations-<pid>-*` dirs remain in temp. A SIGTERM or SIGKILL from the 300s timeout skips `Drop`. Root removes leftovers only after looking at them, and leftovers contain only public fixtures.

## Residuals and limits (not waived)

- **R1:** registry build scripts in the full `mori` graph beyond aws-lc/ring/blst and the prior set were not enumerated individually. They are covered by `--offline` and the ordinary-compile premise only.
- **R2:** I did not read the bodies of `lower_action_segment` (`lib.rs:777-1150`), `frame.rs`, `context.rs`, `liabilities.rs`, `authorization::digest_limbs` or the zkir-v3 Deserialize impls line by line. Their effect closure rests on grep, which found no fs, env, process or net use and no SDK check/prove call. This is not a correctness audit.
- **R3:** the inspect patch, the core suites (32 tests) and this command's results are not acceptance evidence. The stage outputs remain `authentication: none` and `consumption: untracked`.
- Full candidate and actual-result audits by Muse and Opus are still owed. All prior proof, native and target gates keep their full scope.
