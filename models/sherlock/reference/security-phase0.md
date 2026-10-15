# Sherlock Candidate B: Security Phase 0 proposal

**Author:** Opus 5.5, acting as security design author. All judgments are mine.

**Status:** proposed, specified-only and unexecuted. This is not approval, adoption, a freeze or a decision vote. Fresh non-author Opus and Muse review is required.

**Base:** `f218a8d8`.

**Not claimed:** any B schema, runtime, security, native, correspondence or performance result. Native authority is 0. Geometry and the backend winner are unmeasured.

**Unchanged:** UISEC-001–012 and A1.

**Still open and binding:** ZR, MNR, OSR, U0–U7, M1–M17, MC, SP and G, plus the ACTUS, DeFi, composition and Core/2 duties.

**Profiles:** profiles stay separate. Nothing transfers between them without an OSR-1014-02 predicate. No bound or counter resets anywhere.

## 1. Dispositions (DISP)

| Class | Meaning | Effect |
|---|---|---|
| `C-INV` | Malformed untrusted image, witness, boundary or statement | No accepting assignment, no `Outcome`, no statement |
| `APP-R(c)` | Valid execution, application error | `Rejected(c)` is published in the image |
| `APP-X(k)` | Valid execution, budget or capacity limit | `Exhausted(k)` is published |
| `TOOL` | `AdmissionRejected`, `CapacityNotAdmitted`, `UnsupportedFeature`, residual nonconformance | Outside `Outcome`, with no statement |
| `PUB-REFUSE` | Claim fails labelling or hygiene | Not published |

`checkStepInv`, `rowGenerated`, `cacheHit` and host equality are diagnostics only. No host `Bool` establishes any disposition.

## 2. A2′: settles N2, amends A2 items 3–4

A2 items 1–2 and admission are unchanged.

- **Budgets** (fuel and `helperWork`) are eligibility checks. They are checked before the charge, and a failure is uncharged.
- **Capacities** (frames and image tables) are checked after the charge, and a failure is charged.

*Justification:* "a step that cannot be charged performs no work" forces uncharged budget refusal. A1 already fixes frame overflow as a charged failure.

**PREC.** Rows are evaluated in order, and the first match wins.

| # | Guard | Result | State update |
|---|---|---|---|
| P1 | Final control: `Return(v)` with no frames, or `Error(c)` | Publish `Halted(v)` or `Rejected(c)` | Outcome only, cost 0 |
| P2 | `remainingWork = 0` | `Exhausted(Fuel)` | Outcome only, cost 0 |
| P3 | `helperEligible` ∧ `helperWork = maxHelperWork` | `Exhausted(HelperWork)` | Outcome only, cost 0 |
| P4 | — | Charge | `remainingWork−1`, `consumedWork+1`, and `helperWork+1` iff `helperEligible` |
| P5 | Push ∧ `depth+1 > frames` | control := `Error(error4)` | P4 only; frames, arenas and counters unchanged. Then P1 publishes `Rejected(error4)` at cost 0, even at zero fuel |
| P6 | The first of `dataNodes`, `dataFields`, `environmentCells`, `helperTasks` (in that fixed order) whose delta exceeds its cap | `Exhausted(kind)` | P4 only; no write and no allocation-counter change |
| P7 | — | Apply the S-ROW | Per OWN |

Consequences:
- `helperWork ≤ maxHelperWork` and `remainingWork ≥ 0` hold as invariants.
- `stepCap ≥ 1` and cost ≤ 1 make the stepCap arm unreachable.
- `Exhausted(SerializationBytes)` occurs only at a requested `put` or checkpoint, after a completed step. It costs 0 and leaves the image unchanged.

## 3. A3′: settles N3, makes A3's "layout-declared reusable cells" exhaustive

Exactly `frames` and `helperTasks` are reusable. A layout cannot add more without a new profile.

*Justification:* saved environments in frames reference environment cells, so reusing a cell would silently mutate live frames.

**OWN**

| Region | Owner | Write | Release |
|---|---|---|---|
| ProgramImageView tables | Preparation | Never at runtime | Never |
| `data`, `dataFields` | Allocating row | At `activeLength` only, with an exact increment; children are below their parent | `project` only |
| `environments` | Binding row | Append-only persistent cells; the parent link is below the cell's own index | `project` only |
| `frames` | Depth: slot `d−1` is top | A push fully overwrites the slot. Only the top slot may be rewritten, and only fully | A pop zeroes the slot in the same step |
| `helperTasks` | LIFO live count | Same as frames | Exit or fail zeroes the slot |
| Registers (control, outcome, counters, cursor, `envRoot`) | The step | Overwritten deterministically | n/a |

- Data rows, field slots, environment cells, and frames or tasks below the top are immutable.
- Variable-length payloads (completed values, partial bindings) live in the append-only arenas.
- The logical `dataNodes`, `dataFields` and `environmentCells` counters are monotone and survive projection. Frame and task counts are live counts.

## 4. A4′: settles N6 and N1

```text
DetInput = {profileId, layoutDigest, bounds, fieldId, programMembershipDigest,
            preImage:MachineImage, finance:Reserved}
Reserved = Absent          -- sole admitted value in sherlock-helper-microstep/1
stepPhys : DetInput -> (MachineImage, StepEvent)   -- function on StepInv(pre)
```

`preImage` includes every register, cursor, counter and outcome. The following are excluded:
- time and randomness;
- cache state and backend ID;
- chunk size `m`.

Auxiliary witnesses are allowed only when the post-image stays unique.

`finance ≠ Absent` ⇒ `C-INV`. A future financial profile must make finance a typed, authenticated, statement-bound input. None is supplied here.

```text
BoundaryRecord = {identity, stepIndex, kind:Terminal|TechnicalCut|SourceSuspension,
                  physPost:Digest, canon:Digest, counters, remainingWork, helperWork}
ProjectRel(p,L,i,j) := project(p,i,L)=Some(j)
   ∧ decodeCanonical(p,j,L) = decodeStep(p,i,L) ≠ None
   ∧ counters, budgets, bounds, identity of j = those of i
Link(S_k,B,S_k+1) := S_k.post = B.physPost = DP(i) ∧ ProjectRel(p,L,i,j)
   ∧ B.canon = DC(j) ∧ S_k+1.pre = DP(j)
```

- DP and DC are domain-separated binding digests. No algorithm is selected.
- `ProjectRel` must be a constrained component. Host projection is diagnostic only.
- Genesis is `S_0.pre = DP(j₀)` with `decodeCanonical(j₀)`.
- External claims bind only DC digests or published outcomes.
- Every chunked or recursive claim is blocked until `Link` is constrained.

## 5. A-profile UISEC-006 outcomes: settles N4

| Arm | `sherlock-k-a/1` | `sherlock-quint-a/1` |
|---|---|---|
| Fuel | `Exhausted(_)` at nonterminal zero fuel | `phase=Exhausted` (zero-fuel `applyTick`) |
| Cost > `stepCap` | No `stepCap` parameter; frozen per-transition charges. Agreement with B is an OSR-1014-02 predicate | Same |
| Frame cap | A1 | A1 |
| Counter wrap | Unreachable only if frozen counters are mathematical integers. Cite the source; this is not circuit evidence | Same |
| Helper and image caps, split/join, statement reset | Do not exist. N/A, carried to the B statement plane | Same |

For A profiles, "reject" means only these spellings. No new A outcome is introduced.

## 6. S-ROW: requirements on the independent Sol catalog

Every microstep row SHALL supply the fields below. Normal behavior must cite frozen source. Anything uncited is `Unsupported`/OPEN and must never be invented.

```text
{ruleId, legacyRuleId?, eventKind ∈ {HelperEnter,HelperVisit,HelperExit,HelperFail,
 LegacyComplete,Publish}, originOccurrence, helperEligible, charge ∈ {0,1},
 occurrencesInspected ∈ {0,1}, frameDelta, helperTaskDelta ∈ {−1,0,+1},
 dataNodeDelta, dataFieldDelta, envCellDelta, capChecks (ordered per P5–P6),
 reads:[(table,index,boundSource)], writes:[(table, activeLength|topSlot|register)],
 releases, successorControl, failureExits:[(guard, Outcome | Error(code))]}
```

Security consumes these rows under the following constraints:
- `charge=0` iff `Publish`.
- `helperEligible` iff the event kind is `Helper*`.
- `capChecks` covers exactly the positive deltas.
- Every read is bounded by `activeLength` or depth, and writes follow OWN.
- Lookup selects the minimum `sourceTableIndex`, and match selects the minimum arm.
- Equality and validation visit occurrences, not unique nodes.
- `NoMatch` maps to `Error(error9)`, and checked arithmetic overflow maps to `Error(error10)`.

## 7. N5 and N7

**N5 (root edits; the meaning is fixed here):**
- `tasks.md:76` SHALL read: "partial `encode`; `decodeCanonical` at A4 boundaries; `decodeStep` under `StepInv` at runtime."
- `traceability.md:145` SHALL read: "A/B are separate schedules; no transfer without an OSR-1014-02 predicate."

**N7 is retained unmodified:** the `/home/charl` paths in `observations.json` stay as frozen evidence of hygiene noncompliance. This proposal does not cure them, and Clean text must never copy them.

## 8. Hostile contracts (unexecuted)

Gates: REF (reference step), CODEC (`bounded_decode_matches`), CON (constraints/plans), CHUNK (slice/boundary/recursion), FIN (financial adapter, blocked while `finance=Absent`), PUB.

| Case | Mutation | Expected | Gate |
|---|---|---|---|
| B001-a | Fuel ≥ 1, `helperWork=max`, next event is a helper event | `APP-X(HelperWork)`; budgets unchanged | REF |
| B001-b | Fuel 0 and `helperWork=max` | `APP-X(Fuel)` | REF |
| B001-c | Fuel 1, push at `depth=frames` | Charged `Error(error4)`, then `APP-R(error4)` at fuel 0 | REF |
| B001-d | Claimed length 2¹²⁸−1 | Static claim: `TOOL CapacityNotAdmitted`. Witness claim: `C-INV`, with no read past the bound | CON |
| B001-e | Shared 2ⁿ DAG under equality | `APP-X(HelperWork)` at the per-occurrence ordinal | REF |
| B001-f | K-A trace relabelled as B | `C-INV` (`profileId` bound) | PUB |
| B002-a | Images differ only in unreachable bytes | At most one decodes canonically; the other is `C-INV` at the boundary | CODEC |
| B002-b | Nonzero inactive slot or field | `C-INV` | CODEC |
| B002-c | Reference to an unreachable row | `C-INV` | CODEC, CON |
| B002-d | Foreign `codeId` in a frame | `C-INV` | CODEC |
| B002-e | Same-arity tag from another type | `C-INV` | CODEC |
| B002-f | Write below `activeLength`; data or environment row reused | `C-INV` | CON |
| B002-g | Two post-images for one `DetInput`; `finance≠Absent` | `C-INV` | CON |
| B002-h | Stale field in a released frame or task slot | `C-INV` | CON |
| B003-a | `cacheHit` lowers `helperWork`; `consumedWork` omits helper work | `C-INV` | CON |
| B003-b | `helperVisits` substituted for `helperWork` | `C-INV` | CON |
| B004-a | `m=4`, fuel ends at position 2, slice runs 3–4 and charges retrospectively | `C-INV`; the valid slice stops at 2 with full state | CHUNK |
| B004-b | `S_k+1.pre ≠ DP(project(post))`; counters, budget or bounds reduced or reset | `C-INV` | CHUNK |
| B004-c | Projection inside a segment | `C-INV` | CHUNK |
| B005-a | Backend lacks a lookup or field operation | `TOOL UnsupportedFeature`, never `APP` | CON |
| B005-b | `rowGenerated=true` with the component missing | No acceptance | CON |
| B006-a | Residual folds fuel, unrolls recursion or drops a cap check | `TOOL` nonconformance (events differ from generic at depth+1) | CON |
| B006-b | Residual size limit reached | Generic fallback, never `APP-X` | CON |
| B007-a | Arms 1 and 2 both match; witness selects 2 | `C-INV` | REF, CON |
| B007-b | No arm matches | `APP-R(error9)` | REF |
| B007-c | Fields 1 and 2 both fail | Field 1's error; field 2 is never evaluated | REF |
| B007-d | Scrutinee evaluated twice | `C-INV` (charge mismatch) | CON |
| B008-a | Duty dropped or effect reordered on `APP-X` | `C-INV` | FIN |
| B008-b | `TechnicalCut` presented as `Waiting` or commit | `C-INV` | CHUNK, FIN |
| B009-a | Claim lacks profile or evidence class | `PUB-REFUSE` | PUB |
| B009-b | Private path, pin or key in public text | `PUB-REFUSE` | PUB |

## 9. Not claimed

No case has run, and no proof, constraint, model or measurement exists. A2′–A4′ remain proposals until reviewed. `StepInv` preservation, codec laws, `ProjectRel` constraints and every S-ROW remain open obligations.
