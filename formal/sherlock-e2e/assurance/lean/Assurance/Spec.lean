/-!
# Assurance: zero knowledge at the proving layer and the T12 security gate

Lean oracle of the Assurance bounded context (slice B, `bus/ddd-B.md` section 1).
Specs: `sherlock-e2e-zero-knowledge` (ZKE-001 to ZKE-007), `sherlock-e2e-stark-security`
(SSG-001 to SSG-007); reused ZK-012 to ZK-018, SQG-014 to SQG-016, LPC-004, LPC-016
(completion) and ZK, SQG, DG, FPR (processor-final).

Abstraction: secret values are draw counters. A fresh CSPRNG draw is the next counter value,
so "fresh" means "never equal to an earlier draw". Hosts are numbers 0..15; the owned-box
roster is a bit mask. Gate checks G1, G2, G3, G4, G5, FP-8 are bits 0..5 of `gateBits`.

An operation is one natural number `op = code * 16 + arg` (`arg < 16`):

| code | operation | arg |
|---|---|---|
| 0 | `prepareDeployment`: draw `r_P`, write `deploy.secret` | - |
| 1 | `prepareRun`: draw the run salt, write `run.secret` | - |
| 2 | `loseSecret`: `deploy.secret` is absent (a new box, a deleted file) | - |
| 3 | `wrapProve d`: the wrap prover proves the run's statement | 0 CPU, 1 GPU |
| 4 | `submit`: the settlement runner submits the proof to Preview | - |
| 5 | `transfer h`: a secret, witness, trace or STARK proof moves to host `h` | host |
| 6 | `useSlot b`: a request to `wrapd serve` or the STARK worker uses blinding slot `b` | slot |
| 7 | `gateCheck c`: gate check `c` passes (0 G1 .. 4 G5, 5 FP-8) | 0..5 |
| 8 | `writeRecord`: the security gate writes its gate record | - |
| 9 | `newKeys`: a new key set is in use (the rekey completes) | - |
| 10 | `writeRunRecord`: the prover writes a run record | - |
| 11 | `landChange`: a circuit change lands after openvm c4bd312 | - |
| 12 | `fileVerdict`: a Carbon Quint lane files a lead verdict on a change | - |
| 13 | `markDone`: the loop counts a row DONE for T15 | - |
| 14 | `fixDefect`: a CONFIRMED counterexample row gets its fix commit | - |
| 15 | `verifyStep`: an UNVERIFIED RUNBOOK step is verified | - |
| 16 | `attempt`: a NOT ATTEMPTED counterexample row is attempted | - |
-/

namespace Assurance

structure St where
  /-- the binary was built with the cargo feature `test-seed` (ZK-014, ZKE-001, ZKE-002) -/
  testSeedBuild : Bool
  /-- `SHERLOCK_TEST_SEED`, `GW_DET`, `GW_CHIP_SEED` or `GW_SEED` is set (ZK-014) -/
  seedEnv : Bool
  /-- the chain driver reaches a code path that sets `r_P` from a constant (ZKE-001) -/
  constRpPath : Bool
  /-- `r_P` draws of this deployment (ZK-012: once) -/
  rpDraws : Nat
  /-- `deploy.secret` is present on the box (ZKE-005) -/
  deploySecret : Bool
  /-- run salt draws; each run draws one (ZK-013) -/
  saltDraws : Nat
  /-- the current run's salt (a draw number; 0 = no run) -/
  salt : Nat
  /-- blinder draws of the wrap prover (ZK-003, ZK-015, ZK-017) -/
  blinderDraws : Nat
  /-- the blinder of each wrap proof, newest first (ZKE-007) -/
  blinders : List Nat
  /-- the backend of each wrap proof, newest first: 0 CPU, 1 GPU (ZKE-007) -/
  devices : List Nat
  /-- proofs submitted to Preview (ZKE-002) -/
  submitted : Nat
  /-- owned-box roster: bit h set = host h is owned by the user (ZK-001, ZK-018, ZKE-004) -/
  roster : Nat
  /-- admitted transfers (ZKE-004) -/
  transfers : Nat
  /-- blinding slots used: bit b set = slot b used (ZK-016) -/
  usedSlots : Nat
  /-- gate checks passed on the key set in use: bits 0..4 = G1..G5, bit 5 = FP-8 (SSG-006) -/
  gateBits : Nat
  /-- RUNBOOK steps still UNVERIFIED (SQG-016) -/
  unverified : Nat
  /-- counterexample rows CONFIRMED with no fix commit (SSG-003) -/
  confirmedNoFix : Nat
  /-- counterexample rows NOT ATTEMPTED (SSG-004) -/
  notAttempted : Nat
  /-- the G5 mapping names the frozen parameter sets of the key set (SSG-006 scenario) -/
  g5Current : Bool
  /-- a gate record exists for the key set in use (SSG-006) -/
  gateRecord : Bool
  /-- key manifest sha256 in the gate record (SSG-006) -/
  recordManifest : Nat
  /-- rows listed open in the gate record (SSG-004) -/
  recordOpen : Nat
  /-- id of the key set in use (its manifest) -/
  manifestD : Nat
  /-- key set status: false `prototype`, true `qualified` (SSG-006, UKS-007) -/
  qualified : Bool
  /-- run records written; security field of the last one: true `qualified` (SSG-007) -/
  runRecords : Nat
  lastRunQualified : Bool
  /-- circuit changes landed, rows in the change list, lead verdicts, rows counted DONE
      (SSG-005, LPC-004, LPC-016) -/
  changes : Nat
  changeRows : Nat
  verdicts : Nat
  doneRows : Nat
  deriving DecidableEq, Repr

/-- G1..G5 and FP-8 all passed. -/
def allChecks : Nat := 63
def gateCheckNames : List String := ["G1", "G2", "G3", "G4", "G5", "FP-8"]

inductive Refusal where
  | seededWithoutFeature   -- ZK-014, ZKE-001: exit 4, no proof
  | alreadyDeployed        -- ZK-012: r_P is drawn once per deployment
  | notDeployed            -- ZK-013: a run needs a deployment
  | secretMissing          -- ZKE-005
  | noRun                  -- ZK-013: a proof needs a prepared run
  | noProof                -- ZKE-002: nothing to submit
  | testSeedBinary         -- ZKE-002
  | hostOutsideRoster      -- ZKE-004, ZK-018
  | slotReused             -- ZK-016
  | recordExists           -- SSG-006: checks run before the record of a key set
  | checksIncomplete       -- SSG-006, SQG-015
  | stepUnverified         -- SQG-016
  | confirmedDefect        -- SSG-003
  | g5OldParams            -- SSG-006 (scenario "G5 names old parameters")
  | noOpenChange           -- LPC-004
  | noLeadVerdict          -- LPC-016
  | nothingOpen            -- counters at 0
  | outsideCapability
  deriving DecidableEq, Repr

def Refusal.code : Refusal → String
  | .seededWithoutFeature => "seeded-without-feature"
  | .alreadyDeployed => "already-deployed"
  | .notDeployed => "not-deployed"
  | .secretMissing => "secret-missing"
  | .noRun => "no-run"
  | .noProof => "no-proof"
  | .testSeedBinary => "test-seed-binary"
  | .hostOutsideRoster => "host-outside-roster"
  | .slotReused => "slot-reused"
  | .recordExists => "record-exists"
  | .checksIncomplete => "checks-incomplete"
  | .stepUnverified => "step-unverified"
  | .confirmedDefect => "confirmed-defect"
  | .g5OldParams => "g5-old-params"
  | .noOpenChange => "no-open-change"
  | .noLeadVerdict => "no-lead-verdict"
  | .nothingOpen => "nothing-open"
  | .outsideCapability => "outside-capability"

inductive Op where
  | prepareDeployment | prepareRun | loseSecret
  | wrapProve (d : Nat) | submit | transfer (h : Nat) | useSlot (b : Nat)
  | gateCheck (c : Nat) | writeRecord | newKeys | writeRunRecord
  | landChange | fileVerdict | markDone | fixDefect | verifyStep | attempt
  | bad
  deriving DecidableEq, Repr

def decodeOp (op : Nat) : Op :=
  let a := op % 16
  let z (o : Op) : Op := if a = 0 then o else .bad
  match op / 16 with
  | 0 => z .prepareDeployment
  | 1 => z .prepareRun
  | 2 => z .loseSecret
  | 3 => if a < 2 then .wrapProve a else .bad
  | 4 => z .submit
  | 5 => .transfer a
  | 6 => .useSlot a
  | 7 => if a < 6 then .gateCheck a else .bad
  | 8 => z .writeRecord
  | 9 => z .newKeys
  | 10 => z .writeRunRecord
  | 11 => z .landChange
  | 12 => z .fileVerdict
  | 13 => z .markDone
  | 14 => z .fixDefect
  | 15 => z .verifyStep
  | 16 => z .attempt
  | _ => .bad

/-- Bit `i` of `n`. -/
def bit (n i : Nat) : Bool := (n / 2 ^ i) % 2 == 1

/-- Every element of `l` is at most `n`. -/
def allLe (l : List Nat) (n : Nat) : Prop := ∀ x ∈ l, x ≤ n

/-- State domain. Carries ZK-012 (one `r_P` draw per deployment), ZK-013 (one salt per run),
    ZK-003/ZKE-007 (distinct blinders in a production build), SSG-006 (a gate record only with
    every check, naming the key set in use), UKS-007/SSG-007 (status follows the record) and
    SSG-005 (one row per change). -/
def Valid (s : St) : Prop :=
  s.rpDraws ≤ 1 ∧ (s.deploySecret = true → s.rpDraws = 1) ∧ s.salt ≤ s.saltDraws ∧
  (s.testSeedBuild = false → s.blinders.Nodup ∧ allLe s.blinders s.blinderDraws) ∧
  s.blinders.length = s.devices.length ∧
  (s.qualified = true → s.gateRecord = true) ∧
  (s.gateRecord = true → s.gateBits = allChecks ∧ s.recordManifest = s.manifestD) ∧
  (s.lastRunQualified = true → s.qualified = true) ∧
  s.verdicts ≤ s.changes ∧ s.changeRows = s.changes

instance (s : St) : Decidable (allLe s.blinders s.blinderDraws) := by
  unfold allLe; infer_instance
instance (s : St) : Decidable (Valid s) := by unfold Valid; infer_instance

def validState (s : St) : Bool := decide (Valid s)

/-- ZK-014, ZKE-001: a seed source in a build without `test-seed` stops with exit 4. -/
def seedGuard (s : St) : List Refusal :=
  if s.testSeedBuild = false ∧ (s.seedEnv = true ∨ s.constRpPath = true) then
    [.seededWithoutFeature] else []

def stepRefusals (s : St) (op : Nat) : List Refusal :=
  if validState s = false then [.outsideCapability] else
  match decodeOp op with
  | .prepareDeployment =>                                       -- ZK-012, ZKE-001
    seedGuard s ++ (if s.rpDraws = 0 then [] else [.alreadyDeployed])
  | .prepareRun =>                                              -- ZK-013, ZKE-005, ZK-014
    seedGuard s ++ (if s.rpDraws = 1 then [] else [.notDeployed]) ++
    (if s.deploySecret then [] else [.secretMissing])
  | .loseSecret => []                                           -- ZKE-005
  | .wrapProve _ =>                                             -- ZK-003, ZK-014, ZK-015, ZK-017
    seedGuard s ++ (if s.salt = 0 then [.noRun] else [])
  | .submit =>                                                  -- ZKE-002
    (if s.blinders = [] then [.noProof] else []) ++
    (if s.testSeedBuild then [.testSeedBinary] else [])
  | .transfer h => if bit s.roster h then [] else [.hostOutsideRoster]   -- ZKE-004, ZK-018
  | .useSlot b => if bit s.usedSlots b then [.slotReused] else []         -- ZK-016
  | .gateCheck _ => if s.gateRecord then [.recordExists] else []          -- SSG-006
  | .writeRecord =>                                             -- SSG-006, SQG-016, SSG-003
    (if s.gateRecord then [.recordExists] else []) ++
    (if s.gateBits = allChecks then [] else [.checksIncomplete]) ++
    (if s.unverified = 0 then [] else [.stepUnverified]) ++
    (if s.confirmedNoFix = 0 then [] else [.confirmedDefect]) ++
    (if s.g5Current then [] else [.g5OldParams])
  | .newKeys => []                                              -- SQG-015
  | .writeRunRecord => []                                       -- SSG-007
  | .landChange => []                                           -- SSG-005
  | .fileVerdict => if s.verdicts < s.changes then [] else [.noOpenChange]  -- LPC-004
  | .markDone => if s.verdicts = s.changes then [] else [.noLeadVerdict]    -- LPC-016
  | .fixDefect => if s.confirmedNoFix = 0 then [.nothingOpen] else []      -- SSG-003
  | .verifyStep => if s.unverified = 0 then [.nothingOpen] else []         -- SQG-016
  | .attempt => if s.notAttempted = 0 then [.nothingOpen] else []          -- SSG-004
  | .bad => [.outsideCapability]

def step (s : St) (op : Nat) : St :=
  match decodeOp op with
  | .prepareDeployment =>
    -- ZK-012: `r_P` drawn once from the CSPRNG, stored in `deploy.secret`
    { s with rpDraws := s.rpDraws + 1, deploySecret := true }
  | .prepareRun =>
    -- ZK-013: a fresh salt per run in `run.secret`; ZKE-005: no new `r_P`
    { s with saltDraws := s.saltDraws + 1, salt := s.saltDraws + 1 }
  | .loseSecret => { s with deploySecret := false }                         -- ZKE-005
  | .wrapProve d =>
    -- ZK-003, ZK-015, ZK-017: a fresh blinder per proof (OsRng on CPU and GPU);
    -- a seeded (fixed) blinder only in a `test-seed` build with a seed variable (ZK-003)
    if s.testSeedBuild ∧ s.seedEnv then
      { s with blinders := 0 :: s.blinders, devices := d :: s.devices }
    else
      { s with blinderDraws := s.blinderDraws + 1, blinders := (s.blinderDraws + 1) :: s.blinders,
               devices := d :: s.devices }
  | .submit => { s with submitted := s.submitted + 1 }                      -- ZKE-002
  | .transfer _ => { s with transfers := s.transfers + 1 }                  -- ZKE-004
  | .useSlot b => { s with usedSlots := s.usedSlots + 2 ^ b }               -- ZK-016 (bit b was 0)
  | .gateCheck c =>                                                         -- SSG-006
    { s with gateBits := if bit s.gateBits c then s.gateBits else s.gateBits + 2 ^ c }
  | .writeRecord =>
    -- SSG-006: record with the key manifest; SSG-004: NOT ATTEMPTED rows listed open;
    -- UKS-007: the key set becomes `qualified`
    { s with gateRecord := true, recordManifest := s.manifestD, recordOpen := s.notAttempted,
             qualified := true }
  | .newKeys =>
    -- SQG-015: the gate reruns on the new keys; the new set is `prototype`
    { s with manifestD := s.manifestD + 1, gateBits := 0, gateRecord := false, qualified := false,
             lastRunQualified := false }
  | .writeRunRecord =>                                                      -- SSG-007
    { s with runRecords := s.runRecords + 1, lastRunQualified := s.qualified }
  | .landChange => { s with changes := s.changes + 1, changeRows := s.changeRows + 1 }  -- SSG-005
  | .fileVerdict => { s with verdicts := s.verdicts + 1 }                   -- LPC-004
  | .markDone => { s with doneRows := s.doneRows + 1 }                      -- LPC-016
  | .fixDefect => { s with confirmedNoFix := s.confirmedNoFix - 1 }         -- SSG-003
  | .verifyStep => { s with unverified := s.unverified - 1 }                -- SQG-016
  | .attempt => { s with notAttempted := s.notAttempted - 1 }               -- SSG-004
  | .bad => s

def apply (s : St) (op : Nat) : St := if stepRefusals s op = [] then step s op else s

def run (s : St) : List Nat → St
  | [] => s
  | op :: ops => run (apply s op) ops

def runRefusals (s : St) (_ops : List Nat) : List Refusal :=
  if validState s then [] else [.outsideCapability]

/-- A new box set before any deployment: the build kind, the seed environment, the roster,
    the gate inputs. -/
def init (testSeedBuild seedEnv constRpPath : Bool) (roster unverified confirmedNoFix
    notAttempted : Nat) (g5Current : Bool) : St :=
  { testSeedBuild, seedEnv, constRpPath, rpDraws := 0, deploySecret := false, saltDraws := 0,
    salt := 0, blinderDraws := 0, blinders := [], devices := [], submitted := 0, roster,
    transfers := 0, usedSlots := 0, gateBits := 0, unverified, confirmedNoFix, notAttempted,
    g5Current, gateRecord := false, recordManifest := 0, recordOpen := 0, manifestD := 1,
    qualified := false, runRecords := 0, lastRunQualified := false, changes := 0,
    changeRows := 0, verdicts := 0, doneRows := 0 }

/-! ## Theorems -/

private theorem guard_of {α} {c : Prop} [Decidable c] {x : α} {l : List α}
    (h : (if c then [] else [x]) ++ l = []) : c := by
  refine Classical.byContradiction fun hc => ?_
  simp [hc] at h

private theorem valid_of_nil {s : St} {op : Nat} (h : stepRefusals s op = []) :
    validState s = true := by
  cases hv : validState s
  · simp [stepRefusals, hv] at h
  · rfl

/-- ZK-014, ZKE-001: a build without `test-seed` with a seed variable or a constant `r_P` path
    prepares nothing and proves nothing (the refusal is exit 4, no proof written). -/
theorem seeded_refused (s : St) (hb : s.testSeedBuild = false)
    (hs : s.seedEnv = true ∨ s.constRpPath = true) (op : Nat)
    (hop : decodeOp op = .prepareDeployment ∨ decodeOp op = .prepareRun ∨
      ∃ d, decodeOp op = .wrapProve d) :
    stepRefusals s op ≠ [] := by
  intro h
  have hv := valid_of_nil h
  have hg : seedGuard s = [.seededWithoutFeature] := by simp [seedGuard, hb, hs]
  unfold stepRefusals at h
  simp only [hv, Bool.true_eq_false, ite_false] at h
  rcases hop with h1 | h1 | ⟨d, h1⟩ <;> simp [h1, hg] at h

/-- ZK-012: `r_P` is drawn once per deployment: a second preparation is refused. -/
theorem rp_once (s : St) (h : stepRefusals s 0 = []) : s.rpDraws = 0 := by
  have hv := valid_of_nil h
  unfold stepRefusals at h
  simp only [hv, Bool.true_eq_false, ite_false, decodeOp] at h
  refine Classical.byContradiction fun hc => ?_
  simp [hc] at h

/-- ZKE-005: a run starts only with `deploy.secret` present, and it draws no new `r_P`. -/
theorem run_needs_secret (s : St) (h : stepRefusals s (1 * 16) = []) :
    s.deploySecret = true ∧ s.rpDraws = 1 ∧ (step s (1 * 16)).rpDraws = s.rpDraws := by
  have hv := valid_of_nil h
  unfold stepRefusals at h
  simp only [hv, Bool.true_eq_false, ite_false] at h
  simp only [decodeOp] at h
  have h2 : s.rpDraws = 1 := Classical.byContradiction fun hc => by simp [hc] at h
  have h3 : s.deploySecret = true := by
    refine Classical.byContradiction fun hc => ?_; simp [hc] at h
  exact ⟨h3, h2, by simp [step, decodeOp]⟩

/-- ZK-013: each run takes a salt that no earlier run had. -/
theorem salt_fresh (s : St) (hv : Valid s) :
    (step s (1 * 16)).salt > s.salt ∧ (step s (1 * 16)).salt = s.saltDraws + 1 := by
  obtain ⟨_, _, h3, _⟩ := hv
  simp [step, decodeOp]
  omega

/-- ZKE-002: a proof of a `test-seed` binary is never submitted. -/
theorem testseed_not_submitted (s : St) (hb : s.testSeedBuild = true) :
    stepRefusals s (4 * 16) ≠ [] := by
  intro h
  have hv := valid_of_nil h
  unfold stepRefusals at h
  simp [hv, decodeOp, hb] at h

/-- ZKE-004, ZK-018: a transfer is admitted only to a host of the roster. -/
theorem transfer_in_roster (s : St) (h : Nat) (hh : h < 16)
    (hr : stepRefusals s (5 * 16 + h) = []) : bit s.roster h = true := by
  have hv := valid_of_nil hr
  have hd : (5 * 16 + h) / 16 = 5 := by omega
  have hm : (5 * 16 + h) % 16 = h := by omega
  unfold stepRefusals at hr
  simp only [hv, Bool.true_eq_false, ite_false, decodeOp, hd, hm] at hr
  refine Classical.byContradiction fun hc => ?_
  simp [hc] at hr

/-- ZK-016: a request that reuses a blinding slot is refused. -/
theorem slot_reuse_refused (s : St) (b : Nat) (hb : b < 16) (hu : bit s.usedSlots b = true) :
    stepRefusals s (6 * 16 + b) ≠ [] := by
  intro h
  have hv := valid_of_nil h
  have hd : (6 * 16 + b) / 16 = 6 := by omega
  have hm : (6 * 16 + b) % 16 = b := by omega
  unfold stepRefusals at h
  simp [hv, decodeOp, hd, hm, hu] at h

/-- SSG-006, SQG-016, SSG-003: a gate record is written only when G1..G5 and FP-8 passed, no
    RUNBOOK step is UNVERIFIED, no CONFIRMED row lacks a fix and G5 names the current sets. -/
theorem record_guard (s : St) (h : stepRefusals s (8 * 16) = []) :
    s.gateBits = allChecks ∧ s.unverified = 0 ∧ s.confirmedNoFix = 0 ∧ s.g5Current = true := by
  have hv := valid_of_nil h
  unfold stepRefusals at h
  simp only [hv, Bool.true_eq_false, ite_false, decodeOp] at h
  refine ⟨?_, ?_, ?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;> simp_all

/-- SSG-004: the gate record lists every NOT ATTEMPTED row as open. -/
theorem record_lists_open (s : St) : (step s (8 * 16)).recordOpen = s.notAttempted ∧
    (step s (8 * 16)).recordManifest = s.manifestD := by
  simp [step, decodeOp]

/-- LPC-016: a row counts DONE only when every landed change has a lead verdict. -/
theorem done_needs_verdict (s : St) (h : stepRefusals s (13 * 16) = []) :
    s.verdicts = s.changes := by
  have hv := valid_of_nil h
  unfold stepRefusals at h
  simp only [hv, Bool.true_eq_false, ite_false, decodeOp] at h
  refine Classical.byContradiction fun hc => ?_
  simp [hc] at h

/-- The domain is preserved by every applied operation. -/
theorem apply_valid (s : St) (op : Nat) (h : Valid s) : Valid (apply s op) := by
  unfold apply
  split
  · rename_i hr
    have hv : validState s = true := by simp [validState, h]
    simp only [stepRefusals, hv, Bool.true_eq_false, ite_false] at hr
    unfold Valid at h ⊢
    obtain ⟨h1, h2, h3, h4, h5, h6, h7, h8, h9, h10⟩ := h
    unfold step
    split
    · -- prepareDeployment
      rename_i hop
      simp only [hop] at hr
      have h0 : s.rpDraws = 0 := Classical.byContradiction fun hc => by simp [hc] at hr
      exact ⟨by simp; omega, fun _ => by simp; omega, h3, h4, h5, h6, h7, h8, h9, h10⟩
    · exact ⟨h1, h2, by simp, h4, h5, h6, h7, h8, h9, h10⟩
    · exact ⟨h1, fun hc => by simp at hc, h3, h4, h5, h6, h7, h8, h9, h10⟩
    · -- wrapProve
      split
      next hs =>
        refine ⟨h1, h2, h3, ?_, by simp [h5], h6, h7, h8, h9, h10⟩
        intro hb; exact absurd hs.1 (by simp only at hb; simp [hb])
      next =>
        refine ⟨h1, h2, h3, ?_, by simp [h5], h6, h7, h8, h9, h10⟩
        intro hb
        dsimp only
        obtain ⟨hn, hl⟩ := h4 hb
        refine ⟨?_, ?_⟩
        · refine List.nodup_cons.mpr ⟨?_, hn⟩
          intro hm; have := hl _ hm; omega
        · intro x hx
          simp only [List.mem_cons] at hx
          rcases hx with hx | hx
          · omega
          · have := hl x hx; omega
    · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, h9, h10⟩
    · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, h9, h10⟩
    · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, h9, h10⟩
    · -- gateCheck: refused once a record exists
      rename_i c hop
      simp only [hop] at hr
      have hg : s.gateRecord = false := by
        cases hc : s.gateRecord
        · rfl
        · simp [hc] at hr
      exact ⟨h1, h2, h3, h4, h5, fun hq => by simp_all, fun hq => by simp_all, h8, h9, h10⟩
    · -- writeRecord
      rename_i hop
      simp only [hop] at hr
      have hb : s.gateBits = allChecks :=
        Classical.byContradiction fun hc => by simp [hc] at hr
      exact ⟨h1, h2, h3, h4, h5, fun _ => rfl, fun _ => ⟨hb, rfl⟩, fun _ => rfl, h9, h10⟩
    · exact ⟨h1, h2, h3, h4, h5, fun hc => by simp at hc, fun hc => by simp at hc,
        fun hc => by simp at hc, h9, h10⟩
    · exact ⟨h1, h2, h3, h4, h5, h6, h7, fun hq => hq, h9, h10⟩
    · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, by simp; omega, by simp; omega⟩
    · rename_i hop
      simp only [hop] at hr
      have : s.verdicts < s.changes := Classical.byContradiction fun hc => by simp [hc] at hr
      exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, by simp; omega, h10⟩
    · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, h9, h10⟩
    · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, h9, h10⟩
    · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, h9, h10⟩
    · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, h9, h10⟩
    · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, h9, h10⟩
  · exact h

theorem init_valid (a b c : Bool) (r u f n : Nat) (g : Bool) : Valid (init a b c r u f n g) := by
  simp [Valid, init, allLe]

theorem run_valid (s : St) (ops : List Nat) (h : Valid s) : Valid (run s ops) := by
  induction ops generalizing s with
  | nil => exact h
  | cons op ops ih => exact ih _ (apply_valid s op h)

/-- Reachable-state invariants (from `init`):
    ZK-012 at most one `r_P` draw; ZKE-007/ZK-015 the wrap proofs of a production build have
    pairwise distinct blinders (on either backend); SSG-006/UKS-007 a `qualified` key set has a
    gate record with every check and its own manifest; SSG-007 a run record reads `qualified`
    only on a qualified key set; SSG-005 one change-list row per landed change. -/
theorem reachable (a b c : Bool) (r u f n : Nat) (g : Bool) (ops : List Nat) :
    let t := run (init a b c r u f n g) ops
    t.rpDraws ≤ 1 ∧ (a = false → t.blinders.Nodup) ∧
    (t.qualified = true → t.gateBits = allChecks ∧ t.recordManifest = t.manifestD) ∧
    (t.lastRunQualified = true → t.qualified = true) ∧ t.changeRows = t.changes := by
  have hv := run_valid _ ops (init_valid a b c r u f n g)
  have hb : (run (init a b c r u f n g) ops).testSeedBuild = a := by
    have key : ∀ (s : St) (l : List Nat), (run s l).testSeedBuild = s.testSeedBuild := by
      intro s l
      induction l generalizing s with
      | nil => rfl
      | cons op ops ih =>
        simp only [run]; rw [ih]; unfold apply; split
        · unfold step; split <;> (try split) <;> rfl
        · rfl
    rw [key]; rfl
  obtain ⟨h1, _, _, h4, _, h6, h7, h8, _, h10⟩ := hv
  refine ⟨h1, fun ha => (h4 (hb.trans ha)).1, fun hq => h7 (h6 hq), h8, h10⟩

end Assurance
