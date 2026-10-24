# Sherlock completion implementation plan — 2026-10-17

Superseded 2026-10-24: review routing, adoption votes, consumed-attempt and no-retry rules, permanent lanes and campaign queues in this file are retired. The process is the factory loop in [AGENTS.md](../AGENTS.md); technical descriptions below stand as history of the legacy tooling.

Current execution follows [revision03](sherlock/design.md), the [OpenSpec/EARS adoption](../openspec/changes/sherlock-completion-convergence/ADOPTION.md), its [finite B00–B12 batches](../openspec/changes/sherlock-completion-convergence/batch-plan.json), and the [implementation loop](sherlock/implementation-loop.md). All25 SCON features,68 scenarios and35 original UIREQ remain required. The goal tool recorded the newly armed full implementation goal at epoch1792260805 after the ordinary entry conditions closed; this dated event does not attest future worker activity.

PR168 merged the composed ledger source into `sherlock` at `01128456`. PR169 merged the reconciled design, ordinary QA, finite-model results, documentation and knowledge graphs into that branch at `3d3401ef`. Its recorded241 Python tests, eight Rust stream tests/216 independent normal runs, composed333 capture, and44 finite Quint tests/five runs retain their exact source and coverage. They do not close native/target or full-language acceptance. The next compiler increment is the real source-to-B preparation/admission path described in [FIRST-COMPILER-BATCH](../openspec/changes/sherlock-completion-convergence/FIRST-COMPILER-BATCH.md); The first production-library increment has compiled, passed seven positive tests and matched independently specified program tables for two sources; CLI execution, mandatory source-review corrections, full artifact/runtime comparisons and downstream acceptance remain open.

The audit proposal below retains its original cut and sprint rationale. Its baseline, then-pending PR and cleared-goal statements describe that earlier cut. They do not override the adopted design, current batch contracts or the later results above. The [pre-review implementation plan](audits/2026-10-17/sherlock-plan-history/IMPLEMENTATION-PLAN-BEFORE-REVIEW.md) is preserved separately.

## Historical audit proposal

Baseline: clean public HEAD `628615a94d8a74837a1af510293927bb74d5c2b5`. Root supplies merged Sherlock `e34c34e0` and unmerged draft PR168; PR168 is uninspected here. Proposed sprint ordering stays inside U0–U7, with no new scheduler/authority/percentage. The old goal is CLEARED; older active-goal wording is historical.

## Present capability and first blocked boundaries

Current-A host/image/Quint/pure-Lean diagnostic records match eight programs, 216 prefixes and 5,355 projected records. Later direct-rustc Core evidence records 189 library tests. K has one N01/fuel1 engine case matching an independent two-record projection; 215 fixed inputs remain unobserved. Supplied-K comparisons alone are not K execution. These are historical exact-source captures, not runs for this audit.

Full39 conditionally adopts 39 Rust semantic events with a separate 42-row K reconciliation, not generic emission/P1–P6 closure. PR161 records six SDK-free host tests (3 A102 + 3 A80), independent comparison of 250 carriers/210 labels, and an uncompiled/unexecuted native emitter. Two context limbs remain UNBOUND. LiteralComplete names two-step composition, not native/general completion. Generic dispatch, StepInv, F4/G4/C4, full-state Clean, native compilation/proof/independent verification and stagenet settlement remain open.

## Corrections to current summaries

1. Update current ROADMAP summaries that call all A102/component/codec tests UNRUN. Retain the original frozen source packages unchanged; append a dated pointer to the precise later capture. Do not label all 16 original machine tests passed from the six host tests.
2. Replace blanket “K captures/execution remain missing” in current summaries with the one-case/215-unobserved distinction. Preserve the historical paragraphs as historical, with a clear latest-status pointer.
3. ROADMAP says Lean is not a roadmap workstream while UIREQ-026 and AGENTS require the existing private Lean/Clean stage. Clarify: Lean is not a public developer/compiler/proof-production dependency; private project semantic/Clean implementation and correspondence remain mandatory.
4. B-T14/spec/traceability still select Muse. Current AGENTS selects non-author Opus 5.5 high, fallback after observed outage. Correct live guidance; retain historical identities/receipts. Five requested Sol planning audits do not substitute security/result approval.
5. At the original model-only cut B was specified-only. Retain full B acceptance open; later [four owned-observation](results/sherlock-b-complete-observation-20261017/README.md) and [nine complete-comparison](results/sherlock-b-trace-comparison-20261017/README.md) tests have their own exact, narrower source/result scope. A interpreter/codec progress is not B acceptance. Distinguish overlapping captured A observations from unexecuted B acceptance scenarios.
6. Current assignment/plan wording about an active old goal is superseded by the user's CLEARED steering. Preserve source history; do not resume historical SP01 campaign queues. Status CLI reports historical loan-swap blockers, not today's acceptance or priority.
7. First-literal stagenet does not close UIREQ-023/U2: retain supported compiler, two contrasting positive programs, complete readback and grammar coverage. A refusal is not a second positive.

## Evidence contract for every proposed sprint

Each predicate is a callable contract to bind to a real consumer, not an existing-command claim. Reports require case/member IDs, all required fields, expected/actual values, profile/bounds, source/import/toolchain hashes, command/exit, oracle provenance, projection, evidence class and first blocked boundary. Missing/unsupported/unrun means non-pass. Independent expectations predate execution and cannot import the implementation. Historical receipts keep their exact scope.

Separate source, execution, comparison, theorem, native and target status. Preserve actual K rules/cells; Quint domain/seed/bounds; Lean observations; Clean witness/constraints; handwritten circuit; exporter/native consumer. Record shared dependencies. Normal agreement is not soundness/completeness/refinement. Existing security cases/theorems/judgments stay Opus-owned. Native authority zero, C4, caps/counters/failures, private boundaries and permissionless public use remain unchanged.

## Dependency-ordered finishing sprints

### R0 — Small immediate ordinary improvement: truthful complete-observation reporting

Owners I (current docs), non-author O (required normal fields/members), T (existing report consumer). Proposal only: one small PR changes one existing SDK-free report consumer, one required-member manifest and normal regressions; no control/native changes.

EARS R0.F1: WHEN a current report cites a historical observation, it SHALL bind its exact source/scope and distinguish later results from frozen source-time UNRUN labels.
EARS R0.F2: WHEN expected and supplied observations omit a required normal outcome field or member, the report SHALL identify the omission rather than infer agreement from their shared absence.
EARS R0.F3: WHEN an optional measurement is absent, the report SHALL retain “unmeasured” and its reason rather than numeric zero.

Predicate `report_complete(required_members, required_fields, expected, actual, receipts)`: exact required membership; required-field presence at every applicable record; ordered arrays preserved; absent distinct from present-zero; exact values on the normal domain; no pass inherited from an unrun layer. Use complete pure and supplied financial normal snapshots plus ordinary omission regressions. Verify docs/JSON/links and only inspected SDK-free report checks. Current offline.py checks containers, not required state/account members. Opportunity only; no reproduced bug. Exit: one small reviewed PR and reports, independent of native work.

### R1 — Complete the literal transition-to-consumer source closure

Owners I/E (actual ledger-9 bridge), O (independent init/step/final expectation), Opus (security). Bind A102/A80, public context/program/init/mid/final/outcome positions and real consumer. Distinguish supplied expectations from emitted constrained values; retain full39/L04/L05.

EARS R1.F1: WHEN the selected literal program is prepared, its report SHALL identify every consumed statement position, relation and still-unbound context assumption.
Predicate `literal_source_closure(candidate, inventory)`: every required source/consumer edge exists and every required position has an explicit disposition; no UNBOUND limb becomes “bound” by naming it. Exit: exact integrated source and fresh applicable full-candidate reviews. Source closure is not execution acceptance. Draft PR168 receives its own inspection/review and cannot be silently counted merged.

### R2 — Separately admitted native build, proof and independent verification

Serial after R1 and unchanged native/build admission. Owners E and independent result reviewer; Opus security. Authorized prerequisite diagnosis/preparation may overlap without SDK/native launch.

EARS R2.F1: WHEN native evidence is claimed, the actual selected source/toolchain SHALL compile and produce a proof verified independently against the exact statement/relation/key identity.
Predicate `native_literal_observed(receipt_set)`: actual build exit, constraint/native execution, produced proof, independent verification and complete normal projection all observed for the exact candidate, including explicit measured/unmeasured resources. No cached old descriptor/key inherits identity. Exit: raw results and fresh actual-result review under unchanged limits. Currently blocked by missing evidence/admission; no run proposed here.

### R3 — First real Midnight stagenet transaction

Serial after R2, qualified stagenet configuration/owner environment and admitted submission. Owners E/O. Configuration source preparation may overlap. No invented target tuple or transferred Preview/Fenrir identity.

EARS R3.F1: WHEN the scoped literal transaction is reported settled, evidence SHALL bind actual submission, inclusion/finality position and complete supported state/effect readback to its independent expectation.
Predicate `stagenet_literal_settled(target, tx, readback)`: actual qualified target tuple, proof verification, deploy/call, transaction reference, fees and full supported state/effects observed at the readback position. Explicit empty effect sets are observations, not omitted data. Exit: reviewed scoped first-transition demonstration. This closes no full interpreter/U2/release exit.

### R4 — General full39 handwritten machine realization

Owners I for ABI/dispatch; disjoint representation D, frames F, arithmetic A and helper P owners; circuit C integrates. Keep adopted full39 conditions F1–F7, P1–P6, P3 errata, MD2 selection, F4, G4, StepInv/chaining and all applicable owner conditions explicit. Dependent circuit changes wait on C4's admitted A102 compilation; ordinary source inventories and independent normal expectations can proceed now.

EARS R4.F1: WHEN any admitted A rule fires, its actual reusable emitter SHALL produce the complete specified successor and preserved state with its exact named charge/publication behavior.
Predicate `full39_step_coverage(rule_catalog, independent_cases, actual)`: each of 39 events has implementation/consumer, normal exercise or explicit unreachable-domain justification, complete pre/post comparison and preserved-state accounting; startup/no-event APIs and 42 K rows are separately reconciled. Add recursive calls/returns, all frames, matching, equality, scalar/list/tree, argument accumulation and failure/publication prefixes. Applicable security/native/theorem gates remain separate. Exit is general A machine evidence, not B acceptance or full release.

### R5 — B profile, general source, helpers and representations

Owners I/R (profiles/interfaces), P/D/F/A/W/E (disjoint components), O (D01–D36 expectations). Reuse host/Lean foundations. Preserve complete B-T01–10 scope beyond A fixtures. Resolve Phase 0 error/charge/allocation/order/admission rows before B freeze.

EARS R5.F1: WHEN a supported new program is admitted, one unchanged interpreter/lowering SHALL handle its ordered declarations, recursive references and complete states without per-fixture branches.
EARS R5.F2: WHEN preparation exceeds its representation domain, it SHALL return named admission/unsupported status distinct from runtime outcome.
Predicate `b_reference_and_lowering_complete(profile, grammar, cases)`: complete admitted grammar/lowering; lexical/strict order and recursion; resumable helper cursors; exact UInt128 APIs; partial encode/decode; generic declarations/ordered matches; uncached witness DAG/export inventory. Check Schedule, same-ID distinct programs, repeated occurrences and pending-return/boundary prefixes. Retain explicit higher-order/polymorphic/ADT extensions. B-T08/09 optimizations follow the deterministic baseline and preserve every work/frame prefix.

### R6 — Required per-slice K/Quint/Lean/Clean correspondence

Owners R, existing private Fenrir L/C, O and Opus. Iterate each slice beside R4/R5, then converge; required throughout. Existing private destination only; no private code/path disclosure.

EARS R6.F1: WHEN a slice is accepted across layers, each required layer SHALL expose its actual complete projected states on a declared common domain and retain unsupported/unrun gaps.
Predicate `cross_layer_slice_complete(slice, membership, projections)`: independent complete prefix expectations match actual layer artifacts; K engine versus supplied projection distinguished; numeric/profile/terminal/helper differences mapped; Clean witness and constrained step distinguished; operation inventory and actual exporter bound. Separately discharge exact K/Lean, representation, step/run and owner-defined soundness/completeness obligations with premises. Finite tests do not discharge them. Exit: individually scoped correspondence evidence and actual native export, not merely matching diagnostic JSON.

### R7 — General signed financial source-to-target and lifecycle

Owners I/H/O, target/native and Opus. Acceptance needs relevant R4/R5/R6/U1 and qualified targets; library/oracle preparation starts earlier. Preserve Core/2 await and review financial controls/actual ledger phases first.

EARS R7.F1: WHEN two structurally contrasting supported financial programs execute through the same public compiler, complete outcomes SHALL match independent source-derived expectations.
EARS R7.F2: WHEN a later segment fails or awaits, earlier committed effects and residual duties SHALL survive according to the explicit phase/lifecycle contract.
Predicate `two_program_complete_outcomes(programs, targets)`: two new positive source programs, grammar/field coverage, ordered effects, affected/preserved state, gross debit/fees/net receipts, custody/reserve dust, opening/accrued/discharged/closing liabilities, pending/partial/unknown/settled/recovered state, work/resumption. Include D19/D20, conditional partial escrow/recovery and actual retained ledger-phase failures. Proof cuts create no financial commit. Actual native/target evidence remains mandatory for retained U2/U3 exits.

### R8 — Native recursion, history and private composition

Owners existing native/history/private Fenrir, H (accepted financial semantics), Opus. Acceptance follows relevant U2/U3 and accepted SP07/SP08 inputs per scope; not blanket U6 completion.

EARS R8.F1: WHEN recursive/private composition is claimed, actual target evidence SHALL cover the claimed predecessor and multi-parent scope with complete carried financial obligations.
Predicate `history_composition_complete(scope)`: retained two-step native recursive financial evidence, legitimate genesis, bounded multi-parent compatibility and separate-party private handoff; all five operators (sequential, disjoint parallel, shared-state interleaving, atomic synchronization, asynchronous messaging) meet their exact existing predicates. Preserve authenticated completeness/unique consumption/disclosure/witness availability requirements under Opus ownership. Ledger induction alone cannot close MC03/MC06 or full SP09. Requalify affected successor scope after financial extension.

### R9 — Full conformance and optional-service implementation obligations

Financial/library/O owners prepare source/membership/oracles now; acceptance uses applicable R7/R8. U5 service owners preserve all federation/solver exits without requiring federation for direct use.

EARS R9.F1: WHEN conformance is reported, every frozen retained member and every specified field SHALL have its own semantic/native/target disposition.
Predicate `retained_conformance_complete(manifests)`: all 277 ACTUS aggregate fixtures/fields, 18 executable types, 32 taxonomy dispositions, DS01–07; all 72 DeFi rows, DA01–24, adopted supplements, three held-outs, eight intent cases and eight regression classes. Reconcile actual frozen membership, ANN inventory discrepancy and supplement manifests rather than reducing counts. Exact arithmetic/rounding/price orientation and gross economics remain required. Independent held-out contamination is disclosed. U5 separately requires exact adapter bytes, actual federation membership/threshold/epoch policy and services, OWS/x402 separation, durable concurrent reservation/retry/failover and two solvers. Preparation is parallel; shared arithmetic/ABI changes integrate serially.

### R10 — Full public release and retained acceptance

Owners I, independent external developers/builders and applicable reviewers. Docs/tools/install preparation overlaps; acceptance waits for all required U2–U6/member/G exits.

EARS R10.F1: WHEN full v1 completion is claimed, every mandatory retained predicate SHALL have exact-candidate evidence and unresolved required evidence SHALL prevent completion.
Predicate `v1_retained_exits_pass(crosswalk)`: all original U/M/MC/SP/G and canonical/retained predicates; clean public non-fixture own-wallet journey; two builders; two non-toy pilots including ACTUS versus audited Compact baseline; licenses/attribution/dependencies and required privacy/audit outcomes. Distinguish Rust/Node/executables and truthful LSP/MCP support. Stagenet, Preview and required Fenrir each need actual evidence. Maintainer records never become public developer permissions.

## Critical path, parallelism and unknowns

First-demo serial path: R1 source closure → unchanged admitted build/native gate → R2 observed proof/independent verification → qualified target/admission → R3 observed settlement. Native build prerequisite diagnosis and stagenet source configuration are genuinely parallel preparation. Full-v1 path converges R4/R5/R6 → applicable R7 → R8 → applicable R9 → R10; R0 is independent and R9 membership/library/oracle work starts early. Independent oracle production precedes implementation execution. Shared schema/ABI/dispatch and final source integration have one owner; their changes are not safely parallel edits.

Remaining unknowns include an actually admissible native build path, current per-package target qualification, unresolved statement/context and full39/B physical contracts, private Clean exporter/theorem membership, full admitted source grammar and complete financial phase/control mapping. PR168 is merged; actual241/8/216/333 ordinary observations have scoped receipts. These known facts do not remove the remaining unknowns. No calendar promise follows.

## Optimization work retained in v1

The uncached deterministic witness DAG, earliest-stop slices at widths1/2/4/8, bounded static residualization with generic fallback, separate witness/constraint/export plans and declaration-driven matching remain required UIREQ031–035/B-T07–09 work. Start with their correct baseline. Optional later last-use storage, packing, caching and specialization strategy experiments need matched complete-path measurement and preserved semantics. Required ADTs/helpers remain v1; no native/security/acceptance boundary changes here.

## Controlling final03 reconciliation

This dated audit decomposition is interpreted with [revision03](sherlock/design.md), [the adopted convergence change](../openspec/changes/sherlock-completion-convergence/ADOPTION.md), its [concrete compiler increment](../openspec/changes/sherlock-completion-convergence/FIRST-COMPILER-BATCH.md) and [13 finite batch contracts](../openspec/changes/sherlock-completion-convergence/batch-plan.json). UIREQ031–035/B-T07–09 remain required current v1. Only subsequent measured strategy experiments are optional.

Current S1/S2/S3 evidence is the241 ordinary run, actual Rust stream8/216 and actual333 same-carrier named/ordered match. The initial composed lexical check failure and normalized continuation remain preserved. Optimized147 already passed all147 selected normal tests including N03–N08 exhaustive cuts; do not redispatch those on stale omission claims. Full B/full96 and whole independent PassT remain open. PR168 is merged; SDK/native/target acceptance remains unobserved.

R0–R10 are work/dependency groups, not a rule requiring all earlier ordinals before disjoint source preparation. The next substantial source work is the real public compile/admission branch. Every code batch updates affected Diátaxis pages or records no-impact; per-slice K/finite-Quint/Lean/Clean/handwritten/native/target obligations remain separate and conjunctive. The current goal state must be observed from the goal tool, not inferred from this plan.

The [previous implementation plan](audits/2026-10-17/sherlock-plan-history/IMPLEMENTATION-PLAN-BEFORE-REVIEW.md) is retained verbatim as dated history; current revision03 and the adopted task/spec package select new work.
