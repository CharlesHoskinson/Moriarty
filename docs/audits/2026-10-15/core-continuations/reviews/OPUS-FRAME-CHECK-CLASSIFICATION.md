# Opus 5.5: FRAME-PROPOSED-CHECKS classification

Reviewer: Opus 5.5 (`claude-opus-5-5`), sole security owner. Scope: one ordinary command only. This is not a correctness, security or feature acceptance.
Actions taken: read-only inspection of the worktree source. This file is the only write. No cargo, build, test, fmt, SDK, VM, proof, keygen, native verify, deploy, ledger, SRS, key or cache action was taken. Native authority stays 0.
Status: AGENTS/develop status was not refreshed in this context because the shell permission layer refused the compound load. This verdict selects no registered dispatch.

## Verdict: **APPROVE_ORDINARY** (subject to the preconditions below)

## Compiled closure (new versus the prior `--test` approvals)
`--lib` builds the mori-backend library with `cfg(test)` enabled. The build adds the following:
- `stage::frame::tests` (`frame.rs:364-632`): the 6 target tests.
- `proving::tests`: 8 `#[test]`. `signed_relation::tests`: 6 `#[test]`, plus `include!` of `tests/support/signed_shapes.rs` (already pinned).
- `proving::task4_native_driver` (`proving.rs:1777-1779`, `#[path]` to `src/task4_native_driver.rs`, already pinned) has 3 `#[test]`. One of them, `task4_native_resource_gated_entry` (`:1127-1129`), is `#[ignore]` and is the genuine native experiment: it reads `MORIARTY_TASK4_CASE` and calls `execute`. The driver also adds `charge(...)` hooks into the keygen, prove, verify, `params_load` and `params_get` paths (`proving.rs:476/483/505/614/623`, `parameters.rs:42/105`).
- `cfg(test)` helpers `proving::public_values` (`:662`) and `signed_relation::task4_reconstruct_before_caps` (`:1715`).
- New compile-time data inputs from `include_str!`/`include_bytes!`, listed under P2.

**Native reachability: linked, not executed.** The keygen, prove and verify paths and the task4 native entry are compiled into the binary. They are unreachable under this command for two reasons:
1. **Libtest filtering:** the filter is a substring match on full test paths. Only `stage::frame::tests::*` contains `stage::frame::tests::`. The other paths are `proving::tests::*`, `signed_relation::tests::*` and `proving::task4_native_driver::task4_*`, and none of them contain that substring.
2. **Ignored test:** there is no `--ignored`/`--include-ignored`, so the ignored native entry can't be selected even by name.

There are no `ctor`, `inventory` or `linkme` constructors in the backend or in `Cargo.lock`. `rusty-fork` comes in through proptest, and only proptest-forking tests invoke it. None are selected.

**Executed call graph of the 6 tests:**
- `mori_core::stage::load(&str)`: a pure parse/admit from a string. It does not touch `stage::decode`, the only Core stage `fs` site (`decode.rs:21`).
- `frame::{layouts, emit_pre, emit_post, assert_guard}`.
- `Builder::default/public` (`lib.rs:624-657`), which only pushes into `instructions`, `wires` and `bindings`.
- The `midnight_zkir_v3::{Instruction, Operand}` data types and `Fr::from` field construction.

`frame.rs` imports nothing from `proving`, `parameters` or the driver, and has no env, fs, process or net use. The six tests only inspect Builder structure: no SDK check, model, degree/k, VM, proof, keygen or SRS. `PARAMS_VERIFIER` and the SDK's compile-time public-verifier parameter embedding are the same linked residual as before. The tests don't directly read the root SRS.

The dependency graph and profile are unchanged from the integration targets: same deps, same dev-deps (insta, proptest) and same build scripts and proc macros. Only the mori-backend `cfg(test)` crate is newly compiled.

## Preconditions
P1. **Pins:** `BACKEND-CHECK-SOURCE-PINS.json` plus the six additional input pins, with only `stages.rs` changed, to `217c75af42e94b5f01087ccc04d3cb254cf7c50908041a138575ce6e8c51e1e1` per OPUS-CHECK-DELTAS A-Δ. In particular, confirm `stage/frame.rs` = `d13bb9f7…ff2f`, `task4_native_driver.rs` = `6e279fe9…890ba`, `proving.rs`, `parameters.rs`, `signed_relation.rs` and `signed_shapes.rs` against their existing pins, and `rust-toolchain.toml` = `72f7dd8c…b7af`. Check before and after the run. No production byte may change.

P2. **Add pins for these new compile-time inputs** (observed now; root must re-hash):
- `crates/mori-backend/tests/programs/payment.mori` f1f4807b…e786
- `examples/core/native-liability/partial.scenario.json` f37e0e10…74f5
- `examples/core/native-liability/present-zero.scenario.json` 2a830381…4877
- `examples/core/native-liability/settlement.mori` cf653de5…ccb2
- `examples/core/native-liability/zero-lifecycle.mori` 9c9bbec6…068f
- `examples/core/native-owner-intent/golden-vector.json` 9c035c8e…85f0
- `…/payment.authority.json` 0772aaf0…0970
- `…/payment.expected.json` f13b167f…77d6
- `…/payment.mori` a446c4dc…eb2d
- `…/payment.scenario.json` f41a3056…3085
- `…/payment.target.json` 6a1f7c24…d0f
- `crates/mori-backend/tests/support/task4-native/independent-payment.{artifact.json 81ccb2ea…758a, authority.json e63e6128…afa, layout.json 68db1985…49a1, mori a446c4dc…eb2d, payload.json 628c659f…9146, prepared.json a5b0a9df…43f6, scenario.json aa5f0a54…a7, signed.json 86130b08…fc44, target.json 16001ddc…984e, zkir b009f47c…1eff}`

These are data only, embedded as constants, and are not code. They are pinned so that the compiled binary is reproducible.

P3. **Exact argv** as in `FRAME-PROPOSED-CHECKS.json`: `+1.99.0 --offline --locked`, `-j2`, the shared flock, a 300 s timeout, `--lib`, the filter `stage::frame::tests::` and `--test-threads=1`. No `--ignored`, `--include-ignored`, `--exact` change or extra filter. Run once with no retry loop.

P4. **Environment:** the same cleaned environment as backend C1–C7. Unset the compiler variables `RUSTFLAGS`, `RUSTC_WRAPPER`, `RUSTC_WORKSPACE_WRAPPER`, `CARGO_ENCODED_RUSTFLAGS`, `RUSTDOCFLAGS`, plus all `CARGO_*` (except the set `CARGO_TARGET_DIR`), `MORI*`, `MORIARTY_*` (especially `MORIARTY_TASK4_CASE`), `INSTA*`, `PROPTEST*`, `RUST_TEST*` and `OPENSSL*`. Don't print their values. Leave `HOME`/`CODEX_HOME` untouched. There is no `.cargo/` config in the worktree. Root must also confirm there is no Cargo config in the ancestors or `CARGO_HOME`, and no symlink escaping the worktree.

P5. **Expected summary:** `6 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out` (8 + 6 + 3). The ignored native test is counted as filtered, not ignored. If the 6 test names aren't exactly the `frame.rs` set, if any other count differs, or if any `charge`/task4 or params output or file appears, stop and diagnose. Check that no `task4` capture or recipe directory was created.

P6. Record the run under a new check ID and keep the earlier evidence intact.

## Residuals (not waived)
- Third-party build scripts and proc macros, as before.
- The native keygen, prove and verify code and the task4 native entry are present in the test binary. The filter and the absence of `--ignored` are the only barriers. Any rerun that changes the filter or flags needs a new classification.
- The tests are structural checks I wrote, with no independent oracle. Passing doesn't establish frame soundness.
- The 6 tests are owed full Muse and Opus code/result review. CLI, Core, backend, closure and the 52 legacy tests pass; these 6 are excluded from that count until they run.
