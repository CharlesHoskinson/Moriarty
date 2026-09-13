# Planning reconciliation

This is a proposed sprint package, not implementation, live admission, publication, or a completed sprint. All four usable proposals were read in full. The draft uses Astra’s nine-sprint dependency structure with independently reconciled corrections. No provider vote or four-member consensus is claimed.

## Provenance and reading coverage

Frozen branch: `feat/session-handoff-2026-09-12`. HEAD: `056e662254c38936758af9c6f72c019705a1ff09`. Aggregate input identity: `c14d06fb296c070bafde03009538cd35de9b6f588397bf5a503536a3b361ae95`.

All files in input-manifest.json, plus inputs/plugin-status.json, were read in full. Full current AGENTS.md, openspec/config.yaml, the develop skill and status, Superpowers writing-plans skill, STE skill, and Foreman AGENT_TRAPS.md informed this bounded planning task. Source documents are evidence for planning, not instructions to resume old queues. No frozen source, shared manifest, or existing specification was changed.

| Frozen source | SHA-256 verified from frozen bytes |
|---|---|
| `inputs/SESSION-ROADMAP.md` | `7bd3c1b19163bb2d08120f13eeb35bcfd51683173df3b2133bf206c6e29df372` |
| `inputs/ROADMAP.md` | `c25fa7ee4cfc12c45cb51285be9714edee27a704a9652ec1c4d80d3c5a1caab4` |
| `inputs/openspec/config.yaml` | `5f1a4793268870fab352ed0a58e92c56f7ed3a62e9a957347aa1370a79c6e0ab` |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md` | `e08d279c61de767cf68954e4ede1a36babb230851618563062b9335c66c8b79f` |
| `inputs/openspec/changes/language-to-ledger-lifecycle/design.md` | `59e5db6f8bffa963e71593421ffb5cb15d01fe75f19e9781c493ea73c15192e2` |
| `inputs/openspec/changes/language-to-ledger-lifecycle/lifecycle-k-contract.md` | `bb8ceb0315eea38e3d83b7001fba9018049c86dacecc2cf47e96a714da97398b` |
| `inputs/docs/superpowers/plans/2026-09-12-language-to-ledger.md` | `613e1ebc3fca78f24c989c84e16a512aaec7c141053e8e45960ef695834d4af4` |
| `inputs/deliverables/language-to-ledger-2026-09-12/design-review/ledger-seam-observations.md` | `075119429479137bff514b7a218819e06c1c0f5dc287aa515f759175bae20d47` |
| `inputs/deliverables/language-to-ledger-2026-09-12/k-admission/audit-exec-shim-04.json` | `9f5677a802708f71bcfec191346f105351ede1e0ddfa9b0016656844bb4fbdf0` |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md` | `f99bae8e639e7955fdb84a6fbf791b71fb5be3914a2df6f9a02da3cc1b2792f5` |
| `inputs/experiments/moriarty-language/package.json` | `3e607dfd694c06663a6ab61e4d2ee1d1b33ef114956eaae53f0524ed6b900e17` |
| `inputs/experiments/moriarty-midnight-financial/package.json` | `ab05b3df3c50cd88f16aa6550c2512dd3011ee77c573070f662c594a9f7871ec` |
| `inputs/openspec/REPORT-RECONCILIATION-2026-09-07.md` | `a1a1661fc0d554d8e469651fdbd42c41e7c9e2115c8550f369742dcee94bd5da` |
| `inputs/raw/assignments/moriarty-midnight-execution-loop-2026-09-12.md` | `274af2402a37b260969945aacf47083f9d3a74de36167ecfdd2b49fdbda1b2eb` |
| `inputs/raw/assignments/moriarty-overnight-astra-fallback-2026-09-12.md` | `5b6dc50f6edf2e613740f44a78a1dec2ac7ea42d661084f95d9a89a76b35ac3d` |
| `inputs/deliverables/session-sprint-design-2026-09-12/recovery-selection.json` | `1448d0b1341648e4a9eabdd0aeb6b5e65823c940baf111dd4b61070c0f78a0cd` |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/bounded-loan-lifecycle/spec.md` | `954c6e2cdd9c33500ecf397f2e9a2c117f54df38e6cc93b0d3c927fdc404f143` |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/financial-postconditions/spec.md` | `12619b8340e77c682bf2107538d1fda72cdce6cfb9b43388d03e306056e993ec` |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/language-delivery-reconciliation/spec.md` | `2fb10467dff6443f13a57fee1687c034727003b4ccf8728074d9b4023af5dc3c` |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-k-agreement/spec.md` | `be4f3fbedc500c0ed50e14b8e22136dace9fc7bea080d906ea8138fe543edfc8` |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-ledger-integration/spec.md` | `5e841976f842d53b1c9f6f20680c396fee45a1a26b9a43c8879a700839bfbb96` |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/loan-origination-accrual/spec.md` | `1dcb927cf7243e31f9ec44d2bd610916d5c9a3b39ca778dbf3d8441046781c52` |
| `inputs/interface-files.txt` | `2aebe09004971f15050a1535f2457d8f234a3bc77d372b4c3a4ee60798a2a28d` |

| Actual proposal | SHA-256 | Contribution status |
|---|---|---|
| `opus-proposal.json` | `3aec07396ec305976f27a86f92204e12cfd9665303209525a0f74b3ff2f383a7` | Usable independent planning contribution |
| `astra-proposal.json` | `fb0d1c5cba9d1e00934038ed5c59afed4e5102c957fdf0ae9dbbf13bd1fc2646` | Usable independent planning contribution |
| `gemini-proposal.json` | `646a3de79b84f1830833b530dd2c3022c40824b52c2c932e613c8a34cfcf329a` | Usable independent planning contribution |
| `grok-proposal.json` | `d962ece4ea9540cf981b80a41a17a4a5fee8aa3ec93525cb92ba6a75f789e59a` | Usable independent planning contribution |

Grok’s first attempt ended at max-turns2 and remains a failed attempt. The retained retry completed with stopReason=end_turn and returned modelUsage grok-4.6-build, as recorded in grok-retry-result.json. This is a planning contribution, not a K implementation audit or credit-exhaustion fallback. No usage counts are inferred.

## Reconciled contributions and disagreements

| Source | Accepted contribution | Rejected or qualified alternative |
|---|---|---|
| Astra | LL01–LL09 backbone, parallel source preparation, separate actual compiler gate, full Docker/Preview financial acceptance | Full K runtime and source-to-Compact completion precede LL06 acceptance. Existing executor audit requirements remain intact. |
| Opus | Detailed native diagnostic findings, authenticated financial constraints, static lifetime boundary | No standalone accounting sprint or new generic admission validators. The one-use diagnostic grant does not cover all K work. Result-or-blocker is not a completed sprint. |
| Gemini | Clear source/runtime capability boundaries, authenticated time, canonical readback | No clean-tree prerequisite, invented admission/accounting CLI, invented public action, or guessed executor flags. Preserve originate and partial repayment. K differential results do not prove the full tower. OpenSpec validation does not certify task closure. |
| Grok | Retained K author, fresh Astra audit, exhaustion-only fallback, one writer, parallel preparation | Result-or-blocker exits remain incomplete. The requested one current admitted diagnostic is retained. LL04 preparation can begin early, while LL06 still depends on actual K acceptance. Branch closure does not shrink the retained goal. |

The actual choice favors evidence-dependent sprints over a strictly serial preparation queue. It keeps accounting prerequisites inside the affected execution sprint. It retains existing runner and executor interfaces instead of adding another framework. It uses original requirement identities rather than provider-proposed counts as the coverage test.

## Current blockers and claim limits

Current repository status was read through the actual plugin CLI. It reported stale binding/candidate inputs for openspec/sprints/sp01-financial-contract-and-execution-admission.md, missing .moriarty-dev/runtime/current-accounting.json, and unavailable resource live state for sp01-loan-swap-grok-01. Those observations block dependent dispatch. They do not block this draft or independent source preparation. No accounting mutation occurred.

The latest K source audit remains BLOCKED_PENDING_SAME_AUTHOR_REPAIR with source/dispatch/result approval false. Historical tests and harmless preflight evidence do not close that audit. No current K, compiler, Docker, or Preview result is supplied by this planning task. C7/D1/G60 remains prospective one-use diagnostic authority, not a blanket K budget. Historical controller capacity and launcher timeout observations remain failures without inferred credit exhaustion.

## Original task identity mapping

Identity consists of frozen source path, line, and exact task text. Completed local tasks remain historical and are not rescheduled as implementation. Every mapping below preserves the full task text, including its original validation and audit obligations.

| Original identity | Exact obligation | Sprint disposition |
|---|---|---|
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:4` `0.1` | Preserve current main and unrelated preimages; record merged PR1–PR4 receipts. | LL01, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:5` `0.2` | Update ROADMAP.md with implemented local scope and retained wider gates. | LL01, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:6` `0.3` | Validate this package with `openspec validate language-to-ledger-lifecycle --strict --no-interactive`. | LL01, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:7` `0.4` | Review the complete design and plan at docs/superpowers/plans/2026-09-12-language-to-ledger.md. | LL01, LL04, LL05 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:10` `1.1` | Add failing source/Core/CLI tests for the six post reads and atomic failure. | Historical PR5/PR6/PR7 evidence, revalidated where affected in LL03/LL04/LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:11` `1.2` | Implement source/4, Core/3 and private body/kernel/ensures staging. | Historical PR5/PR6/PR7 evidence, revalidated where affected in LL03/LL04/LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:12` `1.3` | Add runnable example and update README syntax, semantics and commands. | Historical PR5/PR6/PR7 evidence, revalidated where affected in LL03/LL04/LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:13` `1.4` | Run package test/typecheck and independent exact-work/rollback/legacy probes. | Historical PR5/PR6/PR7 evidence, revalidated where affected in LL03/LL04/LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:14` `1.5` | Obtain fresh exact-candidate Astra audit; repair, publish and integrate scoped files. | Historical PR5/PR6/PR7 evidence, revalidated where affected in LL03/LL04/LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:17` `2.1` | Freeze versioned closed schemas, terms, authority boundary and independent numeric expectations. | Historical PR5/PR6/PR7 evidence, revalidated where affected in LL03/LL04/LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:18` `2.2` | Add failing funded origination, floor/ceil, duplicate-period and overflow tests. | Historical PR5/PR6/PR7 evidence, revalidated where affected in LL03/LL04/LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:19` `2.3` | Implement protected operations, complete effects, source/CLI consumer and cursor retention. | Historical PR5/PR6/PR7 evidence, revalidated where affected in LL03/LL04/LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:20` `2.4` | Run actual API/CLI and legacy checks; obtain fresh audit, repair and integrate. | Historical PR5/PR6/PR7 evidence, revalidated where affected in LL03/LL04/LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:23` `3.1` | Add source and independent complete states for debt0→100→110→80→0. | Historical PR5/PR6/PR7 evidence, revalidated where affected in LL03/LL04/LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:24` `3.2` | Implement examples/loan-lifecycle.mjs and npm loan-lifecycle-demo. | Historical PR5/PR6/PR7 evidence, revalidated where affected in LL03/LL04/LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:25` `3.3` | Check conservation, cumulative work, IDs, failed continuation and documented commands. | Historical PR5/PR6/PR7 evidence, revalidated where affected in LL03/LL04/LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:26` `3.4` | Audit complete outputs and publish the scoped result. | Historical PR5/PR6/PR7 evidence, revalidated where affected in LL03/LL04/LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:29` `4.1` | Pin retained failing case and compiled artifact; admit one bounded reproduction. | LL01, LL02 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:30` `4.2` | Resolve the reproduced backend defect without narrowing the accepted language. | LL02 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:31` `4.3` | Implement supported lifecycle rules, loader and complete observations in formal/k/. | LL03 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:32` `4.4` | Run fresh positive/negative differential corpus and a comparator mutation control. | LL03 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:33` `4.5` | Audit actual K results and retain unproved metatheorem/correspondence gates. | LL03, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:36` `5.1` | Freeze and independently review source/profile/head, authenticated state/authority/time and failure/fee constraints before implementation. | LL04 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:37` `5.1a` | Bind those reviewed constraints in the actual lowerer/custody/SDK caller. | LL04 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:38` `5.2` | Implement the already specified executor/lifetime boundary and offline fault matrix. | LL05 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:39` `5.2a` | Compile the generated lifecycle Compact contract; retain compiler output and artifact bindings to the reviewed source/profile before Docker execution. | LL06 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:40` `5.3` | Run test:ledger, test:compiled, test:finalized-state and test:preview; audit exact source. | LL04, LL06, LL07 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:41` `5.4` | Reconcile real accounting, binding, funding and bounded reviewed admission evidence. | LL02, LL06, LL07, LL08 prerequisites |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:42` `5.5` | Run complete Docker lifecycle with native readback, rejection control, raw exit and containment. | LL07 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:43` `5.6` | Obtain independent Docker result audit and current Preview admission. | LL07, LL08 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:44` `5.7` | Run the admitted Preview action through the plugin; report IDs/status and verify complete financial effects. | LL08 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:45` `5.8` | Audit actual Preview result and update scoped roadmap evidence without closing unrelated proof gates. | LL08, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:48` `6.1` | Verify every requirement has a current result or a precise open blocker. | LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:49` `6.2` | Publish reviewed changes and preserve unrelated user work and historical failures. | LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/tasks.md:50` `6.3` | Close the native goal only when all six requested acceptance outcomes pass. | LL09, full retained goal check |

## Original specification requirement and scenario identities

Each requirement retains its complete original body. Each scenario retains its complete acceptance behavior. This table maps exact headings, not replacement summaries. The afk executor requirement bodies include the normative classification, fault, and startup-race tables. LL05 must copy those row identities into its executable acceptance inventory before edits.

| Original identity | Exact heading | Sprint disposition |
|---|---|---|
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:3` | Requirement: Item 2 complete callable source boundary | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:12` | Scenario: Complete callable path | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:16` | Requirement: Immutable plan and exact current authority boundary | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:25` | Scenario: Mutable evidence preserves immutable commitments | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:29` | Requirement: Concrete one-shot runner invocation handoff | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:38` | Scenario: Actual runner-to-script protocol | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:42` | Requirement: Fixed deadlines with a retained outer margin | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:49` | Scenario: Delayed entry and deadline gap | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:53` | Requirement: Exact loaded-main classification before evidence-driven stop | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:70` | Scenario: Main failure survives successful wrappers | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:74` | Requirement: Identity-safe external containment and deterministic fault disposition | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:96` | Scenario: Safety cleanup does not manufacture evidence | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:100` | Requirement: Prover lifetime established before daemon startup | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:125` | Scenario: Post-start receipt loss cannot unbound the prover | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:129` | Requirement: Durable result and consuming CLI semantics | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:134` | Scenario: Result cannot be promoted by stdout or child zero | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:138` | Requirement: Complete isolated acceptance and separately pending live instance | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:143` | Scenario: Source approval leaves live predicates open | LL05 source, LL07/LL08 actual instance |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/bounded-loan-lifecycle/spec.md:3` | Requirement: Complete source-driven lifecycle | Historical PR7, LL03/LL04/LL07/LL08, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/bounded-loan-lifecycle/spec.md:6` | Scenario: Principal100 lifecycle | Historical PR7, LL03/LL04/LL07/LL08, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/bounded-loan-lifecycle/spec.md:10` | Requirement: Complete conservation observation | Historical PR7, LL03/LL04/LL07/LL08, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/bounded-loan-lifecycle/spec.md:13` | Scenario: Failed continuation and retry | Historical PR7, LL03/LL04/LL07/LL08, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/bounded-loan-lifecycle/spec.md:17` | Requirement: Repeatable developer example | Historical PR7, LL03/LL04/LL07/LL08, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/bounded-loan-lifecycle/spec.md:20` | Scenario: Clean reproduction | Historical PR7, LL03/LL04/LL07/LL08, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/financial-postconditions/spec.md:3` | Requirement: Versioned typed financial post-state access | Historical PR5, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/financial-postconditions/spec.md:6` | Scenario: Actual debt result | Historical PR5, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/financial-postconditions/spec.md:10` | Scenario: Invalid post-read scope | Historical PR5, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/financial-postconditions/spec.md:14` | Requirement: One staged action and atomic publication | Historical PR5, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/financial-postconditions/spec.md:17` | Scenario: Financial postcondition fails | Historical PR5, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/financial-postconditions/spec.md:21` | Scenario: Complete validation and identity ownership | Historical PR5, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/financial-postconditions/spec.md:25` | Requirement: Exact staged work and error precedence | Historical PR5, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/financial-postconditions/spec.md:28` | Scenario: Exact work boundary | Historical PR5, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/financial-postconditions/spec.md:32` | Scenario: Kernel failure precedes postconditions | Historical PR5, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/financial-postconditions/spec.md:36` | Requirement: Public tooling and regression evidence | Historical PR5, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/financial-postconditions/spec.md:39` | Scenario: Cross-entry-point result | Historical PR5, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/language-delivery-reconciliation/spec.md:3` | Requirement: Scoped merged capability status | LL01, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/language-delivery-reconciliation/spec.md:6` | Scenario: Local implementation status | LL01, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/language-delivery-reconciliation/spec.md:10` | Requirement: Preserve wider acceptance and user changes | LL01, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/language-delivery-reconciliation/spec.md:13` | Scenario: Tests without ledger evidence | LL01, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/language-delivery-reconciliation/spec.md:17` | Requirement: Persistent evidence and honest loop state | LL01, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/language-delivery-reconciliation/spec.md:20` | Scenario: Resuming after a failure | LL01, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-k-agreement/spec.md:3` | Requirement: Executable supported Core in K | LL02, LL03, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-k-agreement/spec.md:6` | Scenario: Actual K execution | LL02, LL03, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-k-agreement/spec.md:10` | Requirement: Complete differential observations | LL02, LL03, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-k-agreement/spec.md:13` | Scenario: Incomplete comparison | LL02, LL03, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-k-agreement/spec.md:17` | Requirement: Explicit proof scope and bounded execution | LL02, LL03, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-k-agreement/spec.md:20` | Scenario: Missing K binary | LL02, LL03, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-ledger-integration/spec.md:3` | Requirement: Authenticated financial state and actual language consumer | LL04–LL08, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-ledger-integration/spec.md:6` | Scenario: Stale or substituted state | LL04–LL08, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-ledger-integration/spec.md:10` | Requirement: Docker financial lifecycle before Preview | LL04–LL08, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-ledger-integration/spec.md:13` | Scenario: Local complete result | LL04–LL08, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-ledger-integration/spec.md:17` | Requirement: Current guarded Preview admission | LL04–LL08, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-ledger-integration/spec.md:20` | Scenario: Admission unavailable | LL04–LL08, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-ledger-integration/spec.md:24` | Requirement: Independent public financial acceptance | LL04–LL08, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-ledger-integration/spec.md:27` | Scenario: Partial or unverified public outcome | LL04–LL08, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/loan-origination-accrual/spec.md:3` | Requirement: Explicit funded origination and authority boundary | Historical PR6, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/loan-origination-accrual/spec.md:6` | Scenario: Funded principal creation | Historical PR6, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/loan-origination-accrual/spec.md:10` | Scenario: Unbacked or duplicate origination | Historical PR6, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/loan-origination-accrual/spec.md:14` | Requirement: Bounded integer interest with explicit rounding | Historical PR6, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/loan-origination-accrual/spec.md:17` | Scenario: Distinguishing rounding | Historical PR6, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/loan-origination-accrual/spec.md:21` | Scenario: Invalid arithmetic | Historical PR6, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/loan-origination-accrual/spec.md:25` | Requirement: Monotone period and duplicate protection | Historical PR6, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/loan-origination-accrual/spec.md:28` | Scenario: Repeated period after repayment | Historical PR6, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/loan-origination-accrual/spec.md:32` | Requirement: Whole-stack operation support | Historical PR6, LL03/LL04 regression, LL09 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/specs/loan-origination-accrual/spec.md:35` | Scenario: Hidden state forgery | Historical PR6, LL03/LL04 regression, LL09 |

## Session roadmap obligation identities

| Original identity | Exact obligation | Sprint disposition |
|---|---|---|
| `inputs/SESSION-ROADMAP.md:14` | PR1: computed funded repayment through the simulation CLI. | Historical merged baseline, LL01 provenance and LL09 scope |
| `inputs/SESSION-ROADMAP.md:15` | PR2: source-defined agreement schemas. | Historical merged baseline, LL01 provenance and LL09 scope |
| `inputs/SESSION-ROADMAP.md:16` | PR3: multiple named actions and explicit selection. | Historical merged baseline, LL01 provenance and LL09 scope |
| `inputs/SESSION-ROADMAP.md:17` | PR4: typed financial PRE reads and computed remaining repayment. | Historical merged baseline, LL01 provenance and LL09 scope |
| `inputs/SESSION-ROADMAP.md:18` | PR5: six financial POST reads in ensures; failed postconditions publish no changes. | Historical merged baseline, LL01 provenance and LL09 scope |
| `inputs/SESSION-ROADMAP.md:19` | PR6: funded origination and explicit rounded accrual, with duplicate-period protection. | Historical merged baseline, LL01 provenance and LL09 scope |
| `inputs/SESSION-ROADMAP.md:20` | PR7: complete source lifecycle, originate → accrue → partial repayment → settlement, with retained liabilities/history and cumulative work. README syntax, semantics and commands were updated. | Historical merged baseline, LL01 provenance and LL09 scope |
| `inputs/SESSION-ROADMAP.md:26` | Finish the retained K crash diagnostic source repair and obtain a fresh exact-source Astra audit. | LL01 |
| `inputs/SESSION-ROADMAP.md:27` | Finalize and independently validate the prospective one-use admission records, then execute the sole guarded diagnostic if admitted. | LL02 |
| `inputs/SESSION-ROADMAP.md:28` | Resolve the native failure without narrowing accepted language bounds. | LL02 |
| `inputs/SESSION-ROADMAP.md:29` | Implement the reviewed lifecycle K contract and complete positive/negative differential corpus, with actual K results and independent full-result audit. | LL03 |
| `inputs/SESSION-ROADMAP.md:45` | Review and implement authenticated source-to-Compact translation: exact source/profile/program/head, state, permissions, time and complete financial effects. | LL04 |
| `inputs/SESSION-ROADMAP.md:46` | Complete the already specified SDK executor/lifetime boundary and meaningful failure checks. | LL05 |
| `inputs/SESSION-ROADMAP.md:47` | Compile the generated lifecycle Compact contract and bind actual compiler artifacts to the reviewed source. | LL06 |
| `inputs/SESSION-ROADMAP.md:48` | Reconcile current accounting/funding/admission, then run the full lifecycle on Docker with complete native readback, rejection control, raw exit and containment. | LL07 |
| `inputs/SESSION-ROADMAP.md:49` | Obtain independent Docker result audit and current bounded Preview admission. | LL07, LL08 |
| `inputs/SESSION-ROADMAP.md:50` | Execute on Midnight Preview through the guarded plugin, report actual transaction IDs/status and audit complete finalized financial effects. | LL08 |
| `inputs/SESSION-ROADMAP.md:51` | Reconcile final roadmap/publication scope; retain remaining proof, correspondence, ACTUS/DeFi and full MC/SP gates. | LL09 |

## Original implementation plan structural identities

All paragraphs, exact interfaces, inputs, commands, and negative controls under these headings remain inherited. The earlier baseline and pre-existing fixed-loan execution shape are reconciled against the later session roadmap and reviewed ledger seam, rather than copied as new acceptance.

| Original identity | Exact heading | Sprint disposition |
|---|---|---|
| `inputs/docs/superpowers/plans/2026-09-12-language-to-ledger.md:11` | Global constraints | All sprints, LL09 |
| `inputs/docs/superpowers/plans/2026-09-12-language-to-ledger.md:25` | Delivery order and acceptance records | All sprints, LL09 |
| `inputs/docs/superpowers/plans/2026-09-12-language-to-ledger.md:38` | Task 0: Reconcile merged local capabilities | LL01, LL09 |
| `inputs/docs/superpowers/plans/2026-09-12-language-to-ledger.md:51` | Task 1: Financial postconditions | Historical baseline, affected regression LL03/LL04, LL09 |
| `inputs/docs/superpowers/plans/2026-09-12-language-to-ledger.md:96` | Task 2: Source origination and bounded accrual | Historical baseline, affected regression LL03/LL04, LL09 |
| `inputs/docs/superpowers/plans/2026-09-12-language-to-ledger.md:117` | Task 3: Complete source lifecycle | Historical baseline, affected regression LL03/LL04, LL09 |
| `inputs/docs/superpowers/plans/2026-09-12-language-to-ledger.md:133` | Task 4: Scoped K agreement | LL01–LL03 |
| `inputs/docs/superpowers/plans/2026-09-12-language-to-ledger.md:149` | Task 5: Authenticated ledger integration | LL04–LL08 |
| `inputs/docs/superpowers/plans/2026-09-12-language-to-ledger.md:182` | Native loop and completion | All sprints, LL09 |

## Wider ROADMAP checkbox identities retained

These obligations remain in their original program scope. Branch evidence can contribute to them only through their own acceptance gates. The September11 ledger-anchored PCD decision takes precedence over the September7 report’s stale route. All non-checkbox normative ROADMAP and report requirements also remain inherited.

| Original identity | Exact obligation | Disposition |
|---|---|---|
| `inputs/ROADMAP.md:13` | [Diagnose and fix Grok review timeouts](docs/GROK-TIMEOUTS.md). Three SP05 attempts consumed about 18 minutes without a final verdict; token usage is unknown. Preserve missing-review gates and continue independent eligible product work. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:72` | **RP01: Financial and intent semantics.** Define bounded, independent traces for the eight intent examples, the three retained held-outs, all eight DeFi regression classes and five composition operators. Map source intent, concrete plan, effects, liabilities, assumptions and successor artifacts. Specify signed nominal-debt authority separately from token spending. Co-design canonical signing and display before freezing new authority fields. Every unsupported required behavior retains an owner and closure task. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:73` | **RP02: Complete native history route.** On-ledger history follows by induction from constrained genesis, immutable operation keys and head read-then-write discipline. The PCD integration settles this in design, pending RP02 review, E1 and E2 for the core, and E4 for migration and reclaim. Specify what a private successor receives, which secrets remain private, and how off-ledger segment certificates reach the ledger through `ledger-10` `verify_proof`. Pin source and deployment versions separately. Resolve source interfaces and run independently admitted component probes before a new native campaign. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:74` | **RP03: Campaign admission.** Freeze existing commands, candidate hashes, current Grok 4.6 high/fresh GPT-6 review records, bounded resources and stop conditions for each campaign. Preserve historical charges. Migrate legacy reviewer admission fields honestly. A full MC07 campaign manifest is required for MC07, not for an earlier small probe. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:82` | Bind the [action matrix](deliverables/defi-language-design-2026-09-07/action-targets.csv) to pinned lifecycle sources and independent fixtures under RP01/MC07, retaining every existing ACTUS and DeFi requirement. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:83` | Complete and review the lexical/EBNF/static specification, formatter obligations and matched syntax study under MC01. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:84` | Implement a bounded Moriarty Core definition in K and establish its evaluator/compiler/proof correspondence within MC01/MC03/MC04/MC05. Begin with a partial-payment trace that preserves its residual duty. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:93` | Finalize versioned grammar, types, canonical representations, source diagnostics and evaluator behavior for each supported profile. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:94` | Define source intent, bounded authority, concrete plan and receipt as distinct objects. Introduce required outcome, liability, residual and temporal constructs through explicit profile extensions. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:95` | Enforce registered limits on source, values, intermediate arithmetic, collections, effects, obligations, nesting, predecessor fan-in, verification work and lifetime. Define units, rounding, overflow and rejection precisely. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:96` | Preserve obligations at episode closure and bound exhaustion. No continuation, split or migration may reset a promised global work limit. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:97` | Map each supported Core operation into Compact with meaningful positive and rejection cases. Requalify changed domains as later packages extend the language. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:105` | Implement the loan and swap integration contract with real asset identity, custody and authenticated participant roles. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:106` | Verify local Docker execution before admitted public submissions. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:107` | Finalize actual loan and swap operations on Preview and compare full state and all effects against independently derived expectations. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:108` | Account for recipients, gross debit, credit, fees, change, token denomination, obligations and remaining principal. Verify canonical finality rather than treating indexer inclusion as sufficient. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:114` | Complete the early native source/interface and component gates below. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:115` | Review the certificate relations, guard-constant lint, `Collapsed` decider constraints, canonical export and retained-proof verifier under a bounded campaign. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:116` | Produce off-ledger segment certificates over the Moriarty step at 1, 10 and 100 steps and verify serialized artifacts in an independent process. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:117` | Reject altered proof bytes, context, state, keys, accumulators, free guards, substituted `vk_repr` and unbound inner instances; the ledger discharges the accumulator pairing. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:123` | Declare and use the ledger verification seam, the operation key in contract state checked by ledger `well_formed`, under exact Midnight source and deployed-version provenance, with a deploy audit of immutable authority. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:124` | Prove the supported compiler-to-ledger correspondence with explicit domains, assumptions and audited theorem dependencies. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:125` | Bind the program digest, contract, instance, head, revision, observations, output state and complete effects across authorization, proof and ledger. Outcome signatures bind constraints; the concrete execution binds their digest and proves refinement. Exact-plan signatures may additionally bind the selected execution. Preserve cumulative partial-fill authority in durable acceptance state. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:126` | Enforce durable authorization, currentness, replay protection and unique consumption through head read-then-write discipline, checked over generated ZKIR and by experiment E1. Test two individually valid conflicting transactions, restart and recovery. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:127` | Demonstrate non-mock Preview acceptance with verification enabled and a meaningful financial state change through the versioned acceptance lineage. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:133` | Enforce ContractInvariant, IntentRefinement, TransitionValidity and HistoryCompliance in every permitted acceptance path, including constrained genesis and administrative transitions in scope. The fused step relation discharges refinement and transition validity; on-ledger history compliance follows by ledger induction. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:134` | Connect permitted route choices to signed gross authority, net outcomes, recipients, fees, new liabilities and complete effects. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:135` | Use non-circular canonical commitments, the compiled claim set and an audited immutable deployment. Reject stripped claims, arbitrary verifiers, missing dependencies, stale observations and proof-valid but intent-invalid actions. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:136` | Replace verifier revocation with forward-declared migration and a principal-threshold pause. Check program-digest bounds before expensive proving, and keep migration consumption-preserving. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:137` | Produce the extended native evidence and requalify compiler/ledger correspondence for the actual mandatory relation. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:143` | Prove a successor in an isolated participant environment without access to predecessor secrets. Inventory artifact recipients, confidentiality and recovery ownership. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:144` | Realize ledger-atomic split and join, cross-contract release with reclaim, and certificates for off-ledger branches, with compatible policies, distinct heads and no duplicate consumption. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:145` | Preserve live liabilities, residual authority and conserved global work. Reject authority amplification, debt erasure and lifecycle reset. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:146` | Give separate semantics and compatibility rules to sequential, disjoint parallel, shared-state interleaving, atomic synchronization and asynchronous messaging. A required unsupported operator remains open. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:147` | Exercise pending/claimable/settled states, cancellation/fill races, unavailable witnesses, conflict and bounded recovery through the same acceptance lineage. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:153` | Compare every present result field in all 277 pinned ACTUS fixtures across the 18 executable types. Preserve all 32 taxonomy dispositions and resolve [DS-01 through DS-07](docs/research/2026-09-06-actus-defi-design-study.md#source-discrepancies-and-dispositions) with primary evidence. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:154` | Implement each of the 72 historical DeFi rows with exact modeled product/version scope, independent expected observations, feasible positives and meaningful mutations. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:155` | Complete NAM19 capitalization, accepted refinance and pending redemption, including zero-payoff capitalization, debt identity and carried unfilled obligations. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:156` | Cover required arithmetic, ordering, bad debt, shared accounting, temporal settlement, margin, contingent claims and environment assumptions through the shared bounded semantics. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:157` | Establish source/model fidelity and scoped certificate judgments. Keep proposed theorems, tests and mechanized proofs distinct. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:158` | Derive the episode manifest and resources from actual behavior coverage. Track semantic, profile-proof, local-acceptance and Preview-acceptance evidence separately; a row count or small public sample cannot close all four. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:159` | Extend and reprove affected language, proof, compiler and acceptance domains under the versioned lineage. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:165` | Deliver authoring, checking, simulation, semantic signing, proving, submission and finalized-effect inspection for the supported Midnight language. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:166` | Render from canonical signed bytes. Make fees, debt, limits, locks, recovery rights, assumptions and outstanding obligations visible. Reject unknown semantic extensions and display/signature mismatches. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:167` | Demonstrate valid loan/swap flows, rejected intent, stale data, unavailable witness, pending settlement, restart, conflict, revocation and migration. | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:168` | Assemble exact source/proof/ledger/audit evidence for every release gate, including the charter's [G01-G24 obligations](openspec/MORIARTY-COMPLETION-PROGRAM.md#completion-and-broader-release-scope). | Retained wider-program gate, LL09 checks scope |
| `inputs/ROADMAP.md:169` | Obtain final independent Fable 5.1 at medium effort and GPT-6 reviews of the actual accepted candidates. Recompute acceptance against the final deployed lineage. | Retained wider-program gate, LL09 checks scope |

## Inherited executor matrix row identities

The exact original cells remain normative, including failure-code precedence and retained ownership. These rows map to LL05 isolated source acceptance and LL07/LL08 real-instance evidence. The identity is the source line and exact first cell, not a required row count.

| Original identity | Exact row identity | Sprint disposition |
|---|---|---|

| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:60` | loaded active/running, MainPID positive, no terminal main record | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:61` | loaded active/exited, MainPID 0, Result success, ExecMainCode 1, ExecMainStatus 0 | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:62` | loaded failed/failed or active/exited, MainPID 0, Result exit-code, ExecMainCode 1, ExecMainStatus 1..255 | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:63` | loaded failed/failed or active/exited, MainPID 0, Result signal or core-dump, ExecMainCode 2 or 3, ExecMainStatus 1..64 | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:64` | unloaded, zero/default identity, changed InvocationID, malformed fields, deactivating or any other transitional/inconsistent combination | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:65` | show return nonzero, exception or timeout | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:66` | collection cutoff with only running observations, or outer kill without a retained terminal observation | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:83` | running then exact zero terminal, durable writes, stop zero, receipt and C, timer cancel retained | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:84` | malformed or unpersisted startup after launch | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:85` | InvocationID mismatch or preexisting evidence/stop receipt | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:86` | known nonzero or signal main exit | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:87` | terminal-zero raw/validated write, short write or file/directory fsync failure | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:88` | stop nonzero, exception or timeout after durable zero terminal | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:89` | actual stop zero but receipt persistence fails | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:90` | deadline crossing or parent loss after startup | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:91` | prover survives, containment observation fails or ownership drifts | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:92` | C retained but timer cancel/receipt fails | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:93` | final result write/fsync fails | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:94` | input/current authority or strong containment refusal before resource start | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:115` | Container/config/control validation or pre-start file/directory fsync fails | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:116` | Executor dies after valid control/intent but before Docker start | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:117` | Docker starts successfully, then executor dies before launcher or post-start ownership receipt | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:118` | Post-start ownership write/fsync fails, or launcher activation fails after prover start | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:119` | Docker startup reaches PID 1 at or after outer+120, including completion after parent timeout | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:120` | Wrapper is signalled/killed or prover forks/changes process group | LL05, LL07, LL08 |
| `inputs/openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md:121` | A later container or generation appears under an old name | LL05, LL07, LL08 |

## K contract section identities

All normative paragraphs under each heading remain inherited in LL01–LL03. The referenced independent oracle and constructor inventory must be expanded by exact identities before LL03 edits. Those referenced artifacts were not copied into the frozen manifest. This draft does not claim a fresh case-inventory or constructor-content audit from their names or counts.

| Original identity | Exact section | Sprint disposition |
|---|---|---|
| `inputs/openspec/changes/language-to-ledger-lifecycle/lifecycle-k-contract.md:5` | Objective and scope | LL01–LL03 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/lifecycle-k-contract.md:11` | Transport and context | LL01–LL03 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/lifecycle-k-contract.md:19` | K representation and complete output | LL01–LL03 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/lifecycle-k-contract.md:27` | Frozen lifecycle observations | LL01–LL03 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/lifecycle-k-contract.md:33` | Historical crash and execution gates | LL01–LL03 |
| `inputs/openspec/changes/language-to-ledger-lifecycle/lifecycle-k-contract.md:39` | Files and validation | LL01–LL03 |

## Validation

Actual CLI discovery returned `/home/charl/.local/bin/openspec`, version 1.10.0. Strict validation ran on an unchanged copy of the four draft files at `openspec/changes/session-completion-sprints/` inside a disposable temporary repository. The frozen `openspec/config.yaml` was copied into that temporary repository. The shared OpenSpec tree was not modified.

Executed command from that isolated repository:

```sh
openspec validate session-completion-sprints --type change --strict --no-interactive --json
```

Result: exit 0, valid=true, issues=[], one change passed and none failed. The temporary tree was removed after validation. This is real OpenSpec structural validation of the proposed change, not installation, archival, task completion, behavioral verification, or proof acceptance.

Planning consistency checks verified every frozen manifest SHA-256, all four proposal input identities, every original task/requirement/scenario mapping, all original executor matrix-row identities, all nine sprint headings, and the absence of checked task boxes. Every proposed task remains unchecked. Current CLI status was read successfully. No source tests, native K, actual Compact compilation, Docker, Preview, dispatch, accounting mutation, or publication occurred for this planning task.

Self-review retained each source obligation by identity and rejected blocker-as-completion, clean-tree reset, invented callable commands, omitted lifecycle actions, historical-result promotion, and unapproved resource reuse. Proposed evidence and implementation paths remain identified as proposals. Exact future dispatch argv is intentionally bound to the actual reviewed candidate and current admission, rather than guessed here.
