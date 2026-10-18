# GitHub issue adoption into doctormoriarty<utxo>

Public GitHub snapshots retrieved 2026-10-18T03:44:52.204025+00:00. Repository: `CharlesHoskinson/Moriarty`; source tree: `1721457439dbd6d988d00c6c86d1cf648caf2a0c`. Display alias: `doctormoriarty<utxo>`; stable runner and evidence namespace: `doctorwatson-utxo`.

All 15 open issue bodies are preserved verbatim in [issues/](issues/); [issue-triage.json](issue-triage.json) retains exact public fields, all original descriptions, PR74/75 bodies, scenario locators, consumers, independent checks and four separate dependency axes. `GH-I…` IDs are operational work-item keys, not new EARS requirements. The original **119 / 11 / 18** requirement inventories and all **U0–U7 / M1–M17 / MC01–08 / SP01–12 / G01–24** exits remain unchanged. No product completion percentage is inferred.

This is source/API triage only. Native authority remains zero and the broad Preprod goal remains paused. No remote comment, message, edit, close or Git mutation occurred. No product, Cargo, SDK, VM, proof, wallet, node or network operation outside structured GitHub reads ran. Security interpretation and new security tests/judgments remain Opus-owned under current AGENTS.

| Issue / operational ID | Existing obligations and scope | Missing predicate and real consumer | Disposition and safe cleanup |
|---|---|---|---|
| [#23](https://github.com/CharlesHoskinson/Moriarty/issues/23) `GH-I23` | shared authoring/finance DevEx; Midnight product and release goals retained; Cardano adaptation is additional scope. UNI-001, UNI-004, UNI-005, UNI-014, UNI-016, UNI-018, UNI-020, UNI-023, UNI-024, U0, U2, U3, U6, U7, M1, M10, M11, M15, MC01, MC07, MC08, SP02, SP07, SP08, SP11, SP12, G01, G14, G17, G18, G19, G20, G21, G22, G23, G24. | Preserve and implement all six finance-facing gaps: paid/owed and exact rate/date/rounding vocabulary; chained scenarios with dated decimal cash-flow/balance tables and CSV; invariant/trust status; complete deterministic agreement rendering; real checked finance templates; observed study with 3–5 nonprogrammer finance professionals. Each retains its own validation; no source-profile collision or shortcut around U1/U2.<br>Consumer: packages/moriarty-beta frontend, CLI simulation/test/intent display and starter templates; Rust mori check/test/inspect; eventual same public compiler and U6 libraries. | Adopt as six retained subitems; eligible public-output/wording preparation plus specific grammar/library/study dependencies.<br>Keep issue open; prepare a dated progress crosswalk after reviewed implementation, retaining every original gap and restriction. Do not narrow Midnight vision to Cardano or a fixed loan/swap product. |
| [#76](https://github.com/CharlesHoskinson/Moriarty/issues/76) `GH-I76` | shared compiler/frontend and DevEx; separate Core/beta/Sherlock profiles. UNI-002, UNI-018, UNI-024, U0, U2, U7, M1, M6, M10, MC01, MC04, MC08, SP02, SP03, SP12, G01, G03, G14. | Publish a concrete profile/consumer compatibility decision with reasons: shared beta frontend profile vs separate Core crates, Core check/init support, current spaced-operator acceptance and Sherlock draft scope. Record follow-up work without implying implementation.<br>Consumer: crates/mori/src/commands/check.rs and init.rs; mori-core parser/IR/validate; backend and ledger Program consumers; Sherlock draft/source adapters. | Concrete decision proposal needed; source inventory and compatibility examples eligible now. #82/#83 dependent portions await outcome.<br>Keep open until documented decision and explicit follow-ups exist; no blanket maintainer permission for public developers. |
| [#77](https://github.com/CharlesHoskinson/Moriarty/issues/77) `GH-I77` | shared identity/ABI infrastructure with native Midnight trust-boundary implications. UNI-003, UNI-015, UNI-018, UNI-021, U0, U2, U7, M1, M2, M6, MC04, MC08, SP05, SP09, SP12, G03, G06, G12, G16. | Replace source-text identities with a reviewed, defined artifact/template/toolchain identity contract and explicit saved-manifest migration; unrelated formatting changes leave semantic identities stable while material bound artifacts remain identified.<br>Consumer: mori-ledger layout/static_compile/local_deployment/signed deployment/context/preparation manifest consumers. | Eligible source inventory/proposal; identity contract and migration require concrete reviewed design, native result blocked by current authority.<br>Keep open; do not delete source-closure tests or regenerate evidence blindly. Retain old receipts and enumerate invalidated manifests. |
| [#78](https://github.com/CharlesHoskinson/Moriarty/issues/78) `GH-I78` | shared source IR comparison; existing backend and ledger outputs are distinct Midnight statements, not a Cardano lowering. UNI-003, UNI-018, UNI-019, UNI-020, UNI-017, U0, U1, U2, U4, M1, M2, M6, M8, MC01, MC04, MC05, SP03, SP04, SP05, SP09, G05, G09, G16. | Inventory exact statements, admission subsets and numeric/field representations; share only justified common gadgets/parameter-loader mechanics while preserving each consumer contract. A proposal is required before merging lowerings or loaders.<br>Consumer: mori-backend compile/compile_signed and stage lowering; mori-ledger relation/compiler, arithmetic, layout and ops; OfflineParameters/PinnedCallParameters. | Eligible comparative inventory and bounded proposal; merging semantically different statements is not presumed safe.<br>Keep open; retain title/body and add a future clarification that within-consumer artifact preservation differs from cross-consumer byte equality. |
| [#79](https://github.com/CharlesHoskinson/Moriarty/issues/79) `GH-I79` | shared Sherlock interpreter/frontend packaging; not native qualification. UNI-002, UNI-018, UNI-020, UNI-021, UNI-024, U0, U2, U7, M1, M6, M10, MC01, MC04, MC08, SP03, SP12, G04, G05, G12. | Choose retained prototype ownership, move adopted Sherlock library/frontend to Cargo crate, remove direct-rustc and path remounts, preserve exact interpreter/prefix results and source-projection consumption, and retain historical source pins.<br>Consumer: crates/mori-core/src/lib.rs Sherlock exports; mori-backend source_projection; models/sherlock/reference/check-reference.py and independent JSON oracles. | Eligible crate/prototype/consumer inventory; packaging proposal needed for adoption split.<br>Keep open; retain historical 35-file claim as dated description, attach updated manifest after reviewed migration. |
| [#80](https://github.com/CharlesHoskinson/Moriarty/issues/80) `GH-I80` | shared public CLI and library architecture; native call protocol remains separate controlled consumer. UNI-001, UNI-018, UNI-021, UNI-024, U0, U2, U7, M1, M10, MC01, MC04, MC08, SP02, SP05, SP12, G01, G14. | Extract inspect/test/init and typed call-protocol logic into consumed libraries; keep CLI as parse/read/call/print; specify public command aliases and JSON/review/stdout/stderr compatibility before changing surface.<br>Consumer: crates/mori/src/commands/{inspect,test,init,call_io,core_io,verify_call,prove_call}; public mori CLI and scripts. | Eligible library-boundary extraction proposal and caller/report inventory; native protocol design and command changes need concrete compatibility proposal.<br>Keep open; split independent report/test extraction from native protocol execution in local work records, preserving original issue scope. |
| [#81](https://github.com/CharlesHoskinson/Moriarty/issues/81) `GH-I81` | shared frontend installation/build DevEx; separates Midnight native dependencies without changing validity. UNI-001, UNI-018, UNI-024, U0, U2, U7, M1, M10, MC01, MC08, SP02, SP12, G12, G14, G16. | Define default feature policy and gate actual native consumers so no-default-features beta build excludes Midnight crates; preserve native feature behavior and document unavailable commands accurately.<br>Consumer: crates/mori Cargo dependency graph, command modules and public beta check/format installation. | Eligible dependency/source graph inventory; default behavior proposal needed, local configuration checks then required.<br>Keep open; preserve original CI request as captured evidence and reconcile planned verification to local-only CI. |
| [#82](https://github.com/CharlesHoskinson/Moriarty/issues/82) `GH-I82` | shared diagnostic/DevEx API; retain existing ledger refusal semantics. UNI-018, UNI-024, U0, U2, U7, M1, M10, MC01, MC08, SP02, SP12, G01, G14. | Route source errors through real MoriDiagnostic spans/labels/help, preserve non-source cause chains and typed code contracts, and prevent CLI mori::command wrapping from erasing library diagnostics.<br>Consumer: Core parser/checker, backend/ledger errors, core_io operation renderer and public check/compile/test output. | Partially dependent on #76; code/span/caller inventory eligible now.<br>Keep open; retain original refusal spellings until compatibility proposal and applicable review exist. |
| [#83](https://github.com/CharlesHoskinson/Moriarty/issues/83) `GH-I83` | shared Core semantics and public API cleanup. UNI-002, UNI-005, UNI-018, UNI-020, U0, U2, U3, M1, M6, M13, MC01, MC04, MC05, SP03, SP08, G01, G03, G05, G14. | Deduplicate profile-aware validation and liability/asset identity logic, token-copy/constants/keywords and error-kind comparisons; split runtime responsibilities without changing complete behavior; explicitly decide planned authority API disposition.<br>Consumer: Core/1 runtime validate/simulate/replay and Core/2 stage validate/decode/plan/admit consumers. | Eligible focused inventory/constants preparation; frontend rewrite depends on #76, public API removal needs concrete proposal.<br>Keep open; update future progress per duplicated item rather than closing on a module split alone. |
| [#84](https://github.com/CharlesHoskinson/Moriarty/issues/84) `GH-I84` | shared deterministic fixtures and source-test portability. UNI-020, UNI-021, UNI-023, U2, U6, U7, M6, M10, M15, MC04, MC07, MC08, SP03, SP11, SP12, G04, G05, G12. | Move every cross-boundary test dependency to crates/fixtures or explicit dev helper, retain original bytes/provenance and enforce no relative read beyond crates; define example/fixture synchronization ownership.<br>Consumer: Core/backend/ledger/CLI test include_str and cross-crate support mounts, independent oracle/vector fixtures. | Eligible immediate source/fixture preparation with exact byte preservation.<br>Keep open until consumers and enforcement check are implemented/reviewed; no need to await all structural issues. |
| [#85](https://github.com/CharlesHoskinson/Moriarty/issues/85) `GH-I85` | shared library tests and public consumer coverage. UNI-020, UNI-021, UNI-024, U2, U7, M6, M10, MC04, MC08, SP03, SP12, G05, G12, G14. | Migrate each CLI subprocess/Python test to the actual consumed library behavior or publishing helper without losing report/exit/I/O coverage; remove help assertions after equivalent relevant contract coverage exists.<br>Consumer: crates/mori/tests and core_io capture tests; library compile/replay/metered-intent and publishing helpers. | Partly eligible now; specific test migrations wait on corresponding #80 extraction.<br>Keep open; inventory retained coverage before deleting test files. Treat overlap with #80 as dependency, not duplicate issue. |
| [#86](https://github.com/CharlesHoskinson/Moriarty/issues/86) `GH-I86` | shared Sherlock independent oracle data and reproducibility. UNI-002, UNI-020, UNI-023, U0, U2, U7, M6, M10, MC01, MC04, MC08, SP03, SP11, SP12, G04, G05, G12. | Choose canonical oracle location, deserialize exact independent JSON into snapshot types, remove generated Rust literals and make Cargo-owned validation detect drift without replacing expected data with snapshots.<br>Consumer: models/sherlock/reference/check-reference.py serialize_oracles; Sherlock normal_oracles/extra_normal_oracles and compare/reference tests. | Eligible oracle/schema/digest inventory; canonical-location proposal required before establishing ownership.<br>Keep open; preserve generated historical evidence and canonical JSON provenance when replacing maintained source literals. |
| [#87](https://github.com/CharlesHoskinson/Moriarty/issues/87) `GH-I87` | shared frontend diagnostics/property-test coverage; no native execution implied. UNI-018, UNI-020, UNI-024, U0, U2, U7, M1, M6, M10, MC01, MC04, MC08, SP02, SP03, SP12, G01, G05, G14. | Provide source-bearing tree/diagnostic snapshots and never-panic properties for every original listed source consumer, including stage load, JSON and continuation decoding and Sherlock parser/typechecker, with independent success/refusal expectations.<br>Consumer: Core parse/stage::load/json::parse/continuation decoding and Sherlock draft source/typecheck; human diagnostic renderer. | Eligible frontend coverage preparation now; diagnostic snapshots specifically await #82.<br>Keep open; record existing parser property as partial coverage and preserve all original additional phase obligations. |
| [#88](https://github.com/CharlesHoskinson/Moriarty/issues/88) `GH-I88` | shared evidence/test harness cleanup with native proving/resource boundaries retained. UNI-017, UNI-020, UNI-021, UNI-024, U1, U2, U4, U7, M5, M6, M10, MC03, MC04, MC08, SP04, SP06, SP12, G09, G10, G12, G16. | Inventory every env-controlled capture consumer and required audit artifact; relocate task4 measurement harness and replace implicit/global test hooks with explicit owned tools/providers while preserving resource limits, failures and authenticated evidence contract.<br>Consumer: mori-backend proving/parameters/signed_relation task4 hooks, Python driver and named capture variables consumed by docs/audits and local evidence tooling. | Eligible source/audit-consumer inventory; hook removal awaits concrete evidence/resource-preservation proposal; execution blocked only at native boundary.<br>Keep open; preserve immutable historical captures, audits, failures and counters. Never close because harness moved without equivalent evidence delivery. |
| [#89](https://github.com/CharlesHoskinson/Moriarty/issues/89) `GH-I89` | shared Rust cleanup tracker; all 13 children retained and target-separated. UNI-018, UNI-019, UNI-020, UNI-021, UNI-023, UNI-024, U0, U1, U2, U3, U4, U6, U7, M1, M2, M5, M6, M8, M10, M13, M15, MC01, MC03, MC04, MC05, MC07, MC08, SP02, SP03, SP04, SP05, SP06, SP08, SP09, SP11, SP12, G01, G03, G04, G05, G06, G09, G10, G12, G14, G16. | Track all 13 original child predicates #76–88 with exact issue/PR/source/check/review/acceptance state; mark tracker complete only when each child satisfies its original scope. Do not promote operational work items into original product requirement counts.<br>Consumer: doctormoriarty<utxo> report display; stable doctorwatson-utxo runner/report namespace; existing EARS/scenario and roadmap inventories. | Active parent tracker; independent source tasks eligible and exact dependent boundaries named.<br>Keep open, preserve all 13 unchecked children, correct dated PR74 premise only in a separately reviewed future remote edit. No remote edit performed. |

The #89 tracker retains every child: [#76](issues/76.json), [#77](issues/77.json), [#78](issues/78.json), [#79](issues/79.json), [#80](issues/80.json), [#81](issues/81.json), [#82](issues/82.json), [#83](issues/83.json), [#84](issues/84.json), [#85](issues/85.json), [#86](issues/86.json), [#87](issues/87.json), [#88](issues/88.json). #23 retains all six finance-facing gaps, its original exact accounting/signing restrictions, the full Midnight product vision, U6 financial coverage and U7 release obligations. Finance wording/reporting can proceed independently; finance mathematics and human study keep their stated library/tooling dependencies.

Each row below describes proposed checks, not executed tests or a security verdict. Original EARS scenario text stays authoritative; the JSON records exact source scenario headings and lines. Cardano EUC/EU-FIN/EU-CON/etc. links in JSON describe possible reuse only. Shared Core/DevEx work does not establish Cardano scripts, and Midnight ZKIR/native evidence does not establish Cardano Groth16/ledger correspondence.

## #23: Make Moriarty usable by finance professionals building DeFi protocols on Midnight

Owner: Nonsecurity language/DevEx author; independent finance wording owner and human-study recruiter remain to be assigned.

Issue is an October 8 snapshot, not a current executable inventory. Current Rust test/inspect source exists; roadmap retains local command delivery and open native/release qualification. Six gaps are independent precursors, not a claim that all CLI commands are stubs.

Source: [packages/moriarty-beta/src/frontend.ts](../../../../packages/moriarty-beta/src/frontend.ts), [packages/moriarty-beta/GETTING-STARTED.md](../../../../packages/moriarty-beta/GETTING-STARTED.md), [crates/mori/src/commands/test.rs](../../../../crates/mori/src/commands/test.rs), [crates/mori/src/commands/inspect.rs](../../../../crates/mori/src/commands/inspect.rs), [ROADMAP.md](../../../../ROADMAP.md).

Independent acceptance: Independently derive exact atoms/rounding, owed/paid distinctions, ordered stage state/effects, gross debit/fees/net receipts and residual duties; compare each public report and CSV to those frozen values. Finance reviewer checks every bound and duty in deterministic non-normative agreement text; templates emit source accepted by the ordinary declared pipeline. Observe all three original human tasks, time/stalls/understanding; distinguish model trials and beta observations from SP12 pilots.

Refusal/regression: Original requirement: unsupported, timeout and unchecked invariants must never render as held; SpecifiedOnly family templates remain explicitly unqualified. Original display requirement: signed terms/placeholders remain visible and unverified until checked; no generated approval prose or silent profile replacement.

Concrete proposals: Concrete vocabulary extension/version proposal with grammar, compatibility and reserved profile inventory. Template-order proposal, named finance wording owner and study recruiter.

Dependencies: source — Existing beta public-output schema permits scenario tables, reports and agreement rendering preparation now. Finance-math templates require relevant U6 library semantics; vocabulary requires grammar/checker review. Checks — Use independent finance expectations; human study follows assembled tooling. No compile/prove/target commands executed by triage. Review — Fresh independent nonsecurity exact-candidate/result review under current AGENTS routing; security interpretation/design/results remain with Opus 5.5 or 4.8 high where applicable. Acceptance — Public Midnight journey and full U6/U7, MC07/MC08, SP12 and all G exits remain independent; finance study is not an SP12 pilot.

## #76: Rust: decide how the Core language relates to the beta frontend

Owner: Language frontend/IR integrator.

Core ExprId is still a bare u16 alias; separate parse and validate APIs and draft mounts remain. The current contract already requires explicit profile/consumer reporting; choosing Option A is not authorized merely by the issue recommendation.

Source: [crates/mori-core/src/ir.rs](../../../../crates/mori-core/src/ir.rs), [crates/mori-core/src/parser.rs](../../../../crates/mori-core/src/parser.rs), [crates/mori-core/src/lib.rs](../../../../crates/mori-core/src/lib.rs), [crates/mori/src/commands/check.rs](../../../../crates/mori/src/commands/check.rs), [RUST_GUIDELINES.md](../../../../RUST_GUIDELINES.md).

Independent acceptance: Independently enumerate existing accepted/rejected Core/1, Core/2 and beta forms and actual check/init/compiler consumers before selecting the compatibility proposal. Decision text names semantics preserved or changed, migration and follow-up ownership; downstream #82/#83 can cite it.

Refusal/regression: UNI-018 Different authoring profile: beta checking never supplies a Rust compile pass. Original operator acceptance cannot change incidentally during cleanup; conflicting profile strings and unsupported consumer constructs remain explicit.

Concrete proposals: Option A vs B with concrete grammar/IR adapter, migration cost and compatibility matrix. Explicit proposals for check/init routing, spaced operators and Sherlock draft convergence.

Dependencies: source — #74 is context, not merged prerequisite; inspect active files directly. Checks — Source-only inventory and compatibility proposal are eligible now. Review — Fresh independent nonsecurity exact-candidate/result review under current AGENTS routing; security interpretation/design/results remain with Opus 5.5 or 4.8 high where applicable. Acceptance — Decision completes this planning issue only; actual frontend migration, general grammar correspondence and U0/U2/U7 remain open.

## #77: Rust: derive build and ABI identities from artifacts, not source text

Owner: Artifact/ABI integrator; Opus owns identity/trust and migration security judgment.

layout::templates_hash still hashes include_str!("ops.rs"); proving::source_closure still includes source text, and signed consumers use it. This is a live source gap; replacing a hash without defining its claim is insufficient.

Source: [crates/mori-ledger/src/layout.rs](../../../../crates/mori-ledger/src/layout.rs), [crates/mori-ledger/src/proving.rs](../../../../crates/mori-ledger/src/proving.rs), [crates/mori-ledger/src/signed/deployment.rs](../../../../crates/mori-ledger/src/signed/deployment.rs), [crates/mori-ledger/src/signed/context.rs](../../../../crates/mori-ledger/src/signed/context.rs), [crates/mori-ledger/src/signed/preparation.rs](../../../../crates/mori-ledger/src/signed/preparation.rs).

Independent acceptance: Freeze independently specified template/artifact canonical preimages and expected identity vectors; show unrelated source whitespace does not change semantic identity. Enumerate every saved manifest and example binding affected, and two-builder artifact reproducibility obligations separately.

Refusal/regression: Preserve existing stale/different binding refusal semantics; identity-change controls and compatibility acceptance belong to the Opus-reviewed proposal, not a triage security approval.

Concrete proposals: Define what build attests and exact canonical bytes/version; select explicit reject/regenerate vs transitional migration with security-owner review.

Dependencies: source — Inventory all source_closure/templates_hash consumers and historical hash-bound audit records. Checks — Native/key/artifact regeneration and SDK operations not authorized here; source/static vectors can be prepared. Review — Fresh independent nonsecurity exact-candidate/result review under current AGENTS routing; security interpretation/design/results remain with Opus 5.5 or 4.8 high where applicable. Acceptance — Fresh identity/migration security review; all runtime/native/target evidence stays open until separately authorized and observed.

## #78: Rust: one Program-to-ZKIR lowering path for mori-backend and mori-ledger

Owner: Compiler/ABI integrator; Opus owns cryptographic gadget/SRS judgments.

Both source paths remain. Current UNI-019 explicitly requires common-domain comparison and intentional refusal recording; the issue title does not establish global equivalence or identical cross-consumer bytes.

Source: [crates/mori-backend/src/lib.rs](../../../../crates/mori-backend/src/lib.rs), [crates/mori-backend/src/stage/mod.rs](../../../../crates/mori-backend/src/stage/mod.rs), [crates/mori-backend/src/parameters.rs](../../../../crates/mori-backend/src/parameters.rs), [crates/mori-ledger/src/relation/compiler.rs](../../../../crates/mori-ledger/src/relation/compiler.rs), [crates/mori-ledger/src/relation/arithmetic.rs](../../../../crates/mori-ledger/src/relation/arithmetic.rs), [crates/mori-ledger/src/proving.rs](../../../../crates/mori-ledger/src/proving.rs), [openspec/changes/consolidated-language-kernel/reconciliation-2026-10-14.md](../../../../openspec/changes/consolidated-language-kernel/reconciliation-2026-10-14.md).

Independent acceptance: Freeze complete expected observations independently on an explicitly mapped common subset; retain each existing artifact unchanged within its own statement/profile. Inventory unique gadget consumers and loader caps/pins before a shared-module/crate proposal.

Refusal/regression: UNI-019 Different admitted statements: preserve intentional refusal and reason; refuse an unproved global-equivalence claim. SRS/cap/pin refusal requirements retain original scope and Opus review; no parameter loading performed.

Concrete proposals: Separate products vs shared subset, module/crate ownership and loader contract with per-consumer cap/pin behavior.

Dependencies: source — Actual consumer inspection and explicit admission/representation map precede refactor. Checks — Existing artifact tests are regressions, not an independent semantic oracle; broad compile/SDK/native suites remain unauthorized. Review — Fresh independent nonsecurity exact-candidate/result review under current AGENTS routing; security interpretation/design/results remain with Opus 5.5 or 4.8 high where applicable. Acceptance — Source sharing alone closes no native correspondence, U1/U2/U4 or SRS security claim.

## #79: Rust: move Sherlock into its own crate and build it only through Cargo

Owner: Sherlock crate and source-projection integrator.

Core lib retains draft/source #[path] mounts; check-reference.py requires --rustc and directly invokes rustc --test. New Sherlock layout files exist, so the issue count of 35 files is historical, not a current inventory.

Source: [crates/mori-core/src/lib.rs](../../../../crates/mori-core/src/lib.rs), [crates/mori-core/src/sherlock/mod.rs](../../../../crates/mori-core/src/sherlock/mod.rs), [crates/mori-backend/src/source_projection.rs](../../../../crates/mori-backend/src/source_projection.rs), [models/sherlock/reference/check-reference.py](../../../../models/sherlock/reference/check-reference.py).

Independent acceptance: Freeze current adopted module/public-export and consumer inventory, independent whole-state and prefix oracle membership before moving files. Cargo-owned equivalent checks must retain JSON transcription, state/effect ordering and stack semantics; stack push/pop change preserves observed order.

Refusal/regression: Missing prototype/module/prefix coverage cannot count as preserved behavior; generated or shared evaluator output alone is not an independent oracle.

Concrete proposals: Concrete module retention/experiments/feature split; new audit records for changed tree while historical records remain immutable.

Dependencies: source — Coordinate #77 source identity consumers and #86 oracle location; current module inventory precedes migration. Checks — Inspect chosen Cargo targets for native side effects before any future ordinary check; no Cargo suite run here. Review — Fresh independent nonsecurity exact-candidate/result review under current AGENTS routing; security interpretation/design/results remain with Opus 5.5 or 4.8 high where applicable. Acceptance — Packaging/test delivery is a child result, not interpreter formal correspondence or U/M/MC/SP/G closure.

## #80: Rust: move business logic out of the mori CLI

Owner: CLI/library interface integrator; native protocol interpretation to Opus.

inspect.rs still walks Program/Expr and implements enforcement_report; test.rs still owns manifest and runner. PR75 changes this runner but is open/source-only, so do not overwrite or infer it is integrated.

Source: [crates/mori/src/commands/inspect.rs](../../../../crates/mori/src/commands/inspect.rs), [crates/mori/src/commands/test.rs](../../../../crates/mori/src/commands/test.rs), [crates/mori/src/commands/init.rs](../../../../crates/mori/src/commands/init.rs), [crates/mori/src/commands/call_io.rs](../../../../crates/mori/src/commands/call_io.rs), [crates/mori/src/commands/core_io.rs](../../../../crates/mori/src/commands/core_io.rs), [crates/mori/src/cli.rs](../../../../crates/mori/src/cli.rs).

Independent acceptance: Freeze existing public report shapes, ordered complete cases, exit results and file-publishing behavior independently; demonstrate production commands consume extracted functions. Concrete command/alias plan preserves existing script callers; JSON output has only its declared JSON payload and review/status output follows documented channels.

Refusal/regression: UNI-024 Complete aggregate CLI test report: mismatches keep complete actual reports and exit 1; successful smaller report bytes/newline remain preserved. No library error may be silently flattened or unsupported native path relabeled successful during extraction.

Concrete proposals: Command nesting/aliases and owning crate for call protocol; exact stdout/stderr/flag compatibility.

Dependencies: source — Coordinate PR75 runner candidate; identify public callers before command rename. Checks — Source-only report extraction can be prepared now; native protocol execution remains outside current authority. Review — Fresh independent nonsecurity exact-candidate/result review under current AGENTS routing; security interpretation/design/results remain with Opus 5.5 or 4.8 high where applicable. Acceptance — #85 migration depends only on functions not yet extracted, not all #80 acceptance. U7 journey remains open.

## #81: Rust: put the Midnight ledger dependencies behind a feature

Owner: Workspace/package integrator.

crates/mori/Cargo.toml unconditionally depends on mori-backend and mori-ledger and has Midnight dev-dependencies; no feature table exists there. Issue says cargo test --workspace means all features, which Cargo does not guarantee; exact feature configurations need explicit definition.

Source: [crates/mori/Cargo.toml](../../../../crates/mori/Cargo.toml), [Cargo.toml](../../../../Cargo.toml), [crates/mori/src/commands/mod.rs](../../../../crates/mori/src/commands/mod.rs), [crates/mori-core/Cargo.toml](../../../../crates/mori-core/Cargo.toml), [AGENTS.md](../../../../AGENTS.md).

Independent acceptance: Inspect the resolved feature/dependency graph for explicit default, no-default and native configurations; later build evidence must show frontend commands without Midnight dependency compilation. Retain distinct local checks for supported configurations; use local CI under current user rule.

Refusal/regression: Native-disabled commands must report absence/unsupported honestly; no stub or native symbol may leak into claimed frontend build. Do not translate issue CI suggestion into hosted CI; --workspace alone is not an all-features assertion.

Concrete proposals: Concrete default native-on/off proposal with command availability and compatibility; inspect whether Core k256 is required independently.

Dependencies: source — Inventory all native imports and tests/dev-dependencies; feature-gate actual consumers. Checks — Build/network/dependency execution not performed; future commands require side-effect inspection and current authorization. Review — Fresh independent nonsecurity exact-candidate/result review under current AGENTS routing; security interpretation/design/results remain with Opus 5.5 or 4.8 high where applicable. Acceptance — Build packaging does not discharge target/native security requirements or independent release.

## #82: Rust: route Core, backend, ledger and CLI errors through MoriDiagnostic

Owner: Frontend/diagnostic API integrator; Opus for security refusal-code implications.

Core ir.rs still defines string-coded Error; CLI core_io still owns library-error conversion. The frontend migration outcome changes where diagnostics should originate.

Source: [crates/mori-core/src/ir.rs](../../../../crates/mori-core/src/ir.rs), [crates/mori-core/src/parser.rs](../../../../crates/mori-core/src/parser.rs), [crates/mori/src/commands/core_io.rs](../../../../crates/mori/src/commands/core_io.rs), [crates/mori-diagnostics/src/lib.rs](../../../../crates/mori-diagnostics/src/lib.rs), [crates/mori-ledger/src/lib.rs](../../../../crates/mori-ledger/src/lib.rs).

Independent acceptance: Freeze independent source/span examples including UTF-8 offsets, stable library code/cause expectations, and render snapshots against source after selected frontend ownership.

Refusal/regression: Unsupported profile and malformed source retain rejection with meaningful location; no renamed ledger refusal may bypass existing consumer matching.

Concrete proposals: Diagnostic compatibility map for externally consumed refusal codes; #76 frontend decision selects owner.

Dependencies: source — #76 for Core frontend migration; #87 for final error snapshots. Checks — Current error-path inventory can proceed; final source snapshots follow diagnostic API. Review — Fresh independent nonsecurity exact-candidate/result review under current AGENTS routing; security interpretation/design/results remain with Opus 5.5 or 4.8 high where applicable. Acceptance — Diagnostic improvement is not semantic, proof or ledger acceptance.

## #83: Rust: remove duplicated logic inside mori-core

Owner: Core semantics integrator.

Stage validation already imports runtime validate_asset/validate_liability_reads, so duplication is partial rather than total. stage/decode still has registry/asset_identity; stage/plan still hard-codes moriarty-stage-plan/1. Preserve deliberate Core/2 Await/profile differences.

Source: [crates/mori-core/src/runtime.rs](../../../../crates/mori-core/src/runtime.rs), [crates/mori-core/src/stage/validate.rs](../../../../crates/mori-core/src/stage/validate.rs), [crates/mori-core/src/stage/decode.rs](../../../../crates/mori-core/src/stage/decode.rs), [crates/mori-core/src/stage/plan.rs](../../../../crates/mori-core/src/stage/plan.rs), [crates/mori-core/src/stage/admit.rs](../../../../crates/mori-core/src/stage/admit.rs), [crates/mori-core/src/parser.rs](../../../../crates/mori-core/src/parser.rs).

Independent acceptance: Freeze independently expected whole snapshots, ordered effects, liability identity, absent-vs-zero duties and profile-specific refusals before sharing functions. Public API/export and keyword inventory must show one intended definition and every existing consumer updated.

Refusal/regression: Core/1 must continue refusing Core/2-only constructs; profile-aware sharing must retain Await, bounds and reserved-word differences. Original liability identity mismatch and wrong asset refusals remain; same-evaluator comparison alone cannot prove semantics.

Concrete proposals: #76 outcome before frontend rewrite; concrete keep/wire/deprecate proposal only for unused public authority types.

Dependencies: source — #76 may supersede parser/validation ownership; independently safe constants/consumer inventory eligible. Checks — Behavioral expectations precede refactor; no runtime or native check executed here. Review — Fresh independent nonsecurity exact-candidate/result review under current AGENTS routing; security interpretation/design/results remain with Opus 5.5 or 4.8 high where applicable. Acceptance — Source deduplication does not close complete semantic/correspondence or lifecycle parents.

## #84: Rust: move test fixtures into crates/fixtures

Owner: Test fixture owner and per-crate consumers.

Cross-boundary examples/deliverables includes and #[path] mounts to ledger test support remain in current tests. Do not treat copied evaluator-derived output as a new independent expectation.

Source: [crates/mori/tests/core_continuations.rs](../../../../crates/mori/tests/core_continuations.rs), [crates/mori/tests/check_compile_io.rs](../../../../crates/mori/tests/check_compile_io.rs), [crates/mori/tests/prepare_call.rs](../../../../crates/mori/tests/prepare_call.rs), [crates/mori-core/tests/continuations.rs](../../../../crates/mori-core/tests/continuations.rs), [RUST_GUIDELINES.md](../../../../RUST_GUIDELINES.md).

Independent acceptance: Freeze source and destination byte digests plus independent fixture membership; compare copied data exactly and show every old test consumer points to owned fixture data. A source/path enforcement check rejects an intentionally out-of-crates relative read while valid crate fixtures remain accessible.

Refusal/regression: Omitted fixture, changed expected result or stale generated copy must fail drift/membership checks; no weakened frozen denominator.

Concrete proposals: Source-of-truth and synchronization/generation direction for examples vs fixture copies, without blocking byte-preserving copying.

Dependencies: source — Copying and provenance inventory need no frontend/API decision; coordinate #86 canonical oracle ownership. Checks — Any future test target inspected for native operations; no broad Cargo test authorized by this task. Review — Fresh independent nonsecurity exact-candidate/result review under current AGENTS routing; security interpretation/design/results remain with Opus 5.5 or 4.8 high where applicable. Acceptance — Fixture relocation closes only its concrete portability predicate; conformance and release remain open.

## #85: Rust: replace mori binary tests with library tests

Owner: CLI/library test integrator.

CARGO_BIN_EXE_mori invocations remain across current tests; #80 extraction is required only for logic still inside commands. Some existing library-backed coverage can move independently.

Source: [crates/mori/tests/check_compile_io.rs](../../../../crates/mori/tests/check_compile_io.rs), [crates/mori/tests/replay.rs](../../../../crates/mori/tests/replay.rs), [crates/mori/tests/metered_intents.rs](../../../../crates/mori/tests/metered_intents.rs), [crates/mori/tests/installed_call.rs](../../../../crates/mori/tests/installed_call.rs), [crates/mori/src/commands/core_io.rs](../../../../crates/mori/src/commands/core_io.rs), [crates/mori/src/commands/compile.rs](../../../../crates/mori/src/commands/compile.rs).

Independent acceptance: Freeze test-by-test behavior-to-production-function mapping and independent expected report/file outputs before removing each old test. Retain atomic publishing, no clobber and write-fault expectations through direct helper tests; prove public command still calls the tested helper by source inspection.

Refusal/regression: A replacement testing a sibling helper unused by CLI is insufficient; deleting subprocess assertions without retained behavior coverage cannot count as completion.

Concrete proposals: No new API/semantic decision required for the independently eligible portion.

Dependencies: source — #80 only for tests of unextracted business logic; already-library-backed paths eligible now; #84 shared fixtures. Checks — Inspect each moved target for prohibited SDK/native execution before future test selection. Review — Fresh independent nonsecurity exact-candidate/result review under current AGENTS routing; security interpretation/design/results remain with Opus 5.5 or 4.8 high where applicable. Acceptance — Library tests do not demonstrate cold-install public journey or actual ledger submission.

## #86: Rust: load Sherlock oracles from JSON instead of generated Rust

Owner: Sherlock reference/oracle fixture owner.

Generated Rust oracle files and Python serialize_oracles remain; Cargo-only canonical-data drift enforcement is not established by source presence.

Source: [models/sherlock/reference/check-reference.py](../../../../models/sherlock/reference/check-reference.py), [models/sherlock/oracles](../../../../models/sherlock/oracles), [crates/mori-core/src/sherlock/normal_oracles.rs](../../../../crates/mori-core/src/sherlock/normal_oracles.rs), [crates/mori-core/src/sherlock/extra_normal_oracles.rs](../../../../crates/mori-core/src/sherlock/extra_normal_oracles.rs), [crates/mori-core/src/sherlock/reference_oracles_tests.rs](../../../../crates/mori-core/src/sherlock/reference_oracles_tests.rs).

Independent acceptance: Freeze canonical oracle byte digests, case/program/prefix membership and independently authored expectations before deserialization migration. Cargo-owned fixture/transcription validation must compare complete snapshots against unchanged independent JSON; coordinate #79 crate location.

Refusal/regression: A changed/omitted oracle field or stale copy must fail drift checks; regenerated evaluator output and insta snapshots cannot replace independent expected values.

Concrete proposals: Canonical models-vs-crate source-of-truth and synchronization direction; resolve the tension with #84 no-outside-crates reads via owned fixtures/build preparation.

Dependencies: source — #84 fixture contract; #79 eventual crate location is coordinateable, not a blanket implementation blocker. Checks — No direct rustc/Cargo or interpreter execution in this triage; future ordinary target inspected first. Review — Fresh independent nonsecurity exact-candidate/result review under current AGENTS routing; security interpretation/design/results remain with Opus 5.5 or 4.8 high where applicable. Acceptance — JSON migration alone proves no interpreter correspondence or native qualification.

## #87: Rust: snapshot and fuzz the Core and Sherlock frontends

Owner: Frontend/property-test owner; Opus owns any new security hostile-test design.

Core language.rs has a parse proptest; remaining listed consumers need reconciled coverage rather than an assertion that no properties exist anywhere. Draft/source adapter Rust-tree comparisons remain.

Source: [crates/mori-core/tests/language.rs](../../../../crates/mori-core/tests/language.rs), [crates/mori-core/src/stage/decode.rs](../../../../crates/mori-core/src/stage/decode.rs), [crates/mori-core/src/json.rs](../../../../crates/mori-core/src/json.rs), [crates/mori-core/src/sherlock/draft_source_tests.rs](../../../../crates/mori-core/src/sherlock/draft_source_tests.rs), [crates/mori-core/src/sherlock/draft_typecheck_tests.rs](../../../../crates/mori-core/src/sherlock/draft_typecheck_tests.rs), [crates/mori-core/src/sherlock/source_adapter_tests.rs](../../../../crates/mori-core/src/sherlock/source_adapter_tests.rs).

Independent acceptance: Freeze per-phase consumer list, bounded random/tricky/damaged-program inputs and source-bearing snapshot expectations; every phase has explicit coverage. Render final error snapshots after #82 through source spans/miette, with exact candidate and retained source.

Refusal/regression: Never-panic is only a robustness predicate; it cannot establish correct acceptance or semantic soundness. Unsupported/damaged inputs retain documented errors; no diagnostic snapshot may hide the source or expected refusal.

Concrete proposals: No new API/semantic decision required for the independently eligible portion.

Dependencies: source — Ordinary robustness inventory/property scaffolding can start now; final error snapshots depend on #82; #76 may alter frontend ownership. Checks — New security test judgments remain Opus-owned; source robustness preparations run no native operation. Review — Fresh independent nonsecurity exact-candidate/result review under current AGENTS routing; security interpretation/design/results remain with Opus 5.5 or 4.8 high where applicable. Acceptance — Snapshot/property coverage is not U2 correspondence or native/security acceptance.

## #88: Rust: remove env-var-driven test behavior and the task4 driver from proving paths

Owner: Evidence harness integrator; Opus owns native/resource/security controls.

proving.rs still #[path]-mounts task4_native_driver; parameters/proving call cfg(test) charge hooks. Their presence is observed, not authority to execute or remove resource accounting.

Source: [crates/mori-backend/src/task4_native_driver.rs](../../../../crates/mori-backend/src/task4_native_driver.rs), [crates/mori-backend/src/proving.rs](../../../../crates/mori-backend/src/proving.rs), [crates/mori-backend/src/parameters.rs](../../../../crates/mori-backend/src/parameters.rs), [crates/mori-backend/src/signed_relation.rs](../../../../crates/mori-backend/src/signed_relation.rs), [crates/mori-backend/tests/support/task4-native-driver.py](../../../../crates/mori-backend/tests/support/task4-native-driver.py), [docs/audits/2026-10-14/SECURITY-ROUTING.md](../../../../docs/audits/2026-10-14/SECURITY-ROUTING.md).

Independent acceptance: Freeze env-variable-to-artifact-to-audit-consumer map, output schemas/digests and historical counters before replacement proposal. Future ordinary tests use owned temp directories with explicit inputs; every required measurement artifact has a reviewed explicit replacement consumer.

Refusal/regression: Do not delete native charge hooks/caps as cosmetic cleanup or reset failures/debits; existing native authority zero still applies. Environment-invariant behavior requires specified configurations and retained ignored/native inputs; no evidence capture may be silently dropped.

Concrete proposals: Concrete replacement for each still-required capture/measurement and review of resource-accounting preservation by designated Opus owner.

Dependencies: source — Audit consumer inventory and current resource records are prerequisites to removal, not to independent source inspection. Checks — No native proving, SDK, parameter, runtime or broad Cargo checks authorized; no replacement harness executed here. Review — Fresh independent nonsecurity exact-candidate/result review under current AGENTS routing; security interpretation/design/results remain with Opus 5.5 or 4.8 high where applicable. Acceptance — Native/proof/resource result and security acceptance need separately authorized execution and applicable reviews.

## #89: Rust cleanup after #74 (tracking)

Owner: DoctorMoriarty UTXO reporting integrator plus named child owners.

GitHub shows #74 OPEN despite tracker wording that it did tier 1. #89 is a parent tracker, not a duplicate of its 13 children. Suggested broad Cargo checks and hosted CI claims do not override active native authority/local-CI constraints.

Source: [docs/utxo/doctorwatson/EARS-BASELINE.json](../../../../docs/utxo/doctorwatson/EARS-BASELINE.json), [docs/utxo/doctorwatson/ASSESSMENT.json](../../../../docs/utxo/doctorwatson/ASSESSMENT.json), [scripts/doctorwatson-utxo.sh](../../../../scripts/doctorwatson-utxo.sh), [AGENTS.md](../../../../AGENTS.md), [ROADMAP.md](../../../../ROADMAP.md).

Independent acceptance: Exact child membership is set(range(76,89)), no omission/duplicate; each has original body capture and independent missing predicate, consumer, dependencies and cleanup action. Report implementation/evidence/acceptance separately and keep PR status, source review and actual results distinct.

Refusal/regression: A merged PR, green source inventory, duplicate pointer or child test result cannot close a parent product exit. Never drop #23 finance goals or any #76–88 child; do not call all items blocked or all decisions maintainer gates on public use.

Concrete proposals: Only child-specific consequential proposals; tracker itself authorizes no new design, native execution or public developer permission.

Dependencies: source — #74 and #75 are open context; exact GitHub states and active worktree sources are separately captured. Checks — Existing ordinary report/check integration belongs to integrator; no new framework or native/provider invocation. Review — Fresh independent nonsecurity exact-candidate/result review under current AGENTS routing; security interpretation/design/results remain with Opus 5.5 or 4.8 high where applicable. Acceptance — All U0–U7, M1–M17, MC01–08, SP01–12 and G01–24 retained at original scope; tracker closure would not close them.

## Stale, overlapping and dependent facts

- Tracker describes #74 as completed tier 1; actual captured PR state is OPEN. Current local source inspection must establish each adopted child, not assume base changes. References: GH-I89, https://github.com/CharlesHoskinson/Moriarty/pull/74.
- PR74 proposes hosted Rust CI and broad workspace checks. Current user rule requires local CI; native authority zero prevents treating arbitrary broad Cargo tests as ordinary. PR74 says independent model reviews were not run. References: https://github.com/CharlesHoskinson/Moriarty/pull/74, AGENTS.md.
- PR75 says exact source/draft reviews approve source scope only; builds/tests/runtime UNRUN; binding/provenance/discovery questions and fresh changed-byte review remain before readiness/merge. References: https://github.com/CharlesHoskinson/Moriarty/pull/75.
- One-lowering title and byte-identical suggestion must be reconciled with different admitted statements/subsets; within-consumer byte regression is separate from cross-consumer semantic correspondence. References: GH-I78, UNI-019.
- October 8 tooling/test counts and 35 Sherlock files are dated issue descriptions. Current source has additional Sherlock files and local Rust commands; do not erase the originals or reuse their counts as current verification. References: GH-I23, GH-I79, ROADMAP.md.
- Tracker/child and extraction/diagnostic/fixture/oracle/property overlaps are dependencies, not proven duplicate completed work. No duplicate issue closure is justified. References: GH-I89, GH-I76, GH-I80, GH-I82, GH-I83, GH-I84, GH-I85, GH-I86, GH-I87.
- Vocabulary/profile, frontend semantics, identity migration, lowering contract, CLI aliases, feature defaults, refusal-code compatibility and evidence/resource-hook replacement need concrete proposals in their own scope. Independent inventory, copying, reporting preparation and existing-library test migration remain eligible. References: GH-I23, GH-I76, GH-I77, GH-I78, GH-I80, GH-I81, GH-I82, GH-I88.
- The body parenthesis equating cargo test --workspace with all features is not a reliable feature matrix; explicit default/no-default/native configurations must be specified. No Cargo command was run to infer results. References: GH-I81, crates/mori/Cargo.toml.

PR74 is [open](https://github.com/CharlesHoskinson/Moriarty/pull/74), not adopted or merged by this report. Its source/hash changes need exact candidate review, source-pin reconciliation and allowed local validation; the hosted-workflow proposal requires correction to the local-CI rule before acceptance. PR75 is [open](https://github.com/CharlesHoskinson/Moriarty/pull/75) and source-only; preserve the frozen expectations, invalidated historical evidence and unrun execution status. Neither PR supplies downstream product acceptance.

No issue is a demonstrated duplicate or implemented closure candidate on the inspected evidence. Preserve all original descriptions and keep all 15 open. A later reviewed GitHub cleanup can add accurate cross-links, dated progress and exact completion evidence; that remote mutation is outside this triage. Maintainer design/result review governs repository changes, while public developer compile/prove/deploy remains permissionless subject to objective validity and participant authorization.
