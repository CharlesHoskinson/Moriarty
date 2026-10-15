# DRAFT security delta: Sherlock Candidate B

## Candidate B profile reconciliation (Opus 5.5)

**Precedence.** This section governs proposed Candidate B only. It takes precedence over any conflicting wording in C1–C4, B001, B002, B006, and B007 that follows. The original Candidate A document is frozen as historical requirements, and C1–C4 are disposed per profile. All security and financial obligations not changed here remain in full force.

**Profiles.**
- **A-K (frozen):** helpers are unmetered and frame push is charged. Frame-cap overflow is `Rejected(error4)`.
- **A-Quint (frozen):** keeps its own terminal-tick rule.
- **B-helper (new):** legacy completion costs 1 and each helper event costs 1. Startup, terminal publication, and error publication cost 0. Frame-cap overflow remains `Rejected(error4)`.
- **Runtime caps:** exceeding a runtime image cap or helper cap yields `Exhausted(kind)`, where `kind` names the cap.
- **Preparation:** admission failure and `UnsupportedFeature` are reported outside the application `Outcome`.

**Accounting.**
- `helperWork` counts charged helper events and is bounded by `Bounds.maxHelperWork`.
- `helperVisits` counts occurrence inspections. Entry and exit MAY count 0.
- `helperTasks` is live scratch only.
- `ReferenceState.consumedWork` totals all charged work, including helper work.
- The budget MUST be checked before each charged step. Caching MUST NOT remove any charge.

**NoMatch.** `NoMatch` is a match-plan or helper result that maps to `Rejected(error9)`. It adds no application `Outcome`.

**B002 codec obligation.** Each layout MUST define a canonical codec with:
- a deterministic root order;
- topological order, with each child before its parent;
- zeroed inactive fields.

Sharing has structural meaning only. A layout-defined canonical projection MAY compact the physical representation. The reference keeps its logical allocation history as explicit counters. This is an obligation, not a claim that canonicality is implemented. The exact codec and its proofs remain open.

**B006 occurrence IDs.** Immutable source-occurrence IDs MAY serve as static identifiers, constant indices, and keys, provided provenance is preserved. Expanded ordinals, control, and fuel stay dynamic at runtime. Recursion MUST NOT be unrolled, and charged steps MUST NOT be dropped.

**Status.** No implementation, proof, or adoption is claimed. This is author text for later independent review.

Author: Opus 5.5; security judgments mine. Proposed, specified-only, unexecuted; not candidate approval (fresh non-author Opus and Muse audits required). `security-requirements.md` (UISEC-001–012) is unchanged and binds. ZR, MNR, U0–U7, M1–M17, MC, SP, G stay open. No proof, soundness, refinement or measurement is claimed. Backend realization (per-program/universal/family) remains **OPEN**; no foreign proof system or external code is selected.

**Profiles.** `sherlock-k-a/1` and `sherlock-quint-a/1` are frozen Candidate A: unmetered helpers, K stack overflow `Rejected(error4)`, differing terminal charges. `sherlock-helper-microstep/1` (B) is a *new* schedule, neither refinement of nor oracle for A. Nothing transfers between profiles without a stated differential predicate (OSR-1014-02).

## Conflicts recorded, not rewritten

- **C1.** UISEC-006 charge-before-effect is vacuous for A's unmetered helpers; under B it binds per microstep.
- **C2.** UISEC-004 says overflow is `Exhausted`; A K yields `Rejected(error4)`. Pending per-profile classification.
- **C3.** UISEC-003 D1 topological allocation: a shared B arena must keep `child < parent`, or adopt D2 with a separate argument.
- **C4.** UISEC-002's explicit reject arm maps to B007 `NoMatch`; name pending.

## EARS clauses

**UISEC-B001 Helper microsteps.** Under `sherlock-helper-microstep/1`, every lookup, bind, validation, equality and traversal step SHALL be an explicit microstep. Before work on witness- or source-supplied sizes, the interpreter SHALL check declared code, data, frame, env, serialization and helper-work capacities. Failure SHALL yield a profile-named `Exhausted` or `Unsupported`, never reclassified as an A outcome.
- *Predicate:* `∀μ∈helperTrace: capCheck(μ) precedes work(μ) ∧ helperWork' = helperWork + w(μ) ≤ capHelper`. Here `w` counts source occurrences, not unique nodes.
- *Scenarios:* a length field of 2¹²⁸−1; a shared 2ⁿ DAG under equality; a deep lookup chain; reusing an A trace as B evidence. Each must exhaust or reject.
- *Binds:* UISEC-006 (C1), ZR09, ZR12.

**UISEC-B002 Canonical MachineImage.** `encode` SHALL be partial, and `decode` SHALL accept only canonical images. In a canonical image, every live slot is reachable from control, env or frames; inactive slots hold zero; arena nodes are typed and immutable; and frames bind a `codeId` from the bound ProgramImage.
- *Predicate:* `decode(i)=some s ⇒ encode(s)=some i`. Two images with the same reachable meaning but different unreachable bytes SHALL NOT both decode.
- *Scenarios:* garbage in an inactive slot; an unreachable forged node; a frame `codeId` from another table; a tag mismatching its `CtorLayout`.
- *Binds:* UISEC-001/002/003 (C3)/004/009, ZR03.

**UISEC-B003 Cost planes.** Reference work, helper visits, physical rows/work, frames, bytes, circuit cost and fees SHALL be counted separately. Caching, batching, sharing or specialization SHALL NOT reduce reference or helper work.
- *Predicate:* `refWork(cold)=refWork(warm)=refWork(batched)` and `helperVisits` is invariant in the same way. Physical counters are diagnostic only.
- *Scenarios:* a cache hit that skips a charge; padding counted as reference work; DUST netted against fuel.
- *Binds:* UISEC-006, ZR13.

**UISEC-B004 ReferenceSlice earliest stop.** When steps are batched, a ReferenceSlice SHALL apply the profile's outcome/fuel/frame-cap precedence at every internal position, stop at the earliest reference outcome, record actual charges and export complete state at the cut.
- *Predicate:* `slice(m) = prefix_k(ref)` with `k = min(m, firstStop)`. Charges are per position, never retrospective.
- *Scenarios:* an error at internal position 3 of 8; zero-fuel final publication; a padded tail claiming extra steps; resume dropping pending operands.
- *Binds:* UISEC-004/006/009.

**UISEC-B005 Plans as capabilities.** ConstraintPlan, WitnessPlan and export SHALL be separate. Unsupported lookup, interact, checked-u128 lowering or native closures SHALL be named `Unsupported` capability holes. Host predicates (`cacheHit`, `rowGenerated`) SHALL NOT be validity booleans or acceptance evidence.
- *Predicate:* acceptance depends only on emitted constraint components. `Unsupported` is never an A outcome.
- *Scenarios:* a witness solver succeeding over an omitted constraint; a basic constraint JSON presented as a witness plan; a wrapping U64 used as u128.
- *Binds:* UISEC-005/012, ZR06, ZR11.

**UISEC-B006 ResidualStepIR retains dynamics.** Static residualization SHALL keep fuel, depth, frames, env, control, inputs, outcomes and source occurrences dynamic; SHALL NOT key on fuel/depth, fold charged steps or unroll recursion. Size-limit fallback SHALL emit generic dispatch, not exhaustion.
- *Predicate:* universal, family and per-program masks yield identical decoded StepRecord prefixes, outcomes, work and cap checks.
- *Scenarios:* a factorial with constant n folded to 6; a dropped frame-cap check; a fallback reported as `Exhausted`.
- *Binds:* UISEC-001/004/006.

**UISEC-B007 Ordered ADT matching.** Constructor fields SHALL evaluate left-to-right, once, before allocation. A match SHALL evaluate the scrutinee once, select the *minimum* matching arm, bind freshly and restore the caller env. Equality SHALL be structural, never by address; each occurrence path is charged separately.
- *Predicate:* `arm = min{i | matches(row_i, v)}`. With no match, the outcome is `NoMatch` (C4).
- *Scenarios:* for `Both(Pay(4,Done),Pay(5,Done))`, only arm one fires; a same-arity tag from another type; shared versus separate children charging differently.
- *Binds:* UISEC-002.

**UISEC-B008 Financial/await adapter unchanged.** B SHALL preserve Core/2 successful-prefix await, residual duties, complete ordered effects, fees and gross debits. A proof chunk or cut SHALL NOT be a financial commit. Commit/Yield/Await opcodes stay OPEN; nothing moves an await, effect or commit across a residual block.
- *Scenarios:* a chunk boundary treated as commit; a duty dropped at a slice cut; an await relocated by specialization.
- *Binds:* UISEC-007/008/010/011.

**UISEC-B009 Profile and stage labelling.** Every claim SHALL name its profile and evidence class. Any public Clean-stage addendum SHALL describe the dependency and scope only, with no private paths, code, pins or keys.
- *Scenarios:* a B result cited as A conformance; a private pin in a public file.
- *Binds:* UISEC-012, MNR05.

## Not claimed

No theorem, test, model run or measurement performed.
