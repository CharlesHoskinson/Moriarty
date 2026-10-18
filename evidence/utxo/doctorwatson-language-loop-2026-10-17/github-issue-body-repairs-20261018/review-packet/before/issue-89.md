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

