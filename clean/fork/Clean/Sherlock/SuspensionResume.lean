import Init

/-!
UIREQ-014/015/016: exact checkpoint and suspension contracts.
`Whole` below stands for the entire snapshot (including admission, frames,
heap, environments, counters and applicable financial state), not a final value.
The definitions expose obligations for Rust decoding and stepping; they are not
an extracted interpreter, cryptographic codec, C6 relation or ledger proof.
The /2 observation interface tracks the in-flight owner_suspend/suspension2 work.
-/

namespace Clean.Sherlock.SuspensionResume

inductive Profile where
  | microstep1 | microstep2
  deriving DecidableEq, Repr

def Profile.id : Profile → Nat
  | .microstep1 => 2
  | .microstep2 => 3

theorem profile_ids_distinct : Profile.microstep1.id ≠ Profile.microstep2.id := by decide

inductive Outcome where
  | running | waiting (site : Nat) | halted | rejected | exhausted
  deriving DecidableEq, Repr

/-- Ordinary stepping on Waiting has no successor, even at zero remaining fuel.
Observation acceptance uses its own atomic relation. -/
def ordinaryStep {Whole : Type} (advance : Whole → Whole) (outcome : Outcome)
    (state : Whole) : Option Whole :=
  match outcome with
  | .running => some (advance state)
  | _ => none

theorem waiting_has_no_ordinary_successor {Whole : Type} (advance : Whole → Whole)
    (site : Nat) (state : Whole) : ordinaryStep advance (.waiting site) state = none := rfl

structure Checkpoint (Whole : Type) where
  profile : Profile
  financial : Bool
  genesisSegment : Bool
  whole : Whole
  deriving DecidableEq

/-- Abstracts successful reconstruction and re-capture from owner_resume.rs.
The concrete decoder and full SI predicate still have to be supplied. -/
def restoreCheckpoint {Whole Image : Type} [DecidableEq Whole]
    (selected : Profile) (capture : Image → Checkpoint Whole) (valid : Image → Bool)
    (checkpoint : Checkpoint Whole) (candidate : Image) : Option Image :=
  if checkpoint.financial then none
  else if !checkpoint.genesisSegment then none
  else if checkpoint.profile ≠ selected then none
  else if !valid candidate then none
  else if capture candidate = checkpoint then some candidate else none

theorem financial_technical_restore_refused {Whole Image : Type} [DecidableEq Whole]
    (selected : Profile) (capture : Image → Checkpoint Whole) (valid : Image → Bool)
    (checkpoint : Checkpoint Whole) (candidate : Image)
    (financial : checkpoint.financial = true) :
    restoreCheckpoint selected capture valid checkpoint candidate = none := by
  simp [restoreCheckpoint, financial]

theorem successful_restore_is_exact {Whole Image : Type} [DecidableEq Whole]
    (selected : Profile) (capture : Image → Checkpoint Whole) (valid : Image → Bool)
    (checkpoint : Checkpoint Whole) (candidate restored : Image)
    (success : restoreCheckpoint selected capture valid checkpoint candidate = some restored) :
    capture restored = checkpoint := by
  unfold restoreCheckpoint at success
  split at success
  · contradiction
  · split at success
    · contradiction
    · split at success
      · contradiction
      · split at success
        · contradiction
        · split at success
          · rename_i exactCapture
            cases success
            exact exactCapture
          · contradiction

/-- Iteration is over a supplied complete deterministic transition. Terminal
publication priority and absorbing outcomes are obligations of that transition. -/
def runSteps {Whole : Type} (step : Whole → Whole) : Nat → Whole → Whole
  | 0, state => state
  | n + 1, state => runSteps step n (step state)

theorem runSteps_add {Whole : Type} (step : Whole → Whole) (first second : Nat)
    (state : Whole) :
    runSteps step (first + second) state = runSteps step second (runSteps step first state) := by
  induction first generalizing state with
  | zero => simp only [Nat.zero_add, runSteps]
  | succ first ih => simpa only [Nat.succ_add, runSteps] using ih (step state)

/-- A technical cut is between events and creates no event, charge or financial commit. -/
def technicalCut {Whole : Type} (state : Whole) : Whole := state

theorem chunk_preserves_complete_observation {Whole Observation : Type}
    (step : Whole → Whole) (observe : Whole → Observation) (first second : Nat)
    (state : Whole) :
    observe (runSteps step second (technicalCut (runSteps step first state))) =
      observe (runSteps step (first + second) state) :=
  congrArg observe (runSteps_add step first second state).symm

inductive GuardResult where
  | accept | notReady | error
  deriving DecidableEq, Repr

/-- Ordered interface checks are kept outside guard execution. These booleans
stand for R0..R5 (carrier/invariants, Waiting, admission/profile, checkpoint,
site/label and typed binder layout), not proofs that a concrete codec checks them. -/
structure ObservationChecks where
  carrier : Bool
  waiting : Bool
  admission : Bool
  checkpoint : Bool
  siteLabel : Bool
  binderLayout : Bool

def ObservationChecks.valid (checks : ObservationChecks) : Bool :=
  checks.carrier && checks.waiting && checks.admission && checks.checkpoint &&
    checks.siteLabel && checks.binderLayout

structure Suspended (Whole Frame : Type) where
  whole : Whole
  callerFrames : List Frame
  remainingWork : Nat
  consumedWork : Nat
  referenceCursor : Nat
  resumeEpoch : Nat

inductive ResumeResult (Whole Frame : Type) where
  | refused
  | notReady (checkpoint : Suspended Whole Frame) (attemptedWork : Nat)
  | accepted (successor : Suspended Whole Frame)

/-- A mathematical pure-route accept update. `install` must implement fresh binder
allocation and the Eval(body) successor while retaining the rest of the whole state.
Natural counters require concrete UInt128 accounting/headroom premises in Rust. -/
def acceptResume {Whole Frame : Type} (install : Whole → Whole) (guardNodes : Nat)
    (state : Suspended Whole Frame) : Suspended Whole Frame :=
  { state with
    whole := install state.whole
    remainingWork := state.remainingWork - (1 + guardNodes)
    consumedWork := state.consumedWork + (1 + guardNodes)
    referenceCursor := state.referenceCursor + (1 + guardNodes)
    resumeEpoch := state.resumeEpoch + 1 }

/-- No successor is constructed for malformed requests or insufficient resources.
False and error guards return the original complete checkpoint, with attempted work
reported separately. The install function runs only for an accepted observation. -/
def resume {Whole Frame : Type} (checks : ObservationChecks) (resources : Bool)
    (guard : GuardResult) (guardNodes : Nat) (install : Whole → Whole)
    (state : Suspended Whole Frame) : ResumeResult Whole Frame :=
  if !checks.valid then .refused
  else match guard with
    | .notReady | .error => .notReady state (1 + guardNodes)
    | .accept =>
        if resources then .accepted (acceptResume install guardNodes state) else .refused

theorem false_guard_preserves_checkpoint {Whole Frame : Type}
    (checks : ObservationChecks) (resources : Bool) (guardNodes : Nat)
    (install : Whole → Whole) (state : Suspended Whole Frame) (valid : checks.valid = true) :
    resume checks resources .notReady guardNodes install state =
      .notReady state (1 + guardNodes) := by
  simp [resume, valid]

theorem error_guard_preserves_checkpoint {Whole Frame : Type}
    (checks : ObservationChecks) (resources : Bool) (guardNodes : Nat)
    (install : Whole → Whole) (state : Suspended Whole Frame) (valid : checks.valid = true) :
    resume checks resources .error guardNodes install state = .notReady state (1 + guardNodes) := by
  simp [resume, valid]

theorem accepted_resume_retains_callers {Whole Frame : Type} (install : Whole → Whole)
    (guardNodes : Nat) (state : Suspended Whole Frame) :
    (acceptResume install guardNodes state).callerFrames = state.callerFrames := rfl

theorem accepted_resume_exact_charge {Whole Frame : Type} (install : Whole → Whole)
    (guardNodes : Nat) (state : Suspended Whole Frame) :
    (acceptResume install guardNodes state).consumedWork = state.consumedWork + (1 + guardNodes) ∧
      (acceptResume install guardNodes state).referenceCursor =
        state.referenceCursor + (1 + guardNodes) := ⟨rfl, rfl⟩

theorem accepted_resume_fuel_balance {Whole Frame : Type} (install : Whole → Whole)
    (guardNodes : Nat) (state : Suspended Whole Frame)
    (enough : 1 + guardNodes ≤ state.remainingWork) :
    (acceptResume install guardNodes state).remainingWork +
      (acceptResume install guardNodes state).consumedWork =
        state.remainingWork + state.consumedWork := by
  change state.remainingWork - (1 + guardNodes) +
    (state.consumedWork + (1 + guardNodes)) = _
  rw [Nat.add_comm state.consumedWork, ← Nat.add_assoc, Nat.sub_add_cancel enough]

end Clean.Sherlock.SuspensionResume
