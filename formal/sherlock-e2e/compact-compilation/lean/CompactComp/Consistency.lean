import CompactComp.Spec

/-!
# SPEC-CHECKED rows of Compact Compilation

These requirements have no state machine of their own (a toolchain shape, a report form,
a proof-library obligation). Each is one Bool predicate `<ID>_ok` over a `Design` record;
`consistent` shows that one design satisfies all of them together with the modelled
requirements they touch, so the set is free of contradiction. `cf001_ecc001_conflict`
shows the one contradiction found (finding FAL-COMPACT-COMPILATION-1): without the prototype scope that
only a status scenario and D-016 give, CF-001 and ECC-001 cannot both hold.
-/

namespace CompactComp.Consistency

structure Design where
  prototype : Bool          -- the build is the e2e prototype (D-016 scope)
  readsLflattened : Bool    -- CF-001: the frontend taps `Lflattened` inside compactc
  readsZkirOnly : Bool      -- ECC-001, MYC-FE-001: only ZKIR 3.1 + contract-info.json
  commands : Nat            -- CF-002: bit mask of compile, plan, simulate, deploy, prove, call
  abiNe : Bool              -- CF-002, ECC-003: `n_e` in the ABI
  abiLe : Bool              -- CF-002, ECC-003: `l_e` in the ABI
  abiTapeLen : Bool         -- CF-002, ECC-003: `tape_len` in the ABI
  abiKits : Bool            -- CF-002 (count per builtin), ECC-003 (`kits`)
  cfL1Proved : Bool         -- CF-012: CF-L1 proved with the allowed axioms only
  kitComplete : Bool        -- CF-012: the compact-frontend kit reported complete
  gatesPass : Bool          -- ECC-002, LPC-013
  leanWfail : Bool          -- LPC-013: `lake build --wfail` of the Mycroft Lean
  t18dCommitted : Bool      -- LPC-013: the T18 (d) deliverables are committed
  s1Discharged : Bool       -- LPC-019: premise S1 proved
  s1NamedHyp : Bool         -- LPC-019: premise S1 kept as a named hypothesis
  designClosed : Bool       -- LPC-019: the Mycroft design closes
  thmsStateSemantics : Bool -- MYC-LOW-019: each exported theorem states source, target, trust
  x1Checks : Nat            -- MYC-TRN-017: number of decision-X1 checks reported
  frameOnlyDiff : Bool      -- MYC-TRN-017: the frame is the only declared difference
  deriving Repr, DecidableEq

/-- CF-001 holds outside the prototype scope (superseded for the prototype by D-016). -/
def CF_001_ok (d : Design) : Bool := d.prototype || d.readsLflattened
/-- ECC-001 as the prototype reads it. -/
def ECC_001_ok (d : Design) : Bool := d.readsZkirOnly && !d.readsLflattened
def CF_002_ok (d : Design) : Bool := d.commands = 63 && d.abiNe && d.abiLe && d.abiTapeLen && d.abiKits
/-- ECC-003 names the same ABI fields as CF-002 (`n_e`, `l_e`, `tape_len`, `kits`). -/
def ECC_003_abi_ok (d : Design) : Bool := d.abiNe && d.abiLe && d.abiTapeLen && d.abiKits
def CF_012_ok (d : Design) : Bool := !d.kitComplete || d.cfL1Proved
def LPC_013_ok (d : Design) : Bool := !d.t18dCommitted || (d.gatesPass && d.leanWfail)
def LPC_019_ok (d : Design) : Bool := !d.designClosed || d.s1Discharged || d.s1NamedHyp
def MYC_LOW_019_ok (d : Design) : Bool := d.thmsStateSemantics
def MYC_TRN_017_ok (d : Design) : Bool := d.x1Checks = 3 && d.frameOnlyDiff
/-- ECC-002: the compiler is used for a settled call only with every gate passing. -/
def ECC_002_ok (d : Design) (usedForSettle : Bool) : Bool := !usedForSettle || d.gatesPass

def allOk (d : Design) : Bool :=
  CF_001_ok d && ECC_001_ok d && CF_002_ok d && ECC_003_abi_ok d && CF_012_ok d && LPC_013_ok d
    && LPC_019_ok d && MYC_LOW_019_ok d && MYC_TRN_017_ok d && ECC_002_ok d true

/-- The prototype design of 2026-11-15: ZKIR input, gates passing (REPORT.md:12-18),
CF-L1 open so the kit is not reported complete (ECC-002 status), S1 kept as a named
hypothesis (SORRIES.md, LPC-019). -/
def witness : Design :=
  { prototype := true, readsLflattened := false, readsZkirOnly := true, commands := 63,
    abiNe := true, abiLe := true, abiTapeLen := true, abiKits := true, cfL1Proved := false,
    kitComplete := false, gatesPass := true, leanWfail := true, t18dCommitted := true,
    s1Discharged := false, s1NamedHyp := true, designClosed := true, thmsStateSemantics := true,
    x1Checks := 3, frameOnlyDiff := true }

theorem consistent : allOk witness = true := by decide

/-- FAL-COMPACT-COMPILATION-1: outside the prototype scope, CF-001 and ECC-001 contradict. -/
theorem cf001_ecc001_conflict (d : Design) (h : d.prototype = false) :
    ¬ (CF_001_ok d = true ∧ ECC_001_ok d = true) := by
  cases d; simp_all [CF_001_ok, ECC_001_ok]

end CompactComp.Consistency
