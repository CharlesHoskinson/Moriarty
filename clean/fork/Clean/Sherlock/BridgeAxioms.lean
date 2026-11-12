import Clean.Sherlock.Bridge
import Clean.Sherlock.PvBinding

/-!
Premise file of `Clean.Sherlock.Bridge` and `Clean.Sherlock.FrLog` (bridge round, QUEUE.md
step 3; CANDIDATE.md section 11.3).

No `axiom` is declared in any of the three files. Every assumption is a field of
`Clean.Sherlock.Bridge.Premises` and enters `bridge_sound` as a hypothesis, by name:

* (A1) `wrapKnowledgeSound`: a proof that ledger 9 `VerifierKey::verify` accepts for public input
  `pub` has a witness of `WrapRel`. Knowledge soundness of midnight-proofs PLONK with KZG over
  BLS12-381 in the algebraic group model with a q-type assumption, as for today's decider.
* (A2) `lastLayerSound`: a reference verification of `L` that accepts yields an accepted root AIR
  with some log digest. The proximity-gap part is the proven Johnson-radius statement (citations per
  the paper review `REVIEW.md` section 4, not read by the Lean lane), BCHKS25
  Theorem 1.5 as stated (`m >= 3`), with Theorems 4.5 and 4.6, in the parameters of
  `ProvenRegime` (owner decision 2026-11-11: folding and `mu` PoW 21 bits, 100.2 and 102.3
  bits at `m = 3`); the unproved p.23 remark is not a premise. Then WHIR (SWIRL) round-by-round
  soundness, Fiat-Shamir in the random-oracle model, the `eq` binding of `pv` (error
  at most `c m / |EF|`, Thaler Lemma 1) and the 5-digit squeeze bias (at most `2^-100`). It has three
  hypotheses that `bridge_sound` discharges: `PackedHashInj` (proved from A2b by
  `packed_hash_inj`: packing with checked length and range), `pvFirstT` and `ProvenRegime`
  (both static key checks; `pvFirstT` and the proof-of-work bits are computed from the key's
  transcript program, the WHIR analysis parameters are declared).
* (A2b) `frSpongeInj`: collision resistance of Midnight Poseidon over Fr (the transcript and
  Merkle hash of `L`, and the public digest form `PoseidonFr(tag, pack d2)`), guarded by the
  carrier `FrDom`. Separate from A5.
* (A3) `rootBinds`: the root AIR of `L` binds its public values to a carrier log tree: at most
  `B` entries, each well-formed, `pv = encode (pad B (flatten t))`, and the digest of `t` is the
  digest the recursion carries. A circuit fact of `L` (its log table, the `eq` binding and the
  onion recompute), measured in M9 and tested in M14.
* (A4) `recSound`: recursion soundness of leaves, discharge shards and nodes: an accepted digest
  is the digest of a carrier tree whose flattened log the run was proved against
  (`RunWith`: every operation either deferred into the log or proved inline by tier E).
* (A5) `p2Inj`: collision resistance of Poseidon2 over BabyBear, guarded by the carrier `P2Dom`
  as `Aggregation.Premises.digest_inj` is. It covers the log onion and the internal
  `sherlock-value/2` digests (owner decision 2026-11-11). Tree-digest injectivity
  (`digest_inj`) is a theorem from it.
* (A6) `oracleElim`: a run whose Fr operations are all `Valid` is a run of the reference machine.
  A premise until the Sherlock step relation is imported (as `Aggregation.Premises.Step`).
* (A7) `wrapZK`: zero knowledge of the blinded wrap: for every satisfying witness the proof
  distribution equals the simulator's on the public input alone. Not used by `bridge_sound`;
  used by `wrap_view_only_claim`. `indist_symm` and `indist_trans` say that indistinguishability
  is an equivalence.
* `vkL_ok`: not cryptographic. Static facts of the frozen key of `L` (`pvFirstT` of its
  transcript program, `ProvenRegime`, and the bound `< r` of every compiled
  expression), decided on the concrete key.

The carriers `P2Dom` and `FrDom` are abstract on purpose: a hash with a finite digest is not
injective on all inputs, so an unguarded injectivity premise would be false in Lean and make every
theorem vacuous. `consistent` below shows the premises are jointly satisfiable.

The allowed `#print axioms` output is the empty set or `propext`, `Quot.sound`,
`Classical.choice`.
-/

open Clean.Sherlock.Bridge

-- Sanity checks (decided by the kernel, not asserted).
example : pack [1, 2] = 1 + 2 * 2 ^ 31 := by decide
example : digitsOf pBB 5 7 = [7, 0, 0, 0, 0] := by decide
example : (Op.sponge 3).arity = 4 := rfl
example : encode [nopE] = [0, 0] := rfl
example : pad 3 [nopE] = [nopE, nopE, nopE] := rfl
example : efRaw3.bound < rFr := by decide

/-- The premises are jointly satisfiable (a trivial model: nothing is accepted). This shows
consistency only, not that the real primitives satisfy them. -/
def consistent : Premises where
  B := 0
  X := { frPerm := id, frSponge := fun _ => 0, jubjubMul := fun _ q => q, jubjubGen := (0, 0),
         p2 := fun _ => List.replicate 8 0, p2_width := fun _ => by simp,
         frSponge_lt := fun _ => by decide }
  vkL := { checks := [.arith (.const 0) (.const 1)], transcript := lTranscript 21 21 20,
           nStmt := 2, pvLen := 4, whir := s4Whir }
  P2Dom := fun _ => False
  FrDom := fun _ => False
  LedgerProof := Unit
  LedgerAccepts := fun _ _ => False
  RootAccepts := fun _ _ _ => False
  RecAccepts := fun _ _ => False
  RunOps := fun _ _ => False
  Run := fun _ => True
  vkL_ok := ⟨by decide, by decide, by
    intro c hc
    simp only [List.mem_singleton] at hc
    subst hc
    exact ⟨by decide, by decide⟩⟩
  wrapKnowledgeSound := fun _ _ h => h.elim
  lastLayerSound := by
    intro _ _ _ s pv π h
    have := (h.2.2.1 _ (List.mem_singleton_self _)).2
    change 0 % pBB = 1 % pBB at this
    exact absurd this (by decide)
  frSpongeInj := fun _ _ h => h.elim
  rootBinds := fun _ _ _ h => h.elim
  recSound := fun _ _ h => h.elim
  p2Inj := fun _ _ h => h.elim
  oracleElim := fun _ _ _ _ => trivial
  Coins := Unit
  prove := fun _ _ _ => ()
  sim := fun _ _ => ()
  Indist := fun _ _ => True
  indist_symm := fun _ _ _ => trivial
  indist_trans := fun _ _ _ _ _ => trivial
  wrapZK := fun _ _ _ => trivial

-- Every theorem of FrLog.lean, Bridge.lean and PvBinding.lean.
#print axioms Clean.Sherlock.Bridge.pBB_eq
#print axioms Clean.Sherlock.Bridge.pBB_pos
#print axioms Clean.Sherlock.Bridge.limb_fits
#print axioms Clean.Sherlock.Bridge.limbs_cover
#print axioms Clean.Sherlock.Bridge.pack_fits
#print axioms Clean.Sherlock.Bridge.packBase_gt
#print axioms Clean.Sherlock.Bridge.qMax_fits
#print axioms Clean.Sherlock.Bridge.r_big
#print axioms Clean.Sherlock.Bridge.add_mul_mod_congr
#print axioms Clean.Sherlock.Bridge.hornerR_eq
#print axioms Clean.Sherlock.Bridge.horner_lt
#print axioms Clean.Sherlock.Bridge.horner_inj
#print axioms Clean.Sherlock.Bridge.digits_unique
#print axioms Clean.Sherlock.Bridge.dec_inj
#print axioms Clean.Sherlock.Bridge.canon_mod_inj
#print axioms Clean.Sherlock.Bridge.limb_embed_inj
#print axioms Clean.Sherlock.Bridge.decR_canon
#print axioms Clean.Sherlock.Bridge.ltLE_sound
#print axioms Clean.Sherlock.Bridge.rLimbs_props
#print axioms Clean.Sherlock.Bridge.canon_check_sound
#print axioms Clean.Sherlock.Bridge.alias_without_canon
#print axioms Clean.Sherlock.Bridge.pack_injective
#print axioms Clean.Sherlock.Bridge.pack_length_check_needed
#print axioms Clean.Sherlock.Bridge.pack_range_check_needed
#print axioms Clean.Sherlock.Bridge.pack_gadget_refines
#print axioms Clean.Sherlock.Bridge.chunks8_nil
#print axioms Clean.Sherlock.Bridge.chunks8_cons
#print axioms Clean.Sherlock.Bridge.packChunksR_eq
#print axioms Clean.Sherlock.Bridge.packChunks_inj
#print axioms Clean.Sherlock.Bridge.packChunks_lt
#print axioms Clean.Sherlock.Bridge.bounded_quotient_sound
#print axioms Clean.Sherlock.Bridge.fmul_sound
#print axioms Clean.Sherlock.Bridge.unbounded_quotient_unsound
#print axioms Clean.Sherlock.Bridge.lift_eq
#print axioms Clean.Sherlock.Bridge.reduce_sound
#print axioms Clean.Sherlock.Bridge.reduce_bits_ok
#print axioms Clean.Sherlock.Bridge.Expr.evalP_eq
#print axioms Clean.Sherlock.Bridge.Expr.evalR_eq
#print axioms Clean.Sherlock.Bridge.Expr.eval_le_bound
#print axioms Clean.Sherlock.Bridge.reduce_side
#print axioms Clean.Sherlock.Bridge.arith_refines
#print axioms Clean.Sherlock.Bridge.arith_var_sound
#print axioms Clean.Sherlock.Bridge.efRaw_ok
#print axioms Clean.Sherlock.Bridge.ef_mul_sound
#print axioms Clean.Sherlock.Bridge.hornerExpr_eval
#print axioms Clean.Sherlock.Bridge.horner_refines
#print axioms Clean.Sherlock.Bridge.sumExpr_eval
#print axioms Clean.Sherlock.Bridge.roundSumExpr_eval
#print axioms Clean.Sherlock.Bridge.sumcheck_round_refines
#print axioms Clean.Sherlock.Bridge.whir_round_refines
#print axioms Clean.Sherlock.Bridge.whir_round_chain
#print axioms Clean.Sherlock.Bridge.coset_fold_refines
#print axioms Clean.Sherlock.Bridge.fold_complete
#print axioms Clean.Sherlock.Bridge.eqExpr_eval
#print axioms Clean.Sherlock.Bridge.eqSumExpr_eval
#print axioms Clean.Sherlock.Bridge.pv_eq_refines
#print axioms Clean.Sherlock.Bridge.eqVal_bool
#print axioms Clean.Sherlock.Bridge.squeeze_digits_sound
#print axioms Clean.Sherlock.Bridge.squeeze_alias
#print axioms Clean.Sherlock.Bridge.Op.code_length
#print axioms Clean.Sherlock.Bridge.Op.code_inj
#print axioms Clean.Sherlock.Bridge.nopE_wf
#print axioms Clean.Sherlock.Bridge.pad_length
#print axioms Clean.Sherlock.Bridge.flatten_append_inj
#print axioms Clean.Sherlock.Bridge.enc_append_inj
#print axioms Clean.Sherlock.Bridge.encode_inj
#print axioms Clean.Sherlock.Bridge.encode_append_inj
#print axioms Clean.Sherlock.Bridge.pvOf_length
#print axioms Clean.Sherlock.Bridge.pvOf_inj
#print axioms Clean.Sherlock.Bridge.pad_wf
#print axioms Clean.Sherlock.Bridge.replay_sound
#print axioms Clean.Sherlock.Bridge.nop_pad_valid
#print axioms Clean.Sherlock.Bridge.onion_snoc
#print axioms Clean.Sherlock.Bridge.onion_empty
#print axioms Clean.Sherlock.Bridge.onionRev_length
#print axioms Clean.Sherlock.Bridge.onionRev_inj
#print axioms Clean.Sherlock.Bridge.onionRev_cases
#print axioms Clean.Sherlock.Bridge.seg_ne_node
#print axioms Clean.Sherlock.Bridge.digest_length
#print axioms Clean.Sherlock.Bridge.digest_inj
#print axioms Clean.Sherlock.Bridge.filter_split
#print axioms Clean.Sherlock.Bridge.spill_equiv
#print axioms Clean.Sherlock.Bridge.runWith_all_valid
#print axioms Clean.Sherlock.Bridge.discharge_sound
#print axioms Clean.Sherlock.Bridge.run_refines
#print axioms Clean.Sherlock.Bridge.pv_absorbed_before_challenge
#print axioms Clean.Sherlock.Bridge.getD_lt
#print axioms Clean.Sherlock.Bridge.check_refines
#print axioms Clean.Sherlock.Bridge.wrap_refines
#print axioms Clean.Sherlock.Bridge.packed_hash_inj
#print axioms Clean.Sherlock.Bridge.bridge_sound
#print axioms Clean.Sherlock.Bridge.wrap_view_only_claim
#print axioms Clean.Sherlock.Bridge.merkle_binding_or_collision
#print axioms Clean.Sherlock.Bridge.public_digest_binds
#print axioms Clean.Sherlock.Bridge.s4_mult_three
#print axioms Clean.Sherlock.Bridge.lKey_ok
#print axioms Clean.Sherlock.Bridge.s4_pow15_not_proven
#print axioms Clean.Sherlock.Bridge.remark_m1_not_proven
#print axioms Clean.Sherlock.Bridge.late_pv_rejected
#print axioms Clean.Sherlock.Bridge.partial_pv_rejected
#print axioms Clean.Sherlock.Bridge.PvBinding.euclid_of_prime
#print axioms Clean.Sherlock.Bridge.PvBinding.mle_mod
#print axioms Clean.Sherlock.Bridge.PvBinding.points_length
#print axioms Clean.Sherlock.Bridge.PvBinding.countP_le_one
#print axioms Clean.Sherlock.Bridge.PvBinding.line_roots
#print axioms Clean.Sherlock.Bridge.PvBinding.flat_count
#print axioms Clean.Sherlock.Bridge.PvBinding.mul_pow_pred_le
#print axioms Clean.Sherlock.Bridge.PvBinding.pv_binding_count
#print axioms Clean.Sherlock.Bridge.PvBinding.noDivFrom_sound
#print axioms Clean.Sherlock.Bridge.PvBinding.prime_of_noDiv
#print axioms Clean.Sherlock.Bridge.PvBinding.pBB_prime
#print axioms Clean.Sherlock.Bridge.PvBinding.pv_binding_count_babybear
