import Clean.Sherlock.UDBits

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
* `EmulL`: the checks and transcript of `L` compiled into Fr cells (lazy arithmetic with bounded
  quotients, packing gadget, digit gadget with the canonical check): a sufficient relation for
  the Fr-native layer `L_Fr` to enforce for one `L` proof (option E, owner decision OD-24);
  `lfrSound_of_emul` shows it discharges A2'.
* `KeyLFr`, `VerifyLFr`: the layer `L_Fr`, abstract while its shape is built (step E2): an Fr
  transcript program with its own hash domain (use codes disjoint from `L`'s), and abstract native
  checks.
* `WrapRel`: the constraint semantics of the one BLS12-381 wrap `W`: the checks and transcript of
  `L_Fr`, native in Fr, a log table of exactly `B` entries with the canonical check on every
  value, `pv = pvOf W T` of the fixed length `W`, and native replay of every entry. Its public
  input is the statement only.
* `Premises`: the named assumptions as fields (no `axiom`), documented in `BridgeAxioms.lean`.

Two static key facts gate premise A2 and are computed from the key's own transcript program:
`pvFirstT` (every word of `pv` is absorbed before the first challenge or proof of work) and the
proof-of-work part of `ProvenRegime`, the gate G4 in unique decoding (`UDBits.lean`). The WHIR
analysis parameters (`UDSet`: blowup, polynomial size, folds, queries, `w_stack`) stay declared
metadata of the key. The list decoding gate of round 2 is kept as `ProvenRegimeLD`.
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

/-- The proof-of-work checks of SWIRL and WHIR: one per fold (round `i`, fold `s`, 1-based), one
for `mu` batching, one per round before its queries, one for LogUp. -/
inductive PowKind where
  | fold (round step : Nat)
  | mu
  | query (round : Nat)
  | logup
  deriving DecidableEq

def PowKind.isFold : PowKind → Bool
  | .fold _ _ => true
  | _ => false

def PowKind.isQuery : PowKind → Bool
  | .query _ => true
  | _ => false

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
  /-- An out-of-domain point of WHIR round `i`: a challenge squeezed as by `squeeze`. -/
  | oodPoint (round : Nat) (dsts : List Nat)
  /-- The prover's answer at an OOD point of round `i`: BabyBear words absorbed as by
  `absorbBB`. -/
  | oodAnswer (round : Nat) (vars : List Nat)

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
  | st, .oodPoint _ ds :: ops => ds.map (env.getD · 0) = digitsOf pBB 5 (X.frSponge st) ∧
      runRef X env (st ++ [X.frSponge st]) ops
  | st, .oodAnswer _ vs :: ops => (∀ i ∈ vs, env.getD i 0 < pBB) ∧
      runRef X env (st ++ packChunks (vs.map (env.getD · 0))) ops

/-- The Fr-cell transcript gadgets (inside `EmulL`): the packing gadget, the digit gadget with the
canonical check, and a bit decomposition for the proof of work. -/
def runGadget (X : Prims) (env : List Nat) : List Nat → List TOp → Prop
  | _, [] => True
  | st, .absorbBB vs :: ops => (∀ i ∈ vs, env.getD i 0 < pBB) ∧
      runGadget X env (st ++ packChunksR (vs.map (env.getD · 0))) ops
  | st, .absorbFr vs :: ops => runGadget X env (st ++ vs.map (env.getD · 0)) ops
  | st, .squeeze ds :: ops => SqueezeGadget (X.frSponge st) (ds.map (env.getD · 0)) ∧
      runGadget X env (st ++ [X.frSponge st]) ops
  | st, .pow n b _ :: ops => (∃ hi, X.frSponge (st ++ [env.getD n 0]) = hi * 2 ^ b) ∧
      runGadget X env (st ++ [env.getD n 0]) ops
  | st, .oodPoint _ ds :: ops => SqueezeGadget (X.frSponge st) (ds.map (env.getD · 0)) ∧
      runGadget X env (st ++ [X.frSponge st]) ops
  | st, .oodAnswer _ vs :: ops => (∀ i ∈ vs, env.getD i 0 < pBB) ∧
      runGadget X env (st ++ packChunksR (vs.map (env.getD · 0))) ops

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
  | st, .oodPoint _ ds :: ops, h =>
    ⟨squeeze_digits_sound (X.frSponge_lt st) h.1, run_refines X env _ ops h.2⟩
  | st, .oodAnswer _ vs :: ops, h => by
    simp only [runGadget, packChunksR_eq] at h
    exact ⟨h.1, run_refines X env _ ops h.2⟩

/-- The variables absorbed before the first challenge or proof of work. -/
def absorbedBeforeChallenge : List TOp → List Nat
  | [] => []
  | .absorbBB vs :: ops => vs ++ absorbedBeforeChallenge ops
  | .absorbFr vs :: ops => vs ++ absorbedBeforeChallenge ops
  | .squeeze _ :: _ => []
  | .pow _ _ _ :: _ => []
  | .oodPoint _ _ :: _ => []
  | .oodAnswer _ vs :: ops => vs ++ absorbedBeforeChallenge ops

/-- `pv` (environment indices `lo, ..., lo + n - 1`) is absorbed before the first challenge.
Computed from the transcript program itself. -/
def pvFirstT (lo n : Nat) (ops : List TOp) : Bool :=
  (List.range' lo n).all (fun i => (absorbedBeforeChallenge ops).contains i)

/-- The proof-of-work bits of the kinds that `k` selects, in transcript order. -/
def powList (k : PowKind → Bool) : List TOp → List Nat
  | [] => []
  | .pow _ b k' :: ops => if k k' then b :: powList k ops else powList k ops
  | _ :: ops => powList k ops

/-- The folding proof-of-work steps of the transcript program, in order: `(round, fold, bits)`. -/
def foldSteps : List TOp → List (Nat × Nat × Nat)
  | [] => []
  | .pow _ b (.fold i s) :: ops => (i, s, b) :: foldSteps ops
  | _ :: ops => foldSteps ops

/-- The query proof-of-work steps of the transcript program, in order: `(round, bits)`. -/
def querySteps : List TOp → List (Nat × Nat)
  | [] => []
  | .pow _ b (.query i) :: ops => (i, b) :: querySteps ops
  | _ :: ops => querySteps ops

/-- The OOD points of round `i` in the transcript program. -/
def oodPoints (i : Nat) : List TOp → Nat
  | [] => 0
  | .oodPoint j _ :: ops => (if j = i then 1 else 0) + oodPoints i ops
  | _ :: ops => oodPoints i ops

/-- The OOD answers of round `i` in the transcript program. -/
def oodAnswers (i : Nat) : List TOp → Nat
  | [] => 0
  | .oodAnswer j _ :: ops => (if j = i then 1 else 0) + oodAnswers i ops
  | _ :: ops => oodAnswers i ops

/-- Every OOD point of round `i` is squeezed before the first answer of round `i` is absorbed
(the order WHIR Lemma 4.25 needs for `s` samples drawn together). -/
def oodPointsFirst (i : Nat) : List TOp → Bool
  | [] => true
  | .oodAnswer j _ :: ops => if j = i then oodPoints i ops == 0 else oodPointsFirst i ops
  | _ :: ops => oodPointsFirst i ops

/-- The OOD shape of the transcript: in every round `1 .. M - 1`, one answer per point, and all
points before any answer. -/
def oodShapeOK (u : UDSet) (ops : List TOp) : Bool :=
  (List.range (u.rounds - 1)).all fun i =>
    oodAnswers (i + 1) ops == oodPoints (i + 1) ops && oodPointsFirst (i + 1) ops

/-- The fold positions of `M` WHIR rounds of `k` folds, in protocol order. -/
def foldSlots (M k : Nat) : List (Nat × Nat) :=
  (List.range M).flatMap fun i => (List.range k).map fun j => (i, j + 1)

/-- The weakest proof of work of a kind (`0` if the transcript has none). -/
def powMin : List Nat → Nat
  | [] => 0
  | b :: bs => bs.foldl min b

/-- The list decoding analysis parameters of round 2 (kept for `ProvenRegimeLD`). They are not
verifier operations: `mult` (the Guruswami-Sudan multiplicity) and `eta` (the slack below the Johnson radius) occur only in the
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
  /-- The declared WHIR analysis parameters (unique decoding). -/
  whir : UDSet

/-- The proof-of-work values of `L` read from its transcript program: the folding vector, one
value per fold step in transcript order; the query vector, one value per round; the weakest
`mu` and LogUp check (`0` if the transcript has none, so a missing check earns no credit); the
OOD sample count of each round `1 .. M - 1`, counted from the transcript's `oodPoint` steps. -/
def powVecOf (u : UDSet) (ops : List TOp) : PowVec :=
  ⟨(foldSteps ops).map (·.2.2), powMin (powList (· == .mu) ops), (querySteps ops).map (·.2),
    powMin (powList (· == .logup) ops),
    (List.range (u.rounds - 1)).map fun i => oodPoints (i + 1) ops⟩

/-- The transcript carries exactly one folding PoW step per fold, at its position in protocol
order (round-major), and one query PoW step per round, in round order. -/
def powShapeOK (u : UDSet) (ops : List TOp) : Bool :=
  (foldSteps ops).map (fun x => (x.1, x.2.1)) == foldSlots u.rounds u.kWhir &&
    (querySteps ops).map (·.1) == List.range u.rounds

/-- Gate G4, the proven regime of owner decision OD-19: unique decoding (list size 1) with the
error terms of WHIR Lemma 4.10, Corollary 4.11, Theorem 4.8 (unique decoding branch),
Theorem 5.2 and Lemma 4.25, and SWIRL Theorems 4.2.4 (unique decoding branch) and 4.2.1, over
`|EF| = p^4`, with the proof-of-work values read from the transcript program
(`UDSet.gate`: every grindable term at least 101 bits, every other term at least 100). -/
def ProvenRegime (vk : KeyL) : Bool :=
  powShapeOK vk.whir vk.transcript && oodShapeOK vk.whir vk.transcript &&
    vk.whir.gate efSize (powVecOf vk.whir vk.transcript)

/-- The list decoding regime of the owner decision of 2026-11-11 (option b of the paper review): the
Johnson-radius proximity-gap theorem applied as stated (`m` from `thm15Mult`, so `m >= 3`; the
unproved p.23 remark that allows `m = 1` is not used), and every folding and `mu` proof of work
of the transcript at least 21 bits, every query proof of work at least 20 bits. The proof-of-work
bits are read from the transcript program. Superseded for `L` by OD-19 (`ProvenRegime`). -/
def ProvenRegimeLD (w : WhirParams) (vk : KeyL) : Bool :=
  w.logBlowup % 2 == 0 && w.etaNum != 0 && w.mult == thm15Mult w &&
    21 ≤ powMin (powList PowKind.isFold vk.transcript) &&
    21 ≤ powMin (powList (· == .mu) vk.transcript) &&
    20 ≤ powMin (powList PowKind.isQuery vk.transcript)

/-- The reference semantics of one check. -/
def Check.Ref (X : Prims) (env : List Nat) : Check → Prop
  | .arith l r => (∀ i ∈ l.vars ++ r.vars, env.getD i 0 < pBB) ∧ l.evalP env = r.evalP env
  | .hash ins out => (∀ i ∈ ins, env.getD i 0 < pBB) ∧
      env.getD out 0 = X.frSponge (packChunks (ins.map (env.getD · 0)))
  | .merkle leaf bits sibs root =>
      merkleRoot X (env.getD leaf 0) (bits.map (fun i => env.getD i 0 == 1))
        (sibs.map (env.getD · 0)) = env.getD root 0

/-- The Fr-cell gadget for one check (inside `EmulL`). -/
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

/-! ## Hash domains on the Fr side (PROPOSAL section 3.6, owner decision OD-18) -/

/-- The initial capacity cell of a fixed-length Fr hash with use code `u` and length `l`. -/
def capInit (u l : Nat) : Nat := l + u * 2 ^ 66

/-- The initial capacity cell of a transcript with use code `u`: `2^64 + u 2^66`. -/
def trStart (u : Nat) : Nat := 2 ^ 64 + u * 2 ^ 66

/-- The use codes of `L` and of native and public hashes: 0 (native operations), 1 and 2
(Merkle leaves and nodes of `L`), 3 (the transcript of `L`), 4 (public digests). -/
def lUses : List Nat := [0, 1, 2, 3, 4]

/-- The use code of the transcript of `L`. -/
def lTTag : Nat := 3

theorem trStart_eq_cap (u : Nat) : trStart u = capInit u (2 ^ 64) := by
  simp [trStart, capInit]

/-- No two uses share an initial capacity: `(u, l)` is recovered from `l + u 2^66` for every
length `l < 2^66`. -/
theorem cap_inj {u u' l l' : Nat} (hl : l < 2 ^ 66) (hl' : l' < 2 ^ 66)
    (h : capInit u l = capInit u' l') : u = u' ∧ l = l' := by
  unfold capInit at h
  have h1 : (l + u * 2 ^ 66) / 2 ^ 66 = u := by
    rw [Nat.add_mul_div_right _ _ (by decide), Nat.div_eq_of_lt hl, Nat.zero_add]
  have h2 : (l' + u' * 2 ^ 66) / 2 ^ 66 = u' := by
    rw [Nat.add_mul_div_right _ _ (by decide), Nat.div_eq_of_lt hl', Nat.zero_add]
  have h3 : (l + u * 2 ^ 66) % 2 ^ 66 = l := by rw [Nat.add_mul_mod_self_right, Nat.mod_eq_of_lt hl]
  have h4 : (l' + u' * 2 ^ 66) % 2 ^ 66 = l' := by
    rw [Nat.add_mul_mod_self_right, Nat.mod_eq_of_lt hl']
  refine ⟨?_, ?_⟩
  · rw [← h1, ← h2, h]
  · rw [← h3, ← h4, h]

/-- The reference verifier of `L` on statement `s`, public values `pv` and proof `π`. The
transcript starts from the capacity `trStart lTTag`. -/
def VerifyL (X : Prims) (vk : KeyL) (s pv π : List Nat) : Prop :=
  s.length = vk.nStmt ∧ pv.length = vk.pvLen ∧ (∀ c ∈ vk.checks, c.Ref X (s ++ pv ++ π)) ∧
    runRef X (s ++ pv ++ π) [trStart lTTag] vk.transcript

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

/-- The batched OOD claim of a WHIR round (WHIR Construction 5.1, decision 2c):
`sum_j gamma^(j+1) v_j` over the OOD answers and the shift values, `gamma * poly_v(gamma)`. -/
def oodBatchExpr (gamma : Nat) (vs : List Nat) : Expr := .mul (.var gamma) (hornerExpr gamma vs)

/-- The Fr-cell check of the batched OOD claim gives the reference value mod `p`. -/
theorem ood_batch_refines {env : List Nat} {gamma c : Nat} {vs : List Nat}
    (hok : ArithOK (oodBatchExpr gamma vs) (.var c))
    (h : ArithGadget env (oodBatchExpr gamma vs) (.var c)) :
    env.getD gamma 0 * horner (env.getD gamma 0) (vs.map (env.getD · 0)) % pBB =
      env.getD c 0 % pBB := by
  have := (arith_refines hok h).2
  simpa [oodBatchExpr, Expr.evalP_eq, Expr.eval, hornerExpr_eval] using this

/-- What the OOD steps mean for an accepted verification: the digits of each OOD point are the
digits of the sponge output on everything absorbed before it, so a later answer cannot steer it. -/
theorem ood_point_digits {X : Prims} {env st : List Nat} {i : Nat} {ds : List Nat}
    {ops : List TOp} (h : runRef X env st (.oodPoint i ds :: ops)) :
    ds.map (env.getD · 0) = digitsOf pBB 5 (X.frSponge st) := h.1

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

/-! ## The emulation of `L` inside `L_Fr` -/

/-- The Fr-cell emulation of the verifier of `L` on `(s, pv, π)`: a sufficient relation for
`L_Fr`'s AIRs to enforce for one `L` proof (owner decision OD-24, option E), with the gadgets of
the round 2 wrap: range checks, lazy Fr arithmetic with bounded quotients, the packing gadget,
the digit gadget with the canonical check, the bit decomposition for proof of work. The real
shape of `L_Fr` is built in step E2; A2' does not depend on this relation. -/
def EmulL (X : Prims) (vk : KeyL) (s pv π : List Nat) : Prop :=
  s.length = vk.nStmt ∧ pv.length = vk.pvLen ∧ (∀ x ∈ s ++ pv ++ π, x < rFr) ∧
    (∀ c ∈ vk.checks, c.Gadget X (s ++ pv ++ π)) ∧
    runGadget X (s ++ pv ++ π) [trStart lTTag] vk.transcript

/-- Lemma 16 for `L_Fr` (`emulL_refines`): the emulated verification is an accepting reference
verification of `L` with the same statement and public values. -/
theorem emulL_refines {X : Prims} {vk : KeyL} {s pv π : List Nat} (hok : KeyOK vk)
    (h : EmulL X vk s pv π) : VerifyL X vk s pv π := by
  obtain ⟨hs, hp, _, hchk, hrun⟩ := h
  exact ⟨hs, hp, fun c hc => check_refines (hok.2.2 c hc) (hchk c hc), run_refines X _ _ _ hrun⟩

/-! ## The layer `L_Fr` (abstract) -/

/-- One operation of the Fr transcript of `L_Fr`: a native Fr duplex (no packing, whole-element
challenges). -/
inductive TOpFr where
  | absorb (vars : List Nat)
  | squeeze (dst : Nat)
  | pow (nonce bits : Nat)

/-- The reference transcript run of `L_Fr`. -/
def runFr (X : Prims) (env : List Nat) : List Nat → List TOpFr → Prop
  | _, [] => True
  | st, .absorb vs :: ops => runFr X env (st ++ vs.map (env.getD · 0)) ops
  | st, .squeeze d :: ops => env.getD d 0 = X.frSponge st ∧ runFr X env (st ++ [X.frSponge st]) ops
  | st, .pow n b :: ops => X.frSponge (st ++ [env.getD n 0]) % 2 ^ b = 0 ∧
      runFr X env (st ++ [env.getD n 0]) ops

/-- The wrap's native gadget for the transcript of `L_Fr` (proof of work by bit decomposition). -/
def runFrGadget (X : Prims) (env : List Nat) : List Nat → List TOpFr → Prop
  | _, [] => True
  | st, .absorb vs :: ops => runFrGadget X env (st ++ vs.map (env.getD · 0)) ops
  | st, .squeeze d :: ops => env.getD d 0 = X.frSponge st ∧
      runFrGadget X env (st ++ [X.frSponge st]) ops
  | st, .pow n b :: ops => (∃ hi, X.frSponge (st ++ [env.getD n 0]) = hi * 2 ^ b) ∧
      runFrGadget X env (st ++ [env.getD n 0]) ops

theorem runFr_refines (X : Prims) (env : List Nat) : ∀ (st : List Nat) (ops : List TOpFr),
    runFrGadget X env st ops → runFr X env st ops
  | _, [], _ => trivial
  | st, .absorb vs :: ops, h => runFr_refines X env _ ops h
  | st, .squeeze d :: ops, h => ⟨h.1, runFr_refines X env _ ops h.2⟩
  | st, .pow n b :: ops, h => by
    obtain ⟨⟨hi, hh⟩, h2⟩ := h
    exact ⟨by rw [hh, Nat.mul_mod_left], runFr_refines X env _ ops h2⟩

def absorbedBeforeChallengeFr : List TOpFr → List Nat
  | [] => []
  | .absorb vs :: ops => vs ++ absorbedBeforeChallengeFr ops
  | .squeeze _ :: _ => []
  | .pow _ _ :: _ => []

/-- `pv` is absorbed by `L_Fr`'s transcript before its first challenge. -/
def pvFirstFr (lo n : Nat) (ops : List TOpFr) : Bool :=
  (List.range' lo n).all (fun i => (absorbedBeforeChallengeFr ops).contains i)

/-- The key of `L_Fr`, abstract while its shape is built (step E2): the transcript program, the
fixed statement and `pv` lengths (those of `L`), and the Fr use codes of its hashes. Its other
checks (constraints, Merkle openings, WHIR arithmetic, all native in Fr) are the abstract
predicate `LFrAccept` of `Premises`. -/
structure KeyLFr where
  transcript : List TOpFr
  nStmt : Nat
  pvLen : Nat
  /-- The Fr use codes `u` of `L_Fr`'s hashes (Merkle leaves, nodes, transcript). -/
  uses : List Nat
  /-- The use code of `L_Fr`'s transcript. -/
  tTag : Nat

/-- Tag separation: `L_Fr`'s transcript code is one of its use codes, no use code of `L_Fr` is a
use code of `L` (or of a native or public hash), and every initial capacity is an Fr element. -/
def tagSep (vk : KeyLFr) : Bool :=
  vk.uses.contains vk.tTag && vk.uses.all (fun u => !lUses.contains u && u < 2 ^ 186)

/-- Static facts of the key of `L_Fr`: `pv` absorbed first, tag separation, and the statement
and `pv` lengths of `L`. -/
def KeyOKFr (vkL : KeyL) (vk : KeyLFr) : Prop :=
  pvFirstFr vk.nStmt vk.pvLen vk.transcript = true ∧ tagSep vk = true ∧ vk.nStmt = vkL.nStmt ∧
    vk.pvLen = vkL.pvLen

/-- The reference verifier of `L_Fr` on statement `s`, public values `pv` and proof `π`, with the
abstract native checks `acc`. Its transcript starts from `trStart vk.tTag`. -/
def VerifyLFr (X : Prims) (acc : List Nat → Prop) (vk : KeyLFr) (s pv π : List Nat) : Prop :=
  s.length = vk.nStmt ∧ pv.length = vk.pvLen ∧ (∀ x ∈ s ++ pv ++ π, x < rFr) ∧
    acc (s ++ pv ++ π) ∧ runFr X (s ++ pv ++ π) [trStart vk.tTag] vk.transcript

/-- Tag separation, in the capacity cell: no initial state of an `L_Fr` hash is the initial state
of an `L` (or native or public) hash, for every length below `2^66` (transcripts use
`l = 2^64`, `trStart_eq_cap`). -/
theorem tags_separated {vk : KeyLFr} (h : tagSep vk = true) {u u' l l' : Nat} (hu : u ∈ vk.uses)
    (hu' : u' ∈ lUses) (hl : l < 2 ^ 66) (hl' : l' < 2 ^ 66) : capInit u l ≠ capInit u' l' := by
  intro he
  have := (cap_inj hl hl' he).1
  subst this
  simp only [tagSep, Bool.and_eq_true, List.all_eq_true] at h
  have := (h.2 u hu)
  simp only [Bool.and_eq_true, Bool.not_eq_true', decide_eq_true_eq] at this
  rw [List.contains_eq_any_beq] at this
  simp only [List.any_eq_false, beq_iff_eq] at this
  exact this.1 u hu' rfl

/-- The transcripts of `L_Fr` and `L` never give the same sponge output on carrier inputs:
under A2b, equal outputs would need equal initial capacities, which tag separation excludes. -/
theorem transcripts_separated {X : Prims} {FrDom : List Nat → Prop} (hinj : ∀ a b, FrDom a →
    FrDom b → X.frSponge a = X.frSponge b → a = b) {vk : KeyLFr} (h : tagSep vk = true)
    {a b : List Nat} (ha : FrDom (trStart vk.tTag :: a)) (hb : FrDom (trStart lTTag :: b)) :
    X.frSponge (trStart vk.tTag :: a) ≠ X.frSponge (trStart lTTag :: b) := by
  intro he
  have := hinj _ _ ha hb he
  simp only [List.cons.injEq] at this
  have ht : vk.tTag ∈ vk.uses := by
    simp only [tagSep, Bool.and_eq_true] at h
    exact List.contains_iff_mem.mp h.1
  rw [trStart_eq_cap, trStart_eq_cap] at this
  exact tags_separated h ht (by decide : lTTag ∈ lUses) (by decide) (by decide) this.1

/-- A2' from the emulation: if every accepting `L_Fr` verification has an `L` proof whose Fr-cell
emulation holds, then (by `emulL_refines`) it has one that the reference verifier of `L` accepts,
with the same statement and public values. -/
theorem lfrSound_of_emul {X : Prims} {acc : List Nat → Prop} {vkL : KeyL} {vk : KeyLFr}
    (hok : KeyOK vkL)
    (h : ∀ s pv π, VerifyLFr X acc vk s pv π → ∃ πL, EmulL X vkL s pv πL) :
    ∀ s pv π, VerifyLFr X acc vk s pv π → ∃ πL, VerifyL X vkL s pv πL := by
  intro s pv π hv
  obtain ⟨πL, hE⟩ := h s pv π hv
  exact ⟨πL, emulL_refines hok hE⟩

/-! ## The wrap -/

/-- The private witness of the wrap: the public values of `L` (the log words), the proof of
`L_Fr` and the log table. -/
structure WrapWit where
  pv : List Nat
  π : List Nat
  T : List Entry

/-- The wrap's gadget on one table slot: range-checked tag and parameter, the arity of the
operation, and the canonical check on every value. -/
def EntryGadget (e : Entry) : Prop :=
  (∀ x ∈ e.op.code, x < pBB) ∧ e.vals.length = e.op.arity ∧ ∀ v ∈ e.vals, CanonGadget v

/-- The constraint semantics of the one BLS12-381 wrap under option E (OD-24): it verifies
`L_Fr` natively in Fr (the native checks `acc` and the transcript of the key `vk` of `L_Fr`),
with capacity `B` entries and `pv` length `vk.pvLen`. Its public input `pub` is the statement
only; everything else is private witness. -/
def WrapRel (X : Prims) (B : Nat) (acc : List Nat → Prop) (vk : KeyLFr) (pub : List Nat)
    (w : WrapWit) : Prop :=
  pub.length = vk.nStmt ∧
  (∀ x ∈ pub ++ w.pv ++ w.π, x < rFr) ∧
  acc (pub ++ w.pv ++ w.π) ∧
  runFrGadget X (pub ++ w.pv ++ w.π) [trStart vk.tTag] vk.transcript ∧
  w.T.length = B ∧
  (∀ e ∈ w.T, EntryGadget e) ∧
  (encode w.T).length ≤ vk.pvLen ∧
  w.pv = pvOf vk.pvLen w.T ∧
  (∀ e ∈ w.T, ReplayGadget X pub e)

/-- Lemma 16 (`wrap_refines`): an accepted wrap witness is an accepting reference verification of
`L_Fr` on the statement, with a table of exactly `B` well-formed, valid entries whose padded
encoding of the fixed length is `pv`. -/
theorem wrap_refines {X : Prims} {B : Nat} {acc : List Nat → Prop} {vk : KeyLFr}
    {pub : List Nat} {w : WrapWit} (h : WrapRel X B acc vk pub w) :
    VerifyLFr X acc vk pub w.pv w.π ∧ w.T.length = B ∧ (∀ e ∈ w.T, e.WF ∧ Valid X pub e) ∧
      w.pv = pvOf vk.pvLen w.T := by
  obtain ⟨hpl, hr, hacc, hrun, hlen, hent, henc, hpv, hrep⟩ := h
  refine ⟨⟨hpl, by rw [hpv]; exact pvOf_length henc, hr, hacc, runFr_refines X _ _ _ hrun⟩,
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
  /-- The key of `L_Fr` (option E, OD-24). -/
  vkLFr : KeyLFr
  /-- The native Fr checks of `L_Fr` other than its transcript (abstract until step E2). -/
  LFrAccept : List Nat → Prop
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
  /-- Static facts of the key of `L_Fr`. -/
  vkLFr_ok : KeyOKFr vkL vkLFr
  /-- (A1) Knowledge soundness of midnight-proofs PLONK with KZG (AGM, q-type assumption). -/
  wrapKnowledgeSound : ∀ pub prf, LedgerAccepts pub prf →
    ∃ w, WrapRel X B LFrAccept vkLFr pub w
  /-- (A2') Soundness of `L_Fr` (option E, OD-24): if the reference verifier of `L_Fr` accepts
  `(s, pv, π)`, the reference verifier of `L` accepts an embedded `L` proof `πL` with the same
  statement and public values (knowledge soundness of the Fr STARK `L_Fr`). Hypotheses: A2b, and
  the static key facts of `L_Fr` (`pv` absorbed first, tag separation from `L`'s hash domains).
  `lfrSound_of_emul`: an `L_Fr` that enforces `EmulL` meets it. -/
  lfrSound : FrInj X FrDom → KeyOKFr vkL vkLFr →
    ∀ s pv π, VerifyLFr X LFrAccept vkLFr s pv π → ∃ πL, VerifyL X vkL s pv πL
  /-- (A2) Soundness of `L` (statements as recorded in
  `research/sherlock-design-paper-review-20261111/G5-PAPER-MAP.md`, which reads the full papers).
  WHIR round-by-round soundness (WHIR Theorem 5.2 with list size 1) with mutual correlated
  agreement in unique decoding (WHIR Lemma 4.10, Corollary 4.11, Theorem 4.8 unique decoding
  branch), the OOD samples of each round (WHIR Lemma 4.25), `mu` batching (SWIRL Theorem 4.2.4,
  unique decoding branch), LogUp (SWIRL Theorem 4.2.1, total interactions below `p`), the
  composition of SWIRL Theorem 4.2.5, Fiat-Shamir in the random-oracle model (non-interactive
  error `Q_RO 2^-100 + 3 (Q_RO^2 + 1) / 2^kappa`, SWIRL Corollary 4.2.6), the `eq` binding of `pv`
  (`c m / |EF|`, counting core `pv_binding_count`), the squeeze bias. Every term of the WHIR and
  `mu` part is computed by `UDSet.gate` and checked at 101 bits (grindable) or 100 bits on the
  key; the bit counts of the frozen set are `lUD_bits`. Hypotheses: the packed Fr hash is
  injective, the transcript absorbs every word of `pv` before any challenge (`pvFirstT`), and
  the key passes `ProvenRegime` (proof of work read from the transcript). -/
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
  wrapZK : ∀ pub w, WrapRel X B LFrAccept vkLFr pub w → Indist (prove pub w) (sim pub)

/-! ## The main theorem -/

/-- Lemma 22 (`bridge_sound`): a proof that ledger 9 accepts for public input `pub` implies the
claim `pub` of the reference machine. The path: the wrap (A1) verifies `L_Fr`, `L_Fr` (A2')
verifies `L`, `L` (A2) verifies the root. -/
theorem bridge_sound (P : Premises) {pub : List Nat} {prf : P.LedgerProof}
    (h : P.LedgerAccepts pub prf) : P.Run pub := by
  obtain ⟨w, hw⟩ := P.wrapKnowledgeSound pub prf h
  obtain ⟨hVFr, hTlen, hTok, hpv⟩ := wrap_refines hw
  obtain ⟨πL, hV⟩ := P.lfrSound P.frSpongeInj P.vkLFr_ok pub w.pv w.π hVFr
  obtain ⟨d, hroot⟩ :=
    P.lastLayerSound (packed_hash_inj P.frSpongeInj) P.vkL_ok.1 P.vkL_ok.2.1 pub w.pv πL hV
  obtain ⟨hrec, t, ⟨htlen, htwf, htin⟩, henc, hdig⟩ := P.rootBinds _ _ _ hroot
  rw [P.vkLFr_ok.2.2.2] at hpv
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
(different logs, used-entry counts, deferred-or-emulated splits, proofs of `L_Fr` and so of
`L`) give
indistinguishable proofs. -/
theorem wrap_view_only_claim (P : Premises) {pub : List Nat} {w₁ w₂ : WrapWit}
    (h₁ : WrapRel P.X P.B P.LFrAccept P.vkLFr pub w₁)
    (h₂ : WrapRel P.X P.B P.LFrAccept P.vkLFr pub w₂) :
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

/-- One WHIR round of the example transcript of `L`: for `i >= 1` the commitment of round `i`,
its `s` OOD points, then their `s` answers (4 words each); then the `k` folds (a squeezed folding
challenge and the PoW step of that fold), then the query PoW and the query challenge. Round `i`
uses environment indices `100 + 50 i ..` (`s <= 2`). -/
def lRound (k s : Nat) (pfold pquery : List Nat) (i : Nat) : List TOp :=
  let b := 100 + 50 * i
  (if i = 0 then [] else
    [.absorbFr [b + 24]] ++ (List.range s).map (fun j => .oodPoint i (List.range' (b + 25 + 5 * j) 5)) ++
      (List.range s).map (fun j => .oodAnswer i (List.range' (b + 35 + 4 * j) 4))) ++
  ((List.range k).flatMap fun j =>
    [.squeeze (List.range' (b + 6 * j) 5), .pow (b + 6 * j + 5) (pfold.getD (k * i + j) 0) (.fold i (j + 1))]) ++
  [.pow (b + 45) (pquery.getD i 0) (.query i), .squeeze (List.range' (b + 44) 1)]

/-- A transcript skeleton of `L` in the order of stark-backend's verifier preamble
(`verifier/mod.rs:128-175`): commitments, then the public values, then challenges, with the LogUp
and `mu` proof of work; then 3 WHIR rounds of 4 folds, each fold with its PoW step, `s` OOD
samples in rounds 1 and 2, and the query PoW of each round. Indices: statement `0..1`, `pv` `2..5`, proof from `6`. -/
def lTranscript (pfold : List Nat) (pmu : Nat) (pquery : List Nat) (plogup s : Nat) : List TOp :=
  [.absorbFr [6], .absorbBB [2, 3, 4, 5], .squeeze [7, 8, 9, 10, 11], .pow 26 plogup .logup,
   .squeeze [27, 28, 29, 30, 31], .pow 12 pmu .mu, .squeeze [13, 14, 15, 16, 17]] ++
  (List.range 3).flatMap (lRound 4 s pfold pquery)

/-- The OD-19 transcript: folding PoW 0 on all 12 folds, `mu` 6, query 20 per round, LogUp 18,
one OOD sample per round. -/
def lTranscriptUD : List TOp := lTranscript (List.replicate 12 0) 6 [20, 20, 20] 18 1

/-- The same with two OOD samples per round (points `z_0, z_1`, then answers `y_0, y_1`). -/
def lTranscriptUD2 : List TOp := lTranscript (List.replicate 12 0) 6 [20, 20, 20] 18 2

/-- The example key at the OD-19 set: two statement words, four `pv` words. -/
def lKey (T : List TOp) : KeyL := ⟨[], T, 2, 4, lUD⟩

/-- Gate G4 on the OD-19 key: `pv` is absorbed before the first challenge, the transcript has one
PoW step per fold and per round, and its proof of work passes `ProvenRegime`. -/
theorem lKey_ok : pvFirstT 2 4 lTranscriptUD = true ∧ ProvenRegime (lKey lTranscriptUD) = true := by
  decide

/-- The PoW vector the gate reads is the transcript's own. -/
theorem lKey_powVec : powVecOf lUD lTranscriptUD = lPow := by decide

/-- The transcript's proof of work is what the gate reads: `mu` PoW 5 fails it. -/
theorem lKey_mu5_not_proven :
    ProvenRegime (lKey (lTranscript (List.replicate 12 0) 5 [20, 20, 20] 18 1)) = false := by decide

/-- Query PoW 19 in one round (round 1) fails it. -/
theorem lKey_query19_not_proven :
    ProvenRegime (lKey (lTranscript (List.replicate 12 0) 6 [20, 19, 20] 18 1)) = false := by decide

/-- LogUp PoW 17, below the frozen 18, fails it. -/
theorem lKey_logup17_not_proven :
    ProvenRegime (lKey (lTranscript (List.replicate 12 0) 6 [20, 20, 20] 17 1)) = false := by decide

/-- A transcript with only 2 WHIR rounds of fold steps (8 fold steps for 12 folds) fails it. -/
theorem lKey_missing_folds_not_proven :
    ProvenRegime (lKey ([.absorbFr [6], .absorbBB [2, 3, 4, 5], .squeeze [7, 8, 9, 10, 11],
      .pow 26 18 .logup, .squeeze [27, 28, 29, 30, 31], .pow 12 6 .mu] ++
      (List.range 2).flatMap (lRound 4 1 (List.replicate 12 0) [20, 20, 20]) ++
      [.pow 245 20 (.query 2)])) = false := by decide

/-- The key with two OOD samples per round passes the gate, with the OOD counts read from the
transcript (OOD terms 216 and 224 bits, `lUD_two_ood`). -/
theorem lKey2_ok : pvFirstT 2 4 lTranscriptUD2 = true ∧ ProvenRegime (lKey lTranscriptUD2) = true ∧
    (powVecOf lUD lTranscriptUD2).ood = [2, 2] := by decide

/-- Interleaved OOD steps (`z_0, y_0, z_1, y_1`) fail the order check. -/
theorem ood_interleaved_rejected :
    oodPointsFirst 1 [.oodPoint 1 [0], .oodAnswer 1 [1], .oodPoint 1 [2], .oodAnswer 1 [3]] = false ∧
    oodPointsFirst 1 [.oodPoint 1 [0], .oodPoint 1 [2], .oodAnswer 1 [1], .oodAnswer 1 [3]] = true := by
  decide

/-- A transcript whose rounds 1 and 2 have no OOD sample fails the gate. -/
theorem lKey_no_ood_not_proven :
    ProvenRegime (lKey (lTranscript (List.replicate 12 0) 6 [20, 20, 20] 18 0)) = false := by decide

/-- Fold steps in the wrong order (round 0 fold 2 before fold 1) fail it. -/
theorem lKey_fold_order_not_proven :
    ProvenRegime (lKey ([.pow 26 18 .logup, .pow 12 6 .mu, .pow 101 0 (.fold 0 2),
      .pow 102 0 (.fold 0 1)] ++ (lTranscriptUD.filter fun
        | .pow _ _ (.fold 0 1) => false | .pow _ _ (.fold 0 2) => false | _ => true))) = false := by
  decide

/-- The list decoding transcript of round 2 (21-bit folding and `mu`, 20-bit query PoW). -/
def lTranscriptLD (pfold pmu pquery : Nat) : List TOp :=
  [.absorbFr [6], .absorbBB [2, 3, 4, 5], .squeeze [7, 8, 9, 10, 11], .pow 12 pmu .mu,
   .absorbFr [13], .squeeze [14, 15, 16, 17, 18], .pow 19 pfold (.fold 0 1),
   .squeeze [20, 21, 22, 23, 24], .pow 25 pquery (.query 0)]

/-- S4 after the owner decision of 2026-11-11: log blowup 4, `eta = 1/8`, `m = 3`. -/
def s4Whir : WhirParams := ⟨4, 1, 8, 3⟩

def lKeyLD (pfold pmu pquery : Nat) : KeyL := ⟨[], lTranscriptLD pfold pmu pquery, 2, 4, lUD⟩

theorem s4_mult_three : thm15Mult s4Whir = 3 := by decide

/-- The round 2 key is in the list decoding regime. -/
theorem lKeyLD_ok : ProvenRegimeLD s4Whir (lKeyLD 21 21 20) = true := by decide

/-- The former 15-bit folding and `mu` proof of work is outside the list decoding regime. -/
theorem s4_pow15_not_proven : ProvenRegimeLD s4Whir (lKeyLD 15 15 20) = false := by decide

/-- The `m = 1` reading (the p.23 remark) is outside the list decoding regime. -/
theorem remark_m1_not_proven :
    ProvenRegimeLD { s4Whir with mult := 1 } (lKeyLD 21 21 20) = false := by decide

/-- An example key of `L_Fr` (not frozen; step E2 builds the real one): statement `0..1`, `pv`
`2..5`, a commitment, then challenges; use codes 5, 6 and 7 with the transcript on 7. -/
def lfrKey : KeyLFr := ⟨[.absorb [0, 1, 2, 3, 4, 5, 6], .squeeze 7, .pow 8 20, .squeeze 9], 2, 4,
  [5, 6, 7], 7⟩

theorem lfrKey_ok : KeyOKFr (lKey lTranscriptUD) lfrKey := ⟨by decide, by decide, rfl, rfl⟩

/-- A key of `L_Fr` that reuses `L`'s transcript code 3 fails tag separation. -/
theorem lfr_tag_reuse_rejected : tagSep { lfrKey with uses := [5, 6, 3], tTag := 3 } = false := by
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
