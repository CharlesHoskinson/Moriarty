import Init

/-!
UIREQ-023/024/031/033: mathematical interfaces for the compiler finish push.
The inventory flags reflect the source-only native inventory at 0699194f9.
Separate plans and residual preservation below are obligations, not extracted
Rust definitions or a proof that missing compiler/backend components exist.
-/

namespace Clean.Sherlock.CompilerResidualization

inductive Family where
  | schedule | scalar | callBind | frameReturn | dataMatch
  deriving DecidableEq, Repr

inductive ConstraintOwner where
  | dispatch | scalar | frame | data
  deriving DecidableEq, Repr

def Family.owner : Family → ConstraintOwner
  | .schedule => .dispatch
  | .scalar => .scalar
  | .callBind | .dataMatch => .data
  | .frameReturn => .frame

structure InventoryRow where
  rule : String
  family : Family
  fragment : Option String
  deriving DecidableEq, Repr

/-- One fixed table serves different programs; their exercised counts remain separate. -/
structure InventoryReport where
  table : List InventoryRow
  program : String
  exercised : List (String × Nat)
  c6RecursionWired : Bool
  nativeStepRecords : Bool
  nativeVerified : Bool
  deriving DecidableEq, Repr

def inventoryReport (table : List InventoryRow) (program : String)
    (exercised : List (String × Nat)) : InventoryReport :=
  ⟨table, program, exercised, false, false, false⟩

theorem inventory_independent_of_program (table : List InventoryRow)
    (p q : String) (xs ys : List (String × Nat)) :
    (inventoryReport table p xs).table = (inventoryReport table q ys).table := rfl

theorem inventory_grants_no_native_evidence (table : List InventoryRow)
    (program : String) (exercised : List (String × Nat)) :
    (inventoryReport table program exercised).c6RecursionWired = false ∧
      (inventoryReport table program exercised).nativeStepRecords = false ∧
      (inventoryReport table program exercised).nativeVerified = false := ⟨rfl, rfl, rfl⟩

/-- The complete observation type must include outcomes, ordered effects, boundaries,
duties, frames, resource checks and semantic work. A final value alone is insufficient. -/
def Preserves {Input Result Observation : Type}
    (observe : Result → Observation) (reference candidate : Input → Result) : Prop :=
  ∀ input, observe (candidate input) = observe (reference input)

/-- Logical host diagnostics are separate from the decoded complete semantic observation. -/
structure Materialization (Observation : Type) where
  decoded : Observation
  evaluatedNodes : Nat
  copiedNodes : Nat
  retainedSlots : Nat
  cacheHits : Nat

theorem materialization_preserves_of_decoded_eq {Input Observation : Type}
    (reference candidate : Input → Materialization Observation)
    (decoded : ∀ input, (candidate input).decoded = (reference input).decoded) :
    Preserves Materialization.decoded reference candidate := decoded

inductive BackendFeature where
  | constraintExport | witnessProgramExport | nativeWitnessClosure
  | lookup | interaction | customGate | fieldLayout
  deriving DecidableEq, Repr

inductive ExportDisposition where
  | supported | unsupported (feature : BackendFeature)
  deriving DecidableEq, Repr

/-- Tooling support is independent per feature; constraint JSON cannot imply witness support. -/
def exportDisposition (supports : BackendFeature → Bool)
    (feature : BackendFeature) : ExportDisposition :=
  if supports feature then .supported else .unsupported feature

theorem unsupported_is_tooling_disposition (supports : BackendFeature → Bool)
    (feature : BackendFeature) (h : supports feature = false) :
    exportDisposition supports feature = .unsupported feature := by
  simp [exportDisposition, h]

theorem constraint_support_does_not_supply_witness :
    ∃ supports : BackendFeature → Bool,
      exportDisposition supports .constraintExport = .supported ∧
      exportDisposition supports .witnessProgramExport = .unsupported .witnessProgramExport :=
  ⟨fun f ↦ decide (f = .constraintExport), rfl, rfl⟩

/-- Distinct plan types and a typed, ordered operation manifest are required inputs.
This structure supplies no exporter or actual backend capability evidence. -/
structure SeparatePlans (ConstraintPlan WitnessPlan Operation : Type) where
  constraints : ConstraintPlan
  witness : WitnessPlan
  orderedOperations : List Operation
  constraintDisposition : ExportDisposition
  witnessDisposition : ExportDisposition

inductive StaticMask where
  | empty | fixedFamily (family : Family) | fixedProgram
  deriving DecidableEq, Repr

/-- Immutable identity tokens only. Equality of tokens does not prove hash injectivity. -/
structure ImmutableFacts where
  profile : String
  layout : String
  program : String
  declarations : String
  deriving DecidableEq, Repr

structure SpecializationKey where
  profile : String
  layout : String
  family : Option Family
  program : Option String
  declarations : Option String
  deriving DecidableEq, Repr

def specializationKey (facts : ImmutableFacts) : StaticMask → SpecializationKey
  | .empty => ⟨facts.profile, facts.layout, none, none, none⟩
  | .fixedFamily f => ⟨facts.profile, facts.layout, some f, none, none⟩
  | .fixedProgram =>
      ⟨facts.profile, facts.layout, none, some facts.program, some facts.declarations⟩

/-- Dynamic state is deliberately not inspected by key construction. Whether a real
compiler obeys this restriction is a separate implementation obligation. -/
def keyAtRuntime {Runtime : Type} (facts : ImmutableFacts) (mask : StaticMask)
    (_runtime : Runtime) : SpecializationKey := specializationKey facts mask

theorem keys_ignore_runtime {Runtime : Type} (facts : ImmutableFacts) (mask : StaticMask)
    (before after : Runtime) : keyAtRuntime facts mask before = keyAtRuntime facts mask after := rfl

structure ResidualBudget where
  worklistLimit : Nat
  sizeLimit : Nat

/-- A candidate retains runtime calls and every internal rule/source stop. Its
preservation proof is required separately; this record does not construct one. -/
structure ResidualCandidate (State Observation : Type) where
  worklistVisited : Nat
  residualSize : Nat
  run : State → Observation

def withinBudget {State Observation : Type} (budget : ResidualBudget)
    (candidate : ResidualCandidate State Observation) : Bool :=
  decide (candidate.worklistVisited ≤ budget.worklistLimit) &&
    decide (candidate.residualSize ≤ budget.sizeLimit)

/-- Exhausting a compiler budget selects the generic operation on the same runtime state. -/
def realize {State Observation : Type} (generic : State → Observation) (budget : ResidualBudget)
    (candidate : Option (ResidualCandidate State Observation)) (state : State) : Observation :=
  match candidate with
  | none => generic state
  | some residual => if withinBudget budget residual then residual.run state else generic state

theorem missing_candidate_falls_back {State Observation : Type}
    (generic : State → Observation) (budget : ResidualBudget) (state : State) :
    realize generic budget none state = generic state := rfl

theorem budget_refusal_falls_back {State Observation : Type}
    (generic : State → Observation) (budget : ResidualBudget)
    (candidate : ResidualCandidate State Observation) (state : State)
    (refused : withinBudget budget candidate = false) :
    realize generic budget (some candidate) state = generic state := by
  simp [realize, refused]

/-- This conditional theorem states the actual preservation obligation. The premise
must cover complete prefixes, not just terminal values or an unverified IR digest. -/
theorem realization_preserves {State Observation : Type}
    (generic : State → Observation) (budget : ResidualBudget)
    (candidate : Option (ResidualCandidate State Observation))
    (sound : ∀ residual, candidate = some residual → ∀ state, residual.run state = generic state)
    (state : State) : realize generic budget candidate state = generic state := by
  cases candidate with
  | none => rfl
  | some residual =>
    simp only [realize]
    split
    · exact sound residual rfl state
    · rfl

end Clean.Sherlock.CompilerResidualization
