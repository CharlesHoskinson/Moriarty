import Settlement.Spec

/-!
# Known answers, written from the requirement sentences

Each answer is pinned twice: `decide +kernel` (kernel) and `#guard` (compiled evaluator).
Parameters: collection proof time 200 s, inclusion time 100 s, registry root 5.
-/

namespace Settlement

def o (code arg : Int) : Int := code * opBase + arg
def r (ops : List Int) : St := run 200 100 5 ops

/-- Deploy program 7, build the arguments of call 1 (outcome 42, c_p 7), settle it. -/
def honest : List Int := [o 1 7, o 2 0, o 3 7421]

-- SLC-001 to SLC-004, STL-003: shape of a deployed instance
theorem pin_deploy : (r [o 1 7]).entries = 4 ∧ (r [o 1 7]).anchorSlot = 15 ∧ (r [o 1 7]).anchor = 7 ∧
    (r [o 1 7]).keyRun = keyWrap ∧ (r [o 1 7]).keyColl = keyWrap ∧ (r [o 1 7]).keyRoot = keyNative ∧
    (r [o 1 7]).keyRunRead = keyNative := by decide +kernel
#guard (r [o 1 7]).anchorSlot == 15 && (r [o 1 7]).keyRoot == keyNative

-- STL-004: the honest call sets runs[claim] to outcome 42
theorem pin_settle : (r honest).r1 = 42 ∧ (r honest).acceptRunTx = 1 ∧ (r honest).lastRefusal = 0 := by decide +kernel
#guard (r honest).r1 == 42 && (r honest).acceptRunTx == 1
-- LDG-002: a replay is refused (bit 8) and runs keeps its outcome
theorem pin_replay : (r (honest ++ [o 2 0, o 3 7991])).lastRefusal = rReplay ∧
    (r (honest ++ [o 2 0, o 3 7991])).r1 = 42 ∧ (r (honest ++ [o 2 0, o 3 7991])).acceptRunTx = 1 := by decide +kernel
#guard (r (honest ++ [o 2 0, o 3 7991])).lastRefusal == rReplay
-- SLC-005: program 8's call on program 7's ledger: custom error 115 (bit 4)
theorem pin_other_program : (r [o 1 7, o 2 0, o 3 8421]).lastRefusal = rOtherProgram ∧
    (r [o 1 7, o 2 0, o 3 8421]).acceptRunTx = 0 := by decide +kernel
#guard (r [o 1 7, o 2 0, o 3 8421]).lastRefusal == rOtherProgram
-- both node refusals apply: both bits
theorem pin_both_node : (r (honest ++ [o 2 0, o 3 8421])).lastRefusal = rOtherProgram + rReplay := by decide +kernel
#guard (r (honest ++ [o 2 0, o 3 8421])).lastRefusal == 12
-- SLC-009: a lane mismatch writes no argument file
theorem pin_mismatch : (r [o 1 7, o 2 1]).lastRefusal = rMismatch ∧ (r [o 1 7, o 2 1]).argFiles = 0 ∧
    (r [o 1 7, o 2 1]).argsReady = false := by decide +kernel
#guard (r [o 1 7, o 2 1]).argFiles == 0
-- no acceptRun without an argument file
theorem pin_no_args : stepRefusals (r [o 1 7]) (o 3 7421) = [.opNotEnabled] := by decide +kernel
#guard stepRefusals (r [o 1 7]) (o 3 7421) == [.opNotEnabled]
-- BAT-002: an operator's job goes to the collection, no acceptRun
theorem pin_operator : (r [o 1 7, o 2 0, o 3 107421]).acceptRunTx = 0 ∧ (r [o 1 7, o 2 0, o 3 107421]).routedJobs = 1 := by
  decide +kernel
#guard (r [o 1 7, o 2 0, o 3 107421]).routedJobs == 1
-- SLC-010: a natively refused call: no acceptRun, counted
theorem pin_native : (r [o 1 7, o 2 0, o 3 1007421]).acceptRunTx = 0 ∧ (r [o 1 7, o 2 0, o 3 1007421]).refusedNative = 1 ∧
    (r [o 1 7, o 2 0, o 3 1007421]).lastRefusal = rNative := by decide +kernel
#guard (r [o 1 7, o 2 0, o 3 1007421]).refusedNative == 1
-- SLC-012: an unbound statement: no live settle; with a native refusal too: both bits
theorem pin_unbound : (r [o 1 7, o 2 0, o 3 10007421]).lastRefusal = rUnbound ∧
    (r [o 1 7, o 2 0, o 3 11007421]).lastRefusal = rNative + rUnbound := by decide +kernel
#guard (r [o 1 7, o 2 0, o 3 10007421]).lastRefusal == rUnbound
-- STL-005: a settled outcome that differs from the native transcript is a mismatch
theorem pin_stl005 : (r [o 1 7, o 2 0, o 3 100007421]).mismatches = 1 := by decide +kernel
#guard (r [o 1 7, o 2 0, o 3 100007421]).mismatches == 1

-- BAT-001, BAT-007, BAT-008, LDG-006: 3 jobs, close at 3,600 s, one collection
def epoch3 : List Int := [o 11 0, o 5 0, o 5 0, o 5 0, o 4 3600, o 6 51]
theorem pin_epoch : (r epoch3).collTx = 1 ∧ (r epoch3).pathsGiven = 3 ∧ (r epoch3).leafBytes = 96 ∧
    (r epoch3).cr1 = true ∧ (r epoch3).epochOpen = false := by decide +kernel
#guard (r epoch3).collTx == 1 && (r epoch3).leafBytes == 96
-- BAT-001: the clock does not pass the close time; no job at the close time; no early close
theorem pin_no_overrun : stepRefusals (r [o 5 0]) (o 4 3601) = [.opNotEnabled] := by decide +kernel
theorem pin_no_late_job : stepRefusals (r [o 5 0, o 4 3600]) (o 5 0) = [.opNotEnabled] := by decide +kernel
theorem pin_no_early_close : stepRefusals (r [o 5 0, o 4 100]) (o 6 51) = [.opNotEnabled] := by decide +kernel
-- LDG-006: an empty epoch writes nothing (no epoch to close)
theorem pin_empty_epoch : stepRefusals (r []) (o 6 51) = [.opNotEnabled] := by decide +kernel
#guard stepRefusals (r [o 5 0]) (o 4 3601) == [.opNotEnabled]
-- BAT-009: without the L collection form the submit is refused
theorem pin_no_form : (r (epoch3.drop 1)).collTx = 0 ∧ (r (epoch3.drop 1)).blockedColl = 1 ∧
    (r (epoch3.drop 1)).lastRefusal = rNoForm := by decide +kernel
#guard (r (epoch3.drop 1)).blockedColl == 1
-- LDG-003: the same claims_root twice is refused; LDG-008: another registry root is refused
theorem pin_dup_root : (r (epoch3 ++ [o 5 0, o 4 3600, o 6 51])).lastRefusal = rDupRoot ∧
    (r (epoch3 ++ [o 5 0, o 4 3600, o 6 51])).collTx = 1 := by decide +kernel
theorem pin_registry : (r (epoch3.take 5 ++ [o 6 61])).lastRefusal = rRegistry := by decide +kernel
#guard (r (epoch3.take 5 ++ [o 6 61])).lastRefusal == rRegistry

-- BAT-010: era end 2,000: the epoch closes at 2,000 - 200 - 100 - 1 (one block) = 1,699
theorem pin_era_close : stepRefusals (r [o 7 2000, o 5 0]) (o 4 1700) = [.opNotEnabled] ∧
    stepRefusals (r [o 7 2000, o 5 0, o 4 1699]) (o 6 51) = [] := by decide +kernel
#guard stepRefusals (r [o 7 2000, o 5 0, o 4 1699]) (o 6 51) == []
-- BAT-011: end_eff in 500 s and a proving time of 757 s: refused; with end 1,000 it starts
theorem pin_era_run : (r [o 7 500, o 8 757]).lastRefusal = rEra ∧ (r [o 7 500, o 8 757]).runsStarted = 0 ∧
    (r [o 7 1000, o 8 757]).runsStarted = 1 := by decide +kernel
#guard (r [o 7 500, o 8 757]).lastRefusal == rEra

-- BAT-006: no TTL given: 3,600 s
theorem pin_ttl : (r [o 9 0]).intentTtl = 3600 ∧ (r [o 9 14400]).intentTtl = 14400 := by decide +kernel
#guard (r [o 9 0]).intentTtl == 3600
-- BAT-005: 3 chunks refused at 14,400 s; 2 chunks pass at the default TTL; one call passes
theorem pin_plan : (r [o 9 14400, o 10 2970]).planRefused = true ∧ (r [o 9 14400, o 10 2970]).lastK = 3 ∧
    (r [o 10 1980]).planRefused = false ∧ (r [o 10 1980]).lastK = 2 ∧ (r [o 10 1015]).lastK = 1 := by decide +kernel
#guard (r [o 9 14400, o 10 2970]).planRefused
-- ANC-012: below 1,755 s a chunked entry is refused, a one-call entry passes
theorem pin_short_ttl : (r [o 9 1754, o 10 1016]).planRefused = true ∧ (r [o 9 1754, o 10 1015]).planRefused = false := by
  decide +kernel
#guard (r [o 9 1754, o 10 1016]).planRefused

-- SLC-011: a changed state gets a new instance; an equal state keeps the instance and its runs
theorem pin_reuse : (r [o 1 7, o 12 0]).deploys = 2 ∧ (r [o 1 7, o 12 0]).anchor = 7 ∧
    (r (honest ++ [o 12 1])).r1 = 42 ∧ (r (honest ++ [o 12 1])).deploys = 1 := by decide +kernel
#guard (r [o 1 7, o 12 0]).deploys == 2

-- BAT-012: a freeze with no collection receipt lists collections as open
theorem pin_freeze_open : (r [o 13 0]).openListed = true ∧ (r (epoch3 ++ [o 13 0])).openListed = false := by decide +kernel
#guard (r [o 13 0]).openListed

-- step domain
theorem pin_refuse_code : stepRefusals (r []) (o 14 0) = [.notAnOperation] := by decide +kernel
theorem pin_refuse_claim4 : stepRefusals (r honest) (o 3 7424) = [.notAnOperation] := by decide +kernel
#guard stepRefusals (r []) (o 14 0) == [.notAnOperation]

end Settlement
