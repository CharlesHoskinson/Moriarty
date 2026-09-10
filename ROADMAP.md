# Moriarty roadmap

Moriarty is a Midnight-centric language for bounded financial contracts and proof-carrying transactions. Developers should be able to express a financial agreement, inspect its possible effects, authorize an outcome, prove a valid transition and settle it on Midnight. Each accepted transition must preserve the contract's rules, signed intent and compliant predecessor history.

Compact, Midnight native proofs, private state and ledger acceptance constrain the language design. Preview is the public development network. ACTUS supplies standardized financial events and cash flows; the DeFi corpus supplies protocol behaviors and adversarial cases. Examples from other chains are financial references, not additional backend deliverables.

**Hard release gate: Moriarty must work end to end on Midnight Preview.** Programs compiled from Moriarty source must produce actual financial transactions whose finalized state and complete effects match independent expectations. Retain the source/profile/compiler bindings, submitted transaction bytes and IDs, canonical finality, exact readback and current independent audits. SP05 must demonstrate both loan and swap settlement; SP09 must demonstrate verification-enabled mandatory acceptance; SP11 and SP12 must satisfy their full behavior coverage and release obligations. Local tests, local settlement, hello-world receipts and source reviews cannot close this gate. It remains open until the required Preview evidence exists. The user's September 10 instruction makes this requirement non-negotiable; a blocked Midnight interface does not authorize changing the target or weakening acceptance.

This is the complete current roadmap. [OpenSpec](openspec/MORIARTY-COMPLETION-PROGRAM.md) contains detailed package contracts; the [machine register](openspec/moriarty-completion-program.json) records scoped status and dependencies. The [three-report reconciliation](openspec/REPORT-RECONCILIATION-2026-09-07.md) defines the early decisions and staged proof admission. Editing these plans neither completes a package nor arms an execution loop.

## Grok review reliability

- [ ] [Diagnose and fix Grok review timeouts](docs/GROK-TIMEOUTS.md). Three SP05 attempts consumed about 18 minutes without a final verdict; token usage is unknown. Preserve missing-review gates and continue independent eligible product work.

## Refined delivery roadmap

The [asset-study integration](openspec/ASSET-STUDY-INTEGRATION-2026-09-09.md) adds eleven required implementation obligations and eight acceptance cases across the existing sprints. The [task crosswalk](openspec/sprints/asset-study.json) covers asset/claim transformations, encumbrances, operation-specific policies, exceptional authority, servicing, identity/privacy and history. Supported profiles require Docker testing followed by separately admitted Midnight Preview evidence. These obligations are open; the research does not establish implemented language features.

The [September 9 refinement](openspec/ROADMAP-REFINEMENT-2026-09-09.md) incorporates the PCD/intents reports, four-paper taxonomy, vault report, modern standards atlas and orchestration postmortems. It preserves all original OpenSpec requirements and gates. The [report crosswalk](openspec/sprints/report-lessons.json) assigns eighteen lesson groups, TX01–TX12, VX01–VX06, all 24 modern cases and three identified source gaps to existing tasks. Research classifications become explicit acceptance obligations, never proof or network evidence.

| Sprint | Deliverable | Decisive completion evidence |
| --- | --- | --- |
| [SP01](openspec/sprints/sp01-financial-contract-and-execution-admission.md) | Freeze behavior and reuse accepted foundations | Complete behavior/source/authority crosswalk; current atomic evidence retained; bounded native go/no-go. |
| [SP02](openspec/sprints/sp02-complete-mori-authoring-frontend.md) | Finish the language contract and authoring tools | Full lexical/EBNF/static specification plus working check/format and matched syntax study. |
| [SP03](openspec/sprints/sp03-executable-bounded-semantics-in-k.md) | Make K execute the financial distinctions | Runnable K/evaluator agreement and discharged base-domain semantic/correspondence claims. |
| [SP04](openspec/sprints/sp04-complete-native-verifier-component-feasibility.md) | Decide whether the full native verifier works | All native/outer verifier controls, including the real final accumulator decision. |
| [SP05](openspec/sprints/sp05-financial-integration-on-preview.md) | Finalize actual financial loan and swap effects on Preview | Both loan and swap finalized on Preview with complete independently checked effects. |
| [SP06](openspec/sprints/sp06-real-recursive-financial-history.md) | Produce and independently verify real recursive history | Real two-step recursive proof; retained bytes independently verified and mutations rejected. |
| [SP07](openspec/sprints/sp07-actus-obligations-and-lifecycle-semantics.md) | Implement ACTUS without losing debt or fields | All 277 ACTUS fixtures/all fields, 18 executable types and 32 source-backed dispositions. |
| [SP08](openspec/sprints/sp08-defi-actions-and-outcome-intents.md) | Implement behavior-driven DeFi and intent libraries | All 72 DeFi rows, DA24, intent/request lifecycles and report-derived modeled regressions. |
| [SP09](openspec/sprints/sp09-mandatory-pcd-and-ledger-correspondence.md) | Require complete proof and authority at ledger acceptance | All four mandatory claims, proved correspondence and verification-enabled Preview acceptance. |
| [SP10](openspec/sprints/sp10-private-handoff-and-bounded-composition.md) | Prove private handoff and all five composition operators | Isolated private handoff, real split/join and all five composition operators through acceptance. |
| [SP11](openspec/sprints/sp11-full-financial-and-formal-conformance.md) | Qualify the complete financial and formal scope | Every required behavior qualified across semantics, proof, local and required Preview evidence. |
| [SP12](openspec/sprints/sp12-developer-release-and-reproducible-evidence.md) | Release something developers can use and reproduce | Usable end-to-end flow, two clean builders, two non-toy pilots and every G01–G24 gate. |

Start three eligible tracks: **SP01→SP02→SP03** for language, **SP01 F0→SP04→SP06** for native feasibility/proofs, and **accepted atomic + loan/swap subset→SP05** for financial Preview integration. SP09.1 atomic F3 joins SP05/SP06 early; the full successor joins after SP07/SP08. Then SP10→SP11→SP12. Use the exact stage prerequisites below, not whole-sprint barriers.

The next successor demonstration is a `.mori` partial payment that preserves its residual obligation in Core, K and the evaluator. The bounded Preview [loan](deliverables/sp05-financial-integration-2026-09-09/preview-loan-01/RESULT.md) and [swap](deliverables/sp05-financial-integration-2026-09-09/preview-swap-01/RESULT.md) now have both independent result audits. The [admission-bound cumulative native DUST comparison](deliverables/sp05-financial-integration-2026-09-09/preview-fee-comparison-01/RESULT.md) is implemented with both source reviews. The [Preview command-completion correction](deliverables/sp05-financial-integration-2026-09-09/preview-exit-completion-01/RESULT.md) also has both source reviews. The [corrected Preview loan run](deliverables/sp05-financial-integration-2026-09-09/preview-loan-exit-01/RESULT.md) has both independent financial result approvals and emitted `FINANCIAL_COMPLETE`, with separate outer containment observed. Its transient service unloaded before the raw main-process exit was captured; that missing evidence keeps the command exit-zero gate open. The [corrected Preview swap](deliverables/sp05-financial-integration-2026-09-09/preview-swap-exit-01/RESULT.md) now has both independent approvals for its financial effects, actual zero exit and separate containment. Both cases have consumed their two public attempts; a further attempt requires an explicit reviewed amendment. The original launchers exited one, and the later swap success does not repair the loan’s missing raw exit. Mandatory verification-enabled Preview acceptance remains a later hard gate. The missing current accounting/operational history blocks its dependent campaign dispatch, not this planning work or unrelated eligible development. No new runner, loop, dashboard or approval bureaucracy is part of this refinement.

## Sprint delivery plan

The [twelve OpenSpec sprints](openspec/sprints/README.md) schedule the complete roadmap: financial design and admission; source language; K semantics; native verifier feasibility; Preview financial integration; real recursion; ACTUS; DeFi and intents; mandatory PCD and ledger correspondence; private composition; full conformance; developer release. Language and native feasibility work can progress independently until their acceptance boundary. Each sprint has explicit deliverables, file ownership and rejection criteria. The [coverage crosswalk](openspec/sprints/coverage.json) retains every original requirement. These are delivery gates, not calendar or compute estimates.

## What exists

The repository contains an experimental bounded agreement syntax, parser, type checker, canonical encoding, local evaluator and restricted Compact lowering. Loan and swap examples run locally. A separate browser mock explores proposed developer flows with simulated authority and certificates. The initial atomic profile has accepted input-boundary correction and candidate-bound reviews: atomic-prepare, atomic-accept and RP01-MC02 are complete in the [retained reconciliation](evidence/moriarty-completion-program-2026-09-07/report-reconciliation/atomic-reconciliation.json). Full successor language and MC01 acceptance remain open.

[Retained Preview evidence](evidence/midnight-preview-2026-09-07/README.md) records a hello-world deployment and call finalized with exact state readback. That demonstrates basic network integration. The [reviewed first-period Preview loan](deliverables/sp05-financial-integration-2026-09-09/preview-loan-01/RESULT.md) now demonstrates four finalized native stages and a lender payment of 533,972,602 test-asset units with 4,500,000,000 notional still outstanding. The [reviewed Preview swap](deliverables/sp05-financial-integration-2026-09-09/preview-swap-01/RESULT.md) also demonstrates a 10,000-A trade for 19,743 B and reserve withdrawal at close. Full financial transfer coverage on Preview and mandatory Moriarty PCD acceptance remain open. The original native recursion experiment exhausted rows at k17. Its fixed-instance replacement is source work that has not produced a recursive proof. The complete native-to-Preview verifier remains unresolved. A [reviewed source and cost analysis](deliverables/sp04-native-finalizer-source-2026-09-10/RESULT.md) now pins the foreign-field and pairing schedule, but leaves subgroup/identity correspondence, exact SRS binding, the complete verifier cost and deployed Preview alignment open. Its partial bounds do not authorize a native retry.

A [provisional successor syntax profile](experiments/moriarty-language/spec/successor/README.md) now has separate lexical/EBNF files, a bounded parser, canonical formatter and read-only source CLI. Its [checks and independent review](deliverables/successor-syntax-2026-09-09/README.md) cover syntax only. It does not type or execute partial payments, freeze RP01, or close SP02.

The [40-constructor expression contract](deliverables/sp01-expression-contract-2026-09-10/RESULT.md) now has both independent source/design approvals, including exact value encoding and simultaneous bounds. This closes the missing expression-table and representation portion of C04-F1. The [source Boolean reconciliation](deliverables/sp01-surface-core-contract-2026-09-10/RESULT.md) now has both independent scoped approvals for short-circuit evaluation and valid diagnostic spans. The [bounded signing/context checker](deliverables/sp01-signing-context-contract-2026-09-10/RESULT.md) also has both scoped source approvals after correcting exact-permission reuse, lifecycle and currentness. Full38 financial authority and operation equations, authenticated signatures and full RP01 freeze remain open; no full successor runtime or K acceptance is inferred.

The [local execution proposal](deliverables/sp05-financial-integration-2026-09-09/local-command-execution-proposal-01/RESULT.md) has both source and bounded-resource approvals for testing the corrected local command path. No case has been admitted or started. The retained available-memory preflight failed its 20 GiB minimum; execution must satisfy that and all other live gates before consuming a case.

A separate [funded repayment projection](deliverables/funded-repayment-2026-09-09/README.md) now moves cash, consumes transfer funding once and retains residual principal and accrued debt. The [funded source preparation path](deliverables/source-core-repayment-2026-09-09/README.md) connects `.mori` elaboration to this projection. The [bounded K experiment](deliverables/bounded-k-2026-09-09/README.md) executes 16 cases with complete K/source/independent-result agreement. Authenticated state, formal correspondence, full SP03/RP01 and complete Midnight Preview financial coverage remain open.

The research corpus, source snapshots, target inventories and scoped experimental evidence remain useful inputs. None substitutes for the acceptance results below. Superseded A4/A5 work remains historical and is not an active execution queue.

## Design decisions before expanded execution

These checks belong to the existing work packages. They can progress from the accepted atomic subset while successor language work remains open.

- [ ] **RP01: Financial and intent semantics.** Define bounded, independent traces for the eight intent examples, the three retained held-outs, all eight DeFi regression classes and five composition operators. Map source intent, concrete plan, effects, liabilities, assumptions and successor artifacts. Specify signed nominal-debt authority separately from token spending. Co-design canonical signing and display before freezing new authority fields. Every unsupported required behavior retains an owner and closure task.
- [ ] **RP02: Complete native history route.** Specify what a private successor receives, which secrets remain private, how predecessor proofs compose, and how the final native accumulator decision reaches the actual Midnight verifier. Pin source and deployment versions separately. Resolve source interfaces and run independently admitted component probes before a new native campaign.
- [ ] **RP03: Campaign admission.** Freeze existing commands, candidate hashes, current Grok 4.6 high/fresh GPT-6 review records, bounded resources and stop conditions for each campaign. Preserve historical charges. Migrate legacy reviewer admission fields honestly. A full MC07 campaign manifest is required for MC07, not for an earlier small probe.

RP01 preserves all 277 ACTUS fixtures, 32 ACTUS taxonomy dispositions and 72 historical DeFi rows. Normalizing product/version scope cannot reduce these requirements. Additional report holdouts need pinned primary sources before becoming source-defined behavior cases. The early review is design coverage; full implementation and evidence remain MC07.

## Source specification and DeFi reference actions

The [DeFi and language-design amendment](openspec/DEFI-LANGUAGE-DESIGN-2026-09-07.md) adds action-level reference targets while preserving the existing financial corpus. Source files use `.mori`. The successor specification uses EBNF, separate lexical rules, static judgments and executable operational semantics in K. The [surface and semantics proposal](deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md) recommends financial blocks with explicit pre/post state; it does not change the current atomic profile.

- [ ] Bind the [action matrix](deliverables/defi-language-design-2026-09-07/action-targets.csv) to pinned lifecycle sources and independent fixtures under RP01/MC07, retaining every existing ACTUS and DeFi requirement.
- [ ] Complete and review the lexical/EBNF/static specification, formatter obligations and matched syntax study under MC01.
- [ ] Implement a bounded Moriarty Core definition in K and establish its evaluator/compiler/proof correspondence within MC01/MC03/MC04/MC05. Begin with a partial-payment trace that preserves its residual duty.

## Implementation and acceptance checklist

### MC01: Bounded language and developer-facing semantics

`MC01-ATOMIC` is the existing atomic profile after its input correction, applicable checks and current candidate-bound implementation audits. Only this milestone gates the initial financial/native experiments. The remaining source/IR expansions form `MC01-EXTENSIONS`; full RP01 gates those successor profiles, not acceptance of the existing subset.

Retained completed submilestone: the atomic input-boundary correction and its exact candidate audits. Requalify changed scope; do not repeat the historical repair.
- [ ] Finalize versioned grammar, types, canonical representations, source diagnostics and evaluator behavior for each supported profile.
- [ ] Define source intent, bounded authority, concrete plan and receipt as distinct objects. Introduce required outcome, liability, residual and temporal constructs through explicit profile extensions.
- [ ] Enforce registered limits on source, values, intermediate arithmetic, collections, effects, obligations, nesting, predecessor fan-in, verification work and lifetime. Define units, rounding, overflow and rejection precisely.
- [ ] Preserve obligations at episode closure and bound exhaustion. No continuation, split or migration may reset a promised global work limit.
- [ ] Map each supported Core operation into Compact with meaningful positive and rejection cases. Requalify changed domains as later packages extend the language.

Acceptance: the supported language profile has a tested frontend/evaluator/lowering and current scoped audits. This does not establish native proofs or financial corpus conformance. [Detailed plan](openspec/changes/mc01-bounded-language/README.md).

### MC02: Actual financial operations on Preview

The [reviewed local loan trace](deliverables/sp05-financial-integration-2026-09-09/local-continuation-02/REVIEWED-RESULT.md) retains four matching finalized stages, actual lender payment, borrower change and residual principal. The [reviewed local swap trace](deliverables/sp05-financial-integration-2026-09-09/local-swap-continuation-01/REVIEWED-RESULT.md) also covers swap and close with independently checked financial effects. Independent outer containment was observed separately from the raw inner `INCOMPLETE` results. The [reviewed local rejection](deliverables/sp05-financial-integration-2026-09-09/local-stale-loan-01/REVIEWED-RESULT.md) and [separate read-only comparison](deliverables/sp05-financial-integration-2026-09-09/after-rejection-readback-01/REVIEWED-RESULT.md) now satisfy one scoped local failed-transaction case: actual trusted-node pool rejection plus complete native state, decoded-field and balance equality at canonical endpoints 20422 and 20426, after post-terminal barrier 20425. This does not establish a stale-specific rejection reason, exclude intermediate changes, or recover the original missing report. The [reviewed Preview loan](deliverables/sp05-financial-integration-2026-09-09/preview-loan-01/RESULT.md) now adds actual deploy, initialize, accrue and first-period settle transactions at blocks 807289, 807293, 807297 and 807301. Independent native decoding and historical state reads match the reviewed build, complete fields, balances and financial effects. Its raw inner FAILED/INCOMPLETE and separate actual outer containment remain intact. The [reviewed Preview swap](deliverables/sp05-financial-integration-2026-09-09/preview-swap-01/RESULT.md) adds deploy, initialize, trade and close at blocks 807510, 807515, 807526 and 807530, with a 10,000-A trade for 19,743 B and provider withdrawal of 1,010,000 A and 1,980,257 B. Both native asset reserves finish at zero. The admission-bound cumulative fee comparator and command-completion correction now have both source reviews. The corrected loan adds four reviewed financial stages at blocks 808053, 808058, 808063 and 808067; its unavailable raw process exit remains a recorded gap. The corrected swap adds deploy, initialize, swap and close at blocks 808320, 808325, 808337 and 808341, with both result reviews accepting the actual zero exit and separate containment. The [local command-completion correction](deliverables/sp05-financial-integration-2026-09-09/local-command-completion-01/RESULT.md) now has both independent source approvals. The [actual corrected local loan command](deliverables/sp05-financial-integration-2026-09-09/local-command-execution-proposal-01/loan/RESULT.md) completed with captured exit zero and separately verified cleanup. Both result audits approve command completion, while independent historical state bytes remain missing and block the next local swap admission. The existing loan must not be rerun to recover that read-only evidence. Final SP05 reconciliation and the loan raw-exit gap remain open; the MC02 acceptance boxes stay open.

- [ ] Implement the loan and swap integration contract with real asset identity, custody and authenticated participant roles.
- [ ] Verify local Docker execution before admitted public submissions.
- [ ] Finalize actual loan and swap operations on Preview and compare full state and all effects against independently derived expectations.
- [ ] Account for recipients, gross debit, credit, fees, change, token denomination, obligations and remaining principal. Verify canonical finality rather than treating indexer inclusion as sufficient.

Acceptance: retained transaction and finalized-block evidence establishes exact financial integration. This contract remains an uncertified probe until the mandatory acceptance lineage is complete. [Detailed plan](openspec/changes/mc02-preview-financial-operation/README.md).

### MC03: Real native recursive proof

- [ ] Complete the early native source/interface and component gates below.
- [ ] Correct and review the fixed financial relation, successor commands, canonical export and retained-proof verifier under a bounded campaign.
- [ ] Produce genuine recursive proofs for the admitted two-step loan episode and verify serialized artifacts in an independent process.
- [ ] Reject altered proof bytes, context, state, keys and accumulators; discharge the complete native final decision.

Acceptance: actual retained native IVC evidence for the exact fixed episode. A nonrecursive re-proof of the same table, hash chain or host verdict cannot substitute. General DSL execution and private branching remain later requirements. [Detailed plan](openspec/changes/mc03-native-recursive-proof/README.md).

### MC04: Compiler and ledger correspondence

- [ ] Implement the complete native verification boundary under exact Midnight source and deployed-version provenance, including canonical decoding and final accumulator/pairing verification.
- [ ] Prove the supported compiler-to-ledger correspondence with explicit domains, assumptions and audited theorem dependencies.
- [ ] Bind program, semantic profile, policy, verifier, predecessors, observations, output state and complete effects across authorization, proof and ledger. Outcome signatures bind constraints; the concrete execution binds their digest and proves refinement. Exact-plan signatures may additionally bind the selected execution. Preserve cumulative partial-fill authority in durable acceptance state.
- [ ] Enforce durable authorization, currentness, replay protection and unique predecessor consumption. Test two individually valid conflicting transactions, restart and recovery.
- [ ] Demonstrate non-mock Preview acceptance with verification enabled and a meaningful financial state change through the versioned acceptance lineage.

Acceptance: the actual ledger consumes the checked history and applies exactly the authorized effects. A circuit preparation gadget or unconstrained host boolean is insufficient. [Detailed plan](openspec/changes/mc04-ledger-correspondence-and-consumption/README.md).

### MC05: Mandatory correctness and intent acceptance

- [ ] Enforce ContractInvariant, IntentRefinement, TransitionValidity and HistoryCompliance in every permitted acceptance path, including constrained genesis and administrative transitions in scope.
- [ ] Connect permitted route choices to signed gross authority, net outcomes, recipients, fees, new liabilities and complete effects.
- [ ] Use non-circular canonical commitments and trusted deployment policy. Reject stripped claims, arbitrary verifiers, missing dependencies, stale observations and proof-valid but intent-invalid actions.
- [ ] Enforce verifier/spec activation and revocation, cheap bounded admission before expensive verification, and safe consumption-preserving migration.
- [ ] Produce the extended native evidence and requalify compiler/ledger correspondence for the actual mandatory relation.

Acceptance: a required claim cannot be removed, downgraded or replaced with a simulation. [Detailed plan](openspec/changes/mc05-mandatory-claim-acceptance/README.md).

### MC06: Private handoff and composition

- [ ] Prove a successor in an isolated participant environment without access to predecessor secrets. Inventory artifact recipients, confidentiality and recovery ownership.
- [ ] Produce genuine split, independent branch and join proofs with compatible policies, distinct identities and no duplicate predecessor use.
- [ ] Preserve live liabilities, residual authority and conserved global work. Reject authority amplification, debt erasure and lifecycle reset.
- [ ] Give separate semantics and compatibility rules to sequential, disjoint parallel, shared-state interleaving, atomic synchronization and asynchronous messaging. A required unsupported operator remains open.
- [ ] Exercise pending/claimable/settled states, cancellation/fill races, unavailable witnesses, conflict and bounded recovery through the same acceptance lineage.

Acceptance: actual private multi-party history composition and its financial/ledger predicates, not merely a linear proof or shared-process demonstration. [Detailed plan](openspec/changes/mc06-private-handoff-and-composition/README.md).

### MC07: Full ACTUS and DeFi conformance

- [ ] Compare every present result field in all 277 pinned ACTUS fixtures across the 18 executable types. Preserve all 32 taxonomy dispositions and resolve [DS-01 through DS-07](docs/research/2026-09-06-actus-defi-design-study.md#source-discrepancies-and-dispositions) with primary evidence.
- [ ] Implement each of the 72 historical DeFi rows with exact modeled product/version scope, independent expected observations, feasible positives and meaningful mutations.
- [ ] Complete NAM19 capitalization, accepted refinance and pending redemption, including zero-payoff capitalization, debt identity and carried unfilled obligations.
- [ ] Cover required arithmetic, ordering, bad debt, shared accounting, temporal settlement, margin, contingent claims and environment assumptions through the shared bounded semantics.
- [ ] Establish source/model fidelity and scoped certificate judgments. Keep proposed theorems, tests and mechanized proofs distinct.
- [ ] Derive the episode manifest and resources from actual behavior coverage. Track semantic, profile-proof, local-acceptance and Preview-acceptance evidence separately; a row count or small public sample cannot close all four.
- [ ] Extend and reprove affected language, proof, compiler and acceptance domains under the versioned lineage.

Acceptance: no missing required fixture, field or behavior and no substituted taxonomy, permissive tolerance or unrelated validator. [Detailed plan](openspec/changes/mc07-complete-financial-conformance/README.md).

### MC08: Developer workflow and release evidence

- [ ] Deliver authoring, checking, simulation, semantic signing, proving, submission and finalized-effect inspection for the supported Midnight language.
- [ ] Render from canonical signed bytes. Make fees, debt, limits, locks, recovery rights, assumptions and outstanding obligations visible. Reject unknown semantic extensions and display/signature mismatches.
- [ ] Demonstrate valid loan/swap flows, rejected intent, stale data, unavailable witness, pending settlement, restart, conflict, revocation and migration.
- [ ] Assemble exact source/proof/ledger/audit evidence for every release gate, including the charter's [G01-G24 obligations](openspec/MORIARTY-COMPLETION-PROGRAM.md#completion-and-broader-release-scope).
- [ ] Obtain final independent Fable 5.1 at medium effort and GPT-6 reviews of the actual accepted candidates. Recompute acceptance against the final deployed lineage.

Acceptance: a reproducible developer release whose advertised guarantees match retained evidence. [Detailed plan](openspec/changes/mc08-release-evidence-and-developer-flow/README.md).

## Execution order and stop conditions

```mermaid
flowchart TD
  RP1[RP01 semantic design map] -->|RP01-MC02 subset| MC02
  RP1 -->|RP01-MC03 subset| MC03
  MC01[MC01-ATOMIC acceptance] --> MC02
  MC01 --> MC03
  SRC[MC04 pinned source findings] --> F0[RP02 F0 feasibility and ledger-family decision]
  F0 --> F0A[F0a admitted component authorship]
  F0A --> F1[F1 positive and negative component probes]
  F1 --> MC03[MC03 / F2 native proof]
  ADM[RP03 separate candidate-bound campaign admission] --> F0A
  ADM --> F1
  ADM --> MC02[MC02 / I2 uncertified financial integration]
  ADM --> MC03
  ADM --> MC04[MC04 / F3 complete ledger wrapper]
  MC02 --> MC04
  MC03 --> MC04
  MC04 --> MC05[MC05 mandatory acceptance]
  MC05 --> MC06[MC06 private composition]
  MC06 --> MC07[MC07 complete finance]
  MC07 --> MC08[MC08 developer release]
```

The diagram shows the main path; the machine register carries every direct accepted-profile dependency and each campaign requires its own RP03 record, including MC05-MC08.

F0 establishes a reviewed proposed route without native execution, including finalizer arithmetic/constraint-fit estimates, go/no-go criteria and a ledger-family decision with explicit deployment-provenance limits. F0a cannot start until these decisions and a conservative reservation amendment are reviewed. F0a assigns MC04 component authorship and MC03 export work, with a recorded preparation allocation, conservative reservation amendment and current source reviews. F1 requires its own frozen implemented candidate, resource decision and current reviews before compilation or synthesis. P1 and P3 execute in the exact outer ledger circuit stack selected at F0, with native tests as reference fixtures only. Missing outer sources block F2/F3. P1 includes full ported IVC challenge/preparation/carried-accumulator agreement; its finite transcript vector alone is insufficient. All three F1 probe families must pass: transcript agreement, canonical export/import and constrained final pairing acceptance of an independently checked nontrivial valid fixture plus rejection of invalid direct-assignment mutations. No incomplete probe is waived. [Pinned independent fixture recipes](openspec/REPORT-RECONCILIATION-2026-09-07.md#independent-f1-fixture-sources) use native transcript tests, a separately admitted small non-loan Poseidon IVC derivative and a non-loan verifier-test accumulator. They are source recipes, not generated evidence. Missing codecs/finalizer and fixture generation require bounded admission; no F1 fixture may depend on the MC03 loan proof. MC03 supplies the terminal proof needed for F3, so full MC04 acceptance is not a prerequisite for MC03.

The native precondition is the reviewed RP01-MC03 fixed-statement subset and F0/F0a, successful F1, MC01-ATOMIC acceptance and campaign-specific RP03 admission. The full RP01 design map gates general successor profiles and later semantic extensions; it is not a prerequisite for the unchanged fixed-instance experiment. MC01-ATOMIC remains the existing source/evaluator comparison prerequisite. The register now represents each preparation and execution stage separately; null campaign IDs reject admission, and full package dependencies never block their own preparation stages. Each later extension has its own resource and review gate. Stop on the first failed required control, undefined essential interface or resource ceiling. Preserve failed evidence and change a justified hypothesis before another admitted attempt. A blocked Midnight interface does not authorize dropping PCD or changing the product target.

## Guarantees and research notes

Turing incompleteness and finite bounds make evaluation bounded; they do not by themselves prove financial correctness, practical proving cost or future settlement. Each theorem or proof must name its predicate, domain, version and assumptions. Ledger uniqueness, oracle truth, solvency, custody, witness availability and solver optimality remain separate questions.

The maintained LLM wiki notes live in [wiki/](wiki/index.md), alongside this roadmap in the repository. Start with [architecture](wiki/moriarty-architecture.md), [financial taxonomy](wiki/defiformal-taxonomy.md), [formal assurance](wiki/formal-assurance.md), [contradictions](wiki/contradictions.md), and the [research journal](wiki/research-journal.md). The [wiki log](wiki/log.md) records source and synthesis updates under [WIKI_SCHEMA.md](WIKI_SCHEMA.md).

The [combined report review and interactive graph](deliverables/moriarty-report-plan-review-2026-09-07/README.md) links the three supplied reports to this plan. [Raw snapshots](raw/reports/unified-2026-09-07/receipt.json) preserve original bytes and hashes. Report claims and opaque citations remain secondary evidence until their primary sources and relevant predicates are checked. [Footguns](docs/FOOTGUNS.md) preserve the design lessons that constrain future work.
