### A1. UISEC-004 overflow arm (all named profiles)

Frame-cap overflow is terminal and explicit. It is never success, and it never truncates frames, effects or duties. Each frozen profile keeps its own spelling:

| Profile | Frame overflow | `resourceLimit(o)` |
|---|---|---|
| `sherlock-k-a/1` | `Rejected(error4)` | `Exhausted(_) ∨ Rejected(error4)` |
| `sherlock-quint-a/1` | `pushFrame` with `depth > MAX_FRAMES` ⇒ `refuse(s,StackBound)` | `phase=Exhausted ∨ (phase=Rejected ∧ reason=StackBound)` |
| `sherlock-helper-microstep/1` | `Rejected(error4)`: a charged failing push, then zero-cost error publication | `Exhausted(_) ∨ Rejected(error4)` |

Only these arms are resourceLimit. `ScalarBound`, `ConstructorMismatch`, `MissingReference` and all other errors are not. No consumer SHALL read a resourceLimit outcome as a semantic, typing or financial rejection. UISEC-007 and B008 hold under it.

Frozen sources and their charges are unchanged. Two agreements are differential-predicate obligations (OSR-1014-02), not assumptions:
- the cap boundary (Quint uses `>`);
- the overflow charge.

Rationale: security needs explicitness, terminality and charging per profile, not a shared constructor name.

### Handoff updates

**`interfaces.md`**
- Replace `isResourceLimit(Outcome) -> Bool` with `isResourceLimit(ProfileId, Observation) -> Bool`. It is defined by the A1 table and total over the three named profiles.
- Add `stepCap: Nat` to `Bounds`. It is the B per-step charge capacity.
- Add `admit(program, bounds, layout) -> Result<Admitted, CapacityNotAdmitted>`. It checks `stepCap ≥ 1` and the counter widths for `maxReferenceWork` and `maxHelperWork`.
- `CapacityNotAdmitted` is an admission error, never an `Outcome`.
- In the error table, mark K/B error4 and Quint `StackBound` as ResourceLimit.

**`tasks.md:16`**: "Frame-cap overflow: error4 (K-a, B); StackBound (Quint-a, frozen); resourceLimit per A1 table."

**Phase 0 gate**: add "`Bounds.stepCap`, the `admit` signature and the per-profile `isResourceLimit` table".
