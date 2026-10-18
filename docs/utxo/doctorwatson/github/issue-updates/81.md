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

