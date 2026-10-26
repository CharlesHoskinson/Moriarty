import Clean.Sherlock.CompilerResidualization

/-!
U033 at 8ffed57fc/abc64975d: actual HostOnly K-A residualization, distinct from
historical Python examples. This models the fixed partition, size policy,
immutable folding, dynamic dispatcher and record check. Rust correspondence,
complete runs and byte identities remain explicit metadata obligations.
-/

namespace Clean.Sherlock.CompilerResidualIR

open Clean.Sherlock.CompilerResidualization (InternStepRecord)

inductive Family where
  | schedule | scalar | callBind | frameReturn | dataMatch
  deriving DecidableEq, Repr

inductive BlockId where
  | publish | scalar | scalarCheck | branch | matchData | call | frame | generic
  deriving DecidableEq, Repr

inductive BlockKind where
  | returnBlock | staticDispatch | step | dynamicBranch | runtimeCall
  deriving DecidableEq, Repr

structure Group where
  id : BlockId
  kind : BlockKind
  family : Family
  rules : List Nat
  deriving DecidableEq, Repr

/-- Exact source order: the worklist always keeps a prefix of this partition. -/
def groups : List Group :=
  [⟨.publish, .returnBlock, .schedule, [80, 82, 84]⟩,
   ⟨.scalar, .staticDispatch, .scalar, [102, 108, 114, 117, 120, 145, 149, 153, 157, 223]⟩,
   ⟨.scalarCheck, .step, .scalar, [111, 225]⟩,
   ⟨.branch, .dynamicBranch, .schedule, [123, 161, 165, 169]⟩,
   ⟨.matchData, .dynamicBranch, .dataMatch, [126, 173, 212, 214, 217, 220]⟩,
   ⟨.call, .runtimeCall, .callBind, [129, 131, 177, 181, 200, 202, 204, 206, 208]⟩,
   ⟨.frame, .returnBlock, .frameReturn, [135, 140, 185, 190, 195]⟩]

def ruleInventory : List Nat :=
  [80, 82, 84, 102, 108, 111, 114, 117, 120, 123, 126, 129, 131, 135, 140,
   145, 149, 153, 157, 161, 165, 169, 173, 177, 181, 185, 190, 195, 200,
   202, 204, 206, 208, 212, 214, 217, 220, 223, 225]

theorem partition_covers_inventory_once :
    (groups.flatMap Group.rules).length = ruleInventory.length ∧
      (groups.flatMap Group.rules).Nodup ∧
      ruleInventory.all (fun line ↦ (groups.flatMap Group.rules).contains line) = true := by
  decide

inductive Mode where
  | universal | family (selected : Family) | perProgram
  deriving DecidableEq, Repr

inductive FoldKind where
  | layout | family | code | declarations | sourceOccurrences | operandSlices
  deriving DecidableEq, Repr

/-- Block annotations and top-level bindings differ in the actual JSON. -/
def foldedKinds : Mode → Family → List FoldKind
  | .family selected, family => if selected = family then [.layout, .family] else [.layout]
  | .perProgram, _ => [.code, .layout, .sourceOccurrences]
  | .universal, _ => [.layout]

def staticKinds : Mode → List FoldKind
  | .universal => [.layout]
  | .family _ => [.layout, .family]
  | .perProgram => [.code, .declarations, .layout, .sourceOccurrences, .operandSlices]

def dynamicInputs : List String :=
  ["control", "data", "environment", "frames", "depth", "fuel", "work",
   "helper-cursor", "capacity-checks", "outcome"]

/-- The actual source hashes raw source bytes for this token. No encoding or
hash injectivity theorem is assumed by the contract. -/
def programImageId (sourceHash : String) : Mode → Option String
  | .perProgram => some sourceHash
  | _ => none

theorem universal_has_no_program_binding (sourceHash : String) :
    programImageId sourceHash .universal = none := rfl

theorem family_folds_only_matching_family (selected : Family) :
    foldedKinds (.family selected) selected = [.layout, .family] := by
  simp [foldedKinds]

def keptGroups (limit : Nat) : List Group :=
  if groups.length ≤ limit then groups else groups.take (limit - 1)

/-- RULES is ascending. Filtering its uncovered entries models the sorted
generic_rules vector; equivalence to Rust sorting is a correspondence obligation. -/
def genericRules (limit : Nat) : List Nat :=
  ruleInventory.filter (fun line ↦ !(keptGroups limit).any (fun group ↦ group.rules.contains line))

def presentBlocks (limit : Nat) : List BlockId :=
  (keptGroups limit).map Group.id ++ if (genericRules limit).isEmpty then [] else [.generic]

structure Partition where
  kept : List Group
  genericRules : List Nat
  present : List BlockId
  deriving DecidableEq, Repr

/-- Zero is a tooling configuration refusal, before application execution. -/
def partition (limit : Nat) : Option Partition :=
  if limit = 0 then none
  else some ⟨keptGroups limit, genericRules limit, presentBlocks limit⟩

theorem zero_limit_refused : partition 0 = none := rfl

theorem one_limit_is_entirely_generic :
    partition 1 = some ⟨[], ruleInventory, [.generic]⟩ := by decide

theorem seven_limit_keeps_all_groups :
    partition 7 = some ⟨groups, [], groups.map Group.id⟩ := by decide

/-- Every positive limit emits min(limit,7) blocks, including generic dispatch. -/
def blockCount (limit : Nat) : Nat :=
  if limit = 0 then 0 else min limit groups.length

theorem block_count_bounded (limit : Nat) : blockCount limit ≤ limit := by
  simp only [blockCount]
  split
  · omega
  · exact Nat.min_le_left _ _

theorem large_limit_has_no_fallback (limit : Nat) (enough : groups.length ≤ limit) :
    keptGroups limit = groups ∧ genericRules limit = [] ∧
      presentBlocks limit = groups.map Group.id := by
  have kept : keptGroups limit = groups := by simp [keptGroups, enough]
  have generic : genericRules limit = [] := by
    simp only [genericRules, kept]
    decide
  exact ⟨kept, generic, by simp [presentBlocks, kept, generic]⟩

theorem emitted_block_count_matches_policy (limit : Nat) (positive : 0 < limit) :
    (presentBlocks limit).length = blockCount limit := by
  by_cases enough : groups.length ≤ limit
  · have present := (large_limit_has_no_fallback limit enough).2.2
    rw [present, List.length_map]
    simp only [blockCount, if_neg (Nat.ne_of_gt positive)]
    omega
  · have size : groups.length = 7 := rfl
    have small : limit = 1 ∨ limit = 2 ∨ limit = 3 ∨ limit = 4 ∨ limit = 5 ∨ limit = 6 := by
      omega
    rcases small with rfl | rfl | rfl | rfl | rfl | rfl <;> decide

/-- Source adjacency is a conservative dense graph; publish has no successors. -/
def successors (limit : Nat) (block : BlockId) : List BlockId :=
  if block = .publish then [] else presentBlocks limit

theorem publish_has_no_successors (limit : Nat) : successors limit .publish = [] := rfl

theorem runtime_call_keeps_self_edge : BlockId.call ∈ successors 7 .call := by decide

theorem branch_keeps_both_dynamic_outcomes :
    BlockId.branch ∈ successors 7 .branch ∧ BlockId.scalar ∈ successors 7 .branch := by decide

/-- The source marks calls unrolled=false and frame_check=dynamic. -/
def callUnrolled : Bool := false
def callFrameCheck : String := "dynamic"

theorem recursion_stays_runtime : callUnrolled = false ∧ callFrameCheck = "dynamic" := ⟨rfl, rfl⟩

inductive ControlTag where
  | absent | returned | error
  | evalIf | evalMatch | evalCall | evalUnboundVar | evalOther
  | push | invoke | enter | bound | cases | caseResult | primitiveError | primitiveOk
  deriving DecidableEq, Repr

inductive TopFrame where
  | ifFrame | matchFrame | args | returnFrame | scope | other
  deriving DecidableEq, Repr

/-- These fields must be projected from the actual complete pre-step Snapshot.
This abstraction does not establish that projection or machine determinism. -/
structure DispatchInput where
  control : ControlTag
  framesEmpty : Bool
  fuelZero : Bool
  top : TopFrame

/-- Publication priority precedes the zero-fuel test; all remaining decisions
are dynamic and are made before the supplied machine step. -/
def dispatchGroup (input : DispatchInput) : BlockId :=
  match input.control with
  | .absent | .error => .publish
  | .returned =>
      if input.framesEmpty || input.fuelZero then .publish
      else match input.top with
        | .ifFrame => .branch
        | .matchFrame => .matchData
        | .args => .call
        | .returnFrame | .scope => .frame
        | .other => .scalar
  | control =>
      if input.fuelZero then .publish
      else match control with
        | .evalIf => .branch
        | .evalMatch | .cases | .caseResult => .matchData
        | .evalCall | .invoke | .enter | .bound => .call
        | .evalUnboundVar | .primitiveError => .scalarCheck
        | .push => .frame
        | _ => .scalar

theorem terminal_return_publishes_at_zero_fuel (top : TopFrame) :
    dispatchGroup ⟨.returned, true, true, top⟩ = .publish := rfl

theorem pending_call_at_zero_fuel_publishes (empty : Bool) (top : TopFrame) :
    dispatchGroup ⟨.evalCall, empty, true, top⟩ = .publish := rfl

theorem recursive_invoke_uses_call (empty : Bool) (top : TopFrame) :
    dispatchGroup ⟨.invoke, empty, false, top⟩ = .call := rfl

/-- Rust chooses a group only if some rule is assigned to that group. -/
def selectBlock (present : List BlockId) (input : DispatchInput) : BlockId :=
  if (present.contains (dispatchGroup input)) then dispatchGroup input else .generic

theorem missing_group_falls_back (input : DispatchInput) :
    selectBlock [] input = .generic := rfl

def assignedBlock (limit line : Nat) : BlockId :=
  match (keptGroups limit).find? (fun group ↦ group.rules.contains line) with
  | some group => group.id
  | none => .generic

theorem one_limit_assigns_every_rule_to_generic (line : Nat) :
    assignedBlock 1 line = .generic := rfl

structure ProducedRecord (Whole : Type) where
  block : BlockId
  reference : InternStepRecord Whole

inductive ExecutionRefusal where
  | blockMismatch
  deriving DecidableEq, Repr

/-- The record is produced by the separately supplied machine step. Checking
assignment never changes that record's complete states, charge or rule. -/
def checkRecord {Whole : Type} (assignment : Nat → BlockId) (selected : BlockId)
    (record : InternStepRecord Whole) : Except ExecutionRefusal (ProducedRecord Whole) :=
  if assignment record.ruleStartLine = selected then .ok ⟨selected, record⟩
  else .error .blockMismatch

theorem matching_record_retained_exactly {Whole : Type} (assignment : Nat → BlockId)
    (selected : BlockId) (record : InternStepRecord Whole)
    (matching : assignment record.ruleStartLine = selected) :
    checkRecord assignment selected record = .ok ⟨selected, record⟩ := by
  simp [checkRecord, matching]

theorem mismatched_record_refused {Whole : Type} (assignment : Nat → BlockId)
    (selected : BlockId) (record : InternStepRecord Whole)
    (mismatch : assignment record.ruleStartLine ≠ selected) :
    checkRecord assignment selected record = .error .blockMismatch := by
  simp [checkRecord, mismatch]

/-- One loop boundary, before serialization into width-one slices. The supplied
step must be the actual K-A transition; this model does not implement it. -/
def executeBoundary {Whole : Type} (pending : Whole → Bool)
    (project : Whole → DispatchInput) (step : Whole → Option (InternStepRecord Whole))
    (present : List BlockId) (assignment : Nat → BlockId) (state : Whole) :
    Except ExecutionRefusal (Option (ProducedRecord Whole)) :=
  if pending state then
    match step state with
    | none => .ok none
    | some record =>
        match checkRecord assignment (selectBlock present (project state)) record with
        | .ok produced => .ok (some produced)
        | .error reason => .error reason
  else .ok none

theorem terminal_state_not_stepped {Whole : Type} (pending : Whole → Bool)
    (project : Whole → DispatchInput) (step : Whole → Option (InternStepRecord Whole))
    (present : List BlockId) (assignment : Nat → BlockId) (state : Whole)
    (terminal : pending state = false) :
    executeBoundary pending project step present assignment state = .ok none := by
  simp [executeBoundary, terminal]

theorem executed_record_preserved {Whole : Type} (pending : Whole → Bool)
    (project : Whole → DispatchInput) (step : Whole → Option (InternStepRecord Whole))
    (present : List BlockId) (assignment : Nat → BlockId) (state : Whole)
    (record : InternStepRecord Whole) (live : pending state = true)
    (produced : step state = some record)
    (matching : assignment record.ruleStartLine = selectBlock present (project state)) :
    executeBoundary pending project step present assignment state =
      .ok (some ⟨selectBlock present (project state), record⟩) := by
  simp [executeBoundary, live, produced, checkRecord, matching]

/-- No field of a runtime state participates in the static partition. -/
def partitionAtRuntime {Runtime : Type} (_mode : Mode) (limit : Nat)
    (_runtime : Runtime) : Option Partition := partition limit

theorem partition_ignores_runtime {Runtime : Type} (mode : Mode) (limit : Nat)
    (before after : Runtime) :
    partitionAtRuntime mode limit before = partitionAtRuntime mode limit after := rfl

end Clean.Sherlock.CompilerResidualIR
