import Init

/-!
Source32, source only: a mathematical accounting slice of enabled Nullary03.
The concrete owner checks SI03/SI04 and performs staged P4/P7 updates. Decoding those
checked Rust relations into these natural-number equations remains a separate obligation.
No claim of failure atomicity, circuit/field semantics, reachability or Rust refinement.
-/

namespace Clean.Sherlock.NullaryCounterHeadroom

def laneModulus : Nat := 2 ^ 128

/-- Only the counters used by the accounting deduction, not a complete host-state encoding. -/
structure Counters where
  initial : Nat
  remaining : Nat
  consumed : Nat
  helper : Nat
  reference : Nat

/-- Mathematical content to be obtained from decoded SI03/SI04 and UInt128 initial fuel. -/
structure Accounting (s : Counters) : Prop where
  fuel : s.remaining + s.consumed = s.initial
  referenceBalance : s.reference + s.helper = s.consumed
  initialBound : s.initial < laneModulus

/-- Enabled logical charge only. This is not a model of Rust's intermediate error states. -/
def charge (s : Counters) : Counters :=
  { s with
    remaining := s.remaining - 1
    consumed := s.consumed + 1
    reference := s.reference + 1 }

theorem reference_le_consumed (s : Counters) (accounting : Accounting s) :
    s.reference ≤ s.consumed := by
  calc
    s.reference ≤ s.reference + s.helper := Nat.le_add_right _ _
    _ = s.consumed := accounting.referenceBalance

theorem consumed_next_bounded (s : Counters) (accounting : Accounting s)
    (positive : 0 < s.remaining) : s.consumed + 1 < laneModulus := by
  have next_le_initial : s.consumed + 1 ≤ s.initial := by
    calc
      s.consumed + 1 = 1 + s.consumed := Nat.add_comm _ _
      _ ≤ s.remaining + s.consumed :=
        Nat.add_le_add_right (Nat.succ_le_of_lt positive) _
      _ = s.initial := accounting.fuel
  exact Nat.lt_of_le_of_lt next_le_initial accounting.initialBound

/-- Derives both emitter headroom guards instead of taking either as a premise. -/
theorem headroom_from_accounting (s : Counters) (accounting : Accounting s)
    (positive : 0 < s.remaining) :
    s.consumed < laneModulus - 1 ∧ s.reference < laneModulus - 1 := by
  have consumed_headroom : s.consumed < laneModulus - 1 :=
    Nat.lt_sub_of_add_lt (consumed_next_bounded s accounting positive)
  exact ⟨consumed_headroom,
    Nat.lt_of_le_of_lt (reference_le_consumed s accounting) consumed_headroom⟩

theorem charge_preserves_accounting (s : Counters) (accounting : Accounting s)
    (positive : 0 < s.remaining) : Accounting (charge s) := by
  refine ⟨?_, ?_, accounting.initialBound⟩
  · change s.remaining - 1 + (s.consumed + 1) = s.initial
    rw [Nat.add_comm s.consumed 1, ← Nat.add_assoc,
      Nat.sub_add_cancel (Nat.succ_le_of_lt positive)]
    exact accounting.fuel
  · change (s.reference + 1) + s.helper = s.consumed + 1
    rw [Nat.add_assoc, Nat.add_comm 1 s.helper, ← Nat.add_assoc,
      accounting.referenceBalance]

theorem charge_updated_counters_bounded (s : Counters) (accounting : Accounting s)
    (positive : 0 < s.remaining) :
    (charge s).remaining < laneModulus ∧ (charge s).consumed < laneModulus ∧
      (charge s).reference < laneModulus := by
  have remaining_le_initial : s.remaining ≤ s.initial := by
    calc
      s.remaining ≤ s.remaining + s.consumed := Nat.le_add_right _ _
      _ = s.initial := accounting.fuel
  refine ⟨?_, consumed_next_bounded s accounting positive, ?_⟩
  · exact Nat.sub_lt_of_lt
      (Nat.lt_of_le_of_lt remaining_le_initial accounting.initialBound)
  · exact Nat.add_lt_of_lt_sub (headroom_from_accounting s accounting positive).2

/-- The enabled charge consumes one unit even at the last-fuel boundary. -/
theorem charge_strictly_reduces_remaining (s : Counters) (positive : 0 < s.remaining) :
    (charge s).remaining < s.remaining := by
  change s.remaining - 1 < s.remaining
  calc
    s.remaining - 1 < (s.remaining - 1) + 1 := Nat.lt_succ_self _
    _ = s.remaining := Nat.sub_add_cancel (Nat.succ_le_of_lt positive)

end Clean.Sherlock.NullaryCounterHeadroom
