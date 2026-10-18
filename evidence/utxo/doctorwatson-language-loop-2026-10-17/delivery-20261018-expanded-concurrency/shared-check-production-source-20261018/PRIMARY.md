# Shared source-check production prerequisite

Complete UNAPPLIED ordinary source candidate against exact main `e4d7a2b5709caa18a94f5256d03a0302364b8235`, tree `b44043fba64fa2bd396d18412adb3472434c70ac`. GPT-6.1 Sol authored this source lane. The parent integrator owns shared application, execution and review. Startup status is inherited from the current root operational snapshot under the explicit delegation; no campaign, scheduler or status process was invoked here. Installed `moriarty-dev:develop`, pinned AGENTS, product contract, Rust guidelines and Rust checklist were applied. No SDK or dependency decision is adopted.

The falsifiable predicate is that the real public `mori check FILE --json` consumer routes Beta, Core1 and Core2 through one shared source dispatcher while retaining original complete stdout bytes, exits and diagnostics/spans for the frozen eight normal cases. Human mode must continue to avoid constructing owned JSON reports. Candidate execution remains unobserved by this author.

## Physical delivery and consumer

`candidate.patch` contains 21 paths: two modified production files, one new production module, one new real binary regression target and 17 fixture/manifest/golden files. SHA-256: `ac54860302c6c94f1cba92cfdbec243eb1f0eac1bc9b69d3222625b3ef161583`.

- `after/crates/mori/src/commands/check.rs`: the actual public CLI now calls `check_source::dispatch`, then prints the Beta owned report or human completion only after the borrowed AST/check data has dropped. Existing file loading and its bound are unchanged. Diagnostic rendering helper `finish` is unchanged.
- `after/crates/mori/src/commands/check_source.rs`: owns profile dispatch, real parser/checker calls and the existing report constructors. Beta passes `Option<&mori_checker::Checked<'_>>` plus real diagnostics to a synchronous typed consumer while the AST lives. Core passes actual `Result<mori_core::Program, mori_core::Error>`. No mirrored AST or substitute checker is introduced.
- `after/crates/mori/src/commands/mod.rs`: registers the shared module.
- `after/crates/mori/tests/check_source_dispatch.rs`: exercises the actual Cargo-built public binary with ordinary source files, complete frozen stdout, exits and JSON stderr. Each case also checks quiet human stdout/exit and diagnostic presence. These are real consumer tests, not source-marker assertions.

The public call path is `mori check FILE --json` → existing command dispatch → `commands::check::run` → `commands::check_source::dispatch` → Beta parser/checker, Core1 parse/validation, or Core2 stage loader → CLI consumer → original core JSON exporter → stdout. Existing command dispatch and exporter are pinned before-context, not replaced. There is no callable MCP tool in this increment. A future official SDK MCP handler can implement the same synchronous typed consumer and request the shared report constructors, after its separate design/dependency/resource requirements are met.

## Preservation rationale from source

Beta still passes the original `CheckReport` struct directly to `core_io::print`; it is never converted into a JSON Value before printing. Its declared field order, source hash, names, partial declarations/actions, diagnostics and UTF-8 byte spans therefore retain their existing serialization implementation. The JSON report constructor and file-name ownership conversion run only inside `self.json.then(...)`. The dispatcher itself never constructs an owned report. Human mode retains only its existing parser/checker results and diagnostic rendering allocations. This is source inspection, not an allocation measurement.

The `Parsed` consumer hook runs after successful Beta parsing and before calling the checker, preserving the human verbose ordering. The owned Beta outcome returns through the dispatcher before final printing/completion, preserving the previous AST/check-data release boundary. Human elapsed-time text remains inherently variable and is not given a fixed-byte oracle.

Core1 uses the same `language` token detection and `mori_core::parse`. Core2 uses the same exact `moriarty/2` detection and `stage::load(...).map(|program| program.program().clone())`. Core report construction retains the current Value representation, accepted fields, rejected error shape and hash policy. The same `STAGE_LABELS` are inserted on both accepted and rejected Core2 objects before the same JSON exporter. These labels remain `qualification=local-reference-stage`, `authentication=none`, `consumption=untracked`; they do not become stronger qualification claims. The original stage printer remains unchanged for its other consumers.

No Cargo manifest, lockfile, feature, editor, public protocol, supported profile, runtime limit or public developer admission rule changes. Maintainer review requirements remain internal. Public tooling stays permissionless.

## Eight normal regression cases

| Case | Predicate covered | Exit |
|---|---|---:|
| accepted | Beta accepted declarations, action/support and complete report order | 0 |
| rejected | Beta checked partial results and complete diagnostic payload | 1 |
| unparsable | Beta parse error, absent checked program and diagnostic span | 1 |
| core1-accepted | Core1 actual program/action report and source hash | 0 |
| core1-rejected | Core1 existing validation error shape | 1 |
| core2-accepted | Stage loader and all three public qualification labels | 0 |
| core2-rejected | Stage rejection and all three public qualification labels | 1 |
| unicode-byte-offsets | Beta diagnostics after a multibyte source prefix | 1 |

All eight source bodies are unchanged from the earlier frozen source candidate. `INPUT-ONLY.json` preserves an initial proposal with relocated labels; it was not the baseline invocation. `REVISED-INPUT-BINDING.json` and production fixture `cases.json` explicitly bind the actual tests to the parent's original `test.mori`, `minimal.mori` and `empty.mori` labels. Tests copy unchanged source bytes to those labels in a new isolated per-case temporary cwd. They do not transform baseline JSON or its file fields. The temporary-directory and child-process operations exist only in regression source; this author did not execute them.

The root independently built the pre-refactor ordinary binary at `97c9b1565d71404413aa35e9f95fe7661b8a09b1` and ran all eight JSON cases before this candidate. `../shared-check-production-check-cycle/BASELINE-RESULT.json` records exit 0 for the cold scoped build, 41.284 seconds, all eight specified DTO comparisons and all three historical Beta raw-byte comparisons passing. This is attributed parent evidence, not an execution performed by this author. The source-check production path is unchanged from the pinned E4D baseline according to the parent source pins. Root-owned temporary build resources were not accessed by this author.

The eight `expected/*.stdout` files are exact copies of that baseline's complete stdout, with provenance and SHA-256 in `BASELINE-GOLDEN-PROVENANCE.json`. They are compatibility oracles from the original implementation, not independent semantic or financial expectations. Existing earlier specified complete DTOs also equal all eight captured reports under artifact-only JSON comparison. No evaluator, oracle/provider, financial execution, proof or ledger result is implied.

## Evidence and remaining work

`before/` retains exact Git source bodies and `BASE-SOURCE-PINS.json` their hashes; `after/` contains all complete candidate changed/new bodies. `CHANGE-MANIFEST.json` pins every path. `ARTIFACT-VERIFICATION.json` records exact in-memory reconstruction of every patch path against pinned before bytes, unchanged eight inputs/original labels/exits, full captured DTO equality, and syntax-only parsing of four Rust files. All 21 reconstructed files match exactly. Tree-sitter reports no syntax errors. It establishes no Rust type correctness or runtime result.

This author ran no Cargo, rustc, rustfmt, clippy, product binary, SDK, native, provider, network, Node, Git mutation or shared source write. Rust build/type checks, new integration execution, changed-CLI byte comparison, formatting and clippy remain UNRUN here. The parent has separate ordinary build/test authority and will qualify its actual post-application observations; that evidence must remain distinct from this source packet. Reviewers should check the whole candidate, preserved report/diagnostic behavior, Rust guidelines and normal fixture regression.

This is an interim production prerequisite only. The full prior 35-path official-SDK MCP candidate remains separate and unapplied/unadopted. Its two substantive ordinary design votes, designated dependency/build-script/resource judgments, independent result review, lockfile resolution and full first-delivery requirements remain outstanding. No reduced MCP delivery or roadmap completion is claimed. No security authoring, hostile tests or new security judgments are supplied by this lane.
