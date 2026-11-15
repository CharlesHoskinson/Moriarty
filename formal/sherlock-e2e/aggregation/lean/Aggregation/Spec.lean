/-!
# Aggregation: N1, N2, N3, MERGE-WORK, the GP hooks, continuation

Lean oracle of the Aggregation bounded context (slice B, `bus/ddd-B.md` section 1).
Specs: `sherlock-e2e-node-aggregation` (NAG-001 to NAG-011), `sherlock-e2e-gp-hooks`
(GPH-001 to GPH-008); reused NODE, SHP, CLM, CONT, FREG, FVR (modular-leaf), GP-001 to
GP-004 (vm-consensus), LPC-001 to LPC-003 (completion).

Abstraction: digests are natural numbers. The oracle follows one run. The accumulator is
the SEG that the merged children prove so far (`segPre`, `segPost`, `segLo`, `segHi`).
The `c*` fields are the candidate child (or the candidate input of a seal, a consume, an
`L` check, a resume): they are inputs that the model sets before each operation; the
oracle reads them and leaves them unchanged.

An operation is one natural number `op = code * 16 + arg` (`arg < 16`):

| code | operation | arg |
|---|---|---|
| 0 | `merge`: N1 or N2 merges the candidate child (a LEAF or a NODE SEG) | 0 |
| 1 | `mergeWork k`: a MERGE-WORK node adds `k` owner steps over the candidate child | k |
| 2 | `keygenWork`: the key generator makes the MERGE-WORK key | 0 |
| 3 | `seal`: SEAL of the accumulated SEG | 0 |
| 4 | `consume c`: the run consumes the claim of the candidate (GP-2) | claim 0..15 |
| 5 | `sealWithDeps`: SEAL-WITH-DEPS against the candidate's `DL.chain_d` | 0 |
| 6 | `lCheck`: `L` opens the sealed head against the candidate outcome word | 0 |
| 7 | `foreignUnit`: N3 takes the candidate foreign-verifier unit (GP-3) | 0 |
| 8 | `suspend`: the run suspends at a checkpoint | 0 |
| 9 | `resume`: the run resumes from the checkpoint | 0 |
-/

namespace Aggregation

/-- Child kinds of the key selector (NODE-004, NODE-009). -/
def kLeaf : Nat := 0
def kNode : Nat := 1
def kForeign : Nat := 2

/-- Run status of the candidate (CONT-003): 0 Running, 1 Halted, 2 Rejected, 3 Exhausted. -/
def stRunning : Nat := 0

/-- Bits of the required AIRs (NAG-002): StmtAir, ProofShapeAir, PublicValuesAir. -/
def allAirs : Nat := 7
/-- MERGE-WORK column bound (NAG-005, SHP-008, LPC-002). -/
def mergeWorkColumns : Nat := 5400
/-- Node arity bound (NODE-007). -/
def maxArity : Nat := 4
/-- `keys_d` rows: 7 node rows 0..6 (SHP-009, UROWS) and 2 foreign rows 7, 8 (GPH-003). -/
def uRows : Nat := 7
def foreignRows : Nat := 2
/-- Heights stay below 2^64 (NODE-002, SHP-003). -/
def hBound : Nat := 2 ^ 64

structure St where
  -- run inputs
  runProg : Nat        -- `prog_d` of the run (NAG-003)
  keyRoot : Nat        -- `key_root` of the run (NAG-003, NAG-004)
  regRoot : Nat        -- parent `reg_root` (NODE-008, NAG-011)
  sMax : Nat           -- `S_MAX` (NODE-002)
  mwColumns : Nat      -- columns of the MERGE-WORK key (NAG-005)
  nodeArity : Nat      -- node arity (NODE-007)
  -- accumulator
  segOn : Bool
  segPre : Nat
  segPost : Nat
  segLo : Nat
  segHi : Nat
  chipSets : Nat       -- bit c: a leaf of chip set c merged (NAG-004)
  leaves : Nat
  works : Nat
  depsD : Nat          -- `deps_d`; 0 = `EMPTY_DEPS` (CLM-004)
  sealed : Bool
  claimSteps : Nat     -- `n_steps` of the sealed claim (NODE-003)
  outcome : Nat        -- outcome word proved by the leaves, fixed at the seal
  lAccepted : Bool     -- `L` accepted the node root (NAG-008)
  mwKeyOk : Bool       -- the MERGE-WORK key exists (NAG-005)
  foreignUnits : Nat   -- FV units accepted under N3 (GPH-003)
  suspended : Bool     -- (GPH-006, CONT-002)
  ckPost : Nat
  ckHi : Nat
  -- candidate (input) fields
  cKind : Nat
  cProg : Nat
  cKeyRoot : Nat
  cPre : Nat
  cPost : Nat
  cHPre : Nat
  cSteps : Nat
  cAirs : Nat
  cRegRoot : Nat
  cChipSet : Nat
  cForged : Nat        -- 0 honest; 1 MW-F01, 2 AFK-F7, 3 GP2-F4.2, 4 AFK-F9(b), 5 GP2-C2.1
  cKeyRow : Nat
  cStatus : Nat
  cOutcome : Nat
  cChain : Nat         -- `DL.chain_d` of a SEAL-WITH-DEPS
  cManifestOk : Bool   -- checkpoint manifest equals the key manifest (CONT-004)
  deriving DecidableEq, Repr

inductive Refusal where
  | suspendedRun      -- GPH-006, CONT-002 (no proving while suspended)
  | sealedRun         -- NODE-003, CONT-003 (a sealed run takes no more children)
  | airMissing        -- NAG-002
  | keySelector       -- NODE-004, NODE-009
  | progDMismatch     -- NAG-003, LFP-007, NODE-001
  | keyRootMismatch   -- NAG-003, NODE-004
  | linkBroken        -- NAG-009, NODE-001, SHP-003
  | range             -- NODE-002, SHP-003
  | regRootMismatch   -- NODE-008
  | forgedWitness     -- NAG-006, NAG-007
  | keyRow            -- SHP-009, GPH-003
  | noWorkKey         -- NODE-015, NAG-005
  | noChild           -- SHP-003, NODE-003
  | columns           -- NAG-005, SHP-008, LPC-002
  | arity             -- NODE-007
  | notFromStart      -- NODE-003 (h_lo == 0)
  | notTerminal       -- CONT-003, NODE-003
  | depsNonempty      -- CLM-004 (plain SEAL)
  | depsMismatch      -- CLM-004 (SEAL-WITH-DEPS)
  | notAClaim         -- CLM-002, CLM-007
  | notSealed         -- NODE-003 (`L` opens the sealed head)
  | outcomeMismatch   -- NAG-008
  | notSuspended      -- CONT-002
  | manifestMismatch  -- CONT-004
  | outsideCapability
  deriving DecidableEq, Repr

def Refusal.code : Refusal → String
  | .suspendedRun => "suspended"
  | .sealedRun => "sealed"
  | .airMissing => "air-missing"
  | .keySelector => "key-selector"
  | .progDMismatch => "prog-d-mismatch"
  | .keyRootMismatch => "key-root-mismatch"
  | .linkBroken => "link-broken"
  | .range => "range"
  | .regRootMismatch => "reg-root-mismatch"
  | .forgedWitness => "forged-witness"
  | .keyRow => "key-row"
  | .noWorkKey => "no-work-key"
  | .noChild => "no-child"
  | .columns => "columns"
  | .arity => "arity"
  | .notFromStart => "not-from-start"
  | .notTerminal => "not-terminal"
  | .depsNonempty => "deps-nonempty"
  | .depsMismatch => "deps-mismatch"
  | .notAClaim => "not-a-claim"
  | .notSealed => "not-sealed"
  | .outcomeMismatch => "outcome-mismatch"
  | .notSuspended => "not-suspended"
  | .manifestMismatch => "manifest-mismatch"
  | .outsideCapability => "outside-capability"

inductive Op where
  | merge
  | mergeWork (k : Nat)
  | keygenWork
  | seal
  | consume (c : Nat)
  | sealWithDeps
  | lCheck
  | foreignUnit
  | suspend
  | resume
  | bad
  deriving DecidableEq, Repr

def decodeOp (op : Nat) : Op :=
  let a := op % 16
  match op / 16 with
  | 0 => if a = 0 then .merge else .bad
  | 1 => .mergeWork a
  | 2 => if a = 0 then .keygenWork else .bad
  | 3 => if a = 0 then .seal else .bad
  | 4 => .consume a
  | 5 => if a = 0 then .sealWithDeps else .bad
  | 6 => if a = 0 then .lCheck else .bad
  | 7 => if a = 0 then .foreignUnit else .bad
  | 8 => if a = 0 then .suspend else .bad
  | 9 => if a = 0 then .resume else .bad
  | _ => .bad

/-- `[r]` when `c` holds. -/
def rf (c : Prop) [Decidable c] (r : Refusal) : List Refusal := if c then [r] else []

/-- State domain. It carries: `L` accepts only a sealed root (NODE-003, NAG-008); a sealed
    SEG starts at height 0 and its `n_steps` is its `h_hi` (NODE-003); the MERGE-WORK key
    exists only within its bounds (NAG-005, NODE-007); `h_lo <= h_hi` (NODE-001); a
    suspended run is not sealed (CONT-003). -/
def Valid (s : St) : Prop :=
  (s.lAccepted = true → s.sealed = true) ∧
  (s.sealed = true → s.segOn = true ∧ s.segLo = 0 ∧ s.claimSteps = s.segHi) ∧
  (s.mwKeyOk = true → s.mwColumns ≤ mergeWorkColumns ∧ s.nodeArity ≤ maxArity) ∧
  s.segLo ≤ s.segHi ∧
  (s.suspended = true → s.sealed = false)

instance (s : St) : Decidable (Valid s) := by unfold Valid; infer_instance

def validState (s : St) : Bool := decide (Valid s)

/-- Guards of a merge (N1, N2). -/
def mergeRefusals (s : St) : List Refusal :=
  rf (s.suspended = true) .suspendedRun ++                                     -- GPH-006
  rf (s.sealed = true) .sealedRun ++                                           -- CONT-003
  rf (s.cAirs ≠ allAirs) .airMissing ++                                        -- NAG-002
  rf (s.cKind ≠ kLeaf ∧ s.cKind ≠ kNode) .keySelector ++                       -- NODE-009
  rf (s.cProg ≠ s.runProg) .progDMismatch ++                                   -- NAG-003, LFP-007
  rf (s.cKeyRoot ≠ s.keyRoot) .keyRootMismatch ++                              -- NAG-003, NODE-004
  rf (s.segOn = true ∧ (s.cPre ≠ s.segPost ∨ s.cHPre ≠ s.segHi)) .linkBroken ++  -- NAG-009, NODE-001
  rf ((s.cKind = kLeaf ∧ (s.cSteps = 0 ∨ s.sMax < s.cSteps)) ∨
      hBound ≤ s.cHPre + s.cSteps) .range ++                                   -- NODE-002
  rf ((s.cKind = kLeaf ∧ s.cRegRoot ≠ 0) ∨
      (s.cKind = kNode ∧ s.cRegRoot ≠ s.regRoot)) .regRootMismatch ++          -- NODE-008
  rf (s.cKind = kNode ∧ uRows ≤ s.cKeyRow) .keyRow ++                          -- SHP-009
  rf (s.cForged ≠ 0) .forgedWitness                                            -- NAG-006, NAG-007

/-- Guards of a MERGE-WORK node with `k` owner steps (SHP-003, NAG-005, NAG-011). -/
def workRefusals (s : St) (k : Nat) : List Refusal :=
  rf (s.suspended = true) .suspendedRun ++
  rf (s.sealed = true) .sealedRun ++
  rf (s.mwKeyOk = false) .noWorkKey ++                                         -- NODE-015
  rf (s.segOn = false) .noChild ++                                             -- SHP-003, NAG-006 (AFK-F9 b)
  rf (s.segOn = true ∧ (s.cPre ≠ s.segPost ∨ s.cHPre ≠ s.segHi)) .linkBroken ++  -- SHP-003
  rf (k = 0 ∨ hBound ≤ s.segHi + k) .range ++                                  -- SHP-003
  rf (s.cForged ≠ 0) .forgedWitness                                            -- NAG-006, NAG-007

/-- Guards of SEAL and SEAL-WITH-DEPS (NODE-003, CONT-003, CLM-004). -/
def sealRefusals (s : St) (withDeps : Bool) : List Refusal :=
  rf (s.suspended = true) .suspendedRun ++
  rf (s.sealed = true) .sealedRun ++
  rf (s.segOn = false) .noChild ++
  rf (s.segLo ≠ 0) .notFromStart ++                                            -- NODE-003
  rf (s.cStatus = stRunning) .notTerminal ++                                   -- CONT-003
  (if withDeps then rf (s.cChain ≠ s.depsD) .depsMismatch                      -- CLM-004
   else rf (s.depsD ≠ 0) .depsNonempty)                                        -- CLM-004

def stepRefusals (s : St) (op : Nat) : List Refusal :=
  if validState s = false then [.outsideCapability] else
  match decodeOp op with
  | .merge => mergeRefusals s
  | .mergeWork k => workRefusals s k
  | .keygenWork =>                                                             -- NAG-005, NODE-007
    rf (mergeWorkColumns < s.mwColumns) .columns ++ rf (maxArity < s.nodeArity) .arity
  | .seal => sealRefusals s false
  | .consume _ =>                                                              -- CLM-002, CONT-003
    rf (s.sealed = true) .sealedRun ++ rf (s.cStatus = stRunning) .notTerminal ++
    rf (s.cKind = kNode) .notAClaim
  | .sealWithDeps => sealRefusals s true
  | .lCheck =>                                                                 -- NODE-003, NAG-008
    rf (s.sealed = false) .notSealed ++ rf (s.cOutcome ≠ s.outcome) .outcomeMismatch
  | .foreignUnit =>                                                            -- GPH-003, SHP-009
    rf (s.cKind ≠ kForeign) .keySelector ++
    rf (s.cKeyRow < uRows ∨ uRows + foreignRows ≤ s.cKeyRow) .keyRow ++
    rf (s.cForged ≠ 0) .forgedWitness
  | .suspend =>                                                                -- GPH-006
    rf (s.suspended = true) .suspendedRun ++ rf (s.sealed = true) .sealedRun ++
    rf (s.segOn = false) .noChild
  | .resume =>                                                                 -- CONT-002, CONT-004
    rf (s.suspended = false) .notSuspended ++ rf (s.cManifestOk = false) .manifestMismatch
  | .bad => [.outsideCapability]

def step (s : St) (op : Nat) : St :=
  match decodeOp op with
  | .merge =>
    -- NODE-001: MERGE outputs (prog_d, L.pre_d, R.post_d, L.h_lo, R.h_hi); a LEAF child is
    -- the SEG (prog_d, pre_d, post_d, h_pre, h_pre + steps). NAG-004: chip sets accumulate.
    { s with segOn := true,
             segPre := if s.segOn then s.segPre else s.cPre,
             segLo := if s.segOn then s.segLo else s.cHPre,
             segPost := s.cPost, segHi := s.cHPre + s.cSteps,
             chipSets := s.chipSets ||| 2 ^ s.cChipSet, leaves := s.leaves + 1 }
  | .mergeWork k =>
    -- SHP-003: output height W.h_pre + k, W.pre_d = the right-most child's post_d.
    { s with segPost := s.cPost, segHi := s.segHi + k, works := s.works + 1 }
  | .keygenWork => { s with mwKeyOk := true }                                    -- NAG-005
  | .seal =>
    -- NODE-003: claim_d binds prog_d, input_d, outcome_d; n_steps = h_hi.
    { s with sealed := true, claimSteps := s.segHi, outcome := s.cOutcome }
  | .consume c => { s with depsD := s.depsD * 16 + c + 1 }                       -- CLM-004
  | .sealWithDeps =>                                                             -- CLM-004, GPH-002
    { s with sealed := true, claimSteps := s.segHi, outcome := s.cOutcome }
  | .lCheck => { s with lAccepted := true }                                      -- NAG-008
  | .foreignUnit => { s with foreignUnits := s.foreignUnits + 1 }                -- GPH-003
  | .suspend => { s with suspended := true, ckPost := s.segPost, ckHi := s.segHi } -- GPH-006
  | .resume => { s with suspended := false }                                     -- CONT-002
  | .bad => s

def apply (s : St) (op : Nat) : St := if stepRefusals s op = [] then step s op else s

def run (s : St) : List Nat → St
  | [] => s
  | op :: ops => run (apply s op) ops

def runRefusals (s : St) (_ops : List Nat) : List Refusal :=
  if validState s then [] else [.outsideCapability]

/-- A run before its first node: inputs set, empty accumulator, candidate fields zero. -/
def init (runProg keyRoot regRoot sMax mwColumns nodeArity : Nat) : St :=
  { runProg, keyRoot, regRoot, sMax, mwColumns, nodeArity,
    segOn := false, segPre := 0, segPost := 0, segLo := 0, segHi := 0, chipSets := 0,
    leaves := 0, works := 0, depsD := 0, sealed := false, claimSteps := 0, outcome := 0,
    lAccepted := false, mwKeyOk := false, foreignUnits := 0, suspended := false,
    ckPost := 0, ckHi := 0, cKind := 0, cProg := 0, cKeyRoot := 0, cPre := 0, cPost := 0,
    cHPre := 0, cSteps := 0, cAirs := 0, cRegRoot := 0, cChipSet := 0, cForged := 0,
    cKeyRow := 0, cStatus := 0, cOutcome := 0, cChain := 0, cManifestOk := false }

/-! ## Theorems -/

theorem rf_nil {c : Prop} {d : Decidable c} {r : Refusal} : @rf c d r = [] ↔ ¬ c := by
  unfold rf; split <;> simp_all

theorem valid_of_nil {s : St} {op : Nat} (h : stepRefusals s op = []) : validState s = true := by
  unfold stepRefusals at h
  cases hv : validState s <;> simp_all

/-- NAG-002, NAG-003, LFP-007, NAG-009, NODE-001, NODE-002, NODE-008, NAG-006, NAG-007:
    a merged child has every required AIR, the run's `prog_d` and `key_root`, links to the
    child before it by height and state digest, has a ranged step count, the right
    `reg_root`, and is no forged witness. -/
theorem merge_guard (s : St) (h : stepRefusals s 0 = []) :
    s.cAirs = allAirs ∧ s.cProg = s.runProg ∧ s.cKeyRoot = s.keyRoot ∧
    (s.segOn = true → s.cPre = s.segPost ∧ s.cHPre = s.segHi) ∧
    s.cHPre + s.cSteps < hBound ∧ (s.cKind = kLeaf → 1 ≤ s.cSteps ∧ s.cSteps ≤ s.sMax ∧ s.cRegRoot = 0) ∧
    (s.cKind = kNode → s.cRegRoot = s.regRoot) ∧ s.cForged = 0 ∧ s.sealed = false := by
  have hv := valid_of_nil h
  simp only [stepRefusals, hv, decodeOp, mergeRefusals] at h
  simp [rf_nil] at h
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_⟩
  · exact Classical.byContradiction fun hc => by simp_all
  · exact Classical.byContradiction fun hc => by simp_all
  · exact Classical.byContradiction fun hc => by simp_all
  · intro ho; refine ⟨Classical.byContradiction fun hc => ?_, Classical.byContradiction fun hc => ?_⟩ <;> simp_all
  · exact Classical.byContradiction fun hc => by simp_all <;> omega
  · intro hl; refine ⟨?_, ?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;> simp_all <;> omega
  · intro hn; exact Classical.byContradiction fun hc => by simp_all [kLeaf, kNode]
  · exact Classical.byContradiction fun hc => by simp_all
  · exact Classical.byContradiction fun hc => by simp_all

/-- NAG-005, NODE-015, SHP-003, NAG-011: a MERGE-WORK node needs the MERGE-WORK key, at
    least one linked prior child and `k >= 1` steps below `2^64`. -/
theorem work_guard (s : St) (k : Nat) (hk : k < 16) (h : stepRefusals s (1 * 16 + k) = []) :
    s.mwKeyOk = true ∧ s.mwColumns ≤ mergeWorkColumns ∧ s.segOn = true ∧
    s.cPre = s.segPost ∧ s.cHPre = s.segHi ∧ 1 ≤ k ∧ s.segHi + k < hBound ∧ s.cForged = 0 := by
  have hv := valid_of_nil h
  have hd : (1 * 16 + k) / 16 = 1 := by omega
  have hm : (1 * 16 + k) % 16 = k := by omega
  have hV : Valid s := by simpa [validState] using hv
  simp only [stepRefusals, hv, decodeOp, hd, hm, workRefusals] at h
  simp [rf_nil] at h
  have hw : s.mwKeyOk = true := Classical.byContradiction fun hc => by simp_all
  have hon : s.segOn = true := Classical.byContradiction fun hc => by simp_all
  refine ⟨hw, (hV.2.2.1 hw).1, hon, ?_, ?_, ?_, ?_, ?_⟩ <;>
    refine Classical.byContradiction fun hc => ?_ <;> simp_all <;> omega

/-- NODE-003, CONT-003, CLM-004: SEAL needs a SEG from height 0, a terminal outcome and an
    empty dependency chain. -/
theorem seal_guard (s : St) (h : stepRefusals s (3 * 16) = []) :
    s.segOn = true ∧ s.segLo = 0 ∧ s.cStatus ≠ stRunning ∧ s.depsD = 0 := by
  have hv := valid_of_nil h
  simp only [stepRefusals, hv, decodeOp, sealRefusals] at h
  simp [rf_nil] at h
  refine ⟨?_, ?_, ?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;> simp_all

/-- CLM-004: SEAL-WITH-DEPS needs `DL.chain_d` equal to the opened `deps_d`. -/
theorem sealWithDeps_guard (s : St) (h : stepRefusals s (5 * 16) = []) :
    s.segLo = 0 ∧ s.cStatus ≠ stRunning ∧ s.cChain = s.depsD := by
  have hv := valid_of_nil h
  simp only [stepRefusals, hv, decodeOp, sealRefusals] at h
  simp [rf_nil] at h
  refine ⟨?_, ?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;> simp_all

/-- NAG-008: `L` accepts the node root only when the outcome word equals the outcome that the
    leaves proved (fixed at the seal). -/
theorem lCheck_guard (s : St) (h : stepRefusals s (6 * 16) = []) :
    s.sealed = true ∧ s.cOutcome = s.outcome := by
  have hv := valid_of_nil h
  simp only [stepRefusals, hv, decodeOp] at h
  simp [rf_nil] at h
  refine ⟨?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;> simp_all

/-- GPH-003, SHP-009: N3 accepts a foreign-verifier unit only as a FOREIGN child at a foreign
    row of `keys_d`. -/
theorem foreign_guard (s : St) (h : stepRefusals s (7 * 16) = []) :
    s.cKind = kForeign ∧ uRows ≤ s.cKeyRow ∧ s.cKeyRow < uRows + foreignRows ∧ s.cForged = 0 := by
  have hv := valid_of_nil h
  simp only [stepRefusals, hv, decodeOp] at h
  simp [rf_nil] at h
  refine ⟨?_, ?_, ?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;> simp_all <;> omega

/-- NAG-005, NODE-007, LPC-002, SHP-008: the key generator refuses a MERGE-WORK key over
    5,400 columns (and a node over arity 4). -/
theorem keygenWork_guard (s : St) (h : stepRefusals s (2 * 16) = []) :
    s.mwColumns ≤ mergeWorkColumns ∧ s.nodeArity ≤ maxArity := by
  have hv := valid_of_nil h
  simp only [stepRefusals, hv, decodeOp] at h
  simp [rf_nil] at h
  omega

/-- CONT-004: a resume from a checkpoint whose manifest differs from the key manifest is
    refused. -/
theorem resume_guard (s : St) (h : stepRefusals s (9 * 16) = []) :
    s.suspended = true ∧ s.cManifestOk = true := by
  have hv := valid_of_nil h
  simp only [stepRefusals, hv, decodeOp] at h
  simp [rf_nil] at h
  refine ⟨?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;> simp_all

/-- The statement of the run: what `claim_pub` and the node root depend on. -/
def stmtView (s : St) : List Nat :=
  [s.runProg, s.keyRoot, s.segPre, s.segPost, s.segLo, s.segHi, s.depsD, s.claimSteps, s.outcome]

/-- GPH-006, CONT-002: a suspend and a resume execute no machine event: the run statement
    after suspend and resume equals the statement without the suspension. -/
theorem suspend_resume_same_statement (s : St) :
    stmtView (step (step s (8 * 16)) (9 * 16)) = stmtView s ∧
    (step (step s (8 * 16)) (9 * 16)).suspended = false := by
  simp [step, decodeOp, stmtView]

/-! ### SEG algebra (NODE-001): the fold order does not change the statement -/

structure Seg where
  prog : Nat
  pre : Nat
  post : Nat
  lo : Nat
  hi : Nat
  deriving DecidableEq, Repr

/-- NODE-001: MERGE needs equal `prog_d`, `L.post_d == R.pre_d`, `L.h_hi == R.h_lo`. -/
def linked (l r : Seg) : Prop := l.prog = r.prog ∧ l.post = r.pre ∧ l.hi = r.lo

def mergeSeg (l r : Seg) : Seg := { prog := l.prog, pre := l.pre, post := r.post, lo := l.lo, hi := r.hi }

/-- CONT-001, GPH-005, GP-004, SHP-001: merging is associative, and the links of a chain hold
    for both bracketings; so any fold order or tree shape over one chain of SEGs gives one
    statement. -/
theorem mergeSeg_assoc (a b c : Seg) : mergeSeg (mergeSeg a b) c = mergeSeg a (mergeSeg b c) := rfl

theorem linked_assoc (a b c : Seg) (hab : linked a b) (hbc : linked b c) :
    linked (mergeSeg a b) c ∧ linked a (mergeSeg b c) := by
  unfold linked mergeSeg at *
  simp_all

/-- NAG-009 (scenario "Leaves in the wrong order"): two linked children of nonzero length
    cannot be merged in the swapped order unless their digests coincide. -/
theorem swapped_not_linked (a b : Seg) (hab : linked a b) (ha : a.lo < a.hi) (hb : b.lo < b.hi) :
    ¬ linked b a := by
  unfold linked at *
  omega

/-- The step domain is preserved. -/
theorem apply_valid (s : St) (op : Nat) (h : Valid s) : Valid (apply s op) := by
  unfold apply
  split
  · rename_i hr
    have hv : validState s = true := by simp [validState, h]
    simp only [stepRefusals, hv, Bool.true_eq_false, ite_false] at hr
    obtain ⟨h1, h2, h3, h4, h5⟩ := h
    unfold step
    split
    · -- merge
      rename_i hop
      simp only [hop, mergeRefusals, List.append_eq_nil_iff, rf_nil] at hr
      have hs : s.sealed = false := by cases hq : s.sealed <;> simp_all
      have hl : s.lAccepted = false := by cases hq : s.lAccepted <;> simp_all
      refine ⟨by simp [hl], by simp [hs], h3, ?_, by simp [hs]⟩
      cases ho : s.segOn
      · simp
      · have : s.cHPre = s.segHi := Classical.byContradiction fun hc => by simp_all
        simp; omega
    · -- mergeWork
      rename_i k hop
      simp only [hop, workRefusals, List.append_eq_nil_iff, rf_nil] at hr
      have hs : s.sealed = false := by cases hq : s.sealed <;> simp_all
      have hl : s.lAccepted = false := by cases hq : s.lAccepted <;> simp_all
      refine ⟨by simp [hl], by simp [hs], h3, by simp; omega, by simp [hs]⟩
    · -- keygenWork
      rename_i hop
      simp only [hop, List.append_eq_nil_iff, rf_nil] at hr
      refine ⟨h1, h2, fun _ => ⟨by simp; omega, by simp; omega⟩, h4, h5⟩
    · -- seal
      rename_i hop
      simp only [hop, sealRefusals, List.append_eq_nil_iff, rf_nil] at hr
      have hu : s.suspended = false := by cases hq : s.suspended <;> simp_all
      have ho : s.segOn = true := by cases hq : s.segOn <;> simp_all
      have h0 : s.segLo = 0 := Classical.byContradiction fun hc => by simp_all
      refine ⟨fun _ => rfl, fun _ => ⟨ho, h0, rfl⟩, h3, h4, by simp [hu]⟩
    · -- consume
      rename_i c hop
      exact ⟨h1, h2, h3, h4, h5⟩
    · -- sealWithDeps
      rename_i hop
      simp only [hop, sealRefusals, List.append_eq_nil_iff, rf_nil] at hr
      have hu : s.suspended = false := by cases hq : s.suspended <;> simp_all
      have ho : s.segOn = true := by cases hq : s.segOn <;> simp_all
      have h0 : s.segLo = 0 := Classical.byContradiction fun hc => by simp_all
      refine ⟨fun _ => rfl, fun _ => ⟨ho, h0, rfl⟩, h3, h4, by simp [hu]⟩
    · -- lCheck
      rename_i hop
      simp only [hop, List.append_eq_nil_iff, rf_nil] at hr
      have hs : s.sealed = true := by cases hq : s.sealed <;> simp_all
      exact ⟨fun _ => hs, h2, h3, h4, h5⟩
    · exact ⟨h1, h2, h3, h4, h5⟩
    · -- suspend
      rename_i hop
      simp only [hop, List.append_eq_nil_iff, rf_nil] at hr
      have hs : s.sealed = false := by cases hq : s.sealed <;> simp_all
      exact ⟨h1, h2, h3, h4, fun _ => hs⟩
    · exact ⟨h1, h2, h3, h4, fun hc => by simp at hc⟩
    · exact ⟨h1, h2, h3, h4, h5⟩
  · exact h

theorem init_valid (a b c d e f : Nat) : Valid (init a b c d e f) := by
  simp [Valid, init]

theorem run_valid (s : St) (ops : List Nat) (h : Valid s) : Valid (run s ops) := by
  induction ops generalizing s with
  | nil => exact h
  | cons op ops ih => exact ih _ (apply_valid s op h)

/-- NAG-008, NODE-003, NAG-005: in every reachable state, an `L`-accepted root is sealed, the
    sealed SEG starts at height 0 with `n_steps = h_hi`, and a MERGE-WORK key exists only
    within 5,400 columns. -/
theorem reachable_root (a b c d e f : Nat) (ops : List Nat) :
    let t := run (init a b c d e f) ops
    (t.lAccepted = true → t.sealed = true ∧ t.segLo = 0 ∧ t.claimSteps = t.segHi) ∧
    (t.mwKeyOk = true → t.mwColumns ≤ mergeWorkColumns) := by
  have hv := run_valid _ ops (init_valid a b c d e f)
  refine ⟨fun hl => ?_, fun hw => (hv.2.2.1 hw).1⟩
  have hs := hv.1 hl
  exact ⟨hs, (hv.2.1 hs).2.1, (hv.2.1 hs).2.2⟩

end Aggregation
