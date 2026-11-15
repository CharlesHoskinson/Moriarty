import Placement.Spec

/-!
# Known answers, written from the requirement sentences

Each answer is pinned twice: `decide +kernel` (kernel) and `#guard` (compiled evaluator).
-/

namespace Placement

def o (code arg : Int) : Int := code * opBase + arg

/-- One honest job on an owned box (case A) with call vector 7. -/
def honest : List Int := [o 1 71, o 2 71, o 3 1, o 4 0]

-- HLP-005, ZK-001, PRD-003: a box the user does not own gets no job
theorem pin_not_owned : (run [o 1 70]).lastRefusal = rNotOwned ∧ (run [o 1 70]).phase = 0 := by decide +kernel
#guard (run [o 1 70]).lastRefusal == rNotOwned && (run [o 1 70]).phase == 0

-- HLP-001, HLP-003: the honest job is checked, kept and submitted once
theorem pin_honest : (run honest).phase = 5 ∧ (run honest).submitted = 1 ∧ (run honest).kept = 1
    ∧ (run honest).resWords = 4 ∧ (run honest).resPrivate = 0 := by decide +kernel
#guard (run honest).phase == 5 && (run honest).submitted == 1

-- HLP-008: another call vector is dropped before the proof is verified
theorem pin_cv_differs : (run [o 1 71, o 2 81, o 3 1]).phase = 4 ∧ (run [o 1 71, o 2 81, o 3 1]).dropCode = dBinding
    ∧ (run [o 1 71, o 2 81, o 3 1]).verifyRan = false := by decide +kernel
#guard (run [o 1 71, o 2 81, o 3 1]).dropCode == dBinding && !(run [o 1 71, o 2 81, o 3 1]).verifyRan
-- HLP-008: a sha that differs from the box report is dropped before the proof is verified
theorem pin_sha_differs : (run [o 1 71, o 2 70, o 3 1]).dropCode = dBinding ∧
    (run [o 1 71, o 2 70, o 3 1]).verifyRan = false := by decide +kernel
#guard (run [o 1 71, o 2 70, o 3 1]).dropCode == dBinding

-- HLP-002: a refused box wrap is dropped and no call carries it
theorem pin_refused : (run [o 1 71, o 2 71, o 3 0, o 4 0]).phase = 4 ∧
    (run [o 1 71, o 2 71, o 3 0, o 4 0]).lastRefusal = rDropped ∧
    (run [o 1 71, o 2 71, o 3 0, o 4 0]).submitted = 0 := by decide +kernel
#guard (run [o 1 71, o 2 71, o 3 0, o 4 0]).submitted == 0

-- HLP-007: a box with no wallet directory refuses to sign
theorem pin_no_wallet : (run [o 6 0]).lastRefusal = rNoWallet ∧ (run [o 6 0]).signed = 0 := by decide +kernel
#guard (run [o 6 0]).lastRefusal == rNoWallet
theorem pin_wallet : (run [o 6 1]).signed = 1 := by decide +kernel
#guard (run [o 6 1]).signed == 1

-- PRD-002: a helper-made slot is refused; no slot, no wrap_prove; one slot, one proof
theorem pin_helper_slot : (run [o 7 1]).lastRefusal = rHelperSlot ∧ (run [o 7 1]).slotsFree = 0 := by decide +kernel
#guard (run [o 7 1]).lastRefusal == rHelperSlot
theorem pin_no_slot : (run [o 8 0]).lastRefusal = rNoSlot ∧ (run [o 8 0]).wrapProofs = 0 := by decide +kernel
#guard (run [o 8 0]).lastRefusal == rNoSlot
theorem pin_one_slot : (run [o 7 0, o 8 0, o 8 0]).wrapProofs = 1 ∧
    (run [o 7 0, o 8 0, o 8 0]).lastRefusal = rNoSlot := by decide +kernel
#guard (run [o 7 0, o 8 0, o 8 0]).wrapProofs == 1
-- PRD-002: a slot is not serialized off the machine
theorem pin_export : (run [o 7 0, o 9 0]).lastRefusal = rSlotExport ∧ (run [o 7 0, o 9 0]).slotsFree = 1 := by
  decide +kernel
#guard (run [o 7 0, o 9 0]).lastRefusal == rSlotExport

-- HLP-005, PRD-003: a STARK proof to a host the user does not own is refused
theorem pin_send_helper : (run [o 10 20]).lastRefusal = rNotOwned ∧ (run [o 10 20]).sentNotOwned = 0 := by
  decide +kernel
#guard (run [o 10 20]).lastRefusal == rNotOwned
theorem pin_send_owned : (run [o 10 21]).sentOwned = 1 := by decide +kernel
#guard (run [o 10 21]).sentOwned == 1

-- step domain
theorem pin_refuse_code : stepRefusals init (o 11 0) = [.notAnOperation] := by decide +kernel
theorem pin_refuse_disabled : stepRefusals init (o 3 1) = [.opNotEnabled] := by decide +kernel
theorem pin_refuse_kind : stepRefusals init (o 10 51) = [.notAnOperation] := by decide +kernel
#guard stepRefusals init (o 3 1) == [.opNotEnabled]

end Placement
