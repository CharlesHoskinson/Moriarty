import Init

/-!
UIREQ-017--020: the host profile /2 financial publication boundary.
This module mirrors `stage/sherlock2.rs`, not the older read-only Core/2 adapter.
Snapshots, captures and effects remain parameters: producing their exact Rust
projection, admission, statement evaluation and target phases are separate obligations.
No ledger rollback, circuit correspondence or stock-verifier acceptance is inferred.
-/

namespace Clean.Sherlock.FinancialItems

inductive PhasePolicy where
  | allOrNothing
  | retainGuaranteedFees
  deriving DecidableEq

/-- Syntactic classes used by `check_policy`, not statement evaluation outcomes. -/
inductive StatementClass where
  | fee
  | letBinding
  | precondition
  | other

def segmentFeesFirstFrom : Bool → List StatementClass → Bool
  | _, [] => true
  | seenOther, .fee :: rest =>
    if seenOther then false else segmentFeesFirstFrom false rest
  | seenOther, .letBinding :: rest => segmentFeesFirstFrom seenOther rest
  | seenOther, .precondition :: rest => segmentFeesFirstFrom seenOther rest
  | _, .other :: rest => segmentFeesFirstFrom true rest

def segmentFeesFirst (statements : List StatementClass) : Bool :=
  segmentFeesFirstFrom false statements

/-- Start checks every segment, including segments never reached by execution. -/
def admitActionPolicy : PhasePolicy → List (List StatementClass) → Bool
  | .allOrNothing, _ => true
  | .retainGuaranteedFees, segments => segments.all segmentFeesFirst

theorem all_or_nothing_admission_unrestricted (segments : List (List StatementClass)) :
    admitActionPolicy .allOrNothing segments = true := by
  rfl

theorem retain_admission_iff_all_segments (segments : List (List StatementClass)) :
    admitActionPolicy .retainGuaranteedFees segments = true ↔
      ∀ segment, segment ∈ segments → segmentFeesFirst segment = true := by
  simp [admitActionPolicy]

theorem bad_segment_blocks_retain_admission (segments : List (List StatementClass))
    (segment : List StatementClass) (member : segment ∈ segments)
    (bad : segmentFeesFirst segment = false) :
    admitActionPolicy .retainGuaranteedFees segments = false := by
  cases accepted : admitActionPolicy .retainGuaranteedFees segments with
  | false => rfl
  | true =>
    have good := (retain_admission_iff_all_segments segments).mp accepted segment member
    rw [bad] at good
    cases good

theorem fee_after_other_refused (suffix : List StatementClass) :
    segmentFeesFirst (.other :: .fee :: suffix) = false := by
  rfl

theorem let_and_requires_before_fee_admitted :
    segmentFeesFirst [.letBinding, .precondition, .fee] = true := by
  rfl

inductive Outcome where
  | running
  | waiting (site : Nat)
  | halted
  | rejected (code message : String)
  | exhausted (reason : String)
  deriving DecidableEq

/-- The two outcomes accepted by the local-failure boundary. -/
inductive Failure where
  | rejected (code message : String)
  | exhausted (reason : String)

def Failure.outcome : Failure → Outcome
  | .rejected code message => .rejected code message
  | .exhausted reason => .exhausted reason

/-- A projection of FinImage2 with opaque complete snapshot/capture/effect payloads.
Naturals require `WorkAccounting` to represent UInt128 work counters. -/
structure Image (Snapshot Capture Effect : Type) where
  programHash : String
  planHash : String
  action : String
  policy : PhasePolicy
  outcome : Outcome
  segment : Nat
  captures : List Capture
  committed : Snapshot
  committedEffects : List Effect
  committedLogical : Nat
  initialFuel : Nat
  remainingWork : Nat
  consumedWork : Nat
  discardedEffects : List Effect
  retainedByPolicy : List Effect

variable {Snapshot Capture Effect : Type}

def uint128Max : Nat := 2 ^ 128 - 1

structure WorkAccounting (image : Image Snapshot Capture Effect) : Prop where
  balance : image.remainingWork + image.consumedWork = image.initialFuel
  bounded : image.initialFuel ≤ uint128Max

/-- Successful machine charge. The caller supplies the positive-fuel dispatch guard. -/
def charge (image : Image Snapshot Capture Effect) : Image Snapshot Capture Effect :=
  { image with
    remainingWork := image.remainingWork - 1
    consumedWork := image.consumedWork + 1 }

theorem charge_preserves_work (image : Image Snapshot Capture Effect)
    (accounting : WorkAccounting image) (positive : 0 < image.remainingWork) :
    WorkAccounting (charge image) := by
  refine ⟨?_, accounting.bounded⟩
  change image.remainingWork - 1 + (image.consumedWork + 1) = image.initialFuel
  have balance := accounting.balance
  omega

theorem charge_preserves_financial_prefix (image : Image Snapshot Capture Effect) :
    (charge image).committed = image.committed ∧
    (charge image).committedEffects = image.committedEffects ∧
    (charge image).committedLogical = image.committedLogical := by
  exact ⟨rfl, rfl, rfl⟩

theorem charge_stays_uint128 (image : Image Snapshot Capture Effect)
    (accounting : WorkAccounting image) (positive : 0 < image.remainingWork) :
    (charge image).remainingWork ≤ uint128Max ∧
    (charge image).consumedWork ≤ uint128Max := by
  have balance := accounting.balance
  have bounded := accounting.bounded
  change image.remainingWork - 1 ≤ uint128Max ∧ image.consumedWork + 1 ≤ uint128Max
  omega

/-- Successful XFinAwaitEnter after dispatch and logical-plan validation.
Capture extraction success is supplied, not proved from an arbitrary environment. -/
def awaitCommit (image : Image Snapshot Capture Effect) (tentative : Snapshot)
    (_resumeFuel : 0 < image.remainingWork) (feeEffects fallible : List Effect)
    (logical site : Nat) (captures : List Capture) :
    Image Snapshot Capture Effect :=
  { image with
    committed := tentative
    committedEffects := image.committedEffects ++ feeEffects ++ fallible
    committedLogical := logical
    captures := captures
    segment := image.segment + 1
    outcome := .waiting site }

theorem await_publishes_committed_prefix (image : Image Snapshot Capture Effect)
    (tentative : Snapshot) (fees fallible : List Effect) (logical site : Nat)
    (captures : List Capture) (resumeFuel : 0 < image.remainingWork) :
    let next := awaitCommit image tentative resumeFuel fees fallible logical site captures
    next.committed = tentative ∧
    next.committedEffects = image.committedEffects ++ fees ++ fallible ∧
    next.committedLogical = logical ∧ next.outcome = .waiting site ∧
    next.captures = captures ∧ next.consumedWork = image.consumedWork := by
  exact ⟨rfl, rfl, rfl, rfl, rfl, rfl⟩

/-- Successful XFinComplete after the same charged dispatch and plan checks. -/
def completeCommit (image : Image Snapshot Capture Effect) (tentative : Snapshot)
    (feeEffects fallible : List Effect) (logical : Nat) : Image Snapshot Capture Effect :=
  { image with
    committed := tentative
    committedEffects := image.committedEffects ++ feeEffects ++ fallible
    committedLogical := logical
    captures := []
    outcome := .halted }

theorem complete_publishes_ordered_effects (image : Image Snapshot Capture Effect)
    (tentative : Snapshot) (fees fallible : List Effect) (logical : Nat) :
    let next := completeCommit image tentative fees fallible logical
    next.committed = tentative ∧
    next.committedEffects = image.committedEffects ++ fees ++ fallible ∧
    next.outcome = .halted ∧ next.captures = [] ∧ next.segment = image.segment := by
  exact ⟨rfl, rfl, rfl, rfl, rfl⟩

/-- A sealed fee phase includes its snapshot and its exact ordered effect log. -/
structure FeeSeal (Snapshot Effect : Type) where
  snapshot : Snapshot
  effects : List Effect

/-- `fail`: machine charges have already happened in the supplied image.
A failed fee phase has no seal; even the retain policy then discards all effects. -/
def localFailure (image : Image Snapshot Capture Effect) (failure : Failure)
    (feeSeal : Option (FeeSeal Snapshot Effect)) (feeEffects fallible : List Effect) :
    Image Snapshot Capture Effect :=
  match image.policy, feeSeal with
  | .retainGuaranteedFees, some sealed =>
    { image with
      committed := sealed.snapshot
      committedEffects := image.committedEffects ++ sealed.effects
      retainedByPolicy := sealed.effects
      discardedEffects := fallible
      captures := []
      outcome := failure.outcome }
  | _, _ =>
    { image with
      discardedEffects := feeEffects ++ fallible
      captures := []
      outcome := failure.outcome }

theorem failed_unsealed_phase_preserves_prefix (image : Image Snapshot Capture Effect)
    (failure : Failure) (fees fallible : List Effect) :
    let next := localFailure image failure none fees fallible
    next.committed = image.committed ∧ next.committedEffects = image.committedEffects ∧
    next.discardedEffects = fees ++ fallible := by
  cases policy : image.policy <;> simp [localFailure, policy]

theorem all_or_nothing_preserves_prefix (image : Image Snapshot Capture Effect)
    (policy : image.policy = .allOrNothing) (failure : Failure)
    (feeSeal : Option (FeeSeal Snapshot Effect)) (fees fallible : List Effect) :
    let next := localFailure image failure feeSeal fees fallible
    next.committed = image.committed ∧ next.committedEffects = image.committedEffects ∧
    next.discardedEffects = fees ++ fallible := by
  simp [localFailure, policy]

theorem sealed_fees_retained_separately (image : Image Snapshot Capture Effect)
    (policy : image.policy = .retainGuaranteedFees) (failure : Failure)
    (sealed : FeeSeal Snapshot Effect) (fees fallible : List Effect) :
    let next := localFailure image failure (some sealed) fees fallible
    next.committed = sealed.snapshot ∧
    next.committedEffects = image.committedEffects ++ sealed.effects ∧
    next.retainedByPolicy = sealed.effects ∧ next.discardedEffects = fallible := by
  simp [localFailure, policy]

theorem failure_keeps_charged_work (image : Image Snapshot Capture Effect)
    (failure : Failure) (feeSeal : Option (FeeSeal Snapshot Effect))
    (fees fallible : List Effect) :
    let next := localFailure image failure feeSeal fees fallible
    next.consumedWork = image.consumedWork ∧ next.remainingWork = image.remainingWork ∧
    next.committedLogical = image.committedLogical ∧ next.captures = [] ∧
    next.outcome = failure.outcome := by
  cases policy : image.policy <;> cases feeSeal <;> simp [localFailure, policy]

/-- Post-dispatch await fuel guard; other plan/capture checks are supplied separately. -/
def awaitEnter (image : Image Snapshot Capture Effect) (tentative : Snapshot)
    (feeSeal : Option (FeeSeal Snapshot Effect)) (fees fallible : List Effect)
    (logical site : Nat) (captures : List Capture) : Image Snapshot Capture Effect :=
  if resumeFuel : 0 < image.remainingWork then
    awaitCommit image tentative resumeFuel fees fallible logical site captures
  else
    localFailure image (.exhausted "Fuel") feeSeal fees fallible

theorem await_enter_zero_fuel_exhausts (image : Image Snapshot Capture Effect)
    (tentative : Snapshot) (feeSeal : Option (FeeSeal Snapshot Effect))
    (fees fallible : List Effect) (logical site : Nat) (captures : List Capture)
    (zero : image.remainingWork = 0) :
    awaitEnter image tentative feeSeal fees fallible logical site captures =
      localFailure image (.exhausted "Fuel") feeSeal fees fallible := by
  simp [awaitEnter, zero]

theorem await_enter_waiting_has_fuel (image : Image Snapshot Capture Effect)
    (tentative : Snapshot) (feeSeal : Option (FeeSeal Snapshot Effect))
    (fees fallible : List Effect) (logical site : Nat) (captures : List Capture)
    (waiting : (awaitEnter image tentative feeSeal fees fallible logical site captures).outcome =
      .waiting site) : 0 < image.remainingWork := by
  by_cases resumeFuel : 0 < image.remainingWork
  · exact resumeFuel
  · have failed := failure_keeps_charged_work image (.exhausted "Fuel") feeSeal fees fallible
    rw [awaitEnter, dif_neg resumeFuel, failed.2.2.2.2] at waiting
    cases waiting

/-- The new financial R0 fuel check alone; not a complete image validator. -/
def resumeFuelAllowed (image : Image Snapshot Capture Effect) : Bool :=
  decide (0 < image.remainingWork)

theorem resume_zero_fuel_refused (image : Image Snapshot Capture Effect)
    (zero : image.remainingWork = 0) : resumeFuelAllowed image = false := by
  simp [resumeFuelAllowed, zero]

/-- False/error guards may report attempted logical work but return the original image. -/
def notReady (image : Image Snapshot Capture Effect) (_attempted : Nat) :
    Image Snapshot Capture Effect := image

theorem not_ready_preserves_image (image : Image Snapshot Capture Effect) (attempted : Nat) :
    notReady image attempted = image := by
  rfl

/-- Positive liabilities in snapshot order, retaining each complete duty payload. -/
def residualDuties {Duty : Type} (amount : Duty → Nat) (duties : List Duty) : List Duty :=
  duties.filter (fun duty ↦ decide (0 < amount duty))

/-- The actual snapshot-to-liabilities and amount projections remain caller obligations. -/
def committedDuties {Duty : Type} (dutiesOf : Snapshot → List Duty) (amount : Duty → Nat)
    (image : Image Snapshot Capture Effect) : List Duty :=
  residualDuties amount (dutiesOf image.committed)

theorem residual_duty_mem_iff {Duty : Type} (amount : Duty → Nat) (duties : List Duty)
    (duty : Duty) :
    duty ∈ residualDuties amount duties ↔ duty ∈ duties ∧ 0 < amount duty := by
  simp [residualDuties]

theorem residual_duties_preserve_order {Duty : Type} (amount : Duty → Nat)
    (duties : List Duty) : (residualDuties amount duties).Sublist duties := by
  exact List.filter_sublist

theorem await_commits_residual_duties {Duty : Type} (dutiesOf : Snapshot → List Duty)
    (amount : Duty → Nat) (image : Image Snapshot Capture Effect) (tentative : Snapshot)
    (resumeFuel : 0 < image.remainingWork) (fees fallible : List Effect)
    (logical site : Nat) (captures : List Capture) :
    committedDuties dutiesOf amount
      (awaitCommit image tentative resumeFuel fees fallible logical site captures) =
      residualDuties amount (dutiesOf tentative) := by
  rfl

theorem failed_unsealed_retains_residual_duties {Duty : Type}
    (dutiesOf : Snapshot → List Duty) (amount : Duty → Nat)
    (image : Image Snapshot Capture Effect) (failure : Failure) (fees fallible : List Effect) :
    committedDuties dutiesOf amount (localFailure image failure none fees fallible) =
      committedDuties dutiesOf amount image := by
  have dutyFacts := failed_unsealed_phase_preserves_prefix image failure fees fallible
  unfold committedDuties
  rw [dutyFacts.1]

theorem sealed_failure_projects_residual_duties {Duty : Type}
    (dutiesOf : Snapshot → List Duty) (amount : Duty → Nat)
    (image : Image Snapshot Capture Effect) (policy : image.policy = .retainGuaranteedFees)
    (failure : Failure) (sealed : FeeSeal Snapshot Effect) (fees fallible : List Effect) :
    committedDuties dutiesOf amount
      (localFailure image failure (some sealed) fees fallible) =
      residualDuties amount (dutiesOf sealed.snapshot) := by
  have dutyFacts := sealed_fees_retained_separately image policy failure sealed fees fallible
  unfold committedDuties
  rw [dutyFacts.1]

/-- Literal residual amount in UIREQ-017, with extinguished duties omitted. -/
theorem first_paid_residual_amount : residualDuties id [(0 : Nat), 10, 0] = [10] := by
  decide

/-- Only accounting-relevant payload is projected here; full effects remain in Image. -/
inductive AccountingEffect where
  | transfer (amount : Fin (2 ^ 128))
  | fee (amount : Fin (2 ^ 128))
  | other

def checkedAdd (left right : Nat) : Option Nat :=
  if left + right ≤ uint128Max then some (left + right) else none

theorem checkedAdd_success {left right result : Nat}
    (success : checkedAdd left right = some result) :
    left + right = result ∧ result ≤ uint128Max := by
  by_cases bounded : left + right ≤ uint128Max
  · rw [checkedAdd, if_pos bounded] at success
    cases Option.some.inj success
    exact ⟨rfl, bounded⟩
  · rw [checkedAdd, if_neg bounded] at success
    cases success

def accountEffect (totals : Nat × Nat) : AccountingEffect → Option (Nat × Nat)
  | .transfer amount =>
    match checkedAdd totals.1 amount.val with
    | some payments => some (payments, totals.2)
    | none => none
  | .fee amount =>
    match checkedAdd totals.2 amount.val with
    | some fees => some (totals.1, fees)
    | none => none
  | .other => some totals

def accumulate : (Nat × Nat) → List AccountingEffect → Option (Nat × Nat)
  | totals, [] => some totals
  | totals, effect :: rest =>
    match accountEffect totals effect with
    | some next => accumulate next rest
    | none => none

/-- Left-to-right checked sums; the first overflowing accumulator refuses the report. -/
def totals (effects : List AccountingEffect) : Option (Nat × Nat) :=
  accumulate (0, 0) effects

structure AccountingReport where
  grossDebit : Nat
  fees : Nat
  recipientReceipt : Nat
  deriving DecidableEq

/-- Current host accounting returns no report on accumulator or gross overflow. -/
def accounting (effects : List AccountingEffect) : Option AccountingReport :=
  match totals effects with
  | none => none
  | some sums =>
    match checkedAdd sums.1 sums.2 with
    | none => none
    | some gross => some ⟨gross, sums.2, sums.1⟩

theorem accounting_success (effects : List AccountingEffect) (report : AccountingReport)
    (success : accounting effects = some report) :
    report.grossDebit = report.recipientReceipt + report.fees ∧
    report.grossDebit ≤ uint128Max := by
  cases sums : totals effects with
  | none => simp [accounting, sums] at success
  | some values =>
    cases added : checkedAdd values.1 values.2 with
    | none => simp [accounting, sums, added] at success
    | some gross =>
      have reportEq : (⟨gross, values.2, values.1⟩ : AccountingReport) = report :=
        Option.some.inj (by simpa [accounting, sums, added] using success)
      have exactSum := checkedAdd_success added
      cases reportEq
      exact ⟨exactSum.1.symm, exactSum.2⟩

/-- UIREQ-017's literal accounting example; snapshot evaluation is a separate premise. -/
def firstPaidEffects : List AccountingEffect :=
  [.transfer ⟨30, by decide⟩, .fee ⟨2, by decide⟩, .other]

theorem first_paid_prefix_accounting :
    accounting firstPaidEffects = some ⟨32, 2, 30⟩ ∧
    (100 : Nat) - 32 = 68 ∧ (40 : Nat) - 30 = 10 := by
  decide

/-- The formerly saturated gross example now refuses publication. -/
theorem accounting_gross_overflow_refused :
    accounting [.transfer ⟨uint128Max, by decide⟩, .fee ⟨1, by decide⟩] = none := by
  decide

theorem accounting_payments_overflow_refused :
    accounting [.transfer ⟨uint128Max, by decide⟩, .transfer ⟨1, by decide⟩] = none := by
  decide

theorem accounting_fees_overflow_refused :
    accounting [.fee ⟨uint128Max, by decide⟩, .fee ⟨1, by decide⟩] = none := by
  decide

end Clean.Sherlock.FinancialItems
