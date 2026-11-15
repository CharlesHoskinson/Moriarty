import Governance.Spec

/-!
# Known answers, written from the requirement sentences

Parameters of every pin: `TL = 10`, `RTL = 5`, `Q = 2`, `n = 3`, `GRID = 10` (test values;
EGV-008 lists the real values as open at T15). Each answer is pinned twice:
`decide +kernel` (kernel) and `#guard` (compiled evaluator).
-/

namespace Governance

def o (code arg : Int) : Int := code * opBase + arg
def r (ops : List Int) : St := run 10 5 2 3 10 ops

/-- Deploy the era table and era ledgers 1 and 2, all with a passing readback. -/
def setup : List Int := [o 9 10100, o 9 110101, o 9 210101]
/-- Announce era 1 (key set 7, ledger 1, start 10). -/
def ann1 : Int := o 2 1001072
/-- Era 1 active from block time 10. -/
def era1 : List Int := setup ++ [ann1, o 1 10, o 3 1]
/-- Announce era 2 (key set 8, ledger 2, start 30). -/
def ann2 : Int := o 2 3002082

-- GOV-001: one signature is below the quorum Q = 2
theorem pin_quorum : (r (setup ++ [o 2 1001071])).lastRefusal = rNoQuorum ∧
    (r (setup ++ [o 2 1001071])).nextId = 1 := by decide +kernel
#guard (r (setup ++ [o 2 1001071])).lastRefusal == rNoQuorum
-- ERT-002: the era table is used only after its readback gate
theorem pin_table_first : (r [ann1]).lastRefusal = rTableNotSealed := by decide +kernel
#guard (r [ann1]).lastRefusal == rTableNotSealed
-- GOV-002, GOV-004: T >= now + TL; T = now + TL is accepted
theorem pin_too_early : (r (setup ++ [o 2 1072])).lastRefusal = rTooEarly := by decide +kernel
#guard (r (setup ++ [o 2 1072])).lastRefusal == rTooEarly
theorem pin_announce : (r (setup ++ [ann1])).pending = 1 ∧ (r (setup ++ [ann1])).e1.st = 1 ∧
    (r (setup ++ [ann1])).e1.start = 10 := by decide +kernel
#guard (r (setup ++ [ann1])).pending == 1
-- ERT-009: a start off the grid is refused
theorem pin_grid : (r (setup ++ [o 2 1501072])).lastRefusal = rOffGrid := by decide +kernel
#guard (r (setup ++ [o 2 1501072])).lastRefusal == rOffGrid
-- ELD-002: a ledger with no readback gate; EGV-002: a ledger whose readback has threshold 0
theorem pin_ledger_not_deployed : (r (setup ++ [o 2 1003072])).lastRefusal = rLedgerNotGated := by decide +kernel
#guard (r (setup ++ [o 2 1003072])).lastRefusal == rLedgerNotGated
theorem pin_threshold0 : (r [o 9 10100, o 9 310001, o 2 1003072]).l3 = 2 ∧
    (r [o 9 10100, o 9 310001, o 2 1003072]).lastRefusal = rLedgerNotGated := by decide +kernel
#guard (r [o 9 10100, o 9 310001, o 2 1003072]).lastRefusal == rLedgerNotGated
-- EGV-002 (LP-2): threshold 2 also fails the readback {[], 1, 0}
theorem pin_threshold2 : (r [o 9 10200]).tableSealed = false ∧ (r [o 9 10200]).unusable = 1 := by decide +kernel
#guard !(r [o 9 10200]).tableSealed

-- GOV-003: activate before start is refused; at start it activates
theorem pin_activate_early : (r (setup ++ [ann1, o 3 1])).lastRefusal = rNotStarted := by decide +kernel
#guard (r (setup ++ [ann1, o 3 1])).lastRefusal == rNotStarted
theorem pin_activate : (r era1).current = 1 ∧ (r era1).e1.st = 2 ∧ (r era1).pending = 0 := by decide +kernel
#guard (r era1).current == 1

-- ERT-003, EGV-004: a key set or a ledger of an earlier era is refused
theorem pin_keyset_reuse : (r (era1 ++ [o 2 3002072])).lastRefusal = rKeysetReused := by decide +kernel
#guard (r (era1 ++ [o 2 3002072])).lastRefusal == rKeysetReused
theorem pin_ledger_reuse : (r (era1 ++ [o 2 3001082])).lastRefusal = rLedgerReused := by decide +kernel
#guard (r (era1 ++ [o 2 3001082])).lastRefusal == rLedgerReused

-- ERT-004, ERT-005: at block time 30 the pending era 2 accepts and era 1 does not
def two : List Int := era1 ++ [ann2, o 1 20]
theorem pin_handover : active (r two) 1 = false ∧ active (r two) 2 = true := by decide +kernel
#guard !active (r two) 1 && active (r two) 2
theorem pin_before_handover : active (r (era1 ++ [ann2])) 1 = true ∧ active (r (era1 ++ [ann2])) 2 = false := by
  decide +kernel
#guard active (r (era1 ++ [ann2])) 1
-- GOV-003, ERT-007: activation writes end(1) = start(2) once
theorem pin_end_once : (r (two ++ [o 3 2])).e1.fin = 30 ∧ (r (two ++ [o 3 2])).e1.st = 3 ∧
    (r (two ++ [o 3 2])).current = 2 := by decide +kernel
#guard (r (two ++ [o 3 2])).e1.fin == 30

-- GOV-007: cancel before start; the id is not used again
theorem pin_cancel : (r (era1 ++ [ann2, o 4 22])).e2.st = 4 ∧ (r (era1 ++ [ann2, o 4 22])).pending = 0 ∧
    (r (era1 ++ [ann2, o 4 22, o 9 310101, o 2 3003092])).pending = 3 := by decide +kernel
#guard (r (era1 ++ [ann2, o 4 22, o 9 310101, o 2 3003092])).pending == 3
-- LP-3: the cancelled era 2 still holds ledger 2
theorem pin_cancel_holds : (r (era1 ++ [ann2, o 4 22, o 2 3002092])).lastRefusal = rLedgerReused := by
  decide +kernel
#guard (r (era1 ++ [ann2, o 4 22, o 2 3002092])).lastRefusal == rLedgerReused
theorem pin_cancel_late : (r (two ++ [o 4 22])).lastRefusal = rCancelLate := by decide +kernel
#guard (r (two ++ [o 4 22])).lastRefusal == rCancelLate

-- GOV-006: soundness end of era 1 at time 10 cancels the pending era 2
theorem pin_soundness : (r (era1 ++ [ann2, o 5 2])).e1.st = 5 ∧ (r (era1 ++ [ann2, o 5 2])).e1.fin = 10 ∧
    (r (era1 ++ [ann2, o 5 2])).e2.st = 4 ∧ (r (era1 ++ [ann2, o 5 2])).current = 0 := by decide +kernel
#guard (r (era1 ++ [ann2, o 5 2])).e2.st == 4
-- GOV-006: a due activation is applied first
theorem pin_soundness_due : (r (two ++ [o 5 2])).e1.st = 3 ∧ (r (two ++ [o 5 2])).e2.st = 5 ∧
    (r (two ++ [o 5 2])).e2.fin = 30 := by decide +kernel
#guard (r (two ++ [o 5 2])).e2.st == 5
theorem pin_soundness_none : (r (setup ++ [o 5 2])).lastRefusal = rNoActiveEra := by decide +kernel
#guard (r (setup ++ [o 5 2])).lastRefusal == rNoActiveEra

-- ELD-003, ELD-004: on the primary path a call outside the era's interval is refused
def later : List Int := two ++ [o 3 2]
theorem pin_eld_refuse : (r (later ++ [o 15 1])).lastRefusal = rEraInactive ∧ (r (later ++ [o 15 1])).c1 = 0 := by
  decide +kernel
#guard (r (later ++ [o 15 1])).lastRefusal == rEraInactive
-- ELD-004: a call for a cancelled era is refused whole (a state outcome)
theorem pin_eld_cancelled : (r (era1 ++ [ann2, o 4 22, o 15 2])).lastRefusal = rEraInactive ∧
    (r (era1 ++ [ann2, o 4 22, o 15 2])).c2 = 0 := by decide +kernel
#guard (r (era1 ++ [ann2, o 4 22, o 15 2])).lastRefusal == rEraInactive
theorem pin_eld_accept : (r (era1 ++ [o 15 1])).c1 = 1 := by decide +kernel
#guard (r (era1 ++ [o 15 1])).c1 == 1
-- ELD-007, EGV-006, CRR-006: with premise A2 failed the ledger accepts and the consumer checks at use
def fb : List Int := era1 ++ [o 12 22] ++ [ann2, o 1 20, o 3 2]
theorem pin_fallback : (r (fb ++ [o 15 1])).c1 = 1 ∧ (r (fb ++ [o 15 1, o 16 1])).lastRefusal = rEraInactive ∧
    (r (fb ++ [o 15 1, o 16 1])).uses = 0 := by decide +kernel
#guard (r (fb ++ [o 15 1, o 16 1])).lastRefusal == rEraInactive
-- CRR-004: a claim of a soundness-ended era is refused
theorem pin_unsafe : (r (era1 ++ [o 15 1, o 5 2, o 16 1])).lastRefusal = rSoundnessEnded := by decide +kernel
#guard (r (era1 ++ [o 15 1, o 5 2, o 16 1])).lastRefusal == rSoundnessEnded
-- CRR-007: a use writes only the consumer's nullifier
theorem pin_use : (r (era1 ++ [o 15 1, o 16 1])).uses = 1 ∧
    (r (era1 ++ [o 15 1, o 16 1])).c1 = (r (era1 ++ [o 15 1])).c1 := by decide +kernel
#guard (r (era1 ++ [o 15 1, o 16 1])).uses == 1

-- EGV-005: the rekey runner refuses with no era row and no failed premise
theorem pin_rekey_refused : (r [o 14 0]).lastRefusal = rRekeyNoEraCheck ∧ (r [o 14 0]).rekeyStarted = false := by
  decide +kernel
#guard (r [o 14 0]).lastRefusal == rRekeyNoEraCheck
theorem pin_rekey_row : (r [o 13 0, o 14 0]).rekeyStarted = true := by decide +kernel
#guard (r [o 13 0, o 14 0]).rekeyStarted
theorem pin_rekey_fallback : (r [o 12 32, o 14 0]).rekeyStarted = true := by decide +kernel
#guard (r [o 12 32, o 14 0]).rekeyStarted
-- EGV-005: a failed A1 alone does not open the fallback
theorem pin_rekey_a1 : (r [o 12 12, o 14 0]).rekeyStarted = false := by decide +kernel
#guard !(r [o 12 12, o 14 0]).rekeyStarted
-- EGV-009: one result per premise
theorem pin_premise : (r [o 12 11, o 12 22, o 12 31]).prem1 = 1 ∧ (r [o 12 11, o 12 22, o 12 31]).prem2 = 2 ∧
    stepRefusals (r [o 12 11]) (o 12 12) = [.opNotEnabled] := by decide +kernel
#guard stepRefusals (r [o 12 11]) (o 12 12) == [.opNotEnabled]

-- EGV-007, GOV-009: a verifier-key change is refused and changes nothing else
theorem pin_vk_change : (r (era1 ++ [o 10 0])).lastRefusal = rVkChange ∧
    { r (era1 ++ [o 10 0]) with lastRefusal := 0 } = { r era1 with lastRefusal := 0 } := by decide +kernel
#guard (r (era1 ++ [o 10 0])).lastRefusal == rVkChange
-- EGV-003: an unsealed ledger's receipts are path evidence
theorem pin_path_evidence : (r [o 11 0]).pathEvidence = 1 ∧ (r [o 11 1]).pathEvidence = 0 := by decide +kernel
#guard (r [o 11 0]).pathEvidence == 1

-- GOV-005: a rotation takes effect at now + RTL and changes no era
theorem pin_rotate : (r (era1 ++ [o 6 52])).roster = 1 ∧ (r (era1 ++ [o 6 52, o 1 5])).roster = 5 ∧
    (r (era1 ++ [o 6 52, o 1 5])).e1 = (r era1).e1 := by decide +kernel
#guard (r (era1 ++ [o 6 52, o 1 5])).roster == 5
-- GOV-011: a pending rotation is cancelled before it takes effect
theorem pin_cancel_rotation : (r [o 6 52, o 7 2, o 1 5]).roster = 1 := by decide +kernel
#guard (r [o 6 52, o 7 2, o 1 5]).roster == 1
theorem pin_no_rotation : (r [o 7 2]).lastRefusal = rNoRotation := by decide +kernel
#guard (r [o 7 2]).lastRefusal == rNoRotation
-- GOV-008: an attested root is usable only TL after it was recorded
theorem pin_attest : (r [o 8 92, o 17 0]).lastRefusal = rAttestNotUsable ∧
    (r [o 8 92, o 1 10, o 17 0]).attestUses = 1 := by decide +kernel
#guard (r [o 8 92, o 1 10, o 17 0]).attestUses == 1
-- GOV-010: every accepted signed verb increments the nonce (announce, announce, cancel, rotate)
theorem pin_nonce : (r (era1 ++ [ann2, o 4 22, o 6 52])).nonce = 4 := by decide +kernel
#guard (r (era1 ++ [ann2, o 4 22, o 6 52])).nonce == 4

-- step domain
theorem pin_refuse_code : stepRefusals (r []) (o 18 0) = [.notAnOperation] := by decide +kernel
#guard stepRefusals (r []) (o 18 0) == [.notAnOperation]
theorem pin_refuse_tick : stepRefusals (r [o 9 10100]) (o 9 10100) = [.opNotEnabled] := by decide +kernel
#guard stepRefusals (r [o 9 10100]) (o 9 10100) == [.opNotEnabled]

end Governance
