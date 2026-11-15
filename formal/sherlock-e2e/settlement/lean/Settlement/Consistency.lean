import Settlement.Spec

/-!
# SPEC-CHECKED requirements of Settlement, and the findings shown in Lean

The requirements below have no state machine in this context: they fix data forms (the
statement words, the digests, the claims tree), circuit-side kind rules of the node
(CLM, FREG: the AIRs belong to slice B's Aggregation context) or report contents. Each is a
Boolean predicate `<ID>_ok` over one design record; `consistent` shows that the record at the
spec's values satisfies all of them together with the modeled rules of `Spec.lean`, so the
requirements do not contradict each other or the oracle. Where they do, the theorem is named
`finding_*` and the finding is in `research/sherlock-e2e-spec-20261115/bus/formal-findings.md`.
-/

namespace Settlement.Consistency

structure Design where
  publics : List String          -- SLC-006, STL-001: StatementPublics
  acceptArgs : List String       -- LDG-002: acceptRun call data
  outcomeTag : String            -- SLC-007
  outcomeHead : String
  inputPreimage : List String    -- SLC-008, ZK-006
  saltWords : Nat
  fuel : Nat
  stackLimit : Nat
  digestTags : List String       -- ZK-011
  rPWords : Nat
  proveChain : List String       -- STL-002
  wrapK : Nat
  suitePrograms : Nat            -- STL-006
  suiteCompiled : Nat
  entries : List String          -- SLC-001, LDG-004
  stateMaps : List String        -- LDG-001, LDG-007
  collectionPublics : List String -- LW-009, LDG-003
  treeDepth : Nat                -- LDG-005, BAT-007
  leafKinds : List String        -- LDG-005, LDG-007
  runLeafTag : String
  foreignLeafTag : String
  consumerWrites : List String   -- CRR-007, LDG-004
  reportCounts : List String     -- BAT-003
  setKinds : List String         -- CLM-001
  domainWords : List String
  consumable : List String       -- CLM-002, CLM-007
  openable : List String         -- CLM-003, LDG-004 (in circuit)
  chainOnly : List String        -- CLM-005
  dlFields : List String         -- CLM-007
  buildOption : String           -- CLM-006
  airHeightLog : Nat             -- CLM-008, FREG-005
  airConstraints : Nat
  airInteractions : Nat
  airs : Nat
  columns : Nat
  refusalTwins : Bool            -- CLM-009
  regEntryFields : Nat           -- FREG-001
  piCountBound : Nat
  fclaimDomain : String          -- FREG-002
  trailingRefused : Bool         -- FREG-003
  accumulators : Nat             -- FREG-004
  readEntryWrites : Nat          -- ELD-005
  grid : Nat                     -- ERT-009
  deriving Repr

def SLC_006_ok (d : Design) : Bool :=
  d.publics == ["c_p", "input_pub", "outcome_pub", "claim_pub"] && d.acceptArgs == d.publics.take 3
def STL_001_ok (d : Design) : Bool := d.publics.length == 4 && SLC_006_ok d
def LDG_002_args_ok (d : Design) : Bool :=
  !(d.acceptArgs.contains "prog_d") && !(d.acceptArgs.contains "n_steps")
/-- SLC-007: `Halted([comm, E_0 .. E_(L_e-1)])`. -/
def outcomeWords (comm : Int) (effects : List Int) : List Int := comm :: effects
def SLC_007_ok (d : Design) : Bool := d.outcomeTag == "Halted" && d.outcomeHead == "comm"
def SLC_008_ok (d : Design) : Bool := d.fuel == 0 && d.stackLimit == 0 && d.saltWords == 8
/-- ZK-006: 8 salt words first in the preimage of `input_d`. -/
def ZK_006_ok (d : Design) : Bool :=
  d.saltWords == 8 && d.inputPreimage == ["salt", "val_d", "fuel_lo", "fuel_hi", "stack_limit"]
/-- ZK-011: three public digests under distinct tags, `r_P` of 8 words. -/
def ZK_011_ok (d : Design) : Bool :=
  d.digestTags == ["SHK_PROGC", "SHK_INPUT", "SHK_OUTCOME"] && d.digestTags.Nodup && d.rPWords == 8
def STL_002_ok (d : Design) : Bool :=
  d.proveChain == ["app", "leaf", "N1", "N2", "N3", "L", "L_Fr", "wrap"] && d.wrapK ≤ 21
def STL_006_ok (d : Design) : Bool := d.suiteCompiled == 33 && d.suiteCompiled ≤ d.suitePrograms
def SLC_001_ok (d : Design) : Bool :=
  d.entries == ["acceptRun", "acceptCollection", "rootAccepted", "runAccepted"]
def LDG_004_ok (d : Design) : Bool :=
  !(d.entries.contains "openClaim") && !(d.entries.contains "descend") && d.treeDepth == 20
    && d.consumerWrites == ["own-nullifier"]
def LDG_001_ok (d : Design) : Bool := d.stateMaps == ["runs", "claim_roots", "registry_root"]
def LDG_007_ok (d : Design) : Bool :=
  !(d.stateMaps.contains "foreign") && d.runLeafTag != d.foreignLeafTag
    && d.leafKinds == ["run", "foreign"]
def LDG_005_ok (d : Design) : Bool :=
  d.treeDepth == 20 && !(d.leafKinds.contains "ADM") && !(d.leafKinds.contains "VIEW")
def LW_009_ok (d : Design) : Bool :=
  d.collectionPublics == ["reg_pub", "claims_root"] && !(d.collectionPublics.contains "shape_d")
    && !(d.collectionPublics.contains "claim_count")
def CRR_007_ok (d : Design) : Bool := d.consumerWrites == ["own-nullifier"]
def BAT_003_ok (d : Design) : Bool :=
  ["tx", "bytes", "wrap-proofs"].all fun k =>
    ["per-request", "per-epoch", "per-day"].all fun p => d.reportCounts.contains (k ++ "/" ++ p)
def CLM_001_ok (d : Design) : Bool :=
  d.setKinds == ["LEAF", "DEP", "JOIN", "FOREIGN", "ADM", "VIEW"] && (d.domainWords ++ ["SHK_RUN", "SHK_DEPS"]).Nodup
    && d.domainWords.length == 6
def CLM_002_ok (d : Design) : Bool := !(d.consumable.contains "SEG") && !(d.consumable.contains "JOIN")
def CLM_003_ok (d : Design) : Bool := d.openable == ["JOIN", "DEP"]
def CLM_004_ok (_ : Design) : Bool :=
  -- deps_d starts at EMPTY_DEPS and each consume folds one claim, in order: two different
  -- orders give two different chains (order and repetition are kept)
  [1, 2].foldl (fun acc c => acc * 31 + c) 7 != [2, 1].foldl (fun acc c => acc * 31 + c) 7
def CLM_005_ok (d : Design) : Bool := d.chainOnly == ["Rejected", "Exhausted"]
def CLM_006_ok (d : Design) : Bool := d.buildOption == "AD-20 (a)"
def CLM_007_ok (d : Design) : Bool :=
  d.dlFields == ["chain_d", "cover_root", "in_link", "prem_root"] && d.consumable == ["LEAF", "DEP", "FOREIGN"]
def SG_ok (d : Design) : Bool :=
  d.airHeightLog ≤ 19 && d.airConstraints ≤ 1000 && d.airInteractions ≤ 200 && d.airs ≤ 64 && d.columns ≤ 3000
def CLM_008_ok (d : Design) : Bool := SG_ok d
def FREG_005_ok (d : Design) : Bool := SG_ok d
def CLM_009_ok (d : Design) : Bool := d.refusalTwins
def FREG_001_ok (d : Design) : Bool := d.regEntryFields == 9 && d.piCountBound == 2 ^ 32
def FREG_002_ok (d : Design) : Bool := d.fclaimDomain != "SHK_RUN"
def FREG_003_ok (d : Design) : Bool := d.trailingRefused
def FREG_004_ok (d : Design) : Bool := d.accumulators == 0
def ELD_005_ok (d : Design) : Bool := d.readEntryWrites == 0
def ERT_009_ok (d : Design) (t : Nat) : Bool := t % d.grid == 0

def allOk (d : Design) : Bool :=
  SLC_006_ok d && STL_001_ok d && LDG_002_args_ok d && SLC_007_ok d && SLC_008_ok d && ZK_006_ok d
    && ZK_011_ok d && STL_002_ok d && STL_006_ok d && SLC_001_ok d && LDG_004_ok d && LDG_001_ok d
    && LDG_007_ok d && LDG_005_ok d && LW_009_ok d && CRR_007_ok d && BAT_003_ok d && CLM_001_ok d
    && CLM_002_ok d && CLM_003_ok d && CLM_004_ok d && CLM_005_ok d && CLM_006_ok d && CLM_007_ok d
    && CLM_008_ok d && FREG_005_ok d && CLM_009_ok d && FREG_001_ok d && FREG_002_ok d && FREG_003_ok d
    && FREG_004_ok d && ELD_005_ok d

/-- The spec values (t20-batched texts for LDG and LW-009; STL-006 33 of 36 programs,
`/home/hoskinson/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md:45`). -/
def specValues : Design :=
  { publics := ["c_p", "input_pub", "outcome_pub", "claim_pub"],
    acceptArgs := ["c_p", "input_pub", "outcome_pub"], outcomeTag := "Halted", outcomeHead := "comm",
    inputPreimage := ["salt", "val_d", "fuel_lo", "fuel_hi", "stack_limit"], saltWords := 8, fuel := 0,
    stackLimit := 0, digestTags := ["SHK_PROGC", "SHK_INPUT", "SHK_OUTCOME"], rPWords := 8,
    proveChain := ["app", "leaf", "N1", "N2", "N3", "L", "L_Fr", "wrap"], wrapK := 21,
    suitePrograms := 36, suiteCompiled := 33,
    entries := ["acceptRun", "acceptCollection", "rootAccepted", "runAccepted"],
    stateMaps := ["runs", "claim_roots", "registry_root"],
    collectionPublics := ["reg_pub", "claims_root"], treeDepth := 20, leafKinds := ["run", "foreign"],
    runLeafTag := "claim_pub", foreignLeafTag := "SHK_CLAIM_FGN", consumerWrites := ["own-nullifier"],
    reportCounts := ["tx/per-request", "tx/per-epoch", "tx/per-day", "bytes/per-request", "bytes/per-epoch",
      "bytes/per-day", "wrap-proofs/per-request", "wrap-proofs/per-epoch", "wrap-proofs/per-day"],
    setKinds := ["LEAF", "DEP", "JOIN", "FOREIGN", "ADM", "VIEW"],
    domainWords := ["D_LEAF", "D_DEP", "D_JOIN", "D_FOREIGN", "D_ADM", "D_VIEW"],
    consumable := ["LEAF", "DEP", "FOREIGN"], openable := ["JOIN", "DEP"],
    chainOnly := ["Rejected", "Exhausted"], dlFields := ["chain_d", "cover_root", "in_link", "prem_root"],
    buildOption := "AD-20 (a)", airHeightLog := 19, airConstraints := 1000, airInteractions := 200,
    airs := 64, columns := 3000, refusalTwins := true, regEntryFields := 9, piCountBound := 2 ^ 32,
    fclaimDomain := "SHK_CLAIM", trailingRefused := true, accumulators := 0, readEntryWrites := 0, grid := 1 }

theorem consistent : allOk specValues = true := by decide

/-- SLC-007: a changed last effect word changes the outcome words. -/
theorem SLC_007_changed_effect : outcomeWords 3 [1, 2] ≠ outcomeWords 3 [1, 5] := by decide

/-- SLC-001 and the deployed instance of `Spec.lean` agree: 4 entries. -/
theorem SLC_001_matches_oracle : specValues.entries.length = (run 0 0 1 [1 * opBase + 7]).entries.toNat := by
  decide

/-- LDG-008: one registry write per registry epoch with a change, none without. -/
def regWrites (changes : Nat) : Nat := if changes = 0 then 0 else 1
theorem LDG_008_once (n : Nat) : regWrites n ≤ 1 ∧ (n = 0 → regWrites n = 0) := by
  unfold regWrites; split <;> simp_all

/-! ## Findings shown in Lean -/

/-- ERT-004: `active(e)` holds at block time `t` only in `[start, end_eff)`. -/
def active (start endEff t : Int) : Bool := decide (start ≤ t) && decide (t < endEff)

/-- FAL-SETTLEMENT-3 (BAT-011 against ERT-004): BAT-011 admits a run whose proving time plus
inclusion time ends exactly at `end_eff` (it refuses only "after"), and ERT-004 does not
accept at `end_eff`. Counterexample: start 0, end_eff 1,000, now 143, proving 757 s, inclusion
100 s: the oracle starts the run, and the era is not active when it lands. -/
theorem finding_bat011_boundary :
    (run 0 100 5 [7 * opBase + 1000, 4 * opBase + 143, 8 * opBase + 757]).runsStarted = 1 ∧
    active 0 1000 (143 + 757 + 100) = false := by decide

/-- FAL-SETTLEMENT-3 (BAT-010 against ERT-004): the epoch closes at `end_eff` minus the
collection proof time and the inclusion time, so a root submitted at the close lands at
`end_eff`, where ERT-004 accepts nothing; BAT-010's scenario says "lands before `end_eff`".
Counterexample: end_eff 2,000, budgets 200 + 100: close at 1,700, landing 2,000. -/
theorem finding_bat010_boundary :
    stepRefusals (run 200 100 5 [7 * opBase + 2000, 5 * opBase, 4 * opBase + 1700]) (6 * opBase + 51) = [] ∧
    active 0 2000 (1700 + 200 + 100) = false := by decide

/-- FAL-LEDGER-VERIFICATION-1 (MID-002 against SLC-004): MID-002 says every exported entry
holds the universal wrap key; SLC-004 installs native keys on the two read entries. In
every deployed instance of the oracle the read entries hold a key other than the write key. -/
theorem finding_mid002_read_entries (cb it rr : Int) (ns : List Int) (h : (run cb it rr ns).deployed = true) :
    (run cb it rr ns).keyRoot ≠ (run cb it rr ns).keyRun := by
  have := SLC_001_004_deploy_shape cb it rr ns h
  rw [this.2.2.2.1, this.2.1]; decide

/-- FAL-SETTLEMENT-2 (ANC-012 against BAT-005): the ANC-012 formula gives `K_MAX` = 8 at a
14,400 s TTL (the runner default today, BAT-006 status), BAT-005 caps at 2 at every TTL. -/
theorem finding_anc012_formula : min (14400 / 1755 : Int) 18 = 8 ∧ kMax 14400 = 2 := by decide

/-! ## Era rows consumed here (governance owns the era table) -/

/-- ERT-004 (SPEC-CHECKED here): `active` is the half-open interval `[start, end_eff)`, with
`end_eff = min(end, start(P))` for a pending era `P` (`pend = 0`: none). The era state machine is
the governance context's; settlement only consumes `end_eff`. -/
def endEff (endE pend : Int) : Int := if pend = 0 then endE else min endE pend
def ERT_004_ok : Bool :=
  active 0 (endEff 1000 0) 999 && !active 0 (endEff 1000 0) 1000 &&
  !active 0 (endEff 1000 600) 600 && active 0 (endEff 1000 600) 599
theorem ERT_004_consistent : ERT_004_ok = true := by decide

/-- GOV-006 (SPEC-CHECKED here): `soundness_end(N)` at time `now` sets `end(N) = now` and cancels
an unstarted pending era, so `end_eff = now` and `active` fails from `now` on; settlement sees
only the new `end_eff` (announceEra cell `eraEnd`). -/
def GOV_006_ok (now : Int) : Bool := endEff now 0 == now && !active 0 (endEff now 0) now
theorem GOV_006_consistent : GOV_006_ok 500 = true := by decide

end Settlement.Consistency
