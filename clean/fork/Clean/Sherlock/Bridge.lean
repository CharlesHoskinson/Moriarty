import Clean.Sherlock.FrLog

/-!
Soundness of the Sherlock bridge candidate (`research/sherlock-stark-bls-bridge-20261111/
CANDIDATE.md`, section 11): a ledger-accepted wrap proof implies the run's claim.

The objects:

* `KeyL`: the frozen key of the last STARK layer `L`, as a list of verifier checks (BabyBear
  equalities over the constraint DAG, Fr-sponge hashes of packed BabyBear values, squeezed
  challenge digits, Merkle openings) and the order of its transcript events.
* `VerifyL`: the reference verifier of `L` (BabyBear arithmetic, hash `frSponge ∘ packChunks`).
* `WrapRel`: the constraint semantics of the wrap `W`: the same checks compiled into Fr cells with
  lazy arithmetic and bounded quotients, a log table of exactly `B` entries with the canonical
  check on every value, `pv = encode T`, and native replay of every entry. Its public input is
  the statement only.
* `Premises`: the named assumptions A1 to A7 as fields (no `axiom`), documented in
  `BridgeAxioms.lean`.

`bridge_sound` composes them. Every soundness point of the design is a clause of a definition
that the proof uses: the bounded quotient (`ArithGadget`, `arith_refines`), packing with checked
length (`packed_hash_inj`, used to discharge A2's hypothesis), the canonical check
(`CanonGadget`, `canon_check_sound`), `pv` before the first challenge (`pvFirst`, A2's other
hypothesis), the fixed capacity (`WrapRel`'s `T.length = B`, `LogTree.Carrier`, `pad`) with the
overflow to STARK shards (`RunWith`, `runWith_all_valid`), and the ZK boundary (A7).
-/

set_option linter.unusedSectionVars false

namespace Clean.Sherlock.Bridge

/-! ## The verifier of `L` -/

/-- One check of the reference verifier of `L`. Indices address the environment
`statement ++ pv ++ proof`. -/
inductive Check where
  /-- BabyBear equality of two expressions (constraint DAG, sumcheck, folding, `eq` binding). -/
  | arith (l r : Expr)
  /-- `out = frSponge (packChunks ins)`: leaf hashes and transcript absorbs of BabyBear values. -/
  | hash (ins : List Nat) (out : Nat)
  /-- The 5 base-`p` digits of a squeezed Fr value. -/
  | squeeze (src : Nat) (dsts : List Nat)
  /-- A Merkle opening over Midnight's two-input hash. -/
  | merkle (leaf : Nat) (bits : List Nat) (sibs : List Nat) (root : Nat)

/-- Transcript events of `L`'s Fiat-Shamir transcript. -/
inductive TEv where
  | commit
  | absorbPv
  | challenge
  deriving DecidableEq

/-- `pv` is absorbed before the first challenge is drawn. -/
def pvFirst : List TEv → Bool
  | [] => false
  | .absorbPv :: _ => true
  | .challenge :: _ => false
  | .commit :: t => pvFirst t

/-- The frozen key of `L` (universal: one for every program). -/
structure KeyL where
  checks : List Check
  sched : List TEv

/-- The reference semantics of one check. -/
def Check.Ref (X : Prims) (env : List Nat) : Check → Prop
  | .arith l r => (∀ i ∈ l.vars ++ r.vars, env.getD i 0 < pBB) ∧ l.evalP env = r.evalP env
  | .hash ins out => (∀ i ∈ ins, env.getD i 0 < pBB) ∧
      env.getD out 0 = X.frSponge (packChunks (ins.map (env.getD · 0)))
  | .squeeze src dsts => dsts.map (env.getD · 0) = digitsOf pBB 5 (env.getD src 0)
  | .merkle leaf bits sibs root =>
      merkleRoot X (env.getD leaf 0) (bits.map (fun i => env.getD i 0 == 1))
        (sibs.map (env.getD · 0)) = env.getD root 0

/-- The wrap's gadget for one check (Fr cells). -/
def Check.Gadget (X : Prims) (env : List Nat) : Check → Prop
  | .arith l r => ArithGadget env l r
  | .hash ins out => (∀ i ∈ ins, env.getD i 0 < pBB) ∧
      env.getD out 0 = X.frSponge (packChunksR (ins.map (env.getD · 0)))
  | .squeeze src dsts => SqueezeGadget (env.getD src 0) (dsts.map (env.getD · 0))
  | .merkle leaf bits sibs root =>
      merkleRoot X (env.getD leaf 0) (bits.map (fun i => env.getD i 0 == 1))
        (sibs.map (env.getD · 0)) = env.getD root 0

/-- Static facts of a check of the frozen key (decidable on the concrete key). -/
def Check.StaticOK : Check → Prop
  | .arith l r => ArithOK l r
  | _ => True

/-- Static facts of the frozen key: `pv` first, and every bound below `r`. -/
def KeyOK (vk : KeyL) : Prop := pvFirst vk.sched = true ∧ ∀ c ∈ vk.checks, c.StaticOK

/-- The reference verifier of `L` on statement `s`, public values `pv` and proof `π`. -/
def VerifyL (X : Prims) (vk : KeyL) (s pv π : List Nat) : Prop :=
  ∀ c ∈ vk.checks, c.Ref X (s ++ pv ++ π)

theorem getD_lt {env : List Nat} (h : ∀ x ∈ env, x < rFr) (i : Nat) : env.getD i 0 < rFr := by
  rw [List.getD_eq_getElem?_getD]
  cases hi : env[i]? with
  | none => simp only [Option.getD_none]; decide
  | some a => simpa using h a (List.mem_iff_getElem?.mpr ⟨i, hi⟩)

/-- Lemmas 10 to 13 (`sponge_refines`, `merkle_path_refines`, the `arith` instances): every
compiled check refines its reference check. -/
theorem check_refines {X : Prims} {env : List Nat} {c : Check} (hcells : ∀ x ∈ env, x < rFr)
    (hok : c.StaticOK) (h : c.Gadget X env) : c.Ref X env := by
  cases c with
  | arith l r => exact arith_refines hok h
  | hash ins out =>
    simp only [Check.Gadget] at h
    simp only [Check.Ref]
    exact ⟨h.1, by rw [h.2, packChunksR_eq]⟩
  | squeeze src dsts => exact squeeze_digits_sound (getD_lt hcells src) h
  | merkle leaf bits sibs root => exact h

/-! ## The wrap -/

/-- The private witness of the wrap: the public values of `L`, the proof of `L` and the log
table. -/
structure WrapWit where
  pv : List Nat
  π : List Nat
  T : List Entry

/-- The wrap's gadget on one table slot: range-checked tag and parameter, the arity of the
operation, and the canonical check on every value. -/
def EntryGadget (e : Entry) : Prop :=
  (∀ x ∈ e.op.code, x < pBB) ∧ e.vals.length = e.op.arity ∧ ∀ v ∈ e.vals, CanonGadget v

/-- The constraint semantics of the wrap with capacity `B`. Its public input `pub` is the
statement only; everything else is private witness. -/
def WrapRel (X : Prims) (B : Nat) (vk : KeyL) (pub : List Nat) (w : WrapWit) : Prop :=
  (∀ x ∈ pub ++ w.pv ++ w.π, x < rFr) ∧
  (∀ c ∈ vk.checks, c.Gadget X (pub ++ w.pv ++ w.π)) ∧
  w.T.length = B ∧
  (∀ e ∈ w.T, EntryGadget e) ∧
  w.pv = encode w.T ∧
  (∀ e ∈ w.T, ReplayGadget X pub e)

/-- Lemma 16 (`wrap_refines`): an accepted wrap witness is an accepting reference verification of
`L` on the statement, with a table of exactly `B` well-formed, valid entries encoded as `pv`. -/
theorem wrap_refines {X : Prims} {B : Nat} {vk : KeyL} {pub : List Nat} {w : WrapWit}
    (hok : KeyOK vk) (h : WrapRel X B vk pub w) :
    VerifyL X vk pub w.pv w.π ∧ w.T.length = B ∧ (∀ e ∈ w.T, e.WF ∧ Valid X pub e) ∧
      w.pv = encode w.T := by
  obtain ⟨hcells, hchk, hlen, hent, hpv, hrep⟩ := h
  refine ⟨fun c hc => check_refines hcells (hok.2 c hc) (hchk c hc), hlen, ?_, hpv⟩
  intro e he
  obtain ⟨h1, h2, h3⟩ := hent e he
  have hc : ∀ v ∈ e.vals, Canon v := fun v hv => canon_check_sound (h3 v hv)
  exact ⟨⟨h1, h2, hc⟩, replay_sound hc (hrep e he)⟩

/-! ## Hash premises in carrier-guarded form -/

/-- Carrier-guarded injectivity of Midnight's Poseidon sponge over Fr (premise A2b). -/
def FrInj (X : Prims) (FrDom : List Nat → Prop) : Prop :=
  ∀ a b, FrDom a → FrDom b → X.frSponge a = X.frSponge b → a = b

/-- The hash that `L`'s Merkle trees and transcript use, `frSponge ∘ packChunks`, is injective on
equal-length vectors of 31-bit words (the hypothesis A2 rests on). -/
def PackedHashInj (X : Prims) (FrDom : List Nat → Prop) : Prop :=
  ∀ a b : List Nat, a.length = b.length → (∀ x ∈ a, x < packBase) → (∀ x ∈ b, x < packBase) →
    FrDom (packChunks a) → FrDom (packChunks b) →
    X.frSponge (packChunks a) = X.frSponge (packChunks b) → a = b

/-- Packing with checked length and range turns the Fr hash premise into injectivity of the
packed hash. -/
theorem packed_hash_inj {X : Prims} {FrDom : List Nat → Prop} (h : FrInj X FrDom) :
    PackedHashInj X FrDom := by
  intro a b hl ha hb hda hdb he
  exact packChunks_inj a.length (Nat.le_refl _) hl ha hb (h _ _ hda hdb he)

/-! ## The premises -/

/-- The named premises. Each field is a stated assumption, not an axiom. -/
structure Premises where
  /-- The fixed log capacity of the universal wrap key. -/
  B : Nat
  /-- The primitives (data). -/
  X : Prims
  /-- The frozen key of `L`. -/
  vkL : KeyL
  /-- Carrier domain of the small-field hash premise (inputs of Poseidon2 over BabyBear). -/
  P2Dom : List Nat → Prop
  /-- Carrier domain of the Fr hash premise (inputs of Midnight Poseidon over Fr). -/
  FrDom : List Nat → Prop
  LedgerProof : Type
  /-- Ledger 9 `VerifierKey::verify` accepts the proof for public input `pub`. -/
  LedgerAccepts : List Nat → LedgerProof → Prop
  /-- `L`'s root AIR accepts with public values `pv`, log digest `d` and statement `s`. -/
  RootAccepts : List Nat → List Nat → List Nat → Prop
  /-- The recursion below `L` (leaves, discharge shards, nodes) accepts digest `d` for `s`. -/
  RecAccepts : List Nat → List Nat → Prop
  /-- The leaves' relation over the run's whole sequence of Fr operations (oracle answers). -/
  RunOps : List Entry → List Nat → Prop
  /-- The reference machine: the claim `s` holds. -/
  Run : List Nat → Prop
  /-- Static facts of the frozen key of `L` (decided on the concrete key). -/
  vkL_ok : KeyOK vkL
  /-- (A1) Knowledge soundness of midnight-proofs PLONK with KZG (AGM, q-type assumption). -/
  wrapKnowledgeSound : ∀ pub prf, LedgerAccepts pub prf → ∃ w, WrapRel X B vkL pub w
  /-- (A2) Soundness of `L` (WHIR round-by-round soundness in the list-decoding regime,
  Fiat-Shamir in the random-oracle model, `eq` binding of `pv` with error `c m / |EF|`, squeeze
  bias), for the packed Fr hash and a transcript that absorbs `pv` before any challenge. -/
  lastLayerSound : PackedHashInj X FrDom → pvFirst vkL.sched = true →
    ∀ s pv π, VerifyL X vkL s pv π → ∃ d, RootAccepts pv d s
  /-- (A2b) Collision resistance of Midnight Poseidon over Fr, carrier-guarded. -/
  frSpongeInj : FrInj X FrDom
  /-- (A3) The root AIR binds `pv` to a carrier log tree with that digest, padded to `B`. -/
  rootBinds : ∀ pv d s, RootAccepts pv d s → RecAccepts d s ∧
    ∃ t : LogTree, t.Carrier X P2Dom B ∧ encode (pad B t.flatten) = pv ∧ t.digest X = d
  /-- (A4) Recursion soundness of leaves, discharge shards and nodes. -/
  recSound : ∀ d s, RecAccepts d s → ∃ t : LogTree, t.Carrier X P2Dom B ∧ t.digest X = d ∧
    RunWith X RunOps t.flatten s
  /-- (A5) Collision resistance of Poseidon2 over BabyBear (the `sherlock-value/2` digest and the
  log onion), carrier-guarded. -/
  p2Inj : P2Inj X P2Dom
  /-- (A6) Oracle elimination: a run whose Fr operations are all valid is a run of the machine. -/
  oracleElim : ∀ ops s, RunOps ops s → (∀ e ∈ ops, Valid X s e) → Run s
  /-- Randomness of the wrap prover. -/
  Coins : Type
  /-- The blinded wrap prover and its simulator. -/
  prove : List Nat → WrapWit → Coins → LedgerProof
  sim : List Nat → Coins → LedgerProof
  /-- Indistinguishability of proof distributions (an equivalence). -/
  Indist : (Coins → LedgerProof) → (Coins → LedgerProof) → Prop
  indist_symm : ∀ a b, Indist a b → Indist b a
  indist_trans : ∀ a b c, Indist a b → Indist b c → Indist a c
  /-- (A7) Zero knowledge of the blinded wrap: for a true statement the proof distribution is
  simulated from the public input alone. -/
  wrapZK : ∀ pub w, WrapRel X B vkL pub w → Indist (prove pub w) (sim pub)

/-! ## The main theorem -/

/-- Lemma 22 (`bridge_sound`): a proof that ledger 9 accepts for public input `pub` implies the
claim `pub` of the reference machine. -/
theorem bridge_sound (P : Premises) {pub : List Nat} {prf : P.LedgerProof}
    (h : P.LedgerAccepts pub prf) : P.Run pub := by
  obtain ⟨w, hw⟩ := P.wrapKnowledgeSound pub prf h
  obtain ⟨hV, hTlen, hTok, hpv⟩ := wrap_refines P.vkL_ok hw
  obtain ⟨d, hroot⟩ :=
    P.lastLayerSound (packed_hash_inj P.frSpongeInj) P.vkL_ok.1 pub w.pv w.π hV
  obtain ⟨hrec, t, ⟨htlen, htwf, htin⟩, henc, hdig⟩ := P.rootBinds _ _ _ hroot
  have hpad : pad P.B t.flatten = w.T :=
    encode_inj (by rw [pad_length htlen, hTlen]) (pad_wf htwf) (fun e he => (hTok e he).1)
      (henc.trans hpv)
  obtain ⟨t', ⟨_, htwf', htin'⟩, hdig', hrun⟩ := P.recSound d pub hrec
  have htt : t' = t := digest_inj P.p2Inj htwf' htwf htin' htin (hdig'.trans hdig.symm)
  rw [htt] at hrun
  obtain ⟨ops, hops, hval⟩ := runWith_all_valid hrun (by
    intro e he
    have : e ∈ w.T := by rw [← hpad]; exact List.mem_append_left _ he
    exact (hTok e this).2)
  exact P.oracleElim ops pub hops hval

/-! ## Zero-knowledge boundary -/

/-- The ledger's view depends on the claim only: two wrap witnesses for the same public input
(different logs, used-entry counts, deferred-or-emulated splits, proofs of `L`) give
indistinguishable proofs. -/
theorem wrap_view_only_claim (P : Premises) {pub : List Nat} {w₁ w₂ : WrapWit}
    (h₁ : WrapRel P.X P.B P.vkL pub w₁) (h₂ : WrapRel P.X P.B P.vkL pub w₂) :
    P.Indist (P.prove pub w₁) (P.prove pub w₂) :=
  P.indist_trans _ _ _ (P.wrapZK pub w₁ h₁) (P.indist_symm _ _ (P.wrapZK pub w₂ h₂))

/-! ## Supporting lemmas off the main path -/

/-- Lemma 10 (`merkle_binding_or_collision`): two openings along the same path with different
leaves and the same root give an explicit collision of the two-input hash. -/
theorem merkle_binding_or_collision (X : Prims) : ∀ (bits : List Bool) (l₁ l₂ : Nat)
    (s₁ s₂ : List Nat), s₁.length = bits.length → s₂.length = bits.length → l₁ ≠ l₂ →
    merkleRoot X l₁ bits s₁ = merkleRoot X l₂ bits s₂ →
    ∃ u v : List Nat, u ≠ v ∧ u.length = 2 ∧ v.length = 2 ∧ X.frSponge u = X.frSponge v
  | [], _, _, _, _, _, _, hne, h => (hne h).elim
  | _ :: _, _, _, [], _, h1, _, _, _ => by simp at h1
  | _ :: _, _, _, _ :: _, [], _, h2, _, _ => by simp at h2
  | b :: bs, l₁, l₂, x :: xs, y :: ys, h1, h2, hne, h => by
    simp only [merkleRoot] at h
    by_cases heq : (if b then X.frH2 x l₁ else X.frH2 l₁ x) = (if b then X.frH2 y l₂ else X.frH2 l₂ y)
    · cases b with
      | true => exact ⟨[x, l₁], [y, l₂], by simp [hne], rfl, rfl, by simpa [Prims.frH2] using heq⟩
      | false => exact ⟨[l₁, x], [l₂, y], by simp [hne], rfl, rfl, by simpa [Prims.frH2] using heq⟩
    · exact merkle_binding_or_collision X bs _ _ xs ys (by simpa using h1) (by simpa using h2)
        heq h

/-- The public form of a `sherlock-value/2` digest binds the internal small-field digest:
`PoseidonFr(tag, pack d2)` determines `d2` (8 BabyBear words), by A2b and `pack_injective`. -/
theorem public_digest_binds {X : Prims} {FrDom : List Nat → Prop} (h : FrInj X FrDom)
    {tag : Nat} {a b : List Nat} (ha : a.length = 8) (hb : b.length = 8)
    (hra : ∀ x ∈ a, x < packBase) (hrb : ∀ x ∈ b, x < packBase)
    (hda : FrDom [tag, pack a]) (hdb : FrDom [tag, pack b])
    (he : X.frSponge [tag, pack a] = X.frSponge [tag, pack b]) : a = b := by
  have := h _ _ hda hdb he
  simp only [List.cons.injEq, true_and, and_true] at this
  exact pack_injective (ha.trans hb.symm) (by omega) hra hrb this

/-- The preamble of stark-backend's verifier (`verifier/mod.rs:128-175`): commitments, then the
public values, then the first challenge. -/
def lPreamble : List TEv := [.commit, .commit, .absorbPv, .challenge, .commit, .challenge]

theorem lPreamble_pvFirst : pvFirst lPreamble = true := rfl

/-- A transcript that draws a challenge before absorbing `pv` fails the key check. -/
theorem late_pv_rejected : pvFirst [.commit, .challenge, .absorbPv] = false := rfl

end Clean.Sherlock.Bridge
