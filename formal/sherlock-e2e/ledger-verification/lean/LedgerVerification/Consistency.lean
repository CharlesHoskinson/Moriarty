/-!
# SPEC-CHECKED requirements of Ledger Verification

MID-004, MID-006 and MID-007 have no state machine: they name records, a node build pin
and the absence of a Lean obligation. Each is a Boolean predicate over one design record;
`consistent` shows that one record satisfies them together with the constant requirements
of the context (MID-003) at the recorded values, so they do not contradict each other.
-/

namespace LedgerVerification.Consistency

structure Design where
  vkBytes : Nat
  lTx : Nat
  pubInputs : Nat
  txBytes : Nat
  txLimit : Nat
  qualRecords : List String
  previewBuild : String
  pinBuild : String
  midnightLeanObligations : Nat
  settlementKit : String
  deriving Repr

/-- MID-003: VK at most 50,000 bytes, at most 65,536 public inputs, `2 + L_TX` = 1,026,
transactions at most 1,048,576 bytes. -/
def MID_003_ok (d : Design) : Bool :=
  d.vkBytes ≤ 50000 && d.pubInputs ≤ 65536 && d.pubInputs == 2 + d.lTx && d.pubInputs == 1026
    && d.txBytes ≤ d.txLimit && d.txLimit == 1048576

/-- MID-004: the on-chain qualification records (VK bytes, proof bytes, verify time, fee at
1,026 inputs, a devnet deploy and call, a Preview deploy and call). Reason: a list of records,
no transition. -/
def MID_004_ok (d : Design) : Bool :=
  ["vk-bytes", "proof-bytes", "verify-time", "fee", "devnet-deploy-call", "preview-deploy-call"].all
    d.qualRecords.contains

/-- MID-006: the Preview qualification names build `2.1.101-bbd616b1` and the pin records the
same build. Reason: a configuration pin. -/
def MID_006_ok (d : Design) : Bool :=
  d.previewBuild == "2.1.101-bbd616b1" && d.pinBuild == d.previewBuild

/-- MID-007: no Lean obligation on the Midnight side; settlement rests on LDG-L1 (the
Settlement oracle `formal/sherlock-e2e/settlement` models LDG). Reason: a statement about
obligations. -/
def MID_007_ok (d : Design) : Bool := d.midnightLeanObligations == 0 && d.settlementKit == "LDG-L1"

def allOk (d : Design) : Bool := MID_003_ok d && MID_004_ok d && MID_006_ok d && MID_007_ok d

/-- The recorded values: VK 1,325 bytes and `acceptRun` 6,780 bytes (T17 (a),
`t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:17,19`), Preview
limit 1,048,576 (`t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/RESULTS.md:15`). -/
def recorded : Design :=
  { vkBytes := 1325, lTx := 1024, pubInputs := 1026, txBytes := 6780, txLimit := 1048576,
    qualRecords := ["vk-bytes", "proof-bytes", "verify-time", "fee", "devnet-deploy-call",
      "preview-deploy-call"],
    previewBuild := "2.1.101-bbd616b1", pinBuild := "2.1.101-bbd616b1",
    midnightLeanObligations := 0, settlementKit := "LDG-L1" }

theorem consistent : allOk recorded = true := by decide

/-- MID-004 is not vacuous: a record set without the Preview deploy fails it. -/
theorem MID_004_needs_preview : MID_004_ok { recorded with qualRecords := ["vk-bytes"] } = false := by decide
/-- MID-006 is not vacuous: another pinned build fails it. -/
theorem MID_006_needs_pin : MID_006_ok { recorded with pinBuild := "2.1.100" } = false := by decide

end LedgerVerification.Consistency
