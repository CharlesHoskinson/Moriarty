# Target-bound reference replay implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans inline, task by task. The assigned GPT-6.1 Sol HIGH writer owns all edits; no subagents. Root clears the frozen design and independent expectations before any feature implementation. Independent reviewers remain separate from implementation.

**Goal:** Generate and independently consume a bounded, public, target-bound replay sidecar through actual Moriarty CLI commands with first-step and deterministic field diagnostics.

**Architecture:** Add detailed comparison to Core without modifying its existing replay or serialized execution. A focused backend replay module reconstructs trusted source/target/signature inputs and shares the exact existing v3 Claim constructor; a separate version1 record binds its public result. CLI trace generation and replay dispatch use bounded existing IO.

**Tech Stack:** Rust1.99, existing serde/serde_json/sha2/clap, unchanged pinned ledger9 ZKIRv3 and Core. No new dependencies.

## Global constraints

- Normative byte/guard/API contract: [bound replay design](../specs/2026-10-11-bound-replay-design.md); preserve all original October10 M1–M17/U0–U7 scope/exits/history.
- Worktree `/home/charl/Moriarty/.worktrees/bound-replay-sol-20261011`; baseline `31b392b972ecb054b8385019b1184f0b2ead80b7`, tree `89e9f5d00ed648dc243d6dbe249d6c041ad110e8`.
- Keep Program/Step/Execution, signing/preparation/Claim/context/authorization/artifact/ABI/relation/proof/binding revision3 canonical bytes and native independent reconstruction unchanged.
- Trust only caller source/action/Scenario/Target and separately supplied authority/at/attachment. Never trust record roots, verification/status flags or candidate poststate.
- Sidecar is `moriarty-reference-replay/1`, strict/public-only, 1MiB/depth32/nodes65536, full bounded ASCII-safe export; no truncation or private witness/point memory/R/s/SRS/keys/credentials.
- Execution comparison precedes trace-sensitive Claim/digest equality. Record schema validation checks encodings/shape only and cannot preempt earliest semantic divergence with recalculated digests.
- No SRS read, keygen/prove/native verify/MockProver/ignored/native tests, reused budget, installation/upgrade/network/push/merge. Original/signed-QA worktrees and Fenrir are immutable.
- Commands use `RUSTUP_TOOLCHAIN=1.99.0 CARGO_TARGET_DIR=/home/charl/Moriarty/.worktrees/readiness-20261010/target cargo ... --locked --offline`. Existing dependencies only.
- Runtime evidence directory `/home/charl/Moriarty/.moriarty-dev/runtime/bound-replay-20261011/`; public oracle is separately authored there. Root's prior Core baseline:30pass/0fail, no proofcalls.

## Task1: Detailed Core comparison with compatible replay

**Files:** Create `crates/mori-core/src/replay.rs`; modify `crates/mori-core/src/lib.rs`; create `crates/mori-core/tests/replay_detailed.rs`. Leave ir.rs and existing runtime replay unchanged.

**Interfaces:** Produce `ReplayDifference {step:Option<u16>,field:String}`, `DetailedReplayError::{Execution(Error),Divergence(ReplayDifference)}`, `compare_execution(actual:&Execution,candidate:&Execution)->Result<(),ReplayDifference>`, `replay_detailed(program:&Program,action:&str,scenario:&Scenario,candidate:&Execution)->Result<(),DetailedReplayError>`.

- [ ] Add tests with the independent oracle's exact full Execution. Mutate steps1 and4 together: assert step1 and `/steps/1/effects/0/amount`. Change statement/index/value/snapshot/effects independently; assert the design's within-step ordering. Test a missing/extra first step, ordered effect changes, finalpost, untouched state/balance/duty and absent-versus-present-zero.
- [ ] Add escaped-name tests (`~` and `/`) proving RFC6901 pointer escaping, lexical object-key ordering and array ordinal behavior. Compare complete effect list after steps, then post. A later mutation may not change the first report.
- [ ] Run focused test before implementation: `cargo test -p mori-core --test replay_detailed --locked --offline`; expect missing new API failure.
- [ ] Implement the bounded typed-to-JSON comparison: explicit Execution/Step field order, recursive lexical object union keys, ascending array elements then length, scalar equality, pointer escaping. Do not use Debug strings, candidate index or HashMap order. `replay_detailed` first simulates trusted inputs, then compares. Error::Execution contains no partial execution.
- [ ] Run that test and `cargo test -p mori-core --locked --offline`; expect all tests pass. Confirm existing replay errors remain unchanged with retained compatibility assertions. Commit focused Core change.

## Task2: Share exact v3 public Claim without proof activity

**Files:** Modify `crates/mori-backend/src/proving.rs`; create `crates/mori-backend/tests/replay_claim.rs`.

**Interfaces:** Public typed `proving::TransitionClaim` with exactly the previous private Claim fields; crate-private `claim_from_reconstruction(statement:&signed_relation::Reconstruction,action:&str,scenario:&Scenario,target:&Target,circuit_k:u8)->Result<TransitionClaim,Error>`. Keep native prepare's options/IR/capacity/report guards at the same points. Claim shape validation remains current v3 ProofBundle rules, with required explicit authorization null.

- [ ] Add a nonnative compatibility test whose expected JSON is the public published `deliverables/native-owner-intent-2026-10-11/bundle.json` with only proof_hex removed. Independently reconstruct from `inputs/independent-payment.{mori,scenario.json,target.json,authority.json,signed.json}` and at15. Assert full C(Claim), all old identity/context/authorization/public-values fields, old execution_hash and SHA256(C([BINDING_VERSION,Claim])) match. Do not call native verify or read proof parameter/key files.
- [ ] Preserve signed golden-vector canonical payload/message/context and existing compiled snapshots through exact-byte assertions. Test unsigned Claim authorization:null and current descriptor/schema encoding validation. No synthetic proof status is an expected value.
- [ ] Extract current Claim construction into the named helper using the same field expressions and order, derive only needed Clone/Deserialize/public visibility. Factor its existing validate_claim shape checks without adding digest-equality guards to sidecar decoding or weakening native verification.
- [ ] Retain `prepare` reconstruction mode, IR/circuit_k/limit checks, full Claim binding modulo-Fr computation and bounded proof/report preflight order. Call the helper where Claim was previously built. Existing `check` still removes only proof_hex and compares independently reconstructed entire Claim before parameters.
- [ ] Run `cargo test -p mori-backend --test replay_claim --locked --offline`, `cargo test -p mori-backend --test compile --locked --offline`, `cargo test -p mori-backend --test authorization --locked --offline`, `cargo test -p mori-backend --test signed_relation golden_ --locked --offline`, `cargo test -p mori-backend --lib financial_abi_has_exact_order_and_complete_nonframe_commitment --locked --offline`, `cargo test -p mori-backend --lib digest_reduction_uses_high_limb_and_preserves_separate_source_identity --locked --offline`. Inspect the named filtered targets first; none may call prove/verify or dispatch native/ignored tests, including missing-parameter prove/verify controls. Ordinary verify-intent and curve signature checks are permitted. Commit exact Claim refactor.

## Task3: Public record generator and trusted replay consumer

**Files:** Create `crates/mori-backend/src/replay.rs`, `crates/mori-backend/tests/replay.rs`; modify `crates/mori-backend/src/lib.rs` to export the module. If source identity is needed after existing reconstruction, expose only crate-private parsed program name/language/semantics in Reconstruction, without affecting serialized outputs.

**Interfaces:** Design's exact ReplayRecord/ReplayBody/ReplayAuthorization/ReplayError and `generate`, `replay`, `ReplayRecord::from_json`. Shared Claim constructor from Task2; Core `compare_execution` from Task1. No params/ProofOptions or native entry point in this API.

- [ ] Write API tests for honest signed payment and independent contrasting unsigned program. Assert the independently supplied full Execution, every Claim identity, source-step count, selected raw-ZKIR digest, authority/input/trace/record framing and round-trip canonical record. Signed records have no attachment/witness fields; unsigned program_address/authority/authorization are required null.
- [ ] Test trusted-input substitution: raw source whitespace, different actual source/action/Scenario supplied order/target limits/network/Anchor/signed policy/address, authority key/nonce/predecessor/caps/interval, at inside/outside interval, mode mismatch and missing signature. Recompute changed record hashes; refusal remains driven by trusted reconstruction.
- [ ] Implement the focused pipeline: unsigned reconstruct(None), signed accept(actual attachment), load actual selected ZKIR for circuit_k, exact shared Claim, public identities, bounded canonical framing, final export preflight. Do not call witness or check/native/proving paths; strict FENS acceptance already validates the actual attachment. Do not infer compiled/native verification from compilation.
- [ ] Implement strict record parsing with Core parse first, required nested Step.value presence checks, deny_unknown_fields, explicit required nulls and shape/version/digest/native encodings. Every direct typed generate/replay API applies record/public JSON bounds too. Do not recompute hash equality in decode.
- [ ] Implement guard/comparison order exactly: trusted static identities, full ordered Execution via Core comparison, derived Claim execution/public/message fields, count, trace_hash, record_hash. Prefix Core pointer with `/body/execution`. Reconstructed actual values are authoritative. Return only step+field on divergence.
- [ ] Add hostile tests that mutate early semantic steps AND later fields, then recompute all sidecar hashes and Claim execution_hash. Assert earliest pointer/ordinal survives. Test reorder/drop/add steps/effects, altered post and untouched/present-zero duty, header/descriptors/public-values/context/selected-ZKIR/digest-only changes. Check no output Execution/effects on source/requires/late-ensures/overflow/target/authority failure.
- [ ] Add malformed/duplicate-escaped-key/unknown/missing/null/trailing/number/digest/native/depth/node/byte/export-bound tests, including input and escaped export limit cases. Test valid alternate same-payload attachment gives byte-identical public record; invalid attachment always refuses. Use existing public alternative attachment, no signer creation.
- [ ] Run `cargo test -p mori-backend --test replay --locked --offline` and Task2 compatibility target; expect all pass with no proofcalls. Commit backend consumer.

## Task4: Wire and exercise actual CLI generation/replay

**Files:** Modify `crates/mori/src/commands/simulate.rs`, `crates/mori/src/cli.rs`, `crates/mori/src/commands/mod.rs`; create `crates/mori/src/commands/replay.rs` and `crates/mori/tests/replay.rs`. Reuse `core_io` bounded source/json/authority/signed-intent IO; modify it only for necessary common public IO.

**Interfaces:** Trace args `--trace --target FILE` plus complete signed profile/authority/at/signature group. Replay args FILE/--action/--scenario/--target/--record plus same signed group. Ordinary simulate args/output unchanged; trace stdout is directly ReplayRecord. Replay reports reconstructed record_hash on success and typed first divergence on rejection.

- [ ] Write production binary tests (`CARGO_BIN_EXE_mori`) that assemble temporary public fixture files, invoke simulate trace, parse stdout with ReplayRecord::from_json, then invoke replay with all separately supplied roots. Assert exit0/local-reference-replay. Repeat for the independently authored contrasting source through the same commands.
- [ ] Invoke CLI with rehashed early+late tampering and assert exit1, exact ordinal+pointer and no proposed effects. Invoke trusted source/Scenario/Target/authority/at substitutions, invalid actual attachment, partial/contradictory flags and malformed/oversize record. Assert signed Target never falls back; options without --trace refuse; source rejection emits no record/Execution/effects. Diagnostics remain display-safe.
- [ ] Implement Args/Command/module dispatch, complete mode grouping and existing IO loading. Print only fully bounded exporter results. Ordinary simulate stays on its established Core path. New paths call Task3 generator/consumer and never proof APIs. Clap/file/schema errors remain existing diagnostics.
- [ ] Run `cargo test -p mori --test replay --locked --offline`, `cargo test -p mori --test signed_intents signed_preparation_reproduces_golden_and_review_names_exact_scope --locked --offline`, `cargo test -p mori --test signed_intents signed_verify_intent_accepts_exact_and_alternative_attachment_and_rejects_others --locked --offline`, `cargo test -p mori --test intent_review --locked --offline`, `cargo test -p mori --test verified_review --locked --offline`. Do not run mori native_proofs.
- [ ] Build `cargo build -p mori --locked --offline`, capture actual binary help/generate/replay/honest and tampered public outputs in the assigned runtime directory, then independently compare with the oracle. No parameter files or private witness captures. Commit CLI consumer.

## Task5: Documentation, compatibility evidence and review freeze

**Files:** Modify `docs/NATIVE-CORE.md`, `crates/mori-core/README.md`, `crates/mori-backend/README.md` and `experiments/moriarty-language/formal/quint/language/README.md`. Create `docs/evidence/2026-10-11-bound-replay.md` with exact checked command outputs/candidate identity and public artifact hashes; no hidden native acceptance inference.

- [ ] Document the working trace/replay commands, separately trusted arguments, schema/guard order, explicit limited source-step measure and signed attachment exclusion. Qualify all observations as local reference replay. Explain public record equality for two valid same-payload attachments and separate actual signature validation.
- [ ] Correct only the previously identified stale NATIVE-CORE242–244/291–292 and Quint README281–282 wording: preserve the genuine selected local signed native result and its retained original evidence, distinguish broader U1/ledger/native qualification still open. Do not modify historical raw/native reports, proof bundles or review receipts.
- [ ] Check compatibility evidence: unchanged golden signing/prepared/context outputs; unchanged artifact/ABI snapshots and exact Execution/Claim/binding canonical bytes, independently compared to published proof metadata without verification. No broad test rerun without new change/failure justification.
- [ ] Run `cargo fmt --all --check`, `git diff --check`; inspect all touched files and schema/public artifacts for private witness/point memory/R/s/keys/credentials, unsupported status flags and premature qualification. Check the exact CLI production path is used and record refuses fully rehashed tampering.
- [ ] Freeze code/docs/tests commit and tree, file SHA256s and public command outputs. Report exact passing/failing counts, commands, no-proofcalls scope and unresolved gates. Root obtains both fresh exact-candidate source and actual-result audits; writer repairs only returned substantive findings and changed bytes receive fresh audits.
- [ ] Wait for root integration direction. No push/merge or proof/resource dispatch by the writer.

## Phase-one handoff

The design/plan-only commit is review material. Independent expected cases and root design clearance remain prerequisites to feature logic, not a new user question. Full typed external Fenrir fraud-game interfaces and live qualification remain blocked external predicates; they do not block this local sidecar/CLI implementation. All other original acceptance predicates remain open until demonstrated.
