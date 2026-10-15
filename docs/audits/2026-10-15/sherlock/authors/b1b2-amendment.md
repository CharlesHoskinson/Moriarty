# DRAFT security delta: Sherlock Candidate B

**Author:** Opus 5.5. All security judgments here are mine.
**Status:** proposed, specified-only and unexecuted. This is not approval, adoption or a semantic freeze. Fresh non-author Opus and Muse audits are required.
**Not claimed:** no theorem, test, model run or measurement.
**Still open:** ZR, MNR, OSR, U0–U7, M1–M17, MC, SP and G. Backend realization (per-program, universal or family) is OPEN. Commit/Yield/Await opcodes are OPEN.
**Not required:** no foreign proof system, external code, ledger10 or VerifyProof.
**Authority:** native closures and host predicates carry zero authority.

## Relation to UISEC-001–012

- `security-requirements.md` is immutable at source. It binds every profile, except for the profile-specific amendments A1–A4 below. Nothing else in this file overrides it.
- `sherlock-k-a/1` and `sherlock-quint-a/1` are frozen Candidate A.
- `sherlock-helper-microstep/1` (B) is a new schedule. It is neither a refinement of A nor an oracle for it. Nothing transfers between profiles without a stated differential predicate (OSR-1014-02).
- **Planes:**
  - *Source:* UISEC text and ProgramImage.
  - *Boundary:* genesis, codec, checkpoint, ChunkStore and external state claims.
  - *Runtime:* per-step physical images, events and outcomes.

### A1. UISEC-004 overflow arm (all named profiles)

Frame-cap overflow yields `Rejected(error4)`. It is a charged failing push followed by zero-cost error publication. It is terminal and explicit, never success, and never truncates frames, effects or duties.

`resourceLimit(o) ≝ o = Exhausted(_) ∨ o = Rejected(error4)`. No consumer SHALL read a resourceLimit outcome as a semantic, typing or financial rejection. UISEC-007 and B008 hold under it.

Rationale: keeping error4 keeps B's outcome comparable to A. Security needs explicitness, charging and terminality, not a particular constructor name.

### A2. UISEC-006 outcome words (B)

"Reject" on budget overflow means one of:
- `Exhausted(Fuel)` at a nonterminal state with zero fuel;
- `Exhausted(HelperWork)` when a charge would exceed `maxHelperWork`;
- `Exhausted(kind)` when an image cap is reached.

Per-step cost is at most 1. Two conditions are checked at admission and fail with `CapacityNotAdmitted`: `stepCap ≥ 1`, and counter widths large enough for `maxReferenceWork` and `maxHelperWork`. Counter wrap is therefore unreachable rather than a runtime outcome.

Precedence:
1. Publish an already-final value or error.
2. Zero fuel.
3. Charge the rule, then the frame-cap check.
4. Data, env and helper caps. Their order among themselves is a Phase 0 item.

A step that cannot be charged performs no work.

### A3. UISEC-003 scope

UISEC-003 governs two things:
- **The physical data arena between canonical boundaries.** It is append-only, `[0, activeLength)` is unchanged by every step, and every child index is below its parent's.
- **Logical allocation counters across the run.** They are monotone and are never reduced by sharing, interning, slot reuse or projection.

Only canonical projection (A4) may renumber or drop physical rows, and it preserves the reachable structure and the counters exactly. Frames, helper tasks and layout-declared reusable cells are not heap H. A reused slot is fully overwritten in the step that activates it and zeroed in the step that releases it.

### A4. Canonicality scope

Canonical-only decoding applies only at boundaries: genesis admission, `ChunkStore.get`, checkpoint resume and any external state claim. `ChunkStore.put` and checkpoints emit projected images.

Mid-segment images may hold unreachable rows but must satisfy `StepInv`:
1. Active data rows are typed: the tag belongs to its type, the slice length equals the arity, field sorts are correct and scalars are in range.
2. Each data reference is below its holder's index. Control, env and frame references are below their active lengths.
3. Inactive rows and fields are zero.
4. Frame `codeId`s come from the bound table.
5. Counters are within their widths and bounds.

No rule may create a reference to an unreachable row, and such rows are never reused before projection.

**Obligation (unproved):**
- `decodeCanonical` at genesis ⇒ `StepInv`.
- `StepInv(pre) ∧ stepConstraints ⇒ StepInv(post)`.

**Per-step constraints** must do the following, because no step decodes:
- write only at `activeLength`, with an exact length increment;
- bound-check every read;
- type-check and range-check every written field;
- zero each released slot;
- range-check every `codeId`.

The physical post-image SHALL be a deterministic function of the pre-image and the ProgramImage, so the prover has no choice of representation.

**Statements:** UISEC-009 per-step statements bind physical digests injectively. External claims bind only canonical digests or published outcomes.

## Clauses (B profile)

**UISEC-B001 Helper microsteps.**
- Every lookup, bind, validation, equality and traversal step is an explicit microstep costing 1.
- Capacities are checked before any work on witness- or source-supplied sizes.
- Failure follows A1/A2 and is never reclassified as an A outcome.
- *Predicate:* `capCheck(μ) ≺ work(μ) ∧ helperWork' ≤ maxHelperWork`, where `w` counts occurrences, not unique nodes.
- *Scenarios:* length 2¹²⁸−1; a shared 2ⁿ DAG under equality; a deep lookup chain; an A trace reused as B evidence.
- *Binds:* UISEC-006, ZR09, ZR12.

**UISEC-B002 MachineImage codec.**
- `encode` is partial.
- `decodeCanonical` accepts only canonical images: every live slot is reachable, inactive slots are zero, the root order is deterministic and children come before parents.
- `decodeCanonical(i) = some s ⇒ encode(s) = some i`.
- Images that differ only in unreachable bytes SHALL NOT both decode canonically.
- Runtime images follow A4.
- *Binds:* UISEC-001–004 and 009, ZR03.

**UISEC-B003 Cost planes.**
- Reference work, `helperWork`, `helperVisits`, rows, frames, bytes, circuit cost and fees are counted separately.
- `consumedWork` includes helper work. `helperTasks` counts live scratch only.
- Caching, batching, sharing and specialization never remove a charge or an inspection.
- *Binds:* UISEC-006, ZR13.

**UISEC-B004 Earliest stop.**
- A `ReferenceSlice` applies A2 precedence at every internal position and stops at the earliest outcome.
- Charges are recorded per position, never retrospectively.
- The complete state is exported at the cut.
- *Binds:* UISEC-004, 006 and 009.

**UISEC-B005 Plans as capabilities.**
- ConstraintPlan, WitnessPlan and export are separate.
- Unsupported features are named `Unsupported` holes.
- `cacheHit` and `rowGenerated` are diagnostics only. Acceptance depends only on emitted constraint components.
- *Binds:* UISEC-005 and 012, ZR06, ZR11.

**UISEC-B006 Residualization.**
- Immutable source-occurrence IDs may be static keys, provided provenance is preserved.
- Ordinals, control, fuel, frames, env and outcomes stay dynamic.
- No unrolling, folding of charged steps or deletion of cap checks.
- At the size limit, fall back to generic dispatch, not exhaustion.
- *Binds:* UISEC-001, 004 and 006.

**UISEC-B007 Ordered ADT matching.**
- Fields are evaluated left-to-right, exactly once.
- The scrutinee is evaluated once, the minimum matching arm is selected, and binding is fresh.
- Equality is structural, and each occurrence path is charged.
- `NoMatch` maps to `Rejected(error9)` and adds no `Outcome`.
- *Binds:* UISEC-002.

**UISEC-B008 Financial adapter.**
- Core/2 successful-prefix await, residual duties, ordered effects, fees and gross debit are preserved.
- Chunks and cuts are never commits.
- Nothing moves an await, effect or commit across a residual block.
- *Binds:* UISEC-007, 008, 010 and 011.

**UISEC-B009 Labelling.**
- Every claim names its profile and evidence class.
- Public Clean-stage text gives scope only, with no private paths, pins or keys.
- *Binds:* UISEC-012, MNR05.

## Engineering handoff

These are document edits only. No implementation is required before source integration.

**`interfaces.md`**
- Replace `decode` with:
  ```text
  decodeCanonical(program,image,layout) -> Option<ReferenceState>
  decodeStep(program,image,layout) -> Option<ReferenceState>   -- accepts StepInv; ignores unreachable rows
  project(program,image,layout) -> Option<MachineImage>
  checkStepInv(program,image,layout) -> Bool                    -- host-only
  isResourceLimit(Outcome) -> Bool
  ```
- Laws:
  - `decodeCanonical i = some s ⇒ decodeStep i = some s`
  - `decodeStep i = some s ⇒ ∃i'. project i = some i' ∧ decodeCanonical i' = some s`
- `materializeStep` uses `decodeStep` on both `preImage` and `output.next`. It is host-side and reported separately.
- `MachineImage`:
  - `activeLengths` holds physical lengths.
  - `allocationCounters` holds logical counts.
  - The two are never conflated.
- `ChunkStore.put` emits `project` output, and `get` uses `decodeCanonical`.
- In the error table, error4 is classed ResourceLimit.
- Replace "compact" wording with "only via `project` at A4 boundaries".

**Other files**
- `tasks.md:16` should read: "error4 retained for frame-cap overflow in all profiles; resourceLimit classifier per A1."
- `spec.md` UIREQ-028 should add "error publication" as a zero-cost event.

**Gates**
- **Phase 0:** "No B schema freeze until the error table, `isResourceLimit`, `StepInv` fields, per-rule charge and allocation rows (including failure transitions) and intra-cap order are specified and Opus-reviewed."
- **Phase 3:** "`bounded_decode_matches` SHALL NOT be claimed for B unless gated on Opus-authored A4/B002 hostile cases. These are: inactive-slot garbage, a referenced unreachable node, a foreign `codeId`, a tag/layout mismatch, a write below `activeLength`, and a nondeterministic physical post-image."
- **Every phase:** "No B-specific claim without the applicable Opus-authored B001–B009 hostile cases as its gate. Phase 8 is not the sole gate."
