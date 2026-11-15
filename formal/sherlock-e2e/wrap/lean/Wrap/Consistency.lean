import Wrap.Spec

/-!
# SPEC-CHECKED and CONSTANT-CHECKED rows of the Wrap context

The adopted circuit requirements (LW, DCD, GPW, CPR-004, DG-010, ANC-001) describe
what the wrap circuit and its build constrain, not a transition of the wrap tool.
Each gets one Boolean predicate `<ID>_ok` over the `Design` record, which holds the
design facts the sentences name. `consistent` proves that one design satisfies every
predicate together with the wrap-tool constants of `Spec.lean`: the adopted
requirements do not contradict each other or WRP-001 to WRP-012. A predicate is the
sentence's checkable part; the reason for SPEC-CHECKED is in its doc comment.
-/

namespace Wrap

/-- Design facts named by the adopted sentences. Hash and chip names are small codes. -/
structure Design where
  k : Int                    -- key size of the wrap
  rows : Int                 -- measured wrap rows at k
  vkBytes : Int              -- serialized ledger key
  lTx : Int                  -- call vector payload words
  publicInputs : Int         -- wrap public inputs
  anchorSlot : Int           -- ANCHOR_SLOT, template lanes 6 and 15
  anchorHoldsPlainProgD : Bool
  userAddressesSlot15 : Bool
  ledgerVerifierChanged : Bool   -- CPR-004, LW-017: ledger 9.1.0.1 verifier unchanged
  g4LaneGates : Bool         -- LW-013, LW-017
  foreignFieldChip : Bool    -- LW-013
  pairingChip : Bool
  shaChip : Bool
  zkStdLibArch : Bool        -- OD-24 option E
  accumulatorToLedger : Bool -- DCD-002
  haloRunDecider : Bool      -- DCD-001
  claimDArgs : List Int      -- DCD-003, LW-010: 0 SHK_RUN, 1 version, 2 prog_d, 3 input_d, 4 outcome_d, 9 n_steps
  collectionWords : List Int -- DCD-004: 0 reg_root, 1 set_root, 2 shape_d, 3 count
  rootOpenings : List Int    -- DCD-006, LW-008: 0 LEAF, 1 DEP (admitted); 2 SEG, 3 JOIN, 4 FOREIGN, 5 ADM, 6 VIEW
  ad20 : Int                 -- DCD-007, LW-011: 0 option (a), 1 option (b)
  depForeignChecked : Bool   -- option (b): refuse or check reg_root
  gpTargetsNarrowedOnFail : Bool -- DG-010
  gpuKernelOrder : List Int  -- GPW-002: K0 .. K6
  mskWitnessIndependent : Bool   -- GPW-004
  maskChangesProofBytes : Bool
  gpuMsmMsPer2p20 : Int      -- GPW-005: GPU MSM ms per 2^20 points
  msmSplit : Bool
  secondFrEngine : Bool      -- GPW-006, LW-014
  gpuWrapMs : Int            -- GPW-006: measured GPU wrap at the chosen k
  fallbackTriggered : Bool
  measuredWithRecord : Bool  -- GPW-007
  heavyJobsSerial : Bool
  leanObligationsGpu : Int   -- GPW-008: no Lean obligation for the GPU prover
  keygenRefusesSmallLog : Bool   -- LW-014
  keygenRefusesKeyDrift : Bool
  kitAxioms : List Int       -- LW-015: 0 propext, 1 Quot.sound, 2 Classical.choice, 3 sorryAx
  lPublicWords : Int         -- LW-016: 12,166
  lanePairs : Int            -- LW-016: 5
  allNopBenchmarkQualifies : Bool -- LW-016, FZ-3
  lfrExtensionField : Bool   -- LW-018
  lfrPoseidonU : List Int    -- LW-018: u = 5, 6, 7
  logBlowup : Int            -- LW-019
  lSkip : Int
  nStack : Int
  wStack : Int
  kWhir : Int
  queries : List Int
  queryPow : Int
  weakestBits10k : Int       -- LW-019: weakest term, bits * 10^4
  lVerifiesRoot : Bool       -- LW-001
  lFrVerifiesLUnderFrozenKey : Bool -- LW-002
  lFrFollowsProofStructure : Bool
  logChecksAll : Bool        -- LW-004: range, Canon, quotient, recomposition, native replay
  popeqEqualsAIn : Bool      -- LW-005
  canonOnX : Bool            -- LW-006
  xEqualsCachedCommitment : Bool
  runStatementWords : List Int  -- LW-007: 0 C_P, 1 input_pub, 2 outcome_pub
  nStepsPublic : Bool
  freshBlinders : Bool       -- ZK-003, LW-012 (also MODELED in Spec)
  deriving Repr

/-- CPR-004, LW-017: ledger 9.1.0.1 verifier unchanged; ZkStdLib key; no G4 gates. -/
def CPR_004_ok (d : Design) : Bool := !d.ledgerVerifierChanged && d.zkStdLibArch && !d.g4LaneGates
/-- ANC-001: slot 15, never plain prog_d, never addressed by user code. -/
def ANC_001_ok (d : Design) : Bool := d.anchorSlot == 15 && !d.anchorHoldsPlainProgD && !d.userAddressesSlot15
/-- DCD-001, LW-007: the run statement is [C_P, input_pub, outcome_pub]; no halo2 run decider. -/
def DCD_001_ok (d : Design) : Bool := d.runStatementWords == [0, 1, 2] && !d.haloRunDecider
/-- DCD-002: no accumulator to the ledger; L verified under the frozen key. -/
def DCD_002_ok (d : Design) : Bool := !d.accumulatorToLedger && d.lFrVerifiesLUnderFrozenKey
/-- DCD-003, LW-010: claim_d = P2(SHK_RUN, 1, prog_d, input_d, outcome_d), no n_steps. -/
def DCD_003_ok (d : Design) : Bool := d.claimDArgs == [0, 1, 2, 3, 4] && !d.claimDArgs.contains 9
/-- DCD-004: collection statement [reg_root, set_root], no shape_d, no count. -/
def DCD_004_ok (d : Design) : Bool := d.collectionWords == [0, 1]
/-- DCD-005, DG-002: 85 % of `2^k - 7` at the smallest k of 19, 20, 21 that fits. -/
def gateAt (k : Int) : Int := (85 * (2 ^ k.toNat - 7)) / 100
def smallestK (rows : Int) : Int :=
  if rows ≤ gateAt 19 then 19 else if rows ≤ gateAt 20 then 20 else 21
def DCD_005_ok (d : Design) : Bool := d.rows ≤ gateAt d.k && d.k == smallestK d.rows
/-- DCD-006, LW-008: only LEAF and DEP roots open; SEG, JOIN, FOREIGN, ADM, VIEW refused. -/
def DCD_006_ok (d : Design) : Bool := d.rootOpenings == [0, 1]
/-- DCD-007, LW-011: under option (b) a DEP head with a FOREIGN member is refused or checked. -/
def DCD_007_ok (d : Design) : Bool := d.ad20 == 0 || d.depForeignChecked
/-- DG-001, LW-013: no BLS12-381 circuit above k = 21. -/
def DG_001_ok (d : Design) : Bool := d.k ≤ 21
/-- DG-003, MID-003: VK at most 50,000 bytes. -/
def DG_003_ok (d : Design) : Bool := d.vkBytes ≤ 50000
/-- DG-010: a failed gate never narrows a GP target (process rule). -/
def DG_010_ok (d : Design) : Bool := !d.gpTargetsNarrowedOnFail
/-- GPW-002: kernels in the order K0 .. K6. -/
def GPW_002_ok (d : Design) : Bool := d.gpuKernelOrder == [0, 1, 2, 3, 4, 5, 6]
/-- GPW-004: the MSM mask is witness-independent and leaves the proof bytes unchanged. -/
def GPW_004_ok (d : Design) : Bool := d.mskWitnessIndependent && !d.maskChangesProofBytes
/-- GPW-005: the MSM stays split while the GPU MSM takes more than 0.2 s per 2^20 points. -/
def GPW_005_ok (d : Design) : Bool := d.gpuMsmMsPer2p20 ≤ 200 || d.msmSplit
/-- GPW-006, LW-014: no second Fr engine; a GPU wrap above 8 s triggers the fallback. -/
def GPW_006_ok (d : Design) : Bool := !d.secondFrEngine && (d.gpuWrapMs ≤ 8000 || d.fallbackTriggered)
/-- GPW-007: MEASURED only with run record; heavy jobs one at a time. -/
def GPW_007_ok (d : Design) : Bool := d.measuredWithRecord && d.heavyJobsSerial
/-- GPW-008: no Lean obligation for the GPU wrap prover (rests on GPW-003, LW-015). -/
def GPW_008_ok (d : Design) : Bool := d.leanObligationsGpu == 0
/-- LW-001: L verifies the node root (or path C leaf layer). -/
def LW_001_ok (d : Design) : Bool := d.lVerifiesRoot
/-- LW-002: L_Fr verifies L under the frozen key, by the schedule, not the proof structure. -/
def LW_002_ok (d : Design) : Bool := d.lFrVerifiesLUnderFrozenKey && !d.lFrFollowsProofStructure
/-- LW-003: exactly `2 + L_TX` public inputs with `L_TX` = 1,024. -/
def LW_003_ok (d : Design) : Bool := d.lTx == 1024 && d.publicInputs == 2 + d.lTx
/-- LW-004: every logged value checked, every entry replayed natively. -/
def LW_004_ok (d : Design) : Bool := d.logChecksAll
/-- LW-005: popeq lane equals a_in. -/
def LW_005_ok (d : Design) : Bool := d.popeqEqualsAIn
/-- LW-006: Canon on X, X equals the cached-trace commitment of L. -/
def LW_006_ok (d : Design) : Bool := d.canonOnX && d.xEqualsCachedCommitment
/-- LW-007: n_steps not public. -/
def LW_007_ok (d : Design) : Bool := DCD_001_ok d && !d.nStepsPublic
/-- LW-013: ZkStdLib wrap at k <= 21, no foreign-field, pairing or SHA chip, no G4 gate. -/
def LW_013_ok (d : Design) : Bool :=
  d.zkStdLibArch && d.k ≤ 21 && !d.foreignFieldChip && !d.pairingChip && !d.shaChip && !d.g4LaneGates
/-- LW-014: keygen refusals; no second Fr engine. -/
def LW_014_ok (d : Design) : Bool := d.keygenRefusesSmallLog && d.keygenRefusesKeyDrift && !d.secondFrEngine
/-- LW-015: the kit's axiom reports hold only propext, Quot.sound, Classical.choice. -/
def LW_015_ok (d : Design) : Bool := d.kitAxioms.all (· ≤ 2)
/-- LW-016: qualified over 12,166 public words at 5 lane-map pairs; rows from keygen fit the
gate; key measured against 50,000 bytes; an all-NOP benchmark qualifies nothing. -/
def LW_016_ok (d : Design) : Bool :=
  d.lPublicWords == 12166 && d.lanePairs == 5 && d.rows ≤ rowGate && d.vkBytes ≤ 50000
    && !d.allNopBenchmarkQualifies
/-- LW-017: ledger reads the key unchanged; no G4 gates. -/
def LW_017_ok (d : Design) : Bool := CPR_004_ok d
/-- LW-018: L_Fr over BLS12-381 Fr with no extension field; own hashes u = 5, 6, 7. -/
def LW_018_ok (d : Design) : Bool := !d.lfrExtensionField && d.lfrPoseidonU == [5, 6, 7]
/-- LW-019: L_Fr key parameters and bit floors (100, grindable 101). -/
def LW_019_ok (d : Design) : Bool :=
  d.logBlowup == 3 && d.lSkip == 2 && d.nStack == 18 && d.wStack ≤ 48 && d.kWhir == 4
    && d.queries == [98, 83, 82] && d.queryPow == 20 && d.weakestBits10k ≥ 1010000
/-- GPW-001: GPU prover with CPU fallback (key equality is MODELED by WRP-011). -/
def GPW_001_ok (d : Design) : Bool := !d.secondFrEngine
/-- GPW-003: one proof SHA-256 across configurations under GW_DET (byte identity is a run
check; the model holds only that the mask path does not change bytes). -/
def GPW_003_ok (d : Design) : Bool := !d.maskChangesProofBytes
/-- LW-012, ZK-003: fresh CSPRNG blinders (MODELED in Spec as `fresh`). -/
def LW_012_ok (d : Design) : Bool := d.freshBlinders

def allOk (d : Design) : Bool :=
  [CPR_004_ok d, ANC_001_ok d, DCD_001_ok d, DCD_002_ok d, DCD_003_ok d, DCD_004_ok d,
   DCD_005_ok d, DCD_006_ok d, DCD_007_ok d, DG_001_ok d, DG_003_ok d, DG_010_ok d,
   GPW_001_ok d, GPW_002_ok d, GPW_003_ok d, GPW_004_ok d, GPW_005_ok d, GPW_006_ok d,
   GPW_007_ok d, GPW_008_ok d, LW_001_ok d, LW_002_ok d, LW_003_ok d, LW_004_ok d, LW_005_ok d,
   LW_006_ok d, LW_007_ok d, LW_012_ok d, LW_013_ok d, LW_014_ok d, LW_015_ok d, LW_016_ok d,
   LW_017_ok d, LW_018_ok d, LW_019_ok d].all id

/-- The T10m design at its MEASURED values (LOOP.md:31: wrap 1,685,845 rows at k = 21,
VK 1,325 bytes, D3 weakest 101.0110 bits), AD-20 at option (a), the spec values elsewhere.
`gpuMsmMsPer2p20` and `gpuWrapMs` are witness choices, not MEASURED values. -/
def t10m : Design :=
  { k := 21, rows := 1685845, vkBytes := 1325, lTx := 1024, publicInputs := 1026, anchorSlot := 15,
    anchorHoldsPlainProgD := false, userAddressesSlot15 := false, ledgerVerifierChanged := false,
    g4LaneGates := false, foreignFieldChip := false, pairingChip := false, shaChip := false,
    zkStdLibArch := true, accumulatorToLedger := false, haloRunDecider := false,
    claimDArgs := [0, 1, 2, 3, 4], collectionWords := [0, 1], rootOpenings := [0, 1], ad20 := 0,
    depForeignChecked := false, gpTargetsNarrowedOnFail := false,
    gpuKernelOrder := [0, 1, 2, 3, 4, 5, 6], mskWitnessIndependent := true,
    maskChangesProofBytes := false, gpuMsmMsPer2p20 := 250, msmSplit := true, secondFrEngine := false,
    gpuWrapMs := 7000, fallbackTriggered := false, measuredWithRecord := true, heavyJobsSerial := true,
    leanObligationsGpu := 0, keygenRefusesSmallLog := true, keygenRefusesKeyDrift := true,
    kitAxioms := [0, 1, 2], lPublicWords := 12166, lanePairs := 5, allNopBenchmarkQualifies := false,
    lfrExtensionField := false, lfrPoseidonU := [5, 6, 7], logBlowup := 3, lSkip := 2, nStack := 18,
    wStack := 48, kWhir := 4, queries := [98, 83, 82], queryPow := 20, weakestBits10k := 1010110,
    lVerifiesRoot := true, lFrVerifiesLUnderFrozenKey := true, lFrFollowsProofStructure := false,
    logChecksAll := true, popeqEqualsAIn := true, canonOnX := true, xEqualsCachedCommitment := true,
    runStatementWords := [0, 1, 2], nStepsPublic := false, freshBlinders := true }

/-- The adopted wrap requirements hold together on one design (no contradiction). -/
theorem consistent : allOk t10m = true := by decide

/-- The same with AD-20 at option (b) and the DEP check on (DCD-007, LW-011 both options). -/
theorem consistent_ad20_b : allOk { t10m with ad20 := 1, depForeignChecked := true } = true := by decide

/-- DCD-005, DG-002 (CONSTANT-CHECKED): the measured T10m rows (LOOP.md:31) exceed the gate
at k = 20, so the smallest k of DG-002 is 21, which DG-001 admits: the two do not conflict. -/
theorem dg002_forces_k21 : smallestK 1685845 = 21 ∧ gateAt 20 < 1685845 ∧ gateAt 21 = rowGate := by
  decide

/-- The tool gate of WRP-003 is the DG-002 gate at k = 21. -/
theorem wrp003_is_dg002 : gateAt 21 = 1782573 := by decide

/-- LW-016 (CONSTANT-CHECKED): T4 budget with every check, 1,676,222 rows (LOOP.md:21),
fits the gate at k = 21. -/
theorem lw016_t4_rows : (1676222 : Int) ≤ gateAt 21 := by decide

/-- A design that keys at k = 22 fails DG-001 and LW-013 (the predicates are not vacuous). -/
theorem k22_rejected : DG_001_ok { t10m with k := 22 } = false ∧ LW_013_ok { t10m with k := 22 } = false := by
  decide

/-- A design with a 50,001-byte VK fails DG-003 and LW-016. -/
theorem vk_over_rejected : DG_003_ok { t10m with vkBytes := 50001 } = false ∧
    LW_016_ok { t10m with vkBytes := 50001 } = false := by decide

end Wrap
