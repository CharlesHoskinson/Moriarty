# Current external-install and starter command documentation

Exact base: `e4d7a2b5709caa18a94f5256d03a0302364b8235`.
Status: **unapplied two-document candidate**. Root is the sole integrator. Only this new packet directory was written.

The earlier beta launcher correction remains applicable without a source merge: all thirteen source members from the earlier `74a909d69639df8823a527a4d608a7516ccd69a2` packet are byte-identical at this base. The broader, explicitly selected 33-file set captured here is also byte-identical between those commits. `SELECTED-SOURCE-EQUIVALENCE.json` gives both SHA-256 values per file. This is finite byte equivalence only: neither whole-tree equality nor behavioral/build/install/product acceptance follows.

`packages/moriarty-beta/GETTING-STARTED.md` retains the earlier proposed correction exactly. Local archive installation maps the `mori` bin to bundled `dist/cli.js`, but the generated starter README invokes bare `mori` while the package guide supplies only `npx mori` in the install project. The candidate defines a bash/zsh launcher using the absolute installed beta path before entering the generated project. It therefore follows the actual artifact even when a different `mori` is on PATH. The site's existing archive how-to already uses this pattern. Its shared documentation links remain pinned to the older `df8b155` snapshot; this candidate does not change that historical site pin or claim live publication.

The complete selected Rust chain reveals a separate stale instruction in `docs/NATIVE-CORE.md`: it says to replace generated bare `mori` commands, although current `init` already writes the POSIX-quoted absolute `current_exe` path. The full applicable patch corrects this and provides the explicit local starter command `cargo run -p mori --locked --no-default-features --bin mori -- init NEW_DIR`. The Rust package is `mori`, its ordinary binary entrypoint is `src/main.rs`, and `native` is the default feature. The generated Core starter does not need that feature. The distinct `source6-expansion` opt-in controls `expand --emit-source6-transfer`; it is neither silently selected nor required by this starter.

The Rust transfer and repayment templates both generate `source.mori`, `scenario.json`, `mori-tests.json` and `README.md`, with actions `pay` and `repay`. The beta templates instead generate `invoice.mori`/`pay` or `repayment.mori`/`repay_loan`, and `mori.tests.json`. The patch preserves these command and manifest distinctions. `COMMAND-CHAIN.json` records actual init/check/test/simulate dispatch consumers. The financial expectations were read as source literals, not recomputed or claimed executed.

Bootstrap and dependency installation, artifact building, running local examples, native proof production, deployment and observed settlement remain separate. The proposed Cargo command selects features; it is not an observed build. It promises no offline dependency availability or successful fresh installation. Beta local preparation stays **PreparedUnqualified**; Rust ordinary simulation stays local reference execution. Neither is a native or target result. The contract's independent external developer requirement still needs a new supported program, inspected claims, real proofs and submission in the developer's own Midnight environment without project metadata/reviewer records. No part of this source-only correction closes that exit.

The installed develop skill and exact-base AGENTS were read. Root supplied its startup status for this turn: historical SP01 blockers remain preserved. That operational report is inherited only and establishes no source acceptance. Root instructed this lane not to rerun status because the actual status consumer writes an administrative interval to the shared store. No campaign or live work was selected.

Pinned source locations:

- `packages/moriarty-beta/GETTING-STARTED.md:29`.
- `packages/moriarty-beta/package.json:10`.
- `packages/moriarty-beta/src/cli.ts:106`.
- `crates/mori/Cargo.toml:12`.
- `crates/mori/src/main.rs:16`.
- `crates/mori/src/commands/init.rs:106`.
- `crates/mori/src/commands/test.rs:42`.
- `crates/mori/src/commands/simulate.rs:136`.
- `docs/NATIVE-CORE.md:29`.
- `docs/MORIARTY-PRODUCT-CONTRACT.md:74`.
- `plugins/moriarty-dev/scripts/moriarty_dev/cli.py:322`.

Reviewable artifacts are `EXTERNAL-INSTALL-DOCS.patch`, the two complete documents under `candidate/`, exact public blobs under `source/`, `SOURCE-MANIFEST.json`, `SELECTED-SOURCE-EQUIVALENCE.json`, `COMMAND-CHAIN.json` and `VALIDATION.json`. Validation records nine direct source comparisons, not a runtime suite. No contrived regression test was added for these reversible documentation corrections.

No installer/npm/Cargo/build, program runtime, network, provider, native/SDK/proof/deployment/ledger command, global-tool change, shared-worktree write or Git mutation was performed. No security judgment is supplied. The candidate is ready for root's applicable review and integration; fresh installation and execution remain unrun.
