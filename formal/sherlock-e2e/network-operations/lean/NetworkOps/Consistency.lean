import NetworkOps.Spec

/-!
# Consistency of the requirements with no state machine (SPEC-CHECKED)

These requirements name tool surfaces, file layouts or loop process. They have
no transition the replay can check. Each has one Bool predicate `<ID>_ok` over
one design record `Cfg`; `consistent` shows that one record (the values the
spec states) satisfies every predicate together with the state-machine facts the
oracle fixes (`passed`, `ttlMax`, the chain set). A contradiction between two
requirements would make `consistent` false. Small decision functions named by a
requirement (privacy scan, push hold, suite skip, setup tool, figure labels) are
defined here with their own theorems.

Codes (Nat): steps of PVW-005 `check`=0, `balance-before`=1, `deploy`=2, `call`=3,
`refusals`=4, `balance-after`=5, `receipts`=6. Labels of OBS-009: MEASURED=0,
DERIVED=1, ESTIMATE=2. Fee source of PVW-003: chain parameters=1,
`initialParameters()`=0. Chains of PVW-002: devnet `undeployed1`=1, pinned
Preview=2, Mainnet=3.
-/

namespace NetworkOps

structure Cfg where
  feeSource : Nat
  proofServerPort : Nat
  devnetPort : Nat
  proofServerVersion : Nat      -- 9.0.1 as 901
  proofServerLoopback : Bool
  steps : List Nat
  phraseInProcess : Bool
  phraseInEnvOrArgs : Bool
  receiptFields : List Nat
  allowList : List Nat
  balanceFields : List Nat      -- tDUST=0, NIGHT=1, tip=2, address=3
  keyMaterial : List Nat        -- phrase=10, seed=11, private key=12, r_P=13
  repoSecrets : Nat
  sdkCommands : Nat
  e5jsonKeys : List Nat         -- k, keygen_s, prove_s, proof_bytes, key_bytes_midnightvk, library_verify_ok, public_inputs
  compactc : Nat                -- 0.36.0 as 3600
  suiteCommands : Nat
  userConfigItems : Nat         -- wallet dir, pinned address, chain pin, box roster
  factoryNamesInUserRun : Nat
  scanSurfaces : Nat
  manifestCovers : List Nat     -- inputs=0, L_Fr key=1, binaries=2
  suiteStages : Nat
  guiModes : Nat
  unitAboveMinutes : Nat
  gateFailsToStop : Nat
  integrationMerge : Nat        -- `t20-batched` 5dc1be2ee as 1
  publishedView : List Nat      -- call vector=20, wrap proof=21
  forbidden : List Nat          -- prog_d=30, pack(prog_d)=31, pack(r_P)=32, R=33, n_steps=34, h_hi=35, r_P=36
  anchorCarries : List Nat      -- C_P=40, A_j=41
  mainnetConds : Nat

def stepsAll : List Nat := [0, 1, 2, 3, 4, 5, 6]

def PVW_003_ok (c : Cfg) : Bool := c.feeSource = 1
def PVW_004_ok (c : Cfg) : Bool :=
  c.proofServerPort = 6341 && c.proofServerPort ≠ c.devnetPort && c.proofServerVersion = 901 && c.proofServerLoopback
def PVW_005_ok (c : Cfg) : Bool := c.steps = stepsAll && c.steps.length = 7
/-- WAL-001: the phrase is read inside the runner's own process (the order of the read is
modelled in `Spec`: `walletRead` after the chain, state and mode checks). -/
def WAL_001_ok (c : Cfg) : Bool := c.phraseInProcess && !c.phraseInEnvOrArgs
def WAL_005_ok (c : Cfg) : Bool := c.receiptFields.all (c.allowList.contains ·)
def WAL_006_ok (c : Cfg) : Bool := c.balanceFields.all (fun f => !c.keyMaterial.contains f)
def WAL_008_ok (c : Cfg) : Bool := c.repoSecrets = 0
def SCL_001_ok (c : Cfg) : Bool := c.sdkCommands = 7
def SCL_005_ok (c : Cfg) : Bool := c.e5jsonKeys.length = 7
def SCL_006_ok (c : Cfg) : Bool := c.compactc = 3600
def SCL_008_ok (c : Cfg) : Bool := c.suiteCommands = 2
def SCL_010_ok (c : Cfg) : Bool := c.userConfigItems = 4 && c.factoryNamesInUserRun = 0
def OBS_003_ok (c : Cfg) : Bool := c.scanSurfaces = 3
def OBS_006_ok (c : Cfg) : Bool := [0, 1, 2].all (c.manifestCovers.contains ·)
def OBS_007_ok (c : Cfg) : Bool := c.suiteStages = 5
def OBS_012_ok (c : Cfg) : Bool := c.guiModes = 2
/-- LPC-011 names the same two GUI modes as OBS-012. -/
def LPC_011_ok (c : Cfg) : Bool := c.guiModes = 2
def FRC_007_ok (c : Cfg) : Bool := c.unitAboveMinutes = 30
def FRC_009_ok (c : Cfg) : Bool := c.gateFailsToStop = 2
def LPC_010_ok (c : Cfg) : Bool := c.integrationMerge = 1
/-- ZK-002: the published view is the call vector and one wrap proof; WAL-005 and OBS-003
must agree: no forbidden word is published. -/
def ZK_002_ok (c : Cfg) : Bool :=
  c.publishedView = [20, 21] && c.publishedView.all (fun f => !c.forbidden.contains f)
/-- ZK-007: the anchor carries `C_P` or a lock value, never `prog_d` (a forbidden word). -/
def ZK_007_ok (c : Cfg) : Bool := c.anchorCarries.all (fun f => !c.forbidden.contains f) && c.forbidden.contains 30
/-- SWF-009: four Mainnet conditions; PVW-002 keeps the settle runner off Mainnet whatever
they say (the runner's chain set has no Mainnet). -/
def SWF_009_ok (c : Cfg) : Bool := c.mainnetConds = 4 && !([1, 2].contains 3)

/-- The values the spec states. -/
def specCfg : Cfg :=
  { feeSource := 1, proofServerPort := 6341, devnetPort := 6340, proofServerVersion := 901,
    proofServerLoopback := true, steps := stepsAll, phraseInProcess := true, phraseInEnvOrArgs := false,
    receiptFields := [20, 50, 51, 52], allowList := [20, 50, 51, 52, 53], balanceFields := [0, 1, 2, 3],
    keyMaterial := [10, 11, 12, 13], repoSecrets := 0, sdkCommands := 7, e5jsonKeys := [0, 1, 2, 3, 4, 5, 6],
    compactc := 3600, suiteCommands := 2, userConfigItems := 4, factoryNamesInUserRun := 0,
    scanSurfaces := 3, manifestCovers := [0, 1, 2], suiteStages := 5, guiModes := 2,
    unitAboveMinutes := 30, gateFailsToStop := 2, integrationMerge := 1, publishedView := [20, 21],
    forbidden := [30, 31, 32, 33, 34, 35, 36], anchorCarries := [40, 41], mainnetConds := 4 }

def allOk (c : Cfg) : Bool :=
  PVW_003_ok c && PVW_004_ok c && PVW_005_ok c && WAL_001_ok c && WAL_005_ok c && WAL_006_ok c &&
  WAL_008_ok c && SCL_001_ok c && SCL_005_ok c && SCL_006_ok c && SCL_008_ok c && SCL_010_ok c &&
  OBS_003_ok c && OBS_006_ok c && OBS_007_ok c && OBS_012_ok c && LPC_011_ok c && FRC_007_ok c &&
  FRC_009_ok c && LPC_010_ok c && ZK_002_ok c && ZK_007_ok c && SWF_009_ok c

/-- Every SPEC-CHECKED predicate holds together at the spec's values. -/
theorem consistent : allOk specCfg = true := by decide

/-! ## Decision functions named by a requirement -/

/-- OBS-003, OBS-004: the privacy scan over the surfaces (call vector, transcript, ledger
state). Each surface reports whether a forbidden word was found and whether every control
word (`claim_pub`, `outcome_pub`, anchor) was found. -/
def scanPass (surfaces : List (Bool × Bool)) : Bool :=
  surfaces.all (fun (forbiddenFound, controlsFound) => !forbiddenFound && controlsFound)

theorem OBS_004_missing_control_fails (pre post : List (Bool × Bool)) (f : Bool) :
    scanPass (pre ++ (f, false) :: post) = false := by
  simp [scanPass, List.all_append]
theorem OBS_003_forbidden_fails (pre post : List (Bool × Bool)) (c : Bool) :
    scanPass (pre ++ (true, c) :: post) = false := by
  simp [scanPass, List.all_append]
theorem OBS_003_clean_passes : scanPass [(false, true), (false, true), (false, true)] = true := by decide

/-- OBS-009: a figure carries one of three labels; a sum of measured figures is DERIVED. -/
def labelOfSum (a b : Nat) : Nat := if a = 0 && b = 0 then 1 else max a b
def OBS_009_ok (l : Nat) : Bool := l ≤ 2
theorem OBS_009_sum_of_measured : labelOfSum 0 0 = 1 ∧ OBS_009_ok (labelOfSum 0 0) = true := by decide

/-- OBS-005: one `runs.log` line per step start and end; FRC-001: a skipped step (marker
present) still logs one line ("skip deploy (ok)"). -/
def runsLogLines (ran skipped : Nat) : Nat := 2 * ran + skipped
theorem OBS_005_lines : runsLogLines 7 0 = 14 ∧ runsLogLines 5 2 = 12 := by decide

/-- FRC-001: the marker `<step>.ok` is written exactly for exit status 0; a rerun skips a
step whose marker exists. -/
def markerWritten (status : Nat) : Bool := status = 0
def skipStep (markers : List Nat) (step : Nat) : Bool := markers.contains step
theorem FRC_001_marker : markerWritten 0 = true ∧ markerWritten 4 = false ∧
    skipStep [0, 1, 2] 2 = true ∧ skipStep [0, 1, 2] 3 = false := by decide

/-- FRC-011 and LPC-015: one push rule. FRC-011 says "before the owner signs T15", LPC-015
"before T15"; the same decision function serves both (they agree). -/
def holdPush (isT10q t15Signed : Bool) : Bool := !isT10q && !t15Signed
def FRC_011_ok (isT10q t15Signed held : Bool) : Bool := held == holdPush isT10q t15Signed
def LPC_015_ok (isT10q t15Signed held : Bool) : Bool := held == (!isT10q && !t15Signed)
theorem FRC_011_LPC_015_agree (a b h : Bool) : FRC_011_ok a b h = LPC_015_ok a b h := by
  cases a <;> cases b <;> cases h <;> rfl
theorem FRC_011_holds_other_branch : holdPush false false = true ∧ holdPush true false = false := by decide

/-- FRC-012: on a restart, the suite driver settles only the calls whose readback is not
yet their `outcome_pub`. A call is `(id, settled)`. No settled call is sent again. -/
def toSettle (calls : List (Nat × Bool)) : List Nat := (calls.filter (fun c => !c.2)).map (·.1)
theorem FRC_012_no_call_twice (calls : List (Nat × Bool)) (i : Nat)
    (hs : (i, true) ∈ calls) (hu : ∀ b, (i, b) ∈ calls → b = true) : i ∉ toSettle calls := by
  simp only [toSettle, List.mem_map, List.mem_filter, not_exists, not_and]
  intro x hx hxi
  obtain ⟨a, b⟩ := x
  simp only at hxi; subst hxi
  have := hu b hx.1
  simp_all
/-- FRC-012 scenario: 60 of 145 settled, 85 to send. -/
theorem FRC_012_scenario :
    (toSettle ((List.range 145).map (fun i => (i, decide (i < 60))))).length = 85 := by decide

/-- WAL-009: the setup tool creates a wallet only where no phrase file exists. -/
def setupCreates (phraseExists : Bool) : Bool := !phraseExists
theorem WAL_009_refuses : setupCreates true = false ∧ setupCreates false = true := by decide

/-- SCL-009: the SDK node suite starts only with `SHERLOCK_SDK_ROOT` set. -/
def suiteStarts (rootSet : Bool) : Bool := rootSet
theorem SCL_009_stops : suiteStarts false = false := by decide

/-- FRC-008: after a crash every killed run restarts from its resumable state as a new unit;
FRC-010: a DONE row has every file committed; FRC-015: a merge records ancestry and suites;
OBS-010: a record change runs `openspec validate --strict`; OBS-011: one log entry per lane
event; LPC-008: T17 (c) proves on huginn (an owned GPU box, PVW-010 host 1); LPC-020: an
unknown box-queue row is added to LOOP.md. Loop process: each is a one-line implication. -/
def FRC_008_ok (killed restarted : Nat) : Bool := restarted = killed
def FRC_010_ok (done committed : Bool) : Bool := !done || committed
def FRC_015_ok (ancestor suitesPass recorded : Bool) : Bool := !recorded || (ancestor && suitesPass)
def OBS_010_ok (changed validated : Bool) : Bool := !changed || validated
def OBS_011_ok (events entries : Nat) : Bool := entries = events
def LPC_008_ok (proverHost : Int) : Bool := proverHost = 1
def LPC_020_ok (unknownRow added : Bool) : Bool := !unknownRow || added
theorem loop_consistent : FRC_008_ok 2 2 && FRC_010_ok true true && FRC_015_ok true true true &&
    OBS_010_ok true true && OBS_011_ok 3 3 && LPC_008_ok 1 && LPC_020_ok true true = true := by decide

/-- LPC-008 agrees with the oracle: the final call proved on host 1 is accepted by `doProve`. -/
theorem LPC_008_with_PVW_010 (s : St) (_hf : s.final = true) :
    (doProve s 1).saved = true := by
  simp [doProve]

end NetworkOps
