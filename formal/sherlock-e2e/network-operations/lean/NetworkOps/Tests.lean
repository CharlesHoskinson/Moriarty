import NetworkOps.Spec

/-!
# Known answers, written from the requirement sentences

Each answer is pinned twice: `decide +kernel` and `#guard`.
-/

namespace NetworkOps

def o (code arg : Int) : Int := code * opBase + arg

/-- Start to a confirmed final deploy on Preview (PVW-009, PVW-013). -/
def toDeployed : List Int :=
  [o 1 3600, o 2 2, o 3 1, o 4 1, o 5 1, o 6 1, o 7 63, o 8 0]
/-- The honest call on an owned GPU box, included and read back (PVW-010, OBS-001). -/
def honest : List Int := toDeployed ++ [o 9 1, o 11 1, o 12 0, o 13 1, o 15 1, o 16 0]
/-- The four controls, each refused by the node (PVW-006). -/
def controlsRefused : List Int := [o 12 1, o 13 0, o 12 2, o 13 0, o 12 3, o 13 0, o 12 4, o 13 0]

-- SCL-002, PVW-006: the full honest run passes with exit 0
theorem pin_pass : (run 0 (honest ++ controlsRefused ++ [o 18 0])).exitCode = 0 ∧
    (run 0 (honest ++ controlsRefused ++ [o 18 0])).controlsRefused = 4 := by decide +kernel
#guard (run 0 (honest ++ controlsRefused ++ [o 18 0])).exitCode == 0

-- SCL-007: TTL 0 and 1,209,600 stop; 1,209,599 starts
theorem pin_ttl0 : (run 0 [o 1 0]).stop = stopTtl := by decide +kernel
#guard (run 0 [o 1 0]).stop == stopTtl
theorem pin_ttl_bound : (run 0 [o 1 1209600]).stop = stopTtl := by decide +kernel
#guard (run 0 [o 1 1209600]).stop == stopTtl
theorem pin_ttl_ok : (run 0 [o 1 1209599]).phase = 1 := by decide +kernel
#guard (run 0 [o 1 1209599]).phase == 1

-- PVW-001, PVW-002: another chain stops before any wallet step
theorem pin_chain : (run 0 [o 1 60, o 2 3]).stop = stopChain ∧ (run 0 [o 1 60, o 2 3]).walletRead = false ∧
    stepRefusals (run 0 [o 1 60, o 2 3]) (o 4 1) = [.opNotEnabled] := by decide +kernel
#guard (run 0 [o 1 60, o 2 3]).stop == stopChain
theorem pin_devnet : (run 0 [o 1 60, o 2 1]).phase = 2 := by decide +kernel
#guard (run 0 [o 1 60, o 2 1]).phase == 2

-- SCL-004: a state folder of another key is refused
theorem pin_state : (run 0 [o 1 60, o 2 2, o 3 0]).stop = stopStateVk := by decide +kernel
#guard (run 0 [o 1 60, o 2 2, o 3 0]).stop == stopStateVk

-- WAL-010: wide modes stop before the phrase is read
theorem pin_modes : (run 0 [o 1 60, o 2 2, o 3 1, o 4 0]).stop = stopMode ∧
    (run 0 [o 1 60, o 2 2, o 3 1, o 4 0]).walletRead = false := by decide +kernel
#guard (run 0 [o 1 60, o 2 2, o 3 1, o 4 0]).stop == stopMode

-- WAL-003: another address stops before the wallet starts
theorem pin_addr : (run 0 [o 1 60, o 2 2, o 3 1, o 4 1, o 5 0]).stop = stopAddr ∧
    (run 0 [o 1 60, o 2 2, o 3 1, o 4 1, o 5 0]).walletStarted = false := by decide +kernel
#guard (run 0 [o 1 60, o 2 2, o 3 1, o 4 1, o 5 0]).stop == stopAddr

-- PVW-013: a final deploy with 4 of 5 gate passes is refused
theorem pin_gate : (run 0 (toDeployed.take 6 ++ [o 7 62])).lastRefusal = refGate ∧
    (run 0 (toDeployed.take 6 ++ [o 7 62])).deploys = 0 := by decide +kernel
#guard (run 0 (toDeployed.take 6 ++ [o 7 62])).lastRefusal == refGate

-- FRC-002: a sent, unconfirmed deploy is read back; no second deploy
theorem pin_deploy_resume : (run 0 (toDeployed.take 7 ++ [o 7 63])).deploys = 1 ∧
    (run 0 (toDeployed.take 7 ++ [o 7 63])).phase = 6 := by decide +kernel
#guard (run 0 (toDeployed.take 7 ++ [o 7 63])).deploys == 1

-- PVW-010: the final call is not proved on the wallet machine CPU, nor on a box not owned
theorem pin_final_cpu : (run 0 (toDeployed ++ [o 9 0])).lastRefusal = refFinalNotGpu := by decide +kernel
#guard (run 0 (toDeployed ++ [o 9 0])).lastRefusal == refFinalNotGpu
theorem pin_not_owned : (run 0 (toDeployed ++ [o 9 2])).lastRefusal = refNotOwned := by decide +kernel
#guard (run 0 (toDeployed ++ [o 9 2])).lastRefusal == refNotOwned

-- FRC-004, OBS-008: four failed balancing attempts with resyncs stop the run
def fail4 : List Int := toDeployed ++ [o 9 1, o 11 0, o 19 0, o 11 0, o 19 0, o 11 0, o 19 0, o 11 0]
theorem pin_attempts : (run 0 fail4).stop = stopBalance ∧ (run 0 fail4).balanceAttempts = 4 ∧
    (run 0 fail4).errRecords = 4 := by decide +kernel
#guard (run 0 fail4).stop == stopBalance
-- FRC-004 (LEAD PICK): another wallet error stops at once
theorem pin_other_error : (run 0 (toDeployed ++ [o 9 1, o 11 2])).stop = stopBalance ∧
    (run 0 (toDeployed ++ [o 9 1, o 11 2])).balanceAttempts = 1 := by decide +kernel
#guard (run 0 (toDeployed ++ [o 9 1, o 11 2])).stop == stopBalance
-- FRC-004: no new attempt without a resync
theorem pin_resync : stepRefusals (run 0 (toDeployed ++ [o 9 1, o 11 0])) (o 11 1) = [.opNotEnabled] := by
  decide +kernel

-- WAL-007: a submit from a machine without the wallet is refused
theorem pin_offhost : (run 0 (toDeployed ++ [o 9 1, o 11 1, o 12 10])).lastRefusal = refOffHost ∧
    (run 0 (toDeployed ++ [o 9 1, o 11 1, o 12 10])).submitted = false := by decide +kernel
#guard (run 0 (toDeployed ++ [o 9 1, o 11 1, o 12 10])).lastRefusal == refOffHost

-- PVW-012: an included control fails the verdict (exit 4)
def r2Included : List Int := honest ++ [o 12 1, o 13 0, o 12 2, o 13 1, o 12 3, o 13 0, o 12 4, o 13 0, o 18 0]
theorem pin_r2 : (run 0 r2Included).exitCode = 4 ∧ (run 0 r2Included).controlFails = 1 := by decide +kernel
#guard (run 0 r2Included).exitCode == 4
-- PVW-007: no node verdict fails the verdict
def r1NoVerdict : List Int := honest ++ [o 12 1, o 13 3, o 12 2, o 13 0, o 12 3, o 13 0, o 12 4, o 13 0, o 18 0]
theorem pin_noverdict : (run 0 r1NoVerdict).exitCode = 4 := by decide +kernel
#guard (run 0 r1NoVerdict).exitCode == 4
-- PVW-008: a control timeout fails the verdict
def r1Timeout : List Int := honest ++ [o 12 1, o 13 2, o 12 2, o 13 0, o 12 3, o 13 0, o 12 4, o 13 0, o 18 0]
theorem pin_ctrl_timeout : (run 0 r1Timeout).exitCode = 4 := by decide +kernel
#guard (run 0 r1Timeout).exitCode == 4

-- PVW-008, FRC-013: an honest timeout leaves the state unknown; no resubmit until resolved
def timedOut : List Int := toDeployed ++ [o 9 1, o 11 1, o 12 0, o 13 2]
theorem pin_timeout : (run 0 timedOut).unknown = true ∧ (run 0 timedOut).lastVerdict = vTimeout ∧
    stepRefusals (run 0 timedOut) (o 12 0) = [.opNotEnabled] := by decide +kernel
#guard (run 0 timedOut).unknown
-- FRC-013: resolved as included by the indexer; the saved call is not marked refused
theorem pin_resolve : (run 0 (timedOut ++ [o 14 1])).included = true ∧
    (run 0 (timedOut ++ [o 14 1])).savedRefused = false := by decide +kernel
#guard (run 0 (timedOut ++ [o 14 1])).included

-- FRC-005: a refused honest call is dropped on the rerun and a new proof follows
def refused : List Int := toDeployed ++ [o 9 1, o 11 1, o 12 0, o 13 0]
theorem pin_refused : (run 0 (refused ++ [o 10 7201])).saved = false ∧
    (run 0 (refused ++ [o 10 7201, o 9 1])).proofs = 2 := by decide +kernel
#guard (run 0 (refused ++ [o 10 7201, o 9 1])).proofs == 2

-- FRC-003: 301 s of TTL left and a good hash reuse; 300 s does not; FRC-014 a bad hash does not
def provenOnly : List Int := toDeployed ++ [o 9 1]
theorem pin_reuse : (run 0 (provenOnly ++ [o 10 603])).saved = true := by decide +kernel
#guard (run 0 (provenOnly ++ [o 10 603])).saved
theorem pin_no_reuse_300 : (run 0 (provenOnly ++ [o 10 601])).saved = false := by decide +kernel
#guard !(run 0 (provenOnly ++ [o 10 601])).saved
theorem pin_no_reuse_hash : (run 0 (provenOnly ++ [o 10 602])).saved = false := by decide +kernel
#guard !(run 0 (provenOnly ++ [o 10 602])).saved

-- FRC-006: the refusals step needs the readback
def noReadback : List Int := toDeployed ++ [o 9 1, o 11 1, o 12 0, o 13 1, o 16 0]
theorem pin_no_readback : (run 0 noReadback).lastRefusal = refNoReadback ∧
    (run 0 noReadback).refusalsStarted = false := by decide +kernel
#guard (run 0 noReadback).lastRefusal == refNoReadback

-- SCL-002: a stopped run exits 1
theorem pin_exit1 : (run 0 [o 1 0, o 18 0]).exitCode = 1 := by decide +kernel
#guard (run 0 [o 1 0, o 18 0]).exitCode == 1

-- SCL-003: the dry run opens no connection and passes
theorem pin_dry : (run 1 [o 1 60, o 17 0, o 18 0]).netConns = 0 ∧ (run 1 [o 1 60, o 18 0]).exitCode = 0 ∧
    stepRefusals (run 1 [o 1 60]) (o 2 2) = [.opNotEnabled] := by decide +kernel
#guard (run 1 [o 1 60, o 18 0]).exitCode == 0

-- WAL-002: a secret in a log line is redacted
theorem pin_redact : (run 0 [o 17 1, o 17 0]).redactions = 1 ∧ (run 0 [o 17 1, o 17 0]).logLines = 2 := by
  decide +kernel
#guard (run 0 [o 17 1, o 17 0]).redactions == 1

-- WAL-004: the lock cannot be released while a call is balanced
theorem pin_lock : stepRefusals (run 0 (toDeployed ++ [o 9 1, o 11 1])) (o 6 0) = [.opNotEnabled] := by
  decide +kernel

-- PVW-011, LPC-009: two batches settle; another program and a stale root are refused
def validium : List Int := [o 20 (1 + 2 * 0 + 2000 * 5), o 20 (1 + 2 * 5 + 2000 * 9),
  o 20 (0 + 2 * 9 + 2000 * 11), o 20 (1 + 2 * 5 + 2000 * 11)]
theorem pin_validium : (run 0 validium).vBatches = 2 ∧ (run 0 validium).vRefused = 2 ∧
    (run 0 validium).vRoot = 9 := by decide +kernel
#guard (run 0 validium).vBatches == 2 && (run 0 validium).vRoot == 9

-- step domain
theorem pin_not_op : stepRefusals (init false) (o 21 0) = [.notAnOperation] := by decide +kernel
theorem pin_disabled : stepRefusals (init false) (o 12 0) = [.opNotEnabled] := by decide +kernel

end NetworkOps
