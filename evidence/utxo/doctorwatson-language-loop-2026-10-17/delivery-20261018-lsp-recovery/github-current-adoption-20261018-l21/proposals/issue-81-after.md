**Tier 3 — structural.**

## Problem
The following is the original problem report, retained as historical context rather than a current dependency-graph claim:

`RUST_GUIDELINES.md` says Git-pinned Midnight dependencies stay behind a feature. Today `mori` depends on `mori-backend` and `mori-ledger` unconditionally. Together they pull ten Git crates from midnight-ledger plus `midnight-curves` and `midnight-proofs`, and `Cargo.lock` grew from 136 to about 450 packages. Building only the beta `check` and `format` compiles all of it. The root `[patch.crates-io]` also pulls a second midnight-ledger revision (`a01a1ea`) as `midnight-transient-crypto-old`.

**Current source and selected evidence (2026-10-18).** [PR #193](https://github.com/CharlesHoskinson/Moriarty/pull/193) merged the opt-in, nondefault `midnight-compat-tests` dependency boundary for Core. [PR #199](https://github.com/CharlesHoskinson/Moriarty/pull/199) merged the CLI's default-enabled `native` feature and explicit `--no-default-features` configuration; the backend, ledger and native dependencies are optional in that configuration. The command schema is preserved, and native-required routes return a typed capability refusal. [PR #200](https://github.com/CharlesHoskinson/Moriarty/pull/200) merged its selected non-native boundary tests. Thus the original unconditional-dependency premise and unanswered default-feature question below describe the earlier tree; the implemented default is `native` enabled. This does not resolve every item in this issue.

The captured PR records report only bounded local results on their exact trees: #193/#194 together passed five existing continuation and two financial tests; #199 passed twelve selected CLI integration tests; #200 passed two tests covering 27 cases and 44 CLI invocations. The selected #199 build log contains no Midnight/backend/ledger compilation. These are separate observations, not one combined current-tree suite pass or an SDK-free whole-workspace claim. Full default/native/workspace checks for this feature/configuration cleanup remain UNRUN. General compiler/SDK qualification and cryptographic proof/ledger acceptance remain open; the later finite compiler/SDK observation is qualified separately below. The native compatibility test, complete beta check/format installation/configuration matrix and the original `k256` question still require explicit reconciliation.

A separate earlier compiler/IR baseline at exact source `7e0d753c40a3e5f694aa226bd657c831da111f65`, tree `55fab2c3cbd818ffb04eeb7d5930a28758a858b3`, passed one selected legacy Counter/Payment test (five filtered) in 169.097748 seconds, with its scoped actual-result reviews adopted. It covers the historical Counter role/name ABI, Payment first effect frame and unsigned compiler/SDK IR-loader roundtrip for two programs and three circuits. It is not an independently complete Artifact/IR, financial, VM, proving or ledger result.

A later separate scoped compiler/SDK observation is now available in [PR #210](https://github.com/CharlesHoskinson/Moriarty/pull/210): two positive Core/1 financial tests passed in 149.424869 seconds at source `4f83f0f613464343efd7a6322efacd8f87fdbc1d`, tree `2d4bddedb32182737e5536996a411317d44d8763`. They compile four unsigned actions, reopen emitted SDK IR and check three positive transcripts with `ProofPreimage::check`. This is an observed finite compiler/interpreter result, not a native cryptographic proof or general compiler/SDK qualification. Independent full Artifact/IR byte goldens, register transcripts, negative-witness rejection and general grammar/field coverage remain open. No SRS loading, key generation, cryptographic proving or ledger validation is established by that result.

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

