Fresh nonauthor AGY Gemini HIGH source-fidelity review of exactly two proposed full GitHub issue bodies,81 and89. Read public packet /home/charl/moriarty-c2-opus55-restored-20261017/github-issue-body-repairs-20261018/review-packet only. Full before/after bodies and exact original API captures are under before/ and after/. Public PR state/body captures, two Cargo feature sources and selected ordinary result receipts are under evidence/. Review all candidate bytes, original requests/Done-when conditions/all13 children preservation, complete source-only versus observed-local versus pending-review separation and current-status corrections. Issue81 original unconditional-dependency/default question is historical; default=native implemented, only selected no-default/Core/CLI checks have observed results; no SDK-free wholeworkspace claim. Issue89 PR74/75 are merged source-only, final stale premerge claim removed; later193/194/197/199/200/202/203 bounded results remain separate;204 source-only adoption;205 exacthead7e0d/tree55fab selected2tests52.624672563s actual but resultreviewpending. Do NOT infer source/result/security/parent approval from merge/tests. Full default/native/workspace and compiler/SDK/proof/ledger validation remains UNRUN for cleanup scope. Original acceptance and13 children remain. No remote mutations or issue closure authorized in review. Only read pinned publicpacket files: no commands, networking, config/auth/provider files, testing, SDK/proof/wallet/runtime, edits, delegation or additional research. Verify full before/after sourcefidelity against captures/receipts; return changesrequired for factual overreach or dropped originalrequirement. Return ONE whole6field JSON matching schema with reviewToken doctorwatson-github-issue-body-source-ordinary-20261018-a1-whole-review. Root handles current-state beforeBody SHA guards and separate authorized publication after review. This is exact proposed-body source review only.

--- before/issue-81.md ---
**Tier 3 — structural.**

## Problem
`RUST_GUIDELINES.md` says Git-pinned Midnight dependencies stay behind a feature. Today `mori` depends on `mori-backend` and `mori-ledger` unconditionally. Together they pull ten Git crates from midnight-ledger plus `midnight-curves` and `midnight-proofs`, and `Cargo.lock` grew from 136 to about 450 packages. Building only the beta `check` and `format` compiles all of it. The root `[patch.crates-io]` also pulls a second midnight-ledger revision (`a01a1ea`) as `midnight-transient-crypto-old`.

## Suggested fix
1. Add a `native` feature to `mori` that enables `mori-backend`, `mori-ledger` and the commands that need them, and gate those command modules with `#[cfg(feature = "native")]`.
2. Check whether `mori-core` needs `k256` outside native use, and gate it if not.
3. CI: keep `cargo test --workspace` (all features) and add `cargo check -p mori --no-default-features`.

## Needs input
Should the default `mori` build include the native commands (feature on by default) or not?

## Done when
`cargo build -p mori --no-default-features` builds the beta toolchain without any Midnight crate, and CI checks both configurations.

---
**Working on this:** builds on #74. Read `RUST_GUIDELINES.md` and `plugins/moriarty-dev/skills/develop/references/rust.md` first; a deviation from the guidelines needs agreement before it's made. Before handing off, all of these pass:

```
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## UTXO workstream adoption

Tracked as `GH-I81` in [doctormoriarty<utxo>](https://github.com/CharlesHoskinson/Moriarty/blob/utxo/docs/utxo/doctorwatson/github/work-items.json) and its [EARS/consumer assessment](https://github.com/CharlesHoskinson/Moriarty/blob/utxo/docs/utxo/doctorwatson/github/issue-triage.md).

Shared requirement lineage: UNI-001, UNI-018, UNI-024. Retained delivery exits: U0, U2, U7, MC01, MC08, SP02, SP12. Cardano candidate relevance: EUC-001, EUC-016. This is a traceability mapping, not a claim that the issue or any parent requirement is complete.

Source preparation can proceed within the current user authorization. Concrete API/identity proposals and applicable independent reviews remain required where semantics change. Original suggested fixes and historical tool counts are proposals and observations, not automatic migration or test oracles. Keep public developer use permissionless. Run checks locally; automatic hosted CI is outside the current workflow. Existing native/proving/ledger limits remain in force.

Check-matrix correction: `--workspace` selects packages; enabled features must be stated separately. A default-feature workspace run and an explicit `--all-features` run are different configurations. Keep default/no-default/native configurations explicit and run only the currently permitted local checks.



--- after/issue-81.md ---
**Tier 3 — structural.**

## Problem
The following is the original problem report, retained as historical context rather than a current dependency-graph claim:

`RUST_GUIDELINES.md` says Git-pinned Midnight dependencies stay behind a feature. Today `mori` depends on `mori-backend` and `mori-ledger` unconditionally. Together they pull ten Git crates from midnight-ledger plus `midnight-curves` and `midnight-proofs`, and `Cargo.lock` grew from 136 to about 450 packages. Building only the beta `check` and `format` compiles all of it. The root `[patch.crates-io]` also pulls a second midnight-ledger revision (`a01a1ea`) as `midnight-transient-crypto-old`.

**Current source and selected evidence (2026-10-18).** [PR #193](https://github.com/CharlesHoskinson/Moriarty/pull/193) merged the opt-in, nondefault `midnight-compat-tests` dependency boundary for Core. [PR #199](https://github.com/CharlesHoskinson/Moriarty/pull/199) merged the CLI's default-enabled `native` feature and explicit `--no-default-features` configuration; the backend, ledger and native dependencies are optional in that configuration. The command schema is preserved, and native-required routes return a typed capability refusal. [PR #200](https://github.com/CharlesHoskinson/Moriarty/pull/200) merged its selected non-native boundary tests. Thus the original unconditional-dependency premise and unanswered default-feature question below describe the earlier tree; the implemented default is `native` enabled. This does not resolve every item in this issue.

The captured PR records report only bounded local results on their exact trees: #193/#194 together passed five existing continuation and two financial tests; #199 passed twelve selected CLI integration tests; #200 passed two tests covering 27 cases and 44 CLI invocations. The selected #199 build log contains no Midnight/backend/ledger compilation. These are separate observations, not one combined current-tree suite pass or an SDK-free whole-workspace claim. Full default/native/workspace checks and compiler, SDK, proof and ledger validation for this cleanup remain UNRUN. The native compatibility test, complete beta check/format installation/configuration matrix and the original `k256` question still require explicit reconciliation.

## Suggested fix
1. Add a `native` feature to `mori` that enables `mori-backend`, `mori-ledger` and the commands that need them, and gate those command modules with `#[cfg(feature = "native")]`.
2. Check whether `mori-core` needs `k256` outside native use, and gate it if not.
3. CI: keep `cargo test --workspace` (all features) and add `cargo check -p mori --no-default-features`.

## Needs input
Should the default `mori` build include the native commands (feature on by default) or not?

Current implementation records the choice as default-on in #199. The original question remains above for provenance; completing this issue still requires the original full configuration and acceptance checks, rather than another default-choice blocker.

## Done when
`cargo build -p mori --no-default-features` builds the beta toolchain without any Midnight crate, and CI checks both configurations.

---
**Working on this:** builds on #74. Read `RUST_GUIDELINES.md` and `plugins/moriarty-dev/skills/develop/references/rust.md` first; a deviation from the guidelines needs agreement before it's made. Before handing off, all of these pass:

```
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## UTXO workstream adoption

Tracked as `GH-I81` in [doctormoriarty<utxo>](https://github.com/CharlesHoskinson/Moriarty/blob/utxo/docs/utxo/doctorwatson/github/work-items.json) and its [EARS/consumer assessment](https://github.com/CharlesHoskinson/Moriarty/blob/utxo/docs/utxo/doctorwatson/github/issue-triage.md).

Shared requirement lineage: UNI-001, UNI-018, UNI-024. Retained delivery exits: U0, U2, U7, MC01, MC08, SP02, SP12. Cardano candidate relevance: EUC-001, EUC-016. This is a traceability mapping, not a claim that the issue or any parent requirement is complete.

Source preparation can proceed within the current user authorization. Concrete API/identity proposals and applicable independent reviews remain required where semantics change. Original suggested fixes and historical tool counts are proposals and observations, not automatic migration or test oracles. Keep public developer use permissionless. Run checks locally; automatic hosted CI is outside the current workflow. Existing native/proving/ledger limits remain in force.

Check-matrix correction: `--workspace` selects packages; enabled features must be stated separately. A default-feature workspace run and an explicit `--all-features` run are different configurations. Keep default/no-default/native configurations explicit and run only the currently permitted local checks.



--- before/issue-89.md ---
Follow-up cleanup from the review of the Rust crates added since the beta toolchain. PR #74 and PR #75 are now merged as reviewed source-only integration. Tier 1 source changes include shared dependencies and diagnostics, source-list coverage and Rust guidance; the quality workflow is manual only. Current Rust/Cargo and native/ledger validation remain UNRUN. The original author's macOS checks describe the earlier candidate and do not qualify this integrated source.

Each issue lists its problem, a suggested fix, what still **needs input**, and when it's done. Issues whose *Needs input* section names a decision need maintainer agreement before an agent starts on them.

## Tier 4: decision first
- [ ] #76 Decide how the Core language relates to the beta frontend (blocks #82 and #83)

## Tier 3: structural (agree the approach first)
- [ ] #77 Derive build and ABI identities from artifacts, not source text
- [ ] #78 One Program-to-ZKIR lowering path for mori-backend and mori-ledger
- [ ] #79 Move Sherlock into its own crate and build it only through Cargo
- [ ] #80 Move business logic out of the mori CLI (blocks most of #85)
- [ ] #81 Put the Midnight ledger dependencies behind a feature
- [ ] #82 Route Core, backend, ledger and CLI errors through MoriDiagnostic
- [ ] #83 Remove duplicated logic inside mori-core

## Tier 2: test cleanup (mostly no decisions needed)
- [ ] #84 Move test fixtures into crates/fixtures
- [ ] #85 Replace mori binary tests with library tests
- [ ] #86 Load Sherlock oracles from JSON instead of generated Rust
- [ ] #87 Snapshot and fuzz the Core and Sherlock frontends
- [ ] #88 Remove env-var-driven test behavior and the task4 driver from proving paths

## Suggested order
1. Tier 2 issues that need no input: #84, #86 (proptests in #87), #88.
2. Decide #76, then #77 and #80.
3. The rest of tier 3, then #85 and the error snapshots in #87.

Every change follows `RUST_GUIDELINES.md` and `plugins/moriarty-dev/skills/develop/references/rust.md`. Before handoff, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace` pass.

## UTXO workstream adoption

Tracked as `GH-I89` in [doctormoriarty<utxo>](https://github.com/CharlesHoskinson/Moriarty/blob/utxo/docs/utxo/doctorwatson/github/work-items.json) and its [EARS/consumer assessment](https://github.com/CharlesHoskinson/Moriarty/blob/utxo/docs/utxo/doctorwatson/github/issue-triage.md).

Shared requirement lineage: UNI-018, UNI-019, UNI-020, UNI-021, UNI-023, UNI-024. Retained delivery exits: U0, U1, U2, U3, U4, U6, U7, MC01, MC03, MC04, MC05, MC07, MC08, SP02, SP03, SP04, SP05, SP06, SP08, SP09, SP11, SP12. Cardano candidate relevance: shared-language/release dependency; no automatic Cardano scope transfer. This is a traceability mapping, not a claim that the issue or any parent requirement is complete.

Source preparation can proceed within the current user authorization. Concrete API/identity proposals and applicable independent reviews remain required where semantics change. Original suggested fixes and historical tool counts are proposals and observations, not automatic migration or test oracles. Keep public developer use permissionless. Run checks locally; automatic hosted CI is outside the current workflow. Existing native/proving/ledger limits remain in force.

All thirteen children #76–#88 remain open. PR #74 is being repaired as a source-only candidate; its current coverage and review gaps must be resolved before merge. Source-only cleanup does not establish current Rust, Cardano or Midnight acceptance.



--- after/issue-89.md ---
Follow-up cleanup from the review of the Rust crates added since the beta toolchain. [PR #74](https://github.com/CharlesHoskinson/Moriarty/pull/74) and [PR #75](https://github.com/CharlesHoskinson/Moriarty/pull/75) are merged as reviewed source-only integration, at merge commits `7a25a75e6074ac6c9728864dd9b874bd00ef564e` and `baf61ebd0cb49f2c4441cb7fa8de53b343e056dc`. Tier 1 source changes include shared dependencies and diagnostics, source-list coverage and Rust guidance; the quality workflow is manual only. The original author's macOS checks describe the earlier candidate and do not qualify this integrated source. Later bounded local Core/CLI results are recorded separately below; full default/native/workspace checks and compiler, SDK, proof and ledger validation for this cleanup remain UNRUN.

Each issue lists its problem, a suggested fix, what still **needs input**, and when it's done. Issues whose *Needs input* section names a decision need maintainer agreement before an agent starts on them.

## Tier 4: decision first
- [ ] #76 Decide how the Core language relates to the beta frontend (blocks #82 and #83)

## Tier 3: structural (agree the approach first)
- [ ] #77 Derive build and ABI identities from artifacts, not source text
- [ ] #78 One Program-to-ZKIR lowering path for mori-backend and mori-ledger
- [ ] #79 Move Sherlock into its own crate and build it only through Cargo
- [ ] #80 Move business logic out of the mori CLI (blocks most of #85)
- [ ] #81 Put the Midnight ledger dependencies behind a feature
- [ ] #82 Route Core, backend, ledger and CLI errors through MoriDiagnostic
- [ ] #83 Remove duplicated logic inside mori-core

## Tier 2: test cleanup (mostly no decisions needed)
- [ ] #84 Move test fixtures into crates/fixtures
- [ ] #85 Replace mori binary tests with library tests
- [ ] #86 Load Sherlock oracles from JSON instead of generated Rust
- [ ] #87 Snapshot and fuzz the Core and Sherlock frontends
- [ ] #88 Remove env-var-driven test behavior and the task4 driver from proving paths

## Suggested order
1. Tier 2 issues that need no input: #84, #86 (proptests in #87), #88.
2. Decide #76, then #77 and #80.
3. The rest of tier 3, then #85 and the error snapshots in #87.

Every change follows `RUST_GUIDELINES.md` and `plugins/moriarty-dev/skills/develop/references/rust.md`. Before handoff, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace` pass.

## UTXO workstream adoption

Tracked as `GH-I89` in [doctormoriarty<utxo>](https://github.com/CharlesHoskinson/Moriarty/blob/utxo/docs/utxo/doctorwatson/github/work-items.json) and its [EARS/consumer assessment](https://github.com/CharlesHoskinson/Moriarty/blob/utxo/docs/utxo/doctorwatson/github/issue-triage.md).

Shared requirement lineage: UNI-018, UNI-019, UNI-020, UNI-021, UNI-023, UNI-024. Retained delivery exits: U0, U1, U2, U3, U4, U6, U7, MC01, MC03, MC04, MC05, MC07, MC08, SP02, SP03, SP04, SP05, SP06, SP08, SP09, SP11, SP12. Cardano candidate relevance: shared-language/release dependency; no automatic Cardano scope transfer. This is a traceability mapping, not a claim that the issue or any parent requirement is complete.

Source preparation can proceed within the current user authorization. Concrete API/identity proposals and applicable independent reviews remain required where semantics change. Original suggested fixes and historical tool counts are proposals and observations, not automatic migration or test oracles. Keep public developer use permissionless. Run checks locally; automatic hosted CI is outside the current workflow. Existing native/proving/ledger limits remain in force.

All thirteen children #76–#88 remain open. PR #74 and PR #75 have already merged; there is no remaining pre-merge repair of those PRs in this tracker. Their source-only integration and later selected tests do not close the original child predicates or establish full Rust, Cardano or Midnight acceptance.

## Dated child evidence — 2026-10-18

- [#191](https://github.com/CharlesHoskinson/Moriarty/pull/191) merged the independently frozen publication transport fixture as source only. Its Rust publication tests remain UNRUN; the fixture is opaque transport data, not compiled ZKIR or a proof.
- [#193](https://github.com/CharlesHoskinson/Moriarty/pull/193) and [#194](https://github.com/CharlesHoskinson/Moriarty/pull/194) supply Core dependency isolation and a finite financial-pair test child. Their combined selected run passed five existing continuation tests and two financial tests.
- [#197](https://github.com/CharlesHoskinson/Moriarty/pull/197) extracted the consumed Core/2 report renderer and passed two selected Core report tests. This is partial #80/#84/#85/#86-related work, not completion of those issues.
- [#199](https://github.com/CharlesHoskinson/Moriarty/pull/199) merged default-enabled `native` and the SDK-free CLI configuration, with twelve selected local CLI integration tests. [#200](https://github.com/CharlesHoskinson/Moriarty/pull/200) added two non-native boundary tests covering 27 cases and 44 CLI invocations. These are bounded #81-related observations, not an SDK-free whole-workspace or full configuration pass.
- [#202](https://github.com/CharlesHoskinson/Moriarty/pull/202) passed two public normal-report tests covering five CLI calls on its exact tree. [#203](https://github.com/CharlesHoskinson/Moriarty/pull/203) passed two complete finite Program parser/serialization tests. [#204](https://github.com/CharlesHoskinson/Moriarty/pull/204) adopted the additive financial observation contract as source only. These retain separate report, Program and design scopes.
- At the captured #205 draft head `7e0d753c40a3e5f694aa226bd657c831da111f65`, the selected financial-report target passed two tests in 52.624672563 seconds on tree `55fab2c3cbd818ffb04eeb7d5930a28758a858b3`. The local result review remains pending at this body proposal; the passing run is not result approval, merge approval or parent acceptance. Its actual-result receipts remain separate from #204's source-only adoption.

These counts describe different exact candidates and are not added into a current whole-suite result. Historical captures, failures, freezes and review dispositions remain preserved. All thirteen original requests and their completion conditions remain in force; shared language/DevEx evidence does not transfer to Cardano scripts, native proofs or ledger acceptance.


