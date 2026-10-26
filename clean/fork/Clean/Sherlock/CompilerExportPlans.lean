import Init

/-!
U031: source-level contract of sherlock/export_plans.rs at 97412cb2c.
This is the real selected-record host exporter, separate from the earlier
Python reporting model. Classification and data-shape laws below do not prove
Rust/JSON correspondence, executable constraints/witnesses or native acceptance.
-/

namespace Clean.Sherlock.CompilerExportPlans

def profileId : String := "sherlock-k-a/1"
def layoutVersion : String := "sherlock-layout/1"
def fieldId : String := "host-u128/1"
def backendId : String := "sherlock-host-export/1"

inductive Request where
  | constraintJson | structuredWitness | nativeWitnessClosure
  | lookup | interaction | customGate
  deriving DecidableEq, Repr

def requestName : Request → String
  | .constraintJson => "constraint-json"
  | .structuredWitness => "structured-witness"
  | .nativeWitnessClosure => "native-witness-closure"
  | .lookup => "lookup"
  | .interaction => "interaction"
  | .customGate => "custom-gate"

def parseRequest : String → Option Request
  | "constraint-json" => some .constraintJson
  | "structured-witness" => some .structuredWitness
  | "native-witness-closure" => some .nativeWitnessClosure
  | "lookup" => some .lookup
  | "interaction" => some .interaction
  | "custom-gate" => some .customGate
  | _ => none

theorem request_name_roundtrip (request : Request) :
    parseRequest (requestName request) = some request := by
  cases request <;> rfl

def requestOrder : List Request :=
  [.constraintJson, .structuredWitness, .nativeWitnessClosure, .lookup, .interaction, .customGate]

/-- Source order is canonical and independent of request repetition/order. -/
def normalizeRequests (requests : List Request) : List Request :=
  requestOrder.filter (fun request ↦ decide (request ∈ requests))

theorem duplicate_request_emitted_once (request : Request) :
    normalizeRequests [request, request] = [request] := by
  cases request <;> decide

/-- The supplied list is the exact K-A record stream, not an arbitrary program
oracle. Connecting it to select_record/advance_m is a separate obligation. -/
def selectRecord {Record : Type} (records : List Record) (position : Nat) : Option Record :=
  if position = 0 then none else records[position - 1]?

theorem zero_position_refused {Record : Type} (records : List Record) :
    selectRecord records 0 = none := rfl

theorem first_position_is_head {Record : Type} (records : List Record) :
    selectRecord records 1 = records.head? := by
  cases records <;> rfl

inductive BinaryKind where
  | add | sub | mul | eq | lt | cons | branch
  deriving DecidableEq, Repr

inductive UnaryKind where
  | booleanNot | leaf
  deriving DecidableEq, Repr

inductive FrameKind where
  | right (operation : BinaryKind)
  | returnFrame | scope | unary (operation : UnaryKind) | ifFrame | other
  deriving DecidableEq, Repr

inductive OperationKind where
  | checkedAdd | checkedSub | checkedMul | uint128EqLabel | uint128Lt
  | listCons | treeBranch | frameRestore | scopeRestore | boolNot | treeLeaf
  | select | arenaRead
  deriving DecidableEq, Repr

/-- The current Eq label is mirrored literally, not endorsed for non-UInt
operands. K-A Eq accepts arbitrary sorts; the source-label gap remains open. -/
def binaryOperation : BinaryKind → OperationKind
  | .add => .checkedAdd
  | .sub => .checkedSub
  | .mul => .checkedMul
  | .eq => .uint128EqLabel
  | .lt => .uint128Lt
  | .cons => .listCons
  | .branch => .treeBranch

def operationName : OperationKind → String
  | .checkedAdd => "CheckedU128Add"
  | .checkedSub => "CheckedU128Sub"
  | .checkedMul => "CheckedU128Mul"
  | .uint128EqLabel => "UInt128Eq"
  | .uint128Lt => "UInt128Lt"
  | .listCons => "ListCons"
  | .treeBranch => "TreeBranch"
  | .frameRestore => "FrameRestore"
  | .scopeRestore => "ScopeRestore"
  | .boolNot => "BoolNot"
  | .treeLeaf => "TreeLeaf"
  | .select => "Select"
  | .arenaRead => "ArenaRead"

/-- Only the current rule, returned control and top frame classify the step.
Frames below the top are pending continuations, not the current operation. -/
def classify (rule : Nat) (returned : Bool) (frames : List FrameKind) : OperationKind :=
  if !returned then .arenaRead
  else match frames.head? with
    | some (.right operation) => if rule = 153 then binaryOperation operation else .arenaRead
    | some .returnFrame => if rule = 185 then .frameRestore else .arenaRead
    | some .scope => if rule = 195 then .scopeRestore else .arenaRead
    | some (.unary operation) =>
        if rule = 157 then
          match operation with
          | .booleanNot => .boolNot
          | .leaf => .treeLeaf
        else .arenaRead
    | some .ifFrame => if rule = 161 ∨ rule = 165 then .select else .arenaRead
    | _ => .arenaRead

theorem classification_ignores_pending_frames (rule : Nat) (returned : Bool)
    (top : FrameKind) (first second : List FrameKind) :
    classify rule returned (top :: first) = classify rule returned (top :: second) := rfl

theorem checked_mul_classifies_rule153 (pending : List FrameKind) :
    classify 153 true (.right .mul :: pending) = .checkedMul := rfl

theorem return_frame_classifies_rule185 (pending : List FrameKind) :
    classify 185 true (.returnFrame :: pending) = .frameRestore := rfl

theorem pending_return_frame_is_not_current (rule : Nat) (returned : Bool)
    (pending : List FrameKind) :
    classify rule returned (.other :: .returnFrame :: pending) = .arenaRead := by
  cases returned <;> rfl

inductive BindingSort where
  | uint128 | bool | uint128List | uint128Tree | control
  deriving DecidableEq, Repr

structure Atom where
  sort : BindingSort
  display : String

structure Binding where
  name : String
  sort : BindingSort
  source : Option String
  value : Option String

inductive PrimitiveOutcome where
  | success (atom : Atom) | error (code : Nat) | notPrimitive

/-- Mirrors the current fallback: errors omit a value/error variant and expose
only a UInt128 type placeholder. This is a recorded limitation, not a proof that
the placeholder is a valid result of the selected reference operation. -/
def resultBinding : PrimitiveOutcome → Binding
  | .success atom =>
      ⟨"result", atom.sort, some "after.control.primitive_result", some atom.display⟩
  | .error _ | .notPrimitive => ⟨"result", .uint128, none, none⟩

theorem error_result_has_no_bound_value (code : Nat) :
    (resultBinding (.error code)).value = none ∧
      (resultBinding (.error code)).source = none := ⟨rfl, rfl⟩

structure Identified (Body : Type) where
  body : Body
  id : String

/-- The caller supplies canonical JSON bytes without this object's own ID.
No SHA injectivity or correctness of the actual JSON encoder is assumed. -/
def identify {Body : Type} (canonical : Body → List UInt8) (hash : List UInt8 → String)
    (body : Body) : Identified Body := ⟨body, hash (canonical body)⟩

theorem identity_uses_canonical_body {Body : Type}
    (canonical : Body → List UInt8) (hash : List UInt8 → String) (body : Body) :
    (identify canonical hash body).id = hash (canonical body) := rfl

theorem reidentification_ignores_prior_id {Body : Type}
    (canonical : Body → List UInt8) (hash : List UInt8 → String)
    (body : Body) (first second : String) :
    identify canonical hash (Identified.mk body first).body =
      identify canonical hash (Identified.mk body second).body := rfl

inductive SupportStatus where
  | specified | implemented | unsupported (reason : String)

structure Operation where
  id : String
  kind : OperationKind
  ordering : Nat
  inputs : List Binding
  outputs : List Binding
  sourceLocators : List String
  componentMembership : String
  constraintComponentLocator : Option String
  witnessDependencies : List String
  witnessProgramKind : String
  status : SupportStatus

structure OperationManifest where
  referenceDefinitionId : String
  profileId : String
  layoutVersion : String
  operations : List Operation
  omissions : List String

structure WitnessNode where
  operationId : String
  inputs : List Binding
  outputs : List Binding
  dependencies : List String
  lastUse : Bool

def nodeFor (operation : Operation) (dependencies : List String) (lastUse : Bool) : WitnessNode :=
  ⟨operation.id, operation.inputs, operation.outputs, dependencies, lastUse⟩

theorem witness_node_keeps_typed_bindings (operation : Operation)
    (dependencies : List String) (lastUse : Bool) :
    (nodeFor operation dependencies lastUse).inputs = operation.inputs ∧
      (nodeFor operation dependencies lastUse).outputs = operation.outputs := ⟨rfl, rfl⟩

structure ConstraintPlan where
  manifestId : String
  layoutVersion : String
  fieldId : String
  publicInputs : List Binding
  privateInputs : List Binding
  outputs : List Binding
  componentOperationIds : List String
  requiredCapabilities : List String
  omissions : List String

structure WitnessPlan where
  manifestId : String
  layoutVersion : String
  inputs : List Binding
  nodes : List (Identified WitnessNode)
  order : List String
  outputs : List Binding
  slots : Nat
  programKind : String

def shareManifest (id : String) (constraints : ConstraintPlan) (witness : WitnessPlan) :
    ConstraintPlan × WitnessPlan :=
  ({ constraints with manifestId := id }, { witness with manifestId := id })

theorem separate_plans_share_manifest (id : String) (constraints : ConstraintPlan)
    (witness : WitnessPlan) :
    (shareManifest id constraints witness).1.manifestId = id ∧
      (shareManifest id constraints witness).2.manifestId = id := ⟨rfl, rfl⟩

structure Artifact where
  path : String
  bytes : List UInt8
  sha256 : String

/-- The artifact hashes its complete serialized plan, including plan ID.
Its hash is distinct in purpose from identify's hash of the body without ID. -/
def artifactFor {Payload : Type} (serialize : Payload → List UInt8)
    (hash : List UInt8 → String) (path : String) (payload : Payload) : Artifact :=
  let bytes := serialize payload
  ⟨path, bytes, hash bytes⟩

theorem artifact_hash_covers_emitted_bytes {Payload : Type}
    (serialize : Payload → List UInt8) (hash : List UInt8 → String)
    (path : String) (payload : Payload) :
    (artifactFor serialize hash path payload).sha256 =
      hash (artifactFor serialize hash path payload).bytes := rfl

/-- Planned outcomes assuming serialization and writes succeed; filesystem
failure is separate and no atomic multi-artifact write is claimed. -/
def artifactPath : Request → Option String
  | .constraintJson => some "constraint-plan.json"
  | .structuredWitness => some "witness-plan.json"
  | _ => none

def supported (request : Request) : Bool := (artifactPath request).isSome

theorem only_two_request_formats_supported (request : Request) :
    supported request = true ↔ request = .constraintJson ∨ request = .structuredWitness := by
  cases request <;> decide

/-- Reported capability, not a proof that an exported DAG computes the witness. -/
def declaredWitnessComputation : Request → Bool
  | .structuredWitness => true
  | _ => false

def declaredMidnightExporter (_request : Request) : Bool := false

def fieldLayoutSupported : Bool := false

theorem constraint_json_declares_no_witness :
    declaredWitnessComputation .constraintJson = false := rfl

theorem no_midnight_exporter (request : Request) : declaredMidnightExporter request = false := rfl

theorem host_exporter_has_no_field_layout : fieldLayoutSupported = false := rfl

/-- Unsupported is a tooling result beside the same application object.
This wrapper does not prove actual CLI or application refinement. -/
def attachExport {Application : Type} (application : Application) (request : Request) :
    Application × Bool := (application, supported request)

theorem tooling_disposition_keeps_application {Application : Type}
    (application : Application) (request : Request) :
    (attachExport application request).1 = application := rfl

end Clean.Sherlock.CompilerExportPlans
