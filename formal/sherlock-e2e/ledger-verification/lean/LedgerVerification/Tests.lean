import LedgerVerification.Spec

/-!
# Known answers, written from the requirement sentences

Each answer is pinned twice: `decide +kernel` (kernel) and `#guard` (compiled evaluator).
-/

namespace LedgerVerification

def o (code arg : Int) : Int := code * opBase + arg

/-- The four crates at rev a2b30128, then the gate (LVR-002). -/
def pinned : List Int := [o 1 1, o 1 11, o 1 21, o 1 31, o 2 0]
/-- An honest check: honest accepted, 7 lanes and 3 bytes refused (LVR-004). -/
def honestCheck : Int := o 4 371

-- LVR-002: all four at the revision pass the gate; a path override of one crate fails it
theorem pin_gate_pass : (run pinned).gateOk = true := by decide +kernel
#guard (run pinned).gateOk
theorem pin_gate_override : (run [o 1 1, o 1 11, o 1 20, o 1 31, o 2 0]).gateOk = false := by decide +kernel
#guard !(run [o 1 1, o 1 11, o 1 20, o 1 31, o 2 0]).gateOk
-- LVR-002: a manifest change after the gate invalidates it
theorem pin_gate_stale : (run (pinned ++ [o 1 1])).gateOk = false := by decide +kernel
#guard !(run (pinned ++ [o 1 1])).gateOk

-- LVR-003, MID-001: a G4 key is refused by the reader
theorem pin_g4 : (run [o 3 1]).keyRead = false ∧ (run [o 3 1]).lastRefusal = refG4 := by decide +kernel
#guard !(run [o 3 1]).keyRead && (run [o 3 1]).lastRefusal == refG4
-- no check without a read key
theorem pin_no_key_no_check : stepRefusals (run [o 3 1]) honestCheck = [.opNotEnabled] := by decide +kernel
#guard stepRefusals (run [o 3 1]) honestCheck == [.opNotEnabled]

-- LVR-004, LVR-005: the honest check exits 0 and runs 10 controls
theorem pin_honest : (run (pinned ++ [o 3 0, honestCheck])).exitNonzero = false ∧
    (run (pinned ++ [o 3 0, honestCheck])).controls = 10 := by decide +kernel
#guard !(run (pinned ++ [o 3 0, honestCheck])).exitNonzero
-- LVR-005: one changed lane accepted (6 of 7 refused) exits nonzero
theorem pin_lane_passed : (run (pinned ++ [o 3 0, o 4 361])).exitNonzero = true := by decide +kernel
#guard (run (pinned ++ [o 3 0, o 4 361])).exitNonzero
-- LVR-005: one flipped byte accepted (2 of 3 refused) exits nonzero
theorem pin_flip_passed : (run (pinned ++ [o 3 0, o 4 271])).exitNonzero = true := by decide +kernel
#guard (run (pinned ++ [o 3 0, o 4 271])).exitNonzero
-- LVR-005: the honest proof refused exits nonzero
theorem pin_honest_refused : (run (pinned ++ [o 3 0, o 4 370])).exitNonzero = true := by decide +kernel
#guard (run (pinned ++ [o 3 0, o 4 370])).exitNonzero
-- LVR-001 (LEAD PICK): a check without the passed manifest gate exits nonzero
theorem pin_unpinned : (run [o 3 0, honestCheck]).exitNonzero = true := by decide +kernel
#guard (run [o 3 0, honestCheck]).exitNonzero

-- LVR-006: no submit before the native check
theorem pin_submit_before_native : stepRefusals (run [o 5 0]) (o 7 0) = [.opNotEnabled] := by decide +kernel
#guard stepRefusals (run [o 5 0]) (o 7 0) == [.opNotEnabled]
-- LVR-006: native accepted, then submit
theorem pin_submit : (run [o 5 0, o 6 1, o 7 0]).submits = 1 := by decide +kernel
#guard (run [o 5 0, o 6 1, o 7 0]).submits == 1
-- LVR-007: native refusal: nothing submitted, the call marked refused, the next run proves again
theorem pin_native_refused : (run [o 5 0, o 6 0]).callPhase = 4 ∧ (run [o 5 0, o 6 0]).refusedCalls = 1 ∧
    stepRefusals (run [o 5 0, o 6 0]) (o 7 0) = [.opNotEnabled] := by decide +kernel
#guard (run [o 5 0, o 6 0]).refusedCalls == 1
theorem pin_reprove : (run [o 5 0, o 6 0, o 5 0, o 6 1, o 7 0]).submits = 1 := by decide +kernel
#guard (run [o 5 0, o 6 0, o 5 0, o 6 1, o 7 0]).submits == 1

-- LVR-008: first connect records genesis 7 and version 5; another genesis stops the runner
theorem pin_first_connect : (run [o 8 7005]).genesis = 7 ∧ (run [o 8 7005]).nodeVersion = 5 := by decide +kernel
#guard (run [o 8 7005]).genesis == 7
theorem pin_other_chain : (run [o 8 7005, o 8 9005]).chainStop = true ∧
    (run [o 8 7005, o 8 9005]).lastRefusal = refChain ∧ (run [o 8 7005, o 8 9005]).genesis = 7 := by decide +kernel
#guard (run [o 8 7005, o 8 9005]).chainStop
-- LVR-008: a reconnect to the same chain keeps the first recorded version
theorem pin_same_chain : (run [o 8 7005, o 8 7006]).nodeVersion = 5 ∧ (run [o 8 7005, o 8 7006]).connects = 2 := by
  decide +kernel
#guard (run [o 8 7005, o 8 7006]).nodeVersion == 5

-- step domain
theorem pin_refuse_code : stepRefusals init (o 9 0) = [.notAnOperation] := by decide +kernel
theorem pin_refuse_genesis0 : stepRefusals init (o 8 5) = [.notAnOperation] := by decide +kernel
#guard stepRefusals init (o 9 0) == [.notAnOperation]

end LedgerVerification
