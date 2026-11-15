import Conformance.Spec

/-!
# Known answers, written from the requirement sentences

Each answer is pinned twice: `decide +kernel` and `#guard`. `cfg` 31 = dispatch, Preview
and all three kits; 3 = dispatch and Preview, no kit; 1 = dispatch only; 2 = Preview only.
-/

namespace Conformance

def o (code arg : Int) : Int := code * opBase + arg
/-- One honest live call: native ok, W equal, proved, both checks, included, equal. -/
def honest (id : Int) : List Int := [o 1 id, o 2 1, o 3 1, o 4 0, o 5 3, o 6 1, o 7 3]

-- ECS-004, ECS-014: an honest live call passes with all seven evidence items; complete
theorem pin_pass : (run 31 (honest 1)).cls = clsPass ∧ (run 31 (honest 1)).evidence = 7 := by decide +kernel
#guard (run 31 (honest 1)).cls == clsPass
theorem pin_complete : (run 31 (honest 1 ++ [o 8 0])).complete = true := by decide +kernel
#guard (run 31 (honest 1 ++ [o 8 0])).complete

-- ECS-004 scenario: the pure circuit is n/a
theorem pin_na : (run 31 [o 1 1, o 2 3, o 3 1]).cls = clsNa := by decide +kernel
#guard (run 31 [o 1 1, o 2 3, o 3 1]).cls == clsNa

-- ECS-002 scenario: a rejected reference has no transcript words
theorem pin_rejected_ref : (run 31 [o 1 1, o 2 2]).transcriptWords = 0 ∧
    (run 31 [o 1 1, o 2 1]).transcriptWords = 1 := by decide +kernel
#guard (run 31 [o 1 1, o 2 2]).transcriptWords == 0

-- ECS-009: no executed rejection: DERIVED; executed rejection: not DERIVED
theorem pin_derived : (run 31 [o 1 1, o 2 2, o 3 1]).cls = clsRefused ∧
    (run 31 [o 1 1, o 2 2, o 3 1]).derived = true ∧ (run 31 [o 1 1, o 2 2, o 3 3]).derived = false := by
  decide +kernel
#guard (run 31 [o 1 1, o 2 2, o 3 1]).derived && !(run 31 [o 1 1, o 2 2, o 3 3]).derived

-- ECS-003 scenario: W differs at compile: mismatch, no prove stage, not compared
theorem pin_compile_mismatch : (run 31 [o 1 1, o 2 1, o 3 0]).cls = clsMismatch ∧
    (run 31 [o 1 1, o 2 1, o 3 0]).proveRan = false ∧ (run 31 [o 1 1, o 2 1, o 3 0]).compared = 0 := by
  decide +kernel
#guard (run 31 [o 1 1, o 2 1, o 3 0]).cls == clsMismatch

-- ECS-006: keccak256 kit absent: blocked with the kit reason; present: compiled
theorem pin_kit_absent : (run 3 [o 1 1, o 2 1, o 3 17]).reason = rKeccak := by decide +kernel
#guard (run 3 [o 1 1, o 2 1, o 3 17]).reason == rKeccak
theorem pin_kit_present : (run 31 [o 1 1, o 2 1, o 3 17]).stage = 3 := by decide +kernel
#guard (run 31 [o 1 1, o 2 1, o 3 17]).stage == 3

-- ECS-005: no R_SL chip set; no native Preview transcript
theorem pin_chipset : (run 31 [o 1 1, o 2 1, o 3 1, o 4 2]).reason = rChipSet := by decide +kernel
#guard (run 31 [o 1 1, o 2 1, o 3 1, o 4 2]).reason == rChipSet
theorem pin_no_native : (run 31 [o 1 1, o 2 1, o 3 1, o 4 0, o 5 3, o 6 1, o 7 2]).reason = rNoNative := by
  decide +kernel
#guard (run 31 [o 1 1, o 2 1, o 3 1, o 4 0, o 5 3, o 6 1, o 7 2]).reason == rNoNative

-- ECS-007: dispatch off: blocked, no box message
theorem pin_dispatch_off : (run 30 [o 1 1, o 2 1, o 3 1, o 4 0]).boxMsgs = 0 ∧
    (run 30 [o 1 1, o 2 1, o 3 1, o 4 0]).reason = rDispatchOff := by decide +kernel
#guard (run 30 [o 1 1, o 2 1, o 3 1, o 4 0]).boxMsgs == 0

-- ECS-008, ECS-014: settle dry: no Preview transaction; a pass after a dry settle leaves
-- the run incomplete (missing receipt)
theorem pin_settle_dry : (run 29 (honest 1)).previewTx = 0 ∧ (run 29 (honest 1)).evidence = 5 ∧
    (run 29 (honest 1)).eligibleIncomplete = 1 := by decide +kernel
#guard (run 29 (honest 1)).previewTx == 0
theorem pin_incomplete_report : (run 29 (honest 1 ++ [o 8 0])).complete = false ∧
    (run 29 (honest 1 ++ [o 8 0])).completeBlocked = 1 := by decide +kernel
#guard !(run 29 (honest 1 ++ [o 8 0])).complete

-- ECS-012: the dry run: 0 of 0; one mismatch after a proof: 1 of 1
theorem pin_denominator_dry : (run 0 []).mismatchCompared = 0 ∧ (run 0 []).compared = 0 := by decide +kernel
def mism : List Int := [o 1 1, o 2 1, o 3 1, o 4 0, o 5 3, o 6 1, o 7 1]
theorem pin_denominator : (run 31 mism).mismatchCompared = 1 ∧ (run 31 mism).compared = 1 := by decide +kernel
#guard (run 31 mism).mismatchCompared == 1 && (run 31 mism).compared == 1

-- ECS-010 scenario: one mismatch in the live run: row T23 not done
theorem pin_mismatch_not_done : (run 31 (mism ++ [o 8 0])).complete = false := by decide +kernel
#guard !(run 31 (mism ++ [o 8 0])).complete

-- ECS-013: a repeated identity is not counted twice
theorem pin_duplicate : (run 31 (honest 1 ++ [o 1 1])).dupRefused = 1 ∧
    (run 31 (honest 1 ++ [o 1 1])).calls = 1 := by decide +kernel
#guard (run 31 (honest 1 ++ [o 1 1])).dupRefused == 1

-- step domain
theorem pin_refuse_order : stepRefusals (init true true 7) (o 4 0) = [.opNotEnabled] := by decide +kernel
theorem pin_refuse_kit : stepRefusals (init true true 7) (o 3 5) = [.notAnOperation] := by decide +kernel
#guard stepRefusals (init true true 7) (o 4 0) == [.opNotEnabled]

end Conformance
