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

/-! U024 at 69aeb882d: the candidate interns trees in the A profile only.
These are semantic contents and abstract canonical allocation IDs; they do not
encode Arc pointers or prove the Rust HashMap/work-stack invariant. -/

inductive InternTree where
  | leaf (value : Fin (2 ^ 128))
  | branch (left right : InternTree)
  deriving DecidableEq

inductive InternNodeKey where
  | leaf (value : Fin (2 ^ 128))
  | branch (leftId rightId : Nat)
  deriving DecidableEq

def internKeyMeaning (content : Nat → InternTree) : InternNodeKey → InternTree
  | .leaf value => .leaf value
  | .branch left right => .branch (content left) (content right)

/-- A live canonical ID must have a stable semantic content. Equal keys then
cannot merge unequal trees; proving stable Rust allocation IDs is separate. -/
theorem equal_intern_keys_preserve_content (content : Nat → InternTree)
    (left right : InternNodeKey) (same : left = right) :
    internKeyMeaning content left = internKeyMeaning content right := congrArg _ same

/-- These equations are the table/work-stack invariant to establish for the
bottom-up Rust interner. Given them, every finite input tree retains its content. -/
theorem bottom_up_interning_preserves_content (intern : InternTree → InternTree)
    (leaf : ∀ value, intern (.leaf value) = .leaf value)
    (branch : ∀ left right, intern (.branch left right) = .branch (intern left) (intern right))
    (tree : InternTree) : intern tree = tree := by
  induction tree with
  | leaf value => exact leaf value
  | branch left right leftPreserved rightPreserved =>
    rw [branch, leftPreserved, rightPreserved]

/-- Exactly the compared StepRecord fields in optimization_intern.rs.
`Whole` must be the complete structural A snapshot, including its program. -/
structure InternStepRecord (Whole : Type) where
  ruleStartLine : Nat
  charge : Nat
  before : Whole
  after : Whole
  deriving DecidableEq

structure InternObservation (Whole : Type) where
  initial : Whole
  records : List (InternStepRecord Whole)
  finalState : Whole
  deriving DecidableEq

/-- Mirrors the acceptance predicate, not its first-divergence diagnostic order.
The A profile has no financial effect/duty state; no effectful claim is implied. -/
theorem intern_observation_preservation_iff {Whole : Type}
    (reference candidate : InternObservation Whole) :
    reference = candidate ↔ reference.initial = candidate.initial ∧
      reference.records = candidate.records ∧ reference.finalState = candidate.finalState := by
  cases reference
  cases candidate
  simp only [InternObservation.mk.injEq]

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

/-! U031/U033 at cfbab29c5 and a82446698: finite Python reporting models, not Rust
exporters, residual execution or native artifacts. -/

/-- The domain is a model annotation, not a verified operand binding or field encoding. -/
structure ToolingOperation where
  operationId : String
  domain : String

structure ToolingWitnessNode where
  index : Nat
  operation : ToolingOperation
  typedInputs : List (String × String)
  typedOutputs : List (String × String)
  dependencies : List Nat

/-- a82446698 emits one generic in/out label per operation and a preceding-node
dependency. This does not bind a real Sherlock step's operands or result values. -/
def toolingWitnessNode (index : Nat) (operation : ToolingOperation) : ToolingWitnessNode :=
  ⟨index, operation, [("in", operation.domain)], [("out", operation.domain)],
    if index = 0 then [] else [index - 1]⟩

theorem tooling_witness_domain_annotations (index : Nat) (operation : ToolingOperation) :
    (toolingWitnessNode index operation).typedInputs = [("in", operation.domain)] ∧
      (toolingWitnessNode index operation).typedOutputs = [("out", operation.domain)] := ⟨rfl, rfl⟩

inductive ToolingBackend where
  | constraintJsonBasic | structuredWitnessBasic | midnightZkir
  deriving DecidableEq, Repr

/-- Per-feature primitive capability, distinct from aggregate complete-plan export. -/
def toolingSupports (backend : ToolingBackend) : BackendFeature → Bool
  | .constraintExport => decide (backend ≠ .midnightZkir)
  | .witnessProgramExport => decide (backend = .structuredWitnessBasic)
  | .nativeWitnessClosure | .lookup | .interaction | .customGate => false
  | .fieldLayout => true

/-- The fixed STEP manifest always contains native closure, lookup and interaction
operations. Hence neither complete plan can be exported by any listed backend. -/
def toolingWholePlanExport (_backend : ToolingBackend) : ExportDisposition :=
  .unsupported .nativeWitnessClosure

theorem tooling_native_closures_unsupported (backend : ToolingBackend) :
    exportDisposition (toolingSupports backend) .nativeWitnessClosure =
      .unsupported .nativeWitnessClosure := rfl

theorem tooling_constraint_json_has_no_witness :
    exportDisposition (toolingSupports .constraintJsonBasic) .witnessProgramExport =
      .unsupported .witnessProgramExport := rfl

def toolingAcceptance (_backend : ToolingBackend) : String := "none"

theorem tooling_reports_grant_no_acceptance (backend : ToolingBackend) :
    toolingAcceptance backend = "none" := rfl

inductive ModelRule where
  | fetch | condition | thenBranch | elseBranch | call
  deriving DecidableEq, Repr

/-- a82446698 preserves all possible targets; static binding controls which
target is specialized, while other targets retain generic runtime dispatch. -/
def modelTargets (_arithOnly : Bool) : ModelRule → List ModelRule
  | .fetch => [.condition, .call]
  | .condition => [.thenBranch, .elseBranch]
  | .call => [.fetch]
  | .thenBranch | .elseBranch => []

def modelGenericTargets (arithOnly : Bool) : ModelRule → List ModelRule
  | .fetch => if arithOnly then [.call] else []
  | _ => []

/-- Historical cfbab29c5 whitelist: omitted runtime targets were dropped. -/
def legacyModelTargets (arithOnly : Bool) (rule : ModelRule) : List ModelRule :=
  match rule with
  | .fetch => if arithOnly then [.condition] else [.condition, .call]
  | other => modelTargets arithOnly other

theorem model_dynamic_branch_keeps_both_targets (arithOnly : Bool) :
    modelTargets arithOnly .condition = [.thenBranch, .elseBranch] := rfl

theorem model_recursive_call_keeps_back_edge (arithOnly : Bool) :
    modelTargets arithOnly .call = [.fetch] := rfl

/-- Keep the earlier negative witness, explicitly pinned to the historical model. -/
theorem legacy_model_arith_prunes_call :
    ModelRule.call ∈ legacyModelTargets false .fetch ∧
      ModelRule.call ∉ legacyModelTargets true .fetch := by decide

theorem model_arith_keeps_call_via_generic :
    ModelRule.call ∈ modelTargets true .fetch ∧
      ModelRule.call ∈ modelGenericTargets true .fetch := by decide

/-! U033 contract abe5ee146: required real-IR provenance. These are obligations
for the forthcoming Rust executor, separate from the synthetic Python graph. -/

structure ResidualBlock where
  id : Nat
  rules : List Nat
  successors : List Nat
  genericDispatch : Bool

structure ResidualRecord (Whole : Type) where
  blockId : Nat
  reference : InternStepRecord Whole

/-- The executor is supplied from the actual implementation. A matching annotation
or a copied reference stream is not evidence for this production equation. -/
def RecordProduced {Whole : Type} (lookup : Nat → Option ResidualBlock)
    (execute : ResidualBlock → Whole → Option (InternStepRecord Whole))
    (record : ResidualRecord Whole) : Prop :=
  ∃ block, lookup record.blockId = some block ∧ block.id = record.blockId ∧
    (block.genericDispatch = true ∨ record.reference.ruleStartLine ∈ block.rules) ∧
    execute block record.reference.before = some record.reference

def AllowedBlockEdge (lookup : Nat → Option ResidualBlock) (current next : Nat) : Prop :=
  ∃ block, lookup current = some block ∧
    (block.genericDispatch = true ∨ next = current ∨ next ∈ block.successors)

/-- Complete observed records must actually be produced, follow IR edges and
chain through the same complete states. Concrete lookup, decoding and execution
correspondence still have to be proved; this record constructs no Rust executor. -/
structure ResidualExecution {Whole : Type} (lookup : Nat → Option ResidualBlock)
    (execute : ResidualBlock → Whole → Option (InternStepRecord Whole)) where
  records : List (ResidualRecord Whole)
  production : ∀ record, record ∈ records → RecordProduced lookup execute record
  adjacency : ∀ index current next,
    records[index]? = some current → records[index + 1]? = some next →
      AllowedBlockEdge lookup current.blockId next.blockId
  continuity : ∀ index current next,
    records[index]? = some current → records[index + 1]? = some next →
      current.reference.after = next.reference.before

theorem residual_record_has_execution_witness {Whole : Type}
    (lookup : Nat → Option ResidualBlock)
    (execute : ResidualBlock → Whole → Option (InternStepRecord Whole))
    (run : ResidualExecution lookup execute) (record : ResidualRecord Whole)
    (member : record ∈ run.records) :
    ∃ block, lookup record.blockId = some block ∧
      execute block record.reference.before = some record.reference := by
  obtain ⟨block, found, _, _, produced⟩ := run.production record member
  exact ⟨block, found, produced⟩

/-- Negative control: a reference stream cannot become a genuine nonempty IR
execution merely by adding block IDs when the supplied executor emits nothing. -/
theorem refusing_executor_has_no_records {Whole : Type}
    (lookup : Nat → Option ResidualBlock)
    (run : ResidualExecution (Whole := Whole) lookup (fun _ _ ↦ none)) :
    run.records = [] := by
  cases recordsEq : run.records with
  | nil => rfl
  | cons record rest =>
    obtain ⟨_, _, _, _, impossible⟩ := run.production record (by simp [recordsEq])
    cases impossible

inductive StaticMask where
  | empty | fixedFamily (family : String) | fixedProgram
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
  family : Option String
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
