import SherlockExec.Spec

/-!
# Known answers, written from the requirement sentences

Each answer is pinned twice: `decide +kernel` (kernel) and `#guard` (compiled evaluator).
Digest values follow the formulas of `Spec.lean` (image 7, plan 1, key_root 2, tape digest 5:
`prog_d` 221007 for R_SL, `comm` 30, W digest 72, payload 30000162).
-/

namespace SherlockExec

def o (code arg : Int) : Int := code * opBase + arg

def admitRsl : Int := o 2 1221007       -- image 7, plan 1, key_root 2, mv 2, app key
def admitInterp : Int := o 2 121007     -- mv 1
def tapeOk : Int := o 3 80              -- tape digest 5, no flag
def halt : Int := o 4 1000301           -- Halted, 3 effect words, box 1
def honestRsl : List Int := [o 1 0, admitRsl, tapeOk, halt, o 5 0, o 6 1, o 7 30000162]
def honestInterp : List Int := [o 1 0, admitInterp, tapeOk, halt, o 5 0, o 6 1, o 7 30000162]

-- ERS-014: a missing field is refused by name; a wrong-type field too
theorem pin_missing_field : (run 0 [o 1 103]).stop = stopMalformed ∧ (run 0 [o 1 103]).refusedField = 3 := by
  decide +kernel
#guard (run 0 [o 1 103]).stop == stopMalformed && (run 0 [o 1 103]).refusedField == 3
theorem pin_wrong_type : (run 0 [o 1 214]).stop = stopMalformed ∧ (run 0 [o 1 214]).refusedField = 14 := by
  decide +kernel
#guard (run 0 [o 1 214]).refusedField == 14
theorem pin_field_15_outside : stepRefusals (init 0) (o 1 115) = [.opNotEnabled] := by decide +kernel

-- ERS-006, EUI-001: an admitted R_SL program has family 2 and prog_d = H(image, plan, key_root, mv)
theorem pin_admit : (run 0 [o 1 0, admitRsl]).family = 2 ∧ (run 0 [o 1 0, admitRsl]).progD = 221007 := by
  decide +kernel
#guard (run 0 [o 1 0, admitRsl]).progD == 221007
-- ERS-015: the app key is committed under key_root
theorem pin_app_key : (run 0 [o 1 0, admitRsl]).appKeyCommitted = true := by decide +kernel
#guard (run 0 [o 1 0, admitRsl]).appKeyCommitted
-- EUI-001 (LEAD PICK): machine version 3 names no family
theorem pin_version : (run 0 [o 1 0, o 2 321007]).stop = stopVersion := by decide +kernel
#guard (run 0 [o 1 0, o 2 321007]).stop == stopVersion

-- EKT-004: an R_SL program with both hashes is refused; an interpreted one is admitted
theorem pin_mixed_hash : (run 0 [o 1 0, o 2 110221007]).stop = stopMixedHash := by decide +kernel
#guard (run 0 [o 1 0, o 2 110221007]).stop == stopMixedHash
theorem pin_mixed_interp : (run 0 [o 1 0, o 2 110121007]).phase = 2 := by decide +kernel
#guard (run 0 [o 1 0, o 2 110121007]).phase == 2
theorem pin_sha_only : (run 0 [o 1 0, o 2 10221007]).phase = 2 := by decide +kernel
#guard (run 0 [o 1 0, o 2 10221007]).phase == 2

-- EUI-007: each tape defect is refused and starts no proof
theorem pin_tape_selector : (run 0 [o 1 0, admitRsl, o 3 81]).stop = stopBadTape := by decide +kernel
#guard (run 0 [o 1 0, admitRsl, o 3 81]).stop == stopBadTape
theorem pin_tape_noncanonical : (run 0 [o 1 0, admitRsl, o 3 82]).stop = stopBadTape := by decide +kernel
theorem pin_tape_missing : (run 0 [o 1 0, admitRsl, o 3 84]).stop = stopBadTape := by decide +kernel
theorem pin_tape_leftover : (run 0 [o 1 0, admitRsl, o 3 88]).stop = stopBadTape ∧
    (run 0 [o 1 0, admitRsl, o 3 88]).proofsStarted = 0 := by decide +kernel
#guard (run 0 [o 1 0, admitRsl, o 3 88]).proofsStarted == 0

-- ERS-005, ERS-003, EUI-008: a halted call gives native comm and W, 1 + 3 PUBLIC rows, 17 micro-ops
theorem pin_halt : (run 0 (honestRsl.take 4)).comm = 30 ∧ (run 0 (honestRsl.take 4)).wD = 72 ∧
    (run 0 (honestRsl.take 4)).publicRows = 4 ∧ (run 0 (honestRsl.take 4)).microOps = 17 := by decide +kernel
#guard (run 0 (honestRsl.take 4)).publicRows == 4 && (run 0 (honestRsl.take 4)).microOps == 17
-- EUI-008: another tape of the same entry takes the same count of micro-ops
theorem pin_static : (run 0 [o 1 0, admitRsl, o 3 160, halt]).microOps = 17 := by decide +kernel
#guard (run 0 [o 1 0, admitRsl, o 3 160, halt]).microOps == 17

-- EKT-011: an ALU result at or above the modulus rejects the call
theorem pin_alu : (run 0 [o 1 0, admitRsl, tapeOk, o 4 1000311]).outcome = outRejected := by decide +kernel
#guard (run 0 [o 1 0, admitRsl, tapeOk, o 4 1000311]).outcome == outRejected
-- EUI-004: an R_SL call cannot end Exhausted; the interpreted family can (EUI-005)
theorem pin_rsl_no_exhausted :
    stepRefusals (run 0 [o 1 0, admitRsl, tapeOk]) (o 4 1000303) = [.opNotEnabled] := by decide +kernel
theorem pin_interp_exhausted : (run 0 [o 1 0, admitInterp, tapeOk, o 4 1000303]).outcome = outExhausted := by
  decide +kernel
#guard (run 0 [o 1 0, admitInterp, tapeOk, o 4 1000303]).outcome == outExhausted

-- ERS-009: a rejected R_SL call has no statement; GP-001: an interpreted Rejected run keeps one
theorem pin_rsl_reject : (run 0 [o 1 0, admitRsl, tapeOk, o 4 1000302, o 5 0]).phase = 8 ∧
    (run 0 [o 1 0, admitRsl, tapeOk, o 4 1000302, o 5 0]).stmtCount = 0 := by decide +kernel
#guard (run 0 [o 1 0, admitRsl, tapeOk, o 4 1000302, o 5 0]).stmtCount == 0
theorem pin_interp_reject : (run 0 [o 1 0, admitInterp, tapeOk, o 4 1000302, o 5 0]).stmtCount = 1 ∧
    (run 0 [o 1 0, admitInterp, tapeOk, o 4 1000302, o 5 0]).outcomeKind = outRejected := by decide +kernel
#guard (run 0 [o 1 0, admitInterp, tapeOk, o 4 1000302, o 5 0]).stmtCount == 1

-- ERS-007, ERS-008, ERS-010: one statement binding the tape digest, Halted, payload vd(comm ++ W)
theorem pin_statement : (run 0 (honestRsl.take 5)).stmtCount = 1 ∧ (run 0 (honestRsl.take 5)).inputVal = 5 ∧
    (run 0 (honestRsl.take 5)).outcomeKind = 1 ∧ (run 0 (honestRsl.take 5)).outcomePayload = 30000162 := by
  decide +kernel
#guard (run 0 (honestRsl.take 5)).outcomePayload == 30000162

-- EKT-007: a kit-relation violation is refused by the app verifier
theorem pin_kit : (run 0 (honestRsl.take 5 ++ [o 6 0])).lastRefusal = refKit ∧
    (run 0 (honestRsl.take 5 ++ [o 6 0])).proofAccepted = false := by decide +kernel
#guard (run 0 (honestRsl.take 5 ++ [o 6 0])).lastRefusal == refKit

-- ERS-012: recursion takes only the statement's outcome commitment
theorem pin_unbound : (run 0 (honestRsl.take 6 ++ [o 7 30000163])).lastRefusal = refUnbound ∧
    (run 0 (honestRsl.take 6 ++ [o 7 30000163])).phase = 6 := by decide +kernel
#guard (run 0 (honestRsl.take 6 ++ [o 7 30000163])).lastRefusal == refUnbound
theorem pin_bound : (run 0 honestRsl).phase = 9 ∧ (run 0 honestRsl).boundOutcome = 30000162 := by decide +kernel
#guard (run 0 honestRsl).phase == 9

-- ERS-002: a private record to a box the user does not own is refused; StatementPublics leave
theorem pin_private_out : (run 0 (honestRsl.take 5 ++ [o 8 0])).lastRefusal = refPrivate := by decide +kernel
#guard (run 0 (honestRsl.take 5 ++ [o 8 0])).lastRefusal == refPrivate
theorem pin_private_owned : (run 0 (honestRsl.take 5 ++ [o 8 1])).lastRefusal = refNone := by decide +kernel
theorem pin_publics_out : (run 0 (honestRsl.take 5 ++ [o 8 10])).publicsOut = 1 := by decide +kernel
#guard (run 0 (honestRsl.take 5 ++ [o 8 10])).publicsOut == 1
theorem pin_publics_need_stmt : stepRefusals (run 0 (honestRsl.take 4)) (o 8 10) = [.opNotEnabled] := by
  decide +kernel

-- EUI-002, EUI-003: both families reach recursion under one universal key set
theorem pin_two_families : (run 42 honestRsl).ukey = 42 ∧ (run 42 honestInterp).ukey = 42 ∧
    (run 42 honestRsl).phase = 9 ∧ (run 42 honestInterp).phase = 9 ∧
    (run 42 (honestRsl ++ [o 9 0] ++ honestInterp)).ukey = 42 := by decide +kernel
#guard (run 42 (honestRsl ++ [o 9 0] ++ honestInterp)).ukey == 42

-- step domain
theorem pin_refuse_code : stepRefusals (init 0) (o 10 0) = [.notAnOperation] := by decide +kernel
theorem pin_refuse_flag : stepRefusals (run 0 [o 1 0]) (o 2 221007 + 2000000) = [.notAnOperation] := by
  decide +kernel
#guard stepRefusals (init 0) (o 5 0) == [.opNotEnabled]

end SherlockExec
