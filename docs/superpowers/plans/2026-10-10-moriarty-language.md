# Moriarty Language Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to implement this plan task-by-task.

**Goal:** Update Moriarty itself with composable bounded actions, deterministic reference execution, ZKIRv3 compilation, signing/domain and resource support, and financial library primitives needed by M1–M17.

**Architecture:** Add a versioned native financial core alongside the preserved beta/1 frontend. A flat expression arena and ordered operations carry exact integer semantics, source origins, complete effects and replay steps. CLI commands consume this public library; a portable backend lowers that same checked program to ledger-9 ZKIRv3. Quint models constrain acceptance and lifecycle behavior independently of cryptographic assumptions.

**Tech Stack:** Rust 1.99/edition 2024, existing Mori lexer/parser crates, serde, SHA-256, ledger-9 ZKIRv3, Quint 0.32.

## Global Constraints

- All source edits stay in this Moriarty checkout; external repositories are read-only reference material.
- User approved the design and agent execution; latest steering selects language development, not Fenrir development.
- Preserve beta/1 parsing, checking, formatting and all existing fixtures.
- No fixture-specific compiler lookup, unchecked native escape, arbitrary recursion, floating point amounts, invented proof success or devnet qualification.
- Scalar arithmetic uses checked u128; overflowing and undefined operations reject atomically. Expression nodes use flat indices.
- Target identifiers, network/anchor binding, exact asset identity and resource limits are explicit versioned data.
- Sampling in Quint is not exhaustive verification or cryptographic soundness.
- General compile does not consume scenario state or project review/campaign records.
- Existing wallet identities, credentials and external dirty work remain intact.
- Mandatory whole-candidate independent GPT-6.1 Sol high and Grok 4.6 high reviews retain available returned identities separately from requested routing, per the October 10 user update. Earlier Astra reviews remain historical evidence.

## Task 1: Native composable language and reference execution

**Files:** Create `crates/mori-core/Cargo.toml`, `crates/mori-core/src/{lib,ir,parser,runtime}.rs`, `crates/mori-core/tests/language.rs`, `examples/core/language/*.mori` and scenario fixtures.

**Interface contract:** `parse(source: &str) -> Result<Program, Error>`; `simulate(program: &Program, action: &str, scenario: &Scenario) -> Result<Execution, Error>`; `replay(program, action, scenario, expected: &Execution) -> Result<(), Error>`. Public serde IR and exact decimal-string amounts; asset identities include network/issuer/reference/scale. `Program` contains bounded named state declarations, typed inputs, a flat expression arena and ordered actions. Runtime returns deterministic pre/post state, ordered steps and complete financial effects. Publish the concrete struct schema to backend/root before completing.

- [ ] Write failing tests for two distinct source-authored actions (counter/conditional payment), multiple action selection, sequential dependencies and rollback.
- [ ] Parse `language "moriarty/1"; agreement NAME { ... }` into flat IR, using existing lexer where useful. Support state, exact asset declarations, typed UInt inputs, let, requires/ensures, next assignment, emit and composable financial transfer/fee/liability operations. Reject unsupported constructs explicitly.
- [ ] Validate names/types, finite bounds and conflicting identities; execute checked arithmetic in source order on cloned state.
- [ ] Test overflow/underflow, wrong/missing/extra inputs, incomplete balances, unsupported syntax, conservation, distinct assets, fee/gross/net accounting, liability persistence and replay divergence.
- [ ] Run `cargo test -p mori-core`; preserve meaningful negative results and report actual support limits.

## Task 2: Portable general ZKIRv3 backend

**Files:** Create `crates/mori-backend/{Cargo.toml,src/lib.rs,tests/compile.rs}`, `toolchains/ledger-9.json`; add pinned compiler dependencies only when necessary. Root owns shared workspace manifest.

**Interface contract:** `compile(program: &mori_core::Program, target: &Target) -> Result<Artifact, Error>`. Artifact is independent of scenario state, contains one circuit per action, source/program/semantics/target identities, resource obligations and genuine ledger-9 ZKIRv3 bytes/text. Backend consumes Task 1's published schema, never reparses its own source or computes success flags on the host.

- [ ] Write tests for independently authored contrasting programs and mismatch/unsupported refusals.
- [ ] Inspect frozen ledger-9 IR serializer/instructions, pin actual full revision and reusable subset. No absolute external paths in shipping dependencies, no whole Fenrir applications vendored.
- [ ] Lower exact state/input arithmetic, predicates, updates and supported complete effects with range/overflow constraints; constrain public predecessor/poststate and action identity. Reject unsupported nodes, unbound witness or missing effects.
- [ ] Deserialize emitted artifacts with actual pinned upstream ZKIR parser and compare constrained execution/reference outcomes where feasible.
- [ ] Report measured versus unmeasured k/size/replay limits separately; never claim a local artifact is a proof/deployment.
- [ ] Run backend tests. Backend owns only its crate/profile, coordinate shared edits through root.

## Task 3: Quint language QA expectations

**Files:** Create `experiments/moriarty-language/formal/quint/language/{acceptance,acceptance_test,lifecycle,lifecycle_test,batch,batch_test}.qnt` and README.

**Interface contract:** Named safety invariants and non-vacuous deterministic witnesses covering atomic complete effects, source/domain/anchor/asset identity, cumulative authority, nonce/head, fee sponsorship and persistent duties. Use symbolic verified premises explicitly, not claims of native crypto guarantees.

- [ ] Implement one acceptance action and passing witness first; typecheck incrementally.
- [ ] Add accept/reject and concurrency witnesses: valid transfer, sponsored fees, same nonce, stale predecessor, wrong binding/asset, insufficient balances, gross cap despite refunds, missing effects.
- [ ] Add lifecycle reservation/timeout/late success/recovery and basic uniform-price/oracle/bounds witnesses for library support.
- [ ] Run `quint typecheck`, `quint test`, seeded `quint run` with nonzero accepts and rejects; retain actual outputs.
- [ ] Publish expected implementation controls and a correspondence table naming unsupported runtime predicates.

## Task 4: CLI consumer and signing/resource/library support (root)

**Files:** Modify `Cargo.toml`, `crates/mori/Cargo.toml`, `crates/mori/src/{cli,error}.rs`, command modules. Create `crates/mori-core/src/{intent,policy}.rs` only after coordinating ownership; create library fixtures/docs.

- [ ] Add workspace dependencies and source-profile routing; `check` supports core/1 while beta/1 behavior remains.
- [ ] Add `compile --target PROFILE --out DIR` consuming actual backend artifacts, with explicit errors.
- [ ] Replace simulate/test panic stubs with direct native library calls and structured rejection/report output. Test manifests use source/action/scenario and expected accepted/error/effects rather than blindly counting scenarios.
- [ ] Implement canonical intent preparation that binds program/semantics/network/anchor/action/predecessor/assets/nonce/validity/gross/fee/net/liability authority. Actual signature verification must use a real verifier and reject unsupported schemes; canonical comparison alone is not signature verification.
- [ ] Add fee-capability, domain/role policy and deterministic counters with explicit cost planes. Library examples express conditional pending/recovery, batch price bounds and scoped operations in supported language.
- [ ] Run Rust workspace and beta compatibility tests, CLI smoke on authored source and negative cases. Review exact candidate with fresh required auditors.

## Remaining delivery gates

All M1–M17 remain in the approved requirement matrix. Local language delivery is distinct from proof generation, strict in-circuit Schnorr correspondence, live deploy/call/readback and external devnet qualification. Implement later slices against these interfaces as dependencies become concrete; no Fenrir fixes and no reclassification of unperformed gates as completed.
