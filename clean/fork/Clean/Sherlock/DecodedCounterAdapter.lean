import Clean.Sherlock.NullaryCounterHeadroom

/-!
Source33 revision02, source only. Transparent finite UInt128 arithmetic and an
exact projection of IMAGE counter lanes26,27,28,29,31 into Source32's Nat model.
The prefix is not a complete C6 decoder. Successful checked-operation results
must come from the SAME actual decoded pre-image through a separate Rust
correspondence argument. Positive fuel must come from actual enabled nullary
dispatch; it does not prove dispatch, admission, StepInv or provenance.
P4 and P7 below model successful counter stages separately. No failure rollback,
reachability, field/SDK semantics, Rust refinement or recursive proof is claimed.
Compile only against the independently qualified actual Source32 public module.
-/

namespace Clean.Sherlock.DecodedCounterAdapter

open Clean.Sherlock.NullaryCounterHeadroom

abbrev U128 := Fin laneModulus

/-- Only the five counter values used by Source32; not a complete machine image. -/
structure DecodedCounters where
  initial : U128
  remaining : U128
  consumed : U128
  helper : U128
  reference : U128

/-- A canonical UInt128-valued IMAGE prefix through lane31, not decoder success. -/
abbrev ImageCounterPrefix := Fin 32 → U128

def initialLane : Fin 32 := ⟨26, by decide⟩
def remainingLane : Fin 32 := ⟨27, by decide⟩
def consumedLane : Fin 32 := ⟨28, by decide⟩
def helperLane : Fin 32 := ⟨29, by decide⟩
def referenceLane : Fin 32 := ⟨31, by decide⟩

/-- Literal C6Root counter positions; helperVisits at30 remains outside this slice. -/
def decodeCounterPrefix (image : ImageCounterPrefix) : DecodedCounters :=
  { initial := image initialLane
    remaining := image remainingLane
    consumed := image consumedLane
    helper := image helperLane
    reference := image referenceLane }

def project (u : DecodedCounters) : Counters :=
  { initial := u.initial.val
    remaining := u.remaining.val
    consumed := u.consumed.val
    helper := u.helper.val
    reference := u.reference.val }

/-- Finite arithmetic specification, not an assertion about compiled Rust. -/
def checkedAdd (x y : U128) : Option U128 :=
  if bounded : x.val + y.val < laneModulus then
    some ⟨x.val + y.val, bounded⟩
  else none

/-- Underflow refuses rather than wrapping; successful subtraction is exact. -/
def checkedSub (x y : U128) : Option U128 :=
  if _ordered : y.val ≤ x.val then
    some ⟨x.val - y.val, Nat.sub_lt_of_lt x.isLt⟩
  else none

theorem exact_image_counter_projection (image : ImageCounterPrefix) :
    (project (decodeCounterPrefix image)).initial = (image initialLane).val ∧
    (project (decodeCounterPrefix image)).remaining = (image remainingLane).val ∧
    (project (decodeCounterPrefix image)).consumed = (image consumedLane).val ∧
    (project (decodeCounterPrefix image)).helper = (image helperLane).val ∧
    (project (decodeCounterPrefix image)).reference = (image referenceLane).val := by
  exact ⟨rfl, rfl, rfl, rfl, rfl⟩

theorem checkedAdd_value {x y z : U128}
    (result : checkedAdd x y = some z) : x.val + y.val = z.val := by
  by_cases bounded : x.val + y.val < laneModulus
  · rw [checkedAdd, dif_pos bounded] at result
    exact congrArg (fun v : U128 => v.val) (Option.some.inj result)
  · rw [checkedAdd, dif_neg bounded] at result
    cases result

theorem checkedSub_value {x y z : U128}
    (result : checkedSub x y = some z) :
    y.val ≤ x.val ∧ x.val - y.val = z.val := by
  by_cases ordered : y.val ≤ x.val
  · rw [checkedSub, dif_pos ordered] at result
    exact ⟨ordered, congrArg (fun v : U128 => v.val) (Option.some.inj result)⟩
  · rw [checkedSub, dif_neg ordered] at result
    cases result

theorem checkedAdd_overflow_none (x y : U128)
    (overflow : ¬ x.val + y.val < laneModulus) : checkedAdd x y = none := by
  rw [checkedAdd, dif_neg overflow]

theorem checkedSub_underflow_none (x y : U128)
    (underflow : ¬ y.val ≤ x.val) : checkedSub x y = none := by
  rw [checkedSub, dif_neg underflow]

/-- The finite checked SI03/SI04 counterparts imply the existing Nat premises. -/
theorem accounting_of_checked_results (u : DecodedCounters)
    (fuel : checkedAdd u.remaining u.consumed = some u.initial)
    (cursor : checkedSub u.consumed u.helper = some u.reference) :
    Accounting (project u) := by
  refine ⟨?_, ?_, u.initial.isLt⟩
  · exact checkedAdd_value fuel
  · have difference := checkedSub_value cursor
    change u.reference.val + u.helper.val = u.consumed.val
    rw [← difference.2, Nat.sub_add_cancel difference.1]

/-- Positive fuel is supplied by actual dispatch, never inferred from decoding. -/
theorem headroom_of_checked_results (u : DecodedCounters)
    (fuel : checkedAdd u.remaining u.consumed = some u.initial)
    (cursor : checkedSub u.consumed u.helper = some u.reference)
    (positiveFromActualNullaryDispatch : 0 < u.remaining.val) :
    u.consumed.val < laneModulus - 1 ∧ u.reference.val < laneModulus - 1 := by
  exact headroom_from_accounting (project u)
    (accounting_of_checked_results u fuel cursor) positiveFromActualNullaryDispatch

/-- Same two checked premises applied to the literal five IMAGE positions. -/
theorem image_headroom_of_checked_results (image : ImageCounterPrefix)
    (fuel : checkedAdd (image remainingLane) (image consumedLane) =
      some (image initialLane))
    (cursor : checkedSub (image consumedLane) (image helperLane) =
      some (image referenceLane))
    (positiveFromActualNullaryDispatch : 0 < (image remainingLane).val) :
    (image consumedLane).val < laneModulus - 1 ∧
      (image referenceLane).val < laneModulus - 1 := by
  exact headroom_of_checked_results (decodeCounterPrefix image)
    fuel cursor positiveFromActualNullaryDispatch

/-- Successful non-helper P4 commits work counters, retaining the old reference. -/
def p4WorkCharge (u : DecodedCounters)
    (_positiveFromActualNullaryDispatch : 0 < u.remaining.val)
    (consumedNextBound : u.consumed.val + 1 < laneModulus) : DecodedCounters :=
  { u with
    remaining := ⟨u.remaining.val - 1, Nat.sub_lt_of_lt u.remaining.isLt⟩
    consumed := ⟨u.consumed.val + 1, consumedNextBound⟩ }

/-- Successful P7 separately increments reference; no error rollback is modeled. -/
def p7ReferenceCharge (middle : DecodedCounters)
    (referenceNextBound : middle.reference.val + 1 < laneModulus) : DecodedCounters :=
  { middle with reference := ⟨middle.reference.val + 1, referenceNextBound⟩ }

theorem staged_projection (u : DecodedCounters)
    (positive : 0 < u.remaining.val)
    (consumedNextBound : u.consumed.val + 1 < laneModulus)
    (referenceNextBound : u.reference.val + 1 < laneModulus) :
    project (p7ReferenceCharge (p4WorkCharge u positive consumedNextBound)
      referenceNextBound) = charge (project u) := by
  rfl

/-- Both successful-stage bounds are derived, not extra headroom assumptions.
The intermediate P4 reference is unchanged; Accounting is claimed only after P7. -/
theorem staged_charge_from_checked_results (u : DecodedCounters)
    (fuel : checkedAdd u.remaining u.consumed = some u.initial)
    (cursor : checkedSub u.consumed u.helper = some u.reference)
    (positiveFromActualNullaryDispatch : 0 < u.remaining.val) :
    ∃ (consumedNextBound : u.consumed.val + 1 < laneModulus)
      (referenceNextBound : u.reference.val + 1 < laneModulus),
      (p4WorkCharge u positiveFromActualNullaryDispatch consumedNextBound).reference =
        u.reference ∧
      project (p7ReferenceCharge
        (p4WorkCharge u positiveFromActualNullaryDispatch consumedNextBound)
        referenceNextBound) = charge (project u) ∧
      Accounting (project (p7ReferenceCharge
        (p4WorkCharge u positiveFromActualNullaryDispatch consumedNextBound)
        referenceNextBound)) := by
  have accounting := accounting_of_checked_results u fuel cursor
  have consumedNextBound : u.consumed.val + 1 < laneModulus :=
    consumed_next_bounded (project u) accounting positiveFromActualNullaryDispatch
  have referenceNextBound : u.reference.val + 1 < laneModulus :=
    Nat.add_lt_of_lt_sub
      (headroom_of_checked_results u fuel cursor positiveFromActualNullaryDispatch).2
  refine ⟨consumedNextBound, referenceNextBound, rfl, ?_, ?_⟩
  · exact staged_projection u positiveFromActualNullaryDispatch
      consumedNextBound referenceNextBound
  · rw [staged_projection u positiveFromActualNullaryDispatch
      consumedNextBound referenceNextBound]
    exact charge_preserves_accounting (project u) accounting
      positiveFromActualNullaryDispatch

end Clean.Sherlock.DecodedCounterAdapter
