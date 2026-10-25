import Init

/-!
Public contract for models/sherlock/qa/model_package.py at U021/U026.
This describes task packaging and parameterized correspondence obligations.
It does not implement the private semantic state, Clean circuit or exporter.
-/

namespace Clean.Sherlock.StagePackage

inductive Task where
  | recursiveSemantics | fuelStepRun | boundedEncoding | boundedStepCircuit
  | representationCorrespondence | nativeIntegration
  deriving DecidableEq, Repr

inductive TaskStatus where
  | historicalNarrowFoundation | notImplemented | unproved
  deriving DecidableEq, Repr

def tasks : List Task :=
  [.recursiveSemantics, .fuelStepRun, .boundedEncoding, .boundedStepCircuit,
    .representationCorrespondence, .nativeIntegration]

def taskId : Task → String
  | .recursiveSemantics => "recursive-lean-semantics"
  | .fuelStepRun => "fuel-step-run"
  | .boundedEncoding => "bounded-clean-encoding"
  | .boundedStepCircuit => "bounded-clean-step-circuit"
  | .representationCorrespondence => "semantic-representation-correspondence"
  | .nativeIntegration => "backend-native-integration"

def status : Task → TaskStatus
  | .recursiveSemantics | .fuelStepRun => .historicalNarrowFoundation
  | .boundedEncoding | .boundedStepCircuit | .nativeIntegration => .notImplemented
  | .representationCorrespondence => .unproved

def statusId : TaskStatus → String
  | .historicalNarrowFoundation => "historical-narrow-foundation"
  | .notImplemented => "not-implemented"
  | .unproved => "unproved"

def dependencies : Task → List Task
  | .recursiveSemantics => []
  | .fuelStepRun | .boundedEncoding => [.recursiveSemantics]
  | .boundedStepCircuit => [.fuelStepRun, .boundedEncoding]
  | .representationCorrespondence => [.boundedEncoding, .boundedStepCircuit]
  | .nativeIntegration => [.boundedStepCircuit]

/-- No comparison is distinct from a comparison with no reported divergence. -/
inductive Comparison where
  | notRun
  | supplied (aggregateMismatch pairedMismatch kMismatch quintMismatch hasDifference : Bool)
  deriving DecidableEq, Repr

inductive PackageStatus where
  | packageReady | mismatch
  deriving DecidableEq, Repr

/-- Mirrors stage(): any mismatching component or reported divergence dominates. -/
def packageStatus : Comparison → PackageStatus
  | .notRun => .packageReady
  | .supplied aggregate paired k quint difference =>
    if aggregate || paired || k || quint || difference then .mismatch else .packageReady

/-- The stage entrypoint emits acceptance="none" for every comparison result. -/
def acceptance (_ : Comparison) : String := "none"

theorem package_ready_without_comparison : packageStatus .notRun = .packageReady := rfl

theorem divergence_dominates (aggregate paired k quint : Bool) :
    packageStatus (.supplied aggregate paired k quint true) = .mismatch := by
  cases aggregate <;> cases paired <;> cases k <;> cases quint <;> rfl

theorem comparison_does_not_grant_acceptance (comparison : Comparison) :
    acceptance comparison = "none" := rfl

/-- A partial codec must state both admission and the relation to the complete image. -/
structure CodecObligations (Reference Image : Type) where
  admitted : Reference → Prop
  related : Reference → Image → Prop
  encode : Reference → Option Image
  decode : Image → Option Reference
  encodeAdmitted : ∀ state, admitted state → ∃ image, encode state = some image
  encodeRelated : ∀ state image, encode state = some image → related state image
  decodeRelated : ∀ state image, related state image → decode image = some state

/-- Admitted round trips follow only after an implementation supplies all codec obligations. -/
theorem admitted_round_trip {Reference Image : Type}
    (codec : CodecObligations Reference Image) (state : Reference)
    (admitted : codec.admitted state) :
    ∃ image, codec.encode state = some image ∧ codec.decode image = some state := by
  obtain ⟨image, encoded⟩ := codec.encodeAdmitted state admitted
  exact ⟨image, encoded, codec.decodeRelated state image (codec.encodeRelated state image encoded)⟩

/-- Labeled steps retain the entire ordered work schedule, including zero-work publication.
The state types and transition relations must be instantiated from actual implementations. -/
inductive Trace {State : Type} (step : State → Nat → State → Prop) :
    State → List Nat → State → Prop where
  | nil (state) : Trace step state [] state
  | cons {before middle after work rest} :
      step before work middle → Trace step middle rest after →
      Trace step before (work :: rest) after

/-- A required premise, not an axiom asserting that the Clean implementation refines Lean. -/
def StepSoundness {Reference Image : Type}
    (related : Reference → Image → Prop)
    (semanticStep : Reference → Nat → Reference → Prop)
    (boundedStep : Image → Nat → Image → Prop) : Prop :=
  ∀ state image work nextImage, related state image → boundedStep image work nextImage →
    ∃ nextState, semanticStep state work nextState ∧ related nextState nextImage

/-- Conditional trace lifting preserves every step's charge, rather than only the final value.
It applies to a chosen common schedule; helper-clock erasure needs a separate relation. -/
theorem trace_soundness {Reference Image : Type}
    {related : Reference → Image → Prop}
    {semanticStep : Reference → Nat → Reference → Prop}
    {boundedStep : Image → Nat → Image → Prop}
    (sound : StepSoundness related semanticStep boundedStep)
    {image nextImage : Image} {work : List Nat}
    (trace : Trace boundedStep image work nextImage) :
    ∀ state, related state image →
      ∃ nextState, Trace semanticStep state work nextState ∧ related nextState nextImage := by
  induction trace with
  | nil image => exact fun state relation ↦ ⟨state, .nil state, relation⟩
  | cons transition _ ih =>
    intro state relation
    obtain ⟨middle, semanticTransition, middleRelation⟩ := sound _ _ _ _ relation transition
    obtain ⟨last, semanticTrace, lastRelation⟩ := ih middle middleRelation
    exact ⟨last, .cons semanticTransition semanticTrace, lastRelation⟩

end Clean.Sherlock.StagePackage
