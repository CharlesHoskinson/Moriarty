# Backend check preparation — source inspection only

Requested result: prepare two bounded backend integration checks for Opus classification. Predicate to test later: the selected tests exercise the public staged compiler and compare emitted IR/layout/metadata with fixed expectations. This handoff grants no execution authority and makes no security judgment. Installed develop, AGENTS and guarded status were refreshed. The historical guarded loan/swap queue was not dispatched.

## Source-path inventory

All paths below are relative to `/home/charl/Moriarty/.worktrees/core-continuations-20261015`.

| Entry / operation | Concrete source closure observed |
|---|---|
| Public API | `crates/mori-backend/src/lib.rs` declares `pub mod stage`; `src/stage/mod.rs` reexports `compile_staged` and the distinct artifact types. |
| Staged entry | `crates/mori-backend/src/stage/lower.rs:18` calls Core `stage::load`, Target validation, whole-program `liabilities::registry`, `frame::layouts`, and shared `lower_action_segment`. The `out` argument is informational; this backend function performs no publication. |
| Core source and plan | `crates/mori-core/src/stage/mod.rs::load` reaches `parser::parse_v2`, profile, validate, plan, cost and digest modules; parser reaches `mori-lexer`. Canonical hashing reaches Core `intent::canonical`. These are the eight Core `src/stage/*.rs` modules plus `parser.rs`, `runtime.rs::expr_type`, `ir.rs`, `logical_cost.rs`, `json.rs`, `intent.rs`. |
| Financial and scalar lowering | `crates/mori-backend/src/lib.rs::lower_action_segment` uses the existing `Builder` expression/arithmetic/public methods, full original action balance keys and full program registry from `src/liabilities.rs`. It lowers only each layout's statement range, retaining original ordinals. |
| Frame and context | `src/stage/frame.rs::{layouts,emit_pre,assert_guard,emit_post}` supplies frame wires. `src/stage/context.rs::stage_context_hash` reaches `signed_relation::{Projection,json}`, backend `digest`, canonical serialization and SHA-256. Context limbs use `authorization::digest_limbs`. |
| Serialization/parser | Shared lowerer serializes `IrSource`, sets version 3.0, and calls `midnight_zkir_v3::IrSource::load` for round-trip equality. At pinned SDK `04c9c5d`, `zkir-v3/src/ir.rs:955`, the load body is serde JSON read, version match/normalization, then serde from_value. The adjacent `model` method is separate and is not called here. `proving::hex` called here only iterates bytes and formats hexadecimal. |
| Resources | `src/stage/lower.rs` sums instruction/IR-byte/public-value counts, copies plan work into `logical_resources`, and applies the existing 12-iteration pretty JSON envelope-size procedure. This is source/metadata inspection, not measured circuit cost. |
| Artifact decode | `src/stage/artifact.rs` serde `TryFrom<StagedArtifactFields>` reaches its local validation helpers, `authorization::decode_digest`, Target validation, `signed_relation::validate_shape`, and pretty JSON byte-count comparison. `tests/stage_relation.rs` calls this through `serde_json::from_value`. |
| Legacy projections | `tests/stages.rs` calls legacy Core parse/backend compile on unchanged counter/payment source, then compares existing snapshot text. The signed layout branch is not selected by their unsigned Targets. |

Build/link scope is larger than selected runtime calls: the backend library declares `authorization`, `liabilities`, `parameters`, `proving`, `replay`, `schnorr`, `signed_relation`, and all five stage modules. Native/proving functions therefore exist in the compiled crate even though the selected test bodies do not call them. The test-only `task4_native_driver.rs` is behind `cfg(test)` in proving; these two integration targets do not select backend library unit tests. The local dependency source also includes `mori-core`, `mori-lexer`, `mori-diagnostics`, and `mori-span`, with their manifests. Third-party proc macros/build scripts and inherited Cargo/config/compiler environment remain part of Opus's build-closure classification; this report does not certify them.

## Selected checks and exclusions

- `--test stages`: **five active tests and one ignored comparator**. Covers the frozen escrow two-stage and reserve three-stage specimens, scalar expressions/zero-await/declaration order, whole-program duty registry, and existing counter/payment projections. Operand checks read ordered PublicInput assignments and ConstrainEq instructions to locate held/quote data flow. Assertions parse IR and inspect its structure; they do not execute it.
- `--test stage_relation`: **four active Opus-authored tests**, exercising recorded pending positions, context/gate metadata, artifact decoding and legacy compile refusal. Their authorship and judgment remain Opus-owned.
- Six `stage::frame::tests` unit tests exist but are **not selected** by either proposed command. No unit-test pass is implied.
- The full artifact/IR baseline comparator stays ignored. It requires separately generated immutable baseline files and `MORIARTY_STAGE_V1_BASELINE`; neither baseline generation nor `--ignored` is proposed.
- Broad `cargo test -p mori-backend`, `--tests`, `--workspace`, doctests and existing legacy native/proving suites are outside these two proposed checks. Public CLI, ledger, native correspondence and full parent exits remain separate.

## Exact proposed commands

These reproduce the existing `BACKEND-PROPOSED-CHECKS.json` arrays without changing that packet. CWD is the worktree above. Run sequentially and stop on the first failure **only after Opus classification and root dispatch**. Root retains responsibility for the classified child environment/config controls. No command was run by this preparation task.

```bash
flock -w 120 /tmp/moriarty-language-wave1-build.lock timeout --signal=TERM --kill-after=10s 300s env -u RUSTFLAGS -u MORI_FEE_POOL_CAPTURE_DIR CARGO_TARGET_DIR=/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target cargo +1.99.0 --offline --locked test -p mori-backend --test stages -j2 -- --test-threads=1
```

```bash
flock -w 120 /tmp/moriarty-language-wave1-build.lock timeout --signal=TERM --kill-after=10s 300s env -u RUSTFLAGS -u MORI_FEE_POOL_CAPTURE_DIR CARGO_TARGET_DIR=/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target cargo +1.99.0 --offline --locked test -p mori-backend --test stage_relation -j2 -- --test-threads=1
```

## Pins and current integration status

Read-only SHA-256 comparison found **all 48 entries in `BACKEND-CHECK-SOURCE-PINS.json` matching current bytes**. Root has also supplied `BACKEND-CHECK-ADDITIONAL-INPUT-PINS.json`, covering the following direct include/toolchain leaves and retained Core/lexer dependencies. The table records the inspected leaf hashes:

| Path | Current SHA-256 |
|---|---|
| `rust-toolchain.toml` | `72f7dd8c9fdf9c1fcd8eb8bf9326d4bc66833e4ebf4bc2932636e2b85efcb7af` |
| `crates/mori-backend/tests/programs/counter.mori` | `8db5630f0fedbadc8ef5824a6f39afcaaec5113049cc0dac45e3bbc69979db52` |
| `crates/mori-backend/tests/programs/payment.mori` | `f1f4807bc53b62e6ad8f816c44aebb92011925935a63479f4e3afa025a29e786` |
| `crates/mori-backend/tests/snapshots/compile__authored_counter_abi.snap` | `49cc42f9b64a7eb1dbeb710fd7defc76d4370e59b94ba6167b68365a82ac93cb` |
| `crates/mori-backend/tests/snapshots/compile__authored_payment_effects.snap` | `5c0c29b8579628905d9e4440fe129dc0eecf7c187dea701b0c49cc9a93d4af14` |

The packet also omits `crates/mori-lexer/{Cargo.toml,src/*.rs}`, `crates/mori-diagnostics/{Cargo.toml,src/*.rs}` and `crates/mori-span/{Cargo.toml,src/lib.rs}`. The additional-input packet now covers these dependencies. A read-only comparison found all 54 additional entries matching current bytes. Root will verify both packets before and after any approved checks. Workspace `Cargo.toml`/`Cargo.lock` pin SDK revision `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`; SDK dependencies include midnight-zkir-v3 3.0.0-rc.2 and transient-crypto 3.0.0. Repository `.cargo/config.toml` is absent; ancestor/home configuration was not inspected here.

The earlier concrete API mismatches are corrected in current source: Projection/json visibility now permits the context import; guard helper emits Instruction::Assert; module/type reexports and frame/context caller signatures align. The queued backend proving Await refusal is now present. No further concrete missing symbol or signature gap was found by source inspection. Compilation remains unperformed, so this is not a successful type-check claim.

No backend test result is claimed. The parent reports 32 passed Core tests; those are separate evidence. This task ran no Cargo, build, test, formatter, SDK relation/model, VM, proving, native or ledger operation, and changed no product/Core source. Full candidate/result Muse audits and Opus reviews remain outstanding; all U/M/MC/SP/G exits remain open.
