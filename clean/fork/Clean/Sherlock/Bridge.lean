import Clean.Sherlock.FrLog

/-!
Soundness of the Sherlock bridge candidate (`research/sherlock-stark-bls-bridge-20261111/
CANDIDATE.md`, section 11, with the owner decisions and lead amendments at its end): a
ledger-accepted wrap proof implies the run's claim.

The objects:

* `KeyL`: the frozen key of the last STARK layer `L`: the verifier checks (BabyBear equalities
  over the constraint DAG, Fr-sponge hashes of packed BabyBear values, Merkle openings), the
  Fiat-Shamir transcript program (`TOp`: absorbs, challenge squeezes, proof-of-work checks), the
  fixed statement length, the fixed `pv` length, and the analysis parameters of WHIR.
* `VerifyL`: the reference verifier of `L` (BabyBear arithmetic, hash `frSponge ∘ packChunks`,
  the transcript run from the empty state).
* `WrapRel`: the constraint semantics of the wrap `W`: the same checks and transcript compiled
  into Fr cells (lazy arithmetic with bounded quotients, packing gadget, digit gadget with the
  canonical check), a log table of exactly `B` entries with the canonical check on every value,
  `pv = pvOf W T` of the fixed length `W`, and native replay of every entry. Its public input is
  the statement only.
* `Premises`: the named assumptions as fields (no `axiom`), documented in `BridgeAxioms.lean`.

Two static key facts gate premise A2 and are computed from the key's own transcript program:
`pvFirstT` (every word of `pv` is absorbed before the first challenge or proof of work) and the
proof-of-work part of `ProvenRegime`. The WHIR analysis parameters (`logBlowup`, `eta`, `m`)
stay declared metadata; `WhirParams` says why.
-/

set_option linter.unusedSectionVars false

namespace Clean.Sherlock.Bridge

/-! ## The verifier of `L` -/

/-- One check of the reference verifier of `L`. Indices address the environment
`statement ++ pv ++ proof`. -/
inductive Check where
  /-- BabyBear equality of two expressions (constraint DAG, sumcheck, folding, `eq` binding). -/
  | arith (l r : Expr)
  /-- `out = frSponge (packChunks ins)`: leaf hashes of BabyBear values. -/
  | hash (ins : List Nat) (out : Nat)
  /-- A Merkle opening over Midnight's two-input hash. -/
  | merkle (leaf : Nat) (bits : List Nat) (sibs : List Nat) (root : Nat)

/-- The three proof-of-work checks of WHIR. -/
inductive PowKind where
  | fold
  | mu
  | query
  deriving DecidableEq

/-- One operation of the Fiat-Shamir transcript of `L` (a duplex over Midnight Poseidon on Fr). -/
inductive TOp where
  /-- Absorb BabyBear words (public values, opened values), packed 8 per Fr element. -/
  | absorbBB (vars : List Nat)
  /-- Absorb Fr elements (Merkle roots). -/
  | absorbFr (vars : List Nat)
  /-- Draw a challenge: the 5 base-`p` digits of the sponge output. -/
  | squeeze (dsts : List Nat)
  /-- A proof-of-work check: the sponge output after absorbing the nonce has `bits` low zero
  bits. -/
  | pow (nonce : Nat) (bits : Nat) (kind : PowKind)

/-- The reference transcript run from state `st` (all Fr elements absorbed so far). -/
def runRef (X : Prims) (env : List Nat) : List Nat → List TOp → Prop
  | _, [] => True
  | st, .absorbBB vs :: ops => (∀ i ∈ vs, env.getD i 0 < pBB) ∧
      runRef X env (st ++ packChunks (vs.map (env.getD · 0))) ops
  | st, .absorbFr vs :: ops => runRef X env (st ++ vs.map (env.getD · 0)) ops
  | st, .squeeze ds :: ops => ds.map (env.getD · 0) = digitsOf pBB 5 (X.frSponge st) ∧
      runRef X env (st ++ [X.frSponge st]) ops
  | st, .pow n b _ :: ops => X.frSponge (st ++ [env.getD n 0]) % 2 ^ b = 0 ∧
      runRef X env (st ++ [env.getD n 0]) ops

/-- The wrap's transcript gadgets: the packing gadget, the digit gadget with the canonical check,
and a bit decomposition for the proof of work. -/
def runGadget (X : Prims) (env : List Nat) : List Nat → List TOp → Prop
  | _, [] => True
  | st, .absorbBB vs :: ops => (∀ i ∈ vs, env.getD i 0 < pBB) ∧
      runGadget X env (st ++ packChunksR (vs.map (env.getD · 0))) ops
  | st, .absorbFr vs :: ops => runGadget X env (st ++ vs.map (env.getD · 0)) ops
  | st, .squeeze ds :: ops => SqueezeGadget (X.frSponge st) (ds.map (env.getD · 0)) ∧
      runGadget X env (st ++ [X.frSponge st]) ops
  | st, .pow n b _ :: ops => (∃ hi, X.frSponge (st ++ [env.getD n 0]) = hi * 2 ^ b) ∧
      runGadget X env (st ++ [env.getD n 0]) ops

/-- The compiled transcript refines the reference transcript. -/
theorem run_refines (X : Prims) (env : List Nat) : ∀ (st : List Nat) (ops : List TOp),
    runGadget X env st ops → runRef X env st ops
  | _, [], _ => trivial
  | st, .absorbBB vs :: ops, h => by
    simp only [runGadget, packChunksR_eq] at h
    exact ⟨h.1, run_refines X env _ ops h.2⟩
  | st, .absorbFr vs :: ops, h => run_refines X env _ ops h
  | st, .squeeze ds :: ops, h =>
    ⟨squeeze_digits_sound (X.frSponge_lt st) h.1, run_refines X env _ ops h.2⟩
  | st, .pow n b _ :: ops, h => by
    obtain ⟨⟨hi, hh⟩, h2⟩ := h
    exact ⟨by rw [hh, Nat.mul_mod_left], run_refines X env _ ops h2⟩

/-- The variables absorbed before the first challenge or proof of work. -/
def absorbedBeforeChallenge : List TOp → List Nat
  | [] => []
  | .absorbBB vs :: ops => vs ++ absorbedBeforeChallenge ops
  | .absorbFr vs :: ops => vs ++ absorbedBeforeChallenge ops
  | .squeeze _ :: _ => []
  | .pow _ _ _ :: _ => []

/-- `pv` (environment indices `lo, ..., lo + n - 1`) is absorbed before the first challenge.
Computed from the transcript program itself. -/
def pvFirstT (lo n : Nat) (ops : List TOp) : Bool :=
  (List.range' lo n).all (fun i => (absorbedBeforeChallenge ops).contains i)

/-- The proof-of-work bits of one kind in the transcript program. -/
def powList (k : PowKind) : List TOp → List Nat
  | [] => []
  | .pow _ b k' :: ops => if k' = k then b :: powList k ops else powList k ops
  | _ :: ops => powList k ops

/-- The weakest proof of work of a kind (`0` if the transcript has none). -/
def powMin : List Nat → Nat
  | [] => 0
  | b :: bs => bs.foldl min b

/-- The WHIR analysis parameters of `L`. They are not verifier operations: `mult` (the
Guruswami-Sudan multiplicity) and `eta` (the slack below the Johnson radius) occur only in the
soundness proof, and `logBlowup` occurs in the verifier only through the domain constants of the
`arith` checks, which this model does not tie to a code. So they stay declared metadata of the
key, checked against Theorem 1.5 by `thm15Mult`. -/
structure WhirParams where
  /-- `log2` of the inverse rate (`rho = 2^-logBlowup`); even, so `sqrt rho = 2^-(logBlowup/2)`. -/
  logBlowup : Nat
  /-- Johnson-radius slack `eta = etaNum / etaDen` (`gamma = 1 - sqrt rho - eta`). -/
  etaNum : Nat
  etaDen : Nat
  /-- Guruswami-Sudan multiplicity used in the soundness bound. -/
  mult : Nat

/-- The multiplicity that BCHKS25 Theorem 1.5 sets as stated:
`m = max(ceil(sqrt(rho) / (2 eta)), 3)`. -/
def thm15Mult (w : WhirParams) : Nat :=
  max ((w.etaDen + 2 ^ (w.logBlowup / 2) * 2 * w.etaNum - 1) /
    (2 ^ (w.logBlowup / 2) * 2 * w.etaNum)) 3

/-- The frozen key of `L` (universal: one for every program). -/
structure KeyL where
  checks : List Check
  transcript : List TOp
  /-- Fixed statement length (words of the claim). -/
  nStmt : Nat
  /-- Fixed `pv` length `W` in BabyBear words (lead amendment 4). -/
  pvLen : Nat
  whir : WhirParams

/-- The proven regime of the owner decision (2026-11-11, option b of the paper review): the
Johnson-radius proximity-gap theorem applied as stated (`m` from `thm15Mult`, so `m >= 3`; the
unproved p.23 remark that allows `m = 1` is not used), and every folding and `mu` proof of work
of the transcript at least 21 bits, every query proof of work at least 20 bits. The proof-of-work
bits are read from the transcript program. -/
def ProvenRegime (vk : KeyL) : Bool :=
  vk.whir.logBlowup % 2 == 0 && vk.whir.etaNum != 0 && vk.whir.mult == thm15Mult vk.whir &&
    21 ≤ powMin (powList .fold vk.transcript) && 21 ≤ powMin (powList .mu vk.transcript) &&
    20 ≤ powMin (powList .query vk.transcript)

/-- The reference semantics of one check. -/
def Check.Ref (X : Prims) (env : List Nat) : Check → Prop
  | .arith l r => (∀ i ∈ l.vars ++ r.vars, env.getD i 0 < pBB) ∧ l.evalP env = r.evalP env
  | .hash ins out => (∀ i ∈ ins, env.getD i 0 < pBB) ∧
      env.getD out 0 = X.frSponge (packChunks (ins.map (env.getD · 0)))
  | .merkle leaf bits sibs root =>
      merkleRoot X (env.getD leaf 0) (bits.map (fun i => env.getD i 0 == 1))
        (sibs.map (env.getD · 0)) = env.getD root 0

/-- The wrap's gadget for one check (Fr cells). -/
def Check.Gadget (X : Prims) (env : List Nat) : Check → Prop
  | .arith l r => ArithGadget env l r
  | .hash ins out => (∀ i ∈ ins, env.getD i 0 < pBB) ∧
      env.getD out 0 = X.frSponge (packChunksR (ins.map (env.getD · 0)))
  | .merkle leaf bits sibs root =>
      merkleRoot X (env.getD leaf 0) (bits.map (fun i => env.getD i 0 == 1))
        (sibs.map (env.getD · 0)) = env.getD root 0

/-- Static facts of a check of the frozen key (decidable on the concrete key). -/
def Check.StaticOK : Check → Prop
  | .arith l r => ArithOK l r
  | _ => True

/-- Static facts of the frozen key: `pv` absorbed first (from the transcript), the proven WHIR
regime (proof of work from the transcript), and every bound below `r`. -/
def KeyOK (vk : KeyL) : Prop :=
  pvFirstT vk.nStmt vk.pvLen vk.transcript = true ∧ ProvenRegime vk = true ∧
    ∀ c ∈ vk.checks, c.StaticOK

/-- The reference verifier of `L` on statement `s`, public values `pv` and proof `π`. -/
def VerifyL (X : Prims) (vk : KeyL) (s pv π : List Nat) : Prop :=
  s.length = vk.nStmt ∧ pv.length = vk.pvLen ∧ (∀ c ∈ vk.checks, c.Ref X (s ++ pv ++ π)) ∧
    runRef X (s ++ pv ++ π) [] vk.transcript

/-- What `pvFirstT` means for an accepted verification: every word of `pv` sits at an
environment index that the transcript absorbs before its first challenge. -/
theorem pv_absorbed_before_challenge {X : Prims} {vk : KeyL} {s pv π : List Nat}
    (hk : pvFirstT vk.nStmt vk.pvLen vk.transcript = true) (hv : VerifyL X vk s pv π) :
    ∀ j < pv.length, vk.nStmt + j ∈ absorbedBeforeChallenge vk.transcript ∧
      (s ++ pv ++ π).getD (vk.nStmt + j) 0 = pv.getD j 0 := by
  intro j hj
  obtain ⟨hs, hp, _, _⟩ := hv
  refine ⟨?_, ?_⟩
  · have := List.all_eq_true.mp hk (vk.nStmt + j) (List.mem_range'_1.mpr ⟨by omega, by omega⟩)
    simpa using this
  · rw [← hs, List.getD_eq_getElem?_getD, List.getD_eq_getElem?_getD, List.append_assoc,
      List.getElem?_append_right (by omega), Nat.add_sub_cancel_left,
      List.getElem?_append_left hj]

theorem getD_lt {env : List Nat} (h : ∀ x ∈ env, x < rFr) (i : Nat) : env.getD i 0 < rFr := by
  rw [List.getD_eq_getElem?_getD]
  cases hi : env[i]? with
  | none => simp only [Option.getD_none]; decide
  | some a => simpa using h a (List.mem_iff_getElem?.mpr ⟨i, hi⟩)

/-- Lemmas 10 to 13 (`sponge_refines`, `merkle_path_refines`, the `arith` instances): every
compiled check refines its reference check. -/
theorem check_refines {X : Prims} {env : List Nat} {c : Check}
    (hok : c.StaticOK) (h : c.Gadget X env) : c.Ref X env := by
  cases c with
  | arith l r => exact arith_refines hok h
  | hash ins out =>
    simp only [Check.Gadget] at h
    simp only [Check.Ref]
    exact ⟨h.1, by rw [h.2, packChunksR_eq]⟩
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

/-- The constraint semantics of the wrap with capacity `B` entries and `pv` length
`vk.pvLen`. Its public input `pub` is the statement only; everything else is private witness. -/
def WrapRel (X : Prims) (B : Nat) (vk : KeyL) (pub : List Nat) (w : WrapWit) : Prop :=
  pub.length = vk.nStmt ∧
  (∀ x ∈ pub ++ w.pv ++ w.π, x < rFr) ∧
  (∀ c ∈ vk.checks, c.Gadget X (pub ++ w.pv ++ w.π)) ∧
  runGadget X (pub ++ w.pv ++ w.π) [] vk.transcript ∧
  w.T.length = B ∧
  (∀ e ∈ w.T, EntryGadget e) ∧
  (encode w.T).length ≤ vk.pvLen ∧
  w.pv = pvOf vk.pvLen w.T ∧
  (∀ e ∈ w.T, ReplayGadget X pub e)

/-- Lemma 16 (`wrap_refines`): an accepted wrap witness is an accepting reference verification of
`L` on the statement, with a table of exactly `B` well-formed, valid entries whose padded
encoding of the fixed length is `pv`. -/
theorem wrap_refines {X : Prims} {B : Nat} {vk : KeyL} {pub : List Nat} {w : WrapWit}
    (hok : KeyOK vk) (h : WrapRel X B vk pub w) :
    VerifyL X vk pub w.pv w.π ∧ w.T.length = B ∧ (∀ e ∈ w.T, e.WF ∧ Valid X pub e) ∧
      w.pv = pvOf vk.pvLen w.T := by
  obtain ⟨hpl, _, hchk, hrun, hlen, hent, henc, hpv, hrep⟩ := h
  refine ⟨⟨hpl, by rw [hpv]; exact pvOf_length henc,
    fun c hc => check_refines (hok.2.2 c hc) (hchk c hc), run_refines X _ [] _ hrun⟩,
    hlen, ?_, hpv⟩
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
  /-- (A2) Soundness of `L`. The proximity-gap part is the proven Johnson-radius statement
  (citations per `research/sherlock-design-paper-review-20261111/REVIEW.md`, not read by the
  Lean lane): BCHKS25 (ECCC TR25-169) Theorem 1.5 as stated, with `m = max(ceil(sqrt(rho)/(2 eta)), 3)`,
  Theorem 4.5 (batching) and Theorem 4.6 (mutual correlated agreement), in the parameters of
  `ProvenRegime` (100.2 bits folding, 102.3 bits `mu` batching at S4 with 21-bit PoW, per the
  paper review). The unproved remark on p.23 (`m < 3`) is not assumed. The rest: WHIR round-by-
  round soundness, Fiat-Shamir in the random-oracle model (non-interactive error
  `Q_RO 2^-100 + 3 (Q_RO^2 + 1) / 2^kappa`, SWIRL Cor 4.2.6), the `eq` binding of `pv`
  (`c m / |EF|`), the squeeze bias. Hypotheses: the packed Fr hash is injective, the transcript
  absorbs every word of `pv` before any challenge (`pvFirstT`, computed from the transcript),
  and the key is in the proven regime (proof of work read from the transcript). -/
  lastLayerSound : PackedHashInj X FrDom → pvFirstT vkL.nStmt vkL.pvLen vkL.transcript = true →
    ProvenRegime vkL = true →
    ∀ s pv π, VerifyL X vkL s pv π → ∃ d, RootAccepts pv d s
  /-- (A2b) Collision resistance of Midnight Poseidon over Fr, carrier-guarded. -/
  frSpongeInj : FrInj X FrDom
  /-- (A3) The root AIR binds `pv` to a carrier log tree with that digest, padded to `B`. -/
  rootBinds : ∀ pv d s, RootAccepts pv d s → RecAccepts d s ∧
    ∃ t : LogTree, t.Carrier X P2Dom B ∧ pvOf vkL.pvLen (pad B t.flatten) = pv ∧ t.digest X = d
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
    P.lastLayerSound (packed_hash_inj P.frSpongeInj) P.vkL_ok.1 P.vkL_ok.2.1 pub w.pv w.π hV
  obtain ⟨hrec, t, ⟨htlen, htwf, htin⟩, henc, hdig⟩ := P.rootBinds _ _ _ hroot
  have hpad : pad P.B t.flatten = w.T :=
    pvOf_inj (by rw [pad_length htlen, hTlen]) (pad_wf htwf) (fun e he => (hTok e he).1)
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

/-- A transcript skeleton of `L` in the order of stark-backend's verifier preamble
(`verifier/mod.rs:128-175`): commitments, then the public values, then challenges; WHIR rounds
with 21-bit folding and `mu` proof of work and 20-bit query proof of work. Indices: statement
`0..1`, `pv` `2..5`, proof from `6`. -/
def lTranscript (pfold pmu pquery : Nat) : List TOp :=
  [.absorbFr [6], .absorbBB [2, 3, 4, 5], .squeeze [7, 8, 9, 10, 11], .pow 12 pmu .mu,
   .absorbFr [13], .squeeze [14, 15, 16, 17, 18], .pow 19 pfold .fold,
   .squeeze [20, 21, 22, 23, 24], .pow 25 pquery .query]

/-- S4 after the owner decision: log blowup 4, `eta = 1/8`, `m = 3`. -/
def s4Whir : WhirParams := ⟨4, 1, 8, 3⟩

/-- The example key: two statement words, four `pv` words. -/
def lKey (pfold pmu pquery : Nat) (w : WhirParams) : KeyL :=
  ⟨[], lTranscript pfold pmu pquery, 2, 4, w⟩

theorem s4_mult_three : thm15Mult s4Whir = 3 := by decide

/-- The example key absorbs `pv` before its first challenge and is in the proven regime. -/
theorem lKey_ok : pvFirstT 2 4 (lTranscript 21 21 20) = true ∧
    ProvenRegime (lKey 21 21 20 s4Whir) = true := by decide

/-- The former 15-bit folding and `mu` proof of work is outside the proven regime. -/
theorem s4_pow15_not_proven : ProvenRegime (lKey 15 15 20 s4Whir) = false := by decide

/-- The `m = 1` reading (the p.23 remark) is outside the proven regime. -/
theorem remark_m1_not_proven : ProvenRegime (lKey 21 21 20 { s4Whir with mult := 1 }) = false := by
  decide

/-- A transcript that draws a challenge before absorbing `pv` fails the key check. -/
theorem late_pv_rejected :
    pvFirstT 2 4 [.absorbFr [6], .squeeze [7, 8, 9, 10, 11], .absorbBB [2, 3, 4, 5]] = false := by
  decide

/-- A transcript that absorbs only part of `pv` before the first challenge fails too. -/
theorem partial_pv_rejected :
    pvFirstT 2 4 [.absorbBB [2, 3], .squeeze [7, 8, 9, 10, 11], .absorbBB [4, 5]] = false := by
  decide

end Clean.Sherlock.Bridge
