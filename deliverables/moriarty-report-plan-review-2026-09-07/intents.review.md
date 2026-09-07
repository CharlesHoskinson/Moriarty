# Intents report review

Reviewed the complete 2,517-line report at `raw/reports/unified-2026-09-07/intents.md`. The graph records what the report says, including proposals and cited claims. It does not establish that those claims are true or implemented. The report has unresolved citation tokens such as `turn20view3`, not resolvable primary-source URLs. Its prototype download is a `sandbox:` link; the claimed two passing tests and prototype ZIP hash were not reproduced in this review. Token usage was unavailable; graph schema zeros are placeholders.

The report supports the project's central direction: a bounded financial language with separate signed authority, proposed execution, checked effects, obligations and evidence. It does not justify accepting the present atomic profile as the final intents language. The remaining packages are broadly suitable, but several report requirements need explicit acceptance criteria and earlier design validation. This is a Midnight-centric language, not the report’s backend-neutral IKL product proposal.

## Decisions that must override this report

| Report recommendation | Source lines | Moriarty disposition |
|---|---|---|
| Transparent deterministic checking first; introduce ZK only when justified; defer Compact/ZKIR dependencies | 2012-2016 | Superseded by the user's mandatory proof-carrying transaction requirement and Midnight target. Local checking remains an oracle/tooling layer, not a production acceptance substitute. |
| Public intent semantics first; defer generalized private predicates | 1922-1954, 2227-2246 | Preserve leakage limits, but do not remove the user's private witness handoff requirement. Private handoff does not by itself imply hidden solver preferences or globally private intent solving. |
| Implement NEAR and EVM first, plus ERC-7540 | 2498-2509 | Superseded by the user’s explicit clarification: “This is a midnight centric language.” NEAR/EVM are comparative sources only, with no adapter or deployment deliverables. Midnight Compact, native proof interfaces, private state and ledger semantics constrain the language design; Preview remains the public test target. |
| The fixed 30/90/180-day implementation schedule | 2154-2225 | Treat as a report proposal, not an estimated delivery commitment or evidence of feasibility. Reorder around proof-to-ledger compatibility and semantic coverage. |

## Requirements and package crosswalk

| Requirement from report | Source lines | Existing plan/code evidence | Remaining work |
|---|---|---|---|
| Intent, authority, solver query, plan, execution and receipt remain distinct | 9-45, 995-1190 | MC01 `runtime-types.ts:18-25` separates signed outcome/exact plan and result; MC05 requires real refinement | Specify user-authored intent syntax and canonical Solver Query/Plan boundaries, including whether preferences are advisory. Current `types.ts:25` and `spec/grammar.ebnf` expose agreement/actions, not source-level intent, workflow or preference declarations. |
| Gross authority, permitted recipients and net goals precede solver ranking | 435-509, 1861-1870 | MC01 `evaluate.ts:242-247` checks recipient/call restrictions; MC05 spec requires gross and net checks and alternate routes | Require accepted real alternative plans for the same signed intent; reject plans that hit the goal by over-debiting, callback effects or altered assumptions. MC01 simulation is not an acceptance backend. |
| Debt creation is an independently authorized effect | 1570-1607, 2412-2440 | MC01 `spec/semantics.md:575-578` explicitly says nominal obligations are outside outcome debit caps | Add versioned debt/liability authority with issuer, debtor, creditor, amount and lifecycle scope before accepting refinance. MC05/MC07 must reject extra debt even when balances and net-credit goals pass. |
| Domain, issuer, asset reference and claim kind distinguish assets | 265-327 | MC01 nominal amount units and textual settlement bindings; `runtime-types.ts:20` uses asset strings | Document authenticated identity interpretation in MC02/MC04 and type/lowering extension in MC01/MC07. Reject same ticker on different domains, wrong issuer and claim-for-token substitution. Plain strings are not evidence of identity or equivalence. |
| Lifecycle requirements apply to observable traces, with persistent obligations | 511-552, 595-690, 883-934 | MC06 carries obligations through composition; MC07 requires pending redemption | Define observable prefix/event phases, request/claim evidence, cancellation/lock boundaries and late fulfillment/refund race arbitration. MC01 explicitly rejects Pending and partial settlement. A finite lifetime counter is insufficient evidence of workflow semantics. |
| Capabilities are atomically residualized across partial fills and recurring periods | 475-509, 1507-1568, 1653-1694 | MC04 requires durable replay/consumption; MC06 requires residual authority | Add aggregate budget, per-period ticket and revocation semantics, including concurrent fills and duplicate-period attempts. A fresh nonce must not replenish residual authority. |
| Observation provenance contains source, domain, anchor, time and finality | 732-759 | MC01 `runtime-types.ts:13` has provider/value/evidenceDigest; `typed-schemas.md:469` calls evidence digest opaque | Version authenticated evidence interpretation and freshness/finality policy; MC05 must bind and enforce it, MC04 must reconcile actual finality. A hash or authentic signature does not establish freshness or external truth. |
| Compiler lowering preserves trace inclusion and positive feasibility | 692-730, 2063-2093 | MC01 design already requires positive feasibility; MC04 spec requires mechanical correspondence | Preserve positive witnesses for each supported financial behavior, not merely rejection tests. Extend correspondence whenever domain, effects, obligations or authority change. |
| Typed adapters cannot hide unbounded semantic computation | 554-593, 965-993 | MC01 source is finite and rejects calls; MC04 describes proof/ledger adapter | Every accepted foreign effect must have a bounded interpreted relation or a verified proof relation with exact version/code commitments. Do not transplant the report's unsafe reapproval escape hatch into the required verified language. |
| Signing display is a semantic correspondence obligation | 1190-1236, 1876-1920 | MC08 requires exact signed authority in developer flow | Test render(parse(canonical bytes)), all loss/debt/lock/recovery/assumption fields, unknown semantic extensions and display/signature mismatch. Author the display schema before freezing new authority fields. |
| Proof classes have precise statements and boundaries | 1956-2016 | MC03 native proof, MC04 correspondence, MC05 mandatory claims | Map intent hash, plan hash, adapter/version, anchors, public observations, complete effects and result status into actual proof public inputs or proven commitments. Fixed-instance proof evidence cannot establish a general intent-refinement relation. |
| All eight semantic stress cases and frozen held-outs challenge kernel sufficiency | 1466-1853, 2223, 2442-2474 | MC07 requires NAM19, refinance and pending redemption plus all ACTUS/DeFi rows | Register exact-output/minimum-output exchange, partial batch, refinance, pending redemption, recurring permission, delegated rebalance, contingent claim and recovery as explicit report coverage. Classify unsupported cases before kernel extension, retaining the frozen denominator and independent oracle. |
| Semantic, adapter and advisory extensions have different authority | 2248-2284 | MC05-07 lineage rules require versioning and requalification | Specify rejection of unknown semantic versions, no silent authority widening, advisory noninterference and adapter substitution tests. |

These are implementation and acceptance gaps, not claims that every item is absent from earlier design prose. Existing MC01 design already states positive feasibility, MC04 already owns replay/correspondence, and MC05 already owns mandatory intent refinement. The needed correction is an explicit report-to-predicate crosswalk and executable evidence.

## Sequencing recommendations

1. Use Midnight as the semantic implementation target; keep the other networks as comparative research only. Before declaring the authoring language final, freeze a semantic challenge matrix from all eight worked cases and the difficult DeFi/ACTUS cases. Mark which can be expressed now, which need finite extensions and which depend on external capabilities. Design validation can happen before the complete MC07 conformance campaign.
2. Resolve the exact complete native-verifier-to-Preview acceptance interface while planning native proof work. A native fixed-loan proof and a ledger-valid Compact circuit do not alone establish their connection.
3. Co-design versioned intent source syntax, debt authority, temporal obligations, observation policy, canonical signing and mandatory claim statements. Preserve the present atomic profile as scoped experimental evidence.
4. Make MC05 enforce those semantics in the acceptance path; make MC06 test residual and private composition. MC07 then checks financial behavior at its full denominator and requalifies affected proof/correspondence packages.
5. MC08 should test usable signing and recovery, but its display schema must already inform the fields signed in earlier packages. This is an interface dependency, not a request to postpone all UI work.

## Claims requiring caution

- The theorem ledger explicitly labels results as targets or conditional claims; its Lean residual theorem was not run (936-963). Do not import those rows as completed formal evidence.
- Report citation tokens are disconnected from a source list. Named standards and project behavior need pinned primary sources before implementation decisions. This review has not independently refreshed those external claims.
- Authority attenuation is described as not changing risk (205-217). Subset preservation limits executable authority; it does not by itself preserve financial feasibility, hedging or every economic risk metric. The final semantics should claim the precise subset property.
- The report's `always` requirements and refinance example need a defined observation boundary (595-690, 1570-1607). Atomic commitment, intermediate callback-visible states and cross-domain prefixes are different models.
- The example named exact-output exchange uses a minimum-output inequality (1466-1505). Specify whether exact means equality or a minimum receive requirement; do not let a label change the predicate.
- Concrete bounded verification is not automatic solving (965-993). Total functions and finite inputs establish decidability only for the selected finite interpretation and environmental evidence policy; they do not establish solver availability or settlement liveness.
- The arbitrary adapter proposal requires stronger closure for mandatory PCD. Merely declaring an effect summary is insufficient; implementation correspondence and hidden-effect exclusion are separate obligations (554-593, 2063-2093).
- The withdrawn DeFi four-primitive basis and keyword certificate criticism are report claims with repo-path citations (2095-2107). Retain the actual source evidence; do not generalize that criticism into discarding the DeFi behavior corpus.

## Full section coverage

Every section below was read. Ranges are inclusive and end immediately before the next heading; parent section rows cover their introductory text. Concepts across these sections appear in `intents.graph.json`.

| Heading | Lines |
|---|---|
| Designing an Intents-First Language for Composable DeFi | 1-2 |
| Executive design decision | 3-103 |
| Evidence from NEAR, CAKE, Ethereum standards, and prior intent systems | 104-107 |
| What NEAR Intents actually contributes | 108-153 |
| What CAKE contributes | 154-174 |
| What the Ethereum standards actually standardize | 175-220 |
| Lessons from CoW, UniswapX, Anoma, Essential, and Marlowe | 221-236 |
| The recommended language and semantic kernel | 237-240 |
| Semantic objects | 241-264 |
| Core types | 265-328 |
| Surface syntax | 329-398 |
| A basic exchange | 399-434 |
| Hard conditions and soft ranking | 435-474 |
| Capabilities as affine resources | 475-510 |
| Temporal intents and obligations | 511-553 |
| Foreign calls | 554-594 |
| Formal semantics, composition, and theorem program | 595-653 |
| Verification judgment | 654-691 |
| Semantic refinement | 692-731 |
| Assumptions and observations | 732-760 |
| Asset conservation versus solvency | 761-808 |
| Composition | 809-882 |
| Asynchronous composition | 883-935 |
| The theorem ledger | 936-964 |
| Decidable and solvable fragments | 965-994 |
| Compiler, IR, runtime, signing, and backend mappings | 995-1014 |
| Canonical Intent IR | 1015-1091 |
| Solver Query | 1092-1118 |
| Plan IR | 1119-1150 |
| Receipt | 1151-1189 |
| Wallet signing | 1190-1237 |
| NEAR lowering | 1238-1279 |
| EVM lowering | 1280-1312 |
| Extended-UTxO portability | 1313-1338 |
| Compiler pipeline | 1339-1382 |
| Prototype | 1383-1461 |
| Worked financial programs and adversarial cases | 1462-1465 |
| Exact-output exchange | 1466-1506 |
| Partial fills and a matched batch | 1507-1569 |
| Atomic lending refinance | 1570-1608 |
| Asynchronous vault redemption | 1609-1652 |
| Recurring payments | 1653-1695 |
| Portfolio rebalancing with delegated agent | 1696-1743 |
| Contingent claim | 1744-1782 |
| Cross-domain payment with recovery | 1783-1840 |
| What these examples show | 1841-1854 |
| Security, privacy, validation, and implementation plan | 1855-1858 |
| Core attack classes | 1859-1875 |
| Clear signing as a proof obligation | 1876-1921 |
| Privacy | 1922-1955 |
| Proof-carrying plans | 1956-2017 |
| Conformance testing | 2018-2062 |
| Contract-to-model fidelity | 2063-2094 |
| Relationship to the DeFi kernel | 2095-2153 |
| Implementation milestones | 2154-2226 |
| First release boundaries | 2227-2247 |
| Extension governance | 2248-2285 |
| Final answers | 2286-2517 |

Graph validation: 141 uniquely identified nodes, 421 provenance-bearing edges, three hyperedges, and no dangling endpoints. External citation claims remain report-sourced; no primary-source URL was invented.
