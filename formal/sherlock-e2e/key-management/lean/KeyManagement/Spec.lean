/-!
# Key Management: the universal key set, the single rekey, the key manifest

Lean oracle of the Key Management bounded context (slice B, `bus/ddd-B.md` section 1).
Specs: `sherlock-e2e-universal-keys` (UKS-001 to UKS-010); reused RKY-001, RKY-002,
RKY-005 (completion), CPR-003, SPL-004, SPL-005 (processor-final).

Abstraction: digests are natural numbers. A key manifest is named by its id
(`manifestD`, 0 = none yet). A box holds the id of the manifest it received.
The six key-changing rows that UKS-010 names are bits of `listRows`; the twelve
gates of UKS-006 are bits of `gatesPass`.

An operation is one natural number `op = code * 16 + arg` (`arg < 16`):

| code | operation | arg |
|---|---|---|
| 0 | `keygen`: the rekey runner starts and makes the keys | - |
| 1 | `gatePass g`: gate `g` of UKS-006 passed on the new keys | gate 0..11 |
| 2 | `complete`: the rekey is DONE | - |
| 3 | `distribute b`: box `b` receives the keys and the manifest | box 0..2 |
| 4 | `startProof b`: a proof job starts on box `b` | box 0..2 |
| 5 | `setup b`: setup mode runs on box `b` | box 0..2 |
| 6 | `addProgram p`: a new program `p` is planned (its keys go under `key_root`) | program 0..15 |
| 7 | `securityGate`: the gate of SSG-006 (G1 to G5, FP-8) passed on this key set | - |
| 8 | `changeList d`: `LIST.md` changes (its sha256 changes) | 0..15 |
-/

namespace KeyManagement

/-- Observable state. Field names are the `--step` JSON keys. -/
structure St where
  /-- bit i set: `LIST.md` has row i of UKS-010 (0 FP-3, 1 anchor slot 15, 2 range checks,
      3 R_SL statement bindings, 4 R_SL key leaf index, 5 era-check words) -/
  listRows : Nat
  /-- every row of the list has a commit on its branch (RKY-002) -/
  rowsCommitted : Bool
  /-- the keygen tree descends from every commit of the list (UKS-009) -/
  treeHoldsAll : Bool
  /-- `key_root` leaf index of the R_SL key leaves (UKS-003) -/
  rslKeyIdx : Nat
  /-- path C DAG leaf index `KEYDAG_IDX` (UKS-003) -/
  keydagIdx : Nat
  /-- sha256 of `LIST.md` now (abstract) -/
  listD : Nat
  /-- id of the current universal key manifest; 0 = no keys yet (UKS-004) -/
  manifestD : Nat
  /-- `LIST.md` sha256 recorded in the manifest (UKS-004) -/
  manifestListD : Nat
  /-- number of keygens so far (RKY-001) -/
  keygens : Nat
  /-- 0 no keys, 1 keys made and gates pending, 2 rekey DONE (UKS-006) -/
  phase : Nat
  /-- bit g set: gate g of UKS-006 passed on the current keys -/
  gatesPass : Nat
  /-- D3: lowest term in bits, lowest grindable term in bits (RKY-005) -/
  d3MinBits : Nat
  d3MinGrindBits : Nat
  /-- manifest status: false = `prototype`, true = `qualified` (UKS-007) -/
  qualified : Bool
  /-- manifest id held by box 0 (muninn), 1 (huginn), 2 (odin); 0 = none (UKS-005) -/
  box0 : Nat
  box1 : Nat
  box2 : Nat
  /-- proof jobs started (UKS-005) -/
  proofs : Nat
  /-- setup runs (UKS-008) -/
  setupRuns : Nat
  /-- `key_root`: commitment of the per-program keys (SPL-004) -/
  keyRoot : Nat
  /-- programs planned under `key_root` (CPR-003) -/
  programs : Nat
  deriving DecidableEq, Repr

/-- The six rows of UKS-010, all present. -/
def allRows : Nat := 63
/-- The twelve gates of UKS-006, all passed. -/
def allGates : Nat := 4095
/-- The gates that UKS-006 names, in its order. -/
def gateNames : List String :=
  ["check-cases", "V.9 omit", "V.10 check-chain", "pathc", "pathc-d3", "gp2", "x7",
   "d3work", "iface-fixture", "lfr-e3", "i7-decode", "sg6"]
/-- RKY-005: every term at least 100 bits, every grindable term at least 101 bits. -/
def d3Bits : Nat := 100
def d3GrindBits : Nat := 101
/-- UKS-001, CPR-003: the universal key set is the internal-recursive, `L`, `L_Fr` and wrap key. -/
def universalKeys : List String := ["internal-recursive", "L", "L_Fr", "wrap"]
/-- UKS-002: `keys_d` has 7 node rows. -/
def nodeRows : Nat := 7
def boxes : Nat := 3

/-- Closed refusal type: one constructor per response the specs distinguish. -/
inductive Refusal where
  | listRowMissing      -- UKS-010
  | rowUncommitted      -- RKY-002
  | treeNotDescendant   -- UKS-009
  | keyLeafCollision    -- UKS-003
  | keysCurrent         -- RKY-001 (keys are made once from one list)
  | wrongPhase          -- UKS-006 (order of the rekey steps)
  | gatesIncomplete     -- UKS-006
  | d3BelowBound        -- RKY-005
  | manifestMismatch    -- UKS-005
  | outsideCapability   -- an operation code or argument outside the model
  deriving DecidableEq, Repr

def Refusal.code : Refusal → String
  | .listRowMissing => "list-row-missing"
  | .rowUncommitted => "row-uncommitted"
  | .treeNotDescendant => "tree-not-descendant"
  | .keyLeafCollision => "key-leaf-collision"
  | .keysCurrent => "keys-current"
  | .wrongPhase => "wrong-phase"
  | .gatesIncomplete => "gates-incomplete"
  | .d3BelowBound => "d3-below-bound"
  | .manifestMismatch => "manifest-mismatch"
  | .outsideCapability => "outside-capability"

inductive Op where
  | keygen
  | gatePass (g : Nat)
  | complete
  | distribute (b : Nat)
  | startProof (b : Nat)
  | setup (b : Nat)
  | addProgram (p : Nat)
  | securityGate
  | changeList (d : Nat)
  | bad
  deriving DecidableEq, Repr

def decodeOp (op : Nat) : Op :=
  let a := op % 16
  match op / 16 with
  | 0 => if a = 0 then .keygen else .bad
  | 1 => if a < 12 then .gatePass a else .bad
  | 2 => if a = 0 then .complete else .bad
  | 3 => if a < boxes then .distribute a else .bad
  | 4 => if a < boxes then .startProof a else .bad
  | 5 => if a < boxes then .setup a else .bad
  | 6 => .addProgram a
  | 7 => if a = 0 then .securityGate else .bad
  | 8 => .changeList a
  | _ => .bad

def boxManifest (s : St) (b : Nat) : Nat :=
  if b = 0 then s.box0 else if b = 1 then s.box1 else s.box2

def setBox (s : St) (b m : Nat) : St :=
  if b = 0 then { s with box0 := m } else if b = 1 then { s with box1 := m } else { s with box2 := m }

/-- State domain (not reachability). It carries UKS-006 (DONE keys passed every gate) and
    UKS-007 (only DONE keys can be `qualified`). -/
def Valid (s : St) : Prop :=
  s.phase ≤ 2 ∧ s.manifestD ≤ s.keygens ∧ (s.phase = 0 ↔ s.manifestD = 0) ∧
  s.box0 ≤ s.manifestD ∧ s.box1 ≤ s.manifestD ∧ s.box2 ≤ s.manifestD ∧
  (s.qualified = true → s.phase = 2) ∧ (s.phase = 2 → s.gatesPass = allGates)

instance (s : St) : Decidable (Valid s) := by unfold Valid; infer_instance

def validState (s : St) : Bool := decide (Valid s)

/-- Refusals of the keygen guard, in a fixed order. -/
def keygenRefusals (s : St) : List Refusal :=
  (if s.listRows = allRows then [] else [.listRowMissing]) ++         -- UKS-010
  (if s.rowsCommitted then [] else [.rowUncommitted]) ++              -- RKY-002
  (if s.treeHoldsAll then [] else [.treeNotDescendant]) ++            -- UKS-009
  (if s.rslKeyIdx = s.keydagIdx then [.keyLeafCollision] else []) ++  -- UKS-003
  (if s.manifestD ≠ 0 ∧ s.manifestListD = s.listD then [.keysCurrent] else [])  -- RKY-001

def stepRefusals (s : St) (op : Nat) : List Refusal :=
  if validState s = false then [.outsideCapability] else
  match decodeOp op with
  | .keygen => keygenRefusals s
  | .gatePass _ => if s.phase = 1 then [] else [.wrongPhase]                      -- UKS-006
  | .complete =>                                                                   -- UKS-006, RKY-005
    (if s.phase = 1 then [] else [.wrongPhase]) ++
    (if s.gatesPass = allGates then [] else [.gatesIncomplete]) ++
    (if d3Bits ≤ s.d3MinBits ∧ d3GrindBits ≤ s.d3MinGrindBits then [] else [.d3BelowBound])
  | .distribute _ => if s.phase = 2 then [] else [.wrongPhase]                    -- UKS-006
  | .startProof b =>                                                               -- UKS-005
    (if s.phase = 2 then [] else [.wrongPhase]) ++
    (if boxManifest s b = s.manifestD then [] else [.manifestMismatch])
  | .setup _ => []                                                                 -- UKS-008
  | .addProgram _ => []                                                            -- CPR-003, SPL-004
  | .securityGate => if s.phase = 2 then [] else [.wrongPhase]                    -- UKS-007
  | .changeList _ => []                                                            -- UKS-004
  | .bad => [.outsideCapability]

/-- One transition (applied only when `stepRefusals` is empty). -/
def step (s : St) (op : Nat) : St :=
  match decodeOp op with
  | .keygen =>
    -- RKY-001: the keys are made once from every row of the list.
    -- UKS-004: the manifest records the list sha256; a new manifest id.
    -- UKS-007: a new key set starts as `prototype`.
    { s with keygens := s.keygens + 1, manifestD := s.keygens + 1, manifestListD := s.listD,
             phase := 1, gatesPass := 0, qualified := false }
  | .gatePass g => { s with gatesPass := s.gatesPass ||| (2 ^ g) }                 -- UKS-006
  | .complete => { s with phase := 2 }                                             -- UKS-006, RKY-005
  | .distribute b => setBox s b s.manifestD                                        -- UKS-006, UKS-005
  | .startProof _ => { s with proofs := s.proofs + 1 }                             -- UKS-005
  | .setup _ => { s with setupRuns := s.setupRuns + 1 }                            -- UKS-008
  | .addProgram p => { s with keyRoot := s.keyRoot * 16 + p + 1, programs := s.programs + 1 }  -- SPL-004
  | .securityGate => { s with qualified := true }                                  -- UKS-007, SSG-006
  | .changeList d => { s with listD := s.listD + d + 1 }                           -- UKS-004
  | .bad => s

/-- Apply a refused operation as a no-op (the `--step` contract: state unchanged). -/
def apply (s : St) (op : Nat) : St := if stepRefusals s op = [] then step s op else s

def run (s : St) : List Nat → St
  | [] => s
  | op :: ops => run (apply s op) ops

/-- Whole-run refusals: only the state domain of the initial state. -/
def runRefusals (s : St) (_ops : List Nat) : List Refusal :=
  if validState s then [] else [.outsideCapability]

/-- The initial state of a deployment: no keys yet. -/
def init (listRows : Nat) (rowsCommitted treeHoldsAll : Bool) (rslKeyIdx keydagIdx listD
    d3MinBits d3MinGrindBits : Nat) : St :=
  { listRows, rowsCommitted, treeHoldsAll, rslKeyIdx, keydagIdx, listD, manifestD := 0,
    manifestListD := 0, keygens := 0, phase := 0, gatesPass := 0, d3MinBits, d3MinGrindBits,
    qualified := false, box0 := 0, box1 := 0, box2 := 0, proofs := 0, setupRuns := 0,
    keyRoot := 0, programs := 0 }

/-! ## Theorems (each names its requirement) -/

/-- UKS-010, RKY-002, UKS-009, UKS-003: the rekey starts only with every row, every commit,
    a tree that holds every lane and distinct key leaf indexes. -/
theorem keygen_guard (s : St) (h : stepRefusals s 0 = []) :
    s.listRows = allRows ∧ s.rowsCommitted = true ∧ s.treeHoldsAll = true ∧
    s.rslKeyIdx ≠ s.keydagIdx := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;>
    simp_all [stepRefusals, decodeOp, keygenRefusals] <;> split at h <;> simp_all

/-- RKY-001: a second keygen from the same list is refused (the keys are made once). -/
theorem keygen_once (s : St) (hm : s.manifestD ≠ 0) (hl : s.manifestListD = s.listD) :
    Refusal.keysCurrent ∈ stepRefusals s 0 ∨ stepRefusals s 0 = [.outsideCapability] := by
  unfold stepRefusals
  split
  · right; rfl
  · left; simp [decodeOp, keygenRefusals, hm, hl]

/-- UKS-004: after a keygen the manifest records the list sha256 and a fresh id. -/
theorem keygen_writes_manifest (s : St) :
    (step s 0).manifestListD = s.listD ∧ (step s 0).manifestD = s.keygens + 1 ∧
    (step s 0).qualified = false := by
  simp [step, decodeOp]

/-- UKS-004 (scenario "A second keygen"): a later change of `LIST.md` makes the recorded
    sha256 differ, so a second keygen is no longer refused as `keys-current`. -/
theorem changed_list_needs_keygen (s : St) (d : Nat) (h : s.manifestListD = s.listD) :
    (step s (8 * 16 + d % 16)).manifestListD ≠ (step s (8 * 16 + d % 16)).listD := by
  have hd : (8 * 16 + d % 16) / 16 = 8 := by omega
  have hm : (8 * 16 + d % 16) % 16 = d % 16 := by omega
  simp only [step, decodeOp, hd, hm]
  omega

/-- UKS-006, RKY-005: the rekey is DONE only after every gate passed and D3 holds. -/
theorem complete_guard (s : St) (h : stepRefusals s (2 * 16) = []) :
    s.phase = 1 ∧ s.gatesPass = allGates ∧ d3Bits ≤ s.d3MinBits ∧ d3GrindBits ≤ s.d3MinGrindBits := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;>
    simp_all [stepRefusals, decodeOp] <;> split at h <;> simp_all <;> omega

/-- UKS-006: no box receives keys before the rekey is DONE. -/
theorem distribute_guard (s : St) (b : Nat) (hb : b < boxes)
    (h : stepRefusals s (3 * 16 + b) = []) : s.phase = 2 := by
  have hd : (3 * 16 + b) / 16 = 3 := by unfold boxes at hb; omega
  have hm : (3 * 16 + b) % 16 = b := by unfold boxes at hb; omega
  unfold stepRefusals at h
  split at h
  · simp at h
  · simp only [decodeOp, hd, hm, hb, ite_true] at h
    refine Classical.byContradiction fun hc => ?_; simp [hc] at h

/-- UKS-005: a proof starts on a box only when the box holds the manifest of the rekey. -/
theorem startProof_guard (s : St) (b : Nat) (hb : b < boxes)
    (h : stepRefusals s (4 * 16 + b) = []) : boxManifest s b = s.manifestD ∧ s.phase = 2 := by
  have hd : (4 * 16 + b) / 16 = 4 := by unfold boxes at hb; omega
  have hm : (4 * 16 + b) % 16 = b := by unfold boxes at hb; omega
  unfold stepRefusals at h
  split at h
  · simp at h
  · simp only [decodeOp, hd, hm, hb, ite_true, List.append_eq_nil_iff] at h
    obtain ⟨h1, h2⟩ := h
    constructor
    · refine Classical.byContradiction fun hc => ?_; simp [hc] at h2
    · refine Classical.byContradiction fun hc => ?_; simp [hc] at h1

/-- The fields that a key, a shape, `plan_d` or a proof value depend on (all but `setupRuns`). -/
def keyView (s : St) : St := { s with setupRuns := 0 }

/-- UKS-008: setup mode makes, changes and selects no key. -/
theorem setup_changes_no_key (s : St) (b : Nat) (hb : b < boxes) :
    keyView (apply s (5 * 16 + b)) = keyView s := by
  have hd : (5 * 16 + b) / 16 = 5 := by unfold boxes at hb; omega
  have hm : (5 * 16 + b) % 16 = b := by unfold boxes at hb; omega
  unfold apply
  split
  · simp [step, decodeOp, hd, hm, hb, keyView]
  · rfl

/-- UKS-001, CPR-003, SPL-004: a new program changes `key_root` and no universal key
    (the manifest, its status and the key set held by each box are unchanged). -/
theorem addProgram_keeps_universal (s : St) (p : Nat) (hp : p < 16) :
    let t := step s (6 * 16 + p)
    t.manifestD = s.manifestD ∧ t.qualified = s.qualified ∧ t.box0 = s.box0 ∧
    t.box1 = s.box1 ∧ t.box2 = s.box2 ∧ t.keyRoot ≠ s.keyRoot := by
  have hd : (6 * 16 + p) / 16 = 6 := by omega
  have hm : (6 * 16 + p) % 16 = p := by omega
  simp [step, decodeOp, hd, hm] <;> omega

/-- The step domain is preserved: every applied operation keeps `Valid`. With
    `init_valid` this makes UKS-006 (DONE only with every gate) and UKS-007 (`qualified` only
    on DONE keys) invariants of every reachable state. -/
theorem apply_valid (s : St) (op : Nat) (h : Valid s) : Valid (apply s op) := by
  unfold apply
  split
  · rename_i hr
    have hv : validState s = true := by simp [validState, h]
    simp only [stepRefusals, hv, Bool.true_eq_false, ite_false] at hr
    unfold Valid at h ⊢
    obtain ⟨h1, h2, h3, h4, h5, h6, h7, h8⟩ := h
    unfold step
    split
    · -- keygen
      simp only
      refine ⟨by omega, by omega, by simp, by omega, by omega, by omega, by simp, by simp⟩
    · -- gatePass
      rename_i g hop
      simp only [hop] at hr
      have hp : s.phase = 1 := Classical.byContradiction fun hc => by simp [hc] at hr
      exact ⟨h1, h2, h3, h4, h5, h6, fun hq => by simp_all, fun hq => by simp_all⟩
    · -- complete
      rename_i hop
      simp only [hop, List.append_eq_nil_iff] at hr
      have hp : s.phase = 1 := Classical.byContradiction fun hc => by simp [hc] at hr
      have hg : s.gatesPass = allGates := Classical.byContradiction fun hc => by simp [hc] at hr
      refine ⟨by simp, h2, ⟨fun hc => by simp at hc, fun hc => absurd (h3.mpr hc) (by omega)⟩,
        h4, h5, h6, fun _ => rfl, fun _ => hg⟩
    · -- distribute
      rename_i b hop
      simp only [hop] at hr
      unfold setBox
      split
      · exact ⟨h1, h2, h3, Nat.le_refl _, h5, h6, h7, h8⟩
      · split
        · exact ⟨h1, h2, h3, h4, Nat.le_refl _, h6, h7, h8⟩
        · exact ⟨h1, h2, h3, h4, h5, Nat.le_refl _, h7, h8⟩
    · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8⟩
    · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8⟩
    · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8⟩
    · -- securityGate
      rename_i hop
      simp only [hop] at hr
      have hp : s.phase = 2 := Classical.byContradiction fun hc => by simp [hc] at hr
      exact ⟨h1, h2, h3, h4, h5, h6, fun _ => hp, h8⟩
    · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8⟩
    · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8⟩
  · exact h

theorem init_valid (a : Nat) (b c : Bool) (d e f g k : Nat) : Valid (init a b c d e f g k) := by
  simp [Valid, init]

theorem run_valid (s : St) (ops : List Nat) (h : Valid s) : Valid (run s ops) := by
  induction ops generalizing s with
  | nil => exact h
  | cons op ops ih => exact ih _ (apply_valid s op h)

/-- UKS-007: in every reachable state a `qualified` key set is a DONE key set, and a key set
    is DONE only after all twelve gates of UKS-006 passed. -/
theorem reachable_status (a : Nat) (b c : Bool) (d e f g k : Nat) (ops : List Nat) :
    let t := run (init a b c d e f g k) ops
    (t.qualified = true → t.phase = 2) ∧ (t.phase = 2 → t.gatesPass = allGates) := by
  have := run_valid _ ops (init_valid a b c d e f g k)
  exact ⟨this.2.2.2.2.2.2.1, this.2.2.2.2.2.2.2⟩

end KeyManagement
