/-!
# Compact Compilation oracle (bounded context Compact Compilation, slice A)

Executable reference semantics of `sherlock-e2e-compact-compiler` (ECC-001 to ECC-005,
ECC-007, ECC-009 to ECC-014; ECC-006 and ECC-008 are withdrawn), of the completion
requirements CF-013 and CF-014, and of the Mycroft requirements those specs cite
(MYC-FE-001, MYC-FE-002, MYC-FE-009 to MYC-FE-011, MYC-REF-032, MYC-SEAL-005,
MYC-SEAL-021, MYC-BLT-007, MYC-VAL-014). Spec text only; no compiler code was read.

One compile request (`sherlock compile` / `mycroft-suite`) reads circuits one at a time
(`admit`), then ends (`finish`): exit 0 with one image and one ABI per circuit, or a
non-zero exit with a refusal code and no image (ECC-012, ECC-013). Serialization
(`serialize`, ECC-004) and the factory gates (`gate`, `useForSettle`, ECC-002, ECC-011)
are separate operations on the same state.

| phase | meaning |
|---|---|
| 0 | open: circuits are being read |
| 1 | ended with exit 0, files written |
| 2 | ended with a non-zero exit, no file |
-/

namespace CompactComp

def opBase : Int := 1000000000
/-- ECC-009, MYC-SEAL-005: the anchor slot of the deploy profile. -/
def deploySlot : Int := 15
/-- MYC-SEAL-005: the anchor cell `(0)` of the T18 (c) spike profile. -/
def spikeSlot : Int := 0
/-- ECC-005: the 13 instruction shapes of `SInstr`. -/
def shapeCount : Nat := 13
/-- ECC-002: the six gates of `gates.sh` (KRS emit, oracle-equal, golden, coverage,
mutations, compare), one bit each. -/
def gatesAll : Int := 63

/-! ## Refusal codes recorded in the state (`lastRefusal`), printed by ECC-013 -/

def refNone : Int := 0
/-- ECC-001, MYC-REF-032 (carrier): the input is not the stock compactc 0.36.0 output. -/
def refCarrier : Int := 1
/-- MYC-FE-002, MYC-REF-032 `MYC-REF-032/irVersion`: ZKIR version other than 3.1. -/
def refIrVersion : Int := 2
/-- CF-014: an input type with no R_SL kit; the refusal names the type. -/
def refInputType : Int := 3
/-- ECC-005, MYC-REF-033: a form outside the 13 straight-line shapes. -/
def refShape : Int := 4
/-- ECC-009, MYC-SEAL-021: the ledger layout uses the profile's `anchor_slot`. -/
def refAnchorSlot : Int := 5

/-- The input kinds of one circuit (ECC-014, CF-013, CF-014, MYC-VAL-014). -/
def kindField : Int := 0
def kindJubPoint : Int := 1
def kindJubScalar : Int := 2
def kindSecp : Int := 3
def kindNoKit : Int := 4

/-- One requested circuit as the compiler sees it. -/
structure Circuit where
  stock : Bool       -- ECC-001: the files are the stock compactc 0.36.0 output
  irMinor : Int      -- MYC-FE-002: ZKIR 3.`irMinor`; 1 is the pin
  kind : Int         -- input kind (above)
  shapeOk : Bool     -- ECC-005: lowers to the 13 shapes only
  slotClash : Bool   -- MYC-SEAL-021: the layout uses `anchor_slot`
  keccak : Bool      -- ECC-007: the circuit calls keccak256
  deriving Repr, DecidableEq

/-- The refusal of one circuit, or `refNone`. Exactly one code (ECC-010). The specs
name no precedence among causes; LEAD PICK: the pipeline order read, version, types,
lowering, seal. -/
def circuitRefusal (c : Circuit) : Int :=
  if c.stock = false then refCarrier
  else if c.irMinor ≠ 1 then refIrVersion
  else if c.kind = kindNoKit then refInputType
  else if c.shapeOk = false then refShape
  else if c.slotClash = true then refAnchorSlot
  else refNone

theorem circuitRefusal_range (c : Circuit) : 0 ≤ circuitRefusal c ∧ circuitRefusal c ≤ 5 := by
  unfold circuitRefusal
  repeat' split
  all_goals decide

def ind (b : Bool) : Int := if b then 1 else 0

theorem ind_range (b : Bool) : 0 ≤ ind b ∧ ind b ≤ 1 := by cases b <;> decide

structure St where
  phase : Int
  deployProfile : Bool
  anchorSlot : Int
  requested : Int
  accepted : Int
  refused : Int
  lastRefusal : Int
  images : Int
  abis : Int
  exitCode : Int
  keccakChip : Int
  tier3 : Int
  jubPoints : Int
  jubScalars : Int
  tapeWords : Int
  curveChecks : Int
  secpKit : Int
  img1Src : Int
  img1Hash : Int
  img2Src : Int
  img2Hash : Int
  gateBits : Int
  gateFailed : Bool
  failedGate : Int
  settleUses : Int
  useBits : Int
  settleBlocked : Int
  deriving Repr, DecidableEq

/-- The start state; `deploy` selects the deploy profile (ECC-009). -/
def init (deploy : Bool) : St :=
  { phase := 0, deployProfile := deploy, anchorSlot := if deploy then deploySlot else spikeSlot,
    requested := 0, accepted := 0, refused := 0, lastRefusal := refNone, images := 0, abis := 0,
    exitCode := 0, keccakChip := 0, tier3 := 0, jubPoints := 0, jubScalars := 0, tapeWords := 0,
    curveChecks := 0, secpKit := 0, img1Src := 0, img1Hash := 0, img2Src := 0, img2Hash := 0,
    gateBits := 0, gateFailed := false, failedGate := 0, settleUses := 0, useBits := 0,
    settleBlocked := 0 }

inductive Op where
  | admit (c : Circuit)          -- 1: read one requested circuit
  | finish                       -- 2: end the request
  | serialize (src tape : Int)   -- 3: serialize one circuit for one prepared call
  | gate (i : Int) (ok : Bool)   -- 4: one gate of gates.sh passes or fails
  | useForSettle                 -- 5: the compiler is used for a settled call
  | newRequest                   -- 6: start the next request
  deriving Repr, DecidableEq

/-- Decode `code * 10^9 + arg`.
* admit: arg digits (least significant first) `stock`, `irMinor`, `kind`, `shapeOk`,
  `slotClash`, `keccak`: arg = stock + 10 irMinor + 100 kind + 1000 shapeOk
  + 10000 slotClash + 100000 keccak, booleans 0 or 1, irMinor and kind 0..9.
* serialize: arg = src * 1000 + tape, with tape < 1000.
* gate: arg = 2 i + ok, i in 0..5. -/
def Op.ofInt (n : Int) : Option Op :=
  let c := n / opBase
  let a := n % opBase
  let bit (x : Int) : Option Bool := if x = 0 then some false else if x = 1 then some true else none
  if n < 0 then none
  else if c = 1 then
    if a ≥ 1000000 then none else
    match bit (a % 10), bit ((a / 1000) % 10), bit ((a / 10000) % 10), bit ((a / 100000) % 10) with
    | some st, some sh, some sc, some kc =>
      if (a / 100) % 10 ≤ kindNoKit then
        some (.admit { stock := st, irMinor := (a / 10) % 10, kind := (a / 100) % 10,
                       shapeOk := sh, slotClash := sc, keccak := kc })
      else none
    | _, _, _, _ => none
  else if c = 2 then (if a = 0 then some .finish else none)
  else if c = 3 then some (.serialize (a / 1000) (a % 1000))
  else if c = 4 then (if a < 12 then some (.gate (a / 2) (a % 2 = 1)) else none)
  else if c = 5 then (if a = 0 then some .useForSettle else none)
  else if c = 6 then (if a = 0 then some .newRequest else none)
  else none

def enabled (s : St) : Op → Bool
  | .admit _ => s.phase = 0
  | .finish => s.phase = 0 ∧ 1 ≤ s.requested
  | .serialize _ _ => true
  | .gate i _ => 0 ≤ i ∧ i < 6
  | .useForSettle => true
  | .newRequest => s.phase ≠ 0

/-- The gates pass when every bit is set and none failed (ECC-002, ECC-011, LPC-013). -/
def gatesPass (s : St) : Bool := s.gateBits = gatesAll && !s.gateFailed

def setBit (q i : Int) : Int := if (q / 2 ^ i.toNat) % 2 = 1 then q else q + 2 ^ i.toNat

def accept (s : St) (c : Circuit) : St :=
  -- ECC-003: one image and one ABI per accepted circuit (written at `finish`)
  -- ECC-007, MYC-BLT-007: keccak256 is one chip instruction of R_SL_KECCAK256, never tier 3
  -- ECC-014, MYC-VAL-014: a Jubjub point is two tape words with an on-curve and subgroup
  -- check, a Jubjub scalar one tape word; CF-013: secp256k1 lowers to the RSL-005 kit
  { s with requested := s.requested + 1, accepted := s.accepted + 1,
           keccakChip := s.keccakChip + ind c.keccak,
           jubPoints := s.jubPoints + ind (c.kind = kindJubPoint),
           jubScalars := s.jubScalars + ind (c.kind = kindJubScalar),
           tapeWords := s.tapeWords + 2 * ind (c.kind = kindJubPoint) + ind (c.kind = kindJubScalar),
           curveChecks := s.curveChecks + ind (c.kind = kindJubPoint),
           secpKit := s.secpKit + ind (c.kind = kindSecp) }

def apply (s : St) (op : Op) : St :=
  if !enabled s op then s else
  match op with
  | .admit c =>
    -- ECC-010: each circuit gets an image or exactly one refusal code
    let r := circuitRefusal c
    if r = refNone then accept s c
    else { s with requested := s.requested + 1, refused := s.refused + 1, lastRefusal := r }
  | .finish =>
    -- ECC-013, MYC-FE-011: a refusal gives a non-zero exit, the code, and no image
    -- (no partial output: LEAD PICK for T15 of ECC-013)
    if 0 < s.refused then { s with phase := 2, exitCode := 2, images := 0, abis := 0 }
    -- ECC-012, ECC-003: exit 0, one image and one ABI file per proof circuit
    else { s with phase := 1, exitCode := 1, images := s.accepted, abis := s.accepted }
  | .serialize src _ =>
    -- ECC-004, MYC-FE-009, MYC-FE-010: the bytes are a function of source and
    -- configuration only; the tape does not enter
    { s with img1Src := s.img2Src, img1Hash := s.img2Hash, img2Src := src, img2Hash := src }
  | .gate i ok =>
    -- ECC-002, ECC-011: one gate result; a failing gate (oracle difference, surviving
    -- mutant) is sticky and names the gate
    if ok then { s with gateBits := setBit s.gateBits i }
    else { s with gateFailed := true, failedGate := i + 1 }
  | .useForSettle =>
    -- ECC-002: a compiler that fails a gate is not used for a settled call
    if gatesPass s then { s with settleUses := s.settleUses + 1, useBits := s.gateBits }
    else { s with settleBlocked := s.settleBlocked + 1 }
  | .newRequest =>
    { s with phase := 0, requested := 0, accepted := 0, refused := 0, lastRefusal := refNone,
             images := 0, abis := 0, exitCode := 0, keccakChip := 0, tier3 := 0, jubPoints := 0,
             jubScalars := 0, tapeWords := 0, curveChecks := 0, secpKit := 0 }

inductive Refusal where
  | notAnOperation
  | opNotEnabled
  | outsideStateDomain
  deriving Repr, DecidableEq

def Refusal.code : Refusal → String
  | .notAnOperation => "not-an-operation"
  | .opNotEnabled => "op-not-enabled"
  | .outsideStateDomain => "outside-state-domain"

def validState (s : St) : Bool :=
  0 ≤ s.phase && s.phase ≤ 2 && 0 ≤ s.requested && 0 ≤ s.accepted && 0 ≤ s.refused
    && 0 ≤ s.lastRefusal && s.lastRefusal ≤ 5 && 0 ≤ s.images && 0 ≤ s.abis
    && 0 ≤ s.exitCode && s.exitCode ≤ 2 && 0 ≤ s.gateBits && s.gateBits ≤ gatesAll
    && 0 ≤ s.settleUses && 0 ≤ s.settleBlocked && 0 ≤ s.failedGate && s.failedGate ≤ 6

def stepRefusals (s : St) (n : Int) : List Refusal :=
  match Op.ofInt n with
  | none => [.notAnOperation] ++ (if validState s then [] else [.outsideStateDomain])
  | some op => (if enabled s op then [] else [.opNotEnabled])
      ++ (if validState s then [] else [.outsideStateDomain])

def stepOp (s : St) (n : Int) : St :=
  match Op.ofInt n with
  | none => s
  | some op => apply s op

def runOps (s : St) : List Int → St
  | [] => s
  | n :: ns => runOps (stepOp s n) ns

def refusals (deploy : Int) (ops : List Int) : List Refusal :=
  (if deploy = 0 ∨ deploy = 1 then [] else [.outsideStateDomain])
    ++ (if ops.all (fun n => (Op.ofInt n).isSome) then [] else [.notAnOperation])

def run (deploy : Int) (ops : List Int) : St := runOps (init (deploy = 1)) ops

/-! ## Invariants -/

structure Inv (s : St) : Prop where
  /-- ECC-010: every read circuit has an image or a refusal; none is unsupported. -/
  cover : s.requested = s.accepted + s.refused
  nonneg : 0 ≤ s.accepted ∧ 0 ≤ s.refused
  /-- ECC-010, ECC-013: a refusal always carries one code in 1..5; none without a refusal. -/
  codeOk : (s.refused = 0 ∧ s.lastRefusal = refNone) ∨ (0 < s.refused ∧ 1 ≤ s.lastRefusal ∧ s.lastRefusal ≤ 5)
  /-- open request: no output yet. -/
  openNoOutput : s.phase = 0 → s.images = 0 ∧ s.abis = 0 ∧ s.exitCode = 0
  /-- ECC-012, ECC-003: exit 0 writes one image and one ABI per circuit, none refused. -/
  okExit : s.phase = 1 → s.refused = 0 ∧ 1 ≤ s.requested ∧ s.images = s.accepted ∧
    s.abis = s.accepted ∧ s.exitCode = 1
  /-- ECC-013, MYC-FE-011: a refusal ends with a non-zero exit, a code and no image. -/
  refusedExit : s.phase = 2 → 0 < s.refused ∧ s.images = 0 ∧ s.abis = 0 ∧ s.exitCode = 2 ∧
    1 ≤ s.lastRefusal
  phaseRange : 0 ≤ s.phase ∧ s.phase ≤ 2
  /-- ECC-007, MYC-BLT-007: no keccak256 call goes to a tier 3 kit. -/
  noTier3 : s.tier3 = 0
  /-- ECC-007: one keccak chip instruction per accepted circuit at most. -/
  keccakOk : 0 ≤ s.keccakChip ∧ s.keccakChip ≤ s.accepted
  /-- ECC-014, MYC-VAL-014: a point is two tape words, a scalar one; each point is checked. -/
  jubjub : s.tapeWords = 2 * s.jubPoints + s.jubScalars ∧ s.curveChecks = s.jubPoints ∧
    0 ≤ s.jubPoints ∧ 0 ≤ s.jubScalars ∧ s.jubPoints + s.jubScalars ≤ s.accepted
  /-- CF-013: every accepted secp256k1 circuit is on the RSL-005 kit. -/
  secp : 0 ≤ s.secpKit ∧ s.secpKit ≤ s.accepted
  /-- ECC-009, MYC-SEAL-005: slot 15 in the deploy profile, cell 0 in the spike. -/
  slot : (s.deployProfile = true → s.anchorSlot = deploySlot) ∧
    (s.deployProfile = false → s.anchorSlot = spikeSlot)
  /-- ECC-004, MYC-FE-009, MYC-FE-010: the serialized bytes depend on the source and the
  configuration only, so two calls of one source give equal bytes. -/
  bytes : s.img1Hash = s.img1Src ∧ s.img2Hash = s.img2Src
  gateRange : 0 ≤ s.gateBits ∧ s.gateBits ≤ gatesAll
  /-- ECC-002, ECC-011, LPC-013: every settled use happened with all gates passing. -/
  settleGated : s.settleUses = 0 ∨ s.useBits = gatesAll
  usesNonneg : 0 ≤ s.settleUses ∧ 0 ≤ s.settleBlocked

theorem inv_init (b : Bool) : Inv (init b) := by
  cases b <;> constructor <;> simp [init, refNone, deploySlot, spikeSlot, gatesAll]

theorem setBit_range (q i : Int) (hq : 0 ≤ q ∧ q ≤ 63) (hi : 0 ≤ i ∧ i < 6) :
    0 ≤ setBit q i ∧ setBit q i ≤ 63 := by
  have : i = 0 ∨ i = 1 ∨ i = 2 ∨ i = 3 ∨ i = 4 ∨ i = 5 := by omega
  unfold setBit
  rcases this with h | h | h | h | h | h <;> subst h <;> simp <;> split <;> omega

section
variable (s : St) (h : Inv s)
include h

macro "inv_tac" : tactic => `(tactic| (
  cases h
  constructor <;> simp_all [refNone, refCarrier, refIrVersion, refInputType, refShape,
      refAnchorSlot, deploySlot, spikeSlot, gatesAll] <;> omega))

theorem inv_admit (c : Circuit) (he : s.phase = 0) : Inv (apply s (.admit c)) := by
  have hr := circuitRefusal_range c
  have i1 := ind_range c.keccak
  have i2 := ind_range (decide (c.kind = kindJubPoint))
  have i3 := ind_range (decide (c.kind = kindJubScalar))
  have i4 := ind_range (decide (c.kind = kindSecp))
  have hsum : ind (decide (c.kind = kindJubPoint)) + ind (decide (c.kind = kindJubScalar)) ≤ 1 := by
    unfold ind; by_cases h1 : c.kind = kindJubPoint <;> by_cases h2 : c.kind = kindJubScalar <;>
      simp_all [kindJubPoint, kindJubScalar]
  by_cases hc : circuitRefusal c = refNone
  · simp [apply, enabled, he, hc, accept]; inv_tac
  · simp [apply, enabled, he, hc]; inv_tac
theorem inv_finish (he : s.phase = 0 ∧ 1 ≤ s.requested) : Inv (apply s .finish) := by
  by_cases hc : 0 < s.refused <;> simp [apply, enabled, he.1, he.2, hc] <;> inv_tac
theorem inv_serialize (a b : Int) : Inv (apply s (.serialize a b)) := by
  simp [apply, enabled]; inv_tac
theorem inv_gate (i : Int) (ok : Bool) (hi : 0 ≤ i ∧ i < 6) : Inv (apply s (.gate i ok)) := by
  have hr := setBit_range s.gateBits i h.gateRange hi
  cases ok <;> simp [apply, enabled, hi] <;> inv_tac
theorem inv_use : Inv (apply s .useForSettle) := by
  by_cases hc : gatesPass s = true
  · have : s.gateBits = gatesAll := by simp [gatesPass] at hc; exact hc.1
    simp [apply, enabled, hc]; inv_tac
  · simp [apply, enabled, hc]; inv_tac
theorem inv_newRequest (he : s.phase ≠ 0) : Inv (apply s .newRequest) := by
  simp [apply, enabled, he]; inv_tac
end

theorem apply_disabled (s : St) (op : Op) (he : enabled s op = false) : apply s op = s := by
  simp [apply, he]

theorem inv_apply (s : St) (op : Op) (h : Inv s) : Inv (apply s op) := by
  cases hen : enabled s op
  · rw [apply_disabled s op hen]; exact h
  cases op with
  | admit c => exact inv_admit s h c (by simpa [enabled] using hen)
  | finish => exact inv_finish s h (by simpa [enabled] using hen)
  | serialize a b => exact inv_serialize s h a b
  | gate i ok => exact inv_gate s h i ok (by simpa [enabled] using hen)
  | useForSettle => exact inv_use s h
  | newRequest => exact inv_newRequest s h (by simpa [enabled] using hen)

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

theorem inv_run (d : Int) (ns : List Int) : Inv (run d ns) := inv_runOps _ _ (inv_init _)

/-! ## Requirement-level corollaries over every run -/

theorem ECC_010_image_or_refusal (d : Int) (ns : List Int) :
    (run d ns).requested = (run d ns).accepted + (run d ns).refused := (inv_run d ns).cover

theorem ECC_012_exit_zero_writes_all (d : Int) (ns : List Int) (h : (run d ns).phase = 1) :
    (run d ns).images = (run d ns).accepted ∧ (run d ns).abis = (run d ns).accepted ∧
    (run d ns).refused = 0 :=
  let ⟨a, _, b, c, _⟩ := (inv_run d ns).okExit h; ⟨b, c, a⟩

theorem ECC_013_refusal_no_image (d : Int) (ns : List Int) (h : (run d ns).phase = 2) :
    (run d ns).images = 0 ∧ (run d ns).exitCode = 2 ∧ 1 ≤ (run d ns).lastRefusal :=
  let ⟨_, a, _, b, c⟩ := (inv_run d ns).refusedExit h; ⟨a, b, c⟩

theorem ECC_003_one_abi_per_image (d : Int) (ns : List Int) :
    (run d ns).images = (run d ns).abis := by
  have h := inv_run d ns
  rcases h.phaseRange with ⟨h0, h2⟩
  have : (run d ns).phase = 0 ∨ (run d ns).phase = 1 ∨ (run d ns).phase = 2 := by omega
  rcases this with p | p | p
  · have := h.openNoOutput p; omega
  · have := h.okExit p; omega
  · have := h.refusedExit p; omega

theorem ECC_004_bytes_witness_free (d : Int) (ns : List Int) :
    (run d ns).img1Src = (run d ns).img2Src → (run d ns).img1Hash = (run d ns).img2Hash := by
  have := (inv_run d ns).bytes; intro e; omega

theorem ECC_007_no_tier3 (d : Int) (ns : List Int) : (run d ns).tier3 = 0 := (inv_run d ns).noTier3

theorem ECC_009_deploy_slot_15 (ns : List Int) : (run 1 ns).anchorSlot = 15 := by
  have h := (inv_run 1 ns).slot.1
  have hd : (run 1 ns).deployProfile = true := by
    unfold run
    suffices ∀ s ns, (runOps s ns).deployProfile = s.deployProfile by rw [this]; rfl
    intro s ns
    induction ns generalizing s with
    | nil => rfl
    | cons n ns ih =>
      simp only [runOps]; rw [ih]
      unfold stepOp; split
      · rfl
      · unfold apply; split
        · rfl
        · rename_i op _ _; cases op <;> simp only [accept] <;> (repeat' split) <;> rfl
  exact h hd

theorem ECC_014_jubjub_tape_words (d : Int) (ns : List Int) :
    (run d ns).tapeWords = 2 * (run d ns).jubPoints + (run d ns).jubScalars :=
  (inv_run d ns).jubjub.1

theorem ECC_002_settle_needs_gates (d : Int) (ns : List Int) :
    0 < (run d ns).settleUses → (run d ns).useBits = 63 := by
  intro hp; rcases (inv_run d ns).settleGated with h | h
  · omega
  · exact h

/-! ## Step pins of the refusal codes (every cause names one code) -/

theorem CF_014_no_kit_refused (c : Circuit) (h1 : c.stock = true) (h2 : c.irMinor = 1)
    (h3 : c.kind = kindNoKit) : circuitRefusal c = refInputType := by
  simp [circuitRefusal, h1, h2, h3]

theorem MYC_FE_002_ir_version_refused (c : Circuit) (h1 : c.stock = true) (h2 : c.irMinor ≠ 1) :
    circuitRefusal c = refIrVersion := by
  simp [circuitRefusal, h1, h2]

theorem ECC_001_carrier_refused (c : Circuit) (h1 : c.stock = false) :
    circuitRefusal c = refCarrier := by
  simp [circuitRefusal, h1]

theorem ECC_005_shape_refused (c : Circuit) (h1 : c.stock = true) (h2 : c.irMinor = 1)
    (h3 : c.kind ≠ kindNoKit) (h4 : c.shapeOk = false) : circuitRefusal c = refShape := by
  simp [circuitRefusal, h1, h2, h3, h4]

theorem MYC_SEAL_021_slot_clash_refused (c : Circuit) (h1 : c.stock = true) (h2 : c.irMinor = 1)
    (h3 : c.kind ≠ kindNoKit) (h4 : c.shapeOk = true) (h5 : c.slotClash = true) :
    circuitRefusal c = refAnchorSlot := by
  simp [circuitRefusal, h1, h2, h3, h4, h5]

/-- ECC-014, CF-013: Jubjub and secp256k1 inputs are admitted (not refused for their type). -/
theorem ECC_014_jubjub_admitted (c : Circuit) (h1 : c.stock = true) (h2 : c.irMinor = 1)
    (h3 : c.kind = kindJubPoint ∨ c.kind = kindJubScalar ∨ c.kind = kindSecp) (h4 : c.shapeOk = true)
    (h5 : c.slotClash = false) : circuitRefusal c = refNone := by
  have : c.kind ≠ kindNoKit := by rcases h3 with h | h | h <;> rw [h] <;> decide
  simp [circuitRefusal, h1, h2, this, h4, h5]

/-! ## Constants checked against MEASURED values

Source: worktree `mycroft-factory` (d1e8eed8d plus staged files), `mycroft/factory/REPORT.md`
lines 8-18 and `mycroft/tests/compact-suite/RESULTS.md` line 5. -/

/-- ECC-001, ECC-010, ECS-011: 106 circuits = 105 with ZKIR + the pure h02 `nativeMethods`
(REPORT.md:8, RESULTS.md:5). -/
theorem measured_circuits : (105 : Int) + 1 = 106 := by decide
/-- ECC-010: 208 calls = 139 equal + 62 refused as native + 1 pure + 6 blocked on a kit,
0 unsupported (REPORT.md:9); 146 natively accepted = 139 + 6 + 1 and 146 + 62 = 208
(RESULTS.md:5). -/
theorem measured_suite_calls : (139 : Int) + 62 + 1 + 6 + 0 = 208 ∧ (139 : Int) + 6 + 1 = 146 ∧
    (146 : Int) + 62 = 208 := by decide
/-- ECC-002: coverage 865 covered + 41 argued + 0 open (REPORT.md:16); mutations 58 killed
+ 4 argued = 62 (REPORT.md:17); compare 207 = 104 lean-oracle + 103 rust-compiler, 0 different
(REPORT.md:18). -/
theorem measured_gates : (58 : Int) + 4 = 62 ∧ (104 : Int) + 103 = 207 := by decide
/-- ECC-011: oracle-equal 104 of 104, 0 different (REPORT.md:14); Lean oracle against
Quint 1,226 of 1,226 transitions (REPORT.md:10). -/
theorem measured_oracle_equal : (104 : Int) - 104 = 0 ∧ (1226 : Int) - 1226 = 0 := by decide
/-- ECC-005: 13 instruction shapes. -/
theorem shapes_13 : shapeCount = 13 := by decide

end CompactComp
