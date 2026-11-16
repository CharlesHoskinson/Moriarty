/-!
# Leaf Proving: the app proof and the leaf-layer proof of each shard of one run

Lean oracle of the Leaf Proving bounded context (slice B, `bus/ddd-B.md` section 1).
Spec: `sherlock-e2e-leaf-proving` (LFP-001 to LFP-012); reused RSL-001 to RSL-008
(completion), CPR-001, CPR-002, CPR-004, CPR-005, SPL-006 (processor-final), LPC-018.

Abstraction: one run of one R_SL program. Digests (`prog_d`, `input_d`, the tape digest,
the statement digest of `comm` and the ordered W, the FP-1 outcome commitment) are natural
numbers. The app shape is given by its AIR count without FP-1 (`appAirs`), its hash kits
and its column count. A run of `steps` steps cut into shards of `shardCap` steps has
`shards = ceil(steps / shardCap)` shards.

An operation is one natural number `op = code * 16 + arg` (`arg < 16`):

| code | operation | arg |
|---|---|---|
| 0 | `admit`: the leaf prover builds the chip set and admits the app shape | - |
| 1 | `proveShard`: one app proof and one leaf-layer proof of the next shard | - |
| 2 | `foreignLeaf p`: a leaf proof of program `p` joins the leaves of the run | program 0..15 |
| 3 | `publish`: the leaf prover publishes `LeafProofsReady` | - |
| 4 | `n1Merge`: N1 merges the leaves of the run | - |
| 5 | `extend d`: the run grows by `d + 1` shard lengths (before any shard is proved) | 0..15 |
-/

namespace LeafProving

structure St where
  /-- app AIRs without FP-1 (LFP-003, RSL-002) -/
  appAirs : Nat
  /-- the program calls `persistentHash` (SHA-256 kit) / `keccak256` (LFP-003, RSL-004, RSL-007) -/
  hasSha : Bool
  hasKeccak : Bool
  /-- columns of the app verifying key (LFP-004) -/
  appCols : Nat
  /-- an ALU or Poseidon input word of a row is not canonical (RSL-008) -/
  nonCanonical : Bool
  /-- every leaf trace has the ordered AIR height vector of the admitted shape (LFP-005) -/
  shapeOk : Bool
  /-- `prog_d` of the run's program (LFP-006, LFP-011) -/
  progD : Nat
  /-- `input_d` of the leaf statement / digest of the tape the app consumed (LFP-012) -/
  inputD : Nat
  tapeD : Nat
  /-- digest of `comm` and the ordered W / outcome commitment in FP-1 (LFP-010) -/
  stmtD : Nat
  fp1Outcome : Nat
  /-- run length and shard length in steps (LFP-008, CPR-002) -/
  steps : Nat
  shardCap : Nat
  /-- `key_root` of the program and the universal key manifest id (LFP-008) -/
  keyRoot : Nat
  manifestD : Nat
  /-- the shape was admitted (LFP-003, LFP-004, RSL-002) -/
  admitted : Bool
  /-- shards proved; app proofs; leaf-layer proofs (LFP-002) -/
  shardsProved : Nat
  appProofs : Nat
  leafProofs : Nat
  /-- `PROG_D` slot of the last proved leaf (LFP-006, LFP-011) -/
  leafProgD : Nat
  /-- leaves of other programs that joined (LFP-007); those whose `prog_d` differs -/
  foreignLeaves : Nat
  badLeaves : Nat
  /-- `LeafProofsReady` records published and the leaf count the record names (LFP-009) -/
  records : Nat
  recordLeaves : Nat
  /-- N1 merged the leaves (LFP-007) -/
  merged : Bool
  deriving DecidableEq, Repr

/-- LFP-003: at most 8 app AIRs (`MAX_APP_AIRS`). -/
def maxAppAirs : Nat := 8
/-- RSL-002: the 6-or-9 AIR bucket with FP-1. -/
def inBucket (appAirs : Nat) : Bool := appAirs + 1 == 6 || appAirs + 1 == 9
/-- LFP-004: at most 30,000 app columns. -/
def maxAppCols : Nat := 30000

def shards (s : St) : Nat := (s.steps + s.shardCap - 1) / s.shardCap

/-- LFP-002, RSL-001: the universal leaf gives the universal N1 pre-hash exactly for the
    apps in the bucket (the proof-shape encoder keeps 3 columns for 6 to 9 child AIRs;
    the a05 probe at 10 AIRs gave another pre-hash, LIST.md row 5d77cfc). -/
def n1PreHashUniversal (s : St) : Bool := inBucket s.appAirs

inductive Refusal where
  | airCountOverBound   -- LFP-003
  | mixedHash           -- LFP-003 (scenario: SHA-256 and Keccak-256 in one program)
  | outsideBucket       -- RSL-002
  | columnBound         -- LFP-004
  | notAdmitted         -- LFP-003 (no proof before the shape is admitted)
  | allShardsProved     -- LFP-002 (one proof per shard)
  | shapeMismatch       -- LFP-005, CPR-002, SPL-006
  | inputMismatch       -- LFP-012
  | statementUnbound    -- LFP-010
  | noncanonicalWord    -- RSL-008
  | leavesIncomplete    -- LFP-009
  | wrongProgD          -- LFP-007
  | notPublished        -- LFP-009 (N1 reads the record)
  | runStarted          -- LFP-008 (the run length is fixed once proving starts)
  | alreadyMerged       -- LFP-007 (a leaf joins before N1 merges)
  | outsideCapability
  deriving DecidableEq, Repr

def Refusal.code : Refusal → String
  | .airCountOverBound => "air-count-over-bound"
  | .mixedHash => "mixed-hash"
  | .outsideBucket => "outside-bucket"
  | .columnBound => "column-bound"
  | .notAdmitted => "not-admitted"
  | .allShardsProved => "all-shards-proved"
  | .shapeMismatch => "shape-mismatch"
  | .inputMismatch => "input-mismatch"
  | .statementUnbound => "statement-unbound"
  | .noncanonicalWord => "noncanonical-word"
  | .leavesIncomplete => "leaves-incomplete"
  | .wrongProgD => "wrong-prog-d"
  | .notPublished => "not-published"
  | .runStarted => "run-started"
  | .alreadyMerged => "already-merged"
  | .outsideCapability => "outside-capability"

inductive Op where
  | admit
  | proveShard
  | foreignLeaf (p : Nat)
  | publish
  | n1Merge
  | extend (d : Nat)
  | bad
  deriving DecidableEq, Repr

def decodeOp (op : Nat) : Op :=
  let a := op % 16
  match op / 16 with
  | 0 => if a = 0 then .admit else .bad
  | 1 => if a = 0 then .proveShard else .bad
  | 2 => .foreignLeaf a
  | 3 => if a = 0 then .publish else .bad
  | 4 => if a = 0 then .n1Merge else .bad
  | 5 => .extend a
  | _ => .bad

/-- The shape facts that admission checks (LFP-003, RSL-002, LFP-004). -/
def ShapeOk (s : St) : Prop :=
  s.appAirs ≤ maxAppAirs ∧ ¬(s.hasSha = true ∧ s.hasKeccak = true) ∧ inBucket s.appAirs = true ∧
  s.appCols ≤ maxAppCols

/-- State domain. It carries the invariants of every reachable state: proofs only after
    admission of a good shape (LFP-003, LFP-004, RSL-002), one app proof and one leaf-layer
    proof per proved shard (LFP-002), no more proved shards than the run has (CPR-002),
    a merge only of leaves of the run's program after the record (LFP-007, LFP-009). -/
def Valid (s : St) : Prop :=
  0 < s.shardCap ∧ s.shardsProved ≤ shards s ∧ s.appProofs = s.shardsProved ∧
  s.leafProofs = s.shardsProved ∧ s.badLeaves ≤ s.foreignLeaves ∧
  (s.admitted = false → s.shardsProved = 0) ∧ (s.admitted = true → ShapeOk s) ∧
  (s.shardsProved ≠ 0 → s.leafProgD = s.progD) ∧
  (s.records ≠ 0 → s.recordLeaves = shards s) ∧
  (s.merged = true → s.badLeaves = 0 ∧ s.records ≠ 0)

instance (s : St) : Decidable (ShapeOk s) := by unfold ShapeOk; infer_instance
instance (s : St) : Decidable (Valid s) := by unfold Valid; infer_instance

def validState (s : St) : Bool := decide (Valid s)

def admitRefusals (s : St) : List Refusal :=
  (if s.appAirs ≤ maxAppAirs then [] else [.airCountOverBound]) ++                 -- LFP-003
  (if s.hasSha = true ∧ s.hasKeccak = true then [.mixedHash] else []) ++           -- LFP-003
  (if inBucket s.appAirs then [] else [.outsideBucket]) ++                         -- RSL-002
  (if s.appCols ≤ maxAppCols then [] else [.columnBound])                          -- LFP-004

def proveRefusals (s : St) : List Refusal :=
  (if s.admitted then [] else [.notAdmitted]) ++                                   -- LFP-003
  (if s.shardsProved < shards s then [] else [.allShardsProved]) ++                -- LFP-002
  (if s.shapeOk then [] else [.shapeMismatch]) ++                                  -- LFP-005
  (if s.tapeD = s.inputD then [] else [.inputMismatch]) ++                         -- LFP-012
  (if s.fp1Outcome = s.stmtD then [] else [.statementUnbound]) ++                  -- LFP-010
  (if s.nonCanonical then [.noncanonicalWord] else [])                             -- RSL-008

def stepRefusals (s : St) (op : Nat) : List Refusal :=
  if validState s = false then [.outsideCapability] else
  match decodeOp op with
  | .admit => admitRefusals s
  | .proveShard => proveRefusals s
  -- LFP-007: N1 refuses a leaf whose prog_d differs from the run's; the run keeps its honest
  -- leaves and still merges (Formal-Quint D-leaf-2, lead ruling).
  | .foreignLeaf p => (if s.merged then [.alreadyMerged] else []) ++
                      (if p = s.progD then [] else [.wrongProgD])
  | .publish => if s.shardsProved = shards s then [] else [.leavesIncomplete]      -- LFP-009
  | .n1Merge =>                                                                     -- LFP-007, LFP-009
    (if s.records ≠ 0 then [] else [.notPublished]) ++
    (if s.badLeaves = 0 then [] else [.wrongProgD])
  | .extend _ => if s.shardsProved = 0 ∧ s.records = 0 then [] else [.runStarted]  -- LFP-008
  | .bad => [.outsideCapability]

def step (s : St) (op : Nat) : St :=
  match decodeOp op with
  | .admit => { s with admitted := true }                                          -- LFP-003, LFP-004, RSL-002
  | .proveShard =>
    -- LFP-002: one app proof (chip set 3) and one leaf-layer proof per shard.
    -- LFP-006, LFP-011: the leaf sets PROG_D to the program's prog_d.
    { s with shardsProved := s.shardsProved + 1, appProofs := s.appProofs + 1,
             leafProofs := s.leafProofs + 1, leafProgD := s.progD }
  | .foreignLeaf p =>                                                               -- LFP-007
    { s with foreignLeaves := s.foreignLeaves + 1,
             badLeaves := if p = s.progD then s.badLeaves else s.badLeaves + 1 }
  | .publish => { s with records := s.records + 1, recordLeaves := shards s }      -- LFP-009
  | .n1Merge => { s with merged := true }                                           -- LFP-007
  | .extend d => { s with steps := s.steps + (d + 1) * s.shardCap }                -- LFP-008, CPR-002
  | .bad => s

def apply (s : St) (op : Nat) : St := if stepRefusals s op = [] then step s op else s

def run (s : St) : List Nat → St
  | [] => s
  | op :: ops => run (apply s op) ops

def runRefusals (s : St) (_ops : List Nat) : List Refusal :=
  if validState s then [] else [.outsideCapability]

/-- A run before proving: the program's shape, digests and length. -/
def init (appAirs : Nat) (hasSha hasKeccak : Bool) (appCols : Nat) (nonCanonical shapeOk : Bool)
    (progD inputD tapeD stmtD fp1Outcome steps shardCap keyRoot manifestD : Nat) : St :=
  { appAirs, hasSha, hasKeccak, appCols, nonCanonical, shapeOk, progD, inputD, tapeD, stmtD,
    fp1Outcome, steps, shardCap, keyRoot, manifestD, admitted := false, shardsProved := 0,
    appProofs := 0, leafProofs := 0, leafProgD := 0, foreignLeaves := 0, badLeaves := 0,
    records := 0, recordLeaves := 0, merged := false }

/-! ## Theorems -/

/-- LFP-003, RSL-002, LFP-004: a shape is admitted only with at most 8 app AIRs, not both
    hash kits, in the 6-or-9 bucket and at most 30,000 columns. -/
theorem admit_guard (s : St) (h : stepRefusals s 0 = []) : ShapeOk s := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;>
    simp_all [stepRefusals, decodeOp, admitRefusals] <;> split at h <;> simp_all

/-- LFP-005, LFP-010, LFP-012, RSL-008: a shard is proved only from an admitted shape, with the
    admitted height vector, the bound input digest, the bound statement and canonical words. -/
theorem prove_guard (s : St) (h : stepRefusals s 16 = []) :
    s.admitted = true ∧ s.shardsProved < shards s ∧ s.shapeOk = true ∧ s.tapeD = s.inputD ∧
    s.fp1Outcome = s.stmtD ∧ s.nonCanonical = false := by
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;>
    simp_all [stepRefusals, decodeOp, proveRefusals] <;> split at h <;> simp_all

/-- LFP-007: N1 merges only when no leaf carries another `prog_d`, and only after the record. -/
theorem merge_guard (s : St) (h : stepRefusals s (4 * 16) = []) :
    s.badLeaves = 0 ∧ s.records ≠ 0 := by
  refine ⟨?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;>
    simp_all [stepRefusals, decodeOp] <;> split at h <;> simp_all

/-- LFP-009: the record is published only when every leaf of the run is proved, and it names
    every leaf. -/
theorem publish_guard (s : St) (h : stepRefusals s (3 * 16) = []) :
    s.shardsProved = shards s ∧ (step s (3 * 16)).recordLeaves = shards s := by
  refine ⟨Classical.byContradiction fun hc => ?_, ?_⟩
  · simp_all [stepRefusals, decodeOp]; split at h <;> simp_all
  · simp [step, decodeOp, shards]

/-- LFP-006, LFP-011: a proved leaf carries the run's `prog_d` in slot `PROG_D`. -/
theorem prove_sets_progD (s : St) : (step s 16).leafProgD = s.progD := by
  simp [step, decodeOp]

/-- LFP-008, CPR-002, SPL-006: a longer run adds shards and leaves `key_root` and the universal
    key manifest unchanged (no key constant bounds the run length). -/
theorem extend_keeps_keys (s : St) (d : Nat) (hd : d < 16) (hc : 0 < s.shardCap) :
    let t := step s (5 * 16 + d)
    t.keyRoot = s.keyRoot ∧ t.manifestD = s.manifestD ∧ t.shardCap = s.shardCap ∧
    shards t = shards s + (d + 1) := by
  have h1 : (5 * 16 + d) / 16 = 5 := by omega
  have h2 : (5 * 16 + d) % 16 = d := by omega
  refine ⟨by simp [step, decodeOp, h1, h2], by simp [step, decodeOp, h1, h2],
    by simp [step, decodeOp, h1, h2], ?_⟩
  simp only [step, decodeOp, h1, h2, shards]
  have : s.steps + (d + 1) * s.shardCap + s.shardCap - 1 =
      (s.steps + s.shardCap - 1) + (d + 1) * s.shardCap := by
    have : 1 ≤ s.shardCap := hc
    omega
  rw [this, Nat.add_mul_div_right _ _ hc]

/-- The shard count grows with the run length; the shard shape does not (CPR-002). -/
theorem shards_monotone (s : St) (k : Nat) (hc : 0 < s.shardCap) :
    shards s ≤ shards { s with steps := s.steps + k } := by
  show (s.steps + s.shardCap - 1) / s.shardCap ≤ (s.steps + k + s.shardCap - 1) / s.shardCap
  exact Nat.div_le_div_right (by omega)

/-- Every operation keeps `Valid`. -/
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
    · -- admit
      rename_i hop
      simp only [hop] at hr
      have hs : ShapeOk s := by
        refine ⟨?_, ?_, ?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;>
          simp_all [admitRefusals]
      exact ⟨h1, h2, h3, h4, h5, fun hc => by simp at hc, fun _ => hs, h8, h9, h10⟩
    · -- proveShard
      rename_i hop
      simp only [hop] at hr
      have ha : s.admitted = true := Classical.byContradiction fun hc => by
        simp_all [proveRefusals]
      have hl : s.shardsProved < shards s := Classical.byContradiction fun hc => by
        simp_all [proveRefusals]
      refine ⟨h1, ?_, by simp [h3], by simp [h4], h5, fun hc => by simp_all, fun _ => h7 ha,
        fun _ => rfl, h9, h10⟩
      simp only [shards] at hl ⊢
      omega
    · -- foreignLeaf
      rename_i p hop
      simp only [hop] at hr
      have hm0 : s.merged = false := by cases hx : s.merged <;> simp_all
      refine ⟨h1, h2, h3, h4, ?_, h6, h7, h8, h9, ?_⟩
      · simp only; split <;> omega
      · intro hm; simp_all
    · -- publish
      rename_i hop
      simp only [hop] at hr
      have hp : s.shardsProved = shards s := Classical.byContradiction fun hc => by simp_all
      exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, fun _ => rfl, fun hm => ⟨(h10 hm).1, by simp⟩⟩
    · -- n1Merge
      rename_i hop
      simp only [hop, List.append_eq_nil_iff] at hr
      have hr1 : s.records ≠ 0 := Classical.byContradiction fun hc => by simp_all
      have hb : s.badLeaves = 0 := Classical.byContradiction fun hc => by simp_all
      exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, h9, fun _ => ⟨hb, hr1⟩⟩
    · -- extend
      rename_i d hop
      simp only [hop] at hr
      have h0 : s.shardsProved = 0 ∧ s.records = 0 := Classical.byContradiction fun hc => by
        simp_all
      refine ⟨h1, by simp [h0.1], by simp [h3, h0.1], by simp [h4, h0.1], h5, h6, ?_, ?_, ?_, ?_⟩
      · intro ha; have := h7 ha; exact this
      · intro hc; simp_all
      · intro hc; simp_all
      · intro hm; have := h10 hm; simp_all
    · rename_i hop
      simp [hop] at hr
  · exact h

theorem init_valid (a : Nat) (b c : Bool) (d : Nat) (e f : Bool) (g i j k l m n o q : Nat)
    (hn : 0 < n) : Valid (init a b c d e f g i j k l m n o q) := by
  simp [Valid, init, hn]

theorem run_valid (s : St) (ops : List Nat) (h : Valid s) : Valid (run s ops) := by
  induction ops generalizing s with
  | nil => exact h
  | cons op ops ih => exact ih _ (apply_valid s op h)

/-- LFP-002, LFP-003, LFP-004, RSL-002, RSL-001: in every reachable state, one app proof and one
    leaf-layer proof per proved shard, every proof from an admitted shape inside the bound,
    the bucket and the column bound, and so with the universal N1 pre-hash; every proved
    leaf carries the run's `prog_d` (LFP-006, LFP-011); a merge has no foreign leaf (LFP-007). -/
theorem reachable_leaves (a : Nat) (b c : Bool) (d : Nat) (e f : Bool) (g i j k l m n o q : Nat)
    (hn : 0 < n) (ops : List Nat) :
    let t := run (init a b c d e f g i j k l m n o q) ops
    t.appProofs = t.shardsProved ∧ t.leafProofs = t.shardsProved ∧
    (t.shardsProved ≠ 0 → ShapeOk t ∧ n1PreHashUniversal t = true ∧ t.leafProgD = t.progD) ∧
    (t.merged = true → t.badLeaves = 0) := by
  have hv := run_valid _ ops (init_valid a b c d e f g i j k l m n o q hn)
  obtain ⟨_, _, h3, h4, _, h6, h7, h8, _, h10⟩ := hv
  refine ⟨h3, h4, fun hp => ?_, fun hm => (h10 hm).1⟩
  have ha : (run (init a b c d e f g i j k l m n o q) ops).admitted = true := by
    cases hx : (run (init a b c d e f g i j k l m n o q) ops).admitted
    · exact absurd (h6 hx) hp
    · rfl
  have hs := h7 ha
  exact ⟨hs, hs.2.2.1, h8 hp⟩

end LeafProving
