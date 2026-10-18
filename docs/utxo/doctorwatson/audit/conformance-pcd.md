# doctorwatson<UTXO> conformance and retained PCD source audit

Source reconciliation covers **47 clauses and 90 scenarios**: EU-CON 18/36, retained first-contract 11/16, and retained Midnight PCD 18/38. All 47 whole-clause acceptance declarations remain open; this audit establishes no product scenario observation. The [JSON audit](conformance-pcd.json) contains every original key, nullable ID, source pin, scenario name, exact missing predicate, consumer, owner and next check.

Implementation, recorded observations, semantic acceptance and byte currentness are independent. The built reader is a declarations/currentness consumer. Parent-reported inventory integrity and synthetic ordinary tests do not establish original source/ABI/UPLC/ledger/private predicates. Final reader checks belong to integration because its test source changed during this audit.

## Recorded evidence corrections

K7 resolves K6’s historical public verification-record availability gap with the public 3,926-byte record including its 488-byte VK. It supplies no new pairing rerun or consumer-checked VK-to-matrix correspondence. K8/K9 retain their local verifier and typed claim.issue scopes.

The later [K10 adoption](../../../../evidence/utxo/current-cardano-compiled-claim-wrapper-2026-10-16/root/K10-ACTUAL-ADOPTION.json) records one already compiled C2 mint wrapper accepting the genuine K6 proof and six semantic EvaluationFailure refusals. This supersedes earlier unrun-wrapper wording only at that finite scope. Guard labels are source-expected, without runtime guard instrumentation; costs are local ExBudget figures. Publication review is pending, target identity remains unknown, and spending/claim consumption, complete transaction fit, ledger acceptance and Preprod are open. These are attributed captured facts, not a new security judgment.

CS expected-outcome ADOPTION is design-only with source/native approval false. CS01 retains profile/5, CS10 retains Cardano Preview. No Rust/Core1 or Preprod relabeling satisfies them. The four Midnight PCD specs retain operation-key verification, constrained genesis, immutable authority, head discipline, bounded certificates and their own ledger-generation scope. Their original IDs remain null.

## Concrete implementation boundaries

The specified experiments/moriarty-eutxo/src/conformance/ and tests/conformance/ directories are absent. The built metadata reader is partial support for EU-CON-001/004/010/017, with independently authored reporting fixtures relevant only to reporting. It does not perform full source/ABI/generated-spending-UPLC/ledger differential replay.

The legacy runNativeCaller branch explicitly returns PRODUCTION_FINANCIAL_CONSUMER_INCOMPLETE before using an admitted production sibling because current policy/script/budget/minimum-ADA binding is required. Proposed native-production.mjs, native-chain-observer.mjs and their named integration tests are absent in this checkout. The inspected race return remains ok:false, confirmed:false and ledgerExecution:false even for RACE_OBSERVED. These are bounded source observations, not proof that no later external implementation exists.

K/finite Quint/Lean-Clean/handwritten-circuit correspondence must use the same complete state, effects and work with explicit profile/field adapters and premises. Neither this audit, the metadata reader nor a sampled DoctorWatson workflow model establishes it. The original Lender7→107 differential case and Witness5 preservation case remain unrun here.

## Clause and scenario ledger

| Original clause | Implementation disposition | Original scenario names | Next actual consumer |
|---|---|---|---|
| EU-CON-001 | partial; acceptance open | EU-CON-001 positive; EU-CON-001 negative | experiments/moriarty-eutxo/src/conformance/freeze-inputs.mjs -> experiments/moriarty-eutxo/tests/conformance/freeze-inputs.test.mjs (specified paths absent) |
| EU-CON-002 | specified-only; acceptance open | EU-CON-002 positive; EU-CON-002 negative | experiments/moriarty-eutxo/src/conformance/full-envelope.mjs -> experiments/moriarty-eutxo/tests/conformance/full-envelope.test.mjs (specified paths absent) |
| EU-CON-003 | specified-only; acceptance open | EU-CON-003 positive; EU-CON-003 negative | experiments/moriarty-eutxo/src/conformance/failure-layers.mjs -> experiments/moriarty-eutxo/tests/conformance/failure-layers.test.mjs (specified paths absent) |
| EU-CON-004 | partial; acceptance open | EU-CON-004 positive; EU-CON-004 negative | experiments/moriarty-eutxo/src/conformance/inventory.mjs -> experiments/moriarty-eutxo/tests/conformance/inventory.test.mjs (specified paths absent) |
| EU-CON-005 | partial; acceptance open | EU-CON-005 positive; EU-CON-005 negative | experiments/moriarty-eutxo/src/conformance/oracle-independence.mjs -> experiments/moriarty-eutxo/tests/conformance/oracle-independence.test.mjs (specified paths absent) |
| EU-CON-006 | specified-only; acceptance open | EU-CON-006 positive; EU-CON-006 negative | experiments/moriarty-eutxo/src/conformance/differential.mjs -> experiments/moriarty-eutxo/tests/conformance/differential.test.mjs (specified paths absent) |
| EU-CON-007 | specified-only; acceptance open | EU-CON-007 positive; EU-CON-007 negative | experiments/moriarty-eutxo/src/conformance/properties.mjs -> experiments/moriarty-eutxo/tests/conformance/properties.test.mjs (specified paths absent) |
| EU-CON-008 | specified-only; acceptance open | EU-CON-008 positive; EU-CON-008 negative | experiments/moriarty-eutxo/src/conformance/metamorphic.mjs -> experiments/moriarty-eutxo/tests/conformance/metamorphic.test.mjs (specified paths absent) |
| EU-CON-009 | specified-only; acceptance open | EU-CON-009 positive; EU-CON-009 negative | experiments/moriarty-eutxo/src/conformance/native-discrimination.mjs -> experiments/moriarty-eutxo/tests/conformance/native-discrimination.test.mjs (specified paths absent) |
| EU-CON-010 | partial; acceptance open | EU-CON-010 positive; EU-CON-010 negative | experiments/moriarty-eutxo/src/conformance/review-gates.mjs -> experiments/moriarty-eutxo/tests/conformance/review-gates.test.mjs (specified paths absent) |
| EU-CON-011 | specified-only; acceptance open | EU-CON-011 positive; EU-CON-011 negative | experiments/moriarty-eutxo/src/conformance/private-refusal.mjs -> experiments/moriarty-eutxo/tests/conformance/private-refusal.test.mjs (specified paths absent) |
| EU-CON-012 | partial; acceptance open | EU-CON-012 positive; EU-CON-012 negative | experiments/moriarty-eutxo/src/conformance/proof-seam.mjs -> experiments/moriarty-eutxo/tests/conformance/proof-seam.test.mjs (specified paths absent) |
| EU-CON-013 | specified-only; acceptance open | EU-CON-013 positive; EU-CON-013 negative | experiments/moriarty-eutxo/src/conformance/proof-binding.mjs -> experiments/moriarty-eutxo/tests/conformance/proof-binding.test.mjs (specified paths absent) |
| EU-CON-014 | partial; acceptance open | EU-CON-014 positive; EU-CON-014 negative | experiments/moriarty-eutxo/src/conformance/proof-negatives.mjs -> experiments/moriarty-eutxo/tests/conformance/proof-negatives.test.mjs (specified paths absent) |
| EU-CON-015 | specified-only; acceptance open | EU-CON-015 positive; EU-CON-015 negative | experiments/moriarty-eutxo/src/conformance/pcd-gates.mjs -> experiments/moriarty-eutxo/tests/conformance/pcd-gates.test.mjs (specified paths absent) |
| EU-CON-016 | partial; acceptance open | EU-CON-016 positive; EU-CON-016 negative | experiments/moriarty-eutxo/src/conformance/feasibility.mjs -> experiments/moriarty-eutxo/tests/conformance/feasibility.test.mjs (specified paths absent) |
| EU-CON-017 | partial; acceptance open | EU-CON-017 positive; EU-CON-017 negative | experiments/moriarty-eutxo/src/conformance/assessment-graph.mjs -> experiments/moriarty-eutxo/tests/conformance/assessment-graph.test.mjs (specified paths absent) |
| EU-CON-018 | partial; acceptance open | EU-CON-018 positive; EU-CON-018 negative | experiments/moriarty-eutxo/src/conformance/amendments.mjs -> experiments/moriarty-eutxo/tests/conformance/amendments.test.mjs (specified paths absent) |
| CS01 | partial-source-inspected; acceptance open | Calibrated sibling; Forgery or stale source | compileGuardSibling / compileCustody -> runNativeCaller; planned native-production.test.mjs O01–O08/O38–O42 |
| CS02 | partial-source-inspected; acceptance open | One identity everywhere | runNativeCaller -> planned bindProductionContext/assertProductionBody; O01–O18/O25–O27 |
| CS03 | partial-source-inspected; acceptance open | Actual funding dependencies; Forbidden seed/policy | buildApprovedFunding -> runNativeCaller; O19–O27/O43/O45 |
| CS04 | partial-source-inspected; acceptance open | Valid bounded body; Measurement refusal | measureProductionBody (planned) -> native-runtime acquireBodyEvaluation/compareBudget -> runNativeCaller; O09–O18 |
| CS05 | partial-source-inspected; acceptance open | Original windows | assembleConsumedFinancial/runNativeCaller/native-financial -> planned cardano-t1.test.mjs O28–O42 and retained68 quantity controls |
| CS06 | partial-source-inspected; acceptance open | Complete effect; Wrong effects | runNativeCaller + planned native-chain-observer observeChainInclusion/judgeOperationalConfirmation; O43–O45/O49 |
| CS07 | partial-source-inspected; acceptance open | Missing outcome | native-caller runOwnedRace -> planned chain observer; O46–O50/L01/P01 |
| CS08 | partial-source-inspected; acceptance open | Changed source or missing auditor | runNativeCaller/loadAuthority + planned cardano-t1 CLI/auditBinding admission; O01–O08 |
| CS09 | partial-source-inspected; acceptance open | Actual local exit | cardano-t1 local-devnet (planned) -> runNativeCaller -> I1/F18 observer; L01–L03 |
| CS10 | partial-source-inspected; acceptance open | Preview demonstration; Preview race and result | cardano-t1 cardano-preview (planned) -> preview-execution-dependency.json -> same reviewed caller; P01–P02 |
| CS11 | partial-source-inspected; acceptance open | Four-pass failure and retained limits | runNativeCaller/native-runtime bounded fee/query loops -> cardano-t1 public result; O18/O42 and four-pass refusal |
| Bounded native certificates | specified-only-owner-unassessed; acceptance open | Accepted certificate; Unsound certificate | MC03 / SP04.3 E3 -> evidence/moriarty-completion-program-2026-09-07/MC03/pcd-e3/ |
| Off-ledger segment certificate | specified-only-owner-unassessed; acceptance open | Verified segment; Substituted segment evidence | MC03 / SP06.2–SP06.3 E5 -> evidence/moriarty-completion-program-2026-09-07/MC03/pcd-e5/ |
| Recursion dependency tracking and stop | specified-only-owner-unassessed; acceptance open | Tracker satisfied; Unsatisfied tracker item | MC03 / SP04.3–SP06.3 admission -> openspec/sprints/pcd-integration.json dependencyTracker |
| Ledger-atomic split and join | specified-only-owner-unassessed; acceptance open | Split then join; Composition attack | MC06 / SP10.3 Split/Join -> evidence/moriarty-completion-program-2026-09-07/MC06/pcd-composition/ |
| Cross-contract release with reclaim | specified-only-owner-unassessed; acceptance open | Recovered unclaimed release; Unsafe release or reclaim | MC06 / SP10.3 E4 Release/JoinFrom/Reclaim -> MC06/pcd-composition/ |
| Recipient-keyed successor handoff | specified-only-owner-unassessed; acceptance open | Independent successor; Tampered opening | MC06 / SP10.2 successor -> evidence/moriarty-completion-program-2026-09-07/MC06/pcd-handoff/ |
| Composition operators under head discipline | specified-only-owner-unassessed; acceptance open | Mapped operators; Operator without a rule | MC06 / SP10.3 operator mapping -> Step/Split/Join/claimed call |
| Declared ledger verification seam | specified-only-owner-unassessed; acceptance open | Ledger-verified call; Substituted verification | MC04 / SP01.4 -> evidence/moriarty-completion-program-2026-09-07/MC04/pcd-seam/ |
| Fused step relation per entry point | specified-only-owner-unassessed; acceptance open | Fused proof verifies; Unbound field or separate claim proofs; Fit failure | MC04 / SP09.1 E2 -> evidence/moriarty-completion-program-2026-09-07/MC04/pcd-e2/ |
| Head read-then-write discipline | specified-only-owner-unassessed; acceptance open | Conflicting calls; Blind write; E1 fails | MC04 / SP09.1 generated-ZKIR checker + E1 Preview -> MC04/pcd-e1/ |
| Immutable authority deployment audit | specified-only-owner-unassessed; acceptance open | Audited deployment; Weak or altered deployment | MC04 / SP09.1 deploy audit -> evidence/moriarty-completion-program-2026-09-07/MC04/pcd-deploy-audit/ |
| Measured bounds frozen into the program digest | specified-only-owner-unassessed; acceptance open | Frozen profile; Unmeasured bound | MC01 / SP11.1 -> bounds.json and MC04/pcd-bounds/ |
| Claim discharge map | specified-only-owner-unassessed; acceptance open | Induction conditions hold; Induction condition missing | MC05 / SP09.1–SP09.3 mandatory acceptance -> native-path-freeze claim consumers |
| Constrained genesis and termination | specified-only-owner-unassessed; acceptance open | Genesis and terminal step; Invalid lifecycle action | MC05 / SP09.2–SP09.3 -> evidence/moriarty-completion-program-2026-09-07/MC05/pcd-lifecycle/ |
| Intent digest v2 and program digest | specified-only-owner-unassessed; acceptance open | Canonical vectors; Altered or replayed intent | MC05 / SP09.3 -> evidence/moriarty-completion-program-2026-09-07/MC05/pcd-digests/ |
| Observation freshness at application | specified-only-owner-unassessed; acceptance open | Fresh observation; Stale observation | MC05 / SP09.2–SP09.3 step relation + ledger application -> MC05/pcd-lifecycle/ |
| Forward-declared migration replaces in-place revocation | specified-only-owner-unassessed; acceptance open | Declared migration; Unauthorized or unsafe migration | MC05 / SP09.3 E4 -> evidence/moriarty-completion-program-2026-09-07/MC05/pcd-e4/ |
| Governed deploy-time property certificate | specified-only-owner-unassessed; acceptance open | Checked certificate; Hash without a check | MC05 deploy audit -> actual universal ContractProperty certificate checker; future ledger10 Initialize option |

## Scope and read limits

Retain ACTUS 277 fixtures/fields, 18 executable types, 32 taxonomy dispositions and DS01–07; DeFi 72 rows, DA01–24, supplements, three held-outs, eight intents and eight regression classes. Sequential, disjoint parallel, shared-state interleaving, atomic synchronization, asynchronous messaging, Pending, real recursion, private handoff, bounded multi-parent history and universal certificates remain independent obligations. A finite public proof does not waive them.

No whole-language completion percentage is inferred. All 90 original scenarios have source-bound assessments; none is marked observed by this audit. Native execution is blocked by current zero authority. Source preparation can proceed independently, with cryptographic/security contracts, test adequacy and judgments retained by the designated Opus owner. Public developer use remains permissionless.

- All six assigned normative specs read; exact baseline clauses and ordered scenario identities preserved. Binding conformance design/tasks, CS revision2 spec addendum and ADOPTION, PCD design/tasks/proposal and selected integration/tracker spans inspected. CS long design/task prose and large PCD report/measurement files were only partly displayed; no exhaustive semantic review of linked closure.
- K6/K8/K9/K10 public summaries and K7/K10 root adoptions inspected as captured declarations. Underlying proof/circuit/validator payloads, raw execution logs and full independent reviews were not re-audited or executed; substantive adequacy remains owner-unassessed.
- All three Astra annexes inspected as advisory source context; their read limits remain. No advisory absence claim supersedes later exact adoption.
- Forbidden external validation tree and historical absolute pointers not traversed. No private material, SDK/compiler/import/proof/VM/model/wallet/provider/network/Foreman/ledger execution, new agents or Git mutation.
- Ordinary reader source was inspected in bounded spans; parent supplied build/test success. No subject test or reader rerun performed by this shard. Source/hash checking below establishes membership/byte identity only.
- No full ACTUS/DeFi fixture audit, K/Quint/Lean-Clean/circuit correspondence audit, or all-roadmap acceptance audit is claimed by this shard; those families and parent exits remain required.

Source membership verification parses only these six pinned local normative files and compares exact ordered requirement/scenario names with the baseline and this output. Evidence hashes bind public local records; they do not authenticate provider sessions or prove substantive adequacy. No external historical absolute pointer was followed.
