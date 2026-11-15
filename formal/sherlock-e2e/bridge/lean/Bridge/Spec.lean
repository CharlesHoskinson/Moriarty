/-!
# Bridge: `L`, `L_Fr`, the call binding and the hand-off to the wrap

Lean oracle of the Bridge bounded context (slice B, `bus/ddd-B.md` section 1).
Specs: `sherlock-e2e-l-transcript` (LTR-001 to LTR-008), `sherlock-e2e-call-binding`
(CLB-001 to CLB-008); reused LW, FLG, HDM (processor-final, completion), ANC (processor-final,
completion).

Abstraction: one call of one run goes from the `L` key to one `LfrProofReady` record and the
wrap start. Witness facts (the lanes equal the guest effects, the tape exposes the public
words, the transcript equals the stock one, the lock operand is the run's) are booleans of
the state that the model chooses and that no operation changes. Digests are natural numbers.

An operation is one natural number `op = code * 16 + arg` (`arg < 16`):

| code | operation | arg |
|---|---|---|
| 0 | `keygenL`: the key generator makes the `L` (and `L_Fr`) key | - |
| 1 | `plan`: the chunk planner admits the call | - |
| 2 | `proveL f`: `L` is proved over the node root (f = 1) or on path C (f = 0) | 0..1 |
| 3 | `proveLfr`: `L_Fr` is proved over the `L` proof | - |
| 4 | `stockVerify`: the stock verifier checks the `L_Fr` proof | - |
| 5 | `publish`: the `L_Fr` prover publishes `LfrProofReady` | - |
| 6 | `transfer c`: the proof file moves to muninn; c = 1 changes one byte | 0..1 |
| 7 | `wrapStart`: muninn checks the record and starts the wrap | - |
-/

namespace Bridge

structure St where
  /-- columns of the `L` key and the largest interaction count of one AIR (LTR-003) -/
  lCols : Nat
  lInter : Nat
  /-- anchor slot constant of the `L_Fr` interface and of the wrap decoder (CLB-004, CLB-008) -/
  slotIface : Nat
  slotWrap : Nat
  /-- the `L` key id (0 = none) and the keygens so far (LTR-007) -/
  lKeyId : Nat
  keygens : Nat
  /-- 0 ordinary call (ORD), 1 chunked run (ANC-003) -/
  mode : Nat
  /-- stated chunk count and the run's slice count (CLB-003, CLB-007) -/
  k : Nat
  slices : Nat
  /-- every effect lane holds the guest's word at that lane (CLB-002, ANC-018) -/
  laneOk : Bool
  /-- the call carries the anchor-write template in lanes 11 to 19 and 21 (CLB-006, ANC-016) -/
  tplWrite : Bool
  /-- every lock operand `x_j` is the run's operand (CLB-005, ANC-017) -/
  lockOk : Bool
  /-- the tape exposes the public words through `Tape::expose` (LTR-004) -/
  tapeExposed : Bool
  /-- every tape operand is well typed (LW-020, LW-021) -/
  tapeTyped : Bool
  /-- the tape replays the stock `L` transcript (LTR-005, HDM-004) -/
  transcriptOk : Bool
  /-- the stock verifier accepts the `L_Fr` proof (LTR-008) -/
  stockAccepts : Bool
  /-- the planner admitted the call -/
  planned : Bool
  /-- 0 planned or not, 1 `L` proved, 2 `L_Fr` proved, 3 verified, 4 record published -/
  phase : Nat
  /-- `L` key id used on path C and on the full tree (0 = not yet) (LTR-007) -/
  keyPathC : Nat
  keyFull : Nat
  /-- the stock verifier refused the proof (LTR-008) -/
  verifierRefused : Bool
  /-- sha256 of the `L_Fr` proof bytes, the record's `proof_sha256`, the moved bytes (LTR-006) -/
  proofSha : Nat
  recordSha : Nat
  movedSha : Nat
  /-- `LfrProofReady` records published and wrap starts (LTR-006, LTR-008) -/
  records : Nat
  wraps : Nat
  deriving DecidableEq, Repr

/-- LTR-003: `L` key bounds (2,200 columns after F-B-BR-1: FP-3 `L` is 2,071 DERIVED). -/
def lMaxCols : Nat := 2200
def lMaxInter : Nat := 100
/-- CLB-004, ANC-001, ANC-013: the anchor slot. -/
def anchorSlot : Nat := 15
/-- CLB-007, ANC-012: chunk cap. -/
def kMax : Nat := 2

inductive Refusal where
  | lKeyBounds          -- LTR-003
  | anchorSlotWrong     -- CLB-004
  | slotMismatch        -- CLB-008
  | keysExist           -- LTR-007 (one L key for every tree form)
  | noKey               -- the call needs the L key
  | kMaxExceeded        -- CLB-007
  | wrongPhase          -- order of the pipeline
  | laneUnbound         -- CLB-002
  | kMismatch           -- CLB-003
  | ordinaryAnchorWrite -- CLB-006
  | tapeNotExposed      -- LTR-004
  | illTypedTape        -- LW-021
  | transcriptMismatch  -- LTR-005
  | lockMismatch        -- CLB-005
  | notVerified         -- LTR-008
  | digestMismatch      -- LTR-006
  | outsideCapability
  deriving DecidableEq, Repr

def Refusal.code : Refusal → String
  | .lKeyBounds => "l-key-bounds"
  | .anchorSlotWrong => "anchor-slot"
  | .slotMismatch => "slot-mismatch"
  | .keysExist => "keys-exist"
  | .noKey => "no-key"
  | .kMaxExceeded => "k-max"
  | .wrongPhase => "wrong-phase"
  | .laneUnbound => "lane-unbound"
  | .kMismatch => "k-mismatch"
  | .ordinaryAnchorWrite => "ordinary-anchor-write"
  | .tapeNotExposed => "tape-not-exposed"
  | .illTypedTape => "ill-typed-tape"
  | .transcriptMismatch => "transcript-mismatch"
  | .lockMismatch => "lock-mismatch"
  | .notVerified => "not-verified"
  | .digestMismatch => "digest-mismatch"
  | .outsideCapability => "outside-capability"

inductive Op where
  | keygenL
  | plan
  | proveL (full : Nat)
  | proveLfr
  | stockVerify
  | publish
  | transfer (corrupt : Nat)
  | wrapStart
  | bad
  deriving DecidableEq, Repr

def decodeOp (op : Nat) : Op :=
  let a := op % 16
  match op / 16 with
  | 0 => if a = 0 then .keygenL else .bad
  | 1 => if a = 0 then .plan else .bad
  | 2 => if a < 2 then .proveL a else .bad
  | 3 => if a = 0 then .proveLfr else .bad
  | 4 => if a = 0 then .stockVerify else .bad
  | 5 => if a = 0 then .publish else .bad
  | 6 => if a < 2 then .transfer a else .bad
  | 7 => if a = 0 then .wrapStart else .bad
  | _ => .bad

/-- State domain. Each conjunct carries a requirement as an invariant of reachable states. -/
def Valid (s : St) : Prop :=
  s.phase ≤ 4 ∧ s.mode ≤ 1 ∧ s.lKeyId ≤ s.keygens ∧
  -- LTR-003, CLB-004, CLB-008: an L key exists only within its bounds and with slot 15 twice
  (s.lKeyId ≠ 0 → s.lCols ≤ lMaxCols ∧ s.lInter ≤ lMaxInter ∧ s.slotIface = anchorSlot ∧
    s.slotWrap = anchorSlot) ∧
  -- CLB-007: an admitted chunked call has at most K_MAX chunks
  (s.planned = true → s.lKeyId ≠ 0 ∧ (s.mode = 1 → s.k ≤ kMax)) ∧
  -- CLB-002, CLB-003, CLB-006: an L proof binds lanes, k and the ordinary-call template
  (s.phase ≥ 1 → s.planned = true ∧ s.laneOk = true ∧ (s.mode = 1 → s.k = s.slices) ∧
    (s.mode = 0 → s.tplWrite = false)) ∧
  -- LTR-004, LTR-005, LW-021, CLB-005: an L_Fr proof has an exposed, typed, stock tape and locks
  (s.phase ≥ 2 → s.tapeExposed = true ∧ s.tapeTyped = true ∧ s.transcriptOk = true ∧
    (s.mode = 1 → s.lockOk = true)) ∧
  -- LTR-008: verified only when the stock verifier accepts
  (s.phase ≥ 3 → s.stockAccepts = true) ∧
  -- LTR-006, LTR-008: a record exists only for a verified proof and names its sha256
  (s.records ≠ 0 → s.phase = 4 ∧ s.recordSha = s.proofSha ∧ s.stockAccepts = true) ∧
  (s.phase = 4 → s.records = 1) ∧
  -- LTR-006: the wrap starts only on bytes equal to the record's digest
  (s.wraps ≠ 0 → s.records ≠ 0 ∧ s.movedSha = s.recordSha) ∧
  -- LTR-007: path C and the full tree use the one L key
  (s.keyPathC ≠ 0 → s.keyPathC = s.lKeyId) ∧ (s.keyFull ≠ 0 → s.keyFull = s.lKeyId)

set_option synthInstance.maxSize 4096 in
set_option synthInstance.maxHeartbeats 400000 in
instance (s : St) : Decidable (Valid s) := by unfold Valid; infer_instance

def validState (s : St) : Bool := decide (Valid s)

def keygenRefusals (s : St) : List Refusal :=
  (if s.lCols ≤ lMaxCols ∧ s.lInter ≤ lMaxInter then [] else [.lKeyBounds]) ++  -- LTR-003
  (if s.slotIface = anchorSlot then [] else [.anchorSlotWrong]) ++             -- CLB-004
  (if s.slotIface = s.slotWrap then [] else [.slotMismatch]) ++                -- CLB-008
  (if s.lKeyId = 0 then [] else [.keysExist])                                  -- LTR-007

def proveLRefusals (s : St) : List Refusal :=
  (if s.planned = true ∧ s.phase = 0 then [] else [.wrongPhase]) ++
  (if s.laneOk = true then [] else [.laneUnbound]) ++                          -- CLB-002, ANC-018
  (if s.mode = 1 ∧ s.k ≠ s.slices then [.kMismatch] else []) ++                -- CLB-003, ANC-019
  (if s.mode = 0 ∧ s.tplWrite = true then [.ordinaryAnchorWrite] else [])      -- CLB-006, ANC-016

def proveLfrRefusals (s : St) : List Refusal :=
  (if s.phase = 1 then [] else [.wrongPhase]) ++
  (if s.tapeExposed = true then [] else [.tapeNotExposed]) ++                  -- LTR-004
  (if s.tapeTyped = true then [] else [.illTypedTape]) ++                      -- LW-021
  (if s.transcriptOk = true then [] else [.transcriptMismatch]) ++             -- LTR-005
  (if s.mode = 1 ∧ s.lockOk = false then [.lockMismatch] else [])              -- CLB-005, ANC-017

def stepRefusals (s : St) (op : Nat) : List Refusal :=
  if validState s = false then [.outsideCapability] else
  match decodeOp op with
  | .keygenL => keygenRefusals s
  | .plan =>                                                                   -- CLB-007
    (if s.lKeyId ≠ 0 then [] else [.noKey]) ++
    (if s.planned = false then [] else [.wrongPhase]) ++
    (if s.mode = 1 ∧ kMax < s.k then [.kMaxExceeded] else [])
  | .proveL _ => proveLRefusals s
  | .proveLfr => proveLfrRefusals s
  | .stockVerify => if s.phase = 2 ∧ s.verifierRefused = false then [] else [.wrongPhase]
  | .publish => if s.phase = 3 then [] else [.notVerified]                   -- LTR-008
  | .transfer _ => if s.records ≠ 0 ∧ s.wraps = 0 then [] else [.wrongPhase]  -- LTR-006
  | .wrapStart =>                                                              -- LTR-006
    (if s.records ≠ 0 then [] else [.wrongPhase]) ++
    (if s.movedSha = s.recordSha then [] else [.digestMismatch])
  | .bad => [.outsideCapability]

def step (s : St) (op : Nat) : St :=
  match decodeOp op with
  | .keygenL => { s with keygens := s.keygens + 1, lKeyId := s.keygens + 1 }  -- LTR-003, LTR-007
  | .plan => { s with planned := true }                                       -- CLB-007
  | .proveL f =>                                                               -- CLB-001..003, CLB-006, LTR-007
    if f = 0 then { s with phase := 1, keyPathC := s.lKeyId }
    else { s with phase := 1, keyFull := s.lKeyId }
  | .proveLfr => { s with phase := 2, proofSha := s.proofSha + 1 }            -- LTR-002, LTR-004, LTR-005, CLB-005
  | .stockVerify =>                                                            -- LTR-008
    if s.stockAccepts = true then { s with phase := 3 } else { s with verifierRefused := true }
  | .publish => { s with phase := 4, records := s.records + 1, recordSha := s.proofSha }  -- LTR-006
  | .transfer c => { s with movedSha := s.recordSha + c }                     -- LTR-006
  | .wrapStart => { s with wraps := s.wraps + 1 }                             -- LTR-006
  | .bad => s

def apply (s : St) (op : Nat) : St := if stepRefusals s op = [] then step s op else s

def run (s : St) : List Nat → St
  | [] => s
  | op :: ops => run (apply s op) ops

def runRefusals (s : St) (_ops : List Nat) : List Refusal :=
  if validState s then [] else [.outsideCapability]

/-- A call before the `L` key exists. -/
def init (lCols lInter slotIface slotWrap mode k slices : Nat)
    (laneOk tplWrite lockOk tapeExposed tapeTyped transcriptOk stockAccepts : Bool) : St :=
  { lCols, lInter, slotIface, slotWrap, lKeyId := 0, keygens := 0, mode, k, slices, laneOk,
    tplWrite, lockOk, tapeExposed, tapeTyped, transcriptOk, stockAccepts, planned := false,
    phase := 0, keyPathC := 0, keyFull := 0, verifierRefused := false, proofSha := 0,
    recordSha := 0, movedSha := 0, records := 0, wraps := 0 }

/-! ## Theorems -/

/-- LTR-003, CLB-004, CLB-008: the `L` key is made only within the bounds and with one anchor
    slot constant, 15, on the interface and in the wrap decoder. -/
theorem keygen_guard (s : St) (h : stepRefusals s 0 = []) :
    s.lCols ≤ lMaxCols ∧ s.lInter ≤ lMaxInter ∧ s.slotIface = anchorSlot ∧
    s.slotWrap = anchorSlot := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;>
    simp_all [stepRefusals, decodeOp, keygenRefusals] <;> split at h <;> simp_all <;> omega

/-- CLB-007: the planner admits no chunked call above K_MAX = 2. -/
theorem plan_guard (s : St) (h : stepRefusals s (1 * 16) = []) (hm : s.mode = 1) :
    s.k ≤ kMax := by
  refine Classical.byContradiction fun hc => ?_
  simp_all [stepRefusals, decodeOp] <;> split at h <;> simp_all <;> omega

/-- CLB-002, CLB-003, CLB-006: `L` is proved only when every lane is the guest's word, `k` is
    the slice count of a chunked run and an ordinary call carries no anchor-write template. -/
theorem proveL_guard (s : St) (f : Nat) (hf : f < 2) (h : stepRefusals s (2 * 16 + f) = []) :
    s.laneOk = true ∧ (s.mode = 1 → s.k = s.slices) ∧ (s.mode = 0 → s.tplWrite = false) := by
  have hd : (2 * 16 + f) / 16 = 2 := by omega
  have hm : (2 * 16 + f) % 16 = f := by omega
  refine ⟨?_, ?_, ?_⟩
  · refine Classical.byContradiction fun hc => ?_
    simp_all [stepRefusals, decodeOp, proveLRefusals] <;> split at h <;> simp_all
  · intro h1; refine Classical.byContradiction fun hc => ?_
    simp_all [stepRefusals, decodeOp, proveLRefusals] <;> split at h <;> simp_all
  · intro h1; refine Classical.byContradiction fun hc => ?_
    simp_all [stepRefusals, decodeOp, proveLRefusals] <;> split at h <;> simp_all

/-- LTR-004, LTR-005, LW-021, CLB-005: `L_Fr` is proved only over an exposed, well-typed tape
    that replays the stock transcript, and over the run's lock operands. -/
theorem proveLfr_guard (s : St) (h : stepRefusals s (3 * 16) = []) :
    s.tapeExposed = true ∧ s.tapeTyped = true ∧ s.transcriptOk = true ∧
    (s.mode = 1 → s.lockOk = true) := by
  refine ⟨?_, ?_, ?_, ?_⟩
  · refine Classical.byContradiction fun hc => ?_
    simp_all [stepRefusals, decodeOp, proveLfrRefusals] <;> split at h <;> simp_all
  · refine Classical.byContradiction fun hc => ?_
    simp_all [stepRefusals, decodeOp, proveLfrRefusals] <;> split at h <;> simp_all
  · refine Classical.byContradiction fun hc => ?_
    simp_all [stepRefusals, decodeOp, proveLfrRefusals] <;> split at h <;> simp_all
  · intro h1; refine Classical.byContradiction fun hc => ?_
    simp_all [stepRefusals, decodeOp, proveLfrRefusals] <;> split at h <;> simp_all

/-- LTR-006: the wrap starts only on moved bytes whose sha256 equals `proof_sha256`. -/
theorem wrapStart_guard (s : St) (h : stepRefusals s (7 * 16) = []) :
    s.records ≠ 0 ∧ s.movedSha = s.recordSha := by
  refine ⟨?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;>
    simp_all [stepRefusals, decodeOp] <;> split at h <;> simp_all

/-- LTR-008: when the stock verifier refuses, the proof never reaches the verified phase, so
    `publish` is refused for it. -/
theorem refused_proof_no_record (s : St) (hp : s.phase = 2) (hr : s.stockAccepts = false) :
    (step s (4 * 16)).phase = 2 ∧ (step s (4 * 16)).records = s.records := by
  simp [step, decodeOp, hr, hp]

theorem apply_valid (s : St) (op : Nat) (h : Valid s) : Valid (apply s op) := by
  unfold apply
  split
  · rename_i hr
    have hv : validState s = true := by simp [validState, h]
    simp only [stepRefusals, hv, Bool.true_eq_false, ite_false] at hr
    unfold Valid at h ⊢
    obtain ⟨h1, h2, h3, h4, h5, h6, h7, h8, h9, h10, h11, h12, h13⟩ := h
    unfold step
    split
    · -- keygenL
      rename_i hop
      simp only [hop, keygenRefusals, List.append_eq_nil_iff] at hr
      have hb : s.lCols ≤ lMaxCols ∧ s.lInter ≤ lMaxInter :=
        Classical.byContradiction fun hc => by simp [hc] at hr
      have hs : s.slotIface = anchorSlot := Classical.byContradiction fun hc => by simp [hc] at hr
      have hw : s.slotIface = s.slotWrap := Classical.byContradiction fun hc => by simp [hc] at hr
      have hk : s.lKeyId = 0 := Classical.byContradiction fun hc => by simp [hc] at hr
      refine ⟨h1, h2, by simp, fun _ => ⟨hb.1, hb.2, hs, by rw [← hw]; exact hs⟩, ?_, ?_, h7, h8, h9, h10, h11,
        ?_, ?_⟩ <;> simp_all
    · -- plan
      rename_i hop
      simp only [hop, List.append_eq_nil_iff] at hr
      have hk : s.lKeyId ≠ 0 := Classical.byContradiction fun hc => by simp_all
      have hc : s.mode = 1 → s.k ≤ kMax := fun hm =>
        Classical.byContradiction fun hc => by simp_all
      dsimp only
      exact ⟨h1, h2, h3, h4, fun _ => ⟨hk, hc⟩, fun hp => ⟨rfl, (h6 hp).2⟩, h7, h8, h9, h10, h11, h12, h13⟩
    · -- proveL
      rename_i f hop
      simp only [hop, proveLRefusals, List.append_eq_nil_iff] at hr
      have hp : s.planned = true ∧ s.phase = 0 :=
        Classical.byContradiction fun hc => by simp [hc] at hr
      have hl : s.laneOk = true := Classical.byContradiction fun hc => by simp_all
      have hk : s.mode = 1 → s.k = s.slices := fun hm =>
        Classical.byContradiction fun hc => by simp_all
      have ht : s.mode = 0 → s.tplWrite = false := fun hm =>
        Classical.byContradiction fun hc => by simp_all
      have hr0 : s.records = 0 := Classical.byContradiction fun hc => by
        have := (h9 hc).1; omega
      split
      · refine ⟨by simp, h2, h3, h4, h5, fun _ => ⟨hp.1, hl, hk, ht⟩, ?_, ?_, ?_, ?_, ?_, ?_, h13⟩
          <;> simp_all
      · refine ⟨by simp, h2, h3, h4, h5, fun _ => ⟨hp.1, hl, hk, ht⟩, ?_, ?_, ?_, ?_, ?_, h12, ?_⟩
          <;> simp_all
    · -- proveLfr
      rename_i hop
      simp only [hop, proveLfrRefusals, List.append_eq_nil_iff] at hr
      have hp : s.phase = 1 := Classical.byContradiction fun hc => by simp [hc] at hr
      have ha : s.tapeExposed = true := Classical.byContradiction fun hc => by simp_all
      have hb : s.tapeTyped = true := Classical.byContradiction fun hc => by simp_all
      have hc' : s.transcriptOk = true := Classical.byContradiction fun hc => by simp_all
      have hd : s.mode = 1 → s.lockOk = true := fun hm =>
        Classical.byContradiction fun hc => by simp_all
      have hr0 : s.records = 0 := Classical.byContradiction fun hc => by
        have := (h9 hc).1; omega
      refine ⟨by simp, h2, h3, h4, h5, fun _ => h6 (by omega), fun _ => ⟨ha, hb, hc', hd⟩,
        ?_, ?_, ?_, ?_, h12, h13⟩ <;> simp_all
    · -- stockVerify
      rename_i hop
      simp only [hop] at hr
      have hp : s.phase = 2 := Classical.byContradiction fun hc => by simp [hc] at hr
      have hr0 : s.records = 0 := Classical.byContradiction fun hc => by
        have := (h9 hc).1; omega
      split
      · rename_i ha
        refine ⟨by simp, h2, h3, h4, h5, fun _ => h6 (by omega), fun _ => h7 (by omega),
          fun _ => ha, ?_, ?_, ?_, h12, h13⟩ <;> simp_all
      · dsimp only
        exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, h9, h10, h11, h12, h13⟩
    · -- publish
      rename_i hop
      simp only [hop] at hr
      have hp : s.phase = 3 := Classical.byContradiction fun hc => by simp [hc] at hr
      have hr0 : s.records = 0 := Classical.byContradiction fun hc => by
        have := (h9 hc).1; omega
      have hw0 : s.wraps = 0 := Classical.byContradiction fun hc => (h11 hc).1 hr0
      refine ⟨by simp, h2, h3, h4, h5, fun _ => h6 (by omega), fun _ => h7 (by omega),
        fun _ => h8 (by omega), ?_, ?_, ?_, h12, h13⟩ <;> simp_all
    · -- transfer
      rename_i c hop
      simp only [hop] at hr
      have hw : s.wraps = 0 := Classical.byContradiction fun hc => by simp_all
      dsimp only
      exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, h9, h10, fun hc => absurd hw hc, h12, h13⟩
    · -- wrapStart
      rename_i hop
      simp only [hop, List.append_eq_nil_iff] at hr
      have ha : s.records ≠ 0 := Classical.byContradiction fun hc => by simp_all
      have hb : s.movedSha = s.recordSha := Classical.byContradiction fun hc => by simp_all
      dsimp only
      exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, h9, h10, fun _ => ⟨ha, hb⟩, h12, h13⟩
    · -- bad
      rename_i hop
      simp [hop] at hr
  · exact h

theorem init_valid (a b c d e f g : Nat) (p q r x u v w : Bool) (he : e ≤ 1) :
    Valid (init a b c d e f g p q r x u v w) := by
  simp [Valid, init, he]

theorem run_valid (s : St) (ops : List Nat) (h : Valid s) : Valid (run s ops) := by
  induction ops generalizing s with
  | nil => exact h
  | cons op ops ih => exact ih _ (apply_valid s op h)

/-- LTR-008, LTR-006, CLB-002, CLB-003, CLB-006, CLB-007, LTR-007: in every reachable state a
    published `LfrProofReady` record belongs to a proof the stock verifier accepted, over an
    `L` proof that bound the lanes, `k` and the ordinary-call template, under the one `L` key. -/
theorem reachable_record (a b c d e f g : Nat) (p q r x u v w : Bool) (he : e ≤ 1)
    (ops : List Nat) :
    let t := run (init a b c d e f g p q r x u v w) ops
    (t.records ≠ 0 → t.stockAccepts = true ∧ t.recordSha = t.proofSha ∧ t.laneOk = true ∧
      (t.mode = 1 → t.k = t.slices ∧ t.k ≤ kMax ∧ t.lockOk = true) ∧
      (t.mode = 0 → t.tplWrite = false)) ∧
    (t.keyPathC ≠ 0 → t.keyFull ≠ 0 → t.keyPathC = t.keyFull) := by
  have hv := run_valid _ ops (init_valid a b c d e f g p q r x u v w he)
  simp only
  obtain ⟨_, _, _, _, h5, h6, h7, _, h9, _, _, h12, h13⟩ := hv
  refine ⟨fun hr => ?_, fun hp hf => by rw [h12 hp, h13 hf]⟩
  have ⟨hp4, hsha, hacc⟩ := h9 hr
  have ⟨hpl, hl, hk, ht⟩ := h6 (by omega)
  have ⟨_, _, _, hlock⟩ := h7 (by omega)
  exact ⟨hacc, hsha, hl, fun hm => ⟨hk hm, (h5 hpl).2 hm, hlock hm⟩, ht⟩

end Bridge
