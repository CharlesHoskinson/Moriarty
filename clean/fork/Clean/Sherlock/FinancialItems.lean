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
    (feeEffects fallible : List Effect) (logical site : Nat) (captures : List Capture) :
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
    (captures : List Capture) :
    let next := awaitCommit image tentative fees fallible logical site captures
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

/-- False/error guards may report attempted logical work but return the original image. -/
def notReady (image : Image Snapshot Capture Effect) (_attempted : Nat) :
    Image Snapshot Capture Effect := image

theorem not_ready_preserves_image (image : Image Snapshot Capture Effect) (attempted : Nat) :
    notReady image attempted = image := by
  rfl

/-- Only accounting-relevant payload is projected here; full effects remain in Image. -/
inductive AccountingEffect where
  | transfer (amount : Fin (2 ^ 128))
  | fee (amount : Fin (2 ^ 128))
  | other

def satAdd (left right : Nat) : Nat := min (left + right) uint128Max

def accountEffect (totals : Nat × Nat) : AccountingEffect → Nat × Nat
  | .transfer amount => (satAdd totals.1 amount.val, totals.2)
  | .fee amount => (totals.1, satAdd totals.2 amount.val)
  | .other => totals

/-- Exactly the two independent, left-to-right accumulators in sherlock2::accounting. -/
def totals (effects : List AccountingEffect) : Nat × Nat :=
  effects.foldl accountEffect (0, 0)

structure AccountingReport where
  grossDebit : Nat
  fees : Nat
  recipientReceipt : Nat

/-- Saturated report, not an unbounded conservation identity. -/
def accounting (effects : List AccountingEffect) : AccountingReport :=
  let (payments, fees) := totals effects
  ⟨satAdd payments fees, fees, payments⟩

theorem accounting_gross_is_saturated (effects : List AccountingEffect) :
    (accounting effects).grossDebit =
      min ((accounting effects).recipientReceipt + (accounting effects).fees) uint128Max := by
  rfl

theorem accounting_conservation_without_saturation (effects : List AccountingEffect)
    (bounded : (accounting effects).recipientReceipt + (accounting effects).fees ≤ uint128Max) :
    (accounting effects).grossDebit =
      (accounting effects).recipientReceipt + (accounting effects).fees := by
  exact Nat.min_eq_left bounded

/-- UIREQ-017's literal accounting example; snapshot evaluation is a separate premise. -/
def firstPaidEffects : List AccountingEffect :=
  [.transfer ⟨30, by decide⟩, .fee ⟨2, by decide⟩, .other]

theorem first_paid_prefix_accounting :
    (accounting firstPaidEffects).grossDebit = 32 ∧
    (accounting firstPaidEffects).fees = 2 ∧
    (accounting firstPaidEffects).recipientReceipt = 30 ∧
    100 - (accounting firstPaidEffects).grossDebit = 68 ∧ (40 : Nat) - 30 = 10 := by
  decide

/-- A reporting overflow saturates; an unqualified gross = receipt + fees is false. -/
theorem saturated_gross_example :
    (accounting [.transfer ⟨uint128Max, by decide⟩, .fee ⟨1, by decide⟩]).grossDebit =
      uint128Max ∧
    (accounting [.transfer ⟨uint128Max, by decide⟩, .fee ⟨1, by decide⟩]).recipientReceipt +
      (accounting [.transfer ⟨uint128Max, by decide⟩, .fee ⟨1, by decide⟩]).fees =
        uint128Max + 1 := by
  decide

end Clean.Sherlock.FinancialItems
