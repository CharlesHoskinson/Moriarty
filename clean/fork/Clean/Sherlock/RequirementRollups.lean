import Init

/-!
Exact C-group dependency contract from qa/sherlock/predicates.yaml.
`U` is an externally supplied proposition for each UIREQ: no predicate execution,
formal refinement or common implementation relation is manufactured by a rollup.
-/

namespace Clean.Sherlock.RequirementRollups

inductive Group where
  | c1 | c2 | c3 | c4 | c5 | c6
  deriving DecidableEq

def dependencies : Group → List Nat
  | .c1 => [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
  | .c2 => [13, 14, 15, 16, 32]
  | .c3 => [22, 27, 31, 34, 35]
  | .c4 => [23, 24, 28, 29, 30, 33]
  | .c5 => [17, 18, 19, 20]
  | .c6 => [21, 25, 26]

def Rollup (U : Nat → Prop) (group : Group) : Prop :=
  ∀ id, id ∈ dependencies group → U id

theorem component_required {U : Nat → Prop} {group : Group}
    (rollup : Rollup U group) {id : Nat} (member : id ∈ dependencies group) : U id := by
  exact rollup id member

theorem missing_component_blocks {U : Nat → Prop} {group : Group} {id : Nat}
    (member : id ∈ dependencies group) (missing : ¬ U id) : ¬ Rollup U group := by
  exact fun rollup ↦ missing (rollup id member)

theorem c1_iff (U : Nat → Prop) :
    Rollup U .c1 ↔ U 1 ∧ U 2 ∧ U 3 ∧ U 4 ∧ U 5 ∧ U 6 ∧
      U 7 ∧ U 8 ∧ U 9 ∧ U 10 ∧ U 11 ∧ U 12 := by
  simp [Rollup, dependencies]

theorem c2_iff (U : Nat → Prop) :
    Rollup U .c2 ↔ U 13 ∧ U 14 ∧ U 15 ∧ U 16 ∧ U 32 := by
  simp [Rollup, dependencies]

theorem c3_iff (U : Nat → Prop) :
    Rollup U .c3 ↔ U 22 ∧ U 27 ∧ U 31 ∧ U 34 ∧ U 35 := by
  simp [Rollup, dependencies]

theorem c4_iff (U : Nat → Prop) :
    Rollup U .c4 ↔ U 23 ∧ U 24 ∧ U 28 ∧ U 29 ∧ U 30 ∧ U 33 := by
  simp [Rollup, dependencies]

theorem c5_iff (U : Nat → Prop) :
    Rollup U .c5 ↔ U 17 ∧ U 18 ∧ U 19 ∧ U 20 := by
  simp [Rollup, dependencies]

theorem c6_iff (U : Nat → Prop) :
    Rollup U .c6 ↔ U 21 ∧ U 25 ∧ U 26 := by
  simp [Rollup, dependencies]

end Clean.Sherlock.RequirementRollups
