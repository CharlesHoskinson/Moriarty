import Clean.Sherlock.DecodedCounterAdapter

/-!
Counter-only extension for helper_machine/shared/owner_runtime.rs P4/P6/P7.
P3 leaves all counters unchanged. P4 charges remaining/consumed exactly once,
and charges helper work only for helper-eligible selection. A P6 cap refusal
retains P4 without P7. These definitions take successful arithmetic bounds;
they neither select a rule nor establish full StepInv or Rust correspondence.
The original adapter and its historical metadata remain unchanged.
-/

namespace Clean.Sherlock.DecodedCounterStages

open Clean.Sherlock.NullaryCounterHeadroom
open Clean.Sherlock.DecodedCounterAdapter

/-- The extra helper increment at P4; reference advances only for LegacyComplete at P7.
Helper visits at IMAGE lane30 are outside this five-counter model. -/
def p4HelperCharge (u : DecodedCounters)
    (_positiveFromActualDispatch : 0 < u.remaining.val)
    (consumedNextBound : u.consumed.val + 1 < laneModulus)
    (helperNextBound : u.helper.val + 1 < laneModulus) : DecodedCounters :=
  { u with
    remaining := ⟨u.remaining.val - 1, Nat.sub_lt_of_lt u.remaining.isLt⟩
    consumed := ⟨u.consumed.val + 1, consumedNextBound⟩
    helper := ⟨u.helper.val + 1, helperNextBound⟩ }

theorem helper_next_bounded (u : DecodedCounters)
    (accounting : Accounting (project u)) (positive : 0 < u.remaining.val) :
    u.helper.val + 1 < laneModulus := by
  have helper_le : u.helper.val ≤ u.consumed.val := by
    calc
      u.helper.val ≤ u.reference.val + u.helper.val := Nat.le_add_left _ _
      _ = u.consumed.val := accounting.referenceBalance
  exact Nat.lt_of_le_of_lt (Nat.add_le_add_right helper_le 1)
    (consumed_next_bounded (project u) accounting positive)

/-- Non-helper P4 preserves fuel accounting even when P6 refuses the selected work. -/
theorem p4_nonhelper_preserves_fuel (u : DecodedCounters)
    (accounting : Accounting (project u)) (positive : 0 < u.remaining.val)
    (consumedNextBound : u.consumed.val + 1 < laneModulus) :
    (project (p4WorkCharge u positive consumedNextBound)).remaining +
      (project (p4WorkCharge u positive consumedNextBound)).consumed = u.initial.val := by
  change u.remaining.val - 1 + (u.consumed.val + 1) = u.initial.val
  rw [Nat.add_comm u.consumed.val 1, ← Nat.add_assoc,
    Nat.sub_add_cancel (Nat.succ_le_of_lt positive)]
  exact accounting.fuel

/-- A charged non-helper cap refusal has no P7 reference increment.
It therefore does not preserve this adapter's reference-balance equation. -/
theorem p4_nonhelper_not_accounting (u : DecodedCounters)
    (accounting : Accounting (project u)) (positive : 0 < u.remaining.val)
    (consumedNextBound : u.consumed.val + 1 < laneModulus) :
    ¬ Accounting (project (p4WorkCharge u positive consumedNextBound)) := by
  intro post
  have impossible : u.consumed.val = u.consumed.val + 1 :=
    accounting.referenceBalance.symm.trans post.referenceBalance
  exact Nat.ne_of_lt (Nat.lt_succ_self _) impossible

/-- Helper P4 increments consumed and helper together, preserving both equations.
The result also describes the counter slice retained by a helper P6 cap refusal. -/
theorem p4_helper_preserves_accounting (u : DecodedCounters)
    (accounting : Accounting (project u)) (positive : 0 < u.remaining.val)
    (consumedNextBound : u.consumed.val + 1 < laneModulus)
    (helperNextBound : u.helper.val + 1 < laneModulus) :
    Accounting (project (p4HelperCharge u positive consumedNextBound helperNextBound)) := by
  refine ⟨?_, ?_, accounting.initialBound⟩
  · change u.remaining.val - 1 + (u.consumed.val + 1) = u.initial.val
    exact p4_nonhelper_preserves_fuel u accounting positive consumedNextBound
  · change u.reference.val + (u.helper.val + 1) = u.consumed.val + 1
    rw [← Nat.add_assoc]
    exact congrArg (fun n : Nat ↦ n + 1) accounting.referenceBalance

end Clean.Sherlock.DecodedCounterStages
