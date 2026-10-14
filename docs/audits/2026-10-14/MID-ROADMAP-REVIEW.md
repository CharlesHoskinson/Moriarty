# Mid-roadmap review — 2026-10-14

Reviewed baseline: `cb8280ef54082853dd8bb6c6a52050328d0fc6fa` (PR49 merge). This report records static discovery, reconciles existing public evidence and designs the remaining audit. No product tests or native/ledger actions ran in this review. Initial proposals are advisory; their disagreements and corrections are retained below. All security work and security recommendations are owned by Opus.

## What we learned

The repository contains a substantial working language implementation and several distinct authoring/execution profiles. Its roadmap reports broad acceptance exits, so it does not currently show the delivered capabilities clearly. A usable local child capability and an open parent milestone can both be true.

Replace earlier rough whole-language percentages with separately reported implementation, evidence and acceptance status. The discovery index reconciles 69 U/M/MC/SP/G identifiers; it has not flattened every mandatory clause into an atomic denominator. Neither these IDs, graph nodes, tests, lines, merged PRs nor advisory agreement provide a V1 completion percentage.

The product remains a permissionless general financial language. Fenrir requirements are language-capability and conformance obligations with additional external target dependencies. FeePool and other applications are library specimens; they do not become the product boundary or justify separate application-specific compilers.

## Current capability observations

| Capability | Public path/profile | Evidence and qualification | Parent exit still owed |
|---|---|---|---|
| Bounded parsing, validation, deterministic evaluation and step replay | Rust `mori`; language `moriarty/1`, semantics `moriarty-core/1` | Implemented source; current review inspected source, with prior scoped local results | U0 semantic contract and U2 general correspondence/replay qualification |
| Compile to ledger-9 ZKIR | `mori compile`; `mori-backend` transition relation | Implemented bounded lowering; source target descriptor is not a qualified deployed tuple | U1 basis certification; U2 general admitted-grammar/native/current-state path |
| All-action static native-call export and exact reopen | `compile-call`; unsigned/signed static modes and explicit `unsigned-mapped-roles` | PR47/49 captured source-static results; mapped mode unproven, uninstalled and unauthenticated | Native custody/authorization/proof/installation/settlement and full M1/M3 |
| Simulation, manifest tests and intent tooling | Rust `simulate`, `test`, `intent`, `verify-intent` | Real local consumers; the historical “all stubs” description is obsolete within supported scopes | Full M10/U7 developer and target qualification |
| Fee/sponsor accounting, recovery holds and metered local intents | Core policy/sponsor/intent consumers | PR46/48 local precursors; caller-supplied observations and bounded schedules | Authenticated funding/finality, actual DUST and M4/M13/M16 cost-plane qualification |
| FeePool allocation and claims | Finite `examples/core/fee-pool` source component | PR49: 23 actions, 162 captured finite financial cases, reviewed source/static exports | Complete M11 including Capacity Exchange and target-qualified effects; full U6 coverage |
| Conditional escrow and batch examples | Bounded Core library examples | Source/reference precursors; limited roles/arithmetic and supplied conditions | General U3 lifecycle and M13–M15 conformance/target evidence |
| Beta authoring/editor/MCP tools | Separate Node beta package and Source/6 bridge | Local `PreparedUnqualified`; separate experimental TypeScript Core/5 | A documented complete compile-capable developer journey and U7 release |
| Rust LSP and MCP | Rust `mori lsp` / `mori mcp` | Explicit `todo!` command bodies; no runtime invocation in this review | Honest public diagnostics/tool support and release qualification |

The PR49 counts above identify captured candidate evidence, not fresh measurements made by this review. The reviewed product source `001bfac7…` and merge baseline have the same tree `ce05a5c7…`. The [published delivery dossier](https://github.com/CharlesHoskinson/Moriarty/tree/882669a1cb44afe7b6f84372c9bce51c94e38e1e/deliverables/fee-pool-mapped-2026-10-14) preserves actual commands, failures, results and the full independent Muse review. That approval retains its source/static scope.

## Integration findings and audit priorities

1. **Name each profile and its real consumers.** Rust `mori check` routes between Core and Beta; compilation loads Rust Core. The Node beta bridge goes to a separate Source/6/TypeScript Core/5 local preparation path. A passing check in one profile does not imply compilation in another. Two executables also use the name `mori`. Inspect `crates/mori/src/commands/check.rs`, `compile.rs`, and `packages/moriarty-beta/src/bridge.ts`/`package.json`.
2. **Define admitted grammar per lowering.** `mori-backend` and `mori-ledger` encode different statements and admit different subsets. Map interpreter/transition/native-call behavior on the common subset and classify intended refusals. Public `check`/reference `test` success is not compiler correspondence. Do not merge the stacks merely because both emit ZKIR.
3. **Reconcile semantic contracts before expanding examples.** Core's positional `ensures` and successor trailing postconditions are different documented semantics. Present-zero duty tombstones, eager Boolean evaluation, finite-width arithmetic and target restrictions need profile-specific expectations. The numeric decision currently scopes successor primitives; its price/rounding policy must not silently transfer to another profile.
4. **Break shared-oracle dependencies.** Exact static reopen establishes deterministic identity; replay through the same evaluator and wrapper/API comparisons establish useful internal agreement. Neither is an independent semantic oracle. Require separately derived complete financial/work outcomes and explicit K/Quint trace mappings.
5. **Expose real developer defects separately from native blockers.** Rust server stubs, executable/profile documentation and the retained 1 MiB aggregate test-report overflow are actionable local engineering. They do not require Fenrir endpoints or a new proving campaign.
6. **Keep security claims under Opus review.** The Opus security report owns threat boundaries, confirmed observations, hypotheses, test design and security-specific roadmap recommendations. Read it with its source-only scope; no finding is promoted to a runtime exploit or security approval by this synthesis.

## U milestone review and next demonstrated capability

| Milestone | Shipped precursor | Required next evidence; full exit remains open |
|---|---|---|
| U0 | Multiple versioned source/Core implementations, target descriptors, inspect/replay tooling | One explicit profile/consumer/semantic/enforcement/target matrix, numeric scope and premise reconciliation |
| U1 | Checked arithmetic/constraint source and narrowly scoped historical native work | Required primitive completeness/soundness and target-cost certificates for the actual slice; current native evidence separately admitted |
| U2 | Public compile, local simulation/intents and static call export/reopen | Two genuinely new positive contrasting programs through the same claimed pipeline, rule-level generality, real verification and complete authoritative effects |
| U3 | Reference escrow/duties, local recovery/sponsor holds | Full conditional/partial lifecycle, joins, late outcomes, recovery authority, reservations and actual phase effects |
| U4 | Retained semantics/models and research | Required native recursive financial history, private separate-party handoff and all retained composition operators |
| U5 | Local policy, metering and sponsor precursors | Exact external adapters, actual federation policy/epoch migration, service concurrency, independent solvers and retained evidence requirements |
| U6 | Finite libraries and conformance fixtures | Full retained ACTUS/DeFi membership, exact financial semantics and separate semantic/local/native/target evidence |
| U7 | Beta docs/package/editor tools and Rust local CLI | Independent clean install/authorship/submission, two-builder reproduction, every retained release gate and full acceptance dossier |

Keep U0–U7 as the single sequence and preserve each original exit. Add child-capability observations rather than a competing milestone system. The critical path remains U0 integration → required U1 basis → general U2 execution → U3 lifecycle; U4–U7 keep their existing dependencies, including applicable SP07/SP08 semantic inputs and extension requalification. Full U6 completion is not manufactured into a prerequisite for every bounded U4 preparation task.

Near-term work should produce one demonstrated compiler/consumer correspondence slice with independent financial/work expectations, plus the directly actionable tooling/reporting fixes. Bounded source library preparation can proceed in parallel. Additional fixtures alone should not displace resolving the admitted-grammar and complete-effect seams.

## External dependencies versus language engineering

Fenrir network ID, Anchor, committee/registry identities and a qualified toolchain/ledger target tuple remain unestablished in the provided configuration. The user says Fenrir has no versioning system yet. Unknown fields stay unknown; no invented version, target identity or deployed interface is required to continue independent language work.

Preview evidence remains scoped to Preview. Fenrir is a required qualification target under the user's M rows. Neither target's receipts automatically substitute for the other. Historical Preview loan/swap evidence and local native results retain their original limited statements and trees; they do not close the general current source-to-target chain.

Native authority remains zero and old contracts/counters are unchanged. This review does not authorize SDK checks/queries, VM execution, keygen, proving, native verification, ledger submission or private-material access. Missing native/target evidence is an explicit acceptance blocker, not an excuse to label existing APIs as complete or to stop eligible source work.

## Proposal dispositions and retained dissent

- **Adopt:** one unchanged U schedule, a requirement/evidence register, an admission/profile matrix, two independently authored positive program walkthroughs, independent financial/work oracles and complete effect/lifecycle accounting.
- **Adopt:** status on separate implementation/evidence/acceptance axes; unknowns and blocked stages remain visible; no whole-language percentage while mandatory predicates are unflattened.
- **Correct:** the original Luna frontend map calls Rust Core “Core/5.” Rust declares `moriarty/1` and `moriarty-core/1`; experimental TypeScript declares its separate Core/5. `graph-profile-corrections.json` records the correction while original reports stay unchanged. The combined graph applies it.
- **Correct:** the structural producer inventories experiments without content extraction, but the shipped Node bridge imports three experiment modules. `active-import-closure.json` and the additional AST extraction cover that reachable implementation without scanning the entire historical archive.
- **Do not adopt:** proposals asking whether M1–M17 were adopted. The direct user table supplies that authority; the canonical crosswalk records it without another approval request.
- **Do not adopt:** proposals treating a mixed-payer native refusal as the second successful U2 contrast. Keep it as a separate static/refusal test; two positive programs must meet the pipeline claimed by U2.
- **Do not adopt:** proposed SDK `relation::check`/degree/VM work as ordinary zero-native testing, or broad `cargo test` commands that may reach native paths. The audit plan preserves the existing limits and inspected ordinary filters.
- **Do not adopt:** blanket claims that no independent verification exists, that no source capability may receive captured-local status, or that a historic proof qualifies the current general pipeline. Reconcile exact consumers, tuples, receipts and trees. Opus owns security-specific dispositions.
- **Defer:** choosing or unifying a canonical lowering/profile, changing numeric semantics, expanding native custody, modifying native limits or accepting a deployed target. Those need their concrete design/evidence; this review does not choose by advisory vote.

The proposals differ on requirement-first versus developer-first ordering, on compiler consolidation, and on the best contrast specimen. This review combines requirement coverage with public journeys and preserves those disagreements. All ten original proposals remain available. The earlier superseded GPT security draft is excluded from adopted findings; Opus exclusively owns security work.

## What has and has not completed

Completed here: bounded source/requirement discovery, complete tracked metadata inventory, six semantic mapping passes with corrections, ten independent audit-design proposals, an Opus-owned security review, a minimal roadmap/crosswalk revision and a concrete audit output/coverage plan.

Not completed here: exhaustive review of every historical artifact, execution of the two new programs, new product test runs, native/target experiments, full end-to-end product audit, security certification, or any U/M/MC/SP/G acceptance exit. `obligation-index.json` is a reconciliation starting point; the atomic ledger and remaining experiments are work to perform under the plan, not evidence already supplied.
