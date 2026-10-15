# Opus 5.5 final scope-A repair: A-C1, A-C2, A-D1 (source author and command classifier)

- **Role:** Opus 5.5, high effort. I am the source author of these proposed repairs and the ordinary-command classifier. I am **not** an independent reviewer, so these files need a fresh non-author audit.
- **Candidate:** `7d6c3965b89d713575ade6a8dddc4f1780b3fdd3`. I did not modify the repository: every write is under `R/opus-final-a-repair/` plus this report.
- **Startup:** I loaded the develop skill and `execution-focus.md` (Opus-owned execution boundary) and ran the guarded `status --json`. It shows only historical SP01 state. I changed no counters, and native authority is 0.
- **What I ran:** no build, test, fmt, Cargo, SDK, native, key, parameter, SRS or cache operation. I did only source reading, `sha256sum`, `diff`, and the `cp`/`mkdir` writes under `R`.
- **Status of this report:** it does not establish correctness or a pass, and it is not a review.

## WORKTREE OBSERVATION (root must resolve before integration)

At the start the worktree was clean. During this session `git status` began showing ` M crates/mori-backend/tests/stage_relation.rs`:
- 30 insertions, mtime 2026-10-15 00:13:10 -0600;
- current sha256 `3c5d0111…`; the HEAD blob is `e6b35eeb…`.

**I did not write this file.** It is presumably from the live B session. I left it untouched.

It is not compiled by either command below: it is a mori-backend integration test, and neither command selects it. But the tree is no longer pristine `7d6c3965`.

## Payload (`R/opus-final-a-repair/`)

| File | Pre sha256 (= HEAD) | Post sha256 |
|---|---|---|
| `crates/mori/tests/core_continuations.rs` | `81cdd7552250b7da4587beb5c03af1c742d0b3d64bb236085c5ae73a79fcd813` | `816a43200b438e84bb333548f64c77363581d701d53974852ca37a0f994670ae` |
| `crates/mori-ledger/tests/core_profile_refusals.rs` (new) | absent | `ca14acf37b565f44f24dc223a68cd9ffaa82f319c91338dfae476ffd7c470eeb` |
| `docs/CORE-CONTINUATIONS.md` | `8ab60fd80fe84e14c003705c111c33de84b9eedf692132d8b65dabf8dc3ce581` | `e0f2253486bc3cfd8023963394e32bf0ca499fc9dfb1214725eb048628be8a6e` |
| `examples/core/continuations/README.md` | `905fa30f2f617c9641c07cd26452af8310ff29c81e4d3546c3aaf8930ad19cae` | `9548a0f7b84a9af5db51fc57c58efa2d90ec9db023fe46e6d61093eb9f706ee9` |

**Layout of the payload directory:**
- `pre/`: byte copies of HEAD.
- `proposed/`: complete proposed files.
- `OPUS-FINAL-A-REPAIR.patch`: the unified patch, sha256 `7e106f3a5805516613f19742d85337ed894eecd9add5e971f994b247c3591401`.

**Applying the patch.** It was produced by `git diff --no-index` with absolute paths, so it must be applied with `git apply -p9` from the worktree root. My `git apply -p9 --check` was not approved, so root must run that check itself; the `proposed/` copies are authoritative.

**Old tests are unchanged.** All 7 existing test bodies and helpers in `core_continuations.rs` are untouched. The only edit above line 216 is the header comment on line 2: it said no test "requests … replay or ledger execution", which would become false, so it now says no test *reaches* them and that the `--trace` and `compile-call` requests are refused at the profile first.

## A-C2: frozen H16, three public CLI cases (appended to `core_continuations.rs`)

All expected values are written out as literals from the frozen typed errors. None is read from the callee, and the labels are a local constant.

1. **`h16_core_one_resume_is_labelled_profile_rejection_before_continuation_or_event_read`**
   - Input: the `/1` source `atomic-compatibility.mori` to `simulate --resume <absent> --event <absent>`.
   - Path: `run_resume` reads the source, then `stage::load` → `parse_v2` gives a profile mismatch.
   - Expected: exit 1, empty stderr, and exactly these keys: {status, error, attempted, committed, and the three labels}. Also `attempted` `"0"`, `committed` `"0"`, and `error` = `{"code":"mori::core::profile","message":"Expected language \"moriarty/2\"; found \"moriarty/1\".","offset":21,"step":null}`.
   - The offset is the `;` at byte 21 of `language "moriarty/1";`.
   - The absent paths stay absent. Since nothing exists to read, this shows pure ordering.
2. **`h16_core_two_trace_target_prints_backend_program_profile_rejection_before_reconstruction`**
   - Inputs: `delivery.mori` (`/2`) with `--action deliver`, the oracle escrow opening scenario, `--trace`, and `--target`. The target is `Target::ledger9("local","continuations")`, the existing unsigned public fixture with `signed_policy:None`. No signed arguments are given.
   - Inspected path (`simulate.rs:66-92` → `replay.rs:141-153` → `signed_relation.rs:651-677`):
     - read the Target JSON;
     - `AuthorizationArgs::load` → `Unsigned`;
     - read the source and scenario;
     - `bounded()` ×2, which is JSON export only;
     - `reconstruct_statement`: the policy/signed match is (None, None), then `mori_core::parse` → Err.
   - Nothing past that point is reached: not `require_core1`, `compile_mode`, `IrSource::load`, `claim_from_reconstruction` or any SDK call.
   - Expected stdout is exactly `{"status":"rejected","qualification":"local-reference-replay","error":{"code":"mori::backend::program","message":"mori::core::profile: Expected language \"moriarty/1\"; found \"moriarty/2\"."}}`, with exit 1 and empty stderr. The temp directory holds only its 3 input files, so no sidecar is written.
   - This pins A9's `mori::backend::program` wrapping. Note that the refusal is the structural parse; `require_core1` there is defence in depth and unreachable for a `/2` source.
3. **`h16_core_two_inspect_target_refuses_with_core_profile_before_target_read`**
   - Input: `/2` to `inspect --target <absent>`.
   - Path (`inspect.rs:39-49`): `stage::load`, then `require_core1` → Err.
   - Expected: exit 1, empty stdout, stderr = `mori::command: mori::core::profile: Unsupported language or semantics identity.`, and the target is never created or read.

**How stderr is pinned.** `diagnostic()` splits on whitespace and drops the miette `×`/`│` gutter glyphs, which removes the 80-column wrapping. It then compares for **equality**, not `contains`. None of the pinned messages contains a hyphen, so textwrap's hyphen splitting cannot change the result.

## A-C1: H19 and the A9 ledger identity

**`h19_core_two_compile_call_refuses_before_deployment_read_or_emission`** (CLI, in the same file)
- Input: `/2` to `compile-call --action deliver --deployment <absent> --out <absent>`.
- Path: `validate_shape`, then `core_io::program` → parse refusal. This happens before the deployment read (`compile_call.rs:60-63`) and before `create_dir`.
- Expected: exit 1, empty stdout, stderr = `mori::command: mori::core::profile: Expected language "moriarty/1"; found "moriarty/2".`, and neither `out` nor the deployment exists.

**New `crates/mori-ledger/tests/core_profile_refusals.rs`, 3 tests.** These exist because no public CLI path reaches a ledger `require_core1` with a `/2` program: every CLI path parse-refuses first. The `/2` program is built with `stage::load(delivery.mori)` and passed as `.program()`. Fixtures are copies of the existing public `static_compile.rs` fixtures; the policy uses the public Jubjub generator, with no signing key. The tests do no file I/O and use no SDK.
- **`static_compile_refuses_core_two_for_every_authorization_before_limits_or_lowering`**
  - `static_compile::compile` (`:370`) with each of Unsigned, UnsignedMappedRoles and SignedV2, using both valid limits and `max_degree:0` limits.
  - Expected: `(layer, code) == ("profile","core1-required")` and Display `mori::ledger::profile: core1-required`.
  - If the limits were checked first, the invalid ones would give `input/native-work-limits`. Getting `core1-required` for both shows the guard comes first, ahead of the budget and of `compile_mapped_roles` / `compile_source_static_roles`.
- **`signed_static_admission_refuses_core_two_before_policy_checks`**
  - `SignedStaticBindings::from_source` (`signed/mod.rs:210`) with each of LegacyV1 and ConsumingV2, using both the valid and a wrong policy profile.
  - Expected: the same identity in every case.
- **`core_one_program_passes_the_profile_guard_to_the_next_typed_refusal`** (pure `/1` control)
  - `from_source` on the `/1` program → `native-relation/source-identity`: the source-hash mismatch short-circuits before parse.
  - `compile` with `max_degree:0` → `input/native-work-limits`.
  - This shows the guard is profile-specific. No lowering is reached.

**A9 coverage not pinned, and why**
- `relation/mod.rs:361` (`compile_source_static_roles`) is `pub(crate)`. It is reachable only behind the `static_compile::compile` guard above.
- The `run_static`/CompileCall static path, `relation::compile`/`admit` and `static_compile::reopen` all refuse `/2` structurally, by parse or source-identity, before their guard.
- Preparation needs a `SignedStaticBindings`, and a `/2` program cannot be used to construct one. I deliberately did not call `prepare_*`.
- The Await arms (`ops.rs:332`, `compiler.rs:512,521`, `signed/mod.rs:307`, `layout.rs:261`) cannot be reached from public APIs, because `require_core1` refuses an Await even under a `/1` label.
- **Observation, not changed:** `layout::admit_source` runs `mori_core::validate` before `admit_bindings` → `require_core1`. A `/2` program therefore gets `source-profile/core-validation` there. It still fails closed, and A9 does not name this function. I made no production change.

## A-D1: documentation (both files, Opus-authored, no change to the 78 count)

I added one paragraph after the resume/rejection text in `CORE-CONTINUATIONS.md`, and one after the labels/frozen-contract paragraph in the examples README. They say:
- `continuation.logical.cap` is outside both digests (`program_hash`, `plan_hash`), and a caller can edit, remove or raise it without detection.
- It is a local advisory ceiling, not an authenticated lifetime work budget.
- Resume still checks:
  - `committed` against the planned prefix for the stored `site`;
  - a resume `--logical-cap` against a present stored cap (it may not exceed it);
  - the effective cap (the flag if given, otherwise the stored cap) against `committed` plus the next segment.
  
  These are verified against `cost.rs:183-216`.
- None of this authenticates the file or records its consumption.

Existing source limits, static limits and all open parents (U/M/MC/SP/G, `/4` gates, native 0) are unchanged. The "78 distinct focused Rust test functions" text is unchanged, because the new tests are unexecuted. After execution root must update it with actual counts: 78 + 7 if all pass.

## Command classification (ordinary; only these exact commands)

**C-A** `cargo +1.99.0 --offline --locked test -p mori --test core_continuations -j2 -- --test-threads=1`
**C-B** `cargo +1.99.0 --offline --locked test -p mori-ledger --test core_profile_refusals -j2 -- --test-threads=1`

Both run under the existing flock 120 / timeout 300 wrapper, the shared ordinary Cargo target directory, and the environment scrub from OPUS-CLI-CHECK-CLASSIFICATION C2. Each runs once, serially, with no retry.

**Verdict: ORDINARY (author classification)**, conditional on the following:
- **K1.** Root integrates exactly the `proposed/` bytes listed above and confirms every relied-on pin below before and after the run.
- **K2.** The foreign `stage_relation.rs` modification is resolved or acknowledged. It is not compiled by C-A or C-B.

**Anticipated selected tests**
- C-A: 11 (the 7 existing plus 4 new).
- C-B: 3. Each `--test` selects exactly one auto-discovered target. Neither `Cargo.toml` has `[[test]]` or `autotests` overrides, and no crate in the workspace has a `build.rs`.

**Runtime reach**
- **C-A:**
  - The 7 old tests are as classified for `cli-normal-03`.
  - The 4 new tests reach only file reads of the test's own temp files, Core `language`/`parse`/`stage::load`/`require_core1`, Target/Scenario serde, `bounded` JSON export and JSON/diagnostic printing.
  - `simulate --trace` normally belongs to the blocked class. It is ordinary here only because the `/2` source is refused at `signed_relation.rs:676`, before any compile or SDK call (traced above).
  - Side effects: one `mori-core-continuations-<pid>-<n>` temp directory per test, holding at most 3 files and removed on `Drop`, including during panic unwind. A timeout kill leaves directories behind, which root removes after inspecting them (C6). No `out` directory, sidecar or deployment file is created.
- **C-B:** pure in-process calls: Core `stage::load`/`parse`, ledger guards, `FixedSignerPolicy::validate`, a Jubjub point decode of the public generator, and `NativeWorkLimits::validate`. The test does no file I/O at runtime (`include_str!` happens at compile time), creates no temp directories, and uses no network, keys, parameters, SRS or cache.

**Compile-time residual**
- Both commands compile and link the midnight SDK crates (mori-ledger has `midnight-ledger` with `proof-verifying`, plus `midnight-proofs` and `midnight-curves`).
- I did not read registry sources. Those crates may embed public verifier constants or parameters at compile time. That would be public data embedded in the binaries, not a runtime private-key/SRS/cache read, and none of the selected tests calls a verifier or prover path.
- The only workspace build script previously reported, in `onchain-runtime`, is inert under the default `vendored` feature (OPUS-CLI-CHECK-CLASSIFICATION).
- C-B builds the same mori-ledger dependency graph as the already-run `ledger-source-closure-02`. Building `-p mori-ledger` alone may unify features differently and so rebuild some objects in the shared cache; that is ordinary.

**Relied-on pins (sha256 prefixes at the time of reading)**
- Fixtures: `delivery.mori` 2eb952dd…, `atomic-compatibility.mori` 4ad04eed…, `normal-oracle.json` 76075b63…
- CLI: `simulate.rs` 72488636…, `inspect.rs` 5cef6e03…, `compile_call.rs` 8a8ebae6…, `core_io.rs` 391fd301…, CLI `replay.rs` 41c907d7…
- Backend: `replay.rs` bd035651…, `signed_relation.rs` 7a810ed1…, `stage/mod.rs` 69fdceb1…
- Core: `parser.rs` 83c8d4fa…, `stage/profile.rs` 93356046…, `stage/mod.rs` 428246f8…, `stage/cost.rs` 6edebd92…
- Ledger: `layout.rs` 54174ce9…, `static_compile.rs` d829d34c…, `signed/mod.rs` d9674af1…, `profile.rs` 8bde32c8…, `lib.rs` 3631afbc…, `Cargo.toml` 821e3415…
- `crates/mori/Cargo.toml` 0bcaf62e…

## Limits

- **Unexecuted.** The tests have not been compiled or run, and a typo-level compile error is possible.
- **Unverified premises:**
  - miette's exact unicode glyphs and its no-code header;
  - `serde_json` `Value` equality for `offset:21`;
  - `BTreeSet::from` an array via `.into()`;
  - that `StaticAuthorization` variants and `FixedSignerPolicy` fields are constructible from an integration test (they are, by the existing `static_compile.rs`).
- **Read only in part:** the miette and textwrap internals and the registry `build.rs` files. I did not read them.
- **Still open:** all historical findings (A-1…A-4, scope B, the integrated Opus review), and every U, M, MC, SP and G parent. Native authority stays 0.
- **Effort:** this exceeded the 650 s target.
