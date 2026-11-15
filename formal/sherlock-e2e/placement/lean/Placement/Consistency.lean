/-!
# SPEC-CHECKED rows of the Placement context

These requirements state design or process rules with no state machine of their own.
Each is a `Bool` predicate over one `Design` record; `consistent` shows that one design
meets all of them together with the modelled placement rules (no contradiction).

* HLP-006: a design that proves on a host the user does not own is a new OpenSpec change
  on the Veritee seald Sessions model, and edits no requirement of this change. Reason:
  a rule on how changes are made (process).
* ZK-009: the list of designs that are not allowed. The slot clauses (no slot reuse, no
  helper-made slot) are also modelled by `PRD_002_one_slot_per_wrap`; the other clauses are
  design facts. Reason: a design-space restriction.
-/

namespace Placement

structure Design where
  proposesHelperProving : Bool    -- a design proposes proving on a host the user does not own
  isNewChange : Bool              -- ... and is written as a new OpenSpec change
  onSealdSessions : Bool          -- ... on the Veritee seald Sessions model
  editsThisChange : Bool          -- ... and edits a requirement of this change
  sdkHelperCommands : Nat         -- HLP-005: commands or options that send private parts away
  helperProvesStark : Bool
  remoteUnmaskedOps : Bool
  sponseStateOnHelper : Bool
  slotReuse : Bool
  helperSlot : Bool
  aggregatorGetsStark : Bool
  wrapsPerContractCall : Nat
  userBatchOptIn : Bool
  batchDigestsPerAcceptRun : Nat
  slotPool : Nat                  -- PRD-002: the daemon holds a pool of one-use slots
  deriving Repr, DecidableEq

def HLP_006_ok (d : Design) : Bool :=
  !d.proposesHelperProving || (d.isNewChange && d.onSealdSessions && !d.editsThisChange)
def HLP_005_ok (d : Design) : Bool := d.sdkHelperCommands == 0
def ZK_009_ok (d : Design) : Bool :=
  !d.helperProvesStark && !d.remoteUnmaskedOps && !d.sponseStateOnHelper && !d.slotReuse
    && !d.helperSlot && !d.aggregatorGetsStark && d.wrapsPerContractCall == 1 && d.userBatchOptIn
    && d.batchDigestsPerAcceptRun == 1
def PRD_002_ok (d : Design) : Bool := 1 ≤ d.slotPool && !d.helperSlot && !d.slotReuse

def allOk (d : Design) : Bool := HLP_006_ok d && HLP_005_ok d && ZK_009_ok d && PRD_002_ok d

/-- The prototype design, with a future case-B proposal written as its own change. -/
def witness : Design :=
  { proposesHelperProving := true, isNewChange := true, onSealdSessions := true,
    editsThisChange := false, sdkHelperCommands := 0, helperProvesStark := false,
    remoteUnmaskedOps := false, sponseStateOnHelper := false, slotReuse := false,
    helperSlot := false, aggregatorGetsStark := false, wrapsPerContractCall := 1,
    userBatchOptIn := true, batchDigestsPerAcceptRun := 1, slotPool := 4 }

theorem consistent : allOk witness = true := by decide

/-- HLP-006 refuses a case-B proposal that edits this change. -/
theorem HLP_006_refuses_edit :
    HLP_006_ok { witness with editsThisChange := true } = false := by decide

end Placement
