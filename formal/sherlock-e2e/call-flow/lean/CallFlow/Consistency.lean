import CallFlow.Spec

/-!
# SPEC-CHECKED rows of Call Flow

EUF-011 (no maintainer tooling), LPC-014 (the freeze presentation), LPC-017 (formula
updates) and MYC-ACC-004 (transcript version pin) are process or configuration rules with
no transition here. Each is one Bool predicate over a `Setup` record; `consistent` shows
that one setup satisfies all of them with the EUF-012 pin rule.
-/

namespace CallFlow.Consistency

structure Setup where
  usesDoctorWatson : Bool   -- EUF-011
  usesHerdr : Bool          -- EUF-011
  usesBoxBus : Bool         -- EUF-011
  usesAgentTool : Bool      -- EUF-011
  rows : Nat                -- LPC-014: LOOP rows
  rowsDone : Nat            -- LPC-014: rows DONE with a lead verdict
  rowsOwner : Nat           -- LPC-014: rows waiting on the owner
  freezePresented : Bool    -- LPC-014
  rulingsChangingFormula : Nat -- LPC-017
  docsUpdated : Nat         -- LPC-017: rulings carried into FINAL-DESIGN.md and ARCHITECTURE.md
  transcriptMajor : Nat     -- MYC-ACC-004: transcript version 2.3
  transcriptMinor : Nat
  ledgerPinned : Bool       -- EUF-012: the ledger version is one of the run pins
  deriving Repr, DecidableEq

def EUF_011_ok (c : Setup) : Bool :=
  !c.usesDoctorWatson && !c.usesHerdr && !c.usesBoxBus && !c.usesAgentTool
def LPC_014_ok (c : Setup) : Bool := (c.rowsDone + c.rowsOwner != c.rows) || c.freezePresented
def LPC_017_ok (c : Setup) : Bool := c.docsUpdated = c.rulingsChangingFormula
def MYC_ACC_004_ok (c : Setup) : Bool := c.transcriptMajor = 2 && c.transcriptMinor = 3
/-- MYC-ACC-004 and EUF-012 agree: the transcript version is part of the ledger pin. -/
def EUF_012_ledger_ok (c : Setup) : Bool := c.ledgerPinned

def allOk (c : Setup) : Bool :=
  EUF_011_ok c && LPC_014_ok c && LPC_017_ok c && MYC_ACC_004_ok c && EUF_012_ledger_ok c

def witness : Setup :=
  { usesDoctorWatson := false, usesHerdr := false, usesBoxBus := false, usesAgentTool := false,
    rows := 30, rowsDone := 28, rowsOwner := 2, freezePresented := true,
    rulingsChangingFormula := 1, docsUpdated := 1, transcriptMajor := 2, transcriptMinor := 3,
    ledgerPinned := true }

theorem consistent : allOk witness = true := by decide

/-- EUF-011 against the prototype runs today (EUF-011 status: `t7_chain`, `t17.sh`, `t23.py`
and the lead's box bus): such a run does not meet EUF-011, so EUF-011 is not yet shown. -/
theorem euf011_today_open : EUF_011_ok { witness with usesBoxBus := true } = false := by decide

end CallFlow.Consistency
