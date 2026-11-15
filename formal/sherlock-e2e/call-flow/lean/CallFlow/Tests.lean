import CallFlow.Spec

/-!
# Known answers, written from the requirement sentences

Each answer is pinned twice: `decide +kernel` and `#guard`. `ok n` runs stage n with
success, `ko n` with failure. The universal wrap key is 5.
-/

namespace CallFlow

def o (code arg : Int) : Int := code * opBase + arg
def ok (n : Int) : Int := o 1 (n + 100)
def ko (n : Int) : Int := o 1 n
def stages (a b : Nat) : List Int := (List.range (b + 1 - a)).map (fun i => ok (((a + i : Nat)) : Int))
def full : List Int := stages 1 11

-- EUF-001: the eleven stages in order give one transaction and an equal read-back
theorem pin_full : (run 5 full).stage = 11 ∧ (run 5 full).outcome = 1 ∧ (run 5 full).txs = 1 := by
  decide +kernel
#guard (run 5 full).stage == 11 && (run 5 full).outcome == 1
theorem pin_order : stepRefusals (init 5) (ok 3) = [.opNotEnabled] := by decide +kernel
#guard stepRefusals (init 5) (ok 3) == [.opNotEnabled]

-- EUF-002 scenario: a second call starts at tape and sends no deployment
def second : List Int := full ++ [o 4 2] ++ stages 5 11
theorem pin_second_call : (run 5 (full ++ [o 4 2])).stage = 4 ∧ (run 5 second).deploys = 1 ∧
    (run 5 second).txTotal = 2 := by decide +kernel
#guard (run 5 second).deploys == 1 && (run 5 second).txTotal == 2

-- EUF-005: the wrap stage fails: no transaction, the stage is recorded
theorem pin_wrap_fails : (run 5 (stages 1 7 ++ [ko 8])).stopped = true ∧
    (run 5 (stages 1 7 ++ [ko 8])).failedStage = 8 ∧ (run 5 (stages 1 7 ++ [ko 8])).txs = 0 := by
  decide +kernel
#guard (run 5 (stages 1 7 ++ [ko 8])).failedStage == 8
-- EUF-005 scenario: the ledger check on muninn refuses: not submitted
theorem pin_check_fails : (run 5 (stages 1 8 ++ [ko 9])).txs = 0 := by decide +kernel
#guard (run 5 (stages 1 8 ++ [ko 9])).txs == 0

-- EUF-006: a native refusal at execute: no prove, no submit
theorem pin_native_refusal : (run 5 (stages 1 5 ++ [ko 6])).nativeRejected = true ∧
    (run 5 (stages 1 5 ++ [ko 6])).stage = 5 ∧
    stepRefusals (run 5 (stages 1 5 ++ [ko 6])) (ok 7) = [.opNotEnabled] := by decide +kernel
#guard (run 5 (stages 1 5 ++ [ko 6])).nativeRejected

-- EUF-007, EUF-012: a new key set after setup: the prove stage stops and names the key set
def staleProfile : List Int := stages 1 2 ++ [o 3 2] ++ stages 3 7
theorem pin_stale_profile : (run 5 staleProfile).stopped = true ∧ (run 5 staleProfile).failedPin = 4 ∧
    (run 5 staleProfile).failedStage = 7 := by decide +kernel
#guard (run 5 staleProfile).failedPin == 4
-- EUF-007 scenario: the box runs setup again for the new key set: the proof runs
theorem pin_setup_again : (run 5 (stages 1 2 ++ [o 3 2, o 2 2] ++ stages 3 7)).stage = 7 := by decide +kernel
#guard (run 5 (stages 1 2 ++ [o 3 2, o 2 2] ++ stages 3 7)).stage == 7

-- EUF-008: two boxes give one claim_pub
theorem pin_two_boxes : (run 5 (stages 1 7 ++ [o 6 1, o 6 2])).claimA = (run 5 (stages 1 7 ++ [o 6 1, o 6 2])).claimB ∧
    (run 5 (stages 1 7 ++ [o 6 1, o 6 2])).claimA = 1 := by decide +kernel
#guard (run 5 (stages 1 7 ++ [o 6 1, o 6 2])).claimB == 1

-- EUF-009: a second program with the same key deploys; with another key it is refused
theorem pin_second_program : (run 5 (stages 1 4 ++ [o 7 5])).programs = 2 ∧
    (run 5 (stages 1 4 ++ [o 7 6])).vkRefused = 1 ∧ (run 5 (stages 1 4 ++ [o 7 6])).programs = 1 := by
  decide +kernel
#guard (run 5 (stages 1 4 ++ [o 7 6])).vkRefused == 1

-- EUF-010: a read-back that differs is recorded as a mismatch
theorem pin_readback_differs : (run 5 (stages 1 10 ++ [ko 11])).outcome = 2 := by decide +kernel
#guard (run 5 (stages 1 10 ++ [ko 11])).outcome == 2

-- EUF-012: a ZKIR pin differs: stop, pin 2 named
theorem pin_zkir_pin : (run 5 [o 5 2]).stopped = true ∧ (run 5 [o 5 2]).failedPin = 2 := by decide +kernel
#guard (run 5 [o 5 2]).failedPin == 2

-- EUF-013: complete after every eligible call joined; not with a stopped eligible call;
-- not with two key sets
theorem pin_complete : (run 5 (full ++ [o 10 0])).complete = true := by decide +kernel
#guard (run 5 (full ++ [o 10 0])).complete
def stoppedSecond : List Int := full ++ [o 4 2] ++ stages 5 7 ++ [ko 8, o 10 0]
theorem pin_incomplete : (run 5 stoppedSecond).complete = false ∧ (run 5 stoppedSecond).completeBlocked = 1 := by
  decide +kernel
#guard !(run 5 stoppedSecond).complete
def mixed : List Int := full ++ [o 4 2, o 3 3, o 2 3] ++ stages 5 11 ++ [o 10 0]
theorem pin_mixed : (run 5 mixed).mixedKeys = true ∧ (run 5 mixed).complete = false := by decide +kernel
#guard (run 5 mixed).mixedKeys

-- EUF-014: lost response: reconciled by hash, no second transaction; no resend of the claim
def lost : List Int := stages 1 9 ++ [ko 10, o 9 1]
theorem pin_reconcile : (run 5 lost).stage = 11 ∧ (run 5 lost).txTotal = 1 ∧ (run 5 lost).outcome = 1 := by
  decide +kernel
#guard (run 5 lost).txTotal == 1
theorem pin_no_resend : (run 5 (lost ++ [o 4 1])).resendRefused = 1 ∧ (run 5 (lost ++ [o 4 1])).txTotal = 1 := by
  decide +kernel
#guard (run 5 (lost ++ [o 4 1])).resendRefused == 1
theorem pin_no_step_while_unknown : stepRefusals (run 5 (stages 1 9 ++ [ko 10])) (ok 11) = [.opNotEnabled] := by
  decide +kernel

end CallFlow
