/-!
# Wrap oracle (bounded context Wrap, slice C)

Executable reference semantics of `sherlock-e2e-wrap` (WRP-001 to WRP-012, no
WRP-005: withdrawn) and of the adopted requirements that constrain the wrap
tool: DG-001 to DG-003, LW-003, LW-012, ZK-003, GPW-001, GPW-003, ANC-013,
ANC-016, RKY-003, RKY-004 (spec text only; no code was read).

One wrap tool run for one ledger call goes through these phases:

| phase | meaning |
|---|---|
| 0 | idle: no request |
| 1 | sized: the caller asked for k <= 21 |
| 2 | costed: the cost model rows fit the gate |
| 3 | keyed: keygen ran, the VK fits 50,000 bytes |
| 4 | checked: the key equals the key file of `E6_EXPECT_VK` |
| 5 | decoded: the anchor prologue passed |
| 6 | written: one proof, verified, three output files |
| 7 | stopped: a guard stopped the run (`stop` names it) |

Era-level cells (`eraVk`, `keysDeployed`, `deployed`, `qual`) and the CSPRNG
cells (`draws`, `lastBlinder`, `prevBlinder`) survive `newCall`.
-/

namespace Wrap

/-! ## Constants of the spec -/

/-- DG-001, WRP-002: never k = 22. -/
def kMax : Int := 21
/-- WRP-003, DG-002, RKY-003: 85 % of `2^21 - 7` usable rows, rounded down. -/
def rowGate : Int := 1782573
/-- WRP-004, DG-003, MID-003, RKY-004. -/
def vkMax : Int := 50000
/-- WRP-007, ANC-001, ANC-013: the key constant of the anchor slot. -/
def anchorSlot : Int := 15
/-- LW-003: `L_TX`. -/
def lTx : Int := 1024
/-- LW-003: the wrap has exactly `2 + L_TX` public inputs. -/
def publicInputs : Int := 2 + lTx
/-- WRP-012: one bit for each of LW-003, LW-004, LW-005, LW-006, LW-014. -/
def qualAll : Int := 31
/-- Operation codes are `code * opBase + arg`. -/
def opBase : Int := 1000000000

/-! ## Stop and refusal codes recorded in the state -/

/-- `stop` (terminal, phase 7) and `lastRefusal` (non-terminal, deploy and GPU). -/
def stopNone : Int := 0
def stopK22 : Int := 1        -- WRP-002 `never k = 22`
def stopRows : Int := 2       -- WRP-003
def stopVk : Int := 3         -- WRP-004
def stopKeyDiff : Int := 4    -- WRP-008 `the wrap key differs`
def stopDecode : Int := 5     -- WRP-007, ANC-013, ANC-016
def stopVerify : Int := 6     -- WRP-009 (the library verifier refused)
def refQual : Int := 8        -- WRP-012 (record incomplete, no deploy)
def refSecondKey : Int := 9   -- WRP-010 (a second wrap key in one era)
def refGpuKey : Int := 10     -- WRP-011 (GPU key not byte-equal to the CPU key)

structure St where
  phase : Int
  k : Int
  rows : Int
  vkBytes : Int
  vkId : Int
  keygenRan : Bool
  vkFileWritten : Bool
  files : Int
  proofs : Int
  stop : Int
  lastRefusal : Int
  anchorRead : Int
  chunkWrite : Int
  mode : Int
  template : Bool
  testSeed : Bool
  draws : Int
  lastBlinder : Int
  prevBlinder : Int
  qual : Int
  eraVk : Int
  keysDeployed : Int
  deployed : Int
  gpuVk : Int
  deriving Repr, DecidableEq

/-- The start state. `testSeed` is the cargo feature `test-seed` (ZK-003, ZK-014). -/
def init (testSeed : Bool) : St :=
  { phase := 0, k := 0, rows := 0, vkBytes := 0, vkId := 0, keygenRan := false,
    vkFileWritten := false, files := 0, proofs := 0, stop := stopNone, lastRefusal := stopNone,
    anchorRead := 0, chunkWrite := 0, mode := 0, template := false, testSeed,
    draws := 0, lastBlinder := 0, prevBlinder := 0, qual := 0, eraVk := 0,
    keysDeployed := 0, deployed := 0, gpuVk := 0 }

/-! ## Operations -/

inductive Op where
  | askK (k : Int)                    -- 1: the caller asks for circuit size k
  | cost (rows : Int)                 -- 2: the cost model reports rows at k
  | keygen (vkId bytes : Int)         -- 3: keygen; the serialized VK has `bytes`
  | expect (same : Bool)              -- 4: compare with the key file of E6_EXPECT_VK
  | decode (readKey writeKey mode : Int) (template : Bool)  -- 5: anchor prologue
  | prove (accepted : Bool)           -- 6: prove once; the library verifier answer
  | qualify (i : Int)                 -- 7: a pass result for check i of WRP-012
  | deploy                            -- 8: deploy with the written key
  | gpuProve (vkId : Int)             -- 9: a GPU wrap run with key vkId
  | newCall                           -- 10: start the next ledger call
  deriving Repr, DecidableEq

/-- Decode an integer operation; `none` is outside the operation domain. -/
def Op.ofInt (n : Int) : Option Op :=
  let c := n / opBase
  let a := n % opBase
  if n < 0 then none
  else if c = 1 then some (.askK a)
  else if c = 2 then some (.cost a)
  else if c = 3 then (if a / 100000 = 0 then none else some (.keygen (a / 100000) (a % 100000)))
  else if c = 4 then (if a = 0 then some (.expect false) else if a = 1 then some (.expect true) else none)
  else if c = 5 then
    let r := a % 100
    let w := (a / 100) % 100
    let m := (a / 10000) % 10
    let t := a / 100000
    if t = 0 then some (.decode r w m false) else if t = 1 then some (.decode r w m true) else none
  else if c = 6 then (if a = 0 then some (.prove false) else if a = 1 then some (.prove true) else none)
  else if c = 7 then (if 0 ≤ a ∧ a < 5 then some (.qualify a) else none)
  else if c = 8 then (if a = 0 then some .deploy else none)
  else if c = 9 then some (.gpuProve a)
  else if c = 10 then (if a = 0 then some .newCall else none)
  else none

/-- The phase in which each operation is enabled (`qualify` in every phase). -/
def enabled (s : St) : Op → Bool
  | .askK _ => s.phase = 0
  | .cost _ => s.phase = 1
  | .keygen v _ => s.phase = 2 ∧ 1 ≤ v
  | .expect _ => s.phase = 3
  | .decode _ _ _ _ => s.phase = 4
  | .prove _ => s.phase = 5
  | .qualify i => 0 ≤ i ∧ i < 5
  | .deploy => s.phase = 6
  | .gpuProve _ => s.phase = 6
  | .newCall => s.phase = 6 ∨ s.phase = 7

def stopped (s : St) (code : Int) : St := { s with phase := 7, stop := code }

/-- Add bit `i` to the qualification record. -/
def addBit (q i : Int) : Int := if (q / 2 ^ i.toNat) % 2 = 1 then q else q + 2 ^ i.toNat

/-- One operation. A disabled operation leaves the state unchanged (`--step` refuses it). -/
def apply (s : St) (op : Op) : St :=
  if !enabled s op then s else
  match op with
  | .askK k =>
    -- WRP-002, DG-001: above k = 21 the tool stops before keygen
    if kMax < k then stopped { s with k } stopK22
    else { s with phase := 1, k }
  | .cost rows =>
    -- WRP-003, DG-002, RKY-003: more than 1,782,573 rows stops before keygen
    if rowGate < rows then stopped { s with rows } stopRows
    else { s with phase := 2, rows }
  | .keygen vkId bytes =>
    -- WRP-004, DG-003, RKY-004: a VK above 50,000 bytes stops without writing wrap.vk
    if vkMax < bytes then stopped { s with keygenRan := true, vkBytes := bytes, vkId } stopVk
    else { s with phase := 3, keygenRan := true, vkBytes := bytes, vkId }
  | .expect same =>
    -- WRP-008: a key that differs from E6_EXPECT_VK stops before any output file
    if same then { s with phase := 4 } else stopped s stopKeyDiff
  | .decode r w m t =>
    -- WRP-007, ANC-013: lanes 6 and 15 hold the key constant 15
    -- ANC-016: an ordinary call (mode 0) with the anchor-write template is refused
    let s' := { s with anchorRead := r, chunkWrite := w, mode := m, template := t }
    if r = anchorSlot ∧ w = anchorSlot ∧ ¬(m = 0 ∧ t = true) then { s' with phase := 5 }
    else stopped s' stopDecode
  | .prove acc =>
    -- WRP-001: one proof for the call; WRP-006, LW-012, ZK-003: fresh blinders
    -- from the CSPRNG (a draw counter), a fixed seed only under test-seed
    let d := s.draws + 1
    let s' := { s with proofs := s.proofs + 1, draws := d, prevBlinder := s.lastBlinder,
                       lastBlinder := if s.testSeed then 0 else d }
    -- WRP-009: the three files are written when the library verifier accepts
    if acc then { s' with phase := 6, files := 3, vkFileWritten := true }
    else stopped s' stopVerify
  | .qualify i =>
    -- WRP-012: a pass result for LW-003, LW-004, LW-005, LW-006 or LW-014
    if 0 ≤ i ∧ i < 5 then { s with qual := addBit s.qual i } else s
  | .deploy =>
    -- WRP-012: no deploy uses a key whose record is incomplete
    if s.qual ≠ qualAll then { s with lastRefusal := refQual }
    -- WRP-010, MID-002 (narrowed): one wrap key for every deployment of one era
    else if s.eraVk ≠ 0 ∧ s.eraVk ≠ s.vkId then { s with lastRefusal := refSecondKey }
    else { s with lastRefusal := stopNone, deployed := s.deployed + 1,
                  keysDeployed := if s.eraVk = 0 then s.keysDeployed + 1 else s.keysDeployed,
                  eraVk := s.vkId }
  | .gpuProve v =>
    -- WRP-011, GPW-001, GPW-003: the GPU prover uses the CPU wrap key, byte-equal
    if v = s.vkId then { s with gpuVk := v, lastRefusal := stopNone }
    else { s with lastRefusal := refGpuKey }
  | .newCall =>
    { s with phase := 0, k := 0, rows := 0, vkBytes := 0, vkId := 0, keygenRan := false,
             vkFileWritten := false, files := 0, proofs := 0, stop := stopNone,
             lastRefusal := stopNone, anchorRead := 0, chunkWrite := 0, mode := 0,
             template := false, gpuVk := 0 }

/-! ## Refusals (step domain only; every spec response is a state above) -/

inductive Refusal where
  | notAnOperation
  | opNotEnabled
  | outsideStateDomain
  deriving Repr, DecidableEq

def Refusal.code : Refusal → String
  | .notAnOperation => "not-an-operation"
  | .opNotEnabled => "op-not-enabled"
  | .outsideStateDomain => "outside-state-domain"

/-- The state domain: phase and codes in range, counts non-negative. Not reachability. -/
def validState (s : St) : Bool :=
  0 ≤ s.phase && s.phase ≤ 7 && 0 ≤ s.k && 0 ≤ s.rows && 0 ≤ s.vkBytes && 0 ≤ s.vkId
    && 0 ≤ s.files && 0 ≤ s.proofs && 0 ≤ s.stop && s.stop ≤ 6
    && 0 ≤ s.lastRefusal && s.lastRefusal ≤ 10 && 0 ≤ s.draws && 0 ≤ s.qual && s.qual ≤ qualAll
    && 0 ≤ s.eraVk && 0 ≤ s.keysDeployed && 0 ≤ s.deployed && 0 ≤ s.gpuVk

def stepRefusals (s : St) (n : Int) : List Refusal :=
  match Op.ofInt n with
  | none => [.notAnOperation] ++ (if validState s then [] else [.outsideStateDomain])
  | some op => (if enabled s op then [] else [.opNotEnabled])
      ++ (if validState s then [] else [.outsideStateDomain])

/-- `--step`: one integer operation. -/
def stepOp (s : St) (n : Int) : St :=
  match Op.ofInt n with
  | none => s
  | some op => apply s op

def runOps (s : St) : List Int → St
  | [] => s
  | n :: ns => runOps (stepOp s n) ns

/-- Default mode: `testSeed` is 0 or 1; every operation must decode. -/
def refusals (testSeed : Int) (ops : List Int) : List Refusal :=
  (if testSeed = 0 ∨ testSeed = 1 then [] else [.outsideStateDomain])
    ++ (if ops.all (fun n => (Op.ofInt n).isSome) then [] else [.notAnOperation])

def run (testSeed : Int) (ops : List Int) : St := runOps (init (testSeed = 1)) ops

/-! ## Invariants -/

/-- Every wrap invariant, each named by its requirement IDs. -/
structure Inv (s : St) : Prop where
  /-- WRP-002, DG-001, LW-013: keygen never runs above k = 21. -/
  kOk : s.keygenRan = true → s.k ≤ kMax
  /-- WRP-003, DG-002, RKY-003: keygen never runs above the row gate. -/
  rowsOk : s.keygenRan = true → s.rows ≤ rowGate
  /-- WRP-004, DG-003, MID-003, RKY-004: wrap.vk is never written above 50,000 bytes. -/
  vkOk : s.vkFileWritten = true → s.vkBytes ≤ vkMax
  /-- WRP-002, WRP-003: a stop at the size or row guard comes before keygen. -/
  stopBeforeKeygen : (s.stop = stopK22 ∨ s.stop = stopRows) → s.keygenRan = false
  /-- WRP-004, WRP-008: no output file after a VK-size or key-difference stop. -/
  noFileAfterStop : (s.stop = stopVk ∨ s.stop = stopKeyDiff) → s.files = 0 ∧ s.vkFileWritten = false
  /-- WRP-009: output files are exactly the three, only in phase 6. -/
  filesOk : s.files = 0 ∨ (s.files = 3 ∧ s.phase = 6 ∧ s.vkFileWritten = true)
  /-- WRP-008, WRP-009: files are only written after the key check and the decode. -/
  writtenChecked : s.vkFileWritten = true →
    s.keygenRan = true ∧ s.anchorRead = anchorSlot ∧ s.chunkWrite = anchorSlot
  /-- WRP-001: at most one wrap proof for one ledger call. -/
  oneProof : s.proofs ≤ 1
  /-- WRP-001: phase 6 holds exactly one proof. -/
  writtenOneProof : s.phase = 6 → s.proofs = 1
  /-- WRP-007, ANC-013, ANC-016: a decoded call has slot 15 in lanes 6 and 15 and is
  not an ordinary call carrying the anchor-write template. -/
  decodeOk : (s.phase = 5 ∨ s.phase = 6) →
    s.anchorRead = anchorSlot ∧ s.chunkWrite = anchorSlot ∧ ¬(s.mode = 0 ∧ s.template = true)
  /-- WRP-006, LW-012, ZK-003: in a production build each proof's blinder is a fresh draw. -/
  fresh : s.testSeed = false → s.lastBlinder = s.draws ∧ (s.draws = 0 ∨ s.prevBlinder = s.draws - 1)
  drawsNonneg : 0 ≤ s.draws
  /-- WRP-012: the record holds only the five bits. -/
  qualRange : 0 ≤ s.qual ∧ s.qual ≤ qualAll
  /-- WRP-012: a deploy happened only with a complete record. -/
  deployQual : 0 < s.deployed → s.qual = qualAll
  /-- WRP-010: one wrap key for every deployment of one era. -/
  oneKey : s.keysDeployed ≤ 1 ∧ (s.keysDeployed = 0 ↔ s.eraVk = 0) ∧ (0 < s.deployed → s.keysDeployed = 1)
  eraKeyPos : 0 ≤ s.eraVk
  /-- WRP-011, GPW-001, GPW-003: a GPU run uses the CPU key. -/
  gpuKey : s.gpuVk = 0 ∨ s.gpuVk = s.vkId
  keysNonneg : 0 ≤ s.keysDeployed ∧ 0 ≤ s.deployed
  -- Phase facts: what each phase and each stop code implies (the guards above
  -- are proved from these).
  phaseRange : 0 ≤ s.phase ∧ s.phase ≤ 7
  stopIff : s.phase = 7 ↔ s.stop ≠ 0
  stopRange : 0 ≤ s.stop ∧ s.stop ≤ 6
  pK : 1 ≤ s.phase ∧ s.phase ≤ 6 → s.k ≤ kMax
  pRows : 2 ≤ s.phase ∧ s.phase ≤ 6 → s.rows ≤ rowGate
  pNoKeygen : s.phase ≤ 2 → s.keygenRan = false
  pKeyed : 3 ≤ s.phase ∧ s.phase ≤ 6 → s.keygenRan = true ∧ s.vkBytes ≤ vkMax ∧ 1 ≤ s.vkId
  pNoProof : s.phase ≤ 5 → s.proofs = 0
  pNoGpu : s.phase ≠ 6 → s.gpuVk = 0
  pNoFile : s.phase ≤ 5 → s.files = 0 ∧ s.vkFileWritten = false
  sNoFile : s.phase = 7 → s.files = 0 ∧ s.vkFileWritten = false
  sProof : s.phase = 7 → (s.stop ≤ 5 → s.proofs = 0) ∧ (s.stop = 6 → s.proofs = 1)
  sKeyed : s.phase = 7 → 3 ≤ s.stop → s.keygenRan = true ∧ s.k ≤ kMax ∧ s.rows ≤ rowGate
  sNoKeygen : s.phase = 7 → s.stop ≤ 2 → s.keygenRan = false
  sDecoded : s.phase = 7 → s.stop = 6 →
    s.anchorRead = anchorSlot ∧ s.chunkWrite = anchorSlot ∧ ¬(s.mode = 0 ∧ s.template = true)

theorem inv_init (b : Bool) : Inv (init b) := by
  constructor <;> simp [init, stopNone, stopK22, stopRows, stopVk, stopKeyDiff, qualAll, kMax, anchorSlot]

theorem addBit_range (q i : Int) (hq : 0 ≤ q ∧ q ≤ 31) (hi : 0 ≤ i ∧ i < 5) :
    0 ≤ addBit q i ∧ addBit q i ≤ 31 := by
  have : i = 0 ∨ i = 1 ∨ i = 2 ∨ i = 3 ∨ i = 4 := by omega
  unfold addBit
  rcases this with h | h | h | h | h <;> subst h <;> simp <;> split <;> omega

theorem addBit_full (i : Int) (hi : 0 ≤ i ∧ i < 5) : addBit 31 i = 31 := by
  have : i = 0 ∨ i = 1 ∨ i = 2 ∨ i = 3 ∨ i = 4 := by omega
  rcases this with h | h | h | h | h <;> subst h <;> decide

/-! Each operation keeps every invariant. One lemma per operation keeps the goals small. -/

section
variable (s : St) (h : Inv s)
include h

macro "inv_tac" : tactic => `(tactic| (
  cases h
  constructor <;> simp_all [stopNone, stopK22, stopRows, stopVk, stopKeyDiff, stopDecode, stopVerify,
      refQual, refSecondKey, refGpuKey, kMax, rowGate, vkMax, anchorSlot, qualAll] <;> omega))

theorem inv_askK (k : Int) (he : s.phase = 0) : Inv (apply s (.askK k)) := by
  by_cases hc : kMax < k <;> simp [apply, enabled, he, stopped, hc] <;> inv_tac
theorem inv_cost (r : Int) (he : s.phase = 1) : Inv (apply s (.cost r)) := by
  by_cases hc : rowGate < r <;> simp [apply, enabled, he, stopped, hc] <;> inv_tac
theorem inv_keygen (v b : Int) (he : s.phase = 2 ∧ 1 ≤ v) : Inv (apply s (.keygen v b)) := by
  by_cases hc : vkMax < b <;> simp [apply, enabled, he.1, he.2, stopped, hc] <;> inv_tac
theorem inv_expect (x : Bool) (he : s.phase = 3) : Inv (apply s (.expect x)) := by
  cases x <;> simp [apply, enabled, he, stopped] <;> inv_tac
set_option linter.unusedSimpArgs false in
theorem inv_decode (r w m : Int) (t : Bool) (he : s.phase = 4) : Inv (apply s (.decode r w m t)) := by
  by_cases hc : r = anchorSlot ∧ w = anchorSlot ∧ ¬(m = 0 ∧ t = true) <;>
    simp only [apply, enabled, he, stopped, hc, decide_true, Bool.not_true, ite_false] <;>
    simp [hc] <;> inv_tac
theorem inv_prove (x : Bool) (he : s.phase = 5) : Inv (apply s (.prove x)) := by
  cases x <;> simp [apply, enabled, he, stopped] <;> inv_tac
set_option maxHeartbeats 1000000 in
theorem inv_deploy (he : s.phase = 6) : Inv (apply s .deploy) := by
  have hen : (!enabled s .deploy) = false := by simp [enabled, he]
  unfold apply; rw [hen]; dsimp only [Bool.false_eq_true, ite_false]
  rw [ite_eq_right (by decide : ¬ (false = true))]
  by_cases hq : s.qual ≠ qualAll
  · rw [ite_eq_left hq]; inv_tac
  · rw [ite_eq_right hq]
    by_cases hk : s.eraVk ≠ 0 ∧ s.eraVk ≠ s.vkId
    · rw [ite_eq_left hk]; inv_tac
    · rw [ite_eq_right hk]
      by_cases h0 : s.eraVk = 0
      · rw [ite_eq_left h0]; inv_tac
      · rw [ite_eq_right h0]; inv_tac
theorem inv_gpu (v : Int) (he : s.phase = 6) : Inv (apply s (.gpuProve v)) := by
  by_cases hc : v = s.vkId <;> simp [apply, enabled, he, hc] <;> inv_tac
theorem inv_newCall (he : s.phase = 6 ∨ s.phase = 7) : Inv (apply s .newCall) := by
  simp [apply, enabled, he]; inv_tac
theorem inv_qualify (i : Int) (hi : 0 ≤ i ∧ i < 5) : Inv (apply s (.qualify i)) := by
  have hr := addBit_range s.qual i h.qualRange hi
  have hf : 0 < s.deployed → addBit s.qual i = qualAll := fun hd => by
    rw [h.deployQual hd]; exact addBit_full i hi
  simp only [apply, enabled, hi, decide_true, Bool.not_true, and_self]
  cases h
  constructor <;> simp_all [qualAll] <;> omega
end

/-- A disabled operation leaves the state unchanged. -/
theorem apply_disabled (s : St) (op : Op) (he : enabled s op = false) : apply s op = s := by
  simp [apply, he]

/-- Every invariant holds after every decoded operation. -/
theorem inv_apply (s : St) (op : Op) (h : Inv s) :
    Inv (apply s op) := by
  cases hen : enabled s op
  · rw [apply_disabled s op hen]; exact h
  cases op with
  | askK k => exact inv_askK s h k (by simpa [enabled] using hen)
  | cost r => exact inv_cost s h r (by simpa [enabled] using hen)
  | keygen v b => exact inv_keygen s h v b (by simpa [enabled] using hen)
  | expect x => exact inv_expect s h x (by simpa [enabled] using hen)
  | decode r w m t => exact inv_decode s h r w m t (by simpa [enabled] using hen)
  | prove x => exact inv_prove s h x (by simpa [enabled] using hen)
  | qualify i => exact inv_qualify s h i (by simpa [enabled] using hen)
  | deploy => exact inv_deploy s h (by simpa [enabled] using hen)
  | gpuProve v => exact inv_gpu s h v (by simpa [enabled] using hen)
  | newCall => exact inv_newCall s h (by simpa [enabled] using hen)

theorem inv_stepOp (s : St) (n : Int) (h : Inv s) : Inv (stepOp s n) := by
  unfold stepOp
  split
  · exact h
  · rename_i op _
    exact inv_apply s op h

theorem inv_runOps (s : St) (ns : List Int) (h : Inv s) : Inv (runOps s ns) := by
  induction ns generalizing s with
  | nil => exact h
  | cons n ns ih => exact ih _ (inv_stepOp s n h)

/-- Every state the oracle reaches satisfies every invariant. -/
theorem inv_run (t : Int) (ns : List Int) : Inv (run t ns) := inv_runOps _ _ (inv_init _)

/-! ## Requirement-level corollaries over every run -/

theorem WRP_002_never_k22 (t : Int) (ns : List Int) :
    (run t ns).keygenRan = true → (run t ns).k ≤ 21 := (inv_run t ns).kOk

theorem WRP_003_row_gate (t : Int) (ns : List Int) :
    (run t ns).keygenRan = true → (run t ns).rows ≤ 1782573 := (inv_run t ns).rowsOk

theorem WRP_004_vk_bound (t : Int) (ns : List Int) :
    (run t ns).vkFileWritten = true → (run t ns).vkBytes ≤ 50000 := (inv_run t ns).vkOk

theorem WRP_001_one_proof (t : Int) (ns : List Int) : (run t ns).proofs ≤ 1 := (inv_run t ns).oneProof

theorem runOps_testSeed (s : St) (ns : List Int) : (runOps s ns).testSeed = s.testSeed := by
  induction ns generalizing s with
  | nil => rfl
  | cons n ns ih =>
    simp only [runOps]; rw [ih]
    unfold stepOp; split
    · rfl
    · unfold apply; split
      · rfl
      · rename_i op _ _; cases op <;> simp only [stopped] <;> (repeat' split) <;> rfl

/-- WRP-006, LW-012, ZK-003: in a production build two successive proofs never share
a blinder. -/
theorem WRP_006_fresh_blinders (ns : List Int) :
    2 ≤ (run 0 ns).draws → (run 0 ns).prevBlinder ≠ (run 0 ns).lastBlinder := by
  intro hd
  have h := (inv_run 0 ns).fresh (by rw [run, runOps_testSeed]; decide)
  omega

theorem WRP_007_anchor_slot (t : Int) (ns : List Int) :
    (run t ns).phase = 6 → (run t ns).anchorRead = 15 ∧ (run t ns).chunkWrite = 15 :=
  fun h => let ⟨a, b, _⟩ := (inv_run t ns).decodeOk (Or.inr h); ⟨a, b⟩

theorem WRP_009_three_files (t : Int) (ns : List Int) :
    (run t ns).files = 0 ∨ (run t ns).files = 3 := by
  rcases (inv_run t ns).filesOk with h | ⟨h, _⟩ <;> simp [h]

theorem WRP_010_one_key (t : Int) (ns : List Int) : (run t ns).keysDeployed ≤ 1 :=
  (inv_run t ns).oneKey.1

theorem WRP_011_gpu_key (t : Int) (ns : List Int) :
    (run t ns).gpuVk = 0 ∨ (run t ns).gpuVk = (run t ns).vkId := (inv_run t ns).gpuKey

theorem WRP_012_deploy_qualified (t : Int) (ns : List Int) :
    0 < (run t ns).deployed → (run t ns).qual = 31 := (inv_run t ns).deployQual

theorem ANC_016_no_template_in_ordinary_call (t : Int) (ns : List Int) :
    (run t ns).phase = 6 → ¬((run t ns).mode = 0 ∧ (run t ns).template = true) :=
  fun h => ((inv_run t ns).decodeOk (Or.inr h)).2.2

/-! ## Constants checked against MEASURED and DERIVED values

Sources (Moriarty working copy, `research/sherlock-tested-design-20261113/LOOP.md`):
line 20 (T3: wrap 1,668,183 rows, key 1,325 bytes), line 21 (T4: 1,676,222 rows with
every check), line 31 (T10m: wrap 1,685,845 rows at k = 21, VK 1,325 bytes, D3 weakest
101.0110 bits), line 102 (hard rules). These are the T10m key set; the same checks on
the T25 key set are open (RKY-003, RKY-004 to build). -/

/-- DG-002: the gate is 85 % of the `2^21 - 7` usable rows, rounded down. -/
theorem rowGate_derived : rowGate = (85 * (2 ^ 21 - 7)) / 100 := by decide
/-- DG-001, WRP-002: the largest admitted k is 21 (hard rule, LOOP.md:102). -/
theorem kMax_is_21 : kMax = 21 ∧ kMax < 22 := by decide
/-- WRP-003, RKY-003 on the T10m keys: 1,685,845 rows (LOOP.md:31) fit the gate. -/
theorem measured_t10m_rows : (1685845 : Int) ≤ rowGate := by decide
/-- WRP-003, LW-016: T4 budget with every check, 1,676,222 rows (LOOP.md:21). -/
theorem measured_t4_rows : (1676222 : Int) ≤ rowGate ∧ rowGate - 1676222 = 106351 := by decide
/-- T3 wrap rows, 1,668,183 (LOOP.md:20). -/
theorem measured_t3_rows : (1668183 : Int) ≤ rowGate := by decide
/-- WRP-004, DG-003, MID-003, RKY-004 on the T10m keys: VK 1,325 bytes (LOOP.md:31). -/
theorem measured_t10m_vk : (1325 : Int) ≤ vkMax := by decide
/-- LW-003: 1,026 public inputs (WRP-010 status, LW-003). -/
theorem publicInputs_1026 : publicInputs = 1026 := by decide
/-- LW-019, SQG-004: D3 weakest term 101.0110 bits on the T10m keys (LOOP.md:31),
scaled by 10^4, is at least 100 bits and at least the grindable 101 bits. -/
theorem measured_d3_weakest : (1010110 : Int) ≥ 1000000 ∧ (1010110 : Int) ≥ 1010000 := by decide

end Wrap
