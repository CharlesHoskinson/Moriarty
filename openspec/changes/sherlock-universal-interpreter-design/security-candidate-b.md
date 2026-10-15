# DRAFT security delta: Sherlock Candidate B

Author: Opus 5.5; security judgments mine. Reconciled with [interfaces.md](interfaces.md). Specified-only, unexecuted, unapproved; fresh non-author Opus and Muse audits required. `security-requirements.md` (UISEC-001–012) is unchanged and binds. ZR, MNR, U0–U7, M1–M17, MC, SP, G stay open. Rule/charge/allocation tables, capacities, layouts, security theorems, backend realization and financial opcodes remain **OPEN**.

**Profiles.** `sherlock-k-a/1` and `sherlock-quint-a/1` are frozen Candidate A; `sherlock-helper-microstep/1` (B) is a new schedule, neither refinement of nor oracle for A. Nothing transfers between profiles without a stated differential predicate (OSR-1014-02).

## Dispositions

- **C1 (UISEC-006, unmetered A helpers).** Per-profile scope. In A, UISEC-006 binds each charged legacy transition; A helpers are pure parts of one transition, so A supplies no helper-work bound and is never B001 evidence. In B it binds each helper event.
- **C2 (UISEC-004 overflow label).** Per-profile binding that keeps existing observations:

  | Profile | Logical frame-cap overflow |
  |---|---|
  | `sherlock-k-a/1` | charged failing push, then uncharged `Rejected(4)` publication (`models/sherlock/k/main.k:140-143`) |
  | `sherlock-quint-a/1` | `Rejected(StackBound)` under its own schedule (`models/sherlock/quint/universalMachine.qnt:66-71`) |
  | `sherlock-helper-microstep/1` | as `sherlock-k-a/1` |

  UISEC-004's predicate (`|K| ≤ maxDepth`, well-bracketed trace, explicit terminal non-success) binds unchanged; its word "Exhausted" is realized as these rejections. This author deviation is recorded for review. No profile defines `Exhausted(Frames)`.
- **C3 (UISEC-003 D1 `child < parent`).** Chosen: the order holds on logical allocation ordinals (B002), not assumed physical addresses. D2 is not selected.
- **C4 (UISEC-002 explicit reject arm).** Every `MatchPlan` default chain ends in `Select` or `NoMatch`; `NoMatch` is the reject arm, mapped to `Rejected(9)` (B007).

## EARS clauses

**UISEC-B001 Helper microsteps.** Under B, every lookup, bind, validation, equality and traversal step SHALL be an explicit helper event. Before each event the interpreter SHALL apply this precedence: publish an already-final value or error; else zero fuel yields `Exhausted(Fuel)`; else the frame-cap check (C2); else the data, environment, helper-task and `maxHelperWork` checks. A refused capacity check SHALL perform no update or charge and SHALL publish `Exhausted(kind)`, `kind ∈ {DataNodes, DataFields, EnvironmentCells, HelperTasks, HelperWork}`. Serialization refusal occurs only at an encode/checkpoint request. `AdmissionRejected`, `CapacityNotAdmitted` and `UnsupportedFeature` are tooling outcomes outside `Outcome`, never reported as application or A outcomes.
- *Predicate:* `∀μ: checks(μ) precede work(μ) ∧ helperWork' = helperWork + 1 ≤ maxHelperWork`. Repeated references create repeated tasks, so sharing never lowers `helperWork` or `helperVisits`.
- *Scenarios:* a length field of 2¹²⁸−1; a shared 2ⁿ DAG under equality; an A trace reused as B evidence.
- *Binds:* UISEC-006 (C1), ZR09, ZR12.

**UISEC-B002 Deterministic per-layout codec.** Each `LayoutSchema` SHALL declare a codec policy. The initial policy:
1. *Logical ordinals.* A data node's ordinal is `allocation.dataNodes` before its allocating event; field slots and environment cells use their own counters likewise. Fields are evaluated before allocation, so each data reference in a new row names a smaller ordinal or an acyclic prepared constant (C3).
2. *Append-only identity projection.* Physical data, field and environment indices equal logical ordinals. Rows below the counters are immutable historical allocation metadata: possibly unreachable, meaningless, still counted against capacity.
3. *Unused zero.* Every physical slot at or above its active length, including popped frame and released helper-task slots, SHALL be zero.
4. *Determinism.* Genesis SHALL equal `encode_L(program, s₀)`, which visits roots in fixed order and nodes in left-to-right post-order without structural merging. `step_L` SHALL be a function of program and pre-image, and SHALL reference only rows reachable from pre-image roots, prepared constants or rows it allocates.
5. *No address semantics.* Ordinals, sharing and bytes are unobservable; no rule branches on an address. Image equality neither implies nor is implied by semantic equality. Realizations and layouts are compared via `decode`/projection, never bytes.

Canonical images are genesis closed under `step_L`; `encode_L(decode_L(i)) = i` is required only at genesis. Interning, slot reuse or compaction SHALL declare a deterministic projection to logical ordinals, meets UISEC-003 only on live ordinals, and needs separate review; none is assumed.
- *Predicate:* `decode_L(i)=some s ⇒ WF_L(i)`: rows typed per `CtorLayout`, `ord(child) < ord(parent)`, unused slots zero, frame `codeId`s from the bound table, counters within `Bounds`.
- *Scenarios:* a stale popped frame; a forward or self reference; a reference to an unreachable historical row; a mismatched genesis.
- *Binds:* UISEC-001/002/003 (C3)/004/009, ZR03.

**UISEC-B003 Cost planes.** Reference work, helper work, helper visits, physical rows/work, frames, bytes, circuit cost and fees SHALL be counted separately. Caching, batching, sharing, interning or specialization SHALL NOT reduce reference work, helper work or logical allocation counters.
- *Predicate:* those counters are equal across cold, warm and batched runs.
- *Scenarios:* a cache hit skipping a charge; DUST netted against fuel.
- *Binds:* UISEC-006, ZR13.

**UISEC-B004 ReferenceSlice earliest stop.** A batched slice SHALL apply the B001/C2 precedence at every internal position, stop at the earliest outcome or suspension, record actual charges and export complete state at the cut.
- *Predicate:* `slice(m) = prefix_k(ref)`, `k = min(m, firstStop)`.
- *Scenarios:* an error at position 3 of 8; a padded tail claiming steps; a resume dropping operands.
- *Binds:* UISEC-004/006/009.

**UISEC-B005 Plans as capabilities.** ConstraintPlan, WitnessPlan and export SHALL be separate. Missing lookup, interaction, checked-u128 lowering or native closures SHALL return located `UnsupportedFeature`. `cacheHit` and `rowGenerated` SHALL NOT be validity or acceptance evidence.
- *Predicate:* acceptance depends only on emitted constraint components.
- *Scenarios:* a solver succeeding over an omitted constraint; wrapping U64 as u128.
- *Binds:* UISEC-005/012, ZR06, ZR11.

**UISEC-B006 ResidualStepIR preserves provenance.** The residualizer MAY key on static source occurrence, reference rule and layout facts. Execution ordinal (`ExpandedStepId`, `referenceCursor`), control, `controlOrigin`, fuel/work, `helperWork`, depth, frames, environment, allocation counters, helper cursors, inputs and outcomes SHALL stay dynamic. Each residual event SHALL carry its originating `sourceOccurrenceId` and `ruleId` via `sourceOccurrenceMap`/`ruleBoundaryMap`. Specialization SHALL NOT merge executions of one occurrence, fold charged events, unroll recursion or drop cap checks. Size-limit fallback SHALL emit generic dispatch, never exhaustion.
- *Predicate:* universal, family and per-program masks yield identical decoded StepRecord prefixes, including occurrence/rule IDs, ordinals, outcomes, work and cap checks.
- *Scenarios:* factorial with constant n folded to 6; two executions of one occurrence sharing an ordinal; fallback reported as `Exhausted`.
- *Binds:* UISEC-001/004/006.

**UISEC-B007 Ordered ADT matching.** Constructor fields SHALL be evaluated left-to-right, once, before allocation. A match SHALL evaluate the scrutinee once, select the minimum matching arm, bind freshly and restore the caller environment. Reaching `NoMatch` SHALL complete one charged legacy event carrying the rule ID of K's empty-case rule (`main.k:212-213`), then publish `Rejected(9)` uncharged. Equality SHALL be structural; each occurrence path is charged separately.
- *Predicate:* `arm = min{i | matches(row_i, v)}`, else `Rejected(9)`.
- *Scenarios:* in `Both(Pay(4,Done),Pay(5,Done))` only arm one fires; a same-arity foreign tag; a plan lacking a terminal node.
- *Binds:* UISEC-002 (C4).

**UISEC-B008 Financial/await adapter unchanged.** B SHALL preserve Core/2 successful-prefix await, residual duties, complete ordered effects, fees and gross debits. A proof chunk or cut SHALL NOT be a financial commit. Commit/Yield/Await opcodes stay OPEN; nothing moves an await, effect or commit across a residual block.
- *Scenarios:* a chunk boundary as commit; an await relocated by specialization.
- *Binds:* UISEC-007/008/010/011.

**UISEC-B009 Profile and stage labelling.** Every claim SHALL name its profile and evidence class. Public Clean-stage text SHALL omit private paths, code, pins and keys.
- *Binds:* UISEC-012, MNR05.

## Not claimed

No theorem, test, model run, review or measurement was performed.
